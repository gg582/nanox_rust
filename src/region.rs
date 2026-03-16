extern "C" {
    static mut thisflag: ::core::ffi::c_int;
    static mut lastflag: ::core::ffi::c_int;
    static mut curwp: *mut window;
    static mut curbp: *mut buffer;
    fn rdonly() -> ::core::ffi::c_int;
    fn mlwrite(fmt: *const ::core::ffi::c_char, ...);
    fn lchange(flag: ::core::ffi::c_int);
    fn ldelete(n: ::core::ffi::c_long, kflag: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn kdelete();
    fn kinsert(c: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
pub struct region {
    pub r_linep: *mut line,
    pub r_offset: ::core::ffi::c_int,
    pub r_size: ::core::ffi::c_long,
}
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CFKILL: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const WFHARD: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const MDVIEW: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
unsafe extern "C" fn nanox_resolve_region(
    mut region: *mut region,
) -> ::core::ffi::c_int {
    if (*curwp).w_markp.is_null() {
        mlwrite(
            b"No mark set in this window\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return FALSE;
    }
    return getregion(region);
}
unsafe extern "C" fn nanox_prime_kill_buffer() {
    if lastflag & CFKILL == 0 as ::core::ffi::c_int {
        kdelete();
    }
    thisflag |= CFKILL;
}
unsafe extern "C" fn nanox_copy_region_to_kill(
    mut region: *const region,
) -> ::core::ffi::c_int {
    let mut linep: *mut line = (*region).r_linep;
    let mut loffs: ::core::ffi::c_int = (*region).r_offset;
    let mut remaining: ::core::ffi::c_long = (*region).r_size;
    loop {
        let fresh0 = remaining;
        remaining = remaining - 1;
        if !(fresh0 > 0 as ::core::ffi::c_long) {
            break;
        }
        if linep == (*curbp).b_linep {
            return FALSE;
        }
        if loffs == (*linep).l_used {
            if kinsert('\n' as i32) != TRUE {
                return FALSE;
            }
            linep = (*linep).l_fp;
            loffs = 0 as ::core::ffi::c_int;
        } else {
            if kinsert(
                (*(&raw mut (*linep).l_text as *mut ::core::ffi::c_uchar)
                    .offset(loffs as isize) as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar
                    as ::core::ffi::c_int,
            ) != TRUE
            {
                return FALSE;
            }
            loffs += 1;
        }
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn kill_region_nanox(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut region: region = region {
        r_linep: ::core::ptr::null_mut::<line>(),
        r_offset: 0,
        r_size: 0,
    };
    if (*curbp).b_mode & MDVIEW != 0 {
        return rdonly();
    }
    if nanox_resolve_region(&raw mut region) != TRUE {
        return FALSE;
    }
    nanox_prime_kill_buffer();
    if nanox_copy_region_to_kill(&raw mut region) != TRUE {
        return FALSE;
    }
    (*curwp).w_dotp = region.r_linep;
    (*curwp).w_doto = region.r_offset;
    if ldelete(region.r_size, FALSE) != TRUE {
        return FALSE;
    }
    (*curwp).w_flag = ((*curwp).w_flag as ::core::ffi::c_int | WFHARD)
        as ::core::ffi::c_char;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn killregion(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return kill_region_nanox(f, n);
}
#[no_mangle]
pub unsafe extern "C" fn copy_region_nanox(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut region: region = region {
        r_linep: ::core::ptr::null_mut::<line>(),
        r_offset: 0,
        r_size: 0,
    };
    if nanox_resolve_region(&raw mut region) != TRUE {
        return FALSE;
    }
    nanox_prime_kill_buffer();
    if nanox_copy_region_to_kill(&raw mut region) != TRUE {
        return FALSE;
    }
    mlwrite(b"(region copied)\0" as *const u8 as *const ::core::ffi::c_char);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn copyregion(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return copy_region_nanox(f, n);
}
#[no_mangle]
pub unsafe extern "C" fn lowerregion(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut linep: *mut line = ::core::ptr::null_mut::<line>();
    let mut loffs: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    let mut s: ::core::ffi::c_int = 0;
    let mut region: region = region {
        r_linep: ::core::ptr::null_mut::<line>(),
        r_offset: 0,
        r_size: 0,
    };
    if (*curbp).b_mode & MDVIEW != 0 {
        return rdonly();
    }
    s = getregion(&raw mut region);
    if s != TRUE {
        return s;
    }
    lchange(WFHARD);
    linep = region.r_linep;
    loffs = region.r_offset;
    loop {
        let fresh1 = region.r_size;
        region.r_size = region.r_size - 1;
        if !(fresh1 != 0) {
            break;
        }
        if loffs == (*linep).l_used {
            linep = (*linep).l_fp;
            loffs = 0 as ::core::ffi::c_int;
        } else {
            c = (*(&raw mut (*linep).l_text as *mut ::core::ffi::c_uchar)
                .offset(loffs as isize) as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar
                as ::core::ffi::c_int;
            if c >= 'A' as i32 && c <= 'Z' as i32 {
                *(&raw mut (*linep).l_text as *mut ::core::ffi::c_uchar)
                    .offset(loffs as isize) = (c + 'a' as i32 - 'A' as i32)
                    as ::core::ffi::c_uchar;
            }
            loffs += 1;
        }
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn upperregion(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut linep: *mut line = ::core::ptr::null_mut::<line>();
    let mut loffs: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    let mut s: ::core::ffi::c_int = 0;
    let mut region: region = region {
        r_linep: ::core::ptr::null_mut::<line>(),
        r_offset: 0,
        r_size: 0,
    };
    if (*curbp).b_mode & MDVIEW != 0 {
        return rdonly();
    }
    s = getregion(&raw mut region);
    if s != TRUE {
        return s;
    }
    lchange(WFHARD);
    linep = region.r_linep;
    loffs = region.r_offset;
    loop {
        let fresh2 = region.r_size;
        region.r_size = region.r_size - 1;
        if !(fresh2 != 0) {
            break;
        }
        if loffs == (*linep).l_used {
            linep = (*linep).l_fp;
            loffs = 0 as ::core::ffi::c_int;
        } else {
            c = (*(&raw mut (*linep).l_text as *mut ::core::ffi::c_uchar)
                .offset(loffs as isize) as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar
                as ::core::ffi::c_int;
            if c >= 'a' as i32 && c <= 'z' as i32 {
                *(&raw mut (*linep).l_text as *mut ::core::ffi::c_uchar)
                    .offset(loffs as isize) = (c - 'a' as i32 + 'A' as i32)
                    as ::core::ffi::c_uchar;
            }
            loffs += 1;
        }
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn getregion(mut rp: *mut region) -> ::core::ffi::c_int {
    let mut flp: *mut line = ::core::ptr::null_mut::<line>();
    let mut blp: *mut line = ::core::ptr::null_mut::<line>();
    let mut fsize: ::core::ffi::c_long = 0;
    let mut bsize: ::core::ffi::c_long = 0;
    if (*curwp).w_markp.is_null() {
        mlwrite(
            b"No mark set in this window\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return FALSE;
    }
    if (*curwp).w_dotp == (*curwp).w_markp {
        (*rp).r_linep = (*curwp).w_dotp;
        if (*curwp).w_doto < (*curwp).w_marko {
            (*rp).r_offset = (*curwp).w_doto;
            (*rp).r_size = ((*curwp).w_marko - (*curwp).w_doto) as ::core::ffi::c_long;
        } else {
            (*rp).r_offset = (*curwp).w_marko;
            (*rp).r_size = ((*curwp).w_doto - (*curwp).w_marko) as ::core::ffi::c_long;
        }
        return TRUE;
    }
    blp = (*curwp).w_dotp;
    bsize = (*curwp).w_doto as ::core::ffi::c_long;
    flp = (*curwp).w_dotp;
    fsize = ((*flp).l_used - (*curwp).w_doto + 1 as ::core::ffi::c_int)
        as ::core::ffi::c_long;
    while flp != (*curbp).b_linep || (*blp).l_bp != (*curbp).b_linep {
        if flp != (*curbp).b_linep {
            flp = (*flp).l_fp;
            if flp == (*curwp).w_markp {
                (*rp).r_linep = (*curwp).w_dotp;
                (*rp).r_offset = (*curwp).w_doto;
                (*rp).r_size = fsize + (*curwp).w_marko as ::core::ffi::c_long;
                return TRUE;
            }
            fsize += ((*flp).l_used + 1 as ::core::ffi::c_int) as ::core::ffi::c_long;
        }
        if (*blp).l_bp != (*curbp).b_linep {
            blp = (*blp).l_bp;
            bsize += ((*blp).l_used + 1 as ::core::ffi::c_int) as ::core::ffi::c_long;
            if blp == (*curwp).w_markp {
                (*rp).r_linep = blp;
                (*rp).r_offset = (*curwp).w_marko;
                (*rp).r_size = bsize - (*curwp).w_marko as ::core::ffi::c_long;
                return TRUE;
            }
        }
    }
    mlwrite(b"Bug: lost mark\0" as *const u8 as *const ::core::ffi::c_char);
    return FALSE;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
