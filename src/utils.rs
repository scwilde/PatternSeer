use std::cmp::PartialOrd;

/// Traits for internal `utils` use only
mod sealed {
    /// Sealed trait that encapsulates the f32 and f64 types
    pub trait Float {}
    impl Float for f32 {}
    impl Float for f64 {}
}

/// Min function to be used on f32 and f64 where a.min(b) is more ambiguous.
///
/// # Returns
///
/// Either an f32 or an f64 depending on which input parameter is selected.
/// If either `a` or `b` is `NaN` then `NaN` of that parameter's type will be returned.
pub fn minf<F: sealed::Float + PartialOrd>(a: F, b: F) -> F {
    if a != a {
        a
    } else if b != b {
        b
    } else if a < b {
        a
    } else {
        b
    }
}
/// Max function to be used on f32 and f64 where a.max(b) is more ambiguous.
///
/// # Returns
///
/// Either an f32 or an f64 depending on which input parameter is selected.
/// If either `a` or `b` is `NaN` then `NaN` of that parameter's type will be returned.
pub fn maxf<F: sealed::Float + PartialOrd>(a: F, b: F) -> F {
    if a != a {
        a
    } else if b != b {
        b
    } else if a > b {
        a
    } else {
        b
    }
}

/// Object for storing a minimum and maximum value.
#[derive(Default, Debug)]
pub struct Bounds<T>
where
    T: PartialOrd + Clone + Copy,
{
    pub min: T,
    pub max: T,
}

/// Object for storing a 2-Dimensional bounding box.
#[derive(Default, Debug)]
pub struct Bounds2d<T>
where
    T: PartialOrd + Clone + Copy,
{
    pub x: Bounds<T>,
    pub y: Bounds<T>,
}

/// Creates a new `Bounds2d` object from an array arranged like so:
/// ```
/// [
///     x: [min, max],
///     y: [min, max],
/// ]
/// ```
pub fn bounds2d<T>(bb: [[T; 2]; 2]) -> Bounds2d<T>
where
    T: PartialOrd + Clone + Copy,
{
    Bounds2d {
        x: Bounds {
            min: bb[0][0],
            max: bb[0][1],
        },
        y: Bounds {
            min: bb[1][0],
            max: bb[1][1],
        },
    }
}


/// A struct which contains indicators for various deferred commands. Each field should be a `CommandSlot`
/// with any data the command might need contained within it. Command slots are activated by calling `push()` and
/// passing in the equivalent `Command` variant. 
pub trait CommandBuffer {
    /// Enum containing all variants that correspond to the various commands this buffer can consume.
    type Command;

    /// Pushes a new command to the buffer.
    /// 
    /// # Parameters
    /// 
    /// - `cmd`: The `Command` variant to push into the buffer. May contian data.
    /// 
    /// # Returns
    /// 
    /// - `None`: if the slot corresponding to the passed command was inactive.
    /// - `Some`: If the slot was already active. Contains the same command passed but with ownership of
    /// the data previously stored within the slot.
    /// 
    /// # Side Effects
    /// 
    /// If inactive, the command slot corresponsing to the passed command will be activated and will take ownership
    /// of the data contained within the passed command. If active, the data inside the command slot will be replaced
    /// with the data in the passed command.
    fn push(&mut self, cmd: Self::Command) -> Option<Self::Command>;

    /// Creates a new `CommandBuffer` with all command slots inactive.
    fn new() -> Self;
}

/// A slot for a deferred command.
pub enum CommandSlot<T> {
    /// The command does not need to be acted on right now.
    Inactive,
    /// The command should be acted on at earliest convenience. Contains any data that should be acted with.
    Active(T),
}
impl<T> CommandSlot<T> {
    #![allow(dead_code)]

    /// Deactivates the command slot and pulls out the data passed to the command.
    /// 
    /// # Returns
    /// 
    /// - `None`: If the slot is inactive.
    /// - `Some`: If the slot is active. Contains the data to given to the command.
    pub fn take(&mut self) -> Option<T> {
        match std::mem::replace(self, Self::Inactive) {
            Self::Inactive => None,
            Self::Active(inner) => Some(inner),
        }
    }

    /// Checks if a command is active without deactivating it.
    /// 
    /// # Returns
    /// 
    /// - `None`: If the slot is not active.
    /// - `Some`: If the slot is active. Contains a reference to the data given to the command.
    pub fn query(&self) -> Option<&T> {
        match self {
            Self::Inactive => None,
            Self::Active(inner) => Some(inner),
        }
    }

    /// Activates a command slot if inactive.
    /// 
    /// # Parameters
    /// 
    /// - `data`: Data to give to the command when activating the slot.
    /// 
    /// # Returns
    /// 
    /// - `None`: If the slot was previously inactive.
    /// - `Some`: If the slot was already active. Contains the data previously given to the command.
    pub fn activate(&mut self, data: T) -> Option<T> {
        match std::mem::replace(self, Self::Active(data)) {
            Self::Inactive => None,
            Self::Active(inner) => Some(inner),
        }
    }
}
