use std::ffi::CStr;

use gio_sys::{
    G_APPLICATION_DEFAULT_FLAGS, GActionEntry, GActionGroup, GActionMap, GApplication,
    GSimpleAction, g_action_group_activate_action, g_action_group_get_type,
    g_action_map_add_action_entries, g_action_map_get_type, g_application_get_application_id,
    g_application_get_is_registered, g_application_get_is_remote, g_application_get_type,
    g_application_id_is_valid, g_application_register,
};
use glib_sys::{
    G_VARIANT_TYPE_STRING, GFALSE, GTRUE, GType, GVariant, GVariantType, g_error_free,
    g_variant_get_string, g_variant_is_of_type, g_variant_new_string, gboolean, gpointer,
};
use gobject_sys::{GObject, GTypeInstance, g_object_unref, g_type_check_instance_cast};
use gtk_sys::{gtk_application_get_active_window, gtk_application_new, gtk_window_get_screen};
use libc::{c_void, intptr_t, strcmp};
use mlua_sys::{
    LUA_GLOBALSINDEX, LUA_MULTRET, lua_CFunction, lua_State, lua_createtable, lua_getfield,
    lua_gettable, lua_gettop, lua_pushboolean, lua_pushcclosure, lua_pushlightuserdata,
    lua_pushlstring, lua_pushstring, lua_pushvalue, lua_rawset, lua_setfield, lua_setmetatable,
    lua_settop, luaL_Reg, luaL_checklstring, luaL_error, luaL_newmetatable, luaL_register,
};

use crate::common::common;
use crate::common::luaclass::signal_h::{signal_new, signal_t};
use crate::common::luaclass::{
    lua_class_property_array_t, lua_class_t, luaH_class_add_signal, luaH_class_emit_signal,
    luaH_class_remove_signal,
};
use crate::common::luaobject::signal_object_emit;
use crate::globalconf::globalconf;
use crate::log::{_log, LOG_LEVEL_verbose, LOG_LEVEL_warn};

use crate::gtypes::{gchar, gint};

