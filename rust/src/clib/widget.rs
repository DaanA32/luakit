use std::ffi::CStr;

use glib_sys::{GType, g_assertion_message_expr, g_free, g_strdup, g_strdup_printf, gpointer};
use gobject_sys::{GObject, GTypeInstance, g_object_set_data, g_type_check_instance_cast};
use gtk_sys::{
    GTK_STYLE_PROVIDER_PRIORITY_APPLICATION, GtkStyleProvider, GtkWidget,
    gtk_css_provider_load_from_data, gtk_css_provider_new, gtk_css_provider_to_string,
    gtk_style_context_add_provider, gtk_style_provider_get_type, gtk_widget_get_style_context,
    gtk_widget_get_type, gtk_widget_set_name,
};
use libc::{c_void, memset, strlen};
use mlua::ffi::{
    LUA_MULTRET, lua_State, lua_createtable, lua_gettop, lua_newuserdata, lua_pushboolean,
    lua_pushstring, lua_pushvalue, lua_setmetatable, lua_settop, lua_touserdata, luaL_Reg,
    luaL_checklstring, luaL_error,
};
use mlua::lua_CFunction;

use crate::common::common;
use crate::common::luaobject::luaH_object_ref_class;
use crate::common::tokenize::{
    L_TK_CAN_FOCUS, L_TK_CSS, L_TK_DRAWING_AREA, L_TK_ENTRY, L_TK_EVENTBOX, L_TK_HBOX, L_TK_HPANED,
    L_TK_IMAGE, L_TK_IS_ALIVE, L_TK_LABEL, L_TK_MARGIN, L_TK_MARGIN_BOTTOM, L_TK_MARGIN_LEFT,
    L_TK_MARGIN_RIGHT, L_TK_MARGIN_TOP, L_TK_NOTEBOOK, L_TK_OVERLAY, L_TK_SCROLLED, L_TK_SPINNER,
    L_TK_STACK, L_TK_TYPE, L_TK_UNKNOWN, L_TK_VBOX, L_TK_VPANED, L_TK_WEBVIEW, L_TK_WINDOW,
    l_tokenize,
};

use crate::widgets::r#box::widget_box;
use crate::widgets::common::GOBJECT_LUAKIT_WIDGET_DATA_KEY;
use crate::widgets::drawing_area::widget_drawing_area;
use crate::widgets::entry::widget_entry;
use crate::widgets::eventbox::widget_eventbox;
use crate::widgets::image::widget_image;
use crate::widgets::label::widget_label;
use crate::widgets::notebook::widget_notebook;
use crate::widgets::overlay::widget_overlay;
use crate::widgets::paned::widget_paned;
use crate::widgets::scrolled::widget_scrolled;
use crate::widgets::spinner::widget_spinner;
use crate::widgets::stack::widget_stack;
use crate::widgets::webview::widget_webview;
use crate::widgets::window::widget_window;
use crate::widgets::{
    luaH_checkwidget, widget_class, widget_constructor_t, widget_info_t, widget_t,
};
use crate::{
    common::{
        luaclass::{
            lua_class_allocator_t, lua_class_propfunc_t, luaH_checkudata, luaH_class_add_property,
            luaH_class_add_signal, luaH_class_emit_signal, luaH_class_index, luaH_class_new,
            luaH_class_newindex, luaH_class_remove_signal, luaH_class_setup, signal_h::signal_new,
        },
        luaobject::{
            luaH_object_add_signal_simple, luaH_object_emit_signal_simple, luaH_object_gc,
            luaH_object_property_signal, luaH_object_remove_signal_simple,
            luaH_object_remove_signals_simple, luaH_object_tostring, luaH_settype,
        },
        messages::G_LOG_DOMAIN,
        property::{BOOL, INT, luaH_gobject_index, luaH_gobject_newindex, property_t},
        tokenize::*,
    },
    gtypes::{gchar, gint, guint},
    log::{_log, LOG_LEVEL_debug, LOG_LEVEL_verbose},
};

