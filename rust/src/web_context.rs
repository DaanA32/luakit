use ::libc;
use ::c2rust_bitfields;
#[c2rust::header_src = "/usr/lib/glib-2.0/include/glibconfig.h:21"]
pub mod glibconfig_h {
    #[c2rust::src_loc = "66:1"]
    pub type gint64 = std::ffi::c_long;
    #[c2rust::src_loc = "67:1"]
    pub type guint64 = std::ffi::c_ulong;
    #[c2rust::src_loc = "83:1"]
    pub type gsize = std::ffi::c_ulong;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtypes.h:21"]
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
    use super::gtypes_h::{gpointer, guint};
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
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gtype.h:21"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gvalue.h:21"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gclosure.h:21"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gsignal.h:21"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gobject.h:21"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/gio/gapplication.h:21"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/gio/giotypes.h:21"]
pub mod giotypes_h {
    #[c2rust::src_loc = "59:1"]
    pub type GApplication = _GApplication;
    use super::gapplication_h::_GApplication;
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gtk/gtkapplication.h:21"]
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
#[c2rust::header_src = "/home/daana/git/luakit/globalconf.h:21"]
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
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkit/WebKitCookieManager.h:23"]
pub mod WebKitCookieManager_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "52:1"]
    pub struct _WebKitCookieManager {
        pub parent: GObject,
        pub priv_0: *mut WebKitCookieManagerPrivate,
    }
    #[c2rust::src_loc = "52:1"]
    pub type WebKitCookieManagerPrivate = _WebKitCookieManagerPrivate;
    #[c2rust::src_loc = "52:1"]
    pub type WebKitCookieManager = _WebKitCookieManager;
    #[c2rust::src_loc = "76:9"]
    pub type WebKitCookieAcceptPolicy = std::ffi::c_uint;
    #[c2rust::src_loc = "79:5"]
    pub const WEBKIT_COOKIE_POLICY_ACCEPT_NO_THIRD_PARTY: WebKitCookieAcceptPolicy = 2;
    #[c2rust::src_loc = "78:5"]
    pub const WEBKIT_COOKIE_POLICY_ACCEPT_NEVER: WebKitCookieAcceptPolicy = 1;
    #[c2rust::src_loc = "77:5"]
    pub const WEBKIT_COOKIE_POLICY_ACCEPT_ALWAYS: WebKitCookieAcceptPolicy = 0;
    use super::gobject_h::GObject;
    extern "C" {
        #[c2rust::src_loc = "52:1"]
        pub type _WebKitCookieManagerPrivate;
        #[c2rust::src_loc = "87:1"]
        pub fn webkit_cookie_manager_set_accept_policy(
            cookie_manager: *mut WebKitCookieManager,
            policy: WebKitCookieAcceptPolicy,
        );
    }
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkit/WebKitDownload.h:23"]
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
    extern "C" {
        #[c2rust::src_loc = "41:1"]
        pub type _WebKitDownloadPrivate;
    }
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkit/WebKitWebsiteDataManager.h:23"]
pub mod WebKitWebsiteDataManager_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "53:1"]
    pub struct _WebKitWebsiteDataManager {
        pub parent: GObject,
        pub priv_0: *mut WebKitWebsiteDataManagerPrivate,
    }
    #[c2rust::src_loc = "53:1"]
    pub type WebKitWebsiteDataManagerPrivate = _WebKitWebsiteDataManagerPrivate;
    #[c2rust::src_loc = "53:1"]
    pub type WebKitWebsiteDataManager = _WebKitWebsiteDataManager;
    use super::gobject_h::GObject;
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "53:1"]
        pub type _WebKitWebsiteDataManagerPrivate;
        #[c2rust::src_loc = "70:1"]
        pub fn webkit_website_data_manager_new(
            first_option_name: *const gchar,
            _: ...
        ) -> *mut WebKitWebsiteDataManager;
        #[c2rust::src_loc = "79:1"]
        pub fn webkit_website_data_manager_get_base_data_directory(
            manager: *mut WebKitWebsiteDataManager,
        ) -> *const gchar;
        #[c2rust::src_loc = "82:1"]
        pub fn webkit_website_data_manager_get_base_cache_directory(
            manager: *mut WebKitWebsiteDataManager,
        ) -> *const gchar;
    }
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/webkit/WebKitWebContext.h:23"]
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
    use super::WebKitWebsiteDataManager_h::WebKitWebsiteDataManager;
    use super::WebKitCookieManager_h::WebKitCookieManager;
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "49:1"]
        pub type _WebKitWebContextPrivate;
        #[c2rust::src_loc = "134:1"]
        pub fn webkit_web_context_new_with_website_data_manager(
            manager: *mut WebKitWebsiteDataManager,
        ) -> *mut WebKitWebContext;
        #[c2rust::src_loc = "176:1"]
        pub fn webkit_web_context_get_cookie_manager(
            context: *mut WebKitWebContext,
        ) -> *mut WebKitCookieManager;
        #[c2rust::src_loc = "185:1"]
        pub fn webkit_web_context_set_favicon_database_directory(
            context: *mut WebKitWebContext,
            path: *const gchar,
        );
        #[c2rust::src_loc = "234:1"]
        pub fn webkit_web_context_get_spell_checking_languages(
            context: *mut WebKitWebContext,
        ) -> *const *const gchar;
        #[c2rust::src_loc = "237:1"]
        pub fn webkit_web_context_set_spell_checking_languages(
            context: *mut WebKitWebContext,
            languages: *const *const gchar,
        );
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gmem.h:21"]
pub mod gmem_h {
    use super::gtypes_h::gpointer;
    extern "C" {
        #[c2rust::src_loc = "73:1"]
        pub fn g_free(mem: gpointer);
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gstrfuncs.h:21"]
pub mod gstrfuncs_h {
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "363:1"]
        pub fn g_strjoinv(
            separator: *const gchar,
            str_array: *mut *mut gchar,
        ) -> *mut gchar;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtestutils.h:21"]
pub mod gtestutils_h {
    extern "C" {
        #[c2rust::src_loc = "624:1"]
        pub fn g_assertion_message_expr(
            domain: *const std::ffi::c_char,
            file: *const std::ffi::c_char,
            line: std::ffi::c_int,
            func: *const std::ffi::c_char,
            expr: *const std::ffi::c_char,
        ) -> !;
    }
}
pub use self::glibconfig_h::{gint64, guint64, gsize};
pub use self::gtypes_h::{
    gchar, glong, gint, gboolean, gulong, guint, gfloat, gdouble, gpointer,
};
pub use self::garray_h::{_GPtrArray, GPtrArray};
pub use self::gdataset_h::{GData, _GData};
pub use self::gtype_h::{
    GType, GValue, _GTypeClass, GTypeClass, _GTypeInstance, GTypeInstance,
    g_type_check_instance_cast,
};
pub use self::gvalue_h::{_GValue, C2RustUnnamed};
pub use self::gclosure_h::{
    _GClosure, GClosureNotifyData, _GClosureNotifyData, GClosureNotify, GClosure,
    GCallback,
};
pub use self::gsignal_h::{
    GConnectFlags, G_CONNECT_SWAPPED, G_CONNECT_AFTER, G_CONNECT_DEFAULT,
    g_signal_connect_data,
};
pub use self::gobject_h::{_GObject, GObject};
pub use self::gapplication_h::{_GApplication, GApplicationPrivate, _GApplicationPrivate};
pub use self::giotypes_h::GApplication;
pub use self::gtkapplication_h::{
    _GtkApplication, GtkApplicationPrivate, GtkApplication, _GtkApplicationPrivate,
};
pub use self::globalconf_h::{globalconf_t, globalconf};
pub use self::log_h::{
    log_level_t, LOG_LEVEL_debug, LOG_LEVEL_verbose, LOG_LEVEL_info, LOG_LEVEL_warn,
    LOG_LEVEL_error, LOG_LEVEL_fatal, _log,
};
pub use self::WebKitCookieManager_h::{
    _WebKitCookieManager, WebKitCookieManagerPrivate, WebKitCookieManager,
    WebKitCookieAcceptPolicy, WEBKIT_COOKIE_POLICY_ACCEPT_NO_THIRD_PARTY,
    WEBKIT_COOKIE_POLICY_ACCEPT_NEVER, WEBKIT_COOKIE_POLICY_ACCEPT_ALWAYS,
    _WebKitCookieManagerPrivate, webkit_cookie_manager_set_accept_policy,
};
pub use self::WebKitDownload_h::{
    _WebKitDownload, WebKitDownloadPrivate, WebKitDownload, _WebKitDownloadPrivate,
};
pub use self::WebKitWebsiteDataManager_h::{
    _WebKitWebsiteDataManager, WebKitWebsiteDataManagerPrivate, WebKitWebsiteDataManager,
    _WebKitWebsiteDataManagerPrivate, webkit_website_data_manager_new,
    webkit_website_data_manager_get_base_data_directory,
    webkit_website_data_manager_get_base_cache_directory,
};
pub use self::WebKitWebContext_h::{
    _WebKitWebContext, WebKitWebContextPrivate, WebKitWebContext,
    _WebKitWebContextPrivate, webkit_web_context_new_with_website_data_manager,
    webkit_web_context_get_cookie_manager,
    webkit_web_context_set_favicon_database_directory,
    webkit_web_context_get_spell_checking_languages,
    webkit_web_context_set_spell_checking_languages,
};
use self::gmem_h::g_free;
use self::gstrfuncs_h::g_strjoinv;
use self::gtestutils_h::g_assertion_message_expr;
extern "C" {
    #[c2rust::src_loc = "35:1"]
    pub fn download_start_cb(
        _: *mut WebKitWebContext,
        _: *mut WebKitDownload,
        _: gpointer,
    ) -> gboolean;
}
#[c2rust::src_loc = "28:26"]
static mut web_context: *mut WebKitWebContext = 0 as *const WebKitWebContext
    as *mut WebKitWebContext;
#[c2rust::src_loc = "30:14"]
static mut process_limit: guint = 0 as std::ffi::c_int as guint;
#[c2rust::src_loc = "32:17"]
static mut web_context_started: gboolean = 0 as std::ffi::c_int;
#[no_mangle]
#[c2rust::src_loc = "37:1"]
pub unsafe extern "C" fn web_context_get() -> *mut WebKitWebContext {
    if !web_context.is_null() {} else {
        g_assertion_message_expr(
            0 as *mut gchar,
            b"web_context.c\0" as *const u8 as *const std::ffi::c_char,
            40 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 16],
                &[std::ffi::c_char; 16],
            >(b"web_context_get\0"))
                .as_ptr(),
            b"web_context\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    return web_context;
}
#[no_mangle]
#[c2rust::src_loc = "44:1"]
pub unsafe extern "C" fn web_context_process_limit_get() -> guint {
    return process_limit;
}
#[no_mangle]
#[c2rust::src_loc = "50:1"]
pub unsafe extern "C" fn web_context_process_limit_set(mut limit: guint) -> gboolean {
    if web_context_started != 0 {
        return 0 as std::ffi::c_int;
    }
    process_limit = limit;
    return (0 as std::ffi::c_int == 0) as std::ffi::c_int;
}
#[c2rust::src_loc = "59:1"]
unsafe extern "C" fn website_data_manager_init() {
    let mut data_mgr: *mut WebKitWebsiteDataManager = webkit_website_data_manager_new(
        b"base-cache-directory\0" as *const u8 as *const std::ffi::c_char,
        globalconf.cache_dir,
        b"base-data-directory\0" as *const u8 as *const std::ffi::c_char,
        globalconf.data_dir,
        0 as *mut std::ffi::c_void,
    );
    web_context = webkit_web_context_new_with_website_data_manager(data_mgr);
    _log(
        LOG_LEVEL_verbose,
        b"web_context.c\0" as *const u8 as *const std::ffi::c_char,
        b"base_data_directory:                 %s\0" as *const u8
            as *const std::ffi::c_char,
        webkit_website_data_manager_get_base_data_directory(data_mgr),
    );
    _log(
        LOG_LEVEL_verbose,
        b"web_context.c\0" as *const u8 as *const std::ffi::c_char,
        b"base_cache_directory:                %s\0" as *const u8
            as *const std::ffi::c_char,
        webkit_website_data_manager_get_base_cache_directory(data_mgr),
    );
}
#[c2rust::src_loc = "73:1"]
unsafe extern "C" fn web_context_set_default_spelling_language() {
    let mut null: *const gchar = 0 as *const gchar;
    webkit_web_context_set_spell_checking_languages(web_context, &mut null);
    let mut ret: *mut *mut gchar = webkit_web_context_get_spell_checking_languages(
        web_context,
    ) as *mut *mut gchar;
    if ret.is_null() {
        return;
    }
    let mut langs: *mut gchar = g_strjoinv(
        b", \0" as *const u8 as *const std::ffi::c_char,
        ret,
    );
    _log(
        LOG_LEVEL_verbose,
        b"web_context.c\0" as *const u8 as *const std::ffi::c_char,
        b"setting spell check languages: %s\0" as *const u8 as *const std::ffi::c_char,
        langs,
    );
    g_free(langs as gpointer);
}
#[no_mangle]
#[c2rust::src_loc = "87:1"]
pub unsafe extern "C" fn web_context_init() {
    website_data_manager_init();
    webkit_web_context_set_favicon_database_directory(web_context, 0 as *const gchar);
    g_signal_connect_data(
        g_type_check_instance_cast(
            web_context as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject as gpointer,
        b"download-started\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut WebKitWebContext,
                    *mut WebKitDownload,
                    gpointer,
                ) -> gboolean,
            >,
            GCallback,
        >(
            Some(
                download_start_cb
                    as unsafe extern "C" fn(
                        *mut WebKitWebContext,
                        *mut WebKitDownload,
                        gpointer,
                    ) -> gboolean,
            ),
        ),
        0 as *mut std::ffi::c_void,
        None,
        G_CONNECT_DEFAULT,
    );
    let mut cookie_mgr: *mut WebKitCookieManager = webkit_web_context_get_cookie_manager(
        web_context,
    );
    webkit_cookie_manager_set_accept_policy(
        cookie_mgr,
        WEBKIT_COOKIE_POLICY_ACCEPT_NO_THIRD_PARTY,
    );
    web_context_set_default_spelling_language();
}
#[no_mangle]
#[c2rust::src_loc = "102:1"]
pub unsafe extern "C" fn web_context_init_finish() {
    if web_context_started != 0 {
        return;
    }
    web_context_started = (0 as std::ffi::c_int == 0) as std::ffi::c_int;
}
