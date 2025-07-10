use gdk_sys::{
    GDK_CONTROL_MASK, GDK_LOCK_MASK, GDK_MOD1_MASK, GDK_MOD2_MASK, GDK_MOD3_MASK, GDK_MOD4_MASK,
    GDK_MOD5_MASK, GDK_MODIFIER_MASK, GDK_SHIFT_MASK, gdk_keyval_name, gdk_keyval_to_unicode,
};
use glib_sys::{
    GPtrArray, g_build_filename, g_free, g_get_system_config_dirs, g_ptr_array_add,
    g_ptr_array_free, g_ptr_array_insert, g_ptr_array_new_with_free_func, g_strdup,
    g_strdup_printf, g_strfreev, g_strjoinv, g_strsplit, g_unichar_isgraph, g_unichar_to_utf8,
    gboolean, gpointer,
};
use libc::atoi;
use libc::execvp;
use libc::getenv;
use libc::setenv;
use libc::unsetenv;
use mlua_sys::{
    lua_Integer, lua_State, lua_atpanic, lua_createtable, lua_pushstring, lua_rawseti,
    lua_setfield, lua_settop, lua_tolstring, luaL_loadfile, luaL_newstate, luaL_openlibs,
};

use crate::clib::download::download_class_setup;
use crate::clib::luakit::luakit_lib_setup;
use crate::clib::msg::msg_lib_setup;
use crate::clib::request::request_class_setup;
use crate::clib::soup::soup_lib_setup;
use crate::clib::sqlite3::sqlite3_class_setup;
use crate::clib::stylesheet::stylesheet_class_setup;
use crate::clib::unique::unique_lib_setup;
use crate::clib::web_module::web_module_lib_setup;
use crate::clib::widget::widget_class_setup;
use crate::clib::xdg::xdg_lib_setup;
use crate::common::clib::regex::regex_class_setup;
use crate::common::clib::timer::timer_class_setup;
use crate::common::clib::utf8::utf8_lib_setup;
use crate::common::common;
use crate::common::luah::luaH_fixups;
use crate::common::lualib::luaH_dofunction;
use crate::common::luaobject::luaH_object_setup;
use crate::common::luautil::luaH_add_paths;
use crate::common::luayield::luaH_yield_setup;
use crate::common::util::{file_exists, luaH_panic};
use crate::globalconf::globalconf;
use crate::gtypes::{gchar, gint, guint, guint32};
use crate::ipc::ipc_remove_socket_file;
use crate::ipc_common::clib::ipc::ipc_channel_class_setup;
use crate::log::{
    _log, LOG_LEVEL_error, LOG_LEVEL_info, LOG_LEVEL_verbose, LOG_LEVEL_warn,
    log_dump_queued_emissions,
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_modifier_table_push(mut L: *mut lua_State, mut state: guint) {
    let mut i = 1;
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    if state & GDK_MODIFIER_MASK as std::ffi::c_int as guint != 0 {
        if state & GDK_SHIFT_MASK as std::ffi::c_int as guint != 0 {
            lua_pushstring(L, b"Shift\0" as *const u8 as *const std::ffi::c_char);
            let fresh0 = i;
            i = i + 1;
            lua_rawseti(L, -(2 as std::ffi::c_int), fresh0);
        }
        if state & GDK_LOCK_MASK as std::ffi::c_int as guint != 0 {
            lua_pushstring(L, b"Lock\0" as *const u8 as *const std::ffi::c_char);
            let fresh1 = i;
            i = i + 1;
            lua_rawseti(L, -(2 as std::ffi::c_int), fresh1);
        }
        if state & GDK_CONTROL_MASK as std::ffi::c_int as guint != 0 {
            lua_pushstring(L, b"Control\0" as *const u8 as *const std::ffi::c_char);
            let fresh2 = i;
            i = i + 1;
            lua_rawseti(L, -(2 as std::ffi::c_int), fresh2);
        }
        if state & GDK_MOD1_MASK as std::ffi::c_int as guint != 0 {
            lua_pushstring(L, b"Mod1\0" as *const u8 as *const std::ffi::c_char);
            let fresh3 = i;
            i = i + 1;
            lua_rawseti(L, -(2 as std::ffi::c_int), fresh3);
        }
        if state & GDK_MOD2_MASK as std::ffi::c_int as guint != 0 {
            lua_pushstring(L, b"Mod2\0" as *const u8 as *const std::ffi::c_char);
            let fresh4 = i;
            i = i + 1;
            lua_rawseti(L, -(2 as std::ffi::c_int), fresh4);
        }
        if state & GDK_MOD3_MASK as std::ffi::c_int as guint != 0 {
            lua_pushstring(L, b"Mod3\0" as *const u8 as *const std::ffi::c_char);
            let fresh5 = i;
            i = i + 1;
            lua_rawseti(L, -(2 as std::ffi::c_int), fresh5);
        }
        if state & GDK_MOD4_MASK as std::ffi::c_int as guint != 0 {
            lua_pushstring(L, b"Mod4\0" as *const u8 as *const std::ffi::c_char);
            let fresh6 = i;
            i = i + 1;
            lua_rawseti(L, -(2 as std::ffi::c_int), fresh6);
        }
        if state & GDK_MOD5_MASK as std::ffi::c_int as guint != 0 {
            lua_pushstring(L, b"Mod5\0" as *const u8 as *const std::ffi::c_char);
            let fresh7 = i;
            i = i + 1;
            lua_rawseti(L, -(2 as std::ffi::c_int), fresh7);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_keystr_push(mut L: *mut lua_State, mut keyval: guint) {
    let mut ucs: [gchar; 7] = [0; 7];
    let mut ulen: guint = 0;
    let mut ukval: guint32 = gdk_keyval_to_unicode(keyval);
    if g_unichar_isgraph(ukval) != 0 {
        ulen = g_unichar_to_utf8(ukval, ucs.as_mut_ptr()) as guint;
        ucs[ulen as usize] = 0 as std::ffi::c_int as gchar;
        lua_pushstring(L, ucs.as_mut_ptr());
    } else {
        lua_pushstring(L, gdk_keyval_name(keyval));
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_init(mut uris: *mut *mut gchar) {
    common.L = luaL_newstate();
    let mut L: *mut lua_State = common.L;
    lua_atpanic(L, luaH_panic);
    luaL_openlibs(L);
    luaH_fixups(L);
    luaH_object_setup(L);
    luakit_lib_setup(L);
    xdg_lib_setup(L);
    soup_lib_setup(L);
    if globalconf.nounique == 0 {
        unique_lib_setup(L);
    }
    widget_class_setup(L);
    download_class_setup(L);
    sqlite3_class_setup(L);
    timer_class_setup(L);
    regex_class_setup(L);
    utf8_lib_setup(L);
    request_class_setup(L);
    stylesheet_class_setup(L);
    web_module_lib_setup(L);
    ipc_channel_class_setup(L);
    msg_lib_setup(L);
    luaH_yield_setup(L);
    luaH_add_paths(L, globalconf.config_dir);
    let mut uri: *const gchar = 0 as *const gchar;
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    let mut i: gint = 0 as std::ffi::c_int;
    while !uris.is_null() && {
        uri = *uris.offset(i as isize);
        !uri.is_null()
    } {
        lua_pushstring(L, uri);
        lua_rawseti(L, -(2 as std::ffi::c_int), (i + 1) as lua_Integer);
        i += 1;
        i;
    }
    lua_setfield(
        L,
        -(10002 as std::ffi::c_int),
        b"uris\0" as *const u8 as *const std::ffi::c_char,
    );
}
unsafe extern "C" fn luaH_loadrc(mut confpath: *const gchar, mut run: gboolean) -> gboolean {
    _log(
        LOG_LEVEL_info,
        b"luah.c\0" as *const u8 as *const std::ffi::c_char,
        b"Loading rc: %s\0" as *const u8 as *const std::ffi::c_char,
        confpath,
    );
    let mut L: *mut lua_State = common.L;
    if luaL_loadfile(L, confpath) != 0 {
        _log(
            LOG_LEVEL_error,
            b"luah.c\0" as *const u8 as *const std::ffi::c_char,
            b"Error loading rc: %s\0" as *const u8 as *const std::ffi::c_char,
            lua_tolstring(L, -(1 as std::ffi::c_int), 0 as *mut usize),
        );
        return 0 as std::ffi::c_int;
    }
    if run == 0 {
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        return (0 as std::ffi::c_int == 0) as std::ffi::c_int;
    }
    return luaH_dofunction(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_parserc(mut confpath: *const gchar, mut run: gboolean) -> gboolean {
    let mut i_str: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut i: gint = 0;
    let mut path_0: *const gchar = 0 as *const gchar;
    let mut parts: *mut *mut gchar = 0 as *mut *mut gchar;
    let mut escaped_execpath: *mut gchar = 0 as *mut gchar;
    let mut argv: *mut GPtrArray = 0 as *mut GPtrArray;
    let mut log_dump_file: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut config_dirs: *const *const gchar = 0 as *const *const gchar;
    let mut ret: gboolean = 0 as std::ffi::c_int;
    let mut paths: *mut GPtrArray = 0 as *mut GPtrArray;
    if !confpath.is_null() {
        ret = luaH_loadrc(confpath, run);
    } else {
        paths =
            g_ptr_array_new_with_free_func(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
        g_ptr_array_add(
            paths,
            g_build_filename(
                globalconf.config_dir,
                b"rc.lua\0" as *const u8 as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void,
            ) as gpointer,
        );
        config_dirs = g_get_system_config_dirs();
        while !(*config_dirs).is_null() {
            g_ptr_array_add(
                paths,
                g_build_filename(
                    *config_dirs,
                    b"luakit\0" as *const u8 as *const std::ffi::c_char,
                    b"rc.lua\0" as *const u8 as *const std::ffi::c_char,
                    0 as *mut std::ffi::c_void,
                ) as gpointer,
            );
            config_dirs = config_dirs.offset(1);
            config_dirs;
        }
        i_str = getenv(b"LUAKIT_NEXT_CONFIG_INDEX\0" as *const u8 as *const std::ffi::c_char);
        i = if !i_str.is_null() {
            atoi(i_str)
        } else {
            0 as std::ffi::c_int
        };
        if !(!i_str.is_null() && (i <= 0 as std::ffi::c_int || i >= (*paths).len as gint)) {
            while i < (*paths).len as gint {
                let mut path: *const gchar = *((*paths).pdata).offset(i as isize) as *const gchar;
                if file_exists(path) != 0 {
                    break;
                }
                _log(
                    LOG_LEVEL_verbose,
                    b"luah.c\0" as *const u8 as *const std::ffi::c_char,
                    b"rc file '%s' does not exist\0" as *const u8 as *const std::ffi::c_char,
                    path,
                );
                i += 1;
                i;
            }
            if i == (*paths).len as gint {
                _log(
                    LOG_LEVEL_warn,
                    b"luah.c\0" as *const u8 as *const std::ffi::c_char,
                    b"couldn't load any rc file\0" as *const u8 as *const std::ffi::c_char,
                );
            } else {
                let fresh8 = i;
                i = i + 1;
                path_0 = *((*paths).pdata).offset(fresh8 as isize) as *const gchar;
                if luaH_loadrc(path_0, run) != 0 {
                    unsetenv(b"LUAKIT_NEXT_CONFIG_INDEX\0" as *const u8 as *const std::ffi::c_char);
                    globalconf.confpath = g_strdup(path_0);
                    ret = (0 as std::ffi::c_int == 0) as std::ffi::c_int;
                } else {
                    _log(
                        LOG_LEVEL_warn,
                        b"luah.c\0" as *const u8 as *const std::ffi::c_char,
                        b"loading rc '%s' failed, falling back...\0" as *const u8
                            as *const std::ffi::c_char,
                        path_0,
                    );
                    i_str = g_strdup_printf(b"%i\0" as *const u8 as *const std::ffi::c_char, i);
                    setenv(
                        b"LUAKIT_NEXT_CONFIG_INDEX\0" as *const u8 as *const std::ffi::c_char,
                        i_str,
                        (0 as std::ffi::c_int == 0) as std::ffi::c_int,
                    );
                    g_free(i_str as gpointer);
                    parts = g_strsplit(
                        globalconf.execpath,
                        b" \0" as *const u8 as *const std::ffi::c_char,
                        -(1 as std::ffi::c_int),
                    );
                    escaped_execpath =
                        g_strjoinv(b"\\ \0" as *const u8 as *const std::ffi::c_char, parts);
                    g_strfreev(parts);
                    argv = globalconf.argv;
                    g_ptr_array_insert(argv, 0 as std::ffi::c_int, escaped_execpath as gpointer);
                    g_ptr_array_add(argv, 0 as *mut std::ffi::c_void);
                    _log(
                        LOG_LEVEL_verbose,
                        b"luah.c\0" as *const u8 as *const std::ffi::c_char,
                        b"exec: %s\0" as *const u8 as *const std::ffi::c_char,
                        g_strjoinv(
                            b" \0" as *const u8 as *const std::ffi::c_char,
                            (*argv).pdata as *mut *mut gchar,
                        ),
                    );
                    log_dump_file = log_dump_queued_emissions();
                    if !log_dump_file.is_null() {
                        setenv(
                            b"LUAKIT_QUEUED_EMISSIONS_FILE\0" as *const u8
                                as *const std::ffi::c_char,
                            log_dump_file,
                            (0 as std::ffi::c_int == 0) as std::ffi::c_int,
                        );
                        g_free(log_dump_file as gpointer);
                    }
                    ipc_remove_socket_file();
                    execvp(
                        escaped_execpath,
                        (*argv).pdata as *mut *mut gchar as *const *const std::ffi::c_char,
                    );
                }
            }
        }
    }
    if !paths.is_null() {
        g_ptr_array_free(paths, (0 as std::ffi::c_int == 0) as std::ffi::c_int);
    }
    return ret;
}
