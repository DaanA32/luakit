use gdk_sys::*;
use glib_sys::*;
use libc::*;
use mlua_sys::*;
use webkit2gtk::glib::ToVariant;
use webkit2gtk::glib::Variant;

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
use webkit2gtk::{ffi::*, glib::gobject_ffi::*};

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
    static mut web_module_methods: [luaL_Reg; 1] = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"__call\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_require_web_module,
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
        web_module_methods.as_ptr(),
        web_module_methods.as_ptr(),
    );
    required_web_modules = g_ptr_array_new();
}
