extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type line;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn mkstemp(__template: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn system(__command: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcat(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sleep(__seconds: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    static mut term: *mut terminal;
    fn vttopen();
    fn vttclose();
    fn vttkopen();
    fn vttkclose();
    fn vttputc(c: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn vttflush();
    static mut curwp: *mut window;
    static mut curbp: *mut buffer;
    static mut restflag: ::core::ffi::c_int;
    static mut sgarbf: ::core::ffi::c_int;
    static mut clexec: ::core::ffi::c_int;
    static mut confirmshell: ::core::ffi::c_int;
    static mut chg_width: ::core::ffi::c_int;
    static mut chg_height: ::core::ffi::c_int;
    fn rdonly() -> ::core::ffi::c_int;
    fn resterr() -> ::core::ffi::c_int;
    fn vttidy();
    fn movecursor(row: ::core::ffi::c_int, col: ::core::ffi::c_int);
    fn mlwrite(fmt: *const ::core::ffi::c_char, ...);
    fn mlputs(s: *mut ::core::ffi::c_char);
    fn mlyesno(prompt: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn tgetc() -> ::core::ffi::c_int;
    fn readin(
        fname: *mut ::core::ffi::c_char,
        lockfl: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn writeout(fn_0: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn minibuf_input(
        prompt: *const ::core::ffi::c_char,
        dest: *mut ::core::ffi::c_char,
        max_len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn kill(__pid: __pid_t, __sig: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct terminal {
    pub t_mrow: ::core::ffi::c_short,
    pub t_nrow: ::core::ffi::c_short,
    pub t_mcol: ::core::ffi::c_short,
    pub t_ncol: ::core::ffi::c_short,
    pub t_margin: ::core::ffi::c_short,
    pub t_scrsiz: ::core::ffi::c_short,
    pub t_pause: ::core::ffi::c_int,
    pub t_open: Option<unsafe extern "C" fn() -> ()>,
    pub t_close: Option<unsafe extern "C" fn() -> ()>,
    pub t_kopen: Option<unsafe extern "C" fn() -> ()>,
    pub t_kclose: Option<unsafe extern "C" fn() -> ()>,
    pub t_getchar: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub t_putchar: Option<
        unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
    pub t_flush: Option<unsafe extern "C" fn() -> ()>,
    pub t_move: Option<
        unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int) -> (),
    >,
    pub t_eeol: Option<unsafe extern "C" fn() -> ()>,
    pub t_eeop: Option<unsafe extern "C" fn() -> ()>,
    pub t_beep: Option<unsafe extern "C" fn() -> ()>,
    pub t_rev: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>,
    pub t_italic: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>,
    pub t_set_colors: Option<
        unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int) -> (),
    >,
    pub t_set_attrs: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub t_rez: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ::core::ffi::c_int,
    >,
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
pub const SIGTSTP: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const NLINE: ::core::ffi::c_int = 2048 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WFHARD: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const BFCHG: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MDVIEW: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn spawncli(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut cp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    if restflag != 0 {
        return resterr();
    }
    if confirmshell != 0 {
        if mlyesno(
            b"Spawn shell\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        ) != TRUE
        {
            mlwrite(b"(Aborted)\0" as *const u8 as *const ::core::ffi::c_char);
            return FALSE;
        }
    }
    movecursor((*term).t_nrow as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    vttflush();
    vttclose();
    vttkclose();
    cp = getenv(b"SHELL\0" as *const u8 as *const ::core::ffi::c_char);
    if !cp.is_null() && *cp as ::core::ffi::c_int != '\0' as i32 {
        system(cp);
    } else {
        system(b"exec /bin/sh\0" as *const u8 as *const ::core::ffi::c_char);
    }
    sgarbf = TRUE;
    sleep(2 as ::core::ffi::c_uint);
    vttopen();
    vttkopen();
    chg_width = (*term).t_ncol as ::core::ffi::c_int;
    chg_height = (*term).t_nrow as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
    (*term).t_ncol = 0 as ::core::ffi::c_short;
    (*term).t_nrow = (*term).t_ncol;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn bktoshell(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    vttidy();
    kill(0 as __pid_t, SIGTSTP);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn rtfrmshell() {
    vttopen();
    (*curwp).w_flag = WFHARD as ::core::ffi::c_char;
    sgarbf = TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn spawn(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut s: ::core::ffi::c_int = 0;
    let mut line: [::core::ffi::c_char; 2048] = [0; 2048];
    if restflag != 0 {
        return resterr();
    }
    s = minibuf_input(
        b"!\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut line as *mut ::core::ffi::c_char,
        NLINE,
    );
    if s != TRUE {
        return s;
    }
    if confirmshell != 0 {
        if mlyesno(
            b"Execute command\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        ) != TRUE
        {
            mlwrite(b"(Aborted)\0" as *const u8 as *const ::core::ffi::c_char);
            return FALSE;
        }
    }
    vttflush();
    vttclose();
    vttkclose();
    system(&raw mut line as *mut ::core::ffi::c_char);
    fflush(stdout);
    vttopen();
    if clexec == FALSE {
        mlputs(
            b"(End)\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        vttflush();
        loop {
            s = tgetc();
            if !(s != '\r' as i32 && s != ' ' as i32) {
                break;
            }
        }
        mlputs(
            b"\r\n\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
    }
    vttkopen();
    sgarbf = TRUE;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn execprg(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut s: ::core::ffi::c_int = 0;
    let mut line: [::core::ffi::c_char; 2048] = [0; 2048];
    if restflag != 0 {
        return resterr();
    }
    s = minibuf_input(
        b"!\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut line as *mut ::core::ffi::c_char,
        NLINE,
    );
    if s != TRUE {
        return s;
    }
    if confirmshell != 0 {
        if mlyesno(
            b"Execute program\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        ) != TRUE
        {
            mlwrite(b"(Aborted)\0" as *const u8 as *const ::core::ffi::c_char);
            return FALSE;
        }
    }
    vttputc('\n' as i32);
    vttflush();
    vttclose();
    vttkclose();
    system(&raw mut line as *mut ::core::ffi::c_char);
    fflush(stdout);
    vttopen();
    mlputs(
        b"(End)\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    vttflush();
    loop {
        s = tgetc();
        if !(s != '\r' as i32 && s != ' ' as i32) {
            break;
        }
    }
    sgarbf = TRUE;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn filter_buffer(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut s: ::core::ffi::c_int = 0;
    let mut bp: *mut buffer = ::core::ptr::null_mut::<buffer>();
    let mut line: [::core::ffi::c_char; 2048] = [0; 2048];
    let mut tmpnam: [::core::ffi::c_char; 2048] = [0; 2048];
    let mut filnam1: [::core::ffi::c_char; 25] = ::core::mem::transmute::<
        [u8; 25],
        [::core::ffi::c_char; 25],
    >(*b"/tmp/nanox_fltinp_XXXXXX\0");
    let mut filnam2: [::core::ffi::c_char; 25] = ::core::mem::transmute::<
        [u8; 25],
        [::core::ffi::c_char; 25],
    >(*b"/tmp/nanox_fltout_XXXXXX\0");
    let mut fd: ::core::ffi::c_int = 0;
    if restflag != 0 {
        return resterr();
    }
    if (*curbp).b_mode & MDVIEW != 0 {
        return rdonly();
    }
    s = minibuf_input(
        b"#\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut line as *mut ::core::ffi::c_char,
        NLINE,
    );
    if s != TRUE {
        return s;
    }
    if confirmshell != 0 {
        if mlyesno(
            b"Filter buffer\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        ) != TRUE
        {
            mlwrite(b"(Aborted)\0" as *const u8 as *const ::core::ffi::c_char);
            return FALSE;
        }
    }
    fd = mkstemp(&raw mut filnam1 as *mut ::core::ffi::c_char);
    if fd == -(1 as ::core::ffi::c_int) {
        mlwrite(b"Cannot create temp file\0" as *const u8 as *const ::core::ffi::c_char);
        return FALSE;
    }
    close(fd);
    fd = mkstemp(&raw mut filnam2 as *mut ::core::ffi::c_char);
    if fd == -(1 as ::core::ffi::c_int) {
        mlwrite(b"Cannot create temp file\0" as *const u8 as *const ::core::ffi::c_char);
        unlink(&raw mut filnam1 as *mut ::core::ffi::c_char);
        return FALSE;
    }
    close(fd);
    bp = curbp;
    strcpy(
        &raw mut tmpnam as *mut ::core::ffi::c_char,
        &raw mut (*bp).b_fname as *mut ::core::ffi::c_char,
    );
    strcpy(
        &raw mut (*bp).b_fname as *mut ::core::ffi::c_char,
        &raw mut filnam1 as *mut ::core::ffi::c_char,
    );
    if writeout(&raw mut filnam1 as *mut ::core::ffi::c_char) != TRUE {
        mlwrite(
            b"(Cannot write filter file)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        strcpy(
            &raw mut (*bp).b_fname as *mut ::core::ffi::c_char,
            &raw mut tmpnam as *mut ::core::ffi::c_char,
        );
        unlink(&raw mut filnam1 as *mut ::core::ffi::c_char);
        unlink(&raw mut filnam2 as *mut ::core::ffi::c_char);
        return FALSE;
    }
    vttputc('\n' as i32);
    vttflush();
    vttclose();
    vttkclose();
    if strlen(&raw mut line as *mut ::core::ffi::c_char)
        .wrapping_add(strlen(&raw mut filnam1 as *mut ::core::ffi::c_char))
        .wrapping_add(strlen(&raw mut filnam2 as *mut ::core::ffi::c_char))
        .wrapping_add(10 as size_t) < NLINE as size_t
    {
        strcat(
            &raw mut line as *mut ::core::ffi::c_char,
            b" <\0" as *const u8 as *const ::core::ffi::c_char,
        );
        strcat(
            &raw mut line as *mut ::core::ffi::c_char,
            &raw mut filnam1 as *mut ::core::ffi::c_char,
        );
        strcat(
            &raw mut line as *mut ::core::ffi::c_char,
            b" >\0" as *const u8 as *const ::core::ffi::c_char,
        );
        strcat(
            &raw mut line as *mut ::core::ffi::c_char,
            &raw mut filnam2 as *mut ::core::ffi::c_char,
        );
        system(&raw mut line as *mut ::core::ffi::c_char);
    } else {
        printf(b"Command too long\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
    vttopen();
    vttkopen();
    vttflush();
    sgarbf = TRUE;
    s = TRUE;
    if s != TRUE || readin(&raw mut filnam2 as *mut ::core::ffi::c_char, FALSE) == FALSE
    {
        mlwrite(b"(Execution failed)\0" as *const u8 as *const ::core::ffi::c_char);
        strcpy(
            &raw mut (*bp).b_fname as *mut ::core::ffi::c_char,
            &raw mut tmpnam as *mut ::core::ffi::c_char,
        );
        unlink(&raw mut filnam1 as *mut ::core::ffi::c_char);
        unlink(&raw mut filnam2 as *mut ::core::ffi::c_char);
        return s;
    }
    strcpy(
        &raw mut (*bp).b_fname as *mut ::core::ffi::c_char,
        &raw mut tmpnam as *mut ::core::ffi::c_char,
    );
    (*bp).b_flag = ((*bp).b_flag as ::core::ffi::c_int | BFCHG) as ::core::ffi::c_char;
    unlink(&raw mut filnam1 as *mut ::core::ffi::c_char);
    unlink(&raw mut filnam2 as *mut ::core::ffi::c_char);
    return TRUE;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
