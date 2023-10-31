use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type __sFILEX;
    static mut __stderrp: *mut FILE;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn atof(_: *const libc::c_char) -> libc::c_double;
    fn atoi(_: *const libc::c_char) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn memcpy(
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn __maskrune(_: __darwin_ct_rune_t, _: libc::c_ulong) -> libc::c_int;
    static mut _DefaultRuneLocale: _RuneLocale;
    static mut voice_tb: [VOICE_S; 32];
    fn lvlarena(level: libc::c_int) -> libc::c_int;
    fn getarena(len: libc::c_int) -> *mut libc::c_void;
    static mut tex_buf: [libc::c_char; 0];
    fn do_tune();
}
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_longlong;
pub type __darwin_ct_rune_t = libc::c_int;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_wchar_t = libc::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_off_t = __int64_t;
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
pub type accidentals = libc::c_uint;
pub const A_DF: accidentals = 5;
pub const A_DS: accidentals = 4;
pub const A_FT: accidentals = 3;
pub const A_NT: accidentals = 2;
pub const A_SH: accidentals = 1;
pub const A_NULL: accidentals = 0;
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
pub struct kw_s {
    pub name: *mut libc::c_char,
    pub len: libc::c_short,
    pub index: libc::c_short,
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
pub unsafe extern "C" fn isalnum(mut _c: libc::c_int) -> libc::c_int {
    return __istype(
        _c,
        (0x100 as libc::c_long | 0x400 as libc::c_long) as libc::c_ulong,
    );
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
pub static mut severity: libc::c_int = 0;
static mut ulen: libc::c_int = 0;
static mut meter: libc::c_short = 0;
static mut microscale: libc::c_uchar = 0;
static mut vover: libc::c_schar = 0;
static mut lyric_started: libc::c_char = 0;
static mut gchord: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut dc: decos = decos {
    n: 0,
    tm: [C2RustUnnamed { t: 0, m: 0 }; 32],
};
static mut deco_start: *mut SYMBOL = 0 as *const SYMBOL as *mut SYMBOL;
static mut deco_cont: *mut SYMBOL = 0 as *const SYMBOL as *mut SYMBOL;
static mut g_abc_vers: libc::c_int = 0;
static mut g_ulen: libc::c_int = 0;
static mut g_microscale: libc::c_int = 0;
static mut g_char_tb: [libc::c_char; 128] = [0; 128];
static mut g_deco_tb: [*mut libc::c_char; 128] = [0 as *const libc::c_char
    as *mut libc::c_char; 128];
static mut g_micro_tb: [libc::c_ushort; 32] = [0; 32];
static mut abc_fn: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut linenum: libc::c_int = 0;
static mut colnum: libc::c_int = 0;
static mut abc_line: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut last_sym: *mut SYMBOL = 0 as *const SYMBOL as *mut SYMBOL;
static mut nvoice: libc::c_short = 0;
#[no_mangle]
pub static mut curvoice: *mut VOICE_S = 0 as *const VOICE_S as *mut VOICE_S;
#[no_mangle]
pub static mut parse: parse = parse {
    first_sym: 0 as *const SYMBOL as *mut SYMBOL,
    last_sym: 0 as *const SYMBOL as *mut SYMBOL,
    abc_vers: 0,
    deco_tb: [0 as *const libc::c_char as *mut libc::c_char; 128],
    micro_tb: [0; 32],
    abc_state: 0,
};
static mut char_tb: [libc::c_char; 256] = [
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    18 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    11 as libc::c_int as libc::c_char,
    15 as libc::c_int as libc::c_char,
    5 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    13 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    15 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    12 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    16 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    14 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    14 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    6 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    17 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
];
static mut all_notes: [libc::c_char; 15] = unsafe {
    *::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"CDEFGABcdefgab\0")
};
unsafe extern "C" fn print_error(mut s: *mut libc::c_char, mut col: libc::c_int) {
    if col >= 0 as libc::c_int {
        fprintf(
            __stderrp,
            b"%s:%d:%d: error: %s\n\0" as *const u8 as *const libc::c_char,
            abc_fn,
            linenum,
            col,
            s,
        );
    } else {
        fprintf(
            __stderrp,
            b"%s:%d: error: %s\n\0" as *const u8 as *const libc::c_char,
            abc_fn,
            linenum,
            s,
        );
    };
}
unsafe extern "C" fn abc_new(
    mut type_0: libc::c_int,
    mut text: *mut libc::c_char,
) -> *mut SYMBOL {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    s = getarena(::core::mem::size_of::<SYMBOL>() as libc::c_ulong as libc::c_int)
        as *mut SYMBOL;
    memset(
        s as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<SYMBOL>() as libc::c_ulong,
    );
    if !text.is_null() {
        (*s)
            .text = getarena(
            (strlen(text)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        ) as *mut libc::c_char;
        strcpy((*s).text, text);
    }
    if (parse.last_sym).is_null() {
        parse.first_sym = s;
    } else {
        (*s).abc_next = (*parse.last_sym).abc_next;
        if !((*s).abc_next).is_null() {
            (*(*s).abc_next).abc_prev = s;
        }
        (*parse.last_sym).abc_next = s;
        (*s).abc_prev = parse.last_sym;
    }
    parse.last_sym = s;
    last_sym = parse.last_sym;
    (*s).abc_type = type_0 as libc::c_char;
    (*s).state = parse.abc_state as libc::c_char;
    (*s).fn_0 = abc_fn;
    (*s).linenum = linenum;
    (*s).colnum = colnum as libc::c_ushort;
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn abc_parse(
    mut p: *mut libc::c_char,
    mut fname: *mut libc::c_char,
    mut ln: libc::c_int,
) {
    abc_fn = fname;
    linenum = ln;
    abc_line = p;
    match parse_line(p) {
        2 => {
            g_abc_vers = parse.abc_vers;
            g_ulen = ulen;
            g_microscale = microscale as libc::c_int;
            meter = 2 as libc::c_int as libc::c_short;
            memcpy(
                g_char_tb.as_mut_ptr() as *mut libc::c_void,
                char_tb.as_mut_ptr() as *const libc::c_void,
                ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            );
            memcpy(
                g_deco_tb.as_mut_ptr() as *mut libc::c_void,
                (parse.deco_tb).as_mut_ptr() as *const libc::c_void,
                ::core::mem::size_of::<[*mut libc::c_char; 128]>() as libc::c_ulong,
            );
            memcpy(
                g_micro_tb.as_mut_ptr() as *mut libc::c_void,
                (parse.micro_tb).as_mut_ptr() as *const libc::c_void,
                ::core::mem::size_of::<[libc::c_ushort; 32]>() as libc::c_ulong,
            );
        }
        1 => {
            if !(parse.first_sym).is_null() {
                do_tune();
                parse.last_sym = 0 as *mut SYMBOL;
                parse.first_sym = parse.last_sym;
            }
            parse.abc_state = 0 as libc::c_int;
            parse.abc_vers = g_abc_vers;
            ulen = g_ulen;
            microscale = g_microscale as libc::c_uchar;
            memcpy(
                char_tb.as_mut_ptr() as *mut libc::c_void,
                g_char_tb.as_mut_ptr() as *const libc::c_void,
                ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            );
            memcpy(
                (parse.deco_tb).as_mut_ptr() as *mut libc::c_void,
                g_deco_tb.as_mut_ptr() as *const libc::c_void,
                ::core::mem::size_of::<[*mut libc::c_char; 128]>() as libc::c_ulong,
            );
            memcpy(
                (parse.micro_tb).as_mut_ptr() as *mut libc::c_void,
                g_micro_tb.as_mut_ptr() as *const libc::c_void,
                ::core::mem::size_of::<[libc::c_ushort; 32]>() as libc::c_ulong,
            );
            lvlarena(0 as libc::c_int);
            if dc.n as libc::c_int > 0 as libc::c_int {
                syntax(
                    b"Decoration without symbol\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *mut libc::c_char,
                );
            }
            dc.n = 0 as libc::c_int as libc::c_char;
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn abc_eof() {
    do_tune();
    parse.last_sym = 0 as *mut SYMBOL;
    parse.first_sym = parse.last_sym;
    if parse.abc_state != 0 as libc::c_int {
        parse.abc_vers = g_abc_vers;
        ulen = g_ulen;
        microscale = g_microscale as libc::c_uchar;
        memcpy(
            char_tb.as_mut_ptr() as *mut libc::c_void,
            g_char_tb.as_mut_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        );
    }
}
unsafe extern "C" fn broken_rhythm(mut s: *mut SYMBOL, mut num: libc::c_int) {
    let mut notes: *mut notes = &mut (*s).u.note;
    let mut l: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    num *= 2 as libc::c_int;
    if num > 0 as libc::c_int {
        if num == 6 as libc::c_int {
            num = 8 as libc::c_int;
        }
        n = num * 2 as libc::c_int - 1 as libc::c_int;
        m = 0 as libc::c_int;
        while m <= (*s).nhd as libc::c_int {
            (*notes).notes[m as usize].len = (*notes).notes[m as usize].len * n / num;
            m += 1;
            m;
        }
    } else {
        n = -num;
        if n == 6 as libc::c_int {
            n = 8 as libc::c_int;
        }
        m = 0 as libc::c_int;
        while m <= (*s).nhd as libc::c_int {
            (*notes).notes[m as usize].len /= n;
            m += 1;
            m;
        }
    }
    l = (*notes).notes[0 as libc::c_int as usize].len;
    m = 1 as libc::c_int;
    while m <= (*s).nhd as libc::c_int {
        if (*notes).notes[m as usize].len < l {
            l = (*notes).notes[m as usize].len;
        }
        m += 1;
        m;
    }
}
unsafe extern "C" fn check_nl(mut p: *mut libc::c_char) -> libc::c_int {
    while *p as libc::c_int != '\0' as i32 {
        let fresh0 = p;
        p = p.offset(1);
        match *fresh0 as libc::c_int {
            33 => return 0 as libc::c_int,
            124 | 91 | 58 | 93 | 32 | 9 => return 1 as libc::c_int,
            _ => {}
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn parse_extra(
    mut p: *mut libc::c_char,
    mut p_name: *mut *mut libc::c_char,
    mut p_middle: *mut *mut libc::c_char,
    mut p_stlines: *mut *mut libc::c_char,
    mut p_scale: *mut *mut libc::c_char,
    mut p_octave: *mut *mut libc::c_char,
    mut p_cue: *mut *mut libc::c_char,
    mut p_map: *mut *mut libc::c_char,
) -> *mut libc::c_char {
    loop {
        if strncmp(
            p,
            b"clef=\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
            || strncmp(
                p,
                b"bass\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            || strncmp(
                p,
                b"treble\0" as *const u8 as *const libc::c_char,
                6 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            || strncmp(
                p,
                b"alto\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            || strncmp(
                p,
                b"tenor\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            || strncmp(
                p,
                b"perc\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
        {
            if !(*p_name).is_null() {
                syntax(
                    b"Double clef name\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    p,
                );
            }
            *p_name = p;
        } else if strncmp(
            p,
            b"microscale=\0" as *const u8 as *const libc::c_char,
            11 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
            || strncmp(
                p,
                b"uscale=\0" as *const u8 as *const libc::c_char,
                7 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
        {
            let mut i: libc::c_int = 0;
            p = p
                .offset(
                    (if *p.offset(0 as libc::c_int as isize) as libc::c_int == 'm' as i32
                    {
                        11 as libc::c_int
                    } else {
                        7 as libc::c_int
                    }) as isize,
                );
            i = atoi(p);
            if i < 4 as libc::c_int || i >= 256 as libc::c_int {
                syntax(
                    b"Invalid value in microscale=\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    p,
                );
            } else {
                microscale = i as libc::c_uchar;
            }
        } else if strncmp(
            p,
            b"middle=\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
            || strncmp(
                p,
                b"m=\0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
        {
            if !(*p_middle).is_null() {
                syntax(
                    b"Double clef middle\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    p,
                );
            }
            *p_middle = p
                .offset(
                    (if *p.offset(1 as libc::c_int as isize) as libc::c_int == '=' as i32
                    {
                        2 as libc::c_int
                    } else {
                        7 as libc::c_int
                    }) as isize,
                );
        } else if strncmp(
            p,
            b"octave=\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            if !(*p_octave).is_null() {
                syntax(
                    b"Double octave=\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    p,
                );
            }
            *p_octave = p.offset(7 as libc::c_int as isize);
        } else if strncmp(
            p,
            b"stafflines=\0" as *const u8 as *const libc::c_char,
            11 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            let mut l: libc::c_int = 0;
            let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
            if !(*p_stlines).is_null() {
                syntax(
                    b"Double stafflines\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    p,
                );
            }
            p = p.offset(11 as libc::c_int as isize);
            if isdigit(*p as libc::c_uchar as libc::c_int) != 0 {
                match atoi(p) {
                    0 => {
                        *p_stlines = b"...\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    1 => {
                        *p_stlines = b"..|\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    2 => {
                        *p_stlines = b".||\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    3 => {
                        *p_stlines = b".|||\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    4 => {
                        *p_stlines = b"||||\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    5 => {
                        *p_stlines = b"|||||\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    6 => {
                        *p_stlines = b"||||||\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    7 => {
                        *p_stlines = b"|||||||\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    8 => {
                        *p_stlines = b"||||||||\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    _ => {
                        syntax(
                            b"Bad number of lines\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            p,
                        );
                    }
                }
            } else {
                q = p;
                while isspace(*p as libc::c_uchar as libc::c_int) == 0
                    && *p as libc::c_int != '\0' as i32
                {
                    p = p.offset(1);
                    p;
                }
                l = p.offset_from(q) as libc::c_long as libc::c_int;
                *p_stlines = getarena(l + 1 as libc::c_int) as *mut libc::c_char;
                strncpy(*p_stlines, q, l as libc::c_ulong);
                *(*p_stlines).offset(l as isize) = '\0' as i32 as libc::c_char;
            }
        } else if strncmp(
            p,
            b"staffscale=\0" as *const u8 as *const libc::c_char,
            11 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            if !(*p_scale).is_null() {
                syntax(
                    b"Double staffscale\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    p,
                );
            }
            *p_scale = p.offset(11 as libc::c_int as isize);
        } else if strncmp(
            p,
            b"cue=\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            if !(*p_cue).is_null() {
                syntax(
                    b"Double cue\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    p,
                );
            }
            *p_cue = p.offset(4 as libc::c_int as isize);
        } else {
            if !(strncmp(
                p,
                b"map=\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int)
            {
                break;
            }
            if !(*p_map).is_null() {
                syntax(
                    b"Double map\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    p,
                );
            }
            *p_map = p.offset(4 as libc::c_int as isize);
        }
        while isspace(*p as libc::c_uchar as libc::c_int) == 0
            && *p as libc::c_int != '\0' as i32
        {
            p = p.offset(1);
            p;
        }
        while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
            p = p.offset(1);
            p;
        }
        if *p as libc::c_int == '\0' as i32 {
            break;
        }
    }
    return p;
}
unsafe extern "C" fn get_deco(
    mut p: *mut libc::c_char,
    mut p_dc: *mut libc::c_uchar,
) -> *mut libc::c_char {
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sep: libc::c_char = 0;
    let mut t: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut i: libc::c_uint = 0;
    let mut l: libc::c_uint = 0;
    *p_dc = 0 as libc::c_int as libc::c_uchar;
    q = p;
    sep = *q.offset(-(1 as libc::c_int) as isize);
    if char_tb[sep as libc::c_uchar as usize] as libc::c_int == 15 as libc::c_int {
        if sep as libc::c_int == '+' as i32 {
            if *p as libc::c_int == '+' as i32
                && *p.offset(1 as libc::c_int as isize) as libc::c_int == '+' as i32
            {
                p = p.offset(1);
                p;
            }
        }
    } else {
        sep = '\0' as i32 as libc::c_char;
    }
    while *p as libc::c_int != sep as libc::c_int {
        if *p as libc::c_int == '\0' as i32 {
            syntax(
                b"Decoration not terminated\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                q,
            );
            return p;
        }
        p = p.offset(1);
        p;
    }
    l = p.offset_from(q) as libc::c_long as libc::c_uint;
    if *p as libc::c_int == sep as libc::c_int {
        p = p.offset(1);
        p;
    }
    i = 1 as libc::c_int as libc::c_uint;
    t = &mut *(parse.deco_tb).as_mut_ptr().offset(1 as libc::c_int as isize)
        as *mut *mut libc::c_char;
    while !(*t).is_null() && i < 128 as libc::c_int as libc::c_uint {
        if strlen(*t) == l as libc::c_ulong
            && strncmp(*t, q, l as libc::c_ulong) == 0 as libc::c_int
        {
            *p_dc = i.wrapping_add(128 as libc::c_int as libc::c_uint) as libc::c_uchar;
            return p;
        }
        i = i.wrapping_add(1);
        i;
        t = t.offset(1);
        t;
    }
    if i < 128 as libc::c_int as libc::c_uint {
        *t = getarena(l.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int)
            as *mut libc::c_char;
        memcpy(*t as *mut libc::c_void, q as *const libc::c_void, l as libc::c_ulong);
        *(*t).offset(l as isize) = '\0' as i32 as libc::c_char;
        *p_dc = i.wrapping_add(128 as libc::c_int as libc::c_uint) as libc::c_uchar;
    } else {
        syntax(
            b"Too many decoration types\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            q,
        );
    }
    return p;
}
unsafe extern "C" fn parse_acc(
    mut p: *mut libc::c_char,
    mut s: *mut SYMBOL,
) -> *mut libc::c_char {
    let mut pit: libc::c_int = 0 as libc::c_int;
    let mut acc: libc::c_int = 0;
    let mut nacc: libc::c_uint = 0;
    nacc = 0 as libc::c_int as libc::c_uint;
    loop {
        if nacc as libc::c_ulong
            >= ::core::mem::size_of::<[libc::c_schar; 8]>() as libc::c_ulong
        {
            syntax(
                b"Too many accidentals\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                p,
            );
            break;
        } else {
            p = parse_acc_pit(p, &mut pit, &mut acc);
            if acc < 0 as libc::c_int {
                break;
            }
            (*s).u.key.pits[nacc as usize] = pit as libc::c_schar;
            let fresh1 = nacc;
            nacc = nacc.wrapping_add(1);
            (*s).u.key.accs[fresh1 as usize] = acc as libc::c_uchar;
            while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
                p = p.offset(1);
                p;
            }
            if *p as libc::c_int == '\0' as i32 {
                break;
            }
            if *p as libc::c_int != '^' as i32 && *p as libc::c_int != '_' as i32
                && *p as libc::c_int != '=' as i32
            {
                break;
            }
        }
    }
    (*s).u.key.microscale = microscale;
    if (*s).u.key.empty as libc::c_int != 2 as libc::c_int {
        (*s).u.key.nacc = nacc as libc::c_schar;
    }
    return p;
}
unsafe extern "C" fn parse_clef(
    mut s: *mut SYMBOL,
    mut name: *mut libc::c_char,
    mut middle: *mut libc::c_char,
) {
    let mut clef: libc::c_int = -(1 as libc::c_int);
    let mut transpose: libc::c_int = 0 as libc::c_int;
    let mut clef_line: libc::c_int = 2 as libc::c_int;
    let mut warn: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str: [libc::c_char; 80] = [0; 80];
    str[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    if !name.is_null()
        && strncmp(
            name,
            b"clef=\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        name = name.offset(5 as libc::c_int as isize);
        let mut current_block_22: u64;
        match *name as libc::c_int {
            34 => {
                name = get_str(
                    str.as_mut_ptr(),
                    name,
                    ::core::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong
                        as libc::c_int,
                );
                (*s)
                    .u
                    .clef
                    .name = getarena(
                    (strlen(str.as_mut_ptr()))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                ) as *mut libc::c_char;
                strcpy((*s).u.clef.name, str.as_mut_ptr());
                clef = 0 as libc::c_int;
                current_block_22 = 14401909646449704462;
            }
            103 => {
                warn = name;
                transpose = -(7 as libc::c_int);
                current_block_22 = 12779679808429181866;
            }
            71 => {
                current_block_22 = 12779679808429181866;
            }
            102 => {
                warn = name;
                transpose = -(14 as libc::c_int);
                clef = 2 as libc::c_int;
                clef_line = 4 as libc::c_int;
                current_block_22 = 14401909646449704462;
            }
            70 => {
                if *name.offset(1 as libc::c_int as isize) as libc::c_int == ',' as i32 {
                    transpose = -(7 as libc::c_int);
                }
                clef = 2 as libc::c_int;
                clef_line = 4 as libc::c_int;
                current_block_22 = 14401909646449704462;
            }
            99 => {
                warn = name;
                transpose = -(7 as libc::c_int);
                current_block_22 = 12975250986578824200;
            }
            67 => {
                current_block_22 = 12975250986578824200;
            }
            80 => {
                clef = 3 as libc::c_int;
                clef_line = 3 as libc::c_int;
                current_block_22 = 14401909646449704462;
            }
            _ => {
                current_block_22 = 14401909646449704462;
            }
        }
        match current_block_22 {
            12975250986578824200 => {
                clef = 1 as libc::c_int;
                clef_line = 3 as libc::c_int;
            }
            12779679808429181866 => {
                clef = 0 as libc::c_int;
            }
            _ => {}
        }
        if clef >= 0 as libc::c_int {
            name = name.offset(1);
            name;
            if *name as libc::c_int == ',' as i32 || *name as libc::c_int == '\'' as i32
            {
                warn = name;
            }
            while *name as libc::c_int == ',' as i32 {
                transpose += 7 as libc::c_int;
                name = name.offset(1);
                name;
            }
            while *name as libc::c_int == '\'' as i32 {
                transpose -= 7 as libc::c_int;
                name = name.offset(1);
                name;
            }
        }
    }
    if !name.is_null() && clef < 0 as libc::c_int {
        if strncmp(
            name,
            b"bass\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            clef = 2 as libc::c_int;
            clef_line = 4 as libc::c_int;
            (*s).u.clef.check_pitch = 1 as libc::c_int as libc::c_char;
            name = name.offset(4 as libc::c_int as isize);
        } else if strncmp(
            name,
            b"treble\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            clef = 0 as libc::c_int;
            name = name.offset(6 as libc::c_int as isize);
        } else if strncmp(
            name,
            b"alto\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0
            || strncmp(
                name,
                b"tenor\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int as libc::c_ulong,
            ) == 0
        {
            clef = 1 as libc::c_int;
            clef_line = if *name as libc::c_int == 'a' as i32 {
                3 as libc::c_int
            } else {
                4 as libc::c_int
            };
            (*s).u.clef.check_pitch = 1 as libc::c_int as libc::c_char;
            if *name as libc::c_int == 'a' as i32 {
                name = name.offset(4 as libc::c_int as isize);
            } else {
                name = name.offset(5 as libc::c_int as isize);
            }
        } else if strncmp(
            name,
            b"perc\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            clef = 3 as libc::c_int;
            clef_line = 3 as libc::c_int;
            name = name.offset(4 as libc::c_int as isize);
        } else if strncmp(
            name,
            b"auto\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            clef = 4 as libc::c_int;
            name = name.offset(4 as libc::c_int as isize);
        } else if strncmp(
            name,
            b"none\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            clef = 0 as libc::c_int;
            (*s).u.clef.invis = 1 as libc::c_int as libc::c_char;
            (*s)
                .flags = ((*s).flags as libc::c_int | 0x2 as libc::c_int)
                as libc::c_ushort;
            name = name.offset(4 as libc::c_int as isize);
        } else {
            syntax(
                b"Unknown clef\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                name,
            );
            clef = 0 as libc::c_int;
        }
    }
    if clef >= 0 as libc::c_int {
        if isdigit(*name as libc::c_uchar as libc::c_int) != 0 {
            let fresh2 = name;
            name = name.offset(1);
            clef_line = *fresh2 as libc::c_int - '0' as i32;
        }
        if *name.offset(1 as libc::c_int as isize) as libc::c_int == '8' as i32 {
            let mut current_block_82: u64;
            match *name as libc::c_int {
                94 => {
                    transpose -= 7 as libc::c_int;
                    current_block_82 = 10006674590530960285;
                }
                43 => {
                    current_block_82 = 10006674590530960285;
                }
                95 => {
                    transpose += 7 as libc::c_int;
                    current_block_82 = 6891228234384637087;
                }
                45 => {
                    current_block_82 = 6891228234384637087;
                }
                _ => {
                    current_block_82 = 12961834331865314435;
                }
            }
            match current_block_82 {
                10006674590530960285 => {
                    (*s).u.clef.octave = 1 as libc::c_int as libc::c_schar;
                }
                6891228234384637087 => {
                    (*s).u.clef.octave = -(1 as libc::c_int) as libc::c_schar;
                }
                _ => {}
            }
        }
    }
    if !middle.is_null() {
        let mut pit: libc::c_int = 0 as libc::c_int;
        let mut acc: libc::c_int = 0;
        let mut l: libc::c_int = 0;
        static mut line_tb: [libc::c_char; 7] = [
            1 as libc::c_int as libc::c_char,
            0 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            2 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
            2 as libc::c_int as libc::c_char,
            1 as libc::c_int as libc::c_char,
        ];
        warn = middle;
        parse_acc_pit(middle, &mut pit, &mut acc);
        if acc < 0 as libc::c_int {
            pit = 22 as libc::c_int;
        }
        if clef < 0 as libc::c_int {
            clef = line_tb[((pit + 7 as libc::c_int) % 7 as libc::c_int) as usize]
                as libc::c_int;
        }
        match clef {
            1 => {
                l = 16 as libc::c_int + 4 as libc::c_int;
            }
            2 => {
                l = 12 as libc::c_int + 4 as libc::c_int;
            }
            _ => {
                l = 20 as libc::c_int + 4 as libc::c_int;
            }
        }
        clef_line = (l - pit + 28 as libc::c_int) % 7 as libc::c_int;
        if clef_line & 1 as libc::c_int != 0 {
            syntax(
                b"Bad 'middle' value for the clef\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                middle,
            );
            pit += 1;
            pit;
        }
        clef_line = clef_line / 2 as libc::c_int + 1 as libc::c_int;
        transpose = l - (clef_line - 1 as libc::c_int) * 2 as libc::c_int - pit;
        (*s).u.clef.check_pitch = 0 as libc::c_int as libc::c_char;
    }
    (*s).u.clef.type_0 = clef as libc::c_schar;
    (*s).u.clef.line = clef_line as libc::c_char;
    (*s).u.clef.transpose = transpose as libc::c_schar;
    if !warn.is_null() {
        let mut sev_sav: libc::c_int = 0;
        sev_sav = severity;
        syntax(
            b"Warning: Deprecated or non-standard item\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            warn,
        );
        severity = sev_sav;
    }
}
unsafe extern "C" fn parse_octave(mut p: *mut libc::c_char) -> libc::c_int {
    let mut oct: libc::c_int = 0;
    if !p.is_null() {
        oct = 1 as libc::c_int;
        if *p as libc::c_int == '-' as i32 {
            oct = -(1 as libc::c_int);
            p = p.offset(1);
            p;
        }
        if *p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '4' as i32 {
            return oct * (*p as libc::c_int - '0' as i32);
        }
        syntax(
            b"Bad octave value\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            p,
        );
    }
    return 10 as libc::c_int;
}
unsafe extern "C" fn parse_key(mut p: *mut libc::c_char, mut s: *mut SYMBOL) {
    let mut current_block: u64;
    let mut sf: libc::c_int = 0;
    let mut empty: libc::c_int = 0;
    let mut instr: libc::c_int = 0;
    let mut clef_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut clef_middle: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut clef_stlines: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut clef_scale: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p_octave: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p_cue: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p_map: *mut libc::c_char = 0 as *mut libc::c_char;
    (*s).u.key.octave = 10 as libc::c_int as libc::c_schar;
    if *p as libc::c_int == '\0' as i32 {
        (*s).u.key.empty = 1 as libc::c_int as libc::c_char;
        return;
    }
    sf = 0 as libc::c_int;
    empty = 0 as libc::c_int;
    instr = 0 as libc::c_int;
    let mut current_block_36: u64;
    let fresh3 = p;
    p = p.offset(1);
    match *fresh3 as libc::c_int {
        70 => {
            sf = -(1 as libc::c_int);
            current_block_36 = 9853141518545631134;
        }
        66 => {
            sf += 1;
            sf;
            current_block_36 = 16418587454608096141;
        }
        69 => {
            current_block_36 = 16418587454608096141;
        }
        65 => {
            current_block_36 = 11690893870945745215;
        }
        68 => {
            current_block_36 = 8777491104853333637;
        }
        71 => {
            current_block_36 = 6640544648498507245;
        }
        67 => {
            current_block_36 = 9853141518545631134;
        }
        72 => {
            if *p as libc::c_int == 'P' as i32 {
                instr = 1 as libc::c_int;
                p = p.offset(1);
                p;
            } else if *p as libc::c_int == 'p' as i32 {
                instr = 2 as libc::c_int;
                sf = 2 as libc::c_int;
                p = p.offset(1);
                p;
            } else {
                syntax(
                    b"Unknown bagpipe-like key\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    p,
                );
            }
            current_block_36 = 9853141518545631134;
        }
        80 => {
            instr = 3 as libc::c_int;
            p = p.offset(1);
            p;
            current_block_36 = 9853141518545631134;
        }
        110 => {
            if strncmp(
                p,
                b"one\0" as *const u8 as *const libc::c_char,
                3 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                empty = 2 as libc::c_int;
                p = p.offset(3 as libc::c_int as isize);
                while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
                    p = p.offset(1);
                    p;
                }
                if *p as libc::c_int == '\0' as i32 {
                    (*s).u.key.empty = empty as libc::c_char;
                    return;
                }
                current_block_36 = 9853141518545631134;
            } else {
                current_block_36 = 6958501073019944551;
            }
        }
        _ => {
            current_block_36 = 6958501073019944551;
        }
    }
    match current_block_36 {
        6958501073019944551 => {
            p = p.offset(-1);
            p;
            empty = 1 as libc::c_int;
            current_block_36 = 9853141518545631134;
        }
        16418587454608096141 => {
            sf += 1;
            sf;
            current_block_36 = 11690893870945745215;
        }
        _ => {}
    }
    match current_block_36 {
        11690893870945745215 => {
            sf += 1;
            sf;
            current_block_36 = 8777491104853333637;
        }
        _ => {}
    }
    match current_block_36 {
        8777491104853333637 => {
            sf += 1;
            sf;
            current_block_36 = 6640544648498507245;
        }
        _ => {}
    }
    match current_block_36 {
        6640544648498507245 => {
            sf += 1;
            sf;
        }
        _ => {}
    }
    (*s).u.key.empty = empty as libc::c_char;
    if empty == 0 {
        if *p as libc::c_int == '#' as i32 {
            sf += 7 as libc::c_int;
            p = p.offset(1);
            p;
        } else if *p as libc::c_int == 'b' as i32 {
            sf -= 7 as libc::c_int;
            p = p.offset(1);
            p;
        }
        while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
            p = p.offset(1);
            p;
        }
        match *p as libc::c_int {
            97 | 65 => {
                if strncasecmp(
                    p,
                    b"aeo\0" as *const u8 as *const libc::c_char,
                    3 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    sf -= 3 as libc::c_int;
                    current_block = 7858101417678297991;
                } else {
                    current_block = 18270985416560570515;
                }
            }
            100 | 68 => {
                if strncasecmp(
                    p,
                    b"dor\0" as *const u8 as *const libc::c_char,
                    3 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    sf -= 2 as libc::c_int;
                    current_block = 7858101417678297991;
                } else {
                    current_block = 18270985416560570515;
                }
            }
            105 | 73 => {
                if strncasecmp(
                    p,
                    b"ion\0" as *const u8 as *const libc::c_char,
                    3 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    current_block = 7858101417678297991;
                } else {
                    current_block = 18270985416560570515;
                }
            }
            108 | 76 => {
                if strncasecmp(
                    p,
                    b"loc\0" as *const u8 as *const libc::c_char,
                    3 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    sf -= 5 as libc::c_int;
                    current_block = 7858101417678297991;
                } else if strncasecmp(
                    p,
                    b"lyd\0" as *const u8 as *const libc::c_char,
                    3 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    sf += 1 as libc::c_int;
                    current_block = 7858101417678297991;
                } else {
                    current_block = 18270985416560570515;
                }
            }
            109 | 77 => {
                if strncasecmp(
                    p,
                    b"maj\0" as *const u8 as *const libc::c_char,
                    3 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    current_block = 7858101417678297991;
                } else if strncasecmp(
                    p,
                    b"mix\0" as *const u8 as *const libc::c_char,
                    3 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    sf -= 1 as libc::c_int;
                    current_block = 7858101417678297991;
                } else if strncasecmp(
                    p,
                    b"min\0" as *const u8 as *const libc::c_char,
                    3 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                    || isalpha(
                        *p.offset(1 as libc::c_int as isize) as libc::c_uchar
                            as libc::c_int,
                    ) == 0
                {
                    sf -= 3 as libc::c_int;
                    current_block = 7858101417678297991;
                } else {
                    current_block = 18270985416560570515;
                }
            }
            112 | 80 => {
                if strncasecmp(
                    p,
                    b"phr\0" as *const u8 as *const libc::c_char,
                    3 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    sf -= 4 as libc::c_int;
                    current_block = 7858101417678297991;
                } else {
                    current_block = 18270985416560570515;
                }
            }
            _ => {
                current_block = 18270985416560570515;
            }
        }
        match current_block {
            18270985416560570515 => {
                empty = 1 as libc::c_int;
            }
            _ => {}
        }
        if empty == 0 {
            while isalpha(*p as libc::c_uchar as libc::c_int) != 0 {
                p = p.offset(1);
                p;
            }
            while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
                p = p.offset(1);
                p;
            }
        }
        if strncmp(
            p,
            b"exp \0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            p = p.offset(4 as libc::c_int as isize);
            while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
                p = p.offset(1);
                p;
            }
            if *p as libc::c_int == '\0' as i32 {
                syntax(
                    b"no accidental after 'exp'\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    p,
                );
            }
            (*s).u.key.exp = 1 as libc::c_int as libc::c_char;
        }
        if (*s).u.key.exp as libc::c_int != 0
            && strncmp(
                p,
                b"none\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
        {
            sf = 0 as libc::c_int;
            p = p.offset(4 as libc::c_int as isize);
            while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
                p = p.offset(1);
                p;
            }
        } else {
            match *p as libc::c_int {
                94 | 95 | 61 => {
                    p = parse_acc(p, s);
                }
                _ => {}
            }
        }
    }
    if sf > 7 as libc::c_int || sf < -(7 as libc::c_int) {
        syntax(
            b"Too many sharps/flats\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            p,
        );
        if sf > 0 as libc::c_int {
            sf -= 12 as libc::c_int;
        } else {
            sf += 12 as libc::c_int;
        }
    }
    clef_scale = 0 as *mut libc::c_char;
    clef_stlines = clef_scale;
    clef_middle = clef_stlines;
    clef_name = clef_middle;
    p_map = 0 as *mut libc::c_char;
    p_cue = p_map;
    p_octave = p_cue;
    parse_extra(
        p,
        &mut clef_name,
        &mut clef_middle,
        &mut clef_stlines,
        &mut clef_scale,
        &mut p_octave,
        &mut p_cue,
        &mut p_map,
    );
    (*s).u.key.sf = sf as libc::c_schar;
    (*s).u.key.instr = instr as libc::c_char;
    (*s).u.key.octave = parse_octave(p_octave) as libc::c_schar;
    if !p_cue.is_null() {
        if strncmp(
            p_cue,
            b"on\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            (*s).u.key.cue = 1 as libc::c_int as libc::c_schar;
        } else {
            (*s).u.key.cue = -(1 as libc::c_int) as libc::c_schar;
        }
    }
    if !clef_stlines.is_null() {
        (*s).u.key.stafflines = clef_stlines;
    }
    if !clef_scale.is_null() {
        let mut sc: libc::c_float = 0.;
        sc = atof(clef_scale) as libc::c_float;
        if sc as libc::c_double >= 0.5f64 && sc <= 3 as libc::c_int as libc::c_float {
            (*s).u.key.staffscale = sc;
        } else {
            syntax(
                b"Bad value of staffscale\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                clef_scale,
            );
        }
    }
    if !clef_name.is_null() || !clef_middle.is_null() {
        s = abc_new(3 as libc::c_int, 0 as *mut libc::c_char);
        parse_clef(s, clef_name, clef_middle);
    }
    if !p_map.is_null() {
        strcpy(
            tex_buf.as_mut_ptr(),
            b"%%voicemap \0" as *const u8 as *const libc::c_char,
        );
        get_str(
            &mut *tex_buf.as_mut_ptr().offset(11 as libc::c_int as isize),
            p_map,
            512 as libc::c_int - 12 as libc::c_int,
        );
        abc_new(2 as libc::c_int, tex_buf.as_mut_ptr());
    }
}
unsafe extern "C" fn get_len(
    mut p: *mut libc::c_char,
    mut s: *mut SYMBOL,
) -> *mut libc::c_char {
    let mut l1: libc::c_int = 0;
    let mut l2: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut error_txt: *mut libc::c_char = 0 as *mut libc::c_char;
    if strcmp(p, b"auto\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        ulen = 15120 as libc::c_int;
        (*s).u.length.base_length = -(1 as libc::c_int);
        return error_txt;
    }
    l1 = 0 as libc::c_int;
    l2 = 1 as libc::c_int;
    if sscanf(
        p,
        b"%d /%d \0" as *const u8 as *const libc::c_char,
        &mut l1 as *mut libc::c_int,
        &mut l2 as *mut libc::c_int,
    ) != 2 as libc::c_int || l1 == 0 as libc::c_int
    {
        (*s)
            .u
            .length
            .base_length = if ulen != 0 {
            ulen
        } else {
            1536 as libc::c_int / 8 as libc::c_int
        };
        return b"Bad unit note length: unchanged\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    if l2 == 0 as libc::c_int {
        error_txt = b"Bad length divisor, set to 4\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        l2 = 4 as libc::c_int;
    }
    d = 1536 as libc::c_int / l2;
    if d * l2 != 1536 as libc::c_int {
        error_txt = b"Length incompatible with BASE, using 1/8\0" as *const u8
            as *const libc::c_char as *mut libc::c_char;
        d = 1536 as libc::c_int / 8 as libc::c_int;
    } else {
        d *= l1;
        if l1 != 1 as libc::c_int || l2 & l2 - 1 as libc::c_int != 0 {
            error_txt = b"Incorrect unit note length, using 1/8\0" as *const u8
                as *const libc::c_char as *mut libc::c_char;
            d = 1536 as libc::c_int / 8 as libc::c_int;
        }
    }
    (*s).u.length.base_length = d;
    return error_txt;
}
unsafe extern "C" fn parse_meter(
    mut p: *mut libc::c_char,
    mut s: *mut SYMBOL,
) -> *mut libc::c_char {
    let mut m1: libc::c_int = 0;
    let mut m2: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut wmeasure: libc::c_int = 0;
    let mut nm: libc::c_int = 0;
    let mut in_parenth: libc::c_int = 0;
    let mut i: libc::c_uint = 0;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut top_err: [libc::c_char; 26] = unsafe {
        *::core::mem::transmute::<
            &[u8; 26],
            &mut [libc::c_char; 26],
        >(b"Cannot identify meter top\0")
    };
    if *p as libc::c_int == '\0' as i32 {
        return b"Empty meter string\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    nm = 0 as libc::c_int;
    in_parenth = 0 as libc::c_int;
    m1 = 0 as libc::c_int;
    if strncmp(
        p,
        b"none\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        p = p.offset(4 as libc::c_int as isize);
        wmeasure = 1 as libc::c_int;
    } else {
        wmeasure = 0 as libc::c_int;
        let mut current_block_63: u64;
        while *p as libc::c_int != '\0' as i32 {
            if *p as libc::c_int == '=' as i32 {
                break;
            }
            if nm >= 16 as libc::c_int {
                return b"Too many values in M:\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            match *p as libc::c_int {
                67 => {
                    let fresh4 = p;
                    p = p.offset(1);
                    (*s)
                        .u
                        .meter
                        .meter[nm as usize]
                        .top[0 as libc::c_int as usize] = *fresh4;
                    if *p as libc::c_int == '|' as i32 {
                        let fresh5 = p;
                        p = p.offset(1);
                        (*s)
                            .u
                            .meter
                            .meter[nm as usize]
                            .top[1 as libc::c_int as usize] = *fresh5;
                    }
                    m1 = 4 as libc::c_int;
                    m2 = 4 as libc::c_int;
                    current_block_63 = 6545907279487748450;
                }
                99 | 111 => {
                    if *p as libc::c_int == 'c' as i32 {
                        m1 = 4 as libc::c_int;
                    } else {
                        m1 = 3 as libc::c_int;
                    }
                    m2 = 4 as libc::c_int;
                    let fresh6 = p;
                    p = p.offset(1);
                    (*s)
                        .u
                        .meter
                        .meter[nm as usize]
                        .top[0 as libc::c_int as usize] = *fresh6;
                    if *p as libc::c_int == '.' as i32 {
                        let fresh7 = p;
                        p = p.offset(1);
                        (*s)
                            .u
                            .meter
                            .meter[nm as usize]
                            .top[1 as libc::c_int as usize] = *fresh7;
                    }
                    current_block_63 = 6545907279487748450;
                }
                40 => {
                    if *p.offset(1 as libc::c_int as isize) as libc::c_int == '(' as i32
                    {
                        in_parenth = 1 as libc::c_int;
                        let fresh8 = p;
                        p = p.offset(1);
                        let fresh9 = nm;
                        nm = nm + 1;
                        (*s)
                            .u
                            .meter
                            .meter[fresh9 as usize]
                            .top[0 as libc::c_int as usize] = *fresh8;
                    }
                    q = p.offset(1 as libc::c_int as isize);
                    while *q as libc::c_int != '\0' as i32 {
                        if *q as libc::c_int == ')' as i32
                            || *q as libc::c_int == '/' as i32
                        {
                            break;
                        }
                        q = q.offset(1);
                        q;
                    }
                    if *q as libc::c_int == ')' as i32
                        && *q.offset(1 as libc::c_int as isize) as libc::c_int
                            == '/' as i32
                    {
                        p = p.offset(1);
                        p;
                        continue;
                    } else {
                        current_block_63 = 15869555800145523218;
                    }
                }
                41 => {
                    current_block_63 = 15869555800145523218;
                }
                _ => {
                    if sscanf(
                        p,
                        b"%d\0" as *const u8 as *const libc::c_char,
                        &mut m1 as *mut libc::c_int,
                    ) != 1 as libc::c_int || m1 <= 0 as libc::c_int
                    {
                        return top_err.as_mut_ptr();
                    }
                    i = 0 as libc::c_int as libc::c_uint;
                    m2 = 2 as libc::c_int;
                    loop {
                        while isdigit(*p as libc::c_uchar as libc::c_int) != 0
                            && (i as libc::c_ulong)
                                < ::core::mem::size_of::<[libc::c_char; 8]>()
                                    as libc::c_ulong
                        {
                            let fresh12 = p;
                            p = p.offset(1);
                            let fresh13 = i;
                            i = i.wrapping_add(1);
                            (*s)
                                .u
                                .meter
                                .meter[nm as usize]
                                .top[fresh13 as usize] = *fresh12;
                        }
                        if *p as libc::c_int == ')' as i32 {
                            if *p.offset(1 as libc::c_int as isize) as libc::c_int
                                != '/' as i32
                            {
                                break;
                            }
                            p = p.offset(1);
                            p;
                        }
                        if *p as libc::c_int == '/' as i32 {
                            p = p.offset(1);
                            p;
                            if sscanf(
                                p,
                                b"%d\0" as *const u8 as *const libc::c_char,
                                &mut m2 as *mut libc::c_int,
                            ) != 1 as libc::c_int || m2 <= 0 as libc::c_int
                            {
                                return b"Cannot identify meter bottom\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char;
                            }
                            i = 0 as libc::c_int as libc::c_uint;
                            while isdigit(*p as libc::c_uchar as libc::c_int) != 0
                                && (i as libc::c_ulong)
                                    < ::core::mem::size_of::<[libc::c_char; 2]>()
                                        as libc::c_ulong
                            {
                                let fresh14 = p;
                                p = p.offset(1);
                                let fresh15 = i;
                                i = i.wrapping_add(1);
                                (*s)
                                    .u
                                    .meter
                                    .meter[nm as usize]
                                    .bot[fresh15 as usize] = *fresh14;
                            }
                            break;
                        } else {
                            if *p as libc::c_int != ' ' as i32
                                && *p as libc::c_int != '+' as i32
                            {
                                break;
                            }
                            if *p as libc::c_int == '\0' as i32
                                || *p.offset(1 as libc::c_int as isize) as libc::c_int
                                    == '(' as i32
                            {
                                break;
                            }
                            if (i as libc::c_ulong)
                                < ::core::mem::size_of::<[libc::c_char; 8]>()
                                    as libc::c_ulong
                            {
                                let fresh16 = p;
                                p = p.offset(1);
                                let fresh17 = i;
                                i = i.wrapping_add(1);
                                (*s)
                                    .u
                                    .meter
                                    .meter[nm as usize]
                                    .top[fresh17 as usize] = *fresh16;
                            }
                            if sscanf(
                                p,
                                b"%d\0" as *const u8 as *const libc::c_char,
                                &mut d as *mut libc::c_int,
                            ) != 1 as libc::c_int || d <= 0 as libc::c_int
                            {
                                return top_err.as_mut_ptr();
                            }
                            if *p.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                == ' ' as i32
                            {
                                if d > m1 {
                                    m1 = d;
                                }
                            } else {
                                m1 += d;
                            }
                        }
                    }
                    current_block_63 = 6545907279487748450;
                }
            }
            match current_block_63 {
                6545907279487748450 => {
                    if in_parenth == 0 {
                        wmeasure += m1 * 1536 as libc::c_int / m2;
                    }
                    nm += 1;
                    nm;
                    if *p as libc::c_int == ' ' as i32 {
                        p = p.offset(1);
                        p;
                    } else if *p as libc::c_int == '+' as i32 {
                        let fresh18 = p;
                        p = p.offset(1);
                        let fresh19 = nm;
                        nm = nm + 1;
                        (*s)
                            .u
                            .meter
                            .meter[fresh19 as usize]
                            .top[0 as libc::c_int as usize] = *fresh18;
                    }
                }
                _ => {
                    in_parenth = (*p as libc::c_int == '(' as i32) as libc::c_int;
                    let fresh10 = p;
                    p = p.offset(1);
                    let fresh11 = nm;
                    nm = nm + 1;
                    (*s)
                        .u
                        .meter
                        .meter[fresh11 as usize]
                        .top[0 as libc::c_int as usize] = *fresh10;
                }
            }
        }
    }
    meter = m1 as libc::c_short;
    if *p as libc::c_int == '=' as i32 {
        p = p.offset(1);
        if sscanf(
            p,
            b"%d/%d\0" as *const u8 as *const libc::c_char,
            &mut m1 as *mut libc::c_int,
            &mut m2 as *mut libc::c_int,
        ) != 2 as libc::c_int || m1 <= 0 as libc::c_int || m2 <= 0 as libc::c_int
        {
            return b"Cannot identify meter explicit duration\0" as *const u8
                as *const libc::c_char as *mut libc::c_char;
        }
        wmeasure = m1 * 1536 as libc::c_int / m2;
        (*s).u.meter.expdur = 1 as libc::c_int as libc::c_char;
    }
    if wmeasure > 1536 as libc::c_int * 16 as libc::c_int || wmeasure < 0 as libc::c_int
    {
        return b"Too big meter value\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    (*s).u.meter.wmeasure = wmeasure as libc::c_short;
    (*s).u.meter.nmeter = nm as libc::c_uchar;
    if parse.abc_state == 1 as libc::c_int && ulen == 0 as libc::c_int {
        if wmeasure >= 1536 as libc::c_int * 3 as libc::c_int / 4 as libc::c_int
            || wmeasure <= 1 as libc::c_int
        {
            ulen = 1536 as libc::c_int / 8 as libc::c_int;
        } else {
            ulen = 1536 as libc::c_int / 16 as libc::c_int;
        }
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn get_str(
    mut d: *mut libc::c_char,
    mut s: *mut libc::c_char,
    mut maxlen: libc::c_int,
) -> *mut libc::c_char {
    let mut c: libc::c_char = 0;
    maxlen -= 1;
    maxlen;
    while isspace(*s as libc::c_uchar as libc::c_int) != 0 {
        s = s.offset(1);
        s;
    }
    if *s as libc::c_int == '"' as i32 {
        s = s.offset(1);
        s;
        loop {
            c = *s;
            if !(c as libc::c_int != '\0' as i32) {
                break;
            }
            if c as libc::c_int == '"' as i32 {
                s = s.offset(1);
                s;
                break;
            } else {
                if c as libc::c_int == '\\' as i32 {
                    maxlen -= 1;
                    if maxlen > 0 as libc::c_int {
                        let fresh20 = d;
                        d = d.offset(1);
                        *fresh20 = c;
                    }
                    s = s.offset(1);
                    c = *s;
                }
                maxlen -= 1;
                if maxlen > 0 as libc::c_int {
                    let fresh21 = d;
                    d = d.offset(1);
                    *fresh21 = c;
                }
                s = s.offset(1);
                s;
            }
        }
    } else {
        loop {
            c = *s;
            if !(c as libc::c_int != '\0' as i32) {
                break;
            }
            if isspace(c as libc::c_uchar as libc::c_int) != 0 {
                break;
            }
            maxlen -= 1;
            if maxlen > 0 as libc::c_int {
                let fresh22 = d;
                d = d.offset(1);
                *fresh22 = c;
            }
            s = s.offset(1);
            s;
        }
    }
    *d = '\0' as i32 as libc::c_char;
    while isspace(*s as libc::c_uchar as libc::c_int) != 0 {
        s = s.offset(1);
        s;
    }
    return s;
}
unsafe extern "C" fn parse_tempo(
    mut p: *mut libc::c_char,
    mut s: *mut SYMBOL,
) -> *mut libc::c_char {
    let mut current_block: u64;
    let mut c: libc::c_char = 0;
    let mut str: [libc::c_char; 80] = [0; 80];
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut top: libc::c_int = 0;
    let mut bot: libc::c_int = 0;
    if *p as libc::c_int == '"' as i32 {
        p = get_str(
            str.as_mut_ptr(),
            p,
            ::core::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong as libc::c_int,
        );
        (*s)
            .u
            .tempo
            .str1 = getarena(
            (strlen(str.as_mut_ptr())).wrapping_add(1 as libc::c_int as libc::c_ulong)
                as libc::c_int,
        ) as *mut libc::c_char;
        strcpy((*s).u.tempo.str1, str.as_mut_ptr());
    }
    if *p as libc::c_int == 'C' as i32 || *p as libc::c_int == 'c' as i32
        || *p as libc::c_int == 'L' as i32 || *p as libc::c_int == 'l' as i32
    {
        (*s).u.tempo.beats[0 as libc::c_int as usize] = ulen as libc::c_short;
        if parse.abc_vers >= (2 as libc::c_int) << 16 as libc::c_int {
            syntax(
                b"Deprecated Q: value\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                p,
            );
        }
        p = p.offset(1);
        p;
        while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
            p = p.offset(1);
            p;
        }
        if *p as libc::c_int != '=' as i32 {
            current_block = 3039845260862735784;
        } else {
            c = '=' as i32 as libc::c_char;
            p = p.offset(-1);
            p;
            current_block = 11459959175219260272;
        }
    } else if isdigit(*p as libc::c_uchar as libc::c_int) != 0 {
        if !(strchr(p, '/' as i32)).is_null() {
            i = 0 as libc::c_int;
            loop {
                if !(isdigit(*p as libc::c_uchar as libc::c_int) != 0) {
                    current_block = 15089075282327824602;
                    break;
                }
                if sscanf(
                    p,
                    b"%d/%d%n\0" as *const u8 as *const libc::c_char,
                    &mut top as *mut libc::c_int,
                    &mut bot as *mut libc::c_int,
                    &mut n as *mut libc::c_int,
                ) != 2 as libc::c_int || bot <= 0 as libc::c_int
                {
                    current_block = 3039845260862735784;
                    break;
                }
                l = 1536 as libc::c_int * top / bot;
                if l <= 0 as libc::c_int
                    || i as libc::c_ulong
                        >= (::core::mem::size_of::<[libc::c_short; 4]>()
                            as libc::c_ulong)
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                {
                    current_block = 3039845260862735784;
                    break;
                }
                let fresh23 = i;
                i = i + 1;
                (*s).u.tempo.beats[fresh23 as usize] = l as libc::c_short;
                p = p.offset(n as isize);
                while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
                    p = p.offset(1);
                    p;
                }
            }
            match current_block {
                3039845260862735784 => {}
                _ => {
                    c = *p;
                    if c as libc::c_int != '=' as i32 {
                        current_block = 3039845260862735784;
                    } else {
                        current_block = 11459959175219260272;
                    }
                }
            }
        } else {
            (*s).u.tempo.beats[0 as libc::c_int as usize] = ulen as libc::c_short;
            if parse.abc_vers >= (2 as libc::c_int) << 16 as libc::c_int {
                syntax(
                    b"Deprecated Q: value\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    p,
                );
            }
            c = '=' as i32 as libc::c_char;
            p = p.offset(-1);
            p;
            current_block = 11459959175219260272;
        }
    } else {
        c = '\0' as i32 as libc::c_char;
        current_block = 11459959175219260272;
    }
    match current_block {
        11459959175219260272 => {
            if c as libc::c_int == '=' as i32 {
                p = p.offset(1);
                p;
                if strncmp(
                    p,
                    b"ca. \0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    (*s).u.tempo.circa = 1 as libc::c_int as libc::c_short;
                    p = p.offset(4 as libc::c_int as isize);
                }
                if sscanf(
                    p,
                    b"%d/%d%n\0" as *const u8 as *const libc::c_char,
                    &mut top as *mut libc::c_int,
                    &mut bot as *mut libc::c_int,
                    &mut n as *mut libc::c_int,
                ) == 2 as libc::c_int
                {
                    if bot <= 0 as libc::c_int {
                        current_block = 3039845260862735784;
                    } else {
                        l = 1536 as libc::c_int * top / bot;
                        if l <= 0 as libc::c_int {
                            current_block = 3039845260862735784;
                        } else {
                            (*s).u.tempo.new_beat = l as libc::c_short;
                            current_block = 4567019141635105728;
                        }
                    }
                } else if sscanf(
                    p,
                    b"%d%n\0" as *const u8 as *const libc::c_char,
                    &mut top as *mut libc::c_int,
                    &mut n as *mut libc::c_int,
                ) != 1 as libc::c_int
                {
                    current_block = 3039845260862735784;
                } else {
                    (*s).u.tempo.tempo = top as libc::c_short;
                    current_block = 4567019141635105728;
                }
                match current_block {
                    3039845260862735784 => {}
                    _ => {
                        p = p.offset(n as isize);
                        while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
                            p = p.offset(1);
                            p;
                        }
                        current_block = 14447253356787937536;
                    }
                }
            } else {
                current_block = 14447253356787937536;
            }
            match current_block {
                3039845260862735784 => {}
                _ => {
                    if *p as libc::c_int == '"' as i32 {
                        p = get_str(
                            str.as_mut_ptr(),
                            p,
                            ::core::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong
                                as libc::c_int,
                        );
                        (*s)
                            .u
                            .tempo
                            .str2 = getarena(
                            (strlen(str.as_mut_ptr()))
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as libc::c_int,
                        ) as *mut libc::c_char;
                        strcpy((*s).u.tempo.str2, str.as_mut_ptr());
                    }
                    return 0 as *mut libc::c_char;
                }
            }
        }
        _ => {}
    }
    return b"Invalid tempo\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
}
unsafe extern "C" fn get_user(
    mut p: *mut libc::c_char,
    mut s: *mut SYMBOL,
) -> *mut libc::c_char {
    let mut c: libc::c_uchar = 0;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let fresh24 = p;
    p = p.offset(1);
    c = *fresh24 as libc::c_uchar;
    if c as libc::c_int == '\\' as i32 {
        let fresh25 = p;
        p = p.offset(1);
        c = *fresh25 as libc::c_uchar;
        match c as libc::c_int {
            110 => {
                c = '\n' as i32 as libc::c_uchar;
            }
            116 => {
                c = '\t' as i32 as libc::c_uchar;
            }
            _ => {}
        }
    }
    match char_tb[c as usize] as libc::c_int {
        4 => {}
        0 | 1 | 11 | 15 | 18 => {
            char_tb[c as usize] = 4 as libc::c_int as libc::c_char;
        }
        _ => {
            return b"Bad decoration character\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
    }
    (*s).u.user.symbol = c;
    while isspace(*p as libc::c_uchar as libc::c_int) != 0
        || *p as libc::c_int == '=' as i32
    {
        p = p.offset(1);
        p;
    }
    if char_tb[*p as libc::c_uchar as usize] as libc::c_int == 15 as libc::c_int {
        p = p.offset(1);
        p;
    }
    get_deco(p, &mut (*s).u.user.value);
    if (*s).u.user.value == 0 {
        return 0 as *mut libc::c_char;
    }
    value = parse
        .deco_tb[((*s).u.user.value as libc::c_int - 128 as libc::c_int) as usize];
    if strcmp(value, b"beambreak\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        char_tb[c as usize] = 11 as libc::c_int as libc::c_char;
    } else if strcmp(value, b"ignore\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        char_tb[c as usize] = 1 as libc::c_int as libc::c_char;
    } else if strcmp(value, b"nil\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
        || strcmp(value, b"none\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        char_tb[c as usize] = 0 as libc::c_int as libc::c_char;
    } else {
        return 0 as *mut libc::c_char
    }
    (*s).u.user.value = 0 as libc::c_int as libc::c_uchar;
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn parse_voice(
    mut p: *mut libc::c_char,
    mut s: *mut SYMBOL,
) -> *mut libc::c_char {
    let mut voice: libc::c_int = 0;
    let mut error_txt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut clef_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut clef_middle: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut clef_stlines: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut clef_scale: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p_octave: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p_cue: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p_map: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p_stem: *mut libc::c_schar = 0 as *mut libc::c_schar;
    static mut kw_tb: [kw_s; 16] = [
        {
            let mut init = kw_s {
                name: b"name=\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                len: 5 as libc::c_int as libc::c_short,
                index: 0 as libc::c_int as libc::c_short,
            };
            init
        },
        {
            let mut init = kw_s {
                name: b"nm=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                len: 3 as libc::c_int as libc::c_short,
                index: 0 as libc::c_int as libc::c_short,
            };
            init
        },
        {
            let mut init = kw_s {
                name: b"subname=\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                len: 8 as libc::c_int as libc::c_short,
                index: 1 as libc::c_int as libc::c_short,
            };
            init
        },
        {
            let mut init = kw_s {
                name: b"sname=\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                len: 6 as libc::c_int as libc::c_short,
                index: 1 as libc::c_int as libc::c_short,
            };
            init
        },
        {
            let mut init = kw_s {
                name: b"snm=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                len: 4 as libc::c_int as libc::c_short,
                index: 1 as libc::c_int as libc::c_short,
            };
            init
        },
        {
            let mut init = kw_s {
                name: b"merge\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                len: 5 as libc::c_int as libc::c_short,
                index: 2 as libc::c_int as libc::c_short,
            };
            init
        },
        {
            let mut init = kw_s {
                name: b"up\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                len: 2 as libc::c_int as libc::c_short,
                index: 3 as libc::c_int as libc::c_short,
            };
            init
        },
        {
            let mut init = kw_s {
                name: b"down\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                len: 4 as libc::c_int as libc::c_short,
                index: 4 as libc::c_int as libc::c_short,
            };
            init
        },
        {
            let mut init = kw_s {
                name: b"stem=\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                len: 5 as libc::c_int as libc::c_short,
                index: 5 as libc::c_int as libc::c_short,
            };
            init
        },
        {
            let mut init = kw_s {
                name: b"gstem=\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                len: 6 as libc::c_int as libc::c_short,
                index: 6 as libc::c_int as libc::c_short,
            };
            init
        },
        {
            let mut init = kw_s {
                name: b"auto\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                len: 4 as libc::c_int as libc::c_short,
                index: 7 as libc::c_int as libc::c_short,
            };
            init
        },
        {
            let mut init = kw_s {
                name: b"dyn=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                len: 4 as libc::c_int as libc::c_short,
                index: 8 as libc::c_int as libc::c_short,
            };
            init
        },
        {
            let mut init = kw_s {
                name: b"lyrics=\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                len: 7 as libc::c_int as libc::c_short,
                index: 9 as libc::c_int as libc::c_short,
            };
            init
        },
        {
            let mut init = kw_s {
                name: b"scale=\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                len: 6 as libc::c_int as libc::c_short,
                index: 10 as libc::c_int as libc::c_short,
            };
            init
        },
        {
            let mut init = kw_s {
                name: b"gchord=\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                len: 7 as libc::c_int as libc::c_short,
                index: 11 as libc::c_int as libc::c_short,
            };
            init
        },
        {
            let mut init = kw_s {
                name: 0 as *const libc::c_char as *mut libc::c_char,
                len: 0,
                index: 0,
            };
            init
        },
    ];
    let mut kw: *mut kw_s = 0 as *mut kw_s;
    (*curvoice).ulen = ulen;
    (*curvoice).microscale = microscale;
    if voice_tb[0 as libc::c_int as usize].id[0 as libc::c_int as usize] as libc::c_int
        == '\0' as i32
    {
        match (*(*s).abc_prev).abc_type as libc::c_int {
            7 | 4 | 5 | 6 => {
                voice_tb[0 as libc::c_int as usize]
                    .id[0 as libc::c_int as usize] = '1' as i32 as libc::c_char;
            }
            _ => {}
        }
    }
    let mut current_block_22: u64;
    let mut id: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sep: libc::c_char = 0;
    id = p;
    while isalnum(*p as libc::c_uchar as libc::c_int) != 0
        || *p as libc::c_int == '_' as i32
    {
        p = p.offset(1);
        p;
    }
    sep = *p;
    *p = '\0' as i32 as libc::c_char;
    if voice_tb[0 as libc::c_int as usize].id[0 as libc::c_int as usize] as libc::c_int
        == '\0' as i32
    {
        voice = 0 as libc::c_int;
        current_block_22 = 5634871135123216486;
    } else {
        voice = 0 as libc::c_int;
        loop {
            if !(voice <= nvoice as libc::c_int) {
                current_block_22 = 7175849428784450219;
                break;
            }
            if strcmp(id, (voice_tb[voice as usize].id).as_mut_ptr()) == 0 as libc::c_int
            {
                current_block_22 = 4696528446822315521;
                break;
            }
            voice += 1;
            voice;
        }
        match current_block_22 {
            4696528446822315521 => {}
            _ => {
                if voice >= 32 as libc::c_int {
                    syntax(
                        b"Too many voices\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        id,
                    );
                    voice -= 1;
                    voice;
                }
                current_block_22 = 5634871135123216486;
            }
        }
    }
    match current_block_22 {
        5634871135123216486 => {
            nvoice = voice as libc::c_short;
            strncpy(
                (voice_tb[voice as usize].id).as_mut_ptr(),
                id,
                (::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            voice_tb[voice as usize].mvoice = voice as libc::c_uchar;
        }
        _ => {}
    }
    strcpy(((*s).u.voice.id).as_mut_ptr(), (voice_tb[voice as usize].id).as_mut_ptr());
    *p = sep;
    curvoice = &mut *voice_tb.as_mut_ptr().offset(voice as isize) as *mut VOICE_S;
    (*s).u.voice.voice = voice as libc::c_uchar;
    if parse.abc_state == 2 as libc::c_int {
        ulen = (*curvoice).ulen;
        microscale = (*curvoice).microscale;
    }
    clef_scale = 0 as *mut libc::c_char;
    clef_stlines = clef_scale;
    clef_middle = clef_stlines;
    clef_name = clef_middle;
    p_map = 0 as *mut libc::c_char;
    p_cue = p_map;
    p_octave = p_cue;
    p_stem = &mut (*s).u.voice.stem;
    loop {
        while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
            p = p.offset(1);
            p;
        }
        if *p as libc::c_int == '\0' as i32 {
            break;
        }
        p = parse_extra(
            p,
            &mut clef_name,
            &mut clef_middle,
            &mut clef_stlines,
            &mut clef_scale,
            &mut p_octave,
            &mut p_cue,
            &mut p_map,
        );
        if *p as libc::c_int == '\0' as i32 {
            break;
        }
        kw = kw_tb.as_mut_ptr();
        while !((*kw).name).is_null() {
            if strncmp(p, (*kw).name, (*kw).len as libc::c_ulong) == 0 as libc::c_int {
                break;
            }
            kw = kw.offset(1);
            kw;
        }
        if ((*kw).name).is_null() {
            while isspace(*p as libc::c_uchar as libc::c_int) == 0
                && *p as libc::c_int != '\0' as i32
            {
                p = p.offset(1);
                p;
            }
        } else {
            p = p.offset((*kw).len as libc::c_int as isize);
            match (*kw).index as libc::c_int {
                0 => {
                    p = get_str(tex_buf.as_mut_ptr(), p, 512 as libc::c_int);
                    (*s)
                        .u
                        .voice
                        .fname = getarena(
                        (strlen(tex_buf.as_mut_ptr()))
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            as libc::c_int,
                    ) as *mut libc::c_char;
                    strcpy((*s).u.voice.fname, tex_buf.as_mut_ptr());
                }
                1 => {
                    p = get_str(tex_buf.as_mut_ptr(), p, 512 as libc::c_int);
                    (*s)
                        .u
                        .voice
                        .nname = getarena(
                        (strlen(tex_buf.as_mut_ptr()))
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            as libc::c_int,
                    ) as *mut libc::c_char;
                    strcpy((*s).u.voice.nname, tex_buf.as_mut_ptr());
                }
                2 => {
                    (*s).u.voice.merge = 1 as libc::c_int as libc::c_char;
                }
                3 => {
                    *p_stem = 1 as libc::c_int as libc::c_schar;
                }
                4 => {
                    *p_stem = -(1 as libc::c_int) as libc::c_schar;
                }
                5 => {
                    p_stem = &mut (*s).u.voice.stem;
                }
                6 => {
                    p_stem = &mut (*s).u.voice.gstem;
                }
                7 => {
                    *p_stem = 2 as libc::c_int as libc::c_schar;
                }
                8 => {
                    p_stem = &mut (*s).u.voice.dyn_0;
                }
                9 => {
                    p_stem = &mut (*s).u.voice.lyrics;
                }
                10 => {
                    let mut sc: libc::c_float = 0.;
                    sc = atof(p) as libc::c_float;
                    if sc as libc::c_double >= 0.5f64
                        && sc <= 2 as libc::c_int as libc::c_float
                    {
                        (*s).u.voice.scale = sc;
                    } else {
                        error_txt = b"Bad value for voice scale\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char;
                    }
                    while isspace(*p as libc::c_uchar as libc::c_int) == 0
                        && *p as libc::c_int != '\0' as i32
                    {
                        p = p.offset(1);
                        p;
                    }
                }
                11 => {
                    p_stem = &mut (*s).u.voice.gchord;
                }
                _ => {}
            }
        }
    }
    (*s).u.voice.octave = parse_octave(p_octave) as libc::c_schar;
    if !p_cue.is_null() {
        if strncmp(
            p_cue,
            b"on\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            (*s).u.voice.cue = 1 as libc::c_int as libc::c_schar;
        } else {
            (*s).u.voice.cue = -(1 as libc::c_int) as libc::c_schar;
        }
    }
    if !clef_stlines.is_null() {
        (*s).u.voice.stafflines = clef_stlines;
    }
    if !clef_scale.is_null() {
        let mut sc_0: libc::c_float = 0.;
        sc_0 = atof(clef_scale) as libc::c_float;
        if sc_0 as libc::c_double >= 0.5f64 && sc_0 <= 3 as libc::c_int as libc::c_float
        {
            (*s).u.voice.staffscale = sc_0;
        } else {
            syntax(
                b"Bad value of staffscale\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                clef_scale,
            );
        }
    }
    if !clef_name.is_null() || !clef_middle.is_null() {
        s = abc_new(3 as libc::c_int, 0 as *mut libc::c_char);
        parse_clef(s, clef_name, clef_middle);
    }
    if !p_map.is_null() {
        strcpy(
            tex_buf.as_mut_ptr(),
            b"%%voicemap \0" as *const u8 as *const libc::c_char,
        );
        get_str(
            &mut *tex_buf.as_mut_ptr().offset(11 as libc::c_int as isize),
            p_map,
            512 as libc::c_int - 12 as libc::c_int,
        );
        abc_new(2 as libc::c_int, tex_buf.as_mut_ptr());
    }
    return error_txt;
}
unsafe extern "C" fn parse_bar(mut p: *mut libc::c_char) -> *mut libc::c_char {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bar_type: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut repeat_value: [libc::c_char; 32] = [0; 32];
    p = p.offset(-1);
    q = p;
    bar_type = 0 as libc::c_int;
    loop {
        let fresh26 = p;
        p = p.offset(1);
        match *fresh26 as libc::c_int {
            124 => {
                bar_type <<= 4 as libc::c_int;
                bar_type |= 1 as libc::c_int;
            }
            91 => {
                bar_type <<= 4 as libc::c_int;
                bar_type |= 2 as libc::c_int;
            }
            93 => {
                bar_type <<= 4 as libc::c_int;
                bar_type |= 3 as libc::c_int;
            }
            58 => {
                bar_type <<= 4 as libc::c_int;
                bar_type |= 4 as libc::c_int;
            }
            _ => {
                break;
            }
        }
    }
    p = p.offset(-1);
    p;
    if bar_type & 0xf as libc::c_int == 2 as libc::c_int && bar_type != 2 as libc::c_int
        && *p as libc::c_int != ' ' as i32
    {
        bar_type >>= 4 as libc::c_int;
        p = p.offset(-1);
        p;
    }
    if bar_type
        == ((2 as libc::c_int) << 8 as libc::c_int)
            + ((1 as libc::c_int) << 4 as libc::c_int) + 3 as libc::c_int
    {
        bar_type = ((2 as libc::c_int) << 4 as libc::c_int) + 3 as libc::c_int;
    }
    if vover as libc::c_int > 0 as libc::c_int {
        curvoice = &mut *voice_tb.as_mut_ptr().offset((*curvoice).mvoice as isize)
            as *mut VOICE_S;
        vover = 0 as libc::c_int as libc::c_schar;
    }
    s = abc_new(6 as libc::c_int, gchord);
    if !gchord.is_null() {
        gchord = 0 as *mut libc::c_char;
    }
    if bar_type == 4 as libc::c_int {
        bar_type = 1 as libc::c_int;
        (*s).u.bar.dotted = 1 as libc::c_int as libc::c_char;
    } else if *q as libc::c_int == ']' as i32 {
        i = (p.offset_from(q) as libc::c_long - 1 as libc::c_int as libc::c_long)
            as libc::c_int;
        if i > 0 as libc::c_int {
            (*s).u.bar.type_0
                &= ((1 as libc::c_int) << i * 4 as libc::c_int) - 1 as libc::c_int;
        }
        (*s)
            .flags = ((*s).flags as libc::c_int | 0x200 as libc::c_int)
            as libc::c_ushort;
        (*s).sflags |= 0x8000 as libc::c_int as libc::c_uint;
    } else if bar_type & 0xf as libc::c_int == 4 as libc::c_int
        || *q as libc::c_int == ':' as i32
    {
        (*s)
            .flags = ((*s).flags as libc::c_int | 0x200 as libc::c_int)
            as libc::c_ushort;
        (*s).sflags |= 0x8000 as libc::c_int as libc::c_uint;
        if *q as libc::c_int == ':' as i32 {
            (*s).sflags |= 0x100 as libc::c_int as libc::c_uint;
        }
    }
    (*s).u.bar.type_0 = bar_type;
    if dc.n as libc::c_int > 0 as libc::c_int {
        memcpy(
            &mut (*s).u.bar.dc as *mut decos as *mut libc::c_void,
            &mut dc as *mut decos as *const libc::c_void,
            ::core::mem::size_of::<decos>() as libc::c_ulong,
        );
        dc.n = 0 as libc::c_int as libc::c_char;
    }
    if lyric_started == 0 {
        lyric_started = 1 as libc::c_int as libc::c_char;
        (*s).flags = ((*s).flags as libc::c_int | 0x10 as libc::c_int) as libc::c_ushort;
    }
    if isdigit(*p as libc::c_uchar as libc::c_int) == 0
        && (*p as libc::c_int != '"' as i32
            || *p.offset(-(1 as libc::c_int) as isize) as libc::c_int != '[' as i32)
    {
        return p;
    }
    if *p as libc::c_int == '"' as i32 {
        p = get_str(
            repeat_value.as_mut_ptr(),
            p,
            ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
        );
    } else {
        let mut q_0: *mut libc::c_char = 0 as *mut libc::c_char;
        q_0 = repeat_value.as_mut_ptr();
        while isdigit(*p as libc::c_uchar as libc::c_int) != 0
            || *p as libc::c_int == ',' as i32 || *p as libc::c_int == '-' as i32
            || *p as libc::c_int == '.' as i32
                && isdigit(
                    *p.offset(1 as libc::c_int as isize) as libc::c_uchar as libc::c_int,
                ) != 0
        {
            if q_0
                < &mut *repeat_value
                    .as_mut_ptr()
                    .offset(
                        (::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ) as *mut libc::c_char
            {
                let fresh27 = p;
                p = p.offset(1);
                let fresh28 = q_0;
                q_0 = q_0.offset(1);
                *fresh28 = *fresh27;
            } else {
                p = p.offset(1);
                p;
            }
        }
        *q_0 = '\0' as i32 as libc::c_char;
    }
    if bar_type != 2 as libc::c_int || !((*s).text).is_null() {
        s = abc_new(6 as libc::c_int, repeat_value.as_mut_ptr());
        (*s).u.bar.type_0 = 2 as libc::c_int;
    } else {
        (*s)
            .text = getarena(
            (strlen(repeat_value.as_mut_ptr()))
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        ) as *mut libc::c_char;
        strcpy((*s).text, repeat_value.as_mut_ptr());
    }
    (*s).u.bar.repeat_bar = 1 as libc::c_int as libc::c_char;
    (*s)
        .flags = ((*s).flags as libc::c_int
        | (0x100 as libc::c_int | 0x200 as libc::c_int)) as libc::c_ushort;
    (*s).sflags |= (0x10000000 as libc::c_int | 0x8000 as libc::c_int) as libc::c_uint;
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn parse_acc_pit(
    mut p: *mut libc::c_char,
    mut pit: *mut libc::c_int,
    mut acc: *mut libc::c_int,
) -> *mut libc::c_char {
    match *p as libc::c_int {
        94 => {
            p = p.offset(1);
            p;
            if *p as libc::c_int == '^' as i32 {
                p = p.offset(1);
                p;
                *acc = A_DS as libc::c_int;
            } else {
                *acc = A_SH as libc::c_int;
            }
        }
        61 => {
            p = p.offset(1);
            p;
            *acc = A_NT as libc::c_int;
        }
        95 => {
            p = p.offset(1);
            p;
            if *p as libc::c_int == '_' as i32 {
                p = p.offset(1);
                p;
                *acc = A_DF as libc::c_int;
            } else {
                *acc = A_FT as libc::c_int;
            }
        }
        _ => {
            *acc = 0 as libc::c_int;
        }
    }
    if *acc != 0 as libc::c_int
        && (isdigit(*p as libc::c_uchar as libc::c_int) != 0
            || *p as libc::c_int == '/' as i32
                && microscale as libc::c_int == 0 as libc::c_int)
    {
        let mut n: libc::c_int = 0;
        let mut d: libc::c_int = 0;
        let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
        d = 1 as libc::c_int;
        n = d;
        if *p as libc::c_int != '/' as i32 {
            n = strtol(p, &mut q, 10 as libc::c_int) as libc::c_int;
            p = q;
        }
        if *p as libc::c_int == '/' as i32 {
            p = p.offset(1);
            p;
            if isdigit(*p as libc::c_uchar as libc::c_int) == 0 {
                d = 2 as libc::c_int;
            } else {
                d = strtol(p, &mut q, 10 as libc::c_int) as libc::c_int;
                p = q;
            }
        }
        if microscale as libc::c_int == 0 as libc::c_int {
            d -= 1;
            d;
            d += (n - 1 as libc::c_int) << 8 as libc::c_int;
            if d == 0 as libc::c_int {
                n = 32 as libc::c_int - 1 as libc::c_int;
            } else {
                n = 1 as libc::c_int;
                while n < 32 as libc::c_int {
                    if parse.micro_tb[n as usize] as libc::c_int == d {
                        break;
                    }
                    if parse.micro_tb[n as usize] as libc::c_int == 0 as libc::c_int {
                        parse.micro_tb[n as usize] = d as libc::c_ushort;
                        break;
                    } else {
                        n += 1;
                        n;
                    }
                }
            }
            if n == 32 as libc::c_int {
                syntax(
                    b"Too many microtone accidentals\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    p,
                );
                n = 0 as libc::c_int;
            }
        }
        *acc += n << 3 as libc::c_int;
    }
    let mut p_n: *mut libc::c_char = 0 as *mut libc::c_char;
    p_n = strchr(all_notes.as_ptr(), *p as libc::c_int);
    if p_n.is_null() || *p as libc::c_int == '\0' as i32 {
        syntax(
            (if *acc != 0 {
                b"Missing note after accidental\0" as *const u8 as *const libc::c_char
            } else {
                b"Not a note\0" as *const u8 as *const libc::c_char
            }) as *mut libc::c_char,
            p,
        );
        *acc = -(1 as libc::c_int);
        if *p as libc::c_int == '\0' as i32 {
            p = p.offset(-1);
            p;
        }
    } else {
        *pit = (p_n.offset_from(all_notes.as_ptr()) as libc::c_long
            + 16 as libc::c_int as libc::c_long) as libc::c_int;
    }
    p = p.offset(1);
    p;
    while *p as libc::c_int == '\'' as i32 {
        *pit += 7 as libc::c_int;
        p = p.offset(1);
        p;
    }
    while *p as libc::c_int == ',' as i32 {
        *pit -= 7 as libc::c_int;
        p = p.offset(1);
        p;
    }
    return p;
}
unsafe extern "C" fn parse_deco(
    mut p: *mut libc::c_char,
    mut deco: *mut decos,
    mut m: libc::c_int,
) -> *mut libc::c_char {
    let mut n: libc::c_int = 0;
    let mut t: libc::c_uchar = 0;
    n = (*deco).n as libc::c_int;
    loop {
        let fresh29 = p;
        p = p.offset(1);
        t = *fresh29 as libc::c_uchar;
        if char_tb[t as usize] as libc::c_int != 4 as libc::c_int
            && char_tb[t as usize] as libc::c_int != 15 as libc::c_int
        {
            break;
        }
        if char_tb[t as usize] as libc::c_int == 15 as libc::c_int {
            p = get_deco(p, &mut t);
        }
        if n >= 32 as libc::c_int {
            syntax(
                b"Too many decorations for the note\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                p,
            );
        } else if t as libc::c_int != 0 as libc::c_int {
            (*deco).tm[n as usize].t = t;
            let fresh30 = n;
            n = n + 1;
            (*deco).tm[fresh30 as usize].m = m as libc::c_schar;
        }
    }
    (*deco).n = n as libc::c_char;
    return p.offset(-(1 as libc::c_int as isize));
}
unsafe extern "C" fn parse_decoline(mut p: *mut libc::c_char) -> *mut libc::c_char {
    let mut is: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut t: libc::c_uchar = 0;
    let mut n: libc::c_int = 0;
    is = deco_cont;
    if is.is_null() {
        is = deco_start;
    } else {
        deco_cont = 0 as *mut SYMBOL;
    }
    while *p as libc::c_int != '\0' as i32 {
        while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
            p = p.offset(1);
            p;
        }
        if *p as libc::c_int == '\0' as i32 {
            break;
        }
        match *p as libc::c_int {
            124 => {
                while !is.is_null()
                    && ((*is).abc_type as libc::c_int != 6 as libc::c_int
                        || (*is).u.bar.type_0 == 2 as libc::c_int)
                {
                    is = (*is).abc_next;
                }
                if is.is_null() {
                    syntax(
                        b"Not enough bar lines for deco line\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        p,
                    );
                    return 0 as *mut libc::c_char;
                }
                is = (*is).abc_next;
                p = p.offset(1);
                p;
                continue;
            }
            42 => {
                while !is.is_null() && (*is).abc_type as libc::c_int != 4 as libc::c_int
                {
                    is = (*is).abc_next;
                }
                if is.is_null() {
                    syntax(
                        b"Not enough notes for deco line\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        p,
                    );
                    return 0 as *mut libc::c_char;
                }
                is = (*is).abc_next;
                p = p.offset(1);
                p;
                continue;
            }
            92 => {
                if *p.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
                    if is.is_null() {
                        return b"Not enough notes for deco line\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char;
                    }
                    deco_cont = is;
                    return 0 as *mut libc::c_char;
                }
                syntax(
                    b"'\\' ignored\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    p,
                );
                p = p.offset(1);
                p;
                continue;
            }
            34 => {
                p = parse_gchord(p.offset(1 as libc::c_int as isize));
            }
            _ => {
                if char_tb[*p as libc::c_uchar as usize] as libc::c_int
                    == 15 as libc::c_int
                {
                    p = get_deco(p.offset(1 as libc::c_int as isize), &mut t);
                } else {
                    let fresh31 = p;
                    p = p.offset(1);
                    t = *fresh31 as libc::c_uchar;
                }
            }
        }
        while !is.is_null()
            && ((*is).abc_type as libc::c_int != 4 as libc::c_int
                || (*is).flags as libc::c_int & 0x20 as libc::c_int != 0)
        {
            is = (*is).abc_next;
        }
        if is.is_null() {
            return b"Not enough notes for deco line\0" as *const u8
                as *const libc::c_char as *mut libc::c_char;
        }
        if !gchord.is_null() {
            if !((*is).text).is_null() {
                let mut gch: *mut libc::c_char = 0 as *mut libc::c_char;
                n = strlen((*is).text) as libc::c_int;
                gch = getarena(
                    (n as libc::c_ulong)
                        .wrapping_add(strlen(gchord))
                        .wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_int,
                ) as *mut libc::c_char;
                strcpy(gch, (*is).text);
                *gch.offset(n as isize) = '\n' as i32 as libc::c_char;
                strcpy(gch.offset(n as isize).offset(1 as libc::c_int as isize), gchord);
                gchord = gch;
            }
            (*is).text = gchord;
            gchord = 0 as *mut libc::c_char;
        } else {
            n = (*is).u.note.dc.n as libc::c_int;
            if n >= 32 as libc::c_int {
                syntax(
                    b"Too many decorations for the note\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    p,
                );
            } else if t as libc::c_int != 0 as libc::c_int {
                (*is).u.note.dc.tm[n as usize].t = t;
                (*is).u.note.dc.tm[n as usize].m = -(1 as libc::c_int) as libc::c_schar;
                n += 1;
                (*is).u.note.dc.n = n as libc::c_char;
            }
        }
        is = (*is).abc_next;
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn parse_gchord(mut p: *mut libc::c_char) -> *mut libc::c_char {
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: libc::c_int = 0;
    let mut l2: libc::c_int = 0;
    q = p;
    while *p as libc::c_int != '"' as i32 {
        if *p as libc::c_int == '\\' as i32 {
            p = p.offset(1);
            p;
        }
        if *p as libc::c_int == '\0' as i32 {
            syntax(
                b"No end of guitar chord\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                p,
            );
            break;
        } else {
            p = p.offset(1);
            p;
        }
    }
    l = p.offset_from(q) as libc::c_long as libc::c_int;
    if !gchord.is_null() {
        let mut gch: *mut libc::c_char = 0 as *mut libc::c_char;
        l2 = strlen(gchord) as libc::c_int;
        gch = getarena(l2 + 1 as libc::c_int + l + 1 as libc::c_int)
            as *mut libc::c_char;
        strcpy(gch, gchord);
        let fresh32 = l2;
        l2 = l2 + 1;
        *gch.offset(fresh32 as isize) = '\n' as i32 as libc::c_char;
        strncpy(&mut *gch.offset(l2 as isize), q, l as libc::c_ulong);
        *gch.offset((l2 + l) as isize) = '\0' as i32 as libc::c_char;
        gchord = gch;
    } else {
        gchord = getarena(l + 1 as libc::c_int) as *mut libc::c_char;
        strncpy(gchord, q, l as libc::c_ulong);
        *gchord.offset(l as isize) = '\0' as i32 as libc::c_char;
    }
    if *p as libc::c_int != '\0' as i32 {
        p = p.offset(1);
        p;
    }
    return p;
}
unsafe extern "C" fn parse_len(
    mut p: *mut libc::c_char,
    mut dur_u: libc::c_int,
    mut p_len: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut len: libc::c_int = 0;
    let mut fac: libc::c_int = 0;
    let mut err: libc::c_int = 0 as libc::c_int;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    len = dur_u;
    if isdigit(*p as libc::c_uchar as libc::c_int) != 0 {
        len = (len as libc::c_long * strtol(p, &mut q, 10 as libc::c_int))
            as libc::c_int;
        p = q;
    }
    if *p as libc::c_int == '/' as i32 {
        if isdigit(*p.offset(1 as libc::c_int as isize) as libc::c_uchar as libc::c_int)
            != 0
        {
            fac = strtol(p.offset(1 as libc::c_int as isize), &mut q, 10 as libc::c_int)
                as libc::c_int;
            p = q;
            if fac == 0 as libc::c_int || fac & fac - 1 as libc::c_int != 0 {
                err = 1 as libc::c_int;
            } else {
                len /= fac;
            }
        } else {
            while *p as libc::c_int == '/' as i32 {
                if len & 1 as libc::c_int != 0 {
                    err = 1 as libc::c_int;
                }
                len /= 2 as libc::c_int;
                p = p.offset(1);
                p;
            }
        }
        if err != 0 || len == 0 {
            syntax(
                b"Bad length divisor\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                p.offset(-(1 as libc::c_int as isize)),
            );
            len = dur_u;
        }
    }
    if len <= 0 as libc::c_int || len > 10000 as libc::c_int {
        syntax(
            b"Bad length\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            p,
        );
        len = dur_u;
    }
    *p_len = len;
    return p;
}
unsafe extern "C" fn parse_line(mut p: *mut libc::c_char) -> libc::c_int {
    let mut current_block: u64;
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_char = 0;
    let mut dot: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut last_note_sav: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut dc_sav: decos = decos {
        n: 0,
        tm: [C2RustUnnamed { t: 0, m: 0 }; 32],
    };
    let mut i: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut flags_sav: libc::c_int = 0 as libc::c_int;
    let mut slur: libc::c_int = 0;
    static mut qtb: [libc::c_char; 10] = [
        0 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        3 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        3 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        3 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
    ];
    colnum = 0 as libc::c_int;
    's_300: {
        match *p as libc::c_int {
            0 => {
                's_39: {
                    match parse.abc_state {
                        0 => {
                            if !(parse.last_sym).is_null()
                                && (*parse.last_sym).abc_type as libc::c_int
                                    != 0 as libc::c_int
                            {
                                abc_new(0 as libc::c_int, 0 as *mut libc::c_char);
                            }
                        }
                        1 => {}
                        _ => {
                            break 's_39;
                        }
                    }
                    return 0 as libc::c_int;
                }
                return 1 as libc::c_int;
            }
            37 => {
                if *p.offset(1 as libc::c_int as isize) as libc::c_int == '%' as i32 {
                    s = abc_new(2 as libc::c_int, p);
                    p = p.offset(2 as libc::c_int as isize);
                    if strncasecmp(
                        p,
                        b"decoration \0" as *const u8 as *const libc::c_char,
                        11 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        p = p.offset(11 as libc::c_int as isize);
                        while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
                            p = p.offset(1);
                            p;
                        }
                        match *p as libc::c_int {
                            33 => {
                                char_tb['!' as i32
                                    as usize] = 15 as libc::c_int as libc::c_char;
                                char_tb['+' as i32
                                    as usize] = 0 as libc::c_int as libc::c_char;
                            }
                            43 => {
                                char_tb['+' as i32
                                    as usize] = 15 as libc::c_int as libc::c_char;
                                char_tb['!' as i32
                                    as usize] = 0 as libc::c_int as libc::c_char;
                            }
                            _ => {}
                        }
                        return 0 as libc::c_int;
                    }
                    if strncasecmp(
                        p,
                        b"linebreak \0" as *const u8 as *const libc::c_char,
                        10 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        i = 0 as libc::c_int;
                        while (i as libc::c_ulong)
                            < ::core::mem::size_of::<[libc::c_char; 256]>()
                                as libc::c_ulong
                        {
                            if char_tb[i as usize] as libc::c_int == 18 as libc::c_int {
                                char_tb[i
                                    as usize] = (if i != '!' as i32 {
                                    0 as libc::c_int
                                } else {
                                    15 as libc::c_int
                                }) as libc::c_char;
                            }
                            i += 1;
                            i;
                        }
                        p = p.offset(10 as libc::c_int as isize);
                        loop {
                            while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
                                p = p.offset(1);
                                p;
                            }
                            if *p as libc::c_int == '\0' as i32 {
                                break;
                            }
                            let mut current_block_34: u64;
                            match *p as libc::c_int {
                                33 | 36 | 42 | 59 | 63 | 64 => {
                                    let fresh33 = p;
                                    p = p.offset(1);
                                    char_tb[*fresh33 as libc::c_uchar
                                        as usize] = 18 as libc::c_int as libc::c_char;
                                    current_block_34 = 572715077006366937;
                                }
                                60 => {
                                    if strncmp(
                                        p,
                                        b"<none>\0" as *const u8 as *const libc::c_char,
                                        6 as libc::c_int as libc::c_ulong,
                                    ) == 0 as libc::c_int
                                    {
                                        return 0 as libc::c_int;
                                    }
                                    if strncmp(
                                        p,
                                        b"<EOL>\0" as *const u8 as *const libc::c_char,
                                        5 as libc::c_int as libc::c_ulong,
                                    ) == 0 as libc::c_int
                                    {
                                        char_tb['\n' as i32
                                            as usize] = 18 as libc::c_int as libc::c_char;
                                        p = p.offset(5 as libc::c_int as isize);
                                        current_block_34 = 572715077006366937;
                                    } else {
                                        current_block_34 = 7230608890562899020;
                                    }
                                }
                                _ => {
                                    current_block_34 = 7230608890562899020;
                                }
                            }
                            match current_block_34 {
                                572715077006366937 => {}
                                _ => {
                                    if strcmp(p, b"lock\0" as *const u8 as *const libc::c_char)
                                        != 0 as libc::c_int
                                    {
                                        syntax(
                                            b"Invalid character in %%%%linebreak\0" as *const u8
                                                as *const libc::c_char as *mut libc::c_char,
                                            p,
                                        );
                                    }
                                    return 0 as libc::c_int;
                                }
                            }
                        }
                        return 0 as libc::c_int;
                    }
                    if strncasecmp(
                        p,
                        b"microscale \0" as *const u8 as *const libc::c_char,
                        11 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        let mut v: libc::c_int = 0;
                        p = p.offset(11 as libc::c_int as isize);
                        while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
                            p = p.offset(1);
                            p;
                        }
                        sscanf(
                            p,
                            b"%d\0" as *const u8 as *const libc::c_char,
                            &mut v as *mut libc::c_int,
                        );
                        if v < 4 as libc::c_int || v >= 256 as libc::c_int
                            || v & 1 as libc::c_int != 0
                        {
                            syntax(
                                b"Invalid value in %%microscale\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                                p,
                            );
                        } else {
                            microscale = v as libc::c_uchar;
                        }
                        return 0 as libc::c_int;
                    }
                    if strncasecmp(
                        p,
                        b"user \0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        p = p.offset(5 as libc::c_int as isize);
                        while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
                            p = p.offset(1);
                            p;
                        }
                        get_user(p, s);
                        return 0 as libc::c_int;
                    }
                    return 0 as libc::c_int;
                }
            }
            92 => {}
            _ => {
                break 's_300;
            }
        }
        return 0 as libc::c_int;
    }
    if *p.offset(1 as libc::c_int as isize) as libc::c_int == ':' as i32
        && *p as libc::c_int != '|' as i32 && *p as libc::c_int != ':' as i32
    {
        let mut new_tune: libc::c_int = 0;
        new_tune = parse_info(p);
        if *p as libc::c_int != 'V' as i32 || parse.abc_state != 2 as libc::c_int {
            return new_tune;
        }
        c = *p
            .offset(
                (strlen(p)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            );
        if c as libc::c_int != '|' as i32 && c as libc::c_int != ']' as i32 {
            return new_tune;
        }
        while isspace(*p as libc::c_uchar as libc::c_int) == 0
            && *p as libc::c_int != '\0' as i32
        {
            p = p.offset(1);
            p;
        }
        while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
            p = p.offset(1);
            p;
        }
    }
    if parse.abc_state != 2 as libc::c_int {
        return 0 as libc::c_int;
    }
    flags = 0 as libc::c_int;
    if parse.abc_vers <= (2 as libc::c_int) << 16 as libc::c_int {
        lyric_started = 0 as libc::c_int as libc::c_char;
    }
    deco_cont = 0 as *mut SYMBOL;
    deco_start = deco_cont;
    slur = 0 as libc::c_int;
    while *p as libc::c_int != '\0' as i32 {
        colnum = p.offset_from(abc_line) as libc::c_long as libc::c_int;
        let fresh34 = p;
        p = p.offset(1);
        match char_tb[*fresh34 as libc::c_uchar as usize] as libc::c_int {
            5 => {
                if flags & 0x20 as libc::c_int != 0 {
                    current_block = 18318841331983735395;
                } else {
                    p = parse_gchord(p);
                    continue;
                }
            }
            3 => {
                if flags & 0x20 as libc::c_int != 0 {
                    current_block = 18318841331983735395;
                } else {
                    last_note_sav = (*curvoice).last_note;
                    (*curvoice).last_note = 0 as *mut SYMBOL;
                    memcpy(
                        &mut dc_sav as *mut decos as *mut libc::c_void,
                        &mut dc as *mut decos as *const libc::c_void,
                        ::core::mem::size_of::<decos>() as libc::c_ulong,
                    );
                    dc.n = 0 as libc::c_int as libc::c_char;
                    flags_sav = flags;
                    flags = 0x20 as libc::c_int;
                    if *p as libc::c_int == '/' as i32 {
                        flags |= 0x80 as libc::c_int;
                        p = p.offset(1);
                        p;
                    }
                    continue;
                }
            }
            17 => {
                if flags & 0x20 as libc::c_int == 0 {
                    current_block = 18318841331983735395;
                } else {
                    (*parse.last_sym)
                        .flags = ((*parse.last_sym).flags as libc::c_int
                        | 0x40 as libc::c_int) as libc::c_ushort;
                    if dc.n as libc::c_int != 0 as libc::c_int {
                        syntax(
                            b"Decoration ignored\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            p,
                        );
                    }
                    (*curvoice).last_note = last_note_sav;
                    memcpy(
                        &mut dc as *mut decos as *mut libc::c_void,
                        &mut dc_sav as *mut decos as *const libc::c_void,
                        ::core::mem::size_of::<decos>() as libc::c_ulong,
                    );
                    flags = flags_sav;
                    continue;
                }
            }
            15 => {
                if *p.offset(-(1 as libc::c_int) as isize) as libc::c_int == '!' as i32
                    && char_tb['\n' as i32 as usize] as libc::c_int == 18 as libc::c_int
                    && check_nl(p) != 0
                {
                    s = abc_new(7 as libc::c_int, 0 as *mut libc::c_char);
                    (*s).u.eoln.type_0 = 2 as libc::c_int as libc::c_char;
                    continue;
                } else {
                    current_block = 17404231275461091374;
                }
            }
            4 => {
                current_block = 17404231275461091374;
            }
            18 => {
                s = abc_new(7 as libc::c_int, 0 as *mut libc::c_char);
                continue;
            }
            2 => {
                p = parse_note(p.offset(-(1 as libc::c_int as isize)), flags);
                flags &= 0x20 as libc::c_int;
                if slur != 0
                    && (*parse.last_sym).u.note.notes[0 as libc::c_int as usize].len != 0
                {
                    (*parse.last_sym).u.note.slur_st = slur as libc::c_uchar;
                    slur = 0 as libc::c_int;
                }
                continue;
            }
            16 => {
                if flags & 0x20 as libc::c_int != 0 {
                    current_block = 18318841331983735395;
                } else if char_tb[*p.offset(-(1 as libc::c_int) as isize)
                    as libc::c_uchar as usize] as libc::c_int != 8 as libc::c_int
                {
                    current_block = 18318841331983735395;
                } else {
                    q = p;
                    while *q as libc::c_int == '/' as i32 {
                        q = q.offset(1);
                        q;
                    }
                    if char_tb[*q as libc::c_uchar as usize] as libc::c_int
                        != 8 as libc::c_int
                    {
                        current_block = 18318841331983735395;
                    } else {
                        s = abc_new(9 as libc::c_int, 0 as *mut libc::c_char);
                        (*s).u.bar.type_0 = 0 as libc::c_int;
                        (*s)
                            .u
                            .bar
                            .len = (q.offset_from(p) as libc::c_long
                            + 1 as libc::c_int as libc::c_long) as libc::c_char;
                        syntax(
                            b"Non standard measure repeat syntax\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            p.offset(-(1 as libc::c_int as isize)),
                        );
                        p = q;
                        continue;
                    }
                }
            }
            6 => {
                if *p as libc::c_int == '\0' as i32 {
                    continue;
                }
                syntax(
                    b"'\\' ignored\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    p.offset(-(1 as libc::c_int as isize)),
                );
                continue;
            }
            7 => {
                if *p as libc::c_int == '|' as i32 || *p as libc::c_int == ']' as i32
                    || *p as libc::c_int == ':' as i32
                    || isdigit(*p as libc::c_uchar as libc::c_int) != 0
                    || *p as libc::c_int == '"' as i32 || *p as libc::c_int == ' ' as i32
                {
                    if !(flags & 0x20 as libc::c_int != 0) {
                        p = parse_bar(p);
                        continue;
                    }
                } else if *p.offset(1 as libc::c_int as isize) as libc::c_int
                    != ':' as i32
                {
                    p = parse_note(p.offset(-(1 as libc::c_int as isize)), flags);
                    flags &= 0x20 as libc::c_int;
                    if slur != 0
                        && (*parse.last_sym).u.note.notes[0 as libc::c_int as usize].len
                            != 0
                    {
                        (*parse.last_sym).u.note.slur_st = slur as libc::c_uchar;
                        slur = 0 as libc::c_int;
                    }
                    continue;
                } else {
                    while *p.offset(2 as libc::c_int as isize) as libc::c_int
                        == ' ' as i32
                    {
                        *p
                            .offset(
                                2 as libc::c_int as isize,
                            ) = ':' as i32 as libc::c_char;
                        *p.offset(1 as libc::c_int as isize) = *p;
                        p = p.offset(1);
                        p;
                    }
                    c = ']' as i32 as libc::c_char;
                    q = p;
                    while *p as libc::c_int != '\0' as i32
                        && *p as libc::c_int != c as libc::c_int
                    {
                        p = p.offset(1);
                        p;
                    }
                    if *p as libc::c_int == '\0' as i32 {
                        syntax(
                            b"Escape sequence [..] not closed\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            q,
                        );
                        c = '\0' as i32 as libc::c_char;
                    } else {
                        *p = '\0' as i32 as libc::c_char;
                    }
                    parse_info(q);
                    *p = c;
                    if c as libc::c_int != '\0' as i32 {
                        p = p.offset(1);
                        p;
                    }
                    continue;
                }
                current_block = 18318841331983735395;
            }
            8 => {
                if flags & 0x20 as libc::c_int != 0 {
                    current_block = 18318841331983735395;
                } else {
                    p = parse_bar(p);
                    continue;
                }
            }
            9 => {
                if *p as libc::c_int > '0' as i32 && *p as libc::c_int <= '9' as i32 {
                    let mut pplet: libc::c_int = 0;
                    let mut qplet: libc::c_int = 0;
                    let mut rplet: libc::c_int = 0;
                    pplet = strtol(p, &mut q, 10 as libc::c_int) as libc::c_int;
                    p = q;
                    if (pplet as libc::c_uint as libc::c_ulong)
                        < (::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            )
                    {
                        qplet = qtb[pplet as usize] as libc::c_int;
                    } else {
                        qplet = qtb[0 as libc::c_int as usize] as libc::c_int;
                    }
                    rplet = pplet;
                    if *p as libc::c_int == ':' as i32 {
                        p = p.offset(1);
                        p;
                        if isdigit(*p as libc::c_uchar as libc::c_int) != 0 {
                            qplet = strtol(p, &mut q, 10 as libc::c_int) as libc::c_int;
                            p = q;
                        }
                        if *p as libc::c_int == ':' as i32 {
                            p = p.offset(1);
                            p;
                            if isdigit(*p as libc::c_uchar as libc::c_int) != 0 {
                                rplet = strtol(p, &mut q, 10 as libc::c_int) as libc::c_int;
                                p = q;
                            }
                        }
                    }
                    if rplet < 1 as libc::c_int {
                        syntax(
                            b"Invalid 'r' in tuplet\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            p,
                        );
                        continue;
                    } else if pplet >= 128 as libc::c_int || qplet >= 128 as libc::c_int
                        || rplet >= 128 as libc::c_int
                    {
                        syntax(
                            b"Invalid 'p:q:r' in tuplet\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            p,
                        );
                        continue;
                    } else {
                        if qplet == 0 as libc::c_int {
                            qplet = if meter as libc::c_int % 3 as libc::c_int
                                == 0 as libc::c_int
                            {
                                3 as libc::c_int
                            } else {
                                2 as libc::c_int
                            };
                        }
                        s = abc_new(11 as libc::c_int, 0 as *mut libc::c_char);
                        (*s).u.tuplet.p_plet = pplet as libc::c_char;
                        (*s).u.tuplet.q_plet = qplet as libc::c_char;
                        (*s).u.tuplet.r_plet = rplet as libc::c_char;
                        (*s)
                            .flags = ((*s).flags as libc::c_int | flags)
                            as libc::c_ushort;
                        continue;
                    }
                } else if *p as libc::c_int == '&' as i32 {
                    if !(flags & 0x20 as libc::c_int != 0) {
                        p = p.offset(1);
                        p;
                        if vover as libc::c_int != 0 as libc::c_int {
                            syntax(
                                b"Nested voice overlay\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                                p.offset(-(1 as libc::c_int as isize)),
                            );
                            continue;
                        } else {
                            s = abc_new(10 as libc::c_int, 0 as *mut libc::c_char);
                            (*s).u.v_over.type_0 = 1 as libc::c_int as libc::c_char;
                            (*s)
                                .u
                                .v_over
                                .voice = curvoice.offset_from(voice_tb.as_mut_ptr())
                                as libc::c_long as libc::c_uchar;
                            vover = -(1 as libc::c_int) as libc::c_schar;
                            continue;
                        }
                    }
                } else {
                    slur <<= 4 as libc::c_int;
                    if p == dot.offset(1 as libc::c_int as isize)
                        && dc.n as libc::c_int == 0 as libc::c_int
                    {
                        slur |= 0x8 as libc::c_int;
                    }
                    match *p as libc::c_int {
                        39 => {
                            slur += 0x1 as libc::c_int;
                            p = p.offset(1);
                            p;
                        }
                        44 => {
                            slur += 0x2 as libc::c_int;
                            p = p.offset(1);
                            p;
                        }
                        _ => {
                            slur += 0x4 as libc::c_int;
                        }
                    }
                    continue;
                }
                current_block = 18318841331983735395;
            }
            13 => {
                match (*parse.last_sym).abc_type as libc::c_int {
                    4 | 5 => {
                        (*parse.last_sym).u.note.slur_end += 1;
                        (*parse.last_sym).u.note.slur_end;
                        continue;
                    }
                    _ => {
                        current_block = 18318841331983735395;
                    }
                }
            }
            10 => {
                if flags & 0x20 as libc::c_int != 0 {
                    current_block = 18318841331983735395;
                } else if *p as libc::c_int != ')' as i32
                    || vover as libc::c_int == 0 as libc::c_int
                {
                    if ((*curvoice).last_note).is_null() {
                        syntax(
                            b"Bad start of voice overlay\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            p,
                        );
                        continue;
                    } else {
                        s = abc_new(10 as libc::c_int, 0 as *mut libc::c_char);
                        vover_new();
                        (*s)
                            .u
                            .v_over
                            .voice = curvoice.offset_from(voice_tb.as_mut_ptr())
                            as libc::c_long as libc::c_uchar;
                        if vover as libc::c_int == 0 as libc::c_int {
                            vover = 1 as libc::c_int as libc::c_schar;
                        }
                        continue;
                    }
                } else {
                    p = p.offset(1);
                    p;
                    vover = 0 as libc::c_int as libc::c_schar;
                    s = abc_new(10 as libc::c_int, 0 as *mut libc::c_char);
                    (*s).u.v_over.type_0 = 2 as libc::c_int as libc::c_char;
                    (*s).u.v_over.voice = (*curvoice).mvoice;
                    (*curvoice).last_note = 0 as *mut SYMBOL;
                    curvoice = &mut *voice_tb
                        .as_mut_ptr()
                        .offset((*curvoice).mvoice as isize) as *mut VOICE_S;
                    continue;
                }
            }
            11 => {
                flags |= 0x4 as libc::c_int;
                continue;
            }
            12 => {
                let mut tie_pos: libc::c_int = 0;
                if ((*curvoice).last_note).is_null()
                    || (*(*curvoice).last_note).abc_type as libc::c_int
                        != 4 as libc::c_int
                {
                    current_block = 18318841331983735395;
                } else {
                    if p == dot.offset(1 as libc::c_int as isize)
                        && dc.n as libc::c_int == 0 as libc::c_int
                    {
                        tie_pos = 0x8 as libc::c_int;
                    } else {
                        tie_pos = 0 as libc::c_int;
                    }
                    match *p as libc::c_int {
                        39 => {
                            tie_pos += 0x1 as libc::c_int;
                            p = p.offset(1);
                            p;
                        }
                        44 => {
                            tie_pos += 0x2 as libc::c_int;
                            p = p.offset(1);
                            p;
                        }
                        _ => {
                            tie_pos += 0x4 as libc::c_int;
                        }
                    }
                    i = 0 as libc::c_int;
                    while i <= (*(*curvoice).last_note).nhd as libc::c_int {
                        if (*(*curvoice).last_note).u.note.notes[i as usize].ti1
                            as libc::c_int == 0 as libc::c_int
                        {
                            (*(*curvoice).last_note)
                                .u
                                .note
                                .notes[i as usize]
                                .ti1 = tie_pos as libc::c_char;
                        } else if (*(*curvoice).last_note).nhd as libc::c_int
                            == 0 as libc::c_int
                        {
                            syntax(
                                b"Too many ties\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                p,
                            );
                        }
                        i += 1;
                        i;
                    }
                    continue;
                }
            }
            14 => {
                if ((*curvoice).last_note).is_null() {
                    current_block = 18318841331983735395;
                } else {
                    i = 1 as libc::c_int;
                    while *p as libc::c_int
                        == *p.offset(-(1 as libc::c_int) as isize) as libc::c_int
                    {
                        i += 1;
                        i;
                        p = p.offset(1);
                        p;
                    }
                    if i > 3 as libc::c_int {
                        syntax(
                            b"Bad broken rhythm\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            p.offset(-(1 as libc::c_int as isize)),
                        );
                        i = 3 as libc::c_int;
                    }
                    if *p.offset(-(1 as libc::c_int) as isize) as libc::c_int
                        == '<' as i32
                    {
                        i = -i;
                    }
                    (*(*curvoice).last_note).u.note.brhythm = i as libc::c_schar;
                    continue;
                }
            }
            1 => {
                continue;
            }
            _ => {
                current_block = 18318841331983735395;
            }
        }
        match current_block {
            18318841331983735395 => {
                syntax(
                    (if flags & 0x20 as libc::c_int != 0 {
                        b"Bad character in grace note sequence\0" as *const u8
                            as *const libc::c_char
                    } else {
                        b"Bad character\0" as *const u8 as *const libc::c_char
                    }) as *mut libc::c_char,
                    p.offset(-(1 as libc::c_int as isize)),
                );
            }
            _ => {
                if *p.offset(-(1 as libc::c_int) as isize) as libc::c_int == '.' as i32 {
                    if *p as libc::c_int == '(' as i32 || *p as libc::c_int == '-' as i32
                    {
                        dot = p;
                        continue;
                    }
                }
                p = parse_deco(
                    p.offset(-(1 as libc::c_int as isize)),
                    &mut dc,
                    -(1 as libc::c_int),
                );
            }
        }
    }
    if flags & 0x20 as libc::c_int != 0 {
        syntax(
            b"EOLN in grace note sequence\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            p.offset(-(1 as libc::c_int as isize)),
        );
        if !((*curvoice).last_note).is_null() {
            (*(*curvoice).last_note)
                .flags = ((*(*curvoice).last_note).flags as libc::c_int
                | 0x40 as libc::c_int) as libc::c_ushort;
        }
        (*curvoice).last_note = last_note_sav;
        memcpy(
            &mut dc as *mut decos as *mut libc::c_void,
            &mut dc_sav as *mut decos as *const libc::c_void,
            ::core::mem::size_of::<decos>() as libc::c_ulong,
        );
    }
    s = abc_new(7 as libc::c_int, 0 as *mut libc::c_char);
    if flags & 0x4 as libc::c_int != 0 {
        (*s).flags = ((*s).flags as libc::c_int | 0x4 as libc::c_int) as libc::c_ushort;
    }
    if *p.offset(-(1 as libc::c_int) as isize) as libc::c_int == '\\' as i32
        || char_tb['\n' as i32 as usize] as libc::c_int != 18 as libc::c_int
    {
        (*s).u.eoln.type_0 = 1 as libc::c_int as libc::c_char;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_note(
    mut p: *mut libc::c_char,
    mut flags: libc::c_int,
) -> *mut libc::c_char {
    let mut current_block: u64;
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pit: libc::c_int = 0 as libc::c_int;
    let mut len: libc::c_int = 0;
    let mut acc: libc::c_int = 0;
    let mut nostem: libc::c_int = 0;
    let mut chord: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    if flags & 0x20 as libc::c_int != 0 {
        s = abc_new(4 as libc::c_int, 0 as *mut libc::c_char);
    } else {
        s = abc_new(4 as libc::c_int, gchord);
        if !gchord.is_null() {
            gchord = 0 as *mut libc::c_char;
        }
    }
    (*s).flags = ((*s).flags as libc::c_int | flags) as libc::c_ushort;
    (*s).u.note.notes[0 as libc::c_int as usize].color = -(1 as libc::c_int);
    if lyric_started == 0 {
        lyric_started = 1 as libc::c_int as libc::c_char;
        (*s).flags = ((*s).flags as libc::c_int | 0x10 as libc::c_int) as libc::c_ushort;
    }
    if *p as libc::c_int != 'X' as i32 && *p as libc::c_int != 'Z' as i32
        && flags & 0x20 as libc::c_int == 0
    {
        if deco_start.is_null() {
            deco_start = s;
        }
    }
    chord = 0 as libc::c_int;
    match *p as libc::c_int {
        88 => {
            (*s)
                .flags = ((*s).flags as libc::c_int | 0x2 as libc::c_int)
                as libc::c_ushort;
            current_block = 6690289353540759398;
        }
        90 => {
            current_block = 6690289353540759398;
        }
        121 => {
            (*s).abc_type = 5 as libc::c_int as libc::c_char;
            (*s)
                .flags = ((*s).flags as libc::c_int | 0x2 as libc::c_int)
                as libc::c_ushort;
            p = p.offset(1);
            p;
            if isdigit(*p as libc::c_uchar as libc::c_int) != 0
                || *p as libc::c_int == '-' as i32
            {
                len = strtol(p, &mut q, 10 as libc::c_int) as libc::c_int;
                if len < -(100 as libc::c_int) || len > 100 as libc::c_int {
                    syntax(
                        b"Bad width of y (space)\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        p,
                    );
                    len = 10 as libc::c_int;
                }
                p = q;
            } else {
                len = 10 as libc::c_int;
            }
            (*s).u.note.notes[0 as libc::c_int as usize].shhd = len as libc::c_float;
            current_block = 17172491837804951123;
        }
        120 => {
            (*s)
                .flags = ((*s).flags as libc::c_int | 0x2 as libc::c_int)
                as libc::c_ushort;
            current_block = 5901359447224912758;
        }
        122 => {
            current_block = 5901359447224912758;
        }
        91 => {
            chord = 1 as libc::c_int;
            p = p.offset(1);
            p;
            current_block = 2480299350034459858;
        }
        _ => {
            current_block = 2480299350034459858;
        }
    }
    match current_block {
        2480299350034459858 => {
            q = p;
            m = 0 as libc::c_int;
            nostem = 0 as libc::c_int;
            loop {
                if chord != 0 {
                    if m >= 8 as libc::c_int {
                        syntax(
                            b"Too many notes in chord\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            p,
                        );
                        m -= 1;
                        m;
                    }
                    n = 0 as libc::c_int;
                    if *p as libc::c_int == '.' as i32 {
                        n = 0x8 as libc::c_int;
                        p = p.offset(1);
                        p;
                    }
                    if *p as libc::c_int == '(' as i32 {
                        p = p.offset(1);
                        p;
                        match *p as libc::c_int {
                            39 => {
                                n += 0x1 as libc::c_int;
                                p = p.offset(1);
                                p;
                            }
                            44 => {
                                n += 0x2 as libc::c_int;
                                p = p.offset(1);
                                p;
                            }
                            _ => {
                                n += 0x4 as libc::c_int;
                            }
                        }
                        (*s)
                            .u
                            .note
                            .notes[m as usize]
                            .sl1 = ((((*s).u.note.notes[m as usize].sl1 as libc::c_int)
                            << 3 as libc::c_int) + n) as libc::c_uchar;
                    }
                }
                p = parse_deco(p, &mut dc, m);
                p = parse_acc_pit(p, &mut pit, &mut acc);
                if *p as libc::c_int == '0' as i32 {
                    nostem = 1 as libc::c_int;
                    p = p.offset(1);
                    p;
                }
                p = parse_len(
                    p,
                    if flags & 0x20 as libc::c_int != 0 {
                        1536 as libc::c_int / 8 as libc::c_int
                    } else {
                        ulen
                    },
                    &mut len,
                );
                (*s).u.note.notes[m as usize].pit = pit as libc::c_schar;
                (*s).pits[m as usize] = pit as libc::c_schar;
                (*s).u.note.notes[m as usize].len = len;
                (*s).u.note.notes[m as usize].acc = acc as libc::c_uchar;
                (*s).u.note.notes[m as usize].color = -(1 as libc::c_int);
                if chord != 0 {
                    loop {
                        if *p as libc::c_int == '.' as i32 {
                            if *p.offset(1 as libc::c_int as isize) as libc::c_int
                                != '-' as i32
                            {
                                break;
                            }
                            p = p.offset(1);
                            p;
                        }
                        if *p as libc::c_int == '-' as i32 {
                            match *p.offset(1 as libc::c_int as isize) as libc::c_int {
                                39 => {
                                    (*s)
                                        .u
                                        .note
                                        .notes[m as usize]
                                        .ti1 = 0x1 as libc::c_int as libc::c_char;
                                    p = p.offset(1);
                                    p;
                                }
                                44 => {
                                    (*s)
                                        .u
                                        .note
                                        .notes[m as usize]
                                        .ti1 = 0x2 as libc::c_int as libc::c_char;
                                    p = p.offset(1);
                                    p;
                                }
                                _ => {
                                    (*s)
                                        .u
                                        .note
                                        .notes[m as usize]
                                        .ti1 = 0x4 as libc::c_int as libc::c_char;
                                }
                            }
                        } else {
                            if !(*p as libc::c_int == ')' as i32) {
                                break;
                            }
                            (*s).u.note.notes[m as usize].sl2 += 1;
                            (*s).u.note.notes[m as usize].sl2;
                        }
                        p = p.offset(1);
                        p;
                    }
                }
                if acc >= 0 as libc::c_int {
                    m += 1;
                    m;
                }
                if chord == 0 {
                    break;
                }
                if *p as libc::c_int == ']' as i32 {
                    p = p.offset(1);
                    p;
                    if *p as libc::c_int == '0' as i32 {
                        nostem = 1 as libc::c_int;
                        p = p.offset(1);
                        p;
                    }
                    if *p as libc::c_int == '/' as i32
                        || isdigit(*p as libc::c_uchar as libc::c_int) != 0
                    {
                        p = parse_len(p, ulen, &mut len);
                        j = 0 as libc::c_int;
                        while j < m {
                            (*s)
                                .u
                                .note
                                .notes[j as usize]
                                .len = len * (*s).u.note.notes[j as usize].len / ulen;
                            j += 1;
                            j;
                        }
                    }
                    break;
                } else {
                    if !(*p as libc::c_int == '\0' as i32) {
                        continue;
                    }
                    syntax(
                        b"Chord not closed\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        q,
                    );
                    break;
                }
            }
            if nostem != 0 {
                (*s)
                    .flags = ((*s).flags as libc::c_int | 0x8 as libc::c_int)
                    as libc::c_ushort;
            }
            if m == 0 as libc::c_int {
                current_block = 13011028389781352602;
            } else {
                (*s).u.note.microscale = microscale;
                (*s).nhd = (m - 1 as libc::c_int) as libc::c_uchar;
                current_block = 3399802727763244637;
            }
        }
        5901359447224912758 => {
            (*s).abc_type = 5 as libc::c_int as libc::c_char;
            p = parse_len(p.offset(1 as libc::c_int as isize), ulen, &mut len);
            (*s).u.note.notes[0 as libc::c_int as usize].len = len;
            current_block = 3399802727763244637;
        }
        6690289353540759398 => {
            (*s).abc_type = 8 as libc::c_int as libc::c_char;
            p = p.offset(1);
            p;
            len = 1 as libc::c_int;
            if isdigit(*p as libc::c_uchar as libc::c_int) != 0 {
                len = strtol(p, &mut q, 10 as libc::c_int) as libc::c_int;
                if len == 0 as libc::c_int || len > 100 as libc::c_int {
                    syntax(
                        b"Bad number of measures\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        p,
                    );
                    len = 1 as libc::c_int;
                }
                p = q;
            }
            (*s).u.bar.type_0 = 0 as libc::c_int;
            (*s).u.bar.len = len as libc::c_char;
            current_block = 17172491837804951123;
        }
        _ => {}
    }
    match current_block {
        3399802727763244637 => {
            if !((*curvoice).last_note).is_null()
                && (*(*curvoice).last_note).u.note.brhythm as libc::c_int
                    != 0 as libc::c_int
            {
                broken_rhythm(
                    (*curvoice).last_note,
                    (*(*curvoice).last_note).u.note.brhythm as libc::c_int,
                );
                broken_rhythm(
                    s,
                    -((*(*curvoice).last_note).u.note.brhythm as libc::c_int),
                );
            }
            current_block = 17172491837804951123;
        }
        _ => {}
    }
    match current_block {
        17172491837804951123 => {
            if dc.n as libc::c_int > 0 as libc::c_int {
                memcpy(
                    (if (*s).abc_type as libc::c_int != 8 as libc::c_int {
                        &mut (*s).u.note.dc as *mut decos
                    } else {
                        &mut (*s).u.bar.dc as *mut decos
                    }) as *mut libc::c_void,
                    &mut dc as *mut decos as *const libc::c_void,
                    ::core::mem::size_of::<decos>() as libc::c_ulong,
                );
                dc.n = 0 as libc::c_int as libc::c_char;
            }
            if (*s).abc_type as libc::c_int != 4 as libc::c_int
                && flags & 0x20 as libc::c_int != 0
            {
                syntax(
                    b"Not a note in grace note sequence\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    p,
                );
            } else {
                if (*s).u.note.notes[0 as libc::c_int as usize].len > 0 as libc::c_int {
                    (*curvoice).last_note = s;
                }
                return p;
            }
        }
        _ => {}
    }
    parse.last_sym = (*s).abc_prev;
    if (parse.last_sym).is_null() {
        parse.first_sym = 0 as *mut SYMBOL;
    } else {
        (*(*s).abc_prev).abc_next = 0 as *mut SYMBOL;
        (*(*s).abc_prev)
            .flags = ((*(*s).abc_prev).flags as libc::c_int
            | (*s).flags as libc::c_int & 0x1 as libc::c_int) as libc::c_ushort;
    }
    return p;
}
unsafe extern "C" fn parse_info(mut p: *mut libc::c_char) -> libc::c_int {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut info_type: libc::c_char = *p;
    let mut error_txt: *mut libc::c_char = 0 as *mut libc::c_char;
    s = abc_new(1 as libc::c_int, p);
    p = p.offset(2 as libc::c_int as isize);
    match info_type as libc::c_int {
        100 | 115 => {
            if !(parse.abc_state == 0 as libc::c_int) {
                if deco_start.is_null() {
                    error_txt = b"Erroneous 'd:'/'s:'\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                } else {
                    error_txt = parse_decoline(p);
                }
            }
        }
        75 => {
            if !(parse.abc_state == 0 as libc::c_int) {
                parse_key(p, s);
                if parse.abc_state == 1 as libc::c_int {
                    let mut i: libc::c_int = 0;
                    parse.abc_state = 2 as libc::c_int;
                    if ulen == 0 as libc::c_int {
                        ulen = 1536 as libc::c_int / 8 as libc::c_int;
                    }
                    i = 32 as libc::c_int;
                    loop {
                        i -= 1;
                        if !(i >= 0 as libc::c_int) {
                            break;
                        }
                        voice_tb[i as usize].ulen = ulen;
                    }
                    lyric_started = 0 as libc::c_int as libc::c_char;
                }
            }
        }
        76 => {
            error_txt = get_len(p, s);
            if (*s).u.length.base_length > 0 as libc::c_int {
                ulen = (*s).u.length.base_length;
            }
        }
        77 => {
            error_txt = parse_meter(p, s);
        }
        81 => {
            error_txt = parse_tempo(p, s);
        }
        85 => {
            error_txt = get_user(p, s);
        }
        86 => {
            if !(parse.abc_state == 0 as libc::c_int) {
                error_txt = parse_voice(p, s);
            }
        }
        88 => {
            memset(
                voice_tb.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<[VOICE_S; 32]>() as libc::c_ulong,
            );
            nvoice = 0 as libc::c_int as libc::c_short;
            curvoice = voice_tb.as_mut_ptr();
            parse.abc_state = 1 as libc::c_int;
            lvlarena(1 as libc::c_int);
            return 2 as libc::c_int;
        }
        _ => {}
    }
    if !error_txt.is_null() {
        syntax(error_txt, p);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn syntax(mut msg: *mut libc::c_char, mut q: *mut libc::c_char) {
    let mut n: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut m1: libc::c_int = 0;
    let mut m2: libc::c_int = 0;
    let mut pp: libc::c_int = 0;
    let mut maxcol: libc::c_int = 73 as libc::c_int;
    severity = 1 as libc::c_int;
    n = q.offset_from(abc_line) as libc::c_long as libc::c_int;
    len = strlen(abc_line) as libc::c_int;
    if n as libc::c_uint > len as libc::c_uint {
        n = -(1 as libc::c_int);
    }
    print_error(msg, n);
    if n < 0 as libc::c_int {
        if !q.is_null() && *q as libc::c_int != '\0' as i32 {
            fprintf(
                __stderrp,
                b" (near '%s')\n\0" as *const u8 as *const libc::c_char,
                q,
            );
        }
        return;
    }
    m1 = 0 as libc::c_int;
    m2 = len;
    if m2 > maxcol {
        if n < maxcol {
            m2 = maxcol;
        } else {
            m1 = n - 20 as libc::c_int;
            m2 = m1 + maxcol;
            if m2 > len {
                m2 = len;
            }
        }
    }
    fprintf(__stderrp, b"%4d \0" as *const u8 as *const libc::c_char, linenum);
    pp = 6 as libc::c_int;
    if m1 > 0 as libc::c_int {
        fprintf(__stderrp, b"...\0" as *const u8 as *const libc::c_char);
        pp += 3 as libc::c_int;
    }
    fprintf(
        __stderrp,
        b"%.*s\0" as *const u8 as *const libc::c_char,
        m2 - m1,
        &mut *abc_line.offset(m1 as isize) as *mut libc::c_char,
    );
    if m2 < len {
        fprintf(__stderrp, b"...\0" as *const u8 as *const libc::c_char);
    }
    fprintf(__stderrp, b"\n\0" as *const u8 as *const libc::c_char);
    if (n as libc::c_uint) < 200 as libc::c_int as libc::c_uint {
        fprintf(
            __stderrp,
            b"%*s\n\0" as *const u8 as *const libc::c_char,
            n + pp - m1,
            b"^\0" as *const u8 as *const libc::c_char,
        );
    }
    if !last_sym.is_null() {
        (*last_sym)
            .flags = ((*last_sym).flags as libc::c_int | 0x1 as libc::c_int)
            as libc::c_ushort;
    }
}
unsafe extern "C" fn vover_new() {
    let mut voice: libc::c_int = 0;
    let mut mvoice: libc::c_int = 0;
    mvoice = (*curvoice).mvoice as libc::c_int;
    voice = (curvoice.offset_from(voice_tb.as_mut_ptr()) as libc::c_long
        + 1 as libc::c_int as libc::c_long) as libc::c_int;
    while voice <= nvoice as libc::c_int {
        if voice_tb[voice as usize].mvoice as libc::c_int == mvoice {
            break;
        }
        voice += 1;
        voice;
    }
    if voice > nvoice as libc::c_int {
        if nvoice as libc::c_int >= 32 as libc::c_int {
            syntax(
                b"Too many voices\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut libc::c_char,
            );
            return;
        }
        nvoice = voice as libc::c_short;
        voice_tb[voice as usize]
            .id[0 as libc::c_int as usize] = '&' as i32 as libc::c_char;
        voice_tb[voice as usize].mvoice = mvoice as libc::c_uchar;
    }
    voice_tb[voice as usize].ulen = (*curvoice).ulen;
    voice_tb[voice as usize].microscale = (*curvoice).microscale;
    curvoice = &mut *voice_tb.as_mut_ptr().offset(voice as isize) as *mut VOICE_S;
}
