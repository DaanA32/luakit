use ::libc;
use ::c2rust_bitfields;

pub mod __stddef_ptrdiff_t_h {

    pub type ptrdiff_t = std::ffi::c_long;
}

pub mod __stddef_size_t_h {

    pub type size_t = std::ffi::c_ulong;
}

pub mod glibconfig_h {

    pub type gint8 = std::ffi::c_schar;

    pub type guint8 = std::ffi::c_uchar;

    pub type gint16 = std::ffi::c_short;

    pub type guint16 = std::ffi::c_ushort;

    pub type guint32 = std::ffi::c_uint;

    pub type gint64 = std::ffi::c_long;

    pub type guint64 = std::ffi::c_ulong;

    pub type gssize = std::ffi::c_long;

    pub type gsize = std::ffi::c_ulong;
}

pub mod gtypes_h {

    pub type gchar = std::ffi::c_char;

    pub type gshort = std::ffi::c_short;

    pub type glong = std::ffi::c_long;

    pub type gint = std::ffi::c_int;

    pub type gboolean = gint;

    pub type gulong = std::ffi::c_ulong;

    pub type guint = std::ffi::c_uint;

    pub type gfloat = std::ffi::c_float;

    pub type gdouble = std::ffi::c_double;

    pub type gpointer = *mut std::ffi::c_void;
}

pub mod gdataset_h {

    pub type GData = _GData;
    extern "C" {

        pub type _GData;
    }
}

pub mod glist_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GList {
        pub data: gpointer,
        pub next: *mut GList,
        pub prev: *mut GList,
    }

    pub type GList = _GList;
    use super::gtypes_h::gpointer;
    extern "C" {

        pub fn g_list_free(list: *mut GList);
    }
}

pub mod ghash_h {

    pub type GHashTable = _GHashTable;
    extern "C" {

        pub type _GHashTable;
    }
}

pub mod gslist_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GSList {
        pub data: gpointer,
        pub next: *mut GSList,
    }

    pub type GSList = _GSList;
    use super::gtypes_h::gpointer;
}

pub mod gunicode_h {

    pub type gunichar = guint32;
    use super::glibconfig_h::{guint32, gssize};
    use super::gtypes_h::{gchar, glong, gboolean};
    extern "C" {

        pub fn g_utf8_get_char(p: *const gchar) -> gunichar;

        pub fn g_utf8_strlen(p: *const gchar, max: gssize) -> glong;

        pub fn g_utf8_validate(
            str: *const gchar,
            max_len: gssize,
            end: *mut *const gchar,
        ) -> gboolean;
    }
}

pub mod gstring_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GString {
        pub str_0: *mut gchar,
        pub len: gsize,
        pub allocated_len: gsize,
    }

    pub type GString = _GString;
    use super::gtypes_h::{gchar, gboolean};
    use super::glibconfig_h::gsize;
    extern "C" {

        pub fn g_string_sized_new(dfl_size: gsize) -> *mut GString;

        pub fn g_string_free(string: *mut GString, free_segment: gboolean) -> *mut gchar;

        pub fn g_string_free_and_steal(string: *mut GString) -> *mut gchar;

        pub fn g_string_append_printf(
            string: *mut GString,
            format: *const gchar,
            _: ...
        );
    }
}

pub mod gtree_h {

    pub type GTree = _GTree;
    extern "C" {

        pub type _GTree;
    }
}

pub mod gtype_h {

    pub type GType = gsize;

    pub type GValue = _GValue;
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
    use super::gvalue_h::_GValue;
    use super::gtypes_h::gboolean;
    extern "C" {

        pub fn g_type_check_instance_cast(
            instance: *mut GTypeInstance,
            iface_type: GType,
        ) -> *mut GTypeInstance;

        pub fn g_type_check_instance_is_a(
            instance: *mut GTypeInstance,
            iface_type: GType,
        ) -> gboolean;
    }
}

pub mod gvalue_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GValue {
        pub g_type: GType,
        pub data: [C2RustUnnamed; 2],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]

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
    use super::gtype_h::{GType, GValue};
    use super::gtypes_h::{gint, guint, glong, gulong, gfloat, gdouble, gpointer};
    use super::glibconfig_h::{gint64, guint64};
    extern "C" {

        pub fn g_value_init(value: *mut GValue, g_type: GType) -> *mut GValue;

        pub fn g_value_unset(value: *mut GValue);
    }
}

pub mod gparam_h {

    pub type GParamFlags = std::ffi::c_int;

    pub const G_PARAM_DEPRECATED: GParamFlags = -2147483648;

    pub const G_PARAM_EXPLICIT_NOTIFY: GParamFlags = 1073741824;

    pub const G_PARAM_STATIC_BLURB: GParamFlags = 128;

    pub const G_PARAM_STATIC_NICK: GParamFlags = 64;

    pub const G_PARAM_PRIVATE: GParamFlags = 32;

    pub const G_PARAM_STATIC_NAME: GParamFlags = 32;

    pub const G_PARAM_LAX_VALIDATION: GParamFlags = 16;

    pub const G_PARAM_CONSTRUCT_ONLY: GParamFlags = 8;

    pub const G_PARAM_CONSTRUCT: GParamFlags = 4;

    pub const G_PARAM_READWRITE: GParamFlags = 3;

    pub const G_PARAM_WRITABLE: GParamFlags = 2;

    pub const G_PARAM_READABLE: GParamFlags = 1;
    #[derive(Copy, Clone)]
    #[repr(C)]

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

    pub type GParamSpec = _GParamSpec;
    use super::gtype_h::{GTypeInstance, GType};
    use super::gtypes_h::{gchar, guint};
    use super::gdataset_h::GData;
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
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GObjectClass {
        pub g_type_class: GTypeClass,
        pub construct_properties: *mut GSList,
        pub constructor: Option::<
            unsafe extern "C" fn(
                GType,
                guint,
                *mut GObjectConstructParam,
            ) -> *mut GObject,
        >,
        pub set_property: Option::<
            unsafe extern "C" fn(
                *mut GObject,
                guint,
                *const GValue,
                *mut GParamSpec,
            ) -> (),
        >,
        pub get_property: Option::<
            unsafe extern "C" fn(*mut GObject, guint, *mut GValue, *mut GParamSpec) -> (),
        >,
        pub dispose: Option::<unsafe extern "C" fn(*mut GObject) -> ()>,
        pub finalize: Option::<unsafe extern "C" fn(*mut GObject) -> ()>,
        pub dispatch_properties_changed: Option::<
            unsafe extern "C" fn(*mut GObject, guint, *mut *mut GParamSpec) -> (),
        >,
        pub notify: Option::<unsafe extern "C" fn(*mut GObject, *mut GParamSpec) -> ()>,
        pub constructed: Option::<unsafe extern "C" fn(*mut GObject) -> ()>,
        pub flags: gsize,
        pub n_construct_properties: gsize,
        pub pspecs: gpointer,
        pub n_pspecs: gsize,
        pub pdummy: [gpointer; 3],
    }

    pub type GObjectConstructParam = _GObjectConstructParam;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GObjectConstructParam {
        pub pspec: *mut GParamSpec,
        pub value: *mut GValue,
    }

    pub type GObjectClass = _GObjectClass;

    pub type GInitiallyUnowned = _GObject;
    use super::gtype_h::{GTypeInstance, GTypeClass, GType, GValue};
    use super::gtypes_h::{guint, gpointer, gchar};
    use super::gdataset_h::GData;
    use super::gslist_h::GSList;
    use super::gparam_h::GParamSpec;
    use super::glibconfig_h::gsize;
    extern "C" {

        pub fn g_object_get(object: gpointer, first_property_name: *const gchar, _: ...);

        pub fn g_object_ref(object: gpointer) -> gpointer;

        pub fn g_object_get_data(object: *mut GObject, key: *const gchar) -> gpointer;
    }
}

pub mod cairo_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _cairo_rectangle_int {
        pub x: std::ffi::c_int,
        pub y: std::ffi::c_int,
        pub width: std::ffi::c_int,
        pub height: std::ffi::c_int,
    }

    pub type cairo_rectangle_int_t = _cairo_rectangle_int;

    pub type cairo_region_t = _cairo_region;
    extern "C" {

        pub type _cairo_region;
    }
}

pub mod gdktypes_h {

    pub type GdkRectangle = cairo_rectangle_int_t;

    pub type GdkAtom = *mut _GdkAtom;

    pub type GdkDevice = _GdkDevice;

    pub type GdkDragContext = _GdkDragContext;

    pub type GdkDisplay = _GdkDisplay;

    pub type GdkWindow = _GdkWindow;

    pub type GdkKeymap = _GdkKeymap;

    pub type GdkSeat = _GdkSeat;

    pub type C2RustUnnamed_0 = std::ffi::c_uint;

    pub const GDK_MODIFIER_MASK: C2RustUnnamed_0 = 1543512063;

    pub const GDK_RELEASE_MASK: C2RustUnnamed_0 = 1073741824;

    pub const GDK_MODIFIER_RESERVED_29_MASK: C2RustUnnamed_0 = 536870912;

    pub const GDK_META_MASK: C2RustUnnamed_0 = 268435456;

    pub const GDK_HYPER_MASK: C2RustUnnamed_0 = 134217728;

    pub const GDK_SUPER_MASK: C2RustUnnamed_0 = 67108864;

    pub const GDK_MODIFIER_RESERVED_25_MASK: C2RustUnnamed_0 = 33554432;

    pub const GDK_MODIFIER_RESERVED_24_MASK: C2RustUnnamed_0 = 16777216;

    pub const GDK_MODIFIER_RESERVED_23_MASK: C2RustUnnamed_0 = 8388608;

    pub const GDK_MODIFIER_RESERVED_22_MASK: C2RustUnnamed_0 = 4194304;

    pub const GDK_MODIFIER_RESERVED_21_MASK: C2RustUnnamed_0 = 2097152;

    pub const GDK_MODIFIER_RESERVED_20_MASK: C2RustUnnamed_0 = 1048576;

    pub const GDK_MODIFIER_RESERVED_19_MASK: C2RustUnnamed_0 = 524288;

    pub const GDK_MODIFIER_RESERVED_18_MASK: C2RustUnnamed_0 = 262144;

    pub const GDK_MODIFIER_RESERVED_17_MASK: C2RustUnnamed_0 = 131072;

    pub const GDK_MODIFIER_RESERVED_16_MASK: C2RustUnnamed_0 = 65536;

    pub const GDK_MODIFIER_RESERVED_15_MASK: C2RustUnnamed_0 = 32768;

    pub const GDK_MODIFIER_RESERVED_14_MASK: C2RustUnnamed_0 = 16384;

    pub const GDK_MODIFIER_RESERVED_13_MASK: C2RustUnnamed_0 = 8192;

    pub const GDK_BUTTON5_MASK: C2RustUnnamed_0 = 4096;

    pub const GDK_BUTTON4_MASK: C2RustUnnamed_0 = 2048;

    pub const GDK_BUTTON3_MASK: C2RustUnnamed_0 = 1024;

    pub const GDK_BUTTON2_MASK: C2RustUnnamed_0 = 512;

    pub const GDK_BUTTON1_MASK: C2RustUnnamed_0 = 256;

    pub const GDK_MOD5_MASK: C2RustUnnamed_0 = 128;

    pub const GDK_MOD4_MASK: C2RustUnnamed_0 = 64;

    pub const GDK_MOD3_MASK: C2RustUnnamed_0 = 32;

    pub const GDK_MOD2_MASK: C2RustUnnamed_0 = 16;

    pub const GDK_MOD1_MASK: C2RustUnnamed_0 = 8;

    pub const GDK_CONTROL_MASK: C2RustUnnamed_0 = 4;

    pub const GDK_LOCK_MASK: C2RustUnnamed_0 = 2;

    pub const GDK_SHIFT_MASK: C2RustUnnamed_0 = 1;

    pub type GdkEventMask = std::ffi::c_uint;

    pub const GDK_ALL_EVENTS_MASK: GdkEventMask = 67108862;

    pub const GDK_TABLET_PAD_MASK: GdkEventMask = 33554432;

    pub const GDK_TOUCHPAD_GESTURE_MASK: GdkEventMask = 16777216;

    pub const GDK_SMOOTH_SCROLL_MASK: GdkEventMask = 8388608;

    pub const GDK_TOUCH_MASK: GdkEventMask = 4194304;

    pub const GDK_SCROLL_MASK: GdkEventMask = 2097152;

    pub const GDK_SUBSTRUCTURE_MASK: GdkEventMask = 1048576;

    pub const GDK_PROXIMITY_OUT_MASK: GdkEventMask = 524288;

    pub const GDK_PROXIMITY_IN_MASK: GdkEventMask = 262144;

    pub const GDK_VISIBILITY_NOTIFY_MASK: GdkEventMask = 131072;

    pub const GDK_PROPERTY_CHANGE_MASK: GdkEventMask = 65536;

    pub const GDK_STRUCTURE_MASK: GdkEventMask = 32768;

    pub const GDK_FOCUS_CHANGE_MASK: GdkEventMask = 16384;

    pub const GDK_LEAVE_NOTIFY_MASK: GdkEventMask = 8192;

    pub const GDK_ENTER_NOTIFY_MASK: GdkEventMask = 4096;

    pub const GDK_KEY_RELEASE_MASK: GdkEventMask = 2048;

    pub const GDK_KEY_PRESS_MASK: GdkEventMask = 1024;

    pub const GDK_BUTTON_RELEASE_MASK: GdkEventMask = 512;

    pub const GDK_BUTTON_PRESS_MASK: GdkEventMask = 256;

    pub const GDK_BUTTON3_MOTION_MASK: GdkEventMask = 128;

    pub const GDK_BUTTON2_MOTION_MASK: GdkEventMask = 64;

    pub const GDK_BUTTON1_MOTION_MASK: GdkEventMask = 32;

    pub const GDK_BUTTON_MOTION_MASK: GdkEventMask = 16;

    pub const GDK_POINTER_MOTION_HINT_MASK: GdkEventMask = 8;

    pub const GDK_POINTER_MOTION_MASK: GdkEventMask = 4;

    pub const GDK_EXPOSURE_MASK: GdkEventMask = 2;

    pub const GDK_CURRENT_TIME: std::ffi::c_long = 0 as std::ffi::c_long;
    use super::cairo_h::cairo_rectangle_int_t;
    use super::gdkseat_h::_GdkSeat;
    extern "C" {

        pub type _GdkAtom;

        pub type _GdkDevice;

        pub type _GdkDragContext;

        pub type _GdkDisplay;

        pub type _GdkWindow;

        pub type _GdkKeymap;
    }
}

pub mod gdkseat_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GdkSeat {
        pub parent_instance: GObject,
    }
    use super::gobject_h::GObject;
    use super::gdktypes_h::{GdkSeat, GdkDevice};
    extern "C" {

        pub fn gdk_seat_get_keyboard(seat: *mut GdkSeat) -> *mut GdkDevice;
    }
}

