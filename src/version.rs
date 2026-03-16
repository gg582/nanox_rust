extern "C" {
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
}
pub const PROGRAM_NAME_LONG: [::core::ffi::c_char; 10] = unsafe {
    ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"uEmacs/Pk\0")
};
pub const VERSION: [::core::ffi::c_char; 7] = unsafe {
    ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"4.0.15\0")
};
#[no_mangle]
pub unsafe extern "C" fn version() {
    printf(
        b"%s version %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        PROGRAM_NAME_LONG.as_ptr(),
        VERSION.as_ptr(),
    );
}
