use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn __maskrune(_: __darwin_ct_rune_t, _: libc::c_ulong) -> libc::c_int;
    static mut _DefaultRuneLocale: _RuneLocale;
    fn a2b(fmt: *mut libc::c_char, _: ...);
    fn error(sev: libc::c_int, s: *mut SYMBOL, fmt: *mut libc::c_char, _: ...);
}
pub type __uint32_t = libc::c_uint;
pub type __darwin_ct_rune_t = libc::c_int;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_wchar_t = libc::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
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
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn isspace(mut _c: libc::c_int) -> libc::c_int {
    return __istype(_c, 0x4000 as libc::c_long as libc::c_ulong);
}
static mut c2: [*mut libc::c_char; 64] = [
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    b"nbspace\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"exclamdown\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"cent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"sterling\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"currency\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"yen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"brokenbar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"section\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"dieresis\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"copyright\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ordfeminine\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"guillemotleft\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"logicalnot\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"sfthyphen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"registered\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"macron\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"degree\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"plusminus\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"twosuperior\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"threesuperior\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"acute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mu\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"paragraph\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"periodcentered\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"cedilla\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"onesuperior\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ordmasculine\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"guillemotright\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"onequarter\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"onehalf\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"threequarters\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"questiondown\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
static mut c3: [*mut libc::c_char; 64] = [
    b"Agrave\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Aacute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Acircumflex\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Atilde\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Adieresis\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Aring\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"AE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Ccedilla\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Egrave\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Eacute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Ecircumflex\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Edieresis\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Igrave\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Iacute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Icircumflex\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Idieresis\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Eth\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Ntilde\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Ograve\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Oacute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Ocircumflex\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Otilde\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Odieresis\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"multiply\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Oslash\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Ugrave\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Uacute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Ucircumflex\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Udieresis\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Yacute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Thorn\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"germandbls\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"agrave\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"aacute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"acircumflex\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"atilde\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"adieresis\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"aring\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ae\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ccedilla\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"egrave\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"eacute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ecircumflex\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"edieresis\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"igrave\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"iacute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"icircumflex\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"idieresis\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"eth\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ntilde\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ograve\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"oacute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ocircumflex\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"otilde\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"odieresis\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"divide\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"oslash\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ugrave\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"uacute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ucircumflex\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"udieresis\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"yacute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"thorn\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ydieresis\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
static mut c4: [*mut libc::c_char; 64] = [
    b"Amacron\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"amacron\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Abreve\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"abreve\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Aogonek\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"aogonek\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Cacute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"cacute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Ccircumflex\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ccircumflex\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Cdotaccent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"cdotaccent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Ccaron\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ccaron\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Dcaron\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"dcaron\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Dcroat\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"dcroat\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Emacron\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"emacron\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Ebreve\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ebreve\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Edotaccent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"edotaccent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Eogonek\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"eogonek\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Ecaron\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ecaron\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Gcircumflex\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gcircumflex\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Gbreve\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gbreve\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Gdotaccent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gdotaccent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Gcommaaccent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gcommaaccent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Hcircumflex\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"hcircumflex\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Hbar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"hbar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Itilde\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"itilde\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Imacron\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"imacron\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Ibreve\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ibreve\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Iogonek\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"iogonek\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Idotaccent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"dotlessi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"IJ\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ij\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Jcircumflex\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"jcircumflex\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Kcedilla\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"kcedilla\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"kgreenlandic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Lacute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"lacute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Lcedilla\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"lcedilla\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Lcaron\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"lcaron\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Ldot\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
static mut c5: [*mut libc::c_char; 64] = [
    b"ldot\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Lslash\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"lslash\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Nacute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"nacute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Ncedilla\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ncedilla\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tmacron\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ncaron\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"napostrophe\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Eng\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"eng\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Omacron\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"omacron\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Obreve\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"obreve\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Ohungarumlaut\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ohungarumlaut\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"OE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"oe\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Racute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"racute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Rcommaaccent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"rcommaaccent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Rcaron\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"rcaron\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Sacute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"sacute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Scircumflex\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"scircumflex\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Scedilla\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"scedilla\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Scaron\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"scaron\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Tcedilla\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tcedilla\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Tcaron\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tcaron\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Tbar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tbar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Utilde\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"utilde\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Umacron\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"umacron\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Ubreve\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ubreve\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Uring\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"uring\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Uhungarumlaut\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"uhungarumlaut\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Uogonek\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"uogonek\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Wcircumflex\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"wcircumflex\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Ycircumflex\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ycircumflex\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Ydieresis\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Zacute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"zacute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Zdotaccent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"zdotaccent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Zcaron\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"zcaron\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"longs\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
static mut ce: [*mut libc::c_char; 64] = [
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    b"Delta\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
static mut e299: [*mut libc::c_char; 64] = [
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    b"flat\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"natural\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"sharp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
static mut e2: [*mut *mut libc::c_char; 64] = unsafe {
    [
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        e299.as_ptr() as *mut _,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    ]
};
static mut f09d84: [*mut libc::c_char; 64] = [
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    b"double_sharp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"double_flat\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
static mut f09d: [*mut *mut libc::c_char; 64] = unsafe {
    [
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        f09d84.as_ptr() as *mut _,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    ]
};
static mut f0: [*mut *mut *mut libc::c_char; 64] = unsafe {
    [
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        f09d.as_ptr() as *mut _,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
        0 as *const *mut *mut libc::c_char as *mut *mut *mut libc::c_char,
    ]
};
static mut utf_1: [*mut *mut libc::c_char; 62] = unsafe {
    [
        c2.as_ptr() as *mut _,
        c3.as_ptr() as *mut _,
        c4.as_ptr() as *mut _,
        c5.as_ptr() as *mut _,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        ce.as_ptr() as *mut _,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        e2.as_ptr() as *mut _ as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        f0.as_ptr() as *mut _ as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    ]
};
#[no_mangle]
pub unsafe extern "C" fn glyph_out(mut p: *mut libc::c_char) -> *mut libc::c_char {
    let mut i1: libc::c_int = 0;
    let mut i2: libc::c_int = 0;
    let mut i3: libc::c_int = 0;
    let mut i4: libc::c_int = 0;
    let mut g: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    g = 0 as *mut *mut libc::c_char;
    let fresh0 = p;
    p = p.offset(1);
    i1 = *fresh0 as libc::c_uchar as libc::c_int - 0xc2 as libc::c_int;
    let fresh1 = p;
    p = p.offset(1);
    i2 = *fresh1 as libc::c_uchar as libc::c_int - 0x80 as libc::c_int;
    if i1 >= 0xe0 as libc::c_int - 0xc2 as libc::c_int {
        let fresh2 = p;
        p = p.offset(1);
        i3 = *fresh2 as libc::c_uchar as libc::c_int - 0x80 as libc::c_int;
        if i1 >= 0xf0 as libc::c_int - 0xc2 as libc::c_int {
            let fresh3 = p;
            p = p.offset(1);
            i4 = *fresh3 as libc::c_uchar as libc::c_int - 0x80 as libc::c_int;
        } else {
            i4 = -(1 as libc::c_int);
        }
    } else {
        i3 = -(1 as libc::c_int);
        i4 = -(1 as libc::c_int);
    }
    if i1 >= 0 as libc::c_int && i2 >= 0 as libc::c_int {
        g = utf_1[i1 as usize];
        if !g.is_null() {
            g = *g.offset(i2 as isize) as *mut *mut libc::c_char;
            if i3 >= 0 as libc::c_int && !g.is_null() {
                g = *g.offset(i3 as isize) as *mut *mut libc::c_char;
                if i4 >= 0 as libc::c_int && !g.is_null() {
                    g = *g.offset(i4 as isize) as *mut *mut libc::c_char;
                }
            }
        }
        q = g as *mut libc::c_char;
    } else {
        q = 0 as *mut libc::c_char;
    }
    if q.is_null() {
        q = b".notdef\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    a2b(b"/%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char, q);
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn glyph_add(mut p: *mut libc::c_char) {
    let mut val: libc::c_int = 0;
    let mut i1: libc::c_int = 0;
    let mut i2: libc::c_int = 0;
    let mut i3: libc::c_int = 0;
    let mut i4: libc::c_int = 0;
    let mut g: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut g1: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    val = strtoul(p, &mut q, 16 as libc::c_int) as libc::c_int;
    if val < 0x80 as libc::c_int || val >= 0x100000 as libc::c_int {
        error(
            1 as libc::c_int,
            0 as *mut SYMBOL,
            b"Bad unicode value '%s'\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            p,
        );
        return;
    }
    p = q;
    while isspace(*p as libc::c_int) != 0 {
        p = p.offset(1);
        p;
    }
    i4 = -(1 as libc::c_int);
    i3 = i4;
    if val < 0x400 as libc::c_int {
        i1 = (val >> 6 as libc::c_int) - 2 as libc::c_int;
        i2 = val & 0x3f as libc::c_int;
    } else if val < 0x10000 as libc::c_int {
        i1 = (val >> 12 as libc::c_int) + 0x20 as libc::c_int - 2 as libc::c_int;
        i2 = val >> 6 as libc::c_int & 0x3f as libc::c_int;
        i3 = val & 0x3f as libc::c_int;
    } else {
        i1 = (val >> 18 as libc::c_int) + 0x30 as libc::c_int - 2 as libc::c_int;
        i2 = val >> 12 as libc::c_int & 0x3f as libc::c_int;
        i3 = val >> 6 as libc::c_int & 0x3f as libc::c_int;
        i4 = val & 0x3f as libc::c_int;
    }
    g1 = utf_1[i1 as usize];
    if g1.is_null() {
        g1 = calloc(
            64 as libc::c_int as libc::c_ulong,
            ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
        ) as *mut *mut libc::c_char;
        utf_1[i1 as usize] = g1;
    }
    if i3 < 0 as libc::c_int {
        let ref mut fresh4 = *g1.offset(i2 as isize);
        *fresh4 = strdup(p);
        return;
    }
    g = *g1.offset(i2 as isize) as *mut *mut libc::c_char;
    if g.is_null() {
        g = calloc(
            64 as libc::c_int as libc::c_ulong,
            ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
        ) as *mut *mut libc::c_char;
        let ref mut fresh5 = *g1.offset(i2 as isize);
        *fresh5 = g as *mut libc::c_char;
    }
    if i4 < 0 as libc::c_int {
        let ref mut fresh6 = *g.offset(i3 as isize);
        *fresh6 = strdup(p);
        return;
    }
    g1 = *g.offset(i3 as isize) as *mut *mut libc::c_char;
    if g1.is_null() {
        g1 = calloc(
            64 as libc::c_int as libc::c_ulong,
            ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
        ) as *mut *mut libc::c_char;
        let ref mut fresh7 = *g.offset(i3 as isize);
        *fresh7 = g1 as *mut libc::c_char;
    }
    let ref mut fresh8 = *g1.offset(i4 as isize);
    *fresh8 = strdup(p);
}
