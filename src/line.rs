extern "C" {
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
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    static mut tab_width: ::core::ffi::c_int;
    static mut curwp: *mut window;
    static mut curbp: *mut buffer;
    static mut bheadp: *mut buffer;
    static mut kbufp: *mut kill;
    static mut kbufh: *mut kill;
    static mut kused: ::core::ffi::c_int;
    fn backchar(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn backline(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn killtext(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn rdonly() -> ::core::ffi::c_int;
    fn mlwrite(fmt: *const ::core::ffi::c_char, ...);
}
pub type size_t = usize;
pub type unicode_t = ::core::ffi::c_uint;
pub type StateID = ::core::ffi::c_uint;
pub const HS_TRIPLE_STRING: StateID = 3;
pub const HS_STRING: StateID = 2;
pub const HS_BLOCK_COMMENT: StateID = 1;
pub const HS_NORMAL: StateID = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HighlightStackEntry {
    pub state: StateID,
    pub sub_id: ::core::ffi::c_int,
    pub string_delim: ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HighlightState {
    pub stack: [HighlightStackEntry; 8],
    pub depth: ::core::ffi::c_int,
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
pub struct kill {
    pub d_next: *mut kill,
    pub d_chunk: [::core::ffi::c_char; 8192],
}
#[inline]
unsafe extern "C" fn is_beginning_utf8(
    mut c: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    return (c as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
        != 0x80 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
pub const NSTRING: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const KBLOCK: ::core::ffi::c_int = 8192 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WFEDIT: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const WFHARD: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const WFMODE: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const BFCHG: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MDVIEW: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const BLOCK_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn lalloc(mut used: ::core::ffi::c_int) -> *mut line {
    let mut lp: *mut line = ::core::ptr::null_mut::<line>();
    let mut size: ::core::ffi::c_int = 0;
    size = used + BLOCK_SIZE - 1 as ::core::ffi::c_int
        & !(BLOCK_SIZE - 1 as ::core::ffi::c_int);
    if size == 0 as ::core::ffi::c_int {
        size = BLOCK_SIZE;
    }
    lp = malloc(
        (::core::mem::size_of::<line>() as size_t)
            .wrapping_sub(1 as size_t)
            .wrapping_add(size as size_t),
    ) as *mut line;
    if lp.is_null() {
        mlwrite(b"(OUT OF MEMORY)\0" as *const u8 as *const ::core::ffi::c_char);
        return ::core::ptr::null_mut::<line>();
    }
    (*lp).l_size = size;
    (*lp).l_used = used;
    (*lp).hl_start_state = HighlightState {
        stack: [
            HighlightStackEntry {
                state: HS_NORMAL,
                sub_id: 0,
                string_delim: 0,
            },
            HighlightStackEntry {
                state: HS_NORMAL,
                sub_id: 0,
                string_delim: 0,
            },
            HighlightStackEntry {
                state: HS_NORMAL,
                sub_id: 0,
                string_delim: 0,
            },
            HighlightStackEntry {
                state: HS_NORMAL,
                sub_id: 0,
                string_delim: 0,
            },
            HighlightStackEntry {
                state: HS_NORMAL,
                sub_id: 0,
                string_delim: 0,
            },
            HighlightStackEntry {
                state: HS_NORMAL,
                sub_id: 0,
                string_delim: 0,
            },
            HighlightStackEntry {
                state: HS_NORMAL,
                sub_id: 0,
                string_delim: 0,
            },
            HighlightStackEntry {
                state: HS_NORMAL,
                sub_id: 0,
                string_delim: 0,
            },
        ],
        depth: 0,
    };
    (*lp).hl_end_state = HighlightState {
        stack: [
            HighlightStackEntry {
                state: HS_NORMAL,
                sub_id: 0,
                string_delim: 0,
            },
            HighlightStackEntry {
                state: HS_NORMAL,
                sub_id: 0,
                string_delim: 0,
            },
            HighlightStackEntry {
                state: HS_NORMAL,
                sub_id: 0,
                string_delim: 0,
            },
            HighlightStackEntry {
                state: HS_NORMAL,
                sub_id: 0,
                string_delim: 0,
            },
            HighlightStackEntry {
                state: HS_NORMAL,
                sub_id: 0,
                string_delim: 0,
            },
            HighlightStackEntry {
                state: HS_NORMAL,
                sub_id: 0,
                string_delim: 0,
            },
            HighlightStackEntry {
                state: HS_NORMAL,
                sub_id: 0,
                string_delim: 0,
            },
            HighlightStackEntry {
                state: HS_NORMAL,
                sub_id: 0,
                string_delim: 0,
            },
        ],
        depth: 0,
    };
    return lp;
}
#[no_mangle]
pub unsafe extern "C" fn lfree(mut lp: *mut line) {
    let mut bp: *mut buffer = ::core::ptr::null_mut::<buffer>();
    let mut wp: *mut window = ::core::ptr::null_mut::<window>();
    wp = curwp;
    if (*wp).w_linep == lp {
        (*wp).w_linep = (*lp).l_fp;
    }
    if (*wp).w_dotp == lp {
        (*wp).w_dotp = (*lp).l_fp;
        (*wp).w_doto = 0 as ::core::ffi::c_int;
    }
    if (*wp).w_markp == lp {
        (*wp).w_markp = (*lp).l_fp;
        (*wp).w_marko = 0 as ::core::ffi::c_int;
    }
    bp = bheadp;
    while !bp.is_null() {
        if (*bp).b_nwnd as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if (*bp).b_dotp == lp {
                (*bp).b_dotp = (*lp).l_fp;
                (*bp).b_doto = 0 as ::core::ffi::c_int;
            }
            if (*bp).b_markp == lp {
                (*bp).b_markp = (*lp).l_fp;
                (*bp).b_marko = 0 as ::core::ffi::c_int;
            }
        }
        bp = (*bp).b_bufp;
    }
    (*(*lp).l_bp).l_fp = (*lp).l_fp;
    (*(*lp).l_fp).l_bp = (*lp).l_bp;
    free(lp as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn lchange(mut flag: ::core::ffi::c_int) {
    let mut wp: *mut window = ::core::ptr::null_mut::<window>();
    if (*curbp).b_nwnd as ::core::ffi::c_int != 1 as ::core::ffi::c_int {
        flag = WFHARD;
    }
    if (*curbp).b_flag as ::core::ffi::c_int & BFCHG == 0 as ::core::ffi::c_int {
        flag |= WFMODE;
        (*curbp).b_flag = ((*curbp).b_flag as ::core::ffi::c_int | BFCHG)
            as ::core::ffi::c_char;
    }
    wp = curwp;
    if (*wp).w_bufp == curbp {
        (*wp).w_flag = ((*wp).w_flag as ::core::ffi::c_int | flag)
            as ::core::ffi::c_char;
    }
}
#[no_mangle]
pub unsafe extern "C" fn insspace(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    linsert(n, ' ' as i32);
    backchar(f, n);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn linstr(
    mut instr: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut status: ::core::ffi::c_int = TRUE;
    let mut tmpc: ::core::ffi::c_char = 0;
    if !instr.is_null() {
        loop {
            tmpc = *instr;
            if !(tmpc as ::core::ffi::c_int != 0 && status == TRUE) {
                break;
            }
            if tmpc as ::core::ffi::c_int == '\r' as i32 {
                status = lnewline();
                if *instr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\n' as i32
                {
                    instr = instr.offset(1);
                }
            } else if tmpc as ::core::ffi::c_int == '\n' as i32 {
                status = lnewline();
            } else {
                status = linsert(1 as ::core::ffi::c_int, tmpc as ::core::ffi::c_int);
            }
            if status != TRUE {
                mlwrite(
                    b"%%Out of memory while inserting\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                break;
            } else {
                instr = instr.offset(1);
            }
        }
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn linsert_byte(
    mut n: ::core::ffi::c_int,
    mut c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut cp1: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
        ::core::ffi::c_uchar,
    >();
    let mut cp2: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
        ::core::ffi::c_uchar,
    >();
    let mut lp1: *mut line = ::core::ptr::null_mut::<line>();
    let mut lp2: *mut line = ::core::ptr::null_mut::<line>();
    let mut lp3: *mut line = ::core::ptr::null_mut::<line>();
    let mut doto: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut wp: *mut window = ::core::ptr::null_mut::<window>();
    if (*curbp).b_mode & MDVIEW != 0 {
        return rdonly();
    }
    lchange(WFEDIT);
    lp1 = (*curwp).w_dotp;
    if lp1 == (*curbp).b_linep {
        if (*curwp).w_doto != 0 as ::core::ffi::c_int {
            mlwrite(b"bug: linsert\0" as *const u8 as *const ::core::ffi::c_char);
            return FALSE;
        }
        lp2 = lalloc(n);
        if lp2.is_null() {
            return FALSE;
        }
        lp3 = (*lp1).l_bp;
        (*lp3).l_fp = lp2;
        (*lp2).l_fp = lp1;
        (*lp1).l_bp = lp2;
        (*lp2).l_bp = lp3;
        i = 0 as ::core::ffi::c_int;
        while i < n {
            *(&raw mut (*lp2).l_text as *mut ::core::ffi::c_uchar).offset(i as isize) = c
                as ::core::ffi::c_uchar;
            i += 1;
        }
        (*curwp).w_dotp = lp2;
        (*curwp).w_doto = n;
        return TRUE;
    }
    doto = (*curwp).w_doto;
    if (*lp1).l_used + n > (*lp1).l_size {
        lp2 = lalloc((*lp1).l_used + n);
        if lp2.is_null() {
            return FALSE;
        }
        cp1 = (&raw mut (*lp1).l_text as *mut ::core::ffi::c_uchar)
            .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_uchar;
        cp2 = (&raw mut (*lp2).l_text as *mut ::core::ffi::c_uchar)
            .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_uchar;
        while cp1
            != (&raw mut (*lp1).l_text as *mut ::core::ffi::c_uchar)
                .offset(doto as isize) as *mut ::core::ffi::c_uchar
        {
            let fresh0 = cp1;
            cp1 = cp1.offset(1);
            let fresh1 = cp2;
            cp2 = cp2.offset(1);
            *fresh1 = *fresh0;
        }
        cp2 = cp2.offset(n as isize);
        while cp1
            != (&raw mut (*lp1).l_text as *mut ::core::ffi::c_uchar)
                .offset((*lp1).l_used as isize) as *mut ::core::ffi::c_uchar
        {
            let fresh2 = cp1;
            cp1 = cp1.offset(1);
            let fresh3 = cp2;
            cp2 = cp2.offset(1);
            *fresh3 = *fresh2;
        }
        (*(*lp1).l_bp).l_fp = lp2;
        (*lp2).l_fp = (*lp1).l_fp;
        (*(*lp1).l_fp).l_bp = lp2;
        (*lp2).l_bp = (*lp1).l_bp;
        free(lp1 as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
    } else {
        lp2 = lp1;
        (*lp2).l_used += n;
        cp2 = (&raw mut (*lp1).l_text as *mut ::core::ffi::c_uchar)
            .offset((*lp1).l_used as isize) as *mut ::core::ffi::c_uchar;
        cp1 = cp2.offset(-(n as isize));
        while cp1
            != (&raw mut (*lp1).l_text as *mut ::core::ffi::c_uchar)
                .offset(doto as isize) as *mut ::core::ffi::c_uchar
        {
            cp1 = cp1.offset(-1);
            cp2 = cp2.offset(-1);
            *cp2 = *cp1;
        }
    }
    i = 0 as ::core::ffi::c_int;
    while i < n {
        *(&raw mut (*lp2).l_text as *mut ::core::ffi::c_uchar)
            .offset((doto + i) as isize) = c as ::core::ffi::c_uchar;
        i += 1;
    }
    wp = curwp;
    if (*wp).w_linep == lp1 {
        (*wp).w_linep = lp2;
    }
    if (*wp).w_dotp == lp1 {
        (*wp).w_dotp = lp2;
        (*wp).w_doto += n;
    }
    if (*wp).w_markp == lp1 {
        (*wp).w_markp = lp2;
        if (*wp).w_marko > doto {
            (*wp).w_marko += n;
        }
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn linsert(
    mut n: ::core::ffi::c_int,
    mut c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut utf8: [::core::ffi::c_char; 6] = [0; 6];
    let mut bytes: ::core::ffi::c_int = unicode_to_utf8(
        c as ::core::ffi::c_uint,
        &raw mut utf8 as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar,
    ) as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    if bytes == 1 as ::core::ffi::c_int {
        return linsert_byte(
            n,
            utf8[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_uchar
                as ::core::ffi::c_int,
        );
    }
    i = 0 as ::core::ffi::c_int;
    while i < n {
        let mut j: ::core::ffi::c_int = 0;
        j = 0 as ::core::ffi::c_int;
        while j < bytes {
            let mut c_0: ::core::ffi::c_uchar = utf8[j as usize] as ::core::ffi::c_uchar;
            if linsert_byte(1 as ::core::ffi::c_int, c_0 as ::core::ffi::c_int) == 0 {
                return FALSE;
            }
            j += 1;
        }
        i += 1;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn sanitize_and_insert(
    mut n: ::core::ffi::c_int,
    mut c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if c == '\n' as i32 || c == '\t' as i32 || c == '\r' as i32 {
        if c == '\n' as i32 {
            loop {
                let fresh6 = n;
                n = n - 1;
                if !(fresh6 != 0) {
                    break;
                }
                if lnewline() == FALSE {
                    return FALSE;
                }
            }
            return TRUE;
        }
        return linsert(n, c);
    } else if c < 32 as ::core::ffi::c_int && c >= 0 as ::core::ffi::c_int
        || c == 127 as ::core::ffi::c_int
    {
        return TRUE
    } else {
        return linsert(n, c)
    };
}
#[no_mangle]
pub unsafe extern "C" fn lowrite(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if (*curwp).w_doto < (*(*curwp).w_dotp).l_used {
        let mut existing_char: unicode_t = 0;
        let mut bytes: ::core::ffi::c_int = lgetchar(&raw mut existing_char);
        if existing_char != '\t' as i32 as unicode_t
            || (*curwp).w_doto & tab_width == tab_width
        {
            ldelete(bytes as ::core::ffi::c_long, FALSE);
        }
    }
    return linsert(1 as ::core::ffi::c_int, c);
}
#[no_mangle]
pub unsafe extern "C" fn lover(
    mut ostr: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut status: ::core::ffi::c_int = TRUE;
    let mut tmpc: ::core::ffi::c_char = 0;
    if !ostr.is_null() {
        loop {
            tmpc = *ostr;
            if !(tmpc as ::core::ffi::c_int != 0 && status == TRUE) {
                break;
            }
            if tmpc as ::core::ffi::c_int == '\r' as i32 {
                status = lnewline();
                if *ostr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\n' as i32
                {
                    ostr = ostr.offset(1);
                }
            } else if tmpc as ::core::ffi::c_int == '\n' as i32 {
                status = lnewline();
            } else {
                status = lowrite(tmpc as ::core::ffi::c_int);
            }
            if status != TRUE {
                mlwrite(
                    b"%%Out of memory while overwriting\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                break;
            } else {
                ostr = ostr.offset(1);
            }
        }
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn lnewline() -> ::core::ffi::c_int {
    let mut cp1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut cp2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut lp1: *mut line = ::core::ptr::null_mut::<line>();
    let mut lp2: *mut line = ::core::ptr::null_mut::<line>();
    let mut lp3: *mut line = ::core::ptr::null_mut::<line>();
    let mut doto: ::core::ffi::c_int = 0;
    let mut wp: *mut window = ::core::ptr::null_mut::<window>();
    if (*curbp).b_mode & MDVIEW != 0 {
        return rdonly();
    }
    lp1 = (*curwp).w_dotp;
    doto = (*curwp).w_doto;
    lchange(WFHARD);
    if lp1 == (*curbp).b_linep {
        if doto != 0 as ::core::ffi::c_int {
            mlwrite(
                b"bug: lnewline at sentinel\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return FALSE;
        }
        lp2 = lalloc(0 as ::core::ffi::c_int);
        if lp2.is_null() {
            return FALSE;
        }
        lp3 = (*lp1).l_bp;
        (*lp3).l_fp = lp2;
        (*lp2).l_fp = lp1;
        (*lp1).l_bp = lp2;
        (*lp2).l_bp = lp3;
        return TRUE;
    }
    while doto > 0 as ::core::ffi::c_int && doto < (*lp1).l_used
        && is_beginning_utf8(
            *(&raw mut (*lp1).l_text as *mut ::core::ffi::c_uchar).offset(doto as isize),
        ) == 0
    {
        doto -= 1;
    }
    lp2 = lalloc((*lp1).l_used - doto);
    if lp2.is_null() {
        return FALSE;
    }
    cp1 = (&raw mut (*lp1).l_text as *mut ::core::ffi::c_uchar).offset(doto as isize)
        as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char;
    cp2 = (&raw mut (*lp2).l_text as *mut ::core::ffi::c_uchar)
        .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_uchar
        as *mut ::core::ffi::c_char;
    while cp1
        != (&raw mut (*lp1).l_text as *mut ::core::ffi::c_uchar)
            .offset((*lp1).l_used as isize) as *mut ::core::ffi::c_uchar
            as *mut ::core::ffi::c_char
    {
        let fresh4 = cp1;
        cp1 = cp1.offset(1);
        let fresh5 = cp2;
        cp2 = cp2.offset(1);
        *fresh5 = *fresh4;
    }
    (*lp2).l_fp = (*lp1).l_fp;
    (*lp1).l_fp = lp2;
    (*(*lp2).l_fp).l_bp = lp2;
    (*lp2).l_bp = lp1;
    (*lp1).l_used = doto;
    wp = curwp;
    if (*wp).w_dotp == lp1 {
        if (*wp).w_doto >= doto {
            (*wp).w_dotp = lp2;
            (*wp).w_doto -= doto;
        }
    }
    if (*wp).w_markp == lp1 {
        if (*wp).w_marko > doto {
            (*wp).w_markp = lp2;
            (*wp).w_marko -= doto;
        }
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn lgetchar(mut c: *mut unicode_t) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = (*(*curwp).w_dotp).l_used;
    let mut buf: *mut ::core::ffi::c_uchar = &raw mut (*(*curwp).w_dotp).l_text
        as *mut ::core::ffi::c_uchar;
    return utf8_to_unicode(
        buf,
        (*curwp).w_doto as ::core::ffi::c_uint,
        len as ::core::ffi::c_uint,
        c,
    ) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ldelchar(
    mut n: ::core::ffi::c_long,
    mut kflag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    loop {
        let fresh22 = n;
        n = n - 1;
        if !(fresh22 > 0 as ::core::ffi::c_long) {
            break;
        }
        let mut c: unicode_t = 0;
        let mut bytes: ::core::ffi::c_int = lgetchar(&raw mut c);
        if bytes <= 0 as ::core::ffi::c_int {
            return FALSE;
        }
        if ldelete(bytes as ::core::ffi::c_long, kflag) != TRUE {
            return FALSE;
        }
    }
    lchange(WFHARD);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn ldelete(
    mut n: ::core::ffi::c_long,
    mut kflag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut cp1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut cp2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut dotp: *mut line = ::core::ptr::null_mut::<line>();
    let mut doto: ::core::ffi::c_int = 0;
    let mut chunk: ::core::ffi::c_int = 0;
    let mut wp: *mut window = ::core::ptr::null_mut::<window>();
    if (*curbp).b_mode & MDVIEW != 0 {
        return rdonly();
    }
    while n != 0 as ::core::ffi::c_long {
        dotp = (*curwp).w_dotp;
        doto = (*curwp).w_doto;
        if dotp == (*curbp).b_linep {
            return FALSE;
        }
        chunk = (*dotp).l_used - doto;
        if chunk as ::core::ffi::c_long > n {
            chunk = n as ::core::ffi::c_int;
        }
        if chunk == 0 as ::core::ffi::c_int {
            lchange(WFHARD);
            if ldelnewline() == FALSE || kflag != FALSE && kinsert('\n' as i32) == FALSE
            {
                return FALSE;
            }
            n -= 1;
        } else {
            lchange(WFHARD);
            cp1 = (&raw mut (*dotp).l_text as *mut ::core::ffi::c_uchar)
                .offset(doto as isize) as *mut ::core::ffi::c_uchar
                as *mut ::core::ffi::c_char;
            cp2 = cp1.offset(chunk as isize);
            if kflag != FALSE {
                while cp1 != cp2 {
                    if kinsert(*cp1 as ::core::ffi::c_uchar as ::core::ffi::c_int)
                        == FALSE
                    {
                        return FALSE;
                    }
                    cp1 = cp1.offset(1);
                }
                cp1 = (&raw mut (*dotp).l_text as *mut ::core::ffi::c_uchar)
                    .offset(doto as isize) as *mut ::core::ffi::c_uchar
                    as *mut ::core::ffi::c_char;
            }
            while cp2
                != (&raw mut (*dotp).l_text as *mut ::core::ffi::c_uchar)
                    .offset((*dotp).l_used as isize) as *mut ::core::ffi::c_uchar
                    as *mut ::core::ffi::c_char
            {
                let fresh13 = cp2;
                cp2 = cp2.offset(1);
                let fresh14 = cp1;
                cp1 = cp1.offset(1);
                *fresh14 = *fresh13;
            }
            (*dotp).l_used -= chunk;
            wp = curwp;
            if (*wp).w_dotp == dotp && (*wp).w_doto >= doto {
                (*wp).w_doto -= chunk;
                if (*wp).w_doto < doto {
                    (*wp).w_doto = doto;
                }
            }
            if (*wp).w_markp == dotp && (*wp).w_marko >= doto {
                (*wp).w_marko -= chunk;
                if (*wp).w_marko < doto {
                    (*wp).w_marko = doto;
                }
            }
            n -= chunk as ::core::ffi::c_long;
        }
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn getctext() -> *mut ::core::ffi::c_char {
    let mut lp: *mut line = ::core::ptr::null_mut::<line>();
    let mut size: ::core::ffi::c_int = 0;
    let mut sp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut dp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    static mut rline: [::core::ffi::c_char; 1024] = [0; 1024];
    lp = (*curwp).w_dotp;
    sp = &raw mut (*lp).l_text as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char;
    size = (*lp).l_used;
    if size >= NSTRING {
        size = NSTRING - 1 as ::core::ffi::c_int;
    }
    dp = &raw mut rline as *mut ::core::ffi::c_char;
    loop {
        let fresh23 = size;
        size = size - 1;
        if !(fresh23 != 0) {
            break;
        }
        let fresh24 = sp;
        sp = sp.offset(1);
        let fresh25 = dp;
        dp = dp.offset(1);
        *fresh25 = *fresh24;
    }
    *dp = 0 as ::core::ffi::c_char;
    return &raw mut rline as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn putctext(
    mut iline: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut status: ::core::ffi::c_int = 0;
    (*curwp).w_doto = 0 as ::core::ffi::c_int;
    status = killtext(TRUE, 1 as ::core::ffi::c_int);
    if status != TRUE {
        return status;
    }
    status = linstr(iline);
    if status != TRUE {
        return status;
    }
    status = lnewline();
    backline(TRUE, 1 as ::core::ffi::c_int);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn ldelnewline() -> ::core::ffi::c_int {
    let mut cp1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut cp2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut lp1: *mut line = ::core::ptr::null_mut::<line>();
    let mut lp2: *mut line = ::core::ptr::null_mut::<line>();
    let mut lp3: *mut line = ::core::ptr::null_mut::<line>();
    let mut wp: *mut window = ::core::ptr::null_mut::<window>();
    if (*curbp).b_mode & MDVIEW != 0 {
        return rdonly();
    }
    lp1 = (*curwp).w_dotp;
    lp2 = (*lp1).l_fp;
    if lp2 == (*curbp).b_linep {
        if (*lp1).l_used == 0 as ::core::ffi::c_int {
            lfree(lp1);
        }
        return TRUE;
    }
    if (*lp2).l_used <= (*lp1).l_size - (*lp1).l_used {
        cp1 = (&raw mut (*lp1).l_text as *mut ::core::ffi::c_uchar)
            .offset((*lp1).l_used as isize) as *mut ::core::ffi::c_uchar
            as *mut ::core::ffi::c_char;
        cp2 = (&raw mut (*lp2).l_text as *mut ::core::ffi::c_uchar)
            .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_uchar
            as *mut ::core::ffi::c_char;
        while cp2
            != (&raw mut (*lp2).l_text as *mut ::core::ffi::c_uchar)
                .offset((*lp2).l_used as isize) as *mut ::core::ffi::c_uchar
                as *mut ::core::ffi::c_char
        {
            let fresh16 = cp2;
            cp2 = cp2.offset(1);
            let fresh17 = cp1;
            cp1 = cp1.offset(1);
            *fresh17 = *fresh16;
        }
        wp = curwp;
        if (*wp).w_linep == lp2 {
            (*wp).w_linep = lp1;
        }
        if (*wp).w_dotp == lp2 {
            (*wp).w_dotp = lp1;
            (*wp).w_doto += (*lp1).l_used;
        }
        if (*wp).w_markp == lp2 {
            (*wp).w_markp = lp1;
            (*wp).w_marko += (*lp1).l_used;
        }
        (*lp1).l_used += (*lp2).l_used;
        (*lp1).l_fp = (*lp2).l_fp;
        (*(*lp2).l_fp).l_bp = lp1;
        free(lp2 as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
        return TRUE;
    }
    lp3 = lalloc((*lp1).l_used + (*lp2).l_used);
    if lp3.is_null() {
        return FALSE;
    }
    cp1 = (&raw mut (*lp1).l_text as *mut ::core::ffi::c_uchar)
        .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_uchar
        as *mut ::core::ffi::c_char;
    cp2 = (&raw mut (*lp3).l_text as *mut ::core::ffi::c_uchar)
        .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_uchar
        as *mut ::core::ffi::c_char;
    while cp1
        != (&raw mut (*lp1).l_text as *mut ::core::ffi::c_uchar)
            .offset((*lp1).l_used as isize) as *mut ::core::ffi::c_uchar
            as *mut ::core::ffi::c_char
    {
        let fresh18 = cp1;
        cp1 = cp1.offset(1);
        let fresh19 = cp2;
        cp2 = cp2.offset(1);
        *fresh19 = *fresh18;
    }
    cp1 = (&raw mut (*lp2).l_text as *mut ::core::ffi::c_uchar)
        .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_uchar
        as *mut ::core::ffi::c_char;
    while cp1
        != (&raw mut (*lp2).l_text as *mut ::core::ffi::c_uchar)
            .offset((*lp2).l_used as isize) as *mut ::core::ffi::c_uchar
            as *mut ::core::ffi::c_char
    {
        let fresh20 = cp1;
        cp1 = cp1.offset(1);
        let fresh21 = cp2;
        cp2 = cp2.offset(1);
        *fresh21 = *fresh20;
    }
    (*(*lp1).l_bp).l_fp = lp3;
    (*lp3).l_fp = (*lp2).l_fp;
    (*(*lp2).l_fp).l_bp = lp3;
    (*lp3).l_bp = (*lp1).l_bp;
    wp = curwp;
    if (*wp).w_linep == lp1 || (*wp).w_linep == lp2 {
        (*wp).w_linep = lp3;
    }
    if (*wp).w_dotp == lp1 {
        (*wp).w_dotp = lp3;
    } else if (*wp).w_dotp == lp2 {
        (*wp).w_dotp = lp3;
        (*wp).w_doto += (*lp1).l_used;
    }
    if (*wp).w_markp == lp1 {
        (*wp).w_markp = lp3;
    } else if (*wp).w_markp == lp2 {
        (*wp).w_markp = lp3;
        (*wp).w_marko += (*lp1).l_used;
    }
    free(lp1 as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
    free(lp2 as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn kdelete() {
    let mut kp: *mut kill = ::core::ptr::null_mut::<kill>();
    if !kbufh.is_null() {
        kbufp = kbufh;
        while !kbufp.is_null() {
            kp = (*kbufp).d_next;
            free(kbufp as *mut ::core::ffi::c_void);
            kbufp = kp;
        }
        kbufp = ::core::ptr::null_mut::<kill>();
        kbufh = kbufp;
    }
    kused = KBLOCK;
}
#[no_mangle]
pub unsafe extern "C" fn kinsert(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut nchunk: *mut kill = ::core::ptr::null_mut::<kill>();
    if kused >= KBLOCK {
        nchunk = malloc(::core::mem::size_of::<kill>() as size_t) as *mut kill;
        if nchunk.is_null() {
            return FALSE;
        }
        if kbufh.is_null() {
            kbufh = nchunk;
        }
        if !kbufp.is_null() {
            (*kbufp).d_next = nchunk;
        }
        kbufp = nchunk;
        (*kbufp).d_next = ::core::ptr::null_mut::<kill>();
        kused = 0 as ::core::ffi::c_int;
    }
    let fresh15 = kused;
    kused = kused + 1;
    (*kbufp).d_chunk[fresh15 as usize] = c as ::core::ffi::c_char;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn yank(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut sp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut kp: *mut kill = ::core::ptr::null_mut::<kill>();
    if (*curbp).b_mode & MDVIEW != 0 {
        return rdonly();
    }
    if n < 0 as ::core::ffi::c_int {
        return FALSE;
    }
    if kbufh.is_null() {
        return TRUE;
    }
    loop {
        let fresh26 = n;
        n = n - 1;
        if !(fresh26 != 0) {
            break;
        }
        kp = kbufh;
        while !kp.is_null() {
            if (*kp).d_next.is_null() {
                i = kused;
            } else {
                i = KBLOCK;
            }
            sp = &raw mut (*kp).d_chunk as *mut ::core::ffi::c_char;
            loop {
                let fresh27 = i;
                i = i - 1;
                if !(fresh27 != 0) {
                    break;
                }
                let fresh28 = sp;
                sp = sp.offset(1);
                c = *fresh28 as ::core::ffi::c_int;
                if c == '\r' as i32 {
                    if lnewline() == FALSE {
                        return FALSE;
                    }
                    if i > 0 as ::core::ffi::c_int
                        && *sp as ::core::ffi::c_int == '\n' as i32
                    {
                        sp = sp.offset(1);
                        i -= 1;
                    }
                } else if c == '\n' as i32 {
                    if lnewline() == FALSE {
                        return FALSE;
                    }
                } else if linsert_byte(1 as ::core::ffi::c_int, c) == FALSE {
                    return FALSE
                }
            }
            kp = (*kp).d_next;
        }
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn linsert_block(
    mut block: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut start: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*curbp).b_mode & MDVIEW != 0 {
        return rdonly();
    }
    i = 0 as ::core::ffi::c_int;
    while i <= len {
        if i == len || *block.offset(i as isize) as ::core::ffi::c_int == '\n' as i32
            || *block.offset(i as isize) as ::core::ffi::c_int == '\r' as i32
        {
            let mut segment_len: ::core::ffi::c_int = i - start;
            if segment_len > 0 as ::core::ffi::c_int {
                let mut lp1: *mut line = (*curwp).w_dotp;
                let mut doto: ::core::ffi::c_int = (*curwp).w_doto;
                let mut lp2: *mut line = ::core::ptr::null_mut::<line>();
                if lp1 == (*curbp).b_linep {
                    lp2 = lalloc(segment_len);
                    if lp2.is_null() {
                        return FALSE;
                    }
                    let mut lp3: *mut line = (*lp1).l_bp;
                    (*lp3).l_fp = lp2;
                    (*lp2).l_fp = lp1;
                    (*lp1).l_bp = lp2;
                    (*lp2).l_bp = lp3;
                    j = 0 as ::core::ffi::c_int;
                    while j < segment_len {
                        *(&raw mut (*lp2).l_text as *mut ::core::ffi::c_uchar)
                            .offset(j as isize) = *block.offset((start + j) as isize)
                            as ::core::ffi::c_uchar;
                        j += 1;
                    }
                    (*curwp).w_dotp = lp2;
                    (*curwp).w_doto = segment_len;
                } else {
                    if (*lp1).l_used + segment_len > (*lp1).l_size {
                        lp2 = lalloc((*lp1).l_used + segment_len);
                        if lp2.is_null() {
                            return FALSE;
                        }
                        let mut cp1: *mut ::core::ffi::c_uchar = (&raw mut (*lp1).l_text
                            as *mut ::core::ffi::c_uchar)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_uchar;
                        let mut cp2: *mut ::core::ffi::c_uchar = (&raw mut (*lp2).l_text
                            as *mut ::core::ffi::c_uchar)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_uchar;
                        j = 0 as ::core::ffi::c_int;
                        while j < doto {
                            let fresh7 = cp1;
                            cp1 = cp1.offset(1);
                            let fresh8 = cp2;
                            cp2 = cp2.offset(1);
                            *fresh8 = *fresh7;
                            j += 1;
                        }
                        cp2 = cp2.offset(segment_len as isize);
                        j = doto;
                        while j < (*lp1).l_used {
                            let fresh9 = cp1;
                            cp1 = cp1.offset(1);
                            let fresh10 = cp2;
                            cp2 = cp2.offset(1);
                            *fresh10 = *fresh9;
                            j += 1;
                        }
                        (*(*lp1).l_bp).l_fp = lp2;
                        (*lp2).l_fp = (*lp1).l_fp;
                        (*(*lp1).l_fp).l_bp = lp2;
                        (*lp2).l_bp = (*lp1).l_bp;
                        free(
                            lp1 as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                        );
                        (*curwp).w_dotp = lp2;
                    } else {
                        lp2 = lp1;
                        let mut cp1_0: *mut ::core::ffi::c_uchar = (&raw mut (*lp2)
                            .l_text as *mut ::core::ffi::c_uchar)
                            .offset(
                                ((*lp2).l_used + segment_len - 1 as ::core::ffi::c_int)
                                    as isize,
                            ) as *mut ::core::ffi::c_uchar;
                        let mut cp2_0: *mut ::core::ffi::c_uchar = (&raw mut (*lp2)
                            .l_text as *mut ::core::ffi::c_uchar)
                            .offset(((*lp2).l_used - 1 as ::core::ffi::c_int) as isize)
                            as *mut ::core::ffi::c_uchar;
                        j = (*lp2).l_used - 1 as ::core::ffi::c_int;
                        while j >= doto {
                            let fresh11 = cp2_0;
                            cp2_0 = cp2_0.offset(-1);
                            let fresh12 = cp1_0;
                            cp1_0 = cp1_0.offset(-1);
                            *fresh12 = *fresh11;
                            j -= 1;
                        }
                        (*lp2).l_used += segment_len;
                    }
                    j = 0 as ::core::ffi::c_int;
                    while j < segment_len {
                        *(&raw mut (*lp2).l_text as *mut ::core::ffi::c_uchar)
                            .offset((doto + j) as isize) = *block
                            .offset((start + j) as isize) as ::core::ffi::c_uchar;
                        j += 1;
                    }
                    (*curwp).w_doto += segment_len;
                    if (*curwp).w_linep == lp1 {
                        (*curwp).w_linep = lp2;
                    }
                    if (*curwp).w_dotp == lp1 {
                        (*curwp).w_dotp = lp2;
                    }
                    if (*curwp).w_markp == lp1 {
                        (*curwp).w_markp = lp2;
                    }
                }
            }
            if i < len {
                if lnewline() == FALSE {
                    return FALSE;
                }
                if *block.offset(i as isize) as ::core::ffi::c_int == '\r' as i32
                    && (i + 1 as ::core::ffi::c_int) < len
                    && *block.offset((i + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int == '\n' as i32
                {
                    i += 1;
                }
                start = i + 1 as ::core::ffi::c_int;
            }
        }
        i += 1;
    }
    lchange(WFHARD);
    return TRUE;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
