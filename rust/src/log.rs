use core::intrinsics::AtomicOrdering;
use std::{
    io::stderr,
    os::fd::{AsFd, AsRawFd},
};

use glib_sys::{
    G_REGEX_DEFAULT, G_REGEX_MATCH_DEFAULT, GAsyncQueue, GError, GHashTable, GPtrArray, GQuark,
    GRegex, GString, g_ascii_strtod, g_assertion_message_error, g_assertion_message_expr,
    g_async_queue_lock, g_async_queue_new, g_async_queue_push, g_async_queue_push_unlocked,
    g_async_queue_try_pop, g_async_queue_try_pop_unlocked, g_async_queue_unlock, g_build_path,
    g_error_free, g_file_get_contents, g_file_open_tmp, g_fprintf, g_free, g_hash_table_insert,
    g_hash_table_lookup, g_hash_table_new_full, g_idle_add, g_ptr_array_add,
    g_ptr_array_new_with_free_func, g_regex_new, g_regex_replace_literal, g_slice_alloc0,
    g_slice_free1, g_str_equal, g_str_has_prefix, g_str_hash, g_strdup, g_strdup_printf,
    g_string_append_c, g_string_append_len, g_string_append_printf, g_string_free, g_string_new,
    gboolean, gconstpointer, gpointer,
};
use libc::{
    c_void, close, exit, getenv, isatty, memcmp, size_t, ssize_t, strcmp, strlen, strncmp, strrchr,
    unlink, unsetenv, write,
};
use mlua_sys::{
    lua_State, lua_pushnumber, lua_pushstring, lua_settop, lua_tointeger, lua_tolstring,
};

use crate::{
    clib::msg::msg_lib_get_msg_class,
    common::{
        clib::luakit::l_time,
        common,
        luaclass::{lua_class_t, luaH_class_emit_signal},
        util::{file_exists, strip_ansi_escapes},
    },
    globalconf::globalconf,
    gtypes::{gchar, gint, gint64, glong, guint, guint8, gulong},
    ipc::ipc_endpoint_t,
};

pub type queued_log_t = _queued_log_t;

