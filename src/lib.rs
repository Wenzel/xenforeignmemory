mod libxenforeignmemory;

extern crate xenforeignmemory_sys;
#[macro_use]
extern crate failure;

use failure::Error;
use std::io::Error as IoError;
use std::os::raw::{c_int, c_uchar, c_ulong, c_void};
use std::ptr::null_mut;
use std::slice;

use libxenforeignmemory::LibXenForeignMemory;

#[derive(Debug)]
pub struct XenForeignMem {
    handle: *mut xenforeignmemory_sys::xenforeignmemory_handle,
    libxenforeignmemory: LibXenForeignMemory,
}

pub trait XenForeignMemoryIntrospectable: std::fmt::Debug {
    fn init(&mut self) -> Result<(), Error>;
    fn map(&self, domid: u32, prot: c_int, gfn: u64) -> Result<&mut [u8], Error>;
    fn unmap(&self, page: &mut [u8]) -> Result<(), Box<IoError>>;
    fn close(&mut self) -> Result<(), Error>;
}

pub fn create_xen_foreignmemory() -> XenForeignMem {
    XenForeignMem::new(unsafe { LibXenForeignMemory::new() })
}

impl XenForeignMem {
    pub fn new(libxenforeignmemory: LibXenForeignMemory) -> XenForeignMem {
        XenForeignMem {
            handle: null_mut(),
            libxenforeignmemory,
        }
    }
}

impl XenForeignMemoryIntrospectable for XenForeignMem {
    fn init(&mut self) -> Result<(), Error> {
        let fn_handle = (self.libxenforeignmemory.open)(null_mut(), 0);
        if fn_handle == null_mut() {
            return Err(format_err!("Fail to open xenforeignmemory interface"));
        }
        self.handle = fn_handle;
        Ok(())
    }

    fn map(&self, domid: u32, prot: c_int, gfn: u64) -> Result<&mut [u8], Error> {
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
            return Err(format_err!("Fail to map gfn"));
        }
        // TODO
        // size of the page mapped is always 4K ?
        // if so, get it from xenctrl::PAGE_SIZE
        Ok(unsafe { slice::from_raw_parts_mut(map, 4096 as usize) })
    }

    fn unmap(&self, page: &mut [u8]) -> Result<(), Box<IoError>> {
        let addr = page.as_mut_ptr() as *mut c_void;
        let result = (self.libxenforeignmemory.unmap)(self.handle, addr, 1);
        match result {
            0 => Ok(()),
            _ => Err(Box::new(IoError::last_os_error())),
        }
    }

    fn close(&mut self) -> Result<(), Error> {
        let result = (self.libxenforeignmemory.close)(self.handle);
        match result {
            0 => Ok(()),
            _ => Err(format_err!("Fail to close xenforeignmemory interface")),
        }
    }
}

impl Drop for XenForeignMem {
    fn drop(&mut self) {
        self.close().unwrap();
    }
}
