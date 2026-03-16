extern "C" {
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
    static mut patmatch: *mut ::core::ffi::c_char;
    static mut curwp: *mut window;
    static mut curbp: *mut buffer;
    static mut pat: [::core::ffi::c_char; 0];
    static mut tap: [::core::ffi::c_char; 0];
    static mut matchlen: ::core::ffi::c_uint;
    static mut mlenold: ::core::ffi::c_uint;
    static mut mcpat: *mut magic;
    static mut tapcm: *mut magic;
    static mut metac: ::core::ffi::c_int;
    static mut matchline: *mut line;
    static mut matchoff: ::core::ffi::c_int;
    fn nanox_request_underbar_redraw();
    fn update(force: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn mlwrite(fmt: *const ::core::ffi::c_char, ...);
    fn mlreplyt(
        prompt: *mut ::core::ffi::c_char,
        buf: *mut ::core::ffi::c_char,
        nbuf: ::core::ffi::c_int,
        eolchar: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn tgetc() -> ::core::ffi::c_int;
    fn minibuf_input(
        prompt: *const ::core::ffi::c_char,
        dest: *mut ::core::ffi::c_char,
        max_len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct magic {
    pub mc_type: ::core::ffi::c_short,
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub lchar: ::core::ffi::c_int,
    pub cclmap: *mut ::core::ffi::c_char,
}
#[inline]
unsafe extern "C" fn is_beginning_utf8(
    mut c: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    return (c as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
        != 0x80 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
pub const NPAT: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const CONTROL: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PTBEG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PTEND: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FORWARD: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const REVERSE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DIFCASE: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const WFMOVE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MDEXACT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const MDMAGIC: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const MCNIL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LITCHAR: ::core::ffi::c_int = 1;
pub const ANY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CCL: ::core::ffi::c_int = 3;
pub const NCCL: ::core::ffi::c_int = 4;
pub const BOL: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const EOL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const CLOSURE: ::core::ffi::c_int = 2048 as ::core::ffi::c_int;
pub const MASKCL: ::core::ffi::c_int = CLOSURE - 1 as ::core::ffi::c_int;
pub const MC_ANY: ::core::ffi::c_int = 46;
pub const MC_CCL: ::core::ffi::c_int = 91;
pub const MC_NCCL: ::core::ffi::c_int = '^' as i32;
pub const MC_RCCL: ::core::ffi::c_int = 45;
pub const MC_ECCL: ::core::ffi::c_int = ']' as i32;
pub const MC_BOL: ::core::ffi::c_int = 94;
pub const MC_EOL: ::core::ffi::c_int = 36;
pub const MC_CLOSURE: ::core::ffi::c_int = 42;
pub const MC_ESC: ::core::ffi::c_int = 92;
pub const HICHAR: ::core::ffi::c_int = 2048 as ::core::ffi::c_int;
static mut magical: ::core::ffi::c_short = 0;
#[no_mangle]
pub unsafe extern "C" fn forwsearch(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut status: ::core::ffi::c_int = TRUE;
    if n < 0 as ::core::ffi::c_int {
        return backsearch(f, -n);
    }
    status = readpattern(
        b"Search\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        (&raw mut pat as *mut ::core::ffi::c_char)
            .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
        TRUE,
    );
    if status == TRUE {
        loop {
            if (magical as ::core::ffi::c_int != 0
                && (*(*curwp).w_bufp).b_mode & MDMAGIC != 0) as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
            {
                status = mcscanner(
                    (&raw mut mcpat as *mut magic)
                        .offset(0 as ::core::ffi::c_int as isize) as *mut magic,
                    FORWARD,
                    PTEND,
                );
            } else {
                status = scanner(
                    (&raw mut pat as *mut ::core::ffi::c_char)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char,
                    FORWARD,
                    PTEND,
                );
            }
            n -= 1;
            if !(n > 0 as ::core::ffi::c_int && status != 0) {
                break;
            }
        }
        if status == TRUE {
            savematch();
        } else {
            mlwrite(b"Not found\0" as *const u8 as *const ::core::ffi::c_char);
        }
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn nanox_search_engine(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut query: [::core::ffi::c_char; 1024] = [0; 1024];
    static mut last_dir: ::core::ffi::c_int = FORWARD;
    loop {
        let mut input_status: ::core::ffi::c_int = minibuf_input(
            b"Search: \0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut query as *mut ::core::ffi::c_char,
            NPAT,
        );
        if input_status != TRUE {
            nanox_request_underbar_redraw();
            return input_status;
        }
        let mut dir: ::core::ffi::c_int = last_dir;
        let mut status: ::core::ffi::c_int = TRUE;
        let mut len: ::core::ffi::c_int = strlen(
            &raw mut query as *mut ::core::ffi::c_char,
        ) as ::core::ffi::c_int;
        let mut has_suffix: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if query[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '\0' as i32 {
            if *(&raw mut pat as *mut ::core::ffi::c_char)
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\0' as i32
            {
                mlwrite(b"No pattern set\0" as *const u8 as *const ::core::ffi::c_char);
                nanox_request_underbar_redraw();
                return FALSE;
            }
        } else {
            if len >= 3 as ::core::ffi::c_int {
                if strcmp(
                    (&raw mut query as *mut ::core::ffi::c_char)
                        .offset(len as isize)
                        .offset(-(3 as ::core::ffi::c_int as isize)),
                    b"&nx\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    dir = FORWARD;
                    query[(len - 3 as ::core::ffi::c_int) as usize] = '\0' as i32
                        as ::core::ffi::c_char;
                    has_suffix = 1 as ::core::ffi::c_int;
                } else if strcmp(
                    (&raw mut query as *mut ::core::ffi::c_char)
                        .offset(len as isize)
                        .offset(-(3 as ::core::ffi::c_int as isize)),
                    b"&pr\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    dir = REVERSE;
                    query[(len - 3 as ::core::ffi::c_int) as usize] = '\0' as i32
                        as ::core::ffi::c_char;
                    has_suffix = 1 as ::core::ffi::c_int;
                }
            }
            if query[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                != '\0' as i32
            {
                strcpy(
                    &raw mut pat as *mut ::core::ffi::c_char,
                    &raw mut query as *mut ::core::ffi::c_char,
                );
                rvstrscpy(
                    &raw mut tap as *mut ::core::ffi::c_char,
                    &raw mut query as *mut ::core::ffi::c_char,
                    NPAT,
                );
                if (*(*curwp).w_bufp).b_mode & MDMAGIC != 0 {
                    mcstr();
                }
            } else if has_suffix != 0 {
                if *(&raw mut pat as *mut ::core::ffi::c_char)
                    .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\0' as i32
                {
                    mlwrite(
                        b"No pattern set\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    nanox_request_underbar_redraw();
                    return FALSE;
                }
            }
            last_dir = dir;
        }
        let mut restart: ::core::ffi::c_int = FALSE;
        loop {
            if (magical as ::core::ffi::c_int != 0
                && (*(*curwp).w_bufp).b_mode & MDMAGIC != 0) as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
            {
                if dir == FORWARD {
                    status = mcscanner(
                        (&raw mut mcpat as *mut magic)
                            .offset(0 as ::core::ffi::c_int as isize) as *mut magic,
                        FORWARD,
                        PTEND,
                    );
                } else {
                    status = mcscanner(
                        (&raw mut tapcm as *mut magic)
                            .offset(0 as ::core::ffi::c_int as isize) as *mut magic,
                        REVERSE,
                        PTBEG,
                    );
                }
            } else if dir == FORWARD {
                status = scanner(
                    (&raw mut pat as *mut ::core::ffi::c_char)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char,
                    FORWARD,
                    PTEND,
                );
            } else {
                status = scanner(
                    (&raw mut tap as *mut ::core::ffi::c_char)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char,
                    REVERSE,
                    PTBEG,
                );
            }
            if status != TRUE {
                mlwrite(
                    b"No more matches.\0" as *const u8 as *const ::core::ffi::c_char,
                );
                break;
            } else {
                update(TRUE);
                mlwrite(
                    b"Search Next? (y/n or Enter new)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                let mut c: ::core::ffi::c_int = tgetc();
                if c == 'y' as i32 || c == 'Y' as i32 {
                    continue;
                }
                if c == '\r' as i32 || c == '\n' as i32 || c == CONTROL | 'M' as i32 {
                    restart = TRUE;
                    mlwrite(b"\0" as *const u8 as *const ::core::ffi::c_char);
                    break;
                } else {
                    mlwrite(b"\0" as *const u8 as *const ::core::ffi::c_char);
                    break;
                }
            }
        }
        if restart == 0 {
            break;
        }
    }
    nanox_request_underbar_redraw();
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn forwhunt(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut status: ::core::ffi::c_int = TRUE;
    if n < 0 as ::core::ffi::c_int {
        return backhunt(f, -n);
    }
    if *(&raw mut pat as *mut ::core::ffi::c_char)
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32
    {
        mlwrite(b"No pattern set\0" as *const u8 as *const ::core::ffi::c_char);
        return FALSE;
    }
    if (*(*curwp).w_bufp).b_mode & MDMAGIC != 0 as ::core::ffi::c_int
        && (*mcpat.offset(0)).mc_type as ::core::ffi::c_int == MCNIL
    {
        if mcstr() == 0 {
            return FALSE;
        }
    }
    loop {
        if (magical as ::core::ffi::c_int != 0
            && (*(*curwp).w_bufp).b_mode & MDMAGIC != 0) as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            status = mcscanner(
                (&raw mut mcpat as *mut magic).offset(0 as ::core::ffi::c_int as isize)
                    as *mut magic,
                FORWARD,
                PTEND,
            );
        } else {
            status = scanner(
                (&raw mut pat as *mut ::core::ffi::c_char)
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char,
                FORWARD,
                PTEND,
            );
        }
        n -= 1;
        if !(n > 0 as ::core::ffi::c_int && status != 0) {
            break;
        }
    }
    if status == TRUE {
        savematch();
    } else {
        mlwrite(b"Not found\0" as *const u8 as *const ::core::ffi::c_char);
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn backsearch(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut status: ::core::ffi::c_int = TRUE;
    if n < 0 as ::core::ffi::c_int {
        return forwsearch(f, -n);
    }
    status = readpattern(
        b"Reverse search\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        (&raw mut pat as *mut ::core::ffi::c_char)
            .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
        TRUE,
    );
    if status == TRUE {
        loop {
            if (magical as ::core::ffi::c_int != 0
                && (*(*curwp).w_bufp).b_mode & MDMAGIC != 0) as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
            {
                status = mcscanner(
                    (&raw mut tapcm as *mut magic)
                        .offset(0 as ::core::ffi::c_int as isize) as *mut magic,
                    REVERSE,
                    PTBEG,
                );
            } else {
                status = scanner(
                    (&raw mut tap as *mut ::core::ffi::c_char)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char,
                    REVERSE,
                    PTBEG,
                );
            }
            n -= 1;
            if !(n > 0 as ::core::ffi::c_int && status != 0) {
                break;
            }
        }
        if status == TRUE {
            savematch();
        } else {
            mlwrite(b"Not found\0" as *const u8 as *const ::core::ffi::c_char);
        }
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn backhunt(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut status: ::core::ffi::c_int = TRUE;
    if n < 0 as ::core::ffi::c_int {
        return forwhunt(f, -n);
    }
    if *(&raw mut tap as *mut ::core::ffi::c_char)
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32
    {
        mlwrite(b"No pattern set\0" as *const u8 as *const ::core::ffi::c_char);
        return FALSE;
    }
    if (*(*curwp).w_bufp).b_mode & MDMAGIC != 0 as ::core::ffi::c_int
        && (*tapcm.offset(0)).mc_type as ::core::ffi::c_int == MCNIL
    {
        if mcstr() == 0 {
            return FALSE;
        }
    }
    loop {
        if (magical as ::core::ffi::c_int != 0
            && (*(*curwp).w_bufp).b_mode & MDMAGIC != 0) as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            status = mcscanner(
                (&raw mut tapcm as *mut magic).offset(0 as ::core::ffi::c_int as isize)
                    as *mut magic,
                REVERSE,
                PTBEG,
            );
        } else {
            status = scanner(
                (&raw mut tap as *mut ::core::ffi::c_char)
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char,
                REVERSE,
                PTBEG,
            );
        }
        n -= 1;
        if !(n > 0 as ::core::ffi::c_int && status != 0) {
            break;
        }
    }
    if status == TRUE {
        savematch();
    } else {
        mlwrite(b"Not found\0" as *const u8 as *const ::core::ffi::c_char);
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn mcscanner(
    mut mcpatrn: *mut magic,
    mut direct: ::core::ffi::c_int,
    mut beg_or_end: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut curline: *mut line = ::core::ptr::null_mut::<line>();
    let mut curoff: ::core::ffi::c_int = 0;
    beg_or_end ^= direct;
    mlenold = matchlen;
    curline = (*curwp).w_dotp;
    curoff = (*curwp).w_doto;
    while boundry(curline, curoff, direct) == 0 {
        matchline = curline;
        matchoff = curoff;
        matchlen = 0 as ::core::ffi::c_uint;
        if amatch(mcpatrn, direct, &raw mut curline, &raw mut curoff) != 0 {
            if beg_or_end == PTEND {
                (*curwp).w_dotp = curline;
                (*curwp).w_doto = curoff;
            } else {
                (*curwp).w_dotp = matchline;
                (*curwp).w_doto = matchoff;
            }
            (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | WFMOVE)
                as ::core::ffi::c_char;
            return TRUE;
        }
        nextch(&raw mut curline, &raw mut curoff, direct);
    }
    return FALSE;
}
unsafe extern "C" fn amatch(
    mut mcptr: *mut magic,
    mut direct: ::core::ffi::c_int,
    mut pcwline: *mut *mut line,
    mut pcwoff: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = 0;
    let mut curline: *mut line = ::core::ptr::null_mut::<line>();
    let mut curoff: ::core::ffi::c_int = 0;
    let mut nchars: ::core::ffi::c_int = 0;
    curline = *pcwline;
    curoff = *pcwoff;
    if (*mcptr).mc_type as ::core::ffi::c_int == BOL {
        if curoff != 0 as ::core::ffi::c_int {
            return FALSE;
        }
        mcptr = mcptr.offset(1);
    }
    if (*mcptr).mc_type as ::core::ffi::c_int == EOL {
        if curoff != (*curline).l_used {
            return FALSE;
        }
        mcptr = mcptr.offset(1);
    }
    's_53: while (*mcptr).mc_type as ::core::ffi::c_int != MCNIL {
        c = nextch(&raw mut curline, &raw mut curoff, direct);
        if (*mcptr).mc_type as ::core::ffi::c_int & CLOSURE != 0 {
            nchars = 0 as ::core::ffi::c_int;
            while c != '\n' as i32 && mceq(c, mcptr) != 0 {
                c = nextch(&raw mut curline, &raw mut curoff, direct);
                nchars += 1;
            }
            mcptr = mcptr.offset(1);
            loop {
                c = nextch(&raw mut curline, &raw mut curoff, direct ^ REVERSE);
                if amatch(mcptr, direct, &raw mut curline, &raw mut curoff) != 0 {
                    matchlen = matchlen.wrapping_add(nchars as ::core::ffi::c_uint);
                    break 's_53;
                } else {
                    let fresh2 = nchars;
                    nchars = nchars - 1;
                    if fresh2 == 0 as ::core::ffi::c_int {
                        return FALSE;
                    }
                }
            }
        } else if (*mcptr).mc_type as ::core::ffi::c_int == BOL {
            if curoff == (*curline).l_used {
                c = nextch(&raw mut curline, &raw mut curoff, direct ^ REVERSE);
                break;
            } else {
                return FALSE
            }
        } else if (*mcptr).mc_type as ::core::ffi::c_int == EOL {
            if curoff == 0 as ::core::ffi::c_int {
                c = nextch(&raw mut curline, &raw mut curoff, direct ^ REVERSE);
                break;
            } else {
                return FALSE
            }
        } else {
            if mceq(c, mcptr) == 0 {
                return FALSE;
            }
            if (*mcptr).mc_type as ::core::ffi::c_int & MASKCL == ANY {
                if direct == FORWARD {
                    if c & 0xc0 as ::core::ffi::c_int == 0xc0 as ::core::ffi::c_int {
                        let mut more: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        if c & 0xe0 as ::core::ffi::c_int == 0xc0 as ::core::ffi::c_int {
                            more = 1 as ::core::ffi::c_int;
                        } else if c & 0xf0 as ::core::ffi::c_int
                            == 0xe0 as ::core::ffi::c_int
                        {
                            more = 2 as ::core::ffi::c_int;
                        } else if c & 0xf8 as ::core::ffi::c_int
                            == 0xf0 as ::core::ffi::c_int
                        {
                            more = 3 as ::core::ffi::c_int;
                        }
                        while more > 0 as ::core::ffi::c_int {
                            let mut saved_line: *mut line = curline;
                            let mut saved_off: ::core::ffi::c_int = curoff;
                            let mut next: ::core::ffi::c_int = nextch(
                                &raw mut curline,
                                &raw mut curoff,
                                direct,
                            );
                            if next & 0xc0 as ::core::ffi::c_int
                                != 0x80 as ::core::ffi::c_int
                            {
                                curline = saved_line;
                                curoff = saved_off;
                                break;
                            } else {
                                matchlen = matchlen.wrapping_add(1);
                                more -= 1;
                            }
                        }
                    }
                } else if c & 0xc0 as ::core::ffi::c_int == 0x80 as ::core::ffi::c_int {
                    loop {
                        let mut prev: ::core::ffi::c_int = nextch(
                            &raw mut curline,
                            &raw mut curoff,
                            direct,
                        );
                        matchlen = matchlen.wrapping_add(1);
                        if is_beginning_utf8(prev as ::core::ffi::c_uchar) != 0 {
                            break;
                        }
                        if prev == '\n' as i32 {
                            break;
                        }
                    }
                }
            }
            matchlen = matchlen.wrapping_add(1);
            mcptr = mcptr.offset(1);
        }
    }
    *pcwline = curline;
    *pcwoff = curoff;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn scanner(
    mut patrn: *const ::core::ffi::c_char,
    mut direct: ::core::ffi::c_int,
    mut beg_or_end: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = 0;
    let mut patptr: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut curline: *mut line = ::core::ptr::null_mut::<line>();
    let mut curoff: ::core::ffi::c_int = 0;
    let mut scanline: *mut line = ::core::ptr::null_mut::<line>();
    let mut scanoff: ::core::ffi::c_int = 0;
    beg_or_end ^= direct;
    curline = (*curwp).w_dotp;
    curoff = (*curwp).w_doto;
    while boundry(curline, curoff, direct) == 0 {
        let mut current_block_19: u64;
        matchline = curline;
        matchoff = curoff;
        c = nextch(&raw mut curline, &raw mut curoff, direct);
        if eq(
            c as ::core::ffi::c_uchar,
            *patrn.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar,
        ) != 0
        {
            scanline = curline;
            scanoff = curoff;
            patptr = patrn.offset(0 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char;
            loop {
                patptr = patptr.offset(1);
                if !(*patptr as ::core::ffi::c_int != '\0' as i32) {
                    current_block_19 = 5399440093318478209;
                    break;
                }
                c = nextch(&raw mut scanline, &raw mut scanoff, direct);
                if eq(c as ::core::ffi::c_uchar, *patptr as ::core::ffi::c_uchar) == 0 {
                    current_block_19 = 4808432441040389987;
                    break;
                }
            }
            match current_block_19 {
                4808432441040389987 => {}
                _ => {
                    if beg_or_end == PTEND {
                        (*curwp).w_dotp = scanline;
                        (*curwp).w_doto = scanoff;
                    } else {
                        (*curwp).w_dotp = matchline;
                        (*curwp).w_doto = matchoff;
                    }
                    (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | WFMOVE)
                        as ::core::ffi::c_char;
                    return TRUE;
                }
            }
        }
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn eq(
    mut bc: ::core::ffi::c_uchar,
    mut pc: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    if (*(*curwp).w_bufp).b_mode & MDEXACT == 0 as ::core::ffi::c_int {
        if 'a' as i32 <= 0xff as ::core::ffi::c_int & bc as ::core::ffi::c_int
            && 'z' as i32 >= 0xff as ::core::ffi::c_int & bc as ::core::ffi::c_int
        {
            bc = (bc as ::core::ffi::c_int ^ DIFCASE) as ::core::ffi::c_uchar;
        }
        if 'a' as i32 <= 0xff as ::core::ffi::c_int & pc as ::core::ffi::c_int
            && 'z' as i32 >= 0xff as ::core::ffi::c_int & pc as ::core::ffi::c_int
        {
            pc = (pc as ::core::ffi::c_int ^ DIFCASE) as ::core::ffi::c_uchar;
        }
    }
    return (bc as ::core::ffi::c_int == pc as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn readpattern(
    mut prompt: *mut ::core::ffi::c_char,
    mut apat: *mut ::core::ffi::c_char,
    mut srch: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut status: ::core::ffi::c_int = 0;
    let mut tpat: [::core::ffi::c_char; 1044] = [0; 1044];
    strcpy(&raw mut tpat as *mut ::core::ffi::c_char, prompt);
    strcat(
        &raw mut tpat as *mut ::core::ffi::c_char,
        b" (\0" as *const u8 as *const ::core::ffi::c_char,
    );
    expandp(
        apat.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
        (&raw mut tpat as *mut ::core::ffi::c_char)
            .offset(
                (strlen
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                    ) -> size_t)(&raw mut tpat as *mut ::core::ffi::c_char) as isize,
            ) as *mut ::core::ffi::c_char,
        NPAT / 2 as ::core::ffi::c_int,
    );
    strcat(
        &raw mut tpat as *mut ::core::ffi::c_char,
        b")<Meta>: \0" as *const u8 as *const ::core::ffi::c_char,
    );
    status = mlreplyt(
        &raw mut tpat as *mut ::core::ffi::c_char,
        &raw mut tpat as *mut ::core::ffi::c_char,
        NPAT,
        metac,
    );
    if status == TRUE {
        strcpy(apat, &raw mut tpat as *mut ::core::ffi::c_char);
        if srch != 0 {
            rvstrscpy(&raw mut tap as *mut ::core::ffi::c_char, apat, NPAT);
            matchlen = strlen(apat) as ::core::ffi::c_uint;
            mlenold = matchlen;
        }
        if (*(*curwp).w_bufp).b_mode & MDMAGIC == 0 as ::core::ffi::c_int {
            mcclear();
        } else {
            status = if srch != 0 { mcstr() } else { TRUE };
        }
    } else if status == FALSE
        && *apat.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
    {
        status = TRUE;
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn savematch() {
    let mut ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut j: ::core::ffi::c_int = 0;
    let mut curline: *mut line = ::core::ptr::null_mut::<line>();
    let mut curoff: ::core::ffi::c_int = 0;
    if !patmatch.is_null() {
        free(patmatch as *mut ::core::ffi::c_void);
    }
    patmatch = malloc(matchlen.wrapping_add(1 as ::core::ffi::c_uint) as size_t)
        as *mut ::core::ffi::c_char;
    ptr = patmatch;
    if !ptr.is_null() {
        curoff = matchoff;
        curline = matchline;
        j = 0 as ::core::ffi::c_int;
        while (j as ::core::ffi::c_uint) < matchlen {
            let fresh0 = ptr;
            ptr = ptr.offset(1);
            *fresh0 = nextch(&raw mut curline, &raw mut curoff, FORWARD)
                as ::core::ffi::c_char;
            j += 1;
        }
        *ptr = '\0' as i32 as ::core::ffi::c_char;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rvstrscpy(
    mut rvstr: *mut ::core::ffi::c_char,
    mut str: *mut ::core::ffi::c_char,
    mut size: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_int = strlen(str) as ::core::ffi::c_int;
    if size <= 0 as ::core::ffi::c_int {
        return;
    }
    if len >= size {
        len = size - 1 as ::core::ffi::c_int;
    }
    str = str.offset(len as isize);
    i = 0 as ::core::ffi::c_int;
    while i < len {
        str = str.offset(-1);
        let fresh6 = rvstr;
        rvstr = rvstr.offset(1);
        *fresh6 = *str;
        i += 1;
    }
    *rvstr = '\0' as i32 as ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn expandp(
    mut srcstr: *mut ::core::ffi::c_char,
    mut deststr: *mut ::core::ffi::c_char,
    mut maxlength: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_uchar = 0;
    loop {
        let fresh7 = srcstr;
        srcstr = srcstr.offset(1);
        c = *fresh7 as ::core::ffi::c_uchar;
        if !(c as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
            break;
        }
        if c as ::core::ffi::c_int == '\n' as i32 {
            let fresh8 = deststr;
            deststr = deststr.offset(1);
            *fresh8 = '<' as i32 as ::core::ffi::c_char;
            let fresh9 = deststr;
            deststr = deststr.offset(1);
            *fresh9 = 'N' as i32 as ::core::ffi::c_char;
            let fresh10 = deststr;
            deststr = deststr.offset(1);
            *fresh10 = 'L' as i32 as ::core::ffi::c_char;
            let fresh11 = deststr;
            deststr = deststr.offset(1);
            *fresh11 = '>' as i32 as ::core::ffi::c_char;
            maxlength -= 4 as ::core::ffi::c_int;
        } else if c as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            && (c as ::core::ffi::c_int) < 0x20 as ::core::ffi::c_int
            || c as ::core::ffi::c_int == 0x7f as ::core::ffi::c_int
        {
            let fresh12 = deststr;
            deststr = deststr.offset(1);
            *fresh12 = '^' as i32 as ::core::ffi::c_char;
            let fresh13 = deststr;
            deststr = deststr.offset(1);
            *fresh13 = (c as ::core::ffi::c_int ^ 0x40 as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            maxlength -= 2 as ::core::ffi::c_int;
        } else if c as ::core::ffi::c_int == '%' as i32 {
            let fresh14 = deststr;
            deststr = deststr.offset(1);
            *fresh14 = '%' as i32 as ::core::ffi::c_char;
            let fresh15 = deststr;
            deststr = deststr.offset(1);
            *fresh15 = '%' as i32 as ::core::ffi::c_char;
            maxlength -= 2 as ::core::ffi::c_int;
        } else {
            let fresh16 = deststr;
            deststr = deststr.offset(1);
            *fresh16 = c as ::core::ffi::c_char;
            maxlength -= 1;
        }
        if maxlength < 4 as ::core::ffi::c_int {
            let fresh17 = deststr;
            deststr = deststr.offset(1);
            *fresh17 = '$' as i32 as ::core::ffi::c_char;
            *deststr = '\0' as i32 as ::core::ffi::c_char;
            return FALSE;
        }
    }
    *deststr = '\0' as i32 as ::core::ffi::c_char;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn boundry(
    mut curline: *mut line,
    mut curoff: ::core::ffi::c_int,
    mut dir: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut border: ::core::ffi::c_int = 0;
    if dir == FORWARD {
        border = (curoff == (*curline).l_used && (*curline).l_fp == (*curbp).b_linep)
            as ::core::ffi::c_int;
    } else {
        border = (curoff == 0 as ::core::ffi::c_int
            && (*curline).l_bp == (*curbp).b_linep) as ::core::ffi::c_int;
    }
    return border;
}
unsafe extern "C" fn nextch(
    mut pcurline: *mut *mut line,
    mut pcuroff: *mut ::core::ffi::c_int,
    mut dir: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut curline: *mut line = ::core::ptr::null_mut::<line>();
    let mut curoff: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    curline = *pcurline;
    curoff = *pcuroff;
    if dir == FORWARD {
        if curoff == (*curline).l_used {
            curline = (*curline).l_fp;
            curoff = 0 as ::core::ffi::c_int;
            c = '\n' as i32;
        } else {
            let fresh1 = curoff;
            curoff = curoff + 1;
            c = *(&raw mut (*curline).l_text as *mut ::core::ffi::c_uchar)
                .offset(fresh1 as isize) as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_int;
        }
    } else if curoff == 0 as ::core::ffi::c_int {
        curline = (*curline).l_bp;
        curoff = (*curline).l_used;
        c = '\n' as i32;
    } else {
        curoff -= 1;
        c = *(&raw mut (*curline).l_text as *mut ::core::ffi::c_uchar)
            .offset(curoff as isize) as ::core::ffi::c_int & 0xff as ::core::ffi::c_int;
    }
    *pcurline = curline;
    *pcuroff = curoff;
    return c;
}
unsafe extern "C" fn mcstr() -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut mcptr: *mut magic = ::core::ptr::null_mut::<magic>();
    let mut rtpcm: *mut magic = ::core::ptr::null_mut::<magic>();
    let mut patptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut mj: ::core::ffi::c_int = 0;
    let mut pchr: ::core::ffi::c_int = 0;
    let mut status: ::core::ffi::c_int = TRUE;
    let mut does_closure: ::core::ffi::c_int = FALSE;
    if magical != 0 {
        mcclear();
    }
    magical = FALSE as ::core::ffi::c_short;
    mj = 0 as ::core::ffi::c_int;
    mcptr = (&raw mut mcpat as *mut magic).offset(0 as ::core::ffi::c_int as isize)
        as *mut magic;
    patptr = (&raw mut pat as *mut ::core::ffi::c_char)
        .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char;
    loop {
        pchr = *patptr as ::core::ffi::c_int;
        if !(pchr != 0 && status != 0) {
            break;
        }
        match pchr {
            MC_CCL => {
                status = cclmake(&raw mut patptr, mcptr);
                magical = TRUE as ::core::ffi::c_short;
                does_closure = TRUE;
                current_block = 15897653523371991391;
            }
            MC_BOL => {
                if mj != 0 as ::core::ffi::c_int {
                    current_block = 1601526752931334950;
                } else {
                    (*mcptr).mc_type = BOL as ::core::ffi::c_short;
                    magical = TRUE as ::core::ffi::c_short;
                    does_closure = FALSE;
                    current_block = 15897653523371991391;
                }
            }
            MC_EOL => {
                if *patptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != '\0' as i32
                {
                    current_block = 1601526752931334950;
                } else {
                    (*mcptr).mc_type = EOL as ::core::ffi::c_short;
                    magical = TRUE as ::core::ffi::c_short;
                    does_closure = FALSE;
                    current_block = 15897653523371991391;
                }
            }
            MC_ANY => {
                (*mcptr).mc_type = ANY as ::core::ffi::c_short;
                magical = TRUE as ::core::ffi::c_short;
                does_closure = TRUE;
                current_block = 15897653523371991391;
            }
            MC_CLOSURE => {
                if does_closure == 0 {
                    current_block = 1601526752931334950;
                } else {
                    mj -= 1;
                    mcptr = mcptr.offset(-1);
                    (*mcptr).mc_type = ((*mcptr).mc_type as ::core::ffi::c_int | CLOSURE)
                        as ::core::ffi::c_short;
                    magical = TRUE as ::core::ffi::c_short;
                    does_closure = FALSE;
                    current_block = 15897653523371991391;
                }
            }
            MC_ESC => {
                if *patptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != '\0' as i32
                {
                    patptr = patptr.offset(1);
                    pchr = *patptr as ::core::ffi::c_int;
                    magical = TRUE as ::core::ffi::c_short;
                }
                current_block = 1601526752931334950;
            }
            _ => {
                current_block = 1601526752931334950;
            }
        }
        match current_block {
            1601526752931334950 => {
                (*mcptr).mc_type = LITCHAR as ::core::ffi::c_short;
                (*mcptr).u.lchar = pchr;
                does_closure = (pchr != '\n' as i32) as ::core::ffi::c_int;
            }
            _ => {}
        }
        mcptr = mcptr.offset(1);
        patptr = patptr.offset(1);
        mj += 1;
    }
    (*mcptr).mc_type = MCNIL as ::core::ffi::c_short;
    if status != 0 {
        rtpcm = (&raw mut tapcm as *mut magic).offset(0 as ::core::ffi::c_int as isize)
            as *mut magic;
        loop {
            mj -= 1;
            if !(mj >= 0 as ::core::ffi::c_int) {
                break;
            }
            mcptr = mcptr.offset(-1);
            let fresh3 = rtpcm;
            rtpcm = rtpcm.offset(1);
            *fresh3 = *mcptr;
        }
        (*rtpcm).mc_type = MCNIL as ::core::ffi::c_short;
    } else {
        mcptr = mcptr.offset(-1);
        (*mcptr).mc_type = MCNIL as ::core::ffi::c_short;
        mcclear();
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn mcclear() {
    let mut mcptr: *mut magic = ::core::ptr::null_mut::<magic>();
    mcptr = (&raw mut mcpat as *mut magic).offset(0 as ::core::ffi::c_int as isize)
        as *mut magic;
    while (*mcptr).mc_type as ::core::ffi::c_int != MCNIL {
        if (*mcptr).mc_type as ::core::ffi::c_int & MASKCL == CCL
            || (*mcptr).mc_type as ::core::ffi::c_int & MASKCL == NCCL
        {
            free((*mcptr).u.cclmap as *mut ::core::ffi::c_void);
        }
        mcptr = mcptr.offset(1);
    }
    (*tapcm.offset(0)).mc_type = MCNIL as ::core::ffi::c_short;
    (*mcpat.offset(0)).mc_type = (*tapcm.offset(0)).mc_type;
}
unsafe extern "C" fn mceq(
    mut bc: ::core::ffi::c_int,
    mut mt: *mut magic,
) -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = 0;
    bc = bc & 0xff as ::core::ffi::c_int;
    match (*mt).mc_type as ::core::ffi::c_int & MASKCL {
        LITCHAR => {
            result = eq(
                bc as ::core::ffi::c_uchar,
                (*mt).u.lchar as ::core::ffi::c_uchar,
            );
        }
        ANY => {
            result = (bc != '\n' as i32) as ::core::ffi::c_int;
        }
        CCL => {
            result = biteq(bc, (*mt).u.cclmap);
            if result == 0 {
                if (*(*curwp).w_bufp).b_mode & MDEXACT == 0 as ::core::ffi::c_int
                    && ('a' as i32 <= 0xff as ::core::ffi::c_int & bc
                        && 'z' as i32 >= 0xff as ::core::ffi::c_int & bc
                        || 'A' as i32 <= 0xff as ::core::ffi::c_int & bc
                            && 'Z' as i32 >= 0xff as ::core::ffi::c_int & bc)
                {
                    result = biteq(bc ^ DIFCASE, (*mt).u.cclmap);
                }
            }
        }
        NCCL => {
            result = (biteq(bc, (*mt).u.cclmap) == 0) as ::core::ffi::c_int;
            if (*(*curwp).w_bufp).b_mode & MDEXACT == 0 as ::core::ffi::c_int
                && ('a' as i32 <= 0xff as ::core::ffi::c_int & bc
                    && 'z' as i32 >= 0xff as ::core::ffi::c_int & bc
                    || 'A' as i32 <= 0xff as ::core::ffi::c_int & bc
                        && 'Z' as i32 >= 0xff as ::core::ffi::c_int & bc)
            {
                result
                    &= (biteq(bc ^ DIFCASE, (*mt).u.cclmap) == 0) as ::core::ffi::c_int;
            }
        }
        _ => {
            mlwrite(
                b"mceq: what is %d?\0" as *const u8 as *const ::core::ffi::c_char,
                (*mt).mc_type as ::core::ffi::c_int,
            );
            result = FALSE;
        }
    }
    return result;
}
unsafe extern "C" fn cclmake(
    mut ppatptr: *mut *mut ::core::ffi::c_char,
    mut mcptr: *mut magic,
) -> ::core::ffi::c_int {
    let mut bmap: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut patptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut pchr: ::core::ffi::c_int = 0;
    let mut ochr: ::core::ffi::c_int = 0;
    bmap = clearbits();
    if bmap.is_null() {
        mlwrite(b"%%Out of memory\0" as *const u8 as *const ::core::ffi::c_char);
        return FALSE;
    }
    (*mcptr).u.cclmap = bmap;
    patptr = *ppatptr;
    patptr = patptr.offset(1);
    if *patptr as ::core::ffi::c_int == MC_NCCL {
        patptr = patptr.offset(1);
        (*mcptr).mc_type = NCCL as ::core::ffi::c_short;
    } else {
        (*mcptr).mc_type = CCL as ::core::ffi::c_short;
    }
    ochr = *patptr as ::core::ffi::c_int;
    if ochr == MC_ECCL {
        mlwrite(
            b"%%No characters in character class\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return FALSE;
    } else {
        if ochr == MC_ESC {
            patptr = patptr.offset(1);
            ochr = *patptr as ::core::ffi::c_int;
        }
        setbit(ochr, bmap);
        patptr = patptr.offset(1);
    }
    while ochr != '\0' as i32
        && {
            pchr = *patptr as ::core::ffi::c_int;
            pchr != MC_ECCL
        }
    {
        let mut current_block_27: u64;
        match pchr {
            MC_RCCL => {
                if *patptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == MC_ECCL
                {
                    setbit(pchr, bmap);
                } else {
                    patptr = patptr.offset(1);
                    pchr = *patptr as ::core::ffi::c_int;
                    loop {
                        ochr += 1;
                        if !(ochr <= pchr) {
                            break;
                        }
                        setbit(ochr, bmap);
                    }
                }
                current_block_27 = 2719512138335094285;
            }
            MC_ESC => {
                patptr = patptr.offset(1);
                pchr = *patptr as ::core::ffi::c_int;
                current_block_27 = 7081034042156870609;
            }
            _ => {
                current_block_27 = 7081034042156870609;
            }
        }
        match current_block_27 {
            7081034042156870609 => {
                setbit(pchr, bmap);
            }
            _ => {}
        }
        patptr = patptr.offset(1);
        ochr = pchr;
    }
    *ppatptr = patptr;
    if ochr == '\0' as i32 {
        mlwrite(
            b"%%Character class not ended\0" as *const u8 as *const ::core::ffi::c_char,
        );
        free(bmap as *mut ::core::ffi::c_void);
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn biteq(
    mut bc: ::core::ffi::c_int,
    mut cclmap: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    bc = bc & 0xff as ::core::ffi::c_int;
    if bc >= HICHAR {
        return FALSE;
    }
    return if *cclmap.offset((bc >> 3 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << (bc & 7 as ::core::ffi::c_int) != 0
    {
        TRUE
    } else {
        FALSE
    };
}
unsafe extern "C" fn clearbits() -> *mut ::core::ffi::c_char {
    let mut cclstart: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut cclmap: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut i: ::core::ffi::c_int = 0;
    cclstart = malloc((HICHAR >> 3 as ::core::ffi::c_int) as size_t)
        as *mut ::core::ffi::c_char;
    cclmap = cclstart;
    if !cclmap.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < HICHAR >> 3 as ::core::ffi::c_int {
            let fresh5 = cclmap;
            cclmap = cclmap.offset(1);
            *fresh5 = 0 as ::core::ffi::c_char;
            i += 1;
        }
    }
    return cclstart;
}
unsafe extern "C" fn setbit(
    mut bc: ::core::ffi::c_int,
    mut cclmap: *mut ::core::ffi::c_char,
) {
    bc = bc & 0xff as ::core::ffi::c_int;
    if bc < HICHAR {
        let ref mut fresh4 = *cclmap.offset((bc >> 3 as ::core::ffi::c_int) as isize);
        *fresh4 = (*fresh4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << (bc & 7 as ::core::ffi::c_int))
            as ::core::ffi::c_char;
    }
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
