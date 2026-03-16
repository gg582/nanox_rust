extern "C" {
    fn iswalpha(__wc: wint_t) -> ::core::ffi::c_int;
    fn iswdigit(__wc: wint_t) -> ::core::ffi::c_int;
    fn towlower(__wc: wint_t) -> wint_t;
    fn towupper(__wc: wint_t) -> wint_t;
    static mut fillcol: ::core::ffi::c_int;
    static mut thisflag: ::core::ffi::c_int;
    static mut lastflag: ::core::ffi::c_int;
    static mut curwp: *mut window;
    static mut curbp: *mut buffer;
    static mut justflag: ::core::ffi::c_int;
    fn utf8_to_unicode(
        line: *mut ::core::ffi::c_uchar,
        index: ::core::ffi::c_uint,
        len: ::core::ffi::c_uint,
        res: *mut unicode_t,
    ) -> ::core::ffi::c_uint;
    fn linsert(n: ::core::ffi::c_int, c: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn lnewline() -> ::core::ffi::c_int;
    fn ldelete(n: ::core::ffi::c_long, kflag: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn lgetchar(_: *mut unicode_t) -> ::core::ffi::c_int;
    fn kdelete();
    fn backchar(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn gotoeol(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn forwchar(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn gotobop(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn gotoeop(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn forwdel(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn rdonly() -> ::core::ffi::c_int;
    fn mlwrite(fmt: *const ::core::ffi::c_char, ...);
    fn killregion(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn getregion(rp: *mut region) -> ::core::ffi::c_int;
}
pub type wint_t = ::core::ffi::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct region {
    pub r_linep: *mut line,
    pub r_offset: ::core::ffi::c_int,
    pub r_size: ::core::ffi::c_long,
}
pub type unicode_t = ::core::ffi::c_uint;
pub const NSTRING: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CFKILL: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const TAB: ::core::ffi::c_int = 0x9 as ::core::ffi::c_int;
pub const MDVIEW: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn wrapword(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut cnt: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    if backchar(0 as ::core::ffi::c_int, 1 as ::core::ffi::c_int) == 0 {
        return FALSE;
    }
    cnt = 0 as ::core::ffi::c_int;
    loop {
        c = *(&raw mut (*(*curwp).w_dotp).l_text as *mut ::core::ffi::c_uchar)
            .offset((*curwp).w_doto as isize) as ::core::ffi::c_int
            & 0xff as ::core::ffi::c_int;
        if !(c != ' ' as i32 && c != '\t' as i32) {
            break;
        }
        cnt += 1;
        if backchar(0 as ::core::ffi::c_int, 1 as ::core::ffi::c_int) == 0 {
            return FALSE;
        }
        if (*curwp).w_doto == 0 as ::core::ffi::c_int {
            gotoeol(FALSE, 0 as ::core::ffi::c_int);
            return lnewline();
        }
    }
    if forwdel(0 as ::core::ffi::c_int, 1 as ::core::ffi::c_int) == 0 {
        return FALSE;
    }
    if lnewline() == 0 {
        return FALSE;
    }
    loop {
        let fresh0 = cnt;
        cnt = cnt - 1;
        if !(fresh0 > 0 as ::core::ffi::c_int) {
            break;
        }
        if forwchar(FALSE, 1 as ::core::ffi::c_int) == FALSE {
            return FALSE;
        }
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn backword(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if n < 0 as ::core::ffi::c_int {
        return forwword(f, -n);
    }
    if backchar(FALSE, 1 as ::core::ffi::c_int) == FALSE {
        return FALSE;
    }
    loop {
        let fresh1 = n;
        n = n - 1;
        if !(fresh1 != 0) {
            break;
        }
        while inword() == FALSE {
            if backchar(FALSE, 1 as ::core::ffi::c_int) == FALSE {
                return FALSE;
            }
        }
        while inword() != FALSE {
            if backchar(FALSE, 1 as ::core::ffi::c_int) == FALSE {
                return FALSE;
            }
        }
    }
    return forwchar(FALSE, 1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn forwword(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if n < 0 as ::core::ffi::c_int {
        return backword(f, -n);
    }
    loop {
        let fresh2 = n;
        n = n - 1;
        if !(fresh2 != 0) {
            break;
        }
        while inword() == TRUE {
            if forwchar(FALSE, 1 as ::core::ffi::c_int) == FALSE {
                return FALSE;
            }
        }
        while inword() == FALSE {
            if forwchar(FALSE, 1 as ::core::ffi::c_int) == FALSE {
                return FALSE;
            }
        }
    }
    return TRUE;
}
unsafe extern "C" fn transform_word(
    mut transform: Option<unsafe extern "C" fn(wint_t) -> wint_t>,
) {
    let mut c: unicode_t = 0;
    let mut len: ::core::ffi::c_int = 0;
    while inword() != 0 {
        len = lgetchar(&raw mut c);
        let mut nc: unicode_t = transform
            .expect("non-null function pointer")(c as wint_t) as unicode_t;
        if nc != c {
            ldelete(len as ::core::ffi::c_long, FALSE);
            linsert(1 as ::core::ffi::c_int, nc as ::core::ffi::c_int);
        } else {
            forwchar(FALSE, 1 as ::core::ffi::c_int);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn upperword(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if (*curbp).b_mode & MDVIEW != 0 {
        return rdonly();
    }
    if n < 0 as ::core::ffi::c_int {
        return FALSE;
    }
    loop {
        let fresh3 = n;
        n = n - 1;
        if !(fresh3 != 0) {
            break;
        }
        while inword() == FALSE {
            if forwchar(FALSE, 1 as ::core::ffi::c_int) == FALSE {
                return FALSE;
            }
        }
        transform_word(Some(towupper as unsafe extern "C" fn(wint_t) -> wint_t));
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn lowerword(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if (*curbp).b_mode & MDVIEW != 0 {
        return rdonly();
    }
    if n < 0 as ::core::ffi::c_int {
        return FALSE;
    }
    loop {
        let fresh4 = n;
        n = n - 1;
        if !(fresh4 != 0) {
            break;
        }
        while inword() == FALSE {
            if forwchar(FALSE, 1 as ::core::ffi::c_int) == FALSE {
                return FALSE;
            }
        }
        transform_word(Some(towlower as unsafe extern "C" fn(wint_t) -> wint_t));
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn capword(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut c: unicode_t = 0;
    let mut len: ::core::ffi::c_int = 0;
    if (*curbp).b_mode & MDVIEW != 0 {
        return rdonly();
    }
    if n < 0 as ::core::ffi::c_int {
        return FALSE;
    }
    loop {
        let fresh5 = n;
        n = n - 1;
        if !(fresh5 != 0) {
            break;
        }
        while inword() == FALSE {
            if forwchar(FALSE, 1 as ::core::ffi::c_int) == FALSE {
                return FALSE;
            }
        }
        if inword() != FALSE {
            len = lgetchar(&raw mut c);
            let mut nc: unicode_t = towupper(c as wint_t) as unicode_t;
            if nc != c {
                ldelete(len as ::core::ffi::c_long, FALSE);
                linsert(1 as ::core::ffi::c_int, nc as ::core::ffi::c_int);
            } else {
                forwchar(FALSE, 1 as ::core::ffi::c_int);
            }
            transform_word(Some(towlower as unsafe extern "C" fn(wint_t) -> wint_t));
        }
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn delfword(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut dotp: *mut line = ::core::ptr::null_mut::<line>();
    let mut doto: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    let mut size: ::core::ffi::c_long = 0;
    if (*curbp).b_mode & MDVIEW != 0 {
        return rdonly();
    }
    if n < 0 as ::core::ffi::c_int {
        return FALSE;
    }
    if lastflag & CFKILL == 0 as ::core::ffi::c_int {
        kdelete();
    }
    thisflag |= CFKILL;
    dotp = (*curwp).w_dotp;
    doto = (*curwp).w_doto;
    while inword() == FALSE {
        if forwchar(FALSE, 1 as ::core::ffi::c_int) == FALSE {
            return FALSE;
        }
    }
    if n == 0 as ::core::ffi::c_int {
        while inword() == TRUE {
            if forwchar(FALSE, 1 as ::core::ffi::c_int) == FALSE {
                return FALSE;
            }
        }
    } else {
        loop {
            let fresh6 = n;
            n = n - 1;
            if !(fresh6 != 0) {
                break;
            }
            while (*curwp).w_doto == (*(*curwp).w_dotp).l_used {
                if forwchar(FALSE, 1 as ::core::ffi::c_int) == FALSE {
                    return FALSE;
                }
            }
            while inword() == TRUE {
                if forwchar(FALSE, 1 as ::core::ffi::c_int) == FALSE {
                    return FALSE;
                }
            }
            if n != 0 as ::core::ffi::c_int {
                while inword() == FALSE {
                    if forwchar(FALSE, 1 as ::core::ffi::c_int) == FALSE {
                        return FALSE;
                    }
                }
            }
        }
        while (*curwp).w_doto == (*(*curwp).w_dotp).l_used
            || {
                c = *(&raw mut (*(*curwp).w_dotp).l_text as *mut ::core::ffi::c_uchar)
                    .offset((*curwp).w_doto as isize) as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int;
                c == ' ' as i32
            } || c == '\t' as i32
        {
            if forwchar(FALSE, 1 as ::core::ffi::c_int) == FALSE {
                break;
            }
        }
    }
    let mut lp: *mut line = dotp;
    size = 0 as ::core::ffi::c_long;
    while lp != (*curwp).w_dotp {
        size += ((*lp).l_used + 1 as ::core::ffi::c_int) as ::core::ffi::c_long;
        lp = (*lp).l_fp;
    }
    size += (*curwp).w_doto as ::core::ffi::c_long;
    size -= doto as ::core::ffi::c_long;
    (*curwp).w_dotp = dotp;
    (*curwp).w_doto = doto;
    return ldelete(size, TRUE);
}
#[no_mangle]
pub unsafe extern "C" fn delbword(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut size: ::core::ffi::c_long = 0;
    let mut dotp: *mut line = ::core::ptr::null_mut::<line>();
    let mut endp: *mut line = ::core::ptr::null_mut::<line>();
    let mut doto: ::core::ffi::c_int = 0;
    let mut endo: ::core::ffi::c_int = 0;
    if (*curbp).b_mode & MDVIEW != 0 {
        return rdonly();
    }
    if n <= 0 as ::core::ffi::c_int {
        return FALSE;
    }
    if lastflag & CFKILL == 0 as ::core::ffi::c_int {
        kdelete();
    }
    thisflag |= CFKILL;
    endp = (*curwp).w_dotp;
    endo = (*curwp).w_doto;
    if backchar(FALSE, 1 as ::core::ffi::c_int) == FALSE {
        return FALSE;
    }
    's_45: loop {
        let fresh7 = n;
        n = n - 1;
        if !(fresh7 != 0) {
            current_block = 8831408221741692167;
            break;
        }
        while inword() == FALSE {
            if backchar(FALSE, 1 as ::core::ffi::c_int) == FALSE {
                return FALSE;
            }
        }
        while inword() != FALSE {
            if backchar(FALSE, 1 as ::core::ffi::c_int) == FALSE {
                current_block = 6080698592400345370;
                break 's_45;
            }
        }
    }
    match current_block {
        8831408221741692167 => {
            if forwchar(FALSE, 1 as ::core::ffi::c_int) == FALSE {
                return FALSE;
            }
        }
        _ => {}
    }
    dotp = (*curwp).w_dotp;
    doto = (*curwp).w_doto;
    let mut lp: *mut line = dotp;
    size = 0 as ::core::ffi::c_long;
    while lp != endp {
        size += ((*lp).l_used + 1 as ::core::ffi::c_int) as ::core::ffi::c_long;
        lp = (*lp).l_fp;
    }
    size += endo as ::core::ffi::c_long;
    size -= doto as ::core::ffi::c_long;
    return ldelete(size, TRUE);
}
#[no_mangle]
pub unsafe extern "C" fn inword() -> ::core::ffi::c_int {
    let mut c: unicode_t = 0;
    if (*curwp).w_doto == (*(*curwp).w_dotp).l_used {
        return FALSE;
    }
    lgetchar(&raw mut c);
    if iswalpha(c as wint_t) != 0 || iswdigit(c as wint_t) != 0 {
        return TRUE;
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn fillpara(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut c: unicode_t = 0;
    let mut wbuf: [unicode_t; 1024] = [0; 1024];
    let mut wordlen: ::core::ffi::c_int = 0;
    let mut clength: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut newlength: ::core::ffi::c_int = 0;
    let mut eopflag: ::core::ffi::c_int = 0;
    let mut firstflag: ::core::ffi::c_int = 0;
    let mut eopline: *mut line = ::core::ptr::null_mut::<line>();
    let mut dotflag: ::core::ffi::c_int = 0;
    if (*curbp).b_mode & MDVIEW != 0 {
        return rdonly();
    }
    if fillcol == 0 as ::core::ffi::c_int {
        mlwrite(b"No fill column set\0" as *const u8 as *const ::core::ffi::c_char);
        return FALSE;
    }
    justflag = FALSE;
    gotoeop(FALSE, 1 as ::core::ffi::c_int);
    eopline = (*(*curwp).w_dotp).l_fp;
    gotobop(FALSE, 1 as ::core::ffi::c_int);
    clength = (*curwp).w_doto;
    if clength != 0
        && *(&raw mut (*(*curwp).w_dotp).l_text as *mut ::core::ffi::c_uchar)
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == TAB
    {
        clength = 8 as ::core::ffi::c_int;
    }
    wordlen = 0 as ::core::ffi::c_int;
    dotflag = FALSE;
    firstflag = TRUE;
    eopflag = FALSE;
    while eopflag == 0 {
        let mut bytes: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        if (*curwp).w_doto == (*(*curwp).w_dotp).l_used {
            c = ' ' as i32 as unicode_t;
            if (*(*curwp).w_dotp).l_fp == eopline {
                eopflag = TRUE;
            }
        } else {
            bytes = lgetchar(&raw mut c);
        }
        ldelete(bytes as ::core::ffi::c_long, FALSE);
        if c != ' ' as i32 as unicode_t && c != '\t' as i32 as unicode_t {
            dotflag = (c == '.' as i32 as unicode_t) as ::core::ffi::c_int;
            if wordlen < NSTRING - 1 as ::core::ffi::c_int {
                let fresh8 = wordlen;
                wordlen = wordlen + 1;
                wbuf[fresh8 as usize] = c;
            }
        } else if wordlen != 0 {
            newlength = clength + 1 as ::core::ffi::c_int + wordlen;
            if newlength <= fillcol {
                if firstflag == 0 {
                    linsert(1 as ::core::ffi::c_int, ' ' as i32);
                    clength += 1;
                }
                firstflag = FALSE;
            } else {
                lnewline();
                clength = 0 as ::core::ffi::c_int;
            }
            i = 0 as ::core::ffi::c_int;
            while i < wordlen {
                linsert(1 as ::core::ffi::c_int, wbuf[i as usize] as ::core::ffi::c_int);
                clength += 1;
                i += 1;
            }
            if dotflag != 0 {
                linsert(1 as ::core::ffi::c_int, ' ' as i32);
                clength += 1;
            }
            wordlen = 0 as ::core::ffi::c_int;
        }
    }
    lnewline();
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn justpara(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut c: unicode_t = 0;
    let mut wbuf: [unicode_t; 1024] = [0; 1024];
    let mut wordlen: ::core::ffi::c_int = 0;
    let mut clength: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut newlength: ::core::ffi::c_int = 0;
    let mut eopflag: ::core::ffi::c_int = 0;
    let mut firstflag: ::core::ffi::c_int = 0;
    let mut eopline: *mut line = ::core::ptr::null_mut::<line>();
    let mut leftmarg: ::core::ffi::c_int = 0;
    if (*curbp).b_mode & MDVIEW != 0 {
        return rdonly();
    }
    if fillcol == 0 as ::core::ffi::c_int {
        mlwrite(b"No fill column set\0" as *const u8 as *const ::core::ffi::c_char);
        return FALSE;
    }
    justflag = TRUE;
    leftmarg = (*curwp).w_doto;
    if leftmarg + 10 as ::core::ffi::c_int > fillcol {
        leftmarg = 0 as ::core::ffi::c_int;
        mlwrite(b"Column too narrow\0" as *const u8 as *const ::core::ffi::c_char);
        return FALSE;
    }
    gotoeop(FALSE, 1 as ::core::ffi::c_int);
    eopline = (*(*curwp).w_dotp).l_fp;
    gotobop(FALSE, 1 as ::core::ffi::c_int);
    if leftmarg < (*(*curwp).w_dotp).l_used {
        (*curwp).w_doto = leftmarg;
    }
    clength = (*curwp).w_doto;
    if clength != 0
        && *(&raw mut (*(*curwp).w_dotp).l_text as *mut ::core::ffi::c_uchar)
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == TAB
    {
        clength = 8 as ::core::ffi::c_int;
    }
    wordlen = 0 as ::core::ffi::c_int;
    firstflag = TRUE;
    eopflag = FALSE;
    while eopflag == 0 {
        let mut bytes: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        if (*curwp).w_doto == (*(*curwp).w_dotp).l_used {
            c = ' ' as i32 as unicode_t;
            if (*(*curwp).w_dotp).l_fp == eopline {
                eopflag = TRUE;
            }
        } else {
            bytes = lgetchar(&raw mut c);
        }
        ldelete(bytes as ::core::ffi::c_long, FALSE);
        if c != ' ' as i32 as unicode_t && c != '\t' as i32 as unicode_t {
            if wordlen < NSTRING - 1 as ::core::ffi::c_int {
                let fresh9 = wordlen;
                wordlen = wordlen + 1;
                wbuf[fresh9 as usize] = c;
            }
        } else if wordlen != 0 {
            newlength = clength + 1 as ::core::ffi::c_int + wordlen;
            if newlength <= fillcol {
                if firstflag == 0 {
                    linsert(1 as ::core::ffi::c_int, ' ' as i32);
                    clength += 1;
                }
                firstflag = FALSE;
            } else {
                lnewline();
                i = 0 as ::core::ffi::c_int;
                while i < leftmarg {
                    linsert(1 as ::core::ffi::c_int, ' ' as i32);
                    i += 1;
                }
                clength = leftmarg;
            }
            i = 0 as ::core::ffi::c_int;
            while i < wordlen {
                linsert(1 as ::core::ffi::c_int, wbuf[i as usize] as ::core::ffi::c_int);
                clength += 1;
                i += 1;
            }
            wordlen = 0 as ::core::ffi::c_int;
        }
    }
    lnewline();
    forwword(FALSE, 1 as ::core::ffi::c_int);
    if (*(*curwp).w_dotp).l_used > leftmarg {
        (*curwp).w_doto = leftmarg;
    } else {
        (*curwp).w_doto = (*(*curwp).w_dotp).l_used;
    }
    justflag = FALSE;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn killpara(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut status: ::core::ffi::c_int = 0;
    loop {
        let fresh10 = n;
        n = n - 1;
        if !(fresh10 != 0) {
            break;
        }
        gotoeop(FALSE, 1 as ::core::ffi::c_int);
        (*curwp).w_markp = (*curwp).w_dotp;
        (*curwp).w_marko = (*curwp).w_doto;
        gotobop(FALSE, 1 as ::core::ffi::c_int);
        (*curwp).w_doto = 0 as ::core::ffi::c_int;
        status = killregion(FALSE, 1 as ::core::ffi::c_int);
        if status != TRUE {
            return status;
        }
        ldelete(2 as ::core::ffi::c_long, TRUE);
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn wordcount(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut lp: *mut line = ::core::ptr::null_mut::<line>();
    let mut offset: ::core::ffi::c_int = 0;
    let mut size: ::core::ffi::c_long = 0;
    let mut ch: unicode_t = 0;
    let mut wordflag: ::core::ffi::c_int = 0;
    let mut lastword: ::core::ffi::c_int = 0;
    let mut nwords: ::core::ffi::c_long = 0;
    let mut nchars: ::core::ffi::c_long = 0;
    let mut nlines: ::core::ffi::c_int = 0;
    let mut avgch: ::core::ffi::c_int = 0;
    let mut status: ::core::ffi::c_int = 0;
    let mut region: region = region {
        r_linep: ::core::ptr::null_mut::<line>(),
        r_offset: 0,
        r_size: 0,
    };
    status = getregion(&raw mut region);
    if status != TRUE {
        return status;
    }
    lp = region.r_linep;
    offset = region.r_offset;
    size = region.r_size;
    lastword = FALSE;
    nchars = 0 as ::core::ffi::c_long;
    nwords = 0 as ::core::ffi::c_long;
    nlines = 0 as ::core::ffi::c_int;
    while size > 0 as ::core::ffi::c_long {
        let mut bytes: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        if offset == (*lp).l_used {
            ch = '\n' as i32 as unicode_t;
            lp = (*lp).l_fp;
            offset = 0 as ::core::ffi::c_int;
            nlines += 1;
            bytes = 1 as ::core::ffi::c_int;
        } else {
            bytes = utf8_to_unicode(
                &raw mut (*lp).l_text as *mut ::core::ffi::c_uchar,
                offset as ::core::ffi::c_uint,
                (*lp).l_used as ::core::ffi::c_uint,
                &raw mut ch,
            ) as ::core::ffi::c_int;
            offset += bytes;
        }
        size -= bytes as ::core::ffi::c_long;
        wordflag = (iswalpha(ch as wint_t) != 0 || iswdigit(ch as wint_t) != 0)
            as ::core::ffi::c_int;
        if wordflag == TRUE && lastword == FALSE {
            nwords += 1;
        }
        lastword = wordflag;
        nchars += 1;
    }
    if nwords > 0 as ::core::ffi::c_long {
        avgch = (100 as ::core::ffi::c_long * nchars / nwords) as ::core::ffi::c_int;
    } else {
        avgch = 0 as ::core::ffi::c_int;
    }
    mlwrite(
        b"Words %D Chars %D Lines %d Avg chars/word %f\0" as *const u8
            as *const ::core::ffi::c_char,
        nwords,
        nchars,
        nlines + 1 as ::core::ffi::c_int,
        avgch,
    );
    return TRUE;
}
