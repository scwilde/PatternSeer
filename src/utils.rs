use bytemuck;


#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable, PartialEq, Debug)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 3]
}


pub struct Triangle {
    pub vertices: [Vertex; 3]
}
impl Triangle {
    pub fn rotate_left(&mut self) {
        for mut vert in self.vertices {
            vert.color.rotate_left(1);
        }
    }
    pub fn rotate_right(&mut self) {
        for mut vert in self.vertices {
            vert.color.rotate_right(1);
        }
    }
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
    pub fn inner_mut(&mut self) -> &mut T {
        match self {
            Self::Clean(v) => v,
            Self::Dirty(v) => v
        }
    }
}
impl<T: Default> Volatile<T> {
    // TODO Might be a better function name
    pub fn if_dirty_clean_with<F>(&mut self, func: F)
    where
        F: FnOnce(&T)
    {
        if let Self::Dirty(data) = self {
            func(data);
            *self = Self::Clean(std::mem::take(data));
        }
    }
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
