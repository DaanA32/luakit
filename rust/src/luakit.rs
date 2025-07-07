#![feature(extern_types)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(path_statements)]
#![allow(static_mut_refs)]
#![allow(mutable_transmutes)]
#![allow(unreachable_patterns)]

use ::c2rust_bitfields;
use ::libc;

pub mod __stddef_size_t_h {
    pub type size_t = std::ffi::c_ulong;
}
pub mod glibconfig_h {
    pub type guint32 = std::ffi::c_uint;
    pub type gssize = std::ffi::c_long;
    pub type gsize = std::ffi::c_ulong;
}
pub mod types_h {
    pub type __off_t = std::ffi::c_long;
    pub type __off64_t = std::ffi::c_long;
    pub type __pid_t = std::ffi::c_int;
    pub type __time_t = std::ffi::c_long;
    pub type __suseconds_t = std::ffi::c_long;
}
pub mod include_time_h {
    pub type pid_t = __pid_t;
    use super::types_h::__pid_t;
}
pub mod gtypes_h {
    pub type gchar = std::ffi::c_char;
    pub type gint = std::ffi::c_int;
    pub type gboolean = gint;
    pub type guint = std::ffi::c_uint;
    pub type gdouble = std::ffi::c_double;
    pub type gpointer = *mut std::ffi::c_void;
    pub type gconstpointer = *const std::ffi::c_void;
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
    use super::gtypes_h::{GDestroyNotify, gpointer, guint};
    unsafe extern "C" {
        pub fn g_ptr_array_new() -> *mut GPtrArray;
        pub fn g_ptr_array_new_with_free_func(element_free_func: GDestroyNotify) -> *mut GPtrArray;
        pub fn g_ptr_array_remove_index(array: *mut GPtrArray, index_: guint) -> gpointer;
        pub fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    }
}
pub mod gquark_h {
    pub type GQuark = guint32;
    use super::glibconfig_h::guint32;
}
pub mod gerror_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GError {
        pub domain: GQuark,
        pub code: gint,
        pub message: *mut gchar,
    }
    pub type GError = _GError;
    use super::gquark_h::GQuark;
    use super::gtypes_h::{gchar, gint};
}
pub mod gdataset_h {
    pub type GData = _GData;
    unsafe extern "C" {
        pub type _GData;
    }
}
pub mod ghash_h {
    pub type GHashTable = _GHashTable;
    unsafe extern "C" {
        pub type _GHashTable;
    }
}
pub mod gmessages_h {
    pub type GLogLevelFlags = std::ffi::c_int;
    pub const G_LOG_LEVEL_MASK: GLogLevelFlags = -4;
    pub const G_LOG_LEVEL_DEBUG: GLogLevelFlags = 128;
    pub const G_LOG_LEVEL_INFO: GLogLevelFlags = 64;
    pub const G_LOG_LEVEL_MESSAGE: GLogLevelFlags = 32;
    pub const G_LOG_LEVEL_WARNING: GLogLevelFlags = 16;
    pub const G_LOG_LEVEL_CRITICAL: GLogLevelFlags = 8;
    pub const G_LOG_LEVEL_ERROR: GLogLevelFlags = 4;
    pub const G_LOG_FLAG_FATAL: GLogLevelFlags = 2;
    pub const G_LOG_FLAG_RECURSION: GLogLevelFlags = 1;
    pub type GLogWriterOutput = std::ffi::c_uint;
    pub const G_LOG_WRITER_UNHANDLED: GLogWriterOutput = 0;
    pub const G_LOG_WRITER_HANDLED: GLogWriterOutput = 1;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GLogField {
        pub key: *const gchar,
        pub value: gconstpointer,
        pub length: gssize,
    }
    pub type GLogField = _GLogField;
    pub type GLogWriterFunc = Option<
        unsafe extern "C" fn(GLogLevelFlags, *const GLogField, gsize, gpointer) -> GLogWriterOutput,
    >;
    use super::glibconfig_h::{gsize, gssize};
    use super::gtypes_h::{GDestroyNotify, gchar, gconstpointer, gpointer};
    unsafe extern "C" {
        pub fn g_log_set_writer_func(
            func: GLogWriterFunc,
            user_data: gpointer,
            user_data_free: GDestroyNotify,
        );
    }
}
pub mod goption_h {
    pub type GOptionContext = _GOptionContext;
    pub type GOptionGroup = _GOptionGroup;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GOptionEntry {
        pub long_name: *const gchar,
        pub short_name: gchar,
        pub flags: gint,
        pub arg: GOptionArg,
        pub arg_data: gpointer,
        pub description: *const gchar,
        pub arg_description: *const gchar,
    }
    pub type GOptionArg = std::ffi::c_uint;
    pub const G_OPTION_ARG_INT64: GOptionArg = 8;
    pub const G_OPTION_ARG_DOUBLE: GOptionArg = 7;
    pub const G_OPTION_ARG_FILENAME_ARRAY: GOptionArg = 6;
    pub const G_OPTION_ARG_STRING_ARRAY: GOptionArg = 5;
    pub const G_OPTION_ARG_FILENAME: GOptionArg = 4;
    pub const G_OPTION_ARG_CALLBACK: GOptionArg = 3;
    pub const G_OPTION_ARG_INT: GOptionArg = 2;
    pub const G_OPTION_ARG_STRING: GOptionArg = 1;
    pub const G_OPTION_ARG_NONE: GOptionArg = 0;
    pub type GOptionEntry = _GOptionEntry;
    use super::gerror_h::GError;
    use super::gtypes_h::{gboolean, gchar, gint, gpointer};
    unsafe extern "C" {
        pub type _GOptionContext;
        pub type _GOptionGroup;
        pub fn g_option_context_new(parameter_string: *const gchar) -> *mut GOptionContext;
        pub fn g_option_context_free(context: *mut GOptionContext);
        pub fn g_option_context_add_main_entries(
            context: *mut GOptionContext,
            entries: *const GOptionEntry,
            translation_domain: *const gchar,
        );
        pub fn g_option_context_parse(
            context: *mut GOptionContext,
            argc: *mut gint,
            argv: *mut *mut *mut gchar,
            error: *mut *mut GError,
        ) -> gboolean;
        pub fn g_option_context_add_group(context: *mut GOptionContext, group: *mut GOptionGroup);
    }
}
pub mod gtree_h {
    pub type GTree = _GTree;
    unsafe extern "C" {
        pub type _GTree;
    }
}
pub mod struct_timeval_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct timeval {
        pub tv_sec: __time_t,
        pub tv_usec: __suseconds_t,
    }
    use super::types_h::{__suseconds_t, __time_t};
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
pub mod common_h {
    use lua::ffi::lua_State;

    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _common_t {
        pub L: *mut lua_State,
    }
    pub type common_t = _common_t;
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
pub mod giotypes_h {
    pub type GApplication = _GApplication;
    use super::gapplication_h::_GApplication;
}
pub mod struct_FILE_h {
    use c2rust_bitfields::BitfieldStruct;
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct _IO_FILE {
        pub _flags: std::ffi::c_int,
        pub _IO_read_ptr: *mut std::ffi::c_char,
        pub _IO_read_end: *mut std::ffi::c_char,
        pub _IO_read_base: *mut std::ffi::c_char,
        pub _IO_write_base: *mut std::ffi::c_char,
        pub _IO_write_ptr: *mut std::ffi::c_char,
        pub _IO_write_end: *mut std::ffi::c_char,
        pub _IO_buf_base: *mut std::ffi::c_char,
        pub _IO_buf_end: *mut std::ffi::c_char,
        pub _IO_save_base: *mut std::ffi::c_char,
        pub _IO_backup_base: *mut std::ffi::c_char,
        pub _IO_save_end: *mut std::ffi::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: std::ffi::c_int,
        #[bitfield(name = "_flags2", ty = "std::ffi::c_int", bits = "0..=23")]
        pub _flags2: [u8; 3],
        pub _short_backupbuf: [std::ffi::c_char; 1],
        pub _old_offset: __off_t,
        pub _cur_column: std::ffi::c_ushort,
        pub _vtable_offset: std::ffi::c_schar,
        pub _shortbuf: [std::ffi::c_char; 1],
        pub _lock: *mut std::ffi::c_void,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut std::ffi::c_void,
        pub _prevchain: *mut *mut _IO_FILE,
        pub _mode: std::ffi::c_int,
        pub _unused2: [std::ffi::c_char; 20],
    }
    pub type _IO_lock_t = ();
    use super::types_h::{__off_t, __off64_t};
    unsafe extern "C" {
        pub type _IO_wide_data;
        pub type _IO_codecvt;
        pub type _IO_marker;
    }
}
pub mod FILE_h {
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
pub mod gtkapplication_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _GtkApplication {
        pub parent: GApplication,
        pub priv_0: *mut GtkApplicationPrivate,
    }
    pub type GtkApplicationPrivate = _GtkApplicationPrivate;
    pub type GtkApplication = _GtkApplication;
    use super::giotypes_h::GApplication;
    unsafe extern "C" {
        pub type _GtkApplicationPrivate;
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
}
pub mod signal_h {
    pub type signal_t = GTree;
    use super::gtree_h::GTree;
}
pub mod luaclass_h {
    pub type lua_class_property_array_t = GHashTable;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct lua_object_t {
        pub signals: *mut signal_t,
    }
    pub type lua_class_allocator_t =
        Option<unsafe extern "C" fn(*mut lua_State) -> *mut lua_object_t>;
    pub type lua_class_propfunc_t =
        Option<unsafe extern "C" fn(*mut lua_State, *mut lua_object_t) -> gint>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct lua_class_t {
        pub name: *const gchar,
        pub signals: *mut signal_t,
        pub allocator: lua_class_allocator_t,
        pub properties: *mut lua_class_property_array_t,
        pub index_miss_property: lua_class_propfunc_t,
        pub newindex_miss_property: lua_class_propfunc_t,
    }
    use lua::ffi::lua_State;

