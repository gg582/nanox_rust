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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn getuid() -> __uid_t;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut ::core::ffi::c_char,
    pub pw_passwd: *mut ::core::ffi::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut ::core::ffi::c_char,
    pub pw_dir: *mut ::core::ffi::c_char,
    pub pw_shell: *mut ::core::ffi::c_char,
}
pub type __gid_t = ::core::ffi::c_uint;
pub type __uid_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::core::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type __syscall_slong_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __time_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __off_t = ::core::ffi::c_long;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __ino_t = ::core::ffi::c_ulong;
pub const PATH_SEP: ::core::ffi::c_int = '/' as i32;
pub const PATH_SEP_STR: [::core::ffi::c_char; 2] = unsafe {
    ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"/\0")
};
#[no_mangle]
pub unsafe extern "C" fn nanox_getenv(
    mut name: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    return getenv(name);
}
#[no_mangle]
pub unsafe extern "C" fn nanox_get_user_data_dir(
    mut out: *mut ::core::ffi::c_char,
    mut cap: size_t,
) {
    let mut xdg_data: *const ::core::ffi::c_char = getenv(
        b"XDG_DATA_HOME\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if !xdg_data.is_null() && *xdg_data as ::core::ffi::c_int != 0 {
        snprintf(
            out,
            cap,
            b"%s/nanox\0" as *const u8 as *const ::core::ffi::c_char,
            xdg_data,
        );
    } else {
        let mut home: *const ::core::ffi::c_char = getenv(
            b"HOME\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if home.is_null() {
            let mut pw: *mut passwd = getpwuid(getuid());
            if !pw.is_null() {
                home = (*pw).pw_dir;
            }
        }
        if !home.is_null() {
            snprintf(
                out,
                cap,
                b"%s/.local/share/nanox\0" as *const u8 as *const ::core::ffi::c_char,
                home,
            );
        } else {
            snprintf(
                out,
                cap,
                b"/tmp/nanox\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn nanox_get_user_config_dir(
    mut out: *mut ::core::ffi::c_char,
    mut cap: size_t,
) {
    let mut xdg_config: *const ::core::ffi::c_char = getenv(
        b"XDG_CONFIG_HOME\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if !xdg_config.is_null() && *xdg_config as ::core::ffi::c_int != 0 {
        snprintf(
            out,
            cap,
            b"%s/nanox\0" as *const u8 as *const ::core::ffi::c_char,
            xdg_config,
        );
    } else {
        let mut home: *const ::core::ffi::c_char = getenv(
            b"HOME\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if home.is_null() {
            let mut pw: *mut passwd = getpwuid(getuid());
            if !pw.is_null() {
                home = (*pw).pw_dir;
            }
        }
        if !home.is_null() {
            snprintf(
                out,
                cap,
                b"%s/.config/nanox\0" as *const u8 as *const ::core::ffi::c_char,
                home,
            );
        } else {
            snprintf(
                out,
                cap,
                b"/tmp/nanox\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn nanox_path_join(
    mut out: *mut ::core::ffi::c_char,
    mut cap: size_t,
    mut a: *const ::core::ffi::c_char,
    mut b: *const ::core::ffi::c_char,
) {
    let mut len_a: size_t = strlen(a);
    let mut len_b: size_t = strlen(b);
    if len_a.wrapping_add(len_b).wrapping_add(2 as size_t) > cap {
        if cap > 0 as size_t {
            *out.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32
                as ::core::ffi::c_char;
        }
        return;
    }
    if out != a as *mut ::core::ffi::c_char {
        strcpy(out, a);
    }
    let mut a_ends_sep: bool = len_a > 0 as size_t
        && *out.offset(len_a.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
            == PATH_SEP;
    let mut b_starts_sep: bool = len_b > 0 as size_t
        && *b.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == PATH_SEP;
    if a_ends_sep as ::core::ffi::c_int != 0 && b_starts_sep as ::core::ffi::c_int != 0 {
        strcat(out, b.offset(1 as ::core::ffi::c_int as isize));
    } else if !a_ends_sep && !b_starts_sep {
        strcat(out, PATH_SEP_STR.as_ptr());
        strcat(out, b);
    } else {
        strcat(out, b);
    };
}
#[no_mangle]
pub unsafe extern "C" fn nanox_file_exists(
    mut path: *const ::core::ffi::c_char,
) -> bool {
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    return stat(path, &raw mut st) == 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nanox_normalize_path(mut path: *mut ::core::ffi::c_char) {
    if path.is_null() {
        return;
    }
    let mut p: *mut ::core::ffi::c_char = path;
    while *p != 0 {
        if *p as ::core::ffi::c_int == '\\' as i32 {
            *p = '/' as i32 as ::core::ffi::c_char;
        }
        p = p.offset(1);
    }
}
