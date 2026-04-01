use bytemuck;


#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable, PartialEq, Debug)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 3]
}


pub enum Volatile<T> {
    Dirty(T),
    Clean(T)
}
impl<T> Volatile<T> {
    pub fn inner(&self) -> &T {
        match self {
            Self::Clean(v) => v,
            Self::Dirty(v) => v
        }
    }
}
impl<T: Default> Volatile<T> {
    pub fn transform_with<F>(&mut self, func: F)
    where 
        F: FnOnce(Self) -> Self 
    {
        // ! I really dont like this API
        let taken = std::mem::take(self);
        *self = func(taken);
    }
}
impl<T: Default> Default for Volatile<T> 
{
    fn default() -> Self {
        Volatile::Clean(T::default())
    }
}
