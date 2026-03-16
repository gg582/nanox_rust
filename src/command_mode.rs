extern "C" {
    pub type pcre2_real_general_context_8;
    pub type pcre2_real_compile_context_8;
    pub type pcre2_real_match_context_8;
    pub type pcre2_real_code_8;
    pub type pcre2_real_match_data_8;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn atoi(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn pcre2_match_data_create_from_pattern_8(
        _: *const pcre2_code_8,
        _: *mut pcre2_general_context_8,
    ) -> *mut pcre2_match_data_8;
    fn pcre2_match_8(
        _: *const pcre2_code_8,
        _: PCRE2_SPTR8,
        _: size_t,
        _: size_t,
        _: uint32_t,
        _: *mut pcre2_match_data_8,
        _: *mut pcre2_match_context_8,
    ) -> ::core::ffi::c_int;
    fn pcre2_match_data_free_8(_: *mut pcre2_match_data_8);
    fn pcre2_get_ovector_pointer_8(_: *mut pcre2_match_data_8) -> *mut size_t;
    fn pcre2_get_error_message_8(
        _: ::core::ffi::c_int,
        _: *mut PCRE2_UCHAR8,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn pcre2_compile_8(
        _: PCRE2_SPTR8,
        _: size_t,
        _: uint32_t,
        _: *mut ::core::ffi::c_int,
        _: *mut size_t,
        _: *mut pcre2_compile_context_8,
    ) -> *mut pcre2_code_8;
    fn pcre2_code_free_8(_: *mut pcre2_code_8);
    static mut term: *mut terminal;
    static mut tab_width: ::core::ffi::c_int;
    fn vttputc(c: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn vttflush();
    fn vttsetcolors(fg: ::core::ffi::c_int, bg: ::core::ffi::c_int);
    fn vttsetattrs(
        bold: ::core::ffi::c_int,
        underline: ::core::ffi::c_int,
        italic: ::core::ffi::c_int,
    );
    static mut curwp: *mut window;
    static mut curbp: *mut buffer;
    static mut discmd: ::core::ffi::c_int;
    static mut sgarbf: ::core::ffi::c_int;
    static mut mpresf: ::core::ffi::c_int;
    static mut indent_start_lp: *mut line;
    static mut indent_end_lp: *mut line;
    static mut indent_range_type: ::core::ffi::c_int;
    static mut indent_selection_active: ::core::ffi::c_int;
    fn nanox_help_command(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn nanox_request_underbar_redraw();
    fn gotobol(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn backchar(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn gotoeol(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn forwchar(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn gotobob(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn gotoeob(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn forwline(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn backline(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn forwpage(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn backpage(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn indent_apply_range(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn execute(
        c: ::core::ffi::c_int,
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn rdonly() -> ::core::ffi::c_int;
    fn update(force: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn movecursor(row: ::core::ffi::c_int, col: ::core::ffi::c_int);
    fn mlwrite(fmt: *const ::core::ffi::c_char, ...);
    fn mlyesno(prompt: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn getbind(c: ::core::ffi::c_int) -> fn_t;
    fn minibuf_input(
        prompt: *const ::core::ffi::c_char,
        dest: *mut ::core::ffi::c_char,
        max_len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn colorscheme_get(id: HighlightStyleID) -> HighlightStyle;
    fn linsert(n: ::core::ffi::c_int, c: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn linsert_block(
        block: *const ::core::ffi::c_char,
        len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ldelete(n: ::core::ffi::c_long, kflag: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn utf8_to_unicode(
        line: *mut ::core::ffi::c_uchar,
        index: ::core::ffi::c_uint,
        len: ::core::ffi::c_uint,
        res: *mut unicode_t,
    ) -> ::core::ffi::c_uint;
    fn unicode_width(c: unicode_t) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
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
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type PCRE2_UCHAR8 = uint8_t;
pub type PCRE2_SPTR8 = *const PCRE2_UCHAR8;
pub type pcre2_general_context_8 = pcre2_real_general_context_8;
pub type pcre2_compile_context_8 = pcre2_real_compile_context_8;
pub type pcre2_match_context_8 = pcre2_real_match_context_8;
pub type pcre2_code_8 = pcre2_real_code_8;
pub type pcre2_match_data_8 = pcre2_real_match_data_8;
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
pub type fn_t = Option<
    unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int) -> ::core::ffi::c_int,
>;
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
pub type unicode_t = ::core::ffi::c_uint;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PCRE2_CASELESS: ::core::ffi::c_uint = 0x8 as ::core::ffi::c_uint;
pub const PCRE2_MULTILINE: ::core::ffi::c_uint = 0x400 as ::core::ffi::c_uint;
pub const PCRE2_UCP: ::core::ffi::c_uint = 0x20000 as ::core::ffi::c_uint;
pub const PCRE2_UTF: ::core::ffi::c_uint = 0x80000 as ::core::ffi::c_uint;
pub const PCRE2_ZERO_TERMINATED: size_t = !(0 as ::core::ffi::c_int as size_t);
pub const CONTROL: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WFMOVE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const WFHARD: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const WFMODE: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const MDVIEW: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
static mut block_active: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut block_replace: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut block_anchor_line: *mut line = ::core::ptr::null::<line>() as *mut line;
static mut block_anchor_offset: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn command_mode_write_segment(
    mut text: *const ::core::ffi::c_char,
    mut style: *const HighlightStyle,
    mut col: *mut ::core::ffi::c_int,
) {
    if text.is_null() || *text == 0 || style.is_null() || col.is_null() || term.is_null()
    {
        return;
    }
    vttsetcolors((*style).fg, (*style).bg);
    vttsetattrs(
        (*style).bold as ::core::ffi::c_int,
        (*style).underline as ::core::ffi::c_int,
        (*style).italic as ::core::ffi::c_int,
    );
    let mut bytes: *const ::core::ffi::c_uchar = text as *const ::core::ffi::c_uchar;
    let mut len: ::core::ffi::c_int = strlen(text) as ::core::ffi::c_int;
    let mut idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while idx < len && *col < (*term).t_ncol as ::core::ffi::c_int {
        let mut uc: unicode_t = 0;
        let mut consumed: ::core::ffi::c_int = utf8_to_unicode(
            bytes as *mut ::core::ffi::c_uchar,
            idx as ::core::ffi::c_uint,
            len as ::core::ffi::c_uint,
            &raw mut uc,
        ) as ::core::ffi::c_int;
        if consumed <= 0 as ::core::ffi::c_int {
            break;
        }
        let mut width: ::core::ffi::c_int = mystrnlen_raw_w(uc);
        if *col + width > (*term).t_ncol as ::core::ffi::c_int {
            break;
        }
        vttputc(uc as ::core::ffi::c_int);
        *col += width;
        idx += consumed;
    }
}
unsafe extern "C" fn command_mode_draw_status(
    mut status: *const ::core::ffi::c_char,
    mut input: *const ::core::ffi::c_char,
    mut show_cursor: bool,
) {
    if discmd == 0 || term.is_null() {
        return;
    }
    let mut normal: HighlightStyle = colorscheme_get(HL_NORMAL);
    let mut label: HighlightStyle = colorscheme_get(HL_NOTICE);
    let mut status_style: HighlightStyle = colorscheme_get(HL_HEADER);
    let mut input_style: HighlightStyle = colorscheme_get(HL_FUNCTION);
    movecursor((*term).t_nrow as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    let mut col: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    command_mode_write_segment(
        b"F1 \0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut label,
        &raw mut col,
    );
    command_mode_write_segment(
        b"Command \0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut label,
        &raw mut col,
    );
    if !status.is_null() && *status as ::core::ffi::c_int != 0 {
        command_mode_write_segment(status, &raw mut status_style, &raw mut col);
    }
    if !input.is_null() && *input as ::core::ffi::c_int != 0 {
        if !status.is_null() && *status as ::core::ffi::c_int != 0 {
            command_mode_write_segment(
                b" \0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut status_style,
                &raw mut col,
            );
        }
        command_mode_write_segment(input, &raw mut input_style, &raw mut col);
    }
    if show_cursor {
        command_mode_write_segment(
            b"_\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut input_style,
            &raw mut col,
        );
    }
    vttsetcolors(normal.fg, normal.bg);
    vttsetattrs(
        normal.bold as ::core::ffi::c_int,
        normal.underline as ::core::ffi::c_int,
        normal.italic as ::core::ffi::c_int,
    );
    while col < (*term).t_ncol as ::core::ffi::c_int {
        vttputc(' ' as i32);
        col += 1;
    }
    vttsetcolors(-(1 as ::core::ffi::c_int), -(1 as ::core::ffi::c_int));
    vttsetattrs(
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    vttflush();
    mpresf = TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn command_mode_init() {
    block_active = 0 as ::core::ffi::c_int;
    block_replace = 0 as ::core::ffi::c_int;
    block_anchor_line = ::core::ptr::null_mut::<line>();
    block_anchor_offset = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn command_mode_activate() {
    command_mode_prompt();
}
unsafe extern "C" fn execute_goto_line(mut line_num: ::core::ffi::c_int) {
    let mut total_lines: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut lp: *mut line = ::core::ptr::null_mut::<line>();
    lp = (*(*curbp).b_linep).l_fp;
    while lp != (*curbp).b_linep {
        total_lines += 1;
        lp = (*lp).l_fp;
    }
    if line_num < 1 as ::core::ffi::c_int {
        line_num = 1 as ::core::ffi::c_int;
    }
    if line_num > total_lines {
        line_num = total_lines;
    }
    (*curwp).w_dotp = (*(*curbp).b_linep).l_fp;
    (*curwp).w_doto = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while i < line_num && (*(*curwp).w_dotp).l_fp != (*curbp).b_linep {
        (*curwp).w_dotp = (*(*curwp).w_dotp).l_fp;
        i += 1;
    }
    (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | WFMOVE)
        as ::core::ffi::c_char;
    mlwrite(
        b"Line %d of %d\0" as *const u8 as *const ::core::ffi::c_char,
        line_num,
        total_lines,
    );
}
unsafe extern "C" fn execute_help() {
    sgarbf = TRUE;
    update(TRUE);
    nanox_help_command(FALSE, 1 as ::core::ffi::c_int);
}
unsafe extern "C" fn start_block_mode(mut replace_mode: ::core::ffi::c_int) {
    block_active = 1 as ::core::ffi::c_int;
    block_replace = replace_mode;
    block_anchor_line = (*curwp).w_dotp;
    block_anchor_offset = (*curwp).w_doto;
    (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | (WFHARD | WFMODE))
        as ::core::ffi::c_char;
    render_block_status();
}
unsafe extern "C" fn command_mode_trim(mut text: *mut ::core::ffi::c_char) {
    if text.is_null() {
        return;
    }
    let mut start: *mut ::core::ffi::c_char = text;
    while *start as ::core::ffi::c_int != 0
        && *(*__ctype_b_loc())
            .offset(*start as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int != 0
    {
        start = start.offset(1);
    }
    if start != text {
        memmove(
            text as *mut ::core::ffi::c_void,
            start as *const ::core::ffi::c_void,
            strlen(start).wrapping_add(1 as size_t),
        );
    }
    let mut len: size_t = strlen(text);
    while len > 0 as size_t
        && *(*__ctype_b_loc())
            .offset(
                *text.offset(len.wrapping_sub(1 as size_t) as isize)
                    as ::core::ffi::c_uchar as ::core::ffi::c_int as isize,
            ) as ::core::ffi::c_int
            & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int != 0
    {
        len = len.wrapping_sub(1);
    }
    *text.offset(len as isize) = '\0' as i32 as ::core::ffi::c_char;
}
unsafe extern "C" fn command_mode_prompt() {
    let mut input: [::core::ffi::c_char; 256] = [0; 256];
    let mut status: ::core::ffi::c_int = minibuf_input(
        b"Command Mode: \0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut input as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as ::core::ffi::c_int,
    );
    if status == TRUE {
        execute_command(&raw mut input as *mut ::core::ffi::c_char);
    }
    nanox_request_underbar_redraw();
}
unsafe extern "C" fn command_mode_handle_range_command(
    mut input: *const ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
    mut indent_direction: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut cmd_len: size_t = strlen(name);
    if strncasecmp(input, name, cmd_len) != 0 as ::core::ffi::c_int {
        return FALSE;
    }
    let mut range: *const ::core::ffi::c_char = input.offset(cmd_len as isize);
    if *range as ::core::ffi::c_int != 0
        && *(*__ctype_b_loc())
            .offset(*range as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int == 0
    {
        return FALSE;
    }
    while *range as ::core::ffi::c_int != 0
        && *(*__ctype_b_loc())
            .offset(*range as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int != 0
    {
        range = range.offset(1);
    }
    if *range as ::core::ffi::c_int == '\0' as i32 {
        mlwrite(
            b"[%s syntax: %s start-end]\0" as *const u8 as *const ::core::ffi::c_char,
            name,
            name,
        );
        return TRUE;
    }
    let mut start_line: ::core::ffi::c_int = 0;
    let mut end_line: ::core::ffi::c_int = 0;
    if command_mode_parse_line_range(range, &raw mut start_line, &raw mut end_line) == 0
    {
        mlwrite(
            b"[%s range must be start-end]\0" as *const u8 as *const ::core::ffi::c_char,
            name,
        );
        return TRUE;
    }
    command_mode_apply_indent_range(start_line, end_line, indent_direction);
    return TRUE;
}
unsafe extern "C" fn execute_command(mut input: *const ::core::ffi::c_char) {
    if input.is_null() || *input as ::core::ffi::c_int == '\0' as i32 {
        mlwrite(b"Empty command\0" as *const u8 as *const ::core::ffi::c_char);
        return;
    }
    let mut buffer: [::core::ffi::c_char; 256] = [0; 256];
    mystrscpy(
        &raw mut buffer as *mut ::core::ffi::c_char,
        input,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as ::core::ffi::c_int,
    );
    command_mode_trim(&raw mut buffer as *mut ::core::ffi::c_char);
    if buffer[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '\0' as i32 {
        mlwrite(b"Empty command\0" as *const u8 as *const ::core::ffi::c_char);
        return;
    }
    if command_mode_handle_range_command(
        &raw mut buffer as *mut ::core::ffi::c_char,
        b"indent\0" as *const u8 as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
    ) != 0
    {
        return;
    }
    if command_mode_handle_range_command(
        &raw mut buffer as *mut ::core::ffi::c_char,
        b"outdent\0" as *const u8 as *const ::core::ffi::c_char,
        -(1 as ::core::ffi::c_int),
    ) != 0
    {
        return;
    }
    let mut is_number: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut p: *mut ::core::ffi::c_char = &raw mut buffer as *mut ::core::ffi::c_char;
    while *p != 0 {
        if *(*__ctype_b_loc())
            .offset(*p as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int == 0
        {
            is_number = 0 as ::core::ffi::c_int;
            break;
        } else {
            p = p.offset(1);
        }
    }
    if is_number != 0 {
        let mut line_num: ::core::ffi::c_int = atoi(
            &raw mut buffer as *mut ::core::ffi::c_char,
        );
        execute_goto_line(line_num);
    } else if strcasecmp(
        &raw mut buffer as *mut ::core::ffi::c_char,
        b"help\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        || strcasecmp(
            &raw mut buffer as *mut ::core::ffi::c_char,
            b"h\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        execute_help();
    } else if strcasecmp(
        &raw mut buffer as *mut ::core::ffi::c_char,
        b"viblock-edit\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        || strcasecmp(
            &raw mut buffer as *mut ::core::ffi::c_char,
            b"viblock edit\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        start_block_mode(FALSE);
    } else if strcasecmp(
        &raw mut buffer as *mut ::core::ffi::c_char,
        b"viblock-replace\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        || strcasecmp(
            &raw mut buffer as *mut ::core::ffi::c_char,
            b"viblock replace\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        start_block_mode(TRUE);
    } else {
        mlwrite(
            b"Unknown command: %s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut buffer as *mut ::core::ffi::c_char,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn command_mode_cleanup() {
    block_active = 0 as ::core::ffi::c_int;
    block_replace = 0 as ::core::ffi::c_int;
    block_anchor_line = ::core::ptr::null_mut::<line>();
    block_anchor_offset = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn command_mode_activate_command(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    command_mode_activate();
    return TRUE;
}
unsafe extern "C" fn read_sed_chunk(
    mut pp: *mut *const ::core::ffi::c_char,
    mut delim: ::core::ffi::c_char,
    mut dest: *mut ::core::ffi::c_char,
    mut dest_sz: size_t,
    mut label: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut idx: size_t = 0 as size_t;
    let mut p: *const ::core::ffi::c_char = *pp;
    if dest_sz == 0 as size_t {
        return FALSE;
    }
    while *p as ::core::ffi::c_int != 0
        && *p as ::core::ffi::c_int != delim as ::core::ffi::c_int
    {
        let fresh17 = p;
        p = p.offset(1);
        let mut c: ::core::ffi::c_uchar = *fresh17 as ::core::ffi::c_uchar;
        if c as ::core::ffi::c_int == '\\' as i32 {
            if *p as ::core::ffi::c_int == '\0' as i32 {
                mlwrite(
                    b"Unterminated %s\0" as *const u8 as *const ::core::ffi::c_char,
                    label,
                );
                return FALSE;
            }
            let fresh18 = p;
            p = p.offset(1);
            c = *fresh18 as ::core::ffi::c_uchar;
            match c as ::core::ffi::c_int {
                110 => {
                    c = '\n' as i32 as ::core::ffi::c_uchar;
                }
                116 => {
                    c = '\t' as i32 as ::core::ffi::c_uchar;
                }
                114 => {
                    c = '\r' as i32 as ::core::ffi::c_uchar;
                }
                92 => {
                    c = '\\' as i32 as ::core::ffi::c_uchar;
                }
                _ => {}
            }
        }
        if idx.wrapping_add(1 as size_t) >= dest_sz {
            mlwrite(b"%s too long\0" as *const u8 as *const ::core::ffi::c_char, label);
            return FALSE;
        }
        let fresh19 = idx;
        idx = idx.wrapping_add(1);
        *dest.offset(fresh19 as isize) = c as ::core::ffi::c_char;
    }
    if *p as ::core::ffi::c_int != delim as ::core::ffi::c_int {
        mlwrite(b"Unterminated %s\0" as *const u8 as *const ::core::ffi::c_char, label);
        return FALSE;
    }
    *dest.offset(idx as isize) = '\0' as i32 as ::core::ffi::c_char;
    *pp = p.offset(1 as ::core::ffi::c_int as isize);
    return TRUE;
}
unsafe extern "C" fn parse_sed_expression(
    mut expr: *const ::core::ffi::c_char,
    mut pattern: *mut ::core::ffi::c_char,
    mut pat_sz: size_t,
    mut replacement: *mut ::core::ffi::c_char,
    mut rep_sz: size_t,
    mut is_global: *mut ::core::ffi::c_int,
    mut is_caseless: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut p: *const ::core::ffi::c_char = expr;
    let mut global: ::core::ffi::c_int = FALSE;
    let mut caseless: ::core::ffi::c_int = FALSE;
    while *p as ::core::ffi::c_int != 0
        && *(*__ctype_b_loc())
            .offset(*p as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int != 0
    {
        p = p.offset(1);
    }
    if *p as ::core::ffi::c_int != 's' as i32 && *p as ::core::ffi::c_int != 'S' as i32 {
        mlwrite(
            b"Expression must begin with s/\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return FALSE;
    }
    p = p.offset(1);
    let fresh15 = p;
    p = p.offset(1);
    let mut delim: ::core::ffi::c_char = *fresh15;
    if delim as ::core::ffi::c_int == '\0' as i32 {
        mlwrite(b"Missing delimiter\0" as *const u8 as *const ::core::ffi::c_char);
        return FALSE;
    }
    if read_sed_chunk(
        &raw mut p,
        delim,
        pattern,
        pat_sz,
        b"pattern\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        return FALSE;
    }
    if *pattern.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == '\0' as i32
    {
        mlwrite(b"Empty pattern\0" as *const u8 as *const ::core::ffi::c_char);
        return FALSE;
    }
    if read_sed_chunk(
        &raw mut p,
        delim,
        replacement,
        rep_sz,
        b"replacement\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        return FALSE;
    }
    while *p != 0 {
        let fresh16 = p;
        p = p.offset(1);
        let mut c: ::core::ffi::c_uchar = *fresh16 as ::core::ffi::c_uchar;
        if *(*__ctype_b_loc()).offset(c as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int != 0
        {
            continue;
        }
        if c as ::core::ffi::c_int == 'g' as i32 || c as ::core::ffi::c_int == 'G' as i32
        {
            global = TRUE;
        } else if c as ::core::ffi::c_int == 'i' as i32
            || c as ::core::ffi::c_int == 'I' as i32
        {
            caseless = TRUE;
        } else {
            mlwrite(
                b"Unknown flag '%c'\0" as *const u8 as *const ::core::ffi::c_char,
                c as ::core::ffi::c_int,
            );
            return FALSE;
        }
    }
    if !strchr(pattern, '\n' as i32).is_null() {
        mlwrite(
            b"Multi-line patterns are not supported\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return FALSE;
    }
    if !strchr(replacement, '\n' as i32).is_null() {
        mlwrite(
            b"Multi-line replacements are not supported\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return FALSE;
    }
    *is_global = global;
    *is_caseless = caseless;
    return TRUE;
}
unsafe extern "C" fn utf8_advance(
    mut text: *const ::core::ffi::c_char,
    mut len: size_t,
    mut offset: size_t,
) -> size_t {
    if offset >= len {
        return len;
    }
    let mut next: size_t = offset.wrapping_add(1 as size_t);
    while next < len
        && *text.offset(next as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int
            & 0xc0 as ::core::ffi::c_int == 0x80 as ::core::ffi::c_int
    {
        next = next.wrapping_add(1);
    }
    return next;
}
unsafe extern "C" fn build_preview(
    mut text: *const ::core::ffi::c_char,
    mut len: size_t,
    mut dest: *mut ::core::ffi::c_char,
    mut dest_sz: size_t,
) {
    static mut hex: [::core::ffi::c_char; 17] = unsafe {
        ::core::mem::transmute::<
            [u8; 17],
            [::core::ffi::c_char; 17],
        >(*b"0123456789ABCDEF\0")
    };
    let mut di: size_t = 0 as size_t;
    let mut i: size_t = 0 as size_t;
    if dest_sz == 0 as size_t {
        return;
    }
    while i < len && di < dest_sz.wrapping_sub(1 as size_t) {
        let fresh2 = i;
        i = i.wrapping_add(1);
        let mut c: ::core::ffi::c_uchar = *text.offset(fresh2 as isize)
            as ::core::ffi::c_uchar;
        if c as ::core::ffi::c_int == '\n' as i32 {
            if di.wrapping_add(2 as size_t) >= dest_sz {
                break;
            }
            let fresh3 = di;
            di = di.wrapping_add(1);
            *dest.offset(fresh3 as isize) = '\\' as i32 as ::core::ffi::c_char;
            let fresh4 = di;
            di = di.wrapping_add(1);
            *dest.offset(fresh4 as isize) = 'n' as i32 as ::core::ffi::c_char;
        } else if c as ::core::ffi::c_int == '\t' as i32 {
            if di.wrapping_add(2 as size_t) >= dest_sz {
                break;
            }
            let fresh5 = di;
            di = di.wrapping_add(1);
            *dest.offset(fresh5 as isize) = '\\' as i32 as ::core::ffi::c_char;
            let fresh6 = di;
            di = di.wrapping_add(1);
            *dest.offset(fresh6 as isize) = 't' as i32 as ::core::ffi::c_char;
        } else if (c as ::core::ffi::c_int) < 0x20 as ::core::ffi::c_int
            || c as ::core::ffi::c_int == 0x7f as ::core::ffi::c_int
        {
            if di.wrapping_add(4 as size_t) >= dest_sz {
                break;
            }
            let fresh7 = di;
            di = di.wrapping_add(1);
            *dest.offset(fresh7 as isize) = '\\' as i32 as ::core::ffi::c_char;
            let fresh8 = di;
            di = di.wrapping_add(1);
            *dest.offset(fresh8 as isize) = 'x' as i32 as ::core::ffi::c_char;
            let fresh9 = di;
            di = di.wrapping_add(1);
            *dest.offset(fresh9 as isize) = hex[(c as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as usize];
            let fresh10 = di;
            di = di.wrapping_add(1);
            *dest.offset(fresh10 as isize) = hex[(c as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int) as usize];
        } else {
            let fresh11 = di;
            di = di.wrapping_add(1);
            *dest.offset(fresh11 as isize) = c as ::core::ffi::c_char;
        }
    }
    if i < len && di.wrapping_add(4 as size_t) < dest_sz {
        let fresh12 = di;
        di = di.wrapping_add(1);
        *dest.offset(fresh12 as isize) = '.' as i32 as ::core::ffi::c_char;
        let fresh13 = di;
        di = di.wrapping_add(1);
        *dest.offset(fresh13 as isize) = '.' as i32 as ::core::ffi::c_char;
        let fresh14 = di;
        di = di.wrapping_add(1);
        *dest.offset(fresh14 as isize) = '.' as i32 as ::core::ffi::c_char;
    }
    *dest.offset(di as isize) = '\0' as i32 as ::core::ffi::c_char;
}
unsafe extern "C" fn splice_text(
    mut text: *mut ::core::ffi::c_char,
    mut text_len: size_t,
    mut start: size_t,
    mut end: size_t,
    mut replacement: *const ::core::ffi::c_char,
    mut repl_len: size_t,
    mut new_len: *mut size_t,
) -> *mut ::core::ffi::c_char {
    let mut tail: size_t = text_len.wrapping_sub(end);
    let mut next_len: size_t = text_len
        .wrapping_sub(end.wrapping_sub(start))
        .wrapping_add(repl_len);
    let mut result: *mut ::core::ffi::c_char = malloc(next_len.wrapping_add(1 as size_t))
        as *mut ::core::ffi::c_char;
    if result.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if start > 0 as size_t {
        memcpy(
            result as *mut ::core::ffi::c_void,
            text as *const ::core::ffi::c_void,
            start,
        );
    }
    if repl_len != 0 {
        memcpy(
            result.offset(start as isize) as *mut ::core::ffi::c_void,
            replacement as *const ::core::ffi::c_void,
            repl_len,
        );
    }
    if tail != 0 {
        memcpy(
            result.offset(start as isize).offset(repl_len as isize)
                as *mut ::core::ffi::c_void,
            text.offset(end as isize) as *const ::core::ffi::c_void,
            tail,
        );
    }
    *result.offset(next_len as isize) = '\0' as i32 as ::core::ffi::c_char;
    free(text as *mut ::core::ffi::c_void);
    *new_len = next_len;
    return result;
}
unsafe extern "C" fn line_index_from_top(mut target: *mut line) -> ::core::ffi::c_int {
    let mut lp: *mut line = (*(*curbp).b_linep).l_fp;
    let mut idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while lp != (*curbp).b_linep {
        if lp == target {
            return idx;
        }
        lp = (*lp).l_fp;
        idx += 1;
    }
    return -(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn restore_cursor_to_index(
    mut index: ::core::ffi::c_int,
    mut offset: ::core::ffi::c_int,
) {
    let mut lp: *mut line = (*(*curbp).b_linep).l_fp;
    let mut idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while lp != (*curbp).b_linep && idx < index {
        lp = (*lp).l_fp;
        idx += 1;
    }
    if lp == (*curbp).b_linep {
        let mut last: *mut line = (*lp).l_bp;
        if last != (*curbp).b_linep {
            lp = last;
        }
    }
    (*curwp).w_dotp = lp;
    if lp != (*curbp).b_linep {
        if offset > (*lp).l_used {
            offset = (*lp).l_used;
        }
        (*curwp).w_doto = offset;
    } else {
        (*curwp).w_doto = 0 as ::core::ffi::c_int;
    }
    (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | WFMOVE)
        as ::core::ffi::c_char;
}
unsafe extern "C" fn restore_saved_cursor(
    mut index: ::core::ffi::c_int,
    mut offset: ::core::ffi::c_int,
) {
    if index < 0 as ::core::ffi::c_int {
        (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | WFMOVE)
            as ::core::ffi::c_char;
        return;
    }
    restore_cursor_to_index(index, offset);
}
unsafe extern "C" fn command_mode_total_lines() -> ::core::ffi::c_int {
    let mut total: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut lp: *mut line = (*(*curbp).b_linep).l_fp;
    while lp != (*curbp).b_linep {
        total += 1;
        lp = (*lp).l_fp;
    }
    return total;
}
unsafe extern "C" fn command_mode_line_at_number(
    mut number: ::core::ffi::c_int,
) -> *mut line {
    let mut lp: *mut line = (*(*curbp).b_linep).l_fp;
    let mut idx: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while lp != (*curbp).b_linep && idx < number {
        lp = (*lp).l_fp;
        idx += 1;
    }
    if lp == (*curbp).b_linep {
        return ::core::ptr::null_mut::<line>();
    }
    return lp;
}
unsafe extern "C" fn command_mode_parse_line_range(
    mut text: *const ::core::ffi::c_char,
    mut start_line: *mut ::core::ffi::c_int,
    mut end_line: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if text.is_null() || start_line.is_null() || end_line.is_null() {
        return FALSE;
    }
    let mut buffer: [::core::ffi::c_char; 256] = [0; 256];
    mystrscpy(
        &raw mut buffer as *mut ::core::ffi::c_char,
        text,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as ::core::ffi::c_int,
    );
    let mut dash: *mut ::core::ffi::c_char = strchr(
        &raw mut buffer as *mut ::core::ffi::c_char,
        '-' as i32,
    );
    if dash.is_null() {
        return FALSE;
    }
    *dash = '\0' as i32 as ::core::ffi::c_char;
    let mut left: *mut ::core::ffi::c_char = &raw mut buffer as *mut ::core::ffi::c_char;
    let mut right: *mut ::core::ffi::c_char = dash
        .offset(1 as ::core::ffi::c_int as isize);
    command_mode_trim(left);
    command_mode_trim(right);
    if *left as ::core::ffi::c_int == '\0' as i32
        || *right as ::core::ffi::c_int == '\0' as i32
    {
        return FALSE;
    }
    let mut endptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut start: ::core::ffi::c_long = strtol(
        left,
        &raw mut endptr,
        10 as ::core::ffi::c_int,
    );
    if *endptr as ::core::ffi::c_int != '\0' as i32 {
        return FALSE;
    }
    let mut end: ::core::ffi::c_long = strtol(
        right,
        &raw mut endptr,
        10 as ::core::ffi::c_int,
    );
    if *endptr as ::core::ffi::c_int != '\0' as i32 {
        return FALSE;
    }
    if start > end {
        let mut tmp: ::core::ffi::c_long = start;
        start = end;
        end = tmp;
    }
    if start < 1 as ::core::ffi::c_long {
        start = 1 as ::core::ffi::c_long;
    }
    if end < 1 as ::core::ffi::c_long {
        end = 1 as ::core::ffi::c_long;
    }
    *start_line = start as ::core::ffi::c_int;
    *end_line = end as ::core::ffi::c_int;
    return TRUE;
}
unsafe extern "C" fn command_mode_apply_indent_range(
    mut start_line: ::core::ffi::c_int,
    mut end_line: ::core::ffi::c_int,
    mut indent_direction: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut total: ::core::ffi::c_int = command_mode_total_lines();
    if total <= 0 as ::core::ffi::c_int {
        mlwrite(b"Buffer is empty\0" as *const u8 as *const ::core::ffi::c_char);
        return FALSE;
    }
    if start_line < 1 as ::core::ffi::c_int {
        start_line = 1 as ::core::ffi::c_int;
    }
    if end_line < 1 as ::core::ffi::c_int {
        end_line = 1 as ::core::ffi::c_int;
    }
    if start_line > total {
        start_line = total;
    }
    if end_line > total {
        end_line = total;
    }
    if start_line > end_line {
        let mut tmp: ::core::ffi::c_int = start_line;
        start_line = end_line;
        end_line = tmp;
    }
    let mut start_lp: *mut line = command_mode_line_at_number(start_line);
    let mut end_lp: *mut line = command_mode_line_at_number(end_line);
    if start_lp.is_null() || end_lp.is_null() {
        mlwrite(b"Invalid line range\0" as *const u8 as *const ::core::ffi::c_char);
        return FALSE;
    }
    let mut saved_start: *mut line = indent_start_lp;
    let mut saved_end: *mut line = indent_end_lp;
    let mut saved_type: ::core::ffi::c_int = indent_range_type;
    let mut saved_active: ::core::ffi::c_int = indent_selection_active;
    indent_start_lp = start_lp;
    indent_end_lp = end_lp;
    indent_range_type = indent_direction;
    indent_selection_active = FALSE;
    let mut status: ::core::ffi::c_int = indent_apply_range(
        FALSE,
        1 as ::core::ffi::c_int,
    );
    indent_start_lp = saved_start;
    indent_end_lp = saved_end;
    indent_range_type = saved_type;
    indent_selection_active = saved_active;
    return status;
}
unsafe extern "C" fn block_visual_column(
    mut lp: *mut line,
    mut offset: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut col: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut len: ::core::ffi::c_int = if !lp.is_null() {
        (*lp).l_used
    } else {
        0 as ::core::ffi::c_int
    };
    while !lp.is_null() && idx < offset && idx < len {
        let mut c: unicode_t = 0;
        let mut bytes: ::core::ffi::c_int = utf8_to_unicode(
            &raw mut (*lp).l_text as *mut ::core::ffi::c_uchar,
            idx as ::core::ffi::c_uint,
            len as ::core::ffi::c_uint,
            &raw mut c,
        ) as ::core::ffi::c_int;
        if bytes <= 0 as ::core::ffi::c_int {
            break;
        }
        col = next_column(col, c, tab_width);
        idx += bytes;
    }
    return col;
}
unsafe extern "C" fn block_bounds(
    mut top: *mut ::core::ffi::c_int,
    mut bottom: *mut ::core::ffi::c_int,
    mut left: *mut ::core::ffi::c_int,
    mut right: *mut ::core::ffi::c_int,
) {
    let mut anchor: ::core::ffi::c_int = line_index_from_top(block_anchor_line);
    let mut cursor: ::core::ffi::c_int = line_index_from_top((*curwp).w_dotp);
    let mut anchor_col: ::core::ffi::c_int = block_visual_column(
        block_anchor_line,
        block_anchor_offset,
    );
    let mut cursor_col: ::core::ffi::c_int = block_visual_column(
        (*curwp).w_dotp,
        (*curwp).w_doto,
    );
    if !top.is_null() {
        *top = if anchor < cursor { anchor } else { cursor };
    }
    if !bottom.is_null() {
        *bottom = if anchor > cursor { anchor } else { cursor };
    }
    if !left.is_null() {
        *left = if anchor_col < cursor_col { anchor_col } else { cursor_col };
    }
    if !right.is_null() {
        *right = if anchor_col > cursor_col { anchor_col } else { cursor_col };
    }
}
unsafe extern "C" fn line_offset_for_column(
    mut lp: *mut line,
    mut target_col: ::core::ffi::c_int,
    mut actual_col: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = (*lp).l_used;
    let mut idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut col: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while idx < len {
        let mut c: unicode_t = 0;
        let mut bytes: ::core::ffi::c_int = utf8_to_unicode(
            &raw mut (*lp).l_text as *mut ::core::ffi::c_uchar,
            idx as ::core::ffi::c_uint,
            len as ::core::ffi::c_uint,
            &raw mut c,
        ) as ::core::ffi::c_int;
        let mut next_col: ::core::ffi::c_int = 0;
        if bytes <= 0 as ::core::ffi::c_int {
            break;
        }
        next_col = next_column(col, c, tab_width);
        if next_col > target_col {
            break;
        }
        idx += bytes;
        col = next_col;
    }
    if !actual_col.is_null() {
        *actual_col = col;
    }
    return idx;
}
unsafe extern "C" fn render_block_status() {
    let mut status: [::core::ffi::c_char; 96] = [0; 96];
    let mut top: ::core::ffi::c_int = 0;
    let mut bottom: ::core::ffi::c_int = 0;
    let mut left: ::core::ffi::c_int = 0;
    let mut right: ::core::ffi::c_int = 0;
    if block_active == 0 {
        return;
    }
    block_bounds(&raw mut top, &raw mut bottom, &raw mut left, &raw mut right);
    snprintf(
        &raw mut status as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 96]>() as size_t,
        b"%s lines %d-%d cols %d-%d\0" as *const u8 as *const ::core::ffi::c_char,
        if block_replace != 0 {
            b"viblock-replace\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"viblock-edit\0" as *const u8 as *const ::core::ffi::c_char
        },
        top + 1 as ::core::ffi::c_int,
        bottom + 1 as ::core::ffi::c_int,
        left + 1 as ::core::ffi::c_int,
        right + 1 as ::core::ffi::c_int,
    );
    command_mode_draw_status(
        &raw mut status as *mut ::core::ffi::c_char,
        b"[move cursor, Enter apply, Esc cancel]\0" as *const u8
            as *const ::core::ffi::c_char,
        false_0 != 0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn command_mode_block_is_active() -> ::core::ffi::c_int {
    return block_active;
}
#[no_mangle]
pub unsafe extern "C" fn command_mode_block_selection_contains(
    mut lp: *mut line,
    mut col_start: ::core::ffi::c_int,
    mut col_end: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut top: ::core::ffi::c_int = 0;
    let mut bottom: ::core::ffi::c_int = 0;
    let mut left: ::core::ffi::c_int = 0;
    let mut right: ::core::ffi::c_int = 0;
    let mut line_idx: ::core::ffi::c_int = 0;
    if block_active == 0 || lp.is_null() {
        return FALSE;
    }
    block_bounds(&raw mut top, &raw mut bottom, &raw mut left, &raw mut right);
    line_idx = line_index_from_top(lp);
    if line_idx < top || line_idx > bottom {
        return FALSE;
    }
    if right == left {
        right += 1;
    }
    return (col_start < right && col_end > left) as ::core::ffi::c_int;
}
unsafe extern "C" fn block_motion_allowed(mut func: fn_t) -> ::core::ffi::c_int {
    return (func
        == Some(
            backchar
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
        || func
            == Some(
                forwchar
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
        || func
            == Some(
                backline
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
        || func
            == Some(
                forwline
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
        || func
            == Some(
                gotobol
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
        || func
            == Some(
                gotoeol
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
        || func
            == Some(
                gotobob
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
        || func
            == Some(
                gotoeob
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
        || func
            == Some(
                backpage
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
        || func
            == Some(
                forwpage
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )) as ::core::ffi::c_int;
}
unsafe extern "C" fn block_apply_text(
    mut text: *const ::core::ffi::c_char,
    mut replace_mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut top: ::core::ffi::c_int = 0;
    let mut bottom: ::core::ffi::c_int = 0;
    let mut left: ::core::ffi::c_int = 0;
    let mut right: ::core::ffi::c_int = 0;
    let mut original_index: ::core::ffi::c_int = line_index_from_top((*curwp).w_dotp);
    let mut original_offset: ::core::ffi::c_int = (*curwp).w_doto;
    let mut lp: *mut line = (*(*curbp).b_linep).l_fp;
    let mut idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    block_bounds(&raw mut top, &raw mut bottom, &raw mut left, &raw mut right);
    while lp != (*curbp).b_linep {
        let mut next: *mut line = (*lp).l_fp;
        if idx >= top && idx <= bottom {
            let mut actual_col: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut start_offset: ::core::ffi::c_int = 0;
            (*curwp).w_dotp = lp;
            (*curwp).w_doto = 0 as ::core::ffi::c_int;
            start_offset = line_offset_for_column(lp, left, &raw mut actual_col);
            (*curwp).w_doto = start_offset;
            while actual_col < left {
                if linsert(1 as ::core::ffi::c_int, ' ' as i32) != TRUE {
                    return FALSE;
                }
                actual_col += 1;
            }
            if replace_mode != 0 {
                let mut end_actual_col: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut end_offset: ::core::ffi::c_int = 0;
                end_offset = line_offset_for_column(
                    (*curwp).w_dotp,
                    right,
                    &raw mut end_actual_col,
                );
                (*curwp).w_doto = end_offset;
                while end_actual_col < right {
                    if linsert(1 as ::core::ffi::c_int, ' ' as i32) != TRUE {
                        return FALSE;
                    }
                    end_actual_col += 1;
                }
                (*curwp).w_doto = start_offset;
                if ldelete(
                    (line_offset_for_column(
                        (*curwp).w_dotp,
                        right,
                        ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    ) - start_offset) as ::core::ffi::c_long,
                    FALSE,
                ) != TRUE
                {
                    return FALSE;
                }
            } else {
                (*curwp).w_doto = start_offset;
            }
            if !text.is_null() && *text as ::core::ffi::c_int != 0 {
                if linsert_block(text, strlen(text) as ::core::ffi::c_int) != TRUE {
                    return FALSE;
                }
            }
        }
        lp = next;
        idx += 1;
    }
    restore_saved_cursor(original_index, original_offset);
    (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | (WFHARD | WFMODE))
        as ::core::ffi::c_char;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn command_mode_block_handle_key(
    mut c: ::core::ffi::c_int,
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut text: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut func: fn_t = None;
    let mut status: ::core::ffi::c_int = 0;
    if block_active == 0 {
        return FALSE;
    }
    match c {
        13 | 10 | 268435533 => {
            status = minibuf_input(
                if block_replace != 0 {
                    b"viblock replace: \0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"viblock edit: \0" as *const u8 as *const ::core::ffi::c_char
                },
                &raw mut text as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 1024]>()
                    as ::core::ffi::c_int,
            );
            if status == TRUE {
                if block_apply_text(
                    &raw mut text as *mut ::core::ffi::c_char,
                    block_replace,
                ) == 0
                {
                    return FALSE;
                }
                mlwrite(
                    b"%s applied\0" as *const u8 as *const ::core::ffi::c_char,
                    if block_replace != 0 {
                        b"viblock replace\0" as *const u8 as *const ::core::ffi::c_char
                    } else {
                        b"viblock edit\0" as *const u8 as *const ::core::ffi::c_char
                    },
                );
            } else {
                mlwrite(
                    b"%s cancelled\0" as *const u8 as *const ::core::ffi::c_char,
                    if block_replace != 0 {
                        b"viblock replace\0" as *const u8 as *const ::core::ffi::c_char
                    } else {
                        b"viblock edit\0" as *const u8 as *const ::core::ffi::c_char
                    },
                );
            }
            block_active = 0 as ::core::ffi::c_int;
            block_anchor_line = ::core::ptr::null_mut::<line>();
            block_anchor_offset = 0 as ::core::ffi::c_int;
            return TRUE;
        }
        27 | 268435527 => {
            block_active = 0 as ::core::ffi::c_int;
            block_anchor_line = ::core::ptr::null_mut::<line>();
            block_anchor_offset = 0 as ::core::ffi::c_int;
            (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | (WFHARD | WFMODE))
                as ::core::ffi::c_char;
            mlwrite(
                b"%s cancelled\0" as *const u8 as *const ::core::ffi::c_char,
                if block_replace != 0 {
                    b"viblock replace\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"viblock edit\0" as *const u8 as *const ::core::ffi::c_char
                },
            );
            return TRUE;
        }
        _ => {
            func = getbind(c);
            if block_motion_allowed(func) == 0 {
                render_block_status();
                return TRUE;
            }
            execute(c, f, n);
            (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | (WFHARD | WFMODE))
                as ::core::ffi::c_char;
            render_block_status();
            return TRUE;
        }
    };
}
unsafe extern "C" fn apply_regex_to_line(
    mut lp: *mut line,
    mut code: *mut pcre2_code_8,
    mut match_data: *mut pcre2_match_data_8,
    mut replacement: *const ::core::ffi::c_char,
    mut repl_len: size_t,
    mut is_global: ::core::ffi::c_int,
    mut total_count: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut text: *mut ::core::ffi::c_char = malloc(
        ((*lp).l_used + 1 as ::core::ffi::c_int) as size_t,
    ) as *mut ::core::ffi::c_char;
    let mut text_len: size_t = (*lp).l_used as size_t;
    let mut search_offset: size_t = 0 as size_t;
    let mut changed: ::core::ffi::c_int = FALSE;
    if text.is_null() {
        mlwrite(b"%%Out of memory\0" as *const u8 as *const ::core::ffi::c_char);
        return FALSE;
    }
    memcpy(
        text as *mut ::core::ffi::c_void,
        &raw mut (*lp).l_text as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
        text_len,
    );
    *text.offset(text_len as isize) = '\0' as i32 as ::core::ffi::c_char;
    while search_offset <= text_len {
        let mut rc: ::core::ffi::c_int = pcre2_match_8(
            code,
            text as PCRE2_SPTR8,
            text_len,
            search_offset,
            0 as uint32_t,
            match_data,
            ::core::ptr::null_mut::<pcre2_match_context_8>(),
        );
        if rc < 0 as ::core::ffi::c_int {
            break;
        }
        let mut ovector: *mut size_t = pcre2_get_ovector_pointer_8(match_data);
        let mut match_start: size_t = *ovector.offset(0 as ::core::ffi::c_int as isize);
        let mut match_end: size_t = *ovector.offset(1 as ::core::ffi::c_int as isize);
        let mut zero_width: ::core::ffi::c_int = (match_start == match_end)
            as ::core::ffi::c_int;
        let mut accept: ::core::ffi::c_int = is_global;
        if is_global == 0 {
            let mut match_preview: [::core::ffi::c_char; 48] = [0; 48];
            let mut repl_preview: [::core::ffi::c_char; 48] = [0; 48];
            let mut prompt: [::core::ffi::c_char; 128] = [0; 128];
            build_preview(
                text.offset(match_start as isize),
                match_end.wrapping_sub(match_start),
                &raw mut match_preview as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 48]>() as size_t,
            );
            build_preview(
                replacement,
                repl_len,
                &raw mut repl_preview as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 48]>() as size_t,
            );
            snprintf(
                &raw mut prompt as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
                b"Replace '%s' with '%s'\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut match_preview as *mut ::core::ffi::c_char,
                &raw mut repl_preview as *mut ::core::ffi::c_char,
            );
            accept = mlyesno(&raw mut prompt as *mut ::core::ffi::c_char);
        }
        let mut next_offset: size_t = match_end;
        if accept == TRUE {
            changed = TRUE;
            *total_count += 1;
            let mut new_len: size_t = 0;
            let mut new_text: *mut ::core::ffi::c_char = splice_text(
                text,
                text_len,
                match_start,
                match_end,
                replacement,
                repl_len,
                &raw mut new_len,
            );
            if new_text.is_null() {
                mlwrite(b"%%Out of memory\0" as *const u8 as *const ::core::ffi::c_char);
                free(text as *mut ::core::ffi::c_void);
                return FALSE;
            }
            text = new_text;
            text_len = new_len;
            next_offset = match_start.wrapping_add(repl_len);
        }
        if zero_width != 0 && next_offset == match_start {
            next_offset = utf8_advance(text, text_len, match_start);
            if next_offset == match_start {
                next_offset = next_offset.wrapping_add(1);
            }
        }
        if next_offset > text_len {
            break;
        }
        search_offset = next_offset;
    }
    if changed != 0 {
        (*curwp).w_dotp = lp;
        (*curwp).w_doto = 0 as ::core::ffi::c_int;
        ldelete((*lp).l_used as ::core::ffi::c_long, FALSE);
        if text_len > 0 as size_t {
            linsert_block(text, text_len as ::core::ffi::c_int);
        }
    }
    free(text as *mut ::core::ffi::c_void);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn sed_replace_command(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut expr: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut pattern: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut replacement: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut is_global: ::core::ffi::c_int = FALSE;
    let mut is_caseless: ::core::ffi::c_int = FALSE;
    let mut status: ::core::ffi::c_int = 0;
    let mut total: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut original_index: ::core::ffi::c_int = 0;
    let mut original_offset: ::core::ffi::c_int = 0;
    let mut code: *mut pcre2_code_8 = ::core::ptr::null_mut::<pcre2_code_8>();
    let mut match_data: *mut pcre2_match_data_8 = ::core::ptr::null_mut::<
        pcre2_match_data_8,
    >();
    let mut errornumber: ::core::ffi::c_int = 0;
    let mut erroffset: size_t = 0;
    let mut options: uint32_t = PCRE2_UTF as uint32_t | PCRE2_UCP as uint32_t
        | PCRE2_MULTILINE as uint32_t;
    let mut repl_len: size_t = 0;
    if (*curbp).b_mode & MDVIEW != 0 {
        let mut ro: ::core::ffi::c_int = rdonly();
        nanox_request_underbar_redraw();
        return ro;
    }
    status = minibuf_input(
        b"sed replace: \0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut expr as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as ::core::ffi::c_int,
    );
    if status != TRUE {
        nanox_request_underbar_redraw();
        return status;
    }
    if parse_sed_expression(
        &raw mut expr as *mut ::core::ffi::c_char,
        &raw mut pattern as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
        &raw mut replacement as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
        &raw mut is_global,
        &raw mut is_caseless,
    ) == 0
    {
        nanox_request_underbar_redraw();
        return FALSE;
    }
    if is_caseless != 0 {
        options = (options as ::core::ffi::c_uint | PCRE2_CASELESS) as uint32_t;
    }
    code = pcre2_compile_8(
        &raw mut pattern as *mut ::core::ffi::c_char as PCRE2_SPTR8,
        PCRE2_ZERO_TERMINATED,
        options,
        &raw mut errornumber,
        &raw mut erroffset,
        ::core::ptr::null_mut::<pcre2_compile_context_8>(),
    );
    if code.is_null() {
        let mut errbuf: [::core::ffi::c_char; 128] = [0; 128];
        pcre2_get_error_message_8(
            errornumber,
            &raw mut errbuf as *mut ::core::ffi::c_char as *mut PCRE2_UCHAR8,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
        );
        mlwrite(
            b"Regex error at %d: %s\0" as *const u8 as *const ::core::ffi::c_char,
            erroffset as ::core::ffi::c_int,
            &raw mut errbuf as *mut ::core::ffi::c_char,
        );
        nanox_request_underbar_redraw();
        return FALSE;
    }
    match_data = pcre2_match_data_create_from_pattern_8(
        code,
        ::core::ptr::null_mut::<pcre2_general_context_8>(),
    );
    if match_data.is_null() {
        pcre2_code_free_8(code);
        mlwrite(b"%%Out of memory\0" as *const u8 as *const ::core::ffi::c_char);
        nanox_request_underbar_redraw();
        return FALSE;
    }
    original_index = line_index_from_top((*curwp).w_dotp);
    original_offset = (*curwp).w_doto;
    repl_len = strlen(&raw mut replacement as *mut ::core::ffi::c_char);
    let mut lp: *mut line = (*(*curbp).b_linep).l_fp;
    while lp != (*curbp).b_linep {
        let mut next: *mut line = (*lp).l_fp;
        if apply_regex_to_line(
            lp,
            code,
            match_data,
            &raw mut replacement as *mut ::core::ffi::c_char,
            repl_len,
            is_global,
            &raw mut total,
        ) == 0
        {
            pcre2_match_data_free_8(match_data);
            pcre2_code_free_8(code);
            restore_saved_cursor(original_index, original_offset);
            nanox_request_underbar_redraw();
            return FALSE;
        }
        lp = next;
    }
    pcre2_match_data_free_8(match_data);
    pcre2_code_free_8(code);
    restore_saved_cursor(original_index, original_offset);
    if total == 0 as ::core::ffi::c_int {
        mlwrite(b"No matches for pattern\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        mlwrite(
            b"Replaced %d occurrence%s\0" as *const u8 as *const ::core::ffi::c_char,
            total,
            if total == 1 as ::core::ffi::c_int {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"s\0" as *const u8 as *const ::core::ffi::c_char
            },
        );
    }
    nanox_request_underbar_redraw();
    return TRUE;
}
#[inline]
unsafe extern "C" fn mystrscpy(
    mut dst: *mut ::core::ffi::c_char,
    mut src: *const ::core::ffi::c_char,
    mut size: ::core::ffi::c_int,
) {
    if size == 0 {
        return;
    }
    loop {
        size -= 1;
        if !(size != 0) {
            break;
        }
        let fresh0 = src;
        src = src.offset(1);
        let mut c: ::core::ffi::c_char = *fresh0;
        if c == 0 {
            break;
        }
        let fresh1 = dst;
        dst = dst.offset(1);
        *fresh1 = c;
    }
    *dst = 0 as ::core::ffi::c_char;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
