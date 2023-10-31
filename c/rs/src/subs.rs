use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type __sFILEX;
    pub type _GData;
    pub type _PangoContext;
    pub type _PangoLanguage;
    pub type _PangoFontDescription;
    pub type _PangoAttrList;
    pub type _PangoLayout;
    pub type FT_LibraryRec_;
    pub type FT_DriverRec_;
    pub type FT_Face_InternalRec_;
    pub type FT_Size_InternalRec_;
    pub type FT_Slot_InternalRec_;
    pub type FT_SubGlyphRec_;
    pub type _FcPattern;
    static mut __stdoutp: *mut FILE;
    static mut __stderrp: *mut FILE;
    fn fclose(_: *mut FILE) -> libc::c_int;
    fn fputs(_: *const libc::c_char, _: *mut FILE) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: __builtin_va_list,
    ) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputc(_: libc::c_int, _: *mut FILE) -> libc::c_int;
    fn fgets(_: *mut libc::c_char, _: libc::c_int, _: *mut FILE) -> *mut libc::c_char;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn atof(_: *const libc::c_char) -> libc::c_double;
    fn exit(_: libc::c_int) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __maskrune(_: __darwin_ct_rune_t, _: libc::c_ulong) -> libc::c_int;
    fn __tolower(_: __darwin_ct_rune_t) -> __darwin_ct_rune_t;
    static mut _DefaultRuneLocale: _RuneLocale;
    fn g_string_sized_new(dfl_size: gsize) -> *mut GString;
    fn g_string_insert_len(
        string: *mut GString,
        pos: gssize,
        val: *const gchar,
        len: gssize,
    ) -> *mut GString;
    fn g_string_append_len(
        string: *mut GString,
        val: *const gchar,
        len: gssize,
    ) -> *mut GString;
    fn pango_cairo_font_map_get_default() -> *mut PangoFontMap;
    fn pango_font_description_get_size(desc: *const PangoFontDescription) -> gint;
    fn pango_font_description_from_string(
        str: *const libc::c_char,
    ) -> *mut PangoFontDescription;
    fn pango_font_describe(font: *mut PangoFont) -> *mut PangoFontDescription;
    fn pango_attr_size_new(size: libc::c_int) -> *mut PangoAttribute;
    fn pango_attr_font_desc_new(
        desc: *const PangoFontDescription,
    ) -> *mut PangoAttribute;
    fn pango_attr_list_new() -> *mut PangoAttrList;
    fn pango_attr_list_unref(list: *mut PangoAttrList);
    fn pango_attr_list_insert(list: *mut PangoAttrList, attr: *mut PangoAttribute);
    fn pango_font_map_create_context(fontmap: *mut PangoFontMap) -> *mut PangoContext;
    fn pango_layout_new(context: *mut PangoContext) -> *mut PangoLayout;
    fn pango_layout_set_attributes(
        layout_0: *mut PangoLayout,
        attrs_0: *mut PangoAttrList,
    );
    fn pango_layout_set_text(
        layout_0: *mut PangoLayout,
        text: *const libc::c_char,
        length: libc::c_int,
    );
    fn pango_layout_set_width(layout_0: *mut PangoLayout, width: libc::c_int);
    fn pango_layout_set_wrap(layout_0: *mut PangoLayout, wrap: PangoWrapMode);
    fn pango_layout_set_justify(layout_0: *mut PangoLayout, justify: gboolean);
    fn pango_layout_index_to_pos(
        layout_0: *mut PangoLayout,
        index_: libc::c_int,
        pos: *mut PangoRectangle,
    );
    fn pango_layout_get_size(
        layout_0: *mut PangoLayout,
        width: *mut libc::c_int,
        height: *mut libc::c_int,
    );
    fn pango_layout_get_line_readonly(
        layout_0: *mut PangoLayout,
        line: libc::c_int,
    ) -> *mut PangoLayoutLine;
    fn pango_layout_get_lines_readonly(layout_0: *mut PangoLayout) -> *mut GSList;
    fn pango_layout_line_get_extents(
        line: *mut PangoLayoutLine,
        ink_rect: *mut PangoRectangle,
        logical_rect: *mut PangoRectangle,
    );
    fn pango_fc_font_lock_face(font: *mut PangoFcFont) -> FT_Face;
    fn FT_Get_Postscript_Name(face: FT_Face) -> *const libc::c_char;
    fn FT_Get_Glyph_Name(
        face: FT_Face,
        glyph_index: FT_UInt,
        buffer: FT_Pointer,
        buffer_max: FT_UInt,
    ) -> FT_Error;
    fn FT_Load_Glyph(
        face: FT_Face,
        glyph_index: FT_UInt,
        load_flags: FT_Int32,
    ) -> FT_Error;
    fn pango_fc_font_unlock_face(font: *mut PangoFcFont);
    static mut severity: libc::c_int;
    static mut fontnames: [*mut libc::c_char; 30];
    static mut parse: parse;
    static mut cfmt: FORMAT;
    static mut info: INFO;
    static mut mbf: *mut libc::c_char;
    static mut epsf: libc::c_int;
    static mut svg: libc::c_int;
    static mut pipeformat: libc::c_int;
    static mut fout: *mut FILE;
    static mut first_voice: *mut VOICE_S;
    fn get_str(
        d: *mut libc::c_char,
        s: *mut libc::c_char,
        maxlen: libc::c_int,
    ) -> *mut libc::c_char;
    fn a2b(fmt: *mut libc::c_char, _: ...);
    fn buffer_eob(eot: libc::c_int);
    fn bskip(h: libc::c_float);
    fn tempo_width(s: *mut SYMBOL) -> libc::c_float;
    fn write_tempo(s: *mut SYMBOL, beat: libc::c_int, sc: libc::c_float);
    fn get_font_encoding(ft: libc::c_int) -> libc::c_int;
    fn set_font(ft: libc::c_int);
    fn glyph_out(p: *mut libc::c_char) -> *mut libc::c_char;
    fn svg_write(buf: *mut libc::c_char, len: libc::c_int);
}
pub type __builtin_va_list = *mut libc::c_char;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_longlong;
pub type __darwin_ct_rune_t = libc::c_int;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_va_list = __builtin_va_list;
pub type __darwin_wchar_t = libc::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_off_t = __int64_t;
pub type va_list = __darwin_va_list;
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut libc::c_uchar,
    pub _size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut libc::c_uchar,
    pub _r: libc::c_int,
    pub _w: libc::c_int,
    pub _flags: libc::c_short,
    pub _file: libc::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: libc::c_int,
    pub _cookie: *mut libc::c_void,
    pub _close: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub _read: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub _seek: Option::<
        unsafe extern "C" fn(*mut libc::c_void, fpos_t, libc::c_int) -> fpos_t,
    >,
    pub _write: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: libc::c_int,
    pub _ubuf: [libc::c_uchar; 3],
    pub _nbuf: [libc::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: libc::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneEntry {
    pub __min: __darwin_rune_t,
    pub __max: __darwin_rune_t,
    pub __map: __darwin_rune_t,
    pub __types: *mut __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneRange {
    pub __nranges: libc::c_int,
    pub __ranges: *mut _RuneEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneCharClass {
    pub __name: [libc::c_char; 14],
    pub __mask: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneLocale {
    pub __magic: [libc::c_char; 8],
    pub __encoding: [libc::c_char; 32],
    pub __sgetrune: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            __darwin_size_t,
            *mut *const libc::c_char,
        ) -> __darwin_rune_t,
    >,
    pub __sputrune: Option::<
        unsafe extern "C" fn(
            __darwin_rune_t,
            *mut libc::c_char,
            __darwin_size_t,
            *mut *mut libc::c_char,
        ) -> libc::c_int,
    >,
    pub __invalid_rune: __darwin_rune_t,
    pub __runetype: [__uint32_t; 256],
    pub __maplower: [__darwin_rune_t; 256],
    pub __mapupper: [__darwin_rune_t; 256],
    pub __runetype_ext: _RuneRange,
    pub __maplower_ext: _RuneRange,
    pub __mapupper_ext: _RuneRange,
    pub __variable: *mut libc::c_void,
    pub __variable_len: libc::c_int,
    pub __ncharclasses: libc::c_int,
    pub __charclasses: *mut _RuneCharClass,
}
pub type guint8 = libc::c_uchar;
pub type gint32 = libc::c_int;
pub type guint32 = libc::c_uint;
pub type gssize = libc::c_long;
pub type gsize = libc::c_ulong;
pub type gchar = libc::c_char;
pub type gint = libc::c_int;
pub type gboolean = gint;
pub type guint = libc::c_uint;
pub type gpointer = *mut libc::c_void;
pub type GData = _GData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}
pub type GSList = _GSList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GString {
    pub str_0: *mut gchar,
    pub len: gsize,
    pub allocated_len: gsize,
}
pub type GString = _GString;
pub type GType = gsize;
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
pub struct _PangoEngineLang {
    pub parent_instance: PangoEngine,
}
pub type PangoEngine = _PangoEngine;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _PangoEngine {
    pub parent_instance: GObject,
}
pub type PangoEngineLang = _PangoEngineLang;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _PangoEngineShape {
    pub parent_instance: PangoEngine,
}
pub type PangoEngineShape = _PangoEngineShape;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _PangoFont {
    pub parent_instance: GObject,
}
pub type PangoFont = _PangoFont;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _PangoFontMap {
    pub parent_instance: GObject,
}
pub type PangoFontMap = _PangoFontMap;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _PangoRectangle {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
}
pub type PangoRectangle = _PangoRectangle;
pub type PangoContext = _PangoContext;
pub type PangoLanguage = _PangoLanguage;
pub type PangoGlyph = guint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _PangoMatrix {
    pub xx: libc::c_double,
    pub xy: libc::c_double,
    pub yx: libc::c_double,
    pub yy: libc::c_double,
    pub x0: libc::c_double,
    pub y0: libc::c_double,
}
pub type PangoMatrix = _PangoMatrix;
pub type PangoFontDescription = _PangoFontDescription;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _PangoAttribute {
    pub klass: *const PangoAttrClass,
    pub start_index: guint,
    pub end_index: guint,
}
pub type PangoAttrClass = _PangoAttrClass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _PangoAttrClass {
    pub type_0: PangoAttrType,
    pub copy: Option::<
        unsafe extern "C" fn(*const PangoAttribute) -> *mut PangoAttribute,
    >,
    pub destroy: Option::<unsafe extern "C" fn(*mut PangoAttribute) -> ()>,
    pub equal: Option::<
        unsafe extern "C" fn(*const PangoAttribute, *const PangoAttribute) -> gboolean,
    >,
}
pub type PangoAttribute = _PangoAttribute;
pub type PangoAttrType = libc::c_uint;
pub const PANGO_ATTR_FONT_SCALE: PangoAttrType = 37;
pub const PANGO_ATTR_BASELINE_SHIFT: PangoAttrType = 36;
pub const PANGO_ATTR_SENTENCE: PangoAttrType = 35;
pub const PANGO_ATTR_WORD: PangoAttrType = 34;
pub const PANGO_ATTR_TEXT_TRANSFORM: PangoAttrType = 33;
pub const PANGO_ATTR_ABSOLUTE_LINE_HEIGHT: PangoAttrType = 32;
pub const PANGO_ATTR_LINE_HEIGHT: PangoAttrType = 31;
pub const PANGO_ATTR_OVERLINE_COLOR: PangoAttrType = 30;
pub const PANGO_ATTR_OVERLINE: PangoAttrType = 29;
pub const PANGO_ATTR_INSERT_HYPHENS: PangoAttrType = 28;
pub const PANGO_ATTR_SHOW: PangoAttrType = 27;
pub const PANGO_ATTR_ALLOW_BREAKS: PangoAttrType = 26;
pub const PANGO_ATTR_BACKGROUND_ALPHA: PangoAttrType = 25;
pub const PANGO_ATTR_FOREGROUND_ALPHA: PangoAttrType = 24;
pub const PANGO_ATTR_FONT_FEATURES: PangoAttrType = 23;
pub const PANGO_ATTR_GRAVITY_HINT: PangoAttrType = 22;
pub const PANGO_ATTR_GRAVITY: PangoAttrType = 21;
pub const PANGO_ATTR_ABSOLUTE_SIZE: PangoAttrType = 20;
pub const PANGO_ATTR_STRIKETHROUGH_COLOR: PangoAttrType = 19;
pub const PANGO_ATTR_UNDERLINE_COLOR: PangoAttrType = 18;
pub const PANGO_ATTR_LETTER_SPACING: PangoAttrType = 17;
pub const PANGO_ATTR_FALLBACK: PangoAttrType = 16;
pub const PANGO_ATTR_SCALE: PangoAttrType = 15;
pub const PANGO_ATTR_SHAPE: PangoAttrType = 14;
pub const PANGO_ATTR_RISE: PangoAttrType = 13;
pub const PANGO_ATTR_STRIKETHROUGH: PangoAttrType = 12;
pub const PANGO_ATTR_UNDERLINE: PangoAttrType = 11;
pub const PANGO_ATTR_BACKGROUND: PangoAttrType = 10;
pub const PANGO_ATTR_FOREGROUND: PangoAttrType = 9;
pub const PANGO_ATTR_FONT_DESC: PangoAttrType = 8;
pub const PANGO_ATTR_SIZE: PangoAttrType = 7;
pub const PANGO_ATTR_STRETCH: PangoAttrType = 6;
pub const PANGO_ATTR_VARIANT: PangoAttrType = 5;
pub const PANGO_ATTR_WEIGHT: PangoAttrType = 4;
pub const PANGO_ATTR_STYLE: PangoAttrType = 3;
pub const PANGO_ATTR_FAMILY: PangoAttrType = 2;
pub const PANGO_ATTR_LANGUAGE: PangoAttrType = 1;
pub const PANGO_ATTR_INVALID: PangoAttrType = 0;
pub type PangoAttrList = _PangoAttrList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _PangoAnalysis {
    pub shape_engine: *mut PangoEngineShape,
    pub lang_engine: *mut PangoEngineLang,
    pub font: *mut PangoFont,
    pub level: guint8,
    pub gravity: guint8,
    pub flags: guint8,
    pub script: guint8,
    pub language: *mut PangoLanguage,
    pub extra_attrs: *mut GSList,
}
pub type PangoAnalysis = _PangoAnalysis;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _PangoItem {
    pub offset: libc::c_int,
    pub length: libc::c_int,
    pub num_chars: libc::c_int,
    pub analysis: PangoAnalysis,
}
pub type PangoItem = _PangoItem;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _PangoGlyphGeometry {
    pub width: PangoGlyphUnit,
    pub x_offset: PangoGlyphUnit,
    pub y_offset: PangoGlyphUnit,
}
pub type PangoGlyphUnit = gint32;
pub type PangoGlyphGeometry = _PangoGlyphGeometry;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _PangoGlyphVisAttr {
    #[bitfield(name = "is_cluster_start", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "is_color", ty = "guint", bits = "1..=1")]
    pub is_cluster_start_is_color: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type PangoGlyphVisAttr = _PangoGlyphVisAttr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _PangoGlyphInfo {
    pub glyph: PangoGlyph,
    pub geometry: PangoGlyphGeometry,
    pub attr: PangoGlyphVisAttr,
}
pub type PangoGlyphInfo = _PangoGlyphInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _PangoGlyphString {
    pub num_glyphs: libc::c_int,
    pub glyphs: *mut PangoGlyphInfo,
    pub log_clusters: *mut libc::c_int,
    pub space: libc::c_int,
}
pub type PangoGlyphString = _PangoGlyphString;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _PangoGlyphItem {
    pub item: *mut PangoItem,
    pub glyphs: *mut PangoGlyphString,
    pub y_offset: libc::c_int,
    pub start_x_offset: libc::c_int,
    pub end_x_offset: libc::c_int,
}
pub type PangoGlyphItem = _PangoGlyphItem;
pub type PangoLayout = _PangoLayout;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _PangoLayoutLine {
    pub layout: *mut PangoLayout,
    pub start_index: gint,
    pub length: gint,
    pub runs: *mut GSList,
    #[bitfield(name = "is_paragraph_start", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "resolved_dir", ty = "guint", bits = "1..=3")]
    pub is_paragraph_start_resolved_dir: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type PangoLayoutLine = _PangoLayoutLine;
pub type PangoLayoutRun = PangoGlyphItem;
pub type PangoWrapMode = libc::c_uint;
pub const PANGO_WRAP_WORD_CHAR: PangoWrapMode = 2;
pub const PANGO_WRAP_CHAR: PangoWrapMode = 1;
pub const PANGO_WRAP_WORD: PangoWrapMode = 0;
pub type FT_Int32 = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_MemoryRec_ {
    pub user: *mut libc::c_void,
    pub alloc: FT_Alloc_Func,
    pub free: FT_Free_Func,
    pub realloc: FT_Realloc_Func,
}
pub type FT_Realloc_Func = Option::<
    unsafe extern "C" fn(
        FT_Memory,
        libc::c_long,
        libc::c_long,
        *mut libc::c_void,
    ) -> *mut libc::c_void,
>;
pub type FT_Memory = *mut FT_MemoryRec_;
pub type FT_Free_Func = Option::<
    unsafe extern "C" fn(FT_Memory, *mut libc::c_void) -> (),
>;
pub type FT_Alloc_Func = Option::<
    unsafe extern "C" fn(FT_Memory, libc::c_long) -> *mut libc::c_void,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_StreamRec_ {
    pub base: *mut libc::c_uchar,
    pub size: libc::c_ulong,
    pub pos: libc::c_ulong,
    pub descriptor: FT_StreamDesc,
    pub pathname: FT_StreamDesc,
    pub read: FT_Stream_IoFunc,
    pub close: FT_Stream_CloseFunc,
    pub memory: FT_Memory,
    pub cursor: *mut libc::c_uchar,
    pub limit: *mut libc::c_uchar,
}
pub type FT_Stream_CloseFunc = Option::<unsafe extern "C" fn(FT_Stream) -> ()>;
pub type FT_Stream = *mut FT_StreamRec_;
pub type FT_Stream_IoFunc = Option::<
    unsafe extern "C" fn(
        FT_Stream,
        libc::c_ulong,
        *mut libc::c_uchar,
        libc::c_ulong,
    ) -> libc::c_ulong,
>;
pub type FT_StreamDesc = FT_StreamDesc_;
#[derive(Copy, Clone)]
#[repr(C)]
pub union FT_StreamDesc_ {
    pub value: libc::c_long,
    pub pointer: *mut libc::c_void,
}
pub type FT_Pos = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_Vector_ {
    pub x: FT_Pos,
    pub y: FT_Pos,
}
pub type FT_Vector = FT_Vector_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_BBox_ {
    pub xMin: FT_Pos,
    pub yMin: FT_Pos,
    pub xMax: FT_Pos,
    pub yMax: FT_Pos,
}
pub type FT_BBox = FT_BBox_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_Bitmap_ {
    pub rows: libc::c_uint,
    pub width: libc::c_uint,
    pub pitch: libc::c_int,
    pub buffer: *mut libc::c_uchar,
    pub num_grays: libc::c_ushort,
    pub pixel_mode: libc::c_uchar,
    pub palette_mode: libc::c_uchar,
    pub palette: *mut libc::c_void,
}
pub type FT_Bitmap = FT_Bitmap_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_Outline_ {
    pub n_contours: libc::c_short,
    pub n_points: libc::c_short,
    pub points: *mut FT_Vector,
    pub tags: *mut libc::c_char,
    pub contours: *mut libc::c_short,
    pub flags: libc::c_int,
}
pub type FT_Outline = FT_Outline_;
pub type FT_Glyph_Format_ = libc::c_uint;
pub const FT_GLYPH_FORMAT_SVG: FT_Glyph_Format_ = 1398163232;
pub const FT_GLYPH_FORMAT_PLOTTER: FT_Glyph_Format_ = 1886154612;
pub const FT_GLYPH_FORMAT_OUTLINE: FT_Glyph_Format_ = 1869968492;
pub const FT_GLYPH_FORMAT_BITMAP: FT_Glyph_Format_ = 1651078259;
pub const FT_GLYPH_FORMAT_COMPOSITE: FT_Glyph_Format_ = 1668246896;
pub const FT_GLYPH_FORMAT_NONE: FT_Glyph_Format_ = 0;
pub type FT_Glyph_Format = FT_Glyph_Format_;
pub type FT_String = libc::c_char;
pub type FT_Short = libc::c_short;
pub type FT_UShort = libc::c_ushort;
pub type FT_Int = libc::c_int;
pub type FT_UInt = libc::c_uint;
pub type FT_Long = libc::c_long;
pub type FT_Fixed = libc::c_long;
pub type FT_Error = libc::c_int;
pub type FT_Pointer = *mut libc::c_void;
pub type FT_Generic_Finalizer = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_Generic_ {
    pub data: *mut libc::c_void,
    pub finalizer: FT_Generic_Finalizer,
}
pub type FT_Generic = FT_Generic_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_ListNodeRec_ {
    pub prev: FT_ListNode,
    pub next: FT_ListNode,
    pub data: *mut libc::c_void,
}
pub type FT_ListNode = *mut FT_ListNodeRec_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_ListRec_ {
    pub head: FT_ListNode,
    pub tail: FT_ListNode,
}
pub type FT_ListRec = FT_ListRec_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_Glyph_Metrics_ {
    pub width: FT_Pos,
    pub height: FT_Pos,
    pub horiBearingX: FT_Pos,
    pub horiBearingY: FT_Pos,
    pub horiAdvance: FT_Pos,
    pub vertBearingX: FT_Pos,
    pub vertBearingY: FT_Pos,
    pub vertAdvance: FT_Pos,
}
pub type FT_Glyph_Metrics = FT_Glyph_Metrics_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_Bitmap_Size_ {
    pub height: FT_Short,
    pub width: FT_Short,
    pub size: FT_Pos,
    pub x_ppem: FT_Pos,
    pub y_ppem: FT_Pos,
}
pub type FT_Bitmap_Size = FT_Bitmap_Size_;
pub type FT_Library = *mut FT_LibraryRec_;
pub type FT_Driver = *mut FT_DriverRec_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_FaceRec_ {
    pub num_faces: FT_Long,
    pub face_index: FT_Long,
    pub face_flags: FT_Long,
    pub style_flags: FT_Long,
    pub num_glyphs: FT_Long,
    pub family_name: *mut FT_String,
    pub style_name: *mut FT_String,
    pub num_fixed_sizes: FT_Int,
    pub available_sizes: *mut FT_Bitmap_Size,
    pub num_charmaps: FT_Int,
    pub charmaps: *mut FT_CharMap,
    pub generic: FT_Generic,
    pub bbox: FT_BBox,
    pub units_per_EM: FT_UShort,
    pub ascender: FT_Short,
    pub descender: FT_Short,
    pub height: FT_Short,
    pub max_advance_width: FT_Short,
    pub max_advance_height: FT_Short,
    pub underline_position: FT_Short,
    pub underline_thickness: FT_Short,
    pub glyph: FT_GlyphSlot,
    pub size: FT_Size,
    pub charmap: FT_CharMap,
    pub driver: FT_Driver,
    pub memory: FT_Memory,
    pub stream: FT_Stream,
    pub sizes_list: FT_ListRec,
    pub autohint: FT_Generic,
    pub extensions: *mut libc::c_void,
    pub internal: FT_Face_Internal,
}
pub type FT_Face_Internal = *mut FT_Face_InternalRec_;
pub type FT_CharMap = *mut FT_CharMapRec_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_CharMapRec_ {
    pub face: FT_Face,
    pub encoding: FT_Encoding,
    pub platform_id: FT_UShort,
    pub encoding_id: FT_UShort,
}
pub type FT_Encoding = FT_Encoding_;
pub type FT_Encoding_ = libc::c_uint;
pub const FT_ENCODING_APPLE_ROMAN: FT_Encoding_ = 1634889070;
pub const FT_ENCODING_OLD_LATIN_2: FT_Encoding_ = 1818326066;
pub const FT_ENCODING_ADOBE_LATIN_1: FT_Encoding_ = 1818326065;
pub const FT_ENCODING_ADOBE_CUSTOM: FT_Encoding_ = 1094992451;
pub const FT_ENCODING_ADOBE_EXPERT: FT_Encoding_ = 1094992453;
pub const FT_ENCODING_ADOBE_STANDARD: FT_Encoding_ = 1094995778;
pub const FT_ENCODING_MS_JOHAB: FT_Encoding_ = 1785686113;
pub const FT_ENCODING_MS_WANSUNG: FT_Encoding_ = 2002873971;
pub const FT_ENCODING_MS_BIG5: FT_Encoding_ = 1651074869;
pub const FT_ENCODING_MS_GB2312: FT_Encoding_ = 1734484000;
pub const FT_ENCODING_MS_SJIS: FT_Encoding_ = 1936353651;
pub const FT_ENCODING_GB2312: FT_Encoding_ = 1734484000;
pub const FT_ENCODING_JOHAB: FT_Encoding_ = 1785686113;
pub const FT_ENCODING_WANSUNG: FT_Encoding_ = 2002873971;
pub const FT_ENCODING_BIG5: FT_Encoding_ = 1651074869;
pub const FT_ENCODING_PRC: FT_Encoding_ = 1734484000;
pub const FT_ENCODING_SJIS: FT_Encoding_ = 1936353651;
pub const FT_ENCODING_UNICODE: FT_Encoding_ = 1970170211;
pub const FT_ENCODING_MS_SYMBOL: FT_Encoding_ = 1937337698;
pub const FT_ENCODING_NONE: FT_Encoding_ = 0;
pub type FT_Face = *mut FT_FaceRec_;
pub type FT_Size = *mut FT_SizeRec_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_SizeRec_ {
    pub face: FT_Face,
    pub generic: FT_Generic,
    pub metrics: FT_Size_Metrics,
    pub internal: FT_Size_Internal,
}
pub type FT_Size_Internal = *mut FT_Size_InternalRec_;
pub type FT_Size_Metrics = FT_Size_Metrics_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_Size_Metrics_ {
    pub x_ppem: FT_UShort,
    pub y_ppem: FT_UShort,
    pub x_scale: FT_Fixed,
    pub y_scale: FT_Fixed,
    pub ascender: FT_Pos,
    pub descender: FT_Pos,
    pub height: FT_Pos,
    pub max_advance: FT_Pos,
}
pub type FT_GlyphSlot = *mut FT_GlyphSlotRec_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FT_GlyphSlotRec_ {
    pub library: FT_Library,
    pub face: FT_Face,
    pub next: FT_GlyphSlot,
    pub glyph_index: FT_UInt,
    pub generic: FT_Generic,
    pub metrics: FT_Glyph_Metrics,
    pub linearHoriAdvance: FT_Fixed,
    pub linearVertAdvance: FT_Fixed,
    pub advance: FT_Vector,
    pub format: FT_Glyph_Format,
    pub bitmap: FT_Bitmap,
    pub bitmap_left: FT_Int,
    pub bitmap_top: FT_Int,
    pub outline: FT_Outline,
    pub num_subglyphs: FT_UInt,
    pub subglyphs: FT_SubGlyph,
    pub control_data: *mut libc::c_void,
    pub control_len: libc::c_long,
    pub lsb_delta: FT_Pos,
    pub rsb_delta: FT_Pos,
    pub other: *mut libc::c_void,
    pub internal: FT_Slot_Internal,
}
pub type FT_Slot_Internal = *mut FT_Slot_InternalRec_;
pub type FT_SubGlyph = *mut FT_SubGlyphRec_;
pub type FT_FaceRec = FT_FaceRec_;
pub type FcPattern = _FcPattern;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _PangoFcFont {
    pub parent_instance: PangoFont,
    pub font_pattern: *mut FcPattern,
    pub fontmap: *mut PangoFontMap,
    pub priv_0: gpointer,
    pub matrix: PangoMatrix,
    pub description: *mut PangoFontDescription,
    pub metrics_by_lang: *mut GSList,
    #[bitfield(name = "is_hinted", ty = "guint", bits = "0..=0")]
    #[bitfield(name = "is_transformed", ty = "guint", bits = "1..=1")]
    pub is_hinted_is_transformed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type PangoFcFont = _PangoFcFont;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct decos {
    pub n: libc::c_char,
    pub tm: [C2RustUnnamed; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub t: libc::c_uchar,
    pub m: libc::c_schar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct note {
    pub len: libc::c_int,
    pub pit: libc::c_schar,
    pub acc: libc::c_uchar,
    pub sl1: libc::c_uchar,
    pub sl2: libc::c_char,
    pub ti1: libc::c_char,
    pub hlen: libc::c_char,
    pub invisible: libc::c_char,
    pub shhd: libc::c_float,
    pub shac: libc::c_float,
    pub head: *mut libc::c_char,
    pub color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct notes {
    pub notes: [note; 8],
    pub slur_st: libc::c_uchar,
    pub slur_end: libc::c_char,
    pub brhythm: libc::c_schar,
    pub microscale: libc::c_uchar,
    pub sdx: libc::c_float,
    pub dc: decos,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FONTSPEC {
    pub fnum: libc::c_int,
    pub size: libc::c_float,
    pub swfac: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lyl {
    pub f: *mut FONTSPEC,
    pub w: libc::c_float,
    pub s: libc::c_float,
    pub t: [libc::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lyrics {
    pub lyl: [*mut lyl; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gch {
    pub type_0: libc::c_char,
    pub idx: libc::c_uchar,
    pub font: libc::c_uchar,
    pub box_0: libc::c_char,
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub w: libc::c_float,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct posit_s {
    #[bitfield(name = "dyn_0", ty = "libc::c_ushort", bits = "0..=3")]
    #[bitfield(name = "gch", ty = "libc::c_ushort", bits = "4..=7")]
    #[bitfield(name = "orn", ty = "libc::c_ushort", bits = "8..=11")]
    #[bitfield(name = "voc", ty = "libc::c_ushort", bits = "12..=15")]
    #[bitfield(name = "vol", ty = "libc::c_ushort", bits = "16..=19")]
    #[bitfield(name = "std", ty = "libc::c_ushort", bits = "20..=23")]
    #[bitfield(name = "gsd", ty = "libc::c_ushort", bits = "24..=27")]
    pub dyn_0_gch_orn_voc_vol_std_gsd: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SYMBOL {
    pub abc_next: *mut SYMBOL,
    pub abc_prev: *mut SYMBOL,
    pub next: *mut SYMBOL,
    pub prev: *mut SYMBOL,
    pub ts_next: *mut SYMBOL,
    pub ts_prev: *mut SYMBOL,
    pub extra: *mut SYMBOL,
    pub abc_type: libc::c_char,
    pub type_0: libc::c_uchar,
    pub voice: libc::c_uchar,
    pub staff: libc::c_uchar,
    pub nhd: libc::c_uchar,
    pub pits: [libc::c_schar; 8],
    pub dur: libc::c_int,
    pub time: libc::c_int,
    pub sflags: libc::c_uint,
    pub posit: posit_s,
    pub stem: libc::c_schar,
    pub combine: libc::c_schar,
    pub nflags: libc::c_schar,
    pub dots: libc::c_char,
    pub head: libc::c_uchar,
    pub multi: libc::c_schar,
    pub nohdi1: libc::c_schar,
    pub nohdi2: libc::c_schar,
    pub doty: libc::c_schar,
    pub aux: libc::c_short,
    pub color: libc::c_int,
    pub x: libc::c_float,
    pub y: libc::c_schar,
    pub ymn: libc::c_schar,
    pub ymx: libc::c_schar,
    pub mid: libc::c_schar,
    pub xmx: libc::c_float,
    pub xs: libc::c_float,
    pub ys: libc::c_float,
    pub wl: libc::c_float,
    pub wr: libc::c_float,
    pub space: libc::c_float,
    pub shrink: libc::c_float,
    pub xmax: libc::c_float,
    pub gch: *mut gch,
    pub ly: *mut lyrics,
    pub state: libc::c_char,
    pub flags: libc::c_ushort,
    pub colnum: libc::c_ushort,
    pub linenum: libc::c_int,
    pub fn_0: *mut libc::c_char,
    pub text: *mut libc::c_char,
    pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub key: key_s,
    pub length: C2RustUnnamed_9,
    pub meter: meter_s,
    pub tempo: C2RustUnnamed_7,
    pub voice: C2RustUnnamed_6,
    pub bar: C2RustUnnamed_5,
    pub clef: clef_s,
    pub note: notes,
    pub user: C2RustUnnamed_4,
    pub eoln: C2RustUnnamed_3,
    pub v_over: C2RustUnnamed_2,
    pub tuplet: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub p_plet: libc::c_char,
    pub q_plet: libc::c_char,
    pub r_plet: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub type_0: libc::c_char,
    pub voice: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub type_0: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub symbol: libc::c_uchar,
    pub value: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clef_s {
    pub name: *mut libc::c_char,
    pub type_0: libc::c_schar,
    pub line: libc::c_char,
    pub octave: libc::c_schar,
    pub transpose: libc::c_schar,
    pub invis: libc::c_char,
    pub check_pitch: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub type_0: libc::c_int,
    pub repeat_bar: libc::c_char,
    pub len: libc::c_char,
    pub dotted: libc::c_char,
    pub dc: decos,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub id: [libc::c_char; 16],
    pub fname: *mut libc::c_char,
    pub nname: *mut libc::c_char,
    pub scale: libc::c_float,
    pub voice: libc::c_uchar,
    pub octave: libc::c_schar,
    pub merge: libc::c_char,
    pub stem: libc::c_schar,
    pub gstem: libc::c_schar,
    pub dyn_0: libc::c_schar,
    pub lyrics: libc::c_schar,
    pub gchord: libc::c_schar,
    pub cue: libc::c_schar,
    pub stafflines: *mut libc::c_char,
    pub staffscale: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub str1: *mut libc::c_char,
    pub beats: [libc::c_short; 4],
    pub circa: libc::c_short,
    pub tempo: libc::c_short,
    pub new_beat: libc::c_short,
    pub str2: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct meter_s {
    pub wmeasure: libc::c_short,
    pub nmeter: libc::c_uchar,
    pub expdur: libc::c_char,
    pub meter: [C2RustUnnamed_8; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub top: [libc::c_char; 8],
    pub bot: [libc::c_char; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub base_length: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct key_s {
    pub sf: libc::c_schar,
    pub empty: libc::c_char,
    pub exp: libc::c_char,
    pub instr: libc::c_char,
    pub nacc: libc::c_schar,
    pub cue: libc::c_schar,
    pub octave: libc::c_schar,
    pub microscale: libc::c_uchar,
    pub clef_delta: libc::c_char,
    pub key_delta: libc::c_char,
    pub stafflines: *mut libc::c_char,
    pub staffscale: libc::c_float,
    pub pits: [libc::c_schar; 8],
    pub accs: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parse {
    pub first_sym: *mut SYMBOL,
    pub last_sym: *mut SYMBOL,
    pub abc_vers: libc::c_int,
    pub deco_tb: [*mut libc::c_char; 128],
    pub micro_tb: [libc::c_ushort; 32],
    pub abc_state: libc::c_int,
}
pub type e_fonts = libc::c_uint;
pub const FONT_DYN: e_fonts = 27;
pub const WORDSFONT: e_fonts = 26;
pub const VOICEFONT: e_fonts = 25;
pub const VOCALFONT: e_fonts = 24;
pub const TITLEFONT: e_fonts = 23;
pub const TEXTFONT: e_fonts = 22;
pub const TEMPOFONT: e_fonts = 21;
pub const SUBTITLEFONT: e_fonts = 20;
pub const REPEATFONT: e_fonts = 19;
pub const PARTSFONT: e_fonts = 18;
pub const MEASUREFONT: e_fonts = 17;
pub const INFOFONT: e_fonts = 16;
pub const HISTORYFONT: e_fonts = 15;
pub const HEADERFONT: e_fonts = 14;
pub const GCHORDFONT: e_fonts = 13;
pub const FOOTERFONT: e_fonts = 12;
pub const COMPOSERFONT: e_fonts = 11;
pub const ANNOTATIONFONT: e_fonts = 10;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FORMAT {
    pub pageheight: libc::c_float,
    pub pagewidth: libc::c_float,
    pub topmargin: libc::c_float,
    pub botmargin: libc::c_float,
    pub leftmargin: libc::c_float,
    pub rightmargin: libc::c_float,
    pub topspace: libc::c_float,
    pub wordsspace: libc::c_float,
    pub titlespace: libc::c_float,
    pub subtitlespace: libc::c_float,
    pub partsspace: libc::c_float,
    pub composerspace: libc::c_float,
    pub musicspace: libc::c_float,
    pub vocalspace: libc::c_float,
    pub textspace: libc::c_float,
    pub breaklimit: libc::c_float,
    pub maxshrink: libc::c_float,
    pub lineskipfac: libc::c_float,
    pub parskipfac: libc::c_float,
    pub stemheight: libc::c_float,
    pub gutter: libc::c_float,
    pub indent: libc::c_float,
    pub infospace: libc::c_float,
    pub slurheight: libc::c_float,
    pub tieheight: libc::c_float,
    pub notespacingfactor: libc::c_float,
    pub scale: libc::c_float,
    pub staffsep: libc::c_float,
    pub sysstaffsep: libc::c_float,
    pub maxstaffsep: libc::c_float,
    pub maxsysstaffsep: libc::c_float,
    pub stretchlast: libc::c_float,
    pub staffscale: libc::c_float,
    pub abc2pscompat: libc::c_int,
    pub alignbars: libc::c_int,
    pub aligncomposer: libc::c_int,
    pub autoclef: libc::c_int,
    pub barsperstaff: libc::c_int,
    pub breakoneoln: libc::c_int,
    pub bstemdown: libc::c_int,
    pub cancelkey: libc::c_int,
    pub capo: libc::c_int,
    pub combinevoices: libc::c_int,
    pub contbarnb: libc::c_int,
    pub continueall: libc::c_int,
    pub custos: libc::c_int,
    pub dblrepbar: libc::c_int,
    pub decoerr: libc::c_int,
    pub dynalign: libc::c_int,
    pub flatbeams: libc::c_int,
    pub flatbeamgracing: libc::c_int,
    pub infoline: libc::c_int,
    pub gchordbox: libc::c_int,
    pub graceslurs: libc::c_int,
    pub graceword: libc::c_int,
    pub gracespace: libc::c_int,
    pub hyphencont: libc::c_int,
    pub keywarn: libc::c_int,
    pub landscape: libc::c_int,
    pub linewarn: libc::c_int,
    pub measurebox: libc::c_int,
    pub measurefirst: libc::c_int,
    pub measurenb: libc::c_int,
    pub nedo: libc::c_int,
    pub oneperpage: libc::c_int,
    pub pango: libc::c_int,
    pub partsbox: libc::c_int,
    pub pdfmark: libc::c_int,
    pub rbdbstop: libc::c_int,
    pub rbmax: libc::c_int,
    pub rbmin: libc::c_int,
    pub setdefl: libc::c_int,
    pub shiftunison: libc::c_int,
    pub splittune: libc::c_int,
    pub squarebreve: libc::c_int,
    pub staffnonote: libc::c_int,
    pub straightflags: libc::c_int,
    pub stretchstaff: libc::c_int,
    pub textoption: libc::c_int,
    pub titlecaps: libc::c_int,
    pub titleleft: libc::c_int,
    pub titletrim: libc::c_int,
    pub timewarn: libc::c_int,
    pub transpose: libc::c_int,
    pub tuplets: libc::c_int,
    pub bgcolor: *mut libc::c_char,
    pub dateformat: *mut libc::c_char,
    pub header: *mut libc::c_char,
    pub footer: *mut libc::c_char,
    pub header2: *mut libc::c_char,
    pub footer2: *mut libc::c_char,
    pub titleformat: *mut libc::c_char,
    pub musicfont: *mut libc::c_char,
    pub font_tb: [FONTSPEC; 39],
    pub ndfont: libc::c_char,
    pub gcf: libc::c_uchar,
    pub anf: libc::c_uchar,
    pub vof: libc::c_uchar,
    pub fields: [libc::c_uint; 2],
    pub posit: posit_s,
}
pub type INFO = [*mut SYMBOL; 26];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tblt_s {
    pub head: *mut libc::c_char,
    pub note: *mut libc::c_char,
    pub bar: *mut libc::c_char,
    pub wh: libc::c_float,
    pub ha: libc::c_float,
    pub hu: libc::c_float,
    pub pitch: libc::c_short,
    pub instr: [libc::c_char; 2],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct VOICE_S {
    pub id: [libc::c_char; 16],
    pub next: *mut VOICE_S,
    pub sym: *mut SYMBOL,
    pub last_sym: *mut SYMBOL,
    pub lyric_start: *mut SYMBOL,
    pub nm: *mut libc::c_char,
    pub snm: *mut libc::c_char,
    pub bar_text: *mut libc::c_char,
    pub bar_gch: *mut gch,
    pub tie: *mut SYMBOL,
    pub rtie: *mut SYMBOL,
    pub tblts: [*mut tblt_s; 2],
    pub scale: libc::c_float,
    pub time: libc::c_int,
    pub s_clef: *mut SYMBOL,
    pub key: key_s,
    pub meter: meter_s,
    pub ckey: key_s,
    pub okey: key_s,
    pub hy_st: libc::c_uint,
    #[bitfield(name = "ignore", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "second", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "floating", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "bar_repeat", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "norepbra", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "have_ly", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "new_name", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "space", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "perc", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "auto_len", ty = "libc::c_uint", bits = "9..=9")]
    pub ignore_second_floating_bar_repeat_norepbra_have_ly_new_name_space_perc_auto_len: [u8; 2],
    pub wmeasure: libc::c_short,
    pub transpose: libc::c_short,
    pub bar_start: libc::c_short,
    pub posit: posit_s,
    pub octave: libc::c_schar,
    pub ottava: libc::c_schar,
    pub clone: libc::c_schar,
    pub over: libc::c_schar,
    pub staff: libc::c_uchar,
    pub cstaff: libc::c_uchar,
    pub slur_st: libc::c_uchar,
    pub combine: libc::c_schar,
    pub color: libc::c_int,
    pub stafflines: *mut libc::c_char,
    pub staffscale: libc::c_float,
    pub last_note: *mut SYMBOL,
    pub map_name: *mut libc::c_char,
    pub ulen: libc::c_int,
    pub microscale: libc::c_uchar,
    pub mvoice: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct u_ps {
    pub next: *mut u_ps,
    pub text: [libc::c_char; 100000],
}
#[inline]
unsafe extern "C" fn isascii(mut _c: libc::c_int) -> libc::c_int {
    return (_c & !(0x7f as libc::c_int) == 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn __istype(
    mut _c: __darwin_ct_rune_t,
    mut _f: libc::c_ulong,
) -> libc::c_int {
    return if isascii(_c) != 0 {
        (_DefaultRuneLocale.__runetype[_c as usize] as libc::c_ulong & _f != 0)
            as libc::c_int
    } else {
        (__maskrune(_c, _f) != 0) as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn __isctype(
    mut _c: __darwin_ct_rune_t,
    mut _f: libc::c_ulong,
) -> __darwin_ct_rune_t {
    return if _c < 0 as libc::c_int || _c >= (1 as libc::c_int) << 8 as libc::c_int {
        0 as libc::c_int
    } else {
        (_DefaultRuneLocale.__runetype[_c as usize] as libc::c_ulong & _f != 0)
            as libc::c_int
    };
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn isdigit(mut _c: libc::c_int) -> libc::c_int {
    return __isctype(_c, 0x400 as libc::c_long as libc::c_ulong);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn isspace(mut _c: libc::c_int) -> libc::c_int {
    return __istype(_c, 0x4000 as libc::c_long as libc::c_ulong);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn isupper(mut _c: libc::c_int) -> libc::c_int {
    return __istype(_c, 0x8000 as libc::c_long as libc::c_ulong);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn tolower(mut _c: libc::c_int) -> libc::c_int {
    return __tolower(_c);
}
#[inline(always)]
unsafe extern "C" fn g_string_append_len_inline(
    mut gstring: *mut GString,
    mut val: *const libc::c_char,
    mut len: gssize,
) -> *mut GString {
    let mut len_unsigned: gsize = 0;
    if ({
        let mut _g_boolean_var_4: libc::c_int = 0;
        if gstring.is_null() {
            _g_boolean_var_4 = 1 as libc::c_int;
        } else {
            _g_boolean_var_4 = 0 as libc::c_int;
        }
        _g_boolean_var_4
    }) as libc::c_long != 0
    {
        return g_string_append_len(gstring, val, len);
    }
    if ({
        let mut _g_boolean_var_5: libc::c_int = 0;
        if val.is_null() {
            _g_boolean_var_5 = 1 as libc::c_int;
        } else {
            _g_boolean_var_5 = 0 as libc::c_int;
        }
        _g_boolean_var_5
    }) as libc::c_long != 0
    {
        return if len != 0 as libc::c_int as gssize {
            g_string_append_len(gstring, val, len)
        } else {
            gstring
        };
    }
    if len < 0 as libc::c_int as gssize {
        len_unsigned = strlen(val);
    } else {
        len_unsigned = len as gsize;
    }
    if ({
        let mut _g_boolean_var_6: libc::c_int = 0;
        if ((*gstring).len).wrapping_add(len_unsigned) < (*gstring).allocated_len {
            _g_boolean_var_6 = 1 as libc::c_int;
        } else {
            _g_boolean_var_6 = 0 as libc::c_int;
        }
        _g_boolean_var_6
    }) as libc::c_long != 0
    {
        let mut end: *mut libc::c_char = ((*gstring).str_0)
            .offset((*gstring).len as isize);
        if ({
            let mut _g_boolean_var_7: libc::c_int = 0;
            if val.offset(len_unsigned as isize) <= end as *const libc::c_char
                || val > end.offset(len_unsigned as isize) as *const libc::c_char
            {
                _g_boolean_var_7 = 1 as libc::c_int;
            } else {
                _g_boolean_var_7 = 0 as libc::c_int;
            }
            _g_boolean_var_7
        }) as libc::c_long != 0
        {
            memcpy(end as *mut libc::c_void, val as *const libc::c_void, len_unsigned);
        } else {
            memmove(end as *mut libc::c_void, val as *const libc::c_void, len_unsigned);
        }
        (*gstring).len = ((*gstring).len).wrapping_add(len_unsigned);
        *((*gstring).str_0).offset((*gstring).len as isize) = 0 as libc::c_int as gchar;
        return gstring;
    } else {
        return g_string_insert_len(gstring, -(1 as libc::c_int) as gssize, val, len)
    };
}
#[inline(always)]
unsafe extern "C" fn g_string_truncate_inline(
    mut gstring: *mut GString,
    mut len: gsize,
) -> *mut GString {
    (*gstring).len = if len < (*gstring).len { len } else { (*gstring).len };
    *((*gstring).str_0).offset((*gstring).len as isize) = '\0' as i32 as gchar;
    return gstring;
}
#[no_mangle]
pub static mut tex_buf: [libc::c_char; 512] = [0; 512];
#[no_mangle]
pub static mut outft: libc::c_int = -(1 as libc::c_int);
static mut stropx: libc::c_int = 0;
static mut strlw: libc::c_float = 0.;
static mut curft: libc::c_int = 0;
static mut defft: libc::c_int = 0;
static mut strtx: libc::c_char = 0;
static mut cw_tb: [libc::c_short; 128] = [
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    250 as libc::c_int as libc::c_short,
    333 as libc::c_int as libc::c_short,
    408 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    833 as libc::c_int as libc::c_short,
    778 as libc::c_int as libc::c_short,
    333 as libc::c_int as libc::c_short,
    333 as libc::c_int as libc::c_short,
    333 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    564 as libc::c_int as libc::c_short,
    250 as libc::c_int as libc::c_short,
    564 as libc::c_int as libc::c_short,
    250 as libc::c_int as libc::c_short,
    278 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    278 as libc::c_int as libc::c_short,
    278 as libc::c_int as libc::c_short,
    564 as libc::c_int as libc::c_short,
    564 as libc::c_int as libc::c_short,
    564 as libc::c_int as libc::c_short,
    444 as libc::c_int as libc::c_short,
    921 as libc::c_int as libc::c_short,
    722 as libc::c_int as libc::c_short,
    667 as libc::c_int as libc::c_short,
    667 as libc::c_int as libc::c_short,
    722 as libc::c_int as libc::c_short,
    611 as libc::c_int as libc::c_short,
    556 as libc::c_int as libc::c_short,
    722 as libc::c_int as libc::c_short,
    722 as libc::c_int as libc::c_short,
    333 as libc::c_int as libc::c_short,
    389 as libc::c_int as libc::c_short,
    722 as libc::c_int as libc::c_short,
    611 as libc::c_int as libc::c_short,
    889 as libc::c_int as libc::c_short,
    722 as libc::c_int as libc::c_short,
    722 as libc::c_int as libc::c_short,
    556 as libc::c_int as libc::c_short,
    722 as libc::c_int as libc::c_short,
    667 as libc::c_int as libc::c_short,
    556 as libc::c_int as libc::c_short,
    611 as libc::c_int as libc::c_short,
    722 as libc::c_int as libc::c_short,
    722 as libc::c_int as libc::c_short,
    944 as libc::c_int as libc::c_short,
    722 as libc::c_int as libc::c_short,
    722 as libc::c_int as libc::c_short,
    611 as libc::c_int as libc::c_short,
    333 as libc::c_int as libc::c_short,
    278 as libc::c_int as libc::c_short,
    333 as libc::c_int as libc::c_short,
    469 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    333 as libc::c_int as libc::c_short,
    444 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    444 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    444 as libc::c_int as libc::c_short,
    333 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    278 as libc::c_int as libc::c_short,
    278 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    278 as libc::c_int as libc::c_short,
    778 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    333 as libc::c_int as libc::c_short,
    389 as libc::c_int as libc::c_short,
    278 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    722 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
    444 as libc::c_int as libc::c_short,
    480 as libc::c_int as libc::c_short,
    200 as libc::c_int as libc::c_short,
    480 as libc::c_int as libc::c_short,
    541 as libc::c_int as libc::c_short,
    500 as libc::c_int as libc::c_short,
];
static mut user_ps: *mut u_ps = 0 as *const u_ps as *mut u_ps;
#[no_mangle]
pub unsafe extern "C" fn bug(mut msg: *mut libc::c_char, mut fatal: libc::c_int) {
    error(
        1 as libc::c_int,
        0 as *mut SYMBOL,
        b"Internal error: %s.\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        msg,
    );
    if fatal != 0 {
        fprintf(__stderrp, b"Emergency stop.\n\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    fprintf(__stderrp, b"Trying to continue...\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn error(
    mut sev: libc::c_int,
    mut s: *mut SYMBOL,
    mut fmt: *mut libc::c_char,
    mut args: ...
) {
    let mut args_0: va_list = 0 as *mut libc::c_char;
    if !s.is_null() {
        if !((*s).fn_0).is_null() {
            fprintf(
                __stderrp,
                b"%s:%d:%d: \0" as *const u8 as *const libc::c_char,
                (*s).fn_0,
                (*s).linenum,
                (*s).colnum as libc::c_int,
            );
        }
        (*s).flags = ((*s).flags as libc::c_int | 0x1 as libc::c_int) as libc::c_ushort;
    }
    fprintf(
        __stderrp,
        if sev == 0 as libc::c_int {
            b"warning: \0" as *const u8 as *const libc::c_char
        } else {
            b"error: \0" as *const u8 as *const libc::c_char
        },
    );
    va_start(args_0, fmt);
    vfprintf(__stderrp, fmt, args_0);
    va_end(args_0);
    fprintf(__stderrp, b"\n\0" as *const u8 as *const libc::c_char);
    if sev > severity {
        severity = sev;
    }
}
unsafe extern "C" fn cap_str(mut p: *mut libc::c_char) {
    while *p as libc::c_int != '\0' as i32 {
        let mut c: libc::c_uchar = 0;
        c = *p as libc::c_uchar;
        if c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'z' as i32 {
            *p = (c as libc::c_int & !(0x20 as libc::c_int)) as libc::c_char;
        } else if c as libc::c_int == 0xc3 as libc::c_int {
            p = p.offset(1);
            p;
            c = *p as libc::c_uchar;
            if c as libc::c_int >= 0xa0 as libc::c_int
                && c as libc::c_int <= 0xbe as libc::c_int
                && c as libc::c_int != 0xb7 as libc::c_int
            {
                *p = (c as libc::c_int & !(0x20 as libc::c_int)) as libc::c_char;
            }
        } else if c as libc::c_int == 0xc4 as libc::c_int {
            p = p.offset(1);
            p;
            c = *p as libc::c_uchar;
            if c as libc::c_int >= 0x81 as libc::c_int
                && c as libc::c_int <= 0xb7 as libc::c_int
                && c as libc::c_int & 0x1 as libc::c_int != 0
            {
                *p -= 1;
                *p;
            }
        }
        p = p.offset(1);
        p;
    }
}
#[no_mangle]
pub unsafe extern "C" fn cwid(mut c: libc::c_uchar) -> libc::c_float {
    if c as libc::c_int >= 0x80 as libc::c_int {
        if (c as libc::c_int) < 0xc0 as libc::c_int {
            return 0 as libc::c_int as libc::c_float;
        }
        c = 'a' as i32 as libc::c_uchar;
    }
    return (cw_tb[c as usize] as libc::c_float as libc::c_double / 1000.0f64)
        as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn tex_str(mut s: *mut libc::c_char) -> libc::c_float {
    let mut d: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c1: libc::c_uchar = 0;
    let mut maxlen: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    let mut w: libc::c_float = 0.;
    let mut swfac: libc::c_float = 0.;
    w = 0 as libc::c_int as libc::c_float;
    d = tex_buf.as_mut_ptr();
    maxlen = (::core::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uint;
    i = curft as libc::c_uint;
    if i <= 0 as libc::c_int as libc::c_uint {
        i = defft as libc::c_uint;
    }
    swfac = cfmt.font_tb[i as usize].swfac;
    let mut current_block_95: u64;
    loop {
        let fresh0 = s;
        s = s.offset(1);
        c1 = *fresh0 as libc::c_uchar;
        if c1 as libc::c_int == '\0' as i32 {
            break;
        }
        match c1 as libc::c_int {
            92 => {
                let fresh1 = s;
                s = s.offset(1);
                c1 = *fresh1 as libc::c_uchar;
                if c1 as libc::c_int == '\0' as i32 {
                    *d = '\0' as i32 as libc::c_char;
                    return w;
                }
                match c1 as libc::c_int {
                    110 => {
                        c1 = '\n' as i32 as libc::c_uchar;
                    }
                    116 => {
                        c1 = '\t' as i32 as libc::c_uchar;
                    }
                    _ => {}
                }
                current_block_95 = 10512632378975961025;
            }
            36 => {
                if isdigit(*s as libc::c_uchar as libc::c_int) != 0
                    && ((*s as libc::c_int - '0' as i32) as libc::c_uint)
                        < 10 as libc::c_int as libc::c_uint
                {
                    i = (*s as libc::c_int - '0' as i32) as libc::c_uint;
                    if i == 0 as libc::c_int as libc::c_uint {
                        i = defft as libc::c_uint;
                    }
                    swfac = cfmt.font_tb[i as usize].swfac;
                    maxlen = maxlen.wrapping_sub(1);
                    if maxlen <= 0 as libc::c_int as libc::c_uint {
                        current_block_95 = 10512632378975961025;
                    } else {
                        let fresh2 = d;
                        d = d.offset(1);
                        *fresh2 = c1 as libc::c_char;
                        let fresh3 = s;
                        s = s.offset(1);
                        c1 = *fresh3 as libc::c_uchar;
                        current_block_95 = 12037389606976705528;
                    }
                } else if *s as libc::c_int == '$' as i32 {
                    maxlen = maxlen.wrapping_sub(1);
                    if maxlen <= 0 as libc::c_int as libc::c_uint {
                        current_block_95 = 10512632378975961025;
                    } else {
                        let fresh4 = d;
                        d = d.offset(1);
                        *fresh4 = c1 as libc::c_char;
                        s = s.offset(1);
                        s;
                        current_block_95 = 10512632378975961025;
                    }
                } else {
                    current_block_95 = 10512632378975961025;
                }
            }
            38 => {
                if svg != 0 || epsf > 1 as libc::c_int {
                    p = strchr(s, ';' as i32);
                    if !(p.is_null()
                        || p.offset_from(s) as libc::c_long
                            >= 10 as libc::c_int as libc::c_long)
                    {
                        let fresh5 = d;
                        d = d.offset(1);
                        *fresh5 = c1 as libc::c_char;
                        while s <= p {
                            let fresh6 = s;
                            s = s.offset(1);
                            let fresh7 = d;
                            d = d.offset(1);
                            *fresh7 = *fresh6;
                        }
                        w += cwid('a' as i32 as libc::c_uchar) * swfac;
                        continue;
                    }
                } else if *s as libc::c_int == '#' as i32 {
                    let mut j: libc::c_int = 0;
                    let mut v: libc::c_long = 0;
                    if *s.offset(1 as libc::c_int as isize) as libc::c_int == 'x' as i32
                    {
                        i = sscanf(
                            s,
                            b"#x%lx;%n\0" as *const u8 as *const libc::c_char,
                            &mut v as *mut libc::c_long,
                            &mut j as *mut libc::c_int,
                        ) as libc::c_uint;
                    } else {
                        i = sscanf(
                            s,
                            b"#%ld;%n\0" as *const u8 as *const libc::c_char,
                            &mut v as *mut libc::c_long,
                            &mut j as *mut libc::c_int,
                        ) as libc::c_uint;
                    }
                    if i != 1 as libc::c_int as libc::c_uint {
                        error(
                            0 as libc::c_int,
                            0 as *mut SYMBOL,
                            b"Bad XML char reference\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                    } else {
                        if v < 0x80 as libc::c_int as libc::c_long {
                            let fresh8 = d;
                            d = d.offset(1);
                            *fresh8 = v as libc::c_char;
                        } else if v < 0x800 as libc::c_int as libc::c_long {
                            let fresh9 = d;
                            d = d.offset(1);
                            *fresh9 = (0xc0 as libc::c_int as libc::c_long
                                | v >> 6 as libc::c_int) as libc::c_char;
                            let fresh10 = d;
                            d = d.offset(1);
                            *fresh10 = (0x80 as libc::c_int as libc::c_long
                                | v & 0x3f as libc::c_int as libc::c_long) as libc::c_char;
                        } else if v < 0x10000 as libc::c_int as libc::c_long {
                            let fresh11 = d;
                            d = d.offset(1);
                            *fresh11 = (0xe0 as libc::c_int as libc::c_long
                                | v >> 12 as libc::c_int) as libc::c_char;
                            let fresh12 = d;
                            d = d.offset(1);
                            *fresh12 = (0x80 as libc::c_int as libc::c_long
                                | v >> 6 as libc::c_int
                                    & 0x3f as libc::c_int as libc::c_long) as libc::c_char;
                            let fresh13 = d;
                            d = d.offset(1);
                            *fresh13 = (0x80 as libc::c_int as libc::c_long
                                | v & 0x3f as libc::c_int as libc::c_long) as libc::c_char;
                        } else {
                            let fresh14 = d;
                            d = d.offset(1);
                            *fresh14 = (0xf0 as libc::c_int as libc::c_long
                                | v >> 18 as libc::c_int) as libc::c_char;
                            let fresh15 = d;
                            d = d.offset(1);
                            *fresh15 = (0x80 as libc::c_int as libc::c_long
                                | v >> 12 as libc::c_int
                                    & 0x3f as libc::c_int as libc::c_long) as libc::c_char;
                            let fresh16 = d;
                            d = d.offset(1);
                            *fresh16 = (0x80 as libc::c_int as libc::c_long
                                | v >> 6 as libc::c_int
                                    & 0x3f as libc::c_int as libc::c_long) as libc::c_char;
                            let fresh17 = d;
                            d = d.offset(1);
                            *fresh17 = (0x80 as libc::c_int as libc::c_long
                                | v & 0x3f as libc::c_int as libc::c_long) as libc::c_char;
                        }
                        w += cwid('a' as i32 as libc::c_uchar) * swfac;
                        s = s.offset(j as isize);
                        continue;
                    }
                } else if strncmp(
                    s,
                    b"lt;\0" as *const u8 as *const libc::c_char,
                    3 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    c1 = '<' as i32 as libc::c_uchar;
                    s = s.offset(3 as libc::c_int as isize);
                } else if strncmp(
                    s,
                    b"gt;\0" as *const u8 as *const libc::c_char,
                    3 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    c1 = '>' as i32 as libc::c_uchar;
                    s = s.offset(3 as libc::c_int as isize);
                } else if strncmp(
                    s,
                    b"amp;\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    c1 = '&' as i32 as libc::c_uchar;
                    s = s.offset(4 as libc::c_int as isize);
                } else if strncmp(
                    s,
                    b"apos;\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    c1 = '\'' as i32 as libc::c_uchar;
                    s = s.offset(5 as libc::c_int as isize);
                } else if strncmp(
                    s,
                    b"quot;\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    c1 = '"' as i32 as libc::c_uchar;
                    s = s.offset(5 as libc::c_int as isize);
                }
                current_block_95 = 10512632378975961025;
            }
            _ => {
                current_block_95 = 10512632378975961025;
            }
        }
        match current_block_95 {
            10512632378975961025 => {
                if c1 as libc::c_int >= 0x80 as libc::c_int {
                    if c1 as libc::c_int >= 0xc0 as libc::c_int {
                        w += cwid('a' as i32 as libc::c_uchar) * swfac;
                    }
                } else if c1 as libc::c_int <= 5 as libc::c_int {
                    maxlen = maxlen.wrapping_sub(1);
                    if maxlen < 4 as libc::c_int as libc::c_uint {
                        break;
                    }
                    match c1 as libc::c_int {
                        1 => {
                            let fresh18 = d;
                            d = d.offset(1);
                            *fresh18 = 0xe2 as libc::c_int as libc::c_char;
                            let fresh19 = d;
                            d = d.offset(1);
                            *fresh19 = 0x99 as libc::c_int as libc::c_char;
                            let fresh20 = d;
                            d = d.offset(1);
                            *fresh20 = 0xaf as libc::c_int as libc::c_char;
                        }
                        2 => {
                            let fresh21 = d;
                            d = d.offset(1);
                            *fresh21 = 0xe2 as libc::c_int as libc::c_char;
                            let fresh22 = d;
                            d = d.offset(1);
                            *fresh22 = 0x99 as libc::c_int as libc::c_char;
                            let fresh23 = d;
                            d = d.offset(1);
                            *fresh23 = 0xad as libc::c_int as libc::c_char;
                        }
                        3 => {
                            let fresh24 = d;
                            d = d.offset(1);
                            *fresh24 = 0xe2 as libc::c_int as libc::c_char;
                            let fresh25 = d;
                            d = d.offset(1);
                            *fresh25 = 0x99 as libc::c_int as libc::c_char;
                            let fresh26 = d;
                            d = d.offset(1);
                            *fresh26 = 0xae as libc::c_int as libc::c_char;
                        }
                        4 => {
                            let fresh27 = d;
                            d = d.offset(1);
                            *fresh27 = 0xf0 as libc::c_int as libc::c_char;
                            let fresh28 = d;
                            d = d.offset(1);
                            *fresh28 = 0x9d as libc::c_int as libc::c_char;
                            let fresh29 = d;
                            d = d.offset(1);
                            *fresh29 = 0x84 as libc::c_int as libc::c_char;
                            let fresh30 = d;
                            d = d.offset(1);
                            *fresh30 = 0xaa as libc::c_int as libc::c_char;
                        }
                        5 => {
                            let fresh31 = d;
                            d = d.offset(1);
                            *fresh31 = 0xf0 as libc::c_int as libc::c_char;
                            let fresh32 = d;
                            d = d.offset(1);
                            *fresh32 = 0x9d as libc::c_int as libc::c_char;
                            let fresh33 = d;
                            d = d.offset(1);
                            *fresh33 = 0x84 as libc::c_int as libc::c_char;
                            let fresh34 = d;
                            d = d.offset(1);
                            *fresh34 = 0xab as libc::c_int as libc::c_char;
                        }
                        _ => {}
                    }
                    w += cwid('a' as i32 as libc::c_uchar) * swfac;
                    continue;
                } else {
                    w += cwid(c1) * swfac;
                }
            }
            _ => {}
        }
        maxlen = maxlen.wrapping_sub(1);
        if maxlen <= 0 as libc::c_int as libc::c_uint {
            break;
        }
        let fresh35 = d;
        d = d.offset(1);
        *fresh35 = c1 as libc::c_char;
    }
    *d = '\0' as i32 as libc::c_char;
    if maxlen <= 0 as libc::c_int as libc::c_uint {
        error(
            0 as libc::c_int,
            0 as *mut SYMBOL,
            b"Text too large - ignored part: '%s'\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            s,
        );
    }
    return w;
}
static mut desc_tb: [*mut PangoFontDescription; 30] = [0 as *const PangoFontDescription
    as *mut PangoFontDescription; 30];
static mut layout: *mut PangoLayout = unsafe { -(1 as libc::c_int) as *mut PangoLayout };
static mut attrs: *mut PangoAttrList = 0 as *const PangoAttrList as *mut PangoAttrList;
static mut out_pg_ft: libc::c_int = -(1 as libc::c_int);
static mut pg_str: *mut GString = 0 as *const GString as *mut GString;
#[no_mangle]
pub unsafe extern "C" fn pg_init() {
    static mut context: *mut PangoContext = 0 as *const PangoContext
        as *mut PangoContext;
    context = pango_font_map_create_context(pango_cairo_font_map_get_default());
    if !context.is_null() {
        layout = pango_layout_new(context);
    }
    if layout.is_null() {
        error(
            0 as libc::c_int,
            0 as *mut SYMBOL,
            b"pango disabled\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        cfmt.pango = 0 as libc::c_int;
    } else {
        pango_layout_set_wrap(layout, PANGO_WRAP_WORD);
        pg_str = g_string_sized_new(256 as libc::c_int as gsize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn pg_reset_font() {
    out_pg_ft = -(1 as libc::c_int);
}
unsafe extern "C" fn desc_font(mut fnum: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut font_name: [libc::c_char; 128] = [0; 128];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut bad_tb: [*mut libc::c_char; 4] = [
        b"Times-Roman\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Times\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Helvetica\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Courier\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ];
    static mut good_tb: [*mut libc::c_char; 4] = [
        b"serif\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"serif\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"sans-serif\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"monospace\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ];
    if (desc_tb[fnum as usize]).is_null() {
        p = font_name.as_mut_ptr();
        q = fontnames[fnum as usize];
        i = 0 as libc::c_int;
        while (i as libc::c_ulong)
            < (::core::mem::size_of::<[*mut libc::c_char; 4]>() as libc::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                )
        {
            if strncmp(q, bad_tb[i as usize], strlen(bad_tb[i as usize]))
                == 0 as libc::c_int
            {
                p = p
                    .offset(
                        sprintf(
                            p,
                            b"%s\0" as *const u8 as *const libc::c_char,
                            good_tb[i as usize],
                        ) as isize,
                    );
                q = q.offset(strlen(bad_tb[i as usize]) as isize);
                break;
            } else {
                i += 1;
                i;
            }
        }
        snprintf(
            p,
            (::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
                .wrapping_sub(5 as libc::c_int as libc::c_ulong),
            b"%s 10\0" as *const u8 as *const libc::c_char,
            q,
        );
        while *p as libc::c_int != '\0' as i32 {
            if *p as libc::c_int == '-' as i32 {
                *p = ' ' as i32 as libc::c_char;
            }
            if isupper(*p as libc::c_int) != 0 {
                *p = tolower(*p as libc::c_int) as libc::c_char;
            }
            p = p.offset(1);
            p;
        }
        p = strstr(
            &mut *font_name.as_mut_ptr().offset(1 as libc::c_int as isize),
            b"italic\0" as *const u8 as *const libc::c_char,
        );
        if !p.is_null()
            && *p.offset(-(1 as libc::c_int) as isize) as libc::c_int != ' ' as i32
        {
            memmove(
                p.offset(1 as libc::c_int as isize) as *mut libc::c_void,
                p as *const libc::c_void,
                (strlen(p)).wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            *p = ' ' as i32 as libc::c_char;
        }
        p = strstr(
            &mut *font_name.as_mut_ptr().offset(1 as libc::c_int as isize),
            b"oblique\0" as *const u8 as *const libc::c_char,
        );
        if !p.is_null()
            && *p.offset(-(1 as libc::c_int) as isize) as libc::c_int != ' ' as i32
        {
            memmove(
                p.offset(1 as libc::c_int as isize) as *mut libc::c_void,
                p as *const libc::c_void,
                (strlen(p)).wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            *p = ' ' as i32 as libc::c_char;
        }
        p = strstr(
            &mut *font_name.as_mut_ptr().offset(1 as libc::c_int as isize),
            b"bold\0" as *const u8 as *const libc::c_char,
        );
        if !p.is_null()
            && *p.offset(-(1 as libc::c_int) as isize) as libc::c_int != ' ' as i32
        {
            memmove(
                p.offset(1 as libc::c_int as isize) as *mut libc::c_void,
                p as *const libc::c_void,
                (strlen(p)).wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            *p = ' ' as i32 as libc::c_char;
        }
        desc_tb[fnum
            as usize] = pango_font_description_from_string(font_name.as_mut_ptr());
    }
}
unsafe extern "C" fn pg_line_output(mut line: *mut PangoLayoutLine) {
    let mut runs_list: *mut GSList = 0 as *mut GSList;
    let mut glyph_info: *mut PangoGlyphInfo = 0 as *mut PangoGlyphInfo;
    let mut tmp: [libc::c_char; 256] = [0; 256];
    let mut fontname: *const libc::c_char = 0 as *const libc::c_char;
    let mut ret: libc::c_int = 0;
    let mut glypharray: libc::c_int = 0;
    outft = -(1 as libc::c_int);
    glypharray = 0 as libc::c_int;
    runs_list = (*line).runs;
    while !runs_list.is_null() {
        let mut run: *mut PangoLayoutRun = (*runs_list).data as *mut PangoLayoutRun;
        let mut item: *mut PangoItem = (*run).item;
        let mut glyphs: *mut PangoGlyphString = (*run).glyphs;
        let mut analysis: *mut PangoAnalysis = &mut (*item).analysis;
        let mut font: *mut PangoFont = (*analysis).font;
        let mut fc_font: *mut PangoFcFont = font as *mut libc::c_void
            as *mut PangoFcFont;
        let mut face: FT_Face = pango_fc_font_lock_face(fc_font);
        let mut ftdesc: *mut PangoFontDescription = pango_font_describe(font);
        let mut wi: libc::c_int = pango_font_description_get_size(ftdesc);
        let mut i: libc::c_int = 0;
        let mut c: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < (*glyphs).num_glyphs {
            glyph_info = &mut *((*glyphs).glyphs).offset(i as isize)
                as *mut PangoGlyphInfo;
            c = (*glyph_info).glyph as libc::c_int;
            if !(c as PangoGlyph == 0xfffffff as libc::c_int as PangoGlyph) {
                if c as PangoGlyph & 0x10000000 as libc::c_int as PangoGlyph != 0 {
                    c = (c as PangoGlyph & !(0x10000000 as libc::c_int as PangoGlyph))
                        as libc::c_int;
                    error(
                        0 as libc::c_int,
                        0 as *mut SYMBOL,
                        b"char %04x not treated\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        c,
                    );
                } else {
                    ret = FT_Load_Glyph(
                        face,
                        c as FT_UInt,
                        ((1 as libc::c_long) << 0 as libc::c_int) as FT_Int32,
                    );
                    if ret != 0 as libc::c_int {
                        error(
                            0 as libc::c_int,
                            0 as *mut SYMBOL,
                            b"freetype error %d\n\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            ret,
                        );
                    } else if (*face).face_flags
                        & (1 as libc::c_long) << 9 as libc::c_int != 0
                    {
                        if FT_Get_Postscript_Name(face) != fontname {
                            fontname = FT_Get_Postscript_Name(face);
                            if glypharray != 0 {
                                a2b(
                                    b"]glypharray\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                );
                            }
                            a2b(
                                b"\n/%s %.1f selectfont[\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                                fontname,
                                (wi as libc::c_float
                                    / (1024 as libc::c_int * 72 as libc::c_int
                                        / 96 as libc::c_int) as libc::c_float) as libc::c_double,
                            );
                            glypharray = 1 as libc::c_int;
                        }
                        FT_Get_Glyph_Name(
                            face,
                            c as FT_UInt,
                            tmp.as_mut_ptr() as FT_Pointer,
                            ::core::mem::size_of::<[libc::c_char; 256]>()
                                as libc::c_ulong as FT_UInt,
                        );
                        a2b(
                            b"/%s\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            tmp.as_mut_ptr(),
                        );
                    } else {
                        error(
                            0 as libc::c_int,
                            0 as *mut SYMBOL,
                            b"!! no glyph %d in %s-%s\n\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            c,
                            (*face).family_name,
                            (*face).style_name,
                        );
                    }
                }
            }
            i += 1;
            i;
        }
        pango_fc_font_unlock_face(fc_font);
        runs_list = (*runs_list).next;
    }
    if glypharray != 0 {
        a2b(b"]glypharray\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    }
}
unsafe extern "C" fn str_font_change(mut start: libc::c_int, mut end: libc::c_int) {
    let mut f: *mut FONTSPEC = 0 as *mut FONTSPEC;
    let mut fnum: libc::c_int = 0;
    let mut attr1: *mut PangoAttribute = 0 as *mut PangoAttribute;
    let mut attr2: *mut PangoAttribute = 0 as *mut PangoAttribute;
    f = &mut *(cfmt.font_tb).as_mut_ptr().offset(curft as isize) as *mut FONTSPEC;
    fnum = (*f).fnum;
    if (*f).size == 0 as libc::c_int as libc::c_float {
        error(
            0 as libc::c_int,
            0 as *mut SYMBOL,
            b"Font \"%s\" with a null size - set to 8\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            fontnames[fnum as usize],
        );
        (*f).size = 8 as libc::c_int as libc::c_float;
    }
    desc_font(fnum);
    attr1 = pango_attr_font_desc_new(desc_tb[fnum as usize]);
    (*attr1).start_index = start as guint;
    (*attr1).end_index = end as guint;
    pango_attr_list_insert(attrs, attr1);
    attr2 = pango_attr_size_new(
        ((*f).size
            * (1024 as libc::c_int * 72 as libc::c_int / 96 as libc::c_int)
                as libc::c_float) as libc::c_int,
    );
    (*attr2).start_index = start as guint;
    (*attr2).end_index = end as guint;
    pango_attr_list_insert(attrs, attr2);
}
unsafe extern "C" fn str_set_font(mut p: *mut libc::c_char) {
    let mut str: *mut GString = 0 as *mut GString;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start: libc::c_int = 0;
    str = pg_str;
    start = (*str).len as libc::c_int;
    q = p;
    while *p as libc::c_int != '\0' as i32 {
        match *p as libc::c_int {
            36 => {
                if isdigit(
                    *p.offset(1 as libc::c_int as isize) as libc::c_uchar as libc::c_int,
                ) != 0
                    && ((*p.offset(1 as libc::c_int as isize) as libc::c_int
                        - '0' as i32) as libc::c_uint)
                        < 10 as libc::c_int as libc::c_uint
                {
                    if p > q {
                        str = g_string_append_len_inline(
                            str,
                            q,
                            p.offset_from(q) as libc::c_long,
                        );
                    }
                    if curft
                        != *p.offset(1 as libc::c_int as isize) as libc::c_int
                            - '0' as i32
                    {
                        str_font_change(start, (*str).len as libc::c_int);
                        start = (*str).len as libc::c_int;
                        curft = *p.offset(1 as libc::c_int as isize) as libc::c_int
                            - '0' as i32;
                        if curft == 0 as libc::c_int {
                            curft = defft;
                        }
                    }
                    p = p.offset(2 as libc::c_int as isize);
                    q = p;
                    continue;
                } else if *p.offset(1 as libc::c_int as isize) as libc::c_int
                    == '$' as i32
                {
                    str = g_string_append_len_inline(
                        str,
                        q,
                        p.offset_from(q) as libc::c_long,
                    );
                    p = p.offset(1);
                    q = p;
                }
            }
            _ => {}
        }
        p = p.offset(1);
        p;
    }
    if p > q {
        str = g_string_append_len_inline(str, q, p.offset_from(q) as libc::c_long);
        str_font_change(start, (*str).len as libc::c_int);
    }
    pg_str = str;
}
unsafe extern "C" fn str_pg_out(mut p: *mut libc::c_char, mut action: libc::c_int) {
    let mut line: *mut PangoLayoutLine = 0 as *mut PangoLayoutLine;
    let mut wi: libc::c_int = 0;
    let mut w: libc::c_float = 0.;
    if out_pg_ft != curft {
        out_pg_ft = -(1 as libc::c_int);
    }
    if action == 6 as libc::c_int {
        let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
        q = mbf.offset(-(1 as libc::c_int as isize));
        while *q.offset(-(1 as libc::c_int) as isize) as libc::c_int != ' ' as i32 {
            q = q.offset(-1);
            q;
        }
        mbf = q;
        w = atof(q) as libc::c_float;
        loop {
            q = strchr(p, '\t' as i32);
            if q.is_null() {
                break;
            }
            *q = '\0' as i32 as libc::c_char;
            str_pg_out(p, 0 as libc::c_int);
            a2b(
                b" %.1f 0 RM \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                w as libc::c_double,
            );
            p = q.offset(1 as libc::c_int as isize);
        }
    }
    attrs = pango_attr_list_new();
    str_set_font(p);
    pango_layout_set_text(layout, (*pg_str).str_0, (*pg_str).len as libc::c_int);
    pango_layout_set_attributes(layout, attrs);
    line = pango_layout_get_line_readonly(layout, 0 as libc::c_int);
    match action {
        1 | 2 => {
            pango_layout_get_size(layout, &mut wi, 0 as *mut libc::c_int);
            if action == 1 as libc::c_int {
                wi /= 2 as libc::c_int;
            }
            w = wi as libc::c_float / 1024 as libc::c_int as libc::c_float;
            a2b(
                b" -%.1f 0 RM\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                w as libc::c_double,
            );
        }
        _ => {}
    }
    pg_line_output(line);
    pango_layout_set_attributes(layout, 0 as *mut PangoAttrList);
    pg_str = g_string_truncate_inline(pg_str, 0 as libc::c_int as gsize);
    pango_attr_list_unref(attrs);
}
unsafe extern "C" fn pg_para_output(mut job: libc::c_int) {
    let mut lines: *mut GSList = 0 as *mut GSList;
    let mut runs_list: *mut GSList = 0 as *mut GSList;
    let mut line: *mut PangoLayoutLine = 0 as *mut PangoLayoutLine;
    let mut glyph_info: *mut PangoGlyphInfo = 0 as *mut PangoGlyphInfo;
    let mut tmp: [libc::c_char; 256] = [0; 256];
    let mut fontname: *const libc::c_char = 0 as *const libc::c_char;
    let mut ret: libc::c_int = 0;
    let mut glypharray: libc::c_int = 0;
    let mut wi: libc::c_int = 0;
    let mut y: libc::c_float = 0.;
    pango_layout_set_text(
        layout,
        (*pg_str).str_0,
        ((*pg_str).len).wrapping_sub(1 as libc::c_int as gsize) as libc::c_int,
    );
    pango_layout_set_attributes(layout, attrs);
    outft = -(1 as libc::c_int);
    glypharray = 0 as libc::c_int;
    wi = 0 as libc::c_int;
    y = 0 as libc::c_int as libc::c_float;
    lines = pango_layout_get_lines_readonly(layout);
    while !lines.is_null() {
        let mut pos: PangoRectangle = _PangoRectangle {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        line = (*lines).data as *mut PangoLayoutLine;
        pango_layout_line_get_extents(line, 0 as *mut PangoRectangle, &mut pos);
        y = (y as libc::c_double
            + pos.height as libc::c_float as libc::c_double * 0.87f64
                / 1024 as libc::c_int as libc::c_double) as libc::c_float;
        runs_list = (*line).runs;
        while !runs_list.is_null() {
            let mut run: *mut PangoLayoutRun = (*runs_list).data as *mut PangoLayoutRun;
            let mut item: *mut PangoItem = (*run).item;
            let mut glyphs: *mut PangoGlyphString = (*run).glyphs;
            let mut analysis: *mut PangoAnalysis = &mut (*item).analysis;
            let mut font: *mut PangoFont = (*analysis).font;
            let mut fc_font: *mut PangoFcFont = font as *mut libc::c_void
                as *mut PangoFcFont;
            let mut face: FT_Face = pango_fc_font_lock_face(fc_font);
            let mut ftdesc: *mut PangoFontDescription = pango_font_describe(font);
            let mut i: libc::c_int = 0;
            let mut g: libc::c_int = 0;
            let mut set_move: libc::c_int = 0;
            let mut x: libc::c_int = 0;
            if pango_font_description_get_size(ftdesc) != wi {
                wi = pango_font_description_get_size(ftdesc);
                fontname = 0 as *const libc::c_char;
            }
            pango_layout_index_to_pos(layout, (*item).offset, &mut pos);
            x = pos.x;
            set_move = 1 as libc::c_int;
            i = 0 as libc::c_int;
            while i < (*glyphs).num_glyphs {
                glyph_info = &mut *((*glyphs).glyphs).offset(i as isize)
                    as *mut PangoGlyphInfo;
                g = (*glyph_info).glyph as libc::c_int;
                if !(g as PangoGlyph == 0xfffffff as libc::c_int as PangoGlyph) {
                    if set_move != 0 {
                        set_move = 0 as libc::c_int;
                        if glypharray != 0 {
                            a2b(
                                b"]glypharray\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                            glypharray = 0 as libc::c_int;
                        }
                        a2b(
                            b"\n\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                        a2b(
                            b"%.2f %.2f M \0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            (x as libc::c_float / 1024 as libc::c_int as libc::c_float)
                                as libc::c_double,
                            -y as libc::c_double,
                        );
                    }
                    x += (*glyph_info).geometry.width;
                    if g as PangoGlyph & 0x10000000 as libc::c_int as PangoGlyph != 0 {
                        g = (g as PangoGlyph
                            & !(0x10000000 as libc::c_int as PangoGlyph)) as libc::c_int;
                        error(
                            0 as libc::c_int,
                            0 as *mut SYMBOL,
                            b"char %04x not treated\n\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            g,
                        );
                    } else {
                        ret = FT_Load_Glyph(
                            face,
                            g as FT_UInt,
                            ((1 as libc::c_long) << 0 as libc::c_int) as FT_Int32,
                        );
                        if ret != 0 as libc::c_int {
                            fprintf(
                                __stdoutp,
                                b"%%%% freetype error %d\n\0" as *const u8
                                    as *const libc::c_char,
                                ret,
                            );
                        } else if (*face).face_flags
                            & (1 as libc::c_long) << 9 as libc::c_int != 0
                        {
                            if FT_Get_Postscript_Name(face) != fontname {
                                fontname = FT_Get_Postscript_Name(face);
                                if glypharray != 0 {
                                    a2b(
                                        b"]glypharray\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                    );
                                }
                                a2b(
                                    b"\n/%s %.1f selectfont[\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                    fontname,
                                    (wi as libc::c_float
                                        / (1024 as libc::c_int * 72 as libc::c_int
                                            / 96 as libc::c_int) as libc::c_float) as libc::c_double,
                                );
                                glypharray = 1 as libc::c_int;
                            }
                            FT_Get_Glyph_Name(
                                face as *mut FT_FaceRec,
                                g as FT_UInt,
                                tmp.as_mut_ptr() as FT_Pointer,
                                ::core::mem::size_of::<[libc::c_char; 256]>()
                                    as libc::c_ulong as FT_UInt,
                            );
                            if job == 1 as libc::c_int
                                && strcmp(
                                    tmp.as_mut_ptr(),
                                    b"space\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                            {
                                set_move = 1 as libc::c_int;
                            } else {
                                if glypharray == 0 {
                                    a2b(
                                        b"[\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                    );
                                    glypharray = 1 as libc::c_int;
                                }
                                a2b(
                                    b"/%s\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                    tmp.as_mut_ptr(),
                                );
                            }
                        } else {
                            a2b(
                                b"%% glyph: %s %d\n\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                FT_Get_Postscript_Name(face),
                                g,
                            );
                        }
                    }
                }
                i += 1;
                i;
            }
            pango_fc_font_unlock_face(fc_font);
            if glypharray != 0 {
                a2b(
                    b"]glypharray\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                glypharray = 0 as libc::c_int;
            }
            runs_list = (*runs_list).next;
        }
        if glypharray != 0 {
            a2b(
                b"]glypharray\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            glypharray = 0 as libc::c_int;
        }
        lines = (*lines).next;
    }
    bskip(y);
    pango_layout_set_attributes(layout, 0 as *mut PangoAttrList);
    pg_str = g_string_truncate_inline(pg_str, 0 as libc::c_int as gsize);
}
unsafe extern "C" fn pg_write_text(
    mut s: *mut libc::c_char,
    mut job: libc::c_int,
    mut parskip: libc::c_float,
) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    curft = defft;
    pango_layout_set_width(
        layout,
        (strlw * 1024 as libc::c_int as libc::c_float) as libc::c_int,
    );
    pango_layout_set_justify(layout, (job == 1 as libc::c_int) as libc::c_int);
    attrs = pango_attr_list_new();
    p = s;
    while *p as libc::c_int != '\0' as i32 {
        let fresh36 = p;
        p = p.offset(1);
        if *fresh36 as libc::c_int != '\n' as i32 {
            continue;
        }
        if *p as libc::c_int == '\n' as i32 {
            *p.offset(-(1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
            tex_str(s);
            str_set_font(tex_buf.as_mut_ptr());
            if (*pg_str).len > 0 as libc::c_int as gsize {
                pg_para_output(job);
            }
            bskip(parskip);
            buffer_eob(0 as libc::c_int);
            p = p.offset(1);
            s = p;
        } else {
            *p.offset(-(1 as libc::c_int) as isize) = ' ' as i32 as libc::c_char;
        }
    }
    tex_str(s);
    str_set_font(tex_buf.as_mut_ptr());
    if (*pg_str).len != 0 {
        pg_para_output(job);
    }
    pango_attr_list_unref(attrs);
}
unsafe extern "C" fn is_latin(mut p: *mut libc::c_uchar) -> libc::c_int {
    while *p as libc::c_int != '\0' as i32 {
        if *p as libc::c_int >= 0xc6 as libc::c_int {
            if *p as libc::c_int == 0xe2 as libc::c_int {
                if *p.offset(1 as libc::c_int as isize) as libc::c_int
                    != 0x99 as libc::c_int
                    || (*p.offset(2 as libc::c_int as isize) as libc::c_int)
                        < 0xad as libc::c_int
                    || *p.offset(2 as libc::c_int as isize) as libc::c_int
                        > 0xaf as libc::c_int
                {
                    return 0 as libc::c_int;
                }
                p = p.offset(2 as libc::c_int as isize);
            } else if *p as libc::c_int == 0xf0 as libc::c_int {
                if *p.offset(1 as libc::c_int as isize) as libc::c_int
                    != 0x9d as libc::c_int
                    || *p.offset(2 as libc::c_int as isize) as libc::c_int
                        != 0x84 as libc::c_int
                    || (*p.offset(3 as libc::c_int as isize) as libc::c_int)
                        < 0xaa as libc::c_int
                    || *p.offset(3 as libc::c_int as isize) as libc::c_int
                        > 0xab as libc::c_int
                {
                    return 0 as libc::c_int;
                }
            } else {
                return 0 as libc::c_int
            }
        }
        p = p.offset(1);
        p;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn str_font(mut ft: libc::c_int) {
    defft = ft;
    curft = defft;
}
#[no_mangle]
pub unsafe extern "C" fn get_str_font(
    mut cft: *mut libc::c_int,
    mut dft: *mut libc::c_int,
) {
    *cft = curft;
    *dft = defft;
}
#[no_mangle]
pub unsafe extern "C" fn set_str_font(mut cft: libc::c_int, mut dft: libc::c_int) {
    curft = cft;
    defft = dft;
}
static mut strop_tb: [*mut libc::c_char; 16] = [
    b"show\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"arrayshow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"showc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"arrayshow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"showr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"arrayshow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"lyshow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"alyshow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gcshow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"agcshow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"anshow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"aanshow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gxshow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"arrayshow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"strop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"arrayshow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
unsafe extern "C" fn str_end(mut end: libc::c_int) {
    if strtx as libc::c_int & 1 as libc::c_int != 0 {
        a2b(b")\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        strtx = (strtx as libc::c_int & !(1 as libc::c_int)) as libc::c_char;
        if strtx as libc::c_int & 2 as libc::c_int == 0 {
            a2b(
                b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                strop_tb[stropx as usize],
            );
            return;
        }
    }
    if end == 0 || strtx as libc::c_int & 2 as libc::c_int == 0 {
        return;
    }
    strtx = (strtx as libc::c_int & !(2 as libc::c_int)) as libc::c_char;
    a2b(
        b"]%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        strop_tb[(stropx + 1 as libc::c_int) as usize],
    );
}
unsafe extern "C" fn non_ascii_p(mut p: *mut libc::c_char) -> libc::c_int {
    while *p as libc::c_int != '\0' as i32 {
        let fresh37 = p;
        p = p.offset(1);
        if (*fresh37 as libc::c_schar as libc::c_int) < 0 as libc::c_int {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn str_ft_out1(mut p: *mut libc::c_char, mut l: libc::c_int) {
    if curft != outft {
        str_end(1 as libc::c_int);
        a2b(b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        set_font(curft);
    }
    if strtx as libc::c_int & 1 as libc::c_int == 0 {
        a2b(b"(\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        strtx = (strtx as libc::c_int | 1 as libc::c_int) as libc::c_char;
    }
    a2b(b"%.*s\0" as *const u8 as *const libc::c_char as *mut libc::c_char, l, p);
}
unsafe extern "C" fn str_ft_out(mut p: *mut libc::c_char, mut end: libc::c_int) {
    let mut use_glyph: libc::c_int = 0;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    use_glyph = (svg == 0 && epsf <= 1 as libc::c_int
        && get_font_encoding(curft) == 0 as libc::c_int) as libc::c_int;
    if use_glyph != 0 && non_ascii_p(p) != 0 {
        if curft != outft {
            str_end(1 as libc::c_int);
            a2b(b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char);
            set_font(curft);
        }
        str_end(0 as libc::c_int);
        if strtx as libc::c_int & 2 as libc::c_int == 0 {
            a2b(b"[\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
            strtx = (strtx as libc::c_int | 2 as libc::c_int) as libc::c_char;
        }
    }
    q = p;
    while *p as libc::c_int != '\0' as i32 {
        if (*p as libc::c_schar as libc::c_int) < 0 as libc::c_int && use_glyph != 0 {
            if p > q {
                str_ft_out1(q, p.offset_from(q) as libc::c_long as libc::c_int);
            }
            str_end(0 as libc::c_int);
            if curft != outft {
                str_end(1 as libc::c_int);
                a2b(b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char);
                set_font(curft);
            }
            if strtx as libc::c_int & 2 as libc::c_int == 0 {
                a2b(b"[\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
                strtx = (strtx as libc::c_int | 2 as libc::c_int) as libc::c_char;
            }
            p = glyph_out(p);
            q = p;
        } else {
            match *p as libc::c_int {
                36 => {
                    if isdigit(
                        *p.offset(1 as libc::c_int as isize) as libc::c_uchar
                            as libc::c_int,
                    ) != 0
                        && ((*p.offset(1 as libc::c_int as isize) as libc::c_int
                            - '0' as i32) as libc::c_uint)
                            < 10 as libc::c_int as libc::c_uint
                    {
                        if p > q {
                            str_ft_out1(
                                q,
                                p.offset_from(q) as libc::c_long as libc::c_int,
                            );
                        }
                        if curft
                            != *p.offset(1 as libc::c_int as isize) as libc::c_int
                                - '0' as i32
                        {
                            curft = *p.offset(1 as libc::c_int as isize) as libc::c_int
                                - '0' as i32;
                            if curft == 0 as libc::c_int {
                                curft = defft;
                            }
                            use_glyph = (svg == 0 && epsf <= 1 as libc::c_int
                                && get_font_encoding(curft) == 0 as libc::c_int)
                                as libc::c_int;
                        }
                        p = p.offset(2 as libc::c_int as isize);
                        q = p;
                        continue;
                    } else if *p.offset(1 as libc::c_int as isize) as libc::c_int
                        == '$' as i32
                    {
                        str_ft_out1(q, p.offset_from(q) as libc::c_long as libc::c_int);
                        p = p.offset(1);
                        q = p;
                    }
                }
                40 | 41 | 92 => {
                    if p > q {
                        str_ft_out1(q, p.offset_from(q) as libc::c_long as libc::c_int);
                    }
                    str_ft_out1(
                        b"\\\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        1 as libc::c_int,
                    );
                    q = p;
                }
                _ => {}
            }
            p = p.offset(1);
            p;
        }
    }
    if p > q {
        str_ft_out1(q, p.offset_from(q) as libc::c_long as libc::c_int);
    }
    if end != 0 && strtx as libc::c_int != 0 {
        str_end(1 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn str_out(mut p: *mut libc::c_char, mut action: libc::c_int) {
    if curft <= 0 as libc::c_int {
        curft = defft;
    }
    if *p as libc::c_int == '$' as i32
        && isdigit(*p.offset(1 as libc::c_int as isize) as libc::c_uchar as libc::c_int)
            != 0
        && ((*p.offset(1 as libc::c_int as isize) as libc::c_int - '0' as i32)
            as libc::c_uint) < 10 as libc::c_int as libc::c_uint
    {
        if curft != *p.offset(1 as libc::c_int as isize) as libc::c_int - '0' as i32 {
            curft = *p.offset(1 as libc::c_int as isize) as libc::c_int - '0' as i32;
            if curft == 0 as libc::c_int {
                curft = defft;
            }
        }
        p = p.offset(2 as libc::c_int as isize);
    }
    if cfmt.pango != 0 {
        if cfmt.pango == 2 as libc::c_int || is_latin(p as *mut libc::c_uchar) == 0 {
            str_pg_out(p, action);
            return;
        }
    }
    stropx = action * 2 as libc::c_int;
    if (strchr(p, '$' as i32)).is_null() && non_ascii_p(p) == 0 {
        str_ft_out(p, 1 as libc::c_int);
        return;
    }
    match action {
        1 | 2 => {
            if svg == 0 && epsf <= 1 as libc::c_int {
                a2b(b"/str{\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
                outft = -(1 as libc::c_int);
                stropx = 0 as libc::c_int;
            }
        }
        _ => {}
    }
    str_ft_out(p, 1 as libc::c_int);
    if svg != 0 || epsf > 1 as libc::c_int {
        return;
    }
    if action == 1 as libc::c_int || action == 2 as libc::c_int {
        a2b(b"}def\nstrw w\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        if action == 1 as libc::c_int {
            a2b(b" 0.5 mul\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        }
        a2b(b" neg 0 RM str\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    }
}
#[no_mangle]
pub unsafe extern "C" fn put_str(mut str: *mut libc::c_char, mut action: libc::c_int) {
    tex_str(str);
    str_out(tex_buf.as_mut_ptr(), action);
    a2b(b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
}
unsafe extern "C" fn put_inf(mut s: *mut SYMBOL) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = (*s).text;
    if *p.offset(1 as libc::c_int as isize) as libc::c_int == ':' as i32 {
        p = p.offset(2 as libc::c_int as isize);
    }
    while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
        p = p.offset(1);
        p;
    }
    put_str(p, 0 as libc::c_int);
}
unsafe extern "C" fn put_inf2r(
    mut s1: *mut SYMBOL,
    mut s2: *mut SYMBOL,
    mut action: libc::c_int,
) {
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    if s1.is_null() {
        s1 = s2;
        s2 = 0 as *mut SYMBOL;
    }
    p = &mut *((*s1).text).offset(2 as libc::c_int as isize) as *mut libc::c_char;
    if *((*s1).text).offset(0 as libc::c_int as isize) as libc::c_int == 'T' as i32 {
        p = trim_title(p, s1);
    }
    if !s2.is_null() {
        buf[(::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            as usize] = '\0' as i32 as libc::c_char;
        strncpy(
            buf.as_mut_ptr(),
            p,
            (::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        q = buf.as_mut_ptr().offset(strlen(buf.as_mut_ptr()) as isize);
        if q
            < buf
                .as_mut_ptr()
                .offset(
                    ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                        as isize,
                )
                .offset(-(4 as libc::c_int as isize))
        {
            let fresh38 = q;
            q = q.offset(1);
            *fresh38 = ' ' as i32 as libc::c_char;
            let fresh39 = q;
            q = q.offset(1);
            *fresh39 = '(' as i32 as libc::c_char;
            p = &mut *((*s2).text).offset(2 as libc::c_int as isize)
                as *mut libc::c_char;
            strncpy(
                q,
                p,
                buf
                    .as_mut_ptr()
                    .offset(
                        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                            as isize,
                    )
                    .offset(-(2 as libc::c_int as isize))
                    .offset_from(q) as libc::c_long as libc::c_ulong,
            );
            q = q.offset(strlen(q) as isize);
            let fresh40 = q;
            q = q.offset(1);
            *fresh40 = ')' as i32 as libc::c_char;
            *q = '\0' as i32 as libc::c_char;
        }
        p = buf.as_mut_ptr();
    }
    put_str(p, action);
}
#[no_mangle]
pub unsafe extern "C" fn write_text(
    mut cmd: *mut libc::c_char,
    mut s: *mut libc::c_char,
    mut job: libc::c_int,
) {
    let mut current_block: u64;
    let mut nw: libc::c_int = 0;
    let mut do_pango: libc::c_int = 0;
    let mut lineskip: libc::c_float = 0.;
    let mut parskip: libc::c_float = 0.;
    let mut strw: libc::c_float = 0.;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut f: *mut FONTSPEC = 0 as *mut FONTSPEC;
    str_font(TEXTFONT as libc::c_int);
    strlw = ((if cfmt.landscape != 0 { cfmt.pageheight } else { cfmt.pagewidth })
        - cfmt.leftmargin - cfmt.rightmargin) / cfmt.scale;
    f = &mut *(cfmt.font_tb).as_mut_ptr().offset(TEXTFONT as libc::c_int as isize)
        as *mut FONTSPEC;
    lineskip = (*f).size * cfmt.lineskipfac;
    parskip = (*f).size * cfmt.parskipfac;
    match job {
        0 | 3 | 5 => {
            match job {
                0 => {
                    strlw = 0 as libc::c_int as libc::c_float;
                }
                3 => {
                    job = 1 as libc::c_int;
                    strlw /= 2 as libc::c_int as libc::c_float;
                }
                _ => {
                    job = 2 as libc::c_int;
                }
            }
            p = s;
            while *s as libc::c_int != '\0' as i32 {
                while *p as libc::c_int != '\0' as i32
                    && *p as libc::c_int != '\n' as i32
                {
                    p = p.offset(1);
                    p;
                }
                if *p as libc::c_int != '\0' as i32 {
                    let fresh41 = p;
                    p = p.offset(1);
                    *fresh41 = '\0' as i32 as libc::c_char;
                }
                if *s as libc::c_int == '\0' as i32 {
                    bskip(parskip);
                    buffer_eob(0 as libc::c_int);
                    while *p as libc::c_int == '\n' as i32 {
                        bskip(lineskip);
                        p = p.offset(1);
                        p;
                    }
                    if *p as libc::c_int == '\0' as i32 {
                        break;
                    }
                } else {
                    bskip(lineskip);
                    a2b(
                        b"%.1f 0 M\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        strlw as libc::c_double,
                    );
                    put_str(s, job);
                }
                s = p;
            }
        }
        _ => {
            do_pango = cfmt.pango;
            if do_pango == 1 as libc::c_int {
                do_pango = (is_latin(s as *mut libc::c_uchar) == 0) as libc::c_int;
            }
            if do_pango != 0 {
                pg_write_text(s, job, parskip);
            } else {
                nw = 0 as libc::c_int;
                strw = 0 as libc::c_int as libc::c_float;
                stropx = (if job == 2 as libc::c_int {
                    0 as libc::c_int
                } else {
                    7 as libc::c_int
                }) * 2 as libc::c_int;
                loop {
                    if !(*s as libc::c_int != '\0' as i32) {
                        current_block = 1852451392920375136;
                        break;
                    }
                    let mut lw: libc::c_float = 0.;
                    if *s as libc::c_int == '\n' as i32 {
                        if strtx != 0 {
                            str_end(1 as libc::c_int);
                            if job == 1 as libc::c_int {
                                a2b(
                                    b"}def\n/strop/show load def str\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                );
                            }
                            a2b(
                                b"\n\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                        }
                        bskip(parskip);
                        buffer_eob(0 as libc::c_int);
                        while *s as libc::c_int == '\n' as i32 {
                            bskip(lineskip);
                            s = s.offset(1);
                            s;
                        }
                        if *s as libc::c_int == '\0' as i32 {
                            current_block = 13930098733778104621;
                            break;
                        }
                        nw = 0 as libc::c_int;
                    }
                    if nw == 0 as libc::c_int {
                        bskip(lineskip);
                        a2b(
                            b"0 0 M\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                        if job != 2 as libc::c_int {
                            a2b(
                                b"/str{\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                            outft = -(1 as libc::c_int);
                        }
                        strw = 0 as libc::c_int as libc::c_float;
                    }
                    p = s;
                    while *p as libc::c_int != '\0' as i32
                        && isspace(*p as libc::c_uchar as libc::c_int) == 0
                    {
                        p = p.offset(1);
                        p;
                    }
                    if *p as libc::c_int != '\0' as i32 {
                        let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
                        q = p;
                        if *p as libc::c_int != '\n' as i32 {
                            loop {
                                p = p.offset(1);
                                p;
                                if !(*p as libc::c_int != '\n' as i32
                                    && isspace(*p as libc::c_uchar as libc::c_int) != 0)
                                {
                                    break;
                                }
                            }
                        }
                        if *p as libc::c_int == '\n' as i32 {
                            p = p.offset(1);
                            p;
                        }
                        *q = '\0' as i32 as libc::c_char;
                    }
                    lw = tex_str(s);
                    if strw + lw > strlw {
                        str_end(1 as libc::c_int);
                        if job == 1 as libc::c_int {
                            let mut n: libc::c_int = 0;
                            n = nw - 1 as libc::c_int;
                            if n <= 0 as libc::c_int {
                                n = 1 as libc::c_int;
                            }
                            if svg != 0 || epsf > 1 as libc::c_int {
                                a2b(
                                    b"}def\n%.1f jshow/strop/show load def str\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                    strlw as libc::c_double,
                                );
                            } else {
                                a2b(
                                    b"}def\nstrw/w %.1f w sub %d div def/strop/jshow load def str\0"
                                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                                    strlw as libc::c_double,
                                    n,
                                );
                            }
                        }
                        a2b(
                            b"\n\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                        bskip(lineskip);
                        a2b(
                            b"0 0 M\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                        if job == 1 as libc::c_int {
                            a2b(
                                b"/str{\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                            outft = -(1 as libc::c_int);
                        }
                        nw = 0 as libc::c_int;
                        strw = 0 as libc::c_int as libc::c_float;
                    }
                    if nw != 0 as libc::c_int {
                        str_ft_out1(
                            b" \0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            1 as libc::c_int,
                        );
                        strw
                            += cwid(' ' as i32 as libc::c_uchar)
                                * cfmt.font_tb[curft as usize].swfac;
                    }
                    str_ft_out(tex_buf.as_mut_ptr(), 0 as libc::c_int);
                    strw += lw;
                    nw += 1;
                    nw;
                    s = p;
                }
                match current_block {
                    13930098733778104621 => {}
                    _ => {
                        if strtx != 0 {
                            str_end(1 as libc::c_int);
                            if job == 1 as libc::c_int {
                                a2b(
                                    b"}def\n/strop/show load def str\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                );
                            }
                        }
                        a2b(
                            b"\n\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                    }
                }
            }
        }
    }
    bskip(parskip);
    buffer_eob(0 as libc::c_int);
}
unsafe extern "C" fn put_wline(
    mut p: *mut libc::c_char,
    mut x: libc::c_float,
    mut right: libc::c_int,
) -> libc::c_int {
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sep: libc::c_char = 0;
    while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
        p = p.offset(1);
        p;
    }
    if *p as libc::c_int == '$' as i32
        && isdigit(*p.offset(1 as libc::c_int as isize) as libc::c_uchar as libc::c_int)
            != 0
        && ((*p.offset(1 as libc::c_int as isize) as libc::c_int - '0' as i32)
            as libc::c_uint) < 10 as libc::c_int as libc::c_uint
    {
        if curft != *p.offset(1 as libc::c_int as isize) as libc::c_int - '0' as i32 {
            curft = *p.offset(1 as libc::c_int as isize) as libc::c_int - '0' as i32;
            if curft == 0 as libc::c_int {
                curft = defft;
            }
        }
        p = p.offset(2 as libc::c_int as isize);
    }
    r = 0 as *mut libc::c_char;
    q = p;
    if isdigit(*p as libc::c_uchar as libc::c_int) != 0
        || *p as libc::c_int != '\0' as i32
            && *p.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
    {
        while *p as libc::c_int != '\0' as i32 {
            p = p.offset(1);
            p;
            if *p as libc::c_int == ' ' as i32
                || *p.offset(-(1 as libc::c_int) as isize) as libc::c_int == ':' as i32
                || *p.offset(-(1 as libc::c_int) as isize) as libc::c_int == '.' as i32
            {
                break;
            }
        }
        r = p;
        while *p as libc::c_int == ' ' as i32 {
            p = p.offset(1);
            p;
        }
    }
    if !r.is_null() {
        sep = *r;
        *r = '\0' as i32 as libc::c_char;
        a2b(
            b"%.1f 0 M\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            x as libc::c_double,
        );
        put_str(q, 2 as libc::c_int);
        *r = sep;
    }
    if *p as libc::c_int != '\0' as i32 {
        a2b(
            b"%.1f 0 M\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (x + 5 as libc::c_int as libc::c_float) as libc::c_double,
        );
        put_str(p, 0 as libc::c_int);
    }
    return (*p as libc::c_int == '\0' as i32 && r.is_null()) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn put_words(mut words: *mut SYMBOL) {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s_end: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut have_text: libc::c_int = 0;
    let mut max2col: libc::c_int = 0;
    let mut middle: libc::c_float = 0.;
    buffer_eob(0 as libc::c_int);
    str_font(WORDSFONT as libc::c_int);
    middle = (0.5f64
        * ((if cfmt.landscape != 0 { cfmt.pageheight } else { cfmt.pagewidth })
            - cfmt.leftmargin - cfmt.rightmargin) as libc::c_double
        / cfmt.scale as libc::c_double) as libc::c_float;
    max2col = ((middle as libc::c_double - 45.0f64)
        / (cwid('a' as i32 as libc::c_uchar)
            * cfmt.font_tb[WORDSFONT as libc::c_int as usize].swfac) as libc::c_double)
        as libc::c_int;
    n = 0 as libc::c_int;
    have_text = 0 as libc::c_int;
    s = words;
    while !s.is_null() {
        p = &mut *((*s).text).offset(2 as libc::c_int as isize) as *mut libc::c_char;
        if strlen(p) as libc::c_int > max2col {
            n = 0 as libc::c_int;
            break;
        } else {
            if *p as libc::c_int == '\0' as i32 {
                if have_text != 0 {
                    n += 1;
                    n;
                    have_text = 0 as libc::c_int;
                }
            } else {
                have_text = 1 as libc::c_int;
            }
            s = (*s).next;
        }
    }
    if n > 0 as libc::c_int {
        n += 1;
        n;
        n /= 2 as libc::c_int;
        i = n;
        have_text = 0 as libc::c_int;
        s_end = words;
        loop {
            p = &mut *((*s_end).text).offset(2 as libc::c_int as isize)
                as *mut libc::c_char;
            while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
                p = p.offset(1);
                p;
            }
            if *p as libc::c_int == '\0' as i32 {
                if have_text != 0
                    && {
                        i -= 1;
                        i <= 0 as libc::c_int
                    }
                {
                    break;
                }
                have_text = 0 as libc::c_int;
            } else {
                have_text = 1 as libc::c_int;
            }
            s_end = (*s_end).next;
        }
        s2 = (*s_end).next;
    } else {
        s_end = 0 as *mut SYMBOL;
        s2 = 0 as *mut SYMBOL;
    }
    bskip(cfmt.wordsspace);
    s = words;
    while !s.is_null() || !s2.is_null() {
        if !s.is_null()
            && *((*s).text).offset(2 as libc::c_int as isize) as libc::c_int
                == '\0' as i32
        {
            buffer_eob(0 as libc::c_int);
        }
        bskip(cfmt.lineskipfac * cfmt.font_tb[WORDSFONT as libc::c_int as usize].size);
        if !s.is_null() {
            put_wline(
                &mut *((*s).text).offset(2 as libc::c_int as isize),
                45.0f64 as libc::c_float,
                0 as libc::c_int,
            );
            s = (*s).next;
            if s == s_end {
                s = 0 as *mut SYMBOL;
            }
        }
        if !s2.is_null() {
            if put_wline(
                &mut *((*s2).text).offset(2 as libc::c_int as isize),
                (20.0f64 + middle as libc::c_double) as libc::c_float,
                1 as libc::c_int,
            ) != 0
            {
                n -= 1;
                if n == 0 as libc::c_int {
                    if !s.is_null() {
                        n += 1;
                        n;
                    } else if !((*s2).next).is_null() {
                        middle = (middle as libc::c_double * 0.6f64) as libc::c_float;
                    }
                }
            }
            s2 = (*s2).next;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn put_history() {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut font: libc::c_int = 0;
    let mut u: libc::c_uint = 0;
    let mut w: libc::c_float = 0.;
    let mut h: libc::c_float = 0.;
    let mut tmp: [libc::c_char; 265] = [0; 265];
    font = 0 as libc::c_int;
    s = info[('I' as i32 - 'A' as i32) as usize];
    while !s.is_null() {
        u = (*((*s).text).offset(0 as libc::c_int as isize) as libc::c_int - 'A' as i32)
            as libc::c_uint;
        if !(cfmt.fields[0 as libc::c_int as usize]
            & ((1 as libc::c_int) << u) as libc::c_uint == 0
            || {
                s2 = info[u as usize];
                s2.is_null()
            })
        {
            if font == 0 {
                bskip(cfmt.textspace);
                str_font(HISTORYFONT as libc::c_int);
                font = 1 as libc::c_int;
            }
            get_str(
                tmp.as_mut_ptr(),
                &mut *((*s).text).offset(1 as libc::c_int as isize),
                ::core::mem::size_of::<[libc::c_char; 265]>() as libc::c_ulong
                    as libc::c_int,
            );
            w = tex_str(tmp.as_mut_ptr());
            h = cfmt.font_tb[HISTORYFONT as libc::c_int as usize].size
                * cfmt.lineskipfac;
            set_font(HISTORYFONT as libc::c_int);
            a2b(b"0 0 M\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
            str_out(tex_buf.as_mut_ptr(), 0 as libc::c_int);
            loop {
                put_inf(s2);
                s2 = (*s2).next;
                if s2.is_null() {
                    break;
                }
                if *((*s2).text).offset(0 as libc::c_int as isize) as libc::c_int
                    == '+' as i32
                    && *((*s2).text).offset(1 as libc::c_int as isize) as libc::c_int
                        == ':' as i32
                {
                    put_str(
                        b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        0 as libc::c_int,
                    );
                } else {
                    bskip(h);
                    a2b(
                        b"%.2f 0 M \0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        w as libc::c_double,
                    );
                }
            }
            bskip((h as libc::c_double * 1.2f64) as libc::c_float);
            buffer_eob(0 as libc::c_int);
        }
        s = (*s).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn trim_title(
    mut p: *mut libc::c_char,
    mut title: *mut SYMBOL,
) -> *mut libc::c_char {
    let mut b: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut buf: [libc::c_char; 256] = [0; 256];
    q = 0 as *mut libc::c_char;
    if cfmt.titletrim != 0 {
        q = strrchr(p, ',' as i32);
        if !q.is_null() {
            if *q.offset(1 as libc::c_int as isize) as libc::c_int != ' ' as i32
                || isupper(
                    *q.offset(2 as libc::c_int as isize) as libc::c_uchar as libc::c_int,
                ) == 0
            {
                q = 0 as *mut libc::c_char;
            } else if cfmt.titletrim == 1 as libc::c_int {
                if strlen(q) > 7 as libc::c_int as libc::c_ulong
                    || !(strchr(q.offset(2 as libc::c_int as isize), ' ' as i32))
                        .is_null()
                {
                    q = 0 as *mut libc::c_char;
                }
            } else if strlen(q) > (cfmt.titletrim - 2 as libc::c_int) as libc::c_ulong {
                q = 0 as *mut libc::c_char;
            }
        }
    }
    if title != info[('T' as i32 - 'A' as i32) as usize]
        || cfmt.fields[0 as libc::c_int as usize]
            & ((1 as libc::c_int) << 'X' as i32 - 'A' as i32) as libc::c_uint == 0
    {
        title = 0 as *mut SYMBOL;
    }
    if q.is_null() && title.is_null() && cfmt.titlecaps == 0 {
        return p;
    }
    b = buf.as_mut_ptr();
    r = &mut *((**info.as_mut_ptr().offset(('X' as i32 - 'A' as i32) as isize)).text)
        .offset(2 as libc::c_int as isize) as *mut libc::c_char;
    if !title.is_null() && *r as libc::c_int != '\0' as i32 {
        if (strlen(p))
            .wrapping_add(strlen(r))
            .wrapping_add(3 as libc::c_int as libc::c_ulong)
            >= ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
        {
            error(
                1 as libc::c_int,
                0 as *mut SYMBOL,
                b"Title or X: too long\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return p;
        }
        b = b
            .offset(
                sprintf(b, b"%s.  \0" as *const u8 as *const libc::c_char, r) as isize,
            );
    } else if strlen(p) >= ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
    {
        error(
            1 as libc::c_int,
            0 as *mut SYMBOL,
            b"Title too long\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return p;
    }
    if !q.is_null() {
        sprintf(
            b,
            b"%s %.*s\0" as *const u8 as *const libc::c_char,
            q.offset(2 as libc::c_int as isize),
            q.offset_from(p) as libc::c_long as libc::c_int,
            p,
        );
    } else {
        strcpy(b, p);
    }
    if cfmt.titlecaps != 0 {
        cap_str(buf.as_mut_ptr());
    }
    return buf.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn write_title(mut s: *mut SYMBOL) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sz: libc::c_float = 0.;
    p = &mut *((*s).text).offset(2 as libc::c_int as isize) as *mut libc::c_char;
    if *p as libc::c_int == '\0' as i32 {
        return;
    }
    if s == info[('T' as i32 - 'A' as i32) as usize] {
        sz = cfmt.font_tb[TITLEFONT as libc::c_int as usize].size;
        bskip(cfmt.titlespace + sz);
        str_font(TITLEFONT as libc::c_int);
        a2b(b"%% --- title\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    } else {
        sz = cfmt.font_tb[SUBTITLEFONT as libc::c_int as usize].size;
        bskip(cfmt.subtitlespace + sz);
        str_font(SUBTITLEFONT as libc::c_int);
        a2b(
            b"%% --- titlesub\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    a2b(b" %s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char, p);
    if cfmt.titleleft != 0 {
        a2b(b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    } else {
        a2b(
            b"%.1f\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0.5f64
                * ((if cfmt.landscape != 0 { cfmt.pageheight } else { cfmt.pagewidth })
                    - cfmt.leftmargin - cfmt.rightmargin) as libc::c_double
                / cfmt.scale as libc::c_double,
        );
    }
    a2b(
        b" %.1f M \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sz as libc::c_double * 0.2f64,
    );
    p = trim_title(p, s);
    put_str(p, if cfmt.titleleft != 0 { 0 as libc::c_int } else { 1 as libc::c_int });
}
unsafe extern "C" fn write_headform(mut lwidth: libc::c_float) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut f: *mut FONTSPEC = 0 as *mut FONTSPEC;
    let mut align: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_uint = 0;
    let mut x: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    let mut xa: [libc::c_float; 3] = [0.; 3];
    let mut ya: [libc::c_float; 3] = [0.; 3];
    let mut sz: libc::c_float = 0.;
    let mut yb: [libc::c_float; 3] = [0.; 3];
    let mut inf_nb: [libc::c_char; 26] = [0; 26];
    let mut inf_s: INFO = [0 as *mut SYMBOL; 26];
    let mut inf_ft: [libc::c_char; 26] = [0; 26];
    let mut inf_sz: [libc::c_float; 26] = [0.; 26];
    let mut fmt: [libc::c_char; 64] = [0; 64];
    memset(
        inf_nb.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong,
    );
    memset(
        inf_ft.as_mut_ptr() as *mut libc::c_void,
        HISTORYFONT as libc::c_int,
        ::core::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong,
    );
    inf_ft[('A' as i32 - 'A' as i32) as usize] = INFOFONT as libc::c_int as libc::c_char;
    inf_ft[('C' as i32 - 'A' as i32)
        as usize] = COMPOSERFONT as libc::c_int as libc::c_char;
    inf_ft[('O' as i32 - 'A' as i32)
        as usize] = COMPOSERFONT as libc::c_int as libc::c_char;
    inf_ft[('P' as i32 - 'A' as i32)
        as usize] = PARTSFONT as libc::c_int as libc::c_char;
    inf_ft[('Q' as i32 - 'A' as i32)
        as usize] = TEMPOFONT as libc::c_int as libc::c_char;
    inf_ft[('R' as i32 - 'A' as i32) as usize] = INFOFONT as libc::c_int as libc::c_char;
    inf_ft[('T' as i32 - 'A' as i32)
        as usize] = TITLEFONT as libc::c_int as libc::c_char;
    inf_ft[('X' as i32 - 'A' as i32)
        as usize] = TITLEFONT as libc::c_int as libc::c_char;
    memcpy(
        inf_s.as_mut_ptr() as *mut libc::c_void,
        info.as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<INFO>() as libc::c_ulong,
    );
    memset(
        inf_sz.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[libc::c_float; 26]>() as libc::c_ulong,
    );
    inf_sz[('A' as i32 - 'A' as i32) as usize] = cfmt.infospace;
    inf_sz[('C' as i32 - 'A' as i32) as usize] = cfmt.composerspace;
    inf_sz[('O' as i32 - 'A' as i32) as usize] = cfmt.composerspace;
    inf_sz[('R' as i32 - 'A' as i32) as usize] = cfmt.infospace;
    p = cfmt.titleformat;
    j = 0 as libc::c_int as libc::c_uint;
    loop {
        while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
            p = p.offset(1);
            p;
        }
        if *p as libc::c_int == '\0' as i32 {
            break;
        }
        i = *p as libc::c_int - 'A' as i32;
        if (i as libc::c_uint) < 26 as libc::c_int as libc::c_uint {
            inf_nb[i as usize] += 1;
            inf_nb[i as usize];
            match *p.offset(1 as libc::c_int as isize) as libc::c_int {
                49 => {
                    align = 2 as libc::c_int;
                    p = p.offset(1);
                    p;
                }
                45 => {
                    align = 0 as libc::c_int;
                    p = p.offset(1);
                    p;
                }
                _ => {
                    align = 1 as libc::c_int;
                }
            }
            if (j as libc::c_ulong)
                < (::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong)
                    .wrapping_sub(4 as libc::c_int as libc::c_ulong)
            {
                let fresh42 = j;
                j = j.wrapping_add(1);
                fmt[fresh42 as usize] = i as libc::c_char;
                let fresh43 = j;
                j = j.wrapping_add(1);
                fmt[fresh43 as usize] = align as libc::c_char;
            }
        } else if *p as libc::c_int == ',' as i32 {
            if (j as libc::c_ulong)
                < (::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong)
                    .wrapping_sub(3 as libc::c_int as libc::c_ulong)
            {
                let fresh44 = j;
                j = j.wrapping_add(1);
                fmt[fresh44 as usize] = 126 as libc::c_int as libc::c_char;
            }
        } else if *p as libc::c_int == '+' as i32 {
            if j > 0 as libc::c_int as libc::c_uint
                && (fmt[j.wrapping_sub(1 as libc::c_int as libc::c_uint) as usize]
                    as libc::c_int) < 125 as libc::c_int
                && (j as libc::c_ulong)
                    < (::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong)
                        .wrapping_sub(4 as libc::c_int as libc::c_ulong)
            {
                let fresh45 = j;
                j = j.wrapping_add(1);
                fmt[fresh45 as usize] = 125 as libc::c_int as libc::c_char;
                let fresh46 = j;
                j = j.wrapping_add(1);
                fmt[fresh46 as usize] = 0 as libc::c_int as libc::c_char;
            }
        }
        p = p.offset(1);
        p;
    }
    let fresh47 = j;
    j = j.wrapping_add(1);
    fmt[fresh47 as usize] = 126 as libc::c_int as libc::c_char;
    fmt[j as usize] = 127 as libc::c_int as libc::c_char;
    ya[2 as libc::c_int as usize] = cfmt.titlespace;
    ya[1 as libc::c_int as usize] = ya[2 as libc::c_int as usize];
    ya[0 as libc::c_int as usize] = ya[1 as libc::c_int as usize];
    xa[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
    xa[1 as libc::c_int as usize] = (lwidth as libc::c_double * 0.5f64) as libc::c_float;
    xa[2 as libc::c_int as usize] = lwidth;
    p = fmt.as_mut_ptr();
    loop {
        y = 0 as libc::c_int as libc::c_float;
        yb[2 as libc::c_int as usize] = y;
        yb[1 as libc::c_int as usize] = yb[2 as libc::c_int as usize];
        yb[0 as libc::c_int as usize] = yb[1 as libc::c_int as usize];
        q = p;
        loop {
            let fresh48 = q;
            q = q.offset(1);
            i = *fresh48 as libc::c_int;
            if i >= 126 as libc::c_int {
                break;
            }
            let fresh49 = q;
            q = q.offset(1);
            align = *fresh49 as libc::c_int;
            if yb[align as usize] != 0 as libc::c_int as libc::c_float
                || i == 125 as libc::c_int
            {
                continue;
            }
            s = inf_s[i as usize];
            if s.is_null() || inf_nb[i as usize] as libc::c_int == 0 as libc::c_int {
                continue;
            }
            j = inf_ft[i as usize] as libc::c_uint;
            f = &mut *(cfmt.font_tb).as_mut_ptr().offset(j as isize) as *mut FONTSPEC;
            sz = ((*f).size as libc::c_double * 1.1f64
                + inf_sz[i as usize] as libc::c_double) as libc::c_float;
            if y < sz {
                y = sz;
            }
            yb[align as usize] = sz;
        }
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            ya[i as usize] += y - yb[i as usize];
            i += 1;
            i;
        }
        loop {
            let fresh50 = p;
            p = p.offset(1);
            i = *fresh50 as libc::c_int;
            if i >= 126 as libc::c_int {
                break;
            }
            let fresh51 = p;
            p = p.offset(1);
            align = *fresh51 as libc::c_int;
            if i == 125 as libc::c_int {
                continue;
            }
            s = inf_s[i as usize];
            if s.is_null() || inf_nb[i as usize] as libc::c_int == 0 as libc::c_int {
                continue;
            }
            j = inf_ft[i as usize] as libc::c_uint;
            str_font(j as libc::c_int);
            x = xa[align as usize];
            f = &mut *(cfmt.font_tb).as_mut_ptr().offset(j as isize) as *mut FONTSPEC;
            sz = ((*f).size as libc::c_double * 1.1f64
                + inf_sz[i as usize] as libc::c_double) as libc::c_float;
            y = ya[align as usize] + sz;
            if *((*s).text).offset(2 as libc::c_int as isize) as libc::c_int
                != '\0' as i32
            {
                if i == 'T' as i32 - 'A' as i32 {
                    if s == info[('T' as i32 - 'A' as i32) as usize] {
                        a2b(
                            b"%% --- title\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                    } else {
                        a2b(
                            b"%% --- titlesub\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                    }
                    a2b(
                        b" %s\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        &mut *((*s).text).offset(2 as libc::c_int as isize)
                            as *mut libc::c_char,
                    );
                }
                a2b(
                    b"%.1f %.1f M \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    x as libc::c_double,
                    -y as libc::c_double,
                );
            }
            if *p as libc::c_int == 125 as libc::c_int {
                p = p.offset(2 as libc::c_int as isize);
                if *p as libc::c_int == i
                    && *p.offset(1 as libc::c_int as isize) as libc::c_int == align
                    && !((*s).next).is_null()
                {
                    let mut buf: [libc::c_char; 256] = [0; 256];
                    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
                    q = (*s).text;
                    if *q.offset(1 as libc::c_int as isize) as libc::c_int == ':' as i32
                    {
                        q = q.offset(2 as libc::c_int as isize);
                    }
                    while isspace(*q as libc::c_uchar as libc::c_int) != 0 {
                        q = q.offset(1);
                        q;
                    }
                    if i == 'T' as i32 - 'A' as i32 {
                        q = trim_title(q, s);
                    }
                    strncpy(
                        buf.as_mut_ptr(),
                        q,
                        (::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    );
                    buf[(::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        as usize] = '\0' as i32 as libc::c_char;
                    j = strlen(buf.as_mut_ptr()) as libc::c_uint;
                    if (j as libc::c_ulong)
                        < (::core::mem::size_of::<[libc::c_char; 256]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    {
                        buf[j as usize] = ' ' as i32 as libc::c_char;
                        buf[j.wrapping_add(1 as libc::c_int as libc::c_uint)
                            as usize] = '\0' as i32 as libc::c_char;
                    }
                    s = (*s).next;
                    q = (*s).text;
                    if *q.offset(1 as libc::c_int as isize) as libc::c_int == ':' as i32
                    {
                        q = q.offset(2 as libc::c_int as isize);
                    }
                    while isspace(*q as libc::c_uchar as libc::c_int) != 0 {
                        q = q.offset(1);
                        q;
                    }
                    if *((*s).text).offset(0 as libc::c_int as isize) as libc::c_int
                        == 'T' as i32
                    {
                        q = trim_title(q, s);
                    }
                    r = buf.as_mut_ptr().offset(strlen(buf.as_mut_ptr()) as isize);
                    strncpy(
                        r,
                        q,
                        (buf
                            .as_mut_ptr()
                            .offset(
                                ::core::mem::size_of::<[libc::c_char; 256]>()
                                    as libc::c_ulong as isize,
                            )
                            .offset_from(r) as libc::c_long
                            - 1 as libc::c_int as libc::c_long) as libc::c_ulong,
                    );
                    tex_str(buf.as_mut_ptr());
                    str_out(tex_buf.as_mut_ptr(), align);
                    a2b(
                        b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                    inf_nb[i as usize] -= 1;
                    inf_nb[i as usize];
                    p = p.offset(2 as libc::c_int as isize);
                } else {
                    put_inf2r(s, 0 as *mut SYMBOL, align);
                }
            } else if i == 'Q' as i32 - 'A' as i32 {
                if align != 0 as libc::c_int {
                    let mut w: libc::c_float = 0.;
                    w = -tempo_width(s);
                    if align == 1 as libc::c_int {
                        w = (w as libc::c_double * 0.5f64) as libc::c_float;
                    }
                    a2b(
                        b"%.1f 0 RM \0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        w as libc::c_double,
                    );
                }
                write_tempo(s, 0 as libc::c_int, 0.75f64 as libc::c_float);
                info[('Q' as i32 - 'A' as i32) as usize] = 0 as *mut SYMBOL;
            } else {
                put_inf2r(s, 0 as *mut SYMBOL, align);
            }
            if inf_s[i as usize] == info[('T' as i32 - 'A' as i32) as usize] {
                inf_ft[i as usize] = SUBTITLEFONT as libc::c_int as libc::c_char;
                str_font(SUBTITLEFONT as libc::c_int);
                f = &mut *(cfmt.font_tb)
                    .as_mut_ptr()
                    .offset(SUBTITLEFONT as libc::c_int as isize) as *mut FONTSPEC;
                inf_sz[i as usize] = cfmt.subtitlespace;
                sz = ((*f).size as libc::c_double * 1.1f64
                    + inf_sz[i as usize] as libc::c_double) as libc::c_float;
            }
            s = (*s).next;
            if inf_nb[i as usize] as libc::c_int == 1 as libc::c_int {
                while !s.is_null() {
                    y += sz;
                    a2b(
                        b"%.1f %.1f M \0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        x as libc::c_double,
                        -y as libc::c_double,
                    );
                    put_inf2r(s, 0 as *mut SYMBOL, align);
                    s = (*s).next;
                }
            }
            inf_s[i as usize] = s;
            inf_nb[i as usize] -= 1;
            inf_nb[i as usize];
            ya[align as usize] = y;
        }
        if ya[1 as libc::c_int as usize] > ya[0 as libc::c_int as usize] {
            ya[0 as libc::c_int as usize] = ya[1 as libc::c_int as usize];
        }
        if ya[2 as libc::c_int as usize] > ya[0 as libc::c_int as usize] {
            ya[0 as libc::c_int as usize] = ya[2 as libc::c_int as usize];
        }
        if *p as libc::c_int == 127 as libc::c_int {
            bskip(ya[0 as libc::c_int as usize]);
            break;
        } else {
            ya[2 as libc::c_int as usize] = ya[0 as libc::c_int as usize];
            ya[1 as libc::c_int as usize] = ya[2 as libc::c_int as usize];
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn write_heading() {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut rhythm: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut area: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut author: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut composer: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut origin: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut lwidth: libc::c_float = 0.;
    let mut down1: libc::c_float = 0.;
    let mut down2: libc::c_float = 0.;
    lwidth = ((if cfmt.landscape != 0 { cfmt.pageheight } else { cfmt.pagewidth })
        - cfmt.leftmargin - cfmt.rightmargin) / cfmt.scale;
    if !(cfmt.titleformat).is_null()
        && *(cfmt.titleformat).offset(0 as libc::c_int as isize) as libc::c_int
            != '\0' as i32
    {
        write_headform(lwidth);
        bskip(cfmt.musicspace);
        return;
    }
    if cfmt.fields[0 as libc::c_int as usize]
        & ((1 as libc::c_int) << 'T' as i32 - 'A' as i32) as libc::c_uint != 0
    {
        s = info[('T' as i32 - 'A' as i32) as usize];
        while !s.is_null() {
            write_title(s);
            s = (*s).next;
        }
    }
    down1 = cfmt.composerspace + cfmt.font_tb[COMPOSERFONT as libc::c_int as usize].size;
    rhythm = if ((*first_voice).key.instr as libc::c_int == 1 as libc::c_int
        || (*first_voice).key.instr as libc::c_int == 2 as libc::c_int
        || pipeformat != 0) && cfmt.infoline == 0
        && cfmt.fields[0 as libc::c_int as usize]
            & ((1 as libc::c_int) << 'R' as i32 - 'A' as i32) as libc::c_uint != 0
    {
        info[('R' as i32 - 'A' as i32) as usize]
    } else {
        0 as *mut SYMBOL
    };
    if !rhythm.is_null() {
        str_font(COMPOSERFONT as libc::c_int);
        a2b(
            b"0 %.1f M \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            -cfmt.composerspace as libc::c_double,
        );
        put_inf(rhythm);
        down1 = cfmt.composerspace;
    }
    author = 0 as *mut SYMBOL;
    area = author;
    if parse.abc_vers != (2 as libc::c_int) << 16 as libc::c_int {
        area = info[('A' as i32 - 'A' as i32) as usize];
    } else {
        author = info[('A' as i32 - 'A' as i32) as usize];
    }
    composer = if cfmt.fields[0 as libc::c_int as usize]
        & ((1 as libc::c_int) << 'C' as i32 - 'A' as i32) as libc::c_uint != 0
    {
        info[('C' as i32 - 'A' as i32) as usize]
    } else {
        0 as *mut SYMBOL
    };
    origin = if cfmt.fields[0 as libc::c_int as usize]
        & ((1 as libc::c_int) << 'O' as i32 - 'A' as i32) as libc::c_uint != 0
    {
        info[('O' as i32 - 'A' as i32) as usize]
    } else {
        0 as *mut SYMBOL
    };
    if !composer.is_null() || !origin.is_null() || !author.is_null()
        || cfmt.infoline != 0
    {
        let mut xcomp: libc::c_float = 0.;
        let mut align: libc::c_int = 0;
        str_font(COMPOSERFONT as libc::c_int);
        bskip(cfmt.composerspace);
        if cfmt.aligncomposer < 0 as libc::c_int {
            xcomp = 0 as libc::c_int as libc::c_float;
            align = 0 as libc::c_int;
        } else if cfmt.aligncomposer == 0 as libc::c_int {
            xcomp = (lwidth as libc::c_double * 0.5f64) as libc::c_float;
            align = 1 as libc::c_int;
        } else {
            xcomp = lwidth;
            align = 2 as libc::c_int;
        }
        down2 = down1;
        if !author.is_null() {
            loop {
                bskip(cfmt.font_tb[COMPOSERFONT as libc::c_int as usize].size);
                down2 += cfmt.font_tb[COMPOSERFONT as libc::c_int as usize].size;
                a2b(
                    b"0 0 M \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                put_inf(author);
                author = (*author).next;
                if author.is_null() {
                    break;
                }
            }
        }
        if !composer.is_null() || !origin.is_null() {
            if cfmt.aligncomposer >= 0 as libc::c_int && down1 != down2 {
                bskip(down1 - down2);
            }
            s = composer;
            loop {
                bskip(cfmt.font_tb[COMPOSERFONT as libc::c_int as usize].size);
                a2b(
                    b"%.1f 0 M \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    xcomp as libc::c_double,
                );
                put_inf2r(
                    s,
                    if s.is_null() || ((*s).next).is_null() {
                        origin
                    } else {
                        0 as *mut SYMBOL
                    },
                    align,
                );
                if s.is_null() {
                    break;
                }
                s = (*s).next;
                if s.is_null() {
                    break;
                }
                down1 += cfmt.font_tb[COMPOSERFONT as libc::c_int as usize].size;
            }
            if down2 > down1 {
                bskip(down2 - down1);
            }
        }
        rhythm = if !rhythm.is_null() {
            0 as *mut SYMBOL
        } else {
            info[('R' as i32 - 'A' as i32) as usize]
        };
        if (!rhythm.is_null() || !area.is_null()) && cfmt.infoline != 0 {
            str_font(INFOFONT as libc::c_int);
            bskip(cfmt.font_tb[INFOFONT as libc::c_int as usize].size + cfmt.infospace);
            a2b(
                b"%.1f 0 M \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                lwidth as libc::c_double,
            );
            put_inf2r(rhythm, area, 2 as libc::c_int);
            down1
                += cfmt.font_tb[INFOFONT as libc::c_int as usize].size + cfmt.infospace;
        }
        down2 = 0 as libc::c_int as libc::c_float;
    } else {
        down2 = cfmt.composerspace;
    }
    if !(info[('P' as i32 - 'A' as i32) as usize]).is_null()
        && cfmt.fields[0 as libc::c_int as usize]
            & ((1 as libc::c_int) << 'P' as i32 - 'A' as i32) as libc::c_uint != 0
    {
        down1 = cfmt.partsspace + cfmt.font_tb[PARTSFONT as libc::c_int as usize].size
            - down1;
        if down1 > 0 as libc::c_int as libc::c_float {
            down2 += down1;
        }
        if down2 as libc::c_double > 0.01f64 {
            bskip(down2);
        }
        str_font(PARTSFONT as libc::c_int);
        a2b(b"0 0 M \0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        put_inf(info[('P' as i32 - 'A' as i32) as usize]);
        down2 = 0 as libc::c_int as libc::c_float;
    }
    bskip(down2 + cfmt.musicspace);
}
#[no_mangle]
pub unsafe extern "C" fn user_ps_add(mut s: *mut libc::c_char, mut use_0: libc::c_char) {
    let mut t: *mut u_ps = 0 as *mut u_ps;
    let mut r: *mut u_ps = 0 as *mut u_ps;
    let mut l: libc::c_int = 0;
    if *s as libc::c_int == '\0' as i32 || *s as libc::c_int == '%' as i32 {
        return;
    }
    l = strlen(s) as libc::c_int;
    if use_0 as libc::c_int == 'g' as i32 {
        t = malloc(
            (::core::mem::size_of::<u_ps>() as libc::c_ulong)
                .wrapping_sub(
                    ::core::mem::size_of::<[libc::c_char; 100000]>() as libc::c_ulong,
                )
                .wrapping_add(l as libc::c_ulong)
                .wrapping_add(6 as libc::c_int as libc::c_ulong),
        ) as *mut u_ps;
        sprintf(
            ((*t).text).as_mut_ptr(),
            b"%%svg %s\0" as *const u8 as *const libc::c_char,
            s,
        );
    } else {
        t = malloc(
            (::core::mem::size_of::<u_ps>() as libc::c_ulong)
                .wrapping_sub(
                    ::core::mem::size_of::<[libc::c_char; 100000]>() as libc::c_ulong,
                )
                .wrapping_add(l as libc::c_ulong)
                .wrapping_add(2 as libc::c_int as libc::c_ulong),
        ) as *mut u_ps;
        sprintf(
            ((*t).text).as_mut_ptr(),
            b"%c%s\0" as *const u8 as *const libc::c_char,
            use_0 as libc::c_int,
            s,
        );
    }
    (*t).next = 0 as *mut u_ps;
    r = user_ps;
    if r.is_null() {
        user_ps = t;
    } else {
        while !((*r).next).is_null() {
            r = (*r).next;
        }
        (*r).next = t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn user_ps_write() {
    let mut t: *mut u_ps = 0 as *mut u_ps;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut current_block_14: u64;
    t = user_ps;
    while !t.is_null() {
        p = ((*t).text).as_mut_ptr();
        match *p as libc::c_int {
            1 => {
                let mut f: *mut FILE = 0 as *mut FILE;
                let mut line: [libc::c_char; 512] = [0; 512];
                f = fopen(
                    p.offset(1 as libc::c_int as isize),
                    b"r\0" as *const u8 as *const libc::c_char,
                );
                if f.is_null() {
                    error(
                        1 as libc::c_int,
                        0 as *mut SYMBOL,
                        b"Cannot open PS file '%s'\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        &mut *((*t).text).as_mut_ptr().offset(1 as libc::c_int as isize)
                            as *mut libc::c_char,
                    );
                } else {
                    while !(fgets(
                        line.as_mut_ptr(),
                        ::core::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong
                            as libc::c_int,
                        f,
                    ))
                        .is_null()
                    {
                        fputs(line.as_mut_ptr(), fout);
                    }
                    fclose(f);
                }
                current_block_14 = 16668937799742929182;
            }
            37 => {
                fputs(p.offset(5 as libc::c_int as isize), fout);
                fputc('\n' as i32, fout);
                current_block_14 = 16668937799742929182;
            }
            112 => {
                current_block_14 = 5948590327928692120;
            }
            98 => {
                if svg != 0 || epsf > 1 as libc::c_int {
                    svg_write(
                        p.offset(1 as libc::c_int as isize),
                        strlen(p.offset(1 as libc::c_int as isize)) as libc::c_int,
                    );
                    current_block_14 = 16668937799742929182;
                } else {
                    current_block_14 = 5948590327928692120;
                }
            }
            115 => {
                svg_write(
                    p.offset(1 as libc::c_int as isize),
                    strlen(
                        &mut *((*t).text).as_mut_ptr().offset(1 as libc::c_int as isize),
                    ) as libc::c_int,
                );
                current_block_14 = 16668937799742929182;
            }
            _ => {
                current_block_14 = 5948590327928692120;
            }
        }
        match current_block_14 {
            5948590327928692120 => {
                fputs(p.offset(1 as libc::c_int as isize), fout);
                fputc('\n' as i32, fout);
            }
            _ => {}
        }
        t = (*t).next;
    }
}
