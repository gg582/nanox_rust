extern "C" {
    fn die(err: *const ::core::ffi::c_char, ...);
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn mkstemp(__template: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
}
pub type size_t = usize;
#[no_mangle]
pub unsafe extern "C" fn xmkstemp(
    mut template: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut fd: ::core::ffi::c_int = 0;
    fd = mkstemp(template);
    if fd < 0 as ::core::ffi::c_int {
        die(
            b"Unable to create temporary file\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    return fd;
}
#[no_mangle]
pub unsafe extern "C" fn xmalloc(mut size: size_t) -> *mut ::core::ffi::c_void {
    let mut ret: *mut ::core::ffi::c_void = malloc(size);
    if ret.is_null() {
        die(b"Out of memory\0" as *const u8 as *const ::core::ffi::c_char);
    }
    return ret;
}
