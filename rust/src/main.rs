pub fn main() {
    let mut args: Vec<*mut std::ffi::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as gint,
            args.as_mut_ptr() as *mut *mut gchar,
        ) as i32)
    }
}
unsafe fn main_0(mut argc: gint, mut argv: *mut *mut gchar) -> gint {
    let mut nonblock: *mut gboolean = 0 as *mut gboolean;
    globalconf.starttime = l_time();
    log_init();
    gtk_disable_setlocale();
    setlocale(
        6 as std::ffi::c_int,
        b"\0" as *const u8 as *const std::ffi::c_char,
    );
    setlocale(
        1 as std::ffi::c_int,
        b"C\0" as *const u8 as *const std::ffi::c_char,
    );
    let mut uris: *mut *mut gchar = parseopts(&mut argc, argv, &mut nonblock);
    let mut i: gint = 1 as std::ffi::c_int;
    while i < argc {
        memset(
            *argv.offset(i as isize) as *mut std::ffi::c_void,
            0 as std::ffi::c_int,
            strlen(*argv.offset(i as isize)),
        );
        i += 1;
        i;
    }
    globalconf.windows = g_ptr_array_new();
    if !nonblock.is_null() {
        let mut pid: pid_t = fork();
        if pid < 0 as std::ffi::c_int {
            _log(
                LOG_LEVEL_fatal,
                b"luakit.c\0" as *const u8 as *const std::ffi::c_char,
                b"Cannot fork: %d\0" as *const u8 as *const std::ffi::c_char,
                *__errno_location(),
            );
        } else if pid > 0 as std::ffi::c_int {
            exit(0 as std::ffi::c_int);
        }
        let mut sid: pid_t = setsid();
        if sid < 0 as std::ffi::c_int {
            _log(
                LOG_LEVEL_fatal,
                b"luakit.c\0" as *const u8 as *const std::ffi::c_char,
                b"New SID creation failure: %d\0" as *const u8 as *const std::ffi::c_char,
                *__errno_location(),
            );
        }
    }
    gtk_init(&mut argc, &mut argv);
    g_log_set_writer_func(Some(glib_log_writer), 0 as *mut std::ffi::c_void, None);
    init_directories();
    web_context_init();
    ipc_init();
    luaH_init(uris);
    if luaH_parserc(
        globalconf.confpath,
        (0 as std::ffi::c_int == 0) as std::ffi::c_int,
    ) == 0
    {
        _log(
            LOG_LEVEL_fatal,
            b"luakit.c\0" as *const u8 as *const std::ffi::c_char,
            b"couldn't find rc file\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if (*globalconf.windows).len == 0 {
        _log(
            LOG_LEVEL_fatal,
            b"luakit.c\0" as *const u8 as *const std::ffi::c_char,
            b"no windows spawned by rc file, exiting\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    todo!("gtk_main()");
    return 0 as std::ffi::c_int;
}
