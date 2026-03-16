extern "C" {
    fn rename(
        __old: *const ::core::ffi::c_char,
        __new: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
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
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn chmod(__file: *const ::core::ffi::c_char, __mode: __mode_t) -> ::core::ffi::c_int;
    static mut curwp: *mut window;
    static mut curbp: *mut buffer;
    static mut bheadp: *mut buffer;
    static mut restflag: ::core::ffi::c_int;
    static mut fline: *mut ::core::ffi::c_char;
    static mut makebackup: ::core::ffi::c_int;
    static mut removebackup: ::core::ffi::c_int;
    fn nanox_set_lamp(state: nanox_lamp_state);
    fn nanox_text_rows() -> ::core::ffi::c_int;
    fn reposition(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn execute(
        c: ::core::ffi::c_int,
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn rdonly() -> ::core::ffi::c_int;
    fn resterr() -> ::core::ffi::c_int;
    fn mlwrite(fmt: *const ::core::ffi::c_char, ...);
    fn mlyesno(prompt: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn swbuffer(bp: *mut buffer) -> ::core::ffi::c_int;
    fn bclear(bp: *mut buffer) -> ::core::ffi::c_int;
    fn bfind(
        bname: *mut ::core::ffi::c_char,
        cflag: ::core::ffi::c_int,
        bflag: ::core::ffi::c_int,
    ) -> *mut buffer;
    fn ffropen(fn_0: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn ffwopen(fn_0: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn ffclose() -> ::core::ffi::c_int;
    fn ffputline(
        buf: *mut ::core::ffi::c_char,
        nbuf: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ffgetline() -> ::core::ffi::c_int;
    fn fexist(fname: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn minibuf_input(
        prompt: *const ::core::ffi::c_char,
        dest: *mut ::core::ffi::c_char,
        max_len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn lockchk(fname: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn lalloc(_: ::core::ffi::c_int) -> *mut line;
}
pub type size_t = usize;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type mode_t = __mode_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::core::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct window {
    pub w_bufp: *mut buffer,
    pub w_linep: *mut line,
    pub w_dotp: *mut line,
    pub w_markp: *mut line,
    pub w_doto: ::core::ffi::c_int,
    pub w_marko: ::core::ffi::c_int,
    pub w_force: ::core::ffi::c_char,
    pub w_flag: ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line {
    pub l_fp: *mut line,
    pub l_bp: *mut line,
    pub l_size: ::core::ffi::c_int,
    pub l_used: ::core::ffi::c_int,
    pub hl_start_state: HighlightState,
    pub hl_end_state: HighlightState,
    pub l_text: [::core::ffi::c_uchar; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HighlightState {
    pub stack: [HighlightStackEntry; 8],
    pub depth: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HighlightStackEntry {
    pub state: StateID,
    pub sub_id: ::core::ffi::c_int,
    pub string_delim: ::core::ffi::c_char,
}
pub type StateID = ::core::ffi::c_uint;
pub const HS_TRIPLE_STRING: StateID = 3;
pub const HS_STRING: StateID = 2;
pub const HS_BLOCK_COMMENT: StateID = 1;
pub const HS_NORMAL: StateID = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer {
    pub b_bufp: *mut buffer,
    pub b_dotp: *mut line,
    pub b_markp: *mut line,
    pub b_linep: *mut line,
    pub b_doto: ::core::ffi::c_int,
    pub b_marko: ::core::ffi::c_int,
    pub b_mode: ::core::ffi::c_int,
    pub b_active: ::core::ffi::c_char,
    pub b_nwnd: ::core::ffi::c_char,
    pub b_flag: ::core::ffi::c_char,
    pub b_fname: [::core::ffi::c_char; 2048],
    pub b_bname: [::core::ffi::c_char; 16],
    pub b_tabsize: ::core::ffi::c_int,
}
pub type nanox_lamp_state = ::core::ffi::c_uint;
pub const NANOX_LAMP_ERROR: nanox_lamp_state = 2;
pub const NANOX_LAMP_WARN: nanox_lamp_state = 1;
pub const NANOX_LAMP_OFF: nanox_lamp_state = 0;
pub const NFILEN: ::core::ffi::c_int = 2048 as ::core::ffi::c_int;
pub const NBUFN: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const NLINE: ::core::ffi::c_int = 2048 as ::core::ffi::c_int;
pub const META: ::core::ffi::c_int = 0x20000000 as ::core::ffi::c_int;
pub const SPEC: ::core::ffi::c_uint = 0x80000000 as ::core::ffi::c_uint;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ABORT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FIOSUC: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FIOFNF: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FIOEOF: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FIOERR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const FIOMEM: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const WFHARD: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const WFMODE: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const BFINVS: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const BFCHG: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const BFTRUNC: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const BFMAKE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const MDCMOD: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const MDVIEW: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const MAXNLINE: ::core::ffi::c_int = 10000000 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn fileread(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut s: ::core::ffi::c_int = 0;
    let mut fname: [::core::ffi::c_char; 2048] = [0; 2048];
    if restflag != 0 {
        return resterr();
    }
    s = minibuf_input(
        b"Read file: \0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut fname as *mut ::core::ffi::c_char,
        NFILEN,
    );
    if s != TRUE {
        return s;
    }
    return readin(&raw mut fname as *mut ::core::ffi::c_char, TRUE);
}
#[no_mangle]
pub unsafe extern "C" fn insfile(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut s: ::core::ffi::c_int = 0;
    let mut fname: [::core::ffi::c_char; 2048] = [0; 2048];
    if restflag != 0 {
        return resterr();
    }
    if (*curbp).b_mode & MDVIEW != 0 {
        return rdonly();
    }
    s = minibuf_input(
        b"Insert file: \0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut fname as *mut ::core::ffi::c_char,
        NFILEN,
    );
    if s != TRUE {
        return s;
    }
    s = ifile(&raw mut fname as *mut ::core::ffi::c_char);
    if s != TRUE {
        return s;
    }
    return reposition(TRUE, -(1 as ::core::ffi::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn filefind(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut fname: [::core::ffi::c_char; 2048] = [0; 2048];
    let mut s: ::core::ffi::c_int = 0;
    if restflag != 0 {
        return resterr();
    }
    s = minibuf_input(
        b"Find file: \0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut fname as *mut ::core::ffi::c_char,
        NFILEN,
    );
    if s != TRUE {
        return s;
    }
    return getfile(&raw mut fname as *mut ::core::ffi::c_char, TRUE);
}
#[no_mangle]
pub unsafe extern "C" fn viewfile(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut fname: [::core::ffi::c_char; 2048] = [0; 2048];
    let mut s: ::core::ffi::c_int = 0;
    if restflag != 0 {
        return resterr();
    }
    s = minibuf_input(
        b"View file: \0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut fname as *mut ::core::ffi::c_char,
        NFILEN,
    );
    if s != TRUE {
        return s;
    }
    s = getfile(&raw mut fname as *mut ::core::ffi::c_char, FALSE);
    if s != 0 {
        (*(*curwp).w_bufp).b_mode |= MDVIEW;
        (*curwp).w_flag = WFMODE as ::core::ffi::c_char;
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn getfile(
    mut fname: *mut ::core::ffi::c_char,
    mut lockfl: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut bp: *mut buffer = ::core::ptr::null_mut::<buffer>();
    let mut lp: *mut line = ::core::ptr::null_mut::<line>();
    let mut i: ::core::ffi::c_int = 0;
    let mut s: ::core::ffi::c_int = 0;
    let mut bname: [::core::ffi::c_char; 16] = [0; 16];
    bp = bheadp;
    while !bp.is_null() {
        if (*bp).b_flag as ::core::ffi::c_int & BFINVS == 0 as ::core::ffi::c_int
            && strcmp(&raw mut (*bp).b_fname as *mut ::core::ffi::c_char, fname)
                == 0 as ::core::ffi::c_int
        {
            swbuffer(bp);
            lp = (*curwp).w_dotp;
            i = nanox_text_rows() / 2 as ::core::ffi::c_int;
            loop {
                let fresh2 = i;
                i = i - 1;
                if !(fresh2 != 0 && (*lp).l_bp != (*curbp).b_linep) {
                    break;
                }
                lp = (*lp).l_bp;
            }
            (*curwp).w_linep = lp as *mut line;
            (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | (WFMODE | WFHARD))
                as ::core::ffi::c_char;
            mlwrite(b"(Old buffer)\0" as *const u8 as *const ::core::ffi::c_char);
            return TRUE;
        }
        bp = (*bp).b_bufp;
    }
    makename(&raw mut bname as *mut ::core::ffi::c_char, fname);
    loop {
        bp = bfind(
            &raw mut bname as *mut ::core::ffi::c_char,
            FALSE,
            0 as ::core::ffi::c_int,
        );
        if bp.is_null() {
            break;
        }
        s = minibuf_input(
            b"Buffer name: \0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut bname as *mut ::core::ffi::c_char,
            NBUFN,
        );
        if s == ABORT {
            return s;
        }
        if !(s == FALSE) {
            continue;
        }
        makename(&raw mut bname as *mut ::core::ffi::c_char, fname);
        break;
    }
    if bp.is_null()
        && {
            bp = bfind(
                &raw mut bname as *mut ::core::ffi::c_char,
                TRUE,
                0 as ::core::ffi::c_int,
            );
            bp.is_null()
        }
    {
        mlwrite(b"Cannot create buffer\0" as *const u8 as *const ::core::ffi::c_char);
        return FALSE;
    }
    (*curbp).b_nwnd -= 1;
    if (*curbp).b_nwnd as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        (*curbp).b_dotp = (*curwp).w_dotp;
        (*curbp).b_doto = (*curwp).w_doto;
        (*curbp).b_markp = (*curwp).w_markp;
        (*curbp).b_marko = (*curwp).w_marko;
    }
    curbp = bp;
    (*curwp).w_bufp = bp as *mut buffer;
    (*curbp).b_nwnd += 1;
    s = readin(fname, lockfl);
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn readin(
    mut fname: *mut ::core::ffi::c_char,
    mut lockfl: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ext: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut bname: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut lp1: *mut line = ::core::ptr::null_mut::<line>();
    let mut lp2: *mut line = ::core::ptr::null_mut::<line>();
    let mut i: ::core::ffi::c_int = 0;
    let mut wp: *mut window = ::core::ptr::null_mut::<window>();
    let mut bp: *mut buffer = ::core::ptr::null_mut::<buffer>();
    let mut s: ::core::ffi::c_int = 0;
    let mut nbytes: ::core::ffi::c_int = 0;
    let mut nline: ::core::ffi::c_int = 0;
    let mut mesg: [::core::ffi::c_char; 1024] = [0; 1024];
    if lockfl != 0 && lockchk(fname) == ABORT {
        s = FIOFNF;
        bp = curbp;
        strcpy(
            &raw mut (*bp).b_fname as *mut ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        bp = curbp;
        s = bclear(bp);
        if s != TRUE {
            return s;
        }
        (*bp).b_flag = ((*bp).b_flag as ::core::ffi::c_int & !(BFINVS | BFCHG))
            as ::core::ffi::c_char;
        mystrscpy(&raw mut (*bp).b_fname as *mut ::core::ffi::c_char, fname, NFILEN);
        ext = strrchr(fname, '.' as i32);
        if !ext.is_null()
            && (strcasecmp(ext, b".c\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
                || strcasecmp(ext, b".h\0" as *const u8 as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                || strcasecmp(ext, b".cpp\0" as *const u8 as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                || strcasecmp(ext, b".hpp\0" as *const u8 as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                || strcasecmp(ext, b".java\0" as *const u8 as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                || strcasecmp(ext, b".js\0" as *const u8 as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                || strcasecmp(ext, b".ts\0" as *const u8 as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                || strcasecmp(ext, b".rs\0" as *const u8 as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                || strcasecmp(ext, b".go\0" as *const u8 as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                || strcasecmp(ext, b".php\0" as *const u8 as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                || strcasecmp(
                    ext,
                    b".swift\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int)
        {
            (*bp).b_mode |= MDCMOD;
        }
        bname = strrchr(fname, '/' as i32);
        if !bname.is_null() {
            bname = bname.offset(1);
        } else {
            bname = fname;
        }
        if strcasecmp(bname, b"makefile\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
            || strncasecmp(
                bname,
                b"makefile.\0" as *const u8 as *const ::core::ffi::c_char,
                9 as size_t,
            ) == 0 as ::core::ffi::c_int
            || !ext.is_null()
                && (strcasecmp(ext, b".md\0" as *const u8 as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcasecmp(
                        ext,
                        b".markdown\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int)
        {
            (*bp).b_tabsize = 0 as ::core::ffi::c_int;
            if strcasecmp(
                bname,
                b"makefile\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                || strncasecmp(
                    bname,
                    b"makefile.\0" as *const u8 as *const ::core::ffi::c_char,
                    9 as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                (*bp).b_flag = ((*bp).b_flag as ::core::ffi::c_int | BFMAKE)
                    as ::core::ffi::c_char;
            }
        }
        execute(
            (META as ::core::ffi::c_uint | SPEC | 'R' as i32 as ::core::ffi::c_uint)
                as ::core::ffi::c_int,
            FALSE,
            1 as ::core::ffi::c_int,
        );
        s = ffropen(fname);
        if s == FIOERR {
            nanox_set_lamp(NANOX_LAMP_ERROR);
        } else if s == FIOFNF {
            mlwrite(b"(New file)\0" as *const u8 as *const ::core::ffi::c_char);
        } else {
            mlwrite(b"(Reading file)\0" as *const u8 as *const ::core::ffi::c_char);
            nline = 0 as ::core::ffi::c_int;
            loop {
                s = ffgetline();
                if !(s == FIOSUC) {
                    break;
                }
                nbytes = strlen(fline) as ::core::ffi::c_int;
                lp1 = lalloc(nbytes);
                if lp1.is_null() {
                    s = FIOMEM;
                    break;
                } else if nline > MAXNLINE {
                    s = FIOMEM;
                    break;
                } else {
                    lp2 = (*(*curbp).b_linep).l_bp;
                    (*lp2).l_fp = lp1;
                    (*lp1).l_fp = (*curbp).b_linep;
                    (*lp1).l_bp = lp2;
                    (*(*curbp).b_linep).l_bp = lp1;
                    i = 0 as ::core::ffi::c_int;
                    while i < nbytes {
                        *(&raw mut (*lp1).l_text as *mut ::core::ffi::c_uchar)
                            .offset(i as isize) = *fline.offset(i as isize)
                            as ::core::ffi::c_uchar;
                        i += 1;
                    }
                    nline += 1;
                }
            }
            ffclose();
            strcpy(
                &raw mut mesg as *mut ::core::ffi::c_char,
                b"(\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if s == FIOERR {
                strcat(
                    &raw mut mesg as *mut ::core::ffi::c_char,
                    b"I/O ERROR, \0" as *const u8 as *const ::core::ffi::c_char,
                );
                (*curbp).b_flag = ((*curbp).b_flag as ::core::ffi::c_int | BFTRUNC)
                    as ::core::ffi::c_char;
            }
            if s == FIOMEM {
                strcat(
                    &raw mut mesg as *mut ::core::ffi::c_char,
                    b"OUT OF MEMORY, \0" as *const u8 as *const ::core::ffi::c_char,
                );
                (*curbp).b_flag = ((*curbp).b_flag as ::core::ffi::c_int | BFTRUNC)
                    as ::core::ffi::c_char;
            }
            sprintf(
                (&raw mut mesg as *mut ::core::ffi::c_char)
                    .offset(
                        (strlen
                            as unsafe extern "C" fn(
                                *const ::core::ffi::c_char,
                            ) -> size_t)(&raw mut mesg as *mut ::core::ffi::c_char)
                            as isize,
                    ) as *mut ::core::ffi::c_char,
                b"Read %d line\0" as *const u8 as *const ::core::ffi::c_char,
                nline,
            );
            if nline != 1 as ::core::ffi::c_int {
                strcat(
                    &raw mut mesg as *mut ::core::ffi::c_char,
                    b"s\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            strcat(
                &raw mut mesg as *mut ::core::ffi::c_char,
                b")\0" as *const u8 as *const ::core::ffi::c_char,
            );
            mlwrite(&raw mut mesg as *mut ::core::ffi::c_char);
            if s == FIOERR || s == FIOMEM {
                nanox_set_lamp(NANOX_LAMP_ERROR);
            } else {
                nanox_set_lamp(NANOX_LAMP_OFF);
            }
        }
    }
    wp = curwp;
    if (*wp).w_bufp == curbp {
        (*wp).w_linep = (*(*curbp).b_linep).l_fp as *mut line;
        (*wp).w_dotp = (*(*curbp).b_linep).l_fp;
        (*wp).w_doto = 0 as ::core::ffi::c_int;
        (*wp).w_markp = ::core::ptr::null_mut::<line>();
        (*wp).w_marko = 0 as ::core::ffi::c_int;
        (*wp).w_flag = ((*wp).w_flag as ::core::ffi::c_int | (WFMODE | WFHARD))
            as ::core::ffi::c_char;
    }
    if s == FIOERR || s == FIOFNF {
        return FALSE;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn makename(
    mut bname: *mut ::core::ffi::c_char,
    mut fname: *mut ::core::ffi::c_char,
) {
    let mut cp1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut cp2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    cp1 = fname.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char;
    while *cp1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        cp1 = cp1.offset(1);
    }
    while cp1
        != fname.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char
        && *cp1.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            != '/' as i32
    {
        cp1 = cp1.offset(-1);
    }
    cp2 = bname.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char;
    while cp2
        != bname.offset((NBUFN - 1 as ::core::ffi::c_int) as isize)
            as *mut ::core::ffi::c_char
        && *cp1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        && *cp1 as ::core::ffi::c_int != ';' as i32
    {
        let fresh3 = cp1;
        cp1 = cp1.offset(1);
        let fresh4 = cp2;
        cp2 = cp2.offset(1);
        *fresh4 = *fresh3;
    }
    *cp2 = 0 as ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn unqname(mut name: *mut ::core::ffi::c_char) {
    let mut sp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    while !bfind(name, 0 as ::core::ffi::c_int, FALSE).is_null() {
        sp = name;
        while *sp != 0 {
            sp = sp.offset(1);
        }
        if sp == name
            || ((*sp.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int)
                < '0' as i32
                || *sp.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                    > '8' as i32)
        {
            let fresh5 = sp;
            sp = sp.offset(1);
            *fresh5 = '0' as i32 as ::core::ffi::c_char;
            *sp = 0 as ::core::ffi::c_char;
        } else {
            sp = sp.offset(-1);
            *sp = (*sp as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                as ::core::ffi::c_char;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn filewrite(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut wp: *mut window = ::core::ptr::null_mut::<window>();
    let mut s: ::core::ffi::c_int = 0;
    let mut fname: [::core::ffi::c_char; 2048] = [0; 2048];
    if restflag != 0 {
        return resterr();
    }
    s = minibuf_input(
        b"Write file: \0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut fname as *mut ::core::ffi::c_char,
        NFILEN,
    );
    if s != TRUE {
        return s;
    }
    s = writeout(&raw mut fname as *mut ::core::ffi::c_char);
    if s == TRUE {
        strcpy(
            &raw mut (*curbp).b_fname as *mut ::core::ffi::c_char,
            &raw mut fname as *mut ::core::ffi::c_char,
        );
        (*curbp).b_flag = ((*curbp).b_flag as ::core::ffi::c_int & !BFCHG)
            as ::core::ffi::c_char;
        wp = curwp;
        if (*wp).w_bufp == curbp {
            (*wp).w_flag = ((*wp).w_flag as ::core::ffi::c_int | WFMODE)
                as ::core::ffi::c_char;
        }
        nanox_set_lamp(NANOX_LAMP_OFF);
    } else {
        nanox_set_lamp(NANOX_LAMP_ERROR);
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn normalize_whitespace(mut bp: *mut buffer) {
    let mut lp: *mut line = ::core::ptr::null_mut::<line>();
    lp = (*(*bp).b_linep).l_fp;
    while lp != (*bp).b_linep {
        let mut len: ::core::ffi::c_int = (*lp).l_used;
        let mut i: ::core::ffi::c_int = len - 1 as ::core::ffi::c_int;
        while i >= 0 as ::core::ffi::c_int
            && (*(&raw mut (*lp).l_text as *mut ::core::ffi::c_uchar).offset(i as isize)
                as ::core::ffi::c_int == ' ' as i32
                || *(&raw mut (*lp).l_text as *mut ::core::ffi::c_uchar)
                    .offset(i as isize) as ::core::ffi::c_int == '\t' as i32)
        {
            i -= 1;
        }
        if (i + 1 as ::core::ffi::c_int) < len {
            (*lp).l_used = i + 1 as ::core::ffi::c_int;
            (*bp).b_flag = ((*bp).b_flag as ::core::ffi::c_int | BFCHG)
                as ::core::ffi::c_char;
        }
        lp = (*lp).l_fp;
    }
}
#[no_mangle]
pub unsafe extern "C" fn filesave(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut s: ::core::ffi::c_int = 0;
    if (*curbp).b_mode & MDVIEW != 0 {
        return rdonly();
    }
    normalize_whitespace(curbp);
    if (*curbp).b_flag as ::core::ffi::c_int & BFCHG == 0 as ::core::ffi::c_int {
        return TRUE;
    }
    if (*curbp).b_fname[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        mlwrite(b"No file name\0" as *const u8 as *const ::core::ffi::c_char);
        nanox_set_lamp(NANOX_LAMP_WARN);
        return FALSE;
    }
    if (*curbp).b_flag as ::core::ffi::c_int & BFTRUNC != 0 as ::core::ffi::c_int {
        if mlyesno(
            b"Truncated file ... write it out\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) == FALSE
        {
            mlwrite(b"(Aborted)\0" as *const u8 as *const ::core::ffi::c_char);
            return FALSE;
        }
    }
    s = writeout(&raw mut (*curbp).b_fname as *mut ::core::ffi::c_char);
    if s == TRUE {
        (*curbp).b_flag = ((*curbp).b_flag as ::core::ffi::c_int & !BFCHG)
            as ::core::ffi::c_char;
        (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | WFMODE)
            as ::core::ffi::c_char;
        nanox_set_lamp(NANOX_LAMP_OFF);
    } else {
        nanox_set_lamp(NANOX_LAMP_ERROR);
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn is_effectively_same(
    mut fname: *mut ::core::ffi::c_char,
    mut bp: *mut buffer,
) -> ::core::ffi::c_int {
    let mut s: ::core::ffi::c_int = 0;
    let mut lp: *mut line = (*(*bp).b_linep).l_fp;
    let mut f_eof: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut b_eof: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if ffropen(fname) != FIOSUC {
        return FALSE;
    }
    loop {
        while lp != (*bp).b_linep && (*lp).l_used == 0 as ::core::ffi::c_int {
            lp = (*lp).l_fp;
        }
        if lp == (*bp).b_linep {
            b_eof = 1 as ::core::ffi::c_int;
        }
        while f_eof == 0 {
            s = ffgetline();
            if s == FIOEOF {
                f_eof = 1 as ::core::ffi::c_int;
                break;
            } else {
                if s != FIOSUC {
                    ffclose();
                    return FALSE;
                }
                if strlen(fline) > 0 as size_t {
                    break;
                }
            }
        }
        if f_eof != 0 && b_eof != 0 {
            ffclose();
            return TRUE;
        }
        if f_eof != 0 || b_eof != 0 {
            ffclose();
            return FALSE;
        }
        if (*lp).l_used != strlen(fline) as ::core::ffi::c_int
            || memcmp(
                &raw mut (*lp).l_text as *mut ::core::ffi::c_uchar
                    as *const ::core::ffi::c_void,
                fline as *const ::core::ffi::c_void,
                (*lp).l_used as size_t,
            ) != 0 as ::core::ffi::c_int
        {
            ffclose();
            return FALSE;
        }
        lp = (*lp).l_fp;
    };
}
#[no_mangle]
pub unsafe extern "C" fn writeout(
    mut fn_0: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut s: ::core::ffi::c_int = 0;
    let mut lp: *mut line = ::core::ptr::null_mut::<line>();
    let mut nline: ::core::ffi::c_int = 0;
    let mut sb: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut file_mode: mode_t = 0o644 as mode_t;
    let mut backupName: [::core::ffi::c_char; 2048] = [0; 2048];
    let mut backupCreated: ::core::ffi::c_int = FALSE;
    if fexist(fn_0) != 0 {
        if stat(fn_0, &raw mut sb) == 0 as ::core::ffi::c_int {
            file_mode = sb.st_mode as mode_t;
        }
        if makebackup != 0 {
            if strlen(fn_0).wrapping_add(2 as size_t) < NFILEN as size_t {
                strcpy(&raw mut backupName as *mut ::core::ffi::c_char, fn_0);
                strcat(
                    &raw mut backupName as *mut ::core::ffi::c_char,
                    b"~\0" as *const u8 as *const ::core::ffi::c_char,
                );
                unlink(&raw mut backupName as *mut ::core::ffi::c_char);
                if rename(fn_0, &raw mut backupName as *mut ::core::ffi::c_char)
                    != 0 as ::core::ffi::c_int
                {
                    mlwrite(
                        b"(Cannot create backup)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    nanox_set_lamp(NANOX_LAMP_ERROR);
                    return FALSE;
                }
                backupCreated = TRUE;
            }
        }
    }
    s = ffwopen(fn_0);
    if s != FIOSUC {
        nanox_set_lamp(NANOX_LAMP_ERROR);
        return FALSE;
    }
    mlwrite(b"(Writing...)\0" as *const u8 as *const ::core::ffi::c_char);
    lp = (*(*curbp).b_linep).l_fp;
    nline = 0 as ::core::ffi::c_int;
    while lp != (*curbp).b_linep {
        let mut len: ::core::ffi::c_int = (*lp).l_used;
        let mut clean_buf: [::core::ffi::c_uchar; 2048] = [0; 2048];
        let mut clean_idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < len && clean_idx < NLINE - 1 as ::core::ffi::c_int {
            let mut c: ::core::ffi::c_uchar = *(&raw mut (*lp).l_text
                as *mut ::core::ffi::c_uchar)
                .offset(i as isize);
            if c as ::core::ffi::c_int >= 32 as ::core::ffi::c_int
                || c as ::core::ffi::c_int == '\t' as i32
                || c as ::core::ffi::c_int >= 0x80 as ::core::ffi::c_int
            {
                let fresh6 = clean_idx;
                clean_idx = clean_idx + 1;
                clean_buf[fresh6 as usize] = c;
            }
            i += 1;
        }
        while clean_idx > 0 as ::core::ffi::c_int
            && (clean_buf[(clean_idx - 1 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int == ' ' as i32
                || clean_buf[(clean_idx - 1 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int == '\t' as i32)
        {
            clean_idx -= 1;
        }
        s = ffputline(
            &raw mut clean_buf as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
            clean_idx,
        );
        if s != FIOSUC {
            break;
        }
        nline += 1;
        lp = (*lp).l_fp;
    }
    if s == FIOSUC {
        s = ffclose();
        if s == FIOSUC {
            chmod(fn_0, file_mode as __mode_t);
            if backupCreated != 0
                && (removebackup != 0
                    || is_effectively_same(
                        &raw mut backupName as *mut ::core::ffi::c_char,
                        curbp,
                    ) != 0)
            {
                unlink(&raw mut backupName as *mut ::core::ffi::c_char);
            }
            if nline == 1 as ::core::ffi::c_int {
                mlwrite(b"(Wrote 1 line)\0" as *const u8 as *const ::core::ffi::c_char);
            } else {
                mlwrite(
                    b"(Wrote %d lines)\0" as *const u8 as *const ::core::ffi::c_char,
                    nline,
                );
            }
        }
    } else {
        ffclose();
    }
    if s != FIOSUC {
        nanox_set_lamp(NANOX_LAMP_ERROR);
        return FALSE;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn filename(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut s: ::core::ffi::c_int = 0;
    let mut fname: [::core::ffi::c_char; 2048] = [0; 2048];
    if restflag != 0 {
        return resterr();
    }
    s = minibuf_input(
        b"Name: \0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut fname as *mut ::core::ffi::c_char,
        NFILEN,
    );
    if s == ABORT {
        return s;
    }
    if s == FALSE {
        strcpy(
            &raw mut (*curbp).b_fname as *mut ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        strcpy(
            &raw mut (*curbp).b_fname as *mut ::core::ffi::c_char,
            &raw mut fname as *mut ::core::ffi::c_char,
        );
    }
    (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | WFMODE)
        as ::core::ffi::c_char;
    (*curbp).b_mode &= !MDVIEW;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn ifile(
    mut fname: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut lp0: *mut line = ::core::ptr::null_mut::<line>();
    let mut lp1: *mut line = ::core::ptr::null_mut::<line>();
    let mut lp2: *mut line = ::core::ptr::null_mut::<line>();
    let mut i: ::core::ffi::c_int = 0;
    let mut bp: *mut buffer = ::core::ptr::null_mut::<buffer>();
    let mut s: ::core::ffi::c_int = 0;
    let mut nbytes: ::core::ffi::c_int = 0;
    let mut nline: ::core::ffi::c_int = 0;
    let mut mesg: [::core::ffi::c_char; 1024] = [0; 1024];
    bp = curbp;
    (*bp).b_flag = ((*bp).b_flag as ::core::ffi::c_int | BFCHG) as ::core::ffi::c_char;
    (*bp).b_flag = ((*bp).b_flag as ::core::ffi::c_int & !BFINVS) as ::core::ffi::c_char;
    s = ffropen(fname);
    if !(s == FIOERR) {
        if s == FIOFNF {
            mlwrite(b"(No such file)\0" as *const u8 as *const ::core::ffi::c_char);
            nanox_set_lamp(NANOX_LAMP_ERROR);
            return FALSE;
        }
        mlwrite(b"(Inserting file)\0" as *const u8 as *const ::core::ffi::c_char);
        (*curwp).w_dotp = (*(*curwp).w_dotp).l_bp;
        (*curwp).w_doto = 0 as ::core::ffi::c_int;
        (*curwp).w_markp = (*curwp).w_dotp;
        (*curwp).w_marko = 0 as ::core::ffi::c_int;
        nline = 0 as ::core::ffi::c_int;
        loop {
            s = ffgetline();
            if !(s == FIOSUC) {
                break;
            }
            nbytes = strlen(fline) as ::core::ffi::c_int;
            lp1 = lalloc(nbytes);
            if lp1.is_null() {
                s = FIOMEM;
                break;
            } else {
                lp0 = (*curwp).w_dotp;
                lp2 = (*lp0).l_fp;
                (*lp2).l_bp = lp1;
                (*lp0).l_fp = lp1;
                (*lp1).l_bp = lp0;
                (*lp1).l_fp = lp2;
                (*curwp).w_dotp = lp1;
                i = 0 as ::core::ffi::c_int;
                while i < nbytes {
                    *(&raw mut (*lp1).l_text as *mut ::core::ffi::c_uchar)
                        .offset(i as isize) = *fline.offset(i as isize)
                        as ::core::ffi::c_uchar;
                    i += 1;
                }
                nline += 1;
            }
        }
        ffclose();
        (*curwp).w_markp = (*(*curwp).w_markp).l_fp;
        strcpy(
            &raw mut mesg as *mut ::core::ffi::c_char,
            b"(\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if s == FIOERR {
            strcat(
                &raw mut mesg as *mut ::core::ffi::c_char,
                b"I/O ERROR, \0" as *const u8 as *const ::core::ffi::c_char,
            );
            (*curbp).b_flag = ((*curbp).b_flag as ::core::ffi::c_int | BFTRUNC)
                as ::core::ffi::c_char;
        }
        if s == FIOMEM {
            strcat(
                &raw mut mesg as *mut ::core::ffi::c_char,
                b"OUT OF MEMORY, \0" as *const u8 as *const ::core::ffi::c_char,
            );
            (*curbp).b_flag = ((*curbp).b_flag as ::core::ffi::c_int | BFTRUNC)
                as ::core::ffi::c_char;
        }
        sprintf(
            (&raw mut mesg as *mut ::core::ffi::c_char)
                .offset(
                    (strlen
                        as unsafe extern "C" fn(
                            *const ::core::ffi::c_char,
                        ) -> size_t)(&raw mut mesg as *mut ::core::ffi::c_char) as isize,
                ) as *mut ::core::ffi::c_char,
            b"Inserted %d line\0" as *const u8 as *const ::core::ffi::c_char,
            nline,
        );
        if nline > 1 as ::core::ffi::c_int {
            strcat(
                &raw mut mesg as *mut ::core::ffi::c_char,
                b"s\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        strcat(
            &raw mut mesg as *mut ::core::ffi::c_char,
            b")\0" as *const u8 as *const ::core::ffi::c_char,
        );
        mlwrite(&raw mut mesg as *mut ::core::ffi::c_char);
        nanox_set_lamp(NANOX_LAMP_OFF);
    }
    (*curwp).w_dotp = (*(*curwp).w_dotp).l_fp;
    (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | (WFHARD | WFMODE))
        as ::core::ffi::c_char;
    (*curbp).b_dotp = (*curwp).w_dotp;
    (*curbp).b_doto = (*curwp).w_doto;
    (*curbp).b_markp = (*curwp).w_markp;
    (*curbp).b_marko = (*curwp).w_marko;
    if s == FIOERR {
        return FALSE;
    }
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
        let fresh0 = src;
        src = src.offset(1);
        let mut c: ::core::ffi::c_char = *fresh0;
        if c == 0 {
            break;
        }
        let fresh1 = dst;
        dst = dst.offset(1);
        *fresh1 = c;
    }
    *dst = 0 as ::core::ffi::c_char;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
