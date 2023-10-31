use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type __sFILEX;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
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
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn __maskrune(_: __darwin_ct_rune_t, _: libc::c_ulong) -> libc::c_int;
    static mut _DefaultRuneLocale: _RuneLocale;
    static mut __stdoutp: *mut FILE;
    static mut __stderrp: *mut FILE;
    fn fclose(_: *mut FILE) -> libc::c_int;
    fn fgets(_: *mut libc::c_char, _: libc::c_int, _: *mut FILE) -> *mut libc::c_char;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputc(_: libc::c_int, _: *mut FILE) -> libc::c_int;
    fn fputs(_: *const libc::c_char, _: *mut FILE) -> libc::c_int;
    fn ftell(_: *mut FILE) -> libc::c_long;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: __builtin_va_list,
    ) -> libc::c_int;
    fn localtime(_: *const time_t) -> *mut tm;
    fn strftime(
        _: *mut libc::c_char,
        _: size_t,
        _: *const libc::c_char,
        _: *const tm,
    ) -> size_t;
    fn time(_: *mut time_t) -> time_t;
    static mut cfmt: FORMAT;
    static mut dfmt: FORMAT;
    static mut info: INFO;
    static mut outft: libc::c_int;
    static mut tunenum: libc::c_int;
    static mut pagenum: libc::c_int;
    static mut pagenum_nr: libc::c_int;
    static mut defl: libc::c_int;
    static mut quiet: libc::c_int;
    static mut pagenumbers: libc::c_int;
    static mut epsf: libc::c_int;
    static mut svg: libc::c_int;
    static mut outfn: [libc::c_char; 1024];
    static mut in_fname: *mut libc::c_char;
    static mut mtime: time_t;
    static mut file_initialized: libc::c_int;
    static mut fout: *mut FILE;
    static mut s_argc: libc::c_int;
    static mut s_argv: *mut *mut libc::c_char;
    fn strext(fid: *mut libc::c_char, ext: *mut libc::c_char);
    fn svg_output(out: *mut FILE, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn svg_write(buf: *mut libc::c_char, len: libc::c_int);
    fn error(sev: libc::c_int, s: *mut SYMBOL, fmt: *mut libc::c_char, _: ...);
    fn set_str_font(cft: libc::c_int, dft: libc::c_int);
    fn str_out(p: *mut libc::c_char, action: libc::c_int);
    static mut tex_buf: [libc::c_char; 0];
    fn tex_str(s: *mut libc::c_char) -> libc::c_float;
    fn str_font(ft: libc::c_int);
    fn get_str_font(cft: *mut libc::c_int, dft: *mut libc::c_int);
    fn define_fonts();
    fn user_ps_write();
    fn define_symbols();
    fn svg_close();
    static mut multicol_start: libc::c_float;
    fn define_svg_symbols(
        title: *mut libc::c_char,
        num: libc::c_int,
        w: libc::c_float,
        h: libc::c_float,
    );
}
pub type __builtin_va_list = *mut libc::c_char;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_longlong;
pub type __darwin_ct_rune_t = libc::c_int;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_va_list = __builtin_va_list;
pub type __darwin_wchar_t = libc::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_time_t = libc::c_long;
pub type __darwin_off_t = __int64_t;
pub type size_t = __darwin_size_t;
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
pub type time_t = __darwin_time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *mut libc::c_char,
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
static mut ln_num: libc::c_int = 0;
static mut ln_pos: [libc::c_float; 80] = [0.; 80];
static mut ln_buf: [*mut libc::c_char; 80] = [0 as *const libc::c_char
    as *mut libc::c_char; 80];
static mut ln_lmarg: [libc::c_float; 80] = [0.; 80];
static mut ln_scale: [libc::c_float; 80] = [0.; 80];
static mut ln_font: [libc::c_schar; 80] = [0; 80];
static mut cur_lmarg: libc::c_float = 0 as libc::c_int as libc::c_float;
static mut max_rmarg: libc::c_float = 0.;
static mut cur_scale: libc::c_float = 1.0f64 as libc::c_float;
static mut maxy: libc::c_float = 0.;
static mut remy: libc::c_float = 0.;
static mut bposy: libc::c_float = 0.;
static mut nepsf: libc::c_int = 0;
static mut nbpages: libc::c_int = 0;
#[no_mangle]
pub static mut outbufsz: libc::c_int = 0;
static mut outfnam: [libc::c_char; 1024] = [0; 1024];
static mut p_fmt: *mut FORMAT = 0 as *const FORMAT as *mut FORMAT;
#[no_mangle]
pub static mut output: Option::<
    unsafe extern "C" fn(*mut FILE, *const libc::c_char, ...) -> libc::c_int,
