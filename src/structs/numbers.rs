pub trait Float {
    fn to_f32(&self) -> f32;
}
impl Float for f32 {
    fn to_f32(&self) -> f32 {
        *self
    }
}
impl Float for f64 {
    fn to_f32(&self) -> f32 {
        *self as f32
    }
}

pub trait Integer {
    fn to_isize(&self) -> isize;
}
impl Integer for f32 {
    fn to_isize(&self) -> isize {
        *self as isize
    }
}
impl Integer for usize {
    fn to_isize(&self) -> isize {
        *self as isize
    }
}
impl Integer for u32 {
    fn to_isize(&self) -> isize {
        *self as isize
    }
}
impl Integer for i32 {
    fn to_isize(&self) -> isize {
        *self as isize
    }
}
impl Integer for isize {
    fn to_isize(&self) -> isize {
        *self
    }
}
