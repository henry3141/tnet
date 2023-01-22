use std::alloc::{alloc, dealloc, Layout};

#[derive(Debug)]
pub struct Any {
    ptr: *mut u8,
    rc:*mut u8,
    layout: Layout,
}

impl Any {
    pub fn new<T>(data: T) -> Self {
        let layout = Layout::new::<T>();
        let ptr = unsafe { alloc(layout) };
        let rc = unsafe { alloc(Layout::new::<usize>()) };
        unsafe { std::ptr::write(rc as *mut usize, 1) };
        unsafe { std::ptr::write(ptr as *mut T, data) };
        Any { ptr, layout , rc}
    }

    pub fn deref<T>(&self) -> Option<&mut T> {
        if Layout::new::<T>() == self.layout {
            Some(unsafe { &mut *(self.ptr as *mut T) })
        } else {
            None
        }
    }
}


impl Clone for Any {
    fn clone(&self) -> Self {
        unsafe { std::ptr::write(self.rc as *mut usize, *self.rc as usize + 1) };
        Any {
            ptr: self.ptr,
            layout: self.layout,
            rc: self.rc,
        }
    }
}

impl Drop for Any {
    fn drop(&mut self) {
        unsafe { std::ptr::write(self.rc as *mut usize, *self.rc as usize - 1) };
        if unsafe{*self.rc} as usize == 0 {
            unsafe { dealloc(self.ptr, self.layout) };
            unsafe { dealloc(self.rc, Layout::new::<usize>()) };
        }
    }
}

unsafe impl Send for Any {}