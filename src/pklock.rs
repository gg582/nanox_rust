extern "C" {
    fn lstat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn lseek(
        __fd: ::core::ffi::c_int,
        __offset: __off_t,
        __whence: ::core::ffi::c_int,
    ) -> __off_t;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(
        __fd: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_void,
        __nbytes: size_t,
    ) -> ssize_t;
    fn write(
        __fd: ::core::ffi::c_int,
        __buf: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ssize_t;
    fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn gethostname(
        __name: *mut ::core::ffi::c_char,
        __len: size_t,
    ) -> ::core::ffi::c_int;
    fn cuserid(__s: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcat(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn __errno_location() -> *mut ::core::ffi::c_int;
}
pub type size_t = usize;
pub type ssize_t = isize;
pub type __off_t = ::core::ffi::c_long;
pub type __mode_t = ::core::ffi::c_uint;
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
pub type __dev_t = ::core::ffi::c_ulong;
pub type __gid_t = ::core::ffi::c_uint;
pub type __uid_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __ino_t = ::core::ffi::c_ulong;
pub const MAXNAME: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn dolock(
    mut fname: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut fd: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    static mut lname: [::core::ffi::c_char; 512] = [0; 512];
    static mut locker: [::core::ffi::c_char; 129] = [0; 129];
    let mut mask: ::core::ffi::c_int = 0;
    let mut sbuf: stat = stat {
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
    strcat(
        strcpy(&raw mut lname as *mut ::core::ffi::c_char, fname),
        b".lock~\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if lstat(&raw mut lname as *mut ::core::ffi::c_char, &raw mut sbuf)
        == 0 as ::core::ffi::c_int
    {
        if !(sbuf.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t) {
            return b"LOCK ERROR: not a regular file\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
    }
    mask = umask(0 as __mode_t) as ::core::ffi::c_int;
    fd = open(
        &raw mut lname as *mut ::core::ffi::c_char,
        O_RDWR | O_CREAT,
        0o666 as ::core::ffi::c_int,
    );
    umask(mask as __mode_t);
    if fd < 0 as ::core::ffi::c_int {
        if *__errno_location() == EACCES {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if *__errno_location() == EROFS {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        return b"LOCK ERROR: cannot access lock file\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    }
    n = read(
        fd,
        &raw mut locker as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        MAXNAME as size_t,
    ) as ::core::ffi::c_int;
    if n < 1 as ::core::ffi::c_int {
        lseek(fd, 0 as __off_t, SEEK_SET);
        cuserid(&raw mut locker as *mut ::core::ffi::c_char);
        strcat(
            (&raw mut locker as *mut ::core::ffi::c_char)
                .offset(strlen(&raw mut locker as *mut ::core::ffi::c_char) as isize),
            b"@\0" as *const u8 as *const ::core::ffi::c_char,
        );
        gethostname(
            (&raw mut locker as *mut ::core::ffi::c_char)
                .offset(strlen(&raw mut locker as *mut ::core::ffi::c_char) as isize),
            64 as size_t,
        );
        write(
            fd,
            &raw mut locker as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            strlen(&raw mut locker as *mut ::core::ffi::c_char),
        );
        close(fd);
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    locker[(if n > MAXNAME { MAXNAME } else { n }) as usize] = 0 as ::core::ffi::c_char;
    return &raw mut locker as *mut ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn undolock(
    mut fname: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    static mut lname: [::core::ffi::c_char; 512] = [0; 512];
    strcat(
        strcpy(&raw mut lname as *mut ::core::ffi::c_char, fname),
        b".lock~\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if unlink(&raw mut lname as *mut ::core::ffi::c_char) != 0 as ::core::ffi::c_int {
        if *__errno_location() == EACCES || *__errno_location() == ENOENT {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if *__errno_location() == EROFS {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        return b"LOCK ERROR: cannot remove lock file\0" as *const u8
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    }
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EACCES: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const EROFS: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const O_RDWR: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
