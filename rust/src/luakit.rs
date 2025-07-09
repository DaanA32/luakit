#![feature(extern_types)]
#![allow(internal_features)]
#![feature(core_intrinsics)]
#![feature(c_variadic)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(path_statements)]
#![allow(static_mut_refs)]
#![allow(mutable_transmutes)]
#![allow(unreachable_patterns)]
#![allow(unreachable_code)]
#![allow(unused_parens)]
#![allow(unused_unsafe)]

use gtk_sys::{gtk_disable_setlocale, gtk_get_option_group, gtk_init};

pub mod clib;
pub mod common;
pub mod extension;
pub mod globalconf;
pub mod gtypes;
pub mod ipc;
pub mod log;
pub mod luah;
pub mod web_context;
pub mod widgets;

use glib_sys::{
    G_LOG_LEVEL_MASK, G_LOG_WRITER_HANDLED, G_LOG_WRITER_UNHANDLED, G_OPTION_ARG_NONE,
    G_OPTION_ARG_STRING, G_OPTION_ARG_STRING_ARRAY, GError, GLogField, GLogLevelFlags,
    GLogWriterOutput, GOptionContext, GOptionEntry, g_build_filename, g_fprintf, g_free,
    g_get_user_cache_dir, g_get_user_config_dir, g_get_user_data_dir, g_log_set_writer_func,
    g_mkdir_with_parents, g_option_context_add_group, g_option_context_add_main_entries,
    g_option_context_free, g_option_context_new, g_option_context_parse, g_ptr_array_add,
    g_ptr_array_new, g_ptr_array_new_with_free_func, g_ptr_array_remove_index, g_strdup, g_strdupv,
    g_strfreev, g_strsplit, gboolean, gpointer,
};
use globalconf::globalconf;
use libc::{
    __errno_location, c_void, exit, fork, memset, pid_t, setlocale, setsid, strchr, strcmp, strlen,
};
use mlua_sys::lua_State;
use webkit2gtk::ffi::{
    webkit_get_major_version, webkit_get_micro_version, webkit_get_minor_version,
};

use crate::{
    common::clib::luakit::l_time,
    gtypes::{gchar, gint, guint},
    ipc::*,
    log::{
        _log, LOG_LEVEL_debug, LOG_LEVEL_fatal, LOG_LEVEL_info, LOG_LEVEL_verbose, LOG_LEVEL_warn,
        log_init, log_level_from_string, log_level_t, log_set_verbosity,
    },
    luah::{luaH_init, luaH_parserc},
    web_context::*,
};

