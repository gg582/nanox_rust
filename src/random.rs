extern "C" {
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
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    static mut tab_width: ::core::ffi::c_int;
    fn vttbeep();
    static mut fillcol: ::core::ffi::c_int;
    static mut modename: [*mut ::core::ffi::c_char; 0];
    static mut thisflag: ::core::ffi::c_int;
    static mut lastflag: ::core::ffi::c_int;
    static mut curwp: *mut window;
    static mut curbp: *mut buffer;
    static mut gmode: ::core::ffi::c_int;
    static mut metac: ::core::ffi::c_int;
    static mut indent_start_lp: *mut line;
    static mut indent_end_lp: *mut line;
    static mut indent_range_type: ::core::ffi::c_int;
    static mut indent_selection_active: ::core::ffi::c_int;
    static mut nanox_cfg: nanox_config;
    fn backchar(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn forwchar(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn forwline(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn setmark(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn execute(
        c: ::core::ffi::c_int,
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn rdonly() -> ::core::ffi::c_int;
    fn update(force: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn upmode();
    fn mlerase();
    fn mlwrite(fmt: *const ::core::ffi::c_char, ...);
    fn mlforce(s: *mut ::core::ffi::c_char);
    fn ttpause();
    fn mlreplyt(
        prompt: *mut ::core::ffi::c_char,
        buf: *mut ::core::ffi::c_char,
        nbuf: ::core::ffi::c_int,
        eolchar: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn tgetc() -> ::core::ffi::c_int;
    fn getcmd() -> ::core::ffi::c_int;
    fn boundry(
        curline: *mut line,
        curoff: ::core::ffi::c_int,
        dir: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn minibuf_input(
        prompt: *const ::core::ffi::c_char,
        dest: *mut ::core::ffi::c_char,
        max_len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn lchange(flag: ::core::ffi::c_int);
    fn insspace(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn linstr(instr: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn linsert(n: ::core::ffi::c_int, c: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn lover(ostr: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn lnewline() -> ::core::ffi::c_int;
    fn ldelete(n: ::core::ffi::c_long, kflag: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn ldelchar(n: ::core::ffi::c_long, kflag: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn kdelete();
    fn completion_try_at_cursor() -> ::core::ffi::c_int;
    fn utf8_to_unicode(
        line: *mut ::core::ffi::c_uchar,
        index: ::core::ffi::c_uint,
        len: ::core::ffi::c_uint,
        res: *mut unicode_t,
    ) -> ::core::ffi::c_uint;
    fn unicode_width(c: unicode_t) -> ::core::ffi::c_int;
}
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
pub struct nanox_config {
    pub hint_bar: bool,
    pub warning_lamp: bool,
    pub warning_format: [::core::ffi::c_char; 8],
    pub error_format: [::core::ffi::c_char; 8],
    pub help_key: ::core::ffi::c_int,
    pub help_language: [::core::ffi::c_char; 8],
    pub soft_tab: bool,
    pub soft_tab_width: ::core::ffi::c_int,
    pub case_sensitive_default: bool,
    pub nonr: bool,
    pub no_function_slot: bool,
}
pub type unicode_t = ::core::ffi::c_uint;
pub const NPAT: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const META: ::core::ffi::c_int = 0x20000000 as ::core::ffi::c_int;
pub const SPEC: ::core::ffi::c_uint = 0x80000000 as ::core::ffi::c_uint;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FORWARD: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const REVERSE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CFCPCN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const CFKILL: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const WFMOVE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const WFEDIT: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const WFHARD: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const BFMAKE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const NUMMODES: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const MDWRAP: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const MDCMOD: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const MDVIEW: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
#[no_mangle]
pub static mut tabsize: ::core::ffi::c_int = 0;
unsafe extern "C" fn get_indent(mut lp: *mut line) -> ::core::ffi::c_int {
    let mut nicol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*lp).l_used {
        c = *(&raw mut (*lp).l_text as *mut ::core::ffi::c_uchar).offset(i as isize)
            as ::core::ffi::c_int & 0xff as ::core::ffi::c_int;
        if c != ' ' as i32 && c != '\t' as i32 {
            break;
        }
        if c == '\t' as i32 {
            nicol = (nicol / tab_width + 1 as ::core::ffi::c_int) * tab_width;
        } else {
            nicol += 1;
        }
        i += 1;
    }
    return nicol;
}
unsafe extern "C" fn set_indent(mut target: ::core::ffi::c_int) {
    let mut ch: ::core::ffi::c_int = 0;
    let mut cur: ::core::ffi::c_int = get_indent((*curwp).w_dotp);
    if cur == target {
        (*curwp).w_doto = 0 as ::core::ffi::c_int;
        while (*curwp).w_doto < (*(*curwp).w_dotp).l_used {
            ch = *(&raw mut (*(*curwp).w_dotp).l_text as *mut ::core::ffi::c_uchar)
                .offset((*curwp).w_doto as isize) as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_int;
            if ch != ' ' as i32 && ch != '\t' as i32 {
                break;
            }
            (*curwp).w_doto += 1;
        }
        return;
    }
    (*curwp).w_doto = 0 as ::core::ffi::c_int;
    while (*curwp).w_doto < (*(*curwp).w_dotp).l_used {
        ch = *(&raw mut (*(*curwp).w_dotp).l_text as *mut ::core::ffi::c_uchar)
            .offset((*curwp).w_doto as isize) as ::core::ffi::c_int
            & 0xff as ::core::ffi::c_int;
        if ch != ' ' as i32 && ch != '\t' as i32 {
            break;
        }
        ldelchar(1 as ::core::ffi::c_long, FALSE);
    }
    if target > 0 as ::core::ffi::c_int {
        if nanox_cfg.soft_tab {
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while i < target {
                linsert(1 as ::core::ffi::c_int, ' ' as i32);
                i += 1;
            }
        } else {
            let mut step: ::core::ffi::c_int = tab_width;
            if step == 0 as ::core::ffi::c_int {
                step = 1 as ::core::ffi::c_int;
            }
            let mut num_tabs: ::core::ffi::c_int = target / step;
            let mut num_spaces: ::core::ffi::c_int = target % step;
            loop {
                let fresh0 = num_tabs;
                num_tabs = num_tabs - 1;
                if !(fresh0 != 0) {
                    break;
                }
                linsert(1 as ::core::ffi::c_int, '\t' as i32);
            }
            loop {
                let fresh1 = num_spaces;
                num_spaces = num_spaces - 1;
                if !(fresh1 != 0) {
                    break;
                }
                linsert(1 as ::core::ffi::c_int, ' ' as i32);
            }
        }
    }
    (*curwp).w_doto = 0 as ::core::ffi::c_int;
    while (*curwp).w_doto < (*(*curwp).w_dotp).l_used {
        let mut ch2: ::core::ffi::c_int = *(&raw mut (*(*curwp).w_dotp).l_text
            as *mut ::core::ffi::c_uchar)
            .offset((*curwp).w_doto as isize) as ::core::ffi::c_int
            & 0xff as ::core::ffi::c_int;
        if ch2 != ' ' as i32 && ch2 != '\t' as i32 {
            break;
        }
        (*curwp).w_doto += 1;
    }
}
unsafe extern "C" fn is_closing_block(mut lp: *mut line) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    let mut buffer: [::core::ffi::c_char; 32] = [0; 32];
    let mut buf_idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ext: *mut ::core::ffi::c_char = strrchr(
        &raw mut (*curbp).b_fname as *mut ::core::ffi::c_char,
        '.' as i32,
    );
    len = (*lp).l_used;
    i = 0 as ::core::ffi::c_int;
    while i < len {
        c = *(&raw mut (*lp).l_text as *mut ::core::ffi::c_uchar).offset(i as isize)
            as ::core::ffi::c_int & 0xff as ::core::ffi::c_int;
        if c != ' ' as i32 && c != '\t' as i32 {
            break;
        }
        i += 1;
    }
    if i == len {
        return FALSE;
    }
    c = *(&raw mut (*lp).l_text as *mut ::core::ffi::c_uchar).offset(i as isize)
        as ::core::ffi::c_int & 0xff as ::core::ffi::c_int;
    if c == '}' as i32 || c == ')' as i32 || c == ']' as i32 {
        return TRUE;
    }
    while i < len && buf_idx < 31 as ::core::ffi::c_int {
        c = *(&raw mut (*lp).l_text as *mut ::core::ffi::c_uchar).offset(i as isize)
            as ::core::ffi::c_int & 0xff as ::core::ffi::c_int;
        if c == ' ' as i32 || c == '\t' as i32 || c == '\n' as i32 || c == '(' as i32
            || c == '{' as i32 || c == '[' as i32 || c == ')' as i32 || c == '}' as i32
            || c == ']' as i32
        {
            break;
        }
        let fresh8 = buf_idx;
        buf_idx = buf_idx + 1;
        buffer[fresh8 as usize] = c as ::core::ffi::c_char;
        i += 1;
    }
    buffer[buf_idx as usize] = '\0' as i32 as ::core::ffi::c_char;
    if ext.is_null() {
        return FALSE;
    }
    if strcasecmp(ext, b".sh\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
        || strcasecmp(ext, b".bash\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        if strcmp(
            &raw mut buffer as *mut ::core::ffi::c_char,
            b"fi\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return TRUE;
        }
        if strcmp(
            &raw mut buffer as *mut ::core::ffi::c_char,
            b"done\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return TRUE;
        }
        if strcmp(
            &raw mut buffer as *mut ::core::ffi::c_char,
            b"esac\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return TRUE;
        }
        if strcmp(
            &raw mut buffer as *mut ::core::ffi::c_char,
            b"else\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return TRUE;
        }
        if strcmp(
            &raw mut buffer as *mut ::core::ffi::c_char,
            b"elif\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return TRUE;
        }
    } else if strcasecmp(ext, b".py\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        if strcmp(
            &raw mut buffer as *mut ::core::ffi::c_char,
            b"else\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return TRUE;
        }
        if strcmp(
            &raw mut buffer as *mut ::core::ffi::c_char,
            b"elif\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return TRUE;
        }
        if strcmp(
            &raw mut buffer as *mut ::core::ffi::c_char,
            b"except\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return TRUE;
        }
        if strcmp(
            &raw mut buffer as *mut ::core::ffi::c_char,
            b"finally\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return TRUE;
        }
    } else if strcasecmp(ext, b".lua\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        if strcmp(
            &raw mut buffer as *mut ::core::ffi::c_char,
            b"end\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return TRUE;
        }
        if strcmp(
            &raw mut buffer as *mut ::core::ffi::c_char,
            b"else\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return TRUE;
        }
        if strcmp(
            &raw mut buffer as *mut ::core::ffi::c_char,
            b"elseif\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return TRUE;
        }
        if strcmp(
            &raw mut buffer as *mut ::core::ffi::c_char,
            b"until\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return TRUE;
        }
    }
    if (*curbp).b_mode & MDCMOD != 0 {
        if strcmp(
            &raw mut buffer as *mut ::core::ffi::c_char,
            b"#else\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                &raw mut buffer as *mut ::core::ffi::c_char,
                b"#elif\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcmp(
                &raw mut buffer as *mut ::core::ffi::c_char,
                b"#endif\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            return TRUE;
        }
    }
    return FALSE;
}
unsafe extern "C" fn check_indent_dedent() {
    let mut lp: *mut line = (*curwp).w_dotp;
    let mut target_indent: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut scan_lp: *mut line = ::core::ptr::null_mut::<line>();
    if is_closing_block(lp) == 0 {
        return;
    }
    let mut i: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_int = (*lp).l_used;
    i = 0 as ::core::ffi::c_int;
    while i < len {
        let mut c: ::core::ffi::c_int = *(&raw mut (*lp).l_text
            as *mut ::core::ffi::c_uchar)
            .offset(i as isize) as ::core::ffi::c_int & 0xff as ::core::ffi::c_int;
        if c != ' ' as i32 && c != '\t' as i32 {
            break;
        }
        i += 1;
    }
    let mut first_char: ::core::ffi::c_int = if i < len {
        *(&raw mut (*lp).l_text as *mut ::core::ffi::c_uchar).offset(i as isize)
            as ::core::ffi::c_int & 0xff as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    if (*curbp).b_mode & MDCMOD != 0
        && (first_char == '}' as i32 || first_char == ')' as i32
            || first_char == ']' as i32)
    {
        let mut oldlp: *mut line = (*curwp).w_dotp;
        let mut oldoff: ::core::ffi::c_int = (*curwp).w_doto;
        let mut count: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut oc: ::core::ffi::c_int = 0;
        if first_char == '}' as i32 {
            oc = '{' as i32;
        } else if first_char == ')' as i32 {
            oc = '(' as i32;
        } else {
            oc = '[' as i32;
        }
        (*curwp).w_doto = i;
        while backchar(FALSE, 1 as ::core::ffi::c_int) != 0 {
            let mut ch: ::core::ffi::c_int = 0;
            if (*curwp).w_doto == (*(*curwp).w_dotp).l_used {
                ch = '\n' as i32;
            } else {
                ch = *(&raw mut (*(*curwp).w_dotp).l_text as *mut ::core::ffi::c_uchar)
                    .offset((*curwp).w_doto as isize) as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int;
            }
            if ch == first_char {
                count += 1;
            } else if ch == oc {
                count -= 1;
            }
            if count == 0 as ::core::ffi::c_int {
                target_indent = get_indent((*curwp).w_dotp);
                break;
            } else if boundry((*curwp).w_dotp, (*curwp).w_doto, REVERSE) != 0 {
                break;
            }
        }
        (*curwp).w_dotp = oldlp;
        (*curwp).w_doto = oldoff;
    } else {
        let mut ext: *mut ::core::ffi::c_char = strrchr(
            &raw mut (*curbp).b_fname as *mut ::core::ffi::c_char,
            '.' as i32,
        );
        if !ext.is_null()
            && (strcasecmp(ext, b".sh\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
                || strcasecmp(ext, b".bash\0" as *const u8 as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int)
        {
            let mut word: [::core::ffi::c_char; 32] = [0; 32];
            let mut idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut k: ::core::ffi::c_int = i;
            while k < len && idx < 31 as ::core::ffi::c_int {
                let mut c_0: ::core::ffi::c_int = *(&raw mut (*lp).l_text
                    as *mut ::core::ffi::c_uchar)
                    .offset(k as isize) as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int;
                if *(*__ctype_b_loc()).offset(c_0 as isize) as ::core::ffi::c_int
                    & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                        as ::core::ffi::c_int != 0
                {
                    break;
                }
                let fresh4 = idx;
                idx = idx + 1;
                word[fresh4 as usize] = c_0 as ::core::ffi::c_char;
                k += 1;
            }
            word[idx as usize] = '\0' as i32 as ::core::ffi::c_char;
            if strcmp(
                &raw mut word as *mut ::core::ffi::c_char,
                b"fi\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                || strcmp(
                    &raw mut word as *mut ::core::ffi::c_char,
                    b"done\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                || strcmp(
                    &raw mut word as *mut ::core::ffi::c_char,
                    b"esac\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                || strcmp(
                    &raw mut word as *mut ::core::ffi::c_char,
                    b"else\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                || strcmp(
                    &raw mut word as *mut ::core::ffi::c_char,
                    b"elif\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                let mut oldlp_0: *mut line = (*curwp).w_dotp;
                let mut oldoff_0: ::core::ffi::c_int = (*curwp).w_doto;
                let mut count_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                let mut open_word: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                    ::core::ffi::c_char,
                >();
                let mut close_word: *mut ::core::ffi::c_char = &raw mut word
                    as *mut ::core::ffi::c_char;
                if strcmp(
                    &raw mut word as *mut ::core::ffi::c_char,
                    b"fi\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    open_word = b"if\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                } else if strcmp(
                    &raw mut word as *mut ::core::ffi::c_char,
                    b"done\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    open_word = b"for\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                } else if strcmp(
                    &raw mut word as *mut ::core::ffi::c_char,
                    b"esac\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    open_word = b"case\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                } else if strcmp(
                    &raw mut word as *mut ::core::ffi::c_char,
                    b"else\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                    || strcmp(
                        &raw mut word as *mut ::core::ffi::c_char,
                        b"elif\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                {
                    open_word = b"if\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char;
                    count_0 = 1 as ::core::ffi::c_int;
                }
                let mut scan: *mut line = (*lp).l_bp;
                while scan != (*curbp).b_linep {
                    let mut slen: ::core::ffi::c_int = (*scan).l_used;
                    let mut si: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while si < slen
                        && *(*__ctype_b_loc())
                            .offset(
                                (*(&raw mut (*scan).l_text as *mut ::core::ffi::c_uchar)
                                    .offset(si as isize) as ::core::ffi::c_int
                                    & 0xff as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_int
                            & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                                as ::core::ffi::c_int != 0
                    {
                        si += 1;
                    }
                    if si < slen {
                        let mut sword: [::core::ffi::c_char; 32] = [0; 32];
                        let mut sidx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        let mut k_0: ::core::ffi::c_int = si;
                        while k_0 < slen && sidx < 31 as ::core::ffi::c_int {
                            let mut sc: ::core::ffi::c_int = *(&raw mut (*scan).l_text
                                as *mut ::core::ffi::c_uchar)
                                .offset(k_0 as isize) as ::core::ffi::c_int
                                & 0xff as ::core::ffi::c_int;
                            if *(*__ctype_b_loc()).offset(sc as isize)
                                as ::core::ffi::c_int
                                & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                                    as ::core::ffi::c_int != 0
                            {
                                break;
                            }
                            let fresh5 = sidx;
                            sidx = sidx + 1;
                            sword[fresh5 as usize] = sc as ::core::ffi::c_char;
                            k_0 += 1;
                        }
                        sword[sidx as usize] = '\0' as i32 as ::core::ffi::c_char;
                        if !open_word.is_null()
                            && strcmp(
                                &raw mut sword as *mut ::core::ffi::c_char,
                                open_word,
                            ) == 0 as ::core::ffi::c_int
                        {
                            count_0 -= 1;
                            if count_0 == 0 as ::core::ffi::c_int {
                                target_indent = get_indent(scan);
                                break;
                            }
                        } else if strcmp(
                            &raw mut sword as *mut ::core::ffi::c_char,
                            close_word,
                        ) == 0 as ::core::ffi::c_int
                        {
                            count_0 += 1;
                        }
                    }
                    scan = (*scan).l_bp;
                }
                (*curwp).w_dotp = oldlp_0;
                (*curwp).w_doto = oldoff_0;
            }
        } else if (*curbp).b_mode & MDCMOD != 0 {
            let mut word_0: [::core::ffi::c_char; 32] = [0; 32];
            let mut idx_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut k_1: ::core::ffi::c_int = i;
            while k_1 < len && idx_0 < 31 as ::core::ffi::c_int {
                let mut c_1: ::core::ffi::c_int = *(&raw mut (*lp).l_text
                    as *mut ::core::ffi::c_uchar)
                    .offset(k_1 as isize) as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int;
                if *(*__ctype_b_loc()).offset(c_1 as isize) as ::core::ffi::c_int
                    & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                        as ::core::ffi::c_int != 0
                {
                    break;
                }
                let fresh6 = idx_0;
                idx_0 = idx_0 + 1;
                word_0[fresh6 as usize] = c_1 as ::core::ffi::c_char;
                k_1 += 1;
            }
            word_0[idx_0 as usize] = '\0' as i32 as ::core::ffi::c_char;
            if strcmp(
                &raw mut word_0 as *mut ::core::ffi::c_char,
                b"#else\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                || strcmp(
                    &raw mut word_0 as *mut ::core::ffi::c_char,
                    b"#elif\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                || strcmp(
                    &raw mut word_0 as *mut ::core::ffi::c_char,
                    b"#endif\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                let mut count_1: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                let mut scan_0: *mut line = (*lp).l_bp;
                while scan_0 != (*curbp).b_linep {
                    let mut slen_0: ::core::ffi::c_int = (*scan_0).l_used;
                    let mut si_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while si_0 < slen_0
                        && *(*__ctype_b_loc())
                            .offset(
                                (*(&raw mut (*scan_0).l_text as *mut ::core::ffi::c_uchar)
                                    .offset(si_0 as isize) as ::core::ffi::c_int
                                    & 0xff as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_int
                            & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                                as ::core::ffi::c_int != 0
                    {
                        si_0 += 1;
                    }
                    if si_0 < slen_0 {
                        let mut sword_0: [::core::ffi::c_char; 32] = [0; 32];
                        let mut sidx_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        let mut k_2: ::core::ffi::c_int = si_0;
                        while k_2 < slen_0 && sidx_0 < 31 as ::core::ffi::c_int {
                            let mut sc_0: ::core::ffi::c_int = *(&raw mut (*scan_0)
                                .l_text as *mut ::core::ffi::c_uchar)
                                .offset(k_2 as isize) as ::core::ffi::c_int
                                & 0xff as ::core::ffi::c_int;
                            if *(*__ctype_b_loc()).offset(sc_0 as isize)
                                as ::core::ffi::c_int
                                & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                                    as ::core::ffi::c_int != 0
                            {
                                break;
                            }
                            let fresh7 = sidx_0;
                            sidx_0 = sidx_0 + 1;
                            sword_0[fresh7 as usize] = sc_0 as ::core::ffi::c_char;
                            k_2 += 1;
                        }
                        sword_0[sidx_0 as usize] = '\0' as i32 as ::core::ffi::c_char;
                        if strcmp(
                            &raw mut sword_0 as *mut ::core::ffi::c_char,
                            b"#if\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                            || strcmp(
                                &raw mut sword_0 as *mut ::core::ffi::c_char,
                                b"#ifdef\0" as *const u8 as *const ::core::ffi::c_char,
                            ) == 0 as ::core::ffi::c_int
                            || strcmp(
                                &raw mut sword_0 as *mut ::core::ffi::c_char,
                                b"#ifndef\0" as *const u8 as *const ::core::ffi::c_char,
                            ) == 0 as ::core::ffi::c_int
                        {
                            count_1 -= 1;
                            if count_1 == 0 as ::core::ffi::c_int {
                                target_indent = get_indent(scan_0);
                                break;
                            }
                        } else if strcmp(
                            &raw mut sword_0 as *mut ::core::ffi::c_char,
                            b"#endif\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            count_1 += 1;
                        }
                    }
                    scan_0 = (*scan_0).l_bp;
                }
            }
        }
    }
    if target_indent < 0 as ::core::ffi::c_int {
        scan_lp = (*lp).l_bp;
        while scan_lp != (*curbp).b_linep {
            let mut is_blank: ::core::ffi::c_int = TRUE;
            let mut k_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while k_3 < (*scan_lp).l_used {
                let mut c_2: ::core::ffi::c_int = *(&raw mut (*scan_lp).l_text
                    as *mut ::core::ffi::c_uchar)
                    .offset(k_3 as isize) as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int;
                if c_2 != ' ' as i32 && c_2 != '\t' as i32 {
                    is_blank = FALSE;
                    break;
                } else {
                    k_3 += 1;
                }
            }
            if is_blank == 0 {
                target_indent = get_indent(scan_lp);
                break;
            } else {
                scan_lp = (*scan_lp).l_bp;
            }
        }
    }
    if target_indent >= 0 as ::core::ffi::c_int {
        let mut cur_indent: ::core::ffi::c_int = get_indent(lp);
        if cur_indent != target_indent {
            let mut old_doto: ::core::ffi::c_int = (*curwp).w_doto;
            set_indent(target_indent);
            if old_doto > (*(*curwp).w_dotp).l_used {
                old_doto = (*(*curwp).w_dotp).l_used;
            }
            if old_doto < 0 as ::core::ffi::c_int {
                old_doto = 0 as ::core::ffi::c_int;
            }
            (*curwp).w_doto = old_doto;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn setfillcol(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    fillcol = n;
    mlwrite(b"(Fill column is %d)\0" as *const u8 as *const ::core::ffi::c_char, n);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn showcpos(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut lp: *mut line = ::core::ptr::null_mut::<line>();
    let mut numchars: ::core::ffi::c_long = 0;
    let mut numlines: ::core::ffi::c_int = 0;
    let mut predchars: ::core::ffi::c_long = 0;
    let mut predlines: ::core::ffi::c_int = 0;
    let mut curchar: ::core::ffi::c_int = 0;
    let mut ratio: ::core::ffi::c_int = 0;
    let mut col: ::core::ffi::c_int = 0;
    let mut savepos: ::core::ffi::c_int = 0;
    let mut ecol: ::core::ffi::c_int = 0;
    lp = (*(*curbp).b_linep).l_fp;
    numchars = 0 as ::core::ffi::c_long;
    numlines = 0 as ::core::ffi::c_int;
    predchars = 0 as ::core::ffi::c_long;
    predlines = 0 as ::core::ffi::c_int;
    curchar = 0 as ::core::ffi::c_int;
    while lp != (*curbp).b_linep {
        if lp == (*curwp).w_dotp {
            predlines = numlines;
            predchars = numchars + (*curwp).w_doto as ::core::ffi::c_long;
            if (*curwp).w_doto == (*lp).l_used {
                curchar = '\n' as i32;
            } else {
                curchar = *(&raw mut (*lp).l_text as *mut ::core::ffi::c_uchar)
                    .offset((*curwp).w_doto as isize) as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int;
            }
        }
        numlines += 1;
        numchars += ((*lp).l_used + 1 as ::core::ffi::c_int) as ::core::ffi::c_long;
        lp = (*lp).l_fp;
    }
    if (*curwp).w_dotp == (*curbp).b_linep {
        predlines = numlines;
        predchars = numchars;
        curchar = 0 as ::core::ffi::c_int;
    }
    col = getccol(FALSE);
    savepos = (*curwp).w_doto;
    (*curwp).w_doto = (*(*curwp).w_dotp).l_used;
    ecol = getccol(FALSE);
    (*curwp).w_doto = savepos;
    ratio = 0 as ::core::ffi::c_int;
    if numchars != 0 as ::core::ffi::c_long {
        ratio = (100 as ::core::ffi::c_long * predchars / numchars)
            as ::core::ffi::c_int;
    }
    mlwrite(
        b"Line %d/%d Col %d/%d Char %D/%D (%d%%) char = 0x%x\0" as *const u8
            as *const ::core::ffi::c_char,
        predlines + 1 as ::core::ffi::c_int,
        numlines + 1 as ::core::ffi::c_int,
        col,
        ecol,
        predchars,
        numchars,
        ratio,
        curchar,
    );
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn getcline() -> ::core::ffi::c_int {
    let mut lp: *mut line = ::core::ptr::null_mut::<line>();
    let mut numlines: ::core::ffi::c_int = 0;
    lp = (*(*curbp).b_linep).l_fp;
    numlines = 0 as ::core::ffi::c_int;
    while lp != (*curbp).b_linep {
        if lp == (*curwp).w_dotp {
            break;
        }
        numlines += 1;
        lp = (*lp).l_fp;
    }
    return numlines + 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getccol(mut bflg: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut col: ::core::ffi::c_int = 0;
    let mut dlp: *mut line = (*curwp).w_dotp;
    let mut byte_offset: ::core::ffi::c_int = (*curwp).w_doto;
    let mut len: ::core::ffi::c_int = (*dlp).l_used;
    i = 0 as ::core::ffi::c_int;
    col = i;
    while i < byte_offset {
        let mut c: unicode_t = 0;
        i = (i as ::core::ffi::c_uint)
            .wrapping_add(
                utf8_to_unicode(
                    &raw mut (*dlp).l_text as *mut ::core::ffi::c_uchar,
                    i as ::core::ffi::c_uint,
                    len as ::core::ffi::c_uint,
                    &raw mut c,
                ),
            ) as ::core::ffi::c_int as ::core::ffi::c_int;
        if c != ' ' as i32 as unicode_t && c != '\t' as i32 as unicode_t && bflg != 0 {
            break;
        }
        col = next_column(col, c, tab_width);
    }
    return col;
}
#[no_mangle]
pub unsafe extern "C" fn setccol(mut pos: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut col: ::core::ffi::c_int = 0;
    let mut llen: ::core::ffi::c_int = 0;
    let mut dlp: *mut line = (*curwp).w_dotp;
    col = 0 as ::core::ffi::c_int;
    llen = (*dlp).l_used;
    i = 0 as ::core::ffi::c_int;
    while i < llen && col < pos {
        let mut c: unicode_t = 0;
        let mut bytes: ::core::ffi::c_int = 0;
        bytes = utf8_to_unicode(
            &raw mut (*dlp).l_text as *mut ::core::ffi::c_uchar,
            i as ::core::ffi::c_uint,
            llen as ::core::ffi::c_uint,
            &raw mut c,
        ) as ::core::ffi::c_int;
        col = next_column(col, c, tab_width);
        i += bytes;
    }
    (*curwp).w_doto = i;
    return (col >= pos) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn twiddle(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut dotp: *mut line = ::core::ptr::null_mut::<line>();
    let mut doto: ::core::ffi::c_int = 0;
    let mut cr: unicode_t = 0;
    let mut cl: unicode_t = 0;
    let mut len_r: ::core::ffi::c_int = 0;
    let mut len_l: ::core::ffi::c_int = 0;
    let mut doto_l: ::core::ffi::c_int = 0;
    let mut doto_r: ::core::ffi::c_int = 0;
    if (*curbp).b_mode & MDVIEW != 0 {
        return rdonly();
    }
    dotp = (*curwp).w_dotp;
    doto = (*curwp).w_doto;
    if doto == (*dotp).l_used {
        if doto == 0 as ::core::ffi::c_int {
            return FALSE;
        }
        loop {
            doto -= 1;
            if !(doto > 0 as ::core::ffi::c_int
                && is_beginning_utf8(
                    *(&raw mut (*dotp).l_text as *mut ::core::ffi::c_uchar)
                        .offset(doto as isize),
                ) == 0)
            {
                break;
            }
        }
    }
    doto_r = doto;
    if doto_r == 0 as ::core::ffi::c_int {
        return FALSE;
    }
    doto_l = doto_r;
    loop {
        doto_l -= 1;
        if !(doto_l > 0 as ::core::ffi::c_int
            && is_beginning_utf8(
                *(&raw mut (*dotp).l_text as *mut ::core::ffi::c_uchar)
                    .offset(doto_l as isize),
            ) == 0)
        {
            break;
        }
    }
    len_l = utf8_to_unicode(
        &raw mut (*dotp).l_text as *mut ::core::ffi::c_uchar,
        doto_l as ::core::ffi::c_uint,
        (*dotp).l_used as ::core::ffi::c_uint,
        &raw mut cl,
    ) as ::core::ffi::c_int;
    len_r = utf8_to_unicode(
        &raw mut (*dotp).l_text as *mut ::core::ffi::c_uchar,
        doto_r as ::core::ffi::c_uint,
        (*dotp).l_used as ::core::ffi::c_uint,
        &raw mut cr,
    ) as ::core::ffi::c_int;
    (*curwp).w_doto = doto_l;
    ldelete((len_l + len_r) as ::core::ffi::c_long, FALSE);
    linsert(1 as ::core::ffi::c_int, cr as ::core::ffi::c_int);
    linsert(1 as ::core::ffi::c_int, cl as ::core::ffi::c_int);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn quote(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut s: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    if (*curbp).b_mode & MDVIEW != 0 {
        return rdonly();
    }
    c = tgetc();
    if n < 0 as ::core::ffi::c_int {
        return FALSE;
    }
    if n == 0 as ::core::ffi::c_int {
        return TRUE;
    }
    if c == '\n' as i32 {
        loop {
            s = lnewline();
            if !(s == TRUE
                && {
                    n -= 1;
                    n != 0
                })
            {
                break;
            }
        }
        return s;
    }
    return linsert(n, c);
}
#[no_mangle]
pub unsafe extern "C" fn insert_tab(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if n < 0 as ::core::ffi::c_int {
        return FALSE;
    }
    if indent_selection_active != 0 && !indent_start_lp.is_null() {
        indent_end_lp = (*curwp).w_dotp;
        indent_selection_active = FALSE;
        return indent_apply_range(f, n);
    }
    if !indent_start_lp.is_null() && !indent_end_lp.is_null() {
        return indent_apply_range(f, n);
    }
    if (*curbp).b_mode & MDCMOD != 0 as ::core::ffi::c_int
        && n == 1 as ::core::ffi::c_int
    {
        let mut i: ::core::ffi::c_int = 0;
        let mut only_white: ::core::ffi::c_int = TRUE;
        i = 0 as ::core::ffi::c_int;
        while i < (*curwp).w_doto {
            let mut ch: ::core::ffi::c_int = *(&raw mut (*(*curwp).w_dotp).l_text
                as *mut ::core::ffi::c_uchar)
                .offset(i as isize) as ::core::ffi::c_int & 0xff as ::core::ffi::c_int;
            if ch != ' ' as i32 && ch != '\t' as i32 {
                only_white = FALSE;
                break;
            } else {
                i += 1;
            }
        }
        if only_white != 0 {
            let mut lp: *mut line = (*(*curwp).w_dotp).l_bp;
            let mut target: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while lp != (*curbp).b_linep {
                let mut is_blank: ::core::ffi::c_int = TRUE;
                let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while k < (*lp).l_used {
                    let mut ch2: ::core::ffi::c_int = *(&raw mut (*lp).l_text
                        as *mut ::core::ffi::c_uchar)
                        .offset(k as isize) as ::core::ffi::c_int
                        & 0xff as ::core::ffi::c_int;
                    if ch2 != ' ' as i32 && ch2 != '\t' as i32 {
                        is_blank = FALSE;
                        break;
                    } else {
                        k += 1;
                    }
                }
                if is_blank == 0 {
                    target = get_indent(lp);
                    let mut last_idx: ::core::ffi::c_int = (*lp).l_used
                        - 1 as ::core::ffi::c_int;
                    while last_idx >= 0 as ::core::ffi::c_int
                        && (*(&raw mut (*lp).l_text as *mut ::core::ffi::c_uchar)
                            .offset(last_idx as isize) as ::core::ffi::c_int
                            & 0xff as ::core::ffi::c_int == ' ' as i32
                            || *(&raw mut (*lp).l_text as *mut ::core::ffi::c_uchar)
                                .offset(last_idx as isize) as ::core::ffi::c_int
                                & 0xff as ::core::ffi::c_int == '\t' as i32)
                    {
                        last_idx -= 1;
                    }
                    if last_idx >= 0 as ::core::ffi::c_int
                        && *(&raw mut (*lp).l_text as *mut ::core::ffi::c_uchar)
                            .offset(last_idx as isize) as ::core::ffi::c_int
                            & 0xff as ::core::ffi::c_int == '{' as i32
                    {
                        target
                            += if (*curbp).b_tabsize != 0 {
                                (*curbp).b_tabsize
                            } else {
                                tab_width
                            };
                    }
                    break;
                } else {
                    lp = (*lp).l_bp;
                }
            }
            set_indent(target);
            return TRUE;
        }
    }
    if n == 0 as ::core::ffi::c_int || n > 1 as ::core::ffi::c_int {
        (*curbp).b_tabsize = n;
        if (*curbp).b_flag as ::core::ffi::c_int & BFMAKE == 0 {
            tabsize = n;
        }
        return TRUE;
    }
    if (*curbp).b_tabsize == 0 {
        return linsert(1 as ::core::ffi::c_int, '\t' as i32);
    }
    return linsert((*curbp).b_tabsize - getccol(FALSE) % (*curbp).b_tabsize, ' ' as i32);
}
#[no_mangle]
pub unsafe extern "C" fn completion_menu_command(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if completion_try_at_cursor() == 0 {
        vttbeep();
        return FALSE;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn detab(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut inc: ::core::ffi::c_int = 0;
    if (*curbp).b_mode & MDVIEW != 0 {
        return rdonly();
    }
    if f == FALSE {
        n = 1 as ::core::ffi::c_int;
    }
    inc = if n > 0 as ::core::ffi::c_int {
        1 as ::core::ffi::c_int
    } else {
        -(1 as ::core::ffi::c_int)
    };
    while n != 0 {
        (*curwp).w_doto = 0 as ::core::ffi::c_int;
        while (*curwp).w_doto < (*(*curwp).w_dotp).l_used {
            if *(&raw mut (*(*curwp).w_dotp).l_text as *mut ::core::ffi::c_uchar)
                .offset((*curwp).w_doto as isize) as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_int == '\t' as i32
            {
                ldelchar(1 as ::core::ffi::c_long, FALSE);
                let mut step: ::core::ffi::c_int = tab_width;
                if step == 0 as ::core::ffi::c_int {
                    step = 1 as ::core::ffi::c_int;
                }
                insspace(TRUE, step - (*curwp).w_doto % step);
            }
            forwchar(FALSE, 1 as ::core::ffi::c_int);
        }
        forwline(TRUE, inc);
        n -= inc;
    }
    (*curwp).w_doto = 0 as ::core::ffi::c_int;
    thisflag &= !CFCPCN;
    lchange(WFEDIT);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn entab(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut inc: ::core::ffi::c_int = 0;
    let mut fspace: ::core::ffi::c_int = 0;
    let mut ccol: ::core::ffi::c_int = 0;
    let mut cchar: ::core::ffi::c_char = 0;
    if (*curbp).b_mode & MDVIEW != 0 {
        return rdonly();
    }
    if f == FALSE {
        n = 1 as ::core::ffi::c_int;
    }
    inc = if n > 0 as ::core::ffi::c_int {
        1 as ::core::ffi::c_int
    } else {
        -(1 as ::core::ffi::c_int)
    };
    while n != 0 {
        (*curwp).w_doto = 0 as ::core::ffi::c_int;
        fspace = -(1 as ::core::ffi::c_int);
        ccol = 0 as ::core::ffi::c_int;
        while (*curwp).w_doto < (*(*curwp).w_dotp).l_used {
            if fspace >= 0 as ::core::ffi::c_int
                && (fspace / tab_width + 1 as ::core::ffi::c_int) * tab_width <= ccol
            {
                if ccol - fspace < 2 as ::core::ffi::c_int {
                    fspace = -(1 as ::core::ffi::c_int);
                } else {
                    backchar(TRUE, ccol - fspace);
                    ldelete((ccol - fspace) as ::core::ffi::c_long, FALSE);
                    linsert(1 as ::core::ffi::c_int, '\t' as i32);
                    fspace = -(1 as ::core::ffi::c_int);
                }
            }
            cchar = (*(&raw mut (*(*curwp).w_dotp).l_text as *mut ::core::ffi::c_uchar)
                .offset((*curwp).w_doto as isize) as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_int) as ::core::ffi::c_char;
            match cchar as ::core::ffi::c_int {
                9 => {
                    ccol = (ccol / tab_width + 1 as ::core::ffi::c_int) * tab_width;
                }
                32 => {
                    if fspace == -(1 as ::core::ffi::c_int) {
                        fspace = ccol;
                    }
                    ccol += 1;
                }
                _ => {
                    ccol += 1;
                    fspace = -(1 as ::core::ffi::c_int);
                }
            }
            forwchar(FALSE, 1 as ::core::ffi::c_int);
        }
        forwline(TRUE, inc);
        n -= inc;
    }
    (*curwp).w_doto = 0 as ::core::ffi::c_int;
    thisflag &= !CFCPCN;
    lchange(WFEDIT);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn trim(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut lp: *mut line = ::core::ptr::null_mut::<line>();
    let mut offset: ::core::ffi::c_int = 0;
    let mut length: ::core::ffi::c_int = 0;
    let mut inc: ::core::ffi::c_int = 0;
    if (*curbp).b_mode & MDVIEW != 0 {
        return rdonly();
    }
    if f == FALSE {
        n = 1 as ::core::ffi::c_int;
    }
    inc = if n > 0 as ::core::ffi::c_int {
        1 as ::core::ffi::c_int
    } else {
        -(1 as ::core::ffi::c_int)
    };
    while n != 0 {
        lp = (*curwp).w_dotp;
        offset = (*curwp).w_doto;
        length = (*lp).l_used;
        while length > offset {
            if *(&raw mut (*lp).l_text as *mut ::core::ffi::c_uchar)
                .offset((length - 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int & 0xff as ::core::ffi::c_int != ' ' as i32
                && *(&raw mut (*lp).l_text as *mut ::core::ffi::c_uchar)
                    .offset((length - 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int & 0xff as ::core::ffi::c_int != '\t' as i32
            {
                break;
            }
            length -= 1;
        }
        (*lp).l_used = length;
        forwline(TRUE, inc);
        n -= inc;
    }
    lchange(WFEDIT);
    thisflag &= !CFCPCN;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn openline(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut s: ::core::ffi::c_int = 0;
    if (*curbp).b_mode & MDVIEW != 0 {
        return rdonly();
    }
    if n < 0 as ::core::ffi::c_int {
        return FALSE;
    }
    if n == 0 as ::core::ffi::c_int {
        return TRUE;
    }
    i = n;
    loop {
        s = lnewline();
        if !(s == TRUE
            && {
                i -= 1;
                i != 0
            })
        {
            break;
        }
    }
    if s == TRUE {
        s = backchar(f, n);
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn insert_newline(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut s: ::core::ffi::c_int = 0;
    if (*curbp).b_mode & MDVIEW != 0 {
        return rdonly();
    }
    if n < 0 as ::core::ffi::c_int {
        return FALSE;
    }
    if n == 1 as ::core::ffi::c_int && (*curwp).w_dotp != (*curbp).b_linep {
        return cinsert();
    }
    if (*(*curwp).w_bufp).b_mode & MDWRAP != 0 && fillcol > 0 as ::core::ffi::c_int
        && getccol(FALSE) > fillcol && (*(*curwp).w_bufp).b_mode & MDVIEW == FALSE
    {
        execute(
            (META as ::core::ffi::c_uint | SPEC | 'W' as i32 as ::core::ffi::c_uint)
                as ::core::ffi::c_int,
            FALSE,
            1 as ::core::ffi::c_int,
        );
    }
    loop {
        let fresh2 = n;
        n = n - 1;
        if !(fresh2 != 0) {
            break;
        }
        s = lnewline();
        if s != TRUE {
            return s;
        }
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn cinsert() -> ::core::ffi::c_int {
    let mut bracef: ::core::ffi::c_int = 0;
    let mut target_indent: ::core::ffi::c_int = 0;
    let mut lp: *mut line = (*curwp).w_dotp;
    let mut doto: ::core::ffi::c_int = (*curwp).w_doto;
    let mut tptr: ::core::ffi::c_int = doto - 1 as ::core::ffi::c_int;
    while tptr >= 0 as ::core::ffi::c_int
        && (*(&raw mut (*lp).l_text as *mut ::core::ffi::c_uchar).offset(tptr as isize)
            as ::core::ffi::c_int & 0xff as ::core::ffi::c_int == ' ' as i32
            || *(&raw mut (*lp).l_text as *mut ::core::ffi::c_uchar)
                .offset(tptr as isize) as ::core::ffi::c_int & 0xff as ::core::ffi::c_int
                == '\t' as i32)
    {
        tptr -= 1;
    }
    bracef = (tptr >= 0 as ::core::ffi::c_int
        && *(&raw mut (*lp).l_text as *mut ::core::ffi::c_uchar).offset(tptr as isize)
            as ::core::ffi::c_int & 0xff as ::core::ffi::c_int == '{' as i32)
        as ::core::ffi::c_int;
    check_indent_dedent();
    lp = (*curwp).w_dotp;
    target_indent = get_indent(lp);
    let mut ppf: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*curbp).b_mode & MDCMOD != 0 {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*lp).l_used
            && *(*__ctype_b_loc())
                .offset(
                    (*(&raw mut (*lp).l_text as *mut ::core::ffi::c_uchar)
                        .offset(i as isize) as ::core::ffi::c_int
                        & 0xff as ::core::ffi::c_int) as isize,
                ) as ::core::ffi::c_int
                & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int != 0
        {
            i += 1;
        }
        if i < (*lp).l_used
            && *(&raw mut (*lp).l_text as *mut ::core::ffi::c_uchar).offset(i as isize)
                as ::core::ffi::c_int & 0xff as ::core::ffi::c_int == '#' as i32
        {
            let mut word: [::core::ffi::c_char; 32] = [0; 32];
            let mut idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < (*lp).l_used
                && *(*__ctype_b_loc())
                    .offset(
                        (*(&raw mut (*lp).l_text as *mut ::core::ffi::c_uchar)
                            .offset(i as isize) as ::core::ffi::c_int
                            & 0xff as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int
                    & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                        as ::core::ffi::c_int == 0 && idx < 31 as ::core::ffi::c_int
            {
                let fresh3 = idx;
                idx = idx + 1;
                word[fresh3 as usize] = (*(&raw mut (*lp).l_text
                    as *mut ::core::ffi::c_uchar)
                    .offset(i as isize) as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int) as ::core::ffi::c_char;
                i += 1;
            }
            word[idx as usize] = '\0' as i32 as ::core::ffi::c_char;
            if strcmp(
                &raw mut word as *mut ::core::ffi::c_char,
                b"#if\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                || strcmp(
                    &raw mut word as *mut ::core::ffi::c_char,
                    b"#ifdef\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                || strcmp(
                    &raw mut word as *mut ::core::ffi::c_char,
                    b"#ifndef\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                || strcmp(
                    &raw mut word as *mut ::core::ffi::c_char,
                    b"#else\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                || strcmp(
                    &raw mut word as *mut ::core::ffi::c_char,
                    b"#elif\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                ppf = 1 as ::core::ffi::c_int;
            }
        }
    }
    if lnewline() == FALSE {
        return FALSE;
    }
    set_indent(target_indent);
    if bracef != 0 || ppf != 0 {
        let mut step: ::core::ffi::c_int = if (*curbp).b_tabsize != 0 {
            (*curbp).b_tabsize
        } else {
            tab_width + 1 as ::core::ffi::c_int
        };
        if step == 8 as ::core::ffi::c_int && !nanox_cfg.soft_tab {
            linsert(1 as ::core::ffi::c_int, '\t' as i32);
        } else {
            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_0 < step {
                linsert(1 as ::core::ffi::c_int, ' ' as i32);
                i_0 += 1;
            }
        }
        (*curwp).w_doto = (*(*curwp).w_dotp).l_used;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn insbrace(
    mut n: ::core::ffi::c_int,
    mut c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ch: ::core::ffi::c_int = 0;
    let mut oc: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut target: ::core::ffi::c_int = 0;
    let mut oldlp: *mut line = ::core::ptr::null_mut::<line>();
    let mut oldoff: ::core::ffi::c_int = 0;
    if (*curwp).w_doto < (*(*curwp).w_dotp).l_used
        && *(&raw mut (*(*curwp).w_dotp).l_text as *mut ::core::ffi::c_uchar)
            .offset((*curwp).w_doto as isize) as ::core::ffi::c_int
            & 0xff as ::core::ffi::c_int == c
    {
        (*curwp).w_doto += 1;
        return TRUE;
    }
    if (*curwp).w_doto != 0 as ::core::ffi::c_int {
        i = (*curwp).w_doto - 1 as ::core::ffi::c_int;
        while i >= 0 as ::core::ffi::c_int {
            ch = *(&raw mut (*(*curwp).w_dotp).l_text as *mut ::core::ffi::c_uchar)
                .offset(i as isize) as ::core::ffi::c_int & 0xff as ::core::ffi::c_int;
            if ch != ' ' as i32 && ch != '\t' as i32 {
                return linsert(n, c);
            }
            i -= 1;
        }
    }
    if (*curwp).w_doto < (*(*curwp).w_dotp).l_used {
        i = (*curwp).w_doto;
        while i < (*(*curwp).w_dotp).l_used {
            ch = *(&raw mut (*(*curwp).w_dotp).l_text as *mut ::core::ffi::c_uchar)
                .offset(i as isize) as ::core::ffi::c_int & 0xff as ::core::ffi::c_int;
            if ch != ' ' as i32 && ch != '\t' as i32 {
                return linsert(n, c);
            }
            i += 1;
        }
    }
    match c {
        125 => {
            oc = '{' as i32;
        }
        93 => {
            oc = '[' as i32;
        }
        41 => {
            oc = '(' as i32;
        }
        _ => return linsert(n, c),
    }
    oldlp = (*curwp).w_dotp;
    oldoff = (*curwp).w_doto;
    count = 1 as ::core::ffi::c_int;
    while backchar(FALSE, 1 as ::core::ffi::c_int) != 0 {
        if (*curwp).w_doto == (*(*curwp).w_dotp).l_used {
            ch = '\n' as i32;
        } else {
            ch = *(&raw mut (*(*curwp).w_dotp).l_text as *mut ::core::ffi::c_uchar)
                .offset((*curwp).w_doto as isize) as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_int;
        }
        if ch == c {
            count += 1;
        } else if ch == oc {
            count -= 1;
        }
        if count == 0 as ::core::ffi::c_int {
            break;
        }
        if boundry((*curwp).w_dotp, (*curwp).w_doto, REVERSE) != 0 {
            break;
        }
    }
    if count != 0 as ::core::ffi::c_int {
        let mut lp: *mut line = (*oldlp).l_bp;
        target = 0 as ::core::ffi::c_int;
        while lp != (*curbp).b_linep {
            let mut is_blank: ::core::ffi::c_int = TRUE;
            let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while k < (*lp).l_used {
                let mut ch2: ::core::ffi::c_int = *(&raw mut (*lp).l_text
                    as *mut ::core::ffi::c_uchar)
                    .offset(k as isize) as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int;
                if ch2 != ' ' as i32 && ch2 != '\t' as i32 {
                    is_blank = FALSE;
                    break;
                } else {
                    k += 1;
                }
            }
            if is_blank == 0 {
                target = get_indent(lp);
                break;
            } else {
                lp = (*lp).l_bp;
            }
        }
        (*curwp).w_dotp = oldlp;
        (*curwp).w_doto = oldoff;
    } else {
        target = get_indent((*curwp).w_dotp);
        (*curwp).w_dotp = oldlp;
        (*curwp).w_doto = oldoff;
    }
    set_indent(target);
    (*curwp).w_doto = (*(*curwp).w_dotp).l_used;
    return linsert(n, c);
}
#[no_mangle]
pub unsafe extern "C" fn inspound() -> ::core::ffi::c_int {
    let mut ch: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    if (*curwp).w_doto == 0 as ::core::ffi::c_int {
        return linsert(1 as ::core::ffi::c_int, '#' as i32);
    }
    i = (*curwp).w_doto - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        ch = *(&raw mut (*(*curwp).w_dotp).l_text as *mut ::core::ffi::c_uchar)
            .offset(i as isize) as ::core::ffi::c_int & 0xff as ::core::ffi::c_int;
        if ch != ' ' as i32 && ch != '\t' as i32 {
            return linsert(1 as ::core::ffi::c_int, '#' as i32);
        }
        i -= 1;
    }
    if (*curbp).b_mode & MDCMOD != 0 {
        return linsert(1 as ::core::ffi::c_int, '#' as i32);
    }
    while getccol(FALSE) >= 1 as ::core::ffi::c_int {
        backdel(FALSE, 1 as ::core::ffi::c_int);
    }
    return linsert(1 as ::core::ffi::c_int, '#' as i32);
}
#[no_mangle]
pub unsafe extern "C" fn deblank(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut lp1: *mut line = ::core::ptr::null_mut::<line>();
    let mut lp2: *mut line = ::core::ptr::null_mut::<line>();
    let mut nld: ::core::ffi::c_long = 0;
    if (*curbp).b_mode & MDVIEW != 0 {
        return rdonly();
    }
    lp1 = (*curwp).w_dotp;
    while (*lp1).l_used == 0 as ::core::ffi::c_int
        && {
            lp2 = (*lp1).l_bp;
            lp2 != (*curbp).b_linep
        }
    {
        lp1 = lp2;
    }
    lp2 = lp1;
    nld = 0 as ::core::ffi::c_long;
    loop {
        lp2 = (*lp2).l_fp;
        if !(lp2 != (*curbp).b_linep && (*lp2).l_used == 0 as ::core::ffi::c_int) {
            break;
        }
        nld += 1;
    }
    if nld == 0 as ::core::ffi::c_long {
        return TRUE;
    }
    (*curwp).w_dotp = (*lp1).l_fp;
    (*curwp).w_doto = 0 as ::core::ffi::c_int;
    return ldelete(nld, FALSE);
}
#[no_mangle]
pub unsafe extern "C" fn indent(
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
        let fresh9 = n;
        n = n - 1;
        if !(fresh9 != 0) {
            break;
        }
        if (*curbp).b_mode & MDCMOD != 0 {
            check_indent_dedent();
        }
        let mut lp: *mut line = (*curwp).w_dotp;
        let mut target_indent: ::core::ffi::c_int = get_indent(lp);
        let mut doto: ::core::ffi::c_int = (*lp).l_used;
        let mut tptr: ::core::ffi::c_int = doto - 1 as ::core::ffi::c_int;
        while tptr >= 0 as ::core::ffi::c_int
            && (*(&raw mut (*lp).l_text as *mut ::core::ffi::c_uchar)
                .offset(tptr as isize) as ::core::ffi::c_int & 0xff as ::core::ffi::c_int
                == ' ' as i32
                || *(&raw mut (*lp).l_text as *mut ::core::ffi::c_uchar)
                    .offset(tptr as isize) as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int == '\t' as i32)
        {
            tptr -= 1;
        }
        let mut bracef: ::core::ffi::c_int = (tptr >= 0 as ::core::ffi::c_int
            && *(&raw mut (*lp).l_text as *mut ::core::ffi::c_uchar)
                .offset(tptr as isize) as ::core::ffi::c_int & 0xff as ::core::ffi::c_int
                == '{' as i32) as ::core::ffi::c_int;
        let mut ppf: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if (*curbp).b_mode & MDCMOD != 0 {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < (*lp).l_used
                && *(*__ctype_b_loc())
                    .offset(
                        (*(&raw mut (*lp).l_text as *mut ::core::ffi::c_uchar)
                            .offset(i as isize) as ::core::ffi::c_int
                            & 0xff as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int
                    & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                        as ::core::ffi::c_int != 0
            {
                i += 1;
            }
            if i < (*lp).l_used
                && *(&raw mut (*lp).l_text as *mut ::core::ffi::c_uchar)
                    .offset(i as isize) as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int == '#' as i32
            {
                let mut word: [::core::ffi::c_char; 32] = [0; 32];
                let mut idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i < (*lp).l_used
                    && *(*__ctype_b_loc())
                        .offset(
                            (*(&raw mut (*lp).l_text as *mut ::core::ffi::c_uchar)
                                .offset(i as isize) as ::core::ffi::c_int
                                & 0xff as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int
                        & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                            as ::core::ffi::c_int == 0 && idx < 31 as ::core::ffi::c_int
                {
                    let fresh10 = idx;
                    idx = idx + 1;
                    word[fresh10 as usize] = (*(&raw mut (*lp).l_text
                        as *mut ::core::ffi::c_uchar)
                        .offset(i as isize) as ::core::ffi::c_int
                        & 0xff as ::core::ffi::c_int) as ::core::ffi::c_char;
                    i += 1;
                }
                word[idx as usize] = '\0' as i32 as ::core::ffi::c_char;
                if strcmp(
                    &raw mut word as *mut ::core::ffi::c_char,
                    b"#if\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                    || strcmp(
                        &raw mut word as *mut ::core::ffi::c_char,
                        b"#ifdef\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    || strcmp(
                        &raw mut word as *mut ::core::ffi::c_char,
                        b"#ifndef\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    || strcmp(
                        &raw mut word as *mut ::core::ffi::c_char,
                        b"#else\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    || strcmp(
                        &raw mut word as *mut ::core::ffi::c_char,
                        b"#elif\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                {
                    ppf = 1 as ::core::ffi::c_int;
                }
            }
        }
        if bracef != 0 || ppf != 0 {
            target_indent
                += if (*curbp).b_tabsize != 0 { (*curbp).b_tabsize } else { tab_width };
        }
        if lnewline() == FALSE {
            return FALSE;
        }
        set_indent(target_indent);
    }
    return TRUE;
}
pub const MAX_INDENT_DETECT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
unsafe extern "C" fn detect_indent_step() -> ::core::ffi::c_int {
    let mut lp: *mut line = ::core::ptr::null_mut::<line>();
    let mut freq: [::core::ffi::c_int; 9] = [0; 9];
    let mut i: ::core::ffi::c_int = 0;
    let mut prev_indent: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    i = 0 as ::core::ffi::c_int;
    while i <= MAX_INDENT_DETECT {
        freq[i as usize] = 0 as ::core::ffi::c_int;
        i += 1;
    }
    lp = (*(*curbp).b_linep).l_fp;
    while lp != (*curbp).b_linep {
        if (*lp).l_used > 0 as ::core::ffi::c_int
            && *(&raw mut (*lp).l_text as *mut ::core::ffi::c_uchar)
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_int == '\t' as i32
        {
            return tab_width;
        }
        lp = (*lp).l_fp;
    }
    lp = (*(*curbp).b_linep).l_fp;
    while lp != (*curbp).b_linep {
        let mut blank: ::core::ffi::c_int = TRUE;
        i = 0 as ::core::ffi::c_int;
        while i < (*lp).l_used {
            let mut c: ::core::ffi::c_int = *(&raw mut (*lp).l_text
                as *mut ::core::ffi::c_uchar)
                .offset(i as isize) as ::core::ffi::c_int & 0xff as ::core::ffi::c_int;
            if c != ' ' as i32 && c != '\t' as i32 {
                blank = FALSE;
                break;
            } else {
                i += 1;
            }
        }
        if blank == 0 {
            let mut ind: ::core::ffi::c_int = get_indent(lp);
            if prev_indent >= 0 as ::core::ffi::c_int && ind > prev_indent {
                let mut delta: ::core::ffi::c_int = ind - prev_indent;
                if delta >= 1 as ::core::ffi::c_int && delta <= MAX_INDENT_DETECT {
                    freq[delta as usize] += 1;
                }
            }
            prev_indent = ind;
        }
        lp = (*lp).l_fp;
    }
    let mut best: ::core::ffi::c_int = tab_width;
    let mut best_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i <= MAX_INDENT_DETECT {
        if freq[i as usize] > best_count {
            best_count = freq[i as usize];
            best = i;
        }
        i += 1;
    }
    return best;
}
unsafe extern "C" fn adjust_indent(mut lp: *mut line, mut delta: ::core::ffi::c_int) {
    let mut step: ::core::ffi::c_int = if (*curbp).b_tabsize != 0 {
        (*curbp).b_tabsize
    } else {
        detect_indent_step()
    };
    let mut cur: ::core::ffi::c_int = get_indent(lp);
    let mut target: ::core::ffi::c_int = cur + delta * step;
    if target < 0 as ::core::ffi::c_int {
        target = 0 as ::core::ffi::c_int;
    }
    let mut save_lp: *mut line = (*curwp).w_dotp;
    let mut save_doto: ::core::ffi::c_int = (*curwp).w_doto;
    (*curwp).w_dotp = lp;
    set_indent(target);
    if !(save_lp == lp) {
        (*curwp).w_dotp = save_lp;
        (*curwp).w_doto = save_doto;
    }
}
unsafe extern "C" fn line_number_for(mut lp: *mut line) -> ::core::ffi::c_long {
    if lp.is_null() {
        return -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
    }
    let mut lineno: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut scan: *mut line = (*curbp).b_linep;
    loop {
        scan = (*scan).l_fp;
        if !(scan != (*curbp).b_linep) {
            break;
        }
        lineno += 1;
        if scan == lp {
            return lineno;
        }
    }
    return -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
}
unsafe extern "C" fn indent_mode_label() -> *const ::core::ffi::c_char {
    return if indent_range_type < 0 as ::core::ffi::c_int {
        b"Outdent\0" as *const u8 as *const ::core::ffi::c_char
    } else {
        b"Indent\0" as *const u8 as *const ::core::ffi::c_char
    };
}
unsafe extern "C" fn indent_start_key() -> ::core::ffi::c_char {
    return (if indent_range_type < 0 as ::core::ffi::c_int {
        'H' as i32
    } else {
        'J' as i32
    }) as ::core::ffi::c_char;
}
unsafe extern "C" fn announce_indent_state(mut action: *const ::core::ffi::c_char) {
    let mut mode: *const ::core::ffi::c_char = indent_mode_label();
    let mut start_key: ::core::ffi::c_char = indent_start_key();
    let mut start_line: ::core::ffi::c_long = line_number_for(indent_start_lp);
    let mut end_line: ::core::ffi::c_long = line_number_for(indent_end_lp);
    if !indent_start_lp.is_null() && !indent_end_lp.is_null()
        && start_line > 0 as ::core::ffi::c_long && end_line > 0 as ::core::ffi::c_long
    {
        let mut first: ::core::ffi::c_long = start_line;
        let mut last: ::core::ffi::c_long = end_line;
        if first > last {
            let mut tmp: ::core::ffi::c_long = first;
            first = last;
            last = tmp;
        }
        mlwrite(
            b"[%s: lines %ld-%ld | Tab/gg: %s | BS: cancel]\0" as *const u8
                as *const ::core::ffi::c_char,
            action,
            first,
            last,
            if indent_range_type < 0 as ::core::ffi::c_int {
                b"outdent\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"indent\0" as *const u8 as *const ::core::ffi::c_char
            },
        );
        return;
    }
    if !indent_start_lp.is_null() && start_line > 0 as ::core::ffi::c_long {
        mlwrite(
            b"[%s: %s start line %ld | move to end, then Tab/gg | BS: cancel]\0"
                as *const u8 as *const ::core::ffi::c_char,
            action,
            mode,
            start_line,
        );
        return;
    }
    if !indent_start_lp.is_null() {
        mlwrite(
            b"[%s: %s start set | move to end, then Tab/gg | BS: cancel]\0" as *const u8
                as *const ::core::ffi::c_char,
            action,
            mode,
        );
        return;
    }
    if !indent_end_lp.is_null() && end_line > 0 as ::core::ffi::c_long {
        mlwrite(
            b"[%s: %s end line %ld | set start with Ctrl+%c]\0" as *const u8
                as *const ::core::ffi::c_char,
            action,
            mode,
            end_line,
            start_key as ::core::ffi::c_int,
        );
        return;
    }
    if !indent_end_lp.is_null() {
        mlwrite(
            b"[%s: %s end set | set start with Ctrl+%c]\0" as *const u8
                as *const ::core::ffi::c_char,
            action,
            mode,
            start_key as ::core::ffi::c_int,
        );
        return;
    }
    mlwrite(b"[%s]\0" as *const u8 as *const ::core::ffi::c_char, action);
}
unsafe extern "C" fn indent_reset_range() {
    indent_selection_active = FALSE;
    indent_start_lp = ::core::ptr::null_mut::<line>();
    indent_end_lp = ::core::ptr::null_mut::<line>();
    indent_range_type = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn indent_range_pending() -> ::core::ffi::c_int {
    if indent_selection_active != 0 {
        return TRUE;
    }
    if !indent_start_lp.is_null() && !indent_end_lp.is_null() {
        return TRUE;
    }
    return FALSE;
}
unsafe extern "C" fn indent_set_range_from_mark() -> ::core::ffi::c_int {
    if (*curwp).w_markp.is_null() {
        indent_start_lp = ::core::ptr::null_mut::<line>();
        indent_end_lp = ::core::ptr::null_mut::<line>();
        mlwrite(
            b"[%s: Set start with Ctrl+%c first]\0" as *const u8
                as *const ::core::ffi::c_char,
            indent_mode_label(),
            indent_start_key() as ::core::ffi::c_int,
        );
        return FALSE;
    }
    indent_start_lp = (*curwp).w_markp;
    indent_end_lp = (*curwp).w_dotp;
    if indent_start_lp == indent_end_lp {
        return TRUE;
    }
    let mut scan: *mut line = (*curbp).b_linep;
    let mut first: *mut line = ::core::ptr::null_mut::<line>();
    let mut second: *mut line = ::core::ptr::null_mut::<line>();
    loop {
        scan = (*scan).l_fp;
        if !(scan != (*curbp).b_linep) {
            break;
        }
        if scan == (*curwp).w_markp {
            first = (*curwp).w_markp;
            second = (*curwp).w_dotp;
            break;
        } else {
            if !(scan == (*curwp).w_dotp) {
                continue;
            }
            first = (*curwp).w_dotp;
            second = (*curwp).w_markp;
            break;
        }
    }
    if first.is_null() || second.is_null() {
        indent_start_lp = ::core::ptr::null_mut::<line>();
        indent_end_lp = ::core::ptr::null_mut::<line>();
        mlwrite(
            b"[%s: Selection invalid. Set range again]\0" as *const u8
                as *const ::core::ffi::c_char,
            indent_mode_label(),
        );
        return FALSE;
    }
    indent_start_lp = first;
    indent_end_lp = second;
    return TRUE;
}
unsafe extern "C" fn indent_begin_range(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
    mut type_0: ::core::ffi::c_int,
    mut action: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut status: ::core::ffi::c_int = setmark(f, n);
    if status != TRUE {
        return status;
    }
    indent_range_type = type_0;
    indent_selection_active = TRUE;
    indent_start_lp = (*curwp).w_dotp;
    indent_end_lp = ::core::ptr::null_mut::<line>();
    announce_indent_state(action);
    return TRUE;
}
unsafe extern "C" fn indent_finalize_range(
    mut type_0: ::core::ffi::c_int,
    mut action: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    indent_range_type = type_0;
    if indent_selection_active == 0 {
        mlwrite(
            b"[%s: Set start with Ctrl+%c first]\0" as *const u8
                as *const ::core::ffi::c_char,
            indent_mode_label(),
            indent_start_key() as ::core::ffi::c_int,
        );
        return FALSE;
    }
    if indent_set_range_from_mark() == 0 {
        indent_reset_range();
        return FALSE;
    }
    indent_selection_active = FALSE;
    announce_indent_state(action);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn indent_start_set(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return indent_begin_range(
        f,
        n,
        1 as ::core::ffi::c_int,
        b"Indent start set\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn indent_end_set(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return indent_finalize_range(
        1 as ::core::ffi::c_int,
        b"Indent end set\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn outdent_start_set(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return indent_begin_range(
        f,
        n,
        -(1 as ::core::ffi::c_int),
        b"Outdent start set\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn outdent_end_set(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return indent_finalize_range(
        -(1 as ::core::ffi::c_int),
        b"Outdent end set\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn indent_apply_range(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if indent_start_lp.is_null() || indent_end_lp.is_null() {
        mlwrite(
            b"[Range not set. Use Ctrl+J (indent) or Ctrl+H (outdent) to mark start, move cursor to end, then Tab or gg]\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return FALSE;
    }
    if (*curbp).b_mode & MDVIEW != 0 {
        return rdonly();
    }
    let mut lp1: *mut line = indent_start_lp;
    let mut lp2: *mut line = indent_end_lp;
    let mut scan: *mut line = (*curbp).b_linep;
    let mut first: *mut line = ::core::ptr::null_mut::<line>();
    let mut last: *mut line = ::core::ptr::null_mut::<line>();
    loop {
        scan = (*scan).l_fp;
        if !(scan != (*curbp).b_linep) {
            break;
        }
        if scan == lp1 {
            if first.is_null() {
                first = lp1;
                last = lp2;
            }
            break;
        } else {
            if !(scan == lp2) {
                continue;
            }
            if first.is_null() {
                first = lp2;
                last = lp1;
            }
            break;
        }
    }
    if first.is_null() {
        indent_reset_range();
        mlwrite(
            b"[gg: Selection invalid. Mark the range again.]\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return FALSE;
    }
    let mut start_line: ::core::ffi::c_long = line_number_for(lp1);
    let mut end_line: ::core::ffi::c_long = line_number_for(lp2);
    scan = first;
    while scan != (*last).l_fp {
        adjust_indent(scan, indent_range_type);
        scan = (*scan).l_fp;
    }
    lchange(WFHARD);
    let mut mode: *const ::core::ffi::c_char = indent_mode_label();
    if start_line > 0 as ::core::ffi::c_long && end_line > 0 as ::core::ffi::c_long {
        let mut first_line: ::core::ffi::c_long = start_line;
        let mut last_line: ::core::ffi::c_long = end_line;
        if first_line > last_line {
            let mut tmp: ::core::ffi::c_long = first_line;
            first_line = last_line;
            last_line = tmp;
        }
        mlwrite(
            b"[%s applied: lines %ld-%ld]\0" as *const u8 as *const ::core::ffi::c_char,
            mode,
            first_line,
            last_line,
        );
    } else {
        mlwrite(
            b"[%s range applied]\0" as *const u8 as *const ::core::ffi::c_char,
            mode,
        );
    }
    indent_reset_range();
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn indent_cancel(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !indent_start_lp.is_null() || !indent_end_lp.is_null()
        || indent_selection_active != 0
    {
        indent_reset_range();
        mlwrite(
            b"[Indent selection canceled | start with Ctrl+J or Ctrl+H]\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return TRUE;
    }
    return backdel(f, n);
}
#[no_mangle]
pub unsafe extern "C" fn g_prefix_handler(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = 0;
    if indent_range_pending() == 0 {
        return linsert(1 as ::core::ffi::c_int, 'g' as i32);
    }
    loop {
        c = getcmd();
        if !(c == 0 as ::core::ffi::c_int) {
            break;
        }
        ttpause();
    }
    if c == 'g' as i32 {
        if indent_selection_active != 0 && !indent_start_lp.is_null() {
            indent_end_lp = (*curwp).w_dotp;
            indent_selection_active = FALSE;
        }
        return indent_apply_range(f, n);
    }
    linsert(1 as ::core::ffi::c_int, 'g' as i32);
    execute(c, FALSE, 1 as ::core::ffi::c_int);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn forwdel(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if (*curbp).b_mode & MDVIEW != 0 {
        return rdonly();
    }
    if n < 0 as ::core::ffi::c_int {
        return backdel(f, -n);
    }
    if f != FALSE {
        if lastflag & CFKILL == 0 as ::core::ffi::c_int {
            kdelete();
        }
        thisflag |= CFKILL;
    }
    return ldelchar(n as ::core::ffi::c_long, f);
}
#[no_mangle]
pub unsafe extern "C" fn backdel(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut s: ::core::ffi::c_int = 0;
    if (*curbp).b_mode & MDVIEW != 0 {
        return rdonly();
    }
    if n < 0 as ::core::ffi::c_int {
        return forwdel(f, -n);
    }
    if f != FALSE {
        if lastflag & CFKILL == 0 as ::core::ffi::c_int {
            kdelete();
        }
        thisflag |= CFKILL;
    }
    s = backchar(f, n);
    if s == TRUE {
        s = ldelchar(n as ::core::ffi::c_long, f);
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn killtext(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut nextp: *mut line = ::core::ptr::null_mut::<line>();
    let mut chunk: ::core::ffi::c_long = 0;
    if (*curbp).b_mode & MDVIEW != 0 {
        return rdonly();
    }
    if lastflag & CFKILL == 0 as ::core::ffi::c_int {
        kdelete();
    }
    thisflag |= CFKILL;
    if f == FALSE {
        chunk = ((*(*curwp).w_dotp).l_used - (*curwp).w_doto) as ::core::ffi::c_long;
        if chunk == 0 as ::core::ffi::c_long {
            chunk = 1 as ::core::ffi::c_long;
        }
    } else if n == 0 as ::core::ffi::c_int {
        chunk = (*curwp).w_doto as ::core::ffi::c_long;
        (*curwp).w_doto = 0 as ::core::ffi::c_int;
    } else if n > 0 as ::core::ffi::c_int {
        chunk = ((*(*curwp).w_dotp).l_used - (*curwp).w_doto + 1 as ::core::ffi::c_int)
            as ::core::ffi::c_long;
        nextp = (*(*curwp).w_dotp).l_fp;
        loop {
            n -= 1;
            if !(n != 0) {
                break;
            }
            if nextp == (*curbp).b_linep {
                return FALSE;
            }
            chunk += ((*nextp).l_used + 1 as ::core::ffi::c_int) as ::core::ffi::c_long;
            nextp = (*nextp).l_fp;
        }
    } else {
        mlwrite(b"neg kill\0" as *const u8 as *const ::core::ffi::c_char);
        return FALSE;
    }
    return ldelete(chunk, TRUE);
}
#[no_mangle]
pub unsafe extern "C" fn setemode(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return adjustmode(TRUE, FALSE);
}
#[no_mangle]
pub unsafe extern "C" fn delmode(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return adjustmode(FALSE, FALSE);
}
#[no_mangle]
pub unsafe extern "C" fn setgmode(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return adjustmode(TRUE, TRUE);
}
#[no_mangle]
pub unsafe extern "C" fn delgmode(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return adjustmode(FALSE, TRUE);
}
#[no_mangle]
pub unsafe extern "C" fn adjustmode(
    mut kind: ::core::ffi::c_int,
    mut global: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut scan: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut i: ::core::ffi::c_int = 0;
    let mut status: ::core::ffi::c_int = 0;
    let mut prompt: [::core::ffi::c_char; 50] = [0; 50];
    let mut cbuf: [::core::ffi::c_char; 1024] = [0; 1024];
    if global != 0 {
        strcpy(
            &raw mut prompt as *mut ::core::ffi::c_char,
            b"Global mode to \0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        strcpy(
            &raw mut prompt as *mut ::core::ffi::c_char,
            b"Mode to \0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if kind == TRUE {
        strcat(
            &raw mut prompt as *mut ::core::ffi::c_char,
            b"add: \0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        strcat(
            &raw mut prompt as *mut ::core::ffi::c_char,
            b"delete: \0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    status = minibuf_input(
        &raw mut prompt as *mut ::core::ffi::c_char,
        &raw mut cbuf as *mut ::core::ffi::c_char,
        NPAT - 1 as ::core::ffi::c_int,
    );
    if status != TRUE {
        return status;
    }
    scan = &raw mut cbuf as *mut ::core::ffi::c_char;
    while *scan as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        if *scan as ::core::ffi::c_int >= 'a' as i32
            && *scan as ::core::ffi::c_int <= 'z' as i32
        {
            *scan = (*scan as ::core::ffi::c_int - 32 as ::core::ffi::c_int)
                as ::core::ffi::c_char;
        }
        scan = scan.offset(1);
    }
    i = 0 as ::core::ffi::c_int;
    while i < NUMMODES {
        if strcmp(
            &raw mut cbuf as *mut ::core::ffi::c_char,
            *(&raw mut modename as *mut *mut ::core::ffi::c_char).offset(i as isize),
        ) == 0 as ::core::ffi::c_int
        {
            if kind == TRUE {
                if global != 0 {
                    gmode |= (1 as ::core::ffi::c_int) << i;
                } else {
                    (*curbp).b_mode |= (1 as ::core::ffi::c_int) << i;
                }
            } else if global != 0 {
                gmode &= !((1 as ::core::ffi::c_int) << i);
            } else {
                (*curbp).b_mode &= !((1 as ::core::ffi::c_int) << i);
            }
            if global == 0 as ::core::ffi::c_int {
                upmode();
            }
            mlerase();
            return TRUE;
        }
        i += 1;
    }
    mlwrite(b"No such mode!\0" as *const u8 as *const ::core::ffi::c_char);
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn clrmes(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    mlforce(
        b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn writemsg(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut sp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut np: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut status: ::core::ffi::c_int = 0;
    let mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut nbuf: [::core::ffi::c_char; 2048] = [0; 2048];
    status = minibuf_input(
        b"Message to write: \0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut buf as *mut ::core::ffi::c_char,
        NPAT - 1 as ::core::ffi::c_int,
    );
    if status != TRUE {
        return status;
    }
    sp = &raw mut buf as *mut ::core::ffi::c_char;
    np = &raw mut nbuf as *mut ::core::ffi::c_char;
    while *sp != 0 {
        let fresh11 = np;
        np = np.offset(1);
        *fresh11 = *sp;
        let fresh12 = sp;
        sp = sp.offset(1);
        if *fresh12 as ::core::ffi::c_int == '%' as i32 {
            let fresh13 = np;
            np = np.offset(1);
            *fresh13 = '%' as i32 as ::core::ffi::c_char;
        }
    }
    *np = '\0' as i32 as ::core::ffi::c_char;
    mlforce(&raw mut nbuf as *mut ::core::ffi::c_char);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn getfence(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut oldlp: *mut line = ::core::ptr::null_mut::<line>();
    let mut oldoff: ::core::ffi::c_int = 0;
    let mut sdir: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut ch: ::core::ffi::c_char = 0;
    let mut ofence: ::core::ffi::c_char = 0;
    let mut c: ::core::ffi::c_char = 0;
    oldlp = (*curwp).w_dotp;
    oldoff = (*curwp).w_doto;
    if oldoff == (*oldlp).l_used {
        ch = '\n' as i32 as ::core::ffi::c_char;
    } else {
        ch = (*(&raw mut (*oldlp).l_text as *mut ::core::ffi::c_uchar)
            .offset(oldoff as isize) as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
            as ::core::ffi::c_char;
    }
    match ch as ::core::ffi::c_int {
        40 => {
            ofence = ')' as i32 as ::core::ffi::c_char;
            sdir = FORWARD;
        }
        123 => {
            ofence = '}' as i32 as ::core::ffi::c_char;
            sdir = FORWARD;
        }
        91 => {
            ofence = ']' as i32 as ::core::ffi::c_char;
            sdir = FORWARD;
        }
        41 => {
            ofence = '(' as i32 as ::core::ffi::c_char;
            sdir = REVERSE;
        }
        125 => {
            ofence = '{' as i32 as ::core::ffi::c_char;
            sdir = REVERSE;
        }
        93 => {
            ofence = '[' as i32 as ::core::ffi::c_char;
            sdir = REVERSE;
        }
        _ => {
            vttbeep();
            return FALSE;
        }
    }
    count = 1 as ::core::ffi::c_int;
    if sdir == REVERSE {
        backchar(FALSE, 1 as ::core::ffi::c_int);
    } else {
        forwchar(FALSE, 1 as ::core::ffi::c_int);
    }
    while count > 0 as ::core::ffi::c_int {
        if (*curwp).w_doto == (*(*curwp).w_dotp).l_used {
            c = '\n' as i32 as ::core::ffi::c_char;
        } else {
            c = (*(&raw mut (*(*curwp).w_dotp).l_text as *mut ::core::ffi::c_uchar)
                .offset((*curwp).w_doto as isize) as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_int) as ::core::ffi::c_char;
        }
        if c as ::core::ffi::c_int == ch as ::core::ffi::c_int {
            count += 1;
        }
        if c as ::core::ffi::c_int == ofence as ::core::ffi::c_int {
            count -= 1;
        }
        if sdir == FORWARD {
            forwchar(FALSE, 1 as ::core::ffi::c_int);
        } else {
            backchar(FALSE, 1 as ::core::ffi::c_int);
        }
        if boundry((*curwp).w_dotp, (*curwp).w_doto, sdir) != 0 {
            break;
        }
    }
    if count == 0 as ::core::ffi::c_int {
        if sdir == FORWARD {
            backchar(FALSE, 1 as ::core::ffi::c_int);
        } else {
            forwchar(FALSE, 1 as ::core::ffi::c_int);
        }
        (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | WFMOVE)
            as ::core::ffi::c_char;
        return TRUE;
    }
    (*curwp).w_dotp = oldlp;
    (*curwp).w_doto = oldoff;
    vttbeep();
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn fmatch(mut ch: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut oldlp: *mut line = ::core::ptr::null_mut::<line>();
    let mut oldoff: ::core::ffi::c_int = 0;
    let mut toplp: *mut line = ::core::ptr::null_mut::<line>();
    let mut count: ::core::ffi::c_int = 0;
    let mut opench: ::core::ffi::c_char = 0;
    let mut c: ::core::ffi::c_char = 0;
    update(FALSE);
    oldlp = (*curwp).w_dotp;
    oldoff = (*curwp).w_doto;
    if ch == ')' as i32 {
        opench = '(' as i32 as ::core::ffi::c_char;
    } else if ch == '}' as i32 {
        opench = '{' as i32 as ::core::ffi::c_char;
    } else {
        opench = '[' as i32 as ::core::ffi::c_char;
    }
    toplp = (*(*curwp).w_linep).l_bp;
    count = 1 as ::core::ffi::c_int;
    if backchar(FALSE, 2 as ::core::ffi::c_int) == FALSE {
        return TRUE;
    }
    while count > 0 as ::core::ffi::c_int && (*curwp).w_dotp != toplp {
        if (*curwp).w_doto == (*(*curwp).w_dotp).l_used {
            c = '\n' as i32 as ::core::ffi::c_char;
        } else {
            c = (*(&raw mut (*(*curwp).w_dotp).l_text as *mut ::core::ffi::c_uchar)
                .offset((*curwp).w_doto as isize) as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_int) as ::core::ffi::c_char;
        }
        if c as ::core::ffi::c_int == ch {
            count += 1;
        }
        if c as ::core::ffi::c_int == opench as ::core::ffi::c_int {
            count -= 1;
        }
        backchar(FALSE, 1 as ::core::ffi::c_int);
        if (*curwp).w_dotp == (*(*(*curwp).w_bufp).b_linep).l_fp
            && (*curwp).w_doto == 0 as ::core::ffi::c_int
        {
            break;
        }
    }
    if count == 0 as ::core::ffi::c_int {
        forwchar(FALSE, 1 as ::core::ffi::c_int);
        update(FALSE);
        ttpause();
    }
    (*curwp).w_dotp = oldlp;
    (*curwp).w_doto = oldoff;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn istring(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut status: ::core::ffi::c_int = 0;
    let mut tstring: [::core::ffi::c_char; 1025] = [0; 1025];
    status = mlreplyt(
        b"String to insert<META>: \0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        &raw mut tstring as *mut ::core::ffi::c_char,
        NPAT,
        metac,
    );
    if status != TRUE {
        return status;
    }
    if f == FALSE {
        n = 1 as ::core::ffi::c_int;
    }
    if n < 0 as ::core::ffi::c_int {
        n = -n;
    }
    loop {
        let fresh14 = n;
        n = n - 1;
        if !(fresh14 != 0
            && {
                status = linstr(&raw mut tstring as *mut ::core::ffi::c_char);
                status != 0
            })
        {
            break;
        }
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn ovstring(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut status: ::core::ffi::c_int = 0;
    let mut tstring: [::core::ffi::c_char; 1025] = [0; 1025];
    status = mlreplyt(
        b"String to overwrite<META>: \0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        &raw mut tstring as *mut ::core::ffi::c_char,
        NPAT,
        metac,
    );
    if status != TRUE {
        return status;
    }
    if f == FALSE {
        n = 1 as ::core::ffi::c_int;
    }
    if n < 0 as ::core::ffi::c_int {
        n = -n;
    }
    loop {
        let fresh15 = n;
        n = n - 1;
        if !(fresh15 != 0
            && {
                status = lover(&raw mut tstring as *mut ::core::ffi::c_char);
                status != 0
            })
        {
            break;
        }
    }
    return status;
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
