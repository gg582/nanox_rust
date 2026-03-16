extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn getc(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn rewind(__stream: *mut FILE);
    fn system(__command: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
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
    fn write(
        __fd: ::core::ffi::c_int,
        __buf: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ssize_t;
    fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    static mut term: *mut terminal;
    fn vttgetc() -> ::core::ffi::c_int;
    fn vttputc(c: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn vttflush();
    fn vttbeep();
    static mut kbdm: [::core::ffi::c_int; 0];
    static mut kbdptr: *mut ::core::ffi::c_int;
    static mut kbdend: *mut ::core::ffi::c_int;
    static mut kbdmode: ::core::ffi::c_int;
    static mut kbdrep: ::core::ffi::c_int;
    static mut lastkey: ::core::ffi::c_int;
    static mut discmd: ::core::ffi::c_int;
    static mut disinp: ::core::ffi::c_int;
    static mut ttcol: ::core::ffi::c_int;
    static mut quotec: ::core::ffi::c_int;
    static mut clexec: ::core::ffi::c_int;
    static mut names: [name_bind; 0];
    fn update(force: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn movecursor(row: ::core::ffi::c_int, col: ::core::ffi::c_int);
    fn mlwrite(fmt: *const ::core::ffi::c_char, ...);
    fn ttpause();
    fn typahead() -> ::core::ffi::c_int;
    fn fncmatch(_: *mut ::core::ffi::c_char) -> fn_t;
    fn macarg(tok: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn nextarg(
        prompt: *mut ::core::ffi::c_char,
        buffer: *mut ::core::ffi::c_char,
        size: ::core::ffi::c_int,
        terminator: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
    fn unicode_width(c: unicode_t) -> ::core::ffi::c_int;
    fn xmkstemp(template: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
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
pub type ssize_t = __ssize_t;
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
pub struct name_bind {
    pub n_name: *mut ::core::ffi::c_char,
    pub n_func: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
}
pub type fn_t = Option<
    unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int) -> ::core::ffi::c_int,
>;
pub type unicode_t = ::core::ffi::c_uint;
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const NSTRING: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const NKBDM: ::core::ffi::c_int = 2048 as ::core::ffi::c_int;
pub const CONTROL: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;
pub const META: ::core::ffi::c_int = 0x20000000 as ::core::ffi::c_int;
pub const SHIFT: ::core::ffi::c_int = 0x8000000 as ::core::ffi::c_int;
pub const SPEC: ::core::ffi::c_uint = 0x80000000 as ::core::ffi::c_uint;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const STOP: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PLAY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const RECORD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
unsafe extern "C" fn erase_prev_glyph(
    mut buf: *mut ::core::ffi::c_char,
    mut cpos: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut start: ::core::ffi::c_int = 0;
    let mut uc: unicode_t = 0 as unicode_t;
    let mut width: ::core::ffi::c_int = 0;
    if buf.is_null() || cpos.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if *cpos <= 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    start = *cpos;
    loop {
        start -= 1;
        if !(start > 0 as ::core::ffi::c_int
            && is_beginning_utf8(*buf.offset(start as isize) as ::core::ffi::c_uchar)
                == 0)
        {
            break;
        }
    }
    utf8_to_unicode(
        buf as *mut ::core::ffi::c_uchar,
        start as ::core::ffi::c_uint,
        (*cpos - start) as ::core::ffi::c_uint,
        &raw mut uc,
    );
    width = mystrnlen_raw_w(uc);
    *cpos = start;
    loop {
        let fresh8 = width;
        width = width - 1;
        if !(fresh8 > 0 as ::core::ffi::c_int) {
            break;
        }
        vttputc('\u{8}' as i32);
        vttputc(' ' as i32);
        vttputc('\u{8}' as i32);
        if ttcol > 0 as ::core::ffi::c_int {
            ttcol -= 1;
        }
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mlyesno(
    mut prompt: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_char = 0;
    let mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
    loop {
        mystrscpy(
            &raw mut buf as *mut ::core::ffi::c_char,
            prompt,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as ::core::ffi::c_int,
        );
        strcat(
            &raw mut buf as *mut ::core::ffi::c_char,
            b" (y/n)? \0" as *const u8 as *const ::core::ffi::c_char,
        );
        mlwrite(&raw mut buf as *mut ::core::ffi::c_char);
        c = tgetc() as ::core::ffi::c_char;
        if c as ::core::ffi::c_int == 'y' as i32 || c as ::core::ffi::c_int == 'Y' as i32
        {
            return TRUE;
        }
        if c as ::core::ffi::c_int == 'n' as i32 || c as ::core::ffi::c_int == 'N' as i32
        {
            return FALSE;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mlreply(
    mut prompt: *mut ::core::ffi::c_char,
    mut buf: *mut ::core::ffi::c_char,
    mut nbuf: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    movecursor((*term).t_nrow as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    vttputc('\u{1b}' as i32);
    vttputc('[' as i32);
    vttputc('K' as i32);
    write(
        1 as ::core::ffi::c_int,
        b"\x1B[K\0" as *const u8 as *const ::core::ffi::c_char
            as *const ::core::ffi::c_void,
        3 as size_t,
    );
    return nextarg(prompt, buf, nbuf, ctoec('\n' as i32));
}
#[no_mangle]
pub unsafe extern "C" fn mlreplyt(
    mut prompt: *mut ::core::ffi::c_char,
    mut buf: *mut ::core::ffi::c_char,
    mut nbuf: ::core::ffi::c_int,
    mut eolchar: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    movecursor((*term).t_nrow as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    write(
        1 as ::core::ffi::c_int,
        b"\x1B[K\0" as *const u8 as *const ::core::ffi::c_char
            as *const ::core::ffi::c_void,
        3 as size_t,
    );
    return nextarg(prompt, buf, nbuf, eolchar);
}
#[no_mangle]
pub unsafe extern "C" fn ectoc(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if c & CONTROL != 0 {
        c = c & !(CONTROL | 0x40 as ::core::ffi::c_int);
    }
    if c as ::core::ffi::c_uint & SPEC != 0 {
        c = c & 255 as ::core::ffi::c_int;
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn ctoec(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if c >= 0 as ::core::ffi::c_int && c <= 0x1f as ::core::ffi::c_int {
        c = CONTROL | c + '@' as i32;
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn getname() -> fn_t {
    let mut cpos: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    let mut sp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut ffp: *mut name_bind = ::core::ptr::null_mut::<name_bind>();
    let mut cffp: *mut name_bind = ::core::ptr::null_mut::<name_bind>();
    let mut lffp: *mut name_bind = ::core::ptr::null_mut::<name_bind>();
    let mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
    cpos = 0 as ::core::ffi::c_int;
    if clexec != 0 {
        if macarg(&raw mut buf as *mut ::core::ffi::c_char) != TRUE {
            return None;
        }
        return fncmatch(
            (&raw mut buf as *mut ::core::ffi::c_char)
                .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
        );
    }
    loop {
        c = tgetc();
        if c == 0xd as ::core::ffi::c_int {
            buf[cpos as usize] = 0 as ::core::ffi::c_char;
            return fncmatch(
                (&raw mut buf as *mut ::core::ffi::c_char)
                    .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
            );
        } else if c == 0x7f as ::core::ffi::c_int || c == 0x8 as ::core::ffi::c_int {
            if erase_prev_glyph(&raw mut buf as *mut ::core::ffi::c_char, &raw mut cpos)
                != 0
            {
                vttflush();
            }
        } else if c == 0x15 as ::core::ffi::c_int {
            while erase_prev_glyph(
                &raw mut buf as *mut ::core::ffi::c_char,
                &raw mut cpos,
            ) != 0
            {}
            vttflush();
        } else if c == ' ' as i32 || c == 0x1b as ::core::ffi::c_int
            || c == 0x9 as ::core::ffi::c_int
        {
            let mut current_block_35: u64;
            buf[cpos as usize] = 0 as ::core::ffi::c_char;
            ffp = (&raw mut names as *mut name_bind)
                .offset(0 as ::core::ffi::c_int as isize) as *mut name_bind;
            's_97: loop {
                if !(*ffp).n_func.is_some() {
                    current_block_35 = 7746103178988627676;
                    break;
                }
                if strncmp(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    (*ffp).n_name,
                    strlen(&raw mut buf as *mut ::core::ffi::c_char),
                ) == 0 as ::core::ffi::c_int
                {
                    if (*ffp.offset(1 as ::core::ffi::c_int as isize)).n_func.is_none()
                        || strncmp(
                            &raw mut buf as *mut ::core::ffi::c_char,
                            (*ffp.offset(1 as ::core::ffi::c_int as isize)).n_name,
                            strlen(&raw mut buf as *mut ::core::ffi::c_char),
                        ) != 0 as ::core::ffi::c_int
                    {
                        sp = (*ffp).n_name.offset(cpos as isize);
                        while *sp != 0 {
                            let fresh5 = sp;
                            sp = sp.offset(1);
                            vttputc(*fresh5 as ::core::ffi::c_int);
                        }
                        vttflush();
                        return (*ffp).n_func as fn_t;
                    } else {
                        lffp = ffp.offset(1 as ::core::ffi::c_int as isize);
                        while (*lffp.offset(1 as ::core::ffi::c_int as isize))
                            .n_func
                            .is_some()
                        {
                            if strncmp(
                                &raw mut buf as *mut ::core::ffi::c_char,
                                (*lffp.offset(1 as ::core::ffi::c_int as isize)).n_name,
                                strlen(&raw mut buf as *mut ::core::ffi::c_char),
                            ) != 0 as ::core::ffi::c_int
                            {
                                break;
                            }
                            lffp = lffp.offset(1);
                        }
                        loop {
                            buf[cpos as usize] = *(*ffp).n_name.offset(cpos as isize);
                            cffp = ffp.offset(1 as ::core::ffi::c_int as isize);
                            while cffp <= lffp {
                                if *(*cffp).n_name.offset(cpos as isize)
                                    as ::core::ffi::c_int
                                    != buf[cpos as usize] as ::core::ffi::c_int
                                {
                                    current_block_35 = 7226443171521532240;
                                    break 's_97;
                                }
                                cffp = cffp.offset(1);
                            }
                            let fresh6 = cpos;
                            cpos = cpos + 1;
                            vttputc(buf[fresh6 as usize] as ::core::ffi::c_int);
                        }
                    }
                } else {
                    ffp = ffp.offset(1);
                }
            }
            match current_block_35 {
                7746103178988627676 => {
                    vttbeep();
                }
                _ => {}
            }
            vttflush();
        } else {
            if cpos < NSTRING - 1 as ::core::ffi::c_int && c > ' ' as i32 {
                let fresh7 = cpos;
                cpos = cpos + 1;
                buf[fresh7 as usize] = c as ::core::ffi::c_char;
                vttputc(c);
            }
            ttcol += 1;
            vttflush();
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn tgetc() -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = 0;
    if kbdmode == PLAY {
        if kbdptr < kbdend {
            let fresh0 = kbdptr;
            kbdptr = kbdptr.offset(1);
            return *fresh0;
        }
        kbdrep -= 1;
        if kbdrep < 1 as ::core::ffi::c_int {
            kbdmode = STOP;
            update(FALSE);
        } else {
            kbdptr = (&raw mut kbdm as *mut ::core::ffi::c_int)
                .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int;
            let fresh1 = kbdptr;
            kbdptr = kbdptr.offset(1);
            return *fresh1;
        }
    }
    c = vttgetc();
    lastkey = c;
    if kbdmode == RECORD {
        let fresh2 = kbdptr;
        kbdptr = kbdptr.offset(1);
        *fresh2 = c;
        kbdend = kbdptr;
        if kbdptr
            == (&raw mut kbdm as *mut ::core::ffi::c_int)
                .offset((NKBDM - 1 as ::core::ffi::c_int) as isize)
                as *mut ::core::ffi::c_int
        {
            kbdmode = STOP;
            vttbeep();
        }
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn get1key() -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = 0;
    c = tgetc();
    if c >= 0 as ::core::ffi::c_int && c <= 0x1f as ::core::ffi::c_int {
        c = CONTROL | c + '@' as i32;
    }
    return c;
}
unsafe extern "C" fn apply_modifier_bits(
    mut modifier: ::core::ffi::c_int,
    mut cmask: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if modifier == 3 as ::core::ffi::c_int || modifier == 4 as ::core::ffi::c_int
        || modifier == 7 as ::core::ffi::c_int || modifier == 8 as ::core::ffi::c_int
    {
        cmask |= META;
    }
    if modifier == 5 as ::core::ffi::c_int || modifier == 6 as ::core::ffi::c_int
        || modifier == 7 as ::core::ffi::c_int || modifier == 8 as ::core::ffi::c_int
    {
        cmask |= CONTROL;
    }
    if modifier == 2 as ::core::ffi::c_int || modifier == 4 as ::core::ffi::c_int
        || modifier == 6 as ::core::ffi::c_int || modifier == 8 as ::core::ffi::c_int
    {
        cmask |= SHIFT;
    }
    return cmask;
}
unsafe extern "C" fn map_csi_function(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    match code {
        1 => return 'H' as i32,
        2 => return 'L' as i32,
        3 => return 127 as ::core::ffi::c_int,
        4 => return 'F' as i32,
        5 => return '5' as i32,
        6 => return '6' as i32,
        11 => return 'P' as i32,
        12 => return 'Q' as i32,
        13 => return 'R' as i32,
        14 => return 'S' as i32,
        15 => return 'U' as i32,
        17 => return 'W' as i32,
        18 => return 'X' as i32,
        19 => return 'Y' as i32,
        20 => return '`' as i32,
        21 => return 'a' as i32,
        23 => return '{' as i32,
        24 => return '}' as i32,
        _ => return 0 as ::core::ffi::c_int,
    };
}
unsafe extern "C" fn decode_csi_sequence(
    mut cmask: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut params: [::core::ffi::c_int; 3] = [
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    ];
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut value: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut have_value: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ch: ::core::ffi::c_int = 0;
    loop {
        ch = get1key();
        if ch >= '0' as i32 && ch <= '9' as i32 {
            value = value * 10 as ::core::ffi::c_int + (ch - '0' as i32);
            have_value = 1 as ::core::ffi::c_int;
        } else {
            if !(ch == ';' as i32) {
                break;
            }
            if count < 3 as ::core::ffi::c_int {
                let fresh9 = count;
                count = count + 1;
                params[fresh9 as usize] = if have_value != 0 {
                    value
                } else {
                    0 as ::core::ffi::c_int
                };
            }
            value = 0 as ::core::ffi::c_int;
            have_value = 0 as ::core::ffi::c_int;
        }
    }
    if have_value != 0 && count < 3 as ::core::ffi::c_int {
        let fresh10 = count;
        count = count + 1;
        params[fresh10 as usize] = value;
    }
    if ch == '~' as i32 {
        let mut modifier: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut spec: ::core::ffi::c_int = 0;
        if count == 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        if count > 1 as ::core::ffi::c_int {
            modifier = params[1 as ::core::ffi::c_int as usize];
        }
        spec = map_csi_function(params[0 as ::core::ffi::c_int as usize]);
        if spec == 0 {
            return 0 as ::core::ffi::c_int;
        }
        return (SPEC | spec as ::core::ffi::c_uint
            | apply_modifier_bits(modifier, cmask) as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
    }
    if ch >= 'A' as i32 && ch <= 'D' as i32 {
        let mut modifier_0: ::core::ffi::c_int = if count > 0 as ::core::ffi::c_int {
            params[(count - 1 as ::core::ffi::c_int) as usize]
        } else {
            1 as ::core::ffi::c_int
        };
        return (SPEC | ch as ::core::ffi::c_uint
            | apply_modifier_bits(modifier_0, cmask) as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
    }
    if ch >= 'E' as i32 && ch <= 'z' as i32 && ch != 'i' as i32 && ch != 'c' as i32 {
        let mut modifier_1: ::core::ffi::c_int = if count > 0 as ::core::ffi::c_int {
            params[(count - 1 as ::core::ffi::c_int) as usize]
        } else {
            1 as ::core::ffi::c_int
        };
        if ch == 'u' as i32 {
            let mut key: ::core::ffi::c_int = params[0 as ::core::ffi::c_int as usize];
            if key < 128 as ::core::ffi::c_int {
                let mut code: ::core::ffi::c_int = key;
                if code >= 'a' as i32 && code <= 'z' as i32 {
                    code -= 0x20 as ::core::ffi::c_int;
                }
                return code | apply_modifier_bits(modifier_1, cmask);
            }
        }
        return (SPEC | ch as ::core::ffi::c_uint
            | apply_modifier_bits(modifier_1, cmask) as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getcmd() -> ::core::ffi::c_int {
    let mut cmask: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut c: ::core::ffi::c_int = 0;
    c = get1key();
    if c == 128 as ::core::ffi::c_int + 27 as ::core::ffi::c_int {
        let mut code: ::core::ffi::c_int = decode_csi_sequence(cmask);
        return code;
    }
    if c == CONTROL | '[' as i32 {
        if typahead() == 0 {
            ttpause();
            if typahead() == 0 {
                return c;
            }
        }
        c = get1key();
        if c == '[' as i32 {
            let mut code_0: ::core::ffi::c_int = decode_csi_sequence(cmask);
            return code_0;
        }
        if c == 'O' as i32 {
            let mut code_1: ::core::ffi::c_int = get1key();
            return (SPEC | code_1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn getstring(
    mut prompt: *mut ::core::ffi::c_char,
    mut buf: *mut ::core::ffi::c_char,
    mut nbuf: ::core::ffi::c_int,
    mut eolchar: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut cpos: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut c_int: ::core::ffi::c_int = 0;
    let mut quotef: ::core::ffi::c_int = 0;
    let mut ffile: ::core::ffi::c_int = 0;
    let mut ocpos: ::core::ffi::c_int = 0;
    let mut nskip: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut didtry: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    static mut tmp: [::core::ffi::c_char; 14] = unsafe {
        ::core::mem::transmute::<
            [u8; 14],
            [::core::ffi::c_char; 14],
        >(*b"/tmp/meXXXXXX\0")
    };
    let mut tmpf: *mut FILE = ::core::ptr::null_mut::<FILE>();
    ffile = (strcmp(prompt, b"Find file: \0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
        || strcmp(prompt, b"View file: \0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        || strcmp(prompt, b"Insert file: \0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        || strcmp(prompt, b"Write file: \0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        || strcmp(prompt, b"Read file: \0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        || strcmp(
            prompt,
            b"File to execute: \0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    cpos = 0 as ::core::ffi::c_int;
    quotef = FALSE;
    mlwrite(prompt);
    loop {
        if didtry == 0 {
            nskip = -(1 as ::core::ffi::c_int);
        }
        didtry = 0 as ::core::ffi::c_int;
        c_int = get1key();
        if c_int as ::core::ffi::c_uint & SPEC != 0 {
            vttbeep();
        } else {
            c = c_int;
            if c == CONTROL | 0x4d as ::core::ffi::c_int && quotef == 0 {
                c = CONTROL | 0x40 as ::core::ffi::c_int | '\n' as i32;
            }
            if c == eolchar && quotef == FALSE {
                let fresh11 = cpos;
                cpos = cpos + 1;
                *buf.offset(fresh11 as isize) = 0 as ::core::ffi::c_char;
                mlwrite(b"\0" as *const u8 as *const ::core::ffi::c_char);
                vttflush();
                if *buf.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    return FALSE;
                }
                return TRUE;
            }
            c = ectoc(c);
            if (c == 0x7f as ::core::ffi::c_int || c == 0x8 as ::core::ffi::c_int)
                && quotef == FALSE
            {
                if erase_prev_glyph(buf, &raw mut cpos) != 0 {
                    vttflush();
                }
            } else if c == 0x15 as ::core::ffi::c_int && quotef == FALSE {
                while erase_prev_glyph(buf, &raw mut cpos) != 0 {}
                vttflush();
            } else if (c == 0x9 as ::core::ffi::c_int || c == ' ' as i32)
                && quotef == FALSE && ffile != 0
            {
                let mut ffbuf: [::core::ffi::c_char; 255] = [0; 255];
                let mut n: ::core::ffi::c_int = 0;
                let mut iswild: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                didtry = 1 as ::core::ffi::c_int;
                ocpos = cpos;
                while cpos != 0 as ::core::ffi::c_int {
                    outstring(
                        b"\x08 \x08\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    );
                    ttcol -= 1;
                    cpos -= 1;
                    if (*buf.offset(cpos as isize) as ::core::ffi::c_int)
                        < 0x20 as ::core::ffi::c_int
                    {
                        outstring(
                            b"\x08 \x08\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                        ttcol -= 1;
                    }
                    if *buf.offset(cpos as isize) as ::core::ffi::c_int == '\n' as i32 {
                        outstring(
                            b"\x08\x08  \x08\x08\0" as *const u8
                                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                        );
                        ttcol -= 2 as ::core::ffi::c_int;
                    }
                    if *buf.offset(cpos as isize) as ::core::ffi::c_int == '*' as i32
                        || *buf.offset(cpos as isize) as ::core::ffi::c_int == '?' as i32
                    {
                        iswild = 1 as ::core::ffi::c_int;
                    }
                }
                vttflush();
                if nskip < 0 as ::core::ffi::c_int {
                    *buf.offset(ocpos as isize) = 0 as ::core::ffi::c_char;
                    if !tmpf.is_null() {
                        fclose(tmpf);
                    }
                    strcpy(
                        &raw mut tmp as *mut ::core::ffi::c_char,
                        b"/tmp/meXXXXXX\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    strcpy(
                        &raw mut ffbuf as *mut ::core::ffi::c_char,
                        b"echo \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    strcat(&raw mut ffbuf as *mut ::core::ffi::c_char, buf);
                    if iswild == 0 {
                        strcat(
                            &raw mut ffbuf as *mut ::core::ffi::c_char,
                            b"*\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                    strcat(
                        &raw mut ffbuf as *mut ::core::ffi::c_char,
                        b" >\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    xmkstemp(&raw mut tmp as *mut ::core::ffi::c_char);
                    strcat(
                        &raw mut ffbuf as *mut ::core::ffi::c_char,
                        &raw mut tmp as *mut ::core::ffi::c_char,
                    );
                    strcat(
                        &raw mut ffbuf as *mut ::core::ffi::c_char,
                        b" 2>&1\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    system(&raw mut ffbuf as *mut ::core::ffi::c_char);
                    tmpf = fopen(
                        &raw mut tmp as *mut ::core::ffi::c_char,
                        b"r\0" as *const u8 as *const ::core::ffi::c_char,
                    ) as *mut FILE;
                    nskip = 0 as ::core::ffi::c_int;
                }
                c = ' ' as i32;
                n = nskip;
                while n > 0 as ::core::ffi::c_int {
                    loop {
                        c = getc(tmpf);
                        if !(c != EOF && c != ' ' as i32) {
                            break;
                        }
                    }
                    n -= 1;
                }
                nskip += 1;
                if c != ' ' as i32 {
                    vttbeep();
                    nskip = 0 as ::core::ffi::c_int;
                }
                loop {
                    c = getc(tmpf);
                    if !(c != EOF && c != '\n' as i32 && c != ' ' as i32
                        && c != '*' as i32)
                    {
                        break;
                    }
                    if cpos < nbuf - 1 as ::core::ffi::c_int {
                        let fresh12 = cpos;
                        cpos = cpos + 1;
                        *buf.offset(fresh12 as isize) = c as ::core::ffi::c_char;
                    }
                }
                if c == '*' as i32 {
                    vttbeep();
                }
                n = 0 as ::core::ffi::c_int;
                while n < cpos {
                    c = *buf.offset(n as isize) as ::core::ffi::c_int;
                    if c < ' ' as i32 && c != '\n' as i32 {
                        outstring(
                            b"^\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                        ttcol += 1;
                        c ^= 0x40 as ::core::ffi::c_int;
                    }
                    if c != '\n' as i32 {
                        if disinp != 0 {
                            vttputc(c);
                        }
                    } else {
                        outstring(
                            b"<NL>\0" as *const u8 as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char,
                        );
                        ttcol += 3 as ::core::ffi::c_int;
                    }
                    ttcol += 1;
                    n += 1;
                }
                vttflush();
                rewind(tmpf);
                unlink(&raw mut tmp as *mut ::core::ffi::c_char);
            } else if (c == quotec || c == 0x16 as ::core::ffi::c_int) && quotef == FALSE
            {
                quotef = TRUE;
            } else {
                quotef = FALSE;
                if cpos < nbuf - 1 as ::core::ffi::c_int {
                    if c_int >= 0x80 as ::core::ffi::c_int {
                        let mut utf8_buf: [::core::ffi::c_char; 6] = [0; 6];
                        let mut len: ::core::ffi::c_int = unicode_to_utf8(
                            c_int as ::core::ffi::c_uint,
                            &raw mut utf8_buf as *mut ::core::ffi::c_char
                                as *mut ::core::ffi::c_uchar,
                        ) as ::core::ffi::c_int;
                        if cpos + len < nbuf {
                            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while i < len {
                                let fresh13 = cpos;
                                cpos = cpos + 1;
                                *buf.offset(fresh13 as isize) = utf8_buf[i as usize];
                                i += 1;
                            }
                            if disinp != 0 {
                                vttputc(c_int);
                                ttcol += unicode_width(c_int as unicode_t);
                            }
                        } else {
                            vttbeep();
                        }
                    } else {
                        let mut c_byte: ::core::ffi::c_uchar = c_int
                            as ::core::ffi::c_uchar;
                        let fresh14 = cpos;
                        cpos = cpos + 1;
                        *buf.offset(fresh14 as isize) = c_byte as ::core::ffi::c_char;
                        if disinp != 0 {
                            if (c_byte as ::core::ffi::c_int)
                                < 0x20 as ::core::ffi::c_int
                                && c_byte as ::core::ffi::c_int != '\n' as i32
                            {
                                outstring(
                                    b"^\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                );
                                vttputc(c_byte as ::core::ffi::c_int + '@' as i32);
                                ttcol += 2 as ::core::ffi::c_int;
                            } else if c_byte as ::core::ffi::c_int == '\n' as i32 {
                                outstring(
                                    b"<NL>\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                );
                                ttcol += 3 as ::core::ffi::c_int;
                            } else {
                                vttputc(c_byte as ::core::ffi::c_int);
                                ttcol += 1;
                            }
                        }
                    }
                }
                vttflush();
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn outstring(mut s: *mut ::core::ffi::c_char) {
    if disinp != 0 {
        while *s != 0 {
            let fresh15 = s;
            s = s.offset(1);
            vttputc(*fresh15 as ::core::ffi::c_int);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ostring(mut s: *mut ::core::ffi::c_char) {
    if discmd != 0 {
        while *s != 0 {
            let fresh16 = s;
            s = s.offset(1);
            vttputc(*fresh16 as ::core::ffi::c_int);
        }
    }
}
#[inline]
unsafe extern "C" fn is_beginning_utf8(
    mut c: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    return (c as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
        != 0x80 as ::core::ffi::c_int) as ::core::ffi::c_int;
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
        let fresh3 = src;
        src = src.offset(1);
        let mut c: ::core::ffi::c_char = *fresh3;
        if c == 0 {
            break;
        }
        let fresh4 = dst;
        dst = dst.offset(1);
        *fresh4 = c;
    }
    *dst = 0 as ::core::ffi::c_char;
}
#[inline]
unsafe extern "C" fn mystrnlen_raw_w(mut c: unicode_t) -> ::core::ffi::c_int {
    if c >= 0x4e00 as unicode_t && c <= 0x9fff as unicode_t {
        return 2 as ::core::ffi::c_int;
    }
    if c >= 0xac00 as unicode_t && c <= 0xd7af as unicode_t {
        return 2 as ::core::ffi::c_int;
    }
    if c >= 0x3040 as unicode_t && c <= 0x309f as unicode_t {
        return 2 as ::core::ffi::c_int;
    }
    if c >= 0x30a0 as unicode_t && c <= 0x30ff as unicode_t {
        return 2 as ::core::ffi::c_int;
    }
    return unicode_width(c);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
