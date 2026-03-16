extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    static mut term: *mut terminal;
    fn vttflush();
    fn utf8_to_unicode(
        line: *mut ::core::ffi::c_uchar,
        index: ::core::ffi::c_uint,
        len: ::core::ffi::c_uint,
        res: *mut unicode_t,
    ) -> ::core::ffi::c_uint;
    fn unicode_width(c: unicode_t) -> ::core::ffi::c_int;
    fn vtmove(row: ::core::ffi::c_int, col: ::core::ffi::c_int);
    fn updupd(force: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn mlwrite(fmt: *const ::core::ffi::c_char, ...);
    fn vtputc(c: ::core::ffi::c_int);
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
pub type unicode_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct video_cell {
    pub ch: unicode_t,
    pub fg: ::core::ffi::c_int,
    pub bg: ::core::ffi::c_int,
    pub bold: bool,
    pub underline: bool,
    pub italic: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct video {
    pub v_flag: ::core::ffi::c_int,
    pub v_text: [video_cell; 1],
}
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn mystrnlen_raw_w(mut c: unicode_t) -> ::core::ffi::c_int {
    if c >= 0x4e00 as unicode_t && c <= 0x9fff as unicode_t {
        return 2 as ::core::ffi::c_int;
    }
    if c >= 0xac00 as unicode_t && c <= 0xd7af as unicode_t {
        return 2 as ::core::ffi::c_int;
    }
    if c >= 0x3040 as unicode_t && c <= 0x309f as unicode_t {
        return 2 as ::core::ffi::c_int;
    }
    if c >= 0x30a0 as unicode_t && c <= 0x30ff as unicode_t {
        return 2 as ::core::ffi::c_int;
    }
    return unicode_width(c);
}
pub const VFCHG: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
static mut paste_slot_buffer: *mut ::core::ffi::c_char = ::core::ptr::null::<
    ::core::ffi::c_char,
>() as *mut ::core::ffi::c_char;
static mut paste_slot_size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut paste_slot_capacity: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut paste_slot_active: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn paste_slot_init() {
    if paste_slot_buffer.is_null() {
        paste_slot_capacity = 16384 as ::core::ffi::c_int;
        paste_slot_buffer = malloc(paste_slot_capacity as size_t)
            as *mut ::core::ffi::c_char;
        if !paste_slot_buffer.is_null() {
            paste_slot_size = 0 as ::core::ffi::c_int;
            *paste_slot_buffer.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32
                as ::core::ffi::c_char;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn paste_slot_add_char(
    mut c: ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if paste_slot_buffer.is_null() {
        paste_slot_init();
    }
    if paste_slot_buffer.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if paste_slot_size + 1 as ::core::ffi::c_int >= paste_slot_capacity {
        let mut new_capacity: ::core::ffi::c_int = paste_slot_capacity
            * 2 as ::core::ffi::c_int;
        let mut new_buffer: *mut ::core::ffi::c_char = realloc(
            paste_slot_buffer as *mut ::core::ffi::c_void,
            new_capacity as size_t,
        ) as *mut ::core::ffi::c_char;
        if new_buffer.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        paste_slot_buffer = new_buffer;
        paste_slot_capacity = new_capacity;
    }
    let fresh0 = paste_slot_size;
    paste_slot_size = paste_slot_size + 1;
    *paste_slot_buffer.offset(fresh0 as isize) = c;
    *paste_slot_buffer.offset(paste_slot_size as isize) = '\0' as i32
        as ::core::ffi::c_char;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn paste_slot_clear() {
    if !paste_slot_buffer.is_null() {
        paste_slot_size = 0 as ::core::ffi::c_int;
        *paste_slot_buffer.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32
            as ::core::ffi::c_char;
    }
}
#[no_mangle]
pub unsafe extern "C" fn paste_slot_free() {
    if !paste_slot_buffer.is_null() {
        free(paste_slot_buffer as *mut ::core::ffi::c_void);
        paste_slot_buffer = ::core::ptr::null_mut::<::core::ffi::c_char>();
        paste_slot_size = 0 as ::core::ffi::c_int;
        paste_slot_capacity = 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn paste_slot_get_content() -> *mut ::core::ffi::c_char {
    return paste_slot_buffer;
}
#[no_mangle]
pub unsafe extern "C" fn paste_slot_get_size() -> ::core::ffi::c_int {
    return paste_slot_size;
}
#[no_mangle]
pub unsafe extern "C" fn paste_slot_set_active(mut active: ::core::ffi::c_int) {
    paste_slot_active = active;
}
#[no_mangle]
pub unsafe extern "C" fn paste_slot_is_active() -> ::core::ffi::c_int {
    return paste_slot_active;
}
#[no_mangle]
pub unsafe extern "C" fn paste_slot_display() {
    extern "C" {
        static mut vscreen: *mut *mut video;
    }
    let mut col: ::core::ffi::c_int = 0;
    let mut content: *mut ::core::ffi::c_char = paste_slot_buffer;
    let mut content_len: ::core::ffi::c_int = paste_slot_size;
    let mut current_row: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut byte_pos: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut title: *mut ::core::ffi::c_char = b" PASTE SLOT \0" as *const u8
        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*term).t_nrow as ::core::ffi::c_int {
        (**vscreen.offset(i as isize)).v_flag |= VFCHG;
        vtmove(i, 0 as ::core::ffi::c_int);
        extern "C" {
            #[link_name = "vteeol"]
            fn vteeol_0();
        }
        vteeol_0();
        i += 1;
    }
    vtmove(0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    vtputc('+' as i32);
    col = 1 as ::core::ffi::c_int;
    while col < (*term).t_ncol as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
        vtputc('-' as i32);
        col += 1;
    }
    vtputc('+' as i32);
    vtmove(1 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    vtputc('|' as i32);
    vtmove(1 as ::core::ffi::c_int, 2 as ::core::ffi::c_int);
    while *title != 0 {
        let fresh1 = title;
        title = title.offset(1);
        vtputc(*fresh1 as ::core::ffi::c_int);
    }
    vtmove(
        1 as ::core::ffi::c_int,
        (*term).t_ncol as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
    );
    vtputc('|' as i32);
    vtmove(2 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    vtputc('+' as i32);
    col = 1 as ::core::ffi::c_int;
    while col < (*term).t_ncol as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
        vtputc('-' as i32);
        col += 1;
    }
    vtputc('+' as i32);
    current_row = 3 as ::core::ffi::c_int;
    while byte_pos < content_len
        && current_row < (*term).t_nrow as ::core::ffi::c_int - 1 as ::core::ffi::c_int
    {
        vtmove(current_row, 0 as ::core::ffi::c_int);
        vtputc('|' as i32);
        col = 1 as ::core::ffi::c_int;
        while byte_pos < content_len
            && col < (*term).t_ncol as ::core::ffi::c_int - 1 as ::core::ffi::c_int
        {
            let mut c: ::core::ffi::c_uchar = *content.offset(byte_pos as isize)
                as ::core::ffi::c_uchar;
            if c as ::core::ffi::c_int == '\n' as i32
                || c as ::core::ffi::c_int == '\r' as i32
            {
                if c as ::core::ffi::c_int == '\r' as i32
                    && (byte_pos + 1 as ::core::ffi::c_int) < content_len
                    && *content.offset((byte_pos + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int == '\n' as i32
                {
                    byte_pos += 1;
                }
                byte_pos += 1;
                break;
            } else {
                let mut uc: unicode_t = 0;
                let mut bytes: ::core::ffi::c_int = utf8_to_unicode(
                    content as *mut ::core::ffi::c_uchar,
                    byte_pos as ::core::ffi::c_uint,
                    content_len as ::core::ffi::c_uint,
                    &raw mut uc,
                ) as ::core::ffi::c_int;
                vtputc(uc as ::core::ffi::c_int);
                byte_pos += bytes;
                col += mystrnlen_raw_w(uc);
            }
        }
        while col < (*term).t_ncol as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
            vtputc(' ' as i32);
            col += 1;
        }
        vtmove(
            current_row,
            (*term).t_ncol as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
        );
        vtputc('|' as i32);
        current_row += 1;
    }
    while current_row < (*term).t_nrow as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
        vtmove(current_row, 0 as ::core::ffi::c_int);
        vtputc('|' as i32);
        col = 1 as ::core::ffi::c_int;
        while col < (*term).t_ncol as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
            vtputc(' ' as i32);
            col += 1;
        }
        vtmove(
            current_row,
            (*term).t_ncol as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
        );
        vtputc('|' as i32);
        current_row += 1;
    }
    vtmove(
        (*term).t_nrow as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    vtputc('+' as i32);
    col = 1 as ::core::ffi::c_int;
    while col < (*term).t_ncol as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
        vtputc('-' as i32);
        col += 1;
    }
    vtputc('+' as i32);
    updupd(TRUE);
    mlwrite(
        b"Press 'p' or Enter to paste, ESC to cancel\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    vttflush();
}
#[no_mangle]
pub unsafe extern "C" fn paste_slot_insert() -> ::core::ffi::c_int {
    extern "C" {
        #[link_name = "linsert_block"]
        fn linsert_block_0(
            block: *const ::core::ffi::c_char,
            len: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
    }
    if paste_slot_buffer.is_null() || paste_slot_size == 0 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    mlwrite(b"Pasting...\0" as *const u8 as *const ::core::ffi::c_char);
    return linsert_block_0(paste_slot_buffer, paste_slot_size);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
