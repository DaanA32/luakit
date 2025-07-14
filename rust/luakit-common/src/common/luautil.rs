use gdk_sys::*;
use glib_sys::*;
use libc::*;
use mlua_sys::*;
use std::mem::MaybeUninit;

use crate::common::luaclass::*;
use crate::common::luah::*;
use crate::common::lualib::*;
use crate::common::luaobject::*;
use crate::common::util::*;
use crate::common::*;
use crate::gtypes::{gchar, gint, guint};
use crate::log::{_log, LOG_LEVEL_warn};

#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_traceback(
    mut L: *mut lua_State,
    mut T: *mut lua_State,
    mut min_level: gint,
) -> gint {
    let mut ar = MaybeUninit::uninit();
    let mut max_level: gint = 0;
    let mut loc_pad: gint = 0 as std::ffi::c_int;
    if lua_getstack(T, min_level, ar.as_mut_ptr()) == 0 {
        lua_pushlstring(L, b"\0" as *const u8 as *const std::ffi::c_char, 0);
        return 1 as std::ffi::c_int;
    }
    let mut level: gint = min_level;
    while lua_getstack(T, level, ar.as_mut_ptr()) != 0 {
        lua_getinfo(
            T,
            b"Sl\0" as *const u8 as *const std::ffi::c_char,
            ar.as_mut_ptr(),
        );
        max_level = level;
        let mut cur_pad: gint = snprintf(
            std::ptr::null_mut(),
            0,
            b"%s:%d\0" as *const u8 as *const std::ffi::c_char,
            if !(g_strstr_len(
                (*ar.as_ptr()).source,
                3 as std::ffi::c_int as ssize_t,
                b"@./\0" as *const u8 as *const std::ffi::c_char,
            ))
            .is_null()
            {
                ((*ar.as_ptr()).source).offset(3 as std::ffi::c_int as isize)
            } else if *((*ar.as_ptr()).source).offset(0 as std::ffi::c_int as isize)
                as std::ffi::c_int
                == '@' as i32
            {
                ((*ar.as_ptr()).source).offset(1 as std::ffi::c_int as isize)
            } else {
                ((*ar.as_ptr()).short_src).as_ptr() as *const std::ffi::c_char
            },
            (*ar.as_ptr()).currentline,
        );
        if cur_pad > loc_pad {
            loc_pad = cur_pad;
        }
        level += 1;
        level;
    }
    let mut tb: *mut GString = g_string_new(b"\0" as *const u8 as *const std::ffi::c_char);
    let mut level_pad: gint = snprintf(
        std::ptr::null_mut(),
        0,
        b"%d\0" as *const u8 as *const std::ffi::c_char,
        max_level,
    );
    let mut level_0: gint = min_level;
    while level_0 <= max_level {
        lua_getstack(T, level_0, ar.as_mut_ptr());
        lua_getinfo(
            T,
            b"Sln\0" as *const u8 as *const std::ffi::c_char,
            ar.as_mut_ptr(),
        );
        let mut shown_level: gint = level_0 - min_level + 1 as std::ffi::c_int;
        g_string_append_printf(
            tb,
            b"\x1B[37m(%*d)\x1B[0m \0" as *const u8 as *const std::ffi::c_char,
            level_pad,
            shown_level,
        );
        if strcmp(
            (*ar.as_ptr()).what,
            b"C\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            g_string_append_printf(
                tb,
                b"%-*s\0" as *const u8 as *const std::ffi::c_char,
                loc_pad,
                b"[C]\0" as *const u8 as *const std::ffi::c_char,
            );
        } else {
            let mut src: *const std::ffi::c_char = if !(g_strstr_len(
                (*ar.as_ptr()).source,
                3 as std::ffi::c_int as ssize_t,
                b"@./\0" as *const u8 as *const std::ffi::c_char,
            ))
            .is_null()
            {
                ((*ar.as_ptr()).source).offset(3 as std::ffi::c_int as isize)
            } else if *((*ar.as_ptr()).source).offset(0 as std::ffi::c_int as isize)
                as std::ffi::c_int
                == '@' as i32
            {
                ((*ar.as_ptr()).source).offset(1 as std::ffi::c_int as isize)
            } else {
                ((*ar.as_ptr()).short_src).as_ptr() as *const std::ffi::c_char
            };
            let mut n: std::ffi::c_int = 0;
            let mut cl: [std::ffi::c_char; 8] = *::core::mem::transmute::<
                &[u8; 8],
                &mut [std::ffi::c_char; 8],
            >(b"\0\0\0\0\0\0\0\0");
            snprintf(
                cl.as_mut_ptr(),
                ::core::mem::size_of::<[std::ffi::c_char; 8]>(),
                b"%d\0" as *const u8 as *const std::ffi::c_char,
                (*ar.as_ptr()).currentline,
            );
            n = (strlen(src))
                .wrapping_add(strlen(cl.as_mut_ptr()))
                .wrapping_add(1) as std::ffi::c_int;
            g_string_append_printf(
                tb,
                b"%s:%d\0" as *const u8 as *const std::ffi::c_char,
                src,
                (*ar.as_ptr()).currentline,
            );
            g_string_append_printf(
                tb,
                b"%*.*s\0" as *const u8 as *const std::ffi::c_char,
                loc_pad - n,
                loc_pad - n,
                b"\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        if strcmp(
            (*ar.as_ptr()).what,
            b"main\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int
        {
            if 0 != 0 {
                ({
                    let __val: *const std::ffi::c_char =
                        b"\x1B[37m in main chunk\x1B[0m\0" as *const u8 as *const std::ffi::c_char;
                    g_string_append_len(
                        tb,
                        __val,
                        if !__val.is_null() {
                            strlen(__val.offset(__val.is_null() as std::ffi::c_int as isize))
                                as ssize_t
                        } else {
                            -(1 as std::ffi::c_int) as ssize_t
                        },
                    );
                });
                ({
                    let __val: *const std::ffi::c_char =
                        b"\x1B[37m in main chunk\x1B[0m\0" as *const u8 as *const std::ffi::c_char;
                    g_string_append_len(
                        tb,
                        __val,
                        if !__val.is_null() {
                            strlen(__val.offset(__val.is_null() as std::ffi::c_int as isize))
                                as ssize_t
                        } else {
                            -(1 as std::ffi::c_int) as ssize_t
                        },
                    );
                });
            } else {
                g_string_append_len(
                    tb,
                    b"\x1B[37m in main chunk\x1B[0m\0" as *const u8 as *const std::ffi::c_char,
                    -(1 as std::ffi::c_int) as ssize_t,
                );
            };
        } else {
            g_string_append_printf(
                tb,
                b"\x1B[37m in function \x1B[0m%s\0" as *const u8 as *const std::ffi::c_char,
                if !((*ar.as_ptr()).name).is_null() {
                    (*ar.as_ptr()).name
                } else {
                    b"[anonymous]\0" as *const u8 as *const std::ffi::c_char
                },
            );
        }
        if level_0 != max_level {
            if 0 != 0 {
                ({
                    let __val: *const std::ffi::c_char =
                        b"\n\0" as *const u8 as *const std::ffi::c_char;
                    g_string_append_len(
                        tb,
                        __val,
                        if !__val.is_null() {
                            strlen(__val.offset(__val.is_null() as std::ffi::c_int as isize))
                                as ssize_t
                        } else {
                            -(1 as std::ffi::c_int) as ssize_t
                        },
                    );
                });
                ({
                    let __val: *const std::ffi::c_char =
                        b"\n\0" as *const u8 as *const std::ffi::c_char;
                    g_string_append_len(
                        tb,
                        __val,
                        if !__val.is_null() {
                            strlen(__val.offset(__val.is_null() as std::ffi::c_int as isize))
                                as ssize_t
                        } else {
                            -(1 as std::ffi::c_int) as ssize_t
                        },
                    );
                });
            } else {
                g_string_append_len(
                    tb,
                    b"\n\0" as *const u8 as *const std::ffi::c_char,
                    -(1 as std::ffi::c_int) as ssize_t,
                );
            };
        }
        level_0 += 1;
        level_0;
    }
    lua_pushstring(L, (*tb).str);
    if 0 != 0 {
        if 0 as std::ffi::c_int == 0 {
            g_string_free(tb, (0 as std::ffi::c_int == 0) as std::ffi::c_int);
        } else {
            g_string_free_and_steal(tb);
        };
    } else {
        g_string_free(tb, (0 as std::ffi::c_int == 0) as std::ffi::c_int);
    };
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn extract_error_message(
    mut L: *mut lua_State,
    mut message: *const gchar,
) -> *const gchar {
    let mut ar = MaybeUninit::uninit();
    let mut level: gint = 0 as std::ffi::c_int;
    loop {
        if lua_getstack(L, level, ar.as_mut_ptr()) == 0 {
            return message;
        }
        lua_getinfo(
            L,
            b"Sl\0" as *const u8 as *const std::ffi::c_char,
            ar.as_mut_ptr(),
        );
        if !(strcmp(
            (*ar.as_ptr()).what,
            b"C\0" as *const u8 as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int)
        {
            break;
        }
        level += 1;
        level;
    }
    if strncmp(
        message,
        ((*ar.as_ptr()).short_src).as_ptr(),
        strlen(((*ar.as_ptr()).short_src).as_ptr()),
    ) != 0
    {
        return message;
    }
    let mut tail: *const gchar = message.offset(strlen((*ar.as_ptr()).short_src.as_ptr()) as isize);
    if *tail as std::ffi::c_int != ':' as i32 {
        return message;
    }
    tail = tail.offset(1);
    tail;
    return (strchr(tail, ' ' as i32)).offset(1 as std::ffi::c_int as isize);
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_dofunction_on_error(mut L: *mut lua_State) -> gint {
    if lua_checkstack(L, 5 as std::ffi::c_int) != 0 {
    } else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/luautil.c\0" as *const u8 as *const std::ffi::c_char,
            129 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 25], &[std::ffi::c_char; 25]>(
                b"luaH_dofunction_on_error\0",
            ))
            .as_ptr(),
            b"lua_checkstack(L, 5)\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    lua_pushlstring(
        L,
        b"Lua error: \0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 12]>())
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
            .wrapping_sub(1),
    );
    lua_pushstring(
        L,
        extract_error_message(
            L,
            lua_tolstring(L, -(2 as std::ffi::c_int), 0 as *mut usize),
        ),
    );
    lua_pushlstring(
        L,
        b"\nTraceback:\n\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 13]>())
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
            .wrapping_sub(1),
    );
    luaH_traceback(L, L, 1 as std::ffi::c_int);
    lua_concat(L, 4 as std::ffi::c_int);
    return 1 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_add_paths(
    mut L: *mut lua_State,
    mut config_dir: *const gchar,
) {
    lua_getfield(
        L,
        -(10002 as std::ffi::c_int),
        b"package\0" as *const u8 as *const std::ffi::c_char,
    );
    if 5 as std::ffi::c_int != lua_type(L, -(1 as std::ffi::c_int)) {
        _log(
            LOG_LEVEL_warn,
            b"common/luautil.c\0" as *const u8 as *const std::ffi::c_char,
            "package is not a table\0",
        );
        return;
    }
    lua_getfield(
        L,
        -(1 as std::ffi::c_int),
        b"path\0" as *const u8 as *const std::ffi::c_char,
    );
    if 4 as std::ffi::c_int != lua_type(L, -(1 as std::ffi::c_int)) {
        _log(
            LOG_LEVEL_warn,
            b"common/luautil.c\0" as *const u8 as *const std::ffi::c_char,
            "package.path is not a string",
        );
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        return;
    }
    let mut paths: *mut GPtrArray = g_ptr_array_new_with_free_func(Some(g_free));
    g_ptr_array_add(
        paths,
        g_build_filename(
            b"/usr/local/share/luakit\0" as *const u8 as *const std::ffi::c_char,
            b"lib\0" as *const u8 as *const std::ffi::c_char,
            0 as *mut std::ffi::c_void,
        ) as gpointer,
    );
    if !config_dir.is_null() {
        g_ptr_array_add(paths, g_strdup(config_dir) as gpointer);
    }
    let mut config_dirs: *const *const gchar = g_get_system_config_dirs();
    while !(*config_dirs).is_null() {
        g_ptr_array_add(
            paths,
            g_build_filename(
                *config_dirs,
                b"luakit\0" as *const u8 as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void,
            ) as gpointer,
        );
        config_dirs = config_dirs.offset(1);
        config_dirs;
    }
    let mut path: *const gchar = 0 as *const gchar;
    let mut i: guint = 0 as std::ffi::c_int as guint;
    while i < (*paths).len {
        path = *((*paths).pdata).offset(i as isize) as *const gchar;
        lua_pushlstring(
            L,
            b";\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 2]>())
                .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
                .wrapping_sub(1),
        );
        lua_pushstring(L, path);
        lua_pushlstring(
            L,
            b"/?.lua\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 7]>())
                .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
                .wrapping_sub(1),
        );
        lua_concat(L, 3 as std::ffi::c_int);
        lua_pushlstring(
            L,
            b";\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 2]>())
                .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
                .wrapping_sub(1),
        );
        lua_pushstring(L, path);
        lua_pushlstring(
            L,
            b"/?/init.lua\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 12]>())
                .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
                .wrapping_sub(1),
        );
        lua_concat(L, 3 as std::ffi::c_int);
        lua_concat(L, 3 as std::ffi::c_int);
        i = i.wrapping_add(1);
        i;
    }
    g_ptr_array_free(paths, (0 as std::ffi::c_int == 0) as std::ffi::c_int);
    lua_setfield(
        L,
        -(2 as std::ffi::c_int),
        b"path\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_push_gerror(
    mut L: *mut lua_State,
    mut error: *mut GError,
) -> gint {
    if !error.is_null() {
    } else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"common/luautil.c\0" as *const u8 as *const std::ffi::c_char,
            205 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 17], &[std::ffi::c_char; 17]>(b"luaH_push_gerror\0"))
                .as_ptr(),
            b"error\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    lua_createtable(L, 0 as std::ffi::c_int, 2 as std::ffi::c_int);
    lua_pushfstring(
        L,
        b"%s-%d\0" as *const u8 as *const std::ffi::c_char,
        g_quark_to_string((*error).domain),
        (*error).code,
    );
    lua_setfield(
        L,
        -(2 as std::ffi::c_int),
        b"code\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_pushstring(L, (*error).message);
    lua_setfield(
        L,
        -(2 as std::ffi::c_int),
        b"message\0" as *const u8 as *const std::ffi::c_char,
    );
    return 1 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_push_strv(
    mut L: *mut lua_State,
    mut strv: *const *const gchar,
) -> gint {
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    if strv.is_null() {
        return 1 as std::ffi::c_int;
    }
    let mut n = 1;
    while !(*strv).is_null() {
        lua_pushstring(L, *strv);
        let fresh0 = n;
        n = n + 1;
        lua_rawseti(L, -(2 as std::ffi::c_int), fresh0);
        strv = strv.offset(1);
        strv;
    }
    return 1 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_checkstrv(
    mut L: *mut lua_State,
    mut idx: gint,
) -> *mut *const gchar {
    if !(lua_type(L, idx) == 5 as std::ffi::c_int) {
        luaL_typerror(L, idx, b"table\0" as *const u8 as *const std::ffi::c_char);
    }
    let mut len: gint = lua_rawlen(L, idx) as gint;
    let mut langs: *mut GPtrArray = g_ptr_array_new();
    let mut i: gint = 1 as std::ffi::c_int;
    while i <= len {
        lua_rawgeti(L, idx, i as lua_Integer);
        if lua_isstring(L, -(1 as std::ffi::c_int)) == 0 {
            g_ptr_array_free(langs, (0 as std::ffi::c_int == 0) as std::ffi::c_int);
            luaL_error(
                L,
                b"bad argument %d ({string} expected, but array item %d has type %s)\0" as *const u8
                    as *const std::ffi::c_char,
                idx,
                i,
                lua_typename(L, lua_type(L, -(1 as std::ffi::c_int))),
            );
        }
        g_ptr_array_add(
            langs,
            lua_tolstring(L, -(1 as std::ffi::c_int), std::ptr::null_mut()) as *mut gchar
                as gpointer,
        );
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        i += 1;
        i;
    }
    g_ptr_array_add(langs, 0 as *mut std::ffi::c_void);
    let mut strv: *mut *const gchar = (*langs).pdata as *mut *const gchar;
    g_ptr_array_free(langs, 0 as std::ffi::c_int);
    return strv;
}
