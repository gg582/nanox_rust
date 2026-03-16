extern "C" {
    fn wcwidth(__c: wchar_t) -> ::core::ffi::c_int;
}
pub type wchar_t = ::libc::wchar_t;
pub type unicode_t = ::core::ffi::c_uint;
#[no_mangle]
pub unsafe extern "C" fn utf8_to_unicode(
    mut line: *mut ::core::ffi::c_uchar,
    mut index: ::core::ffi::c_uint,
    mut len: ::core::ffi::c_uint,
    mut res: *mut unicode_t,
) -> ::core::ffi::c_uint {
    let mut value: ::core::ffi::c_uint = 0;
    let mut c: ::core::ffi::c_uchar = *line.offset(index as isize);
    let mut bytes: ::core::ffi::c_uint = 0;
    let mut mask: ::core::ffi::c_uint = 0;
    let mut i: ::core::ffi::c_uint = 0;
    *res = c as unicode_t;
    line = line.offset(index as isize);
    len = len.wrapping_sub(index);
    if (c as ::core::ffi::c_int) < 0xc0 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_uint;
    }
    mask = 0x20 as ::core::ffi::c_uint;
    bytes = 2 as ::core::ffi::c_uint;
    while c as ::core::ffi::c_uint & mask != 0 {
        bytes = bytes.wrapping_add(1);
        mask >>= 1 as ::core::ffi::c_int;
    }
    if bytes > 6 as ::core::ffi::c_uint {
        return 1 as ::core::ffi::c_uint;
    }
    if bytes > len {
        *res = 0xfffd as unicode_t;
        return 1 as ::core::ffi::c_uint;
    }
    value = c as ::core::ffi::c_uint & mask.wrapping_sub(1 as ::core::ffi::c_uint);
    i = 1 as ::core::ffi::c_uint;
    while i < bytes {
        c = *line.offset(i as isize);
        if c as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
            != 0x80 as ::core::ffi::c_int
        {
            return 1 as ::core::ffi::c_uint;
        }
        value = value << 6 as ::core::ffi::c_int
            | (c as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int)
                as ::core::ffi::c_uint;
        i = i.wrapping_add(1);
    }
    *res = value as unicode_t;
    return bytes;
}
unsafe extern "C" fn reverse_string(
    mut begin: *mut ::core::ffi::c_uchar,
    mut end: *mut ::core::ffi::c_uchar,
) {
    loop {
        let mut a: ::core::ffi::c_char = *begin as ::core::ffi::c_char;
        let mut b: ::core::ffi::c_char = *end as ::core::ffi::c_char;
        *end = a as ::core::ffi::c_uchar;
        *begin = b as ::core::ffi::c_uchar;
        begin = begin.offset(1);
        end = end.offset(-1);
        if !(begin < end) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn unicode_to_utf8(
    mut c: ::core::ffi::c_uint,
    mut utf8: *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_uint {
    let mut bytes: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    *utf8 = c as ::core::ffi::c_uchar;
    if c > 0x7f as ::core::ffi::c_uint {
        let mut prefix: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
        let mut p: *mut ::core::ffi::c_char = utf8 as *mut ::core::ffi::c_char;
        loop {
            let fresh0 = p;
            p = p.offset(1);
            *fresh0 = (0x80 as ::core::ffi::c_uint)
                .wrapping_add(c & 0x3f as ::core::ffi::c_uint) as ::core::ffi::c_char;
            bytes += 1;
            prefix >>= 1 as ::core::ffi::c_int;
            c >>= 6 as ::core::ffi::c_int;
            if !(c >= prefix as ::core::ffi::c_uint) {
                break;
            }
        }
        *p = c.wrapping_sub((2 as ::core::ffi::c_int * prefix) as ::core::ffi::c_uint)
            as ::core::ffi::c_char;
        reverse_string(utf8, p as *mut ::core::ffi::c_uchar);
    }
    return bytes as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn unicode_width(mut c: unicode_t) -> ::core::ffi::c_int {
    let mut width: ::core::ffi::c_int = 0;
    if c < 0x20 as unicode_t || c == 0x7f as unicode_t {
        return 2 as ::core::ffi::c_int;
    }
    if c >= 0x80 as unicode_t && c <= 0xa0 as unicode_t {
        return 3 as ::core::ffi::c_int;
    }
    width = wcwidth(c as wchar_t);
    if width < 0 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    return width;
}
