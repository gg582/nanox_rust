pub static mut lname: [*mut core::ffi::c_char; 1024] = [core::ptr::null_mut(); 1024];
extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcat(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    static mut numlocks: ::core::ffi::c_int;
    fn mlwrite(fmt: *const ::core::ffi::c_char, ...);
    fn mlyesno(prompt: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn dolock(fname: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn undolock(fname: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn __errno_location() -> *mut ::core::ffi::c_int;
}
pub type size_t = usize;
pub const NLOCKS: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ABORT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn lockchk(
    mut fname: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut status: ::core::ffi::c_int = 0;
    if numlocks > 0 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i < numlocks {
            if strcmp(fname, lname[i as usize]) == 0 as ::core::ffi::c_int {
                return TRUE;
            }
            i += 1;
        }
    }
    if numlocks == NLOCKS {
        mlwrite(
            b"LOCK ERROR: Lock table full\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ABORT;
    }
    status = lock(fname);
    if status == ABORT {
        return ABORT;
    }
    if status == FALSE {
        return TRUE;
    }
    numlocks += 1;
    lname[(numlocks - 1 as ::core::ffi::c_int) as usize] = malloc(
        strlen(fname).wrapping_add(1 as size_t),
    ) as *mut ::core::ffi::c_char;
    if lname[(numlocks - 1 as ::core::ffi::c_int) as usize].is_null() {
        undolock(fname);
        mlwrite(
            b"Cannot lock, out of memory\0" as *const u8 as *const ::core::ffi::c_char,
        );
        numlocks -= 1;
        return ABORT;
    }
    strcpy(lname[(numlocks - 1 as ::core::ffi::c_int) as usize], fname);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn lockrel() -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut status: ::core::ffi::c_int = 0;
    let mut s: ::core::ffi::c_int = 0;
    status = TRUE;
    if numlocks > 0 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i < numlocks {
            s = unlock(lname[i as usize]);
            if s != TRUE {
                status = s;
            }
            free(lname[i as usize] as *mut ::core::ffi::c_void);
            i += 1;
        }
    }
    numlocks = 0 as ::core::ffi::c_int;
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn lock(
    mut fname: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut locker: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut status: ::core::ffi::c_int = 0;
    let mut msg: [::core::ffi::c_char; 1024] = [0; 1024];
    locker = dolock(fname);
    if locker.is_null() {
        return TRUE;
    }
    if strncmp(locker, b"LOCK\0" as *const u8 as *const ::core::ffi::c_char, 4 as size_t)
        == 0 as ::core::ffi::c_int
    {
        lckerror(locker);
        return ABORT;
    }
    strcpy(
        &raw mut msg as *mut ::core::ffi::c_char,
        b"File in use by \0" as *const u8 as *const ::core::ffi::c_char,
    );
    strcat(&raw mut msg as *mut ::core::ffi::c_char, locker);
    strcat(
        &raw mut msg as *mut ::core::ffi::c_char,
        b", override?\0" as *const u8 as *const ::core::ffi::c_char,
    );
    status = mlyesno(&raw mut msg as *mut ::core::ffi::c_char);
    if status == TRUE { return FALSE } else { return ABORT };
}
#[no_mangle]
pub unsafe extern "C" fn unlock(
    mut fname: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut locker: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    locker = undolock(fname);
    if locker.is_null() {
        return TRUE;
    }
    lckerror(locker);
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn lckerror(mut errstr: *mut ::core::ffi::c_char) {
    let mut obuf: [::core::ffi::c_char; 1024] = [0; 1024];
    strcpy(&raw mut obuf as *mut ::core::ffi::c_char, errstr);
    strcat(
        &raw mut obuf as *mut ::core::ffi::c_char,
        b" - \0" as *const u8 as *const ::core::ffi::c_char,
    );
    strcat(&raw mut obuf as *mut ::core::ffi::c_char, strerror(*__errno_location()));
    mlwrite(&raw mut obuf as *mut ::core::ffi::c_char);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
