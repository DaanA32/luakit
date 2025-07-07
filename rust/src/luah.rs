use ::libc;
pub mod __stddef_size_t_h {
    pub type size_t = std::ffi::c_ulong;
}
pub mod glibconfig_h {
    pub type guint32 = std::ffi::c_uint;
    pub type gsize = std::ffi::c_ulong;
}
pub mod gtypes_h {
    pub type gchar = std::ffi::c_char;
    pub type gint = std::ffi::c_int;
    pub type gboolean = gint;
    pub type guint = std::ffi::c_uint;
    pub type gdouble = std::ffi::c_double;
    pub type gpointer = *mut std::ffi::c_void;
    pub type GDestroyNotify = Option<unsafe extern "C" fn(gpointer) -> ()>;
}
pub mod garray_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GPtrArray {
        pub pdata: *mut gpointer,
        pub len: guint,
    }
    pub type GPtrArray = _GPtrArray;
    use super::gtypes_h::{GDestroyNotify, gboolean, gint, gpointer, guint};
    unsafe extern "C" {
        pub fn g_ptr_array_new_with_free_func(element_free_func: GDestroyNotify) -> *mut GPtrArray;
        pub fn g_ptr_array_free(array: *mut GPtrArray, free_segment: gboolean) -> *mut gpointer;
        pub fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
        pub fn g_ptr_array_insert(array: *mut GPtrArray, index_: gint, data: gpointer);
    }
}
pub mod gdataset_h {
    pub type GData = _GData;
    unsafe extern "C" {
        pub type _GData;
    }
}
pub mod gunicode_h {
    pub type gunichar = guint32;
    use super::glibconfig_h::guint32;
    use super::gtypes_h::{gboolean, gchar, gint};
    unsafe extern "C" {
        pub fn g_unichar_isgraph(c: gunichar) -> gboolean;
        pub fn g_unichar_to_utf8(c: gunichar, outbuf: *mut gchar) -> gint;
    }
}
pub mod lua_h {
    pub type lua_CFunction = Option<unsafe extern "C" fn(*mut lua_State) -> std::ffi::c_int>;
    use super::__stddef_size_t_h::size_t;
    unsafe extern "C" {
        pub type lua_State;
        pub fn lua_atpanic(L: *mut lua_State, panicf: lua_CFunction) -> lua_CFunction;
        pub fn lua_gettop(L: *mut lua_State) -> std::ffi::c_int;
        pub fn lua_settop(L: *mut lua_State, idx: std::ffi::c_int);
        pub fn lua_remove(L: *mut lua_State, idx: std::ffi::c_int);
        pub fn lua_insert(L: *mut lua_State, idx: std::ffi::c_int);
        pub fn lua_tolstring(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            len: *mut size_t,
        ) -> *const std::ffi::c_char;
        pub fn lua_pushstring(L: *mut lua_State, s: *const std::ffi::c_char);
        pub fn lua_pushcclosure(L: *mut lua_State, fn_0: lua_CFunction, n: std::ffi::c_int);
        pub fn lua_createtable(L: *mut lua_State, narr: std::ffi::c_int, nrec: std::ffi::c_int);
        pub fn lua_setfield(L: *mut lua_State, idx: std::ffi::c_int, k: *const std::ffi::c_char);
        pub fn lua_rawseti(L: *mut lua_State, idx: std::ffi::c_int, n: std::ffi::c_int);
        pub fn lua_pcall(
            L: *mut lua_State,
            nargs: std::ffi::c_int,
            nresults: std::ffi::c_int,
            errfunc: std::ffi::c_int,
        ) -> std::ffi::c_int;
    }
}
pub mod log_h {
    pub type log_level_t = std::ffi::c_uint;
    pub const LOG_LEVEL_debug: log_level_t = 5;
    pub const LOG_LEVEL_verbose: log_level_t = 4;
    pub const LOG_LEVEL_info: log_level_t = 3;
    pub const LOG_LEVEL_warn: log_level_t = 2;
    pub const LOG_LEVEL_error: log_level_t = 1;
    pub const LOG_LEVEL_fatal: log_level_t = 0;
    use super::gtypes_h::gchar;
    unsafe extern "C" {
        pub fn _log(lvl: log_level_t, _: *const gchar, _: *const gchar, _: ...);
    }
}
pub mod gtype_h {
    pub type GType = gsize;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GTypeClass {
        pub g_type: GType,
    }
    pub type GTypeClass = _GTypeClass;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GTypeInstance {
        pub g_class: *mut GTypeClass,
    }
    pub type GTypeInstance = _GTypeInstance;
    use super::glibconfig_h::gsize;
}
pub mod gobject_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GObject {
        pub g_type_instance: GTypeInstance,
        pub ref_count: guint,
        pub qdata: *mut GData,
    }
    pub type GObject = _GObject;
    use super::gdataset_h::GData;
    use super::gtype_h::GTypeInstance;
    use super::gtypes_h::guint;
}
pub mod common_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _common_t {
        pub L: *mut lua_State,
    }
    pub type common_t = _common_t;
    use super::lua_h::lua_State;
    unsafe extern "C" {
        pub static mut common: common_t;
    }
}
pub mod globalconf_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct globalconf_t {
        pub application: *mut GtkApplication,
        pub config_dir: *mut gchar,
        pub data_dir: *mut gchar,
        pub cache_dir: *mut gchar,
        pub profile: *mut gchar,
        pub confpath: *mut gchar,
        pub execpath: *mut gchar,
        pub nounique: gboolean,
        pub argv: *mut GPtrArray,
        pub windows: *mut GPtrArray,
        pub webviews: *mut GPtrArray,
        pub stylesheets: *mut GPtrArray,
        pub starttime: gdouble,
    }
    use super::garray_h::GPtrArray;
    use super::gtkapplication_h::GtkApplication;
    use super::gtypes_h::{gboolean, gchar, gdouble};
    unsafe extern "C" {
        pub static mut globalconf: globalconf_t;
    }
}
pub mod gtkapplication_h {
    pub type GtkApplication = _GtkApplication;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GtkApplication {
        pub parent: GApplication,
        pub priv_0: *mut GtkApplicationPrivate,
    }
    pub type GtkApplicationPrivate = _GtkApplicationPrivate;
    use super::giotypes_h::GApplication;
    unsafe extern "C" {
        pub type _GtkApplicationPrivate;
    }
}
pub mod giotypes_h {
    pub type GApplication = _GApplication;
    use super::gapplication_h::_GApplication;
}
pub mod gapplication_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GApplication {
        pub parent_instance: GObject,
        pub priv_0: *mut GApplicationPrivate,
    }
    pub type GApplicationPrivate = _GApplicationPrivate;
    use super::gobject_h::GObject;
    unsafe extern "C" {
        pub type _GApplicationPrivate;
    }
}
pub mod gdktypes_h {
    pub const GDK_MOD5_MASK: C2RustUnnamed = 128;
    pub const GDK_MOD4_MASK: C2RustUnnamed = 64;
    pub const GDK_MOD3_MASK: C2RustUnnamed = 32;
    pub const GDK_MOD2_MASK: C2RustUnnamed = 16;
    pub const GDK_MOD1_MASK: C2RustUnnamed = 8;
    pub const GDK_CONTROL_MASK: C2RustUnnamed = 4;
    pub const GDK_LOCK_MASK: C2RustUnnamed = 2;
    pub const GDK_SHIFT_MASK: C2RustUnnamed = 1;
    pub const GDK_MODIFIER_MASK: C2RustUnnamed = 1543512063;
    pub type C2RustUnnamed = std::ffi::c_uint;
    pub const GDK_RELEASE_MASK: C2RustUnnamed = 1073741824;
    pub const GDK_MODIFIER_RESERVED_29_MASK: C2RustUnnamed = 536870912;
    pub const GDK_META_MASK: C2RustUnnamed = 268435456;
    pub const GDK_HYPER_MASK: C2RustUnnamed = 134217728;
    pub const GDK_SUPER_MASK: C2RustUnnamed = 67108864;
    pub const GDK_MODIFIER_RESERVED_25_MASK: C2RustUnnamed = 33554432;
    pub const GDK_MODIFIER_RESERVED_24_MASK: C2RustUnnamed = 16777216;
    pub const GDK_MODIFIER_RESERVED_23_MASK: C2RustUnnamed = 8388608;
    pub const GDK_MODIFIER_RESERVED_22_MASK: C2RustUnnamed = 4194304;
    pub const GDK_MODIFIER_RESERVED_21_MASK: C2RustUnnamed = 2097152;
    pub const GDK_MODIFIER_RESERVED_20_MASK: C2RustUnnamed = 1048576;
    pub const GDK_MODIFIER_RESERVED_19_MASK: C2RustUnnamed = 524288;
    pub const GDK_MODIFIER_RESERVED_18_MASK: C2RustUnnamed = 262144;
    pub const GDK_MODIFIER_RESERVED_17_MASK: C2RustUnnamed = 131072;
    pub const GDK_MODIFIER_RESERVED_16_MASK: C2RustUnnamed = 65536;
    pub const GDK_MODIFIER_RESERVED_15_MASK: C2RustUnnamed = 32768;
    pub const GDK_MODIFIER_RESERVED_14_MASK: C2RustUnnamed = 16384;
    pub const GDK_MODIFIER_RESERVED_13_MASK: C2RustUnnamed = 8192;
    pub const GDK_BUTTON5_MASK: C2RustUnnamed = 4096;
    pub const GDK_BUTTON4_MASK: C2RustUnnamed = 2048;
    pub const GDK_BUTTON3_MASK: C2RustUnnamed = 1024;
    pub const GDK_BUTTON2_MASK: C2RustUnnamed = 512;
    pub const GDK_BUTTON1_MASK: C2RustUnnamed = 256;
}
pub mod string_h {
    unsafe extern "C" {
        pub fn memcpy(
            _: *mut std::ffi::c_void,
            _: *const std::ffi::c_void,
            _: std::ffi::c_ulong,
        ) -> *mut std::ffi::c_void;
        pub fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    }
}
pub mod gutils_h {
    use super::gtypes_h::gchar;
    unsafe extern "C" {
        pub fn g_get_system_config_dirs() -> *const *const gchar;
    }
}
pub mod stdlib_h {
    unsafe extern "C" {
        pub fn atoi(__nptr: *const std::ffi::c_char) -> std::ffi::c_int;
        pub fn getenv(__name: *const std::ffi::c_char) -> *mut std::ffi::c_char;
        pub fn setenv(
            __name: *const std::ffi::c_char,
            __value: *const std::ffi::c_char,
            __replace: std::ffi::c_int,
        ) -> std::ffi::c_int;
        pub fn unsetenv(__name: *const std::ffi::c_char) -> std::ffi::c_int;
    }
}
pub mod gfileutils_h {
    use super::gtypes_h::gchar;
    unsafe extern "C" {
        pub fn g_build_filename(first_element: *const gchar, _: ...) -> *mut gchar;
    }
}
pub mod gmem_h {
    use super::glibconfig_h::gsize;
    use super::gtypes_h::gpointer;
    #[link(name = "gtk-4")]
    unsafe extern "C" {
        pub fn g_free(mem: gpointer);
        pub fn g_malloc(n_bytes: gsize) -> gpointer;
    }
}
pub mod gstrfuncs_h {
    #[inline(always)]
    pub unsafe extern "C" fn g_strdup_inline(
        mut str: *const std::ffi::c_char,
    ) -> *mut std::ffi::c_char {
        if 0 != 0 && str.is_null() {
            return 0 as *mut std::ffi::c_char;
        }
        if 0 != 0 && !str.is_null() && 0 != 0 {
            let len: size_t = (strlen(str)).wrapping_add(1 as std::ffi::c_int as std::ffi::c_ulong);
            let mut dup_str: *mut std::ffi::c_char = g_malloc(len) as *mut std::ffi::c_char;
            return memcpy(
                dup_str as *mut std::ffi::c_void,
                str as *const std::ffi::c_void,
                len,
            ) as *mut std::ffi::c_char;
        }
        return g_strdup(str);
    }
    use super::__stddef_size_t_h::size_t;
    use super::gmem_h::g_malloc;
    use super::gtypes_h::{gchar, gint};
    use super::string_h::{memcpy, strlen};
    unsafe extern "C" {
        pub fn g_strdup(str: *const gchar) -> *mut gchar;
        pub fn g_strdup_printf(format: *const gchar, _: ...) -> *mut gchar;
        pub fn g_strsplit(
            string: *const gchar,
            delimiter: *const gchar,
            max_tokens: gint,
        ) -> *mut *mut gchar;
        pub fn g_strjoinv(separator: *const gchar, str_array: *mut *mut gchar) -> *mut gchar;
        pub fn g_strfreev(str_array: *mut *mut gchar);
    }
}
pub mod unistd_h {
    unsafe extern "C" {
        pub fn execvp(
            __file: *const std::ffi::c_char,
            __argv: *const *mut std::ffi::c_char,
        ) -> std::ffi::c_int;
    }
}
pub mod util_h {
    use super::gtypes_h::{gboolean, gchar, gint};
    use super::lua_h::lua_State;
    unsafe extern "C" {
        pub fn file_exists(_: *const gchar) -> gboolean;
        pub fn luaH_panic(L: *mut lua_State) -> gint;
    }
}
pub mod ipc_h {
    unsafe extern "C" {
        pub fn ipc_remove_socket_file();
    }
}
pub mod lauxlib_h {
    use super::lua_h::lua_State;
    unsafe extern "C" {
        pub fn luaL_loadfile(
            L: *mut lua_State,
            filename: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
        pub fn luaL_newstate() -> *mut lua_State;
    }
}
pub mod lualib_h {
    use super::lua_h::lua_State;
    unsafe extern "C" {
        pub fn luaL_openlibs(L: *mut lua_State);
    }
}
pub mod luautil_h {
    use super::gtypes_h::{gchar, gint};
    use super::lua_h::lua_State;
    unsafe extern "C" {
        pub fn luaH_dofunction_on_error(L: *mut lua_State) -> gint;
        pub fn luaH_add_paths(L: *mut lua_State, config_dir: *const gchar);
    }
}
pub mod common_lualib_h {
    #[inline]
    pub unsafe extern "C" fn luaH_dofunction(
        mut L: *mut lua_State,
        mut nargs: gint,
        mut nret: gint,
    ) -> gboolean {
        lua_insert(L, -nargs - 1 as std::ffi::c_int);
        lua_pushcclosure(
            L,
            Some(luaH_dofunction_on_error as unsafe extern "C" fn(*mut lua_State) -> gint),
            0 as std::ffi::c_int,
        );
        lua_insert(L, -nargs - 2 as std::ffi::c_int);
        let mut error_func_pos: gint = lua_gettop(L) - nargs - 1 as std::ffi::c_int;
        if lua_pcall(L, nargs, nret, -nargs - 2 as std::ffi::c_int) != 0 {
            _log(
                LOG_LEVEL_error,
                b"./common/lualib.h\0" as *const u8 as *const std::ffi::c_char,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                lua_tolstring(L, -(1 as std::ffi::c_int), 0 as *mut size_t),
            );
            lua_settop(L, -(2 as std::ffi::c_int) - 1 as std::ffi::c_int);
            return 0 as std::ffi::c_int;
        }
        lua_remove(L, error_func_pos);
        return (0 as std::ffi::c_int == 0) as std::ffi::c_int;
    }
    use super::__stddef_size_t_h::size_t;
    use super::gtypes_h::{gboolean, gint};
    use super::log_h::{_log, LOG_LEVEL_error, log_level_t};
    use super::lua_h::{
        lua_State, lua_gettop, lua_insert, lua_pcall, lua_pushcclosure, lua_remove, lua_settop,
        lua_tolstring,
    };
    use super::luautil_h::luaH_dofunction_on_error;
}
pub mod luaobject_h {
    use super::lua_h::lua_State;
    unsafe extern "C" {
        pub fn luaH_object_setup(L: *mut lua_State);
    }
}
pub mod luah_h {
    use super::lua_h::lua_State;
    unsafe extern "C" {
        pub fn luaH_fixups(L: *mut lua_State);
    }
}
pub mod luakit_log_h {
    unsafe extern "C" {
        pub fn log_dump_queued_emissions() -> *mut std::ffi::c_char;
    }
}
pub mod luayield_h {
    use super::lua_h::lua_State;
    unsafe extern "C" {
        pub fn luaH_yield_setup(L: *mut lua_State);
    }
}
pub mod gdkkeys_h {
    use super::glibconfig_h::guint32;
    use super::gtypes_h::{gchar, guint};
    unsafe extern "C" {
        pub fn gdk_keyval_name(keyval: guint) -> *mut gchar;
        pub fn gdk_keyval_to_unicode(keyval: guint) -> guint32;
    }
}
pub mod download_h {
    use super::lua_h::lua_State;
    unsafe extern "C" {
        pub fn download_class_setup(_: *mut lua_State);
    }
}
pub mod luakit_h {
    use super::lua_h::lua_State;
    unsafe extern "C" {
        pub fn luakit_lib_setup(L: *mut lua_State);
    }
}
pub mod request_h {
    use super::lua_h::lua_State;
    unsafe extern "C" {
        pub fn request_class_setup(_: *mut lua_State);
    }
}
pub mod sqlite3_h {
    use super::lua_h::lua_State;
    unsafe extern "C" {
        pub fn sqlite3_class_setup(_: *mut lua_State);
    }
}
pub mod soup_h {
    use super::lua_h::lua_State;
    unsafe extern "C" {
        pub fn soup_lib_setup(L: *mut lua_State);
    }
}
pub mod unique_h {
    use super::lua_h::lua_State;
    unsafe extern "C" {
        pub fn unique_lib_setup(_: *mut lua_State);
    }
}
pub mod widget_h {
    use super::lua_h::lua_State;
    unsafe extern "C" {
        pub fn widget_class_setup(_: *mut lua_State);
    }
}
pub mod xdg_h {
    use super::lua_h::lua_State;
    unsafe extern "C" {
        pub fn xdg_lib_setup(_: *mut lua_State);
    }
}
pub mod stylesheet_h {
    use super::lua_h::lua_State;
    unsafe extern "C" {
        pub fn stylesheet_class_setup(_: *mut lua_State);
    }
}
pub mod web_module_h {
    use super::lua_h::lua_State;
    unsafe extern "C" {
        pub fn web_module_lib_setup(_: *mut lua_State);
    }
}
pub mod msg_h {
    use super::lua_h::lua_State;
    unsafe extern "C" {
        pub fn msg_lib_setup(L: *mut lua_State);
    }
}
pub mod clib_ipc_h {
    use super::lua_h::lua_State;
    unsafe extern "C" {
        pub fn ipc_channel_class_setup(_: *mut lua_State);
    }
}
pub mod timer_h {
    use super::lua_h::lua_State;
    unsafe extern "C" {
        pub fn timer_class_setup(_: *mut lua_State);
    }
}
pub mod regex_h {
    use super::lua_h::lua_State;
    unsafe extern "C" {
        pub fn regex_class_setup(_: *mut lua_State);
    }
}
pub mod utf8_h {
    use super::lua_h::lua_State;
    unsafe extern "C" {
        pub fn utf8_lib_setup(_: *mut lua_State);
    }
}
pub use self::__stddef_size_t_h::size_t;
use self::clib_ipc_h::ipc_channel_class_setup;
pub use self::common_h::{_common_t, common, common_t};
pub use self::common_lualib_h::luaH_dofunction;
use self::download_h::download_class_setup;
pub use self::gapplication_h::{_GApplication, _GApplicationPrivate, GApplicationPrivate};
pub use self::garray_h::{
    _GPtrArray, GPtrArray, g_ptr_array_add, g_ptr_array_free, g_ptr_array_insert,
    g_ptr_array_new_with_free_func,
};
pub use self::gdataset_h::{_GData, GData};
use self::gdkkeys_h::{gdk_keyval_name, gdk_keyval_to_unicode};
pub use self::gdktypes_h::{
    C2RustUnnamed, GDK_BUTTON1_MASK, GDK_BUTTON2_MASK, GDK_BUTTON3_MASK, GDK_BUTTON4_MASK,
    GDK_BUTTON5_MASK, GDK_CONTROL_MASK, GDK_HYPER_MASK, GDK_LOCK_MASK, GDK_META_MASK,
    GDK_MOD1_MASK, GDK_MOD2_MASK, GDK_MOD3_MASK, GDK_MOD4_MASK, GDK_MOD5_MASK, GDK_MODIFIER_MASK,
    GDK_MODIFIER_RESERVED_13_MASK, GDK_MODIFIER_RESERVED_14_MASK, GDK_MODIFIER_RESERVED_15_MASK,
    GDK_MODIFIER_RESERVED_16_MASK, GDK_MODIFIER_RESERVED_17_MASK, GDK_MODIFIER_RESERVED_18_MASK,
    GDK_MODIFIER_RESERVED_19_MASK, GDK_MODIFIER_RESERVED_20_MASK, GDK_MODIFIER_RESERVED_21_MASK,
    GDK_MODIFIER_RESERVED_22_MASK, GDK_MODIFIER_RESERVED_23_MASK, GDK_MODIFIER_RESERVED_24_MASK,
    GDK_MODIFIER_RESERVED_25_MASK, GDK_MODIFIER_RESERVED_29_MASK, GDK_RELEASE_MASK, GDK_SHIFT_MASK,
    GDK_SUPER_MASK,
};
use self::gfileutils_h::g_build_filename;
pub use self::giotypes_h::GApplication;
pub use self::glibconfig_h::{gsize, guint32};
pub use self::globalconf_h::{globalconf, globalconf_t};
use self::gmem_h::{g_free, g_malloc};
pub use self::gobject_h::{_GObject, GObject};
pub use self::gstrfuncs_h::{
    g_strdup, g_strdup_inline, g_strdup_printf, g_strfreev, g_strjoinv, g_strsplit,
};
pub use self::gtkapplication_h::{
    _GtkApplication, _GtkApplicationPrivate, GtkApplication, GtkApplicationPrivate,
};
pub use self::gtype_h::{_GTypeClass, _GTypeInstance, GType, GTypeClass, GTypeInstance};
pub use self::gtypes_h::{GDestroyNotify, gboolean, gchar, gdouble, gint, gpointer, guint};
pub use self::gunicode_h::{g_unichar_isgraph, g_unichar_to_utf8, gunichar};
use self::gutils_h::g_get_system_config_dirs;
use self::ipc_h::ipc_remove_socket_file;
use self::lauxlib_h::{luaL_loadfile, luaL_newstate};
pub use self::log_h::{
    _log, LOG_LEVEL_debug, LOG_LEVEL_error, LOG_LEVEL_fatal, LOG_LEVEL_info, LOG_LEVEL_verbose,
    LOG_LEVEL_warn, log_level_t,
};
pub use self::lua_h::{
    lua_CFunction, lua_State, lua_atpanic, lua_createtable, lua_gettop, lua_insert, lua_pcall,
    lua_pushcclosure, lua_pushstring, lua_rawseti, lua_remove, lua_setfield, lua_settop,
    lua_tolstring,
};
use self::luah_h::luaH_fixups;
use self::luakit_h::luakit_lib_setup;
use self::luakit_log_h::log_dump_queued_emissions;
use self::lualib_h::luaL_openlibs;
use self::luaobject_h::luaH_object_setup;
use self::luautil_h::{luaH_add_paths, luaH_dofunction_on_error};
use self::luayield_h::luaH_yield_setup;
use self::msg_h::msg_lib_setup;
use self::regex_h::regex_class_setup;
use self::request_h::request_class_setup;
use self::soup_h::soup_lib_setup;
use self::sqlite3_h::sqlite3_class_setup;
use self::stdlib_h::{atoi, getenv, setenv, unsetenv};
use self::string_h::{memcpy, strlen};
use self::stylesheet_h::stylesheet_class_setup;
use self::timer_h::timer_class_setup;
use self::unique_h::unique_lib_setup;
use self::unistd_h::execvp;
use self::utf8_h::utf8_lib_setup;
use self::util_h::{file_exists, luaH_panic};
use self::web_module_h::web_module_lib_setup;
use self::widget_h::widget_class_setup;
use self::xdg_h::xdg_lib_setup;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn luaH_modifier_table_push(mut L: *mut lua_State, mut state: guint) {
    let mut i: gint = 1 as std::ffi::c_int;
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
    lua_atpanic(
        L,
        Some(luaH_panic as unsafe extern "C" fn(*mut lua_State) -> gint),
    );
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
        lua_rawseti(L, -(2 as std::ffi::c_int), i + 1 as std::ffi::c_int);
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
            lua_tolstring(L, -(1 as std::ffi::c_int), 0 as *mut size_t),
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
                    globalconf.confpath = g_strdup_inline(path_0);
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
                        (*argv).pdata as *mut *mut gchar as *const *mut std::ffi::c_char,
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
