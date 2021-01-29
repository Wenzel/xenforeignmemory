mod libxenforeignmemory;

extern crate xenforeignmemory_sys;

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

impl XenForeignMem {
    pub fn new() -> Result<Self, XenForeignMemoryError> {
        let libxenforeignmemory = unsafe { LibXenForeignMemory::new() };
        let fn_handle = (libxenforeignmemory.open)(null_mut(), 0);
        if fn_handle.is_null() {
            Err(XenForeignMemoryError::OpenError(IoError::last_os_error()))
        } else {
            Ok(XenForeignMem {
                handle: fn_handle,
                libxenforeignmemory,
            })
        }
    }

    pub fn map(
        &self,
        domid: u32,
        prot: c_int,
        gfn: u64,
    ) -> Result<&mut [u8], XenForeignMemoryError> {
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
            Err(XenForeignMemoryError::MappingError {
                gfn,
                source: IoError::last_os_error(),
            })
        } else {
            // TODO
            // size of the page mapped is always 4K ?
            // if so, get it from xenctrl::PAGE_SIZE
            Ok(unsafe { slice::from_raw_parts_mut(map, 4096) })
        }
    }

    pub fn unmap(&self, page: &mut [u8]) -> Result<(), XenForeignMemoryError> {
        let addr = page.as_mut_ptr() as *mut c_void;
        let result = (self.libxenforeignmemory.unmap)(self.handle, addr, 1);
        match result {
            0 => Ok(()),
            _ => Err(XenForeignMemoryError::UnmappingError {
                addr: addr as u64,
                source: IoError::last_os_error(),
            }),
        }
    }

    fn close(&mut self) -> Result<(), XenForeignMemoryError> {
        let result = (self.libxenforeignmemory.close)(self.handle);
        match result {
            0 => Ok(()),
            _ => Err(XenForeignMemoryError::CloseError),
        }
    }
}

impl Drop for XenForeignMem {
    fn drop(&mut self) {
        self.close().unwrap();
    }
}

#[derive(thiserror::Error, Debug)]
pub enum XenForeignMemoryError {
    #[error("Failed to open xenforeignmemory interface.")]
    OpenError(#[source] IoError),
    #[error("Failed to close xenforeignmemory interface.")]
    CloseError,
    #[error("Failed to map gfn 0x{gfn:X}.")]
    MappingError {
        gfn: u64,
        #[source]
        source: IoError,
    },
    #[error("Failed to unmap page at 0x{addr:X}.")]
    UnmappingError {
        addr: u64,
        #[source]
        source: IoError,
    },
}
