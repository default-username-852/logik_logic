use std::fmt::Debug;
use std::ffi::CString;
use crate::ffi::FFIComponent;

pub trait Component: Debug {
    fn export(&self) -> FFIComponent;
}

#[derive(Debug)]
pub struct Constant {

}

impl Constant {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for Constant {
    fn export(&self) -> FFIComponent {
        FFIComponent::new(CString::new("const.png").unwrap().into_raw(), [0, 0])
    }
}