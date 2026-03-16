extern "C" {
    fn reserve_jump_numeric_mode(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn wrapword(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn backword(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn forwword(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn upperword(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn lowerword(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn capword(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn delfword(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn delbword(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn fillpara(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn justpara(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn killpara(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn wordcount(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn reposition(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn redraw(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn newsize(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn newwidth(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn gotobol(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn backchar(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn gotoeol(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn forwchar(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn gotoline(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn gotobob(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn gotoeob(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn forwline(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn backline(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn gotobop(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn gotoeop(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn forwpage(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn backpage(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn setmark(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn swapmark(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn setfillcol(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn showcpos(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn twiddle(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn quote(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn insert_tab(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn detab(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn entab(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn trim(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn openline(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn insert_newline(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn deblank(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn indent(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn indent_start_set(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn indent_end_set(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn outdent_start_set(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn outdent_end_set(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn indent_apply_range(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn indent_cancel(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn forwdel(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn backdel(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn killtext(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn setemode(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn delmode(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn setgmode(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn delgmode(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn clrmes(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn writemsg(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn getfence(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn istring(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn ovstring(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn quickexit(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn quit(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn ctlxlp(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn ctlxrp(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn ctlxe(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn ctrlg(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn nullproc(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn metafn(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cex(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn unarg(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn upscreen(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn killregion(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn copyregion(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn lowerregion(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn upperregion(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn deskey(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn bindtokey(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn unbindkey(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn usebuffer(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn nextbuffer(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn killbuffer(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn namebuffer(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn unmark(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn fileread(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn insfile(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn filefind(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn viewfile(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cutln_end_cut(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cutln_start_cut(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn cutln_end_copy(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn cutln_start_copy(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn cutln_paste_menu(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn cutln_cut_current_line(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sed_replace_command(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn filewrite(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn filesave(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn filename(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn namedcmd(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn execcmd(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn storemac(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn storeproc(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn execproc(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn execbuf(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn execfile(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf1(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf2(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf3(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf4(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf5(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf6(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf7(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf8(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf9(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf10(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf11(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf12(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf13(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf14(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf15(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf16(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf17(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf18(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf19(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf20(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf21(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf22(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf23(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf24(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf25(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf26(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf27(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf28(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf29(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf30(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf31(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf32(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf33(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf34(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf35(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf36(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf37(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf38(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf39(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cbuf40(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn spawncli(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn bktoshell(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn spawn(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn execprg(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn filter_buffer(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn forwsearch(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn forwhunt(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn backsearch(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn backhunt(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn nanox_search_engine(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn risearch(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn fisearch(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn setvar(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cscope_complete(
        f: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn insspace(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn yank(f: ::core::ffi::c_int, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
#[no_mangle]
pub static mut names: [name_bind; 164] = {
    [
        name_bind {
            n_name: b"abort-command\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                ctrlg
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"add-mode\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                setemode
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"add-global-mode\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                setgmode
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"backward-character\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                backchar
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"begin-macro\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                ctlxlp
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"beginning-of-file\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                gotobob
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"beginning-of-line\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                gotobol
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"bind-to-key\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                bindtokey
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"buffer-position\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                showcpos
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"case-region-lower\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                lowerregion
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"case-region-upper\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                upperregion
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"case-word-capitalize\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                capword
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"case-word-lower\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                lowerword
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"case-word-upper\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                upperword
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"change-file-name\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                filename
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"change-screen-size\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                newsize
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"change-screen-width\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                newwidth
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"clear-and-redraw\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                redraw
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"clear-message-line\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                clrmes
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"copy-region\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                copyregion
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"count-words\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                wordcount
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"cscope-complete\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cscope_complete
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"ctlx-prefix\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cex
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"cutln-start-copy\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cutln_start_copy
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"cutln-end-copy\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cutln_end_copy
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"cutln-start-cut\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cutln_start_cut
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"cutln-end-cut\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cutln_end_cut
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"cutln-paste-menu\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cutln_paste_menu
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"cutln-cut-current-line\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            n_func: Some(
                cutln_cut_current_line
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"delete-blank-lines\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                deblank
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"delete-buffer\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                killbuffer
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"delete-mode\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                delmode
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"delete-global-mode\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                delgmode
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"delete-next-character\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                forwdel
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"delete-next-word\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                delfword
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"delete-previous-character\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            n_func: Some(
                backdel
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"delete-previous-word\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                delbword
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"describe-key\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                deskey
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"detab-line\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                detab
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"end-macro\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                ctlxrp
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"end-of-file\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                gotoeob
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"end-of-line\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                gotoeol
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"entab-line\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                entab
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"exchange-point-and-mark\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            n_func: Some(
                swapmark
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-buffer\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                execbuf
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-command-line\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                execcmd
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-file\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                execfile
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                ctlxe
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-1\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf1
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-2\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf2
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-3\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf3
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-4\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf4
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-5\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf5
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-6\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf6
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-7\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf7
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-8\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf8
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-9\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf9
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-10\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf10
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-11\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf11
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-12\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf12
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-13\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf13
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-14\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf14
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-15\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf15
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-16\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf16
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-17\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf17
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-18\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf18
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-19\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf19
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-20\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf20
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-21\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf21
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-22\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf22
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-23\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf23
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-24\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf24
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-25\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf25
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-26\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf26
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-27\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf27
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-28\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf28
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-29\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf29
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-30\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf30
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-31\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf31
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-32\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf32
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-33\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf33
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-34\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf34
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-35\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf35
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-36\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf36
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-37\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf37
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-38\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf38
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-39\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf39
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-macro-40\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                cbuf40
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-named-command\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                namedcmd
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-procedure\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                execproc
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"execute-program\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                execprg
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"exit-emacs\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                quit
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"fill-paragraph\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                fillpara
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"filter-buffer\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                filter_buffer
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"find-file\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                filefind
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"forward-character\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                forwchar
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"goto-line\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                gotoline
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"goto-matching-fence\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                getfence
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"handle-tab\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                insert_tab
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"hunt-forward\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                forwhunt
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"hunt-backward\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                backhunt
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"i-shell\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                spawncli
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"indent-apply-range\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                indent_apply_range
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"indent-cancel\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                indent_cancel
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"indent-start-set\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                indent_start_set
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"indent-end-set\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                indent_end_set
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"incremental-search\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                fisearch
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"insert-file\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                insfile
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"insert-space\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                insspace
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"insert-string\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                istring
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"justify-paragraph\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                justpara
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"kill-paragraph\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                killpara
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"kill-region\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                killregion
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"kill-to-end-of-line\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                killtext
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"meta-prefix\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                metafn
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"name-buffer\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                namebuffer
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"nanox-search\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                nanox_search_engine
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"sed-replace\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                sed_replace_command
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"slot-numeric-mode\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                reserve_jump_numeric_mode
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"newline\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                insert_newline
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"newline-and-indent\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                indent
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"next-buffer\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                nextbuffer
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"next-line\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                forwline
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"next-page\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                forwpage
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"next-paragraph\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                gotoeop
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"next-word\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                forwword
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"nop\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                nullproc
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"open-line\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                openline
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"outdent-start-set\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                outdent_start_set
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"outdent-end-set\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                outdent_end_set
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"overwrite-string\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                ovstring
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"previous-line\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                backline
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"previous-page\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                backpage
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"previous-paragraph\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                gotobop
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"previous-word\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                backword
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"quick-exit\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                quickexit
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"quote-character\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                quote
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"read-file\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                fileread
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"redraw-display\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                reposition
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"reverse-incremental-search\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            n_func: Some(
                risearch
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"run\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                execproc
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"save-file\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                filesave
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"search-forward\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                forwsearch
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"search-reverse\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                backsearch
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"select-buffer\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                usebuffer
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"set\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                setvar
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"set-fill-column\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                setfillcol
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"set-mark\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                setmark
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"shell-command\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                spawn
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"store-macro\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                storemac
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"store-procedure\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                storeproc
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"suspend-emacs\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                bktoshell
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"transpose-characters\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                twiddle
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"trim-line\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                trim
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"unbind-key\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                unbindkey
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"universal-argument\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                unarg
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"unmark-buffer\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                unmark
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"update-screen\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                upscreen
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"view-file\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                viewfile
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"wrap-word\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                wrapword
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"write-file\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                filewrite
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"write-message\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                writemsg
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"yank\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: Some(
                yank
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        },
        name_bind {
            n_name: b"\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            n_func: None,
        },
    ]
};
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
