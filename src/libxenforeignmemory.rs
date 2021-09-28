use std::os::raw::{c_int, c_uint, c_void};

use xenforeignmemory_sys::{xen_pfn_t, xenforeignmemory_handle, xentoollog_logger};

use libloading::os::unix::Symbol as RawSymbol;
use libloading::{library_filename, Error, Library, Symbol};
use log::info;

const LIBXENFOREIGNMEMORY_BASENAME: &str = "xenforeignmemory.so";
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
    pub unsafe fn new() -> Result<Self, Error> {
        let lib_filename = library_filename(LIBXENFOREIGNMEMORY_BASENAME);
        info!("Loading {}", lib_filename.to_str().unwrap());
        let lib = Library::new(lib_filename)?;
        // load symbols
        let open_sym: Symbol<FnOpen> = lib.get(b"xenforeignmemory_open\0")?;
        let open = open_sym.into_raw();

        let close_sym: Symbol<FnClose> = lib.get(b"xenforeignmemory_close\0")?;
        let close = close_sym.into_raw();

        let map_sym: Symbol<FnMap> = lib.get(b"xenforeignmemory_map\0")?;
        let map = map_sym.into_raw();

        let unmap_sym: Symbol<FnUnmap> = lib.get(b"xenforeignmemory_unmap\0")?;
        let unmap = unmap_sym.into_raw();

        Ok(LibXenForeignMemory {
            lib,
            open,
            close,
            map,
            unmap,
        })
    }
}
