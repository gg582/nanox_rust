extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn tolower(__c: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
    fn atoi(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
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
    fn strchr(
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
    static mut term: *mut terminal;
    fn vttsetcolors(fg: ::core::ffi::c_int, bg: ::core::ffi::c_int);
    static mut curwp: *mut window;
    static mut gmode: ::core::ffi::c_int;
    static mut sgarbf: ::core::ffi::c_int;
    fn utf8_to_unicode(
        line: *mut ::core::ffi::c_uchar,
        index: ::core::ffi::c_uint,
        len: ::core::ffi::c_uint,
        res: *mut unicode_t,
    ) -> ::core::ffi::c_uint;
    fn unicode_width(c: unicode_t) -> ::core::ffi::c_int;
    static mut tabsize: ::core::ffi::c_int;
    fn update(force: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn movecursor(row: ::core::ffi::c_int, col: ::core::ffi::c_int);
    fn mlwrite(fmt: *const ::core::ffi::c_char, ...);
    fn ttputc(c: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn ttflush();
    fn flook(
        fname: *mut ::core::ffi::c_char,
        hflag: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn getfile(
        fname: *mut ::core::ffi::c_char,
        lockfl: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn minibuf_input(
        prompt: *const ::core::ffi::c_char,
        dest: *mut ::core::ffi::c_char,
        max_len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn minibuf_show(msg: *const ::core::ffi::c_char);
    fn highlight_init(rule_config_path: *const ::core::ffi::c_char);
    fn nanox_get_user_data_dir(out: *mut ::core::ffi::c_char, cap: size_t);
    fn nanox_get_user_config_dir(out: *mut ::core::ffi::c_char, cap: size_t);
    fn nanox_path_join(
        out: *mut ::core::ffi::c_char,
        cap: size_t,
        a: *const ::core::ffi::c_char,
        b: *const ::core::ffi::c_char,
    );
    fn nanox_file_exists(path: *const ::core::ffi::c_char) -> bool;
    fn paste_slot_clear();
    fn paste_slot_set_active(active: ::core::ffi::c_int);
    fn paste_slot_is_active() -> ::core::ffi::c_int;
    fn paste_slot_insert() -> ::core::ffi::c_int;
    fn scraper_init();
}
pub type size_t = usize;
pub type nanox_lamp_state = ::core::ffi::c_uint;
pub const NANOX_LAMP_ERROR: nanox_lamp_state = 2;
pub const NANOX_LAMP_WARN: nanox_lamp_state = 1;
pub const NANOX_LAMP_OFF: nanox_lamp_state = 0;
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
pub type FILE = _IO_FILE;
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
pub type __off64_t = ::core::ffi::c_long;
pub type _IO_lock_t = ();
pub type __off_t = ::core::ffi::c_long;
pub const _ISspace: C2RustUnnamed = 8192;
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
pub type unicode_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nanox_help_topic {
    pub title: *mut ::core::ffi::c_char,
    pub lines: *mut *mut ::core::ffi::c_char,
    pub line_count: size_t,
    pub line_cap: size_t,
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
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub const NANOX_SLOT_MAX: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const CONTROL: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;
pub const SPEC: ::core::ffi::c_uint = 0x80000000 as ::core::ffi::c_uint;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WFHARD: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const WFMODE: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const MDEXACT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
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
#[no_mangle]
pub static mut nanox_cfg: nanox_config = unsafe {
    nanox_config {
        hint_bar: true_0 != 0,
        warning_lamp: true_0 != 0,
        warning_format: ::core::mem::transmute::<
            [u8; 8],
            [::core::ffi::c_char; 8],
        >(*b"--W\0\0\0\0\0"),
        error_format: ::core::mem::transmute::<
            [u8; 8],
            [::core::ffi::c_char; 8],
        >(*b"--E\0\0\0\0\0"),
        help_key: (SPEC | 'P' as i32 as ::core::ffi::c_uint) as ::core::ffi::c_int,
        help_language: ::core::mem::transmute::<
            [u8; 8],
            [::core::ffi::c_char; 8],
        >(*b"en\0\0\0\0\0\0"),
        soft_tab: false_0 != 0,
        soft_tab_width: 8 as ::core::ffi::c_int,
        case_sensitive_default: false_0 != 0,
        nonr: false,
        no_function_slot: false_0 != 0,
    }
};
#[no_mangle]
pub static mut file_reserve: [[::core::ffi::c_char; 4096]; 64] = [[0; 4096]; 64];
#[no_mangle]
pub static mut should_redraw_underbar: bool = false_0 != 0;
static mut lamp_state: nanox_lamp_state = NANOX_LAMP_OFF;
static mut startup_slot_queue: *mut *mut ::core::ffi::c_char = ::core::ptr::null::<
    *mut ::core::ffi::c_char,
>() as *mut *mut ::core::ffi::c_char;
static mut startup_slot_queue_count: size_t = 0 as size_t;
static mut startup_slot_queue_cap: size_t = 0 as size_t;
static mut startup_slot_queue_next: size_t = 0 as size_t;
static mut help_active: bool = false;
static mut dynamic_topics: *mut nanox_help_topic = ::core::ptr::null::<
    nanox_help_topic,
>() as *mut nanox_help_topic;
static mut dynamic_topic_count: size_t = 0 as size_t;
#[no_mangle]
pub unsafe extern "C" fn nanox_request_underbar_redraw() {
    should_redraw_underbar = true_0 != 0;
}
unsafe extern "C" fn load_help_file() {
    if !dynamic_topics.is_null() {
        return;
    }
    static mut localized_name: [::core::ffi::c_char; 32] = [0; 32];
    let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut dir: [::core::ffi::c_char; 512] = [0; 512];
    static mut config_path: [::core::ffi::c_char; 4096] = [0; 4096];
    static mut localized_config_path: [::core::ffi::c_char; 4096] = [0; 4096];
    nanox_get_user_config_dir(
        &raw mut dir as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as size_t,
    );
    if nanox_cfg.help_language[0 as ::core::ffi::c_int as usize] != 0 {
        snprintf(
            &raw mut localized_name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>() as size_t,
            b"emacs-%s.hlp\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut nanox_cfg.help_language as *mut ::core::ffi::c_char,
        );
        nanox_path_join(
            &raw mut localized_config_path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as size_t,
            &raw mut dir as *mut ::core::ffi::c_char,
            &raw mut localized_name as *mut ::core::ffi::c_char,
        );
        if nanox_file_exists(
            &raw mut localized_config_path as *mut ::core::ffi::c_char,
        ) {
            path = &raw mut localized_config_path as *mut ::core::ffi::c_char;
        }
    }
    if path.is_null() {
        nanox_path_join(
            &raw mut config_path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as size_t,
            &raw mut dir as *mut ::core::ffi::c_char,
            b"emacs.hlp\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if nanox_file_exists(&raw mut config_path as *mut ::core::ffi::c_char) {
            path = &raw mut config_path as *mut ::core::ffi::c_char;
        }
    }
    if path.is_null()
        && nanox_file_exists(b"emacs.hlp\0" as *const u8 as *const ::core::ffi::c_char)
            as ::core::ffi::c_int != 0
    {
        path = b"emacs.hlp\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
    }
    if path.is_null()
        && nanox_cfg.help_language[0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int != 0
    {
        path = flook(&raw mut localized_name as *mut ::core::ffi::c_char, TRUE);
    }
    if path.is_null() {
        path = flook(
            b"emacs.hlp\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            TRUE,
        );
    }
    if path.is_null() {
        return;
    }
    let mut fp: *mut FILE = fopen(
        path,
        b"r\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if fp.is_null() {
        return;
    }
    let mut line: [::core::ffi::c_char; 256] = [0; 256];
    let mut curr: *mut nanox_help_topic = ::core::ptr::null_mut::<nanox_help_topic>();
    while !fgets(
            &raw mut line as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as ::core::ffi::c_int,
            fp,
        )
        .is_null()
    {
        let mut len: size_t = strlen(&raw mut line as *mut ::core::ffi::c_char);
        while len > 0 as size_t
            && (line[len.wrapping_sub(1 as size_t) as usize] as ::core::ffi::c_int
                == '\n' as i32
                || line[len.wrapping_sub(1 as size_t) as usize] as ::core::ffi::c_int
                    == '\r' as i32)
        {
            len = len.wrapping_sub(1);
            line[len as usize] = 0 as ::core::ffi::c_char;
        }
        if strncmp(
            &raw mut line as *mut ::core::ffi::c_char,
            b"=>\0" as *const u8 as *const ::core::ffi::c_char,
            2 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            let mut new_topics: *mut nanox_help_topic = realloc(
                dynamic_topics as *mut ::core::ffi::c_void,
                (::core::mem::size_of::<nanox_help_topic>() as size_t)
                    .wrapping_mul(dynamic_topic_count.wrapping_add(1 as size_t)),
            ) as *mut nanox_help_topic;
            if new_topics.is_null() {
                break;
            }
            dynamic_topics = new_topics;
            let fresh5 = dynamic_topic_count;
            dynamic_topic_count = dynamic_topic_count.wrapping_add(1);
            curr = dynamic_topics.offset(fresh5 as isize) as *mut nanox_help_topic;
            let mut title: *mut ::core::ffi::c_char = (&raw mut line
                as *mut ::core::ffi::c_char)
                .offset(2 as ::core::ffi::c_int as isize);
            while *title as ::core::ffi::c_int == ' ' as i32 {
                title = title.offset(1);
            }
            (*curr).title = malloc(strlen(title).wrapping_add(1 as size_t))
                as *mut ::core::ffi::c_char;
            if !(*curr).title.is_null() {
                strcpy((*curr).title, title);
            }
            (*curr).lines = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
            (*curr).line_count = 0 as size_t;
            (*curr).line_cap = 0 as size_t;
        } else {
            if curr.is_null() && len > 0 as size_t {
                let mut new_topics_0: *mut nanox_help_topic = realloc(
                    dynamic_topics as *mut ::core::ffi::c_void,
                    (::core::mem::size_of::<nanox_help_topic>() as size_t)
                        .wrapping_mul(dynamic_topic_count.wrapping_add(1 as size_t)),
                ) as *mut nanox_help_topic;
                if new_topics_0.is_null() {
                    break;
                }
                dynamic_topics = new_topics_0;
                let fresh6 = dynamic_topic_count;
                dynamic_topic_count = dynamic_topic_count.wrapping_add(1);
                curr = dynamic_topics.offset(fresh6 as isize) as *mut nanox_help_topic;
                (*curr).title = malloc(
                    strlen(
                            b"Help Information\0" as *const u8
                                as *const ::core::ffi::c_char,
                        )
                        .wrapping_add(1 as size_t),
                ) as *mut ::core::ffi::c_char;
                if !(*curr).title.is_null() {
                    strcpy(
                        (*curr).title,
                        b"Help Information\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                (*curr).lines = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
                (*curr).line_count = 0 as size_t;
                (*curr).line_cap = 0 as size_t;
            }
            if curr.is_null() {
                continue;
            }
            if (*curr).line_count >= (*curr).line_cap {
                let mut new_cap: size_t = if (*curr).line_cap != 0 {
                    (*curr).line_cap.wrapping_mul(2 as size_t)
                } else {
                    16 as size_t
                };
                let mut new_lines: *mut *mut ::core::ffi::c_char = realloc(
                    (*curr).lines as *mut ::core::ffi::c_void,
                    (::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t)
                        .wrapping_mul(new_cap),
                ) as *mut *mut ::core::ffi::c_char;
                if new_lines.is_null() {
                    break;
                }
                (*curr).lines = new_lines;
                (*curr).line_cap = new_cap;
            }
            let ref mut fresh7 = *(*curr).lines.offset((*curr).line_count as isize);
            *fresh7 = malloc(
                strlen(&raw mut line as *mut ::core::ffi::c_char)
                    .wrapping_add(1 as size_t),
            ) as *mut ::core::ffi::c_char;
            if !(*(*curr).lines.offset((*curr).line_count as isize)).is_null() {
                strcpy(
                    *(*curr).lines.offset((*curr).line_count as isize),
                    &raw mut line as *mut ::core::ffi::c_char,
                );
                (*curr).line_count = (*curr).line_count.wrapping_add(1);
            }
        }
    }
    fclose(fp);
}
unsafe extern "C" fn config_defaults() {
    nanox_cfg.hint_bar = true_0 != 0;
    nanox_cfg.warning_lamp = true_0 != 0;
    mystrscpy(
        &raw mut nanox_cfg.warning_format as *mut ::core::ffi::c_char,
        b"--W\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 8]>() as ::core::ffi::c_int,
    );
    mystrscpy(
        &raw mut nanox_cfg.error_format as *mut ::core::ffi::c_char,
        b"--E\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 8]>() as ::core::ffi::c_int,
    );
    nanox_cfg.help_key = (SPEC | 'P' as i32 as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
    mystrscpy(
        &raw mut nanox_cfg.help_language as *mut ::core::ffi::c_char,
        b"en\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 8]>() as ::core::ffi::c_int,
    );
    nanox_cfg.soft_tab = false_0 != 0;
    nanox_cfg.soft_tab_width = 8 as ::core::ffi::c_int;
    nanox_cfg.case_sensitive_default = false_0 != 0;
    nanox_cfg.nonr = false_0 != 0;
    nanox_cfg.no_function_slot = false_0 != 0;
}
unsafe extern "C" fn parse_bool(
    mut value: *const ::core::ffi::c_char,
    mut out: *mut bool,
) -> bool {
    if strcasecmp(value, b"true\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
        || strcasecmp(value, b"yes\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        || strcmp(value, b"1\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        *out = true_0 != 0;
        return true_0 != 0;
    }
    if strcasecmp(value, b"false\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
        || strcasecmp(value, b"no\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        || strcmp(value, b"0\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        *out = false_0 != 0;
        return true_0 != 0;
    }
    return false_0 != 0;
}
unsafe extern "C" fn mark_config_error() {
    nanox_set_lamp(NANOX_LAMP_ERROR);
}
unsafe extern "C" fn parse_ui_option(
    mut key: *const ::core::ffi::c_char,
    mut value: *const ::core::ffi::c_char,
) {
    if strcasecmp(key, b"hint_bar\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        if !parse_bool(value, &raw mut nanox_cfg.hint_bar) {
            mark_config_error();
        }
    } else if strcasecmp(
        key,
        b"warning_lamp\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if !parse_bool(value, &raw mut nanox_cfg.warning_lamp) {
            mark_config_error();
        }
    } else if strcasecmp(
        key,
        b"warning_format\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        mystrscpy(
            &raw mut nanox_cfg.warning_format as *mut ::core::ffi::c_char,
            value,
            ::core::mem::size_of::<[::core::ffi::c_char; 8]>() as ::core::ffi::c_int,
        );
    } else if strcasecmp(
        key,
        b"error_format\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        mystrscpy(
            &raw mut nanox_cfg.error_format as *mut ::core::ffi::c_char,
            value,
            ::core::mem::size_of::<[::core::ffi::c_char; 8]>() as ::core::ffi::c_int,
        );
    } else if strcasecmp(key, b"help_key\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        if strcasecmp(value, b"F1\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            nanox_cfg.help_key = (SPEC | 'P' as i32 as ::core::ffi::c_uint)
                as ::core::ffi::c_int;
        } else {
            mark_config_error();
        }
    } else if strcasecmp(
        key,
        b"help_language\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if *value == 0 {
            mark_config_error();
        } else {
            mystrscpy(
                &raw mut nanox_cfg.help_language as *mut ::core::ffi::c_char,
                value,
                ::core::mem::size_of::<[::core::ffi::c_char; 8]>() as ::core::ffi::c_int,
            );
            let mut p: *mut ::core::ffi::c_char = &raw mut nanox_cfg.help_language
                as *mut ::core::ffi::c_char;
            while *p != 0 {
                *p = tolower(*p as ::core::ffi::c_uchar as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
                p = p.offset(1);
            }
        }
    } else if strcasecmp(key, b"nonr\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        if !parse_bool(value, &raw mut nanox_cfg.nonr) {
            mark_config_error();
        }
    } else if strcasecmp(
        key,
        b"no_function_slot\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if !parse_bool(value, &raw mut nanox_cfg.no_function_slot) {
            mark_config_error();
        }
    }
}
unsafe extern "C" fn parse_edit_option(
    mut key: *const ::core::ffi::c_char,
    mut value: *const ::core::ffi::c_char,
) {
    if strcasecmp(key, b"soft_tab\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        if !parse_bool(value, &raw mut nanox_cfg.soft_tab) {
            mark_config_error();
        }
    } else if strcasecmp(
        key,
        b"soft_tab_width\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut width: ::core::ffi::c_int = atoi(value);
        if width <= 0 as ::core::ffi::c_int {
            mark_config_error();
        } else {
            nanox_cfg.soft_tab_width = width;
        }
    }
}
unsafe extern "C" fn parse_search_option(
    mut key: *const ::core::ffi::c_char,
    mut value: *const ::core::ffi::c_char,
) {
    if strcasecmp(
        key,
        b"case_sensitive_default\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if !parse_bool(value, &raw mut nanox_cfg.case_sensitive_default) {
            mark_config_error();
        }
    }
}
unsafe extern "C" fn parse_config_line(
    mut section: *const ::core::ffi::c_char,
    mut line: *mut ::core::ffi::c_char,
) {
    let mut equals: *mut ::core::ffi::c_char = strchr(line, '=' as i32);
    if equals.is_null() {
        return;
    }
    *equals = 0 as ::core::ffi::c_char;
    let mut key: *mut ::core::ffi::c_char = line;
    let mut value: *mut ::core::ffi::c_char = equals
        .offset(1 as ::core::ffi::c_int as isize);
    while *key as ::core::ffi::c_int != 0
        && *(*__ctype_b_loc())
            .offset(*key as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int != 0
    {
        key = key.offset(1);
    }
    while *value as ::core::ffi::c_int != 0
        && *(*__ctype_b_loc())
            .offset(*value as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int != 0
    {
        value = value.offset(1);
    }
    let mut end: *mut ::core::ffi::c_char = value.offset(strlen(value) as isize);
    while end > value
        && *(*__ctype_b_loc())
            .offset(
                *end.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_uchar
                    as ::core::ffi::c_int as isize,
            ) as ::core::ffi::c_int
            & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int != 0
    {
        end = end.offset(-1);
        *end = 0 as ::core::ffi::c_char;
    }
    end = key.offset(strlen(key) as isize);
    while end > key
        && *(*__ctype_b_loc())
            .offset(
                *end.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_uchar
                    as ::core::ffi::c_int as isize,
            ) as ::core::ffi::c_int
            & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int != 0
    {
        end = end.offset(-1);
        *end = 0 as ::core::ffi::c_char;
    }
    if strcasecmp(section, b"ui\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        parse_ui_option(key, value);
    } else if strcasecmp(section, b"edit\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        parse_edit_option(key, value);
    } else if strcasecmp(section, b"search\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        parse_search_option(key, value);
    }
}
unsafe extern "C" fn parse_config_file() {
    let mut path: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut dir: [::core::ffi::c_char; 512] = [0; 512];
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut line: [::core::ffi::c_char; 512] = [0; 512];
    let mut section: [::core::ffi::c_char; 32] = ::core::mem::transmute::<
        [u8; 32],
        [::core::ffi::c_char; 32],
    >(*b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
    nanox_get_user_config_dir(
        &raw mut dir as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as size_t,
    );
    nanox_path_join(
        &raw mut path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as size_t,
        &raw mut dir as *mut ::core::ffi::c_char,
        b"config\0" as *const u8 as *const ::core::ffi::c_char,
    );
    fp = fopen(
        &raw mut path as *mut ::core::ffi::c_char,
        b"r\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if fp.is_null() {
        nanox_get_user_data_dir(
            &raw mut dir as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as size_t,
        );
        nanox_path_join(
            &raw mut path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as size_t,
            &raw mut dir as *mut ::core::ffi::c_char,
            b"config\0" as *const u8 as *const ::core::ffi::c_char,
        );
        fp = fopen(
            &raw mut path as *mut ::core::ffi::c_char,
            b"r\0" as *const u8 as *const ::core::ffi::c_char,
        ) as *mut FILE;
    }
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
        let mut ptr: *mut ::core::ffi::c_char = &raw mut line
            as *mut ::core::ffi::c_char;
        while *ptr as ::core::ffi::c_int != 0
            && *(*__ctype_b_loc())
                .offset(*ptr as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int != 0
        {
            ptr = ptr.offset(1);
        }
        if *ptr as ::core::ffi::c_int == '#' as i32
            || *ptr as ::core::ffi::c_int == ';' as i32
            || *ptr as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            continue;
        }
        if *ptr as ::core::ffi::c_int == '[' as i32 {
            let mut close: *mut ::core::ffi::c_char = strchr(ptr, ']' as i32);
            if !close.is_null() {
                *close = 0 as ::core::ffi::c_char;
                mystrscpy(
                    &raw mut section as *mut ::core::ffi::c_char,
                    ptr.offset(1 as ::core::ffi::c_int as isize),
                    ::core::mem::size_of::<[::core::ffi::c_char; 32]>()
                        as ::core::ffi::c_int,
                );
            }
        } else {
            parse_config_line(&raw mut section as *mut ::core::ffi::c_char, ptr);
        }
    }
    fclose(fp);
}
unsafe extern "C" fn join_if_exists(
    mut out: *mut ::core::ffi::c_char,
    mut cap: size_t,
    mut dir: *const ::core::ffi::c_char,
    mut file: *const ::core::ffi::c_char,
) -> bool {
    if dir.is_null() || *dir == 0 {
        return false_0 != 0;
    }
    nanox_path_join(out, cap, dir, file);
    if *out.offset(0 as ::core::ffi::c_int as isize) == 0 {
        return false_0 != 0;
    }
    return nanox_file_exists(out);
}
unsafe extern "C" fn find_highlight_rules(
    mut out: *mut ::core::ffi::c_char,
    mut cap: size_t,
) -> bool {
    if out.is_null() || cap == 0 as size_t {
        return false_0 != 0;
    }
    *out.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
    let mut dir: [::core::ffi::c_char; 512] = [0; 512];
    nanox_get_user_config_dir(
        &raw mut dir as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as size_t,
    );
    if join_if_exists(
        out,
        cap,
        &raw mut dir as *mut ::core::ffi::c_char,
        b"highlight.ini\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        return true_0 != 0;
    }
    if join_if_exists(
        out,
        cap,
        &raw mut dir as *mut ::core::ffi::c_char,
        b"syntax.ini\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        return true_0 != 0;
    }
    nanox_get_user_data_dir(
        &raw mut dir as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as size_t,
    );
    if join_if_exists(
        out,
        cap,
        &raw mut dir as *mut ::core::ffi::c_char,
        b"highlight.ini\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        return true_0 != 0;
    }
    if join_if_exists(
        out,
        cap,
        &raw mut dir as *mut ::core::ffi::c_char,
        b"syntax.ini\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        return true_0 != 0;
    }
    let mut fallbacks: [*const ::core::ffi::c_char; 2] = [
        b"configs/nanox/syntax.ini\0" as *const u8 as *const ::core::ffi::c_char,
        b"syntax.ini\0" as *const u8 as *const ::core::ffi::c_char,
    ];
    let mut i: size_t = 0 as size_t;
    while i
        < (::core::mem::size_of::<[*const ::core::ffi::c_char; 2]>() as usize)
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
    {
        if nanox_file_exists(fallbacks[i as usize]) {
            mystrscpy(out, fallbacks[i as usize], cap as ::core::ffi::c_int);
            return true_0 != 0;
        }
        i = i.wrapping_add(1);
    }
    return false_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn nanox_apply_config() {
    if nanox_cfg.soft_tab {
        tabsize = nanox_cfg.soft_tab_width;
    } else {
        tabsize = 0 as ::core::ffi::c_int;
    }
    if nanox_cfg.case_sensitive_default {
        gmode |= MDEXACT;
    } else {
        gmode &= !MDEXACT;
    };
}
#[no_mangle]
pub unsafe extern "C" fn nanox_init() {
    config_defaults();
    parse_config_file();
    nanox_apply_config();
    let mut path: [::core::ffi::c_char; 1024] = [0; 1024];
    if !find_highlight_rules(
        &raw mut path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
    ) {
        path[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    }
    highlight_init(
        if path[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 0 {
            &raw mut path as *mut ::core::ffi::c_char
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        },
    );
    scraper_init();
}
#[no_mangle]
pub unsafe extern "C" fn nanox_set_lamp(mut state: nanox_lamp_state) {
    if !nanox_cfg.warning_lamp {
        return;
    }
    if state as ::core::ffi::c_uint
        == NANOX_LAMP_OFF as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        lamp_state = NANOX_LAMP_OFF;
        return;
    }
    if state as ::core::ffi::c_uint > lamp_state as ::core::ffi::c_uint {
        lamp_state = state;
    }
}
#[no_mangle]
pub unsafe extern "C" fn nanox_current_lamp() -> nanox_lamp_state {
    if !nanox_cfg.warning_lamp {
        return NANOX_LAMP_OFF;
    }
    return lamp_state;
}
#[no_mangle]
pub unsafe extern "C" fn nanox_lamp_label() -> *const ::core::ffi::c_char {
    match nanox_current_lamp() as ::core::ffi::c_uint {
        1 => return &raw mut nanox_cfg.warning_format as *mut ::core::ffi::c_char,
        2 => return &raw mut nanox_cfg.error_format as *mut ::core::ffi::c_char,
        _ => return b"\0" as *const u8 as *const ::core::ffi::c_char,
    };
}
#[no_mangle]
pub unsafe extern "C" fn nanox_text_rows() -> ::core::ffi::c_int {
    let mut rows: ::core::ffi::c_int = (*term).t_nrow as ::core::ffi::c_int
        - 2 as ::core::ffi::c_int;
    if rows < 1 as ::core::ffi::c_int {
        rows = 1 as ::core::ffi::c_int;
    }
    return rows;
}
#[no_mangle]
pub unsafe extern "C" fn nanox_text_cols() -> ::core::ffi::c_int {
    let mut cols: ::core::ffi::c_int = (*term).t_ncol as ::core::ffi::c_int;
    if !nanox_cfg.nonr {
        cols -= 6 as ::core::ffi::c_int;
    }
    if cols < 1 as ::core::ffi::c_int {
        cols = 1 as ::core::ffi::c_int;
    }
    return cols;
}
#[no_mangle]
pub unsafe extern "C" fn nanox_hint_top_row() -> ::core::ffi::c_int {
    let mut row: ::core::ffi::c_int = (*term).t_nrow as ::core::ffi::c_int
        - 2 as ::core::ffi::c_int;
    return if row < 0 as ::core::ffi::c_int { 0 as ::core::ffi::c_int } else { row };
}
#[no_mangle]
pub unsafe extern "C" fn nanox_hint_bottom_row() -> ::core::ffi::c_int {
    let mut row: ::core::ffi::c_int = (*term).t_nrow as ::core::ffi::c_int
        - 1 as ::core::ffi::c_int;
    return if row < 0 as ::core::ffi::c_int { 0 as ::core::ffi::c_int } else { row };
}
unsafe extern "C" fn replace_all(
    mut buffer: *mut ::core::ffi::c_char,
    mut bufsz: size_t,
    mut needle: *const ::core::ffi::c_char,
    mut replacement: *const ::core::ffi::c_char,
) {
    let mut tmp: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut needle_len: size_t = strlen(needle);
    let mut repl_len: size_t = strlen(replacement);
    let mut tmp_size: size_t = ::core::mem::size_of::<[::core::ffi::c_char; 1024]>()
        as size_t;
    if needle_len == 0 {
        return;
    }
    loop {
        let mut pos: *mut ::core::ffi::c_char = strstr(buffer, needle);
        let mut prefix_len: size_t = 0;
        let mut suffix_len: size_t = 0;
        let mut total: size_t = 0;
        if pos.is_null() {
            break;
        }
        prefix_len = pos.offset_from(buffer) as ::core::ffi::c_long as size_t;
        suffix_len = strlen(pos.offset(needle_len as isize));
        total = prefix_len.wrapping_add(repl_len).wrapping_add(suffix_len);
        if total.wrapping_add(1 as size_t) > tmp_size {
            break;
        }
        memcpy(
            &raw mut tmp as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            buffer as *const ::core::ffi::c_void,
            prefix_len,
        );
        memcpy(
            (&raw mut tmp as *mut ::core::ffi::c_char).offset(prefix_len as isize)
                as *mut ::core::ffi::c_void,
            replacement as *const ::core::ffi::c_void,
            repl_len,
        );
        memcpy(
            (&raw mut tmp as *mut ::core::ffi::c_char)
                .offset(prefix_len as isize)
                .offset(repl_len as isize) as *mut ::core::ffi::c_void,
            pos.offset(needle_len as isize) as *const ::core::ffi::c_void,
            suffix_len.wrapping_add(1 as size_t),
        );
        mystrscpy(
            buffer,
            &raw mut tmp as *mut ::core::ffi::c_char,
            bufsz as ::core::ffi::c_int,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn nanox_message_prefix(
    mut input: *const ::core::ffi::c_char,
    mut output: *mut ::core::ffi::c_char,
    mut outsz: size_t,
) {
    let mut temp: [::core::ffi::c_char; 1024] = [0; 1024];
    mystrscpy(
        &raw mut temp as *mut ::core::ffi::c_char,
        if !input.is_null() {
            input
        } else {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        },
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as ::core::ffi::c_int,
    );
    replace_all(
        &raw mut temp as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
        b"Buffer List\0" as *const u8 as *const ::core::ffi::c_char,
        b"File List\0" as *const u8 as *const ::core::ffi::c_char,
    );
    replace_all(
        &raw mut temp as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
        b"buffer list\0" as *const u8 as *const ::core::ffi::c_char,
        b"file list\0" as *const u8 as *const ::core::ffi::c_char,
    );
    replace_all(
        &raw mut temp as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
        b"Switch Buffer\0" as *const u8 as *const ::core::ffi::c_char,
        b"Switch File\0" as *const u8 as *const ::core::ffi::c_char,
    );
    replace_all(
        &raw mut temp as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
        b"switch buffer\0" as *const u8 as *const ::core::ffi::c_char,
        b"switch file\0" as *const u8 as *const ::core::ffi::c_char,
    );
    replace_all(
        &raw mut temp as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
        b"Buffer\0" as *const u8 as *const ::core::ffi::c_char,
        b"File\0" as *const u8 as *const ::core::ffi::c_char,
    );
    replace_all(
        &raw mut temp as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
        b"buffer\0" as *const u8 as *const ::core::ffi::c_char,
        b"file\0" as *const u8 as *const ::core::ffi::c_char,
    );
    replace_all(
        &raw mut temp as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
        b"BUFFER\0" as *const u8 as *const ::core::ffi::c_char,
        b"FILE\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if *(&raw mut temp as *mut ::core::ffi::c_char) == 0 {
        mystrscpy(
            output,
            &raw mut temp as *mut ::core::ffi::c_char,
            outsz as ::core::ffi::c_int,
        );
        return;
    }
    snprintf(
        output,
        outsz,
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut temp as *mut ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nanox_notify_message(mut text: *const ::core::ffi::c_char) {
    if text.is_null() || *text == 0 {
        nanox_set_lamp(NANOX_LAMP_OFF);
    }
}
unsafe extern "C" fn help_puts(mut text: *const ::core::ffi::c_char) {
    while *text != 0 {
        let fresh4 = text;
        text = text.offset(1);
        ttputc(*fresh4 as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn help_puts_width(
    mut text: *const ::core::ffi::c_char,
    mut max_cols: ::core::ffi::c_int,
) {
    if text.is_null() || max_cols <= 0 as ::core::ffi::c_int {
        return;
    }
    let mut len: size_t = strlen(text);
    let mut idx: size_t = 0 as size_t;
    let mut used: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bytes: *const ::core::ffi::c_uchar = text as *const ::core::ffi::c_uchar;
    while idx < len {
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
        if used + width > max_cols {
            break;
        }
        ttputc(uc as ::core::ffi::c_int);
        used += width;
        idx = idx.wrapping_add(consumed as size_t);
    }
}
static mut nanox_help_sheet: [*const ::core::ffi::c_char; 23] = [
    b"===============================================================================\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"=>                      NANOX SYSTEM BINDINGS & SEARCH SPEC\0" as *const u8
        as *const ::core::ffi::c_char,
    b"-------------------------------------------------------------------------------\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"FILE & SLOT CONTROL     EDITING & SEARCH        INDENT / OUTDENT\0" as *const u8
        as *const ::core::ffi::c_char,
    b"F2 / ^S : Save File     ^K : Cut Current Line   ^J : Start Indent Range\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"F3 / ^O : Open File     F7 / ^X : Cut(S:End)    ^H : Start Outdent Range\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"F4 / ^Q : Quit nanox    F6 / ^W : Copy(S:End)   Tab/gg: Apply Range\0" as *const u8
        as *const ::core::ffi::c_char,
    b"F1 / ^H : Help Menu     ^V : Command Mode       BS : Cancel Range\0" as *const u8
        as *const ::core::ffi::c_char,
    b"F9-F12 : File Slots     F8/^Y : Paste           ------------------\0" as *const u8
        as *const ::core::ffi::c_char,
    b"===============================================================================\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"* Ctrl+V opens command mode (goto/help/viblock-edit/viblock-replace/indent/outdent)\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"* viblock-edit inserts the same text on each selected line\0" as *const u8
        as *const ::core::ffi::c_char,
    b"* viblock-replace replaces the selected rectangle on each selected line\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"* F9-F12 are slot jumps; Ctrl+Alt+9, 0, -, = map to F9-F12\0" as *const u8
        as *const ::core::ffi::c_char,
    b"* F8 (and Ctrl+Alt+8) pastes from the copy/cut buffer\0" as *const u8
        as *const ::core::ffi::c_char,
    b"* nx *.txt queues files into slots instead of opening every file immediately\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"* no_function_slot = true changes the hint bar to ^A+num Slot and switches\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"  slot access to Ctrl+Alt+number mode with 64 slots\0" as *const u8
        as *const ::core::ffi::c_char,
    b"* Indent/Outdent: Ctrl+J (indent) or Ctrl+H (outdent) to mark start line\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"* Move cursor to end line, then press Tab or gg to apply to the range\0"
        as *const u8 as *const ::core::ffi::c_char,
    b"* Auto-detects file indentation width (spaces or tabs)\0" as *const u8
        as *const ::core::ffi::c_char,
    b"* BS: Backspace cancels current range operation\0" as *const u8
        as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
#[no_mangle]
pub unsafe extern "C" fn nanox_help_render() {
    if !help_active {
        return;
    }
    let mut max_r: ::core::ffi::c_int = nanox_hint_top_row() - 1 as ::core::ffi::c_int;
    vttsetcolors(-(1 as ::core::ffi::c_int), -(1 as ::core::ffi::c_int));
    let mut r: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while r <= max_r {
        movecursor(r, 0 as ::core::ffi::c_int);
        let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while c < (*term).t_ncol as ::core::ffi::c_int {
            help_puts(b" \0" as *const u8 as *const ::core::ffi::c_char);
            c += 1;
        }
        r += 1;
    }
    movecursor(0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    help_puts(b" [ Nanox Help Sheet ]\0" as *const u8 as *const ::core::ffi::c_char);
    if !dynamic_topics.is_null() && dynamic_topic_count > 0 as size_t {
        let mut r_0: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
        let mut t: size_t = 0 as size_t;
        while t < dynamic_topic_count && r_0 < max_r {
            if strcmp(
                (*dynamic_topics.offset(t as isize)).title,
                b"Help Information\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
            {
                let fresh2 = r_0;
                r_0 = r_0 + 1;
                movecursor(fresh2, 2 as ::core::ffi::c_int);
                help_puts(b"=> \0" as *const u8 as *const ::core::ffi::c_char);
                let mut avail: ::core::ffi::c_int = (*term).t_ncol as ::core::ffi::c_int
                    - 2 as ::core::ffi::c_int - 3 as ::core::ffi::c_int;
                if avail > 0 as ::core::ffi::c_int {
                    help_puts_width((*dynamic_topics.offset(t as isize)).title, avail);
                }
            }
            let mut l: size_t = 0 as size_t;
            while l < (*dynamic_topics.offset(t as isize)).line_count && r_0 < max_r {
                let fresh3 = r_0;
                r_0 = r_0 + 1;
                movecursor(fresh3, 2 as ::core::ffi::c_int);
                help_puts_width(
                    *(*dynamic_topics.offset(t as isize)).lines.offset(l as isize),
                    (*term).t_ncol as ::core::ffi::c_int - 2 as ::core::ffi::c_int,
                );
                l = l.wrapping_add(1);
            }
            t = t.wrapping_add(1);
        }
    } else {
        let mut avail_0: ::core::ffi::c_int = (*term).t_ncol as ::core::ffi::c_int
            - 2 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while !nanox_help_sheet[i as usize].is_null()
            && (i + 2 as ::core::ffi::c_int) < max_r
        {
            movecursor(i + 2 as ::core::ffi::c_int, 2 as ::core::ffi::c_int);
            help_puts_width(nanox_help_sheet[i as usize], avail_0);
            i += 1;
        }
    }
    movecursor(max_r, 0 as ::core::ffi::c_int);
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < (*term).t_ncol as ::core::ffi::c_int {
        help_puts(b"-\0" as *const u8 as *const ::core::ffi::c_char);
        i_0 += 1;
    }
    movecursor(max_r, 2 as ::core::ffi::c_int);
    help_puts_width(
        b" Press F1, ESC or Backspace to Exit Help \0" as *const u8
            as *const ::core::ffi::c_char,
        (*term).t_ncol as ::core::ffi::c_int - 2 as ::core::ffi::c_int,
    );
    vttsetcolors(-(1 as ::core::ffi::c_int), -(1 as ::core::ffi::c_int));
    ttflush();
}
#[no_mangle]
pub unsafe extern "C" fn nanox_help_command(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    load_help_file();
    help_active = true_0 != 0;
    sgarbf = TRUE;
    nanox_help_render();
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn help_close() {
    help_active = false_0 != 0;
    sgarbf = TRUE;
    update(TRUE);
}
#[no_mangle]
pub unsafe extern "C" fn nanox_help_handle_key(
    mut key: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !help_active {
        return FALSE;
    }
    let mut base_key: ::core::ffi::c_int = key;
    if key as ::core::ffi::c_uint & SPEC != 0 {
        base_key = (SPEC | (key & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uint)
            as ::core::ffi::c_int;
    }
    match base_key {
        268435527 | 268435547 | 27 | -2147483568 | 127 | 268435528 => {
            help_close();
            return TRUE;
        }
        _ => {}
    }
    return TRUE;
}
unsafe extern "C" fn slot_capacity() -> ::core::ffi::c_int {
    return if nanox_cfg.no_function_slot as ::core::ffi::c_int != 0 {
        NANOX_SLOT_MAX
    } else {
        4 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn slot_name(
    mut slot: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    static mut label: [::core::ffi::c_char; 32] = [0; 32];
    static mut names: [*const ::core::ffi::c_char; 4] = [
        b"F9\0" as *const u8 as *const ::core::ffi::c_char,
        b"F10\0" as *const u8 as *const ::core::ffi::c_char,
        b"F11\0" as *const u8 as *const ::core::ffi::c_char,
        b"F12\0" as *const u8 as *const ::core::ffi::c_char,
    ];
    if slot < 0 as ::core::ffi::c_int || slot >= slot_capacity() {
        return b"?\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if !nanox_cfg.no_function_slot && slot < 4 as ::core::ffi::c_int {
        return names[slot as usize];
    }
    snprintf(
        &raw mut label as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>() as size_t,
        b"slot %d\0" as *const u8 as *const ::core::ffi::c_char,
        slot + 1 as ::core::ffi::c_int,
    );
    return &raw mut label as *mut ::core::ffi::c_char;
}
unsafe extern "C" fn seed_startup_slots() {
    let mut max_slots: ::core::ffi::c_int = slot_capacity();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < max_slots && startup_slot_queue_next < startup_slot_queue_count {
        if !(file_reserve[i as usize][0 as ::core::ffi::c_int as usize] != 0) {
            let fresh10 = startup_slot_queue_next;
            startup_slot_queue_next = startup_slot_queue_next.wrapping_add(1);
            mystrscpy(
                &raw mut *(&raw mut file_reserve as *mut [::core::ffi::c_char; 4096])
                    .offset(i as isize) as *mut ::core::ffi::c_char,
                *startup_slot_queue.offset(fresh10 as isize),
                ::core::mem::size_of::<[::core::ffi::c_char; 4096]>()
                    as ::core::ffi::c_int,
            );
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn nanox_queue_startup_file(mut path: *const ::core::ffi::c_char) {
    let mut new_queue: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        *mut ::core::ffi::c_char,
    >();
    let mut copy: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    if path.is_null() || *path == 0 {
        return;
    }
    if startup_slot_queue_count == startup_slot_queue_cap {
        let mut new_cap: size_t = if startup_slot_queue_cap != 0 {
            startup_slot_queue_cap.wrapping_mul(2 as size_t)
        } else {
            32 as size_t
        };
        new_queue = realloc(
            startup_slot_queue as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t)
                .wrapping_mul(new_cap),
        ) as *mut *mut ::core::ffi::c_char;
        if new_queue.is_null() {
            return;
        }
        startup_slot_queue = new_queue;
        startup_slot_queue_cap = new_cap;
    }
    copy = malloc(strlen(path).wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
    if copy.is_null() {
        return;
    }
    strcpy(copy, path);
    let fresh8 = startup_slot_queue_count;
    startup_slot_queue_count = startup_slot_queue_count.wrapping_add(1);
    let ref mut fresh9 = *startup_slot_queue.offset(fresh8 as isize);
    *fresh9 = copy;
}
#[no_mangle]
pub unsafe extern "C" fn nanox_handle_closed_file(mut path: *const ::core::ffi::c_char) {
    let mut max_slots: ::core::ffi::c_int = slot_capacity();
    if path.is_null() || *path == 0 {
        return;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < max_slots {
        if strcmp(
            &raw mut *(&raw mut file_reserve as *mut [::core::ffi::c_char; 4096])
                .offset(i as isize) as *mut ::core::ffi::c_char,
            path,
        ) != 0 as ::core::ffi::c_int
        {
            i += 1;
        } else {
            file_reserve[i as usize][0 as ::core::ffi::c_int as usize] = '\0' as i32
                as ::core::ffi::c_char;
            if startup_slot_queue_next < startup_slot_queue_count {
                let fresh11 = startup_slot_queue_next;
                startup_slot_queue_next = startup_slot_queue_next.wrapping_add(1);
                mystrscpy(
                    &raw mut *(&raw mut file_reserve as *mut [::core::ffi::c_char; 4096])
                        .offset(i as isize) as *mut ::core::ffi::c_char,
                    *startup_slot_queue.offset(fresh11 as isize),
                    ::core::mem::size_of::<[::core::ffi::c_char; 4096]>()
                        as ::core::ffi::c_int,
                );
            }
            break;
        }
    }
}
unsafe extern "C" fn reserve_set(mut slot: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut prompt: [::core::ffi::c_char; 64] = [0; 64];
    let mut path: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut msg: [::core::ffi::c_char; 4160] = [0; 4160];
    let mut rc: ::core::ffi::c_int = 0;
    if slot < 0 as ::core::ffi::c_int || slot >= slot_capacity() {
        return FALSE;
    }
    snprintf(
        &raw mut prompt as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as size_t,
        b"Reserve %s file: \0" as *const u8 as *const ::core::ffi::c_char,
        slot_name(slot),
    );
    rc = minibuf_input(
        &raw mut prompt as *mut ::core::ffi::c_char,
        &raw mut path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as ::core::ffi::c_int,
    );
    if rc != TRUE {
        return rc;
    }
    if *(&raw mut path as *mut ::core::ffi::c_char) == 0 {
        return FALSE;
    }
    mystrscpy(
        &raw mut *(&raw mut file_reserve as *mut [::core::ffi::c_char; 4096])
            .offset(slot as isize) as *mut ::core::ffi::c_char,
        &raw mut path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as ::core::ffi::c_int,
    );
    snprintf(
        &raw mut msg as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4160]>() as size_t,
        b"Reserved %s = %s\0" as *const u8 as *const ::core::ffi::c_char,
        slot_name(slot),
        &raw mut path as *mut ::core::ffi::c_char,
    );
    minibuf_show(&raw mut msg as *mut ::core::ffi::c_char);
    return TRUE;
}
unsafe extern "C" fn reserve_jump(mut slot: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    let mut msg: [::core::ffi::c_char; 4160] = [0; 4160];
    if slot < 0 as ::core::ffi::c_int || slot >= slot_capacity() {
        return FALSE;
    }
    if file_reserve[slot as usize][0 as ::core::ffi::c_int as usize] == 0 {
        nanox_set_lamp(NANOX_LAMP_WARN);
        snprintf(
            &raw mut msg as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4160]>() as size_t,
            b"%s is empty (load files with nx ... or reserve it first)\0" as *const u8
                as *const ::core::ffi::c_char,
            slot_name(slot),
        );
        minibuf_show(&raw mut msg as *mut ::core::ffi::c_char);
        return FALSE;
    }
    rc = getfile(
        &raw mut *(&raw mut file_reserve as *mut [::core::ffi::c_char; 4096])
            .offset(slot as isize) as *mut ::core::ffi::c_char,
        TRUE,
    );
    if rc == TRUE {
        snprintf(
            &raw mut msg as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4160]>() as size_t,
            b"Jump %s -> %s\0" as *const u8 as *const ::core::ffi::c_char,
            slot_name(slot),
            &raw mut *(&raw mut file_reserve as *mut [::core::ffi::c_char; 4096])
                .offset(slot as isize) as *mut ::core::ffi::c_char,
        );
        minibuf_show(&raw mut msg as *mut ::core::ffi::c_char);
        nanox_set_lamp(NANOX_LAMP_OFF);
    } else {
        nanox_set_lamp(NANOX_LAMP_ERROR);
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn nanox_open_startup_slot() -> ::core::ffi::c_int {
    seed_startup_slots();
    if file_reserve[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize]
        == 0
    {
        return FALSE;
    }
    return reserve_jump(0 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn reserve_jump_numeric_mode(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut buf: [::core::ffi::c_char; 16] = [0; 16];
    let mut prompt: [::core::ffi::c_char; 32] = [0; 32];
    let mut slot: ::core::ffi::c_int = 0;
    let mut max_slots: ::core::ffi::c_int = slot_capacity();
    if !nanox_cfg.no_function_slot {
        return FALSE;
    }
    snprintf(
        &raw mut prompt as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 32]>() as size_t,
        b"Open slot (1-%d): \0" as *const u8 as *const ::core::ffi::c_char,
        max_slots,
    );
    if minibuf_input(
        &raw mut prompt as *mut ::core::ffi::c_char,
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as ::core::ffi::c_int,
    ) != TRUE
    {
        return FALSE;
    }
    slot = atoi(&raw mut buf as *mut ::core::ffi::c_char);
    if slot < 1 as ::core::ffi::c_int || slot > max_slots {
        mlwrite(
            b"Slot must be between 1 and %d\0" as *const u8
                as *const ::core::ffi::c_char,
            max_slots,
        );
        return FALSE;
    }
    return reserve_jump(slot - 1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn reserve_set_1(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return reserve_set(0 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn reserve_set_2(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return reserve_set(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn reserve_set_3(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return reserve_set(2 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn reserve_set_4(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return reserve_set(3 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn reserve_jump_1(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return reserve_jump(0 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn reserve_jump_2(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return reserve_jump(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn reserve_jump_3(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return reserve_jump(2 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn reserve_jump_4(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return reserve_jump(3 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn reserve_jump_fallback_1(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return if nanox_cfg.no_function_slot as ::core::ffi::c_int != 0 {
        reserve_jump_numeric_mode(f, n)
    } else {
        reserve_jump(0 as ::core::ffi::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn reserve_jump_fallback_2(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return if nanox_cfg.no_function_slot as ::core::ffi::c_int != 0 {
        reserve_jump_numeric_mode(f, n)
    } else {
        reserve_jump(1 as ::core::ffi::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn reserve_jump_fallback_3(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return if nanox_cfg.no_function_slot as ::core::ffi::c_int != 0 {
        reserve_jump_numeric_mode(f, n)
    } else {
        reserve_jump(2 as ::core::ffi::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn reserve_jump_fallback_4(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return if nanox_cfg.no_function_slot as ::core::ffi::c_int != 0 {
        reserve_jump_numeric_mode(f, n)
    } else {
        reserve_jump(3 as ::core::ffi::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn nanox_cleanup() {
    if !dynamic_topics.is_null() {
        let mut i: size_t = 0 as size_t;
        while i < dynamic_topic_count {
            let mut topic: *mut nanox_help_topic = dynamic_topics.offset(i as isize)
                as *mut nanox_help_topic;
            if !(*topic).title.is_null() {
                free((*topic).title as *mut ::core::ffi::c_void);
            }
            if !(*topic).lines.is_null() {
                let mut j: size_t = 0 as size_t;
                while j < (*topic).line_count {
                    if !(*(*topic).lines.offset(j as isize)).is_null() {
                        free(
                            *(*topic).lines.offset(j as isize)
                                as *mut ::core::ffi::c_void,
                        );
                    }
                    j = j.wrapping_add(1);
                }
                free((*topic).lines as *mut ::core::ffi::c_void);
            }
            i = i.wrapping_add(1);
        }
        free(dynamic_topics as *mut ::core::ffi::c_void);
        dynamic_topics = ::core::ptr::null_mut::<nanox_help_topic>();
        dynamic_topic_count = 0 as size_t;
    }
    if !startup_slot_queue.is_null() {
        let mut i_0: size_t = 0 as size_t;
        while i_0 < startup_slot_queue_count {
            free(*startup_slot_queue.offset(i_0 as isize) as *mut ::core::ffi::c_void);
            i_0 = i_0.wrapping_add(1);
        }
        free(startup_slot_queue as *mut ::core::ffi::c_void);
        startup_slot_queue = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        startup_slot_queue_count = 0 as size_t;
        startup_slot_queue_cap = 0 as size_t;
        startup_slot_queue_next = 0 as size_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn nanox_help_is_active() -> bool {
    return help_active;
}
#[no_mangle]
pub unsafe extern "C" fn help(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return nanox_help_command(f, n);
}
#[no_mangle]
pub unsafe extern "C" fn paste_slot_handle_key(
    mut c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    extern "C" {
        #[link_name = "sgarbf"]
        static mut sgarbf_0: ::core::ffi::c_int;
    }
    let mut action_taken: ::core::ffi::c_int = FALSE;
    if c == 'p' as i32 || c == 'P' as i32 || c == '\r' as i32 || c == '\n' as i32
        || c == 13 as ::core::ffi::c_int
    {
        paste_slot_insert();
        action_taken = TRUE;
    } else if c == CONTROL | '[' as i32
        || c & 0xff as ::core::ffi::c_int == 27 as ::core::ffi::c_int
        || c == CONTROL | 'G' as i32 || c == 0x7f as ::core::ffi::c_int
    {
        action_taken = TRUE;
    }
    if action_taken != 0 {
        paste_slot_set_active(0 as ::core::ffi::c_int);
        paste_slot_clear();
        (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | (WFHARD | WFMODE))
            as ::core::ffi::c_char;
        sgarbf = TRUE;
        update(TRUE);
        return TRUE;
    }
    mlwrite(
        b"Press 'p' to paste, ESC/Ctrl+G/BS to cancel\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn check_paste_slot_active() -> ::core::ffi::c_int {
    return paste_slot_is_active();
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
