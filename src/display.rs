extern "C" {
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn write(
        __fd: ::core::ffi::c_int,
        __buf: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ssize_t;
    static mut term: *mut terminal;
    static mut tab_width: ::core::ffi::c_int;
    fn vttopen();
    fn vttclose();
    fn vttkopen();
    fn vttkclose();
    fn vttputc(c: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn vttflush();
    fn vttmove(row: ::core::ffi::c_int, col: ::core::ffi::c_int);
    fn vtteeol();
    fn vttrev(state: ::core::ffi::c_int);
    fn vttsetcolors(fg: ::core::ffi::c_int, bg: ::core::ffi::c_int);
    fn vttsetattrs(
        bold: ::core::ffi::c_int,
        underline: ::core::ffi::c_int,
        italic: ::core::ffi::c_int,
    );
    static mut eolexist: ::core::ffi::c_int;
    static mut currow: ::core::ffi::c_int;
    static mut curcol: ::core::ffi::c_int;
    static mut curwp: *mut window;
    static mut curbp: *mut buffer;
    static mut kbdmode: ::core::ffi::c_int;
    static mut discmd: ::core::ffi::c_int;
    static mut ttrow: ::core::ffi::c_int;
    static mut ttcol: ::core::ffi::c_int;
    static mut taboff: ::core::ffi::c_int;
    static mut vtrow: ::core::ffi::c_int;
    static mut vtcol: ::core::ffi::c_int;
    static mut lbound: ::core::ffi::c_int;
    static mut sgarbf: ::core::ffi::c_int;
    static mut mpresf: ::core::ffi::c_int;
    static mut nanox_cfg: nanox_config;
    static mut should_redraw_underbar: bool;
    fn nanox_lamp_label() -> *const ::core::ffi::c_char;
    fn nanox_text_rows() -> ::core::ffi::c_int;
    fn nanox_text_cols() -> ::core::ffi::c_int;
    fn nanox_hint_top_row() -> ::core::ffi::c_int;
    fn nanox_hint_bottom_row() -> ::core::ffi::c_int;
    fn nanox_notify_message(text: *const ::core::ffi::c_char);
    fn nanox_message_prefix(
        input: *const ::core::ffi::c_char,
        output: *mut ::core::ffi::c_char,
        outsz: size_t,
    );
    fn newsize(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn newwidth(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn spellcheck(word: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn command_mode_block_selection_contains(
        lp: *mut line,
        col_start: ::core::ffi::c_int,
        col_end: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn colorscheme_get(id: HighlightStyleID) -> HighlightStyle;
    fn highlight_get_profile(
        filename: *const ::core::ffi::c_char,
    ) -> *const HighlightProfile;
    fn highlight_line(
        text: *const ::core::ffi::c_char,
        len: ::core::ffi::c_int,
        start: HighlightState,
        profile: *const HighlightProfile,
        out: *mut SpanVec,
        end: *mut HighlightState,
    );
    fn highlight_is_enabled() -> bool;
    fn span_vec_free(vec: *mut SpanVec);
    fn highlight_find_colors(
        text: *const ::core::ffi::c_char,
        len: ::core::ffi::c_int,
        colors: *mut ColorInfo,
        max_colors: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn lchange(flag: ::core::ffi::c_int);
    fn xmalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn signal(__sig: ::core::ffi::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn ioctl(
        __fd: ::core::ffi::c_int,
        __request: ::core::ffi::c_ulong,
        ...
    ) -> ::core::ffi::c_int;
    fn utf8_to_unicode(
        line: *mut ::core::ffi::c_uchar,
        index: ::core::ffi::c_uint,
        len: ::core::ffi::c_uint,
        res: *mut unicode_t,
    ) -> ::core::ffi::c_uint;
    fn unicode_width(c: unicode_t) -> ::core::ffi::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type size_t = usize;
pub type __gnuc_va_list = __builtin_va_list;
pub type __ssize_t = ::core::ffi::c_long;
pub type va_list = __gnuc_va_list;
pub type ssize_t = __ssize_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct video {
    pub v_flag: ::core::ffi::c_int,
    pub v_text: [video_cell; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct video_cell {
    pub ch: unicode_t,
    pub fg: ::core::ffi::c_int,
    pub bg: ::core::ffi::c_int,
    pub bold: bool,
    pub underline: bool,
    pub italic: bool,
}
pub type unicode_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HighlightStyle {
    pub fg: ::core::ffi::c_int,
    pub bg: ::core::ffi::c_int,
    pub bold: bool,
    pub underline: bool,
    pub italic: bool,
}
pub type HighlightStyleID = ::core::ffi::c_uint;
pub const HL_COUNT: HighlightStyleID = 23;
pub const HL_LINENUM: HighlightStyleID = 22;
pub const HL_MD_UNDERLINE: HighlightStyleID = 21;
pub const HL_MD_ITALIC: HighlightStyleID = 20;
pub const HL_MD_BOLD: HighlightStyleID = 19;
pub const HL_HEADER: HighlightStyleID = 18;
pub const HL_SELECTION: HighlightStyleID = 17;
pub const HL_NOTICE: HighlightStyleID = 16;
pub const HL_ERROR: HighlightStyleID = 15;
pub const HL_TERNARY: HighlightStyleID = 14;
pub const HL_CONTROL: HighlightStyleID = 13;
pub const HL_ESCAPE: HighlightStyleID = 12;
pub const HL_RETURN: HighlightStyleID = 11;
pub const HL_PREPROC: HighlightStyleID = 10;
pub const HL_FLOW: HighlightStyleID = 9;
pub const HL_FUNCTION: HighlightStyleID = 8;
pub const HL_TYPE: HighlightStyleID = 7;
pub const HL_KEYWORD: HighlightStyleID = 6;
pub const HL_OPERATOR: HighlightStyleID = 5;
pub const HL_BRACKET: HighlightStyleID = 4;
pub const HL_NUMBER: HighlightStyleID = 3;
pub const HL_STRING: HighlightStyleID = 2;
pub const HL_COMMENT: HighlightStyleID = 1;
pub const HL_NORMAL: HighlightStyleID = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColorInfo {
    pub start: ::core::ffi::c_int,
    pub end: ::core::ffi::c_int,
    pub r: ::core::ffi::c_int,
    pub g: ::core::ffi::c_int,
    pub b: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SpanVec {
    pub spans: [Span; 256],
    pub heap_spans: *mut Span,
    pub count: ::core::ffi::c_int,
    pub capacity: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Span {
    pub start: ::core::ffi::c_int,
    pub end: ::core::ffi::c_int,
    pub style: HighlightStyleID,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HighlightProfile {
    pub name: [::core::ffi::c_char; 1232],
    pub extensions: [[::core::ffi::c_char; 64]; 1232],
    pub ext_count: ::core::ffi::c_int,
    pub file_match_patterns: [[::core::ffi::c_char; 128]; 16],
    pub file_match_count: ::core::ffi::c_int,
    pub line_comments: [[::core::ffi::c_char; 64]; 32],
    pub line_comment_count: ::core::ffi::c_int,
    pub block_comments: [BlockCommentPair; 32],
    pub block_comment_count: ::core::ffi::c_int,
    pub string_delims: [::core::ffi::c_char; 32],
    pub keywords: [[::core::ffi::c_char; 64]; 1024],
    pub keyword_count: ::core::ffi::c_int,
    pub type_keywords: [[::core::ffi::c_char; 64]; 1024],
    pub type_keyword_count: ::core::ffi::c_int,
    pub flow_keywords: [[::core::ffi::c_char; 64]; 1024],
    pub flow_keyword_count: ::core::ffi::c_int,
    pub preproc_keywords: [[::core::ffi::c_char; 64]; 128],
    pub preproc_keyword_count: ::core::ffi::c_int,
    pub return_keywords: [[::core::ffi::c_char; 64]; 32],
    pub return_keyword_count: ::core::ffi::c_int,
    pub enable_triple_quotes: bool,
    pub enable_number_highlight: bool,
    pub enable_bracket_highlight: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockCommentPair {
    pub start: [::core::ffi::c_char; 64],
    pub end: [::core::ffi::c_char; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mlbuf {
    pub buf: *mut ::core::ffi::c_char,
    pub len: size_t,
    pub cap: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: ::core::ffi::c_ushort,
    pub ws_col: ::core::ffi::c_ushort,
    pub ws_xpixel: ::core::ffi::c_ushort,
    pub ws_ypixel: ::core::ffi::c_ushort,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SIGWINCH: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const TIOCGWINSZ: ::core::ffi::c_int = 0x5413 as ::core::ffi::c_int;
pub const MAXCOL: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PLAY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WFFORCE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const WFMOVE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const WFEDIT: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const WFHARD: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const WFMODE: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const BFCHG: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MDSOFTWRAP: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MDSPELL: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
#[no_mangle]
pub static mut vscreen: *mut *mut video = ::core::ptr::null::<*mut video>()
    as *mut *mut video;
unsafe extern "C" fn get_gutter_width() -> ::core::ffi::c_int {
    return if !nanox_cfg.nonr {
        6 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn render_gutter(
    mut row: ::core::ffi::c_int,
    mut lnum: ::core::ffi::c_int,
) {
    if nanox_cfg.nonr {
        return;
    }
    let mut buf: [::core::ffi::c_char; 10] = [0; 10];
    if lnum > 0 as ::core::ffi::c_int {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t,
            b"%5d\0" as *const u8 as *const ::core::ffi::c_char,
            lnum,
        );
    } else {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t,
            b"     \0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut num_style: HighlightStyle = colorscheme_get(HL_LINENUM);
    let mut vcp: *mut video_cell = &raw mut (**vscreen.offset(row as isize)).v_text
        as *mut video_cell;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 5 as ::core::ffi::c_int {
        (*vcp.offset(i as isize)).ch = buf[i as usize] as unicode_t;
        (*vcp.offset(i as isize)).fg = num_style.fg;
        (*vcp.offset(i as isize)).bg = num_style.bg;
        (*vcp.offset(i as isize)).bold = num_style.bold;
        (*vcp.offset(i as isize)).underline = num_style.underline;
        (*vcp.offset(i as isize)).italic = num_style.italic;
        i += 1;
    }
    (*vcp.offset(5 as ::core::ffi::c_int as isize)).ch = 0x2502 as unicode_t;
    (*vcp.offset(5 as ::core::ffi::c_int as isize)).fg = num_style.fg;
    (*vcp.offset(5 as ::core::ffi::c_int as isize)).bg = num_style.bg;
    (*vcp.offset(5 as ::core::ffi::c_int as isize)).bold = num_style.bold;
    (*vcp.offset(5 as ::core::ffi::c_int as isize)).underline = num_style.underline;
    (*vcp.offset(5 as ::core::ffi::c_int as isize)).italic = num_style.italic;
}
static mut current_color_fg: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
static mut current_color_bg: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
static mut current_color_bold: bool = false_0 != 0;
static mut current_color_underline: bool = false_0 != 0;
static mut current_color_italic: bool = false_0 != 0;
static mut vt_margin_left: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut displaying: ::core::ffi::c_int = TRUE;
#[no_mangle]
pub static mut chg_width: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut chg_height: ::core::ffi::c_int = 0;
unsafe extern "C" fn get_char_width(
    mut c: unicode_t,
    mut col: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if c == '\t' as i32 as unicode_t {
        let mut rel_col: ::core::ffi::c_int = col - vt_margin_left;
        return tab_width + 1 as ::core::ffi::c_int - (rel_col + taboff & tab_width);
    }
    if c < 0x20 as unicode_t || c == 0x7f as unicode_t {
        return 2 as ::core::ffi::c_int;
    }
    return mystrnlen_raw_w(c);
}
unsafe extern "C" fn get_line_height(mut lp: *mut line) -> ::core::ffi::c_int {
    if lp == (*curbp).b_linep {
        return 0 as ::core::ffi::c_int;
    }
    let mut len: ::core::ffi::c_int = (*lp).l_used;
    let mut col: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut height: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < len {
        let mut c: unicode_t = 0;
        let mut bytes: ::core::ffi::c_int = utf8_to_unicode(
            &raw mut (*lp).l_text as *mut ::core::ffi::c_uchar,
            i as ::core::ffi::c_uint,
            len as ::core::ffi::c_uint,
            &raw mut c,
        ) as ::core::ffi::c_int;
        let mut w: ::core::ffi::c_int = get_char_width(c, col);
        if col + w > nanox_text_cols() {
            height += 1;
            col = 4 as ::core::ffi::c_int;
            w = get_char_width(c, col);
        }
        col += w;
        i += bytes;
    }
    return height;
}
unsafe extern "C" fn mlbuf_init(
    mut dest: *mut mlbuf,
    mut storage: *mut ::core::ffi::c_char,
    mut size: size_t,
) {
    (*dest).buf = storage;
    (*dest).len = 0 as size_t;
    (*dest).cap = size;
    if size != 0 {
        *(*dest).buf.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
    }
}
unsafe extern "C" fn mlbuf_putc(mut dest: *mut mlbuf, mut ch: ::core::ffi::c_int) {
    if (*dest).len.wrapping_add(1 as size_t) >= (*dest).cap {
        return;
    }
    let fresh4 = (*dest).len;
    (*dest).len = (*dest).len.wrapping_add(1);
    *(*dest).buf.offset(fresh4 as isize) = ch as ::core::ffi::c_char;
    *(*dest).buf.offset((*dest).len as isize) = 0 as ::core::ffi::c_char;
}
unsafe extern "C" fn mlbuf_puts(
    mut dest: *mut mlbuf,
    mut text: *const ::core::ffi::c_char,
) {
    while !text.is_null() && *text as ::core::ffi::c_int != 0 {
        let fresh5 = text;
        text = text.offset(1);
        mlbuf_putc(dest, *fresh5 as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn draw_hint_row(
    mut row: ::core::ffi::c_int,
    mut left: *const ::core::ffi::c_char,
    mut status: *const ::core::ffi::c_char,
) {
    let mut width: ::core::ffi::c_int = (*term).t_ncol as ::core::ffi::c_int;
    if width > MAXCOL {
        width = MAXCOL;
    }
    if width <= 0 as ::core::ffi::c_int {
        return;
    }
    vtmove(row, 0 as ::core::ffi::c_int);
    let mut display_col: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !left.is_null() && *left as ::core::ffi::c_int != 0 {
        let mut left_len: ::core::ffi::c_int = strlen(left) as ::core::ffi::c_int;
        let mut left_i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while left_i < left_len && display_col < width {
            let mut c: unicode_t = 0;
            let mut bytes: ::core::ffi::c_int = utf8_to_unicode(
                left as *mut ::core::ffi::c_uchar,
                left_i as ::core::ffi::c_uint,
                left_len as ::core::ffi::c_uint,
                &raw mut c,
            ) as ::core::ffi::c_int;
            if bytes <= 0 as ::core::ffi::c_int {
                break;
            }
            let mut char_width: ::core::ffi::c_int = mystrnlen_raw_w(c);
            if display_col + char_width > width {
                break;
            }
            vtputc(c as ::core::ffi::c_int);
            display_col += char_width;
            left_i += bytes;
        }
    }
    let mut status_width: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !status.is_null() && *status as ::core::ffi::c_int != 0 {
        status_width = utf8_display_width(status, strlen(status) as ::core::ffi::c_int);
    }
    let mut status_start_col: ::core::ffi::c_int = width - status_width;
    if status_start_col < display_col {
        status_start_col = display_col;
    }
    while display_col < status_start_col {
        vtputc(' ' as i32);
        display_col += 1;
    }
    if !status.is_null() && *status as ::core::ffi::c_int != 0 && display_col < width {
        let mut status_len: ::core::ffi::c_int = strlen(status) as ::core::ffi::c_int;
        let mut status_i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while status_i < status_len && display_col < width {
            let mut c_0: unicode_t = 0;
            let mut bytes_0: ::core::ffi::c_int = utf8_to_unicode(
                status as *mut ::core::ffi::c_uchar,
                status_i as ::core::ffi::c_uint,
                status_len as ::core::ffi::c_uint,
                &raw mut c_0,
            ) as ::core::ffi::c_int;
            if bytes_0 <= 0 as ::core::ffi::c_int {
                break;
            }
            let mut char_width_0: ::core::ffi::c_int = mystrnlen_raw_w(c_0);
            if display_col + char_width_0 > width {
                break;
            }
            vtputc(c_0 as ::core::ffi::c_int);
            display_col += char_width_0;
            status_i += bytes_0;
        }
    }
    vteeol();
}
unsafe extern "C" fn window_line_number(mut wp: *mut window) -> ::core::ffi::c_int {
    let mut lp: *mut line = (*(*(*wp).w_bufp).b_linep).l_fp;
    let mut count: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while lp != (*(*wp).w_bufp).b_linep {
        if lp == (*wp).w_dotp {
            break;
        }
        count += 1;
        lp = (*lp).l_fp;
    }
    return count;
}
unsafe extern "C" fn get_line_num(
    mut bp: *mut buffer,
    mut target: *mut line,
) -> ::core::ffi::c_int {
    let mut lp: *mut line = (*(*bp).b_linep).l_fp;
    let mut count: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while lp != (*bp).b_linep {
        if lp == target {
            break;
        }
        count += 1;
        lp = (*lp).l_fp;
    }
    return count;
}
unsafe extern "C" fn window_column_number(mut wp: *mut window) -> ::core::ffi::c_int {
    let mut lp: *mut line = (*wp).w_dotp;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut len: ::core::ffi::c_int = (*lp).l_used;
    let mut col: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*wp).w_doto {
        let mut c: unicode_t = 0;
        let mut bytes: ::core::ffi::c_int = utf8_to_unicode(
            &raw mut (*lp).l_text as *mut ::core::ffi::c_uchar,
            i as ::core::ffi::c_uint,
            len as ::core::ffi::c_uint,
            &raw mut c,
        ) as ::core::ffi::c_int;
        i += bytes;
        col = next_column(col, c, tab_width);
    }
    return col + 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn vtinit() {
    let mut i: ::core::ffi::c_int = 0;
    let mut vp: *mut video = ::core::ptr::null_mut::<video>();
    vttopen();
    vttkopen();
    vttrev(FALSE);
    vscreen = xmalloc(
        ((*term).t_mrow as size_t)
            .wrapping_mul(::core::mem::size_of::<*mut video>() as size_t),
    ) as *mut *mut video;
    memset(
        vscreen as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ((*term).t_mrow as size_t)
            .wrapping_mul(::core::mem::size_of::<*mut video>() as size_t),
    );
    current_color_fg = -(1 as ::core::ffi::c_int);
    current_color_bg = -(1 as ::core::ffi::c_int);
    current_color_bold = false_0 != 0;
    current_color_underline = false_0 != 0;
    current_color_italic = false_0 != 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*term).t_mrow as ::core::ffi::c_int {
        vp = xmalloc(
            (::core::mem::size_of::<video>() as size_t)
                .wrapping_add(
                    ((*term).t_mcol as size_t)
                        .wrapping_mul(::core::mem::size_of::<video_cell>() as size_t),
                ),
        ) as *mut video;
        memset(
            vp as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<video>() as size_t)
                .wrapping_add(
                    ((*term).t_mcol as size_t)
                        .wrapping_mul(::core::mem::size_of::<video_cell>() as size_t),
                ),
        );
        (*vp).v_flag = 0 as ::core::ffi::c_int;
        let ref mut fresh0 = *vscreen.offset(i as isize);
        *fresh0 = vp;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn vttidy() {
    let mut i: ::core::ffi::c_int = 0;
    mlerase();
    movecursor((*term).t_nrow as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    vttflush();
    vttclose();
    vttkclose();
    if !vscreen.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < (*term).t_mrow as ::core::ffi::c_int {
            if !(*vscreen.offset(i as isize)).is_null() {
                free(*vscreen.offset(i as isize) as *mut ::core::ffi::c_void);
            }
            i += 1;
        }
        free(vscreen as *mut ::core::ffi::c_void);
        vscreen = ::core::ptr::null_mut::<*mut video>();
    }
    write(
        1 as ::core::ffi::c_int,
        b"\r\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        1 as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn vtmove(
    mut row: ::core::ffi::c_int,
    mut col: ::core::ffi::c_int,
) {
    vtrow = row;
    vtcol = col + vt_margin_left;
}
#[no_mangle]
pub unsafe extern "C" fn vtputc(mut c: ::core::ffi::c_int) {
    let mut vp: *mut video = ::core::ptr::null_mut::<video>();
    let mut char_width: ::core::ffi::c_int = 0;
    if c == '\t' as i32 {
        loop {
            vtputc(' ' as i32);
            if !(vtcol - vt_margin_left + taboff & tab_width != 0 as ::core::ffi::c_int)
            {
                break;
            }
        }
        return;
    }
    if c < 0x20 as ::core::ffi::c_int {
        vtputc('^' as i32);
        vtputc(c ^ 0x40 as ::core::ffi::c_int);
        return;
    }
    if c == 0x7f as ::core::ffi::c_int {
        vtputc('^' as i32);
        vtputc('?' as i32);
        return;
    }
    char_width = mystrnlen_raw_w(c as unicode_t);
    if vtcol + char_width > (*term).t_ncol as ::core::ffi::c_int {
        if vtrow < nanox_text_rows() - 1 as ::core::ffi::c_int {
            vtrow += 1;
            vtcol = vt_margin_left;
            (**vscreen.offset(vtrow as isize)).v_flag |= VFCHG;
            render_gutter(vtrow, 0 as ::core::ffi::c_int);
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < 4 as ::core::ffi::c_int {
                vtputc(' ' as i32);
                i += 1;
            }
        } else {
            (*(&raw mut (**vscreen.offset(vtrow as isize)).v_text as *mut video_cell)
                .offset(
                    ((*term).t_ncol as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        as isize,
                ))
                .ch = '$' as i32 as unicode_t;
            vtcol += char_width;
            return;
        }
    }
    vp = *vscreen.offset(vtrow as isize);
    if vtcol >= 0 as ::core::ffi::c_int {
        let mut i_0: ::core::ffi::c_int = 0;
        i_0 = 0 as ::core::ffi::c_int;
        while i_0 < char_width {
            if vtcol + i_0 < (*term).t_ncol as ::core::ffi::c_int {
                (*(&raw mut (*vp).v_text as *mut video_cell)
                    .offset((vtcol + i_0) as isize))
                    .ch = (if i_0 == 0 as ::core::ffi::c_int {
                    c
                } else {
                    0 as ::core::ffi::c_int
                }) as unicode_t;
                (*(&raw mut (*vp).v_text as *mut video_cell)
                    .offset((vtcol + i_0) as isize))
                    .fg = current_color_fg;
                (*(&raw mut (*vp).v_text as *mut video_cell)
                    .offset((vtcol + i_0) as isize))
                    .bg = current_color_bg;
                (*(&raw mut (*vp).v_text as *mut video_cell)
                    .offset((vtcol + i_0) as isize))
                    .bold = current_color_bold;
                (*(&raw mut (*vp).v_text as *mut video_cell)
                    .offset((vtcol + i_0) as isize))
                    .underline = current_color_underline;
                (*(&raw mut (*vp).v_text as *mut video_cell)
                    .offset((vtcol + i_0) as isize))
                    .italic = current_color_italic;
            }
            i_0 += 1;
        }
        vtcol += char_width;
    }
}
#[no_mangle]
pub unsafe extern "C" fn vteeol() {
    let mut vcp: *mut video_cell = &raw mut (**vscreen.offset(vtrow as isize)).v_text
        as *mut video_cell;
    if vtcol < 0 as ::core::ffi::c_int {
        vtcol = 0 as ::core::ffi::c_int;
    }
    if vtcol > (*term).t_ncol as ::core::ffi::c_int {
        vtcol = (*term).t_ncol as ::core::ffi::c_int;
    }
    while vtcol < (*term).t_ncol as ::core::ffi::c_int {
        (*vcp.offset(vtcol as isize)).ch = ' ' as i32 as unicode_t;
        (*vcp.offset(vtcol as isize)).fg = current_color_fg;
        (*vcp.offset(vtcol as isize)).bg = current_color_bg;
        (*vcp.offset(vtcol as isize)).bold = current_color_bold;
        (*vcp.offset(vtcol as isize)).underline = current_color_underline;
        (*vcp.offset(vtcol as isize)).italic = current_color_italic;
        vtcol += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn upscreen(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    update(TRUE);
    return TRUE;
}
unsafe extern "C" fn update_syntax_highlighting(mut bp: *mut buffer) {
    if !highlight_is_enabled() {
        return;
    }
    let mut fname: *const ::core::ffi::c_char = if (*bp)
        .b_fname[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 0
    {
        &raw mut (*bp).b_fname as *mut ::core::ffi::c_char
    } else {
        &raw mut (*bp).b_bname as *mut ::core::ffi::c_char
    };
    let mut profile: *const HighlightProfile = highlight_get_profile(fname);
    if profile.is_null() {
        return;
    }
    let mut lp: *mut line = (*(*bp).b_linep).l_fp;
    let mut current_state: HighlightState = HighlightState {
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
    while lp != (*bp).b_linep {
        if memcmp(
            &raw mut (*lp).hl_start_state as *const ::core::ffi::c_void,
            &raw mut current_state as *const ::core::ffi::c_void,
            ::core::mem::size_of::<HighlightState>() as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            let mut computed_end: HighlightState = HighlightState {
                stack: [HighlightStackEntry {
                    state: HS_NORMAL,
                    sub_id: 0,
                    string_delim: 0,
                }; 8],
                depth: 0,
            };
            highlight_line(
                &raw mut (*lp).l_text as *mut ::core::ffi::c_uchar
                    as *const ::core::ffi::c_char,
                (*lp).l_used,
                current_state,
                profile,
                ::core::ptr::null_mut::<SpanVec>(),
                &raw mut computed_end,
            );
            if memcmp(
                &raw mut (*lp).hl_end_state as *const ::core::ffi::c_void,
                &raw mut computed_end as *const ::core::ffi::c_void,
                ::core::mem::size_of::<HighlightState>() as size_t,
            ) == 0 as ::core::ffi::c_int
            {
                break;
            }
            (*lp).hl_end_state = computed_end;
            current_state = computed_end;
        } else {
            (*lp).hl_start_state = current_state;
            let mut computed_end_0: HighlightState = HighlightState {
                stack: [HighlightStackEntry {
                    state: HS_NORMAL,
                    sub_id: 0,
                    string_delim: 0,
                }; 8],
                depth: 0,
            };
            highlight_line(
                &raw mut (*lp).l_text as *mut ::core::ffi::c_uchar
                    as *const ::core::ffi::c_char,
                (*lp).l_used,
                current_state,
                profile,
                ::core::ptr::null_mut::<SpanVec>(),
                &raw mut computed_end_0,
            );
            (*lp).hl_end_state = computed_end_0;
            current_state = computed_end_0;
        }
        lp = (*lp).l_fp;
    }
}
#[no_mangle]
pub unsafe extern "C" fn update(mut force: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut wp: *mut window = ::core::ptr::null_mut::<window>();
    if force == FALSE && kbdmode == PLAY {
        return TRUE;
    }
    displaying = TRUE;
    let mut normal: HighlightStyle = colorscheme_get(HL_NORMAL);
    current_color_fg = normal.fg;
    current_color_bg = normal.bg;
    current_color_bold = normal.bold;
    current_color_underline = normal.underline;
    current_color_italic = normal.italic;
    wp = curwp;
    if (*wp).w_flag != 0 {
        if (*wp).w_flag as ::core::ffi::c_int & WFHARD != 0 {
            update_syntax_highlighting((*wp).w_bufp as *mut buffer);
        }
        reframe(wp);
        if (*wp).w_flag as ::core::ffi::c_int & !WFMODE == WFEDIT {
            updone(wp);
        } else if (*wp).w_flag as ::core::ffi::c_int & !WFMOVE != 0 {
            updall(wp);
        }
        if (*wp).w_flag as ::core::ffi::c_int & WFMODE != 0 {
            modeline(wp);
        }
        (*wp).w_flag = 0 as ::core::ffi::c_char;
        (*wp).w_force = 0 as ::core::ffi::c_char;
    }
    if should_redraw_underbar as ::core::ffi::c_int != 0 && !curwp.is_null() {
        modeline(curwp);
        should_redraw_underbar = false_0 != 0;
    }
    updpos();
    if sgarbf != FALSE {
        updgar();
    }
    updupd(force);
    movecursor(currow, curcol + get_gutter_width() - lbound);
    vttflush();
    displaying = FALSE;
    while chg_width != 0 || chg_height != 0 {
        newscreensize(chg_height, chg_width);
    }
    return TRUE;
}
unsafe extern "C" fn reframe(mut wp: *mut window) -> ::core::ffi::c_int {
    let mut lp: *mut line = ::core::ptr::null_mut::<line>();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rows: ::core::ffi::c_int = nanox_text_rows();
    if (*wp).w_flag as ::core::ffi::c_int & WFFORCE == 0 as ::core::ffi::c_int {
        lp = (*wp).w_linep as *mut line;
        let mut vrow: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while vrow < rows && lp != (*(*wp).w_bufp).b_linep {
            if lp == (*wp).w_dotp {
                let mut dot_vrow: ::core::ffi::c_int = vrow;
                let mut col: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut char_idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while char_idx < (*wp).w_doto {
                    let mut c: unicode_t = 0;
                    let mut bytes: ::core::ffi::c_int = utf8_to_unicode(
                        &raw mut (*lp).l_text as *mut ::core::ffi::c_uchar,
                        char_idx as ::core::ffi::c_uint,
                        (*wp).w_doto as ::core::ffi::c_uint,
                        &raw mut c,
                    ) as ::core::ffi::c_int;
                    let mut w: ::core::ffi::c_int = get_char_width(c, col);
                    if col + w > nanox_text_cols() {
                        dot_vrow += 1;
                        col = 4 as ::core::ffi::c_int;
                        w = get_char_width(c, col);
                    }
                    col += w;
                    char_idx += bytes;
                }
                if !(dot_vrow < rows) {
                    break;
                }
                return TRUE;
            } else {
                vrow += get_line_height(lp);
                lp = (*lp).l_fp;
            }
        }
    }
    if (*wp).w_flag as ::core::ffi::c_int & WFFORCE != 0 {
        i = (*wp).w_force as ::core::ffi::c_int;
    } else {
        i = rows / 2 as ::core::ffi::c_int;
    }
    (*wp).w_linep = (*wp).w_dotp as *mut line;
    while i > 0 as ::core::ffi::c_int && (*(*wp).w_linep).l_bp != (*(*wp).w_bufp).b_linep
    {
        (*wp).w_linep = (*(*wp).w_linep).l_bp as *mut line;
        i -= 1;
    }
    (*wp).w_flag = ((*wp).w_flag as ::core::ffi::c_int | WFHARD) as ::core::ffi::c_char;
    (*wp).w_flag = ((*wp).w_flag as ::core::ffi::c_int & !WFFORCE)
        as ::core::ffi::c_char;
    return TRUE;
}
unsafe extern "C" fn show_line(mut wp: *mut window, mut lp: *mut line) {
    let mut len: ::core::ffi::c_int = (*lp).l_used;
    let mut spans: SpanVec = SpanVec {
        spans: [Span {
            start: 0,
            end: 0,
            style: HL_NORMAL,
        }; 256],
        heap_spans: ::core::ptr::null_mut::<Span>(),
        count: 0,
        capacity: 0,
    };
    let mut end_state: HighlightState = HighlightState {
        stack: [HighlightStackEntry {
            state: HS_NORMAL,
            sub_id: 0,
            string_delim: 0,
        }; 8],
        depth: 0,
    };
    let mut fname: *const ::core::ffi::c_char = &raw mut (*(*wp).w_bufp).b_fname
        as *mut ::core::ffi::c_char;
    if fname.is_null() || *fname == 0 {
        fname = &raw mut (*(*wp).w_bufp).b_bname as *mut ::core::ffi::c_char;
    }
    let mut profile: *const HighlightProfile = highlight_get_profile(fname);
    highlight_line(
        &raw mut (*lp).l_text as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_char,
        len,
        (*lp).hl_start_state,
        profile,
        &raw mut spans,
        &raw mut end_state,
    );
    if memcmp(
        &raw mut (*lp).hl_end_state as *const ::core::ffi::c_void,
        &raw mut end_state as *const ::core::ffi::c_void,
        ::core::mem::size_of::<HighlightState>() as size_t,
    ) != 0 as ::core::ffi::c_int
    {
        (*lp).hl_end_state = end_state;
        let mut next: *mut line = (*lp).l_fp;
        if next != (*curbp).b_linep {
            (*next).hl_start_state = end_state;
            lchange(WFHARD);
        }
    }
    let mut current_span_idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut char_idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut text_col: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while char_idx < len {
        let mut style: ::core::ffi::c_int = HL_NORMAL as ::core::ffi::c_int;
        while current_span_idx < spans.count {
            let mut s: *mut Span = if !spans.heap_spans.is_null() {
                spans.heap_spans.offset(current_span_idx as isize) as *mut Span
            } else {
                (&raw mut spans.spans as *mut Span).offset(current_span_idx as isize)
                    as *mut Span
            };
            if char_idx >= (*s).end {
                current_span_idx += 1;
            } else {
                if char_idx >= (*s).start {
                    style = (*s).style as ::core::ffi::c_int;
                }
                break;
            }
        }
        let mut c: unicode_t = 0;
        let mut bytes: ::core::ffi::c_int = utf8_to_unicode(
            &raw mut (*lp).l_text as *mut ::core::ffi::c_uchar,
            char_idx as ::core::ffi::c_uint,
            len as ::core::ffi::c_uint,
            &raw mut c,
        ) as ::core::ffi::c_int;
        let mut next_col: ::core::ffi::c_int = next_column(text_col, c, tab_width);
        if command_mode_block_selection_contains(lp, text_col, next_col) != 0 {
            style = HL_SELECTION as ::core::ffi::c_int;
        }
        let mut style_def: HighlightStyle = colorscheme_get(style as HighlightStyleID);
        let mut normal_def: HighlightStyle = colorscheme_get(HL_NORMAL);
        current_color_fg = if style_def.fg == -(1 as ::core::ffi::c_int) {
            normal_def.fg
        } else {
            style_def.fg
        };
        current_color_bg = if style_def.bg == -(1 as ::core::ffi::c_int) {
            normal_def.bg
        } else {
            style_def.bg
        };
        current_color_bold = style_def.bold;
        current_color_underline = style_def.underline;
        current_color_italic = style_def.italic;
        vtputc(c as ::core::ffi::c_int);
        char_idx += bytes;
        text_col = next_col;
    }
    span_vec_free(&raw mut spans);
    let mut colors: [ColorInfo; 16] = [ColorInfo {
        start: 0,
        end: 0,
        r: 0,
        g: 0,
        b: 0,
    }; 16];
    let mut color_count: ::core::ffi::c_int = highlight_find_colors(
        &raw mut (*lp).l_text as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_char,
        len,
        &raw mut colors as *mut ColorInfo,
        MAX_COLORS_PER_LINE,
    );
    if color_count > 0 as ::core::ffi::c_int {
        let mut normal: HighlightStyle = colorscheme_get(HL_NORMAL);
        current_color_fg = normal.fg;
        current_color_bg = normal.bg;
        current_color_bold = false_0 != 0;
        current_color_underline = false_0 != 0;
        vtputc(' ' as i32);
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < color_count && i < 8 as ::core::ffi::c_int {
            let mut packed_color: ::core::ffi::c_int = 0x1000000 as ::core::ffi::c_int
                | colors[i as usize].r << 16 as ::core::ffi::c_int
                | colors[i as usize].g << 8 as ::core::ffi::c_int | colors[i as usize].b;
            current_color_bg = packed_color;
            current_color_fg = packed_color;
            vtputc(' ' as i32);
            vtputc(' ' as i32);
            current_color_bg = normal.bg;
            current_color_fg = normal.fg;
            if i < color_count - 1 as ::core::ffi::c_int && i < 7 as ::core::ffi::c_int {
                vtputc(' ' as i32);
            }
            i += 1;
        }
    }
    let mut normal_0: HighlightStyle = colorscheme_get(HL_NORMAL);
    current_color_fg = normal_0.fg;
    current_color_bg = normal_0.bg;
    current_color_bold = normal_0.bold;
    current_color_underline = normal_0.underline;
    current_color_italic = normal_0.italic;
}
unsafe extern "C" fn show_line_wrapped(mut wp: *mut window, mut lp: *mut line) {
    let mut len: ::core::ffi::c_int = (*lp).l_used;
    if len == 0 as ::core::ffi::c_int {
        return;
    }
    let mut spans: SpanVec = SpanVec {
        spans: [Span {
            start: 0,
            end: 0,
            style: HL_NORMAL,
        }; 256],
        heap_spans: ::core::ptr::null_mut::<Span>(),
        count: 0,
        capacity: 0,
    };
    let mut end_state: HighlightState = HighlightState {
        stack: [HighlightStackEntry {
            state: HS_NORMAL,
            sub_id: 0,
            string_delim: 0,
        }; 8],
        depth: 0,
    };
    let mut fname: *const ::core::ffi::c_char = &raw mut (*(*wp).w_bufp).b_fname
        as *mut ::core::ffi::c_char;
    if fname.is_null() || *fname == 0 {
        fname = &raw mut (*(*wp).w_bufp).b_bname as *mut ::core::ffi::c_char;
    }
    let mut profile: *const HighlightProfile = highlight_get_profile(fname);
    highlight_line(
        &raw mut (*lp).l_text as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_char,
        len,
        (*lp).hl_start_state,
        profile,
        &raw mut spans,
        &raw mut end_state,
    );
    if memcmp(
        &raw mut (*lp).hl_end_state as *const ::core::ffi::c_void,
        &raw mut end_state as *const ::core::ffi::c_void,
        ::core::mem::size_of::<HighlightState>() as size_t,
    ) != 0 as ::core::ffi::c_int
    {
        (*lp).hl_end_state = end_state;
        let mut next: *mut line = (*lp).l_fp;
        if next != (*curbp).b_linep {
            (*next).hl_start_state = end_state;
            lchange(WFHARD);
        }
    }
    let mut current_span_idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut line_start_idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut line_start_col: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut current_block_54: u64;
    while line_start_idx < len {
        let mut char_idx: ::core::ffi::c_int = line_start_idx;
        let mut current_col: ::core::ffi::c_int = vtcol;
        let mut last_space_idx: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut text_col: ::core::ffi::c_int = line_start_col;
        let mut last_space_col: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut segment_end_idx: ::core::ffi::c_int = len;
        let mut segment_end_col: ::core::ffi::c_int = text_col;
        loop {
            if !(char_idx < len) {
                current_block_54 = 2569451025026770673;
                break;
            }
            let mut c: unicode_t = 0;
            let mut bytes: ::core::ffi::c_int = utf8_to_unicode(
                &raw mut (*lp).l_text as *mut ::core::ffi::c_uchar,
                char_idx as ::core::ffi::c_uint,
                len as ::core::ffi::c_uint,
                &raw mut c,
            ) as ::core::ffi::c_int;
            let mut char_width: ::core::ffi::c_int = get_char_width(c, current_col);
            let mut next_text_col: ::core::ffi::c_int = next_column(
                text_col,
                c,
                tab_width,
            );
            if c == ' ' as i32 as unicode_t || c == '\t' as i32 as unicode_t {
                last_space_idx = char_idx;
                last_space_col = next_text_col;
            }
            if current_col + char_width > (*term).t_ncol as ::core::ffi::c_int
                && char_idx > line_start_idx
            {
                if last_space_idx != -(1 as ::core::ffi::c_int) {
                    segment_end_idx = last_space_idx + 1 as ::core::ffi::c_int;
                    segment_end_col = last_space_col;
                } else {
                    segment_end_idx = char_idx;
                    segment_end_col = text_col;
                }
                current_block_54 = 12536556423873315668;
                break;
            } else {
                current_col += char_width;
                text_col = next_text_col;
                char_idx += bytes;
            }
        }
        match current_block_54 {
            2569451025026770673 => {
                segment_end_col = text_col;
            }
            _ => {}
        }
        char_idx = line_start_idx;
        text_col = line_start_col;
        while char_idx < segment_end_idx {
            let mut style: ::core::ffi::c_int = HL_NORMAL as ::core::ffi::c_int;
            while current_span_idx < spans.count {
                let mut s: *mut Span = if !spans.heap_spans.is_null() {
                    spans.heap_spans.offset(current_span_idx as isize) as *mut Span
                } else {
                    (&raw mut spans.spans as *mut Span).offset(current_span_idx as isize)
                        as *mut Span
                };
                if char_idx >= (*s).end {
                    current_span_idx += 1;
                } else {
                    if char_idx >= (*s).start {
                        style = (*s).style as ::core::ffi::c_int;
                    }
                    break;
                }
            }
            let mut c_0: unicode_t = 0;
            let mut bytes_0: ::core::ffi::c_int = utf8_to_unicode(
                &raw mut (*lp).l_text as *mut ::core::ffi::c_uchar,
                char_idx as ::core::ffi::c_uint,
                len as ::core::ffi::c_uint,
                &raw mut c_0,
            ) as ::core::ffi::c_int;
            let mut next_text_col_0: ::core::ffi::c_int = next_column(
                text_col,
                c_0,
                tab_width,
            );
            if command_mode_block_selection_contains(lp, text_col, next_text_col_0) != 0
            {
                style = HL_SELECTION as ::core::ffi::c_int;
            }
            let mut style_def: HighlightStyle = colorscheme_get(
                style as HighlightStyleID,
            );
            let mut normal_def: HighlightStyle = colorscheme_get(HL_NORMAL);
            current_color_fg = if style_def.fg == -(1 as ::core::ffi::c_int) {
                normal_def.fg
            } else {
                style_def.fg
            };
            current_color_bg = if style_def.bg == -(1 as ::core::ffi::c_int) {
                normal_def.bg
            } else {
                style_def.bg
            };
            current_color_bold = style_def.bold;
            current_color_underline = style_def.underline;
            current_color_italic = style_def.italic;
            vtputc(c_0 as ::core::ffi::c_int);
            char_idx += bytes_0;
            text_col = next_text_col_0;
        }
        line_start_idx = segment_end_idx;
        line_start_col = segment_end_col;
        if !(line_start_idx < len) {
            continue;
        }
        vtrow += 1;
        if vtrow >= nanox_text_rows() {
            break;
        }
        vtcol = vt_margin_left;
        (**vscreen.offset(vtrow as isize)).v_flag |= VFCHG;
        render_gutter(vtrow, 0 as ::core::ffi::c_int);
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            vtputc(' ' as i32);
            i += 1;
        }
    }
    span_vec_free(&raw mut spans);
}
unsafe extern "C" fn updone(mut wp: *mut window) {
    let mut lp: *mut line = ::core::ptr::null_mut::<line>();
    let mut sline: ::core::ffi::c_int = 0;
    let mut rows: ::core::ffi::c_int = nanox_text_rows();
    if (*(*wp).w_bufp).b_mode & MDSOFTWRAP != 0 {
        updall(wp);
        return;
    }
    lp = (*wp).w_linep as *mut line;
    sline = 0 as ::core::ffi::c_int;
    while lp != (*wp).w_dotp && sline < rows {
        sline += get_line_height(lp);
        lp = (*lp).l_fp;
    }
    if sline < rows {
        (**vscreen.offset(sline as isize)).v_flag |= VFCHG;
        (**vscreen.offset(sline as isize)).v_flag &= !VFREQ;
        let mut lnum: ::core::ffi::c_int = get_line_num((*wp).w_bufp as *mut buffer, lp);
        render_gutter(sline, lnum);
        vt_margin_left = get_gutter_width();
        vtmove(sline, 0 as ::core::ffi::c_int);
        if (*(*wp).w_bufp).b_mode & MDSOFTWRAP != 0 {
            show_line_wrapped(wp, lp);
        } else {
            show_line(wp, lp);
        }
        vteeol();
        vt_margin_left = 0 as ::core::ffi::c_int;
        if vtrow != sline {
            updall(wp);
        }
    }
}
unsafe extern "C" fn updall(mut wp: *mut window) {
    let mut lp: *mut line = ::core::ptr::null_mut::<line>();
    let mut sline: ::core::ffi::c_int = 0;
    let mut rows: ::core::ffi::c_int = nanox_text_rows();
    let mut lnum: ::core::ffi::c_int = get_line_num(
        (*wp).w_bufp as *mut buffer,
        (*wp).w_linep as *mut line,
    );
    lp = (*wp).w_linep as *mut line;
    sline = 0 as ::core::ffi::c_int;
    vt_margin_left = get_gutter_width();
    while sline < rows && sline < (*term).t_mrow as ::core::ffi::c_int {
        (**vscreen.offset(sline as isize)).v_flag |= VFCHG;
        (**vscreen.offset(sline as isize)).v_flag &= !VFREQ;
        render_gutter(
            sline,
            if lp != (*(*wp).w_bufp).b_linep { lnum } else { -(1 as ::core::ffi::c_int) },
        );
        vtmove(sline, 0 as ::core::ffi::c_int);
        if lp != (*(*wp).w_bufp).b_linep {
            if (*(*wp).w_bufp).b_mode & MDSOFTWRAP != 0 {
                show_line_wrapped(wp, lp);
            } else {
                show_line(wp, lp);
            }
            lp = (*lp).l_fp;
            lnum += 1;
            sline = vtrow;
        }
        vteeol();
        sline += 1;
    }
    vt_margin_left = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn updpos() {
    let mut lp: *mut line = ::core::ptr::null_mut::<line>();
    let mut i: ::core::ffi::c_int = 0;
    let mut rows: ::core::ffi::c_int = nanox_text_rows();
    lp = (*curwp).w_linep as *mut line;
    currow = 0 as ::core::ffi::c_int;
    while lp != (*curwp).w_dotp && currow < rows {
        currow += get_line_height(lp);
        lp = (*lp).l_fp;
    }
    curcol = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    lp = (*curwp).w_dotp;
    while i < (*curwp).w_doto {
        let mut c: unicode_t = 0;
        let mut bytes: ::core::ffi::c_int = 0;
        bytes = utf8_to_unicode(
            &raw mut (*lp).l_text as *mut ::core::ffi::c_uchar,
            i as ::core::ffi::c_uint,
            (*curwp).w_doto as ::core::ffi::c_uint,
            &raw mut c,
        ) as ::core::ffi::c_int;
        let mut w: ::core::ffi::c_int = get_char_width(c, curcol);
        if curcol + w > nanox_text_cols() {
            currow += 1;
            curcol = 4 as ::core::ffi::c_int;
            w = get_char_width(c, curcol);
        }
        curcol += w;
        i += bytes;
    }
    lbound = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn upddex() {
    let mut wp: *mut window = ::core::ptr::null_mut::<window>();
    let mut lp: *mut line = ::core::ptr::null_mut::<line>();
    let mut i: ::core::ffi::c_int = 0;
    wp = curwp;
    lp = (*wp).w_linep as *mut line;
    i = 0 as ::core::ffi::c_int;
    while i < nanox_text_rows() {
        if (**vscreen.offset(i as isize)).v_flag & VFEXT != 0 {
            if wp != curwp || lp != (*wp).w_dotp
                || curcol < nanox_text_cols() - 1 as ::core::ffi::c_int
            {
                vtmove(i, 0 as ::core::ffi::c_int);
                show_line(wp, lp);
                vteeol();
                (**vscreen.offset(i as isize)).v_flag &= !VFEXT;
                (**vscreen.offset(i as isize)).v_flag |= VFCHG;
            }
        }
        lp = (*lp).l_fp;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn TTputs(mut s: *const ::core::ffi::c_char) {
    let mut c: ::core::ffi::c_char = 0;
    loop {
        c = *s;
        if !(c as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
            break;
        }
        vttputc(c as ::core::ffi::c_int);
        s = s.offset(1);
    };
}
unsafe extern "C" fn start_screen_reset_smoothing() {
    TTputs(b"\x1B[?25l\0" as *const u8 as *const ::core::ffi::c_char);
    TTputs(b"\x1B[0m\0" as *const u8 as *const ::core::ffi::c_char);
    TTputs(b"\x1B[?7l\0" as *const u8 as *const ::core::ffi::c_char);
}
unsafe extern "C" fn finish_screen_reset_smoothing() {
    TTputs(b"\x1B[?7h\0" as *const u8 as *const ::core::ffi::c_char);
    TTputs(b"\x1B[?25h\0" as *const u8 as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn updgar() {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*term).t_nrow as ::core::ffi::c_int {
        (**vscreen.offset(i as isize)).v_flag |= VFCHG;
        i += 1;
    }
    let mut normal: HighlightStyle = colorscheme_get(HL_NORMAL);
    if normal.bg != -(1 as ::core::ffi::c_int) {
        vttsetcolors(normal.fg, normal.bg);
    }
    start_screen_reset_smoothing();
    movecursor(0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    Some((*term).t_eeop.expect("non-null function pointer"))
        .expect("non-null function pointer")();
    finish_screen_reset_smoothing();
    vttsetcolors(-(1 as ::core::ffi::c_int), -(1 as ::core::ffi::c_int));
    vttflush();
    sgarbf = FALSE;
    mpresf = FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn updupd(mut force: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut vp1: *mut video = ::core::ptr::null_mut::<video>();
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*term).t_nrow as ::core::ffi::c_int {
        vp1 = *vscreen.offset(i as isize);
        if (*vp1).v_flag & VFCHG != 0 as ::core::ffi::c_int {
            updateline(i, vp1);
        }
        i += 1;
    }
    return TRUE;
}
unsafe extern "C" fn is_letter(mut ch: unicode_t) -> bool {
    return ch > 128 as unicode_t
        || *(*__ctype_b_loc()).offset(ch as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & _ISalpha as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int != 0;
}
unsafe extern "C" fn is_notaword(mut ch: unicode_t) -> bool {
    return ch == '_' as i32 as unicode_t
        || ch >= '0' as i32 as unicode_t && ch <= '9' as i32 as unicode_t;
}
unsafe extern "C" fn find_letter(
    mut line: *mut unicode_t,
    mut len: size_t,
    mut pos: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    while (pos as size_t) < len {
        if is_letter(*line.offset(pos as isize)) {
            return pos;
        }
        pos += 1;
    }
    return -(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn find_not_letter(
    mut line: *mut unicode_t,
    mut len: size_t,
    mut pos: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    while (pos as size_t) < len {
        if !is_letter(*line.offset(pos as isize)) {
            return pos;
        }
        pos += 1;
    }
    return len as ::core::ffi::c_int;
}
pub const BAD_WORD_BEGIN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BAD_WORD_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
unsafe extern "C" fn findwords(
    mut line: *mut unicode_t,
    mut len: size_t,
    mut result: *mut ::core::ffi::c_uchar,
    mut size: size_t,
) -> size_t {
    let mut pos: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if len < size {
        size = len;
    }
    memset(result as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, size);
    loop {
        pos = find_letter(line, len, pos);
        if !(pos >= 0 as ::core::ffi::c_int) {
            break;
        }
        let mut start: ::core::ffi::c_int = pos;
        let mut end: ::core::ffi::c_int = find_not_letter(
            line,
            len,
            pos + 1 as ::core::ffi::c_int,
        );
        if ((end + 1 as ::core::ffi::c_int) as size_t) < len
            && *line.offset(end as isize) == '\'' as i32 as unicode_t
            && is_letter(*line.offset((end + 1 as ::core::ffi::c_int) as isize))
                as ::core::ffi::c_int != 0
        {
            end = find_not_letter(line, len, end + 2 as ::core::ffi::c_int);
        }
        pos = end + 1 as ::core::ffi::c_int;
        if start != 0
            && is_notaword(*line.offset((start - 1 as ::core::ffi::c_int) as isize))
                as ::core::ffi::c_int != 0
        {
            continue;
        }
        if (end as size_t) < len
            && is_notaword(*line.offset(end as isize)) as ::core::ffi::c_int != 0
        {
            continue;
        }
        if end as size_t > size {
            break;
        }
        let mut word_buffer: [::core::ffi::c_char; 80] = [0; 80];
        let mut word_len: ::core::ffi::c_int = end - start;
        if word_len as usize
            >= (::core::mem::size_of::<[::core::ffi::c_char; 80]>() as usize)
                .wrapping_sub(1 as usize)
        {
            continue;
        }
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < word_len {
            word_buffer[i as usize] = *line.offset((start + i) as isize)
                as ::core::ffi::c_char;
            i += 1;
        }
        word_buffer[word_len as usize] = 0 as ::core::ffi::c_char;
        if spellcheck(&raw mut word_buffer as *mut ::core::ffi::c_char) != 0 {
            continue;
        }
        *result.offset(start as isize) = BAD_WORD_BEGIN as ::core::ffi::c_uchar;
        let ref mut fresh1 = *result.offset((end - 1 as ::core::ffi::c_int) as isize);
        *fresh1 = (*fresh1 as ::core::ffi::c_int | BAD_WORD_END) as ::core::ffi::c_uchar;
    }
    return size;
}
unsafe extern "C" fn updateline(
    mut row: ::core::ffi::c_int,
    mut vp: *mut video,
) -> ::core::ffi::c_int {
    let mut maxchar: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut analyzed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut array: [::core::ffi::c_uchar; 256] = [0; 256];
    let mut text_buf: [unicode_t; 4096] = [0; 4096];
    let mut spellcheck_0: bool = (*(*curwp).w_bufp).b_mode & MDSPELL != 0;
    movecursor(row, 0 as ::core::ffi::c_int);
    vttsetcolors(-(1 as ::core::ffi::c_int), -(1 as ::core::ffi::c_int));
    vttsetattrs(
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    let mut phys_fg: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut phys_bg: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut phys_bold: bool = false_0 != 0;
    let mut phys_underline: bool = false_0 != 0;
    let mut phys_italic: bool = false_0 != 0;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*term).t_ncol as ::core::ffi::c_int {
        text_buf[i as usize] = (*(&raw mut (*vp).v_text as *mut video_cell)
            .offset(i as isize))
            .ch;
        if text_buf[i as usize] != 0 as unicode_t
            && (text_buf[i as usize] != ' ' as i32 as unicode_t
                || (*(&raw mut (*vp).v_text as *mut video_cell).offset(i as isize)).bg
                    != -(1 as ::core::ffi::c_int)
                || (*(&raw mut (*vp).v_text as *mut video_cell).offset(i as isize))
                    .underline as ::core::ffi::c_int != 0
                || (*(&raw mut (*vp).v_text as *mut video_cell).offset(i as isize))
                    .italic as ::core::ffi::c_int != 0)
        {
            maxchar = i + 1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    if (*vp).v_flag & VFREQ != 0 {
        maxchar = (*term).t_ncol as ::core::ffi::c_int;
        vttrev(TRUE);
        spellcheck_0 = false_0 != 0;
    }
    if spellcheck_0 {
        analyzed = findwords(
            &raw mut text_buf as *mut unicode_t,
            maxchar as size_t,
            &raw mut array as *mut ::core::ffi::c_uchar,
            ::core::mem::size_of::<[::core::ffi::c_uchar; 256]>() as size_t,
        ) as ::core::ffi::c_int;
    }
    let mut started: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < maxchar {
        let mut cell: *mut video_cell = (&raw mut (*vp).v_text as *mut video_cell)
            .offset(i_0 as isize) as *mut video_cell;
        if !((*cell).ch == 0 as unicode_t) {
            if (*cell).bold as ::core::ffi::c_int != phys_bold as ::core::ffi::c_int
                || (*cell).underline as ::core::ffi::c_int
                    != phys_underline as ::core::ffi::c_int
                || (*cell).italic as ::core::ffi::c_int
                    != phys_italic as ::core::ffi::c_int
            {
                vttsetattrs(
                    (*cell).bold as ::core::ffi::c_int,
                    (*cell).underline as ::core::ffi::c_int,
                    (*cell).italic as ::core::ffi::c_int,
                );
                phys_bold = (*cell).bold;
                phys_underline = (*cell).underline;
                phys_italic = (*cell).italic;
            }
            if (*cell).fg != phys_fg || (*cell).bg != phys_bg {
                vttsetcolors((*cell).fg, (*cell).bg);
                phys_fg = (*cell).fg;
                phys_bg = (*cell).bg;
            }
            if i_0 < analyzed
                && array[i_0 as usize] as ::core::ffi::c_int & BAD_WORD_BEGIN != 0
            {
                started = 1 as ::core::ffi::c_int;
                vttsetattrs(
                    1 as ::core::ffi::c_int,
                    phys_underline as ::core::ffi::c_int,
                    phys_italic as ::core::ffi::c_int,
                );
            }
            vttputc((*cell).ch as ::core::ffi::c_int);
            if i_0 < analyzed
                && array[i_0 as usize] as ::core::ffi::c_int & BAD_WORD_END != 0
            {
                vttsetattrs(
                    phys_bold as ::core::ffi::c_int,
                    phys_underline as ::core::ffi::c_int,
                    phys_italic as ::core::ffi::c_int,
                );
                started = 0 as ::core::ffi::c_int;
            }
        }
        i_0 += 1;
    }
    if started != 0 {
        vttsetattrs(
            phys_bold as ::core::ffi::c_int,
            phys_underline as ::core::ffi::c_int,
            phys_italic as ::core::ffi::c_int,
        );
    }
    ttcol = maxchar;
    vttsetcolors(-(1 as ::core::ffi::c_int), -(1 as ::core::ffi::c_int));
    vttsetattrs(
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    let mut normal: HighlightStyle = colorscheme_get(HL_NORMAL);
    if normal.bg != -(1 as ::core::ffi::c_int) {
        vttsetcolors(normal.fg, normal.bg);
    }
    vtteeol();
    vttsetcolors(-(1 as ::core::ffi::c_int), -(1 as ::core::ffi::c_int));
    vttsetattrs(
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    vttrev(FALSE);
    (*vp).v_flag &= !VFCHG;
    return TRUE;
}
unsafe extern "C" fn modeline(mut wp: *mut window) {
    let mut bp: *mut buffer = (*wp).w_bufp as *mut buffer;
    let mut row1: *const ::core::ffi::c_char = if nanox_cfg.hint_bar
        as ::core::ffi::c_int != 0
    {
        b"F1/^H Help F2/^S Save F3/^O Open F4/^Q Quit F5/^F Search\0" as *const u8
            as *const ::core::ffi::c_char
    } else {
        b"\0" as *const u8 as *const ::core::ffi::c_char
    };
    let mut row2: *const ::core::ffi::c_char = b"\0" as *const u8
        as *const ::core::ffi::c_char;
    let mut status: [::core::ffi::c_char; 4097] = [0; 4097];
    let mut fname: *const ::core::ffi::c_char = if (*bp)
        .b_fname[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 0
    {
        &raw mut (*bp).b_fname as *mut ::core::ffi::c_char
    } else {
        &raw mut (*bp).b_bname as *mut ::core::ffi::c_char
    };
    let mut lamp: *const ::core::ffi::c_char = nanox_lamp_label();
    let mut mark: ::core::ffi::c_char = (if (*bp).b_flag as ::core::ffi::c_int & BFCHG
        != 0
    {
        '*' as i32
    } else {
        '-' as i32
    }) as ::core::ffi::c_char;
    let mut line: ::core::ffi::c_int = window_line_number(wp);
    let mut col: ::core::ffi::c_int = window_column_number(wp);
    let mut top: ::core::ffi::c_int = nanox_hint_top_row();
    let mut bottom: ::core::ffi::c_int = nanox_hint_bottom_row();
    if nanox_cfg.hint_bar {
        row2 = if nanox_cfg.no_function_slot as ::core::ffi::c_int != 0 {
            b"F6/^W Copy(S:End) F7/^X Cut(S:End) F8/^V Paste ^A+num Slot\0" as *const u8
                as *const ::core::ffi::c_char
        } else {
            b"F6/^W Copy(S:End) F7/^X Cut(S:End) F8/^V Paste F9-12 Slot\0" as *const u8
                as *const ::core::ffi::c_char
        };
    }
    snprintf(
        &raw mut status as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4097]>() as size_t,
        b"%s L%d C%d %c%s%s\0" as *const u8 as *const ::core::ffi::c_char,
        fname,
        line,
        col,
        mark as ::core::ffi::c_int,
        if *lamp as ::core::ffi::c_int != 0 {
            b" \0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        },
        if *lamp as ::core::ffi::c_int != 0 {
            lamp
        } else {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        },
    );
    if top >= 0 as ::core::ffi::c_int && top < (*term).t_nrow as ::core::ffi::c_int {
        (**vscreen.offset(top as isize)).v_flag |= VFCHG | VFCOL;
        draw_hint_row(top, row1, &raw mut status as *mut ::core::ffi::c_char);
    }
    if bottom >= 0 as ::core::ffi::c_int && bottom < (*term).t_nrow as ::core::ffi::c_int
    {
        (**vscreen.offset(bottom as isize)).v_flag |= VFCHG | VFCOL;
        draw_hint_row(bottom, row2, b"\0" as *const u8 as *const ::core::ffi::c_char);
    }
}
#[no_mangle]
pub unsafe extern "C" fn upmode() {
    (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | WFMODE)
        as ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn movecursor(
    mut row: ::core::ffi::c_int,
    mut col: ::core::ffi::c_int,
) {
    if row != ttrow || col != ttcol {
        ttrow = row;
        ttcol = col;
        vttmove(row, col);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mlerase() {
    let mut i: ::core::ffi::c_int = 0;
    movecursor((*term).t_nrow as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    let mut normal: HighlightStyle = colorscheme_get(HL_NORMAL);
    vttsetcolors(normal.fg, normal.bg);
    if eolexist == TRUE {
        vtteeol();
    } else {
        i = 0 as ::core::ffi::c_int;
        while i < (*term).t_ncol as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
            vttputc(' ' as i32);
            i += 1;
        }
        movecursor((*term).t_nrow as ::core::ffi::c_int, 1 as ::core::ffi::c_int);
        movecursor((*term).t_nrow as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    }
    if normal.bg != -(1 as ::core::ffi::c_int) {
        TTputs(b"\x1B[0m\0" as *const u8 as *const ::core::ffi::c_char);
    }
    vttflush();
    mpresf = FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn mlwrite(mut fmt: *const ::core::ffi::c_char, mut args: ...) {
    let mut c: ::core::ffi::c_int = 0;
    let mut ap: ::core::ffi::VaListImpl;
    let mut raw: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut final_0: [::core::ffi::c_char; 1200] = [0; 1200];
    let mut dest: mlbuf = mlbuf {
        buf: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        len: 0,
        cap: 0,
    };
    if discmd == FALSE {
        movecursor((*term).t_nrow as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
        return;
    }
    if eolexist == FALSE {
        mlerase();
        vttflush();
    }
    movecursor((*term).t_nrow as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    let mut normal: HighlightStyle = colorscheme_get(HL_NORMAL);
    vttsetcolors(normal.fg, normal.bg);
    mlbuf_init(
        &raw mut dest,
        &raw mut raw as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
    );
    ap = args.clone();
    loop {
        let fresh2 = fmt;
        fmt = fmt.offset(1);
        c = *fresh2 as ::core::ffi::c_int;
        if !(c != 0 as ::core::ffi::c_int) {
            break;
        }
        if c != '%' as i32 {
            mlbuf_putc(&raw mut dest, c);
        } else {
            let fresh3 = fmt;
            fmt = fmt.offset(1);
            c = *fresh3 as ::core::ffi::c_int;
            match c {
                100 => {
                    mlputi(
                        ap.arg::<::core::ffi::c_int>(),
                        10 as ::core::ffi::c_int,
                        &raw mut dest,
                    );
                }
                111 => {
                    mlputi(
                        ap.arg::<::core::ffi::c_int>(),
                        8 as ::core::ffi::c_int,
                        &raw mut dest,
                    );
                }
                120 => {
                    mlputi(
                        ap.arg::<::core::ffi::c_int>(),
                        16 as ::core::ffi::c_int,
                        &raw mut dest,
                    );
                }
                68 => {
                    mlputli(
                        ap.arg::<::core::ffi::c_long>(),
                        10 as ::core::ffi::c_int,
                        &raw mut dest,
                    );
                }
                115 => {
                    mlbuf_puts(&raw mut dest, ap.arg::<*mut ::core::ffi::c_char>());
                }
                102 => {
                    mlputf(ap.arg::<::core::ffi::c_int>(), &raw mut dest);
                }
                _ => {
                    mlbuf_putc(&raw mut dest, c);
                }
            }
        }
    }
    nanox_message_prefix(
        dest.buf,
        &raw mut final_0 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1200]>() as size_t,
    );
    let mut p: *mut ::core::ffi::c_uchar = &raw mut final_0 as *mut ::core::ffi::c_char
        as *mut ::core::ffi::c_uchar;
    let mut len: ::core::ffi::c_int = strlen(p as *mut ::core::ffi::c_char)
        as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < len {
        let mut uc: unicode_t = 0;
        let mut bytes: ::core::ffi::c_int = utf8_to_unicode(
            p,
            i as ::core::ffi::c_uint,
            len as ::core::ffi::c_uint,
            &raw mut uc,
        ) as ::core::ffi::c_int;
        vttputc(uc as ::core::ffi::c_int);
        ttcol += unicode_width(uc);
        i += bytes;
    }
    if eolexist == TRUE {
        vtteeol();
    }
    vttflush();
    mpresf = TRUE;
    nanox_notify_message(&raw mut final_0 as *mut ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn mlforce(mut s: *mut ::core::ffi::c_char) {
    let mut oldcmd: ::core::ffi::c_int = 0;
    oldcmd = discmd;
    discmd = TRUE;
    mlwrite(s);
    discmd = oldcmd;
}
#[no_mangle]
pub unsafe extern "C" fn mlputs(mut s: *mut ::core::ffi::c_char) {
    let mut p: *mut ::core::ffi::c_uchar = s as *mut ::core::ffi::c_uchar;
    let mut len: ::core::ffi::c_int = strlen(s) as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < len {
        let mut uc: unicode_t = 0;
        let mut bytes: ::core::ffi::c_int = utf8_to_unicode(
            p,
            i as ::core::ffi::c_uint,
            len as ::core::ffi::c_uint,
            &raw mut uc,
        ) as ::core::ffi::c_int;
        vttputc(uc as ::core::ffi::c_int);
        ttcol += unicode_width(uc);
        i += bytes;
    }
}
unsafe extern "C" fn mlputi(
    mut i: ::core::ffi::c_int,
    mut r: ::core::ffi::c_int,
    mut dest: *mut mlbuf,
) {
    let mut q: ::core::ffi::c_int = 0;
    static mut hexdigits: [::core::ffi::c_char; 17] = unsafe {
        ::core::mem::transmute::<
            [u8; 17],
            [::core::ffi::c_char; 17],
        >(*b"0123456789ABCDEF\0")
    };
    if i < 0 as ::core::ffi::c_int {
        i = -i;
        mlbuf_putc(dest, '-' as i32);
    }
    q = i / r;
    if q != 0 as ::core::ffi::c_int {
        mlputi(q, r, dest);
    }
    mlbuf_putc(dest, hexdigits[(i % r) as usize] as ::core::ffi::c_int);
}
unsafe extern "C" fn mlputli(
    mut l: ::core::ffi::c_long,
    mut r: ::core::ffi::c_int,
    mut dest: *mut mlbuf,
) {
    let mut q: ::core::ffi::c_long = 0;
    if l < 0 as ::core::ffi::c_long {
        l = -l;
        mlbuf_putc(dest, '-' as i32);
    }
    q = l / r as ::core::ffi::c_long;
    if q != 0 as ::core::ffi::c_long {
        mlputli(q, r, dest);
    }
    mlbuf_putc(dest, (l % r as ::core::ffi::c_long) as ::core::ffi::c_int + '0' as i32);
}
unsafe extern "C" fn mlputf(mut s: ::core::ffi::c_int, mut dest: *mut mlbuf) {
    let mut i: ::core::ffi::c_int = 0;
    let mut f: ::core::ffi::c_int = 0;
    i = s / 100 as ::core::ffi::c_int;
    f = s % 100 as ::core::ffi::c_int;
    if f < 0 as ::core::ffi::c_int {
        f = -f;
    }
    if i < 0 as ::core::ffi::c_int && s > 0 as ::core::ffi::c_int {
        i = -i;
    }
    mlputi(i, 10 as ::core::ffi::c_int, dest);
    mlbuf_putc(dest, '.' as i32);
    if f < 10 as ::core::ffi::c_int {
        mlbuf_putc(dest, '0' as i32);
    }
    mlbuf_putc(dest, f / 10 as ::core::ffi::c_int + '0' as i32);
    mlbuf_putc(dest, f % 10 as ::core::ffi::c_int + '0' as i32);
}
#[no_mangle]
pub unsafe extern "C" fn getscreensize(
    mut widthp: *mut ::core::ffi::c_int,
    mut heightp: *mut ::core::ffi::c_int,
) {
    let mut size: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    *widthp = 0 as ::core::ffi::c_int;
    *heightp = 0 as ::core::ffi::c_int;
    if ioctl(0 as ::core::ffi::c_int, TIOCGWINSZ as ::core::ffi::c_ulong, &raw mut size)
        < 0 as ::core::ffi::c_int
    {
        return;
    }
    *widthp = size.ws_col as ::core::ffi::c_int;
    *heightp = size.ws_row as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sizesignal(mut signr: ::core::ffi::c_int) {
    let mut w: ::core::ffi::c_int = 0;
    let mut h: ::core::ffi::c_int = 0;
    let mut old_errno: ::core::ffi::c_int = *__errno_location();
    getscreensize(&raw mut w, &raw mut h);
    if h != 0 && w != 0
        && (h - 1 as ::core::ffi::c_int != (*term).t_nrow as ::core::ffi::c_int
            || w != (*term).t_ncol as ::core::ffi::c_int)
    {
        newscreensize(h, w);
    }
    signal(SIGWINCH, Some(sizesignal as unsafe extern "C" fn(::core::ffi::c_int) -> ()));
    *__errno_location() = old_errno;
}
#[no_mangle]
pub unsafe extern "C" fn newscreensize(
    mut h: ::core::ffi::c_int,
    mut w: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if displaying != 0 {
        chg_width = w;
        chg_height = h;
        return FALSE;
    }
    chg_height = 0 as ::core::ffi::c_int;
    chg_width = chg_height;
    if h - 1 as ::core::ffi::c_int != (*term).t_mrow as ::core::ffi::c_int {
        newsize(TRUE, h);
    }
    if w != (*term).t_mcol as ::core::ffi::c_int {
        newwidth(TRUE, w);
    }
    update(TRUE);
    return TRUE;
}
pub const MAX_COLORS_PER_LINE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
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
unsafe extern "C" fn utf8_display_width(
    mut str: *const ::core::ffi::c_char,
    mut byte_len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut width: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < byte_len && *str.offset(i as isize) as ::core::ffi::c_int != 0 {
        let mut c: unicode_t = 0;
        let mut bytes: ::core::ffi::c_int = utf8_to_unicode(
            str as *mut ::core::ffi::c_uchar,
            i as ::core::ffi::c_uint,
            byte_len as ::core::ffi::c_uint,
            &raw mut c,
        ) as ::core::ffi::c_int;
        if bytes <= 0 as ::core::ffi::c_int {
            break;
        }
        width += mystrnlen_raw_w(c);
        i += bytes;
    }
    return width;
}
pub const VFCHG: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const VFEXT: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const VFREQ: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const VFCOL: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
