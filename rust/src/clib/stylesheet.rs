use glib_sys::{g_free, g_strdup, gboolean, gpointer};
use libc::memset;
use mlua::ffi::*;
use mlua::ffi::{
    LUA_MULTRET, lua_State, lua_createtable, lua_gettop, lua_newuserdata, lua_pushstring,
    lua_pushvalue, lua_setmetatable, luaL_Reg, luaL_checklstring,
};
use webkit2gtk_sys::*;
pub mod stylesheet_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct lstylesheet_t {
        pub signals: *mut signal_t,
        pub stylesheet: *mut WebKitUserStyleSheet,
        pub source: *mut gchar,
    }
    use webkit2gtk_sys::*;

    use crate::{common::luaclass::signal_h::signal_t, gtypes::gchar, widgets::widget_t};

    unsafe extern "C-unwind" {
        pub fn webview_stylesheets_regenerate_stylesheet(
            w: *mut widget_t,
            stylesheet: *mut lstylesheet_t,
        );
    }
}
use crate::clib::luakit::{luaH_class_index_miss_property, luaH_class_newindex_miss_property};
pub use crate::clib::stylesheet::stylesheet_h::{
    lstylesheet_t, webview_stylesheets_regenerate_stylesheet,
};
use crate::common::luaclass::signal_h::{signal_new, signal_t};
use crate::common::luaclass::{
    lua_class_allocator_t, lua_class_property_array_t, lua_class_propfunc_t, lua_class_t,
    luaH_checkudata, luaH_class_add_property, luaH_class_add_signal, luaH_class_emit_signal,
    luaH_class_index, luaH_class_new, luaH_class_newindex, luaH_class_remove_signal,
    luaH_class_setup,
};
use crate::common::luaobject::{
    luaH_object_add_signal_simple, luaH_object_emit_signal_simple, luaH_object_gc,
    luaH_object_remove_signal_simple, luaH_object_remove_signals_simple, luaH_object_tostring,
    luaH_settype,
};
use crate::common::tokenize::L_TK_SOURCE;
use crate::globalconf::globalconf;
use crate::gtypes::{gchar, gint};
use crate::widgets::widget_t;

