use ::libc;
use ::c2rust_bitfields;
extern "C" {
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    static mut _DefaultRuneLocale: _RuneLocale;
    static mut cfmt: FORMAT;
    static mut outbuf: *mut libc::c_char;
    static mut outbufsz: libc::c_int;
    static mut mbf: *mut libc::c_char;
    static mut outft: libc::c_int;
    static mut showerror: libc::c_int;
    static mut nstaff: libc::c_int;
    static mut voice_tb: [VOICE_S; 32];
    static mut first_voice: *mut VOICE_S;
    static mut tsfirst: *mut SYMBOL;
    static mut cursys: *mut SYSTEM;
    fn lvlarena(level: libc::c_int) -> libc::c_int;
    fn getarena(len: libc::c_int) -> *mut libc::c_void;
    fn a2b(fmt: *mut libc::c_char, _: ...);
    fn buffer_eob(eot: libc::c_int);
    fn bskip(h: libc::c_float);
    fn check_buffer();
    fn deco_width(s: *mut SYMBOL) -> libc::c_float;
    fn draw_all_deco();
    fn draw_sym_near();
    fn draw_all_symb();
    fn draw_systems(indent: libc::c_float) -> libc::c_float;
    fn output_ps(s: *mut SYMBOL, color: libc::c_int);
    fn putxy(x: libc::c_float, y: libc::c_float);
    fn cwid(c: libc::c_uchar) -> libc::c_float;
    fn sort_pitch(s: *mut SYMBOL);
    fn error(sev: libc::c_int, s: *mut SYMBOL, fmt: *mut libc::c_char, _: ...);
    fn tex_str(s: *mut libc::c_char) -> libc::c_float;
    fn str_font(ft: libc::c_int);
    fn sym_add(p_voice: *mut VOICE_S, type_0: libc::c_int) -> *mut SYMBOL;
    fn bug(msg: *mut libc::c_char, fatal: libc::c_int);
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct STAFF_S {
    pub s_clef: *mut SYMBOL,
    pub empty: libc::c_char,
    pub stafflines: *mut libc::c_char,
    pub staffscale: libc::c_float,
    pub botbar: libc::c_short,
    pub topbar: libc::c_short,
    pub y: libc::c_float,
    pub top: [libc::c_float; 128],
    pub bot: [libc::c_float; 128],
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
pub struct SYSTEM {
    pub next: *mut SYSTEM,
    pub top_voice: libc::c_short,
    pub nstaff: libc::c_short,
    pub staff: [C2RustUnnamed_11; 32],
    pub voice: [C2RustUnnamed_10; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub range: libc::c_schar,
    pub staff: libc::c_uchar,
    pub second: libc::c_char,
    pub dum: libc::c_char,
    pub sep: libc::c_float,
    pub maxsep: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub flags: libc::c_short,
    pub empty: libc::c_char,
    pub stafflines: *mut libc::c_char,
    pub staffscale: libc::c_float,
    pub sep: libc::c_float,
    pub maxsep: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub s: *mut SYMBOL,
    pub staff: libc::c_int,
    pub end_time: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub voice: libc::c_int,
    pub ymn: libc::c_short,
    pub ymx: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub nvoice: libc::c_int,
    pub st: [C2RustUnnamed_13; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub st1: libc::c_schar,
    pub st2: libc::c_schar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub clef: *mut SYMBOL,
    pub autoclef: libc::c_short,
    pub mid: libc::c_short,
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
pub static mut staff_tb: [STAFF_S; 32] = [STAFF_S {
    s_clef: 0 as *const SYMBOL as *mut SYMBOL,
    empty: 0,
    stafflines: 0 as *const libc::c_char as *mut libc::c_char,
    staffscale: 0.,
    botbar: 0,
    topbar: 0,
    y: 0.,
    top: [0.; 128],
    bot: [0.; 128],
}; 32];
#[no_mangle]
pub static mut tsnext: *mut SYMBOL = 0 as *const SYMBOL as *mut SYMBOL;
#[no_mangle]
pub static mut realwidth: libc::c_float = 0.;
static mut insert_meter: libc::c_int = 0;
static mut beta_last: libc::c_float = 0.;
#[no_mangle]
pub static mut space_tb: [libc::c_float; 10] = [
    7 as libc::c_int as libc::c_float,
    10 as libc::c_int as libc::c_float,
    14.15f64 as libc::c_float,
    20 as libc::c_int as libc::c_float,
    28.3f64 as libc::c_float,
    40 as libc::c_int as libc::c_float,
    56.6f64 as libc::c_float,
    80 as libc::c_int as libc::c_float,
    113 as libc::c_int as libc::c_float,
    150 as libc::c_int as libc::c_float,
];
#[no_mangle]
pub static mut hw_tb: [libc::c_float; 4] = [
    4.5f64 as libc::c_float,
    5 as libc::c_int as libc::c_float,
    6 as libc::c_int as libc::c_float,
    8 as libc::c_int as libc::c_float,
];
static mut smallest_duration: libc::c_int = 0;
static mut rest_sp: [[libc::c_char; 2]; 10] = [
    [18 as libc::c_int as libc::c_char, 18 as libc::c_int as libc::c_char],
    [12 as libc::c_int as libc::c_char, 18 as libc::c_int as libc::c_char],
    [12 as libc::c_int as libc::c_char, 12 as libc::c_int as libc::c_char],
    [8 as libc::c_int as libc::c_char, 12 as libc::c_int as libc::c_char],
    [6 as libc::c_int as libc::c_char, 8 as libc::c_int as libc::c_char],
    [10 as libc::c_int as libc::c_char, 10 as libc::c_int as libc::c_char],
    [6 as libc::c_int as libc::c_char, 4 as libc::c_int as libc::c_char],
    [10 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char],
    [10 as libc::c_int as libc::c_char, 4 as libc::c_int as libc::c_char],
    [10 as libc::c_int as libc::c_char, 10 as libc::c_int as libc::c_char],
];
unsafe extern "C" fn set_heads(mut s: *mut SYMBOL) -> libc::c_float {
    let mut note: *mut note = 0 as *mut note;
    let mut i: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut w: libc::c_float = 0.;
    let mut wmax: libc::c_float = 0.;
    static mut dx_tb: [libc::c_float; 4] = [
        7 as libc::c_int as libc::c_float,
        8 as libc::c_int as libc::c_float,
        10 as libc::c_int as libc::c_float,
        13.3f64 as libc::c_float,
    ];
    n = (*s).nhd as libc::c_int;
    wmax = -(1 as libc::c_int) as libc::c_float;
    m = 0 as libc::c_int;
    while m <= n {
        note = &mut *((*s).u.note.notes).as_mut_ptr().offset(m as isize) as *mut note;
        p = (*note).head;
        if !p.is_null() {
            i = (*s).head as libc::c_int;
            loop {
                q = strchr(p, ',' as i32);
                if q.is_null() {
                    break;
                }
                i -= 1;
                if i < 0 as libc::c_int {
                    break;
                }
                p = q.offset(1 as libc::c_int as isize);
            }
            if q.is_null() {
                q = p.offset(strlen(p) as isize);
            }
            r = strchr(p, '/' as i32);
            if !r.is_null() && r < q {
                if (*s).stem as libc::c_int >= 0 as libc::c_int {
                    q = r;
                } else {
                    p = r.offset(1 as libc::c_int as isize);
                }
            }
            r = strchr(p, ':' as i32);
            if !r.is_null() && r < q {
                q = r;
                sscanf(
                    r,
                    b":%f\0" as *const u8 as *const libc::c_char,
                    &mut w as *mut libc::c_float,
                );
                if w > wmax {
                    wmax = w;
                }
            }
            (*note).head = p;
            (*note).hlen = q.offset_from(p) as libc::c_long as libc::c_char;
        }
        m += 1;
        m;
    }
    if wmax < 0 as libc::c_int as libc::c_float {
        wmax = dx_tb[(*s).head as usize];
        if (*s).dur >= 1536 as libc::c_int * 2 as libc::c_int
            && (*s).head as libc::c_int == 2 as libc::c_int
        {
            wmax = 13.8f64 as libc::c_float;
        }
    }
    (*s).u.note.sdx = wmax / 2 as libc::c_int as libc::c_float;
    return wmax;
}
unsafe extern "C" fn set_head_shift(mut s: *mut SYMBOL) {
    let mut i: libc::c_int = 0;
    let mut i1: libc::c_int = 0;
    let mut i2: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut sig: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut shift: libc::c_int = 0;
    let mut ps: libc::c_int = 0;
    let mut dx: libc::c_float = 0.;
    let mut dx_max: libc::c_float = 0.;
    let mut dx_head: libc::c_float = 0.;
    dx_head = set_heads(s) + 2 as libc::c_int as libc::c_float;
    n = (*s).nhd as libc::c_int;
    if n == 0 as libc::c_int {
        return;
    }
    dx = (dx_head as libc::c_double * 0.78f64) as libc::c_float;
    if (*s).flags as libc::c_int & 0x20 as libc::c_int != 0 {
        dx = (dx as libc::c_double * 0.5f64) as libc::c_float;
    }
    sig = (*s).stem as libc::c_int;
    if sig >= 0 as libc::c_int {
        i1 = 1 as libc::c_int;
        i2 = n + 1 as libc::c_int;
        ps = (*s).pits[0 as libc::c_int as usize] as libc::c_int;
    } else {
        dx = -dx;
        i1 = n - 1 as libc::c_int;
        i2 = -(1 as libc::c_int);
        ps = (*s).pits[n as usize] as libc::c_int;
    }
    shift = 0 as libc::c_int;
    dx_max = 0 as libc::c_int as libc::c_float;
    let mut current_block_39: u64;
    i = i1;
    while i != i2 {
        d = (*s).pits[i as usize] as libc::c_int - ps;
        ps = (*s).pits[i as usize] as libc::c_int;
        if d == 0 as libc::c_int {
            if shift != 0 {
                (*s)
                    .u
                    .note
                    .notes[i as usize]
                    .shhd = (*s).u.note.notes[(i - sig) as usize].shhd + dx;
                let mut new_dx: libc::c_float = (*s).u.note.notes[i as usize].shhd;
                if dx_max < new_dx {
                    dx_max = new_dx;
                }
                current_block_39 = 17860125682698302841;
            } else if i + sig != i2
                && ps + sig == (*s).pits[(i + sig) as usize] as libc::c_int
            {
                (*s).u.note.notes[i as usize].shhd = -dx;
                if dx_max < -dx {
                    dx_max = -dx;
                }
                current_block_39 = 17860125682698302841;
            } else {
                current_block_39 = 7172762164747879670;
            }
        } else {
            current_block_39 = 7172762164747879670;
        }
        match current_block_39 {
            7172762164747879670 => {
                if d < 0 as libc::c_int {
                    d = -d;
                }
                if d > 3 as libc::c_int
                    || d >= 2 as libc::c_int
                        && ((*s).head as libc::c_int) < 3 as libc::c_int
                {
                    shift = 0 as libc::c_int;
                } else {
                    shift = (shift == 0) as libc::c_int;
                    if shift != 0 {
                        (*s).u.note.notes[i as usize].shhd = dx;
                        if dx_max < dx {
                            dx_max = dx;
                        }
                    }
                }
            }
            _ => {}
        }
        i += sig;
    }
    (*s).xmx = dx_max;
}
unsafe extern "C" fn acc_shift(
    mut notes: *mut *mut note,
    mut n: libc::c_int,
    mut dx_head: libc::c_float,
) {
    let mut i: libc::c_int = 0;
    let mut i1: libc::c_int = 0;
    let mut ps: libc::c_int = 0;
    let mut p1: libc::c_int = 0;
    let mut acc: libc::c_int = 0;
    let mut dx: libc::c_float = 0.;
    let mut dx1: libc::c_float = 0.;
    i = n - 1 as libc::c_int;
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int) {
            break;
        }
        dx = (**notes.offset(i as isize)).shhd;
        if dx == 0 as libc::c_int as libc::c_float
            || dx > 0 as libc::c_int as libc::c_float
        {
            continue;
        }
        dx = dx_head - dx;
        ps = (**notes.offset(i as isize)).pit as libc::c_int;
        i1 = n;
        loop {
            i1 -= 1;
            if !(i1 >= 0 as libc::c_int) {
                break;
            }
            if (**notes.offset(i1 as isize)).acc == 0 {
                continue;
            }
            p1 = (**notes.offset(i1 as isize)).pit as libc::c_int;
            if p1 < ps - 3 as libc::c_int {
                break;
            }
            if p1 > ps + 3 as libc::c_int {
                continue;
            }
            if (**notes.offset(i1 as isize)).shac < dx {
                (**notes.offset(i1 as isize)).shac = dx;
            }
        }
    }
    i = n;
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int) {
            break;
        }
        acc = (**notes.offset(i as isize)).acc as libc::c_int;
        if acc == 0 {
            continue;
        }
        dx = (**notes.offset(i as isize)).shac;
        if dx == 0 as libc::c_int as libc::c_float {
            dx = (**notes.offset(i as isize)).shhd;
            if dx < 0 as libc::c_int as libc::c_float {
                dx = dx_head - dx;
            } else {
                dx = dx_head;
            }
        }
        ps = (**notes.offset(i as isize)).pit as libc::c_int;
        i1 = n;
        loop {
            i1 -= 1;
            if !(i1 > i) {
                break;
            }
            if (**notes.offset(i1 as isize)).acc == 0 {
                continue;
            }
            p1 = (**notes.offset(i1 as isize)).pit as libc::c_int;
            if p1 >= ps + 4 as libc::c_int {
                if p1 > ps + 4 as libc::c_int {
                    continue;
                }
                match acc {
                    0 | 3 | 5 => {
                        continue;
                    }
                    _ => {}
                }
                match (**notes.offset(i1 as isize)).acc as libc::c_int {
                    0 | 3 | 5 => {
                        continue;
                    }
                    _ => {}
                }
            }
            if dx
                > (**notes.offset(i1 as isize)).shac - 6 as libc::c_int as libc::c_float
            {
                dx1 = (**notes.offset(i1 as isize)).shac
                    + 7 as libc::c_int as libc::c_float;
                if dx1 > dx {
                    dx = dx1;
                }
            }
        }
        (**notes.offset(i as isize)).shac = dx;
    };
}
unsafe extern "C" fn set_acc_shft() {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut i: libc::c_int = 0;
    let mut staff: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut acc: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut nx: libc::c_int = 0;
    let mut dx_head: libc::c_float = 0.;
    let mut notes: [*mut note; 32] = [0 as *mut note; 32];
    let mut nt: *mut note = 0 as *mut note;
    s = tsfirst;
    while !s.is_null() {
        if (*s).abc_type as libc::c_int != 4 as libc::c_int
            || (*s).flags as libc::c_int & 0x2 as libc::c_int != 0
        {
            s = (*s).ts_next;
        } else {
            staff = (*s).staff as libc::c_int;
            t = (*s).time;
            acc = 0 as libc::c_int;
            s2 = s;
            while !s2.is_null() {
                if (*s2).time != t || (*s2).abc_type as libc::c_int != 4 as libc::c_int
                    || (*s2).staff as libc::c_int != staff
                {
                    break;
                }
                if !(acc != 0) {
                    i = 0 as libc::c_int;
                    while i <= (*s2).nhd as libc::c_int {
                        if (*s2).u.note.notes[i as usize].acc != 0 {
                            acc = 1 as libc::c_int;
                        }
                        i += 1;
                        i;
                    }
                }
                s2 = (*s2).ts_next;
            }
            if acc == 0 {
                s = s2;
            } else {
                dx_head = set_heads(s) + 2 as libc::c_int as libc::c_float;
                n = 0 as libc::c_int;
                while s != s2 {
                    i = 0 as libc::c_int;
                    while i <= (*s).nhd as libc::c_int {
                        let fresh0 = n;
                        n = n + 1;
                        notes[fresh0
                            as usize] = &mut *((*s).u.note.notes)
                            .as_mut_ptr()
                            .offset(i as isize) as *mut note;
                        i += 1;
                        i;
                    }
                    s = (*s).ts_next;
                }
                loop {
                    nx = 0 as libc::c_int;
                    i = 1 as libc::c_int;
                    while i < n {
                        if !((*notes[i as usize]).pit as libc::c_int
                            >= (*notes[(i - 1 as libc::c_int) as usize]).pit
                                as libc::c_int)
                        {
                            nt = notes[i as usize];
                            notes[i as usize] = notes[(i - 1 as libc::c_int) as usize];
                            notes[(i - 1 as libc::c_int) as usize] = nt;
                            nx += 1;
                            nx;
                        }
                        i += 1;
                        i;
                    }
                    if nx == 0 as libc::c_int {
                        break;
                    }
                }
                acc_shift(notes.as_mut_ptr(), n, dx_head);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn unlksym(mut s: *mut SYMBOL) {
    if !((*s).next).is_null() {
        (*(*s).next).prev = (*s).prev;
    }
    if !((*s).prev).is_null() {
        (*(*s).prev).next = (*s).next;
    } else {
        voice_tb[(*s).voice as usize].sym = (*s).next;
    }
    if !((*s).ts_next).is_null() {
        if !((*s).extra).is_null() {
            let mut g: *mut SYMBOL = 0 as *mut SYMBOL;
            g = (*(*s).ts_next).extra;
            if g.is_null() {
                (*(*s).ts_next).extra = (*s).extra;
            } else {
                while !((*g).next).is_null() {
                    g = (*g).next;
                }
                (*g).next = (*s).extra;
            }
        }
        if (*s).sflags & 0x80000 as libc::c_int as libc::c_uint != 0
            && (*(*s).ts_next).sflags & 0x80000 as libc::c_int as libc::c_uint == 0
        {
            (*(*s).ts_next).sflags |= 0x80000 as libc::c_int as libc::c_uint;
            (*(*s).ts_next).shrink = (*s).shrink;
            (*(*s).ts_next).space = (*s).space;
        }
        if (*s).sflags & 0x8000000 as libc::c_int as libc::c_uint != 0 {
            (*(*s).ts_next).sflags |= 0x8000000 as libc::c_int as libc::c_uint;
        }
        (*(*s).ts_next).ts_prev = (*s).ts_prev;
    }
    if !((*s).ts_prev).is_null() {
        (*(*s).ts_prev).ts_next = (*s).ts_next;
    }
    if tsfirst == s {
        tsfirst = (*s).ts_next;
    }
    if tsnext == s {
        tsnext = (*s).ts_next;
    }
}
unsafe extern "C" fn may_combine(mut s: *mut SYMBOL) -> libc::c_int {
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut nhd2: libc::c_int = 0;
    if (*s).combine as libc::c_int == 0 as libc::c_int
        && (*s).abc_type as libc::c_int != 5 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    s2 = (*s).ts_next;
    if s2.is_null() || (*s2).type_0 as libc::c_int != 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*s2).voice as libc::c_int == (*s).voice as libc::c_int
        || (*s2).staff as libc::c_int != (*s).staff as libc::c_int
        || (*s2).time != (*s).time || (*s2).dur != (*s).dur
    {
        return 0 as libc::c_int;
    }
    if (*s).combine as libc::c_int <= 0 as libc::c_int
        && (*s2).abc_type as libc::c_int != (*s).abc_type as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if (*s).u.note.dc.n as libc::c_int + (*s2).u.note.dc.n as libc::c_int
        >= 32 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if !((*s).gch).is_null() && !((*s2).gch).is_null() {
        return 0 as libc::c_int;
    }
    if (*s).abc_type as libc::c_int == 5 as libc::c_int {
        if (*s2).abc_type as libc::c_int == 5 as libc::c_int
            && (*s).flags as libc::c_int & 0x2 as libc::c_int != 0
            && (*s2).flags as libc::c_int & 0x2 as libc::c_int == 0
        {
            return 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    }
    if !((*s2).ly).is_null()
        || (*s2).sflags & (0x800 as libc::c_int | 0x1000 as libc::c_int) as libc::c_uint
            != 0 || (*s2).u.note.slur_st as libc::c_int != 0 as libc::c_int
        || (*s2).u.note.slur_end as libc::c_int != 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if ((*s2).sflags ^ (*s).sflags)
        & (0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint != 0
    {
        return 0 as libc::c_int;
    }
    nhd2 = (*s2).nhd as libc::c_int;
    if (*s).nhd as libc::c_int + nhd2 + 1 as libc::c_int >= 8 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*s).combine as libc::c_int <= 1 as libc::c_int
        && (*s).pits[0 as libc::c_int as usize] as libc::c_int
            <= (*s2).pits[nhd2 as usize] as libc::c_int + 1 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn do_combine(mut s: *mut SYMBOL) {
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut i: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut nhd: libc::c_int = 0;
    let mut nhd2: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    loop {
        nhd = (*s).nhd as libc::c_int;
        s2 = (*s).ts_next;
        (*s2).extra = 0 as *mut SYMBOL;
        if (*s).abc_type as libc::c_int != (*s2).abc_type as libc::c_int {
            if (*s2).abc_type as libc::c_int != 5 as libc::c_int {
                s2 = s;
                s = (*s2).ts_next;
            }
        } else if (*s).abc_type as libc::c_int == 5 as libc::c_int {
            if (*s).flags as libc::c_int & 0x2 as libc::c_int != 0
                && (*s2).flags as libc::c_int & 0x2 as libc::c_int == 0
            {
                (*s)
                    .flags = ((*s).flags as libc::c_int & !(0x2 as libc::c_int))
                    as libc::c_ushort;
            }
        } else {
            nhd2 = (*s2).nhd as libc::c_int + 1 as libc::c_int;
            memcpy(
                &mut *((*s).u.note.notes)
                    .as_mut_ptr()
                    .offset((nhd + 1 as libc::c_int) as isize) as *mut note
                    as *mut libc::c_void,
                ((*s2).u.note.notes).as_mut_ptr() as *const libc::c_void,
                (::core::mem::size_of::<note>() as libc::c_ulong)
                    .wrapping_mul(nhd2 as libc::c_ulong),
            );
            memcpy(
                &mut *((*s).pits).as_mut_ptr().offset((nhd + 1 as libc::c_int) as isize)
                    as *mut libc::c_schar as *mut libc::c_void,
                ((*s2).pits).as_mut_ptr() as *const libc::c_void,
                (::core::mem::size_of::<libc::c_schar>() as libc::c_ulong)
                    .wrapping_mul((nhd2 + 1 as libc::c_int) as libc::c_ulong),
            );
            (*s).sflags
                |= (*s2).sflags
                    & (0x800 as libc::c_int | 0x1000 as libc::c_int
                        | 0x2000 as libc::c_int) as libc::c_uint;
            nhd += nhd2;
            (*s).nhd = nhd as libc::c_uchar;
            sort_pitch(s);
            if (*s).combine as libc::c_int >= 3 as libc::c_int {
                m = nhd;
                while m > 0 as libc::c_int {
                    if (*s).u.note.notes[m as usize].pit as libc::c_int
                        == (*s).u.note.notes[(m - 1 as libc::c_int) as usize].pit
                            as libc::c_int
                        && (*s).u.note.notes[m as usize].acc as libc::c_int
                            == (*s).u.note.notes[(m - 1 as libc::c_int) as usize].acc
                                as libc::c_int
                    {
                        i = nhd - m;
                        if i > 0 as libc::c_int {
                            memmove(
                                &mut *((*s).u.note.notes).as_mut_ptr().offset(m as isize)
                                    as *mut note as *mut libc::c_void,
                                &mut *((*s).u.note.notes)
                                    .as_mut_ptr()
                                    .offset((m + 1 as libc::c_int) as isize) as *mut note
                                    as *const libc::c_void,
                                (::core::mem::size_of::<note>() as libc::c_ulong)
                                    .wrapping_mul(i as libc::c_ulong),
                            );
                            memmove(
                                &mut *((*s).pits).as_mut_ptr().offset(m as isize)
                                    as *mut libc::c_schar as *mut libc::c_void,
                                &mut *((*s).pits)
                                    .as_mut_ptr()
                                    .offset((m + 1 as libc::c_int) as isize)
                                    as *mut libc::c_schar as *const libc::c_void,
                                (::core::mem::size_of::<libc::c_schar>() as libc::c_ulong)
                                    .wrapping_mul(i as libc::c_ulong),
                            );
                        }
                        nhd -= 1;
                        (*s).nhd = nhd as libc::c_uchar;
                    }
                    m -= 1;
                    m;
                }
            }
            (*s)
                .ymx = (3 as libc::c_int
                * ((*s).pits[nhd as usize] as libc::c_int - 18 as libc::c_int)
                + 4 as libc::c_int) as libc::c_schar;
            (*s)
                .ymn = (3 as libc::c_int
                * ((*s).pits[0 as libc::c_int as usize] as libc::c_int
                    - 18 as libc::c_int) - 4 as libc::c_int) as libc::c_schar;
            type_0 = (*s).u.note.notes[0 as libc::c_int as usize].ti1 as libc::c_int;
            if type_0 & 0xf as libc::c_int == 0x4 as libc::c_int {
                (*s)
                    .u
                    .note
                    .notes[0 as libc::c_int as usize]
                    .ti1 = (0x2 as libc::c_int | type_0 & !(0x8 as libc::c_int))
                    as libc::c_char;
            }
            type_0 = (*s).u.note.notes[nhd as usize].ti1 as libc::c_int;
            if type_0 & 0xf as libc::c_int == 0x4 as libc::c_int {
                (*s)
                    .u
                    .note
                    .notes[nhd as usize]
                    .ti1 = (0x1 as libc::c_int | type_0 & !(0x8 as libc::c_int))
                    as libc::c_char;
            }
        }
        if !((*s2).text).is_null() && ((*s).text).is_null() {
            (*s).text = (*s2).text;
            (*s).gch = (*s2).gch;
        }
        if (*s2).u.note.dc.n as libc::c_int > 0 as libc::c_int {
            let mut n: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < (*s2).u.note.dc.n as libc::c_int {
                if (*s2).u.note.dc.tm[i as usize].m as libc::c_int >= 0 as libc::c_int {
                    (*s2)
                        .u
                        .note
                        .dc
                        .tm[i as usize]
                        .m = ((*s2).u.note.dc.tm[i as usize].m as libc::c_int
                        + (nhd + 1 as libc::c_int)) as libc::c_schar;
                }
                i += 1;
                i;
            }
            n = (*s).u.note.dc.n as libc::c_int;
            memcpy(
                &mut *((*s).u.note.dc.tm).as_mut_ptr().offset(n as isize)
                    as *mut C2RustUnnamed as *mut libc::c_void,
                ((*s2).u.note.dc.tm).as_mut_ptr() as *const libc::c_void,
                (::core::mem::size_of::<C2RustUnnamed>() as libc::c_ulong)
                    .wrapping_mul((*s2).u.note.dc.n as libc::c_ulong),
            );
            (*s)
                .u
                .note
                .dc
                .n = ((*s).u.note.dc.n as libc::c_int + (*s2).u.note.dc.n as libc::c_int)
                as libc::c_char;
        }
        unlksym(s2);
        if !((*s).sflags & 0x40 as libc::c_int as libc::c_uint == 0
            && may_combine(s) != 0)
        {
            break;
        }
    };
}
unsafe extern "C" fn combine_voices() {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut g: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    s = tsfirst;
    while !((*s).ts_next).is_null() {
        if !(((*s).combine as libc::c_int) < 0 as libc::c_int) {
            if (*s).sflags & 0x40 as libc::c_int as libc::c_uint != 0 {
                g = (*s).extra;
                if !g.is_null() {
                    r = 0 as libc::c_int;
                    while !g.is_null() {
                        if (*g).type_0 as libc::c_int == 13 as libc::c_int
                            && (*g).u.tuplet.r_plet as libc::c_int > r
                        {
                            r = (*g).u.tuplet.r_plet as libc::c_int;
                        }
                        g = (*g).next;
                    }
                    if !(r == 0 as libc::c_int) {
                        i = r;
                        s2 = s;
                        while !s2.is_null() {
                            if ((*s2).ts_next).is_null() {
                                break;
                            }
                            if !((*s2).type_0 as libc::c_int != 1 as libc::c_int) {
                                if may_combine(s2) == 0 {
                                    break;
                                }
                                i -= 1;
                                if i <= 0 as libc::c_int {
                                    break;
                                }
                            }
                            s2 = (*s2).next;
                        }
                        if !(i > 0 as libc::c_int) {
                            s2 = s;
                            loop {
                                if !((*s2).type_0 as libc::c_int != 1 as libc::c_int) {
                                    do_combine(s2);
                                    r -= 1;
                                    if r <= 0 as libc::c_int {
                                        break;
                                    }
                                }
                                s2 = (*s2).next;
                            }
                        }
                    }
                }
            } else if !((*s).type_0 as libc::c_int != 1 as libc::c_int) {
                if (*s).abc_type as libc::c_int == 4 as libc::c_int {
                    if !((*s).sflags & 0x2 as libc::c_int as libc::c_uint == 0) {
                        if (*s).sflags & 0x10 as libc::c_int as libc::c_uint != 0 {
                            if may_combine(s) != 0 {
                                do_combine(s);
                            }
                        } else {
                            s2 = s;
                            loop {
                                if may_combine(s2) == 0 {
                                    s2 = 0 as *mut SYMBOL;
                                    break;
                                } else {
                                    if (*s2).sflags & 0x10 as libc::c_int as libc::c_uint != 0 {
                                        break;
                                    }
                                    loop {
                                        s2 = (*s2).next;
                                        if !((*s2).type_0 as libc::c_int != 1 as libc::c_int) {
                                            break;
                                        }
                                    }
                                }
                            }
                            if !s2.is_null() {
                                s2 = s;
                                loop {
                                    do_combine(s2);
                                    if (*s2).sflags & 0x10 as libc::c_int as libc::c_uint != 0 {
                                        break;
                                    }
                                    loop {
                                        s2 = (*s2).next;
                                        if !((*s2).type_0 as libc::c_int != 1 as libc::c_int) {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else if may_combine(s) != 0 {
                    do_combine(s);
                }
            }
        }
        s = (*s).ts_next;
    }
}
unsafe extern "C" fn insert_clef(
    mut s: *mut SYMBOL,
    mut clef_type: libc::c_int,
    mut clef_line: libc::c_int,
) -> *mut SYMBOL {
    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut new_s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut staff: libc::c_int = 0;
    staff = (*s).staff as libc::c_int;
    if (*s).type_0 as libc::c_int == 3 as libc::c_int && !((*s).prev).is_null()
        && (*(*s).prev).type_0 as libc::c_int == 3 as libc::c_int
    {
        s = (*s).prev;
    }
    p_voice = &mut *voice_tb.as_mut_ptr().offset((*s).voice as isize) as *mut VOICE_S;
    (*p_voice).last_sym = (*s).prev;
    if ((*p_voice).last_sym).is_null() {
        (*p_voice).sym = 0 as *mut SYMBOL;
    }
    (*p_voice).time = (*s).time;
    new_s = sym_add(p_voice, 4 as libc::c_int);
    (*new_s).next = s;
    (*s).prev = new_s;
    (*new_s).u.clef.type_0 = clef_type as libc::c_schar;
    (*new_s).u.clef.line = clef_line as libc::c_char;
    (*new_s).staff = staff as libc::c_uchar;
    (*new_s).aux = 1 as libc::c_int as libc::c_short;
    (*new_s).sflags &= !(0x100000 as libc::c_int) as libc::c_uint;
    while (*s).sflags & 0x80000 as libc::c_int as libc::c_uint == 0 {
        s = (*s).ts_prev;
    }
    if (*(*s).ts_prev).type_0 as libc::c_int != 4 as libc::c_int {
        (*new_s).sflags |= 0x80000 as libc::c_int as libc::c_uint;
    }
    (*new_s).ts_prev = (*s).ts_prev;
    (*(*new_s).ts_prev).ts_next = new_s;
    (*new_s).ts_next = s;
    (*s).ts_prev = new_s;
    return new_s;
}
unsafe extern "C" fn set_float() {
    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut staff: libc::c_int = 0;
    let mut staff_chg: libc::c_int = 0;
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s1: *mut SYMBOL = 0 as *mut SYMBOL;
    p_voice = first_voice;
    while !p_voice.is_null() {
        if !((*p_voice).floating() == 0) {
            staff_chg = 0 as libc::c_int;
            staff = (*p_voice).staff as libc::c_int;
            let mut current_block_33: u64;
            s = (*p_voice).sym;
            while !s.is_null() {
                let mut up: libc::c_schar = 0;
                let mut down: libc::c_schar = 0;
                if (*s).dur == 0 {
                    if staff_chg != 0 {
                        (*s).staff = ((*s).staff).wrapping_add(1);
                        (*s).staff;
                    }
                } else if (*s).sflags & 0x200000 as libc::c_int as libc::c_uint == 0 {
                    staff_chg = 0 as libc::c_int;
                } else if (*s).pits[0 as libc::c_int as usize] as libc::c_int
                    >= 19 as libc::c_int
                {
                    staff_chg = 0 as libc::c_int;
                } else if (*s).pits[(*s).nhd as usize] as libc::c_int
                    <= 12 as libc::c_int
                {
                    staff_chg = 1 as libc::c_int;
                    (*s).staff = ((*s).staff).wrapping_add(1);
                    (*s).staff;
                } else {
                    up = 127 as libc::c_int as libc::c_schar;
                    s1 = (*s).ts_prev;
                    while !s1.is_null() {
                        if (*s1).staff as libc::c_int != staff
                            || (*s1).voice as libc::c_int == (*s).voice as libc::c_int
                        {
                            break;
                        }
                        if (*s1).abc_type as libc::c_int == 4 as libc::c_int {
                            if ((*s1).pits[0 as libc::c_int as usize] as libc::c_int)
                                < up as libc::c_int
                            {
                                up = (*s1).pits[0 as libc::c_int as usize];
                            }
                        }
                        s1 = (*s1).ts_prev;
                    }
                    if up as libc::c_int == 127 as libc::c_int {
                        if staff_chg != 0 {
                            (*s).staff = ((*s).staff).wrapping_add(1);
                            (*s).staff;
                        }
                    } else if (*s).pits[(*s).nhd as usize] as libc::c_int
                        > up as libc::c_int - 3 as libc::c_int
                    {
                        staff_chg = 0 as libc::c_int;
                    } else {
                        down = -(127 as libc::c_int) as libc::c_schar;
                        s1 = (*s).ts_next;
                        while !s1.is_null() {
                            if (*s1).staff as libc::c_int != staff + 1 as libc::c_int
                                || (*s1).voice as libc::c_int == (*s).voice as libc::c_int
                            {
                                break;
                            }
                            if (*s1).abc_type as libc::c_int == 4 as libc::c_int {
                                if (*s1).pits[(*s1).nhd as usize] as libc::c_int
                                    > down as libc::c_int
                                {
                                    down = (*s1).pits[(*s1).nhd as usize];
                                }
                            }
                            s1 = (*s1).ts_next;
                        }
                        if down as libc::c_int == -(127 as libc::c_int) {
                            if staff_chg != 0 {
                                (*s).staff = ((*s).staff).wrapping_add(1);
                                (*s).staff;
                            }
                        } else if ((*s).pits[0 as libc::c_int as usize] as libc::c_int)
                            < down as libc::c_int + 3 as libc::c_int
                        {
                            staff_chg = 1 as libc::c_int;
                            (*s).staff = ((*s).staff).wrapping_add(1);
                            (*s).staff;
                        } else {
                            up = (up as libc::c_int
                                - (*s).pits[(*s).nhd as usize] as libc::c_int)
                                as libc::c_schar;
                            down = ((*s).pits[0 as libc::c_int as usize] as libc::c_int
                                - down as libc::c_int) as libc::c_schar;
                            if staff_chg == 0 {
                                if (up as libc::c_int)
                                    < down as libc::c_int + 3 as libc::c_int
                                {
                                    current_block_33 = 6873731126896040597;
                                } else {
                                    staff_chg = 1 as libc::c_int;
                                    current_block_33 = 2989495919056355252;
                                }
                            } else if (up as libc::c_int)
                                < down as libc::c_int - 3 as libc::c_int
                            {
                                staff_chg = 0 as libc::c_int;
                                current_block_33 = 6873731126896040597;
                            } else {
                                current_block_33 = 2989495919056355252;
                            }
                            match current_block_33 {
                                6873731126896040597 => {}
                                _ => {
                                    (*s).staff = ((*s).staff).wrapping_add(1);
                                    (*s).staff;
                                }
                            }
                        }
                    }
                }
                s = (*s).next;
            }
        }
        p_voice = (*p_voice).next;
    }
}
unsafe extern "C" fn set_graceoffs(mut s: *mut SYMBOL) -> libc::c_float {
    let mut g: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut next: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut m: libc::c_int = 0;
    let mut xx: libc::c_float = 0.;
    let mut dx: libc::c_float = 0.;
    let mut gspleft: libc::c_float = 0.;
    let mut gspinside: libc::c_float = 0.;
    let mut gspright: libc::c_float = 0.;
    let mut notes: [*mut note; 8] = [0 as *mut note; 8];
    gspleft = ((cfmt.gracespace >> 16 as libc::c_int) as libc::c_double * 0.1f64)
        as libc::c_float;
    gspinside = ((cfmt.gracespace >> 8 as libc::c_int & 0xff as libc::c_int)
        as libc::c_double * 0.1f64) as libc::c_float;
    gspright = ((cfmt.gracespace & 0xff as libc::c_int) as libc::c_double * 0.1f64)
        as libc::c_float;
    xx = 0 as libc::c_int as libc::c_float;
    g = (*s).extra;
    while !((*g).type_0 as libc::c_int == 1 as libc::c_int) {
        g = (*g).next;
    }
    (*g).sflags |= 0x2 as libc::c_int as libc::c_uint;
    loop {
        if (*g).type_0 as libc::c_int != 1 as libc::c_int {
            if ((*g).next).is_null() {
                break;
            }
        } else {
            set_head_shift(g);
            m = 0 as libc::c_int;
            while m <= (*g).nhd as libc::c_int {
                notes[m
                    as usize] = &mut *((*g).u.note.notes).as_mut_ptr().offset(m as isize)
                    as *mut note;
                m += 1;
                m;
            }
            acc_shift(
                notes.as_mut_ptr(),
                (*g).nhd as libc::c_int + 1 as libc::c_int,
                7 as libc::c_int as libc::c_float,
            );
            dx = 0 as libc::c_int as libc::c_float;
            m = (*g).nhd as libc::c_int;
            while m >= 0 as libc::c_int {
                if (*g).u.note.notes[m as usize].shac > dx {
                    dx = (*g).u.note.notes[m as usize].shac;
                }
                m -= 1;
                m;
            }
            xx += dx;
            (*g).x = xx;
            if (*g).nflags as libc::c_int <= 0 as libc::c_int {
                (*g).sflags
                    |= (0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint;
            }
            next = (*g).next;
            if next.is_null() {
                (*g).sflags |= 0x10 as libc::c_int as libc::c_uint;
                break;
            } else {
                if (*next).nflags as libc::c_int <= 0 as libc::c_int
                    || (*next).flags as libc::c_int & 0x4 as libc::c_int != 0
                {
                    (*g).sflags |= 0x10 as libc::c_int as libc::c_uint;
                }
                if (*g).sflags & 0x10 as libc::c_int as libc::c_uint != 0 {
                    (*next).sflags |= 0x2 as libc::c_int as libc::c_uint;
                    xx += gspinside / 4 as libc::c_int as libc::c_float;
                }
                if (*g).nflags as libc::c_int <= 0 as libc::c_int {
                    xx += gspinside / 4 as libc::c_int as libc::c_float;
                }
                if (*g).y as libc::c_int > (*next).y as libc::c_int + 8 as libc::c_int {
                    xx = (xx as libc::c_double - 1.5f64) as libc::c_float;
                }
                xx += gspinside;
            }
        }
        g = (*g).next;
    }
    xx += gspleft + gspright;
    next = (*s).next;
    if !next.is_null() && (*next).abc_type as libc::c_int == 4 as libc::c_int {
        if (*g).y as libc::c_int
            >= 3 as libc::c_int
                * ((*next).pits[(*next).nhd as usize] as libc::c_int - 18 as libc::c_int)
        {
            xx -= 1 as libc::c_int as libc::c_float;
        } else if (*g).sflags & 0x2 as libc::c_int as libc::c_uint != 0
            && ((*g).y as libc::c_int)
                < 3 as libc::c_int
                    * ((*next).pits[0 as libc::c_int as usize] as libc::c_int
                        - 18 as libc::c_int) - 7 as libc::c_int
        {
            xx += 2 as libc::c_int as libc::c_float;
        }
    }
    return xx;
}
unsafe extern "C" fn gchord_width(
    mut s: *mut SYMBOL,
    mut wlnote: libc::c_float,
    mut wlw: libc::c_float,
) -> libc::c_float {
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut gch: *mut gch = 0 as *mut gch;
    let mut ix: libc::c_int = 0;
    let mut lspc: libc::c_float = 0.;
    let mut rspc: libc::c_float = 0.;
    let mut w: libc::c_float = 0.;
    let mut alspc: libc::c_float = 0.;
    let mut arspc: libc::c_float = 0.;
    arspc = 0 as libc::c_int as libc::c_float;
    alspc = arspc;
    rspc = alspc;
    lspc = rspc;
    ix = 0 as libc::c_int;
    gch = (*s).gch;
    while ix < 8 as libc::c_int {
        if (*gch).type_0 as libc::c_int == '\0' as i32 {
            break;
        }
        match (*gch).type_0 as libc::c_int {
            60 => {
                w = (*gch).w + wlnote;
                if w > alspc {
                    alspc = w;
                }
            }
            62 => {
                w = (*gch).w + (*s).wr;
                if w > arspc {
                    arspc = w;
                }
            }
            _ => {
                let mut wl: libc::c_float = 0.;
                wl = -(*gch).x;
                if wl > lspc {
                    lspc = wl;
                }
                w = (*gch).w + 2 as libc::c_int as libc::c_float - wl;
                if w > rspc {
                    rspc = w;
                }
            }
        }
        ix += 1;
        ix;
        gch = gch.offset(1);
        gch;
    }
    s2 = (*s).prev;
    if !s2.is_null() {
        if !((*s2).gch).is_null() {
            s2 = (*s).ts_prev;
            loop {
                if s2 == (*s).prev {
                    let mut tmp: libc::c_float = lspc;
                    if wlw < tmp {
                        wlw = tmp;
                    }
                    break;
                } else {
                    if (*s2).sflags & 0x80000 as libc::c_int as libc::c_uint != 0 {
                        lspc -= (*s2).shrink;
                    }
                    s2 = (*s2).ts_prev;
                }
            }
        }
        if alspc != 0 as libc::c_int as libc::c_float {
            let mut tmp_0: libc::c_float = alspc;
            if wlw < tmp_0 {
                wlw = tmp_0;
            }
        }
    }
    s2 = (*s).next;
    if !s2.is_null() {
        if !((*s2).gch).is_null() {
            s2 = (*s).ts_next;
            loop {
                if s2 == (*s).next {
                    let mut tmp_1: libc::c_float = rspc;
                    if (*s).wr < tmp_1 {
                        (*s).wr = tmp_1;
                    }
                    break;
                } else {
                    if (*s2).sflags & 0x80000 as libc::c_int as libc::c_uint != 0 {
                        rspc -= 8 as libc::c_int as libc::c_float;
                    }
                    s2 = (*s2).ts_next;
                }
            }
        }
        if arspc != 0 as libc::c_int as libc::c_float {
            let mut tmp_2: libc::c_float = arspc;
            if (*s).wr < tmp_2 {
                (*s).wr = tmp_2;
            }
        }
    }
    return wlw;
}
unsafe extern "C" fn ly_width(
    mut s: *mut SYMBOL,
    mut wlw: libc::c_float,
) -> libc::c_float {
    let mut k: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut ly: *mut lyrics = (*s).ly;
    let mut lyl: *mut lyl = 0 as *mut lyl;
    let mut tblt: *mut tblt_s = 0 as *mut tblt_s;
    let mut align: libc::c_float = 0.;
    let mut xx: libc::c_float = 0.;
    let mut w: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        tblt = voice_tb[(*s).voice as usize].tblts[i as usize];
        if !tblt.is_null() {
            if (*tblt).pitch as libc::c_int == 0 as libc::c_int {
                i = 0 as libc::c_int;
                while i < 16 as libc::c_int {
                    lyl = (*ly).lyl[i as usize];
                    if !lyl.is_null() {
                        (*lyl).s = 0 as libc::c_int as libc::c_float;
                    }
                    i += 1;
                    i;
                }
                return wlw;
            }
        }
        i += 1;
        i;
    }
    align = 0 as libc::c_int as libc::c_float;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        let mut swfac: libc::c_float = 0.;
        let mut shift: libc::c_float = 0.;
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        lyl = (*ly).lyl[i as usize];
        if !lyl.is_null() {
            p = ((*lyl).t).as_mut_ptr();
            w = (*lyl).w;
            swfac = (*(*lyl).f).swfac;
            xx = w
                + 2 as libc::c_int as libc::c_float * cwid(' ' as i32 as libc::c_uchar)
                    * swfac;
            if (*s).type_0 as libc::c_int == 11 as libc::c_int {
                shift = (*s).wl;
            } else if isdigit(*p as libc::c_uchar as libc::c_int) != 0
                && strlen(p) > 2 as libc::c_int as libc::c_ulong
                || *p.offset(1 as libc::c_int as isize) as libc::c_int == ':' as i32
                || *p as libc::c_int == '(' as i32 || *p as libc::c_int == ')' as i32
            {
                let mut sz: libc::c_float = 0.;
                if *p as libc::c_int == '(' as i32 {
                    sz = cwid(*p as libc::c_uchar);
                } else {
                    sz = 0 as libc::c_int as libc::c_float;
                    while *p as libc::c_int != '\0' as i32 {
                        if *p as libc::c_int == '\\' as i32 {
                            p = p.offset(1);
                            p;
                        } else {
                            sz += cwid(*p as libc::c_uchar);
                            if *p as libc::c_int == ' ' as i32 {
                                break;
                            }
                            p = p.offset(1);
                            p;
                        }
                    }
                }
                sz *= swfac;
                shift = ((w - sz
                    + 2 as libc::c_int as libc::c_float
                        * cwid(' ' as i32 as libc::c_uchar) * swfac) as libc::c_double
                    * 0.4f64) as libc::c_float;
                if shift > 20 as libc::c_int as libc::c_float {
                    shift = 20 as libc::c_int as libc::c_float;
                }
                shift += sz;
                if isdigit(
                    (*lyl).t[0 as libc::c_int as usize] as libc::c_uchar as libc::c_int,
                ) != 0
                {
                    if shift > align {
                        align = shift;
                    }
                }
            } else if *p as libc::c_int == 0x10 as libc::c_int
                || *p as libc::c_int == 0x11 as libc::c_int
            {
                shift = 0 as libc::c_int as libc::c_float;
            } else {
                shift = (xx as libc::c_double * 0.4f64) as libc::c_float;
                if shift > 20 as libc::c_int as libc::c_float {
                    shift = 20 as libc::c_int as libc::c_float;
                }
            }
            (*lyl).s = shift;
            let mut tmp: libc::c_float = shift;
            if wlw < tmp {
                wlw = tmp;
            }
            xx -= shift;
            shift = 2 as libc::c_int as libc::c_float * cwid(' ' as i32 as libc::c_uchar)
                * swfac;
            k = (*s).next;
            while !k.is_null() {
                match (*k).type_0 as libc::c_int {
                    1 => {
                        if ((*k).ly).is_null() || ((*(*k).ly).lyl[i as usize]).is_null()
                        {
                            xx -= 9 as libc::c_int as libc::c_float;
                        } else {
                            if !((*(*(*k).ly).lyl[i as usize])
                                .t[0 as libc::c_int as usize] as libc::c_int
                                == 0x10 as libc::c_int
                                || (*(*(*k).ly).lyl[i as usize])
                                    .t[0 as libc::c_int as usize] as libc::c_int
                                    == 0x11 as libc::c_int)
                            {
                                break;
                            }
                            xx -= shift;
                        }
                        if xx <= 0 as libc::c_int as libc::c_float {
                            break;
                        }
                    }
                    4 | 5 | 6 => {
                        xx -= 10 as libc::c_int as libc::c_float;
                    }
                    _ => {
                        xx -= 5 as libc::c_int as libc::c_float;
                        break;
                    }
                }
                k = (*k).next;
            }
            if xx > (*s).wr {
                (*s).wr = xx;
            }
        }
        i += 1;
        i;
    }
    if align > 0 as libc::c_int as libc::c_float {
        i = 0 as libc::c_int;
        while i < 16 as libc::c_int {
            lyl = (*ly).lyl[i as usize];
            if !lyl.is_null() {
                if isdigit(
                    (*lyl).t[0 as libc::c_int as usize] as libc::c_uchar as libc::c_int,
                ) != 0
                {
                    (*lyl).s = align;
                }
            }
            i += 1;
            i;
        }
    }
    return wlw;
}
unsafe extern "C" fn set_width(mut s: *mut SYMBOL) {
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut i: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut xx: libc::c_float = 0.;
    let mut w: libc::c_float = 0.;
    let mut wlnote: libc::c_float = 0.;
    let mut wlw: libc::c_float = 0.;
    match (*s).type_0 as libc::c_int {
        1 => {
            wlnote = hw_tb[(*s).head as usize];
            (*s).wr = wlnote;
            if (*s).xmx > 0 as libc::c_int as libc::c_float {
                (*s).wr += (*s).xmx + 4 as libc::c_int as libc::c_float;
            }
            s2 = (*s).prev;
            if !s2.is_null() {
                match (*s2).type_0 as libc::c_int {
                    3 | 4 | 6 | 5 => {
                        wlnote += 3 as libc::c_int as libc::c_float;
                    }
                    _ => {}
                }
            }
            m = 0 as libc::c_int;
            while m <= (*s).nhd as libc::c_int {
                xx = (*s).u.note.notes[m as usize].shhd;
                if xx < 0 as libc::c_int as libc::c_float {
                    let mut tmp: libc::c_float = -xx + 5 as libc::c_int as libc::c_float;
                    if wlnote < tmp {
                        wlnote = tmp;
                    }
                }
                if (*s).u.note.notes[m as usize].acc != 0 {
                    let mut tmp_0: libc::c_float = ((*s).u.note.notes[m as usize].shac
                        as libc::c_double
                        + (if (*s).u.note.notes[m as usize].acc as libc::c_int
                            & 0xf8 as libc::c_int != 0
                        {
                            6.5f64
                        } else {
                            4.5f64
                        })) as libc::c_float;
                    if wlnote < tmp_0 {
                        wlnote = tmp_0;
                    }
                }
                m += 1;
                m;
            }
            if !s2.is_null() {
                match (*s2).type_0 as libc::c_int {
                    3 | 4 | 6 | 5 => {
                        wlnote -= 3 as libc::c_int as libc::c_float;
                    }
                    _ => {}
                }
            }
            if (*s).u.note.dc.n as libc::c_int != 0 as libc::c_int {
                wlnote += deco_width(s);
            }
            if (*s).sflags & (0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
                == (0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
                && (*s).stem as libc::c_int > 0 as libc::c_int
                && (*s).nflags as libc::c_int > 0 as libc::c_int
            {
                let mut tmp_1: libc::c_float = (*s).xmx
                    + 9 as libc::c_int as libc::c_float;
                if (*s).wr < tmp_1 {
                    (*s).wr = tmp_1;
                }
            }
            if (*s).dots as libc::c_int > 0 as libc::c_int {
                match (*s).head as libc::c_int {
                    3 | 2 => {
                        (*s).xmx += 2 as libc::c_int as libc::c_float;
                    }
                    1 => {
                        (*s).xmx += 1 as libc::c_int as libc::c_float;
                    }
                    _ => {}
                }
                let mut tmp_2: libc::c_float = (*s).xmx
                    + 12 as libc::c_int as libc::c_float;
                if (*s).wr < tmp_2 {
                    (*s).wr = tmp_2;
                }
                if (*s).dots as libc::c_int >= 2 as libc::c_int {
                    (*s)
                        .wr = ((*s).wr as libc::c_double
                        + 3.5f64
                            * ((*s).dots as libc::c_int - 1 as libc::c_int)
                                as libc::c_double) as libc::c_float;
                }
            }
            if (*s).sflags & (0x80 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
                == (0x80 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
            {
                let mut tmp_3: libc::c_float = 20 as libc::c_int as libc::c_float;
                if wlnote < tmp_3 {
                    wlnote = tmp_3;
                }
            }
            wlw = wlnote;
            if !s2.is_null() {
                match (*s2).type_0 as libc::c_int {
                    1 => {
                        if !((*s2).abc_type as libc::c_int == 5 as libc::c_int) {
                            if (*s2).stem as libc::c_int > 0 as libc::c_int
                                && ((*s).stem as libc::c_int) < 0 as libc::c_int
                            {
                                let mut tmp_4: libc::c_float = 7 as libc::c_int
                                    as libc::c_float;
                                if wlw < tmp_4 {
                                    wlw = tmp_4;
                                }
                            }
                            if (*s).y as libc::c_int > 27 as libc::c_int
                                && (*s2).y as libc::c_int > 27 as libc::c_int
                                || ((*s).y as libc::c_int) < -(3 as libc::c_int)
                                    && ((*s2).y as libc::c_int) < -(3 as libc::c_int)
                            {
                                let mut tmp_5: libc::c_float = 6 as libc::c_int
                                    as libc::c_float;
                                if wlw < tmp_5 {
                                    wlw = tmp_5;
                                }
                            }
                            if (*s2).sflags & 0x2000 as libc::c_int as libc::c_uint != 0
                            {
                                let mut tmp_6: libc::c_float = 14 as libc::c_int
                                    as libc::c_float;
                                if wlw < tmp_6 {
                                    wlw = tmp_6;
                                }
                            }
                        }
                    }
                    4 => {
                        if !((*s2).sflags & 0x100000 as libc::c_int as libc::c_uint != 0
                            || (*s2).aux as libc::c_int != 0)
                        {
                            wlw += 8 as libc::c_int as libc::c_float;
                        }
                    }
                    6 => {
                        wlw += 4 as libc::c_int as libc::c_float;
                    }
                    _ => {}
                }
            }
            if !((*s).gch).is_null() {
                wlw = gchord_width(s, wlnote, wlw);
            }
            if !((*s).ly).is_null() {
                wlw = ly_width(s, wlw);
            }
            if !s2.is_null() && (*s2).type_0 as libc::c_int == 11 as libc::c_int {
                (*s).wl = (wlnote as libc::c_double - 4.5f64) as libc::c_float;
            } else {
                (*s).wl = wlw;
            }
        }
        2 => {
            xx = ((*s).u.note.notes[0 as libc::c_int as usize].shhd as libc::c_double
                * 0.5f64) as libc::c_float;
            (*s).wr = xx;
            if !((*s).gch).is_null() {
                xx = gchord_width(s, xx, xx);
            }
            if (*s).u.note.dc.n as libc::c_int != 0 as libc::c_int {
                xx += deco_width(s);
            }
            (*s).wl = xx;
        }
        3 => {
            if (*s).flags as libc::c_int & 0x2 as libc::c_int == 0 {
                let mut bar_type: libc::c_int = 0;
                bar_type = (*s).u.bar.type_0;
                match bar_type {
                    1 => {
                        w = (5 as libc::c_int + 3 as libc::c_int) as libc::c_float;
                    }
                    20 | 65 => {
                        w = (5 as libc::c_int + 3 as libc::c_int + 3 as libc::c_int
                            + 5 as libc::c_int) as libc::c_float;
                    }
                    68 => {
                        w = (5 as libc::c_int + 5 as libc::c_int + 3 as libc::c_int
                            + 3 as libc::c_int + 3 as libc::c_int + 5 as libc::c_int)
                            as libc::c_float;
                    }
                    _ => {
                        w = 5 as libc::c_int as libc::c_float;
                        loop {
                            match bar_type & 0xf as libc::c_int {
                                2 | 3 => {
                                    w += 3 as libc::c_int as libc::c_float;
                                }
                                4 => {
                                    w += 2 as libc::c_int as libc::c_float;
                                }
                                _ => {}
                            }
                            bar_type >>= 4 as libc::c_int;
                            if bar_type == 0 as libc::c_int {
                                break;
                            }
                            w += 3 as libc::c_int as libc::c_float;
                        }
                    }
                }
                (*s).wl = w;
                if !((*s).next).is_null()
                    && (*(*s).next).type_0 as libc::c_int != 5 as libc::c_int
                {
                    (*s).wr = 8 as libc::c_int as libc::c_float;
                } else {
                    (*s).wr = 5 as libc::c_int as libc::c_float;
                }
            }
            if (*s).u.bar.dc.n as libc::c_int != 0 as libc::c_int {
                (*s).wl += deco_width(s);
            }
            if !((*s).gch).is_null()
                && strlen((*s).text) < 4 as libc::c_int as libc::c_ulong
            {
                (*s).wl = gchord_width(s, (*s).wl, (*s).wl);
            }
        }
        4 => {
            (*s).wl = (12 as libc::c_int + 10 as libc::c_int) as libc::c_float;
            (*s)
                .wr = ((if (*s).aux as libc::c_int != 0 {
                10 as libc::c_int
            } else {
                12 as libc::c_int
            }) - 10 as libc::c_int) as libc::c_float;
        }
        6 => {
            let mut n1: libc::c_int = 0;
            let mut n2: libc::c_int = 0;
            let mut esp: libc::c_int = 0;
            (*s).wl = 3 as libc::c_int as libc::c_float;
            esp = 4 as libc::c_int;
            if (*s).u.key.nacc as libc::c_int == 0 as libc::c_int {
                n1 = (*s).u.key.sf as libc::c_int;
                if cfmt.cancelkey != 0 || n1 == 0 as libc::c_int {
                    n2 = (*s).aux as libc::c_int;
                } else {
                    n2 = 0 as libc::c_int;
                }
                if n1 * n2 >= 0 as libc::c_int {
                    if n1 < 0 as libc::c_int {
                        n1 = -n1;
                    }
                    if n2 < 0 as libc::c_int {
                        n2 = -n2;
                    }
                    if n2 > n1 {
                        n1 = n2;
                    }
                } else {
                    n1 -= n2;
                    if n1 < 0 as libc::c_int {
                        n1 = -n1;
                    }
                    esp += 3 as libc::c_int;
                }
            } else {
                let mut last_acc: libc::c_int = 0;
                n2 = (*s).u.key.nacc as libc::c_int;
                n1 = n2;
                last_acc = (*s).u.key.accs[0 as libc::c_int as usize] as libc::c_int;
                i = 1 as libc::c_int;
                while i < n2 {
                    if (*s).u.key.pits[i as usize] as libc::c_int
                        > (*s).u.key.pits[(i - 1 as libc::c_int) as usize] as libc::c_int
                            + 6 as libc::c_int
                        || ((*s).u.key.pits[i as usize] as libc::c_int)
                            < (*s).u.key.pits[(i - 1 as libc::c_int) as usize]
                                as libc::c_int - 6 as libc::c_int
                    {
                        n1 -= 1;
                        n1;
                    } else if (*s).u.key.accs[i as usize] as libc::c_int != last_acc {
                        esp += 3 as libc::c_int;
                    }
                    last_acc = (*s).u.key.accs[i as usize] as libc::c_int;
                    i += 1;
                    i;
                }
            }
            (*s)
                .wr = (5.5f64 * n1 as libc::c_double + esp as libc::c_double)
                as libc::c_float;
        }
        5 => {
            w = 0 as libc::c_int as libc::c_float;
            i = 0 as libc::c_int;
            while i < (*s).u.meter.nmeter as libc::c_int {
                let mut l: libc::c_int = 0;
                l = ::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong
                    as libc::c_int;
                if (*s).u.meter.meter[i as usize].top[(l - 1 as libc::c_int) as usize]
                    as libc::c_int == '\0' as i32
                {
                    l = strlen(((*s).u.meter.meter[i as usize].top).as_mut_ptr())
                        as libc::c_int;
                    if (*s).u.meter.meter[i as usize].top[1 as libc::c_int as usize]
                        as libc::c_int == '|' as i32
                        || (*s).u.meter.meter[i as usize].top[1 as libc::c_int as usize]
                            as libc::c_int == '.' as i32
                    {
                        l -= 1;
                        l;
                    }
                }
                if (*s).u.meter.meter[i as usize].bot[0 as libc::c_int as usize]
                    as libc::c_int != '\0' as i32
                {
                    let mut l2: libc::c_int = 0;
                    l2 = ::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                        as libc::c_int;
                    if (*s)
                        .u
                        .meter
                        .meter[i as usize]
                        .bot[(l2 - 1 as libc::c_int) as usize] as libc::c_int
                        == '\0' as i32
                    {
                        l2 = strlen(((*s).u.meter.meter[i as usize].bot).as_mut_ptr())
                            as libc::c_int;
                    }
                    if l2 > l {
                        l = l2;
                    }
                }
                w = (w as libc::c_double + 6.5f64 * l as libc::c_double)
                    as libc::c_float;
                i += 1;
                i;
            }
            (*s).wl = w;
            (*s).wr = w + 7 as libc::c_int as libc::c_float;
        }
        9 => {
            (*s)
                .wl = (40 as libc::c_int / 2 as libc::c_int + 16 as libc::c_int)
                as libc::c_float;
            (*s)
                .wr = (40 as libc::c_int / 2 as libc::c_int + 16 as libc::c_int)
                as libc::c_float;
        }
        11 => {
            (*s).wl = set_graceoffs(s);
            if !((*s).ly).is_null() {
                ly_width(s, 0 as libc::c_int as libc::c_float);
            }
        }
        14 => {
            if !((*s).next).is_null()
                && (*(*s).next).type_0 as libc::c_int == 4 as libc::c_int
            {
                (*s).wr = 2 as libc::c_int as libc::c_float;
                (*(*s).next).aux = 0 as libc::c_int as libc::c_short;
            } else {
                (*s).wr = 8 as libc::c_int as libc::c_float;
            }
            (*s).wl = (*s).xmx;
        }
        12 => {}
        _ => {
            bug(
                b"Cannot set width for symbol\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                1 as libc::c_int,
            );
        }
    };
}
unsafe extern "C" fn set_space(mut s: *mut SYMBOL) -> libc::c_float {
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut stemdir: libc::c_int = 0;
    let mut prev_time: libc::c_int = 0;
    let mut space: libc::c_float = 0.;
    prev_time = (*(*s).ts_prev).time;
    len = (*s).time - prev_time;
    if len == 0 as libc::c_int {
        match (*s).type_0 as libc::c_int {
            9 => return (*s).wl + 16 as libc::c_int as libc::c_float,
            1 => {
                if (*(*s).ts_prev).type_0 as libc::c_int == 3 as libc::c_int {
                    i = 2 as libc::c_int;
                    if ((*s).nflags as libc::c_int) < -(2 as libc::c_int) {
                        i = 0 as libc::c_int;
                    }
                    return space_tb[i as usize];
                }
            }
            _ => {}
        }
        return 0 as libc::c_int as libc::c_float;
    }
    if (*(*s).ts_prev).type_0 as libc::c_int == 9 as libc::c_int {
        return (*(*s).ts_prev).wr + 16 as libc::c_int as libc::c_float
            + 3 as libc::c_int as libc::c_float;
    }
    if smallest_duration >= 1536 as libc::c_int / 2 as libc::c_int {
        if smallest_duration >= 1536 as libc::c_int {
            len /= 4 as libc::c_int;
        } else {
            len /= 2 as libc::c_int;
        }
    }
    if len >= 1536 as libc::c_int / 4 as libc::c_int {
        if len < 1536 as libc::c_int / 2 as libc::c_int {
            i = 5 as libc::c_int;
        } else if len < 1536 as libc::c_int {
            i = 6 as libc::c_int;
        } else if len < 1536 as libc::c_int * 2 as libc::c_int {
            i = 7 as libc::c_int;
        } else if len < 1536 as libc::c_int * 2 as libc::c_int * 2 as libc::c_int {
            i = 8 as libc::c_int;
        } else {
            i = 9 as libc::c_int;
        }
    } else if len >= 1536 as libc::c_int / 8 as libc::c_int {
        i = 4 as libc::c_int;
    } else if len >= 1536 as libc::c_int / 16 as libc::c_int {
        i = 3 as libc::c_int;
    } else if len >= 1536 as libc::c_int / 16 as libc::c_int / 2 as libc::c_int {
        i = 2 as libc::c_int;
    } else if len >= 1536 as libc::c_int / 16 as libc::c_int / 4 as libc::c_int {
        i = 1 as libc::c_int;
    } else {
        i = 0 as libc::c_int;
    }
    l = len - ((1536 as libc::c_int / 16 as libc::c_int / 8 as libc::c_int) << i);
    space = space_tb[i as usize];
    if l != 0 as libc::c_int {
        if l < 0 as libc::c_int {
            space = space_tb[0 as libc::c_int as usize] * len as libc::c_float
                / (1536 as libc::c_int / 16 as libc::c_int / 8 as libc::c_int)
                    as libc::c_float;
        } else {
            if i >= 9 as libc::c_int {
                i = 8 as libc::c_int;
            }
            space
                += (space_tb[(i + 1 as libc::c_int) as usize] - space_tb[i as usize])
                    * l as libc::c_float / len as libc::c_float;
        }
    }
    if (*s).dur == 0 as libc::c_int {
        if (*s).type_0 as libc::c_int == 3 as libc::c_int {
            if (*s).u.bar.type_0 & 0xf0 as libc::c_int != 0 {
                space = (space as libc::c_double * 0.8f64) as libc::c_float;
            } else {
                space = (space as libc::c_double * 0.7f64) as libc::c_float;
            }
        }
        return space;
    }
    if (*s).sflags & 0x2 as libc::c_int as libc::c_uint == 0 {
        space = (space as libc::c_double * 0.9f64) as libc::c_float;
    }
    if (*s).abc_type as libc::c_int == 4 as libc::c_int
        && (*s).nflags as libc::c_int >= -(1 as libc::c_int)
        && (*s).stem as libc::c_int > 0 as libc::c_int
    {
        stemdir = 1 as libc::c_int;
        s2 = (*s).ts_prev;
        while !s2.is_null() && (*s2).time == prev_time {
            if ((*s2).nflags as libc::c_int) < -(1 as libc::c_int)
                || (*s2).stem as libc::c_int > 0 as libc::c_int
            {
                stemdir = 0 as libc::c_int;
                break;
            } else {
                s2 = (*s2).ts_prev;
            }
        }
        if stemdir != 0 {
            s2 = (*s).ts_next;
            while !s2.is_null() && (*s2).time == (*s).time {
                if ((*s2).nflags as libc::c_int) < -(1 as libc::c_int)
                    || ((*s2).stem as libc::c_int) < 0 as libc::c_int
                {
                    stemdir = 0 as libc::c_int;
                    break;
                } else {
                    s2 = (*s2).ts_next;
                }
            }
            if stemdir != 0 {
                space = (space as libc::c_double * 0.9f64) as libc::c_float;
            }
        }
    }
    return space;
}
unsafe extern "C" fn set_allsymwidth(mut last_s: *mut SYMBOL) {
    let mut new_val: libc::c_float = 0.;
    let mut maxx: libc::c_float = 0.;
    let mut s: *mut SYMBOL = tsfirst;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut xa: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut xl: [libc::c_float; 32] = [0.; 32];
    memset(
        xl.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[libc::c_float; 32]>() as libc::c_ulong,
    );
    loop {
        maxx = xa;
        s2 = s;
        loop {
            set_width(s);
            new_val = xl[(*s).staff as usize] + (*s).wl;
            if new_val > maxx {
                maxx = new_val;
            }
            s = (*s).ts_next;
            if !(s != last_s
                && (*s).sflags & 0x80000 as libc::c_int as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint)
            {
                break;
            }
        }
        (*s2).shrink = maxx - xa;
        if !((*s2).ts_prev).is_null() {
            (*s2).space = set_space(s2);
        }
        if (*s2).shrink == 0 as libc::c_int as libc::c_float
            && (*s2).space == 0 as libc::c_int as libc::c_float
            && (*s2).type_0 as libc::c_int == 4 as libc::c_int
        {
            (*s2).sflags &= !(0x80000 as libc::c_int) as libc::c_uint;
            (*s2).time = (*(*s2).ts_prev).time;
        }
        if s == last_s {
            return;
        }
        xa = maxx;
        s = s2;
        loop {
            if xl[(*s).staff as usize] < xa + (*s).wr {
                xl[(*s).staff as usize] = xa + (*s).wr;
            }
            s = (*s).ts_next;
            if !((*s).sflags & 0x80000 as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint)
            {
                break;
            }
        }
    };
}
unsafe extern "C" fn to_rest(mut s: *mut SYMBOL) {
    (*s).type_0 = 1 as libc::c_int as libc::c_uchar;
    (*s).abc_type = 5 as libc::c_int as libc::c_char;
    (*s).sflags &= (0x40000 as libc::c_int | 0x80000 as libc::c_int) as libc::c_uint;
    (*s).doty = -(1 as libc::c_int) as libc::c_schar;
    (*s).u.note.dc.n = 0 as libc::c_int as libc::c_char;
    (*s).gch = 0 as *mut gch;
    (*s).extra = 0 as *mut SYMBOL;
    (*s).u.note.slur_end = 0 as libc::c_int as libc::c_char;
    (*s).u.note.slur_st = (*s).u.note.slur_end as libc::c_uchar;
}
unsafe extern "C" fn set_repeat(mut g: *mut SYMBOL, mut s: *mut SYMBOL) {
    let mut current_block: u64;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s3: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut dur: libc::c_int = 0;
    let mut staff: libc::c_int = 0;
    let mut voice: libc::c_int = 0;
    staff = (*s).staff as libc::c_int;
    voice = (*s).voice as libc::c_int;
    n = (*g).doty as libc::c_int;
    if n < 0 as libc::c_int {
        n = -n;
        i = n;
        s3 = (*s).prev;
        loop {
            if s3.is_null() {
                current_block = 2868539653012386629;
                break;
            }
            if (*s3).dur == 0 as libc::c_int {
                if (*s3).type_0 as libc::c_int == 3 as libc::c_int {
                    error(
                        0 as libc::c_int,
                        s3,
                        b"Bar in sequence to repeat\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                    current_block = 17572711083185699900;
                    break;
                }
            } else {
                i -= 1;
                if i <= 0 as libc::c_int {
                    current_block = 2868539653012386629;
                    break;
                }
            }
            s3 = (*s3).prev;
        }
        match current_block {
            17572711083185699900 => {}
            _ => {
                if s3.is_null() {
                    error(
                        0 as libc::c_int,
                        s,
                        b"Not enough symbols to repeat\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                } else {
                    dur = (*s).time - (*s3).time;
                    i = (*g).nohdi1 as libc::c_int * n;
                    s2 = s;
                    loop {
                        if s2.is_null() {
                            current_block = 12147880666119273379;
                            break;
                        }
                        if (*s2).dur == 0 as libc::c_int {
                            if (*s2).type_0 as libc::c_int == 3 as libc::c_int {
                                error(
                                    0 as libc::c_int,
                                    s2,
                                    b"Bar in repeat sequence\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                );
                                current_block = 17572711083185699900;
                                break;
                            }
                        } else {
                            i -= 1;
                            if i <= 0 as libc::c_int {
                                current_block = 12147880666119273379;
                                break;
                            }
                        }
                        s2 = (*s2).next;
                    }
                    match current_block {
                        17572711083185699900 => {}
                        _ => {
                            if s2.is_null() || ((*s2).next).is_null() {
                                error(
                                    0 as libc::c_int,
                                    s,
                                    b"Not enough symbols after repeat sequence\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                );
                            } else {
                                s2 = (*s).prev;
                                while s2 != s3 {
                                    if (*s2).abc_type as libc::c_int == 4 as libc::c_int {
                                        (*s2).sflags |= 0x10 as libc::c_int as libc::c_uint;
                                        break;
                                    } else {
                                        s2 = (*s2).prev;
                                    }
                                }
                                j = (*g).nohdi1 as libc::c_int;
                                loop {
                                    j -= 1;
                                    if !(j >= 0 as libc::c_int) {
                                        break;
                                    }
                                    i = n;
                                    if (*s).dur != 0 as libc::c_int {
                                        i -= 1;
                                        i;
                                    }
                                    s2 = (*s).ts_next;
                                    while i > 0 as libc::c_int {
                                        if (*s2).staff as libc::c_int == staff {
                                            (*s2).extra = 0 as *mut SYMBOL;
                                            unlksym(s2);
                                            if (*s2).voice as libc::c_int == voice && (*s2).dur != 0 {
                                                i -= 1;
                                                i;
                                            }
                                        }
                                        s2 = (*s2).ts_next;
                                    }
                                    to_rest(s);
                                    (*s).u.note.notes[0 as libc::c_int as usize].len = dur;
                                    (*s).dur = (*s).u.note.notes[0 as libc::c_int as usize].len;
                                    (*s).sflags |= 0x20000 as libc::c_int as libc::c_uint;
                                    set_width(s);
                                    if (*s).sflags & 0x80000 as libc::c_int as libc::c_uint != 0
                                    {
                                        (*s).space = set_space(s);
                                    }
                                    (*s).head = 3 as libc::c_int as libc::c_uchar;
                                    s = s2;
                                    while !s.is_null() {
                                        if (*s).staff as libc::c_int == staff
                                            && (*s).voice as libc::c_int == voice && (*s).dur != 0
                                        {
                                            break;
                                        }
                                        s = (*s).ts_next;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        i = n;
        s2 = (*(*s).prev).prev;
        while !s2.is_null() {
            if (*s2).type_0 as libc::c_int == 3 as libc::c_int
                || (*s2).time == (*tsfirst).time
            {
                i -= 1;
                if i <= 0 as libc::c_int {
                    break;
                }
            }
            s2 = (*s2).prev;
        }
        if s2.is_null() {
            error(
                0 as libc::c_int,
                s,
                b"Not enough measures to repeat\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        } else {
            dur = (*s).time - (*s2).time;
            if n == 1 as libc::c_int {
                i = (*g).nohdi1 as libc::c_int;
            } else {
                i = n;
            }
            s2 = s;
            while !s2.is_null() {
                if (*s2).type_0 as libc::c_int == 3 as libc::c_int {
                    i -= 1;
                    if i <= 0 as libc::c_int {
                        break;
                    }
                }
                s2 = (*s2).next;
            }
            if s2.is_null() {
                error(
                    0 as libc::c_int,
                    s,
                    b"Not enough bars after repeat measure\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            } else {
                i = (*g).nohdi1 as libc::c_int;
                if n == 2 as libc::c_int && i > 1 as libc::c_int {
                    s2 = (*s2).next;
                    if s2.is_null() {
                        error(
                            0 as libc::c_int,
                            s,
                            b"Not enough bars after repeat measure\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                        current_block = 17572711083185699900;
                    } else {
                        (*g).nohdi1 = 1 as libc::c_int as libc::c_schar;
                        s = getarena(
                            ::core::mem::size_of::<SYMBOL>() as libc::c_ulong
                                as libc::c_int,
                        ) as *mut SYMBOL;
                        memcpy(
                            s as *mut libc::c_void,
                            g as *const libc::c_void,
                            ::core::mem::size_of::<SYMBOL>() as libc::c_ulong,
                        );
                        (*s).next = (*s2).extra;
                        if !((*s).next).is_null() {
                            (*(*s).next).prev = s;
                        }
                        (*s).prev = 0 as *mut SYMBOL;
                        (*s2).extra = s;
                        i -= 1;
                        (*s).nohdi1 = i as libc::c_schar;
                        current_block = 479107131381816815;
                    }
                } else {
                    current_block = 479107131381816815;
                }
                match current_block {
                    17572711083185699900 => {}
                    _ => {
                        dur /= n;
                        if n == 2 as libc::c_int {
                            s3 = s;
                            s2 = (*s).ts_next;
                            loop {
                                if !((*s2).staff as libc::c_int != staff) {
                                    if (*s2).voice as libc::c_int == voice
                                        && (*s2).type_0 as libc::c_int == 3 as libc::c_int
                                    {
                                        break;
                                    }
                                    (*s2).extra = 0 as *mut SYMBOL;
                                    unlksym(s2);
                                }
                                s2 = (*s2).ts_next;
                            }
                            to_rest(s3);
                            (*s3).u.note.notes[0 as libc::c_int as usize].len = dur;
                            (*s3)
                                .dur = (*s3).u.note.notes[0 as libc::c_int as usize].len;
                            (*s3).flags = 0x2 as libc::c_int as libc::c_ushort;
                            if (*s3).sflags & 0x80000 as libc::c_int as libc::c_uint != 0
                            {
                                (*s3).space = set_space(s3);
                            }
                            (*s2).u.bar.len = 2 as libc::c_int as libc::c_char;
                            if (*s2).sflags & 0x80000 as libc::c_int as libc::c_uint != 0
                            {
                                (*s2).space = set_space(s2);
                            }
                            s3 = (*s2).next;
                            s2 = (*s3).ts_next;
                            loop {
                                if !((*s2).staff as libc::c_int != staff) {
                                    if (*s2).voice as libc::c_int == voice
                                        && (*s2).type_0 as libc::c_int == 3 as libc::c_int
                                    {
                                        break;
                                    }
                                    (*s2).extra = 0 as *mut SYMBOL;
                                    unlksym(s2);
                                }
                                s2 = (*s2).ts_next;
                            }
                            to_rest(s3);
                            (*s3).u.note.notes[0 as libc::c_int as usize].len = dur;
                            (*s3)
                                .dur = (*s3).u.note.notes[0 as libc::c_int as usize].len;
                            (*s3).flags = 0x2 as libc::c_int as libc::c_ushort;
                            set_width(s3);
                            if (*s3).sflags & 0x80000 as libc::c_int as libc::c_uint != 0
                            {
                                (*s3).space = set_space(s3);
                            }
                            if (*s2).sflags & 0x80000 as libc::c_int as libc::c_uint != 0
                            {
                                (*s2).space = set_space(s2);
                            }
                            return;
                        }
                        s3 = s;
                        j = (*g).nohdi1 as libc::c_int;
                        loop {
                            j -= 1;
                            if !(j >= 0 as libc::c_int) {
                                break;
                            }
                            s2 = (*s3).ts_next;
                            loop {
                                if !((*s2).staff as libc::c_int != staff) {
                                    if (*s2).voice as libc::c_int == voice
                                        && (*s2).type_0 as libc::c_int == 3 as libc::c_int
                                    {
                                        break;
                                    }
                                    (*s2).extra = 0 as *mut SYMBOL;
                                    unlksym(s2);
                                }
                                s2 = (*s2).ts_next;
                            }
                            to_rest(s3);
                            (*s3).u.note.notes[0 as libc::c_int as usize].len = dur;
                            (*s3)
                                .dur = (*s3).u.note.notes[0 as libc::c_int as usize].len;
                            (*s3).sflags |= 0x20000 as libc::c_int as libc::c_uint;
                            if (*s3).sflags & 0x80000 as libc::c_int as libc::c_uint != 0
                            {
                                (*s3).space = set_space(s3);
                            }
                            if (*s2).sflags & 0x80000 as libc::c_int as libc::c_uint != 0
                            {
                                (*s2).space = set_space(s2);
                            }
                            if (*g).nohdi1 as libc::c_int == 1 as libc::c_int {
                                (*s3).doty = 1 as libc::c_int as libc::c_schar;
                                break;
                            } else {
                                (*s3)
                                    .doty = ((*g).nohdi1 as libc::c_int - j + 1 as libc::c_int)
                                    as libc::c_schar;
                                s3 = (*s2).next;
                            }
                        }
                        return;
                    }
                }
            }
        }
    }
    (*g).aux = -(1 as libc::c_int) as libc::c_short;
}
unsafe extern "C" fn custos_add(mut s: *mut SYMBOL) {
    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut new_s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut i: libc::c_int = 0;
    s2 = s;
    loop {
        if s2.is_null() {
            return;
        }
        if (*s2).abc_type as libc::c_int == 4 as libc::c_int {
            break;
        }
        s2 = (*s2).next;
    }
    p_voice = &mut *voice_tb.as_mut_ptr().offset((*s).voice as isize) as *mut VOICE_S;
    (*p_voice).last_sym = (*s).prev;
    if ((*p_voice).last_sym).is_null() {
        (*p_voice).sym = 0 as *mut SYMBOL;
    }
    (*p_voice).time = (*s).time;
    new_s = sym_add(p_voice, 15 as libc::c_int);
    (*new_s).next = s;
    (*s).prev = new_s;
    (*new_s).ts_prev = (*s).ts_prev;
    (*(*new_s).ts_prev).ts_next = new_s;
    (*new_s).ts_next = s;
    (*s).ts_prev = new_s;
    (*new_s).sflags |= 0x80000 as libc::c_int as libc::c_uint;
    (*new_s).wl = 8 as libc::c_int as libc::c_float;
    (*new_s).wr = 4 as libc::c_int as libc::c_float;
    (*new_s).shrink = (*s).shrink;
    if (*new_s).shrink < (8 as libc::c_int + 4 as libc::c_int) as libc::c_float {
        (*new_s).shrink = (8 as libc::c_int + 4 as libc::c_int) as libc::c_float;
    }
    (*new_s).space = (*s2).space;
    (*new_s).nhd = (*s2).nhd;
    i = 0 as libc::c_int;
    while i <= (*new_s).nhd as libc::c_int {
        (*new_s).pits[i as usize] = (*s2).pits[i as usize];
        (*new_s).u.note.notes[i as usize].len = 1536 as libc::c_int / 4 as libc::c_int;
        i += 1;
        i;
    }
    (*new_s).flags = 0x8 as libc::c_int as libc::c_ushort;
}
unsafe extern "C" fn set_nl(mut s: *mut SYMBOL) -> *mut SYMBOL {
    let mut current_block: u64;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut extra: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut new_sy: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut done: libc::c_int = 0;
    if (*s).sflags & 0x1 as libc::c_int as libc::c_uint != 0 && cfmt.keywarn == 0
        && cfmt.timewarn == 0
    {
        s = (*s).next;
        if s.is_null() {
            return s;
        }
        while (*s).sflags & 0x80000 as libc::c_int as libc::c_uint == 0 {
            s = (*s).ts_prev;
        }
    } else {
        match (*s).type_0 as libc::c_int {
            4 | 3 => {
                current_block = 2370887241019905314;
            }
            6 => {
                if cfmt.keywarn != 0 && (*s).u.key.empty == 0 {
                    current_block = 2370887241019905314;
                } else {
                    current_block = 2626880832993077584;
                }
            }
            5 => {
                if cfmt.timewarn != 0 {
                    current_block = 2370887241019905314;
                } else {
                    current_block = 2626880832993077584;
                }
            }
            11 => {
                s = (*s).next;
                if s.is_null() {
                    return s;
                }
                current_block = 2626880832993077584;
            }
            _ => {
                current_block = 2626880832993077584;
            }
        }
        match current_block {
            2626880832993077584 => {
                s = (*s).next;
                if s.is_null() {
                    return s;
                }
                while (*s).sflags & 0x80000 as libc::c_int as libc::c_uint == 0 {
                    s = (*s).ts_prev;
                }
            }
            _ => {
                while !s.is_null() {
                    if !((*s).sflags & 0x80000 as libc::c_int as libc::c_uint == 0) {
                        match (*s).type_0 as libc::c_int {
                            4 | 6 | 5 => {}
                            _ => {
                                break;
                            }
                        }
                    }
                    s = (*s).ts_prev;
                }
                done = 0 as libc::c_int;
                extra = 0 as *mut SYMBOL;
                new_sy = extra;
                loop {
                    if s.is_null() {
                        return s;
                    }
                    if (*s).sflags & 0x8000000 as libc::c_int as libc::c_uint != 0 {
                        new_sy = s;
                    }
                    if !((*s).sflags & 0x80000 as libc::c_int as libc::c_uint == 0) {
                        if done < 0 as libc::c_int {
                            break;
                        }
                        match (*s).type_0 as libc::c_int {
                            3 => {
                                if done != 0 {
                                    break;
                                }
                                done = 1 as libc::c_int;
                            }
                            14 => {
                                if (*s).doty as libc::c_int == 0 as libc::c_int {
                                    unlksym(s);
                                } else {
                                    done = -(1 as libc::c_int);
                                }
                            }
                            5 => {
                                if cfmt.timewarn == 0 {
                                    break;
                                }
                            }
                            4 => {
                                if done != 0 {
                                    break;
                                }
                            }
                            6 => {
                                if cfmt.keywarn == 0 || (*s).u.key.empty as libc::c_int != 0
                                {
                                    break;
                                }
                            }
                            _ => {
                                if !(done == 0
                                    || !((*s).prev).is_null()
                                        && (*(*s).prev).type_0 as libc::c_int == 11 as libc::c_int)
                                {
                                    break;
                                }
                            }
                        }
                        if !((*s).extra).is_null()
                            && (*s).type_0 as libc::c_int != 11 as libc::c_int
                        {
                            if extra.is_null() {
                                extra = s;
                            } else {
                                error(
                                    0 as libc::c_int,
                                    s,
                                    b"abcm2ps problem: Extra symbol may be misplaced\0"
                                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                                );
                            }
                        }
                    }
                    s = (*s).ts_next;
                }
                if !extra.is_null() && extra != s {
                    s2 = (*extra).extra;
                    while !((*s2).next).is_null() {
                        s2 = (*s2).next;
                    }
                    (*s2).next = (*s).extra;
                    (*s).extra = (*extra).extra;
                    (*extra).extra = 0 as *mut SYMBOL;
                }
                if !new_sy.is_null() && s != new_sy {
                    (*new_sy).sflags &= !(0x8000000 as libc::c_int) as libc::c_uint;
                    (*s).sflags |= 0x8000000 as libc::c_int as libc::c_uint;
                }
            }
        }
    }
    if cfmt.custos != 0 && ((*first_voice).next).is_null() {
        custos_add(s);
    } else {
        s2 = (*s).ts_prev;
        match (*s2).type_0 as libc::c_int {
            3 | 12 | 4 | 6 | 5 => {}
            _ => {
                p_voice = &mut *voice_tb.as_mut_ptr().offset((*s2).voice as isize)
                    as *mut VOICE_S;
                (*p_voice).last_sym = s2;
                (*p_voice).time = (*s).time;
                s2 = (*s2).next;
                extra = sym_add(p_voice, 12 as libc::c_int);
                (*extra).next = s2;
                if !s2.is_null() {
                    (*s2).prev = extra;
                }
                (*extra).ts_prev = (*extra).prev;
                (*(*extra).ts_prev).ts_next = extra;
                (*extra).ts_next = s;
                (*s).ts_prev = extra;
                (*extra).aux = -(1 as libc::c_int) as libc::c_short;
                (*extra).sflags |= 0x80000 as libc::c_int as libc::c_uint;
                (*extra).wl = 6 as libc::c_int as libc::c_float;
                (*extra).wr = 6 as libc::c_int as libc::c_float;
                (*extra).shrink = (*s).shrink;
                (*extra).space = (*s).space;
                if (*s).x != 0 as libc::c_int as libc::c_float {
                    (*extra).x = (*s).x - 1 as libc::c_int as libc::c_float;
                }
            }
        }
    }
    (*s).sflags |= 0x40000 as libc::c_int as libc::c_uint;
    return s;
}
unsafe extern "C" fn set_lines(
    mut first: *mut SYMBOL,
    mut last: *mut SYMBOL,
    mut lwidth: libc::c_float,
    mut indent: libc::c_float,
) -> *mut SYMBOL {
    let mut current_block: u64;
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s3: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut x: libc::c_float = 0.;
    let mut xmin: libc::c_float = 0.;
    let mut xmax: libc::c_float = 0.;
    let mut wwidth: libc::c_float = 0.;
    let mut shrink: libc::c_float = 0.;
    let mut space: libc::c_float = 0.;
    let mut nlines: libc::c_int = 0;
    let mut beam: libc::c_int = 0;
    let mut bar_time: libc::c_int = 0;
    wwidth = indent;
    s = first;
    while s != last {
        if !((*s).sflags & 0x80000 as libc::c_int as libc::c_uint == 0) {
            (*s).x = wwidth;
            shrink = (*s).shrink;
            space = (*s).space;
            if space < shrink {
                wwidth += shrink;
            } else {
                wwidth
                    += shrink * cfmt.maxshrink
                        + space * (1 as libc::c_int as libc::c_float - cfmt.maxshrink);
            }
        }
        s = (*s).ts_next;
    }
    s = first;
    loop {
        nlines = ((wwidth / lwidth) as libc::c_double + 0.999f64) as libc::c_int;
        if nlines <= 1 as libc::c_int {
            if !last.is_null() {
                last = set_nl(last);
            }
            return last;
        }
        first = s;
        s2 = first;
        xmin = (*s).x + wwidth / nlines as libc::c_float * cfmt.breaklimit;
        xmax = (*s).x + lwidth;
        loop {
            if !(s != last) {
                current_block = 11194104282611034094;
                break;
            }
            x = (*s).x;
            if !(x == 0 as libc::c_int as libc::c_float) {
                if x > xmax {
                    current_block = 11194104282611034094;
                    break;
                }
                if !((*s).type_0 as libc::c_int != 3 as libc::c_int) {
                    if x > xmin {
                        current_block = 10909530066327524379;
                        break;
                    }
                    s2 = s;
                }
            }
            s = (*s).ts_next;
        }
        match current_block {
            11194104282611034094 => {
                if s == last {
                    return last;
                }
                if !((*s).type_0 as libc::c_int == 3 as libc::c_int) {
                    bar_time = (*s2).time;
                    beam = if (*s2).type_0 as libc::c_int == 1 as libc::c_int
                        && (*s2).sflags
                            & (0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
                            == 0 as libc::c_int as libc::c_uint
                    {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    };
                    s = s2;
                    s3 = 0 as *mut SYMBOL;
                    s2 = s3;
                    xmax -= 6 as libc::c_int as libc::c_float;
                    while s != last {
                        if (*s).sflags
                            & (0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
                            == 0x2 as libc::c_int as libc::c_uint
                        {
                            beam += 1;
                            beam;
                        } else {
                            if (*s).sflags
                                & (0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
                                == 0x10 as libc::c_int as libc::c_uint
                            {
                                beam -= 1;
                                beam;
                            }
                            x = (*s).x;
                            if !(x < xmin) {
                                if x + (*s).shrink >= xmax {
                                    break;
                                }
                                if !(beam != 0 as libc::c_int) {
                                    s2 = s;
                                    if ((*s).time - bar_time)
                                        % (1536 as libc::c_int / 4 as libc::c_int
                                            / 2 as libc::c_int) == 0 as libc::c_int
                                    {
                                        s3 = s;
                                    }
                                }
                            }
                        }
                        s = (*s).ts_next;
                    }
                    if !s3.is_null() {
                        s2 = s3;
                    }
                    if !s2.is_null() {
                        s = s2;
                    }
                    while !s.is_null()
                        && ((*s).x == 0 as libc::c_int as libc::c_float
                            || (*s).x + (*s).shrink * 2 as libc::c_int as libc::c_float
                                >= xmax)
                    {
                        s = (*s).ts_prev;
                    }
                    if s.is_null() {
                        break;
                    }
                }
            }
            _ => {}
        }
        if (*s).sflags & 0x40000 as libc::c_int as libc::c_uint != 0 {
            error(
                0 as libc::c_int,
                s,
                b"Line split problem - adjust maxshrink and/or breaklimit\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
            nlines = 2 as libc::c_int;
            s = (*s).ts_next;
            while s != last {
                if !((*s).x == 0 as libc::c_int as libc::c_float) {
                    nlines -= 1;
                    if nlines <= 0 as libc::c_int {
                        break;
                    }
                }
                s = (*s).ts_next;
            }
        }
        s = set_nl(s);
        if s.is_null() || !last.is_null() && (*s).time >= (*last).time {
            break;
        }
        wwidth -= (*s).x - (*first).x;
    }
    return s;
}
unsafe extern "C" fn cut_tune(mut lwidth: libc::c_float, mut indent: libc::c_float) {
    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut i: libc::c_int = 0;
    let mut xmin: libc::c_float = 0.;
    s = tsfirst;
    p_voice = first_voice;
    while !p_voice.is_null() {
        i = p_voice.offset_from(voice_tb.as_mut_ptr()) as libc::c_long as libc::c_int;
        if (*cursys).voice[i as usize].range as libc::c_int >= 0 as libc::c_int {
            break;
        }
        p_voice = (*p_voice).next;
    }
    lwidth = (lwidth as libc::c_double
        - ((12 as libc::c_int + 10 as libc::c_int + 12 as libc::c_int - 10 as libc::c_int
            + 3 as libc::c_int + 3 as libc::c_int) as libc::c_double
            + (*p_voice).key.sf as libc::c_int as libc::c_double * 5.5f64))
        as libc::c_float;
    if cfmt.custos != 0 && ((*first_voice).next).is_null() {
        lwidth -= 12 as libc::c_int as libc::c_float;
    }
    if cfmt.continueall != 0 {
        set_lines(s, 0 as *mut SYMBOL, lwidth, indent);
        return;
    }
    i = cfmt.barsperstaff;
    if i != 0 as libc::c_int {
        s2 = s;
        while !s.is_null() {
            if !((*s).type_0 as libc::c_int != 3 as libc::c_int
                || (*s).aux as libc::c_int == 0 as libc::c_int)
            {
                i -= 1;
                if !(i > 0 as libc::c_int) {
                    (*s).sflags |= 0x1 as libc::c_int as libc::c_uint;
                    i = cfmt.barsperstaff;
                }
            }
            s = (*s).ts_next;
        }
        s = s2;
    }
    xmin = indent;
    s2 = s;
    while !s.is_null() {
        if !((*s).sflags & (0x80000 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint
            == 0)
        {
            xmin += (*s).shrink;
            if xmin > lwidth {
                if cfmt.linewarn != 0 {
                    error(
                        0 as libc::c_int,
                        s,
                        b"Line overfull (%.0fpt of %.0fpt)\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        xmin as libc::c_double,
                        lwidth as libc::c_double,
                    );
                }
                while !s.is_null() {
                    if (*s).sflags & 0x1 as libc::c_int as libc::c_uint != 0 {
                        break;
                    }
                    s = (*s).ts_next;
                }
                s2 = set_lines(s2, s, lwidth, indent);
                s = s2;
                if s.is_null() {
                    break;
                }
                xmin = (*s).shrink;
                indent = 0 as libc::c_int as libc::c_float;
            } else if !((*s).sflags & 0x1 as libc::c_int as libc::c_uint == 0) {
                s2 = set_nl(s);
                (*s).sflags &= !(0x1 as libc::c_int) as libc::c_uint;
                s = s2;
                if s.is_null() {
                    break;
                }
                xmin = (*s).shrink;
                indent = 0 as libc::c_int as libc::c_float;
            }
        }
        s = (*s).ts_next;
    }
}
unsafe extern "C" fn set_yval(mut s: *mut SYMBOL) {
    match (*s).type_0 as libc::c_int {
        4 => {
            if (*s).sflags & 0x100000 as libc::c_int as libc::c_uint != 0
                || (*s).flags as libc::c_int & 0x2 as libc::c_int != 0
            {
                (*s).ymn = 12 as libc::c_int as libc::c_schar;
                (*s).ymx = (*s).ymn;
            } else {
                (*s)
                    .y = (((*s).u.clef.line as libc::c_int - 1 as libc::c_int)
                    * 6 as libc::c_int) as libc::c_schar;
                match (*s).u.clef.type_0 as libc::c_int {
                    1 => {
                        (*s)
                            .ymx = ((*s).y as libc::c_int + 13 as libc::c_int)
                            as libc::c_schar;
                        (*s)
                            .ymn = ((*s).y as libc::c_int - 11 as libc::c_int)
                            as libc::c_schar;
                    }
                    2 => {
                        (*s)
                            .ymx = ((*s).y as libc::c_int + 7 as libc::c_int)
                            as libc::c_schar;
                        (*s)
                            .ymn = ((*s).y as libc::c_int - 12 as libc::c_int)
                            as libc::c_schar;
                    }
                    _ => {
                        (*s)
                            .ymx = ((*s).y as libc::c_int + 28 as libc::c_int)
                            as libc::c_schar;
                        (*s)
                            .ymn = ((*s).y as libc::c_int - 14 as libc::c_int)
                            as libc::c_schar;
                    }
                }
                if (*s).aux != 0 {
                    (*s)
                        .ymx = ((*s).ymx as libc::c_int - 2 as libc::c_int)
                        as libc::c_schar;
                    (*s)
                        .ymn = ((*s).ymn as libc::c_int + 2 as libc::c_int)
                        as libc::c_schar;
                }
                if ((*s).ymx as libc::c_int) < 26 as libc::c_int {
                    (*s).ymx = 26 as libc::c_int as libc::c_schar;
                }
                if (*s).ymn as libc::c_int > -(1 as libc::c_int) {
                    (*s).ymn = -(1 as libc::c_int) as libc::c_schar;
                }
                if (*s).u.clef.octave as libc::c_int > 0 as libc::c_int {
                    (*s)
                        .ymx = ((*s).ymx as libc::c_int + 9 as libc::c_int)
                        as libc::c_schar;
                } else if ((*s).u.clef.octave as libc::c_int) < 0 as libc::c_int {
                    (*s)
                        .ymn = ((*s).ymn as libc::c_int - 9 as libc::c_int)
                        as libc::c_schar;
                }
            }
        }
        6 => {
            if (*s).u.key.sf as libc::c_int > 2 as libc::c_int {
                (*s).ymx = (24 as libc::c_int + 10 as libc::c_int) as libc::c_schar;
            } else if (*s).u.key.sf as libc::c_int > 0 as libc::c_int {
                (*s).ymx = (24 as libc::c_int + 6 as libc::c_int) as libc::c_schar;
            } else {
                (*s).ymx = (24 as libc::c_int + 2 as libc::c_int) as libc::c_schar;
            }
            (*s).ymn = -(2 as libc::c_int) as libc::c_schar;
        }
        _ => {
            (*s).ymx = (24 as libc::c_int + 2 as libc::c_int) as libc::c_schar;
            (*s).ymn = -(2 as libc::c_int) as libc::c_schar;
        }
    };
}
unsafe extern "C" fn set_auto_clef(
    mut staff: libc::c_int,
    mut s_start: *mut SYMBOL,
    mut clef_type_start: libc::c_int,
) -> libc::c_int {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s_last: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s_last_chg: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut clef_type: libc::c_int = 0;
    let mut min: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    let mut time: libc::c_int = 0;
    max = 12 as libc::c_int;
    min = 20 as libc::c_int;
    s = s_start;
    while !s.is_null() {
        if (*s).sflags & 0x8000000 as libc::c_int as libc::c_uint != 0 && s != s_start {
            break;
        }
        if !((*s).staff as libc::c_int != staff) {
            if (*s).abc_type as libc::c_int != 4 as libc::c_int {
                if (*s).type_0 as libc::c_int == 4 as libc::c_int {
                    if (*s).u.clef.type_0 as libc::c_int != 4 as libc::c_int {
                        break;
                    }
                    unlksym(s);
                }
            } else if ((*s).pits[0 as libc::c_int as usize] as libc::c_int) < min {
                min = (*s).pits[0 as libc::c_int as usize] as libc::c_int;
            } else if (*s).pits[(*s).nhd as usize] as libc::c_int > max {
                max = (*s).pits[(*s).nhd as usize] as libc::c_int;
            }
        }
        s = (*s).ts_next;
    }
    if min >= 19 as libc::c_int
        || min >= 13 as libc::c_int && clef_type_start != 2 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if max <= 13 as libc::c_int
        || max <= 19 as libc::c_int && clef_type_start != 0 as libc::c_int
    {
        return 2 as libc::c_int;
    }
    if clef_type_start == 4 as libc::c_int {
        if (max + min) / 2 as libc::c_int >= 16 as libc::c_int {
            clef_type_start = 0 as libc::c_int;
        } else {
            clef_type_start = 2 as libc::c_int;
        }
    }
    clef_type = clef_type_start;
    s_last = s;
    s_last_chg = 0 as *mut SYMBOL;
    let mut current_block_43: u64;
    s = s_start;
    while s != s_last {
        let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
        let mut s3: *mut SYMBOL = 0 as *mut SYMBOL;
        if (*s).sflags & 0x8000000 as libc::c_int as libc::c_uint != 0 && s != s_start {
            break;
        }
        if !((*s).staff as libc::c_int != staff
            || (*s).abc_type as libc::c_int != 4 as libc::c_int)
        {
            time = (*s).time;
            if clef_type == 0 as libc::c_int {
                if (*s).pits[0 as libc::c_int as usize] as libc::c_int
                    > 12 as libc::c_int
                    || (*s).pits[(*s).nhd as usize] as libc::c_int > 20 as libc::c_int
                {
                    if (*s).pits[0 as libc::c_int as usize] as libc::c_int
                        > 20 as libc::c_int
                    {
                        s_last_chg = s;
                    }
                    current_block_43 = 13472856163611868459;
                } else {
                    s2 = (*s).ts_prev;
                    if !s2.is_null() && (*s2).time == time
                        && (*s2).staff as libc::c_int == staff
                        && (*s2).abc_type as libc::c_int == 4 as libc::c_int
                        && (*s2).pits[0 as libc::c_int as usize] as libc::c_int
                            >= 19 as libc::c_int
                    {
                        current_block_43 = 13472856163611868459;
                    } else {
                        s2 = (*s).ts_next;
                        if !s2.is_null() && (*s2).staff as libc::c_int == staff
                            && (*s2).time == time
                            && (*s2).abc_type as libc::c_int == 4 as libc::c_int
                            && (*s2).pits[0 as libc::c_int as usize] as libc::c_int
                                >= 19 as libc::c_int
                        {
                            current_block_43 = 13472856163611868459;
                        } else {
                            current_block_43 = 2480299350034459858;
                        }
                    }
                }
            } else if ((*s).pits[0 as libc::c_int as usize] as libc::c_int)
                < 12 as libc::c_int
                || ((*s).pits[(*s).nhd as usize] as libc::c_int) < 20 as libc::c_int
            {
                if ((*s).pits[(*s).nhd as usize] as libc::c_int) < 12 as libc::c_int {
                    s_last_chg = s;
                }
                current_block_43 = 13472856163611868459;
            } else {
                s2 = (*s).ts_prev;
                if !s2.is_null() && (*s2).time == time
                    && (*s2).staff as libc::c_int == staff
                    && (*s2).abc_type as libc::c_int == 4 as libc::c_int
                    && (*s2).pits[0 as libc::c_int as usize] as libc::c_int
                        <= 13 as libc::c_int
                {
                    current_block_43 = 13472856163611868459;
                } else {
                    s2 = (*s).ts_next;
                    if !s2.is_null() && (*s2).staff as libc::c_int == staff
                        && (*s2).time == time
                        && (*s2).abc_type as libc::c_int == 4 as libc::c_int
                        && (*s2).pits[0 as libc::c_int as usize] as libc::c_int
                            <= 13 as libc::c_int
                    {
                        current_block_43 = 13472856163611868459;
                    } else {
                        current_block_43 = 2480299350034459858;
                    }
                }
            }
            match current_block_43 {
                13472856163611868459 => {}
                _ => {
                    if s_last_chg.is_null() {
                        clef_type_start = if clef_type == 0 as libc::c_int {
                            2 as libc::c_int
                        } else {
                            0 as libc::c_int
                        };
                        clef_type = clef_type_start;
                        s_last_chg = s;
                    } else {
                        s3 = s;
                        s2 = (*s).ts_prev;
                        while s2 != s_last_chg {
                            if !((*s2).staff as libc::c_int != staff) {
                                if (*s2).type_0 as libc::c_int == 3 as libc::c_int
                                    && (*s2).voice as libc::c_int == (*s).voice as libc::c_int
                                {
                                    s3 = s2;
                                    break;
                                } else if !((*s2).abc_type as libc::c_int
                                    != 4 as libc::c_int)
                                {
                                    if (*s2).sflags & 0x2 as libc::c_int as libc::c_uint != 0
                                        && (voice_tb[(*s2).voice as usize]).second() == 0
                                    {
                                        s3 = s2;
                                    }
                                }
                            }
                            s2 = (*s2).ts_prev;
                        }
                        if (*s3).time == (*s_last_chg).time {
                            s_last_chg = s;
                        } else {
                            s_last_chg = s;
                            clef_type = if clef_type == 0 as libc::c_int {
                                2 as libc::c_int
                            } else {
                                0 as libc::c_int
                            };
                            s2 = insert_clef(
                                s3,
                                clef_type,
                                if clef_type == 0 as libc::c_int {
                                    2 as libc::c_int
                                } else {
                                    4 as libc::c_int
                                },
                            );
                            (*s2).sflags |= 0x20 as libc::c_int as libc::c_uint;
                        }
                    }
                }
            }
        }
        s = (*s).ts_next;
    }
    return clef_type_start;
}
unsafe extern "C" fn set_clefs() {
    let mut sy: *mut SYSTEM = 0 as *mut SYSTEM;
    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut g: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut staff: libc::c_int = 0;
    let mut voice: libc::c_int = 0;
    let mut pitch: libc::c_int = 0;
    let mut new_type: libc::c_int = 0;
    let mut new_line: libc::c_int = 0;
    let mut old_lvl: libc::c_int = 0;
    let mut staff_clef: [C2RustUnnamed_16; 32] = [C2RustUnnamed_16 {
        clef: 0 as *mut SYMBOL,
        autoclef: 0,
        mid: 0,
    }; 32];
    old_lvl = lvlarena(1 as libc::c_int);
    memset(
        staff_tb.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[STAFF_S; 32]>() as libc::c_ulong,
    );
    staff = 0 as libc::c_int;
    while staff <= nstaff {
        staff_clef[staff as usize].clef = 0 as *mut SYMBOL;
        staff_clef[staff as usize].autoclef = 1 as libc::c_int as libc::c_short;
        staff += 1;
        staff;
    }
    sy = cursys;
    p_voice = first_voice;
    while !p_voice.is_null() {
        voice = p_voice.offset_from(voice_tb.as_mut_ptr()) as libc::c_long
            as libc::c_int;
        if !(((*sy).voice[voice as usize].range as libc::c_int) < 0 as libc::c_int) {
            staff = (*sy).voice[voice as usize].staff as libc::c_int;
            if (*sy).voice[voice as usize].second == 0 {
                if !((*p_voice).stafflines).is_null() {
                    (*sy).staff[staff as usize].stafflines = (*p_voice).stafflines;
                }
                if (*p_voice).staffscale != 0 as libc::c_int as libc::c_float {
                    (*sy).staff[staff as usize].staffscale = (*p_voice).staffscale;
                }
                if (*sy).voice[voice as usize].sep != 0. {
                    (*sy).staff[staff as usize].sep = (*sy).voice[voice as usize].sep;
                }
                if (*sy).voice[voice as usize].maxsep != 0. {
                    (*sy)
                        .staff[staff as usize]
                        .maxsep = (*sy).voice[voice as usize].maxsep;
                }
            }
            s = (*p_voice).s_clef;
            if (*sy).voice[voice as usize].second == 0
                && (*s).sflags & 0x20 as libc::c_int as libc::c_uint == 0
            {
                staff_clef[staff as usize].autoclef = 0 as libc::c_int as libc::c_short;
            }
        }
        p_voice = (*p_voice).next;
    }
    p_voice = first_voice;
    while !p_voice.is_null() {
        voice = p_voice.offset_from(voice_tb.as_mut_ptr()) as libc::c_long
            as libc::c_int;
        if !(((*sy).voice[voice as usize].range as libc::c_int) < 0 as libc::c_int
            || (*sy).voice[voice as usize].second as libc::c_int != 0)
        {
            staff = (*sy).voice[voice as usize].staff as libc::c_int;
            s = (*p_voice).s_clef;
            if staff_clef[staff as usize].autoclef != 0 {
                (*s)
                    .u
                    .clef
                    .type_0 = set_auto_clef(
                    staff,
                    tsfirst,
                    (*s).u.clef.type_0 as libc::c_int,
                ) as libc::c_schar;
                (*s)
                    .u
                    .clef
                    .line = (if (*s).u.clef.type_0 as libc::c_int == 0 as libc::c_int {
                    2 as libc::c_int
                } else {
                    4 as libc::c_int
                }) as libc::c_char;
            }
            staff_tb[staff as usize].s_clef = s;
            staff_clef[staff as usize].clef = staff_tb[staff as usize].s_clef;
        }
        p_voice = (*p_voice).next;
    }
    staff = 0 as libc::c_int;
    while staff <= (*sy).nstaff as libc::c_int {
        staff_clef[staff as usize]
            .mid = (strlen((*sy).staff[staff as usize].stafflines))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(3 as libc::c_int as libc::c_ulong) as libc::c_short;
        staff += 1;
        staff;
    }
    let mut current_block_109: u64;
    s = tsfirst;
    while !s.is_null() {
        g = (*s).extra;
        while !g.is_null() {
            if (*g).type_0 as libc::c_int == 12 as libc::c_int
                && (*g).aux as libc::c_int == 2 as libc::c_int
            {
                set_repeat(g, s);
                break;
            } else {
                g = (*g).next;
            }
        }
        if (*s).sflags & 0x8000000 as libc::c_int as libc::c_uint != 0 {
            sy = (*sy).next;
            staff = 0 as libc::c_int;
            while staff <= nstaff {
                staff_clef[staff as usize].autoclef = 1 as libc::c_int as libc::c_short;
                staff += 1;
                staff;
            }
            p_voice = first_voice;
            while !p_voice.is_null() {
                voice = p_voice.offset_from(voice_tb.as_mut_ptr()) as libc::c_long
                    as libc::c_int;
                if !(((*sy).voice[voice as usize].range as libc::c_int)
                    < 0 as libc::c_int)
                {
                    staff = (*sy).voice[voice as usize].staff as libc::c_int;
                    if (*sy).voice[voice as usize].second == 0 {
                        if !((*p_voice).stafflines).is_null() {
                            (*sy)
                                .staff[staff as usize]
                                .stafflines = (*p_voice).stafflines;
                        }
                        if (*p_voice).staffscale != 0 as libc::c_int as libc::c_float {
                            (*sy)
                                .staff[staff as usize]
                                .staffscale = (*p_voice).staffscale;
                        }
                        if (*sy).voice[voice as usize].sep != 0. {
                            (*sy)
                                .staff[staff as usize]
                                .sep = (*sy).voice[voice as usize].sep;
                        }
                        if (*sy).voice[voice as usize].maxsep != 0. {
                            (*sy)
                                .staff[staff as usize]
                                .maxsep = (*sy).voice[voice as usize].maxsep;
                        }
                    }
                    s2 = (*p_voice).s_clef;
                    if (*s2).sflags & 0x20 as libc::c_int as libc::c_uint == 0 {
                        staff_clef[staff as usize]
                            .autoclef = 0 as libc::c_int as libc::c_short;
                    }
                }
                p_voice = (*p_voice).next;
            }
            staff = 0 as libc::c_int;
            while staff <= (*sy).nstaff as libc::c_int {
                staff_clef[staff as usize]
                    .mid = (strlen((*sy).staff[staff as usize].stafflines))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(3 as libc::c_int as libc::c_ulong) as libc::c_short;
                staff += 1;
                staff;
            }
            p_voice = first_voice;
            while !p_voice.is_null() {
                voice = p_voice.offset_from(voice_tb.as_mut_ptr()) as libc::c_long
                    as libc::c_int;
                if !(((*sy).voice[voice as usize].range as libc::c_int)
                    < 0 as libc::c_int
                    || (*sy).voice[voice as usize].second as libc::c_int != 0)
                {
                    staff = (*sy).voice[voice as usize].staff as libc::c_int;
                    s2 = (*p_voice).s_clef;
                    if (*s2).sflags & 0x20 as libc::c_int as libc::c_uint != 0 {
                        new_type = set_auto_clef(
                            staff,
                            s,
                            if !(staff_clef[staff as usize].clef).is_null() {
                                (*staff_clef[staff as usize].clef).u.clef.type_0
                                    as libc::c_int
                            } else {
                                4 as libc::c_int
                            },
                        );
                        new_line = if new_type == 0 as libc::c_int {
                            2 as libc::c_int
                        } else {
                            4 as libc::c_int
                        };
                    } else {
                        new_type = (*s2).u.clef.type_0 as libc::c_int;
                        new_line = (*s2).u.clef.line as libc::c_int;
                    }
                    if (staff_clef[staff as usize].clef).is_null() {
                        if (*s2).sflags & 0x20 as libc::c_int as libc::c_uint != 0 {
                            if (*s2).u.clef.type_0 as libc::c_int != 4 as libc::c_int {
                                (*p_voice)
                                    .s_clef = getarena(
                                    ::core::mem::size_of::<SYMBOL>() as libc::c_ulong
                                        as libc::c_int,
                                ) as *mut SYMBOL;
                                memcpy(
                                    (*p_voice).s_clef as *mut libc::c_void,
                                    s2 as *const libc::c_void,
                                    ::core::mem::size_of::<SYMBOL>() as libc::c_ulong,
                                );
                            }
                            (*(*p_voice).s_clef)
                                .u
                                .clef
                                .type_0 = new_type as libc::c_schar;
                            (*(*p_voice).s_clef).u.clef.line = new_line as libc::c_char;
                        }
                        staff_clef[staff as usize].clef = (*p_voice).s_clef;
                        staff_tb[staff as usize]
                            .s_clef = staff_clef[staff as usize].clef;
                    } else if !(new_type
                        == (*staff_clef[staff as usize].clef).u.clef.type_0
                            as libc::c_int
                        && new_line
                            == (*staff_clef[staff as usize].clef).u.clef.line
                                as libc::c_int)
                    {
                        g = s;
                        while !g.is_null() && (*g).voice as libc::c_int != voice {
                            g = (*g).ts_next;
                        }
                        if !g.is_null() {
                            if (*g).type_0 as libc::c_int != 4 as libc::c_int {
                                g = insert_clef(g, new_type, new_line);
                                if (*s2).sflags & 0x20 as libc::c_int as libc::c_uint != 0 {
                                    (*g).sflags |= 0x20 as libc::c_int as libc::c_uint;
                                }
                            }
                            (*p_voice).s_clef = g;
                            staff_clef[staff as usize].clef = (*p_voice).s_clef;
                        }
                    }
                }
                p_voice = (*p_voice).next;
            }
        }
        if (*s).type_0 as libc::c_int != 4 as libc::c_int {
            (*s).mid = staff_clef[(*s).staff as usize].mid as libc::c_schar;
        } else {
            if (*s).u.clef.type_0 as libc::c_int == 4 as libc::c_int {
                (*s)
                    .u
                    .clef
                    .type_0 = set_auto_clef(
                    (*s).staff as libc::c_int,
                    (*s).ts_next,
                    (*staff_clef[(*s).staff as usize].clef).u.clef.type_0 as libc::c_int,
                ) as libc::c_schar;
                (*s)
                    .u
                    .clef
                    .line = (if (*s).u.clef.type_0 as libc::c_int == 0 as libc::c_int {
                    2 as libc::c_int
                } else {
                    4 as libc::c_int
                }) as libc::c_char;
            }
            p_voice = &mut *voice_tb.as_mut_ptr().offset((*s).voice as isize)
                as *mut VOICE_S;
            (*p_voice).s_clef = s;
            if (*s).sflags & 0x100000 as libc::c_int as libc::c_uint != 0 {
                unlksym(s);
            } else {
                staff = (*s).staff as libc::c_int;
                if !(staff_tb[staff as usize].s_clef).is_null() {
                    if (*s).u.clef.type_0 as libc::c_int
                        == (*staff_clef[staff as usize].clef).u.clef.type_0
                            as libc::c_int
                        && (*s).u.clef.line as libc::c_int
                            == (*staff_clef[staff as usize].clef).u.clef.line
                                as libc::c_int
                        && (*s).sflags & 0x8000000 as libc::c_int as libc::c_uint == 0
                    {
                        current_block_109 = 2604890879466389055;
                    } else {
                        current_block_109 = 4804377075063615140;
                    }
                } else {
                    staff_tb[staff as usize].s_clef = s;
                    current_block_109 = 4804377075063615140;
                }
                match current_block_109 {
                    2604890879466389055 => {}
                    _ => {
                        staff_clef[staff as usize].clef = s;
                    }
                }
            }
        }
        s = (*s).ts_next;
    }
    sy = cursys;
    p_voice = first_voice;
    while !p_voice.is_null() {
        voice = p_voice.offset_from(voice_tb.as_mut_ptr()) as libc::c_long
            as libc::c_int;
        if !(((*sy).voice[voice as usize].range as libc::c_int) < 0 as libc::c_int) {
            s2 = (*p_voice).sym;
            if !(s2.is_null()
                || (*s2).pits[0 as libc::c_int as usize] as libc::c_int
                    != 127 as libc::c_int)
            {
                staff = (*sy).voice[voice as usize].staff as libc::c_int;
                match (*staff_tb[staff as usize].s_clef).u.clef.type_0 as libc::c_int {
                    1 => {
                        pitch = 16 as libc::c_int;
                    }
                    2 => {
                        pitch = 10 as libc::c_int;
                    }
                    _ => {
                        pitch = 22 as libc::c_int;
                    }
                }
                s = s2;
                while !s.is_null() {
                    (*s).pits[0 as libc::c_int as usize] = pitch as libc::c_schar;
                    s = (*s).next;
                }
            }
        }
        p_voice = (*p_voice).next;
    }
    lvlarena(old_lvl);
}
unsafe extern "C" fn set_pitch(mut last_s: *mut SYMBOL) {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut g: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut staff: libc::c_int = 0;
    let mut delta: libc::c_int = 0;
    let mut dur: libc::c_int = 0;
    let mut staff_delta: [libc::c_schar; 32] = [0; 32];
    static mut delta_tb: [libc::c_schar; 4] = [
        (0 as libc::c_int - 2 as libc::c_int * 2 as libc::c_int) as libc::c_schar,
        (6 as libc::c_int - 3 as libc::c_int * 2 as libc::c_int) as libc::c_schar,
        (12 as libc::c_int - 4 as libc::c_int * 2 as libc::c_int) as libc::c_schar,
        (0 as libc::c_int - 3 as libc::c_int * 2 as libc::c_int) as libc::c_schar,
    ];
    staff = 0 as libc::c_int;
    while staff <= nstaff {
        s = staff_tb[staff as usize].s_clef;
        staff_delta[staff
            as usize] = (delta_tb[(*s).u.clef.type_0 as usize] as libc::c_int
            + (*s).u.clef.line as libc::c_int * 2 as libc::c_int
            + (*s).u.clef.transpose as libc::c_int) as libc::c_schar;
        staff += 1;
        staff;
    }
    dur = 1536 as libc::c_int;
    s = tsfirst;
    while s != last_s {
        let mut np: libc::c_int = 0;
        let mut m: libc::c_int = 0;
        staff = (*s).staff as libc::c_int;
        let mut current_block_44: u64;
        match (*s).type_0 as libc::c_int {
            4 => {
                staff_delta[staff
                    as usize] = (delta_tb[(*s).u.clef.type_0 as usize] as libc::c_int
                    + (*s).u.clef.line as libc::c_int * 2 as libc::c_int
                    + (*s).u.clef.transpose as libc::c_int) as libc::c_schar;
                set_yval(s);
                current_block_44 = 12381812505308290051;
            }
            11 => {
                g = (*s).extra;
                while !g.is_null() {
                    if !((*g).type_0 as libc::c_int != 1 as libc::c_int) {
                        delta = staff_delta[(*g).staff as usize] as libc::c_int;
                        if delta != 0 as libc::c_int
                            && voice_tb[(*s).voice as usize].key.instr as libc::c_int
                                != 3 as libc::c_int
                        {
                            m = (*g).nhd as libc::c_int;
                            while m >= 0 as libc::c_int {
                                (*g)
                                    .pits[m
                                    as usize] = ((*g).pits[m as usize] as libc::c_int + delta)
                                    as libc::c_schar;
                                m -= 1;
                                m;
                            }
                        }
                        (*g)
                            .ymn = (3 as libc::c_int
                            * ((*g).pits[0 as libc::c_int as usize] as libc::c_int
                                - 18 as libc::c_int) - 2 as libc::c_int) as libc::c_schar;
                        (*g)
                            .ymx = (3 as libc::c_int
                            * ((*g).pits[(*g).nhd as usize] as libc::c_int
                                - 18 as libc::c_int) + 2 as libc::c_int) as libc::c_schar;
                    }
                    g = (*g).next;
                }
                set_yval(s);
                current_block_44 = 12381812505308290051;
            }
            6 => {
                (*s).u.key.clef_delta = staff_delta[staff as usize] as libc::c_char;
                current_block_44 = 14109400006505043039;
            }
            9 => {
                if (*s).flags as libc::c_int & 0x2 as libc::c_int != 0 {
                    current_block_44 = 12381812505308290051;
                } else {
                    (*s).ymx = (24 as libc::c_int + 15 as libc::c_int) as libc::c_schar;
                    (*s).ymn = -(2 as libc::c_int) as libc::c_schar;
                    current_block_44 = 12381812505308290051;
                }
            }
            1 => {
                if (*s).abc_type as libc::c_int != 4 as libc::c_int
                    && ((*first_voice).next).is_null()
                {
                    (*s).y = 12 as libc::c_int as libc::c_schar;
                    (*s).ymx = 24 as libc::c_int as libc::c_schar;
                    (*s).ymn = 0 as libc::c_int as libc::c_schar;
                } else {
                    np = (*s).nhd as libc::c_int;
                    delta = staff_delta[staff as usize] as libc::c_int;
                    if delta != 0 as libc::c_int
                        && voice_tb[(*s).voice as usize].key.instr as libc::c_int
                            != 3 as libc::c_int
                    {
                        m = np;
                        while m >= 0 as libc::c_int {
                            (*s)
                                .pits[m
                                as usize] = ((*s).pits[m as usize] as libc::c_int + delta)
                                as libc::c_schar;
                            m -= 1;
                            m;
                        }
                    }
                    if (*s).abc_type as libc::c_int == 4 as libc::c_int {
                        (*s)
                            .ymx = (3 as libc::c_int
                            * ((*s).pits[np as usize] as libc::c_int - 18 as libc::c_int)
                            + 4 as libc::c_int) as libc::c_schar;
                        (*s)
                            .ymn = (3 as libc::c_int
                            * ((*s).pits[0 as libc::c_int as usize] as libc::c_int
                                - 18 as libc::c_int) - 4 as libc::c_int) as libc::c_schar;
                    } else {
                        (*s)
                            .y = (((*s).pits[0 as libc::c_int as usize] as libc::c_int
                            - 18 as libc::c_int) / 2 as libc::c_int * 6 as libc::c_int)
                            as libc::c_schar;
                        (*s)
                            .ymx = ((*s).y as libc::c_int
                            + rest_sp[(5 as libc::c_int - (*s).nflags as libc::c_int)
                                as usize][0 as libc::c_int as usize] as libc::c_int)
                            as libc::c_schar;
                        (*s)
                            .ymn = ((*s).y as libc::c_int
                            - rest_sp[(5 as libc::c_int - (*s).nflags as libc::c_int)
                                as usize][1 as libc::c_int as usize] as libc::c_int)
                            as libc::c_schar;
                    }
                    if (*s).dur < dur {
                        dur = (*s).dur;
                    }
                }
                current_block_44 = 12381812505308290051;
            }
            _ => {
                current_block_44 = 14109400006505043039;
            }
        }
        match current_block_44 {
            14109400006505043039 => {
                set_yval(s);
            }
            _ => {}
        }
        s = (*s).ts_next;
    }
    smallest_duration = dur;
}
unsafe extern "C" fn set_stem_dir() {
    let mut sy: *mut SYSTEM = 0 as *mut SYSTEM;
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut t: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut u: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut i: libc::c_int = 0;
    let mut staff: libc::c_int = 0;
    let mut nst: libc::c_int = 0;
    let mut rvoice: libc::c_int = 0;
    let mut voice: libc::c_int = 0;
    let mut stb: [C2RustUnnamed_14; 32] = [C2RustUnnamed_14 {
        nvoice: 0,
        st: [C2RustUnnamed_13 {
            voice: 0,
            ymn: 0,
            ymx: 0,
        }; 4],
    }; 32];
    let mut vtb: [C2RustUnnamed_15; 32] = [C2RustUnnamed_15 { st1: 0, st2: 0 }; 32];
    s = tsfirst;
    sy = cursys;
    nst = (*sy).nstaff as libc::c_int;
    while !s.is_null() {
        staff = nst;
        while staff >= 0 as libc::c_int {
            stb[staff as usize].nvoice = -(1 as libc::c_int);
            i = 4 as libc::c_int;
            loop {
                i -= 1;
                if !(i >= 0 as libc::c_int) {
                    break;
                }
                stb[staff as usize].st[i as usize].voice = -(1 as libc::c_int);
                stb[staff as usize]
                    .st[i as usize]
                    .ymx = 0 as libc::c_int as libc::c_short;
                stb[staff as usize]
                    .st[i as usize]
                    .ymn = 24 as libc::c_int as libc::c_short;
            }
            staff -= 1;
            staff;
        }
        i = 0 as libc::c_int;
        while i < 32 as libc::c_int {
            vtb[i as usize].st2 = -(1 as libc::c_int) as libc::c_schar;
            vtb[i as usize].st1 = vtb[i as usize].st2;
            i += 1;
            i;
        }
        u = s;
        while !u.is_null() {
            if (*u).type_0 as libc::c_int == 3 as libc::c_int {
                break;
            }
            if (*u).sflags & 0x8000000 as libc::c_int as libc::c_uint != 0 {
                if u != s {
                    break;
                }
                sy = (*sy).next;
                staff = nst;
                while staff <= (*sy).nstaff as libc::c_int {
                    stb[staff as usize].nvoice = -(1 as libc::c_int);
                    i = 4 as libc::c_int;
                    loop {
                        i -= 1;
                        if !(i >= 0 as libc::c_int) {
                            break;
                        }
                        stb[staff as usize].st[i as usize].voice = -(1 as libc::c_int);
                        stb[staff as usize]
                            .st[i as usize]
                            .ymx = 0 as libc::c_int as libc::c_short;
                        stb[staff as usize]
                            .st[i as usize]
                            .ymn = 24 as libc::c_int as libc::c_short;
                    }
                    staff += 1;
                    staff;
                }
                nst = (*sy).nstaff as libc::c_int;
            }
            if !((*u).type_0 as libc::c_int != 1 as libc::c_int
                || (*u).flags as libc::c_int & 0x2 as libc::c_int != 0)
            {
                staff = (*u).staff as libc::c_int;
                if staff > nst {
                    bug(
                        b"set_stem_dir(): bad staff number\n\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        1 as libc::c_int,
                    );
                }
                voice = (*u).voice as libc::c_int;
                if (vtb[voice as usize].st1 as libc::c_int) < 0 as libc::c_int {
                    vtb[voice as usize].st1 = staff as libc::c_schar;
                } else if vtb[voice as usize].st1 as libc::c_int != staff {
                    if staff > vtb[voice as usize].st1 as libc::c_int {
                        if staff > vtb[voice as usize].st2 as libc::c_int {
                            vtb[voice as usize].st2 = staff as libc::c_schar;
                        }
                    } else {
                        if vtb[voice as usize].st1 as libc::c_int
                            > vtb[voice as usize].st2 as libc::c_int
                        {
                            vtb[voice as usize].st2 = vtb[voice as usize].st1;
                        }
                        vtb[voice as usize].st1 = staff as libc::c_schar;
                    }
                }
                rvoice = (*sy).voice[voice as usize].range as libc::c_int;
                i = stb[staff as usize].nvoice;
                while i >= 0 as libc::c_int {
                    if stb[staff as usize].st[i as usize].voice == rvoice {
                        break;
                    }
                    i -= 1;
                    i;
                }
                if i < 0 as libc::c_int {
                    stb[staff as usize].nvoice += 1;
                    if stb[staff as usize].nvoice >= 4 as libc::c_int {
                        bug(
                            b"Too many voices per staff\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            1 as libc::c_int,
                        );
                    }
                    i = 0 as libc::c_int;
                    while i < stb[staff as usize].nvoice {
                        if rvoice < stb[staff as usize].st[i as usize].voice {
                            memmove(
                                &mut *((*stb.as_mut_ptr().offset(staff as isize)).st)
                                    .as_mut_ptr()
                                    .offset((i + 1 as libc::c_int) as isize)
                                    as *mut C2RustUnnamed_13 as *mut libc::c_void,
                                &mut *((*stb.as_mut_ptr().offset(staff as isize)).st)
                                    .as_mut_ptr()
                                    .offset(i as isize) as *mut C2RustUnnamed_13
                                    as *const libc::c_void,
                                (::core::mem::size_of::<C2RustUnnamed_13>()
                                    as libc::c_ulong)
                                    .wrapping_mul(
                                        (stb[staff as usize].nvoice - i) as libc::c_ulong,
                                    ),
                            );
                            stb[staff as usize]
                                .st[i as usize]
                                .ymx = 0 as libc::c_int as libc::c_short;
                            stb[staff as usize]
                                .st[i as usize]
                                .ymn = 24 as libc::c_int as libc::c_short;
                            break;
                        } else {
                            i += 1;
                            i;
                        }
                    }
                    stb[staff as usize].st[i as usize].voice = rvoice;
                }
                if !((*u).abc_type as libc::c_int != 4 as libc::c_int) {
                    if (*u).ymx as libc::c_int
                        > stb[staff as usize].st[i as usize].ymx as libc::c_int
                    {
                        stb[staff as usize]
                            .st[i as usize]
                            .ymx = (*u).ymx as libc::c_short;
                    }
                    if ((*u).ymn as libc::c_int)
                        < stb[staff as usize].st[i as usize].ymn as libc::c_int
                    {
                        stb[staff as usize]
                            .st[i as usize]
                            .ymn = (*u).ymn as libc::c_short;
                    }
                    if (*u).sflags & 0x200 as libc::c_int as libc::c_uint != 0 {
                        if ((*u).ts_prev).is_null()
                            || (*(*u).ts_prev).staff as libc::c_int
                                != staff - 1 as libc::c_int
                            || (*(*u).ts_prev).abc_type as libc::c_int
                                != 4 as libc::c_int
                        {
                            error(
                                1 as libc::c_int,
                                s,
                                b"Bad !xstem!\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                            (*u).sflags &= !(0x200 as libc::c_int) as libc::c_uint;
                        } else {
                            (*(*u).ts_prev).multi = 1 as libc::c_int as libc::c_schar;
                            (*u).multi = 1 as libc::c_int as libc::c_schar;
                            (*u)
                                .flags = ((*u).flags as libc::c_int | 0x8 as libc::c_int)
                                as libc::c_ushort;
                        }
                    }
                }
            }
            u = (*u).ts_next;
        }
        while s != u {
            if !((*s).multi != 0) {
                if !((*s).type_0 as libc::c_int != 1 as libc::c_int
                    && (*s).type_0 as libc::c_int != 11 as libc::c_int)
                {
                    staff = (*s).staff as libc::c_int;
                    voice = (*s).voice as libc::c_int;
                    if vtb[voice as usize].st2 as libc::c_int >= 0 as libc::c_int {
                        if staff == vtb[voice as usize].st1 as libc::c_int {
                            (*s).multi = -(1 as libc::c_int) as libc::c_schar;
                        } else if staff == vtb[voice as usize].st2 as libc::c_int {
                            (*s).multi = 1 as libc::c_int as libc::c_schar;
                        }
                    } else if stb[staff as usize].nvoice <= 0 as libc::c_int {
                        if (*s).sflags & 0x200000 as libc::c_int as libc::c_uint != 0 {
                            if staff == voice_tb[voice as usize].staff as libc::c_int {
                                (*s).multi = -(1 as libc::c_int) as libc::c_schar;
                            } else {
                                (*s).multi = 1 as libc::c_int as libc::c_schar;
                            }
                        }
                    } else {
                        rvoice = (*sy).voice[voice as usize].range as libc::c_int;
                        i = stb[staff as usize].nvoice;
                        while i >= 0 as libc::c_int {
                            if stb[staff as usize].st[i as usize].voice == rvoice {
                                break;
                            }
                            i -= 1;
                            i;
                        }
                        if !(i < 0 as libc::c_int) {
                            if i == stb[staff as usize].nvoice {
                                (*s).multi = -(1 as libc::c_int) as libc::c_schar;
                            } else {
                                (*s).multi = 1 as libc::c_int as libc::c_schar;
                                if i != 0 as libc::c_int
                                    && i + 1 as libc::c_int == stb[staff as usize].nvoice
                                {
                                    if stb[staff as usize].st[i as usize].ymn as libc::c_int
                                        as libc::c_float - cfmt.stemheight
                                        > stb[staff as usize]
                                            .st[(i + 1 as libc::c_int) as usize]
                                            .ymx as libc::c_int as libc::c_float
                                    {
                                        (*s).multi = -(1 as libc::c_int) as libc::c_schar;
                                    }
                                    if !((*s).ts_prev).is_null()
                                        && (*(*s).ts_prev).time == (*s).time
                                        && (*(*s).ts_prev).staff as libc::c_int
                                            == (*s).staff as libc::c_int
                                        && (*s).pits[(*s).nhd as usize] as libc::c_int
                                            == (*(*s).ts_prev).pits[0 as libc::c_int as usize]
                                                as libc::c_int
                                        && (*s).sflags
                                            & (0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
                                            == (0x2 as libc::c_int | 0x10 as libc::c_int)
                                                as libc::c_uint
                                        && {
                                            t = (*s).ts_next;
                                            t.is_null()
                                                || (*t).staff as libc::c_int != (*s).staff as libc::c_int
                                                || (*t).time != (*s).time
                                        }
                                    {
                                        (*s).multi = -(1 as libc::c_int) as libc::c_schar;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            s = (*s).ts_next;
        }
        while !s.is_null() && (*s).type_0 as libc::c_int == 3 as libc::c_int {
            if (*s).sflags & 0x8000000 as libc::c_int as libc::c_uint != 0 {
                sy = (*sy).next;
                nst = (*sy).nstaff as libc::c_int;
            }
            s = (*s).ts_next;
        }
    }
}
unsafe extern "C" fn set_rest_offset() {
    let mut sy: *mut SYSTEM = 0 as *mut SYSTEM;
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut nvoice: libc::c_int = 0;
    let mut voice: libc::c_int = 0;
    let mut end_time: libc::c_int = 0;
    let mut not_alone: libc::c_int = 0;
    let mut ymax: libc::c_int = 0;
    let mut ymin: libc::c_int = 0;
    let mut shift: libc::c_int = 0;
    let mut dots: libc::c_int = 0;
    let mut dx: libc::c_float = 0.;
    let mut vtb: [C2RustUnnamed_12; 32] = [C2RustUnnamed_12 {
        s: 0 as *mut SYMBOL,
        staff: 0,
        end_time: 0,
    }; 32];
    let mut v: *mut C2RustUnnamed_12 = 0 as *mut C2RustUnnamed_12;
    memset(
        vtb.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[C2RustUnnamed_12; 32]>() as libc::c_ulong,
    );
    sy = cursys;
    nvoice = 0 as libc::c_int;
    s = tsfirst;
    while !s.is_null() {
        if !((*s).flags as libc::c_int & 0x2 as libc::c_int != 0) {
            if (*s).sflags & 0x8000000 as libc::c_int as libc::c_uint != 0 {
                sy = (*sy).next;
            }
            if !((*s).type_0 as libc::c_int != 1 as libc::c_int) {
                if (*s).voice as libc::c_int > nvoice {
                    nvoice = (*s).voice as libc::c_int;
                }
                v = &mut *vtb.as_mut_ptr().offset((*s).voice as isize)
                    as *mut C2RustUnnamed_12;
                (*v).s = s;
                (*v).staff = (*s).staff as libc::c_int;
                (*v).end_time = (*s).time + (*s).dur;
                if !((*s).abc_type as libc::c_int != 5 as libc::c_int) {
                    ymin = -(127 as libc::c_int);
                    ymax = 127 as libc::c_int;
                    dots = 0 as libc::c_int;
                    not_alone = dots;
                    voice = 0 as libc::c_int;
                    v = vtb.as_mut_ptr();
                    while voice <= nvoice {
                        s2 = (*v).s;
                        if !(s2.is_null() || (*v).staff != (*s).staff as libc::c_int
                            || voice == (*s).voice as libc::c_int)
                        {
                            if !((*v).end_time <= (*s).time) {
                                not_alone += 1;
                                not_alone;
                                if ((*sy).voice[voice as usize].range as libc::c_int)
                                    < (*sy).voice[(*s).voice as usize].range as libc::c_int
                                {
                                    if (*s2).time == (*s).time {
                                        if ((*s2).ymn as libc::c_int) < ymax {
                                            ymax = (*s2).ymn as libc::c_int;
                                            if (*s2).dots != 0 {
                                                dots = 1 as libc::c_int;
                                            }
                                        }
                                    } else if ((*s2).y as libc::c_int) < ymax {
                                        ymax = (*s2).y as libc::c_int;
                                    }
                                } else if (*s2).time == (*s).time {
                                    if (*s2).ymx as libc::c_int > ymin {
                                        ymin = (*s2).ymx as libc::c_int;
                                        if (*s2).dots != 0 {
                                            dots = 1 as libc::c_int;
                                        }
                                    }
                                } else if (*s2).y as libc::c_int > ymin {
                                    ymin = (*s2).y as libc::c_int;
                                }
                            }
                        }
                        voice += 1;
                        voice;
                        v = v.offset(1);
                        v;
                    }
                    end_time = (*s).time + (*s).dur;
                    s2 = (*s).ts_next;
                    while !s2.is_null() {
                        if (*s2).time >= end_time {
                            break;
                        }
                        if !((*s2).staff as libc::c_int != (*s).staff as libc::c_int
                            || (*s2).type_0 as libc::c_int != 1 as libc::c_int
                            || (*s2).flags as libc::c_int & 0x2 as libc::c_int != 0)
                        {
                            not_alone += 1;
                            not_alone;
                            if ((*sy).voice[(*s2).voice as usize].range as libc::c_int)
                                < (*sy).voice[(*s).voice as usize].range as libc::c_int
                            {
                                if (*s2).time == (*s).time {
                                    if ((*s2).ymn as libc::c_int) < ymax {
                                        ymax = (*s2).ymn as libc::c_int;
                                        if (*s2).dots != 0 {
                                            dots = 1 as libc::c_int;
                                        }
                                    }
                                } else if ((*s2).y as libc::c_int) < ymax {
                                    ymax = (*s2).y as libc::c_int;
                                }
                            } else if (*s2).time == (*s).time {
                                if (*s2).ymx as libc::c_int > ymin {
                                    ymin = (*s2).ymx as libc::c_int;
                                    if (*s2).dots != 0 {
                                        dots = 1 as libc::c_int;
                                    }
                                }
                            } else if (*s2).y as libc::c_int > ymin {
                                ymin = (*s2).y as libc::c_int;
                            }
                        }
                        s2 = (*s2).ts_next;
                    }
                    shift = ymax - (*s).ymx as libc::c_int;
                    if shift < 0 as libc::c_int {
                        shift = (-shift + 5 as libc::c_int) / 6 as libc::c_int
                            * 6 as libc::c_int;
                        if (*s).ymn as libc::c_int - shift >= ymin {
                            (*s).y = ((*s).y as libc::c_int - shift) as libc::c_schar;
                            (*s)
                                .ymx = ((*s).ymx as libc::c_int - shift) as libc::c_schar;
                            (*s)
                                .ymn = ((*s).ymn as libc::c_int - shift) as libc::c_schar;
                        } else {
                            dx = (if dots != 0 {
                                15 as libc::c_int
                            } else {
                                10 as libc::c_int
                            }) as libc::c_float;
                            (*s).u.note.notes[0 as libc::c_int as usize].shhd = dx;
                            (*s).xmx = dx;
                        }
                    } else {
                        shift = ymin - (*s).ymn as libc::c_int;
                        if shift > 0 as libc::c_int {
                            shift = (shift + 5 as libc::c_int) / 6 as libc::c_int
                                * 6 as libc::c_int;
                            if (*s).ymx as libc::c_int + shift <= ymax {
                                (*s).y = ((*s).y as libc::c_int + shift) as libc::c_schar;
                                (*s)
                                    .ymx = ((*s).ymx as libc::c_int + shift) as libc::c_schar;
                                (*s)
                                    .ymn = ((*s).ymn as libc::c_int + shift) as libc::c_schar;
                            } else {
                                dx = (if dots != 0 {
                                    15 as libc::c_int
                                } else {
                                    10 as libc::c_int
                                }) as libc::c_float;
                                (*s).u.note.notes[0 as libc::c_int as usize].shhd = dx;
                                (*s).xmx = dx;
                            }
                        } else if not_alone == 0 {
                            (*s).y = 12 as libc::c_int as libc::c_schar;
                            (*s).ymx = 24 as libc::c_int as libc::c_schar;
                            (*s).ymn = 0 as libc::c_int as libc::c_schar;
                        }
                    }
                }
            }
        }
        s = (*s).ts_next;
    }
}
unsafe extern "C" fn sym_new(
    mut type_0: libc::c_int,
    mut p_voice: *mut VOICE_S,
    mut last_s: *mut SYMBOL,
) -> *mut SYMBOL {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    s = getarena(::core::mem::size_of::<SYMBOL>() as libc::c_ulong as libc::c_int)
        as *mut SYMBOL;
    memset(
        s as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<SYMBOL>() as libc::c_ulong,
    );
    (*s).type_0 = type_0 as libc::c_uchar;
    (*s)
        .voice = p_voice.offset_from(voice_tb.as_mut_ptr()) as libc::c_long
        as libc::c_uchar;
    (*s).staff = (*p_voice).staff;
    (*s).time = (*last_s).time;
    (*s).next = (*(*p_voice).last_sym).next;
    if !((*s).next).is_null() {
        (*(*s).next).prev = s;
    }
    (*(*p_voice).last_sym).next = s;
    (*s).prev = (*p_voice).last_sym;
    (*p_voice).last_sym = s;
    (*s).ts_next = last_s;
    (*s).ts_prev = (*last_s).ts_prev;
    if !((*s).ts_prev).is_null() {
        (*(*s).ts_prev).ts_next = s;
    }
    if ((*s).ts_prev).is_null() || (*(*s).ts_prev).type_0 as libc::c_int != type_0 {
        (*s).sflags |= 0x80000 as libc::c_int as libc::c_uint;
    }
    (*last_s).ts_prev = s;
    if (*last_s).type_0 as libc::c_int == type_0
        && (*s).voice as libc::c_int != (*last_s).voice as libc::c_int
    {
        (*last_s).sflags &= !(0x80000 as libc::c_int) as libc::c_uint;
        (*last_s).shrink = 0 as libc::c_int as libc::c_float;
    }
    (*s).fn_0 = (*last_s).fn_0;
    (*s).linenum = (*last_s).linenum;
    (*s).colnum = (*last_s).colnum;
    return s;
}
unsafe extern "C" fn init_music_line() {
    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut last_s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut voice: libc::c_int = 0;
    let mut staff: libc::c_int = 0;
    p_voice = first_voice;
    while !p_voice.is_null() {
        voice = p_voice.offset_from(voice_tb.as_mut_ptr()) as libc::c_long
            as libc::c_int;
        (*p_voice).last_sym = (*p_voice).sym;
        if !(((*cursys).voice[voice as usize].range as libc::c_int) < 0 as libc::c_int) {
            (*p_voice)
                .set_second((*cursys).voice[voice as usize].second as libc::c_uint);
            staff = (*cursys).voice[voice as usize].staff as libc::c_int;
            while staff <= nstaff
                && (*cursys).staff[staff as usize].empty as libc::c_int != 0
            {
                staff += 1;
                staff;
            }
            if staff <= nstaff {
                (*p_voice).staff = staff as libc::c_uchar;
            }
        }
        p_voice = (*p_voice).next;
    }
    last_s = tsfirst;
    while !last_s.is_null() && (*last_s).type_0 as libc::c_int == 4 as libc::c_int {
        voice = (*last_s).voice as libc::c_int;
        p_voice = &mut *voice_tb.as_mut_ptr().offset(voice as isize) as *mut VOICE_S;
        if (*cursys).voice[voice as usize].range as libc::c_int >= 0 as libc::c_int
            && (*cursys).voice[voice as usize].second == 0
        {
            (*last_s).aux = 0 as libc::c_int as libc::c_short;
            (*p_voice).sym = last_s;
            (*p_voice).last_sym = (*p_voice).sym;
        }
        last_s = (*last_s).ts_next;
    }
    p_voice = first_voice;
    while !p_voice.is_null() {
        if !(!((*p_voice).sym).is_null()
            && (*(*p_voice).sym).type_0 as libc::c_int == 4 as libc::c_int)
        {
            voice = p_voice.offset_from(voice_tb.as_mut_ptr()) as libc::c_long
                as libc::c_int;
            if !(((*cursys).voice[voice as usize].range as libc::c_int)
                < 0 as libc::c_int
                || (*cursys).voice[voice as usize].second as libc::c_int != 0)
            {
                staff = (*cursys).voice[voice as usize].staff as libc::c_int;
                if !(staff_tb[staff as usize].s_clef).is_null() {
                    s = getarena(
                        ::core::mem::size_of::<SYMBOL>() as libc::c_ulong as libc::c_int,
                    ) as *mut SYMBOL;
                    memset(
                        s as *mut libc::c_void,
                        0 as libc::c_int,
                        ::core::mem::size_of::<SYMBOL>() as libc::c_ulong,
                    );
                    memcpy(
                        &mut (*s).u.clef as *mut clef_s as *mut libc::c_void,
                        &mut (*(*staff_tb.as_mut_ptr().offset(staff as isize)).s_clef)
                            .u
                            .clef as *mut clef_s as *const libc::c_void,
                        ::core::mem::size_of::<clef_s>() as libc::c_ulong,
                    );
                    (*s).type_0 = 4 as libc::c_int as libc::c_uchar;
                    (*s).voice = voice as libc::c_uchar;
                    (*s).staff = staff as libc::c_uchar;
                    (*s)
                        .time = if !last_s.is_null() {
                        (*last_s).time
                    } else {
                        0 as libc::c_int
                    };
                    (*s).next = (*p_voice).sym;
                    if !((*s).next).is_null() {
                        (*(*s).next).prev = s;
                        (*s).fn_0 = (*(*s).next).fn_0;
                        (*s).linenum = (*(*s).next).linenum;
                        (*s).colnum = (*(*s).next).colnum;
                    }
                    (*p_voice).sym = s;
                    (*p_voice).last_sym = (*p_voice).sym;
                    (*s).ts_next = last_s;
                    (*s)
                        .ts_prev = if !last_s.is_null() {
                        (*last_s).ts_prev
                    } else {
                        0 as *mut SYMBOL
                    };
                    if ((*s).ts_prev).is_null() {
                        tsfirst = s;
                        (*s).sflags |= 0x80000 as libc::c_int as libc::c_uint;
                    } else {
                        (*(*s).ts_prev).ts_next = s;
                    }
                    if !last_s.is_null() {
                        (*last_s).ts_prev = s;
                        if (*last_s).type_0 as libc::c_int == 4 as libc::c_int {
                            (*last_s).sflags
                                &= !(0x80000 as libc::c_int) as libc::c_uint;
                        }
                    }
                    if (*staff_tb[staff as usize].s_clef).u.clef.invis as libc::c_int
                        != 0 || (*cursys).staff[staff as usize].empty as libc::c_int != 0
                    {
                        (*s)
                            .flags = ((*s).flags as libc::c_int | 0x2 as libc::c_int)
                            as libc::c_ushort;
                    }
                }
            }
        }
        p_voice = (*p_voice).next;
    }
    p_voice = first_voice;
    while !p_voice.is_null() {
        voice = p_voice.offset_from(voice_tb.as_mut_ptr()) as libc::c_long
            as libc::c_int;
        if !(((*cursys).voice[voice as usize].range as libc::c_int) < 0 as libc::c_int
            || (*cursys).voice[voice as usize].second as libc::c_int != 0
            || (*cursys).staff[(*cursys).voice[voice as usize].staff as usize].empty
                as libc::c_int != 0)
        {
            if !last_s.is_null() && (*last_s).voice as libc::c_int == voice
                && (*last_s).type_0 as libc::c_int == 6 as libc::c_int
            {
                (*p_voice).last_sym = last_s;
                (*last_s).aux = (*last_s).u.key.sf as libc::c_short;
                last_s = (*last_s).ts_next;
            } else if (*p_voice).key.sf as libc::c_int != 0 as libc::c_int
                || (*p_voice).key.nacc as libc::c_int != 0 as libc::c_int
            {
                s = sym_new(6 as libc::c_int, p_voice, last_s);
                memcpy(
                    &mut (*s).u.key as *mut key_s as *mut libc::c_void,
                    &mut (*p_voice).key as *mut key_s as *const libc::c_void,
                    ::core::mem::size_of::<key_s>() as libc::c_ulong,
                );
                if (*s).u.key.instr as libc::c_int == 2 as libc::c_int {
                    (*s).aux = 3 as libc::c_int as libc::c_short;
                }
            }
        }
        p_voice = (*p_voice).next;
    }
    if insert_meter & 1 as libc::c_int != 0 {
        p_voice = first_voice;
        while !p_voice.is_null() {
            voice = p_voice.offset_from(voice_tb.as_mut_ptr()) as libc::c_long
                as libc::c_int;
            if !(((*cursys).voice[voice as usize].range as libc::c_int)
                < 0 as libc::c_int
                || (*cursys).voice[voice as usize].second as libc::c_int != 0
                || (*cursys).staff[(*cursys).voice[voice as usize].staff as usize].empty
                    as libc::c_int != 0
                || (*p_voice).meter.nmeter as libc::c_int == 0 as libc::c_int)
            {
                if !last_s.is_null() && (*last_s).voice as libc::c_int == voice
                    && (*last_s).type_0 as libc::c_int == 5 as libc::c_int
                {
                    (*p_voice).last_sym = last_s;
                    last_s = (*last_s).ts_next;
                } else {
                    s = sym_new(5 as libc::c_int, p_voice, last_s);
                    memcpy(
                        &mut (*s).u.meter as *mut meter_s as *mut libc::c_void,
                        &mut (*p_voice).meter as *mut meter_s as *const libc::c_void,
                        ::core::mem::size_of::<meter_s>() as libc::c_ulong,
                    );
                }
            }
            p_voice = (*p_voice).next;
        }
        insert_meter &= !(1 as libc::c_int);
    }
    p_voice = first_voice;
    while !p_voice.is_null() {
        let mut bar_start: libc::c_int = 0;
        voice = p_voice.offset_from(voice_tb.as_mut_ptr()) as libc::c_long
            as libc::c_int;
        bar_start = (*p_voice).bar_start as libc::c_int;
        (*p_voice).bar_start = 0 as libc::c_int as libc::c_short;
        if !last_s.is_null() && (*last_s).voice as libc::c_int == voice
            && (*last_s).type_0 as libc::c_int == 3 as libc::c_int
        {
            (*p_voice).last_sym = last_s;
            last_s = (*last_s).ts_next;
        } else if !(bar_start == 0) {
            if !(((*cursys).voice[voice as usize].range as libc::c_int)
                < 0 as libc::c_int
                || (*cursys).voice[voice as usize].second as libc::c_int != 0
                || (*cursys).staff[(*cursys).voice[voice as usize].staff as usize].empty
                    as libc::c_int != 0)
            {
                s = sym_new(3 as libc::c_int, p_voice, last_s);
                (*s).u.bar.type_0 = bar_start & 0xfff as libc::c_int;
                if bar_start & 0x8000 as libc::c_int != 0 {
                    (*s)
                        .flags = ((*s).flags as libc::c_int | 0x2 as libc::c_int)
                        as libc::c_ushort;
                }
                if bar_start & 0x4000 as libc::c_int != 0 {
                    (*s).sflags |= 0x400000 as libc::c_int as libc::c_uint;
                }
                if bar_start & 0x2000 as libc::c_int != 0 {
                    (*s)
                        .flags = ((*s).flags as libc::c_int | 0x100 as libc::c_int)
                        as libc::c_ushort;
                }
                if bar_start & 0x1000 as libc::c_int != 0 {
                    (*s).sflags |= 0x10000000 as libc::c_int as libc::c_uint;
                }
                (*s).text = (*p_voice).bar_text;
                (*s).gch = (*p_voice).bar_gch;
                if (*p_voice).bar_repeat() != 0 {
                    (*s).u.bar.repeat_bar = (*p_voice).bar_repeat() as libc::c_char;
                }
                (*p_voice).set_bar_repeat(0 as libc::c_int as libc::c_uint);
                (*p_voice).bar_text = 0 as *mut libc::c_char;
                (*p_voice).bar_gch = 0 as *mut gch;
            }
        }
        p_voice = (*p_voice).next;
    }
    set_pitch(last_s);
    s = last_s;
    if !s.is_null() {
        while !s.is_null() {
            if (*s).sflags & 0x80000 as libc::c_int as libc::c_uint != 0 {
                break;
            }
            s = (*s).ts_next;
        }
        if !s.is_null() {
            s = (*s).ts_next;
            while !s.is_null() {
                if (*s).sflags & 0x80000 as libc::c_int as libc::c_uint != 0 {
                    break;
                }
                s = (*s).ts_next;
            }
        }
    }
    set_allsymwidth(s);
}
unsafe extern "C" fn set_words(mut p_voice: *mut VOICE_S) {
    let mut pitch: libc::c_int = 0;
    let mut beam_start: libc::c_int = 0;
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut lastnote: *mut SYMBOL = 0 as *mut SYMBOL;
    s = (*p_voice).sym;
    while !s.is_null() {
        if (*s).abc_type as libc::c_int == 4 as libc::c_int {
            pitch = (*s).pits[0 as libc::c_int as usize] as libc::c_int;
            break;
        } else {
            s = (*s).next;
        }
    }
    if s.is_null() {
        pitch = 127 as libc::c_int;
    }
    beam_start = 1 as libc::c_int;
    lastnote = 0 as *mut SYMBOL;
    s = (*p_voice).sym;
    while !s.is_null() {
        match (*s).type_0 as libc::c_int {
            9 => {
                beam_start = 1 as libc::c_int;
            }
            3 => {
                if (*s).sflags & 0x400 as libc::c_int as libc::c_uint == 0 {
                    beam_start = 1 as libc::c_int;
                }
                if ((*s).next).is_null() && !((*s).prev).is_null()
                    && (*(*s).prev).abc_type as libc::c_int == 4 as libc::c_int
                    && (*(*s).prev).dur >= 1536 as libc::c_int * 2 as libc::c_int
                {
                    (*(*s).prev).head = 3 as libc::c_int as libc::c_uchar;
                }
            }
            1 => {
                if !((*s).sflags & 0x80 as libc::c_int as libc::c_uint != 0) {
                    if (*s).flags as libc::c_int & 0x4 as libc::c_int != 0 {
                        beam_start = 1 as libc::c_int;
                    }
                    if beam_start != 0
                        || (*s).nflags as libc::c_int - (*s).aux as libc::c_int
                            <= 0 as libc::c_int
                    {
                        if !lastnote.is_null() {
                            (*lastnote).sflags |= 0x10 as libc::c_int as libc::c_uint;
                            lastnote = 0 as *mut SYMBOL;
                        }
                        if (*s).nflags as libc::c_int - (*s).aux as libc::c_int
                            <= 0 as libc::c_int
                        {
                            (*s).sflags
                                |= (0x2 as libc::c_int | 0x10 as libc::c_int)
                                    as libc::c_uint;
                        } else if (*s).abc_type as libc::c_int == 4 as libc::c_int {
                            (*s).sflags |= 0x2 as libc::c_int as libc::c_uint;
                            beam_start = 0 as libc::c_int;
                        }
                    }
                    if (*s).sflags & 0x10 as libc::c_int as libc::c_uint != 0 {
                        beam_start = 1 as libc::c_int;
                    }
                    if (*s).abc_type as libc::c_int == 4 as libc::c_int {
                        lastnote = s;
                    }
                }
            }
            _ => {
                if (*s).flags as libc::c_int & 0x4 as libc::c_int != 0 {
                    beam_start = 1 as libc::c_int;
                }
            }
        }
        if (*s).abc_type as libc::c_int == 4 as libc::c_int {
            pitch = (*s).pits[0 as libc::c_int as usize] as libc::c_int;
            s2 = (*s).prev;
            while !s2.is_null() {
                if (*s2).abc_type as libc::c_int != 5 as libc::c_int {
                    break;
                }
                (*s2).pits[0 as libc::c_int as usize] = pitch as libc::c_schar;
                s2 = (*s2).prev;
            }
        } else {
            (*s).pits[0 as libc::c_int as usize] = pitch as libc::c_schar;
        }
        s = (*s).next;
    }
    if !lastnote.is_null() {
        (*lastnote).sflags |= 0x10 as libc::c_int as libc::c_uint;
    }
}
unsafe extern "C" fn set_rb(mut p_voice: *mut VOICE_S) {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut n: libc::c_int = 0;
    s = (*p_voice).sym;
    while !s.is_null() {
        if (*s).type_0 as libc::c_int != 3 as libc::c_int
            || (*s).sflags & 0x10000000 as libc::c_int as libc::c_uint == 0
            || (*s).sflags & 0x400000 as libc::c_int as libc::c_uint != 0
        {
            s = (*s).next;
        } else {
            n = 0 as libc::c_int;
            s2 = 0 as *mut SYMBOL;
            s = (*s).next;
            while !s.is_null() {
                if !((*s).type_0 as libc::c_int != 3 as libc::c_int) {
                    n += 1;
                    n;
                    if (*s).sflags & 0x8000 as libc::c_int as libc::c_uint != 0 {
                        break;
                    }
                    if ((*s).next).is_null() {
                        (*s)
                            .flags = ((*s).flags as libc::c_int | 0x200 as libc::c_int)
                            as libc::c_ushort;
                        (*s).sflags |= 0x8000 as libc::c_int as libc::c_uint;
                        break;
                    } else {
                        if n == cfmt.rbmin {
                            s2 = s;
                        }
                        if n == cfmt.rbmax {
                            if !s2.is_null() {
                                s = s2;
                            }
                            (*s).sflags |= 0x8000 as libc::c_int as libc::c_uint;
                            break;
                        }
                    }
                }
                s = (*s).next;
            }
        }
    }
}
unsafe extern "C" fn set_global() {
    let mut sy: *mut SYSTEM = 0 as *mut SYSTEM;
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut staff: libc::c_int = 0;
    static mut delpit: [libc::c_schar; 4] = [
        0 as libc::c_int as libc::c_schar,
        -(7 as libc::c_int) as libc::c_schar,
        -(14 as libc::c_int) as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
    ];
    sy = cursys;
    staff = (*cursys).nstaff as libc::c_int;
    loop {
        sy = (*sy).next;
        if sy.is_null() {
            break;
        }
        if (*sy).nstaff as libc::c_int > staff {
            staff = (*sy).nstaff as libc::c_int;
        }
    }
    nstaff = staff;
    if cfmt.abc2pscompat != 0 {
        let mut i: libc::c_int = 0;
        p_voice = first_voice;
        while !p_voice.is_null() {
            let mut delta: libc::c_int = 0;
            let mut g: *mut SYMBOL = 0 as *mut SYMBOL;
            if !((*p_voice).octave as libc::c_int != 0 as libc::c_int) {
                i = (*(*p_voice).s_clef).u.clef.type_0 as libc::c_int;
                if !(i == 3 as libc::c_int) {
                    delta = delpit[i as usize] as libc::c_int;
                    s = (*p_voice).sym;
                    while !s.is_null() {
                        match (*s).type_0 as libc::c_int {
                            4 => {
                                i = (*s).u.clef.type_0 as libc::c_int;
                                if (*s).u.clef.check_pitch == 0 {
                                    i = 0 as libc::c_int;
                                }
                                delta = delpit[i as usize] as libc::c_int;
                            }
                            1 => {
                                if !(delta == 0 as libc::c_int) {
                                    if !((*s).abc_type as libc::c_int == 5 as libc::c_int) {
                                        i = (*s).nhd as libc::c_int;
                                        while i >= 0 as libc::c_int {
                                            (*s)
                                                .pits[i
                                                as usize] = ((*s).pits[i as usize] as libc::c_int + delta)
                                                as libc::c_schar;
                                            i -= 1;
                                            i;
                                        }
                                    }
                                }
                            }
                            11 => {
                                if !(delta == 0 as libc::c_int) {
                                    g = (*s).extra;
                                    while !g.is_null() {
                                        if !((*g).type_0 as libc::c_int != 1 as libc::c_int) {
                                            i = (*g).nhd as libc::c_int;
                                            while i >= 0 as libc::c_int {
                                                (*g)
                                                    .pits[i
                                                    as usize] = ((*g).pits[i as usize] as libc::c_int + delta)
                                                    as libc::c_schar;
                                                i -= 1;
                                                i;
                                            }
                                        }
                                        g = (*g).next;
                                    }
                                }
                            }
                            _ => {}
                        }
                        s = (*s).next;
                    }
                }
            }
            p_voice = (*p_voice).next;
        }
    }
    p_voice = first_voice;
    while !p_voice.is_null() {
        set_words(p_voice);
        set_rb(p_voice);
        p_voice = (*p_voice).next;
    }
    set_float();
    set_clefs();
    set_pitch(0 as *mut SYMBOL);
}
unsafe extern "C" fn set_indent() -> libc::c_float {
    let mut staff: libc::c_int = 0;
    let mut voice: libc::c_int = 0;
    let mut w: libc::c_float = 0.;
    let mut maxw: libc::c_float = 0.;
    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    maxw = 0 as libc::c_int as libc::c_float;
    p_voice = first_voice;
    while !p_voice.is_null() {
        voice = p_voice.offset_from(voice_tb.as_mut_ptr()) as libc::c_long
            as libc::c_int;
        if !(((*cursys).voice[voice as usize].range as libc::c_int) < 0 as libc::c_int) {
            staff = (*cursys).voice[voice as usize].staff as libc::c_int;
            if !((*cursys).staff[staff as usize].empty != 0) {
                p = if (*p_voice).new_name() as libc::c_int != 0 {
                    (*p_voice).nm
                } else {
                    (*p_voice).snm
                };
                if !p.is_null() {
                    str_font(VOICEFONT as libc::c_int);
                    loop {
                        q = strstr(p, b"\\n\0" as *const u8 as *const libc::c_char);
                        if !q.is_null() {
                            *q = '\0' as i32 as libc::c_char;
                        }
                        w = tex_str(p);
                        if w > maxw {
                            maxw = w;
                        }
                        if q.is_null() {
                            break;
                        }
                        *q = '\\' as i32 as libc::c_char;
                        p = q.offset(2 as libc::c_int as isize);
                    }
                }
            }
        }
        p_voice = (*p_voice).next;
    }
    if maxw != 0 as libc::c_int as libc::c_float {
        w = 0 as libc::c_int as libc::c_float;
        staff = 0 as libc::c_int;
        while staff <= (*cursys).nstaff as libc::c_int {
            if (*cursys).staff[staff as usize].flags as libc::c_int
                & (0x100 as libc::c_int | 0x400 as libc::c_int) != 0
            {
                w = 20 as libc::c_int as libc::c_float;
                break;
            } else {
                if (*cursys).staff[staff as usize].flags as libc::c_int
                    & (0x1 as libc::c_int | 0x4 as libc::c_int) != 0
                    && w == 0 as libc::c_int as libc::c_float
                {
                    w = 10 as libc::c_int as libc::c_float;
                }
                staff += 1;
                staff;
            }
        }
        maxw
            += 4 as libc::c_int as libc::c_float * cwid(' ' as i32 as libc::c_uchar)
                * cfmt.font_tb[VOICEFONT as libc::c_int as usize].swfac + w;
    }
    if insert_meter & 2 as libc::c_int != 0 {
        maxw += cfmt.indent;
    }
    return maxw;
}
unsafe extern "C" fn set_beams(mut sym: *mut SYMBOL) {
    let mut sy: *mut SYSTEM = 0 as *mut SYSTEM;
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut t: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut g: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s_opp: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut n: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut beam: libc::c_int = 0;
    let mut laststem: libc::c_int = 0;
    let mut mid_p: libc::c_int = 0;
    beam = 0 as libc::c_int;
    laststem = -(1 as libc::c_int);
    s_opp = 0 as *mut SYMBOL;
    sy = cursys;
    let mut current_block_76: u64;
    s = sym;
    while !s.is_null() {
        if (*s).abc_type as libc::c_int != 4 as libc::c_int {
            match (*s).type_0 as libc::c_int {
                8 => {
                    current_block_76 = 1103207446816333396;
                    match current_block_76 {
                        1103207446816333396 => {
                            sy = (*sy).next;
                        }
                        _ => {
                            g = (*s).extra;
                            while (*g).abc_type as libc::c_int != 4 as libc::c_int {
                                g = (*g).next;
                            }
                            if (*g).stem as libc::c_int == 2 as libc::c_int {
                                s_opp = s;
                            } else {
                                if (*s).stem as libc::c_int == 0 as libc::c_int
                                    && {
                                        (*s).stem = (*s).multi;
                                        (*s).stem as libc::c_int == 0 as libc::c_int
                                    }
                                {
                                    (*s).stem = 1 as libc::c_int as libc::c_schar;
                                }
                                while !g.is_null() {
                                    (*g).stem = (*s).stem;
                                    (*g).multi = (*s).multi;
                                    g = (*g).next;
                                }
                            }
                        }
                    }
                }
                11 => {
                    current_block_76 = 13183875560443969876;
                    match current_block_76 {
                        1103207446816333396 => {
                            sy = (*sy).next;
                        }
                        _ => {
                            g = (*s).extra;
                            while (*g).abc_type as libc::c_int != 4 as libc::c_int {
                                g = (*g).next;
                            }
                            if (*g).stem as libc::c_int == 2 as libc::c_int {
                                s_opp = s;
                            } else {
                                if (*s).stem as libc::c_int == 0 as libc::c_int
                                    && {
                                        (*s).stem = (*s).multi;
                                        (*s).stem as libc::c_int == 0 as libc::c_int
                                    }
                                {
                                    (*s).stem = 1 as libc::c_int as libc::c_schar;
                                }
                                while !g.is_null() {
                                    (*g).stem = (*s).stem;
                                    (*g).multi = (*s).multi;
                                    g = (*g).next;
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        } else {
            mid_p = (*s).mid as libc::c_int / 3 as libc::c_int + 18 as libc::c_int;
            if (*s).stem as libc::c_int == 0 as libc::c_int
                && {
                    (*s).stem = (*s).multi;
                    (*s).stem as libc::c_int == 0 as libc::c_int
                }
            {
                if beam != 0 {
                    (*s).stem = laststem as libc::c_schar;
                } else if (*s).sflags
                    & (0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
                    == 0x2 as libc::c_int as libc::c_uint
                {
                    let mut pu: libc::c_int = (*s).pits[(*s).nhd as usize]
                        as libc::c_int;
                    let mut pd: libc::c_int = (*s).pits[0 as libc::c_int as usize]
                        as libc::c_int;
                    beam = 1 as libc::c_int;
                    t = (*s).next;
                    while !t.is_null() {
                        if !((*t).abc_type as libc::c_int != 4 as libc::c_int) {
                            if (*t).stem as libc::c_int != 0
                                || (*t).multi as libc::c_int != 0
                            {
                                (*s)
                                    .stem = (if (*t).multi as libc::c_int != 0 {
                                    (*t).multi as libc::c_int
                                } else {
                                    (*t).stem as libc::c_int
                                }) as libc::c_schar;
                                break;
                            } else {
                                if (*t).pits[(*t).nhd as usize] as libc::c_int > pu {
                                    pu = (*t).pits[(*t).nhd as usize] as libc::c_int;
                                }
                                if ((*t).pits[0 as libc::c_int as usize] as libc::c_int)
                                    < pd
                                {
                                    pd = (*t).pits[0 as libc::c_int as usize] as libc::c_int;
                                }
                                if (*t).sflags & 0x10 as libc::c_int as libc::c_uint != 0 {
                                    break;
                                }
                            }
                        }
                        t = (*t).next;
                    }
                    if (*t).sflags & 0x10 as libc::c_int as libc::c_uint != 0 {
                        mid_p *= 2 as libc::c_int;
                        if pu + pd < mid_p {
                            (*s).stem = 1 as libc::c_int as libc::c_schar;
                        } else if pu + pd > mid_p {
                            (*s).stem = -(1 as libc::c_int) as libc::c_schar;
                        } else if cfmt.bstemdown != 0 {
                            (*s).stem = -(1 as libc::c_int) as libc::c_schar;
                        }
                    }
                    if (*s).stem == 0 {
                        (*s).stem = laststem as libc::c_schar;
                    }
                } else {
                    n = (*s).pits[(*s).nhd as usize] as libc::c_int
                        + (*s).pits[0 as libc::c_int as usize] as libc::c_int;
                    if n == mid_p * 2 as libc::c_int {
                        n = 0 as libc::c_int;
                        m = 0 as libc::c_int;
                        while m <= (*s).nhd as libc::c_int {
                            n += (*s).pits[m as usize] as libc::c_int;
                            m += 1;
                            m;
                        }
                        mid_p *= (*s).nhd as libc::c_int + 1 as libc::c_int;
                    } else {
                        mid_p *= 2 as libc::c_int;
                    }
                    if n < mid_p {
                        (*s).stem = 1 as libc::c_int as libc::c_schar;
                    } else if n > mid_p {
                        (*s).stem = -(1 as libc::c_int) as libc::c_schar;
                    } else if cfmt.bstemdown != 0 {
                        (*s).stem = -(1 as libc::c_int) as libc::c_schar;
                    } else {
                        (*s).stem = laststem as libc::c_schar;
                    }
                }
            } else if (*s).sflags
                & (0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
                == 0x2 as libc::c_int as libc::c_uint
            {
                beam = 1 as libc::c_int;
            }
            if (*s).sflags & 0x10 as libc::c_int as libc::c_uint != 0 {
                beam = 0 as libc::c_int;
            }
            laststem = (*s).stem as libc::c_int;
            if !s_opp.is_null() {
                g = (*s_opp).extra;
                while !g.is_null() {
                    (*g).stem = -laststem as libc::c_schar;
                    g = (*g).next;
                }
                (*s_opp).stem = -laststem as libc::c_schar;
                s_opp = 0 as *mut SYMBOL;
            }
        }
        s = (*s).next;
    }
}
unsafe extern "C" fn same_head(mut s1: *mut SYMBOL, mut s2: *mut SYMBOL) -> libc::c_int {
    let mut current_block: u64;
    let mut i1: libc::c_int = 0;
    let mut i2: libc::c_int = 0;
    let mut l1: libc::c_int = 0;
    let mut l2: libc::c_int = 0;
    let mut i11: libc::c_int = 0;
    let mut i12: libc::c_int = 0;
    let mut i21: libc::c_int = 0;
    let mut i22: libc::c_int = 0;
    let mut sh1: libc::c_float = 0.;
    let mut sh2: libc::c_float = 0.;
    if (*s1).sflags
        & (0x2000000 as libc::c_int | 0x4000000 as libc::c_int) as libc::c_uint
        == (0x2000000 as libc::c_int | 0x4000000 as libc::c_int) as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    l1 = (*s1).dur;
    if l1 >= 1536 as libc::c_int {
        return 0 as libc::c_int;
    }
    l2 = (*s2).dur;
    if l2 >= 1536 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*s1).flags as libc::c_int & (*s2).flags as libc::c_int & 0x8 as libc::c_int != 0
    {
        return 0 as libc::c_int;
    }
    if (*s1).dots as libc::c_int != (*s2).dots as libc::c_int {
        if (*s1).sflags & 0x2000000 as libc::c_int as libc::c_uint != 0
            || (*s1).dots as libc::c_int * (*s2).dots as libc::c_int != 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
    }
    if (*s1).stem as libc::c_int * (*s2).stem as libc::c_int > 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    i2 = 0 as libc::c_int;
    i1 = i2;
    if (*s1).pits[0 as libc::c_int as usize] as libc::c_int
        > (*s2).pits[0 as libc::c_int as usize] as libc::c_int
    {
        if ((*s1).stem as libc::c_int) < 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        while (*s2).pits[i2 as usize] as libc::c_int
            != (*s1).pits[0 as libc::c_int as usize] as libc::c_int
        {
            i2 += 1;
            if i2 > (*s2).nhd as libc::c_int {
                return 0 as libc::c_int;
            }
        }
    } else if ((*s1).pits[0 as libc::c_int as usize] as libc::c_int)
        < (*s2).pits[0 as libc::c_int as usize] as libc::c_int
    {
        if ((*s2).stem as libc::c_int) < 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        while (*s2).pits[0 as libc::c_int as usize] as libc::c_int
            != (*s1).pits[i1 as usize] as libc::c_int
        {
            i1 += 1;
            if i1 > (*s1).nhd as libc::c_int {
                return 0 as libc::c_int;
            }
        }
    }
    if (*s2).u.note.notes[i2 as usize].acc as libc::c_int
        != (*s1).u.note.notes[i1 as usize].acc as libc::c_int
    {
        return 0 as libc::c_int;
    }
    i11 = i1;
    i21 = i2;
    sh1 = (*s1).u.note.notes[i1 as usize].shhd;
    sh2 = (*s2).u.note.notes[i2 as usize].shhd;
    loop {
        i1 += 1;
        i1;
        i2 += 1;
        i2;
        if i1 > (*s1).nhd as libc::c_int {
            break;
        } else {
            if i2 > (*s2).nhd as libc::c_int {
                break;
            }
            if (*s2).u.note.notes[i2 as usize].acc as libc::c_int
                != (*s1).u.note.notes[i1 as usize].acc as libc::c_int
            {
                return 0 as libc::c_int;
            }
            if sh1 < (*s1).u.note.notes[i1 as usize].shhd {
                sh1 = (*s1).u.note.notes[i1 as usize].shhd;
            }
            if sh2 < (*s2).u.note.notes[i2 as usize].shhd {
                sh2 = (*s2).u.note.notes[i2 as usize].shhd;
            }
            if !((*s2).pits[i2 as usize] as libc::c_int
                == (*s1).pits[i1 as usize] as libc::c_int)
            {
                break;
            }
        }
    }
    if i1 <= (*s1).nhd as libc::c_int {
        if i2 <= (*s2).nhd as libc::c_int {
            return 0 as libc::c_int;
        }
        if (*s2).stem as libc::c_int > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    } else if i2 <= (*s2).nhd as libc::c_int {
        if (*s1).stem as libc::c_int > 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    i12 = i1;
    i22 = i2;
    if l1 == l2 {
        current_block = 10373051536174908719;
    } else {
        if l1 < l2 {
            l1 = l2;
            l2 = (*s1).dur;
        }
        if l1 < 1536 as libc::c_int / 2 as libc::c_int {
            if (*s2).dots as libc::c_int > 0 as libc::c_int {
                current_block = 8669339788170803034;
            } else if (*s1).dots as libc::c_int > 0 as libc::c_int {
                current_block = 3998676797473096740;
            } else {
                current_block = 10373051536174908719;
            }
        } else if l2 < 1536 as libc::c_int / 4 as libc::c_int {
            if (*s1).sflags & 0x4000000 as libc::c_int as libc::c_uint != 0 {
                return 0 as libc::c_int;
            }
            if (*s2).dur >= 1536 as libc::c_int / 2 as libc::c_int {
                current_block = 8669339788170803034;
            } else {
                current_block = 3998676797473096740;
            }
        } else {
            return 0 as libc::c_int
        }
    }
    match current_block {
        10373051536174908719 => {
            if voice_tb[(*s1).voice as usize].scale
                < voice_tb[(*s2).voice as usize].scale
            {
                current_block = 8669339788170803034;
            } else {
                current_block = 3998676797473096740;
            }
        }
        _ => {}
    }
    match current_block {
        8669339788170803034 => {
            i1 = i11;
            while i1 < i12 {
                (*s1)
                    .u
                    .note
                    .notes[i1 as usize]
                    .invisible = 1 as libc::c_int as libc::c_char;
                (*s1).u.note.notes[i1 as usize].acc = 0 as libc::c_int as libc::c_uchar;
                i1 += 1;
                i1;
            }
            i1 = 0 as libc::c_int;
            while i1 <= (*s1).nhd as libc::c_int {
                (*s1).u.note.notes[i1 as usize].shhd += sh2;
                i1 += 1;
                i1;
            }
            return 1 as libc::c_int;
        }
        _ => {
            i2 = i21;
            while i2 < i22 {
                (*s2)
                    .u
                    .note
                    .notes[i2 as usize]
                    .invisible = 1 as libc::c_int as libc::c_char;
                (*s2).u.note.notes[i2 as usize].acc = 0 as libc::c_int as libc::c_uchar;
                i2 += 1;
                i2;
            }
            i2 = 0 as libc::c_int;
            while i2 <= (*s2).nhd as libc::c_int {
                (*s2).u.note.notes[i2 as usize].shhd += sh1;
                i2 += 1;
                i2;
            }
            return 1 as libc::c_int;
        }
    };
}
static mut w_note: [libc::c_float; 4] = [
    3.5f64 as libc::c_float,
    3.7f64 as libc::c_float,
    5 as libc::c_int as libc::c_float,
    7 as libc::c_int as libc::c_float,
];
unsafe extern "C" fn unison_acc(
    mut s1: *mut SYMBOL,
    mut s2: *mut SYMBOL,
    mut i1: libc::c_int,
    mut i2: libc::c_int,
) {
    let mut m: libc::c_int = 0;
    let mut d: libc::c_float = 0.;
    if (*s2).u.note.notes[i2 as usize].acc as libc::c_int == 0 as libc::c_int {
        d = w_note[(*s2).head as usize] * 2 as libc::c_int as libc::c_float + (*s2).xmx
            + (*s1).u.note.notes[i1 as usize].shac + 2 as libc::c_int as libc::c_float;
        if (*s1).u.note.notes[i1 as usize].acc as libc::c_int & 0xf8 as libc::c_int != 0
        {
            d += 2 as libc::c_int as libc::c_float;
        }
        if (*s2).dots != 0 {
            d += 6 as libc::c_int as libc::c_float;
        }
        m = 0 as libc::c_int;
        while m <= (*s1).nhd as libc::c_int {
            (*s1).u.note.notes[m as usize].shhd += d;
            (*s1).u.note.notes[m as usize].shac -= d;
            m += 1;
            m;
        }
        (*s1).xmx += d;
    } else {
        d = w_note[(*s1).head as usize] * 2 as libc::c_int as libc::c_float + (*s1).xmx
            + (*s2).u.note.notes[i2 as usize].shac + 2 as libc::c_int as libc::c_float;
        if (*s2).u.note.notes[i2 as usize].acc as libc::c_int & 0xf8 as libc::c_int != 0
        {
            d += 2 as libc::c_int as libc::c_float;
        }
        if (*s1).dots != 0 {
            d += 6 as libc::c_int as libc::c_float;
        }
        m = 0 as libc::c_int;
        while m <= (*s2).nhd as libc::c_int {
            (*s2).u.note.notes[m as usize].shhd += d;
            (*s2).u.note.notes[m as usize].shac -= d;
            m += 1;
            m;
        }
        (*s2).xmx += d;
    };
}
unsafe extern "C" fn set_left(mut s: *mut SYMBOL, mut left: *mut libc::c_float) {
    let mut m: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut w_base: libc::c_float = 0.;
    let mut w: libc::c_float = 0.;
    let mut shift: libc::c_float = 0.;
    i = 0 as libc::c_int;
    while i < 48 as libc::c_int * 2 as libc::c_int {
        *left.offset(i as isize) = -(100 as libc::c_int) as libc::c_float;
        i += 1;
        i;
    }
    w_base = w_note[(*s).head as usize];
    w = w_base;
    if (*s).nflags as libc::c_int > -(2 as libc::c_int) {
        if (*s).stem as libc::c_int > 0 as libc::c_int {
            w = -w;
            i = (*s).pits[0 as libc::c_int as usize] as libc::c_int * 2 as libc::c_int;
            j = (((*s).ymx as libc::c_int - 2 as libc::c_int) / 3 as libc::c_int
                + 18 as libc::c_int) * 2 as libc::c_int;
        } else {
            i = (((*s).ymn as libc::c_int + 2 as libc::c_int) / 3 as libc::c_int
                + 18 as libc::c_int) * 2 as libc::c_int;
            j = (*s).pits[(*s).nhd as usize] as libc::c_int * 2 as libc::c_int;
        }
        if i < 0 as libc::c_int {
            i = 0 as libc::c_int;
        }
        while i < 48 as libc::c_int * 2 as libc::c_int && i <= j {
            *left.offset(i as isize) = w;
            i += 1;
            i;
        }
    }
    if (*s).stem as libc::c_int > 0 as libc::c_int {
        shift = (*s).u.note.notes[0 as libc::c_int as usize].shhd;
    } else {
        shift = (*s).u.note.notes[(*s).nhd as usize].shhd;
    }
    m = 0 as libc::c_int;
    while m <= (*s).nhd as libc::c_int {
        w = -(*s).u.note.notes[m as usize].shhd + w_base + shift;
        i = (*s).pits[m as usize] as libc::c_int * 2 as libc::c_int;
        if i < 0 as libc::c_int {
            i = 0 as libc::c_int;
        } else if i >= 48 as libc::c_int * 2 as libc::c_int - 1 as libc::c_int {
            i = 48 as libc::c_int * 2 as libc::c_int - 2 as libc::c_int;
        }
        if w > *left.offset(i as isize) {
            *left.offset(i as isize) = w;
        }
        if (*s).head as libc::c_int != 3 as libc::c_int {
            w -= 1 as libc::c_int as libc::c_float;
        }
        if w > *left.offset((i - 1 as libc::c_int) as isize) {
            *left.offset((i - 1 as libc::c_int) as isize) = w;
        }
        if w > *left.offset((i + 1 as libc::c_int) as isize) {
            *left.offset((i + 1 as libc::c_int) as isize) = w;
        }
        m += 1;
        m;
    }
}
unsafe extern "C" fn set_right(mut s: *mut SYMBOL, mut right: *mut libc::c_float) {
    let mut m: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut w_base: libc::c_float = 0.;
    let mut w: libc::c_float = 0.;
    let mut shift: libc::c_float = 0.;
    i = 0 as libc::c_int;
    while i < 48 as libc::c_int * 2 as libc::c_int {
        *right.offset(i as isize) = -(100 as libc::c_int) as libc::c_float;
        i += 1;
        i;
    }
    flags = ((*s).nflags as libc::c_int > 0 as libc::c_int
        && (*s).sflags & (0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
            == (0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint)
        as libc::c_int;
    w_base = w_note[(*s).head as usize];
    w = w_base;
    if (*s).nflags as libc::c_int > -(2 as libc::c_int) {
        if ((*s).stem as libc::c_int) < 0 as libc::c_int {
            w = -w;
            i = (((*s).ymn as libc::c_int + 2 as libc::c_int) / 3 as libc::c_int
                + 18 as libc::c_int) * 2 as libc::c_int;
            j = (*s).pits[(*s).nhd as usize] as libc::c_int * 2 as libc::c_int;
            k = i + 4 as libc::c_int;
        } else {
            i = (*s).pits[0 as libc::c_int as usize] as libc::c_int * 2 as libc::c_int;
            j = (((*s).ymx as libc::c_int - 2 as libc::c_int) / 3 as libc::c_int
                + 18 as libc::c_int) * 2 as libc::c_int;
            k = i;
        }
        if i < 0 as libc::c_int {
            i = 0 as libc::c_int;
        }
        while i < 48 as libc::c_int * 2 as libc::c_int && i < j {
            *right.offset(i as isize) = w;
            i += 1;
            i;
        }
    }
    if flags != 0 {
        if (*s).stem as libc::c_int > 0 as libc::c_int {
            if (*s).xmx == 0 as libc::c_int as libc::c_float {
                i = (*s).pits[(*s).nhd as usize] as libc::c_int * 2 as libc::c_int;
            } else {
                i = (*s).pits[0 as libc::c_int as usize] as libc::c_int
                    * 2 as libc::c_int;
            }
            i += 4 as libc::c_int;
            if i < 0 as libc::c_int {
                i = 0 as libc::c_int;
            }
            while i < 48 as libc::c_int * 2 as libc::c_int && i <= j - 4 as libc::c_int {
                *right.offset(i as isize) = 11 as libc::c_int as libc::c_float;
                i += 1;
                i;
            }
        } else {
            i = k;
            if i < 0 as libc::c_int {
                i = 0 as libc::c_int;
            }
            while i < 48 as libc::c_int * 2 as libc::c_int
                && i
                    <= (*s).pits[0 as libc::c_int as usize] as libc::c_int
                        * 2 as libc::c_int - 4 as libc::c_int
            {
                *right.offset(i as isize) = 3.5f64 as libc::c_float;
                i += 1;
                i;
            }
        }
    }
    if (*s).stem as libc::c_int > 0 as libc::c_int {
        shift = (*s).u.note.notes[0 as libc::c_int as usize].shhd;
    } else {
        shift = (*s).u.note.notes[(*s).nhd as usize].shhd;
    }
    m = 0 as libc::c_int;
    while m <= (*s).nhd as libc::c_int {
        w = (*s).u.note.notes[m as usize].shhd + w_base - shift;
        i = (*s).pits[m as usize] as libc::c_int * 2 as libc::c_int;
        if i < 0 as libc::c_int {
            i = 0 as libc::c_int;
        } else if i >= 48 as libc::c_int * 2 as libc::c_int - 1 as libc::c_int {
            i = 48 as libc::c_int * 2 as libc::c_int - 2 as libc::c_int;
        }
        if w > *right.offset(i as isize) {
            *right.offset(i as isize) = w;
        }
        if (*s).head as libc::c_int != 3 as libc::c_int {
            w -= 1 as libc::c_int as libc::c_float;
        }
        if w > *right.offset((i - 1 as libc::c_int) as isize) {
            *right.offset((i - 1 as libc::c_int) as isize) = w;
        }
        if w > *right.offset((i + 1 as libc::c_int) as isize) {
            *right.offset((i + 1 as libc::c_int) as isize) = w;
        }
        m += 1;
        m;
    }
}
unsafe extern "C" fn set_overlap() {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s1: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s3: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut i: libc::c_int = 0;
    let mut i1: libc::c_int = 0;
    let mut i2: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut sd: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut dp: libc::c_int = 0;
    let mut d: libc::c_float = 0.;
    let mut d2: libc::c_float = 0.;
    let mut dr: libc::c_float = 0.;
    let mut dr2: libc::c_float = 0.;
    let mut dx: libc::c_float = 0.;
    let mut left1: [libc::c_float; 96] = [0.; 96];
    let mut right1: [libc::c_float; 96] = [0.; 96];
    let mut left2: [libc::c_float; 96] = [0.; 96];
    let mut right2: [libc::c_float; 96] = [0.; 96];
    let mut right3: [libc::c_float; 96] = [0.; 96];
    let mut pl: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut pr: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut current_block_168: u64;
    s = tsfirst;
    while !s.is_null() {
        if !((*s).abc_type as libc::c_int != 4 as libc::c_int
            || (*s).flags as libc::c_int & 0x2 as libc::c_int != 0)
        {
            if (*s).sflags & 0x200 as libc::c_int as libc::c_uint != 0
                && ((*(*s).ts_prev).stem as libc::c_int) < 0 as libc::c_int
            {
                s2 = (*s).ts_prev;
                m = 0 as libc::c_int;
                while m <= (*s2).nhd as libc::c_int {
                    (*s2)
                        .u
                        .note
                        .notes[m as usize]
                        .shhd = ((*s2).u.note.notes[m as usize].shhd as libc::c_double
                        + 3.5f64 * 2 as libc::c_int as libc::c_double) as libc::c_float;
                    (*s2)
                        .u
                        .note
                        .notes[m as usize]
                        .shac = ((*s2).u.note.notes[m as usize].shac as libc::c_double
                        - 3.5f64 * 2 as libc::c_int as libc::c_double) as libc::c_float;
                    m += 1;
                    m;
                }
                (*s2)
                    .xmx = ((*s2).xmx as libc::c_double
                    + 3.5f64 * 2 as libc::c_int as libc::c_double) as libc::c_float;
            }
            s2 = s;
            loop {
                s2 = (*s2).ts_next;
                if s2.is_null() {
                    break;
                }
                if (*s2).time != (*s).time {
                    s2 = 0 as *mut SYMBOL;
                    break;
                } else if (*s2).abc_type as libc::c_int == 4 as libc::c_int
                    && (*s2).flags as libc::c_int & 0x2 as libc::c_int == 0
                    && (*s2).staff as libc::c_int == (*s).staff as libc::c_int
                {
                    break;
                }
            }
            if !s2.is_null() {
                s1 = s;
                if ((*cursys).voice[(*s1).voice as usize].range as libc::c_int)
                    < (*cursys).voice[(*s2).voice as usize].range as libc::c_int
                {
                    (*s2).doty = -(3 as libc::c_int) as libc::c_schar;
                } else {
                    (*s1).doty = -(3 as libc::c_int) as libc::c_schar;
                }
                if !((*s1).ymn as libc::c_int > (*s2).ymx as libc::c_int
                    || ((*s1).ymx as libc::c_int) < (*s2).ymn as libc::c_int)
                {
                    if !(same_head(s1, s2) != 0) {
                        set_right(s1, right1.as_mut_ptr());
                        set_left(s2, left2.as_mut_ptr());
                        s3 = (*s1).ts_prev;
                        if !s3.is_null() && (*s3).time == (*s1).time
                            && (*s3).staff as libc::c_int == (*s1).staff as libc::c_int
                            && (*s3).abc_type as libc::c_int == 4 as libc::c_int
                            && (*s3).flags as libc::c_int & 0x2 as libc::c_int == 0
                        {
                            set_right(s3, right3.as_mut_ptr());
                            i = 0 as libc::c_int;
                            while i < 48 as libc::c_int * 2 as libc::c_int {
                                if right3[i as usize] > right1[i as usize] {
                                    right1[i as usize] = right3[i as usize];
                                }
                                i += 1;
                                i;
                            }
                        } else {
                            s3 = 0 as *mut SYMBOL;
                        }
                        d = -(100 as libc::c_int) as libc::c_float;
                        i = 0 as libc::c_int;
                        while i < 48 as libc::c_int * 2 as libc::c_int {
                            if left2[i as usize] + right1[i as usize] > d {
                                d = left2[i as usize] + right1[i as usize];
                            }
                            i += 1;
                            i;
                        }
                        if d < -(3 as libc::c_int) as libc::c_float {
                            if (*s1).dots == 0 || (*s2).dots == 0
                                || (*s2).doty as libc::c_int >= 0 as libc::c_int
                                || (*s1).stem as libc::c_int > 0 as libc::c_int
                                || ((*s2).stem as libc::c_int) < 0 as libc::c_int
                                || (*s1).pits[(*s1).nhd as usize] as libc::c_int
                                    + 2 as libc::c_int
                                    != (*s2).pits[0 as libc::c_int as usize] as libc::c_int
                                || (*s2).pits[0 as libc::c_int as usize] as libc::c_int
                                    & 1 as libc::c_int != 0
                            {
                                current_block_168 = 14916268686031723178;
                            } else {
                                current_block_168 = 2543120759711851213;
                            }
                        } else {
                            current_block_168 = 2543120759711851213;
                        }
                        match current_block_168 {
                            14916268686031723178 => {}
                            _ => {
                                set_right(s2, right2.as_mut_ptr());
                                set_left(s1, left1.as_mut_ptr());
                                if !s3.is_null() {
                                    set_left(s3, right3.as_mut_ptr());
                                    i = 0 as libc::c_int;
                                    while i < 48 as libc::c_int * 2 as libc::c_int {
                                        if right3[i as usize] > left1[i as usize] {
                                            left1[i as usize] = right3[i as usize];
                                        }
                                        i += 1;
                                        i;
                                    }
                                }
                                dr2 = -(100 as libc::c_int) as libc::c_float;
                                dr = dr2;
                                d2 = dr;
                                i = 0 as libc::c_int;
                                while i < 48 as libc::c_int * 2 as libc::c_int {
                                    if left1[i as usize] + right2[i as usize] > d2 {
                                        d2 = left1[i as usize] + right2[i as usize];
                                    }
                                    if right1[i as usize] > dr {
                                        dr = right1[i as usize];
                                    }
                                    if right2[i as usize] > dr2 {
                                        dr2 = right2[i as usize];
                                    }
                                    i += 1;
                                    i;
                                }
                                t = 0 as libc::c_int;
                                i1 = (*s1).nhd as libc::c_int;
                                i2 = (*s2).nhd as libc::c_int;
                                loop {
                                    dp = (*s1).pits[i1 as usize] as libc::c_int
                                        - (*s2).pits[i2 as usize] as libc::c_int;
                                    match dp {
                                        0 => {
                                            if (*s1).u.note.notes[i1 as usize].acc as libc::c_int
                                                != (*s2).u.note.notes[i2 as usize].acc as libc::c_int
                                            {
                                                t = -(1 as libc::c_int);
                                            } else {
                                                if (*s2).u.note.notes[i2 as usize].acc != 0 {
                                                    (*s2)
                                                        .u
                                                        .note
                                                        .notes[i2 as usize]
                                                        .acc = 0 as libc::c_int as libc::c_uchar;
                                                }
                                                if (*s1).dots as libc::c_int != 0
                                                    && (*s2).dots as libc::c_int != 0
                                                    && (*s1).pits[i1 as usize] as libc::c_int & 1 as libc::c_int
                                                        != 0
                                                {
                                                    t = 1 as libc::c_int;
                                                }
                                            }
                                        }
                                        -1 => {
                                            if (*s1).dots as libc::c_int != 0
                                                && (*s2).dots as libc::c_int != 0
                                            {
                                                if (*s1).pits[i1 as usize] as libc::c_int & 1 as libc::c_int
                                                    != 0
                                                {
                                                    (*s1).doty = 0 as libc::c_int as libc::c_schar;
                                                    (*s2).doty = 0 as libc::c_int as libc::c_schar;
                                                } else {
                                                    (*s1).doty = -(3 as libc::c_int) as libc::c_schar;
                                                    (*s2).doty = -(3 as libc::c_int) as libc::c_schar;
                                                }
                                            }
                                        }
                                        -2 => {
                                            if (*s1).dots as libc::c_int != 0
                                                && (*s2).dots as libc::c_int != 0
                                                && (*s1).pits[i1 as usize] as libc::c_int & 1 as libc::c_int
                                                    == 0
                                            {
                                                (*s1).doty = 0 as libc::c_int as libc::c_schar;
                                                (*s2).doty = 0 as libc::c_int as libc::c_schar;
                                            }
                                        }
                                        _ => {}
                                    }
                                    if t < 0 as libc::c_int {
                                        break;
                                    }
                                    if dp >= 0 as libc::c_int {
                                        i1 -= 1;
                                        if i1 < 0 as libc::c_int {
                                            break;
                                        }
                                    }
                                    if !(dp <= 0 as libc::c_int) {
                                        continue;
                                    }
                                    i2 -= 1;
                                    if i2 < 0 as libc::c_int {
                                        break;
                                    }
                                }
                                if t < 0 as libc::c_int {
                                    unison_acc(s1, s2, i1, i2);
                                } else {
                                    sd = 0 as libc::c_int;
                                    pl = left2.as_mut_ptr();
                                    pr = right2.as_mut_ptr();
                                    if (*s1).dots != 0 {
                                        if (*s2).dots != 0 {
                                            if t == 0 {
                                                sd = 1 as libc::c_int;
                                            }
                                        }
                                    } else if (*s2).dots != 0 {
                                        if d2 + dr < d + dr2 {
                                            sd = 1 as libc::c_int;
                                        }
                                    }
                                    if s3.is_null() && d2 + dr < d + dr2 {
                                        s1 = s2;
                                        s2 = s;
                                        d = d2;
                                        pl = left1.as_mut_ptr();
                                        pr = right1.as_mut_ptr();
                                        dr2 = dr;
                                    }
                                    d += 3 as libc::c_int as libc::c_float;
                                    if d < 0 as libc::c_int as libc::c_float {
                                        d = 0 as libc::c_int as libc::c_float;
                                    }
                                    m = if (*s1).stem as libc::c_int >= 0 as libc::c_int {
                                        0 as libc::c_int
                                    } else {
                                        (*s1).nhd as libc::c_int
                                    };
                                    d += (*s1).u.note.notes[m as usize].shhd;
                                    m = if (*s2).stem as libc::c_int >= 0 as libc::c_int {
                                        0 as libc::c_int
                                    } else {
                                        (*s2).nhd as libc::c_int
                                    };
                                    d -= (*s2).u.note.notes[m as usize].shhd;
                                    if (*s1).dots != 0 {
                                        dx = (7.7f64 + (*s1).xmx as libc::c_double
                                            + 3.5f64 * (*s1).dots as libc::c_int as libc::c_double
                                            - 3.5f64 + 3 as libc::c_int as libc::c_double)
                                            as libc::c_float;
                                        if sd == 0 {
                                            d2 = -(100 as libc::c_int) as libc::c_float;
                                            i1 = 0 as libc::c_int;
                                            while i1 <= (*s1).nhd as libc::c_int {
                                                i = (*s1).pits[i1 as usize] as libc::c_int;
                                                if i & 1 as libc::c_int == 0 {
                                                    if (*s1).doty as libc::c_int >= 0 as libc::c_int {
                                                        i += 1;
                                                        i;
                                                    } else {
                                                        i -= 1;
                                                        i;
                                                    }
                                                }
                                                i *= 2 as libc::c_int;
                                                if i < 1 as libc::c_int {
                                                    i = 1 as libc::c_int;
                                                } else if i
                                                    >= 48 as libc::c_int * 2 as libc::c_int - 1 as libc::c_int
                                                {
                                                    i = 48 as libc::c_int * 2 as libc::c_int - 2 as libc::c_int;
                                                }
                                                if *pl.offset(i as isize) > d2 {
                                                    d2 = *pl.offset(i as isize);
                                                }
                                                if *pl.offset((i - 1 as libc::c_int) as isize)
                                                    + 1 as libc::c_int as libc::c_float > d2
                                                {
                                                    d2 = *pl.offset((i - 1 as libc::c_int) as isize)
                                                        + 1 as libc::c_int as libc::c_float;
                                                }
                                                if *pl.offset((i + 1 as libc::c_int) as isize)
                                                    + 1 as libc::c_int as libc::c_float > d2
                                                {
                                                    d2 = *pl.offset((i + 1 as libc::c_int) as isize)
                                                        + 1 as libc::c_int as libc::c_float;
                                                }
                                                i1 += 1;
                                                i1;
                                            }
                                            if dx + d2 + 2 as libc::c_int as libc::c_float > d {
                                                d = dx + d2 + 2 as libc::c_int as libc::c_float;
                                            }
                                        } else if dx < d + dr2 + (*s2).xmx {
                                            d2 = 0 as libc::c_int as libc::c_float;
                                            i1 = 0 as libc::c_int;
                                            while i1 <= (*s1).nhd as libc::c_int {
                                                i = (*s1).pits[i1 as usize] as libc::c_int;
                                                if i & 1 as libc::c_int == 0 {
                                                    if (*s1).doty as libc::c_int >= 0 as libc::c_int {
                                                        i += 1;
                                                        i;
                                                    } else {
                                                        i -= 1;
                                                        i;
                                                    }
                                                }
                                                i *= 2 as libc::c_int;
                                                if i < 1 as libc::c_int {
                                                    i = 1 as libc::c_int;
                                                } else if i
                                                    >= 48 as libc::c_int * 2 as libc::c_int - 1 as libc::c_int
                                                {
                                                    i = 48 as libc::c_int * 2 as libc::c_int - 2 as libc::c_int;
                                                }
                                                if *pr.offset(i as isize) > d2 {
                                                    d2 = *pr.offset(i as isize);
                                                }
                                                if *pr.offset((i - 1 as libc::c_int) as isize)
                                                    + 1 as libc::c_int as libc::c_float > d2
                                                {
                                                    let ref mut fresh1 = *pr
                                                        .offset((i - 1 as libc::c_int) as isize);
                                                    *fresh1 = 1 as libc::c_int as libc::c_float;
                                                    d2 = *fresh1;
                                                }
                                                if *pr.offset((i + 1 as libc::c_int) as isize)
                                                    + 1 as libc::c_int as libc::c_float > d2
                                                {
                                                    d2 = *pr.offset((i + 1 as libc::c_int) as isize)
                                                        + 1 as libc::c_int as libc::c_float;
                                                }
                                                i1 += 1;
                                                i1;
                                            }
                                            if d2 as libc::c_double > 4.5f64
                                                && (7.7f64 + (*s1).xmx as libc::c_double
                                                    + 2 as libc::c_int as libc::c_double)
                                                    < (d + d2 + (*s2).xmx) as libc::c_double
                                            {
                                                (*s2)
                                                    .xmx = ((d2 + 3 as libc::c_int as libc::c_float)
                                                    as libc::c_double - 7.7f64) as libc::c_float;
                                            }
                                        }
                                    }
                                    m = (*s2).nhd as libc::c_int;
                                    while m >= 0 as libc::c_int {
                                        (*s2).u.note.notes[m as usize].shhd += d;
                                        m -= 1;
                                        m;
                                    }
                                    (*s2).xmx += d;
                                    if sd != 0 {
                                        (*s1).xmx = (*s2).xmx;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        s = (*s).ts_next;
    }
}
unsafe extern "C" fn set_stems() {
    let mut sy: *mut SYSTEM = 0 as *mut SYSTEM;
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut g: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut slen: libc::c_float = 0.;
    let mut scale: libc::c_float = 0.;
    let mut ymn: libc::c_int = 0;
    let mut ymx: libc::c_int = 0;
    let mut nflags: libc::c_int = 0;
    let mut mid: libc::c_int = 0;
    sy = cursys;
    let mut current_block_109: u64;
    s = tsfirst;
    while !s.is_null() {
        if (*s).abc_type as libc::c_int != 4 as libc::c_int {
            let mut ymin: libc::c_int = 0;
            let mut ymax: libc::c_int = 0;
            match (*s).type_0 as libc::c_int {
                8 => {
                    current_block_109 = 11800758854290119221;
                    match current_block_109 {
                        11800758854290119221 => {
                            sy = (*sy).next;
                        }
                        _ => {
                            ymax = (*s).mid as libc::c_int;
                            ymin = ymax;
                            g = (*s).extra;
                            while !g.is_null() {
                                if !((*g).type_0 as libc::c_int != 1 as libc::c_int) {
                                    slen = 15 as libc::c_int as libc::c_float;
                                    if (*g).nflags as libc::c_int > 1 as libc::c_int {
                                        slen = (slen as libc::c_double
                                            + 1.2f64
                                                * ((*g).nflags as libc::c_int - 1 as libc::c_int)
                                                    as libc::c_double) as libc::c_float;
                                    }
                                    ymn = 3 as libc::c_int
                                        * ((*g).pits[0 as libc::c_int as usize] as libc::c_int
                                            - 18 as libc::c_int);
                                    ymx = 3 as libc::c_int
                                        * ((*g).pits[(*g).nhd as usize] as libc::c_int
                                            - 18 as libc::c_int);
                                    if (*s).stem as libc::c_int >= 0 as libc::c_int {
                                        (*g).y = ymn as libc::c_schar;
                                        (*g).ys = ymx as libc::c_float + slen;
                                        ymx = ((*g).ys as libc::c_double + 0.5f64) as libc::c_int;
                                    } else {
                                        (*g).y = ymx as libc::c_schar;
                                        (*g).ys = ymn as libc::c_float - slen;
                                        ymn = ((*g).ys as libc::c_double - 0.5f64) as libc::c_int;
                                    }
                                    ymx += 2 as libc::c_int;
                                    ymn -= 2 as libc::c_int;
                                    if ymn < ymin {
                                        ymin = ymn;
                                    } else if ymx > ymax {
                                        ymax = ymx;
                                    }
                                    (*g).ymx = ymx as libc::c_schar;
                                    (*g).ymn = ymn as libc::c_schar;
                                }
                                g = (*g).next;
                            }
                            (*s).ymx = ymax as libc::c_schar;
                            (*s).ymn = ymin as libc::c_schar;
                        }
                    }
                }
                11 => {
                    current_block_109 = 17216689946888361452;
                    match current_block_109 {
                        11800758854290119221 => {
                            sy = (*sy).next;
                        }
                        _ => {
                            ymax = (*s).mid as libc::c_int;
                            ymin = ymax;
                            g = (*s).extra;
                            while !g.is_null() {
                                if !((*g).type_0 as libc::c_int != 1 as libc::c_int) {
                                    slen = 15 as libc::c_int as libc::c_float;
                                    if (*g).nflags as libc::c_int > 1 as libc::c_int {
                                        slen = (slen as libc::c_double
                                            + 1.2f64
                                                * ((*g).nflags as libc::c_int - 1 as libc::c_int)
                                                    as libc::c_double) as libc::c_float;
                                    }
                                    ymn = 3 as libc::c_int
                                        * ((*g).pits[0 as libc::c_int as usize] as libc::c_int
                                            - 18 as libc::c_int);
                                    ymx = 3 as libc::c_int
                                        * ((*g).pits[(*g).nhd as usize] as libc::c_int
                                            - 18 as libc::c_int);
                                    if (*s).stem as libc::c_int >= 0 as libc::c_int {
                                        (*g).y = ymn as libc::c_schar;
                                        (*g).ys = ymx as libc::c_float + slen;
                                        ymx = ((*g).ys as libc::c_double + 0.5f64) as libc::c_int;
                                    } else {
                                        (*g).y = ymx as libc::c_schar;
                                        (*g).ys = ymn as libc::c_float - slen;
                                        ymn = ((*g).ys as libc::c_double - 0.5f64) as libc::c_int;
                                    }
                                    ymx += 2 as libc::c_int;
                                    ymn -= 2 as libc::c_int;
                                    if ymn < ymin {
                                        ymin = ymn;
                                    } else if ymx > ymax {
                                        ymax = ymx;
                                    }
                                    (*g).ymx = ymx as libc::c_schar;
                                    (*g).ymn = ymn as libc::c_schar;
                                }
                                g = (*g).next;
                            }
                            (*s).ymx = ymax as libc::c_schar;
                            (*s).ymn = ymin as libc::c_schar;
                        }
                    }
                }
                _ => {}
            }
        } else {
            set_head_shift(s);
            nflags = (*s).nflags as libc::c_int;
            if (*s).sflags & (0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
                == 0x2 as libc::c_int as libc::c_uint
            {
                if (*s).sflags & 0x10000 as libc::c_int as libc::c_uint != 0 {
                    (*s).nflags += 1;
                    nflags = (*s).nflags as libc::c_int;
                }
                s2 = (*s).next;
                loop {
                    if (*s2).abc_type as libc::c_int == 4 as libc::c_int {
                        if (*s).sflags & 0x10000 as libc::c_int as libc::c_uint != 0 {
                            (*s2).nflags += 1;
                            (*s2).nflags;
                        }
                        if (*s2).sflags & 0x10 as libc::c_int as libc::c_uint != 0 {
                            break;
                        }
                    }
                    s2 = (*s2).next;
                }
                if (*s2).nflags as libc::c_int > nflags {
                    nflags = (*s2).nflags as libc::c_int;
                }
            } else if (*s).sflags
                & (0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
                == 0x10 as libc::c_int as libc::c_uint
            {
                s2 = (*s).prev;
                while !((*s2).sflags & 0x2 as libc::c_int as libc::c_uint != 0) {
                    s2 = (*s2).prev;
                }
                if (*s2).nflags as libc::c_int > nflags {
                    nflags = (*s2).nflags as libc::c_int;
                }
            }
            mid = (*s).mid as libc::c_int;
            slen = cfmt.stemheight;
            match nflags {
                2 => {
                    slen += 2 as libc::c_int as libc::c_float;
                }
                3 => {
                    slen += 5 as libc::c_int as libc::c_float;
                }
                4 => {
                    slen += 10 as libc::c_int as libc::c_float;
                }
                5 => {
                    slen += 16 as libc::c_int as libc::c_float;
                }
                _ => {}
            }
            scale = voice_tb[(*s).voice as usize].scale;
            if scale != 1 as libc::c_int as libc::c_float {
                slen = (slen as libc::c_double
                    * ((scale + 1 as libc::c_int as libc::c_float) as libc::c_double
                        * 0.5f64)) as libc::c_float;
            }
            ymn = 3 as libc::c_int
                * ((*s).pits[0 as libc::c_int as usize] as libc::c_int
                    - 18 as libc::c_int);
            if (*s).nhd as libc::c_int > 0 as libc::c_int {
                slen -= 2 as libc::c_int as libc::c_float;
                ymx = 3 as libc::c_int
                    * ((*s).pits[(*s).nhd as usize] as libc::c_int - 18 as libc::c_int);
            } else {
                ymx = ymn;
            }
            if (*s).aux as libc::c_int != 0 as libc::c_int {
                slen += (2 as libc::c_int * (*s).aux as libc::c_int) as libc::c_float;
            }
            if (*s).flags as libc::c_int & 0x8 as libc::c_int != 0 {
                if (*s).stem as libc::c_int >= 0 as libc::c_int {
                    (*s).y = ymn as libc::c_schar;
                    (*s).ys = ymx as libc::c_float;
                } else {
                    (*s).ys = ymn as libc::c_float;
                    (*s).y = ymx as libc::c_schar;
                }
                if nflags == -(4 as libc::c_int) {
                    ymn -= 6 as libc::c_int;
                }
                (*s).ymx = (ymx + 4 as libc::c_int) as libc::c_schar;
                (*s).ymn = (ymn - 4 as libc::c_int) as libc::c_schar;
            } else if (*s).stem as libc::c_int >= 0 as libc::c_int {
                if nflags >= 2 as libc::c_int {
                    slen -= 1 as libc::c_int as libc::c_float;
                }
                if (*s).pits[(*s).nhd as usize] as libc::c_int > 26 as libc::c_int
                    && (nflags <= 0 as libc::c_int
                        || (*s).sflags
                            & (0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
                            != (0x2 as libc::c_int | 0x10 as libc::c_int)
                                as libc::c_uint)
                {
                    slen -= 2 as libc::c_int as libc::c_float;
                    if (*s).pits[(*s).nhd as usize] as libc::c_int > 28 as libc::c_int {
                        slen -= 2 as libc::c_int as libc::c_float;
                    }
                }
                (*s).y = ymn as libc::c_schar;
                if (*s).u.note.notes[0 as libc::c_int as usize].ti1 as libc::c_int
                    != 0 as libc::c_int
                {
                    ymn -= 3 as libc::c_int;
                }
                (*s).ymn = (ymn - 4 as libc::c_int) as libc::c_schar;
                (*s).ys = ymx as libc::c_float + slen;
                if (*s).ys < mid as libc::c_float {
                    (*s).ys = mid as libc::c_float;
                }
                (*s)
                    .ymx = ((*s).ys as libc::c_double + 2.5f64) as libc::c_int
                    as libc::c_schar;
            } else {
                if ((*s).pits[0 as libc::c_int as usize] as libc::c_int)
                    < 18 as libc::c_int
                    && (nflags <= 0 as libc::c_int
                        || (*s).sflags
                            & (0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
                            != (0x2 as libc::c_int | 0x10 as libc::c_int)
                                as libc::c_uint)
                {
                    slen -= 2 as libc::c_int as libc::c_float;
                    if ((*s).pits[0 as libc::c_int as usize] as libc::c_int)
                        < 16 as libc::c_int
                    {
                        slen -= 2 as libc::c_int as libc::c_float;
                    }
                }
                (*s).ys = ymn as libc::c_float - slen;
                if (*s).ys > mid as libc::c_float {
                    (*s).ys = mid as libc::c_float;
                }
                (*s)
                    .ymn = ((*s).ys as libc::c_double - 2.5f64) as libc::c_int
                    as libc::c_schar;
                (*s).y = ymx as libc::c_schar;
                if (*s).u.note.notes[(*s).nhd as usize].ti1 as libc::c_int
                    != 0 as libc::c_int
                {
                    ymx += 3 as libc::c_int;
                }
                (*s).ymx = (ymx + 4 as libc::c_int) as libc::c_schar;
            }
        }
        s = (*s).ts_next;
    }
}
unsafe extern "C" fn check_bar(mut s: *mut SYMBOL) {
    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut bar_type: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    p_voice = &mut *voice_tb.as_mut_ptr().offset((*s).voice as isize) as *mut VOICE_S;
    while (*s).type_0 as libc::c_int == 4 as libc::c_int
        || (*s).type_0 as libc::c_int == 6 as libc::c_int
        || (*s).type_0 as libc::c_int == 5 as libc::c_int
    {
        if (*s).type_0 as libc::c_int == 5 as libc::c_int
            && (*s).time > (*(*p_voice).sym).time
        {
            insert_meter |= 1 as libc::c_int;
        }
        s = (*s).prev;
        if s.is_null() {
            return;
        }
    }
    if (*s).type_0 as libc::c_int != 3 as libc::c_int {
        return;
    }
    if (*s).u.bar.repeat_bar != 0 {
        (*p_voice).bar_start = 2 as libc::c_int as libc::c_short;
        (*p_voice).bar_text = (*s).text;
        (*p_voice).bar_gch = (*s).gch;
        (*p_voice).set_bar_repeat(1 as libc::c_int as libc::c_uint);
        (*s).text = 0 as *mut libc::c_char;
        (*s).gch = 0 as *mut gch;
        (*s).u.bar.repeat_bar = 0 as libc::c_int as libc::c_char;
        if (*s).flags as libc::c_int & 0x2 as libc::c_int != 0 {
            (*p_voice)
                .bar_start = ((*p_voice).bar_start as libc::c_int
                | 0x8000 as libc::c_int) as libc::c_short;
        }
        if (*s).sflags & 0x400000 as libc::c_int as libc::c_uint != 0 {
            (*p_voice)
                .bar_start = ((*p_voice).bar_start as libc::c_int
                | 0x4000 as libc::c_int) as libc::c_short;
        }
        if (*s).flags as libc::c_int & 0x100 as libc::c_int != 0 {
            (*p_voice)
                .bar_start = ((*p_voice).bar_start as libc::c_int
                | 0x2000 as libc::c_int) as libc::c_short;
        }
        if (*s).sflags & 0x10000000 as libc::c_int as libc::c_uint != 0 {
            (*p_voice)
                .bar_start = ((*p_voice).bar_start as libc::c_int
                | 0x1000 as libc::c_int) as libc::c_short;
        }
    }
    bar_type = (*s).u.bar.type_0;
    if bar_type == 4 as libc::c_int {
        return;
    }
    if bar_type & 0xf as libc::c_int != 4 as libc::c_int {
        return;
    }
    if (*s).sflags & 0x100 as libc::c_int as libc::c_uint == 0 {
        if bar_type == (0x1 as libc::c_int) << 8 as libc::c_int | 0x14 as libc::c_int {
            (*p_voice).bar_start = 0x14 as libc::c_int as libc::c_short;
            (*s).u.bar.type_0 = 0x11 as libc::c_int;
            return;
        }
        (*p_voice).bar_start = (bar_type & 0xfff as libc::c_int) as libc::c_short;
        if (*s).flags as libc::c_int & 0x2 as libc::c_int != 0 {
            (*p_voice)
                .bar_start = ((*p_voice).bar_start as libc::c_int
                | 0x8000 as libc::c_int) as libc::c_short;
        }
        if (*s).sflags & 0x400000 as libc::c_int as libc::c_uint != 0 {
            (*p_voice)
                .bar_start = ((*p_voice).bar_start as libc::c_int
                | 0x4000 as libc::c_int) as libc::c_short;
        }
        if !((*s).prev).is_null()
            && (*(*s).prev).type_0 as libc::c_int == 3 as libc::c_int
        {
            unlksym(s);
        } else {
            (*s).u.bar.type_0 = 1 as libc::c_int;
        }
        return;
    }
    if bar_type == 0x44 as libc::c_int {
        (*s).u.bar.type_0 = 0x41 as libc::c_int;
        (*p_voice).bar_start = 0x14 as libc::c_int as libc::c_short;
        if (*s).flags as libc::c_int & 0x2 as libc::c_int != 0 {
            (*p_voice)
                .bar_start = ((*p_voice).bar_start as libc::c_int
                | 0x8000 as libc::c_int) as libc::c_short;
        }
        if (*s).sflags & 0x400000 as libc::c_int as libc::c_uint != 0 {
            (*p_voice)
                .bar_start = ((*p_voice).bar_start as libc::c_int
                | 0x4000 as libc::c_int) as libc::c_short;
        }
        if (*s).flags as libc::c_int & 0x100 as libc::c_int != 0 {
            (*p_voice)
                .bar_start = ((*p_voice).bar_start as libc::c_int
                | 0x2000 as libc::c_int) as libc::c_short;
        }
        if (*s).sflags & 0x10000000 as libc::c_int as libc::c_uint != 0 {
            (*p_voice)
                .bar_start = ((*p_voice).bar_start as libc::c_int
                | 0x1000 as libc::c_int) as libc::c_short;
        }
        return;
    }
    i = 0 as libc::c_int;
    while bar_type != 0 as libc::c_int {
        bar_type >>= 4 as libc::c_int;
        i += 1;
        i;
    }
    bar_type = (*s).u.bar.type_0;
    (*s).u.bar.type_0 = bar_type >> i / 2 as libc::c_int * 4 as libc::c_int;
    i = (i + 1 as libc::c_int) / 2 as libc::c_int * 4 as libc::c_int;
    bar_type &= 0xfff as libc::c_int;
    (*p_voice)
        .bar_start = (bar_type & ((1 as libc::c_int) << i) - 1 as libc::c_int)
        as libc::c_short;
    if (*s).flags as libc::c_int & 0x2 as libc::c_int != 0 {
        (*p_voice)
            .bar_start = ((*p_voice).bar_start as libc::c_int | 0x8000 as libc::c_int)
            as libc::c_short;
    }
    if (*s).sflags & 0x400000 as libc::c_int as libc::c_uint != 0 {
        (*p_voice)
            .bar_start = ((*p_voice).bar_start as libc::c_int | 0x4000 as libc::c_int)
            as libc::c_short;
    }
    if (*s).flags as libc::c_int & 0x100 as libc::c_int != 0 {
        (*p_voice)
            .bar_start = ((*p_voice).bar_start as libc::c_int | 0x2000 as libc::c_int)
            as libc::c_short;
    }
    if (*s).sflags & 0x10000000 as libc::c_int as libc::c_uint != 0 {
        (*p_voice)
            .bar_start = ((*p_voice).bar_start as libc::c_int | 0x1000 as libc::c_int)
            as libc::c_short;
    }
}
unsafe extern "C" fn sym_staff_move(mut staff: libc::c_int) {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    s = tsfirst;
    while !s.is_null() {
        if (*s).sflags & 0x40000 as libc::c_int as libc::c_uint != 0 {
            break;
        }
        if (*s).staff as libc::c_int == staff
            && (*s).type_0 as libc::c_int != 4 as libc::c_int
        {
            (*s).staff = ((*s).staff).wrapping_add(1);
            (*s).staff;
            (*s)
                .flags = ((*s).flags as libc::c_int | 0x2 as libc::c_int)
                as libc::c_ushort;
        }
        s = (*s).ts_next;
    }
}
unsafe extern "C" fn set_brace(
    mut sy: *mut SYSTEM,
    mut empty: *mut libc::c_char,
    mut empty_gl: *mut libc::c_char,
) {
    let mut staff: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut empty_fl: libc::c_int = 0;
    staff = 0 as libc::c_int;
    while staff <= nstaff {
        if !((*sy).staff[staff as usize].flags as libc::c_int
            & (0x1 as libc::c_int | 0x100 as libc::c_int) == 0)
        {
            empty_fl = 0 as libc::c_int;
            i = staff;
            while staff <= nstaff {
                empty_fl
                    |= if *empty.offset(staff as isize) as libc::c_int != 0 {
                        1 as libc::c_int
                    } else {
                        2 as libc::c_int
                    };
                if (*cursys).staff[staff as usize].flags as libc::c_int
                    & (0x2 as libc::c_int | 0x200 as libc::c_int) != 0
                {
                    break;
                }
                staff += 1;
                staff;
            }
            if empty_fl == 3 as libc::c_int {
                while i <= staff {
                    *empty.offset(i as isize) = 0 as libc::c_int as libc::c_char;
                    let fresh2 = i;
                    i = i + 1;
                    *empty_gl.offset(fresh2 as isize) = 0 as libc::c_int as libc::c_char;
                }
            }
        }
        staff += 1;
        staff;
    }
}
unsafe extern "C" fn set_piece() {
    let mut sy: *mut SYSTEM = 0 as *mut SYSTEM;
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut p_staff: *mut STAFF_S = 0 as *mut STAFF_S;
    let mut staff: libc::c_int = 0;
    let mut empty: [libc::c_char; 32] = [0; 32];
    let mut empty_gl: [libc::c_char; 32] = [0; 32];
    sy = cursys;
    staff = 0 as libc::c_int;
    while staff <= nstaff {
        p_staff = &mut *staff_tb.as_mut_ptr().offset(staff as isize) as *mut STAFF_S;
        (*p_staff).y = 0 as libc::c_int as libc::c_float;
        (*p_staff).stafflines = (*sy).staff[staff as usize].stafflines;
        (*p_staff).staffscale = (*sy).staff[staff as usize].staffscale;
        staff += 1;
        staff;
    }
    memset(
        empty.as_mut_ptr() as *mut libc::c_void,
        1 as libc::c_int,
        ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
    );
    memset(
        empty_gl.as_mut_ptr() as *mut libc::c_void,
        1 as libc::c_int,
        ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
    );
    s = tsfirst;
    while !s.is_null() {
        if (*s).sflags & 0x40000 as libc::c_int as libc::c_uint != 0 {
            break;
        }
        if (*s).sflags & 0x8000000 as libc::c_int as libc::c_uint != 0 {
            set_brace(sy, empty.as_mut_ptr(), empty_gl.as_mut_ptr());
            staff = 0 as libc::c_int;
            while staff <= nstaff {
                (*sy).staff[staff as usize].empty = empty[staff as usize];
                empty[staff as usize] = 1 as libc::c_int as libc::c_char;
                staff += 1;
                staff;
            }
            sy = (*sy).next;
            staff = 0 as libc::c_int;
            while staff <= (*sy).nstaff as libc::c_int {
                p_staff = &mut *staff_tb.as_mut_ptr().offset(staff as isize)
                    as *mut STAFF_S;
                (*p_staff).stafflines = (*sy).staff[staff as usize].stafflines;
                if ((*p_staff).stafflines).is_null() {
                    (*p_staff)
                        .stafflines = b"|||||\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                (*p_staff).staffscale = (*sy).staff[staff as usize].staffscale;
                if (*p_staff).staffscale == 0 as libc::c_int as libc::c_float {
                    (*p_staff).staffscale = 1 as libc::c_int as libc::c_float;
                }
                staff += 1;
                staff;
            }
        }
        if !(empty[(*s).staff as usize] == 0) {
            match (*s).type_0 as libc::c_int {
                11 => {
                    empty[(*s).staff as usize] = 0 as libc::c_int as libc::c_char;
                    empty_gl[(*s).staff as usize] = empty[(*s).staff as usize];
                }
                1 | 2 | 9 => {
                    if cfmt.staffnonote > 1 as libc::c_int {
                        empty[(*s).staff as usize] = 0 as libc::c_int as libc::c_char;
                        empty_gl[(*s).staff as usize] = empty[(*s).staff as usize];
                    } else if (*s).flags as libc::c_int & 0x2 as libc::c_int == 0 {
                        if (*s).abc_type as libc::c_int == 4 as libc::c_int
                            || cfmt.staffnonote != 0 as libc::c_int
                        {
                            empty[(*s).staff
                                as usize] = 0 as libc::c_int as libc::c_char;
                            empty_gl[(*s).staff as usize] = empty[(*s).staff as usize];
                        }
                    }
                }
                _ => {}
            }
        }
        s = (*s).ts_next;
    }
    tsnext = s;
    set_brace(sy, empty.as_mut_ptr(), empty_gl.as_mut_ptr());
    staff = 0 as libc::c_int;
    while staff <= nstaff {
        (*sy).staff[staff as usize].empty = empty[staff as usize];
        staff += 1;
        staff;
    }
    staff = 0 as libc::c_int;
    while staff <= nstaff {
        let mut i: libc::c_int = 0;
        let mut l: libc::c_int = 0;
        let mut stafflines: *mut libc::c_char = 0 as *mut libc::c_char;
        if !(empty_gl[staff as usize] != 0) {
            p_staff = &mut *staff_tb.as_mut_ptr().offset(staff as isize) as *mut STAFF_S;
            stafflines = (*p_staff).stafflines;
            l = strlen(stafflines) as libc::c_int;
            (*p_staff)
                .topbar = (6 as libc::c_int * (l - 1 as libc::c_int)) as libc::c_short;
            i = 0 as libc::c_int;
            while i < l - 1 as libc::c_int {
                if *stafflines.offset(i as isize) as libc::c_int != '.' as i32 {
                    break;
                }
                i += 1;
                i;
            }
            (*p_staff).botbar = (i * 6 as libc::c_int) as libc::c_short;
            if i >= l - 2 as libc::c_int {
                (*p_staff)
                    .botbar = ((*p_staff).botbar as libc::c_int - 6 as libc::c_int)
                    as libc::c_short;
                (*p_staff)
                    .topbar = ((*p_staff).topbar as libc::c_int + 6 as libc::c_int)
                    as libc::c_short;
            }
        }
        staff += 1;
        staff;
    }
    staff = 0 as libc::c_int;
    while staff < nstaff {
        if empty_gl[staff as usize] != 0 {
            sym_staff_move(staff);
        }
        staff += 1;
        staff;
    }
    init_music_line();
    if tsnext.is_null() {
        return;
    }
    s = tsnext;
    (*s).sflags &= !(0x40000 as libc::c_int) as libc::c_uint;
    s = (*s).ts_prev;
    (*s).ts_next = 0 as *mut SYMBOL;
    p_voice = first_voice;
    while !p_voice.is_null() {
        let mut voice: libc::c_int = 0;
        voice = p_voice.offset_from(voice_tb.as_mut_ptr()) as libc::c_long
            as libc::c_int;
        s = (*tsnext).ts_prev;
        while !s.is_null() {
            if (*s).voice as libc::c_int == voice {
                (*s).next = 0 as *mut SYMBOL;
                check_bar(s);
                break;
            } else {
                s = (*s).ts_prev;
            }
        }
        if s.is_null() {
            (*p_voice).sym = 0 as *mut SYMBOL;
        }
        p_voice = (*p_voice).next;
    }
}
unsafe extern "C" fn set_sym_glue(mut width: libc::c_float) {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut beta0: libc::c_float = 0.;
    let mut alfa: libc::c_float = 0.;
    let mut beta: libc::c_float = 0.;
    let mut some_grace: libc::c_int = 0;
    let mut xmin: libc::c_float = 0.;
    let mut x: libc::c_float = 0.;
    let mut xmax: libc::c_float = 0.;
    let mut spafac: libc::c_float = 0.;
    some_grace = 0 as libc::c_int;
    xmax = 0 as libc::c_int as libc::c_float;
    x = xmax;
    xmin = x;
    s = tsfirst;
    loop {
        if (*s).type_0 as libc::c_int == 11 as libc::c_int {
            some_grace = 1 as libc::c_int;
        }
        if (*s).sflags & 0x80000 as libc::c_int as libc::c_uint != 0 {
            let mut space: libc::c_float = 0.;
            xmin += (*s).shrink;
            space = (*s).space;
            if space < (*s).shrink {
                space = (*s).shrink;
            }
            x += space;
            if cfmt.stretchstaff != 0 {
                space = (space as libc::c_double * 1.8f64) as libc::c_float;
            }
            xmax += space;
        }
        if ((*s).ts_next).is_null() {
            break;
        }
        s = (*s).ts_next;
    }
    if cfmt.continueall == 0 {
        beta0 = 1.0f64 as libc::c_float;
    } else {
        beta0 = 0.1f64 as libc::c_float;
    }
    if !tsnext.is_null() {
        if x - width >= 0 as libc::c_int as libc::c_float {
            beta_last = 0 as libc::c_int as libc::c_float;
        } else {
            beta_last = (width - x) / (xmax - x);
            if beta_last > beta0 {
                if cfmt.stretchstaff != 0 {
                    if cfmt.continueall == 0 && cfmt.linewarn != 0 {
                        error(
                            0 as libc::c_int,
                            s,
                            b"Line underfull (%.0fpt of %.0fpt)\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            (beta0 * xmax
                                + (1 as libc::c_int as libc::c_float - beta0) * x)
                                as libc::c_double,
                            width as libc::c_double,
                        );
                    }
                } else {
                    width = x;
                    beta_last = 0 as libc::c_int as libc::c_float;
                }
            }
        }
    } else if x < width {
        beta = (width - x) / (xmax - x);
        if beta >= beta_last {
            beta = beta_last * xmax
                + (1 as libc::c_int as libc::c_float - beta_last) * x;
            if (beta as libc::c_double)
                < width as libc::c_double * (1.0f64 - cfmt.stretchlast as libc::c_double)
            {
                width = beta;
            }
        }
    }
    spafac = width / x;
    xmax = 0 as libc::c_int as libc::c_float;
    x = xmax;
    s = tsfirst;
    loop {
        if (*s).sflags & 0x80000 as libc::c_int as libc::c_uint != 0 {
            let mut new_space: libc::c_float = 0.;
            new_space = (*s).shrink;
            if (*s).space != 0 as libc::c_int as libc::c_float {
                if new_space < (*s).space * spafac {
                    new_space = (*s).space * spafac;
                }
                xmax = (xmax as libc::c_double
                    + ((*s).space * spafac) as libc::c_double * 1.8f64) as libc::c_float;
            }
            x += new_space;
            xmax += new_space;
            (*s).x = x;
            (*s).xmax = xmax;
        }
        if ((*s).ts_next).is_null() {
            break;
        }
        s = (*s).ts_next;
    }
    match (*s).type_0 as libc::c_int {
        3 | 12 => {}
        15 => {
            x += (*s).wr;
            xmin += (*s).wr;
            xmax += (*s).wr;
        }
        _ => {
            let mut min: libc::c_float = 0.;
            min = (*s).wr;
            while (*s).sflags & 0x80000 as libc::c_int as libc::c_uint == 0 {
                s = (*s).ts_prev;
                if (*s).wr > min {
                    min = (*s).wr;
                }
            }
            xmin += min + 3 as libc::c_int as libc::c_float;
            if !tsnext.is_null()
                && (*tsnext).space as libc::c_double * 0.8f64
                    > ((*s).wr + 4 as libc::c_int as libc::c_float) as libc::c_double
            {
                x = (x as libc::c_double
                    + (*tsnext).space as libc::c_double * 0.8f64
                        * spafac as libc::c_double) as libc::c_float;
                xmax = (xmax as libc::c_double
                    + (*tsnext).space as libc::c_double * 0.8f64
                        * spafac as libc::c_double * 1.8f64) as libc::c_float;
            } else {
                x += min + 4 as libc::c_int as libc::c_float;
                xmax += min + 4 as libc::c_int as libc::c_float;
            }
        }
    }
    if x >= width {
        beta = 0 as libc::c_int as libc::c_float;
        if x == xmin {
            alfa = 1 as libc::c_int as libc::c_float;
        } else {
            alfa = (x - width) / (x - xmin);
            if alfa > 1 as libc::c_int as libc::c_float {
                error(
                    1 as libc::c_int,
                    s,
                    b"Line too much shrunk (%.0f/%0.fpt of %.0fpt)\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    xmin as libc::c_double,
                    x as libc::c_double,
                    width as libc::c_double,
                );
            }
        }
        realwidth = xmin * alfa + x * (1 as libc::c_int as libc::c_float - alfa);
    } else {
        alfa = 0 as libc::c_int as libc::c_float;
        if xmax > x {
            beta = (width - x) / (xmax - x);
        } else {
            beta = 1 as libc::c_int as libc::c_float;
        }
        if beta > beta0 {
            if cfmt.stretchstaff == 0 {
                beta = 0 as libc::c_int as libc::c_float;
            }
        }
        realwidth = xmax * beta + x * (1 as libc::c_int as libc::c_float - beta);
    }
    s = tsfirst;
    if alfa != 0 as libc::c_int as libc::c_float {
        if alfa < 1 as libc::c_int as libc::c_float {
            xmin = 0 as libc::c_int as libc::c_float;
            x = xmin;
            while !s.is_null() {
                if (*s).sflags & 0x80000 as libc::c_int as libc::c_uint != 0 {
                    xmin += (*s).shrink * alfa;
                    x = xmin + (*s).x * (1 as libc::c_int as libc::c_float - alfa);
                }
                (*s).x = x;
                s = (*s).ts_next;
            }
        } else {
            alfa = realwidth / x;
            x = 0 as libc::c_int as libc::c_float;
            while !s.is_null() {
                if (*s).sflags & 0x80000 as libc::c_int as libc::c_uint != 0 {
                    x = (*s).x * alfa;
                }
                (*s).x = x;
                s = (*s).ts_next;
            }
        }
    } else {
        x = 0 as libc::c_int as libc::c_float;
        while !s.is_null() {
            if (*s).sflags & 0x80000 as libc::c_int as libc::c_uint != 0 {
                x = (*s).xmax * beta
                    + (*s).x * (1 as libc::c_int as libc::c_float - beta);
            }
            (*s).x = x;
            s = (*s).ts_next;
        }
    }
    if some_grace != 0 {
        s = tsfirst;
        while !s.is_null() {
            let mut g: *mut SYMBOL = 0 as *mut SYMBOL;
            if !((*s).type_0 as libc::c_int != 11 as libc::c_int) {
                x = (((*s).x - (*s).wl) as libc::c_double
                    + (cfmt.gracespace >> 16 as libc::c_int) as libc::c_double * 0.1f64)
                    as libc::c_float;
                g = (*s).extra;
                while !g.is_null() {
                    if (*g).type_0 as libc::c_int == 1 as libc::c_int {
                        (*g).x += x;
                    }
                    g = (*g).next;
                }
            }
            s = (*s).ts_next;
        }
    }
}
unsafe extern "C" fn new_music_line() {
    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut voice: libc::c_int = 0;
    (*tsfirst).ts_prev = 0 as *mut SYMBOL;
    p_voice = first_voice;
    while !p_voice.is_null() {
        (*p_voice).sym = 0 as *mut SYMBOL;
        voice = p_voice.offset_from(voice_tb.as_mut_ptr()) as libc::c_long
            as libc::c_int;
        s = tsfirst;
        while !s.is_null() {
            if (*s).voice as libc::c_int == voice {
                (*p_voice).sym = s;
                (*s).prev = 0 as *mut SYMBOL;
                break;
            } else {
                s = (*s).ts_next;
            }
        }
        p_voice = (*p_voice).next;
    }
}
unsafe extern "C" fn gen_init() {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    s = tsfirst;
    while !s.is_null() {
        if !((*s).extra).is_null() {
            output_ps(s, 0 as libc::c_int);
            if ((*s).extra).is_null() && (*s).type_0 as libc::c_int == 12 as libc::c_int
            {
                unlksym(s);
                if tsfirst.is_null() {
                    return;
                }
            }
        }
        if (*s).sflags & 0x8000000 as libc::c_int as libc::c_uint != 0 {
            (*s).sflags &= !(0x8000000 as libc::c_int) as libc::c_uint;
            cursys = (*cursys).next;
        }
        match (*s).type_0 as libc::c_int {
            4 | 6 | 5 => {
                s = (*s).ts_next;
            }
            _ => return,
        }
    }
    tsfirst = 0 as *mut SYMBOL;
}
unsafe extern "C" fn error_show() {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    s = tsfirst;
    while !s.is_null() {
        if (*s).flags as libc::c_int & 0x1 as libc::c_int != 0 {
            putxy(
                (*s).x,
                staff_tb[(*s).staff as usize].y + (*s).y as libc::c_int as libc::c_float,
            );
            a2b(
                b"showerror\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        s = (*s).ts_next;
    }
}
unsafe extern "C" fn delayed_output(mut indent: libc::c_float) -> libc::c_float {
    let mut line_height: libc::c_float = 0.;
    let mut outbuf_sav: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mbf_sav: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmpbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    outbuf_sav = outbuf;
    mbf_sav = mbf;
    tmpbuf = malloc(outbufsz as libc::c_ulong) as *mut libc::c_char;
    if tmpbuf.is_null() {
        error(
            1 as libc::c_int,
            0 as *mut SYMBOL,
            b"Out of memory for delayed outbuf - abort\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    outbuf = tmpbuf;
    mbf = outbuf;
    *outbuf = '\0' as i32 as libc::c_char;
    outft = -(1 as libc::c_int);
    draw_sym_near();
    outbuf = outbuf_sav;
    mbf = mbf_sav;
    outft = -(1 as libc::c_int);
    line_height = draw_systems(indent);
    a2b(b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char, tmpbuf);
    free(tmpbuf as *mut libc::c_void);
    return line_height;
}
#[no_mangle]
pub unsafe extern "C" fn output_music() {
    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut lwidth: libc::c_float = 0.;
    let mut indent: libc::c_float = 0.;
    gen_init();
    if tsfirst.is_null() {
        return;
    }
    check_buffer();
    set_global();
    if !((*first_voice).next).is_null() {
        combine_voices();
        set_stem_dir();
    }
    p_voice = first_voice;
    while !p_voice.is_null() {
        set_beams((*p_voice).sym);
        p_voice = (*p_voice).next;
    }
    set_stems();
    if !((*first_voice).next).is_null() {
        set_rest_offset();
        set_overlap();
    }
    set_acc_shft();
    set_allsymwidth(0 as *mut SYMBOL);
    lwidth = ((if cfmt.landscape != 0 { cfmt.pageheight } else { cfmt.pagewidth })
        - cfmt.leftmargin - cfmt.rightmargin) / cfmt.scale;
    if lwidth < 50 as libc::c_int as libc::c_float {
        error(
            1 as libc::c_int,
            0 as *mut SYMBOL,
            b"Bad page width %.1f\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            lwidth as libc::c_double,
        );
        lwidth = (10 as libc::c_int as libc::c_double * 37.8f64) as libc::c_float;
    }
    indent = set_indent();
    cut_tune(lwidth, indent);
    beta_last = 0 as libc::c_int as libc::c_float;
    loop {
        let mut line_height: libc::c_float = 0.;
        set_piece();
        indent = set_indent();
        set_sym_glue(lwidth - indent);
        if indent != 0 as libc::c_int as libc::c_float {
            a2b(
                b"%.2f 0 T\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                indent as libc::c_double,
            );
        }
        line_height = delayed_output(indent);
        draw_all_symb();
        draw_all_deco();
        if showerror != 0 {
            error_show();
        }
        bskip(line_height);
        if indent != 0 as libc::c_int as libc::c_float {
            a2b(
                b"%.2f 0 T\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                -indent as libc::c_double,
            );
            insert_meter &= !(2 as libc::c_int);
        }
        tsfirst = tsnext;
        gen_init();
        if tsfirst.is_null() {
            break;
        }
        buffer_eob(0 as libc::c_int);
        new_music_line();
    }
    outft = -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn reset_gen() {
    if cfmt.fields[0 as libc::c_int as usize]
        & ((1 as libc::c_int) << 'M' as i32 - 'A' as i32) as libc::c_uint != 0
    {
        insert_meter = 3 as libc::c_int;
    } else {
        insert_meter = 2 as libc::c_int;
    };
}
