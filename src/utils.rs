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


pub trait CommandBuffer {
    type Command;
    fn push(&mut self, cmd: Self::Command) -> Option<Self::Command>;
    fn new() -> Self;
}

pub enum CommandSlot<T> {
    Inactive,
    Active(T),
}
impl<T> CommandSlot<T> {
    pub fn take(&mut self) -> Option<T> {
        match std::mem::replace(self, Self::Inactive) {
            Self::Inactive => None,
            Self::Active(inner) => Some(inner),
        }
    }

    pub fn query(&mut self) -> Option<&T> {
        match self {
            Self::Inactive => None,
            Self::Active(inner) => Some(inner),
        }
    }

    pub fn replace(&mut self, msg: T) -> Option<T> {
        match std::mem::replace(self, Self::Active(msg)) {
            Self::Inactive => None,
            Self::Active(inner) => Some(inner),
        }
    }
}
