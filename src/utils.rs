use bytemuck;
use glam::{Vec2, Vec3};
use std::cmp::PartialOrd;

/// Denotes a value that is processed somewhere else and should only be reprocessed if it has changed.
/// In particular its useful for values that would be expensive to repeatedly `PartialEq` or `Clone`.
/// 
/// # Type parameters
/// 
/// * `T` - This can be pretty much any type that implements or derives `Default`.
/// It needs to be `Default` as `std::mem::take()` is used internally to hot-swap `Dirty` to `Clean`.
pub enum Volatile<T> {
    /// Value has been changed since last processed.
    Dirty(T),
    /// Value has not been changed since last processed.
    Clean(T)
}
impl<T> Volatile<T> {
    /// Gets an immutable reference to the stored internal value.
    pub fn inner(&self) -> &T {
        match self {
            Self::Clean(v) => v,
            Self::Dirty(v) => v
        }
    }
    /// Gets a mutable refernce to the stored internal value.
    pub fn inner_mut(&mut self) -> &mut T {
        match self {
            Self::Clean(v) => v,
            Self::Dirty(v) => v
        }
    }
}
impl<T: Default> Volatile<T> {
    // TODO Might be a better function name
    /// Runs the passed closure if the variant is `Dirty`.
    /// Once the closure finishes running the variant is hot-swapped from `Dirty` to `Clean`
    /// while preserving the internal value.
    pub fn if_dirty_clean_with<F>(&mut self, func: F)
    where
        F: FnOnce(&T)
    {
        if let Self::Dirty(data) = self {
            func(&data);
            *self = Self::Clean(std::mem::take(data));
        }
    }
    /// Once the closure finishes the variant is hot-swapped from to `Dirty`, regardless of the starting variant.
    /// Preserves internal value.
    pub fn dirty_with<F>(&mut self, func: F)
    where
        F: FnOnce(&mut T)
    {
        let data = self.inner_mut();
        func(data);
        *self = Self::Dirty(std::mem::take(data));
    }

    pub fn to_clean(&mut self) {
        if let Self::Dirty(data) = self {
            *self = Self::Clean(std::mem::take(data));
        }
    }
}
impl<T: Default> Default for Volatile<T> {
    fn default() -> Self {
        Volatile::Clean(T::default())
    }
}

// ! These will produce undefined behavior if either value is a NaN
mod sealed {
    pub trait Float {  }
}

impl sealed::Float for f32 {  }
impl sealed::Float for f64 {  }
pub fn minf<F: sealed::Float + PartialOrd>(a: F, b: F) -> F  {
    if a != a { a } 
    else if b != b { b }
    else if a < b { a } 
    else { b }
}
pub fn maxf<F: sealed::Float + PartialOrd>(a: F, b: F) -> F  {
    if a != a { a } 
    else if b != b { b }
    else if a > b { a } 
    else { b }
}
