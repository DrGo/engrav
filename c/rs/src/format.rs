use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type __sFILEX;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
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
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    static mut _DefaultRuneLocale: _RuneLocale;
    fn __maskrune(_: __darwin_ct_rune_t, _: libc::c_ulong) -> libc::c_int;
    fn __toupper(_: __darwin_ct_rune_t) -> __darwin_ct_rune_t;
    fn fputs(_: *const libc::c_char, _: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut deco: [*mut libc::c_char; 256];
    static mut info: INFO;
    static mut outft: libc::c_int;
    static mut epsf: libc::c_int;
    static mut svg: libc::c_int;
    static mut file_initialized: libc::c_int;
    static mut fout: *mut FILE;
    static mut tblts: [*mut tblt_s; 8];
    fn a2b(fmt: *mut libc::c_char, _: ...);
    fn get_str(
        d: *mut libc::c_char,
        s: *mut libc::c_char,
        maxlen: libc::c_int,
    ) -> *mut libc::c_char;
    static mut voice_tb: [VOICE_S; 32];
    fn getarena(len: libc::c_int) -> *mut libc::c_void;
    static mut space_tb: [libc::c_float; 10];
    fn error(sev: libc::c_int, s: *mut SYMBOL, fmt: *mut libc::c_char, _: ...);
    fn pg_reset_font();
    fn svg_font_switch();
    fn define_font(name: *mut libc::c_char, num: libc::c_int, enc: libc::c_int);
}
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_longlong;
pub type __darwin_ct_rune_t = libc::c_int;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_wchar_t = libc::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_off_t = __int64_t;
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
pub struct format {
    pub name: *mut libc::c_char,
    pub v: *mut libc::c_void,
    pub type_0: libc::c_char,
    pub subtype: libc::c_char,
    pub lock: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpar {
    pub name: *mut libc::c_char,
    pub f: Option::<unsafe extern "C" fn(*mut VOICE_S, libc::c_int) -> ()>,
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
pub unsafe extern "C" fn toupper(mut _c: libc::c_int) -> libc::c_int {
    return __toupper(_c);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn isspace(mut _c: libc::c_int) -> libc::c_int {
    return __istype(_c, 0x4000 as libc::c_long as libc::c_ulong);
}
#[no_mangle]
pub static mut cfmt: FORMAT = FORMAT {
    pageheight: 0.,
    pagewidth: 0.,
    topmargin: 0.,
    botmargin: 0.,
    leftmargin: 0.,
    rightmargin: 0.,
    topspace: 0.,
    wordsspace: 0.,
    titlespace: 0.,
    subtitlespace: 0.,
    partsspace: 0.,
    composerspace: 0.,
    musicspace: 0.,
    vocalspace: 0.,
    textspace: 0.,
    breaklimit: 0.,
    maxshrink: 0.,
    lineskipfac: 0.,
    parskipfac: 0.,
    stemheight: 0.,
    gutter: 0.,
    indent: 0.,
    infospace: 0.,
    slurheight: 0.,
    tieheight: 0.,
    notespacingfactor: 0.,
    scale: 0.,
    staffsep: 0.,
    sysstaffsep: 0.,
    maxstaffsep: 0.,
    maxsysstaffsep: 0.,
    stretchlast: 0.,
    staffscale: 0.,
    abc2pscompat: 0,
    alignbars: 0,
    aligncomposer: 0,
    autoclef: 0,
    barsperstaff: 0,
    breakoneoln: 0,
    bstemdown: 0,
    cancelkey: 0,
    capo: 0,
    combinevoices: 0,
    contbarnb: 0,
    continueall: 0,
    custos: 0,
    dblrepbar: 0,
    decoerr: 0,
    dynalign: 0,
    flatbeams: 0,
    flatbeamgracing: 0,
    infoline: 0,
    gchordbox: 0,
    graceslurs: 0,
    graceword: 0,
    gracespace: 0,
    hyphencont: 0,
    keywarn: 0,
    landscape: 0,
    linewarn: 0,
    measurebox: 0,
    measurefirst: 0,
    measurenb: 0,
    nedo: 0,
    oneperpage: 0,
    pango: 0,
    partsbox: 0,
    pdfmark: 0,
    rbdbstop: 0,
    rbmax: 0,
    rbmin: 0,
    setdefl: 0,
    shiftunison: 0,
    splittune: 0,
    squarebreve: 0,
    staffnonote: 0,
    straightflags: 0,
    stretchstaff: 0,
    textoption: 0,
    titlecaps: 0,
    titleleft: 0,
    titletrim: 0,
    timewarn: 0,
    transpose: 0,
    tuplets: 0,
    bgcolor: 0 as *const libc::c_char as *mut libc::c_char,
    dateformat: 0 as *const libc::c_char as *mut libc::c_char,
    header: 0 as *const libc::c_char as *mut libc::c_char,
    footer: 0 as *const libc::c_char as *mut libc::c_char,
    header2: 0 as *const libc::c_char as *mut libc::c_char,
    footer2: 0 as *const libc::c_char as *mut libc::c_char,
    titleformat: 0 as *const libc::c_char as *mut libc::c_char,
    musicfont: 0 as *const libc::c_char as *mut libc::c_char,
    font_tb: [FONTSPEC {
        fnum: 0,
        size: 0.,
        swfac: 0.,
    }; 39],
    ndfont: 0,
    gcf: 0,
    anf: 0,
    vof: 0,
    fields: [0; 2],
    posit: posit_s {
        dyn_0_gch_orn_voc_vol_std_gsd: [0; 4],
    },
};
#[no_mangle]
pub static mut fontnames: [*mut libc::c_char; 30] = [0 as *const libc::c_char
    as *mut libc::c_char; 30];
static mut font_enc: [libc::c_char; 30] = [0; 30];
static mut def_font_enc: [libc::c_char; 30] = [0; 30];
static mut used_font: [libc::c_char; 30] = [0; 30];
static mut swfac_font: [libc::c_float; 30] = [0.; 30];
static mut nfontnames: libc::c_int = 0;
static mut staffwidth: libc::c_float = 0.;
static mut format_tb: [format; 110] = [format {
    name: 0 as *mut libc::c_char,
    v: 0 as *mut libc::c_void,
    type_0: 0,
    subtype: 0,
    lock: 0,
}; 110];
static mut helvetica: [libc::c_char; 10] = unsafe {
    *::core::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"Helvetica\0")
};
static mut times: [libc::c_char; 12] = unsafe {
    *::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"Times-Roman\0")
};
static mut times_bold: [libc::c_char; 11] = unsafe {
    *::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"Times-Bold\0")
};
static mut times_italic: [libc::c_char; 13] = unsafe {
    *::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"Times-Italic\0")
};
static mut sans: [libc::c_char; 11] = unsafe {
    *::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"sans-serif\0")
};
static mut serif: [libc::c_char; 6] = unsafe {
    *::core::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"serif\0")
};
static mut serif_italic: [libc::c_char; 13] = unsafe {
    *::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"serif-Italic\0")
};
static mut serif_bold: [libc::c_char; 11] = unsafe {
    *::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"serif-Bold\0")
};
unsafe extern "C" fn get_font(
    mut fname: *const libc::c_char,
    mut encoding: libc::c_int,
) -> libc::c_int {
    let mut fnum: libc::c_int = 0;
    fnum = nfontnames;
    loop {
        fnum -= 1;
        if !(fnum >= 0 as libc::c_int) {
            break;
        }
        if !(strcmp(fname, fontnames[fnum as usize]) == 0 as libc::c_int) {
            continue;
        }
        if encoding < 0 as libc::c_int {
            encoding = def_font_enc[fnum as usize] as libc::c_int;
        }
        if encoding == font_enc[fnum as usize] as libc::c_int {
            return fnum;
        }
        break;
    }
    loop {
        fnum -= 1;
        if !(fnum >= 0 as libc::c_int) {
            break;
        }
        if strcmp(fname, fontnames[fnum as usize]) == 0 as libc::c_int
            && encoding == font_enc[fnum as usize] as libc::c_int
        {
            return fnum;
        }
    }
    if nfontnames >= 30 as libc::c_int {
        error(
            1 as libc::c_int,
            0 as *mut SYMBOL,
            b"Too many fonts\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if epsf <= 1 as libc::c_int && svg == 0 {
        if file_initialized > 0 as libc::c_int {
            error(
                1 as libc::c_int,
                0 as *mut SYMBOL,
                b"Cannot have a new font when the output file is opened\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        if !(strchr(fname, ' ' as i32)).is_null() {
            error(
                1 as libc::c_int,
                0 as *mut SYMBOL,
                b"PostScript fonts cannot have names with spaces\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
            return 0 as libc::c_int;
        }
    }
    let fresh0 = nfontnames;
    nfontnames = nfontnames + 1;
    fnum = fresh0;
    fontnames[fnum as usize] = strdup(fname);
    if encoding < 0 as libc::c_int {
        encoding = 0 as libc::c_int;
    }
    font_enc[fnum as usize] = encoding as libc::c_char;
    return fnum;
}
unsafe extern "C" fn dfont_set(mut f: *mut FONTSPEC) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = FONT_DYN as libc::c_int;
    while i < cfmt.ndfont as libc::c_int {
        if cfmt.font_tb[i as usize].fnum == (*f).fnum
            && cfmt.font_tb[i as usize].size == (*f).size
        {
            return i;
        }
        i += 1;
        i;
    }
    if i >= FONT_DYN as libc::c_int + 12 as libc::c_int - 1 as libc::c_int {
        error(
            1 as libc::c_int,
            0 as *mut SYMBOL,
            b"Too many dynamic fonts\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return FONT_DYN as libc::c_int + 12 as libc::c_int - 1 as libc::c_int;
    }
    memcpy(
        &mut *(cfmt.font_tb).as_mut_ptr().offset(i as isize) as *mut FONTSPEC
            as *mut libc::c_void,
        f as *const libc::c_void,
        ::core::mem::size_of::<FONTSPEC>() as libc::c_ulong,
    );
    cfmt.ndfont = (i + 1 as libc::c_int) as libc::c_char;
    return i;
}
unsafe extern "C" fn fontspec(
    mut f: *mut FONTSPEC,
    mut name: *const libc::c_char,
    mut encoding: libc::c_int,
    mut size: libc::c_float,
) {
    if !name.is_null() {
        (*f).fnum = get_font(name, encoding);
    } else {
        name = fontnames[(*f).fnum as usize];
    }
    (*f).size = size;
    (*f).swfac = size;
    if swfac_font[(*f).fnum as usize] != 0 as libc::c_int as libc::c_float {
        (*f).swfac *= swfac_font[(*f).fnum as usize];
    } else if strncmp(name, times.as_ptr(), 5 as libc::c_int as libc::c_ulong)
        == 0 as libc::c_int
        || strncmp(name, serif.as_ptr(), 5 as libc::c_int as libc::c_ulong)
            == 0 as libc::c_int
    {
        if strcmp(name, times_bold.as_ptr()) == 0 as libc::c_int
            || strcmp(name, serif_bold.as_ptr()) == 0 as libc::c_int
        {
            (*f).swfac = ((*f).swfac as libc::c_double * 1.05f64) as libc::c_float;
        }
    } else if strcmp(name, b"Helvetica-Bold\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        (*f).swfac = ((*f).swfac as libc::c_double * 1.15f64) as libc::c_float;
    } else if strncmp(name, helvetica.as_ptr(), 9 as libc::c_int as libc::c_ulong)
        == 0 as libc::c_int
        || strncmp(
            name,
            b"Palatino\0" as *const u8 as *const libc::c_char,
            8 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        (*f).swfac = ((*f).swfac as libc::c_double * 1.1f64) as libc::c_float;
    } else if strncmp(
        name,
        b"Courier\0" as *const u8 as *const libc::c_char,
        7 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        (*f).swfac = ((*f).swfac as libc::c_double * 1.35f64) as libc::c_float;
    } else {
        (*f).swfac = ((*f).swfac as libc::c_double * 1.1f64) as libc::c_float;
    }
    if f
        == &mut *(cfmt.font_tb).as_mut_ptr().offset(GCHORDFONT as libc::c_int as isize)
            as *mut FONTSPEC
    {
        cfmt.gcf = dfont_set(f) as libc::c_uchar;
    } else if f
        == &mut *(cfmt.font_tb)
            .as_mut_ptr()
            .offset(ANNOTATIONFONT as libc::c_int as isize) as *mut FONTSPEC
    {
        cfmt.anf = dfont_set(f) as libc::c_uchar;
    } else if f
        == &mut *(cfmt.font_tb).as_mut_ptr().offset(VOCALFONT as libc::c_int as isize)
            as *mut FONTSPEC
    {
        cfmt.vof = dfont_set(f) as libc::c_uchar;
    }
}
#[no_mangle]
pub unsafe extern "C" fn define_fonts() {
    let mut i: libc::c_int = 0;
    static mut mkfont: *mut libc::c_char = b"/mkfont{findfont dup length 1 add dict begin\n\t{1 index/FID ne{def}{pop pop}ifelse}forall\n\tCharStrings/double_sharp known not{\n\t    /CharStrings CharStrings dup length dict copy def\n\t    FontMatrix 0 get 1 eq{\n\t\tCharStrings/sharp{pop .46 0 setcharwidth .001 dup scale usharp ufill}bind put\n\t\tCharStrings/flat{pop .46 0 setcharwidth .001 dup scale uflat ufill}bind put\n\t\tCharStrings/natural{pop .40 0 setcharwidth .001 dup scale unat ufill}bind put\n\t\tCharStrings/double_sharp{pop .46 0 setcharwidth .001 dup scale udblesharp ufill}bind put\n\t\tCharStrings/double_flat{pop .50 0 setcharwidth .001 dup scale udbleflat ufill}bind put\n\t    }{\n\t\tCharStrings/sharp{pop 460 0 setcharwidth usharp ufill}bind put\n\t\tCharStrings/flat{pop 460 0 setcharwidth uflat ufill}bind put\n\t\tCharStrings/natural{pop 400 0 setcharwidth unat ufill}bind put\n\t\tCharStrings/double_sharp{pop 460 0 setcharwidth udblesharp ufill}bind put\n\t\tCharStrings/double_flat{pop 500 0 setcharwidth udbleflat ufill}bind put\n\t    }ifelse\n\t}if currentdict definefont pop end}!\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    fputs(mkfont, fout);
    make_font_list();
    i = 0 as libc::c_int;
    while i < nfontnames {
        if used_font[i as usize] != 0 {
            define_font(fontnames[i as usize], i, font_enc[i as usize] as libc::c_int);
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn make_font_list() {
    let mut f: *mut FORMAT = 0 as *mut FORMAT;
    let mut i: libc::c_int = 0;
    f = &mut cfmt;
    i = 10 as libc::c_int;
    while i < FONT_DYN as libc::c_int {
        used_font[(*f).font_tb[i as usize].fnum
            as usize] = 1 as libc::c_int as libc::c_char;
        i += 1;
        i;
    }
}
unsafe extern "C" fn set_infoname(mut p: *mut libc::c_char) {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut prev: *mut SYMBOL = 0 as *mut SYMBOL;
    if *p as libc::c_int == 'I' as i32 {
        return;
    }
    s = info[('I' as i32 - 'A' as i32) as usize];
    prev = 0 as *mut SYMBOL;
    while !s.is_null() {
        if *((*s).text).offset(0 as libc::c_int as isize) as libc::c_int
            == *p as libc::c_int
        {
            break;
        }
        prev = s;
        s = (*s).next;
    }
    if *p.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
        if !s.is_null() {
            if prev.is_null() {
                info[('I' as i32 - 'A' as i32) as usize] = (*s).next;
            } else {
                (*prev).next = (*s).next;
                if !((*prev).next).is_null() {
                    (*(*prev).next).prev = prev;
                }
            }
        }
        return;
    }
    if s.is_null() {
        s = getarena(::core::mem::size_of::<SYMBOL>() as libc::c_ulong as libc::c_int)
            as *mut SYMBOL;
        memset(
            s as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<SYMBOL>() as libc::c_ulong,
        );
        if prev.is_null() {
            info[('I' as i32 - 'A' as i32) as usize] = s;
        } else {
            (*prev).next = s;
            (*s).prev = prev;
        }
    }
    (*s)
        .text = getarena(
        (strlen(p)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
    ) as *mut libc::c_char;
    strcpy((*s).text, p);
}
#[no_mangle]
pub unsafe extern "C" fn set_format() {
    let mut f: *mut FORMAT = 0 as *mut FORMAT;
    f = &mut cfmt;
    memset(
        f as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<FORMAT>() as libc::c_ulong,
    );
    (*f).pageheight = (11.0f64 * 96.0f64) as libc::c_float;
    (*f).pagewidth = (8.5f64 * 96.0f64) as libc::c_float;
    (*f).leftmargin = (0.7f64 * 96.0f64) as libc::c_float;
    (*f).rightmargin = (0.7f64 * 96.0f64) as libc::c_float;
    (*f).topmargin = (1.0f64 * 37.8f64) as libc::c_float;
    (*f).botmargin = (1.0f64 * 37.8f64) as libc::c_float;
    (*f).topspace = 22.0f64 as libc::c_float;
    (*f).titlespace = 6.0f64 as libc::c_float;
    (*f).subtitlespace = 3.0f64 as libc::c_float;
    (*f).composerspace = 6.0f64 as libc::c_float;
    (*f).musicspace = 6.0f64 as libc::c_float;
    (*f).partsspace = 8.0f64 as libc::c_float;
    (*f).staffsep = 46.0f64 as libc::c_float;
    (*f).sysstaffsep = 34.0f64 as libc::c_float;
    (*f).maxstaffsep = 2000.0f64 as libc::c_float;
    (*f).maxsysstaffsep = 2000.0f64 as libc::c_float;
    (*f).vocalspace = 10 as libc::c_int as libc::c_float;
    (*f).textspace = 14 as libc::c_int as libc::c_float;
    (*f).scale = 1.0f64 as libc::c_float;
    (*f).slurheight = 1.0f64 as libc::c_float;
    (*f).tieheight = 1.0f64 as libc::c_float;
    (*f).maxshrink = 0.65f64 as libc::c_float;
    (*f).breaklimit = 0.7f64 as libc::c_float;
    (*f).stretchlast = 0.25f64 as libc::c_float;
    (*f).stretchstaff = 1 as libc::c_int;
    (*f).graceslurs = 1 as libc::c_int;
    (*f).hyphencont = 1 as libc::c_int;
    (*f).lineskipfac = 1.1f64 as libc::c_float;
    (*f).parskipfac = 0.4f64 as libc::c_float;
    (*f).measurenb = -(1 as libc::c_int);
    (*f).measurefirst = 1 as libc::c_int;
    (*f).autoclef = 1 as libc::c_int;
    (*f).breakoneoln = 1 as libc::c_int;
    (*f).cancelkey = 1 as libc::c_int;
    (*f)
        .dblrepbar = ((4 as libc::c_int) << 12 as libc::c_int)
        + ((3 as libc::c_int) << 8 as libc::c_int)
        + ((2 as libc::c_int) << 4 as libc::c_int) + 4 as libc::c_int;
    (*f).decoerr = 1 as libc::c_int;
    (*f).dynalign = 1 as libc::c_int;
    (*f).keywarn = 1 as libc::c_int;
    (*f).linewarn = 1 as libc::c_int;
    if svg == 0 && epsf <= 1 as libc::c_int {
        (*f).pango = 1 as libc::c_int;
    } else {
        lock_fmt(&mut cfmt.pango as *mut libc::c_int as *mut libc::c_void);
    }
    (*f).rbdbstop = 1 as libc::c_int;
    (*f).rbmax = 4 as libc::c_int;
    (*f).rbmin = 2 as libc::c_int;
    (*f).staffnonote = 1 as libc::c_int;
    (*f).titletrim = 1 as libc::c_int;
    (*f).aligncomposer = 2 as libc::c_int;
    (*f).notespacingfactor = 1.414f64 as libc::c_float;
    (*f).stemheight = 21 as libc::c_int as libc::c_float;
    (*f).dateformat = strdup(b"%b %e, %Y %H:%M\0" as *const u8 as *const libc::c_char);
    (*f)
        .gracespace = (65 as libc::c_int) << 16 as libc::c_int
        | (80 as libc::c_int) << 8 as libc::c_int | 120 as libc::c_int;
    (*f).textoption = 0 as libc::c_int;
    (*f).ndfont = FONT_DYN as libc::c_int as libc::c_char;
    if svg != 0 || epsf > 2 as libc::c_int {
        fontspec(
            &mut *((*f).font_tb)
                .as_mut_ptr()
                .offset(ANNOTATIONFONT as libc::c_int as isize),
            sans.as_ptr(),
            0 as libc::c_int,
            12.0f64 as libc::c_float,
        );
        fontspec(
            &mut *((*f).font_tb)
                .as_mut_ptr()
                .offset(COMPOSERFONT as libc::c_int as isize),
            serif_italic.as_ptr(),
            0 as libc::c_int,
            14.0f64 as libc::c_float,
        );
        fontspec(
            &mut *((*f).font_tb).as_mut_ptr().offset(FOOTERFONT as libc::c_int as isize),
            serif.as_ptr(),
            0 as libc::c_int,
            16.0f64 as libc::c_float,
        );
        fontspec(
            &mut *((*f).font_tb).as_mut_ptr().offset(GCHORDFONT as libc::c_int as isize),
            sans.as_ptr(),
            0 as libc::c_int,
            12.0f64 as libc::c_float,
        );
        fontspec(
            &mut *((*f).font_tb).as_mut_ptr().offset(HEADERFONT as libc::c_int as isize),
            serif.as_ptr(),
            0 as libc::c_int,
            16.0f64 as libc::c_float,
        );
        fontspec(
            &mut *((*f).font_tb)
                .as_mut_ptr()
                .offset(HISTORYFONT as libc::c_int as isize),
            serif.as_ptr(),
            0 as libc::c_int,
            16.0f64 as libc::c_float,
        );
        fontspec(
            &mut *((*f).font_tb).as_mut_ptr().offset(INFOFONT as libc::c_int as isize),
            serif_italic.as_ptr(),
            0 as libc::c_int,
            14.0f64 as libc::c_float,
        );
        fontspec(
            &mut *((*f).font_tb)
                .as_mut_ptr()
                .offset(MEASUREFONT as libc::c_int as isize),
            serif_italic.as_ptr(),
            0 as libc::c_int,
            14.0f64 as libc::c_float,
        );
        fontspec(
            &mut *((*f).font_tb).as_mut_ptr().offset(PARTSFONT as libc::c_int as isize),
            serif.as_ptr(),
            0 as libc::c_int,
            15.0f64 as libc::c_float,
        );
        fontspec(
            &mut *((*f).font_tb).as_mut_ptr().offset(REPEATFONT as libc::c_int as isize),
            serif.as_ptr(),
            0 as libc::c_int,
            13.0f64 as libc::c_float,
        );
        fontspec(
            &mut *((*f).font_tb)
                .as_mut_ptr()
                .offset(SUBTITLEFONT as libc::c_int as isize),
            serif.as_ptr(),
            0 as libc::c_int,
            16.0f64 as libc::c_float,
        );
        fontspec(
            &mut *((*f).font_tb).as_mut_ptr().offset(TEMPOFONT as libc::c_int as isize),
            serif_bold.as_ptr(),
            0 as libc::c_int,
            15.0f64 as libc::c_float,
        );
        fontspec(
            &mut *((*f).font_tb).as_mut_ptr().offset(TEXTFONT as libc::c_int as isize),
            serif.as_ptr(),
            0 as libc::c_int,
            16.0f64 as libc::c_float,
        );
        fontspec(
            &mut *((*f).font_tb).as_mut_ptr().offset(TITLEFONT as libc::c_int as isize),
            serif.as_ptr(),
            0 as libc::c_int,
            20.0f64 as libc::c_float,
        );
        fontspec(
            &mut *((*f).font_tb).as_mut_ptr().offset(VOCALFONT as libc::c_int as isize),
            serif_bold.as_ptr(),
            0 as libc::c_int,
            13.0f64 as libc::c_float,
        );
        fontspec(
            &mut *((*f).font_tb).as_mut_ptr().offset(VOICEFONT as libc::c_int as isize),
            serif_bold.as_ptr(),
            0 as libc::c_int,
            13.0f64 as libc::c_float,
        );
        fontspec(
            &mut *((*f).font_tb).as_mut_ptr().offset(WORDSFONT as libc::c_int as isize),
            serif.as_ptr(),
            0 as libc::c_int,
            16.0f64 as libc::c_float,
        );
    } else {
        fontspec(
            &mut *((*f).font_tb)
                .as_mut_ptr()
                .offset(ANNOTATIONFONT as libc::c_int as isize),
            helvetica.as_ptr(),
            0 as libc::c_int,
            12.0f64 as libc::c_float,
        );
        fontspec(
            &mut *((*f).font_tb)
                .as_mut_ptr()
                .offset(COMPOSERFONT as libc::c_int as isize),
            times_italic.as_ptr(),
            0 as libc::c_int,
            14.0f64 as libc::c_float,
        );
        fontspec(
            &mut *((*f).font_tb).as_mut_ptr().offset(FOOTERFONT as libc::c_int as isize),
            times.as_ptr(),
            0 as libc::c_int,
            16.0f64 as libc::c_float,
        );
        fontspec(
            &mut *((*f).font_tb).as_mut_ptr().offset(GCHORDFONT as libc::c_int as isize),
            helvetica.as_ptr(),
            0 as libc::c_int,
            12.0f64 as libc::c_float,
        );
        fontspec(
            &mut *((*f).font_tb).as_mut_ptr().offset(HEADERFONT as libc::c_int as isize),
            times.as_ptr(),
            0 as libc::c_int,
            16.0f64 as libc::c_float,
        );
        fontspec(
            &mut *((*f).font_tb)
                .as_mut_ptr()
                .offset(HISTORYFONT as libc::c_int as isize),
            times.as_ptr(),
            0 as libc::c_int,
            16.0f64 as libc::c_float,
        );
        fontspec(
            &mut *((*f).font_tb).as_mut_ptr().offset(INFOFONT as libc::c_int as isize),
            times_italic.as_ptr(),
            0 as libc::c_int,
            14.0f64 as libc::c_float,
        );
        fontspec(
            &mut *((*f).font_tb)
                .as_mut_ptr()
                .offset(MEASUREFONT as libc::c_int as isize),
            times_italic.as_ptr(),
            0 as libc::c_int,
            14.0f64 as libc::c_float,
        );
        fontspec(
            &mut *((*f).font_tb).as_mut_ptr().offset(PARTSFONT as libc::c_int as isize),
            times.as_ptr(),
            0 as libc::c_int,
            15.0f64 as libc::c_float,
        );
        fontspec(
            &mut *((*f).font_tb).as_mut_ptr().offset(REPEATFONT as libc::c_int as isize),
            times.as_ptr(),
            0 as libc::c_int,
            13.0f64 as libc::c_float,
        );
        fontspec(
            &mut *((*f).font_tb)
                .as_mut_ptr()
                .offset(SUBTITLEFONT as libc::c_int as isize),
            times.as_ptr(),
            0 as libc::c_int,
            16.0f64 as libc::c_float,
        );
        fontspec(
            &mut *((*f).font_tb).as_mut_ptr().offset(TEMPOFONT as libc::c_int as isize),
            times_bold.as_ptr(),
            0 as libc::c_int,
            15.0f64 as libc::c_float,
        );
        fontspec(
            &mut *((*f).font_tb).as_mut_ptr().offset(TEXTFONT as libc::c_int as isize),
            times.as_ptr(),
            0 as libc::c_int,
            16.0f64 as libc::c_float,
        );
        fontspec(
            &mut *((*f).font_tb).as_mut_ptr().offset(TITLEFONT as libc::c_int as isize),
            times.as_ptr(),
            0 as libc::c_int,
            20.0f64 as libc::c_float,
        );
        fontspec(
            &mut *((*f).font_tb).as_mut_ptr().offset(VOCALFONT as libc::c_int as isize),
            times_bold.as_ptr(),
            0 as libc::c_int,
            13.0f64 as libc::c_float,
        );
        fontspec(
            &mut *((*f).font_tb).as_mut_ptr().offset(VOICEFONT as libc::c_int as isize),
            times_bold.as_ptr(),
            0 as libc::c_int,
            13.0f64 as libc::c_float,
        );
        fontspec(
            &mut *((*f).font_tb).as_mut_ptr().offset(WORDSFONT as libc::c_int as isize),
            times.as_ptr(),
            0 as libc::c_int,
            16.0f64 as libc::c_float,
        );
    }
    (*f)
        .fields[0 as libc::c_int
        as usize] = ((1 as libc::c_int) << 'C' as i32 - 'A' as i32
        | (1 as libc::c_int) << 'M' as i32 - 'A' as i32
        | (1 as libc::c_int) << 'O' as i32 - 'A' as i32
        | (1 as libc::c_int) << 'P' as i32 - 'A' as i32
        | (1 as libc::c_int) << 'Q' as i32 - 'A' as i32
        | (1 as libc::c_int) << 'T' as i32 - 'A' as i32
        | (1 as libc::c_int) << 'W' as i32 - 'A' as i32) as libc::c_uint;
    (*f)
        .fields[1 as libc::c_int
        as usize] = ((1 as libc::c_int) << 'w' as i32 - 'a' as i32) as libc::c_uint;
    set_infoname(
        b"R \"Rhythm: \"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    set_infoname(
        b"B \"Book: \"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    set_infoname(
        b"S \"Source: \"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    set_infoname(
        b"D \"Discography: \"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    set_infoname(
        b"N \"Notes: \"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    set_infoname(
        b"Z \"Transcription: \"\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    set_infoname(
        b"H \"History: \"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn print_format() {
    let mut fd: *mut format = 0 as *mut format;
    static mut yn: [*mut libc::c_char; 2] = [
        b"no\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"yes\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ];
    fd = format_tb.as_mut_ptr();
    while !((*fd).name).is_null() {
        printf(b"%-15s \0" as *const u8 as *const libc::c_char, (*fd).name);
        match (*fd).type_0 as libc::c_int {
            4 => {
                let mut current_block_11: u64;
                match (*fd).subtype as libc::c_int {
                    2 => {
                        if cfmt.pango == 2 as libc::c_int {
                            printf(b"2\n\0" as *const u8 as *const libc::c_char);
                            current_block_11 = 5143058163439228106;
                        } else {
                            current_block_11 = 11729212326994078217;
                        }
                    }
                    1 => {
                        let mut i: libc::c_int = 0;
                        i = 0 as libc::c_int;
                        while i < 32 as libc::c_int {
                            if cfmt.fields[0 as libc::c_int as usize]
                                & ((1 as libc::c_int) << i) as libc::c_uint != 0
                            {
                                printf(
                                    b"%c\0" as *const u8 as *const libc::c_char,
                                    ('A' as i32 + i) as libc::c_char as libc::c_int,
                                );
                            }
                            if cfmt.fields[1 as libc::c_int as usize]
                                & ((1 as libc::c_int) << i) as libc::c_uint != 0
                            {
                                printf(
                                    b"%c\0" as *const u8 as *const libc::c_char,
                                    ('a' as i32 + i) as libc::c_char as libc::c_int,
                                );
                            }
                            i += 1;
                            i;
                        }
                        printf(b"\n\0" as *const u8 as *const libc::c_char);
                        current_block_11 = 5143058163439228106;
                    }
                    0 | _ => {
                        current_block_11 = 11729212326994078217;
                    }
                }
                match current_block_11 {
                    11729212326994078217 => {
                        printf(
                            b"%s\n\0" as *const u8 as *const libc::c_char,
                            yn[*((*fd).v as *mut libc::c_int) as usize],
                        );
                    }
                    _ => {}
                }
            }
            0 => {
                match (*fd).subtype as libc::c_int {
                    2 => {
                        let mut v: libc::c_int = 0;
                        let mut tmp: [libc::c_char; 16] = [0; 16];
                        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
                        p = &mut *tmp
                            .as_mut_ptr()
                            .offset(
                                (::core::mem::size_of::<[libc::c_char; 16]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            ) as *mut libc::c_char;
                        *p = '\0' as i32 as libc::c_char;
                        v = cfmt.dblrepbar;
                        while v != 0 as libc::c_int {
                            match v & 0xf as libc::c_int {
                                1 => {
                                    p = p.offset(-1);
                                    *p = '|' as i32 as libc::c_char;
                                }
                                2 => {
                                    p = p.offset(-1);
                                    *p = '[' as i32 as libc::c_char;
                                }
                                3 => {
                                    p = p.offset(-1);
                                    *p = ']' as i32 as libc::c_char;
                                }
                                _ => {
                                    p = p.offset(-1);
                                    *p = ':' as i32 as libc::c_char;
                                }
                            }
                            v >>= 4 as libc::c_int;
                        }
                        printf(b"%s\n\0" as *const u8 as *const libc::c_char, p);
                    }
                    3 => {
                        printf(
                            b"%d %d %d %d\n\0" as *const u8 as *const libc::c_char,
                            cfmt.tuplets >> 12 as libc::c_int,
                            cfmt.tuplets >> 8 as libc::c_int & 0xf as libc::c_int,
                            cfmt.tuplets >> 4 as libc::c_int & 0xf as libc::c_int,
                            cfmt.tuplets & 0xf as libc::c_int,
                        );
                    }
                    5 => {
                        printf(
                            b"%d.%d %d.%d %d.%d\n\0" as *const u8 as *const libc::c_char,
                            (cfmt.gracespace >> 16 as libc::c_int) / 10 as libc::c_int,
                            (cfmt.gracespace >> 16 as libc::c_int) % 10 as libc::c_int,
                            (cfmt.gracespace >> 8 as libc::c_int & 0xff as libc::c_int)
                                / 10 as libc::c_int,
                            (cfmt.gracespace >> 8 as libc::c_int & 0xff as libc::c_int)
                                % 10 as libc::c_int,
                            (cfmt.gracespace & 0xff as libc::c_int) / 10 as libc::c_int,
                            (cfmt.gracespace & 0xff as libc::c_int) % 10 as libc::c_int,
                        );
                    }
                    _ => {
                        printf(
                            b"%d\n\0" as *const u8 as *const libc::c_char,
                            *((*fd).v as *mut libc::c_int),
                        );
                    }
                }
            }
            1 => {
                printf(
                    b"%.2f\n\0" as *const u8 as *const libc::c_char,
                    *((*fd).v as *mut libc::c_float) as libc::c_double,
                );
            }
            2 => {
                let mut s: *mut FONTSPEC = 0 as *mut FONTSPEC;
                s = (*fd).v as *mut FONTSPEC;
                printf(
                    b"%s\0" as *const u8 as *const libc::c_char,
                    fontnames[(*s).fnum as usize],
                );
                printf(
                    b" %s\0" as *const u8 as *const libc::c_char,
                    if font_enc[(*s).fnum as usize] as libc::c_int != 0 {
                        b"native\0" as *const u8 as *const libc::c_char
                    } else {
                        b"utf-8\0" as *const u8 as *const libc::c_char
                    },
                );
                printf(
                    b" %.1f\0" as *const u8 as *const libc::c_char,
                    (*s).size as libc::c_double,
                );
                if (*fd).subtype as libc::c_int == 1 as libc::c_int && cfmt.partsbox != 0
                    || (*fd).subtype as libc::c_int == 2 as libc::c_int
                        && cfmt.measurebox != 0
                    || (*fd).subtype as libc::c_int == 3 as libc::c_int
                        && cfmt.gchordbox != 0
                {
                    printf(b" box\0" as *const u8 as *const libc::c_char);
                }
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
            3 => {
                if (*fd).subtype as libc::c_int == 0 as libc::c_int {
                    printf(
                        b"%.2f\n\0" as *const u8 as *const libc::c_char,
                        *((*fd).v as *mut libc::c_float) as libc::c_double,
                    );
                } else if (*fd).subtype as libc::c_int == 1 as libc::c_int {
                    printf(
                        b"%.2fcm\n\0" as *const u8 as *const libc::c_char,
                        *((*fd).v as *mut libc::c_float) as libc::c_double
                            / (1 as libc::c_int as libc::c_double * 37.8f64),
                    );
                } else {
                    printf(
                        b"%.2fcm\n\0" as *const u8 as *const libc::c_char,
                        (cfmt.pagewidth - cfmt.leftmargin - cfmt.rightmargin)
                            as libc::c_double
                            / (1 as libc::c_int as libc::c_double * 37.8f64),
                    );
                }
            }
            5 => {
                printf(
                    b"\"%s\"\n\0" as *const u8 as *const libc::c_char,
                    if !(*((*fd).v as *mut *mut libc::c_char)).is_null() {
                        *((*fd).v as *mut *mut libc::c_char) as *const libc::c_char
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                );
            }
            _ => {}
        }
        fd = fd.offset(1);
        fd;
    }
}
#[no_mangle]
pub unsafe extern "C" fn scan_u(
    mut str: *mut libc::c_char,
    mut type_0: libc::c_int,
) -> libc::c_float {
    let mut a: libc::c_float = 0.;
    let mut nch: libc::c_int = 0;
    if sscanf(
        str,
        b"%f%n\0" as *const u8 as *const libc::c_char,
        &mut a as *mut libc::c_float,
        &mut nch as *mut libc::c_int,
    ) == 1 as libc::c_int
    {
        str = str.offset(nch as isize);
        if a == 0 as libc::c_int as libc::c_float {
            return 0 as libc::c_int as libc::c_float;
        }
        if *str as libc::c_int == '\0' as i32 || *str as libc::c_int == ' ' as i32 {
            if type_0 != 0 as libc::c_int {
                error(
                    0 as libc::c_int,
                    0 as *mut SYMBOL,
                    b"No unit \"%s\"\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    str.offset(-(nch as isize)),
                );
            }
            return a;
        }
        if strncasecmp(
            str,
            b"pt\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            return a;
        }
        if strncasecmp(
            str,
            b"cm\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            return (if type_0 != 0 {
                a as libc::c_double * 37.8f64
            } else {
                a as libc::c_double * 28.35f64
            }) as libc::c_float;
        }
        if strncasecmp(
            str,
            b"in\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            return (if type_0 != 0 {
                a as libc::c_double * 96.0f64
            } else {
                (a * 72 as libc::c_int as libc::c_float) as libc::c_double
            }) as libc::c_float;
        }
    }
    error(
        1 as libc::c_int,
        0 as *mut SYMBOL,
        b"Unknown unit value \"%s\"\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        str,
    );
    return 20 as libc::c_int as libc::c_float;
}
unsafe extern "C" fn parse_encoding(mut p: *mut libc::c_char) -> libc::c_int {
    return (strncasecmp(
        p,
        b"native\0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn get_posit(mut p: *mut libc::c_char) -> libc::c_int {
    if strcmp(p, b"up\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(p, b"above\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        return 0x1 as libc::c_int;
    }
    if strcmp(p, b"down\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(p, b"below\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(p, b"under\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        return 0x2 as libc::c_int;
    }
    if strcmp(p, b"hidden\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        || strcmp(p, b"opposite\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        return 0x3 as libc::c_int;
    }
    if strcmp(p, b"auto\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn get_textopt(mut p: *mut libc::c_char) -> libc::c_int {
    if strncmp(
        p,
        b"align\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
        || strncmp(
            p,
            b"justify\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if strncmp(
        p,
        b"ragged\0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
        || strncmp(
            p,
            b"fill\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        return 2 as libc::c_int;
    }
    if strncmp(
        p,
        b"center\0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        return 3 as libc::c_int;
    }
    if strncmp(
        p,
        b"skip\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        return 4 as libc::c_int;
    }
    if strncmp(
        p,
        b"right\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        return 5 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_dblrepbar(mut p: *mut libc::c_char) -> libc::c_int {
    let mut bar_type: libc::c_int = 0;
    bar_type = 0 as libc::c_int;
    loop {
        let fresh1 = p;
        p = p.offset(1);
        match *fresh1 as libc::c_int {
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
    return bar_type;
}
#[no_mangle]
pub unsafe extern "C" fn get_bool(mut p: *mut libc::c_char) -> libc::c_int {
    match *p as libc::c_int {
        0 | 49 | 121 | 89 | 116 | 84 => return 1 as libc::c_int,
        48 | 110 | 78 | 102 | 70 => {}
        _ => {
            error(
                0 as libc::c_int,
                0 as *mut SYMBOL,
                b"Unknown logical '%s' - false assumed\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                p,
            );
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn g_fspc(mut p: *mut libc::c_char, mut f: *mut FONTSPEC) {
    let mut fname: [libc::c_char; 80] = [0; 80];
    let mut encoding: libc::c_int = 0;
    let mut fsize: libc::c_float = 0.;
    p = get_str(
        fname.as_mut_ptr(),
        p,
        ::core::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong as libc::c_int,
    );
    if isalpha(*p as libc::c_uchar as libc::c_int) != 0
        || *p as libc::c_int == '*' as i32
    {
        if *p as libc::c_int == '*' as i32 {
            encoding = font_enc[(*f).fnum as usize] as libc::c_int;
        } else {
            encoding = parse_encoding(p);
        }
        while *p as libc::c_int != '\0' as i32
            && isspace(*p as libc::c_uchar as libc::c_int) == 0
        {
            p = p.offset(1);
            p;
        }
        while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
            p = p.offset(1);
            p;
        }
    } else {
        encoding = -(1 as libc::c_int);
    }
    fsize = (*f).size;
    if *p as libc::c_int != '\0' as i32 && *p as libc::c_int != '*' as i32 {
        let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut v: libc::c_float = 0.;
        v = strtod(p, &mut q) as libc::c_float;
        if v <= 0 as libc::c_int as libc::c_float
            || *q as libc::c_int != '\0' as i32 && *q as libc::c_int != ' ' as i32
        {
            error(
                1 as libc::c_int,
                0 as *mut SYMBOL,
                b"Bad font size '%s'\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                p,
            );
        } else {
            fsize = v;
        }
    }
    fontspec(
        f,
        if strcmp(fname.as_mut_ptr(), b"*\0" as *const u8 as *const libc::c_char)
            != 0 as libc::c_int
        {
            fname.as_mut_ptr()
        } else {
            0 as *mut libc::c_char
        },
        encoding,
        fsize,
    );
    if file_initialized <= 0 as libc::c_int {
        used_font[(*f).fnum as usize] = 1 as libc::c_int as libc::c_char;
    }
    if f.offset_from((cfmt.font_tb).as_mut_ptr()) as libc::c_long
        == outft as libc::c_long
    {
        outft = -(1 as libc::c_int);
    }
    pg_reset_font();
}
#[no_mangle]
pub unsafe extern "C" fn tblt_parse(mut p: *mut libc::c_char) -> *mut tblt_s {
    let mut current_block: u64;
    let mut tblt: *mut tblt_s = 0 as *mut tblt_s;
    let mut n: libc::c_int = 0;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut notes_tb: [libc::c_char; 15] = unsafe {
        *::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"CDEFGABcdefgab\0")
    };
    static mut pitch_tb: [libc::c_char; 14] = [
        60 as libc::c_int as libc::c_char,
        62 as libc::c_int as libc::c_char,
        64 as libc::c_int as libc::c_char,
        65 as libc::c_int as libc::c_char,
        67 as libc::c_int as libc::c_char,
        69 as libc::c_int as libc::c_char,
        71 as libc::c_int as libc::c_char,
        72 as libc::c_int as libc::c_char,
        74 as libc::c_int as libc::c_char,
        76 as libc::c_int as libc::c_char,
        77 as libc::c_int as libc::c_char,
        79 as libc::c_int as libc::c_char,
        81 as libc::c_int as libc::c_char,
        83 as libc::c_int as libc::c_char,
    ];
    if *p as libc::c_int == '#' as i32 {
        p = p.offset(1);
        p;
        let fresh2 = p;
        p = p.offset(1);
        n = *fresh2 as libc::c_int - '0' as i32 - 1 as libc::c_int;
        if n as libc::c_uint >= 8 as libc::c_int as libc::c_uint
            || *p as libc::c_int != '\0' as i32 && *p as libc::c_int != ' ' as i32
        {
            error(
                1 as libc::c_int,
                0 as *mut SYMBOL,
                b"Invalid number in %%%%tablature\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return 0 as *mut tblt_s;
        }
        if *p as libc::c_int == '\0' as i32 {
            return tblts[n as usize];
        }
        while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
            p = p.offset(1);
            p;
        }
    } else {
        n = -(1 as libc::c_int);
    }
    tblt = malloc(::core::mem::size_of::<tblt_s>() as libc::c_ulong) as *mut tblt_s;
    memset(
        tblt as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<tblt_s>() as libc::c_ulong,
    );
    if strncmp(
        p,
        b"pitch=\0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        p = p.offset(6 as libc::c_int as isize);
        if *p as libc::c_int == '^' as i32 || *p as libc::c_int == '_' as i32 {
            if *p as libc::c_int == '^' as i32 {
                (*tblt).pitch += 1;
                (*tblt).pitch;
                (*tblt).instr[1 as libc::c_int as usize] = '#' as i32 as libc::c_char;
            } else {
                (*tblt).pitch -= 1;
                (*tblt).pitch;
                (*tblt).instr[1 as libc::c_int as usize] = 'b' as i32 as libc::c_char;
            }
            p = p.offset(1);
            p;
        }
        if *p as libc::c_int == '\0' as i32
            || {
                q = strchr(notes_tb.as_ptr(), *p as libc::c_int);
                q.is_null()
            }
        {
            error(
                1 as libc::c_int,
                0 as *mut SYMBOL,
                b"Invalid pitch in %%%%tablature\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return 0 as *mut tblt_s;
        }
        (*tblt)
            .pitch = ((*tblt).pitch as libc::c_int
            + pitch_tb[q.offset_from(notes_tb.as_ptr()) as libc::c_long as usize]
                as libc::c_int) as libc::c_short;
        let fresh3 = p;
        p = p.offset(1);
        (*tblt)
            .instr[0 as libc::c_int
            as usize] = toupper(*fresh3 as libc::c_int) as libc::c_char;
        while *p as libc::c_int == '\'' as i32 || *p as libc::c_int == ',' as i32 {
            let fresh4 = p;
            p = p.offset(1);
            if *fresh4 as libc::c_int == '\'' as i32 {
                (*tblt)
                    .pitch = ((*tblt).pitch as libc::c_int + 12 as libc::c_int)
                    as libc::c_short;
            } else {
                (*tblt)
                    .pitch = ((*tblt).pitch as libc::c_int - 12 as libc::c_int)
                    as libc::c_short;
            }
        }
        if *p as libc::c_int == '#' as i32 || *p as libc::c_int == 'b' as i32 {
            if *p as libc::c_int == '#' as i32 {
                (*tblt).pitch += 1;
                (*tblt).pitch;
            } else {
                (*tblt).pitch -= 1;
                (*tblt).pitch;
            }
            let fresh5 = p;
            p = p.offset(1);
            (*tblt).instr[1 as libc::c_int as usize] = *fresh5;
        }
        while *p as libc::c_int == '\'' as i32 || *p as libc::c_int == ',' as i32 {
            let fresh6 = p;
            p = p.offset(1);
            if *fresh6 as libc::c_int == '\'' as i32 {
                (*tblt)
                    .pitch = ((*tblt).pitch as libc::c_int + 12 as libc::c_int)
                    as libc::c_short;
            } else {
                (*tblt)
                    .pitch = ((*tblt).pitch as libc::c_int - 12 as libc::c_int)
                    as libc::c_short;
            }
        }
        while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
            p = p.offset(1);
            p;
        }
    }
    if isdigit(*p as libc::c_int) == 0 {
        error(
            1 as libc::c_int,
            0 as *mut SYMBOL,
            b"Invalid width/height in %%%%tablature\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        return 0 as *mut tblt_s;
    }
    (*tblt).hu = scan_u(p, 0 as libc::c_int);
    while *p as libc::c_int != '\0' as i32
        && isspace(*p as libc::c_uchar as libc::c_int) == 0
    {
        p = p.offset(1);
        p;
    }
    while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
        p = p.offset(1);
        p;
    }
    if isdigit(*p as libc::c_int) != 0 {
        (*tblt).ha = (*tblt).hu;
        (*tblt).hu = scan_u(p, 0 as libc::c_int);
        while *p as libc::c_int != '\0' as i32
            && isspace(*p as libc::c_uchar as libc::c_int) == 0
        {
            p = p.offset(1);
            p;
        }
        while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
            p = p.offset(1);
            p;
        }
        if isdigit(*p as libc::c_int) != 0 {
            (*tblt).wh = (*tblt).ha;
            (*tblt).ha = (*tblt).hu;
            (*tblt).hu = scan_u(p, 0 as libc::c_int);
            while *p as libc::c_int != '\0' as i32
                && isspace(*p as libc::c_uchar as libc::c_int) == 0
            {
                p = p.offset(1);
                p;
            }
            while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
                p = p.offset(1);
                p;
            }
        }
    }
    if !(*p as libc::c_int == '\0' as i32) {
        p = strdup(p);
        (*tblt).head = p;
        while *p as libc::c_int != '\0' as i32
            && isspace(*p as libc::c_uchar as libc::c_int) == 0
        {
            p = p.offset(1);
            p;
        }
        if !(*p as libc::c_int == '\0' as i32) {
            let fresh7 = p;
            p = p.offset(1);
            *fresh7 = '\0' as i32 as libc::c_char;
            while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
                p = p.offset(1);
                p;
            }
            (*tblt).note = p;
            while *p as libc::c_int != '\0' as i32
                && isspace(*p as libc::c_uchar as libc::c_int) == 0
            {
                p = p.offset(1);
                p;
            }
            if *p as libc::c_int != '\0' as i32 {
                let fresh8 = p;
                p = p.offset(1);
                *fresh8 = '\0' as i32 as libc::c_char;
                while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
                    p = p.offset(1);
                    p;
                }
                (*tblt).bar = p;
                while *p as libc::c_int != '\0' as i32
                    && isspace(*p as libc::c_uchar as libc::c_int) == 0
                {
                    p = p.offset(1);
                    p;
                }
                if *p as libc::c_int != '\0' as i32 {
                    current_block = 10030445491968229628;
                } else {
                    current_block = 17485376261910781866;
                }
            } else {
                current_block = 17485376261910781866;
            }
            match current_block {
                10030445491968229628 => {}
                _ => {
                    if n >= 0 as libc::c_int {
                        tblts[n as usize] = tblt;
                    }
                    return tblt;
                }
            }
        }
    }
    error(
        1 as libc::c_int,
        0 as *mut SYMBOL,
        b"Wrong values in %%%%tablature\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return 0 as *mut tblt_s;
}
unsafe extern "C" fn set_dyn(mut p_voice: *mut VOICE_S, mut val: libc::c_int) {
    ((*p_voice).posit).set_dyn_0(val as libc::c_ushort);
}
unsafe extern "C" fn set_gch(mut p_voice: *mut VOICE_S, mut val: libc::c_int) {
    ((*p_voice).posit).set_gch(val as libc::c_ushort);
}
unsafe extern "C" fn set_orn(mut p_voice: *mut VOICE_S, mut val: libc::c_int) {
    ((*p_voice).posit).set_orn(val as libc::c_ushort);
}
unsafe extern "C" fn set_voc(mut p_voice: *mut VOICE_S, mut val: libc::c_int) {
    ((*p_voice).posit).set_voc(val as libc::c_ushort);
}
unsafe extern "C" fn set_vol(mut p_voice: *mut VOICE_S, mut val: libc::c_int) {
    ((*p_voice).posit).set_vol(val as libc::c_ushort);
}
unsafe extern "C" fn set_std(mut p_voice: *mut VOICE_S, mut val: libc::c_int) {
    ((*p_voice).posit).set_std(val as libc::c_ushort);
}
unsafe extern "C" fn set_gsd(mut p_voice: *mut VOICE_S, mut val: libc::c_int) {
    ((*p_voice).posit).set_gsd(val as libc::c_ushort);
}
static mut vpar_tb: [vpar; 8] = unsafe {
    [
        {
            let mut init = vpar {
                name: b"dynamic\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                f: Some(set_dyn as unsafe extern "C" fn(*mut VOICE_S, libc::c_int) -> ()),
            };
            init
        },
        {
            let mut init = vpar {
                name: b"gchord\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                f: Some(set_gch as unsafe extern "C" fn(*mut VOICE_S, libc::c_int) -> ()),
            };
            init
        },
        {
            let mut init = vpar {
                name: b"gstemdir\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                f: Some(set_gsd as unsafe extern "C" fn(*mut VOICE_S, libc::c_int) -> ()),
            };
            init
        },
        {
            let mut init = vpar {
                name: b"ornament\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                f: Some(set_orn as unsafe extern "C" fn(*mut VOICE_S, libc::c_int) -> ()),
            };
            init
        },
        {
            let mut init = vpar {
                name: b"stemdir\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                f: Some(set_std as unsafe extern "C" fn(*mut VOICE_S, libc::c_int) -> ()),
            };
            init
        },
        {
            let mut init = vpar {
                name: b"vocal\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                f: Some(set_voc as unsafe extern "C" fn(*mut VOICE_S, libc::c_int) -> ()),
            };
            init
        },
        {
            let mut init = vpar {
                name: b"volume\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                f: Some(set_vol as unsafe extern "C" fn(*mut VOICE_S, libc::c_int) -> ()),
            };
            init
        },
        {
            let mut init = vpar {
                name: 0 as *const libc::c_char as *mut libc::c_char,
                f: None,
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn set_voice_param(
    mut p_voice: *mut VOICE_S,
    mut state: libc::c_int,
    mut w: *mut libc::c_char,
    mut p: *mut libc::c_char,
) {
    let mut vpar: *const vpar = 0 as *const vpar;
    let mut vpar2: *const vpar = 0 as *const vpar;
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    l = strlen(w) as libc::c_int;
    vpar = vpar_tb.as_ptr();
    while !((*vpar).name).is_null() {
        if strncmp(w, (*vpar).name, l as libc::c_ulong) != 0 {
            vpar = vpar.offset(1);
            vpar;
        } else {
            val = get_posit(p);
            break;
        }
    }
    if ((*vpar).name).is_null() {
        val = -(1 as libc::c_int);
        match *w as libc::c_int {
            101 => {
                if strcmp(w, b"exprabove\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    vpar = &*vpar.offset(0 as libc::c_int as isize) as *const vpar;
                    vpar2 = &*vpar.offset(6 as libc::c_int as isize) as *const vpar;
                    if get_bool(p) != 0 {
                        val = 0x1 as libc::c_int;
                    } else {
                        val = 0x2 as libc::c_int;
                    }
                } else if strcmp(w, b"exprbelow\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    vpar = &*vpar.offset(0 as libc::c_int as isize) as *const vpar;
                    vpar2 = &*vpar.offset(6 as libc::c_int as isize) as *const vpar;
                    if get_bool(p) != 0 {
                        val = 0x2 as libc::c_int;
                    } else {
                        val = 0x1 as libc::c_int;
                    }
                }
            }
            118 => {
                if strcmp(w, b"vocalabove\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    vpar = &*vpar.offset(5 as libc::c_int as isize) as *const vpar;
                    if get_bool(p) != 0 {
                        val = 0x1 as libc::c_int;
                    } else {
                        val = 0x2 as libc::c_int;
                    }
                }
            }
            _ => {}
        }
        if val < 0 as libc::c_int {
            error(
                1 as libc::c_int,
                0 as *mut SYMBOL,
                b"Bad value %%%%%s %s\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                w,
                p,
            );
            return;
        }
    }
    if state == 2 as libc::c_int {
        ((*vpar).f).expect("non-null function pointer")(p_voice, val);
        if !vpar2.is_null() {
            ((*vpar2).f).expect("non-null function pointer")(p_voice, val);
        }
        return;
    }
    i = 32 as libc::c_int;
    p_voice = voice_tb.as_mut_ptr();
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int) {
            break;
        }
        ((*vpar).f).expect("non-null function pointer")(p_voice, val);
        if !vpar2.is_null() {
            ((*vpar2).f).expect("non-null function pointer")(p_voice, val);
        }
        p_voice = p_voice.offset(1);
        p_voice;
    }
    cfmt.posit = voice_tb[0 as libc::c_int as usize].posit;
}
#[no_mangle]
pub unsafe extern "C" fn interpret_fmt_line(
    mut w: *mut libc::c_char,
    mut p: *mut libc::c_char,
    mut lock: libc::c_int,
) {
    let mut current_block: u64;
    let mut fd: *mut format = 0 as *mut format;
    let mut i: libc::c_int = 0;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut f: libc::c_float = 0.;
    match *w as libc::c_int {
        98 => {
            if strcmp(w, b"barnumbers\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                w = b"measurenb\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            current_block = 17711149709958600598;
        }
        99 => {
            if strcmp(w, b"comball\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                cfmt.combinevoices = 2 as libc::c_int;
                return;
            }
            current_block = 17711149709958600598;
        }
        102 => {
            if strcmp(w, b"font\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                let mut fnum: libc::c_int = 0;
                let mut encoding: libc::c_int = 0;
                let mut swfac: libc::c_float = 0.;
                let mut fname: [libc::c_char; 80] = [0; 80];
                if file_initialized > 0 as libc::c_int && svg == 0
                    && epsf <= 1 as libc::c_int
                {
                    error(
                        1 as libc::c_int,
                        0 as *mut SYMBOL,
                        b"Cannot define a font when the output file is opened\0"
                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                    return;
                }
                p = get_str(
                    fname.as_mut_ptr(),
                    p,
                    ::core::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong
                        as libc::c_int,
                );
                swfac = 0 as libc::c_int as libc::c_float;
                encoding = 0 as libc::c_int;
                if *p as libc::c_int != '\0' as i32 {
                    if isalpha(*p as libc::c_uchar as libc::c_int) != 0 {
                        encoding = parse_encoding(p);
                        while *p as libc::c_int != '\0' as i32
                            && isspace(*p as libc::c_uchar as libc::c_int) == 0
                        {
                            p = p.offset(1);
                            p;
                        }
                        while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
                            p = p.offset(1);
                            p;
                        }
                    }
                    if isdigit(*p as libc::c_uchar as libc::c_int) != 0 {
                        f = strtod(p, &mut q) as libc::c_float;
                        if f > 2 as libc::c_int as libc::c_float
                            || *q as libc::c_int != '\0' as i32
                                && *q as libc::c_int != '\0' as i32
                        {
                            current_block = 11158206164158514448;
                        } else {
                            swfac = f;
                            current_block = 15768484401365413375;
                        }
                    } else {
                        current_block = 15768484401365413375;
                    }
                } else {
                    current_block = 15768484401365413375;
                }
                match current_block {
                    11158206164158514448 => {}
                    _ => {
                        fnum = get_font(fname.as_mut_ptr(), encoding);
                        def_font_enc[fnum as usize] = encoding as libc::c_char;
                        swfac_font[fnum as usize] = swfac;
                        used_font[fnum as usize] = 1 as libc::c_int as libc::c_char;
                        i = 10 as libc::c_int;
                        while i < FONT_DYN as libc::c_int + 12 as libc::c_int {
                            if cfmt.font_tb[i as usize].fnum == fnum {
                                cfmt
                                    .font_tb[i as usize]
                                    .swfac = cfmt.font_tb[i as usize].size * swfac;
                            }
                            i += 1;
                            i;
                        }
                        return;
                    }
                }
            } else {
                current_block = 17711149709958600598;
            }
        }
        105 => {
            if strcmp(w, b"infoname\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                if (*p as libc::c_int) < 'A' as i32 || *p as libc::c_int > 'Z' as i32 {
                    current_block = 11158206164158514448;
                } else {
                    set_infoname(p);
                    return;
                }
            } else {
                current_block = 17711149709958600598;
            }
        }
        109 => {
            if strcmp(w, b"musiconly\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                if get_bool(p) != 0 {
                    cfmt.fields[1 as libc::c_int as usize]
                        &= !((1 as libc::c_int) << 'w' as i32 - 'a' as i32)
                            as libc::c_uint;
                } else {
                    cfmt.fields[1 as libc::c_int as usize]
                        |= ((1 as libc::c_int) << 'w' as i32 - 'a' as i32)
                            as libc::c_uint;
                }
                return;
            }
            current_block = 17711149709958600598;
        }
        112 => {
            if strcmp(w, b"printparts\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                if get_bool(p) != 0 {
                    cfmt.fields[0 as libc::c_int as usize]
                        |= ((1 as libc::c_int) << 'P' as i32 - 'A' as i32)
                            as libc::c_uint;
                } else {
                    cfmt.fields[0 as libc::c_int as usize]
                        &= !((1 as libc::c_int) << 'P' as i32 - 'A' as i32)
                            as libc::c_uint;
                }
                return;
            }
            if strcmp(w, b"printtempo\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                if get_bool(p) != 0 {
                    cfmt.fields[0 as libc::c_int as usize]
                        |= ((1 as libc::c_int) << 'Q' as i32 - 'A' as i32)
                            as libc::c_uint;
                } else {
                    cfmt.fields[0 as libc::c_int as usize]
                        &= !((1 as libc::c_int) << 'Q' as i32 - 'A' as i32)
                            as libc::c_uint;
                }
                return;
            }
            current_block = 17711149709958600598;
        }
        115 => {
            if strncmp(
                w,
                b"setfont-\0" as *const u8 as *const libc::c_char,
                8 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                i = *w.offset(8 as libc::c_int as isize) as libc::c_int - '0' as i32;
                if i < 0 as libc::c_int || i >= 10 as libc::c_int {
                    return;
                }
                g_fspc(p, &mut *(cfmt.font_tb).as_mut_ptr().offset(i as isize));
                return;
            }
            if strcmp(w, b"scale\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                fd = format_tb.as_mut_ptr();
                while !((*fd).name).is_null() {
                    if strcmp(
                        b"pagescale\0" as *const u8 as *const libc::c_char,
                        (*fd).name,
                    ) == 0 as libc::c_int
                    {
                        break;
                    }
                    fd = fd.offset(1);
                    fd;
                }
                if (*fd).lock != 0 {
                    return;
                }
                (*fd).lock = lock as libc::c_short;
                f = strtod(p, &mut q) as libc::c_float;
                if *q as libc::c_int != '\0' as i32 && *q as libc::c_int != ' ' as i32 {
                    current_block = 11158206164158514448;
                } else if (f as libc::c_double) < 0.1f64 || f as libc::c_double > 10.0f64
                {
                    current_block = 11158206164158514448;
                } else {
                    cfmt.scale = (f as libc::c_double / 0.75f64) as libc::c_float;
                    return;
                }
            } else {
                current_block = 17711149709958600598;
            }
        }
        119 => {
            if strcmp(w, b"withxrefs\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                if get_bool(p) != 0 {
                    cfmt.fields[0 as libc::c_int as usize]
                        |= ((1 as libc::c_int) << 'X' as i32 - 'A' as i32)
                            as libc::c_uint;
                } else {
                    cfmt.fields[0 as libc::c_int as usize]
                        &= !((1 as libc::c_int) << 'X' as i32 - 'A' as i32)
                            as libc::c_uint;
                }
                return;
            }
            if strcmp(w, b"writehistory\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
                let mut bool: libc::c_int = 0;
                let mut u: libc::c_uint = 0;
                bool = get_bool(p);
                s = info[('I' as i32 - 'A' as i32) as usize];
                while !s.is_null() {
                    u = (*((*s).text).offset(0 as libc::c_int as isize) as libc::c_int
                        - 'A' as i32) as libc::c_uint;
                    if bool != 0 {
                        cfmt.fields[0 as libc::c_int as usize]
                            |= ((1 as libc::c_int) << u) as libc::c_uint;
                    } else {
                        cfmt.fields[0 as libc::c_int as usize]
                            &= !((1 as libc::c_int) << u) as libc::c_uint;
                    }
                    s = (*s).next;
                }
                return;
            }
            current_block = 17711149709958600598;
        }
        _ => {
            current_block = 17711149709958600598;
        }
    }
    match current_block {
        17711149709958600598 => {
            fd = format_tb.as_mut_ptr();
            while !((*fd).name).is_null() {
                if strcmp(w, (*fd).name) == 0 as libc::c_int {
                    break;
                }
                fd = fd.offset(1);
                fd;
            }
            if ((*fd).name).is_null() {
                return;
            }
            i = strlen(p) as libc::c_int;
            if strcmp(
                p.offset(i as isize).offset(-(5 as libc::c_int as isize)),
                b" lock\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                *p.offset((i - 5 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
                lock = 1 as libc::c_int;
            }
            if lock != 0 {
                (*fd).lock = 1 as libc::c_int as libc::c_short;
            } else if (*fd).lock != 0 {
                return
            }
            match (*fd).type_0 as libc::c_int {
                4 => {
                    let mut current_block_119: u64;
                    match (*fd).subtype as libc::c_int {
                        2 => {
                            if svg != 0 || epsf > 1 as libc::c_int {
                                current_block_119 = 1930794479672247912;
                            } else if *p as libc::c_int == '2' as i32 {
                                cfmt.pango = 2 as libc::c_int;
                                current_block_119 = 1930794479672247912;
                            } else {
                                current_block_119 = 6718427872050625195;
                            }
                        }
                        1 => {
                            let mut bool_0: libc::c_int = 0;
                            let mut u_0: libc::c_uint = 0;
                            q = p;
                            while *p as libc::c_int != '\0' as i32
                                && isspace(*p as libc::c_uchar as libc::c_int) == 0
                            {
                                p = p.offset(1);
                                p;
                            }
                            while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
                                p = p.offset(1);
                                p;
                            }
                            bool_0 = get_bool(p);
                            while *q as libc::c_int != '\0' as i32
                                && isspace(*q as libc::c_uchar as libc::c_int) == 0
                            {
                                u_0 = (*q as libc::c_int - 'A' as i32) as libc::c_uint;
                                if u_0 < 26 as libc::c_int as libc::c_uint {
                                    i = 0 as libc::c_int;
                                } else {
                                    u_0 = (*q as libc::c_int - 'a' as i32) as libc::c_uint;
                                    if !(u_0 < 26 as libc::c_int as libc::c_uint) {
                                        break;
                                    }
                                    i = 1 as libc::c_int;
                                }
                                if bool_0 != 0 {
                                    cfmt.fields[i as usize]
                                        |= ((1 as libc::c_int) << u_0) as libc::c_uint;
                                } else {
                                    cfmt.fields[i as usize]
                                        &= !((1 as libc::c_int) << u_0) as libc::c_uint;
                                }
                                q = q.offset(1);
                                q;
                            }
                            current_block_119 = 1930794479672247912;
                        }
                        0 | 3 | _ => {
                            current_block_119 = 6718427872050625195;
                        }
                    }
                    match current_block_119 {
                        6718427872050625195 => {
                            *((*fd).v as *mut libc::c_int) = get_bool(p);
                            if (*fd).subtype as libc::c_int == 3 as libc::c_int {
                                if cfmt.abc2pscompat != 0 {
                                    deco['M' as i32
                                        as usize] = b"tenuto\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char;
                                } else {
                                    deco['M' as i32
                                        as usize] = b"lowermordent\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char;
                                }
                            }
                        }
                        _ => {}
                    }
                    current_block = 2102591061404402436;
                }
                0 => {
                    if (*fd).subtype as libc::c_int == 3 as libc::c_int {
                        let mut i1: libc::c_uint = 0;
                        let mut i2: libc::c_uint = 0;
                        let mut i3: libc::c_uint = 0;
                        let mut i4: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                        if sscanf(
                            p,
                            b"%d %d %d %d\0" as *const u8 as *const libc::c_char,
                            &mut i1 as *mut libc::c_uint,
                            &mut i2 as *mut libc::c_uint,
                            &mut i3 as *mut libc::c_uint,
                            &mut i4 as *mut libc::c_uint,
                        ) != 4 as libc::c_int
                            && sscanf(
                                p,
                                b"%d %d %d\0" as *const u8 as *const libc::c_char,
                                &mut i1 as *mut libc::c_uint,
                                &mut i2 as *mut libc::c_uint,
                                &mut i3 as *mut libc::c_uint,
                            ) != 3 as libc::c_int
                        {
                            current_block = 11158206164158514448;
                        } else if i1 > 2 as libc::c_int as libc::c_uint
                            || i2 > 2 as libc::c_int as libc::c_uint
                            || i3 > 2 as libc::c_int as libc::c_uint
                            || i4 > 2 as libc::c_int as libc::c_uint
                        {
                            current_block = 11158206164158514448;
                        } else {
                            cfmt
                                .tuplets = (i1 << 12 as libc::c_int | i2 << 8 as libc::c_int
                                | i3 << 4 as libc::c_int | i4) as libc::c_int;
                            current_block = 2102591061404402436;
                        }
                    } else if (*fd).subtype as libc::c_int == 5 as libc::c_int {
                        let mut i1_0: libc::c_uint = 0;
                        let mut i2_0: libc::c_uint = 0;
                        let mut i3_0: libc::c_uint = 0;
                        let mut f1: libc::c_float = 0.;
                        let mut f2: libc::c_float = 0.;
                        let mut f3: libc::c_float = 0.;
                        if sscanf(
                            p,
                            b"%f %f %f\0" as *const u8 as *const libc::c_char,
                            &mut f1 as *mut libc::c_float,
                            &mut f2 as *mut libc::c_float,
                            &mut f3 as *mut libc::c_float,
                        ) != 3 as libc::c_int || f1 > 25 as libc::c_int as libc::c_float
                            || f2 > 25 as libc::c_int as libc::c_float
                            || f3 > 25 as libc::c_int as libc::c_float
                        {
                            current_block = 11158206164158514448;
                        } else {
                            i1_0 = (f1 * 10 as libc::c_int as libc::c_float)
                                as libc::c_uint;
                            i2_0 = (f2 * 10 as libc::c_int as libc::c_float)
                                as libc::c_uint;
                            i3_0 = (f3 * 10 as libc::c_int as libc::c_float)
                                as libc::c_uint;
                            cfmt
                                .gracespace = (i1_0 << 16 as libc::c_int
                                | i2_0 << 8 as libc::c_int | i3_0) as libc::c_int;
                            current_block = 2102591061404402436;
                        }
                    } else {
                        if (*fd).subtype as libc::c_int == 1 as libc::c_int
                            && (strcmp(p, b"odd\0" as *const u8 as *const libc::c_char)
                                == 0 as libc::c_int
                                || strcmp(p, b"even\0" as *const u8 as *const libc::c_char)
                                    == 0 as libc::c_int)
                        {
                            cfmt
                                .splittune = if *p.offset(0 as libc::c_int as isize)
                                as libc::c_int == 'e' as i32
                            {
                                2 as libc::c_int
                            } else {
                                3 as libc::c_int
                            };
                        } else if (*fd).subtype as libc::c_int == 2 as libc::c_int {
                            cfmt.dblrepbar = get_dblrepbar(p);
                        } else if (*fd).subtype as libc::c_int == 4 as libc::c_int
                            && isdigit(*p as libc::c_int) == 0
                        {
                            cfmt.textoption = get_textopt(p);
                        } else if isdigit(*p as libc::c_int) != 0
                            || *p as libc::c_int == '-' as i32
                            || *p as libc::c_int == '+' as i32
                        {
                            sscanf(
                                p,
                                b"%d\0" as *const u8 as *const libc::c_char,
                                (*fd).v as *mut libc::c_int,
                            );
                        } else {
                            *((*fd).v as *mut libc::c_int) = get_bool(p);
                        }
                        if (*fd).subtype as libc::c_int == 4 as libc::c_int {
                            if cfmt.textoption < 0 as libc::c_int {
                                cfmt.textoption = 0 as libc::c_int;
                                current_block = 11158206164158514448;
                            } else {
                                current_block = 2102591061404402436;
                            }
                        } else {
                            current_block = 2102591061404402436;
                        }
                    }
                }
                1 => {
                    f = strtod(p, &mut q) as libc::c_float;
                    if *q as libc::c_int != '\0' as i32
                        && *q as libc::c_int != ' ' as i32
                    {
                        current_block = 11158206164158514448;
                    } else {
                        match (*fd).subtype as libc::c_int {
                            1 => {
                                let mut v2: libc::c_float = 0.;
                                if f < 1 as libc::c_int as libc::c_float
                                    || f > 2 as libc::c_int as libc::c_float
                                {
                                    current_block = 11158206164158514448;
                                } else {
                                    i = 5 as libc::c_int;
                                    v2 = space_tb[i as usize];
                                    loop {
                                        i -= 1;
                                        if !(i >= 0 as libc::c_int) {
                                            break;
                                        }
                                        v2 /= f;
                                        space_tb[i as usize] = v2;
                                    }
                                    i = 5 as libc::c_int;
                                    v2 = space_tb[i as usize];
                                    loop {
                                        i += 1;
                                        if !(i < 10 as libc::c_int) {
                                            break;
                                        }
                                        v2 *= f;
                                        space_tb[i as usize] = v2;
                                    }
                                    current_block = 3297745280902459415;
                                }
                            }
                            2 => {
                                if f < 0 as libc::c_int as libc::c_float
                                    || f > 1 as libc::c_int as libc::c_float
                                {
                                    current_block = 11158206164158514448;
                                } else {
                                    current_block = 3297745280902459415;
                                }
                            }
                            3 => {
                                if (f as libc::c_double) < 0.5f64
                                    || f > 1 as libc::c_int as libc::c_float
                                {
                                    current_block = 11158206164158514448;
                                } else {
                                    current_block = 3297745280902459415;
                                }
                            }
                            _ => {
                                if f <= 0 as libc::c_int as libc::c_float {
                                    current_block = 11158206164158514448;
                                } else {
                                    current_block = 3297745280902459415;
                                }
                            }
                        }
                        match current_block {
                            11158206164158514448 => {}
                            _ => {
                                *((*fd).v as *mut libc::c_float) = f;
                                current_block = 2102591061404402436;
                            }
                        }
                    }
                }
                2 => {
                    let mut b: libc::c_int = 0;
                    g_fspc(p, (*fd).v as *mut FONTSPEC);
                    b = (strstr(p, b"box\0" as *const u8 as *const libc::c_char)
                        != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
                    match (*fd).subtype as libc::c_int {
                        1 => {
                            cfmt.partsbox = b;
                        }
                        2 => {
                            cfmt.measurebox = b;
                        }
                        3 => {
                            cfmt.gchordbox = b;
                        }
                        _ => {}
                    }
                    current_block = 2102591061404402436;
                }
                3 => {
                    *((*fd).v
                        as *mut libc::c_float) = scan_u(p, (*fd).subtype as libc::c_int);
                    match (*fd).subtype as libc::c_int {
                        1 => {
                            if !(strcmp(
                                (*fd).name,
                                b"rightmargin\0" as *const u8 as *const libc::c_char,
                            ) != 0 as libc::c_int
                                && strcmp(
                                    (*fd).name,
                                    b"leftmargin\0" as *const u8 as *const libc::c_char,
                                ) != 0 as libc::c_int)
                            {
                                staffwidth = cfmt.pagewidth - cfmt.leftmargin
                                    - cfmt.rightmargin;
                                if !(staffwidth > 100 as libc::c_int as libc::c_float) {
                                    error(
                                        1 as libc::c_int,
                                        0 as *mut SYMBOL,
                                        b"'staffwidth' too small\n\0" as *const u8
                                            as *const libc::c_char as *mut libc::c_char,
                                    );
                                    staffwidth = 100 as libc::c_int as libc::c_float;
                                    if *((*fd).name).offset(0 as libc::c_int as isize)
                                        as libc::c_int == 'r' as i32
                                    {
                                        cfmt
                                            .rightmargin = cfmt.pagewidth - cfmt.leftmargin
                                            - staffwidth;
                                    } else {
                                        cfmt
                                            .leftmargin = cfmt.pagewidth - cfmt.rightmargin
                                            - staffwidth;
                                    }
                                }
                            }
                        }
                        2 => {
                            if staffwidth < 100 as libc::c_int as libc::c_float {
                                error(
                                    1 as libc::c_int,
                                    0 as *mut SYMBOL,
                                    b"'staffwidth' too small\n\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                );
                            } else {
                                f = (if cfmt.landscape != 0 {
                                    cfmt.pageheight
                                } else {
                                    cfmt.pagewidth
                                }) - staffwidth - cfmt.leftmargin;
                                if f < 0 as libc::c_int as libc::c_float {
                                    error(
                                        1 as libc::c_int,
                                        0 as *mut SYMBOL,
                                        b"'staffwidth' too big\n\0" as *const u8
                                            as *const libc::c_char as *mut libc::c_char,
                                    );
                                } else {
                                    cfmt.rightmargin = f;
                                }
                            }
                        }
                        _ => {}
                    }
                    current_block = 2102591061404402436;
                }
                5 => {
                    i = (strlen(p)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                        as libc::c_int;
                    let ref mut fresh9 = *((*fd).v as *mut *mut libc::c_char);
                    *fresh9 = getarena(i) as *mut libc::c_char;
                    if *p as libc::c_int == '"' as i32 {
                        get_str(*((*fd).v as *mut *mut libc::c_char), p, i);
                    } else {
                        strcpy(*((*fd).v as *mut *mut libc::c_char), p);
                    }
                    if (*fd).subtype as libc::c_int == 1 as libc::c_int {
                        svg_font_switch();
                    }
                    current_block = 2102591061404402436;
                }
                _ => {
                    current_block = 2102591061404402436;
                }
            }
            match current_block {
                11158206164158514448 => {}
                _ => return,
            }
        }
        _ => {}
    }
    error(
        1 as libc::c_int,
        0 as *mut SYMBOL,
        b"Bad value '%s' for '%s' - ignored\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        p,
        w,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lock_fmt(mut fmt: *mut libc::c_void) {
    let mut fd: *mut format = 0 as *mut format;
    fd = format_tb.as_mut_ptr();
    while !((*fd).name).is_null() {
        if (*fd).v == fmt {
            break;
        }
        fd = fd.offset(1);
        fd;
    }
    if ((*fd).name).is_null() {
        return;
    }
    (*fd).lock = 1 as libc::c_int as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn set_font(mut ft: libc::c_int) {
    let mut fnum: libc::c_int = 0;
    let mut f: *mut FONTSPEC = 0 as *mut FONTSPEC;
    let mut f2: *mut FONTSPEC = 0 as *mut FONTSPEC;
    if ft == outft {
        return;
    }
    f = &mut *(cfmt.font_tb).as_mut_ptr().offset(ft as isize) as *mut FONTSPEC;
    if outft >= 0 as libc::c_int {
        f2 = &mut *(cfmt.font_tb).as_mut_ptr().offset(outft as isize) as *mut FONTSPEC;
        outft = ft;
        fnum = (*f).fnum;
        if fnum == (*f2).fnum && (*f).size == (*f2).size {
            return;
        }
    } else {
        outft = ft;
        fnum = (*f).fnum;
    }
    if used_font[fnum as usize] == 0 && epsf <= 1 as libc::c_int && svg == 0 {
        if file_initialized <= 0 as libc::c_int {
            used_font[fnum as usize] = 1 as libc::c_int as libc::c_char;
        } else {
            error(
                1 as libc::c_int,
                0 as *mut SYMBOL,
                b"Font '%s' not predefined; using first in list\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                fontnames[fnum as usize],
            );
            fnum = 0 as libc::c_int;
        }
    }
    if (*f).size == 0 as libc::c_int as libc::c_float {
        error(
            0 as libc::c_int,
            0 as *mut SYMBOL,
            b"Font '%s' with a null size - set to 8\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            fontnames[fnum as usize],
        );
        (*f).size = 8 as libc::c_int as libc::c_float;
    }
    a2b(
        b"%.1f F%d \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*f).size as libc::c_double,
        fnum,
    );
}
#[no_mangle]
pub unsafe extern "C" fn get_font_encoding(mut ft: libc::c_int) -> libc::c_int {
    return font_enc[cfmt.font_tb[ft as usize].fnum as usize] as libc::c_int;
}
unsafe extern "C" fn run_static_initializers() {
    format_tb = [
        {
            let mut init = format {
                name: b"abc2pscompat\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.abc2pscompat as *mut libc::c_int as *mut libc::c_void,
                type_0: 4 as libc::c_int as libc::c_char,
                subtype: 3 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"alignbars\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.alignbars as *mut libc::c_int as *mut libc::c_void,
                type_0: 0 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"aligncomposer\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.aligncomposer as *mut libc::c_int as *mut libc::c_void,
                type_0: 0 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"annotationfont\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut *(cfmt.font_tb)
                    .as_mut_ptr()
                    .offset(ANNOTATIONFONT as libc::c_int as isize) as *mut FONTSPEC
                    as *mut libc::c_void,
                type_0: 2 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"autoclef\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.autoclef as *mut libc::c_int as *mut libc::c_void,
                type_0: 4 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"barsperstaff\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.barsperstaff as *mut libc::c_int as *mut libc::c_void,
                type_0: 0 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"bgcolor\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.bgcolor as *mut *mut libc::c_char as *mut libc::c_void,
                type_0: 5 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"botmargin\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.botmargin as *mut libc::c_float as *mut libc::c_void,
                type_0: 3 as libc::c_int as libc::c_char,
                subtype: 1 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"breaklimit\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.breaklimit as *mut libc::c_float as *mut libc::c_void,
                type_0: 1 as libc::c_int as libc::c_char,
                subtype: 3 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"breakoneoln\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.breakoneoln as *mut libc::c_int as *mut libc::c_void,
                type_0: 4 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"bstemdown\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.bstemdown as *mut libc::c_int as *mut libc::c_void,
                type_0: 4 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"cancelkey\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.cancelkey as *mut libc::c_int as *mut libc::c_void,
                type_0: 4 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"capo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                v: &mut cfmt.capo as *mut libc::c_int as *mut libc::c_void,
                type_0: 0 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"combinevoices\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.combinevoices as *mut libc::c_int as *mut libc::c_void,
                type_0: 0 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"composerfont\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut *(cfmt.font_tb)
                    .as_mut_ptr()
                    .offset(COMPOSERFONT as libc::c_int as isize) as *mut FONTSPEC
                    as *mut libc::c_void,
                type_0: 2 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"composerspace\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.composerspace as *mut libc::c_float as *mut libc::c_void,
                type_0: 3 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"contbarnb\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.contbarnb as *mut libc::c_int as *mut libc::c_void,
                type_0: 4 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"continueall\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.continueall as *mut libc::c_int as *mut libc::c_void,
                type_0: 4 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"custos\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.custos as *mut libc::c_int as *mut libc::c_void,
                type_0: 4 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"dateformat\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.dateformat as *mut *mut libc::c_char as *mut libc::c_void,
                type_0: 5 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"dblrepbar\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.dblrepbar as *mut libc::c_int as *mut libc::c_void,
                type_0: 0 as libc::c_int as libc::c_char,
                subtype: 2 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"decoerr\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.decoerr as *mut libc::c_int as *mut libc::c_void,
                type_0: 4 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"dynalign\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.dynalign as *mut libc::c_int as *mut libc::c_void,
                type_0: 4 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"footer\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.footer as *mut *mut libc::c_char as *mut libc::c_void,
                type_0: 5 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"footer2\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.footer2 as *mut *mut libc::c_char as *mut libc::c_void,
                type_0: 5 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"footerfont\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut *(cfmt.font_tb)
                    .as_mut_ptr()
                    .offset(FOOTERFONT as libc::c_int as isize) as *mut FONTSPEC
                    as *mut libc::c_void,
                type_0: 2 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"flatbeams\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.flatbeams as *mut libc::c_int as *mut libc::c_void,
                type_0: 4 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"flatbeamgracing\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.flatbeamgracing as *mut libc::c_int as *mut libc::c_void,
                type_0: 4 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"gchordbox\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.gchordbox as *mut libc::c_int as *mut libc::c_void,
                type_0: 4 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"gchordfont\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut *(cfmt.font_tb)
                    .as_mut_ptr()
                    .offset(GCHORDFONT as libc::c_int as isize) as *mut FONTSPEC
                    as *mut libc::c_void,
                type_0: 2 as libc::c_int as libc::c_char,
                subtype: 3 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"graceslurs\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.graceslurs as *mut libc::c_int as *mut libc::c_void,
                type_0: 4 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"graceword\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.graceword as *mut libc::c_int as *mut libc::c_void,
                type_0: 4 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"gracespace\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.gracespace as *mut libc::c_int as *mut libc::c_void,
                type_0: 0 as libc::c_int as libc::c_char,
                subtype: 5 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"gutter\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.gutter as *mut libc::c_float as *mut libc::c_void,
                type_0: 3 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"header\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.header as *mut *mut libc::c_char as *mut libc::c_void,
                type_0: 5 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"header2\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.header2 as *mut *mut libc::c_char as *mut libc::c_void,
                type_0: 5 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"headerfont\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut *(cfmt.font_tb)
                    .as_mut_ptr()
                    .offset(HEADERFONT as libc::c_int as isize) as *mut FONTSPEC
                    as *mut libc::c_void,
                type_0: 2 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"historyfont\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut *(cfmt.font_tb)
                    .as_mut_ptr()
                    .offset(HISTORYFONT as libc::c_int as isize) as *mut FONTSPEC
                    as *mut libc::c_void,
                type_0: 2 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"hyphencont\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.hyphencont as *mut libc::c_int as *mut libc::c_void,
                type_0: 4 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"indent\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.indent as *mut libc::c_float as *mut libc::c_void,
                type_0: 3 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"infofont\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut *(cfmt.font_tb)
                    .as_mut_ptr()
                    .offset(INFOFONT as libc::c_int as isize) as *mut FONTSPEC
                    as *mut libc::c_void,
                type_0: 2 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"infoline\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.infoline as *mut libc::c_int as *mut libc::c_void,
                type_0: 4 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"infospace\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.infospace as *mut libc::c_float as *mut libc::c_void,
                type_0: 3 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"keywarn\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.keywarn as *mut libc::c_int as *mut libc::c_void,
                type_0: 4 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"landscape\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.landscape as *mut libc::c_int as *mut libc::c_void,
                type_0: 4 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"leftmargin\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.leftmargin as *mut libc::c_float as *mut libc::c_void,
                type_0: 3 as libc::c_int as libc::c_char,
                subtype: 1 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"lineskipfac\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.lineskipfac as *mut libc::c_float as *mut libc::c_void,
                type_0: 1 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"linewarn\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.linewarn as *mut libc::c_int as *mut libc::c_void,
                type_0: 4 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"maxshrink\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.maxshrink as *mut libc::c_float as *mut libc::c_void,
                type_0: 1 as libc::c_int as libc::c_char,
                subtype: 2 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"maxstaffsep\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.maxstaffsep as *mut libc::c_float as *mut libc::c_void,
                type_0: 3 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"maxsysstaffsep\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.maxsysstaffsep as *mut libc::c_float as *mut libc::c_void,
                type_0: 3 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"measurebox\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.measurebox as *mut libc::c_int as *mut libc::c_void,
                type_0: 4 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"measurefirst\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.measurefirst as *mut libc::c_int as *mut libc::c_void,
                type_0: 0 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"measurefont\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut *(cfmt.font_tb)
                    .as_mut_ptr()
                    .offset(MEASUREFONT as libc::c_int as isize) as *mut FONTSPEC
                    as *mut libc::c_void,
                type_0: 2 as libc::c_int as libc::c_char,
                subtype: 2 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"measurenb\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.measurenb as *mut libc::c_int as *mut libc::c_void,
                type_0: 0 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"musicfont\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.musicfont as *mut *mut libc::c_char as *mut libc::c_void,
                type_0: 5 as libc::c_int as libc::c_char,
                subtype: 1 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"musicspace\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.musicspace as *mut libc::c_float as *mut libc::c_void,
                type_0: 3 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"notespacingfactor\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.notespacingfactor as *mut libc::c_float
                    as *mut libc::c_void,
                type_0: 1 as libc::c_int as libc::c_char,
                subtype: 1 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"oneperpage\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.oneperpage as *mut libc::c_int as *mut libc::c_void,
                type_0: 4 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"pageheight\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.pageheight as *mut libc::c_float as *mut libc::c_void,
                type_0: 3 as libc::c_int as libc::c_char,
                subtype: 1 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"pagewidth\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.pagewidth as *mut libc::c_float as *mut libc::c_void,
                type_0: 3 as libc::c_int as libc::c_char,
                subtype: 1 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"pagescale\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.scale as *mut libc::c_float as *mut libc::c_void,
                type_0: 1 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"pango\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.pango as *mut libc::c_int as *mut libc::c_void,
                type_0: 4 as libc::c_int as libc::c_char,
                subtype: 2 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"parskipfac\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.parskipfac as *mut libc::c_float as *mut libc::c_void,
                type_0: 1 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"partsbox\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.partsbox as *mut libc::c_int as *mut libc::c_void,
                type_0: 4 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"partsfont\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut *(cfmt.font_tb)
                    .as_mut_ptr()
                    .offset(PARTSFONT as libc::c_int as isize) as *mut FONTSPEC
                    as *mut libc::c_void,
                type_0: 2 as libc::c_int as libc::c_char,
                subtype: 1 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"partsspace\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.partsspace as *mut libc::c_float as *mut libc::c_void,
                type_0: 3 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"pdfmark\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.pdfmark as *mut libc::c_int as *mut libc::c_void,
                type_0: 0 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"rbdbstop\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.rbdbstop as *mut libc::c_int as *mut libc::c_void,
                type_0: 4 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"rbmax\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.rbmax as *mut libc::c_int as *mut libc::c_void,
                type_0: 0 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"rbmin\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.rbmin as *mut libc::c_int as *mut libc::c_void,
                type_0: 0 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"repeatfont\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut *(cfmt.font_tb)
                    .as_mut_ptr()
                    .offset(REPEATFONT as libc::c_int as isize) as *mut FONTSPEC
                    as *mut libc::c_void,
                type_0: 2 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"rightmargin\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.rightmargin as *mut libc::c_float as *mut libc::c_void,
                type_0: 3 as libc::c_int as libc::c_char,
                subtype: 1 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"setdefl\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.setdefl as *mut libc::c_int as *mut libc::c_void,
                type_0: 4 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"shiftunison\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.shiftunison as *mut libc::c_int as *mut libc::c_void,
                type_0: 0 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"slurheight\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.slurheight as *mut libc::c_float as *mut libc::c_void,
                type_0: 1 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"splittune\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.splittune as *mut libc::c_int as *mut libc::c_void,
                type_0: 0 as libc::c_int as libc::c_char,
                subtype: 1 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"squarebreve\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.squarebreve as *mut libc::c_int as *mut libc::c_void,
                type_0: 4 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"staffnonote\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.staffnonote as *mut libc::c_int as *mut libc::c_void,
                type_0: 0 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"staffsep\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.staffsep as *mut libc::c_float as *mut libc::c_void,
                type_0: 3 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"staffwidth\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut staffwidth as *mut libc::c_float as *mut libc::c_void,
                type_0: 3 as libc::c_int as libc::c_char,
                subtype: 2 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"stemheight\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.stemheight as *mut libc::c_float as *mut libc::c_void,
                type_0: 1 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"straightflags\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.straightflags as *mut libc::c_int as *mut libc::c_void,
                type_0: 4 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"stretchlast\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.stretchlast as *mut libc::c_float as *mut libc::c_void,
                type_0: 1 as libc::c_int as libc::c_char,
                subtype: 2 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"stretchstaff\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.stretchstaff as *mut libc::c_int as *mut libc::c_void,
                type_0: 4 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"subtitlefont\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut *(cfmt.font_tb)
                    .as_mut_ptr()
                    .offset(SUBTITLEFONT as libc::c_int as isize) as *mut FONTSPEC
                    as *mut libc::c_void,
                type_0: 2 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"subtitlespace\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.subtitlespace as *mut libc::c_float as *mut libc::c_void,
                type_0: 3 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"sysstaffsep\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.sysstaffsep as *mut libc::c_float as *mut libc::c_void,
                type_0: 3 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"tempofont\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut *(cfmt.font_tb)
                    .as_mut_ptr()
                    .offset(TEMPOFONT as libc::c_int as isize) as *mut FONTSPEC
                    as *mut libc::c_void,
                type_0: 2 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"textfont\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut *(cfmt.font_tb)
                    .as_mut_ptr()
                    .offset(TEXTFONT as libc::c_int as isize) as *mut FONTSPEC
                    as *mut libc::c_void,
                type_0: 2 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"textoption\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.textoption as *mut libc::c_int as *mut libc::c_void,
                type_0: 0 as libc::c_int as libc::c_char,
                subtype: 4 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"textspace\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.textspace as *mut libc::c_float as *mut libc::c_void,
                type_0: 3 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"tieheight\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.tieheight as *mut libc::c_float as *mut libc::c_void,
                type_0: 1 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"titlecaps\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.titlecaps as *mut libc::c_int as *mut libc::c_void,
                type_0: 4 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"titlefont\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut *(cfmt.font_tb)
                    .as_mut_ptr()
                    .offset(TITLEFONT as libc::c_int as isize) as *mut FONTSPEC
                    as *mut libc::c_void,
                type_0: 2 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"titleformat\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.titleformat as *mut *mut libc::c_char as *mut libc::c_void,
                type_0: 5 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"titleleft\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.titleleft as *mut libc::c_int as *mut libc::c_void,
                type_0: 4 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"titlespace\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.titlespace as *mut libc::c_float as *mut libc::c_void,
                type_0: 3 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"titletrim\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.titletrim as *mut libc::c_int as *mut libc::c_void,
                type_0: 0 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"timewarn\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.timewarn as *mut libc::c_int as *mut libc::c_void,
                type_0: 4 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"topmargin\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.topmargin as *mut libc::c_float as *mut libc::c_void,
                type_0: 3 as libc::c_int as libc::c_char,
                subtype: 1 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"topspace\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.topspace as *mut libc::c_float as *mut libc::c_void,
                type_0: 3 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"tuplets\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.tuplets as *mut libc::c_int as *mut libc::c_void,
                type_0: 0 as libc::c_int as libc::c_char,
                subtype: 3 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"vocalfont\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut *(cfmt.font_tb)
                    .as_mut_ptr()
                    .offset(VOCALFONT as libc::c_int as isize) as *mut FONTSPEC
                    as *mut libc::c_void,
                type_0: 2 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"vocalspace\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.vocalspace as *mut libc::c_float as *mut libc::c_void,
                type_0: 3 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"voicefont\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut *(cfmt.font_tb)
                    .as_mut_ptr()
                    .offset(VOICEFONT as libc::c_int as isize) as *mut FONTSPEC
                    as *mut libc::c_void,
                type_0: 2 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"wordsfont\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut *(cfmt.font_tb)
                    .as_mut_ptr()
                    .offset(WORDSFONT as libc::c_int as isize) as *mut FONTSPEC
                    as *mut libc::c_void,
                type_0: 2 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"wordsspace\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.wordsspace as *mut libc::c_float as *mut libc::c_void,
                type_0: 3 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: b"writefields\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                v: &mut cfmt.fields as *mut [libc::c_uint; 2] as *mut libc::c_void,
                type_0: 4 as libc::c_int as libc::c_char,
                subtype: 1 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
        {
            let mut init = format {
                name: 0 as *mut libc::c_char,
                v: 0 as *mut libc::c_void,
                type_0: 0 as libc::c_int as libc::c_char,
                subtype: 0 as libc::c_int as libc::c_char,
                lock: 0,
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
