use ::c2rust_bitfields;
use c2rust_bitfields::BitfieldStruct;
extern "C" {
    pub type __dirstream;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type re_dfa_t;
    fn colorscheme_init(scheme_name: *const ::core::ffi::c_char);
    fn nanox_get_user_data_dir(out: *mut ::core::ffi::c_char, cap: size_t);
    fn nanox_get_user_config_dir(out: *mut ::core::ffi::c_char, cap: size_t);
    fn nanox_path_join(
        out: *mut ::core::ffi::c_char,
        cap: size_t,
        a: *const ::core::ffi::c_char,
        b: *const ::core::ffi::c_char,
    );
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fgets(
        __s: *mut ::core::ffi::c_char,
        __n: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
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
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
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
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strtok(
        __s: *mut ::core::ffi::c_char,
        __delim: *const ::core::ffi::c_char,
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
    fn regcomp(
        __preg: *mut regex_t,
        __pattern: *const ::core::ffi::c_char,
        __cflags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn regexec(
        __preg: *const regex_t,
        __String: *const ::core::ffi::c_char,
        __nmatch: size_t,
        __pmatch: *mut regmatch_t,
        __eflags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn regerror(
        __errcode: ::core::ffi::c_int,
        __preg: *const regex_t,
        __errbuf: *mut ::core::ffi::c_char,
        __errbuf_size: size_t,
    ) -> size_t;
    fn regfree(__preg: *mut regex_t);
    fn closedir(__dirp: *mut DIR) -> ::core::ffi::c_int;
    fn opendir(__name: *const ::core::ffi::c_char) -> *mut DIR;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
}
pub type ptrdiff_t = isize;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Span {
    pub start: ::core::ffi::c_int,
    pub end: ::core::ffi::c_int,
    pub style: HighlightStyleID,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SpanVec {
    pub spans: [Span; 256],
    pub heap_spans: *mut Span,
    pub count: ::core::ffi::c_int,
    pub capacity: ::core::ffi::c_int,
}
pub type StateID = ::core::ffi::c_uint;
pub const HS_TRIPLE_STRING: StateID = 3;
pub const HS_STRING: StateID = 2;
pub const HS_BLOCK_COMMENT: StateID = 1;
pub const HS_NORMAL: StateID = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HighlightStackEntry {
    pub state: StateID,
    pub sub_id: ::core::ffi::c_int,
    pub string_delim: ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HighlightState {
    pub stack: [HighlightStackEntry; 8],
    pub depth: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HighlightGlobalConfig {
    pub enable_colorscheme: bool,
    pub colorscheme_name: [::core::ffi::c_char; 64],
}
pub type DIR = __dirstream;
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
pub struct CompiledFileMatch {
    pub compiled: bool,
    pub regex: regex_t,
}
pub type regex_t = re_pattern_buffer;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut re_dfa_t,
    pub allocated: __re_long_size_t,
    pub used: __re_long_size_t,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut ::core::ffi::c_char,
    pub translate: *mut ::core::ffi::c_uchar,
    pub re_nsub: size_t,
    #[bitfield(name = "can_be_null", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "regs_allocated", ty = "::core::ffi::c_uint", bits = "1..=2")]
    #[bitfield(name = "fastmap_accurate", ty = "::core::ffi::c_uint", bits = "3..=3")]
    #[bitfield(name = "no_sub", ty = "::core::ffi::c_uint", bits = "4..=4")]
    #[bitfield(name = "not_bol", ty = "::core::ffi::c_uint", bits = "5..=5")]
    #[bitfield(name = "not_eol", ty = "::core::ffi::c_uint", bits = "6..=6")]
    #[bitfield(name = "newline_anchor", ty = "::core::ffi::c_uint", bits = "7..=7")]
    pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type reg_syntax_t = ::core::ffi::c_ulong;
pub type __re_long_size_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: ::core::ffi::c_ushort,
    pub d_type: ::core::ffi::c_uchar,
    pub d_name: [::core::ffi::c_char; 256],
}
pub type __ino_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regmatch_t {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
}
pub type regoff_t = ::core::ffi::c_int;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISalnum: C2RustUnnamed = 8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColorInfo {
    pub start: ::core::ffi::c_int,
    pub end: ::core::ffi::c_int,
    pub r: ::core::ffi::c_int,
    pub g: ::core::ffi::c_int,
    pub b: ::core::ffi::c_int,
}
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const HL_MAX_SPANS: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const MAX_TOKENS: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const MAX_TOKEN_LEN: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const MAX_PROFILES: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const MAX_EXTS: ::core::ffi::c_int = 1232 as ::core::ffi::c_int;
pub const MAX_EXT_LEN: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const MAX_FILE_MATCHES: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const MAX_FILE_MATCH_PATTERN: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const HL_STATE_STACK_MAX: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
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
        let fresh9 = src;
        src = src.offset(1);
        let mut c: ::core::ffi::c_char = *fresh9;
        if c == 0 {
            break;
        }
        let fresh10 = dst;
        dst = dst.offset(1);
        *fresh10 = c;
    }
    *dst = 0 as ::core::ffi::c_char;
}
pub const REG_EXTENDED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const REG_ICASE: ::core::ffi::c_int = (1 as ::core::ffi::c_int)
    << 1 as ::core::ffi::c_int;
pub const REG_NOSUB: ::core::ffi::c_int = (1 as ::core::ffi::c_int)
    << 3 as ::core::ffi::c_int;
static mut global_config: HighlightGlobalConfig = HighlightGlobalConfig {
    enable_colorscheme: false,
    colorscheme_name: [0; 64],
};
static mut profiles: [HighlightProfile; 512] = [HighlightProfile {
    name: [0; 1232],
    extensions: [[0; 64]; 1232],
    ext_count: 0,
    file_match_patterns: [[0; 128]; 16],
    file_match_count: 0,
    line_comments: [[0; 64]; 32],
    line_comment_count: 0,
    block_comments: [BlockCommentPair {
        start: [0; 64],
        end: [0; 64],
    }; 32],
    block_comment_count: 0,
    string_delims: [0; 32],
    keywords: [[0; 64]; 1024],
    keyword_count: 0,
    type_keywords: [[0; 64]; 1024],
    type_keyword_count: 0,
    flow_keywords: [[0; 64]; 1024],
    flow_keyword_count: 0,
    preproc_keywords: [[0; 64]; 128],
    preproc_keyword_count: 0,
    return_keywords: [[0; 64]; 32],
    return_keyword_count: 0,
    enable_triple_quotes: false,
    enable_number_highlight: false,
    enable_bracket_highlight: false,
}; 512];
static mut profile_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut initialized: bool = false_0 != 0;
static mut profile_file_matches: [[CompiledFileMatch; 16]; 512] = [[CompiledFileMatch {
    compiled: false,
    regex: re_pattern_buffer {
        buffer: ::core::ptr::null::<re_dfa_t>() as *mut re_dfa_t,
        allocated: 0,
        used: 0,
        syntax: 0,
        fastmap: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        translate: ::core::ptr::null::<::core::ffi::c_uchar>()
            as *mut ::core::ffi::c_uchar,
        re_nsub: 0,
        can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [0; 1],
        c2rust_padding: [0; 7],
    },
}; 16]; 512];
unsafe extern "C" fn profile_index_from_ptr(
    mut p: *const HighlightProfile,
) -> ::core::ffi::c_int {
    if p.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    let mut idx: ptrdiff_t = p.offset_from(&raw mut profiles as *mut HighlightProfile)
        as ptrdiff_t;
    if idx < 0 as ptrdiff_t || idx >= MAX_PROFILES as ptrdiff_t {
        return -(1 as ::core::ffi::c_int);
    }
    return idx as ::core::ffi::c_int;
}
unsafe extern "C" fn clear_profile_file_matches(mut profile_index: ::core::ffi::c_int) {
    if profile_index < 0 as ::core::ffi::c_int || profile_index >= MAX_PROFILES {
        return;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_FILE_MATCHES {
        if profile_file_matches[profile_index as usize][i as usize].compiled {
            regfree(
                &raw mut (*(&raw mut *(&raw mut profile_file_matches
                    as *mut [CompiledFileMatch; 16])
                    .offset(profile_index as isize) as *mut CompiledFileMatch)
                    .offset(i as isize))
                    .regex,
            );
            profile_file_matches[profile_index as usize][i as usize].compiled = false_0
                != 0;
        }
        i += 1;
    }
}
unsafe extern "C" fn clear_all_profile_file_matches() {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < MAX_PROFILES {
        clear_profile_file_matches(i);
        i += 1;
    }
}
unsafe extern "C" fn compile_profile_file_match(
    mut profile_index: ::core::ffi::c_int,
    mut slot: ::core::ffi::c_int,
    mut pattern: *const ::core::ffi::c_char,
    mut profile_name: *const ::core::ffi::c_char,
) -> bool {
    if profile_index < 0 as ::core::ffi::c_int || profile_index >= MAX_PROFILES
        || slot < 0 as ::core::ffi::c_int || slot >= MAX_FILE_MATCHES
    {
        return false_0 != 0;
    }
    let mut entry: *mut CompiledFileMatch = (&raw mut *(&raw mut profile_file_matches
        as *mut [CompiledFileMatch; 16])
        .offset(profile_index as isize) as *mut CompiledFileMatch)
        .offset(slot as isize) as *mut CompiledFileMatch;
    let mut ret: ::core::ffi::c_int = regcomp(
        &raw mut (*entry).regex,
        pattern,
        REG_EXTENDED | REG_NOSUB | REG_ICASE,
    );
    if ret != 0 as ::core::ffi::c_int {
        let mut errbuf: [::core::ffi::c_char; 128] = [0; 128];
        regerror(
            ret,
            &raw mut (*entry).regex,
            &raw mut errbuf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
        );
        fprintf(
            stderr,
            b"nanox: invalid file_matches regex '%s' for profile '%s': %s\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            pattern,
            if !profile_name.is_null() {
                profile_name
            } else {
                b"(unknown)\0" as *const u8 as *const ::core::ffi::c_char
            },
            &raw mut errbuf as *mut ::core::ffi::c_char,
        );
        (*entry).compiled = false_0 != 0;
        return false_0 != 0;
    }
    (*entry).compiled = true_0 != 0;
    return true_0 != 0;
}
unsafe extern "C" fn profile_matches_filename(
    mut profile_index: ::core::ffi::c_int,
    mut basename: *const ::core::ffi::c_char,
) -> bool {
    if profile_index < 0 as ::core::ffi::c_int || profile_index >= profile_count
        || basename.is_null() || *basename == 0
    {
        return false_0 != 0;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < profiles[profile_index as usize].file_match_count {
        if profile_file_matches[profile_index as usize][i as usize].compiled {
            if regexec(
                &raw mut (*(&raw mut *(&raw mut profile_file_matches
                    as *mut [CompiledFileMatch; 16])
                    .offset(profile_index as isize) as *mut CompiledFileMatch)
                    .offset(i as isize))
                    .regex,
                basename,
                0 as size_t,
                ::core::ptr::null_mut::<regmatch_t>(),
                0 as ::core::ffi::c_int,
            ) == 0 as ::core::ffi::c_int
            {
                return true_0 != 0;
            }
        }
        i += 1;
    }
    return false_0 != 0;
}
unsafe extern "C" fn profile_init(
    mut p: *mut HighlightProfile,
    mut name: *const ::core::ffi::c_char,
) {
    let mut profile_index: ::core::ffi::c_int = profile_index_from_ptr(p);
    if profile_index >= 0 as ::core::ffi::c_int {
        clear_profile_file_matches(profile_index);
    }
    memset(
        p as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<HighlightProfile>() as size_t,
    );
    mystrscpy(
        &raw mut (*p).name as *mut ::core::ffi::c_char,
        name,
        ::core::mem::size_of::<[::core::ffi::c_char; 1232]>() as ::core::ffi::c_int,
    );
    (*p).enable_number_highlight = true_0 != 0;
    (*p).enable_bracket_highlight = true_0 != 0;
    (*p).enable_triple_quotes = false_0 != 0;
}
unsafe extern "C" fn prepare_profile(
    mut name: *const ::core::ffi::c_char,
) -> *mut HighlightProfile {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < profile_count {
        if strcasecmp(
            &raw mut (*(&raw mut profiles as *mut HighlightProfile).offset(i as isize))
                .name as *mut ::core::ffi::c_char,
            name,
        ) == 0 as ::core::ffi::c_int
        {
            profile_init(
                (&raw mut profiles as *mut HighlightProfile).offset(i as isize)
                    as *mut HighlightProfile,
                name,
            );
            return (&raw mut profiles as *mut HighlightProfile).offset(i as isize)
                as *mut HighlightProfile;
        }
        i += 1;
    }
    if profile_count < MAX_PROFILES {
        let fresh11 = profile_count;
        profile_count = profile_count + 1;
        let mut slot: *mut HighlightProfile = (&raw mut profiles
            as *mut HighlightProfile)
            .offset(fresh11 as isize) as *mut HighlightProfile;
        profile_init(slot, name);
        return slot;
    }
    return ::core::ptr::null_mut::<HighlightProfile>();
}
unsafe extern "C" fn trim(mut s: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let mut p: *mut ::core::ffi::c_char = s;
    while *(*__ctype_b_loc()).offset(*p as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
        != 0
    {
        p = p.offset(1);
    }
    if *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return p;
    }
    let mut end: *mut ::core::ffi::c_char = p
        .offset(strlen(p) as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    while end > p
        && *(*__ctype_b_loc()).offset(*end as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int != 0
    {
        let fresh8 = end;
        end = end.offset(-1);
        *fresh8 = 0 as ::core::ffi::c_char;
    }
    return p;
}
unsafe extern "C" fn load_config_file(
    mut path: *const ::core::ffi::c_char,
    mut allow_global: bool,
) -> bool {
    if path.is_null() || *path == 0 {
        return false_0 != 0;
    }
    let mut f: *mut FILE = fopen(path, b"r\0" as *const u8 as *const ::core::ffi::c_char)
        as *mut FILE;
    if f.is_null() {
        return false_0 != 0;
    }
    let mut line: [::core::ffi::c_char; 512] = [0; 512];
    let mut curr: *mut HighlightProfile = ::core::ptr::null_mut::<HighlightProfile>();
    let mut ignore_section: bool = false_0 != 0;
    let mut added: bool = false_0 != 0;
    while !fgets(
            &raw mut line as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as ::core::ffi::c_int,
            f,
        )
        .is_null()
    {
        let mut p: *mut ::core::ffi::c_char = trim(
            &raw mut line as *mut ::core::ffi::c_char,
        );
        if *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == ';' as i32
            || *p as ::core::ffi::c_int == '#' as i32
        {
            continue;
        }
        if *p as ::core::ffi::c_int == '[' as i32 {
            let mut end: *mut ::core::ffi::c_char = strchr(p, ']' as i32);
            if end.is_null() {
                continue;
            }
            *end = 0 as ::core::ffi::c_char;
            let mut sect: *mut ::core::ffi::c_char = p
                .offset(1 as ::core::ffi::c_int as isize);
            if strcasecmp(
                sect,
                b"highlight\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                if allow_global {
                    curr = ::core::ptr::null_mut::<HighlightProfile>();
                    ignore_section = false_0 != 0;
                } else {
                    ignore_section = true_0 != 0;
                }
            } else {
                curr = prepare_profile(sect);
                if !curr.is_null() {
                    ignore_section = false_0 != 0;
                    added = true_0 != 0;
                } else {
                    ignore_section = true_0 != 0;
                }
            }
        } else {
            if ignore_section {
                continue;
            }
            let mut eq: *mut ::core::ffi::c_char = strchr(p, '=' as i32);
            if eq.is_null() {
                continue;
            }
            *eq = 0 as ::core::ffi::c_char;
            let mut key: *mut ::core::ffi::c_char = trim(p);
            let mut val: *mut ::core::ffi::c_char = trim(
                eq.offset(1 as ::core::ffi::c_int as isize),
            );
            if curr.is_null() {
                if !allow_global {
                    continue;
                }
                if strcmp(
                    key,
                    b"enable_colorscheme\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    global_config.enable_colorscheme = strcasecmp(
                        val,
                        b"true\0" as *const u8 as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int;
                } else if strcmp(
                    key,
                    b"colorscheme\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    mystrscpy(
                        &raw mut global_config.colorscheme_name
                            as *mut ::core::ffi::c_char,
                        val,
                        ::core::mem::size_of::<[::core::ffi::c_char; 64]>()
                            as ::core::ffi::c_int,
                    );
                }
            } else if strcmp(
                key,
                b"extensions\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                let mut tok: *mut ::core::ffi::c_char = strtok(
                    val,
                    b",\0" as *const u8 as *const ::core::ffi::c_char,
                );
                (*curr).ext_count = 0 as ::core::ffi::c_int;
                while !tok.is_null() && (*curr).ext_count < MAX_EXTS {
                    let fresh0 = (*curr).ext_count;
                    (*curr).ext_count = (*curr).ext_count + 1;
                    mystrscpy(
                        &raw mut *(&raw mut (*curr).extensions
                            as *mut [::core::ffi::c_char; 64])
                            .offset(fresh0 as isize) as *mut ::core::ffi::c_char,
                        trim(tok),
                        MAX_EXT_LEN,
                    );
                    tok = strtok(
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        b",\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            } else if strcmp(
                key,
                b"file_matches\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                let mut profile_index: ::core::ffi::c_int = profile_index_from_ptr(curr);
                if profile_index >= 0 as ::core::ffi::c_int {
                    clear_profile_file_matches(profile_index);
                }
                (*curr).file_match_count = 0 as ::core::ffi::c_int;
                let mut tok_0: *mut ::core::ffi::c_char = strtok(
                    val,
                    b",\0" as *const u8 as *const ::core::ffi::c_char,
                );
                while !tok_0.is_null() && (*curr).file_match_count < MAX_FILE_MATCHES {
                    let mut pattern: *mut ::core::ffi::c_char = trim(tok_0);
                    if *pattern == 0 {
                        tok_0 = strtok(
                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            b",\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    } else {
                        let mut pattern_buf: [::core::ffi::c_char; 128] = [0; 128];
                        mystrscpy(
                            &raw mut pattern_buf as *mut ::core::ffi::c_char,
                            pattern,
                            ::core::mem::size_of::<[::core::ffi::c_char; 128]>()
                                as ::core::ffi::c_int,
                        );
                        let mut compiled: bool = true_0 != 0;
                        if profile_index >= 0 as ::core::ffi::c_int {
                            compiled = compile_profile_file_match(
                                profile_index,
                                (*curr).file_match_count,
                                &raw mut pattern_buf as *mut ::core::ffi::c_char,
                                &raw mut (*curr).name as *mut ::core::ffi::c_char,
                            );
                        }
                        if compiled {
                            mystrscpy(
                                &raw mut *(&raw mut (*curr).file_match_patterns
                                    as *mut [::core::ffi::c_char; 128])
                                    .offset((*curr).file_match_count as isize)
                                    as *mut ::core::ffi::c_char,
                                &raw mut pattern_buf as *mut ::core::ffi::c_char,
                                MAX_FILE_MATCH_PATTERN,
                            );
                            (*curr).file_match_count += 1;
                        }
                        tok_0 = strtok(
                            ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            b",\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                }
            } else if strcmp(
                key,
                b"line_comment_tokens\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                let mut tok_1: *mut ::core::ffi::c_char = strtok(
                    val,
                    b",\0" as *const u8 as *const ::core::ffi::c_char,
                );
                (*curr).line_comment_count = 0 as ::core::ffi::c_int;
                while !tok_1.is_null() && (*curr).line_comment_count < MAX_TOKENS {
                    let fresh1 = (*curr).line_comment_count;
                    (*curr).line_comment_count = (*curr).line_comment_count + 1;
                    mystrscpy(
                        &raw mut *(&raw mut (*curr).line_comments
                            as *mut [::core::ffi::c_char; 64])
                            .offset(fresh1 as isize) as *mut ::core::ffi::c_char,
                        trim(tok_1),
                        MAX_TOKEN_LEN,
                    );
                    tok_1 = strtok(
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        b",\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            } else if strcmp(
                key,
                b"block_comment_pairs\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                let mut tok_2: *mut ::core::ffi::c_char = strtok(
                    val,
                    b",\0" as *const u8 as *const ::core::ffi::c_char,
                );
                (*curr).block_comment_count = 0 as ::core::ffi::c_int;
                while !tok_2.is_null() && (*curr).block_comment_count < MAX_TOKENS {
                    tok_2 = trim(tok_2);
                    let mut sp: *mut ::core::ffi::c_char = strchr(tok_2, ' ' as i32);
                    if !sp.is_null() {
                        *sp = 0 as ::core::ffi::c_char;
                        mystrscpy(
                            &raw mut (*(&raw mut (*curr).block_comments
                                as *mut BlockCommentPair)
                                .offset((*curr).block_comment_count as isize))
                                .start as *mut ::core::ffi::c_char,
                            tok_2,
                            MAX_TOKEN_LEN,
                        );
                        mystrscpy(
                            &raw mut (*(&raw mut (*curr).block_comments
                                as *mut BlockCommentPair)
                                .offset((*curr).block_comment_count as isize))
                                .end as *mut ::core::ffi::c_char,
                            trim(sp.offset(1 as ::core::ffi::c_int as isize)),
                            MAX_TOKEN_LEN,
                        );
                        (*curr).block_comment_count += 1;
                    }
                    tok_2 = strtok(
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        b",\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            } else if strcmp(
                key,
                b"string_delims\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while *val.offset(i as isize) != 0 {
                    if *val.offset(i as isize) as ::core::ffi::c_int != ',' as i32
                        && *(*__ctype_b_loc())
                            .offset(
                                *val.offset(i as isize) as ::core::ffi::c_uchar
                                    as ::core::ffi::c_int as isize,
                            ) as ::core::ffi::c_int
                            & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                                as ::core::ffi::c_int == 0
                        && j < MAX_TOKENS - 1 as ::core::ffi::c_int
                    {
                        let fresh2 = j;
                        j = j + 1;
                        (*curr).string_delims[fresh2 as usize] = *val.offset(i as isize);
                    }
                    i += 1;
                }
                (*curr).string_delims[j as usize] = 0 as ::core::ffi::c_char;
            } else if strcmp(
                key,
                b"keywords\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                let mut tok_3: *mut ::core::ffi::c_char = strtok(
                    val,
                    b",\0" as *const u8 as *const ::core::ffi::c_char,
                );
                (*curr).keyword_count = 0 as ::core::ffi::c_int;
                while !tok_3.is_null()
                    && (*curr).keyword_count < MAX_TOKENS * 8 as ::core::ffi::c_int
                {
                    let fresh3 = (*curr).keyword_count;
                    (*curr).keyword_count = (*curr).keyword_count + 1;
                    mystrscpy(
                        &raw mut *(&raw mut (*curr).keywords
                            as *mut [::core::ffi::c_char; 64])
                            .offset(fresh3 as isize) as *mut ::core::ffi::c_char,
                        trim(tok_3),
                        MAX_TOKEN_LEN,
                    );
                    tok_3 = strtok(
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        b",\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            } else if strcmp(key, b"types\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                let mut tok_4: *mut ::core::ffi::c_char = strtok(
                    val,
                    b",\0" as *const u8 as *const ::core::ffi::c_char,
                );
                (*curr).type_keyword_count = 0 as ::core::ffi::c_int;
                while !tok_4.is_null()
                    && (*curr).type_keyword_count < MAX_TOKENS * 8 as ::core::ffi::c_int
                {
                    let fresh4 = (*curr).type_keyword_count;
                    (*curr).type_keyword_count = (*curr).type_keyword_count + 1;
                    mystrscpy(
                        &raw mut *(&raw mut (*curr).type_keywords
                            as *mut [::core::ffi::c_char; 64])
                            .offset(fresh4 as isize) as *mut ::core::ffi::c_char,
                        trim(tok_4),
                        MAX_TOKEN_LEN,
                    );
                    tok_4 = strtok(
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        b",\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            } else if strcmp(key, b"flow\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                let mut tok_5: *mut ::core::ffi::c_char = strtok(
                    val,
                    b",\0" as *const u8 as *const ::core::ffi::c_char,
                );
                (*curr).flow_keyword_count = 0 as ::core::ffi::c_int;
                while !tok_5.is_null()
                    && (*curr).flow_keyword_count < MAX_TOKENS * 8 as ::core::ffi::c_int
                {
                    let fresh5 = (*curr).flow_keyword_count;
                    (*curr).flow_keyword_count = (*curr).flow_keyword_count + 1;
                    mystrscpy(
                        &raw mut *(&raw mut (*curr).flow_keywords
                            as *mut [::core::ffi::c_char; 64])
                            .offset(fresh5 as isize) as *mut ::core::ffi::c_char,
                        trim(tok_5),
                        MAX_TOKEN_LEN,
                    );
                    tok_5 = strtok(
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        b",\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            } else if strcmp(
                key,
                b"preproc\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                let mut tok_6: *mut ::core::ffi::c_char = strtok(
                    val,
                    b",\0" as *const u8 as *const ::core::ffi::c_char,
                );
                (*curr).preproc_keyword_count = 0 as ::core::ffi::c_int;
                while !tok_6.is_null()
                    && (*curr).preproc_keyword_count
                        < MAX_TOKENS * 4 as ::core::ffi::c_int
                {
                    let fresh6 = (*curr).preproc_keyword_count;
                    (*curr).preproc_keyword_count = (*curr).preproc_keyword_count + 1;
                    mystrscpy(
                        &raw mut *(&raw mut (*curr).preproc_keywords
                            as *mut [::core::ffi::c_char; 64])
                            .offset(fresh6 as isize) as *mut ::core::ffi::c_char,
                        trim(tok_6),
                        MAX_TOKEN_LEN,
                    );
                    tok_6 = strtok(
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        b",\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            } else if strcmp(
                key,
                b"return_keywords\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                let mut tok_7: *mut ::core::ffi::c_char = strtok(
                    val,
                    b",\0" as *const u8 as *const ::core::ffi::c_char,
                );
                (*curr).return_keyword_count = 0 as ::core::ffi::c_int;
                while !tok_7.is_null() && (*curr).return_keyword_count < MAX_TOKENS {
                    let fresh7 = (*curr).return_keyword_count;
                    (*curr).return_keyword_count = (*curr).return_keyword_count + 1;
                    mystrscpy(
                        &raw mut *(&raw mut (*curr).return_keywords
                            as *mut [::core::ffi::c_char; 64])
                            .offset(fresh7 as isize) as *mut ::core::ffi::c_char,
                        trim(tok_7),
                        MAX_TOKEN_LEN,
                    );
                    tok_7 = strtok(
                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                        b",\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            } else if strcmp(
                key,
                b"enable_triple_quotes\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                (*curr).enable_triple_quotes = strcasecmp(
                    val,
                    b"true\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int;
            } else if strcmp(
                key,
                b"enable_number_highlight\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                (*curr).enable_number_highlight = strcasecmp(
                    val,
                    b"true\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int;
            } else if strcmp(
                key,
                b"enable_bracket_highlight\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                (*curr).enable_bracket_highlight = strcasecmp(
                    val,
                    b"true\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int;
            }
        }
    }
    fclose(f);
    return added;
}
unsafe extern "C" fn load_lang_dir(mut dir: *const ::core::ffi::c_char) -> bool {
    if dir.is_null() || *dir == 0 {
        return false_0 != 0;
    }
    let mut loaded: bool = false_0 != 0;
    let mut dp: *mut DIR = opendir(dir);
    if dp.is_null() {
        return false_0 != 0;
    }
    let mut entry: *mut dirent = ::core::ptr::null_mut::<dirent>();
    loop {
        entry = readdir(dp);
        if entry.is_null() {
            break;
        }
        if (*entry).d_name[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            == '.' as i32
        {
            continue;
        }
        let mut ext: *const ::core::ffi::c_char = strrchr(
            &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
            '.' as i32,
        );
        if ext.is_null()
            || strcasecmp(
                ext.offset(1 as ::core::ffi::c_int as isize),
                b"ini\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
        {
            continue;
        }
        let mut path: [::core::ffi::c_char; 4096] = [0; 4096];
        nanox_path_join(
            &raw mut path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as size_t,
            dir,
            &raw mut (*entry).d_name as *mut ::core::ffi::c_char,
        );
        if path[0 as ::core::ffi::c_int as usize] == 0 {
            continue;
        }
        if load_config_file(&raw mut path as *mut ::core::ffi::c_char, false_0 != 0) {
            loaded = true_0 != 0;
        }
    }
    closedir(dp);
    return loaded;
}
unsafe extern "C" fn load_external_langs(
    mut rule_config_path: *const ::core::ffi::c_char,
) -> bool {
    let mut loaded: bool = false_0 != 0;
    let mut dir: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut lang_path: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut tried_repo_langs: bool = false_0 != 0;
    if !rule_config_path.is_null() && *rule_config_path as ::core::ffi::c_int != 0 {
        let mut base: [::core::ffi::c_char; 4096] = [0; 4096];
        mystrscpy(
            &raw mut base as *mut ::core::ffi::c_char,
            rule_config_path,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as ::core::ffi::c_int,
        );
        let mut sep: *mut ::core::ffi::c_char = strrchr(
            &raw mut base as *mut ::core::ffi::c_char,
            '/' as i32,
        );
        if !sep.is_null() {
            *sep = 0 as ::core::ffi::c_char;
            nanox_path_join(
                &raw mut lang_path as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as size_t,
                &raw mut base as *mut ::core::ffi::c_char,
                b"langs\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if lang_path[0 as ::core::ffi::c_int as usize] != 0 {
                loaded = (loaded as ::core::ffi::c_int
                    | load_lang_dir(&raw mut lang_path as *mut ::core::ffi::c_char)
                        as ::core::ffi::c_int) != 0;
                if strcmp(
                    &raw mut lang_path as *mut ::core::ffi::c_char,
                    b"configs/nanox/langs\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    tried_repo_langs = true_0 != 0;
                }
            }
        }
    }
    nanox_get_user_config_dir(
        &raw mut dir as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as size_t,
    );
    if dir[0 as ::core::ffi::c_int as usize] != 0 {
        nanox_path_join(
            &raw mut lang_path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as size_t,
            &raw mut dir as *mut ::core::ffi::c_char,
            b"langs\0" as *const u8 as *const ::core::ffi::c_char,
        );
        loaded = (loaded as ::core::ffi::c_int
            | load_lang_dir(&raw mut lang_path as *mut ::core::ffi::c_char)
                as ::core::ffi::c_int) != 0;
    }
    nanox_get_user_data_dir(
        &raw mut dir as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as size_t,
    );
    if dir[0 as ::core::ffi::c_int as usize] != 0 {
        nanox_path_join(
            &raw mut lang_path as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as size_t,
            &raw mut dir as *mut ::core::ffi::c_char,
            b"langs\0" as *const u8 as *const ::core::ffi::c_char,
        );
        loaded = (loaded as ::core::ffi::c_int
            | load_lang_dir(&raw mut lang_path as *mut ::core::ffi::c_char)
                as ::core::ffi::c_int) != 0;
    }
    if !tried_repo_langs {
        loaded = (loaded as ::core::ffi::c_int
            | load_lang_dir(
                b"configs/nanox/langs\0" as *const u8 as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int) != 0;
    }
    return loaded;
}
#[no_mangle]
pub unsafe extern "C" fn highlight_init(
    mut rule_config_path: *const ::core::ffi::c_char,
) {
    clear_all_profile_file_matches();
    profile_count = 0 as ::core::ffi::c_int;
    global_config.enable_colorscheme = true_0 != 0;
    mystrscpy(
        &raw mut global_config.colorscheme_name as *mut ::core::ffi::c_char,
        b"nanox-dark\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as ::core::ffi::c_int,
    );
    let mut loaded_any: bool = false_0 != 0;
    if !rule_config_path.is_null() && *rule_config_path as ::core::ffi::c_int != 0 {
        loaded_any = (loaded_any as ::core::ffi::c_int
            | load_config_file(rule_config_path, true_0 != 0) as ::core::ffi::c_int)
            != 0;
    }
    loaded_any = (loaded_any as ::core::ffi::c_int
        | load_external_langs(rule_config_path) as ::core::ffi::c_int) != 0;
    if global_config.enable_colorscheme {
        colorscheme_init(
            &raw mut global_config.colorscheme_name as *mut ::core::ffi::c_char,
        );
    }
    initialized = profile_count > 0 as ::core::ffi::c_int
        && loaded_any as ::core::ffi::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn highlight_is_enabled() -> bool {
    return initialized as ::core::ffi::c_int != 0
        && global_config.enable_colorscheme as ::core::ffi::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn highlight_get_profile(
    mut filename: *const ::core::ffi::c_char,
) -> *const HighlightProfile {
    if filename.is_null() || *filename == 0 {
        return ::core::ptr::null::<HighlightProfile>();
    }
    let mut base: *const ::core::ffi::c_char = filename;
    let mut slash: *const ::core::ffi::c_char = strrchr(base, '/' as i32);
    if !slash.is_null()
        && *slash.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
    {
        base = slash.offset(1 as ::core::ffi::c_int as isize);
    }
    let mut ext: *const ::core::ffi::c_char = strrchr(base, '.' as i32);
    if !ext.is_null() {
        ext = ext.offset(1);
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < profile_count {
        if profile_matches_filename(i, base) {
            return (&raw mut profiles as *mut HighlightProfile).offset(i as isize)
                as *mut HighlightProfile;
        }
        i += 1;
    }
    if ext.is_null() {
        return ::core::ptr::null::<HighlightProfile>();
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < profile_count {
        let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while j < profiles[i_0 as usize].ext_count {
            if strcasecmp(
                ext,
                &raw mut *(&raw mut (*(&raw mut profiles as *mut HighlightProfile)
                    .offset(i_0 as isize))
                    .extensions as *mut [::core::ffi::c_char; 64])
                    .offset(j as isize) as *mut ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                return (&raw mut profiles as *mut HighlightProfile).offset(i_0 as isize)
                    as *mut HighlightProfile;
            }
            j += 1;
        }
        i_0 += 1;
    }
    return ::core::ptr::null::<HighlightProfile>();
}
unsafe extern "C" fn add_span(
    mut vec: *mut SpanVec,
    mut start: ::core::ffi::c_int,
    mut end: ::core::ffi::c_int,
    mut style: HighlightStyleID,
) {
    if start >= end {
        return;
    }
    let mut s: Span = Span {
        start: start,
        end: end,
        style: style,
    };
    if (*vec).count < HL_MAX_SPANS {
        let fresh12 = (*vec).count;
        (*vec).count = (*vec).count + 1;
        (*vec).spans[fresh12 as usize] = s;
    } else {
        if (*vec).heap_spans.is_null() {
            (*vec).capacity = HL_MAX_SPANS * 2 as ::core::ffi::c_int;
            (*vec).heap_spans = malloc(
                (::core::mem::size_of::<Span>() as size_t)
                    .wrapping_mul((*vec).capacity as size_t),
            ) as *mut Span;
            memcpy(
                (*vec).heap_spans as *mut ::core::ffi::c_void,
                &raw mut (*vec).spans as *mut Span as *const ::core::ffi::c_void,
                (::core::mem::size_of::<Span>() as size_t)
                    .wrapping_mul(HL_MAX_SPANS as size_t),
            );
        } else if (*vec).count >= (*vec).capacity {
            (*vec).capacity *= 2 as ::core::ffi::c_int;
            (*vec).heap_spans = realloc(
                (*vec).heap_spans as *mut ::core::ffi::c_void,
                (::core::mem::size_of::<Span>() as size_t)
                    .wrapping_mul((*vec).capacity as size_t),
            ) as *mut Span;
        }
        if !(*vec).heap_spans.is_null() {
            let fresh13 = (*vec).count;
            (*vec).count = (*vec).count + 1;
            *(*vec).heap_spans.offset(fresh13 as isize) = s;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn span_vec_free(mut vec: *mut SpanVec) {
    if !(*vec).heap_spans.is_null() {
        free((*vec).heap_spans as *mut ::core::ffi::c_void);
        (*vec).heap_spans = ::core::ptr::null_mut::<Span>();
    }
    (*vec).count = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn is_punct(mut c: ::core::ffi::c_char) -> bool {
    return !strchr(
            b"()[]{},;:.\0" as *const u8 as *const ::core::ffi::c_char,
            c as ::core::ffi::c_int,
        )
        .is_null();
}
unsafe extern "C" fn is_operator(mut c: ::core::ffi::c_char) -> bool {
    return !strchr(
            b"+-*/%=&|<>!^~\0" as *const u8 as *const ::core::ffi::c_char,
            c as ::core::ffi::c_int,
        )
        .is_null();
}
unsafe extern "C" fn starts_with(
    mut text: *const ::core::ffi::c_char,
    mut prefix: *const ::core::ffi::c_char,
) -> bool {
    return strncmp(text, prefix, strlen(prefix)) == 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn is_control(mut c: ::core::ffi::c_uchar) -> bool {
    return (c as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
        && c as ::core::ffi::c_int != '\t' as i32
        && c as ::core::ffi::c_int != '\n' as i32
        && c as ::core::ffi::c_int != '\r' as i32
        || c as ::core::ffi::c_int == 127 as ::core::ffi::c_int;
}
unsafe extern "C" fn is_markdown_profile(mut profile: *const HighlightProfile) -> bool {
    if profile.is_null() {
        return false_0 != 0;
    }
    return strcasecmp(
        &raw const (*profile).name as *const ::core::ffi::c_char,
        b"markdown\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn is_html_profile(mut profile: *const HighlightProfile) -> bool {
    if profile.is_null() {
        return false_0 != 0;
    }
    return strcasecmp(
        &raw const (*profile).name as *const ::core::ffi::c_char,
        b"html\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn profile_supports_at_annotations(
    mut profile: *const HighlightProfile,
) -> bool {
    if profile.is_null()
        || (*profile).name[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            == '\0' as i32
    {
        return false_0 != 0;
    }
    let mut name: *const ::core::ffi::c_char = &raw const (*profile).name
        as *const ::core::ffi::c_char;
    return strcasecmp(name, b"java\0" as *const u8 as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
        || strcasecmp(name, b"kotlin\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        || strcasecmp(name, b"scala\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        || strcasecmp(name, b"groovy\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn find_md_closing(
    mut text: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
    mut start: ::core::ffi::c_int,
    mut delim: *const ::core::ffi::c_char,
    mut delim_len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pos: ::core::ffi::c_int = start;
    while pos <= len - delim_len {
        if strncmp(text.offset(pos as isize), delim, delim_len as size_t)
            == 0 as ::core::ffi::c_int
        {
            let mut backslash_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut check: ::core::ffi::c_int = pos - 1 as ::core::ffi::c_int;
            while check >= start
                && *text.offset(check as isize) as ::core::ffi::c_int == '\\' as i32
            {
                backslash_count += 1;
                check -= 1;
            }
            if backslash_count % 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                return pos + delim_len;
            }
        }
        pos += 1;
    }
    return -(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn hex_digit_value(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    if c as ::core::ffi::c_int >= '0' as i32 && c as ::core::ffi::c_int <= '9' as i32 {
        return c as ::core::ffi::c_int - '0' as i32;
    }
    if c as ::core::ffi::c_int >= 'a' as i32 && c as ::core::ffi::c_int <= 'f' as i32 {
        return 10 as ::core::ffi::c_int + (c as ::core::ffi::c_int - 'a' as i32);
    }
    if c as ::core::ffi::c_int >= 'A' as i32 && c as ::core::ffi::c_int <= 'F' as i32 {
        return 10 as ::core::ffi::c_int + (c as ::core::ffi::c_int - 'A' as i32);
    }
    return -(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn is_hex_color(
    mut text: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
    mut pos: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if pos >= len || *text.offset(pos as isize) as ::core::ffi::c_int != '#' as i32 {
        return 0 as ::core::ffi::c_int;
    }
    if pos + 7 as ::core::ffi::c_int <= len {
        let mut valid: bool = true_0 != 0;
        let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        while i <= 6 as ::core::ffi::c_int {
            if hex_digit_value(*text.offset((pos + i) as isize))
                < 0 as ::core::ffi::c_int
            {
                valid = false_0 != 0;
                break;
            } else {
                i += 1;
            }
        }
        if valid {
            if pos + 7 as ::core::ffi::c_int >= len {
                return 7 as ::core::ffi::c_int;
            }
            let mut next: ::core::ffi::c_char = *text
                .offset((pos + 7 as ::core::ffi::c_int) as isize);
            if hex_digit_value(next) >= 0 as ::core::ffi::c_int {
                return 7 as ::core::ffi::c_int;
            }
            if *(*__ctype_b_loc())
                .offset(next as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & _ISalnum as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int != 0
                || next as ::core::ffi::c_int == '_' as i32
            {
                return 0 as ::core::ffi::c_int;
            }
            return 7 as ::core::ffi::c_int;
        }
    }
    if pos + 4 as ::core::ffi::c_int <= len {
        let mut valid_0: bool = true_0 != 0;
        let mut i_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        while i_0 <= 3 as ::core::ffi::c_int {
            if hex_digit_value(*text.offset((pos + i_0) as isize))
                < 0 as ::core::ffi::c_int
            {
                valid_0 = false_0 != 0;
                break;
            } else {
                i_0 += 1;
            }
        }
        if valid_0 {
            if pos + 4 as ::core::ffi::c_int >= len {
                return 4 as ::core::ffi::c_int;
            }
            let mut next_0: ::core::ffi::c_char = *text
                .offset((pos + 4 as ::core::ffi::c_int) as isize);
            if hex_digit_value(next_0) >= 0 as ::core::ffi::c_int {
                return 0 as ::core::ffi::c_int;
            }
            if *(*__ctype_b_loc())
                .offset(next_0 as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & _ISalnum as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int != 0
                || next_0 as ::core::ffi::c_int == '_' as i32
            {
                return 0 as ::core::ffi::c_int;
            }
            return 4 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn is_rgb_color(
    mut text: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
    mut pos: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut start_paren: ::core::ffi::c_int = 0;
    if pos + 4 as ::core::ffi::c_int <= len
        && strncmp(
            text.offset(pos as isize),
            b"rgb(\0" as *const u8 as *const ::core::ffi::c_char,
            4 as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        start_paren = pos + 4 as ::core::ffi::c_int;
    } else if pos + 5 as ::core::ffi::c_int <= len
        && strncmp(
            text.offset(pos as isize),
            b"rgba(\0" as *const u8 as *const ::core::ffi::c_char,
            5 as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        start_paren = pos + 5 as ::core::ffi::c_int;
    } else {
        return 0 as ::core::ffi::c_int
    }
    let mut search: ::core::ffi::c_int = start_paren;
    let mut paren_depth: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while search < len && paren_depth > 0 as ::core::ffi::c_int {
        if *text.offset(search as isize) as ::core::ffi::c_int == '(' as i32 {
            paren_depth += 1;
        } else if *text.offset(search as isize) as ::core::ffi::c_int == ')' as i32 {
            paren_depth -= 1;
        }
        search += 1;
    }
    if paren_depth == 0 as ::core::ffi::c_int {
        return search - pos;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn is_hsl_color(
    mut text: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
    mut pos: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut start_paren: ::core::ffi::c_int = 0;
    if pos + 4 as ::core::ffi::c_int <= len
        && strncmp(
            text.offset(pos as isize),
            b"hsl(\0" as *const u8 as *const ::core::ffi::c_char,
            4 as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        start_paren = pos + 4 as ::core::ffi::c_int;
    } else if pos + 5 as ::core::ffi::c_int <= len
        && strncmp(
            text.offset(pos as isize),
            b"hsla(\0" as *const u8 as *const ::core::ffi::c_char,
            5 as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        start_paren = pos + 5 as ::core::ffi::c_int;
    } else {
        return 0 as ::core::ffi::c_int
    }
    let mut search: ::core::ffi::c_int = start_paren;
    let mut paren_depth: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while search < len && paren_depth > 0 as ::core::ffi::c_int {
        if *text.offset(search as isize) as ::core::ffi::c_int == '(' as i32 {
            paren_depth += 1;
        } else if *text.offset(search as isize) as ::core::ffi::c_int == ')' as i32 {
            paren_depth -= 1;
        }
        search += 1;
    }
    if paren_depth == 0 as ::core::ffi::c_int {
        return search - pos;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn parse_hex_color(
    mut text: *const ::core::ffi::c_char,
    mut pos: ::core::ffi::c_int,
    mut len: ::core::ffi::c_int,
    mut r: *mut ::core::ffi::c_int,
    mut g: *mut ::core::ffi::c_int,
    mut b: *mut ::core::ffi::c_int,
) -> bool {
    if len == 7 as ::core::ffi::c_int {
        *r = hex_digit_value(*text.offset((pos + 1 as ::core::ffi::c_int) as isize))
            * 16 as ::core::ffi::c_int
            + hex_digit_value(*text.offset((pos + 2 as ::core::ffi::c_int) as isize));
        *g = hex_digit_value(*text.offset((pos + 3 as ::core::ffi::c_int) as isize))
            * 16 as ::core::ffi::c_int
            + hex_digit_value(*text.offset((pos + 4 as ::core::ffi::c_int) as isize));
        *b = hex_digit_value(*text.offset((pos + 5 as ::core::ffi::c_int) as isize))
            * 16 as ::core::ffi::c_int
            + hex_digit_value(*text.offset((pos + 6 as ::core::ffi::c_int) as isize));
        return true_0 != 0;
    } else if len == 4 as ::core::ffi::c_int {
        let mut rv: ::core::ffi::c_int = hex_digit_value(
            *text.offset((pos + 1 as ::core::ffi::c_int) as isize),
        );
        let mut gv: ::core::ffi::c_int = hex_digit_value(
            *text.offset((pos + 2 as ::core::ffi::c_int) as isize),
        );
        let mut bv: ::core::ffi::c_int = hex_digit_value(
            *text.offset((pos + 3 as ::core::ffi::c_int) as isize),
        );
        *r = rv * 16 as ::core::ffi::c_int + rv;
        *g = gv * 16 as ::core::ffi::c_int + gv;
        *b = bv * 16 as ::core::ffi::c_int + bv;
        return true_0 != 0;
    }
    return false_0 != 0;
}
unsafe extern "C" fn parse_rgb_color(
    mut text: *const ::core::ffi::c_char,
    mut pos: ::core::ffi::c_int,
    mut color_len: ::core::ffi::c_int,
    mut r: *mut ::core::ffi::c_int,
    mut g: *mut ::core::ffi::c_int,
    mut b: *mut ::core::ffi::c_int,
) -> bool {
    let mut paren: ::core::ffi::c_int = pos;
    while paren < pos + color_len
        && *text.offset(paren as isize) as ::core::ffi::c_int != '(' as i32
    {
        paren += 1;
    }
    if paren >= pos + color_len {
        return false_0 != 0;
    }
    paren += 1;
    let mut values: [::core::ffi::c_int; 4] = [
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        255 as ::core::ffi::c_int,
    ];
    let mut value_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut current_value: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut in_number: bool = false_0 != 0;
    let mut has_percent: bool = false_0 != 0;
    while paren < pos + color_len
        && *text.offset(paren as isize) as ::core::ffi::c_int != ')' as i32
        && value_count < 4 as ::core::ffi::c_int
    {
        let mut c: ::core::ffi::c_char = *text.offset(paren as isize);
        if c as ::core::ffi::c_int >= '0' as i32 && c as ::core::ffi::c_int <= '9' as i32
        {
            current_value = current_value * 10 as ::core::ffi::c_int
                + (c as ::core::ffi::c_int - '0' as i32);
            in_number = true_0 != 0;
        } else if c as ::core::ffi::c_int == '%' as i32 {
            has_percent = true_0 != 0;
        } else if c as ::core::ffi::c_int == ',' as i32
            || c as ::core::ffi::c_int == ' ' as i32
            || c as ::core::ffi::c_int == '/' as i32
        {
            if in_number {
                if has_percent {
                    current_value = current_value * 255 as ::core::ffi::c_int
                        / 100 as ::core::ffi::c_int;
                    has_percent = false_0 != 0;
                }
                let fresh14 = value_count;
                value_count = value_count + 1;
                values[fresh14 as usize] = current_value;
                current_value = 0 as ::core::ffi::c_int;
                in_number = false_0 != 0;
            }
        } else if c as ::core::ffi::c_int == '.' as i32 {
            paren += 1;
            while paren < pos + color_len
                && *text.offset(paren as isize) as ::core::ffi::c_int >= '0' as i32
                && *text.offset(paren as isize) as ::core::ffi::c_int <= '9' as i32
            {
                paren += 1;
            }
            continue;
        }
        paren += 1;
    }
    if in_number as ::core::ffi::c_int != 0 && value_count < 4 as ::core::ffi::c_int {
        if has_percent {
            current_value = current_value * 255 as ::core::ffi::c_int
                / 100 as ::core::ffi::c_int;
        }
        let fresh15 = value_count;
        value_count = value_count + 1;
        values[fresh15 as usize] = current_value;
    }
    if value_count >= 3 as ::core::ffi::c_int {
        *r = if values[0 as ::core::ffi::c_int as usize] > 255 as ::core::ffi::c_int {
            255 as ::core::ffi::c_int
        } else {
            values[0 as ::core::ffi::c_int as usize]
        };
        *g = if values[1 as ::core::ffi::c_int as usize] > 255 as ::core::ffi::c_int {
            255 as ::core::ffi::c_int
        } else {
            values[1 as ::core::ffi::c_int as usize]
        };
        *b = if values[2 as ::core::ffi::c_int as usize] > 255 as ::core::ffi::c_int {
            255 as ::core::ffi::c_int
        } else {
            values[2 as ::core::ffi::c_int as usize]
        };
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[inline]
unsafe extern "C" fn normalize_state(mut state: *mut HighlightState) {
    if state.is_null() {
        return;
    }
    if (*state).depth < 0 as ::core::ffi::c_int || (*state).depth > HL_STATE_STACK_MAX {
        (*state).depth = 0 as ::core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C" fn current_state(mut state: *const HighlightState) -> StateID {
    if state.is_null() || (*state).depth <= 0 as ::core::ffi::c_int
        || (*state).depth > HL_STATE_STACK_MAX
    {
        return HS_NORMAL;
    }
    return (*state).stack[((*state).depth - 1 as ::core::ffi::c_int) as usize].state;
}
#[inline]
unsafe extern "C" fn state_top(
    mut state: *mut HighlightState,
) -> *mut HighlightStackEntry {
    if state.is_null() || (*state).depth <= 0 as ::core::ffi::c_int
        || (*state).depth > HL_STATE_STACK_MAX
    {
        return ::core::ptr::null_mut::<HighlightStackEntry>();
    }
    return (&raw mut (*state).stack as *mut HighlightStackEntry)
        .offset(((*state).depth - 1 as ::core::ffi::c_int) as isize)
        as *mut HighlightStackEntry;
}
#[inline]
unsafe extern "C" fn pop_state(mut state: *mut HighlightState) {
    if !state.is_null() && (*state).depth > 0 as ::core::ffi::c_int {
        (*state).depth -= 1;
    }
}
unsafe extern "C" fn push_block_comment(
    mut state: *mut HighlightState,
    mut idx: ::core::ffi::c_int,
) -> bool {
    if state.is_null() || idx < 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if (*state).depth >= HL_STATE_STACK_MAX {
        return false_0 != 0;
    }
    (*state).stack[(*state).depth as usize].state = HS_BLOCK_COMMENT;
    (*state).stack[(*state).depth as usize].sub_id = idx;
    (*state).stack[(*state).depth as usize].string_delim = 0 as ::core::ffi::c_char;
    (*state).depth += 1;
    return true_0 != 0;
}
unsafe extern "C" fn push_string_state(
    mut state: *mut HighlightState,
    mut triple: bool,
    mut delim: ::core::ffi::c_char,
) -> bool {
    if state.is_null() {
        return false_0 != 0;
    }
    if (*state).depth >= HL_STATE_STACK_MAX {
        return false_0 != 0;
    }
    (*state).stack[(*state).depth as usize].state = (if triple as ::core::ffi::c_int != 0
    {
        HS_TRIPLE_STRING as ::core::ffi::c_int
    } else {
        HS_STRING as ::core::ffi::c_int
    }) as StateID;
    (*state).stack[(*state).depth as usize].sub_id = 0 as ::core::ffi::c_int;
    (*state).stack[(*state).depth as usize].string_delim = delim;
    (*state).depth += 1;
    return true_0 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn highlight_line(
    mut text: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
    mut start: HighlightState,
    mut profile: *const HighlightProfile,
    mut out: *mut SpanVec,
    mut end: *mut HighlightState,
) {
    if !out.is_null() {
        (*out).count = 0 as ::core::ffi::c_int;
        (*out).heap_spans = ::core::ptr::null_mut::<Span>();
        (*out).capacity = 0 as ::core::ffi::c_int;
    }
    *end = start;
    if text.is_null() {
        return;
    }
    if len < 0 as ::core::ffi::c_int && !text.is_null() {
        len = strlen(text) as ::core::ffi::c_int;
    }
    if len == 0 as ::core::ffi::c_int {
        return;
    }
    let mut is_md: bool = is_markdown_profile(profile);
    let mut is_html: bool = is_html_profile(profile);
    if profile.is_null() {
        let mut pos: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while pos < len {
            let mut color_len: ::core::ffi::c_int = is_hex_color(text, len, pos);
            if color_len == 0 as ::core::ffi::c_int {
                color_len = is_rgb_color(text, len, pos);
            }
            if color_len == 0 as ::core::ffi::c_int {
                color_len = is_hsl_color(text, len, pos);
            }
            if color_len > 0 as ::core::ffi::c_int {
                if !out.is_null() {
                    add_span(out, pos, pos + color_len, HL_NUMBER);
                }
                pos += color_len;
            } else {
                pos += 1;
            }
        }
        return;
    }
    let mut state: HighlightState = start;
    normalize_state(&raw mut state);
    let mut pos_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while pos_0 < len {
        let mut c: ::core::ffi::c_uchar = *text.offset(pos_0 as isize)
            as ::core::ffi::c_uchar;
        let mut active: StateID = current_state(&raw mut state);
        if active as ::core::ffi::c_uint
            == HS_NORMAL as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if is_control(c) {
                if !out.is_null() {
                    add_span(out, pos_0, pos_0 + 1 as ::core::ffi::c_int, HL_CONTROL);
                }
                pos_0 += 1;
            } else {
                if *(*__ctype_b_loc()).offset(c as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                        as ::core::ffi::c_int != 0
                {
                    let mut trailing: bool = true_0 != 0;
                    let mut i: ::core::ffi::c_int = pos_0;
                    while i < len {
                        if *(*__ctype_b_loc())
                            .offset(
                                *text.offset(i as isize) as ::core::ffi::c_uchar
                                    as ::core::ffi::c_int as isize,
                            ) as ::core::ffi::c_int
                            & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                                as ::core::ffi::c_int == 0
                        {
                            trailing = false_0 != 0;
                            break;
                        } else {
                            i += 1;
                        }
                    }
                    if trailing {
                        if !out.is_null() {
                            add_span(out, pos_0, len, HL_CONTROL);
                        }
                        pos_0 = len;
                        continue;
                    }
                }
                let mut color_len_0: ::core::ffi::c_int = is_hex_color(text, len, pos_0);
                if color_len_0 == 0 as ::core::ffi::c_int {
                    color_len_0 = is_rgb_color(text, len, pos_0);
                }
                if color_len_0 == 0 as ::core::ffi::c_int {
                    color_len_0 = is_hsl_color(text, len, pos_0);
                }
                if color_len_0 > 0 as ::core::ffi::c_int {
                    if !out.is_null() {
                        add_span(out, pos_0, pos_0 + color_len_0, HL_NUMBER);
                    }
                    pos_0 += color_len_0;
                } else {
                    if is_md as ::core::ffi::c_int != 0
                        && (pos_0 == 0 as ::core::ffi::c_int
                            || pos_0 < 3 as ::core::ffi::c_int
                                && *(*__ctype_b_loc())
                                    .offset(
                                        *text.offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_uchar as ::core::ffi::c_int as isize,
                                    ) as ::core::ffi::c_int
                                    & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                                        as ::core::ffi::c_int != 0)
                    {
                        let mut h: ::core::ffi::c_int = pos_0;
                        while h < len
                            && *text.offset(h as isize) as ::core::ffi::c_int
                                == '#' as i32
                        {
                            h += 1;
                        }
                        if h > pos_0 && h <= pos_0 + 6 as ::core::ffi::c_int && h < len
                            && *(*__ctype_b_loc())
                                .offset(
                                    *text.offset(h as isize) as ::core::ffi::c_uchar
                                        as ::core::ffi::c_int as isize,
                                ) as ::core::ffi::c_int
                                & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                                    as ::core::ffi::c_int != 0
                        {
                            if !out.is_null() {
                                add_span(out, pos_0, h, HL_HEADER);
                            }
                            pos_0 = len;
                            continue;
                        }
                    }
                    if is_md {
                        if pos_0 + 4 as ::core::ffi::c_int <= len
                            && (*text.offset(pos_0 as isize) as ::core::ffi::c_int
                                == '*' as i32
                                && *text.offset((pos_0 + 1 as ::core::ffi::c_int) as isize)
                                    as ::core::ffi::c_int == '*' as i32
                                || *text.offset(pos_0 as isize) as ::core::ffi::c_int
                                    == '_' as i32
                                    && *text.offset((pos_0 + 1 as ::core::ffi::c_int) as isize)
                                        as ::core::ffi::c_int == '_' as i32)
                        {
                            let mut delim: [::core::ffi::c_char; 3] = [
                                *text.offset(pos_0 as isize),
                                *text.offset(pos_0 as isize),
                                0 as ::core::ffi::c_int as ::core::ffi::c_char,
                            ];
                            let mut end_pos: ::core::ffi::c_int = find_md_closing(
                                text,
                                len,
                                pos_0 + 2 as ::core::ffi::c_int,
                                &raw mut delim as *mut ::core::ffi::c_char,
                                2 as ::core::ffi::c_int,
                            );
                            if end_pos > 0 as ::core::ffi::c_int {
                                if !out.is_null() {
                                    add_span(out, pos_0, end_pos, HL_MD_BOLD);
                                }
                                pos_0 = end_pos;
                                continue;
                            }
                        }
                        if pos_0 + 2 as ::core::ffi::c_int <= len
                            && (*text.offset(pos_0 as isize) as ::core::ffi::c_int
                                == '*' as i32
                                || *text.offset(pos_0 as isize) as ::core::ffi::c_int
                                    == '_' as i32)
                        {
                            if pos_0 + 1 as ::core::ffi::c_int >= len
                                || *text.offset((pos_0 + 1 as ::core::ffi::c_int) as isize)
                                    as ::core::ffi::c_int
                                    != *text.offset(pos_0 as isize) as ::core::ffi::c_int
                            {
                                let mut delim_0: [::core::ffi::c_char; 2] = [
                                    *text.offset(pos_0 as isize),
                                    0 as ::core::ffi::c_int as ::core::ffi::c_char,
                                ];
                                let mut end_pos_0: ::core::ffi::c_int = find_md_closing(
                                    text,
                                    len,
                                    pos_0 + 1 as ::core::ffi::c_int,
                                    &raw mut delim_0 as *mut ::core::ffi::c_char,
                                    1 as ::core::ffi::c_int,
                                );
                                if end_pos_0 > 0 as ::core::ffi::c_int {
                                    if !out.is_null() {
                                        add_span(out, pos_0, end_pos_0, HL_MD_ITALIC);
                                    }
                                    pos_0 = end_pos_0;
                                    continue;
                                }
                            }
                        }
                    }
                    if is_html as ::core::ffi::c_int != 0
                        || is_md as ::core::ffi::c_int != 0
                    {
                        if pos_0 + 3 as ::core::ffi::c_int <= len
                            && strncasecmp(
                                text.offset(pos_0 as isize),
                                b"<u>\0" as *const u8 as *const ::core::ffi::c_char,
                                3 as size_t,
                            ) == 0 as ::core::ffi::c_int
                        {
                            let mut search: ::core::ffi::c_int = pos_0
                                + 3 as ::core::ffi::c_int;
                            while search + 4 as ::core::ffi::c_int <= len {
                                if strncasecmp(
                                    text.offset(search as isize),
                                    b"</u>\0" as *const u8 as *const ::core::ffi::c_char,
                                    4 as size_t,
                                ) == 0 as ::core::ffi::c_int
                                {
                                    if !out.is_null() {
                                        add_span(
                                            out,
                                            pos_0,
                                            search + 4 as ::core::ffi::c_int,
                                            HL_MD_UNDERLINE,
                                        );
                                    }
                                    pos_0 = search + 4 as ::core::ffi::c_int;
                                    break;
                                } else {
                                    search += 1;
                                }
                            }
                            if pos_0 == search + 4 as ::core::ffi::c_int {
                                continue;
                            }
                        }
                        if pos_0 + 3 as ::core::ffi::c_int <= len
                            && strncasecmp(
                                text.offset(pos_0 as isize),
                                b"<b>\0" as *const u8 as *const ::core::ffi::c_char,
                                3 as size_t,
                            ) == 0 as ::core::ffi::c_int
                        {
                            let mut search_0: ::core::ffi::c_int = pos_0
                                + 3 as ::core::ffi::c_int;
                            while search_0 + 4 as ::core::ffi::c_int <= len {
                                if strncasecmp(
                                    text.offset(search_0 as isize),
                                    b"</b>\0" as *const u8 as *const ::core::ffi::c_char,
                                    4 as size_t,
                                ) == 0 as ::core::ffi::c_int
                                {
                                    if !out.is_null() {
                                        add_span(
                                            out,
                                            pos_0,
                                            search_0 + 4 as ::core::ffi::c_int,
                                            HL_MD_BOLD,
                                        );
                                    }
                                    pos_0 = search_0 + 4 as ::core::ffi::c_int;
                                    break;
                                } else {
                                    search_0 += 1;
                                }
                            }
                            if pos_0 == search_0 + 4 as ::core::ffi::c_int {
                                continue;
                            }
                        }
                        if pos_0 + 3 as ::core::ffi::c_int <= len
                            && strncasecmp(
                                text.offset(pos_0 as isize),
                                b"<i>\0" as *const u8 as *const ::core::ffi::c_char,
                                3 as size_t,
                            ) == 0 as ::core::ffi::c_int
                        {
                            let mut search_1: ::core::ffi::c_int = pos_0
                                + 3 as ::core::ffi::c_int;
                            while search_1 + 4 as ::core::ffi::c_int <= len {
                                if strncasecmp(
                                    text.offset(search_1 as isize),
                                    b"</i>\0" as *const u8 as *const ::core::ffi::c_char,
                                    4 as size_t,
                                ) == 0 as ::core::ffi::c_int
                                {
                                    if !out.is_null() {
                                        add_span(
                                            out,
                                            pos_0,
                                            search_1 + 4 as ::core::ffi::c_int,
                                            HL_MD_ITALIC,
                                        );
                                    }
                                    pos_0 = search_1 + 4 as ::core::ffi::c_int;
                                    break;
                                } else {
                                    search_1 += 1;
                                }
                            }
                            if pos_0 == search_1 + 4 as ::core::ffi::c_int {
                                continue;
                            }
                        }
                    }
                    let mut matched_block: bool = false_0 != 0;
                    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while i_0 < (*profile).block_comment_count {
                        let mut start_tok: *const ::core::ffi::c_char = &raw const (*(&raw const (*profile)
                            .block_comments as *const BlockCommentPair)
                            .offset(i_0 as isize))
                            .start as *const ::core::ffi::c_char;
                        let mut start_len: ::core::ffi::c_int = strlen(start_tok)
                            as ::core::ffi::c_int;
                        if start_len != 0
                            && starts_with(text.offset(pos_0 as isize), start_tok)
                                as ::core::ffi::c_int != 0
                        {
                            let mut start_pos: ::core::ffi::c_int = pos_0;
                            if !out.is_null() {
                                add_span(out, start_pos, start_pos + start_len, HL_COMMENT);
                            }
                            pos_0 += start_len;
                            if !push_block_comment(&raw mut state, i_0) {
                                if !out.is_null() {
                                    add_span(out, pos_0, len, HL_COMMENT);
                                }
                                pos_0 = len;
                            }
                            matched_block = true_0 != 0;
                            break;
                        } else {
                            i_0 += 1;
                        }
                    }
                    if matched_block {
                        continue;
                    }
                    let mut matched_line: bool = false_0 != 0;
                    let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while i_1 < (*profile).line_comment_count {
                        if starts_with(
                            text.offset(pos_0 as isize),
                            &raw const *(&raw const (*profile).line_comments
                                as *const [::core::ffi::c_char; 64])
                                .offset(i_1 as isize) as *const ::core::ffi::c_char,
                        ) {
                            if !out.is_null() {
                                add_span(out, pos_0, len, HL_COMMENT);
                            }
                            pos_0 = len;
                            matched_line = true_0 != 0;
                            break;
                        } else {
                            i_1 += 1;
                        }
                    }
                    if matched_line {
                        continue;
                    }
                    if c as ::core::ffi::c_int == '#' as i32
                        && (pos_0 == 0 as ::core::ffi::c_int
                            || *(*__ctype_b_loc())
                                .offset(
                                    *text.offset((pos_0 - 1 as ::core::ffi::c_int) as isize)
                                        as ::core::ffi::c_uchar as ::core::ffi::c_int as isize,
                                ) as ::core::ffi::c_int
                                & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                                    as ::core::ffi::c_int != 0)
                    {
                        let mut search_2: ::core::ffi::c_int = pos_0
                            + 1 as ::core::ffi::c_int;
                        while search_2 < len
                            && (*(*__ctype_b_loc())
                                .offset(
                                    *text.offset(search_2 as isize) as ::core::ffi::c_uchar
                                        as ::core::ffi::c_int as isize,
                                ) as ::core::ffi::c_int
                                & _ISalnum as ::core::ffi::c_int as ::core::ffi::c_ushort
                                    as ::core::ffi::c_int != 0
                                || *text.offset(search_2 as isize) as ::core::ffi::c_int
                                    == '_' as i32)
                        {
                            search_2 += 1;
                        }
                        if !out.is_null() {
                            add_span(out, pos_0, search_2, HL_PREPROC);
                        }
                        pos_0 = search_2;
                    } else if (*profile).enable_triple_quotes as ::core::ffi::c_int != 0
                        && strncmp(
                            text.offset(pos_0 as isize),
                            b"\"\"\"\0" as *const u8 as *const ::core::ffi::c_char,
                            3 as size_t,
                        ) == 0 as ::core::ffi::c_int
                    {
                        if push_string_state(
                            &raw mut state,
                            true_0 != 0,
                            '"' as i32 as ::core::ffi::c_char,
                        ) {
                            if !out.is_null() {
                                add_span(
                                    out,
                                    pos_0,
                                    pos_0 + 3 as ::core::ffi::c_int,
                                    HL_STRING,
                                );
                            }
                        } else if !out.is_null() {
                            add_span(
                                out,
                                pos_0,
                                pos_0 + 3 as ::core::ffi::c_int,
                                HL_STRING,
                            );
                        }
                        pos_0 += 3 as ::core::ffi::c_int;
                    } else {
                        let mut delim_ptr: *mut ::core::ffi::c_char = strchr(
                            &raw const (*profile).string_delims
                                as *const ::core::ffi::c_char,
                            c as ::core::ffi::c_int,
                        );
                        if !delim_ptr.is_null() && *delim_ptr as ::core::ffi::c_int != 0
                        {
                            if push_string_state(
                                &raw mut state,
                                false_0 != 0,
                                c as ::core::ffi::c_char,
                            ) {
                                if !out.is_null() {
                                    add_span(
                                        out,
                                        pos_0,
                                        pos_0 + 1 as ::core::ffi::c_int,
                                        HL_STRING,
                                    );
                                }
                            } else if !out.is_null() {
                                add_span(
                                    out,
                                    pos_0,
                                    pos_0 + 1 as ::core::ffi::c_int,
                                    HL_STRING,
                                );
                            }
                            pos_0 += 1;
                        } else if (*profile).enable_number_highlight
                            as ::core::ffi::c_int != 0
                            && (*(*__ctype_b_loc())
                                .offset(c as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                                    as ::core::ffi::c_int != 0
                                || c as ::core::ffi::c_int == '.' as i32
                                    && *(*__ctype_b_loc())
                                        .offset(
                                            *text.offset((pos_0 + 1 as ::core::ffi::c_int) as isize)
                                                as ::core::ffi::c_uchar as ::core::ffi::c_int as isize,
                                        ) as ::core::ffi::c_int
                                        & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                                            as ::core::ffi::c_int != 0)
                        {
                            let mut search_3: ::core::ffi::c_int = pos_0;
                            if c as ::core::ffi::c_int == '0' as i32
                                && (pos_0 + 1 as ::core::ffi::c_int) < len
                            {
                                let mut next: ::core::ffi::c_char = *text
                                    .offset((pos_0 + 1 as ::core::ffi::c_int) as isize);
                                if next as ::core::ffi::c_int == 'x' as i32
                                    || next as ::core::ffi::c_int == 'X' as i32
                                {
                                    search_3 += 2 as ::core::ffi::c_int;
                                    while search_3 < len
                                        && *(*__ctype_b_loc())
                                            .offset(
                                                *text.offset(search_3 as isize) as ::core::ffi::c_uchar
                                                    as ::core::ffi::c_int as isize,
                                            ) as ::core::ffi::c_int
                                            & _ISxdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                                                as ::core::ffi::c_int != 0
                                    {
                                        search_3 += 1;
                                    }
                                } else if next as ::core::ffi::c_int == 'b' as i32
                                    || next as ::core::ffi::c_int == 'B' as i32
                                {
                                    search_3 += 2 as ::core::ffi::c_int;
                                    while search_3 < len
                                        && (*text.offset(search_3 as isize) as ::core::ffi::c_int
                                            == '0' as i32
                                            || *text.offset(search_3 as isize) as ::core::ffi::c_int
                                                == '1' as i32)
                                    {
                                        search_3 += 1;
                                    }
                                } else if *(*__ctype_b_loc())
                                    .offset(
                                        next as ::core::ffi::c_uchar as ::core::ffi::c_int as isize,
                                    ) as ::core::ffi::c_int
                                    & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                                        as ::core::ffi::c_int != 0
                                {
                                    search_3 += 1 as ::core::ffi::c_int;
                                    while search_3 < len
                                        && *(*__ctype_b_loc())
                                            .offset(
                                                *text.offset(search_3 as isize) as ::core::ffi::c_uchar
                                                    as ::core::ffi::c_int as isize,
                                            ) as ::core::ffi::c_int
                                            & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                                                as ::core::ffi::c_int != 0
                                    {
                                        search_3 += 1;
                                    }
                                } else {
                                    search_3 += 1;
                                }
                            } else {
                                while search_3 < len
                                    && (*(*__ctype_b_loc())
                                        .offset(
                                            *text.offset(search_3 as isize) as ::core::ffi::c_uchar
                                                as ::core::ffi::c_int as isize,
                                        ) as ::core::ffi::c_int
                                        & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                                            as ::core::ffi::c_int != 0
                                        || *text.offset(search_3 as isize) as ::core::ffi::c_int
                                            == '.' as i32
                                        || *text.offset(search_3 as isize) as ::core::ffi::c_int
                                            == 'e' as i32
                                        || *text.offset(search_3 as isize) as ::core::ffi::c_int
                                            == 'E' as i32)
                                {
                                    if (*text.offset(search_3 as isize) as ::core::ffi::c_int
                                        == 'e' as i32
                                        || *text.offset(search_3 as isize) as ::core::ffi::c_int
                                            == 'E' as i32)
                                        && (*text
                                            .offset((search_3 + 1 as ::core::ffi::c_int) as isize)
                                            as ::core::ffi::c_int == '+' as i32
                                            || *text
                                                .offset((search_3 + 1 as ::core::ffi::c_int) as isize)
                                                as ::core::ffi::c_int == '-' as i32)
                                    {
                                        search_3 += 1;
                                    }
                                    search_3 += 1;
                                }
                            }
                            while search_3 < len
                                && !strchr(
                                        b"uUlLfF\0" as *const u8 as *const ::core::ffi::c_char,
                                        *text.offset(search_3 as isize) as ::core::ffi::c_int,
                                    )
                                    .is_null()
                            {
                                search_3 += 1;
                            }
                            if !out.is_null() {
                                add_span(out, pos_0, search_3, HL_NUMBER);
                            }
                            pos_0 = search_3;
                        } else if (*profile).enable_bracket_highlight
                            as ::core::ffi::c_int != 0
                            && (c as ::core::ffi::c_int == '?' as i32
                                || c as ::core::ffi::c_int == ':' as i32)
                        {
                            if !out.is_null() {
                                add_span(
                                    out,
                                    pos_0,
                                    pos_0 + 1 as ::core::ffi::c_int,
                                    HL_TERNARY,
                                );
                            }
                            pos_0 += 1;
                        } else {
                            if profile_supports_at_annotations(profile)
                                as ::core::ffi::c_int != 0
                                && c as ::core::ffi::c_int == '@' as i32
                            {
                                let mut start_0: ::core::ffi::c_int = pos_0
                                    + 1 as ::core::ffi::c_int;
                                let mut end_0: ::core::ffi::c_int = start_0;
                                while end_0 < len
                                    && (*(*__ctype_b_loc())
                                        .offset(
                                            *text.offset(end_0 as isize) as ::core::ffi::c_uchar
                                                as ::core::ffi::c_int as isize,
                                        ) as ::core::ffi::c_int
                                        & _ISalnum as ::core::ffi::c_int as ::core::ffi::c_ushort
                                            as ::core::ffi::c_int != 0
                                        || *text.offset(end_0 as isize) as ::core::ffi::c_int
                                            == '_' as i32
                                        || *text.offset(end_0 as isize) as ::core::ffi::c_int
                                            == '.' as i32)
                                {
                                    end_0 += 1;
                                }
                                if end_0 > start_0 {
                                    if !out.is_null() {
                                        add_span(out, pos_0, end_0, HL_PREPROC);
                                    }
                                    pos_0 = end_0;
                                    continue;
                                }
                            }
                            if (*profile).enable_bracket_highlight as ::core::ffi::c_int
                                != 0
                                && is_punct(c as ::core::ffi::c_char) as ::core::ffi::c_int
                                    != 0
                            {
                                if !out.is_null() {
                                    add_span(
                                        out,
                                        pos_0,
                                        pos_0 + 1 as ::core::ffi::c_int,
                                        HL_BRACKET,
                                    );
                                }
                                pos_0 += 1;
                            } else if (*profile).enable_bracket_highlight
                                as ::core::ffi::c_int != 0
                                && is_operator(c as ::core::ffi::c_char)
                                    as ::core::ffi::c_int != 0
                            {
                                if !out.is_null() {
                                    add_span(
                                        out,
                                        pos_0,
                                        pos_0 + 1 as ::core::ffi::c_int,
                                        HL_OPERATOR,
                                    );
                                }
                                pos_0 += 1;
                            } else {
                                let mut next_stop: ::core::ffi::c_int = pos_0;
                                while next_stop < len
                                    && (*(*__ctype_b_loc())
                                        .offset(
                                            *text.offset(next_stop as isize) as ::core::ffi::c_uchar
                                                as ::core::ffi::c_int as isize,
                                        ) as ::core::ffi::c_int
                                        & _ISalnum as ::core::ffi::c_int as ::core::ffi::c_ushort
                                            as ::core::ffi::c_int != 0
                                        || *text.offset(next_stop as isize) as ::core::ffi::c_int
                                            == '_' as i32)
                                {
                                    next_stop += 1;
                                }
                                if next_stop > pos_0 {
                                    if !out.is_null() {
                                        let mut word: [::core::ffi::c_char; 64] = [0; 64];
                                        let mut word_len: ::core::ffi::c_int = next_stop - pos_0;
                                        let mut style: HighlightStyleID = HL_NORMAL;
                                        if word_len < MAX_TOKEN_LEN {
                                            memcpy(
                                                &raw mut word as *mut ::core::ffi::c_char
                                                    as *mut ::core::ffi::c_void,
                                                text.offset(pos_0 as isize) as *const ::core::ffi::c_void,
                                                word_len as size_t,
                                            );
                                            word[word_len as usize] = 0 as ::core::ffi::c_char;
                                            let mut found: bool = false_0 != 0;
                                            let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                            while i_2 < (*profile).return_keyword_count {
                                                if strcmp(
                                                    &raw mut word as *mut ::core::ffi::c_char,
                                                    &raw const *(&raw const (*profile).return_keywords
                                                        as *const [::core::ffi::c_char; 64])
                                                        .offset(i_2 as isize) as *const ::core::ffi::c_char,
                                                ) == 0 as ::core::ffi::c_int
                                                {
                                                    style = HL_RETURN;
                                                    found = true_0 != 0;
                                                    break;
                                                } else {
                                                    i_2 += 1;
                                                }
                                            }
                                            if !found {
                                                let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                                while i_3 < (*profile).flow_keyword_count {
                                                    if strcmp(
                                                        &raw mut word as *mut ::core::ffi::c_char,
                                                        &raw const *(&raw const (*profile).flow_keywords
                                                            as *const [::core::ffi::c_char; 64])
                                                            .offset(i_3 as isize) as *const ::core::ffi::c_char,
                                                    ) == 0 as ::core::ffi::c_int
                                                    {
                                                        style = HL_FLOW;
                                                        found = true_0 != 0;
                                                        break;
                                                    } else {
                                                        i_3 += 1;
                                                    }
                                                }
                                            }
                                            if !found {
                                                let mut i_4: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                                while i_4 < (*profile).preproc_keyword_count {
                                                    if strcmp(
                                                        &raw mut word as *mut ::core::ffi::c_char,
                                                        &raw const *(&raw const (*profile).preproc_keywords
                                                            as *const [::core::ffi::c_char; 64])
                                                            .offset(i_4 as isize) as *const ::core::ffi::c_char,
                                                    ) == 0 as ::core::ffi::c_int
                                                    {
                                                        style = HL_PREPROC;
                                                        found = true_0 != 0;
                                                        break;
                                                    } else {
                                                        i_4 += 1;
                                                    }
                                                }
                                            }
                                            if !found {
                                                let mut i_5: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                                while i_5 < (*profile).type_keyword_count {
                                                    if strcmp(
                                                        &raw mut word as *mut ::core::ffi::c_char,
                                                        &raw const *(&raw const (*profile).type_keywords
                                                            as *const [::core::ffi::c_char; 64])
                                                            .offset(i_5 as isize) as *const ::core::ffi::c_char,
                                                    ) == 0 as ::core::ffi::c_int
                                                    {
                                                        style = HL_TYPE;
                                                        found = true_0 != 0;
                                                        break;
                                                    } else {
                                                        i_5 += 1;
                                                    }
                                                }
                                            }
                                            if !found {
                                                let mut i_6: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                                while i_6 < (*profile).keyword_count {
                                                    if strcmp(
                                                        &raw mut word as *mut ::core::ffi::c_char,
                                                        &raw const *(&raw const (*profile).keywords
                                                            as *const [::core::ffi::c_char; 64])
                                                            .offset(i_6 as isize) as *const ::core::ffi::c_char,
                                                    ) == 0 as ::core::ffi::c_int
                                                    {
                                                        style = HL_KEYWORD;
                                                        found = true_0 != 0;
                                                        break;
                                                    } else {
                                                        i_6 += 1;
                                                    }
                                                }
                                            }
                                            if !found {
                                                let mut s: ::core::ffi::c_int = next_stop;
                                                while s < len
                                                    && *(*__ctype_b_loc())
                                                        .offset(
                                                            *text.offset(s as isize) as ::core::ffi::c_uchar
                                                                as ::core::ffi::c_int as isize,
                                                        ) as ::core::ffi::c_int
                                                        & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort
                                                            as ::core::ffi::c_int != 0
                                                {
                                                    s += 1;
                                                }
                                                if s < len
                                                    && *text.offset(s as isize) as ::core::ffi::c_int
                                                        == '(' as i32
                                                {
                                                    style = HL_FUNCTION;
                                                }
                                            }
                                        }
                                        add_span(out, pos_0, next_stop, style);
                                    }
                                    pos_0 = next_stop;
                                } else {
                                    if !out.is_null() {
                                        add_span(
                                            out,
                                            pos_0,
                                            pos_0 + 1 as ::core::ffi::c_int,
                                            HL_NORMAL,
                                        );
                                    }
                                    pos_0 += 1;
                                }
                            }
                        }
                    }
                }
            }
        } else if active as ::core::ffi::c_uint
            == HS_BLOCK_COMMENT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut frame: *mut HighlightStackEntry = state_top(&raw mut state);
            if frame.is_null() {
                pop_state(&raw mut state);
            } else {
                let mut pair: *const BlockCommentPair = (&raw const (*profile)
                    .block_comments as *const BlockCommentPair)
                    .offset((*frame).sub_id as isize) as *const BlockCommentPair;
                let mut end_str: *const ::core::ffi::c_char = &raw const (*pair).end
                    as *const ::core::ffi::c_char;
                let mut end_len: ::core::ffi::c_int = strlen(end_str)
                    as ::core::ffi::c_int;
                let mut chunk_start: ::core::ffi::c_int = pos_0;
                let mut close_pos: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
                if end_len > 0 as ::core::ffi::c_int {
                    let mut scan: ::core::ffi::c_int = pos_0;
                    while scan <= len - end_len {
                        if strncmp(
                            text.offset(scan as isize),
                            end_str,
                            end_len as size_t,
                        ) == 0 as ::core::ffi::c_int
                        {
                            close_pos = scan;
                            break;
                        } else {
                            scan += 1;
                        }
                    }
                }
                if close_pos >= 0 as ::core::ffi::c_int {
                    if !out.is_null() && close_pos > chunk_start {
                        add_span(out, chunk_start, close_pos, HL_COMMENT);
                    }
                    if !out.is_null() {
                        add_span(out, close_pos, close_pos + end_len, HL_COMMENT);
                    }
                    pos_0 = close_pos + end_len;
                    pop_state(&raw mut state);
                } else {
                    if !out.is_null() {
                        add_span(out, chunk_start, len, HL_COMMENT);
                    }
                    pos_0 = len;
                }
            }
        } else {
            if !(active as ::core::ffi::c_uint
                == HS_STRING as ::core::ffi::c_int as ::core::ffi::c_uint
                || active as ::core::ffi::c_uint
                    == HS_TRIPLE_STRING as ::core::ffi::c_int as ::core::ffi::c_uint)
            {
                continue;
            }
            let mut frame_0: *mut HighlightStackEntry = state_top(&raw mut state);
            if frame_0.is_null() {
                pop_state(&raw mut state);
            } else {
                let mut delim_1: ::core::ffi::c_char = (*frame_0).string_delim;
                let mut is_triple: bool = active as ::core::ffi::c_uint
                    == HS_TRIPLE_STRING as ::core::ffi::c_int as ::core::ffi::c_uint;
                if *text.offset(pos_0 as isize) as ::core::ffi::c_int == '\\' as i32 {
                    let mut esc_len: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
                    if (pos_0 + 1 as ::core::ffi::c_int) < len {
                        if *text.offset((pos_0 + 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int == 'x' as i32
                        {
                            esc_len = 2 as ::core::ffi::c_int;
                            while pos_0 + esc_len < len
                                && *(*__ctype_b_loc())
                                    .offset(
                                        *text.offset((pos_0 + esc_len) as isize)
                                            as ::core::ffi::c_uchar as ::core::ffi::c_int as isize,
                                    ) as ::core::ffi::c_int
                                    & _ISxdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                                        as ::core::ffi::c_int != 0
                                && esc_len < 4 as ::core::ffi::c_int
                            {
                                esc_len += 1;
                            }
                        } else if *(*__ctype_b_loc())
                            .offset(
                                *text.offset((pos_0 + 1 as ::core::ffi::c_int) as isize)
                                    as ::core::ffi::c_uchar as ::core::ffi::c_int as isize,
                            ) as ::core::ffi::c_int
                            & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                                as ::core::ffi::c_int != 0
                        {
                            esc_len = 2 as ::core::ffi::c_int;
                            while pos_0 + esc_len < len
                                && *(*__ctype_b_loc())
                                    .offset(
                                        *text.offset((pos_0 + esc_len) as isize)
                                            as ::core::ffi::c_uchar as ::core::ffi::c_int as isize,
                                    ) as ::core::ffi::c_int
                                    & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort
                                        as ::core::ffi::c_int != 0
                                && esc_len < 4 as ::core::ffi::c_int
                            {
                                esc_len += 1;
                            }
                        }
                    }
                    if !out.is_null() {
                        add_span(out, pos_0, pos_0 + esc_len, HL_ESCAPE);
                    }
                    pos_0 += esc_len;
                } else {
                    if is_triple {
                        if pos_0 + 3 as ::core::ffi::c_int <= len
                            && *text.offset(pos_0 as isize) as ::core::ffi::c_int
                                == delim_1 as ::core::ffi::c_int
                            && *text.offset((pos_0 + 1 as ::core::ffi::c_int) as isize)
                                as ::core::ffi::c_int == delim_1 as ::core::ffi::c_int
                            && *text.offset((pos_0 + 2 as ::core::ffi::c_int) as isize)
                                as ::core::ffi::c_int == delim_1 as ::core::ffi::c_int
                        {
                            if !out.is_null() {
                                add_span(
                                    out,
                                    pos_0,
                                    pos_0 + 3 as ::core::ffi::c_int,
                                    HL_STRING,
                                );
                            }
                            pos_0 += 3 as ::core::ffi::c_int;
                            pop_state(&raw mut state);
                            continue;
                        }
                    } else if *text.offset(pos_0 as isize) as ::core::ffi::c_int
                        == delim_1 as ::core::ffi::c_int
                    {
                        if !out.is_null() {
                            add_span(
                                out,
                                pos_0,
                                pos_0 + 1 as ::core::ffi::c_int,
                                HL_STRING,
                            );
                        }
                        pos_0 += 1;
                        pop_state(&raw mut state);
                        continue;
                    }
                    if !out.is_null() {
                        add_span(out, pos_0, pos_0 + 1 as ::core::ffi::c_int, HL_STRING);
                    }
                    pos_0 += 1;
                }
            }
        }
    }
    *end = state;
}
#[no_mangle]
pub unsafe extern "C" fn highlight_find_colors(
    mut text: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
    mut colors: *mut ColorInfo,
    mut max_colors: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if text.is_null() || len <= 0 as ::core::ffi::c_int || colors.is_null()
        || max_colors <= 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pos: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while pos < len && count < max_colors {
        let mut color_len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut r: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut g: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut b: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        color_len = is_hex_color(text, len, pos);
        if color_len > 0 as ::core::ffi::c_int {
            if parse_hex_color(
                text,
                pos,
                color_len,
                &raw mut r,
                &raw mut g,
                &raw mut b,
            ) {
                (*colors.offset(count as isize)).start = pos;
                (*colors.offset(count as isize)).end = pos + color_len;
                (*colors.offset(count as isize)).r = r;
                (*colors.offset(count as isize)).g = g;
                (*colors.offset(count as isize)).b = b;
                count += 1;
            }
            pos += color_len;
        } else {
            color_len = is_rgb_color(text, len, pos);
            if color_len > 0 as ::core::ffi::c_int {
                if parse_rgb_color(
                    text,
                    pos,
                    color_len,
                    &raw mut r,
                    &raw mut g,
                    &raw mut b,
                ) {
                    (*colors.offset(count as isize)).start = pos;
                    (*colors.offset(count as isize)).end = pos + color_len;
                    (*colors.offset(count as isize)).r = r;
                    (*colors.offset(count as isize)).g = g;
                    (*colors.offset(count as isize)).b = b;
                    count += 1;
                }
                pos += color_len;
            } else {
                color_len = is_hsl_color(text, len, pos);
                if color_len > 0 as ::core::ffi::c_int {
                    (*colors.offset(count as isize)).start = pos;
                    (*colors.offset(count as isize)).end = pos + color_len;
                    (*colors.offset(count as isize)).r = 128 as ::core::ffi::c_int;
                    (*colors.offset(count as isize)).g = 128 as ::core::ffi::c_int;
                    (*colors.offset(count as isize)).b = 128 as ::core::ffi::c_int;
                    count += 1;
                    pos += color_len;
                } else {
                    pos += 1;
                }
            }
        }
    }
    return count;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
