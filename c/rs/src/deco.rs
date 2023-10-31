use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
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
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn __maskrune(_: __darwin_ct_rune_t, _: libc::c_ulong) -> libc::c_int;
    static mut _DefaultRuneLocale: _RuneLocale;
    fn lroundf(_: libc::c_float) -> libc::c_long;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    static mut parse: parse;
    static mut cfmt: FORMAT;
    fn a2b(fmt: *mut libc::c_char, _: ...);
    fn getarena(len: libc::c_int) -> *mut libc::c_void;
    static mut outft: libc::c_int;
    static mut cursys: *mut SYSTEM;
    static mut hw_tb: [libc::c_float; 0];
    static mut realwidth: libc::c_float;
    static mut tsfirst: *mut SYMBOL;
    static mut first_voice: *mut VOICE_S;
    static mut voice_tb: [VOICE_S; 32];
    static mut curvoice: *mut VOICE_S;
    fn set_color(color: libc::c_int);
    fn set_sscale(staff: libc::c_int);
    fn putxy(x: libc::c_float, y: libc::c_float);
    fn putf(f: libc::c_float);
    fn putx(x: libc::c_float);
    fn set_scale(s: *mut SYMBOL);
    fn prev_scut(s: *mut SYMBOL) -> *mut SYMBOL;
    fn puty(y: libc::c_float);
    static mut nbar: libc::c_int;
    static mut nstaff: libc::c_int;
    static mut staff_tb: [STAFF_S; 32];
    fn set_font(ft: libc::c_int);
    fn error(sev: libc::c_int, s: *mut SYMBOL, fmt: *mut libc::c_char, _: ...);
    static mut tex_buf: [libc::c_char; 0];
    fn str_out(p: *mut libc::c_char, action: libc::c_int);
    fn tex_str(s: *mut libc::c_char) -> libc::c_float;
    fn identify_note(
        s: *mut SYMBOL,
        len: libc::c_int,
        p_head: *mut libc::c_int,
        p_dots: *mut libc::c_int,
        p_flags: *mut libc::c_int,
    );
    fn put_str(str: *mut libc::c_char, action: libc::c_int);
    fn str_font(ft: libc::c_int);
    fn cwid(c: libc::c_uchar) -> libc::c_float;
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
pub struct u_deco {
    pub next: *mut u_deco,
    pub text: [libc::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct deco_def_s {
    pub name: *mut libc::c_char,
    pub func: libc::c_uchar,
    pub ps_func: libc::c_schar,
    pub h: libc::c_uchar,
    pub wl: libc::c_uchar,
    pub wr: libc::c_uchar,
    pub strx: libc::c_uchar,
    pub ld_start: libc::c_uchar,
    pub ld_end: libc::c_uchar,
    pub flags: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct deco_elt {
    pub next: *mut deco_elt,
    pub prev: *mut deco_elt,
    pub s: *mut SYMBOL,
    pub start: *mut deco_elt,
    pub t: libc::c_uchar,
    pub staff: libc::c_uchar,
    pub flags: libc::c_uchar,
    pub defl: libc::c_uchar,
    pub m: libc::c_schar,
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub dy: libc::c_float,
    pub val: libc::c_float,
}
pub type draw_f = unsafe extern "C" fn(*mut deco_elt) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub ymin: libc::c_float,
    pub ymax: libc::c_float,
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
#[no_mangle]
pub static mut defl: libc::c_int = 0;
#[no_mangle]
pub static mut deco: [*mut libc::c_char; 256] = [0 as *const libc::c_char
    as *mut libc::c_char; 256];
static mut deco_head: *mut deco_elt = 0 as *const deco_elt as *mut deco_elt;
static mut deco_tail: *mut deco_elt = 0 as *const deco_elt as *mut deco_elt;
static mut deco_def_tb: [deco_def_s; 128] = [deco_def_s {
    name: 0 as *const libc::c_char as *mut libc::c_char,
    func: 0,
    ps_func: 0,
    h: 0,
    wl: 0,
    wr: 0,
    strx: 0,
    ld_start: 0,
    ld_end: 0,
    flags: 0,
}; 128];
static mut func_tb: [Option::<draw_f>; 8] = unsafe {
    [
        Some(d_near as draw_f),
        Some(d_slide as draw_f),
        Some(d_arp as draw_f),
        Some(d_upstaff as draw_f),
        Some(d_upstaff as draw_f),
        Some(d_trill as draw_f),
        Some(d_pf as draw_f),
        Some(d_cresc as draw_f),
    ]
};
static mut f_near: libc::c_short = ((1 as libc::c_int) << 0 as libc::c_int
    | (1 as libc::c_int) << 1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int)
    as libc::c_short;
static mut f_note: libc::c_short = ((1 as libc::c_int) << 3 as libc::c_int
    | (1 as libc::c_int) << 4 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int)
    as libc::c_short;
static mut f_staff: libc::c_short = ((1 as libc::c_int) << 6 as libc::c_int
    | (1 as libc::c_int) << 7 as libc::c_int) as libc::c_short;
static mut ps_func_tb: [*mut libc::c_char; 128] = [0 as *const libc::c_char
    as *mut libc::c_char; 128];
static mut str_tb: [*mut libc::c_char; 32] = [0 as *const libc::c_char
    as *mut libc::c_char; 32];
static mut std_deco_tb: [*mut libc::c_char; 100] = [
    b"dot 0 stc 5 1 1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"roll 3 cpu 7 6 6\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"fermata 3 hld 12 7 7\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"emphasis 3 accent 7 4 4\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"lowermordent 3 lmrd 10 2 2\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"coda 3 coda 24 10 10\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"dacoda 3 dacoda 16 10 10\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"uppermordent 3 umrd 10 2 2\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"segno 3 sgno 20 4 4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"trill 3 trl 11 4 4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"upbow 3 upb 10 5 5\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"downbow 3 dnb 9 5 5\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"gmark 3 grm 6 5 5\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"slide 1 sld 3 7 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tenuto 0 emb 5 2 2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"breath 3 brth 0 1 20\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"longphrase 3 lphr 0 1 1\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"mediumphrase 3 mphr 0 1 1\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"shortphrase 3 sphr 0 1 1\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"invertedfermata 3 hld 12 7 7\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"invertedturn 3 turn 10 0 5\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"invertedturnx 3 turnx 10 0 5\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"0 3 fng 8 3 3 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"1 3 fng 8 3 3 1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"2 3 fng 8 3 3 2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"3 3 fng 8 3 3 3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"4 3 fng 8 3 3 4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"5 3 fng 8 3 3 5\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"plus 3 dplus 7 3 3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"+ 3 dplus 7 3 3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"accent 3 accent 7 4 4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"> 3 accent 7 4 4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"marcato 3 marcato 9 3 3\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"^ 3 marcato 9 3 3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"D.C. 3 dacs 16 10 10 D.C.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"D.C.alcoda 3 dacs 16 10 10 D.C. al Coda\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"D.C.alfine 3 dacs 16 10 10 D.C. al Fine\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"dacapo 3 dacs 16 10 10 Da Capo\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"D.S. 3 dacs 16 10 10 D.S.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"D.S.alcoda 3 dacs 16 10 10 D.S. al Coda\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"D.S.alfine 3 dacs 16 10 10 D.S. al Fine\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"fine 3 dacs 16 10 10 FINE\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"f 6 pf 18 1 7\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ff 6 pf 18 2 10\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"fff 6 pf 18 4 13\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ffff 6 pf 18 6 16\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mf 6 pf 18 6 13\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mp 6 pf 18 6 16\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"mordent 3 lmrd 10 2 2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"open 3 opend 10 2 2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"p 6 pf 18 2 8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"pp 6 pf 18 5 14\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ppp 6 pf 18 8 20\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"pppp 6 pf 18 10 25\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"pralltriller 3 umrd 10 2 2\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"sfz 6 sfz 18 4 10\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ped 4 ped 20 0 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ped-up 4 pedoff 20 0 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"turn 3 turn 10 0 5\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"wedge 3 wedge 8 1 1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"turnx 3 turnx 10 0 5\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"trill( 3 ltr 8 0 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"trill) 3 ltr 8 0 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"snap 3 snap 14 3 3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"thumb 3 thumb 14 2 2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"arpeggio 2 arp 12 10 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"crescendo( 6 cresc 18 0 0\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"crescendo) 6 cresc 18 0 0\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"<( 6 cresc 18 0 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"<) 6 cresc 18 0 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"diminuendo( 6 dim 18 0 0\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"diminuendo) 6 dim 18 0 0\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b">( 6 dim 18 0 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b">) 6 dim 18 0 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"-( 8 gliss 0 0 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"-) 8 gliss 0 0 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"~( 8 glisq 0 0 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"~) 8 glisq 0 0 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"8va( 3 o8va 10 0 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"8va) 3 o8va 10 0 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"8vb( 4 o8vb 10 0 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"8vb) 4 o8vb 10 0 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"invisible 32 0 0 0 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"beamon 33 0 0 0 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"trem1 34 0 0 0 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"trem2 34 0 0 0 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"trem3 34 0 0 0 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"trem4 34 0 0 0 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"xstem 35 0 0 0 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"beambr1 36 0 0 0 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"beambr2 36 0 0 0 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"rbstop 37 0 0 0 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"/ 38 0 0 6 6\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"// 38 0 0 6 6\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"/// 38 0 0 6 6\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"beam-accel 39 0 0 0 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"beam-rall 39 0 0 0 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"stemless 40 0 0 0 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"rbend 41 0 0 0 0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
