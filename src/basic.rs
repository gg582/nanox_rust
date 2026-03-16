extern "C" {
    fn atoi(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    static mut tab_width: ::core::ffi::c_int;
    static mut thisflag: ::core::ffi::c_int;
    static mut lastflag: ::core::ffi::c_int;
    static mut curgoal: ::core::ffi::c_int;
    static mut curwp: *mut window;
    static mut curbp: *mut buffer;
    static mut justflag: ::core::ffi::c_int;
    fn nanox_text_rows() -> ::core::ffi::c_int;
    fn inword() -> ::core::ffi::c_int;
    fn getccol(bflg: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn mlwrite(fmt: *const ::core::ffi::c_char, ...);
    fn minibuf_input(
        prompt: *const ::core::ffi::c_char,
        dest: *mut ::core::ffi::c_char,
        max_len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn utf8_to_unicode(
        line: *mut ::core::ffi::c_uchar,
        index: ::core::ffi::c_uint,
        len: ::core::ffi::c_uint,
        res: *mut unicode_t,
    ) -> ::core::ffi::c_uint;
    fn unicode_width(c: unicode_t) -> ::core::ffi::c_int;
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
pub type unicode_t = ::core::ffi::c_uint;
pub const NSTRING: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CFCPCN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const TAB: ::core::ffi::c_int = 0x9 as ::core::ffi::c_int;
pub const WFMOVE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const WFHARD: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
unsafe extern "C" fn getgoal(mut dlp: *mut line) -> ::core::ffi::c_int {
    let mut col: ::core::ffi::c_int = 0;
    let mut dbo: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_int = (*dlp).l_used;
    col = 0 as ::core::ffi::c_int;
    dbo = 0 as ::core::ffi::c_int;
    while dbo != len {
        let mut c: unicode_t = 0;
        let mut width: ::core::ffi::c_int = utf8_to_unicode(
            &raw mut (*dlp).l_text as *mut ::core::ffi::c_uchar,
            dbo as ::core::ffi::c_uint,
            len as ::core::ffi::c_uint,
            &raw mut c,
        ) as ::core::ffi::c_int;
        col = next_column(col, c, tab_width);
        if col > curgoal {
            break;
        }
        dbo += width;
    }
    return dbo;
}
#[no_mangle]
pub unsafe extern "C" fn gotobol(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    (*curwp).w_doto = 0 as ::core::ffi::c_int;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn backchar(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut lp: *mut line = ::core::ptr::null_mut::<line>();
    if n < 0 as ::core::ffi::c_int {
        return forwchar(f, -n);
    }
    loop {
        let fresh0 = n;
        n = n - 1;
        if !(fresh0 != 0) {
            break;
        }
        if (*curwp).w_doto == 0 as ::core::ffi::c_int {
            lp = (*(*curwp).w_dotp).l_bp;
            if lp == (*curbp).b_linep {
                return FALSE;
            }
            (*curwp).w_dotp = lp;
            (*curwp).w_doto = (*lp).l_used;
            (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | WFMOVE)
                as ::core::ffi::c_char;
        } else {
            let mut c: ::core::ffi::c_uchar = 0;
            loop {
                (*curwp).w_doto -= 1;
                c = (*(&raw mut (*(*curwp).w_dotp).l_text as *mut ::core::ffi::c_uchar)
                    .offset((*curwp).w_doto as isize) as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                if is_beginning_utf8(c) != 0 {
                    break;
                }
                if !((*curwp).w_doto != 0) {
                    break;
                }
            }
        }
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn gotoeol(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    (*curwp).w_doto = (*(*curwp).w_dotp).l_used;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn forwchar(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if n < 0 as ::core::ffi::c_int {
        return backchar(f, -n);
    }
    loop {
        let fresh1 = n;
        n = n - 1;
        if !(fresh1 != 0) {
            break;
        }
        let mut len: ::core::ffi::c_int = (*(*curwp).w_dotp).l_used;
        if (*curwp).w_doto == len {
            if (*curwp).w_dotp == (*curbp).b_linep {
                return FALSE;
            }
            (*curwp).w_dotp = (*(*curwp).w_dotp).l_fp;
            (*curwp).w_doto = 0 as ::core::ffi::c_int;
            (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | WFMOVE)
                as ::core::ffi::c_char;
        } else {
            let mut c: unicode_t = 0;
            let mut bytes: ::core::ffi::c_int = utf8_to_unicode(
                &raw mut (*(*curwp).w_dotp).l_text as *mut ::core::ffi::c_uchar,
                (*curwp).w_doto as ::core::ffi::c_uint,
                len as ::core::ffi::c_uint,
                &raw mut c,
            ) as ::core::ffi::c_int;
            (*curwp).w_doto += bytes;
            if (*curwp).w_doto > len {
                (*curwp).w_doto = len;
            }
        }
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn gotoline(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut status: ::core::ffi::c_int = 0;
    let mut arg: [::core::ffi::c_char; 1024] = [0; 1024];
    if f == FALSE {
        status = minibuf_input(
            b"Line to GOTO: \0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut arg as *mut ::core::ffi::c_char,
            NSTRING,
        );
        if status != TRUE {
            mlwrite(b"(Aborted)\0" as *const u8 as *const ::core::ffi::c_char);
            return status;
        }
        n = atoi(&raw mut arg as *mut ::core::ffi::c_char);
    }
    if n == 0 as ::core::ffi::c_int {
        return gotoeob(f, n);
    }
    if n < 0 as ::core::ffi::c_int {
        return FALSE;
    }
    gotobob(f, n);
    return forwline(f, n - 1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn gotobob(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    (*curwp).w_dotp = (*(*curbp).b_linep).l_fp;
    (*curwp).w_doto = 0 as ::core::ffi::c_int;
    (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | WFHARD)
        as ::core::ffi::c_char;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn gotoeob(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    (*curwp).w_dotp = (*curbp).b_linep;
    (*curwp).w_doto = 0 as ::core::ffi::c_int;
    (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | WFHARD)
        as ::core::ffi::c_char;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn forwline(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut dlp: *mut line = ::core::ptr::null_mut::<line>();
    if n < 0 as ::core::ffi::c_int {
        return backline(f, -n);
    }
    if (*curwp).w_dotp == (*curbp).b_linep {
        return FALSE;
    }
    if lastflag & CFCPCN == 0 as ::core::ffi::c_int {
        curgoal = getccol(FALSE);
    }
    thisflag |= CFCPCN;
    dlp = (*curwp).w_dotp;
    loop {
        let fresh2 = n;
        n = n - 1;
        if !(fresh2 != 0 && dlp != (*curbp).b_linep) {
            break;
        }
        dlp = (*dlp).l_fp;
    }
    (*curwp).w_dotp = dlp;
    (*curwp).w_doto = getgoal(dlp);
    (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | WFMOVE)
        as ::core::ffi::c_char;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn backline(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut dlp: *mut line = ::core::ptr::null_mut::<line>();
    if n < 0 as ::core::ffi::c_int {
        return forwline(f, -n);
    }
    if (*(*curwp).w_dotp).l_bp == (*curbp).b_linep {
        return FALSE;
    }
    if lastflag & CFCPCN == 0 as ::core::ffi::c_int {
        curgoal = getccol(FALSE);
    }
    thisflag |= CFCPCN;
    dlp = (*curwp).w_dotp;
    loop {
        let fresh3 = n;
        n = n - 1;
        if !(fresh3 != 0 && (*dlp).l_bp != (*curbp).b_linep) {
            break;
        }
        dlp = (*dlp).l_bp;
    }
    (*curwp).w_dotp = dlp;
    (*curwp).w_doto = getgoal(dlp);
    (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | WFMOVE)
        as ::core::ffi::c_char;
    return TRUE;
}
unsafe extern "C" fn is_new_para() -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_int = 0;
    len = (*(*curwp).w_dotp).l_used;
    i = 0 as ::core::ffi::c_int;
    while i < len {
        let mut c: ::core::ffi::c_int = *(&raw mut (*(*curwp).w_dotp).l_text
            as *mut ::core::ffi::c_uchar)
            .offset(i as isize) as ::core::ffi::c_int & 0xff as ::core::ffi::c_int;
        if c == ' ' as i32 || c == TAB {
            if justflag != 0 {
                i += 1;
            } else {
                return 1 as ::core::ffi::c_int
            }
        } else {
            if !('a' as i32 <= 0xff as ::core::ffi::c_int & c
                && 'z' as i32 >= 0xff as ::core::ffi::c_int & c
                || 'A' as i32 <= 0xff as ::core::ffi::c_int & c
                    && 'Z' as i32 >= 0xff as ::core::ffi::c_int & c)
            {
                return 1 as ::core::ffi::c_int;
            }
            return 0 as ::core::ffi::c_int;
        }
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gotobop(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut suc: ::core::ffi::c_int = 0;
    if n < 0 as ::core::ffi::c_int {
        return gotoeop(f, -n);
    }
    loop {
        let fresh4 = n;
        n = n - 1;
        if !(fresh4 > 0 as ::core::ffi::c_int) {
            break;
        }
        suc = backchar(FALSE, 1 as ::core::ffi::c_int);
        while inword() == 0 && suc != 0 {
            suc = backchar(FALSE, 1 as ::core::ffi::c_int);
        }
        (*curwp).w_doto = 0 as ::core::ffi::c_int;
        while (*(*curwp).w_dotp).l_bp != (*curbp).b_linep {
            if is_new_para() != 0 {
                break;
            }
            (*curwp).w_dotp = (*(*curwp).w_dotp).l_bp;
        }
        suc = forwchar(FALSE, 1 as ::core::ffi::c_int);
        while suc != 0 && inword() == 0 {
            suc = forwchar(FALSE, 1 as ::core::ffi::c_int);
        }
    }
    (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | WFMOVE)
        as ::core::ffi::c_char;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn gotoeop(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut suc: ::core::ffi::c_int = 0;
    if n < 0 as ::core::ffi::c_int {
        return gotobop(f, -n);
    }
    loop {
        let fresh5 = n;
        n = n - 1;
        if !(fresh5 > 0 as ::core::ffi::c_int) {
            break;
        }
        suc = forwchar(FALSE, 1 as ::core::ffi::c_int);
        while inword() == 0 && suc != 0 {
            suc = forwchar(FALSE, 1 as ::core::ffi::c_int);
        }
        (*curwp).w_doto = 0 as ::core::ffi::c_int;
        if suc != 0 {
            (*curwp).w_dotp = (*(*curwp).w_dotp).l_fp;
        }
        while (*curwp).w_dotp != (*curbp).b_linep {
            if is_new_para() != 0 {
                break;
            }
            (*curwp).w_dotp = (*(*curwp).w_dotp).l_fp;
        }
        suc = backchar(FALSE, 1 as ::core::ffi::c_int);
        while suc != 0 && inword() == 0 {
            suc = backchar(FALSE, 1 as ::core::ffi::c_int);
        }
        (*curwp).w_doto = (*(*curwp).w_dotp).l_used;
    }
    (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | WFMOVE)
        as ::core::ffi::c_char;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn forwpage(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut lp: *mut line = ::core::ptr::null_mut::<line>();
    if f == FALSE {
        n = nanox_text_rows() - 2 as ::core::ffi::c_int;
        if n <= 0 as ::core::ffi::c_int {
            n = 1 as ::core::ffi::c_int;
        }
    } else if n < 0 as ::core::ffi::c_int {
        return backpage(f, -n)
    } else {
        n *= nanox_text_rows();
    }
    lp = (*curwp).w_linep as *mut line;
    loop {
        let fresh6 = n;
        n = n - 1;
        if !(fresh6 != 0 && lp != (*curbp).b_linep) {
            break;
        }
        lp = (*lp).l_fp;
    }
    (*curwp).w_linep = lp as *mut line;
    (*curwp).w_dotp = lp;
    (*curwp).w_doto = 0 as ::core::ffi::c_int;
    (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | WFHARD)
        as ::core::ffi::c_char;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn backpage(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut lp: *mut line = ::core::ptr::null_mut::<line>();
    if f == FALSE {
        n = nanox_text_rows() - 2 as ::core::ffi::c_int;
        if n <= 0 as ::core::ffi::c_int {
            n = 1 as ::core::ffi::c_int;
        }
    } else if n < 0 as ::core::ffi::c_int {
        return forwpage(f, -n)
    } else {
        n *= nanox_text_rows();
    }
    lp = (*curwp).w_linep as *mut line;
    loop {
        let fresh7 = n;
        n = n - 1;
        if !(fresh7 != 0 && (*lp).l_bp != (*curbp).b_linep) {
            break;
        }
        lp = (*lp).l_bp;
    }
    (*curwp).w_linep = lp as *mut line;
    (*curwp).w_dotp = lp;
    (*curwp).w_doto = 0 as ::core::ffi::c_int;
    (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | WFHARD)
        as ::core::ffi::c_char;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn setmark(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    (*curwp).w_markp = (*curwp).w_dotp;
    (*curwp).w_marko = (*curwp).w_doto;
    mlwrite(b"(Mark set)\0" as *const u8 as *const ::core::ffi::c_char);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn swapmark(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut odotp: *mut line = ::core::ptr::null_mut::<line>();
    let mut odoto: ::core::ffi::c_int = 0;
    if (*curwp).w_markp.is_null() {
        mlwrite(b"No mark in this window\0" as *const u8 as *const ::core::ffi::c_char);
        return FALSE;
    }
    odotp = (*curwp).w_dotp;
    odoto = (*curwp).w_doto;
    (*curwp).w_dotp = (*curwp).w_markp;
    (*curwp).w_doto = (*curwp).w_marko;
    (*curwp).w_markp = odotp;
    (*curwp).w_marko = odoto;
    (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | WFMOVE)
        as ::core::ffi::c_char;
    return TRUE;
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
#[inline]
unsafe extern "C" fn next_tab_stop(
    mut col: ::core::ffi::c_int,
    mut tab_width_val: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut step: ::core::ffi::c_int = tab_width_val + 1 as ::core::ffi::c_int;
    if step == 0 as ::core::ffi::c_int {
        step = 1 as ::core::ffi::c_int;
    }
    return col - col % step + step;
}
#[inline]
unsafe extern "C" fn next_column(
    mut old: ::core::ffi::c_int,
    mut c: unicode_t,
    mut tab_width_0: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if c == '\t' as i32 as unicode_t {
        return next_tab_stop(old, tab_width_0);
    }
    return old + mystrnlen_raw_w(c);
}
#[inline]
unsafe extern "C" fn is_beginning_utf8(
    mut c: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    return (c as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
        != 0x80 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
