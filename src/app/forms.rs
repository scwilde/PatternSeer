/// The incomplete version of some type that is constructed via a form.
pub trait FormDraft {
    /// Enum with variants for that might be caused while editing the draft or while
    /// turning the draft into its completed type.
    type Error: std::fmt::Debug;
    /// The type that this draft will turn into when the form is completed.
    type Complete: std::fmt::Debug;

    /// Transitions the draft into its completed form.
    ///
    /// # Returns
    ///
    /// A Result that can either be:
    /// -`Ok(c)`: When the draft was successfully finalized, where c is the finalized form.
    /// -'Err': When there was some issue in finalizing the draft, where e is what error happened.
    fn finish(&self) -> Result<Self::Complete, Self::Error>
    where
        Self: Sized;
}


mod sealed {
    use crate::app::forms::{FormTSM, FormDraft};

    /// The form is currently closed to editing.
    #[derive(Debug)]
    pub struct Closed;
    /// The form is currently open and awaiting new edits. Stores the incomplete draft of the form.
    #[derive(Debug)]
    pub struct Pending<D: FormDraft>(D);
    /// The draft has been pulled out in order to be edited.
    #[derive(Debug)]
    pub struct TakenForEdits;
    /// The form is currently in the middle of transitioning to another state. This lets the form act as a passive
    /// mutex if multiple threads try to modify it at once. This state is unable to transition itself to another state.
    #[derive(Debug)]
    pub struct Transitioning;

    impl Closed {
        /// Opens the form to editing.
        ///
        /// # Parameters
        ///
        /// - `new_draft`: The starting `FormDraft` to open the form with.
        ///
        /// # Returns
        ///
        /// A new `Pending` typestate containing `new_draft`.
        pub fn open<D: FormDraft>(self, new_draft: D) -> Pending<D> {
            Pending(new_draft)
        }

        /// Wraps this typestate up into its corresponding `FormTSM` enum variant.
        pub fn save_state<D: FormDraft>(self) -> FormTSM<D> {
            FormTSM::Closed(self)
        }
    }

    impl<D: FormDraft> Pending<D> {
        /// Takes ownership of the `FormDraft` stored within in order to make edits.
        ///
        /// # Returns
        ///
        /// A tuple containing
        /// - A new `TakenForEdits` typestate.
        /// - the `FormDraft` contained within.
        pub fn take_for_edits(self) -> (TakenForEdits, D) {
            (TakenForEdits, self.0)
        }

        /// Wraps this typestate up in its corresponding `FormTSM` enum variant.
        pub fn save_state(self) -> FormTSM<D> {
            FormTSM::Pending(self)
        }
    }

    impl TakenForEdits {
        /// Submits an edited draft, but keeps the form open to more edits.
        ///
        /// # Parameters
        ///
        /// - `draft`: The updated draft version of the form.
        ///
        /// # Returns
        ///
        /// A new `Pending` typestate containing `draft`.
        pub fn submit_draft<D: FormDraft>(self, draft: D) -> Pending<D> {
            Pending(draft)
        }

        /// Assumes the draft has been finalized and closes the form.
        ///
        /// # Returns
        ///
        /// A new `Closed` typestate.
        pub fn close_as_done(self) -> Closed {
            Closed
        }

        /// Discards the draft completely and closes the form.
        ///
        /// # Returns
        ///
        /// A new `Closed` typestate.
        pub fn close_as_cancelled(self) -> Closed {
            Closed
        }
    }
}


