use cairo_sys::*;
use gdk_pixbuf_sys::*;
use gdk_sys::*;
use gio_sys::*;
use glib_sys::*;
use gobject_sys::*;
use gtk_sys::*;
use libc::*;
use mlua_sys::*;
use pango_sys::*;
use webkit2gtk_sys::*;

use crate::clib::widget::widget_set_css_properties;
use crate::common::common;
use crate::common::luaclass::*;
use crate::common::luah::*;
use crate::common::luaobject::*;
use crate::common::luaserialize::lua_deserialize_range;
use crate::common::resource::*;
use crate::common::tokenize::*;
use crate::gtypes::*;
use crate::log::*;
use crate::web_context_get;
use crate::widgets::common::*;
use crate::widgets::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct LuakitAuthData {
    pub request: *mut WebKitAuthenticationRequest,
    pub w: *mut widget_t,
    pub login_entry: *mut GtkWidget,
    pub password_entry: *mut GtkWidget,
    pub checkbutton: *mut GtkWidget,
}
pub unsafe extern "C" fn free_auth_data(auth_data: *mut LuakitAuthData) {
    g_object_unref((*auth_data).request as *mut GObject);
    g_slice_free1(
        ::core::mem::size_of::<LuakitAuthData>(),
        auth_data as gpointer,
    );
}
pub unsafe extern "C" fn luakit_store_password(
    auth_data: *mut LuakitAuthData,
    login: *const gchar,
    password: *const gchar,
) {
    let L = common.L;
    let uri = webkit_web_view_get_uri(g_type_check_instance_cast(
        (*(*auth_data).w).widget as *mut GTypeInstance,
        webkit_web_view_get_type(),
    ) as *mut std::ffi::c_void as *mut WebKitWebView);
    luaH_object_push(L, (*(*auth_data).w).ref_0);
    lua_pushstring(L, uri);
    lua_pushstring(L, login);
    lua_pushstring(L, password);
    luaH_object_emit_signal(
        L,
        -(4 as std::ffi::c_int),
        b"store-password\0" as *const u8 as *const std::ffi::c_char,
        3 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
pub unsafe extern "C" fn luakit_find_password(
    auth_data: *mut LuakitAuthData,
    login: *mut *const gchar,
    password: *mut *const gchar,
) {
    let L = common.L;
    let uri = webkit_web_view_get_uri(g_type_check_instance_cast(
        (*(*auth_data).w).widget as *mut GTypeInstance,
        webkit_web_view_get_type(),
    ) as *mut std::ffi::c_void as *mut WebKitWebView);
    luaH_object_push(L, (*(*auth_data).w).ref_0);
    lua_pushstring(L, uri);
    let ret = luaH_object_emit_signal(
        L,
        -(2 as std::ffi::c_int),
        b"store-password\0" as *const u8 as *const std::ffi::c_char,
        1 as std::ffi::c_int,
        LUA_MULTRET,
    );
    if ret >= 2 as std::ffi::c_int {
        *password = luaL_checklstring(L, -(1 as std::ffi::c_int), std::ptr::null_mut());
        *login = luaL_checklstring(L, -(2 as std::ffi::c_int), std::ptr::null_mut());
    }
    lua_settop(L, -(1 as std::ffi::c_int + ret) - 1 as std::ffi::c_int);
}
pub unsafe extern "C" fn response_callback(
    mut dialog: *mut GtkDialog,
    mut response_id: gint,
    mut auth_data: *mut LuakitAuthData,
) {
    let mut login = 0 as *const gchar;
    let mut password = 0 as *const gchar;
    let mut store_password: gboolean = 0;
    let mut credential = 0 as *mut WebKitCredential;
    match response_id {
        -5 => {
            login = gtk_entry_get_text(g_type_check_instance_cast(
                (*auth_data).login_entry as *mut GTypeInstance,
                gtk_entry_get_type(),
            ) as *mut std::ffi::c_void as *mut GtkEntry);
            password = gtk_entry_get_text(g_type_check_instance_cast(
                (*auth_data).password_entry as *mut GTypeInstance,
                gtk_entry_get_type(),
            ) as *mut std::ffi::c_void as *mut GtkEntry);
            credential = webkit_credential_new(login, password, WEBKIT_CREDENTIAL_PERSISTENCE_NONE);
            webkit_authentication_request_authenticate((*auth_data).request, credential);
            webkit_credential_free(credential);
            store_password = gtk_toggle_button_get_active(g_type_check_instance_cast(
                (*auth_data).checkbutton as *mut GTypeInstance,
                gtk_toggle_button_get_type(),
            ) as *mut std::ffi::c_void
                as *mut GtkToggleButton);
            if store_password != 0 {
                luakit_store_password(auth_data, login, password);
            }
        }
        _ => {}
    }
    free_auth_data(auth_data);
    gtk_widget_destroy(g_type_check_instance_cast(
        dialog as *mut GTypeInstance,
        gtk_widget_get_type(),
    ) as *mut std::ffi::c_void as *mut GtkWidget);
}
pub unsafe extern "C" fn table_add_entry(
    mut table: *mut GtkWidget,
    mut row: gint,
    mut label_text: *const gchar,
    mut value: *const gchar,
    mut UNUSED_user_data: gpointer,
) -> *mut GtkWidget {
    let mut label = gtk_label_new(label_text);
    let mut align = {
        let mut init = GValue {
            g_type: 0 as std::ffi::c_int as GType,
            data: [GValue_data { v_int: 0 }, GValue_data { v_int: 0 }],
        };
        init
    };
    g_value_init(&mut align, G_TYPE_ENUM as GType);
    g_value_set_int(&mut align, GTK_ALIGN_CENTER as std::ffi::c_int);
    g_object_set_property(
        g_type_check_instance_cast(
            label as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
        b"halign\0" as *const u8 as *const std::ffi::c_char,
        &mut align,
    );
    gtk_widget_set_vexpand(
        g_type_check_instance_cast(label as *mut GTypeInstance, gtk_widget_get_type())
            as *mut std::ffi::c_void as *mut GtkWidget,
        GTRUE,
    );
    let mut entry = gtk_entry_new();
    gtk_entry_set_activates_default(
        g_type_check_instance_cast(entry as *mut GTypeInstance, gtk_entry_get_type())
            as *mut std::ffi::c_void as *mut GtkEntry,
        GTRUE,
    );
    if !value.is_null() {
        gtk_entry_set_text(
            g_type_check_instance_cast(entry as *mut GTypeInstance, gtk_entry_get_type())
                as *mut std::ffi::c_void as *mut GtkEntry,
            value,
        );
    }
    gtk_grid_attach(
        g_type_check_instance_cast(table as *mut GTypeInstance, gtk_grid_get_type())
            as *mut std::ffi::c_void as *mut GtkGrid,
        label,
        0 as std::ffi::c_int,
        row,
        1 as std::ffi::c_int,
        1 as std::ffi::c_int,
    );
    gtk_grid_attach(
        g_type_check_instance_cast(table as *mut GTypeInstance, gtk_grid_get_type())
            as *mut std::ffi::c_void as *mut GtkGrid,
        entry,
        1 as std::ffi::c_int,
        row,
        1 as std::ffi::c_int,
        1 as std::ffi::c_int,
    );
    gtk_widget_set_halign(label, GTK_ALIGN_FILL);
    gtk_widget_set_valign(label, GTK_ALIGN_FILL);
    gtk_widget_set_halign(entry, GTK_ALIGN_FILL);
    gtk_widget_set_valign(entry, GTK_ALIGN_FILL);
    gtk_widget_set_vexpand(label, GTRUE);
    gtk_widget_set_vexpand(entry, GTRUE);
    return entry;
}
pub unsafe extern "C" fn show_auth_dialog(
    mut auth_data: *mut LuakitAuthData,
    mut login: *const std::ffi::c_char,
    mut password: *const std::ffi::c_char,
) {
    let mut widget = gtk_dialog_new();
    let mut window = g_type_check_instance_cast(widget as *mut GTypeInstance, gtk_window_get_type())
        as *mut std::ffi::c_void as *mut GtkWindow;
    let mut dialog = g_type_check_instance_cast(widget as *mut GTypeInstance, gtk_dialog_get_type())
        as *mut std::ffi::c_void as *mut GtkDialog;
    gtk_dialog_add_buttons(
        dialog,
        b"_Cancel\0" as *const u8 as *const std::ffi::c_char,
        GTK_RESPONSE_CANCEL as std::ffi::c_int,
        b"_OK\0" as *const u8 as *const std::ffi::c_char,
        GTK_RESPONSE_OK as std::ffi::c_int,
        // std::ptr::null_mut(),
    );
    gtk_container_set_border_width(
        g_type_check_instance_cast(dialog as *mut GTypeInstance, gtk_container_get_type())
            as *mut std::ffi::c_void as *mut GtkContainer,
        5 as std::ffi::c_int as guint,
    );
    let mut button_spacing = {
        let mut init = GValue {
            g_type: 0 as std::ffi::c_int as GType,
            data: [GValue_data { v_int: 0 }, GValue_data { v_int: 0 }],
        };
        init
    };
    g_value_init(&mut button_spacing, G_TYPE_INT as GType);
    g_value_set_int(&mut button_spacing, 6 as std::ffi::c_int);
    g_object_set_property(
        g_type_check_instance_cast(
            dialog as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
        b"button-spacing\0" as *const u8 as *const std::ffi::c_char,
        &mut button_spacing,
    );
    gtk_window_set_resizable(window, GFALSE);
    gtk_window_set_title(window, b"\0" as *const u8 as *const std::ffi::c_char);
    gtk_window_set_icon_name(
        window,
        b"dialog-password\0" as *const u8 as *const std::ffi::c_char,
    );
    gtk_dialog_set_default_response(dialog, GTK_RESPONSE_OK as std::ffi::c_int);
    let mut hbox = gtk_grid_new();
    let mut margin = {
        let mut init = GValue {
            g_type: 0 as std::ffi::c_int as GType,
            data: [GValue_data { v_int: 0 }, GValue_data { v_int: 0 }],
        };
        init
    };
    g_value_init(&mut margin, G_TYPE_INT as GType);
    g_value_set_int(&mut margin, 5 as std::ffi::c_int);
    g_object_set_property(
        g_type_check_instance_cast(
            hbox as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
        b"margin\0" as *const u8 as *const std::ffi::c_char,
        &mut margin,
    );
    gtk_grid_set_column_spacing(
        g_type_check_instance_cast(hbox as *mut GTypeInstance, gtk_grid_get_type())
            as *mut std::ffi::c_void as *mut GtkGrid,
        12 as std::ffi::c_int as guint,
    );
    gtk_box_pack_start(
        g_type_check_instance_cast(
            gtk_dialog_get_content_area(dialog) as *mut GTypeInstance,
            gtk_box_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkBox,
        hbox,
        GTRUE,
        GTRUE,
        0 as std::ffi::c_int as guint,
    );
    let mut icon = gtk_image_new_from_icon_name(
        b"dialog-password\0" as *const u8 as *const std::ffi::c_char,
        GTK_ICON_SIZE_DIALOG,
    );
    let mut align = {
        let mut init = GValue {
            g_type: 0 as std::ffi::c_int as GType,
            data: [GValue_data { v_int: 0 }, GValue_data { v_int: 0 }],
        };
        init
    };
    g_value_init(&mut align, G_TYPE_ENUM as GType);
    g_value_set_int(&mut align, GTK_ALIGN_CENTER as std::ffi::c_int);
    g_object_set_property(
        g_type_check_instance_cast(
            hbox as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
        b"halign\0" as *const u8 as *const std::ffi::c_char,
        &mut align,
    );
    gtk_grid_attach(
        g_type_check_instance_cast(hbox as *mut GTypeInstance, gtk_grid_get_type())
            as *mut std::ffi::c_void as *mut GtkGrid,
        icon,
        0 as std::ffi::c_int,
        0 as std::ffi::c_int,
        1 as std::ffi::c_int,
        2 as std::ffi::c_int,
    );
    gtk_grid_set_row_spacing(
        g_type_check_instance_cast(hbox as *mut GTypeInstance, gtk_grid_get_type())
            as *mut std::ffi::c_void as *mut GtkGrid,
        6 as std::ffi::c_int as guint,
    );
    let mut msg = g_strdup_printf(
        b"A username and password are being requested by the site %s\0" as *const u8
            as *const std::ffi::c_char,
        webkit_authentication_request_get_host((*auth_data).request),
    );
    let mut msg_label = gtk_label_new(msg);
    g_free(msg as gpointer);
    g_object_set_property(
        g_type_check_instance_cast(
            msg_label as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
        b"halign\0" as *const u8 as *const std::ffi::c_char,
        &mut align,
    );
    gtk_label_set_line_wrap(
        g_type_check_instance_cast(msg_label as *mut GTypeInstance, gtk_label_get_type())
            as *mut std::ffi::c_void as *mut GtkLabel,
        GTRUE,
    );
    let mut max_width_chars = {
        let mut init = GValue {
            g_type: 0 as std::ffi::c_int as GType,
            data: [GValue_data { v_int: 0 }, GValue_data { v_int: 0 }],
        };
        init
    };
    g_value_init(&mut max_width_chars, G_TYPE_INT as GType);
    g_value_set_int(&mut max_width_chars, 32 as std::ffi::c_int);
    g_object_set_property(
        g_type_check_instance_cast(
            msg_label as *mut GTypeInstance,
            ((20 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
        ) as *mut std::ffi::c_void as *mut GObject,
        b"max-width-chars\0" as *const u8 as *const std::ffi::c_char,
        &mut max_width_chars,
    );
    gtk_grid_attach_next_to(
        g_type_check_instance_cast(hbox as *mut GTypeInstance, gtk_grid_get_type())
            as *mut std::ffi::c_void as *mut GtkGrid,
        g_type_check_instance_cast(msg_label as *mut GTypeInstance, gtk_widget_get_type())
            as *mut std::ffi::c_void as *mut GtkWidget,
        icon,
        GTK_POS_RIGHT,
        1 as std::ffi::c_int,
        1 as std::ffi::c_int,
    );
    gtk_widget_set_hexpand(
        g_type_check_instance_cast(msg_label as *mut GTypeInstance, gtk_widget_get_type())
            as *mut std::ffi::c_void as *mut GtkWidget,
        GFALSE,
    );
    gtk_widget_set_vexpand(
        g_type_check_instance_cast(msg_label as *mut GTypeInstance, gtk_widget_get_type())
            as *mut std::ffi::c_void as *mut GtkWidget,
        GTRUE,
    );
    let mut table = gtk_grid_new();
    gtk_grid_attach_next_to(
        g_type_check_instance_cast(hbox as *mut GTypeInstance, gtk_grid_get_type())
            as *mut std::ffi::c_void as *mut GtkGrid,
        table,
        g_type_check_instance_cast(msg_label as *mut GTypeInstance, gtk_widget_get_type())
            as *mut std::ffi::c_void as *mut GtkWidget,
        GTK_POS_BOTTOM,
        1 as std::ffi::c_int,
        1 as std::ffi::c_int,
    );
    gtk_grid_set_column_homogeneous(
        g_type_check_instance_cast(table as *mut GTypeInstance, gtk_grid_get_type())
            as *mut std::ffi::c_void as *mut GtkGrid,
        GFALSE,
    );
    gtk_grid_set_row_homogeneous(
        g_type_check_instance_cast(table as *mut GTypeInstance, gtk_grid_get_type())
            as *mut std::ffi::c_void as *mut GtkGrid,
        GFALSE,
    );
    gtk_grid_set_column_spacing(
        g_type_check_instance_cast(table as *mut GTypeInstance, gtk_grid_get_type())
            as *mut std::ffi::c_void as *mut GtkGrid,
        12 as std::ffi::c_int as guint,
    );
    gtk_grid_set_row_spacing(
        g_type_check_instance_cast(table as *mut GTypeInstance, gtk_grid_get_type())
            as *mut std::ffi::c_void as *mut GtkGrid,
        6 as std::ffi::c_int as guint,
    );
    (*auth_data).login_entry = table_add_entry(
        table,
        0 as std::ffi::c_int,
        b"Username:\0" as *const u8 as *const std::ffi::c_char,
        login,
        std::ptr::null_mut(),
    );
    (*auth_data).password_entry = table_add_entry(
        table,
        1 as std::ffi::c_int,
        b"Password:\0" as *const u8 as *const std::ffi::c_char,
        password,
        std::ptr::null_mut(),
    );
    gtk_entry_set_visibility(
        g_type_check_instance_cast(
            (*auth_data).password_entry as *mut GTypeInstance,
            gtk_entry_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkEntry,
        GFALSE,
    );
    let mut checkbutton = gtk_check_button_new_with_label(
        b"Store password\0" as *const u8 as *const std::ffi::c_char,
    );
    gtk_label_set_line_wrap(
        g_type_check_instance_cast(
            gtk_bin_get_child(g_type_check_instance_cast(
                checkbutton as *mut GTypeInstance,
                gtk_bin_get_type(),
            ) as *mut std::ffi::c_void as *mut GtkBin) as *mut GTypeInstance,
            gtk_label_get_type(),
        ) as *mut std::ffi::c_void as *mut GtkLabel,
        GTRUE,
    );
    gtk_grid_attach_next_to(
        g_type_check_instance_cast(hbox as *mut GTypeInstance, gtk_grid_get_type())
            as *mut std::ffi::c_void as *mut GtkGrid,
        checkbutton,
        table,
        GTK_POS_BOTTOM,
        1 as std::ffi::c_int,
        1 as std::ffi::c_int,
    );
    (*auth_data).checkbutton = checkbutton;
    g_signal_connect_data(
        dialog as *mut GObject,
        b"response\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut GtkDialog, gint, *mut LuakitAuthData) -> ()>,
            GCallback,
        >(Some(
            response_callback
                as unsafe extern "C" fn(*mut GtkDialog, gint, *mut LuakitAuthData) -> (),
        )),
        auth_data as gpointer,
        None,
        G_CONNECT_DEFAULT,
    );
    gtk_widget_show_all(widget);
}
pub unsafe extern "C" fn session_authenticate(
    mut UNUSED_web_view: *mut WebKitWebView,
    mut request: *mut WebKitAuthenticationRequest,
    mut w: *mut widget_t,
) -> gboolean {
    g_object_ref(request as *mut GObject);
    let mut auth_data =
        g_slice_alloc(::core::mem::size_of::<LuakitAuthData>()) as *mut LuakitAuthData;
    (*auth_data).request = request;
    (*auth_data).w = w;
    let mut login = std::ptr::null_mut();
    let mut password = std::ptr::null_mut();
    luakit_find_password(auth_data, login, password);
    show_auth_dialog(auth_data, login as *const i8, password as *const i8);
    return GTRUE;
}
