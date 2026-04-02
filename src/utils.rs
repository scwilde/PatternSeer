use bytemuck;


/// A single point in 2D space containing a position and color.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable, PartialEq, Debug)]
pub struct Vertex {
    // TODO Replace with a Vec2 from a crate, prob numix
    /// X and Y position of this vertex in space.
    pub position: [f32; 2],
    // TODO Replace with a Vec3 from a crate, prob numix
    /// red, green, and blue color values of this vertex.
    pub color: [f32; 3]
}


/// A triangle made up of 3 `Vertex`s.
pub struct Triangle {
    /// Vertices that make up this triangle.
    pub vertices: [Vertex; 3]
}


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
            func(data);
            *self = Self::Clean(std::mem::take(data));
        }
    }
    /// Once the closure finishes the variant is hot-swapped from to `Dirty`, regardless of the starting variant.
    /// Preserves internal value.
    pub fn make_dirty_with<F>(&mut self, func: F)
    where
        F: FnOnce(&T)
    {
        let data = self.inner_mut();
        func(data);
        *self = Self::Dirty(std::mem::take(data));
    }
}
impl<T: Default> Default for Volatile<T> 
{
    fn default() -> Self {
        Volatile::Clean(T::default())
    }
}
