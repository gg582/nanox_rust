extern "C" {
    pub type line;
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
pub struct magic {
    pub mc_type: ::core::ffi::c_short,
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub lchar: ::core::ffi::c_int,
    pub cclmap: *mut ::core::ffi::c_char,
}
pub const GFREAD: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const HUGE: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const CONTROL: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const STOP: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut fillcol: ::core::ffi::c_int = 72 as ::core::ffi::c_int;
#[no_mangle]
pub static mut execstr: *mut ::core::ffi::c_char = ::core::ptr::null::<
    ::core::ffi::c_char,
>() as *mut ::core::ffi::c_char;
#[no_mangle]
pub static mut golabel: [::core::ffi::c_char; 1024] = unsafe {
    ::core::mem::transmute::<
        [u8; 1024],
        [::core::ffi::c_char; 1024],
    >(
        *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    )
};
#[no_mangle]
pub static mut execlevel: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut eolexist: ::core::ffi::c_int = TRUE;
#[no_mangle]
pub static mut revexist: ::core::ffi::c_int = FALSE;
#[no_mangle]
pub static mut flickcode: ::core::ffi::c_int = FALSE;
#[no_mangle]
pub static mut modename: [*mut ::core::ffi::c_char; 9] = [
    b"WRAP\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"CMODE\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"SPELL\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"EXACT\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"VIEW\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"OVER\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"MAGIC\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"ASAVE\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"UTF-8\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut mode2name: [*mut ::core::ffi::c_char; 9] = [
    b"Wrap\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Cmode\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Spell\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Exact\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"View\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Over\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Magic\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"Asave\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"utf-8\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut modecode: [::core::ffi::c_char; 11] = unsafe {
    ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"WCSEVOMYAU\0")
};
#[no_mangle]
pub static mut gmode: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut gflags: ::core::ffi::c_int = GFREAD;
#[no_mangle]
pub static mut gfcolor: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
#[no_mangle]
pub static mut gbcolor: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut gasave: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
#[no_mangle]
pub static mut gacount: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
#[no_mangle]
pub static mut sgarbf: ::core::ffi::c_int = TRUE;
#[no_mangle]
pub static mut mpresf: ::core::ffi::c_int = FALSE;
#[no_mangle]
pub static mut clexec: ::core::ffi::c_int = FALSE;
#[no_mangle]
pub static mut mstore: ::core::ffi::c_int = FALSE;
#[no_mangle]
pub static mut discmd: ::core::ffi::c_int = TRUE;
#[no_mangle]
pub static mut disinp: ::core::ffi::c_int = TRUE;
#[no_mangle]
pub static mut bstore: *mut buffer = ::core::ptr::null::<buffer>() as *mut buffer;
#[no_mangle]
pub static mut vtrow: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut vtcol: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut ttrow: ::core::ffi::c_int = HUGE;
#[no_mangle]
pub static mut ttcol: ::core::ffi::c_int = HUGE;
#[no_mangle]
pub static mut lbound: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut taboff: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut metac: ::core::ffi::c_int = CONTROL | '[' as i32;
#[no_mangle]
pub static mut ctlxc: ::core::ffi::c_int = CONTROL | 'X' as i32;
#[no_mangle]
pub static mut reptc: ::core::ffi::c_int = CONTROL | 'U' as i32;
#[no_mangle]
pub static mut abortc: ::core::ffi::c_int = 0x1f as ::core::ffi::c_int;
#[no_mangle]
pub static mut quotec: ::core::ffi::c_int = 0x11 as ::core::ffi::c_int;
#[no_mangle]
pub static mut tab_width: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
#[no_mangle]
pub static mut kbufp: *mut kill = ::core::ptr::null::<kill>() as *mut kill;
#[no_mangle]
pub static mut kbufh: *mut kill = ::core::ptr::null::<kill>() as *mut kill;
#[no_mangle]
pub static mut swindow: *mut window = ::core::ptr::null::<window>() as *mut window;
#[no_mangle]
pub static mut kbdmode: ::core::ffi::c_int = STOP;
#[no_mangle]
pub static mut kbdrep: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut restflag: ::core::ffi::c_int = FALSE;
#[no_mangle]
pub static mut lastkey: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut seed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut envram: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
#[no_mangle]
pub static mut macbug: ::core::ffi::c_int = FALSE;
#[no_mangle]
pub static mut cmdstatus: ::core::ffi::c_int = TRUE;
#[no_mangle]
pub static mut palstr: [::core::ffi::c_char; 49] = unsafe {
    ::core::mem::transmute::<
        [u8; 49],
        [::core::ffi::c_char; 49],
    >(
        *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    )
};
#[no_mangle]
pub static mut saveflag: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut fline: *mut ::core::ffi::c_char = ::core::ptr::null::<
    ::core::ffi::c_char,
>() as *mut ::core::ffi::c_char;
#[no_mangle]
pub static mut flen: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut nullflag: ::core::ffi::c_int = FALSE;
#[no_mangle]
pub static mut term: *mut terminal = ::core::ptr::null::<terminal>() as *mut terminal;
#[no_mangle]
pub static mut justflag: ::core::ffi::c_int = FALSE;
#[no_mangle]
pub static mut overlap: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut scrollcount: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub static mut currow: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut curcol: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut thisflag: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut lastflag: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut curgoal: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut curwp: *mut window = ::core::ptr::null::<window>() as *mut window;
#[no_mangle]
pub static mut curbp: *mut buffer = ::core::ptr::null::<buffer>() as *mut buffer;
#[no_mangle]
pub static mut bheadp: *mut buffer = ::core::ptr::null::<buffer>() as *mut buffer;
#[no_mangle]
pub static mut blistp: *mut buffer = ::core::ptr::null::<buffer>() as *mut buffer;
#[no_mangle]
pub static mut sres: [::core::ffi::c_char; 16] = [0; 16];
#[no_mangle]
pub static mut pat: [::core::ffi::c_char; 1024] = [0; 1024];
#[no_mangle]
pub static mut tap: [::core::ffi::c_char; 1024] = [0; 1024];
#[no_mangle]
pub static mut rpat: [::core::ffi::c_char; 1024] = [0; 1024];
#[no_mangle]
pub static mut patmatch: *mut ::core::ffi::c_char = ::core::ptr::null::<
    ::core::ffi::c_char,
>() as *mut ::core::ffi::c_char;
#[no_mangle]
pub static mut matchline: *mut line = ::core::ptr::null::<line>() as *mut line;
#[no_mangle]
pub static mut matchoff: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut cutln_active: ::core::ffi::c_int = FALSE;
#[no_mangle]
pub static mut confirmshell: ::core::ffi::c_int = TRUE;
#[no_mangle]
pub static mut makebackup: ::core::ffi::c_int = TRUE;
#[no_mangle]
pub static mut removebackup: ::core::ffi::c_int = FALSE;
#[no_mangle]
pub static mut indent_start_lp: *mut line = ::core::ptr::null::<line>() as *mut line;
#[no_mangle]
pub static mut indent_end_lp: *mut line = ::core::ptr::null::<line>() as *mut line;
#[no_mangle]
pub static mut indent_range_type: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut indent_selection_active: ::core::ffi::c_int = FALSE;
#[no_mangle]
pub static mut dname: [*mut ::core::ffi::c_char; 10] = [
    b"if\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"else\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"endif\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"goto\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"return\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"endm\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"while\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"endwhile\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"break\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"force\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut kbdm: [::core::ffi::c_int; 2048] = [0; 2048];
#[no_mangle]
pub static mut kbdptr: *mut ::core::ffi::c_int = ::core::ptr::null::<
    ::core::ffi::c_int,
>() as *mut ::core::ffi::c_int;
#[no_mangle]
pub static mut kbdend: *mut ::core::ffi::c_int = ::core::ptr::null::<
    ::core::ffi::c_int,
>() as *mut ::core::ffi::c_int;
#[no_mangle]
pub static mut matchlen: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
#[no_mangle]
pub static mut mlenold: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
#[no_mangle]
pub static mut errorm: [::core::ffi::c_char; 6] = unsafe {
    ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"ERROR\0")
};
#[no_mangle]
pub static mut truem: [::core::ffi::c_char; 5] = unsafe {
    ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"TRUE\0")
};
#[no_mangle]
pub static mut falsem: [::core::ffi::c_char; 6] = unsafe {
    ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"FALSE\0")
};
#[no_mangle]
pub static mut mcpat: [magic; 1024] = [magic {
    mc_type: 0,
    u: C2RustUnnamed { lchar: 0 },
}; 1024];
#[no_mangle]
pub static mut tapcm: [magic; 1024] = [magic {
    mc_type: 0,
    u: C2RustUnnamed { lchar: 0 },
}; 1024];
#[no_mangle]
pub static mut kused: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut numlocks: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut lname: [*mut ::core::ffi::c_char; 1000] = [::core::ptr::null::<
    ::core::ffi::c_char,
>() as *mut ::core::ffi::c_char; 1000];
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
