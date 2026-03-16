extern "C" {
    pub type ldat;
    fn beep() -> ::core::ffi::c_int;
    fn endwin() -> ::core::ffi::c_int;
    fn has_colors() -> bool;
    fn initscr() -> *mut WINDOW;
    fn init_pair(
        _: ::core::ffi::c_short,
        _: ::core::ffi::c_short,
        _: ::core::ffi::c_short,
    ) -> ::core::ffi::c_int;
    fn keypad(_: *mut WINDOW, _: bool) -> ::core::ffi::c_int;
    fn noecho() -> ::core::ffi::c_int;
    fn nonl() -> ::core::ffi::c_int;
    fn raw() -> ::core::ffi::c_int;
    fn start_color() -> ::core::ffi::c_int;
    fn wattr_set(
        _: *mut WINDOW,
        _: attr_t,
        _: ::core::ffi::c_short,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn wclrtobot(_: *mut WINDOW) -> ::core::ffi::c_int;
    fn wclrtoeol(_: *mut WINDOW) -> ::core::ffi::c_int;
    fn wmove(
        _: *mut WINDOW,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn wrefresh(_: *mut WINDOW) -> ::core::ffi::c_int;
    fn use_default_colors() -> ::core::ffi::c_int;
    static mut stdscr: *mut WINDOW;
    static mut COLOR_PAIRS: ::core::ffi::c_int;
    static mut COLS: ::core::ffi::c_int;
    static mut LINES: ::core::ffi::c_int;
    fn setcchar(
        _: *mut cchar_t,
        _: *const wchar_t,
        _: attr_t,
        _: ::core::ffi::c_short,
        _: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn wadd_wch(_: *mut WINDOW, _: *const cchar_t) -> ::core::ffi::c_int;
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn ttopen();
    fn ttclose();
    fn ttgetc() -> ::core::ffi::c_int;
}
pub type wchar_t = ::libc::wchar_t;
pub type chtype = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _win_st {
    pub _cury: ::core::ffi::c_short,
    pub _curx: ::core::ffi::c_short,
    pub _maxy: ::core::ffi::c_short,
    pub _maxx: ::core::ffi::c_short,
    pub _begy: ::core::ffi::c_short,
    pub _begx: ::core::ffi::c_short,
    pub _flags: ::core::ffi::c_short,
    pub _attrs: attr_t,
    pub _bkgd: chtype,
    pub _notimeout: bool,
    pub _clear: bool,
    pub _leaveok: bool,
    pub _scroll: bool,
    pub _idlok: bool,
    pub _idcok: bool,
    pub _immed: bool,
    pub _sync: bool,
    pub _use_keypad: bool,
    pub _delay: ::core::ffi::c_int,
    pub _line: *mut ldat,
    pub _regtop: ::core::ffi::c_short,
    pub _regbottom: ::core::ffi::c_short,
    pub _parx: ::core::ffi::c_int,
    pub _pary: ::core::ffi::c_int,
    pub _parent: *mut WINDOW,
    pub _pad: pdat,
    pub _yoffset: ::core::ffi::c_short,
    pub _bkgrnd: cchar_t,
    pub _color: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cchar_t {
    pub attr: attr_t,
    pub chars: [wchar_t; 5],
    pub ext_color: ::core::ffi::c_int,
}
pub type attr_t = chtype;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdat {
    pub _pad_y: ::core::ffi::c_short,
    pub _pad_x: ::core::ffi::c_short,
    pub _pad_top: ::core::ffi::c_short,
    pub _pad_left: ::core::ffi::c_short,
    pub _pad_bottom: ::core::ffi::c_short,
    pub _pad_right: ::core::ffi::c_short,
}
pub type WINDOW = _win_st;
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
pub struct color_pair_entry {
    pub fg: ::core::ffi::c_int,
    pub bg: ::core::ffi::c_int,
    pub pair: ::core::ffi::c_int,
}
pub const __LC_ALL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NCURSES_ATTR_SHIFT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const A_NORMAL: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
    .wrapping_sub(1 as ::core::ffi::c_uint);
pub const A_UNDERLINE: chtype = (1 as ::core::ffi::c_uint)
    << 9 as ::core::ffi::c_int + NCURSES_ATTR_SHIFT;
pub const A_REVERSE: chtype = (1 as ::core::ffi::c_uint)
    << 10 as ::core::ffi::c_int + NCURSES_ATTR_SHIFT;
pub const A_BOLD: chtype = (1 as ::core::ffi::c_uint)
    << 13 as ::core::ffi::c_int + NCURSES_ATTR_SHIFT;
pub const A_ITALIC: chtype = (1 as ::core::ffi::c_uint)
    << 23 as ::core::ffi::c_int + NCURSES_ATTR_SHIFT;
pub const LC_ALL: ::core::ffi::c_int = __LC_ALL;
pub const MAXCOL: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const MAXROW: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MARGIN: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SCRSIZ: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const NPAUSE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[no_mangle]
pub static mut ncurses_term: terminal = unsafe {
    terminal {
        t_mrow: 0 as ::core::ffi::c_short,
        t_nrow: 0 as ::core::ffi::c_short,
        t_mcol: 0 as ::core::ffi::c_short,
        t_ncol: 0 as ::core::ffi::c_short,
        t_margin: MARGIN as ::core::ffi::c_short,
        t_scrsiz: SCRSIZ as ::core::ffi::c_short,
        t_pause: NPAUSE,
        t_open: Some(ncurses_open as unsafe extern "C" fn() -> ()),
        t_close: Some(ncurses_close as unsafe extern "C" fn() -> ()),
        t_kopen: Some(ncurses_kopen as unsafe extern "C" fn() -> ()),
        t_kclose: Some(ncurses_kclose as unsafe extern "C" fn() -> ()),
        t_getchar: Some(ncurses_getchar as unsafe extern "C" fn() -> ::core::ffi::c_int),
        t_putchar: Some(
            ncurses_putchar
                as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
        ),
        t_flush: Some(ncurses_flush as unsafe extern "C" fn() -> ()),
        t_move: Some(
            ncurses_move
                as unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int) -> (),
        ),
        t_eeol: Some(ncurses_eeol as unsafe extern "C" fn() -> ()),
        t_eeop: Some(ncurses_eeop as unsafe extern "C" fn() -> ()),
        t_beep: Some(ncurses_beep as unsafe extern "C" fn() -> ()),
        t_rev: Some(ncurses_rev as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
        t_italic: Some(ncurses_italic as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
        t_set_colors: Some(
            ncurses_set_colors
                as unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int) -> (),
        ),
        t_set_attrs: Some(
            ncurses_set_attrs
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        ),
        t_rez: Some(
            ncurses_cres
                as unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ::core::ffi::c_int,
        ),
    }
};
static mut current_pair: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut current_attr: attr_t = 0;
unsafe extern "C" fn map_color(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if c == -(1 as ::core::ffi::c_int) {
        return -(1 as ::core::ffi::c_int);
    }
    if c < 8 as ::core::ffi::c_int {
        return c;
    }
    if c < 16 as ::core::ffi::c_int {
        return c;
    }
    return c;
}
pub const MAX_PAIRS: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
static mut next_pair: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub static mut pair_table: [color_pair_entry; 1024] = [color_pair_entry {
    fg: 0,
    bg: 0,
    pair: 0,
}; 1024];
unsafe extern "C" fn get_pair(
    mut fg: ::core::ffi::c_int,
    mut bg: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if fg == -(1 as ::core::ffi::c_int) && bg == -(1 as ::core::ffi::c_int) {
        return 0 as ::core::ffi::c_int;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < next_pair - 1 as ::core::ffi::c_int {
        if pair_table[i as usize].fg == fg && pair_table[i as usize].bg == bg {
            return pair_table[i as usize].pair;
        }
        i += 1;
    }
    if next_pair < COLOR_PAIRS && next_pair < MAX_PAIRS {
        init_pair(
            next_pair as ::core::ffi::c_short,
            map_color(fg) as ::core::ffi::c_short,
            map_color(bg) as ::core::ffi::c_short,
        );
        pair_table[(next_pair - 1 as ::core::ffi::c_int) as usize].fg = fg;
        pair_table[(next_pair - 1 as ::core::ffi::c_int) as usize].bg = bg;
        pair_table[(next_pair - 1 as ::core::ffi::c_int) as usize].pair = next_pair;
        let fresh0 = next_pair;
        next_pair = next_pair + 1;
        return fresh0;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ncurses_open() {
    ttopen();
    setlocale(LC_ALL, b"\0" as *const u8 as *const ::core::ffi::c_char);
    initscr();
    raw();
    noecho();
    nonl();
    keypad(stdscr, TRUE != 0);
    if has_colors() {
        start_color();
        use_default_colors();
    }
    ncurses_term.t_nrow = (LINES - 1 as ::core::ffi::c_int) as ::core::ffi::c_short;
    ncurses_term.t_ncol = COLS as ::core::ffi::c_short;
    ncurses_term.t_mrow = MAXROW as ::core::ffi::c_short;
    ncurses_term.t_mcol = MAXCOL as ::core::ffi::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn ncurses_close() {
    endwin();
    ttclose();
}
#[no_mangle]
pub unsafe extern "C" fn ncurses_kopen() {}
#[no_mangle]
pub unsafe extern "C" fn ncurses_kclose() {}
#[no_mangle]
pub unsafe extern "C" fn ncurses_getchar() -> ::core::ffi::c_int {
    return ttgetc();
}
#[no_mangle]
pub unsafe extern "C" fn ncurses_putchar(
    mut c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut wc: cchar_t = cchar_t {
        attr: 0,
        chars: [0; 5],
        ext_color: 0,
    };
    let mut wstr: [wchar_t; 2] = [0; 2];
    wstr[0 as ::core::ffi::c_int as usize] = c as wchar_t;
    wstr[1 as ::core::ffi::c_int as usize] = '\0' as i32 as wchar_t;
    if setcchar(
        &raw mut wc,
        &raw mut wstr as *mut wchar_t,
        current_attr,
        current_pair as ::core::ffi::c_short,
        ::core::ptr::null::<::core::ffi::c_void>(),
    ) == OK
    {
        wadd_wch(stdscr, &raw mut wc);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ncurses_flush() {
    wrefresh(stdscr);
}
#[no_mangle]
pub unsafe extern "C" fn ncurses_move(
    mut row: ::core::ffi::c_int,
    mut col: ::core::ffi::c_int,
) {
    wmove(stdscr, row, col);
}
#[no_mangle]
pub unsafe extern "C" fn ncurses_eeol() {
    wclrtoeol(stdscr);
}
#[no_mangle]
pub unsafe extern "C" fn ncurses_eeop() {
    wclrtobot(stdscr);
}
#[no_mangle]
pub unsafe extern "C" fn ncurses_beep() {
    beep();
}
#[no_mangle]
pub unsafe extern "C" fn ncurses_rev(mut state: ::core::ffi::c_int) {
    if state != 0 {
        current_attr |= A_REVERSE;
    } else {
        current_attr &= !A_REVERSE;
    }
    wattr_set(
        stdscr,
        current_attr,
        current_pair as ::core::ffi::c_short,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ncurses_italic(mut state: ::core::ffi::c_int) {
    if state != 0 {
        current_attr |= A_ITALIC;
    } else {
        current_attr &= !A_ITALIC;
    }
    wattr_set(
        stdscr,
        current_attr,
        current_pair as ::core::ffi::c_short,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ncurses_set_colors(
    mut fg: ::core::ffi::c_int,
    mut bg: ::core::ffi::c_int,
) {
    if !has_colors() {
        return;
    }
    if fg & 0x1000000 as ::core::ffi::c_int != 0 {
        fg = -(1 as ::core::ffi::c_int);
    }
    if bg & 0x1000000 as ::core::ffi::c_int != 0 {
        bg = -(1 as ::core::ffi::c_int);
    }
    current_pair = get_pair(fg, bg);
    wattr_set(
        stdscr,
        current_attr,
        current_pair as ::core::ffi::c_short,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ncurses_set_attrs(
    mut bold: ::core::ffi::c_int,
    mut underline: ::core::ffi::c_int,
    mut italic: ::core::ffi::c_int,
) {
    if bold != 0 {
        current_attr |= A_BOLD;
    } else {
        current_attr &= !A_BOLD;
    }
    if underline != 0 {
        current_attr |= A_UNDERLINE;
    } else {
        current_attr &= !A_UNDERLINE;
    }
    if italic != 0 {
        current_attr |= A_ITALIC;
    } else {
        current_attr &= !A_ITALIC;
    }
    wattr_set(
        stdscr,
        current_attr,
        current_pair as ::core::ffi::c_short,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ncurses_cres(
    mut res: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return TRUE;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
unsafe extern "C" fn run_static_initializers() {
    current_attr = A_NORMAL;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