static mut user_deco: *mut u_deco = 0 as *const u_deco as *mut u_deco;
#[no_mangle]
pub unsafe extern "C" fn y_get(
    mut staff: libc::c_int,
    mut up: libc::c_int,
    mut x: libc::c_float,
    mut w: libc::c_float,
) -> libc::c_float {
    let mut p_staff: *mut STAFF_S = 0 as *mut STAFF_S;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut y: libc::c_float = 0.;
    p_staff = &mut *staff_tb.as_mut_ptr().offset(staff as isize) as *mut STAFF_S;
    i = (x / realwidth * 128 as libc::c_int as libc::c_float) as libc::c_int;
    if i < 0 as libc::c_int {
        i = 0 as libc::c_int;
    }
    j = ((x + w) / realwidth * 128 as libc::c_int as libc::c_float) as libc::c_int;
    if j >= 128 as libc::c_int {
        j = 128 as libc::c_int - 1 as libc::c_int;
        if i > j {
            i = j;
        }
    }
    if up != 0 {
        let fresh0 = i;
        i = i + 1;
        y = (*p_staff).top[fresh0 as usize];
        while i <= j {
            if y < (*p_staff).top[i as usize] {
                y = (*p_staff).top[i as usize];
            }
            i += 1;
            i;
        }
    } else {
        let fresh1 = i;
        i = i + 1;
        y = (*p_staff).bot[fresh1 as usize];
        while i <= j {
            if y > (*p_staff).bot[i as usize] {
                y = (*p_staff).bot[i as usize];
            }
            i += 1;
            i;
        }
    }
    return y;
}
#[no_mangle]
pub unsafe extern "C" fn y_set(
    mut staff: libc::c_int,
    mut up: libc::c_int,
    mut x: libc::c_float,
    mut w: libc::c_float,
    mut y: libc::c_float,
) {
    let mut p_staff: *mut STAFF_S = 0 as *mut STAFF_S;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    p_staff = &mut *staff_tb.as_mut_ptr().offset(staff as isize) as *mut STAFF_S;
    i = (x / realwidth * 128 as libc::c_int as libc::c_float) as libc::c_int;
    if i < 0 as libc::c_int {
        i = 0 as libc::c_int;
    }
    j = ((x + w) / realwidth * 128 as libc::c_int as libc::c_float) as libc::c_int;
    if j >= 128 as libc::c_int {
        j = 128 as libc::c_int - 1 as libc::c_int;
        if i > j {
            i = j;
        }
    }
    if up != 0 {
        while i <= j {
            if (*p_staff).top[i as usize] < y {
                (*p_staff).top[i as usize] = y;
            }
            i += 1;
            i;
        }
    } else {
        while i <= j {
            if (*p_staff).bot[i as usize] > y {
                (*p_staff).bot[i as usize] = y;
            }
            i += 1;
            i;
        }
    };
}
unsafe extern "C" fn up_p(mut s: *mut SYMBOL, mut pos: libc::c_int) -> libc::c_int {
    match pos {
        1 => return 1 as libc::c_int,
        2 => return 0 as libc::c_int,
        _ => {}
    }
    if (*s).multi as libc::c_int != 0 as libc::c_int {
        return ((*s).multi as libc::c_int > 0 as libc::c_int) as libc::c_int;
    }
    if (voice_tb[(*s).voice as usize]).have_ly() == 0 {
        return 0 as libc::c_int;
    }
    return (((*s).posit).voc() as libc::c_int != 0x1 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn d_arp(mut de: *mut deco_elt) {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut dd: *mut deco_def_s = 0 as *mut deco_def_s;
    let mut m: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut xc: libc::c_float = 0.;
    let mut dx: libc::c_float = 0.;
    s = (*de).s;
    dd = &mut *deco_def_tb.as_mut_ptr().offset((*de).t as isize) as *mut deco_def_s;
    xc = 0 as libc::c_int as libc::c_float;
    m = 0 as libc::c_int;
    while m <= (*s).nhd as libc::c_int {
        if (*s).u.note.notes[m as usize].acc != 0 {
            dx = 5 as libc::c_int as libc::c_float + (*s).u.note.notes[m as usize].shac;
        } else {
            dx = 6 as libc::c_int as libc::c_float - (*s).u.note.notes[m as usize].shhd;
            match (*s).head as libc::c_int {
                3 | 2 => {
                    dx = (dx as libc::c_double + 2.5f64) as libc::c_float;
                }
                _ => {}
            }
        }
        if dx > xc {
            xc = dx;
        }
        m += 1;
        m;
    }
    h = 3 as libc::c_int
        * ((*s).pits[(*s).nhd as usize] as libc::c_int
            - (*s).pits[0 as libc::c_int as usize] as libc::c_int) + 4 as libc::c_int;
    m = (*dd).h as libc::c_int;
    if h < m {
        h = m;
    }
    (*de).flags = ((*de).flags as libc::c_int | 0x1 as libc::c_int) as libc::c_uchar;
    (*de).val = h as libc::c_float;
    (*de).x = (*s).x - xc;
    (*de)
        .y = (3 as libc::c_int
        * ((*s).pits[0 as libc::c_int as usize] as libc::c_int - 18 as libc::c_int))
        as libc::c_float - 3 as libc::c_int as libc::c_float;
}
unsafe extern "C" fn d_cresc(mut de: *mut deco_elt) {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut dd: *mut deco_def_s = 0 as *mut deco_def_s;
    let mut dd2: *mut deco_def_s = 0 as *mut deco_def_s;
    let mut de1: *mut deco_elt = 0 as *mut deco_elt;
    let mut up: libc::c_int = 0;
    let mut x: libc::c_float = 0.;
    let mut dx: libc::c_float = 0.;
    let mut x2: libc::c_float = 0.;
    if (*de).flags as libc::c_int & 0x40 as libc::c_int != 0 {
        return;
    }
    s2 = (*de).s;
    de1 = (*de).start;
    s = (*de1).s;
    x = (*s).x + 3 as libc::c_int as libc::c_float;
    (*de).staff = (*s2).staff;
    (*de).flags = ((*de).flags as libc::c_int & !(0x80 as libc::c_int)) as libc::c_uchar;
    (*de).flags = ((*de).flags as libc::c_int | 0x1 as libc::c_int) as libc::c_uchar;
    up = up_p(s2, ((*s2).posit).dyn_0() as libc::c_int);
    if up != 0 {
        (*de).flags = ((*de).flags as libc::c_int | 0x2 as libc::c_int) as libc::c_uchar;
    }
    if !de1.is_null() && !((*de1).prev).is_null() && (*(*de1).prev).s == s
        && ((*de).flags as libc::c_int ^ (*(*de1).prev).flags as libc::c_int)
            & 0x2 as libc::c_int == 0 as libc::c_int
    {
        dd2 = &mut *deco_def_tb.as_mut_ptr().offset((*(*de1).prev).t as isize)
            as *mut deco_def_s;
        if f_staff as libc::c_int & (1 as libc::c_int) << (*dd2).func as libc::c_int != 0
        {
            x2 = (*(*de1).prev).x + (*(*de1).prev).val
                + 4 as libc::c_int as libc::c_float;
            if x2 > x {
                x = x2;
            }
        }
    }
    if (*de).defl as libc::c_int & 0x2 as libc::c_int != 0 {
        dx = (*de).x - x;
        if dx < 20 as libc::c_int as libc::c_float {
            x = (*de).x - 20 as libc::c_int as libc::c_float
                - 3 as libc::c_int as libc::c_float;
            dx = 20 as libc::c_int as libc::c_float;
        }
    } else {
        x2 = (*s2).x;
        if !((*de).next).is_null() && (*(*de).next).s == s
            && ((*de).flags as libc::c_int ^ (*(*de).next).flags as libc::c_int)
                & 0x2 as libc::c_int == 0 as libc::c_int
        {
            dd2 = &mut *deco_def_tb.as_mut_ptr().offset((*(*de).next).t as isize)
                as *mut deco_def_s;
            if f_staff as libc::c_int & (1 as libc::c_int) << (*dd2).func as libc::c_int
                != 0
            {
                x2 -= 5 as libc::c_int as libc::c_float;
            }
        }
        dx = x2 - x - 4 as libc::c_int as libc::c_float;
        if dx < 20 as libc::c_int as libc::c_float {
            x = (x as libc::c_double
                - (20 as libc::c_int as libc::c_float - dx) as libc::c_double * 0.5f64)
                as libc::c_float;
            dx = 20 as libc::c_int as libc::c_float;
        }
    }
    (*de).val = dx;
    (*de).x = x;
    (*de).y = y_get((*de).staff as libc::c_int, up, x, dx);
    if up == 0 {
        dd = &mut *deco_def_tb.as_mut_ptr().offset((*de).t as isize) as *mut deco_def_s;
        (*de).y -= (*dd).h as libc::c_int as libc::c_float;
    }
}
unsafe extern "C" fn d_gliss(mut de2: *mut deco_elt) {
    let mut de1: *mut deco_elt = 0 as *mut deco_elt;
    let mut s1: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    de1 = (*de2).start;
    s1 = (*de1).s;
    if (*s1).dots != 0 {
        (*de1).x += 5 as libc::c_int as libc::c_float + (*s1).xmx;
    }
    s2 = (*de2).s;
    (*de2).x
        -= if 2 as libc::c_int as libc::c_float
            + (*s2).u.note.notes[0 as libc::c_int as usize].shac != 0.
        {
            (*s2).u.note.notes[0 as libc::c_int as usize].shac
                + 3 as libc::c_int as libc::c_float
        } else {
            *hw_tb.as_mut_ptr().offset((*s2).head as isize)
        };
}
unsafe extern "C" fn d_near(mut de: *mut deco_elt) {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut dd: *mut deco_def_s = 0 as *mut deco_def_s;
    let mut y: libc::c_int = 0;
    let mut up: libc::c_int = 0;
    s = (*de).s;
    dd = &mut *deco_def_tb.as_mut_ptr().offset((*de).t as isize) as *mut deco_def_s;
    if (*s).multi != 0 {
        up = ((*s).multi as libc::c_int > 0 as libc::c_int) as libc::c_int;
    } else {
        up = (((*s).stem as libc::c_int) < 0 as libc::c_int) as libc::c_int;
    }
    if up != 0 {
        y = (*s).ymx as libc::c_int;
    } else {
        y = (*s).ymn as libc::c_int - (*dd).h as libc::c_int;
    }
    if y > -(6 as libc::c_int) && y < 24 as libc::c_int {
        if up != 0 {
            y += 3 as libc::c_int;
        }
        y = (y + 6 as libc::c_int) / 6 as libc::c_int * 6 as libc::c_int
            - 6 as libc::c_int;
    }
    if up != 0 {
        (*s).ymx = (y + (*dd).h as libc::c_int) as libc::c_schar;
    } else {
        (*s).ymn = y as libc::c_schar;
    }
    (*de).y = y as libc::c_float;
    (*de).x = (*s).x;
    if (*s).type_0 as libc::c_int == 1 as libc::c_int {
        (*de).x
            += (*s)
                .u
                .note
                .notes[(if (*s).stem as libc::c_int >= 0 as libc::c_int {
                    0 as libc::c_int
                } else {
                    (*s).nhd as libc::c_int
                }) as usize]
                .shhd;
    }
    if *((*dd).name).offset(0 as libc::c_int as isize) as libc::c_int == 'd' as i32
        && (*s).nflags as libc::c_int >= -(1 as libc::c_int)
    {
        if up != 0 {
            if (*s).stem as libc::c_int > 0 as libc::c_int {
                (*de).x = ((*de).x as libc::c_double + 3.5f64) as libc::c_float;
            }
        } else if ((*s).stem as libc::c_int) < 0 as libc::c_int {
            (*de).x = ((*de).x as libc::c_double - 3.5f64) as libc::c_float;
        }
    }
    if (*dd).strx as libc::c_int != 0 as libc::c_int
        && (*dd).strx as libc::c_int != 255 as libc::c_int
    {
        (*de).x = (*s).x;
        (*de).y = (*s).y as libc::c_float;
    }
}
unsafe extern "C" fn d_pf(mut de: *mut deco_elt) {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut dd: *mut deco_def_s = 0 as *mut deco_def_s;
    let mut dd2: *mut deco_def_s = 0 as *mut deco_def_s;
    let mut x: libc::c_float = 0.;
    let mut x2: libc::c_float = 0.;
    let mut up: libc::c_int = 0;
    if (*de).flags as libc::c_int & 0x40 as libc::c_int != 0 {
        return;
    }
    if !((*de).start).is_null() {
        d_cresc(de);
        return;
    }
    s = (*de).s;
    dd = &mut *deco_def_tb.as_mut_ptr().offset((*de).t as isize) as *mut deco_def_s;
    (*de).val = ((*dd).wl as libc::c_int + (*dd).wr as libc::c_int) as libc::c_float;
    up = up_p(s, ((*s).posit).vol() as libc::c_int);
    if up != 0 {
        (*de).flags = ((*de).flags as libc::c_int | 0x2 as libc::c_int) as libc::c_uchar;
    }
    x = (*s).x - (*dd).wl as libc::c_int as libc::c_float;
    if !((*de).prev).is_null() && (*(*de).prev).s == s
        && ((*de).flags as libc::c_int ^ (*(*de).prev).flags as libc::c_int)
            & 0x2 as libc::c_int == 0 as libc::c_int
    {
        dd2 = &mut *deco_def_tb.as_mut_ptr().offset((*(*de).prev).t as isize)
            as *mut deco_def_s;
        if f_staff as libc::c_int & (1 as libc::c_int) << (*dd2).func as libc::c_int != 0
        {
            x2 = (*(*de).prev).x + (*(*de).prev).val + 4 as libc::c_int as libc::c_float;
            if x2 > x {
                x = x2;
            }
        }
    }
    (*de).x = x;
    (*de).y = y_get((*s).staff as libc::c_int, up, x, (*de).val);
    if up == 0 {
        (*de).y -= (*dd).h as libc::c_int as libc::c_float;
    }
}
unsafe extern "C" fn d_slide(mut de: *mut deco_elt) {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut m: libc::c_int = 0;
    let mut yc: libc::c_int = 0;
    let mut xc: libc::c_float = 0.;
    let mut dx: libc::c_float = 0.;
    s = (*de).s;
    yc = (*s).pits[0 as libc::c_int as usize] as libc::c_int;
    xc = 5 as libc::c_int as libc::c_float;
    if (*s).type_0 as libc::c_int == 1 as libc::c_int {
        m = 0 as libc::c_int;
        while m <= (*s).nhd as libc::c_int {
            if (*s).u.note.notes[m as usize].acc != 0 {
                dx = 4 as libc::c_int as libc::c_float
                    + (*s).u.note.notes[m as usize].shac;
            } else {
                dx = 5 as libc::c_int as libc::c_float
                    - (*s).u.note.notes[m as usize].shhd;
                match (*s).head as libc::c_int {
                    3 | 2 => {
                        dx = (dx as libc::c_double + 2.5f64) as libc::c_float;
                    }
                    _ => {}
                }
            }
            if (*s).pits[m as usize] as libc::c_int <= yc + 3 as libc::c_int && dx > xc {
                xc = dx;
            }
            m += 1;
            m;
        }
    }
    (*de).x = (*s).x - xc;
    (*de).y = (3 as libc::c_int * (yc - 18 as libc::c_int)) as libc::c_float;
}
unsafe extern "C" fn d_trill(mut de: *mut deco_elt) {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut dd: *mut deco_def_s = 0 as *mut deco_def_s;
    let mut staff: libc::c_int = 0;
    let mut up: libc::c_int = 0;
    let mut x: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    let mut w: libc::c_float = 0.;
    if (*de).flags as libc::c_int & 0x40 as libc::c_int != 0 {
        return;
    }
    s2 = (*de).s;
    if (*s2).sflags & 0x20000000 as libc::c_int as libc::c_uint != 0 {
        let mut de2: *mut deco_elt = 0 as *mut deco_elt;
        de2 = (*de).next;
        while !de2.is_null() {
            if (*de2).t as libc::c_int == (*de).t as libc::c_int
                && (*(*de2).s).time == (*s2).time
                && (*(*de2).s).staff as libc::c_int == (*s2).staff as libc::c_int
            {
                (*(*de2).s).sflags &= !(0x20000000 as libc::c_int) as libc::c_uint;
                (*de2).t = 0 as libc::c_int as libc::c_uchar;
            }
            de2 = (*de2).next;
        }
        (*s2).sflags &= !(0x20000000 as libc::c_int) as libc::c_uint;
    }
    s = (*(*de).start).s;
    x = (*s).x;
    if (*s).abc_type as libc::c_int == 4 as libc::c_int
        && (*s).u.note.dc.n as libc::c_int > 1 as libc::c_int
    {
        x += 10 as libc::c_int as libc::c_float;
    }
    staff = (*s2).staff as libc::c_int;
    (*de).staff = staff as libc::c_uchar;
    dd = &mut *deco_def_tb.as_mut_ptr().offset((*de).t as isize) as *mut deco_def_s;
    if (*dd).func as libc::c_int == 4 as libc::c_int {
        up = 0 as libc::c_int;
    } else if strcmp(
        ps_func_tb[(*dd).ps_func as usize],
        b"o8va\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        up = 1 as libc::c_int;
    } else {
        up = ((*s2).multi as libc::c_int >= 0 as libc::c_int) as libc::c_int;
    }
    if (*de).defl as libc::c_int & 0x2 as libc::c_int != 0 {
        w = (*de).x - x;
        if w < 20 as libc::c_int as libc::c_float {
            x = (*de).x - 20 as libc::c_int as libc::c_float
                - 3 as libc::c_int as libc::c_float;
            w = 20 as libc::c_int as libc::c_float;
        }
    } else {
        w = (*s2).x - x - 6 as libc::c_int as libc::c_float;
        if (*s2).abc_type as libc::c_int == 4 as libc::c_int {
            w -= 6 as libc::c_int as libc::c_float;
        }
        if w < 20 as libc::c_int as libc::c_float {
            x = (x as libc::c_double
                - (20 as libc::c_int as libc::c_float - w) as libc::c_double * 0.5f64)
                as libc::c_float;
            w = 20 as libc::c_int as libc::c_float;
        }
    }
    y = y_get(staff, up, x, w);
    if up != 0 {
        let mut stafft: libc::c_float = 0.;
        stafft = (staff_tb[(*s).staff as usize].topbar as libc::c_int + 2 as libc::c_int)
            as libc::c_float;
        if y < stafft {
            y = stafft;
        }
    } else {
        let mut staffb: libc::c_float = 0.;
        y -= (*dd).h as libc::c_int as libc::c_float;
        staffb = (staff_tb[(*s).staff as usize].botbar as libc::c_int - 2 as libc::c_int)
            as libc::c_float;
        if y > staffb {
            y = staffb;
        }
    }
    (*de).flags = ((*de).flags as libc::c_int & !(0x80 as libc::c_int)) as libc::c_uchar;
    (*de).flags = ((*de).flags as libc::c_int | 0x1 as libc::c_int) as libc::c_uchar;
    (*de).val = w;
    (*de).x = x;
    (*de).y = y;
    if up != 0 {
        y += (*dd).h as libc::c_int as libc::c_float;
    }
    y_set(staff, up, x, w, y);
    if up != 0 {
        (*s2).ymx = y as libc::c_schar;
        (*s).ymx = (*s2).ymx;
    } else {
        (*s2).ymn = y as libc::c_schar;
        (*s).ymn = (*s2).ymn;
    };
}
unsafe extern "C" fn d_upstaff(mut de: *mut deco_elt) {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut dd: *mut deco_def_s = 0 as *mut deco_def_s;
    let mut ps_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut x: libc::c_float = 0.;
    let mut yc: libc::c_float = 0.;
    let mut stafft: libc::c_float = 0.;
    let mut staffb: libc::c_float = 0.;
    let mut w: libc::c_float = 0.;
    let mut up: libc::c_int = 0;
    let mut inv: libc::c_int = 0;
    if (*de).flags as libc::c_int & 0x40 as libc::c_int != 0 {
        return;
    }
    if !((*de).start).is_null() {
        d_trill(de);
        return;
    }
    s = (*de).s;
    dd = &mut *deco_def_tb.as_mut_ptr().offset((*de).t as isize) as *mut deco_def_s;
    inv = 0 as libc::c_int;
    x = (*s).x;
    if (*s).type_0 as libc::c_int == 1 as libc::c_int {
        x
            += (*s)
                .u
                .note
                .notes[(if (*s).stem as libc::c_int >= 0 as libc::c_int {
                    0 as libc::c_int
                } else {
                    (*s).nhd as libc::c_int
                }) as usize]
                .shhd;
    }
    w = ((*dd).wl as libc::c_int + (*dd).wr as libc::c_int) as libc::c_float;
    stafft = (staff_tb[(*s).staff as usize].topbar as libc::c_int + 2 as libc::c_int)
        as libc::c_float;
    staffb = (staff_tb[(*s).staff as usize].botbar as libc::c_int - 2 as libc::c_int)
        as libc::c_float;
    up = -(1 as libc::c_int);
    if (*dd).func as libc::c_int == 4 as libc::c_int {
        up = 0 as libc::c_int;
    } else {
        match ((*s).posit).orn() as libc::c_int {
            1 => {
                up = 1 as libc::c_int;
            }
            2 => {
                up = 0 as libc::c_int;
            }
            _ => {}
        }
    }
    ps_name = ps_func_tb[(*dd).ps_func as usize];
    if strcmp(ps_name, b"accent\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
        || strcmp(ps_name, b"cpu\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        if up == 0
            || up < 0 as libc::c_int
                && (((*s).multi as libc::c_int) < 0 as libc::c_int
                    || (*s).multi as libc::c_int == 0 as libc::c_int
                        && (*s).stem as libc::c_int > 0 as libc::c_int)
        {
            yc = y_get(
                (*s).staff as libc::c_int,
                0 as libc::c_int,
                (*s).x - (*dd).wl as libc::c_int as libc::c_float,
                w,
            );
            if yc > staffb {
                yc = staffb;
            }
            yc -= (*dd).h as libc::c_int as libc::c_float;
            y_set(
                (*s).staff as libc::c_int,
                0 as libc::c_int,
                (*s).x,
                0 as libc::c_int as libc::c_float,
                yc,
            );
            inv = 1 as libc::c_int;
            (*s).ymn = yc as libc::c_schar;
        } else {
            yc = y_get(
                (*s).staff as libc::c_int,
                1 as libc::c_int,
                (*s).x,
                0 as libc::c_int as libc::c_float,
            );
            if yc < stafft {
                yc = stafft;
            }
            y_set(
                (*s).staff as libc::c_int,
                1 as libc::c_int,
                (*s).x - (*dd).wl as libc::c_int as libc::c_float,
                w,
                yc + (*dd).h as libc::c_int as libc::c_float,
            );
            (*s).ymx = (yc + (*dd).h as libc::c_int as libc::c_float) as libc::c_schar;
        }
    } else if strcmp(ps_name, b"brth\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
        || strcmp(ps_name, b"lphr\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        || strcmp(ps_name, b"mphr\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        || strcmp(ps_name, b"sphr\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        yc = stafft + 1 as libc::c_int as libc::c_float;
        if *ps_name.offset(0 as libc::c_int as isize) as libc::c_int == 'b' as i32 {
            if yc < (*s).ymx as libc::c_int as libc::c_float {
                yc = (*s).ymx as libc::c_float;
            }
        }
        s = (*s).ts_next;
        while !s.is_null() {
            if (*s).shrink != 0 as libc::c_int as libc::c_float {
                break;
            }
            s = (*s).ts_next;
        }
        if !s.is_null() {
            x = (x as libc::c_double + ((*s).x - x) as libc::c_double * 0.4f64)
                as libc::c_float;
        } else {
            x = (x as libc::c_double + (realwidth - x) as libc::c_double * 0.4f64)
                as libc::c_float;
        }
    } else {
        if strncmp(
            (*dd).name,
            b"invert\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            inv = 1 as libc::c_int;
        }
        if strcmp((*dd).name, b"invertedfermata\0" as *const u8 as *const libc::c_char)
            != 0 as libc::c_int
            && (up > 0 as libc::c_int
                || up < 0 as libc::c_int
                    && (*s).multi as libc::c_int >= 0 as libc::c_int)
        {
            yc = y_get(
                (*s).staff as libc::c_int,
                1 as libc::c_int,
                (*s).x - (*dd).wl as libc::c_int as libc::c_float,
                w,
            );
            if yc < stafft {
                yc = stafft;
            }
            y_set(
                (*s).staff as libc::c_int,
                1 as libc::c_int,
                (*s).x - (*dd).wl as libc::c_int as libc::c_float,
                w,
                yc + (*dd).h as libc::c_int as libc::c_float,
            );
            (*s).ymx = (yc + (*dd).h as libc::c_int as libc::c_float) as libc::c_schar;
        } else {
            yc = y_get(
                (*s).staff as libc::c_int,
                0 as libc::c_int,
                (*s).x - (*dd).wl as libc::c_int as libc::c_float,
                w,
            );
            if yc > staffb {
                yc = staffb;
            }
            yc -= (*dd).h as libc::c_int as libc::c_float;
            y_set(
                (*s).staff as libc::c_int,
                0 as libc::c_int,
                (*s).x - (*dd).wl as libc::c_int as libc::c_float,
                w,
                yc,
            );
            if strcmp((*dd).name, b"fermata\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                inv = 1 as libc::c_int;
            }
            (*s).ymn = yc as libc::c_schar;
        }
    }
    if inv != 0 {
        yc += (*dd).h as libc::c_int as libc::c_float;
        (*de)
            .flags = ((*de).flags as libc::c_int | 0x20 as libc::c_int) as libc::c_uchar;
    }
    (*de).x = x;
    (*de).y = yc;
}
#[no_mangle]
pub unsafe extern "C" fn deco_add(mut s: *mut libc::c_char) {
    let mut d: *mut u_deco = 0 as *mut u_deco;
    let mut l: libc::c_int = 0;
    l = strlen(s) as libc::c_int;
    d = malloc(
        (::core::mem::size_of::<u_deco>() as libc::c_ulong)
            .wrapping_sub(::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
            .wrapping_add(l as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut u_deco;
    strcpy(((*d).text).as_mut_ptr(), s);
    (*d).next = user_deco;
    user_deco = d;
}
unsafe extern "C" fn get_deco(mut name: *mut libc::c_char) -> libc::c_int {
    let mut dd: *mut deco_def_s = 0 as *mut deco_def_s;
    let mut ideco: libc::c_int = 0;
    ideco = 1 as libc::c_int;
    dd = &mut *deco_def_tb.as_mut_ptr().offset(1 as libc::c_int as isize)
        as *mut deco_def_s;
    while ideco < 128 as libc::c_int {
        if ((*dd).name).is_null() || strcmp((*dd).name, name) == 0 as libc::c_int {
            return ideco;
        }
        ideco += 1;
        ideco;
        dd = dd.offset(1);
        dd;
    }
    error(
        1 as libc::c_int,
        0 as *mut SYMBOL,
        b"Too many decorations\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return ideco;
}
unsafe extern "C" fn deco_build(
    mut name: *mut libc::c_char,
    mut text: *mut libc::c_char,
) -> libc::c_uchar {
    let mut dd: *mut deco_def_s = 0 as *mut deco_def_s;
    let mut c_func: libc::c_int = 0;
    let mut ideco: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut o: libc::c_int = 0;
    let mut wl: libc::c_int = 0;
    let mut wr: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut l: libc::c_uint = 0;
    let mut ps_x: libc::c_uint = 0;
    let mut strx: libc::c_uint = 0;
    let mut name2: [libc::c_char; 32] = [0; 32];
    let mut ps_func: [libc::c_char; 16] = [0; 16];
    if sscanf(
        text,
        b"%15s %d %15s %d %d %d%n\0" as *const u8 as *const libc::c_char,
        name2.as_mut_ptr(),
        &mut c_func as *mut libc::c_int,
        ps_func.as_mut_ptr(),
        &mut h as *mut libc::c_int,
        &mut wl as *mut libc::c_int,
        &mut wr as *mut libc::c_int,
        &mut n as *mut libc::c_int,
    ) != 6 as libc::c_int
    {
        error(
            1 as libc::c_int,
            0 as *mut SYMBOL,
            b"Invalid %%%%deco %s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            text,
        );
        return 128 as libc::c_int as libc::c_uchar;
    }
    if c_func as libc::c_uint > 10 as libc::c_int as libc::c_uint
        && (c_func < 32 as libc::c_int || c_func > 41 as libc::c_int)
    {
        error(
            1 as libc::c_int,
            0 as *mut SYMBOL,
            b"%%%%deco: bad C function index (%s)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            text,
        );
        return 128 as libc::c_int as libc::c_uchar;
    }
    if c_func == 5 as libc::c_int {
        c_func = 6 as libc::c_int;
    }
    if c_func == 7 as libc::c_int {
        c_func = 6 as libc::c_int;
    }
    if h < 0 as libc::c_int || wl < 0 as libc::c_int || wr < 0 as libc::c_int {
        error(
            1 as libc::c_int,
            0 as *mut SYMBOL,
            b"%%%%deco: cannot have a negative value (%s)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            text,
        );
        return 128 as libc::c_int as libc::c_uchar;
    }
    if h > 50 as libc::c_int || wl > 80 as libc::c_int || wr > 80 as libc::c_int {
        error(
            1 as libc::c_int,
            0 as *mut SYMBOL,
            b"%%%%deco: abnormal h/wl/wr value (%s)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            text,
        );
        return 128 as libc::c_int as libc::c_uchar;
    }
    text = text.offset(n as isize);
    while isspace(*text as libc::c_uchar as libc::c_int) != 0 {
        text = text.offset(1);
        text;
    }
    ideco = get_deco(name);
    if ideco == 128 as libc::c_int {
        return ideco as libc::c_uchar;
    }
    dd = &mut *deco_def_tb.as_mut_ptr().offset(ideco as isize) as *mut deco_def_s;
    ps_x = 0 as libc::c_int as libc::c_uint;
    while (ps_x as libc::c_ulong)
        < (::core::mem::size_of::<[*mut libc::c_char; 128]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
    {
        if (ps_func_tb[ps_x as usize]).is_null()
            || strcmp(ps_func_tb[ps_x as usize], ps_func.as_mut_ptr())
                == 0 as libc::c_int
        {
            break;
        }
        ps_x = ps_x.wrapping_add(1);
        ps_x;
    }
    if ps_x as libc::c_ulong
        == (::core::mem::size_of::<[*mut libc::c_char; 128]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
    {
        error(
            1 as libc::c_int,
            0 as *mut SYMBOL,
            b"Too many postscript functions\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return 128 as libc::c_int as libc::c_uchar;
    }
    if strcmp(text, name) == 0 as libc::c_int {
        strx = 255 as libc::c_int as libc::c_uint;
    } else if *text as libc::c_int == '\0' as i32 {
        strx = (if c_func == 6 as libc::c_int {
            255 as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_uint;
    } else {
        strx = 1 as libc::c_int as libc::c_uint;
        while (strx as libc::c_ulong)
            < (::core::mem::size_of::<[*mut libc::c_char; 32]>() as libc::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                )
        {
            if (str_tb[strx as usize]).is_null() {
                if *text as libc::c_int == '"' as i32 {
                    text = text.offset(1);
                    text;
                    l = strlen(text) as libc::c_uint;
                    str_tb[strx
                        as usize] = malloc(l as libc::c_ulong) as *mut libc::c_char;
                    memcpy(
                        str_tb[strx as usize] as *mut libc::c_void,
                        text as *const libc::c_void,
                        l.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ulong,
                    );
                    *(str_tb[strx as usize])
                        .offset(
                            l.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                        ) = '\0' as i32 as libc::c_char;
                } else {
                    str_tb[strx as usize] = strdup(text);
                }
                break;
            } else {
                if strcmp(str_tb[strx as usize], text) == 0 as libc::c_int {
                    break;
                }
                strx = strx.wrapping_add(1);
                strx;
            }
        }
        if strx as libc::c_ulong
            == (::core::mem::size_of::<[*mut libc::c_char; 32]>() as libc::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                )
        {
            error(
                1 as libc::c_int,
                0 as *mut SYMBOL,
                b"Too many decoration strings\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return 128 as libc::c_int as libc::c_uchar;
        }
    }
    if ((*dd).name).is_null() {
        (*dd).name = name;
    }
    (*dd)
        .func = (if strncmp(
        (*dd).name,
        b"head-\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        9 as libc::c_int
    } else {
        c_func
    }) as libc::c_uchar;
    if (ps_func_tb[ps_x as usize]).is_null() {
        if ps_func[0 as libc::c_int as usize] as libc::c_int == '-' as i32
            && ps_func[1 as libc::c_int as usize] as libc::c_int == '\0' as i32
        {
            ps_x = -(1 as libc::c_int) as libc::c_uint;
        } else {
            ps_func_tb[ps_x as usize] = strdup(ps_func.as_mut_ptr());
        }
    }
    (*dd).ps_func = ps_x as libc::c_schar;
    (*dd).h = h as libc::c_uchar;
    (*dd).wl = wl as libc::c_uchar;
    (*dd).wr = wr as libc::c_uchar;
    l = strlen(name) as libc::c_uint;
    if l == 0 as libc::c_int as libc::c_uint {
        return ideco as libc::c_uchar;
    }
    l = l.wrapping_sub(1);
    l;
    if *name.offset(l as isize) as libc::c_int == '(' as i32
        || *name.offset(l as isize) as libc::c_int == ')' as i32
            && (strchr(name, '(' as i32)).is_null()
    {
        let mut ddo: *mut deco_def_s = 0 as *mut deco_def_s;
        strx = 0 as libc::c_int as libc::c_uint;
        strcpy(name2.as_mut_ptr(), name);
        if *name.offset(l as isize) as libc::c_int == '(' as i32 {
            (*dd).flags = 0x40 as libc::c_int as libc::c_uchar;
            name2[l as usize] = ')' as i32 as libc::c_char;
        } else {
            (*dd).flags = 0x80 as libc::c_int as libc::c_uchar;
            name2[l as usize] = '(' as i32 as libc::c_char;
        }
        o = 1 as libc::c_int;
        ddo = &mut *deco_def_tb.as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut deco_def_s;
        while o < 128 as libc::c_int {
            if ((*ddo).name).is_null() {
                break;
            }
            if strcmp((*ddo).name, name2.as_mut_ptr()) == 0 as libc::c_int {
                if *name.offset(l as isize) as libc::c_int == '(' as i32 {
                    (*ddo).ld_start = ideco as libc::c_uchar;
                    (*dd).ld_end = o as libc::c_uchar;
                } else {
                    (*dd).ld_start = o as libc::c_uchar;
                    (*ddo).ld_end = ideco as libc::c_uchar;
                }
                break;
            } else {
                o += 1;
                o;
                ddo = ddo.offset(1);
                ddo;
            }
        }
        if o >= 128 as libc::c_int || ((*ddo).name).is_null() {
            deco_define(strdup(name2.as_mut_ptr()));
        }
    }
    (*dd).strx = strx as libc::c_uchar;
    return ideco as libc::c_uchar;
}
unsafe extern "C" fn set_feathered_beam(mut s1: *mut SYMBOL, mut accel: libc::c_int) {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut n: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut tt: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut a: libc::c_float = 0.;
    d = (*s1).dur;
    s2 = 0 as *mut SYMBOL;
    n = 1 as libc::c_int;
    s = (*s1).abc_next;
    while !s.is_null() {
        if (*s).dur != d || (*s).flags as libc::c_int & 0x4 as libc::c_int != 0 {
            break;
        }
        s2 = s;
        n += 1;
        n;
        s = (*s).abc_next;
    }
    if s2.is_null() {
        return;
    }
    b = d / 2 as libc::c_int;
    a = d as libc::c_float / (n - 1 as libc::c_int) as libc::c_float;
    tt = d * n;
    t = 0 as libc::c_int;
    if accel != 0 {
        s = s1;
        i = n - 1 as libc::c_int;
        while s != s2 {
            d = lroundf(a * i as libc::c_float) as libc::c_int + b;
            (*s).dur = d;
            t += d;
            s = (*s).abc_next;
            i -= 1;
            i;
        }
    } else {
        s = s1;
        i = 0 as libc::c_int;
        while s != s2 {
            d = lroundf(a * i as libc::c_float) as libc::c_int + b;
            (*s).dur = d;
            t += d;
            s = (*s).abc_next;
            i += 1;
            i;
        }
    }
    (*s2).dur = tt - t;
}
unsafe extern "C" fn deco_define(mut name: *mut libc::c_char) -> libc::c_uchar {
    let mut d: *mut u_deco = 0 as *mut u_deco;
    let mut ideco: libc::c_uchar = 0;
    let mut l: libc::c_int = 0;
    l = strlen(name) as libc::c_int;
    d = user_deco;
    while !d.is_null() {
        if strncmp(((*d).text).as_mut_ptr(), name, l as libc::c_ulong)
            == 0 as libc::c_int && (*d).text[l as usize] as libc::c_int == ' ' as i32
        {
            return deco_build(name, ((*d).text).as_mut_ptr());
        }
        d = (*d).next;
    }
    ideco = 0 as libc::c_int as libc::c_uchar;
    while !(std_deco_tb[ideco as usize]).is_null() {
        if strncmp(std_deco_tb[ideco as usize], name, l as libc::c_ulong)
            == 0 as libc::c_int
            && *(std_deco_tb[ideco as usize]).offset(l as isize) as libc::c_int
                == ' ' as i32
        {
            return deco_build(name, std_deco_tb[ideco as usize]);
        }
        ideco = ideco.wrapping_add(1);
        ideco;
    }
    return 128 as libc::c_int as libc::c_uchar;
}
unsafe extern "C" fn deco_intern(
    mut ideco: libc::c_uchar,
    mut s: *mut SYMBOL,
) -> libc::c_uchar {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    name = if (ideco as libc::c_int) < 128 as libc::c_int {
        deco[ideco as usize]
    } else {
        parse.deco_tb[(ideco as libc::c_int - 128 as libc::c_int) as usize]
    };
    if name.is_null() {
        error(
            1 as libc::c_int,
            s,
            b"Bad character '%c'\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            ideco as libc::c_int,
        );
        return 0 as libc::c_int as libc::c_uchar;
    }
    ideco = 1 as libc::c_int as libc::c_uchar;
    while (ideco as libc::c_int) < 128 as libc::c_int {
        if (deco_def_tb[ideco as usize].name).is_null() {
            ideco = deco_define(name);
            break;
        } else {
            if strcmp(deco_def_tb[ideco as usize].name, name) == 0 as libc::c_int {
                break;
            }
            ideco = ideco.wrapping_add(1);
            ideco;
        }
    }
    if ideco as libc::c_int == 128 as libc::c_int {
        if cfmt.decoerr != 0 {
            error(
                1 as libc::c_int,
                s,
                b"Decoration !%s! not defined\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                name,
            );
        }
        ideco = 0 as libc::c_int as libc::c_uchar;
    }
    return ideco;
}
#[no_mangle]
pub unsafe extern "C" fn deco_cnv(
    mut dc: *mut decos,
    mut s: *mut SYMBOL,
    mut prev: *mut SYMBOL,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut dd: *mut deco_def_s = 0 as *mut deco_def_s;
    let mut ideco: libc::c_uchar = 0;
    static mut must_note_fmt: [libc::c_char; 28] = unsafe {
        *::core::mem::transmute::<
            &[u8; 28],
            &mut [libc::c_char; 28],
        >(b"Deco !%s! must be on a note\0")
    };
    static mut no_head_fmt: [libc::c_char; 25] = unsafe {
        *::core::mem::transmute::<
            &[u8; 25],
            &mut [libc::c_char; 25],
        >(b"!%s! cannot be on a head\0")
    };
    let mut current_block_75: u64;
    i = (*dc).n as libc::c_int;
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int) {
            break;
        }
        ideco = (*dc).tm[i as usize].t;
        if ideco as libc::c_int == 0 as libc::c_int {
            continue;
        }
        ideco = deco_intern(ideco, s);
        (*dc).tm[i as usize].t = ideco;
        if ideco as libc::c_int == 0 as libc::c_int {
            continue;
        }
        dd = &mut *deco_def_tb.as_mut_ptr().offset(ideco as isize) as *mut deco_def_s;
        m = (*dc).tm[i as usize].m as libc::c_int;
        match (*dd).func as libc::c_int {
            5 | 6 | 7 | 2 => {
                if m >= 0 as libc::c_int {
                    error(1 as libc::c_int, s, no_head_fmt.as_mut_ptr(), (*dd).name);
                    current_block_75 = 8937240710477387595;
                } else {
                    if (*dd).func as libc::c_int != 2 as libc::c_int {
                        continue;
                    }
                    current_block_75 = 11575586934098241933;
                }
            }
            0 => {
                current_block_75 = 11575586934098241933;
            }
            1 => {
                current_block_75 = 7569392262172777061;
            }
            8 => {
                if (*s).abc_type as libc::c_int != 4 as libc::c_int {
                    error(
                        1 as libc::c_int,
                        s,
                        b"!%s! must be on a note\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        (*dd).name,
                    );
                } else {
                    if m < 0 as libc::c_int {
                        (*dc).tm[i as usize].m = (*s).nhd as libc::c_schar;
                    }
                    continue;
                }
                current_block_75 = 8937240710477387595;
            }
            9 => {
                if (*s).abc_type as libc::c_int != 4 as libc::c_int
                    && (*s).abc_type as libc::c_int != 5 as libc::c_int
                {
                    error(
                        1 as libc::c_int,
                        s,
                        b"!%s! must be on a note or a rest\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        (*dd).name,
                    );
                } else if m >= 0 as libc::c_int {
                    (*s)
                        .u
                        .note
                        .notes[m as usize]
                        .invisible = 1 as libc::c_int as libc::c_char;
                    continue;
                } else {
                    (*dc).tm[i as usize].m = 0 as libc::c_int as libc::c_schar;
                    (*s)
                        .u
                        .note
                        .notes[0 as libc::c_int as usize]
                        .invisible = 1 as libc::c_int as libc::c_char;
                    n = (*dc).n as libc::c_int;
                    m = 1 as libc::c_int;
                    while m <= (*s).nhd as libc::c_int {
                        if n >= 32 as libc::c_int {
                            error(
                                1 as libc::c_int,
                                s,
                                b"Too many decorations\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                            );
                            break;
                        } else {
                            (*dc).tm[n as usize].t = ideco;
                            let fresh2 = n;
                            n = n + 1;
                            (*dc).tm[fresh2 as usize].m = m as libc::c_schar;
                            (*s)
                                .u
                                .note
                                .notes[m as usize]
                                .invisible = 1 as libc::c_int as libc::c_char;
                            m += 1;
                            m;
                        }
                    }
                    (*dc).n = n as libc::c_char;
                    continue;
                }
                current_block_75 = 8937240710477387595;
            }
            32 => {
                if m < 0 as libc::c_int {
                    (*s)
                        .flags = ((*s).flags as libc::c_int | 0x2 as libc::c_int)
                        as libc::c_ushort;
                } else {
                    (*s)
                        .u
                        .note
                        .notes[m as usize]
                        .invisible = 1 as libc::c_int as libc::c_char;
                }
                current_block_75 = 8937240710477387595;
            }
            33 => {
                (*s).sflags |= 0x400 as libc::c_int as libc::c_uint;
                current_block_75 = 8937240710477387595;
            }
            34 => {
                if (*s).abc_type as libc::c_int != 4 as libc::c_int || prev.is_null()
                    || (*prev).abc_type as libc::c_int != 4 as libc::c_int
                    || (*s).dur != (*prev).dur
                {
                    error(
                        1 as libc::c_int,
                        s,
                        b"!%s! must be on the last of a couple of notes\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        (*dd).name,
                    );
                } else {
                    if strlen((*dd).name) != 5 as libc::c_int as libc::c_ulong {
                        n = 0 as libc::c_int;
                    } else {
                        n = *((*dd).name).offset(4 as libc::c_int as isize)
                            as libc::c_int - '0' as i32;
                    }
                    if n <= 0 as libc::c_int || n > 4 as libc::c_int {
                        error(
                            1 as libc::c_int,
                            s,
                            b"bad definition of !%s!\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            (*dd).name,
                        );
                    } else {
                        (*s).sflags
                            |= (0x80 as libc::c_int | 0x10 as libc::c_int)
                                as libc::c_uint;
                        (*s).sflags &= !(0x2 as libc::c_int) as libc::c_uint;
                        (*prev).sflags
                            |= (0x80 as libc::c_int | 0x2 as libc::c_int)
                                as libc::c_uint;
                        (*prev).sflags &= !(0x10 as libc::c_int) as libc::c_uint;
                        (*prev).aux = n as libc::c_short;
                        (*s).aux = (*prev).aux;
                        j = 0 as libc::c_int;
                        while j <= (*s).nhd as libc::c_int {
                            (*s).u.note.notes[j as usize].len *= 2 as libc::c_int;
                            j += 1;
                            j;
                        }
                        j = 0 as libc::c_int;
                        while j <= (*prev).nhd as libc::c_int {
                            (*prev).u.note.notes[j as usize].len *= 2 as libc::c_int;
                            j += 1;
                            j;
                        }
                    }
                }
                current_block_75 = 8937240710477387595;
            }
            35 => {
                if (*s).abc_type as libc::c_int != 4 as libc::c_int {
                    error(1 as libc::c_int, s, must_note_fmt.as_mut_ptr(), (*dd).name);
                } else {
                    (*s).sflags |= 0x200 as libc::c_int as libc::c_uint;
                }
                current_block_75 = 8937240710477387595;
            }
            36 => {
                if (*s).abc_type as libc::c_int != 4 as libc::c_int {
                    error(1 as libc::c_int, s, must_note_fmt.as_mut_ptr(), (*dd).name);
                } else if strlen((*dd).name) != 7 as libc::c_int as libc::c_ulong
                    || *((*dd).name).offset(6 as libc::c_int as isize) as libc::c_int
                        != '1' as i32
                        && *((*dd).name).offset(6 as libc::c_int as isize) as libc::c_int
                            != '2' as i32
                {
                    error(
                        1 as libc::c_int,
                        s,
                        b"bad definition of !%s!\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        (*dd).name,
                    );
                } else {
                    (*s).sflags
                        |= (if *((*dd).name).offset(6 as libc::c_int as isize)
                            as libc::c_int == '1' as i32
                        {
                            0x4 as libc::c_int
                        } else {
                            0x8 as libc::c_int
                        }) as libc::c_uint;
                }
                current_block_75 = 8937240710477387595;
            }
            37 => {
                (*s).sflags |= 0x8000 as libc::c_int as libc::c_uint;
                current_block_75 = 8937240710477387595;
            }
            38 => {
                if (*s).abc_type as libc::c_int != 4 as libc::c_int {
                    error(1 as libc::c_int, s, must_note_fmt.as_mut_ptr(), (*dd).name);
                } else {
                    n = strlen((*dd).name) as libc::c_int;
                    if n > 3 as libc::c_int {
                        error(
                            1 as libc::c_int,
                            s,
                            b"bad definition of !%s!\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            (*dd).name,
                        );
                    } else {
                        (*s).sflags |= 0x800000 as libc::c_int as libc::c_uint;
                        (*s).aux = n as libc::c_short;
                    }
                }
                current_block_75 = 8937240710477387595;
            }
            39 => {
                if (*s).abc_type as libc::c_int != 4 as libc::c_int {
                    error(1 as libc::c_int, s, must_note_fmt.as_mut_ptr(), (*dd).name);
                } else if strlen((*dd).name) < 6 as libc::c_int as libc::c_ulong {
                    error(
                        1 as libc::c_int,
                        s,
                        b"bad definition of !%s!\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        (*dd).name,
                    );
                } else {
                    (*s).sflags |= 0x10000 as libc::c_int as libc::c_uint;
                    set_feathered_beam(
                        s,
                        (*((*dd).name).offset(5 as libc::c_int as isize) as libc::c_int
                            == 'a' as i32) as libc::c_int,
                    );
                }
                current_block_75 = 8937240710477387595;
            }
            40 => {
                if (*s).abc_type as libc::c_int != 4 as libc::c_int {
                    error(1 as libc::c_int, s, must_note_fmt.as_mut_ptr(), (*dd).name);
                } else {
                    (*s)
                        .flags = ((*s).flags as libc::c_int | 0x8 as libc::c_int)
                        as libc::c_ushort;
                }
                current_block_75 = 8937240710477387595;
            }
            41 => {
                (*s)
                    .flags = ((*s).flags as libc::c_int | 0x200 as libc::c_int)
                    as libc::c_ushort;
                (*s).sflags |= 0x8000 as libc::c_int as libc::c_uint;
                current_block_75 = 8937240710477387595;
            }
            _ => {
                if strlen((*dd).name) >= 4 as libc::c_int as libc::c_ulong
                    && *((*dd).name).offset(0 as libc::c_int as isize) as libc::c_int
                        == '8' as i32
                    && *((*dd).name).offset(1 as libc::c_int as isize) as libc::c_int
                        == 'v' as i32
                    && *((*dd).name).offset(4 as libc::c_int as isize) as libc::c_int
                        == '\0' as i32
                {
                    if *((*dd).name).offset(3 as libc::c_int as isize) as libc::c_int
                        == '(' as i32
                    {
                        if *((*dd).name).offset(2 as libc::c_int as isize) as libc::c_int
                            == 'a' as i32
                        {
                            (*curvoice).ottava = -(7 as libc::c_int) as libc::c_schar;
                        } else if *((*dd).name).offset(2 as libc::c_int as isize)
                            as libc::c_int == 'b' as i32
                        {
                            (*curvoice).ottava = 7 as libc::c_int as libc::c_schar;
                        }
                    } else if *((*dd).name).offset(3 as libc::c_int as isize)
                        as libc::c_int == ')' as i32
                    {
                        if *((*dd).name).offset(2 as libc::c_int as isize) as libc::c_int
                            == 'a' as i32
                            || *((*dd).name).offset(2 as libc::c_int as isize)
                                as libc::c_int == 'b' as i32
                        {
                            (*curvoice).ottava = 0 as libc::c_int as libc::c_schar;
                        }
                    }
                    (*s).sflags |= 0x20000000 as libc::c_int as libc::c_uint;
                }
                continue;
            }
        }
        match current_block_75 {
            11575586934098241933 => {
                if (*dd).func as libc::c_int == 0 as libc::c_int
                    && (*s).abc_type as libc::c_int == 6 as libc::c_int
                    && strcmp((*dd).name, b"dot\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                {
                    (*s).u.bar.dotted = 1 as libc::c_int as libc::c_char;
                    current_block_75 = 8937240710477387595;
                } else {
                    current_block_75 = 7569392262172777061;
                }
            }
            _ => {}
        }
        match current_block_75 {
            7569392262172777061 => {
                if !((*s).abc_type as libc::c_int != 4 as libc::c_int
                    && (*s).abc_type as libc::c_int != 5 as libc::c_int)
                {
                    continue;
                }
                error(
                    1 as libc::c_int,
                    s,
                    b"!%s! must be on a note or a rest\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    (*dd).name,
                );
            }
            _ => {}
        }
        (*dc).tm[i as usize].t = 0 as libc::c_int as libc::c_uchar;
    };
}
#[no_mangle]
pub unsafe extern "C" fn deco_update(mut s: *mut SYMBOL, mut dx: libc::c_float) {
    let mut de: *mut deco_elt = 0 as *mut deco_elt;
    de = deco_head;
    while !de.is_null() {
        if (*de).s == s {
            while !de.is_null() && (*de).s == s {
                (*de).x += dx;
                de = (*de).next;
            }
            break;
        } else {
            de = (*de).next;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn deco_width(mut s: *mut SYMBOL) -> libc::c_float {
    let mut dc: *mut decos = 0 as *mut decos;
    let mut i: libc::c_int = 0;
    let mut wl: libc::c_float = 0.;
    wl = 0 as libc::c_int as libc::c_float;
    if (*s).type_0 as libc::c_int == 3 as libc::c_int {
        dc = &mut (*s).u.bar.dc;
    } else {
        dc = &mut (*s).u.note.dc;
    }
    i = (*dc).n as libc::c_int;
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int) {
            break;
        }
        let mut dd: *mut deco_def_s = 0 as *mut deco_def_s;
        dd = &mut *deco_def_tb
            .as_mut_ptr()
            .offset((*((*dc).tm).as_mut_ptr().offset(i as isize)).t as isize)
            as *mut deco_def_s;
        match (*dd).func as libc::c_int {
            1 => {
                if wl < 7 as libc::c_int as libc::c_float {
                    wl = 7 as libc::c_int as libc::c_float;
                }
            }
            2 => {
                if wl < 14 as libc::c_int as libc::c_float {
                    wl = 14 as libc::c_int as libc::c_float;
                }
            }
            _ => {}
        }
    }
    if wl != 0 as libc::c_int as libc::c_float && !((*s).prev).is_null()
        && (*(*s).prev).type_0 as libc::c_int == 3 as libc::c_int
    {
        wl -= 3 as libc::c_int as libc::c_float;
    }
    return wl;
}
#[no_mangle]
pub unsafe extern "C" fn draw_all_deco() {
    let mut de: *mut deco_elt = 0 as *mut deco_elt;
    let mut dd: *mut deco_def_s = 0 as *mut deco_def_s;
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut f: libc::c_int = 0;
    let mut staff: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut gl: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut x: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    let mut y2: libc::c_float = 0.;
    let mut ym: libc::c_float = 0.;
    let mut ymid: [libc::c_float; 32] = [0.; 32];
    if cfmt.dynalign == 0 {
        staff = nstaff;
        y = staff_tb[staff as usize].y;
        loop {
            staff -= 1;
            if !(staff >= 0 as libc::c_int) {
                break;
            }
            y2 = staff_tb[staff as usize].y;
            ymid[staff
                as usize] = ((y + 24 as libc::c_int as libc::c_float + y2)
                as libc::c_double * 0.5f64) as libc::c_float;
            y = y2;
        }
    }
    let mut current_block_92: u64;
    de = deco_head;
    while !de.is_null() {
        if !((*de).t as libc::c_int == 0 as libc::c_int) {
            dd = &mut *deco_def_tb.as_mut_ptr().offset((*de).t as isize)
                as *mut deco_def_s;
            if !((*dd).ld_end as libc::c_int != 0 as libc::c_int) {
                f = (*dd).ps_func as libc::c_int;
                if !(f < 0 as libc::c_int) {
                    gl = ps_func_tb[f as usize];
                    p = strchr(gl, '/' as i32);
                    if !p.is_null() {
                        if (*(*de).s).stem as libc::c_int >= 0 as libc::c_int {
                            l = p.offset_from(gl) as libc::c_long as libc::c_int;
                        } else {
                            gl = p.offset(1 as libc::c_int as isize);
                            l = strlen(gl) as libc::c_int;
                        }
                    } else {
                        l = strlen(gl) as libc::c_int;
                    }
                    s = (*de).s;
                    if f_staff as libc::c_int
                        & (1 as libc::c_int) << (*dd).func as libc::c_int != 0
                    {
                        set_sscale(-(1 as libc::c_int));
                    } else {
                        set_scale(s);
                    }
                    staff = (*de).staff as libc::c_int;
                    x = (*de).x;
                    y = (*de).y + staff_tb[staff as usize].y;
                    if (*de).m as libc::c_int >= 0 as libc::c_int {
                        x
                            += (*s).u.note.notes[(*de).m as usize].shhd
                                * staff_tb[staff as usize].staffscale;
                    } else if f_staff as libc::c_int
                        & (1 as libc::c_int) << (*dd).func as libc::c_int != 0
                        && cfmt.dynalign == 0
                        && ((*de).flags as libc::c_int & 0x2 as libc::c_int != 0
                            && staff > 0 as libc::c_int
                            || (*de).flags as libc::c_int & 0x2 as libc::c_int == 0
                                && staff < nstaff)
                    {
                        if (*de).flags as libc::c_int & 0x2 as libc::c_int != 0 {
                            staff -= 1;
                            ym = ymid[staff as usize];
                        } else {
                            let fresh3 = staff;
                            staff = staff + 1;
                            ym = ymid[fresh3 as usize];
                        }
                        ym = (ym as libc::c_double
                            - (*dd).h as libc::c_int as libc::c_double * 0.5f64)
                            as libc::c_float;
                        if (*de).flags as libc::c_int & 0x2 as libc::c_int != 0 && y < ym
                            || (*de).flags as libc::c_int & 0x2 as libc::c_int == 0
                                && y > ym
                        {
                            y2 = y_get(
                                staff,
                                ((*de).flags as libc::c_int & 0x2 as libc::c_int == 0)
                                    as libc::c_int,
                                (*de).x,
                                (*de).val,
                            ) + staff_tb[staff as usize].y;
                            if (*de).flags as libc::c_int & 0x2 as libc::c_int != 0 {
                                y2 -= (*dd).h as libc::c_int as libc::c_float;
                            }
                            if (*de).flags as libc::c_int & 0x2 as libc::c_int != 0
                                && y2 > ym
                                || (*de).flags as libc::c_int & 0x2 as libc::c_int == 0
                                    && y2 < ym
                            {
                                y = ym;
                                y_set(
                                    staff,
                                    (*de).flags as libc::c_int & 0x2 as libc::c_int,
                                    (*de).x,
                                    (*de).val,
                                    (if (*de).flags as libc::c_int & 0x2 as libc::c_int != 0 {
                                        y + (*dd).h as libc::c_int as libc::c_float
                                    } else {
                                        y
                                    }) - staff_tb[staff as usize].y,
                                );
                            }
                        }
                    }
                    set_defl((*de).defl as libc::c_int);
                    if (*de).flags as libc::c_int & 0x1 as libc::c_int != 0 {
                        if (*dd).func as libc::c_int != 2 as libc::c_int
                            || voice_tb[(*s).voice as usize].scale
                                != 1 as libc::c_int as libc::c_float
                        {
                            putx((*de).val);
                        } else {
                            putf((*de).val);
                        }
                    }
                    if (*dd).strx as libc::c_int != 0 as libc::c_int {
                        let mut p_0: *mut libc::c_char = 0 as *mut libc::c_char;
                        let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
                        y = (y as libc::c_double
                            + (*dd).h as libc::c_int as libc::c_double * 0.2f64)
                            as libc::c_float;
                        if (*dd).strx as libc::c_int == 255 as libc::c_int {
                            p_0 = (*dd).name;
                            current_block_92 = 1623252117315916725;
                        } else {
                            p_0 = str_tb[(*dd).strx as usize];
                            if *p_0 as libc::c_int == '@' as i32 {
                                let mut dx: libc::c_float = 0.;
                                let mut dy: libc::c_float = 0.;
                                let mut n: libc::c_int = 0;
                                if sscanf(
                                    p_0,
                                    b"@%f,%f%n\0" as *const u8 as *const libc::c_char,
                                    &mut dx as *mut libc::c_float,
                                    &mut dy as *mut libc::c_float,
                                    &mut n as *mut libc::c_int,
                                ) == 2 as libc::c_int
                                {
                                    x += dx;
                                    y += dy;
                                    p_0 = p_0.offset(n as isize);
                                }
                                str_font(ANNOTATIONFONT as libc::c_int);
                                outft = -(1 as libc::c_int);
                                putxy(x, y);
                                a2b(
                                    b"M\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                );
                                put_str(p_0, 0 as libc::c_int);
                                current_block_92 = 1917311967535052937;
                            } else {
                                current_block_92 = 1623252117315916725;
                            }
                        }
                        match current_block_92 {
                            1917311967535052937 => {}
                            _ => {
                                a2b(
                                    b"(\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                );
                                q = p_0;
                                while *p_0 as libc::c_int != '\0' as i32 {
                                    if *p_0 as libc::c_int == '(' as i32
                                        || *p_0 as libc::c_int == ')' as i32
                                    {
                                        if p_0 != q {
                                            a2b(
                                                b"%.*s\0" as *const u8 as *const libc::c_char
                                                    as *mut libc::c_char,
                                                p_0.offset_from(q) as libc::c_long as libc::c_int,
                                                q,
                                            );
                                        }
                                        a2b(
                                            b"\\\0" as *const u8 as *const libc::c_char
                                                as *mut libc::c_char,
                                        );
                                        q = p_0;
                                    }
                                    p_0 = p_0.offset(1);
                                    p_0;
                                }
                                if p_0 != q {
                                    a2b(
                                        b"%.*s\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                        p_0.offset_from(q) as libc::c_long as libc::c_int,
                                        q,
                                    );
                                }
                                a2b(
                                    b")\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                );
                                current_block_92 = 2723324002591448311;
                            }
                        }
                    } else {
                        current_block_92 = 2723324002591448311;
                    }
                    match current_block_92 {
                        1917311967535052937 => {}
                        _ => {
                            putxy(x, y);
                            if (*de).flags as libc::c_int & 0x80 as libc::c_int != 0 {
                                x = (*(*de).start).x;
                                y = (*(*de).start).y
                                    + staff_tb[(*(*de).start).staff as usize].y;
                                if x > (*de).x - 20 as libc::c_int as libc::c_float {
                                    x = (*de).x - 20 as libc::c_int as libc::c_float;
                                }
                                putxy(x, y);
                            }
                            if (*de).flags as libc::c_int & 0x20 as libc::c_int != 0 {
                                a2b(
                                    b"gsave 1 -1 scale neg %.*s grestore\n\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                    l,
                                    gl,
                                );
                            } else {
                                a2b(
                                    b"%.*s\n\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                    l,
                                    gl,
                                );
                            }
                        }
                    }
                }
            }
        }
        de = (*de).next;
    }
    set_sscale(-(1 as libc::c_int));
    set_color(0 as libc::c_int);
}
unsafe extern "C" fn deco_create(mut s: *mut SYMBOL, mut dc: *mut decos) {
    let mut k: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut posit: libc::c_int = 0;
    let mut ideco: libc::c_uchar = 0;
    let mut dd: *mut deco_def_s = 0 as *mut deco_def_s;
    let mut de: *mut deco_elt = 0 as *mut deco_elt;
    k = 0 as libc::c_int;
    while k < (*dc).n as libc::c_int {
        m = (*dc).tm[k as usize].m as libc::c_int;
        ideco = (*dc).tm[k as usize].t;
        if !(ideco as libc::c_int == 0 as libc::c_int) {
            dd = &mut *deco_def_tb.as_mut_ptr().offset(ideco as isize)
                as *mut deco_def_s;
            match (*dd).func as libc::c_int {
                3 | 4 => {
                    posit = ((*s).posit).orn() as libc::c_int;
                }
                6 => {
                    posit = ((*s).posit).vol() as libc::c_int;
                }
                7 => {
                    posit = ((*s).posit).dyn_0() as libc::c_int;
                }
                _ => {
                    posit = 0 as libc::c_int;
                }
            }
            if posit == 0x3 as libc::c_int {
                (*dc).tm[k as usize].t = 0 as libc::c_int as libc::c_uchar;
            } else {
                de = getarena(
                    ::core::mem::size_of::<deco_elt>() as libc::c_ulong as libc::c_int,
                ) as *mut deco_elt;
                memset(
                    de as *mut libc::c_void,
                    0 as libc::c_int,
                    ::core::mem::size_of::<deco_elt>() as libc::c_ulong,
                );
                (*de).prev = deco_tail;
                if deco_tail.is_null() {
                    deco_head = de;
                } else {
                    (*deco_tail).next = de;
                }
                deco_tail = de;
                (*de).s = s;
                (*de)
                    .t = dd.offset_from(deco_def_tb.as_mut_ptr()) as libc::c_long
                    as libc::c_uchar;
                (*de).staff = (*s).staff;
                (*de).m = m as libc::c_schar;
                if (*dd).flags as libc::c_int & 0x40 as libc::c_int != 0 {
                    (*de)
                        .flags = ((*de).flags as libc::c_int | 0x40 as libc::c_int)
                        as libc::c_uchar;
                } else if (*dd).flags as libc::c_int & 0x80 as libc::c_int != 0 {
                    (*de)
                        .flags = ((*de).flags as libc::c_int | 0x80 as libc::c_int)
                        as libc::c_uchar;
                    (*de).defl = 0x1 as libc::c_int as libc::c_uchar;
                }
                if cfmt.setdefl != 0 && (*s).stem as libc::c_int >= 0 as libc::c_int {
                    (*de)
                        .defl = ((*de).defl as libc::c_int | 0x4 as libc::c_int)
                        as libc::c_uchar;
                }
                if m >= 0 as libc::c_int {
                    (*de).x = (*s).x;
                    (*de)
                        .y = (3 as libc::c_int
                        * ((*s).pits[m as usize] as libc::c_int - 18 as libc::c_int))
                        as libc::c_float;
                } else if !(f_near as libc::c_int
                    & (1 as libc::c_int) << (*dd).func as libc::c_int == 0)
                {
                    (func_tb[(*dd).func as usize])
                        .expect("non-null function pointer")(de);
                }
            }
        }
        k += 1;
        k;
    }
}
unsafe extern "C" fn ll_deco() {
    let mut de: *mut deco_elt = 0 as *mut deco_elt;
    let mut de2: *mut deco_elt = 0 as *mut deco_elt;
    let mut tail: *mut deco_elt = 0 as *mut deco_elt;
    let mut dd: *mut deco_def_s = 0 as *mut deco_def_s;
    let mut t: libc::c_int = 0;
    let mut voice: libc::c_int = 0;
    let mut staff: libc::c_int = 0;
    tail = deco_tail;
    if tail.is_null() {
        return;
    }
    de = deco_head;
    loop {
        t = (*de).t as libc::c_int;
        dd = &mut *deco_def_tb.as_mut_ptr().offset(t as isize) as *mut deco_def_s;
        if (*de).flags as libc::c_int & 0x40 as libc::c_int == 0 {
            if de == tail {
                break;
            }
        } else {
            t = (*dd).ld_end as libc::c_int;
            voice = (*(*de).s).voice as libc::c_int;
            de2 = (*de).next;
            while !de2.is_null() {
                if ((*de2).start).is_null() && (*de2).t as libc::c_int == t
                    && (*(*de2).s).voice as libc::c_int == voice
                {
                    break;
                }
                de2 = (*de2).next;
            }
            if de2.is_null() {
                staff = (*(*de).s).staff as libc::c_int;
                de2 = (*de).next;
                while !de2.is_null() {
                    if ((*de2).start).is_null() && (*de2).t as libc::c_int == t
                        && (*(*de2).s).staff as libc::c_int == staff
                    {
                        break;
                    }
                    de2 = (*de2).next;
                }
            }
            if de2.is_null() {
                de2 = getarena(
                    ::core::mem::size_of::<deco_elt>() as libc::c_ulong as libc::c_int,
                ) as *mut deco_elt;
                memset(
                    de2 as *mut libc::c_void,
                    0 as libc::c_int,
                    ::core::mem::size_of::<deco_elt>() as libc::c_ulong,
                );
                (*de2).prev = deco_tail;
                (*deco_tail).next = de2;
                deco_tail = de2;
                (*de2).s = (*de).s;
                (*de2).t = t as libc::c_uchar;
                (*de2).defl = 0x2 as libc::c_int as libc::c_uchar;
                (*de2).flags = 0x80 as libc::c_int as libc::c_uchar;
                (*de2).x = realwidth - 6 as libc::c_int as libc::c_float;
                (*de2).y = (*(*de).s).y as libc::c_float;
                (*de2).m = (*de).m;
            }
            (*de2).start = de;
            (*de2)
                .defl = ((*de2).defl as libc::c_int & !(0x1 as libc::c_int))
                as libc::c_uchar;
            if (*dd).func as libc::c_int == 8 as libc::c_int {
                d_gliss(de2);
            }
            if de == tail {
                break;
            }
        }
        de = (*de).next;
    }
    de2 = deco_head;
    loop {
        if (*de2).flags as libc::c_int & 0x80 as libc::c_int == 0
            || !((*de2).start).is_null()
        {
            if de2 == tail {
                break;
            }
        } else {
            t = (*de2).t as libc::c_int;
            dd = &mut *deco_def_tb.as_mut_ptr().offset(t as isize) as *mut deco_def_s;
            de = getarena(
                ::core::mem::size_of::<deco_elt>() as libc::c_ulong as libc::c_int,
            ) as *mut deco_elt;
            memset(
                de as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<deco_elt>() as libc::c_ulong,
            );
            (*de).prev = deco_tail;
            (*deco_tail).next = de;
            deco_tail = de;
            (*de).s = prev_scut((*de2).s);
            (*de).t = (*dd).ld_start;
            (*de).flags = 0x40 as libc::c_int as libc::c_uchar;
            (*de).defl = 0x1 as libc::c_int as libc::c_uchar;
            (*de).x = (*(*de).s).x;
            (*de).y = (*(*de2).s).y as libc::c_float;
            (*de).m = (*de2).m;
            (*de2).start = de;
            if de2 == tail {
                break;
            }
        }
        de2 = (*de2).next;
    };
}
#[no_mangle]
pub unsafe extern "C" fn draw_deco_near() {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut g: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut dc: *mut decos = 0 as *mut decos;
    deco_tail = 0 as *mut deco_elt;
    deco_head = deco_tail;
    let mut current_block_8: u64;
    s = tsfirst;
    while !s.is_null() {
        match (*s).type_0 as libc::c_int {
            3 | 9 => {
                if (*s).u.bar.dc.n as libc::c_int == 0 as libc::c_int {
                    current_block_8 = 16559507199688588974;
                } else {
                    dc = &mut (*s).u.bar.dc;
                    current_block_8 = 17407779659766490442;
                }
            }
            1 | 2 => {
                if (*s).u.note.dc.n as libc::c_int == 0 as libc::c_int {
                    current_block_8 = 16559507199688588974;
                } else {
                    dc = &mut (*s).u.note.dc;
                    current_block_8 = 17407779659766490442;
                }
            }
            11 => {
                g = (*s).extra;
                while !g.is_null() {
                    if !((*g).abc_type as libc::c_int != 4 as libc::c_int
                        || (*g).u.note.dc.n as libc::c_int == 0 as libc::c_int)
                    {
                        dc = &mut (*g).u.note.dc;
                        deco_create(g, dc);
                    }
                    g = (*g).next;
                }
                current_block_8 = 16559507199688588974;
            }
            _ => {
                current_block_8 = 16559507199688588974;
            }
        }
        match current_block_8 {
            17407779659766490442 => {
                deco_create(s, dc);
            }
            _ => {}
        }
        s = (*s).ts_next;
    }
    ll_deco();
}
#[no_mangle]
pub unsafe extern "C" fn draw_deco_note() {
    let mut de: *mut deco_elt = 0 as *mut deco_elt;
    let mut dd: *mut deco_def_s = 0 as *mut deco_def_s;
    let mut f: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    de = deco_head;
    while !de.is_null() {
        t = (*de).t as libc::c_int;
        dd = &mut *deco_def_tb.as_mut_ptr().offset(t as isize) as *mut deco_def_s;
        f = (*dd).func as libc::c_int;
        if !(f_note as libc::c_int & (1 as libc::c_int) << f == 0
            || (*de).m as libc::c_int >= 0 as libc::c_int)
        {
            (func_tb[f as usize]).expect("non-null function pointer")(de);
        }
        de = (*de).next;
    }
}
unsafe extern "C" fn draw_repbra(mut p_voice: *mut VOICE_S) {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s1: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut first_repeat: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut i: libc::c_int = 0;
    let mut x: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    let mut y2: libc::c_float = 0.;
    let mut w: libc::c_float = 0.;
    y = (staff_tb[(*p_voice).staff as usize].topbar as libc::c_int + 6 as libc::c_int
        + 20 as libc::c_int) as libc::c_float;
    first_repeat = 0 as *mut SYMBOL;
    s = (*(*p_voice).sym).next;
    while !s.is_null() {
        if !((*s).type_0 as libc::c_int != 3 as libc::c_int) {
            if !((*s).sflags & 0x10000000 as libc::c_int as libc::c_uint == 0
                || (*s).sflags & 0x400000 as libc::c_int as libc::c_uint != 0)
            {
                if ((*s).next).is_null() {
                    break;
                }
                if first_repeat.is_null() {
                    first_repeat = s;
                }
                s1 = s;
                while !((*s).next).is_null() {
                    s = (*s).next;
                    if (*s).sflags & 0x8000 as libc::c_int as libc::c_uint != 0 {
                        break;
                    }
                }
                y2 = y_get(
                    (*p_voice).staff as libc::c_int,
                    1 as libc::c_int,
                    (*s1).x,
                    (*s).x - (*s1).x,
                );
                if y < y2 {
                    y = y2;
                }
                if !((*s1).gch).is_null() {
                    w = (*(*s1).gch).w;
                    y2 = y_get(
                        (*p_voice).staff as libc::c_int,
                        1 as libc::c_int,
                        (*s1).x + 4 as libc::c_int as libc::c_float,
                        w,
                    );
                    y2
                        += cfmt.font_tb[REPEATFONT as libc::c_int as usize].size
                            + 2 as libc::c_int as libc::c_float;
                    if y < y2 {
                        y = y2;
                    }
                }
                if (*s).sflags & 0x10000000 as libc::c_int as libc::c_uint != 0 {
                    s = (*s).prev;
                }
            }
        }
        s = (*s).next;
    }
    s = first_repeat;
    if s.is_null() {
        return;
    }
    set_sscale(-(1 as libc::c_int));
    set_font(REPEATFONT as libc::c_int);
    while !s.is_null() {
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        if !((*s).sflags & 0x10000000 as libc::c_int as libc::c_uint == 0
            || (*s).sflags & 0x400000 as libc::c_int as libc::c_uint != 0)
        {
            s1 = s;
            while !((*s).next).is_null() {
                s = (*s).next;
                if (*s).sflags & 0x8000 as libc::c_int as libc::c_uint != 0 {
                    break;
                }
            }
            s2 = s;
            if s1 == s2 {
                break;
            }
            x = (*s1).x;
            if (*s1).u.bar.type_0 & 0xf as libc::c_int == 4 as libc::c_int {
                x -= 4 as libc::c_int as libc::c_float;
            }
            if (*s2).type_0 as libc::c_int != 3 as libc::c_int {
                if (*s2).sflags & 0x8000 as libc::c_int as libc::c_uint != 0 {
                    w = 0 as libc::c_int as libc::c_float;
                } else {
                    w = (*s2).x - realwidth + 4 as libc::c_int as libc::c_float;
                }
            } else if (*s2).u.bar.type_0 & 0xf0 as libc::c_int != 0
                && (*s2).u.bar.type_0 != 2 as libc::c_int | 3 as libc::c_int
                || (*s2).u.bar.type_0 == 3 as libc::c_int
            {
                if (*s2).u.bar.type_0 == 3 as libc::c_int {
                    (*s2)
                        .flags = ((*s2).flags as libc::c_int | 0x2 as libc::c_int)
                        as libc::c_ushort;
                }
                if (*s1).staff as libc::c_int > 0 as libc::c_int
                    && (*cursys)
                        .staff[((*s1).staff as libc::c_int - 1 as libc::c_int) as usize]
                        .flags as libc::c_int & 0x40 as libc::c_int == 0
                {
                    w = (*s2).wl;
                } else if (*s2).u.bar.type_0 & 0xf as libc::c_int == 4 as libc::c_int {
                    w = 12 as libc::c_int as libc::c_float;
                } else if (*s2).sflags & 0x100 as libc::c_int as libc::c_uint == 0 {
                    w = 0 as libc::c_int as libc::c_float;
                } else {
                    w = 8 as libc::c_int as libc::c_float;
                }
            } else {
                w = (if (*s2).sflags & 0x8000 as libc::c_int as libc::c_uint != 0 {
                    0 as libc::c_int
                } else {
                    8 as libc::c_int
                }) as libc::c_float;
            }
            w = (*s2).x - x - w;
            p = (*s1).text;
            if p.is_null() {
                p = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            if ((*s2).next).is_null()
                && (*s2).sflags & 0x8000 as libc::c_int as libc::c_uint == 0
                && (*p_voice).bar_start as libc::c_int == 0 as libc::c_int
            {
                (*p_voice)
                    .bar_start = (2 as libc::c_int | 0x1000 as libc::c_int)
                    as libc::c_short;
                (*p_voice).set_bar_repeat(1 as libc::c_int as libc::c_uint);
            }
            if (*s1).flags as libc::c_int & 0x100 as libc::c_int != 0 {
                i = if (*s2).flags as libc::c_int & 0x200 as libc::c_int != 0 {
                    3 as libc::c_int
                } else {
                    1 as libc::c_int
                };
            } else {
                i = if (*s2).flags as libc::c_int & 0x200 as libc::c_int != 0 {
                    2 as libc::c_int
                } else {
                    0 as libc::c_int
                };
            }
            a2b(
                b"(%s)-%.1f %d \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                p,
                cfmt.font_tb[REPEATFONT as libc::c_int as usize].size as libc::c_double
                    * 0.8f64 + 1 as libc::c_int as libc::c_double,
                i,
            );
            putx(w);
            putxy(x, y * staff_tb[(*s1).staff as usize].staffscale);
            a2b(
                b"yns%d repbra\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*s1).staff as libc::c_int,
            );
            y_set(
                (*s1).staff as libc::c_int,
                1 as libc::c_int,
                x,
                w,
                y + 2 as libc::c_int as libc::c_float,
            );
            if (*s).u.bar.repeat_bar != 0 {
                s = (*s).prev;
            }
        }
        s = (*s).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn draw_deco_staff() {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut first_gchord: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut y: libc::c_float = 0.;
    let mut w: libc::c_float = 0.;
    let mut de: *mut deco_elt = 0 as *mut deco_elt;
    let mut minmax: [C2RustUnnamed_12; 32] = [C2RustUnnamed_12 {
        ymin: 0.,
        ymax: 0.,
    }; 32];
    memset(
        minmax.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[C2RustUnnamed_12; 32]>() as libc::c_ulong,
    );
    first_gchord = 0 as *mut SYMBOL;
    s = tsfirst;
    while !s.is_null() {
        let mut gch: *mut gch = 0 as *mut gch;
        let mut gch2: *mut gch = 0 as *mut gch;
        let mut ix: libc::c_int = 0;
        gch = (*s).gch;
        if !gch.is_null() {
            if first_gchord.is_null() {
                first_gchord = s;
            }
            gch2 = 0 as *mut gch;
            ix = 0 as libc::c_int;
            while ix < 8 as libc::c_int {
                if (*gch).type_0 as libc::c_int == '\0' as i32 {
                    break;
                }
                if !((*gch).type_0 as libc::c_int != 'g' as i32) {
                    gch2 = gch;
                    if (*gch).y < 0 as libc::c_int as libc::c_float {
                        break;
                    }
                }
                ix += 1;
                ix;
                gch = gch.offset(1);
                gch;
            }
            if !gch2.is_null() {
                w = (*gch2).w;
                if (*gch2).y >= 0 as libc::c_int as libc::c_float {
                    y = y_get((*s).staff as libc::c_int, 1 as libc::c_int, (*s).x, w);
                    if y > minmax[(*s).staff as usize].ymax {
                        minmax[(*s).staff as usize].ymax = y;
                    }
                } else {
                    y = y_get((*s).staff as libc::c_int, 0 as libc::c_int, (*s).x, w);
                    if y < minmax[(*s).staff as usize].ymin {
                        minmax[(*s).staff as usize].ymin = y;
                    }
                }
            }
        }
        s = (*s).ts_next;
    }
    if !first_gchord.is_null() {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i <= nstaff {
            let mut top: libc::c_int = 0;
            let mut bot: libc::c_int = 0;
            bot = staff_tb[i as usize].botbar as libc::c_int;
            if minmax[i as usize].ymin > (bot - 6 as libc::c_int) as libc::c_float {
                minmax[i as usize].ymin = (bot - 6 as libc::c_int) as libc::c_float;
            }
            top = staff_tb[i as usize].topbar as libc::c_int;
            if minmax[i as usize].ymax < (top + 6 as libc::c_int) as libc::c_float {
                minmax[i as usize].ymax = (top + 6 as libc::c_int) as libc::c_float;
            }
            i += 1;
            i;
        }
        set_sscale(-(1 as libc::c_int));
        let mut current_block_34: u64;
        s = first_gchord;
        while !s.is_null() {
            if !((*s).gch).is_null() {
                match (*s).type_0 as libc::c_int {
                    1 | 2 | 9 => {
                        current_block_34 = 15512526488502093901;
                        match current_block_34 {
                            3111082053746456878 => {
                                if (*s).u.bar.repeat_bar == 0 {
                                    current_block_34 = 15512526488502093901;
                                } else {
                                    current_block_34 = 6450636197030046351;
                                }
                            }
                            _ => {}
                        }
                        match current_block_34 {
                            6450636197030046351 => {}
                            _ => {
                                draw_gchord(
                                    s,
                                    minmax[(*s).staff as usize].ymin,
                                    minmax[(*s).staff as usize].ymax,
                                );
                            }
                        }
                    }
                    3 => {
                        current_block_34 = 3111082053746456878;
                        match current_block_34 {
                            3111082053746456878 => {
                                if (*s).u.bar.repeat_bar == 0 {
                                    current_block_34 = 15512526488502093901;
                                } else {
                                    current_block_34 = 6450636197030046351;
                                }
                            }
                            _ => {}
                        }
                        match current_block_34 {
                            6450636197030046351 => {}
                            _ => {
                                draw_gchord(
                                    s,
                                    minmax[(*s).staff as usize].ymin,
                                    minmax[(*s).staff as usize].ymax,
                                );
                            }
                        }
                    }
                    _ => {}
                }
            }
            s = (*s).ts_next;
        }
    }
    p_voice = first_voice;
    while !p_voice.is_null() {
        if !((*p_voice).second() as libc::c_int != 0 || ((*p_voice).sym).is_null()) {
            draw_repbra(p_voice);
        }
        p_voice = (*p_voice).next;
    }
    memset(
        minmax.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[C2RustUnnamed_12; 32]>() as libc::c_ulong,
    );
    de = deco_head;
    while !de.is_null() {
        let mut dd: *mut deco_def_s = 0 as *mut deco_def_s;
        dd = &mut *deco_def_tb.as_mut_ptr().offset((*de).t as isize) as *mut deco_def_s;
        if !(f_staff as libc::c_int & (1 as libc::c_int) << (*dd).func as libc::c_int
            == 0 || (*de).m as libc::c_int >= 0 as libc::c_int)
        {
            (func_tb[(*dd).func as usize]).expect("non-null function pointer")(de);
            if !(((*dd).ps_func as libc::c_int) < 0 as libc::c_int) {
                if cfmt.dynalign != 0 {
                    if (*de).flags as libc::c_int & 0x2 as libc::c_int != 0 {
                        if (*de).y > minmax[(*de).staff as usize].ymax {
                            minmax[(*de).staff as usize].ymax = (*de).y;
                        }
                    } else if (*de).y < minmax[(*de).staff as usize].ymin {
                        minmax[(*de).staff as usize].ymin = (*de).y;
                    }
                }
            }
        }
        de = (*de).next;
    }
    de = deco_head;
    while !de.is_null() {
        let mut dd_0: *mut deco_def_s = 0 as *mut deco_def_s;
        dd_0 = &mut *deco_def_tb.as_mut_ptr().offset((*de).t as isize)
            as *mut deco_def_s;
        if !(((*dd_0).ps_func as libc::c_int) < 0 as libc::c_int
            || f_staff as libc::c_int & (1 as libc::c_int) << (*dd_0).func as libc::c_int
                == 0)
        {
            if cfmt.dynalign != 0 {
                if (*de).flags as libc::c_int & 0x2 as libc::c_int != 0 {
                    y = minmax[(*de).staff as usize].ymax;
                } else {
                    y = minmax[(*de).staff as usize].ymin;
                }
                (*de).y = y;
            } else {
                y = (*de).y;
            }
            if (*de).flags as libc::c_int & 0x2 as libc::c_int != 0 {
                y += (*dd_0).h as libc::c_int as libc::c_float;
            }
            y_set(
                (*de).staff as libc::c_int,
                (*de).flags as libc::c_int & 0x2 as libc::c_int,
                (*de).x,
                (*de).val,
                y,
            );
        }
        de = (*de).next;
    }
}
unsafe extern "C" fn draw_gchord(
    mut s: *mut SYMBOL,
    mut gchy_min: libc::c_float,
    mut gchy_max: libc::c_float,
) {
    let mut gch: *mut gch = 0 as *mut gch;
    let mut gch2: *mut gch = 0 as *mut gch;
    let mut action: libc::c_int = 0;
    let mut ix: libc::c_int = 0;
    let mut box_0: libc::c_int = 0;
    let mut yav: libc::c_int = 0;
    let mut x: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    let mut w: libc::c_float = 0.;
    let mut h: libc::c_float = 0.;
    let mut y_above: libc::c_float = 0.;
    let mut y_below: libc::c_float = 0.;
    let mut hbox: libc::c_float = 0.;
    let mut xboxl: libc::c_float = 0.;
    let mut yboxh: libc::c_float = 0.;
    let mut yboxl: libc::c_float = 0.;
    let mut expdx: libc::c_float = 0.;
    w = (*(*s).gch).w;
    y_above = y_get(
        (*s).staff as libc::c_int,
        1 as libc::c_int,
        (*s).x - 2 as libc::c_int as libc::c_float,
        w,
    );
    y_below = y_get(
        (*s).staff as libc::c_int,
        0 as libc::c_int,
        (*s).x - 2 as libc::c_int as libc::c_float,
        w,
    );
    gch2 = 0 as *mut gch;
    yav = (((*s).pits[(*s).nhd as usize] as libc::c_int
        + (*s).pits[0 as libc::c_int as usize] as libc::c_int) / 2 as libc::c_int
        - 18 as libc::c_int) * 3 as libc::c_int;
    ix = 0 as libc::c_int;
    gch = (*s).gch;
    while ix < 8 as libc::c_int {
        if (*gch).type_0 as libc::c_int == '\0' as i32 {
            break;
        }
        if !((*gch).type_0 as libc::c_int != 'g' as i32) {
            gch2 = gch;
            if (*gch).y < 0 as libc::c_int as libc::c_float {
                break;
            }
        }
        ix += 1;
        ix;
        gch = gch.offset(1);
        gch;
    }
    if !gch2.is_null() {
        if (*gch2).y >= 0 as libc::c_int as libc::c_float {
            if y_above < gchy_max {
                y_above = gchy_max;
            }
        } else if y_below > gchy_min {
            y_below = gchy_min;
        }
    }
    str_font((*(*s).gch).font as libc::c_int);
    set_font((*(*s).gch).font as libc::c_int);
    set_sscale(-(1 as libc::c_int));
    xboxl = (*s).x;
    yboxh = -(100 as libc::c_int) as libc::c_float;
    yboxl = 100 as libc::c_int as libc::c_float;
    box_0 = 0 as libc::c_int;
    expdx = 0 as libc::c_int as libc::c_float;
    ix = 0 as libc::c_int;
    gch = (*s).gch;
    while ix < 8 as libc::c_int {
        if (*gch).type_0 as libc::c_int == '\0' as i32 {
            break;
        }
        h = cfmt.font_tb[(*gch).font as usize].size;
        str_font((*gch).font as libc::c_int);
        tex_str(((*s).text).offset((*gch).idx as libc::c_int as isize));
        w = (*gch).w;
        if (*gch).type_0 as libc::c_int == 'g' as i32 {
            if (strchr(tex_buf.as_mut_ptr(), '\t' as i32)).is_null() {
                action = 4 as libc::c_int;
            } else {
                let mut next: *mut SYMBOL = 0 as *mut SYMBOL;
                let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut n: libc::c_int = 0;
                x = realwidth;
                next = (*s).next;
                while !next.is_null() {
                    match (*next).type_0 as libc::c_int {
                        1 | 3 => {
                            x = (*next).x;
                            break;
                        }
                        _ => {
                            next = (*next).next;
                        }
                    }
                }
                n = 0 as libc::c_int;
                r = tex_buf.as_mut_ptr();
                loop {
                    n += 1;
                    n;
                    r = strchr(r, '\t' as i32);
                    if r.is_null() {
                        break;
                    }
                    r = r.offset(1);
                    r;
                }
                expdx = (x - (*s).x - w) / n as libc::c_float;
                action = 6 as libc::c_int;
            }
        } else {
            action = 5 as libc::c_int;
        }
        x = (*s).x + (*gch).x;
        match (*gch).type_0 as libc::c_int {
            95 => {
                y = (*gch).y + y_below;
                y_set(
                    (*s).staff as libc::c_int,
                    0 as libc::c_int,
                    x,
                    w,
                    (y as libc::c_double - h as libc::c_double * 0.2f64
                        - 2 as libc::c_int as libc::c_double) as libc::c_float,
                );
            }
            94 => {
                y = (*gch).y + y_above;
                y_set(
                    (*s).staff as libc::c_int,
                    1 as libc::c_int,
                    x,
                    w,
                    (y as libc::c_double + h as libc::c_double * 0.8f64
                        + 2 as libc::c_int as libc::c_double) as libc::c_float,
                );
            }
            60 => {
                if (*s).u.note.notes[0 as libc::c_int as usize].acc != 0 {
                    x -= (*s).u.note.notes[0 as libc::c_int as usize].shac;
                }
                y = yav as libc::c_float + (*gch).y;
            }
            62 => {
                x += (*s).xmx;
                if (*s).dots as libc::c_int > 0 as libc::c_int {
                    x = (x as libc::c_double
                        + (1.5f64 + 3.5f64 * (*s).dots as libc::c_int as libc::c_double))
                        as libc::c_float;
                }
                y = yav as libc::c_float + (*gch).y;
            }
            64 => {
                y = yav as libc::c_float + (*gch).y;
            }
            _ => {
                hbox = (if (*gch).box_0 as libc::c_int != 0 {
                    3 as libc::c_int
                } else {
                    2 as libc::c_int
                }) as libc::c_float;
                if (*gch).y >= 0 as libc::c_int as libc::c_float {
                    y = (*gch).y + y_above;
                    y_set(
                        (*s).staff as libc::c_int,
                        1 as libc::c_int,
                        x,
                        w,
                        y + h + hbox,
                    );
                } else {
                    y = (*gch).y + y_below;
                    y_set((*s).staff as libc::c_int, 0 as libc::c_int, x, w, y - hbox);
                }
                if (*gch).box_0 != 0 {
                    if xboxl > x {
                        xboxl = x;
                    }
                    if yboxl > y {
                        yboxl = y;
                    }
                    if yboxh < y + h {
                        yboxh = y + h;
                    }
                    box_0 += 1;
                    box_0;
                }
            }
        }
        putxy(
            x,
            ((y as libc::c_double + h as libc::c_double * 0.2f64)
                * staff_tb[(*s).staff as usize].staffscale as libc::c_double)
                as libc::c_float,
        );
        a2b(
            b"yns%d M \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*s).staff as libc::c_int,
        );
        if action == 6 as libc::c_int {
            a2b(
                b"%.2f \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                expdx as libc::c_double,
            );
        }
        str_out(tex_buf.as_mut_ptr(), action);
        if (*gch).type_0 as libc::c_int == 'g' as i32 && box_0 > 0 as libc::c_int {
            if box_0 == 1 as libc::c_int {
                a2b(
                    b" boxend\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            } else {
                a2b(
                    b" boxmark\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
        }
        a2b(b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        ix += 1;
        ix;
        gch = gch.offset(1);
        gch;
    }
    if box_0 != 0 {
        xboxl -= 2 as libc::c_int as libc::c_float;
        putxy(
            xboxl,
            (yboxl - 1 as libc::c_int as libc::c_float)
                * staff_tb[(*s).staff as usize].staffscale,
        );
        a2b(
            b"yns%d %.1f boxdraw\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*s).staff as libc::c_int,
            (yboxh - yboxl + 3 as libc::c_int as libc::c_float) as libc::c_double,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn draw_measnb() {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut sy: *mut SYSTEM = 0 as *mut SYSTEM;
    let mut showm: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut any_nb: libc::c_int = 0;
    let mut staff: libc::c_int = 0;
    let mut bar_num: libc::c_int = 0;
    let mut x: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    let mut w: libc::c_float = 0.;
    let mut font_size: libc::c_float = 0.;
    showm = (if cfmt.measurebox != 0 {
        b"showb\0" as *const u8 as *const libc::c_char
    } else {
        b"show\0" as *const u8 as *const libc::c_char
    }) as *mut libc::c_char;
    any_nb = 0 as libc::c_int;
    sy = cursys;
    staff = 0 as libc::c_int;
    while staff <= nstaff {
        if (*sy).staff[staff as usize].empty == 0 {
            break;
        }
        staff += 1;
        staff;
    }
    if staff > nstaff {
        return;
    }
    set_sscale(staff);
    font_size = cfmt.font_tb[MEASUREFONT as libc::c_int as usize].size;
    cfmt.font_tb[MEASUREFONT as libc::c_int as usize].size
        /= staff_tb[staff as usize].staffscale;
    s = tsfirst;
    bar_num = nbar;
    if bar_num > 1 as libc::c_int {
        if cfmt.measurenb == 0 as libc::c_int {
            set_font(MEASUREFONT as libc::c_int);
            any_nb = 1 as libc::c_int;
            x = 0 as libc::c_int as libc::c_float;
            w = 20 as libc::c_int as libc::c_float;
            y = y_get(staff, 1 as libc::c_int, x, w);
            if y
                < (staff_tb[staff as usize].topbar as libc::c_int + 14 as libc::c_int)
                    as libc::c_float
            {
                y = (staff_tb[staff as usize].topbar as libc::c_int + 14 as libc::c_int)
                    as libc::c_float;
            }
            a2b(b"0 \0" as *const u8 as *const libc::c_char as *mut libc::c_char);
            puty(y);
            a2b(
                b"y%d M(%d)%s\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                staff,
                bar_num,
                showm,
            );
            y_set(
                staff,
                1 as libc::c_int,
                x,
                w,
                y + cfmt.font_tb[MEASUREFONT as libc::c_int as usize].size
                    + 2 as libc::c_int as libc::c_float,
            );
        } else if bar_num % cfmt.measurenb == 0 as libc::c_int {
            loop {
                match (*s).type_0 as libc::c_int {
                    5 | 4 | 6 | 12 | 14 => {}
                    _ => {
                        break;
                    }
                }
                s = (*s).ts_next;
            }
            if !((*s).prev).is_null()
                && (*(*s).prev).type_0 as libc::c_int != 4 as libc::c_int
            {
                s = (*s).prev;
            }
            x = (*s).x - (*s).wl;
            set_font(MEASUREFONT as libc::c_int);
            any_nb = 1 as libc::c_int;
            w = cwid('0' as i32 as libc::c_uchar)
                * cfmt.font_tb[MEASUREFONT as libc::c_int as usize].swfac;
            if bar_num >= 10 as libc::c_int {
                if bar_num >= 100 as libc::c_int {
                    w *= 3 as libc::c_int as libc::c_float;
                } else {
                    w *= 2 as libc::c_int as libc::c_float;
                }
            }
            if cfmt.measurebox != 0 {
                w += 4 as libc::c_int as libc::c_float;
            }
            y = y_get(staff, 1 as libc::c_int, x, w);
            if y
                < (staff_tb[staff as usize].topbar as libc::c_int + 6 as libc::c_int)
                    as libc::c_float
            {
                y = (staff_tb[staff as usize].topbar as libc::c_int + 6 as libc::c_int)
                    as libc::c_float;
            }
            y += 2 as libc::c_int as libc::c_float;
            putxy(x, y);
            a2b(
                b"y%d M(%d)%s\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                staff,
                bar_num,
                showm,
            );
            y += cfmt.font_tb[MEASUREFONT as libc::c_int as usize].size;
            y_set(staff, 1 as libc::c_int, x, w, y);
            (*s).ymx = y as libc::c_schar;
        }
    }
    while !s.is_null() {
        if (*s).sflags & 0x8000000 as libc::c_int as libc::c_uint != 0 {
            sy = (*sy).next;
            staff = 0 as libc::c_int;
            while staff < nstaff {
                if (*sy).staff[staff as usize].empty == 0 {
                    break;
                }
                staff += 1;
                staff;
            }
            set_sscale(staff);
        }
        if !((*s).type_0 as libc::c_int != 3 as libc::c_int
            || (*s).aux as libc::c_int <= 0 as libc::c_int)
        {
            bar_num = (*s).aux as libc::c_int;
            if !(cfmt.measurenb == 0 as libc::c_int
                || bar_num % cfmt.measurenb != 0 as libc::c_int || ((*s).next).is_null())
            {
                if any_nb == 0 {
                    any_nb = 1 as libc::c_int;
                    set_font(MEASUREFONT as libc::c_int);
                }
                w = cwid('0' as i32 as libc::c_uchar)
                    * cfmt.font_tb[MEASUREFONT as libc::c_int as usize].swfac;
                if bar_num >= 10 as libc::c_int {
                    if bar_num >= 100 as libc::c_int {
                        w *= 3 as libc::c_int as libc::c_float;
                    } else {
                        w *= 2 as libc::c_int as libc::c_float;
                    }
                }
                if cfmt.measurebox != 0 {
                    w += 4 as libc::c_int as libc::c_float;
                }
                x = ((*s).x as libc::c_double - w as libc::c_double * 0.4f64)
                    as libc::c_float;
                y = y_get(staff, 1 as libc::c_int, x, w);
                if y
                    < (staff_tb[staff as usize].topbar as libc::c_int + 6 as libc::c_int)
                        as libc::c_float
                {
                    y = (staff_tb[staff as usize].topbar as libc::c_int
                        + 6 as libc::c_int) as libc::c_float;
                }
                if (*(*s).next).abc_type as libc::c_int == 4 as libc::c_int {
                    if (*(*s).next).stem as libc::c_int > 0 as libc::c_int {
                        if y
                            < (*(*s).next).ys
                                - cfmt.font_tb[MEASUREFONT as libc::c_int as usize].size
                        {
                            y = (*(*s).next).ys
                                - cfmt.font_tb[MEASUREFONT as libc::c_int as usize].size;
                        }
                    } else if y < (*(*s).next).y as libc::c_int as libc::c_float {
                        y = (*(*s).next).y as libc::c_float;
                    }
                }
                y += 2 as libc::c_int as libc::c_float;
                a2b(b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char);
                putxy(x, y);
                a2b(
                    b"y%d M(%d)%s\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    staff,
                    bar_num,
                    showm,
                );
                y += cfmt.font_tb[MEASUREFONT as libc::c_int as usize].size;
                y_set(staff, 1 as libc::c_int, x, w, y);
                (*s).ymx = y as libc::c_schar;
            }
        }
        s = (*s).ts_next;
    }
    if any_nb != 0 {
        a2b(b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    }
    nbar = bar_num;
    cfmt.font_tb[MEASUREFONT as libc::c_int as usize].size = font_size;
}
unsafe extern "C" fn get_beat(mut m: *mut meter_s) -> libc::c_int {
    let mut top: libc::c_int = 0;
    let mut bot: libc::c_int = 0;
    if (*m).meter[0 as libc::c_int as usize].top[0 as libc::c_int as usize]
        as libc::c_int == 'C' as i32
    {
        if (*m).meter[0 as libc::c_int as usize].top[0 as libc::c_int as usize]
            as libc::c_int == '|' as i32
        {
            return 1536 as libc::c_int / 2 as libc::c_int;
        }
        return 1536 as libc::c_int / 4 as libc::c_int;
    }
    if (*m).meter[0 as libc::c_int as usize].bot[0 as libc::c_int as usize]
        as libc::c_int == '\0' as i32
    {
        return 1536 as libc::c_int / 4 as libc::c_int;
    }
    sscanf(
        ((*m).meter[0 as libc::c_int as usize].top).as_mut_ptr(),
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut top as *mut libc::c_int,
    );
    sscanf(
        ((*m).meter[0 as libc::c_int as usize].bot).as_mut_ptr(),
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut bot as *mut libc::c_int,
    );
    if bot >= 8 as libc::c_int && top >= 6 as libc::c_int
        && top % 3 as libc::c_int == 0 as libc::c_int
    {
        return 1536 as libc::c_int * 3 as libc::c_int / 8 as libc::c_int;
    }
    return 1536 as libc::c_int / bot;
}
unsafe extern "C" fn draw_notempo(
    mut s: *mut SYMBOL,
    mut len: libc::c_int,
    mut sc: libc::c_float,
) {
    let mut head: libc::c_int = 0;
    let mut dots: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut dx: libc::c_float = 0.;
    a2b(
        b"gsave %.2f dup scale 8 3 RM currentpoint \0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        sc as libc::c_double,
    );
    identify_note(s, len, &mut head, &mut dots, &mut flags);
    match head {
        2 => {
            a2b(b"HD\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        }
        1 => {
            a2b(b"Hd\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        }
        _ => {
            a2b(b"hd\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        }
    }
    dx = 4 as libc::c_int as libc::c_float;
    if dots != 0 {
        let mut dotx: libc::c_float = 0.;
        dotx = 8 as libc::c_int as libc::c_float;
        if flags > 0 as libc::c_int {
            dotx += 4 as libc::c_int as libc::c_float;
        }
        match head {
            3 | 2 => {
                dotx += 2 as libc::c_int as libc::c_float;
            }
            1 => {
                dotx += 1 as libc::c_int as libc::c_float;
            }
            _ => {}
        }
        loop {
            dots -= 1;
            if !(dots >= 0 as libc::c_int) {
                break;
            }
            a2b(
                b" %.1f 0 dt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                dotx as libc::c_double,
            );
            dx = dotx;
            dotx = (dotx as libc::c_double + 3.5f64) as libc::c_float;
        }
    }
    if len < 1536 as libc::c_int {
        if flags <= 0 as libc::c_int {
            a2b(
                b" %d su\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                21 as libc::c_int,
            );
        } else {
            a2b(
                b" %d %d sfu\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                flags,
                21 as libc::c_int,
            );
            if dx < 6 as libc::c_int as libc::c_float {
                dx = 6 as libc::c_int as libc::c_float;
            }
        }
    }
    a2b(
        b" grestore %.1f 0 RM\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        ((dx + 15 as libc::c_int as libc::c_float) * sc) as libc::c_double,
    );
}
#[no_mangle]
pub unsafe extern "C" fn tempo_width(mut s: *mut SYMBOL) -> libc::c_float {
    let mut i: libc::c_uint = 0;
    let mut w: libc::c_float = 0.;
    w = 0 as libc::c_int as libc::c_float;
    if !((*s).u.tempo.str1).is_null() {
        w += tex_str((*s).u.tempo.str1);
    }
    if (*s).u.tempo.beats[0 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int {
        if (*s).u.tempo.circa != 0 {
            w
                += tex_str(
                    b"ca. \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
        }
        i = 1 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong)
            < (::core::mem::size_of::<[libc::c_short; 4]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
            && (*s).u.tempo.beats[i as usize] as libc::c_int != 0 as libc::c_int
        {
            w += 10 as libc::c_int as libc::c_float;
            i = i.wrapping_add(1);
            i;
        }
        w
            += 6 as libc::c_int as libc::c_float
                + cwid(' ' as i32 as libc::c_uchar)
                    * cfmt.font_tb[TEMPOFONT as libc::c_int as usize].swfac
                    * 6 as libc::c_int as libc::c_float
                + 10 as libc::c_int as libc::c_float
                + 10 as libc::c_int as libc::c_float;
    }
    if !((*s).u.tempo.str2).is_null() {
        w += tex_str((*s).u.tempo.str2);
    }
    return w;
}
#[no_mangle]
pub unsafe extern "C" fn write_tempo(
    mut s: *mut SYMBOL,
    mut beat: libc::c_int,
    mut sc: libc::c_float,
) {
    let mut i: libc::c_uint = 0;
    let mut tmp: [libc::c_char; 16] = [0; 16];
    if !((*s).u.tempo.str1).is_null() {
        put_str((*s).u.tempo.str1, 0 as libc::c_int);
    }
    if (*s).u.tempo.beats[0 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int {
        sc = (sc as libc::c_double
            * (0.7f64
                * cfmt.font_tb[TEMPOFONT as libc::c_int as usize].size as libc::c_double
                / 15.0f64)) as libc::c_float;
        i = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong)
            < (::core::mem::size_of::<[libc::c_short; 4]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_short>() as libc::c_ulong)
            && (*s).u.tempo.beats[i as usize] as libc::c_int != 0 as libc::c_int
        {
            draw_notempo(s, (*s).u.tempo.beats[i as usize] as libc::c_int, sc);
            i = i.wrapping_add(1);
            i;
        }
        put_str(
            b"= \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as libc::c_int,
        );
        if (*s).u.tempo.tempo as libc::c_int != 0 as libc::c_int {
            if (*s).u.tempo.circa != 0 {
                put_str(
                    b"ca. \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    0 as libc::c_int,
                );
            }
            snprintf(
                tmp.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                b"%d\0" as *const u8 as *const libc::c_char,
                (*s).u.tempo.tempo as libc::c_int,
            );
            put_str(tmp.as_mut_ptr(), 0 as libc::c_int);
        } else {
            draw_notempo(s, (*s).u.tempo.new_beat as libc::c_int, sc);
        }
    }
    if !((*s).u.tempo.str2).is_null() {
        put_str((*s).u.tempo.str2, 0 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn draw_partempo(
    mut staff: libc::c_int,
    mut top: libc::c_float,
) -> libc::c_float {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut g: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut beat: libc::c_int = 0;
    let mut dosh: libc::c_int = 0;
    let mut shift: libc::c_int = 0;
    let mut some_part: libc::c_int = 0;
    let mut some_tempo: libc::c_int = 0;
    let mut h: libc::c_float = 0.;
    let mut ht: libc::c_float = 0.;
    let mut w: libc::c_float = 0.;
    let mut x: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    let mut ymin: libc::c_float = 0.;
    let mut dy: libc::c_float = 0.;
    dy = 0 as libc::c_int as libc::c_float;
    ht = 0 as libc::c_int as libc::c_float;
    some_tempo = 0 as libc::c_int;
    some_part = some_tempo;
    ymin = (staff_tb[staff as usize].topbar as libc::c_int + 12 as libc::c_int)
        as libc::c_float;
    dosh = 0 as libc::c_int;
    shift = 1 as libc::c_int;
    x = 0 as libc::c_int as libc::c_float;
    s = tsfirst;
    while !s.is_null() {
        g = (*s).extra;
        if !g.is_null() {
            while !g.is_null() {
                if (*g).type_0 as libc::c_int == 7 as libc::c_int {
                    break;
                }
                g = (*g).next;
            }
            if !g.is_null() {
                if some_tempo == 0 {
                    some_tempo = 1 as libc::c_int;
                    str_font(TEMPOFONT as libc::c_int);
                }
                w = tempo_width(g);
                y = y_get(
                    staff,
                    1 as libc::c_int,
                    (*s).x - 5 as libc::c_int as libc::c_float,
                    w,
                ) + 2 as libc::c_int as libc::c_float;
                if y > ymin {
                    ymin = y;
                }
                if x >= (*s).x - 5 as libc::c_int as libc::c_float
                    && dosh & shift >> 1 as libc::c_int == 0
                {
                    dosh |= shift;
                }
                shift <<= 1 as libc::c_int;
                x = (*s).x - 5 as libc::c_int as libc::c_float + w;
            }
        }
        s = (*s).ts_next;
    }
    if some_tempo != 0 {
        ht = cfmt.font_tb[TEMPOFONT as libc::c_int as usize].size
            + 2 as libc::c_int as libc::c_float + 2 as libc::c_int as libc::c_float;
        y = 2 as libc::c_int as libc::c_float - ht;
        h = y - ht;
        if dosh != 0 as libc::c_int {
            ht *= 2 as libc::c_int as libc::c_float;
        }
        if top < ymin + ht {
            dy = ymin + ht - top;
        }
        str_font(TEMPOFONT as libc::c_int);
        beat = 0 as libc::c_int;
        s = tsfirst;
        while !s.is_null() {
            if !((*s).sflags & 0x80000 as libc::c_int as libc::c_uint == 0) {
                if (*s).type_0 as libc::c_int == 5 as libc::c_int {
                    beat = get_beat(&mut (*s).u.meter);
                }
                g = (*s).extra;
                while !g.is_null() {
                    if (*g).type_0 as libc::c_int == 7 as libc::c_int {
                        break;
                    }
                    g = (*g).next;
                }
                if !g.is_null() {
                    a2b(
                        b"%.1f %.1f M \0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        ((*s).x - 5 as libc::c_int as libc::c_float) as libc::c_double,
                        (if dosh & 1 as libc::c_int != 0 { h } else { y })
                            as libc::c_double,
                    );
                    dosh >>= 1 as libc::c_int;
                    write_tempo(g, beat, 1 as libc::c_int as libc::c_float);
                }
            }
            s = (*s).ts_next;
        }
    }
    ymin = (staff_tb[staff as usize].topbar as libc::c_int + 14 as libc::c_int)
        as libc::c_float;
    s = tsfirst;
    while !s.is_null() {
        g = (*s).extra;
        if !g.is_null() {
            while !g.is_null() {
                if (*g).type_0 as libc::c_int == 10 as libc::c_int {
                    break;
                }
                g = (*g).next;
            }
            if !g.is_null() {
                if some_part == 0 {
                    some_part = 1 as libc::c_int;
                    str_font(PARTSFONT as libc::c_int);
                }
                w = tex_str(&mut *((*g).text).offset(2 as libc::c_int as isize));
                y = y_get(
                    staff,
                    1 as libc::c_int,
                    (*s).x - 10 as libc::c_int as libc::c_float,
                    w + 3 as libc::c_int as libc::c_float,
                ) + 5 as libc::c_int as libc::c_float;
                if ymin < y {
                    ymin = y;
                }
            }
        }
        s = (*s).ts_next;
    }
    if !(some_part == 0) {
        h = cfmt.font_tb[PARTSFONT as libc::c_int as usize].size
            + 2 as libc::c_int as libc::c_float + 2 as libc::c_int as libc::c_float;
        if top < ymin + h + ht {
            dy = ymin + h + ht - top;
        }
        set_font(PARTSFONT as libc::c_int);
        s = tsfirst;
        while !s.is_null() {
            g = (*s).extra;
            if !g.is_null() {
                while !g.is_null() {
                    if (*g).type_0 as libc::c_int == 10 as libc::c_int {
                        break;
                    }
                    g = (*g).next;
                }
                if !g.is_null() {
                    a2b(
                        b"%.1f %.1f M\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        ((*s).x - 10 as libc::c_int as libc::c_float) as libc::c_double,
                        (2 as libc::c_int as libc::c_float - ht - h) as libc::c_double,
                    );
                    tex_str(&mut *((*g).text).offset(2 as libc::c_int as isize));
                    str_out(tex_buf.as_mut_ptr(), 0 as libc::c_int);
                    if cfmt.partsbox != 0 {
                        a2b(
                            b" %.1f %.1f %.1f boxend boxdraw\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            ((*s).x - 10 as libc::c_int as libc::c_float
                                - 2 as libc::c_int as libc::c_float) as libc::c_double,
                            (2 as libc::c_int as libc::c_float - ht - h
                                - 4 as libc::c_int as libc::c_float) as libc::c_double,
                            h as libc::c_double,
                        );
                    }
                    a2b(
                        b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                }
            }
            s = (*s).ts_next;
        }
    }
    return dy;
}
#[no_mangle]
pub unsafe extern "C" fn init_deco() {
    memset(
        &mut deco as *mut [*mut libc::c_char; 256] as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[*mut libc::c_char; 256]>() as libc::c_ulong,
    );
    deco['.' as i32
        as usize] = b"dot\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    deco['H' as i32
        as usize] = b"fermata\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    deco['L' as i32
        as usize] = b"emphasis\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    deco['M' as i32
        as usize] = b"lowermordent\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    deco['O' as i32
        as usize] = b"coda\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    deco['P' as i32
        as usize] = b"uppermordent\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    deco['S' as i32
        as usize] = b"segno\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    deco['T' as i32
        as usize] = b"trill\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    deco['u' as i32
        as usize] = b"upbow\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    deco['v' as i32
        as usize] = b"downbow\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    deco['~' as i32
        as usize] = b"gmark\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    deco['J' as i32
        as usize] = b"slide\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    deco['R' as i32
        as usize] = b"roll\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn reset_deco() {
    memset(
        deco_def_tb.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[deco_def_s; 128]>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn set_defl(mut new_defl: libc::c_int) {
    if defl == new_defl {
        return;
    }
    defl = new_defl;
    a2b(
        b"/defl %d def \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        new_defl,
    );
}
