use glib_sys::{GPtrArray, g_ptr_array_add, g_ptr_array_new, g_strdup, gpointer};
use libc::{c_void, strlen};
use mlua::ffi::{lua_State, luaL_Reg, luaL_checklstring};
use mlua::lua_CFunction;

use crate::common::common;
use crate::common::luaclass::luaH_openlib;
use crate::gtypes::{gchar, guint};
use crate::ipc::{_ipc_header_t, IPC_TYPE_lua_require_module, ipc_endpoint_t, ipc_send};
use crate::ipc_common::clib::ipc::luaH_ipc_channel_new;

static mut required_web_modules: *mut GPtrArray = 0 as *const GPtrArray as *mut GPtrArray;
unsafe extern "C-unwind" fn luaH_require_web_module(mut L: *mut lua_State) -> std::ffi::c_int {
    let mut name = luaL_checklstring(L, -(1 as std::ffi::c_int), std::ptr::null_mut());
    g_ptr_array_add(required_web_modules, g_strdup(name) as gpointer);
    return luaH_ipc_channel_new(L);
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn web_module_load_modules_on_endpoint(mut ipc: *mut ipc_endpoint_t) {
    let mut i = 0 as std::ffi::c_int as std::ffi::c_uint;
    while i < (*required_web_modules).len {
        let mut module_name = *((*required_web_modules).pdata).offset(i as isize) as *const gchar;
        let mut header = {
            let mut init = _ipc_header_t {
                length: (strlen(module_name)).wrapping_add(1) as guint,
                type_0: IPC_TYPE_lua_require_module,
            };
            init
        };
        ipc_send(ipc, &mut header, module_name as *const std::ffi::c_void);
        i = i.wrapping_add(1);
        i;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn web_module_lib_setup(mut L: *mut lua_State) {
    let web_module_methods = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"__call\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_require_web_module,
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
            //         name: std::ptr::null(),
            //         func: None,
            //     };
            //     init
            // },
        ]
    };
    luaH_openlib(
        L,
        b"require_web_module\0" as *const u8 as *const std::ffi::c_char,
        &web_module_methods,
        &web_module_methods,
    );
    required_web_modules = g_ptr_array_new();
}
