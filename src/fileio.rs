extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fgetc(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fputc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn fgets(
        __s: *mut ::core::ffi::c_char,
        __n: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    fn ferror(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    static mut nullflag: ::core::ffi::c_int;
    static mut fline: *mut ::core::ffi::c_char;
    static mut flen: ::core::ffi::c_int;
    fn mlwrite(fmt: *const ::core::ffi::c_char, ...);
}
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const NSTRING: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FIOSUC: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FIOFNF: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FIOEOF: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FIOERR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const FIOMEM: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
static mut ffp: *mut FILE = ::core::ptr::null::<FILE>() as *mut FILE;
static mut eofflag: ::core::ffi::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn ffropen(
    mut fn_0: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    ffp = fopen(fn_0, b"r\0" as *const u8 as *const ::core::ffi::c_char) as *mut FILE;
    if ffp.is_null() {
        return FIOFNF;
    }
    eofflag = FALSE;
    return FIOSUC;
}
#[no_mangle]
pub unsafe extern "C" fn ffwopen(
    mut fn_0: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    ffp = fopen(fn_0, b"w\0" as *const u8 as *const ::core::ffi::c_char) as *mut FILE;
    if ffp.is_null() {
        mlwrite(
            b"Cannot open file for writing\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return FIOERR;
    }
    return FIOSUC;
}
#[no_mangle]
pub unsafe extern "C" fn ffclose() -> ::core::ffi::c_int {
    if !fline.is_null() {
        free(fline as *mut ::core::ffi::c_void);
        fline = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    eofflag = FALSE;
    if fclose(ffp) != FALSE {
        mlwrite(b"Error closing file\0" as *const u8 as *const ::core::ffi::c_char);
        return FIOERR;
    }
    return FIOSUC;
}
#[no_mangle]
pub unsafe extern "C" fn ffputline(
    mut buf: *mut ::core::ffi::c_char,
    mut nbuf: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < nbuf {
        fputc(
            *buf.offset(i as isize) as ::core::ffi::c_int & 0xff as ::core::ffi::c_int,
            ffp,
        );
        i += 1;
    }
    fputc('\n' as i32, ffp);
    if ferror(ffp) != 0 {
        mlwrite(b"Write I/O error\0" as *const u8 as *const ::core::ffi::c_char);
        return FIOERR;
    }
    return FIOSUC;
}
#[no_mangle]
pub unsafe extern "C" fn ffgetline() -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut tmpline: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    if eofflag != 0 {
        return FIOEOF;
    }
    if flen > NSTRING {
        free(fline as *mut ::core::ffi::c_void);
        fline = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if fline.is_null() {
        flen = NSTRING;
        fline = malloc(flen as size_t) as *mut ::core::ffi::c_char;
        if fline.is_null() {
            return FIOMEM;
        }
    }
    if nullflag == 0 {
        if fgets(fline, NSTRING, ffp).is_null() {
            i = 0 as ::core::ffi::c_int;
            c = EOF;
        } else {
            i = strlen(fline) as ::core::ffi::c_int;
            c = 0 as ::core::ffi::c_int;
            if i > 0 as ::core::ffi::c_int {
                c = *fline.offset((i - 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
                i -= 1;
            }
        }
    } else {
        i = 0 as ::core::ffi::c_int;
        c = fgetc(ffp);
    }
    while c != EOF && c != '\n' as i32 {
        if c != 0 && c != '\r' as i32 {
            let fresh0 = i;
            i = i + 1;
            *fline.offset(fresh0 as isize) = c as ::core::ffi::c_char;
            if i >= flen {
                tmpline = malloc((flen + NSTRING) as size_t) as *mut ::core::ffi::c_char;
                if tmpline.is_null() {
                    return FIOMEM;
                }
                mystrscpy(tmpline, fline, flen);
                flen += NSTRING;
                free(fline as *mut ::core::ffi::c_void);
                fline = tmpline;
            }
        }
        c = fgetc(ffp);
    }
    if c == EOF {
        if ferror(ffp) != 0 {
            mlwrite(b"File read error\0" as *const u8 as *const ::core::ffi::c_char);
            return FIOERR;
        }
        if i != 0 as ::core::ffi::c_int {
            eofflag = TRUE;
        } else {
            return FIOEOF
        }
    }
    *fline.offset(i as isize) = 0 as ::core::ffi::c_char;
    return FIOSUC;
}
#[no_mangle]
pub unsafe extern "C" fn fexist(
    mut fname: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    fp = fopen(fname, b"r\0" as *const u8 as *const ::core::ffi::c_char) as *mut FILE;
    if fp.is_null() {
        return FALSE;
    }
    fclose(fp);
    return TRUE;
}
#[inline]
unsafe extern "C" fn mystrscpy(
    mut dst: *mut ::core::ffi::c_char,
    mut src: *const ::core::ffi::c_char,
    mut size: ::core::ffi::c_int,
) {
    if size == 0 {
        return;
    }
    loop {
        size -= 1;
        if !(size != 0) {
            break;
        }
        let fresh1 = src;
        src = src.offset(1);
        let mut c: ::core::ffi::c_char = *fresh1;
        if c == 0 {
            break;
        }
        let fresh2 = dst;
        dst = dst.offset(1);
        *fresh2 = c;
    }
    *dst = 0 as ::core::ffi::c_char;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
