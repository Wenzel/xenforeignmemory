use std::os::raw::{c_int, c_uint, c_void};

use xenforeignmemory_sys::{xen_pfn_t, xenforeignmemory_handle, xentoollog_logger};

use libloading::os::unix::Symbol as RawSymbol;
use libloading::{Library, Symbol};
use log::info;

const LIBXENFOREIGNMEMORY_FILENAME: &str = "libxenforeignmemory.so";
// xenforeignmemory_open
type FnOpen =
    fn(logger: *mut xentoollog_logger, open_flags: c_uint) -> *mut xenforeignmemory_handle;
// xenforeignmemory_close
type FnClose = fn(fmem: *mut xenforeignmemory_handle) -> c_int;
// xenforeignmemory_map
type FnMap = fn(
    fmem: *mut xenforeignmemory_handle,
    dom: u32,
    prot: c_int,
    pages: usize,
    arr: *const xen_pfn_t,
    err: *mut c_int,
) -> *mut c_void;
// xenforeignmemory_unmap
type FnUnmap = fn(fmem: *mut xenforeignmemory_handle, addr: *mut c_void, pages: usize) -> c_int;

#[derive(Debug)]
pub struct LibXenForeignMemory {
    lib: Library,
    pub open: RawSymbol<FnOpen>,
    pub close: RawSymbol<FnClose>,
    pub map: RawSymbol<FnMap>,
    pub unmap: RawSymbol<FnUnmap>,
}

impl LibXenForeignMemory {
    pub unsafe fn new() -> Self {
        info!("Loading {}", LIBXENFOREIGNMEMORY_FILENAME);
        let lib = Library::new(LIBXENFOREIGNMEMORY_FILENAME).unwrap();
        // load symbols
        let open_sym: Symbol<FnOpen> = lib.get(b"xenforeignmemory_open\0").unwrap();
        let open = open_sym.into_raw();

        let close_sym: Symbol<FnClose> = lib.get(b"xenforeignmemory_close\0").unwrap();
        let close = close_sym.into_raw();

        let map_sym: Symbol<FnMap> = lib.get(b"xenforeignmemory_map\0").unwrap();
        let map = map_sym.into_raw();

        let unmap_sym: Symbol<FnUnmap> = lib.get(b"xenforeignmemory_unmap\0").unwrap();
        let unmap = unmap_sym.into_raw();

        LibXenForeignMemory {
            lib,
            open,
            close,
            map,
            unmap,
        }
    }
}
