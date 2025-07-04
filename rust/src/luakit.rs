use ::libc;
use ::c2rust_bitfields;
#[c2rust::header_src = "/usr/lib/clang/20/include/__stddef_size_t.h:21"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = std::ffi::c_ulong;
}
#[c2rust::header_src = "/usr/lib/glib-2.0/include/glibconfig.h:21"]
pub mod glibconfig_h {
    #[c2rust::src_loc = "57:1"]
    pub type guint32 = std::ffi::c_uint;
    #[c2rust::src_loc = "82:1"]
    pub type gssize = std::ffi::c_long;
    #[c2rust::src_loc = "83:1"]
    pub type gsize = std::ffi::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:21"]
pub mod types_h {
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = std::ffi::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = std::ffi::c_long;
    #[c2rust::src_loc = "154:1"]
    pub type __pid_t = std::ffi::c_int;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = std::ffi::c_long;
    #[c2rust::src_loc = "162:1"]
    pub type __suseconds_t = std::ffi::c_long;
}
#[c2rust::header_src = "/usr/include/time.h:21"]
pub mod include_time_h {
    #[c2rust::src_loc = "54:1"]
    pub type pid_t = __pid_t;
    use super::types_h::__pid_t;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtypes.h:21"]
pub mod gtypes_h {
    #[c2rust::src_loc = "52:1"]
    pub type gchar = std::ffi::c_char;
    #[c2rust::src_loc = "55:1"]
    pub type gint = std::ffi::c_int;
    #[c2rust::src_loc = "56:1"]
    pub type gboolean = gint;
    #[c2rust::src_loc = "61:1"]
    pub type guint = std::ffi::c_uint;
    #[c2rust::src_loc = "64:1"]
    pub type gdouble = std::ffi::c_double;
    #[c2rust::src_loc = "109:1"]
    pub type gpointer = *mut std::ffi::c_void;
    #[c2rust::src_loc = "110:1"]
    pub type gconstpointer = *const std::ffi::c_void;
    #[c2rust::src_loc = "140:1"]
    pub type GDestroyNotify = Option::<unsafe extern "C" fn(gpointer) -> ()>;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/garray.h:21"]
pub mod garray_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "55:8"]
    pub struct _GPtrArray {
        pub pdata: *mut gpointer,
        pub len: guint,
    }
    #[c2rust::src_loc = "41:1"]
    pub type GPtrArray = _GPtrArray;
    use super::gtypes_h::{gpointer, guint, GDestroyNotify};
    extern "C" {
        #[c2rust::src_loc = "150:1"]
        pub fn g_ptr_array_new() -> *mut GPtrArray;
        #[c2rust::src_loc = "152:1"]
        pub fn g_ptr_array_new_with_free_func(
            element_free_func: GDestroyNotify,
        ) -> *mut GPtrArray;
        #[c2rust::src_loc = "201:1"]
        pub fn g_ptr_array_remove_index(
            array: *mut GPtrArray,
            index_: guint,
        ) -> gpointer;
        #[c2rust::src_loc = "223:1"]
        pub fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gquark.h:21"]
pub mod gquark_h {
    #[c2rust::src_loc = "38:1"]
    pub type GQuark = guint32;
    use super::glibconfig_h::guint32;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gerror.h:21"]
pub mod gerror_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "45:8"]
    pub struct _GError {
        pub domain: GQuark,
        pub code: gint,
        pub message: *mut gchar,
    }
    #[c2rust::src_loc = "43:1"]
    pub type GError = _GError;
    use super::gquark_h::GQuark;
    use super::gtypes_h::{gint, gchar};
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gdataset.h:21"]
pub mod gdataset_h {
    #[c2rust::src_loc = "38:1"]
    pub type GData = _GData;
    extern "C" {
        #[c2rust::src_loc = "38:16"]
        pub type _GData;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/ghash.h:21"]
pub mod ghash_h {
    #[c2rust::src_loc = "40:1"]
    pub type GHashTable = _GHashTable;
    extern "C" {
        #[c2rust::src_loc = "40:16"]
        pub type _GHashTable;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gmessages.h:21"]
pub mod gmessages_h {
    #[c2rust::src_loc = "55:9"]
    pub type GLogLevelFlags = std::ffi::c_int;
    #[c2rust::src_loc = "69:3"]
    pub const G_LOG_LEVEL_MASK: GLogLevelFlags = -4;
    #[c2rust::src_loc = "67:3"]
    pub const G_LOG_LEVEL_DEBUG: GLogLevelFlags = 128;
    #[c2rust::src_loc = "66:3"]
    pub const G_LOG_LEVEL_INFO: GLogLevelFlags = 64;
    #[c2rust::src_loc = "65:3"]
    pub const G_LOG_LEVEL_MESSAGE: GLogLevelFlags = 32;
    #[c2rust::src_loc = "64:3"]
    pub const G_LOG_LEVEL_WARNING: GLogLevelFlags = 16;
    #[c2rust::src_loc = "63:3"]
    pub const G_LOG_LEVEL_CRITICAL: GLogLevelFlags = 8;
    #[c2rust::src_loc = "62:3"]
    pub const G_LOG_LEVEL_ERROR: GLogLevelFlags = 4;
    #[c2rust::src_loc = "59:3"]
    pub const G_LOG_FLAG_FATAL: GLogLevelFlags = 2;
    #[c2rust::src_loc = "58:3"]
    pub const G_LOG_FLAG_RECURSION: GLogLevelFlags = 1;
    #[c2rust::src_loc = "136:9"]
    pub type GLogWriterOutput = std::ffi::c_uint;
    #[c2rust::src_loc = "139:3"]
    pub const G_LOG_WRITER_UNHANDLED: GLogWriterOutput = 0;
    #[c2rust::src_loc = "138:3"]
    pub const G_LOG_WRITER_HANDLED: GLogWriterOutput = 1;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "159:8"]
    pub struct _GLogField {
        pub key: *const gchar,
        pub value: gconstpointer,
        pub length: gssize,
    }
    #[c2rust::src_loc = "158:1"]
    pub type GLogField = _GLogField;
    #[c2rust::src_loc = "197:1"]
    pub type GLogWriterFunc = Option::<
        unsafe extern "C" fn(
            GLogLevelFlags,
            *const GLogField,
            gsize,
            gpointer,
        ) -> GLogWriterOutput,
    >;
    use super::gtypes_h::{gchar, gconstpointer, gpointer, GDestroyNotify};
    use super::glibconfig_h::{gssize, gsize};
    extern "C" {
        #[c2rust::src_loc = "216:1"]
        pub fn g_log_set_writer_func(
            func: GLogWriterFunc,
            user_data: gpointer,
            user_data_free: GDestroyNotify,
        );
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/goption.h:21"]
pub mod goption_h {
    #[c2rust::src_loc = "40:1"]
    pub type GOptionContext = _GOptionContext;
    #[c2rust::src_loc = "53:1"]
    pub type GOptionGroup = _GOptionGroup;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "283:8"]
    pub struct _GOptionEntry {
        pub long_name: *const gchar,
        pub short_name: gchar,
        pub flags: gint,
        pub arg: GOptionArg,
        pub arg_data: gpointer,
        pub description: *const gchar,
        pub arg_description: *const gchar,
    }
    #[c2rust::src_loc = "142:9"]
    pub type GOptionArg = std::ffi::c_uint;
    #[c2rust::src_loc = "152:3"]
    pub const G_OPTION_ARG_INT64: GOptionArg = 8;
    #[c2rust::src_loc = "151:3"]
    pub const G_OPTION_ARG_DOUBLE: GOptionArg = 7;
    #[c2rust::src_loc = "150:3"]
    pub const G_OPTION_ARG_FILENAME_ARRAY: GOptionArg = 6;
    #[c2rust::src_loc = "149:3"]
    pub const G_OPTION_ARG_STRING_ARRAY: GOptionArg = 5;
    #[c2rust::src_loc = "148:3"]
    pub const G_OPTION_ARG_FILENAME: GOptionArg = 4;
    #[c2rust::src_loc = "147:3"]
    pub const G_OPTION_ARG_CALLBACK: GOptionArg = 3;
    #[c2rust::src_loc = "146:3"]
    pub const G_OPTION_ARG_INT: GOptionArg = 2;
    #[c2rust::src_loc = "145:3"]
    pub const G_OPTION_ARG_STRING: GOptionArg = 1;
    #[c2rust::src_loc = "144:3"]
    pub const G_OPTION_ARG_NONE: GOptionArg = 0;
    #[c2rust::src_loc = "54:1"]
    pub type GOptionEntry = _GOptionEntry;
    use super::gtypes_h::{gchar, gint, gpointer, gboolean};
    use super::gerror_h::GError;
    extern "C" {
        #[c2rust::src_loc = "40:16"]
        pub type _GOptionContext;
        #[c2rust::src_loc = "53:16"]
        pub type _GOptionGroup;
        #[c2rust::src_loc = "332:1"]
        pub fn g_option_context_new(
            parameter_string: *const gchar,
        ) -> *mut GOptionContext;
        #[c2rust::src_loc = "344:1"]
        pub fn g_option_context_free(context: *mut GOptionContext);
        #[c2rust::src_loc = "363:1"]
        pub fn g_option_context_add_main_entries(
            context: *mut GOptionContext,
            entries: *const GOptionEntry,
            translation_domain: *const gchar,
        );
        #[c2rust::src_loc = "367:1"]
        pub fn g_option_context_parse(
            context: *mut GOptionContext,
            argc: *mut gint,
            argv: *mut *mut *mut gchar,
            error: *mut *mut GError,
        ) -> gboolean;
        #[c2rust::src_loc = "385:1"]
        pub fn g_option_context_add_group(
            context: *mut GOptionContext,
            group: *mut GOptionGroup,
        );
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtree.h:21"]
pub mod gtree_h {
    #[c2rust::src_loc = "40:1"]
    pub type GTree = _GTree;
    extern "C" {
        #[c2rust::src_loc = "40:16"]
        pub type _GTree;
    }
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lua.h:21"]
pub mod lua_h {
    extern "C" {
        #[c2rust::src_loc = "51:16"]
        pub type lua_State;
    }
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timeval.h:21"]
pub mod struct_timeval_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "8:8"]
    pub struct timeval {
        pub tv_sec: __time_t,
        pub tv_usec: __suseconds_t,
    }
    use super::types_h::{__time_t, __suseconds_t};
}
#[c2rust::header_src = "/home/daana/git/luakit/common/log.h:21"]
pub mod log_h {
    #[c2rust::src_loc = "36:9"]
    pub type log_level_t = std::ffi::c_uint;
    #[c2rust::src_loc = "36:16"]
    pub const LOG_LEVEL_debug: log_level_t = 5;
    #[c2rust::src_loc = "36:16"]
    pub const LOG_LEVEL_verbose: log_level_t = 4;
    #[c2rust::src_loc = "36:16"]
    pub const LOG_LEVEL_info: log_level_t = 3;
    #[c2rust::src_loc = "36:16"]
    pub const LOG_LEVEL_warn: log_level_t = 2;
    #[c2rust::src_loc = "36:16"]
    pub const LOG_LEVEL_error: log_level_t = 1;
    #[c2rust::src_loc = "36:16"]
    pub const LOG_LEVEL_fatal: log_level_t = 0;
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "54:1"]
        pub fn _log(lvl: log_level_t, _: *const gchar, _: *const gchar, _: ...);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/common.h:22"]
pub mod common_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "24:16"]
    pub struct _common_t {
        pub L: *mut lua_State,
    }
    #[c2rust::src_loc = "24:1"]
    pub type common_t = _common_t;
    use super::lua_h::lua_State;
}
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gtype.h:22"]
pub mod gtype_h {
    #[c2rust::src_loc = "427:1"]
    pub type GType = gsize;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "451:8"]
    pub struct _GTypeClass {
        pub g_type: GType,
    }
    #[c2rust::src_loc = "434:1"]
    pub type GTypeClass = _GTypeClass;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "461:8"]
    pub struct _GTypeInstance {
        pub g_class: *mut GTypeClass,
    }
    #[c2rust::src_loc = "436:1"]
    pub type GTypeInstance = _GTypeInstance;
    use super::glibconfig_h::gsize;
}
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gobject.h:22"]
pub mod gobject_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "252:9"]
    pub struct _GObject {
        pub g_type_instance: GTypeInstance,
        pub ref_count: guint,
        pub qdata: *mut GData,
    }
    #[c2rust::src_loc = "192:1"]
    pub type GObject = _GObject;
    use super::gtype_h::GTypeInstance;
    use super::gtypes_h::guint;
    use super::gdataset_h::GData;
}
#[c2rust::header_src = "/usr/include/glib-2.0/gio/gapplication.h:22"]
pub mod gapplication_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "46:8"]
    pub struct _GApplication {
        pub parent_instance: GObject,
        pub priv_0: *mut GApplicationPrivate,
    }
    #[c2rust::src_loc = "43:1"]
    pub type GApplicationPrivate = _GApplicationPrivate;
    use super::gobject_h::GObject;
    extern "C" {
        #[c2rust::src_loc = "43:16"]
        pub type _GApplicationPrivate;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/gio/giotypes.h:22"]
pub mod giotypes_h {
    #[c2rust::src_loc = "59:1"]
    pub type GApplication = _GApplication;
    use super::gapplication_h::_GApplication;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:22"]
pub mod struct_FILE_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "50:8"]
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
    #[c2rust::src_loc = "44:1"]
    pub type _IO_lock_t = ();
    use super::types_h::{__off_t, __off64_t};
    extern "C" {
        #[c2rust::src_loc = "39:8"]
        pub type _IO_wide_data;
        #[c2rust::src_loc = "38:8"]
        pub type _IO_codecvt;
        #[c2rust::src_loc = "37:8"]
        pub type _IO_marker;
    }
}
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:22"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gtk/gtkapplication.h:22"]
pub mod gtkapplication_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "43:8"]
    pub struct _GtkApplication {
        pub parent: GApplication,
        pub priv_0: *mut GtkApplicationPrivate,
    }
    #[c2rust::src_loc = "41:1"]
    pub type GtkApplicationPrivate = _GtkApplicationPrivate;
    #[c2rust::src_loc = "39:1"]
    pub type GtkApplication = _GtkApplication;
    use super::giotypes_h::GApplication;
    extern "C" {
        #[c2rust::src_loc = "41:16"]
        pub type _GtkApplicationPrivate;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/globalconf.h:22"]
pub mod globalconf_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "32:9"]
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
    use super::gtkapplication_h::GtkApplication;
    use super::gtypes_h::{gchar, gboolean, gdouble};
    use super::garray_h::GPtrArray;
}
#[c2rust::header_src = "/home/daana/git/luakit/common/signal.h:23"]
pub mod signal_h {
    #[c2rust::src_loc = "29:1"]
    pub type signal_t = GTree;
    use super::gtree_h::GTree;
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luaclass.h:23"]
pub mod luaclass_h {
    #[c2rust::src_loc = "32:1"]
    pub type lua_class_property_array_t = GHashTable;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "39:9"]
    pub struct lua_object_t {
        pub signals: *mut signal_t,
    }
    #[c2rust::src_loc = "43:1"]
    pub type lua_class_allocator_t = Option::<
        unsafe extern "C" fn(*mut lua_State) -> *mut lua_object_t,
    >;
    #[c2rust::src_loc = "45:1"]
    pub type lua_class_propfunc_t = Option::<
        unsafe extern "C" fn(*mut lua_State, *mut lua_object_t) -> gint,
    >;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "47:9"]
    pub struct lua_class_t {
        pub name: *const gchar,
        pub signals: *mut signal_t,
        pub allocator: lua_class_allocator_t,
        pub properties: *mut lua_class_property_array_t,
        pub index_miss_property: lua_class_propfunc_t,
        pub newindex_miss_property: lua_class_propfunc_t,
    }
    use super::ghash_h::GHashTable;
    use super::signal_h::signal_t;
    use super::lua_h::lua_State;
    use super::gtypes_h::{gint, gchar};
}
#[c2rust::header_src = "/usr/include/string.h:21"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "43:14"]
        pub fn memcpy(
            _: *mut std::ffi::c_void,
            _: *const std::ffi::c_void,
            _: std::ffi::c_ulong,
        ) -> *mut std::ffi::c_void;
        #[c2rust::src_loc = "61:14"]
        pub fn memset(
            _: *mut std::ffi::c_void,
            _: std::ffi::c_int,
            _: std::ffi::c_ulong,
        ) -> *mut std::ffi::c_void;
        #[c2rust::src_loc = "156:12"]
        pub fn strcmp(
            _: *const std::ffi::c_char,
            _: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "246:14"]
        pub fn strchr(
            _: *const std::ffi::c_char,
            _: std::ffi::c_int,
        ) -> *mut std::ffi::c_char;
        #[c2rust::src_loc = "407:15"]
        pub fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gutils.h:21"]