pub use self::log_h::{
    LOG_LEVEL_debug, LOG_LEVEL_error, LOG_LEVEL_fatal, LOG_LEVEL_info, LOG_LEVEL_verbose,
    LOG_LEVEL_warn, log_level_t,
};
pub mod log_h {
    pub type log_level_t = std::ffi::c_uint;
    pub const LOG_LEVEL_debug: log_level_t = 5;
    pub const LOG_LEVEL_verbose: log_level_t = 4;
    pub const LOG_LEVEL_info: log_level_t = 3;
    pub const LOG_LEVEL_warn: log_level_t = 2;
    pub const LOG_LEVEL_error: log_level_t = 1;
    pub const LOG_LEVEL_fatal: log_level_t = 0;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _queued_log_t {
    pub lvl: log_level_t,
    pub time: std::ffi::c_double,
    pub group: *mut std::ffi::c_char,
    pub msg: *mut std::ffi::c_char,
}
static mut group_levels: *mut GHashTable = 0 as *const GHashTable as *mut GHashTable;
static mut queued_emissions: *mut GAsyncQueue = 0 as *const GAsyncQueue as *mut GAsyncQueue;
static mut block_log: gboolean = 0 as std::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_set_verbosity(
    mut group: *const std::ffi::c_char,
    mut lvl: log_level_t,
) {
    group_levels = if !group_levels.is_null() {
        group_levels
    } else {
        g_hash_table_new_full(
            Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(g_str_equal as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean),
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            None,
        )
    };
    g_hash_table_insert(
        group_levels,
        g_strdup(group) as gpointer,
        (lvl as std::ffi::c_uint).wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint) as glong
            as gpointer,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_get_verbosity(mut group: *mut std::ffi::c_char) -> log_level_t {
    if group_levels.is_null() {
        return LOG_LEVEL_info;
    }
    let mut len: gint = strlen(group) as gint;
    let mut lvl: log_level_t = LOG_LEVEL_fatal;
    while 0 as std::ffi::c_int == 0 {
        lvl = g_hash_table_lookup(group_levels, group as gpointer as gconstpointer) as gulong
            as guint as log_level_t;
        if lvl as std::ffi::c_uint > 0 as std::ffi::c_int as std::ffi::c_uint {
            break;
        }
        let mut slash: *mut std::ffi::c_char = strrchr(group, '/' as i32);
        if !slash.is_null() {
            *slash = '\0' as i32 as std::ffi::c_char;
        } else {
            lvl = g_hash_table_lookup(
                group_levels,
                b"all\0" as *const u8 as *const std::ffi::c_char as gpointer as gconstpointer,
            ) as gulong as guint as log_level_t;
            break;
        }
    }
    let mut i: gint = 0 as std::ffi::c_int;
    while i < len {
        if *group.offset(i as isize) as std::ffi::c_int == '\0' as i32 {
            *group.offset(i as isize) = '/' as i32 as std::ffi::c_char;
        }
        i += 1;
        i;
    }
    return (lvl as std::ffi::c_uint).wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint)
        as log_level_t;
}
unsafe extern "C" fn log_group_from_fct(mut fct: *const std::ffi::c_char) -> *mut std::ffi::c_char {
    static mut paths: *mut GPtrArray = 0 as *const GPtrArray as *mut GPtrArray;
    if paths.is_null() {
        paths =
            g_ptr_array_new_with_free_func(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
        g_ptr_array_add(
            paths,
            b"./\0" as *const u8 as *const std::ffi::c_char as gpointer,
        );
        g_ptr_array_add(
            paths,
            g_build_path(
                b"/\0" as *const u8 as *const std::ffi::c_char,
                b"/usr/local/share/luakit\0" as *const u8 as *const std::ffi::c_char,
                b"lib/\0" as *const u8 as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void,
            ) as gpointer,
        );
        g_ptr_array_add(
            paths,
            g_build_path(
                b"/\0" as *const u8 as *const std::ffi::c_char,
                b"/etc/xdg\0" as *const u8 as *const std::ffi::c_char,
                b"/luakit/\0" as *const u8 as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void,
            ) as gpointer,
        );
        g_ptr_array_add(
            paths,
            g_build_path(
                b"/\0" as *const u8 as *const std::ffi::c_char,
                globalconf.config_dir,
                b"/\0" as *const u8 as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void,
            ) as gpointer,
        );
    }
    let mut i: std::ffi::c_uint = 0 as std::ffi::c_int as std::ffi::c_uint;
    while i < (*paths).len {
        if if 0 != 0 {
            ({
                let __str: *const std::ffi::c_char = fct;
                let __prefix: *const std::ffi::c_char =
                    *((*paths).pdata).offset(i as isize) as *const std::ffi::c_char;
                let mut __result: gboolean = 0 as std::ffi::c_int;
                if __str.is_null() || __prefix.is_null() {
                    __result = g_str_has_prefix(__str, __prefix);
                } else {
                    let __str_len: size_t =
                        strlen(__str.offset(__str.is_null() as std::ffi::c_int as isize));
                    let __prefix_len: size_t =
                        strlen(__prefix.offset(__prefix.is_null() as std::ffi::c_int as isize));
                    if __str_len >= __prefix_len {
                        __result = (memcmp(
                            __str.offset(__str.is_null() as std::ffi::c_int as isize)
                                as *const std::ffi::c_void,
                            __prefix.offset(__prefix.is_null() as std::ffi::c_int as isize)
                                as *const std::ffi::c_void,
                            __prefix_len,
                        ) == 0 as std::ffi::c_int)
                            as std::ffi::c_int;
                    }
                }
                __result
            })
        } else {
            g_str_has_prefix(fct, *((*paths).pdata).offset(i as isize) as *const gchar)
        } != 0
        {
            fct = fct.offset(
                strlen(*((*paths).pdata).offset(i as isize) as *const std::ffi::c_char) as isize,
            );
            break;
        } else {
            i = i.wrapping_add(1);
            i;
        }
    }
    let mut len: std::ffi::c_int = strlen(fct) as std::ffi::c_int;
    let mut core: gboolean = (strcmp(
        &*fct.offset((len - 2 as std::ffi::c_int) as isize),
        b".c\0" as *const u8 as *const std::ffi::c_char,
    ) == 0
        || strcmp(
            &*fct.offset((len - 2 as std::ffi::c_int) as isize),
            b".h\0" as *const u8 as *const std::ffi::c_char,
        ) == 0) as std::ffi::c_int;
    let mut lua: gboolean = (strcmp(
        &*fct.offset((len - 4 as std::ffi::c_int) as isize),
        b".lua\0" as *const u8 as *const std::ffi::c_char,
    ) == 0
        || strncmp(
            fct,
            b"[string \"\0" as *const u8 as *const std::ffi::c_char,
            9,
        ) == 0) as std::ffi::c_int;
    if core != 0 {
        return g_strdup_printf(
            b"core/%.*s\0" as *const u8 as *const std::ffi::c_char,
            len - 2 as std::ffi::c_int,
            fct,
        );
    } else if lua != 0 {
        return g_strdup_printf(
            b"lua/%.*s\0" as *const u8 as *const std::ffi::c_char,
            len - 4 as std::ffi::c_int,
            fct,
        );
    } else {
        return g_strdup(fct);
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_level_from_string(
    mut out: *mut log_level_t,
    mut str: *const std::ffi::c_char,
) -> std::ffi::c_int {
    if strcmp(b"fatal\0" as *const u8 as *const std::ffi::c_char, str) == 0 {
        *out = LOG_LEVEL_fatal;
        return 0 as std::ffi::c_int;
    }
    if strcmp(b"error\0" as *const u8 as *const std::ffi::c_char, str) == 0 {
        *out = LOG_LEVEL_error;
        return 0 as std::ffi::c_int;
    }
    if strcmp(b"warn\0" as *const u8 as *const std::ffi::c_char, str) == 0 {
        *out = LOG_LEVEL_warn;
        return 0 as std::ffi::c_int;
    }
    if strcmp(b"info\0" as *const u8 as *const std::ffi::c_char, str) == 0 {
        *out = LOG_LEVEL_info;
        return 0 as std::ffi::c_int;
    }
    if strcmp(b"verbose\0" as *const u8 as *const std::ffi::c_char, str) == 0 {
        *out = LOG_LEVEL_verbose;
        return 0 as std::ffi::c_int;
    }
    if strcmp(b"debug\0" as *const u8 as *const std::ffi::c_char, str) == 0 {
        *out = LOG_LEVEL_debug;
        return 0 as std::ffi::c_int;
    }
    return 1 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_string_from_level(mut lvl: log_level_t) -> *const std::ffi::c_char {
    match lvl as std::ffi::c_uint {
        0 => return b"fatal\0" as *const u8 as *const std::ffi::c_char,
        1 => return b"error\0" as *const u8 as *const std::ffi::c_char,
        2 => return b"warn\0" as *const u8 as *const std::ffi::c_char,
        3 => return b"info\0" as *const u8 as *const std::ffi::c_char,
        4 => return b"verbose\0" as *const u8 as *const std::ffi::c_char,
        5 => return b"debug\0" as *const u8 as *const std::ffi::c_char,
        _ => 0 as *const std::ffi::c_char,
    }
    /*
    g_assertion_message_expr(
        0 as *mut gchar,
        b"log.c\0" as *const u8 as *const std::ffi::c_char,
        122 as std::ffi::c_int,
        (*::core::mem::transmute::<&[u8; 22], &[std::ffi::c_char; 22]>(b"log_string_from_level\0"))
            .as_ptr(),
        0 as *const std::ffi::c_char,
    );
    */
}
unsafe extern "C" fn emit_log_signal(
    mut time: std::ffi::c_double,
    mut lvl: log_level_t,
    mut group: *const gchar,
    mut msg: *const gchar,
) {
    let mut msg_class: *mut lua_class_t = msg_lib_get_msg_class();
    lua_pushnumber(common.L, time);
    lua_pushstring(common.L, log_string_from_level(lvl));
    lua_pushstring(common.L, group);
    lua_pushstring(common.L, msg);
    block_log = (0 as std::ffi::c_int == 0) as std::ffi::c_int;
    luaH_class_emit_signal(
        common.L,
        msg_class,
        b"log\0" as *const u8 as *const std::ffi::c_char,
        4 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    block_log = 0 as std::ffi::c_int;
}
static mut consumer_added: std::ffi::c_int = 0 as std::ffi::c_int;
unsafe extern "C" fn log_emit_pending_signals(
    mut UNUSED_usedata: *mut std::ffi::c_void,
) -> gboolean {
    let mut entry: *mut queued_log_t = 0 as *mut queued_log_t;
    loop {
        entry = g_async_queue_try_pop(queued_emissions) as *mut queued_log_t;
        if entry.is_null() {
            break;
        }
        emit_log_signal((*entry).time, (*entry).lvl, (*entry).group, (*entry).msg);
        g_free((*entry).group as gpointer);
        g_free((*entry).msg as gpointer);
        g_slice_free1(::core::mem::size_of::<queued_log_t>(), entry as gpointer);
    }
    let mut gais_temp: gint = 0 as std::ffi::c_int;
    if 0 as std::ffi::c_int != 0 {
        consumer_added;
    } else {
    };
    //_seqcst
    core::intrinsics::atomic_store::<_, { AtomicOrdering::SeqCst }>(
        &mut consumer_added as *mut std::ffi::c_int as *mut gint,
        *&mut gais_temp,
    );
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn queue_log_signal(
    mut time: std::ffi::c_double,
    mut lvl: log_level_t,
    mut group: *const gchar,
    mut msg: *const gchar,
) {
    let mut entry: *mut queued_log_t =
        g_slice_alloc0(::core::mem::size_of::<queued_log_t>()) as *mut queued_log_t;
    (*entry).time = time;
    (*entry).lvl = lvl;
    (*entry).group = g_strdup(group);
    (*entry).msg = g_strdup(msg);
    g_async_queue_push(queued_emissions, entry as gpointer);
    if ({
        let mut gaicae_oldval: gint = 0 as std::ffi::c_int;
        if 0 as std::ffi::c_int != 0 {
            consumer_added;
            (0 as std::ffi::c_int == 0) as std::ffi::c_int;
        } else {
        };

        //_seqcst_seqcst
        let fresh1 = ::core::intrinsics::atomic_cxchg::<
            _,
            { AtomicOrdering::SeqCst },
            { AtomicOrdering::SeqCst },
        >(
            &mut consumer_added as *mut std::ffi::c_int,
            *(&mut gaicae_oldval as *mut gint as *mut std::ffi::c_void as *mut std::ffi::c_int),
            (0 as std::ffi::c_int == 0) as std::ffi::c_int,
        );
        *(&mut gaicae_oldval as *mut gint as *mut std::ffi::c_void as *mut std::ffi::c_int) =
            fresh1.0;
        if fresh1.1 as std::ffi::c_int != 0 {
            (0 as std::ffi::c_int == 0) as std::ffi::c_int
        } else {
            0 as std::ffi::c_int
        }
    }) != 0
    {
        g_idle_add(
            Some(
                log_emit_pending_signals as unsafe extern "C" fn(*mut std::ffi::c_void) -> gboolean,
            ),
            0 as *mut std::ffi::c_void,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _log(
    mut lvl: log_level_t,
    mut fct: *const gchar,
    mut fmt: *const gchar,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    va_log(lvl, fct, fmt, ap.as_va_list());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn va_log(
    mut lvl: log_level_t,
    mut fct: *const gchar,
    mut fmt: *const gchar,
    mut ap: ::core::ffi::VaList,
) {
    let mut msg: *mut gchar = 0 as *mut gchar;
    let mut log_fd: gint = 0;
    let mut time: std::ffi::c_double = 0.;
    let mut prefix_char: gchar = 0;
    let mut style: *mut gchar = 0 as *mut gchar;
    static mut indent_lines_reg: *mut GRegex = 0 as *const GRegex as *mut GRegex;
    let mut wrapped: *mut gchar = 0 as *mut gchar;
    if block_log != 0 {
        return;
    }
    let mut group: *mut std::ffi::c_char = log_group_from_fct(fct);
    let mut verbosity: log_level_t = log_get_verbosity(group);
    if !(lvl as std::ffi::c_uint > verbosity as std::ffi::c_uint) {
        msg = g_strdup_printf(fmt, ap.as_va_list());
        log_fd = 2 as std::ffi::c_int;
        time = l_time() - globalconf.starttime;
        queue_log_signal(time, lvl, group, msg);
        prefix_char = 0;
        style = b"\0" as *const u8 as *const std::ffi::c_char as *mut gchar;
        match lvl as std::ffi::c_uint {
            0 => {
                prefix_char = 'F' as i32 as gchar;
                style = b"\x1B[41m\0" as *const u8 as *const std::ffi::c_char as *mut gchar;
            }
            1 => {
                prefix_char = 'E' as i32 as gchar;
                style = b"\x1B[31m\0" as *const u8 as *const std::ffi::c_char as *mut gchar;
            }
            2 => {
                prefix_char = 'W' as i32 as gchar;
                style = b"\x1B[33m\0" as *const u8 as *const std::ffi::c_char as *mut gchar;
            }
            3 => {
                prefix_char = 'I' as i32 as gchar;
            }
            4 => {
                prefix_char = 'V' as i32 as gchar;
            }
            5 => {
                prefix_char = 'D' as i32 as gchar;
            }
            _ => {
                g_assertion_message_expr(
                    0 as *mut gchar,
                    b"log.c\0" as *const u8 as *const std::ffi::c_char,
                    214 as std::ffi::c_int,
                    (*::core::mem::transmute::<&[u8; 7], &[std::ffi::c_char; 7]>(b"va_log\0"))
                        .as_ptr(),
                    0 as *const std::ffi::c_char,
                );
            }
        }
        if indent_lines_reg.is_null() {
            let mut err: *mut GError = 0 as *mut GError;
            indent_lines_reg = g_regex_new(
                b"\n\0" as *const u8 as *const std::ffi::c_char,
                G_REGEX_DEFAULT,
                G_REGEX_MATCH_DEFAULT,
                &mut err,
            );
            if !err.is_null() {
                g_assertion_message_error(
                    0 as *mut gchar,
                    b"log.c\0" as *const u8 as *const std::ffi::c_char,
                    226 as std::ffi::c_int,
                    (*::core::mem::transmute::<&[u8; 7], &[std::ffi::c_char; 7]>(b"va_log\0"))
                        .as_ptr(),
                    b"err\0" as *const u8 as *const std::ffi::c_char,
                    err,
                    0 as std::ffi::c_int as GQuark,
                    0 as std::ffi::c_int,
                );
            }
        }
        wrapped = g_regex_replace_literal(
            indent_lines_reg,
            msg,
            -(1 as std::ffi::c_int) as isize,
            0 as std::ffi::c_int,
            b"\n                 \0" as *const u8 as *const std::ffi::c_char,
            G_REGEX_MATCH_DEFAULT,
            0 as *mut *mut GError,
        );
        g_free(msg as gpointer);
        msg = wrapped;
        if isatty(log_fd) == 0 {
            let mut stripped: *mut gchar = strip_ansi_escapes(msg);
            g_free(msg as gpointer);
            msg = stripped;
            eprintln!("[{:12}] {:?} [{:?}]: {:?}", time, prefix_char, group, msg,);
        } else {
            eprintln!(
                "{:?}[{:12}] {:?} [{:?}]: {:?}\x1B[0m",
                style, time, prefix_char, group, msg,
            );
        }
        g_free(msg as gpointer);
        if lvl as std::ffi::c_uint == LOG_LEVEL_fatal as std::ffi::c_int as std::ffi::c_uint {
            exit(1 as std::ffi::c_int);
        }
    }
    g_free(group as gpointer);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ipc_recv_log(
    mut UNUSED_ipc: *mut ipc_endpoint_t,
    mut lua_msg: *const guint8,
    mut length: guint,
) {
    let mut L: *mut lua_State = common.L;
    let mut n: gint = lua_deserialize_range(L, lua_msg, length);
    let mut __n1: gint64 = n as gint64;
    let mut __n2: gint64 = 3 as std::ffi::c_int as gint64;
    if !(__n1 == __n2) {
        /*
        g_assertion_message(
            0 as *mut gchar,
            b"log.c\0" as *const u8 as *const std::ffi::c_char,
            256 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 13], &[std::ffi::c_char; 13]>(b"ipc_recv_log\0"))
                .as_ptr(),
            b"n == 3\0" as *const u8 as *const std::ffi::c_char,
            __n1 as guint64,
            b"==\0" as *const u8 as *const std::ffi::c_char,
            __n2 as guint64,
            'i' as i32 as std::ffi::c_char,
        );
        */
    }
    let mut lvl: log_level_t = lua_tointeger(L, -(3 as std::ffi::c_int)) as log_level_t;
    let mut fct: *const gchar = lua_tolstring(L, -(2 as std::ffi::c_int), 0 as *mut size_t);
    let mut msg: *const gchar = lua_tolstring(L, -(1 as std::ffi::c_int), 0 as *mut size_t);
    _log(
        lvl,
        fct,
        b"%s\0" as *const u8 as *const std::ffi::c_char,
        msg,
    );
    lua_settop(L, -(3 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_init() {
    queued_emissions = g_async_queue_new();
    let mut log_dump_file: *const std::ffi::c_char =
        getenv(b"LUAKIT_QUEUED_EMISSIONS_FILE\0" as *const u8 as *const std::ffi::c_char);
    unsetenv(b"LUAKIT_QUEUED_EMISSIONS_FILE\0" as *const u8 as *const std::ffi::c_char);
    if log_dump_file.is_null() || file_exists(log_dump_file) == 0 {
        return;
    }
    let mut dump = 0 as *mut std::ffi::c_char;
    let mut len: size_t = 0;
    let mut error: *mut GError = 0 as *mut GError;
    if g_file_get_contents(log_dump_file, &mut (dump as *mut u8), &mut len, &mut error) == 0 {
        _log(
            LOG_LEVEL_error,
            b"log.c\0" as *const u8 as *const std::ffi::c_char,
            b"unable to load previous log messages: %s\0" as *const u8 as *const std::ffi::c_char,
            (*error).message,
        );
        g_error_free(error);
        return;
    }
    unlink(log_dump_file);
    let mut end: *mut std::ffi::c_char = dump.offset(len as isize);
    g_async_queue_lock(queued_emissions);
    while dump < end {
        let mut entry: *mut queued_log_t =
            g_slice_alloc0(::core::mem::size_of::<queued_log_t>()) as *mut queued_log_t;
        let fresh2 = dump;
        dump = dump.offset(1);
        (*entry).lvl = *fresh2 as log_level_t;
        (*entry).time = g_ascii_strtod(dump, &mut dump);
        (*entry).group = g_strdup(dump);
        dump = dump.offset((strlen(dump)).wrapping_add(1) as isize);
        (*entry).msg = g_strdup(dump);
        dump = dump.offset((strlen(dump)).wrapping_add(1) as isize);
        g_async_queue_push_unlocked(queued_emissions, entry as gpointer);
    }
    g_async_queue_unlock(queued_emissions);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_dump_queued_emissions() -> *mut std::ffi::c_char {
    let mut dump: *mut GString = g_string_new(0 as *const gchar);
    g_async_queue_lock(queued_emissions);
    let mut entry: *mut queued_log_t = 0 as *mut queued_log_t;
    loop {
        entry = g_async_queue_try_pop_unlocked(queued_emissions) as *mut queued_log_t;
        if entry.is_null() {
            break;
        }
        g_string_append_c(dump, (*entry).lvl as std::ffi::c_char);
        g_string_append_printf(
            dump,
            b"%f\0" as *const u8 as *const std::ffi::c_char,
            (*entry).time,
        );
        if 0 != 0 {
            ({
                let __val: *const std::ffi::c_char = (*entry).group;
                g_string_append_len(
                    dump,
                    __val,
                    if !__val.is_null() {
                        strlen(__val.offset(__val.is_null() as std::ffi::c_int as isize)) as isize
                    } else {
                        -(1 as std::ffi::c_int) as isize
                    },
                );
                // compile_error!("Function call expression is not supposed to be used")
            });
            ({
                let __val: *const std::ffi::c_char = (*entry).group;
                g_string_append_len(
                    dump,
                    __val,
                    if !__val.is_null() {
                        strlen(__val.offset(__val.is_null() as std::ffi::c_int as isize)) as isize
                    } else {
                        -(1 as std::ffi::c_int) as isize
                    },
                );
                // compile_error!("Function call expression is not supposed to be used")
            });
        } else {
            g_string_append_len(dump, (*entry).group, -(1 as std::ffi::c_int) as isize);
        };
        g_string_append_c(dump, '\0' as i32 as gchar);
        if 0 != 0 {
            ({
                let __val: *const std::ffi::c_char = (*entry).msg;
                g_string_append_len(
                    dump,
                    __val,
                    if !__val.is_null() {
                        strlen(__val.offset(__val.is_null() as std::ffi::c_int as isize)) as isize
                    } else {
                        -(1 as std::ffi::c_int) as isize
                    },
                );
                // compile_error!("Function call expression is not supposed to be used")
            });
            ({
                let __val: *const std::ffi::c_char = (*entry).msg;
                g_string_append_len(
                    dump,
                    __val,
                    if !__val.is_null() {
                        strlen(__val.offset(__val.is_null() as std::ffi::c_int as isize)) as isize
                    } else {
                        -(1 as std::ffi::c_int) as isize
                    },
                );
                // compile_error!("Function call expression is not supposed to be used")
            });
        } else {
            g_string_append_len(dump, (*entry).msg, -1);
        };
        g_string_append_c(dump, '\0' as i32 as gchar);
        g_free((*entry).group as gpointer);
        g_free((*entry).msg as gpointer);
        g_slice_free1(::core::mem::size_of::<queued_log_t>(), entry as gpointer);
    }
    g_async_queue_unlock(queued_emissions);
    let mut name_used: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut log_dump_fd: std::ffi::c_int = g_file_open_tmp(
        b"luakit-log-dump.XXXXXX\0" as *const u8 as *const std::ffi::c_char,
        &mut name_used,
        0 as *mut *mut GError,
    );
    if log_dump_fd != -(1 as std::ffi::c_int) {
        let mut written: ssize_t = write(
            log_dump_fd,
            (*dump).str as *const std::ffi::c_void,
            (*dump).len,
        );
        close(log_dump_fd);
        if written != (*dump).len as ssize_t {
            unlink(name_used);
            g_free(name_used as gpointer);
            name_used = 0 as *mut std::ffi::c_char;
        }
    }
    g_string_free(dump, (0 as std::ffi::c_int == 0) as std::ffi::c_int);
    return name_used;
}
