extern "C" {
    static mut thisflag: ::core::ffi::c_int;
    static mut lastflag: ::core::ffi::c_int;
    static mut curwp: *mut window;
    static mut curbp: *mut buffer;
    static mut cutln_active: ::core::ffi::c_int;
    fn setmark(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn rdonly() -> ::core::ffi::c_int;
    fn mlwrite(fmt: *const ::core::ffi::c_char, ...);
    fn killregion(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn copyregion(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn paste_slot_set_active(active: ::core::ffi::c_int);
    fn paste_slot_display();
    fn ldelete(n: ::core::ffi::c_long, kflag: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn kdelete();
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
pub const CFKILL: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const WFHARD: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const MDVIEW: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
unsafe extern "C" fn cutln_prime_kill_buffer() {
    if lastflag & CFKILL == 0 as ::core::ffi::c_int {
        kdelete();
    }
    thisflag |= CFKILL;
}
#[no_mangle]
pub unsafe extern "C" fn cutln_cut_current_line(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut lines_to_cut: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if (*curbp).b_mode & MDVIEW != 0 {
        return rdonly();
    }
    if f != FALSE {
        if n <= 0 as ::core::ffi::c_int {
            return FALSE;
        }
        lines_to_cut = n;
    }
    if (*curwp).w_dotp == (*curbp).b_linep {
        return FALSE;
    }
    cutln_prime_kill_buffer();
    loop {
        let fresh0 = lines_to_cut;
        lines_to_cut = lines_to_cut - 1;
        if !(fresh0 != 0) {
            break;
        }
        let mut lp: *mut line = (*curwp).w_dotp;
        let mut chunk: ::core::ffi::c_long = 0;
        let mut len: ::core::ffi::c_int = 0;
        let mut has_next_line: ::core::ffi::c_int = 0;
        if lp == (*curbp).b_linep {
            break;
        }
        (*curwp).w_dotp = lp;
        (*curwp).w_doto = 0 as ::core::ffi::c_int;
        len = (*lp).l_used;
        has_next_line = ((*lp).l_fp != (*curbp).b_linep) as ::core::ffi::c_int;
        chunk = len as ::core::ffi::c_long
            + (if has_next_line != 0 {
                1 as ::core::ffi::c_long
            } else {
                0 as ::core::ffi::c_long
            });
        if chunk == 0 as ::core::ffi::c_long {
            chunk = 1 as ::core::ffi::c_long;
        }
        if ldelete(chunk, TRUE) != TRUE {
            return FALSE;
        }
    }
    (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | WFHARD)
        as ::core::ffi::c_char;
    mlwrite(b"Line cut.\0" as *const u8 as *const ::core::ffi::c_char);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn cutln_end_cut(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if cutln_active != 0 {
        let mut s: ::core::ffi::c_int = killregion(f, n);
        cutln_active = FALSE;
        mlwrite(b"Region cut.\0" as *const u8 as *const ::core::ffi::c_char);
        return s;
    } else {
        mlwrite(
            b"No selection active. Press F7 to start cut.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return FALSE;
    };
}
#[no_mangle]
pub unsafe extern "C" fn cutln_start_cut(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut s: ::core::ffi::c_int = setmark(f, n);
    if s == TRUE {
        cutln_active = TRUE;
        mlwrite(
            b"Cut selection started. Press Shift+F7 to cut.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn cutln_end_copy(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if cutln_active != 0 {
        let mut s: ::core::ffi::c_int = copyregion(f, n);
        cutln_active = FALSE;
        mlwrite(b"Region copied.\0" as *const u8 as *const ::core::ffi::c_char);
        return s;
    } else {
        mlwrite(
            b"No selection active. Press F6 to start copy.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return FALSE;
    };
}
#[no_mangle]
pub unsafe extern "C" fn cutln_start_copy(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut s: ::core::ffi::c_int = setmark(f, n);
    if s == TRUE {
        cutln_active = TRUE;
        mlwrite(
            b"Copy selection started. Press Shift+F6 to copy.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn cutln_paste_menu(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    paste_slot_set_active(1 as ::core::ffi::c_int);
    paste_slot_display();
    return TRUE;
}
