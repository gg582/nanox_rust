extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strncasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn access(
        __name: *const ::core::ffi::c_char,
        __type: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn getcwd(
        __buf: *mut ::core::ffi::c_char,
        __size: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    static mut term: *mut terminal;
    fn vttputc(c: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn vttflush();
    fn vttmove(row: ::core::ffi::c_int, col: ::core::ffi::c_int);
    fn vttrev(state: ::core::ffi::c_int);
    static mut currow: ::core::ffi::c_int;
    static mut curcol: ::core::ffi::c_int;
    static mut curwp: *mut window;
    fn update(force: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn mlwrite(fmt: *const ::core::ffi::c_char, ...);
    fn getcmd() -> ::core::ffi::c_int;
    fn linsert(n: ::core::ffi::c_int, c: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn ldelete(n: ::core::ffi::c_long, kflag: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
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
pub struct CompletionMatch {
    pub word: [::core::ffi::c_char; 64],
    pub file: [::core::ffi::c_char; 256],
    pub line: ::core::ffi::c_int,
}
pub const R_OK: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const F_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CONTROL: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;
pub const SPEC: ::core::ffi::c_uint = 0x80000000 as ::core::ffi::c_uint;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MAX_MATCHES: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
pub const MAX_WORD_LEN: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
static mut matches: [CompletionMatch; 50] = [CompletionMatch {
    word: [0; 64],
    file: [0; 256],
    line: 0,
}; 50];
static mut match_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn find_cscope_file(
    mut path_out: *mut ::core::ffi::c_char,
    mut size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut cwd: [::core::ffi::c_char; 1024] = [0; 1024];
    if getcwd(
            &raw mut cwd as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
        )
        .is_null()
    {
        return FALSE;
    }
    let mut current: [::core::ffi::c_char; 1024] = [0; 1024];
    mystrscpy(
        &raw mut current as *mut ::core::ffi::c_char,
        &raw mut cwd as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as ::core::ffi::c_int,
    );
    loop {
        let mut attempt: [::core::ffi::c_char; 1024] = [0; 1024];
        snprintf(
            &raw mut attempt as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
            b"%s/cscope.out\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut current as *mut ::core::ffi::c_char,
        );
        if access(&raw mut attempt as *mut ::core::ffi::c_char, R_OK)
            == 0 as ::core::ffi::c_int
        {
            mystrscpy(path_out, &raw mut attempt as *mut ::core::ffi::c_char, size);
            return TRUE;
        }
        snprintf(
            &raw mut attempt as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
            b"%s/cscope.files\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut current as *mut ::core::ffi::c_char,
        );
        if access(&raw mut attempt as *mut ::core::ffi::c_char, R_OK)
            == 0 as ::core::ffi::c_int
        {
            mystrscpy(path_out, &raw mut attempt as *mut ::core::ffi::c_char, size);
            return TRUE;
        }
        snprintf(
            &raw mut attempt as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
            b"%s/.git\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut current as *mut ::core::ffi::c_char,
        );
        access(&raw mut attempt as *mut ::core::ffi::c_char, F_OK)
            == 0 as ::core::ffi::c_int;
        let mut last_slash: *mut ::core::ffi::c_char = strrchr(
            &raw mut current as *mut ::core::ffi::c_char,
            '/' as i32,
        );
        if last_slash.is_null()
            || last_slash == &raw mut current as *mut ::core::ffi::c_char
        {
            break;
        }
        *last_slash = 0 as ::core::ffi::c_char;
    }
    return FALSE;
}
unsafe extern "C" fn add_match(
    mut word: *const ::core::ffi::c_char,
    mut file: *const ::core::ffi::c_char,
    mut line: ::core::ffi::c_int,
) {
    if match_count >= MAX_MATCHES {
        return;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < match_count {
        if strcmp(
            &raw mut (*(&raw mut matches as *mut CompletionMatch).offset(i as isize))
                .word as *mut ::core::ffi::c_char,
            word,
        ) == 0 as ::core::ffi::c_int
        {
            return;
        }
        i += 1;
    }
    mystrscpy(
        &raw mut (*(&raw mut matches as *mut CompletionMatch)
            .offset(match_count as isize))
            .word as *mut ::core::ffi::c_char,
        word,
        MAX_WORD_LEN,
    );
    mystrscpy(
        &raw mut (*(&raw mut matches as *mut CompletionMatch)
            .offset(match_count as isize))
            .file as *mut ::core::ffi::c_char,
        if !file.is_null() {
            file
        } else {
            b"\0" as *const u8 as *const ::core::ffi::c_char
        },
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as ::core::ffi::c_int,
    );
    matches[match_count as usize].line = line;
    match_count += 1;
}
unsafe extern "C" fn scan_line_for_matches(
    mut line: *const ::core::ffi::c_char,
    mut prefix: *const ::core::ffi::c_char,
    mut prefix_len: ::core::ffi::c_int,
) {
    let mut p: *const ::core::ffi::c_char = line;
    while *p != 0 {
        if *(*__ctype_b_loc())
            .offset(*p as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & _ISalnum as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int == 0 && *p as ::core::ffi::c_int != '_' as i32
        {
            p = p.offset(1);
        } else {
            let mut start: *const ::core::ffi::c_char = p;
            while *p as ::core::ffi::c_int != 0
                && (*(*__ctype_b_loc())
                    .offset(*p as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & _ISalnum as ::core::ffi::c_int as ::core::ffi::c_ushort
                        as ::core::ffi::c_int != 0
                    || *p as ::core::ffi::c_int == '_' as i32)
            {
                p = p.offset(1);
            }
            let mut len: ::core::ffi::c_int = p.offset_from(start) as ::core::ffi::c_long
                as ::core::ffi::c_int;
            if len >= prefix_len && len < MAX_WORD_LEN {
                if strncasecmp(start, prefix, prefix_len as size_t)
                    == 0 as ::core::ffi::c_int
                {
                    let mut word: [::core::ffi::c_char; 64] = [0; 64];
                    memcpy(
                        &raw mut word as *mut ::core::ffi::c_char
                            as *mut ::core::ffi::c_void,
                        start as *const ::core::ffi::c_void,
                        len as size_t,
                    );
                    word[len as usize] = 0 as ::core::ffi::c_char;
                    if strcmp(&raw mut word as *mut ::core::ffi::c_char, prefix)
                        != 0 as ::core::ffi::c_int
                    {
                        add_match(
                            &raw mut word as *mut ::core::ffi::c_char,
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            0 as ::core::ffi::c_int,
                        );
                    }
                }
            }
        }
    }
}
unsafe extern "C" fn scan_cscope_file(
    mut path: *const ::core::ffi::c_char,
    mut prefix: *const ::core::ffi::c_char,
) {
    let mut f: *mut FILE = fopen(path, b"r\0" as *const u8 as *const ::core::ffi::c_char)
        as *mut FILE;
    if f.is_null() {
        return;
    }
    let mut line: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut prefix_len: ::core::ffi::c_int = strlen(prefix) as ::core::ffi::c_int;
    while !fgets(
            &raw mut line as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as ::core::ffi::c_int,
            f,
        )
        .is_null()
    {
        line[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '\t' as i32;
        scan_line_for_matches(
            &raw mut line as *mut ::core::ffi::c_char,
            prefix,
            prefix_len,
        );
    }
    fclose(f);
}
unsafe extern "C" fn draw_menu(
    mut row: ::core::ffi::c_int,
    mut col: ::core::ffi::c_int,
    mut selection: ::core::ffi::c_int,
) {
    let mut width: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < match_count {
        let mut l: ::core::ffi::c_int = strlen(
            &raw mut (*(&raw mut matches as *mut CompletionMatch).offset(i as isize))
                .word as *mut ::core::ffi::c_char,
        ) as ::core::ffi::c_int;
        if l > width {
            width = l;
        }
        i += 1;
    }
    width += 4 as ::core::ffi::c_int;
    if width > 40 as ::core::ffi::c_int {
        width = 40 as ::core::ffi::c_int;
    }
    let mut height: ::core::ffi::c_int = match_count;
    if height > 10 as ::core::ffi::c_int {
        height = 10 as ::core::ffi::c_int;
    }
    if row + height > (*term).t_nrow as ::core::ffi::c_int {
        row = (*term).t_nrow as ::core::ffi::c_int - height;
    }
    if col + width > (*term).t_ncol as ::core::ffi::c_int {
        col = (*term).t_ncol as ::core::ffi::c_int - width;
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < height {
        let mut idx: ::core::ffi::c_int = i_0;
        if idx >= match_count {
            break;
        }
        vttmove(row + i_0, col);
        if idx == selection {
            vttrev(TRUE);
        } else {
            vttrev(FALSE);
        }
        let mut buf: [::core::ffi::c_char; 64] = [0; 64];
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as size_t,
            b" %-*s \0" as *const u8 as *const ::core::ffi::c_char,
            width - 2 as ::core::ffi::c_int,
            &raw mut (*(&raw mut matches as *mut CompletionMatch).offset(idx as isize))
                .word as *mut ::core::ffi::c_char,
        );
        let mut p: *mut ::core::ffi::c_char = &raw mut buf as *mut ::core::ffi::c_char;
        while *p != 0 {
            vttputc(*p as ::core::ffi::c_int);
            p = p.offset(1);
        }
        vttrev(FALSE);
        i_0 += 1;
    }
    vttflush();
}
#[no_mangle]
pub unsafe extern "C" fn cscope_complete(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut prefix: [::core::ffi::c_char; 64] = [
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut c: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut lp: *mut line = (*curwp).w_dotp;
    let mut curoff: ::core::ffi::c_int = (*curwp).w_doto;
    let mut start: ::core::ffi::c_int = curoff;
    while start > 0 as ::core::ffi::c_int {
        let mut ch: ::core::ffi::c_int = *(&raw mut (*lp).l_text
            as *mut ::core::ffi::c_uchar)
            .offset((start - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            & 0xff as ::core::ffi::c_int;
        if *(*__ctype_b_loc()).offset(ch as isize) as ::core::ffi::c_int
            & _ISalnum as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int == 0 && ch != '_' as i32
        {
            break;
        }
        start -= 1;
    }
    if start == curoff {
        mlwrite(b"No word at cursor\0" as *const u8 as *const ::core::ffi::c_char);
        return FALSE;
    }
    let mut len: ::core::ffi::c_int = curoff - start;
    if len >= MAX_WORD_LEN {
        len = MAX_WORD_LEN - 1 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < len {
        prefix[i as usize] = (*(&raw mut (*lp).l_text as *mut ::core::ffi::c_uchar)
            .offset((start + i) as isize) as ::core::ffi::c_int
            & 0xff as ::core::ffi::c_int) as ::core::ffi::c_char;
        i += 1;
    }
    prefix[len as usize] = 0 as ::core::ffi::c_char;
    let mut cscope_path: [::core::ffi::c_char; 1024] = [0; 1024];
    match_count = 0 as ::core::ffi::c_int;
    if find_cscope_file(
        &raw mut cscope_path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as ::core::ffi::c_int,
    ) != 0
    {
        scan_cscope_file(
            &raw mut cscope_path as *mut ::core::ffi::c_char,
            &raw mut prefix as *mut ::core::ffi::c_char,
        );
    }
    if match_count == 0 as ::core::ffi::c_int {
        mlwrite(
            b"No matches found for '%s'\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut prefix as *mut ::core::ffi::c_char,
        );
        return FALSE;
    }
    let mut selection: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut row: ::core::ffi::c_int = currow + 1 as ::core::ffi::c_int;
    let mut col: ::core::ffi::c_int = curcol;
    mlwrite(
        b"Select: Arrows, Shift+Tab to insert, Esc to cancel\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    loop {
        draw_menu(row, col, selection);
        c = getcmd();
        if c == CONTROL | 'G' as i32 || c == 0x1b as ::core::ffi::c_int {
            update(TRUE);
            return FALSE;
        }
        if c as ::core::ffi::c_uint == SPEC | 'P' as i32 as ::core::ffi::c_uint
            || c == CONTROL | 'P' as i32
        {
            if selection > 0 as ::core::ffi::c_int {
                selection -= 1;
            } else {
                selection = match_count - 1 as ::core::ffi::c_int;
            }
        } else if c as ::core::ffi::c_uint == SPEC | 'N' as i32 as ::core::ffi::c_uint
            || c == CONTROL | 'N' as i32
        {
            if selection < match_count - 1 as ::core::ffi::c_int {
                selection += 1;
            } else {
                selection = 0 as ::core::ffi::c_int;
            }
        } else if c as ::core::ffi::c_uint == SPEC | 'Z' as i32 as ::core::ffi::c_uint {
            let mut dotp: *mut line = (*curwp).w_dotp;
            let mut doto: ::core::ffi::c_int = (*curwp).w_doto;
            (*curwp).w_doto = start;
            ldelete(len as ::core::ffi::c_long, FALSE);
            let mut str: *const ::core::ffi::c_char = &raw mut (*(&raw mut matches
                as *mut CompletionMatch)
                .offset(selection as isize))
                .word as *mut ::core::ffi::c_char;
            while *str != 0 {
                let fresh0 = str;
                str = str.offset(1);
                linsert(1 as ::core::ffi::c_int, *fresh0 as ::core::ffi::c_int);
            }
            update(TRUE);
            return TRUE;
        }
    };
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
        let fresh1 = src;
        src = src.offset(1);
        let mut c: ::core::ffi::c_char = *fresh1;
        if c == 0 {
            break;
        }
        let fresh2 = dst;
        dst = dst.offset(1);
        *fresh2 = c;
    }
    *dst = 0 as ::core::ffi::c_char;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
