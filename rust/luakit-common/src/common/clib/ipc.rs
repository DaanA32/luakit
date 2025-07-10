use glib_sys::{g_free, g_strdup, gpointer};
use libc::{c_void, memset, size_t};
use mlua_sys::{
    lua_State, lua_createtable, lua_gettop, lua_newuserdata, lua_pushstring, lua_pushvalue,
    lua_rawset, lua_remove, lua_setfenv, lua_setmetatable, luaL_Reg, luaL_checklstring,
};

use crate::clib::ipc::ipc_channel_send;
use crate::common::common;
use crate::common::luaclass::signal_h::{signal_new, signal_t};
use crate::common::luaclass::{
    lua_class_allocator_t, lua_class_property_array_t, lua_class_t, luaH_checkudata,
    luaH_class_add_signal, luaH_class_emit_signal, luaH_class_new, luaH_class_remove_signal,
    luaH_class_setup,
};
use crate::common::luaobject::{
    luaH_object_add_signal_simple, luaH_object_emit_signal_simple, luaH_object_gc,
    luaH_object_remove_signal_simple, luaH_object_remove_signals_simple, luaH_object_tostring,
    luaH_settype,
};
use crate::common::luauniq::{luaH_uniq_add, luaH_uniq_get};

use crate::gtypes::{gchar, gint};

pub struct ipc_channel_t {
    pub signals: *mut signal_t,
    pub name: *mut std::ffi::c_char,
}

static mut ipc_channel_class: lua_class_t = lua_class_t {
    name: 0 as *const gchar,
    signals: 0 as *const signal_t as *mut signal_t,
    allocator: None,
    properties: 0 as *const lua_class_property_array_t as *mut lua_class_property_array_t,
    index_miss_property: None,
    newindex_miss_property: None,
};
#[inline]
unsafe extern "C-unwind" fn luaH_ipc_channel_class_add_signal(mut L: *mut lua_State) -> gint {
    luaH_class_add_signal(
        L,
        &mut ipc_channel_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn ipc_channel_new(mut L: *mut lua_State) -> *mut ipc_channel_t {
    let mut p: *mut ipc_channel_t =
        lua_newuserdata(L, ::core::mem::size_of::<ipc_channel_t>()) as *mut ipc_channel_t;
    memset(
        p as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        (::core::mem::size_of::<ipc_channel_t>()).wrapping_mul(1),
    );
    (*p).signals = signal_new();
    luaH_settype(L, &mut ipc_channel_class);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    lua_setfenv(L, -(2 as std::ffi::c_int));
    lua_pushvalue(L, -(1 as std::ffi::c_int));
    luaH_class_emit_signal(
        L,
        &mut ipc_channel_class,
        b"new\0" as *const u8 as *const std::ffi::c_char,
        1 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    return p;
}
#[inline]
unsafe extern "C-unwind" fn luaH_ipc_channel_class_remove_signal(mut L: *mut lua_State) -> gint {
    luaH_class_remove_signal(
        L,
        &mut ipc_channel_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn luaH_ipc_channel_class_emit_signal(mut L: *mut lua_State) -> gint {
    return luaH_class_emit_signal(
        L,
        &mut ipc_channel_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        lua_gettop(L) - 1 as std::ffi::c_int,
        -(1 as std::ffi::c_int),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_check_ipc_channel(
    mut L: *mut lua_State,
    mut idx: gint,
) -> *mut ipc_channel_t {
    return luaH_checkudata(L, idx, &mut ipc_channel_class) as *mut ipc_channel_t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_ipc_channel_new(mut L: *mut lua_State) -> gint {
    let mut name: *const std::ffi::c_char =
        luaL_checklstring(L, -(1 as std::ffi::c_int), 0 as *mut size_t);
    if luaH_uniq_get(
        L,
        b"luakit.registry.ipc_channel\0" as *const u8 as *const std::ffi::c_char,
        -(1 as std::ffi::c_int),
    ) != 0
    {
        return 1 as std::ffi::c_int;
    }
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    luaH_class_new(L, &mut ipc_channel_class);
    lua_remove(L, -(2 as std::ffi::c_int));
    let mut ipc_channel: *mut ipc_channel_t = luaH_check_ipc_channel(L, -(1 as std::ffi::c_int));
    (*ipc_channel).name = g_strdup(name);
    luaH_uniq_add(
        L,
        b"luakit.registry.ipc_channel\0" as *const u8 as *const std::ffi::c_char,
        -(2 as std::ffi::c_int),
        -(1 as std::ffi::c_int),
    );
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_ipc_channel_gc(mut L: *mut lua_State) -> gint {
    let mut ipc_channel: *mut ipc_channel_t = luaH_check_ipc_channel(L, -(1 as std::ffi::c_int));
    g_free((*ipc_channel).name as gpointer);
    return luaH_object_gc(L);
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn ipc_channel_class_setup(mut L: *mut lua_State) {
    static mut ipc_channel_methods: [luaL_Reg; 4] = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"add_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_ipc_channel_class_add_signal,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_ipc_channel_class_remove_signal,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"emit_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_ipc_channel_class_emit_signal,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__call\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_ipc_channel_new,
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
    static mut ipc_channel_meta: [luaL_Reg; 7] = unsafe {
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
                    name: b"emit_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: ipc_channel_send,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__gc\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_ipc_channel_gc,
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
        &mut ipc_channel_class,
        b"ipc_channel\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<unsafe extern "C-unwind" fn(*mut lua_State) -> *mut ipc_channel_t>,
            lua_class_allocator_t,
        >(Some(
            ipc_channel_new as unsafe extern "C-unwind" fn(*mut lua_State) -> *mut ipc_channel_t,
        )),
        None,
        None,
        ipc_channel_methods.as_ptr(),
        ipc_channel_meta.as_ptr(),
    );
    lua_pushstring(
        L,
        b"luakit.registry.ipc_channel\0" as *const u8 as *const std::ffi::c_char,
    );
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_rawset(L, -(10000 as std::ffi::c_int));
}
