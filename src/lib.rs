extern crate xenforeignmemory_sys;
use std::ptr::{null_mut};

#[derive(Debug)]
pub struct XenForeignMem {
    handle: *mut xenforeignmemory_sys::xenforeignmemory_handle,
}

impl XenForeignMem {

    pub fn new() -> Result<Self,String> {
        let fn_handle = unsafe {
            xenforeignmemory_sys::xenforeignmemory_open(null_mut(), 0)
        };
        if fn_handle == null_mut() {
            return Err("Fail to open xenforeignmemory interface".to_string());
        }
        return Ok(XenForeignMem {
            handle: fn_handle,
        });
    }

    fn close(&mut self) -> Result<(),&str> {
        let result = unsafe {
            xenforeignmemory_sys::xenforeignmemory_close(self.handle)
        };
        match result {
            0 => Ok(()),
            _ => Err("Fail to close xenforeignmemory interface"),
        }
    }
}

impl Drop for XenForeignMem {
    fn drop(&mut self) {
        self.close();
    }
}