pub mod gdkevents_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GdkEventAny {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
    }

    pub type GdkEventType = std::ffi::c_int;

    pub const GDK_EVENT_LAST: GdkEventType = 48;

    pub const GDK_PAD_GROUP_MODE: GdkEventType = 47;

    pub const GDK_PAD_STRIP: GdkEventType = 46;

    pub const GDK_PAD_RING: GdkEventType = 45;

    pub const GDK_PAD_BUTTON_RELEASE: GdkEventType = 44;

    pub const GDK_PAD_BUTTON_PRESS: GdkEventType = 43;

    pub const GDK_TOUCHPAD_PINCH: GdkEventType = 42;

    pub const GDK_TOUCHPAD_SWIPE: GdkEventType = 41;

    pub const GDK_TOUCH_CANCEL: GdkEventType = 40;

    pub const GDK_TOUCH_END: GdkEventType = 39;

    pub const GDK_TOUCH_UPDATE: GdkEventType = 38;

    pub const GDK_TOUCH_BEGIN: GdkEventType = 37;

    pub const GDK_DAMAGE: GdkEventType = 36;

    pub const GDK_GRAB_BROKEN: GdkEventType = 35;

    pub const GDK_OWNER_CHANGE: GdkEventType = 34;

    pub const GDK_SETTING: GdkEventType = 33;

    pub const GDK_WINDOW_STATE: GdkEventType = 32;

    pub const GDK_SCROLL: GdkEventType = 31;

    pub const GDK_VISIBILITY_NOTIFY: GdkEventType = 29;

    pub const GDK_CLIENT_EVENT: GdkEventType = 28;

    pub const GDK_DROP_FINISHED: GdkEventType = 27;

    pub const GDK_DROP_START: GdkEventType = 26;

    pub const GDK_DRAG_STATUS: GdkEventType = 25;

    pub const GDK_DRAG_MOTION: GdkEventType = 24;

    pub const GDK_DRAG_LEAVE: GdkEventType = 23;

    pub const GDK_DRAG_ENTER: GdkEventType = 22;

    pub const GDK_PROXIMITY_OUT: GdkEventType = 21;

    pub const GDK_PROXIMITY_IN: GdkEventType = 20;

    pub const GDK_SELECTION_NOTIFY: GdkEventType = 19;

    pub const GDK_SELECTION_REQUEST: GdkEventType = 18;

    pub const GDK_SELECTION_CLEAR: GdkEventType = 17;

    pub const GDK_PROPERTY_NOTIFY: GdkEventType = 16;

    pub const GDK_UNMAP: GdkEventType = 15;

    pub const GDK_MAP: GdkEventType = 14;

    pub const GDK_CONFIGURE: GdkEventType = 13;

    pub const GDK_FOCUS_CHANGE: GdkEventType = 12;

    pub const GDK_LEAVE_NOTIFY: GdkEventType = 11;

    pub const GDK_ENTER_NOTIFY: GdkEventType = 10;

    pub const GDK_KEY_RELEASE: GdkEventType = 9;

    pub const GDK_KEY_PRESS: GdkEventType = 8;

    pub const GDK_BUTTON_RELEASE: GdkEventType = 7;

    pub const GDK_TRIPLE_BUTTON_PRESS: GdkEventType = 6;

    pub const GDK_3BUTTON_PRESS: GdkEventType = 6;

    pub const GDK_DOUBLE_BUTTON_PRESS: GdkEventType = 5;

    pub const GDK_2BUTTON_PRESS: GdkEventType = 5;

    pub const GDK_BUTTON_PRESS: GdkEventType = 4;

    pub const GDK_MOTION_NOTIFY: GdkEventType = 3;

    pub const GDK_EXPOSE: GdkEventType = 2;

    pub const GDK_DESTROY: GdkEventType = 1;

    pub const GDK_DELETE: GdkEventType = 0;

    pub const GDK_NOTHING: GdkEventType = -1;

    pub type GdkEventAny = _GdkEventAny;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GdkEventExpose {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub area: GdkRectangle,
        pub region: *mut cairo_region_t,
        pub count: gint,
    }

    pub type GdkEventExpose = _GdkEventExpose;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GdkEventVisibility {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub state: GdkVisibilityState,
    }

    pub type GdkVisibilityState = std::ffi::c_uint;

    pub const GDK_VISIBILITY_FULLY_OBSCURED: GdkVisibilityState = 2;

    pub const GDK_VISIBILITY_PARTIAL: GdkVisibilityState = 1;

    pub const GDK_VISIBILITY_UNOBSCURED: GdkVisibilityState = 0;

    pub type GdkEventVisibility = _GdkEventVisibility;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GdkEventMotion {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub time: guint32,
        pub x: gdouble,
        pub y: gdouble,
        pub axes: *mut gdouble,
        pub state: guint,
        pub is_hint: gint16,
        pub device: *mut GdkDevice,
        pub x_root: gdouble,
        pub y_root: gdouble,
    }

    pub type GdkEventMotion = _GdkEventMotion;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GdkEventButton {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub time: guint32,
        pub x: gdouble,
        pub y: gdouble,
        pub axes: *mut gdouble,
        pub state: guint,
        pub button: guint,
        pub device: *mut GdkDevice,
        pub x_root: gdouble,
        pub y_root: gdouble,
    }

    pub type GdkEventButton = _GdkEventButton;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GdkEventTouch {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub time: guint32,
        pub x: gdouble,
        pub y: gdouble,
        pub axes: *mut gdouble,
        pub state: guint,
        pub sequence: *mut GdkEventSequence,
        pub emulating_pointer: gboolean,
        pub device: *mut GdkDevice,
        pub x_root: gdouble,
        pub y_root: gdouble,
    }

    pub type GdkEventSequence = _GdkEventSequence;

    pub type GdkEventTouch = _GdkEventTouch;
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]

    pub struct _GdkEventScroll {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub time: guint32,
        pub x: gdouble,
        pub y: gdouble,
        pub state: guint,
        pub direction: GdkScrollDirection,
        pub device: *mut GdkDevice,
        pub x_root: gdouble,
        pub y_root: gdouble,
        pub delta_x: gdouble,
        pub delta_y: gdouble,
        #[bitfield(name = "is_stop", ty = "guint", bits = "0..=0")]
        pub is_stop: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 7],
    }

    pub type GdkScrollDirection = std::ffi::c_uint;

    pub const GDK_SCROLL_SMOOTH: GdkScrollDirection = 4;

    pub const GDK_SCROLL_RIGHT: GdkScrollDirection = 3;

    pub const GDK_SCROLL_LEFT: GdkScrollDirection = 2;

    pub const GDK_SCROLL_DOWN: GdkScrollDirection = 1;

    pub const GDK_SCROLL_UP: GdkScrollDirection = 0;

    pub type GdkEventScroll = _GdkEventScroll;
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]

    pub struct _GdkEventKey {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub time: guint32,
        pub state: guint,
        pub keyval: guint,
        pub length: gint,
        pub string: *mut gchar,
        pub hardware_keycode: guint16,
        pub group: guint8,
        #[bitfield(name = "is_modifier", ty = "guint", bits = "0..=0")]
        pub is_modifier: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 4],
    }

    pub type GdkEventKey = _GdkEventKey;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GdkEventFocus {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub in_0: gint16,
    }

    pub type GdkEventFocus = _GdkEventFocus;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GdkEventCrossing {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub subwindow: *mut GdkWindow,
        pub time: guint32,
        pub x: gdouble,
        pub y: gdouble,
        pub x_root: gdouble,
        pub y_root: gdouble,
        pub mode: GdkCrossingMode,
        pub detail: GdkNotifyType,
        pub focus: gboolean,
        pub state: guint,
    }

    pub type GdkNotifyType = std::ffi::c_uint;

    pub const GDK_NOTIFY_UNKNOWN: GdkNotifyType = 5;

    pub const GDK_NOTIFY_NONLINEAR_VIRTUAL: GdkNotifyType = 4;

    pub const GDK_NOTIFY_NONLINEAR: GdkNotifyType = 3;

    pub const GDK_NOTIFY_INFERIOR: GdkNotifyType = 2;

    pub const GDK_NOTIFY_VIRTUAL: GdkNotifyType = 1;

    pub const GDK_NOTIFY_ANCESTOR: GdkNotifyType = 0;

    pub type GdkCrossingMode = std::ffi::c_uint;

    pub const GDK_CROSSING_DEVICE_SWITCH: GdkCrossingMode = 8;

    pub const GDK_CROSSING_TOUCH_END: GdkCrossingMode = 7;

    pub const GDK_CROSSING_TOUCH_BEGIN: GdkCrossingMode = 6;

    pub const GDK_CROSSING_STATE_CHANGED: GdkCrossingMode = 5;

    pub const GDK_CROSSING_GTK_UNGRAB: GdkCrossingMode = 4;

    pub const GDK_CROSSING_GTK_GRAB: GdkCrossingMode = 3;

    pub const GDK_CROSSING_UNGRAB: GdkCrossingMode = 2;

    pub const GDK_CROSSING_GRAB: GdkCrossingMode = 1;

    pub const GDK_CROSSING_NORMAL: GdkCrossingMode = 0;

    pub type GdkEventCrossing = _GdkEventCrossing;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GdkEventConfigure {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub x: gint,
        pub y: gint,
        pub width: gint,
        pub height: gint,
    }

    pub type GdkEventConfigure = _GdkEventConfigure;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GdkEventProperty {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub atom: GdkAtom,
        pub time: guint32,
        pub state: guint,
    }

    pub type GdkEventProperty = _GdkEventProperty;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GdkEventSelection {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub selection: GdkAtom,
        pub target: GdkAtom,
        pub property: GdkAtom,
        pub time: guint32,
        pub requestor: *mut GdkWindow,
    }

    pub type GdkEventSelection = _GdkEventSelection;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GdkEventOwnerChange {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub owner: *mut GdkWindow,
        pub reason: GdkOwnerChange,
        pub selection: GdkAtom,
        pub time: guint32,
        pub selection_time: guint32,
    }

    pub type GdkOwnerChange = std::ffi::c_uint;

    pub const GDK_OWNER_CHANGE_CLOSE: GdkOwnerChange = 2;

    pub const GDK_OWNER_CHANGE_DESTROY: GdkOwnerChange = 1;

    pub const GDK_OWNER_CHANGE_NEW_OWNER: GdkOwnerChange = 0;

    pub type GdkEventOwnerChange = _GdkEventOwnerChange;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GdkEventProximity {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub time: guint32,
        pub device: *mut GdkDevice,
    }

    pub type GdkEventProximity = _GdkEventProximity;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GdkEventDND {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub context: *mut GdkDragContext,
        pub time: guint32,
        pub x_root: gshort,
        pub y_root: gshort,
    }

    pub type GdkEventDND = _GdkEventDND;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GdkEventWindowState {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub changed_mask: GdkWindowState,
        pub new_window_state: GdkWindowState,
    }

    pub type GdkWindowState = std::ffi::c_uint;

    pub const GDK_WINDOW_STATE_LEFT_RESIZABLE: GdkWindowState = 65536;

    pub const GDK_WINDOW_STATE_LEFT_TILED: GdkWindowState = 32768;

    pub const GDK_WINDOW_STATE_BOTTOM_RESIZABLE: GdkWindowState = 16384;

    pub const GDK_WINDOW_STATE_BOTTOM_TILED: GdkWindowState = 8192;

    pub const GDK_WINDOW_STATE_RIGHT_RESIZABLE: GdkWindowState = 4096;

    pub const GDK_WINDOW_STATE_RIGHT_TILED: GdkWindowState = 2048;

    pub const GDK_WINDOW_STATE_TOP_RESIZABLE: GdkWindowState = 1024;

    pub const GDK_WINDOW_STATE_TOP_TILED: GdkWindowState = 512;

    pub const GDK_WINDOW_STATE_TILED: GdkWindowState = 256;

    pub const GDK_WINDOW_STATE_FOCUSED: GdkWindowState = 128;

    pub const GDK_WINDOW_STATE_BELOW: GdkWindowState = 64;

    pub const GDK_WINDOW_STATE_ABOVE: GdkWindowState = 32;

    pub const GDK_WINDOW_STATE_FULLSCREEN: GdkWindowState = 16;

    pub const GDK_WINDOW_STATE_STICKY: GdkWindowState = 8;

    pub const GDK_WINDOW_STATE_MAXIMIZED: GdkWindowState = 4;

    pub const GDK_WINDOW_STATE_ICONIFIED: GdkWindowState = 2;

    pub const GDK_WINDOW_STATE_WITHDRAWN: GdkWindowState = 1;

    pub type GdkEventWindowState = _GdkEventWindowState;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GdkEventSetting {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub action: GdkSettingAction,
        pub name: *mut std::ffi::c_char,
    }

    pub type GdkSettingAction = std::ffi::c_uint;

    pub const GDK_SETTING_ACTION_DELETED: GdkSettingAction = 2;

    pub const GDK_SETTING_ACTION_CHANGED: GdkSettingAction = 1;

    pub const GDK_SETTING_ACTION_NEW: GdkSettingAction = 0;

    pub type GdkEventSetting = _GdkEventSetting;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GdkEventGrabBroken {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub keyboard: gboolean,
        pub implicit: gboolean,
        pub grab_window: *mut GdkWindow,
    }

    pub type GdkEventGrabBroken = _GdkEventGrabBroken;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GdkEventTouchpadSwipe {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub phase: gint8,
        pub n_fingers: gint8,
        pub time: guint32,
        pub x: gdouble,
        pub y: gdouble,
        pub dx: gdouble,
        pub dy: gdouble,
        pub x_root: gdouble,
        pub y_root: gdouble,
        pub state: guint,
    }

    pub type GdkEventTouchpadSwipe = _GdkEventTouchpadSwipe;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GdkEventTouchpadPinch {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub phase: gint8,
        pub n_fingers: gint8,
        pub time: guint32,
        pub x: gdouble,
        pub y: gdouble,
        pub dx: gdouble,
        pub dy: gdouble,
        pub angle_delta: gdouble,
        pub scale: gdouble,
        pub x_root: gdouble,
        pub y_root: gdouble,
        pub state: guint,
    }

    pub type GdkEventTouchpadPinch = _GdkEventTouchpadPinch;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GdkEventPadButton {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub time: guint32,
        pub group: guint,
        pub button: guint,
        pub mode: guint,
    }

    pub type GdkEventPadButton = _GdkEventPadButton;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GdkEventPadAxis {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub time: guint32,
        pub group: guint,
        pub index: guint,
        pub mode: guint,
        pub value: gdouble,
    }

    pub type GdkEventPadAxis = _GdkEventPadAxis;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GdkEventPadGroupMode {
        pub type_0: GdkEventType,
        pub window: *mut GdkWindow,
        pub send_event: gint8,
        pub time: guint32,
        pub group: guint,
        pub mode: guint,
    }

    pub type GdkEventPadGroupMode = _GdkEventPadGroupMode;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub union _GdkEvent {
        pub type_0: GdkEventType,
        pub any: GdkEventAny,
        pub expose: GdkEventExpose,
        pub visibility: GdkEventVisibility,
        pub motion: GdkEventMotion,
        pub button: GdkEventButton,
        pub touch: GdkEventTouch,
        pub scroll: GdkEventScroll,
        pub key: GdkEventKey,
        pub crossing: GdkEventCrossing,
        pub focus_change: GdkEventFocus,
        pub configure: GdkEventConfigure,
        pub property: GdkEventProperty,
        pub selection: GdkEventSelection,
        pub owner_change: GdkEventOwnerChange,
        pub proximity: GdkEventProximity,
        pub dnd: GdkEventDND,
        pub window_state: GdkEventWindowState,
        pub setting: GdkEventSetting,
        pub grab_broken: GdkEventGrabBroken,
        pub touchpad_swipe: GdkEventTouchpadSwipe,
        pub touchpad_pinch: GdkEventTouchpadPinch,
        pub pad_button: GdkEventPadButton,
        pub pad_axis: GdkEventPadAxis,
        pub pad_group_mode: GdkEventPadGroupMode,
    }

    pub type GdkEvent = _GdkEvent;
    use super::gdktypes_h::{GdkWindow, GdkRectangle, GdkDevice, GdkAtom, GdkDragContext};
    use super::glibconfig_h::{gint8, guint32, gint16, guint16, guint8};
    use super::cairo_h::cairo_region_t;
    use super::gtypes_h::{gint, gdouble, guint, gboolean, gchar, gshort};
    extern "C" {

        pub type _GdkEventSequence;

        pub fn gdk_event_new(type_0: GdkEventType) -> *mut GdkEvent;

        pub fn gdk_event_get_scroll_deltas(
            event: *const GdkEvent,
            delta_x: *mut gdouble,
            delta_y: *mut gdouble,
        ) -> gboolean;

        pub fn gdk_event_set_device(event: *mut GdkEvent, device: *mut GdkDevice);
    }
}

