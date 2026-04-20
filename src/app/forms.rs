pub trait FormDraft {
    type Error: FormError;
    type Complete: FormComplete;

    fn finish(&self) -> Result<Self::Complete, Self::Error>
    where
        Self: Sized;
}
pub trait FormComplete {}
pub trait FormError {}


mod sealed {
    use crate::app::forms::{Form, FormComplete, FormDraft};

    pub struct Closed;
    pub struct Pending<D: FormDraft>(D);
    pub struct TakenForEdits;
    pub struct Completed<C: FormComplete>(C);
    pub struct Transitioning;

    impl Closed {
        pub fn open<D>(self, new_draft: D) -> Pending<D>
        where
            D: FormDraft,
        {
            Pending(new_draft)
        }

        pub fn save_state<D,C>(self) -> Form<D,C>
        where
            D: FormDraft,
            C: FormComplete,
        {
            Form::Closed(self)
        }
    }

    impl<D> Pending<D>
    where
        D: FormDraft
    {
        pub fn take_for_edits(self) -> (TakenForEdits, D) {
            (TakenForEdits, self.0)
        }

        pub fn save_state<C>(self) -> Form<D,C>
        where
            C: FormComplete,
        {
            Form::Pending(self)
        }
    }

    impl<C> Completed<C>
    where
        C: FormComplete
    {
        pub fn close_as_done(self) -> (Closed, C) {
            (Closed, self.0)
        }

        pub fn save_state<D>(self) -> Form<D,C>
        where
            D: FormDraft
        {
            Form::Completed(self)
        }
    }

    impl TakenForEdits {
        pub fn close_as_cancelled(self) -> Closed {
            Closed
        }

        pub fn submit_draft<D>(self, draft: D) -> Pending<D>
        where
            D: FormDraft
        {
            Pending(draft)
        }

        pub fn submit_complete<C>(self, complete: C) -> Completed<C>
        where
            C: FormComplete
        {
            Completed(complete)
        }
    }
}


pub enum Form<D,C>
where
    D: FormDraft,
    C: FormComplete,
{
    Closed(sealed::Closed),
    Pending(sealed::Pending<D>),
    TakenForEdits(sealed::TakenForEdits),
    Transitioning(sealed::Transitioning),
    Completed(sealed::Completed<C>),
}
impl<D,C> Form<D,C>
where
    D: FormDraft,
    C: FormComplete,
{
    pub fn new() -> Self {
        Self::Closed(sealed::Closed {})
    }

    pub fn transition(&mut self) -> Self {
        let old = std::mem::replace(self, Self::Transitioning(sealed::Transitioning));
        old
    }
}