static mut unique_class: lua_class_t = lua_class_t {
    name: 0 as *const gchar,
    signals: 0 as *const signal_t as *mut signal_t,
    allocator: None,
    properties: 0 as *const lua_class_property_array_t as *mut lua_class_property_array_t,
    index_miss_property: None,
    newindex_miss_property: None,
};
#[inline]
unsafe extern "C-unwind" fn luaH_unique_class_emit_signal(mut L: *mut lua_State) -> gint {
    return luaH_class_emit_signal(
        L,
        &mut unique_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, std::ptr::null_mut()),
        lua_gettop(L) - 1 as std::ffi::c_int,
        LUA_MULTRET,
    );
}
#[inline]
unsafe extern "C-unwind" fn luaH_unique_class_remove_signal(mut L: *mut lua_State) -> gint {
    luaH_class_remove_signal(
        L,
        &mut unique_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, std::ptr::null_mut()),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn luaH_unique_class_add_signal(mut L: *mut lua_State) -> gint {
    luaH_class_add_signal(
        L,
        &mut unique_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, std::ptr::null_mut()),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn message_cb(
    mut UNUSED_a: *mut GSimpleAction,
    mut message_data: *mut GVariant,
    mut L: *mut lua_State,
) {
    if !message_data.is_null()
        && g_variant_is_of_type(
            message_data,
            G_VARIANT_TYPE_STRING.as_ptr() as *const GVariantType,
        ) == GTRUE
    {
        let mut text = g_variant_get_string(message_data, std::ptr::null_mut());
        lua_pushstring(L, text);
        let mut window = gtk_application_get_active_window(globalconf.application);
        if window.is_null() {
            _log(
                LOG_LEVEL_warn,
                b"clib/unique.c\0" as *const u8 as *const std::ffi::c_char,
                "It's not a window!!!",
            );
        }
        let mut screen = gtk_window_get_screen(window);
        lua_pushlightuserdata(L, screen as *mut std::ffi::c_void);
        signal_object_emit(
            L,
            unique_class.signals,
            b"message\0" as *const u8 as *const std::ffi::c_char,
            2 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
    }
}
unsafe extern "C-unwind" fn unique_is_registered() -> gboolean {
    if (globalconf.application).is_null() {
        return GFALSE;
    }
    if g_application_get_is_registered(g_type_check_instance_cast(
        globalconf.application as *mut GTypeInstance,
        g_application_get_type(),
    ) as *mut std::ffi::c_void as *mut GApplication)
        == 0
    {
        return GFALSE;
    }
    return GTRUE;
}
unsafe extern "C-unwind" fn luaH_unique_new(mut L: *mut lua_State) -> gint {
    let mut name = luaL_checklstring(L, 1 as std::ffi::c_int, std::ptr::null_mut());
    if g_application_id_is_valid(name) == 0 {
        return luaL_error(
            L,
            b"invalid application name\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if unique_is_registered() != 0 {
        let mut other = g_application_get_application_id(g_type_check_instance_cast(
            globalconf.application as *mut GTypeInstance,
            g_application_get_type(),
        ) as *mut std::ffi::c_void
            as *mut GApplication);
        if !(strcmp(
            name as *const std::ffi::c_char,
            other as *const std::ffi::c_char,
        ) == 0 as std::ffi::c_int)
        {
            luaL_error(
                L,
                b"GApplication '%s' already setup\0" as *const u8 as *const std::ffi::c_char,
                other,
            );
        } else {
            _log(
                LOG_LEVEL_verbose,
                b"clib/unique.c\0" as *const u8 as *const std::ffi::c_char,
                &format!(
                    "GApplication '{}' already setup",
                    CStr::from_ptr(name).to_string_lossy()
                ),
            );
        }
        return 0 as std::ffi::c_int;
    }
    let mut error = std::ptr::null_mut();
    if (globalconf.application).is_null() {
        globalconf.application = gtk_application_new(name, G_APPLICATION_DEFAULT_FLAGS);
    }
    g_application_register(
        g_type_check_instance_cast(
            globalconf.application as *mut GTypeInstance,
            g_application_get_type(),
        ) as *mut std::ffi::c_void as *mut GApplication,
        std::ptr::null_mut(),
        &mut error,
    );
    if !error.is_null() {
        luaL_error(
            L,
            b"unable to register GApplication: %s\0" as *const u8 as *const std::ffi::c_char,
            (*error).message,
        );
        g_error_free(error);
        g_object_unref(g_type_check_instance_cast(
            globalconf.application as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject);
        globalconf.application = std::ptr::null_mut();
        return 0 as std::ffi::c_int;
    }
    let entries: [GActionEntry; 1] = [{
        let mut init = GActionEntry {
            name: b"message\0" as *const u8 as *const std::ffi::c_char,
            activate: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(*mut GSimpleAction, *mut GVariant, *mut lua_State) -> (),
                >,
                Option<unsafe extern "C" fn(*mut GSimpleAction, *mut GVariant, gpointer) -> ()>,
            >(Some(
                message_cb
                    as unsafe extern "C" fn(
                        *mut GSimpleAction,
                        *mut GVariant,
                        *mut lua_State,
                    ) -> (),
            )),
            parameter_type: b"s\0" as *const u8 as *const std::ffi::c_char,
            state: 0 as *const gchar,
            change_state: None,
            padding: [0; 3],
        };
        init
    }];
    g_action_map_add_action_entries(
        g_type_check_instance_cast(
            globalconf.application as *mut GTypeInstance,
            g_action_map_get_type(),
        ) as *mut std::ffi::c_void as *mut GActionMap,
        entries.as_ptr(),
        (::core::mem::size_of::<[GActionEntry; 1]>() as std::ffi::c_ulong)
            .wrapping_div(::core::mem::size_of::<GActionEntry>() as std::ffi::c_ulong)
            as gint,
        L as gpointer,
    );
    return 0 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_unique_is_running(mut L: *mut lua_State) -> gint {
    if unique_is_registered() == 0 {
        luaL_error(
            L,
            b"GApplication is not registered\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut running = g_application_get_is_remote(g_type_check_instance_cast(
        globalconf.application as *mut GTypeInstance,
        g_application_get_type(),
    ) as *mut std::ffi::c_void
        as *mut GApplication);
    lua_pushboolean(L, running);
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_unique_send_message(mut L: *mut lua_State) -> gint {
    if unique_is_registered() == 0 {
        luaL_error(
            L,
            b"GApplication is not registered\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if g_application_get_is_remote(g_type_check_instance_cast(
        globalconf.application as *mut GTypeInstance,
        g_application_get_type(),
    ) as *mut std::ffi::c_void as *mut GApplication)
        == 0
    {
        luaL_error(
            L,
            b"no other instances running\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut text = g_variant_new_string(luaL_checklstring(
        L,
        1 as std::ffi::c_int,
        std::ptr::null_mut(),
    ));
    g_action_group_activate_action(
        g_type_check_instance_cast(
            globalconf.application as *mut GTypeInstance,
            g_action_group_get_type(),
        ) as *mut std::ffi::c_void as *mut GActionGroup,
        b"message\0" as *const u8 as *const std::ffi::c_char,
        text,
    );
    return 0 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_open_luakit_unique(
    mut L: *mut lua_State,
    mut methods: *const luaL_Reg,
    mut meta: *const luaL_Reg,
) {
    luaL_newmetatable(L, b"unique\0" as *const u8 as *const std::ffi::c_char);
    lua_pushvalue(L, -(1 as std::ffi::c_int));
    lua_setfield(
        L,
        -(2 as std::ffi::c_int),
        b"__index\0" as *const u8 as *const std::ffi::c_char,
    );
    luaL_register(L, std::ptr::null(), meta);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    luaL_register(L, std::ptr::null(), methods);
    lua_getfield(
        L,
        LUA_GLOBALSINDEX,
        b"package\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_getfield(
        L,
        -(1 as std::ffi::c_int),
        b"loaded\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_pushvalue(L, -(3 as std::ffi::c_int));
    lua_setfield(
        L,
        -(2 as std::ffi::c_int),
        b"luakit.unique\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_settop(L, -(2 as std::ffi::c_int) - 1 as std::ffi::c_int);
    lua_getfield(
        L,
        LUA_GLOBALSINDEX,
        b"luakit\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_pushlstring(
        L,
        b"unique\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 7]>())
            .wrapping_div(::core::mem::size_of::<std::ffi::c_char>())
            .wrapping_sub(1),
    );
    lua_pushvalue(L, -(3 as std::ffi::c_int));
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    lua_pushvalue(L, -(1 as std::ffi::c_int));
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    lua_settop(L, -(2 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
static mut warned: gboolean = GFALSE;
unsafe extern "C-unwind" fn luaH_unique_proxy_index(mut L: *mut lua_State) -> std::ffi::c_int {
    if warned == 0 {
        warned = GTRUE;
        _log(
            LOG_LEVEL_warn,
            b"clib/unique.c\0" as *const u8 as *const std::ffi::c_char,
            "the unique library has been moved to luakit.unique",
        );
        _log(
            LOG_LEVEL_warn,
            b"clib/unique.c\0" as *const u8 as *const std::ffi::c_char,
            "this compatibility wrapper will be removed in a future version",
        );
        _log(
            LOG_LEVEL_warn,
            b"clib/unique.c\0" as *const u8 as *const std::ffi::c_char,
            "you should remove the two `if unique then ... end` blocks from your rc.lua",
        );
        _log(
            LOG_LEVEL_warn,
            b"clib/unique.c\0" as *const u8 as *const std::ffi::c_char,
            "then, at the start of your rc.lua, add `require \"unique_instance\"`",
        );
    }
    lua_getfield(
        L,
        LUA_GLOBALSINDEX,
        b"luakit\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_getfield(
        L,
        -(1 as std::ffi::c_int),
        b"unique\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_pushvalue(L, 2 as std::ffi::c_int);
    lua_gettable(L, -(2 as std::ffi::c_int));
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_open_unique_proxy(mut L: *mut lua_State) {
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_pushcclosure(L, luaH_unique_proxy_index, 0 as std::ffi::c_int);
    lua_setfield(
        L,
        -(2 as std::ffi::c_int),
        b"__index\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    lua_getfield(
        L,
        LUA_GLOBALSINDEX,
        b"package\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_getfield(
        L,
        -(1 as std::ffi::c_int),
        b"loaded\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_pushvalue(L, -(3 as std::ffi::c_int));
    lua_setfield(
        L,
        -(2 as std::ffi::c_int),
        b"unique\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_settop(L, -(2 as std::ffi::c_int) - 1 as std::ffi::c_int);
    lua_setfield(
        L,
        LUA_GLOBALSINDEX,
        b"unique\0" as *const u8 as *const std::ffi::c_char,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn unique_lib_setup(mut L: *mut lua_State) {
    static mut unique_lib: [luaL_Reg; 6] = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"add_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_unique_class_add_signal,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_unique_class_remove_signal,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"emit_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_unique_class_emit_signal,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"new\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_unique_new,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"send_message\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_unique_send_message,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"is_running\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_unique_is_running,
                };
                init
            },
            // {
            //     let mut init = luaL_Reg {
            //         name: std::ptr::null(),
            //         func: ::core::mem::transmute::<libc::intptr_t, lua_CFunction>(0),
            //     };
            //     init
            // },
        ]
    };
    unique_class.signals = signal_new();
    luaH_open_luakit_unique(L, unique_lib.as_ptr(), unique_lib.as_ptr());
    luaH_open_unique_proxy(L);
}
