extern "C" {
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcat(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    static mut abortc: ::core::ffi::c_int;
    static mut metac: ::core::ffi::c_int;
    static mut ctlxc: ::core::ffi::c_int;
    static mut reptc: ::core::ffi::c_int;
    static mut clexec: ::core::ffi::c_int;
    static mut keytab: [key_tab; 0];
    static mut names: [name_bind; 0];
    fn ctrlg(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn metafn(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cex(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn unarg(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn mlwrite(fmt: *const ::core::ffi::c_char, ...);
    fn getname() -> fn_t;
    fn get1key() -> ::core::ffi::c_int;
    fn getcmd() -> ::core::ffi::c_int;
    fn ostring(s: *mut ::core::ffi::c_char);
    fn ffropen(fn_0: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn ffclose() -> ::core::ffi::c_int;
    fn macarg(tok: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn dofile(fname: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
}
pub type size_t = usize;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct name_bind {
    pub n_name: *mut ::core::ffi::c_char,
    pub n_func: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
}
pub type fn_t = Option<
    unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int) -> ::core::ffi::c_int,
>;
pub const NBINDS: ::core::ffi::c_int = 2048 as ::core::ffi::c_int;
pub const CONTROL: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;
pub const META: ::core::ffi::c_int = 0x20000000 as ::core::ffi::c_int;
pub const CTLX: ::core::ffi::c_int = 0x40000000 as ::core::ffi::c_int;
pub const SPEC: ::core::ffi::c_uint = 0x80000000 as ::core::ffi::c_uint;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FIOSUC: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PATHCHR: ::core::ffi::c_int = ':' as i32;
#[no_mangle]
pub unsafe extern "C" fn deskey(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = 0;
    let mut ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut outseq: [::core::ffi::c_char; 1024] = [0; 1024];
    mlwrite(b": describe-key \0" as *const u8 as *const ::core::ffi::c_char);
    c = getckey(FALSE) as ::core::ffi::c_int;
    cmdstr(
        c,
        (&raw mut outseq as *mut ::core::ffi::c_char)
            .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
    );
    ostring(&raw mut outseq as *mut ::core::ffi::c_char);
    ostring(
        b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    ptr = getfname(getbind(c));
    if ptr.is_null() {
        ptr = b"Not Bound\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
    }
    ostring(ptr);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn bindtokey(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_uint = 0;
    let mut kfunc: fn_t = None;
    let mut ktp: *mut key_tab = ::core::ptr::null_mut::<key_tab>();
    let mut found: ::core::ffi::c_int = 0;
    let mut outseq: [::core::ffi::c_char; 80] = [0; 80];
    mlwrite(b": bind-to-key \0" as *const u8 as *const ::core::ffi::c_char);
    kfunc = getname();
    if kfunc.is_none() {
        mlwrite(b"(No such function)\0" as *const u8 as *const ::core::ffi::c_char);
        return FALSE;
    }
    ostring(
        b" \0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    );
    c = getckey(
        (kfunc
            == Some(
                metafn
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
            || kfunc
                == Some(
                    cex
                        as unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                )
            || kfunc
                == Some(
                    unarg
                        as unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                )
            || kfunc
                == Some(
                    ctrlg
                        as unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int,
                )) as ::core::ffi::c_int,
    );
    cmdstr(
        c as ::core::ffi::c_int,
        (&raw mut outseq as *mut ::core::ffi::c_char)
            .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
    );
    ostring(&raw mut outseq as *mut ::core::ffi::c_char);
    if kfunc
        == Some(
            metafn
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
        || kfunc
            == Some(
                cex
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
        || kfunc
            == Some(
                unarg
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
        || kfunc
            == Some(
                ctrlg
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
    {
        ktp = (&raw mut keytab as *mut key_tab).offset(0 as ::core::ffi::c_int as isize)
            as *mut key_tab;
        found = FALSE;
        while (*ktp).k_fp.is_some() {
            if (*ktp).k_fp == kfunc {
                unbindchar((*ktp).k_code);
            }
            ktp = ktp.offset(1);
        }
        if kfunc
            == Some(
                metafn
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
        {
            metac = c as ::core::ffi::c_int;
        }
        if kfunc
            == Some(
                cex
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
        {
            ctlxc = c as ::core::ffi::c_int;
        }
        if kfunc
            == Some(
                unarg
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
        {
            reptc = c as ::core::ffi::c_int;
        }
        if kfunc
            == Some(
                ctrlg
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            )
        {
            abortc = c as ::core::ffi::c_int;
        }
    }
    ktp = (&raw mut keytab as *mut key_tab).offset(0 as ::core::ffi::c_int as isize)
        as *mut key_tab;
    found = FALSE;
    while (*ktp).k_fp.is_some() {
        if (*ktp).k_code as ::core::ffi::c_uint == c {
            found = TRUE;
            break;
        } else {
            ktp = ktp.offset(1);
        }
    }
    if found != 0 {
        (*ktp).k_fp = kfunc
            as Option<
                unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
    } else {
        if ktp
            >= (&raw mut keytab as *mut key_tab).offset(NBINDS as isize) as *mut key_tab
        {
            mlwrite(b"Binding table FULL!\0" as *const u8 as *const ::core::ffi::c_char);
            return FALSE;
        }
        (*ktp).k_code = c as ::core::ffi::c_int;
        (*ktp).k_fp = kfunc
            as Option<
                unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        ktp = ktp.offset(1);
        (*ktp).k_code = 0 as ::core::ffi::c_int;
        (*ktp).k_fp = None;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn unbindkey(
    mut f: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = 0;
    let mut outseq: [::core::ffi::c_char; 80] = [0; 80];
    mlwrite(b": unbind-key \0" as *const u8 as *const ::core::ffi::c_char);
    c = getckey(FALSE) as ::core::ffi::c_int;
    cmdstr(
        c,
        (&raw mut outseq as *mut ::core::ffi::c_char)
            .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
    );
    ostring(&raw mut outseq as *mut ::core::ffi::c_char);
    if unbindchar(c) == FALSE {
        mlwrite(b"(Key not bound)\0" as *const u8 as *const ::core::ffi::c_char);
        return FALSE;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn unbindchar(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut ktp: *mut key_tab = ::core::ptr::null_mut::<key_tab>();
    let mut sktp: *mut key_tab = ::core::ptr::null_mut::<key_tab>();
    let mut found: ::core::ffi::c_int = 0;
    ktp = (&raw mut keytab as *mut key_tab).offset(0 as ::core::ffi::c_int as isize)
        as *mut key_tab;
    found = FALSE;
    while (*ktp).k_fp.is_some() {
        if (*ktp).k_code == c {
            found = TRUE;
            break;
        } else {
            ktp = ktp.offset(1);
        }
    }
    if found == 0 {
        return FALSE;
    }
    sktp = ktp;
    while (*ktp).k_fp.is_some() {
        ktp = ktp.offset(1);
    }
    ktp = ktp.offset(-1);
    (*sktp).k_code = (*ktp).k_code;
    (*sktp).k_fp = (*ktp).k_fp;
    (*ktp).k_code = 0 as ::core::ffi::c_int;
    (*ktp).k_fp = None;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn strinc(
    mut source: *mut ::core::ffi::c_char,
    mut sub: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut sp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut nxtsp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut tp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    sp = source;
    while *sp != 0 {
        tp = sub;
        nxtsp = sp;
        while *tp != 0 {
            let fresh8 = nxtsp;
            nxtsp = nxtsp.offset(1);
            if *fresh8 as ::core::ffi::c_int != *tp as ::core::ffi::c_int {
                break;
            }
            tp = tp.offset(1);
        }
        if *tp as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return TRUE;
        }
        sp = sp.offset(1);
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn getckey(mut mflag: ::core::ffi::c_int) -> ::core::ffi::c_uint {
    let mut c: ::core::ffi::c_uint = 0;
    let mut tok: [::core::ffi::c_char; 1024] = [0; 1024];
    if clexec != 0 {
        macarg(&raw mut tok as *mut ::core::ffi::c_char);
        return stock(&raw mut tok as *mut ::core::ffi::c_char);
    }
    if mflag != 0 {
        c = get1key() as ::core::ffi::c_uint;
    } else {
        c = getcmd() as ::core::ffi::c_uint;
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn startup(
    mut sfname: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut fname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    if *sfname as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        fname = flook(sfname, TRUE);
    } else {
        fname = flook(pathname[0 as ::core::ffi::c_int as usize], TRUE);
    }
    if fname.is_null() {
        return TRUE;
    }
    return dofile(fname);
}
#[no_mangle]
pub unsafe extern "C" fn flook(
    mut fname: *mut ::core::ffi::c_char,
    mut hflag: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut home: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut sp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut i: ::core::ffi::c_int = 0;
    static mut fspec: [::core::ffi::c_char; 1024] = [0; 1024];
    if hflag != 0 {
        home = getenv(b"HOME\0" as *const u8 as *const ::core::ffi::c_char);
        if !home.is_null() {
            snprintf(
                &raw mut fspec as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
                b"%s/%s\0" as *const u8 as *const ::core::ffi::c_char,
                home,
                fname,
            );
            if ffropen(&raw mut fspec as *mut ::core::ffi::c_char) == FIOSUC {
                ffclose();
                return &raw mut fspec as *mut ::core::ffi::c_char;
            }
            snprintf(
                &raw mut fspec as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
                b"%s/lib/%s\0" as *const u8 as *const ::core::ffi::c_char,
                home,
                fname,
            );
            if ffropen(&raw mut fspec as *mut ::core::ffi::c_char) == FIOSUC {
                ffclose();
                return &raw mut fspec as *mut ::core::ffi::c_char;
            }
        }
    }
    if ffropen(fname) == FIOSUC {
        ffclose();
        return fname;
    }
    path = getenv(b"PATH\0" as *const u8 as *const ::core::ffi::c_char);
    if !path.is_null() {
        while *path != 0 {
            sp = &raw mut fspec as *mut ::core::ffi::c_char;
            while *path as ::core::ffi::c_int != 0
                && *path as ::core::ffi::c_int != PATHCHR
            {
                let fresh9 = path;
                path = path.offset(1);
                let fresh10 = sp;
                sp = sp.offset(1);
                *fresh10 = *fresh9;
            }
            if sp != &raw mut fspec as *mut ::core::ffi::c_char {
                let fresh11 = sp;
                sp = sp.offset(1);
                *fresh11 = '/' as i32 as ::core::ffi::c_char;
            }
            *sp = 0 as ::core::ffi::c_char;
            strcat(&raw mut fspec as *mut ::core::ffi::c_char, fname);
            if ffropen(&raw mut fspec as *mut ::core::ffi::c_char) == FIOSUC {
                ffclose();
                return &raw mut fspec as *mut ::core::ffi::c_char;
            }
            if *path as ::core::ffi::c_int == PATHCHR {
                path = path.offset(1);
            }
        }
    }
    i = 2 as ::core::ffi::c_int;
    while (i as usize)
        < (::core::mem::size_of::<[*mut ::core::ffi::c_char; 10]>() as usize)
            .wrapping_div(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
    {
        strcpy(&raw mut fspec as *mut ::core::ffi::c_char, pathname[i as usize]);
        strcat(&raw mut fspec as *mut ::core::ffi::c_char, fname);
        if ffropen(&raw mut fspec as *mut ::core::ffi::c_char) == FIOSUC {
            ffclose();
            return &raw mut fspec as *mut ::core::ffi::c_char;
        }
        i += 1;
    }
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn cmdstr(
    mut c: ::core::ffi::c_int,
    mut seq: *mut ::core::ffi::c_char,
) {
    let mut ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    ptr = seq;
    if c & META != 0 {
        let fresh0 = ptr;
        ptr = ptr.offset(1);
        *fresh0 = 'M' as i32 as ::core::ffi::c_char;
        let fresh1 = ptr;
        ptr = ptr.offset(1);
        *fresh1 = '-' as i32 as ::core::ffi::c_char;
    }
    if c & CTLX != 0 {
        let fresh2 = ptr;
        ptr = ptr.offset(1);
        *fresh2 = '^' as i32 as ::core::ffi::c_char;
        let fresh3 = ptr;
        ptr = ptr.offset(1);
        *fresh3 = 'X' as i32 as ::core::ffi::c_char;
    }
    if c as ::core::ffi::c_uint & SPEC != 0 {
        let fresh4 = ptr;
        ptr = ptr.offset(1);
        *fresh4 = 'F' as i32 as ::core::ffi::c_char;
        let fresh5 = ptr;
        ptr = ptr.offset(1);
        *fresh5 = 'N' as i32 as ::core::ffi::c_char;
    }
    if c & CONTROL != 0 {
        let fresh6 = ptr;
        ptr = ptr.offset(1);
        *fresh6 = '^' as i32 as ::core::ffi::c_char;
    }
    let fresh7 = ptr;
    ptr = ptr.offset(1);
    *fresh7 = (c & 255 as ::core::ffi::c_int) as ::core::ffi::c_char;
    *ptr = 0 as ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn getbind(mut c: ::core::ffi::c_int) -> fn_t {
    let mut ktp: *mut key_tab = ::core::ptr::null_mut::<key_tab>();
    ktp = (&raw mut keytab as *mut key_tab).offset(0 as ::core::ffi::c_int as isize)
        as *mut key_tab;
    while (*ktp).k_fp.is_some() {
        if (*ktp).k_code == c {
            return (*ktp).k_fp as fn_t;
        }
        ktp = ktp.offset(1);
    }
    return None;
}
#[no_mangle]
pub unsafe extern "C" fn getfname(mut func: fn_t) -> *mut ::core::ffi::c_char {
    let mut nptr: *mut name_bind = ::core::ptr::null_mut::<name_bind>();
    nptr = (&raw mut names as *mut name_bind).offset(0 as ::core::ffi::c_int as isize)
        as *mut name_bind;
    while (*nptr).n_func.is_some() {
        if (*nptr).n_func == func {
            return (*nptr).n_name;
        }
        nptr = nptr.offset(1);
    }
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn fncmatch(mut fname: *mut ::core::ffi::c_char) -> fn_t {
    let mut ffp: *mut name_bind = ::core::ptr::null_mut::<name_bind>();
    ffp = (&raw mut names as *mut name_bind).offset(0 as ::core::ffi::c_int as isize)
        as *mut name_bind;
    while (*ffp).n_func.is_some() {
        if strcmp(fname, (*ffp).n_name) == 0 as ::core::ffi::c_int {
            return (*ffp).n_func as fn_t;
        }
        ffp = ffp.offset(1);
    }
    return None;
}
#[no_mangle]
pub unsafe extern "C" fn stock(
    mut keyname: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_uint {
    let mut c: ::core::ffi::c_uint = 0;
    c = 0 as ::core::ffi::c_uint;
    if *keyname as ::core::ffi::c_int == 'M' as i32
        && *keyname.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '-' as i32
    {
        c = META as ::core::ffi::c_uint;
        keyname = keyname.offset(2 as ::core::ffi::c_int as isize);
    }
    if *keyname as ::core::ffi::c_int == 'F' as i32
        && *keyname.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'N' as i32
    {
        c |= SPEC;
        keyname = keyname.offset(2 as ::core::ffi::c_int as isize);
    }
    if *keyname as ::core::ffi::c_int == '^' as i32
        && *keyname.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'X' as i32 && c & SPEC == 0
    {
        c |= CTLX as ::core::ffi::c_uint;
        keyname = keyname.offset(2 as ::core::ffi::c_int as isize);
    }
    if *keyname as ::core::ffi::c_int == '^' as i32
        && *keyname.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
    {
        c |= CONTROL as ::core::ffi::c_uint;
        keyname = keyname.offset(1);
    }
    if (*keyname as ::core::ffi::c_int) < 32 as ::core::ffi::c_int {
        c |= CONTROL as ::core::ffi::c_uint;
        *keyname = (*keyname as ::core::ffi::c_int + 'A' as i32) as ::core::ffi::c_char;
    }
    if *keyname as ::core::ffi::c_int >= 'a' as i32
        && *keyname as ::core::ffi::c_int <= 'z' as i32 && c & SPEC == 0
    {
        *keyname = (*keyname as ::core::ffi::c_int - 32 as ::core::ffi::c_int)
            as ::core::ffi::c_char;
    }
    c |= *keyname as ::core::ffi::c_uint;
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn transbind(
    mut skey: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut bindname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    bindname = getfname(getbind(stock(skey) as ::core::ffi::c_int));
    if bindname.is_null() {
        bindname = b"ERROR\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
    }
    return bindname;
}
static mut pathname: [*mut ::core::ffi::c_char; 10] = [
    b".emacsrc\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"emacs.hlp\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"/usr/share/nanox/\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"/usr/local/share/nanox/\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"/usr/global/lib/\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"/usr/local/bin/\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"/usr/local/lib/\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"/usr/local/\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"/usr/lib/\0" as *const u8 as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char,
    b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
];
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
