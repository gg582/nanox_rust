extern "C" {
    static mut term: *mut terminal;
    static mut curwp: *mut window;
    static mut sgarbf: ::core::ffi::c_int;
    fn mlwrite(fmt: *const ::core::ffi::c_char, ...);
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
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WFFORCE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const WFMOVE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const WFHARD: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const WFMODE: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn reposition(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if f == FALSE {
        n = 0 as ::core::ffi::c_int;
    }
    (*curwp).w_force = n as ::core::ffi::c_char;
    (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | WFFORCE)
        as ::core::ffi::c_char;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn redraw(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if f == FALSE {
        sgarbf = TRUE;
    } else {
        (*curwp).w_force = 0 as ::core::ffi::c_char;
        (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | WFFORCE)
            as ::core::ffi::c_char;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn newsize(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut wp: *mut window = ::core::ptr::null_mut::<window>();
    if f == FALSE {
        n = (*term).t_mrow as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
    }
    if n < 3 as ::core::ffi::c_int
        || n > (*term).t_mrow as ::core::ffi::c_int + 1 as ::core::ffi::c_int
    {
        mlwrite(
            b"%%Screen size out of range\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return FALSE;
    }
    if (*term).t_nrow as ::core::ffi::c_int == n - 1 as ::core::ffi::c_int {
        return TRUE;
    }
    wp = curwp;
    (*wp).w_flag = ((*wp).w_flag as ::core::ffi::c_int | (WFHARD | WFMODE))
        as ::core::ffi::c_char;
    (*term).t_nrow = (n - 1 as ::core::ffi::c_int) as ::core::ffi::c_short;
    sgarbf = TRUE;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn newwidth(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if f == FALSE {
        n = (*term).t_mcol as ::core::ffi::c_int;
    }
    if n < 10 as ::core::ffi::c_int || n > (*term).t_mcol as ::core::ffi::c_int {
        mlwrite(
            b"%%Screen width out of range\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return FALSE;
    }
    (*term).t_ncol = n as ::core::ffi::c_short;
    (*term).t_margin = (n / 10 as ::core::ffi::c_int) as ::core::ffi::c_short;
    (*term).t_scrsiz = (n
        - (*term).t_margin as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
        as ::core::ffi::c_short;
    (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int
        | (WFHARD | WFMOVE | WFMODE)) as ::core::ffi::c_char;
    sgarbf = TRUE;
    return TRUE;
}
