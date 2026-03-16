extern "C" {
    fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    static mut curwp: *mut window;
    static mut curbp: *mut buffer;
    static mut bheadp: *mut buffer;
    static mut blistp: *mut buffer;
    static mut gmode: ::core::ffi::c_int;
    static mut removebackup: ::core::ffi::c_int;
    fn nanox_handle_closed_file(path: *const ::core::ffi::c_char);
    static mut tabsize: ::core::ffi::c_int;
    fn mlerase();
    fn mlwrite(fmt: *const ::core::ffi::c_char, ...);
    fn mlyesno(prompt: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn readin(
        fname: *mut ::core::ffi::c_char,
        lockfl: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn is_effectively_same(
        fname: *mut ::core::ffi::c_char,
        bp: *mut buffer,
    ) -> ::core::ffi::c_int;
    fn minibuf_input(
        prompt: *const ::core::ffi::c_char,
        dest: *mut ::core::ffi::c_char,
        max_len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn lfree(lp: *mut line);
    fn lalloc(_: ::core::ffi::c_int) -> *mut line;
}
pub type size_t = usize;
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
pub const NFILEN: ::core::ffi::c_int = 2048 as ::core::ffi::c_int;
pub const NBUFN: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WFFORCE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const WFHARD: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const WFMODE: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const BFINVS: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const BFCHG: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const BFMAKE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn usebuffer(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut bp: *mut buffer = ::core::ptr::null_mut::<buffer>();
    let mut s: ::core::ffi::c_int = 0;
    let mut bufn: [::core::ffi::c_char; 16] = [0; 16];
    s = minibuf_input(
        b"Use buffer: \0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut bufn as *mut ::core::ffi::c_char,
        NBUFN,
    );
    if s != TRUE {
        return s;
    }
    bp = bfind(&raw mut bufn as *mut ::core::ffi::c_char, TRUE, 0 as ::core::ffi::c_int);
    if bp.is_null() {
        return FALSE;
    }
    return swbuffer(bp);
}
#[no_mangle]
pub unsafe extern "C" fn nextbuffer(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut bp: *mut buffer = ::core::ptr::null_mut::<buffer>();
    let mut bbp: *mut buffer = ::core::ptr::null_mut::<buffer>();
    if f == FALSE {
        n = 1 as ::core::ffi::c_int;
    }
    if n < 1 as ::core::ffi::c_int {
        return FALSE;
    }
    bbp = curbp;
    loop {
        let fresh1 = n;
        n = n - 1;
        if !(fresh1 > 0 as ::core::ffi::c_int) {
            break;
        }
        bp = (*bbp).b_bufp;
        while bp.is_null() || (*bp).b_flag as ::core::ffi::c_int & BFINVS != 0 {
            if bp.is_null() {
                bp = bheadp;
            } else {
                bp = (*bp).b_bufp;
            }
            if bp == bbp {
                return FALSE;
            }
        }
        bbp = bp;
    }
    return swbuffer(bp);
}
#[no_mangle]
pub unsafe extern "C" fn swbuffer(mut bp: *mut buffer) -> ::core::ffi::c_int {
    (*curbp).b_nwnd -= 1;
    if (*curbp).b_nwnd as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        (*curbp).b_dotp = (*curwp).w_dotp;
        (*curbp).b_doto = (*curwp).w_doto;
        (*curbp).b_markp = (*curwp).w_markp;
        (*curbp).b_marko = (*curwp).w_marko;
    }
    curbp = bp;
    if (*curbp).b_flag as ::core::ffi::c_int & BFMAKE == 0 {
        tabsize = (*curbp).b_tabsize;
    }
    if (*curbp).b_active as ::core::ffi::c_int != TRUE {
        readin(&raw mut (*curbp).b_fname as *mut ::core::ffi::c_char, TRUE);
        (*curbp).b_dotp = (*(*curbp).b_linep).l_fp;
        (*curbp).b_doto = 0 as ::core::ffi::c_int;
        (*curbp).b_active = TRUE as ::core::ffi::c_char;
        (*curbp).b_mode |= gmode;
    }
    (*curwp).w_bufp = bp as *mut buffer;
    (*curwp).w_linep = (*bp).b_linep as *mut line;
    (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int
        | (WFMODE | WFFORCE | WFHARD)) as ::core::ffi::c_char;
    let fresh0 = (*bp).b_nwnd;
    (*bp).b_nwnd = (*bp).b_nwnd + 1;
    if fresh0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        (*curwp).w_dotp = (*bp).b_dotp;
        (*curwp).w_doto = (*bp).b_doto;
        (*curwp).w_markp = (*bp).b_markp;
        (*curwp).w_marko = (*bp).b_marko;
        return TRUE;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn killbuffer(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut bp: *mut buffer = ::core::ptr::null_mut::<buffer>();
    let mut s: ::core::ffi::c_int = 0;
    let mut bufn: [::core::ffi::c_char; 16] = [0; 16];
    s = minibuf_input(
        b"Kill buffer: \0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut bufn as *mut ::core::ffi::c_char,
        NBUFN,
    );
    if s != TRUE {
        return s;
    }
    bp = bfind(
        &raw mut bufn as *mut ::core::ffi::c_char,
        FALSE,
        0 as ::core::ffi::c_int,
    );
    if bp.is_null() {
        return TRUE;
    }
    if (*bp).b_flag as ::core::ffi::c_int & BFINVS != 0 {
        return TRUE;
    }
    return zotbuf(bp);
}
#[no_mangle]
pub unsafe extern "C" fn zotbuf(mut bp: *mut buffer) -> ::core::ffi::c_int {
    let mut bp1: *mut buffer = ::core::ptr::null_mut::<buffer>();
    let mut bp2: *mut buffer = ::core::ptr::null_mut::<buffer>();
    let mut s: ::core::ffi::c_int = 0;
    let mut closed_fname: [::core::ffi::c_char; 2048] = [0; 2048];
    if (*bp).b_nwnd as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        mlwrite(
            b"Buffer is being displayed\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return FALSE;
    }
    strcpy(
        &raw mut closed_fname as *mut ::core::ffi::c_char,
        &raw mut (*bp).b_fname as *mut ::core::ffi::c_char,
    );
    s = bclear(bp);
    if s != TRUE {
        return s;
    }
    free((*bp).b_linep as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
    bp1 = ::core::ptr::null_mut::<buffer>();
    bp2 = bheadp;
    while bp2 != bp {
        bp1 = bp2;
        bp2 = (*bp2).b_bufp;
    }
    bp2 = (*bp2).b_bufp;
    if bp1.is_null() {
        bheadp = bp2;
    } else {
        (*bp1).b_bufp = bp2;
    }
    free(bp as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
    nanox_handle_closed_file(&raw mut closed_fname as *mut ::core::ffi::c_char);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn bfreeall() {
    let mut bp: *mut buffer = bheadp;
    while !bp.is_null() {
        let mut next: *mut buffer = (*bp).b_bufp;
        let mut hlp: *mut line = (*bp).b_linep;
        if !hlp.is_null() {
            let mut lp: *mut line = (*hlp).l_fp;
            while lp != hlp {
                let mut nlp: *mut line = (*lp).l_fp;
                free(lp as *mut ::core::ffi::c_void);
                lp = nlp;
            }
            free(hlp as *mut ::core::ffi::c_void);
        }
        free(bp as *mut ::core::ffi::c_void);
        bp = next;
    }
    bheadp = ::core::ptr::null_mut::<buffer>();
}
#[no_mangle]
pub unsafe extern "C" fn namebuffer(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut bp: *mut buffer = ::core::ptr::null_mut::<buffer>();
    let mut bufn: [::core::ffi::c_char; 16] = [0; 16];
    '_ask: loop {
        if minibuf_input(
            b"Change buffer name to: \0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut bufn as *mut ::core::ffi::c_char,
            NBUFN,
        ) != TRUE
        {
            return FALSE;
        }
        bp = bheadp;
        loop {
            if bp.is_null() {
                break '_ask;
            }
            if bp != curbp {
                if strcmp(
                    &raw mut bufn as *mut ::core::ffi::c_char,
                    &raw mut (*bp).b_bname as *mut ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    break;
                }
            }
            bp = (*bp).b_bufp;
        }
    }
    strcpy(
        &raw mut (*curbp).b_bname as *mut ::core::ffi::c_char,
        &raw mut bufn as *mut ::core::ffi::c_char,
    );
    (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | WFMODE)
        as ::core::ffi::c_char;
    mlerase();
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn ltoa(
    mut buf: *mut ::core::ffi::c_char,
    mut width: ::core::ffi::c_int,
    mut num: ::core::ffi::c_long,
) {
    *buf.offset(width as isize) = 0 as ::core::ffi::c_char;
    while num >= 10 as ::core::ffi::c_long {
        width -= 1;
        *buf.offset(width as isize) = ((num % 10 as ::core::ffi::c_long)
            as ::core::ffi::c_int + '0' as i32) as ::core::ffi::c_char;
        num /= 10 as ::core::ffi::c_long;
    }
    width -= 1;
    *buf.offset(width as isize) = (num as ::core::ffi::c_int + '0' as i32)
        as ::core::ffi::c_char;
    while width != 0 as ::core::ffi::c_int {
        width -= 1;
        *buf.offset(width as isize) = ' ' as i32 as ::core::ffi::c_char;
    }
}
#[no_mangle]
pub unsafe extern "C" fn addline(
    mut text: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut lp: *mut line = ::core::ptr::null_mut::<line>();
    let mut i: ::core::ffi::c_int = 0;
    let mut ntext: ::core::ffi::c_int = 0;
    ntext = strlen(text) as ::core::ffi::c_int;
    lp = lalloc(ntext);
    if lp.is_null() {
        return FALSE;
    }
    i = 0 as ::core::ffi::c_int;
    while i < ntext {
        *(&raw mut (*lp).l_text as *mut ::core::ffi::c_uchar).offset(i as isize) = *text
            .offset(i as isize) as ::core::ffi::c_uchar;
        i += 1;
    }
    (*(*(*blistp).b_linep).l_bp).l_fp = lp;
    (*lp).l_bp = (*(*blistp).b_linep).l_bp;
    (*(*blistp).b_linep).l_bp = lp;
    (*lp).l_fp = (*blistp).b_linep;
    if (*blistp).b_dotp == (*blistp).b_linep {
        (*blistp).b_dotp = lp;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn anycb() -> ::core::ffi::c_int {
    let mut bp: *mut buffer = ::core::ptr::null_mut::<buffer>();
    bp = bheadp;
    while !bp.is_null() {
        if (*bp).b_flag as ::core::ffi::c_int & BFINVS == 0 as ::core::ffi::c_int
            && (*bp).b_flag as ::core::ffi::c_int & BFCHG != 0 as ::core::ffi::c_int
        {
            return TRUE;
        }
        bp = (*bp).b_bufp;
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn bfind(
    mut bname: *mut ::core::ffi::c_char,
    mut cflag: ::core::ffi::c_int,
    mut bflag: ::core::ffi::c_int,
) -> *mut buffer {
    let mut bp: *mut buffer = ::core::ptr::null_mut::<buffer>();
    let mut sb: *mut buffer = ::core::ptr::null_mut::<buffer>();
    let mut lp: *mut line = ::core::ptr::null_mut::<line>();
    bp = bheadp;
    while !bp.is_null() {
        if strcmp(bname, &raw mut (*bp).b_bname as *mut ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            return bp;
        }
        bp = (*bp).b_bufp;
    }
    if cflag != FALSE {
        bp = malloc(::core::mem::size_of::<buffer>() as size_t) as *mut buffer;
        if bp.is_null() {
            return ::core::ptr::null_mut::<buffer>();
        }
        lp = lalloc(0 as ::core::ffi::c_int);
        if lp.is_null() {
            free(bp as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<buffer>();
        }
        if bheadp.is_null()
            || strcmp(&raw mut (*bheadp).b_bname as *mut ::core::ffi::c_char, bname)
                > 0 as ::core::ffi::c_int
        {
            (*bp).b_bufp = bheadp;
            bheadp = bp;
        } else {
            sb = bheadp;
            while !(*sb).b_bufp.is_null() {
                if strcmp(
                    &raw mut (*(*sb).b_bufp).b_bname as *mut ::core::ffi::c_char,
                    bname,
                ) > 0 as ::core::ffi::c_int
                {
                    break;
                }
                sb = (*sb).b_bufp;
            }
            (*bp).b_bufp = (*sb).b_bufp;
            (*sb).b_bufp = bp;
        }
        (*bp).b_active = TRUE as ::core::ffi::c_char;
        (*bp).b_dotp = lp;
        (*bp).b_doto = 0 as ::core::ffi::c_int;
        (*bp).b_markp = ::core::ptr::null_mut::<line>();
        (*bp).b_marko = 0 as ::core::ffi::c_int;
        (*bp).b_flag = bflag as ::core::ffi::c_char;
        (*bp).b_mode = gmode;
        (*bp).b_nwnd = 0 as ::core::ffi::c_char;
        (*bp).b_linep = lp;
        (*bp).b_tabsize = tabsize;
        strcpy(
            &raw mut (*bp).b_fname as *mut ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
        );
        strcpy(&raw mut (*bp).b_bname as *mut ::core::ffi::c_char, bname);
        (*lp).l_fp = lp;
        (*lp).l_bp = lp;
    }
    return bp;
}
#[no_mangle]
pub unsafe extern "C" fn cleanup_backup(
    mut bp: *mut buffer,
    mut force: ::core::ffi::c_int,
) {
    if (force != 0 || removebackup != 0)
        && (*bp).b_flag as ::core::ffi::c_int & BFINVS == 0 as ::core::ffi::c_int
        && (*bp).b_fname[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            != '\0' as i32
    {
        let mut backupName: [::core::ffi::c_char; 2048] = [0; 2048];
        if strlen(&raw mut (*bp).b_fname as *mut ::core::ffi::c_char)
            .wrapping_add(2 as size_t) < NFILEN as size_t
        {
            strcpy(
                &raw mut backupName as *mut ::core::ffi::c_char,
                &raw mut (*bp).b_fname as *mut ::core::ffi::c_char,
            );
            strcat(
                &raw mut backupName as *mut ::core::ffi::c_char,
                b"~\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if force != 0 || (*bp).b_flag as ::core::ffi::c_int & BFCHG == 0
                || is_effectively_same(
                    &raw mut (*bp).b_fname as *mut ::core::ffi::c_char,
                    bp,
                ) != 0
            {
                unlink(&raw mut backupName as *mut ::core::ffi::c_char);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn bclear(mut bp: *mut buffer) -> ::core::ffi::c_int {
    let mut lp: *mut line = ::core::ptr::null_mut::<line>();
    let mut s: ::core::ffi::c_int = 0;
    cleanup_backup(bp, FALSE);
    if (*bp).b_flag as ::core::ffi::c_int & BFINVS == 0 as ::core::ffi::c_int
        && (*bp).b_flag as ::core::ffi::c_int & BFCHG != 0 as ::core::ffi::c_int
        && {
            s = mlyesno(
                b"Discard changes\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
            s != TRUE
        }
    {
        return s;
    }
    (*bp).b_flag = ((*bp).b_flag as ::core::ffi::c_int & !BFCHG) as ::core::ffi::c_char;
    loop {
        lp = (*(*bp).b_linep).l_fp;
        if !(lp != (*bp).b_linep) {
            break;
        }
        lfree(lp);
    }
    (*bp).b_dotp = (*bp).b_linep;
    (*bp).b_doto = 0 as ::core::ffi::c_int;
    (*bp).b_markp = ::core::ptr::null_mut::<line>();
    (*bp).b_marko = 0 as ::core::ffi::c_int;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn unmark(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    (*curbp).b_flag = ((*curbp).b_flag as ::core::ffi::c_int & !BFCHG)
        as ::core::ffi::c_char;
    (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | WFMODE)
        as ::core::ffi::c_char;
    return TRUE;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
