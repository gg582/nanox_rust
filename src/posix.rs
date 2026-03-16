extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn fcntl(
        __fd: ::core::ffi::c_int,
        __cmd: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn read(
        __fd: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_void,
        __nbytes: size_t,
    ) -> ssize_t;
    fn write(
        __fd: ::core::ffi::c_int,
        __buf: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ssize_t;
    fn sleep(__seconds: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn setbuffer(__stream: *mut FILE, __buf: *mut ::core::ffi::c_char, __size: size_t);
    fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn tcgetattr(
        __fd: ::core::ffi::c_int,
        __termios_p: *mut termios,
    ) -> ::core::ffi::c_int;
    fn tcsetattr(
        __fd: ::core::ffi::c_int,
        __optional_actions: ::core::ffi::c_int,
        __termios_p: *const termios,
    ) -> ::core::ffi::c_int;
    fn ioctl(
        __fd: ::core::ffi::c_int,
        __request: ::core::ffi::c_ulong,
        ...
    ) -> ::core::ffi::c_int;
    fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> ::core::ffi::c_int;
    static mut term: *mut terminal;
    static mut ttrow: ::core::ffi::c_int;
    static mut ttcol: ::core::ffi::c_int;
    fn paste_slot_init();
    fn paste_slot_add_char(c: ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn paste_slot_clear();
    fn paste_slot_set_active(active: ::core::ffi::c_int);
    fn paste_slot_display();
    fn utf8_to_unicode(
        line: *mut ::core::ffi::c_uchar,
        index: ::core::ffi::c_uint,
        len: ::core::ffi::c_uint,
        res: *mut unicode_t,
    ) -> ::core::ffi::c_uint;
    fn unicode_to_utf8(
        c: ::core::ffi::c_uint,
        utf8: *mut ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_uint;
}
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type ssize_t = __ssize_t;
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
pub type cc_t = ::core::ffi::c_uchar;
pub type speed_t = ::core::ffi::c_uint;
pub type tcflag_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 32],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
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
pub struct C2RustUnnamed {
    pub nr: ::core::ffi::c_int,
    pub buf: [::core::ffi::c_char; 32],
}
pub type unicode_t = ::core::ffi::c_uint;
pub const O_NONBLOCK: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const F_GETFL: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const F_SETFL: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const VTIME: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const VMIN: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const IGNBRK: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const BRKINT: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const IGNPAR: ::core::ffi::c_int = 0o4 as ::core::ffi::c_int;
pub const PARMRK: ::core::ffi::c_int = 0o10 as ::core::ffi::c_int;
pub const INPCK: ::core::ffi::c_int = 0o20 as ::core::ffi::c_int;
pub const INLCR: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const IGNCR: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const ICRNL: ::core::ffi::c_int = 0o400 as ::core::ffi::c_int;
pub const IXON: ::core::ffi::c_int = 0o2000 as ::core::ffi::c_int;
pub const IXOFF: ::core::ffi::c_int = 0o10000 as ::core::ffi::c_int;
pub const OPOST: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const OLCUC: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const ONLCR: ::core::ffi::c_int = 0o4 as ::core::ffi::c_int;
pub const OCRNL: ::core::ffi::c_int = 0o10 as ::core::ffi::c_int;
pub const ONOCR: ::core::ffi::c_int = 0o20 as ::core::ffi::c_int;
pub const ONLRET: ::core::ffi::c_int = 0o40 as ::core::ffi::c_int;
pub const ISIG: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const ICANON: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const XCASE: ::core::ffi::c_int = 0o4 as ::core::ffi::c_int;
pub const ECHO: ::core::ffi::c_int = 0o10 as ::core::ffi::c_int;
pub const ECHOE: ::core::ffi::c_int = 0o20 as ::core::ffi::c_int;
pub const ECHOK: ::core::ffi::c_int = 0o40 as ::core::ffi::c_int;
pub const ECHONL: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const NOFLSH: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const TOSTOP: ::core::ffi::c_int = 0o400 as ::core::ffi::c_int;
pub const ECHOCTL: ::core::ffi::c_int = 0o1000 as ::core::ffi::c_int;
pub const ECHOPRT: ::core::ffi::c_int = 0o2000 as ::core::ffi::c_int;
pub const ECHOKE: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const FLUSHO: ::core::ffi::c_int = 0o10000 as ::core::ffi::c_int;
pub const PENDIN: ::core::ffi::c_int = 0o40000 as ::core::ffi::c_int;
pub const IEXTEN: ::core::ffi::c_int = 0o100000 as ::core::ffi::c_int;
pub const TCSANOW: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TCSADRAIN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FIONREAD: ::core::ffi::c_int = 0x541b as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut kbdflgs: ::core::ffi::c_int = 0;
static mut kbdpoll: ::core::ffi::c_int = 0;
static mut otermios: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
};
static mut ntermios: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
};
pub const TBUFSIZ: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
static mut tobuf: [::core::ffi::c_char; 128] = [0; 128];
#[no_mangle]
pub unsafe extern "C" fn ttopen() {
    tcgetattr(0 as ::core::ffi::c_int, &raw mut otermios);
    ntermios = otermios;
    ntermios.c_iflag
        &= !(IGNBRK | BRKINT | IGNPAR | PARMRK | INPCK | INLCR | IGNCR | ICRNL | IXON
            | IXOFF) as tcflag_t;
    ntermios.c_oflag &= !(OPOST | ONLCR | OLCUC | OCRNL | ONOCR | ONLRET) as tcflag_t;
    ntermios.c_lflag
        &= !(ISIG | ICANON | XCASE | ECHO | ECHOE | ECHOK | ECHONL | NOFLSH | TOSTOP
            | ECHOCTL | ECHOPRT | ECHOKE | FLUSHO | PENDIN | IEXTEN) as tcflag_t;
    ntermios.c_cc[VMIN as usize] = 1 as cc_t;
    ntermios.c_cc[VTIME as usize] = 0 as cc_t;
    tcsetattr(0 as ::core::ffi::c_int, TCSADRAIN, &raw mut ntermios);
    setbuffer(
        stdout,
        (&raw mut tobuf as *mut ::core::ffi::c_char)
            .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
        TBUFSIZ as size_t,
    );
    kbdflgs = fcntl(0 as ::core::ffi::c_int, F_GETFL, 0 as ::core::ffi::c_int);
    kbdpoll = FALSE;
    ttrow = 999 as ::core::ffi::c_int;
    ttcol = 999 as ::core::ffi::c_int;
    write(
        1 as ::core::ffi::c_int,
        b"\x1B[?2004h\0" as *const u8 as *const ::core::ffi::c_char
            as *const ::core::ffi::c_void,
        8 as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ttclose() {
    write(
        1 as ::core::ffi::c_int,
        b"\x1B[?2004l\0" as *const u8 as *const ::core::ffi::c_char
            as *const ::core::ffi::c_void,
        8 as size_t,
    );
    tcsetattr(0 as ::core::ffi::c_int, TCSADRAIN, &raw mut otermios);
}
#[no_mangle]
pub unsafe extern "C" fn ttputc(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut utf8: [::core::ffi::c_char; 6] = [0; 6];
    let mut bytes: ::core::ffi::c_int = 0;
    bytes = unicode_to_utf8(
        c as ::core::ffi::c_uint,
        &raw mut utf8 as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
    ) as ::core::ffi::c_int;
    fwrite(
        &raw mut utf8 as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        1 as size_t,
        bytes as size_t,
        stdout,
    );
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ttflush() {
    let mut status: ::core::ffi::c_int = 0;
    status = fflush(stdout);
    while status < 0 as ::core::ffi::c_int && *__errno_location() == EAGAIN {
        sleep(1 as ::core::ffi::c_uint);
        status = fflush(stdout);
    }
    if status < 0 as ::core::ffi::c_int {
        exit(15 as ::core::ffi::c_int);
    }
}
static mut TT: C2RustUnnamed = C2RustUnnamed {
    nr: 0,
    buf: [0; 32],
};
unsafe extern "C" fn pause_read(mut pause: ::core::ffi::c_int) {
    let mut n: ::core::ffi::c_int = 0;
    ntermios.c_cc[VMIN as usize] = 0 as cc_t;
    ntermios.c_cc[VTIME as usize] = pause as cc_t;
    tcsetattr(0 as ::core::ffi::c_int, TCSANOW, &raw mut ntermios);
    n = read(
        0 as ::core::ffi::c_int,
        (&raw mut TT.buf as *mut ::core::ffi::c_char).offset(TT.nr as isize)
            as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 32]>() as size_t)
            .wrapping_sub(TT.nr as size_t),
    ) as ::core::ffi::c_int;
    ntermios.c_cc[VMIN as usize] = 1 as cc_t;
    ntermios.c_cc[VTIME as usize] = 0 as cc_t;
    tcsetattr(0 as ::core::ffi::c_int, TCSANOW, &raw mut ntermios);
    if n > 0 as ::core::ffi::c_int {
        TT.nr += n;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ttpause() {
    if (*term).t_pause != 0 && TT.nr == 0 {
        pause_read((*term).t_pause);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ttgetc() -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut c: unicode_t = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut bytes: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut expected: ::core::ffi::c_int = 0;
    count = TT.nr;
    if count == 0 {
        count = read(
            0 as ::core::ffi::c_int,
            &raw mut TT.buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>() as size_t,
        ) as ::core::ffi::c_int;
        if count <= 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        TT.nr = count;
    }
    if handle_bracketed_paste() != 0 {
        return 0 as ::core::ffi::c_int;
    }
    c = TT.buf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_uchar as unicode_t;
    if !(c != 27 as unicode_t && c < 128 as unicode_t) {
        expected = 2 as ::core::ffi::c_int;
        if c & 0xe0 as unicode_t == 0xe0 as unicode_t {
            expected = 6 as ::core::ffi::c_int;
        }
        if count < expected {
            pause_read(1 as ::core::ffi::c_int);
        }
        if c == 27 as unicode_t && TT.nr > 1 as ::core::ffi::c_int {
            let mut second: ::core::ffi::c_uchar = TT
                .buf[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_uchar;
            if second as ::core::ffi::c_int == '[' as i32
                || second as ::core::ffi::c_int == 'O' as i32
            {
                let mut timeout: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
                loop {
                    let fresh0 = timeout;
                    timeout = timeout - 1;
                    if !(fresh0 != 0) {
                        break;
                    }
                    let mut last: ::core::ffi::c_uchar = TT
                        .buf[(TT.nr - 1 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_uchar;
                    if second as ::core::ffi::c_int == '[' as i32
                        && TT.nr >= 3 as ::core::ffi::c_int
                        && last as ::core::ffi::c_int >= 0x40 as ::core::ffi::c_int
                        && last as ::core::ffi::c_int <= 0x7e as ::core::ffi::c_int
                    {
                        break;
                    }
                    if second as ::core::ffi::c_int == 'O' as i32
                        && TT.nr >= 3 as ::core::ffi::c_int
                    {
                        break;
                    }
                    pause_read(1 as ::core::ffi::c_int);
                }
                if second as ::core::ffi::c_int == '[' as i32
                    || second as ::core::ffi::c_int == 'O' as i32
                        && TT.nr >= 3 as ::core::ffi::c_int
                {
                    bytes = 2 as ::core::ffi::c_int;
                    c = (128 as ::core::ffi::c_int + 27 as ::core::ffi::c_int)
                        as unicode_t;
                    current_block = 9977722817845244874;
                } else {
                    current_block = 11307063007268554308;
                }
            } else {
                current_block = 11307063007268554308;
            }
        } else {
            current_block = 11307063007268554308;
        }
        match current_block {
            9977722817845244874 => {}
            _ => {
                bytes = utf8_to_unicode(
                    &raw mut TT.buf as *mut ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    0 as ::core::ffi::c_uint,
                    TT.nr as ::core::ffi::c_uint,
                    &raw mut c,
                ) as ::core::ffi::c_int;
                if c == 0xa0 as unicode_t {
                    c = ' ' as i32 as unicode_t;
                }
            }
        }
    }
    TT.nr -= bytes;
    memmove(
        &raw mut TT.buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (&raw mut TT.buf as *mut ::core::ffi::c_char).offset(bytes as isize)
            as *const ::core::ffi::c_void,
        TT.nr as size_t,
    );
    return c as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn typahead() -> ::core::ffi::c_int {
    let mut x: ::core::ffi::c_int = 0;
    if ioctl(0 as ::core::ffi::c_int, FIONREAD as ::core::ffi::c_ulong, &raw mut x)
        < 0 as ::core::ffi::c_int
    {
        x = 0 as ::core::ffi::c_int;
    }
    return x + TT.nr;
}
unsafe extern "C" fn handle_bracketed_paste() -> ::core::ffi::c_int {
    if TT.nr < 6 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if TT.buf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        != 27 as ::core::ffi::c_int
        || TT.buf[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != '[' as i32
        || TT.buf[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != '2' as i32
        || TT.buf[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != '0' as i32
        || TT.buf[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != '0' as i32
        || TT.buf[5 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != '~' as i32
    {
        return 0 as ::core::ffi::c_int;
    }
    TT.nr -= 6 as ::core::ffi::c_int;
    memmove(
        &raw mut TT.buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (&raw mut TT.buf as *mut ::core::ffi::c_char)
            .offset(6 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
        TT.nr as size_t,
    );
    paste_slot_init();
    paste_slot_clear();
    let mut paste_active: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while paste_active != 0 {
        if TT.nr == 0 as ::core::ffi::c_int {
            fcntl(0 as ::core::ffi::c_int, F_SETFL, kbdflgs | O_NONBLOCK);
            let mut ts: timespec = timespec {
                tv_sec: 0 as __time_t,
                tv_nsec: 10000000 as __syscall_slong_t,
            };
            nanosleep(&raw mut ts, ::core::ptr::null_mut::<timespec>());
            let mut count: ::core::ffi::c_int = read(
                0 as ::core::ffi::c_int,
                (&raw mut TT.buf as *mut ::core::ffi::c_char).offset(TT.nr as isize)
                    as *mut ::core::ffi::c_void,
                (::core::mem::size_of::<[::core::ffi::c_char; 32]>() as size_t)
                    .wrapping_sub(TT.nr as size_t),
            ) as ::core::ffi::c_int;
            fcntl(0 as ::core::ffi::c_int, F_SETFL, kbdflgs);
            if count <= 0 as ::core::ffi::c_int {
                paste_active = 0 as ::core::ffi::c_int;
                break;
            } else {
                TT.nr += count;
            }
        }
        if TT.nr == 0 as ::core::ffi::c_int {
            break;
        }
        if TT.nr >= 6 as ::core::ffi::c_int
            && TT.buf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == 27 as ::core::ffi::c_int
            && TT.buf[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == '[' as i32
            && TT.buf[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == '2' as i32
            && TT.buf[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == '0' as i32
            && TT.buf[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == '1' as i32
            && TT.buf[5 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == '~' as i32
        {
            TT.nr -= 6 as ::core::ffi::c_int;
            memmove(
                &raw mut TT.buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                (&raw mut TT.buf as *mut ::core::ffi::c_char)
                    .offset(6 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_void,
                TT.nr as size_t,
            );
            paste_active = 0 as ::core::ffi::c_int;
            break;
        } else {
            let mut c: ::core::ffi::c_uchar = TT.buf[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_uchar;
            TT.nr -= 1;
            memmove(
                &raw mut TT.buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                (&raw mut TT.buf as *mut ::core::ffi::c_char)
                    .offset(1 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_void,
                TT.nr as size_t,
            );
            paste_slot_add_char(c as ::core::ffi::c_char);
        }
    }
    paste_slot_set_active(1 as ::core::ffi::c_int);
    paste_slot_display();
    return 1 as ::core::ffi::c_int;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
