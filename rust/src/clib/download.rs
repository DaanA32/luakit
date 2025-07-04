use ::libc;
use ::c2rust_bitfields;
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __builtin_va_list = [__va_list_tag; 1];
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "0:0"]
    pub struct __va_list_tag {
        pub gp_offset: std::ffi::c_uint,
        pub fp_offset: std::ffi::c_uint,
        pub overflow_arg_area: *mut std::ffi::c_void,
        pub reg_save_area: *mut std::ffi::c_void,
    }
}
#[c2rust::header_src = "/usr/lib/clang/20/include/__stddef_size_t.h:22"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = std::ffi::c_ulong;
}
#[c2rust::header_src = "/usr/lib/glib-2.0/include/glibconfig.h:22"]
pub mod glibconfig_h {
    #[c2rust::src_loc = "57:1"]
    pub type guint32 = std::ffi::c_uint;
    #[c2rust::src_loc = "66:1"]
    pub type gint64 = std::ffi::c_long;
    #[c2rust::src_loc = "67:1"]
    pub type guint64 = std::ffi::c_ulong;
    #[c2rust::src_loc = "83:1"]
    pub type gsize = std::ffi::c_ulong;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtypes.h:22"]
pub mod gtypes_h {
    #[c2rust::src_loc = "52:1"]
    pub type gchar = std::ffi::c_char;
    #[c2rust::src_loc = "54:1"]
    pub type glong = std::ffi::c_long;
    #[c2rust::src_loc = "55:1"]
    pub type gint = std::ffi::c_int;
    #[c2rust::src_loc = "56:1"]
    pub type gboolean = gint;
    #[c2rust::src_loc = "60:1"]
    pub type gulong = std::ffi::c_ulong;
    #[c2rust::src_loc = "61:1"]
    pub type guint = std::ffi::c_uint;
    #[c2rust::src_loc = "63:1"]
    pub type gfloat = std::ffi::c_float;
    #[c2rust::src_loc = "64:1"]
    pub type gdouble = std::ffi::c_double;
    #[c2rust::src_loc = "109:1"]
    pub type gpointer = *mut std::ffi::c_void;
    #[c2rust::src_loc = "110:1"]
    pub type gconstpointer = *const std::ffi::c_void;
    #[c2rust::src_loc = "114:1"]
    pub type GCompareDataFunc = Option::<
        unsafe extern "C" fn(gconstpointer, gconstpointer, gpointer) -> gint,
    >;
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
    use super::gtypes_h::{gpointer, guint, gboolean};
    extern "C" {
        #[c2rust::src_loc = "188:1"]
        pub fn g_ptr_array_free(
            array: *mut GPtrArray,
            free_segment: gboolean,
        ) -> *mut gpointer;
    }
}
#[c2rust::header_src = "/usr/lib/clang/20/include/__stdarg_va_list.h:22"]
pub mod __stdarg_va_list_h {
    #[c2rust::src_loc = "12:1"]
    pub type va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gquark.h:22"]
pub mod gquark_h {
    #[c2rust::src_loc = "38:1"]
    pub type GQuark = guint32;
    use super::glibconfig_h::guint32;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gerror.h:22"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gdataset.h:22"]
pub mod gdataset_h {
    #[c2rust::src_loc = "38:1"]
    pub type GData = _GData;
    extern "C" {
        #[c2rust::src_loc = "38:16"]
        pub type _GData;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/ghash.h:22"]
pub mod ghash_h {
    #[c2rust::src_loc = "40:1"]
    pub type GHashTable = _GHashTable;
    extern "C" {
        #[c2rust::src_loc = "40:16"]
        pub type _GHashTable;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtree.h:22"]
pub mod gtree_h {
    #[c2rust::src_loc = "40:1"]
    pub type GTree = _GTree;
    use super::gtypes_h::{GCompareDataFunc, gpointer, GDestroyNotify};
    extern "C" {
        #[c2rust::src_loc = "40:16"]
        pub type _GTree;
        #[c2rust::src_loc = "78:1"]
        pub fn g_tree_new_full(
            key_compare_func: GCompareDataFunc,
            key_compare_data: gpointer,
            key_destroy_func: GDestroyNotify,
            value_destroy_func: GDestroyNotify,
        ) -> *mut GTree;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gtype.h:22"]
pub mod gtype_h {
    #[c2rust::src_loc = "427:1"]
    pub type GType = gsize;
    #[c2rust::src_loc = "431:1"]
    pub type GValue = _GValue;
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
    use super::gvalue_h::_GValue;
    extern "C" {
        #[c2rust::src_loc = "2626:1"]
        pub fn g_type_check_instance_cast(
            instance: *mut GTypeInstance,
            iface_type: GType,
        ) -> *mut GTypeInstance;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gvalue.h:22"]
pub mod gvalue_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "113:8"]
    pub struct _GValue {
        pub g_type: GType,
        pub data: [C2RustUnnamed; 2],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "119:3"]
    pub union C2RustUnnamed {
        pub v_int: gint,
        pub v_uint: guint,
        pub v_long: glong,
        pub v_ulong: gulong,
        pub v_int64: gint64,
        pub v_uint64: guint64,
        pub v_float: gfloat,
        pub v_double: gdouble,
        pub v_pointer: gpointer,
    }
    use super::gtype_h::GType;
    use super::gtypes_h::{gint, guint, glong, gulong, gfloat, gdouble, gpointer};
    use super::glibconfig_h::{gint64, guint64};
}
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gparam.h:22"]
pub mod gparam_h {
    #[c2rust::src_loc = "156:9"]
    pub type GParamFlags = std::ffi::c_int;
    #[c2rust::src_loc = "171:3"]
    pub const G_PARAM_DEPRECATED: GParamFlags = -2147483648;
    #[c2rust::src_loc = "169:3"]
    pub const G_PARAM_EXPLICIT_NOTIFY: GParamFlags = 1073741824;
    #[c2rust::src_loc = "167:3"]
    pub const G_PARAM_STATIC_BLURB: GParamFlags = 128;
    #[c2rust::src_loc = "166:3"]
    pub const G_PARAM_STATIC_NICK: GParamFlags = 64;
    #[c2rust::src_loc = "165:3"]
    pub const G_PARAM_PRIVATE: GParamFlags = 32;
    #[c2rust::src_loc = "164:3"]
    pub const G_PARAM_STATIC_NAME: GParamFlags = 32;
    #[c2rust::src_loc = "163:3"]
    pub const G_PARAM_LAX_VALIDATION: GParamFlags = 16;
    #[c2rust::src_loc = "162:3"]
    pub const G_PARAM_CONSTRUCT_ONLY: GParamFlags = 8;
    #[c2rust::src_loc = "161:3"]
    pub const G_PARAM_CONSTRUCT: GParamFlags = 4;
    #[c2rust::src_loc = "160:3"]
    pub const G_PARAM_READWRITE: GParamFlags = 3;
    #[c2rust::src_loc = "159:3"]
    pub const G_PARAM_WRITABLE: GParamFlags = 2;
    #[c2rust::src_loc = "158:3"]
    pub const G_PARAM_READABLE: GParamFlags = 1;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "209:8"]
    pub struct _GParamSpec {
        pub g_type_instance: GTypeInstance,
        pub name: *const gchar,
        pub flags: GParamFlags,
        pub value_type: GType,
        pub owner_type: GType,
        pub _nick: *mut gchar,
        pub _blurb: *mut gchar,
        pub qdata: *mut GData,
        pub ref_count: guint,
        pub param_id: guint,
    }
    #[c2rust::src_loc = "204:1"]
    pub type GParamSpec = _GParamSpec;
    use super::gtype_h::{GTypeInstance, GType};
    use super::gtypes_h::{gchar, guint};
    use super::gdataset_h::GData;
}
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gclosure.h:22"]
pub mod gclosure_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "173:8"]
    pub struct _GClosure {
        #[bitfield(name = "ref_count", ty = "guint", bits = "0..=14")]
        #[bitfield(name = "meta_marshal_nouse", ty = "guint", bits = "15..=15")]
        #[bitfield(name = "n_guards", ty = "guint", bits = "16..=16")]
        #[bitfield(name = "n_fnotifiers", ty = "guint", bits = "17..=18")]
        #[bitfield(name = "n_inotifiers", ty = "guint", bits = "19..=26")]
        #[bitfield(name = "in_inotify", ty = "guint", bits = "27..=27")]
        #[bitfield(name = "floating", ty = "guint", bits = "28..=28")]
        #[bitfield(name = "derivative_flag", ty = "guint", bits = "29..=29")]
        #[bitfield(name = "in_marshal", ty = "guint", bits = "30..=30")]
        #[bitfield(name = "is_invalid", ty = "guint", bits = "31..=31")]
        pub ref_count_meta_marshal_nouse_n_guards_n_fnotifiers_n_inotifiers_in_inotify_floating_derivative_flag_in_marshal_is_invalid: [u8; 4],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 4],
        pub marshal: Option::<
            unsafe extern "C" fn(
                *mut GClosure,
                *mut GValue,
                guint,
                *const GValue,
                gpointer,
                gpointer,
            ) -> (),
        >,
        pub data: gpointer,
        pub notifiers: *mut GClosureNotifyData,
    }
    #[c2rust::src_loc = "78:1"]
    pub type GClosureNotifyData = _GClosureNotifyData;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "167:8"]
    pub struct _GClosureNotifyData {
        pub data: gpointer,
        pub notify: GClosureNotify,
    }
    #[c2rust::src_loc = "101:1"]
    pub type GClosureNotify = Option::<
        unsafe extern "C" fn(gpointer, *mut GClosure) -> (),
    >;
    #[c2rust::src_loc = "77:1"]
    pub type GClosure = _GClosure;
    #[c2rust::src_loc = "92:1"]
    pub type GCallback = Option::<unsafe extern "C" fn() -> ()>;
    use super::gtypes_h::{guint, gpointer};
    use super::gtype_h::GValue;
}
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gsignal.h:22"]
pub mod gsignal_h {
    #[c2rust::src_loc = "195:9"]
    pub type GConnectFlags = std::ffi::c_uint;
    #[c2rust::src_loc = "199:3"]
    pub const G_CONNECT_SWAPPED: GConnectFlags = 2;
    #[c2rust::src_loc = "198:3"]
    pub const G_CONNECT_AFTER: GConnectFlags = 1;
    #[c2rust::src_loc = "197:3"]
    pub const G_CONNECT_DEFAULT: GConnectFlags = 0;
    use super::gtypes_h::{gpointer, gchar, gulong};
    use super::gclosure_h::{GCallback, GClosureNotify};
    extern "C" {
        #[c2rust::src_loc = "433:1"]
        pub fn g_signal_connect_data(
            instance: gpointer,
            detailed_signal: *const gchar,
            c_handler: GCallback,
            data: gpointer,
            destroy_data: GClosureNotify,
            connect_flags: GConnectFlags,
        ) -> gulong;
    }
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
    use super::gtypes_h::{guint, gpointer};
    use super::gdataset_h::GData;
    extern "C" {
        #[c2rust::src_loc = "512:1"]
        pub fn g_object_ref(object: gpointer) -> gpointer;
        #[c2rust::src_loc = "514:1"]
        pub fn g_object_unref(object: gpointer);
    }
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lua.h:22"]
pub mod lua_h {
    #[c2rust::src_loc = "53:1"]
    pub type lua_CFunction = Option::<
        unsafe extern "C" fn(*mut lua_State) -> std::ffi::c_int,
    >;
    #[c2rust::src_loc = "100:1"]
    pub type lua_Number = std::ffi::c_double;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "360:8"]
    pub struct lua_Debug {
        pub event: std::ffi::c_int,
        pub name: *const std::ffi::c_char,
        pub namewhat: *const std::ffi::c_char,
        pub what: *const std::ffi::c_char,
        pub source: *const std::ffi::c_char,
        pub currentline: std::ffi::c_int,
        pub nups: std::ffi::c_int,
        pub linedefined: std::ffi::c_int,
        pub lastlinedefined: std::ffi::c_int,
        pub short_src: [std::ffi::c_char; 60],
        pub i_ci: std::ffi::c_int,
    }
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "51:16"]
        pub type lua_State;
        #[c2rust::src_loc = "121:1"]
        pub fn lua_gettop(L: *mut lua_State) -> std::ffi::c_int;
        #[c2rust::src_loc = "122:1"]
        pub fn lua_settop(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "123:1"]
        pub fn lua_pushvalue(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "124:1"]
        pub fn lua_remove(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "137:1"]
        pub fn lua_isstring(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;
        #[c2rust::src_loc = "140:1"]
        pub fn lua_type(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;
        #[c2rust::src_loc = "149:1"]
        pub fn lua_toboolean(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;
        #[c2rust::src_loc = "150:1"]
        pub fn lua_tolstring(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            len: *mut size_t,
        ) -> *const std::ffi::c_char;
        #[c2rust::src_loc = "162:1"]
        pub fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
        #[c2rust::src_loc = "164:1"]
        pub fn lua_pushlstring(L: *mut lua_State, s: *const std::ffi::c_char, l: size_t);
        #[c2rust::src_loc = "165:1"]
        pub fn lua_pushstring(L: *mut lua_State, s: *const std::ffi::c_char);
        #[c2rust::src_loc = "168:1"]
        pub fn lua_pushfstring(
            L: *mut lua_State,
            fmt: *const std::ffi::c_char,
            _: ...
        ) -> *const std::ffi::c_char;
        #[c2rust::src_loc = "170:1"]
        pub fn lua_pushboolean(L: *mut lua_State, b: std::ffi::c_int);
        #[c2rust::src_loc = "171:1"]
        pub fn lua_pushlightuserdata(L: *mut lua_State, p: *mut std::ffi::c_void);
        #[c2rust::src_loc = "180:1"]
        pub fn lua_rawget(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "182:1"]
        pub fn lua_createtable(
            L: *mut lua_State,
            narr: std::ffi::c_int,
            nrec: std::ffi::c_int,
        );
        #[c2rust::src_loc = "183:1"]
        pub fn lua_newuserdata(L: *mut lua_State, sz: size_t) -> *mut std::ffi::c_void;
        #[c2rust::src_loc = "195:1"]
        pub fn lua_setmetatable(
            L: *mut lua_State,
            objindex: std::ffi::c_int,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "196:1"]
        pub fn lua_setfenv(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;
        #[c2rust::src_loc = "239:1"]
        pub fn lua_error(L: *mut lua_State) -> std::ffi::c_int;
        #[c2rust::src_loc = "335:1"]
        pub fn lua_getstack(
            L: *mut lua_State,
            level: std::ffi::c_int,
            ar: *mut lua_Debug,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "336:1"]
        pub fn lua_getinfo(
            L: *mut lua_State,
            what: *const std::ffi::c_char,
            ar: *mut lua_Debug,
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
    use super::internal::__va_list_tag;
    extern "C" {
        #[c2rust::src_loc = "54:1"]
        pub fn _log(lvl: log_level_t, _: *const gchar, _: *const gchar, _: ...);
        #[c2rust::src_loc = "56:1"]
        pub fn va_log(
            lvl: log_level_t,
            _: *const gchar,
            _: *const gchar,
            _: ::core::ffi::VaList,
        );
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/signal.h:22"]
pub mod signal_h {
    #[c2rust::src_loc = "29:1"]
    pub type signal_t = GTree;
    #[inline]
    #[c2rust::src_loc = "33:1"]
    pub unsafe extern "C" fn signal_cmp(
        mut a: gconstpointer,
        mut b: gconstpointer,
        mut UNUSED_p: gpointer,
    ) -> gint {
        return g_strcmp0(a as *const std::ffi::c_char, b as *const std::ffi::c_char);
    }
    #[inline]
    #[c2rust::src_loc = "40:1"]
    pub unsafe extern "C" fn signal_array_destroy(mut sigfuncs: *mut gpointer) {
        g_ptr_array_free(
            sigfuncs as *mut GPtrArray,
            (0 as std::ffi::c_int == 0) as std::ffi::c_int,
        );
    }
    #[inline]
    #[c2rust::src_loc = "47:1"]
    pub unsafe extern "C" fn signal_new() -> *mut signal_t {
        return g_tree_new_full(
            ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(gconstpointer, gconstpointer, gpointer) -> gint,
                >,
                GCompareDataFunc,
            >(
                Some(
                    signal_cmp
                        as unsafe extern "C" fn(
                            gconstpointer,
                            gconstpointer,
                            gpointer,
                        ) -> gint,
                ),
            ),
            0 as *mut std::ffi::c_void,
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut gpointer) -> ()>,
                GDestroyNotify,
            >(Some(signal_array_destroy as unsafe extern "C" fn(*mut gpointer) -> ())),
        ) as *mut signal_t;
    }
    use super::gtree_h::{GTree, g_tree_new_full};
    use super::gtypes_h::{
        gconstpointer, gpointer, gint, GCompareDataFunc, GDestroyNotify,
    };
    use super::gtestutils_h::g_strcmp0;
    use super::garray_h::{g_ptr_array_free, GPtrArray};
    use super::gmem_h::g_free;
}
#[c2rust::header_src = "/home/daana/git/luakit/common/tokenize.h:22"]
pub mod tokenize_h {
    #[c2rust::src_loc = "7:9"]
    pub type luakit_token_t = std::ffi::c_uint;
    #[c2rust::src_loc = "282:5"]
    pub const L_TK_ZOOM_TEXT_ONLY: luakit_token_t = 274;
    #[c2rust::src_loc = "281:5"]
    pub const L_TK_ZOOM_LEVEL: luakit_token_t = 273;
    #[c2rust::src_loc = "280:5"]
    pub const L_TK_YPAGE_SIZE: luakit_token_t = 272;
    #[c2rust::src_loc = "279:5"]
    pub const L_TK_YMAX: luakit_token_t = 271;
    #[c2rust::src_loc = "278:5"]
    pub const L_TK_Y: luakit_token_t = 270;
    #[c2rust::src_loc = "277:5"]
    pub const L_TK_XPAGE_SIZE: luakit_token_t = 269;
    #[c2rust::src_loc = "276:5"]
    pub const L_TK_XMAX: luakit_token_t = 268;
    #[c2rust::src_loc = "275:5"]
    pub const L_TK_X: luakit_token_t = 267;
    #[c2rust::src_loc = "274:5"]
    pub const L_TK_WRAP_JS: luakit_token_t = 266;
    #[c2rust::src_loc = "273:5"]
    pub const L_TK_WIN_XID: luakit_token_t = 265;
    #[c2rust::src_loc = "272:5"]
    pub const L_TK_WINDOWS: luakit_token_t = 264;
    #[c2rust::src_loc = "271:5"]
    pub const L_TK_WINDOW: luakit_token_t = 263;
    #[c2rust::src_loc = "270:5"]
    pub const L_TK_WIDTH: luakit_token_t = 262;
    #[c2rust::src_loc = "269:5"]
    pub const L_TK_WEB_PROCESS_ID: luakit_token_t = 261;
    #[c2rust::src_loc = "268:5"]
    pub const L_TK_WEBVIEW: luakit_token_t = 260;
    #[c2rust::src_loc = "267:5"]
    pub const L_TK_WEBSITE_DATA: luakit_token_t = 259;
    #[c2rust::src_loc = "266:5"]
    pub const L_TK_WEBKIT_VERSION: luakit_token_t = 258;
    #[c2rust::src_loc = "265:5"]
    pub const L_TK_WEBKIT_USER_AGENT_VERSION: luakit_token_t = 257;
    #[c2rust::src_loc = "264:5"]
    pub const L_TK_WEBKIT2: luakit_token_t = 256;
    #[c2rust::src_loc = "263:5"]
    pub const L_TK_VPANED: luakit_token_t = 255;
    #[c2rust::src_loc = "262:5"]
    pub const L_TK_VISIBLE_CHILD: luakit_token_t = 254;
    #[c2rust::src_loc = "261:5"]
    pub const L_TK_VISIBLE: luakit_token_t = 253;
    #[c2rust::src_loc = "260:5"]
    pub const L_TK_VIDEOS_DIR: luakit_token_t = 252;
    #[c2rust::src_loc = "259:5"]
    pub const L_TK_VERSION: luakit_token_t = 251;
    #[c2rust::src_loc = "258:5"]
    pub const L_TK_VERBOSE: luakit_token_t = 250;
    #[c2rust::src_loc = "257:5"]
    pub const L_TK_VBOX: luakit_token_t = 249;
    #[c2rust::src_loc = "256:5"]
    pub const L_TK_VALUE: luakit_token_t = 248;
    #[c2rust::src_loc = "255:5"]
    pub const L_TK_USER_AGENT: luakit_token_t = 247;
    #[c2rust::src_loc = "254:5"]
    pub const L_TK_URI: luakit_token_t = 246;
    #[c2rust::src_loc = "253:5"]
    pub const L_TK_URGENCY_HINT: luakit_token_t = 245;
    #[c2rust::src_loc = "252:5"]
    pub const L_TK_TYPE: luakit_token_t = 244;
    #[c2rust::src_loc = "251:5"]
    pub const L_TK_TOTAL_SIZE: luakit_token_t = 243;
    #[c2rust::src_loc = "250:5"]
    pub const L_TK_TOP: luakit_token_t = 242;
    #[c2rust::src_loc = "249:5"]
    pub const L_TK_TOOLTIP: luakit_token_t = 241;
    #[c2rust::src_loc = "248:5"]
    pub const L_TK_TITLE: luakit_token_t = 240;
    #[c2rust::src_loc = "247:5"]
    pub const L_TK_TEXT_CONTENT: luakit_token_t = 239;
    #[c2rust::src_loc = "246:5"]
    pub const L_TK_TEXTWIDTH: luakit_token_t = 238;
    #[c2rust::src_loc = "245:5"]
    pub const L_TK_TEXT: luakit_token_t = 237;
    #[c2rust::src_loc = "244:5"]
    pub const L_TK_TEMPLATES_DIR: luakit_token_t = 236;
    #[c2rust::src_loc = "243:5"]
    pub const L_TK_TAG_NAME: luakit_token_t = 235;
    #[c2rust::src_loc = "242:5"]
    pub const L_TK_SYSTEM_DATA_DIRS: luakit_token_t = 234;
    #[c2rust::src_loc = "241:5"]
    pub const L_TK_SYSTEM_CONFIG_DIRS: luakit_token_t = 233;
    #[c2rust::src_loc = "240:5"]
    pub const L_TK_SWITCH: luakit_token_t = 232;
    #[c2rust::src_loc = "239:5"]
    pub const L_TK_SUGGESTED_FILENAME: luakit_token_t = 231;
    #[c2rust::src_loc = "238:5"]
    pub const L_TK_SUBMIT: luakit_token_t = 230;
    #[c2rust::src_loc = "237:5"]
    pub const L_TK_STYLESHEETS: luakit_token_t = 229;
    #[c2rust::src_loc = "236:5"]
    pub const L_TK_STYLE: luakit_token_t = 228;
    #[c2rust::src_loc = "235:5"]
    pub const L_TK_STOP: luakit_token_t = 227;
    #[c2rust::src_loc = "234:5"]
    pub const L_TK_STATUS: luakit_token_t = 226;
    #[c2rust::src_loc = "233:5"]
    pub const L_TK_STARTED: luakit_token_t = 225;
    #[c2rust::src_loc = "232:5"]
    pub const L_TK_START: luakit_token_t = 224;
    #[c2rust::src_loc = "231:5"]
    pub const L_TK_STACK: luakit_token_t = 223;
    #[c2rust::src_loc = "230:5"]
    pub const L_TK_SSL_TRUSTED: luakit_token_t = 222;
    #[c2rust::src_loc = "229:5"]
    pub const L_TK_SRC: luakit_token_t = 221;
    #[c2rust::src_loc = "228:5"]
    pub const L_TK_SPINNER: luakit_token_t = 220;
    #[c2rust::src_loc = "227:5"]
    pub const L_TK_SPELL_CHECKING_LANGUAGES: luakit_token_t = 219;
    #[c2rust::src_loc = "226:5"]
    pub const L_TK_SPACING: luakit_token_t = 218;
    #[c2rust::src_loc = "225:5"]
    pub const L_TK_SOURCE: luakit_token_t = 217;
    #[c2rust::src_loc = "224:5"]
    pub const L_TK_SOCKET: luakit_token_t = 216;
    #[c2rust::src_loc = "223:5"]
    pub const L_TK_SHOW_TABS: luakit_token_t = 215;
    #[c2rust::src_loc = "222:5"]
    pub const L_TK_SHOW_INSPECTOR: luakit_token_t = 214;
    #[c2rust::src_loc = "221:5"]
    pub const L_TK_SHOW_FRAME: luakit_token_t = 213;
    #[c2rust::src_loc = "220:5"]
    pub const L_TK_SHOW_BORDER: luakit_token_t = 212;
    #[c2rust::src_loc = "219:5"]
    pub const L_TK_SHOW: luakit_token_t = 211;
    #[c2rust::src_loc = "218:5"]
    pub const L_TK_SET_TITLE: luakit_token_t = 210;
    #[c2rust::src_loc = "217:5"]
    pub const L_TK_SET_PDFJS: luakit_token_t = 209;
    #[c2rust::src_loc = "216:5"]
    pub const L_TK_SET_FAVICON_FOR_URI: luakit_token_t = 208;
    #[c2rust::src_loc = "215:5"]
    pub const L_TK_SET_DEFAULT_SIZE: luakit_token_t = 207;
    #[c2rust::src_loc = "214:5"]
    pub const L_TK_SET_DARK_MODE: luakit_token_t = 206;
    #[c2rust::src_loc = "213:5"]
    pub const L_TK_SESSION_STATE: luakit_token_t = 205;
    #[c2rust::src_loc = "212:5"]
    pub const L_TK_SERIF_FONT_FAMILY: luakit_token_t = 204;
    #[c2rust::src_loc = "211:5"]
    pub const L_TK_SEND_KEY: luakit_token_t = 203;
    #[c2rust::src_loc = "210:5"]
    pub const L_TK_SELECT_REGION: luakit_token_t = 202;
    #[c2rust::src_loc = "209:5"]
    pub const L_TK_SELECTION: luakit_token_t = 201;
    #[c2rust::src_loc = "208:5"]
    pub const L_TK_SELECTABLE: luakit_token_t = 200;
    #[c2rust::src_loc = "207:5"]
    pub const L_TK_SECONDARY: luakit_token_t = 199;
    #[c2rust::src_loc = "206:5"]
    pub const L_TK_SEARCH_PREVIOUS: luakit_token_t = 198;
    #[c2rust::src_loc = "205:5"]
    pub const L_TK_SEARCH_NEXT: luakit_token_t = 197;
    #[c2rust::src_loc = "204:5"]
    pub const L_TK_SEARCH: luakit_token_t = 196;
    #[c2rust::src_loc = "203:5"]
    pub const L_TK_SCROLL_Y: luakit_token_t = 195;
    #[c2rust::src_loc = "202:5"]
    pub const L_TK_SCROLL_X: luakit_token_t = 194;
    #[c2rust::src_loc = "201:5"]
    pub const L_TK_SCROLLED: luakit_token_t = 193;
    #[c2rust::src_loc = "200:5"]
    pub const L_TK_SCROLLBARS: luakit_token_t = 192;
    #[c2rust::src_loc = "199:5"]
    pub const L_TK_SCROLL: luakit_token_t = 191;
    #[c2rust::src_loc = "198:5"]
    pub const L_TK_SCREEN: luakit_token_t = 190;
    #[c2rust::src_loc = "197:5"]
    pub const L_TK_SCALE: luakit_token_t = 189;
    #[c2rust::src_loc = "196:5"]
    pub const L_TK_SAVE: luakit_token_t = 188;
    #[c2rust::src_loc = "195:5"]
    pub const L_TK_SANS_SERIF_FONT_FAMILY: luakit_token_t = 187;
    #[c2rust::src_loc = "194:5"]
    pub const L_TK_ROOT_WIN_XID: luakit_token_t = 186;
    #[c2rust::src_loc = "193:5"]
    pub const L_TK_RIGHT: luakit_token_t = 185;
    #[c2rust::src_loc = "192:5"]
    pub const L_TK_RESOURCE_PATH: luakit_token_t = 184;
    #[c2rust::src_loc = "191:5"]
    pub const L_TK_REPLACE: luakit_token_t = 183;
    #[c2rust::src_loc = "190:5"]
    pub const L_TK_REORDER: luakit_token_t = 182;
    #[c2rust::src_loc = "189:5"]
    pub const L_TK_REMOVE_EVENT_LISTENER: luakit_token_t = 181;
    #[c2rust::src_loc = "188:5"]
    pub const L_TK_REMOVE: luakit_token_t = 180;
    #[c2rust::src_loc = "187:5"]
    pub const L_TK_RELOAD_BYPASS_CACHE: luakit_token_t = 179;
    #[c2rust::src_loc = "186:5"]
    pub const L_TK_RELOAD: luakit_token_t = 178;
    #[c2rust::src_loc = "185:5"]
    pub const L_TK_RECT: luakit_token_t = 177;
    #[c2rust::src_loc = "184:5"]
    pub const L_TK_QUERY: luakit_token_t = 176;
    #[c2rust::src_loc = "183:5"]
    pub const L_TK_PUBLIC_SHARE_DIR: luakit_token_t = 175;
    #[c2rust::src_loc = "182:5"]
    pub const L_TK_PROXY_URI: luakit_token_t = 174;
    #[c2rust::src_loc = "181:5"]
    pub const L_TK_PROGRESS: luakit_token_t = 173;
    #[c2rust::src_loc = "180:5"]
    pub const L_TK_PROCESS_LIMIT: luakit_token_t = 172;
    #[c2rust::src_loc = "179:5"]
    pub const L_TK_PRIVATE: luakit_token_t = 171;
    #[c2rust::src_loc = "178:5"]
    pub const L_TK_PRINT_BACKGROUNDS: luakit_token_t = 170;
    #[c2rust::src_loc = "177:5"]
    pub const L_TK_PRIMARY: luakit_token_t = 169;
    #[c2rust::src_loc = "176:5"]
    pub const L_TK_PREV_SIBLING: luakit_token_t = 168;
    #[c2rust::src_loc = "175:5"]
    pub const L_TK_POSITION: luakit_token_t = 167;
    #[c2rust::src_loc = "174:5"]
    pub const L_TK_PLUGGED: luakit_token_t = 166;
    #[c2rust::src_loc = "173:5"]
    pub const L_TK_PICTURES_DIR: luakit_token_t = 165;
    #[c2rust::src_loc = "172:5"]
    pub const L_TK_PICTOGRAPH_FONT_FAMILY: luakit_token_t = 164;
    #[c2rust::src_loc = "171:5"]
    pub const L_TK_PATTERN: luakit_token_t = 163;
    #[c2rust::src_loc = "170:5"]
    pub const L_TK_PARENT: luakit_token_t = 162;
    #[c2rust::src_loc = "169:5"]
    pub const L_TK_PACK2: luakit_token_t = 161;
    #[c2rust::src_loc = "168:5"]
    pub const L_TK_PACK1: luakit_token_t = 160;
    #[c2rust::src_loc = "167:5"]
    pub const L_TK_PACK: luakit_token_t = 159;
    #[c2rust::src_loc = "166:5"]
    pub const L_TK_OWNER_DOCUMENT: luakit_token_t = 158;
    #[c2rust::src_loc = "165:5"]
    pub const L_TK_OVERLAY: luakit_token_t = 157;
    #[c2rust::src_loc = "164:5"]
    pub const L_TK_OPTIONS: luakit_token_t = 156;
    #[c2rust::src_loc = "163:5"]
    pub const L_TK_NOUNIQUE: luakit_token_t = 155;
    #[c2rust::src_loc = "162:5"]
    pub const L_TK_NOTEBOOK: luakit_token_t = 154;
    #[c2rust::src_loc = "161:5"]
    pub const L_TK_NEXT_SIBLING: luakit_token_t = 153;
    #[c2rust::src_loc = "160:5"]
    pub const L_TK_NAME: luakit_token_t = 152;
    #[c2rust::src_loc = "159:5"]
    pub const L_TK_MUSIC_DIR: luakit_token_t = 151;
    #[c2rust::src_loc = "158:5"]
    pub const L_TK_MONOSPACE_FONT_FAMILY: luakit_token_t = 150;
    #[c2rust::src_loc = "157:5"]
    pub const L_TK_MIN_SIZE: luakit_token_t = 149;
    #[c2rust::src_loc = "156:5"]
    pub const L_TK_MINIMUM_FONT_SIZE: luakit_token_t = 148;
    #[c2rust::src_loc = "155:5"]
    pub const L_TK_MIME_TYPE: luakit_token_t = 147;
    #[c2rust::src_loc = "154:5"]
    pub const L_TK_MEDIA_PLAYBACK_REQUIRES_GESTURE: luakit_token_t = 146;
    #[c2rust::src_loc = "153:5"]
    pub const L_TK_MEDIA_PLAYBACK_ALLOWS_INLINE: luakit_token_t = 145;
    #[c2rust::src_loc = "152:5"]
    pub const L_TK_MAXIMIZED: luakit_token_t = 144;
    #[c2rust::src_loc = "151:5"]
    pub const L_TK_MARGIN_TOP: luakit_token_t = 143;
    #[c2rust::src_loc = "150:5"]
    pub const L_TK_MARGIN_RIGHT: luakit_token_t = 142;
    #[c2rust::src_loc = "149:5"]
    pub const L_TK_MARGIN_LEFT: luakit_token_t = 141;
    #[c2rust::src_loc = "148:5"]
    pub const L_TK_MARGIN_BOTTOM: luakit_token_t = 140;
    #[c2rust::src_loc = "147:5"]
    pub const L_TK_MARGIN: luakit_token_t = 139;
    #[c2rust::src_loc = "146:5"]
    pub const L_TK_LOAD_STRING: luakit_token_t = 138;
    #[c2rust::src_loc = "145:5"]
    pub const L_TK_LOADING: luakit_token_t = 137;
    #[c2rust::src_loc = "144:5"]
    pub const L_TK_LEFT: luakit_token_t = 136;
    #[c2rust::src_loc = "143:5"]
    pub const L_TK_LAST_CHILD: luakit_token_t = 135;
    #[c2rust::src_loc = "142:5"]
    pub const L_TK_LABEL: luakit_token_t = 134;
    #[c2rust::src_loc = "141:5"]
    pub const L_TK_JAVASCRIPT_CAN_OPEN_WINDOWS_AUTOMATICALLY: luakit_token_t = 133;
    #[c2rust::src_loc = "140:5"]
    pub const L_TK_JAVASCRIPT_CAN_ACCESS_CLIPBOARD: luakit_token_t = 132;
    #[c2rust::src_loc = "139:5"]
    pub const L_TK_IS_PLAYING_AUDIO: luakit_token_t = 131;
    #[c2rust::src_loc = "138:5"]
    pub const L_TK_IS_LOADING: luakit_token_t = 130;
    #[c2rust::src_loc = "137:5"]
    pub const L_TK_IS_ALIVE: luakit_token_t = 129;
    #[c2rust::src_loc = "136:5"]
    pub const L_TK_INVALIDATE: luakit_token_t = 128;
    #[c2rust::src_loc = "135:5"]
    pub const L_TK_INTERVAL: luakit_token_t = 127;
    #[c2rust::src_loc = "134:5"]
    pub const L_TK_INSTALL_PATHS: luakit_token_t = 126;
    #[c2rust::src_loc = "133:5"]
    pub const L_TK_INSTALL_PATH: luakit_token_t = 125;
    #[c2rust::src_loc = "132:5"]
    pub const L_TK_INSPECTOR: luakit_token_t = 124;
    #[c2rust::src_loc = "131:5"]
    pub const L_TK_INSERT: luakit_token_t = 123;
    #[c2rust::src_loc = "130:5"]
    pub const L_TK_INNER_WIDTH: luakit_token_t = 122;
    #[c2rust::src_loc = "129:5"]
    pub const L_TK_INNER_HTML: luakit_token_t = 121;
    #[c2rust::src_loc = "128:5"]
    pub const L_TK_INNER_HEIGHT: luakit_token_t = 120;
    #[c2rust::src_loc = "127:5"]
    pub const L_TK_INDEXOF: luakit_token_t = 119;
    #[c2rust::src_loc = "126:5"]
    pub const L_TK_IMAGE: luakit_token_t = 118;
    #[c2rust::src_loc = "125:5"]
    pub const L_TK_ID: luakit_token_t = 117;
    #[c2rust::src_loc = "124:5"]
    pub const L_TK_ICON: luakit_token_t = 116;
    #[c2rust::src_loc = "123:5"]
    pub const L_TK_HREF: luakit_token_t = 115;
    #[c2rust::src_loc = "122:5"]
    pub const L_TK_HPANED: luakit_token_t = 114;
    #[c2rust::src_loc = "121:5"]
    pub const L_TK_HOVERED_URI: luakit_token_t = 113;
    #[c2rust::src_loc = "120:5"]
    pub const L_TK_HOMOGENEOUS: luakit_token_t = 112;
    #[c2rust::src_loc = "119:5"]
    pub const L_TK_HISTORY: luakit_token_t = 111;
    #[c2rust::src_loc = "118:5"]
    pub const L_TK_HIDE: luakit_token_t = 110;
    #[c2rust::src_loc = "117:5"]
    pub const L_TK_HEIGHT: luakit_token_t = 109;
    #[c2rust::src_loc = "116:5"]
    pub const L_TK_HBOX: luakit_token_t = 108;
    #[c2rust::src_loc = "115:5"]
    pub const L_TK_HARDWARE_ACCELERATION_POLICY: luakit_token_t = 107;
    #[c2rust::src_loc = "114:5"]
    pub const L_TK_GO_FORWARD: luakit_token_t = 106;
    #[c2rust::src_loc = "113:5"]
    pub const L_TK_GO_BACK: luakit_token_t = 105;
    #[c2rust::src_loc = "112:5"]
    pub const L_TK_GET_TITLE: luakit_token_t = 104;
    #[c2rust::src_loc = "111:5"]
    pub const L_TK_GET_SOURCE: luakit_token_t = 103;
    #[c2rust::src_loc = "110:5"]
    pub const L_TK_FULLSCREEN: luakit_token_t = 102;
    #[c2rust::src_loc = "109:5"]
    pub const L_TK_FONT: luakit_token_t = 101;
    #[c2rust::src_loc = "108:5"]
    pub const L_TK_FOCUSED: luakit_token_t = 100;
    #[c2rust::src_loc = "107:5"]
    pub const L_TK_FOCUS: luakit_token_t = 99;
    #[c2rust::src_loc = "106:5"]
    pub const L_TK_FIRST_CHILD: luakit_token_t = 98;
    #[c2rust::src_loc = "105:5"]
    pub const L_TK_FINISHED: luakit_token_t = 97;
    #[c2rust::src_loc = "104:5"]
    pub const L_TK_FILL: luakit_token_t = 96;
    #[c2rust::src_loc = "103:5"]
    pub const L_TK_FILENAME: luakit_token_t = 95;
    #[c2rust::src_loc = "102:5"]
    pub const L_TK_FG: luakit_token_t = 94;
    #[c2rust::src_loc = "101:5"]
    pub const L_TK_FETCH: luakit_token_t = 93;
    #[c2rust::src_loc = "100:5"]
    pub const L_TK_FANTASY_FONT_FAMILY: luakit_token_t = 92;
    #[c2rust::src_loc = "99:5"]
    pub const L_TK_EXECPATH: luakit_token_t = 91;
    #[c2rust::src_loc = "98:5"]
    pub const L_TK_EVENTBOX: luakit_token_t = 90;
    #[c2rust::src_loc = "97:5"]
    pub const L_TK_EVAL_JS: luakit_token_t = 89;
    #[c2rust::src_loc = "96:5"]
    pub const L_TK_ERROR: luakit_token_t = 88;
    #[c2rust::src_loc = "95:5"]
    pub const L_TK_ENTRY: luakit_token_t = 87;
    #[c2rust::src_loc = "94:5"]
    pub const L_TK_END: luakit_token_t = 86;
    #[c2rust::src_loc = "93:5"]
    pub const L_TK_ENABLE_XSS_AUDITOR: luakit_token_t = 85;
    #[c2rust::src_loc = "92:5"]
    pub const L_TK_ENABLE_WRITE_CONSOLE_MESSAGES_TO_STDOUT: luakit_token_t = 84;
    #[c2rust::src_loc = "91:5"]
    pub const L_TK_ENABLE_WEBGL: luakit_token_t = 83;
    #[c2rust::src_loc = "90:5"]
    pub const L_TK_ENABLE_WEBAUDIO: luakit_token_t = 82;
    #[c2rust::src_loc = "89:5"]
    pub const L_TK_ENABLE_TABS_TO_LINKS: luakit_token_t = 81;
    #[c2rust::src_loc = "88:5"]
    pub const L_TK_ENABLE_SPELL_CHECKING: luakit_token_t = 80;
    #[c2rust::src_loc = "87:5"]
    pub const L_TK_ENABLE_SPATIAL_NAVIGATION: luakit_token_t = 79;
    #[c2rust::src_loc = "86:5"]
    pub const L_TK_ENABLE_SMOOTH_SCROLLING: luakit_token_t = 78;
    #[c2rust::src_loc = "85:5"]
    pub const L_TK_ENABLE_SITE_SPECIFIC_QUIRKS: luakit_token_t = 77;
    #[c2rust::src_loc = "84:5"]
    pub const L_TK_ENABLE_SCRIPTS: luakit_token_t = 76;
    #[c2rust::src_loc = "83:5"]
    pub const L_TK_ENABLE_RESIZABLE_TEXT_AREAS: luakit_token_t = 75;
    #[c2rust::src_loc = "82:5"]
    pub const L_TK_ENABLE_PLUGINS: luakit_token_t = 74;
    #[c2rust::src_loc = "81:5"]
    pub const L_TK_ENABLE_PAGE_CACHE: luakit_token_t = 73;
    #[c2rust::src_loc = "80:5"]
    pub const L_TK_ENABLE_MEDIA_STREAM: luakit_token_t = 72;
    #[c2rust::src_loc = "79:5"]
    pub const L_TK_ENABLE_MEDIASOURCE: luakit_token_t = 71;
    #[c2rust::src_loc = "78:5"]
    pub const L_TK_ENABLE_JAVASCRIPT: luakit_token_t = 70;
    #[c2rust::src_loc = "77:5"]
    pub const L_TK_ENABLE_JAVA: luakit_token_t = 69;
    #[c2rust::src_loc = "76:5"]
    pub const L_TK_ENABLE_HYPERLINK_AUDITING: luakit_token_t = 68;
    #[c2rust::src_loc = "75:5"]
    pub const L_TK_ENABLE_HTML5_LOCAL_STORAGE: luakit_token_t = 67;
    #[c2rust::src_loc = "74:5"]
    pub const L_TK_ENABLE_HTML5_DATABASE: luakit_token_t = 66;
    #[c2rust::src_loc = "73:5"]
    pub const L_TK_ENABLE_FULLSCREEN: luakit_token_t = 65;
    #[c2rust::src_loc = "72:5"]
    pub const L_TK_ENABLE_FRAME_FLATTENING: luakit_token_t = 64;
    #[c2rust::src_loc = "71:5"]
    pub const L_TK_ENABLE_DNS_PREFETCHING: luakit_token_t = 63;
    #[c2rust::src_loc = "70:5"]
    pub const L_TK_ENABLE_DEVELOPER_EXTRAS: luakit_token_t = 62;
    #[c2rust::src_loc = "69:5"]
    pub const L_TK_ENABLE_CARET_BROWSING: luakit_token_t = 61;
    #[c2rust::src_loc = "68:5"]
    pub const L_TK_ENABLE_ACCELERATED_2D_CANVAS: luakit_token_t = 60;
    #[c2rust::src_loc = "67:5"]
    pub const L_TK_ELEMENT_FROM_POINT: luakit_token_t = 59;
    #[c2rust::src_loc = "66:5"]
    pub const L_TK_ELAPSED_TIME: luakit_token_t = 58;
    #[c2rust::src_loc = "65:5"]
    pub const L_TK_EDITABLE: luakit_token_t = 57;
    #[c2rust::src_loc = "64:5"]
    pub const L_TK_DRAW_COMPOSITING_INDICATORS: luakit_token_t = 56;
    #[c2rust::src_loc = "63:5"]
    pub const L_TK_DRAWING_AREA: luakit_token_t = 55;
    #[c2rust::src_loc = "62:5"]
    pub const L_TK_DOWNLOAD_DIR: luakit_token_t = 54;
    #[c2rust::src_loc = "61:5"]
    pub const L_TK_DOCUMENTS_DIR: luakit_token_t = 53;
    #[c2rust::src_loc = "60:5"]
    pub const L_TK_DOCUMENT: luakit_token_t = 52;
    #[c2rust::src_loc = "59:5"]
    pub const L_TK_DEV_PATHS: luakit_token_t = 51;
    #[c2rust::src_loc = "58:5"]
    pub const L_TK_DESTROY: luakit_token_t = 50;
    #[c2rust::src_loc = "57:5"]
    pub const L_TK_DESTINATION: luakit_token_t = 49;
    #[c2rust::src_loc = "56:5"]
    pub const L_TK_DESKTOP_DIR: luakit_token_t = 48;
    #[c2rust::src_loc = "55:5"]
    pub const L_TK_DEFAULT_MONOSPACE_FONT_SIZE: luakit_token_t = 47;
    #[c2rust::src_loc = "54:5"]
    pub const L_TK_DEFAULT_FONT_SIZE: luakit_token_t = 46;
    #[c2rust::src_loc = "53:5"]
    pub const L_TK_DEFAULT_FONT_FAMILY: luakit_token_t = 45;
    #[c2rust::src_loc = "52:5"]
    pub const L_TK_DEFAULT_CHARSET: luakit_token_t = 44;
    #[c2rust::src_loc = "51:5"]
    pub const L_TK_DECORATED: luakit_token_t = 43;
    #[c2rust::src_loc = "50:5"]
    pub const L_TK_DATA_DIR: luakit_token_t = 42;
    #[c2rust::src_loc = "49:5"]
    pub const L_TK_CURSIVE_FONT_FAMILY: luakit_token_t = 41;
    #[c2rust::src_loc = "48:5"]
    pub const L_TK_CURRENT_SIZE: luakit_token_t = 40;
    #[c2rust::src_loc = "47:5"]
    pub const L_TK_CURRENT: luakit_token_t = 39;
    #[c2rust::src_loc = "46:5"]
    pub const L_TK_CSS: luakit_token_t = 38;
    #[c2rust::src_loc = "45:5"]
    pub const L_TK_CREATE_ELEMENT: luakit_token_t = 37;
    #[c2rust::src_loc = "44:5"]
    pub const L_TK_CRASH: luakit_token_t = 36;
    #[c2rust::src_loc = "43:5"]
    pub const L_TK_COUNT: luakit_token_t = 35;
    #[c2rust::src_loc = "42:5"]
    pub const L_TK_COOKIES_STORAGE: luakit_token_t = 34;
    #[c2rust::src_loc = "41:5"]
    pub const L_TK_CONFPATH: luakit_token_t = 33;
    #[c2rust::src_loc = "40:5"]
    pub const L_TK_CONFIG_DIR: luakit_token_t = 32;
    #[c2rust::src_loc = "39:5"]
    pub const L_TK_CLOSE_INSPECTOR: luakit_token_t = 31;
    #[c2rust::src_loc = "38:5"]
    pub const L_TK_CLIPBOARD: luakit_token_t = 30;
    #[c2rust::src_loc = "37:5"]
    pub const L_TK_CLIENT_RECTS: luakit_token_t = 29;
    #[c2rust::src_loc = "36:5"]
    pub const L_TK_CLICK: luakit_token_t = 28;
    #[c2rust::src_loc = "35:5"]
    pub const L_TK_CLEAR_SEARCH: luakit_token_t = 27;
    #[c2rust::src_loc = "34:5"]
    pub const L_TK_CLEAR: luakit_token_t = 26;
    #[c2rust::src_loc = "33:5"]
    pub const L_TK_CHILD_COUNT: luakit_token_t = 25;
    #[c2rust::src_loc = "32:5"]
    pub const L_TK_CHILDREN: luakit_token_t = 24;
    #[c2rust::src_loc = "31:5"]
    pub const L_TK_CHILD: luakit_token_t = 23;
    #[c2rust::src_loc = "30:5"]
    pub const L_TK_CHECKED: luakit_token_t = 22;
    #[c2rust::src_loc = "29:5"]
    pub const L_TK_CERTIFICATE: luakit_token_t = 21;
    #[c2rust::src_loc = "28:5"]
    pub const L_TK_CENTER: luakit_token_t = 20;
    #[c2rust::src_loc = "27:5"]
    pub const L_TK_CAN_GO_FORWARD: luakit_token_t = 19;
    #[c2rust::src_loc = "26:5"]
    pub const L_TK_CAN_GO_BACK: luakit_token_t = 18;
    #[c2rust::src_loc = "25:5"]
    pub const L_TK_CAN_FOCUS: luakit_token_t = 17;
    #[c2rust::src_loc = "24:5"]
    pub const L_TK_CACHE_DIR: luakit_token_t = 16;
    #[c2rust::src_loc = "23:5"]
    pub const L_TK_BOTTOM: luakit_token_t = 15;
    #[c2rust::src_loc = "22:5"]
    pub const L_TK_BODY: luakit_token_t = 14;
    #[c2rust::src_loc = "21:5"]
    pub const L_TK_BG: luakit_token_t = 13;
    #[c2rust::src_loc = "20:5"]
    pub const L_TK_BASELINE: luakit_token_t = 12;
    #[c2rust::src_loc = "19:5"]
    pub const L_TK_AUTO_LOAD_IMAGES: luakit_token_t = 11;
    #[c2rust::src_loc = "18:5"]
    pub const L_TK_ATTR: luakit_token_t = 10;
    #[c2rust::src_loc = "17:5"]
    pub const L_TK_APPEND: luakit_token_t = 9;
    #[c2rust::src_loc = "16:5"]
    pub const L_TK_ALLOW_UNIVERSAL_ACCESS_FROM_FILE_URLS: luakit_token_t = 8;
    #[c2rust::src_loc = "15:5"]
    pub const L_TK_ALLOW_OVERWRITE: luakit_token_t = 7;
    #[c2rust::src_loc = "14:5"]
    pub const L_TK_ALLOW_MODAL_DIALOGS: luakit_token_t = 6;
    #[c2rust::src_loc = "13:5"]
    pub const L_TK_ALLOW_FILE_ACCESS_FROM_FILE_URLS: luakit_token_t = 5;
    #[c2rust::src_loc = "12:5"]
    pub const L_TK_ALLOW_CERTIFICATE: luakit_token_t = 4;
    #[c2rust::src_loc = "11:5"]
    pub const L_TK_ALIGN: luakit_token_t = 3;
    #[c2rust::src_loc = "10:5"]
    pub const L_TK_ADD_EVENT_LISTENER: luakit_token_t = 2;
    #[c2rust::src_loc = "9:5"]
    pub const L_TK_ACCEPT_POLICY: luakit_token_t = 1;
    #[c2rust::src_loc = "8:5"]
    pub const L_TK_UNKNOWN: luakit_token_t = 0;
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lauxlib.h:22"]
pub mod lauxlib_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "21:16"]
    pub struct luaL_Reg {
        pub name: *const std::ffi::c_char,
        pub func: lua_CFunction,
    }
    use super::lua_h::{lua_CFunction, lua_State};
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "32:1"]
        pub fn luaL_typerror(
            L: *mut lua_State,
            narg: std::ffi::c_int,
            tname: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "34:1"]
        pub fn luaL_checklstring(
            L: *mut lua_State,
            numArg: std::ffi::c_int,
            l: *mut size_t,
        ) -> *const std::ffi::c_char;
        #[c2rust::src_loc = "53:1"]
        pub fn luaL_error(
            L: *mut lua_State,
            fmt: *const std::ffi::c_char,
            _: ...
        ) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luaclass.h:22"]
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
    use super::gtypes_h::{gint, gchar, gpointer};
    use super::lauxlib_h::luaL_Reg;
    use super::tokenize_h::{luakit_token_t, L_TK_UNKNOWN};
    extern "C" {
        #[c2rust::src_loc = "65:1"]
        pub fn luaH_class_add_signal(
            _: *mut lua_State,
            _: *mut lua_class_t,
            name: *const gchar,
            ud: gint,
        );
        #[c2rust::src_loc = "67:1"]
        pub fn luaH_class_remove_signal(
            _: *mut lua_State,
            _: *mut lua_class_t,
            name: *const gchar,
            ud: gint,
        );
        #[c2rust::src_loc = "69:1"]
        pub fn luaH_class_emit_signal(
            _: *mut lua_State,
            _: *mut lua_class_t,
            name: *const gchar,
            nargs: gint,
            nret: gint,
        ) -> gint;
        #[c2rust::src_loc = "76:1"]
        pub fn luaH_class_setup(
            _: *mut lua_State,
            _: *mut lua_class_t,
            _: *const gchar,
            _: lua_class_allocator_t,
            _: lua_class_propfunc_t,
            _: lua_class_propfunc_t,
            _: *const luaL_Reg,
            _: *const luaL_Reg,
        );
        #[c2rust::src_loc = "80:1"]
        pub fn luaH_class_add_property(
            _: *mut lua_class_t,
            token: luakit_token_t,
            _: lua_class_propfunc_t,
            _: lua_class_propfunc_t,
            _: lua_class_propfunc_t,
        );
        #[c2rust::src_loc = "84:1"]
        pub fn luaH_class_index(_: *mut lua_State) -> gint;
        #[c2rust::src_loc = "85:1"]
        pub fn luaH_class_newindex(_: *mut lua_State) -> gint;
        #[c2rust::src_loc = "88:1"]
        pub fn luaH_checkudata(
            _: *mut lua_State,
            _: gint,
            _: *mut lua_class_t,
        ) -> gpointer;
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
    extern "C" {
        #[c2rust::src_loc = "29:17"]
        pub static mut common: common_t;
    }
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkit/WebKitURIRequest.h:24"]
pub mod WebKitURIRequest_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "50:1"]
    pub struct _WebKitURIRequest {
        pub parent: GObject,
        pub priv_0: *mut WebKitURIRequestPrivate,
    }
    #[c2rust::src_loc = "50:1"]
    pub type WebKitURIRequestPrivate = _WebKitURIRequestPrivate;
    #[c2rust::src_loc = "50:1"]
    pub type WebKitURIRequest = _WebKitURIRequest;
    use super::gobject_h::GObject;
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "50:1"]
        pub type _WebKitURIRequestPrivate;
        #[c2rust::src_loc = "55:1"]
        pub fn webkit_uri_request_get_uri(
            request: *mut WebKitURIRequest,
        ) -> *const gchar;
    }
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkit/WebKitURIResponse.h:24"]
pub mod WebKitURIResponse_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "50:1"]
    pub struct _WebKitURIResponse {
        pub parent: GObject,
        pub priv_0: *mut WebKitURIResponsePrivate,
    }
    #[c2rust::src_loc = "50:1"]
    pub type WebKitURIResponsePrivate = _WebKitURIResponsePrivate;
    #[c2rust::src_loc = "50:1"]
    pub type WebKitURIResponse = _WebKitURIResponse;
    use super::gobject_h::GObject;
    use super::glibconfig_h::guint64;
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "50:1"]
        pub type _WebKitURIResponsePrivate;
        #[c2rust::src_loc = "58:1"]
        pub fn webkit_uri_response_get_content_length(
            response: *mut WebKitURIResponse,
        ) -> guint64;
        #[c2rust::src_loc = "61:1"]
        pub fn webkit_uri_response_get_mime_type(
            response: *mut WebKitURIResponse,
        ) -> *const gchar;
        #[c2rust::src_loc = "64:1"]
        pub fn webkit_uri_response_get_suggested_filename(
            response: *mut WebKitURIResponse,
        ) -> *const gchar;
    }
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkit/WebKitDownload.h:24"]
pub mod WebKitDownload_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "41:1"]
    pub struct _WebKitDownload {
        pub parent: GObject,
        pub priv_0: *mut WebKitDownloadPrivate,
    }
    #[c2rust::src_loc = "41:1"]
    pub type WebKitDownloadPrivate = _WebKitDownloadPrivate;
    #[c2rust::src_loc = "41:1"]
    pub type WebKitDownload = _WebKitDownload;
    use super::gobject_h::GObject;
    use super::WebKitURIRequest_h::WebKitURIRequest;
    use super::gtypes_h::{gchar, gdouble, gboolean};
    use super::WebKitURIResponse_h::WebKitURIResponse;
    use super::glibconfig_h::guint64;
    extern "C" {
        #[c2rust::src_loc = "41:1"]
        pub type _WebKitDownloadPrivate;
        #[c2rust::src_loc = "59:1"]
        pub fn webkit_download_get_request(
            download: *mut WebKitDownload,
        ) -> *mut WebKitURIRequest;
        #[c2rust::src_loc = "65:1"]
        pub fn webkit_download_set_destination(
            download: *mut WebKitDownload,
            destination: *const gchar,
        );
        #[c2rust::src_loc = "69:1"]
        pub fn webkit_download_get_response(
            download: *mut WebKitDownload,
        ) -> *mut WebKitURIResponse;
        #[c2rust::src_loc = "72:1"]
        pub fn webkit_download_cancel(download: *mut WebKitDownload);
        #[c2rust::src_loc = "75:1"]
        pub fn webkit_download_get_estimated_progress(
            download: *mut WebKitDownload,
        ) -> gdouble;
        #[c2rust::src_loc = "78:1"]
        pub fn webkit_download_get_elapsed_time(
            download: *mut WebKitDownload,
        ) -> gdouble;
        #[c2rust::src_loc = "81:1"]
        pub fn webkit_download_get_received_data_length(
            download: *mut WebKitDownload,
        ) -> guint64;
        #[c2rust::src_loc = "87:1"]
        pub fn webkit_download_get_allow_overwrite(
            download: *mut WebKitDownload,
        ) -> gboolean;
        #[c2rust::src_loc = "90:1"]
        pub fn webkit_download_set_allow_overwrite(
            download: *mut WebKitDownload,
            allowed: gboolean,
        );
    }
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkit/WebKitError.h:24"]
pub mod WebKitError_h {
    #[c2rust::src_loc = "108:9"]
    pub type C2RustUnnamed_0 = std::ffi::c_uint;
    #[c2rust::src_loc = "111:5"]
    pub const WEBKIT_DOWNLOAD_ERROR_DESTINATION: C2RustUnnamed_0 = 401;
    #[c2rust::src_loc = "110:5"]
    pub const WEBKIT_DOWNLOAD_ERROR_CANCELLED_BY_USER: C2RustUnnamed_0 = 400;
    #[c2rust::src_loc = "109:5"]
    pub const WEBKIT_DOWNLOAD_ERROR_NETWORK: C2RustUnnamed_0 = 499;
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkit/WebKitWebContext.h:24"]
pub mod WebKitWebContext_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:1"]
    pub struct _WebKitWebContext {
        pub parent: GObject,
        pub priv_0: *mut WebKitWebContextPrivate,
    }
    #[c2rust::src_loc = "49:1"]
    pub type WebKitWebContextPrivate = _WebKitWebContextPrivate;
    #[c2rust::src_loc = "49:1"]
    pub type WebKitWebContext = _WebKitWebContext;
    use super::gobject_h::GObject;
    use super::gtypes_h::gchar;
    use super::WebKitDownload_h::WebKitDownload;
    extern "C" {
        #[c2rust::src_loc = "49:1"]
        pub type _WebKitWebContextPrivate;
        #[c2rust::src_loc = "172:1"]
        pub fn webkit_web_context_download_uri(
            context: *mut WebKitWebContext,
            uri: *const gchar,
        ) -> *mut WebKitDownload;
    }
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
        #[c2rust::src_loc = "61:14"]
        pub fn memset(
            _: *mut std::ffi::c_void,
            _: std::ffi::c_int,
            _: std::ffi::c_ulong,
        ) -> *mut std::ffi::c_void;
        #[c2rust::src_loc = "407:15"]
        pub fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gconvert.h:22"]
pub mod gconvert_h {
    use super::gtypes_h::gchar;
    use super::gerror_h::GError;
    extern "C" {
        #[c2rust::src_loc = "162:1"]
        pub fn g_filename_to_uri(
            filename: *const gchar,
            hostname: *const gchar,
            error: *mut *mut GError,
        ) -> *mut gchar;
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
    use super::gtypes_h::gchar;
    use super::string_h::{strlen, memcpy};
    use super::__stddef_size_t_h::size_t;
    use super::gmem_h::g_malloc;
    extern "C" {
        #[c2rust::src_loc = "134:1"]
        pub fn g_strrstr(haystack: *const gchar, needle: *const gchar) -> *mut gchar;
        #[c2rust::src_loc = "283:1"]
        pub fn g_strdup(str: *const gchar) -> *mut gchar;
        #[c2rust::src_loc = "285:1"]
        pub fn g_strdup_printf(format: *const gchar, _: ...) -> *mut gchar;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtestutils.h:22"]
pub mod gtestutils_h {
    use super::glibconfig_h::guint64;
    extern "C" {
        #[c2rust::src_loc = "282:1"]
        pub fn g_strcmp0(
            str1: *const std::ffi::c_char,
            str2: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "650:1"]
        pub fn g_assertion_message_cmpint(
            domain: *const std::ffi::c_char,
            file: *const std::ffi::c_char,
            line: std::ffi::c_int,
            func: *const std::ffi::c_char,
            expr: *const std::ffi::c_char,
            arg1: guint64,
            cmp: *const std::ffi::c_char,
            arg2: guint64,
            numtype: std::ffi::c_char,
        );
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luaobject.h:22"]
pub mod luaobject_h {
    #[inline]
    #[c2rust::src_loc = "88:1"]
    pub unsafe extern "C" fn luaH_object_registry_push(mut L: *mut lua_State) {
        lua_pushlstring(
            L,
            b"luakit.object.registry\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 23]>() as std::ffi::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        );
        lua_rawget(L, -(10000 as std::ffi::c_int));
    }
    #[inline]
    #[c2rust::src_loc = "99:1"]
    pub unsafe extern "C" fn luaH_object_ref(
        mut L: *mut lua_State,
        mut oud: gint,
    ) -> gpointer {
        luaH_object_registry_push(L);
        let mut p: gpointer = luaH_object_incref(
            L,
            -(1 as std::ffi::c_int),
            if oud < 0 as std::ffi::c_int { oud - 1 as std::ffi::c_int } else { oud },
        );
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        return p;
    }
    #[inline]
    #[c2rust::src_loc = "121:1"]
    pub unsafe extern "C" fn luaH_object_unref(mut L: *mut lua_State, mut p: gpointer) {
        luaH_object_registry_push(L);
        luaH_object_decref(L, -(1 as std::ffi::c_int), p);
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    }
    #[inline]
    #[c2rust::src_loc = "132:1"]
    pub unsafe extern "C" fn luaH_object_push(
        mut L: *mut lua_State,
        mut p: gpointer,
    ) -> gint {
        luaH_object_registry_push(L);
        lua_pushlightuserdata(L, p);
        lua_rawget(L, -(2 as std::ffi::c_int));
        lua_remove(L, -(2 as std::ffi::c_int));
        return 1 as std::ffi::c_int;
    }
    use super::lua_h::{
        lua_State, lua_pushlstring, lua_rawget, lua_settop, lua_pushlightuserdata,
        lua_remove,
    };
    use super::luaclass_h::lua_class_t;
    use super::gtypes_h::{gint, gpointer, gchar};
    extern "C" {
        #[c2rust::src_loc = "38:1"]
        pub fn luaH_settype(L: *mut lua_State, lua_class: *mut lua_class_t) -> gint;
        #[c2rust::src_loc = "40:1"]
        pub fn luaH_object_incref(L: *mut lua_State, tud: gint, oud: gint) -> gpointer;
        #[c2rust::src_loc = "41:1"]
        pub fn luaH_object_decref(L: *mut lua_State, tud: gint, oud: gpointer);
        #[c2rust::src_loc = "164:1"]
        pub fn luaH_object_emit_signal(
            L: *mut lua_State,
            oud: gint,
            name: *const gchar,
            nargs: gint,
            nret: gint,
        ) -> gint;
        #[c2rust::src_loc = "167:1"]
        pub fn luaH_object_add_signal_simple(L: *mut lua_State) -> gint;
        #[c2rust::src_loc = "168:1"]
        pub fn luaH_object_remove_signal_simple(L: *mut lua_State) -> gint;
        #[c2rust::src_loc = "169:1"]
        pub fn luaH_object_remove_signals_simple(L: *mut lua_State) -> gint;
        #[c2rust::src_loc = "170:1"]
        pub fn luaH_object_emit_signal_simple(L: *mut lua_State) -> gint;
        #[c2rust::src_loc = "203:1"]
        pub fn luaH_object_tostring(_: *mut lua_State) -> gint;
        #[c2rust::src_loc = "204:1"]
        pub fn luaH_object_gc(_: *mut lua_State) -> gint;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luauniq.h:23"]
pub mod luauniq_h {
    use super::lua_h::lua_State;
    use super::gtypes_h::{gchar, gpointer};
    extern "C" {
        #[c2rust::src_loc = "31:1"]
        pub fn luaH_uniq_setup(L: *mut lua_State, reg: *const gchar, mode: *const gchar);
        #[c2rust::src_loc = "33:1"]
        pub fn luaH_uniq_add_ptr(
            L: *mut lua_State,
            reg: *const gchar,
            key: gpointer,
            oud: std::ffi::c_int,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "35:1"]
        pub fn luaH_uniq_get_ptr(
            L: *mut lua_State,
            reg: *const gchar,
            key: gpointer,
        ) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luah.h:25"]
pub mod luah_h {
    #[inline]
    #[c2rust::src_loc = "136:1"]
    pub unsafe extern "C" fn luaH_warn(
        mut L: *mut lua_State,
        mut fmt: *const gchar,
        mut args: ...
    ) {
        let mut top: gint = lua_gettop(L);
        let mut ar: lua_Debug = lua_Debug {
            event: 0,
            name: 0 as *const std::ffi::c_char,
            namewhat: 0 as *const std::ffi::c_char,
            what: 0 as *const std::ffi::c_char,
            source: 0 as *const std::ffi::c_char,
            currentline: 0,
            nups: 0,
            linedefined: 0,
            lastlinedefined: 0,
            short_src: [0; 60],
            i_ci: 0,
        };
        lua_getstack(L, 1 as std::ffi::c_int, &mut ar);
        lua_getinfo(L, b"Sln\0" as *const u8 as *const std::ffi::c_char, &mut ar);
        let mut __n1: gint64 = top as gint64;
        let mut __n2: gint64 = lua_gettop(L) as gint64;
        if !(__n1 == __n2) {
            g_assertion_message_cmpint(
                0 as *mut gchar,
                b"./common/luah.h\0" as *const u8 as *const std::ffi::c_char,
                142 as std::ffi::c_int,
                (*::core::mem::transmute::<
                    &[u8; 10],
                    &[std::ffi::c_char; 10],
                >(b"luaH_warn\0"))
                    .as_ptr(),
                b"top == lua_gettop(L)\0" as *const u8 as *const std::ffi::c_char,
                __n1 as guint64,
                b"==\0" as *const u8 as *const std::ffi::c_char,
                __n2 as guint64,
                'i' as i32 as std::ffi::c_char,
            );
        }
        let mut ap: ::core::ffi::VaListImpl;
        ap = args.clone();
        va_log(LOG_LEVEL_warn, (ar.short_src).as_mut_ptr(), fmt, ap.as_va_list());
    }
    #[inline]
    #[c2rust::src_loc = "150:1"]
    pub unsafe extern "C" fn luaH_rawfield(
        mut L: *mut lua_State,
        mut idx: gint,
        mut field: *const gchar,
    ) -> gint {
        lua_pushstring(L, field);
        lua_rawget(L, idx);
        let mut type_0: gint = lua_type(L, -(1 as std::ffi::c_int));
        if type_0 == 0 as std::ffi::c_int {
            lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        }
        return type_0;
    }
    use super::lua_h::{
        lua_State, lua_gettop, lua_Debug, lua_getstack, lua_getinfo, lua_pushstring,
        lua_rawget, lua_type, lua_settop,
    };
    use super::gtypes_h::{gchar, gint};
    use super::glibconfig_h::{gint64, guint64};
    use super::gtestutils_h::g_assertion_message_cmpint;
    use super::log_h::{va_log, LOG_LEVEL_warn, log_level_t};
}
#[c2rust::header_src = "/home/daana/git/luakit/web_context.h:27"]
pub mod web_context_h {
    use super::WebKitWebContext_h::WebKitWebContext;
    extern "C" {
        #[c2rust::src_loc = "28:1"]
        pub fn web_context_get() -> *mut WebKitWebContext;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gstdio.h:30"]
pub mod gstdio_h {
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "93:1"]
        pub fn g_unlink(filename: *const gchar) -> std::ffi::c_int;
    }
}
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::__stddef_size_t_h::size_t;
pub use self::glibconfig_h::{guint32, gint64, guint64, gsize};
pub use self::gtypes_h::{
    gchar, glong, gint, gboolean, gulong, guint, gfloat, gdouble, gpointer,
    gconstpointer, GCompareDataFunc, GDestroyNotify,
};
pub use self::garray_h::{_GPtrArray, GPtrArray, g_ptr_array_free};
pub use self::__stdarg_va_list_h::va_list;
pub use self::gquark_h::GQuark;
pub use self::gerror_h::{_GError, GError};
pub use self::gdataset_h::{GData, _GData};
pub use self::ghash_h::{GHashTable, _GHashTable};
pub use self::gtree_h::{GTree, _GTree, g_tree_new_full};
pub use self::gtype_h::{
    GType, GValue, _GTypeClass, GTypeClass, _GTypeInstance, GTypeInstance,
    g_type_check_instance_cast,
};
pub use self::gvalue_h::{_GValue, C2RustUnnamed};
pub use self::gparam_h::{
    GParamFlags, G_PARAM_DEPRECATED, G_PARAM_EXPLICIT_NOTIFY, G_PARAM_STATIC_BLURB,
    G_PARAM_STATIC_NICK, G_PARAM_PRIVATE, G_PARAM_STATIC_NAME, G_PARAM_LAX_VALIDATION,
    G_PARAM_CONSTRUCT_ONLY, G_PARAM_CONSTRUCT, G_PARAM_READWRITE, G_PARAM_WRITABLE,
    G_PARAM_READABLE, _GParamSpec, GParamSpec,
};
pub use self::gclosure_h::{
    _GClosure, GClosureNotifyData, _GClosureNotifyData, GClosureNotify, GClosure,
    GCallback,
};
pub use self::gsignal_h::{
    GConnectFlags, G_CONNECT_SWAPPED, G_CONNECT_AFTER, G_CONNECT_DEFAULT,
    g_signal_connect_data,
};
pub use self::gobject_h::{_GObject, GObject, g_object_ref, g_object_unref};
pub use self::lua_h::{
    lua_CFunction, lua_Number, lua_Debug, lua_State, lua_gettop, lua_settop,
    lua_pushvalue, lua_remove, lua_isstring, lua_type, lua_toboolean, lua_tolstring,
    lua_pushnumber, lua_pushlstring, lua_pushstring, lua_pushfstring, lua_pushboolean,
    lua_pushlightuserdata, lua_rawget, lua_createtable, lua_newuserdata,
    lua_setmetatable, lua_setfenv, lua_error, lua_getstack, lua_getinfo,
};
pub use self::log_h::{
    log_level_t, LOG_LEVEL_debug, LOG_LEVEL_verbose, LOG_LEVEL_info, LOG_LEVEL_warn,
    LOG_LEVEL_error, LOG_LEVEL_fatal, _log, va_log,
};
pub use self::signal_h::{signal_t, signal_cmp, signal_array_destroy, signal_new};
pub use self::tokenize_h::{
    luakit_token_t, L_TK_ZOOM_TEXT_ONLY, L_TK_ZOOM_LEVEL, L_TK_YPAGE_SIZE, L_TK_YMAX,
    L_TK_Y, L_TK_XPAGE_SIZE, L_TK_XMAX, L_TK_X, L_TK_WRAP_JS, L_TK_WIN_XID, L_TK_WINDOWS,
    L_TK_WINDOW, L_TK_WIDTH, L_TK_WEB_PROCESS_ID, L_TK_WEBVIEW, L_TK_WEBSITE_DATA,
    L_TK_WEBKIT_VERSION, L_TK_WEBKIT_USER_AGENT_VERSION, L_TK_WEBKIT2, L_TK_VPANED,
    L_TK_VISIBLE_CHILD, L_TK_VISIBLE, L_TK_VIDEOS_DIR, L_TK_VERSION, L_TK_VERBOSE,
    L_TK_VBOX, L_TK_VALUE, L_TK_USER_AGENT, L_TK_URI, L_TK_URGENCY_HINT, L_TK_TYPE,
    L_TK_TOTAL_SIZE, L_TK_TOP, L_TK_TOOLTIP, L_TK_TITLE, L_TK_TEXT_CONTENT,
    L_TK_TEXTWIDTH, L_TK_TEXT, L_TK_TEMPLATES_DIR, L_TK_TAG_NAME, L_TK_SYSTEM_DATA_DIRS,
    L_TK_SYSTEM_CONFIG_DIRS, L_TK_SWITCH, L_TK_SUGGESTED_FILENAME, L_TK_SUBMIT,
    L_TK_STYLESHEETS, L_TK_STYLE, L_TK_STOP, L_TK_STATUS, L_TK_STARTED, L_TK_START,
    L_TK_STACK, L_TK_SSL_TRUSTED, L_TK_SRC, L_TK_SPINNER, L_TK_SPELL_CHECKING_LANGUAGES,
    L_TK_SPACING, L_TK_SOURCE, L_TK_SOCKET, L_TK_SHOW_TABS, L_TK_SHOW_INSPECTOR,
    L_TK_SHOW_FRAME, L_TK_SHOW_BORDER, L_TK_SHOW, L_TK_SET_TITLE, L_TK_SET_PDFJS,
    L_TK_SET_FAVICON_FOR_URI, L_TK_SET_DEFAULT_SIZE, L_TK_SET_DARK_MODE,
    L_TK_SESSION_STATE, L_TK_SERIF_FONT_FAMILY, L_TK_SEND_KEY, L_TK_SELECT_REGION,
    L_TK_SELECTION, L_TK_SELECTABLE, L_TK_SECONDARY, L_TK_SEARCH_PREVIOUS,
    L_TK_SEARCH_NEXT, L_TK_SEARCH, L_TK_SCROLL_Y, L_TK_SCROLL_X, L_TK_SCROLLED,
    L_TK_SCROLLBARS, L_TK_SCROLL, L_TK_SCREEN, L_TK_SCALE, L_TK_SAVE,
    L_TK_SANS_SERIF_FONT_FAMILY, L_TK_ROOT_WIN_XID, L_TK_RIGHT, L_TK_RESOURCE_PATH,
    L_TK_REPLACE, L_TK_REORDER, L_TK_REMOVE_EVENT_LISTENER, L_TK_REMOVE,
    L_TK_RELOAD_BYPASS_CACHE, L_TK_RELOAD, L_TK_RECT, L_TK_QUERY, L_TK_PUBLIC_SHARE_DIR,
    L_TK_PROXY_URI, L_TK_PROGRESS, L_TK_PROCESS_LIMIT, L_TK_PRIVATE,
    L_TK_PRINT_BACKGROUNDS, L_TK_PRIMARY, L_TK_PREV_SIBLING, L_TK_POSITION, L_TK_PLUGGED,
    L_TK_PICTURES_DIR, L_TK_PICTOGRAPH_FONT_FAMILY, L_TK_PATTERN, L_TK_PARENT,
    L_TK_PACK2, L_TK_PACK1, L_TK_PACK, L_TK_OWNER_DOCUMENT, L_TK_OVERLAY, L_TK_OPTIONS,
    L_TK_NOUNIQUE, L_TK_NOTEBOOK, L_TK_NEXT_SIBLING, L_TK_NAME, L_TK_MUSIC_DIR,
    L_TK_MONOSPACE_FONT_FAMILY, L_TK_MIN_SIZE, L_TK_MINIMUM_FONT_SIZE, L_TK_MIME_TYPE,
    L_TK_MEDIA_PLAYBACK_REQUIRES_GESTURE, L_TK_MEDIA_PLAYBACK_ALLOWS_INLINE,
    L_TK_MAXIMIZED, L_TK_MARGIN_TOP, L_TK_MARGIN_RIGHT, L_TK_MARGIN_LEFT,
    L_TK_MARGIN_BOTTOM, L_TK_MARGIN, L_TK_LOAD_STRING, L_TK_LOADING, L_TK_LEFT,
    L_TK_LAST_CHILD, L_TK_LABEL, L_TK_JAVASCRIPT_CAN_OPEN_WINDOWS_AUTOMATICALLY,
    L_TK_JAVASCRIPT_CAN_ACCESS_CLIPBOARD, L_TK_IS_PLAYING_AUDIO, L_TK_IS_LOADING,
    L_TK_IS_ALIVE, L_TK_INVALIDATE, L_TK_INTERVAL, L_TK_INSTALL_PATHS, L_TK_INSTALL_PATH,
    L_TK_INSPECTOR, L_TK_INSERT, L_TK_INNER_WIDTH, L_TK_INNER_HTML, L_TK_INNER_HEIGHT,
    L_TK_INDEXOF, L_TK_IMAGE, L_TK_ID, L_TK_ICON, L_TK_HREF, L_TK_HPANED,
    L_TK_HOVERED_URI, L_TK_HOMOGENEOUS, L_TK_HISTORY, L_TK_HIDE, L_TK_HEIGHT, L_TK_HBOX,
    L_TK_HARDWARE_ACCELERATION_POLICY, L_TK_GO_FORWARD, L_TK_GO_BACK, L_TK_GET_TITLE,
    L_TK_GET_SOURCE, L_TK_FULLSCREEN, L_TK_FONT, L_TK_FOCUSED, L_TK_FOCUS,
    L_TK_FIRST_CHILD, L_TK_FINISHED, L_TK_FILL, L_TK_FILENAME, L_TK_FG, L_TK_FETCH,
    L_TK_FANTASY_FONT_FAMILY, L_TK_EXECPATH, L_TK_EVENTBOX, L_TK_EVAL_JS, L_TK_ERROR,
    L_TK_ENTRY, L_TK_END, L_TK_ENABLE_XSS_AUDITOR,
    L_TK_ENABLE_WRITE_CONSOLE_MESSAGES_TO_STDOUT, L_TK_ENABLE_WEBGL,
    L_TK_ENABLE_WEBAUDIO, L_TK_ENABLE_TABS_TO_LINKS, L_TK_ENABLE_SPELL_CHECKING,
    L_TK_ENABLE_SPATIAL_NAVIGATION, L_TK_ENABLE_SMOOTH_SCROLLING,
    L_TK_ENABLE_SITE_SPECIFIC_QUIRKS, L_TK_ENABLE_SCRIPTS,
    L_TK_ENABLE_RESIZABLE_TEXT_AREAS, L_TK_ENABLE_PLUGINS, L_TK_ENABLE_PAGE_CACHE,
    L_TK_ENABLE_MEDIA_STREAM, L_TK_ENABLE_MEDIASOURCE, L_TK_ENABLE_JAVASCRIPT,
    L_TK_ENABLE_JAVA, L_TK_ENABLE_HYPERLINK_AUDITING, L_TK_ENABLE_HTML5_LOCAL_STORAGE,
    L_TK_ENABLE_HTML5_DATABASE, L_TK_ENABLE_FULLSCREEN, L_TK_ENABLE_FRAME_FLATTENING,
    L_TK_ENABLE_DNS_PREFETCHING, L_TK_ENABLE_DEVELOPER_EXTRAS,
    L_TK_ENABLE_CARET_BROWSING, L_TK_ENABLE_ACCELERATED_2D_CANVAS,
    L_TK_ELEMENT_FROM_POINT, L_TK_ELAPSED_TIME, L_TK_EDITABLE,
    L_TK_DRAW_COMPOSITING_INDICATORS, L_TK_DRAWING_AREA, L_TK_DOWNLOAD_DIR,
    L_TK_DOCUMENTS_DIR, L_TK_DOCUMENT, L_TK_DEV_PATHS, L_TK_DESTROY, L_TK_DESTINATION,
    L_TK_DESKTOP_DIR, L_TK_DEFAULT_MONOSPACE_FONT_SIZE, L_TK_DEFAULT_FONT_SIZE,
    L_TK_DEFAULT_FONT_FAMILY, L_TK_DEFAULT_CHARSET, L_TK_DECORATED, L_TK_DATA_DIR,
    L_TK_CURSIVE_FONT_FAMILY, L_TK_CURRENT_SIZE, L_TK_CURRENT, L_TK_CSS,
    L_TK_CREATE_ELEMENT, L_TK_CRASH, L_TK_COUNT, L_TK_COOKIES_STORAGE, L_TK_CONFPATH,
    L_TK_CONFIG_DIR, L_TK_CLOSE_INSPECTOR, L_TK_CLIPBOARD, L_TK_CLIENT_RECTS, L_TK_CLICK,
    L_TK_CLEAR_SEARCH, L_TK_CLEAR, L_TK_CHILD_COUNT, L_TK_CHILDREN, L_TK_CHILD,
    L_TK_CHECKED, L_TK_CERTIFICATE, L_TK_CENTER, L_TK_CAN_GO_FORWARD, L_TK_CAN_GO_BACK,
    L_TK_CAN_FOCUS, L_TK_CACHE_DIR, L_TK_BOTTOM, L_TK_BODY, L_TK_BG, L_TK_BASELINE,
    L_TK_AUTO_LOAD_IMAGES, L_TK_ATTR, L_TK_APPEND,
    L_TK_ALLOW_UNIVERSAL_ACCESS_FROM_FILE_URLS, L_TK_ALLOW_OVERWRITE,
    L_TK_ALLOW_MODAL_DIALOGS, L_TK_ALLOW_FILE_ACCESS_FROM_FILE_URLS,
    L_TK_ALLOW_CERTIFICATE, L_TK_ALIGN, L_TK_ADD_EVENT_LISTENER, L_TK_ACCEPT_POLICY,
    L_TK_UNKNOWN,
};
pub use self::lauxlib_h::{luaL_Reg, luaL_typerror, luaL_checklstring, luaL_error};
pub use self::luaclass_h::{
    lua_class_property_array_t, lua_object_t, lua_class_allocator_t,
    lua_class_propfunc_t, lua_class_t, luaH_class_add_signal, luaH_class_remove_signal,
    luaH_class_emit_signal, luaH_class_setup, luaH_class_add_property, luaH_class_index,
    luaH_class_newindex, luaH_checkudata,
};
pub use self::common_h::{_common_t, common_t, common};
pub use self::WebKitURIRequest_h::{
    _WebKitURIRequest, WebKitURIRequestPrivate, WebKitURIRequest,
    _WebKitURIRequestPrivate, webkit_uri_request_get_uri,
};
pub use self::WebKitURIResponse_h::{
    _WebKitURIResponse, WebKitURIResponsePrivate, WebKitURIResponse,
    _WebKitURIResponsePrivate, webkit_uri_response_get_content_length,
    webkit_uri_response_get_mime_type, webkit_uri_response_get_suggested_filename,
};
pub use self::WebKitDownload_h::{
    _WebKitDownload, WebKitDownloadPrivate, WebKitDownload, _WebKitDownloadPrivate,
    webkit_download_get_request, webkit_download_set_destination,
    webkit_download_get_response, webkit_download_cancel,
    webkit_download_get_estimated_progress, webkit_download_get_elapsed_time,
    webkit_download_get_received_data_length, webkit_download_get_allow_overwrite,
    webkit_download_set_allow_overwrite,
};
pub use self::WebKitError_h::{
    C2RustUnnamed_0, WEBKIT_DOWNLOAD_ERROR_DESTINATION,
    WEBKIT_DOWNLOAD_ERROR_CANCELLED_BY_USER, WEBKIT_DOWNLOAD_ERROR_NETWORK,
};
pub use self::WebKitWebContext_h::{
    _WebKitWebContext, WebKitWebContextPrivate, WebKitWebContext,
    _WebKitWebContextPrivate, webkit_web_context_download_uri,
};
use self::string_h::{memcpy, memset, strlen};
use self::gconvert_h::g_filename_to_uri;
use self::gmem_h::{g_free, g_malloc};
pub use self::gstrfuncs_h::{g_strdup_inline, g_strrstr, g_strdup, g_strdup_printf};
use self::gtestutils_h::{g_strcmp0, g_assertion_message_cmpint};
pub use self::luaobject_h::{
    luaH_object_registry_push, luaH_object_ref, luaH_object_unref, luaH_object_push,
    luaH_settype, luaH_object_incref, luaH_object_decref, luaH_object_emit_signal,
    luaH_object_add_signal_simple, luaH_object_remove_signal_simple,
    luaH_object_remove_signals_simple, luaH_object_emit_signal_simple,
    luaH_object_tostring, luaH_object_gc,
};
use self::luauniq_h::{luaH_uniq_setup, luaH_uniq_add_ptr, luaH_uniq_get_ptr};
pub use self::luah_h::{luaH_warn, luaH_rawfield};
use self::web_context_h::web_context_get;
use self::gstdio_h::g_unlink;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "35:9"]
pub struct download_t {
    pub signals: *mut signal_t,
    pub webkit_download: *mut WebKitDownload,
    pub ref_0: gpointer,
    pub uri: *mut gchar,
    pub destination: *mut gchar,
    pub error: *mut gchar,
    pub status: luakit_download_status_t,
}
#[c2rust::src_loc = "52:5"]
pub type luakit_download_status_t = std::ffi::c_uint;
#[c2rust::src_loc = "57:9"]
pub const LUAKIT_DOWNLOAD_STATUS_FAILED: luakit_download_status_t = 4;
#[c2rust::src_loc = "56:9"]
pub const LUAKIT_DOWNLOAD_STATUS_CANCELLED: luakit_download_status_t = 3;
#[c2rust::src_loc = "55:9"]
pub const LUAKIT_DOWNLOAD_STATUS_STARTED: luakit_download_status_t = 2;
#[c2rust::src_loc = "54:9"]
pub const LUAKIT_DOWNLOAD_STATUS_CREATED: luakit_download_status_t = 1;
#[c2rust::src_loc = "53:9"]
pub const LUAKIT_DOWNLOAD_STATUS_FINISHED: luakit_download_status_t = 0;
#[c2rust::src_loc = "61:20"]
static mut download_class: lua_class_t = lua_class_t {
    name: 0 as *const gchar,
    signals: 0 as *const signal_t as *mut signal_t,
    allocator: None,
    properties: 0 as *const lua_class_property_array_t
        as *mut lua_class_property_array_t,
    index_miss_property: None,
    newindex_miss_property: None,
};
#[inline]
#[c2rust::src_loc = "62:1"]
unsafe extern "C" fn luaH_download_class_emit_signal(mut L: *mut lua_State) -> gint {
    return luaH_class_emit_signal(
        L,
        &mut download_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        lua_gettop(L) - 1 as std::ffi::c_int,
        -(1 as std::ffi::c_int),
    );
}
#[inline]
#[c2rust::src_loc = "62:1"]
unsafe extern "C" fn luaH_download_class_remove_signal(mut L: *mut lua_State) -> gint {
    luaH_class_remove_signal(
        L,
        &mut download_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]
#[c2rust::src_loc = "62:1"]
unsafe extern "C" fn download_new(mut L: *mut lua_State) -> *mut download_t {
    let mut p: *mut download_t = lua_newuserdata(
        L,
        ::core::mem::size_of::<download_t>() as std::ffi::c_ulong,
    ) as *mut download_t;
    memset(
        p as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        (::core::mem::size_of::<download_t>() as std::ffi::c_ulong)
            .wrapping_mul(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    (*p).signals = signal_new();
    luaH_settype(L, &mut download_class);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    lua_setfenv(L, -(2 as std::ffi::c_int));
    lua_pushvalue(L, -(1 as std::ffi::c_int));
    luaH_class_emit_signal(
        L,
        &mut download_class,
        b"new\0" as *const u8 as *const std::ffi::c_char,
        1 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    return p;
}
#[inline]
#[c2rust::src_loc = "62:1"]
unsafe extern "C" fn luaH_download_class_add_signal(mut L: *mut lua_State) -> gint {
    luaH_class_add_signal(
        L,
        &mut download_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, 0 as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[c2rust::src_loc = "64:20"]
static mut current_destination_cb: *mut download_t = 0 as *const download_t
    as *mut download_t;
#[c2rust::src_loc = "78:1"]
unsafe extern "C" fn luaH_download_unref(
    mut L: *mut lua_State,
    mut download: *mut download_t,
) {
    if !((*download).ref_0).is_null() {
        luaH_object_unref(L, (*download).ref_0);
        (*download).ref_0 = 0 as *mut std::ffi::c_void;
    }
    let mut backup: *mut gchar = g_strdup_printf(
        b"%s~\0" as *const u8 as *const std::ffi::c_char,
        (*download).destination,
    );
    g_unlink(backup);
    g_free(backup as gpointer);
}
#[c2rust::src_loc = "101:1"]
unsafe extern "C" fn luaH_download_gc(mut L: *mut lua_State) -> gint {
    let mut download: *mut download_t = luaH_checkudata(
        L,
        1 as std::ffi::c_int,
        &mut download_class,
    ) as *mut download_t;
    g_object_unref(
        g_type_check_instance_cast(
            (*download).webkit_download as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject as gpointer,
    );
    g_free((*download).destination as gpointer);
    g_free((*download).uri as gpointer);
    g_free((*download).error as gpointer);
    return luaH_object_gc(L);
}
#[c2rust::src_loc = "121:1"]
unsafe extern "C" fn decide_destination_cb(
    mut UNUSED_dl: *mut WebKitDownload,
    mut suggested_filename: *mut gchar,
    mut download: *mut download_t,
) -> gboolean {
    let mut L: *mut lua_State = common.L;
    luaH_object_push(L, (*download).ref_0);
    lua_pushstring(L, suggested_filename);
    current_destination_cb = download;
    let mut ret: gint = luaH_object_emit_signal(
        L,
        -(2 as std::ffi::c_int),
        b"decide-destination\0" as *const u8 as *const std::ffi::c_char,
        1 as std::ffi::c_int,
        1 as std::ffi::c_int,
    );
    let mut handled: gboolean = (ret != 0
        && lua_toboolean(L, -(1 as std::ffi::c_int)) != 0) as std::ffi::c_int;
    lua_settop(L, -(1 as std::ffi::c_int + ret) - 1 as std::ffi::c_int);
    current_destination_cb = 0 as *mut download_t;
    if (*download).status as std::ffi::c_uint
        == LUAKIT_DOWNLOAD_STATUS_CANCELLED as std::ffi::c_int as std::ffi::c_uint
    {
        webkit_download_set_destination(
            (*download).webkit_download,
            b"/tmp/\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    return handled;
}
#[c2rust::src_loc = "146:1"]
unsafe extern "C" fn created_destination_cb(
    mut UNUSED_dl: *mut WebKitDownload,
    mut destination: *mut gchar,
    mut download: *mut download_t,
) {
    let mut L: *mut lua_State = common.L;
    luaH_object_push(L, (*download).ref_0);
    lua_pushstring(L, destination);
    (*download).status = LUAKIT_DOWNLOAD_STATUS_CREATED;
    if !((*download).error).is_null() {
        g_free((*download).error as gpointer);
        (*download).error = 0 as *mut gchar;
    }
    luaH_object_emit_signal(
        L,
        -(2 as std::ffi::c_int),
        b"created-destination\0" as *const u8 as *const std::ffi::c_char,
        1 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
#[c2rust::src_loc = "171:1"]
unsafe extern "C" fn failed_cb(
    mut UNUSED_d: *mut WebKitDownload,
    mut error: *mut GError,
    mut download: *mut download_t,
) {
    if !((*download).error).is_null() {
        g_free((*download).error as gpointer);
    }
    (*download).error = g_strdup_inline((*error).message);
    if (*error).code == WEBKIT_DOWNLOAD_ERROR_CANCELLED_BY_USER as std::ffi::c_int {
        (*download).status = LUAKIT_DOWNLOAD_STATUS_CANCELLED;
    } else {
        _log(
            LOG_LEVEL_warn,
            b"clib/download.c\0" as *const u8 as *const std::ffi::c_char,
            b"download %p failed: %s\0" as *const u8 as *const std::ffi::c_char,
            download,
            (*error).message,
        );
        (*download).status = LUAKIT_DOWNLOAD_STATUS_FAILED;
        if !((*download).ref_0).is_null() {
            let mut L: *mut lua_State = common.L;
            luaH_object_push(L, (*download).ref_0);
            lua_pushstring(L, (*error).message);
            luaH_object_emit_signal(
                L,
                -(2 as std::ffi::c_int),
                b"error\0" as *const u8 as *const std::ffi::c_char,
                1 as std::ffi::c_int,
                0 as std::ffi::c_int,
            );
            lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        }
    };
}
#[c2rust::src_loc = "197:1"]
unsafe extern "C" fn progress_cb(
    mut UNUSED_dl: *mut WebKitDownload,
    mut UNUSED_ps: *mut GParamSpec,
    mut download: *mut download_t,
) {
    (*download).status = LUAKIT_DOWNLOAD_STATUS_STARTED;
}
#[c2rust::src_loc = "207:1"]
unsafe extern "C" fn finished_cb(
    mut UNUSED_dl: *mut WebKitDownload,
    mut download: *mut download_t,
) {
    let mut L: *mut lua_State = common.L;
    luaH_object_push(L, (*download).ref_0);
    if (*download).status as std::ffi::c_uint
        != LUAKIT_DOWNLOAD_STATUS_CANCELLED as std::ffi::c_int as std::ffi::c_uint
        && (*download).status as std::ffi::c_uint
            != LUAKIT_DOWNLOAD_STATUS_FAILED as std::ffi::c_int as std::ffi::c_uint
    {
        (*download).status = LUAKIT_DOWNLOAD_STATUS_FINISHED;
    }
    let mut ret: gint = luaH_object_emit_signal(
        L,
        -(1 as std::ffi::c_int),
        b"finished\0" as *const u8 as *const std::ffi::c_char,
        0 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int + ret) - 1 as std::ffi::c_int);
    luaH_download_unref(L, download);
}
#[no_mangle]
#[c2rust::src_loc = "223:1"]
pub unsafe extern "C" fn luaH_download_new(mut L: *mut lua_State) -> gint {
    if !(lua_type(L, 2 as std::ffi::c_int) == 5 as std::ffi::c_int) {
        luaL_typerror(
            L,
            2 as std::ffi::c_int,
            b"table\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut uri: *const gchar = 0 as *const gchar;
    let mut top: gint = lua_gettop(L);
    if luaH_rawfield(
        L,
        2 as std::ffi::c_int,
        b"uri\0" as *const u8 as *const std::ffi::c_char,
    ) != 0 && lua_isstring(L, -(1 as std::ffi::c_int)) != 0
    {
        uri = lua_tolstring(L, -(1 as std::ffi::c_int), 0 as *mut size_t);
    }
    lua_settop(L, top);
    if uri.is_null() {
        return luaL_error(
            L,
            b"download requires a URI\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut d: *mut WebKitDownload = webkit_web_context_download_uri(
        web_context_get(),
        uri,
    );
    return luaH_download_push(L, d);
}
#[no_mangle]
#[c2rust::src_loc = "252:1"]
pub unsafe extern "C" fn luaH_download_push(
    mut L: *mut lua_State,
    mut d: *mut WebKitDownload,
) -> gint {
    if luaH_uniq_get_ptr(
        L,
        b"luakit.uniq.registry.download\0" as *const u8 as *const std::ffi::c_char,
        d as gpointer,
    ) != 0
    {
        return 1 as std::ffi::c_int;
    }
    (download_class.allocator).expect("non-null function pointer")(L);
    let mut download: *mut download_t = luaH_checkudata(
        L,
        -(1 as std::ffi::c_int),
        &mut download_class,
    ) as *mut download_t;
    let mut r: *mut WebKitURIRequest = webkit_download_get_request(d);
    (*download).uri = g_strdup_inline(webkit_uri_request_get_uri(r));
    (*download).webkit_download = d;
    g_object_ref(
        g_type_check_instance_cast(
            (*download).webkit_download as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject as gpointer,
    );
    g_signal_connect_data(
        g_type_check_instance_cast(
            (*download).webkit_download as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject as gpointer,
        b"decide-destination\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitDownload,
                    *mut gchar,
                    *mut download_t,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                decide_destination_cb
                    as unsafe extern "C" fn(
                        *mut WebKitDownload,
                        *mut gchar,
                        *mut download_t,
                    ) -> gboolean,
            ),
        ),
        download as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        g_type_check_instance_cast(
            (*download).webkit_download as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject as gpointer,
        b"created-destination\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitDownload,
                    *mut gchar,
                    *mut download_t,
                ) -> (),
            >,
            GCallback,
        >(
            Some(
                created_destination_cb
                    as unsafe extern "C" fn(
                        *mut WebKitDownload,
                        *mut gchar,
                        *mut download_t,
                    ) -> (),
            ),
        ),
        download as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        g_type_check_instance_cast(
            (*download).webkit_download as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject as gpointer,
        b"notify::estimated-progress\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitDownload,
                    *mut GParamSpec,
                    *mut download_t,
                ) -> (),
            >,
            GCallback,
        >(
            Some(
                progress_cb
                    as unsafe extern "C" fn(
                        *mut WebKitDownload,
                        *mut GParamSpec,
                        *mut download_t,
                    ) -> (),
            ),
        ),
        download as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        g_type_check_instance_cast(
            (*download).webkit_download as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject as gpointer,
        b"finished\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut WebKitDownload, *mut download_t) -> ()>,
            GCallback,
        >(
            Some(
                finished_cb
                    as unsafe extern "C" fn(*mut WebKitDownload, *mut download_t) -> (),
            ),
        ),
        download as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    g_signal_connect_data(
        g_type_check_instance_cast(
            (*download).webkit_download as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject as gpointer,
        b"failed\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitDownload,
                    *mut GError,
                    *mut download_t,
                ) -> (),
            >,
            GCallback,
        >(
            Some(
                failed_cb
                    as unsafe extern "C" fn(
                        *mut WebKitDownload,
                        *mut GError,
                        *mut download_t,
                    ) -> (),
            ),
        ),
        download as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    lua_pushvalue(L, -(1 as std::ffi::c_int));
    (*download).ref_0 = luaH_object_ref(L, -(1 as std::ffi::c_int));
    luaH_uniq_add_ptr(
        L,
        b"luakit.uniq.registry.download\0" as *const u8 as *const std::ffi::c_char,
        d as gpointer,
        -(1 as std::ffi::c_int),
    );
    return 1 as std::ffi::c_int;
}
#[c2rust::src_loc = "299:1"]
unsafe extern "C" fn luaH_download_set_allow_overwrite(
    mut L: *mut lua_State,
    mut download: *mut download_t,
) -> gint {
    let mut allow: gboolean = lua_toboolean(L, -(1 as std::ffi::c_int));
    webkit_download_set_allow_overwrite((*download).webkit_download, allow);
    luaH_object_emit_signal(
        L,
        -(3 as std::ffi::c_int),
        b"property::allow-overwrite\0" as *const u8 as *const std::ffi::c_char,
        0 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[c2rust::src_loc = "308:1"]
unsafe extern "C" fn luaH_download_get_allow_overwrite(
    mut L: *mut lua_State,
    mut download: *mut download_t,
) -> gint {
    lua_pushboolean(L, webkit_download_get_allow_overwrite((*download).webkit_download));
    return 1 as std::ffi::c_int;
}
#[c2rust::src_loc = "327:1"]
unsafe extern "C" fn luaH_download_set_destination(
    mut L: *mut lua_State,
    mut download: *mut download_t,
) -> gint {
    if download != current_destination_cb {
        luaH_warn(
            L,
            b"cannot set destination outside decide-destination handler\0" as *const u8
                as *const std::ffi::c_char,
        );
        return 0 as std::ffi::c_int;
    }
    let mut destination: *const gchar = luaL_checklstring(
        L,
        -(1 as std::ffi::c_int),
        0 as *mut size_t,
    );
    let mut uri: *mut gchar = g_filename_to_uri(
        destination,
        0 as *const gchar,
        0 as *mut *mut GError,
    );
    if !uri.is_null() {
        (*download).destination = g_strdup_inline(destination);
        webkit_download_set_destination((*download).webkit_download, uri);
        g_free(uri as gpointer);
        luaH_object_emit_signal(
            L,
            -(3 as std::ffi::c_int),
            b"property::destination\0" as *const u8 as *const std::ffi::c_char,
            0 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
    } else {
        lua_pushfstring(
            L,
            b"invalid destination: '%s'\0" as *const u8 as *const std::ffi::c_char,
            destination,
        );
        lua_error(L);
    }
    return 0 as std::ffi::c_int;
}
#[c2rust::src_loc = "360:1"]
unsafe extern "C" fn luaH_download_get_destination(
    mut L: *mut lua_State,
    mut object: *mut download_t,
) -> gint {
    lua_pushstring(L, (*object).destination);
    return 1 as std::ffi::c_int;
}
#[c2rust::src_loc = "372:1"]
unsafe extern "C" fn luaH_download_get_progress(
    mut L: *mut lua_State,
    mut download: *mut download_t,
) -> gint {
    let mut progress: gdouble = webkit_download_get_estimated_progress(
        (*download).webkit_download,
    );
    lua_pushnumber(L, progress);
    return 1 as std::ffi::c_int;
}
#[c2rust::src_loc = "390:1"]
unsafe extern "C" fn luaH_download_get_mime_type(
    mut L: *mut lua_State,
    mut download: *mut download_t,
) -> gint {
    let mut response: *mut WebKitURIResponse = webkit_download_get_response(
        (*download).webkit_download,
    );
    if response.is_null() {
        return 0 as std::ffi::c_int;
    }
    let mut mime_type: *const gchar = webkit_uri_response_get_mime_type(response);
    if !mime_type.is_null() {
        lua_pushstring(L, mime_type);
        return 1 as std::ffi::c_int;
    }
    return 0 as std::ffi::c_int;
}
#[c2rust::src_loc = "424:1"]
unsafe extern "C" fn luaH_download_get_status(
    mut L: *mut lua_State,
    mut download: *mut download_t,
) -> gint {
    match (*download).status as std::ffi::c_uint {
        0 => {
            lua_pushstring(L, b"finished\0" as *const u8 as *const std::ffi::c_char);
        }
        1 => {
            lua_pushstring(L, b"created\0" as *const u8 as *const std::ffi::c_char);
        }
        2 => {
            lua_pushstring(L, b"started\0" as *const u8 as *const std::ffi::c_char);
        }
        3 => {
            lua_pushstring(L, b"cancelled\0" as *const u8 as *const std::ffi::c_char);
        }
        4 => {
            lua_pushstring(L, b"failed\0" as *const u8 as *const std::ffi::c_char);
        }
        _ => {
            luaH_warn(
                L,
                b"unknown download status\0" as *const u8 as *const std::ffi::c_char,
            );
            return 0 as std::ffi::c_int;
        }
    }
    return 1 as std::ffi::c_int;
}
#[c2rust::src_loc = "463:1"]
unsafe extern "C" fn luaH_download_get_error(
    mut L: *mut lua_State,
    mut object: *mut download_t,
) -> gint {
    lua_pushstring(L, (*object).error);
    return 1 as std::ffi::c_int;
}
#[c2rust::src_loc = "477:1"]
unsafe extern "C" fn luaH_download_get_content_length(
    mut L: *mut lua_State,
    mut download: *mut download_t,
) -> gint {
    let mut total_size: gdouble = webkit_uri_response_get_content_length(
        webkit_download_get_response((*download).webkit_download),
    ) as gdouble;
    lua_pushnumber(L, total_size);
    return 1 as std::ffi::c_int;
}
#[c2rust::src_loc = "496:1"]
unsafe extern "C" fn luaH_download_get_received_data_length(
    mut L: *mut lua_State,
    mut download: *mut download_t,
) -> gint {
    let mut current_size: gdouble = webkit_download_get_received_data_length(
        (*download).webkit_download,
    ) as gdouble;
    lua_pushnumber(L, current_size);
    return 1 as std::ffi::c_int;
}
#[c2rust::src_loc = "515:1"]
unsafe extern "C" fn luaH_download_get_elapsed_time(
    mut L: *mut lua_State,
    mut download: *mut download_t,
) -> gint {
    let mut elapsed_time: gdouble = webkit_download_get_elapsed_time(
        (*download).webkit_download,
    );
    lua_pushnumber(L, elapsed_time);
    return 1 as std::ffi::c_int;
}
#[c2rust::src_loc = "535:1"]
unsafe extern "C" fn luaH_download_get_suggested_filename(
    mut L: *mut lua_State,
    mut download: *mut download_t,
) -> gint {
    let mut suggested_filename: *const gchar = webkit_uri_response_get_suggested_filename(
        webkit_download_get_response((*download).webkit_download),
    );
    lua_pushstring(L, suggested_filename);
    return 1 as std::ffi::c_int;
}
#[c2rust::src_loc = "558:1"]
unsafe extern "C" fn luaH_download_set_uri(
    mut L: *mut lua_State,
    mut download: *mut download_t,
) -> gint {
    let mut uri: *mut gchar = luaL_checklstring(
        L,
        -(1 as std::ffi::c_int),
        0 as *mut size_t,
    ) as *mut gchar;
    if !(g_strrstr(uri, b"://\0" as *const u8 as *const std::ffi::c_char)).is_null() {
        uri = g_strdup_inline(uri);
    } else {
        uri = g_strdup_printf(
            b"http://%s\0" as *const u8 as *const std::ffi::c_char,
            uri,
        );
    }
    (*download).uri = uri;
    return 0 as std::ffi::c_int;
}
#[c2rust::src_loc = "581:1"]
unsafe extern "C" fn luaH_download_get_uri(
    mut L: *mut lua_State,
    mut object: *mut download_t,
) -> gint {
    lua_pushstring(L, (*object).uri);
    return 1 as std::ffi::c_int;
}
#[c2rust::src_loc = "595:1"]
unsafe extern "C" fn luaH_download_start(mut UNUSED_L: *mut lua_State) -> gint {
    return 0 as std::ffi::c_int;
}
#[c2rust::src_loc = "614:1"]
unsafe extern "C" fn luaH_download_cancel(mut L: *mut lua_State) -> gint {
    let mut download: *mut download_t = luaH_checkudata(
        L,
        1 as std::ffi::c_int,
        &mut download_class,
    ) as *mut download_t;
    webkit_download_cancel((*download).webkit_download);
    (*download).status = LUAKIT_DOWNLOAD_STATUS_CANCELLED;
    return 0 as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "628:1"]
pub unsafe extern "C" fn download_class_setup(mut L: *mut lua_State) {
    static mut download_methods: [luaL_Reg; 5] = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"add_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_download_class_add_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_download_class_remove_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"emit_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_download_class_emit_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__call\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_download_new as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: 0 as *const std::ffi::c_char,
                    func: None,
                };
                init
            },
        ]
    };
    static mut download_meta: [luaL_Reg; 11] = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"__tostring\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_object_tostring
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"add_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_object_add_signal_simple
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_object_remove_signal_simple
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signals\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_object_remove_signals_simple
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"emit_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_object_emit_signal_simple
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__index\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_class_index as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__newindex\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_class_newindex
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"start\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_download_start
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"cancel\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_download_cancel
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__gc\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_download_gc as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: 0 as *const std::ffi::c_char,
                    func: None,
                };
                init
            },
        ]
    };
    luaH_class_setup(
        L,
        &mut download_class,
        b"download\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut lua_State) -> *mut download_t>,
            lua_class_allocator_t,
        >(Some(download_new as unsafe extern "C" fn(*mut lua_State) -> *mut download_t)),
        None,
        None,
        download_methods.as_ptr(),
        download_meta.as_ptr(),
    );
    luaH_class_add_property(
        &mut download_class,
        L_TK_ALLOW_OVERWRITE,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut lua_State, *mut download_t) -> gint>,
            lua_class_propfunc_t,
        >(
            Some(
                luaH_download_set_allow_overwrite
                    as unsafe extern "C" fn(*mut lua_State, *mut download_t) -> gint,
            ),
        ),
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut lua_State, *mut download_t) -> gint>,
            lua_class_propfunc_t,
        >(
            Some(
                luaH_download_get_allow_overwrite
                    as unsafe extern "C" fn(*mut lua_State, *mut download_t) -> gint,
            ),
        ),
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut lua_State, *mut download_t) -> gint>,
            lua_class_propfunc_t,
        >(
            Some(
                luaH_download_set_allow_overwrite
                    as unsafe extern "C" fn(*mut lua_State, *mut download_t) -> gint,
            ),
        ),
    );
    luaH_class_add_property(
        &mut download_class,
        L_TK_DESTINATION,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut lua_State, *mut download_t) -> gint>,
            lua_class_propfunc_t,
        >(
            Some(
                luaH_download_set_destination
                    as unsafe extern "C" fn(*mut lua_State, *mut download_t) -> gint,
            ),
        ),
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut lua_State, *mut download_t) -> gint>,
            lua_class_propfunc_t,
        >(
            Some(
                luaH_download_get_destination
                    as unsafe extern "C" fn(*mut lua_State, *mut download_t) -> gint,
            ),
        ),
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut lua_State, *mut download_t) -> gint>,
            lua_class_propfunc_t,
        >(
            Some(
                luaH_download_set_destination
                    as unsafe extern "C" fn(*mut lua_State, *mut download_t) -> gint,
            ),
        ),
    );
    luaH_class_add_property(
        &mut download_class,
        L_TK_PROGRESS,
        None,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut lua_State, *mut download_t) -> gint>,
            lua_class_propfunc_t,
        >(
            Some(
                luaH_download_get_progress
                    as unsafe extern "C" fn(*mut lua_State, *mut download_t) -> gint,
            ),
        ),
        None,
    );
    luaH_class_add_property(
        &mut download_class,
        L_TK_STATUS,
        None,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut lua_State, *mut download_t) -> gint>,
            lua_class_propfunc_t,
        >(
            Some(
                luaH_download_get_status
                    as unsafe extern "C" fn(*mut lua_State, *mut download_t) -> gint,
            ),
        ),
        None,
    );
    luaH_class_add_property(
        &mut download_class,
        L_TK_ERROR,
        None,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut lua_State, *mut download_t) -> gint>,
            lua_class_propfunc_t,
        >(
            Some(
                luaH_download_get_error
                    as unsafe extern "C" fn(*mut lua_State, *mut download_t) -> gint,
            ),
        ),
        None,
    );
    luaH_class_add_property(
        &mut download_class,
        L_TK_TOTAL_SIZE,
        None,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut lua_State, *mut download_t) -> gint>,
            lua_class_propfunc_t,
        >(
            Some(
                luaH_download_get_content_length
                    as unsafe extern "C" fn(*mut lua_State, *mut download_t) -> gint,
            ),
        ),
        None,
    );
    luaH_class_add_property(
        &mut download_class,
        L_TK_CURRENT_SIZE,
        None,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut lua_State, *mut download_t) -> gint>,
            lua_class_propfunc_t,
        >(
            Some(
                luaH_download_get_received_data_length
                    as unsafe extern "C" fn(*mut lua_State, *mut download_t) -> gint,
            ),
        ),
        None,
    );
    luaH_class_add_property(
        &mut download_class,
        L_TK_ELAPSED_TIME,
        None,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut lua_State, *mut download_t) -> gint>,
            lua_class_propfunc_t,
        >(
            Some(
                luaH_download_get_elapsed_time
                    as unsafe extern "C" fn(*mut lua_State, *mut download_t) -> gint,
            ),
        ),
        None,
    );
    luaH_class_add_property(
        &mut download_class,
        L_TK_MIME_TYPE,
        None,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut lua_State, *mut download_t) -> gint>,
            lua_class_propfunc_t,
        >(
            Some(
                luaH_download_get_mime_type
                    as unsafe extern "C" fn(*mut lua_State, *mut download_t) -> gint,
            ),
        ),
        None,
    );
    luaH_class_add_property(
        &mut download_class,
        L_TK_SUGGESTED_FILENAME,
        None,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut lua_State, *mut download_t) -> gint>,
            lua_class_propfunc_t,
        >(
            Some(
                luaH_download_get_suggested_filename
                    as unsafe extern "C" fn(*mut lua_State, *mut download_t) -> gint,
            ),
        ),
        None,
    );
    luaH_class_add_property(
        &mut download_class,
        L_TK_URI,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut lua_State, *mut download_t) -> gint>,
            lua_class_propfunc_t,
        >(
            Some(
                luaH_download_set_uri
                    as unsafe extern "C" fn(*mut lua_State, *mut download_t) -> gint,
            ),
        ),
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut lua_State, *mut download_t) -> gint>,
            lua_class_propfunc_t,
        >(
            Some(
                luaH_download_get_uri
                    as unsafe extern "C" fn(*mut lua_State, *mut download_t) -> gint,
            ),
        ),
        ::core::mem::transmute::<
            *mut std::ffi::c_void,
            lua_class_propfunc_t,
        >(0 as *mut std::ffi::c_void),
    );
    luaH_uniq_setup(
        L,
        b"luakit.uniq.registry.download\0" as *const u8 as *const std::ffi::c_char,
        b"v\0" as *const u8 as *const std::ffi::c_char,
    );
}
