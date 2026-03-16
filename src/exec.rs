extern "C" {
    fn atoi(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
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
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    static mut execstr: *mut ::core::ffi::c_char;
    static mut golabel: [::core::ffi::c_char; 0];
    static mut execlevel: ::core::ffi::c_int;
    static mut thisflag: ::core::ffi::c_int;
    static mut lastflag: ::core::ffi::c_int;
    static mut curwp: *mut window;
    static mut curbp: *mut buffer;
    static mut cmdstatus: ::core::ffi::c_int;
    static mut clexec: ::core::ffi::c_int;
    static mut mstore: ::core::ffi::c_int;
    static mut bstore: *mut buffer;
    fn mlwrite(fmt: *const ::core::ffi::c_char, ...);
    fn ctoec(c: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn getname() -> fn_t;
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
    fn fncmatch(_: *mut ::core::ffi::c_char) -> fn_t;
    fn zotbuf(bp: *mut buffer) -> ::core::ffi::c_int;
    fn bclear(bp: *mut buffer) -> ::core::ffi::c_int;
    fn bfind(
        bname: *mut ::core::ffi::c_char,
        cflag: ::core::ffi::c_int,
        bflag: ::core::ffi::c_int,
    ) -> *mut buffer;
    fn readin(
        fname: *mut ::core::ffi::c_char,
        lockfl: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn makename(bname: *mut ::core::ffi::c_char, fname: *mut ::core::ffi::c_char);
    fn unqname(name: *mut ::core::ffi::c_char);
    fn minibuf_input(
        prompt: *const ::core::ffi::c_char,
        dest: *mut ::core::ffi::c_char,
        max_len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn gettyp(token_0: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn getval(
        token_0: *mut ::core::ffi::c_char,
        result: *mut ::core::ffi::c_char,
        size: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn stol(val: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn lalloc(_: ::core::ffi::c_int) -> *mut line;
}
pub type size_t = usize;
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
pub struct while_block {
    pub w_begin: *mut line,
    pub w_end: *mut line,
    pub w_type: ::core::ffi::c_int,
    pub w_next: *mut while_block,
}
pub type fn_t = Option<
    unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int) -> ::core::ffi::c_int,
>;
pub const NBUFN: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const NSTRING: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const NPAT: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DIF: ::core::ffi::c_int = 0;
pub const DELSE: ::core::ffi::c_int = 1;
pub const DENDIF: ::core::ffi::c_int = 2;
pub const DGOTO: ::core::ffi::c_int = 3;
pub const DRETURN: ::core::ffi::c_int = 4;
pub const DENDM: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const DWHILE: ::core::ffi::c_int = 6;
pub const DENDWHILE: ::core::ffi::c_int = 7;
pub const DBREAK: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const DFORCE: ::core::ffi::c_int = 9;
pub const NUMDIRS: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const TKCMD: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const WFHARD: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const BFINVS: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const MDVIEW: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const BTWHILE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BTBREAK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
static mut dname: [*mut ::core::ffi::c_char; 10] = [
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
pub unsafe extern "C" fn namedcmd(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut kfunc: fn_t = None;
    mlwrite(b": \0" as *const u8 as *const ::core::ffi::c_char);
    kfunc = getname();
    if kfunc.is_none() {
        mlwrite(b"(No such function)\0" as *const u8 as *const ::core::ffi::c_char);
        return FALSE;
    }
    return kfunc.expect("non-null function pointer")(f, n);
}
#[no_mangle]
pub unsafe extern "C" fn execcmd(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut status: ::core::ffi::c_int = 0;
    let mut cmdstr: [::core::ffi::c_char; 1024] = [0; 1024];
    status = minibuf_input(
        b": \0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut cmdstr as *mut ::core::ffi::c_char,
        NSTRING,
    );
    if status != TRUE {
        return status;
    }
    execlevel = 0 as ::core::ffi::c_int;
    return docmd(&raw mut cmdstr as *mut ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn docmd(
    mut cline: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut f: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut fnc: fn_t = None;
    let mut status: ::core::ffi::c_int = 0;
    let mut oldcle: ::core::ffi::c_int = 0;
    let mut oldestr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut tkn: [::core::ffi::c_char; 1024] = [0; 1024];
    if execlevel != 0 {
        return TRUE;
    }
    oldestr = execstr;
    execstr = cline;
    f = FALSE;
    n = 1 as ::core::ffi::c_int;
    lastflag = thisflag;
    thisflag = 0 as ::core::ffi::c_int;
    status = macarg(&raw mut tkn as *mut ::core::ffi::c_char);
    if status != TRUE {
        execstr = oldestr;
        return status;
    }
    if gettyp(&raw mut tkn as *mut ::core::ffi::c_char) != TKCMD {
        f = TRUE;
        getval(
            &raw mut tkn as *mut ::core::ffi::c_char,
            &raw mut tkn as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as ::core::ffi::c_int,
        );
        n = atoi(&raw mut tkn as *mut ::core::ffi::c_char);
        status = macarg(&raw mut tkn as *mut ::core::ffi::c_char);
        if status != TRUE {
            execstr = oldestr;
            return status;
        }
    }
    fnc = fncmatch(&raw mut tkn as *mut ::core::ffi::c_char);
    if fnc.is_none() {
        mlwrite(b"(No such Function)\0" as *const u8 as *const ::core::ffi::c_char);
        execstr = oldestr;
        return FALSE;
    }
    oldcle = clexec;
    clexec = TRUE;
    status = Some(fnc.expect("non-null function pointer"))
        .expect("non-null function pointer")(f, n);
    cmdstatus = status;
    clexec = oldcle;
    execstr = oldestr;
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn token(
    mut src: *mut ::core::ffi::c_char,
    mut tok: *mut ::core::ffi::c_char,
    mut size: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut quotef: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_char = 0;
    while *src as ::core::ffi::c_int == ' ' as i32
        || *src as ::core::ffi::c_int == '\t' as i32
    {
        src = src.offset(1);
    }
    quotef = FALSE;
    while *src != 0 {
        if *src as ::core::ffi::c_int == '~' as i32 {
            src = src.offset(1);
            if *src as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                break;
            }
            let fresh0 = src;
            src = src.offset(1);
            match *fresh0 as ::core::ffi::c_int {
                114 => {
                    c = 13 as ::core::ffi::c_char;
                }
                110 => {
                    c = 10 as ::core::ffi::c_char;
                }
                116 => {
                    c = 9 as ::core::ffi::c_char;
                }
                98 => {
                    c = 8 as ::core::ffi::c_char;
                }
                102 => {
                    c = 12 as ::core::ffi::c_char;
                }
                _ => {
                    c = *src.offset(-(1 as ::core::ffi::c_int as isize));
                }
            }
            size -= 1;
            if size > 0 as ::core::ffi::c_int {
                let fresh1 = tok;
                tok = tok.offset(1);
                *fresh1 = c;
            }
        } else {
            if quotef != 0 {
                if *src as ::core::ffi::c_int == '"' as i32 {
                    break;
                }
            } else if *src as ::core::ffi::c_int == ' ' as i32
                || *src as ::core::ffi::c_int == '\t' as i32
            {
                break;
            }
            if *src as ::core::ffi::c_int == '"' as i32 {
                quotef = TRUE;
            }
            let fresh2 = src;
            src = src.offset(1);
            c = *fresh2;
            size -= 1;
            if size > 0 as ::core::ffi::c_int {
                let fresh3 = tok;
                tok = tok.offset(1);
                *fresh3 = c;
            }
        }
    }
    if *src != 0 {
        src = src.offset(1);
    }
    *tok = 0 as ::core::ffi::c_char;
    return src;
}
#[no_mangle]
pub unsafe extern "C" fn macarg(
    mut tok: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut savcle: ::core::ffi::c_int = 0;
    let mut status: ::core::ffi::c_int = 0;
    savcle = clexec;
    clexec = TRUE;
    status = nextarg(
        b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        tok,
        NSTRING,
        ctoec('\n' as i32),
    );
    clexec = savcle;
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn nextarg(
    mut prompt: *mut ::core::ffi::c_char,
    mut buffer: *mut ::core::ffi::c_char,
    mut size: ::core::ffi::c_int,
    mut terminator: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if clexec == FALSE {
        return getstring(prompt, buffer, size, terminator);
    }
    execstr = token(execstr, buffer, size);
    getval(buffer, buffer, size);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn storemac(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut bp: *mut buffer = ::core::ptr::null_mut::<buffer>();
    let mut bname: [::core::ffi::c_char; 16] = [0; 16];
    if f == FALSE {
        mlwrite(b"No macro specified\0" as *const u8 as *const ::core::ffi::c_char);
        return FALSE;
    }
    if n < 1 as ::core::ffi::c_int || n > 40 as ::core::ffi::c_int {
        mlwrite(
            b"Macro number out of range\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return FALSE;
    }
    strcpy(
        &raw mut bname as *mut ::core::ffi::c_char,
        b"*Macro xx*\0" as *const u8 as *const ::core::ffi::c_char,
    );
    bname[7 as ::core::ffi::c_int as usize] = ('0' as i32 + n / 10 as ::core::ffi::c_int)
        as ::core::ffi::c_char;
    bname[8 as ::core::ffi::c_int as usize] = ('0' as i32 + n % 10 as ::core::ffi::c_int)
        as ::core::ffi::c_char;
    bp = bfind(&raw mut bname as *mut ::core::ffi::c_char, TRUE, BFINVS);
    if bp.is_null() {
        mlwrite(b"Can not create macro\0" as *const u8 as *const ::core::ffi::c_char);
        return FALSE;
    }
    bclear(bp);
    mstore = TRUE;
    bstore = bp;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn storeproc(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut bp: *mut buffer = ::core::ptr::null_mut::<buffer>();
    let mut status: ::core::ffi::c_int = 0;
    let mut bname: [::core::ffi::c_char; 16] = [0; 16];
    if f == TRUE {
        return storemac(f, n);
    }
    status = minibuf_input(
        b"Procedure name: \0" as *const u8 as *const ::core::ffi::c_char,
        (&raw mut bname as *mut ::core::ffi::c_char)
            .offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
        NBUFN - 2 as ::core::ffi::c_int,
    );
    if status != TRUE {
        return status;
    }
    bname[0 as ::core::ffi::c_int as usize] = '*' as i32 as ::core::ffi::c_char;
    strcat(
        &raw mut bname as *mut ::core::ffi::c_char,
        b"*\0" as *const u8 as *const ::core::ffi::c_char,
    );
    bp = bfind(&raw mut bname as *mut ::core::ffi::c_char, TRUE, BFINVS);
    if bp.is_null() {
        mlwrite(b"Can not create macro\0" as *const u8 as *const ::core::ffi::c_char);
        return FALSE;
    }
    bclear(bp);
    mstore = TRUE;
    bstore = bp;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn execproc(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut bp: *mut buffer = ::core::ptr::null_mut::<buffer>();
    let mut status: ::core::ffi::c_int = 0;
    let mut bufn: [::core::ffi::c_char; 18] = [0; 18];
    status = minibuf_input(
        b"Execute procedure: \0" as *const u8 as *const ::core::ffi::c_char,
        (&raw mut bufn as *mut ::core::ffi::c_char)
            .offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
        NBUFN,
    );
    if status != TRUE {
        return status;
    }
    bufn[0 as ::core::ffi::c_int as usize] = '*' as i32 as ::core::ffi::c_char;
    strcat(
        &raw mut bufn as *mut ::core::ffi::c_char,
        b"*\0" as *const u8 as *const ::core::ffi::c_char,
    );
    bp = bfind(
        &raw mut bufn as *mut ::core::ffi::c_char,
        FALSE,
        0 as ::core::ffi::c_int,
    );
    if bp.is_null() {
        mlwrite(b"No such procedure\0" as *const u8 as *const ::core::ffi::c_char);
        return FALSE;
    }
    loop {
        let fresh4 = n;
        n = n - 1;
        if !(fresh4 > 0 as ::core::ffi::c_int) {
            break;
        }
        status = dobuf(bp);
        if status != TRUE {
            return status;
        }
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn execbuf(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut bp: *mut buffer = ::core::ptr::null_mut::<buffer>();
    let mut status: ::core::ffi::c_int = 0;
    let mut bufn: [::core::ffi::c_char; 1024] = [0; 1024];
    status = minibuf_input(
        b"Execute buffer: \0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut bufn as *mut ::core::ffi::c_char,
        NBUFN,
    );
    if status != TRUE {
        return status;
    }
    bp = bfind(
        &raw mut bufn as *mut ::core::ffi::c_char,
        FALSE,
        0 as ::core::ffi::c_int,
    );
    if bp.is_null() {
        mlwrite(b"No such buffer\0" as *const u8 as *const ::core::ffi::c_char);
        return FALSE;
    }
    loop {
        let fresh6 = n;
        n = n - 1;
        if !(fresh6 > 0 as ::core::ffi::c_int) {
            break;
        }
        status = dobuf(bp);
        if status != TRUE {
            return status;
        }
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn dobuf(mut bp: *mut buffer) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut status: ::core::ffi::c_int = 0;
    let mut lp: *mut line = ::core::ptr::null_mut::<line>();
    let mut hlp: *mut line = ::core::ptr::null_mut::<line>();
    let mut glp: *mut line = ::core::ptr::null_mut::<line>();
    let mut mp: *mut line = ::core::ptr::null_mut::<line>();
    let mut dirnum: ::core::ffi::c_int = 0;
    let mut linlen: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut force: ::core::ffi::c_int = 0;
    let mut wp: *mut window = ::core::ptr::null_mut::<window>();
    let mut whlist: *mut while_block = ::core::ptr::null_mut::<while_block>();
    let mut scanner: *mut while_block = ::core::ptr::null_mut::<while_block>();
    let mut whtemp: *mut while_block = ::core::ptr::null_mut::<while_block>();
    let mut einit: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut eline: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut tkn: [::core::ffi::c_char; 1024] = [0; 1024];
    execlevel = 0 as ::core::ffi::c_int;
    whlist = ::core::ptr::null_mut::<while_block>();
    scanner = ::core::ptr::null_mut::<while_block>();
    hlp = (*bp).b_linep;
    lp = (*hlp).l_fp;
    loop {
        if lp != hlp {
            eline = &raw mut (*lp).l_text as *mut ::core::ffi::c_uchar
                as *mut ::core::ffi::c_char;
            i = (*lp).l_used;
            loop {
                let fresh5 = i;
                i = i - 1;
                if !(fresh5 > 0 as ::core::ffi::c_int
                    && (*eline as ::core::ffi::c_int == ' ' as i32
                        || *eline as ::core::ffi::c_int == '\t' as i32))
                {
                    break;
                }
                eline = eline.offset(1);
            }
            if !(i <= 0 as ::core::ffi::c_int) {
                if *eline.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '!' as i32
                    && *eline.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == 'w' as i32
                    && *eline.offset(2 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == 'h' as i32
                {
                    whtemp = malloc(::core::mem::size_of::<while_block>() as size_t)
                        as *mut while_block;
                    if whtemp.is_null() {
                        current_block = 4503586385541179488;
                        break;
                    }
                    (*whtemp).w_begin = lp;
                    (*whtemp).w_type = BTWHILE;
                    (*whtemp).w_next = scanner;
                    scanner = whtemp;
                }
                if *eline.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '!' as i32
                    && *eline.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == 'b' as i32
                    && *eline.offset(2 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int == 'r' as i32
                {
                    if scanner.is_null() {
                        mlwrite(
                            b"%%!BREAK outside of any !WHILE loop\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        current_block = 16419820427799594835;
                        break;
                    } else {
                        whtemp = malloc(::core::mem::size_of::<while_block>() as size_t)
                            as *mut while_block;
                        if whtemp.is_null() {
                            current_block = 4503586385541179488;
                            break;
                        }
                        (*whtemp).w_begin = lp;
                        (*whtemp).w_type = BTBREAK;
                        (*whtemp).w_next = scanner;
                        scanner = whtemp;
                    }
                }
                if *eline.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '!' as i32
                    && strncmp(
                        eline.offset(1 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_char,
                        b"endw\0" as *const u8 as *const ::core::ffi::c_char,
                        4 as size_t,
                    ) == 0 as ::core::ffi::c_int
                {
                    if scanner.is_null() {
                        mlwrite(
                            b"%%!ENDWHILE with no preceding !WHILE in '%s'\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            &raw mut (*bp).b_bname as *mut ::core::ffi::c_char,
                        );
                        current_block = 16419820427799594835;
                        break;
                    } else {
                        loop {
                            (*scanner).w_end = lp;
                            whtemp = whlist;
                            whlist = scanner;
                            scanner = (*scanner).w_next;
                            (*whlist).w_next = whtemp;
                            if !((*whlist).w_type == BTBREAK) {
                                break;
                            }
                        }
                    }
                }
            }
            lp = (*lp).l_fp;
        } else if !scanner.is_null() {
            current_block = 7427571413727699167;
            break;
        } else {
            current_block = 5330834795799507926;
            break;
        }
    }
    match current_block {
        5330834795799507926 => {
            thisflag = lastflag;
            hlp = (*bp).b_linep;
            lp = (*hlp).l_fp;
            while lp != hlp {
                linlen = (*lp).l_used;
                eline = malloc((linlen + 1 as ::core::ffi::c_int) as size_t)
                    as *mut ::core::ffi::c_char;
                einit = eline;
                if einit.is_null() {
                    mlwrite(
                        b"%%Out of Memory during macro execution\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    freewhile(whlist);
                    return FALSE;
                }
                strncpy(
                    eline,
                    &raw mut (*lp).l_text as *mut ::core::ffi::c_uchar
                        as *const ::core::ffi::c_char,
                    linlen as size_t,
                );
                *eline.offset(linlen as isize) = 0 as ::core::ffi::c_char;
                while *eline as ::core::ffi::c_int == ' ' as i32
                    || *eline as ::core::ffi::c_int == '\t' as i32
                {
                    eline = eline.offset(1);
                }
                if !(*eline as ::core::ffi::c_int == ';' as i32
                    || *eline as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
                {
                    dirnum = -(1 as ::core::ffi::c_int);
                    if *eline as ::core::ffi::c_int == '!' as i32 {
                        eline = eline.offset(1);
                        dirnum = 0 as ::core::ffi::c_int;
                        while dirnum < NUMDIRS {
                            if strncmp(
                                eline,
                                dname[dirnum as usize],
                                strlen(dname[dirnum as usize]),
                            ) == 0 as ::core::ffi::c_int
                            {
                                break;
                            }
                            dirnum += 1;
                        }
                        if dirnum == NUMDIRS {
                            mlwrite(
                                b"%%Unknown Directive\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                            freewhile(whlist);
                            return FALSE;
                        }
                        if dirnum == DENDM {
                            mstore = FALSE;
                            bstore = ::core::ptr::null_mut::<buffer>();
                            current_block = 15568809317923174358;
                        } else {
                            eline = eline.offset(-1);
                            current_block = 1134115459065347084;
                        }
                    } else {
                        current_block = 1134115459065347084;
                    }
                    match current_block {
                        15568809317923174358 => {}
                        _ => {
                            if mstore != 0 {
                                linlen = strlen(eline) as ::core::ffi::c_int;
                                mp = lalloc(linlen);
                                if mp.is_null() {
                                    mlwrite(
                                        b"Out of memory while storing macro\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                    return FALSE;
                                }
                                i = 0 as ::core::ffi::c_int;
                                while i < linlen {
                                    *(&raw mut (*mp).l_text as *mut ::core::ffi::c_uchar)
                                        .offset(i as isize) = *eline.offset(i as isize)
                                        as ::core::ffi::c_uchar;
                                    i += 1;
                                }
                                (*(*(*bstore).b_linep).l_bp).l_fp = mp;
                                (*mp).l_bp = (*(*bstore).b_linep).l_bp;
                                (*(*bstore).b_linep).l_bp = mp;
                                (*mp).l_fp = (*bstore).b_linep;
                            } else {
                                force = FALSE;
                                if !(*eline as ::core::ffi::c_int == '*' as i32) {
                                    if dirnum != -(1 as ::core::ffi::c_int) {
                                        while *eline as ::core::ffi::c_int != 0
                                            && *eline as ::core::ffi::c_int != ' ' as i32
                                            && *eline as ::core::ffi::c_int != '\t' as i32
                                        {
                                            eline = eline.offset(1);
                                        }
                                        execstr = eline;
                                        match dirnum {
                                            DIF => {
                                                current_block = 9281452336402489912;
                                                match current_block {
                                                    9281452336402489912 => {
                                                        if execlevel == 0 as ::core::ffi::c_int {
                                                            if macarg(&raw mut tkn as *mut ::core::ffi::c_char) != TRUE
                                                            {
                                                                break;
                                                            }
                                                            if stol(&raw mut tkn as *mut ::core::ffi::c_char) == FALSE {
                                                                execlevel += 1;
                                                            }
                                                        } else {
                                                            execlevel += 1;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    15988712159247554322 => {
                                                        if execlevel == 0 as ::core::ffi::c_int {
                                                            if macarg(&raw mut tkn as *mut ::core::ffi::c_char) != TRUE
                                                            {
                                                                break;
                                                            }
                                                            if stol(&raw mut tkn as *mut ::core::ffi::c_char) == TRUE {
                                                                current_block = 15568809317923174358;
                                                            } else {
                                                                current_block = 2750570471926810434;
                                                            }
                                                        } else {
                                                            current_block = 2750570471926810434;
                                                        }
                                                        match current_block {
                                                            15568809317923174358 => {}
                                                            _ => {
                                                                current_block = 5976146557728084725;
                                                            }
                                                        }
                                                    }
                                                    18195282705153904920 => {
                                                        if execlevel == 0 as ::core::ffi::c_int {
                                                            eline = token(
                                                                eline,
                                                                &raw mut golabel as *mut ::core::ffi::c_char,
                                                                NPAT,
                                                            );
                                                            linlen = strlen(
                                                                &raw mut golabel as *mut ::core::ffi::c_char,
                                                            ) as ::core::ffi::c_int;
                                                            glp = (*hlp).l_fp;
                                                            loop {
                                                                if !(glp != hlp) {
                                                                    current_block = 10109057886293123569;
                                                                    break;
                                                                }
                                                                if *(&raw mut (*glp).l_text as *mut ::core::ffi::c_uchar)
                                                                    as ::core::ffi::c_int == '*' as i32
                                                                    && strncmp(
                                                                        (&raw mut (*glp).l_text as *mut ::core::ffi::c_uchar)
                                                                            .offset(1 as ::core::ffi::c_int as isize)
                                                                            as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_char,
                                                                        &raw mut golabel as *mut ::core::ffi::c_char,
                                                                        linlen as size_t,
                                                                    ) == 0 as ::core::ffi::c_int
                                                                {
                                                                    lp = glp;
                                                                    current_block = 15568809317923174358;
                                                                    break;
                                                                } else {
                                                                    glp = (*glp).l_fp;
                                                                }
                                                            }
                                                            match current_block {
                                                                15568809317923174358 => {}
                                                                _ => {
                                                                    mlwrite(
                                                                        b"%%No such label\0" as *const u8
                                                                            as *const ::core::ffi::c_char,
                                                                    );
                                                                    freewhile(whlist);
                                                                    return FALSE;
                                                                }
                                                            }
                                                        } else {
                                                            current_block = 15568809317923174358;
                                                        }
                                                    }
                                                    2430319949834955361 => {
                                                        if execlevel != 0 {
                                                            execlevel -= 1;
                                                        } else {
                                                            whtemp = whlist;
                                                            while !whtemp.is_null() {
                                                                if (*whtemp).w_type == BTWHILE && (*whtemp).w_end == lp {
                                                                    break;
                                                                }
                                                                whtemp = (*whtemp).w_next;
                                                            }
                                                            if whtemp.is_null() {
                                                                mlwrite(
                                                                    b"%%Internal While loop error\0" as *const u8
                                                                        as *const ::core::ffi::c_char,
                                                                );
                                                                freewhile(whlist);
                                                                return FALSE;
                                                            }
                                                            lp = (*(*whtemp).w_begin).l_bp;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    14180127615403059768 => {
                                                        force = TRUE;
                                                        current_block = 4736343053266048700;
                                                    }
                                                    10703124919230875173 => {
                                                        if execlevel == 1 as ::core::ffi::c_int {
                                                            execlevel -= 1;
                                                        } else if execlevel == 0 as ::core::ffi::c_int {
                                                            execlevel += 1;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    6246763041535604849 => {
                                                        if execlevel != 0 {
                                                            execlevel -= 1;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    14928940103145537714 => {
                                                        if execlevel == 0 as ::core::ffi::c_int {
                                                            break;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    _ => {}
                                                }
                                                match current_block {
                                                    15568809317923174358 => {}
                                                    4736343053266048700 => {}
                                                    _ => {
                                                        if dirnum == DBREAK && execlevel != 0 {
                                                            current_block = 15568809317923174358;
                                                        } else {
                                                            whtemp = whlist;
                                                            while !whtemp.is_null() {
                                                                if (*whtemp).w_begin == lp {
                                                                    break;
                                                                }
                                                                whtemp = (*whtemp).w_next;
                                                            }
                                                            if whtemp.is_null() {
                                                                mlwrite(
                                                                    b"%%Internal While loop error\0" as *const u8
                                                                        as *const ::core::ffi::c_char,
                                                                );
                                                                freewhile(whlist);
                                                                return FALSE;
                                                            }
                                                            lp = (*whtemp).w_end;
                                                            current_block = 15568809317923174358;
                                                        }
                                                    }
                                                }
                                            }
                                            DWHILE => {
                                                current_block = 15988712159247554322;
                                                match current_block {
                                                    9281452336402489912 => {
                                                        if execlevel == 0 as ::core::ffi::c_int {
                                                            if macarg(&raw mut tkn as *mut ::core::ffi::c_char) != TRUE
                                                            {
                                                                break;
                                                            }
                                                            if stol(&raw mut tkn as *mut ::core::ffi::c_char) == FALSE {
                                                                execlevel += 1;
                                                            }
                                                        } else {
                                                            execlevel += 1;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    15988712159247554322 => {
                                                        if execlevel == 0 as ::core::ffi::c_int {
                                                            if macarg(&raw mut tkn as *mut ::core::ffi::c_char) != TRUE
                                                            {
                                                                break;
                                                            }
                                                            if stol(&raw mut tkn as *mut ::core::ffi::c_char) == TRUE {
                                                                current_block = 15568809317923174358;
                                                            } else {
                                                                current_block = 2750570471926810434;
                                                            }
                                                        } else {
                                                            current_block = 2750570471926810434;
                                                        }
                                                        match current_block {
                                                            15568809317923174358 => {}
                                                            _ => {
                                                                current_block = 5976146557728084725;
                                                            }
                                                        }
                                                    }
                                                    18195282705153904920 => {
                                                        if execlevel == 0 as ::core::ffi::c_int {
                                                            eline = token(
                                                                eline,
                                                                &raw mut golabel as *mut ::core::ffi::c_char,
                                                                NPAT,
                                                            );
                                                            linlen = strlen(
                                                                &raw mut golabel as *mut ::core::ffi::c_char,
                                                            ) as ::core::ffi::c_int;
                                                            glp = (*hlp).l_fp;
                                                            loop {
                                                                if !(glp != hlp) {
                                                                    current_block = 10109057886293123569;
                                                                    break;
                                                                }
                                                                if *(&raw mut (*glp).l_text as *mut ::core::ffi::c_uchar)
                                                                    as ::core::ffi::c_int == '*' as i32
                                                                    && strncmp(
                                                                        (&raw mut (*glp).l_text as *mut ::core::ffi::c_uchar)
                                                                            .offset(1 as ::core::ffi::c_int as isize)
                                                                            as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_char,
                                                                        &raw mut golabel as *mut ::core::ffi::c_char,
                                                                        linlen as size_t,
                                                                    ) == 0 as ::core::ffi::c_int
                                                                {
                                                                    lp = glp;
                                                                    current_block = 15568809317923174358;
                                                                    break;
                                                                } else {
                                                                    glp = (*glp).l_fp;
                                                                }
                                                            }
                                                            match current_block {
                                                                15568809317923174358 => {}
                                                                _ => {
                                                                    mlwrite(
                                                                        b"%%No such label\0" as *const u8
                                                                            as *const ::core::ffi::c_char,
                                                                    );
                                                                    freewhile(whlist);
                                                                    return FALSE;
                                                                }
                                                            }
                                                        } else {
                                                            current_block = 15568809317923174358;
                                                        }
                                                    }
                                                    2430319949834955361 => {
                                                        if execlevel != 0 {
                                                            execlevel -= 1;
                                                        } else {
                                                            whtemp = whlist;
                                                            while !whtemp.is_null() {
                                                                if (*whtemp).w_type == BTWHILE && (*whtemp).w_end == lp {
                                                                    break;
                                                                }
                                                                whtemp = (*whtemp).w_next;
                                                            }
                                                            if whtemp.is_null() {
                                                                mlwrite(
                                                                    b"%%Internal While loop error\0" as *const u8
                                                                        as *const ::core::ffi::c_char,
                                                                );
                                                                freewhile(whlist);
                                                                return FALSE;
                                                            }
                                                            lp = (*(*whtemp).w_begin).l_bp;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    14180127615403059768 => {
                                                        force = TRUE;
                                                        current_block = 4736343053266048700;
                                                    }
                                                    10703124919230875173 => {
                                                        if execlevel == 1 as ::core::ffi::c_int {
                                                            execlevel -= 1;
                                                        } else if execlevel == 0 as ::core::ffi::c_int {
                                                            execlevel += 1;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    6246763041535604849 => {
                                                        if execlevel != 0 {
                                                            execlevel -= 1;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    14928940103145537714 => {
                                                        if execlevel == 0 as ::core::ffi::c_int {
                                                            break;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    _ => {}
                                                }
                                                match current_block {
                                                    15568809317923174358 => {}
                                                    4736343053266048700 => {}
                                                    _ => {
                                                        if dirnum == DBREAK && execlevel != 0 {
                                                            current_block = 15568809317923174358;
                                                        } else {
                                                            whtemp = whlist;
                                                            while !whtemp.is_null() {
                                                                if (*whtemp).w_begin == lp {
                                                                    break;
                                                                }
                                                                whtemp = (*whtemp).w_next;
                                                            }
                                                            if whtemp.is_null() {
                                                                mlwrite(
                                                                    b"%%Internal While loop error\0" as *const u8
                                                                        as *const ::core::ffi::c_char,
                                                                );
                                                                freewhile(whlist);
                                                                return FALSE;
                                                            }
                                                            lp = (*whtemp).w_end;
                                                            current_block = 15568809317923174358;
                                                        }
                                                    }
                                                }
                                            }
                                            DBREAK => {
                                                current_block = 5976146557728084725;
                                                match current_block {
                                                    9281452336402489912 => {
                                                        if execlevel == 0 as ::core::ffi::c_int {
                                                            if macarg(&raw mut tkn as *mut ::core::ffi::c_char) != TRUE
                                                            {
                                                                break;
                                                            }
                                                            if stol(&raw mut tkn as *mut ::core::ffi::c_char) == FALSE {
                                                                execlevel += 1;
                                                            }
                                                        } else {
                                                            execlevel += 1;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    15988712159247554322 => {
                                                        if execlevel == 0 as ::core::ffi::c_int {
                                                            if macarg(&raw mut tkn as *mut ::core::ffi::c_char) != TRUE
                                                            {
                                                                break;
                                                            }
                                                            if stol(&raw mut tkn as *mut ::core::ffi::c_char) == TRUE {
                                                                current_block = 15568809317923174358;
                                                            } else {
                                                                current_block = 2750570471926810434;
                                                            }
                                                        } else {
                                                            current_block = 2750570471926810434;
                                                        }
                                                        match current_block {
                                                            15568809317923174358 => {}
                                                            _ => {
                                                                current_block = 5976146557728084725;
                                                            }
                                                        }
                                                    }
                                                    18195282705153904920 => {
                                                        if execlevel == 0 as ::core::ffi::c_int {
                                                            eline = token(
                                                                eline,
                                                                &raw mut golabel as *mut ::core::ffi::c_char,
                                                                NPAT,
                                                            );
                                                            linlen = strlen(
                                                                &raw mut golabel as *mut ::core::ffi::c_char,
                                                            ) as ::core::ffi::c_int;
                                                            glp = (*hlp).l_fp;
                                                            loop {
                                                                if !(glp != hlp) {
                                                                    current_block = 10109057886293123569;
                                                                    break;
                                                                }
                                                                if *(&raw mut (*glp).l_text as *mut ::core::ffi::c_uchar)
                                                                    as ::core::ffi::c_int == '*' as i32
                                                                    && strncmp(
                                                                        (&raw mut (*glp).l_text as *mut ::core::ffi::c_uchar)
                                                                            .offset(1 as ::core::ffi::c_int as isize)
                                                                            as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_char,
                                                                        &raw mut golabel as *mut ::core::ffi::c_char,
                                                                        linlen as size_t,
                                                                    ) == 0 as ::core::ffi::c_int
                                                                {
                                                                    lp = glp;
                                                                    current_block = 15568809317923174358;
                                                                    break;
                                                                } else {
                                                                    glp = (*glp).l_fp;
                                                                }
                                                            }
                                                            match current_block {
                                                                15568809317923174358 => {}
                                                                _ => {
                                                                    mlwrite(
                                                                        b"%%No such label\0" as *const u8
                                                                            as *const ::core::ffi::c_char,
                                                                    );
                                                                    freewhile(whlist);
                                                                    return FALSE;
                                                                }
                                                            }
                                                        } else {
                                                            current_block = 15568809317923174358;
                                                        }
                                                    }
                                                    2430319949834955361 => {
                                                        if execlevel != 0 {
                                                            execlevel -= 1;
                                                        } else {
                                                            whtemp = whlist;
                                                            while !whtemp.is_null() {
                                                                if (*whtemp).w_type == BTWHILE && (*whtemp).w_end == lp {
                                                                    break;
                                                                }
                                                                whtemp = (*whtemp).w_next;
                                                            }
                                                            if whtemp.is_null() {
                                                                mlwrite(
                                                                    b"%%Internal While loop error\0" as *const u8
                                                                        as *const ::core::ffi::c_char,
                                                                );
                                                                freewhile(whlist);
                                                                return FALSE;
                                                            }
                                                            lp = (*(*whtemp).w_begin).l_bp;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    14180127615403059768 => {
                                                        force = TRUE;
                                                        current_block = 4736343053266048700;
                                                    }
                                                    10703124919230875173 => {
                                                        if execlevel == 1 as ::core::ffi::c_int {
                                                            execlevel -= 1;
                                                        } else if execlevel == 0 as ::core::ffi::c_int {
                                                            execlevel += 1;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    6246763041535604849 => {
                                                        if execlevel != 0 {
                                                            execlevel -= 1;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    14928940103145537714 => {
                                                        if execlevel == 0 as ::core::ffi::c_int {
                                                            break;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    _ => {}
                                                }
                                                match current_block {
                                                    15568809317923174358 => {}
                                                    4736343053266048700 => {}
                                                    _ => {
                                                        if dirnum == DBREAK && execlevel != 0 {
                                                            current_block = 15568809317923174358;
                                                        } else {
                                                            whtemp = whlist;
                                                            while !whtemp.is_null() {
                                                                if (*whtemp).w_begin == lp {
                                                                    break;
                                                                }
                                                                whtemp = (*whtemp).w_next;
                                                            }
                                                            if whtemp.is_null() {
                                                                mlwrite(
                                                                    b"%%Internal While loop error\0" as *const u8
                                                                        as *const ::core::ffi::c_char,
                                                                );
                                                                freewhile(whlist);
                                                                return FALSE;
                                                            }
                                                            lp = (*whtemp).w_end;
                                                            current_block = 15568809317923174358;
                                                        }
                                                    }
                                                }
                                            }
                                            DELSE => {
                                                current_block = 10703124919230875173;
                                                match current_block {
                                                    9281452336402489912 => {
                                                        if execlevel == 0 as ::core::ffi::c_int {
                                                            if macarg(&raw mut tkn as *mut ::core::ffi::c_char) != TRUE
                                                            {
                                                                break;
                                                            }
                                                            if stol(&raw mut tkn as *mut ::core::ffi::c_char) == FALSE {
                                                                execlevel += 1;
                                                            }
                                                        } else {
                                                            execlevel += 1;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    15988712159247554322 => {
                                                        if execlevel == 0 as ::core::ffi::c_int {
                                                            if macarg(&raw mut tkn as *mut ::core::ffi::c_char) != TRUE
                                                            {
                                                                break;
                                                            }
                                                            if stol(&raw mut tkn as *mut ::core::ffi::c_char) == TRUE {
                                                                current_block = 15568809317923174358;
                                                            } else {
                                                                current_block = 2750570471926810434;
                                                            }
                                                        } else {
                                                            current_block = 2750570471926810434;
                                                        }
                                                        match current_block {
                                                            15568809317923174358 => {}
                                                            _ => {
                                                                current_block = 5976146557728084725;
                                                            }
                                                        }
                                                    }
                                                    18195282705153904920 => {
                                                        if execlevel == 0 as ::core::ffi::c_int {
                                                            eline = token(
                                                                eline,
                                                                &raw mut golabel as *mut ::core::ffi::c_char,
                                                                NPAT,
                                                            );
                                                            linlen = strlen(
                                                                &raw mut golabel as *mut ::core::ffi::c_char,
                                                            ) as ::core::ffi::c_int;
                                                            glp = (*hlp).l_fp;
                                                            loop {
                                                                if !(glp != hlp) {
                                                                    current_block = 10109057886293123569;
                                                                    break;
                                                                }
                                                                if *(&raw mut (*glp).l_text as *mut ::core::ffi::c_uchar)
                                                                    as ::core::ffi::c_int == '*' as i32
                                                                    && strncmp(
                                                                        (&raw mut (*glp).l_text as *mut ::core::ffi::c_uchar)
                                                                            .offset(1 as ::core::ffi::c_int as isize)
                                                                            as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_char,
                                                                        &raw mut golabel as *mut ::core::ffi::c_char,
                                                                        linlen as size_t,
                                                                    ) == 0 as ::core::ffi::c_int
                                                                {
                                                                    lp = glp;
                                                                    current_block = 15568809317923174358;
                                                                    break;
                                                                } else {
                                                                    glp = (*glp).l_fp;
                                                                }
                                                            }
                                                            match current_block {
                                                                15568809317923174358 => {}
                                                                _ => {
                                                                    mlwrite(
                                                                        b"%%No such label\0" as *const u8
                                                                            as *const ::core::ffi::c_char,
                                                                    );
                                                                    freewhile(whlist);
                                                                    return FALSE;
                                                                }
                                                            }
                                                        } else {
                                                            current_block = 15568809317923174358;
                                                        }
                                                    }
                                                    2430319949834955361 => {
                                                        if execlevel != 0 {
                                                            execlevel -= 1;
                                                        } else {
                                                            whtemp = whlist;
                                                            while !whtemp.is_null() {
                                                                if (*whtemp).w_type == BTWHILE && (*whtemp).w_end == lp {
                                                                    break;
                                                                }
                                                                whtemp = (*whtemp).w_next;
                                                            }
                                                            if whtemp.is_null() {
                                                                mlwrite(
                                                                    b"%%Internal While loop error\0" as *const u8
                                                                        as *const ::core::ffi::c_char,
                                                                );
                                                                freewhile(whlist);
                                                                return FALSE;
                                                            }
                                                            lp = (*(*whtemp).w_begin).l_bp;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    14180127615403059768 => {
                                                        force = TRUE;
                                                        current_block = 4736343053266048700;
                                                    }
                                                    10703124919230875173 => {
                                                        if execlevel == 1 as ::core::ffi::c_int {
                                                            execlevel -= 1;
                                                        } else if execlevel == 0 as ::core::ffi::c_int {
                                                            execlevel += 1;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    6246763041535604849 => {
                                                        if execlevel != 0 {
                                                            execlevel -= 1;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    14928940103145537714 => {
                                                        if execlevel == 0 as ::core::ffi::c_int {
                                                            break;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    _ => {}
                                                }
                                                match current_block {
                                                    15568809317923174358 => {}
                                                    4736343053266048700 => {}
                                                    _ => {
                                                        if dirnum == DBREAK && execlevel != 0 {
                                                            current_block = 15568809317923174358;
                                                        } else {
                                                            whtemp = whlist;
                                                            while !whtemp.is_null() {
                                                                if (*whtemp).w_begin == lp {
                                                                    break;
                                                                }
                                                                whtemp = (*whtemp).w_next;
                                                            }
                                                            if whtemp.is_null() {
                                                                mlwrite(
                                                                    b"%%Internal While loop error\0" as *const u8
                                                                        as *const ::core::ffi::c_char,
                                                                );
                                                                freewhile(whlist);
                                                                return FALSE;
                                                            }
                                                            lp = (*whtemp).w_end;
                                                            current_block = 15568809317923174358;
                                                        }
                                                    }
                                                }
                                            }
                                            DENDIF => {
                                                current_block = 6246763041535604849;
                                                match current_block {
                                                    9281452336402489912 => {
                                                        if execlevel == 0 as ::core::ffi::c_int {
                                                            if macarg(&raw mut tkn as *mut ::core::ffi::c_char) != TRUE
                                                            {
                                                                break;
                                                            }
                                                            if stol(&raw mut tkn as *mut ::core::ffi::c_char) == FALSE {
                                                                execlevel += 1;
                                                            }
                                                        } else {
                                                            execlevel += 1;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    15988712159247554322 => {
                                                        if execlevel == 0 as ::core::ffi::c_int {
                                                            if macarg(&raw mut tkn as *mut ::core::ffi::c_char) != TRUE
                                                            {
                                                                break;
                                                            }
                                                            if stol(&raw mut tkn as *mut ::core::ffi::c_char) == TRUE {
                                                                current_block = 15568809317923174358;
                                                            } else {
                                                                current_block = 2750570471926810434;
                                                            }
                                                        } else {
                                                            current_block = 2750570471926810434;
                                                        }
                                                        match current_block {
                                                            15568809317923174358 => {}
                                                            _ => {
                                                                current_block = 5976146557728084725;
                                                            }
                                                        }
                                                    }
                                                    18195282705153904920 => {
                                                        if execlevel == 0 as ::core::ffi::c_int {
                                                            eline = token(
                                                                eline,
                                                                &raw mut golabel as *mut ::core::ffi::c_char,
                                                                NPAT,
                                                            );
                                                            linlen = strlen(
                                                                &raw mut golabel as *mut ::core::ffi::c_char,
                                                            ) as ::core::ffi::c_int;
                                                            glp = (*hlp).l_fp;
                                                            loop {
                                                                if !(glp != hlp) {
                                                                    current_block = 10109057886293123569;
                                                                    break;
                                                                }
                                                                if *(&raw mut (*glp).l_text as *mut ::core::ffi::c_uchar)
                                                                    as ::core::ffi::c_int == '*' as i32
                                                                    && strncmp(
                                                                        (&raw mut (*glp).l_text as *mut ::core::ffi::c_uchar)
                                                                            .offset(1 as ::core::ffi::c_int as isize)
                                                                            as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_char,
                                                                        &raw mut golabel as *mut ::core::ffi::c_char,
                                                                        linlen as size_t,
                                                                    ) == 0 as ::core::ffi::c_int
                                                                {
                                                                    lp = glp;
                                                                    current_block = 15568809317923174358;
                                                                    break;
                                                                } else {
                                                                    glp = (*glp).l_fp;
                                                                }
                                                            }
                                                            match current_block {
                                                                15568809317923174358 => {}
                                                                _ => {
                                                                    mlwrite(
                                                                        b"%%No such label\0" as *const u8
                                                                            as *const ::core::ffi::c_char,
                                                                    );
                                                                    freewhile(whlist);
                                                                    return FALSE;
                                                                }
                                                            }
                                                        } else {
                                                            current_block = 15568809317923174358;
                                                        }
                                                    }
                                                    2430319949834955361 => {
                                                        if execlevel != 0 {
                                                            execlevel -= 1;
                                                        } else {
                                                            whtemp = whlist;
                                                            while !whtemp.is_null() {
                                                                if (*whtemp).w_type == BTWHILE && (*whtemp).w_end == lp {
                                                                    break;
                                                                }
                                                                whtemp = (*whtemp).w_next;
                                                            }
                                                            if whtemp.is_null() {
                                                                mlwrite(
                                                                    b"%%Internal While loop error\0" as *const u8
                                                                        as *const ::core::ffi::c_char,
                                                                );
                                                                freewhile(whlist);
                                                                return FALSE;
                                                            }
                                                            lp = (*(*whtemp).w_begin).l_bp;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    14180127615403059768 => {
                                                        force = TRUE;
                                                        current_block = 4736343053266048700;
                                                    }
                                                    10703124919230875173 => {
                                                        if execlevel == 1 as ::core::ffi::c_int {
                                                            execlevel -= 1;
                                                        } else if execlevel == 0 as ::core::ffi::c_int {
                                                            execlevel += 1;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    6246763041535604849 => {
                                                        if execlevel != 0 {
                                                            execlevel -= 1;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    14928940103145537714 => {
                                                        if execlevel == 0 as ::core::ffi::c_int {
                                                            break;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    _ => {}
                                                }
                                                match current_block {
                                                    15568809317923174358 => {}
                                                    4736343053266048700 => {}
                                                    _ => {
                                                        if dirnum == DBREAK && execlevel != 0 {
                                                            current_block = 15568809317923174358;
                                                        } else {
                                                            whtemp = whlist;
                                                            while !whtemp.is_null() {
                                                                if (*whtemp).w_begin == lp {
                                                                    break;
                                                                }
                                                                whtemp = (*whtemp).w_next;
                                                            }
                                                            if whtemp.is_null() {
                                                                mlwrite(
                                                                    b"%%Internal While loop error\0" as *const u8
                                                                        as *const ::core::ffi::c_char,
                                                                );
                                                                freewhile(whlist);
                                                                return FALSE;
                                                            }
                                                            lp = (*whtemp).w_end;
                                                            current_block = 15568809317923174358;
                                                        }
                                                    }
                                                }
                                            }
                                            DGOTO => {
                                                current_block = 18195282705153904920;
                                                match current_block {
                                                    9281452336402489912 => {
                                                        if execlevel == 0 as ::core::ffi::c_int {
                                                            if macarg(&raw mut tkn as *mut ::core::ffi::c_char) != TRUE
                                                            {
                                                                break;
                                                            }
                                                            if stol(&raw mut tkn as *mut ::core::ffi::c_char) == FALSE {
                                                                execlevel += 1;
                                                            }
                                                        } else {
                                                            execlevel += 1;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    15988712159247554322 => {
                                                        if execlevel == 0 as ::core::ffi::c_int {
                                                            if macarg(&raw mut tkn as *mut ::core::ffi::c_char) != TRUE
                                                            {
                                                                break;
                                                            }
                                                            if stol(&raw mut tkn as *mut ::core::ffi::c_char) == TRUE {
                                                                current_block = 15568809317923174358;
                                                            } else {
                                                                current_block = 2750570471926810434;
                                                            }
                                                        } else {
                                                            current_block = 2750570471926810434;
                                                        }
                                                        match current_block {
                                                            15568809317923174358 => {}
                                                            _ => {
                                                                current_block = 5976146557728084725;
                                                            }
                                                        }
                                                    }
                                                    18195282705153904920 => {
                                                        if execlevel == 0 as ::core::ffi::c_int {
                                                            eline = token(
                                                                eline,
                                                                &raw mut golabel as *mut ::core::ffi::c_char,
                                                                NPAT,
                                                            );
                                                            linlen = strlen(
                                                                &raw mut golabel as *mut ::core::ffi::c_char,
                                                            ) as ::core::ffi::c_int;
                                                            glp = (*hlp).l_fp;
                                                            loop {
                                                                if !(glp != hlp) {
                                                                    current_block = 10109057886293123569;
                                                                    break;
                                                                }
                                                                if *(&raw mut (*glp).l_text as *mut ::core::ffi::c_uchar)
                                                                    as ::core::ffi::c_int == '*' as i32
                                                                    && strncmp(
                                                                        (&raw mut (*glp).l_text as *mut ::core::ffi::c_uchar)
                                                                            .offset(1 as ::core::ffi::c_int as isize)
                                                                            as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_char,
                                                                        &raw mut golabel as *mut ::core::ffi::c_char,
                                                                        linlen as size_t,
                                                                    ) == 0 as ::core::ffi::c_int
                                                                {
                                                                    lp = glp;
                                                                    current_block = 15568809317923174358;
                                                                    break;
                                                                } else {
                                                                    glp = (*glp).l_fp;
                                                                }
                                                            }
                                                            match current_block {
                                                                15568809317923174358 => {}
                                                                _ => {
                                                                    mlwrite(
                                                                        b"%%No such label\0" as *const u8
                                                                            as *const ::core::ffi::c_char,
                                                                    );
                                                                    freewhile(whlist);
                                                                    return FALSE;
                                                                }
                                                            }
                                                        } else {
                                                            current_block = 15568809317923174358;
                                                        }
                                                    }
                                                    2430319949834955361 => {
                                                        if execlevel != 0 {
                                                            execlevel -= 1;
                                                        } else {
                                                            whtemp = whlist;
                                                            while !whtemp.is_null() {
                                                                if (*whtemp).w_type == BTWHILE && (*whtemp).w_end == lp {
                                                                    break;
                                                                }
                                                                whtemp = (*whtemp).w_next;
                                                            }
                                                            if whtemp.is_null() {
                                                                mlwrite(
                                                                    b"%%Internal While loop error\0" as *const u8
                                                                        as *const ::core::ffi::c_char,
                                                                );
                                                                freewhile(whlist);
                                                                return FALSE;
                                                            }
                                                            lp = (*(*whtemp).w_begin).l_bp;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    14180127615403059768 => {
                                                        force = TRUE;
                                                        current_block = 4736343053266048700;
                                                    }
                                                    10703124919230875173 => {
                                                        if execlevel == 1 as ::core::ffi::c_int {
                                                            execlevel -= 1;
                                                        } else if execlevel == 0 as ::core::ffi::c_int {
                                                            execlevel += 1;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    6246763041535604849 => {
                                                        if execlevel != 0 {
                                                            execlevel -= 1;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    14928940103145537714 => {
                                                        if execlevel == 0 as ::core::ffi::c_int {
                                                            break;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    _ => {}
                                                }
                                                match current_block {
                                                    15568809317923174358 => {}
                                                    4736343053266048700 => {}
                                                    _ => {
                                                        if dirnum == DBREAK && execlevel != 0 {
                                                            current_block = 15568809317923174358;
                                                        } else {
                                                            whtemp = whlist;
                                                            while !whtemp.is_null() {
                                                                if (*whtemp).w_begin == lp {
                                                                    break;
                                                                }
                                                                whtemp = (*whtemp).w_next;
                                                            }
                                                            if whtemp.is_null() {
                                                                mlwrite(
                                                                    b"%%Internal While loop error\0" as *const u8
                                                                        as *const ::core::ffi::c_char,
                                                                );
                                                                freewhile(whlist);
                                                                return FALSE;
                                                            }
                                                            lp = (*whtemp).w_end;
                                                            current_block = 15568809317923174358;
                                                        }
                                                    }
                                                }
                                            }
                                            DRETURN => {
                                                current_block = 14928940103145537714;
                                                match current_block {
                                                    9281452336402489912 => {
                                                        if execlevel == 0 as ::core::ffi::c_int {
                                                            if macarg(&raw mut tkn as *mut ::core::ffi::c_char) != TRUE
                                                            {
                                                                break;
                                                            }
                                                            if stol(&raw mut tkn as *mut ::core::ffi::c_char) == FALSE {
                                                                execlevel += 1;
                                                            }
                                                        } else {
                                                            execlevel += 1;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    15988712159247554322 => {
                                                        if execlevel == 0 as ::core::ffi::c_int {
                                                            if macarg(&raw mut tkn as *mut ::core::ffi::c_char) != TRUE
                                                            {
                                                                break;
                                                            }
                                                            if stol(&raw mut tkn as *mut ::core::ffi::c_char) == TRUE {
                                                                current_block = 15568809317923174358;
                                                            } else {
                                                                current_block = 2750570471926810434;
                                                            }
                                                        } else {
                                                            current_block = 2750570471926810434;
                                                        }
                                                        match current_block {
                                                            15568809317923174358 => {}
                                                            _ => {
                                                                current_block = 5976146557728084725;
                                                            }
                                                        }
                                                    }
                                                    18195282705153904920 => {
                                                        if execlevel == 0 as ::core::ffi::c_int {
                                                            eline = token(
                                                                eline,
                                                                &raw mut golabel as *mut ::core::ffi::c_char,
                                                                NPAT,
                                                            );
                                                            linlen = strlen(
                                                                &raw mut golabel as *mut ::core::ffi::c_char,
                                                            ) as ::core::ffi::c_int;
                                                            glp = (*hlp).l_fp;
                                                            loop {
                                                                if !(glp != hlp) {
                                                                    current_block = 10109057886293123569;
                                                                    break;
                                                                }
                                                                if *(&raw mut (*glp).l_text as *mut ::core::ffi::c_uchar)
                                                                    as ::core::ffi::c_int == '*' as i32
                                                                    && strncmp(
                                                                        (&raw mut (*glp).l_text as *mut ::core::ffi::c_uchar)
                                                                            .offset(1 as ::core::ffi::c_int as isize)
                                                                            as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_char,
                                                                        &raw mut golabel as *mut ::core::ffi::c_char,
                                                                        linlen as size_t,
                                                                    ) == 0 as ::core::ffi::c_int
                                                                {
                                                                    lp = glp;
                                                                    current_block = 15568809317923174358;
                                                                    break;
                                                                } else {
                                                                    glp = (*glp).l_fp;
                                                                }
                                                            }
                                                            match current_block {
                                                                15568809317923174358 => {}
                                                                _ => {
                                                                    mlwrite(
                                                                        b"%%No such label\0" as *const u8
                                                                            as *const ::core::ffi::c_char,
                                                                    );
                                                                    freewhile(whlist);
                                                                    return FALSE;
                                                                }
                                                            }
                                                        } else {
                                                            current_block = 15568809317923174358;
                                                        }
                                                    }
                                                    2430319949834955361 => {
                                                        if execlevel != 0 {
                                                            execlevel -= 1;
                                                        } else {
                                                            whtemp = whlist;
                                                            while !whtemp.is_null() {
                                                                if (*whtemp).w_type == BTWHILE && (*whtemp).w_end == lp {
                                                                    break;
                                                                }
                                                                whtemp = (*whtemp).w_next;
                                                            }
                                                            if whtemp.is_null() {
                                                                mlwrite(
                                                                    b"%%Internal While loop error\0" as *const u8
                                                                        as *const ::core::ffi::c_char,
                                                                );
                                                                freewhile(whlist);
                                                                return FALSE;
                                                            }
                                                            lp = (*(*whtemp).w_begin).l_bp;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    14180127615403059768 => {
                                                        force = TRUE;
                                                        current_block = 4736343053266048700;
                                                    }
                                                    10703124919230875173 => {
                                                        if execlevel == 1 as ::core::ffi::c_int {
                                                            execlevel -= 1;
                                                        } else if execlevel == 0 as ::core::ffi::c_int {
                                                            execlevel += 1;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    6246763041535604849 => {
                                                        if execlevel != 0 {
                                                            execlevel -= 1;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    14928940103145537714 => {
                                                        if execlevel == 0 as ::core::ffi::c_int {
                                                            break;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    _ => {}
                                                }
                                                match current_block {
                                                    15568809317923174358 => {}
                                                    4736343053266048700 => {}
                                                    _ => {
                                                        if dirnum == DBREAK && execlevel != 0 {
                                                            current_block = 15568809317923174358;
                                                        } else {
                                                            whtemp = whlist;
                                                            while !whtemp.is_null() {
                                                                if (*whtemp).w_begin == lp {
                                                                    break;
                                                                }
                                                                whtemp = (*whtemp).w_next;
                                                            }
                                                            if whtemp.is_null() {
                                                                mlwrite(
                                                                    b"%%Internal While loop error\0" as *const u8
                                                                        as *const ::core::ffi::c_char,
                                                                );
                                                                freewhile(whlist);
                                                                return FALSE;
                                                            }
                                                            lp = (*whtemp).w_end;
                                                            current_block = 15568809317923174358;
                                                        }
                                                    }
                                                }
                                            }
                                            DENDWHILE => {
                                                current_block = 2430319949834955361;
                                                match current_block {
                                                    9281452336402489912 => {
                                                        if execlevel == 0 as ::core::ffi::c_int {
                                                            if macarg(&raw mut tkn as *mut ::core::ffi::c_char) != TRUE
                                                            {
                                                                break;
                                                            }
                                                            if stol(&raw mut tkn as *mut ::core::ffi::c_char) == FALSE {
                                                                execlevel += 1;
                                                            }
                                                        } else {
                                                            execlevel += 1;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    15988712159247554322 => {
                                                        if execlevel == 0 as ::core::ffi::c_int {
                                                            if macarg(&raw mut tkn as *mut ::core::ffi::c_char) != TRUE
                                                            {
                                                                break;
                                                            }
                                                            if stol(&raw mut tkn as *mut ::core::ffi::c_char) == TRUE {
                                                                current_block = 15568809317923174358;
                                                            } else {
                                                                current_block = 2750570471926810434;
                                                            }
                                                        } else {
                                                            current_block = 2750570471926810434;
                                                        }
                                                        match current_block {
                                                            15568809317923174358 => {}
                                                            _ => {
                                                                current_block = 5976146557728084725;
                                                            }
                                                        }
                                                    }
                                                    18195282705153904920 => {
                                                        if execlevel == 0 as ::core::ffi::c_int {
                                                            eline = token(
                                                                eline,
                                                                &raw mut golabel as *mut ::core::ffi::c_char,
                                                                NPAT,
                                                            );
                                                            linlen = strlen(
                                                                &raw mut golabel as *mut ::core::ffi::c_char,
                                                            ) as ::core::ffi::c_int;
                                                            glp = (*hlp).l_fp;
                                                            loop {
                                                                if !(glp != hlp) {
                                                                    current_block = 10109057886293123569;
                                                                    break;
                                                                }
                                                                if *(&raw mut (*glp).l_text as *mut ::core::ffi::c_uchar)
                                                                    as ::core::ffi::c_int == '*' as i32
                                                                    && strncmp(
                                                                        (&raw mut (*glp).l_text as *mut ::core::ffi::c_uchar)
                                                                            .offset(1 as ::core::ffi::c_int as isize)
                                                                            as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_char,
                                                                        &raw mut golabel as *mut ::core::ffi::c_char,
                                                                        linlen as size_t,
                                                                    ) == 0 as ::core::ffi::c_int
                                                                {
                                                                    lp = glp;
                                                                    current_block = 15568809317923174358;
                                                                    break;
                                                                } else {
                                                                    glp = (*glp).l_fp;
                                                                }
                                                            }
                                                            match current_block {
                                                                15568809317923174358 => {}
                                                                _ => {
                                                                    mlwrite(
                                                                        b"%%No such label\0" as *const u8
                                                                            as *const ::core::ffi::c_char,
                                                                    );
                                                                    freewhile(whlist);
                                                                    return FALSE;
                                                                }
                                                            }
                                                        } else {
                                                            current_block = 15568809317923174358;
                                                        }
                                                    }
                                                    2430319949834955361 => {
                                                        if execlevel != 0 {
                                                            execlevel -= 1;
                                                        } else {
                                                            whtemp = whlist;
                                                            while !whtemp.is_null() {
                                                                if (*whtemp).w_type == BTWHILE && (*whtemp).w_end == lp {
                                                                    break;
                                                                }
                                                                whtemp = (*whtemp).w_next;
                                                            }
                                                            if whtemp.is_null() {
                                                                mlwrite(
                                                                    b"%%Internal While loop error\0" as *const u8
                                                                        as *const ::core::ffi::c_char,
                                                                );
                                                                freewhile(whlist);
                                                                return FALSE;
                                                            }
                                                            lp = (*(*whtemp).w_begin).l_bp;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    14180127615403059768 => {
                                                        force = TRUE;
                                                        current_block = 4736343053266048700;
                                                    }
                                                    10703124919230875173 => {
                                                        if execlevel == 1 as ::core::ffi::c_int {
                                                            execlevel -= 1;
                                                        } else if execlevel == 0 as ::core::ffi::c_int {
                                                            execlevel += 1;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    6246763041535604849 => {
                                                        if execlevel != 0 {
                                                            execlevel -= 1;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    14928940103145537714 => {
                                                        if execlevel == 0 as ::core::ffi::c_int {
                                                            break;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    _ => {}
                                                }
                                                match current_block {
                                                    15568809317923174358 => {}
                                                    4736343053266048700 => {}
                                                    _ => {
                                                        if dirnum == DBREAK && execlevel != 0 {
                                                            current_block = 15568809317923174358;
                                                        } else {
                                                            whtemp = whlist;
                                                            while !whtemp.is_null() {
                                                                if (*whtemp).w_begin == lp {
                                                                    break;
                                                                }
                                                                whtemp = (*whtemp).w_next;
                                                            }
                                                            if whtemp.is_null() {
                                                                mlwrite(
                                                                    b"%%Internal While loop error\0" as *const u8
                                                                        as *const ::core::ffi::c_char,
                                                                );
                                                                freewhile(whlist);
                                                                return FALSE;
                                                            }
                                                            lp = (*whtemp).w_end;
                                                            current_block = 15568809317923174358;
                                                        }
                                                    }
                                                }
                                            }
                                            DFORCE => {
                                                current_block = 14180127615403059768;
                                                match current_block {
                                                    9281452336402489912 => {
                                                        if execlevel == 0 as ::core::ffi::c_int {
                                                            if macarg(&raw mut tkn as *mut ::core::ffi::c_char) != TRUE
                                                            {
                                                                break;
                                                            }
                                                            if stol(&raw mut tkn as *mut ::core::ffi::c_char) == FALSE {
                                                                execlevel += 1;
                                                            }
                                                        } else {
                                                            execlevel += 1;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    15988712159247554322 => {
                                                        if execlevel == 0 as ::core::ffi::c_int {
                                                            if macarg(&raw mut tkn as *mut ::core::ffi::c_char) != TRUE
                                                            {
                                                                break;
                                                            }
                                                            if stol(&raw mut tkn as *mut ::core::ffi::c_char) == TRUE {
                                                                current_block = 15568809317923174358;
                                                            } else {
                                                                current_block = 2750570471926810434;
                                                            }
                                                        } else {
                                                            current_block = 2750570471926810434;
                                                        }
                                                        match current_block {
                                                            15568809317923174358 => {}
                                                            _ => {
                                                                current_block = 5976146557728084725;
                                                            }
                                                        }
                                                    }
                                                    18195282705153904920 => {
                                                        if execlevel == 0 as ::core::ffi::c_int {
                                                            eline = token(
                                                                eline,
                                                                &raw mut golabel as *mut ::core::ffi::c_char,
                                                                NPAT,
                                                            );
                                                            linlen = strlen(
                                                                &raw mut golabel as *mut ::core::ffi::c_char,
                                                            ) as ::core::ffi::c_int;
                                                            glp = (*hlp).l_fp;
                                                            loop {
                                                                if !(glp != hlp) {
                                                                    current_block = 10109057886293123569;
                                                                    break;
                                                                }
                                                                if *(&raw mut (*glp).l_text as *mut ::core::ffi::c_uchar)
                                                                    as ::core::ffi::c_int == '*' as i32
                                                                    && strncmp(
                                                                        (&raw mut (*glp).l_text as *mut ::core::ffi::c_uchar)
                                                                            .offset(1 as ::core::ffi::c_int as isize)
                                                                            as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_char,
                                                                        &raw mut golabel as *mut ::core::ffi::c_char,
                                                                        linlen as size_t,
                                                                    ) == 0 as ::core::ffi::c_int
                                                                {
                                                                    lp = glp;
                                                                    current_block = 15568809317923174358;
                                                                    break;
                                                                } else {
                                                                    glp = (*glp).l_fp;
                                                                }
                                                            }
                                                            match current_block {
                                                                15568809317923174358 => {}
                                                                _ => {
                                                                    mlwrite(
                                                                        b"%%No such label\0" as *const u8
                                                                            as *const ::core::ffi::c_char,
                                                                    );
                                                                    freewhile(whlist);
                                                                    return FALSE;
                                                                }
                                                            }
                                                        } else {
                                                            current_block = 15568809317923174358;
                                                        }
                                                    }
                                                    2430319949834955361 => {
                                                        if execlevel != 0 {
                                                            execlevel -= 1;
                                                        } else {
                                                            whtemp = whlist;
                                                            while !whtemp.is_null() {
                                                                if (*whtemp).w_type == BTWHILE && (*whtemp).w_end == lp {
                                                                    break;
                                                                }
                                                                whtemp = (*whtemp).w_next;
                                                            }
                                                            if whtemp.is_null() {
                                                                mlwrite(
                                                                    b"%%Internal While loop error\0" as *const u8
                                                                        as *const ::core::ffi::c_char,
                                                                );
                                                                freewhile(whlist);
                                                                return FALSE;
                                                            }
                                                            lp = (*(*whtemp).w_begin).l_bp;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    14180127615403059768 => {
                                                        force = TRUE;
                                                        current_block = 4736343053266048700;
                                                    }
                                                    10703124919230875173 => {
                                                        if execlevel == 1 as ::core::ffi::c_int {
                                                            execlevel -= 1;
                                                        } else if execlevel == 0 as ::core::ffi::c_int {
                                                            execlevel += 1;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    6246763041535604849 => {
                                                        if execlevel != 0 {
                                                            execlevel -= 1;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    14928940103145537714 => {
                                                        if execlevel == 0 as ::core::ffi::c_int {
                                                            break;
                                                        }
                                                        current_block = 15568809317923174358;
                                                    }
                                                    _ => {}
                                                }
                                                match current_block {
                                                    15568809317923174358 => {}
                                                    4736343053266048700 => {}
                                                    _ => {
                                                        if dirnum == DBREAK && execlevel != 0 {
                                                            current_block = 15568809317923174358;
                                                        } else {
                                                            whtemp = whlist;
                                                            while !whtemp.is_null() {
                                                                if (*whtemp).w_begin == lp {
                                                                    break;
                                                                }
                                                                whtemp = (*whtemp).w_next;
                                                            }
                                                            if whtemp.is_null() {
                                                                mlwrite(
                                                                    b"%%Internal While loop error\0" as *const u8
                                                                        as *const ::core::ffi::c_char,
                                                                );
                                                                freewhile(whlist);
                                                                return FALSE;
                                                            }
                                                            lp = (*whtemp).w_end;
                                                            current_block = 15568809317923174358;
                                                        }
                                                    }
                                                }
                                            }
                                            _ => {
                                                current_block = 4736343053266048700;
                                            }
                                        }
                                    } else {
                                        current_block = 4736343053266048700;
                                    }
                                    match current_block {
                                        15568809317923174358 => {}
                                        _ => {
                                            status = docmd(eline);
                                            if force != 0 {
                                                status = TRUE;
                                            }
                                            if status != TRUE {
                                                wp = curwp;
                                                if (*wp).w_bufp == bp {
                                                    (*wp).w_dotp = lp;
                                                    (*wp).w_doto = 0 as ::core::ffi::c_int;
                                                    (*wp).w_flag = ((*wp).w_flag as ::core::ffi::c_int | WFHARD)
                                                        as ::core::ffi::c_char;
                                                }
                                                (*bp).b_dotp = lp;
                                                (*bp).b_doto = 0 as ::core::ffi::c_int;
                                                free(einit as *mut ::core::ffi::c_void);
                                                execlevel = 0 as ::core::ffi::c_int;
                                                freewhile(whlist);
                                                return status;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                free(einit as *mut ::core::ffi::c_void);
                lp = (*lp).l_fp;
            }
            execlevel = 0 as ::core::ffi::c_int;
            freewhile(whlist);
            return TRUE;
        }
        4503586385541179488 => {
            mlwrite(
                b"%%Out of memory during while scan\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        7427571413727699167 => {
            mlwrite(
                b"%%!WHILE with no matching !ENDWHILE in '%s'\0" as *const u8
                    as *const ::core::ffi::c_char,
                &raw mut (*bp).b_bname as *mut ::core::ffi::c_char,
            );
        }
        _ => {}
    }
    freewhile(scanner);
    freewhile(whlist);
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn freewhile(mut wp: *mut while_block) {
    if wp.is_null() {
        return;
    }
    if !(*wp).w_next.is_null() {
        freewhile((*wp).w_next);
    }
    free(wp as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn execfile(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut status: ::core::ffi::c_int = 0;
    let mut fname: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut fspec: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    status = minibuf_input(
        b"File to execute: \0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut fname as *mut ::core::ffi::c_char,
        NSTRING - 1 as ::core::ffi::c_int,
    );
    if status != TRUE {
        return status;
    }
    fspec = flook(&raw mut fname as *mut ::core::ffi::c_char, FALSE);
    if fspec.is_null() {
        return FALSE;
    }
    loop {
        let fresh7 = n;
        n = n - 1;
        if !(fresh7 > 0 as ::core::ffi::c_int) {
            break;
        }
        status = dofile(fspec);
        if status != TRUE {
            return status;
        }
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn dofile(
    mut fname: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut bp: *mut buffer = ::core::ptr::null_mut::<buffer>();
    let mut cb: *mut buffer = ::core::ptr::null_mut::<buffer>();
    let mut status: ::core::ffi::c_int = 0;
    let mut bname: [::core::ffi::c_char; 16] = [0; 16];
    makename(&raw mut bname as *mut ::core::ffi::c_char, fname);
    unqname(&raw mut bname as *mut ::core::ffi::c_char);
    bp = bfind(
        &raw mut bname as *mut ::core::ffi::c_char,
        TRUE,
        0 as ::core::ffi::c_int,
    );
    if bp.is_null() {
        return FALSE;
    }
    (*bp).b_mode = MDVIEW;
    cb = curbp;
    curbp = bp;
    status = readin(fname, FALSE);
    if status != TRUE {
        curbp = cb;
        return status;
    }
    curbp = cb;
    status = dobuf(bp);
    if status != TRUE {
        return status;
    }
    if (*bp).b_nwnd as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        zotbuf(bp);
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn cbuf(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
    mut bufnum: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut bp: *mut buffer = ::core::ptr::null_mut::<buffer>();
    let mut status: ::core::ffi::c_int = 0;
    static mut bufname: [::core::ffi::c_char; 11] = unsafe {
        ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"*Macro xx*\0")
    };
    bufname[7 as ::core::ffi::c_int as usize] = ('0' as i32
        + bufnum / 10 as ::core::ffi::c_int) as ::core::ffi::c_char;
    bufname[8 as ::core::ffi::c_int as usize] = ('0' as i32
        + bufnum % 10 as ::core::ffi::c_int) as ::core::ffi::c_char;
    bp = bfind(
        &raw mut bufname as *mut ::core::ffi::c_char,
        FALSE,
        0 as ::core::ffi::c_int,
    );
    if bp.is_null() {
        mlwrite(b"Macro not defined\0" as *const u8 as *const ::core::ffi::c_char);
        return FALSE;
    }
    loop {
        let fresh8 = n;
        n = n - 1;
        if !(fresh8 > 0 as ::core::ffi::c_int) {
            break;
        }
        status = dobuf(bp);
        if status != TRUE {
            return status;
        }
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn cbuf1(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf2(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 2 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf3(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 3 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf4(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 4 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf5(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 5 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf6(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 6 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf7(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 7 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf8(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 8 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf9(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 9 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf10(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 10 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf11(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 11 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf12(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 12 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf13(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 13 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf14(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 14 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf15(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 15 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf16(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 16 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf17(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 17 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf18(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 18 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf19(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 19 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf20(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 20 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf21(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 21 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf22(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 22 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf23(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 23 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf24(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 24 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf25(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 25 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf26(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 26 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf27(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 27 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf28(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 28 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf29(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 29 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf30(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 30 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf31(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 31 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf32(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 32 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf33(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 33 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf34(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 34 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf35(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 35 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf36(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 36 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf37(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 37 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf38(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 38 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf39(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 39 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cbuf40(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return cbuf(f, n, 40 as ::core::ffi::c_int);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