pub mod gdkkeys_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GdkKeymapKey {
        pub keycode: guint,
        pub group: gint,
        pub level: gint,
    }

    pub type GdkKeymapKey = _GdkKeymapKey;
    use super::gtypes_h::{guint, gint, gboolean, gchar};
    use super::gdktypes_h::{GdkDisplay, GdkKeymap};
    use super::glibconfig_h::guint32;
    extern "C" {

        pub fn gdk_keymap_get_for_display(display: *mut GdkDisplay) -> *mut GdkKeymap;

        pub fn gdk_keymap_get_entries_for_keyval(
            keymap: *mut GdkKeymap,
            keyval: guint,
            keys: *mut *mut GdkKeymapKey,
            n_keys: *mut gint,
        ) -> gboolean;

        pub fn gdk_keyval_from_name(keyval_name: *const gchar) -> guint;

        pub fn gdk_unicode_to_keyval(wc: guint32) -> guint;
    }
}

pub mod gtkenums_h {

    pub type GtkAlign = std::ffi::c_uint;

    pub const GTK_ALIGN_BASELINE: GtkAlign = 4;

    pub const GTK_ALIGN_CENTER: GtkAlign = 3;

    pub const GTK_ALIGN_END: GtkAlign = 2;

    pub const GTK_ALIGN_START: GtkAlign = 1;

    pub const GTK_ALIGN_FILL: GtkAlign = 0;
}

pub mod gtkwidget_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GtkWidget {
        pub parent_instance: GInitiallyUnowned,
        pub priv_0: *mut GtkWidgetPrivate,
    }

    pub type GtkWidgetPrivate = _GtkWidgetPrivate;
    use super::gobject_h::GInitiallyUnowned;
    use super::gtype_h::GType;
    use super::gtktypes_h::GtkWidget;
    use super::gtypes_h::{gboolean, gint, gchar};
    use super::gdktypes_h::GdkWindow;
    use super::gtkenums_h::{GtkAlign, GTK_ALIGN_FILL};
    extern "C" {

        pub type _GtkWidgetPrivate;

        pub fn gtk_widget_get_type() -> GType;

        pub fn gtk_widget_destroy(widget: *mut GtkWidget);

        pub fn gtk_widget_show(widget: *mut GtkWidget);

        pub fn gtk_widget_hide(widget: *mut GtkWidget);

        pub fn gtk_widget_is_focus(widget: *mut GtkWidget) -> gboolean;

        pub fn gtk_widget_grab_focus(widget: *mut GtkWidget);

        pub fn gtk_widget_set_visible(widget: *mut GtkWidget, visible: gboolean);

        pub fn gtk_widget_get_visible(widget: *mut GtkWidget) -> gboolean;

        pub fn gtk_widget_get_parent(widget: *mut GtkWidget) -> *mut GtkWidget;

        pub fn gtk_widget_get_window(widget: *mut GtkWidget) -> *mut GdkWindow;

        pub fn gtk_widget_get_allocated_width(widget: *mut GtkWidget) -> std::ffi::c_int;

        pub fn gtk_widget_get_allocated_height(
            widget: *mut GtkWidget,
        ) -> std::ffi::c_int;

        pub fn gtk_widget_set_size_request(
            widget: *mut GtkWidget,
            width: gint,
            height: gint,
        );

        pub fn gtk_widget_get_size_request(
            widget: *mut GtkWidget,
            width: *mut gint,
            height: *mut gint,
        );

        pub fn gtk_widget_get_halign(widget: *mut GtkWidget) -> GtkAlign;

        pub fn gtk_widget_set_halign(widget: *mut GtkWidget, align: GtkAlign);

        pub fn gtk_widget_get_valign_with_baseline(widget: *mut GtkWidget) -> GtkAlign;

        pub fn gtk_widget_set_valign(widget: *mut GtkWidget, align: GtkAlign);

        pub fn gtk_widget_set_tooltip_markup(
            widget: *mut GtkWidget,
            markup: *const gchar,
        );

        pub fn gtk_widget_get_tooltip_markup(widget: *mut GtkWidget) -> *mut gchar;
    }
}

pub mod gtktypes_h {

    pub type GtkWidget = _GtkWidget;

    pub type GtkWindow = _GtkWindow;
    use super::gtkwidget_h::_GtkWidget;
    use super::gtkwindow_h::_GtkWindow;
}

pub mod gtkwindow_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GtkWindow {
        pub bin: GtkBin,
        pub priv_0: *mut GtkWindowPrivate,
    }

    pub type GtkWindowPrivate = _GtkWindowPrivate;
    use super::gtkbin_h::GtkBin;
    use super::gtype_h::GType;
    use super::gtktypes_h::{GtkWindow, GtkWidget};
    use super::gtypes_h::gboolean;
    extern "C" {

        pub type _GtkWindowPrivate;

        pub fn gtk_window_get_type() -> GType;

        pub fn gtk_window_set_focus(window: *mut GtkWindow, focus: *mut GtkWidget);

        pub fn gtk_window_has_toplevel_focus(window: *mut GtkWindow) -> gboolean;
    }
}

pub mod gtkbin_h {

    pub type GtkBin = _GtkBin;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GtkBin {
        pub container: GtkContainer,
        pub priv_0: *mut GtkBinPrivate,
    }

    pub type GtkBinPrivate = _GtkBinPrivate;
    use super::gtkcontainer_h::GtkContainer;
    use super::gtype_h::GType;
    use super::gtktypes_h::GtkWidget;
    extern "C" {

        pub type _GtkBinPrivate;

        pub fn gtk_bin_get_type() -> GType;

        pub fn gtk_bin_get_child(bin: *mut GtkBin) -> *mut GtkWidget;
    }
}

pub mod gtkcontainer_h {

    pub type GtkContainer = _GtkContainer;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GtkContainer {
        pub widget: GtkWidget,
        pub priv_0: *mut GtkContainerPrivate,
    }

    pub type GtkContainerPrivate = _GtkContainerPrivate;
    use super::gtktypes_h::GtkWidget;
    use super::gtype_h::{GType, GValue};
    use super::glist_h::GList;
    use super::gobject_h::GObjectClass;
    use super::gtypes_h::{guint, gchar};
    use super::gparam_h::GParamSpec;
    extern "C" {

        pub type _GtkContainerPrivate;

        pub fn gtk_container_get_type() -> GType;

        pub fn gtk_container_add(container: *mut GtkContainer, widget: *mut GtkWidget);

        pub fn gtk_container_remove(
            container: *mut GtkContainer,
            widget: *mut GtkWidget,
        );

        pub fn gtk_container_get_children(container: *mut GtkContainer) -> *mut GList;

        pub fn gtk_container_class_list_child_properties(
            cclass: *mut GObjectClass,
            n_properties: *mut guint,
        ) -> *mut *mut GParamSpec;

        pub fn gtk_container_child_set_property(
            container: *mut GtkContainer,
            child: *mut GtkWidget,
            property_name: *const gchar,
            value: *const GValue,
        );

        pub fn gtk_container_child_get_property(
            container: *mut GtkContainer,
            child: *mut GtkWidget,
            property_name: *const gchar,
            value: *mut GValue,
        );
    }
}

pub mod gtkentry_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GtkEntry {
        pub parent_instance: GtkWidget,
        pub priv_0: *mut GtkEntryPrivate,
    }

    pub type GtkEntryPrivate = _GtkEntryPrivate;

    pub type GtkEntry = _GtkEntry;
    use super::gtktypes_h::GtkWidget;
    use super::gtype_h::GType;
    extern "C" {

        pub type _GtkEntryPrivate;

        pub fn gtk_entry_get_type() -> GType;

        pub fn gtk_entry_grab_focus_without_selecting(entry: *mut GtkEntry);
    }
}

pub mod gtkcssprovider_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _GtkCssProvider {
        pub parent_instance: GObject,
        pub priv_0: *mut GtkCssProviderPrivate,
    }

    pub type GtkCssProviderPrivate = _GtkCssProviderPrivate;

    pub type GtkCssProvider = _GtkCssProvider;
    use super::gobject_h::GObject;
    extern "C" {

        pub type _GtkCssProviderPrivate;
    }
}

pub mod lua_h {

    pub type lua_Number = std::ffi::c_double;

    pub type lua_Integer = ptrdiff_t;

    pub const LUA_REGISTRYINDEX: std::ffi::c_int = -(10000 as std::ffi::c_int);

    pub const LUA_TNIL: std::ffi::c_int = 0 as std::ffi::c_int;

    pub const LUA_TBOOLEAN: std::ffi::c_int = 1 as std::ffi::c_int;

