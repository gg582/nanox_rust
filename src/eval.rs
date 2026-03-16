extern "C" {
    fn atoi(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
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
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcat(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    static mut term: *mut terminal;
    static mut tab_width: ::core::ffi::c_int;
    fn vttrez(res: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    static mut fillcol: ::core::ffi::c_int;
    static mut execstr: *mut ::core::ffi::c_char;
    static mut patmatch: *mut ::core::ffi::c_char;
    static mut flickcode: ::core::ffi::c_int;
    static mut gflags: ::core::ffi::c_int;
    static mut rval: ::core::ffi::c_int;
    static mut overlap: ::core::ffi::c_int;
    static mut scrollcount: ::core::ffi::c_int;
    static mut thisflag: ::core::ffi::c_int;
    static mut lastflag: ::core::ffi::c_int;
    static mut curgoal: ::core::ffi::c_int;
    static mut curwp: *mut window;
    static mut curbp: *mut buffer;
    static mut lastkey: ::core::ffi::c_int;
    static mut seed: ::core::ffi::c_int;
    static mut envram: ::core::ffi::c_long;
    static mut macbug: ::core::ffi::c_int;
    static mut cmdstatus: ::core::ffi::c_int;
    static mut pat: [::core::ffi::c_char; 0];
    static mut tap: [::core::ffi::c_char; 0];
    static mut rpat: [::core::ffi::c_char; 0];
    static mut discmd: ::core::ffi::c_int;
    static mut disinp: ::core::ffi::c_int;
    static mut gasave: ::core::ffi::c_int;
    static mut gacount: ::core::ffi::c_int;
    static mut gmode: ::core::ffi::c_int;
    static mut clexec: ::core::ffi::c_int;
    static mut errorm: [::core::ffi::c_char; 0];
    static mut truem: [::core::ffi::c_char; 0];
    static mut falsem: [::core::ffi::c_char; 0];
    static mut kbufh: *mut kill;
    static mut kused: ::core::ffi::c_int;
    static mut sres: [::core::ffi::c_char; 0];
    static mut palstr: [::core::ffi::c_char; 0];
    static mut saveflag: ::core::ffi::c_int;
    static mut confirmshell: ::core::ffi::c_int;
    static mut makebackup: ::core::ffi::c_int;
    static mut removebackup: ::core::ffi::c_int;
    fn newsize(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn newwidth(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn backchar(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn gotoline(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn getcline() -> ::core::ffi::c_int;
    fn getccol(bflg: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn setccol(pos: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn mlwrite(fmt: *const ::core::ffi::c_char, ...);
    fn typahead() -> ::core::ffi::c_int;
    fn ctoec(c: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn tgetc() -> ::core::ffi::c_int;
    fn getstring(
        prompt: *mut ::core::ffi::c_char,
        buf: *mut ::core::ffi::c_char,
        nbuf: ::core::ffi::c_int,
        eolchar: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn flook(
        fname: *mut ::core::ffi::c_char,
        hflag: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn transbind(skey: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn bfind(
        bname: *mut ::core::ffi::c_char,
        cflag: ::core::ffi::c_int,
        bflag: ::core::ffi::c_int,
    ) -> *mut buffer;
    fn fexist(fname: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn token(
        src: *mut ::core::ffi::c_char,
        tok: *mut ::core::ffi::c_char,
        size: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn macarg(tok: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn eq(bc: ::core::ffi::c_uchar, pc: ::core::ffi::c_uchar) -> ::core::ffi::c_int;
    fn rvstrscpy(
        rvstr: *mut ::core::ffi::c_char,
        str: *mut ::core::ffi::c_char,
        size: ::core::ffi::c_int,
    );
    fn mcclear();
    fn minibuf_input(
        prompt: *const ::core::ffi::c_char,
        dest: *mut ::core::ffi::c_char,
        max_len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn linsert(n: ::core::ffi::c_int, c: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn lnewline() -> ::core::ffi::c_int;
    fn ldelchar(n: ::core::ffi::c_long, kflag: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn getctext() -> *mut ::core::ffi::c_char;
    fn putctext(iline: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kill {
    pub d_next: *mut kill,
    pub d_chunk: [::core::ffi::c_char; 8192],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct variable_description {
    pub v_type: ::core::ffi::c_int,
    pub v_num: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct user_variable {
    pub u_name: [::core::ffi::c_char; 11],
    pub u_value: *mut ::core::ffi::c_char,
}
pub const TRINAMIC: function_type = 3;
pub type function_type = ::core::ffi::c_uint;
pub const DYNAMIC: function_type = 2;
pub const MONAMIC: function_type = 1;
pub const NILNAMIC: function_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct user_function {
    pub f_name: *mut ::core::ffi::c_char,
    pub f_type: function_type,
}
pub const NSTRING: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const NPAT: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ABORT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const INTWIDTH: usize = (::core::mem::size_of::<::core::ffi::c_int>() as usize)
    .wrapping_mul(3 as usize);
pub const TKNUL: ::core::ffi::c_int = 0;
pub const TKARG: ::core::ffi::c_int = 1;
pub const TKBUF: ::core::ffi::c_int = 2;
pub const TKVAR: ::core::ffi::c_int = 3;
pub const TKENV: ::core::ffi::c_int = 4;
pub const TKFUN: ::core::ffi::c_int = 5;
pub const TKDIR: ::core::ffi::c_int = 6;
pub const TKLBL: ::core::ffi::c_int = 7;
pub const TKLIT: ::core::ffi::c_int = 8;
pub const TKSTR: ::core::ffi::c_int = 9;
pub const TKCMD: ::core::ffi::c_int = 10;
pub const WFMOVE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const WFHARD: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const WFMODE: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const NVSIZE: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
static mut envars: [*mut ::core::ffi::c_char; 44] = [
    b"fillcol\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"pagelen\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"curcol\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"curline\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"ram\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"flicker\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"curwidth\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"cbufname\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"cfname\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"sres\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"debug\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"status\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"palette\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"asave\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"acount\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"lastkey\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"curchar\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"discmd\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"version\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"progname\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"seed\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"disinp\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"nopewline\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"nopecwline\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"target\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"search\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"replace\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"match\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"kill\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"cmode\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"gmode\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"tpause\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"pending\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"lwidth\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"line\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"gflags\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"rval\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"tab\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"overlap\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"scrollcount\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"scroll\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"confirmshell\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"makebackup\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"removebackup\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
];
pub const EVFILLCOL: ::core::ffi::c_int = 0;
pub const EVPAGELEN: ::core::ffi::c_int = 1;
pub const EVCURCOL: ::core::ffi::c_int = 2;
pub const EVCURLINE: ::core::ffi::c_int = 3;
pub const EVRAM: ::core::ffi::c_int = 4;
pub const EVFLICKER: ::core::ffi::c_int = 5;
pub const EVCURWIDTH: ::core::ffi::c_int = 6;
pub const EVCBUFNAME: ::core::ffi::c_int = 7;
pub const EVCFNAME: ::core::ffi::c_int = 8;
pub const EVSRES: ::core::ffi::c_int = 9;
pub const EVDEBUG: ::core::ffi::c_int = 10;
pub const EVSTATUS: ::core::ffi::c_int = 11;
pub const EVPALETTE: ::core::ffi::c_int = 12;
pub const EVASAVE: ::core::ffi::c_int = 13;
pub const EVACOUNT: ::core::ffi::c_int = 14;
pub const EVLASTKEY: ::core::ffi::c_int = 15;
pub const EVCURCHAR: ::core::ffi::c_int = 16;
pub const EVDISCMD: ::core::ffi::c_int = 17;
pub const EVVERSION: ::core::ffi::c_int = 18;
pub const EVPROGNAME: ::core::ffi::c_int = 19;
pub const EVSEED: ::core::ffi::c_int = 20;
pub const EVDISINP: ::core::ffi::c_int = 21;
pub const EVTARGET: ::core::ffi::c_int = 24;
pub const EVSEARCH: ::core::ffi::c_int = 25;
pub const EVREPLACE: ::core::ffi::c_int = 26;
pub const EVMATCH: ::core::ffi::c_int = 27;
pub const EVKILL: ::core::ffi::c_int = 28;
pub const EVCMODE: ::core::ffi::c_int = 29;
pub const EVGMODE: ::core::ffi::c_int = 30;
pub const EVTPAUSE: ::core::ffi::c_int = 31;
pub const EVPENDING: ::core::ffi::c_int = 32;
pub const EVLWIDTH: ::core::ffi::c_int = 33;
pub const EVLINE: ::core::ffi::c_int = 34;
pub const EVGFLAGS: ::core::ffi::c_int = 35;
pub const EVRVAL: ::core::ffi::c_int = 36;
pub const EVTAB: ::core::ffi::c_int = 37;
pub const EVOVERLAP: ::core::ffi::c_int = 38;
pub const EVSCROLLCOUNT: ::core::ffi::c_int = 39;
pub const EVSCROLL: ::core::ffi::c_int = 40;
pub const EVCONFIRMSHELL: ::core::ffi::c_int = 41;
pub const EVMAKEBACKUP: ::core::ffi::c_int = 42;
pub const EVREMOVEBACKUP: ::core::ffi::c_int = 43;
static mut funcs: [user_function; 39] = [
    user_function {
        f_name: b"add\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: DYNAMIC,
    },
    user_function {
        f_name: b"sub\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: DYNAMIC,
    },
    user_function {
        f_name: b"tim\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: DYNAMIC,
    },
    user_function {
        f_name: b"div\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: DYNAMIC,
    },
    user_function {
        f_name: b"mod\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: DYNAMIC,
    },
    user_function {
        f_name: b"neg\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: MONAMIC,
    },
    user_function {
        f_name: b"cat\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: DYNAMIC,
    },
    user_function {
        f_name: b"lef\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: DYNAMIC,
    },
    user_function {
        f_name: b"rig\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: DYNAMIC,
    },
    user_function {
        f_name: b"mid\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: TRINAMIC,
    },
    user_function {
        f_name: b"not\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: MONAMIC,
    },
    user_function {
        f_name: b"equ\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: DYNAMIC,
    },
    user_function {
        f_name: b"les\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: DYNAMIC,
    },
    user_function {
        f_name: b"gre\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: DYNAMIC,
    },
    user_function {
        f_name: b"seq\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: DYNAMIC,
    },
    user_function {
        f_name: b"sle\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: DYNAMIC,
    },
    user_function {
        f_name: b"sgr\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: DYNAMIC,
    },
    user_function {
        f_name: b"ind\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: MONAMIC,
    },
    user_function {
        f_name: b"and\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: DYNAMIC,
    },
    user_function {
        f_name: b"or\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: DYNAMIC,
    },
    user_function {
        f_name: b"len\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: MONAMIC,
    },
    user_function {
        f_name: b"upp\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: MONAMIC,
    },
    user_function {
        f_name: b"low\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: MONAMIC,
    },
    user_function {
        f_name: b"tru\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: MONAMIC,
    },
    user_function {
        f_name: b"asc\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: MONAMIC,
    },
    user_function {
        f_name: b"chr\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: MONAMIC,
    },
    user_function {
        f_name: b"gtk\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: NILNAMIC,
    },
    user_function {
        f_name: b"rnd\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: MONAMIC,
    },
    user_function {
        f_name: b"abs\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: MONAMIC,
    },
    user_function {
        f_name: b"sin\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: DYNAMIC,
    },
    user_function {
        f_name: b"env\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: MONAMIC,
    },
    user_function {
        f_name: b"bin\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: MONAMIC,
    },
    user_function {
        f_name: b"exi\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: MONAMIC,
    },
    user_function {
        f_name: b"fin\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: MONAMIC,
    },
    user_function {
        f_name: b"ban\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: DYNAMIC,
    },
    user_function {
        f_name: b"bor\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: DYNAMIC,
    },
    user_function {
        f_name: b"bxo\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: DYNAMIC,
    },
    user_function {
        f_name: b"bno\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: MONAMIC,
    },
    user_function {
        f_name: b"xla\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
        f_type: TRINAMIC,
    },
];
pub const UFADD: ::core::ffi::c_int = 0;
pub const UFSUB: ::core::ffi::c_int = 1;
pub const UFTIMES: ::core::ffi::c_int = 2;
pub const UFDIV: ::core::ffi::c_int = 3;
pub const UFMOD: ::core::ffi::c_int = 4;
pub const UFNEG: ::core::ffi::c_int = 5;
pub const UFCAT: ::core::ffi::c_int = 6;
pub const UFLEFT: ::core::ffi::c_int = 7;
pub const UFRIGHT: ::core::ffi::c_int = 8;
pub const UFMID: ::core::ffi::c_int = 9;
pub const UFNOT: ::core::ffi::c_int = 10;
pub const UFEQUAL: ::core::ffi::c_int = 11;
pub const UFLESS: ::core::ffi::c_int = 12;
pub const UFGREATER: ::core::ffi::c_int = 13;
pub const UFSEQUAL: ::core::ffi::c_int = 14;
pub const UFSLESS: ::core::ffi::c_int = 15;
pub const UFSGREAT: ::core::ffi::c_int = 16;
pub const UFIND: ::core::ffi::c_int = 17;
pub const UFAND: ::core::ffi::c_int = 18;
pub const UFOR: ::core::ffi::c_int = 19;
pub const UFLENGTH: ::core::ffi::c_int = 20;
pub const UFUPPER: ::core::ffi::c_int = 21;
pub const UFLOWER: ::core::ffi::c_int = 22;
pub const UFTRUTH: ::core::ffi::c_int = 23;
pub const UFASCII: ::core::ffi::c_int = 24;
pub const UFCHR: ::core::ffi::c_int = 25;
pub const UFGTKEY: ::core::ffi::c_int = 26;
pub const UFRND: ::core::ffi::c_int = 27;
pub const UFABS: ::core::ffi::c_int = 28;
pub const UFSINDEX: ::core::ffi::c_int = 29;
pub const UFENV: ::core::ffi::c_int = 30;
pub const UFBIND: ::core::ffi::c_int = 31;
pub const UFEXIST: ::core::ffi::c_int = 32;
pub const UFFIND: ::core::ffi::c_int = 33;
pub const UFBAND: ::core::ffi::c_int = 34;
pub const UFBOR: ::core::ffi::c_int = 35;
pub const UFBXOR: ::core::ffi::c_int = 36;
pub const UFBNOT: ::core::ffi::c_int = 37;
pub const UFXLATE: ::core::ffi::c_int = 38;
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
        let fresh6 = src;
        src = src.offset(1);
        let mut c: ::core::ffi::c_char = *fresh6;
        if c == 0 {
            break;
        }
        let fresh7 = dst;
        dst = dst.offset(1);
        *fresh7 = c;
    }
    *dst = 0 as ::core::ffi::c_char;
}
pub const PROGRAM_NAME_LONG: [::core::ffi::c_char; 10] = unsafe {
    ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"uEmacs/Pk\0")
};
pub const VERSION: [::core::ffi::c_char; 7] = unsafe {
    ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"4.0.15\0")
};
pub const MAXVARS: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
static mut uv: [user_variable; 256] = [user_variable {
    u_name: [0; 11],
    u_value: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
}; 256];
#[no_mangle]
pub unsafe extern "C" fn varinit() {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < MAXVARS {
        uv[i as usize].u_name[0 as ::core::ffi::c_int as usize] = 0
            as ::core::ffi::c_char;
        uv[i as usize].u_value = ::core::ptr::null_mut::<::core::ffi::c_char>();
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn varcleanup() {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < MAXVARS {
        if !uv[i as usize].u_value.is_null() {
            free(uv[i as usize].u_value as *mut ::core::ffi::c_void);
            uv[i as usize].u_value = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gtfun(
    mut fname: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut fnum: ::core::ffi::c_int = 0;
    let mut status: ::core::ffi::c_int = 0;
    let mut tsp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut arg1: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut arg2: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut arg3: [::core::ffi::c_char; 1024] = [0; 1024];
    static mut result: [::core::ffi::c_char; 2048] = [0; 2048];
    *fname.offset(3 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
    mklower(fname, fname);
    fnum = 0 as ::core::ffi::c_int;
    while (fnum as usize)
        < (::core::mem::size_of::<[user_function; 39]>() as usize)
            .wrapping_div(::core::mem::size_of::<user_function>() as usize)
    {
        if strcmp(fname, funcs[fnum as usize].f_name) == 0 as ::core::ffi::c_int {
            break;
        }
        fnum += 1;
    }
    if fnum as usize
        == (::core::mem::size_of::<[user_function; 39]>() as usize)
            .wrapping_div(::core::mem::size_of::<user_function>() as usize)
    {
        return &raw mut errorm as *mut ::core::ffi::c_char;
    }
    if funcs[fnum as usize].f_type as ::core::ffi::c_uint
        >= MONAMIC as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        status = macarg(&raw mut arg1 as *mut ::core::ffi::c_char);
        if status != TRUE {
            return &raw mut errorm as *mut ::core::ffi::c_char;
        }
        if funcs[fnum as usize].f_type as ::core::ffi::c_uint
            >= DYNAMIC as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            status = macarg(&raw mut arg2 as *mut ::core::ffi::c_char);
            if status != TRUE {
                return &raw mut errorm as *mut ::core::ffi::c_char;
            }
            if funcs[fnum as usize].f_type as ::core::ffi::c_uint
                >= TRINAMIC as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                status = macarg(&raw mut arg3 as *mut ::core::ffi::c_char);
                if status != TRUE {
                    return &raw mut errorm as *mut ::core::ffi::c_char;
                }
            }
        }
    }
    match fnum {
        UFADD => {
            return itoa(
                atoi(&raw mut arg1 as *mut ::core::ffi::c_char)
                    + atoi(&raw mut arg2 as *mut ::core::ffi::c_char),
            );
        }
        UFSUB => {
            return itoa(
                atoi(&raw mut arg1 as *mut ::core::ffi::c_char)
                    - atoi(&raw mut arg2 as *mut ::core::ffi::c_char),
            );
        }
        UFTIMES => {
            return itoa(
                atoi(&raw mut arg1 as *mut ::core::ffi::c_char)
                    * atoi(&raw mut arg2 as *mut ::core::ffi::c_char),
            );
        }
        UFDIV => {
            return itoa(
                atoi(&raw mut arg1 as *mut ::core::ffi::c_char)
                    / atoi(&raw mut arg2 as *mut ::core::ffi::c_char),
            );
        }
        UFMOD => {
            return itoa(
                atoi(&raw mut arg1 as *mut ::core::ffi::c_char)
                    % atoi(&raw mut arg2 as *mut ::core::ffi::c_char),
            );
        }
        UFNEG => return itoa(-atoi(&raw mut arg1 as *mut ::core::ffi::c_char)),
        UFCAT => {
            strcpy(
                &raw mut result as *mut ::core::ffi::c_char,
                &raw mut arg1 as *mut ::core::ffi::c_char,
            );
            return strcat(
                &raw mut result as *mut ::core::ffi::c_char,
                &raw mut arg2 as *mut ::core::ffi::c_char,
            );
        }
        UFLEFT => {
            return strncpy(
                &raw mut result as *mut ::core::ffi::c_char,
                &raw mut arg1 as *mut ::core::ffi::c_char,
                atoi(&raw mut arg2 as *mut ::core::ffi::c_char) as size_t,
            );
        }
        UFRIGHT => {
            return strcpy(
                &raw mut result as *mut ::core::ffi::c_char,
                (&raw mut arg1 as *mut ::core::ffi::c_char)
                    .offset(
                        (strlen
                            as unsafe extern "C" fn(
                                *const ::core::ffi::c_char,
                            ) -> size_t)(&raw mut arg1 as *mut ::core::ffi::c_char)
                            .wrapping_sub(
                                (atoi
                                    as unsafe extern "C" fn(
                                        *const ::core::ffi::c_char,
                                    ) -> ::core::ffi::c_int)(
                                    &raw mut arg2 as *mut ::core::ffi::c_char,
                                ) as size_t,
                            ) as isize,
                    ) as *mut ::core::ffi::c_char,
            );
        }
        UFMID => {
            return strncpy(
                &raw mut result as *mut ::core::ffi::c_char,
                (&raw mut arg1 as *mut ::core::ffi::c_char)
                    .offset(
                        ((atoi
                            as unsafe extern "C" fn(
                                *const ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int)(
                            &raw mut arg2 as *mut ::core::ffi::c_char,
                        ) - 1 as ::core::ffi::c_int) as isize,
                    ) as *mut ::core::ffi::c_char,
                atoi(&raw mut arg3 as *mut ::core::ffi::c_char) as size_t,
            );
        }
        UFNOT => {
            return ltos(
                (stol(&raw mut arg1 as *mut ::core::ffi::c_char) == FALSE)
                    as ::core::ffi::c_int,
            );
        }
        UFEQUAL => {
            return ltos(
                (atoi(&raw mut arg1 as *mut ::core::ffi::c_char)
                    == atoi(&raw mut arg2 as *mut ::core::ffi::c_char))
                    as ::core::ffi::c_int,
            );
        }
        UFLESS => {
            return ltos(
                (atoi(&raw mut arg1 as *mut ::core::ffi::c_char)
                    < atoi(&raw mut arg2 as *mut ::core::ffi::c_char))
                    as ::core::ffi::c_int,
            );
        }
        UFGREATER => {
            return ltos(
                (atoi(&raw mut arg1 as *mut ::core::ffi::c_char)
                    > atoi(&raw mut arg2 as *mut ::core::ffi::c_char))
                    as ::core::ffi::c_int,
            );
        }
        UFSEQUAL => {
            return ltos(
                (strcmp(
                    &raw mut arg1 as *mut ::core::ffi::c_char,
                    &raw mut arg2 as *mut ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
            );
        }
        UFSLESS => {
            return ltos(
                (strcmp(
                    &raw mut arg1 as *mut ::core::ffi::c_char,
                    &raw mut arg2 as *mut ::core::ffi::c_char,
                ) < 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
            );
        }
        UFSGREAT => {
            return ltos(
                (strcmp(
                    &raw mut arg1 as *mut ::core::ffi::c_char,
                    &raw mut arg2 as *mut ::core::ffi::c_char,
                ) > 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
            );
        }
        UFIND => {
            return getval(
                &raw mut arg1 as *mut ::core::ffi::c_char,
                &raw mut result as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 2048]>()
                    as ::core::ffi::c_int,
            );
        }
        UFAND => {
            return ltos(
                (stol(&raw mut arg1 as *mut ::core::ffi::c_char) != 0
                    && stol(&raw mut arg2 as *mut ::core::ffi::c_char) != 0)
                    as ::core::ffi::c_int,
            );
        }
        UFOR => {
            return ltos(
                (stol(&raw mut arg1 as *mut ::core::ffi::c_char) != 0
                    || stol(&raw mut arg2 as *mut ::core::ffi::c_char) != 0)
                    as ::core::ffi::c_int,
            );
        }
        UFLENGTH => {
            return itoa(
                strlen(&raw mut arg1 as *mut ::core::ffi::c_char) as ::core::ffi::c_int,
            );
        }
        UFUPPER => {
            return mkupper(
                &raw mut arg1 as *mut ::core::ffi::c_char,
                &raw mut result as *mut ::core::ffi::c_char,
            );
        }
        UFLOWER => {
            return mklower(
                &raw mut arg1 as *mut ::core::ffi::c_char,
                &raw mut result as *mut ::core::ffi::c_char,
            );
        }
        UFTRUTH => {
            return ltos(
                (atoi(&raw mut arg1 as *mut ::core::ffi::c_char)
                    == 42 as ::core::ffi::c_int) as ::core::ffi::c_int,
            );
        }
        UFASCII => {
            return itoa(arg1[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int);
        }
        UFCHR => {
            result[0 as ::core::ffi::c_int as usize] = atoi(
                &raw mut arg1 as *mut ::core::ffi::c_char,
            ) as ::core::ffi::c_char;
            result[1 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
            return &raw mut result as *mut ::core::ffi::c_char;
        }
        UFGTKEY => {
            result[0 as ::core::ffi::c_int as usize] = tgetc() as ::core::ffi::c_char;
            result[1 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
            return &raw mut result as *mut ::core::ffi::c_char;
        }
        UFRND => {
            return itoa(
                ernd() % abs(atoi(&raw mut arg1 as *mut ::core::ffi::c_char))
                    + 1 as ::core::ffi::c_int,
            );
        }
        UFABS => return itoa(abs(atoi(&raw mut arg1 as *mut ::core::ffi::c_char))),
        UFSINDEX => {
            return itoa(
                sindex(
                    &raw mut arg1 as *mut ::core::ffi::c_char,
                    &raw mut arg2 as *mut ::core::ffi::c_char,
                ),
            );
        }
        UFENV => {
            tsp = getenv(&raw mut arg1 as *mut ::core::ffi::c_char);
            return (if tsp.is_null() {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                tsp as *const ::core::ffi::c_char
            }) as *mut ::core::ffi::c_char;
        }
        UFBIND => return transbind(&raw mut arg1 as *mut ::core::ffi::c_char),
        UFEXIST => return ltos(fexist(&raw mut arg1 as *mut ::core::ffi::c_char)),
        UFFIND => {
            tsp = flook(&raw mut arg1 as *mut ::core::ffi::c_char, TRUE);
            return (if tsp.is_null() {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                tsp as *const ::core::ffi::c_char
            }) as *mut ::core::ffi::c_char;
        }
        UFBAND => {
            return itoa(
                atoi(&raw mut arg1 as *mut ::core::ffi::c_char)
                    & atoi(&raw mut arg2 as *mut ::core::ffi::c_char),
            );
        }
        UFBOR => {
            return itoa(
                atoi(&raw mut arg1 as *mut ::core::ffi::c_char)
                    | atoi(&raw mut arg2 as *mut ::core::ffi::c_char),
            );
        }
        UFBXOR => {
            return itoa(
                atoi(&raw mut arg1 as *mut ::core::ffi::c_char)
                    ^ atoi(&raw mut arg2 as *mut ::core::ffi::c_char),
            );
        }
        UFBNOT => return itoa(!atoi(&raw mut arg1 as *mut ::core::ffi::c_char)),
        UFXLATE => {
            return xlat(
                &raw mut arg1 as *mut ::core::ffi::c_char,
                &raw mut arg2 as *mut ::core::ffi::c_char,
                &raw mut arg3 as *mut ::core::ffi::c_char,
            );
        }
        _ => {}
    }
    exit(-(11 as ::core::ffi::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn gtusr(
    mut vname: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut vnum: ::core::ffi::c_int = 0;
    vnum = 0 as ::core::ffi::c_int;
    while vnum < MAXVARS {
        if uv[vnum as usize].u_name[0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            return &raw mut errorm as *mut ::core::ffi::c_char;
        }
        if strcmp(
            vname,
            &raw mut (*(&raw mut uv as *mut user_variable).offset(vnum as isize)).u_name
                as *mut ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            return uv[vnum as usize].u_value;
        }
        vnum += 1;
    }
    return &raw mut errorm as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn gtenv(
    mut vname: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut vnum: ::core::ffi::c_int = 0;
    vnum = 0 as ::core::ffi::c_int;
    while (vnum as usize)
        < (::core::mem::size_of::<[*mut ::core::ffi::c_char; 44]>() as usize)
            .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
    {
        if strcmp(vname, envars[vnum as usize]) == 0 as ::core::ffi::c_int {
            break;
        }
        vnum += 1;
    }
    if vnum as usize
        == (::core::mem::size_of::<[*mut ::core::ffi::c_char; 44]>() as usize)
            .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
    {
        let mut ename: *mut ::core::ffi::c_char = getenv(vname);
        if !ename.is_null() {
            return ename
        } else {
            return &raw mut errorm as *mut ::core::ffi::c_char
        }
    }
    match vnum {
        EVFILLCOL => return itoa(fillcol),
        EVPAGELEN => {
            return itoa((*term).t_nrow as ::core::ffi::c_int + 1 as ::core::ffi::c_int);
        }
        EVCURCOL => return itoa(getccol(FALSE)),
        EVCURLINE => return itoa(getcline()),
        EVRAM => {
            return itoa((envram / 1024 as ::core::ffi::c_long) as ::core::ffi::c_int);
        }
        EVFLICKER => return ltos(flickcode),
        EVCURWIDTH => return itoa((*term).t_ncol as ::core::ffi::c_int),
        EVCBUFNAME => return &raw mut (*curbp).b_bname as *mut ::core::ffi::c_char,
        EVCFNAME => return &raw mut (*curbp).b_fname as *mut ::core::ffi::c_char,
        EVSRES => return &raw mut sres as *mut ::core::ffi::c_char,
        EVDEBUG => return ltos(macbug),
        EVSTATUS => return ltos(cmdstatus),
        EVPALETTE => return &raw mut palstr as *mut ::core::ffi::c_char,
        EVASAVE => return itoa(gasave),
        EVACOUNT => return itoa(gacount),
        EVLASTKEY => return itoa(lastkey),
        EVCURCHAR => {
            return if (*(*curwp).w_dotp).l_used == (*curwp).w_doto {
                itoa('\n' as i32)
            } else {
                itoa(
                    *(&raw mut (*(*curwp).w_dotp).l_text as *mut ::core::ffi::c_uchar)
                        .offset((*curwp).w_doto as isize) as ::core::ffi::c_int
                        & 0xff as ::core::ffi::c_int,
                )
            };
        }
        EVDISCMD => return ltos(discmd),
        EVVERSION => return VERSION.as_ptr() as *mut ::core::ffi::c_char,
        EVPROGNAME => return PROGRAM_NAME_LONG.as_ptr() as *mut ::core::ffi::c_char,
        EVSEED => return itoa(seed),
        EVDISINP => return ltos(disinp),
        EVTARGET => {
            saveflag = lastflag;
            return itoa(curgoal);
        }
        EVSEARCH => return &raw mut pat as *mut ::core::ffi::c_char,
        EVREPLACE => return &raw mut rpat as *mut ::core::ffi::c_char,
        EVMATCH => {
            return (if patmatch.is_null() {
                b"\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                patmatch as *const ::core::ffi::c_char
            }) as *mut ::core::ffi::c_char;
        }
        EVKILL => return getkill(),
        EVCMODE => return itoa((*curbp).b_mode),
        EVGMODE => return itoa(gmode),
        EVTPAUSE => return itoa((*term).t_pause),
        EVPENDING => return ltos(typahead()),
        EVLWIDTH => return itoa((*(*curwp).w_dotp).l_used),
        EVLINE => return getctext(),
        EVGFLAGS => return itoa(gflags),
        EVRVAL => return itoa(rval),
        EVTAB => return itoa(tab_width + 1 as ::core::ffi::c_int),
        EVOVERLAP => return itoa(overlap),
        EVSCROLLCOUNT => return itoa(scrollcount),
        EVSCROLL => return ltos(0 as ::core::ffi::c_int),
        EVCONFIRMSHELL => return ltos(confirmshell),
        EVMAKEBACKUP => return ltos(makebackup),
        EVREMOVEBACKUP => return ltos(removebackup),
        _ => {}
    }
    exit(-(12 as ::core::ffi::c_int));
}
unsafe extern "C" fn getkill() -> *mut ::core::ffi::c_char {
    let mut size: ::core::ffi::c_int = 0;
    static mut value: [::core::ffi::c_char; 1024] = [0; 1024];
    if kbufh.is_null() {
        value[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    } else {
        if kused < NSTRING {
            size = kused;
        } else {
            size = NSTRING - 1 as ::core::ffi::c_int;
        }
        memcpy(
            &raw mut value as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            &raw mut (*kbufh).d_chunk as *mut ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            size as size_t,
        );
        value[size as usize] = 0 as ::core::ffi::c_char;
    }
    return &raw mut value as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn setvar(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut status: ::core::ffi::c_int = 0;
    let mut vd: variable_description = variable_description {
        v_type: 0,
        v_num: 0,
    };
    let mut var: [::core::ffi::c_char; 11] = [0; 11];
    let mut value: [::core::ffi::c_char; 1024] = [0; 1024];
    if clexec == FALSE {
        status = minibuf_input(
            b"Variable to set: \0" as *const u8 as *const ::core::ffi::c_char,
            (&raw mut var as *mut ::core::ffi::c_char)
                .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
            NVSIZE,
        );
        if status != TRUE {
            return status;
        }
    } else {
        execstr = token(
            execstr,
            &raw mut var as *mut ::core::ffi::c_char,
            NVSIZE + 1 as ::core::ffi::c_int,
        );
    }
    findvar(
        &raw mut var as *mut ::core::ffi::c_char,
        &raw mut vd,
        NVSIZE + 1 as ::core::ffi::c_int,
    );
    if vd.v_type == -(1 as ::core::ffi::c_int) {
        mlwrite(
            b"%%No such variable as '%s'\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut var as *mut ::core::ffi::c_char,
        );
        return FALSE;
    }
    if f == TRUE {
        strcpy(&raw mut value as *mut ::core::ffi::c_char, itoa(n));
    } else {
        status = minibuf_input(
            b"Value: \0" as *const u8 as *const ::core::ffi::c_char,
            (&raw mut value as *mut ::core::ffi::c_char)
                .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
            NSTRING,
        );
        if status != TRUE {
            return status;
        }
    }
    status = svar(&raw mut vd, &raw mut value as *mut ::core::ffi::c_char);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn findvar(
    mut var: *mut ::core::ffi::c_char,
    mut vd: *mut variable_description,
    mut size: ::core::ffi::c_int,
) {
    let mut current_block: u64;
    let mut vnum: ::core::ffi::c_int = 0;
    let mut vtype: ::core::ffi::c_int = 0;
    vnum = -(1 as ::core::ffi::c_int);
    loop {
        vtype = -(1 as ::core::ffi::c_int);
        match *var.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int {
            36 => {
                vnum = 0 as ::core::ffi::c_int;
                while (vnum as usize)
                    < (::core::mem::size_of::<[*mut ::core::ffi::c_char; 44]>() as usize)
                        .wrapping_div(
                            ::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize,
                        )
                {
                    if strcmp(
                        var.offset(1 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_char,
                        envars[vnum as usize],
                    ) == 0 as ::core::ffi::c_int
                    {
                        vtype = TKENV;
                        break;
                    } else {
                        vnum += 1;
                    }
                }
                current_block = 14401909646449704462;
                break;
            }
            37 => {
                vnum = 0 as ::core::ffi::c_int;
                while vnum < MAXVARS {
                    if strcmp(
                        var.offset(1 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_char,
                        &raw mut (*(&raw mut uv as *mut user_variable)
                            .offset(vnum as isize))
                            .u_name as *mut ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        vtype = TKVAR;
                        break;
                    } else {
                        vnum += 1;
                    }
                }
                if vnum < MAXVARS {
                    current_block = 14401909646449704462;
                    break;
                } else {
                    current_block = 12800627514080957624;
                    break;
                }
            }
            38 => {
                *var.offset(4 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
                if !(strcmp(
                    var.offset(1 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char,
                    b"ind\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int)
                {
                    current_block = 14401909646449704462;
                    break;
                }
                execstr = token(execstr, var, size);
                getval(var, var, size);
            }
            _ => {
                current_block = 14401909646449704462;
                break;
            }
        }
    }
    match current_block {
        12800627514080957624 => {
            vnum = 0 as ::core::ffi::c_int;
            while vnum < MAXVARS {
                if uv[vnum as usize].u_name[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    vtype = TKVAR;
                    strcpy(
                        &raw mut (*(&raw mut uv as *mut user_variable)
                            .offset(vnum as isize))
                            .u_name as *mut ::core::ffi::c_char,
                        var.offset(1 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_char,
                    );
                    break;
                } else {
                    vnum += 1;
                }
            }
        }
        _ => {}
    }
    (*vd).v_num = vnum;
    (*vd).v_type = vtype;
}
#[no_mangle]
pub unsafe extern "C" fn svar(
    mut var: *mut variable_description,
    mut value: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut vnum: ::core::ffi::c_int = 0;
    let mut vtype: ::core::ffi::c_int = 0;
    let mut status: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    let mut sp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    vnum = (*var).v_num;
    vtype = (*var).v_type;
    status = TRUE;
    match vtype {
        TKVAR => {
            if !uv[vnum as usize].u_value.is_null() {
                free(uv[vnum as usize].u_value as *mut ::core::ffi::c_void);
            }
            sp = malloc(strlen(value).wrapping_add(1 as size_t))
                as *mut ::core::ffi::c_char;
            if sp.is_null() {
                return FALSE;
            }
            strcpy(sp, value);
            uv[vnum as usize].u_value = sp;
        }
        TKENV => {
            status = TRUE;
            let mut current_block_56: u64;
            match vnum {
                EVFILLCOL => {
                    fillcol = atoi(value);
                    current_block_56 = 10512632378975961025;
                }
                EVPAGELEN => {
                    status = newsize(TRUE, atoi(value));
                    current_block_56 = 10512632378975961025;
                }
                EVCURCOL => {
                    status = setccol(atoi(value));
                    current_block_56 = 10512632378975961025;
                }
                EVCURLINE => {
                    status = gotoline(TRUE, atoi(value));
                    current_block_56 = 10512632378975961025;
                }
                EVFLICKER => {
                    flickcode = stol(value);
                    current_block_56 = 10512632378975961025;
                }
                EVCURWIDTH => {
                    status = newwidth(TRUE, atoi(value));
                    current_block_56 = 10512632378975961025;
                }
                EVCBUFNAME => {
                    strcpy(&raw mut (*curbp).b_bname as *mut ::core::ffi::c_char, value);
                    (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | WFMODE)
                        as ::core::ffi::c_char;
                    current_block_56 = 10512632378975961025;
                }
                EVCFNAME => {
                    strcpy(&raw mut (*curbp).b_fname as *mut ::core::ffi::c_char, value);
                    (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | WFMODE)
                        as ::core::ffi::c_char;
                    current_block_56 = 10512632378975961025;
                }
                EVSRES => {
                    status = vttrez(value);
                    current_block_56 = 10512632378975961025;
                }
                EVDEBUG => {
                    macbug = stol(value);
                    current_block_56 = 10512632378975961025;
                }
                EVSTATUS => {
                    cmdstatus = stol(value);
                    current_block_56 = 10512632378975961025;
                }
                EVASAVE => {
                    gasave = atoi(value);
                    current_block_56 = 10512632378975961025;
                }
                EVACOUNT => {
                    gacount = atoi(value);
                    current_block_56 = 10512632378975961025;
                }
                EVLASTKEY => {
                    lastkey = atoi(value);
                    current_block_56 = 10512632378975961025;
                }
                EVCURCHAR => {
                    ldelchar(1 as ::core::ffi::c_long, FALSE);
                    c = atoi(value);
                    if c == '\n' as i32 {
                        lnewline();
                    } else {
                        linsert(1 as ::core::ffi::c_int, c);
                    }
                    backchar(FALSE, 1 as ::core::ffi::c_int);
                    current_block_56 = 10512632378975961025;
                }
                EVDISCMD => {
                    discmd = stol(value);
                    current_block_56 = 10512632378975961025;
                }
                EVSEED => {
                    seed = atoi(value);
                    current_block_56 = 10512632378975961025;
                }
                EVDISINP => {
                    disinp = stol(value);
                    current_block_56 = 10512632378975961025;
                }
                EVTARGET => {
                    curgoal = atoi(value);
                    thisflag = saveflag;
                    current_block_56 = 10512632378975961025;
                }
                EVSEARCH => {
                    strcpy(&raw mut pat as *mut ::core::ffi::c_char, value);
                    rvstrscpy(
                        &raw mut tap as *mut ::core::ffi::c_char,
                        &raw mut pat as *mut ::core::ffi::c_char,
                        NPAT,
                    );
                    mcclear();
                    current_block_56 = 10512632378975961025;
                }
                EVREPLACE => {
                    strcpy(&raw mut rpat as *mut ::core::ffi::c_char, value);
                    current_block_56 = 10512632378975961025;
                }
                EVCMODE => {
                    (*curbp).b_mode = atoi(value);
                    (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | WFMODE)
                        as ::core::ffi::c_char;
                    current_block_56 = 10512632378975961025;
                }
                EVGMODE => {
                    gmode = atoi(value);
                    current_block_56 = 10512632378975961025;
                }
                EVTPAUSE => {
                    (*term).t_pause = atoi(value);
                    current_block_56 = 10512632378975961025;
                }
                EVLINE => {
                    putctext(value);
                    current_block_56 = 17010805083111289497;
                }
                EVGFLAGS => {
                    current_block_56 = 17010805083111289497;
                }
                EVTAB => {
                    tab_width = atoi(value) - 1 as ::core::ffi::c_int;
                    if tab_width != 0x7 as ::core::ffi::c_int
                        && tab_width != 0x3 as ::core::ffi::c_int
                    {
                        tab_width = 0x7 as ::core::ffi::c_int;
                    }
                    (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | WFHARD)
                        as ::core::ffi::c_char;
                    current_block_56 = 10512632378975961025;
                }
                EVOVERLAP => {
                    overlap = atoi(value);
                    current_block_56 = 10512632378975961025;
                }
                EVSCROLLCOUNT => {
                    scrollcount = atoi(value);
                    current_block_56 = 10512632378975961025;
                }
                EVCONFIRMSHELL => {
                    confirmshell = stol(value);
                    current_block_56 = 10512632378975961025;
                }
                EVMAKEBACKUP => {
                    makebackup = stol(value);
                    current_block_56 = 10512632378975961025;
                }
                EVREMOVEBACKUP => {
                    removebackup = stol(value);
                    current_block_56 = 10512632378975961025;
                }
                EVRAM
                | EVVERSION
                | EVPROGNAME
                | EVMATCH
                | EVKILL
                | EVPENDING
                | EVLWIDTH
                | EVRVAL
                | EVSCROLL
                | _ => {
                    current_block_56 = 10512632378975961025;
                }
            }
            match current_block_56 {
                17010805083111289497 => {
                    gflags = atoi(value);
                }
                _ => {}
            }
        }
        _ => {}
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn itoa(mut i: ::core::ffi::c_int) -> *mut ::core::ffi::c_char {
    let mut digit: ::core::ffi::c_int = 0;
    let mut sp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut sign: ::core::ffi::c_int = 0;
    static mut result: [::core::ffi::c_char; 13] = [0; 13];
    sign = 1 as ::core::ffi::c_int;
    if i < 0 as ::core::ffi::c_int {
        sign = -(1 as ::core::ffi::c_int);
        i = -i;
    }
    sp = (&raw mut result as *mut ::core::ffi::c_char).offset(INTWIDTH as isize);
    *sp = 0 as ::core::ffi::c_char;
    loop {
        digit = i % 10 as ::core::ffi::c_int;
        sp = sp.offset(-1);
        *sp = ('0' as i32 + digit) as ::core::ffi::c_char;
        i = i / 10 as ::core::ffi::c_int;
        if !(i != 0) {
            break;
        }
    }
    if sign == -(1 as ::core::ffi::c_int) {
        sp = sp.offset(-1);
        *sp = '-' as i32 as ::core::ffi::c_char;
    }
    return sp;
}
#[no_mangle]
pub unsafe extern "C" fn gettyp(
    mut token_0: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_char = 0;
    c = *token_0;
    if c as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return TKNUL;
    }
    if c as ::core::ffi::c_int >= '0' as i32 && c as ::core::ffi::c_int <= '9' as i32 {
        return TKLIT;
    }
    match c as ::core::ffi::c_int {
        34 => return TKSTR,
        33 => return TKDIR,
        64 => return TKARG,
        35 => return TKBUF,
        36 => return TKENV,
        37 => return TKVAR,
        38 => return TKFUN,
        42 => return TKLBL,
        _ => return TKCMD,
    };
}
unsafe extern "C" fn internal_getval(
    mut token_0: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut status: ::core::ffi::c_int = 0;
    let mut bp: *mut buffer = ::core::ptr::null_mut::<buffer>();
    let mut blen: ::core::ffi::c_int = 0;
    let mut distmp: ::core::ffi::c_int = 0;
    static mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
    match gettyp(token_0) {
        TKNUL => {
            return b"\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
        }
        TKARG => {
            getval(
                token_0.offset(1 as ::core::ffi::c_int as isize),
                token_0,
                -(1 as ::core::ffi::c_int),
            );
            distmp = discmd;
            discmd = TRUE;
            status = getstring(
                token_0,
                &raw mut buf as *mut ::core::ffi::c_char,
                NSTRING,
                ctoec('\n' as i32),
            );
            discmd = distmp;
            if status == ABORT {
                return &raw mut errorm as *mut ::core::ffi::c_char;
            }
            return &raw mut buf as *mut ::core::ffi::c_char;
        }
        TKBUF => {
            getval(
                token_0.offset(1 as ::core::ffi::c_int as isize),
                token_0,
                -(1 as ::core::ffi::c_int),
            );
            bp = bfind(token_0, FALSE, 0 as ::core::ffi::c_int);
            if bp.is_null() {
                return &raw mut errorm as *mut ::core::ffi::c_char;
            }
            if (*bp).b_nwnd as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                (*curbp).b_dotp = (*curwp).w_dotp;
                (*curbp).b_doto = (*curwp).w_doto;
            }
            if (*bp).b_linep == (*bp).b_dotp {
                return &raw mut errorm as *mut ::core::ffi::c_char;
            }
            blen = (*(*bp).b_dotp).l_used - (*bp).b_doto;
            if blen >= NSTRING {
                blen = NSTRING - 1 as ::core::ffi::c_int;
            }
            strncpy(
                &raw mut buf as *mut ::core::ffi::c_char,
                (&raw mut (*(*bp).b_dotp).l_text as *mut ::core::ffi::c_uchar)
                    .offset((*bp).b_doto as isize) as *const ::core::ffi::c_char,
                blen as size_t,
            );
            buf[blen as usize] = 0 as ::core::ffi::c_char;
            (*bp).b_dotp = (*(*bp).b_dotp).l_fp;
            (*bp).b_doto = 0 as ::core::ffi::c_int;
            if (*bp).b_nwnd as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                (*curwp).w_dotp = (*curbp).b_dotp;
                (*curwp).w_doto = 0 as ::core::ffi::c_int;
                (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | WFMOVE)
                    as ::core::ffi::c_char;
            }
            return &raw mut buf as *mut ::core::ffi::c_char;
        }
        TKVAR => return gtusr(token_0.offset(1 as ::core::ffi::c_int as isize)),
        TKENV => return gtenv(token_0.offset(1 as ::core::ffi::c_int as isize)),
        TKFUN => return gtfun(token_0.offset(1 as ::core::ffi::c_int as isize)),
        TKDIR => return &raw mut errorm as *mut ::core::ffi::c_char,
        TKLBL => return &raw mut errorm as *mut ::core::ffi::c_char,
        TKLIT => return token_0,
        TKSTR => return token_0.offset(1 as ::core::ffi::c_int as isize),
        TKCMD => return token_0,
        _ => {}
    }
    return &raw mut errorm as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn getval(
    mut token_0: *mut ::core::ffi::c_char,
    mut dst: *mut ::core::ffi::c_char,
    mut size: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut res: *mut ::core::ffi::c_char = internal_getval(token_0);
    mystrscpy(dst, res, size);
    return dst;
}
#[no_mangle]
pub unsafe extern "C" fn stol(mut val: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    if *val.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'F' as i32
    {
        return FALSE;
    }
    if *val.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'T' as i32
    {
        return TRUE;
    }
    return (atoi(val) != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ltos(mut val: ::core::ffi::c_int) -> *mut ::core::ffi::c_char {
    if val != 0 {
        return &raw mut truem as *mut ::core::ffi::c_char
    } else {
        return &raw mut falsem as *mut ::core::ffi::c_char
    };
}
#[no_mangle]
pub unsafe extern "C" fn mkupper(
    mut str: *const ::core::ffi::c_char,
    mut res: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut out: *mut ::core::ffi::c_char = res;
    let mut c: ::core::ffi::c_char = 0;
    loop {
        let fresh4 = str;
        str = str.offset(1);
        c = *fresh4;
        if 'a' as i32 <= c as ::core::ffi::c_int && c as ::core::ffi::c_int <= 'z' as i32
        {
            c = (c as ::core::ffi::c_int + ('A' as i32 - 'a' as i32))
                as ::core::ffi::c_char;
        }
        let fresh5 = out;
        out = out.offset(1);
        *fresh5 = c;
        if !(c != 0) {
            break;
        }
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mklower(
    mut str: *const ::core::ffi::c_char,
    mut res: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut out: *mut ::core::ffi::c_char = res;
    let mut c: ::core::ffi::c_char = 0;
    out = res;
    loop {
        let fresh2 = str;
        str = str.offset(1);
        c = *fresh2;
        if 'A' as i32 <= c as ::core::ffi::c_int && c as ::core::ffi::c_int <= 'Z' as i32
        {
            c = (c as ::core::ffi::c_int + ('a' as i32 - 'A' as i32))
                as ::core::ffi::c_char;
        }
        let fresh3 = out;
        out = out.offset(1);
        *fresh3 = c;
        if !(c != 0) {
            break;
        }
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn abs(mut x: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return if x < 0 as ::core::ffi::c_int { -x } else { x };
}
#[no_mangle]
pub unsafe extern "C" fn ernd() -> ::core::ffi::c_int {
    seed = abs(seed * 1721 as ::core::ffi::c_int + 10007 as ::core::ffi::c_int);
    return seed;
}
#[no_mangle]
pub unsafe extern "C" fn sindex(
    mut source: *mut ::core::ffi::c_char,
    mut pattern: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut sp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut csp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut cp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    sp = source;
    while *sp != 0 {
        cp = pattern;
        csp = sp;
        while *cp != 0 {
            if eq(*cp as ::core::ffi::c_uchar, *csp as ::core::ffi::c_uchar) == 0 {
                break;
            }
            cp = cp.offset(1);
            csp = csp.offset(1);
        }
        if *cp as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return sp.offset_from(source) as ::core::ffi::c_long as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int;
        }
        sp = sp.offset(1);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xlat(
    mut source: *mut ::core::ffi::c_char,
    mut lookup: *mut ::core::ffi::c_char,
    mut trans: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut sp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut lp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut rp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    static mut result: [::core::ffi::c_char; 1024] = [0; 1024];
    sp = source;
    rp = &raw mut result as *mut ::core::ffi::c_char;
    while *sp != 0 {
        let mut current_block_6: u64;
        lp = lookup;
        loop {
            if !(*lp != 0) {
                current_block_6 = 14523784380283086299;
                break;
            }
            if *sp as ::core::ffi::c_int == *lp as ::core::ffi::c_int {
                let fresh0 = rp;
                rp = rp.offset(1);
                *fresh0 = *trans
                    .offset(lp.offset_from(lookup) as ::core::ffi::c_long as isize);
                current_block_6 = 17816083214013702422;
                break;
            } else {
                lp = lp.offset(1);
            }
        }
        match current_block_6 {
            14523784380283086299 => {
                let fresh1 = rp;
                rp = rp.offset(1);
                *fresh1 = *sp;
            }
            _ => {}
        }
        sp = sp.offset(1);
    }
    *rp = 0 as ::core::ffi::c_char;
    return &raw mut result as *mut ::core::ffi::c_char;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
