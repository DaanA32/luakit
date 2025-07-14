use std::ffi::CStr;

use glib_sys::*;
use gobject_sys::*;
use libc::size_t;
use mlua::ffi::*;
use webkit2gtk_sys::*;

pub mod property_h {
    use glib_sys::gboolean;

    use crate::{common::tokenize::luakit_token_t, gtypes::*};

    pub type property_value_t = std::ffi::c_uint;
    pub const URI: property_value_t = 5;
    pub const INT: property_value_t = 4;
    pub const FLOAT: property_value_t = 3;
    pub const DOUBLE: property_value_t = 2;
    pub const CHAR: property_value_t = 1;
    pub const BOOL: property_value_t = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union property_tmp_t {
        pub c: *mut gchar,
        pub b: gboolean,
        pub d: gdouble,
        pub f: gfloat,
        pub i: gint,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct property_t {
        pub tok: luakit_token_t,
        pub name: *const gchar,
        pub type_0: property_value_t,
        pub writable: gboolean,
    }
}
use crate::{
    common::{luah::luaH_checkboolean, tokenize::luakit_token_t},
    gtypes::*,
    log::{_log, LOG_LEVEL_warn},
};

pub use self::property_h::{
    BOOL, CHAR, DOUBLE, FLOAT, INT, URI, property_t, property_tmp_t, property_value_t,
};
unsafe extern "C" fn luaH_gobject_get(
    mut L: *mut lua_State,
    mut p: *mut property_t,
    mut object: *mut GObject,
) -> gint {
    let mut u: *mut GUri = 0 as *mut GUri;
    let mut tmp: property_tmp_t = property_tmp_t { c: 0 as *mut gchar };
    match (*p).type_0 as std::ffi::c_uint {
        0 => {
            g_object_get(
                object,
                (*p).name,
                &mut tmp.b as *mut gboolean,
                0 as *mut std::ffi::c_void,
            );
            lua_pushboolean(L, tmp.b);
            return 1 as std::ffi::c_int;
        }
        4 => {
            g_object_get(
                object,
                (*p).name,
                &mut tmp.i as *mut gint,
                0 as *mut std::ffi::c_void,
            );
            lua_pushnumber(L, tmp.i as lua_Number);
            return 1 as std::ffi::c_int;
        }
        3 => {
            g_object_get(
                object,
                (*p).name,
                &mut tmp.f as *mut gfloat,
                0 as *mut std::ffi::c_void,
            );
            lua_pushnumber(L, tmp.f as lua_Number);
            return 1 as std::ffi::c_int;
        }
        2 => {
            g_object_get(
                object,
                (*p).name,
                &mut tmp.d as *mut gdouble,
                0 as *mut std::ffi::c_void,
            );
            lua_pushnumber(L, tmp.d);
            return 1 as std::ffi::c_int;
        }
        1 => {
            g_object_get(
                object,
                (*p).name,
                &mut tmp.c as *mut *mut gchar,
                0 as *mut std::ffi::c_void,
            );
            lua_pushstring(L, tmp.c);
            g_free(tmp.c as gpointer);
            return 1 as std::ffi::c_int;
        }
        5 => {
            g_object_get(
                object,
                (*p).name,
                &mut u as *mut *mut GUri,
                0 as *mut std::ffi::c_void,
            );
            tmp.c = if !u.is_null() {
                g_uri_to_string_partial(u, G_URI_HIDE_PASSWORD)
            } else {
                0 as *mut std::ffi::c_char
            };
            lua_pushstring(L, tmp.c);
            if !u.is_null() {
                g_uri_unref(u);
            }
            g_free(tmp.c as gpointer);
            return 1 as std::ffi::c_int;
        }
        _ => {}
    }
    g_assertion_message_expr(
        0 as *mut gchar,
        b"common/property.c\0" as *const u8 as *const std::ffi::c_char,
        75 as std::ffi::c_int,
        (*::core::mem::transmute::<&[u8; 17], &[std::ffi::c_char; 17]>(b"luaH_gobject_get\0"))
            .as_ptr(),
        0 as *const std::ffi::c_char,
    );
    return 1 as std::ffi::c_int;
}
unsafe extern "C" fn luaH_gobject_set(
    mut L: *mut lua_State,
    mut p: *mut property_t,
    mut vidx: gint,
    mut object: *mut GObject,
) -> gboolean {
    let mut u: *mut GUri = 0 as *mut GUri;
    let mut tmp: property_tmp_t = property_tmp_t { c: 0 as *mut gchar };
    let mut len = 0;
    let mut valid: gboolean = 0;
    match (*p).type_0 as std::ffi::c_uint {
        0 => {
            tmp.b = luaH_checkboolean(L, vidx);
            g_object_set(object, (*p).name, tmp.b, 0 as *mut std::ffi::c_void);
        }
        4 => {
            tmp.i = luaL_checknumber(L, vidx) as gint;
            g_object_set(object, (*p).name, tmp.i, 0 as *mut std::ffi::c_void);
        }
        3 => {
            tmp.f = luaL_checknumber(L, vidx) as gfloat;
            g_object_set(
                object,
                (*p).name,
                tmp.f as std::ffi::c_double,
                0 as *mut std::ffi::c_void,
            );
        }
        2 => {
            tmp.d = luaL_checknumber(L, vidx);
            g_object_set(object, (*p).name, tmp.d, 0 as *mut std::ffi::c_void);
        }
        1 => {
            if lua_type(L, vidx) == 0 as std::ffi::c_int {
                tmp.c = 0 as *mut gchar;
            } else {
                tmp.c = luaL_checklstring(L, vidx, 0 as *mut size_t) as *mut gchar;
            }
            g_object_set(object, (*p).name, tmp.c, 0 as *mut std::ffi::c_void);
        }
        5 => {
            if lua_type(L, vidx) == 0 as std::ffi::c_int {
                g_object_set(
                    object,
                    (*p).name,
                    0 as *mut std::ffi::c_void,
                    0 as *mut std::ffi::c_void,
                );
            } else {
                tmp.c = luaL_checklstring(L, vidx, &mut len) as *mut gchar;
                if len == 0
                    || !(g_strrstr(tmp.c, b"://\0" as *const u8 as *const std::ffi::c_char))
                        .is_null()
                {
                    tmp.c = g_strdup(tmp.c);
                } else {
                    tmp.c = g_strdup_printf(
                        b"http://%s\0" as *const u8 as *const std::ffi::c_char,
                        tmp.c,
                    );
                }
                u = g_uri_parse(
                    tmp.c,
                    (G_URI_FLAGS_HAS_PASSWORD as std::ffi::c_int
                        | G_URI_FLAGS_ENCODED_PATH as std::ffi::c_int
                        | G_URI_FLAGS_ENCODED_QUERY as std::ffi::c_int
                        | G_URI_FLAGS_ENCODED_FRAGMENT as std::ffi::c_int
                        | G_URI_FLAGS_SCHEME_NORMALIZE as std::ffi::c_int)
                        as GUriFlags,
                    0 as *mut *mut GError,
                );
                valid = (u.is_null()
                    || (g_strcmp0(
                        g_uri_get_scheme(u),
                        b"http\0" as *const u8 as *const std::ffi::c_char,
                    ) == 0
                        || g_strcmp0(
                            g_uri_get_scheme(u),
                            b"https\0" as *const u8 as *const std::ffi::c_char,
                        ) == 0)
                        && !(g_uri_get_host(u)).is_null()
                        && !(g_uri_get_path(u)).is_null())
                    as std::ffi::c_int;
                if valid != 0 {
                    g_object_set(object, (*p).name, u, 0 as *mut std::ffi::c_void);
                    g_free(tmp.c as gpointer);
                }
                if !u.is_null() {
                    g_uri_unref(u);
                }
                if valid == 0 {
                    lua_pushfstring(
                        L,
                        b"invalid uri: %s\0" as *const u8 as *const std::ffi::c_char,
                        tmp.c,
                    );
                    g_free(tmp.c as gpointer);
                    lua_error(L);
                }
            }
        }
        _ => {
            g_assertion_message_expr(
                0 as *mut gchar,
                b"common/property.c\0" as *const u8 as *const std::ffi::c_char,
                137 as std::ffi::c_int,
                (*::core::mem::transmute::<&[u8; 17], &[std::ffi::c_char; 17]>(
                    b"luaH_gobject_set\0",
                ))
                .as_ptr(),
                0 as *const std::ffi::c_char,
            );
        }
    }
    return (0 as std::ffi::c_int == 0) as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_gobject_index(
    mut L: *mut lua_State,
    mut props: *mut property_t,
    mut tok: luakit_token_t,
    mut object: *mut GObject,
) -> gint {
    let mut p: *mut property_t = props;
    while (*p).tok as u64 != 0 {
        if (*p).tok as std::ffi::c_uint == tok as std::ffi::c_uint {
            return luaH_gobject_get(L, p, object);
        }
        p = p.offset(1);
        p;
    }
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_gobject_newindex(
    mut L: *mut lua_State,
    mut props: *mut property_t,
    mut tok: luakit_token_t,
    mut vidx: gint,
    mut object: *mut GObject,
) -> gboolean {
    let mut p: *mut property_t = props;
    while (*p).tok as u64 != 0 {
        if (*p).tok as std::ffi::c_uint != tok as std::ffi::c_uint {
            p = p.offset(1);
            p;
        } else {
            if (*p).writable != 0 {
                return luaH_gobject_set(L, p, vidx, object);
            } else {
                _log(
                    LOG_LEVEL_warn,
                    b"common/property.c\0" as *const u8 as *const std::ffi::c_char,
                    &format!(
                        "read-only property: ({})",
                        CStr::from_ptr((*p).name).to_string_lossy()
                    ),
                );
            }
            break;
        }
    }
    return 0 as std::ffi::c_int;
}
