use gdk_sys::*;
use glib_sys::*;
use gtk_sys::*;
use libc::*;
use mlua_sys::*;

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

unsafe extern "C-unwind" fn str_chomp_slashes(mut path: *mut gchar) {
    if path.is_null() {
        return;
    }
    let mut last = (strlen(path)).wrapping_sub(1) as gint;
    while last > 0 as std::ffi::c_int
        && *path.offset(last as isize) as std::ffi::c_int == '/' as i32
    {
        let fresh0 = last;
        last = last - 1;
        *path.offset(fresh0 as isize) = '\0' as i32 as gchar;
    }
}
unsafe extern "C-unwind" fn luaH_push_path(
    mut L: *mut lua_State,
    mut path: *const gchar,
) -> std::ffi::c_int {
    let mut p = g_strdup(path);
    str_chomp_slashes(p);
    lua_pushstring(L, p);
    g_free(p as gpointer);
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_push_path_array(
    mut L: *mut lua_State,
    mut paths: *const *const gchar,
) -> std::ffi::c_int {
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    let mut n = 0;
    while !(*paths.offset(n as isize)).is_null() {
        luaH_push_path(L, *paths.offset(n as isize));
        lua_rawseti(L, -(2), n + 1);
        n += 1;
        n;
    }
    return 1 as std::ffi::c_int;
}
unsafe extern "C-unwind" fn luaH_xdg_index(mut L: *mut lua_State) -> gint {
    if lua_isstring(L, 2 as std::ffi::c_int) == 0 {
        return 0 as std::ffi::c_int;
    }
    match l_tokenize(lua_tolstring(L, 2, std::ptr::null_mut())) as std::ffi::c_uint {
        16 => {
            luaH_push_path(L, g_get_user_cache_dir());
            return 1 as std::ffi::c_int;
        }
        32 => {
            luaH_push_path(L, g_get_user_config_dir());
            return 1 as std::ffi::c_int;
        }
        42 => {
            luaH_push_path(L, g_get_user_data_dir());
            return 1 as std::ffi::c_int;
        }
        48 => {
            luaH_push_path(L, g_get_user_special_dir(G_USER_DIRECTORY_DESKTOP));
            return 1 as std::ffi::c_int;
        }
        53 => {
            luaH_push_path(L, g_get_user_special_dir(G_USER_DIRECTORY_DOCUMENTS));
            return 1 as std::ffi::c_int;
        }
        54 => {
            luaH_push_path(L, g_get_user_special_dir(G_USER_DIRECTORY_DOWNLOAD));
            return 1 as std::ffi::c_int;
        }
        151 => {
            luaH_push_path(L, g_get_user_special_dir(G_USER_DIRECTORY_MUSIC));
            return 1 as std::ffi::c_int;
        }
        165 => {
            luaH_push_path(L, g_get_user_special_dir(G_USER_DIRECTORY_PICTURES));
            return 1 as std::ffi::c_int;
        }
        175 => {
            luaH_push_path(L, g_get_user_special_dir(G_USER_DIRECTORY_PUBLIC_SHARE));
            return 1 as std::ffi::c_int;
        }
        236 => {
            luaH_push_path(L, g_get_user_special_dir(G_USER_DIRECTORY_TEMPLATES));
            return 1 as std::ffi::c_int;
        }
        252 => {
            luaH_push_path(L, g_get_user_special_dir(G_USER_DIRECTORY_VIDEOS));
            return 1 as std::ffi::c_int;
        }
        234 => return luaH_push_path_array(L, g_get_system_data_dirs()),
        233 => return luaH_push_path_array(L, g_get_system_config_dirs()),
        _ => {}
    }
    return 0 as std::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn xdg_lib_setup(mut L: *mut lua_State) {
    static mut xdg_lib: [luaL_Reg; 1] = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"__index\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_xdg_index,
                };
                init
            },
            // {
            //     let mut init = luaL_Reg {
            //         name: NULL as *const std::ffi::c_char,
            //         func: ::core::mem::transmute::<libc::intptr_t, lua_CFunction>(
            //             NULL as libc::intptr_t,
            //         ),
            //     };
            //     init
            // },
        ]
    };
    luaH_openlib(
        L,
        b"xdg\0" as *const u8 as *const std::ffi::c_char,
        xdg_lib.as_ptr(),
        xdg_lib.as_ptr(),
    );
}