    pub const LUA_TTABLE: std::ffi::c_int = 5 as std::ffi::c_int;
    use super::__stddef_ptrdiff_t_h::ptrdiff_t;
    use super::__stddef_size_t_h::size_t;
    extern "C" {

        pub type lua_State;

        pub fn lua_gettop(L: *mut lua_State) -> std::ffi::c_int;

        pub fn lua_settop(L: *mut lua_State, idx: std::ffi::c_int);

        pub fn lua_remove(L: *mut lua_State, idx: std::ffi::c_int);

        pub fn lua_insert(L: *mut lua_State, idx: std::ffi::c_int);

        pub fn lua_type(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;

        pub fn lua_tonumber(L: *mut lua_State, idx: std::ffi::c_int) -> lua_Number;

        pub fn lua_toboolean(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;

        pub fn lua_tolstring(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            len: *mut size_t,
        ) -> *const std::ffi::c_char;

        pub fn lua_pushnil(L: *mut lua_State);

        pub fn lua_pushnumber(L: *mut lua_State, n: lua_Number);

        pub fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);

        pub fn lua_pushlstring(L: *mut lua_State, s: *const std::ffi::c_char, l: size_t);

        pub fn lua_pushstring(L: *mut lua_State, s: *const std::ffi::c_char);

        pub fn lua_pushboolean(L: *mut lua_State, b: std::ffi::c_int);

        pub fn lua_pushlightuserdata(L: *mut lua_State, p: *mut std::ffi::c_void);

        pub fn lua_rawget(L: *mut lua_State, idx: std::ffi::c_int);

        pub fn lua_createtable(
            L: *mut lua_State,
            narr: std::ffi::c_int,
            nrec: std::ffi::c_int,
        );

        pub fn lua_rawset(L: *mut lua_State, idx: std::ffi::c_int);

        pub fn lua_rawseti(L: *mut lua_State, idx: std::ffi::c_int, n: std::ffi::c_int);

        pub fn lua_next(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;
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
    extern "C" {

        pub fn _log(lvl: log_level_t, _: *const gchar, _: *const gchar, _: ...);
    }
}

pub mod signal_h {

    pub type signal_t = GTree;
    use super::gtree_h::GTree;
}

pub mod tokenize_h {

    pub type luakit_token_t = std::ffi::c_uint;

    pub const L_TK_ZOOM_TEXT_ONLY: luakit_token_t = 274;

    pub const L_TK_ZOOM_LEVEL: luakit_token_t = 273;

    pub const L_TK_YPAGE_SIZE: luakit_token_t = 272;

    pub const L_TK_YMAX: luakit_token_t = 271;

    pub const L_TK_Y: luakit_token_t = 270;

    pub const L_TK_XPAGE_SIZE: luakit_token_t = 269;

    pub const L_TK_XMAX: luakit_token_t = 268;

    pub const L_TK_X: luakit_token_t = 267;

    pub const L_TK_WRAP_JS: luakit_token_t = 266;

    pub const L_TK_WIN_XID: luakit_token_t = 265;

    pub const L_TK_WINDOWS: luakit_token_t = 264;

    pub const L_TK_WINDOW: luakit_token_t = 263;

    pub const L_TK_WIDTH: luakit_token_t = 262;

    pub const L_TK_WEB_PROCESS_ID: luakit_token_t = 261;

    pub const L_TK_WEBVIEW: luakit_token_t = 260;

    pub const L_TK_WEBSITE_DATA: luakit_token_t = 259;

    pub const L_TK_WEBKIT_VERSION: luakit_token_t = 258;

    pub const L_TK_WEBKIT_USER_AGENT_VERSION: luakit_token_t = 257;

    pub const L_TK_WEBKIT2: luakit_token_t = 256;

    pub const L_TK_VPANED: luakit_token_t = 255;

    pub const L_TK_VISIBLE_CHILD: luakit_token_t = 254;

    pub const L_TK_VISIBLE: luakit_token_t = 253;

    pub const L_TK_VIDEOS_DIR: luakit_token_t = 252;

    pub const L_TK_VERSION: luakit_token_t = 251;

    pub const L_TK_VERBOSE: luakit_token_t = 250;

    pub const L_TK_VBOX: luakit_token_t = 249;

    pub const L_TK_VALUE: luakit_token_t = 248;

    pub const L_TK_USER_AGENT: luakit_token_t = 247;

    pub const L_TK_URI: luakit_token_t = 246;

    pub const L_TK_URGENCY_HINT: luakit_token_t = 245;

    pub const L_TK_TYPE: luakit_token_t = 244;

    pub const L_TK_TOTAL_SIZE: luakit_token_t = 243;

    pub const L_TK_TOP: luakit_token_t = 242;

    pub const L_TK_TOOLTIP: luakit_token_t = 241;

    pub const L_TK_TITLE: luakit_token_t = 240;

    pub const L_TK_TEXT_CONTENT: luakit_token_t = 239;

    pub const L_TK_TEXTWIDTH: luakit_token_t = 238;

    pub const L_TK_TEXT: luakit_token_t = 237;

    pub const L_TK_TEMPLATES_DIR: luakit_token_t = 236;

    pub const L_TK_TAG_NAME: luakit_token_t = 235;

    pub const L_TK_SYSTEM_DATA_DIRS: luakit_token_t = 234;

    pub const L_TK_SYSTEM_CONFIG_DIRS: luakit_token_t = 233;

    pub const L_TK_SWITCH: luakit_token_t = 232;

    pub const L_TK_SUGGESTED_FILENAME: luakit_token_t = 231;

    pub const L_TK_SUBMIT: luakit_token_t = 230;

    pub const L_TK_STYLESHEETS: luakit_token_t = 229;

    pub const L_TK_STYLE: luakit_token_t = 228;

    pub const L_TK_STOP: luakit_token_t = 227;

    pub const L_TK_STATUS: luakit_token_t = 226;

    pub const L_TK_STARTED: luakit_token_t = 225;

    pub const L_TK_START: luakit_token_t = 224;

    pub const L_TK_STACK: luakit_token_t = 223;

    pub const L_TK_SSL_TRUSTED: luakit_token_t = 222;

    pub const L_TK_SRC: luakit_token_t = 221;

    pub const L_TK_SPINNER: luakit_token_t = 220;

    pub const L_TK_SPELL_CHECKING_LANGUAGES: luakit_token_t = 219;

    pub const L_TK_SPACING: luakit_token_t = 218;

    pub const L_TK_SOURCE: luakit_token_t = 217;

    pub const L_TK_SOCKET: luakit_token_t = 216;

    pub const L_TK_SHOW_TABS: luakit_token_t = 215;

    pub const L_TK_SHOW_INSPECTOR: luakit_token_t = 214;

    pub const L_TK_SHOW_FRAME: luakit_token_t = 213;

    pub const L_TK_SHOW_BORDER: luakit_token_t = 212;

    pub const L_TK_SHOW: luakit_token_t = 211;

    pub const L_TK_SET_TITLE: luakit_token_t = 210;

    pub const L_TK_SET_PDFJS: luakit_token_t = 209;

    pub const L_TK_SET_FAVICON_FOR_URI: luakit_token_t = 208;

    pub const L_TK_SET_DEFAULT_SIZE: luakit_token_t = 207;

    pub const L_TK_SET_DARK_MODE: luakit_token_t = 206;

    pub const L_TK_SESSION_STATE: luakit_token_t = 205;

    pub const L_TK_SERIF_FONT_FAMILY: luakit_token_t = 204;

    pub const L_TK_SEND_KEY: luakit_token_t = 203;

    pub const L_TK_SELECT_REGION: luakit_token_t = 202;

    pub const L_TK_SELECTION: luakit_token_t = 201;

    pub const L_TK_SELECTABLE: luakit_token_t = 200;

    pub const L_TK_SECONDARY: luakit_token_t = 199;

    pub const L_TK_SEARCH_PREVIOUS: luakit_token_t = 198;

    pub const L_TK_SEARCH_NEXT: luakit_token_t = 197;

    pub const L_TK_SEARCH: luakit_token_t = 196;

    pub const L_TK_SCROLL_Y: luakit_token_t = 195;

    pub const L_TK_SCROLL_X: luakit_token_t = 194;

    pub const L_TK_SCROLLED: luakit_token_t = 193;

    pub const L_TK_SCROLLBARS: luakit_token_t = 192;

    pub const L_TK_SCROLL: luakit_token_t = 191;

    pub const L_TK_SCREEN: luakit_token_t = 190;

    pub const L_TK_SCALE: luakit_token_t = 189;

    pub const L_TK_SAVE: luakit_token_t = 188;

    pub const L_TK_SANS_SERIF_FONT_FAMILY: luakit_token_t = 187;

    pub const L_TK_ROOT_WIN_XID: luakit_token_t = 186;

    pub const L_TK_RIGHT: luakit_token_t = 185;

    pub const L_TK_RESOURCE_PATH: luakit_token_t = 184;

    pub const L_TK_REPLACE: luakit_token_t = 183;

    pub const L_TK_REORDER: luakit_token_t = 182;

    pub const L_TK_REMOVE_EVENT_LISTENER: luakit_token_t = 181;

    pub const L_TK_REMOVE: luakit_token_t = 180;

    pub const L_TK_RELOAD_BYPASS_CACHE: luakit_token_t = 179;

    pub const L_TK_RELOAD: luakit_token_t = 178;

    pub const L_TK_RECT: luakit_token_t = 177;

    pub const L_TK_QUERY: luakit_token_t = 176;

    pub const L_TK_PUBLIC_SHARE_DIR: luakit_token_t = 175;

    pub const L_TK_PROXY_URI: luakit_token_t = 174;

    pub const L_TK_PROGRESS: luakit_token_t = 173;

    pub const L_TK_PROCESS_LIMIT: luakit_token_t = 172;

    pub const L_TK_PRIVATE: luakit_token_t = 171;

    pub const L_TK_PRINT_BACKGROUNDS: luakit_token_t = 170;

    pub const L_TK_PRIMARY: luakit_token_t = 169;

    pub const L_TK_PREV_SIBLING: luakit_token_t = 168;

    pub const L_TK_POSITION: luakit_token_t = 167;

    pub const L_TK_PLUGGED: luakit_token_t = 166;

    pub const L_TK_PICTURES_DIR: luakit_token_t = 165;

    pub const L_TK_PICTOGRAPH_FONT_FAMILY: luakit_token_t = 164;

    pub const L_TK_PATTERN: luakit_token_t = 163;

    pub const L_TK_PARENT: luakit_token_t = 162;

    pub const L_TK_PACK2: luakit_token_t = 161;

    pub const L_TK_PACK1: luakit_token_t = 160;

    pub const L_TK_PACK: luakit_token_t = 159;

    pub const L_TK_OWNER_DOCUMENT: luakit_token_t = 158;

    pub const L_TK_OVERLAY: luakit_token_t = 157;

    pub const L_TK_OPTIONS: luakit_token_t = 156;

    pub const L_TK_NOUNIQUE: luakit_token_t = 155;

    pub const L_TK_NOTEBOOK: luakit_token_t = 154;

    pub const L_TK_NEXT_SIBLING: luakit_token_t = 153;

    pub const L_TK_NAME: luakit_token_t = 152;

    pub const L_TK_MUSIC_DIR: luakit_token_t = 151;

    pub const L_TK_MONOSPACE_FONT_FAMILY: luakit_token_t = 150;

    pub const L_TK_MIN_SIZE: luakit_token_t = 149;

    pub const L_TK_MINIMUM_FONT_SIZE: luakit_token_t = 148;

    pub const L_TK_MIME_TYPE: luakit_token_t = 147;

    pub const L_TK_MEDIA_PLAYBACK_REQUIRES_GESTURE: luakit_token_t = 146;

    pub const L_TK_MEDIA_PLAYBACK_ALLOWS_INLINE: luakit_token_t = 145;

    pub const L_TK_MAXIMIZED: luakit_token_t = 144;

    pub const L_TK_MARGIN_TOP: luakit_token_t = 143;

    pub const L_TK_MARGIN_RIGHT: luakit_token_t = 142;

    pub const L_TK_MARGIN_LEFT: luakit_token_t = 141;

    pub const L_TK_MARGIN_BOTTOM: luakit_token_t = 140;

    pub const L_TK_MARGIN: luakit_token_t = 139;

    pub const L_TK_LOAD_STRING: luakit_token_t = 138;

    pub const L_TK_LOADING: luakit_token_t = 137;

    pub const L_TK_LEFT: luakit_token_t = 136;

    pub const L_TK_LAST_CHILD: luakit_token_t = 135;

    pub const L_TK_LABEL: luakit_token_t = 134;

    pub const L_TK_JAVASCRIPT_CAN_OPEN_WINDOWS_AUTOMATICALLY: luakit_token_t = 133;

    pub const L_TK_JAVASCRIPT_CAN_ACCESS_CLIPBOARD: luakit_token_t = 132;

    pub const L_TK_IS_PLAYING_AUDIO: luakit_token_t = 131;

    pub const L_TK_IS_LOADING: luakit_token_t = 130;

    pub const L_TK_IS_ALIVE: luakit_token_t = 129;

    pub const L_TK_INVALIDATE: luakit_token_t = 128;

    pub const L_TK_INTERVAL: luakit_token_t = 127;

    pub const L_TK_INSTALL_PATHS: luakit_token_t = 126;

    pub const L_TK_INSTALL_PATH: luakit_token_t = 125;

    pub const L_TK_INSPECTOR: luakit_token_t = 124;

    pub const L_TK_INSERT: luakit_token_t = 123;

    pub const L_TK_INNER_WIDTH: luakit_token_t = 122;

    pub const L_TK_INNER_HTML: luakit_token_t = 121;

    pub const L_TK_INNER_HEIGHT: luakit_token_t = 120;

    pub const L_TK_INDEXOF: luakit_token_t = 119;

    pub const L_TK_IMAGE: luakit_token_t = 118;

    pub const L_TK_ID: luakit_token_t = 117;

    pub const L_TK_ICON: luakit_token_t = 116;

    pub const L_TK_HREF: luakit_token_t = 115;

    pub const L_TK_HPANED: luakit_token_t = 114;

    pub const L_TK_HOVERED_URI: luakit_token_t = 113;

    pub const L_TK_HOMOGENEOUS: luakit_token_t = 112;

    pub const L_TK_HISTORY: luakit_token_t = 111;

    pub const L_TK_HIDE: luakit_token_t = 110;

    pub const L_TK_HEIGHT: luakit_token_t = 109;

    pub const L_TK_HBOX: luakit_token_t = 108;

    pub const L_TK_HARDWARE_ACCELERATION_POLICY: luakit_token_t = 107;

    pub const L_TK_GO_FORWARD: luakit_token_t = 106;

    pub const L_TK_GO_BACK: luakit_token_t = 105;

    pub const L_TK_GET_TITLE: luakit_token_t = 104;

    pub const L_TK_GET_SOURCE: luakit_token_t = 103;

    pub const L_TK_FULLSCREEN: luakit_token_t = 102;

    pub const L_TK_FONT: luakit_token_t = 101;

    pub const L_TK_FOCUSED: luakit_token_t = 100;

    pub const L_TK_FOCUS: luakit_token_t = 99;

    pub const L_TK_FIRST_CHILD: luakit_token_t = 98;

    pub const L_TK_FINISHED: luakit_token_t = 97;

    pub const L_TK_FILL: luakit_token_t = 96;

    pub const L_TK_FILENAME: luakit_token_t = 95;

    pub const L_TK_FG: luakit_token_t = 94;

    pub const L_TK_FETCH: luakit_token_t = 93;

    pub const L_TK_FANTASY_FONT_FAMILY: luakit_token_t = 92;

    pub const L_TK_EXECPATH: luakit_token_t = 91;

    pub const L_TK_EVENTBOX: luakit_token_t = 90;

    pub const L_TK_EVAL_JS: luakit_token_t = 89;

    pub const L_TK_ERROR: luakit_token_t = 88;

    pub const L_TK_ENTRY: luakit_token_t = 87;

    pub const L_TK_END: luakit_token_t = 86;

    pub const L_TK_ENABLE_XSS_AUDITOR: luakit_token_t = 85;

    pub const L_TK_ENABLE_WRITE_CONSOLE_MESSAGES_TO_STDOUT: luakit_token_t = 84;

    pub const L_TK_ENABLE_WEBGL: luakit_token_t = 83;

    pub const L_TK_ENABLE_WEBAUDIO: luakit_token_t = 82;

    pub const L_TK_ENABLE_TABS_TO_LINKS: luakit_token_t = 81;

    pub const L_TK_ENABLE_SPELL_CHECKING: luakit_token_t = 80;

    pub const L_TK_ENABLE_SPATIAL_NAVIGATION: luakit_token_t = 79;

    pub const L_TK_ENABLE_SMOOTH_SCROLLING: luakit_token_t = 78;

    pub const L_TK_ENABLE_SITE_SPECIFIC_QUIRKS: luakit_token_t = 77;

    pub const L_TK_ENABLE_SCRIPTS: luakit_token_t = 76;

    pub const L_TK_ENABLE_RESIZABLE_TEXT_AREAS: luakit_token_t = 75;

    pub const L_TK_ENABLE_PLUGINS: luakit_token_t = 74;

    pub const L_TK_ENABLE_PAGE_CACHE: luakit_token_t = 73;

    pub const L_TK_ENABLE_MEDIA_STREAM: luakit_token_t = 72;

    pub const L_TK_ENABLE_MEDIASOURCE: luakit_token_t = 71;

    pub const L_TK_ENABLE_JAVASCRIPT: luakit_token_t = 70;

    pub const L_TK_ENABLE_JAVA: luakit_token_t = 69;

    pub const L_TK_ENABLE_HYPERLINK_AUDITING: luakit_token_t = 68;

    pub const L_TK_ENABLE_HTML5_LOCAL_STORAGE: luakit_token_t = 67;

    pub const L_TK_ENABLE_HTML5_DATABASE: luakit_token_t = 66;

    pub const L_TK_ENABLE_FULLSCREEN: luakit_token_t = 65;

    pub const L_TK_ENABLE_FRAME_FLATTENING: luakit_token_t = 64;

    pub const L_TK_ENABLE_DNS_PREFETCHING: luakit_token_t = 63;

    pub const L_TK_ENABLE_DEVELOPER_EXTRAS: luakit_token_t = 62;

    pub const L_TK_ENABLE_CARET_BROWSING: luakit_token_t = 61;

    pub const L_TK_ENABLE_ACCELERATED_2D_CANVAS: luakit_token_t = 60;

    pub const L_TK_ELEMENT_FROM_POINT: luakit_token_t = 59;

    pub const L_TK_ELAPSED_TIME: luakit_token_t = 58;

    pub const L_TK_EDITABLE: luakit_token_t = 57;

    pub const L_TK_DRAW_COMPOSITING_INDICATORS: luakit_token_t = 56;

    pub const L_TK_DRAWING_AREA: luakit_token_t = 55;

    pub const L_TK_DOWNLOAD_DIR: luakit_token_t = 54;

    pub const L_TK_DOCUMENTS_DIR: luakit_token_t = 53;

    pub const L_TK_DOCUMENT: luakit_token_t = 52;

    pub const L_TK_DEV_PATHS: luakit_token_t = 51;

    pub const L_TK_DESTROY: luakit_token_t = 50;

    pub const L_TK_DESTINATION: luakit_token_t = 49;

    pub const L_TK_DESKTOP_DIR: luakit_token_t = 48;

    pub const L_TK_DEFAULT_MONOSPACE_FONT_SIZE: luakit_token_t = 47;

    pub const L_TK_DEFAULT_FONT_SIZE: luakit_token_t = 46;

    pub const L_TK_DEFAULT_FONT_FAMILY: luakit_token_t = 45;

    pub const L_TK_DEFAULT_CHARSET: luakit_token_t = 44;

    pub const L_TK_DECORATED: luakit_token_t = 43;

    pub const L_TK_DATA_DIR: luakit_token_t = 42;

    pub const L_TK_CURSIVE_FONT_FAMILY: luakit_token_t = 41;

    pub const L_TK_CURRENT_SIZE: luakit_token_t = 40;

    pub const L_TK_CURRENT: luakit_token_t = 39;

    pub const L_TK_CSS: luakit_token_t = 38;

    pub const L_TK_CREATE_ELEMENT: luakit_token_t = 37;

    pub const L_TK_CRASH: luakit_token_t = 36;

    pub const L_TK_COUNT: luakit_token_t = 35;

    pub const L_TK_COOKIES_STORAGE: luakit_token_t = 34;

    pub const L_TK_CONFPATH: luakit_token_t = 33;

    pub const L_TK_CONFIG_DIR: luakit_token_t = 32;

    pub const L_TK_CLOSE_INSPECTOR: luakit_token_t = 31;

    pub const L_TK_CLIPBOARD: luakit_token_t = 30;

    pub const L_TK_CLIENT_RECTS: luakit_token_t = 29;

    pub const L_TK_CLICK: luakit_token_t = 28;

    pub const L_TK_CLEAR_SEARCH: luakit_token_t = 27;

    pub const L_TK_CLEAR: luakit_token_t = 26;

    pub const L_TK_CHILD_COUNT: luakit_token_t = 25;

    pub const L_TK_CHILDREN: luakit_token_t = 24;

    pub const L_TK_CHILD: luakit_token_t = 23;

    pub const L_TK_CHECKED: luakit_token_t = 22;

    pub const L_TK_CERTIFICATE: luakit_token_t = 21;

    pub const L_TK_CENTER: luakit_token_t = 20;

    pub const L_TK_CAN_GO_FORWARD: luakit_token_t = 19;

    pub const L_TK_CAN_GO_BACK: luakit_token_t = 18;

    pub const L_TK_CAN_FOCUS: luakit_token_t = 17;

    pub const L_TK_CACHE_DIR: luakit_token_t = 16;

    pub const L_TK_BOTTOM: luakit_token_t = 15;

    pub const L_TK_BODY: luakit_token_t = 14;

    pub const L_TK_BG: luakit_token_t = 13;

    pub const L_TK_BASELINE: luakit_token_t = 12;

    pub const L_TK_AUTO_LOAD_IMAGES: luakit_token_t = 11;

    pub const L_TK_ATTR: luakit_token_t = 10;

    pub const L_TK_APPEND: luakit_token_t = 9;

    pub const L_TK_ALLOW_UNIVERSAL_ACCESS_FROM_FILE_URLS: luakit_token_t = 8;

    pub const L_TK_ALLOW_OVERWRITE: luakit_token_t = 7;

    pub const L_TK_ALLOW_MODAL_DIALOGS: luakit_token_t = 6;

    pub const L_TK_ALLOW_FILE_ACCESS_FROM_FILE_URLS: luakit_token_t = 5;

    pub const L_TK_ALLOW_CERTIFICATE: luakit_token_t = 4;

    pub const L_TK_ALIGN: luakit_token_t = 3;

    pub const L_TK_ADD_EVENT_LISTENER: luakit_token_t = 2;

    pub const L_TK_ACCEPT_POLICY: luakit_token_t = 1;

    pub const L_TK_UNKNOWN: luakit_token_t = 0;
    use super::gtypes_h::gchar;
    extern "C" {

        pub fn l_tokenize(_: *const gchar) -> luakit_token_t;
    }
}

pub mod luaclass_h {

    pub type lua_class_property_array_t = GHashTable;
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct lua_object_t {
        pub signals: *mut signal_t,
    }

    pub type lua_class_allocator_t = Option::<
        unsafe extern "C" fn(*mut lua_State) -> *mut lua_object_t,
    >;

    pub type lua_class_propfunc_t = Option::<
        unsafe extern "C" fn(*mut lua_State, *mut lua_object_t) -> gint,
    >;
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
    use super::ghash_h::GHashTable;
    use super::signal_h::signal_t;
    use super::lua_h::lua_State;
    use super::gtypes_h::{gint, gchar, gpointer};
    extern "C" {

        pub fn luaH_checkudata(
            _: *mut lua_State,
            _: gint,
            _: *mut lua_class_t,
        ) -> gpointer;
    }
}

pub mod common_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct _common_t {
        pub L: *mut lua_State,
    }

    pub type common_t = _common_t;
    use super::lua_h::lua_State;
    extern "C" {

        pub static mut common: common_t;
    }
}

pub mod widget_h {
    #[derive(Copy, Clone)]
    #[repr(C)]

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

    pub type widget_destructor_t = unsafe extern "C" fn(*mut widget_t) -> ();
    #[derive(Copy, Clone)]
    #[repr(C)]

    pub struct widget_info_t {
        pub tok: luakit_token_t,
        pub name: *const gchar,
        pub wc: Option::<widget_constructor_t>,
    }

    pub type widget_constructor_t = unsafe extern "C" fn(
        *mut lua_State,
        *mut widget_t,
        luakit_token_t,
    ) -> *mut widget_t;

    pub const GOBJECT_LUAKIT_WIDGET_DATA_KEY: [std::ffi::c_char; 19] = unsafe {
        *::core::mem::transmute::<
            &[u8; 19],
            &[std::ffi::c_char; 19],
        >(b"luakit_widget_data\0")
    };
    #[inline]

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
    #[inline]

    pub unsafe extern "C" fn luaH_checkwidgetornil(
        mut L: *mut lua_State,
        mut udx: gint,
    ) -> *mut widget_t {
        if lua_type(L, udx) == LUA_TNIL {
            return NULL as *mut widget_t;
        }
        return luaH_checkwidget(L, udx);
    }
    use super::signal_h::signal_t;
    use super::gtypes_h::{gint, gpointer, gchar, gboolean};
    use super::lua_h::{lua_State, lua_type, LUA_TNIL};
    use super::tokenize_h::luakit_token_t;
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
    use super::__stddef_null_h::NULL;
    extern "C" {

        pub static mut widget_class: lua_class_t;
    }
}

pub mod string_h {
    extern "C" {

        pub fn strcmp(
            _: *const std::ffi::c_char,
            _: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
    }
}

pub mod gmem_h {
    use super::gtypes_h::gpointer;
    use super::glibconfig_h::gsize;
    extern "C" {

        pub fn g_free(mem: gpointer);

        pub fn g_malloc0_n(n_blocks: gsize, n_block_bytes: gsize) -> gpointer;
    }
}

pub mod gmessages_h {

    pub const G_LOG_DOMAIN: std::ffi::c_int = 0 as std::ffi::c_int;
}

pub mod gtestutils_h {
    extern "C" {

        pub fn g_assertion_message_expr(
            domain: *const std::ffi::c_char,
            file: *const std::ffi::c_char,
            line: std::ffi::c_int,
            func: *const std::ffi::c_char,
            expr: *const std::ffi::c_char,
        ) -> !;
    }
}

pub mod gsignal_h {
    use super::gtypes_h::{gpointer, gchar};
    extern "C" {

        pub fn g_signal_emit_by_name(
            instance: gpointer,
            detailed_signal: *const gchar,
            _: ...
        );
    }
}

pub mod gdkwindow_h {
    use super::gdktypes_h::{GdkWindow, GdkEventMask};
    extern "C" {

        pub fn gdk_window_set_events(window: *mut GdkWindow, event_mask: GdkEventMask);
    }
}

pub mod gdkdisplay_h {
    use super::gdktypes_h::{GdkDisplay, GdkSeat};
    extern "C" {

        pub fn gdk_display_get_default() -> *mut GdkDisplay;

        pub fn gdk_display_get_default_seat(display: *mut GdkDisplay) -> *mut GdkSeat;
    }
}

pub mod lauxlib_h {
    use super::lua_h::lua_State;
    use super::__stddef_size_t_h::size_t;
    extern "C" {

        pub fn luaL_typerror(
            L: *mut lua_State,
            narg: std::ffi::c_int,
            tname: *const std::ffi::c_char,
        ) -> std::ffi::c_int;

        pub fn luaL_checklstring(
            L: *mut lua_State,
            numArg: std::ffi::c_int,
            l: *mut size_t,
        ) -> *const std::ffi::c_char;

        pub fn luaL_error(
            L: *mut lua_State,
            fmt: *const std::ffi::c_char,
            _: ...
        ) -> std::ffi::c_int;
    }
}

pub mod luaobject_h {
    #[inline]

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

    pub unsafe extern "C" fn luaH_object_unref(mut L: *mut lua_State, mut p: gpointer) {
        luaH_object_registry_push(L);
        luaH_object_decref(L, -(1 as std::ffi::c_int), p);
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    }
    #[inline]

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
        lua_State, lua_pushlstring, lua_rawget, LUA_REGISTRYINDEX, lua_settop,
        lua_pushlightuserdata, lua_remove,
    };
    use super::gtypes_h::{gint, gpointer, gchar};
    extern "C" {

        pub fn luaH_object_decref(L: *mut lua_State, tud: gint, oud: gpointer);

        pub fn luaH_object_emit_signal(
            L: *mut lua_State,
            oud: gint,
            name: *const gchar,
            nargs: gint,
            nret: gint,
        ) -> gint;
    }
}

pub mod luah_h {
    #[inline]

    pub unsafe extern "C" fn luaH_checkboolean(
        mut L: *mut lua_State,
        mut n: gint,
    ) -> gboolean {
        if !(lua_type(L, n) == LUA_TBOOLEAN) {
            luaL_typerror(L, n, b"boolean\0" as *const u8 as *const std::ffi::c_char);
        }
        return lua_toboolean(L, n);
    }
    #[inline]

    pub unsafe extern "C" fn luaH_rawfield(
        mut L: *mut lua_State,
        mut idx: gint,
        mut field: *const gchar,
    ) -> gint {
        lua_pushstring(L, field);
        lua_rawget(L, idx);
        let mut type_0 = lua_type(L, -(1 as std::ffi::c_int));
        if type_0 == LUA_TNIL {
            lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        }
        return type_0;
    }
    use super::lua_h::{
        lua_State, lua_type, LUA_TBOOLEAN, lua_toboolean, lua_pushstring, lua_rawget,
        LUA_TNIL, lua_settop,
    };
    use super::gtypes_h::{gint, gboolean, gchar};
    use super::lauxlib_h::luaL_typerror;
}

pub mod luakit_luah_h {
    use super::lua_h::lua_State;
    use super::gtypes_h::guint;
    extern "C" {

        pub fn luaH_modifier_table_push(_: *mut lua_State, _: guint);

        pub fn luaH_keystr_push(_: *mut lua_State, _: guint);
    }
}

pub mod gmacros_h {

    pub const FALSE: std::ffi::c_int = 0 as std::ffi::c_int;

    pub const TRUE: std::ffi::c_int = (FALSE == 0) as std::ffi::c_int;
}

pub mod __stddef_null_h {

    pub const NULL: std::ffi::c_int = 0 as std::ffi::c_int;
}

pub mod gdkkeysyms_h {

    pub const GDK_KEY_VoidSymbol: std::ffi::c_int = 0xffffff as std::ffi::c_int;
}
pub use self::__stddef_ptrdiff_t_h::ptrdiff_t;
pub use self::__stddef_size_t_h::size_t;
pub use self::glibconfig_h::{
    gint8, guint8, gint16, guint16, guint32, gint64, guint64, gssize, gsize,
};
pub use self::gtypes_h::{
    gchar, gshort, glong, gint, gboolean, gulong, guint, gfloat, gdouble, gpointer,
};
pub use self::gdataset_h::{GData, _GData};
pub use self::glist_h::{_GList, GList, g_list_free};
pub use self::ghash_h::{GHashTable, _GHashTable};
pub use self::gslist_h::{_GSList, GSList};
pub use self::gunicode_h::{gunichar, g_utf8_get_char, g_utf8_strlen, g_utf8_validate};
pub use self::gstring_h::{
    _GString, GString, g_string_sized_new, g_string_free, g_string_free_and_steal,
    g_string_append_printf,
};
pub use self::gtree_h::{GTree, _GTree};
pub use self::gtype_h::{
    GType, GValue, _GTypeClass, GTypeClass, _GTypeInstance, GTypeInstance,
    g_type_check_instance_cast, g_type_check_instance_is_a,
};
pub use self::gvalue_h::{_GValue, C2RustUnnamed, g_value_init, g_value_unset};
pub use self::gparam_h::{
    GParamFlags, G_PARAM_DEPRECATED, G_PARAM_EXPLICIT_NOTIFY, G_PARAM_STATIC_BLURB,
    G_PARAM_STATIC_NICK, G_PARAM_PRIVATE, G_PARAM_STATIC_NAME, G_PARAM_LAX_VALIDATION,
    G_PARAM_CONSTRUCT_ONLY, G_PARAM_CONSTRUCT, G_PARAM_READWRITE, G_PARAM_WRITABLE,
    G_PARAM_READABLE, _GParamSpec, GParamSpec,
};
pub use self::gobject_h::{
    _GObject, GObject, _GObjectClass, GObjectConstructParam, _GObjectConstructParam,
    GObjectClass, GInitiallyUnowned, g_object_get, g_object_ref, g_object_get_data,
};
pub use self::cairo_h::{
    _cairo_rectangle_int, cairo_rectangle_int_t, cairo_region_t, _cairo_region,
};
pub use self::gdktypes_h::{
    GdkRectangle, GdkAtom, GdkDevice, GdkDragContext, GdkDisplay, GdkWindow, GdkKeymap,
    GdkSeat, C2RustUnnamed_0, GDK_MODIFIER_MASK, GDK_RELEASE_MASK,
    GDK_MODIFIER_RESERVED_29_MASK, GDK_META_MASK, GDK_HYPER_MASK, GDK_SUPER_MASK,
    GDK_MODIFIER_RESERVED_25_MASK, GDK_MODIFIER_RESERVED_24_MASK,
    GDK_MODIFIER_RESERVED_23_MASK, GDK_MODIFIER_RESERVED_22_MASK,
    GDK_MODIFIER_RESERVED_21_MASK, GDK_MODIFIER_RESERVED_20_MASK,
    GDK_MODIFIER_RESERVED_19_MASK, GDK_MODIFIER_RESERVED_18_MASK,
    GDK_MODIFIER_RESERVED_17_MASK, GDK_MODIFIER_RESERVED_16_MASK,
    GDK_MODIFIER_RESERVED_15_MASK, GDK_MODIFIER_RESERVED_14_MASK,
    GDK_MODIFIER_RESERVED_13_MASK, GDK_BUTTON5_MASK, GDK_BUTTON4_MASK, GDK_BUTTON3_MASK,
    GDK_BUTTON2_MASK, GDK_BUTTON1_MASK, GDK_MOD5_MASK, GDK_MOD4_MASK, GDK_MOD3_MASK,
    GDK_MOD2_MASK, GDK_MOD1_MASK, GDK_CONTROL_MASK, GDK_LOCK_MASK, GDK_SHIFT_MASK,
    GdkEventMask, GDK_ALL_EVENTS_MASK, GDK_TABLET_PAD_MASK, GDK_TOUCHPAD_GESTURE_MASK,
    GDK_SMOOTH_SCROLL_MASK, GDK_TOUCH_MASK, GDK_SCROLL_MASK, GDK_SUBSTRUCTURE_MASK,
    GDK_PROXIMITY_OUT_MASK, GDK_PROXIMITY_IN_MASK, GDK_VISIBILITY_NOTIFY_MASK,
    GDK_PROPERTY_CHANGE_MASK, GDK_STRUCTURE_MASK, GDK_FOCUS_CHANGE_MASK,
    GDK_LEAVE_NOTIFY_MASK, GDK_ENTER_NOTIFY_MASK, GDK_KEY_RELEASE_MASK,
    GDK_KEY_PRESS_MASK, GDK_BUTTON_RELEASE_MASK, GDK_BUTTON_PRESS_MASK,
    GDK_BUTTON3_MOTION_MASK, GDK_BUTTON2_MOTION_MASK, GDK_BUTTON1_MOTION_MASK,
    GDK_BUTTON_MOTION_MASK, GDK_POINTER_MOTION_HINT_MASK, GDK_POINTER_MOTION_MASK,
    GDK_EXPOSURE_MASK, GDK_CURRENT_TIME, _GdkAtom, _GdkDevice, _GdkDragContext,
    _GdkDisplay, _GdkWindow, _GdkKeymap,
};
pub use self::gdkseat_h::{_GdkSeat, gdk_seat_get_keyboard};
pub use self::gdkevents_h::{
    _GdkEventAny, GdkEventType, GDK_EVENT_LAST, GDK_PAD_GROUP_MODE, GDK_PAD_STRIP,
    GDK_PAD_RING, GDK_PAD_BUTTON_RELEASE, GDK_PAD_BUTTON_PRESS, GDK_TOUCHPAD_PINCH,
    GDK_TOUCHPAD_SWIPE, GDK_TOUCH_CANCEL, GDK_TOUCH_END, GDK_TOUCH_UPDATE,
    GDK_TOUCH_BEGIN, GDK_DAMAGE, GDK_GRAB_BROKEN, GDK_OWNER_CHANGE, GDK_SETTING,
    GDK_WINDOW_STATE, GDK_SCROLL, GDK_VISIBILITY_NOTIFY, GDK_CLIENT_EVENT,
    GDK_DROP_FINISHED, GDK_DROP_START, GDK_DRAG_STATUS, GDK_DRAG_MOTION, GDK_DRAG_LEAVE,
    GDK_DRAG_ENTER, GDK_PROXIMITY_OUT, GDK_PROXIMITY_IN, GDK_SELECTION_NOTIFY,
    GDK_SELECTION_REQUEST, GDK_SELECTION_CLEAR, GDK_PROPERTY_NOTIFY, GDK_UNMAP, GDK_MAP,
    GDK_CONFIGURE, GDK_FOCUS_CHANGE, GDK_LEAVE_NOTIFY, GDK_ENTER_NOTIFY, GDK_KEY_RELEASE,
    GDK_KEY_PRESS, GDK_BUTTON_RELEASE, GDK_TRIPLE_BUTTON_PRESS, GDK_3BUTTON_PRESS,
    GDK_DOUBLE_BUTTON_PRESS, GDK_2BUTTON_PRESS, GDK_BUTTON_PRESS, GDK_MOTION_NOTIFY,
    GDK_EXPOSE, GDK_DESTROY, GDK_DELETE, GDK_NOTHING, GdkEventAny, _GdkEventExpose,
    GdkEventExpose, _GdkEventVisibility, GdkVisibilityState,
    GDK_VISIBILITY_FULLY_OBSCURED, GDK_VISIBILITY_PARTIAL, GDK_VISIBILITY_UNOBSCURED,
    GdkEventVisibility, _GdkEventMotion, GdkEventMotion, _GdkEventButton, GdkEventButton,
    _GdkEventTouch, GdkEventSequence, GdkEventTouch, _GdkEventScroll, GdkScrollDirection,
    GDK_SCROLL_SMOOTH, GDK_SCROLL_RIGHT, GDK_SCROLL_LEFT, GDK_SCROLL_DOWN, GDK_SCROLL_UP,
    GdkEventScroll, _GdkEventKey, GdkEventKey, _GdkEventFocus, GdkEventFocus,
    _GdkEventCrossing, GdkNotifyType, GDK_NOTIFY_UNKNOWN, GDK_NOTIFY_NONLINEAR_VIRTUAL,
    GDK_NOTIFY_NONLINEAR, GDK_NOTIFY_INFERIOR, GDK_NOTIFY_VIRTUAL, GDK_NOTIFY_ANCESTOR,
    GdkCrossingMode, GDK_CROSSING_DEVICE_SWITCH, GDK_CROSSING_TOUCH_END,
    GDK_CROSSING_TOUCH_BEGIN, GDK_CROSSING_STATE_CHANGED, GDK_CROSSING_GTK_UNGRAB,
    GDK_CROSSING_GTK_GRAB, GDK_CROSSING_UNGRAB, GDK_CROSSING_GRAB, GDK_CROSSING_NORMAL,
    GdkEventCrossing, _GdkEventConfigure, GdkEventConfigure, _GdkEventProperty,
    GdkEventProperty, _GdkEventSelection, GdkEventSelection, _GdkEventOwnerChange,
    GdkOwnerChange, GDK_OWNER_CHANGE_CLOSE, GDK_OWNER_CHANGE_DESTROY,
    GDK_OWNER_CHANGE_NEW_OWNER, GdkEventOwnerChange, _GdkEventProximity,
    GdkEventProximity, _GdkEventDND, GdkEventDND, _GdkEventWindowState, GdkWindowState,
    GDK_WINDOW_STATE_LEFT_RESIZABLE, GDK_WINDOW_STATE_LEFT_TILED,
    GDK_WINDOW_STATE_BOTTOM_RESIZABLE, GDK_WINDOW_STATE_BOTTOM_TILED,
    GDK_WINDOW_STATE_RIGHT_RESIZABLE, GDK_WINDOW_STATE_RIGHT_TILED,
    GDK_WINDOW_STATE_TOP_RESIZABLE, GDK_WINDOW_STATE_TOP_TILED, GDK_WINDOW_STATE_TILED,
    GDK_WINDOW_STATE_FOCUSED, GDK_WINDOW_STATE_BELOW, GDK_WINDOW_STATE_ABOVE,
    GDK_WINDOW_STATE_FULLSCREEN, GDK_WINDOW_STATE_STICKY, GDK_WINDOW_STATE_MAXIMIZED,
    GDK_WINDOW_STATE_ICONIFIED, GDK_WINDOW_STATE_WITHDRAWN, GdkEventWindowState,
    _GdkEventSetting, GdkSettingAction, GDK_SETTING_ACTION_DELETED,
    GDK_SETTING_ACTION_CHANGED, GDK_SETTING_ACTION_NEW, GdkEventSetting,
    _GdkEventGrabBroken, GdkEventGrabBroken, _GdkEventTouchpadSwipe,
    GdkEventTouchpadSwipe, _GdkEventTouchpadPinch, GdkEventTouchpadPinch,
    _GdkEventPadButton, GdkEventPadButton, _GdkEventPadAxis, GdkEventPadAxis,
    _GdkEventPadGroupMode, GdkEventPadGroupMode, _GdkEvent, GdkEvent, _GdkEventSequence,
    gdk_event_new, gdk_event_get_scroll_deltas, gdk_event_set_device,
};
pub use self::gdkkeys_h::{
    _GdkKeymapKey, GdkKeymapKey, gdk_keymap_get_for_display,
    gdk_keymap_get_entries_for_keyval, gdk_keyval_from_name, gdk_unicode_to_keyval,
};
pub use self::gtkenums_h::{
    GtkAlign, GTK_ALIGN_BASELINE, GTK_ALIGN_CENTER, GTK_ALIGN_END, GTK_ALIGN_START,
    GTK_ALIGN_FILL,
};
pub use self::gtkwidget_h::{
    _GtkWidget, GtkWidgetPrivate, _GtkWidgetPrivate, gtk_widget_get_type,
    gtk_widget_destroy, gtk_widget_show, gtk_widget_hide, gtk_widget_is_focus,
    gtk_widget_grab_focus, gtk_widget_set_visible, gtk_widget_get_visible,
    gtk_widget_get_parent, gtk_widget_get_window, gtk_widget_get_allocated_width,
    gtk_widget_get_allocated_height, gtk_widget_set_size_request,
    gtk_widget_get_size_request, gtk_widget_get_halign, gtk_widget_set_halign,
    gtk_widget_get_valign_with_baseline, gtk_widget_set_valign,
    gtk_widget_set_tooltip_markup, gtk_widget_get_tooltip_markup,
};
pub use self::gtktypes_h::{GtkWidget, GtkWindow};
pub use self::gtkwindow_h::{
    _GtkWindow, GtkWindowPrivate, _GtkWindowPrivate, gtk_window_get_type,
    gtk_window_set_focus, gtk_window_has_toplevel_focus,
};
pub use self::gtkbin_h::{
    GtkBin, _GtkBin, GtkBinPrivate, _GtkBinPrivate, gtk_bin_get_type, gtk_bin_get_child,
};
pub use self::gtkcontainer_h::{
    GtkContainer, _GtkContainer, GtkContainerPrivate, _GtkContainerPrivate,
    gtk_container_get_type, gtk_container_add, gtk_container_remove,
    gtk_container_get_children, gtk_container_class_list_child_properties,
    gtk_container_child_set_property, gtk_container_child_get_property,
};
pub use self::gtkentry_h::{
    _GtkEntry, GtkEntryPrivate, GtkEntry, _GtkEntryPrivate, gtk_entry_get_type,
    gtk_entry_grab_focus_without_selecting,
};
pub use self::gtkcssprovider_h::{
    _GtkCssProvider, GtkCssProviderPrivate, GtkCssProvider, _GtkCssProviderPrivate,
};
pub use self::lua_h::{
    lua_Number, lua_Integer, LUA_REGISTRYINDEX, LUA_TNIL, LUA_TBOOLEAN, LUA_TTABLE,
    lua_State, lua_gettop, lua_settop, lua_remove, lua_insert, lua_type, lua_tonumber,
    lua_toboolean, lua_tolstring, lua_pushnil, lua_pushnumber, lua_pushinteger,
    lua_pushlstring, lua_pushstring, lua_pushboolean, lua_pushlightuserdata, lua_rawget,
    lua_createtable, lua_rawset, lua_rawseti, lua_next,
};
pub use self::log_h::{
    log_level_t, LOG_LEVEL_debug, LOG_LEVEL_verbose, LOG_LEVEL_info, LOG_LEVEL_warn,
    LOG_LEVEL_error, LOG_LEVEL_fatal, _log,
};
pub use self::signal_h::signal_t;
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
pub use self::luaclass_h::{
    lua_class_property_array_t, lua_object_t, lua_class_allocator_t,
    lua_class_propfunc_t, lua_class_t, luaH_checkudata,
};
pub use self::common_h::{_common_t, common_t, common};
pub use self::widget_h::{
    widget_t, widget_destructor_t, widget_info_t, widget_constructor_t,
    GOBJECT_LUAKIT_WIDGET_DATA_KEY, luaH_checkwidget, luaH_checkwidgetornil, widget_class,
};
use self::string_h::strcmp;
use self::gmem_h::{g_free, g_malloc0_n};
pub use self::gmessages_h::G_LOG_DOMAIN;
use self::gtestutils_h::g_assertion_message_expr;
use self::gsignal_h::g_signal_emit_by_name;
use self::gdkwindow_h::gdk_window_set_events;
use self::gdkdisplay_h::{gdk_display_get_default, gdk_display_get_default_seat};
use self::lauxlib_h::{luaL_typerror, luaL_checklstring, luaL_error};
pub use self::luaobject_h::{
    luaH_object_registry_push, luaH_object_unref, luaH_object_push, luaH_object_decref,
    luaH_object_emit_signal,
};
pub use self::luah_h::{luaH_checkboolean, luaH_rawfield};
use self::luakit_luah_h::{luaH_modifier_table_push, luaH_keystr_push};
pub use self::gmacros_h::{FALSE, TRUE};
pub use self::__stddef_null_h::NULL;
pub use self::gdkkeysyms_h::GDK_KEY_VoidSymbol;
#[no_mangle]

pub unsafe extern "C" fn key_press_cb(
    mut UNUSED_win: *mut GtkWidget,
    mut ev: *mut GdkEventKey,
    mut w: *mut widget_t,
) -> gboolean {
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    luaH_modifier_table_push(L, (*ev).state);
    luaH_keystr_push(L, (*ev).keyval);
    lua_pushboolean(L, (*ev).send_event as std::ffi::c_int);
    let mut ret = luaH_object_emit_signal(
        L,
        -(4 as std::ffi::c_int),
        b"key-press\0" as *const u8 as *const std::ffi::c_char,
        3 as std::ffi::c_int,
        1 as std::ffi::c_int,
    );
    let mut catch = if ret != 0 && lua_toboolean(L, -(1 as std::ffi::c_int)) != 0 {
        TRUE
    } else {
        FALSE
    };
    lua_settop(L, -(ret + 1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return catch;
}
#[no_mangle]

pub unsafe extern "C" fn button_cb(
    mut UNUSED_win: *mut GtkWidget,
    mut ev: *mut GdkEventButton,
    mut w: *mut widget_t,
) -> gboolean {
    let mut ret: gint = 0;
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    luaH_modifier_table_push(L, (*ev).state);
    lua_pushinteger(L, (*ev).button as lua_Integer);
    match (*ev).type_0 as std::ffi::c_int {
        5 => {
            ret = luaH_object_emit_signal(
                L,
                -(3 as std::ffi::c_int),
                b"button-double-click\0" as *const u8 as *const std::ffi::c_char,
                2 as std::ffi::c_int,
                1 as std::ffi::c_int,
            );
        }
        7 => {
            ret = luaH_object_emit_signal(
                L,
                -(3 as std::ffi::c_int),
                b"button-release\0" as *const u8 as *const std::ffi::c_char,
                2 as std::ffi::c_int,
                1 as std::ffi::c_int,
            );
        }
        _ => {
            ret = luaH_object_emit_signal(
                L,
                -(3 as std::ffi::c_int),
                b"button-press\0" as *const u8 as *const std::ffi::c_char,
                2 as std::ffi::c_int,
                1 as std::ffi::c_int,
            );
        }
    }
    let mut catch = if ret != 0 && lua_toboolean(L, -(1 as std::ffi::c_int)) != 0 {
        TRUE
    } else {
        FALSE
    };
    lua_settop(L, -(ret + 1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return catch;
}
#[no_mangle]

pub unsafe extern "C" fn scroll_cb(
    mut UNUSED_wid: *mut GtkWidget,
    mut ev: *mut GdkEventScroll,
    mut w: *mut widget_t,
) -> gboolean {
    let mut dx: std::ffi::c_double = 0.;
    let mut dy: std::ffi::c_double = 0.;
    match (*ev).direction as std::ffi::c_uint {
        0 => {
            dx = 0 as std::ffi::c_int as std::ffi::c_double;
            dy = -(1 as std::ffi::c_int) as std::ffi::c_double;
        }
        1 => {
            dx = 0 as std::ffi::c_int as std::ffi::c_double;
            dy = 1 as std::ffi::c_int as std::ffi::c_double;
        }
        2 => {
            dx = -(1 as std::ffi::c_int) as std::ffi::c_double;
            dy = 0 as std::ffi::c_int as std::ffi::c_double;
        }
        3 => {
            dx = 1 as std::ffi::c_int as std::ffi::c_double;
            dy = 0 as std::ffi::c_int as std::ffi::c_double;
        }
        4 => {
            gdk_event_get_scroll_deltas(ev as *mut GdkEvent, &mut dx, &mut dy);
        }
        _ => {
            g_assertion_message_expr(
                G_LOG_DOMAIN as *const std::ffi::c_char,
                b"widgets/common.c\0" as *const u8 as *const std::ffi::c_char,
                79 as std::ffi::c_int,
                (*::core::mem::transmute::<
                    &[u8; 10],
                    &[std::ffi::c_char; 10],
                >(b"scroll_cb\0"))
                    .as_ptr(),
                NULL as *const std::ffi::c_char,
            );
        }
    }
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    luaH_modifier_table_push(L, (*ev).state);
    lua_pushnumber(L, dx);
    lua_pushnumber(L, dy);
    let mut ret = luaH_object_emit_signal(
        L,
        -(4 as std::ffi::c_int),
        b"scroll\0" as *const u8 as *const std::ffi::c_char,
        3 as std::ffi::c_int,
        1 as std::ffi::c_int,
    );
    lua_settop(L, -(ret + 1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn mouse_cb(
    mut UNUSED_win: *mut GtkWidget,
    mut ev: *mut GdkEventCrossing,
    mut w: *mut widget_t,
) -> gboolean {
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    luaH_modifier_table_push(L, (*ev).state);
    let mut type_0 = (*ev).type_0;
    if type_0 as std::ffi::c_int == GDK_ENTER_NOTIFY as std::ffi::c_int
        || type_0 as std::ffi::c_int == GDK_LEAVE_NOTIFY as std::ffi::c_int
    {} else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"widgets/common.c\0" as *const u8 as *const std::ffi::c_char,
            101 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 9], &[std::ffi::c_char; 9]>(b"mouse_cb\0"))
                .as_ptr(),
            b"type == GDK_ENTER_NOTIFY || type == GDK_LEAVE_NOTIFY\0" as *const u8
                as *const std::ffi::c_char,
        );
    }
    let mut ret = luaH_object_emit_signal(
        L,
        -(2 as std::ffi::c_int),
        if type_0 as std::ffi::c_int == GDK_ENTER_NOTIFY as std::ffi::c_int {
            b"mouse-enter\0" as *const u8 as *const std::ffi::c_char
        } else {
            b"mouse-leave\0" as *const u8 as *const std::ffi::c_char
        },
        1 as std::ffi::c_int,
        1 as std::ffi::c_int,
    );
    let mut catch = if ret != 0 && lua_toboolean(L, -(1 as std::ffi::c_int)) != 0 {
        TRUE
    } else {
        FALSE
    };
    lua_settop(L, -(ret + 1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return catch;
}
#[no_mangle]

pub unsafe extern "C" fn focus_cb(
    mut UNUSED_win: *mut GtkWidget,
    mut ev: *mut GdkEventFocus,
    mut w: *mut widget_t,
) -> gboolean {
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    let mut ret: gint = 0;
    if (*ev).in_0 != 0 {
        ret = luaH_object_emit_signal(
            L,
            -(1 as std::ffi::c_int),
            b"focus\0" as *const u8 as *const std::ffi::c_char,
            0 as std::ffi::c_int,
            1 as std::ffi::c_int,
        );
    } else {
        ret = luaH_object_emit_signal(
            L,
            -(1 as std::ffi::c_int),
            b"unfocus\0" as *const u8 as *const std::ffi::c_char,
            0 as std::ffi::c_int,
            1 as std::ffi::c_int,
        );
    }
    if ret != 0 && lua_toboolean(L, -(1 as std::ffi::c_int)) != 0 {
        lua_settop(L, -(ret + 1 as std::ffi::c_int) - 1 as std::ffi::c_int);
        return TRUE;
    }
    lua_settop(L, -(ret + 1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    return FALSE;
}
#[no_mangle]

pub unsafe extern "C" fn add_cb(
    mut UNUSED_c: *mut GtkContainer,
    mut widget: *mut GtkWidget,
    mut w: *mut widget_t,
) {
    let mut child = g_object_get_data(
        g_type_check_instance_cast(
            widget as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
        GOBJECT_LUAKIT_WIDGET_DATA_KEY.as_ptr(),
    ) as *mut widget_t;
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    luaH_object_push(L, (*child).ref_0);
    luaH_object_emit_signal(
        L,
        -(2 as std::ffi::c_int),
        b"add\0" as *const u8 as *const std::ffi::c_char,
        1 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
#[no_mangle]

pub unsafe extern "C" fn resize_cb(
    mut UNUSED_win: *mut GtkWidget,
    mut rect: *mut GdkRectangle,
    mut w: *mut widget_t,
) {
    let mut width = (*rect).width;
    let mut height = (*rect).height;
    if width == (*w).prev_width && height == (*w).prev_height {
        return;
    }
    (*w).prev_width = width;
    (*w).prev_height = height;
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    lua_pushinteger(L, width as lua_Integer);
    lua_pushinteger(L, height as lua_Integer);
    luaH_object_emit_signal(
        L,
        -(3 as std::ffi::c_int),
        b"resize\0" as *const u8 as *const std::ffi::c_char,
        2 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
#[no_mangle]

pub unsafe extern "C" fn remove_cb(
    mut UNUSED_c: *mut GtkContainer,
    mut widget: *mut GtkWidget,
    mut w: *mut widget_t,
) {
    let mut child = g_object_get_data(
        g_type_check_instance_cast(
            widget as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
        GOBJECT_LUAKIT_WIDGET_DATA_KEY.as_ptr(),
    ) as *mut widget_t;
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    luaH_object_push(L, (*child).ref_0);
    luaH_object_emit_signal(
        L,
        -(2 as std::ffi::c_int),
        b"remove\0" as *const u8 as *const std::ffi::c_char,
        1 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
#[no_mangle]

pub unsafe extern "C" fn parent_set_cb(
    mut widget: *mut GtkWidget,
    mut UNUSED_p: *mut GtkWidget,
    mut w: *mut widget_t,
) {
    let mut L = common.L;
    let mut parent = NULL as *mut widget_t;
    let mut new = 0 as *mut GtkContainer;
    g_object_get(
        g_type_check_instance_cast(
            widget as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject as gpointer,
        b"parent\0" as *const u8 as *const std::ffi::c_char,
        &mut new as *mut *mut GtkContainer,
        NULL as *mut std::ffi::c_void,
    );
    luaH_object_push(L, (*w).ref_0);
    if !new.is_null()
        && {
            parent = g_object_get_data(
                g_type_check_instance_cast(
                    new as *mut GTypeInstance,
                    ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
                ) as *mut std::ffi::c_void as *mut GObject,
                GOBJECT_LUAKIT_WIDGET_DATA_KEY.as_ptr(),
            ) as *mut widget_t;
            !parent.is_null()
        }
    {
        luaH_object_push(L, (*parent).ref_0);
    } else {
        lua_pushnil(L);
    }
    luaH_object_emit_signal(
        L,
        -(2 as std::ffi::c_int),
        b"parent-set\0" as *const u8 as *const std::ffi::c_char,
        1 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
#[no_mangle]

pub unsafe extern "C" fn destroy_cb(
    mut UNUSED_win: *mut GtkWidget,
    mut w: *mut widget_t,
) {
    let mut L = common.L;
    luaH_object_push(L, (*w).ref_0);
    luaH_object_emit_signal(
        L,
        -(1 as std::ffi::c_int),
        b"destroy\0" as *const u8 as *const std::ffi::c_char,
        0 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    _log(
        LOG_LEVEL_debug,
        b"widgets/common.c\0" as *const u8 as *const std::ffi::c_char,
        b"destroy %p (%s)\0" as *const u8 as *const std::ffi::c_char,
        w,
        (*(*w).info).name,
    );
    if ((*w).destructor).is_some() {
        ((*w).destructor).expect("non-null function pointer")(w);
    }
    (*w)
        .destructor = ::core::mem::transmute::<
        libc::intptr_t,
        Option::<widget_destructor_t>,
    >(NULL as libc::intptr_t);
    (*w).widget = NULL as *mut GtkWidget;
    luaH_object_unref(L, (*w).ref_0);
}
#[no_mangle]

pub unsafe extern "C" fn true_cb() -> gboolean {
    return TRUE;
}
#[no_mangle]

pub unsafe extern "C" fn luaH_widget_set_child(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
) -> gint {
    let mut child = luaH_checkwidgetornil(L, 3 as std::ffi::c_int);
    let mut widget = gtk_bin_get_child(
        g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_bin_get_type())
            as *mut std::ffi::c_void as *mut GtkBin,
    );
    if !widget.is_null() {
        g_object_ref(
            g_type_check_instance_cast(
                widget as *mut GTypeInstance,
                ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
            ) as *mut std::ffi::c_void as *mut GObject as gpointer,
        );
        gtk_container_remove(
            g_type_check_instance_cast(
                (*w).widget as *mut GTypeInstance,
                gtk_container_get_type(),
            ) as *mut std::ffi::c_void as *mut GtkContainer,
            g_type_check_instance_cast(
                widget as *mut GTypeInstance,
                gtk_widget_get_type(),
            ) as *mut std::ffi::c_void as *mut GtkWidget,
        );
    }
    if !child.is_null() {
        gtk_container_add(
            g_type_check_instance_cast(
                (*w).widget as *mut GTypeInstance,
                gtk_container_get_type(),
            ) as *mut std::ffi::c_void as *mut GtkContainer,
            g_type_check_instance_cast(
                (*child).widget as *mut GTypeInstance,
                gtk_widget_get_type(),
            ) as *mut std::ffi::c_void as *mut GtkWidget,
        );
    }
    return 0 as std::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn luaH_widget_get_child(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
) -> gint {
    let mut widget = gtk_bin_get_child(
        g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_bin_get_type())
            as *mut std::ffi::c_void as *mut GtkBin,
    );
    if widget.is_null() {
        return 0 as std::ffi::c_int;
    }
    let mut child = g_object_get_data(
        g_type_check_instance_cast(
            widget as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
        GOBJECT_LUAKIT_WIDGET_DATA_KEY.as_ptr(),
    ) as *mut widget_t;
    luaH_object_push(L, (*child).ref_0);
    return 1 as std::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn luaH_widget_remove(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkwidget(L, 1 as std::ffi::c_int);
    let mut child = luaH_checkwidget(L, 2 as std::ffi::c_int);
    g_object_ref(
        g_type_check_instance_cast(
            (*child).widget as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject as gpointer,
    );
    gtk_container_remove(
        g_type_check_instance_cast(
            (*w).widget as *mut GTypeInstance,
            gtk_container_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkContainer,
        g_type_check_instance_cast(
            (*child).widget as *mut GTypeInstance,
            gtk_widget_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkWidget,
    );
    return 0 as std::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn luaH_widget_get_children(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
) -> gint {
    if ({
        let mut __inst = (*w).widget as *mut GTypeInstance;
        let mut __t = gtk_container_get_type();
        let mut __r: gboolean = 0;
        if __inst.is_null() {
            __r = FALSE;
        } else if !((*__inst).g_class).is_null() && (*(*__inst).g_class).g_type == __t {
            __r = TRUE;
        } else {
            __r = g_type_check_instance_is_a(__inst, __t);
        }
        __r
    }) == 0
    {
        return 0 as std::ffi::c_int;
    }
    let mut children = gtk_container_get_children(
        g_type_check_instance_cast(
            (*w).widget as *mut GTypeInstance,
            gtk_container_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkContainer,
    );
    let mut iter = children;
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    let mut i = 1 as std::ffi::c_int;
    while !iter.is_null() {
        luaH_object_push(
            L,
            (*(g_object_get_data(
                g_type_check_instance_cast(
                    (*iter).data as *mut GTypeInstance,
                    ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
                ) as *mut std::ffi::c_void as *mut GObject,
                GOBJECT_LUAKIT_WIDGET_DATA_KEY.as_ptr(),
            ) as *mut widget_t))
                .ref_0,
        );
        let fresh0 = i;
        i = i + 1;
        lua_rawseti(L, -(2 as std::ffi::c_int), fresh0);
        iter = (*iter).next;
    }
    g_list_free(children);
    return 1 as std::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn luaH_widget_replace(mut L: *mut lua_State) -> gint {
    let mut och = luaH_checkwidget(L, 1 as std::ffi::c_int);
    let mut nch = luaH_checkwidget(L, 2 as std::ffi::c_int);
    let mut parent = gtk_widget_get_parent(
        g_type_check_instance_cast(
            (*och).widget as *mut GTypeInstance,
            gtk_widget_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkWidget,
    );
    if parent.is_null() {
        return 0 as std::ffi::c_int;
    }
    let mut num_props: guint = 0;
    let mut props = gtk_container_class_list_child_properties(
        (*(parent as *mut GTypeInstance)).g_class as *mut GObjectClass,
        &mut num_props,
    );
    let mut values = g_malloc0_n(
        num_props as gsize,
        ::core::mem::size_of::<GValue>() as std::ffi::c_ulong,
    ) as *mut GValue;
    let mut i = 0 as std::ffi::c_int as guint;
    while i < num_props {
        g_value_init(
            &mut *values.offset(i as isize),
            (*(g_type_check_instance_cast(
                *props.offset(i as isize) as *mut GTypeInstance,
                ((19 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
            ) as *mut std::ffi::c_void as *mut GParamSpec))
                .value_type,
        );
        gtk_container_child_get_property(
            g_type_check_instance_cast(
                parent as *mut GTypeInstance,
                gtk_container_get_type(),
            ) as *mut std::ffi::c_void as *mut GtkContainer,
            g_type_check_instance_cast(
                (*och).widget as *mut GTypeInstance,
                gtk_widget_get_type(),
            ) as *mut std::ffi::c_void as *mut GtkWidget,
            (**props.offset(i as isize)).name,
            &mut *values.offset(i as isize),
        );
        i = i.wrapping_add(1);
        i;
    }
    g_object_ref(
        g_type_check_instance_cast(
            (*och).widget as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject as gpointer,
    );
    gtk_container_remove(
        g_type_check_instance_cast(
            parent as *mut GTypeInstance,
            gtk_container_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkContainer,
        g_type_check_instance_cast(
            (*och).widget as *mut GTypeInstance,
            gtk_widget_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkWidget,
    );
    gtk_container_add(
        g_type_check_instance_cast(
            parent as *mut GTypeInstance,
            gtk_container_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkContainer,
        g_type_check_instance_cast(
            (*nch).widget as *mut GTypeInstance,
            gtk_widget_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkWidget,
    );
    let mut i_0 = 0 as std::ffi::c_int as guint;
    while i_0 < num_props {
        gtk_container_child_set_property(
            g_type_check_instance_cast(
                parent as *mut GTypeInstance,
                gtk_container_get_type(),
            ) as *mut std::ffi::c_void as *mut GtkContainer,
            g_type_check_instance_cast(
                (*nch).widget as *mut GTypeInstance,
                gtk_widget_get_type(),
            ) as *mut std::ffi::c_void as *mut GtkWidget,
            (**props.offset(i_0 as isize)).name,
            &mut *values.offset(i_0 as isize),
        );
        g_value_unset(&mut *values.offset(i_0 as isize));
        i_0 = i_0.wrapping_add(1);
        i_0;
    }
    g_free(props as gpointer);
    g_free(values as gpointer);
    return 0 as std::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn luaH_widget_show(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkwidget(L, 1 as std::ffi::c_int);
    gtk_widget_show((*w).widget);
    return 0 as std::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn luaH_widget_hide(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkwidget(L, 1 as std::ffi::c_int);
    gtk_widget_hide((*w).widget);
    return 0 as std::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn luaH_widget_send_key(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkwidget(L, 1 as std::ffi::c_int);
    let mut key_name = luaL_checklstring(L, 2 as std::ffi::c_int, NULL as *mut size_t);
    if !(lua_type(L, 3 as std::ffi::c_int) == LUA_TTABLE) {
        lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
        lua_insert(L, 3 as std::ffi::c_int);
    }
    let is_release = lua_toboolean(L, 4 as std::ffi::c_int);
    if g_utf8_validate(
        key_name,
        -(1 as std::ffi::c_int) as gssize,
        NULL as *mut *const gchar,
    ) == 0
    {
        return luaL_error(
            L,
            b"key name isn't a utf-8 string\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut keyval: guint = 0;
    if g_utf8_strlen(key_name, -(1 as std::ffi::c_int) as gssize)
        == 1 as std::ffi::c_int as glong
    {
        keyval = gdk_unicode_to_keyval(g_utf8_get_char(key_name));
    } else {
        keyval = gdk_keyval_from_name(key_name);
    }
    if keyval == 0 || keyval == GDK_KEY_VoidSymbol as guint {
        return luaL_error(
            L,
            b"failed to get a valid key value\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut state = 0 as std::ffi::c_int as guint;
    let mut state_string = g_string_sized_new(32 as std::ffi::c_int as gsize);
    lua_pushnil(L);
    while lua_next(L, 3 as std::ffi::c_int) != 0 {
        let mut mod_0 = luaL_checklstring(
            L,
            -(1 as std::ffi::c_int),
            NULL as *mut size_t,
        );
        g_string_append_printf(
            state_string,
            b"%s-\0" as *const u8 as *const std::ffi::c_char,
            mod_0,
        );
        if strcmp(b"shift\0" as *const u8 as *const std::ffi::c_char, mod_0)
            == 0 as std::ffi::c_int
        {
            state = state | GDK_SHIFT_MASK as std::ffi::c_int as guint;
        }
        if strcmp(b"control\0" as *const u8 as *const std::ffi::c_char, mod_0)
            == 0 as std::ffi::c_int
        {
            state = state | GDK_CONTROL_MASK as std::ffi::c_int as guint;
        }
        if strcmp(b"lock\0" as *const u8 as *const std::ffi::c_char, mod_0)
            == 0 as std::ffi::c_int
        {
            state = state | GDK_LOCK_MASK as std::ffi::c_int as guint;
        }
        if strcmp(b"mod1\0" as *const u8 as *const std::ffi::c_char, mod_0)
            == 0 as std::ffi::c_int
        {
            state = state | GDK_MOD1_MASK as std::ffi::c_int as guint;
        }
        if strcmp(b"mod2\0" as *const u8 as *const std::ffi::c_char, mod_0)
            == 0 as std::ffi::c_int
        {
            state = state | GDK_MOD2_MASK as std::ffi::c_int as guint;
        }
        if strcmp(b"mod3\0" as *const u8 as *const std::ffi::c_char, mod_0)
            == 0 as std::ffi::c_int
        {
            state = state | GDK_MOD3_MASK as std::ffi::c_int as guint;
        }
        if strcmp(b"mod4\0" as *const u8 as *const std::ffi::c_char, mod_0)
            == 0 as std::ffi::c_int
        {
            state = state | GDK_MOD4_MASK as std::ffi::c_int as guint;
        }
        if strcmp(b"mod5\0" as *const u8 as *const std::ffi::c_char, mod_0)
            == 0 as std::ffi::c_int
        {
            state = state | GDK_MOD5_MASK as std::ffi::c_int as guint;
        }
        lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
    }
    let mut keys = NULL as *mut GdkKeymapKey;
    let mut n_keys: gint = 0;
    let mut keymap = gdk_keymap_get_for_display(gdk_display_get_default());
    if gdk_keymap_get_entries_for_keyval(keymap, keyval, &mut keys, &mut n_keys) == 0 {
        if 0 != 0 {
            if 0 as std::ffi::c_int == 0 {
                g_string_free(
                    state_string,
                    (0 as std::ffi::c_int == 0) as std::ffi::c_int,
                );
            } else {
                g_string_free_and_steal(state_string);
            };
        } else {
            g_string_free(state_string, (0 as std::ffi::c_int == 0) as std::ffi::c_int);
        };
        return luaL_error(
            L,
            b"cannot type '%s' on current keyboard layout\0" as *const u8
                as *const std::ffi::c_char,
            key_name,
        );
    }
    let mut event = gdk_event_new(
        (if is_release != 0 {
            GDK_KEY_RELEASE as std::ffi::c_int
        } else {
            GDK_KEY_PRESS as std::ffi::c_int
        }) as GdkEventType,
    );
    let mut event_key = event as *mut GdkEventKey;
    (*event_key).window = gtk_widget_get_window((*w).widget);
    (*event_key).send_event = TRUE as gint8;
    (*event_key).time = GDK_CURRENT_TIME as guint32;
    (*event_key).state = state;
    (*event_key).keyval = keyval;
    (*event_key)
        .hardware_keycode = (*keys.offset(0 as std::ffi::c_int as isize)).keycode
        as guint16;
    (*event_key).group = (*keys.offset(0 as std::ffi::c_int as isize)).group as guint8;
    let mut kbd = NULL as *mut GdkDevice;
    let mut seat = gdk_display_get_default_seat(gdk_display_get_default());
    kbd = gdk_seat_get_keyboard(seat);
    if kbd.is_null() {
        return luaL_error(
            L,
            b"failed to find a keyboard device\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    gdk_event_set_device(event, kbd);
    let mut ret: gboolean = 0;
    _log(
        LOG_LEVEL_debug,
        b"widgets/common.c\0" as *const u8 as *const std::ffi::c_char,
        b"sending key '%s%s' to widget %p\0" as *const u8 as *const std::ffi::c_char,
        (*state_string).str_0,
        key_name,
        (*w).widget,
    );
    g_signal_emit_by_name(
        (*w).widget as gpointer,
        if is_release != 0 {
            b"key-release-event\0" as *const u8 as *const std::ffi::c_char
        } else {
            b"key-press-event\0" as *const u8 as *const std::ffi::c_char
        },
        event,
        &mut ret as *mut gboolean,
    );
    if 0 != 0 {
        if 0 as std::ffi::c_int == 0 {
            g_string_free(state_string, (0 as std::ffi::c_int == 0) as std::ffi::c_int);
        } else {
            g_string_free_and_steal(state_string);
        };
    } else {
        g_string_free(state_string, (0 as std::ffi::c_int == 0) as std::ffi::c_int);
    };
    g_free(keys as gpointer);
    return 0 as std::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn luaH_widget_set_visible(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
) -> gint {
    let mut visible = luaH_checkboolean(L, 3 as std::ffi::c_int);
    gtk_widget_set_visible((*w).widget, visible);
    if visible != 0
        && (*(*w).info).tok as std::ffi::c_uint
            == L_TK_WINDOW as std::ffi::c_int as std::ffi::c_uint
    {
        gdk_window_set_events(gtk_widget_get_window((*w).widget), GDK_ALL_EVENTS_MASK);
    }
    return 0 as std::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn luaH_widget_get_min_size(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
) -> gint {
    let mut width: gint = 0;
    let mut height: gint = 0;
    gtk_widget_get_size_request((*w).widget, &mut width, &mut height);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_pushlstring(
        L,
        b"width\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 6]>() as std::ffi::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
            )
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    lua_pushinteger(L, width as lua_Integer);
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_pushlstring(
        L,
        b"height\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 7]>() as std::ffi::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
            )
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    lua_pushinteger(L, height as lua_Integer);
    lua_rawset(L, -(3 as std::ffi::c_int));
    return 1 as std::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn luaH_widget_set_min_size(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
) -> gint {
    if !(lua_type(L, 3 as std::ffi::c_int) == LUA_TTABLE) {
        luaL_typerror(
            L,
            3 as std::ffi::c_int,
            b"table\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut width: gint = 0;
    let mut height: gint = 0;
    gtk_widget_get_size_request((*w).widget, &mut width, &mut height);
    let mut top = lua_gettop(L);
    if luaH_rawfield(
        L,
        3 as std::ffi::c_int,
        b"w\0" as *const u8 as *const std::ffi::c_char,
    ) != 0
    {
        width = lua_tonumber(L, -(1 as std::ffi::c_int)) as gint;
    }
    if luaH_rawfield(
        L,
        3 as std::ffi::c_int,
        b"h\0" as *const u8 as *const std::ffi::c_char,
    ) != 0
    {
        height = lua_tonumber(L, -(1 as std::ffi::c_int)) as gint;
    }
    lua_settop(L, top);
    gtk_widget_set_size_request((*w).widget, width, height);
    return 1 as std::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn luaH_widget_get_align(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
) -> gint {
    let mut halign = gtk_widget_get_halign(
        g_type_check_instance_cast(
            (*w).widget as *mut GTypeInstance,
            gtk_widget_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkWidget,
    );
    let mut valign = gtk_widget_get_valign_with_baseline(
        g_type_check_instance_cast(
            (*w).widget as *mut GTypeInstance,
            gtk_widget_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkWidget,
    );
    lua_createtable(L, 0 as std::ffi::c_int, 2 as std::ffi::c_int);
    lua_pushlstring(
        L,
        b"h\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 2]>() as std::ffi::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
            )
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    lua_pushnumber(L, halign as lua_Number);
    lua_rawset(L, -(3 as std::ffi::c_int));
    lua_pushlstring(
        L,
        b"v\0" as *const u8 as *const std::ffi::c_char,
        (::core::mem::size_of::<[std::ffi::c_char; 2]>() as std::ffi::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<std::ffi::c_char>() as std::ffi::c_ulong,
            )
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
    );
    lua_pushnumber(L, valign as lua_Number);
    lua_rawset(L, -(3 as std::ffi::c_int));
    return 1 as std::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn luaH_widget_set_align(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
) -> gint {
    if !(lua_type(L, 3 as std::ffi::c_int) == LUA_TTABLE) {
        luaL_typerror(
            L,
            3 as std::ffi::c_int,
            b"table\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    let mut halign = gtk_widget_get_halign(
        g_type_check_instance_cast(
            (*w).widget as *mut GTypeInstance,
            gtk_widget_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkWidget,
    );
    let mut valign = gtk_widget_get_valign_with_baseline(
        g_type_check_instance_cast(
            (*w).widget as *mut GTypeInstance,
            gtk_widget_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkWidget,
    );
    if luaH_rawfield(
        L,
        3 as std::ffi::c_int,
        b"h\0" as *const u8 as *const std::ffi::c_char,
    ) != 0
    {
        match l_tokenize(lua_tolstring(L, -(1 as std::ffi::c_int), NULL as *mut size_t))
            as std::ffi::c_uint
        {
            96 => {
                halign = GTK_ALIGN_FILL;
            }
            224 => {
                halign = GTK_ALIGN_START;
            }
            86 => {
                halign = GTK_ALIGN_END;
            }
            20 => {
                halign = GTK_ALIGN_CENTER;
            }
            12 => {
                halign = GTK_ALIGN_BASELINE;
            }
            _ => {
                return luaL_error(
                    L,
                    b"Bad alignment value (expected fill, start, end, center, or baseline)\0"
                        as *const u8 as *const std::ffi::c_char,
                );
            }
        }
    }
    if luaH_rawfield(
        L,
        3 as std::ffi::c_int,
        b"v\0" as *const u8 as *const std::ffi::c_char,
    ) != 0
    {
        match l_tokenize(lua_tolstring(L, -(1 as std::ffi::c_int), NULL as *mut size_t))
            as std::ffi::c_uint
        {
            96 => {
                valign = GTK_ALIGN_FILL;
            }
            224 => {
                valign = GTK_ALIGN_START;
            }
            86 => {
                valign = GTK_ALIGN_END;
            }
            20 => {
                valign = GTK_ALIGN_CENTER;
            }
            12 => {
                valign = GTK_ALIGN_BASELINE;
            }
            _ => {
                return luaL_error(
                    L,
                    b"Bad alignment value (expected fill, start, end, center, or baseline)\0"
                        as *const u8 as *const std::ffi::c_char,
                );
            }
        }
    }
    gtk_widget_set_halign(
        g_type_check_instance_cast(
            (*w).widget as *mut GTypeInstance,
            gtk_widget_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkWidget,
        halign,
    );
    gtk_widget_set_valign(
        g_type_check_instance_cast(
            (*w).widget as *mut GTypeInstance,
            gtk_widget_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkWidget,
        valign,
    );
    return 0 as std::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn luaH_widget_set_tooltip(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
) -> gint {
    let ref mut fresh1 = lua_tolstring(L, 3 as std::ffi::c_int, NULL as *mut size_t);
    gtk_widget_set_tooltip_markup(
        (*w).widget,
        if !(*fresh1).is_null() {
            *fresh1
        } else {
            b"\0" as *const u8 as *const std::ffi::c_char
        },
    );
    return 0 as std::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn luaH_widget_get_tooltip(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
) -> gint {
    lua_pushstring(L, gtk_widget_get_tooltip_markup((*w).widget));
    return 1 as std::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn luaH_widget_get_parent(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
) -> gint {
    let mut widget = gtk_widget_get_parent(
        g_type_check_instance_cast(
            (*w).widget as *mut GTypeInstance,
            gtk_widget_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkWidget,
    );
    if widget.is_null() {
        return 0 as std::ffi::c_int;
    }
    let mut parent = g_object_get_data(
        g_type_check_instance_cast(
            widget as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
        GOBJECT_LUAKIT_WIDGET_DATA_KEY.as_ptr(),
    ) as *mut widget_t;
    luaH_object_push(L, (*parent).ref_0);
    return 1 as std::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn luaH_widget_get_focused(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
) -> gint {
    let mut focused = if (*(*w).info).tok as std::ffi::c_uint
        == L_TK_WINDOW as std::ffi::c_int as std::ffi::c_uint
    {
        gtk_window_has_toplevel_focus(
            g_type_check_instance_cast(
                (*w).widget as *mut GTypeInstance,
                gtk_window_get_type(),
            ) as *mut std::ffi::c_void as *mut GtkWindow,
        )
    } else {
        gtk_widget_is_focus((*w).widget)
    };
    lua_pushboolean(L, focused);
    return 1 as std::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn luaH_widget_get_visible(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
) -> gint {
    lua_pushboolean(L, gtk_widget_get_visible((*w).widget));
    return 1 as std::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn luaH_widget_get_width(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
) -> gint {
    lua_pushnumber(L, gtk_widget_get_allocated_width((*w).widget) as lua_Number);
    return 1 as std::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn luaH_widget_get_height(
    mut L: *mut lua_State,
    mut w: *mut widget_t,
) -> gint {
    lua_pushnumber(L, gtk_widget_get_allocated_height((*w).widget) as lua_Number);
    return 1 as std::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn luaH_widget_focus(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkwidget(L, 1 as std::ffi::c_int);
    match (*(*w).info).tok as std::ffi::c_uint {
        263 => {
            gtk_window_set_focus(
                g_type_check_instance_cast(
                    (*w).widget as *mut GTypeInstance,
                    gtk_window_get_type(),
                ) as *mut std::ffi::c_void as *mut GtkWindow,
                NULL as *mut GtkWidget,
            );
        }
        87 => {
            gtk_entry_grab_focus_without_selecting(
                g_type_check_instance_cast(
                    (*w).widget as *mut GTypeInstance,
                    gtk_entry_get_type(),
                ) as *mut std::ffi::c_void as *mut GtkEntry,
            );
        }
        _ => {
            gtk_widget_grab_focus((*w).widget);
        }
    }
    return 0 as std::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn luaH_widget_destroy(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkwidget(L, 1 as std::ffi::c_int);
    gtk_widget_destroy(
        g_type_check_instance_cast(
            (*w).widget as *mut GTypeInstance,
            gtk_widget_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkWidget,
    );
    return 0 as std::ffi::c_int;
}
