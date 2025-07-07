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

use ::libc;

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
#[c2rust::header_src = "/home/daana/git/luakit/clib/widget.h:22"]
pub mod widget_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "65:8"]
    pub struct widget_t {
        pub signals: *mut signal_t,
        pub info: *const widget_info_t,
        pub destructor: Option::<widget_destructor_t>,
        pub index: Option::<
            unsafe extern "C" fn(*mut lua_State, *mut widget_t, luakit_token_t) -> gint,
        >,
        pub newindex: Option::<
            unsafe extern "C" fn(*mut lua_State, *mut widget_t, luakit_token_t) -> gint,
        >,
        pub ref_0: gpointer,
        pub widget: *mut GtkWidget,
        pub provider: *mut GtkCssProvider,
        pub prev_width: gint,
        pub prev_height: gint,
        pub data: gpointer,
    }
    #[c2rust::src_loc = "41:1"]
    pub type widget_destructor_t = unsafe extern "C" fn(*mut widget_t) -> ();
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "58:15"]
    pub struct widget_info_t {
        pub tok: luakit_token_t,
        pub name: *const gchar,
        pub wc: Option::<widget_constructor_t>,
    }
    #[c2rust::src_loc = "40:1"]
    pub type widget_constructor_t = unsafe extern "C" fn(
        *mut lua_State,
        *mut widget_t,
        luakit_token_t,
    ) -> *mut widget_t;
    #[c2rust::src_loc = "35:9"]
    pub const GOBJECT_LUAKIT_WIDGET_DATA_KEY: [std::ffi::c_char; 19] = unsafe {
        *::core::mem::transmute::<
            &[u8; 19],
            &[std::ffi::c_char; 19],
        >(b"luakit_widget_data\0")
    };
    #[inline]
    #[c2rust::src_loc = "95:1"]
    pub unsafe extern "C" fn luaH_checkwidget(
        mut L: *mut lua_State,
        mut udx: gint,
    ) -> *mut widget_t {
        let mut w = luaH_checkudata(L, udx, &mut widget_class) as *mut widget_t;
        if ((*w).widget).is_null() {
            luaL_error(
                L,
                b"widget %p (%s) has been destroyed\0" as *const u8
                    as *const std::ffi::c_char,
                w,
                (*(*w).info).name,
            );
        }
        if ({
            let mut __inst = (*w).widget as *mut GTypeInstance;
            let mut __t = gtk_widget_get_type();
            let mut __r: gboolean = 0;
            if __inst.is_null() {
                __r = 0 as std::ffi::c_int;
            } else if !((*__inst).g_class).is_null()
                && (*(*__inst).g_class).g_type == __t
            {
                __r = (0 as std::ffi::c_int == 0) as std::ffi::c_int;
            } else {
                __r = g_type_check_instance_is_a(__inst, __t);
            }
            __r
        }) != 0
        {} else {
            g_assertion_message_expr(
                G_LOG_DOMAIN as *const std::ffi::c_char,
                b"./clib/widget.h\0" as *const u8 as *const std::ffi::c_char,
                101 as std::ffi::c_int,
                (*::core::mem::transmute::<
                    &[u8; 17],
                    &[std::ffi::c_char; 17],
                >(b"luaH_checkwidget\0"))
                    .as_ptr(),
                b"GTK_IS_WIDGET(w->widget)\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        return w;
    }
    use super::signal_h::signal_t;
    use super::gtypes_h::{gint, gpointer, gchar, gboolean};
    use super::lua_h::lua_State;
    use super::tokenize_h::{luakit_token_t, L_TK_UNKNOWN};
    use super::gtktypes_h::GtkWidget;
    use super::gtkcssprovider_h::GtkCssProvider;
    use super::luaclass_h::{
        lua_class_t, lua_class_allocator_t, lua_class_property_array_t,
        lua_class_propfunc_t, luaH_checkudata,
    };
    use super::lauxlib_h::luaL_error;
    use super::gtype_h::{GTypeInstance, GType, g_type_check_instance_is_a};
    use super::gtkwidget_h::gtk_widget_get_type;
    use super::gtestutils_h::g_assertion_message_expr;
    use super::gmessages_h::G_LOG_DOMAIN;
    extern "C" {
        #[c2rust::src_loc = "43:1"]
        pub fn widget_box(
            _: *mut lua_State,
            _: *mut widget_t,
            _: luakit_token_t,
        ) -> *mut widget_t;
        #[c2rust::src_loc = "44:1"]
        pub fn widget_entry(
            _: *mut lua_State,
            _: *mut widget_t,
            _: luakit_token_t,
        ) -> *mut widget_t;
        #[c2rust::src_loc = "45:1"]
        pub fn widget_eventbox(
            _: *mut lua_State,
            _: *mut widget_t,
            _: luakit_token_t,
        ) -> *mut widget_t;
        #[c2rust::src_loc = "46:1"]
        pub fn widget_label(
            _: *mut lua_State,
            _: *mut widget_t,
            _: luakit_token_t,
        ) -> *mut widget_t;
        #[c2rust::src_loc = "47:1"]
        pub fn widget_notebook(
            _: *mut lua_State,
            _: *mut widget_t,
            _: luakit_token_t,
        ) -> *mut widget_t;
        #[c2rust::src_loc = "48:1"]
        pub fn widget_paned(
            _: *mut lua_State,
            _: *mut widget_t,
            _: luakit_token_t,
        ) -> *mut widget_t;
        #[c2rust::src_loc = "49:1"]
        pub fn widget_webview(
            _: *mut lua_State,
            _: *mut widget_t,
            _: luakit_token_t,
        ) -> *mut widget_t;
        #[c2rust::src_loc = "50:1"]
        pub fn widget_window(
            _: *mut lua_State,
            _: *mut widget_t,
            _: luakit_token_t,
        ) -> *mut widget_t;
        #[c2rust::src_loc = "51:1"]
        pub fn widget_overlay(
            _: *mut lua_State,
            _: *mut widget_t,
            _: luakit_token_t,
        ) -> *mut widget_t;
        #[c2rust::src_loc = "52:1"]
        pub fn widget_scrolled(
            _: *mut lua_State,
            _: *mut widget_t,
            _: luakit_token_t,
        ) -> *mut widget_t;
        #[c2rust::src_loc = "53:1"]
        pub fn widget_image(
            _: *mut lua_State,
            _: *mut widget_t,
            _: luakit_token_t,
        ) -> *mut widget_t;
        #[c2rust::src_loc = "54:1"]
        pub fn widget_spinner(
            _: *mut lua_State,
            _: *mut widget_t,
            _: luakit_token_t,
        ) -> *mut widget_t;
        #[c2rust::src_loc = "55:1"]
        pub fn widget_drawing_area(
            _: *mut lua_State,
            _: *mut widget_t,
            _: luakit_token_t,
        ) -> *mut widget_t;
        #[c2rust::src_loc = "56:1"]
        pub fn widget_stack(
            _: *mut lua_State,
            _: *mut widget_t,
            _: luakit_token_t,
        ) -> *mut widget_t;
        #[c2rust::src_loc = "90:20"]
        pub static mut widget_class: lua_class_t;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtypes.h:22"]
pub mod gtypes_h {
    #[c2rust::src_loc = "109:1"]
    pub type gpointer = *mut std::ffi::c_void;
    #[c2rust::src_loc = "55:1"]
    pub type gint = std::ffi::c_int;
    #[c2rust::src_loc = "61:1"]
    pub type guint = std::ffi::c_uint;
    #[c2rust::src_loc = "52:1"]
    pub type gchar = std::ffi::c_char;
    #[c2rust::src_loc = "56:1"]
    pub type gboolean = gint;
    #[c2rust::src_loc = "110:1"]
    pub type gconstpointer = *const std::ffi::c_void;
    #[c2rust::src_loc = "114:1"]
    pub type GCompareDataFunc = Option::<
        unsafe extern "C" fn(gconstpointer, gconstpointer, gpointer) -> gint,
    >;
    #[c2rust::src_loc = "140:1"]
    pub type GDestroyNotify = Option::<unsafe extern "C" fn(gpointer) -> ()>;
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gtk/gtkcssprovider.h:22"]
pub mod gtkcssprovider_h {
    #[c2rust::src_loc = "64:1"]
    pub type GtkCssProvider = _GtkCssProvider;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "68:8"]
    pub struct _GtkCssProvider {
        pub parent_instance: GObject,
        pub priv_0: *mut GtkCssProviderPrivate,
    }
    #[c2rust::src_loc = "66:1"]
    pub type GtkCssProviderPrivate = _GtkCssProviderPrivate;
    use super::gobject_h::GObject;
    use super::gtypes_h::{gchar, gboolean};
    use super::glibconfig_h::gssize;
    use super::gerror_h::GError;
    extern "C" {
        #[c2rust::src_loc = "66:16"]
        pub type _GtkCssProviderPrivate;
        #[c2rust::src_loc = "91:1"]
        pub fn gtk_css_provider_new() -> *mut GtkCssProvider;
        #[c2rust::src_loc = "94:1"]
        pub fn gtk_css_provider_to_string(
            provider: *mut GtkCssProvider,
        ) -> *mut std::ffi::c_char;
        #[c2rust::src_loc = "97:1"]
        pub fn gtk_css_provider_load_from_data(
            css_provider: *mut GtkCssProvider,
            data: *const gchar,
            length: gssize,
            error: *mut *mut GError,
        ) -> gboolean;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gobject.h:22"]
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
    use super::gtypes_h::{guint, gchar, gpointer};
    use super::gdataset_h::GData;
    extern "C" {
        #[c2rust::src_loc = "596:1"]
        pub fn g_object_set_data(
            object: *mut GObject,
            key: *const gchar,
            data: gpointer,
        );
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
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gtype.h:22"]
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
    use super::gtypes_h::gboolean;
    extern "C" {
        #[c2rust::src_loc = "2626:1"]
        pub fn g_type_check_instance_cast(
            instance: *mut GTypeInstance,
            iface_type: GType,
        ) -> *mut GTypeInstance;
        #[c2rust::src_loc = "2629:1"]
        pub fn g_type_check_instance_is_a(
            instance: *mut GTypeInstance,
            iface_type: GType,
        ) -> gboolean;
    }
}
#[c2rust::header_src = "/usr/lib/glib-2.0/include/glibconfig.h:22"]
pub mod glibconfig_h {
    #[c2rust::src_loc = "83:1"]
    pub type gsize = std::ffi::c_ulong;
    #[c2rust::src_loc = "57:1"]
    pub type guint32 = std::ffi::c_uint;
    #[c2rust::src_loc = "82:1"]
    pub type gssize = std::ffi::c_long;
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gtk/gtktypes.h:22"]
pub mod gtktypes_h {
    #[c2rust::src_loc = "46:1"]
    pub type GtkWidget = _GtkWidget;
    #[c2rust::src_loc = "44:1"]
    pub type GtkStyleContext = _GtkStyleContext;
    use super::gtkwidget_h::_GtkWidget;
    use super::gtkstylecontext_h::_GtkStyleContext;
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gtk/gtkwidget.h:22"]
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
    use super::gtype_h::GType;
    use super::gtktypes_h::{GtkWidget, GtkStyleContext};
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "66:16"]
        pub type _GtkWidgetPrivate;
        #[c2rust::src_loc = "612:1"]
        pub fn gtk_widget_get_type() -> GType;
        #[c2rust::src_loc = "816:1"]
        pub fn gtk_widget_set_name(widget: *mut GtkWidget, name: *const gchar);
        #[c2rust::src_loc = "1310:1"]
        pub fn gtk_widget_get_style_context(
            widget: *mut GtkWidget,
        ) -> *mut GtkStyleContext;
    }
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
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "285:1"]
        pub fn l_tokenize(_: *const gchar) -> luakit_token_t;
    }
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lua.h:22"]
pub mod lua_h {
    #[c2rust::src_loc = "53:1"]
    pub type lua_CFunction = Option::<
        unsafe extern "C" fn(*mut lua_State) -> std::ffi::c_int,
    >;
    #[c2rust::src_loc = "30:9"]
    pub const LUA_MULTRET: std::ffi::c_int = -(1 as std::ffi::c_int);
    #[c2rust::src_loc = "36:9"]
    pub const LUA_REGISTRYINDEX: std::ffi::c_int = -(10000 as std::ffi::c_int);
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
        #[c2rust::src_loc = "153:1"]
        pub fn lua_touserdata(
            L: *mut lua_State,
            idx: std::ffi::c_int,
        ) -> *mut std::ffi::c_void;
        #[c2rust::src_loc = "164:1"]
        pub fn lua_pushlstring(L: *mut lua_State, s: *const std::ffi::c_char, l: size_t);
        #[c2rust::src_loc = "165:1"]
        pub fn lua_pushstring(L: *mut lua_State, s: *const std::ffi::c_char);
        #[c2rust::src_loc = "170:1"]
        pub fn lua_pushboolean(L: *mut lua_State, b: std::ffi::c_int);
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
            NULL_0 as *mut std::ffi::c_void,
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
    use super::__stddef_null_h::NULL_0;
    use super::gmem_h::g_free;
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
#[c2rust::header_src = "/usr/lib/clang/20/include/__stddef_size_t.h:22"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = std::ffi::c_ulong;
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
#[c2rust::header_src = "/usr/include/glib-2.0/glib/ghash.h:22"]
pub mod ghash_h {
    #[c2rust::src_loc = "40:1"]
    pub type GHashTable = _GHashTable;
    extern "C" {
        #[c2rust::src_loc = "40:16"]
        pub type _GHashTable;
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
    use super::tokenize_h::luakit_token_t;
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
        #[c2rust::src_loc = "86:1"]
        pub fn luaH_class_new(_: *mut lua_State, _: *mut lua_class_t) -> gint;
        #[c2rust::src_loc = "88:1"]
        pub fn luaH_checkudata(
            _: *mut lua_State,
            _: gint,
            _: *mut lua_class_t,
        ) -> gpointer;
    }
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gtk/gtkstylecontext.h:22"]
pub mod gtkstylecontext_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "43:8"]
    pub struct _GtkStyleContext {
        pub parent_object: GObject,
        pub priv_0: *mut GtkStyleContextPrivate,
    }
    #[c2rust::src_loc = "41:1"]
    pub type GtkStyleContextPrivate = _GtkStyleContextPrivate;
    use super::gobject_h::GObject;
    use super::gtktypes_h::GtkStyleContext;
    use super::gtkstyleprovider_h::GtkStyleProvider;
    use super::gtypes_h::guint;
    extern "C" {
        #[c2rust::src_loc = "41:16"]
        pub type _GtkStyleContextPrivate;
        #[c2rust::src_loc = "1032:1"]
        pub fn gtk_style_context_add_provider(
            context: *mut GtkStyleContext,
            provider: *mut GtkStyleProvider,
            priority: guint,
        );
    }
}
#[c2rust::header_src = "/usr/include/gtk-3.0/gtk/gtkstyleprovider.h:22"]
pub mod gtkstyleprovider_h {
    #[c2rust::src_loc = "90:1"]
    pub type GtkStyleProvider = _GtkStyleProvider;
    #[c2rust::src_loc = "76:9"]
    pub const GTK_STYLE_PROVIDER_PRIORITY_APPLICATION: std::ffi::c_int = 600
        as std::ffi::c_int;
    use super::gtype_h::GType;
    extern "C" {
        #[c2rust::src_loc = "90:16"]
        pub type _GtkStyleProvider;
        #[c2rust::src_loc = "118:1"]
        pub fn gtk_style_provider_get_type() -> GType;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/property.h:23"]
pub mod property_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "46:9"]
    pub struct property_t {
        pub tok: luakit_token_t,
        pub name: *const gchar,
        pub type_0: property_value_t,
        pub writable: gboolean,
    }
    #[c2rust::src_loc = "29:9"]
    pub type property_value_t = std::ffi::c_uint;
    #[c2rust::src_loc = "35:5"]
    pub const URI: property_value_t = 5;
    #[c2rust::src_loc = "34:5"]
    pub const INT: property_value_t = 4;
    #[c2rust::src_loc = "33:5"]
    pub const FLOAT: property_value_t = 3;
    #[c2rust::src_loc = "32:5"]
    pub const DOUBLE: property_value_t = 2;
    #[c2rust::src_loc = "31:5"]
    pub const CHAR: property_value_t = 1;
    #[c2rust::src_loc = "30:5"]
    pub const BOOL: property_value_t = 0;
    use super::tokenize_h::luakit_token_t;
    use super::gtypes_h::{gchar, gboolean, gint};
    use super::lua_h::lua_State;
    use super::gobject_h::GObject;
    extern "C" {
        #[c2rust::src_loc = "53:1"]
        pub fn luaH_gobject_index(
            _: *mut lua_State,
            _: *mut property_t,
            _: luakit_token_t,
            _: *mut GObject,
        ) -> gint;
        #[c2rust::src_loc = "54:1"]
        pub fn luaH_gobject_newindex(
            _: *mut lua_State,
            _: *mut property_t,
            _: luakit_token_t,
            _: gint,
            _: *mut GObject,
        ) -> gboolean;
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
            return NULL as *mut std::ffi::c_char;
        }
        if 0 != 0 && !str.is_null() && 0 != 0 {
            let len = (strlen(str))
                .wrapping_add(1 as std::ffi::c_int as std::ffi::c_ulong);
            let mut dup_str = g_malloc(len) as *mut std::ffi::c_char;
            return memcpy(
                dup_str as *mut std::ffi::c_void,
                str as *const std::ffi::c_void,
                len,
            ) as *mut std::ffi::c_char;
        }
        return g_strdup(str);
    }
    use super::gtypes_h::gchar;
    use super::__stddef_null_h::NULL;
    use super::string_h::{strlen, memcpy};
    use super::__stddef_size_t_h::size_t;
    use super::gmem_h::g_malloc;
    extern "C" {
        #[c2rust::src_loc = "283:1"]
        pub fn g_strdup(str: *const gchar) -> *mut gchar;
        #[c2rust::src_loc = "285:1"]
        pub fn g_strdup_printf(format: *const gchar, _: ...) -> *mut gchar;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gmessages.h:22"]
pub mod gmessages_h {
    #[c2rust::src_loc = "320:9"]
    pub const G_LOG_DOMAIN: std::ffi::c_int = 0 as std::ffi::c_int;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtestutils.h:22"]
pub mod gtestutils_h {
    extern "C" {
        #[c2rust::src_loc = "282:1"]
        pub fn g_strcmp0(
            str1: *const std::ffi::c_char,
            str2: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
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
        lua_rawget(L, LUA_REGISTRYINDEX);
    }
    #[inline]
    #[c2rust::src_loc = "99:1"]
    pub unsafe extern "C" fn luaH_object_ref(
        mut L: *mut lua_State,
        mut oud: gint,
    ) -> gpointer {
        luaH_object_registry_push(L);
        let mut p = luaH_object_incref(
            L,
            -(1 as std::ffi::c_int),
            if oud < 0 as std::ffi::c_int { oud - 1 as std::ffi::c_int } else { oud },
        );
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        return p;
    }
    #[inline]
    #[c2rust::src_loc = "112:1"]
    pub unsafe extern "C" fn luaH_object_ref_class(
        mut L: *mut lua_State,
        mut oud: gint,
        mut class: *mut lua_class_t,
    ) -> gpointer {
        luaH_checkudata(L, oud, class);
        return luaH_object_ref(L, oud);
    }
    use super::lua_h::{
        lua_State, lua_pushlstring, lua_rawget, LUA_REGISTRYINDEX, lua_settop,
    };
    use super::luaclass_h::{lua_class_t, luaH_checkudata};
    use super::gtypes_h::{gint, gpointer};
    use super::tokenize_h::luakit_token_t;
    extern "C" {
        #[c2rust::src_loc = "38:1"]
        pub fn luaH_settype(L: *mut lua_State, lua_class: *mut lua_class_t) -> gint;
        #[c2rust::src_loc = "40:1"]
        pub fn luaH_object_incref(L: *mut lua_State, tud: gint, oud: gint) -> gpointer;
        #[c2rust::src_loc = "167:1"]
        pub fn luaH_object_add_signal_simple(L: *mut lua_State) -> gint;
        #[c2rust::src_loc = "168:1"]
        pub fn luaH_object_remove_signal_simple(L: *mut lua_State) -> gint;
        #[c2rust::src_loc = "169:1"]
        pub fn luaH_object_remove_signals_simple(L: *mut lua_State) -> gint;
        #[c2rust::src_loc = "170:1"]
        pub fn luaH_object_emit_signal_simple(L: *mut lua_State) -> gint;
        #[c2rust::src_loc = "171:1"]
        pub fn luaH_object_property_signal(
            _: *mut lua_State,
            _: gint,
            _: luakit_token_t,
        ) -> gint;
        #[c2rust::src_loc = "203:1"]
        pub fn luaH_object_tostring(_: *mut lua_State) -> gint;
        #[c2rust::src_loc = "204:1"]
        pub fn luaH_object_gc(_: *mut lua_State) -> gint;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gmacros.h:22"]
pub mod gmacros_h {
    #[c2rust::src_loc = "931:9"]
    pub const FALSE: std::ffi::c_int = 0 as std::ffi::c_int;
    #[c2rust::src_loc = "935:9"]
    pub const TRUE: std::ffi::c_int = (FALSE == 0) as std::ffi::c_int;
}
#[c2rust::header_src = "/usr/lib/clang/20/include/__stddef_null.h:22"]
pub mod __stddef_null_h {
    #[c2rust::src_loc = "26:9"]
    pub const NULL: std::ffi::c_int = 0 as std::ffi::c_int;
    #[c2rust::src_loc = "26:9"]
    pub const NULL_0: std::ffi::c_int = 0 as std::ffi::c_int;
    #[c2rust::src_loc = "26:9"]
    pub const NULL_1: std::ffi::c_int = 0 as std::ffi::c_int;
}
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::widget_h::{
    widget_t, widget_destructor_t, widget_info_t, widget_constructor_t,
    GOBJECT_LUAKIT_WIDGET_DATA_KEY, luaH_checkwidget, widget_box, widget_entry,
    widget_eventbox, widget_label, widget_notebook, widget_paned, widget_webview,
    widget_window, widget_overlay, widget_scrolled, widget_image, widget_spinner,
    widget_drawing_area, widget_stack, widget_class,
};
pub use self::gtypes_h::{
    gpointer, gint, guint, gchar, gboolean, gconstpointer, GCompareDataFunc,
    GDestroyNotify,
};
pub use self::gtkcssprovider_h::{
    GtkCssProvider, _GtkCssProvider, GtkCssProviderPrivate, _GtkCssProviderPrivate,
    gtk_css_provider_new, gtk_css_provider_to_string, gtk_css_provider_load_from_data,
};
pub use self::gobject_h::{GObject, _GObject, GInitiallyUnowned, g_object_set_data};
pub use self::gdataset_h::{GData, _GData};
pub use self::gtype_h::{
    GTypeInstance, _GTypeInstance, GTypeClass, _GTypeClass, GType,
    g_type_check_instance_cast, g_type_check_instance_is_a,
};
pub use self::glibconfig_h::{gsize, guint32, gssize};
pub use self::gtktypes_h::{GtkWidget, GtkStyleContext};
pub use self::gtkwidget_h::{
    _GtkWidget, GtkWidgetPrivate, _GtkWidgetPrivate, gtk_widget_get_type,
    gtk_widget_set_name, gtk_widget_get_style_context,
};
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
    L_TK_UNKNOWN, l_tokenize,
};
pub use self::lua_h::{
    lua_CFunction, LUA_MULTRET, LUA_REGISTRYINDEX, lua_State, lua_gettop, lua_settop,
    lua_pushvalue, lua_touserdata, lua_pushlstring, lua_pushstring, lua_pushboolean,
    lua_rawget, lua_createtable, lua_newuserdata, lua_setmetatable, lua_setfenv,
};
pub use self::signal_h::{signal_t, signal_cmp, signal_array_destroy, signal_new};
pub use self::gtree_h::{GTree, _GTree, g_tree_new_full};
pub use self::__stddef_size_t_h::size_t;
pub use self::garray_h::{_GPtrArray, GPtrArray, g_ptr_array_free};
pub use self::__stdarg_va_list_h::va_list;
pub use self::gquark_h::GQuark;
pub use self::gerror_h::{_GError, GError};
pub use self::ghash_h::{GHashTable, _GHashTable};
pub use self::log_h::{
    log_level_t, LOG_LEVEL_debug, LOG_LEVEL_verbose, LOG_LEVEL_info, LOG_LEVEL_warn,
    LOG_LEVEL_error, LOG_LEVEL_fatal, _log,
};
pub use self::lauxlib_h::{luaL_Reg, luaL_checklstring, luaL_error};
pub use self::luaclass_h::{
    lua_class_property_array_t, lua_object_t, lua_class_allocator_t,
    lua_class_propfunc_t, lua_class_t, luaH_class_add_signal, luaH_class_remove_signal,
    luaH_class_emit_signal, luaH_class_setup, luaH_class_add_property, luaH_class_index,
    luaH_class_newindex, luaH_class_new, luaH_checkudata,
};
pub use self::gtkstylecontext_h::{
    _GtkStyleContext, GtkStyleContextPrivate, _GtkStyleContextPrivate,
    gtk_style_context_add_provider,
};
pub use self::gtkstyleprovider_h::{
    GtkStyleProvider, GTK_STYLE_PROVIDER_PRIORITY_APPLICATION, _GtkStyleProvider,
    gtk_style_provider_get_type,
};
pub use self::property_h::{
    property_t, property_value_t, URI, INT, FLOAT, DOUBLE, CHAR, BOOL,
    luaH_gobject_index, luaH_gobject_newindex,
};
use self::string_h::{memcpy, memset, strlen};
use self::gmem_h::{g_free, g_malloc};
pub use self::gstrfuncs_h::{g_strdup_inline, g_strdup, g_strdup_printf};
pub use self::gmessages_h::G_LOG_DOMAIN;
use self::gtestutils_h::{g_strcmp0, g_assertion_message_expr};
pub use self::luaobject_h::{
    luaH_object_registry_push, luaH_object_ref, luaH_object_ref_class, luaH_settype,
    luaH_object_incref, luaH_object_add_signal_simple, luaH_object_remove_signal_simple,
    luaH_object_remove_signals_simple, luaH_object_emit_signal_simple,
    luaH_object_property_signal, luaH_object_tostring, luaH_object_gc,
};
pub use self::gmacros_h::{FALSE, TRUE};
pub use self::__stddef_null_h::{NULL, NULL_0, NULL_1};
#[c2rust::src_loc = "25:19"]
static mut widget_properties: [property_t; 7] = [
    {
        let mut init = property_t {
            tok: L_TK_MARGIN,
            name: b"margin\0" as *const u8 as *const std::ffi::c_char,
            type_0: INT,
            writable: TRUE,
        };
        init
    },
    {
        let mut init = property_t {
            tok: L_TK_MARGIN_TOP,
            name: b"margin-top\0" as *const u8 as *const std::ffi::c_char,
            type_0: INT,
            writable: TRUE,
        };
        init
    },
    {
        let mut init = property_t {
            tok: L_TK_MARGIN_BOTTOM,
            name: b"margin-bottom\0" as *const u8 as *const std::ffi::c_char,
            type_0: INT,
            writable: TRUE,
        };
        init
    },
    {
        let mut init = property_t {
            tok: L_TK_MARGIN_LEFT,
            name: b"margin-left\0" as *const u8 as *const std::ffi::c_char,
            type_0: INT,
            writable: TRUE,
        };
        init
    },
    {
        let mut init = property_t {
            tok: L_TK_MARGIN_RIGHT,
            name: b"margin-right\0" as *const u8 as *const std::ffi::c_char,
            type_0: INT,
            writable: TRUE,
        };
        init
    },
    {
        let mut init = property_t {
            tok: L_TK_CAN_FOCUS,
            name: b"can-focus\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: TRUE,
        };
        init
    },
    {
        let mut init = property_t {
            tok: L_TK_UNKNOWN,
            name: NULL_1 as *const gchar,
            type_0: BOOL,
            writable: 0 as std::ffi::c_int,
        };
        init
    },
];
#[c2rust::src_loc = "35:22"]
static mut widgets_list: [widget_info_t; 16] = unsafe {
    [
        {
            let mut init = widget_info_t {
                tok: L_TK_ENTRY,
                name: b"entry\0" as *const u8 as *const std::ffi::c_char,
                wc: Some(widget_entry as widget_constructor_t),
            };
            init
        },
        {
            let mut init = widget_info_t {
                tok: L_TK_EVENTBOX,
                name: b"eventbox\0" as *const u8 as *const std::ffi::c_char,
                wc: Some(widget_eventbox as widget_constructor_t),
            };
            init
        },
        {
            let mut init = widget_info_t {
                tok: L_TK_HBOX,
                name: b"hbox\0" as *const u8 as *const std::ffi::c_char,
                wc: Some(widget_box as widget_constructor_t),
            };
            init
        },
        {
            let mut init = widget_info_t {
                tok: L_TK_HPANED,
                name: b"hpaned\0" as *const u8 as *const std::ffi::c_char,
                wc: Some(widget_paned as widget_constructor_t),
            };
            init
        },
        {
            let mut init = widget_info_t {
                tok: L_TK_LABEL,
                name: b"label\0" as *const u8 as *const std::ffi::c_char,
                wc: Some(widget_label as widget_constructor_t),
            };
            init
        },
        {
            let mut init = widget_info_t {
                tok: L_TK_NOTEBOOK,
                name: b"notebook\0" as *const u8 as *const std::ffi::c_char,
                wc: Some(widget_notebook as widget_constructor_t),
            };
            init
        },
        {
            let mut init = widget_info_t {
                tok: L_TK_VBOX,
                name: b"vbox\0" as *const u8 as *const std::ffi::c_char,
                wc: Some(widget_box as widget_constructor_t),
            };
            init
        },
        {
            let mut init = widget_info_t {
                tok: L_TK_VPANED,
                name: b"vpaned\0" as *const u8 as *const std::ffi::c_char,
                wc: Some(widget_paned as widget_constructor_t),
            };
            init
        },
        {
            let mut init = widget_info_t {
                tok: L_TK_WEBVIEW,
                name: b"webview\0" as *const u8 as *const std::ffi::c_char,
                wc: Some(widget_webview as widget_constructor_t),
            };
            init
        },
        {
            let mut init = widget_info_t {
                tok: L_TK_WINDOW,
                name: b"window\0" as *const u8 as *const std::ffi::c_char,
                wc: Some(widget_window as widget_constructor_t),
            };
            init
        },
        {
            let mut init = widget_info_t {
                tok: L_TK_OVERLAY,
                name: b"overlay\0" as *const u8 as *const std::ffi::c_char,
                wc: Some(widget_overlay as widget_constructor_t),
            };
            init
        },
        {
            let mut init = widget_info_t {
                tok: L_TK_SCROLLED,
                name: b"scrolled\0" as *const u8 as *const std::ffi::c_char,
                wc: Some(widget_scrolled as widget_constructor_t),
            };
            init
        },
        {
            let mut init = widget_info_t {
                tok: L_TK_IMAGE,
                name: b"image\0" as *const u8 as *const std::ffi::c_char,
                wc: Some(widget_image as widget_constructor_t),
            };
            init
        },
        {
            let mut init = widget_info_t {
                tok: L_TK_SPINNER,
                name: b"spinner\0" as *const u8 as *const std::ffi::c_char,
                wc: Some(widget_spinner as widget_constructor_t),
            };
            init
        },
        {
            let mut init = widget_info_t {
                tok: L_TK_DRAWING_AREA,
                name: b"drawing_area\0" as *const u8 as *const std::ffi::c_char,
                wc: Some(widget_drawing_area as widget_constructor_t),
            };
            init
        },
        {
            let mut init = widget_info_t {
                tok: L_TK_STACK,
                name: b"stack\0" as *const u8 as *const std::ffi::c_char,
                wc: Some(widget_stack as widget_constructor_t),
            };
            init
        },
    ]
};
#[inline]
#[c2rust::src_loc = "54:1"]
unsafe extern "C" fn luaH_widget_class_emit_signal(mut L: *mut lua_State) -> gint {
    return luaH_class_emit_signal(
        L,
        &mut widget_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, NULL_1 as *mut size_t),
        lua_gettop(L) - 1 as std::ffi::c_int,
        LUA_MULTRET,
    );
}
#[inline]
#[c2rust::src_loc = "54:1"]
unsafe extern "C" fn luaH_widget_class_remove_signal(mut L: *mut lua_State) -> gint {
    luaH_class_remove_signal(
        L,
        &mut widget_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, NULL_1 as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]
#[c2rust::src_loc = "54:1"]
unsafe extern "C" fn luaH_widget_class_add_signal(mut L: *mut lua_State) -> gint {
    luaH_class_add_signal(
        L,
        &mut widget_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, NULL_1 as *mut size_t),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]
#[c2rust::src_loc = "54:1"]
unsafe extern "C" fn widget_new(mut L: *mut lua_State) -> *mut widget_t {
    let mut p = lua_newuserdata(
        L,
        ::core::mem::size_of::<widget_t>() as std::ffi::c_ulong,
    ) as *mut widget_t;
    memset(
        p as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        (::core::mem::size_of::<widget_t>() as std::ffi::c_ulong)
            .wrapping_mul(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    (*p).signals = signal_new();
    luaH_settype(L, &mut widget_class);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    lua_setfenv(L, -(2 as std::ffi::c_int));
    lua_pushvalue(L, -(1 as std::ffi::c_int));
    luaH_class_emit_signal(
        L,
        &mut widget_class,
        b"new\0" as *const u8 as *const std::ffi::c_char,
        1 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    return p;
}
#[c2rust::src_loc = "60:1"]
unsafe extern "C" fn luaH_widget_gc(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkudata(L, 1 as std::ffi::c_int, &mut widget_class)
        as *mut widget_t;
    if !((*w).info).is_null() {
        _log(
            LOG_LEVEL_debug,
            b"clib/widget.c\0" as *const u8 as *const std::ffi::c_char,
            b"collecting widget at %p of type '%s'\0" as *const u8
                as *const std::ffi::c_char,
            w,
            (*(*w).info).name,
        );
    }
    if ((*w).destructor).is_none() {} else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"clib/widget.c\0" as *const u8 as *const std::ffi::c_char,
            66 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 15],
                &[std::ffi::c_char; 15],
            >(b"luaH_widget_gc\0"))
                .as_ptr(),
            b"!w->destructor\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    return luaH_object_gc(L);
}
#[no_mangle]
#[c2rust::src_loc = "77:1"]
pub unsafe extern "C" fn luaH_widget_new(mut L: *mut lua_State) -> gint {
    luaH_class_new(L, &mut widget_class);
    let mut w = lua_touserdata(L, -(1 as std::ffi::c_int)) as *mut widget_t;
    if ((*w).info).is_null() {
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        luaL_error(
            L,
            b"widget does not have a type\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    lua_pushvalue(L, -(1 as std::ffi::c_int));
    (*w).ref_0 = luaH_object_ref_class(L, -(1 as std::ffi::c_int), &mut widget_class);
    return 1 as std::ffi::c_int;
}
#[inline]
#[c2rust::src_loc = "96:1"]
unsafe extern "C" fn widget_set_css(mut w: *mut widget_t, mut properties: *const gchar) {
    let mut old_css = gtk_css_provider_to_string((*w).provider);
    let mut css = g_strdup_printf(
        b"%s\n#widget { %s }\0" as *const u8 as *const std::ffi::c_char,
        old_css,
        properties,
    );
    gtk_css_provider_load_from_data(
        (*w).provider,
        css,
        strlen(css) as gssize,
        NULL_1 as *mut *mut GError,
    );
    g_free(css as gpointer);
    g_free(old_css as gpointer);
}
#[no_mangle]
#[c2rust::src_loc = "106:1"]
pub unsafe extern "C" fn widget_set_css_properties(mut w: *mut widget_t, mut args: ...) {
    let mut argp: ::core::ffi::VaListImpl;
    argp = args.clone();
    let mut css = g_strdup_inline(b"\0" as *const u8 as *const std::ffi::c_char);
    let mut prop = 0 as *const gchar;
    loop {
        prop = argp.arg::<*mut gchar>();
        if prop.is_null() {
            break;
        }
        let mut value: *const gchar = argp.arg::<*mut gchar>();
        if strlen(prop) > 0 as std::ffi::c_int as std::ffi::c_ulong {} else {
            g_assertion_message_expr(
                G_LOG_DOMAIN as *const std::ffi::c_char,
                b"clib/widget.c\0" as *const u8 as *const std::ffi::c_char,
                116 as std::ffi::c_int,
                (*::core::mem::transmute::<
                    &[u8; 26],
                    &[std::ffi::c_char; 26],
                >(b"widget_set_css_properties\0"))
                    .as_ptr(),
                b"strlen(prop) > 0\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        if value.is_null() || strlen(value) == 0 as std::ffi::c_int as std::ffi::c_ulong
        {
            continue;
        }
        let mut tmp = css;
        css = g_strdup_printf(
            b"%s%s: %s;\0" as *const u8 as *const std::ffi::c_char,
            css,
            prop,
            value,
        );
        g_free(tmp as gpointer);
    }
    widget_set_css(w, css);
    g_free(css as gpointer);
}
#[c2rust::src_loc = "139:1"]
unsafe extern "C" fn luaH_widget_index(mut L: *mut lua_State) -> gint {
    let mut prop = luaL_checklstring(L, 2 as std::ffi::c_int, NULL_1 as *mut size_t);
    let mut token = l_tokenize(prop);
    if luaH_class_index(L) != 0 {
        return 1 as std::ffi::c_int;
    }
    if token as std::ffi::c_uint == L_TK_IS_ALIVE as std::ffi::c_int as std::ffi::c_uint
    {
        let mut w = luaH_checkudata(L, 1 as std::ffi::c_int, &mut widget_class)
            as *mut widget_t;
        lua_pushboolean(L, !w.is_null() as std::ffi::c_int);
        return 1 as std::ffi::c_int;
    }
    let mut ret: gint = 0;
    let mut widget = luaH_checkwidget(L, 1 as std::ffi::c_int);
    ret = luaH_gobject_index(
        L,
        widget_properties.as_mut_ptr(),
        token,
        g_type_check_instance_cast(
            (*widget).widget as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
    );
    if ret != 0 {
        return ret;
    }
    return if ((*widget).index).is_some() {
        ((*widget).index).expect("non-null function pointer")(L, widget, token)
    } else {
        0 as std::ffi::c_int
    };
}
#[c2rust::src_loc = "172:1"]
unsafe extern "C" fn luaH_widget_newindex(mut L: *mut lua_State) -> gint {
    let mut prop = luaL_checklstring(L, 2 as std::ffi::c_int, NULL_1 as *mut size_t);
    let mut token = l_tokenize(prop);
    luaH_class_newindex(L);
    let mut widget = luaH_checkwidget(L, 1 as std::ffi::c_int);
    if token as std::ffi::c_uint == L_TK_CSS as std::ffi::c_int as std::ffi::c_uint {
        widget_set_css(
            widget,
            luaL_checklstring(L, 3 as std::ffi::c_int, NULL_1 as *mut size_t),
        );
        return 0 as std::ffi::c_int;
    }
    let mut emit = luaH_gobject_newindex(
        L,
        widget_properties.as_mut_ptr(),
        token,
        3 as std::ffi::c_int,
        g_type_check_instance_cast(
            (*widget).widget as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
    );
    if emit != 0 {
        return luaH_object_property_signal(L, 1 as std::ffi::c_int, token);
    }
    return if ((*widget).newindex).is_some() {
        ((*widget).newindex).expect("non-null function pointer")(L, widget, token)
    } else {
        0 as std::ffi::c_int
    };
}
#[c2rust::src_loc = "199:1"]
unsafe extern "C" fn luaH_widget_set_type(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
) -> gint {
    if !((*w).info).is_null() {
        luaL_error(
            L,
            b"widget is already of type: %s\0" as *const u8 as *const std::ffi::c_char,
            (*(*w).info).name,
        );
    }
    let mut type_0 = luaL_checklstring(
        L,
        -(1 as std::ffi::c_int),
        NULL_1 as *mut size_t,
    );
    let mut tok = l_tokenize(type_0);
    let mut winfo = 0 as *const widget_info_t;
    (*w).provider = gtk_css_provider_new();
    let mut i = 0 as std::ffi::c_int as guint;
    while (i as std::ffi::c_ulong)
        < (::core::mem::size_of::<[widget_info_t; 16]>() as std::ffi::c_ulong)
            .wrapping_div(::core::mem::size_of::<widget_info_t>() as std::ffi::c_ulong)
    {
        if widgets_list[i as usize].tok as std::ffi::c_uint != tok as std::ffi::c_uint {
            i = i.wrapping_add(1);
            i;
        } else {
            winfo = &*widgets_list.as_ptr().offset(i as isize) as *const widget_info_t;
            (*w).info = winfo;
            ((*winfo).wc).expect("non-null function pointer")(L, w, tok);
            gtk_widget_set_name(
                g_type_check_instance_cast(
                    (*w).widget as *mut GTypeInstance,
                    gtk_widget_get_type(),
                ) as *mut std::ffi::c_void as *mut GtkWidget,
                b"widget\0" as *const u8 as *const std::ffi::c_char,
            );
            let mut context = gtk_widget_get_style_context(
                g_type_check_instance_cast(
                    (*w).widget as *mut GTypeInstance,
                    gtk_widget_get_type(),
                ) as *mut std::ffi::c_void as *mut GtkWidget,
            );
            gtk_style_context_add_provider(
                context,
                g_type_check_instance_cast(
                    (*w).provider as *mut GTypeInstance,
                    gtk_style_provider_get_type(),
                ) as *mut std::ffi::c_void as *mut GtkStyleProvider,
                GTK_STYLE_PROVIDER_PRIORITY_APPLICATION as guint,
            );
            g_object_set_data(
                g_type_check_instance_cast(
                    (*w).widget as *mut GTypeInstance,
                    ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
                ) as *mut std::ffi::c_void as *mut GObject,
                GOBJECT_LUAKIT_WIDGET_DATA_KEY.as_ptr(),
                w as gpointer,
            );
            _log(
                LOG_LEVEL_verbose,
                b"clib/widget.c\0" as *const u8 as *const std::ffi::c_char,
                b"created widget of type: %s\0" as *const u8 as *const std::ffi::c_char,
                (*(*w).info).name,
            );
            lua_pushvalue(L, -(3 as std::ffi::c_int));
            luaH_class_emit_signal(
                L,
                &mut widget_class,
                b"create\0" as *const u8 as *const std::ffi::c_char,
                1 as std::ffi::c_int,
                0 as std::ffi::c_int,
            );
            return 0 as std::ffi::c_int;
        }
    }
    luaL_error(
        L,
        b"unknown widget type: %s\0" as *const u8 as *const std::ffi::c_char,
        type_0,
    );
    return 0 as std::ffi::c_int;
}
#[c2rust::src_loc = "242:1"]
unsafe extern "C" fn luaH_widget_get_type(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
) -> gint {
    if ((*w).info).is_null() {
        return 0 as std::ffi::c_int;
    }
    luaH_checkwidget(L, 1 as std::ffi::c_int);
    lua_pushstring(L, (*(*w).info).name);
    return 1 as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "253:1"]
pub unsafe extern "C" fn widget_class_setup(mut L: *mut lua_State) {
    static mut widget_methods: [luaL_Reg; 5] = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"add_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_widget_class_add_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_widget_class_remove_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"emit_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_widget_class_emit_signal
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__call\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_widget_new as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: NULL_1 as *const std::ffi::c_char,
                    func: ::core::mem::transmute::<
                        libc::intptr_t,
                        lua_CFunction,
                    >(NULL_1 as libc::intptr_t),
                };
                init
            },
        ]
    };
    static mut widget_meta: [luaL_Reg; 9] = unsafe {
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
                        luaH_widget_index as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__newindex\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_widget_newindex
                            as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__gc\0" as *const u8 as *const std::ffi::c_char,
                    func: Some(
                        luaH_widget_gc as unsafe extern "C" fn(*mut lua_State) -> gint,
                    ),
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: NULL_1 as *const std::ffi::c_char,
                    func: ::core::mem::transmute::<
                        libc::intptr_t,
                        lua_CFunction,
                    >(NULL_1 as libc::intptr_t),
                };
                init
            },
        ]
    };
    luaH_class_setup(
        L,
        &mut widget_class,
        b"widget\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut lua_State) -> *mut widget_t>,
            lua_class_allocator_t,
        >(Some(widget_new as unsafe extern "C" fn(*mut lua_State) -> *mut widget_t)),
        ::core::mem::transmute::<
            libc::intptr_t,
            lua_class_propfunc_t,
        >(NULL_1 as libc::intptr_t),
        ::core::mem::transmute::<
            libc::intptr_t,
            lua_class_propfunc_t,
        >(NULL_1 as libc::intptr_t),
        widget_methods.as_ptr(),
        widget_meta.as_ptr(),
    );
    luaH_class_add_property(
        &mut widget_class,
        L_TK_TYPE,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut lua_State, *mut widget_t) -> gint>,
            lua_class_propfunc_t,
        >(
            Some(
                luaH_widget_set_type
                    as unsafe extern "C" fn(*mut lua_State, *mut widget_t) -> gint,
            ),
        ),
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut lua_State, *mut widget_t) -> gint>,
            lua_class_propfunc_t,
        >(
            Some(
                luaH_widget_get_type
                    as unsafe extern "C" fn(*mut lua_State, *mut widget_t) -> gint,
            ),
        ),
        ::core::mem::transmute::<
            libc::intptr_t,
            lua_class_propfunc_t,
        >(NULL_1 as libc::intptr_t),
    );
}
