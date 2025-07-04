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
#[c2rust::header_src = "/usr/lib/clang/20/include/__stdarg_va_list.h:21"]
pub mod __stdarg_va_list_h {
    #[c2rust::src_loc = "12:1"]
    pub type va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
#[c2rust::header_src = "/usr/lib/clang/20/include/__stddef_ptrdiff_t.h:21"]
pub mod __stddef_ptrdiff_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type ptrdiff_t = std::ffi::c_long;
}
#[c2rust::header_src = "/usr/lib/clang/20/include/__stddef_size_t.h:21"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = std::ffi::c_ulong;
}
#[c2rust::header_src = "/usr/include/luajit-2.1/lua.h:21"]
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
        #[c2rust::src_loc = "122:1"]
        pub fn lua_settop(L: *mut lua_State, idx: std::ffi::c_int);
        #[c2rust::src_loc = "148:1"]
        pub fn lua_tointeger(L: *mut lua_State, idx: std::ffi::c_int) -> lua_Integer;
        #[c2rust::src_loc = "150:1"]
        pub fn lua_tolstring(
            L: *mut lua_State,
            idx: std::ffi::c_int,
            len: *mut size_t,
        ) -> *const std::ffi::c_char;
        #[c2rust::src_loc = "162:1"]
        pub fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
        #[c2rust::src_loc = "165:1"]
        pub fn lua_pushstring(L: *mut lua_State, s: *const std::ffi::c_char);
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/common.h:21"]
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
#[c2rust::header_src = "/usr/lib/glib-2.0/include/glibconfig.h:21"]
pub mod glibconfig_h {
    #[c2rust::src_loc = "46:1"]
    pub type guint8 = std::ffi::c_uchar;
    #[c2rust::src_loc = "57:1"]
    pub type guint32 = std::ffi::c_uint;
    #[c2rust::src_loc = "66:1"]
    pub type gint64 = std::ffi::c_long;
    #[c2rust::src_loc = "67:1"]
    pub type guint64 = std::ffi::c_ulong;
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
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = std::ffi::c_long;
    #[c2rust::src_loc = "162:1"]
    pub type __suseconds_t = std::ffi::c_long;
    #[c2rust::src_loc = "194:1"]
    pub type __ssize_t = std::ffi::c_long;
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
    #[c2rust::src_loc = "64:1"]
    pub type gdouble = std::ffi::c_double;
    #[c2rust::src_loc = "109:1"]
    pub type gpointer = *mut std::ffi::c_void;
    #[c2rust::src_loc = "110:1"]
    pub type gconstpointer = *const std::ffi::c_void;
    #[c2rust::src_loc = "117:1"]
    pub type GEqualFunc = Option::<
        unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean,
    >;
    #[c2rust::src_loc = "140:1"]
    pub type GDestroyNotify = Option::<unsafe extern "C" fn(gpointer) -> ()>;
    #[c2rust::src_loc = "143:1"]
    pub type GHashFunc = Option::<unsafe extern "C" fn(gconstpointer) -> guint>;
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
        #[c2rust::src_loc = "152:1"]
        pub fn g_ptr_array_new_with_free_func(
            element_free_func: GDestroyNotify,
        ) -> *mut GPtrArray;
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
    extern "C" {
        #[c2rust::src_loc = "207:1"]
        pub fn g_error_free(error: *mut GError);
    }
}
#[c2rust::header_src = "/usr/include/sys/types.h:21"]
pub mod sys_types_h {
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::__ssize_t;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gasyncqueue.h:21"]
pub mod gasyncqueue_h {
    #[c2rust::src_loc = "38:1"]
    pub type GAsyncQueue = _GAsyncQueue;
    use super::gtypes_h::gpointer;
    extern "C" {
        #[c2rust::src_loc = "38:16"]
        pub type _GAsyncQueue;
        #[c2rust::src_loc = "40:1"]
        pub fn g_async_queue_new() -> *mut GAsyncQueue;
        #[c2rust::src_loc = "44:1"]
        pub fn g_async_queue_lock(queue: *mut GAsyncQueue);
        #[c2rust::src_loc = "46:1"]
        pub fn g_async_queue_unlock(queue: *mut GAsyncQueue);
        #[c2rust::src_loc = "59:1"]
        pub fn g_async_queue_push(queue: *mut GAsyncQueue, data: gpointer);
        #[c2rust::src_loc = "62:1"]
        pub fn g_async_queue_push_unlocked(queue: *mut GAsyncQueue, data: gpointer);
        #[c2rust::src_loc = "79:1"]
        pub fn g_async_queue_try_pop(queue: *mut GAsyncQueue) -> gpointer;
        #[c2rust::src_loc = "81:1"]
        pub fn g_async_queue_try_pop_unlocked(queue: *mut GAsyncQueue) -> gpointer;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gconvert.h:21"]
pub mod gconvert_h {
    #[c2rust::src_loc = "85:1"]
    pub type GIConv = *mut _GIConv;
    extern "C" {
        #[c2rust::src_loc = "85:16"]
        pub type _GIConv;
    }
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
#[c2rust::header_src = "/usr/include/glib-2.0/glib/glist.h:21"]
pub mod glist_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "41:8"]
    pub struct _GList {
        pub data: gpointer,
        pub next: *mut GList,
        pub prev: *mut GList,
    }
    #[c2rust::src_loc = "39:1"]
    pub type GList = _GList;
    use super::gtypes_h::gpointer;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/ghash.h:21"]
pub mod ghash_h {
    #[c2rust::src_loc = "40:1"]
    pub type GHashTable = _GHashTable;
    use super::gtypes_h::{
        GHashFunc, GEqualFunc, GDestroyNotify, gpointer, gboolean, gconstpointer, guint,
    };
    extern "C" {
        #[c2rust::src_loc = "40:16"]
        pub type _GHashTable;
        #[c2rust::src_loc = "62:1"]
        pub fn g_hash_table_new_full(
            hash_func: GHashFunc,
            key_equal_func: GEqualFunc,
            key_destroy_func: GDestroyNotify,
            value_destroy_func: GDestroyNotify,
        ) -> *mut GHashTable;
        #[c2rust::src_loc = "71:1"]
        pub fn g_hash_table_insert(
            hash_table: *mut GHashTable,
            key: gpointer,
            value: gpointer,
        ) -> gboolean;
        #[c2rust::src_loc = "101:1"]
        pub fn g_hash_table_lookup(
            hash_table: *mut GHashTable,
            key: gconstpointer,
        ) -> gpointer;
        #[c2rust::src_loc = "170:1"]
        pub fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
        #[c2rust::src_loc = "177:1"]
        pub fn g_str_hash(v: gconstpointer) -> guint;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gslist.h:21"]
pub mod gslist_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "41:8"]
    pub struct _GSList {
        pub data: gpointer,
        pub next: *mut GSList,
    }
    #[c2rust::src_loc = "39:1"]
    pub type GSList = _GSList;
    use super::gtypes_h::gpointer;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gmain.h:21"]
pub mod gmain_h {
    #[c2rust::src_loc = "33:9"]
    pub type GIOCondition = std::ffi::c_uint;
    #[c2rust::src_loc = "40:3"]
    pub const G_IO_NVAL: GIOCondition = 32;
    #[c2rust::src_loc = "39:3"]
    pub const G_IO_HUP: GIOCondition = 16;
    #[c2rust::src_loc = "38:3"]
    pub const G_IO_ERR: GIOCondition = 8;
    #[c2rust::src_loc = "37:3"]
    pub const G_IO_PRI: GIOCondition = 2;
    #[c2rust::src_loc = "36:3"]
    pub const G_IO_OUT: GIOCondition = 4;
    #[c2rust::src_loc = "35:3"]
    pub const G_IO_IN: GIOCondition = 1;
    #[c2rust::src_loc = "70:1"]
    pub type GMainContext = _GMainContext;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "266:8"]
    pub struct _GSource {
        pub callback_data: gpointer,
        pub callback_funcs: *mut GSourceCallbackFuncs,
        pub source_funcs: *const GSourceFuncs,
        pub ref_count: guint,
        pub context: *mut GMainContext,
        pub priority: gint,
        pub flags: guint,
        pub source_id: guint,
        pub poll_fds: *mut GSList,
        pub prev: *mut GSource,
        pub next: *mut GSource,
        pub name: *mut std::ffi::c_char,
        pub priv_0: *mut GSourcePrivate,
    }
    #[c2rust::src_loc = "87:1"]
    pub type GSourcePrivate = _GSourcePrivate;
    #[c2rust::src_loc = "86:1"]
    pub type GSource = _GSource;
    #[c2rust::src_loc = "157:1"]
    pub type GSourceFuncs = _GSourceFuncs;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "398:8"]
    pub struct _GSourceFuncs {
        pub prepare: GSourceFuncsPrepareFunc,
        pub check: GSourceFuncsCheckFunc,
        pub dispatch: GSourceFuncsDispatchFunc,
        pub finalize: GSourceFuncsFinalizeFunc,
        pub closure_callback: GSourceFunc,
        pub closure_marshal: GSourceDummyMarshal,
    }
    #[c2rust::src_loc = "307:1"]
    pub type GSourceDummyMarshal = Option::<unsafe extern "C" fn() -> ()>;
    #[c2rust::src_loc = "199:1"]
    pub type GSourceFunc = Option::<unsafe extern "C" fn(gpointer) -> gboolean>;
    #[c2rust::src_loc = "396:1"]
    pub type GSourceFuncsFinalizeFunc = Option::<
        unsafe extern "C" fn(*mut GSource) -> (),
    >;
    #[c2rust::src_loc = "379:1"]
    pub type GSourceFuncsDispatchFunc = Option::<
        unsafe extern "C" fn(*mut GSource, GSourceFunc, gpointer) -> gboolean,
    >;
    #[c2rust::src_loc = "354:1"]
    pub type GSourceFuncsCheckFunc = Option::<
        unsafe extern "C" fn(*mut GSource) -> gboolean,
    >;
    #[c2rust::src_loc = "333:1"]
    pub type GSourceFuncsPrepareFunc = Option::<
        unsafe extern "C" fn(*mut GSource, *mut gint) -> gboolean,
    >;
    #[c2rust::src_loc = "99:1"]
    pub type GSourceCallbackFuncs = _GSourceCallbackFuncs;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "291:8"]
    pub struct _GSourceCallbackFuncs {
        pub ref_0: Option::<unsafe extern "C" fn(gpointer) -> ()>,
        pub unref: Option::<unsafe extern "C" fn(gpointer) -> ()>,
        pub get: Option::<
            unsafe extern "C" fn(
                gpointer,
                *mut GSource,
                *mut GSourceFunc,
                *mut gpointer,
            ) -> (),
        >,
    }
    use super::gtypes_h::{gpointer, guint, gint, gboolean};
    use super::gslist_h::GSList;
    extern "C" {
        #[c2rust::src_loc = "70:16"]
        pub type _GMainContext;
        #[c2rust::src_loc = "87:16"]
        pub type _GSourcePrivate;
        #[c2rust::src_loc = "962:1"]
        pub fn g_idle_add(function: GSourceFunc, data: gpointer) -> guint;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gstring.h:21"]
pub mod gstring_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "45:8"]
    pub struct _GString {
        pub str_0: *mut gchar,
        pub len: gsize,
        pub allocated_len: gsize,
    }
    #[c2rust::src_loc = "43:1"]
    pub type GString = _GString;
    #[inline(always)]
    #[c2rust::src_loc = "197:1"]
    pub unsafe extern "C" fn g_string_append_c_inline(
        mut gstring: *mut GString,
        mut c: gchar,
    ) -> *mut GString {
        if !gstring.is_null()
            && ((*gstring).len).wrapping_add(1 as std::ffi::c_int as gsize)
                < (*gstring).allocated_len
        {
            let fresh0 = (*gstring).len;
            (*gstring).len = ((*gstring).len).wrapping_add(1);
            *((*gstring).str_0).offset(fresh0 as isize) = c;
            *((*gstring).str_0)
                .offset((*gstring).len as isize) = 0 as std::ffi::c_int as gchar;
        } else {
            g_string_insert_c(gstring, -(1 as std::ffi::c_int) as gssize, c);
        }
        return gstring;
    }
    #[inline(always)]
    #[c2rust::src_loc = "216:1"]
    pub unsafe extern "C" fn g_string_append_len_inline(
        mut gstring: *mut GString,
        mut val: *const std::ffi::c_char,
        mut len: gssize,
    ) -> *mut GString {
        let mut len_unsigned: gsize = 0;
        if gstring.is_null() {
            return g_string_append_len(gstring, val, len);
        }
        if val.is_null() {
            return if len != 0 as std::ffi::c_int as gssize {
                g_string_append_len(gstring, val, len)
            } else {
                gstring
            };
        }
        if len < 0 as std::ffi::c_int as gssize {
            len_unsigned = strlen(val);
        } else {
            len_unsigned = len as gsize;
        }
        if ((*gstring).len).wrapping_add(len_unsigned) < (*gstring).allocated_len {
            let mut end: *mut std::ffi::c_char = ((*gstring).str_0)
                .offset((*gstring).len as isize);
            if val.offset(len_unsigned as isize) <= end as *const std::ffi::c_char
                || val > end.offset(len_unsigned as isize) as *const std::ffi::c_char
            {
                memcpy(
                    end as *mut std::ffi::c_void,
                    val as *const std::ffi::c_void,
                    len_unsigned,
                );
            } else {
                memmove(
                    end as *mut std::ffi::c_void,
                    val as *const std::ffi::c_void,
                    len_unsigned,
                );
            }
            (*gstring).len = ((*gstring).len).wrapping_add(len_unsigned);
            *((*gstring).str_0)
                .offset((*gstring).len as isize) = 0 as std::ffi::c_int as gchar;
            return gstring;
        } else {
            return g_string_insert_len(
                gstring,
                -(1 as std::ffi::c_int) as gssize,
                val,
                len,
            )
        };
    }
    use super::gtypes_h::{gchar, gboolean};
    use super::glibconfig_h::{gsize, gssize};
    use super::string_h::{strlen, memcpy, memmove};
    extern "C" {
        #[c2rust::src_loc = "52:1"]
        pub fn g_string_new(init: *const gchar) -> *mut GString;
        #[c2rust::src_loc = "61:1"]
        pub fn g_string_free(string: *mut GString, free_segment: gboolean) -> *mut gchar;
        #[c2rust::src_loc = "64:1"]
        pub fn g_string_free_and_steal(string: *mut GString) -> *mut gchar;
        #[c2rust::src_loc = "99:1"]
        pub fn g_string_insert_len(
            string: *mut GString,
            pos: gssize,
            val: *const gchar,
            len: gssize,
        ) -> *mut GString;
        #[c2rust::src_loc = "107:1"]
        pub fn g_string_append_len(
            string: *mut GString,
            val: *const gchar,
            len: gssize,
        ) -> *mut GString;
        #[c2rust::src_loc = "134:1"]
        pub fn g_string_insert_c(
            string: *mut GString,
            pos: gssize,
            c: gchar,
        ) -> *mut GString;
        #[c2rust::src_loc = "178:1"]
        pub fn g_string_append_printf(
            string: *mut GString,
            format: *const gchar,
            _: ...
        );
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/giochannel.h:21"]
pub mod giochannel_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "100:8"]
    pub struct _GIOChannel {
        pub ref_count: gint,
        pub funcs: *mut GIOFuncs,
        pub encoding: *mut gchar,
        pub read_cd: GIConv,
        pub write_cd: GIConv,
        pub line_term: *mut gchar,
        pub line_term_len: guint,
        pub buf_size: gsize,
        pub read_buf: *mut GString,
        pub encoded_read_buf: *mut GString,
        pub write_buf: *mut GString,
        pub partial_write_buf: [gchar; 6],
        #[bitfield(name = "use_buffer", ty = "guint", bits = "0..=0")]
        #[bitfield(name = "do_encode", ty = "guint", bits = "1..=1")]
        #[bitfield(name = "close_on_unref", ty = "guint", bits = "2..=2")]
        #[bitfield(name = "is_readable", ty = "guint", bits = "3..=3")]
        #[bitfield(name = "is_writeable", ty = "guint", bits = "4..=4")]
        #[bitfield(name = "is_seekable", ty = "guint", bits = "5..=5")]
        pub use_buffer_do_encode_close_on_unref_is_readable_is_writeable_is_seekable: [u8; 1],
        #[bitfield(padding)]
        pub c2rust_padding: [u8; 1],
        pub reserved1: gpointer,
        pub reserved2: gpointer,
    }
    #[c2rust::src_loc = "44:1"]
    pub type GIOFuncs = _GIOFuncs;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "134:8"]
    pub struct _GIOFuncs {
        pub io_read: Option::<
            unsafe extern "C" fn(
                *mut GIOChannel,
                *mut gchar,
                gsize,
                *mut gsize,
                *mut *mut GError,
            ) -> GIOStatus,
        >,
        pub io_write: Option::<
            unsafe extern "C" fn(
                *mut GIOChannel,
                *const gchar,
                gsize,
                *mut gsize,
                *mut *mut GError,
            ) -> GIOStatus,
        >,
        pub io_seek: Option::<
            unsafe extern "C" fn(
                *mut GIOChannel,
                gint64,
                GSeekType,
                *mut *mut GError,
            ) -> GIOStatus,
        >,
        pub io_close: Option::<
            unsafe extern "C" fn(*mut GIOChannel, *mut *mut GError) -> GIOStatus,
        >,
        pub io_create_watch: Option::<
            unsafe extern "C" fn(*mut GIOChannel, GIOCondition) -> *mut GSource,
        >,
        pub io_free: Option::<unsafe extern "C" fn(*mut GIOChannel) -> ()>,
        pub io_set_flags: Option::<
            unsafe extern "C" fn(
                *mut GIOChannel,
                GIOFlags,
                *mut *mut GError,
            ) -> GIOStatus,
        >,
        pub io_get_flags: Option::<unsafe extern "C" fn(*mut GIOChannel) -> GIOFlags>,
    }
    #[c2rust::src_loc = "43:1"]
    pub type GIOChannel = _GIOChannel;
    #[c2rust::src_loc = "86:9"]
    pub type GIOFlags = std::ffi::c_uint;
    #[c2rust::src_loc = "97:3"]
    pub const G_IO_FLAG_SET_MASK: GIOFlags = 3;
    #[c2rust::src_loc = "96:3"]
    pub const G_IO_FLAG_GET_MASK: GIOFlags = 31;
    #[c2rust::src_loc = "95:3"]
    pub const G_IO_FLAG_MASK: GIOFlags = 31;
    #[c2rust::src_loc = "94:3"]
    pub const G_IO_FLAG_IS_SEEKABLE: GIOFlags = 16;
    #[c2rust::src_loc = "93:3"]
    pub const G_IO_FLAG_IS_WRITEABLE: GIOFlags = 8;
    #[c2rust::src_loc = "92:3"]
    pub const G_IO_FLAG_IS_WRITABLE: GIOFlags = 8;
    #[c2rust::src_loc = "91:3"]
    pub const G_IO_FLAG_IS_READABLE: GIOFlags = 4;
    #[c2rust::src_loc = "90:3"]
    pub const G_IO_FLAG_NONBLOCK: GIOFlags = 2;
    #[c2rust::src_loc = "89:3"]
    pub const G_IO_FLAG_APPEND: GIOFlags = 1;
    #[c2rust::src_loc = "88:3"]
    pub const G_IO_FLAG_NONE: GIOFlags = 0;
    #[c2rust::src_loc = "71:9"]
    pub type GIOStatus = std::ffi::c_uint;
    #[c2rust::src_loc = "76:3"]
    pub const G_IO_STATUS_AGAIN: GIOStatus = 3;
    #[c2rust::src_loc = "75:3"]
    pub const G_IO_STATUS_EOF: GIOStatus = 2;
    #[c2rust::src_loc = "74:3"]
    pub const G_IO_STATUS_NORMAL: GIOStatus = 1;
    #[c2rust::src_loc = "73:3"]
    pub const G_IO_STATUS_ERROR: GIOStatus = 0;
    #[c2rust::src_loc = "79:9"]
    pub type GSeekType = std::ffi::c_uint;
    #[c2rust::src_loc = "83:3"]
    pub const G_SEEK_END: GSeekType = 2;
    #[c2rust::src_loc = "82:3"]
    pub const G_SEEK_SET: GSeekType = 1;
    #[c2rust::src_loc = "81:3"]
    pub const G_SEEK_CUR: GSeekType = 0;
    use super::gtypes_h::{gint, gchar, guint, gpointer};
    use super::gconvert_h::GIConv;
    use super::glibconfig_h::{gsize, gint64};
    use super::gstring_h::GString;
    use super::gerror_h::GError;
    use super::gmain_h::{GSource, GIOCondition};
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gqueue.h:21"]
pub mod gqueue_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:8"]
    pub struct _GQueue {
        pub head: *mut GList,
        pub tail: *mut GList,
        pub length: guint,
    }
    #[c2rust::src_loc = "38:1"]
    pub type GQueue = _GQueue;
    use super::glist_h::GList;
    use super::gtypes_h::guint;
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gregex.h:21"]
pub mod gregex_h {
    #[c2rust::src_loc = "304:9"]
    pub type GRegexCompileFlags = std::ffi::c_uint;
    #[c2rust::src_loc = "324:3"]
    pub const G_REGEX_JAVASCRIPT_COMPAT: GRegexCompileFlags = 33554432;
    #[c2rust::src_loc = "323:3"]
    pub const G_REGEX_BSR_ANYCRLF: GRegexCompileFlags = 8388608;
    #[c2rust::src_loc = "322:3"]
    pub const G_REGEX_NEWLINE_ANYCRLF: GRegexCompileFlags = 5242880;
    #[c2rust::src_loc = "321:3"]
    pub const G_REGEX_NEWLINE_CRLF: GRegexCompileFlags = 3145728;
    #[c2rust::src_loc = "320:3"]
    pub const G_REGEX_NEWLINE_LF: GRegexCompileFlags = 2097152;
    #[c2rust::src_loc = "319:3"]
    pub const G_REGEX_NEWLINE_CR: GRegexCompileFlags = 1048576;
    #[c2rust::src_loc = "318:3"]
    pub const G_REGEX_DUPNAMES: GRegexCompileFlags = 524288;
    #[c2rust::src_loc = "317:3"]
    pub const G_REGEX_FIRSTLINE: GRegexCompileFlags = 262144;
    #[c2rust::src_loc = "316:3"]
    pub const G_REGEX_OPTIMIZE: GRegexCompileFlags = 8192;
    #[c2rust::src_loc = "315:3"]
    pub const G_REGEX_NO_AUTO_CAPTURE: GRegexCompileFlags = 4096;
    #[c2rust::src_loc = "314:3"]
    pub const G_REGEX_RAW: GRegexCompileFlags = 2048;
    #[c2rust::src_loc = "313:3"]
    pub const G_REGEX_UNGREEDY: GRegexCompileFlags = 512;
    #[c2rust::src_loc = "312:3"]
    pub const G_REGEX_DOLLAR_ENDONLY: GRegexCompileFlags = 32;
    #[c2rust::src_loc = "311:3"]
    pub const G_REGEX_ANCHORED: GRegexCompileFlags = 16;
    #[c2rust::src_loc = "310:3"]
    pub const G_REGEX_EXTENDED: GRegexCompileFlags = 8;
    #[c2rust::src_loc = "309:3"]
    pub const G_REGEX_DOTALL: GRegexCompileFlags = 4;
    #[c2rust::src_loc = "308:3"]
    pub const G_REGEX_MULTILINE: GRegexCompileFlags = 2;
    #[c2rust::src_loc = "307:3"]
    pub const G_REGEX_CASELESS: GRegexCompileFlags = 1;
    #[c2rust::src_loc = "306:3"]
    pub const G_REGEX_DEFAULT: GRegexCompileFlags = 0;
    #[c2rust::src_loc = "396:9"]
    pub type GRegexMatchFlags = std::ffi::c_uint;
    #[c2rust::src_loc = "413:3"]
    pub const G_REGEX_MATCH_NOTEMPTY_ATSTART: GRegexMatchFlags = 268435456;
    #[c2rust::src_loc = "412:3"]
    pub const G_REGEX_MATCH_PARTIAL_HARD: GRegexMatchFlags = 134217728;
    #[c2rust::src_loc = "411:3"]
    pub const G_REGEX_MATCH_PARTIAL_SOFT: GRegexMatchFlags = 32768;
    #[c2rust::src_loc = "410:3"]
    pub const G_REGEX_MATCH_BSR_ANY: GRegexMatchFlags = 16777216;
    #[c2rust::src_loc = "409:3"]
    pub const G_REGEX_MATCH_BSR_ANYCRLF: GRegexMatchFlags = 8388608;
    #[c2rust::src_loc = "408:3"]
    pub const G_REGEX_MATCH_NEWLINE_ANYCRLF: GRegexMatchFlags = 5242880;
    #[c2rust::src_loc = "407:3"]
    pub const G_REGEX_MATCH_NEWLINE_ANY: GRegexMatchFlags = 4194304;
    #[c2rust::src_loc = "406:3"]
    pub const G_REGEX_MATCH_NEWLINE_CRLF: GRegexMatchFlags = 3145728;
    #[c2rust::src_loc = "405:3"]
    pub const G_REGEX_MATCH_NEWLINE_LF: GRegexMatchFlags = 2097152;
    #[c2rust::src_loc = "404:3"]
    pub const G_REGEX_MATCH_NEWLINE_CR: GRegexMatchFlags = 1048576;
    #[c2rust::src_loc = "403:3"]
    pub const G_REGEX_MATCH_PARTIAL: GRegexMatchFlags = 32768;
    #[c2rust::src_loc = "402:3"]
    pub const G_REGEX_MATCH_NOTEMPTY: GRegexMatchFlags = 1024;
    #[c2rust::src_loc = "401:3"]
    pub const G_REGEX_MATCH_NOTEOL: GRegexMatchFlags = 256;
    #[c2rust::src_loc = "400:3"]
    pub const G_REGEX_MATCH_NOTBOL: GRegexMatchFlags = 128;
    #[c2rust::src_loc = "399:3"]
    pub const G_REGEX_MATCH_ANCHORED: GRegexMatchFlags = 16;
    #[c2rust::src_loc = "398:3"]
    pub const G_REGEX_MATCH_DEFAULT: GRegexMatchFlags = 0;
    #[c2rust::src_loc = "416:1"]
    pub type GRegex = _GRegex;
    use super::gtypes_h::{gchar, gint};
    use super::gerror_h::GError;
    use super::glibconfig_h::gssize;
    extern "C" {
        #[c2rust::src_loc = "416:16"]
        pub type _GRegex;
        #[c2rust::src_loc = "449:1"]
        pub fn g_regex_new(
            pattern: *const gchar,
            compile_options: GRegexCompileFlags,
            match_options: GRegexMatchFlags,
            error: *mut *mut GError,
        ) -> *mut GRegex;
        #[c2rust::src_loc = "544:1"]
        pub fn g_regex_replace_literal(
            regex: *const GRegex,
            string: *const gchar,
            string_len: gssize,
            start_position: gint,
            replacement: *const gchar,
            match_options: GRegexMatchFlags,
            error: *mut *mut GError,
        ) -> *mut gchar;
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
#[c2rust::header_src = "/usr/include/glib-2.0/gobject/gtype.h:21"]
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
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:21"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:21"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
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
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timeval.h:24"]
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
#[c2rust::header_src = "/home/daana/git/luakit/common/luaclass.h:24"]
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
#[c2rust::header_src = "/home/daana/git/luakit/common/signal.h:24"]
pub mod signal_h {
    #[c2rust::src_loc = "29:1"]
    pub type signal_t = GTree;
    use super::gtree_h::GTree;
}
#[c2rust::header_src = "/home/daana/git/luakit/common/ipc.h:25"]
pub mod ipc_h {
    #[c2rust::src_loc = "41:9"]
    pub type ipc_type_t = std::ffi::c_uint;
    #[c2rust::src_loc = "41:16"]
    pub const IPC_TYPE_crash: ipc_type_t = 128;
    #[c2rust::src_loc = "41:16"]
    pub const IPC_TYPE_page_created: ipc_type_t = 64;
    #[c2rust::src_loc = "41:16"]
    pub const IPC_TYPE_log: ipc_type_t = 32;
    #[c2rust::src_loc = "41:16"]
    pub const IPC_TYPE_eval_js: ipc_type_t = 16;
    #[c2rust::src_loc = "41:16"]
    pub const IPC_TYPE_extension_init: ipc_type_t = 8;
    #[c2rust::src_loc = "41:16"]
    pub const IPC_TYPE_scroll: ipc_type_t = 4;
    #[c2rust::src_loc = "41:16"]
    pub const IPC_TYPE_lua_ipc: ipc_type_t = 2;
    #[c2rust::src_loc = "41:16"]
    pub const IPC_TYPE_lua_require_module: ipc_type_t = 1;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "47:16"]
    pub struct _ipc_header_t {
        pub length: guint,
        pub type_0: ipc_type_t,
    }
    #[c2rust::src_loc = "47:1"]
    pub type ipc_header_t = _ipc_header_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "94:16"]
    pub struct _ipc_recv_state_t {
        pub watch_in_id: guint,
        pub watch_hup_id: guint,
        pub queued_ipcs: *mut GPtrArray,
        pub hdr: ipc_header_t,
        pub payload: gpointer,
        pub bytes_read: gsize,
        pub hdr_done: gboolean,
    }
    #[c2rust::src_loc = "94:1"]
    pub type ipc_recv_state_t = _ipc_recv_state_t;
    #[c2rust::src_loc = "104:9"]
    pub type ipc_endpoint_status_t = std::ffi::c_uint;
    #[c2rust::src_loc = "107:5"]
    pub const IPC_ENDPOINT_FREED: ipc_endpoint_status_t = 2;
    #[c2rust::src_loc = "106:5"]
    pub const IPC_ENDPOINT_CONNECTED: ipc_endpoint_status_t = 1;
    #[c2rust::src_loc = "105:5"]
    pub const IPC_ENDPOINT_DISCONNECTED: ipc_endpoint_status_t = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "110:16"]
    pub struct _ipc_endpoint_t {
        pub name: *mut gchar,
        pub status: ipc_endpoint_status_t,
        pub channel: *mut GIOChannel,
        pub queue: *mut GQueue,
        pub recv_state: ipc_recv_state_t,
        pub refcount: gint,
        pub creation_notified: gboolean,
    }
    #[c2rust::src_loc = "110:1"]
    pub type ipc_endpoint_t = _ipc_endpoint_t;
    use super::gtypes_h::{guint, gpointer, gboolean, gchar, gint};
    use super::garray_h::GPtrArray;
    use super::glibconfig_h::gsize;
    use super::giochannel_h::GIOChannel;
    use super::gqueue_h::GQueue;
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
        #[c2rust::src_loc = "47:14"]
        pub fn memmove(
            _: *mut std::ffi::c_void,
            _: *const std::ffi::c_void,
            _: std::ffi::c_ulong,
        ) -> *mut std::ffi::c_void;
        #[c2rust::src_loc = "64:12"]
        pub fn memcmp(
            _: *const std::ffi::c_void,
            _: *const std::ffi::c_void,
            _: std::ffi::c_ulong,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "156:12"]
        pub fn strcmp(
            _: *const std::ffi::c_char,
            _: *const std::ffi::c_char,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "159:12"]
        pub fn strncmp(
            _: *const std::ffi::c_char,
            _: *const std::ffi::c_char,
            _: std::ffi::c_ulong,
        ) -> std::ffi::c_int;
        #[c2rust::src_loc = "273:14"]
        pub fn strrchr(
            _: *const std::ffi::c_char,
            _: std::ffi::c_int,
        ) -> *mut std::ffi::c_char;
        #[c2rust::src_loc = "407:15"]
        pub fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:21"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "756:13"]
        pub fn exit(_: std::ffi::c_int) -> !;
        #[c2rust::src_loc = "773:1"]
        pub fn getenv(__name: *const std::ffi::c_char) -> *mut std::ffi::c_char;
        #[c2rust::src_loc = "796:1"]
        pub fn unsetenv(__name: *const std::ffi::c_char) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gfileutils.h:21"]
pub mod gfileutils_h {
    use super::gtypes_h::{gchar, gboolean, gint};
    use super::glibconfig_h::gsize;
    use super::gerror_h::GError;
    extern "C" {
        #[c2rust::src_loc = "119:1"]
        pub fn g_file_get_contents(
            filename: *const gchar,
            contents: *mut *mut gchar,
            length: *mut gsize,
            error: *mut *mut GError,
        ) -> gboolean;
        #[c2rust::src_loc = "158:1"]
        pub fn g_file_open_tmp(
            tmpl: *const gchar,
            name_used: *mut *mut gchar,
            error: *mut *mut GError,
        ) -> gint;
        #[c2rust::src_loc = "166:1"]
        pub fn g_build_path(
            separator: *const gchar,
            first_element: *const gchar,
            _: ...
        ) -> *mut gchar;
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
    use super::gtypes_h::{gchar, gboolean, gdouble};
    use super::internal::__va_list_tag;
    use super::string_h::{strlen, memcpy};
    use super::__stddef_size_t_h::size_t;
    use super::gmem_h::g_malloc;
    extern "C" {
        #[c2rust::src_loc = "145:1"]
        pub fn g_str_has_prefix(str: *const gchar, prefix: *const gchar) -> gboolean;
        #[c2rust::src_loc = "216:1"]
        pub fn g_ascii_strtod(nptr: *const gchar, endptr: *mut *mut gchar) -> gdouble;
        #[c2rust::src_loc = "283:1"]
        pub fn g_strdup(str: *const gchar) -> *mut gchar;
        #[c2rust::src_loc = "285:1"]
        pub fn g_strdup_printf(format: *const gchar, _: ...) -> *mut gchar;
        #[c2rust::src_loc = "288:1"]
        pub fn g_strdup_vprintf(
            format: *const gchar,
            args: ::core::ffi::VaList,
        ) -> *mut gchar;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gslice.h:21"]
pub mod gslice_h {
    use super::glibconfig_h::gsize;
    use super::gtypes_h::gpointer;
    extern "C" {
        #[c2rust::src_loc = "36:1"]
        pub fn g_slice_alloc0(block_size: gsize) -> gpointer;
        #[c2rust::src_loc = "41:1"]
        pub fn g_slice_free1(block_size: gsize, mem_block: gpointer);
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gtestutils.h:21"]
pub mod gtestutils_h {
    use super::glibconfig_h::guint64;
    use super::gerror_h::GError;
    use super::gquark_h::GQuark;
    extern "C" {
        #[c2rust::src_loc = "624:1"]
        pub fn g_assertion_message_expr(
            domain: *const std::ffi::c_char,
            file: *const std::ffi::c_char,
            line: std::ffi::c_int,
            func: *const std::ffi::c_char,
            expr: *const std::ffi::c_char,
        ) -> !;
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
        #[c2rust::src_loc = "670:1"]
        pub fn g_assertion_message_error(
            domain: *const std::ffi::c_char,
            file: *const std::ffi::c_char,
            line: std::ffi::c_int,
            func: *const std::ffi::c_char,
            expr: *const std::ffi::c_char,
            error: *const GError,
            error_domain: GQuark,
            error_code: std::ffi::c_int,
        );
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:21"]
pub mod unistd_h {
    use super::__stddef_size_t_h::size_t;
    use super::sys_types_h::ssize_t;
    extern "C" {
        #[c2rust::src_loc = "358:1"]
        pub fn close(__fd: std::ffi::c_int) -> std::ffi::c_int;
        #[c2rust::src_loc = "378:1"]
        pub fn write(
            __fd: std::ffi::c_int,
            __buf: *const std::ffi::c_void,
            __n: size_t,
        ) -> ssize_t;
        #[c2rust::src_loc = "809:1"]
        pub fn isatty(__fd: std::ffi::c_int) -> std::ffi::c_int;
        #[c2rust::src_loc = "858:1"]
        pub fn unlink(__name: *const std::ffi::c_char) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:21"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[c2rust::src_loc = "151:14"]
        pub static mut stderr: *mut FILE;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/common/luaserialize.h:23"]
pub mod luaserialize_h {
    use super::lua_h::lua_State;
    use super::glibconfig_h::guint8;
    use super::gtypes_h::guint;
    extern "C" {
        #[c2rust::src_loc = "26:1"]
        pub fn lua_deserialize_range(
            L: *mut lua_State,
            in_0: *const guint8,
            length: guint,
        ) -> std::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/sys/time.h:24"]
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
#[c2rust::header_src = "/home/daana/git/luakit/common/util.h:24"]
pub mod util_h {
    #[inline]
    #[c2rust::src_loc = "63:1"]
    pub unsafe extern "C" fn l_time() -> gdouble {
        let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
        gettimeofday(&mut tv, 0 as *mut std::ffi::c_void);
        return tv.tv_sec as std::ffi::c_double
            + tv.tv_usec as std::ffi::c_double / 1e6f64;
    }
    use super::gtypes_h::{gdouble, gchar, gboolean};
    use super::struct_timeval_h::timeval;
    use super::types_h::{__time_t, __suseconds_t};
    use super::time_h::gettimeofday;
    extern "C" {
        #[c2rust::src_loc = "71:1"]
        pub fn file_exists(_: *const gchar) -> gboolean;
        #[c2rust::src_loc = "75:1"]
        pub fn strip_ansi_escapes(in_0: *const gchar) -> *mut gchar;
    }
}
#[c2rust::header_src = "/home/daana/git/luakit/clib/msg.h:26"]
pub mod msg_h {
    use super::luaclass_h::lua_class_t;
    extern "C" {
        #[c2rust::src_loc = "28:1"]
        pub fn msg_lib_get_msg_class() -> *mut lua_class_t;
    }
}
#[c2rust::header_src = "/usr/include/glib-2.0/glib/gprintf.h:28"]
pub mod gprintf_h {
    use super::FILE_h::FILE;
    use super::gtypes_h::{gchar, gint};
    extern "C" {
        #[c2rust::src_loc = "32:1"]
        pub fn g_fprintf(file: *mut FILE, format: *const gchar, _: ...) -> gint;
    }
}
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::__stdarg_va_list_h::va_list;
pub use self::__stddef_ptrdiff_t_h::ptrdiff_t;
pub use self::__stddef_size_t_h::size_t;
pub use self::lua_h::{
    lua_Number, lua_Integer, lua_State, lua_settop, lua_tointeger, lua_tolstring,
    lua_pushnumber, lua_pushstring,
};
pub use self::common_h::{_common_t, common_t, common};
pub use self::glibconfig_h::{guint8, guint32, gint64, guint64, gssize, gsize};
pub use self::types_h::{__off_t, __off64_t, __time_t, __suseconds_t, __ssize_t};
pub use self::gtypes_h::{
    gchar, glong, gint, gboolean, gulong, guint, gdouble, gpointer, gconstpointer,
    GEqualFunc, GDestroyNotify, GHashFunc,
};
pub use self::garray_h::{
    _GPtrArray, GPtrArray, g_ptr_array_new_with_free_func, g_ptr_array_add,
};
pub use self::gquark_h::GQuark;
pub use self::gerror_h::{_GError, GError, g_error_free};
pub use self::sys_types_h::ssize_t;
pub use self::gasyncqueue_h::{
    GAsyncQueue, _GAsyncQueue, g_async_queue_new, g_async_queue_lock,
    g_async_queue_unlock, g_async_queue_push, g_async_queue_push_unlocked,
    g_async_queue_try_pop, g_async_queue_try_pop_unlocked,
};
pub use self::gconvert_h::{GIConv, _GIConv};
pub use self::gdataset_h::{GData, _GData};
pub use self::glist_h::{_GList, GList};
pub use self::ghash_h::{
    GHashTable, _GHashTable, g_hash_table_new_full, g_hash_table_insert,
    g_hash_table_lookup, g_str_equal, g_str_hash,
};
pub use self::gslist_h::{_GSList, GSList};
pub use self::gmain_h::{
    GIOCondition, G_IO_NVAL, G_IO_HUP, G_IO_ERR, G_IO_PRI, G_IO_OUT, G_IO_IN,
    GMainContext, _GSource, GSourcePrivate, GSource, GSourceFuncs, _GSourceFuncs,
    GSourceDummyMarshal, GSourceFunc, GSourceFuncsFinalizeFunc, GSourceFuncsDispatchFunc,
    GSourceFuncsCheckFunc, GSourceFuncsPrepareFunc, GSourceCallbackFuncs,
    _GSourceCallbackFuncs, _GMainContext, _GSourcePrivate, g_idle_add,
};
pub use self::gstring_h::{
    _GString, GString, g_string_append_c_inline, g_string_append_len_inline,
    g_string_new, g_string_free, g_string_free_and_steal, g_string_insert_len,
    g_string_append_len, g_string_insert_c, g_string_append_printf,
};
pub use self::giochannel_h::{
    _GIOChannel, GIOFuncs, _GIOFuncs, GIOChannel, GIOFlags, G_IO_FLAG_SET_MASK,
    G_IO_FLAG_GET_MASK, G_IO_FLAG_MASK, G_IO_FLAG_IS_SEEKABLE, G_IO_FLAG_IS_WRITEABLE,
    G_IO_FLAG_IS_WRITABLE, G_IO_FLAG_IS_READABLE, G_IO_FLAG_NONBLOCK, G_IO_FLAG_APPEND,
    G_IO_FLAG_NONE, GIOStatus, G_IO_STATUS_AGAIN, G_IO_STATUS_EOF, G_IO_STATUS_NORMAL,
    G_IO_STATUS_ERROR, GSeekType, G_SEEK_END, G_SEEK_SET, G_SEEK_CUR,
};
pub use self::gqueue_h::{_GQueue, GQueue};
pub use self::gregex_h::{
    GRegexCompileFlags, G_REGEX_JAVASCRIPT_COMPAT, G_REGEX_BSR_ANYCRLF,
    G_REGEX_NEWLINE_ANYCRLF, G_REGEX_NEWLINE_CRLF, G_REGEX_NEWLINE_LF,
    G_REGEX_NEWLINE_CR, G_REGEX_DUPNAMES, G_REGEX_FIRSTLINE, G_REGEX_OPTIMIZE,
    G_REGEX_NO_AUTO_CAPTURE, G_REGEX_RAW, G_REGEX_UNGREEDY, G_REGEX_DOLLAR_ENDONLY,
    G_REGEX_ANCHORED, G_REGEX_EXTENDED, G_REGEX_DOTALL, G_REGEX_MULTILINE,
    G_REGEX_CASELESS, G_REGEX_DEFAULT, GRegexMatchFlags, G_REGEX_MATCH_NOTEMPTY_ATSTART,
    G_REGEX_MATCH_PARTIAL_HARD, G_REGEX_MATCH_PARTIAL_SOFT, G_REGEX_MATCH_BSR_ANY,
    G_REGEX_MATCH_BSR_ANYCRLF, G_REGEX_MATCH_NEWLINE_ANYCRLF, G_REGEX_MATCH_NEWLINE_ANY,
    G_REGEX_MATCH_NEWLINE_CRLF, G_REGEX_MATCH_NEWLINE_LF, G_REGEX_MATCH_NEWLINE_CR,
    G_REGEX_MATCH_PARTIAL, G_REGEX_MATCH_NOTEMPTY, G_REGEX_MATCH_NOTEOL,
    G_REGEX_MATCH_NOTBOL, G_REGEX_MATCH_ANCHORED, G_REGEX_MATCH_DEFAULT, GRegex, _GRegex,
    g_regex_new, g_regex_replace_literal,
};
pub use self::gtree_h::{GTree, _GTree};
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
pub use self::globalconf_h::{globalconf_t, globalconf};
pub use self::log_h::{
    log_level_t, LOG_LEVEL_debug, LOG_LEVEL_verbose, LOG_LEVEL_info, LOG_LEVEL_warn,
    LOG_LEVEL_error, LOG_LEVEL_fatal,
};
pub use self::struct_timeval_h::timeval;
pub use self::luaclass_h::{
    lua_class_t, lua_class_propfunc_t, lua_object_t, lua_class_property_array_t,
    lua_class_allocator_t, luaH_class_emit_signal,
};
pub use self::signal_h::signal_t;
pub use self::ipc_h::{
    ipc_type_t, IPC_TYPE_crash, IPC_TYPE_page_created, IPC_TYPE_log, IPC_TYPE_eval_js,
    IPC_TYPE_extension_init, IPC_TYPE_scroll, IPC_TYPE_lua_ipc,
    IPC_TYPE_lua_require_module, _ipc_header_t, ipc_header_t, _ipc_recv_state_t,
    ipc_recv_state_t, ipc_endpoint_status_t, IPC_ENDPOINT_FREED, IPC_ENDPOINT_CONNECTED,
    IPC_ENDPOINT_DISCONNECTED, _ipc_endpoint_t, ipc_endpoint_t,
};
use self::string_h::{memcpy, memmove, memcmp, strcmp, strncmp, strrchr, strlen};
use self::stdlib_h::{exit, getenv, unsetenv};
use self::gfileutils_h::{g_file_get_contents, g_file_open_tmp, g_build_path};
use self::gmem_h::{g_free, g_malloc};
pub use self::gstrfuncs_h::{
    g_strdup_inline, g_str_has_prefix, g_ascii_strtod, g_strdup, g_strdup_printf,
    g_strdup_vprintf,
};
use self::gslice_h::{g_slice_alloc0, g_slice_free1};
use self::gtestutils_h::{
    g_assertion_message_expr, g_assertion_message_cmpint, g_assertion_message_error,
};
use self::unistd_h::{close, write, isatty, unlink};
use self::stdio_h::stderr;
use self::luaserialize_h::lua_deserialize_range;
use self::time_h::gettimeofday;
pub use self::util_h::{l_time, file_exists, strip_ansi_escapes};
use self::msg_h::msg_lib_get_msg_class;
use self::gprintf_h::g_fprintf;
#[c2rust::src_loc = "138:1"]
pub type queued_log_t = _queued_log_t;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "138:16"]
pub struct _queued_log_t {
    pub lvl: log_level_t,
    pub time: std::ffi::c_double,
    pub group: *mut std::ffi::c_char,
    pub msg: *mut std::ffi::c_char,
}
#[c2rust::src_loc = "32:20"]
static mut group_levels: *mut GHashTable = 0 as *const GHashTable as *mut GHashTable;
#[c2rust::src_loc = "33:21"]
static mut queued_emissions: *mut GAsyncQueue = 0 as *const GAsyncQueue
    as *mut GAsyncQueue;
#[c2rust::src_loc = "34:17"]
static mut block_log: gboolean = 0 as std::ffi::c_int;
#[no_mangle]
#[c2rust::src_loc = "36:1"]
pub unsafe extern "C" fn log_set_verbosity(
    mut group: *const std::ffi::c_char,
    mut lvl: log_level_t,
) {
    group_levels = if !group_levels.is_null() {
        group_levels
    } else {
        g_hash_table_new_full(
            Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(
                g_str_equal
                    as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean,
            ),
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
            None,
        )
    };
    g_hash_table_insert(
        group_levels,
        g_strdup_inline(group) as gpointer,
        (lvl as std::ffi::c_uint).wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint)
            as glong as gpointer,
    );
}
#[no_mangle]
#[c2rust::src_loc = "44:1"]
pub unsafe extern "C" fn log_get_verbosity(
    mut group: *mut std::ffi::c_char,
) -> log_level_t {
    if group_levels.is_null() {
        return LOG_LEVEL_info;
    }
    let mut len: gint = strlen(group) as gint;
    let mut lvl: log_level_t = LOG_LEVEL_fatal;
    while 0 as std::ffi::c_int == 0 {
        lvl = g_hash_table_lookup(group_levels, group as gpointer as gconstpointer)
            as gulong as guint as log_level_t;
        if lvl as std::ffi::c_uint > 0 as std::ffi::c_int as std::ffi::c_uint {
            break;
        }
        let mut slash: *mut std::ffi::c_char = strrchr(group, '/' as i32);
        if !slash.is_null() {
            *slash = '\0' as i32 as std::ffi::c_char;
        } else {
            lvl = g_hash_table_lookup(
                group_levels,
                b"all\0" as *const u8 as *const std::ffi::c_char as gpointer
                    as gconstpointer,
            ) as gulong as guint as log_level_t;
            break;
        }
    }
    let mut i: gint = 0 as std::ffi::c_int;
    while i < len {
        if *group.offset(i as isize) as std::ffi::c_int == '\0' as i32 {
            *group.offset(i as isize) = '/' as i32 as std::ffi::c_char;
        }
        i += 1;
        i;
    }
    return (lvl as std::ffi::c_uint)
        .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint) as log_level_t;
}
#[c2rust::src_loc = "72:1"]
unsafe extern "C" fn log_group_from_fct(
    mut fct: *const std::ffi::c_char,
) -> *mut std::ffi::c_char {
    static mut paths: *mut GPtrArray = 0 as *const GPtrArray as *mut GPtrArray;
    if paths.is_null() {
        paths = g_ptr_array_new_with_free_func(
            Some(g_free as unsafe extern "C" fn(gpointer) -> ()),
        );
        g_ptr_array_add(
            paths,
            b"./\0" as *const u8 as *const std::ffi::c_char as gpointer,
        );
        g_ptr_array_add(
            paths,
            g_build_path(
                b"/\0" as *const u8 as *const std::ffi::c_char,
                b"/usr/local/share/luakit\0" as *const u8 as *const std::ffi::c_char,
                b"lib/\0" as *const u8 as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void,
            ) as gpointer,
        );
        g_ptr_array_add(
            paths,
            g_build_path(
                b"/\0" as *const u8 as *const std::ffi::c_char,
                b"/etc/xdg\0" as *const u8 as *const std::ffi::c_char,
                b"/luakit/\0" as *const u8 as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void,
            ) as gpointer,
        );
        g_ptr_array_add(
            paths,
            g_build_path(
                b"/\0" as *const u8 as *const std::ffi::c_char,
                globalconf.config_dir,
                b"/\0" as *const u8 as *const std::ffi::c_char,
                0 as *mut std::ffi::c_void,
            ) as gpointer,
        );
    }
    let mut i: std::ffi::c_uint = 0 as std::ffi::c_int as std::ffi::c_uint;
    while i < (*paths).len {
        if if 0 != 0 {
            ({
                let __str: *const std::ffi::c_char = fct;
                let __prefix: *const std::ffi::c_char = *((*paths).pdata)
                    .offset(i as isize) as *const std::ffi::c_char;
                let mut __result: gboolean = 0 as std::ffi::c_int;
                if __str.is_null() || __prefix.is_null() {
                    __result = g_str_has_prefix(__str, __prefix);
                } else {
                    let __str_len: size_t = strlen(
                        __str.offset(__str.is_null() as std::ffi::c_int as isize),
                    );
                    let __prefix_len: size_t = strlen(
                        __prefix.offset(__prefix.is_null() as std::ffi::c_int as isize),
                    );
                    if __str_len >= __prefix_len {
                        __result = (memcmp(
                            __str.offset(__str.is_null() as std::ffi::c_int as isize)
                                as *const std::ffi::c_void,
                            __prefix
                                .offset(__prefix.is_null() as std::ffi::c_int as isize)
                                as *const std::ffi::c_void,
                            __prefix_len,
                        ) == 0 as std::ffi::c_int) as std::ffi::c_int;
                    }
                }
                __result
            })
        } else {
            g_str_has_prefix(fct, *((*paths).pdata).offset(i as isize) as *const gchar)
        } != 0
        {
            fct = fct
                .offset(
                    strlen(
                        *((*paths).pdata).offset(i as isize) as *const std::ffi::c_char,
                    ) as isize,
                );
            break;
        } else {
            i = i.wrapping_add(1);
            i;
        }
    }
    let mut len: std::ffi::c_int = strlen(fct) as std::ffi::c_int;
    let mut core: gboolean = (strcmp(
        &*fct.offset((len - 2 as std::ffi::c_int) as isize),
        b".c\0" as *const u8 as *const std::ffi::c_char,
    ) == 0
        || strcmp(
            &*fct.offset((len - 2 as std::ffi::c_int) as isize),
            b".h\0" as *const u8 as *const std::ffi::c_char,
        ) == 0) as std::ffi::c_int;
    let mut lua: gboolean = (strcmp(
        &*fct.offset((len - 4 as std::ffi::c_int) as isize),
        b".lua\0" as *const u8 as *const std::ffi::c_char,
    ) == 0
        || strncmp(
            fct,
            b"[string \"\0" as *const u8 as *const std::ffi::c_char,
            9 as std::ffi::c_int as std::ffi::c_ulong,
        ) == 0) as std::ffi::c_int;
    if core != 0 {
        return g_strdup_printf(
            b"core/%.*s\0" as *const u8 as *const std::ffi::c_char,
            len - 2 as std::ffi::c_int,
            fct,
        )
    } else if lua != 0 {
        return g_strdup_printf(
            b"lua/%.*s\0" as *const u8 as *const std::ffi::c_char,
            len - 4 as std::ffi::c_int,
            fct,
        )
    } else {
        return g_strdup_inline(fct)
    };
}
#[no_mangle]
#[c2rust::src_loc = "102:1"]
pub unsafe extern "C" fn log_level_from_string(
    mut out: *mut log_level_t,
    mut str: *const std::ffi::c_char,
) -> std::ffi::c_int {
    if strcmp(b"fatal\0" as *const u8 as *const std::ffi::c_char, str) == 0 {
        *out = LOG_LEVEL_fatal;
        return 0 as std::ffi::c_int;
    }
    if strcmp(b"error\0" as *const u8 as *const std::ffi::c_char, str) == 0 {
        *out = LOG_LEVEL_error;
        return 0 as std::ffi::c_int;
    }
    if strcmp(b"warn\0" as *const u8 as *const std::ffi::c_char, str) == 0 {
        *out = LOG_LEVEL_warn;
        return 0 as std::ffi::c_int;
    }
    if strcmp(b"info\0" as *const u8 as *const std::ffi::c_char, str) == 0 {
        *out = LOG_LEVEL_info;
        return 0 as std::ffi::c_int;
    }
    if strcmp(b"verbose\0" as *const u8 as *const std::ffi::c_char, str) == 0 {
        *out = LOG_LEVEL_verbose;
        return 0 as std::ffi::c_int;
    }
    if strcmp(b"debug\0" as *const u8 as *const std::ffi::c_char, str) == 0 {
        *out = LOG_LEVEL_debug;
        return 0 as std::ffi::c_int;
    }
    return 1 as std::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "114:1"]
pub unsafe extern "C" fn log_string_from_level(
    mut lvl: log_level_t,
) -> *const std::ffi::c_char {
    match lvl as std::ffi::c_uint {
        0 => return b"fatal\0" as *const u8 as *const std::ffi::c_char,
        1 => return b"error\0" as *const u8 as *const std::ffi::c_char,
        2 => return b"warn\0" as *const u8 as *const std::ffi::c_char,
        3 => return b"info\0" as *const u8 as *const std::ffi::c_char,
        4 => return b"verbose\0" as *const u8 as *const std::ffi::c_char,
        5 => return b"debug\0" as *const u8 as *const std::ffi::c_char,
        _ => {}
    }
    g_assertion_message_expr(
        0 as *mut gchar,
        b"log.c\0" as *const u8 as *const std::ffi::c_char,
        122 as std::ffi::c_int,
        (*::core::mem::transmute::<
            &[u8; 22],
            &[std::ffi::c_char; 22],
        >(b"log_string_from_level\0"))
            .as_ptr(),
        0 as *const std::ffi::c_char,
    );
}
#[c2rust::src_loc = "125:1"]
unsafe extern "C" fn emit_log_signal(
    mut time: std::ffi::c_double,
    mut lvl: log_level_t,
    mut group: *const gchar,
    mut msg: *const gchar,
) {
    let mut msg_class: *mut lua_class_t = msg_lib_get_msg_class();
    lua_pushnumber(common.L, time);
    lua_pushstring(common.L, log_string_from_level(lvl));
    lua_pushstring(common.L, group);
    lua_pushstring(common.L, msg);
    block_log = (0 as std::ffi::c_int == 0) as std::ffi::c_int;
    luaH_class_emit_signal(
        common.L,
        msg_class,
        b"log\0" as *const u8 as *const std::ffi::c_char,
        4 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    block_log = 0 as std::ffi::c_int;
}
#[c2rust::src_loc = "145:12"]
static mut consumer_added: std::ffi::c_int = 0 as std::ffi::c_int;
#[c2rust::src_loc = "147:1"]
unsafe extern "C" fn log_emit_pending_signals(
    mut UNUSED_usedata: *mut std::ffi::c_void,
) -> gboolean {
    let mut entry: *mut queued_log_t = 0 as *mut queued_log_t;
    loop {
        entry = g_async_queue_try_pop(queued_emissions) as *mut queued_log_t;
        if entry.is_null() {
            break;
        }
        emit_log_signal((*entry).time, (*entry).lvl, (*entry).group, (*entry).msg);
        g_free((*entry).group as gpointer);
        g_free((*entry).msg as gpointer);
        g_slice_free1(
            ::core::mem::size_of::<queued_log_t>() as std::ffi::c_ulong,
            entry as gpointer,
        );
    }
    let mut gais_temp: gint = 0 as std::ffi::c_int;
    if 0 as std::ffi::c_int != 0 {
        consumer_added;
    } else {};
    ::core::intrinsics::atomic_store_seqcst(
        &mut consumer_added as *mut std::ffi::c_int as *mut gint,
        *&mut gais_temp,
    );
    return 0 as std::ffi::c_int;
}
#[c2rust::src_loc = "162:1"]
unsafe extern "C" fn queue_log_signal(
    mut time: std::ffi::c_double,
    mut lvl: log_level_t,
    mut group: *const gchar,
    mut msg: *const gchar,
) {
    let mut entry: *mut queued_log_t = g_slice_alloc0(
        ::core::mem::size_of::<queued_log_t>() as std::ffi::c_ulong,
    ) as *mut queued_log_t;
    (*entry).time = time;
    (*entry).lvl = lvl;
    (*entry).group = g_strdup_inline(group);
    (*entry).msg = g_strdup_inline(msg);
    g_async_queue_push(queued_emissions, entry as gpointer);
    if ({
        let mut gaicae_oldval: gint = 0 as std::ffi::c_int;
        if 0 as std::ffi::c_int != 0 {
            consumer_added;
            (0 as std::ffi::c_int == 0) as std::ffi::c_int;
        } else {};
        let fresh1 = ::core::intrinsics::atomic_cxchg_seqcst_seqcst(
            &mut consumer_added as *mut std::ffi::c_int,
            *(&mut gaicae_oldval as *mut gint as *mut std::ffi::c_void
                as *mut std::ffi::c_int),
            (0 as std::ffi::c_int == 0) as std::ffi::c_int,
        );
        *(&mut gaicae_oldval as *mut gint as *mut std::ffi::c_void
            as *mut std::ffi::c_int) = fresh1.0;
        if fresh1.1 as std::ffi::c_int != 0 {
            (0 as std::ffi::c_int == 0) as std::ffi::c_int
        } else {
            0 as std::ffi::c_int
        }
    }) != 0
    {
        g_idle_add(
            Some(
                log_emit_pending_signals
                    as unsafe extern "C" fn(*mut std::ffi::c_void) -> gboolean,
            ),
            0 as *mut std::ffi::c_void,
        );
    }
}
#[no_mangle]
#[c2rust::src_loc = "177:1"]
pub unsafe extern "C" fn _log(
    mut lvl: log_level_t,
    mut fct: *const gchar,
    mut fmt: *const gchar,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    va_log(lvl, fct, fmt, ap.as_va_list());
}
#[no_mangle]
#[c2rust::src_loc = "186:1"]
pub unsafe extern "C" fn va_log(
    mut lvl: log_level_t,
    mut fct: *const gchar,
    mut fmt: *const gchar,
    mut ap: ::core::ffi::VaList,
) {
    let mut msg: *mut gchar = 0 as *mut gchar;
    let mut log_fd: gint = 0;
    let mut time: std::ffi::c_double = 0.;
    let mut prefix_char: gchar = 0;
    let mut style: *mut gchar = 0 as *mut gchar;
    static mut indent_lines_reg: *mut GRegex = 0 as *const GRegex as *mut GRegex;
    let mut wrapped: *mut gchar = 0 as *mut gchar;
    if block_log != 0 {
        return;
    }
    let mut group: *mut std::ffi::c_char = log_group_from_fct(fct);
    let mut verbosity: log_level_t = log_get_verbosity(group);
    if !(lvl as std::ffi::c_uint > verbosity as std::ffi::c_uint) {
        msg = g_strdup_vprintf(fmt, ap.as_va_list());
        log_fd = 2 as std::ffi::c_int;
        time = l_time() - globalconf.starttime;
        queue_log_signal(time, lvl, group, msg);
        prefix_char = 0;
        style = b"\0" as *const u8 as *const std::ffi::c_char as *mut gchar;
        match lvl as std::ffi::c_uint {
            0 => {
                prefix_char = 'F' as i32 as gchar;
                style = b"\x1B[41m\0" as *const u8 as *const std::ffi::c_char
                    as *mut gchar;
            }
            1 => {
                prefix_char = 'E' as i32 as gchar;
                style = b"\x1B[31m\0" as *const u8 as *const std::ffi::c_char
                    as *mut gchar;
            }
            2 => {
                prefix_char = 'W' as i32 as gchar;
                style = b"\x1B[33m\0" as *const u8 as *const std::ffi::c_char
                    as *mut gchar;
            }
            3 => {
                prefix_char = 'I' as i32 as gchar;
            }
            4 => {
                prefix_char = 'V' as i32 as gchar;
            }
            5 => {
                prefix_char = 'D' as i32 as gchar;
            }
            _ => {
                g_assertion_message_expr(
                    0 as *mut gchar,
                    b"log.c\0" as *const u8 as *const std::ffi::c_char,
                    214 as std::ffi::c_int,
                    (*::core::mem::transmute::<
                        &[u8; 7],
                        &[std::ffi::c_char; 7],
                    >(b"va_log\0"))
                        .as_ptr(),
                    0 as *const std::ffi::c_char,
                );
            }
        }
        if indent_lines_reg.is_null() {
            let mut err: *mut GError = 0 as *mut GError;
            indent_lines_reg = g_regex_new(
                b"\n\0" as *const u8 as *const std::ffi::c_char,
                G_REGEX_DEFAULT,
                G_REGEX_MATCH_DEFAULT,
                &mut err,
            );
            if !err.is_null() {
                g_assertion_message_error(
                    0 as *mut gchar,
                    b"log.c\0" as *const u8 as *const std::ffi::c_char,
                    226 as std::ffi::c_int,
                    (*::core::mem::transmute::<
                        &[u8; 7],
                        &[std::ffi::c_char; 7],
                    >(b"va_log\0"))
                        .as_ptr(),
                    b"err\0" as *const u8 as *const std::ffi::c_char,
                    err,
                    0 as std::ffi::c_int as GQuark,
                    0 as std::ffi::c_int,
                );
            }
        }
        wrapped = g_regex_replace_literal(
            indent_lines_reg,
            msg,
            -(1 as std::ffi::c_int) as gssize,
            0 as std::ffi::c_int,
            b"\n                 \0" as *const u8 as *const std::ffi::c_char,
            G_REGEX_MATCH_DEFAULT,
            0 as *mut *mut GError,
        );
        g_free(msg as gpointer);
        msg = wrapped;
        if isatty(log_fd) == 0 {
            let mut stripped: *mut gchar = strip_ansi_escapes(msg);
            g_free(msg as gpointer);
            msg = stripped;
            g_fprintf(
                stderr,
                b"[%#12f] %c [%s]: %s\n\0" as *const u8 as *const std::ffi::c_char,
                time,
                prefix_char as std::ffi::c_int,
                group,
                msg,
            );
        } else {
            g_fprintf(
                stderr,
                b"%s[%#12f] %c [%s]: %s\x1B[0m\n\0" as *const u8
                    as *const std::ffi::c_char,
                style,
                time,
                prefix_char as std::ffi::c_int,
                group,
                msg,
            );
        }
        g_free(msg as gpointer);
        if lvl as std::ffi::c_uint
            == LOG_LEVEL_fatal as std::ffi::c_int as std::ffi::c_uint
        {
            exit(1 as std::ffi::c_int);
        }
    }
    g_free(group as gpointer);
}
#[no_mangle]
#[c2rust::src_loc = "251:1"]
pub unsafe extern "C" fn ipc_recv_log(
    mut UNUSED_ipc: *mut ipc_endpoint_t,
    mut lua_msg: *const guint8,
    mut length: guint,
) {
    let mut L: *mut lua_State = common.L;
    let mut n: gint = lua_deserialize_range(L, lua_msg, length);
    let mut __n1: gint64 = n as gint64;
    let mut __n2: gint64 = 3 as std::ffi::c_int as gint64;
    if !(__n1 == __n2) {
        g_assertion_message_cmpint(
            0 as *mut gchar,
            b"log.c\0" as *const u8 as *const std::ffi::c_char,
            256 as std::ffi::c_int,
            (*::core::mem::transmute::<
                &[u8; 13],
                &[std::ffi::c_char; 13],
            >(b"ipc_recv_log\0"))
                .as_ptr(),
            b"n == 3\0" as *const u8 as *const std::ffi::c_char,
            __n1 as guint64,
            b"==\0" as *const u8 as *const std::ffi::c_char,
            __n2 as guint64,
            'i' as i32 as std::ffi::c_char,
        );
    }
    let mut lvl: log_level_t = lua_tointeger(L, -(3 as std::ffi::c_int)) as log_level_t;
    let mut fct: *const gchar = lua_tolstring(
        L,
        -(2 as std::ffi::c_int),
        0 as *mut size_t,
    );
    let mut msg: *const gchar = lua_tolstring(
        L,
        -(1 as std::ffi::c_int),
        0 as *mut size_t,
    );
    _log(lvl, fct, b"%s\0" as *const u8 as *const std::ffi::c_char, msg);
    lua_settop(L, -(3 as std::ffi::c_int) - 1 as std::ffi::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "265:1"]
pub unsafe extern "C" fn log_init() {
    queued_emissions = g_async_queue_new();
    let mut log_dump_file: *const std::ffi::c_char = getenv(
        b"LUAKIT_QUEUED_EMISSIONS_FILE\0" as *const u8 as *const std::ffi::c_char,
    );
    unsetenv(b"LUAKIT_QUEUED_EMISSIONS_FILE\0" as *const u8 as *const std::ffi::c_char);
    if log_dump_file.is_null() || file_exists(log_dump_file) == 0 {
        return;
    }
    let mut dump: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut len: size_t = 0;
    let mut error: *mut GError = 0 as *mut GError;
    if g_file_get_contents(log_dump_file, &mut dump, &mut len, &mut error) == 0 {
        _log(
            LOG_LEVEL_error,
            b"log.c\0" as *const u8 as *const std::ffi::c_char,
            b"unable to load previous log messages: %s\0" as *const u8
                as *const std::ffi::c_char,
            (*error).message,
        );
        g_error_free(error);
        return;
    }
    unlink(log_dump_file);
    let mut end: *mut std::ffi::c_char = dump.offset(len as isize);
    g_async_queue_lock(queued_emissions);
    while dump < end {
        let mut entry: *mut queued_log_t = g_slice_alloc0(
            ::core::mem::size_of::<queued_log_t>() as std::ffi::c_ulong,
        ) as *mut queued_log_t;
        let fresh2 = dump;
        dump = dump.offset(1);
        (*entry).lvl = *fresh2 as log_level_t;
        (*entry).time = g_ascii_strtod(dump, &mut dump);
        (*entry).group = g_strdup_inline(dump);
        dump = dump
            .offset(
                (strlen(dump)).wrapping_add(1 as std::ffi::c_int as std::ffi::c_ulong)
                    as isize,
            );
        (*entry).msg = g_strdup_inline(dump);
        dump = dump
            .offset(
                (strlen(dump)).wrapping_add(1 as std::ffi::c_int as std::ffi::c_ulong)
                    as isize,
            );
        g_async_queue_push_unlocked(queued_emissions, entry as gpointer);
    }
    g_async_queue_unlock(queued_emissions);
}
#[no_mangle]
#[c2rust::src_loc = "302:1"]
pub unsafe extern "C" fn log_dump_queued_emissions() -> *mut std::ffi::c_char {
    let mut dump: *mut GString = g_string_new(0 as *const gchar);
    g_async_queue_lock(queued_emissions);
    let mut entry: *mut queued_log_t = 0 as *mut queued_log_t;
    loop {
        entry = g_async_queue_try_pop_unlocked(queued_emissions) as *mut queued_log_t;
        if entry.is_null() {
            break;
        }
        g_string_append_c_inline(dump, (*entry).lvl as std::ffi::c_char);
        g_string_append_printf(
            dump,
            b"%f\0" as *const u8 as *const std::ffi::c_char,
            (*entry).time,
        );
        if 0 != 0 {
            ({
                let __val: *const std::ffi::c_char = (*entry).group;
                g_string_append_len_inline(
                    dump,
                    __val,
                    if !__val.is_null() {
                        strlen(__val.offset(__val.is_null() as std::ffi::c_int as isize))
                            as gssize
                    } else {
                        -(1 as std::ffi::c_int) as gssize
                    },
                );
                compile_error!("Function call expression is not supposed to be used")
            });
            ({
                let __val: *const std::ffi::c_char = (*entry).group;
                g_string_append_len_inline(
                    dump,
                    __val,
                    if !__val.is_null() {
                        strlen(__val.offset(__val.is_null() as std::ffi::c_int as isize))
                            as gssize
                    } else {
                        -(1 as std::ffi::c_int) as gssize
                    },
                );
                compile_error!("Function call expression is not supposed to be used")
            });
        } else {
            g_string_append_len_inline(
                dump,
                (*entry).group,
                -(1 as std::ffi::c_int) as gssize,
            );
        };
        g_string_append_c_inline(dump, '\0' as i32 as gchar);
        if 0 != 0 {
            ({
                let __val: *const std::ffi::c_char = (*entry).msg;
                g_string_append_len_inline(
                    dump,
                    __val,
                    if !__val.is_null() {
                        strlen(__val.offset(__val.is_null() as std::ffi::c_int as isize))
                            as gssize
                    } else {
                        -(1 as std::ffi::c_int) as gssize
                    },
                );
                compile_error!("Function call expression is not supposed to be used")
            });
            ({
                let __val: *const std::ffi::c_char = (*entry).msg;
                g_string_append_len_inline(
                    dump,
                    __val,
                    if !__val.is_null() {
                        strlen(__val.offset(__val.is_null() as std::ffi::c_int as isize))
                            as gssize
                    } else {
                        -(1 as std::ffi::c_int) as gssize
                    },
                );
                compile_error!("Function call expression is not supposed to be used")
            });
        } else {
            g_string_append_len_inline(
                dump,
                (*entry).msg,
                -(1 as std::ffi::c_int) as gssize,
            );
        };
        g_string_append_c_inline(dump, '\0' as i32 as gchar);
        g_free((*entry).group as gpointer);
        g_free((*entry).msg as gpointer);
        g_slice_free1(
            ::core::mem::size_of::<queued_log_t>() as std::ffi::c_ulong,
            entry as gpointer,
        );
    }
    g_async_queue_unlock(queued_emissions);
    let mut name_used: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut log_dump_fd: std::ffi::c_int = g_file_open_tmp(
        b"luakit-log-dump.XXXXXX\0" as *const u8 as *const std::ffi::c_char,
        &mut name_used,
        0 as *mut *mut GError,
    );
    if log_dump_fd != -(1 as std::ffi::c_int) {
        let mut written: ssize_t = write(
            log_dump_fd,
            (*dump).str_0 as *const std::ffi::c_void,
            (*dump).len,
        );
        close(log_dump_fd);
        if written != (*dump).len as ssize_t {
            unlink(name_used);
            g_free(name_used as gpointer);
            name_used = 0 as *mut std::ffi::c_char;
        }
    }
    if 0 != 0 {
        if 0 as std::ffi::c_int == 0 {
            g_string_free(dump, (0 as std::ffi::c_int == 0) as std::ffi::c_int);
        } else {
            g_string_free_and_steal(dump);
        };
    } else {
        g_string_free(dump, (0 as std::ffi::c_int == 0) as std::ffi::c_int);
    };
    return name_used;
}
