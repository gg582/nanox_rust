#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

#![feature(c_variadic)]
#![feature(extern_types)]

mod basic;
mod bind;
#[path = "buffer.rs"]
mod buffer_m;
mod colorscheme;
mod command_mode;
mod completion;
mod cscope;
mod cutln;
mod display;
mod eval;
mod exec;
mod file;
mod fileio;
mod globals;
mod highlight;
mod input;
#[path = "isearch.rs"]
mod isearch_m;
#[path = "line.rs"]
mod line_m;
mod lock;
mod names;
mod nanox;
mod ncurses;
mod paste_slot;
mod pklock;
mod platform;
mod posix;
mod random;
mod region;
mod scraper;
mod search;
mod spawn;
mod tcap;
mod term_wrapper;
#[path = "usage.rs"]
mod usage_m;
mod utf8;
mod version;
#[path = "window.rs"]
mod window_m;
mod word;
mod wrapper;


#[link(name = "ncursesw")]
#[link(name = "tinfo")]
#[link(name = "m")]
extern "C" {}

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type Hunhandle;
    static mut stdout: *mut FILE;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn atoi(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
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
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn Hunspell_create(
        affpath: *const ::core::ffi::c_char,
        dpath: *const ::core::ffi::c_char,
    ) -> *mut Hunhandle;
    fn Hunspell_destroy(pHunspell: *mut Hunhandle);
    fn Hunspell_add_dic(
        pHunspell: *mut Hunhandle,
        dpath: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn Hunspell_spell(
        pHunspell: *mut Hunhandle,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    static mut term: *mut terminal;
    static mut tcap_term: terminal;
    static mut tab_width: ::core::ffi::c_int;
    fn vttclose();
    fn vttkclose();
    fn vttputc(c: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn vttbeep();
    fn nanox_init();
    fn nanox_set_lamp(state: nanox_lamp_state);
    fn nanox_help_is_active() -> bool;
    fn nanox_help_render();
    fn nanox_help_command(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn nanox_help_handle_key(key: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn reserve_jump_1(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn reserve_jump_2(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn reserve_jump_3(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn reserve_jump_4(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn reserve_jump_fallback_1(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn reserve_jump_fallback_2(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn reserve_jump_fallback_3(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn reserve_jump_fallback_4(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn reserve_jump_numeric_mode(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn nanox_queue_startup_file(path: *const ::core::ffi::c_char);
    fn nanox_open_startup_slot() -> ::core::ffi::c_int;
    fn nanox_cleanup();
    fn paste_slot_handle_key(c: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn check_paste_slot_active() -> ::core::ffi::c_int;
    static mut fillcol: ::core::ffi::c_int;
    static mut kbdm: [::core::ffi::c_int; 0];
    static mut patmatch: *mut ::core::ffi::c_char;
    static mut gflags: ::core::ffi::c_int;
    static mut thisflag: ::core::ffi::c_int;
    static mut lastflag: ::core::ffi::c_int;
    static mut curwp: *mut window;
    static mut curbp: *mut buffer;
    static mut bheadp: *mut buffer;
    static mut blistp: *mut buffer;
    static mut kbdptr: *mut ::core::ffi::c_int;
    static mut kbdend: *mut ::core::ffi::c_int;
    static mut kbdmode: ::core::ffi::c_int;
    static mut kbdrep: ::core::ffi::c_int;
    static mut restflag: ::core::ffi::c_int;
    static mut pat: [::core::ffi::c_char; 0];
    static mut discmd: ::core::ffi::c_int;
    static mut nullflag: ::core::ffi::c_int;
    static mut gasave: ::core::ffi::c_int;
    static mut gacount: ::core::ffi::c_int;
    static mut gmode: ::core::ffi::c_int;
    static mut mpresf: ::core::ffi::c_int;
    static mut reptc: ::core::ffi::c_int;
    static mut cutln_active: ::core::ffi::c_int;
    fn backchar(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn forwchar(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn gotoline(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn gotobob(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn gotoeob(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn forwline(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn backline(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn forwpage(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn backpage(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn getccol(bflg: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn insert_tab(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn completion_menu_command(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn insert_newline(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn insbrace(n: ::core::ffi::c_int, c: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn inspound() -> ::core::ffi::c_int;
    fn indent_start_set(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn indent_end_set(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn outdent_start_set(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn outdent_end_set(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn indent_cancel(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn g_prefix_handler(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn forwdel(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn fmatch(ch: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn vtinit();
    fn vttidy();
    fn upscreen(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn update(force: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn mlerase();
    fn mlwrite(fmt: *const ::core::ffi::c_char, ...);
    fn sizesignal(signr: ::core::ffi::c_int);
    fn typahead() -> ::core::ffi::c_int;
    fn mlyesno(prompt: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn getcmd() -> ::core::ffi::c_int;
    fn startup(sfname: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn getbind(c: ::core::ffi::c_int) -> fn_t;
    fn swbuffer(bp: *mut buffer) -> ::core::ffi::c_int;
    fn zotbuf(bp: *mut buffer) -> ::core::ffi::c_int;
    fn anycb() -> ::core::ffi::c_int;
    fn bfreeall();
    fn cleanup_backup(bp: *mut buffer, force: ::core::ffi::c_int);
    fn bfind(
        bname: *mut ::core::ffi::c_char,
        cflag: ::core::ffi::c_int,
        bflag: ::core::ffi::c_int,
    ) -> *mut buffer;
    fn filefind(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn makename(bname: *mut ::core::ffi::c_char, fname: *mut ::core::ffi::c_char);
    fn unqname(name: *mut ::core::ffi::c_char);
    fn cutln_end_cut(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cutln_start_cut(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn cutln_end_copy(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn cutln_start_copy(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn cutln_cut_current_line(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn command_mode_activate_command(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sed_replace_command(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn command_mode_block_is_active() -> ::core::ffi::c_int;
    fn command_mode_block_handle_key(
        c: ::core::ffi::c_int,
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn filesave(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn forwhunt(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn nanox_search_engine(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn varinit();
    fn varcleanup();
    fn lockrel() -> ::core::ffi::c_int;
    fn insspace(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sanitize_and_insert(
        n: ::core::ffi::c_int,
        c: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ldelchar(n: ::core::ffi::c_long, kflag: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn kdelete();
    fn yank(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn version();
    fn completion_dropdown_is_active() -> ::core::ffi::c_int;
    fn completion_dropdown_handle_key(key: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn completion_dropdown_render();
    fn signal(__sig: ::core::ffi::c_int, __handler: __sighandler_t) -> __sighandler_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct key_tab {
    pub k_code: ::core::ffi::c_int,
    pub k_fp: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
}
pub type nanox_lamp_state = ::core::ffi::c_uint;
pub const NANOX_LAMP_ERROR: nanox_lamp_state = 2;
pub const NANOX_LAMP_WARN: nanox_lamp_state = 1;
pub const NANOX_LAMP_OFF: nanox_lamp_state = 0;
pub type fn_t = Option<
    unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int) -> ::core::ffi::c_int,
>;
pub type __sighandler_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const SIGTERM: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const SIGHUP: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SIGWINCH: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const GFREAD: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NPAT: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const CONTROL: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;
pub const META: ::core::ffi::c_int = 0x20000000 as ::core::ffi::c_int;
pub const SHIFT: ::core::ffi::c_int = 0x8000000 as ::core::ffi::c_int;
pub const SPEC: ::core::ffi::c_uint = 0x80000000 as ::core::ffi::c_uint;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const STOP: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PLAY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const RECORD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const WFHARD: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const WFMODE: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const BFINVS: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const BFCHG: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const BFTRUNC: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const MDWRAP: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const MDSOFTWRAP: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MDCMOD: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const MDVIEW: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const MDOVER: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const MDASAVE: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
#[no_mangle]
pub static mut keytab: [key_tab; 2048] = {
    [
        key_tab {
            k_code: (SPEC | 'A' as i32 as ::core::ffi::c_uint) as ::core::ffi::c_int,
            k_fp: Some(
                backline
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: (SPEC | 'B' as i32 as ::core::ffi::c_uint) as ::core::ffi::c_int,
            k_fp: Some(
                forwline
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: (SPEC | 'C' as i32 as ::core::ffi::c_uint) as ::core::ffi::c_int,
            k_fp: Some(
                forwchar
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: (SPEC | 'D' as i32 as ::core::ffi::c_uint) as ::core::ffi::c_int,
            k_fp: Some(
                backchar
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: (SPEC | 'H' as i32 as ::core::ffi::c_uint) as ::core::ffi::c_int,
            k_fp: Some(
                gotobob
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: (SPEC | 'F' as i32 as ::core::ffi::c_uint) as ::core::ffi::c_int,
            k_fp: Some(
                gotoeob
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: (SPEC | '5' as i32 as ::core::ffi::c_uint) as ::core::ffi::c_int,
            k_fp: Some(
                backpage
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: (SPEC | '6' as i32 as ::core::ffi::c_uint) as ::core::ffi::c_int,
            k_fp: Some(
                forwpage
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: (SPEC | 'L' as i32 as ::core::ffi::c_uint) as ::core::ffi::c_int,
            k_fp: Some(
                insspace
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: 0x7f as ::core::ffi::c_int,
            k_fp: Some(
                indent_cancel
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: (SPEC | 0x7f as ::core::ffi::c_uint) as ::core::ffi::c_int,
            k_fp: Some(
                forwdel
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: CONTROL | 'I' as i32,
            k_fp: Some(
                insert_tab
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: CONTROL | 'M' as i32,
            k_fp: Some(
                insert_newline
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: CONTROL | '@' as i32,
            k_fp: Some(
                completion_menu_command
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: CONTROL | 'H' as i32,
            k_fp: Some(
                outdent_start_set
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: CONTROL | SHIFT | 'H' as i32,
            k_fp: Some(
                outdent_end_set
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: CONTROL | 'J' as i32,
            k_fp: Some(
                indent_start_set
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: CONTROL | SHIFT | 'J' as i32,
            k_fp: Some(
                indent_end_set
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: 'g' as i32,
            k_fp: Some(
                g_prefix_handler
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: CONTROL | 'V' as i32,
            k_fp: Some(
                command_mode_activate_command
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: (SPEC | 'P' as i32 as ::core::ffi::c_uint) as ::core::ffi::c_int,
            k_fp: Some(
                nanox_help_command
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: (SPEC | 'Q' as i32 as ::core::ffi::c_uint) as ::core::ffi::c_int,
            k_fp: Some(
                filesave
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: CONTROL | 'S' as i32,
            k_fp: Some(
                filesave
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: (SPEC | 'R' as i32 as ::core::ffi::c_uint) as ::core::ffi::c_int,
            k_fp: Some(
                filefind
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: CONTROL | 'O' as i32,
            k_fp: Some(
                filefind
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: (SPEC | 'S' as i32 as ::core::ffi::c_uint) as ::core::ffi::c_int,
            k_fp: Some(
                quit
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: CONTROL | 'Q' as i32,
            k_fp: Some(
                quit
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: (SPEC | 'U' as i32 as ::core::ffi::c_uint) as ::core::ffi::c_int,
            k_fp: Some(
                nanox_search_engine
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: CONTROL | 'F' as i32,
            k_fp: Some(
                nanox_search_engine
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: CONTROL | 'R' as i32,
            k_fp: Some(
                sed_replace_command
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: (SPEC | 'W' as i32 as ::core::ffi::c_uint) as ::core::ffi::c_int,
            k_fp: Some(
                cutln_start_copy
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: (SPEC | SHIFT as ::core::ffi::c_uint
                | 'W' as i32 as ::core::ffi::c_uint) as ::core::ffi::c_int,
            k_fp: Some(
                cutln_end_copy
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: CONTROL | 'W' as i32,
            k_fp: Some(
                cutln_start_copy
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: CONTROL | SHIFT | 'W' as i32,
            k_fp: Some(
                cutln_end_copy
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: (SPEC | 'X' as i32 as ::core::ffi::c_uint) as ::core::ffi::c_int,
            k_fp: Some(
                cutln_start_cut
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: (SPEC | SHIFT as ::core::ffi::c_uint
                | 'X' as i32 as ::core::ffi::c_uint) as ::core::ffi::c_int,
            k_fp: Some(
                cutln_end_cut
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: CONTROL | 'X' as i32,
            k_fp: Some(
                cutln_start_cut
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: CONTROL | SHIFT | 'X' as i32,
            k_fp: Some(
                cutln_end_cut
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: CONTROL | 'K' as i32,
            k_fp: Some(
                cutln_cut_current_line
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: CONTROL | SHIFT | 'K' as i32,
            k_fp: Some(
                cutln_end_cut
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: CONTROL | 'Y' as i32,
            k_fp: Some(
                yank
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: (SPEC | 'Y' as i32 as ::core::ffi::c_uint) as ::core::ffi::c_int,
            k_fp: Some(
                yank
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: META | CONTROL | '8' as i32,
            k_fp: Some(
                yank
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: (SPEC | '`' as i32 as ::core::ffi::c_uint) as ::core::ffi::c_int,
            k_fp: Some(
                reserve_jump_1
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: (SPEC | 'a' as i32 as ::core::ffi::c_uint) as ::core::ffi::c_int,
            k_fp: Some(
                reserve_jump_2
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: (SPEC | '{' as i32 as ::core::ffi::c_uint) as ::core::ffi::c_int,
            k_fp: Some(
                reserve_jump_3
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: (SPEC | '}' as i32 as ::core::ffi::c_uint) as ::core::ffi::c_int,
            k_fp: Some(
                reserve_jump_4
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: META | CONTROL | '9' as i32,
            k_fp: Some(
                reserve_jump_fallback_1
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: META | CONTROL | '0' as i32,
            k_fp: Some(
                reserve_jump_fallback_2
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: META | CONTROL | '-' as i32,
            k_fp: Some(
                reserve_jump_fallback_3
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: META | CONTROL | '=' as i32,
            k_fp: Some(
                reserve_jump_fallback_4
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: META | CONTROL | '1' as i32,
            k_fp: Some(
                reserve_jump_numeric_mode
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: META | CONTROL | '2' as i32,
            k_fp: Some(
                reserve_jump_numeric_mode
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: META | CONTROL | '3' as i32,
            k_fp: Some(
                reserve_jump_numeric_mode
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: META | CONTROL | '4' as i32,
            k_fp: Some(
                reserve_jump_numeric_mode
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: META | CONTROL | '5' as i32,
            k_fp: Some(
                reserve_jump_numeric_mode
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: META | CONTROL | '6' as i32,
            k_fp: Some(
                reserve_jump_numeric_mode
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: META | CONTROL | '7' as i32,
            k_fp: Some(
                reserve_jump_numeric_mode
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: CONTROL | 'G' as i32,
            k_fp: Some(
                gotoline
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        key_tab {
            k_code: 0 as ::core::ffi::c_int,
            k_fp: None,
        },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
        key_tab { k_code: 0, k_fp: None },
    ]
};
pub const PROGRAM_NAME: [::core::ffi::c_char; 3] = unsafe {
    ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"em\0")
};
#[no_mangle]
pub static mut nanox_help_active_flag: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn nanox_refresh_ui() {
    if nanox_help_is_active() {
        nanox_help_render();
    } else {
        update(FALSE);
        completion_dropdown_render();
    };
}
#[no_mangle]
pub unsafe extern "C" fn usage(mut status: ::core::ffi::c_int) {
    printf(
        b"Usage: %s filename\n\0" as *const u8 as *const ::core::ffi::c_char,
        PROGRAM_NAME.as_ptr(),
    );
    printf(
        b"   or: %s [options]\n\n\0" as *const u8 as *const ::core::ffi::c_char,
        PROGRAM_NAME.as_ptr(),
    );
    fputs(
        b"      +          start at the end of file\n\0" as *const u8
            as *const ::core::ffi::c_char,
        stdout,
    );
    fputs(
        b"      +<n>       start at line <n>\n\0" as *const u8
            as *const ::core::ffi::c_char,
        stdout,
    );
    fputs(
        b"      -g[G]<n>   go to line <n>\n\0" as *const u8
            as *const ::core::ffi::c_char,
        stdout,
    );
    fputs(
        b"      --help     display this help and exit\n\0" as *const u8
            as *const ::core::ffi::c_char,
        stdout,
    );
    fputs(
        b"      --version  output version information and exit\n\0" as *const u8
            as *const ::core::ffi::c_char,
        stdout,
    );
    exit(status);
}
static mut hunhandle: *mut Hunhandle = ::core::ptr::null::<Hunhandle>()
    as *mut Hunhandle;
#[no_mangle]
pub unsafe extern "C" fn spellcheck(
    mut word: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if hunhandle.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    return Hunspell_spell(hunhandle, word);
}
unsafe extern "C" fn local_dictionary(
    mut handle: *mut Hunhandle,
    mut filename: *const ::core::ffi::c_char,
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
    if stat(filename, &raw mut st) == 0
        && st.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t
    {
        Hunspell_add_dic(handle, filename);
    }
}
unsafe extern "C" fn select_terminal_driver() {
    let mut term_env: *mut ::core::ffi::c_char = getenv(
        b"TERM\0" as *const u8 as *const ::core::ffi::c_char,
    );
    let mut selected: *mut terminal = &raw mut tcap_term;
    term = selected as *mut terminal;
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut f: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut mflag: ::core::ffi::c_int = 0;
    let mut bp: *mut buffer = ::core::ptr::null_mut::<buffer>();
    let mut firstfile: ::core::ffi::c_int = 0;
    let mut carg: ::core::ffi::c_int = 0;
    let mut startflag: ::core::ffi::c_int = 0;
    let mut firstbp: *mut buffer = ::core::ptr::null_mut::<buffer>();
    let mut basec: ::core::ffi::c_int = 0;
    let mut viewflag: ::core::ffi::c_int = 0;
    let mut gotoflag: ::core::ffi::c_int = 0;
    let mut gline: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut searchflag: ::core::ffi::c_int = 0;
    let mut saveflag: ::core::ffi::c_int = 0;
    let mut errflag: ::core::ffi::c_int = 0;
    let mut slot_startup_mode: ::core::ffi::c_int = 0;
    let mut file_arg_count: ::core::ffi::c_int = 0;
    let mut bname: [::core::ffi::c_char; 16] = [0; 16];
    let mut newc: ::core::ffi::c_int = 0;
    select_terminal_driver();
    let mut aff_path: *const ::core::ffi::c_char = b"/usr/share/hunspell/en_US.aff\0"
        as *const u8 as *const ::core::ffi::c_char;
    let mut dic_path: *const ::core::ffi::c_char = b"/usr/share/hunspell/en_US.dic\0"
        as *const u8 as *const ::core::ffi::c_char;
    hunhandle = Hunspell_create(aff_path, dic_path);
    if !hunhandle.is_null() {
        local_dictionary(
            hunhandle,
            b".dictionary\0" as *const u8 as *const ::core::ffi::c_char,
        );
        let mut home: *const ::core::ffi::c_char = getenv(
            b"HOME\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !home.is_null() {
            let mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
            snprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
                b"%s/.dictionary\0" as *const u8 as *const ::core::ffi::c_char,
                home,
            );
            local_dictionary(hunhandle, &raw mut buf as *mut ::core::ffi::c_char);
        }
    }
    signal(SIGWINCH, Some(sizesignal as unsafe extern "C" fn(::core::ffi::c_int) -> ()));
    if argc == 2 as ::core::ffi::c_int {
        if strcmp(
            *argv.offset(1 as ::core::ffi::c_int as isize),
            b"--help\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            usage(EXIT_FAILURE);
        }
        if strcmp(
            *argv.offset(1 as ::core::ffi::c_int as isize),
            b"--version\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            version();
            exit(EXIT_SUCCESS);
        }
    }
    vtinit();
    edinit(
        b"main\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    varinit();
    nanox_init();
    viewflag = FALSE;
    gotoflag = FALSE;
    searchflag = FALSE;
    firstfile = TRUE;
    startflag = FALSE;
    errflag = FALSE;
    file_arg_count = 0 as ::core::ffi::c_int;
    carg = 1 as ::core::ffi::c_int;
    while carg < argc {
        if !(*(*argv.offset(carg as isize)).offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int == '+' as i32
            || *(*argv.offset(carg as isize)).offset(0 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int == '-' as i32
            || *(*argv.offset(carg as isize)).offset(0 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int == '@' as i32)
        {
            file_arg_count += 1;
        }
        carg += 1;
    }
    slot_startup_mode = (file_arg_count > 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    carg = 1 as ::core::ffi::c_int;
    while carg < argc {
        if *(*argv.offset(carg as isize)).offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int == '+' as i32
        {
            gotoflag = TRUE;
            gline = atoi(
                (*argv.offset(carg as isize)).offset(1 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char,
            );
        } else if *(*argv.offset(carg as isize)).offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int == '-' as i32
        {
            match *(*argv.offset(carg as isize)).offset(1 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
            {
                97 | 65 => {
                    errflag = TRUE;
                }
                101 | 69 => {
                    viewflag = FALSE;
                }
                103 | 71 => {
                    gotoflag = TRUE;
                    gline = atoi(
                        (*argv.offset(carg as isize))
                            .offset(2 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_char,
                    );
                }
                110 | 78 => {
                    nullflag = TRUE;
                }
                114 | 82 => {
                    restflag = TRUE;
                }
                115 | 83 => {
                    searchflag = TRUE;
                    strncpy(
                        &raw mut pat as *mut ::core::ffi::c_char,
                        (*argv.offset(carg as isize))
                            .offset(2 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_char,
                        NPAT as size_t,
                    );
                }
                118 | 86 => {
                    viewflag = TRUE;
                }
                _ => {}
            }
        } else if *(*argv.offset(carg as isize)).offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int == '@' as i32
        {
            if startup(
                (*argv.offset(carg as isize)).offset(1 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char,
            ) == TRUE
            {
                startflag = TRUE;
            }
        } else if slot_startup_mode != 0 {
            nanox_queue_startup_file(*argv.offset(carg as isize));
            firstfile = FALSE;
        } else {
            makename(
                &raw mut bname as *mut ::core::ffi::c_char,
                *argv.offset(carg as isize),
            );
            unqname(&raw mut bname as *mut ::core::ffi::c_char);
            bp = bfind(
                &raw mut bname as *mut ::core::ffi::c_char,
                TRUE,
                0 as ::core::ffi::c_int,
            );
            strcpy(
                &raw mut (*bp).b_fname as *mut ::core::ffi::c_char,
                *argv.offset(carg as isize),
            );
            (*bp).b_active = FALSE as ::core::ffi::c_char;
            if firstfile != 0 {
                firstbp = bp;
                firstfile = FALSE;
            }
            if viewflag != 0 {
                (*bp).b_mode |= MDVIEW;
            }
            let mut ext: *mut ::core::ffi::c_char = strrchr(
                *argv.offset(carg as isize),
                '.' as i32,
            );
            if !ext.is_null()
                && (strcasecmp(ext, b".c\0" as *const u8 as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcasecmp(
                        ext,
                        b".h\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    || strcasecmp(
                        ext,
                        b".cpp\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    || strcasecmp(
                        ext,
                        b".hpp\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    || strcasecmp(
                        ext,
                        b".java\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    || strcasecmp(
                        ext,
                        b".js\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    || strcasecmp(
                        ext,
                        b".ts\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    || strcasecmp(
                        ext,
                        b".rs\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    || strcasecmp(
                        ext,
                        b".go\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    || strcasecmp(
                        ext,
                        b".php\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    || strcasecmp(
                        ext,
                        b".swift\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int)
            {
                (*bp).b_mode |= MDCMOD;
            }
        }
        carg += 1;
    }
    signal(
        SIGHUP,
        Some(emergencyexit as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
    signal(
        SIGTERM,
        Some(emergencyexit as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
    if errflag != 0 {
        if startup(
            b"error.cmd\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        ) == TRUE
        {
            startflag = TRUE;
        }
    }
    if startflag == FALSE {
        startup(
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        );
        startflag = TRUE;
    }
    discmd = TRUE;
    bp = bfind(
        b"main\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        FALSE,
        0 as ::core::ffi::c_int,
    );
    if slot_startup_mode != 0 && gflags & GFREAD != 0 {
        nanox_open_startup_slot();
    } else if firstfile == FALSE && gflags & GFREAD != 0 {
        swbuffer(firstbp);
        zotbuf(bp);
    } else {
        (*bp).b_mode |= gmode;
    }
    if gotoflag != 0 && searchflag != 0 {
        update(FALSE);
        mlwrite(
            b"(Can not search and goto at the same time!)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    } else if gotoflag != 0 {
        if gotoline(TRUE, gline) == FALSE {
            update(FALSE);
            mlwrite(
                b"(Bogus goto argument)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else if searchflag != 0 {
        if forwhunt(FALSE, 0 as ::core::ffi::c_int) == FALSE {
            update(FALSE);
        }
    }
    lastflag = 0 as ::core::ffi::c_int;
    loop {
        saveflag = lastflag;
        execute(
            (META as ::core::ffi::c_uint | SPEC | 'C' as i32 as ::core::ffi::c_uint)
                as ::core::ffi::c_int,
            FALSE,
            1 as ::core::ffi::c_int,
        );
        lastflag = saveflag;
        if typahead() != 0 {
            loop {
                newc = getcmd();
                if !(newc == 0 as ::core::ffi::c_int) {
                    break;
                }
            }
            nanox_refresh_ui();
            loop {
                let mut execfunc: fn_t = None;
                if !(c == newc
                    && {
                        execfunc = getbind(c) as fn_t;
                        execfunc.is_some()
                    }
                    && execfunc
                        != Some(
                            insert_newline
                                as unsafe extern "C" fn(
                                    ::core::ffi::c_int,
                                    ::core::ffi::c_int,
                                ) -> ::core::ffi::c_int,
                        )
                    && execfunc
                        != Some(
                            insert_tab
                                as unsafe extern "C" fn(
                                    ::core::ffi::c_int,
                                    ::core::ffi::c_int,
                                ) -> ::core::ffi::c_int,
                        ))
                {
                    break;
                }
                loop {
                    newc = getcmd();
                    if !(newc == 0 as ::core::ffi::c_int) {
                        break;
                    }
                }
                if !(typahead() != 0) {
                    break;
                }
            }
            c = newc;
        } else {
            nanox_refresh_ui();
            loop {
                c = getcmd();
                if !(c == 0 as ::core::ffi::c_int) {
                    break;
                }
            }
        }
        if mpresf != FALSE {
            mlerase();
            nanox_refresh_ui();
        }
        if nanox_help_is_active() {
            nanox_help_handle_key(c);
        } else {
            if completion_dropdown_is_active() != 0 {
                if completion_dropdown_handle_key(c) != 0 {
                    continue;
                }
            }
            f = FALSE;
            n = 1 as ::core::ffi::c_int;
            basec = c & !META;
            if c & META != 0
                && (basec >= '0' as i32 && basec <= '9' as i32 || basec == '-' as i32)
            {
                f = TRUE;
                n = 0 as ::core::ffi::c_int;
                mflag = 1 as ::core::ffi::c_int;
                c = basec;
                while c >= '0' as i32 && c <= '9' as i32 || c == '-' as i32 {
                    if c == '-' as i32 {
                        if mflag == -(1 as ::core::ffi::c_int)
                            || n != 0 as ::core::ffi::c_int
                        {
                            break;
                        }
                        mflag = -(1 as ::core::ffi::c_int);
                    } else {
                        n = n * 10 as ::core::ffi::c_int + (c - '0' as i32);
                    }
                    if n == 0 as ::core::ffi::c_int
                        && mflag == -(1 as ::core::ffi::c_int)
                    {
                        mlwrite(b"Arg:\0" as *const u8 as *const ::core::ffi::c_char);
                    } else {
                        mlwrite(
                            b"Arg: %d\0" as *const u8 as *const ::core::ffi::c_char,
                            n * mflag,
                        );
                    }
                    loop {
                        c = getcmd();
                        if !(c == 0 as ::core::ffi::c_int) {
                            break;
                        }
                    }
                }
                n = n * mflag;
            }
            if c == reptc {
                f = TRUE;
                n = 4 as ::core::ffi::c_int;
                mflag = 0 as ::core::ffi::c_int;
                mlwrite(b"Arg: 4\0" as *const u8 as *const ::core::ffi::c_char);
                loop {
                    c = getcmd();
                    if c == 0 as ::core::ffi::c_int {
                        continue;
                    }
                    if !(c >= '0' as i32 && c <= '9' as i32 || c == reptc
                        || c == '-' as i32)
                    {
                        break;
                    }
                    if c == reptc {
                        if (n > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                            == (n * 4 as ::core::ffi::c_int > 0 as ::core::ffi::c_int)
                                as ::core::ffi::c_int
                        {
                            n = n * 4 as ::core::ffi::c_int;
                        } else {
                            n = 1 as ::core::ffi::c_int;
                        }
                    } else if c == '-' as i32 {
                        if mflag != 0 {
                            break;
                        }
                        n = 0 as ::core::ffi::c_int;
                        mflag = -(1 as ::core::ffi::c_int);
                    } else {
                        if mflag == 0 {
                            n = 0 as ::core::ffi::c_int;
                            mflag = 1 as ::core::ffi::c_int;
                        }
                        n = 10 as ::core::ffi::c_int * n + c - '0' as i32;
                    }
                    mlwrite(
                        b"Arg: %d\0" as *const u8 as *const ::core::ffi::c_char,
                        if mflag >= 0 as ::core::ffi::c_int {
                            n
                        } else if n != 0 {
                            -n
                        } else {
                            -(1 as ::core::ffi::c_int)
                        },
                    );
                }
                if mflag == -(1 as ::core::ffi::c_int) {
                    if n == 0 as ::core::ffi::c_int {
                        n += 1;
                    }
                    n = -n;
                }
            }
            if check_paste_slot_active() != 0 {
                paste_slot_handle_key(c);
            } else if command_mode_block_is_active() != 0 {
                command_mode_block_handle_key(c, f, n);
            } else {
                execute(c, f, n);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn edinit(mut bname: *mut ::core::ffi::c_char) {
    let mut bp: *mut buffer = ::core::ptr::null_mut::<buffer>();
    let mut wp: *mut window = ::core::ptr::null_mut::<window>();
    bp = bfind(bname, TRUE, 0 as ::core::ffi::c_int);
    blistp = bfind(
        b"*List*\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        TRUE,
        BFINVS,
    );
    wp = malloc(::core::mem::size_of::<window>() as size_t) as *mut window;
    if bp.is_null() || wp.is_null() || blistp.is_null() {
        exit(1 as ::core::ffi::c_int);
    }
    curbp = bp;
    (*bp).b_mode |= MDSOFTWRAP;
    curwp = wp;
    (*wp).w_bufp = bp as *mut buffer;
    (*bp).b_nwnd = 1 as ::core::ffi::c_char;
    (*wp).w_linep = (*bp).b_linep as *mut line;
    (*wp).w_dotp = (*bp).b_linep;
    (*wp).w_doto = 0 as ::core::ffi::c_int;
    (*wp).w_markp = ::core::ptr::null_mut::<line>();
    (*wp).w_marko = 0 as ::core::ffi::c_int;
    (*wp).w_force = 0 as ::core::ffi::c_char;
    (*wp).w_flag = (WFMODE | WFHARD) as ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn execute(
    mut c: ::core::ffi::c_int,
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut status: ::core::ffi::c_int = 0;
    let mut execfunc: fn_t = None;
    execfunc = getbind(c) as fn_t;
    if execfunc.is_some() {
        thisflag = 0 as ::core::ffi::c_int;
        status = Some(execfunc.expect("non-null function pointer"))
            .expect("non-null function pointer")(f, n);
        lastflag = thisflag;
        return status;
    }
    if c == ' ' as i32 && (*(*curwp).w_bufp).b_mode & MDWRAP != 0
        && fillcol > 0 as ::core::ffi::c_int && n >= 0 as ::core::ffi::c_int
        && getccol(FALSE) > fillcol && (*(*curwp).w_bufp).b_mode & MDVIEW == FALSE
    {
        execute(
            (META as ::core::ffi::c_uint | SPEC | 'W' as i32 as ::core::ffi::c_uint)
                as ::core::ffi::c_int,
            FALSE,
            1 as ::core::ffi::c_int,
        );
    }
    if c >= 0 as ::core::ffi::c_int && c <= 0x10ffff as ::core::ffi::c_int {
        if n <= 0 as ::core::ffi::c_int {
            lastflag = 0 as ::core::ffi::c_int;
            return if n < 0 as ::core::ffi::c_int { FALSE } else { TRUE };
        }
        thisflag = 0 as ::core::ffi::c_int;
        if (*(*curwp).w_bufp).b_mode & MDOVER != 0
            && (*curwp).w_doto < (*(*curwp).w_dotp).l_used
            && (*(&raw mut (*(*curwp).w_dotp).l_text as *mut ::core::ffi::c_uchar)
                .offset((*curwp).w_doto as isize) as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_int != '\t' as i32
                || (*curwp).w_doto & tab_width == tab_width)
        {
            ldelchar(1 as ::core::ffi::c_long, FALSE);
        }
        if (c == '}' as i32 || c == ')' as i32 || c == ']' as i32)
            && (*curbp).b_mode & MDCMOD != 0 as ::core::ffi::c_int
        {
            status = insbrace(n, c);
        } else if c == '#' as i32 && (*curbp).b_mode & MDCMOD != 0 as ::core::ffi::c_int
        {
            status = inspound();
        } else {
            status = sanitize_and_insert(n, c);
        }
        if (c == '}' as i32 || c == ')' as i32 || c == ']' as i32)
            && (*curbp).b_mode & MDCMOD != 0 as ::core::ffi::c_int
        {
            fmatch(c);
        }
        if (*curbp).b_mode & MDASAVE != 0 {
            gacount -= 1;
            if gacount == 0 as ::core::ffi::c_int {
                upscreen(FALSE, 0 as ::core::ffi::c_int);
                filesave(FALSE, 0 as ::core::ffi::c_int);
                gacount = gasave;
            }
        }
        lastflag = thisflag;
        return status;
    }
    vttbeep();
    nanox_set_lamp(NANOX_LAMP_WARN);
    mlwrite(b"(Key not bound)\0" as *const u8 as *const ::core::ffi::c_char);
    lastflag = 0 as ::core::ffi::c_int;
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn quickexit(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut bp: *mut buffer = ::core::ptr::null_mut::<buffer>();
    let mut oldcb: *mut buffer = ::core::ptr::null_mut::<buffer>();
    let mut status: ::core::ffi::c_int = 0;
    oldcb = curbp;
    bp = bheadp;
    while !bp.is_null() {
        if (*bp).b_flag as ::core::ffi::c_int & BFCHG != 0 as ::core::ffi::c_int
            && (*bp).b_flag as ::core::ffi::c_int & BFTRUNC == 0 as ::core::ffi::c_int
            && (*bp).b_flag as ::core::ffi::c_int & BFINVS == 0 as ::core::ffi::c_int
        {
            curbp = bp;
            mlwrite(
                b"(Saving %s)\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut (*bp).b_fname as *mut ::core::ffi::c_char,
            );
            status = filesave(f, n);
            if status != TRUE {
                curbp = oldcb;
                return status;
            }
        }
        bp = (*bp).b_bufp;
    }
    quit(f, n);
    return TRUE;
}
unsafe extern "C" fn emergencyexit(mut signr: ::core::ffi::c_int) {
    quickexit(FALSE, 0 as ::core::ffi::c_int);
    quit(TRUE, 0 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn quit(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut s: ::core::ffi::c_int = 0;
    if f != FALSE || anycb() == FALSE
        || {
            s = mlyesno(
                b"Modified buffers exist. Leave anyway\0" as *const u8
                    as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            );
            s == TRUE
        }
    {
        if lockrel() != TRUE {
            vttputc('\n' as i32);
            vttputc('\r' as i32);
            vttclose();
            vttkclose();
            exit(1 as ::core::ffi::c_int);
        }
        vttidy();
        if !hunhandle.is_null() {
            Hunspell_destroy(hunhandle);
            hunhandle = ::core::ptr::null_mut::<Hunhandle>();
        }
        nanox_cleanup();
        let mut bp: *mut buffer = bheadp;
        while !bp.is_null() {
            cleanup_backup(bp, TRUE);
            bp = (*bp).b_bufp;
        }
        bfreeall();
        varcleanup();
        kdelete();
        if !patmatch.is_null() {
            free(patmatch as *mut ::core::ffi::c_void);
            patmatch = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if !curwp.is_null() {
            free(curwp as *mut ::core::ffi::c_void);
            curwp = ::core::ptr::null_mut::<window>();
        }
        if f != 0 {
            exit(n);
        } else {
            exit(0 as ::core::ffi::c_int);
        }
    }
    mlwrite(b"\0" as *const u8 as *const ::core::ffi::c_char);
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn ctlxlp(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if kbdmode != STOP {
        mlwrite(b"%%Macro already active\0" as *const u8 as *const ::core::ffi::c_char);
        return FALSE;
    }
    mlwrite(b"(Start macro)\0" as *const u8 as *const ::core::ffi::c_char);
    kbdptr = (&raw mut kbdm as *mut ::core::ffi::c_int)
        .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int;
    kbdend = kbdptr;
    kbdmode = RECORD;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn ctlxrp(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if kbdmode == STOP {
        mlwrite(b"%%Macro not active\0" as *const u8 as *const ::core::ffi::c_char);
        return FALSE;
    }
    if kbdmode == RECORD {
        mlwrite(b"(End macro)\0" as *const u8 as *const ::core::ffi::c_char);
        kbdmode = STOP;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn ctlxe(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if kbdmode != STOP {
        mlwrite(b"%%Macro already active\0" as *const u8 as *const ::core::ffi::c_char);
        return FALSE;
    }
    if n <= 0 as ::core::ffi::c_int {
        return TRUE;
    }
    kbdrep = n;
    kbdmode = PLAY;
    kbdptr = (&raw mut kbdm as *mut ::core::ffi::c_int)
        .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn ctrlg(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    (*term).t_beep.expect("non-null function pointer")();
    if kbdmode == RECORD {
        kbdmode = STOP;
        mlwrite(b"(Macro aborted)\0" as *const u8 as *const ::core::ffi::c_char);
        return TRUE;
    }
    cutln_active = FALSE;
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn rdonly() -> ::core::ffi::c_int {
    vttbeep();
    mlwrite(b"(Key illegal in VIEW mode)\0" as *const u8 as *const ::core::ffi::c_char);
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn resterr() -> ::core::ffi::c_int {
    vttbeep();
    mlwrite(
        b"(That command is RESTRICTED)\0" as *const u8 as *const ::core::ffi::c_char,
    );
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn nullproc(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn metafn(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn cex(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn unarg(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return TRUE;
}
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args()
        .map(|arg| {
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut ::core::ffi::c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut ::core::ffi::c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(
            main_0(
                (args_ptrs.len() - 1) as ::core::ffi::c_int,
                args_ptrs.as_mut_ptr() as *mut *mut ::core::ffi::c_char,
            ) as i32,
        )
    }
}
