mod ffi;
mod components;
mod wire;

#[cfg(test)]
mod test {
    use crate::ffi::{init, exit, components, free_components, add_component, FFIArr};
    use std::mem;
    use std::ffi::{CString, CStr};
    
    #[test]
    fn test_adding_components() {
        let data = init();
        
        let mut c = components(data);
        assert_eq!(c.len, 0);
        free_components(c);
        
        add_component(data, 0);
        
        let mut c = components(data);
        assert_eq!(c.len, 1);
        let e1 = unsafe { &*c.elem };
        assert_eq!(unsafe { CStr::from_ptr(e1.path) }, CString::new("const.png").unwrap().as_c_str());
        assert_eq!(e1.position, [0, 0]);
        free_components(c);
        
        exit(data);
    }
    
    #[test]
    fn test_ffi_arr() {
        let vec = vec![1, 2, 3];
        
        let arr = FFIArr::from_vec(vec);
        
        let v2 = unsafe { arr.into_vec() };
        
        assert_eq!(vec![1, 2, 3], v2);
    }
}