unsafe extern "C" fn init_directories() {
    globalconf.cache_dir = g_build_filename(
        g_get_user_cache_dir(),
        b"luakit\0" as *const u8 as *const std::ffi::c_char,
        globalconf.profile,
        0 as *mut std::ffi::c_void,
    );
    globalconf.config_dir = g_build_filename(
        g_get_user_config_dir(),
        b"luakit\0" as *const u8 as *const std::ffi::c_char,
        globalconf.profile,
        0 as *mut std::ffi::c_void,
    );
    globalconf.data_dir = g_build_filename(
        g_get_user_data_dir(),
        b"luakit\0" as *const u8 as *const std::ffi::c_char,
        globalconf.profile,
        0 as *mut std::ffi::c_void,
    );
    g_mkdir_with_parents(globalconf.cache_dir, 0o700 as std::ffi::c_int);
    g_mkdir_with_parents(globalconf.config_dir, 0o700 as std::ffi::c_int);
    g_mkdir_with_parents(globalconf.data_dir, 0o700 as std::ffi::c_int);
}
unsafe extern "C" fn parse_log_level_option(mut log_lvl: *mut gchar) {
    let mut parts: *mut *mut gchar = g_strsplit(
        log_lvl,
        b",\0" as *const u8 as *const std::ffi::c_char,
        0 as std::ffi::c_int,
    );
    let mut part: *mut *mut gchar = parts;
    while !(*part).is_null() {
        let mut lvl: log_level_t = LOG_LEVEL_fatal;
        if log_level_from_string(&mut lvl, *part) == 0 {
            log_set_verbosity(b"all\0" as *const u8 as *const std::ffi::c_char, lvl);
        } else {
            let mut sep: *mut gchar = strchr(*part, '=' as i32);
            if !sep.is_null()
                && log_level_from_string(&mut lvl, sep.offset(1 as std::ffi::c_int as isize)) == 0
            {
                *sep = '\0' as i32 as gchar;
                log_set_verbosity(*part, lvl);
            } else {
                _log(
                    LOG_LEVEL_warn,
                    b"luakit.c\0" as *const u8 as *const std::ffi::c_char,
                    b"ignoring unrecognized --log option '%s'\0" as *const u8
                        as *const std::ffi::c_char,
                    *part,
                );
            }
        }
        part = part.offset(1);
        part;
    }
    g_strfreev(parts);
}
unsafe extern "C" fn parseopts(
    mut argc: *mut std::ffi::c_int,
    mut argv: *mut *mut gchar,
    mut nonblock: *mut *mut gboolean,
) -> *mut *mut gchar {
    let mut context: *mut GOptionContext = 0 as *mut GOptionContext;
    let mut version_only: *mut gboolean = 0 as *mut gboolean;
    let mut check_only: *mut gboolean = 0 as *mut gboolean;
    let mut uris: *mut *mut gchar = 0 as *mut *mut gchar;
    globalconf.profile = 0 as *mut gchar;
    let mut verbose: gboolean = 0 as std::ffi::c_int;
    let mut log_lvl: *mut gchar = 0 as *mut gchar;
    globalconf.execpath = g_strdup(*argv.offset(0 as std::ffi::c_int as isize));
    globalconf.nounique = 0 as std::ffi::c_int;
    let entries: [GOptionEntry; 10] = [
        {
            let mut init = GOptionEntry {
                long_name: b"check\0" as *const u8 as *const std::ffi::c_char,
                short_name: 'k' as i32 as gchar,
                flags: 0 as std::ffi::c_int,
                arg: G_OPTION_ARG_NONE,
                arg_data: &mut check_only as *mut *mut gboolean as gpointer,
                description: b"check config and exit\0" as *const u8 as *const std::ffi::c_char,
                arg_description: 0 as *const gchar,
            };
            init
        },
        {
            let mut init = GOptionEntry {
                long_name: b"config\0" as *const u8 as *const std::ffi::c_char,
                short_name: 'c' as i32 as gchar,
                flags: 0 as std::ffi::c_int,
                arg: G_OPTION_ARG_STRING,
                arg_data: &mut globalconf.confpath as *mut *mut gchar as gpointer,
                description: b"configuration file to use\0" as *const u8 as *const std::ffi::c_char,
                arg_description: b"FILE\0" as *const u8 as *const std::ffi::c_char,
            };
            init
        },
        {
            let mut init = GOptionEntry {
                long_name: b"profile\0" as *const u8 as *const std::ffi::c_char,
                short_name: 'p' as i32 as gchar,
                flags: 0 as std::ffi::c_int,
                arg: G_OPTION_ARG_STRING,
                arg_data: &mut globalconf.profile as *mut *mut gchar as gpointer,
                description: b"profile name to use\0" as *const u8 as *const std::ffi::c_char,
                arg_description: b"NAME\0" as *const u8 as *const std::ffi::c_char,
            };
            init
        },
        {
            let mut init = GOptionEntry {
                long_name: b"nonblock\0" as *const u8 as *const std::ffi::c_char,
                short_name: 'n' as i32 as gchar,
                flags: 0 as std::ffi::c_int,
                arg: G_OPTION_ARG_NONE,
                arg_data: nonblock as gpointer,
                description: b"run in background\0" as *const u8 as *const std::ffi::c_char,
                arg_description: 0 as *const gchar,
            };
            init
        },
        {
            let mut init = GOptionEntry {
                long_name: b"nounique\0" as *const u8 as *const std::ffi::c_char,
                short_name: 'U' as i32 as gchar,
                flags: 0 as std::ffi::c_int,
                arg: G_OPTION_ARG_NONE,
                arg_data: &mut globalconf.nounique as *mut gboolean as gpointer,
                description: b"ignore libunique bindings\0" as *const u8 as *const std::ffi::c_char,
                arg_description: 0 as *const gchar,
            };
            init
        },
        {
            let mut init = GOptionEntry {
                long_name: b"uri\0" as *const u8 as *const std::ffi::c_char,
                short_name: 'u' as i32 as gchar,
                flags: 0 as std::ffi::c_int,
                arg: G_OPTION_ARG_STRING_ARRAY,
                arg_data: &mut uris as *mut *mut *mut gchar as gpointer,
                description: b"uri(s) to load at startup\0" as *const u8 as *const std::ffi::c_char,
                arg_description: b"URI\0" as *const u8 as *const std::ffi::c_char,
            };
            init
        },
        {
            let mut init = GOptionEntry {
                long_name: b"verbose\0" as *const u8 as *const std::ffi::c_char,
                short_name: 'v' as i32 as gchar,
                flags: 0 as std::ffi::c_int,
                arg: G_OPTION_ARG_NONE,
                arg_data: &mut verbose as *mut gboolean as gpointer,
                description: b"print verbose output\0" as *const u8 as *const std::ffi::c_char,
                arg_description: 0 as *const gchar,
            };
            init
        },
        {
            let mut init = GOptionEntry {
                long_name: b"log\0" as *const u8 as *const std::ffi::c_char,
                short_name: 'l' as i32 as gchar,
                flags: 0 as std::ffi::c_int,
                arg: G_OPTION_ARG_STRING,
                arg_data: &mut log_lvl as *mut *mut gchar as gpointer,
                description: b"specify precise log level\0" as *const u8 as *const std::ffi::c_char,
                arg_description: b"NAME\0" as *const u8 as *const std::ffi::c_char,
            };
            init
        },
        {
            let mut init = GOptionEntry {
                long_name: b"version\0" as *const u8 as *const std::ffi::c_char,
                short_name: 'V' as i32 as gchar,
                flags: 0 as std::ffi::c_int,
                arg: G_OPTION_ARG_NONE,
                arg_data: &mut version_only as *mut *mut gboolean as gpointer,
                description: b"print version and exit\0" as *const u8 as *const std::ffi::c_char,
                arg_description: 0 as *const gchar,
            };
            init
        },
        {
            let mut init = GOptionEntry {
                long_name: 0 as *const gchar,
                short_name: 0 as std::ffi::c_int as gchar,
                flags: 0 as std::ffi::c_int,
                arg: G_OPTION_ARG_NONE,
                arg_data: 0 as *mut std::ffi::c_void,
                description: 0 as *const gchar,
                arg_description: 0 as *const gchar,
            };
            init
        },
    ];
    globalconf.argv =
        g_ptr_array_new_with_free_func(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    let mut i: gint = 0 as std::ffi::c_int;
    while i < *argc {
        g_ptr_array_add(
            globalconf.argv,
            g_strdup(*argv.offset(i as isize)) as gpointer,
        );
        i += 1;
        i;
    }
    context = g_option_context_new(b"[URI...]\0" as *const u8 as *const std::ffi::c_char);
    g_option_context_add_main_entries(context, entries.as_ptr(), 0 as *const gchar);
    g_option_context_add_group(context, gtk_get_option_group(0 as std::ffi::c_int));
    g_option_context_parse(context, argc, &mut argv, 0 as *mut *mut GError);
    g_option_context_free(context);
    let mut i_0: gint = 0 as std::ffi::c_int;
    while i_0 < *argc {
        while (i_0 as std::ffi::c_uint) < (*globalconf.argv).len
            && strcmp(
                *((*globalconf.argv).pdata).offset(i_0 as isize) as *const std::ffi::c_char,
                *argv.offset(i_0 as isize),
            ) == 0
        {
            g_ptr_array_remove_index(globalconf.argv, i_0 as guint);
        }
        i_0 += 1;
        i_0;
    }
    if !version_only.is_null() {
        println!("luakit {}\n\0", "64175ca2",);
        println!("  built with: webkit {}.{}.{}", 2, 48, 3,);
        println!(
            "(installed version: {}.{}.{})",
            webkit_get_major_version(),
            webkit_get_minor_version(),
            webkit_get_micro_version(),
        );
        println!("                 GTK {}.{}.{}", 3, 24, 49,);
        println!("                GLIB {}.{}.{}", 2, 84, 3,);
        println!("                SOUP {}.{}.{}", 3, 6, 5,);
        exit(0 as std::ffi::c_int);
    }
    if log_lvl.is_null() {
        log_set_verbosity(
            b"all\0" as *const u8 as *const std::ffi::c_char,
            (if verbose != 0 {
                LOG_LEVEL_verbose as std::ffi::c_int
            } else {
                LOG_LEVEL_info as std::ffi::c_int
            }) as log_level_t,
        );
    } else {
        log_set_verbosity(
            b"all\0" as *const u8 as *const std::ffi::c_char,
            LOG_LEVEL_info,
        );
        parse_log_level_option(log_lvl);
        if verbose != 0 {
            _log(
                LOG_LEVEL_warn,
                b"luakit.c\0" as *const u8 as *const std::ffi::c_char,
                b"invalid mix of -v and -l, ignoring -v...\0" as *const u8
                    as *const std::ffi::c_char,
            );
        }
    }
    if !check_only.is_null() {
        init_directories();
        luaH_init(0 as *mut *mut gchar);
        if luaH_parserc(globalconf.confpath, 0 as std::ffi::c_int) == 0 {
            eprintln!("Confiuration file syntax error.");
            exit(1 as std::ffi::c_int);
        } else {
            eprintln!("Configuration file syntax OK.");
            exit(0 as std::ffi::c_int);
        }
    }
    if !uris.is_null() && !(*argv.offset(1 as std::ffi::c_int as isize)).is_null() {
        _log(
            LOG_LEVEL_fatal,
            b"luakit.c\0" as *const u8 as *const std::ffi::c_char,
            b"invalid mix of -u and default uri arguments\0" as *const u8
                as *const std::ffi::c_char,
        );
    }
    if !uris.is_null() {
        return uris;
    } else {
        return g_strdupv(argv.offset(1 as std::ffi::c_int as isize));
    };
}
unsafe extern "C" fn glib_log_writer(
    mut log_level_flags: GLogLevelFlags,
    mut fields: *const GLogField,
    mut n_fields: usize,
    mut UNUSED_user_data: gpointer,
) -> GLogWriterOutput {
    let mut log_domain: *const gchar = b"(unknown)\0" as *const u8 as *const std::ffi::c_char;
    let mut message: *const gchar = b"(empty)\0" as *const u8 as *const std::ffi::c_char;
    let mut i = 0;
    while i < n_fields {
        if strcmp(
            (*fields.offset(i as isize)).key,
            b"GLIB_DOMAIN\0" as *const u8 as *const std::ffi::c_char,
        ) == 0
        {
            log_domain = (*fields.offset(i as isize)).value as *const gchar;
        }
        if strcmp(
            (*fields.offset(i as isize)).key,
            b"MESSAGE\0" as *const u8 as *const std::ffi::c_char,
        ) == 0
        {
            message = (*fields.offset(i as isize)).value as *const gchar;
        }
        i = i.wrapping_add(1);
        i;
    }
    if G_LOG_LEVEL_MASK as std::ffi::c_int & log_level_flags as std::ffi::c_int == 0 {
        return G_LOG_WRITER_UNHANDLED;
    }
    let mut log_level: log_level_t = [
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_warn,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_warn,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_info,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_verbose,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_debug,
    ][log_level_flags as usize];
    _log(
        log_level,
        b"glib\0" as *const u8 as *const std::ffi::c_char,
        b"%s: %s\0" as *const u8 as *const std::ffi::c_char,
        log_domain,
        message,
    );
    return G_LOG_WRITER_HANDLED;
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
