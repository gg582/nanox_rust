extern "C" {
    fn usleep(__useconds: __useconds_t) -> ::core::ffi::c_int;
    static mut term: *mut terminal;
    static mut tcap_term: terminal;
}
pub type __useconds_t = ::core::ffi::c_uint;
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
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut driver_failed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn vttopen() {
    if (*term).t_open.is_some() {
        (*term).t_open.expect("non-null function pointer")();
    }
    let mut retries: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    loop {
        let fresh0 = retries;
        retries = retries - 1;
        if !(fresh0 > 0 as ::core::ffi::c_int) {
            break;
        }
        if (*term).t_nrow as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            && (*term).t_ncol as ::core::ffi::c_int > 0 as ::core::ffi::c_int
        {
            break;
        }
        usleep(10000 as __useconds_t);
        if (*term).t_rez.is_some() {
            (*term)
                .t_rez
                .expect(
                    "non-null function pointer",
                )(::core::ptr::null_mut::<::core::ffi::c_char>());
        }
    }
    if (*term).t_nrow as ::core::ffi::c_int <= 0 as ::core::ffi::c_int
        || (*term).t_ncol as ::core::ffi::c_int <= 0 as ::core::ffi::c_int
    {
        driver_failed = 1 as ::core::ffi::c_int;
    }
    if driver_failed != 0 {
        if (*term).t_close.is_some() {
            (*term).t_close.expect("non-null function pointer")();
        }
        term = &raw mut tcap_term;
        if (*term).t_open.is_some() {
            (*term).t_open.expect("non-null function pointer")();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn vttclose() {
    if (*term).t_close.is_some() {
        (*term).t_close.expect("non-null function pointer")();
    }
}
#[no_mangle]
pub unsafe extern "C" fn vttkopen() {
    if (*term).t_kopen.is_some() {
        (*term).t_kopen.expect("non-null function pointer")();
    }
}
#[no_mangle]
pub unsafe extern "C" fn vttkclose() {
    if (*term).t_kclose.is_some() {
        (*term).t_kclose.expect("non-null function pointer")();
    }
}
#[no_mangle]
pub unsafe extern "C" fn vttgetc() -> ::core::ffi::c_int {
    if (*term).t_getchar.is_some() {
        return (*term).t_getchar.expect("non-null function pointer")();
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn vttputc(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if (*term).t_putchar.is_some() {
        return (*term).t_putchar.expect("non-null function pointer")(c);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn vttflush() {
    if (*term).t_flush.is_some() {
        (*term).t_flush.expect("non-null function pointer")();
    }
}
#[no_mangle]
pub unsafe extern "C" fn vttmove(
    mut row: ::core::ffi::c_int,
    mut col: ::core::ffi::c_int,
) {
    if (*term).t_move.is_some() {
        (*term).t_move.expect("non-null function pointer")(row, col);
    }
}
#[no_mangle]
pub unsafe extern "C" fn vtteeol() {
    if (*term).t_eeol.is_some() {
        (*term).t_eeol.expect("non-null function pointer")();
    }
}
#[no_mangle]
pub unsafe extern "C" fn vtteeop() {
    if (*term).t_eeop.is_some() {
        (*term).t_eeop.expect("non-null function pointer")();
    }
}
#[no_mangle]
pub unsafe extern "C" fn vttbeep() {}
#[no_mangle]
pub unsafe extern "C" fn vttrev(mut state: ::core::ffi::c_int) {
    if (*term).t_rev.is_some() {
        (*term).t_rev.expect("non-null function pointer")(state);
    }
}
#[no_mangle]
pub unsafe extern "C" fn vttitalic(mut state: ::core::ffi::c_int) {
    if (*term).t_italic.is_some() {
        (*term).t_italic.expect("non-null function pointer")(state);
    }
}
#[no_mangle]
pub unsafe extern "C" fn vttsetcolors(
    mut fg: ::core::ffi::c_int,
    mut bg: ::core::ffi::c_int,
) {
    if (*term).t_set_colors.is_some() {
        (*term).t_set_colors.expect("non-null function pointer")(fg, bg);
    }
}
#[no_mangle]
pub unsafe extern "C" fn vttsetattrs(
    mut bold: ::core::ffi::c_int,
    mut underline: ::core::ffi::c_int,
    mut italic: ::core::ffi::c_int,
) {
    if (*term).t_set_attrs.is_some() {
        (*term).t_set_attrs.expect("non-null function pointer")(bold, underline, italic);
    }
}
#[no_mangle]
pub unsafe extern "C" fn vttrez(
    mut res: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if (*term).t_rez.is_some() {
        return (*term).t_rez.expect("non-null function pointer")(res);
    }
    return TRUE;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
