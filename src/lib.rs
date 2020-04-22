mod libxenforeignmemory;

extern crate xenforeignmemory_sys;

use std::io::Error;
use std::os::raw::{c_int, c_uchar, c_ulong, c_void};
use std::ptr::null_mut;
use std::slice;

use libxenforeignmemory::LibXenForeignMemory;

#[derive(Debug)]
pub struct XenForeignMem {
    handle: *mut xenforeignmemory_sys::xenforeignmemory_handle,
    libxenforeignmemory: LibXenForeignMemory,
}

impl XenForeignMem {
    pub fn new() -> Result<Self, String> {
        let libxenforeignmemory = unsafe { LibXenForeignMemory::new() };
        let fn_handle = (libxenforeignmemory.open)(null_mut(), 0);
        if fn_handle == null_mut() {
            return Err("Fail to open xenforeignmemory interface".to_string());
        }
        return Ok(XenForeignMem {
            handle: fn_handle,
            libxenforeignmemory,
        });
    }

    pub fn map(&self, domid: u32, prot: c_int, gfn: u64) -> Result<&mut [u8], &str> {
        let arr_gfn: [c_ulong; 1] = [gfn];
        let map = (self.libxenforeignmemory.map)(
            self.handle,
            domid,
            prot,
            arr_gfn.len(),
            arr_gfn.as_ptr(),
            null_mut(),
        ) as *mut c_uchar;
        if map.is_null() {
            return Err("Fail to map gfn");
        }
        // TODO
        // size of the page mapped is always 4K ?
        // if so, get it from xenctrl::PAGE_SIZE
        Ok(unsafe { slice::from_raw_parts_mut(map, 4096 as usize) })
    }

    pub fn unmap(&self, page: &mut [u8]) -> Result<(), Error> {
        let addr = page.as_mut_ptr() as *mut c_void;
        let result = (self.libxenforeignmemory.unmap)(self.handle, addr, 1);
        match result {
            0 => Ok(()),
            _ => Err(Error::last_os_error()),
        }
    }

    fn close(&mut self) -> Result<(), &str> {
        let result = (self.libxenforeignmemory.close)(self.handle);
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
