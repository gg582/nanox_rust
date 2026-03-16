extern "C" {
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn puts(__s: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn tgetstr(
        _: *const ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn tgoto(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn tgetent(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn tgetnum(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn tputs(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int>,
    ) -> ::core::ffi::c_int;
    static mut eolexist: ::core::ffi::c_int;
    static mut revexist: ::core::ffi::c_int;
    static mut ttrow: ::core::ffi::c_int;
    static mut ttcol: ::core::ffi::c_int;
    static mut sgarbf: ::core::ffi::c_int;
    static mut sres: [::core::ffi::c_char; 0];
    fn getscreensize(widthp: *mut ::core::ffi::c_int, heightp: *mut ::core::ffi::c_int);
    fn ttopen();
    fn ttclose();
    fn ttputc(c: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn ttflush();
    fn ttgetc() -> ::core::ffi::c_int;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
}
pub type size_t = usize;
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
pub const MAXCOL: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const MAXROW: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MARGIN: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SCRSIZ: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const NPAUSE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const BEL: ::core::ffi::c_int = 0x7 as ::core::ffi::c_int;
pub const ESC: ::core::ffi::c_int = 0x1b as ::core::ffi::c_int;
pub const TCAPSLEN: ::core::ffi::c_int = 315 as ::core::ffi::c_int;
static mut tcapbuf: [::core::ffi::c_char; 315] = [0; 315];
static mut ZR: *mut ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>()
    as *mut ::core::ffi::c_char;
static mut SO: *mut ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>()
    as *mut ::core::ffi::c_char;
static mut UP: *mut ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>()
    as *mut ::core::ffi::c_char;
static mut CM: *mut ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>()
    as *mut ::core::ffi::c_char;
static mut CL: *mut ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>()
    as *mut ::core::ffi::c_char;
static mut SE: *mut ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>()
    as *mut ::core::ffi::c_char;
static mut CE: *mut ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>()
    as *mut ::core::ffi::c_char;
static mut ZH: *mut ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>()
    as *mut ::core::ffi::c_char;
static mut PC: ::core::ffi::c_char = 0;
static mut TI: *mut ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>()
    as *mut ::core::ffi::c_char;
static mut TE: *mut ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>()
    as *mut ::core::ffi::c_char;
#[no_mangle]
pub static mut tcap_term: terminal = {
    terminal {
        t_mrow: 0 as ::core::ffi::c_short,
        t_nrow: 0 as ::core::ffi::c_short,
        t_mcol: 0 as ::core::ffi::c_short,
        t_ncol: 0 as ::core::ffi::c_short,
        t_margin: MARGIN as ::core::ffi::c_short,
        t_scrsiz: SCRSIZ as ::core::ffi::c_short,
        t_pause: NPAUSE,
        t_open: Some(tcapopen as unsafe extern "C" fn() -> ()),
        t_close: Some(tcapclose as unsafe extern "C" fn() -> ()),
        t_kopen: Some(tcapkopen as unsafe extern "C" fn() -> ()),
        t_kclose: Some(tcapkclose as unsafe extern "C" fn() -> ()),
        t_getchar: Some(ttgetc as unsafe extern "C" fn() -> ::core::ffi::c_int),
        t_putchar: Some(
            ttputc as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
        ),
        t_flush: Some(ttflush as unsafe extern "C" fn() -> ()),
        t_move: Some(
            tcapmove
                as unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int) -> (),
        ),
        t_eeol: Some(tcapeeol as unsafe extern "C" fn() -> ()),
        t_eeop: Some(tcapeeop as unsafe extern "C" fn() -> ()),
        t_beep: Some(tcapbeep as unsafe extern "C" fn() -> ()),
        t_rev: Some(tcaprev as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
        t_italic: Some(tcapitalic as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
        t_set_colors: Some(
            tcap_set_colors
                as unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int) -> (),
        ),
        t_set_attrs: Some(
            tcap_set_attrs
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        ),
        t_rez: Some(
            tcapcres
                as unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ::core::ffi::c_int,
        ),
    }
};
#[no_mangle]
pub unsafe extern "C" fn tcapopen() {
    let mut t: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut tcbuf: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut tv_stype: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut err_str: [::core::ffi::c_char; 256] = [0; 256];
    let mut int_col: ::core::ffi::c_int = 0;
    let mut int_row: ::core::ffi::c_int = 0;
    tv_stype = getenv(b"TERM\0" as *const u8 as *const ::core::ffi::c_char);
    if tv_stype.is_null() {
        puts(
            b"Environment variable TERM not defined!\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    if tgetent(&raw mut tcbuf as *mut ::core::ffi::c_char, tv_stype)
        != 1 as ::core::ffi::c_int
    {
        snprintf(
            &raw mut err_str as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
            b"Unknown terminal type %s!\0" as *const u8 as *const ::core::ffi::c_char,
            tv_stype,
        );
        puts(&raw mut err_str as *mut ::core::ffi::c_char);
        exit(1 as ::core::ffi::c_int);
    }
    getscreensize(&raw mut int_col, &raw mut int_row);
    tcap_term.t_nrow = (int_row - 1 as ::core::ffi::c_int) as ::core::ffi::c_short;
    tcap_term.t_ncol = int_col as ::core::ffi::c_short;
    if tcap_term.t_nrow as ::core::ffi::c_int <= 0 as ::core::ffi::c_int
        && {
            tcap_term.t_nrow = (tgetnum(
                b"li\0" as *const u8 as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_short as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_short;
            tcap_term.t_nrow as ::core::ffi::c_int == -(1 as ::core::ffi::c_int)
        }
    {
        puts(
            b"termcap entry incomplete (lines)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    if tcap_term.t_ncol as ::core::ffi::c_int <= 0 as ::core::ffi::c_int
        && {
            tcap_term.t_ncol = tgetnum(
                b"co\0" as *const u8 as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_short;
            tcap_term.t_ncol as ::core::ffi::c_int == -(1 as ::core::ffi::c_int)
        }
    {
        puts(
            b"Termcap entry incomplete (columns)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    tcap_term.t_mrow = MAXROW as ::core::ffi::c_short;
    tcap_term.t_mcol = MAXCOL as ::core::ffi::c_short;
    p = &raw mut tcapbuf as *mut ::core::ffi::c_char;
    t = tgetstr(b"pc\0" as *const u8 as *const ::core::ffi::c_char, &raw mut p);
    if !t.is_null() {
        PC = *t;
    } else {
        PC = 0 as ::core::ffi::c_char;
    }
    CL = tgetstr(b"cl\0" as *const u8 as *const ::core::ffi::c_char, &raw mut p);
    CM = tgetstr(b"cm\0" as *const u8 as *const ::core::ffi::c_char, &raw mut p);
    CE = tgetstr(b"ce\0" as *const u8 as *const ::core::ffi::c_char, &raw mut p);
    UP = tgetstr(b"up\0" as *const u8 as *const ::core::ffi::c_char, &raw mut p);
    SE = tgetstr(b"se\0" as *const u8 as *const ::core::ffi::c_char, &raw mut p);
    SO = tgetstr(b"so\0" as *const u8 as *const ::core::ffi::c_char, &raw mut p);
    ZH = tgetstr(b"ZH\0" as *const u8 as *const ::core::ffi::c_char, &raw mut p);
    ZR = tgetstr(b"ZR\0" as *const u8 as *const ::core::ffi::c_char, &raw mut p);
    if !SO.is_null() {
        revexist = TRUE;
    }
    if tgetnum(b"sg\0" as *const u8 as *const ::core::ffi::c_char)
        > 0 as ::core::ffi::c_int
    {
        revexist = FALSE;
        SE = ::core::ptr::null_mut::<::core::ffi::c_char>();
        SO = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    TI = tgetstr(b"ti\0" as *const u8 as *const ::core::ffi::c_char, &raw mut p);
    TE = tgetstr(b"te\0" as *const u8 as *const ::core::ffi::c_char, &raw mut p);
    if CL.is_null() || CM.is_null() || UP.is_null() {
        puts(b"Incomplete termcap entry\n\0" as *const u8 as *const ::core::ffi::c_char);
        exit(1 as ::core::ffi::c_int);
    }
    if CE.is_null() {
        eolexist = FALSE;
    }
    if p
        >= (&raw mut tcapbuf as *mut ::core::ffi::c_char).offset(TCAPSLEN as isize)
            as *mut ::core::ffi::c_char
    {
        puts(
            b"Terminal description too big!\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    ttopen();
}
#[no_mangle]
pub unsafe extern "C" fn tcapclose() {
    putpad(tgoto(CM, 0 as ::core::ffi::c_int, tcap_term.t_nrow as ::core::ffi::c_int));
    putpad(TE);
    ttflush();
    ttclose();
}
#[no_mangle]
pub unsafe extern "C" fn tcapkopen() {
    putpad(TI);
    ttflush();
    ttrow = 999 as ::core::ffi::c_int;
    ttcol = 999 as ::core::ffi::c_int;
    sgarbf = TRUE;
    strcpy(
        &raw mut sres as *mut ::core::ffi::c_char,
        b"NORMAL\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tcapkclose() {
    putpad(TE);
    ttflush();
}
#[no_mangle]
pub unsafe extern "C" fn tcapmove(
    mut row: ::core::ffi::c_int,
    mut col: ::core::ffi::c_int,
) {
    putpad(tgoto(CM, col, row));
}
#[no_mangle]
pub unsafe extern "C" fn tcapeeol() {
    putpad(CE);
}
#[no_mangle]
pub unsafe extern "C" fn tcapeeop() {
    putpad(CL);
}
#[no_mangle]
pub unsafe extern "C" fn tcaprev(mut state: ::core::ffi::c_int) {
    if state != 0 {
        if !SO.is_null() {
            putpad(SO);
        }
    } else if !SE.is_null() {
        putpad(SE);
    }
}
unsafe extern "C" fn tcapitalic(mut state: ::core::ffi::c_int) {
    if state != 0 {
        if !ZH.is_null() {
            putpad(ZH);
        } else {
            putpad(
                b"\x1B[3m\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        }
    } else if !ZR.is_null() {
        putpad(ZR);
    } else {
        putpad(
            b"\x1B[23m\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn tcapcres(
    mut res: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return TRUE;
}
unsafe extern "C" fn tcap_set_colors(
    mut fg: ::core::ffi::c_int,
    mut bg: ::core::ffi::c_int,
) {
    let mut buf: [::core::ffi::c_char; 64] = [0; 64];
    if fg == -(1 as ::core::ffi::c_int) {
        ttputc(ESC);
        ttputc('[' as i32);
        ttputc('3' as i32);
        ttputc('9' as i32);
        ttputc('m' as i32);
    } else if fg & 0x1000000 as ::core::ffi::c_int != 0 {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as size_t,
            b"\x1B[38;2;%d;%d;%dm\0" as *const u8 as *const ::core::ffi::c_char,
            fg >> 16 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int,
            fg >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int,
            fg & 0xff as ::core::ffi::c_int,
        );
        putpad(&raw mut buf as *mut ::core::ffi::c_char);
    } else if fg >= 8 as ::core::ffi::c_int && fg < 16 as ::core::ffi::c_int {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as size_t,
            b"\x1B[%dm\0" as *const u8 as *const ::core::ffi::c_char,
            90 as ::core::ffi::c_int + (fg - 8 as ::core::ffi::c_int),
        );
        putpad(&raw mut buf as *mut ::core::ffi::c_char);
    } else {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as size_t,
            b"\x1B[%dm\0" as *const u8 as *const ::core::ffi::c_char,
            30 as ::core::ffi::c_int + fg,
        );
        putpad(&raw mut buf as *mut ::core::ffi::c_char);
    }
    if bg == -(1 as ::core::ffi::c_int) {
        ttputc(ESC);
        ttputc('[' as i32);
        ttputc('4' as i32);
        ttputc('9' as i32);
        ttputc('m' as i32);
    } else if bg & 0x1000000 as ::core::ffi::c_int != 0 {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as size_t,
            b"\x1B[48;2;%d;%d;%dm\0" as *const u8 as *const ::core::ffi::c_char,
            bg >> 16 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int,
            bg >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int,
            bg & 0xff as ::core::ffi::c_int,
        );
        putpad(&raw mut buf as *mut ::core::ffi::c_char);
    } else if bg >= 8 as ::core::ffi::c_int && bg < 16 as ::core::ffi::c_int {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as size_t,
            b"\x1B[%dm\0" as *const u8 as *const ::core::ffi::c_char,
            100 as ::core::ffi::c_int + (bg - 8 as ::core::ffi::c_int),
        );
        putpad(&raw mut buf as *mut ::core::ffi::c_char);
    } else {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as size_t,
            b"\x1B[%dm\0" as *const u8 as *const ::core::ffi::c_char,
            40 as ::core::ffi::c_int + bg,
        );
        putpad(&raw mut buf as *mut ::core::ffi::c_char);
    };
}
unsafe extern "C" fn tcap_set_attrs(
    mut bold: ::core::ffi::c_int,
    mut underline: ::core::ffi::c_int,
    mut italic: ::core::ffi::c_int,
) {
    if bold != 0 {
        ttputc(ESC);
        ttputc('[' as i32);
        ttputc('1' as i32);
        ttputc('m' as i32);
    } else {
        ttputc(ESC);
        ttputc('[' as i32);
        ttputc('2' as i32);
        ttputc('2' as i32);
        ttputc('m' as i32);
    }
    if underline != 0 {
        ttputc(ESC);
        ttputc('[' as i32);
        ttputc('4' as i32);
        ttputc('m' as i32);
    } else {
        ttputc(ESC);
        ttputc('[' as i32);
        ttputc('2' as i32);
        ttputc('4' as i32);
        ttputc('m' as i32);
    }
    tcapitalic(italic);
}
#[no_mangle]
pub unsafe extern "C" fn tcapbeep() {
    ttputc(BEL);
}
unsafe extern "C" fn putpad(mut str: *mut ::core::ffi::c_char) {
    tputs(
        str,
        1 as ::core::ffi::c_int,
        Some(ttputc as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int),
    );
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
