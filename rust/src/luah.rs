use ::libc;
#[c2rust::header_src = "/usr/lib/clang/20/include/__stddef_size_t.h:22"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = std::ffi::c_ulong;
}
#[c2rust::header_src = "/usr/lib/glib-2.0/include/glibconfig.h:22"]
pub mod glibconfig_h {
    #[c2rust::src_loc = "57:1"]
    pub type guint32 = std::ffi::c_uint;
    #[c2rust::src_loc = "83:1"]
    pub type gsize = std::ffi::c_ulong;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtypes.h:22"]
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
    #[c2rust::src_loc = "140:1"]
    pub type GDestroyNotify = Option::<unsafe extern "C" fn(gpointer) -> ()>;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/garray.h:22"]
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
    use super::gtypes_h::{gpointer, guint, GDestroyNotify, gboolean, gint};
    extern "C" {
        #[c2rust::src_loc = "152:1"]
        pub fn g_ptr_array_new_with_free_func(
            element_free_func: GDestroyNotify,
        ) -> *mut GPtrArray;
        #[c2rust::src_loc = "188:1"]
        pub fn g_ptr_array_free(
            array: *mut GPtrArray,
            free_segment: gboolean,
        ) -> *mut gpointer;
        #[c2rust::src_loc = "223:1"]
        pub fn g_ptr_array_add(array: *mut GPtrArray, data: gpointer);
        #[c2rust::src_loc = "234:1"]
        pub fn g_ptr_array_insert(array: *mut GPtrArray, index_: gint, data: gpointer);
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gdataset.h:22"]
pub mod gdataset_h {
    #[c2rust::src_loc = "38:1"]
    pub type GData = _GData;
    extern "C" {
        #[c2rust::src_loc = "38:16"]
        pub type _GData;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gunicode.h:22"]
pub mod gunicode_h {
    #[c2rust::src_loc = "61:1"]
    pub type gunichar = guint32;
    use super::glibconfig_h::guint32;
    use super::gtypes_h::{gboolean, gchar, gint};
    extern "C" {
        #[c2rust::src_loc = "698:1"]
        pub fn g_unichar_isgraph(c: gunichar) -> gboolean;
        #[c2rust::src_loc = "921:1"]
        pub fn g_unichar_to_utf8(c: gunichar, outbuf: *mut gchar) -> gint;
    }
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lua.h:22"]
pub mod lua_h {
    #[c2rust::src_loc = "53:1"]
    pub type lua_CFunction = Option::<
        unsafe extern "C" fn(*mut lua_State) -> std::ffi::c_int,
    >;
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "51:16"]
        pub type lua_State;
        #[c2rust::src_loc = "115:1"]
        pub fn lua_atpanic(L: *mut lua_State, panicf: lua_CFunction) -> lua_CFunction;
        #[c2rust::src_loc = "121:1"]
        pub fn lua_gettop(L: *mut lua_State) -> std::ffi::c_int;
        #[c2rust::src_loc = "122:1"]
        pub fn lua_settop(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "124:1"]
        pub fn lua_remove(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "125:1"]
        pub fn lua_insert(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "150:1"]
        pub fn lua_tolstring(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            len: *mut size_t,
        ) -> *const std::ffi::c_char;
        #[c2rust::src_loc = "165:1"]
        pub fn lua_pushstring(L: *mut lua_State, s: *const std::ffi::c_char);
        #[c2rust::src_loc = "169:1"]
        pub fn lua_pushcclosure(
            L: *mut lua_State,
            fn_0: lua_CFunction,
            n: std::ffi::c_int,
        );
        #[c2rust::src_loc = "182:1"]
        pub fn lua_createtable(
            L: *mut lua_State,
            narr: std::ffi::c_int,
            nrec: std::ffi::c_int,
        );
        #[c2rust::src_loc = "192:1"]
        pub fn lua_setfield(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            k: *const std::ffi::c_char,
        );
        #[c2rust::src_loc = "194:1"]
        pub fn lua_rawseti(L: *mut lua_State, idx: std::ffi::c_int, n: std::ffi::c_int);
        #[c2rust::src_loc = "203:1"]
        pub fn lua_pcall(
            L: *mut lua_State,
            nargs: std::ffi::c_int,
            nresults: std::ffi::c_int,
            errfunc: std::ffi::c_int,
        ) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/log.h:22"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gtype.h:23"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gobject.h:23"]
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
#[c2rust::header_src = "/home/daana/git/luakit/common/common.h:23"]
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
    extern "C" {
        #[c2rust::src_loc = "29:17"]
        pub static mut common: common_t;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/globalconf.h:36"]
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
    extern "C" {
        #[c2rust::src_loc = "71:21"]
        pub static mut globalconf: globalconf_t;
    }
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gtk/gtkapplication.h:30"]
pub mod gtkapplication_h {
    #[c2rust::src_loc = "39:1"]
    pub type GtkApplication = _GtkApplication;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "43:8"]
    pub struct _GtkApplication {
        pub parent: GApplication,
        pub priv_0: *mut GtkApplicationPrivate,
    }
    #[c2rust::src_loc = "41:1"]
    pub type GtkApplicationPrivate = _GtkApplicationPrivate;
    use super::giotypes_h::GApplication;
    extern "C" {
        #[c2rust::src_loc = "41:16"]
        pub type _GtkApplicationPrivate;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/gio/giotypes.h:30"]
pub mod giotypes_h {
    #[c2rust::src_loc = "59:1"]
    pub type GApplication = _GApplication;
    use super::gapplication_h::_GApplication;
}
#[c2rust::header_src = "/usr/include/glib-2.0/gio/gapplication.h:30"]
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
#[c2rust::header_src = "/usr/include/gtk-3.0/gdk/gdktypes.h:30"]
pub mod gdktypes_h {
    #[c2rust::src_loc = "241:3"]
    pub const GDK_MOD5_MASK: C2RustUnnamed = 128;
    #[c2rust::src_loc = "240:3"]
    pub const GDK_MOD4_MASK: C2RustUnnamed = 64;
    #[c2rust::src_loc = "239:3"]
    pub const GDK_MOD3_MASK: C2RustUnnamed = 32;
    #[c2rust::src_loc = "238:3"]
    pub const GDK_MOD2_MASK: C2RustUnnamed = 16;
    #[c2rust::src_loc = "237:3"]
    pub const GDK_MOD1_MASK: C2RustUnnamed = 8;
    #[c2rust::src_loc = "236:3"]
    pub const GDK_CONTROL_MASK: C2RustUnnamed = 4;
    #[c2rust::src_loc = "235:3"]
    pub const GDK_LOCK_MASK: C2RustUnnamed = 2;
    #[c2rust::src_loc = "234:3"]
    pub const GDK_SHIFT_MASK: C2RustUnnamed = 1;
    #[c2rust::src_loc = "276:3"]
    pub const GDK_MODIFIER_MASK: C2RustUnnamed = 1543512063;
    #[c2rust::src_loc = "232:9"]
    pub type C2RustUnnamed = std::ffi::c_uint;
    #[c2rust::src_loc = "272:3"]
    pub const GDK_RELEASE_MASK: C2RustUnnamed = 1073741824;
    #[c2rust::src_loc = "270:3"]
    pub const GDK_MODIFIER_RESERVED_29_MASK: C2RustUnnamed = 536870912;
    #[c2rust::src_loc = "268:3"]
    pub const GDK_META_MASK: C2RustUnnamed = 268435456;
    #[c2rust::src_loc = "267:3"]
    pub const GDK_HYPER_MASK: C2RustUnnamed = 134217728;
    #[c2rust::src_loc = "266:3"]
    pub const GDK_SUPER_MASK: C2RustUnnamed = 67108864;
    #[c2rust::src_loc = "260:3"]
    pub const GDK_MODIFIER_RESERVED_25_MASK: C2RustUnnamed = 33554432;
    #[c2rust::src_loc = "259:3"]
    pub const GDK_MODIFIER_RESERVED_24_MASK: C2RustUnnamed = 16777216;
    #[c2rust::src_loc = "258:3"]
    pub const GDK_MODIFIER_RESERVED_23_MASK: C2RustUnnamed = 8388608;
    #[c2rust::src_loc = "257:3"]
    pub const GDK_MODIFIER_RESERVED_22_MASK: C2RustUnnamed = 4194304;
    #[c2rust::src_loc = "256:3"]
    pub const GDK_MODIFIER_RESERVED_21_MASK: C2RustUnnamed = 2097152;
    #[c2rust::src_loc = "255:3"]
    pub const GDK_MODIFIER_RESERVED_20_MASK: C2RustUnnamed = 1048576;
    #[c2rust::src_loc = "254:3"]
    pub const GDK_MODIFIER_RESERVED_19_MASK: C2RustUnnamed = 524288;
    #[c2rust::src_loc = "253:3"]
    pub const GDK_MODIFIER_RESERVED_18_MASK: C2RustUnnamed = 262144;
    #[c2rust::src_loc = "252:3"]
    pub const GDK_MODIFIER_RESERVED_17_MASK: C2RustUnnamed = 131072;
    #[c2rust::src_loc = "251:3"]
    pub const GDK_MODIFIER_RESERVED_16_MASK: C2RustUnnamed = 65536;
    #[c2rust::src_loc = "250:3"]
    pub const GDK_MODIFIER_RESERVED_15_MASK: C2RustUnnamed = 32768;
    #[c2rust::src_loc = "249:3"]
    pub const GDK_MODIFIER_RESERVED_14_MASK: C2RustUnnamed = 16384;
    #[c2rust::src_loc = "248:3"]
    pub const GDK_MODIFIER_RESERVED_13_MASK: C2RustUnnamed = 8192;
    #[c2rust::src_loc = "246:3"]
    pub const GDK_BUTTON5_MASK: C2RustUnnamed = 4096;
    #[c2rust::src_loc = "245:3"]
    pub const GDK_BUTTON4_MASK: C2RustUnnamed = 2048;
    #[c2rust::src_loc = "244:3"]
    pub const GDK_BUTTON3_MASK: C2RustUnnamed = 1024;
    #[c2rust::src_loc = "243:3"]
    pub const GDK_BUTTON2_MASK: C2RustUnnamed = 512;
    #[c2rust::src_loc = "242:3"]
    pub const GDK_BUTTON1_MASK: C2RustUnnamed = 256;
}
#[c2rust::header_src = "/usr/include/string.h:22"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "43:14"]
        pub fn memcpy(
            _: *mut std::ffi::c_void,
            _: *const std::ffi::c_void,
            _: std::ffi::c_ulong,
        ) -> *mut std::ffi::c_void;
        #[c2rust::src_loc = "407:15"]
        pub fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gutils.h:22"]
pub mod gutils_h {
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "213:1"]
        pub fn g_get_system_config_dirs() -> *const *const gchar;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:22"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "105:1"]
        pub fn atoi(__nptr: *const std::ffi::c_char) -> std::ffi::c_int;
        #[c2rust::src_loc = "773:1"]
        pub fn getenv(__name: *const std::ffi::c_char) -> *mut std::ffi::c_char;
        #[c2rust::src_loc = "792:1"]
        pub fn setenv(
            __name: *const std::ffi::c_char,
            __value: *const std::ffi::c_char,
            __replace: std::ffi::c_int,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "796:1"]
        pub fn unsetenv(__name: *const std::ffi::c_char) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gfileutils.h:22"]
pub mod gfileutils_h {
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "174:1"]
        pub fn g_build_filename(first_element: *const gchar, _: ...) -> *mut gchar;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gmem.h:22"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gstrfuncs.h:22"]
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
        #[c2rust::src_loc = "285:1"]
        pub fn g_strdup_printf(format: *const gchar, _: ...) -> *mut gchar;
        #[c2rust::src_loc = "355:1"]
        pub fn g_strsplit(
            string: *const gchar,
            delimiter: *const gchar,
            max_tokens: gint,
        ) -> *mut *mut gchar;
        #[c2rust::src_loc = "363:1"]
        pub fn g_strjoinv(
            separator: *const gchar,
            str_array: *mut *mut gchar,
        ) -> *mut gchar;
        #[c2rust::src_loc = "366:1"]
        pub fn g_strfreev(str_array: *mut *mut gchar);
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:22"]
pub mod unistd_h {
    extern "C" {
        #[c2rust::src_loc = "599:1"]
        pub fn execvp(
            __file: *const std::ffi::c_char,
            __argv: *const *mut std::ffi::c_char,
        ) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/util.h:22"]
pub mod util_h {
    use super::gtypes_h::{gchar, gboolean, gint};
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "71:1"]
        pub fn file_exists(_: *const gchar) -> gboolean;
        #[c2rust::src_loc = "74:1"]
        pub fn luaH_panic(L: *mut lua_State) -> gint;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/ipc.h:22"]
pub mod ipc_h {
    extern "C" {
        #[c2rust::src_loc = "26:1"]
        pub fn ipc_remove_socket_file();
    }
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lauxlib.h:23"]
pub mod lauxlib_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "65:1"]
        pub fn luaL_loadfile(
            L: *mut lua_State,
            filename: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "70:1"]
        pub fn luaL_newstate() -> *mut lua_State;
    }
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lualib.h:23"]
pub mod lualib_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "38:1"]
        pub fn luaL_openlibs(L: *mut lua_State);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luautil.h:23"]
pub mod luautil_h {
    use super::lua_h::lua_State;
    use super::gtypes_h::{gint, gchar};
    extern "C" {
        #[c2rust::src_loc = "26:1"]
        pub fn luaH_dofunction_on_error(L: *mut lua_State) -> gint;
        #[c2rust::src_loc = "27:1"]
        pub fn luaH_add_paths(L: *mut lua_State, config_dir: *const gchar);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/lualib.h:23"]
pub mod common_lualib_h {
    #[inline]
    #[c2rust::src_loc = "101:1"]
    pub unsafe extern "C" fn luaH_dofunction(
        mut L: *mut lua_State,
        mut nargs: gint,
        mut nret: gint,
    ) -> gboolean {
        lua_insert(L, -nargs - 1 as std::ffi::c_int);
        lua_pushcclosure(
            L,
            Some(
                luaH_dofunction_on_error as unsafe extern "C" fn(*mut lua_State) -> gint,
            ),
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
    use super::lua_h::{
        lua_State, lua_insert, lua_pushcclosure, lua_gettop, lua_pcall, lua_tolstring,
        lua_settop, lua_remove,
    };
    use super::gtypes_h::{gint, gboolean};
    use super::luautil_h::luaH_dofunction_on_error;
    use super::log_h::{_log, LOG_LEVEL_error, log_level_t};
    use super::__stddef_size_t_h::size_t;
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luaobject.h:23"]
pub mod luaobject_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "39:1"]
        pub fn luaH_object_setup(L: *mut lua_State);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luah.h:23"]
pub mod luah_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "162:1"]
        pub fn luaH_fixups(L: *mut lua_State);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/log.h:24"]
pub mod luakit_log_h {
    extern "C" {
        #[c2rust::src_loc = "30:1"]
        pub fn log_dump_queued_emissions() -> *mut std::ffi::c_char;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luayield.h:27"]
pub mod luayield_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "27:1"]
        pub fn luaH_yield_setup(L: *mut lua_State);
    }
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gdk/gdkkeys.h:30"]
pub mod gdkkeys_h {
    use super::gtypes_h::{guint, gchar};
    use super::glibconfig_h::guint32;
    extern "C" {
        #[c2rust::src_loc = "137:1"]
        pub fn gdk_keyval_name(keyval: guint) -> *mut gchar;
        #[c2rust::src_loc = "155:1"]
        pub fn gdk_keyval_to_unicode(keyval: guint) -> guint32;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/clib/download.h:30"]
pub mod download_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "28:1"]
        pub fn download_class_setup(_: *mut lua_State);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/clib/luakit.h:31"]
pub mod luakit_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "38:1"]
        pub fn luakit_lib_setup(L: *mut lua_State);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/clib/request.h:32"]
pub mod request_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "30:1"]
        pub fn request_class_setup(_: *mut lua_State);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/clib/sqlite3.h:33"]
pub mod sqlite3_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "26:1"]
        pub fn sqlite3_class_setup(_: *mut lua_State);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/clib/soup.h:34"]
pub mod soup_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "26:1"]
        pub fn soup_lib_setup(L: *mut lua_State);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/clib/unique.h:35"]
pub mod unique_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "27:1"]
        pub fn unique_lib_setup(_: *mut lua_State);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/clib/widget.h:36"]
pub mod widget_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "91:1"]
        pub fn widget_class_setup(_: *mut lua_State);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/clib/xdg.h:37"]
pub mod xdg_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "26:1"]
        pub fn xdg_lib_setup(_: *mut lua_State);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/clib/stylesheet.h:38"]
pub mod stylesheet_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "34:1"]
        pub fn stylesheet_class_setup(_: *mut lua_State);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/clib/web_module.h:39"]
pub mod web_module_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "24:1"]
        pub fn web_module_lib_setup(_: *mut lua_State);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/clib/msg.h:40"]
pub mod msg_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "27:1"]
        pub fn msg_lib_setup(L: *mut lua_State);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/clib/ipc.h:41"]