> = None;
#[no_mangle]
pub static mut in_page: libc::c_int = 0;
#[no_mangle]
pub static mut outbuf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut mbf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut use_buffer: libc::c_int = 0;
unsafe extern "C" fn cutext(mut fid: *mut libc::c_char) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = strrchr(fid, '/' as i32);
    if p.is_null() {
        p = fid;
    }
    p = strrchr(p, '.' as i32);
    if !p.is_null() {
        *p = '\0' as i32 as libc::c_char;
    }
}
#[no_mangle]
pub unsafe extern "C" fn open_fout() {
    let mut i: libc::c_int = 0;
    let mut fnm: [libc::c_char; 1024] = [0; 1024];
    strcpy(fnm.as_mut_ptr(), outfn.as_mut_ptr());
    i = (strlen(fnm.as_mut_ptr())).wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as libc::c_int;
    if i < 0 as libc::c_int {
        strcpy(
            fnm.as_mut_ptr(),
            if svg != 0 || epsf > 1 as libc::c_int {
                b"Out.xhtml\0" as *const u8 as *const libc::c_char
            } else {
                b"Out.ps\0" as *const u8 as *const libc::c_char
            },
        );
    } else if i != 0 as libc::c_int
        || fnm[0 as libc::c_int as usize] as libc::c_int != '-' as i32
    {
        if fnm[i as usize] as libc::c_int == '=' as i32 && !in_fname.is_null() {
            let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
            p = strrchr(in_fname, '/' as i32);
            if p.is_null() {
                p = in_fname;
            } else {
                p = p.offset(1);
                p;
            }
            strcpy(&mut *fnm.as_mut_ptr().offset(i as isize), p);
            strext(
                fnm.as_mut_ptr(),
                (if svg != 0 || epsf > 1 as libc::c_int {
                    b"xhtml\0" as *const u8 as *const libc::c_char
                } else {
                    b"ps\0" as *const u8 as *const libc::c_char
                }) as *mut libc::c_char,
            );
        } else if fnm[i as usize] as libc::c_int == '/' as i32 {
            strcpy(
                &mut *fnm.as_mut_ptr().offset((i + 1 as libc::c_int) as isize),
                if svg != 0 || epsf > 1 as libc::c_int {
                    b"Out.xhtml\0" as *const u8 as *const libc::c_char
                } else {
                    b"Out.ps\0" as *const u8 as *const libc::c_char
                },
            );
        }
    }
    if svg == 1 as libc::c_int
        && (i != 0 as libc::c_int
            || fnm[0 as libc::c_int as usize] as libc::c_int != '-' as i32)
    {
        cutext(fnm.as_mut_ptr());
        i = (strlen(fnm.as_mut_ptr())).wrapping_sub(1 as libc::c_int as libc::c_ulong)
            as libc::c_int;
        if strncmp(fnm.as_mut_ptr(), outfnam.as_mut_ptr(), i as libc::c_ulong)
            != 0 as libc::c_int
        {
            nepsf = 0 as libc::c_int;
        }
        nepsf += 1;
        sprintf(
            &mut *fnm.as_mut_ptr().offset((i + 1 as libc::c_int) as isize)
                as *mut libc::c_char,
            b"%03d.svg\0" as *const u8 as *const libc::c_char,
            nepsf,
        );
    } else if strcmp(fnm.as_mut_ptr(), outfnam.as_mut_ptr()) == 0 as libc::c_int {
        return
    }
    close_output_file();
    strcpy(outfnam.as_mut_ptr(), fnm.as_mut_ptr());
    if i != 0 as libc::c_int
        || fnm[0 as libc::c_int as usize] as libc::c_int != '-' as i32
    {
        fout = fopen(fnm.as_mut_ptr(), b"w\0" as *const u8 as *const libc::c_char);
        if fout.is_null() {
            error(
                1 as libc::c_int,
                0 as *mut SYMBOL,
                b"Cannot create output file %s - abort\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                fnm.as_mut_ptr(),
            );
            exit(1 as libc::c_int);
        }
    } else {
        fout = __stdoutp;
    };
}
unsafe extern "C" fn cnv_date(mut ltime: *mut time_t) {
    let mut buf: [libc::c_char; 512] = [0; 512];
    tex_str(cfmt.dateformat);
    strcpy(buf.as_mut_ptr(), tex_buf.as_mut_ptr());
    strftime(
        tex_buf.as_mut_ptr(),
        512 as libc::c_int as size_t,
        buf.as_mut_ptr(),
        localtime(ltime),
    );
}
#[no_mangle]
pub unsafe extern "C" fn marg_init() {
    max_rmarg = 0 as libc::c_int as libc::c_float;
    cur_lmarg = max_rmarg;
}
unsafe extern "C" fn init_ps(mut str: *mut libc::c_char) {
    let mut ltime: time_t = 0;
    let mut i: libc::c_uint = 0;
    let mut version: [libc::c_char; 33] = *::core::mem::transmute::<
        &[u8; 33],
        &mut [libc::c_char; 33],
    >(b"/creator [(abcm2ps) 8.14.14] def\0");
    if epsf != 0 {
        fprintf(
            fout,
            b"%%!PS-Adobe-2.0 EPSF-2.0\n%%%%BoundingBox: 0 0 %.0f %.0f\n\0" as *const u8
                as *const libc::c_char,
            ((if (*p_fmt).landscape != 0 {
                (*p_fmt).pageheight
            } else {
                (*p_fmt).pagewidth
            }) - cur_lmarg - max_rmarg) as libc::c_double * 0.75f64,
            -bposy as libc::c_double * 0.75f64,
        );
        marg_init();
    } else {
        if fout.is_null() {
            open_fout();
        }
        fprintf(fout, b"%%!PS-Adobe-2.0\n\0" as *const u8 as *const libc::c_char);
        fprintf(
            fout,
            b"%%%%BoundingBox: 0 0 %.0f %.0f\n\0" as *const u8 as *const libc::c_char,
            (*p_fmt).pagewidth as libc::c_double * 0.75f64,
            (*p_fmt).pageheight as libc::c_double * 0.75f64,
        );
    }
    fprintf(fout, b"%%%%Title: %s\n\0" as *const u8 as *const libc::c_char, str);
    time(&mut ltime);
    strftime(
        tex_buf.as_mut_ptr(),
        512 as libc::c_int as size_t,
        b"%b %e, %Y %H:%M\0" as *const u8 as *const libc::c_char,
        localtime(&mut ltime),
    );
    fprintf(
        fout,
        b"%%%%Creator: abcm2ps-8.14.14\n%%%%CreationDate: %s\n\0" as *const u8
            as *const libc::c_char,
        tex_buf.as_mut_ptr(),
    );
    if epsf == 0 {
        fprintf(fout, b"%%%%Pages: (atend)\n\0" as *const u8 as *const libc::c_char);
    }
    fprintf(
        fout,
        b"%%%%LanguageLevel: 3\n%%%%EndComments\n%%CommandLine:\0" as *const u8
            as *const libc::c_char,
    );
    i = 1 as libc::c_int as libc::c_uint;
    while i < s_argc as libc::c_uint {
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut space: libc::c_int = 0;
        p = *s_argv.offset(i as isize);
        space = (!(strchr(p, ' ' as i32)).is_null()
            || !(strchr(p, '\n' as i32)).is_null()) as libc::c_int;
        fputc(' ' as i32, fout);
        if space != 0 {
            fputc('\'' as i32, fout);
        }
        loop {
            q = strchr(p, '\n' as i32);
            if q.is_null() {
                break;
            }
            fprintf(
                fout,
                b" %.*s\n%%\0" as *const u8 as *const libc::c_char,
                q.offset_from(p) as libc::c_long as libc::c_int,
                p,
            );
            p = q.offset(1 as libc::c_int as isize);
        }
        fprintf(fout, b"%s\0" as *const u8 as *const libc::c_char, p);
        if space != 0 {
            fputc('\'' as i32, fout);
        }
        i = i.wrapping_add(1);
        i;
    }
    fprintf(fout, b"\n\n\0" as *const u8 as *const libc::c_char);
    if epsf != 0 {
        fprintf(fout, b"save\n\0" as *const u8 as *const libc::c_char);
    }
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < strlen(version.as_mut_ptr()) {
        if version[i as usize] as libc::c_int == '.' as i32 {
            version[i as usize] = ' ' as i32 as libc::c_char;
        }
        i = i.wrapping_add(1);
        i;
    }
    fprintf(
        fout,
        b"%%%%BeginSetup\n/!{bind def}bind def\n/bdef{bind def}!\n/T/translate load def\n/M/moveto load def\n/RM/rmoveto load def\n/L/lineto load def\n/RL/rlineto load def\n/C/curveto load def\n/RC/rcurveto load def\n/SLW/setlinewidth load def\n/defl 0 def\n/dlw{0.7 SLW}!\n%s\n\0"
            as *const u8 as *const libc::c_char,
        version.as_mut_ptr(),
    );
    define_symbols();
    output = Some(
        fprintf
            as unsafe extern "C" fn(*mut FILE, *const libc::c_char, ...) -> libc::c_int,
    );
    user_ps_write();
    define_fonts();
    if epsf == 0 {
        fprintf(
            fout,
            b"/setpagedevice where{pop\n\t<</PageSize[%.0f %.0f]\0" as *const u8
                as *const libc::c_char,
            (*p_fmt).pagewidth as libc::c_double * 0.75f64,
            (*p_fmt).pageheight as libc::c_double * 0.75f64,
        );
        if cfmt.gutter != 0. {
            fprintf(
                fout,
                b"\n\t/BeginPage{1 and 0 eq{%.1f 0 T}{%.1f 0 T}ifelse}bind\n\t\0"
                    as *const u8 as *const libc::c_char,
                cfmt.gutter as libc::c_double,
                -cfmt.gutter as libc::c_double,
            );
        }
        fprintf(fout, b">>setpagedevice}if\n\0" as *const u8 as *const libc::c_char);
    }
    fprintf(fout, b"%%%%EndSetup\n\0" as *const u8 as *const libc::c_char);
    file_initialized = 1 as libc::c_int;
}
unsafe extern "C" fn init_svg(mut str: *mut libc::c_char) {
    output = Some(
        svg_output
            as unsafe extern "C" fn(*mut FILE, *const libc::c_char, ...) -> libc::c_int,
    );
    if file_initialized > 0 as libc::c_int {
        fprintf(
            __stderrp,
            b"??? init_svg: file_initialized\n\0" as *const u8 as *const libc::c_char,
        );
    }
    define_svg_symbols(
        str,
        nepsf,
        (if (*p_fmt).landscape != 0 { (*p_fmt).pageheight } else { (*p_fmt).pagewidth })
            - cur_lmarg - max_rmarg,
        -bposy,
    );
    file_initialized = 1 as libc::c_int;
    user_ps_write();
}
unsafe extern "C" fn close_fout() {
    let mut m: libc::c_long = 0;
    if !(fout == __stdoutp) {
        if !(quiet != 0) {
            m = ftell(fout);
            if epsf != 0 || svg == 1 as libc::c_int {
                printf(
                    b"Output written on %s (%ld bytes)\n\0" as *const u8
                        as *const libc::c_char,
                    outfnam.as_mut_ptr(),
                    m,
                );
            } else {
                printf(
                    b"Output written on %s (%d page%s, %d title%s, %ld bytes)\n\0"
                        as *const u8 as *const libc::c_char,
                    outfnam.as_mut_ptr(),
                    nbpages,
                    if nbpages == 1 as libc::c_int {
                        b"\0" as *const u8 as *const libc::c_char
                    } else {
                        b"s\0" as *const u8 as *const libc::c_char
                    },
                    tunenum,
                    if tunenum == 1 as libc::c_int {
                        b"\0" as *const u8 as *const libc::c_char
                    } else {
                        b"s\0" as *const u8 as *const libc::c_char
                    },
                    m,
                );
            }
        }
        fclose(fout);
    }
    fout = 0 as *mut FILE;
    file_initialized = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn close_output_file() {
    if fout.is_null() {
        return;
    }
    if multicol_start != 0 as libc::c_int as libc::c_float {
        error(
            1 as libc::c_int,
            0 as *mut SYMBOL,
            b"No \"%%%%multicol end\"\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        multicol_start = 0 as libc::c_int as libc::c_float;
        write_buffer();
    }
    if tunenum == 0 as libc::c_int {
        error(
            0 as libc::c_int,
            0 as *mut SYMBOL,
            b"No tunes written to output file\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    close_page();
    let mut current_block_14: u64;
    match svg {
        0 => {
            if epsf == 0 as libc::c_int {
                fprintf(
                    fout,
                    b"%%%%Trailer\n%%%%Pages: %d\n%%EOF\n\0" as *const u8
                        as *const libc::c_char,
                    nbpages,
                );
            }
            close_fout();
            current_block_14 = 13586036798005543211;
        }
        2 => {
            fputs(b"</body>\n</html>\n\0" as *const u8 as *const libc::c_char, fout);
            current_block_14 = 14400496210763133424;
        }
        3 => {
            current_block_14 = 14400496210763133424;
        }
        _ => {
            current_block_14 = 13586036798005543211;
        }
    }
    match current_block_14 {
        14400496210763133424 => {
            close_fout();
        }
        _ => {}
    }
    tunenum = 0 as libc::c_int;
    nbpages = tunenum;
    defl = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn close_page() {
    if in_page == 0 {
        return;
    }
    in_page = 0 as libc::c_int;
    if svg != 0 {
        svg_close();
        if svg == 1 as libc::c_int && fout != __stdoutp {
            close_fout();
        }
    } else {
        fprintf(
            fout,
            b"grestore\nshowpage\n%%%%EndPage: %d %d\n\0" as *const u8
                as *const libc::c_char,
            nbpages,
            nbpages,
        );
    }
    cur_lmarg = 0 as libc::c_int as libc::c_float;
    cur_scale = 1.0f64 as libc::c_float;
    outft = -(1 as libc::c_int);
    use_buffer = 0 as libc::c_int;
}
unsafe extern "C" fn format_hf(mut d: *mut libc::c_char, mut p: *mut libc::c_char) {
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ltime: time_t = 0;
    let mut current_block_21: u64;
    while !(*p as libc::c_int == '\0' as i32) {
        q = strchr(p, '$' as i32);
        if !q.is_null() {
            *q = '\0' as i32 as libc::c_char;
        }
        d = d
            .offset(sprintf(d, b"%s\0" as *const u8 as *const libc::c_char, p) as isize);
        if q.is_null() {
            break;
        }
        p = q.offset(1 as libc::c_int as isize);
        match *p as libc::c_int {
            100 => {
                ltime = mtime;
                current_block_21 = 8747987628535927043;
            }
            68 => {
                time(&mut ltime);
                current_block_21 = 8747987628535927043;
            }
            70 => {
                d = d
                    .offset(
                        sprintf(d, b"%s\0" as *const u8 as *const libc::c_char, in_fname)
                            as isize,
                    );
                current_block_21 = 10692455896603418738;
            }
            73 => {
                p = p.offset(1);
                p;
                if (*p as libc::c_int) < 'A' as i32 || *p as libc::c_int > 'Z' as i32
                    || (info[(*p as libc::c_int - 'A' as i32) as usize]).is_null()
                {
                    current_block_21 = 10692455896603418738;
                } else {
                    d = d
                        .offset(
                            sprintf(
                                d,
                                b"%s\0" as *const u8 as *const libc::c_char,
                                &mut *((**info
                                    .as_mut_ptr()
                                    .offset((*p as libc::c_int - 'A' as i32) as isize))
                                    .text)
                                    .offset(2 as libc::c_int as isize) as *mut libc::c_char,
                            ) as isize,
                        );
                    current_block_21 = 10692455896603418738;
                }
            }
            80 => {
                if *p.offset(1 as libc::c_int as isize) as libc::c_int == '0' as i32 {
                    p = p.offset(1);
                    p;
                    if pagenum & 1 as libc::c_int != 0 {
                        current_block_21 = 10692455896603418738;
                    } else {
                        current_block_21 = 14576567515993809846;
                    }
                } else if *p.offset(1 as libc::c_int as isize) as libc::c_int
                    == '1' as i32
                {
                    p = p.offset(1);
                    p;
                    if pagenum & 1 as libc::c_int == 0 as libc::c_int {
                        current_block_21 = 10692455896603418738;
                    } else {
                        current_block_21 = 14576567515993809846;
                    }
                } else {
                    current_block_21 = 14576567515993809846;
                }
                match current_block_21 {
                    10692455896603418738 => {}
                    _ => {
                        d = d
                            .offset(
                                sprintf(
                                    d,
                                    b"%d\0" as *const u8 as *const libc::c_char,
                                    pagenum,
                                ) as isize,
                            );
                        current_block_21 = 10692455896603418738;
                    }
                }
            }
            81 => {
                if *p.offset(1 as libc::c_int as isize) as libc::c_int == '0' as i32 {
                    p = p.offset(1);
                    p;
                    if pagenum_nr & 1 as libc::c_int != 0 {
                        current_block_21 = 10692455896603418738;
                    } else {
                        current_block_21 = 11932355480408055363;
                    }
                } else if *p.offset(1 as libc::c_int as isize) as libc::c_int
                    == '1' as i32
                {
                    p = p.offset(1);
                    p;
                    if pagenum_nr & 1 as libc::c_int == 0 as libc::c_int {
                        current_block_21 = 10692455896603418738;
                    } else {
                        current_block_21 = 11932355480408055363;
                    }
                } else {
                    current_block_21 = 11932355480408055363;
                }
                match current_block_21 {
                    10692455896603418738 => {}
                    _ => {
                        d = d
                            .offset(
                                sprintf(
                                    d,
                                    b"%d\0" as *const u8 as *const libc::c_char,
                                    pagenum_nr,
                                ) as isize,
                            );
                        current_block_21 = 10692455896603418738;
                    }
                }
            }
            84 => {
                q = &mut *((**info
                    .as_mut_ptr()
                    .offset(('T' as i32 - 'A' as i32) as isize))
                    .text)
                    .offset(2 as libc::c_int as isize) as *mut libc::c_char;
                tex_str(q);
                d = d
                    .offset(
                        sprintf(
                            d,
                            b"%s\0" as *const u8 as *const libc::c_char,
                            tex_buf.as_mut_ptr(),
                        ) as isize,
                    );
                current_block_21 = 10692455896603418738;
            }
            86 => {
                d = d
                    .offset(
                        sprintf(
                            d,
                            b"abcm2ps-8.14.14\0" as *const u8 as *const libc::c_char,
                        ) as isize,
                    );
                current_block_21 = 10692455896603418738;
            }
            _ => {
                continue;
            }
        }
        match current_block_21 {
            8747987628535927043 => {
                cnv_date(&mut ltime);
                d = d
                    .offset(
                        sprintf(
                            d,
                            b"%s\0" as *const u8 as *const libc::c_char,
                            tex_buf.as_mut_ptr(),
                        ) as isize,
                    );
            }
            _ => {}
        }
        p = p.offset(1);
        p;
    }
    *d = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn headfooter(
    mut header: libc::c_int,
    mut pwidth: libc::c_float,
    mut pheight: libc::c_float,
) -> libc::c_float {
    let mut tmp: [libc::c_char; 2048] = [0; 2048];
    let mut str: [libc::c_char; 1024] = [0; 1024];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut outbuf_sav: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mbf_sav: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    let mut wsize: libc::c_float = 0.;
    let mut f: *mut FONTSPEC = 0 as *mut FONTSPEC;
    let mut f_sav: FONTSPEC = FONTSPEC {
        fnum: 0,
        size: 0.,
        swfac: 0.,
    };
    let mut cft_sav: libc::c_int = 0;
    let mut dft_sav: libc::c_int = 0;
    let mut outbufsz_sav: libc::c_int = 0;
    if header != 0 {
        if pagenum & 1 as libc::c_int != 0 {
            p = if !(cfmt.header).is_null() { cfmt.header } else { cfmt.header2 };
        } else {
            p = if !(cfmt.header2).is_null() { cfmt.header2 } else { cfmt.header };
        }
        f = &mut *(cfmt.font_tb).as_mut_ptr().offset(HEADERFONT as libc::c_int as isize)
            as *mut FONTSPEC;
        size = (*f).size;
        y = -size;
    } else {
        if pagenum & 1 as libc::c_int != 0 {
            p = if !(cfmt.footer).is_null() { cfmt.footer } else { cfmt.footer2 };
        } else {
            p = if !(cfmt.footer2).is_null() { cfmt.footer2 } else { cfmt.footer };
        }
        f = &mut *(cfmt.font_tb).as_mut_ptr().offset(FOOTERFONT as libc::c_int as isize)
            as *mut FONTSPEC;
        size = (*f).size;
        y = -(pheight - cfmt.topmargin - cfmt.botmargin) + size;
    }
    if *p as libc::c_int == '-' as i32 {
        if pagenum == 1 as libc::c_int {
            return 0 as libc::c_int as libc::c_float;
        }
        p = p.offset(1);
        p;
    }
    get_str_font(&mut cft_sav, &mut dft_sav);
    memcpy(
        &mut f_sav as *mut FONTSPEC as *mut libc::c_void,
        &mut *(cfmt.font_tb).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut FONTSPEC as *const libc::c_void,
        ::core::mem::size_of::<FONTSPEC>() as libc::c_ulong,
    );
    str_font(f.offset_from((cfmt.font_tb).as_mut_ptr()) as libc::c_long as libc::c_int);
    output
        .expect(
            "non-null function pointer",
        )(
        fout,
        b"%.1f F%d \0" as *const u8 as *const libc::c_char,
        size as libc::c_double,
        (*f).fnum,
    );
    outft = f.offset_from((cfmt.font_tb).as_mut_ptr()) as libc::c_long as libc::c_int;
    wsize = size;
    r = strstr(p, b"\\n\0" as *const u8 as *const libc::c_char);
    if !r.is_null() {
        if header == 0 {
            y += size;
        }
        wsize += size;
        *r = '\0' as i32 as libc::c_char;
    }
    mbf_sav = mbf;
    outbuf_sav = outbuf;
    outbufsz_sav = outbufsz;
    outbuf = tmp.as_mut_ptr();
    outbufsz = ::core::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong
        as libc::c_int;
    loop {
        tex_str(p);
        strcpy(tmp.as_mut_ptr(), tex_buf.as_mut_ptr());
        format_hf(str.as_mut_ptr(), tmp.as_mut_ptr());
        p = str.as_mut_ptr();
        q = strchr(p, '\t' as i32);
        if !q.is_null() {
            if q != p {
                *q = '\0' as i32 as libc::c_char;
                output
                    .expect(
                        "non-null function pointer",
                    )(
                    fout,
                    b"%.1f %.1f M \0" as *const u8 as *const libc::c_char,
                    (*p_fmt).leftmargin as libc::c_double,
                    y as libc::c_double,
                );
                mbf = tmp.as_mut_ptr();
                str_out(p, 0 as libc::c_int);
                a2b(b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
                if svg != 0 {
                    svg_write(tmp.as_mut_ptr(), strlen(tmp.as_mut_ptr()) as libc::c_int);
                } else {
                    fputs(tmp.as_mut_ptr(), fout);
                }
            }
            p = q.offset(1 as libc::c_int as isize);
        }
        q = strchr(p, '\t' as i32);
        if !q.is_null() {
            *q = '\0' as i32 as libc::c_char;
        }
        if q != p {
            output
                .expect(
                    "non-null function pointer",
                )(
                fout,
                b"%.1f %.1f M \0" as *const u8 as *const libc::c_char,
                pwidth as libc::c_double * 0.5f64,
                y as libc::c_double,
            );
            mbf = tmp.as_mut_ptr();
            str_out(p, 1 as libc::c_int);
            a2b(b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
            if svg != 0 {
                svg_write(tmp.as_mut_ptr(), strlen(tmp.as_mut_ptr()) as libc::c_int);
            } else {
                fputs(tmp.as_mut_ptr(), fout);
            }
        }
        if !q.is_null() {
            p = q.offset(1 as libc::c_int as isize);
            if *p as libc::c_int != '\0' as i32 {
                output
                    .expect(
                        "non-null function pointer",
                    )(
                    fout,
                    b"%.1f %.1f M \0" as *const u8 as *const libc::c_char,
                    (pwidth - (*p_fmt).rightmargin) as libc::c_double,
                    y as libc::c_double,
                );
                mbf = tmp.as_mut_ptr();
                str_out(p, 2 as libc::c_int);
                a2b(b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
                if svg != 0 {
                    svg_write(tmp.as_mut_ptr(), strlen(tmp.as_mut_ptr()) as libc::c_int);
                } else {
                    fputs(tmp.as_mut_ptr(), fout);
                }
            }
        }
        if r.is_null() {
            break;
        }
        *r = '\\' as i32 as libc::c_char;
        p = r.offset(2 as libc::c_int as isize);
        r = 0 as *mut libc::c_char;
        y -= size;
    }
    outbuf = outbuf_sav;
    outbufsz = outbufsz_sav;
    mbf = mbf_sav;
    memcpy(
        &mut *(cfmt.font_tb).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut FONTSPEC as *mut libc::c_void,
        &mut f_sav as *mut FONTSPEC as *const libc::c_void,
        ::core::mem::size_of::<FONTSPEC>() as libc::c_ulong,
    );
    set_str_font(cft_sav, dft_sav);
    return wsize;
}
unsafe extern "C" fn init_page() {
    let mut pheight: libc::c_float = 0.;
    let mut pwidth: libc::c_float = 0.;
    p_fmt = if (info[('X' as i32 - 'A' as i32) as usize]).is_null() {
        &mut cfmt
    } else {
        &mut dfmt
    };
    nbpages += 1;
    nbpages;
    if svg != 0 {
        if file_initialized <= 0 as libc::c_int {
            if fout.is_null() {
                open_fout();
            }
            define_svg_symbols(
                in_fname,
                nbpages,
                if cfmt.landscape != 0 {
                    (*p_fmt).pageheight
                } else {
                    (*p_fmt).pagewidth
                },
                if cfmt.landscape != 0 {
                    (*p_fmt).pagewidth
                } else {
                    (*p_fmt).pageheight
                },
            );
            user_ps_write();
            file_initialized = 1 as libc::c_int;
            output = Some(
                svg_output
                    as unsafe extern "C" fn(
                        *mut FILE,
                        *const libc::c_char,
                        ...
                    ) -> libc::c_int,
            );
        } else {
            define_svg_symbols(
                in_fname,
                nbpages,
                if cfmt.landscape != 0 {
                    (*p_fmt).pageheight
                } else {
                    (*p_fmt).pagewidth
                },
                if cfmt.landscape != 0 {
                    (*p_fmt).pagewidth
                } else {
                    (*p_fmt).pageheight
                },
            );
        }
    } else if file_initialized <= 0 as libc::c_int {
        init_ps(in_fname);
    }
    in_page = 1 as libc::c_int;
    outft = -(1 as libc::c_int);
    if svg == 0 {
        fprintf(
            fout,
            b"%%%%Page: %d %d\n\0" as *const u8 as *const libc::c_char,
            nbpages,
            nbpages,
        );
    }
    if cfmt.landscape != 0 {
        pheight = (*p_fmt).pagewidth;
        pwidth = cfmt.pageheight;
        if svg == 0 {
            fprintf(
                fout,
                b"%%%%PageOrientation: Landscape\ngsave 0.75 dup scale 90 rotate 0 %.1f T\n\0"
                    as *const u8 as *const libc::c_char,
                -cfmt.topmargin as libc::c_double,
            );
        }
    } else {
        pheight = cfmt.pageheight;
        pwidth = (*p_fmt).pagewidth;
        if svg == 0 {
            fprintf(
                fout,
                b"gsave 0.75 dup scale 0 %.1f T\n\0" as *const u8 as *const libc::c_char,
                (pheight - cfmt.topmargin) as libc::c_double,
            );
        }
    }
    if svg != 0 {
        output
            .expect(
                "non-null function pointer",
            )(
            fout,
            b"0 %.1f T\n\0" as *const u8 as *const libc::c_char,
            -cfmt.topmargin as libc::c_double,
        );
    } else {
        output
            .expect(
                "non-null function pointer",
            )(
            fout,
            b"%% --- width %.1f\n\0" as *const u8 as *const libc::c_char,
            ((pwidth - cfmt.leftmargin - cfmt.rightmargin) / cfmt.scale)
                as libc::c_double,
        );
    }
    maxy = pheight - cfmt.topmargin - cfmt.botmargin;
    remy = maxy;
    if (cfmt.header).is_null() {
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        match pagenumbers {
            1 => {
                p = b"$P\t\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            2 => {
                p = b"\t\t$P\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            3 => {
                p = b"$P0\t\t$P1\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            4 => {
                p = b"$P1\t\t$P0\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            _ => {}
        }
        if !p.is_null() {
            cfmt.header = strdup(p);
        }
    }
    if !(cfmt.header).is_null() || !(cfmt.header2).is_null() {
        let mut dy: libc::c_float = 0.;
        dy = headfooter(1 as libc::c_int, pwidth, pheight);
        if dy != 0 as libc::c_int as libc::c_float {
            output
                .expect(
                    "non-null function pointer",
                )(
                fout,
                b"0 %.1f T\n\0" as *const u8 as *const libc::c_char,
                -dy as libc::c_double,
            );
            remy -= dy;
        }
    }
    if !(cfmt.footer).is_null() || !(cfmt.footer2).is_null() {
        remy -= headfooter(0 as libc::c_int, pwidth, pheight);
    }
    pagenum += 1;
    pagenum;
    pagenum_nr += 1;
    pagenum_nr;
    outft = -(1 as libc::c_int);
}
unsafe extern "C" fn epsf_fn_adj(mut p: *mut libc::c_char) {
    let mut c: libc::c_char = 0;
    loop {
        c = *p;
        if !(c as libc::c_int != '\0' as i32) {
            break;
        }
        if c as libc::c_int == ' ' as i32 {
            *p = '_' as i32 as libc::c_char;
        } else if c as libc::c_int == '/' as i32
            || c as libc::c_uint >= 127 as libc::c_int as libc::c_uint
        {
            *p = '.' as i32 as libc::c_char;
        }
        p = p.offset(1);
        p;
    };
}
unsafe extern "C" fn epsf_title(mut p: *mut libc::c_char, mut sz: libc::c_int) {
    let mut c: libc::c_uchar = 0;
    snprintf(
        p,
        sz as libc::c_ulong,
        b"%.72s (%.4s)\0" as *const u8 as *const libc::c_char,
        in_fname,
        &mut *((**info.as_mut_ptr().offset(('X' as i32 - 'A' as i32) as isize)).text)
            .offset(2 as libc::c_int as isize) as *mut libc::c_char,
    );
    loop {
        c = *p as libc::c_uchar;
        if !(c as libc::c_int != '\0' as i32) {
            break;
        }
        if c as libc::c_int >= 0x80 as libc::c_int {
            if c as libc::c_int & 0xf8 as libc::c_int == 0xf0 as libc::c_int {
                if *p.offset(1 as libc::c_int as isize) as libc::c_int
                    & 0xc0 as libc::c_int != 0x80 as libc::c_int
                    && *p.offset(2 as libc::c_int as isize) as libc::c_int
                        & 0xc0 as libc::c_int != 0x80 as libc::c_int
                    && *p.offset(3 as libc::c_int as isize) as libc::c_int
                        & 0xc0 as libc::c_int != 0x80 as libc::c_int
                {
                    *p = ' ' as i32 as libc::c_char;
                }
            } else if c as libc::c_int & 0xf0 as libc::c_int == 0xe0 as libc::c_int {
                if *p.offset(1 as libc::c_int as isize) as libc::c_int
                    & 0xc0 as libc::c_int != 0x80 as libc::c_int
                    && *p.offset(2 as libc::c_int as isize) as libc::c_int
                        & 0xc0 as libc::c_int != 0x80 as libc::c_int
                {
                    *p = ' ' as i32 as libc::c_char;
                }
            } else if c as libc::c_int & 0xe0 as libc::c_int == 0xc0 as libc::c_int {
                if *p.offset(1 as libc::c_int as isize) as libc::c_int
                    & 0xc0 as libc::c_int != 0x80 as libc::c_int
                {
                    *p = ' ' as i32 as libc::c_char;
                }
            } else {
                *p = ' ' as i32 as libc::c_char;
            }
        }
        p = p.offset(1);
        p;
    };
}
#[no_mangle]
pub unsafe extern "C" fn write_eps() {
    let mut i: libc::c_uint = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut title: [libc::c_char; 80] = [0; 80];
    if mbf == outbuf || (info[('X' as i32 - 'A' as i32) as usize]).is_null() {
        return;
    }
    p_fmt = &mut cfmt;
    if epsf != 3 as libc::c_int {
        strcpy(outfnam.as_mut_ptr(), outfn.as_mut_ptr());
        if outfnam[0 as libc::c_int as usize] as libc::c_int == '\0' as i32 {
            strcpy(
                outfnam.as_mut_ptr(),
                b"Out.ps\0" as *const u8 as *const libc::c_char,
            );
        }
        cutext(outfnam.as_mut_ptr());
        i = (strlen(outfnam.as_mut_ptr()))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_uint;
        if i == 0 as libc::c_int as libc::c_uint
            && outfnam[0 as libc::c_int as usize] as libc::c_int == '-' as i32
        {
            if epsf == 1 as libc::c_int {
                error(
                    1 as libc::c_int,
                    0 as *mut SYMBOL,
                    b"Cannot use stdout with '-E' - abort\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            fout = __stdoutp;
        } else {
            if outfnam[i as usize] as libc::c_int == '=' as i32 {
                p = &mut *((**info
                    .as_mut_ptr()
                    .offset(('T' as i32 - 'A' as i32) as isize))
                    .text)
                    .offset(2 as libc::c_int as isize) as *mut libc::c_char;
                while isspace(*p as libc::c_uchar as libc::c_int) != 0 {
                    p = p.offset(1);
                    p;
                }
                strncpy(
                    &mut *outfnam.as_mut_ptr().offset(i as isize),
                    p,
                    (::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                        .wrapping_sub(i as libc::c_ulong)
                        .wrapping_sub(4 as libc::c_int as libc::c_ulong),
                );
                outfnam[(::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                    .wrapping_sub(5 as libc::c_int as libc::c_ulong)
                    as usize] = '\0' as i32 as libc::c_char;
                epsf_fn_adj(&mut *outfnam.as_mut_ptr().offset(i as isize));
            } else {
                if i as libc::c_ulong
                    >= (::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                        .wrapping_sub(4 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(3 as libc::c_int as libc::c_ulong)
                {
                    i = (::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                        .wrapping_sub(4 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(3 as libc::c_int as libc::c_ulong) as libc::c_uint;
                }
                nepsf += 1;
                sprintf(
                    &mut *outfnam
                        .as_mut_ptr()
                        .offset(
                            i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ) as *mut libc::c_char,
                    b"%03d\0" as *const u8 as *const libc::c_char,
                    nepsf,
                );
            }
            strcat(
                outfnam.as_mut_ptr(),
                if epsf == 1 as libc::c_int {
                    b".eps\0" as *const u8 as *const libc::c_char
                } else {
                    b".svg\0" as *const u8 as *const libc::c_char
                },
            );
            fout = fopen(
                outfnam.as_mut_ptr(),
                b"w\0" as *const u8 as *const libc::c_char,
            );
            if fout.is_null() {
                error(
                    1 as libc::c_int,
                    0 as *mut SYMBOL,
                    b"Cannot open output file %s - abort\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    outfnam.as_mut_ptr(),
                );
                exit(1 as libc::c_int);
            }
        }
    }
    epsf_title(
        title.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong as libc::c_int,
    );
    if epsf == 1 as libc::c_int {
        init_ps(title.as_mut_ptr());
        fprintf(
            fout,
            b"0.75 dup scale 0 %.1f T\n\0" as *const u8 as *const libc::c_char,
            -bposy as libc::c_double,
        );
        write_buffer();
        fprintf(fout, b"showpage\nrestore\n\0" as *const u8 as *const libc::c_char);
    } else {
        if epsf == 3 as libc::c_int && file_initialized == 0 as libc::c_int {
            fputs(b"<br/>\n\0" as *const u8 as *const libc::c_char, fout);
        }
        init_svg(title.as_mut_ptr());
        write_buffer();
        svg_close();
    }
    if epsf != 3 as libc::c_int {
        close_fout();
    } else {
        file_initialized = 0 as libc::c_int;
    }
    cur_scale = 1.0f64 as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn a2b(mut fmt: *mut libc::c_char, mut args: ...) {
    let mut args_0: va_list = 0 as *mut libc::c_char;
    if mbf.offset(512 as libc::c_int as isize) > outbuf.offset(outbufsz as isize) {
        if epsf != 0 {
            error(
                1 as libc::c_int,
                0 as *mut SYMBOL,
                b"Output buffer overflow - increase outbufsz\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
            fprintf(__stderrp, b"*** abort\n\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        error(
            0 as libc::c_int,
            0 as *mut SYMBOL,
            b"Possible buffer overflow\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        write_buffer();
    }
    va_start(args_0, fmt);
    mbf = mbf
        .offset(
            vsnprintf(
                mbf,
                outbuf.offset(outbufsz as isize).offset_from(mbf) as libc::c_long
                    as libc::c_ulong,
                fmt,
                args_0,
            ) as isize,
        );
    va_end(args_0);
}
#[no_mangle]
pub unsafe extern "C" fn bskip(mut h: libc::c_float) {
    if h == 0 as libc::c_int as libc::c_float {
        return;
    }
    bposy -= h * cfmt.scale;
    a2b(
        b"0 %.2f T\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        -h as libc::c_double,
    );
}
#[no_mangle]
pub unsafe extern "C" fn init_outbuf(mut kbsz: libc::c_int) {
    if !outbuf.is_null() {
        free(outbuf as *mut libc::c_void);
    }
    outbufsz = kbsz * 1024 as libc::c_int;
    outbuf = malloc(outbufsz as libc::c_ulong) as *mut libc::c_char;
    if outbuf.is_null() {
        error(
            1 as libc::c_int,
            0 as *mut SYMBOL,
            b"Out of memory for outbuf - abort\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    bposy = 0 as libc::c_int as libc::c_float;
    ln_num = 0 as libc::c_int;
    mbf = outbuf;
}
#[no_mangle]
pub unsafe extern "C" fn write_buffer() {
    let mut p_buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: libc::c_int = 0;
    let mut np: libc::c_int = 0;
    let mut p1: libc::c_float = 0.;
    let mut dp: libc::c_float = 0.;
    let mut outft_sav: libc::c_int = 0;
    if mbf == outbuf || multicol_start != 0 as libc::c_int as libc::c_float {
        return;
    }
    if in_page == 0 && epsf == 0 {
        init_page();
    }
    outft_sav = outft;
    p1 = 0 as libc::c_int as libc::c_float;
    p_buf = outbuf;
    l = 0 as libc::c_int;
    while l < ln_num {
        if ln_pos[l as usize] > 0 as libc::c_int as libc::c_float {
            let mut ll: libc::c_int = 0;
            let mut pos: libc::c_float = 0.;
            ll = l + 1 as libc::c_int;
            while ll < ln_num {
                if ln_pos[ll as usize] <= 0 as libc::c_int as libc::c_float {
                    pos = ln_pos[ll as usize];
                    loop {
                        ll -= 1;
                        if !(ll >= l) {
                            break;
                        }
                        ln_pos[ll as usize] = pos;
                    }
                    break;
                } else {
                    ll += 1;
                    ll;
                }
            }
        }
        dp = ln_pos[l as usize] - p1;
        np = (remy + dp < 0 as libc::c_int as libc::c_float && epsf == 0) as libc::c_int;
        if np != 0 {
            close_page();
            init_page();
            if ln_font[l as usize] as libc::c_int >= 0 as libc::c_int {
                let mut f: *mut FONTSPEC = 0 as *mut FONTSPEC;
                f = &mut *(cfmt.font_tb)
                    .as_mut_ptr()
                    .offset(*ln_font.as_mut_ptr().offset(l as isize) as isize)
                    as *mut FONTSPEC;
                output
                    .expect(
                        "non-null function pointer",
                    )(
                    fout,
                    b"%.1f F%d\n\0" as *const u8 as *const libc::c_char,
                    (*f).size as libc::c_double,
                    (*f).fnum,
                );
            }
        }
        if ln_scale[l as usize] != cur_scale {
            output
                .expect(
                    "non-null function pointer",
                )(
                fout,
                b"%.3f dup scale\n\0" as *const u8 as *const libc::c_char,
                (ln_scale[l as usize] / cur_scale) as libc::c_double,
            );
            cur_scale = ln_scale[l as usize];
        }
        if ln_lmarg[l as usize] != cur_lmarg {
            output
                .expect(
                    "non-null function pointer",
                )(
                fout,
                b"%.2f 0 T\n\0" as *const u8 as *const libc::c_char,
                ((ln_lmarg[l as usize] - cur_lmarg) / cur_scale) as libc::c_double,
            );
            cur_lmarg = ln_lmarg[l as usize];
        }
        if np != 0 {
            output
                .expect(
                    "non-null function pointer",
                )(
                fout,
                b"0 %.2f T\n\0" as *const u8 as *const libc::c_char,
                -cfmt.topspace as libc::c_double,
            );
            remy -= cfmt.topspace * cfmt.scale;
        }
        if *p_buf as libc::c_int != '\u{1}' as i32 {
            if epsf > 1 as libc::c_int || svg != 0 {
                svg_write(
                    p_buf,
                    (ln_buf[l as usize]).offset_from(p_buf) as libc::c_long
                        as libc::c_int,
                );
            } else {
                fwrite(
                    p_buf as *const libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    (ln_buf[l as usize]).offset_from(p_buf) as libc::c_long
                        as libc::c_ulong,
                    fout,
                );
            }
        } else {
            let mut f_0: *mut FILE = 0 as *mut FILE;
            let mut line: [libc::c_char; 512] = [0; 512];
            let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
            p = strchr(p_buf.offset(1 as libc::c_int as isize), '\n' as i32);
            fwrite(
                p_buf.offset(1 as libc::c_int as isize) as *const libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                p.offset_from(p_buf) as libc::c_long as libc::c_ulong,
                fout,
            );
            p_buf = p.offset(1 as libc::c_int as isize);
            p = strchr(p_buf, '%' as i32);
            let fresh0 = p;
            p = p.offset(1);
            *fresh0 = '\0' as i32 as libc::c_char;
            q = strchr(p, '\n' as i32);
            *q = '\0' as i32 as libc::c_char;
            f_0 = fopen(p, b"r\0" as *const u8 as *const libc::c_char);
            if f_0.is_null() {
                error(
                    1 as libc::c_int,
                    0 as *mut SYMBOL,
                    b"Cannot open EPS file '%s'\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    p,
                );
            } else {
                if epsf > 1 as libc::c_int || svg != 0 {
                    fprintf(
                        fout,
                        b"<!--Begin document %s-->\n\0" as *const u8
                            as *const libc::c_char,
                        p,
                    );
                    svg_output(
                        fout,
                        b"gsave\n%s T\n\0" as *const u8 as *const libc::c_char,
                        p_buf,
                    );
                    while !(fgets(
                        line.as_mut_ptr(),
                        ::core::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong
                            as libc::c_int,
                        f_0,
                    ))
                        .is_null()
                    {
                        svg_write(
                            line.as_mut_ptr(),
                            strlen(line.as_mut_ptr()) as libc::c_int,
                        );
                    }
                    svg_output(
                        fout,
                        b"grestore\n%s T\n\0" as *const u8 as *const libc::c_char,
                        p_buf,
                    );
                    fprintf(
                        fout,
                        b"<!--End document %s-->\n\0" as *const u8
                            as *const libc::c_char,
                        p,
                    );
                } else {
                    fprintf(
                        fout,
                        b"save\n/showpage{}def/setpagedevice{pop}def\n%s T\n%%%%BeginDocument: %s\n\0"
                            as *const u8 as *const libc::c_char,
                        p_buf,
                        p,
                    );
                    while !(fgets(
                        line.as_mut_ptr(),
                        ::core::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong
                            as libc::c_int,
                        f_0,
                    ))
                        .is_null()
                    {
                        fwrite(
                            line.as_mut_ptr() as *const libc::c_void,
                            1 as libc::c_int as libc::c_ulong,
                            strlen(line.as_mut_ptr()),
                            fout,
                        );
                    }
                    fprintf(
                        fout,
                        b"%%%%EndDocument\nrestore\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                fclose(f_0);
            }
        }
        p_buf = ln_buf[l as usize];
        remy += dp;
        p1 = ln_pos[l as usize];
        l += 1;
        l;
    }
    if *p_buf as libc::c_int != '\0' as i32 {
        memcpy(
            outbuf as *mut libc::c_void,
            p_buf as *const libc::c_void,
            (strlen(p_buf)).wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        mbf = &mut *outbuf
            .offset(
                (strlen
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                    ) -> libc::c_ulong)(outbuf) as isize,
            ) as *mut libc::c_char;
    } else {
        mbf = outbuf;
    }
    outft = outft_sav;
    bposy = 0 as libc::c_int as libc::c_float;
    ln_num = 0 as libc::c_int;
    if epsf != 3 as libc::c_int {
        use_buffer = 0 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn block_put() {
    if mbf == outbuf {
        return;
    }
    if remy == 0 as libc::c_int as libc::c_float {
        maxy = (if cfmt.landscape != 0 { cfmt.pagewidth } else { cfmt.pageheight })
            - cfmt.topmargin - cfmt.botmargin;
        remy = maxy;
    }
    if ln_num > 0 as libc::c_int && mbf == ln_buf[(ln_num - 1 as libc::c_int) as usize] {
        return;
    }
    if ln_num >= 80 as libc::c_int {
        let mut c: libc::c_char = 0;
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        error(
            1 as libc::c_int,
            0 as *mut SYMBOL,
            b"max number of buffer lines exceeded -- check BUFFLN\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        multicol_start = 0 as libc::c_int as libc::c_float;
        p = ln_buf[(ln_num - 1 as libc::c_int) as usize];
        c = *p;
        *p = '\0' as i32 as libc::c_char;
        write_buffer();
        multicol_start = remy + bposy;
        *p = c;
        strcpy(outbuf, p);
    }
    ln_buf[ln_num as usize] = mbf;
    ln_pos[ln_num
        as usize] = if multicol_start == 0 as libc::c_int as libc::c_float {
        bposy
    } else {
        1 as libc::c_int as libc::c_float
    };
    ln_lmarg[ln_num as usize] = cfmt.leftmargin;
    ln_scale[ln_num as usize] = cfmt.scale;
    ln_font[ln_num as usize] = outft as libc::c_schar;
    ln_num += 1;
    ln_num;
    if use_buffer == 0 {
        write_buffer();
    }
}
#[no_mangle]
pub unsafe extern "C" fn buffer_eob(mut eot: libc::c_int) {
    block_put();
    if epsf != 0 {
        if epsf == 3 as libc::c_int {
            write_eps();
        }
        return;
    }
    if remy + bposy >= 0 as libc::c_int as libc::c_float
        || multicol_start != 0 as libc::c_int as libc::c_float
    {
        return;
    }
    if in_page == 0 {
        if cfmt.splittune == 2 as libc::c_int && nbpages & 1 as libc::c_int == 0
            || cfmt.splittune == 3 as libc::c_int && nbpages & 1 as libc::c_int != 0
        {
            if remy + maxy + bposy < 0 as libc::c_int as libc::c_float {
                init_page();
                close_page();
                write_buffer();
                return;
            }
            if eot != 0 {
                write_buffer();
            }
            return;
        }
    } else {
        close_page();
    }
    if cfmt.splittune == 2 as libc::c_int && nbpages & 1 as libc::c_int == 0
        || cfmt.splittune == 3 as libc::c_int && nbpages & 1 as libc::c_int != 0
    {
        use_buffer = 1 as libc::c_int;
    } else {
        write_buffer();
    };
}
#[no_mangle]
pub unsafe extern "C" fn check_buffer() {
    if mbf.offset(5000 as libc::c_int as isize) > outbuf.offset(outbufsz as isize) {
        error(
            0 as libc::c_int,
            0 as *mut SYMBOL,
            b"Possibly bad page breaks, outbufsz exceeded\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        write_buffer();
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_bposy() -> libc::c_float {
    return remy + bposy;
}
