extern crate xenforeignmemory_sys;
use std::io::Error;
use std::ptr::{null_mut};
use std::os::raw::{c_int, c_ulong, c_void};

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

    pub fn map(&self, domid: u32, prot: c_int, gfn: u64) -> Result<*mut c_void,&str>{
        let arr_gfn: [c_ulong; 1] = [gfn];
        let map = unsafe {
            xenforeignmemory_sys::xenforeignmemory_map(self.handle, domid, prot, arr_gfn.len(), arr_gfn.as_ptr(), null_mut())
        };
        if map.is_null() {
            return Err("Fail to map gfn");
        }
        Ok(map)
    }

    pub fn unmap(&self, addr: *mut c_void) -> Result<(),Error>{
        let result = unsafe {
            xenforeignmemory_sys::xenforeignmemory_unmap(self.handle, addr, 1)
        };
        match result {
            0 => Ok(()),
            _ => Err(Error::last_os_error())
        }
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
        self.close().unwrap();
    }
}