pub mod clib_ipc_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "39:1"]
        pub fn ipc_channel_class_setup(_: *mut lua_State);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/clib/timer.h:42"]
pub mod timer_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "26:1"]
        pub fn timer_class_setup(_: *mut lua_State);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/clib/regex.h:43"]
pub mod regex_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "26:1"]
        pub fn regex_class_setup(_: *mut lua_State);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/clib/utf8.h:44"]
pub mod utf8_h {
    use super::lua_h::lua_State;
    extern "C" {
        #[c2rust::src_loc = "26:1"]
        pub fn utf8_lib_setup(_: *mut lua_State);
    }
}
pub use self::__stddef_size_t_h::size_t;
pub use self::glibconfig_h::{guint32, gsize};
pub use self::gtypes_h::{
    gchar, gint, gboolean, guint, gdouble, gpointer, GDestroyNotify,
};
pub use self::garray_h::{
    _GPtrArray, GPtrArray, g_ptr_array_new_with_free_func, g_ptr_array_free,
    g_ptr_array_add, g_ptr_array_insert,
};
pub use self::gdataset_h::{GData, _GData};
pub use self::gunicode_h::{gunichar, g_unichar_isgraph, g_unichar_to_utf8};
pub use self::lua_h::{
    lua_CFunction, lua_State, lua_atpanic, lua_gettop, lua_settop, lua_remove,
    lua_insert, lua_tolstring, lua_pushstring, lua_pushcclosure, lua_createtable,
    lua_setfield, lua_rawseti, lua_pcall,
};
pub use self::log_h::{
    log_level_t, LOG_LEVEL_debug, LOG_LEVEL_verbose, LOG_LEVEL_info, LOG_LEVEL_warn,
    LOG_LEVEL_error, LOG_LEVEL_fatal, _log,
};
pub use self::gtype_h::{GType, _GTypeClass, GTypeClass, _GTypeInstance, GTypeInstance};
pub use self::gobject_h::{_GObject, GObject};
pub use self::common_h::{_common_t, common_t, common};
pub use self::globalconf_h::{globalconf_t, globalconf};
pub use self::gtkapplication_h::{
    GtkApplication, _GtkApplication, GtkApplicationPrivate, _GtkApplicationPrivate,
};
pub use self::giotypes_h::GApplication;
pub use self::gapplication_h::{_GApplication, GApplicationPrivate, _GApplicationPrivate};
pub use self::gdktypes_h::{
    GDK_MOD5_MASK, GDK_MOD4_MASK, GDK_MOD3_MASK, GDK_MOD2_MASK, GDK_MOD1_MASK,
    GDK_CONTROL_MASK, GDK_LOCK_MASK, GDK_SHIFT_MASK, GDK_MODIFIER_MASK, C2RustUnnamed,
    GDK_RELEASE_MASK, GDK_MODIFIER_RESERVED_29_MASK, GDK_META_MASK, GDK_HYPER_MASK,
    GDK_SUPER_MASK, GDK_MODIFIER_RESERVED_25_MASK, GDK_MODIFIER_RESERVED_24_MASK,
    GDK_MODIFIER_RESERVED_23_MASK, GDK_MODIFIER_RESERVED_22_MASK,
    GDK_MODIFIER_RESERVED_21_MASK, GDK_MODIFIER_RESERVED_20_MASK,
    GDK_MODIFIER_RESERVED_19_MASK, GDK_MODIFIER_RESERVED_18_MASK,
    GDK_MODIFIER_RESERVED_17_MASK, GDK_MODIFIER_RESERVED_16_MASK,
    GDK_MODIFIER_RESERVED_15_MASK, GDK_MODIFIER_RESERVED_14_MASK,
    GDK_MODIFIER_RESERVED_13_MASK, GDK_BUTTON5_MASK, GDK_BUTTON4_MASK, GDK_BUTTON3_MASK,
    GDK_BUTTON2_MASK, GDK_BUTTON1_MASK,
};
use self::string_h::{memcpy, strlen};
use self::gutils_h::g_get_system_config_dirs;
use self::stdlib_h::{atoi, getenv, setenv, unsetenv};
use self::gfileutils_h::g_build_filename;
use self::gmem_h::{g_free, g_malloc};
pub use self::gstrfuncs_h::{
    g_strdup_inline, g_strdup, g_strdup_printf, g_strsplit, g_strjoinv, g_strfreev,
};
use self::unistd_h::execvp;
use self::util_h::{file_exists, luaH_panic};
use self::ipc_h::ipc_remove_socket_file;
use self::lauxlib_h::{luaL_loadfile, luaL_newstate};
use self::lualib_h::luaL_openlibs;
use self::luautil_h::{luaH_dofunction_on_error, luaH_add_paths};
pub use self::common_lualib_h::luaH_dofunction;
use self::luaobject_h::luaH_object_setup;
use self::luah_h::luaH_fixups;
use self::luakit_log_h::log_dump_queued_emissions;
use self::luayield_h::luaH_yield_setup;
use self::gdkkeys_h::{gdk_keyval_name, gdk_keyval_to_unicode};
use self::download_h::download_class_setup;
use self::luakit_h::luakit_lib_setup;
use self::request_h::request_class_setup;
use self::sqlite3_h::sqlite3_class_setup;
use self::soup_h::soup_lib_setup;
use self::unique_h::unique_lib_setup;
use self::widget_h::widget_class_setup;
use self::xdg_h::xdg_lib_setup;
use self::stylesheet_h::stylesheet_class_setup;
use self::web_module_h::web_module_lib_setup;
use self::msg_h::msg_lib_setup;
use self::clib_ipc_h::ipc_channel_class_setup;
use self::timer_h::timer_class_setup;
use self::regex_h::regex_class_setup;
use self::utf8_h::utf8_lib_setup;
#[no_mangle]
#[c2rust::src_loc = "51:1"]
pub unsafe extern "C" fn luaH_modifier_table_push(
    mut L: *mut lua_State,
    mut state: guint,
) {
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
#[no_mangle]
#[c2rust::src_loc = "77:1"]
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
#[no_mangle]
#[c2rust::src_loc = "95:1"]
pub unsafe extern "C" fn luaH_init(mut uris: *mut *mut gchar) {
    common.L = luaL_newstate();
    let mut L: *mut lua_State = common.L;
    lua_atpanic(L, Some(luaH_panic as unsafe extern "C" fn(*mut lua_State) -> gint));
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
    while !uris.is_null()
        && {
            uri = *uris.offset(i as isize);
            !uri.is_null()
        }
    {
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
#[c2rust::src_loc = "169:1"]
unsafe extern "C" fn luaH_loadrc(
    mut confpath: *const gchar,
    mut run: gboolean,
) -> gboolean {
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
#[no_mangle]
#[c2rust::src_loc = "190:1"]
pub unsafe extern "C" fn luaH_parserc(
    mut confpath: *const gchar,
    mut run: gboolean,
) -> gboolean {
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
        paths = g_ptr_array_new_with_free_func(
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        );
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
        i_str = getenv(
            b"LUAKIT_NEXT_CONFIG_INDEX\0" as *const u8 as *const std::ffi::c_char,
        );
        i = if !i_str.is_null() { atoi(i_str) } else { 0 as std::ffi::c_int };
        if !(!i_str.is_null()
            && (i <= 0 as std::ffi::c_int || i >= (*paths).len as gint))
        {
            while i < (*paths).len as gint {
                let mut path: *const gchar = *((*paths).pdata).offset(i as isize)
                    as *const gchar;
                if file_exists(path) != 0 {
                    break;
                }
                _log(
                    LOG_LEVEL_verbose,
                    b"luah.c\0" as *const u8 as *const std::ffi::c_char,
                    b"rc file '%s' does not exist\0" as *const u8
                        as *const std::ffi::c_char,
                    path,
                );
                i += 1;
                i;
            }
            if i == (*paths).len as gint {
                _log(
                    LOG_LEVEL_warn,
                    b"luah.c\0" as *const u8 as *const std::ffi::c_char,
                    b"couldn't load any rc file\0" as *const u8
                        as *const std::ffi::c_char,
                );
            } else {
                let fresh8 = i;
                i = i + 1;
                path_0 = *((*paths).pdata).offset(fresh8 as isize) as *const gchar;
                if luaH_loadrc(path_0, run) != 0 {
                    unsetenv(
                        b"LUAKIT_NEXT_CONFIG_INDEX\0" as *const u8
                            as *const std::ffi::c_char,
                    );
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
                    i_str = g_strdup_printf(
                        b"%i\0" as *const u8 as *const std::ffi::c_char,
                        i,
                    );
                    setenv(
                        b"LUAKIT_NEXT_CONFIG_INDEX\0" as *const u8
                            as *const std::ffi::c_char,
                        i_str,
                        (0 as std::ffi::c_int == 0) as std::ffi::c_int,
                    );
                    g_free(i_str as gpointer);
                    parts = g_strsplit(
                        globalconf.execpath,
                        b" \0" as *const u8 as *const std::ffi::c_char,
                        -(1 as std::ffi::c_int),
                    );
                    escaped_execpath = g_strjoinv(
                        b"\\ \0" as *const u8 as *const std::ffi::c_char,
                        parts,
                    );
                    g_strfreev(parts);
                    argv = globalconf.argv;
                    g_ptr_array_insert(
                        argv,
                        0 as std::ffi::c_int,
                        escaped_execpath as gpointer,
                    );
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