unsafe extern "C-unwind" {
    pub fn webview_stylesheet_set_enabled(
        w: *mut widget_t,
        stylesheet: *mut lstylesheet_t,
        enable: gboolean,
    ) -> std::ffi::c_int;
}
static mut stylesheet_class: lua_class_t = lua_class_t {
    name: 0 as *const gchar,
    signals: 0 as *const signal_t as *mut signal_t,
    allocator: None,
    properties: 0 as *const lua_class_property_array_t as *mut lua_class_property_array_t,
    index_miss_property: None,
    newindex_miss_property: None,
};
#[inline]
unsafe extern "C-unwind" fn luaH_stylesheet_class_emit_signal(mut L: *mut lua_State) -> gint {
    return luaH_class_emit_signal(
        L,
        &mut stylesheet_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, std::ptr::null_mut()),
        lua_gettop(L) - 1 as std::ffi::c_int,
        LUA_MULTRET,
    );
}
#[inline]
unsafe extern "C-unwind" fn luaH_stylesheet_class_remove_signal(mut L: *mut lua_State) -> gint {
    luaH_class_remove_signal(
        L,
        &mut stylesheet_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, std::ptr::null_mut()),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn luaH_stylesheet_class_add_signal(mut L: *mut lua_State) -> gint {
    luaH_class_add_signal(
        L,
        &mut stylesheet_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, std::ptr::null_mut()),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn stylesheet_new(mut L: *mut lua_State) -> *mut lstylesheet_t {
    let mut p = lua_newuserdata(L, ::core::mem::size_of::<lstylesheet_t>()) as *mut lstylesheet_t;
    memset(
        p as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        (::core::mem::size_of::<lstylesheet_t>()).wrapping_mul(1),
    );
    (*p).signals = signal_new();
    luaH_settype(L, &mut stylesheet_class);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    println!("lua_setfenv(L, -(2 as std::ffi::c_int));");
    lua_pushvalue(L, -(1 as std::ffi::c_int));
    luaH_class_emit_signal(
        L,
        &mut stylesheet_class,
        b"new\0" as *const u8 as *const std::ffi::c_char,
        1 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    return p;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_checkstylesheet(
    mut L: *mut lua_State,
    mut idx: gint,
) -> gpointer {
    return luaH_checkudata(L, idx, &mut stylesheet_class);
}
unsafe extern "C-unwind" fn luaH_stylesheet_gc(mut L: *mut lua_State) -> gint {
    let mut stylesheet = luaH_checkstylesheet(L, 1 as std::ffi::c_int) as *mut lstylesheet_t;
    if !((*stylesheet).stylesheet).is_null() && !(globalconf.webviews).is_null() {
        let mut i = 0 as std::ffi::c_int as std::ffi::c_uint;
        while i < (*globalconf.webviews).len {
            let mut w = *((*globalconf.webviews).pdata).offset(i as isize) as *mut widget_t;
            webview_stylesheet_set_enabled(w, stylesheet, 0 as gboolean);
            i = i.wrapping_add(1);
            i;
        }
        webkit_user_style_sheet_unref((*stylesheet).stylesheet);
    }
    g_free((*stylesheet).source as gpointer);
    return luaH_object_gc(L);
}
unsafe extern "C-unwind" fn luaH_stylesheet_new(mut L: *mut lua_State) -> gint {
    luaH_class_new(L, &mut stylesheet_class);
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn regenerate_stylesheet(mut stylesheet: *mut lstylesheet_t) {
    let mut old = (*stylesheet).stylesheet;
    if !old.is_null() {
        webkit_user_style_sheet_unref(old);
    }
    (*stylesheet).stylesheet = webkit_user_style_sheet_new(
        (*stylesheet).source,
        WEBKIT_USER_CONTENT_INJECT_ALL_FRAMES,
        WEBKIT_USER_STYLE_LEVEL_USER,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );
    if !old.is_null() && !(globalconf.webviews).is_null() {
        let mut i = 0 as std::ffi::c_int as std::ffi::c_uint;
        while i < (*globalconf.webviews).len {
            let mut w = *((*globalconf.webviews).pdata).offset(i as isize) as *mut widget_t;
            webview_stylesheets_regenerate_stylesheet(w, stylesheet);
            i = i.wrapping_add(1);
            i;
        }
    }
}
unsafe extern "C-unwind" fn luaH_stylesheet_set_source(
    mut L: *mut lua_State,
    mut stylesheet: *mut lstylesheet_t,
) -> std::ffi::c_int {
    let mut new_source = luaL_checklstring(L, -(1 as std::ffi::c_int), std::ptr::null_mut());
    g_free((*stylesheet).source as gpointer);
    (*stylesheet).source = g_strdup(new_source);
    regenerate_stylesheet(stylesheet);
    return 0 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_stylesheet_get_source(
    mut L: *mut lua_State,
    mut stylesheet: *mut lstylesheet_t,
) -> std::ffi::c_int {
    lua_pushstring(L, (*stylesheet).source);
    return 1 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn stylesheet_class_setup(mut L: *mut lua_State) {
    let stylesheet_methods = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"add_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_stylesheet_class_add_signal,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_stylesheet_class_remove_signal,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"emit_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_stylesheet_class_emit_signal,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__call\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_stylesheet_new,
                };
                init
            },
            // {
            //     let mut init = luaL_Reg {
            //         name: std::ptr::null_mut(),
            //         func: None,
            //     };
            //     init
            // },
        ]
    };
    let stylesheet_meta = unsafe {
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
                    name: b"__gc\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_stylesheet_gc,
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
            //         name: std::ptr::null_mut(),
            //         func: None, // func: ::core::mem::transmute::<libc::intptr_t, Function>(
            //                     //     NULL_1 as libc::intptr_t,
            //                     // ),
            //     };
            //     init
            // },
        ]
    };
    luaH_class_setup(
        L,
        &mut stylesheet_class,
        b"stylesheet\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<unsafe extern "C-unwind" fn(*mut lua_State) -> *mut lstylesheet_t>,
            lua_class_allocator_t,
        >(Some(
            stylesheet_new as unsafe extern "C-unwind" fn(*mut lua_State) -> *mut lstylesheet_t,
        )),
        Some(luaH_class_index_miss_property),
        Some(luaH_class_newindex_miss_property),
        stylesheet_methods.as_ptr(),
        stylesheet_meta.as_ptr(),
    );
    luaH_class_add_property(
        &mut stylesheet_class,
        L_TK_SOURCE,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C-unwind" fn(*mut lua_State, *mut lstylesheet_t) -> std::ffi::c_int,
            >,
            lua_class_propfunc_t,
        >(Some(
            luaH_stylesheet_set_source
                as unsafe extern "C-unwind" fn(
                    *mut lua_State,
                    *mut lstylesheet_t,
                ) -> std::ffi::c_int,
        )),
        ::core::mem::transmute::<
            Option<
                unsafe extern "C-unwind" fn(*mut lua_State, *mut lstylesheet_t) -> std::ffi::c_int,
            >,
            lua_class_propfunc_t,
        >(Some(
            luaH_stylesheet_get_source
                as unsafe extern "C-unwind" fn(
                    *mut lua_State,
                    *mut lstylesheet_t,
                ) -> std::ffi::c_int,
        )),
        ::core::mem::transmute::<
            Option<
                unsafe extern "C-unwind" fn(*mut lua_State, *mut lstylesheet_t) -> std::ffi::c_int,
            >,
            lua_class_propfunc_t,
        >(Some(
            luaH_stylesheet_set_source
                as unsafe extern "C-unwind" fn(
                    *mut lua_State,
                    *mut lstylesheet_t,
                ) -> std::ffi::c_int,
        )),
    );
}
