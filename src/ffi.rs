use crate::components::{Component, Constant};
use std::ffi::CString;
use std::fmt::Debug;

#[repr(C)]
pub struct Data {
    components: Vec<Box<dyn Component>>,
    wires: Vec<usize>,
}

#[repr(C)]
#[derive(Debug)]
pub struct FFIArr<T: Debug> {
    pub(crate) elem: *mut T,
    pub(crate) len: usize,
    cap: usize,
}

impl<T: Debug> FFIArr<T> {
    pub(crate) fn from_vec(mut v: Vec<T>) -> Self {
        let mut v = std::mem::ManuallyDrop::new(v);
        Self {
            elem: v.as_mut_ptr(),
            len: v.len(),
            cap: v.capacity()
        }
    }
    
    pub(crate) unsafe fn into_vec(self) -> Vec<T> {
        Vec::from_raw_parts(self.elem, self.len, self.cap)
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct FFIComponent {
    pub(crate) path: *mut libc::c_char,
    pub(crate) position: [i64; 2],
}

impl FFIComponent {
    pub fn new(path: *mut libc::c_char, position: [i64; 2]) -> Self {
        Self { path, position }
    }
}

#[no_mangle]
pub extern "C" fn init() -> *mut Data {
    let components = Vec::<Box<dyn Component>>::new();
    let wires = Vec::<usize>::new();
    
    let d = Box::new(Data { components, wires });
    
    Box::into_raw(d)
}

#[no_mangle]
pub extern "C" fn exit(data: *mut Data) {
    drop(unsafe { Box::from_raw(data) });
}

#[no_mangle]
pub extern "C" fn components(data: *mut Data) -> FFIArr<FFIComponent> {
    let mut components = unsafe { &mut *data }.components
        .iter()
        .map(|e| e.export())
        .collect::<Vec<_>>();
    
    FFIArr::from_vec(components)
}

pub extern "C" fn free_components(components: FFIArr<FFIComponent>) {
    let dropping = unsafe { components.into_vec() };
    
    for d in dropping {
        unsafe { CString::from_raw(d.path) };
    }
}

#[no_mangle]
pub extern "C" fn add_component(data: *mut Data, component_type: u16) -> bool {
    let d = unsafe { &mut *data };
    match component_type {
        0 => {
            d.components.push(Box::new(Constant::new()))
        },
        _ => {
            return false;
        }
    }
    
    true
}

/*
#[no_mangle]
pub extern "C" fn tick(data: *mut Data) {

}*/