/// An enum wrapping around a typestate machine that describes the different states a form can be in.
///
/// This typestate machine is designed to give all the safety of a normal typestate machine,
/// preventing invalid state transitions.
/// However, the enum wrapper gives the added bonus that an instance of this machine can live inside
/// the field of a long-lived struct, like the main `App` struct, without having to shadow it when changing states.
///
/// # States
///
/// **Closed**
///
/// The form is currently closed to editing.
/// - `open(new_draft) -> Pending(new_draft)`: Open up the form to editing.
/// - `save_state() -> FormTSM`: Wrap this state up into a `FormTSM` enum variant.
///
/// **Transitioning**
///
/// The form is currently in the middle of transitioning to another state. This lets the form act as a passive
/// mutex if multiple threads try to modify it at once. This state is unable to transition itself to another state.
///
/// **Pending(draft)**
///
/// The form is currently open and awaiting new edits. Stores the incomplete draft of the form.
/// - `take_for_edits() -> (TakenForEdits, draft)`: Take owndership of the draft stored within to make edits.
/// - `save_state() -> FormTSM`: Wrap this state up into a `FormTSM` enum variant.
///
/// **Completed(complete)**
///
/// The form is fully filled out and ready to be closed. Stores an instance of type that the draft turns into when
/// completed.
/// - `close_as_done() -> (Closed, complete)`: Take the completed draft and close the form.
/// - `save_state() -> FormTSM`: Wrap this state up into a `FormTSM` enum variant.
///
/// **TakenForEdits**
///
/// *(internal only)*
///
/// The draft has been pulled out in order to be edited.
/// - `submit_draft(draft) -> Pending(draft)`: Submit an edited draft, but keep the form open to more edits.
/// - `submit_complete(complete) -> Complete(complete)`: Submit the finalized version of the draft and signal that the
/// form should be closed.
/// - `close_as_cancelled() -> Closed`: Discard the draft completely and close the form.
///
/// # Usage
///
/// Start by creating a form object using `Form::<D>::new()`. This will create a new form in the `Closed` state.
/// `D` here is your type that implements `FormDraft`.
/// ```
/// let example = Form::<ExampleDraft>::new();
/// ```
///
/// Now, to transition to another state we have to "check out" the state machine using `.transition_with()`.
/// The `FormTSM` inside of `example` will be updated to the `Transitioning` state and we will be provided with a
/// scope inside of which we own the `FormTSM`.
/// ```
/// example.transition(|state| {
///     // Transition state
/// });
/// ```
///
/// Transitioning to a new state is done by destructuring the outer enum to get at the inner typestate.
/// We then call a valid state transition function on the inner typestate, which will consume the old typestate
/// and return the new typestate.
/// ```
///     if let Closed(closed_state) = state {
///         let open_state = closed_state.open(new_draft);
///     }
/// ```
///
/// When transitioning from a state that contains data to one that doesn't a tuple will be returned with the
/// new state and the data from the old state.
/// ```
///         let (taken, draft) = open_state.take_for_edits();
/// ```
///
/// Once we're done changing state our scope inside of `.transition_with()` has to return a new `FormTSM` which
/// will automatically update `example` to our new state. To turn an inner typestate into a returnable
/// `FormTSM` we can call `.save_state()` on any state that is valid outside this transitionary scope.
/// ```
///         return open_state.save_state();
/// ```
///
/// Here is a typical full example:
/// ```
/// let example = Form::<ExampleDraft>::new();
///
/// example.transition(|state| {
///     match state {
///         FormTSM::Closed(inner) => inner.open(new_draft).save_state(),
///         FormTSM::Pending(inner) => {
///             let (inner, mut draft) = inner.take_for_edits();
///             // do something with draft
///             inner.submit_draft(draft).save_state()
///         },
///         FormTSM::Closed(inner) => {
///             let (inner, complete) = inner.close_as_done();
///             // do something with complete
///             inner.save_state()
///         },
///         FormTSM::Transitioning(_) => unreachable!() // We cannot transition from `Transitioning` to any other state
///     }
/// });
/// ```
///
/// # Safety
///
/// Typestate safety is guaranteed by the internal `State` structs being sealed. This means there is no way to
/// manually construct a `Form::Pending`, etc. because we can't construct their internal `Pending`, etc.
/// This gives full control over state transitions to the internal `State` structs.
///
/// Additionally, thread safety is guaranteed with the `Transitioning` state acting as a passive mutex in case
/// multiple threads try to change the state at the same time. Due to the closure inside of `.transition_with` being
/// required to return a `FormTSM` it is also impossible to accidentally drop the internal typestate and lock
/// the `FormTSM` into `Transitioning` forever.
#[derive(Debug)]
pub enum FormTSM<D: FormDraft> {
    Closed(sealed::Closed),
    Pending(sealed::Pending<D>),
    Transitioning(sealed::Transitioning),
}
impl<D: FormDraft> FormTSM<D> {
    ///Creates a new form in the `Closed` state.
    pub fn new() -> Self {
        Self::Closed(sealed::Closed)
    }

    /// Sets the state of `self` to `Transitioning` and provides ownership of the old state to the passed closure.
    ///
    /// # Parameters
    ///
    /// - `transition`: A closure that will be passed ownership of the old state. Must return the new `FormTSM` variant
    /// that will replace `self` when the closure finishes.
    pub fn transition_with<F>(&mut self, transition: F)
    where F: FnOnce(Self) -> Self
    {
        let state = std::mem::replace(self, Self::Transitioning(sealed::Transitioning));
        *self = transition(state);
    }
}
