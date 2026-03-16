extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcat(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    static mut term: *mut terminal;
    fn vttputc(c: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn vttflush();
    fn vtteeol();
    static mut curwp: *mut window;
    static mut curbp: *mut buffer;
    static mut pat: [::core::ffi::c_char; 0];
    static mut tap: [::core::ffi::c_char; 0];
    static mut matchlen: ::core::ffi::c_uint;
    static mut mpresf: ::core::ffi::c_int;
    static mut metac: ::core::ffi::c_int;
    static mut clexec: ::core::ffi::c_int;
    fn backchar(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn update(force: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn movecursor(row: ::core::ffi::c_int, col: ::core::ffi::c_int);
    fn mlerase();
    fn mlwrite(fmt: *const ::core::ffi::c_char, ...);
    fn ectoc(c: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn get1key() -> ::core::ffi::c_int;
    fn getcmd() -> ::core::ffi::c_int;
    fn scanner(
        patrn: *const ::core::ffi::c_char,
        direct: ::core::ffi::c_int,
        beg_or_end: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn eq(bc: ::core::ffi::c_uchar, pc: ::core::ffi::c_uchar) -> ::core::ffi::c_int;
    fn rvstrscpy(
        rvstr: *mut ::core::ffi::c_char,
        str: *mut ::core::ffi::c_char,
        size: ::core::ffi::c_int,
    );
    fn expandp(
        srcstr: *mut ::core::ffi::c_char,
        deststr: *mut ::core::ffi::c_char,
        maxlength: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn lfree(lp: *mut line);
    fn linsert(n: ::core::ffi::c_int, c: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn ldelete(n: ::core::ffi::c_long, kflag: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn lalloc(_: ::core::ffi::c_int) -> *mut line;
    fn completion_init();
    fn completion_update(prefix: *const ::core::ffi::c_char, ctx: completion_context_t);
    fn completion_draw(row: ::core::ffi::c_int, col: ::core::ffi::c_int);
    fn completion_get_selected() -> *const ::core::ffi::c_char;
    fn completion_next();
    fn completion_prev();
    fn completion_hide();
    fn utf8_to_unicode(
        line: *mut ::core::ffi::c_uchar,
        index: ::core::ffi::c_uint,
        len: ::core::ffi::c_uint,
        res: *mut unicode_t,
    ) -> ::core::ffi::c_uint;
    fn unicode_width(c: unicode_t) -> ::core::ffi::c_int;
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
pub type completion_context_t = ::core::ffi::c_uint;
pub const COMPLETION_CONTEXT_PATH: completion_context_t = 1;
pub const COMPLETION_CONTEXT_DEFAULT: completion_context_t = 0;
pub const NPAT: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const CONTROL: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;
pub const SHIFT: ::core::ffi::c_int = 0x8000000 as ::core::ffi::c_int;
pub const SPEC: ::core::ffi::c_uint = 0x80000000 as ::core::ffi::c_uint;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PTBEG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PTEND: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FORWARD: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const REVERSE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WFMOVE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const CMDBUFLEN: ::core::ffi::c_int = 2048 as ::core::ffi::c_int;
pub const IS_ABORT: ::core::ffi::c_int = 7;
pub const IS_BACKSP: ::core::ffi::c_int = 8;
pub const IS_TAB: ::core::ffi::c_int = 9;
pub const IS_NEWLINE: ::core::ffi::c_int = 13;
pub const IS_QUOTE: ::core::ffi::c_int = 17;
pub const IS_REVERSE: ::core::ffi::c_int = 0x12 as ::core::ffi::c_int;
pub const IS_FORWARD: ::core::ffi::c_int = 19;
pub const IS_RUBOUT: ::core::ffi::c_int = 127;
#[no_mangle]
pub static mut minibuf_wp: *mut window = ::core::ptr::null::<window>() as *mut window;
#[no_mangle]
pub static mut minibuf_bp: *mut buffer = ::core::ptr::null::<buffer>() as *mut buffer;
#[no_mangle]
pub unsafe extern "C" fn minibuf_init() {
    let mut lp: *mut line = ::core::ptr::null_mut::<line>();
    if !minibuf_bp.is_null() {
        return;
    }
    minibuf_bp = malloc(::core::mem::size_of::<buffer>() as size_t) as *mut buffer;
    if minibuf_bp.is_null() {
        return;
    }
    lp = lalloc(0 as ::core::ffi::c_int);
    if lp.is_null() {
        free(minibuf_bp as *mut ::core::ffi::c_void);
        minibuf_bp = ::core::ptr::null_mut::<buffer>();
        return;
    }
    (*lp).l_fp = lp;
    (*lp).l_bp = lp;
    (*minibuf_bp).b_bufp = ::core::ptr::null_mut::<buffer>();
    (*minibuf_bp).b_dotp = lp;
    (*minibuf_bp).b_doto = 0 as ::core::ffi::c_int;
    (*minibuf_bp).b_markp = lp;
    (*minibuf_bp).b_marko = 0 as ::core::ffi::c_int;
    (*minibuf_bp).b_linep = lp;
    (*minibuf_bp).b_mode = 0 as ::core::ffi::c_int;
    (*minibuf_bp).b_active = TRUE as ::core::ffi::c_char;
    (*minibuf_bp).b_nwnd = 1 as ::core::ffi::c_char;
    (*minibuf_bp).b_flag = 0 as ::core::ffi::c_char;
    strcpy(
        &raw mut (*minibuf_bp).b_fname as *mut ::core::ffi::c_char,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
    );
    strcpy(
        &raw mut (*minibuf_bp).b_bname as *mut ::core::ffi::c_char,
        b"*minibuf*\0" as *const u8 as *const ::core::ffi::c_char,
    );
    minibuf_wp = malloc(::core::mem::size_of::<window>() as size_t) as *mut window;
    if minibuf_wp.is_null() {
        lfree(lp);
        free(minibuf_bp as *mut ::core::ffi::c_void);
        minibuf_bp = ::core::ptr::null_mut::<buffer>();
        return;
    }
    (*minibuf_wp).w_bufp = minibuf_bp as *mut buffer;
    (*minibuf_wp).w_linep = lp as *mut line;
    (*minibuf_wp).w_dotp = lp;
    (*minibuf_wp).w_doto = 0 as ::core::ffi::c_int;
    (*minibuf_wp).w_markp = lp;
    (*minibuf_wp).w_marko = 0 as ::core::ffi::c_int;
    (*minibuf_wp).w_force = 0 as ::core::ffi::c_char;
    (*minibuf_wp).w_flag = 0 as ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn minibuf_clear() {
    if minibuf_bp.is_null() || minibuf_wp.is_null() {
        return;
    }
    let mut lp: *mut line = (*minibuf_bp).b_linep;
    let mut next: *mut line = ::core::ptr::null_mut::<line>();
    lp = (*lp).l_fp;
    while lp != (*minibuf_bp).b_linep {
        next = (*lp).l_fp;
        (*lp).l_used = 0 as ::core::ffi::c_int;
        lp = next;
    }
    (*minibuf_wp).w_dotp = (*minibuf_bp).b_linep;
    (*minibuf_wp).w_doto = 0 as ::core::ffi::c_int;
    (*minibuf_bp).b_dotp = (*minibuf_bp).b_linep;
    (*minibuf_bp).b_doto = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn minibuf_insert_char(mut c: unicode_t) -> ::core::ffi::c_int {
    let mut save_wp: *mut window = curwp;
    let mut save_bp: *mut buffer = curbp;
    let mut result: ::core::ffi::c_int = 0;
    curwp = minibuf_wp;
    curbp = minibuf_bp;
    result = linsert(1 as ::core::ffi::c_int, c as ::core::ffi::c_int);
    curwp = save_wp;
    curbp = save_bp;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn minibuf_delete_char(
    mut n: ::core::ffi::c_long,
) -> ::core::ffi::c_int {
    let mut save_wp: *mut window = curwp;
    let mut save_bp: *mut buffer = curbp;
    let mut result: ::core::ffi::c_int = 0;
    if (*minibuf_wp).w_doto == 0 as ::core::ffi::c_int
        && (*minibuf_wp).w_dotp == (*minibuf_bp).b_linep
    {
        return FALSE;
    }
    if (*minibuf_wp).w_doto > 0 as ::core::ffi::c_int {
        let mut byte_offset: ::core::ffi::c_int = (*minibuf_wp).w_doto
            - 1 as ::core::ffi::c_int;
        let mut text: *mut ::core::ffi::c_uchar = &raw mut (*(*minibuf_wp).w_dotp).l_text
            as *mut ::core::ffi::c_uchar;
        while byte_offset > 0 as ::core::ffi::c_int
            && is_beginning_utf8(*text.offset(byte_offset as isize)) == 0
        {
            byte_offset -= 1;
        }
        let mut bytes_to_delete: ::core::ffi::c_int = (*minibuf_wp).w_doto - byte_offset;
        curwp = minibuf_wp;
        curbp = minibuf_bp;
        (*minibuf_wp).w_doto = byte_offset;
        result = ldelete(bytes_to_delete as ::core::ffi::c_long, FALSE);
        curwp = save_wp;
        curbp = save_bp;
        return result;
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn minibuf_update(mut prompt: *const ::core::ffi::c_char) {
    let mut col: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_int = 0;
    let mut c: unicode_t = 0;
    let mut text: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
        ::core::ffi::c_uchar,
    >();
    let mut lp: *mut line = ::core::ptr::null_mut::<line>();
    if minibuf_wp.is_null() || minibuf_bp.is_null() {
        return;
    }
    movecursor((*term).t_nrow as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    if !prompt.is_null() {
        let mut p: *mut ::core::ffi::c_uchar = prompt as *mut ::core::ffi::c_uchar;
        let mut plen: ::core::ffi::c_int = strlen(prompt) as ::core::ffi::c_int;
        let mut pi: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while pi < plen {
            let mut uc: unicode_t = 0;
            let mut bytes: ::core::ffi::c_int = utf8_to_unicode(
                p,
                pi as ::core::ffi::c_uint,
                plen as ::core::ffi::c_uint,
                &raw mut uc,
            ) as ::core::ffi::c_int;
            if bytes <= 0 as ::core::ffi::c_int {
                break;
            }
            vttputc(uc as ::core::ffi::c_int);
            col += mystrnlen_raw_w(uc);
            pi += bytes;
        }
    }
    lp = (*minibuf_wp).w_dotp;
    if lp == (*minibuf_bp).b_linep {
        vtteeol();
        vttflush();
        movecursor((*term).t_nrow as ::core::ffi::c_int, col);
        vttflush();
        return;
    }
    text = &raw mut (*lp).l_text as *mut ::core::ffi::c_uchar;
    len = (*lp).l_used;
    i = 0 as ::core::ffi::c_int;
    while i < len && col < (*term).t_ncol as ::core::ffi::c_int - 1 as ::core::ffi::c_int
    {
        let mut bytes_0: ::core::ffi::c_int = utf8_to_unicode(
            text,
            i as ::core::ffi::c_uint,
            len as ::core::ffi::c_uint,
            &raw mut c,
        ) as ::core::ffi::c_int;
        if bytes_0 <= 0 as ::core::ffi::c_int {
            break;
        }
        let mut char_width: ::core::ffi::c_int = mystrnlen_raw_w(c);
        if col + char_width
            >= (*term).t_ncol as ::core::ffi::c_int - 1 as ::core::ffi::c_int
        {
            break;
        }
        vttputc(c as ::core::ffi::c_int);
        col += char_width;
        i += bytes_0;
    }
    vtteeol();
    let mut cursor_col: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !prompt.is_null() {
        cursor_col = strlen(prompt) as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    let mut byte_pos: ::core::ffi::c_int = (*minibuf_wp).w_doto;
    while i < byte_pos && i < len {
        let mut bytes_1: ::core::ffi::c_int = utf8_to_unicode(
            text,
            i as ::core::ffi::c_uint,
            len as ::core::ffi::c_uint,
            &raw mut c,
        ) as ::core::ffi::c_int;
        if bytes_1 <= 0 as ::core::ffi::c_int {
            break;
        }
        cursor_col += mystrnlen_raw_w(c);
        i += bytes_1;
    }
    movecursor((*term).t_nrow as ::core::ffi::c_int, cursor_col);
    vttflush();
}
#[no_mangle]
pub unsafe extern "C" fn minibuf_show(mut msg: *const ::core::ffi::c_char) {
    let mut col: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_int = 0;
    let mut c: unicode_t = 0;
    let mut text: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<
        ::core::ffi::c_uchar,
    >();
    movecursor((*term).t_nrow as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    if msg.is_null() || *msg as ::core::ffi::c_int == '\0' as i32 {
        vtteeol();
        vttflush();
        return;
    }
    text = msg as *mut ::core::ffi::c_uchar;
    len = strlen(msg) as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < len && col < (*term).t_ncol as ::core::ffi::c_int - 1 as ::core::ffi::c_int
    {
        let mut bytes: ::core::ffi::c_int = utf8_to_unicode(
            text,
            i as ::core::ffi::c_uint,
            len as ::core::ffi::c_uint,
            &raw mut c,
        ) as ::core::ffi::c_int;
        if bytes <= 0 as ::core::ffi::c_int {
            vttputc(
                *text.offset(i as isize) as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int,
            );
            col += 1;
            i += 1;
        } else {
            let mut char_width: ::core::ffi::c_int = mystrnlen_raw_w(c);
            if col + char_width
                >= (*term).t_ncol as ::core::ffi::c_int - 1 as ::core::ffi::c_int
            {
                break;
            }
            vttputc(c as ::core::ffi::c_int);
            col += char_width;
            i += bytes;
        }
    }
    vtteeol();
    vttflush();
    mpresf = TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn minibuf_get_text(
    mut dest: *mut ::core::ffi::c_char,
    mut max_len: ::core::ffi::c_int,
) {
    let mut lp: *mut line = ::core::ptr::null_mut::<line>();
    let mut i: ::core::ffi::c_int = 0;
    *dest.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
    if minibuf_bp.is_null() || minibuf_wp.is_null() {
        return;
    }
    lp = (*minibuf_wp).w_dotp;
    if lp == (*minibuf_bp).b_linep {
        return;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*lp).l_used && i < max_len - 1 as ::core::ffi::c_int {
        *dest.offset(i as isize) = *(&raw mut (*lp).l_text as *mut ::core::ffi::c_uchar)
            .offset(i as isize) as ::core::ffi::c_char;
        i += 1;
    }
    *dest.offset(i as isize) = '\0' as i32 as ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn minibuf_input(
    mut prompt: *const ::core::ffi::c_char,
    mut dest: *mut ::core::ffi::c_char,
    mut max_len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = 0;
    let mut current_text: [::core::ffi::c_char; 1024] = [0; 1024];
    minibuf_init();
    completion_init();
    if minibuf_bp.is_null() || minibuf_wp.is_null() {
        mlwrite(
            b"? Cannot initialize minibuffer\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return FALSE;
    }
    minibuf_clear();
    minibuf_update(prompt);
    loop {
        update(FALSE);
        completion_draw((*term).t_nrow as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
        c = getcmd();
        match c {
            IS_ABORT | 268435527 => {
                mlerase();
                completion_hide();
                return FALSE;
            }
            IS_NEWLINE | 10 | 268435533 => {
                minibuf_get_text(dest, max_len);
                minibuf_clear();
                mlerase();
                completion_hide();
                if *dest.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\0' as i32
                {
                    return FALSE;
                }
                return TRUE;
            }
            IS_BACKSP | IS_RUBOUT | 268435528 => {
                minibuf_delete_char(1 as ::core::ffi::c_long);
                minibuf_get_text(
                    &raw mut current_text as *mut ::core::ffi::c_char,
                    NPAT,
                );
                completion_update(
                    &raw mut current_text as *mut ::core::ffi::c_char,
                    COMPLETION_CONTEXT_DEFAULT,
                );
                minibuf_update(prompt);
            }
            27 => {
                mlerase();
                completion_hide();
                return FALSE;
            }
            -2147483583 => {
                completion_prev();
            }
            -2147483583 => {
                completion_next();
            }
            402653249 => {
                let mut selected: *const ::core::ffi::c_char = completion_get_selected();
                if !selected.is_null() {
                    minibuf_clear();
                    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut sel_len: ::core::ffi::c_int = strlen(selected)
                        as ::core::ffi::c_int;
                    while i < sel_len {
                        let mut uc: unicode_t = 0;
                        let mut bytes: ::core::ffi::c_int = utf8_to_unicode(
                            selected as *mut ::core::ffi::c_uchar,
                            i as ::core::ffi::c_uint,
                            sel_len as ::core::ffi::c_uint,
                            &raw mut uc,
                        ) as ::core::ffi::c_int;
                        if bytes <= 0 as ::core::ffi::c_int {
                            break;
                        }
                        minibuf_insert_char(uc);
                        i += bytes;
                    }
                    completion_hide();
                    minibuf_update(prompt);
                }
            }
            _ => {
                if c >= 0x20 as ::core::ffi::c_int && c <= 0x7e as ::core::ffi::c_int
                    || c >= 0xa0 as ::core::ffi::c_int
                        && c <= 0x10ffff as ::core::ffi::c_int
                {
                    minibuf_insert_char(c as unicode_t);
                    minibuf_get_text(
                        &raw mut current_text as *mut ::core::ffi::c_char,
                        NPAT,
                    );
                    completion_update(
                        &raw mut current_text as *mut ::core::ffi::c_char,
                        COMPLETION_CONTEXT_DEFAULT,
                    );
                    minibuf_update(prompt);
                }
            }
        }
    };
}
static mut saved_get_char: Option<unsafe extern "C" fn() -> ::core::ffi::c_int> = None;
static mut eaten_char: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
static mut cmd_buff: [::core::ffi::c_int; 2048] = [0; 2048];
static mut cmd_offset: ::core::ffi::c_int = 0;
static mut cmd_reexecute: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
#[no_mangle]
pub unsafe extern "C" fn uneat() -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = 0;
    (*term).t_getchar = saved_get_char;
    c = eaten_char;
    eaten_char = -(1 as ::core::ffi::c_int);
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn reeat(mut c: ::core::ffi::c_int) {
    if eaten_char != -(1 as ::core::ffi::c_int) {
        return;
    }
    eaten_char = c;
    saved_get_char = (*term).t_getchar;
    (*term).t_getchar = Some(uneat as unsafe extern "C" fn() -> ::core::ffi::c_int)
        as Option<unsafe extern "C" fn() -> ::core::ffi::c_int>;
}
#[no_mangle]
pub unsafe extern "C" fn get_char() -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = 0;
    if cmd_reexecute >= 0 as ::core::ffi::c_int {
        let fresh0 = cmd_reexecute;
        cmd_reexecute = cmd_reexecute + 1;
        c = cmd_buff[fresh0 as usize];
        if c != 0 as ::core::ffi::c_int {
            return c;
        }
    }
    cmd_reexecute = -(1 as ::core::ffi::c_int);
    update(FALSE);
    if cmd_offset >= CMDBUFLEN - 1 as ::core::ffi::c_int {
        mlwrite(b"? command too long\0" as *const u8 as *const ::core::ffi::c_char);
        return metac;
    }
    c = get1key();
    let fresh1 = cmd_offset;
    cmd_offset = cmd_offset + 1;
    cmd_buff[fresh1 as usize] = c;
    cmd_buff[cmd_offset as usize] = '\0' as i32;
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn risearch(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut curline: *mut line = ::core::ptr::null_mut::<line>();
    let mut curoff: ::core::ffi::c_int = 0;
    curline = (*curwp).w_dotp;
    curoff = (*curwp).w_doto;
    backchar(TRUE, 1 as ::core::ffi::c_int);
    if isearch(f, -n) == 0 {
        (*curwp).w_dotp = curline;
        (*curwp).w_doto = curoff;
        (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | WFMOVE)
            as ::core::ffi::c_char;
        update(FALSE);
        mlwrite(b"(search failed)\0" as *const u8 as *const ::core::ffi::c_char);
        matchlen = strlen(&raw mut pat as *mut ::core::ffi::c_char)
            as ::core::ffi::c_uint;
    } else {
        mlerase();
    }
    matchlen = strlen(&raw mut pat as *mut ::core::ffi::c_char) as ::core::ffi::c_uint;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn fisearch(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut curline: *mut line = ::core::ptr::null_mut::<line>();
    let mut curoff: ::core::ffi::c_int = 0;
    curline = (*curwp).w_dotp;
    curoff = (*curwp).w_doto;
    if isearch(f, n) == 0 {
        (*curwp).w_dotp = curline;
        (*curwp).w_doto = curoff;
        (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | WFMOVE)
            as ::core::ffi::c_char;
        update(FALSE);
        mlwrite(b"(search failed)\0" as *const u8 as *const ::core::ffi::c_char);
        matchlen = strlen(&raw mut pat as *mut ::core::ffi::c_char)
            as ::core::ffi::c_uint;
    } else {
        mlerase();
    }
    matchlen = strlen(&raw mut pat as *mut ::core::ffi::c_char) as ::core::ffi::c_uint;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn isearch(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut status: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    let mut expc: ::core::ffi::c_int = 0;
    let mut pat_save: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut curline: *mut line = ::core::ptr::null_mut::<line>();
    let mut curoff: ::core::ffi::c_int = 0;
    let mut init_direction: ::core::ffi::c_int = 0;
    minibuf_init();
    if minibuf_bp.is_null() || minibuf_wp.is_null() {
        mlwrite(
            b"? Cannot initialize minibuffer\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return FALSE;
    }
    cmd_reexecute = -(1 as ::core::ffi::c_int);
    cmd_offset = 0 as ::core::ffi::c_int;
    cmd_buff[0 as ::core::ffi::c_int as usize] = '\0' as i32;
    strncpy(
        &raw mut pat_save as *mut ::core::ffi::c_char,
        &raw mut pat as *mut ::core::ffi::c_char,
        (NPAT - 1 as ::core::ffi::c_int) as size_t,
    );
    curline = (*curwp).w_dotp;
    curoff = (*curwp).w_doto;
    init_direction = n;
    loop {
        minibuf_clear();
        let mut prompt: *const ::core::ffi::c_char = if n < 0 as ::core::ffi::c_int {
            b"I-Search backward: \0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"I-Search forward: \0" as *const u8 as *const ::core::ffi::c_char
        };
        minibuf_update(prompt);
        status = TRUE;
        expc = get_char();
        c = ectoc(expc);
        if c == IS_FORWARD || c == IS_REVERSE || expc == metac {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut pat_len: ::core::ffi::c_int = strlen(
                &raw mut pat as *mut ::core::ffi::c_char,
            ) as ::core::ffi::c_int;
            while i < pat_len {
                let mut uc: unicode_t = 0;
                let mut bytes: ::core::ffi::c_int = utf8_to_unicode(
                    &raw mut pat as *mut ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                    i as ::core::ffi::c_uint,
                    pat_len as ::core::ffi::c_uint,
                    &raw mut uc,
                ) as ::core::ffi::c_int;
                if bytes <= 0 as ::core::ffi::c_int {
                    break;
                }
                minibuf_insert_char(uc);
                i += bytes;
            }
            minibuf_update(prompt);
            if c == IS_REVERSE {
                n = -(1 as ::core::ffi::c_int);
                backchar(TRUE, 1 as ::core::ffi::c_int);
            } else {
                n = 1 as ::core::ffi::c_int;
            }
            status = scanmore(&raw mut pat as *mut ::core::ffi::c_char, n);
            expc = get_char();
            c = ectoc(expc);
        }
        loop {
            if expc == metac {
                status = scanmore(&raw mut pat as *mut ::core::ffi::c_char, n);
                update(FALSE);
                expc = get_char();
                c = ectoc(expc);
            } else {
                match c {
                    IS_ABORT => return FALSE,
                    IS_REVERSE | IS_FORWARD => {
                        if c == IS_REVERSE {
                            n = -(1 as ::core::ffi::c_int);
                        } else {
                            n = 1 as ::core::ffi::c_int;
                        }
                        status = scanmore(&raw mut pat as *mut ::core::ffi::c_char, n);
                        expc = get_char();
                        c = ectoc(expc);
                        continue;
                    }
                    IS_NEWLINE | 10 => {
                        minibuf_get_text(&raw mut pat as *mut ::core::ffi::c_char, NPAT);
                        return TRUE;
                    }
                    IS_QUOTE => {
                        expc = get_char();
                        c = ectoc(expc);
                    }
                    IS_TAB => {}
                    IS_BACKSP | IS_RUBOUT => {
                        if cmd_offset <= 1 as ::core::ffi::c_int {
                            return TRUE;
                        }
                        cmd_offset -= 1;
                        cmd_offset -= 1;
                        cmd_buff[cmd_offset as usize] = '\0' as i32;
                        minibuf_delete_char(1 as ::core::ffi::c_long);
                        minibuf_update(prompt);
                        (*curwp).w_dotp = curline;
                        (*curwp).w_doto = curoff;
                        n = init_direction;
                        strncpy(
                            &raw mut pat as *mut ::core::ffi::c_char,
                            &raw mut pat_save as *mut ::core::ffi::c_char,
                            NPAT as size_t,
                        );
                        cmd_reexecute = 0 as ::core::ffi::c_int;
                        break;
                    }
                    _ => {
                        if c < ' ' as i32 {
                            reeat(c);
                            minibuf_get_text(
                                &raw mut pat as *mut ::core::ffi::c_char,
                                NPAT,
                            );
                            return TRUE;
                        }
                    }
                }
                if c >= 0x20 as ::core::ffi::c_int && c <= 0x7e as ::core::ffi::c_int
                    || c >= 0xa0 as ::core::ffi::c_int
                        && c <= 0x10ffff as ::core::ffi::c_int
                {
                    minibuf_insert_char(c as unicode_t);
                } else if c >= ' ' as i32 && c < 0x7f as ::core::ffi::c_int {
                    minibuf_insert_char(c as unicode_t);
                }
                minibuf_update(prompt);
                minibuf_get_text(&raw mut pat as *mut ::core::ffi::c_char, NPAT);
                if strlen(&raw mut pat as *mut ::core::ffi::c_char)
                    >= (NPAT - 1 as ::core::ffi::c_int) as size_t
                {
                    mlwrite(
                        b"? Search string too long\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    return TRUE;
                }
                if !(status == 0) {
                    if strlen(&raw mut pat as *mut ::core::ffi::c_char) > 0 as size_t
                        && {
                            status = checknext(
                                *(&raw mut pat as *mut ::core::ffi::c_char)
                                    .offset(
                                        strlen(&raw mut pat as *mut ::core::ffi::c_char)
                                            .wrapping_sub(1 as size_t) as isize,
                                    ),
                                &raw mut pat as *mut ::core::ffi::c_char,
                                n,
                            );
                            status == 0
                        }
                    {
                        status = scanmore(&raw mut pat as *mut ::core::ffi::c_char, n);
                    }
                }
                expc = get_char();
                c = ectoc(expc);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn checknext(
    mut chr: ::core::ffi::c_char,
    mut patrn: *mut ::core::ffi::c_char,
    mut dir: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut curline: *mut line = ::core::ptr::null_mut::<line>();
    let mut curoff: ::core::ffi::c_int = 0;
    let mut buffchar: ::core::ffi::c_int = 0;
    let mut status: ::core::ffi::c_int = 0;
    curline = (*curwp).w_dotp;
    curoff = (*curwp).w_doto;
    if dir > 0 as ::core::ffi::c_int {
        if curoff == (*curline).l_used {
            curline = (*curline).l_fp;
            if curline == (*curbp).b_linep {
                return FALSE;
            }
            curoff = 0 as ::core::ffi::c_int;
            buffchar = '\n' as i32;
        } else {
            let fresh2 = curoff;
            curoff = curoff + 1;
            buffchar = *(&raw mut (*curline).l_text as *mut ::core::ffi::c_uchar)
                .offset(fresh2 as isize) as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_int;
        }
        status = eq(buffchar as ::core::ffi::c_uchar, chr as ::core::ffi::c_uchar);
        if status != 0 as ::core::ffi::c_int {
            (*curwp).w_dotp = curline;
            (*curwp).w_doto = curoff;
            (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | WFMOVE)
                as ::core::ffi::c_char;
        }
        return status;
    } else {
        return match_pat(patrn)
    };
}
#[no_mangle]
pub unsafe extern "C" fn scanmore(
    mut patrn: *mut ::core::ffi::c_char,
    mut dir: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut sts: ::core::ffi::c_int = 0;
    if dir < 0 as ::core::ffi::c_int {
        rvstrscpy(&raw mut tap as *mut ::core::ffi::c_char, patrn, NPAT);
        sts = scanner(&raw mut tap as *mut ::core::ffi::c_char, REVERSE, PTBEG);
    } else {
        sts = scanner(patrn, FORWARD, PTEND);
    }
    sts == 0;
    return sts;
}
#[no_mangle]
pub unsafe extern "C" fn match_pat(
    mut patrn: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut buffchar: ::core::ffi::c_int = 0;
    let mut curline: *mut line = ::core::ptr::null_mut::<line>();
    let mut curoff: ::core::ffi::c_int = 0;
    curline = (*curwp).w_dotp;
    curoff = (*curwp).w_doto;
    i = 0 as ::core::ffi::c_int;
    while (i as size_t) < strlen(patrn) {
        if curoff == (*curline).l_used {
            curline = (*curline).l_fp;
            curoff = 0 as ::core::ffi::c_int;
            if curline == (*curbp).b_linep {
                return FALSE;
            }
            buffchar = '\n' as i32;
        } else {
            let fresh3 = curoff;
            curoff = curoff + 1;
            buffchar = *(&raw mut (*curline).l_text as *mut ::core::ffi::c_uchar)
                .offset(fresh3 as isize) as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_int;
        }
        if eq(
            buffchar as ::core::ffi::c_uchar,
            *patrn.offset(i as isize) as ::core::ffi::c_uchar,
        ) == 0
        {
            return FALSE;
        }
        i += 1;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn promptpattern(
    mut prompt: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut tpat: [::core::ffi::c_char; 1044] = [0; 1044];
    strcpy(&raw mut tpat as *mut ::core::ffi::c_char, prompt);
    strcat(
        &raw mut tpat as *mut ::core::ffi::c_char,
        b" (\0" as *const u8 as *const ::core::ffi::c_char,
    );
    expandp(
        &raw mut pat as *mut ::core::ffi::c_char,
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
    if clexec == 0 {
        mlwrite(&raw mut tpat as *mut ::core::ffi::c_char);
    }
    return strlen(&raw mut tpat as *mut ::core::ffi::c_char) as ::core::ffi::c_int;
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
unsafe extern "C" fn is_beginning_utf8(
    mut c: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    return (c as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
        != 0x80 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
