use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type __sFILEX;
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    static mut parse: parse;
    static mut cfmt: FORMAT;
    static mut mbf: *mut libc::c_char;
    static mut outft: libc::c_int;
    static mut annotate: libc::c_int;
    static mut pipeformat: libc::c_int;
    static mut staff_tb: [STAFF_S; 32];
    static mut nstaff: libc::c_int;
    static mut voice_tb: [VOICE_S; 32];
    static mut first_voice: *mut VOICE_S;
    static mut tsfirst: *mut SYMBOL;
    static mut tsnext: *mut SYMBOL;
    static mut realwidth: libc::c_float;
    static mut cursys: *mut SYSTEM;
    fn getarena(len: libc::c_int) -> *mut libc::c_void;
    fn a2b(fmt: *mut libc::c_char, _: ...);
    fn deco_update(s: *mut SYMBOL, dx: libc::c_float);
    fn draw_deco_near();
    fn draw_deco_note();
    fn draw_deco_staff();
    fn draw_partempo(staff: libc::c_int, top: libc::c_float) -> libc::c_float;
    fn draw_measnb();
    fn y_get(
        staff: libc::c_int,
        up: libc::c_int,
        x: libc::c_float,
        w: libc::c_float,
    ) -> libc::c_float;
    fn y_set(
        staff: libc::c_int,
        up: libc::c_int,
        x: libc::c_float,
        w: libc::c_float,
        y: libc::c_float,
    );
    fn put_str(str: *mut libc::c_char, action: libc::c_int);
    fn set_str_font(cft: libc::c_int, dft: libc::c_int);
    fn get_str_font(cft: *mut libc::c_int, dft: *mut libc::c_int);
    fn str_font(ft: libc::c_int);
    static mut tex_buf: [libc::c_char; 0];
    fn set_font(ft: libc::c_int);
    static mut __stderrp: *mut FILE;
    fn unlksym(s: *mut SYMBOL);
    fn identify_note(
        s: *mut SYMBOL,
        len: libc::c_int,
        p_head: *mut libc::c_int,
        p_dots: *mut libc::c_int,
        p_flags: *mut libc::c_int,
    );
    fn set_defl(new_defl: libc::c_int);
    fn bug(msg: *mut libc::c_char, fatal: libc::c_int);
    fn error(sev: libc::c_int, s: *mut SYMBOL, fmt: *mut libc::c_char, _: ...);
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
}
pub type __int64_t = libc::c_longlong;
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
    pub nly: libc::c_int,
    pub h: [libc::c_float; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub a: libc::c_short,
    pub b: libc::c_short,
    pub top: libc::c_float,
    pub bot: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BEAM {
    pub s1: *mut SYMBOL,
    pub s2: *mut SYMBOL,
    pub a: libc::c_float,
    pub b: libc::c_float,
    pub nflags: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub nl: libc::c_int,
    pub v: [*mut libc::c_char; 8],
}
static mut acc_tb: [*mut libc::c_char; 6] = [
    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"sh\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"nt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ft\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"dsh\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"dft\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
static mut scale_voice: libc::c_int = 0;
static mut cur_scale: libc::c_float = 1 as libc::c_int as libc::c_float;
static mut cur_trans: libc::c_float = 0 as libc::c_int as libc::c_float;
static mut cur_staff: libc::c_float = 1 as libc::c_int as libc::c_float;
static mut cur_color: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn set_color(mut new_color: libc::c_int) {
    if new_color == cur_color {
        return;
    }
    cur_color = new_color;
    a2b(
        b"%.2f %.2f %.2f setrgbcolor \0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        ((cur_color >> 16 as libc::c_int) as libc::c_float
            / 255 as libc::c_int as libc::c_float) as libc::c_double,
        ((cur_color >> 8 as libc::c_int & 0xff as libc::c_int) as libc::c_float
            / 255 as libc::c_int as libc::c_float) as libc::c_double,
        ((cur_color & 0xff as libc::c_int) as libc::c_float
            / 255 as libc::c_int as libc::c_float) as libc::c_double,
    );
}
unsafe extern "C" fn anno_out(mut s: *mut SYMBOL, mut type_0: libc::c_char) {
    if (*s).linenum == 0 as libc::c_int {
        return;
    }
    if *mbf.offset(-(1 as libc::c_int) as isize) as libc::c_int != '\n' as i32 {
        let fresh0 = mbf;
        mbf = mbf.offset(1);
        *fresh0 = '\n' as i32 as libc::c_char;
    }
    a2b(
        b"%%A %c %d %d \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        type_0 as libc::c_int,
        (*s).linenum,
        (*s).colnum as libc::c_int,
    );
    putxy(
        (*s).x - (*s).wl - 2 as libc::c_int as libc::c_float,
        staff_tb[(*s).staff as usize].y + (*s).ymn as libc::c_int as libc::c_float
            - 2 as libc::c_int as libc::c_float,
    );
    if type_0 as libc::c_int != 'b' as i32 && type_0 as libc::c_int != 'e' as i32 {
        a2b(
            b"%.1f %d\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ((*s).wl + (*s).wr + 4 as libc::c_int as libc::c_float) as libc::c_double,
            (*s).ymx as libc::c_int - (*s).ymn as libc::c_int + 4 as libc::c_int,
        );
    }
    a2b(b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
}
unsafe extern "C" fn rnd6(mut y: libc::c_float) -> libc::c_float {
    let mut iy: libc::c_int = 0;
    iy = ((y as libc::c_double + 2.999f64) as libc::c_int + 12 as libc::c_int)
        / 6 as libc::c_int * 6 as libc::c_int - 12 as libc::c_int;
    return iy as libc::c_float - y;
}
unsafe extern "C" fn b_pos(
    mut grace: libc::c_int,
    mut stem: libc::c_int,
    mut flags: libc::c_int,
    mut b: libc::c_float,
) -> libc::c_float {
    let mut d1: libc::c_float = 0.;
    let mut d2: libc::c_float = 0.;
    let mut shift: libc::c_float = 0.;
    let mut depth: libc::c_float = 0.;
    let mut top: libc::c_float = 0.;
    let mut bot: libc::c_float = 0.;
    shift = (if grace == 0 { 5.0f64 } else { 3.5f64 }) as libc::c_float;
    depth = (if grace == 0 { 3.2f64 } else { 1.8f64 }) as libc::c_float;
    if stem > 0 as libc::c_int {
        bot = b - (flags - 1 as libc::c_int) as libc::c_float * shift - depth;
        if bot > 26 as libc::c_int as libc::c_float {
            return 0 as libc::c_int as libc::c_float;
        }
        top = b;
    } else {
        top = b + (flags - 1 as libc::c_int) as libc::c_float * shift + depth;
        if top < -(2 as libc::c_int) as libc::c_float {
            return 0 as libc::c_int as libc::c_float;
        }
        bot = b;
    }
    d1 = rnd6((top as libc::c_double - 0.25f64) as libc::c_float);
    d2 = rnd6((bot as libc::c_double + 0.25f64) as libc::c_float);
    if d1 * d1 > d2 * d2 {
        return d2;
    }
    return d1;
}
unsafe extern "C" fn sym_dup(mut s_orig: *mut SYMBOL) -> *mut SYMBOL {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut m: libc::c_int = 0;
    s = getarena(::core::mem::size_of::<SYMBOL>() as libc::c_ulong as libc::c_int)
        as *mut SYMBOL;
    memcpy(
        s as *mut libc::c_void,
        s_orig as *const libc::c_void,
        ::core::mem::size_of::<SYMBOL>() as libc::c_ulong,
    );
    (*s).flags = ((*s).flags as libc::c_int | 0x2 as libc::c_int) as libc::c_ushort;
    (*s).text = 0 as *mut libc::c_char;
    m = 0 as libc::c_int;
    while m <= (*s).nhd as libc::c_int {
        (*s).u.note.notes[m as usize].sl1 = 0 as libc::c_int as libc::c_uchar;
        m += 1;
        m;
    }
    memset(
        &mut (*s).u.note.dc as *mut decos as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<decos>() as libc::c_ulong,
    );
    (*s).gch = 0 as *mut gch;
    (*s).ly = 0 as *mut lyrics;
    (*s).extra = 0 as *mut SYMBOL;
    return s;
}
unsafe extern "C" fn calculate_beam(
    mut bm: *mut BEAM,
    mut s1: *mut SYMBOL,
) -> libc::c_int {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut notes: libc::c_int = 0;
    let mut nflags: libc::c_int = 0;
    let mut staff: libc::c_int = 0;
    let mut voice: libc::c_int = 0;
    let mut two_staves: libc::c_int = 0;
    let mut two_dir: libc::c_int = 0;
    let mut visible: libc::c_int = 0 as libc::c_int;
    let mut x: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    let mut ys: libc::c_float = 0.;
    let mut a: libc::c_float = 0.;
    let mut b: libc::c_float = 0.;
    let mut max_stem_err: libc::c_float = 0.;
    let mut sx: libc::c_float = 0.;
    let mut sy: libc::c_float = 0.;
    let mut sxx: libc::c_float = 0.;
    let mut sxy: libc::c_float = 0.;
    let mut syy: libc::c_float = 0.;
    let mut a0: libc::c_float = 0.;
    let mut stem_xoff: libc::c_float = 0.;
    let mut scale: libc::c_float = 0.;
    static mut min_tb: [[libc::c_float; 6]; 2] = [
        [
            16 as libc::c_int as libc::c_float,
            16 as libc::c_int as libc::c_float,
            14 as libc::c_int as libc::c_float,
            12 as libc::c_int as libc::c_float,
            10 as libc::c_int as libc::c_float,
            10 as libc::c_int as libc::c_float,
        ],
        [
            14 as libc::c_int as libc::c_float,
            14 as libc::c_int as libc::c_float,
            10 as libc::c_int as libc::c_float,
            9 as libc::c_int as libc::c_float,
            9 as libc::c_int as libc::c_float,
            9 as libc::c_int as libc::c_float,
        ],
    ];
    if (*s1).sflags & 0x2 as libc::c_int as libc::c_uint == 0 {
        s = sym_dup(s1);
        (*s).prev = (*s1).prev;
        if !((*s).prev).is_null() {
            (*(*s).prev).next = s;
        } else {
            voice_tb[(*s).voice as usize].sym = s;
        }
        (*s1).prev = s;
        (*s).next = s1;
        (*s).ts_prev = (*s1).ts_prev;
        (*(*s).ts_prev).ts_next = s;
        (*s1).ts_prev = s;
        (*s).ts_next = s1;
        s2 = (*s).ts_prev;
        loop {
            match (*s2).type_0 as libc::c_int {
                4 | 6 | 5 => {
                    break;
                }
                _ => {}
            }
            s2 = (*s2).ts_prev;
        }
        (*s).x -= 12 as libc::c_int as libc::c_float;
        if (*s).x > (*s2).x + 12 as libc::c_int as libc::c_float {
            (*s).x = (*s2).x + 12 as libc::c_int as libc::c_float;
        }
        (*s).sflags &= 0x80000 as libc::c_int as libc::c_uint;
        (*s).sflags |= (0x2 as libc::c_int | 0x1000000 as libc::c_int) as libc::c_uint;
        (*s).u.note.slur_st = 0 as libc::c_int as libc::c_uchar;
        (*s).u.note.slur_end = 0 as libc::c_int as libc::c_char;
        s1 = s;
    }
    nflags = 0 as libc::c_int;
    notes = nflags;
    two_dir = 0 as libc::c_int;
    two_staves = two_dir;
    staff = (*s1).staff as libc::c_int;
    voice = (*s1).voice as libc::c_int;
    stem_xoff = (if (*s1).flags as libc::c_int & 0x20 as libc::c_int != 0 {
        2.3f64
    } else {
        (*s1).u.note.sdx as libc::c_double
    }) as libc::c_float;
    s2 = s1;
    loop {
        if (*s2).abc_type as libc::c_int == 4 as libc::c_int {
            if (*s2).nflags as libc::c_int > nflags {
                nflags = (*s2).nflags as libc::c_int;
            }
            notes += 1;
            notes;
            if (*s2).staff as libc::c_int != staff {
                two_staves = 1 as libc::c_int;
            }
            if (*s2).stem as libc::c_int != (*s1).stem as libc::c_int {
                two_dir = 1 as libc::c_int;
            }
            if visible == 0 && (*s2).flags as libc::c_int & 0x2 as libc::c_int == 0
                && ((*s2).flags as libc::c_int & 0x8 as libc::c_int == 0
                    || (*s2).sflags & 0x80 as libc::c_int as libc::c_uint != 0)
            {
                visible = 1 as libc::c_int;
            }
            if (*s2).sflags & 0x10 as libc::c_int as libc::c_uint != 0 {
                break;
            }
        }
        if ((*s2).next).is_null() {
            while !((*s2).abc_type as libc::c_int == 4 as libc::c_int) {
                s2 = (*s2).prev;
            }
            s = sym_dup(s2);
            (*s).next = (*s2).next;
            if !((*s).next).is_null() {
                (*(*s).next).prev = s;
            }
            (*s2).next = s;
            (*s).prev = s2;
            (*s).ts_next = (*s2).ts_next;
            if !((*s).ts_next).is_null() {
                (*(*s).ts_next).ts_prev = s;
            }
            (*s2).ts_next = s;
            (*s).ts_prev = s2;
            (*s).sflags &= 0x80000 as libc::c_int as libc::c_uint;
            (*s).sflags
                |= (0x10 as libc::c_int | 0x1000000 as libc::c_int) as libc::c_uint;
            (*s).u.note.slur_st = 0 as libc::c_int as libc::c_uchar;
            (*s).u.note.slur_end = 0 as libc::c_int as libc::c_char;
            (*s).x += 12 as libc::c_int as libc::c_float;
            if (*s).x < realwidth - 12 as libc::c_int as libc::c_float {
                (*s).x = realwidth - 12 as libc::c_int as libc::c_float;
            }
            s2 = s;
            notes += 1;
            notes;
            break;
        } else {
            s2 = (*s2).next;
        }
    }
    if visible == 0 {
        return 0 as libc::c_int;
    }
    (*bm).s2 = s2;
    if staff_tb[staff as usize].y == 0 as libc::c_int as libc::c_float {
        if two_staves != 0 {
            return 0 as libc::c_int;
        }
    } else if two_staves == 0 {
        (*bm).s1 = s1;
        if (*s1).xs == (*s2).xs {
            bug(
                b"beam with null length\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                1 as libc::c_int,
            );
        }
        (*bm).a = ((*s1).ys - (*s2).ys) / ((*s1).xs - (*s2).xs);
        (*bm).b = (*s1).ys - (*s1).xs * (*bm).a + staff_tb[staff as usize].y;
        (*bm).nflags = nflags as libc::c_short;
        return 1 as libc::c_int;
    }
    syy = 0 as libc::c_int as libc::c_float;
    sxy = syy;
    sxx = sxy;
    sy = sxx;
    sx = sy;
    s = s1;
    loop {
        if !((*s).abc_type as libc::c_int != 4 as libc::c_int) {
            scale = voice_tb[(*s).voice as usize].scale;
            if scale == 1 as libc::c_int as libc::c_float {
                scale = staff_tb[(*s).staff as usize].staffscale;
            }
            if (*s).stem as libc::c_int >= 0 as libc::c_int {
                x = stem_xoff + (*s).u.note.notes[0 as libc::c_int as usize].shhd;
            } else {
                x = -stem_xoff + (*s).u.note.notes[(*s).nhd as usize].shhd;
            }
            x *= scale;
            x += (*s).x;
            (*s).xs = x;
            y = (*s).ys + staff_tb[(*s).staff as usize].y;
            sx += x;
            sy += y;
            sxx += x * x;
            sxy += x * y;
            syy += y * y;
            if s == s2 {
                break;
            }
        }
        s = (*s).next;
    }
    a = (sxy * notes as libc::c_float - sx * sy)
        / (sxx * notes as libc::c_float - sx * sx);
    b = (sy - a * sx) / notes as libc::c_float;
    if (*s1).flags as libc::c_int & 0x20 as libc::c_int == 0 {
        if notes >= 3 as libc::c_int {
            let mut hh: libc::c_float = 0.;
            hh = syy - a * sxy - b * sy;
            if hh > 0 as libc::c_int as libc::c_float
                && (hh / (notes - 2 as libc::c_int) as libc::c_float) as libc::c_double
                    > 0.5f64
            {
                a = (a as libc::c_double * 0.6f64) as libc::c_float;
            }
        }
        if a >= 0 as libc::c_int as libc::c_float {
            a = (0.5f64 * a as libc::c_double / (0.5f64 + a as libc::c_double))
                as libc::c_float;
        } else {
            a = (0.5f64 * a as libc::c_double / (0.5f64 - a as libc::c_double))
                as libc::c_float;
        }
    } else if a as libc::c_double > 0.5f64 {
        a = 0.5f64 as libc::c_float;
    } else if (a as libc::c_double) < -0.5f64 {
        a = -0.5f64 as libc::c_float;
    }
    a0 = a * ((*s2).xs - (*s1).xs)
        / (20 as libc::c_int * (notes - 1 as libc::c_int)) as libc::c_float;
    if ((a0 * a0) as libc::c_double) < 0.06f64 * 0.06f64 {
        a = 0 as libc::c_int as libc::c_float;
    }
    if cfmt.flatbeamgracing != 0 && (*s1).flags as libc::c_int & 0x20 as libc::c_int != 0
    {
        a = 0 as libc::c_int as libc::c_float;
    }
    if cfmt.flatbeams != 0 {
        a = 0 as libc::c_int as libc::c_float;
    }
    b = (sy - a * sx) / notes as libc::c_float;
    max_stem_err = 0 as libc::c_int as libc::c_float;
    s = s1;
    if two_dir != 0 {
        if (*s1).flags as libc::c_int & 0x20 as libc::c_int == 0 {
            ys = 5.0f64 as libc::c_float;
        } else {
            ys = 3.5f64 as libc::c_float;
        }
        ys *= (nflags - 1 as libc::c_int) as libc::c_float;
        ys = (ys as libc::c_double + 3.2f64) as libc::c_float;
        ys = (ys as libc::c_double * 0.5f64) as libc::c_float;
        if (*s1).stem as libc::c_int != (*s2).stem as libc::c_int
            && ((*s1).nflags as libc::c_int) < (*s2).nflags as libc::c_int
        {
            ys *= (*s2).stem as libc::c_int as libc::c_float;
        } else {
            ys *= (*s1).stem as libc::c_int as libc::c_float;
        }
        b += ys;
    } else if (*s1).flags as libc::c_int & 0x20 as libc::c_int == 0 {
        let mut stem_err: libc::c_float = 0.;
        let mut beam_h: libc::c_float = 0.;
        beam_h = (3.2f64 + 5.0f64 * (nflags - 1 as libc::c_int) as libc::c_double)
            as libc::c_float;
        while (*(*s).ts_prev).abc_type as libc::c_int == 4 as libc::c_int
            && (*(*s).ts_prev).time == (*s).time && (*(*s).ts_prev).x > (*s1).xs
        {
            s = (*s).ts_prev;
        }
        let mut current_block_170: u64;
        while !s.is_null() && (*s).time <= (*s2).time {
            if !((*s).abc_type as libc::c_int != 4 as libc::c_int
                || (*s).flags as libc::c_int & 0x2 as libc::c_int != 0
                || (*s).staff as libc::c_int != staff
                    && (*s).voice as libc::c_int != voice)
            {
                x = if (*s).voice as libc::c_int == voice { (*s).xs } else { (*s).x };
                ys = a * x + b - staff_tb[(*s).staff as usize].y;
                if (*s).voice as libc::c_int == voice {
                    if (*s).nhd as libc::c_int == 0 as libc::c_int {
                        stem_err = min_tb[0 as libc::c_int
                            as usize][(*s).nflags as libc::c_uint as usize];
                    } else {
                        stem_err = min_tb[1 as libc::c_int
                            as usize][(*s).nflags as libc::c_uint as usize];
                    }
                    if (*s).stem as libc::c_int > 0 as libc::c_int {
                        if (*s).pits[(*s).nhd as usize] as libc::c_int
                            > 26 as libc::c_int
                        {
                            stem_err -= 2 as libc::c_int as libc::c_float;
                            if (*s).pits[(*s).nhd as usize] as libc::c_int
                                > 28 as libc::c_int
                            {
                                stem_err -= 2 as libc::c_int as libc::c_float;
                            }
                        }
                        stem_err
                            -= ys
                                - (3 as libc::c_int
                                    * ((*s).pits[(*s).nhd as usize] as libc::c_int
                                        - 18 as libc::c_int)) as libc::c_float;
                    } else {
                        if ((*s).pits[0 as libc::c_int as usize] as libc::c_int)
                            < 18 as libc::c_int
                        {
                            stem_err -= 2 as libc::c_int as libc::c_float;
                            if ((*s).pits[0 as libc::c_int as usize] as libc::c_int)
                                < 16 as libc::c_int
                            {
                                stem_err -= 2 as libc::c_int as libc::c_float;
                            }
                        }
                        stem_err
                            -= (3 as libc::c_int
                                * ((*s).pits[0 as libc::c_int as usize] as libc::c_int
                                    - 18 as libc::c_int)) as libc::c_float - ys;
                    }
                    stem_err = (stem_err as libc::c_double
                        + (3.2f64
                            + 5.0f64
                                * ((*s).nflags as libc::c_int - 1 as libc::c_int)
                                    as libc::c_double)) as libc::c_float;
                    current_block_170 = 433373112845341403;
                } else {
                    if (*s1).stem as libc::c_int > 0 as libc::c_int {
                        if (*s).stem as libc::c_int > 0 as libc::c_int {
                            if (*s).ymn as libc::c_int as libc::c_float
                                > ys + 4 as libc::c_int as libc::c_float
                                || ((*s).ymx as libc::c_int as libc::c_float)
                                    < ys - beam_h - 2 as libc::c_int as libc::c_float
                            {
                                current_block_170 = 2956972668325154207;
                            } else {
                                if (*s).voice as libc::c_int > voice {
                                    stem_err = (*s).ymx as libc::c_int as libc::c_float - ys;
                                } else {
                                    stem_err = ((*s).ymn as libc::c_int + 8 as libc::c_int)
                                        as libc::c_float - ys;
                                }
                                current_block_170 = 4367251730605750521;
                            }
                        } else {
                            stem_err = (*s).ymx as libc::c_int as libc::c_float - ys;
                            current_block_170 = 4367251730605750521;
                        }
                    } else if ((*s).stem as libc::c_int) < 0 as libc::c_int {
                        if ((*s).ymx as libc::c_int as libc::c_float)
                            < ys - 4 as libc::c_int as libc::c_float
                            || (*s).ymn as libc::c_int as libc::c_float
                                > ys - beam_h - 2 as libc::c_int as libc::c_float
                        {
                            current_block_170 = 2956972668325154207;
                        } else {
                            if ((*s).voice as libc::c_int) < voice {
                                stem_err = ys - (*s).ymn as libc::c_int as libc::c_float;
                            } else {
                                stem_err = ys - (*s).ymx as libc::c_int as libc::c_float
                                    + 8 as libc::c_int as libc::c_float;
                            }
                            current_block_170 = 4367251730605750521;
                        }
                    } else {
                        stem_err = ys - (*s).ymn as libc::c_int as libc::c_float;
                        current_block_170 = 4367251730605750521;
                    }
                    match current_block_170 {
                        2956972668325154207 => {}
                        _ => {
                            stem_err += 2 as libc::c_int as libc::c_float + beam_h;
                            current_block_170 = 433373112845341403;
                        }
                    }
                }
                match current_block_170 {
                    2956972668325154207 => {}
                    _ => {
                        if stem_err > max_stem_err {
                            max_stem_err = stem_err;
                        }
                    }
                }
            }
            s = (*s).ts_next;
        }
    } else {
        loop {
            let mut stem_err_0: libc::c_float = 0.;
            ys = a * (*s).xs + b - staff_tb[(*s).staff as usize].y;
            stem_err_0 = (15 as libc::c_int - 2 as libc::c_int) as libc::c_float;
            if (*s).stem as libc::c_int > 0 as libc::c_int {
                stem_err_0
                    -= ys
                        - (3 as libc::c_int
                            * ((*s).pits[(*s).nhd as usize] as libc::c_int
                                - 18 as libc::c_int)) as libc::c_float;
            } else {
                stem_err_0
                    += ys
                        - (3 as libc::c_int
                            * ((*s).pits[0 as libc::c_int as usize] as libc::c_int
                                - 18 as libc::c_int)) as libc::c_float;
            }
            stem_err_0
                += (3 as libc::c_int * ((*s).nflags as libc::c_int - 1 as libc::c_int))
                    as libc::c_float;
            if stem_err_0 > max_stem_err {
                max_stem_err = stem_err_0;
            }
            if s == s2 {
                break;
            }
            s = (*s).next;
        }
    }
    if max_stem_err > 0 as libc::c_int as libc::c_float {
        b += (*s1).stem as libc::c_int as libc::c_float * max_stem_err;
    }
    if two_staves == 0 && two_dir == 0 {
        s = (*s1).next;
        loop {
            let mut g: *mut SYMBOL = 0 as *mut SYMBOL;
            let mut current_block_210: u64;
            match (*s).type_0 as libc::c_int {
                1 => {
                    if (*s).abc_type as libc::c_int != 5 as libc::c_int {
                        current_block_210 = 2860109724005416757;
                    } else {
                        g = (*s).ts_next;
                        if g.is_null() || (*g).staff as libc::c_int != staff
                            || (*g).type_0 as libc::c_int != 1 as libc::c_int
                        {
                            current_block_210 = 2860109724005416757;
                        } else {
                            current_block_210 = 143538188824401685;
                        }
                    }
                }
                3 => {
                    current_block_210 = 143538188824401685;
                }
                4 => {
                    current_block_210 = 15734854086810835529;
                }
                11 => {
                    g = (*s).extra;
                    while !g.is_null() {
                        if !((*g).type_0 as libc::c_int != 1 as libc::c_int) {
                            y = a * (*g).x + b;
                            if (*s1).stem as libc::c_int > 0 as libc::c_int {
                                y = (((*g).ymx as libc::c_int as libc::c_float - y)
                                    as libc::c_double + 3.2f64
                                    + 5.0f64 * (nflags - 1 as libc::c_int) as libc::c_double
                                    + 2 as libc::c_int as libc::c_double) as libc::c_float;
                                if y > 0 as libc::c_int as libc::c_float {
                                    b += y;
                                }
                            } else {
                                y = (((*g).ymn as libc::c_int as libc::c_float - y)
                                    as libc::c_double - 3.2f64
                                    - 5.0f64 * (nflags - 1 as libc::c_int) as libc::c_double
                                    - 2 as libc::c_int as libc::c_double) as libc::c_float;
                                if y < 0 as libc::c_int as libc::c_float {
                                    b += y;
                                }
                            }
                        }
                        g = (*g).next;
                    }
                    current_block_210 = 2860109724005416757;
                }
                _ => {
                    current_block_210 = 2860109724005416757;
                }
            }
            match current_block_210 {
                143538188824401685 => {
                    if (*s).flags as libc::c_int & 0x2 as libc::c_int != 0 {
                        current_block_210 = 2860109724005416757;
                    } else {
                        current_block_210 = 15734854086810835529;
                    }
                }
                _ => {}
            }
            match current_block_210 {
                15734854086810835529 => {
                    y = a * (*s).x + b;
                    if (*s1).stem as libc::c_int > 0 as libc::c_int {
                        y = (((*s).ymx as libc::c_int as libc::c_float - y)
                            as libc::c_double + 3.2f64
                            + 5.0f64 * (nflags - 1 as libc::c_int) as libc::c_double
                            + 2 as libc::c_int as libc::c_double) as libc::c_float;
                        if y > 0 as libc::c_int as libc::c_float {
                            b += y;
                        }
                    } else {
                        y = (((*s).ymn as libc::c_int as libc::c_float - y)
                            as libc::c_double - 3.2f64
                            - 5.0f64 * (nflags - 1 as libc::c_int) as libc::c_double
                            - 2 as libc::c_int as libc::c_double) as libc::c_float;
                        if y < 0 as libc::c_int as libc::c_float {
                            b += y;
                        }
                    }
                }
                _ => {}
            }
            if s == s2 {
                break;
            }
            s = (*s).next;
        }
    }
    if a == 0 as libc::c_int as libc::c_float {
        b
            += b_pos(
                (*s1).flags as libc::c_int & 0x20 as libc::c_int,
                (*s1).stem as libc::c_int,
                nflags,
                b - staff_tb[staff as usize].y,
            );
    }
    s = s1;
    loop {
        let mut dy: libc::c_float = 0.;
        let mut current_block_235: u64;
        match (*s).abc_type as libc::c_int {
            4 => {
                (*s).ys = a * (*s).xs + b - staff_tb[(*s).staff as usize].y;
                if (*s).stem as libc::c_int > 0 as libc::c_int {
                    (*s).ymx = ((*s).ys as libc::c_double + 2.5f64) as libc::c_schar;
                } else {
                    (*s).ymn = ((*s).ys as libc::c_double - 2.5f64) as libc::c_schar;
                }
            }
            5 => {
                y = a * (*s).x + b - staff_tb[(*s).staff as usize].y;
                dy = (3.2f64 + 5.0f64 * (nflags - 1 as libc::c_int) as libc::c_double
                    + (if (*s).head as libc::c_int != 0 as libc::c_int {
                        4 as libc::c_int
                    } else {
                        9 as libc::c_int
                    }) as libc::c_double) as libc::c_float;
                if (*s1).stem as libc::c_int > 0 as libc::c_int {
                    y -= dy;
                    if (*s1).multi as libc::c_int == 0 as libc::c_int
                        && y > 12 as libc::c_int as libc::c_float
                    {
                        y = 12 as libc::c_int as libc::c_float;
                    }
                    if (*s).y as libc::c_int as libc::c_float <= y {
                        current_block_235 = 4746208608892471902;
                    } else {
                        current_block_235 = 15689130159044723804;
                    }
                } else {
                    y += dy;
                    if (*s1).multi as libc::c_int == 0 as libc::c_int
                        && y < 12 as libc::c_int as libc::c_float
                    {
                        y = 12 as libc::c_int as libc::c_float;
                    }
                    if (*s).y as libc::c_int as libc::c_float >= y {
                        current_block_235 = 4746208608892471902;
                    } else {
                        current_block_235 = 15689130159044723804;
                    }
                }
                match current_block_235 {
                    4746208608892471902 => {}
                    _ => {
                        if (*s).head as libc::c_int != 0 as libc::c_int {
                            let mut iy: libc::c_int = 0;
                            iy = (y as libc::c_int + 3 as libc::c_int
                                + 12 as libc::c_int) / 6 as libc::c_int * 6 as libc::c_int
                                - 12 as libc::c_int;
                            y = iy as libc::c_float;
                        }
                        (*s).y = y as libc::c_schar;
                    }
                }
            }
            _ => {}
        }
        if s == s2 {
            break;
        }
        s = (*s).next;
    }
    if staff_tb[staff as usize].y == 0 as libc::c_int as libc::c_float {
        return 0 as libc::c_int;
    }
    (*bm).s1 = s1;
    (*bm).a = a;
    (*bm).b = b;
    (*bm).nflags = nflags as libc::c_short;
    return 1 as libc::c_int;
}
unsafe extern "C" fn draw_beam(
    mut x1: libc::c_float,
    mut x2: libc::c_float,
    mut dy: libc::c_float,
    mut h: libc::c_float,
    mut bm: *mut BEAM,
    mut n: libc::c_int,
) {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut y1: libc::c_float = 0.;
    let mut dy2: libc::c_float = 0.;
    s = (*bm).s1;
    if (*s).sflags & 0x80 as libc::c_int as libc::c_uint != 0
        && n > (*s).nflags as libc::c_int - (*s).aux as libc::c_int
        && (*s).head as libc::c_int != 1 as libc::c_int
    {
        if (*s).head as libc::c_int >= 2 as libc::c_int {
            x1 = (*s).x + 6 as libc::c_int as libc::c_float;
            x2 = (*(*bm).s2).x - 6 as libc::c_int as libc::c_float;
        } else {
            x1 += 5 as libc::c_int as libc::c_float;
            x2 -= 6 as libc::c_int as libc::c_float;
        }
    }
    y1 = (*bm).a * x1 + (*bm).b - dy;
    x2 -= x1;
    dy2 = (*bm).a * x2;
    putf(h);
    putx(x2);
    putf(dy2);
    putxy(x1, y1);
    a2b(b"bm\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
}
unsafe extern "C" fn draw_beams(mut bm: *mut BEAM) {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s1: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut i: libc::c_int = 0;
    let mut beam_dir: libc::c_int = 0;
    let mut shift: libc::c_float = 0.;
    let mut bshift: libc::c_float = 0.;
    let mut bstub: libc::c_float = 0.;
    let mut bh: libc::c_float = 0.;
    let mut da: libc::c_float = 0.;
    s1 = (*bm).s1;
    s2 = (*bm).s2;
    if (*s1).flags as libc::c_int & 0x20 as libc::c_int == 0 {
        bshift = 5.0f64 as libc::c_float;
        bstub = 7.0f64 as libc::c_float;
        shift = 0.34f64 as libc::c_float;
        bh = 3.2f64 as libc::c_float;
    } else {
        bshift = 3.5f64 as libc::c_float;
        bstub = 3.2f64 as libc::c_float;
        shift = 0.29f64 as libc::c_float;
        bh = 1.8f64 as libc::c_float;
    }
    beam_dir = (*s1).stem as libc::c_int;
    if (*s1).stem as libc::c_int != (*s2).stem as libc::c_int
        && ((*s1).nflags as libc::c_int) < (*s2).nflags as libc::c_int
    {
        beam_dir = (*s2).stem as libc::c_int;
    }
    if beam_dir < 0 as libc::c_int {
        bh = -bh;
    }
    if cur_trans == 0 as libc::c_int as libc::c_float
        && cur_scale != 1 as libc::c_int as libc::c_float
    {
        (*bm).a /= cur_scale;
        (*bm).b = (*s1).ys - (*s1).xs * (*bm).a + staff_tb[(*s1).staff as usize].y;
        bshift *= cur_scale;
    }
    draw_beam(
        (*s1).xs - shift,
        (*s2).xs + shift,
        0.0f64 as libc::c_float,
        bh,
        bm,
        1 as libc::c_int,
    );
    da = 0 as libc::c_int as libc::c_float;
    s = s1;
    loop {
        if (*s).abc_type as libc::c_int == 4 as libc::c_int
            && (*s).stem as libc::c_int != beam_dir
        {
            (*s)
                .ys = (*bm).a * (*s).xs + (*bm).b - staff_tb[(*s).staff as usize].y
                + bshift
                    * ((*s).nflags as libc::c_int - 1 as libc::c_int) as libc::c_float
                    * (*s).stem as libc::c_int as libc::c_float - bh;
        }
        if s == s2 {
            break;
        }
        s = (*s).next;
    }
    if (*s1).sflags & 0x10000 as libc::c_int as libc::c_uint != 0 {
        da = bshift / ((*s2).xs - (*s1).xs);
        if (*s1).dur > (*s2).dur {
            da = -da;
            bshift = da * (*s1).xs;
        } else {
            bshift = da * (*s2).xs;
        }
        da = da * beam_dir as libc::c_float;
    }
    shift = 0 as libc::c_int as libc::c_float;
    i = 2 as libc::c_int;
    while i <= (*bm).nflags as libc::c_int {
        shift += bshift;
        if da != 0 as libc::c_int as libc::c_float {
            (*bm).a += da;
        }
        s = s1;
        loop {
            let mut k1: *mut SYMBOL = 0 as *mut SYMBOL;
            let mut k2: *mut SYMBOL = 0 as *mut SYMBOL;
            let mut x1: libc::c_float = 0.;
            if (*s).abc_type as libc::c_int != 4 as libc::c_int
                || ((*s).nflags as libc::c_int) < i
            {
                if s == s2 {
                    break;
                }
            } else if (*s).sflags & 0x800000 as libc::c_int as libc::c_uint != 0
                && i > (*s).nflags as libc::c_int - (*s).aux as libc::c_int
            {
                if (*s).head as libc::c_int >= 2 as libc::c_int {
                    x1 = (*s).x;
                } else {
                    x1 = (*s).xs;
                }
                draw_beam(
                    x1 - 5 as libc::c_int as libc::c_float,
                    x1 + 5 as libc::c_int as libc::c_float,
                    ((shift as libc::c_double + 2.5f64) * beam_dir as libc::c_double)
                        as libc::c_float,
                    bh,
                    bm,
                    i,
                );
                if s == s2 {
                    break;
                }
            } else {
                k1 = s;
                while !(s == s2) {
                    if (*(*s).next).type_0 as libc::c_int == 1 as libc::c_int {
                        if (*(*s).next).sflags & 0x800000 as libc::c_int as libc::c_uint
                            != 0
                        {
                            if ((*(*s).next).nflags as libc::c_int
                                - (*(*s).next).aux as libc::c_int) < i
                            {
                                break;
                            }
                        } else if ((*(*s).next).nflags as libc::c_int) < i {
                            break;
                        }
                    }
                    if (*(*s).next).sflags & 0x4 as libc::c_int as libc::c_uint != 0
                        || (*(*s).next).sflags & 0x8 as libc::c_int as libc::c_uint != 0
                            && i > 2 as libc::c_int
                    {
                        break;
                    }
                    s = (*s).next;
                }
                k2 = s;
                while (*k2).abc_type as libc::c_int != 4 as libc::c_int {
                    k2 = (*k2).prev;
                }
                x1 = (*k1).xs;
                if k1 == k2 {
                    if k1 == s1 {
                        x1 += bstub;
                    } else if k1 == s2 {
                        x1 -= bstub;
                    } else if (*k1).sflags & 0x4 as libc::c_int as libc::c_uint != 0
                        || (*k1).sflags & 0x8 as libc::c_int as libc::c_uint != 0
                            && i > 2 as libc::c_int
                    {
                        x1 += bstub;
                    } else {
                        let mut k: *mut SYMBOL = 0 as *mut SYMBOL;
                        k = (*k1).next;
                        while (*k).abc_type as libc::c_int != 4 as libc::c_int {
                            k = (*k).next;
                        }
                        if (*k).sflags & 0x4 as libc::c_int as libc::c_uint != 0
                            || (*k).sflags & 0x8 as libc::c_int as libc::c_uint != 0
                                && i > 2 as libc::c_int
                        {
                            x1 -= bstub;
                        } else {
                            k1 = (*k1).prev;
                            while (*k1).abc_type as libc::c_int != 4 as libc::c_int {
                                k1 = (*k1).prev;
                            }
                            if ((*k1).nflags as libc::c_int) < (*k).nflags as libc::c_int
                                || (*k1).nflags as libc::c_int == (*k).nflags as libc::c_int
                                    && ((*k1).dots as libc::c_int) < (*k).dots as libc::c_int
                            {
                                x1 += bstub;
                            } else {
                                x1 -= bstub;
                            }
                        }
                    }
                }
                draw_beam(x1, (*k2).xs, shift * beam_dir as libc::c_float, bh, bm, i);
                if s == s2 {
                    break;
                }
            }
            s = (*s).next;
        }
        i += 1;
        i;
    }
    if (*s1).sflags & 0x1000000 as libc::c_int as libc::c_uint != 0 {
        unlksym(s1);
    } else if (*s2).sflags & 0x1000000 as libc::c_int as libc::c_uint != 0 {
        unlksym(s2);
    }
}
unsafe extern "C" fn draw_sysbra(
    mut x: libc::c_float,
    mut staff: libc::c_int,
    mut flag: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut yt: libc::c_float = 0.;
    let mut yb: libc::c_float = 0.;
    while (*cursys).staff[staff as usize].empty != 0 {
        if (*cursys).staff[staff as usize].flags as libc::c_int & flag != 0 {
            return;
        }
        staff += 1;
        staff;
    }
    end = staff;
    i = end;
    loop {
        if (*cursys).staff[i as usize].empty == 0 {
            end = i;
        }
        if (*cursys).staff[i as usize].flags as libc::c_int & flag != 0 {
            break;
        }
        i += 1;
        i;
    }
    yt = staff_tb[staff as usize].y
        + staff_tb[staff as usize].topbar as libc::c_int as libc::c_float
            * staff_tb[staff as usize].staffscale;
    yb = staff_tb[end as usize].y
        + staff_tb[end as usize].botbar as libc::c_int as libc::c_float
            * staff_tb[end as usize].staffscale;
    a2b(
        b"%.1f %.1f %.1f %s\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        (yt - yb) as libc::c_double,
        x as libc::c_double,
        yt as libc::c_double,
        if flag & (0x2 as libc::c_int | 0x200 as libc::c_int) != 0 {
            b"brace\0" as *const u8 as *const libc::c_char
        } else {
            b"bracket\0" as *const u8 as *const libc::c_char
        },
    );
}
unsafe extern "C" fn draw_lstaff(mut x: libc::c_float) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut nst: libc::c_int = 0;
    let mut yb: libc::c_float = 0.;
    if cfmt.alignbars != 0 {
        return;
    }
    nst = (*cursys).nstaff as libc::c_int;
    l = 0 as libc::c_int;
    i = 0 as libc::c_int;
    loop {
        if (*cursys).staff[i as usize].flags as libc::c_int
            & (0x1 as libc::c_int | 0x4 as libc::c_int) != 0
        {
            l += 1;
            l;
        }
        if (*cursys).staff[i as usize].empty == 0 {
            break;
        }
        if (*cursys).staff[i as usize].flags as libc::c_int
            & (0x2 as libc::c_int | 0x8 as libc::c_int) != 0
        {
            l -= 1;
            l;
        }
        if i == nst {
            break;
        }
        i += 1;
        i;
    }
    j = nst;
    while j > i {
        if (*cursys).staff[j as usize].empty == 0 {
            break;
        }
        j -= 1;
        j;
    }
    if i == j && l == 0 as libc::c_int {
        return;
    }
    set_sscale(-(1 as libc::c_int));
    yb = staff_tb[j as usize].y
        + staff_tb[j as usize].botbar as libc::c_int as libc::c_float
            * staff_tb[j as usize].staffscale;
    a2b(
        b"%.1f %.1f %.1f bar\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        (staff_tb[i as usize].y
            + staff_tb[i as usize].topbar as libc::c_int as libc::c_float
                * staff_tb[i as usize].staffscale - yb) as libc::c_double,
        x as libc::c_double,
        yb as libc::c_double,
    );
    i = 0 as libc::c_int;
    while i <= nst {
        if (*cursys).staff[i as usize].flags as libc::c_int & 0x1 as libc::c_int != 0 {
            draw_sysbra(x, i, 0x2 as libc::c_int);
        }
        if (*cursys).staff[i as usize].flags as libc::c_int & 0x4 as libc::c_int != 0 {
            draw_sysbra(x, i, 0x8 as libc::c_int);
        }
        if (*cursys).staff[i as usize].flags as libc::c_int & 0x100 as libc::c_int != 0 {
            draw_sysbra(x - 6 as libc::c_int as libc::c_float, i, 0x200 as libc::c_int);
        }
        if (*cursys).staff[i as usize].flags as libc::c_int & 0x400 as libc::c_int != 0 {
            draw_sysbra(x - 6 as libc::c_int as libc::c_float, i, 0x800 as libc::c_int);
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn draw_staff(
    mut staff: libc::c_int,
    mut x1: libc::c_float,
    mut x2: libc::c_float,
) {
    let mut stafflines: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut thick: libc::c_int = -(1 as libc::c_int);
    let mut y: libc::c_float = 0.;
    let mut w: libc::c_float = 0.;
    set_sscale(staff);
    y = staff_tb[staff as usize].y;
    stafflines = staff_tb[staff as usize].stafflines;
    l = strlen(stafflines) as libc::c_int;
    i = 0 as libc::c_int;
    while i < l {
        if *stafflines.offset(i as isize) as libc::c_int != '.' as i32 {
            w = x2 - x1;
            while i < l {
                if *stafflines.offset(i as isize) as libc::c_int != '.' as i32 {
                    if *stafflines.offset(i as isize) as libc::c_int != '|' as i32 {
                        if thick != 1 as libc::c_int {
                            if thick >= 0 as libc::c_int {
                                a2b(
                                    b"stroke\n\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                );
                            }
                            a2b(
                                b"1.5 SLW \0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                            thick = 1 as libc::c_int;
                        }
                    } else if thick != 0 as libc::c_int {
                        if thick >= 0 as libc::c_int {
                            a2b(
                                b"stroke\n\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                        }
                        a2b(
                            b"dlw \0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                        thick = 0 as libc::c_int;
                    }
                    putx(w);
                    putxy(x1, y);
                    a2b(
                        b"M 0 RL \0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                }
                y += 6 as libc::c_int as libc::c_float;
                i += 1;
                i;
            }
            a2b(b"stroke\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
            break;
        } else {
            y += 6 as libc::c_int as libc::c_float;
            i += 1;
            i;
        }
    }
}
unsafe extern "C" fn draw_timesig(mut x: libc::c_float, mut s: *mut SYMBOL) {
    let mut i: libc::c_uint = 0;
    let mut staff: libc::c_uint = 0;
    let mut l: libc::c_uint = 0;
    let mut l2: libc::c_uint = 0;
    let mut f: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut meter: [libc::c_char; 64] = [0; 64];
    let mut dx: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    if (*s).u.meter.nmeter as libc::c_int == 0 as libc::c_int {
        return;
    }
    staff = (*s).staff as libc::c_uint;
    x -= (*s).wl;
    y = staff_tb[staff as usize].y;
    let mut current_block_50: u64;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*s).u.meter.nmeter as libc::c_uint {
        l = strlen(((*s).u.meter.meter[i as usize].top).as_mut_ptr()) as libc::c_uint;
        if l as libc::c_ulong
            > ::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong
        {
            l = ::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong
                as libc::c_uint;
        }
        if (*s).u.meter.meter[i as usize].bot[0 as libc::c_int as usize] as libc::c_int
            != '\0' as i32
        {
            sprintf(
                meter.as_mut_ptr(),
                b"(%.8s)(%.2s)\0" as *const u8 as *const libc::c_char,
                ((*s).u.meter.meter[i as usize].top).as_mut_ptr(),
                ((*s).u.meter.meter[i as usize].bot).as_mut_ptr(),
            );
            f = b"tsig\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            l2 = strlen(((*s).u.meter.meter[i as usize].bot).as_mut_ptr())
                as libc::c_uint;
            if l2 as libc::c_ulong
                > ::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
            {
                l2 = ::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                    as libc::c_uint;
            }
            if l2 > l {
                l = l2;
            }
            current_block_50 = 6450597802325118133;
        } else {
            match (*s).u.meter.meter[i as usize].top[0 as libc::c_int as usize]
                as libc::c_int
            {
                67 => {
                    if (*s).u.meter.meter[i as usize].top[1 as libc::c_int as usize]
                        as libc::c_int != '|' as i32
                    {
                        f = b"csig\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    } else {
                        f = b"ctsig\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                        l = l.wrapping_sub(1);
                        l;
                    }
                    dx = (13 as libc::c_int as libc::c_uint).wrapping_mul(l)
                        as libc::c_float;
                    putxy(
                        ((x - 5 as libc::c_int as libc::c_float) as libc::c_double
                            + dx as libc::c_double * 0.5f64) as libc::c_float,
                        y + 12 as libc::c_int as libc::c_float,
                    );
                    a2b(
                        b"%s\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        f,
                    );
                    x += dx;
                    current_block_50 = 820271813250567934;
                }
                99 => {
                    if (*s).u.meter.meter[i as usize].top[1 as libc::c_int as usize]
                        as libc::c_int != '.' as i32
                    {
                        f = b"imsig\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    } else {
                        f = b"iMsig\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                        l = l.wrapping_sub(1);
                        l;
                    }
                    meter[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                    current_block_50 = 6450597802325118133;
                }
                111 => {
                    if (*s).u.meter.meter[i as usize].top[1 as libc::c_int as usize]
                        as libc::c_int != '.' as i32
                    {
                        f = b"pmsig\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    } else {
                        f = b"pMsig\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                        l = l.wrapping_sub(1);
                        l;
                    }
                    meter[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                    current_block_50 = 6450597802325118133;
                }
                40 | 41 => {
                    sprintf(
                        meter.as_mut_ptr(),
                        b"(\\%s)\0" as *const u8 as *const libc::c_char,
                        ((*s).u.meter.meter[i as usize].top).as_mut_ptr(),
                    );
                    f = b"stsig\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    current_block_50 = 6450597802325118133;
                }
                _ => {
                    sprintf(
                        meter.as_mut_ptr(),
                        b"(%.8s)\0" as *const u8 as *const libc::c_char,
                        ((*s).u.meter.meter[i as usize].top).as_mut_ptr(),
                    );
                    f = b"stsig\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    current_block_50 = 6450597802325118133;
                }
            }
        }
        match current_block_50 {
            6450597802325118133 => {
                if meter[0 as libc::c_int as usize] as libc::c_int != '\0' as i32 {
                    a2b(
                        b"%s \0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        meter.as_mut_ptr(),
                    );
                }
                dx = (13 as libc::c_int as libc::c_uint).wrapping_mul(l)
                    as libc::c_float;
                putxy(
                    (x as libc::c_double + dx as libc::c_double * 0.5f64)
                        as libc::c_float,
                    y,
                );
                a2b(
                    b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    f,
                );
                x += dx;
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn draw_acc(mut acc: libc::c_int, mut microscale: libc::c_int) {
    let mut n: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    n = parse.micro_tb[(acc >> 3 as libc::c_int) as usize] as libc::c_int;
    if acc >> 3 as libc::c_int != 0 as libc::c_int && microscale != 0 {
        if microscale != 0 {
            d = microscale;
            n = acc >> 3 as libc::c_int;
        } else {
            d = ((n & 0xff as libc::c_int) + 1 as libc::c_int) * 2 as libc::c_int;
            n = (n >> 8 as libc::c_int) + 1 as libc::c_int;
        }
        a2b(
            b"%d %s%d \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            n,
            acc_tb[(acc & 0x7 as libc::c_int) as usize],
            d,
        );
    } else {
        if acc >> 3 as libc::c_int != 0 as libc::c_int && cfmt.nedo != 0 {
            n = (((n >> 8 as libc::c_int) + 1 as libc::c_int) * 12 as libc::c_int
                - 1 as libc::c_int) * 256 as libc::c_int + cfmt.nedo - 1 as libc::c_int;
        }
        a2b(
            b"%s%d \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            acc_tb[(acc & 0x7 as libc::c_int) as usize],
            n,
        );
    };
}
unsafe extern "C" fn draw_hl(
    mut x: libc::c_float,
    mut staffb: libc::c_float,
    mut up: libc::c_int,
    mut y: libc::c_int,
    mut stafflines: *mut libc::c_char,
    mut hltype: *mut libc::c_char,
) {
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    l = strlen(stafflines) as libc::c_int;
    if up == 0 {
        i = 0 as libc::c_int;
        while i < l - 1 as libc::c_int {
            if *stafflines.offset(i as isize) as libc::c_int != '.' as i32 {
                break;
            }
            i += 1;
            i;
        }
        i = i * 6 as libc::c_int - 6 as libc::c_int;
        while i >= y {
            putxy(x, staffb + i as libc::c_float);
            a2b(
                b"%s \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                hltype,
            );
            i -= 6 as libc::c_int;
        }
        return;
    }
    i = l * 6 as libc::c_int;
    while i <= y {
        putxy(x, staffb + i as libc::c_float);
        a2b(b"%s \0" as *const u8 as *const libc::c_char as *mut libc::c_char, hltype);
        i += 6 as libc::c_int;
    }
}
unsafe extern "C" fn draw_keysig(
    mut p_voice: *mut VOICE_S,
    mut x: libc::c_float,
    mut s: *mut SYMBOL,
) {
    let mut old_sf: libc::c_int = (*s).aux as libc::c_int;
    let mut staff: libc::c_int = (*p_voice).staff as libc::c_int;
    let mut staffb: libc::c_float = staff_tb[staff as usize].y;
    let mut i: libc::c_int = 0;
    let mut clef_ix: libc::c_int = 0;
    let mut shift: libc::c_int = 0;
    let mut p_seq: *const libc::c_schar = 0 as *const libc::c_schar;
    static mut sharp_cl: [libc::c_char; 7] = [
        24 as libc::c_int as libc::c_char,
        9 as libc::c_int as libc::c_char,
        15 as libc::c_int as libc::c_char,
        21 as libc::c_int as libc::c_char,
        6 as libc::c_int as libc::c_char,
        12 as libc::c_int as libc::c_char,
        18 as libc::c_int as libc::c_char,
    ];
    static mut flat_cl: [libc::c_char; 7] = [
        12 as libc::c_int as libc::c_char,
        18 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        9 as libc::c_int as libc::c_char,
        15 as libc::c_int as libc::c_char,
        21 as libc::c_int as libc::c_char,
        6 as libc::c_int as libc::c_char,
    ];
    static mut sharp1: [libc::c_schar; 7] = [
        -(9 as libc::c_int) as libc::c_schar,
        12 as libc::c_int as libc::c_schar,
        -(9 as libc::c_int) as libc::c_schar,
        -(9 as libc::c_int) as libc::c_schar,
        12 as libc::c_int as libc::c_schar,
        -(9 as libc::c_int) as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
    ];
    static mut sharp2: [libc::c_schar; 7] = [
        12 as libc::c_int as libc::c_schar,
        -(9 as libc::c_int) as libc::c_schar,
        12 as libc::c_int as libc::c_schar,
        -(9 as libc::c_int) as libc::c_schar,
        12 as libc::c_int as libc::c_schar,
        -(9 as libc::c_int) as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
    ];
    static mut flat1: [libc::c_schar; 7] = [
        9 as libc::c_int as libc::c_schar,
        -(12 as libc::c_int) as libc::c_schar,
        9 as libc::c_int as libc::c_schar,
        -(12 as libc::c_int) as libc::c_schar,
        9 as libc::c_int as libc::c_schar,
        -(12 as libc::c_int) as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
    ];
    static mut flat2: [libc::c_schar; 7] = [
        -(12 as libc::c_int) as libc::c_schar,
        9 as libc::c_int as libc::c_schar,
        -(12 as libc::c_int) as libc::c_schar,
        9 as libc::c_int as libc::c_schar,
        -(12 as libc::c_int) as libc::c_schar,
        9 as libc::c_int as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
    ];
    clef_ix = (*s).u.key.clef_delta as libc::c_int;
    if clef_ix & 1 as libc::c_int != 0 {
        clef_ix += 7 as libc::c_int;
    }
    clef_ix /= 2 as libc::c_int;
    while clef_ix < 0 as libc::c_int {
        clef_ix += 7 as libc::c_int;
    }
    clef_ix %= 7 as libc::c_int;
    if (*s).u.key.nacc as libc::c_int == 0 as libc::c_int && (*s).u.key.empty == 0 {
        if cfmt.cancelkey != 0 || (*s).u.key.sf as libc::c_int == 0 as libc::c_int {
            if (*s).u.key.sf as libc::c_int == 0 as libc::c_int
                || (old_sf * (*s).u.key.sf as libc::c_int) < 0 as libc::c_int
            {
                shift = sharp_cl[clef_ix as usize] as libc::c_int;
                p_seq = if shift > 9 as libc::c_int {
                    sharp1.as_ptr()
                } else {
                    sharp2.as_ptr()
                };
                i = 0 as libc::c_int;
                while i < old_sf {
                    putxy(x, staffb + shift as libc::c_float);
                    a2b(
                        b"nt0 \0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    let fresh1 = p_seq;
                    p_seq = p_seq.offset(1);
                    shift += *fresh1 as libc::c_int;
                    x = (x as libc::c_double + 5.5f64) as libc::c_float;
                    i += 1;
                    i;
                }
                shift = flat_cl[clef_ix as usize] as libc::c_int;
                p_seq = if shift < 18 as libc::c_int {
                    flat1.as_ptr()
                } else {
                    flat2.as_ptr()
                };
                i = 0 as libc::c_int;
                while i > old_sf {
                    putxy(x, staffb + shift as libc::c_float);
                    a2b(
                        b"nt0 \0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    let fresh2 = p_seq;
                    p_seq = p_seq.offset(1);
                    shift += *fresh2 as libc::c_int;
                    x = (x as libc::c_double + 5.5f64) as libc::c_float;
                    i -= 1;
                    i;
                }
                if (*s).u.key.sf as libc::c_int != 0 as libc::c_int {
                    x += 3 as libc::c_int as libc::c_float;
                }
            }
        }
        if (*s).u.key.sf as libc::c_int > 0 as libc::c_int {
            shift = sharp_cl[clef_ix as usize] as libc::c_int;
            p_seq = if shift > 9 as libc::c_int {
                sharp1.as_ptr()
            } else {
                sharp2.as_ptr()
            };
            i = 0 as libc::c_int;
            while i < (*s).u.key.sf as libc::c_int {
                putxy(x, staffb + shift as libc::c_float);
                a2b(b"sh0 \0" as *const u8 as *const libc::c_char as *mut libc::c_char);
                let fresh3 = p_seq;
                p_seq = p_seq.offset(1);
                shift += *fresh3 as libc::c_int;
                x = (x as libc::c_double + 5.5f64) as libc::c_float;
                i += 1;
                i;
            }
            if cfmt.cancelkey != 0 && ((*s).u.key.sf as libc::c_int) < old_sf {
                x += 2 as libc::c_int as libc::c_float;
                while i < old_sf {
                    putxy(x, staffb + shift as libc::c_float);
                    a2b(
                        b"nt0 \0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    let fresh4 = p_seq;
                    p_seq = p_seq.offset(1);
                    shift += *fresh4 as libc::c_int;
                    x = (x as libc::c_double + 5.5f64) as libc::c_float;
                    i += 1;
                    i;
                }
            }
        }
        if ((*s).u.key.sf as libc::c_int) < 0 as libc::c_int {
            shift = flat_cl[clef_ix as usize] as libc::c_int;
            p_seq = if shift < 18 as libc::c_int {
                flat1.as_ptr()
            } else {
                flat2.as_ptr()
            };
            i = 0 as libc::c_int;
            while i > (*s).u.key.sf as libc::c_int {
                putxy(x, staffb + shift as libc::c_float);
                a2b(b"ft0 \0" as *const u8 as *const libc::c_char as *mut libc::c_char);
                let fresh5 = p_seq;
                p_seq = p_seq.offset(1);
                shift += *fresh5 as libc::c_int;
                x = (x as libc::c_double + 5.5f64) as libc::c_float;
                i -= 1;
                i;
            }
            if cfmt.cancelkey != 0 && (*s).u.key.sf as libc::c_int > old_sf {
                x += 2 as libc::c_int as libc::c_float;
                while i > old_sf {
                    putxy(x, staffb + shift as libc::c_float);
                    a2b(
                        b"nt0 \0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    let fresh6 = p_seq;
                    p_seq = p_seq.offset(1);
                    shift += *fresh6 as libc::c_int;
                    x = (x as libc::c_double + 5.5f64) as libc::c_float;
                    i -= 1;
                    i;
                }
            }
        }
    } else {
        let mut acc: libc::c_int = 0;
        let mut last_acc: libc::c_int = 0;
        let mut last_shift: libc::c_int = 0;
        last_acc = (*s).u.key.accs[0 as libc::c_int as usize] as libc::c_int;
        last_shift = 100 as libc::c_int;
        i = 0 as libc::c_int;
        while i < (*s).u.key.nacc as libc::c_int {
            acc = (*s).u.key.accs[i as usize] as libc::c_int;
            shift = (*s).u.key.clef_delta as libc::c_int * 3 as libc::c_int
                + 3 as libc::c_int
                    * ((*s).u.key.pits[i as usize] as libc::c_int - 18 as libc::c_int);
            if i != 0 as libc::c_int
                && (shift > last_shift + 18 as libc::c_int
                    || shift < last_shift - 18 as libc::c_int)
            {
                x = (x as libc::c_double - 5.5f64) as libc::c_float;
            } else if acc != last_acc {
                x += 3 as libc::c_int as libc::c_float;
            }
            last_acc = acc;
            if shift < 0 as libc::c_int {
                draw_hl(
                    x,
                    staffb,
                    0 as libc::c_int,
                    shift,
                    staff_tb[(*s).staff as usize].stafflines,
                    b"hl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            } else if shift > 24 as libc::c_int {
                draw_hl(
                    x,
                    staffb,
                    1 as libc::c_int,
                    shift,
                    staff_tb[(*s).staff as usize].stafflines,
                    b"hl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            last_shift = shift;
            putxy(x, staffb + shift as libc::c_float);
            draw_acc(acc, (*s).u.key.microscale as libc::c_int);
            x = (x as libc::c_double + 5.5f64) as libc::c_float;
            i += 1;
            i;
        }
    }
    if old_sf != 0 as libc::c_int || (*s).u.key.sf as libc::c_int != 0 as libc::c_int
        || (*s).u.key.nacc as libc::c_int != 0 as libc::c_int
    {
        a2b(b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    }
}
unsafe extern "C" fn bar_cnv(mut bar_type: libc::c_int) -> libc::c_int {
    let mut current_block_6: u64;
    match bar_type {
        2 => {
            current_block_6 = 16397181454952429878;
        }
        35 => {
            current_block_6 = 16397181454952429878;
        }
        276 | 20 => {
            bar_type |= (2 as libc::c_int) << 8 as libc::c_int;
            current_block_6 = 13109137661213826276;
        }
        324 => {
            bar_type |= (2 as libc::c_int) << 12 as libc::c_int;
            current_block_6 = 13109137661213826276;
        }
        5188 => {
            bar_type |= (2 as libc::c_int) << 16 as libc::c_int;
            current_block_6 = 13109137661213826276;
        }
        65 | 1089 | 17473 => {
            bar_type <<= 4 as libc::c_int;
            bar_type |= 3 as libc::c_int;
            current_block_6 = 13109137661213826276;
        }
        68 => {
            bar_type = cfmt.dblrepbar;
            current_block_6 = 13109137661213826276;
        }
        _ => {
            current_block_6 = 13109137661213826276;
        }
    }
    match current_block_6 {
        13109137661213826276 => {}
        _ => return 0 as libc::c_int,
    }
    return bar_type;
}
unsafe extern "C" fn draw_bar(
    mut s: *mut SYMBOL,
    mut bot: libc::c_float,
    mut h: libc::c_float,
) {
    let mut staff: libc::c_int = 0;
    let mut bar_type: libc::c_int = 0;
    let mut x: libc::c_float = 0.;
    let mut yb: libc::c_float = 0.;
    let mut psf: *mut libc::c_char = 0 as *mut libc::c_char;
    staff = (*s).staff as libc::c_int;
    yb = staff_tb[staff as usize].y;
    x = (*s).x;
    if (*s).u.bar.len as libc::c_int != 0 as libc::c_int {
        let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
        set_scale(s);
        if (*s).u.bar.len as libc::c_int == 1 as libc::c_int {
            s2 = (*s).prev;
            while (*s2).abc_type as libc::c_int != 5 as libc::c_int {
                s2 = (*s2).prev;
            }
            putxy((*s2).x, yb + 12 as libc::c_int as libc::c_float);
            a2b(b"mrep\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        } else {
            putxy(x, yb + 12 as libc::c_int as libc::c_float);
            a2b(b"mrep2\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
            if (*s).voice as libc::c_int == (*cursys).top_voice as libc::c_int {
                set_font(cfmt.anf as libc::c_int);
                putxy(
                    x,
                    yb + staff_tb[staff as usize].topbar as libc::c_int as libc::c_float
                        + 4 as libc::c_int as libc::c_float,
                );
                a2b(
                    b"M(%d)showc\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*s).u.bar.len as libc::c_int,
                );
            }
        }
    }
    if staff != 0 as libc::c_int && !((*s).ts_prev).is_null()
        && (*(*s).ts_prev).type_0 as libc::c_int != 3 as libc::c_int
    {
        h = staff_tb[staff as usize].topbar as libc::c_int as libc::c_float
            * staff_tb[staff as usize].staffscale;
    }
    bar_type = bar_cnv((*s).u.bar.type_0);
    if bar_type == 0 as libc::c_int {
        return;
    }
    loop {
        psf = b"bar\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        match bar_type & 0x7 as libc::c_int {
            1 => {
                if (*s).u.bar.dotted != 0 {
                    psf = b"dotbar\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                x -= 1 as libc::c_int as libc::c_float;
            }
            2 | 3 => {
                psf = b"thbar\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
                x -= 3 as libc::c_int as libc::c_float;
            }
            4 => {
                x -= 2 as libc::c_int as libc::c_float;
            }
            _ => {}
        }
        match bar_type & 0x7 as libc::c_int {
            4 => {
                set_sscale(staff);
                putxy(x + 1 as libc::c_int as libc::c_float, staff_tb[staff as usize].y);
                a2b(
                    b"rdots \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            _ => {
                set_sscale(-(1 as libc::c_int));
                a2b(
                    b"%.1f %.1f %.1f %s \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    h as libc::c_double,
                    x as libc::c_double,
                    bot as libc::c_double,
                    psf,
                );
            }
        }
        bar_type >>= 4 as libc::c_int;
        if bar_type == 0 as libc::c_int {
            break;
        }
        x -= 3 as libc::c_int as libc::c_float;
    }
    a2b(b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
}
unsafe extern "C" fn draw_rest(mut s: *mut SYMBOL) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut stafflines: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut x: libc::c_float = 0.;
    let mut dotx: libc::c_float = 0.;
    let mut staffb: libc::c_float = 0.;
    static mut rest_tb: [*mut libc::c_char; 10] = [
        b"r128\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"r64\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"r32\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"r16\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"r8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"r4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"r2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"r1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"r0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"r00\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ];
    if staff_tb[(*s).staff as usize].empty != 0 {
        return;
    }
    if (*s).dur == voice_tb[(*s).voice as usize].meter.wmeasure as libc::c_int
        || (*s).sflags & 0x20000 as libc::c_int as libc::c_uint != 0
            && (*s).doty as libc::c_int >= 0 as libc::c_int
    {
        let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
        s2 = (*s).ts_next;
        while !s2.is_null() && (*s2).time != (*s).time + (*s).dur {
            s2 = (*s2).ts_next;
        }
        if !s2.is_null() {
            x = (*s2).x;
        } else {
            x = realwidth;
        }
        s2 = s;
        while (*s2).sflags & 0x80000 as libc::c_int as libc::c_uint == 0 {
            s2 = (*s2).ts_prev;
        }
        s2 = (*s2).ts_prev;
        x = ((x + (*s2).x) as libc::c_double * 0.5f64) as libc::c_float;
        if (*s).u.note.dc.n as libc::c_int > 0 as libc::c_int {
            deco_update(s, x - (*s).x);
        }
        (*s).x = x;
    } else {
        x = (*s).x + (*s).u.note.notes[0 as libc::c_int as usize].shhd * cur_scale;
    }
    if (*s).flags as libc::c_int & 0x2 as libc::c_int != 0 {
        return;
    }
    staffb = staff_tb[(*s).staff as usize].y;
    if (*s).sflags & 0x20000 as libc::c_int as libc::c_uint != 0 {
        putxy(x, staffb + 12 as libc::c_int as libc::c_float);
        if ((*s).doty as libc::c_int) < 0 as libc::c_int {
            a2b(b"srep\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        } else {
            a2b(b"mrep\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
            if (*s).doty as libc::c_int > 2 as libc::c_int
                && (*s).voice as libc::c_int == (*cursys).top_voice as libc::c_int
            {
                set_font(cfmt.anf as libc::c_int);
                putxy(
                    x,
                    staffb + 24 as libc::c_int as libc::c_float
                        + 4 as libc::c_int as libc::c_float,
                );
                a2b(
                    b"M(%d)showc\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*s).doty as libc::c_int,
                );
            }
        }
        return;
    }
    y = (*s).y as libc::c_int;
    i = 5 as libc::c_int - (*s).nflags as libc::c_int;
    stafflines = staff_tb[(*s).staff as usize].stafflines;
    l = strlen(stafflines) as libc::c_int;
    if i == 7 as libc::c_int && y == 12 as libc::c_int && l <= 2 as libc::c_int {
        y -= 6 as libc::c_int;
    }
    putxy(x, y as libc::c_float + staffb);
    a2b(
        b"%s \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        if !((*s).u.note.notes[0 as libc::c_int as usize].head).is_null() {
            (*s).u.note.notes[0 as libc::c_int as usize].head
        } else {
            rest_tb[i as usize]
        },
    );
    if i >= 6 as libc::c_int {
        j = y / 6 as libc::c_int;
        match i {
            7 => {
                y += 6 as libc::c_int;
                j += 1;
                j;
            }
            6 => {}
            _ => {
                if j >= l - 1 as libc::c_int
                    || *stafflines.offset((j + 1 as libc::c_int) as isize) as libc::c_int
                        != '|' as i32
                {
                    putxy(x, y as libc::c_float + staffb);
                    a2b(
                        b"hl1 \0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                }
                if i == 9 as libc::c_int {
                    y -= 6 as libc::c_int;
                    j -= 1;
                    j;
                }
            }
        }
        if j >= l || *stafflines.offset(j as isize) as libc::c_int != '|' as i32 {
            putxy(x, y as libc::c_float + staffb);
            a2b(b"hl1 \0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        }
    }
    dotx = 8 as libc::c_int as libc::c_float;
    i = 0 as libc::c_int;
    while i < (*s).dots as libc::c_int {
        a2b(
            b"%.1f 3 dt \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            dotx as libc::c_double,
        );
        dotx = (dotx as libc::c_double + 3.5f64) as libc::c_float;
        i += 1;
        i;
    }
    a2b(b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
}
unsafe extern "C" fn draw_gracenotes(mut s: *mut SYMBOL) {
    let mut yy: libc::c_int = 0;
    let mut x0: libc::c_float = 0.;
    let mut y0: libc::c_float = 0.;
    let mut x1: libc::c_float = 0.;
    let mut y1: libc::c_float = 0.;
    let mut x2: libc::c_float = 0.;
    let mut y2: libc::c_float = 0.;
    let mut x3: libc::c_float = 0.;
    let mut y3: libc::c_float = 0.;
    let mut bet1: libc::c_float = 0.;
    let mut bet2: libc::c_float = 0.;
    let mut dy1: libc::c_float = 0.;
    let mut dy2: libc::c_float = 0.;
    let mut g: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut last: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut bm: BEAM = BEAM {
        s1: 0 as *mut SYMBOL,
        s2: 0 as *mut SYMBOL,
        a: 0.,
        b: 0.,
        nflags: 0,
    };
    bm.s2 = 0 as *mut SYMBOL;
    g = (*s).extra;
    while !g.is_null() {
        if !((*g).type_0 as libc::c_int != 1 as libc::c_int) {
            if (*g).sflags & (0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
                == 0x2 as libc::c_int as libc::c_uint
            {
                if annotate != 0 {
                    anno_out(g, 'b' as i32 as libc::c_char);
                }
                if calculate_beam(&mut bm, g) != 0 {
                    draw_beams(&mut bm);
                }
            }
            draw_note(
                (*g).x,
                g,
                (bm.s2 == 0 as *mut libc::c_void as *mut SYMBOL) as libc::c_int,
            );
            if annotate != 0 {
                anno_out(s, 'g' as i32 as libc::c_char);
            }
            if g == bm.s2 {
                bm.s2 = 0 as *mut SYMBOL;
            }
            if (*g).flags as libc::c_int & 0x80 as libc::c_int != 0 {
                if ((*g).next).is_null() {
                    x1 = 9 as libc::c_int as libc::c_float;
                    y1 = (if (*g).stem as libc::c_int > 0 as libc::c_int {
                        5 as libc::c_int
                    } else {
                        -(5 as libc::c_int)
                    }) as libc::c_float;
                } else {
                    x1 = (((*(*g).next).x - (*g).x) as libc::c_double * 0.5f64
                        + 4 as libc::c_int as libc::c_double) as libc::c_float;
                    y1 = (((*g).ys + (*(*g).next).ys) as libc::c_double * 0.5f64
                        - (*g).y as libc::c_int as libc::c_double) as libc::c_float;
                    if (*g).stem as libc::c_int > 0 as libc::c_int {
                        y1 -= 1 as libc::c_int as libc::c_float;
                    } else {
                        y1 += 1 as libc::c_int as libc::c_float;
                    }
                }
                putxy(x1, y1);
                a2b(
                    b"g%ca\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    if (*g).stem as libc::c_int > 0 as libc::c_int {
                        'u' as i32
                    } else {
                        'd' as i32
                    },
                );
            }
            if annotate != 0
                && (*g).sflags
                    & (0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
                    == 0x10 as libc::c_int as libc::c_uint
            {
                anno_out(g, 'e' as i32 as libc::c_char);
            }
            if ((*g).next).is_null() {
                break;
            }
        }
        g = (*g).next;
    }
    if voice_tb[(*s).voice as usize].key.instr as libc::c_int == 1 as libc::c_int
        || voice_tb[(*s).voice as usize].key.instr as libc::c_int == 2 as libc::c_int
        || pipeformat != 0 || cfmt.graceslurs == 0
        || (*s).u.note.slur_st as libc::c_int != 0 || ((*s).next).is_null()
        || (*(*s).next).abc_type as libc::c_int != 4 as libc::c_int
    {
        return;
    }
    last = g;
    if (*last).stem as libc::c_int >= 0 as libc::c_int {
        yy = 127 as libc::c_int;
        g = (*s).extra;
        while !g.is_null() {
            if !((*g).type_0 as libc::c_int != 1 as libc::c_int) {
                if ((*g).y as libc::c_int) < yy {
                    yy = (*g).y as libc::c_int;
                    last = g;
                }
            }
            g = (*g).next;
        }
        x0 = (*last).x;
        y0 = ((*last).y as libc::c_int - 5 as libc::c_int) as libc::c_float;
        if (*s).extra != last {
            x0 -= 4 as libc::c_int as libc::c_float;
            y0 += 1 as libc::c_int as libc::c_float;
        }
        s = (*s).next;
        x3 = (*s).x - 1 as libc::c_int as libc::c_float;
        if ((*s).stem as libc::c_int) < 0 as libc::c_int {
            x3 -= 4 as libc::c_int as libc::c_float;
        }
        y3 = (3 as libc::c_int
            * ((*s).pits[0 as libc::c_int as usize] as libc::c_int - 18 as libc::c_int)
            - 5 as libc::c_int) as libc::c_float;
        dy1 = ((x3 - x0) as libc::c_double * 0.4f64) as libc::c_float;
        if dy1 > 3 as libc::c_int as libc::c_float {
            dy1 = 3 as libc::c_int as libc::c_float;
        }
        dy2 = dy1;
        bet1 = 0.2f64 as libc::c_float;
        bet2 = 0.8f64 as libc::c_float;
        if y0 > y3 + 7 as libc::c_int as libc::c_float {
            x0 = (*last).x - 1 as libc::c_int as libc::c_float;
            y0 = (y0 as libc::c_double + 0.5f64) as libc::c_float;
            y3 = (y3 as libc::c_double + 6.5f64) as libc::c_float;
            x3 = ((*s).x as libc::c_double - 5.5f64) as libc::c_float;
            dy1 = ((y0 - y3) as libc::c_double * 0.8f64) as libc::c_float;
            dy2 = ((y0 - y3) as libc::c_double * 0.2f64) as libc::c_float;
            bet1 = 0 as libc::c_int as libc::c_float;
        } else if y3 > y0 + 4 as libc::c_int as libc::c_float {
            y3 = y0 + 4 as libc::c_int as libc::c_float;
            x0 = (*last).x + 2 as libc::c_int as libc::c_float;
            y0 = ((*last).y as libc::c_int - 4 as libc::c_int) as libc::c_float;
        }
    } else {
        yy = -(127 as libc::c_int);
        g = (*s).extra;
        while !g.is_null() {
            if !((*g).type_0 as libc::c_int != 1 as libc::c_int) {
                if (*g).y as libc::c_int > yy {
                    yy = (*g).y as libc::c_int;
                    last = g;
                }
            }
            g = (*g).next;
        }
        x0 = (*last).x;
        y0 = ((*last).y as libc::c_int + 5 as libc::c_int) as libc::c_float;
        if (*s).extra != last {
            x0 -= 4 as libc::c_int as libc::c_float;
            y0 -= 1 as libc::c_int as libc::c_float;
        }
        s = (*s).next;
        x3 = (*s).x - 1 as libc::c_int as libc::c_float;
        if (*s).stem as libc::c_int >= 0 as libc::c_int {
            x3 -= 2 as libc::c_int as libc::c_float;
        }
        y3 = (3 as libc::c_int
            * ((*s).pits[(*s).nhd as usize] as libc::c_int - 18 as libc::c_int)
            + 5 as libc::c_int) as libc::c_float;
        dy1 = ((x0 - x3) as libc::c_double * 0.4f64) as libc::c_float;
        if dy1 < -(3 as libc::c_int) as libc::c_float {
            dy1 = -(3 as libc::c_int) as libc::c_float;
        }
        dy2 = dy1;
        bet1 = 0.2f64 as libc::c_float;
        bet2 = 0.8f64 as libc::c_float;
        if y0 < y3 - 7 as libc::c_int as libc::c_float {
            x0 = (*last).x - 1 as libc::c_int as libc::c_float;
            y0 = (y0 as libc::c_double - 0.5f64) as libc::c_float;
            y3 = (y3 as libc::c_double - 6.5f64) as libc::c_float;
            x3 = ((*s).x as libc::c_double - 5.5f64) as libc::c_float;
            dy1 = ((y0 - y3) as libc::c_double * 0.8f64) as libc::c_float;
            dy2 = ((y0 - y3) as libc::c_double * 0.2f64) as libc::c_float;
            bet1 = 0 as libc::c_int as libc::c_float;
        } else if y3 < y0 - 4 as libc::c_int as libc::c_float {
            y3 = y0 - 4 as libc::c_int as libc::c_float;
            x0 = (*last).x + 2 as libc::c_int as libc::c_float;
            y0 = ((*last).y as libc::c_int + 4 as libc::c_int) as libc::c_float;
        }
    }
    x1 = bet1 * x3 + (1 as libc::c_int as libc::c_float - bet1) * x0;
    y1 = bet1 * y3 + (1 as libc::c_int as libc::c_float - bet1) * y0 - dy1;
    x2 = bet2 * x3 + (1 as libc::c_int as libc::c_float - bet2) * x0;
    y2 = bet2 * y3 + (1 as libc::c_int as libc::c_float - bet2) * y0 - dy2;
    a2b(
        b"%.2f %.2f %.2f %.2f %.2f %.2f \0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        (x1 - x0) as libc::c_double,
        (y1 - y0) as libc::c_double,
        (x2 - x0) as libc::c_double,
        (y2 - y0) as libc::c_double,
        (x3 - x0) as libc::c_double,
        (y3 - y0) as libc::c_double,
    );
    putxy(x0, y0 + staff_tb[(*s).staff as usize].y);
    a2b(b"gsl\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
}
unsafe extern "C" fn setdoty(mut s: *mut SYMBOL, mut y_tb: *mut libc::c_schar) {
    let mut m: libc::c_int = 0;
    let mut m1: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut doty: libc::c_int = 0;
    doty = (*s).doty as libc::c_int;
    m = 0 as libc::c_int;
    while m <= (*s).nhd as libc::c_int {
        y = 3 as libc::c_int
            * ((*s).pits[m as usize] as libc::c_int - 18 as libc::c_int);
        if y % 6 as libc::c_int == 0 as libc::c_int {
            if doty != 0 as libc::c_int {
                y -= 3 as libc::c_int;
            } else {
                y += 3 as libc::c_int;
            }
        }
        *y_tb.offset(m as isize) = y as libc::c_schar;
        m += 1;
        m;
    }
    m = 0 as libc::c_int;
    while m < (*s).nhd as libc::c_int {
        if !(*y_tb.offset((m + 1 as libc::c_int) as isize) as libc::c_int
            > *y_tb.offset(m as isize) as libc::c_int)
        {
            m1 = m;
            while m1 > 0 as libc::c_int {
                if *y_tb.offset(m1 as isize) as libc::c_int
                    > *y_tb.offset((m1 - 1 as libc::c_int) as isize) as libc::c_int
                        + 6 as libc::c_int
                {
                    break;
                }
                m1 -= 1;
                m1;
            }
            if (3 as libc::c_int
                * ((*s).pits[m1 as usize] as libc::c_int - 18 as libc::c_int)
                - *y_tb.offset(m1 as isize) as libc::c_int)
                < *y_tb.offset((m + 1 as libc::c_int) as isize) as libc::c_int
                    - 3 as libc::c_int
                        * ((*s).pits[(m + 1 as libc::c_int) as usize] as libc::c_int
                            - 18 as libc::c_int)
            {
                while m1 <= m {
                    let fresh7 = m1;
                    m1 = m1 + 1;
                    let ref mut fresh8 = *y_tb.offset(fresh7 as isize);
                    *fresh8 = (*fresh8 as libc::c_int - 6 as libc::c_int)
                        as libc::c_schar;
                }
            } else {
                *y_tb
                    .offset(
                        (m + 1 as libc::c_int) as isize,
                    ) = (*y_tb.offset(m as isize) as libc::c_int + 6 as libc::c_int)
                    as libc::c_schar;
            }
        }
        m += 1;
        m;
    }
}
unsafe extern "C" fn draw_basic_note(
    mut x: libc::c_float,
    mut s: *mut SYMBOL,
    mut m: libc::c_int,
    mut y_tb: *mut libc::c_schar,
) {
    let mut note: *mut note = &mut *((*s).u.note.notes).as_mut_ptr().offset(m as isize)
        as *mut note;
    let mut y: libc::c_int = 0;
    let mut head: libc::c_int = 0;
    let mut dots: libc::c_int = 0;
    let mut nflags: libc::c_int = 0;
    let mut acc: libc::c_int = 0;
    let mut old_color: libc::c_int = -(1 as libc::c_int);
    let mut staffb: libc::c_float = 0.;
    let mut shhd: libc::c_float = 0.;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut hd: [libc::c_char; 32] = [0; 32];
    staffb = staff_tb[(*s).staff as usize].y;
    y = 3 as libc::c_int * ((*s).pits[m as usize] as libc::c_int - 18 as libc::c_int);
    shhd = (*note).shhd * cur_scale;
    if (*s).flags as libc::c_int & 0x2 as libc::c_int != 0 {
        return;
    }
    putxy(x + shhd, y as libc::c_float + staffb);
    identify_note(s, (*note).len, &mut head, &mut dots, &mut nflags);
    acc = (*note).acc as libc::c_int;
    if y % 6 as libc::c_int == 0 as libc::c_int
        && shhd
            != (if (*s).stem as libc::c_int > 0 as libc::c_int {
                (*s).u.note.notes[0 as libc::c_int as usize].shhd
            } else {
                (*s).u.note.notes[(*s).nhd as usize].shhd
            })
    {
        let mut yy: libc::c_int = 0;
        yy = 0 as libc::c_int;
        if y >= 30 as libc::c_int {
            yy = y;
            if yy % 6 as libc::c_int != 0 {
                yy -= 3 as libc::c_int;
            }
        } else if y <= -(6 as libc::c_int) {
            yy = y;
            if yy % 6 as libc::c_int != 0 {
                yy += 3 as libc::c_int;
            }
        }
        if yy != 0 {
            putxy(x + shhd, yy as libc::c_float + staffb);
            a2b(b"hl \0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        }
    }
    if (*note).invisible != 0 {
        p = b"xydef\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    } else {
        p = (*note).head;
        if !p.is_null() {
            snprintf(
                hd.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
                b"%.*s\0" as *const u8 as *const libc::c_char,
                (*note).hlen as libc::c_int,
                p,
            );
            p = hd.as_mut_ptr();
            a2b(
                b"2 copy xydef \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        } else if (*s).flags as libc::c_int & 0x20 as libc::c_int != 0 {
            p = b"ghd\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else if (*s).type_0 as libc::c_int == 15 as libc::c_int {
            p = b"custos\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else if (*s).sflags & 0x4000 as libc::c_int as libc::c_uint != 0
            && acc != 0 as libc::c_int
        {
            sprintf(
                hd.as_mut_ptr(),
                b"p%shd\0" as *const u8 as *const libc::c_char,
                acc_tb[(acc & 0x7 as libc::c_int) as usize],
            );
            acc = 0 as libc::c_int;
            p = hd.as_mut_ptr();
        } else {
            let mut current_block_45: u64;
            match head {
                2 => {
                    if (*note).len < 1536 as libc::c_int * 2 as libc::c_int {
                        p = b"HD\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                        current_block_45 = 15004371738079956865;
                    } else if (*s).head as libc::c_int != 3 as libc::c_int {
                        p = b"HDD\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                        current_block_45 = 15004371738079956865;
                    } else {
                        current_block_45 = 13852395071023116180;
                    }
                }
                3 => {
                    current_block_45 = 13852395071023116180;
                }
                1 => {
                    p = b"Hd\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                    current_block_45 = 15004371738079956865;
                }
                _ => {
                    p = b"hd\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                    current_block_45 = 15004371738079956865;
                }
            }
            match current_block_45 {
                13852395071023116180 => {
                    p = (if (*note).len
                        < 1536 as libc::c_int * 2 as libc::c_int * 2 as libc::c_int
                    {
                        b"breve\0" as *const u8 as *const libc::c_char
                    } else {
                        b"longa\0" as *const u8 as *const libc::c_char
                    }) as *mut libc::c_char;
                    if tsnext.is_null() && !((*s).next).is_null()
                        && (*(*s).next).type_0 as libc::c_int == 3 as libc::c_int
                        && ((*(*s).next).next).is_null()
                    {
                        dots = 0 as libc::c_int;
                    }
                }
                _ => {}
            }
        }
    }
    if (*note).color >= 0 as libc::c_int {
        old_color = cur_color;
        set_color((*note).color);
    }
    a2b(b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char, p);
    if dots != 0 {
        let mut dotx: libc::c_float = 0.;
        let mut doty: libc::c_int = 0;
        dotx = (7.7f64 + (*s).xmx as libc::c_double - (*note).shhd as libc::c_double)
            as libc::c_float;
        doty = *y_tb.offset(m as isize) as libc::c_int - y;
        if scale_voice != 0 {
            doty = (doty as libc::c_float / cur_scale) as libc::c_int;
        }
        loop {
            dots -= 1;
            if !(dots >= 0 as libc::c_int) {
                break;
            }
            a2b(
                b" %.1f %d dt\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                dotx as libc::c_double,
                doty,
            );
            dotx = (dotx as libc::c_double + 3.5f64) as libc::c_float;
        }
    }
    if acc != 0 as libc::c_int {
        x -= (*note).shac * cur_scale;
        a2b(b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        putx(x);
        a2b(
            (if (*s).flags as libc::c_int & 0x20 as libc::c_int != 0 {
                b"gsc \0" as *const u8 as *const libc::c_char
            } else {
                b"y \0" as *const u8 as *const libc::c_char
            }) as *mut libc::c_char,
        );
        draw_acc(acc, (*s).u.note.microscale as libc::c_int);
        if (*s).flags as libc::c_int & 0x20 as libc::c_int != 0 {
            a2b(b" grestore\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        }
    }
    if old_color >= 0 as libc::c_int {
        a2b(b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        set_color(old_color);
    }
}
unsafe extern "C" fn draw_note(
    mut x: libc::c_float,
    mut s: *mut SYMBOL,
    mut fl: libc::c_int,
) {
    let mut m: libc::c_int = 0;
    let mut ma: libc::c_int = 0;
    let mut staffb: libc::c_float = 0.;
    let mut slen: libc::c_float = 0.;
    let mut shhd: libc::c_float = 0.;
    let mut c: libc::c_char = 0;
    let mut hltype: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut y_tb: [libc::c_schar; 8] = [0; 8];
    if (*s).dots != 0 {
        setdoty(s, y_tb.as_mut_ptr());
    }
    if (*s).head as libc::c_int >= 2 as libc::c_int {
        x += 1 as libc::c_int as libc::c_float;
    }
    staffb = staff_tb[(*s).staff as usize].y;
    if (*s).flags as libc::c_int & 0x2 as libc::c_int == 0 {
        if (*s).flags as libc::c_int & 0x20 as libc::c_int != 0 {
            hltype = b"ghl\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else {
            match (*s).head as libc::c_int {
                2 => {
                    hltype = b"hl1\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                3 => {
                    hltype = b"hl2\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                _ => {
                    hltype = b"hl\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
            }
        }
        shhd = (if (*s).stem as libc::c_int > 0 as libc::c_int {
            (*s).u.note.notes[0 as libc::c_int as usize].shhd
        } else {
            (*s).u.note.notes[(*s).nhd as usize].shhd
        }) * cur_scale;
        if ((*s).pits[0 as libc::c_int as usize] as libc::c_int) < 22 as libc::c_int {
            draw_hl(
                x + shhd,
                staffb,
                0 as libc::c_int,
                3 as libc::c_int
                    * ((*s).pits[0 as libc::c_int as usize] as libc::c_int
                        - 18 as libc::c_int),
                staff_tb[(*s).staff as usize].stafflines,
                hltype,
            );
        }
        if (*s).pits[(*s).nhd as usize] as libc::c_int > 22 as libc::c_int {
            draw_hl(
                x + shhd,
                staffb,
                1 as libc::c_int,
                3 as libc::c_int
                    * ((*s).pits[(*s).nhd as usize] as libc::c_int - 18 as libc::c_int),
                staff_tb[(*s).staff as usize].stafflines,
                hltype,
            );
        }
    }
    if cfmt.setdefl != 0 {
        set_defl(
            if (*s).stem as libc::c_int >= 0 as libc::c_int {
                0x4 as libc::c_int
            } else {
                0 as libc::c_int
            },
        );
    }
    ma = if (*s).stem as libc::c_int >= 0 as libc::c_int {
        0 as libc::c_int
    } else {
        (*s).nhd as libc::c_int
    };
    draw_basic_note(x, s, ma, y_tb.as_mut_ptr());
    if (*s).flags as libc::c_int & (0x2 as libc::c_int | 0x8 as libc::c_int) == 0 {
        let mut c2: libc::c_char = 0;
        c = (if (*s).stem as libc::c_int >= 0 as libc::c_int {
            'u' as i32
        } else {
            'd' as i32
        }) as libc::c_char;
        slen = ((*s).ys - (*s).y as libc::c_int as libc::c_float)
            / voice_tb[(*s).voice as usize].scale;
        if fl == 0
            || (*s).nflags as libc::c_int - (*s).aux as libc::c_int <= 0 as libc::c_int
        {
            c2 = (if (*s).flags as libc::c_int & 0x20 as libc::c_int != 0 {
                'g' as i32
            } else {
                's' as i32
            }) as libc::c_char;
            if (*s).nflags as libc::c_int > 0 as libc::c_int {
                if (*s).stem as libc::c_int >= 0 as libc::c_int {
                    slen -= 1 as libc::c_int as libc::c_float;
                } else {
                    slen += 1 as libc::c_int as libc::c_float;
                }
            }
            a2b(
                b" %.1f %c%c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                slen as libc::c_double,
                c2 as libc::c_int,
                c as libc::c_int,
            );
        } else {
            if cfmt.straightflags != 0 {
                c = 's' as i32 as libc::c_char;
            }
            c2 = (if (*s).flags as libc::c_int & 0x20 as libc::c_int != 0 {
                'g' as i32
            } else {
                'f' as i32
            }) as libc::c_char;
            a2b(
                b" %d %.1f s%c%c\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*s).nflags as libc::c_int - (*s).aux as libc::c_int,
                slen as libc::c_double,
                c2 as libc::c_int,
                c as libc::c_int,
            );
        }
    } else if (*s).sflags & 0x200 as libc::c_int as libc::c_uint != 0 {
        let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
        s2 = (*s).ts_prev;
        slen = (if (*s2).stem as libc::c_int > 0 as libc::c_int {
            (*s2).y as libc::c_int as libc::c_float
        } else {
            (*s2).ys
        }) - (*s).y as libc::c_int as libc::c_float;
        slen += staff_tb[(*s2).staff as usize].y - staffb;
        slen /= voice_tb[(*s).voice as usize].scale;
        a2b(
            b" %.1f su\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            slen as libc::c_double,
        );
    }
    if (*s).flags as libc::c_int & 0x2 as libc::c_int == 0 && fl != 0
        && (*s).sflags & 0x800000 as libc::c_int as libc::c_uint != 0
    {
        let mut x1: libc::c_float = 0.;
        x1 = x
            + (if (*s).stem as libc::c_int > 0 as libc::c_int {
                (*s).u.note.notes[0 as libc::c_int as usize].shhd
            } else {
                (*s).u.note.notes[(*s).nhd as usize].shhd
            }) * cur_scale;
        slen = (3 as libc::c_int
            * ((*s)
                .pits[(if (*s).stem as libc::c_int > 0 as libc::c_int {
                (*s).nhd as libc::c_int
            } else {
                0 as libc::c_int
            }) as usize] as libc::c_int - 18 as libc::c_int)) as libc::c_float;
        if (*s).head as libc::c_int >= 2 as libc::c_int {
            if (*s).stem as libc::c_int > 0 as libc::c_int {
                slen = (slen as libc::c_double
                    + (5 as libc::c_int as libc::c_double
                        + 5.4f64 * (*s).aux as libc::c_int as libc::c_double))
                    as libc::c_float;
            } else {
                slen = (slen as libc::c_double
                    - (5 as libc::c_int as libc::c_double + 5.4f64)) as libc::c_float;
            }
        } else {
            x1 = (x1 as libc::c_double
                + (if (*s).flags as libc::c_int & 0x20 as libc::c_int != 0 {
                    2.3f64
                } else {
                    3.5f64
                }) * (*s).stem as libc::c_int as libc::c_double) as libc::c_float;
            if (*s).stem as libc::c_int > 0 as libc::c_int {
                slen = (slen as libc::c_double
                    + (6 as libc::c_int as libc::c_double
                        + 5.4f64 * (*s).aux as libc::c_int as libc::c_double))
                    as libc::c_float;
            } else {
                slen = (slen as libc::c_double
                    - (6 as libc::c_int as libc::c_double + 5.4f64)) as libc::c_float;
            }
        }
        slen /= voice_tb[(*s).voice as usize].scale;
        a2b(
            b" %d \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*s).aux as libc::c_int,
        );
        putxy(x1, staffb + slen);
        a2b(b"trem\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    }
    m = 0 as libc::c_int;
    while m <= (*s).nhd as libc::c_int {
        if !(m == ma) {
            a2b(b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char);
            draw_basic_note(x, s, m, y_tb.as_mut_ptr());
        }
        m += 1;
        m;
    }
    a2b(b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
}
unsafe extern "C" fn next_scut(mut s: *mut SYMBOL) -> *mut SYMBOL {
    let mut prev: *mut SYMBOL = 0 as *mut SYMBOL;
    prev = s;
    s = (*s).next;
    while !s.is_null() {
        if (*s).type_0 as libc::c_int == 3 as libc::c_int
            && ((*s).sflags & 0x100 as libc::c_int as libc::c_uint != 0
                || (*s).u.bar.type_0 == 0x13 as libc::c_int
                || (*s).u.bar.type_0 == 0x21 as libc::c_int
                || (*s).u.bar.repeat_bar as libc::c_int != 0 && !((*s).text).is_null()
                    && *((*s).text).offset(0 as libc::c_int as isize) as libc::c_int
                        != '1' as i32)
        {
            return s;
        }
        prev = s;
        s = (*s).next;
    }
    return prev;
}
#[no_mangle]
pub unsafe extern "C" fn prev_scut(mut s: *mut SYMBOL) -> *mut SYMBOL {
    while !((*s).prev).is_null() {
        s = (*s).prev;
        if (*s).type_0 as libc::c_int == 3 as libc::c_int
            && ((*s).sflags & 0x100 as libc::c_int as libc::c_uint != 0
                || (*s).u.bar.type_0 == 0x13 as libc::c_int
                || (*s).u.bar.type_0 == 0x21 as libc::c_int
                || (*s).u.bar.repeat_bar as libc::c_int != 0 && !((*s).text).is_null()
                    && *((*s).text).offset(0 as libc::c_int as isize) as libc::c_int
                        != '1' as i32)
        {
            return s;
        }
    }
    s = voice_tb[(*s).voice as usize].sym;
    while (*s).type_0 as libc::c_int != 4 as libc::c_int {
        s = (*s).ts_prev;
    }
    if !((*s).next).is_null() && (*(*s).next).type_0 as libc::c_int == 6 as libc::c_int {
        s = (*s).next;
    }
    if !((*s).next).is_null() && (*(*s).next).type_0 as libc::c_int == 5 as libc::c_int {
        s = (*s).next;
    }
    return s;
}
unsafe extern "C" fn slur_direction(
    mut k1: *mut SYMBOL,
    mut k2: *mut SYMBOL,
) -> libc::c_int {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut some_upstem: libc::c_int = 0;
    let mut low: libc::c_int = 0;
    if (*k1).flags as libc::c_int & 0x20 as libc::c_int != 0
        && (*k1).stem as libc::c_int > 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    low = 0 as libc::c_int;
    some_upstem = low;
    s = k1;
    loop {
        if (*s).abc_type as libc::c_int == 4 as libc::c_int {
            if (*s).flags as libc::c_int & 0x8 as libc::c_int == 0 {
                if ((*s).stem as libc::c_int) < 0 as libc::c_int {
                    return 1 as libc::c_int;
                }
                some_upstem = 1 as libc::c_int;
            }
            if ((*s).pits[0 as libc::c_int as usize] as libc::c_int) < 22 as libc::c_int
            {
                low = 1 as libc::c_int;
            }
        }
        if s == k2 {
            break;
        }
        s = (*s).next;
    }
    if some_upstem == 0 && low == 0 {
        return 1 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn slur_out(
    mut x1: libc::c_float,
    mut y1: libc::c_float,
    mut x2: libc::c_float,
    mut y2: libc::c_float,
    mut s: libc::c_int,
    mut height: libc::c_float,
    mut dotted: libc::c_int,
    mut staff: libc::c_int,
) {
    let mut alfa: libc::c_float = 0.;
    let mut beta: libc::c_float = 0.;
    let mut mx: libc::c_float = 0.;
    let mut my: libc::c_float = 0.;
    let mut xx1: libc::c_float = 0.;
    let mut yy1: libc::c_float = 0.;
    let mut xx2: libc::c_float = 0.;
    let mut yy2: libc::c_float = 0.;
    let mut dx: libc::c_float = 0.;
    let mut dy: libc::c_float = 0.;
    let mut dz: libc::c_float = 0.;
    let mut scale_y: libc::c_float = 0.;
    alfa = 0.3f64 as libc::c_float;
    beta = 0.45f64 as libc::c_float;
    dy = y2 - y1;
    if dy < 0 as libc::c_int as libc::c_float {
        dy = -dy;
    }
    dx = x2 - x1;
    if dx as libc::c_double > 40.0f64 && ((dy / dx) as libc::c_double) < 0.7f64 {
        alfa = (0.3f64 + 0.002f64 * (dx as libc::c_double - 40.0f64)) as libc::c_float;
        if alfa as libc::c_double > 0.7f64 {
            alfa = 0.7f64 as libc::c_float;
        }
    }
    mx = (0.5f64 * (x1 + x2) as libc::c_double) as libc::c_float;
    my = (0.5f64 * (y1 + y2) as libc::c_double) as libc::c_float;
    xx1 = mx + alfa * (x1 - mx);
    yy1 = my + alfa * (y1 - my) + height;
    xx1 = x1 + beta * (xx1 - x1);
    yy1 = y1 + beta * (yy1 - y1);
    xx2 = mx + alfa * (x2 - mx);
    yy2 = my + alfa * (y2 - my) + height;
    xx2 = x2 + beta * (xx2 - x2);
    yy2 = y2 + beta * (yy2 - y2);
    dx = (0.03f64 * (x2 - x1) as libc::c_double) as libc::c_float;
    dy = (2 as libc::c_int * s) as libc::c_float;
    dz = (0.2f64 + 0.001f64 * (x2 - x1) as libc::c_double) as libc::c_float;
    if dz as libc::c_double > 0.6f64 {
        dz = 0.6f64 as libc::c_float;
    }
    dz *= s as libc::c_float;
    scale_y = if scale_voice != 0 {
        cur_scale
    } else {
        1 as libc::c_int as libc::c_float
    };
    if dotted == 0 {
        a2b(
            b"%.2f %.2f %.2f %.2f %.2f %.2f 0 %.2f \0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            ((xx2 - dx - x2) / cur_scale) as libc::c_double,
            ((yy2 + dy - y2 - dz) / scale_y) as libc::c_double,
            ((xx1 + dx - x2) / cur_scale) as libc::c_double,
            ((yy1 + dy - y2 - dz) / scale_y) as libc::c_double,
            ((x1 - x2) / cur_scale) as libc::c_double,
            ((y1 - y2 - dz) / scale_y) as libc::c_double,
            dz as libc::c_double,
        );
    }
    a2b(
        b"%.2f %.2f %.2f %.2f %.2f %.2f \0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        ((xx1 - x1) / cur_scale) as libc::c_double,
        ((yy1 - y1) / scale_y) as libc::c_double,
        ((xx2 - x1) / cur_scale) as libc::c_double,
        ((yy2 - y1) / scale_y) as libc::c_double,
        ((x2 - x1) / cur_scale) as libc::c_double,
        ((y2 - y1) / scale_y) as libc::c_double,
    );
    putxy(x1, y1);
    if staff >= 0 as libc::c_int {
        a2b(b"y%d \0" as *const u8 as *const libc::c_char as *mut libc::c_char, staff);
    }
    a2b(
        (if dotted != 0 {
            b"dSL\n\0" as *const u8 as *const libc::c_char
        } else {
            b"SL\n\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char,
    );
}
unsafe extern "C" fn slur_multi(
    mut k1: *mut SYMBOL,
    mut k2: *mut SYMBOL,
) -> libc::c_int {
    loop {
        if (*k1).multi as libc::c_int != 0 as libc::c_int {
            return (*k1).multi as libc::c_int;
        }
        if k1 == k2 {
            break;
        }
        k1 = (*k1).next;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn draw_slur(
    mut k1_orig: *mut SYMBOL,
    mut k2: *mut SYMBOL,
    mut m1: libc::c_int,
    mut m2: libc::c_int,
    mut slur_type: libc::c_int,
) -> libc::c_int {
    let mut k1: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut k: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut x1: libc::c_float = 0.;
    let mut y1: libc::c_float = 0.;
    let mut x2: libc::c_float = 0.;
    let mut y2: libc::c_float = 0.;
    let mut height: libc::c_float = 0.;
    let mut addy: libc::c_float = 0.;
    let mut a: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    let mut z: libc::c_float = 0.;
    let mut h: libc::c_float = 0.;
    let mut dx: libc::c_float = 0.;
    let mut dy: libc::c_float = 0.;
    let mut s: libc::c_int = 0;
    let mut nn: libc::c_int = 0;
    let mut upstaff: libc::c_int = 0;
    let mut two_staves: libc::c_int = 0;
    k1 = k1_orig;
    while (*k1).voice as libc::c_int != (*k2).voice as libc::c_int {
        k1 = (*k1).ts_next;
    }
    match slur_type & 0x7 as libc::c_int {
        1 => {
            s = 1 as libc::c_int;
        }
        2 => {
            s = -(1 as libc::c_int);
        }
        _ => {
            s = slur_multi(k1, k2);
            if s == 0 as libc::c_int {
                s = slur_direction(k1, k2);
            }
        }
    }
    nn = 1 as libc::c_int;
    upstaff = (*k1).staff as libc::c_int;
    two_staves = 0 as libc::c_int;
    if k1 != k2 {
        k = (*k1).next;
        while !k.is_null() {
            if (*k).type_0 as libc::c_int == 1 as libc::c_int {
                nn += 1;
                nn;
                if (*k).staff as libc::c_int != upstaff {
                    two_staves = 1 as libc::c_int;
                    if ((*k).staff as libc::c_int) < upstaff {
                        upstaff = (*k).staff as libc::c_int;
                    }
                }
            }
            if k == k2 {
                break;
            }
            k = (*k).next;
        }
    }
    if two_staves != 0 {
        error(
            0 as libc::c_int,
            k1,
            b"*** multi-staves slurs not treated yet\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    x1 = (*k1_orig).x + (*k1_orig).u.note.notes[0 as libc::c_int as usize].shhd;
    if k1_orig != k2 {
        x2 = (*k2).x + (*k2).u.note.notes[0 as libc::c_int as usize].shhd;
    } else {
        k = (*k2).ts_next;
        while !k.is_null() {
            if (*k).sflags & 0x8000000 as libc::c_int as libc::c_uint != 0 {
                break;
            }
            k = (*k).ts_next;
        }
        if k.is_null() {
            x2 = realwidth;
        } else {
            x2 = (*k).x;
        }
    }
    if m1 >= 0 as libc::c_int {
        y1 = (3 as libc::c_int
            * ((*k1).pits[m1 as usize] as libc::c_int - 18 as libc::c_int)
            + 5 as libc::c_int * s) as libc::c_float;
    } else {
        y1 = (if s > 0 as libc::c_int {
            (*k1).ymx as libc::c_int + 2 as libc::c_int
        } else {
            (*k1).ymn as libc::c_int - 2 as libc::c_int
        }) as libc::c_float;
        if (*k1).abc_type as libc::c_int == 4 as libc::c_int {
            if s > 0 as libc::c_int {
                if (*k1).stem as libc::c_int > 0 as libc::c_int {
                    x1 += 5 as libc::c_int as libc::c_float;
                    if (*k1).sflags & 0x10 as libc::c_int as libc::c_uint != 0
                        && (*k1).nflags as libc::c_int >= -(1 as libc::c_int)
                        && (*k1).sflags & 0x40 as libc::c_int as libc::c_uint == 0
                    {
                        if (*k1).nflags as libc::c_int > 0 as libc::c_int {
                            x1 += 2 as libc::c_int as libc::c_float;
                            y1 = (*k1).ys - 3 as libc::c_int as libc::c_float;
                        } else {
                            y1 = (*k1).ys - 6 as libc::c_int as libc::c_float;
                        }
                    } else {
                        y1 = (*k1).ys + 3 as libc::c_int as libc::c_float;
                    }
                } else {
                    y1 = ((*k1).y as libc::c_int + 8 as libc::c_int) as libc::c_float;
                }
            } else if ((*k1).stem as libc::c_int) < 0 as libc::c_int {
                x1 -= 1 as libc::c_int as libc::c_float;
                if (*k1).sflags & 0x10 as libc::c_int as libc::c_uint != 0
                    && (*k1).nflags as libc::c_int >= -(1 as libc::c_int)
                    && ((*k1).sflags & 0x40 as libc::c_int as libc::c_uint == 0
                        || (*k1).ys < y1 + 3 as libc::c_int as libc::c_float)
                {
                    if (*k1).nflags as libc::c_int > 0 as libc::c_int {
                        x1 += 2 as libc::c_int as libc::c_float;
                        y1 = (*k1).ys + 3 as libc::c_int as libc::c_float;
                    } else {
                        y1 = (*k1).ys + 6 as libc::c_int as libc::c_float;
                    }
                } else {
                    y1 = (*k1).ys - 3 as libc::c_int as libc::c_float;
                }
            } else {
                y1 = ((*k1).y as libc::c_int - 8 as libc::c_int) as libc::c_float;
            }
        }
    }
    if m2 >= 0 as libc::c_int {
        y2 = (3 as libc::c_int
            * ((*k2).pits[m2 as usize] as libc::c_int - 18 as libc::c_int)
            + 5 as libc::c_int * s) as libc::c_float;
    } else {
        y2 = (if s > 0 as libc::c_int {
            (*k2).ymx as libc::c_int + 2 as libc::c_int
        } else {
            (*k2).ymn as libc::c_int - 2 as libc::c_int
        }) as libc::c_float;
        if (*k2).abc_type as libc::c_int == 4 as libc::c_int {
            if s > 0 as libc::c_int {
                if (*k2).stem as libc::c_int > 0 as libc::c_int {
                    x2 += 1 as libc::c_int as libc::c_float;
                    if (*k2).sflags & 0x2 as libc::c_int as libc::c_uint != 0
                        && (*k2).nflags as libc::c_int >= -(1 as libc::c_int)
                        && (*k2).sflags & 0x40 as libc::c_int as libc::c_uint == 0
                    {
                        y2 = (*k2).ys - 6 as libc::c_int as libc::c_float;
                    } else {
                        y2 = (*k2).ys + 3 as libc::c_int as libc::c_float;
                    }
                } else {
                    y2 = ((*k2).y as libc::c_int + 8 as libc::c_int) as libc::c_float;
                }
            } else if ((*k2).stem as libc::c_int) < 0 as libc::c_int {
                x2 -= 5 as libc::c_int as libc::c_float;
                if (*k2).sflags & 0x2 as libc::c_int as libc::c_uint != 0
                    && (*k2).nflags as libc::c_int >= -(1 as libc::c_int)
                    && (*k2).sflags & 0x40 as libc::c_int as libc::c_uint == 0
                {
                    y2 = (*k2).ys + 6 as libc::c_int as libc::c_float;
                } else {
                    y2 = (*k2).ys - 3 as libc::c_int as libc::c_float;
                }
            } else {
                y2 = ((*k2).y as libc::c_int - 8 as libc::c_int) as libc::c_float;
            }
        }
    }
    if (*k1).abc_type as libc::c_int != 4 as libc::c_int {
        y1 = (y2 as libc::c_double + 1.2f64 * s as libc::c_double) as libc::c_float;
        x1 = ((*k1).x as libc::c_double + (*k1).wr as libc::c_double * 0.5f64)
            as libc::c_float;
        if x1 > x2 - 12 as libc::c_int as libc::c_float {
            x1 = x2 - 12 as libc::c_int as libc::c_float;
        }
    }
    if (*k2).abc_type as libc::c_int != 4 as libc::c_int {
        if (*k1).abc_type as libc::c_int == 4 as libc::c_int {
            y2 = (y1 as libc::c_double + 1.2f64 * s as libc::c_double) as libc::c_float;
        } else {
            y2 = y1;
        }
        if k1 != k2 {
            x2 = ((*k2).x as libc::c_double - (*k2).wl as libc::c_double * 0.3f64)
                as libc::c_float;
        }
    }
    if nn >= 3 as libc::c_int {
        if (*(*k1).next).type_0 as libc::c_int != 3 as libc::c_int
            && (*(*k1).next).x < x1 + 48 as libc::c_int as libc::c_float
        {
            if s > 0 as libc::c_int {
                y = ((*(*k1).next).ymx as libc::c_int - 2 as libc::c_int)
                    as libc::c_float;
                if y1 < y {
                    y1 = y;
                }
            } else {
                y = ((*(*k1).next).ymn as libc::c_int + 2 as libc::c_int)
                    as libc::c_float;
                if y1 > y {
                    y1 = y;
                }
            }
        }
        if !((*k2).prev).is_null()
            && (*(*k2).prev).type_0 as libc::c_int != 3 as libc::c_int
            && (*(*k2).prev).x > x2 - 48 as libc::c_int as libc::c_float
        {
            if s > 0 as libc::c_int {
                y = ((*(*k2).prev).ymx as libc::c_int - 2 as libc::c_int)
                    as libc::c_float;
                if y2 < y {
                    y2 = y;
                }
            } else {
                y = ((*(*k2).prev).ymn as libc::c_int + 2 as libc::c_int)
                    as libc::c_float;
                if y2 > y {
                    y2 = y;
                }
            }
        }
    }
    a = (y2 - y1) / (x2 - x1);
    if a as libc::c_double > 1.0f64 || (a as libc::c_double) < -1.0f64 {
        if a as libc::c_double > 1.0f64 {
            a = 1.0f64 as libc::c_float;
        } else {
            a = -1.0f64 as libc::c_float;
        }
        if a * s as libc::c_float > 0 as libc::c_int as libc::c_float {
            y1 = y2 - a * (x2 - x1);
        } else {
            y2 = y1 + a * (x2 - x1);
        }
    }
    y = y2 - y1;
    if y > 8 as libc::c_int as libc::c_float {
        y = 8 as libc::c_int as libc::c_float;
    } else if y < -(8 as libc::c_int) as libc::c_float {
        y = -(8 as libc::c_int) as libc::c_float;
    }
    z = y;
    if z < 0 as libc::c_int as libc::c_float {
        z = -z;
    }
    dx = (0.5f64 * z as libc::c_double) as libc::c_float;
    dy = (0.3f64 * y as libc::c_double) as libc::c_float;
    if y * s as libc::c_float > 0 as libc::c_int as libc::c_float {
        x2 -= dx;
        y2 -= dy;
    } else {
        x1 += dx;
        y1 += dy;
    }
    if (*k1).flags as libc::c_int & 0x20 as libc::c_int != 0 {
        x1 = ((*k1).x as libc::c_double - 2.3f64 * 0.5f64) as libc::c_float;
    }
    if (*k2).flags as libc::c_int & 0x20 as libc::c_int != 0 {
        x2 = ((*k2).x as libc::c_double + 2.3f64 * 1.5f64) as libc::c_float;
    }
    h = 0 as libc::c_int as libc::c_float;
    a = (y2 - y1) / (x2 - x1);
    if k1 != k2 && (*k1).voice as libc::c_int == (*k2).voice as libc::c_int {
        addy = y1 - a * x1;
        k = (*k1).next;
        while k != k2 {
            if !((*k).staff as libc::c_int != upstaff) {
                match (*k).type_0 as libc::c_int {
                    1 => {
                        if s > 0 as libc::c_int {
                            y = (3 as libc::c_int
                                * ((*k).pits[(*k).nhd as usize] as libc::c_int
                                    - 18 as libc::c_int) + 6 as libc::c_int) as libc::c_float;
                            if y < (*k).ymx as libc::c_int as libc::c_float {
                                y = (*k).ymx as libc::c_float;
                            }
                            y -= a * (*k).x + addy;
                            if y > h {
                                h = y;
                            }
                        } else {
                            y = (3 as libc::c_int
                                * ((*k).pits[0 as libc::c_int as usize] as libc::c_int
                                    - 18 as libc::c_int) - 6 as libc::c_int) as libc::c_float;
                            if y > (*k).ymn as libc::c_int as libc::c_float {
                                y = (*k).ymn as libc::c_float;
                            }
                            y -= a * (*k).x + addy;
                            if y < h {
                                h = y;
                            }
                        }
                    }
                    11 => {
                        let mut g: *mut SYMBOL = 0 as *mut SYMBOL;
                        g = (*k).extra;
                        while !g.is_null() {
                            if !((*g).type_0 as libc::c_int != 1 as libc::c_int) {
                                if s > 0 as libc::c_int {
                                    y = (3 as libc::c_int
                                        * ((*g).pits[(*g).nhd as usize] as libc::c_int
                                            - 18 as libc::c_int) + 6 as libc::c_int) as libc::c_float;
                                    if y < (*g).ymx as libc::c_int as libc::c_float {
                                        y = (*g).ymx as libc::c_float;
                                    }
                                    y -= a * (*g).x + addy;
                                    if y > h {
                                        h = y;
                                    }
                                } else {
                                    y = (3 as libc::c_int
                                        * ((*g).pits[0 as libc::c_int as usize] as libc::c_int
                                            - 18 as libc::c_int) - 6 as libc::c_int) as libc::c_float;
                                    if y > (*g).ymn as libc::c_int as libc::c_float {
                                        y = (*g).ymn as libc::c_float;
                                    }
                                    y -= a * (*g).x + addy;
                                    if y < h {
                                        h = y;
                                    }
                                }
                            }
                            g = (*g).next;
                        }
                    }
                    _ => {}
                }
            }
            k = (*k).next;
        }
        y1 = (y1 as libc::c_double + 0.45f64 * h as libc::c_double) as libc::c_float;
        y2 = (y2 as libc::c_double + 0.45f64 * h as libc::c_double) as libc::c_float;
        h = (h as libc::c_double * 0.65f64) as libc::c_float;
    }
    if nn > 3 as libc::c_int {
        height = ((0.08f64 * (x2 - x1) as libc::c_double
            + 12 as libc::c_int as libc::c_double) * s as libc::c_double)
            as libc::c_float;
    } else {
        height = ((0.03f64 * (x2 - x1) as libc::c_double
            + 8 as libc::c_int as libc::c_double) * s as libc::c_double)
            as libc::c_float;
    }
    if s > 0 as libc::c_int {
        if height < 3 as libc::c_int as libc::c_float * h {
            height = 3 as libc::c_int as libc::c_float * h;
        }
        if height > 40 as libc::c_int as libc::c_float {
            height = 40 as libc::c_int as libc::c_float;
        }
    } else {
        if height > 3 as libc::c_int as libc::c_float * h {
            height = 3 as libc::c_int as libc::c_float * h;
        }
        if height < -(40 as libc::c_int) as libc::c_float {
            height = -(40 as libc::c_int) as libc::c_float;
        }
    }
    y = y2 - y1;
    if y < 0 as libc::c_int as libc::c_float {
        y = -y;
    }
    if s > 0 as libc::c_int {
        if (height as libc::c_double) < 0.8f64 * y as libc::c_double {
            height = (0.8f64 * y as libc::c_double) as libc::c_float;
        }
    } else if height as libc::c_double > -0.8f64 * y as libc::c_double {
        height = (-0.8f64 * y as libc::c_double) as libc::c_float;
    }
    height *= cfmt.slurheight;
    slur_out(x1, y1, x2, y2, s, height, slur_type & 0x8 as libc::c_int, upstaff);
    dx = x2 - x1;
    a = (y2 - y1) / dx;
    addy = ((y1 - a * x1) as libc::c_double + 0.4f64 * height as libc::c_double)
        as libc::c_float;
    if (*k1).voice as libc::c_int == (*k2).voice as libc::c_int {
        k = k1;
        while k != k2 {
            if !((*k).staff as libc::c_int != upstaff) {
                y = a * (*k).x + addy;
                if ((*k).ymx as libc::c_int as libc::c_float) < y {
                    (*k).ymx = y as libc::c_schar;
                } else if (*k).ymn as libc::c_int as libc::c_float > y {
                    (*k).ymn = y as libc::c_schar;
                }
                if (*k).next == k2 {
                    dx = x2;
                    if (*k2).sflags & 0x800 as libc::c_int as libc::c_uint != 0 {
                        dx -= 5 as libc::c_int as libc::c_float;
                    }
                } else {
                    dx = (*(*k).next).x;
                }
                if k != k1 {
                    x1 = (*k).x;
                }
                dx -= x1;
                y_set(upstaff, (s > 0 as libc::c_int) as libc::c_int, x1, dx, y);
            }
            k = (*k).next;
        }
    }
    return (if s > 0 as libc::c_int { 0x1 as libc::c_int } else { 0x2 as libc::c_int })
        | slur_type & 0x8 as libc::c_int;
}
unsafe extern "C" fn draw_slurs(mut first: *mut SYMBOL, mut last: *mut SYMBOL) {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s1: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut k: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut gr1: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut gr2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut i: libc::c_int = 0;
    let mut m1: libc::c_int = 0;
    let mut m2: libc::c_int = 0;
    let mut gr1_out: libc::c_int = 0;
    let mut slur_type: libc::c_int = 0;
    let mut cont: libc::c_int = 0;
    gr2 = 0 as *mut SYMBOL;
    gr1 = gr2;
    s = first;
    loop {
        if s.is_null() || s == last {
            if gr1.is_null()
                || {
                    s = (*gr1).next;
                    s.is_null()
                } || s == last
            {
                break;
            }
            gr1 = 0 as *mut SYMBOL;
        }
        if (*s).type_0 as libc::c_int == 11 as libc::c_int {
            gr1 = s;
            s = (*s).extra;
        } else if (*s).type_0 as libc::c_int != 1 as libc::c_int
            && (*s).type_0 as libc::c_int != 2 as libc::c_int
            || (*s).u.note.slur_st as libc::c_int == 0 as libc::c_int
                && (*s).sflags & 0x800 as libc::c_int as libc::c_uint == 0
        {
            s = (*s).next;
        } else {
            k = 0 as *mut SYMBOL;
            s1 = (*s).next;
            gr1_out = 0 as libc::c_int;
            loop {
                if s1.is_null() {
                    if !gr2.is_null() {
                        s1 = (*gr2).next;
                        gr2 = 0 as *mut SYMBOL;
                    } else {
                        if gr1.is_null() || gr1_out != 0 {
                            break;
                        }
                        s1 = (*gr1).next;
                        gr1_out = 1 as libc::c_int;
                    }
                } else if (*s1).type_0 as libc::c_int == 11 as libc::c_int {
                    gr2 = s1;
                    s1 = (*s1).extra;
                } else if (*s1).type_0 as libc::c_int == 3 as libc::c_int
                    && ((*s1).sflags & 0x100 as libc::c_int as libc::c_uint != 0
                        || (*s1).u.bar.type_0 == 0x13 as libc::c_int
                        || (*s1).u.bar.type_0 == 0x21 as libc::c_int
                        || (*s1).u.bar.repeat_bar as libc::c_int != 0
                            && !((*s1).text).is_null()
                            && *((*s1).text).offset(0 as libc::c_int as isize)
                                as libc::c_int != '1' as i32)
                {
                    k = s1;
                    break;
                } else if (*s1).type_0 as libc::c_int != 1 as libc::c_int
                    && (*s1).type_0 as libc::c_int != 2 as libc::c_int
                {
                    s1 = (*s1).next;
                } else if (*s1).u.note.slur_end as libc::c_int != 0
                    || (*s1).sflags & 0x1000 as libc::c_int as libc::c_uint != 0
                {
                    k = s1;
                    break;
                } else {
                    if (*s1).u.note.slur_st as libc::c_int != 0
                        || (*s1).sflags & 0x800 as libc::c_int as libc::c_uint != 0
                    {
                        if !gr2.is_null() {
                            k = s1;
                            while !((*k).next).is_null() {
                                k = (*k).next;
                            }
                            (*k).next = (*gr2).next;
                            if !((*gr2).next).is_null() {
                                (*(*gr2).next).prev = k;
                            }
                            k = 0 as *mut SYMBOL;
                        }
                        draw_slurs(s1, last);
                        if !gr2.is_null() && !((*gr2).next).is_null() {
                            (*(*(*gr2).next).prev).next = 0 as *mut SYMBOL;
                            (*(*gr2).next).prev = gr2;
                        }
                    }
                    if s1 == last {
                        break;
                    }
                    s1 = (*s1).next;
                }
            }
            if s1.is_null() {
                k = next_scut(s);
            } else if k.is_null() {
                s = s1;
                if s == last {
                    break;
                } else {
                    continue;
                }
            }
            if !gr1.is_null() {
                s1 = s;
                while !((*s1).next).is_null() {
                    s1 = (*s1).next;
                }
                (*s1).next = (*gr1).next;
                if !((*gr1).next).is_null() {
                    (*(*gr1).next).prev = s1;
                }
                (*gr1).u.note.slur_st = 0x4 as libc::c_int as libc::c_uchar;
            }
            if !gr2.is_null() {
                (*(*gr2).prev).next = (*gr2).extra;
                (*(*gr2).extra).prev = (*gr2).prev;
                (*gr2).u.note.slur_st = 0x4 as libc::c_int as libc::c_uchar;
            }
            if (*s).u.note.slur_st != 0 {
                slur_type = (*s).u.note.slur_st as libc::c_int & 0xf as libc::c_int;
                (*s)
                    .u
                    .note
                    .slur_st = ((*s).u.note.slur_st as libc::c_int >> 4 as libc::c_int)
                    as libc::c_uchar;
                m1 = -(1 as libc::c_int);
            } else {
                m1 = 0 as libc::c_int;
                while m1 <= (*s).nhd as libc::c_int {
                    if (*s).u.note.notes[m1 as usize].sl1 != 0 {
                        break;
                    }
                    m1 += 1;
                    m1;
                }
                slur_type = (*s).u.note.notes[m1 as usize].sl1 as libc::c_int
                    & 0xf as libc::c_int;
                (*s)
                    .u
                    .note
                    .notes[m1 as usize]
                    .sl1 = ((*s).u.note.notes[m1 as usize].sl1 as libc::c_int
                    >> 4 as libc::c_int) as libc::c_uchar;
                if (*s).u.note.notes[m1 as usize].sl1 as libc::c_int == 0 as libc::c_int
                {
                    i = m1 + 1 as libc::c_int;
                    while i <= (*s).nhd as libc::c_int {
                        if (*s).u.note.notes[i as usize].sl1 != 0 {
                            break;
                        }
                        i += 1;
                        i;
                    }
                    if i > (*s).nhd as libc::c_int {
                        (*s).sflags &= !(0x800 as libc::c_int) as libc::c_uint;
                    }
                }
            }
            m2 = -(1 as libc::c_int);
            cont = 0 as libc::c_int;
            if ((*k).type_0 as libc::c_int == 1 as libc::c_int
                || (*k).type_0 as libc::c_int == 2 as libc::c_int)
                && ((*k).u.note.slur_end as libc::c_int != 0
                    || (*k).sflags & 0x1000 as libc::c_int as libc::c_uint != 0)
            {
                if (*k).u.note.slur_end != 0 {
                    (*k).u.note.slur_end -= 1;
                    (*k).u.note.slur_end;
                } else {
                    m2 = 0 as libc::c_int;
                    while m2 <= (*k).nhd as libc::c_int {
                        if (*k).u.note.notes[m2 as usize].sl2 != 0 {
                            break;
                        }
                        m2 += 1;
                        m2;
                    }
                    (*k).u.note.notes[m2 as usize].sl2 -= 1;
                    (*k).u.note.notes[m2 as usize].sl2;
                    if (*k).u.note.notes[m2 as usize].sl2 as libc::c_int
                        == 0 as libc::c_int
                    {
                        i = m2 + 1 as libc::c_int;
                        while i <= (*k).nhd as libc::c_int {
                            if (*k).u.note.notes[i as usize].sl2 != 0 {
                                break;
                            }
                            i += 1;
                            i;
                        }
                        if i > (*k).nhd as libc::c_int {
                            (*k).sflags &= !(0x1000 as libc::c_int) as libc::c_uint;
                        }
                    }
                }
            } else if (*k).type_0 as libc::c_int != 3 as libc::c_int
                || (*k).sflags & 0x100 as libc::c_int as libc::c_uint == 0
                    && (*k).u.bar.type_0 != 0x13 as libc::c_int
                    && (*k).u.bar.type_0 != 0x21 as libc::c_int
                    && ((*k).u.bar.repeat_bar == 0 || ((*k).text).is_null()
                        || *((*k).text).offset(0 as libc::c_int as isize) as libc::c_int
                            == '1' as i32)
            {
                cont = 1 as libc::c_int;
            }
            slur_type = draw_slur(s, k, m1, m2, slur_type);
            if cont != 0 {
                voice_tb[(*k).voice as usize]
                    .slur_st = ((voice_tb[(*k).voice as usize].slur_st as libc::c_int)
                    << 4 as libc::c_int) as libc::c_uchar;
                voice_tb[(*k).voice as usize]
                    .slur_st = (voice_tb[(*k).voice as usize].slur_st as libc::c_int
                    + slur_type) as libc::c_uchar;
            }
            if !gr1.is_null() && !((*gr1).next).is_null() {
                (*(*(*gr1).next).prev).next = 0 as *mut SYMBOL;
                (*(*gr1).next).prev = gr1;
            }
            if !gr2.is_null() {
                (*(*gr2).prev).next = gr2;
                (*(*gr2).extra).prev = 0 as *mut SYMBOL;
                gr2 = 0 as *mut SYMBOL;
            }
            if (*s).u.note.slur_st as libc::c_int != 0
                || (*s).sflags & 0x800 as libc::c_int as libc::c_uint != 0
            {
                continue;
            }
            if s == last {
                break;
            }
            s = (*s).next;
        }
    };
}
unsafe extern "C" fn draw_tuplet(mut t: *mut SYMBOL, mut s: *mut SYMBOL) -> *mut SYMBOL {
    let mut s1: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut sy: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut next: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut g: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut r: libc::c_int = 0;
    let mut upstaff: libc::c_int = 0;
    let mut nb_only: libc::c_int = 0;
    let mut some_slur: libc::c_int = 0;
    let mut dir: libc::c_int = 0;
    let mut x1: libc::c_float = 0.;
    let mut x2: libc::c_float = 0.;
    let mut y1: libc::c_float = 0.;
    let mut y2: libc::c_float = 0.;
    let mut xm: libc::c_float = 0.;
    let mut ym: libc::c_float = 0.;
    let mut a: libc::c_float = 0.;
    let mut s0: libc::c_float = 0.;
    let mut yy: libc::c_float = 0.;
    let mut yx: libc::c_float = 0.;
    let mut dy: libc::c_float = 0.;
    next = s;
    if !((*t).aux as libc::c_int & 0xf000 as libc::c_int == 0x1000 as libc::c_int) {
        g = (*t).next;
        while !g.is_null() {
            if (*g).type_0 as libc::c_int == 13 as libc::c_int {
                sy = draw_tuplet(g, s);
                if (*sy).time > (*next).time {
                    next = sy;
                }
            }
            g = (*g).next;
        }
        r = (*t).u.tuplet.r_plet as libc::c_int;
        s1 = 0 as *mut SYMBOL;
        some_slur = 0 as libc::c_int;
        upstaff = (*s).staff as libc::c_int;
        s2 = s;
        while !s2.is_null() {
            if s2 != s && (*s2).sflags & 0x40 as libc::c_int as libc::c_uint != 0 {
                g = (*s2).extra;
                while !g.is_null() {
                    if (*g).type_0 as libc::c_int == 13 as libc::c_int {
                        sy = draw_tuplet(g, s2);
                        if (*sy).time > (*next).time {
                            next = sy;
                        }
                    }
                    g = (*g).next;
                }
            }
            if (*s2).type_0 as libc::c_int != 1 as libc::c_int {
                if (*s2).type_0 as libc::c_int == 11 as libc::c_int {
                    g = (*s2).extra;
                    while !g.is_null() {
                        if !((*g).type_0 as libc::c_int != 1 as libc::c_int) {
                            if (*g).u.note.slur_st as libc::c_int != 0
                                || (*g).sflags & 0x800 as libc::c_int as libc::c_uint != 0
                            {
                                some_slur = 1 as libc::c_int;
                            }
                        }
                        g = (*g).next;
                    }
                }
            } else {
                if (*s2).u.note.slur_st as libc::c_int != 0
                    || (*s2).u.note.slur_end as libc::c_int != 0
                    || (*s2).sflags
                        & (0x800 as libc::c_int | 0x1000 as libc::c_int) as libc::c_uint
                        != 0
                {
                    some_slur = 1 as libc::c_int;
                }
                if ((*s2).staff as libc::c_int) < upstaff {
                    upstaff = (*s2).staff as libc::c_int;
                }
                if s1.is_null() {
                    s1 = s2;
                }
                r -= 1;
                if r <= 0 as libc::c_int {
                    break;
                }
            }
            s2 = (*s2).next;
        }
        if !s2.is_null() {
            if (*s2).time > (*next).time {
                next = s2;
            }
            dir = (*t).aux as libc::c_int & 0xf as libc::c_int;
            if dir == 0 {
                dir = if (*s1).stem as libc::c_int > 0 as libc::c_int {
                    0x1 as libc::c_int
                } else {
                    0x2 as libc::c_int
                };
            }
            if s1 == s2 {
                nb_only = 1 as libc::c_int;
            } else if (*t).aux as libc::c_int & 0xf00 as libc::c_int
                == 0x100 as libc::c_int
            {
                nb_only = 1 as libc::c_int;
                draw_slur(s1, s2, -(1 as libc::c_int), -(1 as libc::c_int), dir);
            } else if (*t).aux as libc::c_int & 0xf000 as libc::c_int
                == 0x2000 as libc::c_int
                || (*s1).abc_type as libc::c_int != 4 as libc::c_int
                || (*s2).abc_type as libc::c_int != 4 as libc::c_int
            {
                nb_only = 0 as libc::c_int;
            } else {
                nb_only = 1 as libc::c_int;
                sy = s1;
                loop {
                    if (*sy).type_0 as libc::c_int != 1 as libc::c_int {
                        if !((*sy).type_0 as libc::c_int == 11 as libc::c_int
                            || (*sy).type_0 as libc::c_int == 2 as libc::c_int)
                        {
                            nb_only = 0 as libc::c_int;
                            break;
                        }
                    } else {
                        if sy == s2 {
                            break;
                        }
                        if (*sy).sflags & 0x10 as libc::c_int as libc::c_uint != 0 {
                            nb_only = 0 as libc::c_int;
                            break;
                        }
                    }
                    sy = (*sy).next;
                }
                if nb_only != 0
                    && (*s1).sflags
                        & (0x2 as libc::c_int | 0x4 as libc::c_int | 0x8 as libc::c_int)
                            as libc::c_uint == 0
                {
                    sy = (*s1).prev;
                    while !sy.is_null() {
                        if (*sy).type_0 as libc::c_int == 1 as libc::c_int {
                            if (*sy).nflags as libc::c_int >= (*s1).nflags as libc::c_int
                            {
                                nb_only = 0 as libc::c_int;
                            }
                            break;
                        } else {
                            sy = (*sy).prev;
                        }
                    }
                }
                if nb_only != 0
                    && (*s2).sflags & 0x10 as libc::c_int as libc::c_uint == 0
                {
                    sy = (*s2).next;
                    while !sy.is_null() {
                        if (*sy).type_0 as libc::c_int == 1 as libc::c_int {
                            if (*sy).sflags
                                & (0x4 as libc::c_int | 0x8 as libc::c_int) as libc::c_uint
                                == 0
                                && (*sy).nflags as libc::c_int
                                    >= (*s2).nflags as libc::c_int
                            {
                                nb_only = 0 as libc::c_int;
                            }
                            break;
                        } else {
                            sy = (*sy).next;
                        }
                    }
                }
            }
            if nb_only != 0 {
                let mut a_0: libc::c_float = 0.;
                let mut b: libc::c_float = 0.;
                if !((*t).aux as libc::c_int & 0xf0 as libc::c_int
                    == 0x10 as libc::c_int)
                {
                    xm = (((*s2).x + (*s1).x) as libc::c_double * 0.5f64)
                        as libc::c_float;
                    if s1 == s2 {
                        a_0 = 0 as libc::c_int as libc::c_float;
                    } else {
                        a_0 = ((*s2).ys - (*s1).ys) / ((*s2).x - (*s1).x);
                    }
                    b = (*s1).ys - a_0 * (*s1).x;
                    yy = a_0 * xm + b;
                    if dir == 0x1 as libc::c_int {
                        ym = y_get(
                            (*s1).staff as libc::c_int,
                            1 as libc::c_int,
                            xm - 3 as libc::c_int as libc::c_float,
                            6 as libc::c_int as libc::c_float,
                        );
                        if ym > yy {
                            b += ym - yy;
                        }
                        b += 2 as libc::c_int as libc::c_float;
                    } else {
                        ym = y_get(
                            (*s1).staff as libc::c_int,
                            0 as libc::c_int,
                            xm - 3 as libc::c_int as libc::c_float,
                            6 as libc::c_int as libc::c_float,
                        );
                        if ym < yy {
                            b += ym - yy;
                        }
                        b -= 10 as libc::c_int as libc::c_float;
                    }
                    sy = s1;
                    while !((*sy).x >= xm) {
                        sy = (*sy).next;
                    }
                    if (*s1).stem as libc::c_int * (*s2).stem as libc::c_int
                        > 0 as libc::c_int
                    {
                        if (*s1).stem as libc::c_int > 0 as libc::c_int {
                            xm = (xm as libc::c_double + 2.3f64) as libc::c_float;
                        } else {
                            xm = (xm as libc::c_double - 2.3f64) as libc::c_float;
                        }
                    }
                    ym = a_0 * xm + b;
                    if (*t).aux as libc::c_int & 0xf0 as libc::c_int == 0 as libc::c_int
                    {
                        a2b(
                            b"(%d)\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            (*t).u.tuplet.p_plet as libc::c_int,
                        );
                    } else {
                        a2b(
                            b"(%d:%d)\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            (*t).u.tuplet.p_plet as libc::c_int,
                            (*t).u.tuplet.q_plet as libc::c_int,
                        );
                    }
                    putxy(xm, ym);
                    a2b(
                        b"y%d bnum\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        (*s1).staff as libc::c_int,
                    );
                    if dir == 0x1 as libc::c_int {
                        ym += 8 as libc::c_int as libc::c_float;
                        if ((*sy).ymx as libc::c_int as libc::c_float) < ym {
                            (*sy).ymx = ym as libc::c_short as libc::c_schar;
                        }
                        y_set(
                            (*s1).staff as libc::c_int,
                            1 as libc::c_int,
                            xm - 3 as libc::c_int as libc::c_float,
                            6 as libc::c_int as libc::c_float,
                            ym,
                        );
                    } else {
                        if (*sy).ymn as libc::c_int as libc::c_float > ym {
                            (*sy).ymn = ym as libc::c_short as libc::c_schar;
                        }
                        y_set(
                            (*s1).staff as libc::c_int,
                            0 as libc::c_int,
                            xm - 3 as libc::c_int as libc::c_float,
                            6 as libc::c_int as libc::c_float,
                            ym,
                        );
                    }
                }
            } else {
                if some_slur != 0 {
                    draw_slurs(s1, s2);
                    if (*s1).u.note.slur_st as libc::c_int != 0
                        || (*s1).sflags & 0x800 as libc::c_int as libc::c_uint != 0
                    {
                        return next;
                    }
                    sy = (*s1).next;
                    while sy != s2 {
                        if (*sy).u.note.slur_st as libc::c_int != 0
                            || (*sy).u.note.slur_end as libc::c_int != 0
                            || (*sy).sflags
                                & (0x800 as libc::c_int | 0x1000 as libc::c_int)
                                    as libc::c_uint != 0
                        {
                            return next;
                        }
                        sy = (*sy).next;
                    }
                    if (*s2).u.note.slur_end as libc::c_int != 0
                        || (*s2).sflags & 0x1000 as libc::c_int as libc::c_uint != 0
                    {
                        return next;
                    }
                }
                if (*t).aux as libc::c_int & 0xf00 as libc::c_int != 0 as libc::c_int {
                    fprintf(
                        __stderrp,
                        b"'what' value of %%%%tuplets not yet coded\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                dir = (*t).aux as libc::c_int & 0xf as libc::c_int;
                if dir == 0 {
                    dir = if (*s1).multi as libc::c_int >= 0 as libc::c_int {
                        0x1 as libc::c_int
                    } else {
                        0x2 as libc::c_int
                    };
                }
                if dir == 0x1 as libc::c_int {
                    x1 = (*s1).x - 4 as libc::c_int as libc::c_float;
                    y1 = 24 as libc::c_int as libc::c_float;
                    if (*s1).staff as libc::c_int == upstaff {
                        sy = s1;
                        if (*sy).abc_type as libc::c_int != 4 as libc::c_int {
                            sy = (*sy).next;
                            while sy != s2 {
                                if (*sy).abc_type as libc::c_int == 4 as libc::c_int {
                                    break;
                                }
                                sy = (*sy).next;
                            }
                        }
                        ym = y_get(
                            upstaff,
                            1 as libc::c_int,
                            (*sy).x,
                            0 as libc::c_int as libc::c_float,
                        );
                        if ym > y1 {
                            y1 = ym;
                        }
                        if (*s1).stem as libc::c_int > 0 as libc::c_int {
                            x1 += 3 as libc::c_int as libc::c_float;
                        }
                    }
                    y2 = 24 as libc::c_int as libc::c_float;
                    if (*s2).staff as libc::c_int == upstaff {
                        sy = s2;
                        if (*sy).abc_type as libc::c_int != 4 as libc::c_int {
                            sy = (*sy).prev;
                            while sy != s1 {
                                if (*sy).abc_type as libc::c_int == 4 as libc::c_int {
                                    break;
                                }
                                sy = (*sy).prev;
                            }
                        }
                        ym = y_get(
                            upstaff,
                            1 as libc::c_int,
                            (*sy).x,
                            0 as libc::c_int as libc::c_float,
                        );
                        if ym > y2 {
                            y2 = ym;
                        }
                    }
                    if (*s2).dur > (*(*s2).prev).dur {
                        if !((*s2).next).is_null() {
                            x2 = (*(*s2).next).x - (*(*s2).next).wl
                                - 5 as libc::c_int as libc::c_float;
                        } else {
                            x2 = realwidth - 6 as libc::c_int as libc::c_float;
                        }
                    } else {
                        x2 = (*s2).x + 4 as libc::c_int as libc::c_float;
                        r = if (*s2).stem as libc::c_int >= 0 as libc::c_int {
                            0 as libc::c_int
                        } else {
                            (*s2).nhd as libc::c_int
                        };
                        if (*s2).u.note.notes[r as usize].shhd
                            > 0 as libc::c_int as libc::c_float
                        {
                            x2 += (*s2).u.note.notes[r as usize].shhd;
                        }
                        if (*s2).staff as libc::c_int == upstaff
                            && (*s2).stem as libc::c_int > 0 as libc::c_int
                        {
                            x2 = (x2 as libc::c_double + 3.5f64) as libc::c_float;
                        }
                    }
                    xm = (0.5f64 * (x1 + x2) as libc::c_double) as libc::c_float;
                    ym = (0.5f64 * (y1 + y2) as libc::c_double) as libc::c_float;
                    a = (y2 - y1) / (x2 - x1);
                    s0 = (3 as libc::c_int
                        * ((*s2).pits[(*s2).nhd as usize] as libc::c_int
                            - (*s1).pits[(*s1).nhd as usize] as libc::c_int))
                        as libc::c_float / (x2 - x1);
                    if s0 > 0 as libc::c_int as libc::c_float {
                        if a < 0 as libc::c_int as libc::c_float {
                            a = 0 as libc::c_int as libc::c_float;
                        } else if a > s0 {
                            a = s0;
                        }
                    } else if a > 0 as libc::c_int as libc::c_float {
                        a = 0 as libc::c_int as libc::c_float;
                    } else if a < s0 {
                        a = s0;
                    }
                    if ((a * a) as libc::c_double) < 0.1f64 * 0.1f64 {
                        a = 0 as libc::c_int as libc::c_float;
                    }
                    dy = 0 as libc::c_int as libc::c_float;
                    sy = s1;
                    loop {
                        if (*sy).dur == 0 as libc::c_int
                            || (*sy).staff as libc::c_int != upstaff
                        {
                            if sy == s2 {
                                break;
                            }
                        } else {
                            yy = ym + ((*sy).x - xm) * a;
                            yx = y_get(
                                upstaff,
                                1 as libc::c_int,
                                (*sy).x,
                                0 as libc::c_int as libc::c_float,
                            );
                            if yx - yy > dy {
                                dy = yx - yy;
                            }
                            if sy == s2 {
                                break;
                            }
                        }
                        sy = (*sy).next;
                    }
                    ym += dy;
                    y1 = ym + a * (x1 - xm);
                    y2 = ym + a * (x2 - xm);
                    putxy(x2 - x1, y2 - y1);
                    putxy(x1, y1 + 4 as libc::c_int as libc::c_float);
                    a2b(
                        b"y%d tubr\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        upstaff,
                    );
                    ym += 8 as libc::c_int as libc::c_float;
                    sy = s1;
                    loop {
                        if (*sy).staff as libc::c_int == upstaff {
                            yy = ym + ((*sy).x - xm) * a;
                            if ((*sy).ymx as libc::c_int as libc::c_float) < yy {
                                (*sy).ymx = yy as libc::c_schar;
                            }
                            if sy == s2 {
                                break;
                            }
                            y_set(
                                upstaff,
                                1 as libc::c_int,
                                (*sy).x,
                                (*(*sy).next).x - (*sy).x,
                                yy,
                            );
                        } else if sy == s2 {
                            break;
                        }
                        sy = (*sy).next;
                    }
                } else {
                    x1 = (*s1).x - 7 as libc::c_int as libc::c_float;
                    if (*s2).dur > (*(*s2).prev).dur {
                        sy = (*s2).next;
                        if sy.is_null() || (*sy).time != (*s2).time + (*s2).dur {
                            sy = (*s2).ts_next;
                            while !sy.is_null() {
                                if (*sy).sflags & 0x80000 as libc::c_int as libc::c_uint
                                    != 0 && (*sy).time >= (*s2).time + (*s2).dur
                                {
                                    break;
                                }
                                sy = (*sy).ts_next;
                            }
                        }
                        x2 = if !sy.is_null() {
                            (*sy).x - (*sy).wl - 5 as libc::c_int as libc::c_float
                        } else {
                            realwidth - 6 as libc::c_int as libc::c_float
                        };
                    } else {
                        x2 = (*s2).x + 2 as libc::c_int as libc::c_float;
                        if (*s2).u.note.notes[(*s2).nhd as usize].shhd
                            > 0 as libc::c_int as libc::c_float
                        {
                            x2 += (*s2).u.note.notes[(*s2).nhd as usize].shhd;
                        }
                    }
                    if (*s1).staff as libc::c_int == upstaff {
                        sy = s1;
                        if (*sy).abc_type as libc::c_int != 4 as libc::c_int {
                            sy = (*sy).next;
                            while sy != s2 {
                                if (*sy).abc_type as libc::c_int == 4 as libc::c_int {
                                    break;
                                }
                                sy = (*sy).next;
                            }
                        }
                        y1 = y_get(
                            upstaff,
                            0 as libc::c_int,
                            (*sy).x,
                            0 as libc::c_int as libc::c_float,
                        );
                    } else {
                        y1 = 0 as libc::c_int as libc::c_float;
                    }
                    if (*s2).staff as libc::c_int == upstaff {
                        sy = s2;
                        if (*sy).abc_type as libc::c_int != 4 as libc::c_int {
                            sy = (*sy).prev;
                            while sy != s1 {
                                if (*sy).abc_type as libc::c_int == 4 as libc::c_int {
                                    break;
                                }
                                sy = (*sy).prev;
                            }
                        }
                        y2 = y_get(
                            upstaff,
                            0 as libc::c_int,
                            (*sy).x,
                            0 as libc::c_int as libc::c_float,
                        );
                    } else {
                        y2 = 0 as libc::c_int as libc::c_float;
                    }
                    xm = (0.5f64 * (x1 + x2) as libc::c_double) as libc::c_float;
                    ym = (0.5f64 * (y1 + y2) as libc::c_double) as libc::c_float;
                    a = (y2 - y1) / (x2 - x1);
                    s0 = (3 as libc::c_int
                        * ((*s2).pits[0 as libc::c_int as usize] as libc::c_int
                            - (*s1).pits[0 as libc::c_int as usize] as libc::c_int))
                        as libc::c_float / (x2 - x1);
                    if s0 > 0 as libc::c_int as libc::c_float {
                        if a < 0 as libc::c_int as libc::c_float {
                            a = 0 as libc::c_int as libc::c_float;
                        } else if a > s0 {
                            a = s0;
                        }
                    } else if a > 0 as libc::c_int as libc::c_float {
                        a = 0 as libc::c_int as libc::c_float;
                    } else if a < s0 {
                        a = s0;
                    }
                    if ((a * a) as libc::c_double) < 0.1f64 * 0.1f64 {
                        a = 0 as libc::c_int as libc::c_float;
                    }
                    dy = 0 as libc::c_int as libc::c_float;
                    sy = s1;
                    loop {
                        if (*sy).dur == 0 as libc::c_int
                            || (*sy).staff as libc::c_int != upstaff
                        {
                            if sy == s2 {
                                break;
                            }
                        } else {
                            yy = ym + ((*sy).x - xm) * a;
                            yx = y_get(
                                upstaff,
                                0 as libc::c_int,
                                (*sy).x,
                                0 as libc::c_int as libc::c_float,
                            );
                            if yx - yy < dy {
                                dy = yx - yy;
                            }
                            if sy == s2 {
                                break;
                            }
                        }
                        sy = (*sy).next;
                    }
                    ym += dy - 10 as libc::c_int as libc::c_float;
                    y1 = ym + a * (x1 - xm);
                    y2 = ym + a * (x2 - xm);
                    putxy(x2 - x1, y2 - y1);
                    putxy(x1, y1 + 4 as libc::c_int as libc::c_float);
                    a2b(
                        b"y%d tubrl\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        upstaff,
                    );
                    ym -= 2 as libc::c_int as libc::c_float;
                    sy = s1;
                    loop {
                        if (*sy).staff as libc::c_int == upstaff {
                            if sy == s2 {
                                break;
                            }
                            yy = ym + ((*sy).x - xm) * a;
                            if (*sy).ymn as libc::c_int as libc::c_float > yy {
                                (*sy).ymn = yy as libc::c_short as libc::c_schar;
                            }
                            y_set(
                                upstaff,
                                0 as libc::c_int,
                                (*sy).x,
                                (*(*sy).next).x - (*sy).x,
                                yy,
                            );
                        }
                        if sy == s2 {
                            break;
                        }
                        sy = (*sy).next;
                    }
                }
                if (*t).aux as libc::c_int & 0xf0 as libc::c_int == 0x10 as libc::c_int {
                    a2b(
                        b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                } else {
                    yy = (0.5f64 * (y1 + y2) as libc::c_double) as libc::c_float;
                    if (*t).aux as libc::c_int & 0xf0 as libc::c_int == 0 as libc::c_int
                    {
                        a2b(
                            b"(%d)\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            (*t).u.tuplet.p_plet as libc::c_int,
                        );
                    } else {
                        a2b(
                            b"(%d:%d)\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            (*t).u.tuplet.p_plet as libc::c_int,
                            (*t).u.tuplet.q_plet as libc::c_int,
                        );
                    }
                    putxy(xm, yy);
                    a2b(
                        b"y%d bnumb\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        upstaff,
                    );
                    if dir == 0x1 as libc::c_int {
                        y_set(
                            upstaff,
                            1 as libc::c_int,
                            xm - 3 as libc::c_int as libc::c_float,
                            6 as libc::c_int as libc::c_float,
                            ym + 3 as libc::c_int as libc::c_float,
                        );
                    } else {
                        y_set(
                            upstaff,
                            0 as libc::c_int,
                            xm - 3 as libc::c_int as libc::c_float,
                            6 as libc::c_int as libc::c_float,
                            ym,
                        );
                    }
                }
            }
        }
    }
    (*s).sflags &= !(0x40 as libc::c_int) as libc::c_uint;
    return next;
}
unsafe extern "C" fn draw_note_ties(
    mut k1: *mut SYMBOL,
    mut k2: *mut SYMBOL,
    mut ntie: libc::c_int,
    mut mhead1: *mut libc::c_uchar,
    mut mhead2: *mut libc::c_uchar,
    mut job: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut m1: libc::c_int = 0;
    let mut m2: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut p1: libc::c_int = 0;
    let mut p2: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut staff: libc::c_int = 0;
    let mut x1: libc::c_float = 0.;
    let mut x2: libc::c_float = 0.;
    let mut h: libc::c_float = 0.;
    let mut sh: libc::c_float = 0.;
    i = 0 as libc::c_int;
    while i < ntie {
        m1 = *mhead1.offset(i as isize) as libc::c_int;
        p1 = (*k1).pits[m1 as usize] as libc::c_int;
        m2 = *mhead2.offset(i as isize) as libc::c_int;
        p2 = if job != 2 as libc::c_int {
            (*k2).pits[m2 as usize] as libc::c_int
        } else {
            p1
        };
        s = if (*k1).u.note.notes[m1 as usize].ti1 as libc::c_int & 0x7 as libc::c_int
            == 0x1 as libc::c_int
        {
            1 as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
        x1 = (*k1).x;
        sh = (*k1).u.note.notes[m1 as usize].shhd;
        if s > 0 as libc::c_int {
            if m1 < (*k1).nhd as libc::c_int
                && p1 + 1 as libc::c_int
                    == (*k1).pits[(m1 + 1 as libc::c_int) as usize] as libc::c_int
            {
                if (*k1).u.note.notes[(m1 + 1 as libc::c_int) as usize].shhd > sh {
                    sh = (*k1).u.note.notes[(m1 + 1 as libc::c_int) as usize].shhd;
                }
            }
        } else if m1 > 0 as libc::c_int
            && p1
                == (*k1).pits[(m1 - 1 as libc::c_int) as usize] as libc::c_int
                    + 1 as libc::c_int
        {
            if (*k1).u.note.notes[(m1 - 1 as libc::c_int) as usize].shhd > sh {
                sh = (*k1).u.note.notes[(m1 - 1 as libc::c_int) as usize].shhd;
            }
        }
        x1 = (x1 as libc::c_double + sh as libc::c_double * 0.6f64) as libc::c_float;
        x2 = (*k2).x;
        if job != 2 as libc::c_int {
            sh = (*k2).u.note.notes[m2 as usize].shhd;
            if s > 0 as libc::c_int {
                if m2 < (*k2).nhd as libc::c_int
                    && p2 + 1 as libc::c_int
                        == (*k2).pits[(m2 + 1 as libc::c_int) as usize] as libc::c_int
                {
                    if (*k2).u.note.notes[(m2 + 1 as libc::c_int) as usize].shhd < sh {
                        sh = (*k2).u.note.notes[(m2 + 1 as libc::c_int) as usize].shhd;
                    }
                }
            } else if m2 > 0 as libc::c_int
                && p2
                    == (*k2).pits[(m2 - 1 as libc::c_int) as usize] as libc::c_int
                        + 1 as libc::c_int
            {
                if (*k2).u.note.notes[(m2 - 1 as libc::c_int) as usize].shhd < sh {
                    sh = (*k2).u.note.notes[(m2 - 1 as libc::c_int) as usize].shhd;
                }
            }
            x2 = (x2 as libc::c_double + sh as libc::c_double * 0.6f64) as libc::c_float;
        }
        staff = (*k1).staff as libc::c_int;
        let mut current_block_56: u64;
        match job {
            0 => {
                if p1 == p2 || p1 & 1 as libc::c_int != 0 {
                    p = p1;
                } else {
                    p = p2;
                }
                current_block_56 = 11777552016271000781;
            }
            3 => {
                s = -s;
                current_block_56 = 13737633757930754920;
            }
            1 => {
                current_block_56 = 13737633757930754920;
            }
            _ => {
                if k1 != k2 {
                    if (*k2).type_0 as libc::c_int == 3 as libc::c_int {
                        x2 -= 2 as libc::c_int as libc::c_float;
                    } else {
                        x2 -= (*k2).wl;
                    }
                } else {
                    let mut k: *mut SYMBOL = 0 as *mut SYMBOL;
                    let mut time: libc::c_int = 0;
                    time = (*k1).time + (*k1).dur;
                    k = (*k1).ts_next;
                    while !k.is_null() {
                        if (*k).time > time {
                            break;
                        }
                        k = (*k).ts_next;
                    }
                    if k.is_null() {
                        x2 = realwidth;
                    } else {
                        x2 = (*k).x;
                    }
                }
                if x2 < x1 + 16 as libc::c_int as libc::c_float {
                    x2 = x1 + 16 as libc::c_int as libc::c_float;
                }
                p = p1;
                current_block_56 = 11777552016271000781;
            }
        }
        match current_block_56 {
            13737633757930754920 => {
                x1 = (*k1).x;
                if x1 > x2 - 20 as libc::c_int as libc::c_float {
                    x1 = x2 - 20 as libc::c_int as libc::c_float;
                }
                p = p2;
                staff = (*k2).staff as libc::c_int;
            }
            _ => {}
        }
        if x2 - x1 > 20 as libc::c_int as libc::c_float {
            x1 = (x1 as libc::c_double + 3.5f64) as libc::c_float;
            x2 = (x2 as libc::c_double - 3.5f64) as libc::c_float;
        } else {
            x1 = (x1 as libc::c_double + 1.5f64) as libc::c_float;
            x2 = (x2 as libc::c_double - 1.5f64) as libc::c_float;
        }
        if (*k1).dots as libc::c_int != 0 && p1 & 1 as libc::c_int == 0
            && (s > 0 as libc::c_int && (*k1).doty as libc::c_int >= 0 as libc::c_int
                || s < 0 as libc::c_int
                    && ((*k1).doty as libc::c_int) < 0 as libc::c_int)
        {
            x1 += 6 as libc::c_int as libc::c_float;
        }
        y = 3 as libc::c_int * (p - 18 as libc::c_int);
        h = ((0.04f64 * (x2 - x1) as libc::c_double
            + 10 as libc::c_int as libc::c_double) * s as libc::c_double)
            as libc::c_float;
        h *= cfmt.tieheight;
        slur_out(
            x1,
            staff_tb[staff as usize].y + y as libc::c_float,
            x2,
            staff_tb[staff as usize].y + y as libc::c_float,
            s,
            h,
            (*k1).u.note.notes[m1 as usize].ti1 as libc::c_int & 0x8 as libc::c_int,
            -(1 as libc::c_int),
        );
        i += 1;
        i;
    }
}
unsafe extern "C" fn draw_ties(
    mut k1: *mut SYMBOL,
    mut k2: *mut SYMBOL,
    mut job: libc::c_int,
) {
    let mut k3: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut i: libc::c_int = 0;
    let mut m1: libc::c_int = 0;
    let mut nh1: libc::c_int = 0;
    let mut pit: libc::c_int = 0;
    let mut ntie: libc::c_int = 0;
    let mut tie2: libc::c_int = 0;
    let mut ntie3: libc::c_int = 0;
    let mut time: libc::c_int = 0;
    let mut mhead1: [libc::c_uchar; 8] = [0; 8];
    let mut mhead2: [libc::c_uchar; 8] = [0; 8];
    let mut mhead3: [libc::c_uchar; 8] = [0; 8];
    time = (*k1).time + (*k1).dur;
    ntie3 = 0 as libc::c_int;
    ntie = ntie3;
    nh1 = (*k1).nhd as libc::c_int;
    if job == 2 as libc::c_int {
        i = 0 as libc::c_int;
        while i <= nh1 {
            if (*k1).u.note.notes[i as usize].ti1 != 0 {
                let fresh9 = ntie3;
                ntie3 = ntie3 + 1;
                mhead3[fresh9 as usize] = i as libc::c_uchar;
            }
            i += 1;
            i;
        }
        draw_note_ties(
            k1,
            if !k2.is_null() { k2 } else { k1 },
            ntie3,
            mhead3.as_mut_ptr(),
            mhead3.as_mut_ptr(),
            job,
        );
        return;
    }
    i = 0 as libc::c_int;
    while i <= nh1 {
        if !((*k1).u.note.notes[i as usize].ti1 as libc::c_int == 0 as libc::c_int) {
            tie2 = -(1 as libc::c_int);
            pit = (*k1).u.note.notes[i as usize].pit as libc::c_int;
            m1 = (*k2).nhd as libc::c_int;
            while m1 >= 0 as libc::c_int {
                match (*k2).u.note.notes[m1 as usize].pit as libc::c_int - pit {
                    1 | -1 => {
                        if (*k1).u.note.notes[i as usize].acc as libc::c_int
                            != (*k2).u.note.notes[m1 as usize].acc as libc::c_int
                        {
                            tie2 = m1;
                        }
                    }
                    0 => {
                        tie2 = m1;
                        break;
                    }
                    _ => {}
                }
                m1 -= 1;
                m1;
            }
            if tie2 >= 0 as libc::c_int {
                mhead1[ntie as usize] = i as libc::c_uchar;
                let fresh10 = ntie;
                ntie = ntie + 1;
                mhead2[fresh10 as usize] = tie2 as libc::c_uchar;
            } else {
                let fresh11 = ntie3;
                ntie3 = ntie3 + 1;
                mhead3[fresh11 as usize] = i as libc::c_uchar;
            }
        }
        i += 1;
        i;
    }
    draw_note_ties(k1, k2, ntie, mhead1.as_mut_ptr(), mhead2.as_mut_ptr(), job);
    if ntie3 == 0 as libc::c_int {
        return;
    }
    k3 = (*k1).ts_next;
    while !k3.is_null() && (*k3).time < time {
        k3 = (*k3).ts_next;
    }
    while !k3.is_null() && (*k3).time == time {
        if (*k3).abc_type as libc::c_int != 4 as libc::c_int
            || (*k3).staff as libc::c_int != (*k1).staff as libc::c_int
        {
            k3 = (*k3).ts_next;
        } else {
            ntie = 0 as libc::c_int;
            i = ntie3;
            loop {
                i -= 1;
                if !(i >= 0 as libc::c_int) {
                    break;
                }
                pit = (*k1).u.note.notes[mhead3[i as usize] as usize].pit as libc::c_int;
                m1 = (*k3).nhd as libc::c_int;
                while m1 >= 0 as libc::c_int {
                    if (*k3).u.note.notes[m1 as usize].pit as libc::c_int == pit {
                        mhead1[ntie as usize] = mhead3[i as usize];
                        let fresh12 = ntie;
                        ntie = ntie + 1;
                        mhead2[fresh12 as usize] = m1 as libc::c_uchar;
                        ntie3 -= 1;
                        ntie3;
                        break;
                    } else {
                        m1 -= 1;
                        m1;
                    }
                }
            }
            if ntie > 0 as libc::c_int {
                draw_note_ties(
                    k1,
                    k3,
                    ntie,
                    mhead1.as_mut_ptr(),
                    mhead2.as_mut_ptr(),
                    if job == 1 as libc::c_int {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    },
                );
                if ntie3 == 0 as libc::c_int {
                    return;
                }
            }
            k3 = (*k3).ts_next;
        }
    }
    error(
        1 as libc::c_int,
        k1,
        b"Bad tie\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
unsafe extern "C" fn draw_ties_g(
    mut s1: *mut SYMBOL,
    mut s2: *mut SYMBOL,
    mut job: libc::c_int,
) {
    let mut g: *mut SYMBOL = 0 as *mut SYMBOL;
    if (*s1).type_0 as libc::c_int == 11 as libc::c_int {
        g = (*s1).extra;
        while !g.is_null() {
            if (*g).sflags & 0x2000 as libc::c_int as libc::c_uint != 0 {
                draw_ties(g, s2, job);
            }
            g = (*g).next;
        }
    } else {
        draw_ties(s1, s2, job);
    };
}
unsafe extern "C" fn tie_comb(mut s: *mut SYMBOL) -> *mut SYMBOL {
    let mut s1: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut time: libc::c_int = 0;
    let mut st: libc::c_int = 0;
    time = (*s).time + (*s).dur;
    st = (*s).staff as libc::c_int;
    s1 = (*s).ts_next;
    while !s1.is_null() {
        if !((*s1).staff as libc::c_int != st) {
            if (*s1).time == time {
                if (*s1).abc_type as libc::c_int == 4 as libc::c_int {
                    return s1;
                }
            } else if (*s1).time > time {
                return s
            }
        }
        s1 = (*s1).ts_next;
    }
    return 0 as *mut SYMBOL;
}
unsafe extern "C" fn draw_all_ties(mut p_voice: *mut VOICE_S) {
    let mut s1: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s3: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s4: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut rtie: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut tie: SYMBOL = SYMBOL {
        abc_next: 0 as *mut SYMBOL,
        abc_prev: 0 as *mut SYMBOL,
        next: 0 as *mut SYMBOL,
        prev: 0 as *mut SYMBOL,
        ts_next: 0 as *mut SYMBOL,
        ts_prev: 0 as *mut SYMBOL,
        extra: 0 as *mut SYMBOL,
        abc_type: 0,
        type_0: 0,
        voice: 0,
        staff: 0,
        nhd: 0,
        pits: [0; 8],
        dur: 0,
        time: 0,
        sflags: 0,
        posit: posit_s {
            dyn_0_gch_orn_voc_vol_std_gsd: [0; 4],
        },
        stem: 0,
        combine: 0,
        nflags: 0,
        dots: 0,
        head: 0,
        multi: 0,
        nohdi1: 0,
        nohdi2: 0,
        doty: 0,
        aux: 0,
        color: 0,
        x: 0.,
        y: 0,
        ymn: 0,
        ymx: 0,
        mid: 0,
        xmx: 0.,
        xs: 0.,
        ys: 0.,
        wl: 0.,
        wr: 0.,
        space: 0.,
        shrink: 0.,
        xmax: 0.,
        gch: 0 as *mut gch,
        ly: 0 as *mut lyrics,
        state: 0,
        flags: 0,
        colnum: 0,
        linenum: 0,
        fn_0: 0 as *mut libc::c_char,
        text: 0 as *mut libc::c_char,
        u: C2RustUnnamed_0 {
            key: key_s {
                sf: 0,
                empty: 0,
                exp: 0,
                instr: 0,
                nacc: 0,
                cue: 0,
                octave: 0,
                microscale: 0,
                clef_delta: 0,
                key_delta: 0,
                stafflines: 0 as *const libc::c_char as *mut libc::c_char,
                staffscale: 0.,
                pits: [0; 8],
                accs: [0; 8],
            },
        },
    };
    let mut clef_chg: libc::c_int = 0;
    let mut time: libc::c_int = 0;
    s1 = (*p_voice).sym;
    while !s1.is_null() {
        match (*s1).type_0 as libc::c_int {
            4 | 6 | 5 => {}
            _ => {
                break;
            }
        }
        s1 = (*s1).next;
    }
    rtie = (*p_voice).rtie;
    s2 = s1;
    while !s2.is_null() {
        if (*s2).dur != 0 || (*s2).type_0 as libc::c_int == 11 as libc::c_int {
            break;
        }
        if !((*s2).type_0 as libc::c_int != 3 as libc::c_int
            || (*s2).u.bar.repeat_bar == 0 || ((*s2).text).is_null())
        {
            if *((*s2).text).offset(0 as libc::c_int as isize) as libc::c_int
                == '1' as i32
            {
                rtie = (*p_voice).tie;
            } else {
                (*p_voice).tie = rtie;
            }
        }
        s2 = (*s2).next;
    }
    if s2.is_null() {
        return;
    }
    if !((*p_voice).tie).is_null() {
        (*(*p_voice).tie).x = (*s1).x + (*s1).wr;
        s1 = (*p_voice).tie;
        (*p_voice).tie = 0 as *mut SYMBOL;
        (*s1).staff = (*s2).staff;
        (*s1).ts_next = (*s2).ts_next;
        (*s1).time = (*s2).time - (*s1).dur;
        draw_ties(s1, s2, 1 as libc::c_int);
    }
    clef_chg = 0 as libc::c_int;
    loop {
        s1 = s2;
        while !s1.is_null() {
            if (*s1).sflags & 0x2000 as libc::c_int as libc::c_uint != 0 {
                break;
            }
            if !rtie.is_null() {
                if !((*s1).type_0 as libc::c_int != 3 as libc::c_int
                    || (*s1).u.bar.repeat_bar == 0 || ((*s1).text).is_null())
                {
                    if *((*s1).text).offset(0 as libc::c_int as isize) as libc::c_int
                        == '1' as i32
                    {
                        rtie = 0 as *mut SYMBOL;
                    } else if !((*s1).u.bar.type_0 == 1 as libc::c_int) {
                        s2 = (*s1).next;
                        while !s2.is_null() {
                            if (*s2).abc_type as libc::c_int == 4 as libc::c_int {
                                break;
                            }
                            s2 = (*s2).next;
                        }
                        if s2.is_null() {
                            s1 = 0 as *mut SYMBOL;
                            break;
                        } else {
                            memcpy(
                                &mut tie as *mut SYMBOL as *mut libc::c_void,
                                rtie as *const libc::c_void,
                                ::core::mem::size_of::<SYMBOL>() as libc::c_ulong,
                            );
                            tie.x = (*s1).x;
                            tie.next = s2;
                            tie.staff = (*s2).staff;
                            tie.time = (*s2).time - tie.dur;
                            draw_ties(&mut tie, s2, 1 as libc::c_int);
                        }
                    }
                }
            }
            s1 = (*s1).next;
        }
        if s1.is_null() {
            break;
        }
        time = (*s1).time + (*s1).dur;
        s2 = (*s1).next;
        while !s2.is_null() {
            if (*s2).dur != 0 as libc::c_int {
                break;
            }
            if (*s2).type_0 as libc::c_int == 3 as libc::c_int
                && (*s2).u.bar.repeat_bar as libc::c_int != 0 && !((*s2).text).is_null()
            {
                if *((*s2).text).offset(0 as libc::c_int as isize) as libc::c_int
                    != '1' as i32
                {
                    break;
                }
                rtie = s1;
            }
            s2 = (*s2).next;
        }
        if s2.is_null() {
            s4 = (*s1).ts_next;
            while !s4.is_null() {
                if !((*s4).staff as libc::c_int != (*s1).staff as libc::c_int) {
                    if !((*s4).time < time) {
                        if (*s4).time > time {
                            s4 = 0 as *mut SYMBOL;
                            break;
                        } else if (*s4).dur != 0 as libc::c_int {
                            break;
                        }
                    }
                }
                s4 = (*s4).ts_next;
            }
            if s4.is_null() {
                draw_ties_g(s1, 0 as *mut SYMBOL, 2 as libc::c_int);
                (*p_voice).tie = s1;
                break;
            }
        } else if (*s2).abc_type as libc::c_int != 4 as libc::c_int
            && (*s2).abc_type as libc::c_int != 6 as libc::c_int
        {
            error(
                1 as libc::c_int,
                s1,
                b"Bad tie\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            continue;
        } else if (*s2).time == time {
            s4 = s2;
        } else {
            s4 = tie_comb(s1);
            if s4 == s1 {
                error(
                    1 as libc::c_int,
                    s1,
                    b"Bad tie\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                continue;
            }
        }
        s3 = (*s1).ts_next;
        while !s3.is_null() {
            if !((*s3).staff as libc::c_int != (*s1).staff as libc::c_int) {
                if (*s3).time > time {
                    break;
                }
                if (*s3).type_0 as libc::c_int == 4 as libc::c_int {
                    clef_chg = 1 as libc::c_int;
                }
            }
            s3 = (*s3).ts_next;
        }
        if clef_chg != 0 || (*s1).staff as libc::c_int != (*s4).staff as libc::c_int {
            let mut x: libc::c_float = 0.;
            let mut dx: libc::c_float = 0.;
            clef_chg = 0 as libc::c_int;
            dx = (((*s4).x - (*s1).x) as libc::c_double * 0.4f64) as libc::c_float;
            x = (*s4).x;
            (*s4).x -= dx;
            if (*s4).x as libc::c_double > (*s1).x as libc::c_double + 32.0f64 {
                (*s4).x = ((*s1).x as libc::c_double + 32.0f64) as libc::c_float;
            }
            draw_ties_g(s1, s4, 2 as libc::c_int);
            (*s4).x = x;
            x = (*s1).x;
            (*s1).x += dx;
            if ((*s1).x as libc::c_double) < (*s4).x as libc::c_double - 24.0f64 {
                (*s1).x = ((*s4).x as libc::c_double - 24.0f64) as libc::c_float;
            }
            draw_ties(s1, s4, 3 as libc::c_int);
            (*s1).x = x;
        } else {
            draw_ties_g(
                s1,
                s4,
                if (*s4).abc_type as libc::c_int == 4 as libc::c_int {
                    0 as libc::c_int
                } else {
                    2 as libc::c_int
                },
            );
        }
    }
    (*p_voice).rtie = rtie;
}
unsafe extern "C" fn draw_all_slurs(mut p_voice: *mut VOICE_S) {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut k: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut i: libc::c_int = 0;
    let mut m2: libc::c_int = 0;
    let mut slur_type: libc::c_int = 0;
    let mut slur_st: libc::c_uchar = 0;
    s = (*p_voice).sym;
    if s.is_null() {
        return;
    }
    slur_type = (*p_voice).slur_st as libc::c_int;
    (*p_voice).slur_st = 0 as libc::c_int as libc::c_uchar;
    slur_st = 0 as libc::c_int as libc::c_uchar;
    while slur_type != 0 as libc::c_int {
        slur_st = ((slur_st as libc::c_int) << 4 as libc::c_int) as libc::c_uchar;
        slur_st = (slur_st as libc::c_int | slur_type & 0xf as libc::c_int)
            as libc::c_uchar;
        slur_type >>= 4 as libc::c_int;
    }
    draw_slurs(s, 0 as *mut SYMBOL);
    while !s.is_null() {
        if !((*s).type_0 as libc::c_int != 1 as libc::c_int
            && (*s).type_0 as libc::c_int != 2 as libc::c_int)
        {
            while (*s).u.note.slur_end as libc::c_int != 0
                || (*s).sflags & 0x1000 as libc::c_int as libc::c_uint != 0
            {
                if (*s).u.note.slur_end != 0 {
                    (*s).u.note.slur_end -= 1;
                    (*s).u.note.slur_end;
                    m2 = -(1 as libc::c_int);
                } else {
                    m2 = 0 as libc::c_int;
                    while m2 <= (*s).nhd as libc::c_int {
                        if (*s).u.note.notes[m2 as usize].sl2 != 0 {
                            break;
                        }
                        m2 += 1;
                        m2;
                    }
                    (*s).u.note.notes[m2 as usize].sl2 -= 1;
                    (*s).u.note.notes[m2 as usize].sl2;
                    if (*s).u.note.notes[m2 as usize].sl2 as libc::c_int
                        == 0 as libc::c_int
                    {
                        i = m2 + 1 as libc::c_int;
                        while i <= (*s).nhd as libc::c_int {
                            if (*s).u.note.notes[i as usize].sl2 != 0 {
                                break;
                            }
                            i += 1;
                            i;
                        }
                        if i > (*s).nhd as libc::c_int {
                            (*s).sflags &= !(0x1000 as libc::c_int) as libc::c_uint;
                        }
                    }
                }
                slur_type = slur_st as libc::c_int & 0xf as libc::c_int;
                k = prev_scut(s);
                draw_slur(k, s, -(1 as libc::c_int), m2, slur_type);
                if (*k).type_0 as libc::c_int != 3 as libc::c_int
                    || (*k).sflags & 0x100 as libc::c_int as libc::c_uint == 0
                        && (*k).u.bar.type_0 != 0x13 as libc::c_int
                        && (*k).u.bar.type_0 != 0x21 as libc::c_int
                        && ((*k).u.bar.repeat_bar == 0 || ((*k).text).is_null()
                            || *((*k).text).offset(0 as libc::c_int as isize)
                                as libc::c_int == '1' as i32)
                {
                    slur_st = (slur_st as libc::c_int >> 4 as libc::c_int)
                        as libc::c_uchar;
                }
            }
        }
        s = (*s).next;
    }
    s = (*p_voice).sym;
    while slur_st as libc::c_int != 0 as libc::c_int {
        slur_type = slur_st as libc::c_int & 0xf as libc::c_int;
        slur_st = (slur_st as libc::c_int >> 4 as libc::c_int) as libc::c_uchar;
        k = next_scut(s);
        draw_slur(s, k, -(1 as libc::c_int), -(1 as libc::c_int), slur_type);
        if (*k).type_0 as libc::c_int != 3 as libc::c_int
            || (*k).sflags & 0x100 as libc::c_int as libc::c_uint == 0
                && (*k).u.bar.type_0 != 0x13 as libc::c_int
                && (*k).u.bar.type_0 != 0x21 as libc::c_int
                && ((*k).u.bar.repeat_bar == 0 || ((*k).text).is_null()
                    || *((*k).text).offset(0 as libc::c_int as isize) as libc::c_int
                        == '1' as i32)
        {
            (*p_voice)
                .slur_st = (((*p_voice).slur_st as libc::c_int) << 4 as libc::c_int)
                as libc::c_uchar;
            (*p_voice)
                .slur_st = ((*p_voice).slur_st as libc::c_int + slur_type)
                as libc::c_uchar;
        }
    }
}
unsafe extern "C" fn setmap(mut sf: libc::c_int, mut map: *mut libc::c_uchar) {
    let mut j: libc::c_int = 0;
    j = 7 as libc::c_int;
    loop {
        j -= 1;
        if !(j >= 0 as libc::c_int) {
            break;
        }
        *map.offset(j as isize) = A_NULL as libc::c_int as libc::c_uchar;
    }
    let mut current_block_16: u64;
    match sf {
        7 => {
            *map
                .offset(
                    6 as libc::c_int as isize,
                ) = A_SH as libc::c_int as libc::c_uchar;
            current_block_16 = 16237308621499531990;
        }
        6 => {
            current_block_16 = 16237308621499531990;
        }
        5 => {
            current_block_16 = 1395401040827046706;
        }
        4 => {
            current_block_16 = 492316982938702853;
        }
        3 => {
            current_block_16 = 12191447810917241774;
        }
        2 => {
            current_block_16 = 10069085333865129812;
        }
        1 => {
            current_block_16 = 2244607058334101863;
        }
        -7 => {
            *map
                .offset(
                    3 as libc::c_int as isize,
                ) = A_FT as libc::c_int as libc::c_uchar;
            current_block_16 = 16827810745824149869;
        }
        -6 => {
            current_block_16 = 16827810745824149869;
        }
        -5 => {
            current_block_16 = 5190931071520054375;
        }
        -4 => {
            current_block_16 = 1331651794105737270;
        }
        -3 => {
            current_block_16 = 9541762626985535097;
        }
        -2 => {
            current_block_16 = 6819057449786810088;
        }
        -1 => {
            current_block_16 = 14801812128931644662;
        }
        _ => {
            current_block_16 = 15904375183555213903;
        }
    }
    match current_block_16 {
        16237308621499531990 => {
            *map
                .offset(
                    2 as libc::c_int as isize,
                ) = A_SH as libc::c_int as libc::c_uchar;
            current_block_16 = 1395401040827046706;
        }
        16827810745824149869 => {
            *map
                .offset(
                    0 as libc::c_int as isize,
                ) = A_FT as libc::c_int as libc::c_uchar;
            current_block_16 = 5190931071520054375;
        }
        _ => {}
    }
    match current_block_16 {
        1395401040827046706 => {
            *map
                .offset(
                    5 as libc::c_int as isize,
                ) = A_SH as libc::c_int as libc::c_uchar;
            current_block_16 = 492316982938702853;
        }
        5190931071520054375 => {
            *map
                .offset(
                    4 as libc::c_int as isize,
                ) = A_FT as libc::c_int as libc::c_uchar;
            current_block_16 = 1331651794105737270;
        }
        _ => {}
    }
    match current_block_16 {
        492316982938702853 => {
            *map
                .offset(
                    1 as libc::c_int as isize,
                ) = A_SH as libc::c_int as libc::c_uchar;
            current_block_16 = 12191447810917241774;
        }
        1331651794105737270 => {
            *map
                .offset(
                    1 as libc::c_int as isize,
                ) = A_FT as libc::c_int as libc::c_uchar;
            current_block_16 = 9541762626985535097;
        }
        _ => {}
    }
    match current_block_16 {
        12191447810917241774 => {
            *map
                .offset(
                    4 as libc::c_int as isize,
                ) = A_SH as libc::c_int as libc::c_uchar;
            current_block_16 = 10069085333865129812;
        }
        9541762626985535097 => {
            *map
                .offset(
                    5 as libc::c_int as isize,
                ) = A_FT as libc::c_int as libc::c_uchar;
            current_block_16 = 6819057449786810088;
        }
        _ => {}
    }
    match current_block_16 {
        10069085333865129812 => {
            *map
                .offset(
                    0 as libc::c_int as isize,
                ) = A_SH as libc::c_int as libc::c_uchar;
            current_block_16 = 2244607058334101863;
        }
        6819057449786810088 => {
            *map
                .offset(
                    2 as libc::c_int as isize,
                ) = A_FT as libc::c_int as libc::c_uchar;
            current_block_16 = 14801812128931644662;
        }
        _ => {}
    }
    match current_block_16 {
        2244607058334101863 => {
            *map
                .offset(
                    3 as libc::c_int as isize,
                ) = A_SH as libc::c_int as libc::c_uchar;
        }
        14801812128931644662 => {
            *map
                .offset(
                    6 as libc::c_int as isize,
                ) = A_FT as libc::c_int as libc::c_uchar;
        }
        _ => {}
    };
}
unsafe extern "C" fn tbl_out(
    mut s: *mut libc::c_char,
    mut x: libc::c_float,
    mut j: libc::c_int,
    mut f: *mut libc::c_char,
) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    a2b(b"(\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    p = s;
    loop {
        while *p as libc::c_int != '\0' as i32 && *p as libc::c_int != '(' as i32
            && *p as libc::c_int != ')' as i32
        {
            p = p.offset(1);
            p;
        }
        if p != s {
            a2b(
                b"%.*s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                p.offset_from(s) as libc::c_long as libc::c_int,
                s,
            );
            s = p;
        }
        if *p as libc::c_int == '\0' as i32 {
            break;
        }
        a2b(b"\\\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        p = p.offset(1);
        p;
    }
    a2b(
        b")%.1f y %d %s \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        x as libc::c_double,
        j,
        f,
    );
}
unsafe extern "C" fn draw_tblt_w(
    mut p_voice: *mut VOICE_S,
    mut nly: libc::c_int,
    mut y: libc::c_float,
    mut tblt: *mut tblt_s,
) {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut ly: *mut lyrics = 0 as *mut lyrics;
    let mut lyl: *mut lyl = 0 as *mut lyl;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut j: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    a2b(
        b"/y{%.1f yns%d}def \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        y as libc::c_double,
        (*p_voice).staff as libc::c_int,
    );
    set_font(VOCALFONT as libc::c_int);
    a2b(
        b"%.1f 0 y %d %s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        realwidth as libc::c_double,
        nly,
        (*tblt).head,
    );
    j = 0 as libc::c_int;
    while j < nly {
        s = (*p_voice).sym;
        while !s.is_null() {
            ly = (*s).ly;
            if ly.is_null()
                || {
                    lyl = (*ly).lyl[j as usize];
                    lyl.is_null()
                }
            {
                if (*s).type_0 as libc::c_int == 3 as libc::c_int {
                    if !((*tblt).bar).is_null() {
                        p = &mut *tex_buf.as_mut_ptr().offset(16 as libc::c_int as isize)
                            as *mut libc::c_char;
                        let fresh13 = p;
                        p = p.offset(-1);
                        *fresh13 = '\0' as i32 as libc::c_char;
                        l = bar_cnv((*s).u.bar.type_0);
                        while l != 0 as libc::c_int {
                            let fresh14 = p;
                            p = p.offset(-1);
                            *fresh14 = (*::core::mem::transmute::<
                                &[u8; 9],
                                &[libc::c_char; 9],
                            >(b"?|[]:???\0"))[(l & 0x7 as libc::c_int) as usize];
                            l >>= 4 as libc::c_int;
                        }
                        p = p.offset(1);
                        p;
                        tbl_out(p, (*s).x, j, (*tblt).bar);
                    }
                }
            } else {
                tbl_out(((*lyl).t).as_mut_ptr(), (*s).x, j, (*tblt).note);
            }
            s = (*s).next;
        }
        a2b(b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        j += 1;
        j;
    }
}
unsafe extern "C" fn draw_tblt_p(
    mut p_voice: *mut VOICE_S,
    mut y: libc::c_float,
    mut tblt: *mut tblt_s,
) {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut j: libc::c_int = 0;
    let mut pitch: libc::c_int = 0;
    let mut octave: libc::c_int = 0;
    let mut sf: libc::c_int = 0;
    let mut tied: libc::c_int = 0;
    let mut acc: libc::c_int = 0;
    let mut workmap: [libc::c_uchar; 70] = [0; 70];
    let mut basemap: [libc::c_uchar; 7] = [0; 7];
    static mut scale: [libc::c_int; 7] = [
        0 as libc::c_int,
        2 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        7 as libc::c_int,
        9 as libc::c_int,
        11 as libc::c_int,
    ];
    static mut acc_pitch: [libc::c_int; 6] = [
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        -(1 as libc::c_int),
        2 as libc::c_int,
        -(2 as libc::c_int),
    ];
    sf = (*p_voice).key.sf as libc::c_int;
    setmap(sf, basemap.as_mut_ptr());
    j = 0 as libc::c_int;
    while j < 10 as libc::c_int {
        memcpy(
            &mut *workmap.as_mut_ptr().offset((7 as libc::c_int * j) as isize)
                as *mut libc::c_uchar as *mut libc::c_void,
            basemap.as_mut_ptr() as *const libc::c_void,
            7 as libc::c_int as libc::c_ulong,
        );
        j += 1;
        j;
    }
    a2b(
        b"gsave 0 %.1f yns%d T(%.2s)%s\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        y as libc::c_double,
        (*p_voice).staff as libc::c_int,
        ((*tblt).instr).as_mut_ptr(),
        (*tblt).head,
    );
    tied = 0 as libc::c_int;
    s = (*p_voice).sym;
    while !s.is_null() {
        match (*s).type_0 as libc::c_int {
            1 => {
                if !((*s).abc_type as libc::c_int == 5 as libc::c_int) {
                    if tied != 0 {
                        tied = (*s).u.note.notes[0 as libc::c_int as usize].ti1
                            as libc::c_int;
                    } else {
                        pitch = (*s).u.note.notes[0 as libc::c_int as usize].pit
                            as libc::c_int + 19 as libc::c_int;
                        acc = (*s).u.note.notes[0 as libc::c_int as usize].acc
                            as libc::c_int;
                        if acc != 0 as libc::c_int {
                            workmap[pitch
                                as usize] = (if acc == A_NT as libc::c_int {
                                A_NULL as libc::c_int
                            } else {
                                acc & 0x7 as libc::c_int
                            }) as libc::c_uchar;
                        }
                        pitch = scale[(pitch % 7 as libc::c_int) as usize]
                            + acc_pitch[workmap[pitch as usize] as usize]
                            + 12 as libc::c_int * (pitch / 7 as libc::c_int)
                            - (*tblt).pitch as libc::c_int;
                        octave = 0 as libc::c_int;
                        while pitch < 0 as libc::c_int {
                            pitch += 12 as libc::c_int;
                            octave -= 1;
                            octave;
                        }
                        while pitch >= 36 as libc::c_int {
                            pitch -= 12 as libc::c_int;
                            octave += 1;
                            octave;
                        }
                        if acc & 0xf8 as libc::c_int == 0 as libc::c_int {
                            a2b(
                                b"%d %d %.2f %s\n\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                octave,
                                pitch,
                                (*s).x as libc::c_double,
                                (*tblt).note,
                            );
                        } else {
                            let mut n: libc::c_int = 0;
                            let mut d: libc::c_int = 0;
                            let mut micro_p: libc::c_float = 0.;
                            n = parse.micro_tb[(acc >> 3 as libc::c_int) as usize]
                                as libc::c_int;
                            d = (n & 0xff as libc::c_int) + 1 as libc::c_int;
                            n = (n >> 8 as libc::c_int) + 1 as libc::c_int;
                            match acc & 0x7 as libc::c_int {
                                3 | 5 => {
                                    n = -n;
                                }
                                _ => {}
                            }
                            micro_p = pitch as libc::c_float
                                + n as libc::c_float / d as libc::c_float;
                            a2b(
                                b"%d %.3f %.2f %s\n\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                octave,
                                micro_p as libc::c_double,
                                (*s).x as libc::c_double,
                                (*tblt).note,
                            );
                        }
                        tied = (*s).u.note.notes[0 as libc::c_int as usize].ti1
                            as libc::c_int;
                    }
                }
            }
            6 => {
                sf = (*s).u.key.sf as libc::c_int;
                setmap(sf, basemap.as_mut_ptr());
                j = 0 as libc::c_int;
                while j < 10 as libc::c_int {
                    memcpy(
                        &mut *workmap
                            .as_mut_ptr()
                            .offset((7 as libc::c_int * j) as isize)
                            as *mut libc::c_uchar as *mut libc::c_void,
                        basemap.as_mut_ptr() as *const libc::c_void,
                        7 as libc::c_int as libc::c_ulong,
                    );
                    j += 1;
                    j;
                }
            }
            3 => {
                if !((*s).flags as libc::c_int & 0x2 as libc::c_int != 0) {
                    j = 0 as libc::c_int;
                    while j < 10 as libc::c_int {
                        memcpy(
                            &mut *workmap
                                .as_mut_ptr()
                                .offset((7 as libc::c_int * j) as isize)
                                as *mut libc::c_uchar as *mut libc::c_void,
                            basemap.as_mut_ptr() as *const libc::c_void,
                            7 as libc::c_int as libc::c_ulong,
                        );
                        j += 1;
                        j;
                    }
                }
            }
            _ => {}
        }
        s = (*s).next;
    }
    a2b(b"grestore\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
}
unsafe extern "C" fn draw_lyric_line(mut p_voice: *mut VOICE_S, mut j: libc::c_int) {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut ly: *mut lyrics = 0 as *mut lyrics;
    let mut lyl: *mut lyl = 0 as *mut lyl;
    let mut hyflag: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut lflag: libc::c_int = 0;
    let mut ft: libc::c_int = 0;
    let mut curft: libc::c_int = 0;
    let mut defft: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lastx: libc::c_float = 0.;
    let mut w: libc::c_float = 0.;
    let mut x0: libc::c_float = 0.;
    let mut shift: libc::c_float = 0.;
    lflag = 0 as libc::c_int;
    hyflag = lflag;
    if (*p_voice).hy_st & ((1 as libc::c_int) << j) as libc::c_uint != 0 {
        hyflag = 1 as libc::c_int;
        (*p_voice).hy_st &= !((1 as libc::c_int) << j) as libc::c_uint;
    }
    s = (*p_voice).sym;
    while !((*s).type_0 as libc::c_int != 4 as libc::c_int
        && (*s).type_0 as libc::c_int != 6 as libc::c_int
        && (*s).type_0 as libc::c_int != 5 as libc::c_int)
    {
        s = (*s).next;
    }
    if !((*s).prev).is_null() {
        lastx = (*(*s).prev).x;
    } else {
        lastx = (*tsfirst).x;
    }
    x0 = 0 as libc::c_int as libc::c_float;
    while !s.is_null() {
        ly = (*s).ly;
        if ly.is_null()
            || {
                lyl = (*ly).lyl[j as usize];
                lyl.is_null()
            }
        {
            let mut current_block_18: u64;
            match (*s).type_0 as libc::c_int {
                1 => {
                    if (*s).abc_type as libc::c_int == 4 as libc::c_int {
                        current_block_18 = 14401909646449704462;
                    } else {
                        current_block_18 = 12434183049504349705;
                    }
                }
                9 => {
                    current_block_18 = 12434183049504349705;
                }
                _ => {
                    current_block_18 = 14401909646449704462;
                }
            }
            match current_block_18 {
                12434183049504349705 => {
                    if lflag != 0 {
                        putx(x0 - lastx);
                        putx(lastx + 3 as libc::c_int as libc::c_float);
                        a2b(
                            b"y wln \0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                        lflag = 0 as libc::c_int;
                        lastx = (*s).x + (*s).wr;
                    }
                }
                _ => {}
            }
        } else {
            ft = ((*lyl).f).offset_from((cfmt.font_tb).as_mut_ptr()) as libc::c_long
                as libc::c_int;
            get_str_font(&mut curft, &mut defft);
            if ft != curft {
                set_str_font(ft, defft);
            }
            p = ((*lyl).t).as_mut_ptr();
            w = (*lyl).w;
            shift = (*lyl).s;
            if hyflag != 0 {
                if *p as libc::c_int == 0x11 as libc::c_int {
                    *p = 0x10 as libc::c_int as libc::c_char;
                } else if *p as libc::c_int != 0x10 as libc::c_int {
                    putx((*s).x - shift - lastx);
                    putx(lastx);
                    a2b(
                        b"y hyph \0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    hyflag = 0 as libc::c_int;
                    lastx = (*s).x + (*s).wr;
                }
            }
            if lflag != 0 && *p as libc::c_int != 0x11 as libc::c_int {
                putx(x0 - lastx + 3 as libc::c_int as libc::c_float);
                putx(lastx + 3 as libc::c_int as libc::c_float);
                a2b(
                    b"y wln \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                lflag = 0 as libc::c_int;
                lastx = (*s).x + (*s).wr;
            }
            if *p as libc::c_int == 0x10 as libc::c_int
                || *p as libc::c_int == 0x11 as libc::c_int
            {
                if x0 == 0 as libc::c_int as libc::c_float
                    && lastx > (*s).x - 18 as libc::c_int as libc::c_float
                {
                    lastx = (*s).x - 18 as libc::c_int as libc::c_float;
                }
                if *p as libc::c_int == 0x10 as libc::c_int {
                    hyflag = 1 as libc::c_int;
                } else {
                    lflag = 1 as libc::c_int;
                }
                x0 = (*s).x - shift;
            } else {
                x0 = (*s).x - shift;
                l = (strlen(p)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as libc::c_int;
                if *p.offset(l as isize) as libc::c_int == 0x10 as libc::c_int {
                    *p.offset(l as isize) = '\0' as i32 as libc::c_char;
                    hyflag = 1 as libc::c_int;
                }
                putx(x0);
                a2b(b"y M \0" as *const u8 as *const libc::c_char as *mut libc::c_char);
                put_str(p, 3 as libc::c_int);
                lastx = x0 + w;
            }
        }
        s = (*s).next;
    }
    if hyflag != 0 {
        x0 = realwidth - 10 as libc::c_int as libc::c_float;
        if x0 < lastx + 10 as libc::c_int as libc::c_float {
            x0 = lastx + 10 as libc::c_int as libc::c_float;
        }
        putx(x0 - lastx);
        putx(lastx);
        a2b(b"y hyph \0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        if cfmt.hyphencont != 0 {
            (*p_voice).hy_st |= ((1 as libc::c_int) << j) as libc::c_uint;
        }
    }
    s = tsnext;
    while !s.is_null() {
        if (*s).voice as libc::c_long
            == p_voice.offset_from(voice_tb.as_mut_ptr()) as libc::c_long
        {
            break;
        }
        s = (*s).ts_next;
    }
    while !s.is_null() {
        if (*s).abc_type as libc::c_int == 4 as libc::c_int {
            if !((*s).ly).is_null() && !((*(*s).ly).lyl[j as usize]).is_null()
                && (*(*(*s).ly).lyl[j as usize]).t[0 as libc::c_int as usize]
                    as libc::c_int == 0x11 as libc::c_int
            {
                lflag = 1 as libc::c_int;
                x0 = realwidth - 15 as libc::c_int as libc::c_float;
                if x0 < lastx + 12 as libc::c_int as libc::c_float {
                    x0 = lastx + 12 as libc::c_int as libc::c_float;
                }
            }
            break;
        } else {
            s = (*s).next;
        }
    }
    if lflag != 0 {
        putx(x0 - lastx + 3 as libc::c_int as libc::c_float);
        putx(lastx + 3 as libc::c_int as libc::c_float);
        a2b(b"y wln\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    }
    a2b(b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
}
unsafe extern "C" fn draw_lyrics(
    mut p_voice: *mut VOICE_S,
    mut nly: libc::c_int,
    mut h: *mut libc::c_float,
    mut y: libc::c_float,
    mut incr: libc::c_int,
) -> libc::c_float {
    let mut j: libc::c_int = 0;
    let mut top: libc::c_int = 0;
    let mut sc: libc::c_float = 0.;
    if !((*p_voice).tblts[0 as libc::c_int as usize]).is_null() {
        if (*(*p_voice).tblts[0 as libc::c_int as usize]).pitch as libc::c_int
            == 0 as libc::c_int
        {
            return y;
        }
        if !((*p_voice).tblts[1 as libc::c_int as usize]).is_null()
            && (*(*p_voice).tblts[1 as libc::c_int as usize]).pitch as libc::c_int
                == 0 as libc::c_int
        {
            return y;
        }
    }
    str_font(VOCALFONT as libc::c_int);
    outft = -(1 as libc::c_int);
    sc = staff_tb[(*p_voice).staff as usize].staffscale;
    if incr > 0 as libc::c_int {
        if y > -cfmt.vocalspace {
            y = -cfmt.vocalspace;
        }
        y += *h.offset(0 as libc::c_int as isize) / 6 as libc::c_int as libc::c_float;
        y *= sc;
        j = 0 as libc::c_int;
        while j < nly {
            y = (y as libc::c_double - *h.offset(j as isize) as libc::c_double * 1.1f64)
                as libc::c_float;
            a2b(
                b"/y{%.1f yns%d}! \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                y as libc::c_double,
                (*p_voice).staff as libc::c_int,
            );
            draw_lyric_line(p_voice, j);
            j += 1;
            j;
        }
        return (y
            - *h.offset((j - 1 as libc::c_int) as isize)
                / 6 as libc::c_int as libc::c_float) / sc;
    }
    top = (staff_tb[(*p_voice).staff as usize].topbar as libc::c_int as libc::c_float
        + cfmt.vocalspace) as libc::c_int;
    if y < top as libc::c_float {
        y = top as libc::c_float;
    }
    y
        += *h.offset((nly - 1 as libc::c_int) as isize)
            / 6 as libc::c_int as libc::c_float;
    y *= sc;
    j = nly;
    loop {
        j -= 1;
        if !(j >= 0 as libc::c_int) {
            break;
        }
        a2b(
            b"/y{%.1f yns%d}! \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            y as libc::c_double,
            (*p_voice).staff as libc::c_int,
        );
        draw_lyric_line(p_voice, j);
        y = (y as libc::c_double + *h.offset(j as isize) as libc::c_double * 1.1f64)
            as libc::c_float;
    }
    return y / sc;
}
unsafe extern "C" fn draw_all_lyrics() {
    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut staff: libc::c_int = 0;
    let mut voice: libc::c_int = 0;
    let mut nly: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut lyst_tb: [C2RustUnnamed_13; 32] = [C2RustUnnamed_13 {
        a: 0,
        b: 0,
        top: 0.,
        bot: 0.,
    }; 32];
    let mut lyvo_tb: [C2RustUnnamed_12; 32] = [C2RustUnnamed_12 {
        nly: 0,
        h: [0.; 16],
    }; 32];
    let mut above_tb: [libc::c_char; 32] = [0; 32];
    let mut rv_tb: [libc::c_char; 32] = [0; 32];
    let mut top: libc::c_float = 0.;
    let mut bot: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    let mut sc: libc::c_float = 0.;
    p_voice = first_voice;
    while !p_voice.is_null() {
        if (*p_voice).have_ly() as libc::c_int != 0
            || !((*p_voice).tblts[0 as libc::c_int as usize]).is_null()
        {
            break;
        }
        p_voice = (*p_voice).next;
    }
    if p_voice.is_null() {
        return;
    }
    memset(
        above_tb.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
    );
    memset(
        lyvo_tb.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[C2RustUnnamed_12; 32]>() as libc::c_ulong,
    );
    memset(
        lyst_tb.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[C2RustUnnamed_13; 32]>() as libc::c_ulong,
    );
    staff = -(1 as libc::c_int);
    bot = 0 as libc::c_int as libc::c_float;
    top = bot;
    p_voice = first_voice;
    while !p_voice.is_null() {
        if !((*p_voice).sym).is_null() {
            voice = p_voice.offset_from(voice_tb.as_mut_ptr()) as libc::c_long
                as libc::c_int;
            if (*p_voice).staff as libc::c_int != staff {
                top = 0 as libc::c_int as libc::c_float;
                bot = 0 as libc::c_int as libc::c_float;
                staff = (*p_voice).staff as libc::c_int;
            }
            nly = -(1 as libc::c_int);
            if (*p_voice).have_ly() != 0 {
                s = (*p_voice).sym;
                while !s.is_null() {
                    let mut ly: *mut lyrics = 0 as *mut lyrics;
                    let mut x: libc::c_float = 0.;
                    let mut w: libc::c_float = 0.;
                    ly = (*s).ly;
                    if !ly.is_null() {
                        x = (*s).x;
                        if !((*ly).lyl[0 as libc::c_int as usize]).is_null() {
                            x -= (*(*ly).lyl[0 as libc::c_int as usize]).s;
                            w = (*(*ly).lyl[0 as libc::c_int as usize]).w;
                        } else {
                            w = 10 as libc::c_int as libc::c_float;
                        }
                        y = y_get(
                            (*p_voice).staff as libc::c_int,
                            1 as libc::c_int,
                            x,
                            w,
                        );
                        if top < y {
                            top = y;
                        }
                        y = y_get(
                            (*p_voice).staff as libc::c_int,
                            0 as libc::c_int,
                            x,
                            w,
                        );
                        if bot > y {
                            bot = y;
                        }
                        i = 0 as libc::c_int;
                        while i < 16 as libc::c_int {
                            if !((*ly).lyl[i as usize]).is_null() {
                                if i > nly {
                                    nly = i;
                                }
                                if lyvo_tb[voice as usize].h[i as usize]
                                    < (*(*(*ly).lyl[i as usize]).f).size
                                {
                                    lyvo_tb[voice as usize]
                                        .h[i as usize] = (*(*(*ly).lyl[i as usize]).f).size;
                                }
                            }
                            i += 1;
                            i;
                        }
                    }
                    s = (*s).next;
                }
            } else {
                y = y_get(
                    (*p_voice).staff as libc::c_int,
                    1 as libc::c_int,
                    0 as libc::c_int as libc::c_float,
                    realwidth,
                );
                if top < y {
                    top = y;
                }
                y = y_get(
                    (*p_voice).staff as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int as libc::c_float,
                    realwidth,
                );
                if bot > y {
                    bot = y;
                }
            }
            lyst_tb[staff as usize].top = top;
            lyst_tb[staff as usize].bot = bot;
            if !(nly < 0 as libc::c_int) {
                nly += 1;
                nly;
                lyvo_tb[voice as usize].nly = nly;
                if ((*p_voice).posit).voc() as libc::c_int != 0 as libc::c_int {
                    above_tb[voice
                        as usize] = (((*p_voice).posit).voc() as libc::c_int
                        == 0x1 as libc::c_int) as libc::c_int as libc::c_char;
                } else if !((*p_voice).next).is_null()
                    && (*(*p_voice).next).staff as libc::c_int == staff
                    && (*(*p_voice).next).have_ly() as libc::c_int != 0
                {
                    above_tb[voice as usize] = 1 as libc::c_int as libc::c_char;
                }
                if above_tb[voice as usize] != 0 {
                    lyst_tb[staff as usize].a = 1 as libc::c_int as libc::c_short;
                } else {
                    lyst_tb[staff as usize].b = 1 as libc::c_int as libc::c_short;
                }
            }
        }
        p_voice = (*p_voice).next;
    }
    i = 0 as libc::c_int;
    p_voice = first_voice;
    while !p_voice.is_null() {
        let mut tblt: *mut tblt_s = 0 as *mut tblt_s;
        if !((*p_voice).sym).is_null() {
            if !((*p_voice).have_ly() == 0
                && ((*p_voice).tblts[0 as libc::c_int as usize]).is_null())
            {
                voice = p_voice.offset_from(voice_tb.as_mut_ptr()) as libc::c_long
                    as libc::c_int;
                if above_tb[voice as usize] != 0 {
                    let fresh15 = i;
                    i = i + 1;
                    rv_tb[fresh15 as usize] = voice as libc::c_char;
                } else {
                    staff = (*p_voice).staff as libc::c_int;
                    if lyvo_tb[voice as usize].nly > 0 as libc::c_int {
                        lyst_tb[staff as usize]
                            .bot = draw_lyrics(
                            p_voice,
                            lyvo_tb[voice as usize].nly,
                            (lyvo_tb[voice as usize].h).as_mut_ptr(),
                            lyst_tb[staff as usize].bot,
                            1 as libc::c_int,
                        );
                    }
                    sc = staff_tb[(*p_voice).staff as usize].staffscale;
                    nly = 0 as libc::c_int;
                    while nly < 2 as libc::c_int {
                        tblt = (*p_voice).tblts[nly as usize];
                        if tblt.is_null() {
                            break;
                        }
                        if (*tblt).pitch as libc::c_int == 0 as libc::c_int {
                            draw_tblt_w(
                                p_voice,
                                lyvo_tb[voice as usize].nly,
                                lyst_tb[staff as usize].bot * sc - (*tblt).hu,
                                tblt,
                            );
                        } else {
                            draw_tblt_p(
                                p_voice,
                                lyst_tb[staff as usize].bot * sc - (*tblt).hu,
                                tblt,
                            );
                        }
                        if (*tblt).hu > 0 as libc::c_int as libc::c_float {
                            lyst_tb[staff as usize].bot -= (*tblt).hu / sc;
                            lyst_tb[staff as usize]
                                .b = 1 as libc::c_int as libc::c_short;
                        }
                        if (*tblt).ha != 0 as libc::c_int as libc::c_float {
                            lyst_tb[staff as usize].top += (*tblt).ha / sc;
                            lyst_tb[staff as usize]
                                .a = 1 as libc::c_int as libc::c_short;
                        }
                        nly += 1;
                        nly;
                    }
                }
            }
        }
        p_voice = (*p_voice).next;
    }
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int) {
            break;
        }
        voice = rv_tb[i as usize] as libc::c_int;
        p_voice = &mut *voice_tb.as_mut_ptr().offset(voice as isize) as *mut VOICE_S;
        staff = (*p_voice).staff as libc::c_int;
        lyst_tb[staff as usize]
            .top = draw_lyrics(
            p_voice,
            lyvo_tb[voice as usize].nly,
            (lyvo_tb[voice as usize].h).as_mut_ptr(),
            lyst_tb[staff as usize].top,
            -(1 as libc::c_int),
        );
    }
    p_voice = first_voice;
    while !p_voice.is_null() {
        if !((*p_voice).sym).is_null() {
            staff = (*p_voice).staff as libc::c_int;
            if lyst_tb[staff as usize].a != 0 {
                top = lyst_tb[staff as usize].top + 2 as libc::c_int as libc::c_float;
                s = (*p_voice).sym;
                while !s.is_null() {
                    if !((*s).ly).is_null() {
                        y_set(
                            staff,
                            1 as libc::c_int,
                            (*s).x - 2 as libc::c_int as libc::c_float,
                            10 as libc::c_int as libc::c_float,
                            top,
                        );
                    }
                    s = (*s).next;
                }
            }
            if lyst_tb[staff as usize].b != 0 {
                bot = lyst_tb[staff as usize].bot - 2 as libc::c_int as libc::c_float;
                if lyvo_tb[p_voice.offset_from(voice_tb.as_mut_ptr()) as libc::c_long
                        as usize]
                    .nly > 0 as libc::c_int
                {
                    s = (*p_voice).sym;
                    while !s.is_null() {
                        if !((*s).ly).is_null() {
                            y_set(
                                staff,
                                0 as libc::c_int,
                                (*s).x - 2 as libc::c_int as libc::c_float,
                                10 as libc::c_int as libc::c_float,
                                bot,
                            );
                        }
                        s = (*s).next;
                    }
                } else {
                    y_set(
                        staff,
                        0 as libc::c_int,
                        0 as libc::c_int as libc::c_float,
                        realwidth,
                        bot,
                    );
                }
            }
        }
        p_voice = (*p_voice).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn draw_sym_near() {
    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut g: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut i: libc::c_int = 0;
    let mut staff: libc::c_int = 0;
    p_voice = first_voice;
    while !p_voice.is_null() {
        let mut bm: BEAM = BEAM {
            s1: 0 as *mut SYMBOL,
            s2: 0 as *mut SYMBOL,
            a: 0.,
            b: 0.,
            nflags: 0,
        };
        let mut first_note: libc::c_int = 1 as libc::c_int;
        s = (*p_voice).sym;
        while !s.is_null() {
            g = (*s).extra;
            while !g.is_null() {
                if !((*g).type_0 as libc::c_int != 1 as libc::c_int) {
                    if (*g).sflags
                        & (0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
                        == 0x2 as libc::c_int as libc::c_uint
                    {
                        calculate_beam(&mut bm, g);
                    }
                }
                g = (*g).next;
            }
            if !((*s).abc_type as libc::c_int != 4 as libc::c_int) {
                if (*s).sflags & 0x2 as libc::c_int as libc::c_uint != 0
                    && (*s).sflags & 0x10 as libc::c_int as libc::c_uint == 0
                    || first_note != 0
                        && (*s).sflags & 0x2 as libc::c_int as libc::c_uint == 0
                {
                    first_note = 0 as libc::c_int;
                    calculate_beam(&mut bm, s);
                }
            }
            s = (*s).next;
        }
        p_voice = (*p_voice).next;
    }
    staff = 0 as libc::c_int;
    while staff <= nstaff {
        i = 0 as libc::c_int;
        while i < 128 as libc::c_int {
            staff_tb[staff as usize].top[i as usize] = 0 as libc::c_int as libc::c_float;
            staff_tb[staff as usize]
                .bot[i as usize] = 24 as libc::c_int as libc::c_float;
            i += 1;
            i;
        }
        staff += 1;
        staff;
    }
    set_tie_room();
    draw_deco_near();
    s = tsfirst;
    while !s.is_null() {
        let mut y: libc::c_int = 0;
        if !((*s).flags as libc::c_int & 0x2 as libc::c_int != 0) {
            if (*s).type_0 as libc::c_int == 11 as libc::c_int {
                g = (*s).extra;
                while !g.is_null() {
                    y_set(
                        (*s).staff as libc::c_int,
                        1 as libc::c_int,
                        (*g).x - 2 as libc::c_int as libc::c_float,
                        4 as libc::c_int as libc::c_float,
                        ((*g).ymx as libc::c_int + 1 as libc::c_int) as libc::c_float,
                    );
                    y_set(
                        (*s).staff as libc::c_int,
                        0 as libc::c_int,
                        (*g).x - 2 as libc::c_int as libc::c_float,
                        4 as libc::c_int as libc::c_float,
                        ((*g).ymn as libc::c_int - 1 as libc::c_int) as libc::c_float,
                    );
                    g = (*g).next;
                }
            } else {
                if (*s).type_0 as libc::c_int != 9 as libc::c_int {
                    y_set(
                        (*s).staff as libc::c_int,
                        1 as libc::c_int,
                        (*s).x - (*s).wl,
                        (*s).wl + (*s).wr,
                        ((*s).ymx as libc::c_int + 2 as libc::c_int) as libc::c_float,
                    );
                    y_set(
                        (*s).staff as libc::c_int,
                        0 as libc::c_int,
                        (*s).x - (*s).wl,
                        (*s).wl + (*s).wr,
                        ((*s).ymn as libc::c_int - 2 as libc::c_int) as libc::c_float,
                    );
                } else {
                    y_set(
                        (*s).staff as libc::c_int,
                        1 as libc::c_int,
                        (*s).x - 16 as libc::c_int as libc::c_float,
                        32 as libc::c_int as libc::c_float,
                        ((*s).ymx as libc::c_int + 2 as libc::c_int) as libc::c_float,
                    );
                }
                if !((*s).abc_type as libc::c_int != 4 as libc::c_int) {
                    if (*s).u.note.notes[(*s).nhd as usize].acc != 0 {
                        y = (*s).y as libc::c_int + 8 as libc::c_int;
                        if ((*s).ymx as libc::c_int) < y {
                            (*s).ymx = y as libc::c_schar;
                        }
                        y_set(
                            (*s).staff as libc::c_int,
                            1 as libc::c_int,
                            (*s).x,
                            0.0f64 as libc::c_float,
                            y as libc::c_float,
                        );
                    }
                    if (*s).u.note.notes[0 as libc::c_int as usize].acc != 0 {
                        y = (*s).y as libc::c_int;
                        if (*s).u.note.notes[0 as libc::c_int as usize].acc
                            as libc::c_int & 0x7 as libc::c_int == A_SH as libc::c_int
                            || (*s).u.note.notes[0 as libc::c_int as usize].acc
                                as libc::c_int == A_NT as libc::c_int
                        {
                            y -= 7 as libc::c_int;
                        } else {
                            y -= 5 as libc::c_int;
                        }
                        if (*s).ymn as libc::c_int > y {
                            (*s).ymn = y as libc::c_schar;
                        }
                        y_set(
                            (*s).staff as libc::c_int,
                            0 as libc::c_int,
                            (*s).x,
                            0.0f64 as libc::c_float,
                            y as libc::c_float,
                        );
                    }
                }
            }
        }
        s = (*s).ts_next;
    }
    if cfmt.measurenb >= 0 as libc::c_int {
        draw_measnb();
    }
    draw_deco_note();
    p_voice = first_voice;
    while !p_voice.is_null() {
        s = (*p_voice).sym;
        if !s.is_null() {
            set_color((*s).color);
            set_sscale((*p_voice).staff as libc::c_int);
            while !s.is_null() {
                if (*s).sflags & 0x40 as libc::c_int as libc::c_uint != 0 {
                    g = (*s).extra;
                    while !g.is_null() {
                        if (*g).type_0 as libc::c_int == 13 as libc::c_int {
                            s = draw_tuplet(g, s);
                            break;
                        } else {
                            g = (*g).next;
                        }
                    }
                }
                s = (*s).next;
            }
            draw_all_slurs(p_voice);
            s = (*p_voice).sym;
            while !s.is_null() {
                if (*s).sflags & 0x40 as libc::c_int as libc::c_uint != 0 {
                    g = (*s).extra;
                    while !g.is_null() {
                        if (*g).type_0 as libc::c_int == 13 as libc::c_int {
                            s = draw_tuplet(g, s);
                            break;
                        } else {
                            g = (*g).next;
                        }
                    }
                }
                s = (*s).next;
            }
        }
        p_voice = (*p_voice).next;
    }
    let mut top: libc::c_int = 0;
    let mut bot: libc::c_int = 0;
    staff = 0 as libc::c_int;
    while staff <= nstaff {
        top = staff_tb[staff as usize].topbar as libc::c_int + 2 as libc::c_int;
        bot = staff_tb[staff as usize].botbar as libc::c_int - 2 as libc::c_int;
        i = 0 as libc::c_int;
        while i < 128 as libc::c_int {
            if top as libc::c_float > staff_tb[staff as usize].top[i as usize] {
                staff_tb[staff as usize].top[i as usize] = top as libc::c_float;
            }
            if (bot as libc::c_float) < staff_tb[staff as usize].bot[i as usize] {
                staff_tb[staff as usize].bot[i as usize] = bot as libc::c_float;
            }
            i += 1;
            i;
        }
        staff += 1;
        staff;
    }
    set_color(0 as libc::c_int);
    draw_deco_staff();
    set_sscale(-(1 as libc::c_int));
    draw_all_lyrics();
}
unsafe extern "C" fn draw_vname(mut indent: libc::c_float) {
    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut n: libc::c_int = 0;
    let mut staff: libc::c_int = 0;
    let mut staff_d: [C2RustUnnamed_14; 32] = [C2RustUnnamed_14 {
        nl: 0,
        v: [0 as *mut libc::c_char; 8],
    }; 32];
    let mut staff_p: *mut C2RustUnnamed_14 = 0 as *mut C2RustUnnamed_14;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut y: libc::c_float = 0.;
    staff = (*cursys).nstaff as libc::c_int;
    while staff >= 0 as libc::c_int {
        if (*cursys).staff[staff as usize].empty == 0 {
            break;
        }
        staff -= 1;
        staff;
    }
    if staff < 0 as libc::c_int {
        return;
    }
    memset(
        staff_d.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[C2RustUnnamed_14; 32]>() as libc::c_ulong,
    );
    n = 0 as libc::c_int;
    p_voice = first_voice;
    while !p_voice.is_null() {
        if !((*p_voice).sym).is_null() {
            staff = (*cursys)
                .voice[p_voice.offset_from(voice_tb.as_mut_ptr()) as libc::c_long
                    as usize]
                .staff as libc::c_int;
            if !((*cursys).staff[staff as usize].empty != 0) {
                if (*p_voice).new_name() != 0 {
                    (*p_voice).set_new_name(0 as libc::c_int as libc::c_uint);
                    p = (*p_voice).nm;
                } else {
                    p = (*p_voice).snm;
                }
                if !p.is_null() {
                    if (*cursys).staff[staff as usize].flags as libc::c_int
                        & 0x200 as libc::c_int != 0
                    {
                        while (*cursys).staff[staff as usize].flags as libc::c_int
                            & 0x100 as libc::c_int == 0
                        {
                            staff -= 1;
                            staff;
                        }
                    } else if (*cursys).staff[staff as usize].flags as libc::c_int
                        & 0x2 as libc::c_int != 0
                    {
                        while (*cursys).staff[staff as usize].flags as libc::c_int
                            & 0x1 as libc::c_int == 0
                        {
                            staff -= 1;
                            staff;
                        }
                    }
                    staff_p = &mut *staff_d.as_mut_ptr().offset(staff as isize)
                        as *mut C2RustUnnamed_14;
                    loop {
                        let fresh16 = (*staff_p).nl;
                        (*staff_p).nl = (*staff_p).nl + 1;
                        (*staff_p).v[fresh16 as usize] = p;
                        p = strstr(p, b"\\n\0" as *const u8 as *const libc::c_char);
                        if p.is_null() || (*staff_p).nl >= 32 as libc::c_int {
                            break;
                        }
                        p = p.offset(2 as libc::c_int as isize);
                    }
                    n += 1;
                    n;
                }
            }
        }
        p_voice = (*p_voice).next;
    }
    if n == 0 as libc::c_int {
        return;
    }
    str_font(VOICEFONT as libc::c_int);
    indent = (-indent as libc::c_double * 0.5f64) as libc::c_float;
    staff = nstaff;
    while staff >= 0 as libc::c_int {
        staff_p = &mut *staff_d.as_mut_ptr().offset(staff as isize)
            as *mut C2RustUnnamed_14;
        if !((*staff_p).nl == 0 as libc::c_int) {
            y = (staff_tb[staff as usize].y as libc::c_double
                + staff_tb[staff as usize].topbar as libc::c_int as libc::c_double
                    * 0.5f64 * staff_tb[staff as usize].staffscale as libc::c_double
                + (9 as libc::c_int * ((*staff_p).nl - 1 as libc::c_int))
                    as libc::c_double
                - cfmt.font_tb[VOICEFONT as libc::c_int as usize].size as libc::c_double
                    * 0.3f64) as libc::c_float;
            n = staff;
            if (*cursys).staff[staff as usize].flags as libc::c_int
                & 0x100 as libc::c_int != 0
            {
                while (*cursys).staff[n as usize].flags as libc::c_int
                    & 0x200 as libc::c_int == 0
                {
                    n += 1;
                    n;
                }
            } else if (*cursys).staff[staff as usize].flags as libc::c_int
                & 0x1 as libc::c_int != 0
            {
                while (*cursys).staff[n as usize].flags as libc::c_int
                    & 0x2 as libc::c_int == 0
                {
                    n += 1;
                    n;
                }
            }
            if n != staff {
                y = (y as libc::c_double
                    - (staff_tb[staff as usize].y - staff_tb[n as usize].y)
                        as libc::c_double * 0.5f64) as libc::c_float;
            }
            n = 0 as libc::c_int;
            while n < (*staff_p).nl {
                p = (*staff_p).v[n as usize];
                q = strstr(p, b"\\n\0" as *const u8 as *const libc::c_char);
                if !q.is_null() {
                    *q = '\0' as i32 as libc::c_char;
                }
                a2b(
                    b"%.1f %.1f M \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    indent as libc::c_double,
                    y as libc::c_double,
                );
                put_str(p, 1 as libc::c_int);
                y = (y as libc::c_double - 18.0f64) as libc::c_float;
                if !q.is_null() {
                    *q = '\\' as i32 as libc::c_char;
                }
                n += 1;
                n;
            }
        }
        staff -= 1;
        staff;
    }
}
unsafe extern "C" fn set_staff() -> libc::c_float {
    let mut sy: *mut SYSTEM = 0 as *mut SYSTEM;
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut i: libc::c_int = 0;
    let mut staff: libc::c_int = 0;
    let mut prev_staff: libc::c_int = 0;
    let mut y: libc::c_float = 0.;
    let mut staffsep: libc::c_float = 0.;
    let mut dy: libc::c_float = 0.;
    let mut maxsep: libc::c_float = 0.;
    let mut mbot: libc::c_float = 0.;
    let mut v: libc::c_float = 0.;
    let mut empty: [libc::c_char; 32] = [0; 32];
    memset(
        empty.as_mut_ptr() as *mut libc::c_void,
        1 as libc::c_int,
        ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
    );
    staff = 0 as libc::c_int;
    while staff <= nstaff {
        staff_tb[staff as usize].empty = 0 as libc::c_int as libc::c_char;
        staff += 1;
        staff;
    }
    sy = cursys;
    staff = 0 as libc::c_int;
    while staff <= nstaff {
        if (*sy).staff[staff as usize].empty == 0 {
            empty[staff as usize] = 0 as libc::c_int as libc::c_char;
        }
        staff += 1;
        staff;
    }
    s = tsfirst;
    while !s.is_null() {
        if !((*s).sflags & 0x8000000 as libc::c_int as libc::c_uint == 0) {
            sy = (*sy).next;
            staff = 0 as libc::c_int;
            while staff <= nstaff {
                if (*sy).staff[staff as usize].empty == 0 {
                    empty[staff as usize] = 0 as libc::c_int as libc::c_char;
                }
                staff += 1;
                staff;
            }
        }
        s = (*s).ts_next;
    }
    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
    p_voice = first_voice;
    while !p_voice.is_null() {
        if (*p_voice).scale != 1 as libc::c_int as libc::c_float {
            a2b(
                b"/scvo%d{gsave %.2f dup scale}!\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                p_voice.offset_from(voice_tb.as_mut_ptr()) as libc::c_long
                    as libc::c_int,
                (*p_voice).scale as libc::c_double,
            );
        }
        if !((*p_voice).tblts[0 as libc::c_int as usize]).is_null() {
            empty[(*p_voice).staff as usize] = 0 as libc::c_int as libc::c_char;
        }
        p_voice = (*p_voice).next;
    }
    staff = 0 as libc::c_int;
    while staff <= nstaff {
        if empty[staff as usize] == 0 {
            break;
        }
        staff_tb[staff as usize].empty = 1 as libc::c_int as libc::c_char;
        staff += 1;
        staff;
    }
    y = 0 as libc::c_int as libc::c_float;
    if staff > nstaff {
        staff -= 1;
        staff;
    }
    i = 0 as libc::c_int;
    while i < 128 as libc::c_int {
        v = staff_tb[staff as usize].top[i as usize];
        if y < v {
            y = v;
        }
        i += 1;
        i;
    }
    y += draw_partempo(staff, y);
    y *= staff_tb[staff as usize].staffscale;
    staffsep = (cfmt.staffsep as libc::c_double * 0.5f64
        + (staff_tb[staff as usize].topbar as libc::c_int as libc::c_float
            * staff_tb[staff as usize].staffscale) as libc::c_double) as libc::c_float;
    if y < staffsep {
        y = staffsep;
    }
    staff_tb[staff as usize].y = -y;
    prev_staff = staff;
    staff += 1;
    staff;
    while staff <= nstaff {
        if empty[staff as usize] != 0 {
            staff_tb[staff as usize].empty = 1 as libc::c_int as libc::c_char;
        } else {
            if (*sy).staff[prev_staff as usize].sep != 0 as libc::c_int as libc::c_float
            {
                staffsep = (*sy).staff[prev_staff as usize].sep;
            } else {
                staffsep = cfmt.sysstaffsep;
            }
            if (*sy).staff[prev_staff as usize].maxsep
                != 0 as libc::c_int as libc::c_float
            {
                maxsep = (*sy).staff[prev_staff as usize].maxsep;
            } else {
                maxsep = cfmt.maxsysstaffsep;
            }
            dy = 0 as libc::c_int as libc::c_float;
            if staff_tb[staff as usize].staffscale
                == staff_tb[prev_staff as usize].staffscale
            {
                i = 0 as libc::c_int;
                while i < 128 as libc::c_int {
                    v = staff_tb[staff as usize].top[i as usize]
                        - staff_tb[prev_staff as usize].bot[i as usize];
                    if dy < v {
                        dy = v;
                    }
                    i += 1;
                    i;
                }
                dy *= staff_tb[staff as usize].staffscale;
            } else {
                i = 0 as libc::c_int;
                while i < 128 as libc::c_int {
                    v = staff_tb[staff as usize].top[i as usize]
                        * staff_tb[staff as usize].staffscale
                        - staff_tb[prev_staff as usize].bot[i as usize]
                            * staff_tb[prev_staff as usize].staffscale;
                    if dy < v {
                        dy = v;
                    }
                    i += 1;
                    i;
                }
            }
            staffsep
                += staff_tb[staff as usize].topbar as libc::c_int as libc::c_float
                    * staff_tb[staff as usize].staffscale;
            if dy < staffsep {
                dy = staffsep;
            }
            maxsep
                += staff_tb[staff as usize].topbar as libc::c_int as libc::c_float
                    * staff_tb[staff as usize].staffscale;
            if dy > maxsep {
                dy = maxsep;
            }
            y += dy;
            staff_tb[staff as usize].y = -y;
            prev_staff = staff;
        }
        staff += 1;
        staff;
    }
    mbot = 0 as libc::c_int as libc::c_float;
    i = 0 as libc::c_int;
    while i < 128 as libc::c_int {
        v = staff_tb[prev_staff as usize].bot[i as usize];
        if mbot > v {
            mbot = v;
        }
        i += 1;
        i;
    }
    mbot *= staff_tb[prev_staff as usize].staffscale;
    staff = nstaff;
    while staff >= 0 as libc::c_int {
        dy = staff_tb[staff as usize].y;
        if staff_tb[staff as usize].staffscale != 1 as libc::c_int as libc::c_float
            && staff_tb[staff as usize].staffscale != 0 as libc::c_int as libc::c_float
        {
            a2b(
                b"/scst%d{gsave 0 %.2f T %.2f dup scale}!\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                staff,
                dy as libc::c_double,
                staff_tb[staff as usize].staffscale as libc::c_double,
            );
            a2b(
                b"/y%d{}!\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                staff,
            );
        } else {
            a2b(
                b"/y%d{%.1f add}!\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                staff,
                dy as libc::c_double,
            );
        }
        a2b(
            b"/yns%d{%.1f add}!\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            staff,
            dy as libc::c_double,
        );
        staff -= 1;
        staff;
    }
    if mbot == 0 as libc::c_int as libc::c_float {
        staff = nstaff;
        while staff >= 0 as libc::c_int {
            if empty[staff as usize] == 0 {
                break;
            }
            staff -= 1;
            staff;
        }
        if staff < 0 as libc::c_int {
            return y;
        }
    }
    dy = -mbot;
    staffsep = (cfmt.staffsep as libc::c_double * 0.5f64) as libc::c_float;
    if dy < staffsep {
        dy = staffsep;
    }
    maxsep = (cfmt.maxstaffsep as libc::c_double * 0.5f64) as libc::c_float;
    if dy > maxsep {
        dy = maxsep;
    }
    return y + dy;
}
unsafe extern "C" fn bar_set(
    mut bar_bot: *mut libc::c_float,
    mut bar_height: *mut libc::c_float,
    mut xstaff: *mut libc::c_float,
) {
    let mut staff: libc::c_int = 0;
    let mut dy: libc::c_float = 0.;
    let mut staffscale: libc::c_float = 0.;
    let mut top: libc::c_float = 0.;
    let mut bot: libc::c_float = 0.;
    dy = 0 as libc::c_int as libc::c_float;
    staff = 0 as libc::c_int;
    while staff <= (*cursys).nstaff as libc::c_int {
        if *xstaff.offset(staff as isize) < 0 as libc::c_int as libc::c_float {
            let ref mut fresh17 = *bar_height.offset(staff as isize);
            *fresh17 = 0 as libc::c_int as libc::c_float;
            *bar_bot.offset(staff as isize) = *fresh17;
        } else {
            staffscale = staff_tb[staff as usize].staffscale;
            top = staff_tb[staff as usize].topbar as libc::c_int as libc::c_float
                * staffscale;
            bot = staff_tb[staff as usize].botbar as libc::c_int as libc::c_float
                * staffscale;
            if dy == 0 as libc::c_int as libc::c_float {
                dy = staff_tb[staff as usize].y + top;
            }
            *bar_bot.offset(staff as isize) = staff_tb[staff as usize].y + bot;
            *bar_height.offset(staff as isize) = dy - *bar_bot.offset(staff as isize);
            if (*cursys).staff[staff as usize].flags as libc::c_int & 0x40 as libc::c_int
                != 0
            {
                dy = 0 as libc::c_int as libc::c_float;
            } else {
                dy = *bar_bot.offset(staff as isize);
            }
        }
        staff += 1;
        staff;
    }
}
#[no_mangle]
pub unsafe extern "C" fn draw_systems(mut indent: libc::c_float) -> libc::c_float {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut staff: libc::c_int = 0;
    let mut bar_force: libc::c_int = 0;
    let mut xstaff: [libc::c_float; 32] = [0.; 32];
    let mut bar_bot: [libc::c_float; 32] = [0.; 32];
    let mut bar_height: [libc::c_float; 32] = [0.; 32];
    let mut staves_bar: libc::c_float = 0.;
    let mut x: libc::c_float = 0.;
    let mut x2: libc::c_float = 0.;
    let mut line_height: libc::c_float = 0.;
    line_height = set_staff();
    draw_vname(indent);
    staff = 0 as libc::c_int;
    while staff <= nstaff {
        xstaff[staff
            as usize] = (if (*cursys).staff[staff as usize].empty as libc::c_int != 0 {
            -(1 as libc::c_int)
        } else {
            0 as libc::c_int
        }) as libc::c_float;
        staff += 1;
        staff;
    }
    bar_set(bar_bot.as_mut_ptr(), bar_height.as_mut_ptr(), xstaff.as_mut_ptr());
    draw_lstaff(0 as libc::c_int as libc::c_float);
    bar_force = 0 as libc::c_int;
    s = tsfirst;
    while !s.is_null() {
        if bar_force != 0 && (*s).time != bar_force {
            bar_force = 0 as libc::c_int;
            staff = 0 as libc::c_int;
            while staff <= nstaff {
                if (*cursys).staff[staff as usize].empty != 0 {
                    xstaff[staff as usize] = -(1 as libc::c_int) as libc::c_float;
                }
                staff += 1;
                staff;
            }
            bar_set(bar_bot.as_mut_ptr(), bar_height.as_mut_ptr(), xstaff.as_mut_ptr());
        }
        if (*s).sflags & 0x8000000 as libc::c_int as libc::c_uint != 0 {
            staves_bar = 0 as libc::c_int as libc::c_float;
            s2 = s;
            while !s2.is_null() {
                if (*s2).time != (*s).time {
                    break;
                }
                if (*s2).type_0 as libc::c_int == 3 as libc::c_int {
                    staves_bar = (*s2).x;
                    break;
                } else {
                    s2 = (*s2).ts_next;
                }
            }
            cursys = (*cursys).next;
            staff = 0 as libc::c_int;
            while staff <= nstaff {
                x = xstaff[staff as usize];
                if x < 0 as libc::c_int as libc::c_float {
                    if (*cursys).staff[staff as usize].empty == 0 {
                        if staves_bar != 0 as libc::c_int as libc::c_float {
                            xstaff[staff as usize] = staves_bar;
                        } else {
                            xstaff[staff
                                as usize] = (*s).x - (*s).wl
                                - 2 as libc::c_int as libc::c_float;
                        }
                    }
                } else if !((*cursys).staff[staff as usize].empty == 0) {
                    if staves_bar != 0 as libc::c_int as libc::c_float {
                        x2 = staves_bar;
                        bar_force = (*s).time;
                    } else {
                        x2 = (*s).x - (*s).wl - 2 as libc::c_int as libc::c_float;
                        xstaff[staff as usize] = -(1 as libc::c_int) as libc::c_float;
                    }
                    draw_staff(staff, x, x2);
                }
                staff += 1;
                staff;
            }
            bar_set(bar_bot.as_mut_ptr(), bar_height.as_mut_ptr(), xstaff.as_mut_ptr());
        }
        staff = (*s).staff as libc::c_int;
        match (*s).type_0 as libc::c_int {
            3 => {
                if (*s).sflags & 0x100000 as libc::c_int as libc::c_uint != 0
                    || xstaff[staff as usize] < 0 as libc::c_int as libc::c_float
                {
                    (*s)
                        .flags = ((*s).flags as libc::c_int | 0x2 as libc::c_int)
                        as libc::c_ushort;
                }
                if !((*s).flags as libc::c_int & 0x2 as libc::c_int != 0) {
                    draw_bar(s, bar_bot[staff as usize], bar_height[staff as usize]);
                    if annotate != 0 {
                        anno_out(s, 'B' as i32 as libc::c_char);
                    }
                }
            }
            14 => {
                if (*cursys).voice[(*s).voice as usize].range as libc::c_int
                    == 0 as libc::c_int
                {
                    if (*s).xmx as libc::c_double > 0.5f64 * 37.8f64 {
                        let mut i: libc::c_int = 0;
                        let mut nvoice: libc::c_int = 0;
                        nvoice = 0 as libc::c_int;
                        i = 0 as libc::c_int;
                        while i < 32 as libc::c_int {
                            if (*cursys).voice[i as usize].range as libc::c_int
                                > 0 as libc::c_int
                            {
                                nvoice += 1;
                                nvoice;
                            }
                            i += 1;
                            i;
                        }
                        s2 = (*s).ts_next;
                        while !s2.is_null() {
                            if (*s2).type_0 as libc::c_int != 14 as libc::c_int {
                                break;
                            }
                            nvoice -= 1;
                            nvoice;
                            s2 = (*s2).ts_next;
                        }
                        if nvoice == 0 as libc::c_int {
                            draw_lstaff((*s).x);
                        }
                    }
                }
                s2 = (*s).prev;
                if !s2.is_null() {
                    if (*s2).type_0 as libc::c_int == 3 as libc::c_int {
                        x2 = (*s2).x;
                    } else {
                        x2 = (*s).x - (*s).xmx;
                    }
                    staff = (*s).staff as libc::c_int;
                    x = xstaff[staff as usize];
                    if x >= 0 as libc::c_int as libc::c_float {
                        if !(x >= x2) {
                            draw_staff(staff, x, x2);
                            xstaff[staff as usize] = (*s).x;
                        }
                    }
                }
            }
            _ => {}
        }
        s = (*s).ts_next;
    }
    staff = 0 as libc::c_int;
    while staff <= nstaff {
        if !(bar_force != 0 && (*cursys).staff[staff as usize].empty as libc::c_int != 0)
        {
            x = xstaff[staff as usize];
            if !(x < 0 as libc::c_int as libc::c_float) {
                draw_staff(staff, x, realwidth);
            }
        }
        staff += 1;
        staff;
    }
    set_sscale(-(1 as libc::c_int));
    return line_height;
}
#[no_mangle]
pub unsafe extern "C" fn output_ps(mut s: *mut SYMBOL, mut color: libc::c_int) {
    let mut g: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut g2: *mut SYMBOL = 0 as *mut SYMBOL;
    g = (*s).extra;
    g2 = 0 as *mut SYMBOL;
    let mut current_block_6: u64;
    while !g.is_null() {
        if (*g).type_0 as libc::c_int == 12 as libc::c_int {
            match (*g).aux as libc::c_int {
                0 => {
                    a2b(
                        b"%s\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        (*g).text,
                    );
                    if g2.is_null() {
                        (*s).extra = (*g).next;
                    } else {
                        (*g2).next = (*g).next;
                    }
                    current_block_6 = 8258075665625361029;
                }
                _ => {
                    current_block_6 = 13513818773234778473;
                }
            }
        } else {
            current_block_6 = 13513818773234778473;
        }
        match current_block_6 {
            13513818773234778473 => {
                g2 = g;
            }
            _ => {}
        }
        g = (*g).next;
    }
}
unsafe extern "C" fn draw_symbols(mut p_voice: *mut VOICE_S) {
    let mut bm: BEAM = BEAM {
        s1: 0 as *mut SYMBOL,
        s2: 0 as *mut SYMBOL,
        a: 0.,
        b: 0.,
        nflags: 0,
    };
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut x: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    let mut staff: libc::c_int = 0;
    let mut first_note: libc::c_int = 0;
    bm.s2 = 0 as *mut SYMBOL;
    first_note = 1 as libc::c_int;
    let mut current_block_70: u64;
    s = (*p_voice).sym;
    while !s.is_null() {
        if !((*s).extra).is_null() {
            output_ps(s, 1 as libc::c_int);
        }
        if (*s).flags as libc::c_int & 0x2 as libc::c_int != 0 {
            match (*s).type_0 as libc::c_int {
                6 => {
                    memcpy(
                        &mut (*p_voice).key as *mut key_s as *mut libc::c_void,
                        &mut (*s).u.key as *mut key_s as *const libc::c_void,
                        ::core::mem::size_of::<key_s>() as libc::c_ulong,
                    );
                    current_block_70 = 16658872821858055392;
                }
                1 | 11 => {
                    current_block_70 = 11650488183268122163;
                }
                _ => {
                    current_block_70 = 16658872821858055392;
                }
            }
        } else {
            current_block_70 = 11650488183268122163;
        }
        match current_block_70 {
            11650488183268122163 => {
                set_color((*s).color);
                x = (*s).x;
                match (*s).type_0 as libc::c_int {
                    1 => {
                        set_scale(s);
                        if (*s).abc_type as libc::c_int == 4 as libc::c_int {
                            if (*s).sflags
                                & (0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
                                == 0x2 as libc::c_int as libc::c_uint
                                || first_note != 0
                                    && (*s).sflags & 0x2 as libc::c_int as libc::c_uint == 0
                            {
                                first_note = 0 as libc::c_int;
                                if calculate_beam(&mut bm, s) != 0 {
                                    if annotate != 0 {
                                        anno_out(s, 'b' as i32 as libc::c_char);
                                    }
                                    draw_beams(&mut bm);
                                }
                            }
                            draw_note(
                                x,
                                s,
                                (bm.s2 == 0 as *mut libc::c_void as *mut SYMBOL)
                                    as libc::c_int,
                            );
                            if annotate != 0 {
                                anno_out(s, 'N' as i32 as libc::c_char);
                            }
                            if s == bm.s2 {
                                bm.s2 = 0 as *mut SYMBOL;
                            }
                            if annotate != 0
                                && (*s).sflags
                                    & (0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint
                                    == 0x10 as libc::c_int as libc::c_uint
                            {
                                anno_out(s, 'e' as i32 as libc::c_char);
                            }
                        } else {
                            draw_rest(s);
                            if annotate != 0 {
                                anno_out(s, 'R' as i32 as libc::c_char);
                            }
                        }
                    }
                    4 => {
                        staff = (*s).staff as libc::c_int;
                        if !((*s).sflags & 0x100000 as libc::c_int as libc::c_uint != 0)
                        {
                            if !((*s).flags as libc::c_int & 0x2 as libc::c_int != 0
                                || staff_tb[staff as usize].empty as libc::c_int != 0)
                            {
                                set_color(0 as libc::c_int);
                                set_sscale(staff);
                                y = staff_tb[staff as usize].y;
                                x -= 10 as libc::c_int as libc::c_float;
                                putxy(x, y + (*s).y as libc::c_int as libc::c_float);
                                if !((*s).u.clef.name).is_null() {
                                    a2b(
                                        b"%s\n\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                        (*s).u.clef.name,
                                    );
                                } else {
                                    a2b(
                                        b"%c%cclef\n\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                        if (*s).aux as libc::c_int != 0 {
                                            's' as i32
                                        } else {
                                            ' ' as i32
                                        },
                                        (*::core::mem::transmute::<
                                            &[u8; 5],
                                            &[libc::c_char; 5],
                                        >(b"tcbp\0"))[(*s).u.clef.type_0 as libc::c_uint as usize]
                                            as libc::c_int,
                                    );
                                }
                                if (*s).u.clef.octave as libc::c_int != 0 as libc::c_int {
                                    if (*s).u.clef.octave as libc::c_int > 0 as libc::c_int {
                                        y
                                            += ((*s).ymx as libc::c_int - 9 as libc::c_int)
                                                as libc::c_float;
                                    } else {
                                        y += (*s).ymn as libc::c_int as libc::c_float;
                                    }
                                    putxy(x - 2 as libc::c_int as libc::c_float, y);
                                    a2b(
                                        b"oct\n\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                    );
                                }
                                if annotate != 0 {
                                    anno_out(s, 'c' as i32 as libc::c_char);
                                }
                            }
                        }
                    }
                    5 => {
                        memcpy(
                            &mut (*p_voice).meter as *mut meter_s as *mut libc::c_void,
                            &mut (*s).u.meter as *mut meter_s as *const libc::c_void,
                            ::core::mem::size_of::<meter_s>() as libc::c_ulong,
                        );
                        if !((*s).sflags & 0x100000 as libc::c_int as libc::c_uint != 0
                            || staff_tb[(*s).staff as usize].empty as libc::c_int != 0)
                        {
                            if !(cfmt.alignbars != 0
                                && (*s).staff as libc::c_int != 0 as libc::c_int)
                            {
                                set_color(0 as libc::c_int);
                                set_sscale((*s).staff as libc::c_int);
                                draw_timesig(x, s);
                                if annotate != 0 {
                                    anno_out(s, 'M' as i32 as libc::c_char);
                                }
                            }
                        }
                    }
                    6 => {
                        memcpy(
                            &mut (*p_voice).key as *mut key_s as *mut libc::c_void,
                            &mut (*s).u.key as *mut key_s as *const libc::c_void,
                            ::core::mem::size_of::<key_s>() as libc::c_ulong,
                        );
                        if !((*s).sflags & 0x100000 as libc::c_int as libc::c_uint != 0
                            || staff_tb[(*s).staff as usize].empty as libc::c_int != 0)
                        {
                            set_color(0 as libc::c_int);
                            set_sscale((*s).staff as libc::c_int);
                            draw_keysig(p_voice, x, s);
                            if annotate != 0 {
                                anno_out(s, 'K' as i32 as libc::c_char);
                            }
                        }
                    }
                    9 => {
                        set_scale(s);
                        putxy(
                            x,
                            staff_tb[(*s).staff as usize].y
                                + 12 as libc::c_int as libc::c_float,
                        );
                        a2b(
                            b"mrest(%d)/Times-Bold 15 selectfont \0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            (*s).u.bar.len as libc::c_int,
                        );
                        putxy(
                            x,
                            staff_tb[(*s).staff as usize].y
                                + 28 as libc::c_int as libc::c_float,
                        );
                        a2b(
                            b"M showc\n\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                        if annotate != 0 {
                            anno_out(s, 'Z' as i32 as libc::c_char);
                        }
                    }
                    11 => {
                        set_scale(s);
                        draw_gracenotes(s);
                    }
                    3 | 2 | 14 | 12 => {}
                    15 => {
                        set_scale(s);
                        (*s).sflags |= 0x8 as libc::c_int as libc::c_uint;
                        draw_note(x, s, 0 as libc::c_int);
                    }
                    _ => {
                        bug(
                            b"Symbol not drawn\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            1 as libc::c_int,
                        );
                    }
                }
            }
            _ => {}
        }
        s = (*s).next;
    }
    set_scale((*p_voice).sym);
    draw_all_ties(p_voice);
    set_sscale(-(1 as libc::c_int));
    set_color(0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn draw_all_symb() {
    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    p_voice = first_voice;
    while !p_voice.is_null() {
        if !((*p_voice).sym).is_null() {
            draw_symbols(p_voice);
        }
        p_voice = (*p_voice).next;
    }
    s = tsfirst;
    while !s.is_null() {
        if (*s).type_0 as libc::c_int == 4 as libc::c_int {
            staff_tb[(*s).staff as usize].s_clef = s;
        }
        s = (*s).ts_next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn putf(mut v: libc::c_float) {
    a2b(
        b"%.1f \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        v as libc::c_double,
    );
}
#[no_mangle]
pub unsafe extern "C" fn putx(mut x: libc::c_float) {
    putf(x / cur_scale);
}
#[no_mangle]
pub unsafe extern "C" fn puty(mut y: libc::c_float) {
    putf(if scale_voice != 0 { y / cur_scale } else { y - cur_trans });
}
#[no_mangle]
pub unsafe extern "C" fn putxy(mut x: libc::c_float, mut y: libc::c_float) {
    if scale_voice != 0 {
        a2b(
            b"%.1f %.1f \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (x / cur_scale) as libc::c_double,
            (y / cur_scale) as libc::c_double,
        );
    } else {
        a2b(
            b"%.1f %.1f \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (x / cur_scale) as libc::c_double,
            (y - cur_trans) as libc::c_double,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn set_scale(mut s: *mut SYMBOL) {
    let mut scale: libc::c_float = 0.;
    let mut trans: libc::c_float = 0.;
    scale = voice_tb[(*s).voice as usize].scale;
    if scale == 1 as libc::c_int as libc::c_float {
        set_sscale((*s).staff as libc::c_int);
        return;
    }
    trans = 0 as libc::c_int as libc::c_float;
    scale_voice = 1 as libc::c_int;
    if scale == cur_scale && trans == cur_trans {
        return;
    }
    if cur_scale != 1 as libc::c_int as libc::c_float {
        a2b(b"grestore \0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    }
    cur_scale = scale;
    cur_trans = trans;
    if scale != 1 as libc::c_int as libc::c_float {
        a2b(
            b"scvo%d \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*s).voice as libc::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn set_sscale(mut staff: libc::c_int) {
    let mut scale: libc::c_float = 0.;
    let mut trans: libc::c_float = 0.;
    scale_voice = 0 as libc::c_int;
    if staff as libc::c_float != cur_staff
        && cur_scale != 1 as libc::c_int as libc::c_float
    {
        cur_scale = 0 as libc::c_int as libc::c_float;
    }
    if staff >= 0 as libc::c_int {
        scale = staff_tb[staff as usize].staffscale;
    } else {
        scale = 1 as libc::c_int as libc::c_float;
    }
    if staff >= 0 as libc::c_int && scale != 1 as libc::c_int as libc::c_float {
        trans = staff_tb[staff as usize].y;
    } else {
        trans = 0 as libc::c_int as libc::c_float;
    }
    if scale == cur_scale && trans == cur_trans {
        return;
    }
    if cur_scale != 1 as libc::c_int as libc::c_float {
        a2b(b"grestore \0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    }
    cur_scale = scale;
    cur_trans = trans;
    if scale != 1 as libc::c_int as libc::c_float {
        a2b(
            b"scst%d \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            staff,
        );
        cur_staff = staff as libc::c_float;
    }
}
unsafe extern "C" fn set_tie_dir(mut sym: *mut SYMBOL) {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut i: libc::c_int = 0;
    let mut ntie: libc::c_int = 0;
    let mut dir: libc::c_int = 0;
    let mut sec: libc::c_int = 0;
    let mut pit: libc::c_int = 0;
    let mut ti: libc::c_int = 0;
    s = sym;
    while !s.is_null() {
        if !((*s).sflags & 0x2000 as libc::c_int as libc::c_uint == 0) {
            if (*s).multi as libc::c_int != 0 as libc::c_int {
                dir = if (*s).multi as libc::c_int > 0 as libc::c_int {
                    0x1 as libc::c_int
                } else {
                    0x2 as libc::c_int
                };
                i = 0 as libc::c_int;
                while i <= (*s).nhd as libc::c_int {
                    ti = (*s).u.note.notes[i as usize].ti1 as libc::c_int;
                    if ti & 0x7 as libc::c_int == 0x4 as libc::c_int {
                        (*s)
                            .u
                            .note
                            .notes[i as usize]
                            .ti1 = (ti & 0x8 as libc::c_int | dir) as libc::c_char;
                    }
                    i += 1;
                    i;
                }
            } else {
                ntie = 0 as libc::c_int;
                sec = ntie;
                pit = 128 as libc::c_int;
                i = 0 as libc::c_int;
                while i <= (*s).nhd as libc::c_int {
                    if (*s).u.note.notes[i as usize].ti1 != 0 {
                        ntie += 1;
                        ntie;
                        if pit < 128 as libc::c_int
                            && (*s).u.note.notes[i as usize].pit as libc::c_int
                                <= pit + 1 as libc::c_int
                        {
                            sec += 1;
                            sec;
                        }
                        pit = (*s).u.note.notes[i as usize].pit as libc::c_int;
                    }
                    i += 1;
                    i;
                }
                if ntie <= 1 as libc::c_int {
                    dir = if ((*s).stem as libc::c_int) < 0 as libc::c_int {
                        0x1 as libc::c_int
                    } else {
                        0x2 as libc::c_int
                    };
                    i = 0 as libc::c_int;
                    while i <= (*s).nhd as libc::c_int {
                        ti = (*s).u.note.notes[i as usize].ti1 as libc::c_int;
                        if ti != 0 as libc::c_int {
                            if ti & 0x7 as libc::c_int == 0x4 as libc::c_int {
                                (*s)
                                    .u
                                    .note
                                    .notes[i as usize]
                                    .ti1 = (ti & 0x8 as libc::c_int | dir) as libc::c_char;
                            }
                            break;
                        } else {
                            i += 1;
                            i;
                        }
                    }
                } else if sec == 0 as libc::c_int {
                    if ntie & 1 as libc::c_int != 0 {
                        ntie /= 2 as libc::c_int;
                        dir = 0x2 as libc::c_int;
                        i = 0 as libc::c_int;
                        while i <= (*s).nhd as libc::c_int {
                            ti = (*s).u.note.notes[i as usize].ti1 as libc::c_int;
                            if !(ti == 0 as libc::c_int) {
                                if ntie == 0 as libc::c_int {
                                    if (*s).pits[i as usize] as libc::c_int >= 22 as libc::c_int
                                    {
                                        dir = 0x1 as libc::c_int;
                                    }
                                }
                                if ti & 0x7 as libc::c_int == 0x4 as libc::c_int {
                                    (*s)
                                        .u
                                        .note
                                        .notes[i as usize]
                                        .ti1 = (ti & 0x8 as libc::c_int | dir) as libc::c_char;
                                }
                                let fresh18 = ntie;
                                ntie = ntie - 1;
                                if fresh18 == 0 as libc::c_int {
                                    dir = 0x1 as libc::c_int;
                                }
                            }
                            i += 1;
                            i;
                        }
                    } else {
                        ntie /= 2 as libc::c_int;
                        dir = 0x2 as libc::c_int;
                        i = 0 as libc::c_int;
                        while i <= (*s).nhd as libc::c_int {
                            ti = (*s).u.note.notes[i as usize].ti1 as libc::c_int;
                            if !(ti == 0 as libc::c_int) {
                                if ti & 0x7 as libc::c_int == 0x4 as libc::c_int {
                                    (*s)
                                        .u
                                        .note
                                        .notes[i as usize]
                                        .ti1 = (ti & 0x8 as libc::c_int | dir) as libc::c_char;
                                }
                                ntie -= 1;
                                if ntie == 0 as libc::c_int {
                                    dir = 0x1 as libc::c_int;
                                }
                            }
                            i += 1;
                            i;
                        }
                    }
                } else {
                    pit = 128 as libc::c_int;
                    i = 0 as libc::c_int;
                    while i <= (*s).nhd as libc::c_int {
                        if (*s).u.note.notes[i as usize].ti1 != 0 {
                            if pit < 128 as libc::c_int
                                && (*s).u.note.notes[i as usize].pit as libc::c_int
                                    <= pit + 1 as libc::c_int
                            {
                                ntie = i;
                                break;
                            } else {
                                pit = (*s).u.note.notes[i as usize].pit as libc::c_int;
                            }
                        }
                        i += 1;
                        i;
                    }
                    dir = 0x2 as libc::c_int;
                    i = 0 as libc::c_int;
                    while i <= (*s).nhd as libc::c_int {
                        ti = (*s).u.note.notes[i as usize].ti1 as libc::c_int;
                        if !(ti == 0 as libc::c_int) {
                            if ntie == i {
                                dir = 0x1 as libc::c_int;
                            }
                            if ti & 0x7 as libc::c_int == 0x4 as libc::c_int {
                                (*s)
                                    .u
                                    .note
                                    .notes[i as usize]
                                    .ti1 = (ti & 0x8 as libc::c_int | dir) as libc::c_char;
                            }
                        }
                        i += 1;
                        i;
                    }
                }
            }
        }
        s = (*s).next;
    }
}
unsafe extern "C" fn set_tie_room() {
    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    p_voice = first_voice;
    while !p_voice.is_null() {
        s = (*p_voice).sym;
        if !s.is_null() {
            s = (*s).next;
            if !s.is_null() {
                set_tie_dir(s);
                let mut current_block_32: u64;
                while !s.is_null() {
                    let mut dx: libc::c_float = 0.;
                    let mut y: libc::c_float = 0.;
                    let mut dy: libc::c_float = 0.;
                    if !((*s).sflags & 0x2000 as libc::c_int as libc::c_uint == 0) {
                        if ((*s).pits[0 as libc::c_int as usize] as libc::c_int)
                            < 20 as libc::c_int
                            && (*s).u.note.notes[0 as libc::c_int as usize].ti1
                                as libc::c_int & 0x7 as libc::c_int == 0x2 as libc::c_int
                        {
                            current_block_32 = 5399440093318478209;
                        } else if (*s).pits[(*s).nhd as usize] as libc::c_int
                            > 24 as libc::c_int
                            && (*s).u.note.notes[(*s).nhd as usize].ti1 as libc::c_int
                                & 0x7 as libc::c_int == 0x1 as libc::c_int
                        {
                            current_block_32 = 5399440093318478209;
                        } else {
                            current_block_32 = 10879442775620481940;
                        }
                        match current_block_32 {
                            10879442775620481940 => {}
                            _ => {
                                s2 = (*s).next;
                                while !s2.is_null()
                                    && (*s2).abc_type as libc::c_int != 4 as libc::c_int
                                {
                                    s2 = (*s2).next;
                                }
                                if !s2.is_null() {
                                    if (*s2).staff as libc::c_int != (*s).staff as libc::c_int {
                                        current_block_32 = 10879442775620481940;
                                    } else {
                                        dx = (*s2).x - (*s).x - 10 as libc::c_int as libc::c_float;
                                        current_block_32 = 13242334135786603907;
                                    }
                                } else {
                                    dx = realwidth - (*s).x
                                        - 10 as libc::c_int as libc::c_float;
                                    current_block_32 = 13242334135786603907;
                                }
                                match current_block_32 {
                                    10879442775620481940 => {}
                                    _ => {
                                        if dx < 100 as libc::c_int as libc::c_float {
                                            dy = 9 as libc::c_int as libc::c_float;
                                        } else if dx < 300 as libc::c_int as libc::c_float {
                                            dy = 12 as libc::c_int as libc::c_float;
                                        } else {
                                            dy = 16 as libc::c_int as libc::c_float;
                                        }
                                        if (*s).pits[(*s).nhd as usize] as libc::c_int
                                            > 24 as libc::c_int
                                        {
                                            y = (3 as libc::c_int
                                                * ((*s).pits[(*s).nhd as usize] as libc::c_int
                                                    - 18 as libc::c_int)) as libc::c_float + dy;
                                            if ((*s).ymx as libc::c_int as libc::c_float) < y {
                                                (*s).ymx = y as libc::c_schar;
                                            }
                                            if !s2.is_null()
                                                && ((*s2).ymx as libc::c_int as libc::c_float) < y
                                            {
                                                (*s2).ymx = y as libc::c_schar;
                                            }
                                            y_set(
                                                (*s).staff as libc::c_int,
                                                1 as libc::c_int,
                                                (*s).x + 5 as libc::c_int as libc::c_float,
                                                dx,
                                                y,
                                            );
                                        }
                                        if ((*s).pits[0 as libc::c_int as usize] as libc::c_int)
                                            < 20 as libc::c_int
                                        {
                                            y = (3 as libc::c_int
                                                * ((*s).pits[0 as libc::c_int as usize] as libc::c_int
                                                    - 18 as libc::c_int)) as libc::c_float - dy;
                                            if (*s).ymn as libc::c_int as libc::c_float > y {
                                                (*s).ymn = y as libc::c_schar;
                                            }
                                            if !s2.is_null()
                                                && (*s2).ymn as libc::c_int as libc::c_float > y
                                            {
                                                (*s2).ymn = y as libc::c_schar;
                                            }
                                            y_set(
                                                (*s).staff as libc::c_int,
                                                0 as libc::c_int,
                                                (*s).x + 5 as libc::c_int as libc::c_float,
                                                dx,
                                                y,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                    s = (*s).next;
                }
            }
        }
        p_voice = (*p_voice).next;
    }
}