pub mod gutils_h {
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "183:1"]
        pub fn g_get_user_data_dir() -> *const gchar;
        #[c2rust::src_loc = "185:1"]
        pub fn g_get_user_config_dir() -> *const gchar;
        #[c2rust::src_loc = "187:1"]
        pub fn g_get_user_cache_dir() -> *const gchar;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:21"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "756:13"]
        pub fn exit(_: std::ffi::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gfileutils.h:21"]
pub mod gfileutils_h {
    use super::gtypes_h::{gchar, gint};
    extern "C" {
        #[c2rust::src_loc = "174:1"]
        pub fn g_build_filename(first_element: *const gchar, _: ...) -> *mut gchar;
        #[c2rust::src_loc = "183:1"]
        pub fn g_mkdir_with_parents(pathname: *const gchar, mode: gint) -> gint;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gmem.h:21"]
pub mod gmem_h {
    use super::gtypes_h::gpointer;
    use super::glibconfig_h::gsize;
    extern "C" {
        #[c2rust::src_loc = "73:1"]
        pub fn g_free(mem: gpointer);
        #[c2rust::src_loc = "83:1"]
        pub fn g_malloc(n_bytes: gsize) -> gpointer;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gstrfuncs.h:21"]
pub mod gstrfuncs_h {
    #[inline(always)]
    #[c2rust::src_loc = "308:1"]
    pub unsafe extern "C" fn g_strdup_inline(
        mut str: *const std::ffi::c_char,
    ) -> *mut std::ffi::c_char {
        if 0 != 0 && str.is_null() {
            return 0 as *mut std::ffi::c_char;
        }
        if 0 != 0 && !str.is_null() && 0 != 0 {
            let len: size_t = (strlen(str))
                .wrapping_add(1 as std::ffi::c_int as std::ffi::c_ulong);
            let mut dup_str: *mut std::ffi::c_char = g_malloc(len)
                as *mut std::ffi::c_char;
            return memcpy(
                dup_str as *mut std::ffi::c_void,
                str as *const std::ffi::c_void,
                len,
            ) as *mut std::ffi::c_char;
        }
        return g_strdup(str);
    }
    use super::gtypes_h::{gchar, gint};
    use super::string_h::{strlen, memcpy};
    use super::__stddef_size_t_h::size_t;
    use super::gmem_h::g_malloc;
    extern "C" {
        #[c2rust::src_loc = "283:1"]
        pub fn g_strdup(str: *const gchar) -> *mut gchar;
        #[c2rust::src_loc = "355:1"]
        pub fn g_strsplit(
            string: *const gchar,
            delimiter: *const gchar,
            max_tokens: gint,
        ) -> *mut *mut gchar;
        #[c2rust::src_loc = "366:1"]
        pub fn g_strfreev(str_array: *mut *mut gchar);
        #[c2rust::src_loc = "368:1"]
        pub fn g_strdupv(str_array: *mut *mut gchar) -> *mut *mut gchar;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:21"]
pub mod errno_h {
    extern "C" {
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut std::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:21"]
pub mod unistd_h {
    use super::types_h::__pid_t;
    extern "C" {
        #[c2rust::src_loc = "689:1"]
        pub fn setsid() -> __pid_t;
        #[c2rust::src_loc = "778:1"]
        pub fn fork() -> __pid_t;
    }
}
#[c2rust::header_src = "/usr/include/sys/time.h:21"]
pub mod time_h {
    use super::struct_timeval_h::timeval;
    extern "C" {
        #[c2rust::src_loc = "67:1"]
        pub fn gettimeofday(
            __tv: *mut timeval,
            __tz: *mut std::ffi::c_void,
        ) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/util.h:21"]
pub mod util_h {
    #[inline]
    #[c2rust::src_loc = "63:1"]
    pub unsafe extern "C" fn l_time() -> gdouble {
        let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
        gettimeofday(&mut tv, 0 as *mut std::ffi::c_void);
        return tv.tv_sec as std::ffi::c_double
            + tv.tv_usec as std::ffi::c_double / 1e6f64;
    }
    use super::gtypes_h::gdouble;
    use super::struct_timeval_h::timeval;
    use super::types_h::{__time_t, __suseconds_t};
    use super::time_h::gettimeofday;
}
#[c2rust::header_src = "/usr/include/stdio.h:22"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[c2rust::src_loc = "151:14"]
        pub static mut stderr: *mut FILE;
    }
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gtk/gtkmain.h:22"]
pub mod gtkmain_h {
    use super::gtypes_h::gboolean;
    use super::goption_h::GOptionGroup;
    extern "C" {
        #[c2rust::src_loc = "100:1"]
        pub fn gtk_init(
            argc: *mut std::ffi::c_int,
            argv: *mut *mut *mut std::ffi::c_char,
        );
        #[c2rust::src_loc = "116:1"]
        pub fn gtk_get_option_group(open_default_display: gboolean) -> *mut GOptionGroup;
        #[c2rust::src_loc = "142:1"]
        pub fn gtk_disable_setlocale();
        #[c2rust::src_loc = "153:1"]
        pub fn gtk_main();
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gprintf.h:23"]
pub mod gprintf_h {
    use super::gtypes_h::{gchar, gint};
    use super::FILE_h::FILE;
    extern "C" {
        #[c2rust::src_loc = "29:1"]
        pub fn g_printf(format: *const gchar, _: ...) -> gint;
        #[c2rust::src_loc = "32:1"]
        pub fn g_fprintf(file: *mut FILE, format: *const gchar, _: ...) -> gint;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/luah.h:23"]
pub mod luah_h {
    use super::gtypes_h::{gchar, gboolean};
    extern "C" {
        #[c2rust::src_loc = "27:1"]
        pub fn luaH_init(_: *mut *mut gchar);
        #[c2rust::src_loc = "28:1"]
        pub fn luaH_parserc(_: *const gchar, _: gboolean) -> gboolean;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/ipc.h:24"]
pub mod ipc_h {
    extern "C" {
        #[c2rust::src_loc = "24:1"]
        pub fn ipc_init();
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/log.h:25"]
pub mod luakit_log_h {
    use super::log_h::log_level_t;
    extern "C" {
        #[c2rust::src_loc = "26:1"]
        pub fn log_init();
        #[c2rust::src_loc = "27:1"]
        pub fn log_level_from_string(
            out: *mut log_level_t,
            str: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "28:1"]
        pub fn log_set_verbosity(group: *const std::ffi::c_char, lvl: log_level_t);
    }
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkit/WebKitVersion.h:26"]
pub mod WebKitVersion_h {
    use super::gtypes_h::guint;
    extern "C" {
        #[c2rust::src_loc = "41:1"]
        pub fn webkit_get_major_version() -> guint;
        #[c2rust::src_loc = "44:1"]
        pub fn webkit_get_minor_version() -> guint;
        #[c2rust::src_loc = "47:1"]
        pub fn webkit_get_micro_version() -> guint;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/web_context.h:26"]
pub mod web_context_h {
    extern "C" {
        #[c2rust::src_loc = "26:1"]
        pub fn web_context_init();
    }
}
#[c2rust::header_src = "/usr/include/locale.h:30"]
pub mod locale_h {
    extern "C" {
        #[c2rust::src_loc = "122:1"]
        pub fn setlocale(
            __category: std::ffi::c_int,
            __locale: *const std::ffi::c_char,
        ) -> *mut std::ffi::c_char;
    }
}
pub use self::__stddef_size_t_h::size_t;
pub use self::glibconfig_h::{guint32, gssize, gsize};
pub use self::types_h::{__off_t, __off64_t, __pid_t, __time_t, __suseconds_t};
pub use self::include_time_h::pid_t;
pub use self::gtypes_h::{
    gchar, gint, gboolean, guint, gdouble, gpointer, gconstpointer, GDestroyNotify,
};
pub use self::garray_h::{
    _GPtrArray, GPtrArray, g_ptr_array_new, g_ptr_array_new_with_free_func,
    g_ptr_array_remove_index, g_ptr_array_add,
};
pub use self::gquark_h::GQuark;
pub use self::gerror_h::{_GError, GError};
pub use self::gdataset_h::{GData, _GData};
pub use self::ghash_h::{GHashTable, _GHashTable};
pub use self::gmessages_h::{
    GLogLevelFlags, G_LOG_LEVEL_MASK, G_LOG_LEVEL_DEBUG, G_LOG_LEVEL_INFO,
    G_LOG_LEVEL_MESSAGE, G_LOG_LEVEL_WARNING, G_LOG_LEVEL_CRITICAL, G_LOG_LEVEL_ERROR,
    G_LOG_FLAG_FATAL, G_LOG_FLAG_RECURSION, GLogWriterOutput, G_LOG_WRITER_UNHANDLED,
    G_LOG_WRITER_HANDLED, _GLogField, GLogField, GLogWriterFunc, g_log_set_writer_func,
};
pub use self::goption_h::{
    GOptionContext, GOptionGroup, _GOptionEntry, GOptionArg, G_OPTION_ARG_INT64,
    G_OPTION_ARG_DOUBLE, G_OPTION_ARG_FILENAME_ARRAY, G_OPTION_ARG_STRING_ARRAY,
    G_OPTION_ARG_FILENAME, G_OPTION_ARG_CALLBACK, G_OPTION_ARG_INT, G_OPTION_ARG_STRING,
    G_OPTION_ARG_NONE, GOptionEntry, _GOptionContext, _GOptionGroup,
    g_option_context_new, g_option_context_free, g_option_context_add_main_entries,
    g_option_context_parse, g_option_context_add_group,
};
pub use self::gtree_h::{GTree, _GTree};
use self::lua_h::lua_State;
pub use self::struct_timeval_h::timeval;
pub use self::log_h::{
    log_level_t, LOG_LEVEL_debug, LOG_LEVEL_verbose, LOG_LEVEL_info, LOG_LEVEL_warn,
    LOG_LEVEL_error, LOG_LEVEL_fatal, _log,
};
pub use self::common_h::{_common_t, common_t};
pub use self::gtype_h::{GType, _GTypeClass, GTypeClass, _GTypeInstance, GTypeInstance};
pub use self::gobject_h::{_GObject, GObject};
pub use self::gapplication_h::{_GApplication, GApplicationPrivate, _GApplicationPrivate};
pub use self::giotypes_h::GApplication;
pub use self::struct_FILE_h::{
    _IO_FILE, _IO_lock_t, _IO_wide_data, _IO_codecvt, _IO_marker,
};
pub use self::FILE_h::FILE;
pub use self::gtkapplication_h::{
    _GtkApplication, GtkApplicationPrivate, GtkApplication, _GtkApplicationPrivate,
};
pub use self::globalconf_h::globalconf_t;
pub use self::signal_h::signal_t;
pub use self::luaclass_h::{
    lua_class_property_array_t, lua_object_t, lua_class_allocator_t,
    lua_class_propfunc_t, lua_class_t,
};
use self::string_h::{memcpy, memset, strcmp, strchr, strlen};
use self::gutils_h::{g_get_user_data_dir, g_get_user_config_dir, g_get_user_cache_dir};
use self::stdlib_h::exit;
use self::gfileutils_h::{g_build_filename, g_mkdir_with_parents};
use self::gmem_h::{g_free, g_malloc};
pub use self::gstrfuncs_h::{
    g_strdup_inline, g_strdup, g_strsplit, g_strfreev, g_strdupv,
};
use self::errno_h::__errno_location;
use self::unistd_h::{setsid, fork};
use self::time_h::gettimeofday;
pub use self::util_h::l_time;
use self::stdio_h::stderr;
use self::gtkmain_h::{gtk_init, gtk_get_option_group, gtk_disable_setlocale, gtk_main};
use self::gprintf_h::{g_printf, g_fprintf};
use self::luah_h::{luaH_init, luaH_parserc};
use self::ipc_h::ipc_init;
use self::luakit_log_h::{log_init, log_level_from_string, log_set_verbosity};
use self::WebKitVersion_h::{
    webkit_get_major_version, webkit_get_minor_version, webkit_get_micro_version,
};
use self::web_context_h::web_context_init;
use self::locale_h::setlocale;
#[no_mangle]
#[c2rust::src_loc = "45:10"]
pub static mut common: common_t = _common_t {
    L: 0 as *const lua_State as *mut lua_State,
};
#[no_mangle]
#[c2rust::src_loc = "46:14"]
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
#[no_mangle]
#[c2rust::src_loc = "47:13"]
pub static mut widget_class: lua_class_t = lua_class_t {
    name: 0 as *const gchar,
    signals: 0 as *const signal_t as *mut signal_t,
    allocator: None,
    properties: 0 as *const lua_class_property_array_t
        as *mut lua_class_property_array_t,
    index_miss_property: None,
    newindex_miss_property: None,
};
#[c2rust::src_loc = "49:1"]
unsafe extern "C" fn init_directories() {
    globalconf
        .cache_dir = g_build_filename(
        g_get_user_cache_dir(),
        b"luakit\0" as *const u8 as *const std::ffi::c_char,
        globalconf.profile,
        0 as *mut std::ffi::c_void,
    );
    globalconf
        .config_dir = g_build_filename(
        g_get_user_config_dir(),
        b"luakit\0" as *const u8 as *const std::ffi::c_char,
        globalconf.profile,
        0 as *mut std::ffi::c_void,
    );
    globalconf
        .data_dir = g_build_filename(
        g_get_user_data_dir(),
        b"luakit\0" as *const u8 as *const std::ffi::c_char,
        globalconf.profile,
        0 as *mut std::ffi::c_void,
    );
    g_mkdir_with_parents(globalconf.cache_dir, 0o700 as std::ffi::c_int);
    g_mkdir_with_parents(globalconf.config_dir, 0o700 as std::ffi::c_int);
    g_mkdir_with_parents(globalconf.data_dir, 0o700 as std::ffi::c_int);
}
#[c2rust::src_loc = "61:1"]
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
                && log_level_from_string(
                    &mut lvl,
                    sep.offset(1 as std::ffi::c_int as isize),
                ) == 0
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
#[c2rust::src_loc = "83:1"]
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
                description: b"check config and exit\0" as *const u8
                    as *const std::ffi::c_char,
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
                description: b"configuration file to use\0" as *const u8
                    as *const std::ffi::c_char,
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
                description: b"profile name to use\0" as *const u8
                    as *const std::ffi::c_char,
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
                description: b"run in background\0" as *const u8
                    as *const std::ffi::c_char,
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
                description: b"ignore libunique bindings\0" as *const u8
                    as *const std::ffi::c_char,
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
                description: b"uri(s) to load at startup\0" as *const u8
                    as *const std::ffi::c_char,
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
                description: b"print verbose output\0" as *const u8
                    as *const std::ffi::c_char,
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
                description: b"specify precise log level\0" as *const u8
                    as *const std::ffi::c_char,
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
                description: b"print version and exit\0" as *const u8
                    as *const std::ffi::c_char,
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
    globalconf
        .argv = g_ptr_array_new_with_free_func(
        Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
    );
    let mut i: gint = 0 as std::ffi::c_int;
    while i < *argc {
        g_ptr_array_add(
            globalconf.argv,
            g_strdup_inline(*argv.offset(i as isize)) as gpointer,
        );
        i += 1;
        i;
    }
    context = g_option_context_new(
        b"[URI...]\0" as *const u8 as *const std::ffi::c_char,
    );
    g_option_context_add_main_entries(context, entries.as_ptr(), 0 as *const gchar);
    g_option_context_add_group(context, gtk_get_option_group(0 as std::ffi::c_int));
    g_option_context_parse(context, argc, &mut argv, 0 as *mut *mut GError);
    g_option_context_free(context);
    let mut i_0: gint = 0 as std::ffi::c_int;
    while i_0 < *argc {
        while (i_0 as std::ffi::c_uint) < (*globalconf.argv).len
            && strcmp(
                *((*globalconf.argv).pdata).offset(i_0 as isize)
                    as *const std::ffi::c_char,
                *argv.offset(i_0 as isize),
            ) == 0
        {
            g_ptr_array_remove_index(globalconf.argv, i_0 as guint);
        }
        i_0 += 1;
        i_0;
    }
    if !version_only.is_null() {
        g_printf(
            b"luakit %s\n\0" as *const u8 as *const std::ffi::c_char,
            b"64175ca2\0" as *const u8 as *const std::ffi::c_char,
        );
        g_printf(
            b"  built with: webkit %i.%i.%i \0" as *const u8 as *const std::ffi::c_char,
            2 as std::ffi::c_int,
            48 as std::ffi::c_int,
            3 as std::ffi::c_int,
        );
        g_printf(
            b"(installed version: %u.%u.%u)\n\0" as *const u8 as *const std::ffi::c_char,
            webkit_get_major_version(),
            webkit_get_minor_version(),
            webkit_get_micro_version(),
        );
        g_printf(
            b"                 GTK %i.%i.%i \n\0" as *const u8
                as *const std::ffi::c_char,
            3 as std::ffi::c_int,
            24 as std::ffi::c_int,
            49 as std::ffi::c_int,
        );
        g_printf(
            b"                GLIB %i.%i.%i \n\0" as *const u8
                as *const std::ffi::c_char,
            2 as std::ffi::c_int,
            84 as std::ffi::c_int,
            3 as std::ffi::c_int,
        );
        g_printf(
            b"                SOUP %i.%i.%i \n\0" as *const u8
                as *const std::ffi::c_char,
            3 as std::ffi::c_int,
            6 as std::ffi::c_int,
            5 as std::ffi::c_int,
        );
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
                b"Confiuration file syntax error.\n\0" as *const u8
                    as *const std::ffi::c_char,
            );
            exit(1 as std::ffi::c_int);
        } else {
            g_fprintf(
                stderr,
                b"Configuration file syntax OK.\n\0" as *const u8
                    as *const std::ffi::c_char,
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
        return uris
    } else {
        return g_strdupv(argv.offset(1 as std::ffi::c_int as isize))
    };
}
#[c2rust::src_loc = "174:1"]
unsafe extern "C" fn glib_log_writer(
    mut log_level_flags: GLogLevelFlags,
    mut fields: *const GLogField,
    mut n_fields: gsize,
    mut UNUSED_user_data: gpointer,
) -> GLogWriterOutput {
    let mut log_domain: *const gchar = b"(unknown)\0" as *const u8
        as *const std::ffi::c_char;
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
#[c2rust::src_loc = "203:1"]
unsafe fn main_0(mut argc: gint, mut argv: *mut *mut gchar) -> gint {
    let mut nonblock: *mut gboolean = 0 as *mut gboolean;
    globalconf.starttime = l_time();
    log_init();
    gtk_disable_setlocale();
    setlocale(6 as std::ffi::c_int, b"\0" as *const u8 as *const std::ffi::c_char);
    setlocale(1 as std::ffi::c_int, b"C\0" as *const u8 as *const std::ffi::c_char);
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
                b"New SID creation failure: %d\0" as *const u8
                    as *const std::ffi::c_char,
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
    if luaH_parserc(globalconf.confpath, (0 as std::ffi::c_int == 0) as std::ffi::c_int)
        == 0
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
            b"no windows spawned by rc file, exiting\0" as *const u8
                as *const std::ffi::c_char,
        );
    }
    gtk_main();
    return 0 as std::ffi::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut std::ffi::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0((args.len() - 1) as gint, args.as_mut_ptr() as *mut *mut gchar) as i32,
        )
    }
}
