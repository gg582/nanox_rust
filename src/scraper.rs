extern "C" {
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
        __arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> ::core::ffi::c_int;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(
        __fd: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_void,
        __nbytes: size_t,
    ) -> ssize_t;
    fn pipe(__pipedes: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn dup2(__fd: ::core::ffi::c_int, __fd2: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn execvp(
        __file: *const ::core::ffi::c_char,
        __argv: *const *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn _exit(__status: ::core::ffi::c_int) -> !;
    fn fork() -> __pid_t;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut ::core::ffi::c_int,
        __options: ::core::ffi::c_int,
    ) -> __pid_t;
}
pub type size_t = usize;
pub type scraper_lang_t = ::core::ffi::c_uint;
pub const SCRAPER_LANG_COUNT: scraper_lang_t = 2;
pub const SCRAPER_LANG_NODE: scraper_lang_t = 1;
pub const SCRAPER_LANG_PYTHON: scraper_lang_t = 0;
pub type scraper_symbol_cb = Option<
    unsafe extern "C" fn(*const ::core::ffi::c_char, *mut ::core::ffi::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: ::core::ffi::c_int,
    pub __count: ::core::ffi::c_uint,
    pub __owner: ::core::ffi::c_int,
    pub __nusers: ::core::ffi::c_uint,
    pub __kind: ::core::ffi::c_int,
    pub __spins: ::core::ffi::c_short,
    pub __elision: ::core::ffi::c_short,
    pub __list: __pthread_list_t,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct symbol_list_t {
    pub items: *mut *mut ::core::ffi::c_char,
    pub count: ::core::ffi::c_int,
    pub capacity: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct runtime_entry_t {
    pub module: *mut ::core::ffi::c_char,
    pub symbols: *mut *mut ::core::ffi::c_char,
    pub count: ::core::ffi::c_int,
    pub capacity: ::core::ffi::c_int,
    pub ready: ::core::ffi::c_int,
    pub in_progress: ::core::ffi::c_int,
    pub failed: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scraper_job_t {
    pub lang: scraper_lang_t,
    pub entry: *mut runtime_entry_t,
}
pub type __pid_t = ::core::ffi::c_int;
pub type pid_t = __pid_t;
pub type ssize_t = isize;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [::core::ffi::c_char; 48],
    pub __align: ::core::ffi::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __glibc_unused___g_refs: [::core::ffi::c_uint; 2],
    pub __g_size: [::core::ffi::c_uint; 2],
    pub __g1_orig_size: ::core::ffi::c_uint,
    pub __wrefs: ::core::ffi::c_uint,
    pub __g_signals: [::core::ffi::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __atomic_wide_counter {
    pub __value64: ::core::ffi::c_ulonglong,
    pub __value32: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub __low: ::core::ffi::c_uint,
    pub __high: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [::core::ffi::c_char; 56],
    pub __align: ::core::ffi::c_long,
}
pub type pthread_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct runtime_cache_t {
    pub entries: *mut *mut runtime_entry_t,
    pub count: ::core::ffi::c_int,
    pub capacity: ::core::ffi::c_int,
}
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const PTHREAD_MUTEX_FAST_NP: C2RustUnnamed_0 = 0;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed_0 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed_0 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed_0 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed_0 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed_0 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed_0 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed_0 = 1;
pub const STDOUT_FILENO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const STDERR_FILENO: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const SCRAPER_MAX_SYMBOLS: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const SCRAPER_MAX_ENTRIES: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
static mut caches: [runtime_cache_t; 2] = [runtime_cache_t {
    entries: ::core::ptr::null::<*mut runtime_entry_t>() as *mut *mut runtime_entry_t,
    count: 0,
    capacity: 0,
}; 2];
static mut job_queue: *mut scraper_job_t = ::core::ptr::null::<scraper_job_t>()
    as *mut scraper_job_t;
static mut job_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut job_capacity: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut scraper_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0 as ::core::ffi::c_int,
        __count: 0 as ::core::ffi::c_uint,
        __owner: 0 as ::core::ffi::c_int,
        __nusers: 0 as ::core::ffi::c_uint,
        __kind: PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
        __spins: 0 as ::core::ffi::c_short,
        __elision: 0 as ::core::ffi::c_short,
        __list: __pthread_internal_list {
            __prev: ::core::ptr::null::<__pthread_internal_list>()
                as *mut __pthread_internal_list,
            __next: ::core::ptr::null::<__pthread_internal_list>()
                as *mut __pthread_internal_list,
        },
    },
};
static mut job_cond: pthread_cond_t = pthread_cond_t {
    __data: __pthread_cond_s {
        __wseq: __atomic_wide_counter {
            __value64: 0 as ::core::ffi::c_int as ::core::ffi::c_ulonglong,
        },
        __g1_start: __atomic_wide_counter {
            __value64: 0 as ::core::ffi::c_int as ::core::ffi::c_ulonglong,
        },
        __glibc_unused___g_refs: [
            0 as ::core::ffi::c_int as ::core::ffi::c_uint,
            0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        ],
        __g_size: [
            0 as ::core::ffi::c_int as ::core::ffi::c_uint,
            0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        ],
        __g1_orig_size: 0 as ::core::ffi::c_uint,
        __wrefs: 0 as ::core::ffi::c_uint,
        __g_signals: [
            0 as ::core::ffi::c_int as ::core::ffi::c_uint,
            0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        ],
    },
};
static mut worker_thread: pthread_t = 0;
static mut worker_started: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut python_script: [::core::ffi::c_char; 149] = unsafe {
    ::core::mem::transmute::<
        [u8; 149],
        [::core::ffi::c_char; 149],
    >(
        *b"import importlib,sys\nmod=sys.argv[1]\ntry:\n    m=importlib.import_module(mod)\n    for name in dir(m):\n        print(name)\nexcept Exception:\n    pass\n\0",
    )
};
static mut node_script: [::core::ffi::c_char; 276] = unsafe {
    ::core::mem::transmute::<
        [u8; 276],
        [::core::ffi::c_char; 276],
    >(
        *b"const mod=process.argv[1];\ntry {\n  const m=require(mod);\n  const seen=new Set();\n  for (const k in m) {\n    if (!seen.has(k)) {\n      console.log(k);\n      seen.add(k);\n    }\n  }\n  if (m && typeof m === 'object' && m.default) {\n    console.log('default');\n  }\n} catch (e) {}\n\0",
    )
};
unsafe extern "C" fn free_entry(mut entry: *mut runtime_entry_t) {
    if entry.is_null() {
        return;
    }
    free((*entry).module as *mut ::core::ffi::c_void);
    (*entry).module = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*entry).count {
        free(*(*entry).symbols.offset(i as isize) as *mut ::core::ffi::c_void);
        i += 1;
    }
    free((*entry).symbols as *mut ::core::ffi::c_void);
    (*entry).symbols = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    (*entry).count = 0 as ::core::ffi::c_int;
    (*entry).capacity = 0 as ::core::ffi::c_int;
    (*entry).ready = 0 as ::core::ffi::c_int;
    (*entry).in_progress = 0 as ::core::ffi::c_int;
    (*entry).failed = 0 as ::core::ffi::c_int;
    free(entry as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn find_entry(
    mut lang: scraper_lang_t,
    mut module: *const ::core::ffi::c_char,
) -> *mut runtime_entry_t {
    let mut cache: *mut runtime_cache_t = (&raw mut caches as *mut runtime_cache_t)
        .offset(lang as isize) as *mut runtime_cache_t;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*cache).count {
        if strcmp((**(*cache).entries.offset(i as isize)).module, module)
            == 0 as ::core::ffi::c_int
        {
            return *(*cache).entries.offset(i as isize);
        }
        i += 1;
    }
    return ::core::ptr::null_mut::<runtime_entry_t>();
}
unsafe extern "C" fn ensure_cache_capacity(
    mut cache: *mut runtime_cache_t,
    mut needed: ::core::ffi::c_int,
) {
    if (*cache).capacity >= needed {
        return;
    }
    let mut new_capacity: ::core::ffi::c_int = if (*cache).capacity != 0 {
        (*cache).capacity * 2 as ::core::ffi::c_int
    } else {
        8 as ::core::ffi::c_int
    };
    if new_capacity < needed {
        new_capacity = needed;
    }
    let mut tmp: *mut *mut runtime_entry_t = realloc(
        (*cache).entries as *mut ::core::ffi::c_void,
        (new_capacity as size_t)
            .wrapping_mul(::core::mem::size_of::<*mut runtime_entry_t>() as size_t),
    ) as *mut *mut runtime_entry_t;
    if tmp.is_null() {
        return;
    }
    (*cache).entries = tmp;
    (*cache).capacity = new_capacity;
}
unsafe extern "C" fn create_entry(
    mut lang: scraper_lang_t,
    mut module: *const ::core::ffi::c_char,
) -> *mut runtime_entry_t {
    let mut cache: *mut runtime_cache_t = (&raw mut caches as *mut runtime_cache_t)
        .offset(lang as isize) as *mut runtime_cache_t;
    if (*cache).count >= SCRAPER_MAX_ENTRIES {
        let mut removed: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*cache).count {
            let mut candidate: *mut runtime_entry_t = *(*cache)
                .entries
                .offset(i as isize);
            if !candidate.is_null() && (*candidate).in_progress == 0 {
                free_entry(candidate);
                if i < (*cache).count - 1 as ::core::ffi::c_int {
                    memmove(
                        (*cache).entries.offset(i as isize) as *mut *mut runtime_entry_t
                            as *mut ::core::ffi::c_void,
                        (*cache).entries.offset((i + 1 as ::core::ffi::c_int) as isize)
                            as *mut *mut runtime_entry_t as *const ::core::ffi::c_void,
                        (((*cache).count - i - 1 as ::core::ffi::c_int) as size_t)
                            .wrapping_mul(
                                ::core::mem::size_of::<*mut runtime_entry_t>() as size_t,
                            ),
                    );
                }
                (*cache).count -= 1;
                removed = 0 as ::core::ffi::c_int;
                break;
            } else {
                i += 1;
            }
        }
        if removed != 0 as ::core::ffi::c_int {
            return ::core::ptr::null_mut::<runtime_entry_t>();
        }
    }
    ensure_cache_capacity(cache, (*cache).count + 1 as ::core::ffi::c_int);
    if (*cache).capacity < (*cache).count + 1 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<runtime_entry_t>();
    }
    let mut entry: *mut runtime_entry_t = calloc(
        1 as size_t,
        ::core::mem::size_of::<runtime_entry_t>() as size_t,
    ) as *mut runtime_entry_t;
    if entry.is_null() {
        return ::core::ptr::null_mut::<runtime_entry_t>();
    }
    (*entry).module = strdup(module);
    if (*entry).module.is_null() {
        free(entry as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<runtime_entry_t>();
    }
    let fresh4 = (*cache).count;
    (*cache).count = (*cache).count + 1;
    let ref mut fresh5 = *(*cache).entries.offset(fresh4 as isize);
    *fresh5 = entry;
    return entry;
}
unsafe extern "C" fn enqueue_job(
    mut lang: scraper_lang_t,
    mut entry: *mut runtime_entry_t,
) {
    if entry.is_null() {
        return;
    }
    if job_count == job_capacity {
        let mut new_capacity: ::core::ffi::c_int = if job_capacity != 0 {
            job_capacity * 2 as ::core::ffi::c_int
        } else {
            8 as ::core::ffi::c_int
        };
        let mut tmp: *mut scraper_job_t = realloc(
            job_queue as *mut ::core::ffi::c_void,
            (new_capacity as size_t)
                .wrapping_mul(::core::mem::size_of::<scraper_job_t>() as size_t),
        ) as *mut scraper_job_t;
        if tmp.is_null() {
            return;
        }
        job_queue = tmp;
        job_capacity = new_capacity;
    }
    (*job_queue.offset(job_count as isize)).lang = lang;
    let ref mut fresh3 = (*job_queue.offset(job_count as isize)).entry;
    *fresh3 = entry;
    job_count += 1;
    pthread_cond_signal(&raw mut job_cond);
}
unsafe extern "C" fn dequeue_job(mut out: *mut scraper_job_t) -> ::core::ffi::c_int {
    if job_count == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    *out = *job_queue.offset(0 as ::core::ffi::c_int as isize);
    if job_count > 1 as ::core::ffi::c_int {
        memmove(
            job_queue as *mut ::core::ffi::c_void,
            job_queue.offset(1 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_void,
            ((job_count - 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<scraper_job_t>() as size_t),
        );
    }
    job_count -= 1;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn run_child_process(
    mut prog: *const ::core::ffi::c_char,
    mut argv: *const *mut ::core::ffi::c_char,
    mut output: *mut ::core::ffi::c_char,
    mut outsz: size_t,
) -> ::core::ffi::c_int {
    let mut pipefd: [::core::ffi::c_int; 2] = [0; 2];
    if output.is_null() || outsz == 0 as size_t {
        return -(1 as ::core::ffi::c_int);
    }
    *output.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32
        as ::core::ffi::c_char;
    if pipe(&raw mut pipefd as *mut ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    let mut pid: pid_t = fork() as pid_t;
    if pid < 0 as ::core::ffi::c_int {
        close(pipefd[0 as ::core::ffi::c_int as usize]);
        close(pipefd[1 as ::core::ffi::c_int as usize]);
        return -(1 as ::core::ffi::c_int);
    }
    if pid == 0 as ::core::ffi::c_int {
        let mut devnull: ::core::ffi::c_int = open(
            b"/dev/null\0" as *const u8 as *const ::core::ffi::c_char,
            O_WRONLY,
        );
        if devnull >= 0 as ::core::ffi::c_int {
            dup2(devnull, STDERR_FILENO);
            close(devnull);
        }
        dup2(pipefd[1 as ::core::ffi::c_int as usize], STDOUT_FILENO);
        close(pipefd[0 as ::core::ffi::c_int as usize]);
        close(pipefd[1 as ::core::ffi::c_int as usize]);
        execvp(prog, argv);
        _exit(127 as ::core::ffi::c_int);
    }
    close(pipefd[1 as ::core::ffi::c_int as usize]);
    let mut total: size_t = 0 as size_t;
    let mut nr: ssize_t = 0;
    while total < outsz.wrapping_sub(1 as size_t)
        && {
            nr = read(
                pipefd[0 as ::core::ffi::c_int as usize],
                output.offset(total as isize) as *mut ::core::ffi::c_void,
                outsz.wrapping_sub(1 as size_t).wrapping_sub(total),
            );
            nr > 0 as ssize_t
        }
    {
        total = total.wrapping_add(nr as size_t);
    }
    *output.offset(total as isize) = '\0' as i32 as ::core::ffi::c_char;
    close(pipefd[0 as ::core::ffi::c_int as usize]);
    let mut status: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    waitpid(pid as __pid_t, &raw mut status, 0 as ::core::ffi::c_int);
    if !(status & 0x7f as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
        || (status & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    return total as ::core::ffi::c_int;
}
unsafe extern "C" fn run_language_command(
    mut lang: scraper_lang_t,
    mut module: *const ::core::ffi::c_char,
    mut buffer: *mut ::core::ffi::c_char,
    mut bufsz: size_t,
) -> ::core::ffi::c_int {
    if module.is_null() || *module == 0 {
        return -(1 as ::core::ffi::c_int);
    }
    if lang as ::core::ffi::c_uint
        == SCRAPER_LANG_PYTHON as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let argv: [*mut ::core::ffi::c_char; 5] = [
            b"python3\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"-c\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            &raw const python_script as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            module as *mut ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ];
        return run_child_process(
            b"python3\0" as *const u8 as *const ::core::ffi::c_char,
            &raw const argv as *const *mut ::core::ffi::c_char,
            buffer,
            bufsz,
        );
    }
    if lang as ::core::ffi::c_uint
        == SCRAPER_LANG_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let argv_0: [*mut ::core::ffi::c_char; 5] = [
            b"node\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"-e\0" as *const u8 as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            &raw const node_script as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            module as *mut ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ];
        return run_child_process(
            b"node\0" as *const u8 as *const ::core::ffi::c_char,
            &raw const argv_0 as *const *mut ::core::ffi::c_char,
            buffer,
            bufsz,
        );
    }
    return -(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn symbol_list_add(
    mut list: *mut symbol_list_t,
    mut symbol: *const ::core::ffi::c_char,
) {
    if symbol.is_null() || *symbol == 0 {
        return;
    }
    if (*list).count >= SCRAPER_MAX_SYMBOLS {
        return;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*list).count {
        if strcmp(*(*list).items.offset(i as isize), symbol) == 0 as ::core::ffi::c_int {
            return;
        }
        i += 1;
    }
    if (*list).count == (*list).capacity {
        let mut new_capacity: ::core::ffi::c_int = if (*list).capacity != 0 {
            (*list).capacity * 2 as ::core::ffi::c_int
        } else {
            32 as ::core::ffi::c_int
        };
        let mut tmp: *mut *mut ::core::ffi::c_char = realloc(
            (*list).items as *mut ::core::ffi::c_void,
            (new_capacity as size_t)
                .wrapping_mul(
                    ::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t,
                ),
        ) as *mut *mut ::core::ffi::c_char;
        if tmp.is_null() {
            return;
        }
        (*list).items = tmp;
        (*list).capacity = new_capacity;
    }
    let fresh1 = (*list).count;
    (*list).count = (*list).count + 1;
    let ref mut fresh2 = *(*list).items.offset(fresh1 as isize);
    *fresh2 = strdup(symbol);
}
unsafe extern "C" fn symbol_list_free(mut list: *mut symbol_list_t) {
    if list.is_null() {
        return;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*list).count {
        free(*(*list).items.offset(i as isize) as *mut ::core::ffi::c_void);
        i += 1;
    }
    free((*list).items as *mut ::core::ffi::c_void);
    (*list).items = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    (*list).count = 0 as ::core::ffi::c_int;
    (*list).capacity = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn collect_symbols_from_buffer(
    mut buffer: *mut ::core::ffi::c_char,
    mut list: *mut symbol_list_t,
) {
    let mut line: *mut ::core::ffi::c_char = buffer;
    while !line.is_null() && *line as ::core::ffi::c_int != 0 {
        let mut next: *mut ::core::ffi::c_char = strchr(line, '\n' as i32);
        if !next.is_null() {
            let fresh0 = next;
            next = next.offset(1);
            *fresh0 = '\0' as i32 as ::core::ffi::c_char;
        }
        while *line as ::core::ffi::c_int == ' ' as i32
            || *line as ::core::ffi::c_int == '\t' as i32
            || *line as ::core::ffi::c_int == '\r' as i32
        {
            line = line.offset(1);
        }
        let mut len: size_t = strlen(line);
        while len > 0 as size_t
            && (*line.offset(len.wrapping_sub(1 as size_t) as isize)
                as ::core::ffi::c_int == ' ' as i32
                || *line.offset(len.wrapping_sub(1 as size_t) as isize)
                    as ::core::ffi::c_int == '\t' as i32
                || *line.offset(len.wrapping_sub(1 as size_t) as isize)
                    as ::core::ffi::c_int == '\r' as i32)
        {
            len = len.wrapping_sub(1);
            *line.offset(len as isize) = '\0' as i32 as ::core::ffi::c_char;
        }
        if len > 0 as size_t {
            symbol_list_add(list, line);
        }
        line = next;
    }
}
unsafe extern "C" fn update_entry_symbols(
    mut entry: *mut runtime_entry_t,
    mut list: *mut symbol_list_t,
    mut success: ::core::ffi::c_int,
) {
    if entry.is_null() {
        return;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*entry).count {
        free(*(*entry).symbols.offset(i as isize) as *mut ::core::ffi::c_void);
        i += 1;
    }
    free((*entry).symbols as *mut ::core::ffi::c_void);
    (*entry).symbols = (*list).items;
    (*entry).count = (*list).count;
    (*entry).capacity = (*list).capacity;
    (*list).items = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    (*list).count = 0 as ::core::ffi::c_int;
    (*list).capacity = 0 as ::core::ffi::c_int;
    (*entry).ready = 1 as ::core::ffi::c_int;
    (*entry).failed = if success != 0 {
        0 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
    (*entry).in_progress = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn scraper_worker(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    loop {
        let mut job: scraper_job_t = scraper_job_t {
            lang: SCRAPER_LANG_PYTHON,
            entry: ::core::ptr::null_mut::<runtime_entry_t>(),
        };
        pthread_mutex_lock(&raw mut scraper_mutex);
        while job_count == 0 as ::core::ffi::c_int {
            pthread_cond_wait(&raw mut job_cond, &raw mut scraper_mutex);
        }
        if dequeue_job(&raw mut job) == 0 {
            pthread_mutex_unlock(&raw mut scraper_mutex);
        } else {
            pthread_mutex_unlock(&raw mut scraper_mutex);
            let mut buffer: [::core::ffi::c_char; 16384] = [0; 16384];
            let mut rc: ::core::ffi::c_int = run_language_command(
                job.lang,
                (*job.entry).module,
                &raw mut buffer as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 16384]>() as size_t,
            );
            let mut list: symbol_list_t = symbol_list_t {
                items: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                count: 0,
                capacity: 0,
            };
            if rc > 0 as ::core::ffi::c_int {
                collect_symbols_from_buffer(
                    &raw mut buffer as *mut ::core::ffi::c_char,
                    &raw mut list,
                );
            }
            pthread_mutex_lock(&raw mut scraper_mutex);
            update_entry_symbols(
                job.entry,
                &raw mut list,
                (rc >= 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
            );
            pthread_mutex_unlock(&raw mut scraper_mutex);
            symbol_list_free(&raw mut list);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn scraper_init() {
    pthread_mutex_lock(&raw mut scraper_mutex);
    if worker_started == 0 {
        if pthread_create(
            &raw mut worker_thread,
            ::core::ptr::null::<pthread_attr_t>(),
            Some(
                scraper_worker
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                    ) -> *mut ::core::ffi::c_void,
            ),
            NULL,
        ) == 0 as ::core::ffi::c_int
        {
            worker_started = 1 as ::core::ffi::c_int;
        }
    }
    pthread_mutex_unlock(&raw mut scraper_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn scraper_iterate_symbols(
    mut lang: scraper_lang_t,
    mut module: *const ::core::ffi::c_char,
    mut cb: scraper_symbol_cb,
    mut userdata: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if module.is_null() || *module == 0
        || (lang as ::core::ffi::c_uint) < 0 as ::core::ffi::c_uint
        || lang as ::core::ffi::c_uint
            >= SCRAPER_LANG_COUNT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int;
    }
    if worker_started == 0 {
        scraper_init();
    }
    let mut entry: *mut runtime_entry_t = ::core::ptr::null_mut::<runtime_entry_t>();
    pthread_mutex_lock(&raw mut scraper_mutex);
    entry = find_entry(lang, module);
    if entry.is_null() {
        entry = create_entry(lang, module);
        if !entry.is_null() {
            (*entry).in_progress = 1 as ::core::ffi::c_int;
        }
        enqueue_job(lang, entry);
    } else if (*entry).ready == 0 && (*entry).in_progress == 0 {
        (*entry).in_progress = 1 as ::core::ffi::c_int;
        enqueue_job(lang, entry);
    }
    let mut ready: ::core::ffi::c_int = (!entry.is_null() && (*entry).ready != 0)
        as ::core::ffi::c_int;
    let mut items: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        *mut ::core::ffi::c_char,
    >();
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if ready != 0 && (*entry).count > 0 as ::core::ffi::c_int {
        items = (*entry).symbols;
        count = (*entry).count;
    }
    pthread_mutex_unlock(&raw mut scraper_mutex);
    if ready != 0 && cb.is_some() && !items.is_null() {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < count {
            cb.expect("non-null function pointer")(*items.offset(i as isize), userdata);
            i += 1;
        }
    }
    return ready;
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
