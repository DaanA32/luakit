use ::libc;
#[c2rust::header_src = "/usr/lib/clang/20/include/__stddef_size_t.h:23"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = std::ffi::c_ulong;
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lua.h:23"]
pub mod lua_h {
    #[c2rust::src_loc = "53:1"]
    pub type lua_CFunction = Option::<
        unsafe extern "C" fn(*mut lua_State) -> std::ffi::c_int,
    >;
    #[c2rust::src_loc = "30:9"]
    pub const LUA_MULTRET: std::ffi::c_int = -(1 as std::ffi::c_int);
    #[c2rust::src_loc = "38:9"]
    pub const LUA_GLOBALSINDEX: std::ffi::c_int = -(10002 as std::ffi::c_int);
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
        #[c2rust::src_loc = "164:1"]
        pub fn lua_pushlstring(L: *mut lua_State, s: *const std::ffi::c_char, l: size_t);
        #[c2rust::src_loc = "165:1"]
        pub fn lua_pushstring(L: *mut lua_State, s: *const std::ffi::c_char);
        #[c2rust::src_loc = "169:1"]
        pub fn lua_pushcclosure(
            L: *mut lua_State,
            fn_0: lua_CFunction,
            n: std::ffi::c_int,
        );
        #[c2rust::src_loc = "170:1"]
        pub fn lua_pushboolean(L: *mut lua_State, b: std::ffi::c_int);
        #[c2rust::src_loc = "171:1"]
        pub fn lua_pushlightuserdata(L: *mut lua_State, p: *mut std::ffi::c_void);
        #[c2rust::src_loc = "178:1"]
        pub fn lua_gettable(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "179:1"]
        pub fn lua_getfield(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            k: *const std::ffi::c_char,
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
        #[c2rust::src_loc = "193:1"]
        pub fn lua_rawset(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "195:1"]
        pub fn lua_setmetatable(
            L: *mut lua_State,
            objindex: std::ffi::c_int,
        ) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/log.h:25"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtypes.h:24"]
pub mod gtypes_h {
    #[c2rust::src_loc = "52:1"]
    pub type gchar = std::ffi::c_char;
    #[c2rust::src_loc = "56:1"]
    pub type gboolean = gint;
    #[c2rust::src_loc = "55:1"]
    pub type gint = std::ffi::c_int;
    #[c2rust::src_loc = "61:1"]
    pub type guint = std::ffi::c_uint;
    #[c2rust::src_loc = "64:1"]
    pub type gdouble = std::ffi::c_double;
    #[c2rust::src_loc = "109:1"]
    pub type gpointer = *mut std::ffi::c_void;
    #[c2rust::src_loc = "140:1"]
    pub type GDestroyNotify = Option::<unsafe extern "C" fn(gpointer) -> ()>;
    #[c2rust::src_loc = "114:1"]
    pub type GCompareDataFunc = Option::<
        unsafe extern "C" fn(gconstpointer, gconstpointer, gpointer) -> gint,
    >;
    #[c2rust::src_loc = "110:1"]
    pub type gconstpointer = *const std::ffi::c_void;
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lauxlib.h:25"]
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
        #[c2rust::src_loc = "28:1"]
        pub fn luaL_register(
            L: *mut lua_State,
            libname: *const std::ffi::c_char,
            l: *const luaL_Reg,
        );
        #[c2rust::src_loc = "34:1"]
        pub fn luaL_checklstring(
            L: *mut lua_State,
            numArg: std::ffi::c_int,
            l: *mut size_t,
        ) -> *const std::ffi::c_char;
        #[c2rust::src_loc = "49:1"]
        pub fn luaL_newmetatable(
            L: *mut lua_State,
            tname: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "53:1"]
        pub fn luaL_error(
            L: *mut lua_State,
            fmt: *const std::ffi::c_char,
            _: ...
        ) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/gio/giotypes.h:24"]
pub mod giotypes_h {
    #[c2rust::src_loc = "59:1"]
    pub type GApplication = _GApplication;
    #[c2rust::src_loc = "55:1"]
    pub type GActionGroup = _GActionGroup;
    #[c2rust::src_loc = "57:1"]
    pub type GSimpleAction = _GSimpleAction;
    #[c2rust::src_loc = "54:1"]
    pub type GActionMap = _GActionMap;
    #[c2rust::src_loc = "40:1"]
    pub type GCancellable = _GCancellable;
    use super::gapplication_h::_GApplication;
    use super::gcancellable_h::_GCancellable;
    extern "C" {
        #[c2rust::src_loc = "55:16"]
        pub type _GActionGroup;
        #[c2rust::src_loc = "57:16"]
        pub type _GSimpleAction;
        #[c2rust::src_loc = "54:16"]
        pub type _GActionMap;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/gio/gapplication.h:24"]
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
    use super::gtype_h::GType;
    use super::gtypes_h::{gchar, gboolean};
    use super::giotypes_h::{GApplication, GCancellable};
    use super::gerror_h::GError;
    extern "C" {
        #[c2rust::src_loc = "43:16"]
        pub type _GApplicationPrivate;
        #[c2rust::src_loc = "126:1"]
        pub fn g_application_get_type() -> GType;
        #[c2rust::src_loc = "129:1"]
        pub fn g_application_id_is_valid(application_id: *const gchar) -> gboolean;
        #[c2rust::src_loc = "136:1"]
        pub fn g_application_get_application_id(
            application: *mut GApplication,
        ) -> *const gchar;
        #[c2rust::src_loc = "199:1"]
        pub fn g_application_get_is_registered(
            application: *mut GApplication,
        ) -> gboolean;
        #[c2rust::src_loc = "201:1"]
        pub fn g_application_get_is_remote(application: *mut GApplication) -> gboolean;
        #[c2rust::src_loc = "204:1"]
        pub fn g_application_register(
            application: *mut GApplication,
            cancellable: *mut GCancellable,
            error: *mut *mut GError,
        ) -> gboolean;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gobject.h:24"]
pub mod gobject_h {
    #[c2rust::src_loc = "192:1"]
    pub type GObject = _GObject;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "252:9"]
    pub struct _GObject {
        pub g_type_instance: GTypeInstance,
        pub ref_count: guint,
        pub qdata: *mut GData,
    }
    #[c2rust::src_loc = "194:1"]
    pub type GInitiallyUnowned = _GObject;
    use super::gtype_h::GTypeInstance;
    use super::gtypes_h::{guint, gpointer};
    use super::gdataset_h::GData;
    extern "C" {
        #[c2rust::src_loc = "514:1"]
        pub fn g_object_unref(object: gpointer);
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gdataset.h:24"]
pub mod gdataset_h {
    #[c2rust::src_loc = "38:1"]
    pub type GData = _GData;
    extern "C" {
        #[c2rust::src_loc = "38:16"]
        pub type _GData;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gtype.h:24"]
pub mod gtype_h {
    #[c2rust::src_loc = "436:1"]
    pub type GTypeInstance = _GTypeInstance;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "461:8"]
    pub struct _GTypeInstance {
        pub g_class: *mut GTypeClass,
    }
    #[c2rust::src_loc = "434:1"]
    pub type GTypeClass = _GTypeClass;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "451:8"]
    pub struct _GTypeClass {
        pub g_type: GType,
    }
    #[c2rust::src_loc = "427:1"]
    pub type GType = gsize;
    use super::glibconfig_h::gsize;
    extern "C" {
        #[c2rust::src_loc = "2626:1"]
        pub fn g_type_check_instance_cast(
            instance: *mut GTypeInstance,
            iface_type: GType,
        ) -> *mut GTypeInstance;
    }
}
#[c2rust::header_src = "/usr/lib/glib-2.0/include/glibconfig.h:24"]
pub mod glibconfig_h {
    #[c2rust::src_loc = "83:1"]
    pub type gsize = std::ffi::c_ulong;
    #[c2rust::src_loc = "57:1"]
    pub type guint32 = std::ffi::c_uint;
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gtk/gtkapplication.h:24"]
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
    use super::gtypes_h::gchar;
    use super::gioenums_h::{GApplicationFlags, G_APPLICATION_FLAGS_NONE};
    use super::gtktypes_h::GtkWindow;
    extern "C" {
        #[c2rust::src_loc = "41:16"]
        pub type _GtkApplicationPrivate;
        #[c2rust::src_loc = "78:1"]
        pub fn gtk_application_new(
            application_id: *const gchar,
            flags: GApplicationFlags,
        ) -> *mut GtkApplication;
        #[c2rust::src_loc = "139:1"]
        pub fn gtk_application_get_active_window(
            application: *mut GtkApplication,
        ) -> *mut GtkWindow;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/globalconf.h:24"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/glib/garray.h:24"]
pub mod garray_h {
    #[c2rust::src_loc = "41:1"]
    pub type GPtrArray = _GPtrArray;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "55:8"]
    pub struct _GPtrArray {
        pub pdata: *mut gpointer,
        pub len: guint,
    }
    use super::gtypes_h::{gpointer, guint, gboolean};
    extern "C" {
        #[c2rust::src_loc = "188:1"]
        pub fn g_ptr_array_free(
            array: *mut GPtrArray,
            free_segment: gboolean,
        ) -> *mut gpointer;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gvariant.h:24"]
pub mod gvariant_h {
    #[c2rust::src_loc = "36:1"]
    pub type GVariant = _GVariant;
    use super::gvarianttype_h::GVariantType;
    use super::gtypes_h::{gboolean, gchar};
    use super::glibconfig_h::gsize;
    extern "C" {
        #[c2rust::src_loc = "36:16"]
        pub type _GVariant;
        #[c2rust::src_loc = "75:1"]
        pub fn g_variant_is_of_type(
            value: *mut GVariant,
            type_0: *const GVariantType,
        ) -> gboolean;
        #[c2rust::src_loc = "102:1"]
        pub fn g_variant_new_string(string: *const gchar) -> *mut GVariant;
        #[c2rust::src_loc = "157:1"]
        pub fn g_variant_get_string(
            value: *mut GVariant,
            length: *mut gsize,
        ) -> *const gchar;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/gio/gactionmap.h:24"]
pub mod gactionmap_h {
    #[c2rust::src_loc = "43:1"]
    pub type GActionEntry = _GActionEntry;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "57:8"]
    pub struct _GActionEntry {
        pub name: *const gchar,
        pub activate: Option::<
            unsafe extern "C" fn(*mut GSimpleAction, *mut GVariant, gpointer) -> (),
        >,
        pub parameter_type: *const gchar,
        pub state: *const gchar,
        pub change_state: Option::<
            unsafe extern "C" fn(*mut GSimpleAction, *mut GVariant, gpointer) -> (),
        >,
        pub padding: [gsize; 3],
    }
    use super::gtypes_h::{gchar, gpointer, gint};
    use super::giotypes_h::{GSimpleAction, GActionMap};
    use super::gvariant_h::GVariant;
    use super::glibconfig_h::gsize;
    use super::gtype_h::GType;
    extern "C" {
        #[c2rust::src_loc = "77:1"]
        pub fn g_action_map_get_type() -> GType;
        #[c2rust::src_loc = "89:1"]
        pub fn g_action_map_add_action_entries(
            action_map: *mut GActionMap,
            entries: *const GActionEntry,
            n_entries: gint,
            user_data: gpointer,
        );
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/signal.h:25"]
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
        g_ptr_array_free(sigfuncs as *mut GPtrArray, TRUE);
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
            NULL as *mut std::ffi::c_void,
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
    use super::gmacros_h::{FALSE, TRUE};
    use super::__stddef_null_h::NULL;
    use super::gmem_h::g_free;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtree.h:24"]
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
#[c2rust::header_src = "/home/daana/git/luakit/common/luaclass.h:25"]
pub mod luaclass_h {
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
    #[c2rust::src_loc = "45:1"]
    pub type lua_class_propfunc_t = Option::<
        unsafe extern "C" fn(*mut lua_State, *mut lua_object_t) -> gint,
    >;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "39:9"]
    pub struct lua_object_t {
        pub signals: *mut signal_t,
    }
    #[c2rust::src_loc = "32:1"]
    pub type lua_class_property_array_t = GHashTable;
    #[c2rust::src_loc = "43:1"]
    pub type lua_class_allocator_t = Option::<
        unsafe extern "C" fn(*mut lua_State) -> *mut lua_object_t,
    >;
    use super::gtypes_h::{gchar, gint};
    use super::signal_h::signal_t;
    use super::lua_h::lua_State;
    use super::ghash_h::GHashTable;
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
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/ghash.h:24"]
pub mod ghash_h {
    #[c2rust::src_loc = "40:1"]
    pub type GHashTable = _GHashTable;
    extern "C" {
        #[c2rust::src_loc = "40:16"]
        pub type _GHashTable;
    }
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gdk/gdktypes.h:24"]
pub mod gdktypes_h {
    #[c2rust::src_loc = "142:1"]
    pub type GdkScreen = _GdkScreen;
    extern "C" {
        #[c2rust::src_loc = "142:16"]
        pub type _GdkScreen;
    }
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gtk/gtktypes.h:24"]
pub mod gtktypes_h {
    #[c2rust::src_loc = "48:1"]
    pub type GtkWindow = _GtkWindow;
    #[c2rust::src_loc = "46:1"]
    pub type GtkWidget = _GtkWidget;
    use super::gtkwindow_h::_GtkWindow;
    use super::gtkwidget_h::_GtkWidget;
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gtk/gtkwindow.h:24"]
pub mod gtkwindow_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "53:8"]
    pub struct _GtkWindow {
        pub bin: GtkBin,
        pub priv_0: *mut GtkWindowPrivate,
    }
    #[c2rust::src_loc = "46:1"]
    pub type GtkWindowPrivate = _GtkWindowPrivate;
    use super::gtkbin_h::GtkBin;
    use super::gtktypes_h::GtkWindow;
    use super::gdktypes_h::GdkScreen;
    extern "C" {
        #[c2rust::src_loc = "46:16"]
        pub type _GtkWindowPrivate;
        #[c2rust::src_loc = "277:1"]
        pub fn gtk_window_get_screen(window: *mut GtkWindow) -> *mut GdkScreen;
    }
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gtk/gtkbin.h:24"]
pub mod gtkbin_h {
    #[c2rust::src_loc = "45:1"]
    pub type GtkBin = _GtkBin;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:8"]
    pub struct _GtkBin {
        pub container: GtkContainer,
        pub priv_0: *mut GtkBinPrivate,
    }
    #[c2rust::src_loc = "46:1"]
    pub type GtkBinPrivate = _GtkBinPrivate;
    use super::gtkcontainer_h::GtkContainer;
    extern "C" {
        #[c2rust::src_loc = "46:16"]
        pub type _GtkBinPrivate;
    }
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gtk/gtkcontainer.h:24"]
pub mod gtkcontainer_h {
    #[c2rust::src_loc = "45:1"]
    pub type GtkContainer = _GtkContainer;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:8"]
    pub struct _GtkContainer {
        pub widget: GtkWidget,
        pub priv_0: *mut GtkContainerPrivate,
    }
    #[c2rust::src_loc = "46:1"]
    pub type GtkContainerPrivate = _GtkContainerPrivate;
    use super::gtktypes_h::GtkWidget;
    extern "C" {
        #[c2rust::src_loc = "46:16"]
        pub type _GtkContainerPrivate;
    }
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gtk/gtkwidget.h:24"]
pub mod gtkwidget_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "133:8"]
    pub struct _GtkWidget {
        pub parent_instance: GInitiallyUnowned,
        pub priv_0: *mut GtkWidgetPrivate,
    }
    #[c2rust::src_loc = "66:1"]
    pub type GtkWidgetPrivate = _GtkWidgetPrivate;
    use super::gobject_h::GInitiallyUnowned;
    extern "C" {
        #[c2rust::src_loc = "66:16"]
        pub type _GtkWidgetPrivate;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gvarianttype.h:24"]
pub mod gvarianttype_h {
    #[c2rust::src_loc = "34:1"]
    pub type GVariantType = _GVariantType;
    #[c2rust::src_loc = "122:9"]
    pub const G_VARIANT_TYPE_STRING: [std::ffi::c_char; 2] = unsafe {
        *::core::mem::transmute::<&[u8; 2], &[std::ffi::c_char; 2]>(b"s\0")
    };
    extern "C" {
        #[c2rust::src_loc = "34:16"]
        pub type _GVariantType;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gerror.h:24"]
pub mod gerror_h {
    #[c2rust::src_loc = "43:1"]
    pub type GError = _GError;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "45:8"]
    pub struct _GError {
        pub domain: GQuark,
        pub code: gint,
        pub message: *mut gchar,
    }
    use super::gquark_h::GQuark;
    use super::gtypes_h::{gint, gchar};
    extern "C" {
        #[c2rust::src_loc = "207:1"]
        pub fn g_error_free(error: *mut GError);
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gquark.h:24"]
pub mod gquark_h {
    #[c2rust::src_loc = "38:1"]
    pub type GQuark = guint32;
    use super::glibconfig_h::guint32;
}
#[c2rust::header_src = "/usr/include/glib-2.0/gio/gcancellable.h:24"]
pub mod gcancellable_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "44:8"]
    pub struct _GCancellable {
        pub parent_instance: GObject,
        pub priv_0: *mut GCancellablePrivate,
    }
    #[c2rust::src_loc = "42:1"]
    pub type GCancellablePrivate = _GCancellablePrivate;
    use super::gobject_h::GObject;
    extern "C" {
        #[c2rust::src_loc = "42:16"]
        pub type _GCancellablePrivate;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/gio/gioenums.h:24"]
pub mod gioenums_h {
    #[c2rust::src_loc = "1554:9"]
    pub type GApplicationFlags = std::ffi::c_uint;
    #[c2rust::src_loc = "1569:3"]
    pub const G_APPLICATION_REPLACE: GApplicationFlags = 256;
    #[c2rust::src_loc = "1568:3"]
    pub const G_APPLICATION_ALLOW_REPLACEMENT: GApplicationFlags = 128;
    #[c2rust::src_loc = "1567:3"]
    pub const G_APPLICATION_CAN_OVERRIDE_APP_ID: GApplicationFlags = 64;
    #[c2rust::src_loc = "1565:3"]
    pub const G_APPLICATION_NON_UNIQUE: GApplicationFlags = 32;
    #[c2rust::src_loc = "1563:3"]
    pub const G_APPLICATION_SEND_ENVIRONMENT: GApplicationFlags = 16;
    #[c2rust::src_loc = "1562:3"]
    pub const G_APPLICATION_HANDLES_COMMAND_LINE: GApplicationFlags = 8;
    #[c2rust::src_loc = "1561:3"]
    pub const G_APPLICATION_HANDLES_OPEN: GApplicationFlags = 4;
    #[c2rust::src_loc = "1559:3"]
    pub const G_APPLICATION_IS_LAUNCHER: GApplicationFlags = 2;
    #[c2rust::src_loc = "1558:3"]
    pub const G_APPLICATION_IS_SERVICE: GApplicationFlags = 1;
    #[c2rust::src_loc = "1557:3"]
    pub const G_APPLICATION_DEFAULT_FLAGS: GApplicationFlags = 0;
    #[c2rust::src_loc = "1556:3"]
    pub const G_APPLICATION_FLAGS_NONE: GApplicationFlags = 0;
}
#[c2rust::header_src = "/usr/include/string.h:24"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "156:12"]
        pub fn strcmp(
            _: *const std::ffi::c_char,
            _: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gmem.h:24"]
pub mod gmem_h {
    use super::gtypes_h::gpointer;
    extern "C" {
        #[c2rust::src_loc = "73:1"]
        pub fn g_free(mem: gpointer);
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtestutils.h:24"]
pub mod gtestutils_h {
    extern "C" {
        #[c2rust::src_loc = "282:1"]
        pub fn g_strcmp0(
            str1: *const std::ffi::c_char,
            str2: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/gio/gactiongroup.h:24"]
pub mod gactiongroup_h {
    use super::gtype_h::GType;
    use super::giotypes_h::GActionGroup;
    use super::gtypes_h::gchar;
    use super::gvariant_h::GVariant;
    extern "C" {
        #[c2rust::src_loc = "99:1"]
        pub fn g_action_group_get_type() -> GType;
        #[c2rust::src_loc = "130:1"]
        pub fn g_action_group_activate_action(
            action_group: *mut GActionGroup,
            action_name: *const gchar,
            parameter: *mut GVariant,
        );
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luaobject.h:25"]
pub mod luaobject_h {
    use super::lua_h::lua_State;
    use super::signal_h::signal_t;
    use super::gtypes_h::{gchar, gint};
    extern "C" {
        #[c2rust::src_loc = "158:1"]
        pub fn signal_object_emit(
            _: *mut lua_State,
            signals: *mut signal_t,
            name: *const gchar,
            nargs: gint,
            nret: gint,
        ) -> gint;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gmacros.h:24"]
pub mod gmacros_h {
    #[c2rust::src_loc = "931:9"]
    pub const FALSE: std::ffi::c_int = 0 as std::ffi::c_int;
    #[c2rust::src_loc = "935:9"]
    pub const TRUE: std::ffi::c_int = (FALSE == 0) as std::ffi::c_int;
}
#[c2rust::header_src = "/usr/lib/clang/20/include/__stddef_null.h:24"]
pub mod __stddef_null_h {
    #[c2rust::src_loc = "26:9"]
    pub const NULL: std::ffi::c_int = 0 as std::ffi::c_int;
}
pub use self::__stddef_size_t_h::size_t;
pub use self::lua_h::{
    lua_CFunction, LUA_MULTRET, LUA_GLOBALSINDEX, lua_State, lua_gettop, lua_settop,
    lua_pushvalue, lua_pushlstring, lua_pushstring, lua_pushcclosure, lua_pushboolean,
    lua_pushlightuserdata, lua_gettable, lua_getfield, lua_createtable, lua_setfield,
    lua_rawset, lua_setmetatable,
};
pub use self::log_h::{
    log_level_t, LOG_LEVEL_debug, LOG_LEVEL_verbose, LOG_LEVEL_info, LOG_LEVEL_warn,
    LOG_LEVEL_error, LOG_LEVEL_fatal, _log,
};
pub use self::gtypes_h::{
    gchar, gboolean, gint, guint, gdouble, gpointer, GDestroyNotify, GCompareDataFunc,
    gconstpointer,
};
pub use self::lauxlib_h::{
    luaL_Reg, luaL_register, luaL_checklstring, luaL_newmetatable, luaL_error,
};
pub use self::giotypes_h::{
    GApplication, GActionGroup, GSimpleAction, GActionMap, GCancellable, _GActionGroup,
    _GSimpleAction, _GActionMap,
};
pub use self::gapplication_h::{
    _GApplication, GApplicationPrivate, _GApplicationPrivate, g_application_get_type,
    g_application_id_is_valid, g_application_get_application_id,
    g_application_get_is_registered, g_application_get_is_remote, g_application_register,
};
pub use self::gobject_h::{GObject, _GObject, GInitiallyUnowned, g_object_unref};
pub use self::gdataset_h::{GData, _GData};
pub use self::gtype_h::{
    GTypeInstance, _GTypeInstance, GTypeClass, _GTypeClass, GType,
    g_type_check_instance_cast,
};
pub use self::glibconfig_h::{gsize, guint32};
pub use self::gtkapplication_h::{
    GtkApplication, _GtkApplication, GtkApplicationPrivate, _GtkApplicationPrivate,
    gtk_application_new, gtk_application_get_active_window,
};
pub use self::globalconf_h::{globalconf_t, globalconf};
pub use self::garray_h::{GPtrArray, _GPtrArray, g_ptr_array_free};
pub use self::gvariant_h::{
    GVariant, _GVariant, g_variant_is_of_type, g_variant_new_string, g_variant_get_string,
};
pub use self::gactionmap_h::{
    GActionEntry, _GActionEntry, g_action_map_get_type, g_action_map_add_action_entries,
};
pub use self::signal_h::{signal_t, signal_cmp, signal_array_destroy, signal_new};
pub use self::gtree_h::{GTree, _GTree, g_tree_new_full};
pub use self::luaclass_h::{
    lua_class_t, lua_class_propfunc_t, lua_object_t, lua_class_property_array_t,
    lua_class_allocator_t, luaH_class_add_signal, luaH_class_remove_signal,
    luaH_class_emit_signal,
};
pub use self::ghash_h::{GHashTable, _GHashTable};
pub use self::gdktypes_h::{GdkScreen, _GdkScreen};
pub use self::gtktypes_h::{GtkWindow, GtkWidget};
pub use self::gtkwindow_h::{
    _GtkWindow, GtkWindowPrivate, _GtkWindowPrivate, gtk_window_get_screen,
};
pub use self::gtkbin_h::{GtkBin, _GtkBin, GtkBinPrivate, _GtkBinPrivate};
pub use self::gtkcontainer_h::{
    GtkContainer, _GtkContainer, GtkContainerPrivate, _GtkContainerPrivate,
};
pub use self::gtkwidget_h::{_GtkWidget, GtkWidgetPrivate, _GtkWidgetPrivate};
pub use self::gvarianttype_h::{GVariantType, G_VARIANT_TYPE_STRING, _GVariantType};
pub use self::gerror_h::{GError, _GError, g_error_free};
pub use self::gquark_h::GQuark;
pub use self::gcancellable_h::{_GCancellable, GCancellablePrivate, _GCancellablePrivate};
pub use self::gioenums_h::{
    GApplicationFlags, G_APPLICATION_REPLACE, G_APPLICATION_ALLOW_REPLACEMENT,
    G_APPLICATION_CAN_OVERRIDE_APP_ID, G_APPLICATION_NON_UNIQUE,
    G_APPLICATION_SEND_ENVIRONMENT, G_APPLICATION_HANDLES_COMMAND_LINE,
    G_APPLICATION_HANDLES_OPEN, G_APPLICATION_IS_LAUNCHER, G_APPLICATION_IS_SERVICE,
    G_APPLICATION_DEFAULT_FLAGS, G_APPLICATION_FLAGS_NONE,
};
use self::string_h::strcmp;
use self::gmem_h::g_free;
use self::gtestutils_h::g_strcmp0;
use self::gactiongroup_h::{g_action_group_get_type, g_action_group_activate_action};
use self::luaobject_h::signal_object_emit;
pub use self::gmacros_h::{FALSE, TRUE};
pub use self::__stddef_null_h::NULL;
#[c2rust::src_loc = "31:20"]
static mut unique_class: lua_class_t = lua_class_t {
    name: 0 as *const gchar,
    signals: 0 as *const signal_t as *mut signal_t,
    allocator: None,
    properties: 0 as *const lua_class_property_array_t
        as *mut lua_class_property_array_t,
    index_miss_property: None,
    newindex_miss_property: None,
};
#[inline]
#[c2rust::src_loc = "32:1"]
unsafe extern "C" fn luaH_unique_class_emit_signal(mut L: *mut lua_State) -> gint {
    return luaH_class_emit_signal(
        L,
        &mut unique_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, NULL as *mut size_t),
        lua_gettop(L) - 1 as std::ffi::c_int,
        LUA_MULTRET,
    );
}
#[inline]
#[c2rust::src_loc = "32:1"]
unsafe extern "C" fn luaH_unique_class_remove_signal(mut L: *mut lua_State) -> gint {
    luaH_class_remove_signal(
        L,
        &mut unique_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, NULL as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]
#[c2rust::src_loc = "32:1"]
unsafe extern "C" fn luaH_unique_class_add_signal(mut L: *mut lua_State) -> gint {
    luaH_class_add_signal(
        L,
        &mut unique_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, NULL as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[c2rust::src_loc = "34:1"]
unsafe extern "C" fn message_cb(
    mut UNUSED_a: *mut GSimpleAction,
    mut message_data: *mut GVariant,
    mut L: *mut lua_State,
) {
    if !message_data.is_null()
        && g_variant_is_of_type(message_data, G_VARIANT_TYPE_STRING.as_ptr()) != 0
    {
        let mut text = g_variant_get_string(message_data, NULL as *mut gsize);
        lua_pushstring(L, text);
        let mut window = gtk_application_get_active_window(globalconf.application);
        if window.is_null() {
            _log(
                LOG_LEVEL_warn,
                b"clib/unique.c\0" as *const u8 as *const std::ffi::c_char,
                b"It's not a window!!!\0" as *const u8 as *const std::ffi::c_char,
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
#[c2rust::src_loc = "52:1"]
unsafe extern "C" fn unique_is_registered() -> gboolean {
    if (globalconf.application).is_null() {
        return FALSE;
    }
    if g_application_get_is_registered(
        g_type_check_instance_cast(
            globalconf.application as *mut GTypeInstance,
            g_application_get_type(),
        ) as *mut std::ffi::c_void as *mut GApplication,
    ) == 0
    {
        return FALSE;
    }
    return TRUE;
}
#[c2rust::src_loc = "62:1"]
unsafe extern "C" fn luaH_unique_new(mut L: *mut lua_State) -> gint {
    let mut name = luaL_checklstring(L, 1 as std::ffi::c_int, NULL as *mut size_t);
    if g_application_id_is_valid(name) == 0 {
        return luaL_error(
            L,
            b"invalid application name\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if unique_is_registered() != 0 {
        let mut other = g_application_get_application_id(
            g_type_check_instance_cast(
                globalconf.application as *mut GTypeInstance,
                g_application_get_type(),
            ) as *mut std::ffi::c_void as *mut GApplication,
        );
        if !(strcmp(name as *const std::ffi::c_char, other as *const std::ffi::c_char)
            == 0 as std::ffi::c_int)
        {
            luaL_error(
                L,
                b"GApplication '%s' already setup\0" as *const u8
                    as *const std::ffi::c_char,
                other,
            );
        } else {
            _log(
                LOG_LEVEL_verbose,
                b"clib/unique.c\0" as *const u8 as *const std::ffi::c_char,
                b"GApplication '%s' already setup\0" as *const u8
                    as *const std::ffi::c_char,
                name,
            );
        }
        return 0 as std::ffi::c_int;
    }
    let mut error = NULL as *mut GError;
    if (globalconf.application).is_null() {
        globalconf.application = gtk_application_new(name, G_APPLICATION_DEFAULT_FLAGS);
    }
    g_application_register(
        g_type_check_instance_cast(
            globalconf.application as *mut GTypeInstance,
            g_application_get_type(),
        ) as *mut std::ffi::c_void as *mut GApplication,
        NULL as *mut GCancellable,
        &mut error,
    );
    if !error.is_null() {
        luaL_error(
            L,
            b"unable to register GApplication: %s\0" as *const u8
                as *const std::ffi::c_char,
            (*error).message,
        );
        g_error_free(error);
        g_object_unref(
            g_type_check_instance_cast(
                globalconf.application as *mut GTypeInstance,
                ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
            ) as *mut std::ffi::c_void as *mut GObject as gpointer,
        );
        globalconf.application = NULL as *mut GtkApplication;
        return 0 as std::ffi::c_int;
    }
    let entries: [GActionEntry; 1] = [
        {
            let mut init = _GActionEntry {
                name: b"message\0" as *const u8 as *const std::ffi::c_char,
                activate: ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut GSimpleAction,
                            *mut GVariant,
                            *mut lua_State,
                        ) -> (),
                    >,
                    Option::<
                        unsafe extern "C" fn(
                            *mut GSimpleAction,
                            *mut GVariant,
                            gpointer,
                        ) -> (),
                    >,
                >(
                    Some(
                        message_cb
                            as unsafe extern "C" fn(
                                *mut GSimpleAction,
                                *mut GVariant,
                                *mut lua_State,
                            ) -> (),
                    ),
                ),
                parameter_type: b"s\0" as *const u8 as *const std::ffi::c_char,
                state: 0 as *const gchar,
                change_state: None,
                padding: [0; 3],
            };
            init
        },
    ];
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
#[c2rust::src_loc = "107:1"]
unsafe extern "C" fn luaH_unique_is_running(mut L: *mut lua_State) -> gint {
    if unique_is_registered() == 0 {
        luaL_error(
            L,
            b"GApplication is not registered\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut running = g_application_get_is_remote(
        g_type_check_instance_cast(
            globalconf.application as *mut GTypeInstance,
            g_application_get_type(),
        ) as *mut std::ffi::c_void as *mut GApplication,
    );
    lua_pushboolean(L, running);
    return 1 as std::ffi::c_int;
}
#[c2rust::src_loc = "119:1"]
unsafe extern "C" fn luaH_unique_send_message(mut L: *mut lua_State) -> gint {
    if unique_is_registered() == 0 {
        luaL_error(
            L,
            b"GApplication is not registered\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if g_application_get_is_remote(
        g_type_check_instance_cast(
            globalconf.application as *mut GTypeInstance,
            g_application_get_type(),
        ) as *mut std::ffi::c_void as *mut GApplication,
    ) == 0
    {
        luaL_error(
            L,
            b"no other instances running\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut text = g_variant_new_string(
        luaL_checklstring(L, 1 as std::ffi::c_int, NULL as *mut size_t),
    );
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
#[c2rust::src_loc = "134:1"]
unsafe extern "C" fn luaH_open_luakit_unique(
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
    luaL_register(L, NULL as *const std::ffi::c_char, meta);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    luaL_register(L, NULL as *const std::ffi::c_char, methods);
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
        (::core::mem::size_of::<[std::ffi::c_char; 7]>() as std::ffi::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
            )
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    lua_pushvalue(L, -(3 as std::ffi::c_int));
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    lua_pushvalue(L, -(1 as std::ffi::c_int));
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    lua_settop(L, -(2 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
#[c2rust::src_loc = "164:17"]
static mut warned: gboolean = FALSE;
#[c2rust::src_loc = "166:1"]
unsafe extern "C" fn luaH_unique_proxy_index(mut L: *mut lua_State) -> std::ffi::c_int {
    if warned == 0 {
        warned = TRUE;
        _log(
            LOG_LEVEL_warn,
            b"clib/unique.c\0" as *const u8 as *const std::ffi::c_char,
            b"the unique library has been moved to luakit.unique\0" as *const u8
                as *const std::ffi::c_char,
        );
        _log(
            LOG_LEVEL_warn,
            b"clib/unique.c\0" as *const u8 as *const std::ffi::c_char,
            b"this compatibility wrapper will be removed in a future version\0"
                as *const u8 as *const std::ffi::c_char,
        );
        _log(
            LOG_LEVEL_warn,
            b"clib/unique.c\0" as *const u8 as *const std::ffi::c_char,
            b"you should remove the two `if unique then ... end` blocks from your rc.lua\0"
                as *const u8 as *const std::ffi::c_char,
        );
        _log(
            LOG_LEVEL_warn,
            b"clib/unique.c\0" as *const u8 as *const std::ffi::c_char,
            b"then, at the start of your rc.lua, add `require \"unique_instance\"`\0"
                as *const u8 as *const std::ffi::c_char,
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
#[c2rust::src_loc = "184:1"]
unsafe extern "C" fn luaH_open_unique_proxy(mut L: *mut lua_State) {
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_pushcclosure(
        L,
        Some(
            luaH_unique_proxy_index
                as unsafe extern "C" fn(*mut lua_State) -> std::ffi::c_int,
        ),
        0 as std::ffi::c_int,
    );
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
#[no_mangle]
#[c2rust::src_loc = "203:1"]
pub unsafe extern "C" fn unique_lib_setup(mut L: *mut lua_State) {
    static mut unique_lib: [luaL_Reg; 7] = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"add_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_unique_class_add_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_unique_class_remove_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"emit_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_unique_class_emit_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"new\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_unique_new as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"send_message\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_unique_send_message
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"is_running\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_unique_is_running
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: NULL as *const std::ffi::c_char,
                    func: ::core::mem::transmute::<
                        libc::intptr_t,
                        lua_CFunction,
                    >(NULL as libc::intptr_t),
                };
                init
            },
        ]
    };
    unique_class.signals = signal_new();
    luaH_open_luakit_unique(L, unique_lib.as_ptr(), unique_lib.as_ptr());
    luaH_open_unique_proxy(L);
}
