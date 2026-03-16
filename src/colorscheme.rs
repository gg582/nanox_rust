extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn nanox_get_user_data_dir(out: *mut ::core::ffi::c_char, cap: size_t);
    fn nanox_get_user_config_dir(out: *mut ::core::ffi::c_char, cap: size_t);
    fn nanox_path_join(
        out: *mut ::core::ffi::c_char,
        cap: size_t,
        a: *const ::core::ffi::c_char,
        b: *const ::core::ffi::c_char,
    );
    fn nanox_file_exists(path: *const ::core::ffi::c_char) -> bool;
    fn nanox_getenv(name: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
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
    fn sscanf(
        __s: *const ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fgets(
        __s: *mut ::core::ffi::c_char,
        __n: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
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
    fn strtok(
        __s: *mut ::core::ffi::c_char,
        __delim: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
}
pub type size_t = usize;
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
pub const _ISalnum: C2RustUnnamed = 8;
pub type C2RustUnnamed = ::core::ffi::c_uint;
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
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
pub const MAX_NAME: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
static mut current_scheme_name: [::core::ffi::c_char; 64] = unsafe {
    ::core::mem::transmute::<
        [u8; 64],
        [::core::ffi::c_char; 64],
    >(
        *b"default\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    )
};
static mut styles: [HighlightStyle; 23] = [HighlightStyle {
    fg: 0,
    bg: 0,
    bold: false,
    underline: false,
    italic: false,
}; 23];
unsafe extern "C" fn set_default_scheme() {
    styles[HL_NORMAL as ::core::ffi::c_int as usize] = HighlightStyle {
        fg: -(1 as ::core::ffi::c_int),
        bg: -(1 as ::core::ffi::c_int),
        bold: false_0 != 0,
        underline: false_0 != 0,
        italic: false_0 != 0,
    };
    styles[HL_COMMENT as ::core::ffi::c_int as usize] = HighlightStyle {
        fg: 8 as ::core::ffi::c_int,
        bg: -(1 as ::core::ffi::c_int),
        bold: false_0 != 0,
        underline: false_0 != 0,
        italic: false_0 != 0,
    };
    styles[HL_STRING as ::core::ffi::c_int as usize] = HighlightStyle {
        fg: 2 as ::core::ffi::c_int,
        bg: -(1 as ::core::ffi::c_int),
        bold: false_0 != 0,
        underline: false_0 != 0,
        italic: false_0 != 0,
    };
    styles[HL_NUMBER as ::core::ffi::c_int as usize] = HighlightStyle {
        fg: 5 as ::core::ffi::c_int,
        bg: -(1 as ::core::ffi::c_int),
        bold: false_0 != 0,
        underline: false_0 != 0,
        italic: false_0 != 0,
    };
    styles[HL_BRACKET as ::core::ffi::c_int as usize] = HighlightStyle {
        fg: 6 as ::core::ffi::c_int,
        bg: -(1 as ::core::ffi::c_int),
        bold: false_0 != 0,
        underline: false_0 != 0,
        italic: false_0 != 0,
    };
    styles[HL_OPERATOR as ::core::ffi::c_int as usize] = HighlightStyle {
        fg: 6 as ::core::ffi::c_int,
        bg: -(1 as ::core::ffi::c_int),
        bold: false_0 != 0,
        underline: false_0 != 0,
        italic: false_0 != 0,
    };
    styles[HL_KEYWORD as ::core::ffi::c_int as usize] = HighlightStyle {
        fg: 3 as ::core::ffi::c_int,
        bg: -(1 as ::core::ffi::c_int),
        bold: true_0 != 0,
        underline: false_0 != 0,
        italic: false_0 != 0,
    };
    styles[HL_TYPE as ::core::ffi::c_int as usize] = HighlightStyle {
        fg: 6 as ::core::ffi::c_int,
        bg: -(1 as ::core::ffi::c_int),
        bold: false_0 != 0,
        underline: false_0 != 0,
        italic: false_0 != 0,
    };
    styles[HL_FUNCTION as ::core::ffi::c_int as usize] = HighlightStyle {
        fg: 4 as ::core::ffi::c_int,
        bg: -(1 as ::core::ffi::c_int),
        bold: false_0 != 0,
        underline: false_0 != 0,
        italic: false_0 != 0,
    };
    styles[HL_FLOW as ::core::ffi::c_int as usize] = HighlightStyle {
        fg: 3 as ::core::ffi::c_int,
        bg: -(1 as ::core::ffi::c_int),
        bold: true_0 != 0,
        underline: false_0 != 0,
        italic: false_0 != 0,
    };
    styles[HL_PREPROC as ::core::ffi::c_int as usize] = HighlightStyle {
        fg: 1 as ::core::ffi::c_int,
        bg: -(1 as ::core::ffi::c_int),
        bold: false_0 != 0,
        underline: false_0 != 0,
        italic: false_0 != 0,
    };
    styles[HL_RETURN as ::core::ffi::c_int as usize] = HighlightStyle {
        fg: 9 as ::core::ffi::c_int,
        bg: -(1 as ::core::ffi::c_int),
        bold: true_0 != 0,
        underline: false_0 != 0,
        italic: false_0 != 0,
    };
    styles[HL_ESCAPE as ::core::ffi::c_int as usize] = HighlightStyle {
        fg: 14 as ::core::ffi::c_int,
        bg: -(1 as ::core::ffi::c_int),
        bold: false_0 != 0,
        underline: false_0 != 0,
        italic: false_0 != 0,
    };
    styles[HL_CONTROL as ::core::ffi::c_int as usize] = HighlightStyle {
        fg: 9 as ::core::ffi::c_int,
        bg: -(1 as ::core::ffi::c_int),
        bold: false_0 != 0,
        underline: true_0 != 0,
        italic: false_0 != 0,
    };
    styles[HL_TERNARY as ::core::ffi::c_int as usize] = HighlightStyle {
        fg: 3 as ::core::ffi::c_int,
        bg: -(1 as ::core::ffi::c_int),
        bold: true_0 != 0,
        underline: false_0 != 0,
        italic: false_0 != 0,
    };
    styles[HL_ERROR as ::core::ffi::c_int as usize] = HighlightStyle {
        fg: 9 as ::core::ffi::c_int,
        bg: -(1 as ::core::ffi::c_int),
        bold: false_0 != 0,
        underline: false_0 != 0,
        italic: false_0 != 0,
    };
    styles[HL_NOTICE as ::core::ffi::c_int as usize] = HighlightStyle {
        fg: 208 as ::core::ffi::c_int,
        bg: -(1 as ::core::ffi::c_int),
        bold: true_0 != 0,
        underline: false_0 != 0,
        italic: false_0 != 0,
    };
    styles[HL_SELECTION as ::core::ffi::c_int as usize] = HighlightStyle {
        fg: 0 as ::core::ffi::c_int,
        bg: 11 as ::core::ffi::c_int,
        bold: false_0 != 0,
        underline: false_0 != 0,
        italic: false_0 != 0,
    };
    styles[HL_HEADER as ::core::ffi::c_int as usize] = HighlightStyle {
        fg: 4 as ::core::ffi::c_int,
        bg: -(1 as ::core::ffi::c_int),
        bold: true_0 != 0,
        underline: false_0 != 0,
        italic: false_0 != 0,
    };
    styles[HL_MD_BOLD as ::core::ffi::c_int as usize] = HighlightStyle {
        fg: -(1 as ::core::ffi::c_int),
        bg: -(1 as ::core::ffi::c_int),
        bold: true_0 != 0,
        underline: false_0 != 0,
        italic: false_0 != 0,
    };
    styles[HL_MD_ITALIC as ::core::ffi::c_int as usize] = HighlightStyle {
        fg: 6 as ::core::ffi::c_int,
        bg: -(1 as ::core::ffi::c_int),
        bold: false_0 != 0,
        underline: false_0 != 0,
        italic: true_0 != 0,
    };
    styles[HL_MD_UNDERLINE as ::core::ffi::c_int as usize] = HighlightStyle {
        fg: -(1 as ::core::ffi::c_int),
        bg: -(1 as ::core::ffi::c_int),
        bold: false_0 != 0,
        underline: true_0 != 0,
        italic: false_0 != 0,
    };
    styles[HL_LINENUM as ::core::ffi::c_int as usize] = HighlightStyle {
        fg: 8 as ::core::ffi::c_int,
        bg: -(1 as ::core::ffi::c_int),
        bold: false_0 != 0,
        underline: false_0 != 0,
        italic: false_0 != 0,
    };
}
unsafe extern "C" fn parse_color(
    mut val: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if strcmp(val, b"default\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    if *val.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '#' as i32
    {
        let mut r: ::core::ffi::c_int = 0;
        let mut g: ::core::ffi::c_int = 0;
        let mut b: ::core::ffi::c_int = 0;
        if sscanf(
            val.offset(1 as ::core::ffi::c_int as isize),
            b"%02x%02x%02x\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut r,
            &raw mut g,
            &raw mut b,
        ) == 3 as ::core::ffi::c_int
        {
            return 0x1000000 as ::core::ffi::c_int | r << 16 as ::core::ffi::c_int
                | g << 8 as ::core::ffi::c_int | b;
        }
    }
    let mut offset: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if strncmp(val, b"bright_\0" as *const u8 as *const ::core::ffi::c_char, 7 as size_t)
        == 0 as ::core::ffi::c_int
    {
        offset = 8 as ::core::ffi::c_int;
        val = val.offset(7 as ::core::ffi::c_int as isize);
    }
    if strcmp(val, b"black\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int + offset;
    }
    if strcmp(val, b"red\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        return 1 as ::core::ffi::c_int + offset;
    }
    if strcmp(val, b"green\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        return 2 as ::core::ffi::c_int + offset;
    }
    if strcmp(val, b"yellow\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        return 3 as ::core::ffi::c_int + offset;
    }
    if strcmp(val, b"blue\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        return 4 as ::core::ffi::c_int + offset;
    }
    if strcmp(val, b"magenta\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        return 5 as ::core::ffi::c_int + offset;
    }
    if strcmp(val, b"cyan\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        return 6 as ::core::ffi::c_int + offset;
    }
    if strcmp(val, b"white\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        return 7 as ::core::ffi::c_int + offset;
    }
    return -(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn parse_attributes(
    mut value: *mut ::core::ffi::c_char,
    mut style: *mut HighlightStyle,
) {
    let mut token: *mut ::core::ffi::c_char = strtok(
        value,
        b" \0" as *const u8 as *const ::core::ffi::c_char,
    );
    while !token.is_null() {
        if strncmp(
            token,
            b"fg=\0" as *const u8 as *const ::core::ffi::c_char,
            3 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            (*style).fg = parse_color(token.offset(3 as ::core::ffi::c_int as isize));
        } else if strncmp(
            token,
            b"bg=\0" as *const u8 as *const ::core::ffi::c_char,
            3 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            (*style).bg = parse_color(token.offset(3 as ::core::ffi::c_int as isize));
        } else if strcmp(
            token,
            b"bold=true\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*style).bold = true_0 != 0;
        } else if strcmp(
            token,
            b"underline=true\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*style).underline = true_0 != 0;
        } else if strcmp(
            token,
            b"italic=true\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            (*style).italic = true_0 != 0;
        }
        token = strtok(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            b" \0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
}
unsafe extern "C" fn is_safe_name(mut name: *const ::core::ffi::c_char) -> bool {
    while *name != 0 {
        if *(*__ctype_b_loc()).offset(*name as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & _ISalnum as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int == 0 && *name as ::core::ffi::c_int != '.' as i32
            && *name as ::core::ffi::c_int != '_' as i32
            && *name as ::core::ffi::c_int != '-' as i32
        {
            return false_0 != 0;
        }
        name = name.offset(1);
    }
    return true_0 != 0;
}
unsafe extern "C" fn load_scheme_file(mut path: *const ::core::ffi::c_char) {
    let mut f: *mut FILE = fopen(path, b"r\0" as *const u8 as *const ::core::ffi::c_char)
        as *mut FILE;
    if f.is_null() {
        return;
    }
    let mut line: [::core::ffi::c_char; 256] = [0; 256];
    let mut in_styles: bool = false_0 != 0;
    while !fgets(
            &raw mut line as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as ::core::ffi::c_int,
            f,
        )
        .is_null()
    {
        let mut p: *mut ::core::ffi::c_char = &raw mut line as *mut ::core::ffi::c_char;
        while *(*__ctype_b_loc()).offset(*p as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int != 0
        {
            p = p.offset(1);
        }
        if *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == ';' as i32
            || *p as ::core::ffi::c_int == '#' as i32
        {
            continue;
        }
        let mut len: size_t = strlen(p);
        while len > 0 as size_t
            && *(*__ctype_b_loc())
                .offset(
                    *p.offset(len.wrapping_sub(1 as size_t) as isize)
                        as ::core::ffi::c_int as isize,
                ) as ::core::ffi::c_int
                & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int != 0
        {
            len = len.wrapping_sub(1);
            *p.offset(len as isize) = 0 as ::core::ffi::c_char;
        }
        if *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '[' as i32
        {
            if strcmp(p, b"[styles]\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                in_styles = true_0 != 0;
            } else {
                in_styles = false_0 != 0;
            }
        } else {
            if !in_styles {
                continue;
            }
            let mut eq: *mut ::core::ffi::c_char = strchr(p, '=' as i32);
            if eq.is_null() {
                continue;
            }
            *eq = 0 as ::core::ffi::c_char;
            let mut key: *mut ::core::ffi::c_char = p;
            let mut val: *mut ::core::ffi::c_char = eq
                .offset(1 as ::core::ffi::c_int as isize);
            while *(*__ctype_b_loc()).offset(*val as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int != 0
            {
                val = val.offset(1);
            }
            while len > 0 as size_t
                && *(*__ctype_b_loc())
                    .offset(
                        *key.offset(strlen(key).wrapping_sub(1 as size_t) as isize)
                            as ::core::ffi::c_int as isize,
                    ) as ::core::ffi::c_int
                    & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                        as ::core::ffi::c_int != 0
            {
                *key.offset(strlen(key).wrapping_sub(1 as size_t) as isize) = 0
                    as ::core::ffi::c_char;
            }
            let mut id: HighlightStyleID = HL_COUNT;
            if strcmp(key, b"normal\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                id = HL_NORMAL;
            } else if strcmp(
                key,
                b"comment\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                id = HL_COMMENT;
            } else if strcmp(key, b"string\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                id = HL_STRING;
            } else if strcmp(key, b"number\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                id = HL_NUMBER;
            } else if strcmp(
                key,
                b"bracket\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                id = HL_BRACKET;
            } else if strcmp(
                key,
                b"operator\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                id = HL_OPERATOR;
            } else if strcmp(
                key,
                b"keyword\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                id = HL_KEYWORD;
            } else if strcmp(key, b"type\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                id = HL_TYPE;
            } else if strcmp(
                key,
                b"function\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                id = HL_FUNCTION;
            } else if strcmp(key, b"flow\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                id = HL_FLOW;
            } else if strcmp(
                key,
                b"preproc\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                id = HL_PREPROC;
            } else if strcmp(key, b"return\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                id = HL_RETURN;
            } else if strcmp(key, b"escape\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                id = HL_ESCAPE;
            } else if strcmp(
                key,
                b"control\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                id = HL_CONTROL;
            } else if strcmp(
                key,
                b"ternary\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                id = HL_TERNARY;
            } else if strcmp(key, b"error\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                id = HL_ERROR;
            } else if strcmp(key, b"notice\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                id = HL_NOTICE;
            } else if strcmp(
                key,
                b"selection\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                id = HL_SELECTION;
            } else if strcmp(key, b"header\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                id = HL_HEADER;
            } else if strcmp(
                key,
                b"md_bold\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                id = HL_MD_BOLD;
            } else if strcmp(
                key,
                b"md_italic\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                id = HL_MD_ITALIC;
            } else if strcmp(
                key,
                b"md_underline\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                id = HL_MD_UNDERLINE;
            } else if strcmp(
                key,
                b"linenum\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                id = HL_LINENUM;
            }
            if id as ::core::ffi::c_uint
                != HL_COUNT as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                parse_attributes(
                    val,
                    (&raw mut styles as *mut HighlightStyle).offset(id as isize)
                        as *mut HighlightStyle,
                );
            }
        }
    }
    fclose(f);
    if styles[HL_NORMAL as ::core::ffi::c_int as usize].bg != -(1 as ::core::ffi::c_int)
    {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < HL_COUNT as ::core::ffi::c_int {
            if !(i == HL_NORMAL as ::core::ffi::c_int
                || i == HL_SELECTION as ::core::ffi::c_int)
            {
                if styles[i as usize].bg == -(1 as ::core::ffi::c_int) {
                    styles[i as usize].bg = styles[HL_NORMAL as ::core::ffi::c_int
                            as usize]
                        .bg;
                }
            }
            i += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn colorscheme_init(
    mut requested_name: *const ::core::ffi::c_char,
) {
    set_default_scheme();
    let mut name: [::core::ffi::c_char; 64] = [
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
    let mut env_scheme: *const ::core::ffi::c_char = nanox_getenv(
        b"NANOX_COLORSCHEME\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if !env_scheme.is_null() && *env_scheme as ::core::ffi::c_int != 0
        && is_safe_name(env_scheme) as ::core::ffi::c_int != 0
    {
        mystrscpy(&raw mut name as *mut ::core::ffi::c_char, env_scheme, MAX_NAME);
    } else if !requested_name.is_null() && *requested_name as ::core::ffi::c_int != 0
        && is_safe_name(requested_name) as ::core::ffi::c_int != 0
    {
        mystrscpy(&raw mut name as *mut ::core::ffi::c_char, requested_name, MAX_NAME);
    } else {
        strcpy(
            &raw mut name as *mut ::core::ffi::c_char,
            b"default\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if strcmp(
        &raw mut name as *mut ::core::ffi::c_char,
        b"default\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        return;
    }
    let mut dir: [::core::ffi::c_char; 512] = [0; 512];
    let mut path: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut file_name: [::core::ffi::c_char; 76] = [0; 76];
    nanox_get_user_config_dir(
        &raw mut dir as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as size_t,
    );
    nanox_path_join(
        &raw mut dir as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as size_t,
        &raw mut dir as *mut ::core::ffi::c_char,
        b"colorscheme\0" as *const u8 as *const ::core::ffi::c_char,
    );
    snprintf(
        &raw mut file_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 76]>() as size_t,
        b"%s.nanoxcolor\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut name as *mut ::core::ffi::c_char,
    );
    nanox_path_join(
        &raw mut path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
        &raw mut dir as *mut ::core::ffi::c_char,
        &raw mut file_name as *mut ::core::ffi::c_char,
    );
    if nanox_file_exists(&raw mut path as *mut ::core::ffi::c_char) {
        load_scheme_file(&raw mut path as *mut ::core::ffi::c_char);
        mystrscpy(
            &raw mut current_scheme_name as *mut ::core::ffi::c_char,
            &raw mut name as *mut ::core::ffi::c_char,
            MAX_NAME,
        );
        return;
    }
    snprintf(
        &raw mut file_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 76]>() as size_t,
        b"%s.ini\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut name as *mut ::core::ffi::c_char,
    );
    nanox_path_join(
        &raw mut path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
        &raw mut dir as *mut ::core::ffi::c_char,
        &raw mut file_name as *mut ::core::ffi::c_char,
    );
    if nanox_file_exists(&raw mut path as *mut ::core::ffi::c_char) {
        load_scheme_file(&raw mut path as *mut ::core::ffi::c_char);
        mystrscpy(
            &raw mut current_scheme_name as *mut ::core::ffi::c_char,
            &raw mut name as *mut ::core::ffi::c_char,
            MAX_NAME,
        );
        return;
    }
    nanox_get_user_data_dir(
        &raw mut dir as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as size_t,
    );
    nanox_path_join(
        &raw mut dir as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as size_t,
        &raw mut dir as *mut ::core::ffi::c_char,
        b"colorscheme\0" as *const u8 as *const ::core::ffi::c_char,
    );
    snprintf(
        &raw mut file_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 76]>() as size_t,
        b"%s.nanoxcolor\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut name as *mut ::core::ffi::c_char,
    );
    nanox_path_join(
        &raw mut path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
        &raw mut dir as *mut ::core::ffi::c_char,
        &raw mut file_name as *mut ::core::ffi::c_char,
    );
    if nanox_file_exists(&raw mut path as *mut ::core::ffi::c_char) {
        load_scheme_file(&raw mut path as *mut ::core::ffi::c_char);
        mystrscpy(
            &raw mut current_scheme_name as *mut ::core::ffi::c_char,
            &raw mut name as *mut ::core::ffi::c_char,
            MAX_NAME,
        );
        return;
    }
    snprintf(
        &raw mut file_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 76]>() as size_t,
        b"%s.ini\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut name as *mut ::core::ffi::c_char,
    );
    nanox_path_join(
        &raw mut path as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
        &raw mut dir as *mut ::core::ffi::c_char,
        &raw mut file_name as *mut ::core::ffi::c_char,
    );
    if nanox_file_exists(&raw mut path as *mut ::core::ffi::c_char) {
        load_scheme_file(&raw mut path as *mut ::core::ffi::c_char);
        mystrscpy(
            &raw mut current_scheme_name as *mut ::core::ffi::c_char,
            &raw mut name as *mut ::core::ffi::c_char,
            MAX_NAME,
        );
        return;
    }
}
#[no_mangle]
pub unsafe extern "C" fn colorscheme_get(mut id: HighlightStyleID) -> HighlightStyle {
    if (id as ::core::ffi::c_uint) < 0 as ::core::ffi::c_uint
        || id as ::core::ffi::c_uint
            >= HL_COUNT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return styles[HL_NORMAL as ::core::ffi::c_int as usize];
    }
    return styles[id as usize];
}
#[no_mangle]
pub unsafe extern "C" fn colorscheme_get_name() -> *const ::core::ffi::c_char {
    return &raw mut current_scheme_name as *mut ::core::ffi::c_char;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
