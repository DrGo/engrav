use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type __sFILEX;
    pub type re_guts;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn __maskrune(_: __darwin_ct_rune_t, _: libc::c_ulong) -> libc::c_int;
    static mut _DefaultRuneLocale: _RuneLocale;
    fn regexec(
        _: *const regex_t,
        _: *const libc::c_char,
        _: size_t,
        __pmatch: *mut regmatch_t,
        _: libc::c_int,
    ) -> libc::c_int;
    fn regfree(_: *mut regex_t);
    fn regcomp(_: *mut regex_t, _: *const libc::c_char, _: libc::c_int) -> libc::c_int;
    static mut parse: parse;
    fn include_file(fn_0: *mut libc::c_uchar);
    fn abc_parse(p: *mut libc::c_char, fname: *mut libc::c_char, linenum: libc::c_int);
    fn abc_eof();
    static mut tex_buf: [libc::c_char; 0];
}
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_longlong;
pub type __darwin_ct_rune_t = libc::c_int;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_wchar_t = libc::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_off_t = __int64_t;
pub type size_t = __darwin_size_t;
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
pub type regoff_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regex_t {
    pub re_magic: libc::c_int,
    pub re_nsub: size_t,
    pub re_endp: *const libc::c_char,
    pub re_g: *mut re_guts,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regmatch_t {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
}
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
pub unsafe extern "C" fn isalpha(mut _c: libc::c_int) -> libc::c_int {
    return __istype(_c, 0x100 as libc::c_long as libc::c_ulong);
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
pub unsafe extern "C" fn isxdigit(mut _c: libc::c_int) -> libc::c_int {
    return __isctype(_c, 0x10000 as libc::c_long as libc::c_ulong);
}
static mut dst: *mut libc::c_uchar = 0 as *const libc::c_uchar as *mut libc::c_uchar;
static mut offset: libc::c_int = 0;
static mut size: libc::c_int = 0;
static mut selection: *mut libc::c_uchar = 0 as *const libc::c_uchar
    as *mut libc::c_uchar;
static mut latin: libc::c_int = 0;
static mut skip: libc::c_int = 0;
static mut prefix: [libc::c_char; 4] = ['%' as i32 as libc::c_char, 0, 0, 0];
static mut state: libc::c_int = 0;
static mut grave: [libc::c_uchar; 31] = unsafe {
    *::core::mem::transmute::<
        &[u8; 31],
        &mut [libc::c_uchar; 31],
    >(
        b"A\xC3\x80E\xC3\x88I\xC3\x8CO\xC3\x92U\xC3\x99a\xC3\xA0e\xC3\xA8i\xC3\xACo\xC3\xB2u\xC3\xB9\0",
    )
};
static mut acute: [libc::c_uchar; 73] = unsafe {
    *::core::mem::transmute::<
        &[u8; 73],
        &mut [libc::c_uchar; 73],
    >(
        b"A\xC3\x81E\xC3\x89I\xC3\x8DO\xC3\x93U\xC3\x9AY\xC3\x9Da\xC3\xA1e\xC3\xA9i\xC3\xADo\xC3\xB3u\xC3\xBAy\xC3\xBDS\xC5\x9AZ\xC5\xB9s\xC5\x9Bz\xC5\xBAR\xC5\x94L\xC4\xB9C\xC4\x86N\xC5\x83r\xC5\x95l\xC4\xBAc\xC4\x87n\xC5\x84\0",
    )
};
static mut circumflex: [libc::c_uchar; 61] = unsafe {
    *::core::mem::transmute::<
        &[u8; 61],
        &mut [libc::c_uchar; 61],
    >(
        b"A\xC3\x82E\xC3\x8AI\xC3\x8EO\xC3\x94U\xC3\x9Ba\xC3\xA2e\xC3\xAAi\xC3\xAEo\xC3\xB4u\xC3\xBBH\xC4\xA4J\xC4\xB4h\xC4\xA5j\xC4\xB5C\xC4\x88G\xC4\x9CS\xC5\x9Cc\xC4\x89g\xC4\x9Ds\xC5\x9D\0",
    )
};
static mut cedilla: [libc::c_uchar; 49] = unsafe {
    *::core::mem::transmute::<
        &[u8; 49],
        &mut [libc::c_uchar; 49],
    >(
        b"C\xC3\x87c\xC3\xA7S\xC5\x9Es\xC5\x9FT\xC5\xA2t\xC5\xA3R\xC5\x96L\xC4\xBBG\xC4\xA2r\xC5\x97l\xC4\xBCg\xC4\xA3N\xC5\x85K\xC4\xB6n\xC5\x86k\xC4\xB7\0",
    )
};
static mut umlaut: [libc::c_uchar; 37] = unsafe {
    *::core::mem::transmute::<
        &[u8; 37],
        &mut [libc::c_uchar; 37],
    >(
        b"A\xC3\x84E\xC3\x8BI\xC3\x8FO\xC3\x96U\xC3\x9CY\xC5\xB8a\xC3\xA4e\xC3\xABi\xC3\xAFo\xC3\xB6u\xC3\xBCy\xC3\xBF\0",
    )
};
static mut tilde: [libc::c_uchar; 31] = unsafe {
    *::core::mem::transmute::<
        &[u8; 31],
        &mut [libc::c_uchar; 31],
    >(
        b"A\xC3\x83N\xC3\x91O\xC3\x95a\xC3\xA3n\xC3\xB1o\xC3\xB5I\xC4\xA8i\xC4\xA9U\xC5\xA8u\xC5\xA9\0",
    )
};
static mut ring: [libc::c_uchar; 16] = unsafe {
    *::core::mem::transmute::<
        &[u8; 16],
        &mut [libc::c_uchar; 16],
    >(b"A\xC3\x85a\xC3\xA5U\xC5\xAEu\xC5\xAFe\xC5\x93\0")
};
static mut macron: [libc::c_uchar; 49] = unsafe {
    *::core::mem::transmute::<
        &[u8; 49],
        &mut [libc::c_uchar; 49],
    >(
        b"A\xC4\x80D\xC4\x90E\xC4\x92H\xC4\xA6I\xC4\xAAO\xC5\x8CT\xC5\xA6U\xC5\xAAa\xC4\x81d\xC4\x91e\xC4\x93h\xC4\xA7i\xC4\xABo\xC5\x8Dt\xC5\xA7u\xC5\xAB\0",
    )
};
static mut slash: [libc::c_uchar; 19] = unsafe {
    *::core::mem::transmute::<
        &[u8; 19],
        &mut [libc::c_uchar; 19],
    >(b"O\xC3\x98o\xC3\xB8D\xC4\x90d\xC4\x91L\xC5\x81l\xC5\x82\0")
};
static mut ogonek: [libc::c_uchar; 25] = unsafe {
    *::core::mem::transmute::<
        &[u8; 25],
        &mut [libc::c_uchar; 25],
    >(b"A\xC4\x84E\xC4\x98I\xC4\xAEU\xC5\xB2a\xC4\x85e\xC4\x99i\xC4\xAFu\xC5\xB3\0")
};
static mut caron: [libc::c_uchar; 55] = unsafe {
    *::core::mem::transmute::<
        &[u8; 55],
        &mut [libc::c_uchar; 55],
    >(
        b"L\xC4\xBDS\xC5\xA0T\xC5\xA4Z\xC5\xBDl\xC4\xBEs\xC5\xA1t\xC5\xA5z\xC5\xBEC\xC4\x8CE\xC4\x9AD\xC4\x8EN\xC5\x87R\xC5\x98c\xC4\x8De\xC4\x9Bd\xC4\x8Fn\xC5\x88r\xC5\x99\0",
    )
};
static mut breve: [libc::c_uchar; 37] = unsafe {
    *::core::mem::transmute::<
        &[u8; 37],
        &mut [libc::c_uchar; 37],
    >(
        b"A\xC4\x82a\xC4\x83E\xC4\x94e\xC4\x95G\xC4\x9Eg\xC4\x9FI\xC4\xACi\xC4\xADO\xC5\x8Eo\xC5\x8FU\xC5\xACu\xC5\xAD\0",
    )
};
static mut hungumlaut: [libc::c_uchar; 13] = unsafe {
    *::core::mem::transmute::<
        &[u8; 13],
        &mut [libc::c_uchar; 13],
    >(b"O\xC5\x90U\xC5\xB0o\xC5\x91u\xC5\xB1\0")
};
static mut dot: [libc::c_uchar; 31] = unsafe {
    *::core::mem::transmute::<
        &[u8; 31],
        &mut [libc::c_uchar; 31],
    >(
        b"Z\xC5\xBBz\xC5\xBCI\xC4\xB0i\xC4\xB1C\xC4\x8Ac\xC4\x8BG\xC4\xA0g\xC4\xA1E\xC4\x96e\xC4\x97\0",
    )
};
static mut ligature: [libc::c_uchar; 53] = unsafe {
    *::core::mem::transmute::<
        &[u8; 53],
        &mut [libc::c_uchar; 53],
    >(
        b"AA\xC3\x85aa\xC3\xA5AE\xC3\x86ae\xC3\xA6cc\xC3\xA7cC\xC3\x87DH\xC3\x90dh\xC3\xB0ng\xC5\x8BOE\xC5\x92ss\xC3\x9FTH\xC3\x9Eth\xC3\xBE\0",
    )
};
static mut latin2: [libc::c_uchar; 193] = unsafe {
    *::core::mem::transmute::<
        &[u8; 193],
        &mut [libc::c_uchar; 193],
    >(
        b"\xC2\xA0\xC4\x84\xCB\x98\xC5\x81\xC2\xA4\xC4\xBD\xC5\x9A\xC2\xA7\xC2\xA8\xC5\xA0\xC5\x9E\xC5\xA4\xC5\xB9\xC2\xAD\xC5\xBD\xC5\xBB\xC2\xB0\xC4\x85\xCB\x9B\xC5\x82\xC2\xB4\xC4\xBE\xC5\x9B\xCB\x87\xC2\xB8\xC5\xA1\xC5\x9F\xC5\xA5\xC5\xBA\xCB\x9D\xC5\xBE\xC5\xBC\xC5\x94\xC3\x81\xC3\x82\xC4\x82\xC3\x84\xC4\xB9\xC4\x86\xC3\x87\xC4\x8C\xC3\x89\xC4\x98\xC3\x8B\xC4\x9A\xC3\x8D\xC3\x8E\xC4\x8E\xC4\x90\xC5\x83\xC5\x87\xC3\x93\xC3\x94\xC5\x90\xC3\x96\xC3\x97\xC5\x98\xC5\xAE\xC3\x9A\xC5\xB0\xC3\x9C\xC3\x9D\xC5\xA2\xC3\x9F\xC5\x95\xC3\xA1\xC3\xA2\xC4\x83\xC3\xA4\xC4\xBA\xC4\x87\xC3\xA7\xC4\x8D\xC3\xA9\xC4\x99\xC3\xAB\xC4\x9B\xC3\xAD\xC3\xAE\xC4\x8F\xC4\x91\xC5\x84\xC5\x88\xC3\xB3\xC3\xB4\xC5\x91\xC3\xB6\xC3\xB7\xC5\x99\xC5\xAF\xC3\xBA\xC5\xB1\xC3\xBC\xC3\xBD\xC5\xA3\xCB\x99\0",
    )
};
static mut latin3: [libc::c_uchar; 193] = unsafe {
    *::core::mem::transmute::<
        &[u8; 193],
        &mut [libc::c_uchar; 193],
    >(
        b"\xC2\xA0\xC4\xA6\xCB\x98\xC2\xA3\xC2\xA4  \xC4\xA4\xC2\xA7\xC2\xA8\xC4\xB0\xC5\x9E\xC4\x9E\xC4\xB4\xC2\xAD  \xC5\xBB\xC2\xB0\xC4\xA7\xC2\xB2\xC2\xB3\xC2\xB4\xC2\xB5\xC4\xA5\xC2\xB7\xC2\xB8\xC4\xB1\xC5\x9F\xC4\x9F\xC4\xB5\xC2\xBD  \xC5\xBC\xC3\x80\xC3\x81\xC3\x82  \xC3\x84\xC4\x8A\xC4\x88\xC3\x87\xC3\x88\xC3\x89\xC3\x8A\xC3\x8B\xC3\x8C\xC3\x8D\xC3\x8E\xC3\x8F  \xC3\x91\xC3\x92\xC3\x93\xC3\x94\xC4\xA0\xC3\x96\xC3\x97\xC4\x9C\xC3\x99\xC3\x9A\xC3\x9B\xC3\x9C\xC5\xAC\xC5\x9C\xC3\x9F\xC3\xA0\xC3\xA1\xC3\xA2  \xC3\xA4\xC4\x8B\xC4\x89\xC3\xA7\xC3\xA8\xC3\xA9\xC3\xAA\xC3\xAB\xC3\xAC\xC3\xAD\xC3\xAE\xC3\xAF  \xC3\xB1\xC3\xB2\xC3\xB3\xC3\xB4\xC4\xA1\xC3\xB6\xC3\xB7\xC4\x9D\xC3\xB9\xC3\xBA\xC3\xBB\xC3\xBC\xC5\xAD\xC5\x9D\xCB\x99\0",
    )
};
static mut latin4: [libc::c_uchar; 193] = unsafe {
    *::core::mem::transmute::<
        &[u8; 193],
        &mut [libc::c_uchar; 193],
    >(
        b"\xC2\xA0\xC4\x84\xC4\xB8\xC5\x96\xC2\xA4\xC4\xA8\xC4\xBB\xC2\xA7\xC2\xA8\xC5\xA0\xC4\x92\xC4\xA2\xC5\xA6\xC2\xAD\xC5\xBD\xC2\xAF\xC2\xB0\xC4\x85\xCB\x9B\xC5\x97\xC2\xB4\xC4\xA9\xC4\xBC\xCB\x87\xC2\xB8\xC5\xA1\xC4\x93\xC4\xA3\xC5\xA7\xC5\x8A\xC5\xBE\xC5\x8B\xC4\x80\xC3\x81\xC3\x82\xC3\x83\xC3\x84\xC3\x85\xC3\x86\xC4\xAE\xC4\x8C\xC3\x89\xC4\x98\xC3\x8B\xC4\x96\xC3\x8D\xC3\x8E\xC4\xAA\xC4\x90\xC5\x85\xC5\x8C\xC4\xB6\xC3\x94\xC3\x95\xC3\x96\xC3\x97\xC3\x98\xC5\xB2\xC3\x9A\xC3\x9B\xC3\x9C\xC5\xA8\xC5\xAA\xC3\x9F\xC4\x81\xC3\xA1\xC3\xA2\xC3\xA3\xC3\xA4\xC3\xA5\xC3\xA6\xC4\xAF\xC4\x8D\xC3\xA9\xC4\x99\xC3\xAB\xC4\x97\xC3\xAD\xC3\xAE\xC4\xAB\xC4\x91\xC5\x86\xC5\x8D\xC4\xB7\xC3\xB4\xC3\xB5\xC3\xB6\xC3\xB7\xC3\xB8\xC5\xB3\xC3\xBA\xC3\xBB\xC3\xBC\xC5\xA9\xC5\xAB\xCB\x99\0",
    )
};
static mut latin5: [libc::c_uchar; 193] = unsafe {
    *::core::mem::transmute::<
        &[u8; 193],
        &mut [libc::c_uchar; 193],
    >(
        b"\xC2\xA0\xC2\xA1\xC2\xA2\xC2\xA3\xC2\xA4\xC2\xA5\xC2\xA6\xC2\xA7\xC2\xA8\xC2\xA9\xC2\xAA\xC2\xAB\xC2\xAC\xC2\xAD\xC2\xAE\xC2\xAF\xC2\xB0\xC2\xB1\xC2\xB2\xC2\xB3\xC2\xB4\xC2\xB5\xC2\xB6\xC2\xB7\xC2\xB8\xC2\xB9\xC2\xBA\xC2\xBB\xC2\xBC\xC2\xBD\xC2\xBE\xC2\xBF\xC3\x80\xC3\x81\xC3\x82\xC3\x83\xC3\x84\xC3\x85\xC3\x86\xC3\x87\xC3\x88\xC3\x89\xC3\x8A\xC3\x8B\xC3\x8C\xC3\x8D\xC3\x8E\xC3\x8F\xC4\x9E\xC3\x91\xC3\x92\xC3\x93\xC3\x94\xC3\x95\xC3\x96\xC3\x97\xC3\x98\xC3\x99\xC3\x9A\xC3\x9B\xC3\x9C\xC4\xB0\xC5\x9E\xC3\x9F\xC3\xA0\xC3\xA1\xC3\xA2\xC3\xA3\xC3\xA4\xC3\xA5\xC3\xA6\xC3\xA7\xC3\xA8\xC3\xA9\xC3\xAA\xC3\xAB\xC3\xAC\xC3\xAD\xC3\xAE\xC3\xAF\xC4\x9F\xC3\xB1\xC3\xB2\xC3\xB3\xC3\xB4\xC3\xB5\xC3\xB6\xC3\xB7\xC3\xB8\xC3\xB9\xC3\xBA\xC3\xBB\xC3\xBC\xC4\xB1\xC5\x9F\xC3\xBF\0",
    )
};
static mut latin6: [libc::c_uchar; 194] = unsafe {
    *::core::mem::transmute::<
        &[u8; 194],
        &mut [libc::c_uchar; 194],
    >(
        b"\xC2\xA0\xC4\x84\xC4\x92\xC4\xA2\xC4\xAA\xC4\xA8\xC4\xB6\xC2\xA7\xC4\xBB\xC4\x90\xC5\xA0\xC5\xA6\xC5\xBD\xC2\xAD\xC5\xAA\xC5\x8A\xC2\xB0\xC4\x85\xC4\x93\xC4\xA3\xC4\xAB\xC4\xA9\xC4\xB7\xC2\xB7\xC4\xBC\xC4\x91\xC5\xA1\xC5\xA7\xC5\xBE\xE2\x80\x95\xC5\xAB\xC5\x8B\xC4\x80\xC3\x81\xC3\x82\xC3\x83\xC3\x84\xC3\x85\xC3\x86\xC4\xAE\xC4\x8C\xC3\x89\xC4\x98\xC3\x8B\xC4\x96\xC3\x8D\xC3\x8E\xC3\x8F\xC3\x90\xC5\x85\xC5\x8C\xC3\x93\xC3\x94\xC3\x95\xC3\x96\xC5\xA8\xC3\x98\xC5\xB2\xC3\x9A\xC3\x9B\xC3\x9C\xC3\x9D\xC3\x9E\xC3\x9F\xC4\x81\xC3\xA1\xC3\xA2\xC3\xA3\xC3\xA4\xC3\xA5\xC3\xA6\xC4\xAF\xC4\x8D\xC3\xA9\xC4\x99\xC3\xAB\xC4\x97\xC3\xAD\xC3\xAE\xC3\xAF\xC3\xB0\xC5\x86\xC5\x8D\xC3\xB3\xC3\xB4\xC3\xB5\xC3\xB6\xC5\xA9\xC3\xB8\xC5\xB3\xC3\xBA\xC3\xBB\xC3\xBC\xC3\xBD\xC3\xBE\xC4\xB8\0",
    )
};
static mut latin_tb: [*mut libc::c_uchar; 5] = unsafe {
    [
        latin2.as_ptr() as *mut _,
        latin3.as_ptr() as *mut _,
        latin4.as_ptr() as *mut _,
        latin5.as_ptr() as *mut _,
        latin6.as_ptr() as *mut _,
    ]
};
unsafe extern "C" fn txt_add(mut s: *mut libc::c_uchar, mut sz: libc::c_int) {
    if skip != 0 {
        return;
    }
    if offset + sz > size {
        size = (offset + sz + 8191 as libc::c_int) / 8192 as libc::c_int
            * 8192 as libc::c_int;
        if dst.is_null() {
            dst = malloc(size as libc::c_ulong) as *mut libc::c_uchar;
        } else {
            dst = realloc(dst as *mut libc::c_void, size as libc::c_ulong)
                as *mut libc::c_uchar;
        }
        if dst.is_null() {
            fprintf(
                __stderrp,
                b"Out of memory - abort\n\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    }
    memcpy(
        dst.offset(offset as isize) as *mut libc::c_void,
        s as *const libc::c_void,
        sz as libc::c_ulong,
    );
    offset += sz;
}
unsafe extern "C" fn txt_add_cnv(
    mut s: *mut libc::c_uchar,
    mut sz: libc::c_int,
    mut comment: libc::c_int,
) {
    let mut current_block: u64;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut c: libc::c_uchar = 0;
    let mut tmp: [libc::c_uchar; 4] = [0; 4];
    let mut in_string: libc::c_int = 0 as libc::c_int;
    p = s;
    while sz > 0 as libc::c_int {
        match *p as libc::c_int {
            34 => {
                if comment != 0 {
                    in_string = (in_string == 0) as libc::c_int;
                }
                current_block = 17392506108461345148;
            }
            37 => {
                if in_string != 0 || comment == 0 {
                    current_block = 17392506108461345148;
                } else {
                    loop {
                        p = p.offset(-1);
                        if !(p >= s) {
                            break;
                        }
                        if *p as libc::c_int != ' ' as i32
                            && *p as libc::c_int != '\t' as i32
                        {
                            break;
                        }
                    }
                    p = p.offset(1);
                    p;
                    break;
                }
            }
            92 => {
                if sz >= 4 as libc::c_int
                    && *p.offset(1 as libc::c_int as isize) as libc::c_int >= '0' as i32
                    && *p.offset(1 as libc::c_int as isize) as libc::c_int <= '3' as i32
                    && *p.offset(2 as libc::c_int as isize) as libc::c_int >= '0' as i32
                    && *p.offset(2 as libc::c_int as isize) as libc::c_int <= '7' as i32
                    && *p.offset(3 as libc::c_int as isize) as libc::c_int >= '0' as i32
                    && *p.offset(3 as libc::c_int as isize) as libc::c_int <= '7' as i32
                {
                    c = (((*p.offset(1 as libc::c_int as isize) as libc::c_int
                        - '0' as i32) << 6 as libc::c_int)
                        + ((*p.offset(2 as libc::c_int as isize) as libc::c_int
                            - '0' as i32) << 3 as libc::c_int)
                        + *p.offset(3 as libc::c_int as isize) as libc::c_int
                        - '0' as i32) as libc::c_uchar;
                    if p != s {
                        txt_add(s, p.offset_from(s) as libc::c_long as libc::c_int);
                    }
                    p = p.offset(4 as libc::c_int as isize);
                    s = p;
                    sz -= 4 as libc::c_int;
                    match c as libc::c_int {
                        1 | 129 => {
                            tmp[0 as libc::c_int
                                as usize] = 0xe2 as libc::c_int as libc::c_uchar;
                            tmp[1 as libc::c_int
                                as usize] = 0x99 as libc::c_int as libc::c_uchar;
                            tmp[2 as libc::c_int
                                as usize] = 0xaf as libc::c_int as libc::c_uchar;
                            txt_add(tmp.as_mut_ptr(), 3 as libc::c_int);
                            continue;
                        }
                        2 | 130 => {
                            tmp[0 as libc::c_int
                                as usize] = 0xe2 as libc::c_int as libc::c_uchar;
                            tmp[1 as libc::c_int
                                as usize] = 0x99 as libc::c_int as libc::c_uchar;
                            tmp[2 as libc::c_int
                                as usize] = 0xad as libc::c_int as libc::c_uchar;
                            txt_add(tmp.as_mut_ptr(), 3 as libc::c_int);
                            continue;
                        }
                        3 | 131 => {
                            tmp[0 as libc::c_int
                                as usize] = 0xe2 as libc::c_int as libc::c_uchar;
                            tmp[1 as libc::c_int
                                as usize] = 0x99 as libc::c_int as libc::c_uchar;
                            tmp[2 as libc::c_int
                                as usize] = 0xae as libc::c_int as libc::c_uchar;
                            txt_add(tmp.as_mut_ptr(), 3 as libc::c_int);
                            continue;
                        }
                        4 | 132 => {
                            tmp[0 as libc::c_int
                                as usize] = 0xf0 as libc::c_int as libc::c_uchar;
                            tmp[1 as libc::c_int
                                as usize] = 0x9d as libc::c_int as libc::c_uchar;
                            tmp[2 as libc::c_int
                                as usize] = 0x84 as libc::c_int as libc::c_uchar;
                            tmp[3 as libc::c_int
                                as usize] = 0xaa as libc::c_int as libc::c_uchar;
                            txt_add(tmp.as_mut_ptr(), 4 as libc::c_int);
                            continue;
                        }
                        5 | 133 => {
                            tmp[0 as libc::c_int
                                as usize] = 0xf0 as libc::c_int as libc::c_uchar;
                            tmp[1 as libc::c_int
                                as usize] = 0x9d as libc::c_int as libc::c_uchar;
                            tmp[2 as libc::c_int
                                as usize] = 0x84 as libc::c_int as libc::c_uchar;
                            tmp[3 as libc::c_int
                                as usize] = 0xab as libc::c_int as libc::c_uchar;
                            txt_add(tmp.as_mut_ptr(), 4 as libc::c_int);
                            continue;
                        }
                        _ => {
                            if !(c as libc::c_int >= 0x80 as libc::c_int
                                && latin > 0 as libc::c_int)
                            {
                                tmp[0 as libc::c_int as usize] = c;
                                txt_add(tmp.as_mut_ptr(), 1 as libc::c_int);
                                continue;
                            }
                        }
                    }
                } else if sz >= 6 as libc::c_int
                    && *p.offset(1 as libc::c_int as isize) as libc::c_int == 'u' as i32
                    && isxdigit(*p.offset(2 as libc::c_int as isize) as libc::c_int) != 0
                    && isxdigit(*p.offset(3 as libc::c_int as isize) as libc::c_int) != 0
                    && isxdigit(*p.offset(4 as libc::c_int as isize) as libc::c_int) != 0
                    && isxdigit(*p.offset(5 as libc::c_int as isize) as libc::c_int) != 0
                {
                    let mut i: libc::c_int = 0;
                    let mut v: libc::c_int = 0;
                    v = 0 as libc::c_int;
                    i = 2 as libc::c_int;
                    while i < 6 as libc::c_int {
                        v <<= 4 as libc::c_int;
                        c = *p.offset(i as isize);
                        if c as libc::c_int <= '9' as i32 {
                            v += c as libc::c_int - '0' as i32;
                        } else if c as libc::c_int <= 'F' as i32 {
                            v += c as libc::c_int - 'A' as i32 + 10 as libc::c_int;
                        } else {
                            v += c as libc::c_int - 'a' as i32 + 10 as libc::c_int;
                        }
                        i += 1;
                        i;
                    }
                    if p != s {
                        txt_add(s, p.offset_from(s) as libc::c_long as libc::c_int);
                    }
                    p = p.offset(6 as libc::c_int as isize);
                    sz -= 6 as libc::c_int;
                    if v & 0xdc00 as libc::c_int == 0xd800 as libc::c_int
                        && sz >= 6 as libc::c_int && *p as libc::c_int == '\\' as i32
                        && *p.offset(1 as libc::c_int as isize) as libc::c_int
                            == 'u' as i32
                        && isxdigit(*p.offset(2 as libc::c_int as isize) as libc::c_int)
                            != 0
                        && isxdigit(*p.offset(3 as libc::c_int as isize) as libc::c_int)
                            != 0
                        && isxdigit(*p.offset(4 as libc::c_int as isize) as libc::c_int)
                            != 0
                        && isxdigit(*p.offset(5 as libc::c_int as isize) as libc::c_int)
                            != 0
                    {
                        let mut v2: libc::c_int = 0;
                        v = (v - 0xd7c0 as libc::c_int) << 10 as libc::c_int;
                        v2 = 0 as libc::c_int;
                        i = 2 as libc::c_int;
                        while i < 6 as libc::c_int {
                            v2 <<= 4 as libc::c_int;
                            c = *p.offset(i as isize);
                            if c as libc::c_int <= '9' as i32 {
                                v2 += c as libc::c_int - '0' as i32;
                            } else if c as libc::c_int <= 'F' as i32 {
                                v2 += c as libc::c_int - 'A' as i32 + 10 as libc::c_int;
                            } else {
                                v2 += c as libc::c_int - 'a' as i32 + 10 as libc::c_int;
                            }
                            i += 1;
                            i;
                        }
                        v2 -= 0xdc00 as libc::c_int;
                        v += v2;
                        p = p.offset(6 as libc::c_int as isize);
                        sz -= 6 as libc::c_int;
                    }
                    s = p;
                    if v < 0x80 as libc::c_int {
                        tmp[0 as libc::c_int as usize] = v as libc::c_uchar;
                        i = 1 as libc::c_int;
                    } else if v < 0x800 as libc::c_int {
                        tmp[0 as libc::c_int
                            as usize] = (0xc0 as libc::c_int | v >> 6 as libc::c_int)
                            as libc::c_uchar;
                        tmp[1 as libc::c_int
                            as usize] = (0x80 as libc::c_int | v & 0x3f as libc::c_int)
                            as libc::c_uchar;
                        i = 2 as libc::c_int;
                    } else if v < 0x10000 as libc::c_int {
                        tmp[0 as libc::c_int
                            as usize] = (0xe0 as libc::c_int | v >> 12 as libc::c_int)
                            as libc::c_uchar;
                        tmp[1 as libc::c_int
                            as usize] = (0x80 as libc::c_int
                            | v >> 6 as libc::c_int & 0x3f as libc::c_int)
                            as libc::c_uchar;
                        tmp[2 as libc::c_int
                            as usize] = (0x80 as libc::c_int | v & 0x3f as libc::c_int)
                            as libc::c_uchar;
                        i = 3 as libc::c_int;
                    } else {
                        tmp[0 as libc::c_int
                            as usize] = (0xf0 as libc::c_int | v >> 18 as libc::c_int)
                            as libc::c_uchar;
                        tmp[1 as libc::c_int
                            as usize] = (0x80 as libc::c_int
                            | v >> 12 as libc::c_int & 0x3f as libc::c_int)
                            as libc::c_uchar;
                        tmp[2 as libc::c_int
                            as usize] = (0x80 as libc::c_int
                            | v >> 6 as libc::c_int & 0x3f as libc::c_int)
                            as libc::c_uchar;
                        tmp[3 as libc::c_int
                            as usize] = (0x80 as libc::c_int | v & 0x3f as libc::c_int)
                            as libc::c_uchar;
                        i = 4 as libc::c_int;
                    }
                    txt_add(tmp.as_mut_ptr(), i);
                    continue;
                } else {
                    if sz >= 3 as libc::c_int {
                        let mut q: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                        match *p.offset(1 as libc::c_int as isize) as libc::c_int {
                            96 => {
                                q = grave.as_mut_ptr();
                            }
                            39 => {
                                q = acute.as_mut_ptr();
                            }
                            94 => {
                                q = circumflex.as_mut_ptr();
                            }
                            44 => {
                                q = cedilla.as_mut_ptr();
                            }
                            34 => {
                                q = umlaut.as_mut_ptr();
                            }
                            126 => {
                                q = tilde.as_mut_ptr();
                            }
                            111 => {
                                q = ring.as_mut_ptr();
                            }
                            61 => {
                                q = macron.as_mut_ptr();
                            }
                            47 => {
                                q = slash.as_mut_ptr();
                            }
                            59 => {
                                q = ogonek.as_mut_ptr();
                            }
                            118 => {
                                q = caron.as_mut_ptr();
                            }
                            117 => {
                                q = breve.as_mut_ptr();
                            }
                            72 | 58 => {
                                q = hungumlaut.as_mut_ptr();
                            }
                            46 => {
                                q = dot.as_mut_ptr();
                            }
                            _ => {
                                q = ligature.as_mut_ptr();
                                while !(*q as libc::c_int
                                    == *p.offset(1 as libc::c_int as isize) as libc::c_int
                                    && *q.offset(1 as libc::c_int as isize) as libc::c_int
                                        == *p.offset(2 as libc::c_int as isize) as libc::c_int)
                                {
                                    q = q.offset(4 as libc::c_int as isize);
                                    if !(*q as libc::c_int != '\0' as i32) {
                                        break;
                                    }
                                }
                                if *q as libc::c_int != '\0' as i32 {
                                    if p != s {
                                        txt_add(s, p.offset_from(s) as libc::c_long as libc::c_int);
                                    }
                                    txt_add(
                                        q.offset(2 as libc::c_int as isize),
                                        2 as libc::c_int,
                                    );
                                    p = p.offset(3 as libc::c_int as isize);
                                    sz -= 3 as libc::c_int;
                                    s = p;
                                    continue;
                                } else {
                                    q = 0 as *mut libc::c_uchar;
                                }
                            }
                        }
                        if !q.is_null() {
                            while !(*q as libc::c_int
                                == *p.offset(2 as libc::c_int as isize) as libc::c_int)
                            {
                                q = q.offset(3 as libc::c_int as isize);
                                if !(*q as libc::c_int != '\0' as i32) {
                                    break;
                                }
                            }
                            if *q as libc::c_int != '\0' as i32 {
                                if p != s {
                                    txt_add(s, p.offset_from(s) as libc::c_long as libc::c_int);
                                }
                                txt_add(
                                    q.offset(1 as libc::c_int as isize),
                                    2 as libc::c_int,
                                );
                                p = p.offset(3 as libc::c_int as isize);
                                sz -= 3 as libc::c_int;
                                s = p;
                                continue;
                            }
                        }
                    }
                    p = p.offset(1);
                    p;
                    sz -= 1;
                    sz;
                    if sz > 0 as libc::c_int {
                        p = p.offset(1);
                        p;
                        sz -= 1;
                        sz;
                    }
                    continue;
                }
                current_block = 11893537781574601772;
            }
            _ => {
                current_block = 17392506108461345148;
            }
        }
        match current_block {
            17392506108461345148 => {
                if *p as libc::c_int >= 0x80 as libc::c_int && latin > 0 as libc::c_int {
                    if p != s {
                        txt_add(s, p.offset_from(s) as libc::c_long as libc::c_int);
                    }
                    let fresh0 = p;
                    p = p.offset(1);
                    c = *fresh0;
                    s = p;
                    sz -= 1;
                    sz;
                } else {
                    p = p.offset(1);
                    p;
                    sz -= 1;
                    sz;
                    continue;
                }
            }
            _ => {}
        }
        if (c as libc::c_int) < 0xa0 as libc::c_int || latin == 1 as libc::c_int {
            tmp[0 as libc::c_int
                as usize] = (0xc0 as libc::c_int
                | c as libc::c_int >> 6 as libc::c_int & 0x3 as libc::c_int)
                as libc::c_uchar;
            tmp[1 as libc::c_int
                as usize] = (0x80 as libc::c_int
                | c as libc::c_int & 0x3f as libc::c_int) as libc::c_uchar;
            txt_add(tmp.as_mut_ptr(), 2 as libc::c_int);
        } else {
            let mut q_0: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
            q_0 = latin_tb[(latin - 2 as libc::c_int) as usize];
            txt_add(
                q_0
                    .offset(
                        ((c as libc::c_int - 0xa0 as libc::c_int) * 2 as libc::c_int)
                            as isize,
                    ),
                2 as libc::c_int,
            );
        }
    }
    if p != s {
        txt_add(s, p.offset_from(s) as libc::c_long as libc::c_int);
    }
}
unsafe extern "C" fn txt_add_eos(
    mut fname: *mut libc::c_char,
    mut linenum: libc::c_int,
) {
    static mut eos: libc::c_uchar = '\0' as i32 as libc::c_uchar;
    if parse.abc_vers == (2 as libc::c_int) << 16 as libc::c_int
        && offset > 0 as libc::c_int
        && *dst.offset((offset - 1 as libc::c_int) as isize) as libc::c_int
            == '\\' as i32
    {
        offset -= 1;
        offset;
        return;
    }
    txt_add(&mut eos, 1 as libc::c_int);
    abc_parse(dst as *mut libc::c_char, fname, linenum);
    offset = 0 as libc::c_int;
}
unsafe extern "C" fn get_vers(mut p: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    k = 0 as libc::c_int;
    j = k;
    i = j;
    if sscanf(
        p,
        b"%d.%d.%d\0" as *const u8 as *const libc::c_char,
        &mut i as *mut libc::c_int,
        &mut j as *mut libc::c_int,
        &mut k as *mut libc::c_int,
    ) != 3 as libc::c_int
    {
        if sscanf(
            p,
            b"%d.%d\0" as *const u8 as *const libc::c_char,
            &mut i as *mut libc::c_int,
            &mut j as *mut libc::c_int,
        ) != 2 as libc::c_int
        {
            sscanf(
                p,
                b"%d\0" as *const u8 as *const libc::c_char,
                &mut i as *mut libc::c_int,
            );
        }
    }
    parse.abc_vers = (i << 16 as libc::c_int) + (j << 8 as libc::c_int) + k;
}
unsafe extern "C" fn tune_select(mut s: *mut libc::c_uchar) -> libc::c_int {
    let mut r: regex_t = regex_t {
        re_magic: 0,
        re_nsub: 0,
        re_endp: 0 as *const libc::c_char,
        re_g: 0 as *mut re_guts,
    };
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut sel: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ret: libc::c_int = 0;
    sel = selection;
    if isdigit(*sel as libc::c_int) != 0 {
        let mut tune_number: libc::c_int = 0;
        let mut cur_sel: libc::c_int = 0;
        let mut end_sel: libc::c_int = 0;
        let mut n: libc::c_int = 0;
        tune_number = strtod(
            (s as *mut libc::c_char).offset(2 as libc::c_int as isize),
            0 as *mut *mut libc::c_char,
        ) as libc::c_int;
        while !(sscanf(
            sel as *mut libc::c_char,
            b"%d%n\0" as *const u8 as *const libc::c_char,
            &mut cur_sel as *mut libc::c_int,
            &mut n as *mut libc::c_int,
        ) != 1 as libc::c_int)
        {
            sel = sel.offset(n as isize);
            if *sel as libc::c_int == '-' as i32 {
                sel = sel.offset(1);
                sel;
                if sscanf(
                    sel as *mut libc::c_char,
                    b"%d%n\0" as *const u8 as *const libc::c_char,
                    &mut end_sel as *mut libc::c_int,
                    &mut n as *mut libc::c_int,
                ) != 1 as libc::c_int
                {
                    end_sel = (!(0 as libc::c_uint) >> 1 as libc::c_int) as libc::c_int;
                } else {
                    sel = sel.offset(n as isize);
                }
            } else {
                end_sel = cur_sel;
            }
            if tune_number >= cur_sel && tune_number <= end_sel {
                return 1 as libc::c_int;
            }
            if *sel as libc::c_int != ',' as i32 {
                break;
            }
            sel = sel.offset(1);
            sel;
        }
        if *sel as libc::c_int == '\0' as i32 {
            return 0 as libc::c_int;
        }
    }
    p = s.offset(2 as libc::c_int as isize);
    loop {
        match *p as libc::c_int {
            0 => return 0 as libc::c_int,
            10 | 13 => {
                if !(*p.offset(1 as libc::c_int as isize) as libc::c_int != 'K' as i32
                    || *p.offset(2 as libc::c_int as isize) as libc::c_int != ':' as i32)
                {
                    p = p.offset(3 as libc::c_int as isize);
                    while *p as libc::c_int != '\n' as i32
                        && *p as libc::c_int != '\r' as i32
                        && *p as libc::c_int != '\0' as i32
                    {
                        p = p.offset(1);
                        p;
                    }
                    if *p as libc::c_int != '\0' as i32 {
                        p = p.offset(1);
                        p;
                    }
                    break;
                }
            }
            _ => {}
        }
        p = p.offset(1);
        p;
    }
    ret = p.offset_from(s) as libc::c_long as libc::c_int;
    if ret >= 512 as libc::c_int - 1 as libc::c_int {
        fprintf(
            __stderrp,
            b"Tune header too big for %%%%select\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    memcpy(
        tex_buf.as_mut_ptr() as *mut libc::c_void,
        s as *const libc::c_void,
        ret as libc::c_ulong,
    );
    *tex_buf.as_mut_ptr().offset(ret as isize) = '\0' as i32 as libc::c_char;
    ret = regcomp(
        &mut r,
        sel as *mut libc::c_char,
        0o1 as libc::c_int | 0o10 as libc::c_int | 0o4 as libc::c_int,
    );
    if ret != 0 {
        return 0 as libc::c_int;
    }
    ret = regexec(
        &mut r,
        tex_buf.as_mut_ptr(),
        0 as libc::c_int as size_t,
        0 as *mut regmatch_t,
        0 as libc::c_int,
    );
    regfree(&mut r);
    return (ret == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn frontend(
    mut s: *mut libc::c_uchar,
    mut ftype: libc::c_int,
    mut fname: *mut libc::c_char,
    mut linenum: libc::c_int,
) {
    let mut current_block: u64;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut q: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut c: libc::c_uchar = 0;
    let mut begin_end: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut sep: libc::c_uchar = 0;
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut str_cnv_p: libc::c_int = 0;
    let mut histo: libc::c_int = 0;
    let mut end_len: libc::c_int = 0;
    let mut prefix_sav: [libc::c_char; 4] = [0; 4];
    let mut latin_sav: libc::c_int = 0 as libc::c_int;
    begin_end = 0 as *mut libc::c_uchar;
    end_len = 0 as libc::c_int;
    histo = 0 as libc::c_int;
    if ftype == 0 as libc::c_int
        && strncmp(
            s as *mut libc::c_char,
            b"%abc-\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        get_vers((s as *mut libc::c_char).offset(5 as libc::c_int as isize));
        while *s as libc::c_int != '\0' as i32 && *s as libc::c_int != '\r' as i32
            && *s as libc::c_int != '\n' as i32
        {
            s = s.offset(1);
            s;
        }
        if *s as libc::c_int != '\0' as i32 {
            s = s.offset(1);
            s;
            if *s.offset(-(1 as libc::c_int) as isize) as libc::c_int == '\r' as i32
                && *s as libc::c_int == '\n' as i32
            {
                s = s.offset(1);
                s;
            }
        }
        linenum += 1;
        linenum;
    }
    if ftype == 0 as libc::c_int
        && parse.abc_vers
            >= (2 as libc::c_int) << 16 as libc::c_int
                | (1 as libc::c_int) << 8 as libc::c_int
    {
        latin = 0 as libc::c_int;
    } else {
        p = s;
        while *p as libc::c_int != '\0' as i32 {
            c = *p;
            if c as libc::c_int == '\\' as i32 {
                if !(isdigit(*p.offset(1 as libc::c_int as isize) as libc::c_int) == 0) {
                    if !((*p.offset(1 as libc::c_int as isize) as libc::c_int
                        == '0' as i32
                        || *p.offset(1 as libc::c_int as isize) as libc::c_int
                            == '2' as i32)
                        && *p.offset(2 as libc::c_int as isize) as libc::c_int
                            == '0' as i32)
                    {
                        latin = 1 as libc::c_int;
                        break;
                    }
                }
            } else if !((c as libc::c_int) < 0x80 as libc::c_int) {
                if c as libc::c_int >= 0xc2 as libc::c_int {
                    if *p.offset(1 as libc::c_int as isize) as libc::c_int
                        & 0xc0 as libc::c_int == 0x80 as libc::c_int
                    {
                        latin = 0 as libc::c_int;
                        break;
                    }
                }
                latin = 1 as libc::c_int;
                break;
            }
            p = p.offset(1);
            p;
        }
    }
    latin_sav = latin;
    skip = 0 as libc::c_int;
    while *s as libc::c_int != '\0' as i32 {
        str_cnv_p = 0 as libc::c_int;
        p = s;
        while *p as libc::c_int != '\0' as i32 && *p as libc::c_int != '\r' as i32
            && *p as libc::c_int != '\n' as i32
        {
            if *p as libc::c_int == '\\' as i32 || *p as libc::c_int == '%' as i32
                || latin > 0 as libc::c_int && *p as libc::c_int >= 0x80 as libc::c_int
            {
                str_cnv_p = 1 as libc::c_int;
            }
            p = p.offset(1);
            p;
        }
        l = p.offset_from(s) as libc::c_long as libc::c_int;
        if *p as libc::c_int != '\0' as i32 {
            p = p.offset(1);
            p;
            if *p.offset(-(1 as libc::c_int) as isize) as libc::c_int == '\r' as i32
                && *p as libc::c_int == '\n' as i32
            {
                p = p.offset(1);
                p;
            }
        }
        linenum += 1;
        linenum;
        if skip != 0 {
            if l != 0 as libc::c_int {
                current_block = 1506177810756849029;
            } else {
                skip = 0 as libc::c_int;
                current_block = 12997042908615822766;
            }
        } else {
            current_block = 12997042908615822766;
        }
        match current_block {
            12997042908615822766 => {
                if !begin_end.is_null() {
                    if ftype == 1 as libc::c_int {
                        if strncmp(
                            s as *mut libc::c_char,
                            b"end\0" as *const u8 as *const libc::c_char,
                            3 as libc::c_int as libc::c_ulong,
                        ) == 0 as libc::c_int
                            && strncmp(
                                (s as *mut libc::c_char).offset(3 as libc::c_int as isize),
                                begin_end as *mut libc::c_char,
                                end_len as libc::c_ulong,
                            ) == 0 as libc::c_int
                        {
                            begin_end = 0 as *mut libc::c_uchar;
                            current_block = 566305580346490864;
                        } else if *s as libc::c_int == '%' as i32 {
                            current_block = 1506177810756849029;
                        } else {
                            current_block = 6407910375339683483;
                        }
                    } else {
                        if *s as libc::c_int == '%' as i32
                            && !(strchr(
                                prefix.as_mut_ptr(),
                                *s.offset(1 as libc::c_int as isize) as libc::c_int,
                            ))
                                .is_null()
                        {
                            q = s.offset(2 as libc::c_int as isize);
                            while *q as libc::c_int == ' ' as i32
                                || *q as libc::c_int == '\t' as i32
                            {
                                q = q.offset(1);
                                q;
                            }
                            if strncmp(
                                q as *mut libc::c_char,
                                b"end\0" as *const u8 as *const libc::c_char,
                                3 as libc::c_int as libc::c_ulong,
                            ) == 0 as libc::c_int
                                && strncmp(
                                    (q as *mut libc::c_char).offset(3 as libc::c_int as isize),
                                    begin_end as *mut libc::c_char,
                                    end_len as libc::c_ulong,
                                ) == 0 as libc::c_int
                            {
                                begin_end = 0 as *mut libc::c_uchar;
                                current_block = 566305580346490864;
                            } else {
                                current_block = 7018308795614528254;
                            }
                        } else {
                            current_block = 7018308795614528254;
                        }
                        match current_block {
                            566305580346490864 => {}
                            _ => {
                                if *s as libc::c_int == '%' as i32 {
                                    if (strchr(
                                        prefix.as_mut_ptr(),
                                        *s.offset(1 as libc::c_int as isize) as libc::c_int,
                                    ))
                                        .is_null()
                                    {
                                        current_block = 1506177810756849029;
                                    } else {
                                        s = s.offset(2 as libc::c_int as isize);
                                        l -= 2 as libc::c_int;
                                        current_block = 6407910375339683483;
                                    }
                                } else {
                                    current_block = 6407910375339683483;
                                }
                            }
                        }
                    }
                } else {
                    while l > 0 as libc::c_int
                        && isspace(
                            *s.offset((l - 1 as libc::c_int) as isize) as libc::c_int,
                        ) != 0
                    {
                        l -= 1;
                        l;
                    }
                    if l == 0 as libc::c_int {
                        if ftype == 1 as libc::c_int {
                            current_block = 566305580346490864;
                        } else {
                            match state {
                                1 => {
                                    current_block = 1616443531237277343;
                                    match current_block {
                                        1616443531237277343 => {
                                            fprintf(
                                                __stderrp,
                                                b"Line %d: Empty line in tune header - K:C added\n\0"
                                                    as *const u8 as *const libc::c_char,
                                                linenum,
                                            );
                                            txt_add(
                                                b"K:C\0" as *const u8 as *const libc::c_char
                                                    as *mut libc::c_uchar,
                                                3 as libc::c_int,
                                            );
                                            txt_add_eos(fname, linenum);
                                        }
                                        _ => {}
                                    }
                                    state = 0 as libc::c_int;
                                    strcpy(prefix.as_mut_ptr(), prefix_sav.as_mut_ptr());
                                    latin = latin_sav;
                                    current_block = 566305580346490864;
                                }
                                2 => {
                                    current_block = 10815138721735271107;
                                    match current_block {
                                        1616443531237277343 => {
                                            fprintf(
                                                __stderrp,
                                                b"Line %d: Empty line in tune header - K:C added\n\0"
                                                    as *const u8 as *const libc::c_char,
                                                linenum,
                                            );
                                            txt_add(
                                                b"K:C\0" as *const u8 as *const libc::c_char
                                                    as *mut libc::c_uchar,
                                                3 as libc::c_int,
                                            );
                                            txt_add_eos(fname, linenum);
                                        }
                                        _ => {}
                                    }
                                    state = 0 as libc::c_int;
                                    strcpy(prefix.as_mut_ptr(), prefix_sav.as_mut_ptr());
                                    latin = latin_sav;
                                    current_block = 566305580346490864;
                                }
                                _ => {
                                    current_block = 1506177810756849029;
                                }
                            }
                        }
                    } else {
                        if histo != 0 {
                            if *s.offset(1 as libc::c_int as isize) as libc::c_int
                                == ':' as i32 && isalpha(*s as libc::c_int) != 0
                                || *s as libc::c_int == '%' as i32
                                    && !(strchr(
                                        prefix.as_mut_ptr(),
                                        *s.offset(1 as libc::c_int as isize) as libc::c_int,
                                    ))
                                        .is_null()
                            {
                                histo = 0 as libc::c_int;
                                current_block = 2168227384378665163;
                            } else {
                                if *s as libc::c_int != '+' as i32
                                    || *s.offset(1 as libc::c_int as isize) as libc::c_int
                                        != ':' as i32
                                {
                                    txt_add(
                                        b"+:\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_uchar,
                                        2 as libc::c_int,
                                    );
                                }
                                current_block = 6407910375339683483;
                            }
                        } else {
                            current_block = 2168227384378665163;
                        }
                        match current_block {
                            6407910375339683483 => {}
                            _ => {
                                if *s as libc::c_int == ' ' as i32
                                    || *s as libc::c_int == '\t' as i32
                                {
                                    q = s;
                                    loop {
                                        q = q.offset(1);
                                        q;
                                        if !(*q as libc::c_int == ' ' as i32
                                            || *q as libc::c_int == '\t' as i32)
                                        {
                                            break;
                                        }
                                    }
                                    if *q as libc::c_int == '%' as i32 {
                                        current_block = 1506177810756849029;
                                    } else {
                                        current_block = 2754258178208450300;
                                    }
                                } else {
                                    current_block = 2754258178208450300;
                                }
                                match current_block {
                                    1506177810756849029 => {}
                                    _ => {
                                        if ftype == 2 as libc::c_int {
                                            if *s as libc::c_int == '%' as i32 {
                                                current_block = 1506177810756849029;
                                            } else {
                                                current_block = 6407910375339683483;
                                            }
                                        } else {
                                            if ftype == 1 as libc::c_int {
                                                if *s as libc::c_int == '%' as i32 {
                                                    current_block = 1506177810756849029;
                                                } else {
                                                    current_block = 8785488209453651919;
                                                }
                                            } else if *s as libc::c_int == 'I' as i32
                                                && *s.offset(1 as libc::c_int as isize) as libc::c_int
                                                    == ':' as i32
                                            {
                                                s = s.offset(2 as libc::c_int as isize);
                                                l -= 2 as libc::c_int;
                                                while *s as libc::c_int == ' ' as i32
                                                    || *s as libc::c_int == '\t' as i32
                                                {
                                                    s = s.offset(1);
                                                    s;
                                                    l -= 1;
                                                    l;
                                                }
                                                if l <= 0 as libc::c_int {
                                                    current_block = 1506177810756849029;
                                                } else {
                                                    txt_add(
                                                        b"%%\0" as *const u8 as *const libc::c_char
                                                            as *mut libc::c_uchar,
                                                        2 as libc::c_int,
                                                    );
                                                    current_block = 11421240547085002608;
                                                }
                                            } else {
                                                if *s as libc::c_int == '%' as i32 {
                                                    if (strchr(
                                                        prefix.as_mut_ptr(),
                                                        *s.offset(1 as libc::c_int as isize) as libc::c_int,
                                                    ))
                                                        .is_null()
                                                    {
                                                        current_block = 1506177810756849029;
                                                    } else {
                                                        s = s.offset(2 as libc::c_int as isize);
                                                        l -= 2 as libc::c_int;
                                                        if strncmp(
                                                            s as *mut libc::c_char,
                                                            b"abc \0" as *const u8 as *const libc::c_char,
                                                            4 as libc::c_int as libc::c_ulong,
                                                        ) == 0 as libc::c_int
                                                        {
                                                            s = s.offset(4 as libc::c_int as isize);
                                                            l -= 4 as libc::c_int;
                                                            current_block = 13158970981314897405;
                                                        } else if strncmp(
                                                            s as *mut libc::c_char,
                                                            b"abcm2ps \0" as *const u8 as *const libc::c_char,
                                                            8 as libc::c_int as libc::c_ulong,
                                                        ) == 0 as libc::c_int
                                                            || strncmp(
                                                                s as *mut libc::c_char,
                                                                b"ss-pref \0" as *const u8 as *const libc::c_char,
                                                                8 as libc::c_int as libc::c_ulong,
                                                            ) == 0 as libc::c_int
                                                        {
                                                            s = s.offset(8 as libc::c_int as isize);
                                                            l -= 8 as libc::c_int;
                                                            while *s as libc::c_int == ' ' as i32
                                                                || *s as libc::c_int == '\t' as i32
                                                            {
                                                                s = s.offset(1);
                                                                s;
                                                                l -= 1;
                                                                l;
                                                            }
                                                            i = 0 as libc::c_int;
                                                            while (i as libc::c_ulong)
                                                                < (::core::mem::size_of::<[libc::c_char; 4]>()
                                                                    as libc::c_ulong)
                                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                            {
                                                                if *s as libc::c_int == ' ' as i32
                                                                    || *s as libc::c_int == '\t' as i32
                                                                    || {
                                                                        l -= 1;
                                                                        l < 0 as libc::c_int
                                                                    }
                                                                {
                                                                    break;
                                                                }
                                                                let fresh1 = s;
                                                                s = s.offset(1);
                                                                prefix[i as usize] = *fresh1 as libc::c_char;
                                                                i += 1;
                                                                i;
                                                            }
                                                            if i == 0 as libc::c_int {
                                                                let fresh2 = i;
                                                                i = i + 1;
                                                                prefix[fresh2 as usize] = '%' as i32 as libc::c_char;
                                                            }
                                                            prefix[i as usize] = '\0' as i32 as libc::c_char;
                                                            current_block = 1506177810756849029;
                                                        } else if strncmp(
                                                            s as *mut libc::c_char,
                                                            b"abc-version \0" as *const u8 as *const libc::c_char,
                                                            12 as libc::c_int as libc::c_ulong,
                                                        ) == 0 as libc::c_int
                                                        {
                                                            get_vers(
                                                                (s as *mut libc::c_char).offset(12 as libc::c_int as isize),
                                                            );
                                                            current_block = 1506177810756849029;
                                                        } else {
                                                            current_block = 8785488209453651919;
                                                        }
                                                    }
                                                } else {
                                                    current_block = 13158970981314897405;
                                                }
                                                match current_block {
                                                    1506177810756849029 => {}
                                                    8785488209453651919 => {}
                                                    _ => {
                                                        if *s.offset(1 as libc::c_int as isize) as libc::c_int
                                                            == ':' as i32
                                                            && (isalpha(*s as libc::c_int) != 0
                                                                || *s as libc::c_int == '+' as i32)
                                                        {
                                                            c = *s;
                                                            match c as libc::c_int {
                                                                73 => {
                                                                    s = s.offset(2 as libc::c_int as isize);
                                                                    l -= 2 as libc::c_int;
                                                                    current_block = 8785488209453651919;
                                                                }
                                                                88 => {
                                                                    match state {
                                                                        1 => {
                                                                            fprintf(
                                                                                __stderrp,
                                                                                b"Line %d: X: found in tune header - K:C added\n\0"
                                                                                    as *const u8 as *const libc::c_char,
                                                                                linenum,
                                                                            );
                                                                            txt_add(
                                                                                b"K:C\0" as *const u8 as *const libc::c_char
                                                                                    as *mut libc::c_uchar,
                                                                                3 as libc::c_int,
                                                                            );
                                                                            txt_add_eos(fname, linenum);
                                                                            txt_add_eos(fname, linenum);
                                                                        }
                                                                        2 => {
                                                                            txt_add_eos(fname, linenum);
                                                                        }
                                                                        _ => {}
                                                                    }
                                                                    if !selection.is_null() {
                                                                        skip = (tune_select(s) == 0) as libc::c_int;
                                                                        if skip != 0 {
                                                                            current_block = 1506177810756849029;
                                                                        } else {
                                                                            current_block = 5441543464777196221;
                                                                        }
                                                                    } else {
                                                                        current_block = 5441543464777196221;
                                                                    }
                                                                    match current_block {
                                                                        1506177810756849029 => {}
                                                                        _ => {
                                                                            state = 1 as libc::c_int;
                                                                            strcpy(prefix_sav.as_mut_ptr(), prefix.as_mut_ptr());
                                                                            latin_sav = latin;
                                                                            current_block = 12788783625999190409;
                                                                        }
                                                                    }
                                                                }
                                                                85 => {
                                                                    current_block = 12788783625999190409;
                                                                }
                                                                72 => {
                                                                    histo = 1 as libc::c_int;
                                                                    current_block = 12788783625999190409;
                                                                }
                                                                _ => {
                                                                    if state == 0 as libc::c_int
                                                                        && !(strchr(
                                                                            b"dKPQsVWw\0" as *const u8 as *const libc::c_char,
                                                                            *s as libc::c_int,
                                                                        ))
                                                                            .is_null()
                                                                    {
                                                                        current_block = 1506177810756849029;
                                                                    } else {
                                                                        if *s as libc::c_int == 'K' as i32 {
                                                                            state = 2 as libc::c_int;
                                                                        }
                                                                        current_block = 12788783625999190409;
                                                                    }
                                                                }
                                                            }
                                                            match current_block {
                                                                8785488209453651919 => {}
                                                                1506177810756849029 => {}
                                                                _ => {
                                                                    txt_add(s, 2 as libc::c_int);
                                                                    s = s.offset(2 as libc::c_int as isize);
                                                                    l -= 2 as libc::c_int;
                                                                    while *s as libc::c_int == ' ' as i32
                                                                        || *s as libc::c_int == '\t' as i32
                                                                    {
                                                                        s = s.offset(1);
                                                                        s;
                                                                        l -= 1;
                                                                        l;
                                                                    }
                                                                    str_cnv_p = 1 as libc::c_int;
                                                                    current_block = 6407910375339683483;
                                                                }
                                                            }
                                                        } else if state == 0 as libc::c_int {
                                                            current_block = 1506177810756849029;
                                                        } else {
                                                            current_block = 6407910375339683483;
                                                        }
                                                    }
                                                }
                                            }
                                            match current_block {
                                                1506177810756849029 => {}
                                                6407910375339683483 => {}
                                                _ => {
                                                    match current_block {
                                                        8785488209453651919 => {
                                                            while *s as libc::c_int == ' ' as i32
                                                                || *s as libc::c_int == '\t' as i32
                                                            {
                                                                s = s.offset(1);
                                                                s;
                                                                l -= 1;
                                                                l;
                                                            }
                                                            if l <= 0 as libc::c_int {
                                                                current_block = 1506177810756849029;
                                                            } else {
                                                                txt_add(
                                                                    b"%%\0" as *const u8 as *const libc::c_char
                                                                        as *mut libc::c_uchar,
                                                                    2 as libc::c_int,
                                                                );
                                                                if strncmp(
                                                                    s as *mut libc::c_char,
                                                                    b"begin\0" as *const u8 as *const libc::c_char,
                                                                    5 as libc::c_int as libc::c_ulong,
                                                                ) == 0 as libc::c_int
                                                                {
                                                                    begin_end = s.offset(5 as libc::c_int as isize);
                                                                    q = begin_end;
                                                                    while isspace(*q as libc::c_int) == 0 {
                                                                        q = q.offset(1);
                                                                        q;
                                                                    }
                                                                    end_len = q.offset_from(begin_end) as libc::c_long
                                                                        as libc::c_int;
                                                                    current_block = 6407910375339683483;
                                                                } else {
                                                                    current_block = 11421240547085002608;
                                                                }
                                                            }
                                                        }
                                                        _ => {}
                                                    }
                                                    match current_block {
                                                        1506177810756849029 => {}
                                                        6407910375339683483 => {}
                                                        _ => {
                                                            if strncmp(
                                                                s as *mut libc::c_char,
                                                                b"encoding \0" as *const u8 as *const libc::c_char,
                                                                9 as libc::c_int as libc::c_ulong,
                                                            ) == 0 as libc::c_int
                                                                || strncmp(
                                                                    s as *mut libc::c_char,
                                                                    b"abc-charset \0" as *const u8 as *const libc::c_char,
                                                                    12 as libc::c_int as libc::c_ulong,
                                                                ) == 0 as libc::c_int
                                                            {
                                                                if *s as libc::c_int == 'e' as i32 {
                                                                    q = s.offset(9 as libc::c_int as isize);
                                                                } else {
                                                                    q = s.offset(12 as libc::c_int as isize);
                                                                }
                                                                while *q as libc::c_int == ' ' as i32
                                                                    || *q as libc::c_int == '\t' as i32
                                                                {
                                                                    q = q.offset(1);
                                                                    q;
                                                                }
                                                                if strncasecmp(
                                                                    q as *mut libc::c_char,
                                                                    b"latin\0" as *const u8 as *const libc::c_char,
                                                                    5 as libc::c_int as libc::c_ulong,
                                                                ) == 0 as libc::c_int
                                                                {
                                                                    q = q.offset(5 as libc::c_int as isize);
                                                                    current_block = 12608488225262500095;
                                                                } else if strncasecmp(
                                                                    q as *mut libc::c_char,
                                                                    b"iso-8859-\0" as *const u8 as *const libc::c_char,
                                                                    9 as libc::c_int as libc::c_ulong,
                                                                ) == 0 as libc::c_int
                                                                {
                                                                    q = q.offset(9 as libc::c_int as isize);
                                                                    current_block = 12608488225262500095;
                                                                } else if strncasecmp(
                                                                    q as *mut libc::c_char,
                                                                    b"utf-8\0" as *const u8 as *const libc::c_char,
                                                                    5 as libc::c_int as libc::c_ulong,
                                                                ) == 0 as libc::c_int
                                                                    || strncasecmp(
                                                                        q as *mut libc::c_char,
                                                                        b"native\0" as *const u8 as *const libc::c_char,
                                                                        6 as libc::c_int as libc::c_ulong,
                                                                    ) == 0 as libc::c_int
                                                                {
                                                                    latin = 0 as libc::c_int;
                                                                    current_block = 6407910375339683483;
                                                                } else if isdigit(*q as libc::c_int) == 0 {
                                                                    current_block = 6407910375339683483;
                                                                } else {
                                                                    current_block = 12608488225262500095;
                                                                }
                                                                match current_block {
                                                                    6407910375339683483 => {}
                                                                    _ => {
                                                                        match *q as libc::c_int {
                                                                            49 => {
                                                                                if *q.offset(1 as libc::c_int as isize) as libc::c_int
                                                                                    == '0' as i32
                                                                                {
                                                                                    latin = 6 as libc::c_int;
                                                                                } else {
                                                                                    latin = 1 as libc::c_int;
                                                                                }
                                                                            }
                                                                            50 => {
                                                                                latin = 2 as libc::c_int;
                                                                            }
                                                                            51 => {
                                                                                latin = 3 as libc::c_int;
                                                                            }
                                                                            52 => {
                                                                                latin = 4 as libc::c_int;
                                                                            }
                                                                            53 => {
                                                                                if *q.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                                                                    != '-' as i32
                                                                                {
                                                                                    latin = 5 as libc::c_int;
                                                                                }
                                                                            }
                                                                            54 => {
                                                                                if *q.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                                                                    != '-' as i32
                                                                                {
                                                                                    latin = 6 as libc::c_int;
                                                                                }
                                                                            }
                                                                            57 => {
                                                                                latin = 5 as libc::c_int;
                                                                            }
                                                                            _ => {}
                                                                        }
                                                                        current_block = 6407910375339683483;
                                                                    }
                                                                }
                                                            } else if strncmp(
                                                                s as *mut libc::c_char,
                                                                b"format \0" as *const u8 as *const libc::c_char,
                                                                7 as libc::c_int as libc::c_ulong,
                                                            ) == 0 as libc::c_int
                                                                || strncmp(
                                                                    s as *mut libc::c_char,
                                                                    b"abc-include \0" as *const u8 as *const libc::c_char,
                                                                    12 as libc::c_int as libc::c_ulong,
                                                                ) == 0 as libc::c_int
                                                            {
                                                                let mut skip_sav: libc::c_int = 0;
                                                                if *s as libc::c_int == 'f' as i32 {
                                                                    s = s.offset(7 as libc::c_int as isize);
                                                                } else {
                                                                    s = s.offset(12 as libc::c_int as isize);
                                                                }
                                                                while *s as libc::c_int == ' ' as i32
                                                                    || *s as libc::c_int == '\t' as i32
                                                                {
                                                                    s = s.offset(1);
                                                                    s;
                                                                }
                                                                q = s;
                                                                while *q as libc::c_int != '\0' as i32
                                                                    && *q as libc::c_int != '%' as i32
                                                                    && *q as libc::c_int != '\n' as i32
                                                                    && *q as libc::c_int != '\r' as i32
                                                                {
                                                                    q = q.offset(1);
                                                                    q;
                                                                }
                                                                while *q.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                                                    == ' ' as i32
                                                                {
                                                                    q = q.offset(-1);
                                                                    q;
                                                                }
                                                                sep = *q;
                                                                *q = '\0' as i32 as libc::c_uchar;
                                                                skip_sav = skip;
                                                                offset = 0 as libc::c_int;
                                                                include_file(s);
                                                                skip = skip_sav;
                                                                *q = sep;
                                                                current_block = 1506177810756849029;
                                                            } else if strncmp(
                                                                s as *mut libc::c_char,
                                                                b"select\0" as *const u8 as *const libc::c_char,
                                                                6 as libc::c_int as libc::c_ulong,
                                                            ) == 0 as libc::c_int
                                                            {
                                                                s = s.offset(6 as libc::c_int as isize);
                                                                if *s as libc::c_int == '\n' as i32 {
                                                                    q = s;
                                                                    current_block = 1230153937815237705;
                                                                } else if *s as libc::c_int != ' ' as i32
                                                                    && *s as libc::c_int != '\t' as i32
                                                                {
                                                                    current_block = 6407910375339683483;
                                                                } else {
                                                                    while *s as libc::c_int == ' ' as i32
                                                                        || *s as libc::c_int == '\t' as i32
                                                                    {
                                                                        s = s.offset(1);
                                                                        s;
                                                                    }
                                                                    q = s;
                                                                    while *q as libc::c_int != '\0' as i32
                                                                        && *q as libc::c_int != '%' as i32
                                                                        && *q as libc::c_int != '\n' as i32
                                                                        && *q as libc::c_int != '\r' as i32
                                                                    {
                                                                        q = q.offset(1);
                                                                        q;
                                                                    }
                                                                    while *q.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                                                        == ' ' as i32
                                                                        || *q.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                                                            == '\t' as i32
                                                                    {
                                                                        q = q.offset(-1);
                                                                        q;
                                                                    }
                                                                    if strncmp(
                                                                        (q as *mut libc::c_char)
                                                                            .offset(-(5 as libc::c_int as isize)),
                                                                        b" lock\0" as *const u8 as *const libc::c_char,
                                                                        5 as libc::c_int as libc::c_ulong,
                                                                    ) == 0 as libc::c_int
                                                                    {
                                                                        q = q.offset(-(5 as libc::c_int as isize));
                                                                    }
                                                                    current_block = 1230153937815237705;
                                                                }
                                                                match current_block {
                                                                    6407910375339683483 => {}
                                                                    _ => {
                                                                        if !selection.is_null() {
                                                                            free(selection as *mut libc::c_void);
                                                                            selection = 0 as *mut libc::c_uchar;
                                                                        }
                                                                        if q != s {
                                                                            sep = *q;
                                                                            *q = '\0' as i32 as libc::c_uchar;
                                                                            selection = strdup(s as *mut libc::c_char)
                                                                                as *mut libc::c_uchar;
                                                                            *q = sep;
                                                                        }
                                                                        offset = 0 as libc::c_int;
                                                                        current_block = 1506177810756849029;
                                                                    }
                                                                }
                                                            } else {
                                                                current_block = 6407910375339683483;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                match current_block {
                    1506177810756849029 => {}
                    _ => {
                        match current_block {
                            6407910375339683483 => {
                                if str_cnv_p != 0 {
                                    txt_add_cnv(s, l, begin_end.is_null() as libc::c_int);
                                } else {
                                    txt_add(s, l);
                                }
                                if !begin_end.is_null() {
                                    txt_add(
                                        b"\n\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_uchar,
                                        1 as libc::c_int,
                                    );
                                    current_block = 1506177810756849029;
                                } else {
                                    current_block = 566305580346490864;
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            1506177810756849029 => {}
                            _ => {
                                txt_add_eos(fname, linenum);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        s = p;
    }
    if !begin_end.is_null() {
        fprintf(
            __stderrp,
            b"Line %d: No %%%%end after %%%%begin\n\0" as *const u8
                as *const libc::c_char,
            linenum,
        );
    }
    if ftype == 1 as libc::c_int {
        return;
    }
    if state == 1 as libc::c_int {
        fprintf(
            __stderrp,
            b"Line %d: Unexpected EOF in header definition\n\0" as *const u8
                as *const libc::c_char,
            linenum,
        );
    }
    abc_eof();
}
