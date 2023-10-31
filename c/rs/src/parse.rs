use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type __sFILEX;
    pub type re_guts;
    fn fclose(_: *mut FILE) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(_: *mut libc::c_char, _: libc::c_int, _: *mut FILE) -> *mut libc::c_char;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn atoi(_: *const libc::c_char) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
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
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn __maskrune(_: __darwin_ct_rune_t, _: libc::c_ulong) -> libc::c_int;
    static mut _DefaultRuneLocale: _RuneLocale;
    fn regcomp(_: *mut regex_t, _: *const libc::c_char, _: libc::c_int) -> libc::c_int;
    fn regexec(
        _: *const regex_t,
        _: *const libc::c_char,
        _: size_t,
        __pmatch: *mut regmatch_t,
        _: libc::c_int,
    ) -> libc::c_int;
    fn regfree(_: *mut regex_t);
    static mut deco: [*mut libc::c_char; 256];
    static mut parse: parse;
    static mut cfmt: FORMAT;
    static mut info: INFO;
    static mut outbuf: *mut libc::c_char;
    static mut mbf: *mut libc::c_char;
    static mut use_buffer: libc::c_int;
    static mut outft: libc::c_int;
    static mut tunenum: libc::c_int;
    static mut pagenum: libc::c_int;
    static mut in_page: libc::c_int;
    static mut secure: libc::c_int;
    static mut epsf: libc::c_int;
    static mut svg: libc::c_int;
    static mut pipeformat: libc::c_int;
    static mut in_fname: *mut libc::c_char;
    static mut file_initialized: libc::c_int;
    static mut tblts: [*mut tblt_s; 8];
    static mut cmdtblts: [cmdtblt_s; 4];
    static mut ncmdtblt: libc::c_int;
    static mut curvoice: *mut VOICE_S;
    fn clrarena(level: libc::c_int);
    fn lvlarena(level: libc::c_int) -> libc::c_int;
    fn getarena(len: libc::c_int) -> *mut libc::c_void;
    fn get_str(
        d: *mut libc::c_char,
        s: *mut libc::c_char,
        maxlen: libc::c_int,
    ) -> *mut libc::c_char;
    fn parse_acc_pit(
        p: *mut libc::c_char,
        pit: *mut libc::c_int,
        acc: *mut libc::c_int,
    ) -> *mut libc::c_char;
    fn a2b(fmt: *mut libc::c_char, _: ...);
    fn block_put();
    fn buffer_eob(eot: libc::c_int);
    fn marg_init();
    fn bskip(h: libc::c_float);
    fn close_page();
    fn get_bposy() -> libc::c_float;
    fn write_buffer();
    fn write_eps();
    fn deco_add(text: *mut libc::c_char);
    fn deco_cnv(dc: *mut decos, s: *mut SYMBOL, prev: *mut SYMBOL);
    fn reset_deco();
    fn get_textopt(p: *mut libc::c_char) -> libc::c_int;
    fn interpret_fmt_line(w: *mut libc::c_char, p: *mut libc::c_char, lock: libc::c_int);
    fn open_file(
        fn_0: *mut libc::c_char,
        ext: *mut libc::c_char,
        rfn: *mut libc::c_char,
    ) -> *mut FILE;
    fn set_font(ft: libc::c_int);
    fn set_voice_param(
        p_voice: *mut VOICE_S,
        state: libc::c_int,
        w: *mut libc::c_char,
        p: *mut libc::c_char,
    );
    fn tblt_parse(p: *mut libc::c_char) -> *mut tblt_s;
    fn glyph_add(p: *mut libc::c_char);
    fn output_music();
    fn reset_gen();
    fn bug(msg: *mut libc::c_char, fatal: libc::c_int);
    fn error(sev: libc::c_int, s: *mut SYMBOL, fmt: *mut libc::c_char, _: ...);
    fn scan_u(str: *mut libc::c_char, type_0: libc::c_int) -> libc::c_float;
    fn put_history();
    fn put_words(words: *mut SYMBOL);
    fn str_font(ft: libc::c_int);
    fn tex_str(s: *mut libc::c_char) -> libc::c_float;
    static mut tex_buf: [libc::c_char; 0];
    fn trim_title(p: *mut libc::c_char, title: *mut SYMBOL) -> *mut libc::c_char;
    fn user_ps_add(s: *mut libc::c_char, use_0: libc::c_char);
    fn write_title(s: *mut SYMBOL);
    fn write_heading();
    fn write_text(cmd: *mut libc::c_char, s: *mut libc::c_char, job: libc::c_int);
    fn svg_def_id(id: *mut libc::c_char, idsz: libc::c_int);
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
pub struct note_map {
    pub next: *mut note_map,
    pub type_0: libc::c_char,
    pub pit: libc::c_schar,
    pub acc: libc::c_uchar,
    pub heads: *mut libc::c_char,
    pub color: libc::c_int,
    pub print_pit: libc::c_schar,
    pub print_acc: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct map {
    pub next: *mut map,
    pub name: *mut libc::c_char,
    pub notes: *mut note_map,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmdtblt_s {
    pub index: libc::c_short,
    pub active: libc::c_short,
    pub vn: *mut libc::c_char,
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
pub struct brk_s {
    pub next: *mut brk_s,
    pub symsel: symsel_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct symsel_s {
    pub bar: libc::c_short,
    pub time: libc::c_short,
    pub seq: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct voice_opt_s {
    pub next: *mut voice_opt_s,
    pub s: *mut SYMBOL,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tune_opt_s {
    pub next: *mut tune_opt_s,
    pub voice_opts: *mut voice_opt_s,
    pub s: *mut SYMBOL,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub name: *mut libc::c_char,
    pub color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct staff_s {
    pub voice: libc::c_short,
    pub flags: libc::c_short,
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
pub static mut nstaff: libc::c_int = 0;
#[no_mangle]
pub static mut tsfirst: *mut SYMBOL = 0 as *const SYMBOL as *mut SYMBOL;
#[no_mangle]
pub static mut voice_tb: [VOICE_S; 32] = [VOICE_S {
    id: [0; 16],
    next: 0 as *const VOICE_S as *mut VOICE_S,
    sym: 0 as *const SYMBOL as *mut SYMBOL,
    last_sym: 0 as *const SYMBOL as *mut SYMBOL,
    lyric_start: 0 as *const SYMBOL as *mut SYMBOL,
    nm: 0 as *const libc::c_char as *mut libc::c_char,
    snm: 0 as *const libc::c_char as *mut libc::c_char,
    bar_text: 0 as *const libc::c_char as *mut libc::c_char,
    bar_gch: 0 as *const gch as *mut gch,
    tie: 0 as *const SYMBOL as *mut SYMBOL,
    rtie: 0 as *const SYMBOL as *mut SYMBOL,
    tblts: [0 as *const tblt_s as *mut tblt_s; 2],
    scale: 0.,
    time: 0,
    s_clef: 0 as *const SYMBOL as *mut SYMBOL,
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
    meter: meter_s {
        wmeasure: 0,
        nmeter: 0,
        expdur: 0,
        meter: [C2RustUnnamed_8 {
            top: [0; 8],
            bot: [0; 2],
        }; 16],
    },
    ckey: key_s {
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
    okey: key_s {
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
    hy_st: 0,
    ignore_second_floating_bar_repeat_norepbra_have_ly_new_name_space_perc_auto_len: [0; 2],
    wmeasure: 0,
    transpose: 0,
    bar_start: 0,
    posit: posit_s {
        dyn_0_gch_orn_voc_vol_std_gsd: [0; 4],
    },
    octave: 0,
    ottava: 0,
    clone: 0,
    over: 0,
    staff: 0,
    cstaff: 0,
    slur_st: 0,
    combine: 0,
    color: 0,
    stafflines: 0 as *const libc::c_char as *mut libc::c_char,
    staffscale: 0.,
    last_note: 0 as *const SYMBOL as *mut SYMBOL,
    map_name: 0 as *const libc::c_char as *mut libc::c_char,
    ulen: 0,
    microscale: 0,
    mvoice: 0,
}; 32];
#[no_mangle]
pub static mut first_voice: *mut VOICE_S = 0 as *const VOICE_S as *mut VOICE_S;
#[no_mangle]
pub static mut cursys: *mut SYSTEM = 0 as *const SYSTEM as *mut SYSTEM;
static mut parsys: *mut SYSTEM = 0 as *const SYSTEM as *mut SYSTEM;
#[no_mangle]
pub static mut dfmt: FORMAT = FORMAT {
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
pub static mut nbar: libc::c_int = 0;
#[no_mangle]
pub static mut maps: *mut map = 0 as *const map as *mut map;
static mut voice_opts: *mut voice_opt_s = 0 as *const voice_opt_s as *mut voice_opt_s;
static mut tune_voice_opts: *mut voice_opt_s = 0 as *const voice_opt_s
    as *mut voice_opt_s;
static mut tune_opts: *mut tune_opt_s = 0 as *const tune_opt_s as *mut tune_opt_s;
static mut cur_tune_opts: *mut tune_opt_s = 0 as *const tune_opt_s as *mut tune_opt_s;
static mut brks: *mut brk_s = 0 as *const brk_s as *mut brk_s;
static mut clip_start: symsel_s = symsel_s {
    bar: 0,
    time: 0,
    seq: 0,
};
static mut clip_end: symsel_s = symsel_s {
    bar: 0,
    time: 0,
    seq: 0,
};
static mut info_glob: INFO = [0 as *const SYMBOL as *mut SYMBOL; 26];
static mut deco_glob: [*mut libc::c_char; 256] = [0 as *const libc::c_char
    as *mut libc::c_char; 256];
static mut maps_glob: *mut map = 0 as *const map as *mut map;
static mut over_time: libc::c_int = 0;
static mut over_mxtime: libc::c_int = 0;
static mut over_bar: libc::c_short = 0;
static mut over_voice: libc::c_short = 0;
static mut staves_found: libc::c_int = 0;
static mut abc2win: libc::c_int = 0;
static mut capo: libc::c_int = 0;
#[no_mangle]
pub static mut multicol_start: libc::c_float = 0.;
static mut multicol_max: libc::c_float = 0.;
static mut lmarg: libc::c_float = 0.;
static mut rmarg: libc::c_float = 0.;
static mut w_tb: [libc::c_char; 16] = [
    0 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    6 as libc::c_int as libc::c_char,
    5 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
];
static mut cde2fcg: [libc::c_schar; 7] = [
    0 as libc::c_int as libc::c_schar,
    2 as libc::c_int as libc::c_schar,
    4 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    1 as libc::c_int as libc::c_schar,
    3 as libc::c_int as libc::c_schar,
    5 as libc::c_int as libc::c_schar,
];
static mut cgd2cde: [libc::c_char; 7] = [
    0 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    5 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    6 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
];
unsafe extern "C" fn sym_link(mut s: *mut SYMBOL, mut type_0: libc::c_int) {
    let mut p_voice: *mut VOICE_S = curvoice;
    if (*p_voice).ignore() == 0 {
        (*s).prev = (*p_voice).last_sym;
        if !((*s).prev).is_null() {
            (*(*p_voice).last_sym).next = s;
        } else {
            (*p_voice).sym = s;
        }
        (*p_voice).last_sym = s;
    }
    (*s).type_0 = type_0 as libc::c_uchar;
    (*s)
        .voice = p_voice.offset_from(voice_tb.as_mut_ptr()) as libc::c_long
        as libc::c_uchar;
    (*s).staff = (*p_voice).cstaff;
    (*s).time = (*p_voice).time;
    (*s).posit = (*p_voice).posit;
}
#[no_mangle]
pub unsafe extern "C" fn sym_add(
    mut p_voice: *mut VOICE_S,
    mut type_0: libc::c_int,
) -> *mut SYMBOL {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut p_voice2: *mut VOICE_S = 0 as *mut VOICE_S;
    s = getarena(::core::mem::size_of::<SYMBOL>() as libc::c_ulong as libc::c_int)
        as *mut SYMBOL;
    memset(
        s as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<SYMBOL>() as libc::c_ulong,
    );
    p_voice2 = curvoice;
    curvoice = p_voice;
    sym_link(s, type_0);
    curvoice = p_voice2;
    if (*p_voice).second() != 0 {
        (*s).sflags |= 0x100000 as libc::c_int as libc::c_uint;
    }
    if (*p_voice).floating() != 0 {
        (*s).sflags |= 0x200000 as libc::c_int as libc::c_uint;
    }
    if !((*s).prev).is_null() {
        (*s).fn_0 = (*(*s).prev).fn_0;
        (*s).linenum = (*(*s).prev).linenum;
        (*s).colnum = (*(*s).prev).colnum;
    }
    return s;
}
unsafe extern "C" fn mrest_expand(mut s: *mut SYMBOL) {
    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut next: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut dc: decos = decos {
        n: 0,
        tm: [C2RustUnnamed { t: 0, m: 0 }; 32],
    };
    let mut nb: libc::c_int = 0;
    let mut dt: libc::c_int = 0;
    nb = (*s).u.bar.len as libc::c_int;
    dt = (*s).dur / nb;
    memcpy(
        &mut dc as *mut decos as *mut libc::c_void,
        &mut (*s).u.bar.dc as *mut decos as *const libc::c_void,
        ::core::mem::size_of::<decos>() as libc::c_ulong,
    );
    memset(
        &mut (*s).u.note as *mut notes as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<notes>() as libc::c_ulong,
    );
    (*s).type_0 = 1 as libc::c_int as libc::c_uchar;
    (*s).abc_type = 5 as libc::c_int as libc::c_char;
    (*s).u.note.notes[0 as libc::c_int as usize].len = dt;
    (*s).dur = (*s).u.note.notes[0 as libc::c_int as usize].len;
    (*s).head = 0 as libc::c_int as libc::c_uchar;
    (*s).nflags = -(2 as libc::c_int) as libc::c_schar;
    next = (*s).next;
    p_voice = &mut *voice_tb.as_mut_ptr().offset((*s).voice as isize) as *mut VOICE_S;
    (*p_voice).last_sym = s;
    (*p_voice).time = (*s).time + dt;
    (*p_voice).cstaff = (*s).staff;
    s2 = s;
    loop {
        nb -= 1;
        if !(nb > 0 as libc::c_int) {
            break;
        }
        s2 = sym_add(p_voice, 3 as libc::c_int);
        (*s2).abc_type = 6 as libc::c_int as libc::c_char;
        (*s2).u.bar.type_0 = 0x1 as libc::c_int;
        s2 = sym_add(p_voice, 1 as libc::c_int);
        (*s2).abc_type = 5 as libc::c_int as libc::c_char;
        (*s2).flags = (*s).flags;
        (*s2).u.note.notes[0 as libc::c_int as usize].len = dt;
        (*s2).dur = (*s2).u.note.notes[0 as libc::c_int as usize].len;
        (*s2).head = 0 as libc::c_int as libc::c_uchar;
        (*s2).nflags = -(2 as libc::c_int) as libc::c_schar;
        (*p_voice).time += dt;
    }
    (*s2).next = next;
    if !next.is_null() {
        (*next).prev = s2;
    }
    memcpy(
        &mut (*s2).u.note.dc as *mut decos as *mut libc::c_void,
        &mut dc as *mut decos as *const libc::c_void,
        ::core::mem::size_of::<decos>() as libc::c_ulong,
    );
}
unsafe extern "C" fn sort_all() {
    let mut sy: *mut SYSTEM = 0 as *mut SYSTEM;
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut prev: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut fl: libc::c_int = 0;
    let mut voice: libc::c_int = 0;
    let mut time: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut wmin: libc::c_int = 0;
    let mut multi: libc::c_int = 0;
    let mut mrest_time: libc::c_int = 0;
    let mut nb: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut r2: libc::c_int = 0;
    let mut v2: libc::c_int = 0;
    let mut set_sy: libc::c_int = 0;
    let mut new_sy: libc::c_int = 0;
    let mut vtb: [*mut SYMBOL; 32] = [0 as *mut SYMBOL; 32];
    let mut vn: [libc::c_schar; 32] = [0; 32];
    mrest_time = -(1 as libc::c_int);
    p_voice = first_voice;
    while !p_voice.is_null() {
        vtb[p_voice.offset_from(voice_tb.as_mut_ptr()) as libc::c_long
            as usize] = (*p_voice).sym;
        p_voice = (*p_voice).next;
    }
    sy = cursys;
    set_sy = 1 as libc::c_int;
    new_sy = 0 as libc::c_int;
    prev = 0 as *mut SYMBOL;
    fl = 1 as libc::c_int;
    multi = -(1 as libc::c_int);
    loop {
        if set_sy != 0 {
            fl = 1 as libc::c_int;
            if new_sy == 0 {
                set_sy = 0 as libc::c_int;
                multi = -(1 as libc::c_int);
                memset(
                    vn.as_mut_ptr() as *mut libc::c_void,
                    -(1 as libc::c_int),
                    ::core::mem::size_of::<[libc::c_schar; 32]>() as libc::c_ulong,
                );
                p_voice = first_voice;
                while !p_voice.is_null() {
                    voice = p_voice.offset_from(voice_tb.as_mut_ptr()) as libc::c_long
                        as libc::c_int;
                    r = (*sy).voice[voice as usize].range as libc::c_int;
                    if !(r < 0 as libc::c_int) {
                        vn[r as usize] = voice as libc::c_schar;
                        multi += 1;
                        multi;
                    }
                    p_voice = (*p_voice).next;
                }
            }
        }
        time = (!(0 as libc::c_int) as libc::c_uint >> 1 as libc::c_int) as libc::c_int;
        wmin = time;
        r = 0 as libc::c_int;
        while r < 32 as libc::c_int {
            voice = vn[r as usize] as libc::c_int;
            if voice < 0 as libc::c_int {
                break;
            }
            s = vtb[voice as usize];
            if !(s.is_null() || (*s).time > time) {
                w = w_tb[(*s).type_0 as usize] as libc::c_int;
                if (*s).time < time {
                    time = (*s).time;
                    wmin = w;
                } else if w < wmin {
                    wmin = w;
                }
                if (*s).type_0 as libc::c_int == 9 as libc::c_int {
                    if (*s).u.bar.len as libc::c_int == 1 as libc::c_int {
                        mrest_expand(s);
                    } else if multi > 0 as libc::c_int {
                        mrest_time = time;
                    }
                }
            }
            r += 1;
            r;
        }
        if wmin > 127 as libc::c_int {
            break;
        }
        if time == mrest_time {
            nb = 0 as libc::c_int;
            r = 0 as libc::c_int;
            while r < 32 as libc::c_int {
                voice = vn[r as usize] as libc::c_int;
                if voice < 0 as libc::c_int {
                    break;
                }
                s = vtb[voice as usize];
                if !(s.is_null() || (*s).time != time) {
                    w = w_tb[(*s).type_0 as usize] as libc::c_int;
                    if !(w != wmin) {
                        if (*s).type_0 as libc::c_int != 9 as libc::c_int {
                            mrest_time = -(1 as libc::c_int);
                            break;
                        } else if nb == 0 as libc::c_int {
                            nb = (*s).u.bar.len as libc::c_int;
                        } else if nb != (*s).u.bar.len as libc::c_int {
                            mrest_time = -(1 as libc::c_int);
                            break;
                        }
                    }
                }
                r += 1;
                r;
            }
            if mrest_time < 0 as libc::c_int {
                r = 0 as libc::c_int;
                while r < 32 as libc::c_int {
                    voice = vn[r as usize] as libc::c_int;
                    if voice < 0 as libc::c_int {
                        break;
                    }
                    s = vtb[voice as usize];
                    if !s.is_null() && (*s).type_0 as libc::c_int == 9 as libc::c_int {
                        mrest_expand(s);
                    }
                    r += 1;
                    r;
                }
            }
        }
        r = 0 as libc::c_int;
        while r < 32 as libc::c_int {
            voice = vn[r as usize] as libc::c_int;
            if voice < 0 as libc::c_int {
                break;
            }
            s = vtb[voice as usize];
            if !(s.is_null() || (*s).time != time
                || w_tb[(*s).type_0 as usize] as libc::c_int != wmin)
            {
                if (*s).type_0 as libc::c_int == 8 as libc::c_int {
                    new_sy = 1 as libc::c_int;
                    set_sy = new_sy;
                    if !((*s).prev).is_null() {
                        (*(*s).prev).next = (*s).next;
                    } else {
                        voice_tb[voice as usize].sym = (*s).next;
                    }
                    if !((*s).next).is_null() {
                        (*(*s).next).prev = (*s).prev;
                    }
                    r2 = 0 as libc::c_int;
                    while r2 < 32 as libc::c_int {
                        if (vn[r2 as usize] as libc::c_int) < 0 as libc::c_int {
                            break;
                        }
                        r2 += 1;
                        r2;
                    }
                    v2 = 0 as libc::c_int;
                    while v2 < 32 as libc::c_int {
                        if !(((*(*sy).next).voice[v2 as usize].range as libc::c_int)
                            < 0 as libc::c_int
                            || (*sy).voice[v2 as usize].range as libc::c_int
                                >= 0 as libc::c_int)
                        {
                            let fresh0 = r2;
                            r2 = r2 + 1;
                            vn[fresh0 as usize] = v2 as libc::c_schar;
                        }
                        v2 += 1;
                        v2;
                    }
                    sy = (*sy).next;
                } else {
                    if fl != 0 {
                        fl = 0 as libc::c_int;
                        (*s).sflags |= 0x80000 as libc::c_int as libc::c_uint;
                    }
                    if new_sy != 0 {
                        new_sy = 0 as libc::c_int;
                        (*s).sflags |= 0x8000000 as libc::c_int as libc::c_uint;
                    }
                    (*s).ts_prev = prev;
                    if !prev.is_null() {
                        (*prev).ts_next = s;
                    } else {
                        tsfirst = s;
                    }
                    prev = s;
                }
                vtb[voice as usize] = (*s).next;
            }
            r += 1;
            r;
        }
        fl = wmin;
    }
    if prev.is_null() {
        return;
    }
    if (*prev).type_0 as libc::c_int != 3 as libc::c_int
        && (*prev).type_0 as libc::c_int != 12 as libc::c_int || new_sy != 0
    {
        p_voice = &mut *voice_tb.as_mut_ptr().offset((*prev).voice as isize)
            as *mut VOICE_S;
        (*p_voice).last_sym = prev;
        s = sym_add(p_voice, 12 as libc::c_int);
        (*s).aux = -(1 as libc::c_int) as libc::c_short;
        (*s).time = (*prev).time + (*prev).dur;
        (*s).sflags = 0x80000 as libc::c_int as libc::c_uint;
        if new_sy != 0 {
            (*s).sflags |= 0x8000000 as libc::c_int as libc::c_uint;
        }
        (*prev).ts_next = s;
        (*s).ts_prev = prev;
        loop {
            (*prev).sflags &= !(0x1 as libc::c_int) as libc::c_uint;
            if (*prev).sflags & 0x80000 as libc::c_int as libc::c_uint != 0 {
                break;
            }
            prev = (*prev).ts_prev;
        }
    }
    s2 = info[('Q' as i32 - 'A' as i32) as usize];
    if s2.is_null() {
        return;
    }
    info[('Q' as i32 - 'A' as i32) as usize] = 0 as *mut SYMBOL;
    s = (*tsfirst).extra;
    while !s.is_null() {
        if (*s).type_0 as libc::c_int == 7 as libc::c_int {
            return;
        }
        s = (*s).next;
    }
    s = tsfirst;
    (*s2).type_0 = 7 as libc::c_int as libc::c_uchar;
    (*s2).voice = (*s).voice;
    (*s2).staff = (*s).staff;
    (*s2).time = (*s).time;
    if !((*s).extra).is_null() {
        (*s2).next = (*s).extra;
        (*(*s2).next).prev = s2;
    }
    (*s).extra = s2;
}
unsafe extern "C" fn voice_compress() {
    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s3: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut ns: *mut SYMBOL = 0 as *mut SYMBOL;
    p_voice = first_voice;
    while !p_voice.is_null() {
        (*p_voice).set_ignore(0 as libc::c_int as libc::c_uint);
        s = (*p_voice).sym;
        while !s.is_null() {
            if (*s).time >= staves_found {
                break;
            }
            s = (*s).next;
        }
        ns = 0 as *mut SYMBOL;
        let mut current_block_59: u64;
        while !s.is_null() {
            match (*s).type_0 as libc::c_int {
                12 | 7 | 10 | 13 => {
                    if ns.is_null() {
                        ns = s;
                    }
                    current_block_59 = 14523784380283086299;
                }
                9 => {
                    if ns.is_null() {
                        current_block_59 = 14523784380283086299;
                    } else {
                        s2 = getarena(
                            ::core::mem::size_of::<SYMBOL>() as libc::c_ulong
                                as libc::c_int,
                        ) as *mut SYMBOL;
                        memset(
                            s2 as *mut libc::c_void,
                            0 as libc::c_int,
                            ::core::mem::size_of::<SYMBOL>() as libc::c_ulong,
                        );
                        (*s2).type_0 = 2 as libc::c_int as libc::c_uchar;
                        (*s2)
                            .u
                            .note
                            .notes[1 as libc::c_int as usize]
                            .len = -(1 as libc::c_int);
                        (*s2).flags = 0x2 as libc::c_int as libc::c_ushort;
                        (*s2).voice = (*s).voice;
                        (*s2).staff = (*s).staff;
                        (*s2).time = (*s).time;
                        (*s2).sflags = (*s).sflags;
                        (*s2).next = s;
                        (*s2).prev = (*s).prev;
                        (*(*s2).prev).next = s2;
                        (*s).prev = s2;
                        s = s2;
                        current_block_59 = 11307063007268554308;
                    }
                }
                _ => {
                    current_block_59 = 11307063007268554308;
                }
            }
            match current_block_59 {
                11307063007268554308 => {
                    if (*s).flags as libc::c_int & 0x20 as libc::c_int != 0 {
                        if ns.is_null() {
                            ns = s;
                        }
                        while (*s).flags as libc::c_int & 0x40 as libc::c_int == 0 {
                            s = (*s).next;
                        }
                        s2 = getarena(
                            ::core::mem::size_of::<SYMBOL>() as libc::c_ulong
                                as libc::c_int,
                        ) as *mut SYMBOL;
                        memcpy(
                            s2 as *mut libc::c_void,
                            s as *const libc::c_void,
                            ::core::mem::size_of::<SYMBOL>() as libc::c_ulong,
                        );
                        (*s2).abc_type = 0 as libc::c_int as libc::c_char;
                        (*s2).type_0 = 11 as libc::c_int as libc::c_uchar;
                        (*s2).dur = 0 as libc::c_int;
                        (*s2).next = (*s).next;
                        if !((*s2).next).is_null() {
                            (*(*s2).next).prev = s2;
                            if cfmt.graceword != 0 {
                                s3 = (*s2).next;
                                while !s3.is_null() {
                                    match (*s3).type_0 as libc::c_int {
                                        2 => {
                                            s3 = (*s3).next;
                                        }
                                        1 => {
                                            (*s2).ly = (*s3).ly;
                                            (*s3).ly = 0 as *mut lyrics;
                                            break;
                                        }
                                        _ => {
                                            break;
                                        }
                                    }
                                }
                            }
                        } else {
                            (*p_voice).last_sym = s2;
                        }
                        (*s2).prev = s;
                        (*s).next = s2;
                        s = s2;
                        if !((*s).next).is_null()
                            && (*(*s).next).type_0 as libc::c_int == 3 as libc::c_int
                        {
                            (*s).time -= 1;
                            (*s).time;
                        }
                    }
                    if !ns.is_null() {
                        s2 = (*s).extra;
                        (*s).extra = ns;
                        (*(*s).prev).next = s2;
                        if !s2.is_null() {
                            (*s2).prev = (*s).prev;
                        }
                        (*s).prev = (*ns).prev;
                        if !((*s).prev).is_null() {
                            (*(*s).prev).next = s;
                        } else {
                            (*p_voice).sym = s;
                        }
                        (*ns).prev = 0 as *mut SYMBOL;
                        ns = 0 as *mut SYMBOL;
                    }
                }
                _ => {}
            }
            s = (*s).next;
        }
        if !ns.is_null() {
            s = sym_add(p_voice, 12 as libc::c_int);
            (*s).aux = -(1 as libc::c_int) as libc::c_short;
            (*s).extra = ns;
            (*(*s).prev).next = 0 as *mut SYMBOL;
            (*s).prev = (*ns).prev;
            if !((*s).prev).is_null() {
                (*(*s).prev).next = s;
            } else {
                (*p_voice).sym = s;
            }
            (*ns).prev = 0 as *mut SYMBOL;
        }
        p_voice = (*p_voice).next;
    }
}
unsafe extern "C" fn voice_dup() {
    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut p_voice2: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut g: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut g2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut voice: libc::c_int = 0;
    p_voice = first_voice;
    while !p_voice.is_null() {
        voice = (*p_voice).clone as libc::c_int;
        if !(voice < 0 as libc::c_int) {
            (*p_voice).clone = -(1 as libc::c_int) as libc::c_schar;
            p_voice2 = &mut *voice_tb.as_mut_ptr().offset(voice as isize)
                as *mut VOICE_S;
            s = (*p_voice).sym;
            while !s.is_null() {
                if (*s).time >= staves_found {
                    break;
                }
                s = (*s).next;
            }
            while !s.is_null() {
                if !((*s).type_0 as libc::c_int == 8 as libc::c_int) {
                    s2 = getarena(
                        ::core::mem::size_of::<SYMBOL>() as libc::c_ulong as libc::c_int,
                    ) as *mut SYMBOL;
                    memcpy(
                        s2 as *mut libc::c_void,
                        s as *const libc::c_void,
                        ::core::mem::size_of::<SYMBOL>() as libc::c_ulong,
                    );
                    (*s2).prev = (*p_voice2).last_sym;
                    (*s2).next = 0 as *mut SYMBOL;
                    if !((*p_voice2).sym).is_null() {
                        (*(*p_voice2).last_sym).next = s2;
                    } else {
                        (*p_voice2).sym = s2;
                    }
                    (*p_voice2).last_sym = s2;
                    (*s2).voice = voice as libc::c_uchar;
                    (*s2).staff = (*p_voice2).staff;
                    if (*p_voice2).second() != 0 {
                        (*s2).sflags |= 0x100000 as libc::c_int as libc::c_uint;
                    } else {
                        (*s2).sflags &= !(0x100000 as libc::c_int) as libc::c_uint;
                    }
                    if (*p_voice2).floating() != 0 {
                        (*s2).sflags |= 0x200000 as libc::c_int as libc::c_uint;
                    } else {
                        (*s2).sflags &= !(0x200000 as libc::c_int) as libc::c_uint;
                    }
                    (*s2).ly = 0 as *mut lyrics;
                    g = (*s2).extra;
                    if !g.is_null() {
                        g2 = getarena(
                            ::core::mem::size_of::<SYMBOL>() as libc::c_ulong
                                as libc::c_int,
                        ) as *mut SYMBOL;
                        memcpy(
                            g2 as *mut libc::c_void,
                            g as *const libc::c_void,
                            ::core::mem::size_of::<SYMBOL>() as libc::c_ulong,
                        );
                        (*s2).extra = g2;
                        s2 = g2;
                        (*s2).voice = voice as libc::c_uchar;
                        (*s2).staff = (*p_voice2).staff;
                        g = (*g).next;
                        while !g.is_null() {
                            g2 = getarena(
                                ::core::mem::size_of::<SYMBOL>() as libc::c_ulong
                                    as libc::c_int,
                            ) as *mut SYMBOL;
                            memcpy(
                                g2 as *mut libc::c_void,
                                g as *const libc::c_void,
                                ::core::mem::size_of::<SYMBOL>() as libc::c_ulong,
                            );
                            (*s2).next = g2;
                            (*g2).prev = s2;
                            s2 = g2;
                            (*s2).voice = voice as libc::c_uchar;
                            (*s2).staff = (*p_voice2).staff;
                            g = (*g).next;
                        }
                    }
                }
                s = (*s).next;
            }
        }
        p_voice = (*p_voice).next;
    }
}
unsafe extern "C" fn system_new() {
    let mut new_sy: *mut SYSTEM = 0 as *mut SYSTEM;
    let mut staff: libc::c_int = 0;
    let mut voice: libc::c_int = 0;
    new_sy = getarena(::core::mem::size_of::<SYSTEM>() as libc::c_ulong as libc::c_int)
        as *mut SYSTEM;
    if parsys.is_null() {
        memset(
            new_sy as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<SYSTEM>() as libc::c_ulong,
        );
        voice = 0 as libc::c_int;
        while voice < 32 as libc::c_int {
            (*new_sy).voice[voice as usize].range = -(1 as libc::c_int) as libc::c_schar;
            voice += 1;
            voice;
        }
        staff = 0 as libc::c_int;
        while staff < 32 as libc::c_int {
            (*new_sy)
                .staff[staff as usize]
                .stafflines = b"|||||\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            (*new_sy)
                .staff[staff as usize]
                .staffscale = 1 as libc::c_int as libc::c_float;
            staff += 1;
            staff;
        }
        cursys = new_sy;
    } else {
        voice = 0 as libc::c_int;
        while voice < 32 as libc::c_int {
            staff = (*parsys).voice[voice as usize].staff as libc::c_int;
            if !(voice_tb[voice as usize].stafflines).is_null() {
                (*parsys)
                    .staff[staff as usize]
                    .stafflines = voice_tb[voice as usize].stafflines;
            }
            if voice_tb[voice as usize].staffscale != 0 as libc::c_int as libc::c_float {
                (*parsys)
                    .staff[staff as usize]
                    .staffscale = voice_tb[voice as usize].staffscale;
            }
            voice += 1;
            voice;
        }
        memcpy(
            new_sy as *mut libc::c_void,
            parsys as *const libc::c_void,
            ::core::mem::size_of::<SYSTEM>() as libc::c_ulong,
        );
        voice = 0 as libc::c_int;
        while voice < 32 as libc::c_int {
            (*new_sy).voice[voice as usize].range = -(1 as libc::c_int) as libc::c_schar;
            (*new_sy).voice[voice as usize].second = 0 as libc::c_int as libc::c_char;
            voice += 1;
            voice;
        }
        staff = 0 as libc::c_int;
        while staff < 32 as libc::c_int {
            (*new_sy).staff[staff as usize].flags = 0 as libc::c_int as libc::c_short;
            staff += 1;
            staff;
        }
        (*parsys).next = new_sy;
    }
    parsys = new_sy;
}
unsafe extern "C" fn system_init() {
    voice_compress();
    voice_dup();
    sort_all();
}
unsafe extern "C" fn go_global_time(
    mut s: *mut SYMBOL,
    mut symsel: *mut symsel_s,
) -> *mut SYMBOL {
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut bar_time: libc::c_int = 0;
    if (*symsel).bar as libc::c_int <= 1 as libc::c_int {
        if !((*symsel).bar as libc::c_int == 0 as libc::c_int) {
            s2 = s;
            while !s2.is_null() {
                if (*s2).type_0 as libc::c_int == 3 as libc::c_int
                    && (*s2).time != 0 as libc::c_int
                {
                    break;
                }
                s2 = (*s2).ts_next;
            }
            if s2.is_null() {
                return s2;
            }
            if (*s2).time
                < voice_tb[(*cursys).top_voice as usize].meter.wmeasure as libc::c_int
            {
                s = s2;
            }
        }
    } else {
        while !s.is_null() {
            if (*s).type_0 as libc::c_int == 3 as libc::c_int
                && (*s).aux as libc::c_int >= (*symsel).bar as libc::c_int
            {
                break;
            }
            s = (*s).ts_next;
        }
        if s.is_null() {
            return 0 as *mut SYMBOL;
        }
        if (*symsel).seq as libc::c_int != 0 as libc::c_int {
            let mut seq: libc::c_int = 0;
            seq = (*symsel).seq as libc::c_int;
            s = (*s).ts_next;
            while !s.is_null() {
                if (*s).type_0 as libc::c_int == 3 as libc::c_int
                    && (*s).aux as libc::c_int == (*symsel).bar as libc::c_int
                {
                    seq -= 1;
                    if seq == 0 as libc::c_int {
                        break;
                    }
                }
                s = (*s).ts_next;
            }
            if s.is_null() {
                return 0 as *mut SYMBOL;
            }
        }
    }
    if (*symsel).time as libc::c_int == 0 as libc::c_int {
        return s;
    }
    bar_time = (*s).time + (*symsel).time as libc::c_int;
    while (*s).time < bar_time {
        s = (*s).ts_next;
        if s.is_null() {
            return s;
        }
    }
    loop {
        s = (*s).ts_prev;
        if !((*s).sflags & 0x80000 as libc::c_int as libc::c_uint == 0) {
            break;
        }
    }
    return s;
}
unsafe extern "C" fn do_clip() {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut sy: *mut SYSTEM = 0 as *mut SYSTEM;
    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut voice: libc::c_int = 0;
    s = tsfirst;
    if clip_start.bar as libc::c_int > 0 as libc::c_int
        || clip_start.time as libc::c_int > 0 as libc::c_int
    {
        s = go_global_time(s, &mut clip_start);
        if s.is_null() {
            tsfirst = 0 as *mut SYMBOL;
            return;
        }
        sy = cursys;
        s2 = tsfirst;
        while s2 != s {
            if (*s).sflags & 0x8000000 as libc::c_int as libc::c_uint != 0 {
                sy = (*sy).next;
            }
            match (*s2).type_0 as libc::c_int {
                4 => {
                    voice_tb[(*s2).voice as usize].s_clef = s2;
                }
                6 => {
                    memcpy(
                        &mut (*voice_tb.as_mut_ptr().offset((*s2).voice as isize)).key
                            as *mut key_s as *mut libc::c_void,
                        &mut (*s2).u.key as *mut key_s as *const libc::c_void,
                        ::core::mem::size_of::<key_s>() as libc::c_ulong,
                    );
                }
                5 => {
                    memcpy(
                        &mut (*voice_tb.as_mut_ptr().offset((*s2).voice as isize)).meter
                            as *mut meter_s as *mut libc::c_void,
                        &mut (*s2).u.meter as *mut meter_s as *const libc::c_void,
                        ::core::mem::size_of::<meter_s>() as libc::c_ulong,
                    );
                }
                _ => {}
            }
            s2 = (*s2).ts_next;
        }
        cursys = sy;
        p_voice = first_voice;
        while !p_voice.is_null() {
            voice = p_voice.offset_from(voice_tb.as_mut_ptr()) as libc::c_long
                as libc::c_int;
            s2 = s;
            while !s2.is_null() {
                if (*s2).voice as libc::c_int == voice {
                    (*s2).prev = 0 as *mut SYMBOL;
                    break;
                } else {
                    s2 = (*s2).ts_next;
                }
            }
            (*p_voice).sym = s2;
            p_voice = (*p_voice).next;
        }
        tsfirst = s;
        (*s).ts_prev = 0 as *mut SYMBOL;
    }
    s = go_global_time(s, &mut clip_end);
    if s.is_null() {
        return;
    }
    loop {
        s = (*s).ts_next;
        if s.is_null() {
            return;
        }
        if !((*s).sflags & 0x80000 as libc::c_int as libc::c_uint == 0) {
            break;
        }
    }
    p_voice = first_voice;
    while !p_voice.is_null() {
        voice = p_voice.offset_from(voice_tb.as_mut_ptr()) as libc::c_long
            as libc::c_int;
        s2 = (*s).ts_prev;
        while !s2.is_null() {
            if (*s2).voice as libc::c_int == voice {
                (*s2).next = 0 as *mut SYMBOL;
                break;
            } else {
                s2 = (*s2).ts_prev;
            }
        }
        if s2.is_null() {
            (*p_voice).sym = 0 as *mut SYMBOL;
        }
        p_voice = (*p_voice).next;
    }
    (*(*s).ts_prev).ts_next = 0 as *mut SYMBOL;
}
unsafe extern "C" fn set_bar_num() {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s3: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut bar_time: libc::c_int = 0;
    let mut wmeasure: libc::c_int = 0;
    let mut tim: libc::c_int = 0;
    let mut bar_num: libc::c_int = 0;
    let mut bar_rep: libc::c_int = 0;
    wmeasure = voice_tb[(*cursys).top_voice as usize].meter.wmeasure as libc::c_int;
    bar_rep = nbar;
    s = tsfirst;
    loop {
        if s.is_null() {
            return;
        }
        match (*s).type_0 as libc::c_int {
            5 | 4 | 6 | 12 | 14 => {
                s = (*s).ts_next;
            }
            3 => {
                if (*s).aux != 0 {
                    nbar = (*s).aux as libc::c_int;
                    break;
                } else {
                    if (*s).u.bar.repeat_bar as libc::c_int != 0
                        && !((*s).text).is_null() && cfmt.contbarnb == 0
                    {
                        if *((*s).text).offset(0 as libc::c_int as isize) as libc::c_int
                            == '1' as i32
                        {
                            bar_rep = nbar;
                        } else {
                            nbar = bar_rep;
                            (*s).aux = nbar as libc::c_short;
                        }
                    }
                    break;
                }
            }
            _ => {
                break;
            }
        }
    }
    bar_time = (*s).time + wmeasure;
    bar_num = nbar;
    while !s.is_null() {
        match (*s).type_0 as libc::c_int {
            4 => {
                if !((*s).sflags & 0x8000000 as libc::c_int as libc::c_uint != 0) {
                    s2 = (*s).ts_prev;
                    while !s2.is_null() {
                        if (*s2).sflags & 0x8000000 as libc::c_int as libc::c_uint != 0 {
                            s2 = 0 as *mut SYMBOL;
                            break;
                        } else {
                            match (*s2).type_0 as libc::c_int {
                                3 => {
                                    if (*s2).sflags & 0x80000 as libc::c_int as libc::c_uint
                                        != 0
                                    {
                                        break;
                                    }
                                }
                                9 | 1 | 2 | 14 | 13 => {
                                    s2 = 0 as *mut SYMBOL;
                                    break;
                                }
                                _ => {}
                            }
                            s2 = (*s2).ts_prev;
                        }
                    }
                    if !s2.is_null() {
                        if !((*s).next).is_null() {
                            (*(*s).next).prev = (*s).prev;
                            (*(*s).prev).next = (*s).next;
                            (*(*s).ts_next).ts_prev = (*s).ts_prev;
                            (*(*s).ts_prev).ts_next = (*s).ts_next;
                            (*s).next = s2;
                            (*s).prev = (*s2).prev;
                            if !((*s).prev).is_null() {
                                (*(*s).prev).next = s;
                            }
                            (*s2).prev = s;
                            (*s).ts_next = s2;
                            (*s).ts_prev = (*s2).ts_prev;
                            if !((*s).ts_prev).is_null() {
                                (*(*s).ts_prev).ts_next = s;
                            }
                            (*s2).ts_prev = s;
                            s3 = (*s).extra;
                            if !s3.is_null() {
                                if !((*(*s).ts_next).extra).is_null() {
                                    while !((*s3).next).is_null() {
                                        s3 = (*s3).next;
                                    }
                                    (*s3).next = (*(*s).ts_next).extra;
                                    (*(*s).ts_next).extra = (*s).extra;
                                } else {
                                    (*(*s).ts_next).extra = s3;
                                }
                                (*s).extra = 0 as *mut SYMBOL;
                            }
                            s = s2;
                        }
                    }
                }
            }
            5 => {
                wmeasure = (*s).u.meter.wmeasure as libc::c_int;
                if (*s).time < bar_time {
                    bar_time = (*s).time + wmeasure;
                }
            }
            9 => {
                bar_num += (*s).u.bar.len as libc::c_int - 1 as libc::c_int;
                while !((*s).ts_next).is_null()
                    && (*(*s).ts_next).type_0 as libc::c_int != 3 as libc::c_int
                {
                    s = (*s).ts_next;
                }
            }
            3 => {
                if (*s).aux != 0 {
                    bar_num = (*s).aux as libc::c_int;
                } else if !((*s).time < bar_time) {
                    bar_num += 1;
                    bar_num;
                    tim = (*s).time;
                    s2 = s;
                    loop {
                        if (*s2).type_0 as libc::c_int == 3 as libc::c_int
                            && (*s2).u.bar.repeat_bar as libc::c_int != 0
                            && !((*s2).text).is_null() && cfmt.contbarnb == 0
                        {
                            if *((*s2).text).offset(0 as libc::c_int as isize)
                                as libc::c_int == '1' as i32
                            {
                                bar_rep = bar_num;
                            } else {
                                bar_num = bar_rep;
                            }
                            break;
                        } else {
                            s2 = (*s2).next;
                            if !(!s2.is_null() && (*s2).time == tim) {
                                break;
                            }
                        }
                    }
                    (*s).aux = bar_num as libc::c_short;
                    bar_time = (*s).time + wmeasure;
                }
            }
            _ => {}
        }
        s = (*s).ts_next;
    }
    if clip_start.bar as libc::c_int >= 0 as libc::c_int {
        if bar_num <= clip_start.bar as libc::c_int || nbar > clip_end.bar as libc::c_int
        {
            tsfirst = 0 as *mut SYMBOL;
            return;
        }
        do_clip();
    }
    let mut brk: *mut brk_s = 0 as *mut brk_s;
    let mut nbar_min: libc::c_int = 0;
    nbar_min = nbar;
    if nbar_min == 1 as libc::c_int {
        nbar_min = -(1 as libc::c_int);
    }
    brk = brks;
    while !brk.is_null() {
        if !((*brk).symsel.bar as libc::c_int <= nbar_min
            || (*brk).symsel.bar as libc::c_int > bar_num)
        {
            s = go_global_time(tsfirst, &mut (*brk).symsel);
            if !s.is_null() {
                (*s).sflags |= 0x1 as libc::c_int as libc::c_uint;
            }
        }
        brk = (*brk).next;
    }
    if cfmt.measurenb < 0 as libc::c_int {
        nbar = bar_num;
    }
}
unsafe extern "C" fn generate() {
    let mut old_lvl: libc::c_int = 0;
    let mut voice: libc::c_int = 0;
    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
    system_init();
    if tsfirst.is_null() {
        return;
    }
    set_bar_num();
    if tsfirst.is_null() {
        return;
    }
    old_lvl = lvlarena(2 as libc::c_int);
    output_music();
    clrarena(2 as libc::c_int);
    lvlarena(old_lvl);
    p_voice = first_voice;
    while !p_voice.is_null() {
        voice = p_voice.offset_from(voice_tb.as_mut_ptr()) as libc::c_long
            as libc::c_int;
        (*p_voice).last_sym = 0 as *mut SYMBOL;
        (*p_voice).sym = (*p_voice).last_sym;
        (*p_voice).time = 0 as libc::c_int;
        (*p_voice).set_have_ly(0 as libc::c_int as libc::c_uint);
        (*p_voice).staff = (*cursys).voice[voice as usize].staff;
        (*p_voice).set_second((*cursys).voice[voice as usize].second as libc::c_uint);
        (*(*p_voice).s_clef).time = 0 as libc::c_int;
        (*p_voice).lyric_start = 0 as *mut SYMBOL;
        p_voice = (*p_voice).next;
    }
    staves_found = 0 as libc::c_int;
}
unsafe extern "C" fn gen_ly(mut eob: libc::c_int) {
    generate();
    if !(info[('W' as i32 - 'A' as i32) as usize]).is_null() {
        put_words(info[('W' as i32 - 'A' as i32) as usize]);
        info[('W' as i32 - 'A' as i32) as usize] = 0 as *mut SYMBOL;
    }
    if eob != 0 {
        buffer_eob(0 as libc::c_int);
    }
}
unsafe extern "C" fn acc_same_pitch(mut pitch: libc::c_int) -> libc::c_int {
    let mut s: *mut SYMBOL = (*(*curvoice).last_sym).prev;
    let mut i: libc::c_int = 0;
    let mut time: libc::c_int = 0;
    if s.is_null() {
        return -(1 as libc::c_int);
    }
    time = (*s).time;
    while !s.is_null() {
        match (*s).abc_type as libc::c_int {
            6 => {
                if (*s).time < time {
                    return -(1 as libc::c_int);
                }
                loop {
                    s = (*s).prev;
                    if s.is_null() {
                        return -(1 as libc::c_int);
                    }
                    if (*s).abc_type as libc::c_int == 4 as libc::c_int {
                        if (*s).time + (*s).dur == time {
                            break;
                        }
                        return -(1 as libc::c_int);
                    } else if (*s).time < time {
                        return -(1 as libc::c_int)
                    }
                }
                i = 0 as libc::c_int;
                while i <= (*s).nhd as libc::c_int {
                    if (*s).u.note.notes[i as usize].pit as libc::c_int == pitch
                        && (*s).u.note.notes[i as usize].ti1 as libc::c_int != 0
                    {
                        return (*s).u.note.notes[i as usize].acc as libc::c_int;
                    }
                    i += 1;
                    i;
                }
                return -(1 as libc::c_int);
            }
            4 => {
                i = 0 as libc::c_int;
                while i <= (*s).nhd as libc::c_int {
                    if (*s).u.note.notes[i as usize].pit as libc::c_int == pitch {
                        return (*s).u.note.notes[i as usize].acc as libc::c_int;
                    }
                    i += 1;
                    i;
                }
            }
            _ => {}
        }
        s = (*s).prev;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn note_transpose(mut s: *mut SYMBOL) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut a: libc::c_int = 0;
    let mut dp: libc::c_int = 0;
    let mut i1: libc::c_int = 0;
    let mut i2: libc::c_int = 0;
    let mut i3: libc::c_int = 0;
    let mut i4: libc::c_int = 0;
    let mut sf_old: libc::c_int = 0;
    static mut acc1: [libc::c_schar; 6] = [
        0 as libc::c_int as libc::c_schar,
        1 as libc::c_int as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        2 as libc::c_int as libc::c_schar,
        -(2 as libc::c_int) as libc::c_schar,
    ];
    static mut acc2: [libc::c_char; 5] = [
        A_DF as libc::c_int as libc::c_char,
        A_FT as libc::c_int as libc::c_char,
        A_NT as libc::c_int as libc::c_char,
        A_SH as libc::c_int as libc::c_char,
        A_DS as libc::c_int as libc::c_char,
    ];
    m = (*s).nhd as libc::c_int;
    sf_old = (*curvoice).okey.sf as libc::c_int;
    i2 = (*curvoice).ckey.sf as libc::c_int - sf_old;
    dp = cgd2cde[((i2 + 4 as libc::c_int * 7 as libc::c_int) % 7 as libc::c_int)
        as usize] as libc::c_int;
    if ((*curvoice).transpose as libc::c_int) < 0 as libc::c_int
        && dp != 0 as libc::c_int
    {
        dp -= 7 as libc::c_int;
    }
    dp
        += (*curvoice).transpose as libc::c_int / 3 as libc::c_int / 12 as libc::c_int
            * 7 as libc::c_int;
    let mut current_block_84: u64;
    i = 0 as libc::c_int;
    while i <= m {
        n = (*s).u.note.notes[i as usize].pit as libc::c_int;
        (*s)
            .u
            .note
            .notes[i as usize]
            .pit = ((*s).u.note.notes[i as usize].pit as libc::c_int + dp)
            as libc::c_schar;
        (*s)
            .pits[i
            as usize] = ((*s).pits[i as usize] as libc::c_int + dp) as libc::c_schar;
        i1 = cde2fcg[((n + 5 as libc::c_int + 16 as libc::c_int * 7 as libc::c_int)
            % 7 as libc::c_int) as usize] as libc::c_int;
        a = (*s).u.note.notes[i as usize].acc as libc::c_int & 0x7 as libc::c_int;
        if a == 0 as libc::c_int {
            if (*curvoice).okey.nacc as libc::c_int == 0 as libc::c_int {
                if sf_old > 0 as libc::c_int {
                    if i1 < sf_old - 1 as libc::c_int {
                        a = A_SH as libc::c_int;
                    }
                } else if sf_old < 0 as libc::c_int {
                    if i1 >= sf_old + 6 as libc::c_int {
                        a = A_FT as libc::c_int;
                    }
                }
            } else {
                j = 0 as libc::c_int;
                while j < (*curvoice).okey.nacc as libc::c_int {
                    if (n + 16 as libc::c_int * 7 as libc::c_int
                        - (*curvoice).okey.pits[j as usize] as libc::c_int)
                        % 7 as libc::c_int == 0 as libc::c_int
                    {
                        a = (*curvoice).okey.accs[j as usize] as libc::c_int;
                        break;
                    } else {
                        j += 1;
                        j;
                    }
                }
            }
        }
        i3 = i1 + i2 + acc1[a as usize] as libc::c_int * 7 as libc::c_int;
        i1 = ((i3 + 1 as libc::c_int + 21 as libc::c_int) / 7 as libc::c_int
            + 2 as libc::c_int - 3 as libc::c_int + 32 as libc::c_int * 5 as libc::c_int)
            % 5 as libc::c_int;
        a = acc2[i1 as libc::c_uint as usize] as libc::c_int;
        if (*s).u.note.notes[i as usize].acc as libc::c_int != 0 as libc::c_int {
            current_block_84 = 572715077006366937;
        } else if (*curvoice).ckey.empty != 0 {
            if a == A_NT as libc::c_int
                || acc_same_pitch((*s).u.note.notes[i as usize].pit as libc::c_int)
                    >= 0 as libc::c_int
            {
                current_block_84 = 6483416627284290920;
            } else {
                current_block_84 = 572715077006366937;
            }
        } else if (*curvoice).ckey.nacc as libc::c_int > 0 as libc::c_int {
            i4 = cgd2cde[((i3 + 16 as libc::c_int * 7 as libc::c_int) % 7 as libc::c_int)
                as libc::c_uint as usize] as libc::c_int;
            j = 0 as libc::c_int;
            while j < (*curvoice).ckey.nacc as libc::c_int {
                if (i4 + 16 as libc::c_int * 7 as libc::c_int
                    - (*curvoice).ckey.pits[j as usize] as libc::c_int)
                    % 7 as libc::c_int == 0 as libc::c_int
                {
                    break;
                }
                j += 1;
                j;
            }
            if j < (*curvoice).ckey.nacc as libc::c_int {
                current_block_84 = 6483416627284290920;
            } else {
                current_block_84 = 572715077006366937;
            }
        } else {
            current_block_84 = 6483416627284290920;
        }
        match current_block_84 {
            572715077006366937 => {
                i1 = (*s).u.note.notes[i as usize].acc as libc::c_int
                    & 0x7 as libc::c_int;
                i4 = (*s).u.note.notes[i as usize].acc as libc::c_int
                    >> 3 as libc::c_int;
                if i4 != 0 as libc::c_int && i1 != a {
                    if (*s).u.note.microscale != 0 {
                        n = i4;
                        d = (*s).u.note.microscale as libc::c_int;
                    } else {
                        n = parse.micro_tb[i4 as usize] as libc::c_int;
                        d = ((n & 0xff as libc::c_int) + 1 as libc::c_int)
                            * 2 as libc::c_int;
                        n = (n >> 8 as libc::c_int) + 1 as libc::c_int;
                    }
                    match a {
                        2 => {
                            if n >= d / 2 as libc::c_int {
                                n -= d / 2 as libc::c_int;
                                a = i1;
                            } else {
                                a = if i1 == A_SH as libc::c_int {
                                    A_FT as libc::c_int
                                } else {
                                    A_SH as libc::c_int
                                };
                            }
                        }
                        4 => {
                            if n >= d / 2 as libc::c_int {
                                (*s)
                                    .u
                                    .note
                                    .notes[i as usize]
                                    .pit = ((*s).u.note.notes[i as usize].pit as libc::c_int
                                    + 1 as libc::c_int) as libc::c_schar;
                                (*s)
                                    .pits[i
                                    as usize] = ((*s).pits[i as usize] as libc::c_int
                                    + 1 as libc::c_int) as libc::c_schar;
                                n -= d / 2 as libc::c_int;
                            } else {
                                n += d / 2 as libc::c_int;
                            }
                            a = i1;
                        }
                        5 => {
                            if n >= d / 2 as libc::c_int {
                                (*s)
                                    .u
                                    .note
                                    .notes[i as usize]
                                    .pit = ((*s).u.note.notes[i as usize].pit as libc::c_int
                                    - 1 as libc::c_int) as libc::c_schar;
                                (*s)
                                    .pits[i
                                    as usize] = ((*s).pits[i as usize] as libc::c_int
                                    - 1 as libc::c_int) as libc::c_schar;
                                n -= d / 2 as libc::c_int;
                            } else {
                                n += d / 2 as libc::c_int;
                            }
                            a = i1;
                        }
                        _ => {}
                    }
                    if (*s).u.note.microscale != 0 {
                        i4 = n;
                    } else {
                        d = d / 2 as libc::c_int - 1 as libc::c_int
                            + ((n - 1 as libc::c_int) << 8 as libc::c_int);
                        i4 = 1 as libc::c_int;
                        while i4 < 32 as libc::c_int {
                            if parse.micro_tb[i4 as usize] as libc::c_int == d {
                                break;
                            }
                            if parse.micro_tb[i4 as usize] as libc::c_int
                                == 0 as libc::c_int
                            {
                                parse.micro_tb[i4 as usize] = d as libc::c_ushort;
                                break;
                            } else {
                                i4 += 1;
                                i4;
                            }
                        }
                        if i4 == 32 as libc::c_int {
                            error(
                                1 as libc::c_int,
                                s,
                                b"Too many microtone accidentals\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                            );
                            i4 = 0 as libc::c_int;
                        }
                    }
                }
                (*s)
                    .u
                    .note
                    .notes[i as usize]
                    .acc = (i4 << 3 as libc::c_int | a) as libc::c_uchar;
            }
            _ => {}
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn gch_tr1(
    mut s: *mut SYMBOL,
    mut i: libc::c_int,
    mut i2: libc::c_int,
) {
    let mut p: *mut libc::c_char = &mut *((*s).text).offset(i as isize)
        as *mut libc::c_char;
    let mut q: *mut libc::c_char = p.offset(1 as libc::c_int as isize);
    let mut new_txt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: libc::c_int = 0;
    let mut latin: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut a: libc::c_int = 0;
    let mut i1: libc::c_int = 0;
    let mut i3: libc::c_int = 0;
    let mut i4: libc::c_int = 0;
    static mut note_names: [libc::c_char; 8] = unsafe {
        *::core::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"CDEFGAB\0")
    };
    static mut latin_names: [*const libc::c_char; 7] = [
        b"Do\0" as *const u8 as *const libc::c_char,
        b"R\xC3\xA9\0" as *const u8 as *const libc::c_char,
        b"Mi\0" as *const u8 as *const libc::c_char,
        b"Fa\0" as *const u8 as *const libc::c_char,
        b"Sol\0" as *const u8 as *const libc::c_char,
        b"La\0" as *const u8 as *const libc::c_char,
        b"Si\0" as *const u8 as *const libc::c_char,
    ];
    static mut acc_name: [*const libc::c_char; 5] = [
        b"bb\0" as *const u8 as *const libc::c_char,
        b"b\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
        b"#\0" as *const u8 as *const libc::c_char,
        b"##\0" as *const u8 as *const libc::c_char,
    ];
    latin = 0 as libc::c_int;
    match *p as libc::c_int {
        65 | 66 => {
            n = *p as libc::c_int - 'A' as i32 + 5 as libc::c_int;
        }
        67 | 69 | 71 => {
            n = *p as libc::c_int - 'C' as i32;
        }
        68 => {
            if *p.offset(1 as libc::c_int as isize) as libc::c_int == 'o' as i32 {
                latin += 1;
                latin;
                n = 0 as libc::c_int;
            } else {
                n = 1 as libc::c_int;
            }
        }
        70 => {
            if *p.offset(1 as libc::c_int as isize) as libc::c_int == 'a' as i32 {
                latin += 1;
                latin;
            }
            n = 3 as libc::c_int;
        }
        76 => {
            latin += 1;
            latin;
            n = 5 as libc::c_int;
        }
        77 => {
            latin += 1;
            latin;
            n = 2 as libc::c_int;
        }
        82 => {
            latin += 1;
            latin;
            if *p.offset(1 as libc::c_int as isize) as libc::c_int != 'e' as i32 {
                latin += 1;
                latin;
            }
            n = 1 as libc::c_int;
        }
        83 => {
            latin += 1;
            latin;
            if *p.offset(1 as libc::c_int as isize) as libc::c_int == 'o' as i32 {
                latin += 1;
                latin;
                n = 4 as libc::c_int;
            } else {
                n = 6 as libc::c_int;
            }
        }
        47 => {
            latin -= 1;
            latin;
        }
        _ => return,
    }
    q = q.offset(latin as isize);
    new_txt = getarena(
        (strlen((*s).text)).wrapping_add(6 as libc::c_int as libc::c_ulong)
            as libc::c_int,
    ) as *mut libc::c_char;
    l = p.offset_from((*s).text) as libc::c_long as libc::c_int;
    memcpy(
        new_txt as *mut libc::c_void,
        (*s).text as *const libc::c_void,
        l as libc::c_ulong,
    );
    (*s).text = new_txt;
    new_txt = new_txt.offset(l as isize);
    p = q;
    if latin >= 0 as libc::c_int {
        a = 0 as libc::c_int;
        while *p as libc::c_int == '#' as i32 {
            a += 1;
            a;
            p = p.offset(1);
            p;
        }
        while *p as libc::c_int == 'b' as i32 {
            a -= 1;
            a;
            p = p.offset(1);
            p;
        }
        i3 = cde2fcg[n as usize] as libc::c_int + i2 + a * 7 as libc::c_int;
        i4 = cgd2cde[((i3 + 16 as libc::c_int * 7 as libc::c_int) % 7 as libc::c_int)
            as libc::c_uint as usize] as libc::c_int;
        i1 = ((i3 + 1 as libc::c_int + 21 as libc::c_int) / 7 as libc::c_int
            + 2 as libc::c_int - 3 as libc::c_int + 32 as libc::c_int * 5 as libc::c_int)
            % 5 as libc::c_int;
        if latin == 0 as libc::c_int {
            let fresh1 = new_txt;
            new_txt = new_txt.offset(1);
            *fresh1 = note_names[i4 as usize];
        } else {
            new_txt = new_txt
                .offset(
                    sprintf(
                        new_txt,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        latin_names[i4 as usize],
                    ) as isize,
                );
        }
        new_txt = new_txt
            .offset(
                sprintf(
                    new_txt,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    acc_name[i1 as usize],
                ) as isize,
            );
    }
    while *p as libc::c_int != '\0' as i32 && *p as libc::c_int != '\n' as i32
        && *p as libc::c_int != '/' as i32
    {
        let fresh2 = p;
        p = p.offset(1);
        let fresh3 = new_txt;
        new_txt = new_txt.offset(1);
        *fresh3 = *fresh2;
    }
    if *p as libc::c_int == '/' as i32 {
        let fresh4 = p;
        p = p.offset(1);
        let fresh5 = new_txt;
        new_txt = new_txt.offset(1);
        *fresh5 = *fresh4;
        q = strchr(note_names.as_ptr(), *p as libc::c_int);
        if !q.is_null() {
            p = p.offset(1);
            p;
            n = q.offset_from(note_names.as_ptr()) as libc::c_long as libc::c_int;
            if *p as libc::c_int == '#' as i32 {
                a = 1 as libc::c_int;
                p = p.offset(1);
                p;
            } else if *p as libc::c_int == 'b' as i32 {
                a = -(1 as libc::c_int);
                p = p.offset(1);
                p;
            } else {
                a = 0 as libc::c_int;
            }
            i3 = cde2fcg[n as usize] as libc::c_int + i2 + a * 7 as libc::c_int;
            i4 = cgd2cde[((i3 + 16 as libc::c_int * 7 as libc::c_int) % 7 as libc::c_int)
                as libc::c_uint as usize] as libc::c_int;
            i1 = ((i3 + 1 as libc::c_int + 21 as libc::c_int) / 7 as libc::c_int
                + 2 as libc::c_int - 3 as libc::c_int
                + 32 as libc::c_int * 5 as libc::c_int) % 5 as libc::c_int;
            let fresh6 = new_txt;
            new_txt = new_txt.offset(1);
            *fresh6 = note_names[i4 as usize];
            new_txt = new_txt
                .offset(
                    sprintf(
                        new_txt,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        acc_name[i1 as usize],
                    ) as isize,
                );
        }
    }
    strcpy(new_txt, p);
}
unsafe extern "C" fn gch_capo(mut s: *mut SYMBOL) {
    let mut p: *mut libc::c_char = (*s).text;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut li: libc::c_int = 0 as libc::c_int;
    static mut capo_txt: *const libc::c_char = b"  (capo: %d)\0" as *const u8
        as *const libc::c_char;
    static mut cap_trans: [libc::c_schar; 12] = [
        0 as libc::c_int as libc::c_schar,
        5 as libc::c_int as libc::c_schar,
        -(2 as libc::c_int) as libc::c_schar,
        3 as libc::c_int as libc::c_schar,
        -(4 as libc::c_int) as libc::c_schar,
        1 as libc::c_int as libc::c_schar,
        -(6 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        4 as libc::c_int as libc::c_schar,
        -(3 as libc::c_int) as libc::c_schar,
        2 as libc::c_int as libc::c_schar,
        -(5 as libc::c_int) as libc::c_schar,
    ];
    while !(strchr(b"^_<>@\0" as *const u8 as *const libc::c_char, *p as libc::c_int))
        .is_null()
    {
        p = strchr(p, '\n' as i32);
        if p.is_null() {
            return;
        }
        p = p.offset(1);
        p;
    }
    i = p.offset_from((*s).text) as libc::c_long as libc::c_int;
    q = strchr(p.offset(1 as libc::c_int as isize), '\n' as i32);
    if !q.is_null() {
        l = q.offset_from(p) as libc::c_long as libc::c_int;
    } else {
        l = strlen(p) as libc::c_int;
    }
    if capo == 0 {
        capo = 1 as libc::c_int;
        li = strlen(capo_txt) as libc::c_int;
    }
    r = getarena(
        (strlen((*s).text))
            .wrapping_add(l as libc::c_ulong)
            .wrapping_add(li as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
    ) as *mut libc::c_char;
    i += l;
    strncpy(r, (*s).text, i as libc::c_ulong);
    let fresh7 = i;
    i = i + 1;
    *r.offset(fresh7 as isize) = '\n' as i32 as libc::c_char;
    strncpy(r.offset(i as isize), p, l as libc::c_ulong);
    if li != 0 {
        sprintf(r.offset(i as isize).offset(l as isize), capo_txt, cfmt.capo);
        l += li;
    }
    if !q.is_null() {
        strcpy(r.offset(i as isize).offset(l as isize), q);
    }
    (*s).text = r;
    gch_tr1(s, i, cap_trans[(cfmt.capo % 12 as libc::c_int) as usize] as libc::c_int);
}
unsafe extern "C" fn gch_transpose(mut s: *mut SYMBOL) {
    let mut in_ch: libc::c_int = 0 as libc::c_int;
    let mut i2: libc::c_int = (*curvoice).ckey.sf as libc::c_int
        - (*curvoice).okey.sf as libc::c_int;
    let mut o: *mut libc::c_char = (*s).text;
    let mut p: *mut libc::c_char = o;
    loop {
        if in_ch != 0
            || (strchr(
                b"^_<>@\0" as *const u8 as *const libc::c_char,
                *p as libc::c_int,
            ))
                .is_null()
        {
            gch_tr1(s, p.offset_from((*s).text) as libc::c_long as libc::c_int, i2);
            p = ((*s).text).offset(p.offset_from(o) as libc::c_long as isize);
            o = (*s).text;
            p = p.offset(1);
            p;
            while *p != 0 {
                if !(strchr(
                    b"\t;\n\0" as *const u8 as *const libc::c_char,
                    *p as libc::c_int,
                ))
                    .is_null()
                {
                    break;
                }
                p = p.offset(1);
                p;
            }
            if *p == 0 {
                break;
            }
            match *p as libc::c_int {
                9 => {
                    in_ch = 1 as libc::c_int;
                }
                59 => {
                    in_ch = (strchr(
                        b"^_<>@\0" as *const u8 as *const libc::c_char,
                        *p.offset(1 as libc::c_int as isize) as libc::c_int,
                    ))
                        .is_null() as libc::c_int;
                }
                _ => {
                    in_ch = 0 as libc::c_int;
                }
            }
        } else {
            p = strchr(p, '\n' as i32);
            if p.is_null() {
                break;
            }
        }
        p = p.offset(1);
        p;
    };
}
unsafe extern "C" fn gch_build(mut s: *mut SYMBOL) {
    let mut gch: *mut gch = 0 as *mut gch;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut antype: libc::c_char = 0;
    let mut sep: libc::c_char = 0;
    let mut w: libc::c_float = 0.;
    let mut h_ann: libc::c_float = 0.;
    let mut h_gch: libc::c_float = 0.;
    let mut y_above: libc::c_float = 0.;
    let mut y_below: libc::c_float = 0.;
    let mut y_left: libc::c_float = 0.;
    let mut y_right: libc::c_float = 0.;
    let mut xspc: libc::c_float = 0.;
    let mut l: libc::c_int = 0;
    let mut ix: libc::c_int = 0;
    let mut box_0: libc::c_int = 0;
    let mut gch_place: libc::c_int = 0;
    if ((*s).posit).gch() as libc::c_int == 0x3 as libc::c_int {
        return;
    }
    (*s)
        .gch = getarena(
        (::core::mem::size_of::<gch>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong) as libc::c_int,
    ) as *mut gch;
    memset(
        (*s).gch as *mut libc::c_void,
        0 as libc::c_int,
        (::core::mem::size_of::<gch>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
    );
    if (*curvoice).transpose as libc::c_int != 0 as libc::c_int {
        gch_transpose(s);
    }
    if cfmt.capo != 0 {
        gch_capo(s);
    }
    gch_place = if ((*s).posit).gch() as libc::c_int == 0x2 as libc::c_int {
        -(1 as libc::c_int)
    } else {
        1 as libc::c_int
    };
    h_gch = cfmt.font_tb[cfmt.gcf as usize].size;
    h_ann = cfmt.font_tb[cfmt.anf as usize].size;
    y_right = 0 as libc::c_int as libc::c_float;
    y_left = y_right;
    y_below = y_left;
    y_above = y_below;
    box_0 = cfmt.gchordbox;
    p = (*s).text;
    gch = (*s).gch;
    sep = '\n' as i32 as libc::c_char;
    antype = 'g' as i32 as libc::c_char;
    loop {
        if sep as libc::c_int != 'n' as i32
            && !(strchr(
                b"^_<>@\0" as *const u8 as *const libc::c_char,
                *p as libc::c_int,
            ))
                .is_null()
        {
            (*gch).font = cfmt.anf;
            let fresh8 = p;
            p = p.offset(1);
            antype = *fresh8;
            if antype as libc::c_int == '@' as i32 {
                let mut n: libc::c_int = 0;
                let mut xo: libc::c_float = 0.;
                let mut yo: libc::c_float = 0.;
                if sscanf(
                    p,
                    b"%f,%f%n\0" as *const u8 as *const libc::c_char,
                    &mut xo as *mut libc::c_float,
                    &mut yo as *mut libc::c_float,
                    &mut n as *mut libc::c_int,
                ) != 2 as libc::c_int
                {
                    error(
                        1 as libc::c_int,
                        s,
                        b"Error in annotation \"@\"\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                } else {
                    p = p.offset(n as isize);
                    if *p as libc::c_int == ' ' as i32 {
                        p = p.offset(1);
                        p;
                    }
                    (*gch).x = xo;
                    (*gch).y = yo;
                }
            }
        } else if sep as libc::c_int == '\n' as i32 {
            (*gch).font = cfmt.gcf;
            (*gch).box_0 = box_0 as libc::c_char;
            antype = 'g' as i32 as libc::c_char;
        } else {
            (*gch).font = (*gch.offset(-(1 as libc::c_int as isize))).font;
            (*gch).box_0 = (*gch.offset(-(1 as libc::c_int as isize))).box_0;
        }
        (*gch).type_0 = antype;
        match antype as libc::c_int {
            94 => {
                y_above += h_ann;
            }
            95 => {}
            60 => {
                y_left = (y_left as libc::c_double + h_ann as libc::c_double * 0.5f64)
                    as libc::c_float;
            }
            62 => {
                y_right = (y_right as libc::c_double + h_ann as libc::c_double * 0.5f64)
                    as libc::c_float;
            }
            64 => {
                if (*gch).x == 0 as libc::c_int as libc::c_float
                    && (*gch).y == 0 as libc::c_int as libc::c_float && gch != (*s).gch
                    && (*(*s).gch).type_0 as libc::c_int == '@' as i32
                {
                    (*gch).x = (*gch.offset(-(1 as libc::c_int as isize))).x;
                    (*gch).y = (*gch.offset(-(1 as libc::c_int as isize))).y - h_ann;
                }
            }
            _ => {
                if !(gch_place < 0 as libc::c_int) {
                    y_above += h_gch;
                    if box_0 != 0 {
                        y_above += 2 as libc::c_int as libc::c_float;
                    }
                }
            }
        }
        (*gch).idx = p.offset_from((*s).text) as libc::c_long as libc::c_uchar;
        loop {
            match *p as libc::c_int {
                92 => {
                    p = p.offset(1);
                    p;
                    if *p as libc::c_int == 'n' as i32 {
                        *p
                            .offset(
                                -(1 as libc::c_int) as isize,
                            ) = '\0' as i32 as libc::c_char;
                        break;
                    } else {
                        p = p.offset(1);
                        p;
                    }
                }
                38 => {
                    loop {
                        match *p as libc::c_int {
                            59 => {
                                p = p.offset(1);
                                p;
                                break;
                            }
                            0 | 10 | 92 => {
                                break;
                            }
                            _ => {
                                p = p.offset(1);
                                p;
                            }
                        }
                    }
                }
                0 | 59 | 10 => {
                    break;
                }
                _ => {
                    p = p.offset(1);
                    p;
                }
            }
        }
        sep = *p;
        if sep as libc::c_int == '\0' as i32 {
            break;
        }
        let fresh9 = p;
        p = p.offset(1);
        *fresh9 = '\0' as i32 as libc::c_char;
        gch = gch.offset(1);
        gch;
        if !(gch.offset_from((*s).gch) as libc::c_long
            >= 8 as libc::c_int as libc::c_long)
        {
            continue;
        }
        error(
            1 as libc::c_int,
            s,
            b"Too many guitar chords / annotations\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        break;
    }
    ix = 0 as libc::c_int;
    gch = (*s).gch;
    while ix < 8 as libc::c_int {
        if (*gch).type_0 as libc::c_int == '\0' as i32 {
            break;
        }
        if !((*gch).type_0 as libc::c_int != 'g' as i32) {
            p = ((*s).text).offset((*gch).idx as libc::c_int as isize);
            q = p;
            while *p as libc::c_int != '\0' as i32 {
                match *p as libc::c_int {
                    35 | 98 | 61 => {
                        if !(p == q
                            || p != q.offset(1 as libc::c_int as isize)
                                && *p.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                    == '\\' as i32)
                        {
                            match *p as libc::c_int {
                                35 => {
                                    *p = 0x1 as libc::c_int as libc::c_char;
                                }
                                98 => {
                                    *p = 0x2 as libc::c_int as libc::c_char;
                                }
                                _ => {
                                    *p = 0x3 as libc::c_int as libc::c_char;
                                }
                            }
                            if *p.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                == '\\' as i32
                            {
                                p = p.offset(-1);
                                p;
                                l = strlen(p) as libc::c_int;
                                memmove(
                                    p as *mut libc::c_void,
                                    p.offset(1 as libc::c_int as isize) as *const libc::c_void,
                                    l as libc::c_ulong,
                                );
                            }
                        }
                    }
                    32 | 47 => {
                        q = p.offset(1 as libc::c_int as isize);
                    }
                    _ => {}
                }
                p = p.offset(1);
                p;
            }
        }
        ix += 1;
        ix;
        gch = gch.offset(1);
        gch;
    }
    ix = 0 as libc::c_int;
    gch = (*s).gch;
    while ix < 8 as libc::c_int {
        if (*gch).type_0 as libc::c_int == '\0' as i32 {
            break;
        }
        if !((*gch).type_0 as libc::c_int == '@' as i32) {
            p = ((*s).text).offset((*gch).idx as libc::c_int as isize);
            str_font((*gch).font as libc::c_int);
            w = tex_str(p);
            (*gch).w = w;
            match (*gch).type_0 as libc::c_int {
                95 => {
                    xspc = (w as libc::c_double * 0.4f64) as libc::c_float;
                    if xspc > 8 as libc::c_int as libc::c_float {
                        xspc = 8 as libc::c_int as libc::c_float;
                    }
                    (*gch).x = -xspc;
                    y_below -= h_ann;
                    (*gch).y = y_below;
                }
                94 => {
                    xspc = (w as libc::c_double * 0.4f64) as libc::c_float;
                    if xspc > 8 as libc::c_int as libc::c_float {
                        xspc = 8 as libc::c_int as libc::c_float;
                    }
                    (*gch).x = -xspc;
                    y_above -= h_ann;
                    (*gch).y = y_above;
                }
                60 => {
                    (*gch).x = -(w + 6 as libc::c_int as libc::c_float);
                    y_left -= h_ann;
                    (*gch).y = y_left;
                }
                62 => {
                    (*gch).x = 6 as libc::c_int as libc::c_float;
                    y_right -= h_ann;
                    (*gch).y = y_right;
                }
                _ => {
                    xspc = (w as libc::c_double * 0.4f64) as libc::c_float;
                    if xspc > 8 as libc::c_int as libc::c_float {
                        xspc = 8 as libc::c_int as libc::c_float;
                    }
                    (*gch).x = -xspc;
                    if gch_place < 0 as libc::c_int {
                        y_below -= h_gch;
                        (*gch).y = y_below;
                        if box_0 != 0 {
                            y_below -= 2 as libc::c_int as libc::c_float;
                            (*gch).y -= 1 as libc::c_int as libc::c_float;
                        }
                    } else {
                        y_above -= h_gch;
                        (*gch).y = y_above;
                        if box_0 != 0 {
                            y_above -= 2 as libc::c_int as libc::c_float;
                            (*gch).y -= 1 as libc::c_int as libc::c_float;
                        }
                    }
                }
            }
        }
        ix += 1;
        ix;
        gch = gch.offset(1);
        gch;
    }
}
unsafe extern "C" fn next_lyric_note(mut s: *mut SYMBOL) -> *mut SYMBOL {
    while !s.is_null()
        && ((*s).abc_type as libc::c_int != 4 as libc::c_int
            || (*s).flags as libc::c_int & 0x20 as libc::c_int != 0)
    {
        s = (*s).next;
    }
    return s;
}
unsafe extern "C" fn get_lyric(mut s: *mut SYMBOL) -> *mut SYMBOL {
    let mut s1: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut word: [libc::c_char; 128] = [0; 128];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ln: libc::c_int = 0;
    let mut cont: libc::c_int = 0;
    let mut f: *mut FONTSPEC = 0 as *mut FONTSPEC;
    (*curvoice)
        .set_have_ly(
            (((*curvoice).posit).voc() as libc::c_int != 0x3 as libc::c_int)
                as libc::c_int as libc::c_uint,
        );
    if (*curvoice).ignore() != 0 {
        loop {
            if ((*s).abc_next).is_null() {
                return s;
            }
            match (*(*s).abc_next).abc_type as libc::c_int {
                2 => {
                    s = process_pscomment((*s).abc_next);
                    continue;
                }
                1 => {
                    if *((*(*s).abc_next).text).offset(0 as libc::c_int as isize)
                        as libc::c_int == 'w' as i32
                        || *((*(*s).abc_next).text).offset(0 as libc::c_int as isize)
                            as libc::c_int == '+' as i32
                    {
                        s = (*s).abc_next;
                        continue;
                    }
                }
                _ => {}
            }
            return s;
        }
    }
    f = &mut *(cfmt.font_tb).as_mut_ptr().offset(cfmt.vof as isize) as *mut FONTSPEC;
    str_font(cfmt.vof as libc::c_int);
    cont = 0 as libc::c_int;
    ln = -(1 as libc::c_int);
    s1 = 0 as *mut SYMBOL;
    s2 = s1;
    's_84: loop {
        if cont == 0 {
            if ln >= 16 as libc::c_int - 1 as libc::c_int {
                error(
                    1 as libc::c_int,
                    s,
                    b"Too many lyric lines\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                ln -= 1;
                ln;
            }
            ln += 1;
            ln;
            s2 = s1;
            s1 = (*curvoice).lyric_start;
            if s1.is_null() {
                s1 = (*curvoice).sym;
            } else {
                s1 = (*s1).next;
            }
            if s1.is_null() {
                error(
                    1 as libc::c_int,
                    s,
                    b"w: without music\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                return s;
            }
        } else {
            cont = 0 as libc::c_int;
        }
        p = &mut *((*s).text).offset(2 as libc::c_int as isize) as *mut libc::c_char;
        while *p as libc::c_int != '\0' as i32 {
            while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
                p = p.offset(1);
                p;
            }
            if *p as libc::c_int == '\0' as i32 {
                break;
            }
            if *p as libc::c_int == '\\' as i32
                && *p.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
            {
                cont = 1 as libc::c_int;
                break;
            } else {
                match *p as libc::c_int {
                    124 => {
                        while !s1.is_null()
                            && (*s1).type_0 as libc::c_int != 3 as libc::c_int
                        {
                            s2 = s1;
                            s1 = (*s1).next;
                        }
                        if s1.is_null() {
                            error(
                                1 as libc::c_int,
                                s2,
                                b"Not enough bar lines for lyric line\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                            );
                            break;
                        } else {
                            s2 = s1;
                            s1 = (*s1).next;
                            p = p.offset(1);
                            p;
                            continue;
                        }
                    }
                    45 => {
                        word[0 as libc::c_int
                            as usize] = 0x10 as libc::c_int as libc::c_char;
                        word[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                        p = p.offset(1);
                        p;
                    }
                    95 => {
                        word[0 as libc::c_int
                            as usize] = 0x11 as libc::c_int as libc::c_char;
                        word[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                        p = p.offset(1);
                        p;
                    }
                    42 => {
                        word[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                        p = p.offset(1);
                        p;
                    }
                    _ => {
                        q = word.as_mut_ptr();
                        loop {
                            let mut c: libc::c_uchar = 0;
                            c = *p as libc::c_uchar;
                            match c as libc::c_int {
                                0 | 32 | 9 | 95 | 42 | 124 => {
                                    break;
                                }
                                126 => {
                                    c = ' ' as i32 as libc::c_uchar;
                                }
                                45 => {
                                    c = 0x10 as libc::c_int as libc::c_uchar;
                                }
                                92 => {
                                    if *p.offset(1 as libc::c_int as isize) as libc::c_int
                                        == '\0' as i32
                                    {
                                        break;
                                    }
                                    match *p.offset(1 as libc::c_int as isize) as libc::c_int {
                                        126 | 95 | 42 | 124 | 45 | 32 | 92 => {
                                            p = p.offset(1);
                                            c = *p as libc::c_uchar;
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                            if q
                                < &mut *word
                                    .as_mut_ptr()
                                    .offset(
                                        (::core::mem::size_of::<[libc::c_char; 128]>()
                                            as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                    ) as *mut libc::c_char
                            {
                                let fresh10 = q;
                                q = q.offset(1);
                                *fresh10 = c as libc::c_char;
                            }
                            p = p.offset(1);
                            p;
                            if c as libc::c_int == 0x10 as libc::c_int {
                                break;
                            }
                        }
                        *q = '\0' as i32 as libc::c_char;
                    }
                }
                if !s1.is_null() {
                    s2 = s1;
                    s1 = next_lyric_note(s1);
                }
                if s1.is_null() {
                    if s2.is_null() {
                        s2 = s;
                    }
                    error(
                        1 as libc::c_int,
                        s2,
                        b"Too many words in lyric line\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                    break;
                } else {
                    if word[0 as libc::c_int as usize] as libc::c_int != '\0' as i32
                        && ((*s1).posit).voc() as libc::c_int != 0x3 as libc::c_int
                    {
                        let mut lyl: *mut lyl = 0 as *mut lyl;
                        let mut w: libc::c_float = 0.;
                        if ((*s1).ly).is_null() {
                            (*s1)
                                .ly = getarena(
                                ::core::mem::size_of::<lyrics>() as libc::c_ulong
                                    as libc::c_int,
                            ) as *mut lyrics;
                            memset(
                                (*s1).ly as *mut libc::c_void,
                                0 as libc::c_int,
                                ::core::mem::size_of::<lyrics>() as libc::c_ulong,
                            );
                        }
                        q = word.as_mut_ptr();
                        if *q as libc::c_int == '$' as i32
                            && isdigit(
                                *q.offset(1 as libc::c_int as isize) as libc::c_uchar
                                    as libc::c_int,
                            ) != 0
                            && ((*q.offset(1 as libc::c_int as isize) as libc::c_int
                                - '0' as i32) as libc::c_uint)
                                < 10 as libc::c_int as libc::c_uint
                        {
                            let mut ft: libc::c_int = 0;
                            ft = *q.offset(1 as libc::c_int as isize) as libc::c_int
                                - '0' as i32;
                            if ft == 0 as libc::c_int {
                                ft = cfmt.vof as libc::c_int;
                            }
                            f = &mut *(cfmt.font_tb).as_mut_ptr().offset(ft as isize)
                                as *mut FONTSPEC;
                            str_font(ft);
                            q = q.offset(2 as libc::c_int as isize);
                        }
                        w = tex_str(q);
                        q = tex_buf.as_mut_ptr();
                        lyl = getarena(
                            (::core::mem::size_of::<lyl>() as libc::c_ulong)
                                .wrapping_sub(
                                    ::core::mem::size_of::<[libc::c_char; 256]>()
                                        as libc::c_ulong,
                                )
                                .wrapping_add(strlen(q))
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as libc::c_int,
                        ) as *mut lyl;
                        (*(*s1).ly).lyl[ln as usize] = lyl;
                        (*lyl).f = f;
                        (*lyl).w = w;
                        strcpy(((*lyl).t).as_mut_ptr(), q);
                        while *q as libc::c_int != '\0' as i32 {
                            if *q as libc::c_int == '$' as i32
                                && isdigit(
                                    *q.offset(1 as libc::c_int as isize) as libc::c_uchar
                                        as libc::c_int,
                                ) != 0
                                && ((*q.offset(1 as libc::c_int as isize) as libc::c_int
                                    - '0' as i32) as libc::c_uint)
                                    < 10 as libc::c_int as libc::c_uint
                            {
                                let mut ft_0: libc::c_int = 0;
                                q = q.offset(1);
                                q;
                                ft_0 = *q as libc::c_int - '0' as i32;
                                if ft_0 == 0 as libc::c_int {
                                    ft_0 = cfmt.vof as libc::c_int;
                                }
                                f = &mut *(cfmt.font_tb).as_mut_ptr().offset(ft_0 as isize)
                                    as *mut FONTSPEC;
                                str_font(ft_0);
                            }
                            q = q.offset(1);
                            q;
                        }
                    }
                    s2 = s1;
                    s1 = (*s1).next;
                }
            }
        }
        loop {
            if ((*s).abc_next).is_null() {
                break 's_84;
            }
            match (*(*s).abc_next).abc_type as libc::c_int {
                2 => {
                    s = process_pscomment((*s).abc_next);
                    f = &mut *(cfmt.font_tb).as_mut_ptr().offset(cfmt.vof as isize)
                        as *mut FONTSPEC;
                    str_font(cfmt.vof as libc::c_int);
                }
                1 => {
                    if *((*(*s).abc_next).text).offset(0 as libc::c_int as isize)
                        as libc::c_int != 'w' as i32
                        && *((*(*s).abc_next).text).offset(0 as libc::c_int as isize)
                            as libc::c_int != '+' as i32
                    {
                        break 's_84;
                    }
                    s = (*s).abc_next;
                    if *((*s).text).offset(0 as libc::c_int as isize) as libc::c_int
                        == '+' as i32
                    {
                        cont = 1 as libc::c_int;
                    }
                    if cont == 0 {
                        s1 = next_lyric_note(s1);
                        if !s1.is_null() {
                            error(
                                1 as libc::c_int,
                                s1,
                                b"Not enough words for lyric line\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                            );
                        }
                    }
                    break;
                }
                _ => {
                    break 's_84;
                }
            }
        }
    }
    if !(next_lyric_note(s1)).is_null() {
        error(
            0 as libc::c_int,
            s1,
            b"Not enough words for lyric line\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    (*curvoice).lyric_start = (*curvoice).last_sym;
    return s;
}
unsafe extern "C" fn voice_link(mut p_voice: *mut VOICE_S) {
    let mut p_voice2: *mut VOICE_S = 0 as *mut VOICE_S;
    p_voice2 = first_voice;
    loop {
        if p_voice2 == p_voice {
            return;
        }
        if ((*p_voice2).next).is_null() {
            break;
        }
        p_voice2 = (*p_voice2).next;
    }
    (*p_voice2).next = p_voice;
}
unsafe extern "C" fn get_over(mut s: *mut SYMBOL) {
    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut p_voice2: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut p_voice3: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut range: libc::c_int = 0;
    let mut voice: libc::c_int = 0;
    let mut voice2: libc::c_int = 0;
    let mut voice3: libc::c_int = 0;
    static mut tx_wrong_dur: [libc::c_char; 32] = unsafe {
        *::core::mem::transmute::<
            &[u8; 32],
            &mut [libc::c_char; 32],
        >(b"Wrong duration in voice overlay\0")
    };
    static mut txt_no_note: [libc::c_char; 25] = unsafe {
        *::core::mem::transmute::<
            &[u8; 25],
            &mut [libc::c_char; 25],
        >(b"No note in voice overlay\0")
    };
    p_voice = curvoice;
    if (*p_voice).ignore() != 0 {
        return;
    }
    if (*s).abc_type as libc::c_int == 6 as libc::c_int
        || (*s).u.v_over.type_0 as libc::c_int == 2 as libc::c_int
    {
        if ((*p_voice).last_sym).is_null() {
            error(1 as libc::c_int, s, txt_no_note.as_mut_ptr());
            return;
        }
        (*(*p_voice).last_sym).sflags |= 0x10 as libc::c_int as libc::c_uint;
        over_bar = 0 as libc::c_int as libc::c_short;
        if over_time < 0 as libc::c_int {
            error(
                1 as libc::c_int,
                s,
                b"Erroneous end of voice overlap\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return;
        }
        curvoice = &mut *voice_tb.as_mut_ptr().offset(over_voice as isize)
            as *mut VOICE_S;
        if (*p_voice).time != over_mxtime {
            error(1 as libc::c_int, s, tx_wrong_dur.as_mut_ptr());
            if (*p_voice).time > over_mxtime {
                (*curvoice).time = (*p_voice).time;
            }
        }
        over_mxtime = 0 as libc::c_int;
        over_voice = -(1 as libc::c_int) as libc::c_short;
        over_time = -(1 as libc::c_int);
        return;
    }
    if (*s).u.v_over.type_0 as libc::c_int == 1 as libc::c_int {
        over_voice = p_voice.offset_from(voice_tb.as_mut_ptr()) as libc::c_long
            as libc::c_short;
        over_time = (*p_voice).time;
        return;
    }
    if ((*p_voice).last_sym).is_null() {
        error(1 as libc::c_int, s, txt_no_note.as_mut_ptr());
        return;
    }
    (*(*p_voice).last_sym).sflags |= 0x10 as libc::c_int as libc::c_uint;
    voice2 = (*s).u.v_over.voice as libc::c_int;
    p_voice2 = &mut *voice_tb.as_mut_ptr().offset(voice2 as isize) as *mut VOICE_S;
    if ((*parsys).voice[voice2 as usize].range as libc::c_int) < 0 as libc::c_int {
        let mut clone: libc::c_int = 0;
        if cfmt.abc2pscompat != 0 {
            error(
                1 as libc::c_int,
                s,
                b"Cannot have %%%%abc2pscompat\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            cfmt.abc2pscompat = 0 as libc::c_int;
        }
        clone = ((*p_voice).clone as libc::c_int >= 0 as libc::c_int) as libc::c_int;
        (*p_voice2).id[0 as libc::c_int as usize] = '&' as i32 as libc::c_char;
        (*p_voice2).id[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        (*p_voice2).set_second(1 as libc::c_int as libc::c_uint);
        (*parsys).voice[voice2 as usize].second = 1 as libc::c_int as libc::c_char;
        (*p_voice2).scale = (*p_voice).scale;
        (*p_voice2).octave = (*p_voice).octave;
        (*p_voice2).transpose = (*p_voice).transpose;
        memcpy(
            &mut (*p_voice2).key as *mut key_s as *mut libc::c_void,
            &mut (*p_voice).key as *mut key_s as *const libc::c_void,
            ::core::mem::size_of::<key_s>() as libc::c_ulong,
        );
        memcpy(
            &mut (*p_voice2).ckey as *mut key_s as *mut libc::c_void,
            &mut (*p_voice).ckey as *mut key_s as *const libc::c_void,
            ::core::mem::size_of::<key_s>() as libc::c_ulong,
        );
        memcpy(
            &mut (*p_voice2).okey as *mut key_s as *mut libc::c_void,
            &mut (*p_voice).okey as *mut key_s as *const libc::c_void,
            ::core::mem::size_of::<key_s>() as libc::c_ulong,
        );
        (*p_voice2).posit = (*p_voice).posit;
        (*p_voice2).staff = (*p_voice).staff;
        (*p_voice2).cstaff = (*p_voice).cstaff;
        (*p_voice2).color = (*p_voice).color;
        (*p_voice2).map_name = (*p_voice).map_name;
        range = (*parsys)
            .voice[p_voice.offset_from(voice_tb.as_mut_ptr()) as libc::c_long as usize]
            .range as libc::c_int;
        voice = 0 as libc::c_int;
        while voice < 32 as libc::c_int {
            if (*parsys).voice[voice as usize].range as libc::c_int > range {
                (*parsys)
                    .voice[voice as usize]
                    .range = ((*parsys).voice[voice as usize].range as libc::c_int
                    + (clone + 1 as libc::c_int)) as libc::c_schar;
            }
            voice += 1;
            voice;
        }
        (*parsys)
            .voice[voice2 as usize]
            .range = (range + 1 as libc::c_int) as libc::c_schar;
        voice_link(p_voice2);
        if clone != 0 {
            voice3 = 32 as libc::c_int;
            loop {
                voice3 -= 1;
                if !(voice3 >= 0 as libc::c_int) {
                    break;
                }
                if ((*parsys).voice[voice3 as usize].range as libc::c_int)
                    < 0 as libc::c_int
                {
                    break;
                }
            }
            if voice3 > 0 as libc::c_int {
                p_voice3 = &mut *voice_tb.as_mut_ptr().offset(voice3 as isize)
                    as *mut VOICE_S;
                strcpy(((*p_voice3).id).as_mut_ptr(), ((*p_voice2).id).as_mut_ptr());
                (*p_voice3).set_second(1 as libc::c_int as libc::c_uint);
                (*parsys)
                    .voice[voice3 as usize]
                    .second = 1 as libc::c_int as libc::c_char;
                (*p_voice3).scale = voice_tb[(*p_voice).clone as usize].scale;
                (*parsys)
                    .voice[voice3 as usize]
                    .range = (range + 2 as libc::c_int) as libc::c_schar;
                voice_link(p_voice3);
                (*p_voice2).clone = voice3 as libc::c_schar;
            } else {
                error(
                    1 as libc::c_int,
                    s,
                    b"Too many voices for overlay cloning\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
        }
    }
    voice = p_voice.offset_from(voice_tb.as_mut_ptr()) as libc::c_long as libc::c_int;
    if over_time < 0 as libc::c_int {
        let mut time: libc::c_int = 0;
        over_bar = 1 as libc::c_int as libc::c_short;
        over_mxtime = (*p_voice).time;
        over_voice = voice as libc::c_short;
        time = (*p_voice2).time;
        s = (*p_voice).last_sym;
        while !((*s).type_0 as libc::c_int == 3 as libc::c_int || (*s).time <= time) {
            s = (*s).prev;
        }
        over_time = (*s).time;
    } else if over_mxtime == 0 as libc::c_int {
        over_mxtime = (*p_voice).time;
    } else if (*p_voice).time != over_mxtime {
        error(1 as libc::c_int, s, tx_wrong_dur.as_mut_ptr());
        if (*p_voice).time > over_mxtime {
            over_mxtime = (*p_voice).time;
            voice_tb[over_voice as usize].time = over_mxtime;
        }
    }
    (*p_voice2).time = over_time;
    curvoice = p_voice2;
}
unsafe extern "C" fn parse_staves(mut s: *mut SYMBOL, mut staves: *mut staff_s) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut voice: libc::c_int = 0;
    let mut flags_st: libc::c_int = 0;
    let mut brace: libc::c_int = 0;
    let mut bracket: libc::c_int = 0;
    let mut parenth: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    let mut flags: libc::c_short = 0;
    let mut p_staff: *mut staff_s = 0 as *mut staff_s;
    err = 0 as libc::c_int;
    flags = 0 as libc::c_int as libc::c_short;
    parenth = 0 as libc::c_int;
    bracket = parenth;
    brace = bracket;
    flags_st = 0 as libc::c_int;
    voice = 0 as libc::c_int;
    p = ((*s).text).offset(7 as libc::c_int as isize);
    while *p as libc::c_int != '\0' as i32
        && isspace(*p as libc::c_uchar as libc::c_int) == 0
    {
        p = p.offset(1);
        p;
    }
    while *p as libc::c_int != '\0' as i32 {
        match *p as libc::c_int {
            32 | 9 => {}
            91 => {
                if parenth != 0 || brace + bracket >= 2 as libc::c_int {
                    error(
                        1 as libc::c_int,
                        s,
                        b"Misplaced '[' in %%%%staves\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                    err = 1 as libc::c_int;
                } else {
                    if brace + bracket == 0 as libc::c_int {
                        flags = (flags as libc::c_int | 0x4 as libc::c_int)
                            as libc::c_short;
                    } else {
                        flags = (flags as libc::c_int | 0x400 as libc::c_int)
                            as libc::c_short;
                    }
                    bracket += 1;
                    bracket;
                    flags_st <<= 8 as libc::c_int;
                    flags_st |= 0x4 as libc::c_int;
                }
            }
            123 => {
                if parenth != 0 || brace != 0 || bracket >= 2 as libc::c_int {
                    error(
                        1 as libc::c_int,
                        s,
                        b"Misplaced '{' in %%%%staves\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                    err = 1 as libc::c_int;
                } else {
                    if bracket == 0 as libc::c_int {
                        flags = (flags as libc::c_int | 0x1 as libc::c_int)
                            as libc::c_short;
                    } else {
                        flags = (flags as libc::c_int | 0x100 as libc::c_int)
                            as libc::c_short;
                    }
                    brace += 1;
                    brace;
                    flags_st <<= 8 as libc::c_int;
                    flags_st |= 0x1 as libc::c_int;
                }
            }
            40 => {
                if parenth != 0 {
                    error(
                        1 as libc::c_int,
                        s,
                        b"Misplaced '(' in %%%%staves\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                    err = 1 as libc::c_int;
                } else {
                    flags = (flags as libc::c_int | 0x10 as libc::c_int)
                        as libc::c_short;
                    parenth += 1;
                    parenth;
                    flags_st <<= 8 as libc::c_int;
                    flags_st |= 0x10 as libc::c_int;
                }
            }
            42 => {
                if brace != 0 && parenth == 0
                    && flags as libc::c_int & (0x1 as libc::c_int | 0x100 as libc::c_int)
                        == 0
                {
                    flags = (flags as libc::c_int | 0x80 as libc::c_int)
                        as libc::c_short;
                }
            }
            43 => {
                flags = (flags as libc::c_int | 0x1000 as libc::c_int) as libc::c_short;
            }
            _ => {
                if isalnum(*p as libc::c_uchar as libc::c_int) == 0
                    && *p as libc::c_int != '_' as i32
                {
                    error(
                        1 as libc::c_int,
                        s,
                        b"Bad voice ID in %%%%staves\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                    err = 1 as libc::c_int;
                } else if voice >= 32 as libc::c_int {
                    error(
                        1 as libc::c_int,
                        s,
                        b"Too many voices in %%%%staves\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                    err = 1 as libc::c_int;
                } else {
                    let mut i: libc::c_int = 0;
                    let mut v: libc::c_int = 0;
                    let mut sep: libc::c_char = 0;
                    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
                    q = p;
                    while isalnum(*p as libc::c_uchar as libc::c_int) != 0
                        || *p as libc::c_int == '_' as i32
                    {
                        p = p.offset(1);
                        p;
                    }
                    sep = *p;
                    *p = '\0' as i32 as libc::c_char;
                    v = -(1 as libc::c_int);
                    i = 0 as libc::c_int;
                    while i < 32 as libc::c_int {
                        if strcmp(q, (voice_tb[i as usize].id).as_mut_ptr())
                            == 0 as libc::c_int
                        {
                            v = i;
                            break;
                        } else {
                            i += 1;
                            i;
                        }
                    }
                    if v < 0 as libc::c_int {
                        error(
                            1 as libc::c_int,
                            s,
                            b"Voice '%s' of %%%%staves has no symbol\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            q,
                        );
                        err = 1 as libc::c_int;
                        p_staff = staves;
                    } else {
                        let fresh11 = voice;
                        voice = voice + 1;
                        p_staff = staves.offset(fresh11 as isize);
                        (*p_staff).voice = v as libc::c_short;
                    }
                    *p = sep;
                    while *p as libc::c_int != '\0' as i32 {
                        match *p as libc::c_int {
                            32 | 9 => {}
                            93 => {
                                if flags_st & 0x4 as libc::c_int == 0 {
                                    error(
                                        1 as libc::c_int,
                                        s,
                                        b"Misplaced ']' in %%%%staves\0" as *const u8
                                            as *const libc::c_char as *mut libc::c_char,
                                    );
                                    err = 1 as libc::c_int;
                                    break;
                                } else {
                                    bracket -= 1;
                                    bracket;
                                    if brace + bracket == 0 as libc::c_int {
                                        flags = (flags as libc::c_int | 0x8 as libc::c_int)
                                            as libc::c_short;
                                    } else {
                                        flags = (flags as libc::c_int | 0x800 as libc::c_int)
                                            as libc::c_short;
                                    }
                                    flags_st >>= 8 as libc::c_int;
                                }
                            }
                            125 => {
                                if flags_st & 0x1 as libc::c_int == 0 {
                                    error(
                                        1 as libc::c_int,
                                        s,
                                        b"Misplaced '}' in %%%%staves\0" as *const u8
                                            as *const libc::c_char as *mut libc::c_char,
                                    );
                                    err = 1 as libc::c_int;
                                    break;
                                } else {
                                    brace -= 1;
                                    brace;
                                    if bracket == 0 as libc::c_int {
                                        flags = (flags as libc::c_int | 0x2 as libc::c_int)
                                            as libc::c_short;
                                    } else {
                                        flags = (flags as libc::c_int | 0x200 as libc::c_int)
                                            as libc::c_short;
                                    }
                                    flags = (flags as libc::c_int & !(0x80 as libc::c_int))
                                        as libc::c_short;
                                    flags_st >>= 8 as libc::c_int;
                                }
                            }
                            41 => {
                                if flags_st & 0x10 as libc::c_int == 0 {
                                    error(
                                        1 as libc::c_int,
                                        s,
                                        b"Misplaced ')' in %%%%staves\0" as *const u8
                                            as *const libc::c_char as *mut libc::c_char,
                                    );
                                    err = 1 as libc::c_int;
                                    break;
                                } else {
                                    parenth -= 1;
                                    parenth;
                                    flags = (flags as libc::c_int | 0x20 as libc::c_int)
                                        as libc::c_short;
                                    flags_st >>= 8 as libc::c_int;
                                }
                            }
                            124 => {
                                flags = (flags as libc::c_int | 0x40 as libc::c_int)
                                    as libc::c_short;
                            }
                            _ => {
                                break;
                            }
                        }
                        p = p.offset(1);
                        p;
                    }
                    (*p_staff).flags = flags;
                    flags = 0 as libc::c_int as libc::c_short;
                    if !(*p as libc::c_int == '\0' as i32) {
                        continue;
                    }
                }
            }
        }
        if *p as libc::c_int == '\0' as i32 {
            break;
        }
        p = p.offset(1);
        p;
    }
    if flags_st != 0 as libc::c_int {
        error(
            1 as libc::c_int,
            s,
            b"'}', ')' or ']' missing in %%%%staves\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        err = 1 as libc::c_int;
    }
    if err != 0 {
        let mut i_0: libc::c_int = 0;
        i_0 = 0 as libc::c_int;
        while i_0 < voice {
            (*staves.offset(i_0 as isize)).flags = 0 as libc::c_int as libc::c_short;
            i_0 += 1;
            i_0;
        }
    }
    if voice < 32 as libc::c_int {
        (*staves.offset(voice as isize)).voice = -(1 as libc::c_int) as libc::c_short;
    }
}
unsafe extern "C" fn get_staves(mut s: *mut SYMBOL) {
    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut p_voice2: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut p_staff: *mut staff_s = 0 as *mut staff_s;
    let mut staves: [staff_s; 32] = [staff_s { voice: 0, flags: 0 }; 32];
    let mut i: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut voice: libc::c_int = 0;
    let mut staff: libc::c_int = 0;
    let mut range: libc::c_int = 0;
    let mut dup_voice: libc::c_int = 0;
    let mut maxtime: libc::c_int = 0;
    memset(
        staves.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[staff_s; 32]>() as libc::c_ulong,
    );
    parse_staves(s, staves.as_mut_ptr());
    if (staves[0 as libc::c_int as usize].voice as libc::c_int) < 0 as libc::c_int {
        return;
    }
    voice_compress();
    voice_dup();
    p_voice = first_voice;
    curvoice = p_voice;
    maxtime = (*p_voice).time;
    flags = ((*p_voice).sym != 0 as *mut libc::c_void as *mut SYMBOL) as libc::c_int;
    p_voice = (*p_voice).next;
    while !p_voice.is_null() {
        if (*p_voice).time > maxtime {
            maxtime = (*p_voice).time;
        }
        if !((*p_voice).sym).is_null() {
            flags = 1 as libc::c_int;
        }
        p_voice = (*p_voice).next;
    }
    if flags == 0 as libc::c_int {
        voice = 0 as libc::c_int;
        while voice < 32 as libc::c_int {
            (*parsys).voice[voice as usize].range = -(1 as libc::c_int) as libc::c_schar;
            voice += 1;
            voice;
        }
    } else {
        if ((*parsys)
            .voice[curvoice.offset_from(voice_tb.as_mut_ptr()) as libc::c_long as usize]
            .range as libc::c_int) < 0 as libc::c_int
        {
            voice = 0 as libc::c_int;
            while voice < 32 as libc::c_int {
                if (*parsys).voice[voice as usize].range as libc::c_int
                    >= 0 as libc::c_int
                {
                    curvoice = &mut *voice_tb.as_mut_ptr().offset(voice as isize)
                        as *mut VOICE_S;
                    break;
                } else {
                    voice += 1;
                    voice;
                }
            }
        }
        (*curvoice).time = maxtime;
        sym_link(s, 8 as libc::c_int);
        (*s).state = 1 as libc::c_int as libc::c_char;
        (*parsys).nstaff = nstaff as libc::c_short;
        system_new();
    }
    staves_found = maxtime;
    voice = 0 as libc::c_int;
    p_voice = voice_tb.as_mut_ptr();
    while voice < 32 as libc::c_int {
        (*p_voice).set_second(0 as libc::c_int as libc::c_uint);
        (*p_voice).set_floating(0 as libc::c_int as libc::c_uint);
        (*p_voice).set_ignore(0 as libc::c_int as libc::c_uint);
        (*p_voice).time = maxtime;
        voice += 1;
        voice;
        p_voice = p_voice.offset(1);
        p_voice;
    }
    dup_voice = 32 as libc::c_int;
    range = 0 as libc::c_int;
    p_staff = staves.as_mut_ptr();
    (*parsys).top_voice = (*p_staff).voice;
    let mut current_block_61: u64;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int && (*p_staff).voice as libc::c_int >= 0 as libc::c_int {
        voice = (*p_staff).voice as libc::c_int;
        p_voice = &mut *voice_tb.as_mut_ptr().offset(voice as isize) as *mut VOICE_S;
        if (*parsys).voice[voice as usize].range as libc::c_int >= 0 as libc::c_int {
            if (*parsys).voice[(dup_voice - 1 as libc::c_int) as usize].range
                as libc::c_int >= 0 as libc::c_int
            {
                error(
                    1 as libc::c_int,
                    s,
                    b"Too many voices for cloning\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                current_block_61 = 6417057564578538666;
            } else {
                dup_voice -= 1;
                voice = dup_voice;
                p_voice2 = &mut *voice_tb.as_mut_ptr().offset(voice as isize)
                    as *mut VOICE_S;
                memcpy(
                    p_voice2 as *mut libc::c_void,
                    p_voice as *const libc::c_void,
                    ::core::mem::size_of::<VOICE_S>() as libc::c_ulong,
                );
                (*p_voice2).next = 0 as *mut VOICE_S;
                (*p_voice2).last_sym = 0 as *mut SYMBOL;
                (*p_voice2).sym = (*p_voice2).last_sym;
                (*p_voice2).tblts[1 as libc::c_int as usize] = 0 as *mut tblt_s;
                (*p_voice2)
                    .tblts[0 as libc::c_int
                    as usize] = (*p_voice2).tblts[1 as libc::c_int as usize];
                (*p_voice2).clone = -(1 as libc::c_int) as libc::c_schar;
                while (*p_voice).clone as libc::c_int > 0 as libc::c_int {
                    p_voice = &mut *voice_tb
                        .as_mut_ptr()
                        .offset((*p_voice).clone as isize) as *mut VOICE_S;
                }
                (*p_voice).clone = voice as libc::c_schar;
                p_voice = p_voice2;
                (*p_staff).voice = voice as libc::c_short;
                current_block_61 = 5891011138178424807;
            }
        } else {
            current_block_61 = 5891011138178424807;
        }
        match current_block_61 {
            5891011138178424807 => {
                let fresh12 = range;
                range = range + 1;
                (*parsys).voice[voice as usize].range = fresh12 as libc::c_schar;
                voice_link(p_voice);
            }
            _ => {}
        }
        i += 1;
        i;
        p_staff = p_staff.offset(1);
        p_staff;
    }
    if *((*s).text).offset(3 as libc::c_int as isize) as libc::c_int == 't' as i32 {
        i = 0 as libc::c_int;
        p_staff = staves.as_mut_ptr();
        while i < 32 as libc::c_int - 2 as libc::c_int
            && (*p_staff).voice as libc::c_int >= 0 as libc::c_int
        {
            flags = (*p_staff).flags as libc::c_int;
            if !(flags & (0x1 as libc::c_int | 0x100 as libc::c_int) == 0) {
                if !(flags & (0x1 as libc::c_int | 0x2 as libc::c_int)
                    == 0x1 as libc::c_int | 0x2 as libc::c_int
                    || flags & (0x100 as libc::c_int | 0x200 as libc::c_int)
                        == 0x100 as libc::c_int | 0x200 as libc::c_int)
                {
                    if !((*p_staff.offset(1 as libc::c_int as isize)).flags
                        as libc::c_int != 0 as libc::c_int)
                    {
                        if !(flags & 0x10 as libc::c_int != 0
                            || (*p_staff.offset(2 as libc::c_int as isize)).flags
                                as libc::c_int & 0x10 as libc::c_int != 0)
                        {
                            if (*p_staff.offset(2 as libc::c_int as isize)).flags
                                as libc::c_int & (0x2 as libc::c_int | 0x200 as libc::c_int)
                                != 0
                            {
                                let ref mut fresh13 = (*p_staff
                                    .offset(1 as libc::c_int as isize))
                                    .flags;
                                *fresh13 = (*fresh13 as libc::c_int | 0x80 as libc::c_int)
                                    as libc::c_short;
                            } else if (*p_staff.offset(2 as libc::c_int as isize)).flags
                                as libc::c_int == 0 as libc::c_int
                                && (*p_staff.offset(3 as libc::c_int as isize)).flags
                                    as libc::c_int & (0x2 as libc::c_int | 0x200 as libc::c_int)
                                    != 0
                            {
                                (*p_staff)
                                    .flags = ((*p_staff).flags as libc::c_int
                                    | 0x10 as libc::c_int) as libc::c_short;
                                let ref mut fresh14 = (*p_staff
                                    .offset(1 as libc::c_int as isize))
                                    .flags;
                                *fresh14 = (*fresh14 as libc::c_int | 0x20 as libc::c_int)
                                    as libc::c_short;
                                let ref mut fresh15 = (*p_staff
                                    .offset(2 as libc::c_int as isize))
                                    .flags;
                                *fresh15 = (*fresh15 as libc::c_int | 0x10 as libc::c_int)
                                    as libc::c_short;
                                let ref mut fresh16 = (*p_staff
                                    .offset(3 as libc::c_int as isize))
                                    .flags;
                                *fresh16 = (*fresh16 as libc::c_int | 0x20 as libc::c_int)
                                    as libc::c_short;
                            }
                        }
                    }
                }
            }
            i += 1;
            i;
            p_staff = p_staff.offset(1);
            p_staff;
        }
    }
    staff = -(1 as libc::c_int);
    i = 0 as libc::c_int;
    p_staff = staves.as_mut_ptr();
    while i < 32 as libc::c_int && (*p_staff).voice as libc::c_int >= 0 as libc::c_int {
        flags = (*p_staff).flags as libc::c_int;
        if flags & (0x10 as libc::c_int | 0x20 as libc::c_int)
            == 0x10 as libc::c_int | 0x20 as libc::c_int
        {
            flags &= !(0x10 as libc::c_int | 0x20 as libc::c_int);
            (*p_staff).flags = flags as libc::c_short;
        }
        voice = (*p_staff).voice as libc::c_int;
        p_voice = &mut *voice_tb.as_mut_ptr().offset(voice as isize) as *mut VOICE_S;
        if flags & 0x80 as libc::c_int != 0 {
            (*p_voice).set_floating(1 as libc::c_int as libc::c_uint);
            (*p_voice).set_second(1 as libc::c_int as libc::c_uint);
        } else {
            staff += 1;
            staff;
            (*parsys).staff[staff as usize].flags = 0 as libc::c_int as libc::c_short;
        }
        (*parsys).voice[voice as usize].staff = staff as libc::c_uchar;
        (*p_voice).cstaff = (*parsys).voice[voice as usize].staff;
        (*p_voice).staff = (*p_voice).cstaff;
        (*parsys)
            .staff[staff as usize]
            .flags = ((*parsys).staff[staff as usize].flags as libc::c_int | flags)
            as libc::c_short;
        if flags & 0x10 as libc::c_int != 0 {
            p_voice2 = p_voice;
            while i < 32 as libc::c_int {
                i += 1;
                i;
                p_staff = p_staff.offset(1);
                p_staff;
                voice = (*p_staff).voice as libc::c_int;
                p_voice = &mut *voice_tb.as_mut_ptr().offset(voice as isize)
                    as *mut VOICE_S;
                if (*p_staff).flags as libc::c_int & 0x1000 as libc::c_int != 0 {
                    (*p_voice2).set_second(1 as libc::c_int as libc::c_uint);
                    p_voice2 = p_voice;
                } else {
                    (*p_voice).set_second(1 as libc::c_int as libc::c_uint);
                }
                (*parsys).voice[voice as usize].staff = staff as libc::c_uchar;
                (*p_voice).cstaff = (*parsys).voice[voice as usize].staff;
                (*p_voice).staff = (*p_voice).cstaff;
                if (*p_staff).flags as libc::c_int & 0x20 as libc::c_int != 0 {
                    break;
                }
            }
            (*parsys)
                .staff[staff as usize]
                .flags = ((*parsys).staff[staff as usize].flags as libc::c_int
                | (*p_staff).flags as libc::c_int) as libc::c_short;
        }
        i += 1;
        i;
        p_staff = p_staff.offset(1);
        p_staff;
    }
    if staff < 0 as libc::c_int {
        staff = 0 as libc::c_int;
    }
    nstaff = staff;
    (*parsys).nstaff = nstaff as libc::c_short;
    if *((*s).text).offset(3 as libc::c_int as isize) as libc::c_int == 'c' as i32 {
        staff = 0 as libc::c_int;
        while staff < nstaff {
            (*parsys)
                .staff[staff as usize]
                .flags = ((*parsys).staff[staff as usize].flags as libc::c_int
                ^ 0x40 as libc::c_int) as libc::c_short;
            staff += 1;
            staff;
        }
    }
    voice = 0 as libc::c_int;
    while voice < 32 as libc::c_int {
        p_voice = &mut *voice_tb.as_mut_ptr().offset(voice as isize) as *mut VOICE_S;
        (*parsys).voice[voice as usize].second = (*p_voice).second() as libc::c_char;
        staff = (*p_voice).staff as libc::c_int;
        if staff > 0 as libc::c_int {
            (*p_voice)
                .set_norepbra(
                    ((*parsys).staff[(staff - 1 as libc::c_int) as usize].flags
                        as libc::c_int & 0x40 as libc::c_int == 0) as libc::c_int
                        as libc::c_uint,
                );
        }
        if (*p_voice).floating() as libc::c_int != 0 && staff == nstaff {
            (*p_voice).set_floating(0 as libc::c_int as libc::c_uint);
        }
        voice += 1;
        voice;
    }
    curvoice = &mut *voice_tb.as_mut_ptr().offset((*parsys).top_voice as isize)
        as *mut VOICE_S;
}
unsafe extern "C" fn voice_init() {
    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    p_voice = voice_tb.as_mut_ptr();
    while i < 32 as libc::c_int {
        (*p_voice).last_sym = 0 as *mut SYMBOL;
        (*p_voice).sym = (*p_voice).last_sym;
        (*p_voice).lyric_start = 0 as *mut SYMBOL;
        (*p_voice).bar_start = 0 as libc::c_int as libc::c_short;
        (*p_voice).time = 0 as libc::c_int;
        (*p_voice).slur_st = 0 as libc::c_int as libc::c_uchar;
        (*p_voice).hy_st = 0 as libc::c_int as libc::c_uint;
        (*p_voice).tie = 0 as *mut SYMBOL;
        (*p_voice).rtie = 0 as *mut SYMBOL;
        i += 1;
        i;
        p_voice = p_voice.offset(1);
        p_voice;
    }
}
unsafe extern "C" fn put_pdfmark(mut p: *mut libc::c_char) {
    let mut c: libc::c_uchar = 0;
    let mut q: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut u: libc::c_int = 0;
    p = trim_title(p, 0 as *mut SYMBOL);
    q = p as *mut libc::c_uchar;
    while *q as libc::c_int != '\0' as i32 {
        match *q as libc::c_int {
            92 | 40 | 41 => {
                break;
            }
            _ => {}
        }
        if *q as libc::c_int >= 0x80 as libc::c_int {
            break;
        }
        q = q.offset(1);
        q;
    }
    if *q as libc::c_int == '\0' as i32 {
        a2b(
            b"[/Title(%s)/OUT pdfmark\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            p,
        );
        return;
    }
    a2b(b"[/Title<FEFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    q = p as *mut libc::c_uchar;
    u = -(1 as libc::c_int);
    while *q as libc::c_int != '\0' as i32 {
        let fresh17 = q;
        q = q.offset(1);
        c = *fresh17;
        if (c as libc::c_int) < 0x80 as libc::c_int {
            if u >= 0 as libc::c_int {
                a2b(
                    b"%04X\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    u,
                );
                u = -(1 as libc::c_int);
            }
            a2b(
                b"%04X\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                c as libc::c_int,
            );
        } else if (c as libc::c_int) < 0xc0 as libc::c_int {
            u = u << 6 as libc::c_int | c as libc::c_int & 0x3f as libc::c_int;
        } else {
            if u >= 0 as libc::c_int {
                a2b(
                    b"%04X\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    u,
                );
                u = -(1 as libc::c_int);
            }
            if (c as libc::c_int) < 0xe0 as libc::c_int {
                u = c as libc::c_int & 0x1f as libc::c_int;
            } else if (c as libc::c_int) < 0xf0 as libc::c_int {
                u = c as libc::c_int & 0xf as libc::c_int;
            } else {
                u = c as libc::c_int & 0x7 as libc::c_int;
            }
        }
    }
    if u >= 0 as libc::c_int {
        a2b(b"%04X\0" as *const u8 as *const libc::c_char as *mut libc::c_char, u);
        u = -(1 as libc::c_int);
    }
    a2b(b">/OUT pdfmark\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
}
unsafe extern "C" fn tune_header_rebuild(mut s: *mut SYMBOL) -> *mut libc::c_char {
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut header: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    len = 0 as libc::c_int;
    s2 = s;
    loop {
        if (*s2).abc_type as libc::c_int == 1 as libc::c_int {
            len = (len as libc::c_ulong)
                .wrapping_add(
                    (strlen((*s2).text)).wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as libc::c_int as libc::c_int;
            if *((*s2).text).offset(0 as libc::c_int as isize) as libc::c_int
                == 'K' as i32
            {
                break;
            }
        }
        s2 = (*s2).abc_next;
    }
    header = malloc((len + 1 as libc::c_int) as libc::c_ulong) as *mut libc::c_char;
    p = header;
    loop {
        if (*s).abc_type as libc::c_int == 1 as libc::c_int {
            strcpy(p, (*s).text);
            p = p.offset(strlen(p) as isize);
            let fresh18 = p;
            p = p.offset(1);
            *fresh18 = '\n' as i32 as libc::c_char;
            if *((*s).text).offset(0 as libc::c_int as isize) as libc::c_int
                == 'K' as i32
            {
                break;
            }
        }
        s = (*s).abc_next;
    }
    let fresh19 = p;
    p = p.offset(1);
    *fresh19 = '\0' as i32 as libc::c_char;
    return header;
}
unsafe extern "C" fn tune_filter(mut s: *mut SYMBOL) {
    let mut opt: *mut tune_opt_s = 0 as *mut tune_opt_s;
    let mut s1: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut r: regex_t = regex_t {
        re_magic: 0,
        re_nsub: 0,
        re_endp: 0 as *const libc::c_char,
        re_g: 0 as *mut re_guts,
    };
    let mut header: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_int = 0;
    header = tune_header_rebuild(s);
    opt = tune_opts;
    while !opt.is_null() {
        let mut last_staves: *mut SYMBOL = 0 as *mut SYMBOL;
        p = &mut *((*(*opt).s).text)
            .offset((2 as libc::c_int + 5 as libc::c_int) as isize) as *mut libc::c_char;
        while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
            p = p.offset(1);
            p;
        }
        ret = regcomp(
            &mut r,
            p,
            0o1 as libc::c_int | 0o10 as libc::c_int | 0o4 as libc::c_int,
        );
        if !(ret != 0) {
            ret = regexec(
                &mut r,
                header,
                0 as libc::c_int as size_t,
                0 as *mut regmatch_t,
                0 as libc::c_int,
            );
            regfree(&mut r);
            if !(ret != 0) {
                cur_tune_opts = opt;
                last_staves = (*s).abc_next;
                s1 = (*(*opt).s).next;
                while !s1.is_null() {
                    if (*s1).abc_type as libc::c_int == 2 as libc::c_int
                        && (strncmp(
                            &mut *((*s1).text).offset(2 as libc::c_int as isize),
                            b"staves\0" as *const u8 as *const libc::c_char,
                            6 as libc::c_int as libc::c_ulong,
                        ) == 0 as libc::c_int
                            || strncmp(
                                &mut *((*s1).text).offset(2 as libc::c_int as isize),
                                b"score\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int as libc::c_ulong,
                            ) == 0 as libc::c_int)
                    {
                        while !last_staves.is_null() {
                            if (*last_staves).abc_type as libc::c_int == 2 as libc::c_int
                                && (strncmp(
                                    &mut *((*last_staves).text)
                                        .offset(2 as libc::c_int as isize),
                                    b"staves\0" as *const u8 as *const libc::c_char,
                                    6 as libc::c_int as libc::c_ulong,
                                ) == 0 as libc::c_int
                                    || strncmp(
                                        &mut *((*last_staves).text)
                                            .offset(2 as libc::c_int as isize),
                                        b"score\0" as *const u8 as *const libc::c_char,
                                        5 as libc::c_int as libc::c_ulong,
                                    ) == 0 as libc::c_int)
                            {
                                (*last_staves).text = (*s1).text;
                                last_staves = (*last_staves).abc_next;
                                break;
                            } else {
                                last_staves = (*last_staves).abc_next;
                            }
                        }
                    } else {
                        s2 = getarena(
                            ::core::mem::size_of::<SYMBOL>() as libc::c_ulong
                                as libc::c_int,
                        ) as *mut SYMBOL;
                        memcpy(
                            s2 as *mut libc::c_void,
                            s1 as *const libc::c_void,
                            ::core::mem::size_of::<SYMBOL>() as libc::c_ulong,
                        );
                        process_pscomment(s2);
                    }
                    s1 = (*s1).next;
                }
                cur_tune_opts = 0 as *mut tune_opt_s;
                tune_voice_opts = (*opt).voice_opts;
            }
        }
        opt = (*opt).next;
    }
    free(header as *mut libc::c_void);
}
unsafe extern "C" fn voice_filter() {
    let mut opt: *mut voice_opt_s = 0 as *mut voice_opt_s;
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut r: regex_t = regex_t {
        re_magic: 0,
        re_nsub: 0,
        re_endp: 0 as *const libc::c_char,
        re_g: 0 as *mut re_guts,
    };
    let mut pass: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    pass = 0 as libc::c_int;
    opt = voice_opts;
    loop {
        if opt.is_null() {
            if pass != 0 as libc::c_int {
                break;
            }
            opt = tune_voice_opts;
            if opt.is_null() {
                break;
            }
            pass += 1;
            pass;
        }
        p = &mut *((*(*opt).s).text)
            .offset((2 as libc::c_int + 6 as libc::c_int) as isize) as *mut libc::c_char;
        while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
            p = p.offset(1);
            p;
        }
        ret = regcomp(&mut r, p, 0o1 as libc::c_int | 0o4 as libc::c_int);
        if !(ret != 0) {
            ret = regexec(
                &mut r,
                ((*curvoice).id).as_mut_ptr(),
                0 as libc::c_int as size_t,
                0 as *mut regmatch_t,
                0 as libc::c_int,
            );
            if ret != 0 && !((*curvoice).nm).is_null() {
                ret = regexec(
                    &mut r,
                    (*curvoice).nm,
                    0 as libc::c_int as size_t,
                    0 as *mut regmatch_t,
                    0 as libc::c_int,
                );
            }
            regfree(&mut r);
            if !(ret != 0) {
                s = (*(*opt).s).next;
                while !s.is_null() {
                    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
                    s2 = getarena(
                        ::core::mem::size_of::<SYMBOL>() as libc::c_ulong as libc::c_int,
                    ) as *mut SYMBOL;
                    memcpy(
                        s2 as *mut libc::c_void,
                        s as *const libc::c_void,
                        ::core::mem::size_of::<SYMBOL>() as libc::c_ulong,
                    );
                    process_pscomment(s2);
                    s = (*s).next;
                }
            }
        }
        opt = (*opt).next;
    };
}
unsafe extern "C" fn check_header(mut s: *mut SYMBOL) -> libc::c_int {
    match *((*s).text).offset(2 as libc::c_int as isize) as libc::c_int {
        69 => {
            if strncmp(
                ((*s).text).offset(2 as libc::c_int as isize),
                b"EPS\0" as *const u8 as *const libc::c_char,
                3 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                return 0 as libc::c_int;
            }
        }
        109 => {
            if strncmp(
                ((*s).text).offset(2 as libc::c_int as isize),
                b"multicol\0" as *const u8 as *const libc::c_char,
                8 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                return 0 as libc::c_int;
            }
        }
        _ => {}
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn set_global_def() {
    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut i: libc::c_int = 0;
    i = 32 as libc::c_int;
    p_voice = voice_tb.as_mut_ptr();
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int) {
            break;
        }
        let mut current_block_2: u64;
        match (*p_voice).key.instr as libc::c_int {
            0 => {
                if pipeformat == 0 {
                    current_block_2 = 6937071982253665452;
                } else {
                    current_block_2 = 3113812946769612536;
                }
            }
            1 | 2 => {
                current_block_2 = 3113812946769612536;
            }
            _ => {
                current_block_2 = 6937071982253665452;
            }
        }
        match current_block_2 {
            3113812946769612536 => {
                if ((*p_voice).posit).std() as libc::c_int == 0 as libc::c_int {
                    ((*p_voice).posit).set_std(0x2 as libc::c_int as libc::c_ushort);
                }
            }
            _ => {}
        }
        if cfmt.autoclef == 0 && !((*p_voice).s_clef).is_null()
            && (*(*p_voice).s_clef).sflags & 0x20 as libc::c_int as libc::c_uint != 0
        {
            (*(*p_voice).s_clef).u.clef.type_0 = 0 as libc::c_int as libc::c_schar;
            (*(*p_voice).s_clef).sflags &= !(0x20 as libc::c_int) as libc::c_uint;
        }
        if cfmt.staffscale != 0. {
            (*p_voice).staffscale = cfmt.staffscale;
        }
        p_voice = p_voice.offset(1);
        p_voice;
    }
    curvoice = &mut *voice_tb.as_mut_ptr().offset((*parsys).top_voice as isize)
        as *mut VOICE_S;
}
unsafe extern "C" fn get_global_def(mut s: *mut SYMBOL) -> *mut SYMBOL {
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    loop {
        s2 = (*s).abc_next;
        if s2.is_null() {
            break;
        }
        match (*s2).abc_type as libc::c_int {
            1 => {
                match *((*s2).text).offset(0 as libc::c_int as isize) as libc::c_int {
                    75 => {
                        s = s2;
                        (*s).state = 1 as libc::c_int as libc::c_char;
                        get_key(s);
                    }
                    73 | 77 | 81 => {
                        s = s2;
                        (*s).state = 1 as libc::c_int as libc::c_char;
                        s = get_info(s);
                    }
                    _ => {
                        break;
                    }
                }
            }
            2 => {
                if check_header(s2) == 0 {
                    break;
                }
                s = s2;
                (*s).state = 1 as libc::c_int as libc::c_char;
                s = process_pscomment(s);
            }
            _ => {
                break;
            }
        }
    }
    set_global_def();
    return s;
}
unsafe extern "C" fn save_maps() {
    let mut omap: *mut map = 0 as *mut map;
    let mut map: *mut map = 0 as *mut map;
    let mut onotes: *mut note_map = 0 as *mut note_map;
    let mut notes: *mut note_map = 0 as *mut note_map;
    omap = maps;
    if omap.is_null() {
        maps_glob = 0 as *mut map;
        return;
    }
    map = getarena(::core::mem::size_of::<map>() as libc::c_ulong as libc::c_int)
        as *mut map;
    maps_glob = map;
    loop {
        memcpy(
            map as *mut libc::c_void,
            omap as *const libc::c_void,
            ::core::mem::size_of::<map>() as libc::c_ulong,
        );
        onotes = (*omap).notes;
        if !onotes.is_null() {
            notes = getarena(
                ::core::mem::size_of::<note_map>() as libc::c_ulong as libc::c_int,
            ) as *mut note_map;
            (*map).notes = notes;
            loop {
                memcpy(
                    notes as *mut libc::c_void,
                    onotes as *const libc::c_void,
                    ::core::mem::size_of::<note_map>() as libc::c_ulong,
                );
                onotes = (*onotes).next;
                if onotes.is_null() {
                    break;
                }
                (*notes)
                    .next = getarena(
                    ::core::mem::size_of::<note_map>() as libc::c_ulong as libc::c_int,
                ) as *mut note_map;
                notes = (*notes).next;
            }
        }
        omap = (*omap).next;
        if omap.is_null() {
            break;
        }
        (*map)
            .next = getarena(
            ::core::mem::size_of::<map>() as libc::c_ulong as libc::c_int,
        ) as *mut map;
        map = (*map).next;
    };
}
unsafe extern "C" fn get_info(mut s: *mut SYMBOL) -> *mut SYMBOL {
    let mut prev: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut current_block: u64;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut info_type: libc::c_char = 0;
    let mut old_lvl: libc::c_int = 0;
    static mut state_txt: [*mut libc::c_char; 3] = [
        b"global\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"header\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"tune\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ];
    old_lvl = lvlarena(((*s).state as libc::c_int != 0 as libc::c_int) as libc::c_int);
    info_type = *((*s).text).offset(0 as libc::c_int as isize);
    match info_type as libc::c_int {
        73 => {
            s = process_pscomment(s);
            current_block = 8507773468922410051;
        }
        75 => {
            get_key(s);
            if (*s).state as libc::c_int != 1 as libc::c_int {
                current_block = 8507773468922410051;
            } else {
                info[('K' as i32 - 'A' as i32) as usize] = s;
                tunenum += 1;
                tunenum;
                if epsf == 0 {
                    bskip(cfmt.topspace);
                }
                a2b(
                    b"%% --- xref %s\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    &mut *((**info
                        .as_mut_ptr()
                        .offset(('X' as i32 - 'A' as i32) as isize))
                        .text)
                        .offset(2 as libc::c_int as isize) as *mut libc::c_char,
                );
                write_heading();
                block_put();
                s2 = info[('T' as i32 - 'A' as i32) as usize];
                p = &mut *((*s2).text).offset(2 as libc::c_int as isize)
                    as *mut libc::c_char;
                if *p as libc::c_int != '\0' as i32 {
                    a2b(
                        b"%% --- font \0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    outft = -(1 as libc::c_int);
                    set_font(TITLEFONT as libc::c_int);
                    a2b(
                        b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                    outft = -(1 as libc::c_int);
                }
                if cfmt.pdfmark != 0 {
                    if *p as libc::c_int != '\0' as i32 {
                        put_pdfmark(p);
                    }
                    if cfmt.pdfmark > 1 as libc::c_int {
                        s2 = (*s2).next;
                        while !s2.is_null() {
                            p = &mut *((*s2).text).offset(2 as libc::c_int as isize)
                                as *mut libc::c_char;
                            if *p as libc::c_int != '\0' as i32 {
                                put_pdfmark(p);
                            }
                            s2 = (*s2).next;
                        }
                    }
                }
                nbar = cfmt.measurefirst;
                over_voice = -(1 as libc::c_int) as libc::c_short;
                over_time = -(1 as libc::c_int);
                over_bar = 0 as libc::c_int as libc::c_short;
                capo = 0 as libc::c_int;
                reset_gen();
                s = get_global_def(s);
                if cfmt.fields[0 as libc::c_int as usize]
                    & ((1 as libc::c_int) << 'Q' as i32 - 'A' as i32) as libc::c_uint
                    == 0
                {
                    info[('Q' as i32 - 'A' as i32) as usize] = 0 as *mut SYMBOL;
                }
                voice_filter();
                if ((*first_voice).tblts[0 as libc::c_int as usize]).is_null() {
                    set_tblt(first_voice);
                }
                current_block = 8507773468922410051;
            }
        }
        76 => {
            match (*s).state as libc::c_int {
                1 => {
                    let mut i: libc::c_int = 0;
                    let mut auto_len: libc::c_int = 0;
                    auto_len = ((*s).u.length.base_length < 0 as libc::c_int)
                        as libc::c_int;
                    i = 32 as libc::c_int;
                    p_voice = voice_tb.as_mut_ptr();
                    loop {
                        i -= 1;
                        if !(i >= 0 as libc::c_int) {
                            break;
                        }
                        (*p_voice).set_auto_len(auto_len as libc::c_uint);
                        p_voice = p_voice.offset(1);
                        p_voice;
                    }
                }
                2 => {
                    (*curvoice)
                        .set_auto_len(
                            ((*s).u.length.base_length < 0 as libc::c_int) as libc::c_int
                                as libc::c_uint,
                        );
                }
                _ => {}
            }
            current_block = 8507773468922410051;
        }
        77 => {
            get_meter(s);
            current_block = 8507773468922410051;
        }
        80 => {
            let mut curvoice_sav: *mut VOICE_S = 0 as *mut VOICE_S;
            if (*s).state as libc::c_int != 2 as libc::c_int {
                info[('P' as i32 - 'A' as i32) as usize] = s;
            } else if !(cfmt.fields[0 as libc::c_int as usize]
                & ((1 as libc::c_int) << 'P' as i32 - 'A' as i32) as libc::c_uint == 0)
            {
                p_voice = &mut *voice_tb
                    .as_mut_ptr()
                    .offset((*parsys).top_voice as isize) as *mut VOICE_S;
                if curvoice != p_voice {
                    if !((*curvoice).time != (*p_voice).time) {
                        if !(!((*p_voice).last_sym).is_null()
                            && (*(*p_voice).last_sym).type_0 as libc::c_int
                                == 10 as libc::c_int)
                        {
                            curvoice_sav = curvoice;
                            curvoice = p_voice;
                            sym_link(s, 10 as libc::c_int);
                            curvoice = curvoice_sav;
                        }
                    }
                } else {
                    sym_link(s, 10 as libc::c_int);
                }
            }
            current_block = 8507773468922410051;
        }
        81 => {
            if cfmt.fields[0 as libc::c_int as usize]
                & ((1 as libc::c_int) << 'Q' as i32 - 'A' as i32) as libc::c_uint == 0
            {
                current_block = 8507773468922410051;
            } else {
                if (*s).state as libc::c_int != 2 as libc::c_int {
                    info[('Q' as i32 - 'A' as i32) as usize] = s;
                } else if !(curvoice
                    != &mut *voice_tb.as_mut_ptr().offset((*parsys).top_voice as isize)
                        as *mut VOICE_S)
                {
                    s2 = (*curvoice).last_sym;
                    if !s2.is_null() {
                        let mut tim: libc::c_int = 0;
                        tim = (*s2).time;
                        loop {
                            if (*s2).type_0 as libc::c_int == 7 as libc::c_int {
                                if ((*s2).next).is_null() {
                                    (*curvoice).last_sym = (*s2).prev;
                                } else {
                                    (*(*s2).next).prev = (*s2).prev;
                                }
                                if ((*s2).prev).is_null() {
                                    (*curvoice).sym = (*s2).next;
                                } else {
                                    (*(*s2).prev).next = (*s2).next;
                                }
                                break;
                            } else {
                                s2 = (*s2).prev;
                                if !(!s2.is_null() && (*s2).time == tim) {
                                    break;
                                }
                            }
                        }
                    }
                    sym_link(s, 7 as libc::c_int);
                }
                current_block = 8507773468922410051;
            }
        }
        84 => {
            if (*s).state as libc::c_int == 0 as libc::c_int {
                current_block = 8507773468922410051;
            } else if (*s).state as libc::c_int == 1 as libc::c_int {
                current_block = 4225995152335578667;
            } else {
                gen_ly(1 as libc::c_int);
                p = &mut *((*s).text).offset(2 as libc::c_int as isize)
                    as *mut libc::c_char;
                if *p as libc::c_int != '\0' as i32 {
                    write_title(s);
                    a2b(
                        b"%% --- + (%s) ---\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        p,
                    );
                    if cfmt.pdfmark != 0 {
                        put_pdfmark(p);
                    }
                }
                voice_init();
                reset_gen();
                s = get_global_def(s);
                current_block = 8507773468922410051;
            }
        }
        85 => {
            if (*s).u.user.value != 0 {
                deco[(*s).u.user.symbol
                    as usize] = parse
                    .deco_tb[((*s).u.user.value as libc::c_int - 128 as libc::c_int)
                    as usize];
            }
            current_block = 8507773468922410051;
        }
        100 | 114 | 115 | 117 => {
            current_block = 8507773468922410051;
        }
        86 => {
            get_voice(s);
            if !((*s).abc_next).is_null()
                && (*(*s).abc_next).abc_type as libc::c_int == 3 as libc::c_int
            {
                s = (*s).abc_next;
                get_clef(s);
            }
            if (*s).state as libc::c_int == 2 as libc::c_int
                && ((*curvoice).last_sym).is_null()
                && (*curvoice).time == 0 as libc::c_int
            {
                voice_filter();
            }
            current_block = 8507773468922410051;
        }
        119 => {
            if (*s).state as libc::c_int != 2 as libc::c_int {
                current_block = 8507773468922410051;
            } else {
                if cfmt.fields[1 as libc::c_int as usize]
                    & ((1 as libc::c_int) << 'w' as i32 - 'a' as i32) as libc::c_uint
                    == 0
                {
                    while !((*s).abc_next).is_null() {
                        if (*(*s).abc_next).abc_type as libc::c_int != 1 as libc::c_int
                            || *((*(*s).abc_next).text).offset(0 as libc::c_int as isize)
                                as libc::c_int != '+' as i32
                        {
                            break;
                        }
                        s = (*s).abc_next;
                    }
                } else {
                    s = get_lyric(s);
                }
                current_block = 8507773468922410051;
            }
        }
        87 => {
            if (*s).state as libc::c_int == 0 as libc::c_int
                || cfmt.fields[0 as libc::c_int as usize]
                    & ((1 as libc::c_int) << 'W' as i32 - 'A' as i32) as libc::c_uint
                    == 0
            {
                current_block = 8507773468922410051;
            } else {
                current_block = 4225995152335578667;
            }
        }
        88 => {
            if epsf == 0 {
                buffer_eob(0 as libc::c_int);
                write_buffer();
                if cfmt.oneperpage != 0 {
                    close_page();
                } else {
                    use_buffer = (cfmt.splittune != 1 as libc::c_int) as libc::c_int;
                }
            }
            memcpy(
                &mut dfmt as *mut FORMAT as *mut libc::c_void,
                &mut cfmt as *mut FORMAT as *const libc::c_void,
                ::core::mem::size_of::<FORMAT>() as libc::c_ulong,
            );
            memcpy(
                &mut info_glob as *mut INFO as *mut libc::c_void,
                &mut info as *mut INFO as *const libc::c_void,
                ::core::mem::size_of::<INFO>() as libc::c_ulong,
            );
            memcpy(
                deco_glob.as_mut_ptr() as *mut libc::c_void,
                deco.as_mut_ptr() as *const libc::c_void,
                ::core::mem::size_of::<[*mut libc::c_char; 256]>() as libc::c_ulong,
            );
            save_maps();
            info[('X' as i32 - 'A' as i32) as usize] = s;
            if !tune_opts.is_null() {
                tune_filter(s);
            }
            current_block = 8507773468922410051;
        }
        _ => {
            if info_type as libc::c_int >= 'A' as i32
                && info_type as libc::c_int <= 'Z' as i32
            {
                prev = 0 as *mut SYMBOL;
                if (*s).state as libc::c_int == 2 as libc::c_int {
                    current_block = 8507773468922410051;
                } else {
                    current_block = 4225995152335578667;
                }
            } else {
                if (*s).state as libc::c_int != 0 as libc::c_int {
                    error(
                        1 as libc::c_int,
                        s,
                        b"%s info '%c:' not treated\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        state_txt[(*s).state as libc::c_int as usize],
                        info_type as libc::c_int,
                    );
                }
                current_block = 8507773468922410051;
            }
        }
    }
    match current_block {
        4225995152335578667 => {
            prev = info[(info_type as libc::c_int - 'A' as i32) as usize];
            if prev.is_null()
                || (*prev).state as libc::c_int == 0 as libc::c_int
                    && (*s).state as libc::c_int != 0 as libc::c_int
            {
                info[(info_type as libc::c_int - 'A' as i32) as usize] = s;
            } else {
                while !((*prev).next).is_null() {
                    prev = (*prev).next;
                }
                (*prev).next = s;
            }
            while !((*s).abc_next).is_null()
                && (*(*s).abc_next).abc_type as libc::c_int == 1 as libc::c_int
                && *((*(*s).abc_next).text).offset(0 as libc::c_int as isize)
                    as libc::c_int == '+' as i32
            {
                prev = s;
                s = (*s).abc_next;
                (*prev).next = s;
            }
            (*s).prev = prev;
        }
        _ => {}
    }
    lvlarena(old_lvl);
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn identify_note(
    mut s: *mut SYMBOL,
    mut dur: libc::c_int,
    mut p_head: *mut libc::c_int,
    mut p_dots: *mut libc::c_int,
    mut p_flags: *mut libc::c_int,
) {
    let mut head: libc::c_int = 0;
    let mut dots: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    if dur % 12 as libc::c_int != 0 as libc::c_int {
        error(
            1 as libc::c_int,
            s,
            b"Invalid note duration\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    dur /= 12 as libc::c_int;
    if dur == 0 as libc::c_int {
        error(
            1 as libc::c_int,
            s,
            b"Note too short\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    flags = 5 as libc::c_int;
    while dur != 0 as libc::c_int {
        if dur & 1 as libc::c_int != 0 {
            break;
        }
        dur >>= 1 as libc::c_int;
        flags -= 1;
        flags;
    }
    dur >>= 1 as libc::c_int;
    match dur {
        0 => {
            dots = 0 as libc::c_int;
        }
        1 => {
            dots = 1 as libc::c_int;
        }
        3 => {
            dots = 2 as libc::c_int;
        }
        7 => {
            dots = 3 as libc::c_int;
        }
        _ => {
            error(
                1 as libc::c_int,
                s,
                b"Note too much dotted\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            dots = 3 as libc::c_int;
        }
    }
    flags -= dots;
    if flags >= 0 as libc::c_int {
        head = 0 as libc::c_int;
    } else {
        let mut current_block_23: u64;
        match flags {
            -4 => {
                current_block_23 = 552038978677407152;
            }
            -3 => {
                head = if cfmt.squarebreve != 0 {
                    3 as libc::c_int
                } else {
                    2 as libc::c_int
                };
                current_block_23 = 13550086250199790493;
            }
            -2 => {
                head = 2 as libc::c_int;
                current_block_23 = 13550086250199790493;
            }
            -1 => {
                head = 1 as libc::c_int;
                current_block_23 = 13550086250199790493;
            }
            _ => {
                error(
                    1 as libc::c_int,
                    s,
                    b"Note too long\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                flags = -(4 as libc::c_int);
                current_block_23 = 552038978677407152;
            }
        }
        match current_block_23 {
            552038978677407152 => {
                head = 3 as libc::c_int;
            }
            _ => {}
        }
    }
    *p_head = head;
    *p_flags = flags;
    *p_dots = dots;
}
unsafe extern "C" fn adjust_dur(mut s: *mut SYMBOL) {
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut time: libc::c_int = 0;
    let mut auto_time: libc::c_int = 0;
    s2 = (*curvoice).last_sym;
    if s2.is_null() {
        return;
    }
    if (*s2).type_0 as libc::c_int == 9 as libc::c_int
        || (*s2).type_0 as libc::c_int == 3 as libc::c_int
    {
        return;
    }
    while (*s2).type_0 as libc::c_int != 3 as libc::c_int && !((*s2).prev).is_null() {
        s2 = (*s2).prev;
    }
    time = (*s2).time;
    auto_time = (*curvoice).time - time;
    if time == 0 as libc::c_int {
        while !s2.is_null() && (*s2).dur == 0 as libc::c_int {
            s2 = (*s2).next;
        }
        if !s2.is_null() && (*s2).abc_type as libc::c_int == 5 as libc::c_int
            && (*s2).flags as libc::c_int & 0x2 as libc::c_int != 0
        {
            time += (*s2).dur * (*curvoice).wmeasure as libc::c_int / auto_time;
            if !((*s2).prev).is_null() {
                (*(*s2).prev).next = (*s2).next;
            } else {
                (*curvoice).sym = (*s2).next;
            }
            if !((*s2).next).is_null() {
                (*(*s2).next).prev = (*s2).prev;
            }
            s2 = (*s2).next;
        }
    }
    if (*curvoice).wmeasure as libc::c_int == auto_time {
        return;
    }
    while !s2.is_null() {
        let mut i: libc::c_int = 0;
        let mut head: libc::c_int = 0;
        let mut dots: libc::c_int = 0;
        let mut nflags: libc::c_int = 0;
        (*s2).time = time;
        if !((*s2).dur == 0 as libc::c_int
            || (*s2).flags as libc::c_int & 0x20 as libc::c_int != 0)
        {
            (*s2).dur = (*s2).dur * (*curvoice).wmeasure as libc::c_int / auto_time;
            time += (*s2).dur;
            if !((*s2).type_0 as libc::c_int != 1 as libc::c_int) {
                i = 0 as libc::c_int;
                while i <= (*s2).nhd as libc::c_int {
                    (*s2)
                        .u
                        .note
                        .notes[i as usize]
                        .len = (*s2).u.note.notes[i as usize].len
                        * (*curvoice).wmeasure as libc::c_int / auto_time;
                    i += 1;
                    i;
                }
                identify_note(
                    s2,
                    (*s2).u.note.notes[0 as libc::c_int as usize].len,
                    &mut head,
                    &mut dots,
                    &mut nflags,
                );
                (*s2).head = head as libc::c_uchar;
                (*s2).dots = dots as libc::c_char;
                (*s2).nflags = nflags as libc::c_schar;
                if (*s2).nflags as libc::c_int <= -(2 as libc::c_int) {
                    (*s2)
                        .flags = ((*s2).flags as libc::c_int | 0x8 as libc::c_int)
                        as libc::c_ushort;
                } else {
                    (*s2)
                        .flags = ((*s2).flags as libc::c_int & !(0x8 as libc::c_int))
                        as libc::c_ushort;
                }
            }
        }
        s2 = (*s2).next;
    }
    (*s).time = time;
    (*curvoice).time = (*s).time;
}
unsafe extern "C" fn get_bar(mut s: *mut SYMBOL) {
    let mut current_block: u64;
    let mut bar_type: libc::c_int = 0;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    if (*s).u.bar.repeat_bar as libc::c_int != 0
        && (*curvoice).norepbra() as libc::c_int != 0 && (*curvoice).second() == 0
    {
        (*s).sflags |= 0x400000 as libc::c_int as libc::c_uint;
    }
    if (*curvoice).auto_len() != 0 {
        adjust_dur(s);
    }
    bar_type = (*s).u.bar.type_0;
    s2 = (*curvoice).last_sym;
    if !s2.is_null() && (*s2).type_0 as libc::c_int == 2 as libc::c_int {
        (*s2).time -= 1;
        (*s2).time;
        current_block = 14576567515993809846;
    } else if !s2.is_null() && (*s2).type_0 as libc::c_int == 3 as libc::c_int {
        if bar_type == 2 as libc::c_int && ((*s2).text).is_null()
            && (curvoice
                == &mut *voice_tb.as_mut_ptr().offset((*parsys).top_voice as isize)
                    as *mut VOICE_S
                || (*parsys)
                    .staff[((*curvoice).staff as libc::c_int - 1 as libc::c_int)
                        as usize]
                    .flags as libc::c_int & 0x40 as libc::c_int != 0
                || (*s).sflags & 0x400000 as libc::c_int as libc::c_uint != 0)
        {
            (*s2).text = (*s).text;
            (*s2).u.bar.repeat_bar = (*s).u.bar.repeat_bar;
            (*s2)
                .flags = ((*s2).flags as libc::c_int
                | (*s).flags as libc::c_int
                    & (0x100 as libc::c_int | 0x200 as libc::c_int)) as libc::c_ushort;
            (*s2).sflags
                |= (*s).sflags
                    & (0x400000 as libc::c_int | 0x10000000 as libc::c_int
                        | 0x8000 as libc::c_int) as libc::c_uint;
            s = s2;
            current_block = 11288868964437395538;
        } else {
            if bar_type == 0x14 as libc::c_int && ((*s).text).is_null() {
                if (*s2).u.bar.type_0 == 0x41 as libc::c_int {
                    (*s2).u.bar.type_0 = 0x44 as libc::c_int;
                    (*s2)
                        .flags = ((*s2).flags as libc::c_int | 0x200 as libc::c_int)
                        as libc::c_ushort;
                    (*s2).sflags |= 0x8000 as libc::c_int as libc::c_uint;
                    return;
                }
                if (*s2).u.bar.type_0 == 0x11 as libc::c_int {
                    (*s2)
                        .u
                        .bar
                        .type_0 = (0x1 as libc::c_int) << 8 as libc::c_int
                        | 0x14 as libc::c_int;
                    (*s2)
                        .flags = ((*s2).flags as libc::c_int | 0x200 as libc::c_int)
                        as libc::c_ushort;
                    (*s2).sflags |= 0x8000 as libc::c_int as libc::c_uint;
                    return;
                }
            }
            current_block = 14576567515993809846;
        }
    } else {
        current_block = 14576567515993809846;
    }
    match current_block {
        14576567515993809846 => {
            if !s2.is_null() && (*s2).type_0 as libc::c_int == 6 as libc::c_int
                && (((*s2).prev).is_null()
                    || (*(*s2).prev).type_0 as libc::c_int != 3 as libc::c_int)
            {
                (*curvoice).last_sym = (*s2).prev;
                if ((*curvoice).last_sym).is_null() {
                    (*curvoice).sym = 0 as *mut SYMBOL;
                }
                sym_link(s, 3 as libc::c_int);
                (*s).next = s2;
                (*s2).prev = s;
                (*curvoice).last_sym = s2;
            } else {
                sym_link(s, 3 as libc::c_int);
            }
            (*s).staff = (*curvoice).staff;
            let mut current_block_43: u64;
            match bar_type {
                2 | 35 => {
                    (*s)
                        .flags = ((*s).flags as libc::c_int | 0x2 as libc::c_int)
                        as libc::c_ushort;
                    current_block_43 = 15090052786889560393;
                }
                1044 | 16660 => {
                    bar_type = ((4 as libc::c_int) << 4 as libc::c_int)
                        + 4 as libc::c_int;
                    (*s).u.bar.type_0 = bar_type;
                    current_block_43 = 15090052786889560393;
                }
                17 => {
                    if cfmt.rbdbstop == 0 {
                        current_block_43 = 15090052786889560393;
                    } else {
                        current_block_43 = 14507434268598112436;
                    }
                }
                33 | 19 => {
                    current_block_43 = 14507434268598112436;
                }
                _ => {
                    current_block_43 = 15090052786889560393;
                }
            }
            match current_block_43 {
                14507434268598112436 => {
                    (*s)
                        .flags = ((*s).flags as libc::c_int | 0x200 as libc::c_int)
                        as libc::c_ushort;
                    (*s).sflags |= 0x8000 as libc::c_int as libc::c_uint;
                }
                _ => {}
            }
            if (*s).u.bar.dc.n as libc::c_int > 0 as libc::c_int {
                deco_cnv(&mut (*s).u.bar.dc, s, 0 as *mut SYMBOL);
            }
        }
        _ => {}
    }
    if !((*s).text).is_null() {
        if (*s).u.bar.repeat_bar == 0 {
            gch_build(s);
        } else {
            (*s)
                .gch = getarena(
                (::core::mem::size_of::<gch>() as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as libc::c_int,
            ) as *mut gch;
            memset(
                (*s).gch as *mut libc::c_void,
                0 as libc::c_int,
                (::core::mem::size_of::<gch>() as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong),
            );
            (*(*s).gch).type_0 = 'r' as i32 as libc::c_char;
            (*(*s).gch).font = REPEATFONT as libc::c_int as libc::c_uchar;
            str_font(REPEATFONT as libc::c_int);
            (*(*s).gch).w = tex_str((*s).text);
            (*(*s).gch).x = (4 as libc::c_int + 4 as libc::c_int) as libc::c_float;
        }
    }
}
unsafe extern "C" fn set_tblt(mut p_voice: *mut VOICE_S) {
    let mut tblt: *mut tblt_s = 0 as *mut tblt_s;
    let mut i: libc::c_int = 0;
    let mut current_block_4: u64;
    i = 0 as libc::c_int;
    while i < ncmdtblt {
        if !(cmdtblts[i as usize].active == 0) {
            if *(cmdtblts[i as usize].vn).offset(0 as libc::c_int as isize)
                as libc::c_int != '\0' as i32
            {
                if strcmp(cmdtblts[i as usize].vn, ((*p_voice).id).as_mut_ptr())
                    != 0 as libc::c_int
                    && (((*p_voice).nm).is_null()
                        || strcmp(cmdtblts[i as usize].vn, (*p_voice).nm)
                            != 0 as libc::c_int)
                    && (((*p_voice).snm).is_null()
                        || strcmp(cmdtblts[i as usize].vn, (*p_voice).snm)
                            != 0 as libc::c_int)
                {
                    current_block_4 = 16668937799742929182;
                } else {
                    current_block_4 = 6873731126896040597;
                }
            } else {
                current_block_4 = 6873731126896040597;
            }
            match current_block_4 {
                16668937799742929182 => {}
                _ => {
                    tblt = tblts[cmdtblts[i as usize].index as usize];
                    if !((*p_voice).tblts[0 as libc::c_int as usize] == tblt
                        || (*p_voice).tblts[1 as libc::c_int as usize] == tblt)
                    {
                        if ((*p_voice).tblts[0 as libc::c_int as usize]).is_null() {
                            (*p_voice).tblts[0 as libc::c_int as usize] = tblt;
                        } else {
                            (*p_voice).tblts[1 as libc::c_int as usize] = tblt;
                        }
                    }
                }
            }
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn do_tune() {
    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s1: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut i: libc::c_int = 0;
    lvlarena(0 as libc::c_int);
    nstaff = 0 as libc::c_int;
    staves_found = -(1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        p_voice = &mut *voice_tb.as_mut_ptr().offset(i as isize) as *mut VOICE_S;
        s1 = getarena(::core::mem::size_of::<SYMBOL>() as libc::c_ulong as libc::c_int)
            as *mut SYMBOL;
        memset(
            s1 as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<SYMBOL>() as libc::c_ulong,
        );
        (*s1).type_0 = 4 as libc::c_int as libc::c_uchar;
        (*s1).voice = i as libc::c_uchar;
        if cfmt.autoclef != 0 {
            (*s1).u.clef.type_0 = 4 as libc::c_int as libc::c_schar;
            (*s1).sflags = 0x20 as libc::c_int as libc::c_uint;
        } else {
            (*s1).u.clef.type_0 = 0 as libc::c_int as libc::c_schar;
        }
        (*s1).u.clef.line = 2 as libc::c_int as libc::c_char;
        (*p_voice).s_clef = s1;
        (*p_voice).meter.wmeasure = 1 as libc::c_int as libc::c_short;
        (*p_voice).wmeasure = 1 as libc::c_int as libc::c_short;
        (*p_voice).scale = 1 as libc::c_int as libc::c_float;
        (*p_voice).clone = -(1 as libc::c_int) as libc::c_schar;
        (*p_voice).over = -(1 as libc::c_int) as libc::c_schar;
        (*p_voice).posit = cfmt.posit;
        (*p_voice).stafflines = 0 as *mut libc::c_char;
        i += 1;
        i;
    }
    first_voice = voice_tb.as_mut_ptr();
    curvoice = first_voice;
    reset_deco();
    abc2win = 0 as libc::c_int;
    clip_start.bar = -(1 as libc::c_int) as libc::c_short;
    clip_end
        .bar = (!(0 as libc::c_int) as libc::c_ushort as libc::c_int >> 1 as libc::c_int)
        as libc::c_short;
    parsys = 0 as *mut SYSTEM;
    system_new();
    (*parsys).voice[0 as libc::c_int as usize].range = 0 as libc::c_int as libc::c_schar;
    (*parsys)
        .top_voice = (*parsys).voice[0 as libc::c_int as usize].range as libc::c_short;
    if epsf == 0 {
        use_buffer = 0 as libc::c_int;
    } else {
        use_buffer = 1 as libc::c_int;
        marg_init();
    }
    s = parse.first_sym;
    while !s.is_null() {
        match (*s).abc_type as libc::c_int {
            7 => {
                if (*s).u.eoln.type_0 as libc::c_int == 2 as libc::c_int {
                    abc2win = 1 as libc::c_int;
                }
            }
            4 | 5 => {
                (*s).dur = (*s).u.note.notes[0 as libc::c_int as usize].len;
            }
            _ => {}
        }
        s = (*s).abc_next;
    }
    if voice_tb[0 as libc::c_int as usize].id[0 as libc::c_int as usize] as libc::c_int
        == '\0' as i32
    {
        voice_tb[0 as libc::c_int as usize]
            .id[0 as libc::c_int as usize] = '1' as i32 as libc::c_char;
        voice_tb[0 as libc::c_int as usize]
            .id[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    }
    let mut current_block_115: u64;
    s = parse.first_sym;
    while !s.is_null() {
        if (*s).flags as libc::c_int & 0x10 as libc::c_int != 0 {
            (*curvoice).lyric_start = (*curvoice).last_sym;
        }
        match (*s).abc_type as libc::c_int {
            1 => {
                s = get_info(s);
                current_block_115 = 12065775993741208975;
            }
            2 => {
                s = process_pscomment(s);
                current_block_115 = 12065775993741208975;
            }
            4 | 5 => {
                if (*curvoice).space() as libc::c_int != 0
                    && (*s).flags as libc::c_int & 0x20 as libc::c_int == 0
                {
                    (*curvoice).set_space(0 as libc::c_int as libc::c_uint);
                    (*s)
                        .flags = ((*s).flags as libc::c_int | 0x4 as libc::c_int)
                        as libc::c_ushort;
                }
                get_note(s);
                current_block_115 = 12065775993741208975;
            }
            6 => {
                if over_bar != 0 {
                    get_over(s);
                }
                get_bar(s);
                current_block_115 = 12065775993741208975;
            }
            3 => {
                get_clef(s);
                current_block_115 = 12065775993741208975;
            }
            7 => {
                if cfmt.breakoneoln != 0
                    || (*s).flags as libc::c_int & 0x4 as libc::c_int != 0
                {
                    (*curvoice).set_space(1 as libc::c_int as libc::c_uint);
                }
                if cfmt.continueall != 0 || cfmt.barsperstaff != 0
                    || (*s).u.eoln.type_0 as libc::c_int == 1 as libc::c_int
                {
                    current_block_115 = 7226443171521532240;
                } else if (*s).u.eoln.type_0 as libc::c_int == 0 as libc::c_int
                    && abc2win != 0
                    && parse.abc_vers != (2 as libc::c_int) << 16 as libc::c_int
                {
                    current_block_115 = 7226443171521532240;
                } else {
                    if (*parsys)
                        .voice[curvoice.offset_from(voice_tb.as_mut_ptr())
                            as libc::c_long as usize]
                        .range as libc::c_int == 0 as libc::c_int
                        && !((*curvoice).last_sym).is_null()
                    {
                        (*(*curvoice).last_sym).sflags
                            |= 0x1 as libc::c_int as libc::c_uint;
                    }
                    if cfmt.alignbars == 0 {
                        current_block_115 = 7226443171521532240;
                    } else {
                        while !((*s).abc_next).is_null() {
                            if (*(*s).abc_next).abc_type as libc::c_int
                                != 1 as libc::c_int
                            {
                                break;
                            }
                            match *((*(*s).abc_next).text)
                                .offset(0 as libc::c_int as isize) as libc::c_int
                            {
                                119 => {
                                    s = get_info((*s).abc_next);
                                }
                                100 | 115 => {
                                    s = (*s).abc_next;
                                }
                                _ => {
                                    break;
                                }
                            }
                        }
                        i = (curvoice.offset_from(voice_tb.as_mut_ptr()) as libc::c_long
                            + 1 as libc::c_int as libc::c_long) as libc::c_int;
                        if i < cfmt.alignbars {
                            curvoice = &mut *voice_tb.as_mut_ptr().offset(i as isize)
                                as *mut VOICE_S;
                        } else {
                            generate();
                            buffer_eob(0 as libc::c_int);
                            curvoice = &mut *voice_tb
                                .as_mut_ptr()
                                .offset(0 as libc::c_int as isize) as *mut VOICE_S;
                        }
                        current_block_115 = 7226443171521532240;
                    }
                }
            }
            8 => {
                let mut dur: libc::c_int = 0;
                dur = (*curvoice).wmeasure as libc::c_int
                    * (*s).u.bar.len as libc::c_int;
                if (*curvoice).second() != 0 {
                    (*curvoice).time += dur;
                } else {
                    sym_link(s, 9 as libc::c_int);
                    (*s).dur = dur;
                    (*curvoice).time += dur;
                    (*s).color = (*curvoice).color;
                    if !((*s).text).is_null() {
                        gch_build(s);
                    }
                    if (*s).u.bar.dc.n as libc::c_int > 0 as libc::c_int {
                        deco_cnv(&mut (*s).u.bar.dc, s, 0 as *mut SYMBOL);
                    }
                }
                current_block_115 = 12065775993741208975;
            }
            9 => {
                let mut n: libc::c_int = 0;
                s2 = (*curvoice).last_sym;
                if s2.is_null() || (*s2).type_0 as libc::c_int != 3 as libc::c_int {
                    error(
                        1 as libc::c_int,
                        s,
                        b"No bar before measure repeat\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                } else if !((*curvoice).ignore() != 0) {
                    n = (*s).u.bar.len as libc::c_int;
                    if (*curvoice).second() != 0 {
                        (*curvoice).time += (*curvoice).wmeasure as libc::c_int * n;
                    } else {
                        s2 = sym_add(curvoice, 1 as libc::c_int);
                        (*s2).abc_type = 5 as libc::c_int as libc::c_char;
                        (*s2)
                            .flags = ((*s2).flags as libc::c_int | 0x2 as libc::c_int)
                            as libc::c_ushort;
                        (*s2).dur = (*curvoice).wmeasure as libc::c_int;
                        (*curvoice).time += (*s2).dur;
                        if n == 1 as libc::c_int {
                            (*(*s).abc_next).u.bar.len = n as libc::c_char;
                        } else {
                            loop {
                                n -= 1;
                                if !(n > 0 as libc::c_int) {
                                    break;
                                }
                                s2 = sym_add(curvoice, 3 as libc::c_int);
                                (*s2).u.bar.type_0 = 0x1 as libc::c_int;
                                if n == (*s).u.bar.len as libc::c_int - 1 as libc::c_int {
                                    (*s2).u.bar.len = (*s).u.bar.len;
                                }
                                s2 = sym_add(curvoice, 1 as libc::c_int);
                                (*s2).abc_type = 5 as libc::c_int as libc::c_char;
                                (*s2)
                                    .flags = ((*s2).flags as libc::c_int | 0x2 as libc::c_int)
                                    as libc::c_ushort;
                                (*s2).dur = (*curvoice).wmeasure as libc::c_int;
                                (*curvoice).time += (*s2).dur;
                            }
                        }
                    }
                }
                current_block_115 = 12065775993741208975;
            }
            10 => {
                get_over(s);
                current_block_115 = 7226443171521532240;
            }
            11 => {
                set_tuplet(s);
                if (*s).type_0 == 0
                    && (*s).flags as libc::c_int & 0x40 as libc::c_int != 0
                {
                    (*(*curvoice).last_sym)
                        .flags = ((*(*curvoice).last_sym).flags as libc::c_int
                        | 0x40 as libc::c_int) as libc::c_ushort;
                }
                current_block_115 = 12065775993741208975;
            }
            _ => {
                current_block_115 = 7226443171521532240;
            }
        }
        match current_block_115 {
            12065775993741208975 => {
                if !((*s).type_0 as libc::c_int == 0 as libc::c_int) {
                    if (*curvoice).second() != 0 {
                        (*s).sflags |= 0x100000 as libc::c_int as libc::c_uint;
                    }
                    if (*curvoice).floating() != 0 {
                        (*s).sflags |= 0x200000 as libc::c_int as libc::c_uint;
                    }
                }
            }
            _ => {}
        }
        s = (*s).abc_next;
    }
    gen_ly(0 as libc::c_int);
    put_history();
    buffer_eob(1 as libc::c_int);
    if epsf != 0 {
        write_eps();
    } else {
        write_buffer();
    }
    if !(info[('X' as i32 - 'A' as i32) as usize]).is_null() {
        memcpy(
            &mut cfmt as *mut FORMAT as *mut libc::c_void,
            &mut dfmt as *mut FORMAT as *const libc::c_void,
            ::core::mem::size_of::<FORMAT>() as libc::c_ulong,
        );
        memcpy(
            &mut info as *mut INFO as *mut libc::c_void,
            &mut info_glob as *mut INFO as *const libc::c_void,
            ::core::mem::size_of::<INFO>() as libc::c_ulong,
        );
        memcpy(
            deco.as_mut_ptr() as *mut libc::c_void,
            deco_glob.as_mut_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[*mut libc::c_char; 256]>() as libc::c_ulong,
        );
        maps = maps_glob;
        info[('X' as i32 - 'A' as i32) as usize] = 0 as *mut SYMBOL;
    }
    let mut brk: *mut brk_s = 0 as *mut brk_s;
    let mut brk2: *mut brk_s = 0 as *mut brk_s;
    brk = brks;
    while !brk.is_null() {
        brk2 = (*brk).next;
        free(brk as *mut libc::c_void);
        brk = brk2;
    }
    brks = brk;
}
unsafe extern "C" fn is_tune_sig() -> libc::c_int {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    if ((*curvoice).sym).is_null() {
        return 1 as libc::c_int;
    }
    if (*curvoice).time != 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    s = (*curvoice).sym;
    while !s.is_null() {
        match (*s).type_0 as libc::c_int {
            7 | 10 | 12 => {}
            _ => return 0 as libc::c_int,
        }
        s = (*s).next;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn get_clef(mut s: *mut SYMBOL) {
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut voice: libc::c_int = 0;
    p_voice = curvoice;
    (*s).type_0 = 4 as libc::c_int as libc::c_uchar;
    if (*(*s).abc_prev).abc_type as libc::c_int == 1 as libc::c_int {
        match *((*(*s).abc_prev).text).offset(0 as libc::c_int as isize) as libc::c_int {
            75 => {
                if !((*(*s).abc_prev).state as libc::c_int != 1 as libc::c_int) {
                    voice = 0 as libc::c_int;
                    while voice < 32 as libc::c_int {
                        voice_tb[voice as usize].s_clef = s;
                        if (*s).u.clef.type_0 as libc::c_int == 3 as libc::c_int {
                            (voice_tb[voice as usize])
                                .set_perc(1 as libc::c_int as libc::c_uint);
                        }
                        voice += 1;
                        voice;
                    }
                    return;
                }
            }
            86 => {
                p_voice = &mut *voice_tb
                    .as_mut_ptr()
                    .offset((*(*s).abc_prev).u.voice.voice as libc::c_int as isize)
                    as *mut VOICE_S;
                curvoice = p_voice;
            }
            _ => {}
        }
    }
    if is_tune_sig() != 0 {
        (*p_voice).s_clef = s;
    } else {
        s2 = (*p_voice).last_sym;
        if !s2.is_null() && !((*s2).prev).is_null() && (*s2).time == (*curvoice).time
            && ((*s2).type_0 as libc::c_int == 6 as libc::c_int
                || (*s2).type_0 as libc::c_int == 3 as libc::c_int)
        {
            let mut s3: *mut SYMBOL = 0 as *mut SYMBOL;
            s3 = s2;
            while !((*s3).prev).is_null() {
                match (*(*s3).prev).type_0 as libc::c_int {
                    6 | 3 => {}
                    _ => {
                        break;
                    }
                }
                s3 = (*s3).prev;
            }
            (*p_voice).last_sym = (*s3).prev;
            sym_link(s, 4 as libc::c_int);
            (*s).next = s3;
            (*s3).prev = s;
            (*p_voice).last_sym = s2;
        } else {
            sym_link(s, 4 as libc::c_int);
        }
        (*s).aux = 1 as libc::c_int as libc::c_short;
    }
    (*p_voice)
        .set_perc(
            ((*s).u.clef.type_0 as libc::c_int == 3 as libc::c_int) as libc::c_int
                as libc::c_uint,
        );
    if (*s).u.clef.type_0 as libc::c_int == 4 as libc::c_int {
        (*s).sflags |= 0x20 as libc::c_int as libc::c_uint;
    }
}
unsafe extern "C" fn clef_def(mut s: *mut SYMBOL) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut clef: libc::c_int = 0;
    let mut clef_line: libc::c_int = 0;
    let mut str: [libc::c_char; 80] = [0; 80];
    clef = -(1 as libc::c_int);
    clef_line = 2 as libc::c_int;
    p = &mut *((*s).text).offset((2 as libc::c_int + 5 as libc::c_int) as isize)
        as *mut libc::c_char;
    while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
        p = p.offset(1);
        p;
    }
    match *p as libc::c_int {
        34 => {
            p = get_str(
                str.as_mut_ptr(),
                p,
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
        }
        71 => {
            clef = 0 as libc::c_int;
            p = p.offset(1);
            p;
        }
        70 => {
            clef = 2 as libc::c_int;
            clef_line = 4 as libc::c_int;
            p = p.offset(1);
            p;
        }
        67 => {
            clef = 1 as libc::c_int;
            clef_line = 3 as libc::c_int;
            p = p.offset(1);
            p;
        }
        80 => {
            clef = 3 as libc::c_int;
            p = p.offset(1);
            p;
        }
        116 => {
            if strncmp(
                p,
                b"treble\0" as *const u8 as *const libc::c_char,
                6 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                clef = 0 as libc::c_int;
                p = p.offset(6 as libc::c_int as isize);
            }
            if strncmp(
                p,
                b"tenor\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                clef = 1 as libc::c_int;
                clef_line = 4 as libc::c_int;
                p = p.offset(5 as libc::c_int as isize);
            }
        }
        97 => {
            if strncmp(
                p,
                b"alto\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                clef = 1 as libc::c_int;
                clef_line = 3 as libc::c_int;
                p = p.offset(4 as libc::c_int as isize);
            } else if strncmp(
                p,
                b"auto\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                clef = 4 as libc::c_int;
                (*s).sflags |= 0x20 as libc::c_int as libc::c_uint;
                p = p.offset(4 as libc::c_int as isize);
            }
        }
        98 => {
            if strncmp(
                p,
                b"bass\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                clef = 2 as libc::c_int;
                clef_line = 4 as libc::c_int;
                p = p.offset(4 as libc::c_int as isize);
            }
        }
        112 => {
            if strncmp(
                p,
                b"perc\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                clef = 3 as libc::c_int;
                p = p.offset(4 as libc::c_int as isize);
            }
        }
        110 => {
            if strncmp(
                p,
                b"none\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                clef = 0 as libc::c_int;
                (*s).u.clef.invis = 1 as libc::c_int as libc::c_char;
                (*s)
                    .flags = ((*s).flags as libc::c_int | 0x2 as libc::c_int)
                    as libc::c_ushort;
                p = p.offset(4 as libc::c_int as isize);
            }
        }
        _ => {}
    }
    if clef < 0 as libc::c_int {
        error(
            1 as libc::c_int,
            s,
            b"Unknown clef '%s'\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            p,
        );
        return;
    }
    match *p as libc::c_int {
        49 | 50 | 51 | 52 | 53 => {
            let fresh20 = p;
            p = p.offset(1);
            clef_line = *fresh20 as libc::c_int - '0' as i32;
        }
        _ => {}
    }
    if *p.offset(1 as libc::c_int as isize) as libc::c_int == '8' as i32 {
        let mut current_block_63: u64;
        match *p as libc::c_int {
            94 => {
                (*s).u.clef.transpose = -(7 as libc::c_int) as libc::c_schar;
                current_block_63 = 5497590635333825015;
            }
            43 => {
                current_block_63 = 5497590635333825015;
            }
            95 => {
                (*s).u.clef.transpose = 7 as libc::c_int as libc::c_schar;
                current_block_63 = 10889044646538147979;
            }
            45 => {
                current_block_63 = 10889044646538147979;
            }
            _ => {
                current_block_63 = 9512719473022792396;
            }
        }
        match current_block_63 {
            5497590635333825015 => {
                (*s).u.clef.octave = 1 as libc::c_int as libc::c_schar;
            }
            10889044646538147979 => {
                (*s).u.clef.octave = -(1 as libc::c_int) as libc::c_schar;
            }
            _ => {}
        }
    }
    (*s).abc_type = 3 as libc::c_int as libc::c_char;
    (*s).u.clef.type_0 = clef as libc::c_schar;
    (*s).u.clef.line = clef_line as libc::c_char;
    get_clef(s);
}
unsafe extern "C" fn key_transpose(mut key: *mut key_s) {
    let mut t: libc::c_int = 0;
    let mut sf: libc::c_int = 0;
    t = (*curvoice).transpose as libc::c_int / 3 as libc::c_int;
    sf = (t & !(1 as libc::c_int)) + (t & 1 as libc::c_int) * 7 as libc::c_int
        + (*key).sf as libc::c_int;
    match ((*curvoice).transpose as libc::c_int + 210 as libc::c_int) % 3 as libc::c_int
    {
        1 => {
            sf = (sf + 4 as libc::c_int + 12 as libc::c_int * 4 as libc::c_int)
                % 12 as libc::c_int - 4 as libc::c_int;
        }
        2 => {
            sf = (sf + 7 as libc::c_int + 12 as libc::c_int * 4 as libc::c_int)
                % 12 as libc::c_int - 7 as libc::c_int;
        }
        _ => {
            sf = (sf + 5 as libc::c_int + 12 as libc::c_int * 4 as libc::c_int)
                % 12 as libc::c_int - 5 as libc::c_int;
        }
    }
    (*key).sf = sf as libc::c_schar;
}
unsafe extern "C" fn set_k_acc(mut s: *mut SYMBOL) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut nacc: libc::c_int = 0;
    let mut accs: [libc::c_char; 8] = [0; 8];
    let mut pits: [libc::c_char; 8] = [0; 8];
    static mut sharp_tb: [libc::c_char; 8] = [
        26 as libc::c_int as libc::c_char,
        23 as libc::c_int as libc::c_char,
        27 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        21 as libc::c_int as libc::c_char,
        25 as libc::c_int as libc::c_char,
        22 as libc::c_int as libc::c_char,
        0,
    ];
    static mut flat_tb: [libc::c_char; 8] = [
        22 as libc::c_int as libc::c_char,
        25 as libc::c_int as libc::c_char,
        21 as libc::c_int as libc::c_char,
        24 as libc::c_int as libc::c_char,
        20 as libc::c_int as libc::c_char,
        23 as libc::c_int as libc::c_char,
        26 as libc::c_int as libc::c_char,
        0,
    ];
    if (*s).u.key.sf as libc::c_int > 0 as libc::c_int {
        nacc = 0 as libc::c_int;
        while nacc < (*s).u.key.sf as libc::c_int {
            accs[nacc as usize] = A_SH as libc::c_int as libc::c_char;
            pits[nacc as usize] = sharp_tb[nacc as usize];
            nacc += 1;
            nacc;
        }
    } else {
        nacc = 0 as libc::c_int;
        while nacc < -((*s).u.key.sf as libc::c_int) {
            accs[nacc as usize] = A_FT as libc::c_int as libc::c_char;
            pits[nacc as usize] = flat_tb[nacc as usize];
            nacc += 1;
            nacc;
        }
    }
    i = 0 as libc::c_int;
    while i < (*s).u.key.nacc as libc::c_int {
        j = 0 as libc::c_int;
        while j < nacc {
            if pits[j as usize] as libc::c_int
                == (*s).u.key.pits[i as usize] as libc::c_int
            {
                accs[j as usize] = (*s).u.key.accs[i as usize] as libc::c_char;
                break;
            } else {
                j += 1;
                j;
            }
        }
        if j == nacc {
            if nacc as libc::c_ulong
                >= ::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong
            {
                error(
                    1 as libc::c_int,
                    s,
                    b"Too many accidentals\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            } else {
                accs[j as usize] = (*s).u.key.accs[i as usize] as libc::c_char;
                pits[j as usize] = (*s).u.key.pits[i as usize] as libc::c_char;
                nacc += 1;
                nacc;
            }
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < nacc {
        (*s).u.key.accs[i as usize] = accs[i as usize] as libc::c_uchar;
        (*s).u.key.pits[i as usize] = pits[i as usize] as libc::c_schar;
        i += 1;
        i;
    }
    (*s).u.key.nacc = nacc as libc::c_schar;
}
unsafe extern "C" fn get_key(mut s: *mut SYMBOL) {
    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut okey: key_s = key_s {
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
    };
    let mut i: libc::c_int = 0;
    if (*s).u.key.octave as libc::c_int != 10 as libc::c_int {
        (*curvoice).octave = (*s).u.key.octave;
    }
    if (*s).u.key.cue as libc::c_int > 0 as libc::c_int {
        (*curvoice).scale = 0.7f64 as libc::c_float;
    } else if ((*s).u.key.cue as libc::c_int) < 0 as libc::c_int {
        (*curvoice).scale = 1 as libc::c_int as libc::c_float;
    }
    if !((*s).u.key.stafflines).is_null() {
        (*curvoice).stafflines = (*s).u.key.stafflines;
    }
    if (*s).u.key.staffscale != 0 as libc::c_int as libc::c_float {
        (*curvoice).staffscale = (*s).u.key.staffscale;
    }
    if (*s).u.key.empty as libc::c_int == 1 as libc::c_int {
        return;
    }
    if (*s).u.key.sf as libc::c_int != 0 as libc::c_int && (*s).u.key.exp == 0
        && (*s).u.key.nacc as libc::c_int != 0 as libc::c_int
    {
        set_k_acc(s);
    }
    memcpy(
        &mut okey as *mut key_s as *mut libc::c_void,
        &mut (*s).u.key as *mut key_s as *const libc::c_void,
        ::core::mem::size_of::<key_s>() as libc::c_ulong,
    );
    if (*s).state as libc::c_int == 1 as libc::c_int {
        i = 32 as libc::c_int;
        p_voice = voice_tb.as_mut_ptr();
        loop {
            i -= 1;
            if !(i >= 0 as libc::c_int) {
                break;
            }
            (*p_voice).transpose = cfmt.transpose as libc::c_short;
            p_voice = p_voice.offset(1);
            p_voice;
        }
    }
    if (*curvoice).transpose as libc::c_int != 0 as libc::c_int {
        key_transpose(&mut (*s).u.key);
    }
    (*s)
        .u
        .key
        .key_delta = ((cgd2cde[(((*s).u.key.sf as libc::c_int + 7 as libc::c_int)
        % 7 as libc::c_int) as usize] as libc::c_int + 14 as libc::c_int)
        % 7 as libc::c_int) as libc::c_char;
    if (*s).state as libc::c_int == 1 as libc::c_int {
        i = 32 as libc::c_int;
        p_voice = voice_tb.as_mut_ptr();
        loop {
            i -= 1;
            if !(i >= 0 as libc::c_int) {
                break;
            }
            memcpy(
                &mut (*p_voice).key as *mut key_s as *mut libc::c_void,
                &mut (*s).u.key as *mut key_s as *const libc::c_void,
                ::core::mem::size_of::<key_s>() as libc::c_ulong,
            );
            memcpy(
                &mut (*p_voice).ckey as *mut key_s as *mut libc::c_void,
                &mut (*s).u.key as *mut key_s as *const libc::c_void,
                ::core::mem::size_of::<key_s>() as libc::c_ulong,
            );
            memcpy(
                &mut (*p_voice).okey as *mut key_s as *mut libc::c_void,
                &mut okey as *mut key_s as *const libc::c_void,
                ::core::mem::size_of::<key_s>() as libc::c_ulong,
            );
            if (*p_voice).key.empty != 0 {
                (*p_voice).key.sf = 0 as libc::c_int as libc::c_schar;
            }
            if (*s).u.key.octave as libc::c_int != 10 as libc::c_int {
                (*p_voice).octave = (*s).u.key.octave;
            }
            if !((*s).u.key.stafflines).is_null() {
                (*p_voice).stafflines = (*s).u.key.stafflines;
            }
            if (*s).u.key.staffscale != 0 as libc::c_int as libc::c_float {
                (*p_voice).staffscale = (*s).u.key.staffscale;
            }
            p_voice = p_voice.offset(1);
            p_voice;
        }
        return;
    }
    if is_tune_sig() != 0 {
        memcpy(
            &mut (*curvoice).key as *mut key_s as *mut libc::c_void,
            &mut (*s).u.key as *mut key_s as *const libc::c_void,
            ::core::mem::size_of::<key_s>() as libc::c_ulong,
        );
        memcpy(
            &mut (*curvoice).ckey as *mut key_s as *mut libc::c_void,
            &mut (*s).u.key as *mut key_s as *const libc::c_void,
            ::core::mem::size_of::<key_s>() as libc::c_ulong,
        );
        memcpy(
            &mut (*curvoice).okey as *mut key_s as *mut libc::c_void,
            &mut okey as *mut key_s as *const libc::c_void,
            ::core::mem::size_of::<key_s>() as libc::c_ulong,
        );
        let mut current_block_45: u64;
        match (*curvoice).key.instr as libc::c_int {
            0 => {
                if pipeformat == 0 {
                    current_block_45 = 12199444798915819164;
                } else {
                    current_block_45 = 17117868024290124789;
                }
            }
            1 | 2 => {
                current_block_45 = 17117868024290124789;
            }
            _ => {
                current_block_45 = 12199444798915819164;
            }
        }
        match current_block_45 {
            17117868024290124789 => {
                if ((*curvoice).posit).std() as libc::c_int == 0 as libc::c_int {
                    ((*curvoice).posit).set_std(0x2 as libc::c_int as libc::c_ushort);
                }
            }
            _ => {}
        }
        if (*curvoice).key.empty != 0 {
            (*curvoice).key.sf = 0 as libc::c_int as libc::c_schar;
        }
        return;
    }
    if (((*s).abc_next).is_null()
        || (*(*s).abc_next).abc_type as libc::c_int != 3 as libc::c_int)
        && (*curvoice).ckey.sf as libc::c_int == (*s).u.key.sf as libc::c_int
        && (*curvoice).ckey.nacc as libc::c_int == 0 as libc::c_int
        && (*s).u.key.nacc as libc::c_int == 0 as libc::c_int
        && (*curvoice).ckey.empty as libc::c_int == (*s).u.key.empty as libc::c_int
        && cfmt.keywarn != 0
    {
        return;
    }
    if (*curvoice).ckey.empty == 0 {
        (*s).aux = (*curvoice).ckey.sf as libc::c_short;
    }
    memcpy(
        &mut (*curvoice).ckey as *mut key_s as *mut libc::c_void,
        &mut (*s).u.key as *mut key_s as *const libc::c_void,
        ::core::mem::size_of::<key_s>() as libc::c_ulong,
    );
    memcpy(
        &mut (*curvoice).okey as *mut key_s as *mut libc::c_void,
        &mut okey as *mut key_s as *const libc::c_void,
        ::core::mem::size_of::<key_s>() as libc::c_ulong,
    );
    if (*s).u.key.empty != 0 {
        (*s).u.key.sf = 0 as libc::c_int as libc::c_schar;
    }
    s2 = (*curvoice).last_sym;
    if !s2.is_null() && (*s2).type_0 as libc::c_int == 5 as libc::c_int {
        (*curvoice).last_sym = (*s2).prev;
        if ((*curvoice).last_sym).is_null() {
            (*curvoice).sym = 0 as *mut SYMBOL;
        }
        sym_link(s, 6 as libc::c_int);
        (*s).next = s2;
        (*s2).prev = s;
        (*curvoice).last_sym = s2;
    } else {
        sym_link(s, 6 as libc::c_int);
    };
}
unsafe extern "C" fn get_meter(mut s: *mut SYMBOL) {
    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut i: libc::c_int = 0;
    match (*s).state as libc::c_int {
        1 => {
            i = 32 as libc::c_int;
            p_voice = voice_tb.as_mut_ptr();
            loop {
                i -= 1;
                if !(i >= 0 as libc::c_int) {
                    break;
                }
                memcpy(
                    &mut (*p_voice).meter as *mut meter_s as *mut libc::c_void,
                    &mut (*s).u.meter as *mut meter_s as *const libc::c_void,
                    ::core::mem::size_of::<meter_s>() as libc::c_ulong,
                );
                (*p_voice).wmeasure = (*s).u.meter.wmeasure;
                p_voice = p_voice.offset(1);
                p_voice;
            }
        }
        2 => {
            (*curvoice).wmeasure = (*s).u.meter.wmeasure;
            if is_tune_sig() != 0 {
                memcpy(
                    &mut (*curvoice).meter as *mut meter_s as *mut libc::c_void,
                    &mut (*s).u.meter as *mut meter_s as *const libc::c_void,
                    ::core::mem::size_of::<meter_s>() as libc::c_ulong,
                );
                reset_gen();
            } else if !((*s).u.meter.nmeter as libc::c_int == 0 as libc::c_int) {
                sym_link(s, 5 as libc::c_int);
            }
        }
        0 | _ => {}
    };
}
unsafe extern "C" fn get_voice(mut s: *mut SYMBOL) {
    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
    let mut voice: libc::c_int = 0;
    voice = (*s).u.voice.voice as libc::c_int;
    p_voice = &mut *voice_tb.as_mut_ptr().offset(voice as isize) as *mut VOICE_S;
    if ((*parsys).voice[voice as usize].range as libc::c_int) < 0 as libc::c_int {
        if cfmt.alignbars != 0 {
            error(
                1 as libc::c_int,
                s,
                b"V: does not work with %%%%alignbars\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        if staves_found < 0 as libc::c_int {
            if (*s).u.voice.merge == 0 {
                nstaff += 1;
                nstaff;
            } else {
                (*p_voice).set_second(1 as libc::c_int as libc::c_uint);
                (*parsys)
                    .voice[voice as usize]
                    .second = 1 as libc::c_int as libc::c_char;
            }
            (*p_voice).cstaff = nstaff as libc::c_uchar;
            (*p_voice).staff = (*p_voice).cstaff;
            (*parsys).voice[voice as usize].staff = nstaff as libc::c_uchar;
            (*parsys).nstaff = nstaff as libc::c_short;
            let mut range: libc::c_int = 0;
            let mut i: libc::c_int = 0;
            range = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < 32 as libc::c_int {
                if (*parsys).voice[i as usize].range as libc::c_int > range {
                    range = (*parsys).voice[i as usize].range as libc::c_int;
                }
                i += 1;
                i;
            }
            (*parsys)
                .voice[voice as usize]
                .range = (range + 1 as libc::c_int) as libc::c_schar;
            voice_link(p_voice);
        } else {
            (*p_voice).set_ignore(1 as libc::c_int as libc::c_uint);
            (*p_voice).cstaff = (nstaff + 1 as libc::c_int) as libc::c_uchar;
            (*p_voice).staff = (*p_voice).cstaff;
        }
    }
    if !((*s).u.voice.fname).is_null() {
        (*p_voice).nm = (*s).u.voice.fname;
        (*p_voice).set_new_name(1 as libc::c_int as libc::c_uint);
    }
    if !((*s).u.voice.nname).is_null() {
        (*p_voice).snm = (*s).u.voice.nname;
    }
    if (*s).u.voice.octave as libc::c_int != 10 as libc::c_int {
        (*p_voice).octave = (*s).u.voice.octave;
    }
    match (*s).u.voice.dyn_0 as libc::c_int {
        1 => {
            ((*p_voice).posit).set_dyn_0(0x1 as libc::c_int as libc::c_ushort);
            ((*p_voice).posit).set_vol(0x1 as libc::c_int as libc::c_ushort);
        }
        -1 => {
            ((*p_voice).posit).set_dyn_0(0x2 as libc::c_int as libc::c_ushort);
            ((*p_voice).posit).set_vol(0x2 as libc::c_int as libc::c_ushort);
        }
        _ => {}
    }
    match (*s).u.voice.lyrics as libc::c_int {
        1 => {
            ((*p_voice).posit).set_voc(0x1 as libc::c_int as libc::c_ushort);
        }
        -1 => {
            ((*p_voice).posit).set_voc(0x2 as libc::c_int as libc::c_ushort);
        }
        _ => {}
    }
    match (*s).u.voice.gchord as libc::c_int {
        1 => {
            ((*p_voice).posit).set_gch(0x1 as libc::c_int as libc::c_ushort);
        }
        -1 => {
            ((*p_voice).posit).set_gch(0x2 as libc::c_int as libc::c_ushort);
        }
        _ => {}
    }
    match (*s).u.voice.stem as libc::c_int {
        1 => {
            ((*p_voice).posit).set_std(0x1 as libc::c_int as libc::c_ushort);
        }
        -1 => {
            ((*p_voice).posit).set_std(0x2 as libc::c_int as libc::c_ushort);
        }
        2 => {
            ((*p_voice).posit).set_std(0 as libc::c_int as libc::c_ushort);
        }
        _ => {}
    }
    match (*s).u.voice.gstem as libc::c_int {
        1 => {
            ((*p_voice).posit).set_gsd(0x1 as libc::c_int as libc::c_ushort);
        }
        -1 => {
            ((*p_voice).posit).set_gsd(0x2 as libc::c_int as libc::c_ushort);
        }
        2 => {
            ((*p_voice).posit).set_gsd(0 as libc::c_int as libc::c_ushort);
        }
        _ => {}
    }
    if (*s).u.voice.scale != 0 as libc::c_int as libc::c_float {
        (*p_voice).scale = (*s).u.voice.scale;
    } else if (*s).u.voice.cue as libc::c_int > 0 as libc::c_int {
        (*p_voice).scale = 0.7f64 as libc::c_float;
    } else if ((*s).u.voice.cue as libc::c_int) < 0 as libc::c_int {
        (*p_voice).scale = 1 as libc::c_int as libc::c_float;
    }
    if !((*s).u.voice.stafflines).is_null() {
        (*p_voice).stafflines = (*s).u.voice.stafflines;
    }
    if (*s).u.voice.staffscale != 0 as libc::c_int as libc::c_float {
        (*p_voice).staffscale = (*s).u.voice.staffscale;
    }
    if (*p_voice).combine == 0 {
        (*p_voice).combine = cfmt.combinevoices as libc::c_schar;
    }
    set_tblt(p_voice);
    if (*s).state as libc::c_int == 2 as libc::c_int {
        curvoice = p_voice;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sort_pitch(mut s: *mut SYMBOL) {
    let mut i: libc::c_int = 0;
    let mut nx: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut v_note: note = note {
        len: 0,
        pit: 0,
        acc: 0,
        sl1: 0,
        sl2: 0,
        ti1: 0,
        hlen: 0,
        invisible: 0,
        shhd: 0.,
        shac: 0.,
        head: 0 as *mut libc::c_char,
        color: 0,
    };
    let mut new_order: [libc::c_uchar; 8] = [0; 8];
    let mut inv_order: [libc::c_uchar; 8] = [0; 8];
    i = 0 as libc::c_int;
    while i <= (*s).nhd as libc::c_int {
        new_order[i as usize] = i as libc::c_uchar;
        i += 1;
        i;
    }
    loop {
        nx = 0 as libc::c_int;
        i = 1 as libc::c_int;
        while i <= (*s).nhd as libc::c_int {
            if !((*s).u.note.notes[i as usize].pit as libc::c_int
                >= (*s).u.note.notes[(i - 1 as libc::c_int) as usize].pit as libc::c_int)
            {
                memcpy(
                    &mut v_note as *mut note as *mut libc::c_void,
                    &mut *((*s).u.note.notes).as_mut_ptr().offset(i as isize)
                        as *mut note as *const libc::c_void,
                    ::core::mem::size_of::<note>() as libc::c_ulong,
                );
                memcpy(
                    &mut *((*s).u.note.notes).as_mut_ptr().offset(i as isize)
                        as *mut note as *mut libc::c_void,
                    &mut *((*s).u.note.notes)
                        .as_mut_ptr()
                        .offset((i - 1 as libc::c_int) as isize) as *mut note
                        as *const libc::c_void,
                    ::core::mem::size_of::<note>() as libc::c_ulong,
                );
                memcpy(
                    &mut *((*s).u.note.notes)
                        .as_mut_ptr()
                        .offset((i - 1 as libc::c_int) as isize) as *mut note
                        as *mut libc::c_void,
                    &mut v_note as *mut note as *const libc::c_void,
                    ::core::mem::size_of::<note>() as libc::c_ulong,
                );
                k = (*s).pits[i as usize] as libc::c_int;
                (*s).pits[i as usize] = (*s).pits[(i - 1 as libc::c_int) as usize];
                (*s).pits[(i - 1 as libc::c_int) as usize] = k as libc::c_schar;
                k = new_order[i as usize] as libc::c_int;
                new_order[i as usize] = new_order[(i - 1 as libc::c_int) as usize];
                new_order[(i - 1 as libc::c_int) as usize] = k as libc::c_uchar;
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
    if (*s).nhd as libc::c_int > 0 as libc::c_int {
        i = 0 as libc::c_int;
        while i <= (*s).nhd as libc::c_int {
            inv_order[new_order[i as usize] as usize] = i as libc::c_uchar;
            i += 1;
            i;
        }
        i = 0 as libc::c_int;
        while i <= (*s).u.note.dc.n as libc::c_int {
            k = (*s).u.note.dc.tm[i as usize].m as libc::c_int;
            if k >= 0 as libc::c_int {
                (*s).u.note.dc.tm[i as usize].m = inv_order[k as usize] as libc::c_schar;
            }
            i += 1;
            i;
        }
    }
}
unsafe extern "C" fn set_map(mut s: *mut SYMBOL) {
    let mut map: *mut map = 0 as *mut map;
    let mut note_map: *mut note_map = 0 as *mut note_map;
    let mut note: *mut note = 0 as *mut note;
    let mut m: libc::c_int = 0;
    let mut delta: libc::c_int = 0;
    map = maps;
    while !map.is_null() {
        if strcmp((*map).name, (*curvoice).map_name) == 0 as libc::c_int {
            break;
        }
        map = (*map).next;
    }
    if map.is_null() {
        return;
    }
    delta = (*curvoice).ckey.key_delta as libc::c_int;
    m = 0 as libc::c_int;
    while m <= (*s).nhd as libc::c_int {
        note = &mut *((*s).u.note.notes).as_mut_ptr().offset(m as isize) as *mut note;
        let mut current_block_14: u64;
        note_map = (*map).notes;
        while !note_map.is_null() {
            match (*note_map).type_0 as libc::c_int {
                0 => {
                    if (*note).pit as libc::c_int == (*note_map).pit as libc::c_int
                        && (*note).acc as libc::c_int == (*note_map).acc as libc::c_int
                    {
                        current_block_14 = 5948590327928692120;
                    } else {
                        current_block_14 = 13109137661213826276;
                    }
                }
                1 => {
                    if ((*note).pit as libc::c_int - (*note_map).pit as libc::c_int
                        + 28 as libc::c_int) % 7 as libc::c_int == 0 as libc::c_int
                        && (*note).acc as libc::c_int == (*note_map).acc as libc::c_int
                    {
                        current_block_14 = 5948590327928692120;
                    } else {
                        current_block_14 = 13109137661213826276;
                    }
                }
                2 => {
                    if ((*note).pit as libc::c_int + 28 as libc::c_int - delta
                        - (*note_map).pit as libc::c_int) % 7 as libc::c_int
                        == 0 as libc::c_int
                    {
                        current_block_14 = 5948590327928692120;
                    } else {
                        current_block_14 = 13109137661213826276;
                    }
                }
                _ => {
                    current_block_14 = 5948590327928692120;
                }
            }
            match current_block_14 {
                13109137661213826276 => {
                    note_map = (*note_map).next;
                }
                _ => {
                    (*note).head = (*note_map).heads;
                    (*note).color = (*note_map).color;
                    if (*note_map).print_pit as libc::c_int != -(128 as libc::c_int) {
                        (*note).pit = (*note_map).print_pit;
                        (*s).pits[m as usize] = (*note).pit;
                        (*note).acc = (*note_map).print_acc;
                    }
                    break;
                }
            }
        }
        m += 1;
        m;
    }
}
unsafe extern "C" fn get_note(mut s: *mut SYMBOL) {
    let mut prev: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut i: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut delta: libc::c_int = 0;
    prev = (*curvoice).last_sym;
    m = (*s).nhd as libc::c_int;
    sym_link(
        s,
        if (*s).u.note.notes[0 as libc::c_int as usize].len != 0 as libc::c_int {
            1 as libc::c_int
        } else {
            2 as libc::c_int
        },
    );
    if (*s).flags as libc::c_int & 0x20 as libc::c_int == 0 {
        (*curvoice).time += (*s).dur;
    }
    if (*curvoice).octave != 0 {
        delta = (*curvoice).octave as libc::c_int * 7 as libc::c_int;
        i = 0 as libc::c_int;
        while i <= m {
            (*s)
                .u
                .note
                .notes[i as usize]
                .pit = ((*s).u.note.notes[i as usize].pit as libc::c_int + delta)
                as libc::c_schar;
            (*s)
                .pits[i
                as usize] = ((*s).pits[i as usize] as libc::c_int + delta)
                as libc::c_schar;
            i += 1;
            i;
        }
    }
    if (*curvoice).ottava != 0 {
        delta = (*curvoice).ottava as libc::c_int;
        i = 0 as libc::c_int;
        while i <= m {
            (*s)
                .pits[i
                as usize] = ((*s).pits[i as usize] as libc::c_int + delta)
                as libc::c_schar;
            i += 1;
            i;
        }
    }
    (*s).combine = (*curvoice).combine;
    (*s).color = (*curvoice).color;
    if (*curvoice).perc() != 0 {
        (*s).sflags |= 0x4000 as libc::c_int as libc::c_uint;
    } else if (*s).abc_type as libc::c_int == 4 as libc::c_int
        && (*curvoice).transpose as libc::c_int != 0 as libc::c_int
    {
        note_transpose(s);
    }
    if (*s).flags as libc::c_int & 0x20 as libc::c_int == 0 {
        match ((*curvoice).posit).std() as libc::c_int {
            1 => {
                (*s).stem = 1 as libc::c_int as libc::c_schar;
            }
            2 => {
                (*s).stem = -(1 as libc::c_int) as libc::c_schar;
            }
            3 => {
                (*s)
                    .flags = ((*s).flags as libc::c_int | 0x8 as libc::c_int)
                    as libc::c_ushort;
            }
            _ => {}
        }
    } else {
        let mut div: libc::c_int = 0;
        if (*curvoice).key.instr as libc::c_int != 1 as libc::c_int
            && (*curvoice).key.instr as libc::c_int != 2 as libc::c_int
            && pipeformat == 0
        {
            div = 2 as libc::c_int;
            if prev.is_null() || (*prev).flags as libc::c_int & 0x20 as libc::c_int == 0
            {
                if (*s).flags as libc::c_int & 0x40 as libc::c_int != 0 {
                    div = 1 as libc::c_int;
                }
            }
        } else {
            div = 4 as libc::c_int;
        }
        i = 0 as libc::c_int;
        while i <= m {
            (*s).u.note.notes[i as usize].len /= div;
            i += 1;
            i;
        }
        (*s).dur /= div;
        match ((*curvoice).posit).gsd() as libc::c_int {
            1 => {
                (*s).stem = 1 as libc::c_int as libc::c_schar;
            }
            2 => {
                (*s).stem = -(1 as libc::c_int) as libc::c_schar;
            }
            3 => {
                (*s).stem = 2 as libc::c_int as libc::c_schar;
            }
            _ => {}
        }
    }
    if (*s).u.note.dc.n as libc::c_int > 0 as libc::c_int {
        deco_cnv(&mut (*s).u.note.dc, s, prev);
    }
    (*s).nohdi2 = -(1 as libc::c_int) as libc::c_schar;
    (*s).nohdi1 = (*s).nohdi2;
    if (*s).abc_type as libc::c_int == 5 as libc::c_int {
        if (*s).dur == (*curvoice).wmeasure as libc::c_int {
            if (*s).dur < 1536 as libc::c_int * 2 as libc::c_int {
                (*s).u.note.notes[0 as libc::c_int as usize].len = 1536 as libc::c_int;
            } else if (*s).dur < 1536 as libc::c_int * 4 as libc::c_int {
                (*s)
                    .u
                    .note
                    .notes[0 as libc::c_int as usize]
                    .len = 1536 as libc::c_int * 2 as libc::c_int;
            } else {
                (*s)
                    .u
                    .note
                    .notes[0 as libc::c_int as usize]
                    .len = 1536 as libc::c_int * 4 as libc::c_int;
            }
        }
    } else {
        if (*s).flags as libc::c_int & 0x20 as libc::c_int == 0
            && !((*curvoice).map_name).is_null()
        {
            set_map(s);
        }
        sort_pitch(s);
    }
    if (*curvoice).auto_len() == 0
        || (*s).flags as libc::c_int & 0x20 as libc::c_int != 0
    {
        let mut head: libc::c_int = 0;
        let mut dots: libc::c_int = 0;
        let mut nflags: libc::c_int = 0;
        let mut l: libc::c_int = 0;
        l = (*s).u.note.notes[0 as libc::c_int as usize].len;
        if l != 0 as libc::c_int {
            identify_note(s, l, &mut head, &mut dots, &mut nflags);
            (*s).head = head as libc::c_uchar;
            (*s).dots = dots as libc::c_char;
            (*s).nflags = nflags as libc::c_schar;
            i = 1 as libc::c_int;
            while i <= m {
                if !((*s).u.note.notes[i as usize].len == l) {
                    identify_note(
                        s,
                        (*s).u.note.notes[i as usize].len,
                        &mut head,
                        &mut dots,
                        &mut nflags,
                    );
                    if head > (*s).head as libc::c_int {
                        (*s).head = head as libc::c_uchar;
                    }
                    if dots > (*s).dots as libc::c_int {
                        (*s).dots = dots as libc::c_char;
                    }
                    if nflags > (*s).nflags as libc::c_int {
                        (*s).nflags = nflags as libc::c_schar;
                    }
                }
                i += 1;
                i;
            }
            if (*s).sflags & 0x200 as libc::c_int as libc::c_uint != 0 {
                (*s).nflags = 0 as libc::c_int as libc::c_schar;
            }
        }
    }
    if (*s).nflags as libc::c_int <= -(2 as libc::c_int) {
        (*s).flags = ((*s).flags as libc::c_int | 0x8 as libc::c_int) as libc::c_ushort;
    }
    if (*s).sflags & (0x800000 as libc::c_int | 0x80 as libc::c_int) as libc::c_uint != 0
    {
        if (*s).nflags as libc::c_int > 0 as libc::c_int {
            (*s)
                .nflags = ((*s).nflags as libc::c_int + (*s).aux as libc::c_int)
                as libc::c_schar;
        } else {
            (*s).nflags = (*s).aux as libc::c_schar;
        }
        if (*s).sflags & 0x80 as libc::c_int as libc::c_uint != 0
            && (*s).sflags & 0x10 as libc::c_int as libc::c_uint != 0
        {
            (*prev).head = (*s).head;
            (*prev).aux = (*s).aux;
            (*prev).nflags = (*s).nflags;
            (*prev)
                .flags = ((*prev).flags as libc::c_int
                | (*s).flags as libc::c_int & 0x8 as libc::c_int) as libc::c_ushort;
        }
    }
    i = 0 as libc::c_int;
    while i <= m {
        if (*s).u.note.notes[i as usize].sl1 as libc::c_int != 0 as libc::c_int {
            (*s).sflags |= 0x800 as libc::c_int as libc::c_uint;
        }
        if (*s).u.note.notes[i as usize].sl2 as libc::c_int != 0 as libc::c_int {
            (*s).sflags |= 0x1000 as libc::c_int as libc::c_uint;
        }
        if (*s).u.note.notes[i as usize].ti1 as libc::c_int != 0 as libc::c_int {
            (*s).sflags |= 0x2000 as libc::c_int as libc::c_uint;
        }
        i += 1;
        i;
    }
    match cfmt.shiftunison {
        0 => {}
        1 => {
            (*s).sflags |= 0x2000000 as libc::c_int as libc::c_uint;
        }
        2 => {
            (*s).sflags |= 0x4000000 as libc::c_int as libc::c_uint;
        }
        _ => {
            (*s).sflags
                |= (0x2000000 as libc::c_int | 0x4000000 as libc::c_int) as libc::c_uint;
        }
    }
    if !((*s).text).is_null() {
        gch_build(s);
    }
}
unsafe extern "C" fn get_val(
    mut p: *mut libc::c_char,
    mut v: *mut libc::c_float,
) -> *mut libc::c_char {
    let mut tmp: [libc::c_char; 32] = [0; 32];
    let mut r: *mut libc::c_char = tmp.as_mut_ptr();
    while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
        p = p.offset(1);
        p;
    }
    while isdigit(*p as libc::c_uchar as libc::c_int) != 0
        && r
            < &mut *tmp
                .as_mut_ptr()
                .offset((32 as libc::c_int - 1 as libc::c_int) as isize)
                as *mut libc::c_char || *p as libc::c_int == '-' as i32
        || *p as libc::c_int == '.' as i32
    {
        let fresh21 = p;
        p = p.offset(1);
        let fresh22 = r;
        r = r.offset(1);
        *fresh22 = *fresh21;
    }
    *r = '\0' as i32 as libc::c_char;
    sscanf(tmp.as_mut_ptr(), b"%f\0" as *const u8 as *const libc::c_char, v);
    return p;
}
unsafe extern "C" fn parse_path(
    mut p: *mut libc::c_char,
    mut q: *mut libc::c_char,
    mut id: *mut libc::c_char,
    mut idsz: libc::c_int,
) {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut op: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut width: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut scale: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut trans: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut fill: libc::c_int = 0;
    let mut npar: libc::c_int = 0 as libc::c_int;
    let mut x1: libc::c_float = 0.;
    let mut y1: libc::c_float = 0.;
    let mut x: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    let mut rmax: *mut libc::c_char = 0 as *mut libc::c_char;
    r = strstr(p, b"class=\"\0" as *const u8 as *const libc::c_char);
    if r.is_null() || r > q {
        return;
    }
    r = r.offset(7 as libc::c_int as isize);
    fill = (strncmp(
        r,
        b"fill\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int) as libc::c_int;
    width = strstr(p, b"stroke-width:\0" as *const u8 as *const libc::c_char);
    scale = strstr(p, b"scale(\0" as *const u8 as *const libc::c_char);
    if !scale.is_null() && scale > q {
        scale = 0 as *mut libc::c_char;
    }
    trans = strstr(p, b"translate(\0" as *const u8 as *const libc::c_char);
    if !trans.is_null() && trans > q {
        trans = 0 as *mut libc::c_char;
    }
    loop {
        p = strstr(p, b"d=\"\0" as *const u8 as *const libc::c_char);
        if p.is_null() {
            return;
        }
        if isspace(
            *p.offset(-(1 as libc::c_int) as isize) as libc::c_uchar as libc::c_int,
        ) != 0
        {
            break;
        }
        p = p.offset(3 as libc::c_int as isize);
    }
    i = q.offset_from(p) as libc::c_long as libc::c_int * 4 as libc::c_int
        + 200 as libc::c_int;
    if i > 512 as libc::c_int {
        buf = malloc(i as libc::c_ulong) as *mut libc::c_char;
    } else {
        buf = tex_buf.as_mut_ptr();
    }
    rmax = buf.offset(i as isize);
    r = buf;
    let fresh23 = r;
    r = r.offset(1);
    *fresh23 = '/' as i32 as libc::c_char;
    idsz -= 5 as libc::c_int;
    strncpy(r, id.offset(4 as libc::c_int as isize), idsz as libc::c_ulong);
    r = r.offset(idsz as isize);
    strcpy(r, b"{gsave T \0" as *const u8 as *const libc::c_char);
    r = r.offset(strlen(r) as isize);
    if !scale.is_null() || !trans.is_null() {
        if !scale.is_null() {
            scale = scale.offset(6 as libc::c_int as isize);
            t = get_val(scale, &mut x1);
            if *t as libc::c_int == ',' as i32 {
                t = get_val(t.offset(1 as libc::c_int as isize), &mut y1);
            } else {
                y1 = x1;
            }
        }
        if !trans.is_null() {
            trans = trans.offset(10 as libc::c_int as isize);
            t = (get_val(trans, &mut x)).offset(1 as libc::c_int as isize);
            t = get_val(t, &mut y);
        }
        if scale.is_null() {
            r = r
                .offset(
                    sprintf(
                        r,
                        b"%.2f %.2f T \0" as *const u8 as *const libc::c_char,
                        x as libc::c_double,
                        -y as libc::c_double,
                    ) as isize,
                );
        } else if trans.is_null() {
            r = r
                .offset(
                    sprintf(
                        r,
                        b"%.2f %.2f scale \0" as *const u8 as *const libc::c_char,
                        x1 as libc::c_double,
                        y1 as libc::c_double,
                    ) as isize,
                );
        } else if scale > trans {
            r = r
                .offset(
                    sprintf(
                        r,
                        b"%.2f %.2f T %.2f %.2f scale \0" as *const u8
                            as *const libc::c_char,
                        x as libc::c_double,
                        -y as libc::c_double,
                        x1 as libc::c_double,
                        y1 as libc::c_double,
                    ) as isize,
                );
        } else {
            r = r
                .offset(
                    sprintf(
                        r,
                        b"%.2f %.2f scale %.2f %.2f T \0" as *const u8
                            as *const libc::c_char,
                        x1 as libc::c_double,
                        y1 as libc::c_double,
                        x as libc::c_double,
                        -y as libc::c_double,
                    ) as isize,
                );
        }
    }
    strcpy(r, b"0 0 M\n\0" as *const u8 as *const libc::c_char);
    r = r.offset(strlen(r) as isize);
    if !width.is_null() && width < q {
        let fresh24 = r;
        r = r.offset(1);
        *fresh24 = ' ' as i32 as libc::c_char;
        width = width.offset(13 as libc::c_int as isize);
        while isdigit(*width as libc::c_int) != 0 || *width as libc::c_int == '.' as i32
        {
            let fresh25 = width;
            width = width.offset(1);
            let fresh26 = r;
            r = r.offset(1);
            *fresh26 = *fresh25;
        }
        let fresh27 = r;
        r = r.offset(1);
        *fresh27 = ' ' as i32 as libc::c_char;
        let fresh28 = r;
        r = r.offset(1);
        *fresh28 = 'S' as i32 as libc::c_char;
        let fresh29 = r;
        r = r.offset(1);
        *fresh29 = 'L' as i32 as libc::c_char;
        let fresh30 = r;
        r = r.offset(1);
        *fresh30 = 'W' as i32 as libc::c_char;
    }
    p = p.offset(3 as libc::c_int as isize);
    while !(*p as libc::c_int == '\0' as i32 || *p as libc::c_int == '"' as i32) {
        i = 0 as libc::c_int;
        let fresh31 = p;
        p = p.offset(1);
        match *fresh31 as libc::c_int {
            77 => {
                op = b"M\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                npar = 2 as libc::c_int;
            }
            109 => {
                op = b"RM\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                npar = 2 as libc::c_int;
            }
            76 => {
                op = b"L\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                npar = 2 as libc::c_int;
            }
            108 => {
                op = b"RL\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                npar = 2 as libc::c_int;
            }
            72 => {
                op = b"H\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                npar = 1 as libc::c_int;
            }
            104 => {
                op = b"h\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                npar = 1 as libc::c_int;
            }
            86 => {
                op = b"V\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                npar = 1 as libc::c_int;
            }
            118 => {
                let fresh32 = r;
                r = r.offset(1);
                *fresh32 = ' ' as i32 as libc::c_char;
                let fresh33 = r;
                r = r.offset(1);
                *fresh33 = '0' as i32 as libc::c_char;
                op = b"RL\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                i = 1 as libc::c_int;
                npar = 2 as libc::c_int;
            }
            122 => {
                op = b"closepath\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
                npar = 0 as libc::c_int;
            }
            67 => {
                op = b"C\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                npar = 6 as libc::c_int;
            }
            99 => {
                op = b"RC\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                npar = 6 as libc::c_int;
            }
            113 => {
                op = b"RC\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                npar = 2 as libc::c_int;
                p = get_val(p, &mut x1);
                p = get_val(p, &mut y1);
                t = get_val(p, &mut x);
                t = get_val(t, &mut y);
                r = r
                    .offset(
                        sprintf(
                            r,
                            b" %.2f %.2f %.2f %.2f\0" as *const u8
                                as *const libc::c_char,
                            (x1 * 2 as libc::c_int as libc::c_float
                                / 3 as libc::c_int as libc::c_float) as libc::c_double,
                            (-y1 * 2 as libc::c_int as libc::c_float
                                / 3 as libc::c_int as libc::c_float) as libc::c_double,
                            (x1
                                + (x - x1) * 2 as libc::c_int as libc::c_float
                                    / 3 as libc::c_int as libc::c_float) as libc::c_double,
                            (-y1
                                - (y - y1) * 2 as libc::c_int as libc::c_float
                                    / 3 as libc::c_int as libc::c_float) as libc::c_double,
                        ) as isize,
                    );
            }
            116 => {
                op = b"RC\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                npar = 2 as libc::c_int;
                x1 = x - x1;
                y1 = y - y1;
                t = get_val(p, &mut x);
                t = get_val(t, &mut y);
                r = r
                    .offset(
                        sprintf(
                            r,
                            b" %.2f %.2f %.2f %.2f\0" as *const u8
                                as *const libc::c_char,
                            (x1 * 2 as libc::c_int as libc::c_float
                                / 3 as libc::c_int as libc::c_float) as libc::c_double,
                            (-y1 * 2 as libc::c_int as libc::c_float
                                / 3 as libc::c_int as libc::c_float) as libc::c_double,
                            (x1
                                + (x - x1) * 2 as libc::c_int as libc::c_float
                                    / 3 as libc::c_int as libc::c_float) as libc::c_double,
                            (-y1
                                - (y - y1) * 2 as libc::c_int as libc::c_float
                                    / 3 as libc::c_int as libc::c_float) as libc::c_double,
                        ) as isize,
                    );
            }
            _ => {
                if !(isdigit(
                    *p.offset(-(1 as libc::c_int) as isize) as libc::c_uchar
                        as libc::c_int,
                ) != 0
                    || *p.offset(-(1 as libc::c_int) as isize) as libc::c_int
                        == '-' as i32
                    || *p.offset(-(1 as libc::c_int) as isize) as libc::c_int
                        == '.' as i32)
                {
                    continue;
                }
                if npar == 0 {
                    continue;
                }
                p = p.offset(-1);
                p;
            }
        }
        let fresh34 = r;
        r = r.offset(1);
        *fresh34 = ' ' as i32 as libc::c_char;
        while i < npar {
            while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
                p = p.offset(1);
                p;
            }
            if i & 1 as libc::c_int != 0 {
                if *p as libc::c_int == '-' as i32 {
                    p = p.offset(1);
                    p;
                } else if *p as libc::c_int != '0' as i32
                    || *p.offset(1 as libc::c_int as isize) as libc::c_int != ' ' as i32
                {
                    let fresh35 = r;
                    r = r.offset(1);
                    *fresh35 = '-' as i32 as libc::c_char;
                }
            }
            while isdigit(*p as libc::c_uchar as libc::c_int) != 0
                || *p as libc::c_int == '-' as i32 || *p as libc::c_int == '.' as i32
            {
                let fresh36 = p;
                p = p.offset(1);
                let fresh37 = r;
                r = r.offset(1);
                *fresh37 = *fresh36;
            }
            let fresh38 = r;
            r = r.offset(1);
            *fresh38 = ' ' as i32 as libc::c_char;
            i += 1;
            i;
        }
        if *op as libc::c_int == 'h' as i32 {
            let fresh39 = r;
            r = r.offset(1);
            *fresh39 = '0' as i32 as libc::c_char;
            let fresh40 = r;
            r = r.offset(1);
            *fresh40 = ' ' as i32 as libc::c_char;
            op = b"RL\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        strcpy(r, op);
        r = r.offset(strlen(r) as isize);
        if r.offset(30 as libc::c_int as isize) > rmax {
            bug(
                b"Buffer overflow in SVG to PS\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                1 as libc::c_int,
            );
        }
    }
    strcpy(
        r,
        if fill != 0 {
            b" fill\0" as *const u8 as *const libc::c_char
        } else {
            b" stroke\0" as *const u8 as *const libc::c_char
        },
    );
    r = r.offset(strlen(r) as isize);
    strcpy(r, b"\ngrestore}!\0" as *const u8 as *const libc::c_char);
    r = r.offset(strlen(r) as isize);
    s = getarena(::core::mem::size_of::<SYMBOL>() as libc::c_ulong as libc::c_int)
        as *mut SYMBOL;
    memset(
        s as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<SYMBOL>() as libc::c_ulong,
    );
    (*s)
        .text = getarena(
        (strlen(buf)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
    ) as *mut libc::c_char;
    strcpy((*s).text, buf);
    ps_def(s, (*s).text, 'p' as i32 as libc::c_char);
    if buf != tex_buf.as_mut_ptr() {
        free(buf as *mut libc::c_void);
    }
}
unsafe extern "C" fn parse_defs(mut p: *mut libc::c_char, mut q: *mut libc::c_char) {
    let mut id: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut idsz: libc::c_int = 0;
    loop {
        id = strstr(p, b"id=\"\0" as *const u8 as *const libc::c_char);
        if id.is_null() || id > q {
            return;
        }
        r = strchr(id.offset(4 as libc::c_int as isize), '"' as i32);
        if r.is_null() {
            return;
        }
        idsz = r.offset(1 as libc::c_int as isize).offset_from(id) as libc::c_long
            as libc::c_int;
        if svg != 0 || epsf > 1 as libc::c_int {
            svg_def_id(id, idsz);
            p = r;
        } else {
            p = id;
            while *p as libc::c_int != '<' as i32 {
                p = p.offset(-1);
                p;
            }
            if !(strncmp(
                p,
                b"<path \0" as *const u8 as *const libc::c_char,
                6 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int)
            {
                break;
            }
            r = strstr(p, b"/>\0" as *const u8 as *const libc::c_char);
            parse_path(p.offset(6 as libc::c_int as isize), r, id, idsz);
            if r.is_null() {
                break;
            }
            p = r.offset(2 as libc::c_int as isize);
        }
    };
}
unsafe extern "C" fn svg_ps(mut p: *mut libc::c_char) {
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    loop {
        q = strstr(p, b"<defs>\0" as *const u8 as *const libc::c_char);
        if q.is_null() {
            break;
        }
        p = strstr(q, b"</defs>\0" as *const u8 as *const libc::c_char);
        if p.is_null() {
            error(
                1 as libc::c_int,
                0 as *mut SYMBOL,
                b"No </defs> in %%beginsvg\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            break;
        } else {
            parse_defs(q.offset(6 as libc::c_int as isize), p);
        }
    };
}
unsafe extern "C" fn ps_def(
    mut s: *mut SYMBOL,
    mut p: *mut libc::c_char,
    mut use_0: libc::c_char,
) {
    if svg == 0 && epsf <= 1 as libc::c_int {
        if secure != 0 || use_0 as libc::c_int == 's' as i32 {
            return;
        }
    } else if use_0 as libc::c_int == 'p' as i32
        || use_0 as libc::c_int == 'g' as i32 && file_initialized > 0 as libc::c_int
    {
        return
    }
    if !((*s).abc_prev).is_null() {
        (*s).state = (*(*s).abc_prev).state;
    }
    if (*s).state as libc::c_int == 2 as libc::c_int {
        if use_0 as libc::c_int == 'g' as i32 {
            return;
        }
        sym_link(s, 12 as libc::c_int);
        (*s).aux = 0 as libc::c_int as libc::c_short;
        (*s).text = p;
        return;
    }
    if use_0 as libc::c_int == 'g' as i32 {
        svg_ps(p);
        if svg == 0 && epsf <= 1 as libc::c_int {
            return;
        }
    }
    if file_initialized > 0 as libc::c_int || mbf != outbuf {
        a2b(b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char, p);
    } else {
        user_ps_add(p, use_0);
    };
}
unsafe extern "C" fn get_symsel(
    mut symsel: *mut symsel_s,
    mut p: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tn: libc::c_int = 0;
    let mut td: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    (*symsel).bar = strtod(p, &mut q) as libc::c_short;
    if *q as libc::c_int >= 'a' as i32 && *q as libc::c_int <= 'z' as i32 {
        let fresh41 = q;
        q = q.offset(1);
        (*symsel).seq = (*fresh41 as libc::c_int - 'a' as i32) as libc::c_char;
    } else {
        (*symsel).seq = 0 as libc::c_int as libc::c_char;
    }
    if *q as libc::c_int == ':' as i32 {
        if sscanf(
            q.offset(1 as libc::c_int as isize),
            b"%d/%d%n\0" as *const u8 as *const libc::c_char,
            &mut tn as *mut libc::c_int,
            &mut td as *mut libc::c_int,
            &mut n as *mut libc::c_int,
        ) != 2 as libc::c_int || td <= 0 as libc::c_int
        {
            return 0 as *mut libc::c_char;
        }
        (*symsel).time = (1536 as libc::c_int * tn / td) as libc::c_short;
        q = q.offset((1 as libc::c_int + n) as isize);
    } else {
        (*symsel).time = 0 as libc::c_int as libc::c_short;
    }
    return q;
}
unsafe extern "C" fn free_voice_opt(mut opt: *mut voice_opt_s) {
    let mut opt2: *mut voice_opt_s = 0 as *mut voice_opt_s;
    while !opt.is_null() {
        opt2 = (*opt).next;
        free(opt as *mut libc::c_void);
        opt = opt2;
    }
}
unsafe extern "C" fn get_color(mut p: *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut color: libc::c_int = 0;
    static mut col_tb: [C2RustUnnamed_12; 16] = [
        {
            let mut init = C2RustUnnamed_12 {
                name: b"aqua\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                color: 0xffff as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_12 {
                name: b"black\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                color: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_12 {
                name: b"blue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                color: 0xff as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_12 {
                name: b"fuchsia\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                color: 0xff00ff as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_12 {
                name: b"gray\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                color: 0x808080 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_12 {
                name: b"green\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                color: 0x8000 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_12 {
                name: b"lime\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                color: 0xff00 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_12 {
                name: b"maroon\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                color: 0x800000 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_12 {
                name: b"navy\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                color: 0x80 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_12 {
                name: b"olive\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                color: 0x808000 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_12 {
                name: b"purple\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                color: 0x800080 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_12 {
                name: b"red\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                color: 0xff0000 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_12 {
                name: b"silver\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                color: 0xc0c0c0 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_12 {
                name: b"teal\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                color: 0x8080 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_12 {
                name: b"white\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                color: 0xffffff as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_12 {
                name: b"yellow\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                color: 0xffff00 as libc::c_int,
            };
            init
        },
    ];
    if *p as libc::c_int == '#' as i32 {
        if sscanf(
            p,
            b"#%06x\0" as *const u8 as *const libc::c_char,
            &mut color as *mut libc::c_int,
        ) != 1 as libc::c_int
            || color as libc::c_uint > 0xffffff as libc::c_int as libc::c_uint
        {
            return -(1 as libc::c_int);
        }
        return color;
    }
    i = (::core::mem::size_of::<[C2RustUnnamed_12; 16]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<C2RustUnnamed_12>() as libc::c_ulong)
        as libc::c_int;
    loop {
        i -= 1;
        if !(i >= 0 as libc::c_int) {
            break;
        }
        if strncasecmp(p, col_tb[i as usize].name, strlen(col_tb[i as usize].name))
            == 0 as libc::c_int
        {
            break;
        }
    }
    if i < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return col_tb[i as usize].color;
}
unsafe extern "C" fn get_transpose(mut p: *mut libc::c_char) -> libc::c_int {
    let mut val: libc::c_int = 0;
    let mut pit1: libc::c_int = 0;
    let mut pit2: libc::c_int = 0;
    let mut acc: libc::c_int = 0;
    static mut pit_st: [libc::c_int; 7] = [
        0 as libc::c_int,
        2 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        7 as libc::c_int,
        9 as libc::c_int,
        11 as libc::c_int,
    ];
    if isdigit(*p as libc::c_int) != 0 || *p as libc::c_int == '-' as i32
        || *p as libc::c_int == '+' as i32
    {
        sscanf(
            p,
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut val as *mut libc::c_int,
        );
        val *= 3 as libc::c_int;
        match *p
            .offset((strlen(p)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int
        {
            35 => {
                val += 1;
                val;
            }
            98 => {
                val += 2 as libc::c_int;
            }
            _ => return val,
        }
        if val > 0 as libc::c_int {
            return val;
        }
        return val - 3 as libc::c_int;
    }
    p = parse_acc_pit(p, &mut pit1, &mut acc);
    if acc < 0 as libc::c_int {
        error(
            1 as libc::c_int,
            0 as *mut SYMBOL,
            b"  in %%%%transpose\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return 0 as libc::c_int;
    }
    pit1 += 126 as libc::c_int - 2 as libc::c_int;
    pit1 = pit1 / 7 as libc::c_int * 12 as libc::c_int
        + pit_st[(pit1 % 7 as libc::c_int) as usize];
    match acc {
        4 => {
            pit1 += 2 as libc::c_int;
        }
        1 => {
            pit1 += 1;
            pit1;
        }
        3 => {
            pit1 -= 1;
            pit1;
        }
        5 => {
            pit1 -= 2 as libc::c_int;
        }
        _ => {}
    }
    p = parse_acc_pit(p, &mut pit2, &mut acc);
    if acc < 0 as libc::c_int {
        error(
            1 as libc::c_int,
            0 as *mut SYMBOL,
            b"  in %%%%transpose\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return 0 as libc::c_int;
    }
    pit2 += 126 as libc::c_int - 2 as libc::c_int;
    pit2 = pit2 / 7 as libc::c_int * 12 as libc::c_int
        + pit_st[(pit2 % 7 as libc::c_int) as usize];
    match acc {
        4 => {
            pit2 += 2 as libc::c_int;
        }
        1 => {
            pit2 += 1;
            pit2;
        }
        3 => {
            pit2 -= 1;
            pit2;
        }
        5 => {
            pit2 -= 2 as libc::c_int;
        }
        _ => {}
    }
    val = (pit2 - pit1) * 3 as libc::c_int;
    match acc {
        4 | 1 => {
            val += 1;
            val;
        }
        3 | 5 => {
            val += 2 as libc::c_int;
        }
        _ => return val,
    }
    if val > 0 as libc::c_int {
        return val;
    }
    return val - 3 as libc::c_int;
}
unsafe extern "C" fn get_map(mut p: *mut libc::c_char) {
    let mut map: *mut map = 0 as *mut map;
    let mut note_map: *mut note_map = 0 as *mut note_map;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut pit: libc::c_int = 0;
    let mut acc: libc::c_int = 0;
    if *p as libc::c_int == '\0' as i32 {
        return;
    }
    name = p;
    while isspace(*p as libc::c_uchar as libc::c_int) == 0
        && *p as libc::c_int != '\0' as i32
    {
        p = p.offset(1);
        p;
    }
    l = p.offset_from(name) as libc::c_long as libc::c_int;
    while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
        p = p.offset(1);
        p;
    }
    if *p as libc::c_int == '*' as i32 {
        type_0 = 3 as libc::c_int;
        p = p.offset(1);
        p;
    } else if strncmp(
        p,
        b"octave,\0" as *const u8 as *const libc::c_char,
        7 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        type_0 = 1 as libc::c_int;
        p = p.offset(7 as libc::c_int as isize);
    } else if strncmp(
        p,
        b"key,\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        type_0 = 2 as libc::c_int;
        p = p.offset(4 as libc::c_int as isize);
    } else if strncmp(
        p,
        b"all\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        type_0 = 3 as libc::c_int;
        while isspace(*p as libc::c_uchar as libc::c_int) == 0
            && *p as libc::c_int != '\0' as i32
        {
            p = p.offset(1);
            p;
        }
    } else {
        type_0 = 0 as libc::c_int;
    }
    if type_0 != 3 as libc::c_int {
        p = parse_acc_pit(p, &mut pit, &mut acc);
        if acc < 0 as libc::c_int {
            acc = 0 as libc::c_int;
            pit = acc;
        }
        if type_0 == 1 as libc::c_int || type_0 == 2 as libc::c_int {
            pit %= 7 as libc::c_int;
            if type_0 == 2 as libc::c_int {
                acc = A_NULL as libc::c_int;
            }
        }
    } else {
        acc = 0 as libc::c_int;
        pit = acc;
    }
    map = maps;
    while !map.is_null() {
        if strncmp(name, (*map).name, l as libc::c_ulong) == 0 as libc::c_int {
            break;
        }
        map = (*map).next;
    }
    if map.is_null() {
        map = getarena(::core::mem::size_of::<map>() as libc::c_ulong as libc::c_int)
            as *mut map;
        (*map).next = maps;
        maps = map;
        (*map).name = getarena(l + 1 as libc::c_int) as *mut libc::c_char;
        strncpy((*map).name, name, l as libc::c_ulong);
        *((*map).name).offset(l as isize) = '\0' as i32 as libc::c_char;
        (*map).notes = 0 as *mut note_map;
    }
    note_map = (*map).notes;
    while !note_map.is_null() {
        if (*note_map).type_0 as libc::c_int == type_0
            && (*note_map).pit as libc::c_int == pit
            && (*note_map).acc as libc::c_int == acc
        {
            break;
        }
        note_map = (*note_map).next;
    }
    if note_map.is_null() {
        note_map = getarena(
            ::core::mem::size_of::<note_map>() as libc::c_ulong as libc::c_int,
        ) as *mut note_map;
        memset(
            note_map as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<note_map>() as libc::c_ulong,
        );
        (*note_map).next = (*map).notes;
        (*map).notes = note_map;
        (*note_map).type_0 = type_0 as libc::c_char;
        (*note_map).pit = pit as libc::c_schar;
        (*note_map).acc = acc as libc::c_uchar;
        (*note_map).print_pit = -(128 as libc::c_int) as libc::c_schar;
        (*note_map).color = -(1 as libc::c_int);
    }
    while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
        p = p.offset(1);
        p;
    }
    if *p as libc::c_int == '\0' as i32 {
        return;
    }
    q = p;
    while isspace(*q as libc::c_uchar as libc::c_int) == 0
        && *q as libc::c_int != '\0' as i32
    {
        if *q as libc::c_int == '=' as i32 {
            break;
        }
        q = q.offset(1);
        q;
    }
    if isspace(*q as libc::c_uchar as libc::c_int) != 0
        || *q as libc::c_int == '\0' as i32
    {
        if *p as libc::c_int != '*' as i32 {
            p = parse_acc_pit(p, &mut pit, &mut acc);
            if acc >= 0 as libc::c_int {
                (*note_map).print_pit = pit as libc::c_schar;
                (*note_map).print_acc = acc as libc::c_uchar;
            }
            if *p as libc::c_int == '\0' as i32 {
                return;
            }
        }
        p = q;
        while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
            p = p.offset(1);
            p;
        }
        if *p as libc::c_int == '\0' as i32 {
            return;
        }
        q = p;
        while isspace(*q as libc::c_uchar as libc::c_int) == 0
            && *q as libc::c_int != '\0' as i32
        {
            if *q as libc::c_int == '=' as i32 {
                break;
            }
            q = q.offset(1);
            q;
        }
        if isspace(*q as libc::c_uchar as libc::c_int) != 0
            || *q as libc::c_int == '\0' as i32
        {
            name = p;
            p = q;
            l = p.offset_from(name) as libc::c_long as libc::c_int;
            (*note_map).heads = getarena(l + 1 as libc::c_int) as *mut libc::c_char;
            strncpy((*note_map).heads, name, l as libc::c_ulong);
            *((*note_map).heads).offset(l as isize) = '\0' as i32 as libc::c_char;
        }
    }
    loop {
        while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
            p = p.offset(1);
            p;
        }
        if *p as libc::c_int == '\0' as i32 {
            break;
        }
        if strncmp(
            p,
            b"heads=\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            p = p.offset(6 as libc::c_int as isize);
            name = p;
            while isspace(*p as libc::c_uchar as libc::c_int) == 0
                && *p as libc::c_int != '\0' as i32
            {
                p = p.offset(1);
                p;
            }
            l = p.offset_from(name) as libc::c_long as libc::c_int;
            (*note_map).heads = getarena(l + 1 as libc::c_int) as *mut libc::c_char;
            strncpy((*note_map).heads, name, l as libc::c_ulong);
            *((*note_map).heads).offset(l as isize) = '\0' as i32 as libc::c_char;
        } else if strncmp(
            p,
            b"print=\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            p = p.offset(6 as libc::c_int as isize);
            p = parse_acc_pit(p, &mut pit, &mut acc);
            if acc >= 0 as libc::c_int {
                (*note_map).print_pit = pit as libc::c_schar;
                (*note_map).print_acc = acc as libc::c_uchar;
            }
        } else if strncmp(
            p,
            b"color=\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            let mut color: libc::c_int = 0;
            color = get_color(p.offset(6 as libc::c_int as isize));
            if color < 0 as libc::c_int {
                error(
                    1 as libc::c_int,
                    0 as *mut SYMBOL,
                    b"Bad color in %%%%map\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                return;
            }
            (*note_map).color = color;
        }
        while isspace(*p as libc::c_uchar as libc::c_int) == 0
            && *p as libc::c_int != '\0' as i32
        {
            p = p.offset(1);
            p;
        }
    };
}
unsafe extern "C" fn process_pscomment(mut s: *mut SYMBOL) -> *mut SYMBOL {
    let mut w: [libc::c_char; 32] = [0; 32];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut voice: libc::c_int = 0;
    let mut h1: libc::c_float = 0.;
    let mut lock: libc::c_int = 0 as libc::c_int;
    p = ((*s).text).offset(2 as libc::c_int as isize);
    q = p.offset(strlen(p) as isize).offset(-(5 as libc::c_int as isize));
    if q > p
        && strncmp(
            q,
            b" lock\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        lock = 1 as libc::c_int;
        *q = '\0' as i32 as libc::c_char;
    }
    p = get_str(
        w.as_mut_ptr(),
        p,
        ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
    );
    if (*s).state as libc::c_int == 1 as libc::c_int && check_header(s) == 0 {
        error(
            1 as libc::c_int,
            s,
            b"Cannot have %%%%%s in tune header\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            w.as_mut_ptr(),
        );
        return s;
    }
    let mut job_0: libc::c_int = 0;
    let mut current_block_685: u64;
    match w[0 as libc::c_int as usize] as libc::c_int {
        98 => {
            if strcmp(w.as_mut_ptr(), b"beginps\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
                || strcmp(
                    w.as_mut_ptr(),
                    b"beginsvg\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                let mut use_0: libc::c_char = 0;
                if w[5 as libc::c_int as usize] as libc::c_int == 'p' as i32 {
                    if strncmp(
                        p,
                        b"svg\0" as *const u8 as *const libc::c_char,
                        3 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        use_0 = 's' as i32 as libc::c_char;
                    } else if strncmp(
                        p,
                        b"nosvg\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        use_0 = 'p' as i32 as libc::c_char;
                    } else {
                        use_0 = 'b' as i32 as libc::c_char;
                    }
                } else {
                    use_0 = 'g' as i32 as libc::c_char;
                }
                p = ((*s).text)
                    .offset(2 as libc::c_int as isize)
                    .offset(7 as libc::c_int as isize);
                while *p as libc::c_int != '\0' as i32
                    && *p as libc::c_int != '\n' as i32
                {
                    p = p.offset(1);
                    p;
                }
                if *p as libc::c_int == '\0' as i32 {
                    return s;
                }
                ps_def(s, p.offset(1 as libc::c_int as isize), use_0);
                return s;
            }
            if strcmp(w.as_mut_ptr(), b"begintext\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                let mut job: libc::c_int = 0;
                if (*s).state as libc::c_int == 2 as libc::c_int {
                    if multicol_start == 0. {
                        gen_ly(1 as libc::c_int);
                    }
                } else if (*s).state as libc::c_int == 0 as libc::c_int {
                    if epsf != 0 || in_fname.is_null() {
                        return s;
                    }
                }
                p = ((*s).text)
                    .offset(2 as libc::c_int as isize)
                    .offset(9 as libc::c_int as isize);
                while *p as libc::c_int == ' ' as i32 || *p as libc::c_int == '\t' as i32
                {
                    p = p.offset(1);
                    p;
                }
                if *p as libc::c_int != '\n' as i32 {
                    job = get_textopt(p);
                    while *p as libc::c_int != '\0' as i32
                        && *p as libc::c_int != '\n' as i32
                    {
                        p = p.offset(1);
                        p;
                    }
                    if *p as libc::c_int == '\0' as i32 {
                        return s;
                    }
                } else {
                    job = cfmt.textoption;
                }
                if job != 4 as libc::c_int {
                    p = p.offset(1);
                    p;
                    write_text(w.as_mut_ptr(), p, job);
                }
                return s;
            }
            if strcmp(w.as_mut_ptr(), b"break\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                let mut brk: *mut brk_s = 0 as *mut brk_s;
                if (*s).state as libc::c_int != 1 as libc::c_int {
                    error(
                        1 as libc::c_int,
                        s,
                        b"%%%%%s ignored\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        w.as_mut_ptr(),
                    );
                    return s;
                }
                if *p as libc::c_int == '\0' as i32 {
                    return s;
                }
                loop {
                    brk = malloc(::core::mem::size_of::<brk_s>() as libc::c_ulong)
                        as *mut brk_s;
                    p = get_symsel(&mut (*brk).symsel, p);
                    if p.is_null() {
                        error(
                            1 as libc::c_int,
                            s,
                            b"Bad selection in %%%%%s\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            w.as_mut_ptr(),
                        );
                        return s;
                    }
                    (*brk).next = brks;
                    brks = brk;
                    if *p as libc::c_int != ',' as i32 && *p as libc::c_int != ' ' as i32
                    {
                        break;
                    }
                    p = p.offset(1);
                    p;
                }
                return s;
            }
            current_block_685 = 3073279508649792450;
        }
        99 => {
            if strcmp(w.as_mut_ptr(), b"center\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                current_block_685 = 15489970745984101349;
            } else {
                if strcmp(w.as_mut_ptr(), b"clef\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    if (*s).state as libc::c_int != 0 as libc::c_int {
                        clef_def(s);
                    }
                    return s;
                }
                if strcmp(w.as_mut_ptr(), b"clip\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    if cur_tune_opts.is_null() {
                        error(
                            1 as libc::c_int,
                            s,
                            b"%%%%%s not in %%%%tune sequence\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            w.as_mut_ptr(),
                        );
                        return s;
                    }
                    if *p as libc::c_int != '-' as i32 {
                        p = get_symsel(&mut clip_start, p);
                        if p.is_null() {
                            error(
                                1 as libc::c_int,
                                s,
                                b"Bad start in %%%%%s\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                w.as_mut_ptr(),
                            );
                            return s;
                        }
                        if *p as libc::c_int != '-' as i32 {
                            error(
                                1 as libc::c_int,
                                s,
                                b"Lack of '-' in %%%%%s\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                                w.as_mut_ptr(),
                            );
                            return s;
                        }
                    }
                    p = p.offset(1);
                    p;
                    p = get_symsel(&mut clip_end, p);
                    if p.is_null() {
                        error(
                            1 as libc::c_int,
                            s,
                            b"Bad end in %%%%%s\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            w.as_mut_ptr(),
                        );
                        return s;
                    }
                    if (clip_start.bar as libc::c_int) < 0 as libc::c_int {
                        clip_start.bar = 0 as libc::c_int as libc::c_short;
                    }
                    if (clip_end.bar as libc::c_int) < clip_start.bar as libc::c_int
                        || clip_end.bar as libc::c_int == clip_start.bar as libc::c_int
                            && clip_end.time as libc::c_int
                                <= clip_start.time as libc::c_int
                    {
                        clip_end
                            .bar = (!(0 as libc::c_int) as libc::c_ushort as libc::c_int
                            >> 1 as libc::c_int) as libc::c_short;
                    }
                    return s;
                }
                current_block_685 = 3073279508649792450;
            }
        }
        100 => {
            if strcmp(w.as_mut_ptr(), b"deco\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                deco_add(p);
                return s;
            }
            if strcmp(w.as_mut_ptr(), b"dynamic\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                set_voice_param(curvoice, (*s).state as libc::c_int, w.as_mut_ptr(), p);
                return s;
            }
            current_block_685 = 3073279508649792450;
        }
        69 => {
            if strcmp(w.as_mut_ptr(), b"EPS\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                let mut x1: libc::c_float = 0.;
                let mut y1: libc::c_float = 0.;
                let mut x2: libc::c_float = 0.;
                let mut y2: libc::c_float = 0.;
                let mut fp: *mut FILE = 0 as *mut FILE;
                let mut fn_0: [libc::c_char; 256] = [0; 256];
                let mut line: [libc::c_char; 256] = [0; 256];
                gen_ly(1 as libc::c_int);
                if secure != 0 || cfmt.textoption == 4 as libc::c_int {
                    return s;
                }
                get_str(
                    line.as_mut_ptr(),
                    p,
                    ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                        as libc::c_int,
                );
                fp = open_file(
                    line.as_mut_ptr(),
                    b"eps\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    fn_0.as_mut_ptr(),
                );
                if fp.is_null() {
                    error(
                        1 as libc::c_int,
                        s,
                        b"No such file: %s\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        line.as_mut_ptr(),
                    );
                    return s;
                }
                x2 = 0 as libc::c_int as libc::c_float;
                x1 = x2;
                while !(fgets(
                    line.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                        as libc::c_int,
                    fp,
                ))
                    .is_null()
                {
                    if !(strncmp(
                        line.as_mut_ptr(),
                        b"%%BoundingBox:\0" as *const u8 as *const libc::c_char,
                        14 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int)
                    {
                        continue;
                    }
                    if sscanf(
                        &mut *line.as_mut_ptr().offset(14 as libc::c_int as isize)
                            as *mut libc::c_char,
                        b"%f %f %f %f\0" as *const u8 as *const libc::c_char,
                        &mut x1 as *mut libc::c_float,
                        &mut y1 as *mut libc::c_float,
                        &mut x2 as *mut libc::c_float,
                        &mut y2 as *mut libc::c_float,
                    ) == 4 as libc::c_int
                    {
                        break;
                    }
                }
                fclose(fp);
                if x1 == x2 {
                    error(
                        1 as libc::c_int,
                        s,
                        b"No bounding box in '%s'\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        fn_0.as_mut_ptr(),
                    );
                    return s;
                }
                if cfmt.textoption == 3 as libc::c_int
                    || cfmt.textoption == 5 as libc::c_int
                {
                    let mut lw: libc::c_float = 0.;
                    lw = ((if cfmt.landscape != 0 {
                        cfmt.pageheight
                    } else {
                        cfmt.pagewidth
                    }) - cfmt.leftmargin - cfmt.rightmargin) / cfmt.scale;
                    if cfmt.textoption == 3 as libc::c_int {
                        x1 = (x1 as libc::c_double
                            + (lw - (x2 - x1)) as libc::c_double * 0.5f64)
                            as libc::c_float;
                    } else {
                        x1 += lw - (x2 - x1);
                    }
                }
                a2b(b"\x01\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
                bskip(y2 - y1);
                a2b(
                    b"%.2f %.2f%%%s\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    x1 as libc::c_double,
                    -y1 as libc::c_double,
                    fn_0.as_mut_ptr(),
                );
                buffer_eob(0 as libc::c_int);
                return s;
            }
            current_block_685 = 3073279508649792450;
        }
        103 => {
            if strcmp(w.as_mut_ptr(), b"gchord\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
                || strcmp(
                    w.as_mut_ptr(),
                    b"gstemdir\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                set_voice_param(curvoice, (*s).state as libc::c_int, w.as_mut_ptr(), p);
                return s;
            }
            if strcmp(w.as_mut_ptr(), b"glyph\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                if svg == 0 && epsf <= 1 as libc::c_int {
                    glyph_add(p);
                }
                return s;
            }
            current_block_685 = 3073279508649792450;
        }
        109 => {
            if strcmp(w.as_mut_ptr(), b"map\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                get_map(p);
                return s;
            }
            if strcmp(
                w.as_mut_ptr(),
                b"maxsysstaffsep\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                if !((*s).state as libc::c_int != 2 as libc::c_int) {
                    (*parsys)
                        .voice[curvoice.offset_from(voice_tb.as_mut_ptr())
                            as libc::c_long as usize]
                        .maxsep = scan_u(p, 0 as libc::c_int);
                    return s;
                }
            } else if strcmp(
                w.as_mut_ptr(),
                b"multicol\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                let mut bposy: libc::c_float = 0.;
                generate();
                if strncmp(
                    p,
                    b"start\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    if in_page == 0 {
                        a2b(
                            b"%%\n\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                    }
                    buffer_eob(0 as libc::c_int);
                    bposy = get_bposy();
                    multicol_start = bposy;
                    multicol_max = multicol_start;
                    lmarg = cfmt.leftmargin;
                    rmarg = cfmt.rightmargin;
                } else if strncmp(
                    p,
                    b"new\0" as *const u8 as *const libc::c_char,
                    3 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    if multicol_start == 0 as libc::c_int as libc::c_float {
                        error(
                            1 as libc::c_int,
                            s,
                            b"%%%%%s new without start\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            w.as_mut_ptr(),
                        );
                    } else {
                        buffer_eob(0 as libc::c_int);
                        bposy = get_bposy();
                        if bposy < multicol_start {
                            bskip((bposy - multicol_start) / cfmt.scale);
                        }
                        if bposy < multicol_max {
                            multicol_max = bposy;
                        }
                        cfmt.leftmargin = lmarg;
                        cfmt.rightmargin = rmarg;
                    }
                } else if strncmp(
                    p,
                    b"end\0" as *const u8 as *const libc::c_char,
                    3 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    if multicol_start == 0 as libc::c_int as libc::c_float {
                        error(
                            1 as libc::c_int,
                            s,
                            b"%%%%%s end without start\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            w.as_mut_ptr(),
                        );
                    } else {
                        buffer_eob(0 as libc::c_int);
                        bposy = get_bposy();
                        if bposy > multicol_max {
                            bskip((bposy - multicol_max) / cfmt.scale);
                        } else {
                            a2b(
                                b"%%\n\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                        }
                        cfmt.leftmargin = lmarg;
                        cfmt.rightmargin = rmarg;
                        multicol_start = 0 as libc::c_int as libc::c_float;
                        buffer_eob(0 as libc::c_int);
                        if (info[('X' as i32 - 'A' as i32) as usize]).is_null()
                            && epsf == 0
                        {
                            write_buffer();
                        }
                    }
                } else {
                    error(
                        1 as libc::c_int,
                        s,
                        b"Unknown keyword '%s' in %%%%%s\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        p,
                        w.as_mut_ptr(),
                    );
                }
                return s;
            }
            current_block_685 = 3073279508649792450;
        }
        77 => {
            if strcmp(w.as_mut_ptr(), b"MIDI\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
                && strncmp(
                    p,
                    b"temperamentequal\0" as *const u8 as *const libc::c_char,
                    16 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                let mut n: libc::c_int = 0;
                if cfmt.nedo != 0 {
                    error(
                        1 as libc::c_int,
                        s,
                        b"%%%%MIDI temperamentequal redefined\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                    return s;
                }
                p = p.offset(16 as libc::c_int as isize);
                while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
                    p = p.offset(1);
                    p;
                }
                n = atoi(p);
                if n < 7 as libc::c_int || n > 53 as libc::c_int {
                    error(
                        1 as libc::c_int,
                        s,
                        b"Bad value in %%%%MIDI temperamentequal\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                    return s;
                }
                cfmt.nedo = n;
            }
            current_block_685 = 3073279508649792450;
        }
        110 => {
            if strcmp(w.as_mut_ptr(), b"newpage\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                if epsf != 0 || in_fname.is_null() {
                    return s;
                }
                if (*s).state as libc::c_int == 2 as libc::c_int {
                    generate();
                }
                buffer_eob(0 as libc::c_int);
                write_buffer();
                if isdigit(*p as libc::c_uchar as libc::c_int) != 0 {
                    pagenum = atoi(p);
                }
                close_page();
                if (*s).state as libc::c_int == 2 as libc::c_int {
                    bskip(cfmt.topspace);
                }
                return s;
            }
            current_block_685 = 3073279508649792450;
        }
        112 => {
            if strcmp(w.as_mut_ptr(), b"pos\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                p = get_str(
                    w.as_mut_ptr(),
                    p,
                    ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong
                        as libc::c_int,
                );
                set_voice_param(curvoice, (*s).state as libc::c_int, w.as_mut_ptr(), p);
                return s;
            }
            if strcmp(w.as_mut_ptr(), b"ps\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
                || strcmp(
                    w.as_mut_ptr(),
                    b"postscript\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                ps_def(s, p, 'b' as i32 as libc::c_char);
                return s;
            }
            current_block_685 = 3073279508649792450;
        }
        111 => {
            if strcmp(w.as_mut_ptr(), b"ornament\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                set_voice_param(curvoice, (*s).state as libc::c_int, w.as_mut_ptr(), p);
                return s;
            }
            current_block_685 = 3073279508649792450;
        }
        114 => {
            if strcmp(w.as_mut_ptr(), b"repbra\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                if (*s).state as libc::c_int != 2 as libc::c_int {
                    return s;
                }
                (*curvoice)
                    .set_norepbra(
                        (!(strchr(
                            b"0FfNn\0" as *const u8 as *const libc::c_char,
                            *p as libc::c_int,
                        ))
                            .is_null() || *p as libc::c_int == '\0' as i32)
                            as libc::c_int as libc::c_uint,
                    );
                return s;
            }
            if strcmp(w.as_mut_ptr(), b"repeat\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                let mut n_0: libc::c_int = 0;
                let mut k: libc::c_int = 0;
                if (*s).state as libc::c_int != 2 as libc::c_int {
                    return s;
                }
                if ((*curvoice).last_sym).is_null() {
                    error(
                        1 as libc::c_int,
                        s,
                        b"%%%s cannot start a tune\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        w.as_mut_ptr(),
                    );
                    return s;
                }
                if *p as libc::c_int == '\0' as i32 {
                    n_0 = 1 as libc::c_int;
                    k = 1 as libc::c_int;
                } else {
                    n_0 = atoi(p);
                    if n_0 < 1 as libc::c_int
                        || (*(*curvoice).last_sym).type_0 as libc::c_int
                            == 3 as libc::c_int && n_0 > 2 as libc::c_int
                    {
                        error(
                            1 as libc::c_int,
                            s,
                            b"Incorrect 1st value in %%%%%s\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            w.as_mut_ptr(),
                        );
                        return s;
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
                    if *p as libc::c_int == '\0' as i32 {
                        k = 1 as libc::c_int;
                    } else {
                        k = atoi(p);
                        if k < 1 as libc::c_int {
                            error(
                                1 as libc::c_int,
                                s,
                                b"Incorrect 2nd value in %%%%%s\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                                w.as_mut_ptr(),
                            );
                            return s;
                        }
                    }
                }
                (*s).aux = 2 as libc::c_int as libc::c_short;
                if (*(*curvoice).last_sym).type_0 as libc::c_int == 3 as libc::c_int {
                    (*s).doty = n_0 as libc::c_schar;
                } else {
                    (*s).doty = -n_0 as libc::c_schar;
                }
                sym_link(s, 12 as libc::c_int);
                (*s).nohdi1 = k as libc::c_schar;
                (*s).text = 0 as *mut libc::c_char;
                return s;
            }
            current_block_685 = 3073279508649792450;
        }
        115 => {
            if strcmp(w.as_mut_ptr(), b"setbarnb\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                if (*s).state as libc::c_int == 2 as libc::c_int {
                    let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
                    let mut n_1: libc::c_int = 0;
                    n_1 = atoi(p);
                    s2 = (*s).abc_next;
                    while !s2.is_null() {
                        if (*s2).abc_type as libc::c_int == 6 as libc::c_int {
                            (*s2).aux = n_1 as libc::c_short;
                            break;
                        } else {
                            s2 = (*s2).abc_next;
                        }
                    }
                    return s;
                }
                strcpy(
                    w.as_mut_ptr(),
                    b"measurefirst\0" as *const u8 as *const libc::c_char,
                );
            } else {
                if strcmp(w.as_mut_ptr(), b"sep\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    let mut h2: libc::c_float = 0.;
                    let mut len: libc::c_float = 0.;
                    let mut lwidth: libc::c_float = 0.;
                    if (*s).state as libc::c_int == 2 as libc::c_int {
                        gen_ly(0 as libc::c_int);
                    } else if (*s).state as libc::c_int == 0 as libc::c_int {
                        if epsf != 0 || in_fname.is_null() {
                            return s;
                        }
                    }
                    lwidth = (if cfmt.landscape != 0 {
                        cfmt.pageheight
                    } else {
                        cfmt.pagewidth
                    }) - cfmt.leftmargin - cfmt.rightmargin;
                    len = 0 as libc::c_int as libc::c_float;
                    h2 = len;
                    h1 = h2;
                    if *p as libc::c_int != '\0' as i32 {
                        h1 = scan_u(p, 0 as libc::c_int);
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
                    if *p as libc::c_int != '\0' as i32 {
                        h2 = scan_u(p, 0 as libc::c_int);
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
                    if *p as libc::c_int != '\0' as i32 {
                        len = scan_u(p, 0 as libc::c_int);
                    }
                    if h1 < 1 as libc::c_int as libc::c_float {
                        h1 = (0.5f64 * 37.8f64) as libc::c_float;
                    }
                    if h2 < 1 as libc::c_int as libc::c_float {
                        h2 = h1;
                    }
                    if len < 1 as libc::c_int as libc::c_float {
                        len = (3.0f64 * 37.8f64) as libc::c_float;
                    }
                    bskip(h1);
                    a2b(
                        b"%.1f %.1f sep0\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        (len / cfmt.scale) as libc::c_double,
                        (lwidth - len) as libc::c_double * 0.5f64
                            / cfmt.scale as libc::c_double,
                    );
                    bskip(h2);
                    buffer_eob(0 as libc::c_int);
                    return s;
                }
                if strcmp(w.as_mut_ptr(), b"staff\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    let mut staff: libc::c_int = 0;
                    if (*s).state as libc::c_int != 2 as libc::c_int {
                        return s;
                    }
                    if *p as libc::c_int == '+' as i32 {
                        staff = (*curvoice).cstaff as libc::c_int
                            + atoi(p.offset(1 as libc::c_int as isize));
                    } else if *p as libc::c_int == '-' as i32 {
                        staff = (*curvoice).cstaff as libc::c_int
                            - atoi(p.offset(1 as libc::c_int as isize));
                    } else {
                        staff = atoi(p) - 1 as libc::c_int;
                    }
                    if staff as libc::c_uint > nstaff as libc::c_uint {
                        error(
                            1 as libc::c_int,
                            s,
                            b"Bad staff in %%%%%s\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            w.as_mut_ptr(),
                        );
                        return s;
                    }
                    (*curvoice).set_floating(0 as libc::c_int as libc::c_uint);
                    (*curvoice).cstaff = staff as libc::c_uchar;
                    return s;
                }
                if strcmp(
                    w.as_mut_ptr(),
                    b"staffbreak\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if (*s).state as libc::c_int != 2 as libc::c_int {
                        return s;
                    }
                    if isdigit(*p as libc::c_int) != 0 {
                        (*s).xmx = scan_u(p, 0 as libc::c_int);
                        if (*s).xmx < 0 as libc::c_int as libc::c_float {
                            error(
                                1 as libc::c_int,
                                s,
                                b"Bad value in %%%%%s\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                w.as_mut_ptr(),
                            );
                            return s;
                        }
                        if *p
                            .offset(
                                (strlen(p)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    as isize,
                            ) as libc::c_int == 'f' as i32
                        {
                            (*s).doty = 1 as libc::c_int as libc::c_schar;
                        }
                    } else {
                        (*s).xmx = (0.5f64 * 37.8f64) as libc::c_float;
                        if *p as libc::c_int == 'f' as i32 {
                            (*s).doty = 1 as libc::c_int as libc::c_schar;
                        }
                    }
                    sym_link(s, 14 as libc::c_int);
                    return s;
                }
                if strcmp(
                    w.as_mut_ptr(),
                    b"stafflines\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if isdigit(*p as libc::c_uchar as libc::c_int) != 0 {
                        match atoi(p) {
                            0 => {
                                p = b"...\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char;
                            }
                            1 => {
                                p = b"..|\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char;
                            }
                            2 => {
                                p = b".||\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char;
                            }
                            3 => {
                                p = b".|||\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char;
                            }
                            4 => {
                                p = b"||||\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char;
                            }
                            5 => {
                                p = b"|||||\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char;
                            }
                            6 => {
                                p = b"||||||\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char;
                            }
                            7 => {
                                p = b"|||||||\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char;
                            }
                            8 => {
                                p = b"||||||||\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char;
                            }
                            _ => {
                                error(
                                    1 as libc::c_int,
                                    s,
                                    b"Bad number of lines\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                );
                            }
                        }
                    } else {
                        let mut l: libc::c_int = 0;
                        l = strlen(p) as libc::c_int;
                        q = p;
                        p = getarena(l + 1 as libc::c_int) as *mut libc::c_char;
                        strcpy(p, q);
                    }
                    if (*s).state as libc::c_int != 2 as libc::c_int {
                        voice = 0 as libc::c_int;
                        while voice < 32 as libc::c_int {
                            voice_tb[voice as usize].stafflines = p;
                            voice += 1;
                            voice;
                        }
                    } else {
                        (*curvoice).stafflines = p;
                    }
                    return s;
                }
                if strcmp(
                    w.as_mut_ptr(),
                    b"staffscale\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    let mut q_0: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut scale: libc::c_float = 0.;
                    scale = strtod(p, &mut q_0) as libc::c_float;
                    if (scale as libc::c_double) < 0.3f64
                        || scale > 2 as libc::c_int as libc::c_float
                        || *q_0 as libc::c_int != '\0' as i32
                            && *q_0 as libc::c_int != ' ' as i32
                    {
                        error(
                            1 as libc::c_int,
                            s,
                            b"Bad value in %%%%%s\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            w.as_mut_ptr(),
                        );
                        return s;
                    }
                    if (*s).state as libc::c_int != 2 as libc::c_int {
                        cfmt.staffscale = scale;
                    } else {
                        (*curvoice).staffscale = scale;
                    }
                    return s;
                }
                if strcmp(
                    w.as_mut_ptr(),
                    b"staves\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                    || strcmp(
                        w.as_mut_ptr(),
                        b"score\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                {
                    if (*s).state as libc::c_int == 0 as libc::c_int {
                        return s;
                    }
                    get_staves(s);
                    return s;
                }
                if strcmp(
                    w.as_mut_ptr(),
                    b"stemdir\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    set_voice_param(
                        curvoice,
                        (*s).state as libc::c_int,
                        w.as_mut_ptr(),
                        p,
                    );
                    return s;
                }
                if strcmp(
                    w.as_mut_ptr(),
                    b"sysstaffsep\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if !((*s).state as libc::c_int != 2 as libc::c_int) {
                        (*parsys)
                            .voice[curvoice.offset_from(voice_tb.as_mut_ptr())
                                as libc::c_long as usize]
                            .sep = scan_u(p, 0 as libc::c_int);
                        return s;
                    }
                }
            }
            current_block_685 = 3073279508649792450;
        }
        116 => {
            if strcmp(w.as_mut_ptr(), b"text\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                job_0 = 0;
                current_block_685 = 15489970745984101349;
            } else {
                if strcmp(
                    w.as_mut_ptr(),
                    b"tablature\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    let mut tblt: *mut tblt_s = 0 as *mut tblt_s;
                    let mut i: libc::c_int = 0;
                    let mut j: libc::c_int = 0;
                    tblt = tblt_parse(p);
                    if tblt.is_null() {
                        return s;
                    }
                    match (*s).state as libc::c_int {
                        2 | 1 => {
                            i = 0 as libc::c_int;
                            while i < ncmdtblt {
                                if !(cmdtblts[i as usize].active != 0) {
                                    j = cmdtblts[i as usize].index as libc::c_int;
                                    if j < 0 as libc::c_int || tblts[j as usize] == tblt {
                                        return s;
                                    }
                                }
                                i += 1;
                                i;
                            }
                            if !((*curvoice).tblts[0 as libc::c_int as usize] == tblt
                                || (*curvoice).tblts[1 as libc::c_int as usize] == tblt)
                            {
                                if !((*curvoice).tblts[1 as libc::c_int as usize]).is_null()
                                {
                                    error(
                                        1 as libc::c_int,
                                        s,
                                        b"Too many tablatures for voice %s\0" as *const u8
                                            as *const libc::c_char as *mut libc::c_char,
                                        ((*curvoice).id).as_mut_ptr(),
                                    );
                                } else if ((*curvoice).tblts[0 as libc::c_int as usize])
                                    .is_null()
                                {
                                    (*curvoice).tblts[0 as libc::c_int as usize] = tblt;
                                } else {
                                    (*curvoice).tblts[1 as libc::c_int as usize] = tblt;
                                }
                            }
                        }
                        _ => {}
                    }
                    return s;
                }
                if strcmp(
                    w.as_mut_ptr(),
                    b"transpose\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    let mut p_voice: *mut VOICE_S = 0 as *mut VOICE_S;
                    let mut s2_0: *mut SYMBOL = 0 as *mut SYMBOL;
                    let mut i_0: libc::c_int = 0;
                    let mut val: libc::c_int = 0;
                    val = get_transpose(p);
                    match (*s).state as libc::c_int {
                        0 => {
                            cfmt.transpose = val;
                            return s;
                        }
                        1 => {
                            cfmt.transpose += val;
                            i_0 = 32 as libc::c_int;
                            p_voice = voice_tb.as_mut_ptr();
                            loop {
                                i_0 -= 1;
                                if !(i_0 >= 0 as libc::c_int) {
                                    break;
                                }
                                (*p_voice).transpose = cfmt.transpose as libc::c_short;
                                memcpy(
                                    &mut (*p_voice).key as *mut key_s as *mut libc::c_void,
                                    &mut (*p_voice).okey as *mut key_s as *const libc::c_void,
                                    ::core::mem::size_of::<key_s>() as libc::c_ulong,
                                );
                                key_transpose(&mut (*p_voice).key);
                                memcpy(
                                    &mut (*p_voice).ckey as *mut key_s as *mut libc::c_void,
                                    &mut (*p_voice).key as *mut key_s as *const libc::c_void,
                                    ::core::mem::size_of::<key_s>() as libc::c_ulong,
                                );
                                if (*p_voice).key.empty != 0 {
                                    (*p_voice).key.sf = 0 as libc::c_int as libc::c_schar;
                                }
                                p_voice = p_voice.offset(1);
                                p_voice;
                            }
                            return s;
                        }
                        _ => {}
                    }
                    (*curvoice).transpose = (cfmt.transpose + val) as libc::c_short;
                    s2_0 = (*curvoice).sym;
                    if s2_0.is_null() {
                        memcpy(
                            &mut (*curvoice).key as *mut key_s as *mut libc::c_void,
                            &mut (*curvoice).okey as *mut key_s as *const libc::c_void,
                            ::core::mem::size_of::<key_s>() as libc::c_ulong,
                        );
                        key_transpose(&mut (*curvoice).key);
                        memcpy(
                            &mut (*curvoice).ckey as *mut key_s as *mut libc::c_void,
                            &mut (*curvoice).key as *mut key_s as *const libc::c_void,
                            ::core::mem::size_of::<key_s>() as libc::c_ulong,
                        );
                        if (*curvoice).key.empty != 0 {
                            (*curvoice).key.sf = 0 as libc::c_int as libc::c_schar;
                        }
                        return s;
                    }
                    while !((*s2_0).type_0 as libc::c_int == 6 as libc::c_int) {
                        if (*s2_0).time == (*curvoice).time {
                            s2_0 = (*s2_0).prev;
                            if !s2_0.is_null() {
                                continue;
                            }
                        }
                        s2_0 = s;
                        (*s2_0).abc_type = 1 as libc::c_int as libc::c_char;
                        (*s2_0).text = getarena(2 as libc::c_int) as *mut libc::c_char;
                        *((*s2_0).text)
                            .offset(
                                0 as libc::c_int as isize,
                            ) = 'K' as i32 as libc::c_char;
                        *((*s2_0).text)
                            .offset(
                                1 as libc::c_int as isize,
                            ) = '\0' as i32 as libc::c_char;
                        sym_link(s2_0, 6 as libc::c_int);
                        (*s2_0).aux = (*curvoice).key.sf as libc::c_short;
                        break;
                    }
                    memcpy(
                        &mut (*s2_0).u.key as *mut key_s as *mut libc::c_void,
                        &mut (*curvoice).okey as *mut key_s as *const libc::c_void,
                        ::core::mem::size_of::<key_s>() as libc::c_ulong,
                    );
                    key_transpose(&mut (*s2_0).u.key);
                    memcpy(
                        &mut (*curvoice).ckey as *mut key_s as *mut libc::c_void,
                        &mut (*s2_0).u.key as *mut key_s as *const libc::c_void,
                        ::core::mem::size_of::<key_s>() as libc::c_ulong,
                    );
                    if (*curvoice).key.empty != 0 {
                        (*s2_0).u.key.sf = 0 as libc::c_int as libc::c_schar;
                    }
                    return s;
                }
                if strcmp(w.as_mut_ptr(), b"tune\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    let mut s2_1: *mut SYMBOL = 0 as *mut SYMBOL;
                    let mut s3: *mut SYMBOL = 0 as *mut SYMBOL;
                    let mut opt: *mut tune_opt_s = 0 as *mut tune_opt_s;
                    let mut opt2: *mut tune_opt_s = 0 as *mut tune_opt_s;
                    if (*s).state as libc::c_int != 0 as libc::c_int {
                        error(
                            1 as libc::c_int,
                            s,
                            b"%%%%%s ignored\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            w.as_mut_ptr(),
                        );
                        return s;
                    }
                    if *p as libc::c_int == '\0' as i32 {
                        opt = tune_opts;
                        while !opt.is_null() {
                            free_voice_opt((*opt).voice_opts);
                            opt2 = (*opt).next;
                            free(opt as *mut libc::c_void);
                            opt = opt2;
                        }
                        tune_opts = 0 as *mut tune_opt_s;
                        return s;
                    }
                    if strcmp(p, b"end\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                        return s;
                    }
                    s2_1 = s;
                    loop {
                        s3 = (*s2_1).abc_next;
                        if s3.is_null() {
                            break;
                        }
                        if (*s3).abc_type as libc::c_int != 0 as libc::c_int
                            && ((*s3).abc_type as libc::c_int != 2 as libc::c_int
                                || strncmp(
                                    &mut *((*s3).text).offset(2 as libc::c_int as isize),
                                    b"tune \0" as *const u8 as *const libc::c_char,
                                    5 as libc::c_int as libc::c_ulong,
                                ) == 0 as libc::c_int)
                        {
                            break;
                        }
                        s2_1 = s3;
                    }
                    opt2 = 0 as *mut tune_opt_s;
                    opt = tune_opts;
                    while !opt.is_null() {
                        if strcmp((*(*opt).s).text, (*s).text) == 0 as libc::c_int {
                            break;
                        }
                        opt2 = opt;
                        opt = (*opt).next;
                    }
                    if !opt.is_null() {
                        free_voice_opt((*opt).voice_opts);
                        if s2_1 == s {
                            if opt2.is_null() {
                                tune_opts = (*opt).next;
                            } else {
                                (*opt2).next = (*opt).next;
                            }
                            free(opt as *mut libc::c_void);
                            return s;
                        }
                        (*opt).voice_opts = 0 as *mut voice_opt_s;
                    } else {
                        if s2_1 == s {
                            return s;
                        }
                        opt = malloc(
                            ::core::mem::size_of::<tune_opt_s>() as libc::c_ulong,
                        ) as *mut tune_opt_s;
                        memset(
                            opt as *mut libc::c_void,
                            0 as libc::c_int,
                            ::core::mem::size_of::<tune_opt_s>() as libc::c_ulong,
                        );
                        (*opt).next = tune_opts;
                        tune_opts = opt;
                    }
                    s3 = s;
                    (*opt).s = s3;
                    cur_tune_opts = opt;
                    s = (*s).abc_next;
                    loop {
                        if (*s).abc_type as libc::c_int != 2 as libc::c_int {
                            continue;
                        }
                        if strncmp(
                            &mut *((*s).text).offset(2 as libc::c_int as isize),
                            b"voice \0" as *const u8 as *const libc::c_char,
                            6 as libc::c_int as libc::c_ulong,
                        ) == 0 as libc::c_int
                        {
                            s = process_pscomment(s);
                        } else {
                            (*s).state = 1 as libc::c_int as libc::c_char;
                            (*s3).next = s;
                            s3 = s;
                        }
                        if s == s2_1 {
                            break;
                        }
                        s = (*s).abc_next;
                    }
                    cur_tune_opts = 0 as *mut tune_opt_s;
                    return s;
                }
                current_block_685 = 3073279508649792450;
            }
        }
        117 => {
            if strcmp(w.as_mut_ptr(), b"user\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                deco[(*s).u.user.symbol
                    as usize] = parse
                    .deco_tb[((*s).u.user.value as libc::c_int - 128 as libc::c_int)
                    as usize];
                return s;
            }
            current_block_685 = 3073279508649792450;
        }
        118 => {
            if strcmp(w.as_mut_ptr(), b"vocal\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                set_voice_param(curvoice, (*s).state as libc::c_int, w.as_mut_ptr(), p);
                return s;
            }
            if strcmp(w.as_mut_ptr(), b"voice\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                let mut s2_2: *mut SYMBOL = 0 as *mut SYMBOL;
                let mut s3_0: *mut SYMBOL = 0 as *mut SYMBOL;
                let mut opt_0: *mut voice_opt_s = 0 as *mut voice_opt_s;
                let mut opt2_0: *mut voice_opt_s = 0 as *mut voice_opt_s;
                if (*s).state as libc::c_int != 0 as libc::c_int {
                    error(
                        1 as libc::c_int,
                        s,
                        b"%%%%voice ignored\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    return s;
                }
                if *p as libc::c_int == '\0' as i32 {
                    if !cur_tune_opts.is_null() {
                        free_voice_opt((*cur_tune_opts).voice_opts);
                        (*cur_tune_opts).voice_opts = 0 as *mut voice_opt_s;
                    } else {
                        free_voice_opt(voice_opts);
                        voice_opts = 0 as *mut voice_opt_s;
                    }
                    return s;
                }
                if strcmp(p, b"end\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    return s;
                }
                if !cur_tune_opts.is_null() {
                    opt_0 = (*cur_tune_opts).voice_opts;
                } else {
                    opt_0 = voice_opts;
                }
                s2_2 = s;
                loop {
                    s3_0 = (*s2_2).abc_next;
                    if s3_0.is_null() {
                        break;
                    }
                    if (*s3_0).abc_type as libc::c_int != 0 as libc::c_int
                        && ((*s3_0).abc_type as libc::c_int != 2 as libc::c_int
                            || strncmp(
                                &mut *((*s3_0).text).offset(2 as libc::c_int as isize),
                                b"score \0" as *const u8 as *const libc::c_char,
                                6 as libc::c_int as libc::c_ulong,
                            ) == 0 as libc::c_int
                            || strncmp(
                                &mut *((*s3_0).text).offset(2 as libc::c_int as isize),
                                b"staves \0" as *const u8 as *const libc::c_char,
                                7 as libc::c_int as libc::c_ulong,
                            ) == 0 as libc::c_int
                            || strncmp(
                                &mut *((*s3_0).text).offset(2 as libc::c_int as isize),
                                b"tune \0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int as libc::c_ulong,
                            ) == 0 as libc::c_int
                            || strncmp(
                                &mut *((*s3_0).text).offset(2 as libc::c_int as isize),
                                b"voice \0" as *const u8 as *const libc::c_char,
                                6 as libc::c_int as libc::c_ulong,
                            ) == 0 as libc::c_int)
                    {
                        break;
                    }
                    s2_2 = s3_0;
                }
                opt2_0 = 0 as *mut voice_opt_s;
                while !opt_0.is_null() {
                    if strcmp((*(*opt_0).s).text, (*s).text) == 0 as libc::c_int {
                        if opt2_0.is_null() {
                            if !cur_tune_opts.is_null() {
                                (*cur_tune_opts).voice_opts = 0 as *mut voice_opt_s;
                            } else {
                                voice_opts = 0 as *mut voice_opt_s;
                            }
                        } else {
                            (*opt2_0).next = (*opt_0).next;
                        }
                        free(opt_0 as *mut libc::c_void);
                        break;
                    } else {
                        opt2_0 = opt_0;
                        opt_0 = (*opt_0).next;
                    }
                }
                if s2_2 == s {
                    return s;
                }
                opt_0 = malloc(
                    (::core::mem::size_of::<voice_opt_s>() as libc::c_ulong)
                        .wrapping_add(strlen(p)),
                ) as *mut voice_opt_s;
                memset(
                    opt_0 as *mut libc::c_void,
                    0 as libc::c_int,
                    ::core::mem::size_of::<voice_opt_s>() as libc::c_ulong,
                );
                if !cur_tune_opts.is_null() {
                    (*opt_0).next = (*cur_tune_opts).voice_opts;
                    (*cur_tune_opts).voice_opts = opt_0;
                } else {
                    (*opt_0).next = voice_opts;
                    voice_opts = opt_0;
                }
                s3_0 = s;
                (*opt_0).s = s3_0;
                while s != s2_2 {
                    if !((*(*s).abc_next).abc_type as libc::c_int != 2 as libc::c_int) {
                        (*(*s).abc_next).state = 2 as libc::c_int as libc::c_char;
                        (*s3_0).next = (*s).abc_next;
                        s3_0 = (*s3_0).next;
                    }
                    s = (*s).abc_next;
                }
                return s;
            }
            if strcmp(
                w.as_mut_ptr(),
                b"voicecolor\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                let mut color: libc::c_int = 0;
                if curvoice.is_null() {
                    return s;
                }
                color = get_color(p);
                if color < 0 as libc::c_int {
                    error(
                        1 as libc::c_int,
                        s,
                        b"Bad color in %%%%voicecolor\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                } else {
                    (*curvoice).color = color;
                }
                return s;
            }
            if strcmp(
                w.as_mut_ptr(),
                b"voicecombine\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                let mut combine: libc::c_int = 0;
                if sscanf(
                    p,
                    b"%d\0" as *const u8 as *const libc::c_char,
                    &mut combine as *mut libc::c_int,
                ) != 1 as libc::c_int
                {
                    error(
                        1 as libc::c_int,
                        s,
                        b"Bad value in %%%%voicecombine\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                    return s;
                }
                match (*s).state as libc::c_int {
                    0 => {
                        cfmt.combinevoices = combine;
                    }
                    1 => {
                        voice = 0 as libc::c_int;
                        while voice < 32 as libc::c_int {
                            voice_tb[voice as usize].combine = combine as libc::c_schar;
                            voice += 1;
                            voice;
                        }
                    }
                    _ => {
                        (*curvoice).combine = combine as libc::c_schar;
                    }
                }
                return s;
            }
            if strcmp(w.as_mut_ptr(), b"voicemap\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                if (*s).state as libc::c_int != 2 as libc::c_int {
                    voice = 0 as libc::c_int;
                    while voice < 32 as libc::c_int {
                        voice_tb[voice as usize].map_name = p;
                        voice += 1;
                        voice;
                    }
                } else {
                    (*curvoice).map_name = p;
                }
                return s;
            }
            if strcmp(
                w.as_mut_ptr(),
                b"voicescale\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                let mut q_1: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut scale_0: libc::c_float = 0.;
                scale_0 = strtod(p, &mut q_1) as libc::c_float;
                if (scale_0 as libc::c_double) < 0.6f64
                    || scale_0 as libc::c_double > 1.5f64
                    || *q_1 as libc::c_int != '\0' as i32
                        && *q_1 as libc::c_int != ' ' as i32
                {
                    error(
                        1 as libc::c_int,
                        s,
                        b"Bad %%%%voicescale value\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    return s;
                }
                if (*s).state as libc::c_int != 2 as libc::c_int {
                    voice = 0 as libc::c_int;
                    while voice < 32 as libc::c_int {
                        voice_tb[voice as usize].scale = scale_0;
                        voice += 1;
                        voice;
                    }
                } else {
                    (*curvoice).scale = scale_0;
                }
                return s;
            }
            if strcmp(w.as_mut_ptr(), b"volume\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                set_voice_param(curvoice, (*s).state as libc::c_int, w.as_mut_ptr(), p);
                return s;
            }
            if strcmp(w.as_mut_ptr(), b"vskip\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                if (*s).state as libc::c_int == 2 as libc::c_int {
                    gen_ly(0 as libc::c_int);
                } else if (*s).state as libc::c_int == 0 as libc::c_int {
                    if epsf != 0 || in_fname.is_null() {
                        return s;
                    }
                }
                bskip(scan_u(p, 0 as libc::c_int));
                buffer_eob(0 as libc::c_int);
                return s;
            }
            current_block_685 = 3073279508649792450;
        }
        _ => {
            current_block_685 = 3073279508649792450;
        }
    }
    match current_block_685 {
        3073279508649792450 => {}
        _ => {
            if (*s).state as libc::c_int == 2 as libc::c_int {
                gen_ly(1 as libc::c_int);
            } else if (*s).state as libc::c_int == 0 as libc::c_int {
                if epsf != 0 || in_fname.is_null() {
                    return s;
                }
            }
            if w[0 as libc::c_int as usize] as libc::c_int == 'c' as i32 {
                job_0 = 3 as libc::c_int;
            } else {
                job_0 = cfmt.textoption;
                match job_0 {
                    4 => return s,
                    0 | 5 | 3 => {}
                    _ => {
                        job_0 = 0 as libc::c_int;
                    }
                }
            }
            write_text(w.as_mut_ptr(), p, job_0);
            return s;
        }
    }
    if (*s).state as libc::c_int == 2 as libc::c_int {
        if strcmp(w.as_mut_ptr(), b"leftmargin\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
            || strcmp(
                w.as_mut_ptr(),
                b"rightmargin\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            || strcmp(w.as_mut_ptr(), b"scale\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
        {
            generate();
            block_put();
        }
    }
    interpret_fmt_line(w.as_mut_ptr(), p, lock);
    if cfmt.alignbars != 0
        && strcmp(w.as_mut_ptr(), b"alignbars\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        let mut i_1: libc::c_int = 0;
        generate();
        if cfmt.alignbars as libc::c_uint > 32 as libc::c_int as libc::c_uint {
            error(
                1 as libc::c_int,
                s,
                b"Too big value in %%%%alignbars\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            cfmt.alignbars = 32 as libc::c_int;
        }
        if staves_found >= 0 as libc::c_int {
            cfmt.alignbars = nstaff + 1 as libc::c_int;
        }
        curvoice = voice_tb.as_mut_ptr();
        first_voice = curvoice;
        i_1 = 0 as libc::c_int;
        while i_1 < cfmt.alignbars {
            voice_tb[i_1 as usize].cstaff = i_1 as libc::c_uchar;
            voice_tb[i_1 as usize].staff = voice_tb[i_1 as usize].cstaff;
            voice_tb[i_1 as usize]
                .next = &mut *voice_tb
                .as_mut_ptr()
                .offset((i_1 + 1 as libc::c_int) as isize) as *mut VOICE_S;
            (*parsys)
                .staff[i_1 as usize]
                .flags = ((*parsys).staff[i_1 as usize].flags as libc::c_int
                | 0x40 as libc::c_int) as libc::c_short;
            (*parsys).voice[i_1 as usize].staff = i_1 as libc::c_uchar;
            (*parsys).voice[i_1 as usize].range = i_1 as libc::c_schar;
            i_1 += 1;
            i_1;
        }
        i_1 -= 1;
        i_1;
        voice_tb[i_1 as usize].next = 0 as *mut VOICE_S;
        nstaff = i_1;
        (*parsys).nstaff = nstaff as libc::c_short;
    }
    return s;
}
unsafe extern "C" fn set_tuplet(mut t: *mut SYMBOL) {
    let mut s: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut s1: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut l: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut lplet: libc::c_int = 0;
    let mut grace: libc::c_int = 0;
    r = (*t).u.tuplet.r_plet as libc::c_int;
    grace = (*t).flags as libc::c_int & 0x20 as libc::c_int;
    if (*t).flags as libc::c_int & 0x40 as libc::c_int != 0 {
        if r != 1 as libc::c_int {
            error(
                1 as libc::c_int,
                t,
                b"Empty tuplet\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return;
        }
        grace = 0 as libc::c_int;
    }
    l = 0 as libc::c_int;
    let mut current_block_37: u64;
    s = (*t).abc_next;
    while !s.is_null() {
        if (*s).abc_type as libc::c_int == 11 as libc::c_int {
            let mut s2: *mut SYMBOL = 0 as *mut SYMBOL;
            let mut l2: libc::c_int = 0;
            let mut r2: libc::c_int = 0;
            r2 = (*s).u.tuplet.r_plet as libc::c_int;
            l2 = 0 as libc::c_int;
            s2 = (*s).abc_next;
            while !s2.is_null() {
                match (*s2).abc_type as libc::c_int {
                    4 | 5 => {
                        if !((*s2).u.note.notes[0 as libc::c_int as usize].len
                            == 0 as libc::c_int)
                        {
                            if !(grace ^ (*s2).flags as libc::c_int & 0x20 as libc::c_int
                                != 0)
                            {
                                s1 = s2;
                                l2 += (*s1).dur;
                                r2 -= 1;
                                if r2 <= 0 as libc::c_int {
                                    break;
                                }
                            }
                        }
                    }
                    7 => {
                        if (*s2).u.eoln.type_0 as libc::c_int != 1 as libc::c_int {
                            error(
                                1 as libc::c_int,
                                t,
                                b"End of line found inside a nested tuplet\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                            );
                            return;
                        }
                    }
                    _ => {}
                }
                s2 = (*s2).abc_next;
            }
            l2 = l2 * (*s).u.tuplet.q_plet as libc::c_int
                / (*s).u.tuplet.p_plet as libc::c_int;
            (*s).aux = l2 as libc::c_short;
            l += l2;
            r -= (*s).u.tuplet.r_plet as libc::c_int;
            if r == 0 as libc::c_int {
                break;
            }
            if r < 0 as libc::c_int {
                error(
                    1 as libc::c_int,
                    t,
                    b"Bad nested tuplet\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                break;
            } else {
                s = s2;
            }
        } else {
            match (*s).abc_type as libc::c_int {
                4 | 5 => {
                    current_block_37 = 6417057564578538666;
                    match current_block_37 {
                        10390987450131780233 => {
                            if (*s).u.eoln.type_0 as libc::c_int != 1 as libc::c_int {
                                error(
                                    1 as libc::c_int,
                                    t,
                                    b"End of line found inside a tuplet\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                );
                                return;
                            }
                        }
                        _ => {
                            if !((*s).u.note.notes[0 as libc::c_int as usize].len
                                == 0 as libc::c_int)
                            {
                                if grace != 0 {
                                    if (*s).flags as libc::c_int & 0x20 as libc::c_int == 0 {
                                        error(
                                            1 as libc::c_int,
                                            t,
                                            b"End of tuplet before end of grace sequence\0" as *const u8
                                                as *const libc::c_char as *mut libc::c_char,
                                        );
                                        return;
                                    }
                                    current_block_37 = 790185930182612747;
                                } else if (*s).flags as libc::c_int & 0x20 as libc::c_int
                                    != 0
                                {
                                    current_block_37 = 14523784380283086299;
                                } else {
                                    current_block_37 = 790185930182612747;
                                }
                                match current_block_37 {
                                    14523784380283086299 => {}
                                    _ => {
                                        s1 = s;
                                        l += (*s).dur;
                                        r -= 1;
                                        if r <= 0 as libc::c_int {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                7 => {
                    current_block_37 = 10390987450131780233;
                    match current_block_37 {
                        10390987450131780233 => {
                            if (*s).u.eoln.type_0 as libc::c_int != 1 as libc::c_int {
                                error(
                                    1 as libc::c_int,
                                    t,
                                    b"End of line found inside a tuplet\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                );
                                return;
                            }
                        }
                        _ => {
                            if !((*s).u.note.notes[0 as libc::c_int as usize].len
                                == 0 as libc::c_int)
                            {
                                if grace != 0 {
                                    if (*s).flags as libc::c_int & 0x20 as libc::c_int == 0 {
                                        error(
                                            1 as libc::c_int,
                                            t,
                                            b"End of tuplet before end of grace sequence\0" as *const u8
                                                as *const libc::c_char as *mut libc::c_char,
                                        );
                                        return;
                                    }
                                    current_block_37 = 790185930182612747;
                                } else if (*s).flags as libc::c_int & 0x20 as libc::c_int
                                    != 0
                                {
                                    current_block_37 = 14523784380283086299;
                                } else {
                                    current_block_37 = 790185930182612747;
                                }
                                match current_block_37 {
                                    14523784380283086299 => {}
                                    _ => {
                                        s1 = s;
                                        l += (*s).dur;
                                        r -= 1;
                                        if r <= 0 as libc::c_int {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        s = (*s).abc_next;
    }
    if l == 0 {
        error(
            1 as libc::c_int,
            t,
            b"No note in the tuplet\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    if s.is_null() {
        error(
            1 as libc::c_int,
            t,
            b"End of tune found inside a tuplet\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    if (*t).aux as libc::c_int != 0 as libc::c_int {
        lplet = (*t).aux as libc::c_int;
    } else {
        lplet = l * (*t).u.tuplet.q_plet as libc::c_int
            / (*t).u.tuplet.p_plet as libc::c_int;
    }
    r = (*t).u.tuplet.r_plet as libc::c_int;
    s = (*t).abc_next;
    while !s.is_null() {
        let mut olddur: libc::c_int = 0;
        if (*s).abc_type as libc::c_int == 11 as libc::c_int {
            let mut r2_0: libc::c_int = 0;
            r2_0 = (*s).u.tuplet.r_plet as libc::c_int;
            s1 = s;
            olddur = (*s).aux as libc::c_int;
            (*s1).aux = (olddur * lplet / l) as libc::c_short;
            l -= olddur;
            lplet -= (*s1).aux as libc::c_int;
            r -= r2_0;
            loop {
                s = (*s).abc_next;
                if (*s).abc_type as libc::c_int != 4 as libc::c_int
                    && (*s).abc_type as libc::c_int != 5 as libc::c_int
                {
                    continue;
                }
                if (*s).u.note.notes[0 as libc::c_int as usize].len == 0 as libc::c_int {
                    continue;
                }
                if grace ^ (*s).flags as libc::c_int & 0x20 as libc::c_int != 0 {
                    continue;
                }
                r2_0 -= 1;
                if r2_0 <= 0 as libc::c_int {
                    break;
                }
            }
            if r <= 0 as libc::c_int {
                break;
            }
        } else if !((*s).abc_type as libc::c_int != 4 as libc::c_int
            && (*s).abc_type as libc::c_int != 5 as libc::c_int)
        {
            if !((*s).u.note.notes[0 as libc::c_int as usize].len == 0 as libc::c_int) {
                (*s).sflags |= 0x40 as libc::c_int as libc::c_uint;
                if !(grace ^ (*s).flags as libc::c_int & 0x20 as libc::c_int != 0) {
                    s1 = s;
                    olddur = (*s).dur;
                    (*s1).dur = olddur * lplet / l;
                    r -= 1;
                    if r <= 0 as libc::c_int {
                        break;
                    }
                    l -= olddur;
                    lplet -= (*s1).dur;
                }
            }
        }
        s = (*s).abc_next;
    }
    if grace != 0 {
        error(
            1 as libc::c_int,
            t,
            b"Tuplets in grace note sequence not yet treated\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    } else {
        sym_link(t, 13 as libc::c_int);
        (*t).aux = cfmt.tuplets as libc::c_short;
    };
}
