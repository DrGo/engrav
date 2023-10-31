#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, linkage)]
#[macro_use]
extern crate c2rust_bitfields;
use ::rs::*;
extern "C" {
    pub type __sFILEX;
    fn exit(_: libc::c_int) -> !;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn __error() -> *mut libc::c_int;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
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
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut _DefaultRuneLocale: _RuneLocale;
    fn fstat(_: libc::c_int, _: *mut stat) -> libc::c_int;
    static mut severity: libc::c_int;
    static mut parse: parse;
    static mut cfmt: FORMAT;
    fn buffer_eob(eot: libc::c_int);
    fn init_outbuf(kbsz: libc::c_int);
    fn close_output_file();
    fn open_fout();
    fn write_buffer();
    fn init_deco();
    fn lock_fmt(fmt: *mut libc::c_void);
    fn make_font_list();
    fn print_format();
    fn set_format();
    fn frontend(
        s: *mut libc::c_uchar,
        ftype: libc::c_int,
        fname: *mut libc::c_char,
        linenum: libc::c_int,
    );
    static mut multicol_start: libc::c_float;
    fn do_tune();
    fn error(sev: libc::c_int, s: *mut SYMBOL, fmt: *mut libc::c_char, _: ...);
    fn pg_init();
    static mut tex_buf: [libc::c_char; 0];
    fn time(_: *mut time_t) -> time_t;
    static mut __stdinp: *mut FILE;
    static mut __stdoutp: *mut FILE;
    static mut __stderrp: *mut FILE;
    fn fileno(_: *mut FILE) -> libc::c_int;
    fn rewind(_: *mut FILE);
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fclose(_: *mut FILE) -> libc::c_int;
    fn ferror(_: *mut FILE) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn ftell(_: *mut FILE) -> libc::c_long;
    fn fseek(_: *mut FILE, _: libc::c_long, _: libc::c_int) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fputs(_: *const libc::c_char, _: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn mmap(
        _: *mut libc::c_void,
        _: size_t,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: off_t,
    ) -> *mut libc::c_void;
    fn munmap(_: *mut libc::c_void, _: size_t) -> libc::c_int;
}
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_longlong;
pub type __uint64_t = libc::c_ulonglong;
pub type __darwin_ct_rune_t = libc::c_int;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_wchar_t = libc::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_time_t = libc::c_long;
pub type __darwin_blkcnt_t = __int64_t;
pub type __darwin_blksize_t = __int32_t;
pub type __darwin_dev_t = __int32_t;
pub type __darwin_gid_t = __uint32_t;
pub type __darwin_ino64_t = __uint64_t;
pub type __darwin_mode_t = __uint16_t;
pub type __darwin_off_t = __int64_t;
pub type __darwin_uid_t = __uint32_t;
pub type size_t = __darwin_size_t;
pub type uid_t = __darwin_uid_t;
pub type dev_t = __darwin_dev_t;
pub type mode_t = __darwin_mode_t;
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
pub struct timespec {
    pub tv_sec: __darwin_time_t,
    pub tv_nsec: libc::c_long,
}
pub type blkcnt_t = __darwin_blkcnt_t;
pub type blksize_t = __darwin_blksize_t;
pub type nlink_t = __uint16_t;
pub type gid_t = __darwin_gid_t;
pub type off_t = __darwin_off_t;
pub type time_t = __darwin_time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: dev_t,
    pub st_mode: mode_t,
    pub st_nlink: nlink_t,
    pub st_ino: __darwin_ino64_t,
    pub st_uid: uid_t,
    pub st_gid: gid_t,
    pub st_rdev: dev_t,
    pub st_atimespec: timespec,
    pub st_mtimespec: timespec,
    pub st_ctimespec: timespec,
    pub st_birthtimespec: timespec,
    pub st_size: off_t,
    pub st_blocks: blkcnt_t,
    pub st_blksize: blksize_t,
    pub st_flags: __uint32_t,
    pub st_gen: __uint32_t,
    pub st_lspare: __int32_t,
    pub st_qspare: [__int64_t; 2],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct str_a {
    pub n: *mut str_a,
    pub p: *mut libc::c_char,
    pub r: libc::c_int,
    pub sz: libc::c_int,
    pub str_0: [libc::c_char; 2],
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
pub static mut info: INFO = [0 as *const SYMBOL as *mut SYMBOL; 26];
#[no_mangle]
pub static mut sym: *mut SYMBOL = 0 as *const SYMBOL as *mut SYMBOL;
#[no_mangle]
pub static mut tunenum: libc::c_int = 0;
#[no_mangle]
pub static mut pagenum: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut pagenum_nr: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut quiet: libc::c_int = 0;
#[no_mangle]
pub static mut secure: libc::c_int = 0;
#[no_mangle]
pub static mut annotate: libc::c_int = 0;
#[no_mangle]
pub static mut pagenumbers: libc::c_int = 0;
#[no_mangle]
pub static mut epsf: libc::c_int = 0;
#[no_mangle]
pub static mut svg: libc::c_int = 0;
#[no_mangle]
pub static mut showerror: libc::c_int = 0;
#[no_mangle]
pub static mut pipeformat: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut outfn: [libc::c_char; 1024] = [0; 1024];
#[no_mangle]
pub static mut file_initialized: libc::c_int = 0;
#[no_mangle]
pub static mut fout: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut in_fname: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut mtime: time_t = 0;
static mut fmtime: time_t = 0;
#[no_mangle]
pub static mut s_argc: libc::c_int = 0;
#[no_mangle]
pub static mut s_argv: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
#[no_mangle]
pub static mut tblts: [*mut tblt_s; 8] = [0 as *const tblt_s as *mut tblt_s; 8];
#[no_mangle]
pub static mut cmdtblts: [cmdtblt_s; 4] = [cmdtblt_s {
    index: 0,
    active: 0,
    vn: 0 as *const libc::c_char as *mut libc::c_char,
}; 4];
#[no_mangle]
pub static mut ncmdtblt: libc::c_int = 0;
static mut styd: *mut libc::c_char = b"/usr/local/share/abcm2ps\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
static mut def_fmt_done: libc::c_int = 0 as libc::c_int;
static mut notitle: SYMBOL = SYMBOL {
    abc_next: 0 as *const SYMBOL as *mut SYMBOL,
    abc_prev: 0 as *const SYMBOL as *mut SYMBOL,
    next: 0 as *const SYMBOL as *mut SYMBOL,
    prev: 0 as *const SYMBOL as *mut SYMBOL,
    ts_next: 0 as *const SYMBOL as *mut SYMBOL,
    ts_prev: 0 as *const SYMBOL as *mut SYMBOL,
    extra: 0 as *const SYMBOL as *mut SYMBOL,
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
    gch: 0 as *const gch as *mut gch,
    ly: 0 as *const lyrics as *mut lyrics,
    state: 0,
    flags: 0,
    colnum: 0,
    linenum: 0,
    fn_0: 0 as *const libc::c_char as *mut libc::c_char,
    text: 0 as *const libc::c_char as *mut libc::c_char,
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
static mut str_level: libc::c_int = 0;
static mut str_r: [*mut str_a; 3] = [0 as *const str_a as *mut str_a; 3];
static mut str_c: [*mut str_a; 3] = [0 as *const str_a as *mut str_a; 3];
unsafe extern "C" fn open_ext(
    mut fn_0: *mut libc::c_char,
    mut ext: *mut libc::c_char,
) -> *mut FILE {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    fp = fopen(fn_0, b"rb\0" as *const u8 as *const libc::c_char);
    if !fp.is_null() {
        return fp;
    }
    p = strrchr(fn_0, '/' as i32);
    if p.is_null() {
        p = fn_0;
    }
    if !(strrchr(p, '.' as i32)).is_null() {
        return 0 as *mut FILE;
    }
    strcat(p, b".\0" as *const u8 as *const libc::c_char);
    strcat(p, ext);
    fp = fopen(fn_0, b"rb\0" as *const u8 as *const libc::c_char);
    if !fp.is_null() {
        return fp;
    }
    return 0 as *mut FILE;
}
#[no_mangle]
pub unsafe extern "C" fn open_file(
    mut fn_0: *mut libc::c_char,
    mut ext: *mut libc::c_char,
    mut rfn: *mut libc::c_char,
) -> *mut FILE {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: libc::c_int = 0;
    if !in_fname.is_null() && in_fname != fn_0
        && {
            p = strrchr(in_fname, '/' as i32);
            !p.is_null()
        }
    {
        l = (p.offset_from(in_fname) as libc::c_long + 1 as libc::c_int as libc::c_long)
            as libc::c_int;
        strncpy(rfn, in_fname, l as libc::c_ulong);
        strcpy(&mut *rfn.offset(l as isize), fn_0);
        fp = open_ext(rfn, ext);
        if !fp.is_null() {
            return fp;
        }
    }
    strcpy(rfn, fn_0);
    fp = open_ext(rfn, ext);
    if !fp.is_null() {
        return fp;
    }
    if *ext as libc::c_int != 'f' as i32 || *styd as libc::c_int == '\0' as i32 {
        return 0 as *mut FILE;
    }
    l = (strlen(styd)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    if *styd.offset(l as isize) as libc::c_int == '/' as i32 {
        sprintf(rfn, b"%s%s\0" as *const u8 as *const libc::c_char, styd, fn_0);
    } else {
        sprintf(
            rfn,
            b"%s%c%s\0" as *const u8 as *const libc::c_char,
            styd,
            '/' as i32,
            fn_0,
        );
    }
    return open_ext(rfn, ext);
}
unsafe extern "C" fn read_file(
    mut fn_0: *mut libc::c_char,
    mut ext: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut fsize: size_t = 0;
    let mut fin: *mut FILE = 0 as *mut FILE;
    let mut file: *mut libc::c_char = 0 as *mut libc::c_char;
    if *fn_0 as libc::c_int == '\0' as i32 {
        strcpy(tex_buf.as_mut_ptr(), b"stdin\0" as *const u8 as *const libc::c_char);
        fsize = 0 as libc::c_int as size_t;
        file = malloc(8192 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
        loop {
            let mut l: libc::c_int = 0;
            l = fread(
                &mut *file.offset(fsize as isize) as *mut libc::c_char
                    as *mut libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                8192 as libc::c_int as libc::c_ulong,
                __stdinp,
            ) as libc::c_int;
            fsize = fsize.wrapping_add(l as size_t);
            if l != 8192 as libc::c_int {
                break;
            }
            file = realloc(
                file as *mut libc::c_void,
                fsize.wrapping_add(8192 as libc::c_int as size_t),
            ) as *mut libc::c_char;
        }
        if ferror(__stdinp) != 0 as libc::c_int {
            free(file as *mut libc::c_void);
            return 0 as *mut libc::c_char;
        }
        if fsize % 8192 as libc::c_int as size_t == 0 as libc::c_int as size_t {
            file = realloc(
                file as *mut libc::c_void,
                fsize.wrapping_add(2 as libc::c_int as size_t),
            ) as *mut libc::c_char;
        }
        time(&mut fmtime);
    } else {
        let mut sbuf: stat = stat {
            st_dev: 0,
            st_mode: 0,
            st_nlink: 0,
            st_ino: 0,
            st_uid: 0,
            st_gid: 0,
            st_rdev: 0,
            st_atimespec: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtimespec: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctimespec: timespec { tv_sec: 0, tv_nsec: 0 },
            st_birthtimespec: timespec { tv_sec: 0, tv_nsec: 0 },
            st_size: 0,
            st_blocks: 0,
            st_blksize: 0,
            st_flags: 0,
            st_gen: 0,
            st_lspare: 0,
            st_qspare: [0; 2],
        };
        fin = open_file(fn_0, ext, tex_buf.as_mut_ptr());
        if fin.is_null() {
            return 0 as *mut libc::c_char;
        }
        if fseek(fin, 0 as libc::c_long, 2 as libc::c_int) < 0 as libc::c_int {
            fclose(fin);
            return 0 as *mut libc::c_char;
        }
        fsize = ftell(fin) as size_t;
        rewind(fin);
        file = malloc(fsize.wrapping_add(2 as libc::c_int as size_t))
            as *mut libc::c_char;
        if file.is_null() {
            fclose(fin);
            return 0 as *mut libc::c_char;
        }
        if fread(
            file as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            fsize,
            fin,
        ) != fsize
        {
            fclose(fin);
            free(file as *mut libc::c_void);
            return 0 as *mut libc::c_char;
        }
        fstat(fileno(fin), &mut sbuf);
        memcpy(
            &mut fmtime as *mut time_t as *mut libc::c_void,
            &mut sbuf.st_mtimespec.tv_sec as *mut __darwin_time_t as *const libc::c_void,
            ::core::mem::size_of::<time_t>() as libc::c_ulong,
        );
        fclose(fin);
    }
    *file.offset(fsize as isize) = '\0' as i32 as libc::c_char;
    return file;
}
unsafe extern "C" fn treat_file(
    mut fn_0: *mut libc::c_char,
    mut ext: *mut libc::c_char,
) {
    let mut file: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut abc_fn: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut file_type: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    if fout.is_null() {
        read_def_format();
        if strcmp(fn_0, tex_buf.as_mut_ptr()) == 0 as libc::c_int {
            return;
        }
    }
    file = read_file(fn_0, ext);
    if file.is_null() {
        if strcmp(fn_0, b"default.fmt\0" as *const u8 as *const libc::c_char)
            != 0 as libc::c_int
        {
            error(
                1 as libc::c_int,
                0 as *mut SYMBOL,
                b"Cannot read the input file '%s'\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                fn_0,
            );
        }
        return;
    }
    abc_fn = strdup(tex_buf.as_mut_ptr());
    if quiet == 0 {
        fprintf(
            if strcmp(outfn.as_mut_ptr(), b"-\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                __stderrp
            } else {
                __stdoutp
            },
            b"File %s\n\0" as *const u8 as *const libc::c_char,
            abc_fn,
        );
    }
    l = strlen(abc_fn) as libc::c_int;
    if strcmp(
        &mut *abc_fn.offset((l - 3 as libc::c_int) as isize),
        b".ps\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        file_type = 2 as libc::c_int;
    } else if strcmp(
        &mut *abc_fn.offset((l - 4 as libc::c_int) as isize),
        b".fmt\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        file_type = 1 as libc::c_int;
    } else {
        file_type = 0 as libc::c_int;
        mtime = fmtime;
    }
    frontend(file as *mut libc::c_uchar, file_type, abc_fn, 0 as libc::c_int);
    free(file as *mut libc::c_void);
    if file_type == 2 as libc::c_int {
        frontend(
            b"%%endps\0" as *const u8 as *const libc::c_char as *mut libc::c_uchar,
            0 as libc::c_int,
            abc_fn,
            0 as libc::c_int,
        );
    }
    if file_type == 0 as libc::c_int {
        clrarena(1 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn include_file(mut fn_0: *mut libc::c_uchar) {
    static mut nbfiles: libc::c_int = 0;
    if nbfiles > 2 as libc::c_int {
        error(
            1 as libc::c_int,
            0 as *mut SYMBOL,
            b"Too many included files\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    nbfiles += 1;
    nbfiles;
    treat_file(
        fn_0 as *mut libc::c_char,
        b"fmt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    nbfiles -= 1;
    nbfiles;
}
unsafe extern "C" fn treat_abc_file(mut fn_0: *mut libc::c_char) {
    let mut fin: *mut FILE = 0 as *mut FILE;
    let mut file: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut file_tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut abc_fn: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fsize: size_t = 0;
    let mut l: size_t = 0;
    let mut l2: size_t = 0;
    let mut linenum: libc::c_int = 0;
    let mut fd: libc::c_int = 0;
    lvlarena(0 as libc::c_int);
    parse.abc_state = 0 as libc::c_int;
    if epsf != 3 as libc::c_int {
        treat_file(
            fn_0,
            b"abc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    if *fn_0 as libc::c_int == '\0' as i32 {
        error(
            1 as libc::c_int,
            0 as *mut SYMBOL,
            b"cannot use stdin with -z - aborting\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    fin = open_file(
        fn_0,
        b"abc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        tex_buf.as_mut_ptr(),
    );
    if !fin.is_null() {
        if fseek(fin, 0 as libc::c_long, 2 as libc::c_int) < 0 as libc::c_int {
            fclose(fin);
        } else {
            fsize = ftell(fin) as size_t;
            rewind(fin);
            fd = fileno(fin);
            file = mmap(
                0 as *mut libc::c_void,
                fsize,
                0x1 as libc::c_int,
                0x2 as libc::c_int,
                fd,
                0 as libc::c_int as off_t,
            ) as *mut libc::c_char;
            if !file.is_null() {
                abc_fn = strdup(tex_buf.as_mut_ptr());
                l = fsize;
                p = file;
                linenum = 0 as libc::c_int;
                while l > 0 as libc::c_int as size_t {
                    q = p;
                    l2 = l.wrapping_sub(10 as libc::c_int as size_t);
                    while l2 > 0 as libc::c_int as size_t {
                        if strncmp(
                            q,
                            b"\n%abc\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int as libc::c_ulong,
                        ) == 0 as libc::c_int
                            || strncmp(
                                q,
                                b"\nX:\0" as *const u8 as *const libc::c_char,
                                3 as libc::c_int as libc::c_ulong,
                            ) == 0 as libc::c_int
                        {
                            break;
                        }
                        l2 = l2.wrapping_sub(1);
                        l2;
                        q = q.offset(1);
                        q;
                    }
                    if l2 <= 0 as libc::c_int as size_t {
                        fwrite(
                            p as *const libc::c_void,
                            1 as libc::c_int as libc::c_ulong,
                            l,
                            fout,
                        );
                        break;
                    } else {
                        q = q.offset(1);
                        q;
                        fwrite(
                            p as *const libc::c_void,
                            1 as libc::c_int as libc::c_ulong,
                            q.offset_from(p) as libc::c_long as libc::c_ulong,
                            fout,
                        );
                        l = l.wrapping_sub(q.offset_from(p) as libc::c_long as size_t);
                        while p != q {
                            let fresh0 = p;
                            p = p.offset(1);
                            if *fresh0 as libc::c_int == '\n' as i32 {
                                linenum += 1;
                                linenum;
                            }
                        }
                        q = p;
                        l2 = l.wrapping_sub(10 as libc::c_int as size_t);
                        while l2 > 0 as libc::c_int as size_t {
                            if *q as libc::c_int == '\n' as i32
                                && *q.offset(1 as libc::c_int as isize) as libc::c_int
                                    == '<' as i32
                            {
                                break;
                            }
                            l2 = l2.wrapping_sub(1);
                            l2;
                            q = q.offset(1);
                            q;
                        }
                        if l2 <= 0 as libc::c_int as size_t {
                            error(
                                1 as libc::c_int,
                                0 as *mut SYMBOL,
                                b"no end of ABC sequence\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                            );
                            q = q.offset(9 as libc::c_int as isize);
                        }
                        q = q.offset(1);
                        q;
                        l2 = q.offset_from(p) as libc::c_long as size_t;
                        file_tmp = malloc(l2.wrapping_add(1 as libc::c_int as size_t))
                            as *mut libc::c_char;
                        if file_tmp.is_null() {
                            error(
                                1 as libc::c_int,
                                0 as *mut SYMBOL,
                                b"out of memory\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                            break;
                        } else {
                            memcpy(
                                file_tmp as *mut libc::c_void,
                                p as *const libc::c_void,
                                l2,
                            );
                            *file_tmp.offset(l2 as isize) = '\0' as i32 as libc::c_char;
                            frontend(
                                file_tmp as *mut libc::c_uchar,
                                0 as libc::c_int,
                                abc_fn,
                                linenum,
                            );
                            free(file_tmp as *mut libc::c_void);
                            clrarena(1 as libc::c_int);
                            file_initialized = -(1 as libc::c_int);
                            l = l
                                .wrapping_sub(q.offset_from(p) as libc::c_long as size_t);
                            while p != q {
                                let fresh1 = p;
                                p = p.offset(1);
                                if *fresh1 as libc::c_int == '\n' as i32 {
                                    linenum += 1;
                                    linenum;
                                }
                            }
                        }
                    }
                }
                munmap(file as *mut libc::c_void, fsize);
                fclose(fin);
                return;
            }
        }
    }
    error(
        1 as libc::c_int,
        0 as *mut SYMBOL,
        b"input file %s error %s - aborting\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        fn_0,
        strerror(*__error()),
    );
    exit(1 as libc::c_int);
}
unsafe extern "C" fn read_def_format() {
    if def_fmt_done != 0 {
        return;
    }
    def_fmt_done = 1 as libc::c_int;
    treat_file(
        b"default.fmt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"fmt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn strext(
    mut fn_0: *mut libc::c_char,
    mut ext: *mut libc::c_char,
) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    p = strrchr(fn_0, '/' as i32);
    if p.is_null() {
        p = fn_0;
    }
    q = strrchr(p, '.' as i32);
    if q.is_null() {
        strcat(p, b".\0" as *const u8 as *const libc::c_char);
    } else {
        *q.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    }
    strcat(p, ext);
}
unsafe extern "C" fn display_version(mut full: libc::c_int) {
    let mut log: *mut FILE = if strcmp(
        outfn.as_mut_ptr(),
        b"-\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        __stderrp
    } else {
        __stdoutp
    };
    fputs(b"abcm2ps-8.14.14 (2022-12-19)\n\0" as *const u8 as *const libc::c_char, log);
    if full == 0 {
        return;
    }
    fputs(b"Options: PANGO\n\0" as *const u8 as *const libc::c_char, log);
    if *styd.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
        fprintf(
            log,
            b"Default format directory: %s\n\0" as *const u8 as *const libc::c_char,
            styd,
        );
    }
}
unsafe extern "C" fn usage() {
    display_version(0 as libc::c_int);
    printf(
        b"ABC to Postscript/SVG translator.\nUsage: abcm2ps [options] file [file_options] ..\nwhere:\n file        input ABC file, or '-'\n options and file_options:\n  .output file options:\n     -E      produce EPSF output, one tune per file\n     -g      produce SVG output, one tune per file\n     -v      produce SVG output, one page per file\n     -X      produce SVG output in one XHTML file\n     -z      produce SVG output from embedded ABC\n     -O fff  set outfile name to fff\n     -O =    make outfile name from infile/title\n     -i      indicate where are the errors\n     -k kk   size of the PS output buffer in Kibytes\n  .output formatting:\n     -s xx   set scale factor to xx\n     -w xx   set staff width (cm/in/pt)\n     -m xx   set left margin (cm/in/pt)\n     -d xx   set staff separation (cm/in/pt)\n     -a xx   set max shrinkage to xx (between 0 and 1)\n     -F foo  read format file \"foo.fmt\"\n     -D bar  look for format files in directory \"bar\"\n     -p      format for bagpipes regardless of key\n  .output options:\n     -l      landscape mode\n     -I xx   indent 1st line (cm/in/pt)\n     -x      add xref numbers in titles\n     -M      don't output the lyrics\n     -N n    set page numbering mode to n=\n             0=off 1=left 2=right 3=even left,odd right 4=even right,odd left\n     -1      write one tune per page\n     -G      no slur in grace notes\n     -j n[b] number the measures every n bars (or on the left if n=0)\n             if 'b', display in a box\n     -b n    set the first measure number to n\n     -f      have flat beams\n     -T n[v]   output the tablature 'n' for voice 'v' / all voices\n  .line breaks:\n     -c      auto line break\n     -B n    break every n bars\n  .input file selection/options:\n     -e pattern\n             tune selection\n  .help/configuration:\n     -V      show program version\n     -h      show this command summary\n     -H      show the format parameters\n     -S      secure mode\n     -q      quiet mode\n\0"
            as *const u8 as *const libc::c_char,
    );
    exit(0 as libc::c_int);
}
unsafe extern "C" fn cmdtblt_parse(mut p: *mut libc::c_char) -> *mut cmdtblt_s {
    let mut cmdtblt: *mut cmdtblt_s = 0 as *mut cmdtblt_s;
    let mut val: libc::c_short = 0;
    if ncmdtblt >= 4 as libc::c_int {
        error(
            1 as libc::c_int,
            0 as *mut SYMBOL,
            b"++++ Too many '-T'\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return 0 as *mut cmdtblt_s;
    }
    if *p as libc::c_int == '\0' as i32 {
        val = -(1 as libc::c_int) as libc::c_short;
    } else {
        let fresh2 = p;
        p = p.offset(1);
        val = (*fresh2 as libc::c_int - '0' as i32 - 1 as libc::c_int) as libc::c_short;
        if val as libc::c_uint > 8 as libc::c_int as libc::c_uint {
            error(
                1 as libc::c_int,
                0 as *mut SYMBOL,
                b"++++ Bad tablature number in '-T'\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
            return 0 as *mut cmdtblt_s;
        }
    }
    let fresh3 = ncmdtblt;
    ncmdtblt = ncmdtblt + 1;
    cmdtblt = &mut *cmdtblts.as_mut_ptr().offset(fresh3 as isize) as *mut cmdtblt_s;
    (*cmdtblt).index = val;
    (*cmdtblt).vn = p;
    return cmdtblt;
}
unsafe extern "C" fn set_opt(mut w: *mut libc::c_char, mut v: *mut libc::c_char) {
    static mut prefix: libc::c_char = '%' as i32 as libc::c_char;
    if v.is_null() {
        v = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (strlen(w)).wrapping_add(strlen(v))
        >= (512 as libc::c_int - 10 as libc::c_int) as libc::c_ulong
    {
        error(
            1 as libc::c_int,
            0 as *mut SYMBOL,
            b"Command line '%s' option too long\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            w,
        );
        return;
    }
    sprintf(
        tex_buf.as_mut_ptr(),
        b"%%%c%s %s lock\n\0" as *const u8 as *const libc::c_char,
        prefix as libc::c_int,
        w,
        v,
    );
    if strcmp(w, b"abcm2ps\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        prefix = *v;
    }
    frontend(
        tex_buf.as_mut_ptr() as *mut libc::c_uchar,
        0 as libc::c_int,
        b"cmd_line\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut j: libc::c_uint = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_char = 0;
    let mut aaa: *mut libc::c_char = 0 as *mut libc::c_char;
    if argc <= 1 as libc::c_int {
        usage();
    }
    outfn[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    init_outbuf(64 as libc::c_int);
    s_argc = argc;
    s_argv = argv;
    aaa = 0 as *mut libc::c_char;
    loop {
        argc -= 1;
        if !(argc > 0 as libc::c_int) {
            break;
        }
        argv = argv.offset(1);
        argv;
        p = *argv;
        if *p as libc::c_int != '-' as i32
            || *p.offset(1 as libc::c_int as isize) as libc::c_int == '-' as i32
        {
            if *p as libc::c_int == '+' as i32
                && *p.offset(1 as libc::c_int as isize) as libc::c_int == 'F' as i32
            {
                def_fmt_done = 1 as libc::c_int;
            }
        } else {
            loop {
                p = p.offset(1);
                c = *p;
                if !(c as libc::c_int != '\0' as i32) {
                    break;
                }
                let mut current_block_56: u64;
                match c as libc::c_int {
                    69 => {
                        svg = 0 as libc::c_int;
                        epsf = 1 as libc::c_int;
                        current_block_56 = 8151474771948790331;
                    }
                    103 => {
                        svg = 0 as libc::c_int;
                        epsf = 2 as libc::c_int;
                        current_block_56 = 8151474771948790331;
                    }
                    72 => {
                        quiet = 1 as libc::c_int;
                        current_block_56 = 8151474771948790331;
                    }
                    104 => {
                        usage();
                        current_block_56 = 11111898925820647993;
                    }
                    112 => {
                        current_block_56 = 11111898925820647993;
                    }
                    113 => {
                        quiet = 1 as libc::c_int;
                        current_block_56 = 8151474771948790331;
                    }
                    83 => {
                        secure = 1 as libc::c_int;
                        current_block_56 = 8151474771948790331;
                    }
                    86 => {
                        display_version(1 as libc::c_int);
                        return 0 as libc::c_int;
                    }
                    118 => {
                        svg = 1 as libc::c_int;
                        epsf = 0 as libc::c_int;
                        current_block_56 = 8151474771948790331;
                    }
                    88 => {
                        svg = 2 as libc::c_int;
                        epsf = 0 as libc::c_int;
                        current_block_56 = 8151474771948790331;
                    }
                    107 => {
                        let mut kbsz: libc::c_int = 0;
                        if *p.offset(1 as libc::c_int as isize) as libc::c_int
                            == '\0' as i32
                        {
                            argc -= 1;
                            if argc <= 0 as libc::c_int {
                                error(
                                    1 as libc::c_int,
                                    0 as *mut SYMBOL,
                                    b"No value for '-k' - aborting\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                );
                                return 1 as libc::c_int;
                            }
                            argv = argv.offset(1);
                            aaa = *argv;
                        } else {
                            aaa = p.offset(1 as libc::c_int as isize);
                            p = p
                                .offset(
                                    (strlen(p)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        as isize,
                                );
                        }
                        sscanf(
                            aaa,
                            b"%d\0" as *const u8 as *const libc::c_char,
                            &mut kbsz as *mut libc::c_int,
                        );
                        init_outbuf(kbsz);
                        current_block_56 = 8151474771948790331;
                    }
                    79 => {
                        if *p.offset(1 as libc::c_int as isize) as libc::c_int
                            == '\0' as i32
                        {
                            argc -= 1;
                            if argc <= 0 as libc::c_int {
                                error(
                                    1 as libc::c_int,
                                    0 as *mut SYMBOL,
                                    b"No value for '-O' - aborting\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                );
                                return 1 as libc::c_int;
                            }
                            argv = argv.offset(1);
                            aaa = *argv;
                        } else {
                            aaa = p.offset(1 as libc::c_int as isize);
                            p = p
                                .offset(
                                    (strlen(p)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        as isize,
                                );
                        }
                        if strlen(aaa)
                            >= ::core::mem::size_of::<[libc::c_char; 1024]>()
                                as libc::c_ulong
                        {
                            error(
                                1 as libc::c_int,
                                0 as *mut SYMBOL,
                                b"'-O' too large - aborting\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                            );
                            return 1 as libc::c_int;
                        }
                        strcpy(outfn.as_mut_ptr(), aaa);
                        current_block_56 = 8151474771948790331;
                    }
                    122 => {
                        epsf = 3 as libc::c_int;
                        svg = 0 as libc::c_int;
                        current_block_56 = 8151474771948790331;
                    }
                    _ => {
                        if !(strchr(
                            b"aBbDdeFfIjmNOsTw\0" as *const u8 as *const libc::c_char,
                            c as libc::c_int,
                        ))
                            .is_null()
                        {
                            p = p
                                .offset(
                                    (strlen(p)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        as isize,
                                );
                        }
                        current_block_56 = 8151474771948790331;
                    }
                }
                match current_block_56 {
                    11111898925820647993 => {
                        pipeformat = 1 as libc::c_int;
                    }
                    _ => {}
                }
            }
        }
    }
    if quiet == 0 {
        display_version(0 as libc::c_int);
    }
    clrarena(0 as libc::c_int);
    clrarena(1 as libc::c_int);
    clrarena(2 as libc::c_int);
    info[('T' as i32 - 'A' as i32) as usize] = &mut notitle;
    notitle.text = b"T:\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    set_format();
    init_deco();
    pg_init();
    if epsf == 3 as libc::c_int {
        open_fout();
        read_def_format();
    }
    argc = s_argc;
    argv = s_argv;
    loop {
        argc -= 1;
        if !(argc > 0 as libc::c_int) {
            break;
        }
        argv = argv.offset(1);
        argv;
        p = *argv;
        c = *p;
        if c as libc::c_int == '\0' as i32 {
            continue;
        }
        if c as libc::c_int == '-' as i32 {
            let mut i: libc::c_int = 0;
            if *p.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
                if !in_fname.is_null() {
                    treat_abc_file(in_fname);
                    frontend(
                        b"select\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_uchar,
                        1 as libc::c_int,
                        b"cmd_line\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        0 as libc::c_int,
                    );
                }
                in_fname = b"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
                continue;
            } else {
                i = (strlen(p)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as libc::c_int;
                if *p.offset(i as isize) as libc::c_int == '-' as i32
                    && *p.offset(1 as libc::c_int as isize) as libc::c_int != '-' as i32
                    && *p.offset(1 as libc::c_int as isize) as libc::c_int != 'e' as i32
                    && *p.offset((i - 1 as libc::c_int) as isize) as libc::c_int
                        != 'O' as i32
                {
                    c = '+' as i32 as libc::c_char;
                }
            }
        }
        if c as libc::c_int == '+' as i32 {
            loop {
                p = p.offset(1);
                if !(*p as libc::c_int != '\0' as i32) {
                    break;
                }
                match *p as libc::c_int {
                    66 => {
                        cfmt.barsperstaff = 0 as libc::c_int;
                        lock_fmt(
                            &mut cfmt.barsperstaff as *mut libc::c_int
                                as *mut libc::c_void,
                        );
                    }
                    99 => {
                        cfmt.continueall = 0 as libc::c_int;
                        lock_fmt(
                            &mut cfmt.continueall as *mut libc::c_int
                                as *mut libc::c_void,
                        );
                    }
                    45 | 70 => {}
                    71 => {
                        cfmt.graceslurs = 1 as libc::c_int;
                        lock_fmt(
                            &mut cfmt.graceslurs as *mut libc::c_int as *mut libc::c_void,
                        );
                    }
                    105 => {
                        showerror = 0 as libc::c_int;
                    }
                    106 => {
                        cfmt.measurenb = -(1 as libc::c_int);
                        lock_fmt(
                            &mut cfmt.measurenb as *mut libc::c_int as *mut libc::c_void,
                        );
                    }
                    108 => {
                        cfmt.landscape = 0 as libc::c_int;
                        lock_fmt(
                            &mut cfmt.landscape as *mut libc::c_int as *mut libc::c_void,
                        );
                    }
                    77 => {
                        cfmt
                            .fields[1 as libc::c_int
                            as usize] = ((1 as libc::c_int) << 'w' as i32 - 'a' as i32)
                            as libc::c_uint;
                        lock_fmt(
                            &mut cfmt.fields as *mut [libc::c_uint; 2]
                                as *mut libc::c_void,
                        );
                    }
                    78 => {
                        pagenumbers = 0 as libc::c_int;
                    }
                    79 => {
                        outfn[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                    }
                    84 => {
                        let mut cmdtblt: *mut cmdtblt_s = 0 as *mut cmdtblt_s;
                        aaa = p.offset(1 as libc::c_int as isize);
                        if *aaa as libc::c_int == '\0' as i32 {
                            if argc > 1 as libc::c_int
                                && *(*argv.offset(1 as libc::c_int as isize))
                                    .offset(0 as libc::c_int as isize) as libc::c_int
                                    != '-' as i32
                            {
                                argv = argv.offset(1);
                                aaa = *argv;
                                argc -= 1;
                                argc;
                            }
                        } else {
                            while *p.offset(1 as libc::c_int as isize) as libc::c_int
                                != '\0' as i32
                            {
                                p = p.offset(1);
                                p;
                            }
                            if *p as libc::c_int == '-' as i32 {
                                let fresh4 = p;
                                p = p.offset(-1);
                                *fresh4 = '\0' as i32 as libc::c_char;
                            }
                        }
                        cmdtblt = cmdtblt_parse(aaa);
                        if !cmdtblt.is_null() {
                            (*cmdtblt).active = 0 as libc::c_int as libc::c_short;
                        }
                    }
                    120 => {
                        cfmt.fields[0 as libc::c_int as usize]
                            &= !((1 as libc::c_int) << 'X' as i32 - 'A' as i32)
                                as libc::c_uint;
                        lock_fmt(
                            &mut cfmt.fields as *mut [libc::c_uint; 2]
                                as *mut libc::c_void,
                        );
                    }
                    48 => {
                        cfmt.splittune = 0 as libc::c_int;
                        lock_fmt(
                            &mut cfmt.splittune as *mut libc::c_int as *mut libc::c_void,
                        );
                    }
                    49 => {
                        cfmt.oneperpage = 0 as libc::c_int;
                        lock_fmt(
                            &mut cfmt.oneperpage as *mut libc::c_int as *mut libc::c_void,
                        );
                    }
                    _ => {
                        error(
                            1 as libc::c_int,
                            0 as *mut SYMBOL,
                            b"++++ Cannot switch off flag: +%c\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            *p as libc::c_int,
                        );
                    }
                }
            }
        } else if c as libc::c_int == '-' as i32 {
            if *p.offset(1 as libc::c_int as isize) as libc::c_int == '-' as i32 {
                p = p.offset(2 as libc::c_int as isize);
                argc -= 1;
                if argc <= 0 as libc::c_int {
                    error(
                        1 as libc::c_int,
                        0 as *mut SYMBOL,
                        b"No argument for '--'\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    return 1 as libc::c_int;
                }
                argv = argv.offset(1);
                argv;
                set_opt(p, *argv);
            } else {
                loop {
                    p = p.offset(1);
                    c = *p;
                    if !(c as libc::c_int != '\0' as i32) {
                        break;
                    }
                    let mut current_block_207: u64;
                    match c as libc::c_int {
                        65 => {
                            annotate = 1 as libc::c_int;
                            current_block_207 = 9239588423676249671;
                        }
                        99 => {
                            cfmt.continueall = 1 as libc::c_int;
                            lock_fmt(
                                &mut cfmt.continueall as *mut libc::c_int
                                    as *mut libc::c_void,
                            );
                            current_block_207 = 9239588423676249671;
                        }
                        102 => {
                            cfmt.flatbeams = 1 as libc::c_int;
                            lock_fmt(
                                &mut cfmt.flatbeams as *mut libc::c_int as *mut libc::c_void,
                            );
                            current_block_207 = 9239588423676249671;
                        }
                        71 => {
                            cfmt.graceslurs = 0 as libc::c_int;
                            lock_fmt(
                                &mut cfmt.graceslurs as *mut libc::c_int
                                    as *mut libc::c_void,
                            );
                            current_block_207 = 9239588423676249671;
                        }
                        72 => {
                            if fout.is_null() {
                                read_def_format();
                                make_font_list();
                                do_tune();
                            }
                            print_format();
                            return 0 as libc::c_int;
                        }
                        105 => {
                            showerror = 1 as libc::c_int;
                            current_block_207 = 9239588423676249671;
                        }
                        108 => {
                            cfmt.landscape = 1 as libc::c_int;
                            lock_fmt(
                                &mut cfmt.landscape as *mut libc::c_int as *mut libc::c_void,
                            );
                            current_block_207 = 9239588423676249671;
                        }
                        77 => {
                            cfmt.fields[1 as libc::c_int as usize]
                                &= !((1 as libc::c_int) << 'w' as i32 - 'a' as i32)
                                    as libc::c_uint;
                            lock_fmt(
                                &mut cfmt.fields as *mut [libc::c_uint; 2]
                                    as *mut libc::c_void,
                            );
                            current_block_207 = 9239588423676249671;
                        }
                        69 | 103 | 112 | 113 | 83 | 118 | 88 | 122 => {
                            current_block_207 = 9239588423676249671;
                        }
                        120 => {
                            cfmt.fields[0 as libc::c_int as usize]
                                |= ((1 as libc::c_int) << 'X' as i32 - 'A' as i32)
                                    as libc::c_uint;
                            lock_fmt(
                                &mut cfmt.fields as *mut [libc::c_uint; 2]
                                    as *mut libc::c_void,
                            );
                            current_block_207 = 9239588423676249671;
                        }
                        48 => {
                            cfmt.splittune = 1 as libc::c_int;
                            lock_fmt(
                                &mut cfmt.splittune as *mut libc::c_int as *mut libc::c_void,
                            );
                            current_block_207 = 9239588423676249671;
                        }
                        49 => {
                            cfmt.oneperpage = 1 as libc::c_int;
                            lock_fmt(
                                &mut cfmt.oneperpage as *mut libc::c_int
                                    as *mut libc::c_void,
                            );
                            current_block_207 = 9239588423676249671;
                        }
                        78 => {
                            if *p.offset(1 as libc::c_int as isize) as libc::c_int
                                == '\0' as i32
                                && (argc <= 1 as libc::c_int
                                    || isdigit(
                                        *(*argv.offset(1 as libc::c_int as isize))
                                            .offset(0 as libc::c_int as isize) as libc::c_uint
                                            as libc::c_int,
                                    ) == 0)
                            {
                                pagenumbers = 2 as libc::c_int;
                                current_block_207 = 9239588423676249671;
                            } else {
                                current_block_207 = 7147294200370292097;
                            }
                        }
                        97 | 66 | 98 | 68 | 100 | 101 | 70 | 73 | 106 | 107 | 76 | 109
                        | 79 | 115 | 84 | 119 => {
                            current_block_207 = 7147294200370292097;
                        }
                        _ => {
                            error(
                                1 as libc::c_int,
                                0 as *mut SYMBOL,
                                b"Unknown flag: -%c ignored\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                                c as libc::c_int,
                            );
                            current_block_207 = 9239588423676249671;
                        }
                    }
                    match current_block_207 {
                        7147294200370292097 => {
                            aaa = p.offset(1 as libc::c_int as isize);
                            if *aaa as libc::c_int == '\0' as i32 {
                                argv = argv.offset(1);
                                aaa = *argv;
                                argc -= 1;
                                if argc <= 0 as libc::c_int
                                    || *aaa as libc::c_int == '-' as i32
                                        && c as libc::c_int != 'O' as i32
                                {
                                    error(
                                        1 as libc::c_int,
                                        0 as *mut SYMBOL,
                                        b"Missing parameter after '-%c' - aborting\0" as *const u8
                                            as *const libc::c_char as *mut libc::c_char,
                                        c as libc::c_int,
                                    );
                                    return 1 as libc::c_int;
                                }
                            } else {
                                p = p
                                    .offset(
                                        (strlen(p)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize,
                                    );
                            }
                            if !(strchr(
                                b"BbfjkNs\0" as *const u8 as *const libc::c_char,
                                c as libc::c_int,
                            ))
                                .is_null()
                            {
                                j = 0 as libc::c_int as libc::c_uint;
                                while (j as libc::c_ulong) < strlen(aaa) {
                                    if (strchr(
                                        b"0123456789.\0" as *const u8 as *const libc::c_char,
                                        *aaa.offset(j as isize) as libc::c_int,
                                    ))
                                        .is_null()
                                    {
                                        if *aaa.offset(j as isize) as libc::c_int == 'b' as i32
                                            && *aaa
                                                .offset(
                                                    j.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                                                ) as libc::c_int == '\0' as i32
                                            && c as libc::c_int == 'j' as i32
                                        {
                                            break;
                                        }
                                        error(
                                            1 as libc::c_int,
                                            0 as *mut SYMBOL,
                                            b"Invalid parameter <%s> for flag -%c\0" as *const u8
                                                as *const libc::c_char as *mut libc::c_char,
                                            aaa,
                                            c as libc::c_int,
                                        );
                                        return 1 as libc::c_int;
                                    } else {
                                        j = j.wrapping_add(1);
                                        j;
                                    }
                                }
                            }
                            match c as libc::c_int {
                                97 => {
                                    set_opt(
                                        b"maxshrink\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                        aaa,
                                    );
                                }
                                66 => {
                                    set_opt(
                                        b"barsperstaff\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                        aaa,
                                    );
                                }
                                98 => {
                                    set_opt(
                                        b"measurefirst\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                        aaa,
                                    );
                                }
                                68 => {
                                    styd = aaa;
                                }
                                100 => {
                                    set_opt(
                                        b"staffsep\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                        aaa,
                                    );
                                }
                                101 => {
                                    set_opt(
                                        b"select\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                        aaa,
                                    );
                                }
                                70 => {
                                    treat_file(
                                        aaa,
                                        b"fmt\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                    );
                                }
                                73 => {
                                    set_opt(
                                        b"indent\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                        aaa,
                                    );
                                }
                                106 => {
                                    sscanf(
                                        aaa,
                                        b"%d\0" as *const u8 as *const libc::c_char,
                                        &mut cfmt.measurenb as *mut libc::c_int,
                                    );
                                    lock_fmt(
                                        &mut cfmt.measurenb as *mut libc::c_int as *mut libc::c_void,
                                    );
                                    if *aaa
                                        .offset(
                                            (strlen(aaa))
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                                        ) as libc::c_int == 'b' as i32
                                    {
                                        cfmt.measurebox = 1 as libc::c_int;
                                    } else {
                                        cfmt.measurebox = 0 as libc::c_int;
                                    }
                                    lock_fmt(
                                        &mut cfmt.measurebox as *mut libc::c_int
                                            as *mut libc::c_void,
                                    );
                                }
                                109 => {
                                    set_opt(
                                        b"leftmargin\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                        aaa,
                                    );
                                }
                                78 => {
                                    sscanf(
                                        aaa,
                                        b"%d\0" as *const u8 as *const libc::c_char,
                                        &mut pagenumbers as *mut libc::c_int,
                                    );
                                    if pagenumbers as libc::c_uint
                                        > 4 as libc::c_int as libc::c_uint
                                    {
                                        error(
                                            1 as libc::c_int,
                                            0 as *mut SYMBOL,
                                            b"'-N' value %s - changed to 2\0" as *const u8
                                                as *const libc::c_char as *mut libc::c_char,
                                            aaa,
                                        );
                                        pagenumbers = 2 as libc::c_int;
                                    }
                                }
                                79 => {
                                    if strlen(aaa)
                                        >= ::core::mem::size_of::<[libc::c_char; 1024]>()
                                            as libc::c_ulong
                                    {
                                        error(
                                            1 as libc::c_int,
                                            0 as *mut SYMBOL,
                                            b"'-O' too large - aborting\0" as *const u8
                                                as *const libc::c_char as *mut libc::c_char,
                                        );
                                        exit(1 as libc::c_int);
                                    }
                                    strcpy(outfn.as_mut_ptr(), aaa);
                                }
                                115 => {
                                    set_opt(
                                        b"scale\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                        aaa,
                                    );
                                }
                                84 => {
                                    let mut cmdtblt_0: *mut cmdtblt_s = 0 as *mut cmdtblt_s;
                                    cmdtblt_0 = cmdtblt_parse(aaa);
                                    if !cmdtblt_0.is_null() {
                                        (*cmdtblt_0).active = 1 as libc::c_int as libc::c_short;
                                    }
                                }
                                119 => {
                                    set_opt(
                                        b"staffwidth\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                        aaa,
                                    );
                                }
                                107 | _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            }
        } else {
            if !in_fname.is_null() {
                treat_abc_file(in_fname);
                frontend(
                    b"select\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_uchar,
                    1 as libc::c_int,
                    b"cmd_line\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as libc::c_int,
                );
            }
            in_fname = p;
        }
    }
    if !in_fname.is_null() {
        treat_abc_file(in_fname);
    }
    if multicol_start != 0 as libc::c_int as libc::c_float {
        error(
            1 as libc::c_int,
            0 as *mut SYMBOL,
            b"Lack of %%%%multicol end\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        multicol_start = 0 as libc::c_int as libc::c_float;
        buffer_eob(0 as libc::c_int);
        if (info[('X' as i32 - 'A' as i32) as usize]).is_null() && epsf == 0 {
            write_buffer();
        }
    }
    if epsf == 0 && fout.is_null() {
        error(
            1 as libc::c_int,
            0 as *mut SYMBOL,
            b"Nothing to generate!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return 1 as libc::c_int;
    }
    close_output_file();
    return if severity == 0 as libc::c_int {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn clrarena(mut level: libc::c_int) {
    let mut a_p: *mut str_a = 0 as *mut str_a;
    a_p = str_r[level as usize];
    if a_p.is_null() {
        a_p = malloc(
            (::core::mem::size_of::<str_a>() as libc::c_ulong)
                .wrapping_add(0x4000 as libc::c_int as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong),
        ) as *mut str_a;
        str_r[level as usize] = a_p;
        (*a_p).sz = 0x4000 as libc::c_int;
        (*a_p).n = 0 as *mut str_a;
    }
    str_c[level as usize] = a_p;
    (*a_p).p = ((*a_p).str_0).as_mut_ptr();
    (*a_p).r = (*a_p).sz;
}
#[no_mangle]
pub unsafe extern "C" fn lvlarena(mut level: libc::c_int) -> libc::c_int {
    let mut old_level: libc::c_int = 0;
    old_level = str_level;
    str_level = level;
    return old_level;
}
#[no_mangle]
pub unsafe extern "C" fn getarena(mut len: libc::c_int) -> *mut libc::c_void {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut a_p: *mut str_a = 0 as *mut str_a;
    a_p = str_c[str_level as usize];
    len = len + 7 as libc::c_int & !(7 as libc::c_int);
    if len > (*a_p).r {
        if len > 0x20000 as libc::c_int {
            error(
                1 as libc::c_int,
                0 as *mut SYMBOL,
                b"getarena - data too wide %d - aborting\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                len,
            );
            exit(1 as libc::c_int);
        }
        if len > 0x4000 as libc::c_int {
            let mut a_n: *mut str_a = 0 as *mut str_a;
            a_n = (*a_p).n;
            (*a_p)
                .n = malloc(
                (::core::mem::size_of::<str_a>() as libc::c_ulong)
                    .wrapping_add(len as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong),
            ) as *mut str_a;
            (*(*a_p).n).n = a_n;
            (*(*a_p).n).sz = len;
        } else if ((*a_p).n).is_null() {
            (*a_p)
                .n = malloc(
                (::core::mem::size_of::<str_a>() as libc::c_ulong)
                    .wrapping_add(0x4000 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong),
            ) as *mut str_a;
            (*(*a_p).n).n = 0 as *mut str_a;
            (*(*a_p).n).sz = 0x4000 as libc::c_int;
        }
        a_p = (*a_p).n;
        str_c[str_level as usize] = a_p;
        (*a_p).p = ((*a_p).str_0).as_mut_ptr();
        (*a_p).r = (*a_p).sz;
    }
    p = (*a_p).p;
    (*a_p).p = ((*a_p).p).offset(len as isize);
    (*a_p).r -= len;
    return p as *mut libc::c_void;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