    use super::ghash_h::GHashTable;
    use super::gtypes_h::{gchar, gint};
    use super::signal_h::signal_t;
}
pub mod string_h {
    unsafe extern "C" {
        pub fn memcpy(
            _: *mut std::ffi::c_void,
            _: *const std::ffi::c_void,
            _: std::ffi::c_ulong,
        ) -> *mut std::ffi::c_void;
        pub fn memset(
            _: *mut std::ffi::c_void,
            _: std::ffi::c_int,
            _: std::ffi::c_ulong,
        ) -> *mut std::ffi::c_void;
        pub fn strcmp(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> std::ffi::c_int;
        pub fn strchr(_: *const std::ffi::c_char, _: std::ffi::c_int) -> *mut std::ffi::c_char;
        pub fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    }
}
pub mod gutils_h {
    use super::gtypes_h::gchar;
    #[link(name = "gtk-4")]
    unsafe extern "C" {
        pub fn g_get_user_data_dir() -> *const gchar;
        pub fn g_get_user_config_dir() -> *const gchar;
        pub fn g_get_user_cache_dir() -> *const gchar;
    }
}
pub mod stdlib_h {
    unsafe extern "C" {
        pub fn exit(_: std::ffi::c_int) -> !;
    }
}
pub mod gfileutils_h {
    use super::gtypes_h::{gchar, gint};
    unsafe extern "C" {
        pub fn g_build_filename(first_element: *const gchar, _: ...) -> *mut gchar;
        pub fn g_mkdir_with_parents(pathname: *const gchar, mode: gint) -> gint;
    }
}
pub mod gmem_h {
    use super::glibconfig_h::gsize;
    use super::gtypes_h::gpointer;
    #[link(name = "gtk-4")]
    unsafe extern "C" {
        // pub fn g_free(mem: gpointer);
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
    #[link(name = "gtk-4")]
    unsafe extern "C" {
        pub fn g_strdup(str: *const gchar) -> *mut gchar;
        pub fn g_strsplit(
            string: *const gchar,
            delimiter: *const gchar,
            max_tokens: gint,
        ) -> *mut *mut gchar;
        pub fn g_strfreev(str_array: *mut *mut gchar);
        pub fn g_strdupv(str_array: *mut *mut gchar) -> *mut *mut gchar;
    }
}
pub mod errno_h {
    unsafe extern "C" {
        pub fn __errno_location() -> *mut std::ffi::c_int;
    }
}
pub mod unistd_h {
    use super::types_h::__pid_t;
    unsafe extern "C" {
        pub fn setsid() -> __pid_t;
        pub fn fork() -> __pid_t;
    }
}
pub mod time_h {
    use super::struct_timeval_h::timeval;
    unsafe extern "C" {
        pub fn gettimeofday(__tv: *mut timeval, __tz: *mut std::ffi::c_void) -> std::ffi::c_int;
    }
}
pub mod util_h {
    #[inline]
    pub unsafe extern "C" fn l_time() -> gdouble {
        let mut tv: timeval = timeval {
            tv_sec: 0,
            tv_usec: 0,
        };
        gettimeofday(&mut tv, 0 as *mut std::ffi::c_void);
        return tv.tv_sec as std::ffi::c_double + tv.tv_usec as std::ffi::c_double / 1e6f64;
    }
    use super::gtypes_h::gdouble;
    use super::struct_timeval_h::timeval;
    use super::time_h::gettimeofday;
    use super::types_h::{__suseconds_t, __time_t};
}
pub mod stdio_h {
    use super::FILE_h::FILE;
    unsafe extern "C" {
        pub static mut stderr: *mut FILE;
    }
}
pub mod gtkmain_h {
    use super::goption_h::GOptionGroup;
    use super::gtypes_h::gboolean;
    unsafe extern "C" {
        pub fn gtk_init(argc: *mut std::ffi::c_int, argv: *mut *mut *mut std::ffi::c_char);
        pub fn gtk_get_option_group(open_default_display: gboolean) -> *mut GOptionGroup;
        pub fn gtk_disable_setlocale();
        pub fn gtk_main();
    }
}
pub mod gprintf_h {
    use super::FILE_h::FILE;
    use super::gtypes_h::{gchar, gint};
    #[link(name = "gtk-4")]
    unsafe extern "C" {
        pub fn g_printf(format: *const gchar, _: ...) -> gint;
        pub fn g_fprintf(file: *mut FILE, format: *const gchar, _: ...) -> gint;
    }
}
pub mod clib;
pub mod common;
pub mod luaclass;
pub mod luah;

pub mod ipc_h {
    unsafe extern "C" {
        pub fn ipc_init();
    }
}
pub mod luakit_log_h {
    use super::log_h::log_level_t;
    unsafe extern "C" {
        pub fn log_init();
        pub fn log_level_from_string(
            out: *mut log_level_t,
            str: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
        pub fn log_set_verbosity(group: *const std::ffi::c_char, lvl: log_level_t);
    }
}
pub mod WebKitVersion_h {
    use super::gtypes_h::guint;
    #[link(name = "webkit2gtk-4.1")]
    unsafe extern "C" {
        pub fn webkit_get_major_version() -> guint;
        pub fn webkit_get_minor_version() -> guint;
        pub fn webkit_get_micro_version() -> guint;
    }
}
pub mod web_context_h {
    unsafe extern "C" {
        pub fn web_context_init();
    }
}
pub mod locale_h {
    unsafe extern "C" {
        pub fn setlocale(
            __category: std::ffi::c_int,
            __locale: *const std::ffi::c_char,
        ) -> *mut std::ffi::c_char;
    }
}
use glib_sys::g_free;
use lua::ffi::lua_State;

pub use self::__stddef_size_t_h::size_t;
pub use self::FILE_h::FILE;
use self::WebKitVersion_h::{
    webkit_get_major_version, webkit_get_micro_version, webkit_get_minor_version,
};
pub use self::common_h::{_common_t, common_t};
use self::errno_h::__errno_location;
pub use self::gapplication_h::{_GApplication, _GApplicationPrivate, GApplicationPrivate};
pub use self::garray_h::{
    _GPtrArray, GPtrArray, g_ptr_array_add, g_ptr_array_new, g_ptr_array_new_with_free_func,
    g_ptr_array_remove_index,
};
pub use self::gdataset_h::{_GData, GData};
pub use self::gerror_h::{_GError, GError};
use self::gfileutils_h::{g_build_filename, g_mkdir_with_parents};
pub use self::ghash_h::{_GHashTable, GHashTable};
pub use self::giotypes_h::GApplication;
pub use self::glibconfig_h::{gsize, gssize, guint32};
pub use self::globalconf_h::globalconf_t;
use self::gmem_h::g_malloc;
pub use self::gmessages_h::{
    _GLogField, G_LOG_FLAG_FATAL, G_LOG_FLAG_RECURSION, G_LOG_LEVEL_CRITICAL, G_LOG_LEVEL_DEBUG,
    G_LOG_LEVEL_ERROR, G_LOG_LEVEL_INFO, G_LOG_LEVEL_MASK, G_LOG_LEVEL_MESSAGE,
    G_LOG_LEVEL_WARNING, G_LOG_WRITER_HANDLED, G_LOG_WRITER_UNHANDLED, GLogField, GLogLevelFlags,
    GLogWriterFunc, GLogWriterOutput, g_log_set_writer_func,
};
pub use self::gobject_h::{_GObject, GObject};
pub use self::goption_h::{
    _GOptionContext, _GOptionEntry, _GOptionGroup, G_OPTION_ARG_CALLBACK, G_OPTION_ARG_DOUBLE,
    G_OPTION_ARG_FILENAME, G_OPTION_ARG_FILENAME_ARRAY, G_OPTION_ARG_INT, G_OPTION_ARG_INT64,
    G_OPTION_ARG_NONE, G_OPTION_ARG_STRING, G_OPTION_ARG_STRING_ARRAY, GOptionArg, GOptionContext,
    GOptionEntry, GOptionGroup, g_option_context_add_group, g_option_context_add_main_entries,
    g_option_context_free, g_option_context_new, g_option_context_parse,
};
use self::gprintf_h::{g_fprintf, g_printf};
pub use self::gquark_h::GQuark;
pub use self::gstrfuncs_h::{g_strdup, g_strdup_inline, g_strdupv, g_strfreev, g_strsplit};
pub use self::gtkapplication_h::{
    _GtkApplication, _GtkApplicationPrivate, GtkApplication, GtkApplicationPrivate,
};
use self::gtkmain_h::{gtk_disable_setlocale, gtk_get_option_group, gtk_init, gtk_main};
pub use self::gtree_h::{_GTree, GTree};
pub use self::gtype_h::{_GTypeClass, _GTypeInstance, GType, GTypeClass, GTypeInstance};
pub use self::gtypes_h::{
    GDestroyNotify, gboolean, gchar, gconstpointer, gdouble, gint, gpointer, guint,
};
use self::gutils_h::{g_get_user_cache_dir, g_get_user_config_dir, g_get_user_data_dir};
pub use self::include_time_h::pid_t;
use self::ipc_h::ipc_init;
use self::locale_h::setlocale;
pub use self::log_h::{
    _log, LOG_LEVEL_debug, LOG_LEVEL_error, LOG_LEVEL_fatal, LOG_LEVEL_info, LOG_LEVEL_verbose,
    LOG_LEVEL_warn, log_level_t,
};
pub use self::luaclass_h::{
    lua_class_allocator_t, lua_class_property_array_t, lua_class_propfunc_t, lua_class_t,
    lua_object_t,
};
use self::luah::{luaH_init, luaH_parserc};
use self::luakit_log_h::{log_init, log_level_from_string, log_set_verbosity};
pub use self::signal_h::signal_t;
use self::stdio_h::stderr;
use self::stdlib_h::exit;
use self::string_h::{memcpy, memset, strchr, strcmp, strlen};
pub use self::struct_FILE_h::{_IO_FILE, _IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data};
pub use self::struct_timeval_h::timeval;
use self::time_h::gettimeofday;
pub use self::types_h::{__off_t, __off64_t, __pid_t, __suseconds_t, __time_t};
use self::unistd_h::{fork, setsid};
pub use self::util_h::l_time;
use self::web_context_h::web_context_init;
#[unsafe(no_mangle)]
pub static mut common: common_t = _common_t {
    L: 0 as *const lua_State as *mut lua_State,
};
#[unsafe(no_mangle)]
pub static mut globalconf: globalconf_t = globalconf_t {
    application: 0 as *const GtkApplication as *mut GtkApplication,
    config_dir: 0 as *const gchar as *mut gchar,
    data_dir: 0 as *const gchar as *mut gchar,
    cache_dir: 0 as *const gchar as *mut gchar,
    profile: 0 as *const gchar as *mut gchar,
    confpath: 0 as *const gchar as *mut gchar,
    execpath: 0 as *const gchar as *mut gchar,
    nounique: 0,
    argv: 0 as *const GPtrArray as *mut GPtrArray,
    windows: 0 as *const GPtrArray as *mut GPtrArray,
    webviews: 0 as *const GPtrArray as *mut GPtrArray,
    stylesheets: 0 as *const GPtrArray as *mut GPtrArray,
    starttime: 0.,
};
#[unsafe(no_mangle)]
pub static mut widget_class: lua_class_t = lua_class_t {
    name: 0 as *const gchar,
    signals: 0 as *const signal_t as *mut signal_t,
    allocator: None,
    properties: 0 as *const lua_class_property_array_t as *mut lua_class_property_array_t,
    index_miss_property: None,
    newindex_miss_property: None,
};
unsafe extern "C" fn init_directories() {
    globalconf.cache_dir = g_build_filename(
        g_get_user_cache_dir(),
        b"luakit\0" as *const u8 as *const std::ffi::c_char,
        globalconf.profile,
        0 as *mut std::ffi::c_void,
    );
    globalconf.config_dir = g_build_filename(
        g_get_user_config_dir(),
        b"luakit\0" as *const u8 as *const std::ffi::c_char,
        globalconf.profile,
        0 as *mut std::ffi::c_void,
    );
    globalconf.data_dir = g_build_filename(
        g_get_user_data_dir(),
        b"luakit\0" as *const u8 as *const std::ffi::c_char,
        globalconf.profile,
        0 as *mut std::ffi::c_void,
    );
    g_mkdir_with_parents(globalconf.cache_dir, 0o700 as std::ffi::c_int);
    g_mkdir_with_parents(globalconf.config_dir, 0o700 as std::ffi::c_int);
    g_mkdir_with_parents(globalconf.data_dir, 0o700 as std::ffi::c_int);
}
unsafe extern "C" fn parse_log_level_option(mut log_lvl: *mut gchar) {
    let mut parts: *mut *mut gchar = g_strsplit(
        log_lvl,
        b",\0" as *const u8 as *const std::ffi::c_char,
        0 as std::ffi::c_int,
    );
    let mut part: *mut *mut gchar = parts;
    while !(*part).is_null() {
        let mut lvl: log_level_t = LOG_LEVEL_fatal;
        if log_level_from_string(&mut lvl, *part) == 0 {
            log_set_verbosity(b"all\0" as *const u8 as *const std::ffi::c_char, lvl);
        } else {
            let mut sep: *mut gchar = strchr(*part, '=' as i32);
            if !sep.is_null()
                && log_level_from_string(&mut lvl, sep.offset(1 as std::ffi::c_int as isize)) == 0
            {
                *sep = '\0' as i32 as gchar;
                log_set_verbosity(*part, lvl);
            } else {
                _log(
                    LOG_LEVEL_warn,
                    b"luakit.c\0" as *const u8 as *const std::ffi::c_char,
                    b"ignoring unrecognized --log option '%s'\0" as *const u8
                        as *const std::ffi::c_char,
                    *part,
                );
            }
        }
        part = part.offset(1);
        part;
    }
    g_strfreev(parts);
}
unsafe extern "C" fn parseopts(
    mut argc: *mut std::ffi::c_int,
    mut argv: *mut *mut gchar,
    mut nonblock: *mut *mut gboolean,
) -> *mut *mut gchar {
    let mut context: *mut GOptionContext = 0 as *mut GOptionContext;
    let mut version_only: *mut gboolean = 0 as *mut gboolean;
    let mut check_only: *mut gboolean = 0 as *mut gboolean;
    let mut uris: *mut *mut gchar = 0 as *mut *mut gchar;
    globalconf.profile = 0 as *mut gchar;
    let mut verbose: gboolean = 0 as std::ffi::c_int;
    let mut log_lvl: *mut gchar = 0 as *mut gchar;
    globalconf.execpath = g_strdup_inline(*argv.offset(0 as std::ffi::c_int as isize));
    globalconf.nounique = 0 as std::ffi::c_int;
    let entries: [GOptionEntry; 10] = [
        {
            let mut init = _GOptionEntry {
                long_name: b"check\0" as *const u8 as *const std::ffi::c_char,
                short_name: 'k' as i32 as gchar,
                flags: 0 as std::ffi::c_int,
                arg: G_OPTION_ARG_NONE,
                arg_data: &mut check_only as *mut *mut gboolean as gpointer,
                description: b"check config and exit\0" as *const u8 as *const std::ffi::c_char,
                arg_description: 0 as *const gchar,
            };
            init
        },
        {
            let mut init = _GOptionEntry {
                long_name: b"config\0" as *const u8 as *const std::ffi::c_char,
                short_name: 'c' as i32 as gchar,
                flags: 0 as std::ffi::c_int,
                arg: G_OPTION_ARG_STRING,
                arg_data: &mut globalconf.confpath as *mut *mut gchar as gpointer,
                description: b"configuration file to use\0" as *const u8 as *const std::ffi::c_char,
                arg_description: b"FILE\0" as *const u8 as *const std::ffi::c_char,
            };
            init
        },
        {
            let mut init = _GOptionEntry {
                long_name: b"profile\0" as *const u8 as *const std::ffi::c_char,
                short_name: 'p' as i32 as gchar,
                flags: 0 as std::ffi::c_int,
                arg: G_OPTION_ARG_STRING,
                arg_data: &mut globalconf.profile as *mut *mut gchar as gpointer,
                description: b"profile name to use\0" as *const u8 as *const std::ffi::c_char,
                arg_description: b"NAME\0" as *const u8 as *const std::ffi::c_char,
            };
            init
        },
        {
            let mut init = _GOptionEntry {
                long_name: b"nonblock\0" as *const u8 as *const std::ffi::c_char,
                short_name: 'n' as i32 as gchar,
                flags: 0 as std::ffi::c_int,
                arg: G_OPTION_ARG_NONE,
                arg_data: nonblock as gpointer,
                description: b"run in background\0" as *const u8 as *const std::ffi::c_char,
                arg_description: 0 as *const gchar,
            };
            init
        },
        {
            let mut init = _GOptionEntry {
                long_name: b"nounique\0" as *const u8 as *const std::ffi::c_char,
                short_name: 'U' as i32 as gchar,
                flags: 0 as std::ffi::c_int,
                arg: G_OPTION_ARG_NONE,
                arg_data: &mut globalconf.nounique as *mut gboolean as gpointer,
                description: b"ignore libunique bindings\0" as *const u8 as *const std::ffi::c_char,
                arg_description: 0 as *const gchar,
            };
            init
        },
        {
            let mut init = _GOptionEntry {
                long_name: b"uri\0" as *const u8 as *const std::ffi::c_char,
                short_name: 'u' as i32 as gchar,
                flags: 0 as std::ffi::c_int,
                arg: G_OPTION_ARG_STRING_ARRAY,
                arg_data: &mut uris as *mut *mut *mut gchar as gpointer,
                description: b"uri(s) to load at startup\0" as *const u8 as *const std::ffi::c_char,
                arg_description: b"URI\0" as *const u8 as *const std::ffi::c_char,
            };
            init
        },
        {
            let mut init = _GOptionEntry {
                long_name: b"verbose\0" as *const u8 as *const std::ffi::c_char,
                short_name: 'v' as i32 as gchar,
                flags: 0 as std::ffi::c_int,
                arg: G_OPTION_ARG_NONE,
                arg_data: &mut verbose as *mut gboolean as gpointer,
                description: b"print verbose output\0" as *const u8 as *const std::ffi::c_char,
                arg_description: 0 as *const gchar,
            };
            init
        },
        {
            let mut init = _GOptionEntry {
                long_name: b"log\0" as *const u8 as *const std::ffi::c_char,
                short_name: 'l' as i32 as gchar,
                flags: 0 as std::ffi::c_int,
                arg: G_OPTION_ARG_STRING,
                arg_data: &mut log_lvl as *mut *mut gchar as gpointer,
                description: b"specify precise log level\0" as *const u8 as *const std::ffi::c_char,
                arg_description: b"NAME\0" as *const u8 as *const std::ffi::c_char,
            };
            init
        },
        {
            let mut init = _GOptionEntry {
                long_name: b"version\0" as *const u8 as *const std::ffi::c_char,
                short_name: 'V' as i32 as gchar,
                flags: 0 as std::ffi::c_int,
                arg: G_OPTION_ARG_NONE,
                arg_data: &mut version_only as *mut *mut gboolean as gpointer,
                description: b"print version and exit\0" as *const u8 as *const std::ffi::c_char,
                arg_description: 0 as *const gchar,
            };
            init
        },
        {
            let mut init = _GOptionEntry {
                long_name: 0 as *const gchar,
                short_name: 0 as std::ffi::c_int as gchar,
                flags: 0 as std::ffi::c_int,
                arg: G_OPTION_ARG_NONE,
                arg_data: 0 as *mut std::ffi::c_void,
                description: 0 as *const gchar,
                arg_description: 0 as *const gchar,
            };
            init
        },
    ];
    globalconf.argv =
        g_ptr_array_new_with_free_func(Some(g_free as unsafe extern "C" fn(gpointer) -> ()));
    let mut i: gint = 0 as std::ffi::c_int;
    while i < *argc {
        g_ptr_array_add(
            globalconf.argv,
            g_strdup_inline(*argv.offset(i as isize)) as gpointer,
        );
        i += 1;
        i;
    }
    context = g_option_context_new(b"[URI...]\0" as *const u8 as *const std::ffi::c_char);
    g_option_context_add_main_entries(context, entries.as_ptr(), 0 as *const gchar);
    g_option_context_add_group(context, gtk_get_option_group(0 as std::ffi::c_int));
    g_option_context_parse(context, argc, &mut argv, 0 as *mut *mut GError);
    g_option_context_free(context);
    let mut i_0: gint = 0 as std::ffi::c_int;
    while i_0 < *argc {
        while (i_0 as std::ffi::c_uint) < (*globalconf.argv).len
            && strcmp(
                *((*globalconf.argv).pdata).offset(i_0 as isize) as *const std::ffi::c_char,
                *argv.offset(i_0 as isize),
            ) == 0
        {
            g_ptr_array_remove_index(globalconf.argv, i_0 as guint);
        }
        i_0 += 1;
        i_0;
    }
    if !version_only.is_null() {
        println!("luakit {}\n\0", "64175ca2",);
        println!("  built with: webkit {}.{}.{}", 2, 48, 3,);
        println!(
            "(installed version: {}.{}.{})",
            webkit_get_major_version(),
            webkit_get_minor_version(),
            webkit_get_micro_version(),
        );
        println!("                 GTK {}.{}.{}", 3, 24, 49,);
        println!("                GLIB {}.{}.{}", 2, 84, 3,);
        println!("                SOUP {}.{}.{}", 3, 6, 5,);
        exit(0 as std::ffi::c_int);
    }
    if log_lvl.is_null() {
        log_set_verbosity(
            b"all\0" as *const u8 as *const std::ffi::c_char,
            (if verbose != 0 {
                LOG_LEVEL_verbose as std::ffi::c_int
            } else {
                LOG_LEVEL_info as std::ffi::c_int
            }) as log_level_t,
        );
    } else {
        log_set_verbosity(
            b"all\0" as *const u8 as *const std::ffi::c_char,
            LOG_LEVEL_info,
        );
        parse_log_level_option(log_lvl);
        if verbose != 0 {
            _log(
                LOG_LEVEL_warn,
                b"luakit.c\0" as *const u8 as *const std::ffi::c_char,
                b"invalid mix of -v and -l, ignoring -v...\0" as *const u8
                    as *const std::ffi::c_char,
            );
        }
    }
    if !check_only.is_null() {
        init_directories();
        luaH_init(0 as *mut *mut gchar);
        if luaH_parserc(globalconf.confpath, 0 as std::ffi::c_int) == 0 {
            g_fprintf(
                stderr,
                b"Confiuration file syntax error.\n\0" as *const u8 as *const std::ffi::c_char,
            );
            exit(1 as std::ffi::c_int);
        } else {
            g_fprintf(
                stderr,
                b"Configuration file syntax OK.\n\0" as *const u8 as *const std::ffi::c_char,
            );
            exit(0 as std::ffi::c_int);
        }
    }
    if !uris.is_null() && !(*argv.offset(1 as std::ffi::c_int as isize)).is_null() {
        _log(
            LOG_LEVEL_fatal,
            b"luakit.c\0" as *const u8 as *const std::ffi::c_char,
            b"invalid mix of -u and default uri arguments\0" as *const u8
                as *const std::ffi::c_char,
        );
    }
    if !uris.is_null() {
        return uris;
    } else {
        return g_strdupv(argv.offset(1 as std::ffi::c_int as isize));
    };
}
unsafe extern "C" fn glib_log_writer(
    mut log_level_flags: GLogLevelFlags,
    mut fields: *const GLogField,
    mut n_fields: gsize,
    mut UNUSED_user_data: gpointer,
) -> GLogWriterOutput {
    let mut log_domain: *const gchar = b"(unknown)\0" as *const u8 as *const std::ffi::c_char;
    let mut message: *const gchar = b"(empty)\0" as *const u8 as *const std::ffi::c_char;
    let mut i: gsize = 0 as std::ffi::c_int as gsize;
    while i < n_fields {
        if strcmp(
            (*fields.offset(i as isize)).key,
            b"GLIB_DOMAIN\0" as *const u8 as *const std::ffi::c_char,
        ) == 0
        {
            log_domain = (*fields.offset(i as isize)).value as *const gchar;
        }
        if strcmp(
            (*fields.offset(i as isize)).key,
            b"MESSAGE\0" as *const u8 as *const std::ffi::c_char,
        ) == 0
        {
            message = (*fields.offset(i as isize)).value as *const gchar;
        }
        i = i.wrapping_add(1);
        i;
    }
    if G_LOG_LEVEL_MASK as std::ffi::c_int & log_level_flags as std::ffi::c_int == 0 {
        return G_LOG_WRITER_UNHANDLED;
    }
    let mut log_level: log_level_t = [
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_warn,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_warn,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_info,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_verbose,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_fatal,
        LOG_LEVEL_debug,
    ][log_level_flags as usize];
    _log(
        log_level,
        b"glib\0" as *const u8 as *const std::ffi::c_char,
        b"%s: %s\0" as *const u8 as *const std::ffi::c_char,
        log_domain,
        message,
    );
    return G_LOG_WRITER_HANDLED;
}
unsafe fn main_0(mut argc: gint, mut argv: *mut *mut gchar) -> gint {
    let mut nonblock: *mut gboolean = 0 as *mut gboolean;
    globalconf.starttime = l_time();
    log_init();
    gtk_disable_setlocale();
    setlocale(
        6 as std::ffi::c_int,
        b"\0" as *const u8 as *const std::ffi::c_char,
    );
    setlocale(
        1 as std::ffi::c_int,
        b"C\0" as *const u8 as *const std::ffi::c_char,
    );
    let mut uris: *mut *mut gchar = parseopts(&mut argc, argv, &mut nonblock);
    let mut i: gint = 1 as std::ffi::c_int;
    while i < argc {
        memset(
            *argv.offset(i as isize) as *mut std::ffi::c_void,
            0 as std::ffi::c_int,
            strlen(*argv.offset(i as isize)),
        );
        i += 1;
        i;
    }
    globalconf.windows = g_ptr_array_new();
    if !nonblock.is_null() {
        let mut pid: pid_t = fork();
        if pid < 0 as std::ffi::c_int {
            _log(
                LOG_LEVEL_fatal,
                b"luakit.c\0" as *const u8 as *const std::ffi::c_char,
                b"Cannot fork: %d\0" as *const u8 as *const std::ffi::c_char,
                *__errno_location(),
            );
        } else if pid > 0 as std::ffi::c_int {
            exit(0 as std::ffi::c_int);
        }
        let mut sid: pid_t = setsid();
        if sid < 0 as std::ffi::c_int {
            _log(
                LOG_LEVEL_fatal,
                b"luakit.c\0" as *const u8 as *const std::ffi::c_char,
                b"New SID creation failure: %d\0" as *const u8 as *const std::ffi::c_char,
                *__errno_location(),
            );
        }
    }
    gtk_init(&mut argc, &mut argv);
    g_log_set_writer_func(
        Some(
            glib_log_writer
                as unsafe extern "C" fn(
                    GLogLevelFlags,
                    *const GLogField,
                    gsize,
                    gpointer,
                ) -> GLogWriterOutput,
        ),
        0 as *mut std::ffi::c_void,
        None,
    );
    init_directories();
    web_context_init();
    ipc_init();
    luaH_init(uris);
    if luaH_parserc(
        globalconf.confpath,
        (0 as std::ffi::c_int == 0) as std::ffi::c_int,
    ) == 0
    {
        _log(
            LOG_LEVEL_fatal,
            b"luakit.c\0" as *const u8 as *const std::ffi::c_char,
            b"couldn't find rc file\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if (*globalconf.windows).len == 0 {
        _log(
            LOG_LEVEL_fatal,
            b"luakit.c\0" as *const u8 as *const std::ffi::c_char,
            b"no windows spawned by rc file, exiting\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    gtk_main();
    return 0 as std::ffi::c_int;
}
pub fn main() {
    let mut args: Vec<*mut std::ffi::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as gint,
            args.as_mut_ptr() as *mut *mut gchar,
        ) as i32)
    }
}
