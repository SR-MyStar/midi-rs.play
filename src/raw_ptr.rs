#[derive(Copy)]
pub struct RawMutPtr<T>(pub *mut T);

impl<T> Clone for RawMutPtr<T> {
    fn clone(&self) -> Self {
        RawMutPtr::new(unsafe { self.0.as_mut().unwrap() })
    }
}

impl<T> RawMutPtr<T> {
    pub fn new(ptr: &mut T) -> Self {
        Self(ptr)
    }
}

unsafe impl<T> Send for RawMutPtr<T> {}
unsafe impl<T> Sync for RawMutPtr<T> {}