static mut widget_properties: [property_t; 7] = [
    {
        let mut init = property_t {
            tok: L_TK_MARGIN,
            name: b"margin\0" as *const u8 as *const std::ffi::c_char,
            type_0: INT,
            writable: 1,
        };
        init
    },
    {
        let mut init = property_t {
            tok: L_TK_MARGIN_TOP,
            name: b"margin-top\0" as *const u8 as *const std::ffi::c_char,
            type_0: INT,
            writable: 1,
        };
        init
    },
    {
        let mut init = property_t {
            tok: L_TK_MARGIN_BOTTOM,
            name: b"margin-bottom\0" as *const u8 as *const std::ffi::c_char,
            type_0: INT,
            writable: 1,
        };
        init
    },
    {
        let mut init = property_t {
            tok: L_TK_MARGIN_LEFT,
            name: b"margin-left\0" as *const u8 as *const std::ffi::c_char,
            type_0: INT,
            writable: 1,
        };
        init
    },
    {
        let mut init = property_t {
            tok: L_TK_MARGIN_RIGHT,
            name: b"margin-right\0" as *const u8 as *const std::ffi::c_char,
            type_0: INT,
            writable: 1,
        };
        init
    },
    {
        let mut init = property_t {
            tok: L_TK_CAN_FOCUS,
            name: b"can-focus\0" as *const u8 as *const std::ffi::c_char,
            type_0: BOOL,
            writable: 1,
        };
        init
    },
    {
        let mut init = property_t {
            tok: L_TK_UNKNOWN,
            name: std::ptr::null(),
            type_0: BOOL,
            writable: 0,
        };
        init
    },
];
static mut widgets_list: [widget_info_t; 16] = unsafe {
    [
        {
            let mut init = widget_info_t {
                tok: L_TK_ENTRY,
                name: b"entry\0" as *const u8 as *const std::ffi::c_char,
                wc: Some(widget_entry),
            };
            init
        },
        {
            let mut init = widget_info_t {
                tok: L_TK_EVENTBOX,
                name: b"eventbox\0" as *const u8 as *const std::ffi::c_char,
                wc: Some(widget_eventbox),
            };
            init
        },
        {
            let mut init = widget_info_t {
                tok: L_TK_HBOX,
                name: b"hbox\0" as *const u8 as *const std::ffi::c_char,
                wc: Some(widget_box),
            };
            init
        },
        {
            let mut init = widget_info_t {
                tok: L_TK_HPANED,
                name: b"hpaned\0" as *const u8 as *const std::ffi::c_char,
                wc: Some(widget_paned),
            };
            init
        },
        {
            let mut init = widget_info_t {
                tok: L_TK_LABEL,
                name: b"label\0" as *const u8 as *const std::ffi::c_char,
                wc: Some(widget_label),
            };
            init
        },
        {
            let mut init = widget_info_t {
                tok: L_TK_NOTEBOOK,
                name: b"notebook\0" as *const u8 as *const std::ffi::c_char,
                wc: Some(widget_notebook),
            };
            init
        },
        {
            let mut init = widget_info_t {
                tok: L_TK_VBOX,
                name: b"vbox\0" as *const u8 as *const std::ffi::c_char,
                wc: Some(widget_box),
            };
            init
        },
        {
            let mut init = widget_info_t {
                tok: L_TK_VPANED,
                name: b"vpaned\0" as *const u8 as *const std::ffi::c_char,
                wc: Some(widget_paned),
            };
            init
        },
        {
            let mut init = widget_info_t {
                tok: L_TK_WEBVIEW,
                name: b"webview\0" as *const u8 as *const std::ffi::c_char,
                wc: Some(widget_webview),
            };
            init
        },
        {
            let mut init = widget_info_t {
                tok: L_TK_WINDOW,
                name: b"window\0" as *const u8 as *const std::ffi::c_char,
                wc: Some(widget_window),
            };
            init
        },
        {
            let mut init = widget_info_t {
                tok: L_TK_OVERLAY,
                name: b"overlay\0" as *const u8 as *const std::ffi::c_char,
                wc: Some(widget_overlay),
            };
            init
        },
        {
            let mut init = widget_info_t {
                tok: L_TK_SCROLLED,
                name: b"scrolled\0" as *const u8 as *const std::ffi::c_char,
                wc: Some(widget_scrolled),
            };
            init
        },
        {
            let mut init = widget_info_t {
                tok: L_TK_IMAGE,
                name: b"image\0" as *const u8 as *const std::ffi::c_char,
                wc: Some(widget_image),
            };
            init
        },
        {
            let mut init = widget_info_t {
                tok: L_TK_SPINNER,
                name: b"spinner\0" as *const u8 as *const std::ffi::c_char,
                wc: Some(widget_spinner),
            };
            init
        },
        {
            let mut init = widget_info_t {
                tok: L_TK_DRAWING_AREA,
                name: b"drawing_area\0" as *const u8 as *const std::ffi::c_char,
                wc: Some(widget_drawing_area),
            };
            init
        },
        {
            let mut init = widget_info_t {
                tok: L_TK_STACK,
                name: b"stack\0" as *const u8 as *const std::ffi::c_char,
                wc: Some(widget_stack),
            };
            init
        },
    ]
};
#[inline]
unsafe extern "C-unwind" fn luaH_widget_class_emit_signal(mut L: *mut lua_State) -> gint {
    return luaH_class_emit_signal(
        L,
        &mut widget_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, std::ptr::null_mut()),
        lua_gettop(L) - 1 as std::ffi::c_int,
        LUA_MULTRET,
    );
}
#[inline]
unsafe extern "C-unwind" fn luaH_widget_class_remove_signal(mut L: *mut lua_State) -> gint {
    luaH_class_remove_signal(
        L,
        &mut widget_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, std::ptr::null_mut()),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn luaH_widget_class_add_signal(mut L: *mut lua_State) -> gint {
    luaH_class_add_signal(
        L,
        &mut widget_class,
        luaL_checklstring(L, 1 as std::ffi::c_int, std::ptr::null_mut()),
        2 as std::ffi::c_int,
    );
    return 0 as std::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn widget_new(mut L: *mut lua_State) -> *mut widget_t {
    let mut p = lua_newuserdata(L, ::core::mem::size_of::<widget_t>()) as *mut widget_t;
    memset(
        p as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        (::core::mem::size_of::<widget_t>()).wrapping_mul(1),
    );
    (*p).signals = signal_new();
    luaH_settype(L, &mut widget_class);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
    lua_setmetatable(L, -(2 as std::ffi::c_int));
    // lua_setfenv(L, -(2 as std::ffi::c_int));
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
unsafe extern "C-unwind" fn luaH_widget_gc(mut L: *mut lua_State) -> gint {
    let mut w = luaH_checkudata(L, 1 as std::ffi::c_int, &mut widget_class) as *mut widget_t;
    if !((*w).info).is_null() {
        _log(
            LOG_LEVEL_debug,
            b"clib/widget.c\0" as *const u8 as *const std::ffi::c_char,
            &format!(
                "collecting widget at {} of type '{}'",
                w as usize,
                CStr::from_ptr((*(*w).info).name).to_string_lossy()
            ),
        );
    }
    if ((*w).destructor).is_none() {
    } else {
        g_assertion_message_expr(
            G_LOG_DOMAIN as *const std::ffi::c_char,
            b"clib/widget.c\0" as *const u8 as *const std::ffi::c_char,
            66 as std::ffi::c_int,
            (*::core::mem::transmute::<&[u8; 15], &[std::ffi::c_char; 15]>(b"luaH_widget_gc\0"))
                .as_ptr(),
            b"!w->destructor\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    return luaH_object_gc(L);
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn luaH_widget_new(mut L: *mut lua_State) -> gint {
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
unsafe extern "C-unwind" fn widget_set_css(mut w: *mut widget_t, mut properties: *const gchar) {
    let mut old_css = gtk_css_provider_to_string((*w).provider);
    let css = g_strdup_printf(
        b"%s\n#widget { %s }\0" as *const u8 as *const std::ffi::c_char,
        old_css,
        properties,
    );
    gtk_css_provider_load_from_data(
        (*w).provider,
        css as *const u8,
        strlen(css).try_into().unwrap(),
        std::ptr::null_mut(),
    );
    g_free(css as gpointer);
    g_free(old_css as gpointer);
}
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn widget_set_css_properties(mut w: *mut widget_t, mut args: ...) {
    let mut argp: ::core::ffi::VaListImpl;
    argp = args.clone();
    let mut css = g_strdup(b"\0" as *const u8 as *const std::ffi::c_char);
    let mut prop = 0 as *const gchar;
    loop {
        prop = argp.arg::<*mut gchar>();
        if prop.is_null() {
            break;
        }
        let mut value: *const gchar = argp.arg::<*mut gchar>();
        if strlen(prop) > 0 {
        } else {
            g_assertion_message_expr(
                G_LOG_DOMAIN as *const std::ffi::c_char,
                b"clib/widget.c\0" as *const u8 as *const std::ffi::c_char,
                116 as std::ffi::c_int,
                (*::core::mem::transmute::<&[u8; 26], &[std::ffi::c_char; 26]>(
                    b"widget_set_css_properties\0",
                ))
                .as_ptr(),
                b"strlen(prop) > 0\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        if value.is_null() || strlen(value) == 0 {
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
unsafe extern "C-unwind" fn luaH_widget_index(mut L: *mut lua_State) -> gint {
    let mut prop = luaL_checklstring(L, 2 as std::ffi::c_int, std::ptr::null_mut());
    let mut token = l_tokenize(prop);
    if luaH_class_index(L) != 0 {
        return 1 as std::ffi::c_int;
    }
    if token as std::ffi::c_uint == L_TK_IS_ALIVE as std::ffi::c_int as std::ffi::c_uint {
        let mut w = luaH_checkudata(L, 1 as std::ffi::c_int, &mut widget_class) as *mut widget_t;
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
unsafe extern "C-unwind" fn luaH_widget_newindex(mut L: *mut lua_State) -> gint {
    let mut prop = luaL_checklstring(L, 2 as std::ffi::c_int, std::ptr::null_mut());
    let mut token = l_tokenize(prop);
    luaH_class_newindex(L);
    let mut widget = luaH_checkwidget(L, 1 as std::ffi::c_int);
    if token as std::ffi::c_uint == L_TK_CSS as std::ffi::c_int as std::ffi::c_uint {
        widget_set_css(
            widget,
            luaL_checklstring(L, 3 as std::ffi::c_int, std::ptr::null_mut()),
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
unsafe extern "C-unwind" fn luaH_widget_set_type(
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
    let mut type_0 = luaL_checklstring(L, -(1 as std::ffi::c_int), std::ptr::null_mut());
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
                g_type_check_instance_cast((*w).widget as *mut GTypeInstance, gtk_widget_get_type())
                    as *mut std::ffi::c_void as *mut GtkWidget,
                b"widget\0" as *const u8 as *const std::ffi::c_char,
            );
            let mut context = gtk_widget_get_style_context(g_type_check_instance_cast(
                (*w).widget as *mut GTypeInstance,
                gtk_widget_get_type(),
            ) as *mut std::ffi::c_void
                as *mut GtkWidget);
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
                &format!(
                    "created widget of type: {}",
                    CStr::from_ptr((*(*w).info).name).to_string_lossy()
                ),
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
unsafe extern "C-unwind" fn luaH_widget_get_type(
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
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn widget_class_setup(mut L: *mut lua_State) {
    let widget_methods = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"add_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_widget_class_add_signal,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_widget_class_remove_signal,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"emit_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_widget_class_emit_signal,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__call\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_widget_new,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: 0 as *const std::ffi::c_char,
                    func: core::mem::transmute::<libc::intptr_t, lua_CFunction>(
                        0 as libc::intptr_t,
                    ),
                };
                init
            },
            // {
            //     let mut init = luaL_Reg {
            //         name: std::ptr::null(),
            //         func: None,
            //     };
            //     init
            // },
        ]
    };
    let widget_meta = unsafe {
        [
            {
                let mut init = luaL_Reg {
                    name: b"__tostring\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_object_tostring,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"add_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_object_add_signal_simple,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_object_remove_signal_simple,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"remove_signals\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_object_remove_signals_simple,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"emit_signal\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_object_emit_signal_simple,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__index\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_widget_index,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__newindex\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_widget_newindex,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: b"__gc\0" as *const u8 as *const std::ffi::c_char,
                    func: luaH_widget_gc,
                };
                init
            },
            {
                let mut init = luaL_Reg {
                    name: 0 as *const std::ffi::c_char,
                    func: core::mem::transmute::<libc::intptr_t, lua_CFunction>(
                        0 as libc::intptr_t,
                    ),
                };
                init
            },
            // {
            //     let mut init = luaL_Reg {
            //         name: std::ptr::null(),
            //         func: None,
            //     };
            //     init
            // },
        ]
    };
    luaH_class_setup(
        L,
        &mut widget_class,
        b"widget\0" as *const u8 as *const std::ffi::c_char,
        ::core::mem::transmute::<
            Option<unsafe extern "C-unwind" fn(*mut lua_State) -> *mut widget_t>,
            lua_class_allocator_t,
        >(Some(
            widget_new as unsafe extern "C-unwind" fn(*mut lua_State) -> *mut widget_t,
        )),
        None,
        None,
        widget_methods.as_ptr(),
        widget_meta.as_ptr(),
    );
    luaH_class_add_property(
        &mut widget_class,
        L_TK_TYPE,
        ::core::mem::transmute::<
            Option<unsafe extern "C-unwind" fn(*mut lua_State, *mut widget_t) -> gint>,
            lua_class_propfunc_t,
        >(Some(
            luaH_widget_set_type
                as unsafe extern "C-unwind" fn(*mut lua_State, *mut widget_t) -> gint,
        )),
        ::core::mem::transmute::<
            Option<unsafe extern "C-unwind" fn(*mut lua_State, *mut widget_t) -> gint>,
            lua_class_propfunc_t,
        >(Some(
            luaH_widget_get_type
                as unsafe extern "C-unwind" fn(*mut lua_State, *mut widget_t) -> gint,
        )),
        None,
    );
}
