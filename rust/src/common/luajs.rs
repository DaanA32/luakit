use ::libc;
#[c2rust::header_src = "/usr/lib/clang/20/include/__stddef_ptrdiff_t.h:19"]
pub mod __stddef_ptrdiff_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type ptrdiff_t = std::ffi::c_long;
}
#[c2rust::header_src = "/usr/lib/clang/20/include/__stddef_size_t.h:19"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = std::ffi::c_ulong;
}
#[c2rust::header_src = "/usr/lib/glib-2.0/include/glibconfig.h:19"]
pub mod glibconfig_h {
    #[c2rust::src_loc = "82:1"]
    pub type gssize = std::ffi::c_long;
    #[c2rust::src_loc = "83:1"]
    pub type gsize = std::ffi::c_ulong;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtypes.h:19"]
pub mod gtypes_h {
    #[c2rust::src_loc = "52:1"]
    pub type gchar = std::ffi::c_char;
    #[c2rust::src_loc = "55:1"]
    pub type gint = std::ffi::c_int;
    #[c2rust::src_loc = "56:1"]
    pub type gboolean = gint;
    #[c2rust::src_loc = "61:1"]
    pub type guint = std::ffi::c_uint;
    #[c2rust::src_loc = "109:1"]
    pub type gpointer = *mut std::ffi::c_void;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gdataset.h:19"]
pub mod gdataset_h {
    #[c2rust::src_loc = "38:1"]
    pub type GData = _GData;
    extern "C" {
        #[c2rust::src_loc = "38:16"]
        pub type _GData;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gtype.h:19"]
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
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gobject.h:19"]
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
        #[c2rust::src_loc = "514:1"]
        pub fn g_object_unref(object: gpointer);
    }
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/jsc/JSCValue.h:19"]
pub mod JSCValue_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "48:1"]
    pub struct _JSCValue {
        pub parent: GObject,
        pub priv_0: *mut JSCValuePrivate,
    }
    #[c2rust::src_loc = "48:1"]
    pub type JSCValuePrivate = _JSCValuePrivate;
    #[c2rust::src_loc = "48:1"]
    pub type JSCValue = _JSCValue;
    #[c2rust::src_loc = "50:1"]
    pub type JSCClass = _JSCClass;
    #[c2rust::src_loc = "51:1"]
    pub type JSCContext = _JSCContext;
    use super::gobject_h::GObject;
    use super::JSCClass_h::_JSCClass;
    use super::JSCContext_h::_JSCContext;
    use super::gtypes_h::{gboolean, gpointer, guint, gchar};
    use super::gtype_h::GType;
    extern "C" {
        #[c2rust::src_loc = "48:1"]
        pub type _JSCValuePrivate;
        #[c2rust::src_loc = "77:1"]
        pub fn jsc_value_new_undefined(context: *mut JSCContext) -> *mut JSCValue;
        #[c2rust::src_loc = "80:1"]
        pub fn jsc_value_is_undefined(value: *mut JSCValue) -> gboolean;
        #[c2rust::src_loc = "83:1"]
        pub fn jsc_value_new_null(context: *mut JSCContext) -> *mut JSCValue;
        #[c2rust::src_loc = "86:1"]
        pub fn jsc_value_is_null(value: *mut JSCValue) -> gboolean;
        #[c2rust::src_loc = "89:1"]
        pub fn jsc_value_new_number(
            context: *mut JSCContext,
            number: std::ffi::c_double,
        ) -> *mut JSCValue;
        #[c2rust::src_loc = "92:1"]
        pub fn jsc_value_is_number(value: *mut JSCValue) -> gboolean;
        #[c2rust::src_loc = "95:1"]
        pub fn jsc_value_to_double(value: *mut JSCValue) -> std::ffi::c_double;
        #[c2rust::src_loc = "101:1"]
        pub fn jsc_value_new_boolean(
            context: *mut JSCContext,
            value: gboolean,
        ) -> *mut JSCValue;
        #[c2rust::src_loc = "104:1"]
        pub fn jsc_value_is_boolean(value: *mut JSCValue) -> gboolean;
        #[c2rust::src_loc = "107:1"]
        pub fn jsc_value_to_boolean(value: *mut JSCValue) -> gboolean;
        #[c2rust::src_loc = "110:1"]
        pub fn jsc_value_new_string(
            context: *mut JSCContext,
            string: *const std::ffi::c_char,
        ) -> *mut JSCValue;
        #[c2rust::src_loc = "118:1"]
        pub fn jsc_value_is_string(value: *mut JSCValue) -> gboolean;
        #[c2rust::src_loc = "121:1"]
        pub fn jsc_value_to_string(value: *mut JSCValue) -> *mut std::ffi::c_char;
        #[c2rust::src_loc = "127:1"]
        pub fn jsc_value_new_array(
            context: *mut JSCContext,
            first_item_type: GType,
            _: ...
        ) -> *mut JSCValue;
        #[c2rust::src_loc = "143:1"]
        pub fn jsc_value_new_object(
            context: *mut JSCContext,
            instance: gpointer,
            jsc_class: *mut JSCClass,
        ) -> *mut JSCValue;
        #[c2rust::src_loc = "148:1"]
        pub fn jsc_value_is_object(value: *mut JSCValue) -> gboolean;
        #[c2rust::src_loc = "155:1"]
        pub fn jsc_value_object_set_property(
            value: *mut JSCValue,
            name: *const std::ffi::c_char,
            property: *mut JSCValue,
        );
        #[c2rust::src_loc = "160:1"]
        pub fn jsc_value_object_get_property(
            value: *mut JSCValue,
            name: *const std::ffi::c_char,
        ) -> *mut JSCValue;
        #[c2rust::src_loc = "164:1"]
        pub fn jsc_value_object_set_property_at_index(
            value: *mut JSCValue,
            index: guint,
            property: *mut JSCValue,
        );
        #[c2rust::src_loc = "181:1"]
        pub fn jsc_value_object_enumerate_properties(
            value: *mut JSCValue,
        ) -> *mut *mut gchar;
    }
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/jsc/JSCClass.h:19"]
pub mod JSCClass_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "37:1"]
    pub struct _JSCClass {
        pub parent: GObject,
        pub priv_0: *mut JSCClassPrivate,
    }
    #[c2rust::src_loc = "37:1"]
    pub type JSCClassPrivate = _JSCClassPrivate;
    use super::gobject_h::GObject;
    extern "C" {
        #[c2rust::src_loc = "37:1"]
        pub type _JSCClassPrivate;
    }
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/jsc/JSCContext.h:19"]
pub mod JSCContext_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "53:1"]
    pub struct _JSCContext {
        pub parent: GObject,
        pub priv_0: *mut JSCContextPrivate,
    }
    #[c2rust::src_loc = "53:1"]
    pub type JSCContextPrivate = _JSCContextPrivate;
    use super::gobject_h::GObject;
    use super::JSCValue_h::{JSCContext, JSCValue};
    use super::JSCException_h::JSCException;
    use super::glibconfig_h::gssize;
    use super::gtypes_h::guint;
    extern "C" {
        #[c2rust::src_loc = "53:1"]
        pub type _JSCContextPrivate;
        #[c2rust::src_loc = "82:1"]
        pub fn jsc_context_get_exception(context: *mut JSCContext) -> *mut JSCException;
        #[c2rust::src_loc = "129:1"]
        pub fn jsc_context_evaluate_with_source_uri(
            context: *mut JSCContext,
            code: *const std::ffi::c_char,
            length: gssize,
            uri: *const std::ffi::c_char,
            line_number: guint,
        ) -> *mut JSCValue;
    }
}
#[c2rust::header_src = "/usr/include/webkitgtk-4.1/jsc/JSCException.h:19"]
pub mod JSCException_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "48:1"]
    pub struct _JSCException {
        pub parent: GObject,
        pub priv_0: *mut JSCExceptionPrivate,
    }
    #[c2rust::src_loc = "48:1"]
    pub type JSCExceptionPrivate = _JSCExceptionPrivate;
    #[c2rust::src_loc = "48:1"]
    pub type JSCException = _JSCException;
    use super::gobject_h::GObject;
    extern "C" {
        #[c2rust::src_loc = "48:1"]
        pub type _JSCExceptionPrivate;
        #[c2rust::src_loc = "101:1"]
        pub fn jsc_exception_to_string(
            exception: *mut JSCException,
        ) -> *mut std::ffi::c_char;
    }
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lua.h:19"]
pub mod lua_h {
    #[c2rust::src_loc = "100:1"]
    pub type lua_Number = std::ffi::c_double;
    #[c2rust::src_loc = "104:1"]
    pub type lua_Integer = ptrdiff_t;
    use super::__stddef_ptrdiff_t_h::ptrdiff_t;
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "51:16"]
        pub type lua_State;
        #[c2rust::src_loc = "121:1"]
        pub fn lua_gettop(L: *mut lua_State) -> std::ffi::c_int;
        #[c2rust::src_loc = "122:1"]
        pub fn lua_settop(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "140:1"]
        pub fn lua_type(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;
        #[c2rust::src_loc = "147:1"]
        pub fn lua_tonumber(L: *mut lua_State, idx: std::ffi::c_int) -> lua_Number;
        #[c2rust::src_loc = "149:1"]
        pub fn lua_toboolean(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;
        #[c2rust::src_loc = "150:1"]
        pub fn lua_tolstring(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            len: *mut size_t,
        ) -> *const std::ffi::c_char;
        #[c2rust::src_loc = "151:1"]
        pub fn lua_objlen(L: *mut lua_State, idx: std::ffi::c_int) -> size_t;
        #[c2rust::src_loc = "161:1"]
        pub fn lua_pushnil(L: *mut lua_State);
        #[c2rust::src_loc = "162:1"]
        pub fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
        #[c2rust::src_loc = "163:1"]
        pub fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);
        #[c2rust::src_loc = "165:1"]
        pub fn lua_pushstring(L: *mut lua_State, s: *const std::ffi::c_char);
        #[c2rust::src_loc = "170:1"]
        pub fn lua_pushboolean(L: *mut lua_State, b: std::ffi::c_int);
        #[c2rust::src_loc = "182:1"]
        pub fn lua_createtable(
            L: *mut lua_State,
            narr: std::ffi::c_int,
            nrec: std::ffi::c_int,
        );
        #[c2rust::src_loc = "193:1"]
        pub fn lua_rawset(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "241:1"]
        pub fn lua_next(L: *mut lua_State, idx: std::ffi::c_int) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:19"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "177:17"]
        pub fn strtol(
            _: *const std::ffi::c_char,
            _: *mut *mut std::ffi::c_char,
            _: std::ffi::c_int,
        ) -> std::ffi::c_long;
        #[c2rust::src_loc = "687:13"]
        pub fn free(_: *mut std::ffi::c_void);
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gstrfuncs.h:19"]
pub mod gstrfuncs_h {
    use super::gtypes_h::gchar;
    extern "C" {
        #[c2rust::src_loc = "366:1"]
        pub fn g_strfreev(str_array: *mut *mut gchar);
    }
}
pub use self::__stddef_ptrdiff_t_h::ptrdiff_t;
pub use self::__stddef_size_t_h::size_t;
pub use self::glibconfig_h::{gssize, gsize};
pub use self::gtypes_h::{gchar, gint, gboolean, guint, gpointer};
pub use self::gdataset_h::{GData, _GData};
pub use self::gtype_h::{GType, _GTypeClass, GTypeClass, _GTypeInstance, GTypeInstance};
pub use self::gobject_h::{_GObject, GObject, g_object_unref};
pub use self::JSCValue_h::{
    _JSCValue, JSCValuePrivate, JSCValue, JSCClass, JSCContext, _JSCValuePrivate,
    jsc_value_new_undefined, jsc_value_is_undefined, jsc_value_new_null,
    jsc_value_is_null, jsc_value_new_number, jsc_value_is_number, jsc_value_to_double,
    jsc_value_new_boolean, jsc_value_is_boolean, jsc_value_to_boolean,
    jsc_value_new_string, jsc_value_is_string, jsc_value_to_string, jsc_value_new_array,
    jsc_value_new_object, jsc_value_is_object, jsc_value_object_set_property,
    jsc_value_object_get_property, jsc_value_object_set_property_at_index,
    jsc_value_object_enumerate_properties,
};
pub use self::JSCClass_h::{_JSCClass, JSCClassPrivate, _JSCClassPrivate};
pub use self::JSCContext_h::{
    _JSCContext, JSCContextPrivate, _JSCContextPrivate, jsc_context_get_exception,
    jsc_context_evaluate_with_source_uri,
};
pub use self::JSCException_h::{
    _JSCException, JSCExceptionPrivate, JSCException, _JSCExceptionPrivate,
    jsc_exception_to_string,
};
pub use self::lua_h::{
    lua_Number, lua_Integer, lua_State, lua_gettop, lua_settop, lua_type, lua_tonumber,
    lua_toboolean, lua_tolstring, lua_objlen, lua_pushnil, lua_pushnumber,
    lua_pushinteger, lua_pushstring, lua_pushboolean, lua_createtable, lua_rawset,
    lua_next,
};
use self::stdlib_h::{strtol, free};
use self::gstrfuncs_h::g_strfreev;
#[no_mangle]
#[c2rust::src_loc = "25:1"]
pub unsafe extern "C" fn luajs_tovalue(
    mut L: *mut lua_State,
    mut idx: std::ffi::c_int,
    mut ctx: *mut JSCContext,
) -> *mut JSCValue {
    match lua_type(L, idx) {
        1 => return jsc_value_new_boolean(ctx, lua_toboolean(L, idx)),
        3 => return jsc_value_new_number(ctx, lua_tonumber(L, idx)),
        0 => return jsc_value_new_null(ctx),
        -1 => return jsc_value_new_undefined(ctx),
        4 => return jsc_value_new_string(ctx, lua_tolstring(L, idx, 0 as *mut size_t)),
        5 => {
            let mut len: size_t = lua_objlen(L, idx);
            let mut top: std::ffi::c_int = lua_gettop(L);
            let mut res: *mut JSCValue = 0 as *mut JSCValue;
            let mut val: *mut JSCValue = 0 as *mut JSCValue;
            if idx < 0 as std::ffi::c_int {
                idx += top + 1 as std::ffi::c_int;
            }
            if len != 0 {
                res = jsc_value_new_array(
                    ctx,
                    ((1 as std::ffi::c_int) << 2 as std::ffi::c_int) as GType,
                );
                lua_pushnil(L);
                let mut i: std::ffi::c_int = 0 as std::ffi::c_int;
                while lua_next(L, idx) != 0 {
                    val = luajs_tovalue(L, -(1 as std::ffi::c_int), ctx);
                    if val.is_null() {
                        lua_settop(L, top);
                        g_object_unref(res as gpointer);
                        return 0 as *mut JSCValue;
                    }
                    let fresh0 = i;
                    i = i + 1;
                    jsc_value_object_set_property_at_index(res, fresh0 as guint, val);
                    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
                    g_object_unref(val as gpointer);
                }
            } else {
                res = jsc_value_new_object(
                    ctx,
                    0 as *mut std::ffi::c_void,
                    0 as *mut JSCClass,
                );
                lua_pushnil(L);
                while lua_next(L, idx) != 0 {
                    if lua_type(L, -(2 as std::ffi::c_int)) != 4 as std::ffi::c_int {
                        continue;
                    }
                    val = luajs_tovalue(L, -(1 as std::ffi::c_int), ctx);
                    if val.is_null() {
                        lua_settop(L, top);
                        g_object_unref(res as gpointer);
                        return 0 as *mut JSCValue;
                    }
                    jsc_value_object_set_property(
                        res,
                        lua_tolstring(L, -(2 as std::ffi::c_int), 0 as *mut size_t),
                        val,
                    );
                    lua_settop(L, -(1 as std::ffi::c_int) - 1 as std::ffi::c_int);
                    g_object_unref(val as gpointer);
                }
            }
            return res;
        }
        _ => {}
    }
    return 0 as *mut JSCValue;
}
#[no_mangle]
#[c2rust::src_loc = "88:1"]
pub unsafe extern "C" fn luajs_pushvalue(
    mut L: *mut lua_State,
    mut value: *mut JSCValue,
) -> std::ffi::c_int {
    if jsc_value_is_undefined(value) != 0 || jsc_value_is_null(value) != 0 {
        lua_pushnil(L);
    } else if jsc_value_is_boolean(value) != 0 {
        lua_pushboolean(L, jsc_value_to_boolean(value));
    } else if jsc_value_is_number(value) != 0 {
        lua_pushnumber(L, jsc_value_to_double(value));
    } else if jsc_value_is_string(value) != 0 {
        let mut str: *mut std::ffi::c_char = jsc_value_to_string(value);
        lua_pushstring(L, str);
        free(str as *mut std::ffi::c_void);
    } else if jsc_value_is_object(value) != 0 {
        let mut keys: *mut *mut std::ffi::c_char = jsc_value_object_enumerate_properties(
            value,
        );
        let mut top: std::ffi::c_int = lua_gettop(L);
        let mut val: *mut JSCValue = 0 as *mut JSCValue;
        let mut eptr: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
        let mut key: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
        let mut i: std::ffi::c_int = 0 as std::ffi::c_int;
        let mut n: std::ffi::c_long = 0;
        lua_createtable(L, 0 as std::ffi::c_int, 0 as std::ffi::c_int);
        while !keys.is_null()
            && {
                let fresh1 = i;
                i = i + 1;
                key = *keys.offset(fresh1 as isize);
                !key.is_null()
            }
        {
            if *key as std::ffi::c_int != 0
                && {
                    n = strtol(key, &mut eptr, 10 as std::ffi::c_int);
                    *eptr == 0
                }
            {
                n += 1;
                lua_pushinteger(L, n);
            } else {
                lua_pushstring(L, key);
            }
            val = jsc_value_object_get_property(value, key);
            if luajs_pushvalue(L, val) == 0 {
                g_object_unref(val as gpointer);
                lua_settop(L, top);
                g_strfreev(keys);
                return 0 as std::ffi::c_int;
            }
            g_object_unref(val as gpointer);
            lua_rawset(L, -(3 as std::ffi::c_int));
        }
        g_strfreev(keys);
    } else {
        return 0 as std::ffi::c_int
    }
    return 1 as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "141:1"]
pub unsafe extern "C" fn luajs_eval_js(
    mut L: *mut lua_State,
    mut ctx: *mut JSCContext,
    mut code: *const std::ffi::c_char,
    mut source: *const std::ffi::c_char,
    mut line: guint,
    mut no_return: bool,
) -> std::ffi::c_int {
    let mut result: *mut JSCValue = jsc_context_evaluate_with_source_uri(
        ctx,
        code,
        -(1 as std::ffi::c_int) as gssize,
        source,
        line,
    );
    let mut exception: *mut JSCException = jsc_context_get_exception(ctx);
    if !exception.is_null() {
        let mut e: *mut std::ffi::c_char = jsc_exception_to_string(exception);
        lua_pushnil(L);
        lua_pushstring(L, e);
        free(e as *mut std::ffi::c_void);
        return 2 as std::ffi::c_int;
    }
    if no_return {
        return 0 as std::ffi::c_int;
    }
    let mut ret: std::ffi::c_int = luajs_pushvalue(L, result);
    g_object_unref(result as gpointer);
    if ret == 0 {
        lua_pushnil(L);
        lua_pushstring(
            L,
            b"unable to push the result onto the Lua stack\0" as *const u8
                as *const std::ffi::c_char,
        );
        return 2 as std::ffi::c_int;
    }
    return ret;
}
