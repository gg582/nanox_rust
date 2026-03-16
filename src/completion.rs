extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fgets(
        __s: *mut ::core::ffi::c_char,
        __n: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn pclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn popen(
        __command: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn iswalnum(__wc: wint_t) -> ::core::ffi::c_int;
    fn closedir(__dirp: *mut DIR) -> ::core::ffi::c_int;
    fn opendir(__name: *const ::core::ffi::c_char) -> *mut DIR;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    static mut term: *mut terminal;
    fn vttputc(c: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn vttflush();
    fn vtteeol();
    fn vttbeep();
    fn vttrev(state: ::core::ffi::c_int);
    fn vttsetcolors(fg: ::core::ffi::c_int, bg: ::core::ffi::c_int);
    fn vttsetattrs(
        bold: ::core::ffi::c_int,
        underline: ::core::ffi::c_int,
        italic: ::core::ffi::c_int,
    );
    static mut currow: ::core::ffi::c_int;
    static mut curcol: ::core::ffi::c_int;
    static mut curwp: *mut window;
    static mut curbp: *mut buffer;
    static mut bheadp: *mut buffer;
    static mut ttrow: ::core::ffi::c_int;
    static mut ttcol: ::core::ffi::c_int;
    static mut file_reserve: [[::core::ffi::c_char; 4096]; 64];
    fn utf8_to_unicode(
        line: *mut ::core::ffi::c_uchar,
        index: ::core::ffi::c_uint,
        len: ::core::ffi::c_uint,
        res: *mut unicode_t,
    ) -> ::core::ffi::c_uint;
    fn unicode_width(c: unicode_t) -> ::core::ffi::c_int;
    fn movecursor(row: ::core::ffi::c_int, col: ::core::ffi::c_int);
    fn colorscheme_get(id: HighlightStyleID) -> HighlightStyle;
    fn highlight_get_profile(
        filename: *const ::core::ffi::c_char,
    ) -> *const HighlightProfile;
    fn linsert_block(
        block: *const ::core::ffi::c_char,
        len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ldelete(n: ::core::ffi::c_long, kflag: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn scraper_iterate_symbols(
        lang: scraper_lang_t,
        module: *const ::core::ffi::c_char,
        cb: scraper_symbol_cb,
        userdata: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
pub type wint_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: ::core::ffi::c_ushort,
    pub d_type: ::core::ffi::c_uchar,
    pub d_name: [::core::ffi::c_char; 256],
}
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::core::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
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
pub struct HighlightStyle {
    pub fg: ::core::ffi::c_int,
    pub bg: ::core::ffi::c_int,
    pub bold: bool,
    pub underline: bool,
    pub italic: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockCommentPair {
    pub start: [::core::ffi::c_char; 64],
    pub end: [::core::ffi::c_char; 64],
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
pub type completion_context_t = ::core::ffi::c_uint;
pub const COMPLETION_CONTEXT_PATH: completion_context_t = 1;
pub const COMPLETION_CONTEXT_DEFAULT: completion_context_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct completion_state_t {
    pub matches: [*mut ::core::ffi::c_char; 100],
    pub count: ::core::ffi::c_int,
    pub selected_index: ::core::ffi::c_int,
    pub is_visible: ::core::ffi::c_int,
    pub scroll_offset: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct completion_dropdown_state_t {
    pub active: ::core::ffi::c_int,
    pub prefix_len: size_t,
    pub popup_row: ::core::ffi::c_int,
    pub popup_col: ::core::ffi::c_int,
    pub popup_width: ::core::ffi::c_int,
    pub popup_height: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct completion_preview_state_t {
    pub active: ::core::ffi::c_int,
    pub line: *mut line,
    pub start_offset: ::core::ffi::c_int,
    pub prefix_len: ::core::ffi::c_int,
    pub last_tail_len: ::core::ffi::c_int,
}
pub type scraper_lang_t = ::core::ffi::c_uint;
pub const SCRAPER_LANG_COUNT: scraper_lang_t = 2;
pub const SCRAPER_LANG_NODE: scraper_lang_t = 1;
pub const SCRAPER_LANG_PYTHON: scraper_lang_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct runtime_completion_ctx_t {
    pub prefix: *const ::core::ffi::c_char,
}
pub type scraper_symbol_cb = Option<
    unsafe extern "C" fn(*const ::core::ffi::c_char, *mut ::core::ffi::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct completion_pool_t {
    pub items: *mut *mut ::core::ffi::c_char,
    pub count: ::core::ffi::c_int,
    pub capacity: ::core::ffi::c_int,
    pub max_items: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct java_member_entry_t {
    pub class_name: *mut ::core::ffi::c_char,
    pub members: completion_pool_t,
    pub loaded: ::core::ffi::c_int,
}
pub const ST_NORMAL: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const ST_TEMPLATE: C2RustUnnamed_0 = 5;
pub const ST_STRING_DQ: C2RustUnnamed_0 = 4;
pub const ST_STRING_SQ: C2RustUnnamed_0 = 3;
pub const ST_BLOCK_CMT: C2RustUnnamed_0 = 2;
pub const ST_LINE_CMT: C2RustUnnamed_0 = 1;
pub const ST_NORMAL_0: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const ST_STRING3: C2RustUnnamed_1 = 3;
pub const ST_STRING1: C2RustUnnamed_1 = 2;
pub const ST_LINE_CMT_0: C2RustUnnamed_1 = 1;
pub const ST_NORMAL_1: C2RustUnnamed_2 = 0;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const ST_CHAR: C2RustUnnamed_2 = 4;
pub const ST_STRING: C2RustUnnamed_2 = 3;
pub const ST_BLOCK_CMT_0: C2RustUnnamed_2 = 2;
pub const ST_LINE_CMT_1: C2RustUnnamed_2 = 1;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const HUGE: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const CONTROL: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;
pub const SPEC: ::core::ffi::c_uint = 0x80000000 as ::core::ffi::c_uint;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WFMOVE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn is_beginning_utf8(
    mut c: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    return (c as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
        != 0x80 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
pub const MAX_COMPLETIONS: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const MAX_COMPLETION_LEN: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
#[no_mangle]
pub static mut completion_state: completion_state_t = completion_state_t {
    matches: [::core::ptr::null::<::core::ffi::c_char>()
        as *mut ::core::ffi::c_char; 100],
    count: 0,
    selected_index: 0,
    is_visible: 0,
    scroll_offset: 0,
};
static mut completion_dropdown_state: completion_dropdown_state_t = completion_dropdown_state_t {
    active: 0 as ::core::ffi::c_int,
    prefix_len: 0 as size_t,
    popup_row: 0,
    popup_col: 0,
    popup_width: 0,
    popup_height: 0,
};
static mut completion_storage: [[::core::ffi::c_char; 128]; 100] = [[0; 128]; 100];
pub const MAX_C_SYMBOLS: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const MAX_JAVA_SYMBOLS: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const MAX_JAVA_MEMBERS: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const MAX_C_SCAN_FILES: ::core::ffi::c_int = 2048 as ::core::ffi::c_int;
pub const MAX_C_SCAN_DEPTH: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MAX_C_FILE_BYTES: ::core::ffi::c_int = 32768 as ::core::ffi::c_int;
pub const MAX_JAVA_SCAN_FILES: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const MAX_JAVA_SCAN_DEPTH: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const MAX_SOURCE_SYMBOLS: ::core::ffi::c_int = 2048 as ::core::ffi::c_int;
pub const MAX_SOURCE_FILE_BYTES: ::core::ffi::c_int = 262144 as ::core::ffi::c_int;
static mut c_symbol_cache: completion_pool_t = completion_pool_t {
    items: ::core::ptr::null::<*mut ::core::ffi::c_char>()
        as *mut *mut ::core::ffi::c_char,
    count: 0 as ::core::ffi::c_int,
    capacity: 0 as ::core::ffi::c_int,
    max_items: MAX_C_SYMBOLS,
};
static mut c_include_paths: completion_pool_t = completion_pool_t {
    items: ::core::ptr::null::<*mut ::core::ffi::c_char>()
        as *mut *mut ::core::ffi::c_char,
    count: 0 as ::core::ffi::c_int,
    capacity: 0 as ::core::ffi::c_int,
    max_items: 0 as ::core::ffi::c_int,
};
static mut c_symbols_loaded: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut c_files_scanned: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut java_class_cache: completion_pool_t = completion_pool_t {
    items: ::core::ptr::null::<*mut ::core::ffi::c_char>()
        as *mut *mut ::core::ffi::c_char,
    count: 0 as ::core::ffi::c_int,
    capacity: 0 as ::core::ffi::c_int,
    max_items: MAX_JAVA_SYMBOLS,
};
static mut java_classpath_entries: completion_pool_t = completion_pool_t {
    items: ::core::ptr::null::<*mut ::core::ffi::c_char>()
        as *mut *mut ::core::ffi::c_char,
    count: 0 as ::core::ffi::c_int,
    capacity: 0 as ::core::ffi::c_int,
    max_items: 0 as ::core::ffi::c_int,
};
static mut java_classpath_loaded: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut java_symbols_loaded: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut java_files_scanned: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut java_classpath_string: [::core::ffi::c_char; 4096] = [0; 4096];
static mut completion_preview_state: completion_preview_state_t = completion_preview_state_t {
    active: 0 as ::core::ffi::c_int,
    line: ::core::ptr::null::<line>() as *mut line,
    start_offset: 0,
    prefix_len: 0,
    last_tail_len: 0,
};
static mut java_member_cache: *mut java_member_entry_t = ::core::ptr::null::<
    java_member_entry_t,
>() as *mut java_member_entry_t;
static mut java_member_cache_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut java_member_cache_capacity: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const COMPLETION_MINIBUFFER_MAX_VISIBLE: ::core::ffi::c_int = 5
    as ::core::ffi::c_int;
pub const COMPLETION_POPUP_MAX_VISIBLE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const COMPLETION_POPUP_MIN_CONTENT: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const COMPLETION_POPUP_MAX_CONTENT: ::core::ffi::c_int = 48 as ::core::ffi::c_int;
unsafe extern "C" fn completion_is_true_color(
    mut color: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (color as ::core::ffi::c_uint & 0xff000000 as ::core::ffi::c_uint
        == 0x1000000 as ::core::ffi::c_int as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
unsafe extern "C" fn completion_mul_channel(
    mut a: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return a * b / 255 as ::core::ffi::c_int;
}
unsafe extern "C" fn completion_mix_color(
    mut base: ::core::ffi::c_int,
    mut overlay: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if base == -(1 as ::core::ffi::c_int) {
        return overlay;
    }
    if overlay == -(1 as ::core::ffi::c_int) {
        return base;
    }
    if completion_is_true_color(base) != 0 && completion_is_true_color(overlay) != 0 {
        let mut br: ::core::ffi::c_int = base >> 16 as ::core::ffi::c_int
            & 0xff as ::core::ffi::c_int;
        let mut bg: ::core::ffi::c_int = base >> 8 as ::core::ffi::c_int
            & 0xff as ::core::ffi::c_int;
        let mut bb: ::core::ffi::c_int = base & 0xff as ::core::ffi::c_int;
        let mut or: ::core::ffi::c_int = overlay >> 16 as ::core::ffi::c_int
            & 0xff as ::core::ffi::c_int;
        let mut og: ::core::ffi::c_int = overlay >> 8 as ::core::ffi::c_int
            & 0xff as ::core::ffi::c_int;
        let mut ob: ::core::ffi::c_int = overlay & 0xff as ::core::ffi::c_int;
        let mut r: ::core::ffi::c_int = completion_mul_channel(br, or);
        let mut g: ::core::ffi::c_int = completion_mul_channel(bg, og);
        let mut b: ::core::ffi::c_int = completion_mul_channel(bb, ob);
        return 0x1000000 as ::core::ffi::c_int | r << 16 as ::core::ffi::c_int
            | g << 8 as ::core::ffi::c_int | b;
    }
    return overlay;
}
unsafe extern "C" fn completion_combine_style(
    mut primary: HighlightStyle,
    mut overlay: HighlightStyle,
) -> HighlightStyle {
    let mut result: HighlightStyle = primary;
    result.fg = completion_mix_color(primary.fg, overlay.fg);
    result.bg = completion_mix_color(primary.bg, overlay.bg);
    result.bold = primary.bold as ::core::ffi::c_int != 0
        || overlay.bold as ::core::ffi::c_int != 0;
    result.underline = primary.underline as ::core::ffi::c_int != 0
        || overlay.underline as ::core::ffi::c_int != 0;
    result.italic = primary.italic as ::core::ffi::c_int != 0
        || overlay.italic as ::core::ffi::c_int != 0;
    return result;
}
unsafe extern "C" fn completion_apply_style(mut style: *const HighlightStyle) {
    if style.is_null() {
        return;
    }
    vttsetcolors((*style).fg, (*style).bg);
    vttsetattrs(
        (*style).bold as ::core::ffi::c_int,
        (*style).underline as ::core::ffi::c_int,
        (*style).italic as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn completion_display_width(
    mut text: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if text.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    return utf8_display_width(text, strlen(text) as ::core::ffi::c_int);
}
static mut source_symbol_cache: completion_pool_t = completion_pool_t {
    items: ::core::ptr::null::<*mut ::core::ffi::c_char>()
        as *mut *mut ::core::ffi::c_char,
    count: 0 as ::core::ffi::c_int,
    capacity: 0 as ::core::ffi::c_int,
    max_items: MAX_SOURCE_SYMBOLS,
};
static mut source_symbol_last_fname: [::core::ffi::c_char; 2048] = [0; 2048];
static mut common_keywords: [*const ::core::ffi::c_char; 82] = [
    b"if\0" as *const u8 as *const ::core::ffi::c_char,
    b"else\0" as *const u8 as *const ::core::ffi::c_char,
    b"while\0" as *const u8 as *const ::core::ffi::c_char,
    b"for\0" as *const u8 as *const ::core::ffi::c_char,
    b"do\0" as *const u8 as *const ::core::ffi::c_char,
    b"return\0" as *const u8 as *const ::core::ffi::c_char,
    b"break\0" as *const u8 as *const ::core::ffi::c_char,
    b"continue\0" as *const u8 as *const ::core::ffi::c_char,
    b"switch\0" as *const u8 as *const ::core::ffi::c_char,
    b"case\0" as *const u8 as *const ::core::ffi::c_char,
    b"default\0" as *const u8 as *const ::core::ffi::c_char,
    b"int\0" as *const u8 as *const ::core::ffi::c_char,
    b"char\0" as *const u8 as *const ::core::ffi::c_char,
    b"float\0" as *const u8 as *const ::core::ffi::c_char,
    b"double\0" as *const u8 as *const ::core::ffi::c_char,
    b"void\0" as *const u8 as *const ::core::ffi::c_char,
    b"struct\0" as *const u8 as *const ::core::ffi::c_char,
    b"union\0" as *const u8 as *const ::core::ffi::c_char,
    b"enum\0" as *const u8 as *const ::core::ffi::c_char,
    b"typedef\0" as *const u8 as *const ::core::ffi::c_char,
    b"static\0" as *const u8 as *const ::core::ffi::c_char,
    b"extern\0" as *const u8 as *const ::core::ffi::c_char,
    b"include\0" as *const u8 as *const ::core::ffi::c_char,
    b"define\0" as *const u8 as *const ::core::ffi::c_char,
    b"ifdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"ifndef\0" as *const u8 as *const ::core::ffi::c_char,
    b"endif\0" as *const u8 as *const ::core::ffi::c_char,
    b"import\0" as *const u8 as *const ::core::ffi::c_char,
    b"from\0" as *const u8 as *const ::core::ffi::c_char,
    b"as\0" as *const u8 as *const ::core::ffi::c_char,
    b"def\0" as *const u8 as *const ::core::ffi::c_char,
    b"class\0" as *const u8 as *const ::core::ffi::c_char,
    b"try\0" as *const u8 as *const ::core::ffi::c_char,
    b"except\0" as *const u8 as *const ::core::ffi::c_char,
    b"finally\0" as *const u8 as *const ::core::ffi::c_char,
    b"with\0" as *const u8 as *const ::core::ffi::c_char,
    b"yield\0" as *const u8 as *const ::core::ffi::c_char,
    b"lambda\0" as *const u8 as *const ::core::ffi::c_char,
    b"assert\0" as *const u8 as *const ::core::ffi::c_char,
    b"pass\0" as *const u8 as *const ::core::ffi::c_char,
    b"None\0" as *const u8 as *const ::core::ffi::c_char,
    b"True\0" as *const u8 as *const ::core::ffi::c_char,
    b"False\0" as *const u8 as *const ::core::ffi::c_char,
    b"and\0" as *const u8 as *const ::core::ffi::c_char,
    b"or\0" as *const u8 as *const ::core::ffi::c_char,
    b"not\0" as *const u8 as *const ::core::ffi::c_char,
    b"is\0" as *const u8 as *const ::core::ffi::c_char,
    b"in\0" as *const u8 as *const ::core::ffi::c_char,
    b"let\0" as *const u8 as *const ::core::ffi::c_char,
    b"const\0" as *const u8 as *const ::core::ffi::c_char,
    b"var\0" as *const u8 as *const ::core::ffi::c_char,
    b"function\0" as *const u8 as *const ::core::ffi::c_char,
    b"async\0" as *const u8 as *const ::core::ffi::c_char,
    b"await\0" as *const u8 as *const ::core::ffi::c_char,
    b"promise\0" as *const u8 as *const ::core::ffi::c_char,
    b"then\0" as *const u8 as *const ::core::ffi::c_char,
    b"catch\0" as *const u8 as *const ::core::ffi::c_char,
    b"export\0" as *const u8 as *const ::core::ffi::c_char,
    b"fn\0" as *const u8 as *const ::core::ffi::c_char,
    b"mut\0" as *const u8 as *const ::core::ffi::c_char,
    b"match\0" as *const u8 as *const ::core::ffi::c_char,
    b"use\0" as *const u8 as *const ::core::ffi::c_char,
    b"mod\0" as *const u8 as *const ::core::ffi::c_char,
    b"pub\0" as *const u8 as *const ::core::ffi::c_char,
    b"impl\0" as *const u8 as *const ::core::ffi::c_char,
    b"trait\0" as *const u8 as *const ::core::ffi::c_char,
    b"type\0" as *const u8 as *const ::core::ffi::c_char,
    b"where\0" as *const u8 as *const ::core::ffi::c_char,
    b"crate\0" as *const u8 as *const ::core::ffi::c_char,
    b"self\0" as *const u8 as *const ::core::ffi::c_char,
    b"super\0" as *const u8 as *const ::core::ffi::c_char,
    b"sizeof\0" as *const u8 as *const ::core::ffi::c_char,
    b"alignas\0" as *const u8 as *const ::core::ffi::c_char,
    b"alignof\0" as *const u8 as *const ::core::ffi::c_char,
    b"bool\0" as *const u8 as *const ::core::ffi::c_char,
    b"static_assert\0" as *const u8 as *const ::core::ffi::c_char,
    b"thread_local\0" as *const u8 as *const ::core::ffi::c_char,
    b"template\0" as *const u8 as *const ::core::ffi::c_char,
    b"typename\0" as *const u8 as *const ::core::ffi::c_char,
    b"mutable\0" as *const u8 as *const ::core::ffi::c_char,
    b"virtual\0" as *const u8 as *const ::core::ffi::c_char,
    b"override\0" as *const u8 as *const ::core::ffi::c_char,
];
unsafe extern "C" fn completion_preview_reset() {
    completion_preview_state.active = 0 as ::core::ffi::c_int;
    completion_preview_state.line = ::core::ptr::null_mut::<line>();
    completion_preview_state.start_offset = 0 as ::core::ffi::c_int;
    completion_preview_state.prefix_len = 0 as ::core::ffi::c_int;
    completion_preview_state.last_tail_len = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn completion_preview_begin(
    mut line: *mut line,
    mut start: ::core::ffi::c_int,
    mut prefix_len: ::core::ffi::c_int,
) {
    completion_preview_state.active = 1 as ::core::ffi::c_int;
    completion_preview_state.line = line;
    completion_preview_state.start_offset = start;
    completion_preview_state.prefix_len = prefix_len;
    completion_preview_state.last_tail_len = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn completion_preview_delete_tail() {
    if completion_preview_state.active == 0
        || completion_preview_state.last_tail_len <= 0 as ::core::ffi::c_int
        || curwp.is_null()
    {
        return;
    }
    (*curwp).w_dotp = completion_preview_state.line;
    (*curwp).w_doto = completion_preview_state.start_offset
        + completion_preview_state.prefix_len;
    ldelete(completion_preview_state.last_tail_len as ::core::ffi::c_long, FALSE);
    (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | WFMOVE)
        as ::core::ffi::c_char;
    completion_preview_state.last_tail_len = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn completion_preview_abort() {
    completion_preview_delete_tail();
    completion_preview_reset();
}
unsafe extern "C" fn completion_preview_commit() {
    completion_preview_state.active = 0 as ::core::ffi::c_int;
    completion_preview_state.last_tail_len = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn completion_preview_apply_match(
    mut match_0: *const ::core::ffi::c_char,
) {
    if completion_preview_state.active == 0 || match_0.is_null() || curwp.is_null() {
        return;
    }
    let mut match_len: size_t = strlen(match_0);
    if match_len < completion_preview_state.prefix_len as size_t {
        return;
    }
    let mut tail: *const ::core::ffi::c_char = match_0
        .offset(completion_preview_state.prefix_len as isize);
    let mut tail_len: ::core::ffi::c_int = strlen(tail) as ::core::ffi::c_int;
    completion_preview_delete_tail();
    (*curwp).w_dotp = completion_preview_state.line;
    (*curwp).w_doto = completion_preview_state.start_offset
        + completion_preview_state.prefix_len;
    if tail_len > 0 as ::core::ffi::c_int {
        linsert_block(tail as *mut ::core::ffi::c_char, tail_len);
        (*curwp).w_doto = completion_preview_state.start_offset
            + completion_preview_state.prefix_len + tail_len;
    }
    completion_preview_state.last_tail_len = tail_len;
    (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | WFMOVE)
        as ::core::ffi::c_char;
}
unsafe extern "C" fn completion_preview_apply_selected() {
    if completion_preview_state.active == 0 {
        return;
    }
    let mut match_0: *const ::core::ffi::c_char = completion_get_selected();
    if match_0.is_null() {
        return;
    }
    completion_preview_apply_match(match_0);
}
unsafe extern "C" fn completion_write_utf8_clipped(
    mut text: *const ::core::ffi::c_char,
    mut max_width: ::core::ffi::c_int,
) {
    let mut used: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut len: ::core::ffi::c_int = strlen(
        if !text.is_null() {
            text
        } else {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        },
    ) as ::core::ffi::c_int;
    let mut idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while idx < len && used < max_width {
        let mut uc: unicode_t = 0;
        let mut bytes: ::core::ffi::c_int = utf8_to_unicode(
            text as *mut ::core::ffi::c_uchar,
            idx as ::core::ffi::c_uint,
            len as ::core::ffi::c_uint,
            &raw mut uc,
        ) as ::core::ffi::c_int;
        if bytes <= 0 as ::core::ffi::c_int {
            break;
        }
        let mut char_width: ::core::ffi::c_int = mystrnlen_raw_w(uc);
        if used + char_width > max_width {
            break;
        }
        vttputc(uc as ::core::ffi::c_int);
        used += char_width;
        idx += bytes;
    }
    while used < max_width {
        vttputc(' ' as i32);
        used += 1;
    }
}
unsafe extern "C" fn pool_add(
    mut pool: *mut completion_pool_t,
    mut value: *const ::core::ffi::c_char,
) {
    if pool.is_null() || value.is_null() || *value as ::core::ffi::c_int == '\0' as i32 {
        return;
    }
    if (*pool).max_items > 0 as ::core::ffi::c_int && (*pool).count >= (*pool).max_items
    {
        return;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*pool).count {
        if strcmp(*(*pool).items.offset(i as isize), value) == 0 as ::core::ffi::c_int {
            return;
        }
        i += 1;
    }
    if (*pool).count == (*pool).capacity {
        let mut new_capacity: ::core::ffi::c_int = if (*pool).capacity != 0 {
            (*pool).capacity * 2 as ::core::ffi::c_int
        } else {
            64 as ::core::ffi::c_int
        };
        let mut tmp: *mut *mut ::core::ffi::c_char = realloc(
            (*pool).items as *mut ::core::ffi::c_void,
            (new_capacity as size_t)
                .wrapping_mul(
                    ::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t,
                ),
        ) as *mut *mut ::core::ffi::c_char;
        if tmp.is_null() {
            return;
        }
        (*pool).items = tmp;
        (*pool).capacity = new_capacity;
    }
    let fresh4 = (*pool).count;
    (*pool).count = (*pool).count + 1;
    let ref mut fresh5 = *(*pool).items.offset(fresh4 as isize);
    *fresh5 = strdup(value);
}
unsafe extern "C" fn add_matches_from_pool(
    mut pool: *const completion_pool_t,
    mut prefix: *const ::core::ffi::c_char,
) {
    if pool.is_null() || prefix.is_null() {
        return;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*pool).count {
        completion_consider_candidate(*(*pool).items.offset(i as isize), prefix);
        if completion_state.count >= MAX_COMPLETIONS {
            break;
        }
        i += 1;
    }
}
unsafe extern "C" fn completion_reset_state() {
    completion_state.count = 0 as ::core::ffi::c_int;
    completion_state.selected_index = 0 as ::core::ffi::c_int;
    completion_state.is_visible = 0 as ::core::ffi::c_int;
    completion_state.scroll_offset = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn completion_word_exists(
    mut word: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < completion_state.count {
        if strcmp(completion_state.matches[i as usize], word) == 0 as ::core::ffi::c_int
        {
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
unsafe extern "C" fn completion_add_match(mut word: *const ::core::ffi::c_char) {
    if word.is_null() || *word as ::core::ffi::c_int == '\0' as i32 {
        return;
    }
    if completion_state.count >= MAX_COMPLETIONS {
        return;
    }
    if completion_word_exists(word) != 0 {
        return;
    }
    mystrscpy(
        &raw mut *(&raw mut completion_storage as *mut [::core::ffi::c_char; 128])
            .offset(completion_state.count as isize) as *mut ::core::ffi::c_char,
        word,
        MAX_COMPLETION_LEN,
    );
    completion_state.matches[completion_state.count as usize] = &raw mut *(&raw mut completion_storage
        as *mut [::core::ffi::c_char; 128])
        .offset(completion_state.count as isize) as *mut ::core::ffi::c_char;
    completion_state.count += 1;
}
unsafe extern "C" fn is_identifier_char(mut uc: unicode_t) -> ::core::ffi::c_int {
    if uc == '_' as i32 as unicode_t {
        return TRUE;
    }
    if uc >= 0x80 as unicode_t {
        return TRUE;
    }
    if iswalnum(uc) != 0 {
        return TRUE;
    }
    return FALSE;
}
unsafe extern "C" fn is_path_char(mut uc: unicode_t) -> ::core::ffi::c_int {
    if uc >= 0x80 as unicode_t {
        return TRUE;
    }
    if iswalnum(uc) != 0 {
        return TRUE;
    }
    match uc {
        95 | 45 | 46 | 47 | 126 | 43 | 58 => return TRUE,
        _ => return FALSE,
    };
}
unsafe extern "C" fn completion_consider_candidate(
    mut candidate: *const ::core::ffi::c_char,
    mut prefix: *const ::core::ffi::c_char,
) {
    if candidate.is_null() || prefix.is_null() {
        return;
    }
    let mut prefix_len: size_t = strlen(prefix);
    if prefix_len == 0 as size_t {
        return;
    }
    if strncmp(candidate, prefix, prefix_len) != 0 as ::core::ffi::c_int {
        return;
    }
    if strcmp(candidate, prefix) == 0 as ::core::ffi::c_int {
        return;
    }
    completion_add_match(candidate);
}
unsafe extern "C" fn collect_keyword_array(
    mut entries: *const [::core::ffi::c_char; 64],
    mut count: ::core::ffi::c_int,
    mut prefix: *const ::core::ffi::c_char,
) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count && completion_state.count < MAX_COMPLETIONS {
        completion_consider_candidate(
            &raw const *entries.offset(i as isize) as *const ::core::ffi::c_char,
            prefix,
        );
        if completion_state.count >= MAX_COMPLETIONS {
            break;
        }
        i += 1;
    }
}
unsafe extern "C" fn collect_language_keywords(mut prefix: *const ::core::ffi::c_char) {
    if curbp.is_null() {
        return;
    }
    let mut profile: *const HighlightProfile = highlight_get_profile(
        &raw mut (*curbp).b_fname as *mut ::core::ffi::c_char,
    );
    if profile.is_null() {
        return;
    }
    collect_keyword_array(
        &raw const (*profile).keywords as *const [::core::ffi::c_char; 64],
        (*profile).keyword_count,
        prefix,
    );
    collect_keyword_array(
        &raw const (*profile).type_keywords as *const [::core::ffi::c_char; 64],
        (*profile).type_keyword_count,
        prefix,
    );
    collect_keyword_array(
        &raw const (*profile).flow_keywords as *const [::core::ffi::c_char; 64],
        (*profile).flow_keyword_count,
        prefix,
    );
    collect_keyword_array(
        &raw const (*profile).preproc_keywords as *const [::core::ffi::c_char; 64],
        (*profile).preproc_keyword_count,
        prefix,
    );
    collect_keyword_array(
        &raw const (*profile).return_keywords as *const [::core::ffi::c_char; 64],
        (*profile).return_keyword_count,
        prefix,
    );
}
unsafe extern "C" fn collect_common_keywords(mut prefix: *const ::core::ffi::c_char) {
    let mut prefix_len: size_t = strlen(prefix);
    let mut i: size_t = 0 as size_t;
    while i
        < (::core::mem::size_of::<[*const ::core::ffi::c_char; 82]>() as usize)
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
    {
        let mut word: *const ::core::ffi::c_char = common_keywords[i as usize];
        if strncmp(word, prefix, prefix_len) == 0 as ::core::ffi::c_int
            && strcmp(word, prefix) != 0 as ::core::ffi::c_int
        {
            completion_add_match(word);
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn collect_buffer_words(mut prefix: *const ::core::ffi::c_char) {
    if curbp.is_null() {
        return;
    }
    let mut prefix_len: size_t = strlen(prefix);
    if prefix_len == 0 as size_t {
        return;
    }
    let mut lp: *mut line = (*(*curbp).b_linep).l_fp;
    while lp != (*curbp).b_linep && completion_state.count < MAX_COMPLETIONS {
        let mut len: ::core::ffi::c_int = (*lp).l_used;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < len && completion_state.count < MAX_COMPLETIONS {
            let mut uc: unicode_t = 0;
            let mut bytes: ::core::ffi::c_int = utf8_to_unicode(
                &raw mut (*lp).l_text as *mut ::core::ffi::c_uchar,
                i as ::core::ffi::c_uint,
                len as ::core::ffi::c_uint,
                &raw mut uc,
            ) as ::core::ffi::c_int;
            if bytes <= 0 as ::core::ffi::c_int {
                bytes = 1 as ::core::ffi::c_int;
            }
            if is_identifier_char(uc) != 0 {
                let mut start: ::core::ffi::c_int = i;
                i += bytes;
                while i < len {
                    let mut next: unicode_t = 0;
                    let mut consumed: ::core::ffi::c_int = utf8_to_unicode(
                        &raw mut (*lp).l_text as *mut ::core::ffi::c_uchar,
                        i as ::core::ffi::c_uint,
                        len as ::core::ffi::c_uint,
                        &raw mut next,
                    ) as ::core::ffi::c_int;
                    if consumed <= 0 as ::core::ffi::c_int {
                        consumed = 1 as ::core::ffi::c_int;
                    }
                    if is_identifier_char(next) == 0 {
                        break;
                    }
                    i += consumed;
                }
                let mut word_len: ::core::ffi::c_int = i - start;
                if word_len >= MAX_COMPLETION_LEN {
                    word_len = MAX_COMPLETION_LEN - 1 as ::core::ffi::c_int;
                }
                let mut tmp: [::core::ffi::c_char; 128] = [0; 128];
                memcpy(
                    &raw mut tmp as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                    (&raw mut (*lp).l_text as *mut ::core::ffi::c_uchar)
                        .offset(start as isize) as *mut ::core::ffi::c_uchar
                        as *const ::core::ffi::c_void,
                    word_len as size_t,
                );
                tmp[word_len as usize] = '\0' as i32 as ::core::ffi::c_char;
                if word_len as size_t >= prefix_len {
                    completion_consider_candidate(
                        &raw mut tmp as *mut ::core::ffi::c_char,
                        prefix,
                    );
                }
            } else {
                i += bytes;
            }
        }
        lp = (*lp).l_fp;
    }
}
unsafe extern "C" fn add_env_paths(
    mut env_name: *const ::core::ffi::c_char,
    mut paths: *mut completion_pool_t,
) {
    let mut value: *const ::core::ffi::c_char = getenv(env_name);
    if !value.is_null() && *value as ::core::ffi::c_int != 0 {
        parse_path_list(value, paths);
    }
}
unsafe extern "C" fn ensure_c_include_paths() {
    static mut defaults: [*const ::core::ffi::c_char; 4] = [
        b"/usr/include\0" as *const u8 as *const ::core::ffi::c_char,
        b"/usr/local/include\0" as *const u8 as *const ::core::ffi::c_char,
        b"/opt/homebrew/include\0" as *const u8 as *const ::core::ffi::c_char,
        b"/opt/local/include\0" as *const u8 as *const ::core::ffi::c_char,
    ];
    if c_include_paths.count > 0 as ::core::ffi::c_int {
        return;
    }
    add_env_paths(
        b"C_INCLUDE_PATH\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut c_include_paths,
    );
    add_env_paths(
        b"CPLUS_INCLUDE_PATH\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut c_include_paths,
    );
    add_env_paths(
        b"CXX_INCLUDE_PATH\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut c_include_paths,
    );
    add_env_paths(
        b"CPATH\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut c_include_paths,
    );
    add_env_paths(
        b"INCLUDE\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut c_include_paths,
    );
    let mut i: size_t = 0 as size_t;
    while i
        < (::core::mem::size_of::<[*const ::core::ffi::c_char; 4]>() as usize)
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
    {
        if file_exists(defaults[i as usize]) != 0 {
            add_path_entry(&raw mut c_include_paths, defaults[i as usize]);
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn add_symbol_token(
    mut pool: *mut completion_pool_t,
    mut token: *const ::core::ffi::c_char,
    mut len: size_t,
) {
    let mut tmp: [::core::ffi::c_char; 128] = [0; 128];
    if pool.is_null() || token.is_null() {
        return;
    }
    if len < 3 as size_t {
        return;
    }
    if len >= MAX_COMPLETION_LEN as size_t {
        len = (MAX_COMPLETION_LEN - 1 as ::core::ffi::c_int) as size_t;
    }
    memcpy(
        &raw mut tmp as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        token as *const ::core::ffi::c_void,
        len,
    );
    tmp[len as usize] = '\0' as i32 as ::core::ffi::c_char;
    pool_add(pool, &raw mut tmp as *mut ::core::ffi::c_char);
}
unsafe extern "C" fn scan_c_header_file(mut path: *const ::core::ffi::c_char) {
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut buf: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut total: size_t = 0 as size_t;
    let mut nread: size_t = 0;
    let mut token: [::core::ffi::c_char; 128] = [0; 128];
    let mut token_len: size_t = 0 as size_t;
    if path.is_null() || c_files_scanned >= MAX_C_SCAN_FILES {
        return;
    }
    fp = fopen(path, b"r\0" as *const u8 as *const ::core::ffi::c_char) as *mut FILE;
    if fp.is_null() {
        return;
    }
    c_files_scanned += 1;
    loop {
        nread = fread(
            &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            1 as size_t,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as size_t,
            fp,
        ) as size_t;
        if !(nread > 0 as size_t) {
            break;
        }
        let mut i: size_t = 0 as size_t;
        while i < nread {
            let mut ch: ::core::ffi::c_uchar = buf[i as usize] as ::core::ffi::c_uchar;
            if *(*__ctype_b_loc()).offset(ch as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & _ISalnum as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int != 0 || ch as ::core::ffi::c_int == '_' as i32
            {
                if token_len
                    < (::core::mem::size_of::<[::core::ffi::c_char; 128]>() as usize)
                        .wrapping_sub(1 as usize)
                {
                    let fresh20 = token_len;
                    token_len = token_len.wrapping_add(1);
                    token[fresh20 as usize] = ch as ::core::ffi::c_char;
                }
            } else if token_len > 0 as size_t {
                add_symbol_token(
                    &raw mut c_symbol_cache,
                    &raw mut token as *mut ::core::ffi::c_char,
                    token_len,
                );
                token_len = 0 as size_t;
                if c_symbol_cache.max_items > 0 as ::core::ffi::c_int
                    && c_symbol_cache.count >= c_symbol_cache.max_items
                {
                    break;
                }
            }
            i = i.wrapping_add(1);
        }
        total = total.wrapping_add(nread);
        if c_symbol_cache.max_items > 0 as ::core::ffi::c_int
            && c_symbol_cache.count >= c_symbol_cache.max_items
            || total >= MAX_C_FILE_BYTES as size_t
        {
            break;
        }
    }
    if token_len > 0 as size_t {
        add_symbol_token(
            &raw mut c_symbol_cache,
            &raw mut token as *mut ::core::ffi::c_char,
            token_len,
        );
    }
    fclose(fp);
}
unsafe extern "C" fn scan_c_include_dir(
    mut path: *const ::core::ffi::c_char,
    mut depth: ::core::ffi::c_int,
) {
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut dp: *mut DIR = ::core::ptr::null_mut::<DIR>();
    let mut dent: *mut dirent = ::core::ptr::null_mut::<dirent>();
    let mut child: [::core::ffi::c_char; 2048] = [0; 2048];
    static mut header_exts: [*const ::core::ffi::c_char; 6] = [
        b".h\0" as *const u8 as *const ::core::ffi::c_char,
        b".hpp\0" as *const u8 as *const ::core::ffi::c_char,
        b".hh\0" as *const u8 as *const ::core::ffi::c_char,
        b".hxx\0" as *const u8 as *const ::core::ffi::c_char,
        b".hp\0" as *const u8 as *const ::core::ffi::c_char,
        b".inc\0" as *const u8 as *const ::core::ffi::c_char,
    ];
    if path.is_null() || *path as ::core::ffi::c_int == '\0' as i32
        || depth > MAX_C_SCAN_DEPTH
    {
        return;
    }
    if c_files_scanned >= MAX_C_SCAN_FILES
        || c_symbol_cache.max_items > 0 as ::core::ffi::c_int
            && c_symbol_cache.count >= c_symbol_cache.max_items
    {
        return;
    }
    dp = opendir(path);
    if dp.is_null() {
        return;
    }
    loop {
        dent = readdir(dp);
        if dent.is_null() {
            break;
        }
        if strcmp(
            &raw mut (*dent).d_name as *mut ::core::ffi::c_char,
            b".\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                &raw mut (*dent).d_name as *mut ::core::ffi::c_char,
                b"..\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            continue;
        }
        snprintf(
            &raw mut child as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 2048]>() as size_t,
            b"%s/%s\0" as *const u8 as *const ::core::ffi::c_char,
            path,
            &raw mut (*dent).d_name as *mut ::core::ffi::c_char,
        );
        if stat(&raw mut child as *mut ::core::ffi::c_char, &raw mut st)
            != 0 as ::core::ffi::c_int
        {
            continue;
        }
        if st.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t {
            if depth + 1 as ::core::ffi::c_int <= MAX_C_SCAN_DEPTH {
                scan_c_include_dir(
                    &raw mut child as *mut ::core::ffi::c_char,
                    depth + 1 as ::core::ffi::c_int,
                );
            }
        } else if st.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t {
            if has_extension(
                &raw mut (*dent).d_name as *mut ::core::ffi::c_char,
                &raw mut header_exts as *mut *const ::core::ffi::c_char,
                (::core::mem::size_of::<[*const ::core::ffi::c_char; 6]>() as size_t)
                    .wrapping_div(
                        ::core::mem::size_of::<*const ::core::ffi::c_char>() as size_t,
                    ),
            ) != 0
            {
                scan_c_header_file(&raw mut child as *mut ::core::ffi::c_char);
            }
        }
        if c_files_scanned >= MAX_C_SCAN_FILES
            || c_symbol_cache.max_items > 0 as ::core::ffi::c_int
                && c_symbol_cache.count >= c_symbol_cache.max_items
        {
            break;
        }
    }
    closedir(dp);
}
unsafe extern "C" fn ensure_c_symbols_loaded() {
    if c_symbols_loaded != 0 {
        return;
    }
    ensure_c_include_paths();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < c_include_paths.count {
        scan_c_include_dir(
            *c_include_paths.items.offset(i as isize),
            0 as ::core::ffi::c_int,
        );
        if c_symbol_cache.max_items > 0 as ::core::ffi::c_int
            && c_symbol_cache.count >= c_symbol_cache.max_items
        {
            break;
        }
        i += 1;
    }
    c_symbols_loaded = 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn class_name_from_relative(
    mut relative_path: *const ::core::ffi::c_char,
    mut out: *mut ::core::ffi::c_char,
    mut outsz: size_t,
) -> ::core::ffi::c_int {
    let mut len: size_t = 0;
    let mut dot: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if relative_path.is_null() || out.is_null() || outsz == 0 as size_t {
        return FALSE;
    }
    dot = strrchr(relative_path, '.' as i32);
    if dot.is_null() || dot == relative_path {
        return FALSE;
    }
    len = dot.offset_from(relative_path) as ::core::ffi::c_long as size_t;
    if len >= outsz {
        len = outsz.wrapping_sub(1 as size_t);
    }
    let mut i: size_t = 0 as size_t;
    while i < len {
        let mut ch: ::core::ffi::c_char = *relative_path.offset(i as isize);
        if ch as ::core::ffi::c_int == '/' as i32
            || ch as ::core::ffi::c_int == '\\' as i32
        {
            ch = '.' as i32 as ::core::ffi::c_char;
        }
        *out.offset(i as isize) = ch;
        i = i.wrapping_add(1);
    }
    *out.offset(len as isize) = '\0' as i32 as ::core::ffi::c_char;
    if *out.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32
        || !strstr(out, b"module-info\0" as *const u8 as *const ::core::ffi::c_char)
            .is_null()
    {
        return FALSE;
    }
    return TRUE;
}
unsafe extern "C" fn add_java_dir_entry(
    mut root: *const ::core::ffi::c_char,
    mut root_len: size_t,
    mut path: *const ::core::ffi::c_char,
) {
    let mut rel: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut class_name: [::core::ffi::c_char; 128] = [0; 128];
    if root.is_null() || path.is_null() {
        return;
    }
    if strlen(path) <= root_len {
        return;
    }
    rel = path.offset(root_len as isize);
    while *rel as ::core::ffi::c_int == '/' as i32
        || *rel as ::core::ffi::c_int == '\\' as i32
    {
        rel = rel.offset(1);
    }
    if *rel as ::core::ffi::c_int == '\0' as i32 {
        return;
    }
    if class_name_from_relative(
        rel,
        &raw mut class_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
    ) == 0
    {
        return;
    }
    pool_add(&raw mut java_class_cache, &raw mut class_name as *mut ::core::ffi::c_char);
}
unsafe extern "C" fn scan_java_dir(
    mut root: *const ::core::ffi::c_char,
    mut root_len: size_t,
    mut path: *const ::core::ffi::c_char,
    mut depth: ::core::ffi::c_int,
) {
    let mut dp: *mut DIR = ::core::ptr::null_mut::<DIR>();
    let mut dent: *mut dirent = ::core::ptr::null_mut::<dirent>();
    let mut child: [::core::ffi::c_char; 2048] = [0; 2048];
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    static mut class_exts: [*const ::core::ffi::c_char; 2] = [
        b".class\0" as *const u8 as *const ::core::ffi::c_char,
        b".java\0" as *const u8 as *const ::core::ffi::c_char,
    ];
    if path.is_null() || *path as ::core::ffi::c_int == '\0' as i32
        || depth > MAX_JAVA_SCAN_DEPTH
    {
        return;
    }
    if java_files_scanned >= MAX_JAVA_SCAN_FILES
        || java_class_cache.max_items > 0 as ::core::ffi::c_int
            && java_class_cache.count >= java_class_cache.max_items
    {
        return;
    }
    dp = opendir(path);
    if dp.is_null() {
        return;
    }
    loop {
        dent = readdir(dp);
        if dent.is_null() {
            break;
        }
        if strcmp(
            &raw mut (*dent).d_name as *mut ::core::ffi::c_char,
            b".\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                &raw mut (*dent).d_name as *mut ::core::ffi::c_char,
                b"..\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            continue;
        }
        snprintf(
            &raw mut child as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 2048]>() as size_t,
            b"%s/%s\0" as *const u8 as *const ::core::ffi::c_char,
            path,
            &raw mut (*dent).d_name as *mut ::core::ffi::c_char,
        );
        if stat(&raw mut child as *mut ::core::ffi::c_char, &raw mut st)
            != 0 as ::core::ffi::c_int
        {
            continue;
        }
        if st.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t {
            if depth + 1 as ::core::ffi::c_int <= MAX_JAVA_SCAN_DEPTH {
                scan_java_dir(
                    root,
                    root_len,
                    &raw mut child as *mut ::core::ffi::c_char,
                    depth + 1 as ::core::ffi::c_int,
                );
            }
        } else if st.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t {
            if has_extension(
                &raw mut (*dent).d_name as *mut ::core::ffi::c_char,
                &raw mut class_exts as *mut *const ::core::ffi::c_char,
                (::core::mem::size_of::<[*const ::core::ffi::c_char; 2]>() as size_t)
                    .wrapping_div(
                        ::core::mem::size_of::<*const ::core::ffi::c_char>() as size_t,
                    ),
            ) != 0
            {
                java_files_scanned += 1;
                add_java_dir_entry(
                    root,
                    root_len,
                    &raw mut child as *mut ::core::ffi::c_char,
                );
            }
        }
        if java_files_scanned >= MAX_JAVA_SCAN_FILES
            || java_class_cache.max_items > 0 as ::core::ffi::c_int
                && java_class_cache.count >= java_class_cache.max_items
        {
            break;
        }
    }
    closedir(dp);
}
unsafe extern "C" fn build_shell_quoted(
    mut input: *const ::core::ffi::c_char,
    mut output: *mut ::core::ffi::c_char,
    mut outsz: size_t,
) -> ::core::ffi::c_int {
    let mut pos: size_t = 0 as size_t;
    if input.is_null() || output.is_null() || outsz < 3 as size_t {
        return FALSE;
    }
    let fresh6 = pos;
    pos = pos.wrapping_add(1);
    *output.offset(fresh6 as isize) = '\'' as i32 as ::core::ffi::c_char;
    let mut p: *const ::core::ffi::c_char = input;
    while *p != 0 {
        if pos.wrapping_add(4 as size_t) >= outsz {
            return FALSE;
        }
        if *p as ::core::ffi::c_int == '\'' as i32 {
            let fresh7 = pos;
            pos = pos.wrapping_add(1);
            *output.offset(fresh7 as isize) = '\'' as i32 as ::core::ffi::c_char;
            let fresh8 = pos;
            pos = pos.wrapping_add(1);
            *output.offset(fresh8 as isize) = '\\' as i32 as ::core::ffi::c_char;
            let fresh9 = pos;
            pos = pos.wrapping_add(1);
            *output.offset(fresh9 as isize) = '\'' as i32 as ::core::ffi::c_char;
            let fresh10 = pos;
            pos = pos.wrapping_add(1);
            *output.offset(fresh10 as isize) = '\'' as i32 as ::core::ffi::c_char;
        } else {
            let fresh11 = pos;
            pos = pos.wrapping_add(1);
            *output.offset(fresh11 as isize) = *p;
        }
        p = p.offset(1);
    }
    if pos.wrapping_add(2 as size_t) > outsz {
        return FALSE;
    }
    let fresh12 = pos;
    pos = pos.wrapping_add(1);
    *output.offset(fresh12 as isize) = '\'' as i32 as ::core::ffi::c_char;
    *output.offset(pos as isize) = '\0' as i32 as ::core::ffi::c_char;
    return TRUE;
}
unsafe extern "C" fn process_java_jar(mut path: *const ::core::ffi::c_char) {
    let mut quoted: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut cmd: [::core::ffi::c_char; 4112] = [0; 4112];
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut line: [::core::ffi::c_char; 512] = [0; 512];
    if path.is_null() || *path == 0 {
        return;
    }
    if build_shell_quoted(
        path,
        &raw mut quoted as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as size_t,
    ) == 0
    {
        return;
    }
    snprintf(
        &raw mut cmd as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4112]>() as size_t,
        b"jar tf %s\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut quoted as *mut ::core::ffi::c_char,
    );
    fp = popen(
        &raw mut cmd as *mut ::core::ffi::c_char,
        b"r\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if fp.is_null() {
        return;
    }
    while !fgets(
            &raw mut line as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as ::core::ffi::c_int,
            fp,
        )
        .is_null()
    {
        let mut len: size_t = strlen(&raw mut line as *mut ::core::ffi::c_char);
        if len > 0 as size_t
            && (line[len.wrapping_sub(1 as size_t) as usize] as ::core::ffi::c_int
                == '\n' as i32
                || line[len.wrapping_sub(1 as size_t) as usize] as ::core::ffi::c_int
                    == '\r' as i32)
        {
            len = len.wrapping_sub(1);
            line[len as usize] = '\0' as i32 as ::core::ffi::c_char;
        }
        if len == 0 as size_t {
            continue;
        }
        if !strstr(
                &raw mut line as *mut ::core::ffi::c_char,
                b"module-info\0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null()
        {
            continue;
        }
        if class_name_from_relative(
            &raw mut line as *mut ::core::ffi::c_char,
            &raw mut line as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as size_t,
        ) != 0
        {
            pool_add(
                &raw mut java_class_cache,
                &raw mut line as *mut ::core::ffi::c_char,
            );
        }
        if java_class_cache.max_items > 0 as ::core::ffi::c_int
            && java_class_cache.count >= java_class_cache.max_items
        {
            break;
        }
    }
    pclose(fp);
}
unsafe extern "C" fn ensure_java_classpath_entries() {
    let mut java_home: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut buf: [::core::ffi::c_char; 2048] = [0; 2048];
    if java_classpath_loaded != 0 {
        return;
    }
    add_env_paths(
        b"CLASSPATH\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut java_classpath_entries,
    );
    add_path_entry(
        &raw mut java_classpath_entries,
        b".\0" as *const u8 as *const ::core::ffi::c_char,
    );
    add_path_entry(
        &raw mut java_classpath_entries,
        b"./lib\0" as *const u8 as *const ::core::ffi::c_char,
    );
    add_path_entry(
        &raw mut java_classpath_entries,
        b"./build/classes\0" as *const u8 as *const ::core::ffi::c_char,
    );
    add_path_entry(
        &raw mut java_classpath_entries,
        b"./out/production\0" as *const u8 as *const ::core::ffi::c_char,
    );
    add_path_entry(
        &raw mut java_classpath_entries,
        b"~/.m2/repository\0" as *const u8 as *const ::core::ffi::c_char,
    );
    java_home = getenv(b"JAVA_HOME\0" as *const u8 as *const ::core::ffi::c_char);
    if !java_home.is_null() && *java_home as ::core::ffi::c_int != 0 {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 2048]>() as size_t,
            b"%s/lib\0" as *const u8 as *const ::core::ffi::c_char,
            java_home,
        );
        if file_exists(&raw mut buf as *mut ::core::ffi::c_char) != 0 {
            add_path_entry(
                &raw mut java_classpath_entries,
                &raw mut buf as *mut ::core::ffi::c_char,
            );
        }
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 2048]>() as size_t,
            b"%s/lib/rt.jar\0" as *const u8 as *const ::core::ffi::c_char,
            java_home,
        );
        if file_exists(&raw mut buf as *mut ::core::ffi::c_char) != 0 {
            add_path_entry(
                &raw mut java_classpath_entries,
                &raw mut buf as *mut ::core::ffi::c_char,
            );
        }
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 2048]>() as size_t,
            b"%s/lib/src.zip\0" as *const u8 as *const ::core::ffi::c_char,
            java_home,
        );
        if file_exists(&raw mut buf as *mut ::core::ffi::c_char) != 0 {
            add_path_entry(
                &raw mut java_classpath_entries,
                &raw mut buf as *mut ::core::ffi::c_char,
            );
        }
    }
    java_classpath_loaded = 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn ensure_java_class_symbols() {
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if java_symbols_loaded != 0 {
        return;
    }
    ensure_java_classpath_entries();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < java_classpath_entries.count {
        let mut entry: *const ::core::ffi::c_char = *java_classpath_entries
            .items
            .offset(i as isize);
        if !(stat(entry, &raw mut st) != 0 as ::core::ffi::c_int) {
            if st.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t {
                let mut root_len: size_t = strlen(entry);
                scan_java_dir(entry, root_len, entry, 0 as ::core::ffi::c_int);
            } else if st.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t {
                let mut exts: [*const ::core::ffi::c_char; 2] = [
                    b".jar\0" as *const u8 as *const ::core::ffi::c_char,
                    b".zip\0" as *const u8 as *const ::core::ffi::c_char,
                ];
                if has_extension(
                    entry,
                    &raw mut exts as *mut *const ::core::ffi::c_char,
                    (::core::mem::size_of::<[*const ::core::ffi::c_char; 2]>() as size_t)
                        .wrapping_div(
                            ::core::mem::size_of::<*const ::core::ffi::c_char>()
                                as size_t,
                        ),
                ) != 0
                {
                    process_java_jar(entry);
                }
            }
            if java_class_cache.max_items > 0 as ::core::ffi::c_int
                && java_class_cache.count >= java_class_cache.max_items
            {
                break;
            }
        }
        i += 1;
    }
    java_symbols_loaded = 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn get_java_classpath_string() -> *const ::core::ffi::c_char {
    let mut used: size_t = 0 as size_t;
    if java_classpath_string[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        != '\0' as i32
    {
        return &raw mut java_classpath_string as *mut ::core::ffi::c_char;
    }
    ensure_java_classpath_entries();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < java_classpath_entries.count {
        let mut entry: *const ::core::ffi::c_char = *java_classpath_entries
            .items
            .offset(i as isize);
        let mut len: size_t = strlen(entry);
        if used.wrapping_add(len).wrapping_add(2 as size_t)
            >= ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize
        {
            break;
        }
        if used > 0 as size_t {
            let fresh13 = used;
            used = used.wrapping_add(1);
            java_classpath_string[fresh13 as usize] = ':' as i32 as ::core::ffi::c_char;
        }
        memcpy(
            (&raw mut java_classpath_string as *mut ::core::ffi::c_char)
                .offset(used as isize) as *mut ::core::ffi::c_void,
            entry as *const ::core::ffi::c_void,
            len,
        );
        used = used.wrapping_add(len);
        i += 1;
    }
    java_classpath_string[used as usize] = '\0' as i32 as ::core::ffi::c_char;
    if java_classpath_string[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        == '\0' as i32
    {
        strcpy(
            &raw mut java_classpath_string as *mut ::core::ffi::c_char,
            b".\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return &raw mut java_classpath_string as *mut ::core::ffi::c_char;
}
unsafe extern "C" fn is_c_like_file(
    mut fname: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    static mut exts: [*const ::core::ffi::c_char; 9] = [
        b".c\0" as *const u8 as *const ::core::ffi::c_char,
        b".h\0" as *const u8 as *const ::core::ffi::c_char,
        b".hpp\0" as *const u8 as *const ::core::ffi::c_char,
        b".hh\0" as *const u8 as *const ::core::ffi::c_char,
        b".hxx\0" as *const u8 as *const ::core::ffi::c_char,
        b".cxx\0" as *const u8 as *const ::core::ffi::c_char,
        b".cc\0" as *const u8 as *const ::core::ffi::c_char,
        b".cpp\0" as *const u8 as *const ::core::ffi::c_char,
        b".ino\0" as *const u8 as *const ::core::ffi::c_char,
    ];
    if fname.is_null() || *fname as ::core::ffi::c_int == '\0' as i32 {
        return FALSE;
    }
    return has_extension(
        fname,
        &raw mut exts as *mut *const ::core::ffi::c_char,
        (::core::mem::size_of::<[*const ::core::ffi::c_char; 9]>() as size_t)
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as size_t),
    );
}
unsafe extern "C" fn is_java_file(
    mut fname: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    static mut exts: [*const ::core::ffi::c_char; 1] = [
        b".java\0" as *const u8 as *const ::core::ffi::c_char,
    ];
    if fname.is_null() || *fname as ::core::ffi::c_int == '\0' as i32 {
        return FALSE;
    }
    return has_extension(
        fname,
        &raw mut exts as *mut *const ::core::ffi::c_char,
        (::core::mem::size_of::<[*const ::core::ffi::c_char; 1]>() as size_t)
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as size_t),
    );
}
unsafe extern "C" fn is_python_file(
    mut fname: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    static mut exts: [*const ::core::ffi::c_char; 1] = [
        b".py\0" as *const u8 as *const ::core::ffi::c_char,
    ];
    if fname.is_null() || *fname as ::core::ffi::c_int == '\0' as i32 {
        return FALSE;
    }
    return has_extension(
        fname,
        &raw mut exts as *mut *const ::core::ffi::c_char,
        (::core::mem::size_of::<[*const ::core::ffi::c_char; 1]>() as size_t)
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as size_t),
    );
}
unsafe extern "C" fn is_node_file(
    mut fname: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    static mut exts: [*const ::core::ffi::c_char; 6] = [
        b".js\0" as *const u8 as *const ::core::ffi::c_char,
        b".jsx\0" as *const u8 as *const ::core::ffi::c_char,
        b".ts\0" as *const u8 as *const ::core::ffi::c_char,
        b".tsx\0" as *const u8 as *const ::core::ffi::c_char,
        b".mjs\0" as *const u8 as *const ::core::ffi::c_char,
        b".cjs\0" as *const u8 as *const ::core::ffi::c_char,
    ];
    if fname.is_null() || *fname as ::core::ffi::c_int == '\0' as i32 {
        return FALSE;
    }
    return has_extension(
        fname,
        &raw mut exts as *mut *const ::core::ffi::c_char,
        (::core::mem::size_of::<[*const ::core::ffi::c_char; 6]>() as size_t)
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as size_t),
    );
}
unsafe extern "C" fn add_language_specific_matches(
    mut prefix: *const ::core::ffi::c_char,
    mut ctx: completion_context_t,
) {
    if ctx as ::core::ffi::c_uint
        == COMPLETION_CONTEXT_PATH as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    if curbp.is_null() || prefix.is_null()
        || *prefix as ::core::ffi::c_int == '\0' as i32
    {
        return;
    }
    if is_c_like_file(&raw mut (*curbp).b_fname as *mut ::core::ffi::c_char) != 0 {
        ensure_c_symbols_loaded();
        add_matches_from_pool(&raw mut c_symbol_cache, prefix);
    } else if is_java_file(&raw mut (*curbp).b_fname as *mut ::core::ffi::c_char) != 0 {
        ensure_java_class_symbols();
        add_matches_from_pool(&raw mut java_class_cache, prefix);
    }
    if completion_state.count > 0 as ::core::ffi::c_int {
        completion_state.is_visible = 1 as ::core::ffi::c_int;
    }
}
static mut c_control_keywords: [*const ::core::ffi::c_char; 12] = [
    b"if\0" as *const u8 as *const ::core::ffi::c_char,
    b"else\0" as *const u8 as *const ::core::ffi::c_char,
    b"while\0" as *const u8 as *const ::core::ffi::c_char,
    b"for\0" as *const u8 as *const ::core::ffi::c_char,
    b"do\0" as *const u8 as *const ::core::ffi::c_char,
    b"switch\0" as *const u8 as *const ::core::ffi::c_char,
    b"return\0" as *const u8 as *const ::core::ffi::c_char,
    b"sizeof\0" as *const u8 as *const ::core::ffi::c_char,
    b"alignof\0" as *const u8 as *const ::core::ffi::c_char,
    b"typeof\0" as *const u8 as *const ::core::ffi::c_char,
    b"static_assert\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
unsafe extern "C" fn is_c_control_kw(
    mut tok: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !c_control_keywords[i as usize].is_null() {
        if strcmp(tok, c_control_keywords[i as usize]) == 0 as ::core::ffi::c_int {
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
unsafe extern "C" fn extract_c_symbols_from_text(
    mut text: *const ::core::ffi::c_char,
    mut text_len: size_t,
    mut pool: *mut completion_pool_t,
) {
    let mut current_block: u64;
    let mut state: C2RustUnnamed_2 = ST_NORMAL_1;
    let mut tok: [::core::ffi::c_char; 128] = [0; 128];
    let mut tok_len: size_t = 0 as size_t;
    let mut prev1: [::core::ffi::c_char; 128] = [0; 128];
    prev1[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
    let mut in_typedef: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut last_ident: [::core::ffi::c_char; 128] = [0; 128];
    last_ident[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
    let mut brace_depth: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: size_t = 0 as size_t;
    while i < text_len {
        let mut c: ::core::ffi::c_uchar = *text.offset(i as isize)
            as ::core::ffi::c_uchar;
        let mut next: ::core::ffi::c_uchar = (if i.wrapping_add(1 as size_t) < text_len {
            *text.offset(i.wrapping_add(1 as size_t) as isize) as ::core::ffi::c_uchar
                as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uchar;
        match state as ::core::ffi::c_uint {
            0 => {
                if c as ::core::ffi::c_int == '/' as i32
                    && next as ::core::ffi::c_int == '/' as i32
                {
                    state = ST_LINE_CMT_1;
                    i = i.wrapping_add(1);
                    current_block = 15111316862508955813;
                } else if c as ::core::ffi::c_int == '/' as i32
                    && next as ::core::ffi::c_int == '*' as i32
                {
                    state = ST_BLOCK_CMT_0;
                    i = i.wrapping_add(1);
                    current_block = 15111316862508955813;
                } else if c as ::core::ffi::c_int == '"' as i32 {
                    state = ST_STRING;
                    current_block = 15111316862508955813;
                } else if c as ::core::ffi::c_int == '\'' as i32 {
                    state = ST_CHAR;
                    current_block = 15111316862508955813;
                } else if *(*__ctype_b_loc()).offset(c as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & _ISalnum as ::core::ffi::c_int as ::core::ffi::c_ushort
                        as ::core::ffi::c_int != 0
                    || c as ::core::ffi::c_int == '_' as i32
                {
                    if tok_len
                        < (::core::mem::size_of::<[::core::ffi::c_char; 128]>() as usize)
                            .wrapping_sub(1 as usize)
                    {
                        let fresh17 = tok_len;
                        tok_len = tok_len.wrapping_add(1);
                        tok[fresh17 as usize] = c as ::core::ffi::c_char;
                    }
                    current_block = 735147466149431745;
                } else {
                    current_block = 15111316862508955813;
                }
                match current_block {
                    735147466149431745 => {}
                    _ => {
                        if tok_len > 0 as size_t {
                            tok[tok_len as usize] = '\0' as i32 as ::core::ffi::c_char;
                            if strcmp(
                                &raw mut prev1 as *mut ::core::ffi::c_char,
                                b"struct\0" as *const u8 as *const ::core::ffi::c_char,
                            ) == 0 as ::core::ffi::c_int
                                || strcmp(
                                    &raw mut prev1 as *mut ::core::ffi::c_char,
                                    b"union\0" as *const u8 as *const ::core::ffi::c_char,
                                ) == 0 as ::core::ffi::c_int
                                || strcmp(
                                    &raw mut prev1 as *mut ::core::ffi::c_char,
                                    b"enum\0" as *const u8 as *const ::core::ffi::c_char,
                                ) == 0 as ::core::ffi::c_int
                            {
                                pool_add(pool, &raw mut tok as *mut ::core::ffi::c_char);
                            }
                            if in_typedef != 0 && brace_depth == 0 as ::core::ffi::c_int
                            {
                                mystrscpy(
                                    &raw mut last_ident as *mut ::core::ffi::c_char,
                                    &raw mut tok as *mut ::core::ffi::c_char,
                                    ::core::mem::size_of::<[::core::ffi::c_char; 128]>()
                                        as ::core::ffi::c_int,
                                );
                            }
                            if strcmp(
                                &raw mut tok as *mut ::core::ffi::c_char,
                                b"typedef\0" as *const u8 as *const ::core::ffi::c_char,
                            ) == 0 as ::core::ffi::c_int
                            {
                                in_typedef = 1 as ::core::ffi::c_int;
                            }
                            mystrscpy(
                                &raw mut prev1 as *mut ::core::ffi::c_char,
                                &raw mut tok as *mut ::core::ffi::c_char,
                                ::core::mem::size_of::<[::core::ffi::c_char; 128]>()
                                    as ::core::ffi::c_int,
                            );
                            tok_len = 0 as size_t;
                        }
                        if c as ::core::ffi::c_char as ::core::ffi::c_int == '(' as i32
                            && brace_depth == 0 as ::core::ffi::c_int
                            && prev1[0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int != '\0' as i32
                        {
                            if is_c_control_kw(
                                &raw mut prev1 as *mut ::core::ffi::c_char,
                            ) == 0
                                && strcmp(
                                    &raw mut prev1 as *mut ::core::ffi::c_char,
                                    b"struct\0" as *const u8 as *const ::core::ffi::c_char,
                                ) != 0 as ::core::ffi::c_int
                                && strcmp(
                                    &raw mut prev1 as *mut ::core::ffi::c_char,
                                    b"union\0" as *const u8 as *const ::core::ffi::c_char,
                                ) != 0 as ::core::ffi::c_int
                                && strcmp(
                                    &raw mut prev1 as *mut ::core::ffi::c_char,
                                    b"enum\0" as *const u8 as *const ::core::ffi::c_char,
                                ) != 0 as ::core::ffi::c_int
                                && strcmp(
                                    &raw mut prev1 as *mut ::core::ffi::c_char,
                                    b"typedef\0" as *const u8 as *const ::core::ffi::c_char,
                                ) != 0 as ::core::ffi::c_int
                            {
                                pool_add(pool, &raw mut prev1 as *mut ::core::ffi::c_char);
                            }
                        }
                        if c as ::core::ffi::c_char as ::core::ffi::c_int == ';' as i32
                            && in_typedef != 0 && brace_depth == 0 as ::core::ffi::c_int
                        {
                            if last_ident[0 as ::core::ffi::c_int as usize] != 0 {
                                pool_add(
                                    pool,
                                    &raw mut last_ident as *mut ::core::ffi::c_char,
                                );
                            }
                            in_typedef = 0 as ::core::ffi::c_int;
                            last_ident[0 as ::core::ffi::c_int as usize] = '\0' as i32
                                as ::core::ffi::c_char;
                        }
                        if c as ::core::ffi::c_char as ::core::ffi::c_int == '{' as i32 {
                            brace_depth += 1;
                        } else if c as ::core::ffi::c_char as ::core::ffi::c_int
                            == '}' as i32 && brace_depth > 0 as ::core::ffi::c_int
                        {
                            brace_depth -= 1;
                        }
                    }
                }
            }
            1 => {
                if c as ::core::ffi::c_int == '\n' as i32 {
                    state = ST_NORMAL_1;
                }
            }
            2 => {
                if c as ::core::ffi::c_int == '*' as i32
                    && next as ::core::ffi::c_int == '/' as i32
                {
                    state = ST_NORMAL_1;
                    i = i.wrapping_add(1);
                }
            }
            3 => {
                if c as ::core::ffi::c_int == '\\' as i32 {
                    i = i.wrapping_add(1);
                } else if c as ::core::ffi::c_int == '"' as i32 {
                    state = ST_NORMAL_1;
                }
            }
            4 => {
                if c as ::core::ffi::c_int == '\\' as i32 {
                    i = i.wrapping_add(1);
                } else if c as ::core::ffi::c_int == '\'' as i32 {
                    state = ST_NORMAL_1;
                }
            }
            _ => {}
        }
        i = i.wrapping_add(1);
    }
    if state as ::core::ffi::c_uint
        == ST_NORMAL_1 as ::core::ffi::c_int as ::core::ffi::c_uint
        && tok_len > 0 as size_t
    {
        tok[tok_len as usize] = '\0' as i32 as ::core::ffi::c_char;
        if strcmp(
            &raw mut prev1 as *mut ::core::ffi::c_char,
            b"struct\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                &raw mut prev1 as *mut ::core::ffi::c_char,
                b"union\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcmp(
                &raw mut prev1 as *mut ::core::ffi::c_char,
                b"enum\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            pool_add(pool, &raw mut tok as *mut ::core::ffi::c_char);
        }
        if in_typedef != 0 {
            pool_add(pool, &raw mut tok as *mut ::core::ffi::c_char);
        }
    }
}
unsafe extern "C" fn extract_python_symbols_from_text(
    mut text: *const ::core::ffi::c_char,
    mut text_len: size_t,
    mut pool: *mut completion_pool_t,
) {
    let mut current_block: u64;
    let mut state: C2RustUnnamed_1 = ST_NORMAL_0;
    let mut tok: [::core::ffi::c_char; 128] = [0; 128];
    let mut tok_len: size_t = 0 as size_t;
    let mut prev1: [::core::ffi::c_char; 128] = [0; 128];
    prev1[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
    let mut triple_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: size_t = 0 as size_t;
    while i < text_len {
        let mut c: ::core::ffi::c_uchar = *text.offset(i as isize)
            as ::core::ffi::c_uchar;
        let mut next: ::core::ffi::c_uchar = (if i.wrapping_add(1 as size_t) < text_len {
            *text.offset(i.wrapping_add(1 as size_t) as isize) as ::core::ffi::c_uchar
                as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uchar;
        let mut next2: ::core::ffi::c_uchar = (if i.wrapping_add(2 as size_t) < text_len
        {
            *text.offset(i.wrapping_add(2 as size_t) as isize) as ::core::ffi::c_uchar
                as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uchar;
        match state as ::core::ffi::c_uint {
            0 => {
                if c as ::core::ffi::c_int == '#' as i32 {
                    state = ST_LINE_CMT_0;
                    current_block = 1024377671772024164;
                } else if c as ::core::ffi::c_int == '"' as i32
                    && next as ::core::ffi::c_int == '"' as i32
                    && next2 as ::core::ffi::c_int == '"' as i32
                {
                    state = ST_STRING3;
                    i = i.wrapping_add(2 as size_t);
                    triple_count = 0 as ::core::ffi::c_int;
                    current_block = 1024377671772024164;
                } else if c as ::core::ffi::c_int == '"' as i32
                    || c as ::core::ffi::c_int == '\'' as i32
                {
                    state = ST_STRING1;
                    current_block = 1024377671772024164;
                } else if *(*__ctype_b_loc()).offset(c as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & _ISalnum as ::core::ffi::c_int as ::core::ffi::c_ushort
                        as ::core::ffi::c_int != 0
                    || c as ::core::ffi::c_int == '_' as i32
                {
                    if tok_len
                        < (::core::mem::size_of::<[::core::ffi::c_char; 128]>() as usize)
                            .wrapping_sub(1 as usize)
                    {
                        let fresh16 = tok_len;
                        tok_len = tok_len.wrapping_add(1);
                        tok[fresh16 as usize] = c as ::core::ffi::c_char;
                    }
                    current_block = 12675440807659640239;
                } else {
                    current_block = 1024377671772024164;
                }
                match current_block {
                    12675440807659640239 => {}
                    _ => {
                        if tok_len > 0 as size_t {
                            tok[tok_len as usize] = '\0' as i32 as ::core::ffi::c_char;
                            if strcmp(
                                &raw mut prev1 as *mut ::core::ffi::c_char,
                                b"class\0" as *const u8 as *const ::core::ffi::c_char,
                            ) == 0 as ::core::ffi::c_int
                                || strcmp(
                                    &raw mut prev1 as *mut ::core::ffi::c_char,
                                    b"def\0" as *const u8 as *const ::core::ffi::c_char,
                                ) == 0 as ::core::ffi::c_int
                            {
                                pool_add(pool, &raw mut tok as *mut ::core::ffi::c_char);
                            }
                            mystrscpy(
                                &raw mut prev1 as *mut ::core::ffi::c_char,
                                &raw mut tok as *mut ::core::ffi::c_char,
                                ::core::mem::size_of::<[::core::ffi::c_char; 128]>()
                                    as ::core::ffi::c_int,
                            );
                            tok_len = 0 as size_t;
                        }
                    }
                }
            }
            1 => {
                if c as ::core::ffi::c_int == '\n' as i32 {
                    state = ST_NORMAL_0;
                }
            }
            3 => {
                if c as ::core::ffi::c_int == '"' as i32 {
                    triple_count += 1;
                    if triple_count == 3 as ::core::ffi::c_int {
                        state = ST_NORMAL_0;
                    }
                } else {
                    triple_count = 0 as ::core::ffi::c_int;
                }
            }
            2 => {
                if c as ::core::ffi::c_int == '\\' as i32 {
                    i = i.wrapping_add(1);
                } else if c as ::core::ffi::c_int == '"' as i32
                    || c as ::core::ffi::c_int == '\'' as i32
                {
                    state = ST_NORMAL_0;
                }
            }
            _ => {}
        }
        i = i.wrapping_add(1);
    }
    if state as ::core::ffi::c_uint
        == ST_NORMAL_0 as ::core::ffi::c_int as ::core::ffi::c_uint
        && tok_len > 0 as size_t
    {
        tok[tok_len as usize] = '\0' as i32 as ::core::ffi::c_char;
        if strcmp(
            &raw mut prev1 as *mut ::core::ffi::c_char,
            b"class\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                &raw mut prev1 as *mut ::core::ffi::c_char,
                b"def\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            pool_add(pool, &raw mut tok as *mut ::core::ffi::c_char);
        }
    }
}
unsafe extern "C" fn extract_js_symbols_from_text(
    mut text: *const ::core::ffi::c_char,
    mut text_len: size_t,
    mut pool: *mut completion_pool_t,
) {
    let mut current_block: u64;
    let mut state: C2RustUnnamed_0 = ST_NORMAL;
    let mut tok: [::core::ffi::c_char; 128] = [0; 128];
    let mut tok_len: size_t = 0 as size_t;
    let mut prev1: [::core::ffi::c_char; 128] = [0; 128];
    let mut prev2: [::core::ffi::c_char; 128] = [0; 128];
    prev1[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
    prev2[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
    let mut i: size_t = 0 as size_t;
    while i < text_len {
        let mut c: ::core::ffi::c_uchar = *text.offset(i as isize)
            as ::core::ffi::c_uchar;
        let mut next: ::core::ffi::c_uchar = (if i.wrapping_add(1 as size_t) < text_len {
            *text.offset(i.wrapping_add(1 as size_t) as isize) as ::core::ffi::c_uchar
                as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uchar;
        match state as ::core::ffi::c_uint {
            0 => {
                if c as ::core::ffi::c_int == '/' as i32
                    && next as ::core::ffi::c_int == '/' as i32
                {
                    state = ST_LINE_CMT;
                    i = i.wrapping_add(1);
                    current_block = 1364665306248054071;
                } else if c as ::core::ffi::c_int == '/' as i32
                    && next as ::core::ffi::c_int == '*' as i32
                {
                    state = ST_BLOCK_CMT;
                    i = i.wrapping_add(1);
                    current_block = 1364665306248054071;
                } else if c as ::core::ffi::c_int == '\'' as i32 {
                    state = ST_STRING_SQ;
                    current_block = 1364665306248054071;
                } else if c as ::core::ffi::c_int == '"' as i32 {
                    state = ST_STRING_DQ;
                    current_block = 1364665306248054071;
                } else if c as ::core::ffi::c_int == '`' as i32 {
                    state = ST_TEMPLATE;
                    current_block = 1364665306248054071;
                } else if *(*__ctype_b_loc()).offset(c as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & _ISalnum as ::core::ffi::c_int as ::core::ffi::c_ushort
                        as ::core::ffi::c_int != 0
                    || c as ::core::ffi::c_int == '_' as i32
                    || c as ::core::ffi::c_int == '$' as i32
                {
                    if tok_len
                        < (::core::mem::size_of::<[::core::ffi::c_char; 128]>() as usize)
                            .wrapping_sub(1 as usize)
                    {
                        let fresh15 = tok_len;
                        tok_len = tok_len.wrapping_add(1);
                        tok[fresh15 as usize] = c as ::core::ffi::c_char;
                    }
                    current_block = 4644295000439058019;
                } else {
                    current_block = 1364665306248054071;
                }
                match current_block {
                    4644295000439058019 => {}
                    _ => {
                        if tok_len > 0 as size_t {
                            tok[tok_len as usize] = '\0' as i32 as ::core::ffi::c_char;
                            if strcmp(
                                &raw mut prev1 as *mut ::core::ffi::c_char,
                                b"function\0" as *const u8 as *const ::core::ffi::c_char,
                            ) == 0 as ::core::ffi::c_int
                                || strcmp(
                                    &raw mut prev1 as *mut ::core::ffi::c_char,
                                    b"class\0" as *const u8 as *const ::core::ffi::c_char,
                                ) == 0 as ::core::ffi::c_int
                            {
                                pool_add(pool, &raw mut tok as *mut ::core::ffi::c_char);
                            }
                            mystrscpy(
                                &raw mut prev2 as *mut ::core::ffi::c_char,
                                &raw mut prev1 as *mut ::core::ffi::c_char,
                                ::core::mem::size_of::<[::core::ffi::c_char; 128]>()
                                    as ::core::ffi::c_int,
                            );
                            mystrscpy(
                                &raw mut prev1 as *mut ::core::ffi::c_char,
                                &raw mut tok as *mut ::core::ffi::c_char,
                                ::core::mem::size_of::<[::core::ffi::c_char; 128]>()
                                    as ::core::ffi::c_int,
                            );
                            tok_len = 0 as size_t;
                        } else if c as ::core::ffi::c_char as ::core::ffi::c_int
                            == '=' as i32
                        {
                            mystrscpy(
                                &raw mut prev2 as *mut ::core::ffi::c_char,
                                &raw mut prev1 as *mut ::core::ffi::c_char,
                                ::core::mem::size_of::<[::core::ffi::c_char; 128]>()
                                    as ::core::ffi::c_int,
                            );
                            mystrscpy(
                                &raw mut prev1 as *mut ::core::ffi::c_char,
                                b"=\0" as *const u8 as *const ::core::ffi::c_char,
                                ::core::mem::size_of::<[::core::ffi::c_char; 128]>()
                                    as ::core::ffi::c_int,
                            );
                        }
                        if c as ::core::ffi::c_char as ::core::ffi::c_int == '(' as i32
                            && prev1[0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int != '\0' as i32
                            && strcmp(
                                &raw mut prev1 as *mut ::core::ffi::c_char,
                                b"function\0" as *const u8 as *const ::core::ffi::c_char,
                            ) != 0 as ::core::ffi::c_int
                            && strcmp(
                                &raw mut prev1 as *mut ::core::ffi::c_char,
                                b"if\0" as *const u8 as *const ::core::ffi::c_char,
                            ) != 0 as ::core::ffi::c_int
                            && strcmp(
                                &raw mut prev1 as *mut ::core::ffi::c_char,
                                b"while\0" as *const u8 as *const ::core::ffi::c_char,
                            ) != 0 as ::core::ffi::c_int
                            && strcmp(
                                &raw mut prev1 as *mut ::core::ffi::c_char,
                                b"for\0" as *const u8 as *const ::core::ffi::c_char,
                            ) != 0 as ::core::ffi::c_int
                            && strcmp(
                                &raw mut prev1 as *mut ::core::ffi::c_char,
                                b"switch\0" as *const u8 as *const ::core::ffi::c_char,
                            ) != 0 as ::core::ffi::c_int
                        {
                            if strcmp(
                                &raw mut prev2 as *mut ::core::ffi::c_char,
                                b"function\0" as *const u8 as *const ::core::ffi::c_char,
                            ) == 0 as ::core::ffi::c_int
                                || strcmp(
                                    &raw mut prev2 as *mut ::core::ffi::c_char,
                                    b"async\0" as *const u8 as *const ::core::ffi::c_char,
                                ) == 0 as ::core::ffi::c_int
                            {
                                pool_add(pool, &raw mut prev1 as *mut ::core::ffi::c_char);
                            }
                        }
                    }
                }
            }
            1 => {
                if c as ::core::ffi::c_int == '\n' as i32 {
                    state = ST_NORMAL;
                }
            }
            2 => {
                if c as ::core::ffi::c_int == '*' as i32
                    && next as ::core::ffi::c_int == '/' as i32
                {
                    state = ST_NORMAL;
                    i = i.wrapping_add(1);
                }
            }
            3 => {
                if c as ::core::ffi::c_int == '\\' as i32 {
                    i = i.wrapping_add(1);
                } else if c as ::core::ffi::c_int == '\'' as i32 {
                    state = ST_NORMAL;
                }
            }
            4 => {
                if c as ::core::ffi::c_int == '\\' as i32 {
                    i = i.wrapping_add(1);
                } else if c as ::core::ffi::c_int == '"' as i32 {
                    state = ST_NORMAL;
                }
            }
            5 => {
                if c as ::core::ffi::c_int == '\\' as i32 {
                    i = i.wrapping_add(1);
                } else if c as ::core::ffi::c_int == '`' as i32 {
                    state = ST_NORMAL;
                }
            }
            _ => {}
        }
        i = i.wrapping_add(1);
    }
    if state as ::core::ffi::c_uint
        == ST_NORMAL as ::core::ffi::c_int as ::core::ffi::c_uint
        && tok_len > 0 as size_t
    {
        tok[tok_len as usize] = '\0' as i32 as ::core::ffi::c_char;
        if strcmp(
            &raw mut prev1 as *mut ::core::ffi::c_char,
            b"function\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                &raw mut prev1 as *mut ::core::ffi::c_char,
                b"class\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            pool_add(pool, &raw mut tok as *mut ::core::ffi::c_char);
        }
    }
}
unsafe extern "C" fn extract_symbols_from_text(
    mut fname: *const ::core::ffi::c_char,
    mut text: *const ::core::ffi::c_char,
    mut text_len: size_t,
    mut pool: *mut completion_pool_t,
) {
    if fname.is_null() || text.is_null() || text_len == 0 as size_t {
        return;
    }
    if is_c_like_file(fname) != 0 {
        extract_c_symbols_from_text(text, text_len, pool);
    } else if is_python_file(fname) != 0 {
        extract_python_symbols_from_text(text, text_len, pool);
    } else if is_node_file(fname) != 0 {
        extract_js_symbols_from_text(text, text_len, pool);
    }
}
unsafe extern "C" fn scan_buffer_for_source_symbols(
    mut bp: *mut buffer,
    mut pool: *mut completion_pool_t,
) {
    if bp.is_null() || pool.is_null() {
        return;
    }
    if (*bp).b_fname[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        == '\0' as i32
    {
        return;
    }
    let mut lp: *mut line = (*(*bp).b_linep).l_fp;
    let mut total: size_t = 0 as size_t;
    while lp != (*bp).b_linep {
        total = total.wrapping_add(((*lp).l_used as size_t).wrapping_add(1 as size_t));
        lp = (*lp).l_fp;
        if total >= MAX_SOURCE_FILE_BYTES as size_t {
            break;
        }
    }
    let mut buf: *mut ::core::ffi::c_char = malloc(total.wrapping_add(1 as size_t))
        as *mut ::core::ffi::c_char;
    if buf.is_null() {
        return;
    }
    let mut pos: size_t = 0 as size_t;
    lp = (*(*bp).b_linep).l_fp;
    while lp != (*bp).b_linep && pos < total {
        let mut len: ::core::ffi::c_int = (*lp).l_used;
        if len > 0 as ::core::ffi::c_int {
            let mut copy: size_t = len as size_t;
            if pos.wrapping_add(copy) > total {
                copy = total.wrapping_sub(pos);
            }
            memcpy(
                buf.offset(pos as isize) as *mut ::core::ffi::c_void,
                &raw mut (*lp).l_text as *mut ::core::ffi::c_uchar
                    as *const ::core::ffi::c_void,
                copy,
            );
            pos = pos.wrapping_add(copy);
        }
        if pos < total {
            let fresh18 = pos;
            pos = pos.wrapping_add(1);
            *buf.offset(fresh18 as isize) = '\n' as i32 as ::core::ffi::c_char;
        }
        lp = (*lp).l_fp;
    }
    *buf.offset(pos as isize) = '\0' as i32 as ::core::ffi::c_char;
    extract_symbols_from_text(
        &raw mut (*bp).b_fname as *mut ::core::ffi::c_char,
        buf,
        pos,
        pool,
    );
    free(buf as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn scan_file_for_source_symbols(
    mut path: *const ::core::ffi::c_char,
    mut pool: *mut completion_pool_t,
) {
    if path.is_null()
        || *path.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '\0' as i32 || pool.is_null()
    {
        return;
    }
    let mut fp: *mut FILE = fopen(
        path,
        b"r\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if fp.is_null() {
        return;
    }
    let mut buf: *mut ::core::ffi::c_char = malloc(
        (MAX_SOURCE_FILE_BYTES + 1 as ::core::ffi::c_int) as size_t,
    ) as *mut ::core::ffi::c_char;
    if buf.is_null() {
        fclose(fp);
        return;
    }
    let mut total: size_t = fread(
        buf as *mut ::core::ffi::c_void,
        1 as size_t,
        MAX_SOURCE_FILE_BYTES as size_t,
        fp,
    ) as size_t;
    fclose(fp);
    *buf.offset(total as isize) = '\0' as i32 as ::core::ffi::c_char;
    extract_symbols_from_text(path, buf, total, pool);
    free(buf as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn source_symbol_cache_clear() {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < source_symbol_cache.count {
        free(*source_symbol_cache.items.offset(i as isize) as *mut ::core::ffi::c_void);
        let ref mut fresh19 = *source_symbol_cache.items.offset(i as isize);
        *fresh19 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        i += 1;
    }
    source_symbol_cache.count = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn refresh_source_symbols() {
    let mut cur_fname: *const ::core::ffi::c_char = if !curbp.is_null()
        && (*curbp).b_fname[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 0
    {
        &raw mut (*curbp).b_fname as *mut ::core::ffi::c_char
            as *const ::core::ffi::c_char
    } else {
        b"\0" as *const u8 as *const ::core::ffi::c_char
    };
    if strcmp(&raw mut source_symbol_last_fname as *mut ::core::ffi::c_char, cur_fname)
        == 0 as ::core::ffi::c_int && source_symbol_cache.count > 0 as ::core::ffi::c_int
    {
        return;
    }
    source_symbol_cache_clear();
    mystrscpy(
        &raw mut source_symbol_last_fname as *mut ::core::ffi::c_char,
        cur_fname,
        ::core::mem::size_of::<[::core::ffi::c_char; 2048]>() as ::core::ffi::c_int,
    );
    if !curbp.is_null() {
        scan_buffer_for_source_symbols(curbp, &raw mut source_symbol_cache);
    }
    let mut slot: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while slot < 4 as ::core::ffi::c_int {
        if !(file_reserve[slot as usize][0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int == '\0' as i32)
        {
            if !(*cur_fname.offset(0 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int != 0
                && strcmp(
                    &raw mut *(&raw mut file_reserve as *mut [::core::ffi::c_char; 4096])
                        .offset(slot as isize) as *mut ::core::ffi::c_char,
                    cur_fname,
                ) == 0 as ::core::ffi::c_int)
            {
                let mut bp: *mut buffer = bheadp;
                let mut found_buf: ::core::ffi::c_int = FALSE;
                while !bp.is_null() {
                    if strcmp(
                        &raw mut (*bp).b_fname as *mut ::core::ffi::c_char,
                        &raw mut *(&raw mut file_reserve
                            as *mut [::core::ffi::c_char; 4096])
                            .offset(slot as isize) as *mut ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        scan_buffer_for_source_symbols(bp, &raw mut source_symbol_cache);
                        found_buf = TRUE;
                        break;
                    } else {
                        bp = (*bp).b_bufp;
                    }
                }
                if found_buf == 0 {
                    scan_file_for_source_symbols(
                        &raw mut *(&raw mut file_reserve
                            as *mut [::core::ffi::c_char; 4096])
                            .offset(slot as isize) as *mut ::core::ffi::c_char,
                        &raw mut source_symbol_cache,
                    );
                }
            }
        }
        slot += 1;
    }
}
unsafe extern "C" fn collect_source_symbol_matches(
    mut prefix: *const ::core::ffi::c_char,
    mut ctx: completion_context_t,
) {
    if ctx as ::core::ffi::c_uint
        == COMPLETION_CONTEXT_PATH as ::core::ffi::c_int as ::core::ffi::c_uint
        || prefix.is_null() || *prefix as ::core::ffi::c_int == '\0' as i32
    {
        return;
    }
    refresh_source_symbols();
    add_matches_from_pool(&raw mut source_symbol_cache, prefix);
}
unsafe extern "C" fn runtime_completion_callback(
    mut symbol: *const ::core::ffi::c_char,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut ctx: *mut runtime_completion_ctx_t = data as *mut runtime_completion_ctx_t;
    if !ctx.is_null() && !symbol.is_null() {
        completion_consider_candidate(symbol, (*ctx).prefix);
    }
}
unsafe extern "C" fn add_runtime_module_matches(
    mut lang: scraper_lang_t,
    mut module: *const ::core::ffi::c_char,
    mut prefix: *const ::core::ffi::c_char,
) {
    if module.is_null() || *module == 0 || prefix.is_null() {
        return;
    }
    let mut ctx: runtime_completion_ctx_t = runtime_completion_ctx_t {
        prefix: prefix,
    };
    scraper_iterate_symbols(
        lang,
        module,
        Some(
            runtime_completion_callback
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
        &raw mut ctx as *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" fn get_owner_symbol_near_cursor(
    mut lp: *mut line,
    mut prefix_start: ::core::ffi::c_int,
    mut out: *mut ::core::ffi::c_char,
    mut outsz: size_t,
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = 0;
    let mut dot_pos: ::core::ffi::c_int = 0;
    let mut uc: unicode_t = 0;
    let mut owner_start: ::core::ffi::c_int = 0;
    let mut owner_end: ::core::ffi::c_int = 0;
    if lp.is_null() || prefix_start <= 0 as ::core::ffi::c_int || out.is_null()
        || outsz == 0 as size_t
    {
        return FALSE;
    }
    len = (*lp).l_used;
    dot_pos = prev_char_start(lp, prefix_start);
    if dot_pos <= 0 as ::core::ffi::c_int {
        return FALSE;
    }
    if utf8_to_unicode(
        &raw mut (*lp).l_text as *mut ::core::ffi::c_uchar,
        dot_pos as ::core::ffi::c_uint,
        len as ::core::ffi::c_uint,
        &raw mut uc,
    ) <= 0 as ::core::ffi::c_uint
    {
        return FALSE;
    }
    if uc != '.' as i32 as unicode_t {
        return FALSE;
    }
    owner_end = dot_pos;
    owner_start = owner_end;
    while owner_start > 0 as ::core::ffi::c_int {
        let mut candidate: ::core::ffi::c_int = prev_char_start(lp, owner_start);
        if candidate == owner_start {
            break;
        }
        if utf8_to_unicode(
            &raw mut (*lp).l_text as *mut ::core::ffi::c_uchar,
            candidate as ::core::ffi::c_uint,
            len as ::core::ffi::c_uint,
            &raw mut uc,
        ) <= 0 as ::core::ffi::c_uint
        {
            break;
        }
        if is_identifier_char(uc) == 0 {
            break;
        }
        owner_start = candidate;
    }
    if owner_start == owner_end {
        return FALSE;
    }
    let mut copy_len: ::core::ffi::c_int = owner_end - owner_start;
    if copy_len >= outsz as ::core::ffi::c_int {
        copy_len = outsz as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    }
    memcpy(
        out as *mut ::core::ffi::c_void,
        (&raw mut (*lp).l_text as *mut ::core::ffi::c_uchar).offset(owner_start as isize)
            as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
        copy_len as size_t,
    );
    *out.offset(copy_len as isize) = '\0' as i32 as ::core::ffi::c_char;
    return TRUE;
}
unsafe extern "C" fn resolve_java_class_name(
    mut owner: *const ::core::ffi::c_char,
    mut out: *mut ::core::ffi::c_char,
    mut outsz: size_t,
) -> ::core::ffi::c_int {
    let mut lp: *mut line = ::core::ptr::null_mut::<line>();
    let mut linebuf: [::core::ffi::c_char; 256] = [0; 256];
    if owner.is_null() || out.is_null() || outsz == 0 as size_t {
        return FALSE;
    }
    if !strchr(owner, '.' as i32).is_null() {
        mystrscpy(out, owner, outsz as ::core::ffi::c_int);
        return TRUE;
    }
    if curbp.is_null() {
        return FALSE;
    }
    lp = (*(*curbp).b_linep).l_fp;
    while lp != (*curbp).b_linep {
        let mut len: ::core::ffi::c_int = (*lp).l_used;
        if len
            >= ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as ::core::ffi::c_int
        {
            len = (::core::mem::size_of::<[::core::ffi::c_char; 256]>() as usize)
                .wrapping_sub(1 as usize) as ::core::ffi::c_int;
        }
        memcpy(
            &raw mut linebuf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            &raw mut (*lp).l_text as *mut ::core::ffi::c_uchar
                as *const ::core::ffi::c_void,
            len as size_t,
        );
        linebuf[len as usize] = '\0' as i32 as ::core::ffi::c_char;
        let mut p: *mut ::core::ffi::c_char = &raw mut linebuf
            as *mut ::core::ffi::c_char;
        while *p as ::core::ffi::c_int == ' ' as i32
            || *p as ::core::ffi::c_int == '\t' as i32
        {
            p = p.offset(1);
        }
        if strncmp(
            p,
            b"import \0" as *const u8 as *const ::core::ffi::c_char,
            7 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            p = p.offset(7 as ::core::ffi::c_int as isize);
            while *p as ::core::ffi::c_int == ' ' as i32
                || *p as ::core::ffi::c_int == '\t' as i32
            {
                p = p.offset(1);
            }
            let mut semi: *mut ::core::ffi::c_char = strchr(p, ';' as i32);
            if !semi.is_null() {
                *semi = '\0' as i32 as ::core::ffi::c_char;
            }
            let mut star: *mut ::core::ffi::c_char = strstr(
                p,
                b".*\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if !star.is_null() {
                *star = '\0' as i32 as ::core::ffi::c_char;
                snprintf(
                    out,
                    outsz,
                    b"%s.%s\0" as *const u8 as *const ::core::ffi::c_char,
                    p,
                    owner,
                );
                return TRUE;
            } else {
                let mut simple: *mut ::core::ffi::c_char = strrchr(p, '.' as i32);
                if !simple.is_null()
                    && strcmp(simple.offset(1 as ::core::ffi::c_int as isize), owner)
                        == 0 as ::core::ffi::c_int
                {
                    mystrscpy(out, p, outsz as ::core::ffi::c_int);
                    return TRUE;
                }
            }
        }
        lp = (*lp).l_fp;
    }
    snprintf(
        out,
        outsz,
        b"java.lang.%s\0" as *const u8 as *const ::core::ffi::c_char,
        owner,
    );
    return TRUE;
}
unsafe extern "C" fn find_java_member_entry(
    mut class_name: *const ::core::ffi::c_char,
) -> *mut java_member_entry_t {
    if class_name.is_null() {
        return ::core::ptr::null_mut::<java_member_entry_t>();
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < java_member_cache_count {
        if strcmp((*java_member_cache.offset(i as isize)).class_name, class_name)
            == 0 as ::core::ffi::c_int
        {
            return java_member_cache.offset(i as isize) as *mut java_member_entry_t;
        }
        i += 1;
    }
    return ::core::ptr::null_mut::<java_member_entry_t>();
}
unsafe extern "C" fn get_java_member_entry(
    mut class_name: *const ::core::ffi::c_char,
) -> *mut java_member_entry_t {
    let mut entry: *mut java_member_entry_t = find_java_member_entry(class_name);
    if !entry.is_null() {
        return entry;
    }
    if class_name.is_null() || *class_name as ::core::ffi::c_int == '\0' as i32 {
        return ::core::ptr::null_mut::<java_member_entry_t>();
    }
    if java_member_cache_count == java_member_cache_capacity {
        let mut new_capacity: ::core::ffi::c_int = if java_member_cache_capacity != 0 {
            java_member_cache_capacity * 2 as ::core::ffi::c_int
        } else {
            16 as ::core::ffi::c_int
        };
        let mut tmp: *mut java_member_entry_t = realloc(
            java_member_cache as *mut ::core::ffi::c_void,
            (new_capacity as size_t)
                .wrapping_mul(::core::mem::size_of::<java_member_entry_t>() as size_t),
        ) as *mut java_member_entry_t;
        if tmp.is_null() {
            return ::core::ptr::null_mut::<java_member_entry_t>();
        }
        java_member_cache = tmp;
        java_member_cache_capacity = new_capacity;
    }
    let fresh14 = java_member_cache_count;
    java_member_cache_count = java_member_cache_count + 1;
    entry = java_member_cache.offset(fresh14 as isize) as *mut java_member_entry_t;
    (*entry).class_name = strdup(class_name);
    (*entry).members.items = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    (*entry).members.count = 0 as ::core::ffi::c_int;
    (*entry).members.capacity = 0 as ::core::ffi::c_int;
    (*entry).members.max_items = MAX_JAVA_MEMBERS;
    (*entry).loaded = 0 as ::core::ffi::c_int;
    return entry;
}
unsafe extern "C" fn load_javap_members(mut entry: *mut java_member_entry_t) {
    let mut quoted_cp: [::core::ffi::c_char; 8192] = [0; 8192];
    let mut quoted_class: [::core::ffi::c_char; 512] = [0; 512];
    let mut cmd: [::core::ffi::c_char; 8704] = [0; 8704];
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut line: [::core::ffi::c_char; 1024] = [0; 1024];
    if entry.is_null() || (*entry).loaded != 0 {
        return;
    }
    if build_shell_quoted(
        get_java_classpath_string(),
        &raw mut quoted_cp as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 8192]>() as size_t,
    ) == 0
    {
        (*entry).loaded = 1 as ::core::ffi::c_int;
        return;
    }
    if build_shell_quoted(
        (*entry).class_name,
        &raw mut quoted_class as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as size_t,
    ) == 0
    {
        (*entry).loaded = 1 as ::core::ffi::c_int;
        return;
    }
    mystrscpy(
        &raw mut cmd as *mut ::core::ffi::c_char,
        b"javap -classpath \0" as *const u8 as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 8704]>() as ::core::ffi::c_int,
    );
    let mut pos: size_t = strlen(&raw mut cmd as *mut ::core::ffi::c_char);
    if pos
        .wrapping_add(strlen(&raw mut quoted_cp as *mut ::core::ffi::c_char))
        .wrapping_add(1 as size_t)
        >= ::core::mem::size_of::<[::core::ffi::c_char; 8704]>() as usize
    {
        (*entry).loaded = 1 as ::core::ffi::c_int;
        return;
    }
    memcpy(
        (&raw mut cmd as *mut ::core::ffi::c_char).offset(pos as isize)
            as *mut ::core::ffi::c_void,
        &raw mut quoted_cp as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        strlen(&raw mut quoted_cp as *mut ::core::ffi::c_char),
    );
    pos = pos.wrapping_add(strlen(&raw mut quoted_cp as *mut ::core::ffi::c_char));
    if pos.wrapping_add(1 as size_t)
        >= ::core::mem::size_of::<[::core::ffi::c_char; 8704]>() as usize
    {
        (*entry).loaded = 1 as ::core::ffi::c_int;
        return;
    }
    let fresh3 = pos;
    pos = pos.wrapping_add(1);
    cmd[fresh3 as usize] = ' ' as i32 as ::core::ffi::c_char;
    if pos
        .wrapping_add(strlen(&raw mut quoted_class as *mut ::core::ffi::c_char))
        .wrapping_add(1 as size_t)
        >= ::core::mem::size_of::<[::core::ffi::c_char; 8704]>() as usize
    {
        (*entry).loaded = 1 as ::core::ffi::c_int;
        return;
    }
    memcpy(
        (&raw mut cmd as *mut ::core::ffi::c_char).offset(pos as isize)
            as *mut ::core::ffi::c_void,
        &raw mut quoted_class as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        strlen(&raw mut quoted_class as *mut ::core::ffi::c_char),
    );
    pos = pos.wrapping_add(strlen(&raw mut quoted_class as *mut ::core::ffi::c_char));
    cmd[pos as usize] = '\0' as i32 as ::core::ffi::c_char;
    fp = popen(
        &raw mut cmd as *mut ::core::ffi::c_char,
        b"r\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if fp.is_null() {
        (*entry).loaded = 1 as ::core::ffi::c_int;
        return;
    }
    while !fgets(
            &raw mut line as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as ::core::ffi::c_int,
            fp,
        )
        .is_null()
    {
        let mut p: *mut ::core::ffi::c_char = &raw mut line as *mut ::core::ffi::c_char;
        while *p as ::core::ffi::c_int == ' ' as i32
            || *p as ::core::ffi::c_int == '\t' as i32
        {
            p = p.offset(1);
        }
        if *p as ::core::ffi::c_int == '\0' as i32
            || *p as ::core::ffi::c_int == '{' as i32
            || *p as ::core::ffi::c_int == '}' as i32
            || strncmp(
                p,
                b"Compiled from\0" as *const u8 as *const ::core::ffi::c_char,
                13 as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            continue;
        }
        let mut paren: *mut ::core::ffi::c_char = strchr(p, '(' as i32);
        if !paren.is_null() {
            let mut name_start: *mut ::core::ffi::c_char = paren;
            while name_start > p
                && (*(*__ctype_b_loc())
                    .offset(
                        *name_start.offset(-(1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_uchar as ::core::ffi::c_int as isize,
                    ) as ::core::ffi::c_int
                    & _ISalnum as ::core::ffi::c_int as ::core::ffi::c_ushort
                        as ::core::ffi::c_int != 0
                    || *name_start.offset(-(1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int == '_' as i32
                    || *name_start.offset(-(1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int == '$' as i32)
            {
                name_start = name_start.offset(-1);
            }
            if name_start < paren {
                add_symbol_token(
                    &raw mut (*entry).members,
                    name_start,
                    paren.offset_from(name_start) as ::core::ffi::c_long as size_t,
                );
            }
        } else {
            let mut semi: *mut ::core::ffi::c_char = strchr(p, ';' as i32);
            if !semi.is_null() {
                let mut name_end: *mut ::core::ffi::c_char = semi;
                while name_end > p
                    && *(*__ctype_b_loc())
                        .offset(
                            *name_end.offset(-(1 as ::core::ffi::c_int) as isize)
                                as ::core::ffi::c_uchar as ::core::ffi::c_int as isize,
                        ) as ::core::ffi::c_int
                        & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                            as ::core::ffi::c_int != 0
                {
                    name_end = name_end.offset(-1);
                }
                let mut name_start_0: *mut ::core::ffi::c_char = name_end;
                while name_start_0 > p
                    && (*(*__ctype_b_loc())
                        .offset(
                            *name_start_0.offset(-(1 as ::core::ffi::c_int) as isize)
                                as ::core::ffi::c_uchar as ::core::ffi::c_int as isize,
                        ) as ::core::ffi::c_int
                        & _ISalnum as ::core::ffi::c_int as ::core::ffi::c_ushort
                            as ::core::ffi::c_int != 0
                        || *name_start_0.offset(-(1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int == '_' as i32
                        || *name_start_0.offset(-(1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int == '$' as i32)
                {
                    name_start_0 = name_start_0.offset(-1);
                }
                if name_start_0 < name_end {
                    add_symbol_token(
                        &raw mut (*entry).members,
                        name_start_0,
                        name_end.offset_from(name_start_0) as ::core::ffi::c_long
                            as size_t,
                    );
                }
            }
        }
        if (*entry).members.max_items > 0 as ::core::ffi::c_int
            && (*entry).members.count >= (*entry).members.max_items
        {
            break;
        }
    }
    pclose(fp);
    (*entry).loaded = 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn add_java_member_matches(
    mut class_name: *const ::core::ffi::c_char,
    mut prefix: *const ::core::ffi::c_char,
) {
    let mut entry: *mut java_member_entry_t = ::core::ptr::null_mut::<
        java_member_entry_t,
    >();
    if class_name.is_null() || prefix.is_null()
        || *prefix as ::core::ffi::c_int == '\0' as i32
    {
        return;
    }
    entry = get_java_member_entry(class_name);
    if entry.is_null() {
        return;
    }
    if (*entry).loaded == 0 {
        load_javap_members(entry);
    }
    if (*entry).members.count == 0 as ::core::ffi::c_int {
        return;
    }
    add_matches_from_pool(&raw mut (*entry).members, prefix);
}
unsafe extern "C" fn expand_tilde(
    mut input: *const ::core::ffi::c_char,
    mut output: *mut ::core::ffi::c_char,
    mut outsz: size_t,
) {
    if !input.is_null()
        && *input.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '~' as i32
    {
        let mut home: *const ::core::ffi::c_char = getenv(
            b"HOME\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !home.is_null() && *home as ::core::ffi::c_int != 0 {
            snprintf(
                output,
                outsz,
                b"%s%s\0" as *const u8 as *const ::core::ffi::c_char,
                home,
                input.offset(1 as ::core::ffi::c_int as isize),
            );
            return;
        }
    }
    mystrscpy(
        output,
        if !input.is_null() {
            input
        } else {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        },
        outsz as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn file_exists(
    mut path: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if path.is_null() || *path as ::core::ffi::c_int == '\0' as i32 {
        return FALSE;
    }
    return (stat(path, &raw mut st) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn normalize_path(mut path: *mut ::core::ffi::c_char) {
    let mut len: size_t = 0;
    if path.is_null() {
        return;
    }
    len = strlen(path);
    while len > 1 as size_t
        && (*path.offset(len.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
            == '/' as i32
            || *path.offset(len.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
                == '\\' as i32)
    {
        *path.offset(len.wrapping_sub(1 as size_t) as isize) = '\0' as i32
            as ::core::ffi::c_char;
        len = len.wrapping_sub(1);
    }
}
unsafe extern "C" fn is_path_delim(mut ch: ::core::ffi::c_char) -> ::core::ffi::c_int {
    return (ch as ::core::ffi::c_int == ':' as i32
        || ch as ::core::ffi::c_int == ';' as i32) as ::core::ffi::c_int;
}
unsafe extern "C" fn add_path_entry(
    mut paths: *mut completion_pool_t,
    mut entry: *const ::core::ffi::c_char,
) {
    let mut expanded: [::core::ffi::c_char; 2048] = [0; 2048];
    if paths.is_null() {
        return;
    }
    if entry.is_null() || *entry as ::core::ffi::c_int == '\0' as i32 {
        entry = b".\0" as *const u8 as *const ::core::ffi::c_char;
    }
    expand_tilde(
        entry,
        &raw mut expanded as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 2048]>() as size_t,
    );
    normalize_path(&raw mut expanded as *mut ::core::ffi::c_char);
    if expanded[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '\0' as i32 {
        return;
    }
    pool_add(paths, &raw mut expanded as *mut ::core::ffi::c_char);
}
unsafe extern "C" fn parse_path_list(
    mut value: *const ::core::ffi::c_char,
    mut paths: *mut completion_pool_t,
) {
    let mut dup: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut segment: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut iter: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    if value.is_null() || *value as ::core::ffi::c_int == '\0' as i32 || paths.is_null()
    {
        return;
    }
    dup = strdup(value);
    if dup.is_null() {
        return;
    }
    segment = dup;
    while *segment != 0 {
        iter = segment;
        while *iter as ::core::ffi::c_int != 0 && is_path_delim(*iter) == 0 {
            iter = iter.offset(1);
        }
        if *iter != 0 {
            *iter = '\0' as i32 as ::core::ffi::c_char;
            add_path_entry(paths, segment);
            segment = iter.offset(1 as ::core::ffi::c_int as isize);
            if *segment as ::core::ffi::c_int == '\0' as i32 {
                add_path_entry(paths, b".\0" as *const u8 as *const ::core::ffi::c_char);
            }
        } else {
            add_path_entry(paths, segment);
            break;
        }
    }
    free(dup as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn has_extension(
    mut name: *const ::core::ffi::c_char,
    mut exts: *mut *const ::core::ffi::c_char,
    mut count: size_t,
) -> ::core::ffi::c_int {
    let mut dot: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if name.is_null() {
        return FALSE;
    }
    dot = strrchr(name, '.' as i32);
    if dot.is_null()
        || *dot.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '\0' as i32
    {
        return FALSE;
    }
    let mut i: size_t = 0 as size_t;
    while i < count {
        if strcasecmp(dot, *exts.offset(i as isize)) == 0 as ::core::ffi::c_int {
            return TRUE;
        }
        i = i.wrapping_add(1);
    }
    return FALSE;
}
unsafe extern "C" fn is_dir_path(
    mut path: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if stat(path, &raw mut st) != 0 as ::core::ffi::c_int {
        return FALSE;
    }
    return (st.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn collect_path_completions(mut prefix: *const ::core::ffi::c_char) {
    let mut expanded: [::core::ffi::c_char; 2048] = [0; 2048];
    expand_tilde(
        prefix,
        &raw mut expanded as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 2048]>() as size_t,
    );
    let mut orig_slash: *const ::core::ffi::c_char = strrchr(prefix, '/' as i32);
    let mut orig_dir_len: size_t = if !orig_slash.is_null() {
        (orig_slash.offset_from(prefix) as ::core::ffi::c_long
            + 1 as ::core::ffi::c_long) as size_t
    } else {
        0 as size_t
    };
    let mut orig_dir: [::core::ffi::c_char; 2048] = [0; 2048];
    if orig_dir_len > 0 as size_t {
        let mut copy_len: size_t = if orig_dir_len
            < (::core::mem::size_of::<[::core::ffi::c_char; 2048]>() as usize)
                .wrapping_sub(1 as usize)
        {
            orig_dir_len
        } else {
            (::core::mem::size_of::<[::core::ffi::c_char; 2048]>() as size_t)
                .wrapping_sub(1 as size_t)
        };
        memcpy(
            &raw mut orig_dir as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            prefix as *const ::core::ffi::c_void,
            copy_len,
        );
        orig_dir[copy_len as usize] = '\0' as i32 as ::core::ffi::c_char;
    } else {
        orig_dir[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
    }
    let mut slash: *const ::core::ffi::c_char = strrchr(
        &raw mut expanded as *mut ::core::ffi::c_char,
        '/' as i32,
    );
    let mut base: *const ::core::ffi::c_char = if !slash.is_null() {
        slash.offset(1 as ::core::ffi::c_int as isize)
    } else {
        &raw mut expanded as *mut ::core::ffi::c_char as *const ::core::ffi::c_char
    };
    let mut dir_len: size_t = if !slash.is_null() {
        (slash.offset_from(&raw mut expanded as *mut ::core::ffi::c_char)
            as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as size_t
    } else {
        0 as size_t
    };
    let mut dir: [::core::ffi::c_char; 2048] = [0; 2048];
    if dir_len > 0 as size_t {
        let mut copy_len_0: size_t = if dir_len
            < (::core::mem::size_of::<[::core::ffi::c_char; 2048]>() as usize)
                .wrapping_sub(1 as usize)
        {
            dir_len
        } else {
            (::core::mem::size_of::<[::core::ffi::c_char; 2048]>() as size_t)
                .wrapping_sub(1 as size_t)
        };
        memcpy(
            &raw mut dir as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            &raw mut expanded as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            copy_len_0,
        );
        dir[copy_len_0 as usize] = '\0' as i32 as ::core::ffi::c_char;
    } else {
        strcpy(
            &raw mut dir as *mut ::core::ffi::c_char,
            b".\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    let mut dp: *mut DIR = opendir(&raw mut dir as *mut ::core::ffi::c_char);
    if dp.is_null() {
        return;
    }
    let mut base_len: size_t = strlen(base);
    let mut dent: *mut dirent = ::core::ptr::null_mut::<dirent>();
    loop {
        dent = readdir(dp);
        if !(!dent.is_null() && completion_state.count < MAX_COMPLETIONS) {
            break;
        }
        if (*dent).d_name[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            == '.' as i32
            && *base.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != '.' as i32
        {
            continue;
        }
        if strncmp(&raw mut (*dent).d_name as *mut ::core::ffi::c_char, base, base_len)
            != 0 as ::core::ffi::c_int
        {
            continue;
        }
        let mut full_path: [::core::ffi::c_char; 2048] = [0; 2048];
        if dir_len > 0 as size_t
            && strcmp(
                &raw mut dir as *mut ::core::ffi::c_char,
                b".\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
        {
            mystrscpy(
                &raw mut full_path as *mut ::core::ffi::c_char,
                &raw mut dir as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 2048]>()
                    as ::core::ffi::c_int,
            );
            let mut used: size_t = strlen(
                &raw mut full_path as *mut ::core::ffi::c_char,
            );
            if used
                < (::core::mem::size_of::<[::core::ffi::c_char; 2048]>() as usize)
                    .wrapping_sub(1 as usize)
            {
                mystrscpy(
                    (&raw mut full_path as *mut ::core::ffi::c_char)
                        .offset(used as isize),
                    &raw mut (*dent).d_name as *mut ::core::ffi::c_char,
                    (::core::mem::size_of::<[::core::ffi::c_char; 2048]>() as usize)
                        .wrapping_sub(used as usize) as ::core::ffi::c_int,
                );
            }
        } else {
            mystrscpy(
                &raw mut full_path as *mut ::core::ffi::c_char,
                &raw mut (*dent).d_name as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 2048]>()
                    as ::core::ffi::c_int,
            );
        }
        let mut is_dir: ::core::ffi::c_int = is_dir_path(
            &raw mut full_path as *mut ::core::ffi::c_char,
        );
        let mut display: [::core::ffi::c_char; 128] = [0; 128];
        if orig_dir_len > 0 as size_t {
            let mut copy: size_t = orig_dir_len;
            if copy
                > (::core::mem::size_of::<[::core::ffi::c_char; 128]>() as usize)
                    .wrapping_sub(1 as usize)
            {
                copy = (::core::mem::size_of::<[::core::ffi::c_char; 128]>() as usize)
                    .wrapping_sub(1 as usize) as size_t;
            }
            memcpy(
                &raw mut display as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                prefix as *const ::core::ffi::c_void,
                copy,
            );
            display[copy as usize] = '\0' as i32 as ::core::ffi::c_char;
            if copy
                < (::core::mem::size_of::<[::core::ffi::c_char; 128]>() as usize)
                    .wrapping_sub(1 as usize)
            {
                mystrscpy(
                    (&raw mut display as *mut ::core::ffi::c_char).offset(copy as isize),
                    &raw mut (*dent).d_name as *mut ::core::ffi::c_char,
                    (::core::mem::size_of::<[::core::ffi::c_char; 128]>() as usize)
                        .wrapping_sub(copy as usize) as ::core::ffi::c_int,
                );
            }
        } else {
            mystrscpy(
                &raw mut display as *mut ::core::ffi::c_char,
                &raw mut (*dent).d_name as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>()
                    as ::core::ffi::c_int,
            );
        }
        let mut used_0: size_t = strlen(&raw mut display as *mut ::core::ffi::c_char);
        if is_dir != 0
            && used_0
                < (::core::mem::size_of::<[::core::ffi::c_char; 128]>() as usize)
                    .wrapping_sub(1 as usize)
        {
            let fresh2 = used_0;
            used_0 = used_0.wrapping_add(1);
            display[fresh2 as usize] = '/' as i32 as ::core::ffi::c_char;
            display[used_0 as usize] = '\0' as i32 as ::core::ffi::c_char;
        }
        completion_consider_candidate(
            &raw mut display as *mut ::core::ffi::c_char,
            prefix,
        );
    }
    closedir(dp);
}
#[no_mangle]
pub unsafe extern "C" fn completion_init() {
    completion_reset_state();
}
#[no_mangle]
pub unsafe extern "C" fn completion_update(
    mut prefix: *const ::core::ffi::c_char,
    mut ctx: completion_context_t,
) {
    if prefix.is_null() || *prefix as ::core::ffi::c_int == '\0' as i32 {
        completion_reset_state();
        return;
    }
    completion_reset_state();
    if ctx as ::core::ffi::c_uint
        == COMPLETION_CONTEXT_PATH as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        collect_path_completions(prefix);
    } else {
        collect_buffer_words(prefix);
        collect_language_keywords(prefix);
        collect_common_keywords(prefix);
    }
    completion_state.is_visible = (completion_state.count > 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn completion_visible_rows() -> ::core::ffi::c_int {
    let mut limit: ::core::ffi::c_int = if completion_dropdown_state.active != 0 {
        COMPLETION_POPUP_MAX_VISIBLE
    } else {
        COMPLETION_MINIBUFFER_MAX_VISIBLE
    };
    if completion_state.count < limit {
        return completion_state.count;
    }
    return limit;
}
unsafe extern "C" fn completion_ensure_visible() {
    let mut height: ::core::ffi::c_int = completion_visible_rows();
    if height <= 0 as ::core::ffi::c_int {
        completion_state.scroll_offset = 0 as ::core::ffi::c_int;
        return;
    }
    if completion_state.scroll_offset < 0 as ::core::ffi::c_int {
        completion_state.scroll_offset = 0 as ::core::ffi::c_int;
    }
    let mut max_offset: ::core::ffi::c_int = completion_state.count - height;
    if max_offset < 0 as ::core::ffi::c_int {
        max_offset = 0 as ::core::ffi::c_int;
    }
    if completion_state.scroll_offset > max_offset {
        completion_state.scroll_offset = max_offset;
    }
    if completion_state.selected_index < completion_state.scroll_offset {
        completion_state.scroll_offset = completion_state.selected_index;
    }
    if completion_state.selected_index >= completion_state.scroll_offset + height {
        completion_state.scroll_offset = completion_state.selected_index - height
            + 1 as ::core::ffi::c_int;
    }
    if completion_state.scroll_offset < 0 as ::core::ffi::c_int {
        completion_state.scroll_offset = 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn completion_draw_minibuffer_list(
    mut row: ::core::ffi::c_int,
    mut col: ::core::ffi::c_int,
) {
    if completion_state.is_visible == 0 {
        return;
    }
    completion_ensure_visible();
    let mut saved_row: ::core::ffi::c_int = ttrow;
    let mut saved_col: ::core::ffi::c_int = ttcol;
    let mut height: ::core::ffi::c_int = completion_visible_rows();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < height {
        let mut idx: ::core::ffi::c_int = completion_state.scroll_offset + i;
        if idx >= completion_state.count {
            break;
        }
        let mut r: ::core::ffi::c_int = row - height + i;
        if !(r < 0 as ::core::ffi::c_int) {
            movecursor(r, col);
            if idx == completion_state.selected_index {
                vttrev(TRUE);
            }
            let mut buf: [::core::ffi::c_char; 128] = [0; 128];
            snprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
                b" %-20s \0" as *const u8 as *const ::core::ffi::c_char,
                completion_state.matches[idx as usize],
            );
            let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while buf[j as usize] != 0 {
                vttputc(buf[j as usize] as ::core::ffi::c_int);
                j += 1;
            }
            vtteeol();
            if idx == completion_state.selected_index {
                vttrev(FALSE);
            }
        }
        i += 1;
    }
    if saved_row < HUGE && saved_col < HUGE {
        movecursor(saved_row, saved_col);
    }
    vttflush();
}
#[no_mangle]
pub unsafe extern "C" fn completion_draw(
    mut row: ::core::ffi::c_int,
    mut col: ::core::ffi::c_int,
) {
    if completion_state.is_visible == 0 {
        return;
    }
    if completion_dropdown_state.active != 0 {
        completion_draw_popup_box();
        return;
    }
    completion_draw_minibuffer_list(row, col);
}
#[no_mangle]
pub unsafe extern "C" fn completion_get_selected() -> *const ::core::ffi::c_char {
    if completion_state.is_visible != 0
        && completion_state.selected_index < completion_state.count
    {
        return completion_state.matches[completion_state.selected_index as usize];
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn completion_next() {
    if completion_state.count > 0 as ::core::ffi::c_int {
        completion_state.selected_index = (completion_state.selected_index
            + 1 as ::core::ffi::c_int) % completion_state.count;
        completion_ensure_visible();
    }
}
#[no_mangle]
pub unsafe extern "C" fn completion_prev() {
    if completion_state.count > 0 as ::core::ffi::c_int {
        completion_state.selected_index = (completion_state.selected_index
            - 1 as ::core::ffi::c_int + completion_state.count) % completion_state.count;
        completion_ensure_visible();
    }
}
#[no_mangle]
pub unsafe extern "C" fn completion_hide() {
    completion_state.is_visible = 0 as ::core::ffi::c_int;
    completion_state.scroll_offset = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn prev_char_start(
    mut lp: *mut line,
    mut pos: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if lp.is_null() || pos <= 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    loop {
        pos -= 1;
        if !(pos > 0 as ::core::ffi::c_int
            && is_beginning_utf8(
                *(&raw mut (*lp).l_text as *mut ::core::ffi::c_uchar)
                    .offset(pos as isize),
            ) == 0)
        {
            break;
        }
    }
    return pos;
}
unsafe extern "C" fn extract_word_prefix(
    mut lp: *mut line,
    mut offset: ::core::ffi::c_int,
    mut dest: *mut ::core::ffi::c_char,
    mut dest_sz: size_t,
    mut start_out: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if lp.is_null() || dest.is_null() || dest_sz == 0 as size_t {
        return FALSE;
    }
    let mut len: ::core::ffi::c_int = (*lp).l_used;
    if offset > len {
        offset = len;
    }
    let mut start: ::core::ffi::c_int = offset;
    while start > 0 as ::core::ffi::c_int {
        let mut candidate: ::core::ffi::c_int = prev_char_start(lp, start);
        let mut uc: unicode_t = 0;
        let mut bytes: ::core::ffi::c_int = utf8_to_unicode(
            &raw mut (*lp).l_text as *mut ::core::ffi::c_uchar,
            candidate as ::core::ffi::c_uint,
            len as ::core::ffi::c_uint,
            &raw mut uc,
        ) as ::core::ffi::c_int;
        if bytes <= 0 as ::core::ffi::c_int {
            bytes = 1 as ::core::ffi::c_int;
        }
        if is_identifier_char(uc) == 0 {
            break;
        }
        start = candidate;
    }
    if start == offset {
        return FALSE;
    }
    let mut copy_len: ::core::ffi::c_int = offset - start;
    if copy_len >= dest_sz as ::core::ffi::c_int {
        copy_len = dest_sz as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    }
    memcpy(
        dest as *mut ::core::ffi::c_void,
        (&raw mut (*lp).l_text as *mut ::core::ffi::c_uchar).offset(start as isize)
            as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
        copy_len as size_t,
    );
    *dest.offset(copy_len as isize) = '\0' as i32 as ::core::ffi::c_char;
    if !start_out.is_null() {
        *start_out = start;
    }
    return TRUE;
}
unsafe extern "C" fn extract_path_prefix(
    mut lp: *mut line,
    mut offset: ::core::ffi::c_int,
    mut dest: *mut ::core::ffi::c_char,
    mut dest_sz: size_t,
    mut start_out: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if lp.is_null() || dest.is_null() || dest_sz == 0 as size_t {
        return FALSE;
    }
    let mut len: ::core::ffi::c_int = (*lp).l_used;
    if offset > len {
        offset = len;
    }
    let mut start: ::core::ffi::c_int = offset;
    while start > 0 as ::core::ffi::c_int {
        let mut candidate: ::core::ffi::c_int = prev_char_start(lp, start);
        let mut uc: unicode_t = 0;
        let mut bytes: ::core::ffi::c_int = utf8_to_unicode(
            &raw mut (*lp).l_text as *mut ::core::ffi::c_uchar,
            candidate as ::core::ffi::c_uint,
            len as ::core::ffi::c_uint,
            &raw mut uc,
        ) as ::core::ffi::c_int;
        if bytes <= 0 as ::core::ffi::c_int {
            bytes = 1 as ::core::ffi::c_int;
        }
        if is_path_char(uc) == 0 {
            break;
        }
        start = candidate;
    }
    if start == offset {
        return FALSE;
    }
    let mut copy_len: ::core::ffi::c_int = offset - start;
    if copy_len >= dest_sz as ::core::ffi::c_int {
        copy_len = dest_sz as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    }
    memcpy(
        dest as *mut ::core::ffi::c_void,
        (&raw mut (*lp).l_text as *mut ::core::ffi::c_uchar).offset(start as isize)
            as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
        copy_len as size_t,
    );
    *dest.offset(copy_len as isize) = '\0' as i32 as ::core::ffi::c_char;
    if !start_out.is_null() {
        *start_out = start;
    }
    if !strchr(dest, '/' as i32).is_null()
        || *dest.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '/' as i32
        || *dest.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '~' as i32
        || *dest.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '.' as i32
            && *dest.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != '\0' as i32
    {
        return TRUE;
    }
    return FALSE;
}
unsafe extern "C" fn determine_completion_prefix(
    mut out: *mut ::core::ffi::c_char,
    mut out_sz: size_t,
    mut ctx: *mut completion_context_t,
    mut line_out: *mut *mut line,
    mut start_out: *mut ::core::ffi::c_int,
    mut end_out: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if curwp.is_null() || out.is_null() || out_sz == 0 as size_t {
        return FALSE;
    }
    let mut lp: *mut line = (*curwp).w_dotp;
    if lp.is_null() || lp == (*curbp).b_linep {
        return FALSE;
    }
    let mut offset: ::core::ffi::c_int = (*curwp).w_doto;
    if offset > (*lp).l_used {
        offset = (*lp).l_used;
    }
    if extract_path_prefix(lp, offset, out, out_sz, start_out) != 0 {
        if !ctx.is_null() {
            *ctx = COMPLETION_CONTEXT_PATH;
        }
        if !line_out.is_null() {
            *line_out = lp;
        }
        if !end_out.is_null() {
            *end_out = offset;
        }
        return TRUE;
    }
    if extract_word_prefix(lp, offset, out, out_sz, start_out) != 0 {
        if !ctx.is_null() {
            *ctx = COMPLETION_CONTEXT_DEFAULT;
        }
        if !line_out.is_null() {
            *line_out = lp;
        }
        if !end_out.is_null() {
            *end_out = offset;
        }
        return TRUE;
    }
    return FALSE;
}
unsafe extern "C" fn completion_insert_text(mut text: *const ::core::ffi::c_char) {
    if text.is_null() || *text as ::core::ffi::c_int == '\0' as i32 {
        return;
    }
    linsert_block(text as *mut ::core::ffi::c_char, strlen(text) as ::core::ffi::c_int);
}
unsafe extern "C" fn completion_dropdown_activate(mut prefix_len: size_t) {
    completion_dropdown_state.active = 1 as ::core::ffi::c_int;
    completion_dropdown_state.prefix_len = prefix_len;
    completion_state.selected_index = 0 as ::core::ffi::c_int;
    completion_state.scroll_offset = 0 as ::core::ffi::c_int;
    completion_state.is_visible = (completion_state.count > 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    completion_dropdown_refresh_geometry();
    completion_ensure_visible();
}
unsafe extern "C" fn completion_dropdown_deactivate(
    mut commit_preview: ::core::ffi::c_int,
) {
    completion_dropdown_state.active = 0 as ::core::ffi::c_int;
    if commit_preview != 0 {
        completion_preview_commit();
    } else {
        completion_preview_abort();
    }
    completion_hide();
}
unsafe extern "C" fn completion_dropdown_apply_selection() {
    if completion_preview_state.active == 0 {
        let mut match_0: *const ::core::ffi::c_char = completion_get_selected();
        if !match_0.is_null() {
            let mut match_len: size_t = strlen(match_0);
            if completion_dropdown_state.prefix_len <= match_len {
                let mut tail: *const ::core::ffi::c_char = match_0
                    .offset(completion_dropdown_state.prefix_len as isize);
                if *tail != 0 {
                    completion_insert_text(tail);
                } else {
                    vttbeep();
                }
            }
        }
    }
    completion_dropdown_deactivate(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn completion_dropdown_refresh_geometry() {
    if completion_dropdown_state.active == 0 {
        return;
    }
    completion_ensure_visible();
    let mut visible: ::core::ffi::c_int = completion_visible_rows();
    if visible <= 0 as ::core::ffi::c_int {
        visible = 1 as ::core::ffi::c_int;
    }
    completion_dropdown_state.popup_height = visible;
    let mut content_width: ::core::ffi::c_int = COMPLETION_POPUP_MIN_CONTENT;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < completion_state.count {
        let mut w: ::core::ffi::c_int = completion_display_width(
            completion_state.matches[i as usize],
        );
        if w > content_width {
            content_width = w;
        }
        if content_width >= COMPLETION_POPUP_MAX_CONTENT {
            break;
        }
        i += 1;
    }
    if content_width > COMPLETION_POPUP_MAX_CONTENT {
        content_width = COMPLETION_POPUP_MAX_CONTENT;
    }
    let mut inner_width: ::core::ffi::c_int = content_width + 2 as ::core::ffi::c_int;
    let mut popup_width: ::core::ffi::c_int = inner_width + 2 as ::core::ffi::c_int;
    let mut safe_cols: ::core::ffi::c_int = (*term).t_ncol as ::core::ffi::c_int;
    if popup_width > safe_cols {
        popup_width = safe_cols;
        if popup_width < 4 as ::core::ffi::c_int {
            popup_width = 4 as ::core::ffi::c_int;
        }
        inner_width = popup_width - 2 as ::core::ffi::c_int;
    }
    completion_dropdown_state.popup_width = popup_width;
    let mut total_height: ::core::ffi::c_int = completion_dropdown_state.popup_height
        + 2 as ::core::ffi::c_int;
    let mut safe_bottom: ::core::ffi::c_int = (*term).t_nrow as ::core::ffi::c_int
        - 1 as ::core::ffi::c_int;
    if safe_bottom < 0 as ::core::ffi::c_int {
        safe_bottom = 0 as ::core::ffi::c_int;
    }
    let mut desired_row: ::core::ffi::c_int = currow + 1 as ::core::ffi::c_int;
    if desired_row + total_height - 1 as ::core::ffi::c_int > safe_bottom {
        desired_row = currow - total_height;
    }
    if desired_row < 0 as ::core::ffi::c_int {
        desired_row = 0 as ::core::ffi::c_int;
    }
    if desired_row + total_height - 1 as ::core::ffi::c_int > safe_bottom {
        desired_row = safe_bottom - total_height + 1 as ::core::ffi::c_int;
    }
    if desired_row < 0 as ::core::ffi::c_int {
        desired_row = 0 as ::core::ffi::c_int;
    }
    completion_dropdown_state.popup_row = desired_row;
    let mut desired_col: ::core::ffi::c_int = curcol;
    if desired_col + popup_width >= safe_cols {
        desired_col = safe_cols - popup_width;
    }
    if desired_col < 0 as ::core::ffi::c_int {
        desired_col = 0 as ::core::ffi::c_int;
    }
    completion_dropdown_state.popup_col = desired_col;
}
unsafe extern "C" fn completion_draw_popup_box() {
    if completion_dropdown_state.active == 0 || completion_state.is_visible == 0 {
        return;
    }
    completion_dropdown_refresh_geometry();
    completion_ensure_visible();
    let mut box_row: ::core::ffi::c_int = completion_dropdown_state.popup_row;
    let mut box_col: ::core::ffi::c_int = completion_dropdown_state.popup_col;
    let mut popup_width: ::core::ffi::c_int = completion_dropdown_state.popup_width;
    let mut visible: ::core::ffi::c_int = completion_dropdown_state.popup_height;
    if popup_width < 4 as ::core::ffi::c_int {
        popup_width = 4 as ::core::ffi::c_int;
    }
    if visible <= 0 as ::core::ffi::c_int {
        return;
    }
    let mut saved_row: ::core::ffi::c_int = ttrow;
    let mut saved_col: ::core::ffi::c_int = ttcol;
    let mut inner_width: ::core::ffi::c_int = popup_width - 2 as ::core::ffi::c_int;
    if inner_width < 0 as ::core::ffi::c_int {
        inner_width = 0 as ::core::ffi::c_int;
    }
    let mut text_width: ::core::ffi::c_int = inner_width - 2 as ::core::ffi::c_int;
    if text_width < 0 as ::core::ffi::c_int {
        text_width = 0 as ::core::ffi::c_int;
    }
    let mut normal: HighlightStyle = colorscheme_get(HL_NORMAL);
    let mut selection: HighlightStyle = colorscheme_get(HL_SELECTION);
    let mut notice: HighlightStyle = colorscheme_get(HL_NOTICE);
    let mut border_style: HighlightStyle = completion_combine_style(normal, notice);
    let mut row_style: HighlightStyle = completion_combine_style(normal, selection);
    let mut selected_style: HighlightStyle = completion_combine_style(selection, notice);
    let mut total_height: ::core::ffi::c_int = visible + 2 as ::core::ffi::c_int;
    movecursor(box_row, box_col);
    completion_apply_style(&raw mut border_style);
    vttputc('+' as i32);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < popup_width - 2 as ::core::ffi::c_int {
        vttputc('-' as i32);
        i += 1;
    }
    vttputc('+' as i32);
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < visible {
        let mut idx: ::core::ffi::c_int = completion_state.scroll_offset + i_0;
        if idx >= completion_state.count {
            break;
        }
        let mut line_row: ::core::ffi::c_int = box_row + 1 as ::core::ffi::c_int + i_0;
        let mut text: *const ::core::ffi::c_char = completion_state
            .matches[idx as usize];
        let mut active_style: *mut HighlightStyle = if idx
            == completion_state.selected_index
        {
            &raw mut selected_style
        } else {
            &raw mut row_style
        };
        movecursor(line_row, box_col);
        completion_apply_style(&raw mut border_style);
        vttputc('|' as i32);
        completion_apply_style(active_style);
        vttputc(' ' as i32);
        completion_write_utf8_clipped(
            if !text.is_null() {
                text
            } else {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            },
            text_width,
        );
        vttputc(' ' as i32);
        completion_apply_style(&raw mut border_style);
        vttputc('|' as i32);
        i_0 += 1;
    }
    movecursor(box_row + total_height - 1 as ::core::ffi::c_int, box_col);
    completion_apply_style(&raw mut border_style);
    vttputc('+' as i32);
    let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_1 < popup_width - 2 as ::core::ffi::c_int {
        vttputc('-' as i32);
        i_1 += 1;
    }
    vttputc('+' as i32);
    vttsetcolors(-(1 as ::core::ffi::c_int), -(1 as ::core::ffi::c_int));
    vttsetattrs(
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if saved_row < HUGE && saved_col < HUGE {
        movecursor(saved_row, saved_col);
    }
    vttflush();
}
#[no_mangle]
pub unsafe extern "C" fn completion_try_at_cursor() -> ::core::ffi::c_int {
    let mut prefix: [::core::ffi::c_char; 128] = [0; 128];
    let mut ctx: completion_context_t = COMPLETION_CONTEXT_DEFAULT;
    let mut line: *mut line = ::core::ptr::null_mut::<line>();
    let mut prefix_start: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    completion_preview_abort();
    if determine_completion_prefix(
        &raw mut prefix as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
        &raw mut ctx,
        &raw mut line,
        &raw mut prefix_start,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    ) == 0
    {
        return FALSE;
    }
    completion_update(&raw mut prefix as *mut ::core::ffi::c_char, ctx);
    add_language_specific_matches(&raw mut prefix as *mut ::core::ffi::c_char, ctx);
    collect_source_symbol_matches(&raw mut prefix as *mut ::core::ffi::c_char, ctx);
    if !line.is_null()
        && ctx as ::core::ffi::c_uint
            == COMPLETION_CONTEXT_DEFAULT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut owner: [::core::ffi::c_char; 128] = [0; 128];
        if get_owner_symbol_near_cursor(
            line,
            prefix_start,
            &raw mut owner as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
        ) != 0
        {
            if is_java_file(
                if !curbp.is_null() {
                    &raw mut (*curbp).b_fname as *mut ::core::ffi::c_char
                } else {
                    ::core::ptr::null_mut::<::core::ffi::c_char>()
                },
            ) != 0
            {
                let mut resolved: [::core::ffi::c_char; 256] = [0; 256];
                if resolve_java_class_name(
                    &raw mut owner as *mut ::core::ffi::c_char,
                    &raw mut resolved as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
                ) != 0
                {
                    add_java_member_matches(
                        &raw mut resolved as *mut ::core::ffi::c_char,
                        &raw mut prefix as *mut ::core::ffi::c_char,
                    );
                }
            } else if is_python_file(
                if !curbp.is_null() {
                    &raw mut (*curbp).b_fname as *mut ::core::ffi::c_char
                } else {
                    ::core::ptr::null_mut::<::core::ffi::c_char>()
                },
            ) != 0
            {
                add_runtime_module_matches(
                    SCRAPER_LANG_PYTHON,
                    &raw mut owner as *mut ::core::ffi::c_char,
                    &raw mut prefix as *mut ::core::ffi::c_char,
                );
            } else if is_node_file(
                if !curbp.is_null() {
                    &raw mut (*curbp).b_fname as *mut ::core::ffi::c_char
                } else {
                    ::core::ptr::null_mut::<::core::ffi::c_char>()
                },
            ) != 0
            {
                add_runtime_module_matches(
                    SCRAPER_LANG_NODE,
                    &raw mut owner as *mut ::core::ffi::c_char,
                    &raw mut prefix as *mut ::core::ffi::c_char,
                );
            }
        }
    }
    if completion_state.count == 0 as ::core::ffi::c_int {
        return FALSE;
    }
    let mut prefix_len: size_t = strlen(&raw mut prefix as *mut ::core::ffi::c_char);
    if completion_state.count == 1 as ::core::ffi::c_int {
        let mut match_0: *const ::core::ffi::c_char = completion_state
            .matches[0 as ::core::ffi::c_int as usize];
        let mut tail: *const ::core::ffi::c_char = match_0.offset(prefix_len as isize);
        if *tail != 0 {
            completion_insert_text(tail);
        } else {
            vttbeep();
        }
        return TRUE;
    }
    completion_dropdown_activate(prefix_len);
    if !line.is_null() {
        completion_preview_begin(line, prefix_start, prefix_len as ::core::ffi::c_int);
        completion_preview_apply_selected();
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn completion_dropdown_is_active() -> ::core::ffi::c_int {
    return completion_dropdown_state.active;
}
#[no_mangle]
pub unsafe extern "C" fn completion_dropdown_handle_key(
    mut key: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if completion_dropdown_state.active == 0 {
        return 0 as ::core::ffi::c_int;
    }
    match key {
        -2147483583 | 268435536 => {
            completion_prev();
            completion_preview_apply_selected();
            return 1 as ::core::ffi::c_int;
        }
        -2147483582 | 268435534 | 268435520 => {
            completion_next();
            completion_preview_apply_selected();
            return 1 as ::core::ffi::c_int;
        }
        268435533_i32 | 10 | 13 | 268435529 => {
            completion_dropdown_apply_selection();
            return 1 as ::core::ffi::c_int;
        }
        268435547_i32 | 268435527 => {
            completion_dropdown_deactivate(0 as ::core::ffi::c_int);
            return 1 as ::core::ffi::c_int;
        }
        _ => {
            completion_dropdown_deactivate(0 as ::core::ffi::c_int);
            return 0 as ::core::ffi::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn completion_dropdown_render() {
    if completion_dropdown_state.active == 0 {
        return;
    }
    completion_draw(-(1 as ::core::ffi::c_int), -(1 as ::core::ffi::c_int));
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
