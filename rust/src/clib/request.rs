use gdk_sys::*;
use gio_sys::*;
use glib_sys::*;
use gobject_sys::*;
use libc::*;
use mlua::ffi::*;

use crate::clib::luakit::*;
use crate::common::clib::luakit::*;
use crate::common::common;
use crate::common::luaclass::signal_h::*;
use crate::common::luaclass::*;
use crate::common::luah::*;
use crate::common::luaobject::*;
use crate::common::luauniq::*;
use crate::common::tokenize::*;
use crate::globalconf::*;
use crate::log::*;
use crate::luah::*;
use crate::web_context::*;

use crate::gtypes::*;
use webkit2gtk_sys::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct request_t {
    pub signals: *mut signal_t,
    pub request: *mut WebKitURISchemeRequest,
    pub finished: gboolean,
}
static mut request_class: lua_class_t = lua_class_t {
    name: 0 as *const gchar,
    signals: 0 as *const signal_t as *mut signal_t,
    allocator: None,
    properties: 0 as *const lua_class_property_array_t as *mut lua_class_property_array_t,
    index_miss_property: None,
    newindex_miss_property: None,
};
#[inline]
unsafe extern "C-unwind" fn luaH_request_class_emit_signal(mut L: *mut lua_State) -> gint {
    return luaH_class_emit_signal(
        L,
        &mut request_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        lua_gettop(L) - 1 as std::ffi::c_int,
        -(1 as std::ffi::c_int),
    );
}
#[inline]
unsafe extern "C-unwind" fn luaH_request_class_add_signal(mut L: *mut lua_State) -> gint {
    luaH_class_add_signal(
        L,
        &mut request_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn luaH_request_class_remove_signal(mut L: *mut lua_State) -> gint {
    luaH_class_remove_signal(
        L,
        &mut request_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn request_new(mut L: *mut lua_State) -> *mut request_t {
    let mut p: *mut request_t =
        lua_newuserdata(L, ::core::mem::size_of::<request_t>()) as *mut request_t;
    memset(
        p as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        (::core::mem::size_of::<request_t>()).wrapping_mul(1),
    );
    (*p).signals = signal_new();
    luaH_settype(L, &mut request_class);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    lua_setfenv(L, -(2 as std::ffi::c_int));
    lua_pushvalue(L, -(1 as std::ffi::c_int));
    luaH_class_emit_signal(
        L,
        &mut request_class,
        b"new\0" as *const u8 as *const std::ffi::c_char,
        1 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    return p;
}
unsafe extern "C-unwind" fn luaH_check_request(
    mut L: *mut lua_State,
    mut idx: gint,
) -> *mut request_t {
    let mut request: *mut request_t = luaH_checkudata(L, idx, &mut request_class) as *mut request_t;
    if (*request).finished != 0 {
        luaL_error(
            L,
            b"request has already been finished\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    return request;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_request_push_uri_scheme_request(
    mut L: *mut lua_State,
    mut r: *mut WebKitURISchemeRequest,
) -> gint {
    if luaH_uniq_get_ptr(
        L,
        b"luakit.uniq.registry.request\0" as *const u8 as *const std::ffi::c_char,
        r as gpointer,
    ) != 0
    {
        return 1 as std::ffi::c_int;
    }
    let mut request: *mut request_t = request_new(L);
    (*request).request = g_object_ref(r as *mut GObject) as *mut WebKitURISchemeRequest;
    (*request).finished = 0 as std::ffi::c_int;
    luaH_uniq_add_ptr(
        L,
        b"luakit.uniq.registry.request\0" as *const u8 as *const std::ffi::c_char,
        r as gpointer,
        -(1 as std::ffi::c_int),
    );
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_request_finish(mut L: *mut lua_State) -> gint {
    let mut length: size_t = 0;
    let mut data: *const gchar = 0 as *const gchar;
    let mut mime: *const gchar = 0 as *const gchar;
    let mut stream: *mut GInputStream = 0 as *mut GInputStream;
    let mut request: *mut request_t = luaH_check_request(L, 1 as std::ffi::c_int);
    (*request).finished = (0 as std::ffi::c_int == 0) as std::ffi::c_int;
    let mut error_message: *const gchar = 0 as *const gchar;
    if lua_isstring(L, 2 as std::ffi::c_int) == 0 {
        error_message = b"data isn't a string\0" as *const u8 as *const std::ffi::c_char;
    } else {
        if lua_type(L, 3 as std::ffi::c_int) == -(1 as std::ffi::c_int) {
            lua_pushnil(L);
        }
        if lua_type(L, 3 as std::ffi::c_int) != 4 as std::ffi::c_int
            && lua_type(L, 3 as std::ffi::c_int) != 0 as std::ffi::c_int
        {
            error_message =
                b"MIME type isn't a string or nil\0" as *const u8 as *const std::ffi::c_char;
        } else {
            length = 0;
            data = lua_tolstring(L, 2 as std::ffi::c_int, &mut length);
            let ref mut fresh0 = lua_tolstring(L, 3 as std::ffi::c_int, 0 as *mut size_t);
            mime = if !(*fresh0).is_null() {
                *fresh0
            } else {
                b"text/html\0" as *const u8 as *const std::ffi::c_char
            };
            stream = g_memory_input_stream_new_from_data(
                g_memdup2(data as *const c_void, length) as *mut u8,
                length as ssize_t,
                Some(g_free),
            );
            webkit_uri_scheme_request_finish((*request).request, stream, length as gint64, mime);
            g_object_unref(stream as *mut GObject);
            return 0 as std::ffi::c_int;
        }
    }
    if !error_message.is_null() {
    } else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"clib/request.c\0" as *const u8 as *const std::ffi::c_char,
            94 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 20], &[std::ffi::c_char; 20]>(
                b"luaH_request_finish\0",
            ))
            .as_ptr(),
            b"error_message\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut error: *mut GError = g_error_new_literal(
        g_quark_from_static_string(b"luakit\0" as *const u8 as *const std::ffi::c_char),
        0 as std::ffi::c_int,
        error_message,
    );
    webkit_uri_scheme_request_finish_error((*request).request, error);
    return luaL_error(L, error_message);
}
unsafe extern "C-unwind" fn luaH_request_get_finished(
    mut L: *mut lua_State,
    mut request: *mut request_t,
) -> std::ffi::c_int {
    lua_pushboolean(L, (*request).finished);
    return 1 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn request_class_setup(mut L: *mut lua_State) {
    let request_methods = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"add_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_request_class_add_signal,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_request_class_remove_signal,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"emit_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_request_class_emit_signal,
                };
                init
            },
            // {
            //     let mut init = luaL_Reg {
            //         name: 0 as *const std::ffi::c_char,
            //         func: None,
            //     };
            //     init
            // },
        ]
    };
    let request_meta = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"__tostring\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_object_tostring,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"add_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_object_add_signal_simple,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_object_remove_signal_simple,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signals\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_object_remove_signals_simple,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"emit_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_object_emit_signal_simple,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__index\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_class_index,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__newindex\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_class_newindex,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"finish\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_request_finish,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: 0 as *const std::ffi::c_char,
                    func: core::mem::transmute::<libc::intptr_t, lua_CFunction>(
                        0 as libc::intptr_t,
                    ),
                };
                init
            },
            // {
            //     let mut init = luaL_Reg {
            //         name: 0 as *const std::ffi::c_char,
            //         func: None,
            //     };
            //     init
            // },
        ]
    };
    luaH_class_setup(
        L,
        &mut request_class,
        b"request\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<unsafe extern "C-unwind" fn(*mut lua_State) -> *mut request_t>,
            lua_class_allocator_t,
        >(Some(
            request_new as unsafe extern "C-unwind" fn(*mut lua_State) -> *mut request_t,
        )),
        None,
        None,
        request_methods.as_ptr(),
        request_meta.as_ptr(),
    );
    luaH_class_add_property(
        &mut request_class,
        L_TK_FINISHED,
        None,
        ::core::mem::transmute::<
            Option<unsafe extern "C-unwind" fn(*mut lua_State, *mut request_t) -> std::ffi::c_int>,
            lua_class_propfunc_t,
        >(Some(
            luaH_request_get_finished
                as unsafe extern "C-unwind" fn(*mut lua_State, *mut request_t) -> std::ffi::c_int,
        )),
        None,
    );
    luaH_uniq_setup(
        L,
        b"luakit.uniq.registry.request\0" as *const u8 as *const std::ffi::c_char,
        b"\0" as *const u8 as *const std::ffi::c_char,
    );
}
