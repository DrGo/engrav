use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type __sFILEX;
    fn exit(_: libc::c_int) -> !;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
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
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    static mut _DefaultRuneLocale: _RuneLocale;
    fn atan(_: libc::c_double) -> libc::c_double;
    fn cosf(_: libc::c_float) -> libc::c_float;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sinf(_: libc::c_float) -> libc::c_float;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn localtime(_: *const time_t) -> *mut tm;
    fn strftime(
        _: *mut libc::c_char,
        _: size_t,
        _: *const libc::c_char,
        _: *const tm,
    ) -> size_t;
    fn time(_: *mut time_t) -> time_t;
    static mut __stdoutp: *mut FILE;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputc(_: libc::c_int, _: *mut FILE) -> libc::c_int;
    fn fputs(_: *const libc::c_char, _: *mut FILE) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: __builtin_va_list,
    ) -> libc::c_int;
    static mut fontnames: [*mut libc::c_char; 30];
    static mut cfmt: FORMAT;
    static mut epsf: libc::c_int;
    static mut svg: libc::c_int;
    static mut in_fname: *mut libc::c_char;
    static mut file_initialized: libc::c_int;
    static mut fout: *mut FILE;
    static mut s_argc: libc::c_int;
    static mut s_argv: *mut *mut libc::c_char;
    fn cwid(c: libc::c_uchar) -> libc::c_float;
    static mut tex_buf: [libc::c_char; 0];
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
pub struct FONTSPEC {
    pub fnum: libc::c_int,
    pub size: libc::c_float,
    pub swfac: libc::c_float,
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
pub struct elt_s {
    pub next: *mut elt_s,
    pub type_0: libc::c_char,
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub v: libc::c_float,
    pub s: *mut libc::c_char,
    pub e: *mut elt_s,
}
pub const STR: elt_t = 1;
pub const VAL: elt_t = 0;
pub const BRK: elt_t = 3;
pub const SEQ: elt_t = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gc {
    pub cx: libc::c_float,
    pub cy: libc::c_float,
    pub xscale: libc::c_float,
    pub yscale: libc::c_float,
    pub xoffs: libc::c_float,
    pub yoffs: libc::c_float,
    pub rotate: libc::c_float,
    pub sin: libc::c_float,
    pub cos: libc::c_float,
    pub font_n: *mut libc::c_char,
    pub font_s: libc::c_float,
    pub font_n_old: *mut libc::c_char,
    pub linewidth: libc::c_float,
    pub rgb: libc::c_int,
    pub dash: [libc::c_char; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub def: *mut libc::c_char,
    pub use_0: libc::c_char,
    pub defined: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ps_sym_s {
    pub n: *mut libc::c_char,
    pub e: *mut elt_s,
    pub exec: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub index: libc::c_int,
    pub def: *mut libc::c_char,
}
pub type elt_t = libc::c_uint;
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
static mut elts: *mut elt_s = 0 as *const elt_s as *mut elt_s;
static mut stack: *mut elt_s = 0 as *const elt_s as *mut elt_s;
static mut free_elt: *mut elt_s = 0 as *const elt_s as *mut elt_s;
static mut ps_sym: [ps_sym_s; 512] = [ps_sym_s {
    n: 0 as *const libc::c_char as *mut libc::c_char,
    e: 0 as *const elt_s as *mut elt_s,
    exec: 0,
}; 512];
static mut n_sym: libc::c_int = 0;
static mut ps_error: libc::c_int = 0;
static mut in_cnt: libc::c_int = 0;
static mut path: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut path_buf: [libc::c_char; 256] = [0; 256];
static mut gcur: gc = gc {
    cx: 0.,
    cy: 0.,
    xscale: 0.,
    yscale: 0.,
    xoffs: 0.,
    yoffs: 0.,
    rotate: 0.,
    sin: 0.,
    cos: 0.,
    font_n: 0 as *const libc::c_char as *mut libc::c_char,
    font_s: 0.,
    font_n_old: 0 as *const libc::c_char as *mut libc::c_char,
    linewidth: 0.,
    rgb: 0,
    dash: [0; 64],
};
static mut gold: gc = gc {
    cx: 0.,
    cy: 0.,
    xscale: 0.,
    yscale: 0.,
    xoffs: 0.,
    yoffs: 0.,
    rotate: 0.,
    sin: 0.,
    cos: 0.,
    font_n: 0 as *const libc::c_char as *mut libc::c_char,
    font_s: 0.,
    font_n_old: 0 as *const libc::c_char as *mut libc::c_char,
    linewidth: 0.,
    rgb: 0,
    dash: [0; 64],
};
static mut gsave: [gc; 8] = [gc {
    cx: 0.,
    cy: 0.,
    xscale: 0.,
    yscale: 0.,
    xoffs: 0.,
    yoffs: 0.,
    rotate: 0.,
    sin: 0.,
    cos: 0.,
    font_n: 0 as *const libc::c_char as *mut libc::c_char,
    font_s: 0.,
    font_n_old: 0 as *const libc::c_char as *mut libc::c_char,
    linewidth: 0.,
    rgb: 0,
    dash: [0; 64],
}; 8];
static mut nsave: libc::c_int = 0;
static mut x_rot: libc::c_float = 0.;
static mut y_rot: libc::c_float = 0.;
static mut g: libc::c_int = 0;
static mut boxend: libc::c_int = 0;
static mut defs: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut defssz: libc::c_int = 0;
static mut def_tb: [C2RustUnnamed_0; 91] = [
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"brace\" class=\"fill\" d=\"m-2.5 101\n\tc-4.5 -4.6 -7.5 -12.2 -4.4 -26.8\n\t3.5 -14.3 3.2 -21.7 -2.1 -24.2\n\t7.4 2.4 7.3 14.2 3.5 29.5\n\t-2.7 9.5 -1.5 16.2 3 21.5\n\tM-2.5 1c-4.5 4.6 -7.5 12.2 -4.4 26.8\n\tc3.5 14.3 3.2 21.7 -2.1 24.2\n\t7.4 -2.4 7.3 -14.2 3.5 -29.5\n\t-2.7 -9.5 -1.5 -16.2 3 -21.5\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"utclef\" class=\"fill\" d=\"m-50 -90\n\tc-72 -41 -72 -158 52 -188\n\t150 -10 220 188 90 256\n\t-114 52 -275 0 -293 -136\n\t-15 -181 93 -229 220 -334\n\t88 -87 79 -133 62 -210\n\t-51 33 -94 105 -89 186\n\t17 267 36 374 49 574\n\t6 96 -19 134 -77 135\n\t-80 1 -126 -93 -61 -133\n\t85 -41 133 101 31 105\n\t23 17 92 37 90 -92\n\t-10 -223 -39 -342 -50 -617\n\t0 -90 0 -162 96 -232\n\t56 72 63 230 22 289\n\t-74 106 -257 168 -255 316\n\t9 153 148 185 252 133\n\t86 -65 29 -192 -80 -176\n\t-71 12 -105 67 -59 124\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<use id=\"tclef\" transform=\"translate(0,6) scale(0.045)\"\n\txlink:href=\"#utclef\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 1 as libc::c_int as libc::c_char,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<use id=\"stclef\" transform=\"translate(0,5.4) scale(0.037)\"\n\txlink:href=\"#utclef\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 1 as libc::c_int as libc::c_char,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"ubclef\" class=\"fill\" d=\"m-200 -87\n\tc124 -35 222 -78 254 -236\n\t43 -228 -167 -246 -192 -103\n\t59 -80 157 22 92 78\n\t-62 47 -115 -22 -106 -88\n\t21 -141 270 -136 274 52\n\t-1 175 -106 264 -322 297\n\tm357 -250\n\tc0 -36 51 -34 51 0\n\t0 37 -51 36 -51 0\n\tm-2 -129\n\tc0 -36 51 -34 51 0\n\t0 38 -51 37 -51 0\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<use id=\"bclef\" transform=\"translate(0,18) scale(0.045)\"\n\txlink:href=\"#ubclef\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 4 as libc::c_int as libc::c_char,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<use id=\"sbclef\" transform=\"translate(0,14.5) scale(0.037)\"\n\txlink:href=\"#ubclef\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 4 as libc::c_int as libc::c_char,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"ucclef\" class=\"fill\" d=\"\n\tm-51 -264\n\tv262\n\th-13\n\tv-529\n\th13\n\tv256\n\tc25 -20 41 -36 63 -109\n\t14 31 13 51 56 70\n\t90 34 96 -266 -41 -185\n\t52 19 27 80 -11 77\n\t-90 -38 33 -176 139 -69\n\t72 79 1 241 -134 186\n\tl-16 39 16 38\n\tc135 -55 206 107 134 186\n\t-106 108 -229 -31 -139 -69\n\t38 -3 63 58 11 77\n\t137 81 131 -219 41 -185\n\t-43 19 -45 30 -56 64\n\t-22 -73 -38 -89 -63 -109\n\tm-99 -267\n\th57\n\tv529\n\th-57\n\tv-529\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<use id=\"cclef\" transform=\"translate(0,12) scale(0.045)\"\n\txlink:href=\"#ucclef\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 7 as libc::c_int as libc::c_char,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<use id=\"scclef\" transform=\"translate(0,9.5) scale(0.037)\"\n\txlink:href=\"#ucclef\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 7 as libc::c_int as libc::c_char,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"pclef\" d=\"m-2.7 9h5.4v-18h-5.4v18\" class=\"stroke\" stroke-width=\"1.4\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<ellipse id=\"hd\" rx=\"4.1\" ry=\"2.9\"\n\ttransform=\"rotate(-20)\" class=\"fill\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"Hd\" class=\"fill\" d=\"m3 -1.6\n\tc-1 -1.8 -7 1.4 -6 3.2\n\t1 1.8 7 -1.4 6 -3.2\n\tm0.5 -0.3\n\tc2 3.8 -5 7.6 -7 3.8\n\t-2 -3.8 5 -7.6 7 -3.8\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"HD\" class=\"fill\" d=\"m-2.7 -1.4\n\tc1.5 -2.8 6.9 0 5.3 2.7\n\t-1.5 2.8 -6.9 0 -5.3 -2.7\n\tm8.3 1.4\n\tc0 -1.5 -2.2 -3 -5.6 -3\n\t-3.4 0 -5.6 1.5 -5.6 3\n\t0 1.5 2.2 3 5.6 3\n\t3.4 0 5.6 -1.5 5.6 -3\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<g id=\"HDD\">\n\t<use xlink:href=\"#HD\"/>\n\t<path d=\"m-6 -4v8m12 0v-8\" class=\"stroke\"/>\n</g>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 13 as libc::c_int as libc::c_char,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<g id=\"breve\" class=\"stroke\">\n\t<path d=\"m-6 -2.7h12m0 5.4h-12\" stroke-width=\"2.5\"/>\n\t<path d=\"m-6 -5v10m12 0v-10\"/>\n</g>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<g id=\"longa\" class=\"stroke\">\n\t<path d=\"m-6 2.7h12m0 -5.4h-12\" stroke-width=\"2.5\"/>\n\t<path d=\"m-6 5v-10m12 0v16\"/>\n</g>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"ghd\" class=\"fill\" d=\"m2.2 -1.5\n\tc-1.32 -2.31 -5.94 0.33 -4.62 2.64\n\t1.32 2.31 5.94 -0.33 4.62 -2.64\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<rect id=\"r00\" class=\"fill\"\n\tx=\"-1.6\" y=\"-6\" width=\"3\" height=\"12\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<rect id=\"r0\" class=\"fill\"\n\tx=\"-1.6\" y=\"-6\" width=\"3\" height=\"6\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<rect id=\"r1\" class=\"fill\"\n\tx=\"-3.5\" y=\"-6\" width=\"7\" height=\"3\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<rect id=\"r2\" class=\"fill\"\n\tx=\"-3.5\" y=\"-3\" width=\"7\" height=\"3\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"r4\" class=\"fill\" d=\"m-1 -8.5\n\tl3.6 5.1 -2.1 5.2 2.2 4.3\n\tc-2.6 -2.3 -5.1 0 -2.4 2.6\n\t-4.8 -3 -1.5 -6.9 1.4 -4.1\n\tl-3.1 -4.5 1.9 -5.1 -1.5 -3.5\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"r8e\" class=\"fill\" d=\"m 0 0\n\tc-1.5 1.5 -2.4 2 -3.6 2\n\t2.4 -2.8 -2.8 -4 -2.8 -1.2\n\tc0 2.7 4.3 2.4 5.9 0.6\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<g id=\"r8\">\n\t<path d=\"m3.3 -4l-3.4 9.6\" class=\"stroke\"/>\n\t<use x=\"3.4\" y=\"-4\" xlink:href=\"#r8e\"/>\n</g>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 23 as libc::c_int as libc::c_char,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<g id=\"r16\">\n\t<path d=\"m3.3 -4l-4 15.6\" class=\"stroke\"/>\n\t<use x=\"3.4\" y=\"-4\" xlink:href=\"#r8e\"/>\n\t<use x=\"1.9\" y=\"2\" xlink:href=\"#r8e\"/>\n</g>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 23 as libc::c_int as libc::c_char,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<g id=\"r32\">\n\t<path d=\"m4.8 -10l-5.5 21.6\" class=\"stroke\"/>\n\t<use x=\"4.9\" y=\"-10\" xlink:href=\"#r8e\"/>\n\t<use x=\"3.4\" y=\"-4\" xlink:href=\"#r8e\"/>\n\t<use x=\"1.9\" y=\"2\" xlink:href=\"#r8e\"/>\n</g>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 23 as libc::c_int as libc::c_char,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<g id=\"r64\">\n\t<path d=\"m4.8 -10 l-7 27.6\" class=\"stroke\"/>\n\t<use x=\"4.9\" y=\"-10\" xlink:href=\"#r8e\"/>\n\t<use x=\"3.4\" y=\"-4\" xlink:href=\"#r8e\"/>\n\t<use x=\"1.9\" y=\"2\" xlink:href=\"#r8e\"/>\n\t<use x=\"0.4\" y=\"8\" xlink:href=\"#r8e\"/>\n</g>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 23 as libc::c_int as libc::c_char,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<g id=\"r128\">\n\t<path d=\"m5.8 -16 l-8.5 33.6\" class=\"stroke\"/>\n\t<use x=\"5.9\" y=\"-16\" xlink:href=\"#r8e\"/>\n\t<use x=\"4.4\" y=\"-10\" xlink:href=\"#r8e\"/>\n\t<use x=\"2.9\" y=\"-4\" xlink:href=\"#r8e\"/>\n\t<use x=\"1.4\" y=\"2\" xlink:href=\"#r8e\"/>\n\t<use x=\"0.1\" y=\"8\" xlink:href=\"#r8e\"/>\n</g>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 23 as libc::c_int as libc::c_char,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<g id=\"mrest\" class=\"stroke\">\n\t<path d=\"m-20 6v-12m40 0v12\"/>\n\t<path d=\"m-20 0h40\" stroke-width=\"5\"/>\n</g>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"usharp\" class=\"fill\" d=\"\n\tm136 -702\n\tv890\n\th32\n\tv-890\n\tm128 840\n\th32\n\tv-888\n\th-32\n\tm-232 286\n\tv116\n\tl338 -96\n\tv-116\n\tm-338 442\n\tv116\n\tl338 -98\n\tv-114\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"uflat\" class=\"fill\" d=\"\n\tm100 -746\n\th32\n\tv734\n\tl-32 4\n\tm32 -332\n\tc46 -72 152 -90 208 -20\n\t100 110 -120 326 -208 348\n\tm0 -28\n\tc54 0 200 -206 130 -290\n\t-50 -60 -130 -4 -130 34\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"unat\" class=\"fill\" d=\"\n\tm96 -750\n\th-32\n\tv716\n\tl32 -8\n\t182 -54\n\tv282\n\th32\n\tv-706\n\tl-34 10\n\t-180 50\n\tv-290\n\tm0 592\n\tv-190\n\tl182 -52\n\tv188\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"udblesharp\" class=\"fill\" d=\"\n\tm240 -282\n\tc40 -38 74 -68 158 -68\n\tv-96\n\th-96\n\tc0 84 -30 118 -68 156\n\t-40 -38 -70 -72 -70 -156\n\th-96\n\tv96\n\tc86 0 120 30 158 68\n\t-38 38 -72 68 -158 68\n\tv96\n\th96\n\tc0 -84 30 -118 70 -156\n\t38 38 68 72 68 156\n\th96\n\tv-96\n\tc-84 0 -118 -30 -158 -68\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"udbleflat\" class=\"fill\" d=\"\n\tm20 -746\n\th24\n\tv734\n\tl-24 4\n\tm24 -332\n\tc34 -72 114 -90 156 -20\n\t75 110 -98 326 -156 348\n\tm0 -28\n\tc40 0 150 -206 97 -290\n\t-37 -60 -97 -4 -97 34\n\tm226 -450\n\th24\n\tv734\n\tl-24 4\n\tm24 -332\n\tc34 -72 114 -90 156 -20\n\t75 110 -98 326 -156 348\n\tm0 -28\n\tc40 0 150 -206 97 -290\n\t-37 -60 -97 -4 -97 34\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<use id=\"sh0\" transform=\"translate(-4,5) scale(0.018)\"\n\txlink:href=\"#usharp\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 30 as libc::c_int as libc::c_char,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<use id=\"ft0\" transform=\"translate(-3.5,3.5) scale(0.018)\"\n\txlink:href=\"#uflat\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 31 as libc::c_int as libc::c_char,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<use id=\"nt0\" transform=\"translate(-3,5) scale(0.018)\"\n\txlink:href=\"#unat\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 32 as libc::c_int as libc::c_char,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<use id=\"dsh0\" transform=\"translate(-4,5) scale(0.018)\"\n\txlink:href=\"#udblesharp\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 33 as libc::c_int as libc::c_char,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<use id=\"dft0\" transform=\"translate(-4,3.5) scale(0.018)\"\n\txlink:href=\"#udbleflat\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 34 as libc::c_int as libc::c_char,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<g id=\"sh1\">\n\t<path d=\"M0 7.8v-15.4\" class=\"stroke\"/>\n\t<path class=\"fill\" d=\"M-1.8 2.7l3.6 -1.1v2.2l-3.6 1.1v-2.2z\n\t\tM-1.8 -3.7l3.6 -1.1v2.2l-3.6 1.1v-2.2\"/>\n</g>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<g id=\"sh513\">\n\t<path d=\"M-2.5 8.7v-15.4M0 7.8v-15.4M2.5 6.9v-15.4\" class=\"stroke\"/>\n\t<path class=\"fill\" d=\"M-3.7 3.1l7.4 -2.2v2.2l-7.4 2.2v-2.2z\n\t\tM-3.7 -3.2l7.4 -2.2v2.2l-7.4 2.2v-2.2\"/>\n</g>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<g id=\"ft1\" transform=\"scale(-1,1)\">\n\t<use xlink:href=\"#ft0\"/>\n</g>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 36 as libc::c_int as libc::c_char,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<g id=\"ft513\">\n\t<path class=\"fill\" d=\"M0.6 -2.7\n\t\tc-5.7 -3.1 -5.7 3.6 0 6.7c-3.9 -4 -4 -7.6 0 -5.8\n\t\tM1 -2.7c5.7 -3.1 5.7 3.6 0 6.7c3.9 -4 4 -7.6 0 -5.8\"/>\n\t<path d=\"M1.6 3.5v-13M0 3.5v-13\" class=\"stroke\" stroke-width=\".6\"/>\n</g>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<g id=\"pshhd\">\n\t<use xlink:href=\"#dsh0\"/>\n</g>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 38 as libc::c_int as libc::c_char,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<g id=\"pfthd\">\n\t<use xlink:href=\"#dsh0\"/>\n\t<circle r=\"4\" class=\"stroke\"/>\n</g>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 38 as libc::c_int as libc::c_char,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"csig\" class=\"fill\" d=\"\n\tm6 -5.3\n\tc0.9 0 2.3 0.7 2.4 2.2\n\t-1.2 -2 -3.6 0.1 -1.6 1.7\n\t2 1 3.8 -3.5 -0.8 -4.7\n\t-2 -0.4 -6.4 1.3 -5.8 7\n\t0.4 6.4 7.9 6.8 9.1 0.7\n\t-2.3 5.6 -6.7 5.1 -6.8 0\n\t-0.5 -4.4 0.7 -7.5 3.5 -6.9\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<g id=\"ctsig\">\n\t<use xlink:href=\"#csig\"/>\n\t<path d=\"m5 8v-16\" class=\"stroke\"/>\n</g>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 46 as libc::c_int as libc::c_char,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"pmsig\" class=\"stroke\" stroke-width=\".8\"\n\td=\"M0 -7a5 5 0 0 1 0 -10a5 5 0 0 1 0 10\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<g id=\"pMsig\">\n\t<use xlink:href=\"#pmsig\"/>\n\t<path class=\"fill\" d=\"M0 -10a2 2 0 0 1 0 -4a2 2 0 0 1 0 4\"/>\n</g>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 48 as libc::c_int as libc::c_char,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"imsig\" class=\"stroke\" stroke-width=\".8\"\n\td=\"M3 -8a5 5 0 1 1 0 -8\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<g id=\"iMsig\">\n\t<use xlink:href=\"#imsig\"/>\n\t<path class=\"fill\" d=\"M0 -10a2 2 0 0 1 0 -4a2 2 0 0 1 0 4\"/>\n</g>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 50 as libc::c_int as libc::c_char,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"hl\" class=\"stroke\" d=\"m-6 0h12\"/>\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"hl1\" class=\"stroke\" d=\"m-7 0h14\"/>\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"hl2\" class=\"stroke\" d=\"m-9 0h18\"/>\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"ghl\" class=\"stroke\" d=\"m-3.5 0h7\"/>\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<g id=\"rdots\" class=\"fill\">\n\t<circle cx=\"0\" cy=\"-9\" r=\"1.2\"/>\n\t<circle cx=\"0\" cy=\"-15\" r=\"1.2\"/>\n</g>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"srep\" class=\"fill\" d=\"M-1 6l11 -12h3l-11 12h-3\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"mrep\" class=\"fill\"\n    d=\"M-5 -4.5a1.5 1.5 0 0 1 0 3a1.5 1.5 0 0 1 0 -3\n\tM4.5 2a1.5 1.5 0 0 1 0 3a1.5 1.5 0 0 1 0 -3\n\tM-7 6l11 -12h3l-11 12h-3\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<g id=\"mrep2\" class=\"fill\">\n\t<path d=\"M-5.5 -7.5a1.5 1.5 0 0 1 0 3a1.5 1.5 0 0 1 0 -3\n\t\tM5 4.5a1.5 1.5 0 0 1 0 3a1.5 1.5 0 0 1 0 -3\"/>\n\t<path d=\"M-7 8l14 -10m-14 4l14 -10\" class=\"stroke\" stroke-width=\"1.8\"/>\n</g>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<g id=\"accent\" class=\"stroke\" stroke-width=\"1.2\">\n\t<path d=\"m-4 0l8 -2l-8 -2\"/>\n</g>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"umrd\" class=\"fill\" d=\"m0 -4\n\tl2.2 -2.2 2.1 2.9 0.7 -0.7 0.2 0.2\n\t-2.2 2.2 -2.1 -2.9 -0.7 0.7\n\t-2.2 2.2 -2.1 -2.9 -0.7 0.7 -0.2 -0.2\n\t2.2 -2.2 2.1 2.9 0.7 -0.7\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<g id=\"lmrd\">\n\t<use xlink:href=\"#umrd\"/>\n\t<line x1=\"0\" y1=\"0\" x2=\"0\" y2=\"-8\" class=\"stroke\" stroke-width=\".6\"/>\n</g>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 61 as libc::c_int as libc::c_char,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"grm\" class=\"fill\" d=\"\n\tm-5 -2.5\n\tc5 -8.5 5.5 4.5 10 -2\n\t-5 8.5 -5.5 -4.5 -10 2\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<circle id=\"stc\" class=\"fill\" cx=\"0\" cy=\"-3\" r=\"1.2\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"sld\" class=\"fill\" d=\"\n\tm-7.2 4.8\n\tc1.8 0.7 4.5 -0.2 7.2 -4.8\n\t-2.1 5 -5.4 6.8 -7.6 6\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"emb\" d=\"m-2.5 -3h5\" class=\"stroke\" stroke-width=\"1.2\" stroke-linecap=\"round\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<g id=\"hld\" class=\"fill\">\n\t<circle cx=\"0\" cy=\"-3\" r=\"1.3\"/>\n\t<path d=\"m-7.5 -1.5\n\t\tc0 -11.5 15 -11.5 15 0\n\t\th-0.25\n\t\tc-1.25 -9 -13.25 -9 -14.5 0\"/>\n</g>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"cpu\" class=\"fill\" d=\"\n\tm-6 0\n\tc0.4 -7.3 11.3 -7.3 11.7 0\n\tc-1.3 -6 -10.4 -6 -11.7 0\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"upb\" class=\"stroke\" d=\"\n\tm-2.6 -9.4\n\tl2.6 8.8\n\t2.6 -8.8\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<g id=\"dnb\">\n\t<path d=\"M-3.2 -2v-7.2m6.4 0v7.2\" class=\"stroke\"/>\n\t<path d=\"M-3.2 -6.8v-2.4l6.4 0v2.4\" class=\"fill\"/>\n</g>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<g id=\"sgno\">\n    <path class=\"fill\" d=\"m0 -3\n\tc1.5 1.7 6.4 -0.3 3 -3.7\n\t-10.4 -7.8 -8 -10.6 -6.5 -11.9\n\t4 -1.9 5.9 1.7 4.2 2.6\n\t-1.3 0.7 -2.9 -1.3 -0.7 -2\n\t-1.5 -1.7 -6.4 0.3 -3 3.7\n\t10.4 7.8 8 10.6 6.5 11.9\n\t-4 1.9 -5.9 -1.7 -4.2 -2.6\n\t1.3 -0.7 2.9 1.3 0.7 2\"/>\n    <line x1=\"-6\" y1=\"-4.2\" x2=\"6.6\" y2=\"-16.8\" class=\"stroke\"/>\n    <circle cx=\"-6\" cy=\"-10\" r=\"1.2\"/>\n    <circle cx=\"6\" cy=\"-11\" r=\"1.2\"/>\n</g>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<g id=\"coda\" class=\"stroke\">\n\t<path d=\"m0 -2v-20m-10 10h20\"/>\n\t<circle cx=\"0\" cy=\"-12\" r=\"6\" stroke-width=\"1.7\"/>\n</g>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"dplus\" class=\"stroke\" stroke-width=\"1.7\"\n\td=\"m0 -0.5v-6m-3 3h6\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"lphr\" class=\"stroke\" stroke-width=\"1.2\"\n\td=\"m0 0v18\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"mphr\" class=\"stroke\" stroke-width=\"1.2\"\n\td=\"m0 0v12\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"sphr\" class=\"stroke\" stroke-width=\"1.2\"\n\td=\"m0 0v6\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<circle id=\"opend\" class=\"stroke\"\n\tcx=\"0\" cy=\"-3\" r=\"2.5\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"snap\" class=\"stroke\"\n\td=\"M-3 -6\n\t\tc0 -5 6 -5 6 0\n\t\tc0 5 -6 5 -6 0\n\t\tM0 -5v6\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"thumb\" class=\"stroke\"\n\td=\"M-2.5 -7\n\t\tc0 -6 5 -6 5 0\n\t\tc0 6 -5 6 -5 0\n\t\tM-2.5 -9v4\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"turn\" class=\"fill\" d=\"\n\tm5.2 -8\n\tc1.4 0.5 0.9 4.8 -2.2 2.8\n\tl-4.8 -3.5\n\tc-3 -2 -5.8 1.8 -3.6 4.4\n\t1 1.1 2 0.8 2.1 -0.1\n\t0.1 -0.9 -0.7 -1.2 -1.9 -0.6\n\t-1.4 -0.5 -0.9 -4.8 2.2 -2.8\n\tl4.8 3.5\n\tc3 2 5.8 -1.8 3.6 -4.4\n\t-1 -1.1 -2 -0.8 -2.1 0.1\n\t-0.1 0.9 0.7 1.2 1.9 0.6\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<g id=\"turnx\">\n\t<use xlink:href=\"#turn\"/>\n\t<path d=\"M0 -1.5v-9\" class=\"stroke\"/>\n</g>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 80 as libc::c_int as libc::c_char,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"wedge\" class=\"fill\" d=\"M0 -1l-1.5 -5h3l-1.5 5\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"ltr\" class=\"fill\"\n    d=\"m0 -0.4c2 -1.5 3.4 -1.9 3.9 0.4\n\tc0.2 0.8 0.7 0.7 2.1 -0.4\n\tv0.8c-2 1.5 -3.4 1.9 -3.9 -0.4\n\tc-0.2 -0.8 -0.7 -0.7 -2.1 0.4z\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<g id=\"custos\">\n\t<path d=\"M-4 0l2 2.5l2 -2.5l2 2.5l2 -2.5\n\t\tl-2 -2.5l-2 2.5l-2 -2.5l-2 2.5\" class=\"fill\"/>\n\t<path d=\"M3.5 0l5 -7\" class=\"stroke\"/>\n</g>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<circle id=\"showerror\" r=\"30\" stroke=\"#ffc0c0\" stroke-width=\"2.5\" fill=\"none\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<text id=\"sfz\" style=\"font:italic 14px serif\"\n\tx=\"-5\" y=\"-7\">s<tspan\n\tfont-size=\"16\" font-weight=\"bold\">f</tspan>z</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<text id=\"trl\" style=\"font:italic bold 16px serif\"\n\tx=\"-2\" y=\"-4\">tr</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<path id=\"marcato\" d=\"m-3 0l3 -7l3 7l-1.5 0l-1.8 -4.2l-1.7 4.2\"/>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<text id=\"ped\" font-family=\"serif\" font-size=\"16\" font-style=\"italic\"\n\tx=\"-10\" y=\"-4\">Ped</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            def: b"<text id=\"pedoff\" font-family=\"serif\" font-size=\"16\" font-style=\"italic\"\n\tx=\"-5\" y=\"-4\">*</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            use_0: 0,
            defined: 0,
        };
        init
    },
];
static mut font_gl: [C2RustUnnamed_1; 45] = [
    {
        let mut init = C2RustUnnamed_1 {
            index: 0 as libc::c_int,
            def: b"<text id=\"brace\" class=\"music\" x=\"-3\" y=\"0\"\n\ttransform=\"scale(3,-4.2)\">&#xe000;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 71 as libc::c_int,
            def: b"<text id=\"sgno\" class=\"music\" x=\"-6\" y=\"-4\">&#xe047;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 72 as libc::c_int,
            def: b"<text id=\"coda\" class=\"music\" x=\"-12\" y=\"-6\">&#xe048;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 2 as libc::c_int,
            def: b"<text id=\"tclef\" class=\"music\" x=\"-8\" y=\"0\">&#xe050;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 8 as libc::c_int,
            def: b"<text id=\"cclef\" class=\"music\" x=\"-7\" y=\"0\">&#xe05c;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 5 as libc::c_int,
            def: b"<text id=\"bclef\" class=\"music\" x=\"-7\" y=\"0\">&#xe062;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 10 as libc::c_int,
            def: b"<text id=\"pclef\" class=\"music\" x=\"-6\" y=\"0\">&#xe069;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 3 as libc::c_int,
            def: b"<text id=\"stclef\" class=\"music\" x=\"-8\" y=\"0\">&#xe07a;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 9 as libc::c_int,
            def: b"<text id=\"scclef\" class=\"music\" x=\"-8\" y=\"0\">&#xe07b;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 6 as libc::c_int,
            def: b"<text id=\"sbclef\" class=\"music\" x=\"-7\" y=\"0\">&#xe07c;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 46 as libc::c_int,
            def: b"<text id=\"csig\" class=\"music\" x=\"0\" y=\"0\">&#xe08a;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 47 as libc::c_int,
            def: b"<text id=\"ctsig\" class=\"music\" x=\"0\" y=\"0\">&#xe08b;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 14 as libc::c_int,
            def: b"<text id=\"HDD\" class=\"music\" x=\"-7\" y=\"0\">&#xe0a0;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 15 as libc::c_int,
            def: b"<text id=\"breve\" class=\"music\" x=\"-6\" y=\"0\">&#xe0a1;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 13 as libc::c_int,
            def: b"<text id=\"HD\" class=\"music\" x=\"-5.2\" y=\"0\">&#xe0a2;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 12 as libc::c_int,
            def: b"<text id=\"Hd\" class=\"music\" x=\"-3.8\" y=\"0\">&#xe0a3;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 11 as libc::c_int,
            def: b"<text id=\"hd\" class=\"music\" x=\"-3.7\" y=\"0\">&#xe0a4;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 36 as libc::c_int,
            def: b"<text id=\"ft0\" class=\"music\" x=\"-3\" y=\"0\">&#xe260;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 37 as libc::c_int,
            def: b"<text id=\"nt0\" class=\"music\" x=\"-2\" y=\"0\">&#xe261;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 35 as libc::c_int,
            def: b"<text id=\"sh0\" class=\"music\" x=\"-3\" y=\"0\">&#xe262;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 38 as libc::c_int,
            def: b"<text id=\"dsh0\" class=\"music\" x=\"-3\" y=\"0\">&#xe263;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 44 as libc::c_int,
            def: b"<text id=\"pshhd\" class=\"music\" x=\"-3\" y=\"0\">&#xe263;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 39 as libc::c_int,
            def: b"<text id=\"dft0\" class=\"music\" x=\"-3\" y=\"0\">&#xe264;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 60 as libc::c_int,
            def: b"<text id=\"accent\" class=\"music\" x=\"-3\" y=\"0\">&#xe4a0;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 88 as libc::c_int,
            def: b"<text id=\"marcato\" class=\"music\" x=\"-3\" y=\"0\">&#xe4ac;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 67 as libc::c_int,
            def: b"<text id=\"hld\" class=\"music\" x=\"-7\" y=\"0\">&#xe4c0;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 18 as libc::c_int,
            def: b"<text id=\"r00\" class=\"music\" x=\"-1.5\" y=\"0\">&#xe4e1;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 19 as libc::c_int,
            def: b"<text id=\"r0\" class=\"music\" x=\"-1.5\" y=\"0\">&#xe4e2;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 20 as libc::c_int,
            def: b"<text id=\"r1\" class=\"music\" x=\"-3.5\" y=\"-6\">&#xe4e3;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 21 as libc::c_int,
            def: b"<text id=\"r2\" class=\"music\" x=\"-3.2\" y=\"0\">&#xe4e4;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 22 as libc::c_int,
            def: b"<text id=\"r4\" class=\"music\" x=\"-3\" y=\"0\">&#xe4e5;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 24 as libc::c_int,
            def: b"<text id=\"r8\" class=\"music\" x=\"-3\" y=\"0\">&#xe4e6;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 25 as libc::c_int,
            def: b"<text id=\"r16\" class=\"music\" x=\"-4\" y=\"0\">&#xe4e7;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 26 as libc::c_int,
            def: b"<text id=\"r32\" class=\"music\" x=\"-4\" y=\"0\">&#xe4e8;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 27 as libc::c_int,
            def: b"<text id=\"r64\" class=\"music\" x=\"-4\" y=\"0\">&#xe4e9;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 28 as libc::c_int,
            def: b"<text id=\"r128\" class=\"music\" x=\"-4\" y=\"0\">&#xe4ea;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 29 as libc::c_int,
            def: b"<text id=\"mrest\" class=\"music\" x=\"-10\" y=\"0\">&#xe4ee;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 58 as libc::c_int,
            def: b"<text id=\"mrep\" class=\"music\" x=\"-6\" y=\"0\">&#xe500;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 59 as libc::c_int,
            def: b"<text id=\"mrep2\" class=\"music\" x=\"-9\" y=\"0\">&#xe501;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 80 as libc::c_int,
            def: b"<text id=\"turn\" class=\"music\" x=\"-4\" y=\"0\">&#xe567;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 61 as libc::c_int,
            def: b"<text id=\"umrd\" class=\"music\" x=\"-7\" y=\"-2\">&#xe56c;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 62 as libc::c_int,
            def: b"<text id=\"lmrd\" class=\"music\" x=\"-7\" y=\"-2\">&#xe56d;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 89 as libc::c_int,
            def: b"<text id=\"ped\" class=\"music\" x=\"-10\" y=\"0\">&#xe650;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 90 as libc::c_int,
            def: b"<text id=\"pedoff\" class=\"music\" x=\"-6\" y=\"0\">&#xe655;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_1 {
            index: 16 as libc::c_int,
            def: b"<text id=\"longa\" class=\"music\" x=\"-6\" y=\"0\">&#xe95c;</text>\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn svg_font_switch() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[C2RustUnnamed_1; 45]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong)
    {
        j = font_gl[i as usize].index;
        def_tb[j as usize].def = font_gl[i as usize].def;
        def_tb[j as usize].use_0 = 0 as libc::c_int as libc::c_char;
        i += 1;
        i;
    }
}
unsafe extern "C" fn elts_link(mut e: *mut elt_s) {
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < 2048 as libc::c_int - 1 as libc::c_int {
        let ref mut fresh0 = (*e.offset(i as isize)).next;
        *fresh0 = &mut *e.offset((i + 1 as libc::c_int) as isize) as *mut elt_s;
        if (*e.offset(i as isize)).type_0 as libc::c_int == STR as libc::c_int {
            free((*e.offset(i as isize)).u.s as *mut libc::c_void);
        }
        (*e.offset(i as isize)).type_0 = VAL as libc::c_int as libc::c_char;
        i += 1;
        i;
    }
    let ref mut fresh1 = (*e.offset((2048 as libc::c_int - 1 as libc::c_int) as isize))
        .next;
    *fresh1 = 0 as *mut elt_s;
}
unsafe extern "C" fn elts_reset() {
    let mut e: *mut elt_s = 0 as *mut elt_s;
    if elts.is_null() {
        elts = calloc(
            ::core::mem::size_of::<elt_s>() as libc::c_ulong,
            2048 as libc::c_int as libc::c_ulong,
        ) as *mut elt_s;
    }
    elts_link(elts);
    free_elt = elts.offset(1 as libc::c_int as isize);
    e = elts;
    while !((*e).u.e).is_null() {
        elts_link((*e).u.e);
        let ref mut fresh2 = (*e
            .offset((2048 as libc::c_int - 1 as libc::c_int) as isize))
            .next;
        *fresh2 = (*e).u.e;
        e = (*e).u.e;
    }
}
unsafe extern "C" fn elt_new() -> *mut elt_s {
    let mut e: *mut elt_s = 0 as *mut elt_s;
    e = free_elt;
    if e.is_null() {
        e = calloc(
            ::core::mem::size_of::<elt_s>() as libc::c_ulong,
            2048 as libc::c_int as libc::c_ulong,
        ) as *mut elt_s;
        if e.is_null() {
            fprintf(
                __stderrp,
                b"svg: elt_new out of memory\n\0" as *const u8 as *const libc::c_char,
            );
            ps_error = 1 as libc::c_int;
            return e;
        }
        elts_link(e);
        (*e).u.e = elts;
        elts = e;
        e = e.offset(1);
        e;
    }
    free_elt = (*e).next;
    (*e).next = 0 as *mut elt_s;
    (*e).type_0 = VAL as libc::c_int as libc::c_char;
    return e;
}
unsafe extern "C" fn elt_free(mut e: *mut elt_s) {
    let mut e2: *mut elt_s = 0 as *mut elt_s;
    (*e).next = free_elt;
    free_elt = e;
    match (*e).type_0 as libc::c_int {
        1 => {
            free((*e).u.s as *mut libc::c_void);
            (*e).type_0 = VAL as libc::c_int as libc::c_char;
            (*e).u.v = 0 as libc::c_int as libc::c_float;
        }
        2 | 3 => {
            e2 = (*e).u.e;
            (*e).type_0 = VAL as libc::c_int as libc::c_char;
            (*e).u.v = 0 as libc::c_int as libc::c_float;
            while !e2.is_null() {
                e = (*e2).next;
                elt_free(e2);
                e2 = e;
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn elt_dup(mut e: *mut elt_s) -> *mut elt_s {
    let mut e2: *mut elt_s = 0 as *mut elt_s;
    let mut e3: *mut elt_s = 0 as *mut elt_s;
    let mut e4: *mut elt_s = 0 as *mut elt_s;
    e2 = elt_new();
    if e2.is_null() {
        return e2;
    }
    (*e2).type_0 = (*e).type_0;
    match (*e).type_0 as libc::c_int {
        0 => {
            (*e2).u.v = (*e).u.v;
        }
        1 => {
            (*e2).u.s = strdup((*e).u.s);
        }
        2 | 3 => {
            e = (*e).u.e;
            if e.is_null() {
                (*e2).u.e = 0 as *mut elt_s;
            } else {
                (*e2).u.e = elt_dup(e);
                e3 = (*e2).u.e;
                if !e3.is_null() {
                    loop {
                        e = (*e).next;
                        if e.is_null() {
                            break;
                        }
                        e4 = elt_dup(e);
                        if e4.is_null() {
                            break;
                        }
                        (*e3).next = e4;
                        e3 = e4;
                    }
                    (*e3).next = 0 as *mut elt_s;
                }
            }
        }
        _ => {}
    }
    return e2;
}
unsafe extern "C" fn elt_dump(mut e: *mut elt_s) {
    let mut type_0: libc::c_int = 0;
    type_0 = (*e).type_0 as libc::c_int;
    match type_0 {
        0 => {
            fprintf(
                __stderrp,
                b" %.2f\0" as *const u8 as *const libc::c_char,
                (*e).u.v as libc::c_double,
            );
        }
        1 => {
            fprintf(__stderrp, b" %s\0" as *const u8 as *const libc::c_char, (*e).u.s);
            if *((*e).u.s).offset(0 as libc::c_int as isize) as libc::c_int == '(' as i32
            {
                fprintf(__stderrp, b")\0" as *const u8 as *const libc::c_char);
            }
        }
        2 | 3 => {
            fprintf(
                __stderrp,
                if type_0 == SEQ as libc::c_int {
                    b" {\0" as *const u8 as *const libc::c_char
                } else {
                    b" [\0" as *const u8 as *const libc::c_char
                },
            );
            e = (*e).u.e;
            while !e.is_null() {
                elt_dump(e);
                e = (*e).next;
            }
            fprintf(
                __stderrp,
                if type_0 == SEQ as libc::c_int {
                    b" }\0" as *const u8 as *const libc::c_char
                } else {
                    b" ]\0" as *const u8 as *const libc::c_char
                },
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn elt_lst_dump(mut e: *mut elt_s) {
    loop {
        elt_dump(e);
        e = (*e).next;
        if e.is_null() {
            break;
        }
    };
}
unsafe extern "C" fn ps_sym_lookup(mut name: *mut libc::c_char) -> *mut ps_sym_s {
    let mut ps: *mut ps_sym_s = 0 as *mut ps_sym_s;
    if n_sym == 0 as libc::c_int {
        return 0 as *mut ps_sym_s;
    }
    ps = &mut *ps_sym.as_mut_ptr().offset(n_sym as isize) as *mut ps_sym_s;
    loop {
        ps = ps.offset(-1);
        ps;
        if strcmp((*ps).n, name) == 0 as libc::c_int {
            break;
        }
        if ps == ps_sym.as_mut_ptr() {
            return 0 as *mut ps_sym_s;
        }
    }
    return ps;
}
unsafe extern "C" fn ps_sym_def(
    mut name: *mut libc::c_char,
    mut e: *mut elt_s,
) -> *mut ps_sym_s {
    let mut ps: *mut ps_sym_s = 0 as *mut ps_sym_s;
    ps = ps_sym_lookup(name);
    if !ps.is_null() {
        elt_free((*ps).e);
    } else {
        if n_sym >= 512 as libc::c_int {
            fprintf(
                __stderrp,
                b"svg: Too many PS symbols\n\0" as *const u8 as *const libc::c_char,
            );
            ps_error = 1 as libc::c_int;
            return 0 as *mut ps_sym_s;
        }
        let fresh3 = n_sym;
        n_sym = n_sym + 1;
        ps = &mut *ps_sym.as_mut_ptr().offset(fresh3 as isize) as *mut ps_sym_s;
        (*ps).n = strdup(name);
    }
    (*ps).e = e;
    (*ps).exec = 0 as libc::c_int;
    return ps;
}
unsafe extern "C" fn push(mut e: *mut elt_s) {
    (*e).next = stack;
    stack = e;
}
unsafe extern "C" fn stack_dump() {
    fprintf(__stderrp, b"stack:\0" as *const u8 as *const libc::c_char);
    if !stack.is_null() {
        elt_lst_dump(stack);
    } else {
        fprintf(__stderrp, b"(empty)\0" as *const u8 as *const libc::c_char);
    }
    fprintf(__stderrp, b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn pop(mut type_0: libc::c_int) -> *mut elt_s {
    let mut e: *mut elt_s = 0 as *mut elt_s;
    e = stack;
    if e.is_null() {
        fprintf(
            __stderrp,
            b"svg pop: Stack empty\n\0" as *const u8 as *const libc::c_char,
        );
        ps_error = 1 as libc::c_int;
        return 0 as *mut elt_s;
    }
    if (*e).type_0 as libc::c_int != type_0 {
        fprintf(
            __stderrp,
            b"svg pop: Bad element type %d != %d\n\0" as *const u8
                as *const libc::c_char,
            (*e).type_0 as libc::c_int,
            type_0,
        );
        stack_dump();
        ps_error = 1 as libc::c_int;
        return 0 as *mut elt_s;
    }
    stack = (*e).next;
    return e;
}
unsafe extern "C" fn pop_free_val() -> libc::c_float {
    let mut e: *mut elt_s = 0 as *mut elt_s;
    e = pop(VAL as libc::c_int);
    if e.is_null() {
        return 0 as libc::c_int as libc::c_float;
    }
    (*e).next = free_elt;
    free_elt = e;
    return (*e).u.v;
}
unsafe extern "C" fn pop_free_str() -> *mut libc::c_char {
    let mut e: *mut elt_s = 0 as *mut elt_s;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    e = pop(STR as libc::c_int);
    if e.is_null() {
        return 0 as *mut libc::c_char;
    }
    s = (*e).u.s;
    (*e).type_0 = VAL as libc::c_int as libc::c_char;
    (*e).next = free_elt;
    free_elt = e;
    return s;
}
unsafe extern "C" fn cond(mut type_0: libc::c_int) {
    let mut v: libc::c_float = 0.;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s2: *mut libc::c_char = 0 as *mut libc::c_char;
    if stack.is_null() || ((*stack).next).is_null() {
        fprintf(
            __stderrp,
            b"svg: Stack underflow in condition\n\0" as *const u8 as *const libc::c_char,
        );
        ps_error = 1 as libc::c_int;
        return;
    }
    if (*stack).type_0 as libc::c_int == STR as libc::c_int
        && (*(*stack).next).type_0 as libc::c_int == STR as libc::c_int
    {
        s = pop_free_str();
        s2 = (*stack).u.s;
        match type_0 {
            0 => {
                (*stack)
                    .u
                    .v = (strcmp(s2, s) == 0 as libc::c_int) as libc::c_int
                    as libc::c_float;
            }
            1 => {
                (*stack)
                    .u
                    .v = (strcmp(s2, s) != 0 as libc::c_int) as libc::c_int
                    as libc::c_float;
            }
            _ => {
                fprintf(
                    __stderrp,
                    b"svg: String condition not treated\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        free(s as *mut libc::c_void);
        free(s2 as *mut libc::c_void);
        (*stack).type_0 = VAL as libc::c_int as libc::c_char;
        return;
    }
    if (*stack).type_0 as libc::c_int == STR as libc::c_int {
        s = (*stack).u.s;
        (*stack).u.v = *s.offset(1 as libc::c_int as isize) as libc::c_float;
        free(s as *mut libc::c_void);
        (*stack).type_0 = VAL as libc::c_int as libc::c_char;
    }
    if (*(*stack).next).type_0 as libc::c_int == STR as libc::c_int {
        s = (*(*stack).next).u.s;
        (*(*stack).next).u.v = *s.offset(1 as libc::c_int as isize) as libc::c_float;
        free(s as *mut libc::c_void);
        (*(*stack).next).type_0 = VAL as libc::c_int as libc::c_char;
    }
    v = pop_free_val();
    if (*stack).type_0 as libc::c_int != VAL as libc::c_int {
        fprintf(
            __stderrp,
            b"svg: Bad type for condition\n\0" as *const u8 as *const libc::c_char,
        );
        ps_error = 1 as libc::c_int;
        return;
    }
    match type_0 {
        0 => {
            (*stack).u.v = ((*stack).u.v == v) as libc::c_int as libc::c_float;
        }
        1 => {
            (*stack).u.v = ((*stack).u.v != v) as libc::c_int as libc::c_float;
        }
        2 => {
            (*stack).u.v = ((*stack).u.v > v) as libc::c_int as libc::c_float;
        }
        3 => {
            (*stack).u.v = ((*stack).u.v >= v) as libc::c_int as libc::c_float;
        }
        4 => {
            (*stack).u.v = ((*stack).u.v < v) as libc::c_int as libc::c_float;
        }
        5 => {
            (*stack).u.v = ((*stack).u.v <= v) as libc::c_int as libc::c_float;
        }
        _ => {}
    };
}
unsafe extern "C" fn xml_str_out(mut p: *mut libc::c_char) {
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    q = p;
    while *p as libc::c_int != '\0' as i32 {
        let fresh4 = p;
        p = p.offset(1);
        match *fresh4 as libc::c_int {
            60 => {
                r = b"&lt;\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            62 => {
                r = b"&gt;\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            39 => {
                r = b"&apos;\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            34 => {
                r = b"&quot;\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            38 => {
                if *p as libc::c_int == '#' as i32
                    || strncmp(
                        p,
                        b"lt;\0" as *const u8 as *const libc::c_char,
                        3 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    || strncmp(
                        p,
                        b"gt;\0" as *const u8 as *const libc::c_char,
                        3 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    || strncmp(
                        p,
                        b"amp;\0" as *const u8 as *const libc::c_char,
                        4 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    || strncmp(
                        p,
                        b"apos;\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    || strncmp(
                        p,
                        b"quot;\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                {
                    continue;
                }
                r = b"&amp;\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            _ => {
                continue;
            }
        }
        if p.offset(-(1 as libc::c_int as isize)) != q {
            fwrite(
                q as *const libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                p.offset(-(1 as libc::c_int as isize)).offset_from(q) as libc::c_long
                    as libc::c_ulong,
                fout,
            );
        }
        q = p;
        fputs(r, fout);
    }
    if p != q {
        fputs(q, fout);
    }
}
unsafe extern "C" fn gen_info() {
    let mut i: libc::c_uint = 0;
    let mut ltime: time_t = 0;
    time(&mut ltime);
    strftime(
        tex_buf.as_mut_ptr(),
        512 as libc::c_int as size_t,
        b"%b %e, %Y %H:%M\0" as *const u8 as *const libc::c_char,
        localtime(&mut ltime),
    );
    fprintf(
        fout,
        b"<!-- CreationDate: %s -->\n<!-- CommandLine:\0" as *const u8
            as *const libc::c_char,
        tex_buf.as_mut_ptr(),
    );
    i = 1 as libc::c_int as libc::c_uint;
    while i < s_argc as libc::c_uint {
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut space: libc::c_int = 0;
        p = *s_argv.offset(i as isize);
        space = (!(strchr(p, ' ' as i32)).is_null()
            || !(strchr(p, '\n' as i32)).is_null()) as libc::c_int;
        fputc(' ' as i32, fout);
        if space != 0 {
            fputc('\'' as i32, fout);
        }
        if *p as libc::c_int == '-' as i32
            && *p.offset(1 as libc::c_int as isize) as libc::c_int == '-' as i32
        {
            fputs(b"-\\\0" as *const u8 as *const libc::c_char, fout);
            p = p.offset(1);
            p;
        }
        fputs(p, fout);
        if space != 0 {
            fputc('\'' as i32, fout);
        }
        i = i.wrapping_add(1);
        i;
    }
    fputs(b" -->\n\0" as *const u8 as *const libc::c_char, fout);
}
unsafe extern "C" fn define_head(mut w: libc::c_float, mut h: libc::c_float) {
    static mut svg_head1: [libc::c_char; 282] = unsafe {
        *::core::mem::transmute::<
            &[u8; 282],
            &[libc::c_char; 282],
        >(
            b"<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\"\n\txmlns:xlink=\"http://www.w3.org/1999/xlink\"\n\tcolor=\"black\"\n\twidth=\"%.2fpx\" height=\"%.2fpx\">\n<style type=\"text/css\">\n.fill {fill: currentColor}\n.stroke {stroke: currentColor; fill: none}\ntext{white-space:pre; fill:currentColor}\n\0",
        )
    };
    static mut svg_font_style: [libc::c_char; 44] = unsafe {
        *::core::mem::transmute::<
            &[u8; 44],
            &[libc::c_char; 44],
        >(b".music {font:24px %s;\n\tfill: currentColor}\n\0")
    };
    static mut svg_font_style_url: [libc::c_char; 89] = unsafe {
        *::core::mem::transmute::<
            &[u8; 89],
            &[libc::c_char; 89],
        >(
            b"@font-face {\n\tfont-family:music;\n\tsrc:%s}\n.music {font:24px music;\n\tfill: currentColor}\n\0",
        )
    };
    static mut svg_head2: [libc::c_char; 17] = unsafe {
        *::core::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"</style>\n<title>\0")
    };
    fprintf(fout, svg_head1.as_ptr(), w as libc::c_double, h as libc::c_double);
    if !(cfmt.musicfont).is_null() {
        if !(strchr(cfmt.musicfont, '(' as i32)).is_null() {
            fprintf(fout, svg_font_style_url.as_ptr(), cfmt.musicfont);
        } else {
            fprintf(fout, svg_font_style.as_ptr(), cfmt.musicfont);
        }
    }
    fputs(svg_head2.as_ptr(), fout);
}
#[no_mangle]
pub unsafe extern "C" fn define_svg_symbols(
    mut title: *mut libc::c_char,
    mut num: libc::c_int,
    mut w: libc::c_float,
    mut h: libc::c_float,
) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_uint = 0;
    static mut svg_head3: [libc::c_char; 16] = unsafe {
        *::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b" %s %d</title>\n\0")
    };
    if svg == 2 as libc::c_int {
        if file_initialized <= 0 as libc::c_int {
            s = strrchr(in_fname, '/' as i32);
            if s.is_null() {
                s = in_fname;
            } else {
                s = s.offset(1);
                s;
            }
            fputs(
                b"<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.1//EN\"\n\"http://www.w3.org/TR/xhtml1/DTD/xhtml1.dtd\">\n<html xmlns=\"http://www.w3.org/1999/xhtml\">\n<head>\n<meta http-equiv=\"Content-Type\" content=\"text/html; charset=UTF-8\"/>\n<meta name=\"generator\" content=\"abcm2ps-8.14.14\"/>\n\0"
                    as *const u8 as *const libc::c_char,
                fout,
            );
            gen_info();
            fprintf(
                fout,
                b"<style type=\"text/css\">\n\tbody {margin:0; padding:0; border:0;\0"
                    as *const u8 as *const libc::c_char,
            );
            if !(cfmt.bgcolor).is_null()
                && *(cfmt.bgcolor).offset(0 as libc::c_int as isize) as libc::c_int
                    != '\0' as i32
            {
                fprintf(
                    fout,
                    b" background-color:%s\0" as *const u8 as *const libc::c_char,
                    cfmt.bgcolor,
                );
            }
            fprintf(
                fout,
                b"}\n\t@page {margin: 0}\n\tsvg {display: block}\n</style>\n<title>%s</title>\n</head>\n<body>\n\0"
                    as *const u8 as *const libc::c_char,
                s,
            );
        } else {
            fputs(b"<br/>\n\0" as *const u8 as *const libc::c_char, fout);
        }
        define_head(w, h);
        xml_str_out(title);
        fprintf(
            fout,
            svg_head3.as_ptr(),
            b"page\0" as *const u8 as *const libc::c_char,
            num,
        );
    } else {
        if epsf != 3 as libc::c_int {
            if fout != __stdoutp {
                fputs(
                    b"<?xml version=\"1.0\" standalone=\"no\"?>\n<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\"\n\t\"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\n\0"
                        as *const u8 as *const libc::c_char,
                    fout,
                );
            }
        }
        define_head(w, h);
        xml_str_out(title);
        fprintf(
            fout,
            svg_head3.as_ptr(),
            if epsf != 0 {
                b"tune\0" as *const u8 as *const libc::c_char
            } else {
                b"page\0" as *const u8 as *const libc::c_char
            },
            num,
        );
        fputs(
            b"<!-- Creator: abcm2ps-8.14.14 -->\n\0" as *const u8 as *const libc::c_char,
            fout,
        );
        gen_info();
        if !(cfmt.bgcolor).is_null()
            && *(cfmt.bgcolor).offset(0 as libc::c_int as isize) as libc::c_int
                != '\0' as i32
        {
            fprintf(
                fout,
                b"<rect width=\"100%%\" height=\"100%%\" fill=\"%s\"/>\n\0" as *const u8
                    as *const libc::c_char,
                cfmt.bgcolor,
            );
        }
    }
    memset(
        &mut gcur as *mut gc as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<gc>() as libc::c_ulong,
    );
    gcur.yscale = 1 as libc::c_int as libc::c_float;
    gcur.xscale = gcur.yscale;
    gcur.linewidth = 0.7f64 as libc::c_float;
    gcur.cos = 1 as libc::c_int as libc::c_float;
    gcur.font_n = strdup(b"\0" as *const u8 as *const libc::c_char);
    gcur.font_n_old = strdup(b"\0" as *const u8 as *const libc::c_char);
    memcpy(
        &mut gold as *mut gc as *mut libc::c_void,
        &mut gcur as *mut gc as *const libc::c_void,
        ::core::mem::size_of::<gc>() as libc::c_ulong,
    );
    y_rot = 0 as libc::c_int as libc::c_float;
    x_rot = y_rot;
    nsave = 0 as libc::c_int;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[C2RustUnnamed_0; 91]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong)
    {
        if def_tb[i as usize].defined as libc::c_int == 1 as libc::c_int {
            def_tb[i as usize].defined = 0 as libc::c_int as libc::c_char;
        }
        i = i.wrapping_add(1);
        i;
    }
    if file_initialized > 0 as libc::c_int {
        return;
    }
    elts_reset();
    n_sym = 0 as libc::c_int;
    in_cnt = 0 as libc::c_int;
    path = 0 as *mut libc::c_char;
    ps_error = 0 as libc::c_int;
    s = strdup(
        b"/defl 0 def\n/svg 1 def\n/dlw{0.7 SLW}def\n/gsc{gsave y T .8 dup scale 0 0}def\n\0"
            as *const u8 as *const libc::c_char,
    );
    svg_write(s, strlen(s) as libc::c_int);
    free(s as *mut libc::c_void);
}
unsafe extern "C" fn output_font(mut span: libc::c_int) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fn_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut imin: libc::c_int = 0;
    if *(gcur.font_n).offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
        && (span != 0 || gcur.rgb == 0)
    {
        return;
    }
    fprintf(fout, b" style=\"\0" as *const u8 as *const libc::c_char);
    if span == 0 && gcur.rgb != 0 {
        fprintf(fout, b"color:#%06x;\0" as *const u8 as *const libc::c_char, gcur.rgb);
        if *(gcur.font_n).offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
        {
            fprintf(fout, b"\"\0" as *const u8 as *const libc::c_char);
            return;
        }
    }
    fprintf(fout, b"font:\0" as *const u8 as *const libc::c_char);
    fn_0 = gcur.font_n;
    if *fn_0.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
        fn_0 = fn_0.offset(1);
        fn_0;
    }
    imin = 255 as libc::c_int;
    p = strchr(fn_0, '-' as i32);
    if !p.is_null() {
        imin = p.offset_from(fn_0) as libc::c_long as libc::c_int;
    }
    p = strstr(fn_0, b"old\0" as *const u8 as *const libc::c_char);
    if !p.is_null()
        && (*p.offset(-(1 as libc::c_int) as isize) as libc::c_int == 'B' as i32
            || *p.offset(-(1 as libc::c_int) as isize) as libc::c_int == 'b' as i32)
    {
        fprintf(fout, b"bold \0" as *const u8 as *const libc::c_char);
        i = (p.offset_from(fn_0) as libc::c_long - 1 as libc::c_int as libc::c_long)
            as libc::c_int;
        if imin > i {
            imin = i;
        }
    }
    p = strstr(fn_0, b"talic\0" as *const u8 as *const libc::c_char);
    if !p.is_null()
        && (*p.offset(-(1 as libc::c_int) as isize) as libc::c_int == 'I' as i32
            || *p.offset(-(1 as libc::c_int) as isize) as libc::c_int == 'i' as i32)
    {
        fprintf(fout, b"italic \0" as *const u8 as *const libc::c_char);
        i = (p.offset_from(fn_0) as libc::c_long - 1 as libc::c_int as libc::c_long)
            as libc::c_int;
        if imin > i {
            imin = i;
        }
    }
    p = strstr(fn_0, b"blique\0" as *const u8 as *const libc::c_char);
    if !p.is_null()
        && (*p.offset(-(1 as libc::c_int) as isize) as libc::c_int == 'O' as i32
            || *p.offset(-(1 as libc::c_int) as isize) as libc::c_int == 'o' as i32)
    {
        fprintf(fout, b"oblique \0" as *const u8 as *const libc::c_char);
        i = (p.offset_from(fn_0) as libc::c_long - 1 as libc::c_int as libc::c_long)
            as libc::c_int;
        if imin > i {
            imin = i;
        }
    }
    if !(strchr(fn_0, ' ' as i32)).is_null() {
        fprintf(
            fout,
            b"%.2fpx '%.*s'\"\0" as *const u8 as *const libc::c_char,
            gcur.font_s as libc::c_double,
            imin,
            fn_0,
        );
    } else {
        fprintf(
            fout,
            b"%.2fpx %.*s\"\0" as *const u8 as *const libc::c_char,
            gcur.font_s as libc::c_double,
            imin,
            fn_0,
        );
    };
}
unsafe extern "C" fn strw(mut s: *mut libc::c_char) -> libc::c_float {
    let mut c: libc::c_uchar = 0;
    let mut w: libc::c_float = 0.;
    w = 0 as libc::c_int as libc::c_float;
    loop {
        let fresh5 = s;
        s = s.offset(1);
        c = *fresh5 as libc::c_uchar;
        if c as libc::c_int == '\0' as i32 {
            break;
        }
        w = (w as libc::c_double + cwid(c) as libc::c_double * 1.1f64) as libc::c_float;
    }
    return w * gcur.font_s;
}
unsafe extern "C" fn defg1() {
    setg(0 as libc::c_int);
    fprintf(
        fout,
        b"<g stroke-width=\"%.2f\"\0" as *const u8 as *const libc::c_char,
        gcur.linewidth as libc::c_double,
    );
    if gcur.xscale != 1 as libc::c_int as libc::c_float
        || gcur.yscale != 1 as libc::c_int as libc::c_float
        || gcur.rotate != 0 as libc::c_int as libc::c_float
    {
        fprintf(fout, b" transform=\"\0" as *const u8 as *const libc::c_char);
        if gcur.xscale != 1 as libc::c_int as libc::c_float
            || gcur.yscale != 1 as libc::c_int as libc::c_float
        {
            if gcur.xscale == gcur.yscale {
                fprintf(
                    fout,
                    b"scale(%.3f)\0" as *const u8 as *const libc::c_char,
                    gcur.xscale as libc::c_double,
                );
            } else {
                fprintf(
                    fout,
                    b"scale(%.3f,%.3f)\0" as *const u8 as *const libc::c_char,
                    gcur.xscale as libc::c_double,
                    gcur.yscale as libc::c_double,
                );
            }
        }
        if gcur.rotate != 0 as libc::c_int as libc::c_float {
            if gcur.xoffs != 0 as libc::c_int as libc::c_float
                || gcur.yoffs != 0 as libc::c_int as libc::c_float
            {
                let mut x: libc::c_float = 0.;
                let mut xtmp: libc::c_float = gcur.xoffs;
                let mut y: libc::c_float = gcur.yoffs;
                let mut _sin: libc::c_float = gcur.sin;
                let mut _cos: libc::c_float = gcur.cos;
                x = xtmp * _cos - y * _sin;
                y = xtmp * _sin + y * _cos;
                fprintf(
                    fout,
                    b" translate(%.2f, %.2f)\0" as *const u8 as *const libc::c_char,
                    x as libc::c_double,
                    y as libc::c_double,
                );
                x_rot = gcur.xoffs;
                y_rot = gcur.yoffs;
                gcur.xoffs = 0 as libc::c_int as libc::c_float;
                gcur.yoffs = 0 as libc::c_int as libc::c_float;
            }
            fprintf(
                fout,
                b" rotate(%.2f)\0" as *const u8 as *const libc::c_char,
                gcur.rotate as libc::c_double,
            );
        }
        fputs(b"\"\0" as *const u8 as *const libc::c_char, fout);
    }
    output_font(0 as libc::c_int);
    fprintf(fout, b">\n\0" as *const u8 as *const libc::c_char);
    g = 1 as libc::c_int;
    memcpy(
        &mut gold as *mut gc as *mut libc::c_void,
        &mut gcur as *mut gc as *const libc::c_void,
        ::core::mem::size_of::<gc>() as libc::c_ulong,
    );
}
unsafe extern "C" fn setg(mut newg: libc::c_int) {
    if g == 2 as libc::c_int {
        fputs(b"</text>\n\0" as *const u8 as *const libc::c_char, fout);
        g = 1 as libc::c_int;
    }
    if newg == 0 as libc::c_int {
        if g != 0 as libc::c_int {
            fputs(b"</g>\n\0" as *const u8 as *const libc::c_char, fout);
            if gcur.rotate != 0 as libc::c_int as libc::c_float {
                gcur.xoffs = x_rot;
                gcur.yoffs = y_rot;
                x_rot = 0 as libc::c_int as libc::c_float;
                y_rot = 0 as libc::c_int as libc::c_float;
            }
            g = 0 as libc::c_int;
        }
    } else {
        gold.cx = gcur.cx;
        gold.cy = gcur.cy;
        if memcmp(
            &mut gcur as *mut gc as *const libc::c_void,
            &mut gold as *mut gc as *const libc::c_void,
            ::core::mem::size_of::<gc>() as libc::c_ulong,
        ) != 0 as libc::c_int
        {
            defg1();
        }
    };
}
unsafe extern "C" fn path_print(mut fmt: *mut libc::c_char, mut args: ...) {
    let mut args_0: va_list = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    va_start(args_0, fmt);
    vsnprintf(
        path_buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        fmt,
        args_0,
    );
    va_end(args_0);
    if path.is_null() {
        path = malloc(
            (strlen(path_buf.as_mut_ptr()))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        p = path;
    } else {
        path = realloc(
            path as *mut libc::c_void,
            (strlen(path))
                .wrapping_add(strlen(path_buf.as_mut_ptr()))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        p = path.offset(strlen(path) as isize);
    }
    if path.is_null() {
        fprintf(__stderrp, b"Out of memory.\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    strcpy(p, path_buf.as_mut_ptr());
}
unsafe extern "C" fn path_def() {
    if !path.is_null() {
        return;
    }
    setg(1 as libc::c_int);
    path_print(
        b"<path d=\"m%.2f %.2f\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        (gcur.xoffs + gcur.cx) as libc::c_double,
        (gcur.yoffs - gcur.cy) as libc::c_double,
    );
}
unsafe extern "C" fn path_end() {
    setg(1 as libc::c_int);
    fputs(path, fout);
    free(path as *mut libc::c_void);
    path = 0 as *mut libc::c_char;
}
unsafe extern "C" fn def_use(mut def: libc::c_int) {
    let mut i: libc::c_int = 0;
    ps_exec(b"dlw\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    setg(1 as libc::c_int);
    if def_tb[def as usize].defined != 0 {
        return;
    }
    def_tb[def as usize].defined = 1 as libc::c_int as libc::c_char;
    fputs(b"<defs>\n\0" as *const u8 as *const libc::c_char, fout);
    i = def_tb[def as usize].use_0 as libc::c_int;
    while i != 0 as libc::c_int && def_tb[i as usize].defined == 0 {
        def_tb[i as usize].defined = 1 as libc::c_int as libc::c_char;
        fputs(def_tb[i as usize].def, fout);
        i = def_tb[i as usize].use_0 as libc::c_int;
    }
    fputs(def_tb[def as usize].def, fout);
    fputs(b"</defs>\n\0" as *const u8 as *const libc::c_char, fout);
}
#[no_mangle]
pub unsafe extern "C" fn svg_def_id(mut id: *mut libc::c_char, mut idsz: libc::c_int) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[C2RustUnnamed_0; 91]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong)
    {
        p = strstr(def_tb[i as usize].def, b"id=\0" as *const u8 as *const libc::c_char);
        if strncmp(p, id, idsz as libc::c_ulong) == 0 as libc::c_int {
            def_tb[i as usize].defined = 2 as libc::c_int as libc::c_char;
            return;
        }
        i += 1;
        i;
    }
    if defs.is_null() {
        defssz = 8192 as libc::c_int;
        defs = malloc(defssz as libc::c_ulong) as *mut libc::c_char;
        *defs = '\0' as i32 as libc::c_char;
    }
    i = strlen(defs) as libc::c_int;
    if idsz + i + 1 as libc::c_int >= defssz {
        defssz += 8192 as libc::c_int;
        defs = realloc(defs as *mut libc::c_void, defssz as libc::c_ulong)
            as *mut libc::c_char;
    }
    strncpy(defs.offset(i as isize), id, idsz as libc::c_ulong);
    *defs.offset((i + idsz) as isize) = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn xysym(mut op: *mut libc::c_char, mut use_0: libc::c_int) {
    let mut x: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    if use_0 >= 0 as libc::c_int {
        def_use(use_0);
    }
    y = gcur.yoffs - pop_free_val();
    x = gcur.xoffs + pop_free_val();
    fprintf(
        fout,
        b"<use x=\"%.2f\" y=\"%.2f\" xlink:href=\"#%s\"/>\n\0" as *const u8
            as *const libc::c_char,
        x as libc::c_double,
        y as libc::c_double,
        op,
    );
}
unsafe extern "C" fn setxory(mut s: *mut libc::c_char, mut v: libc::c_float) {
    let mut e: *mut elt_s = 0 as *mut elt_s;
    let mut sym: *mut ps_sym_s = 0 as *mut ps_sym_s;
    sym = ps_sym_lookup(s);
    if sym.is_null() || (*(*sym).e).type_0 as libc::c_int != VAL as libc::c_int {
        e = elt_new();
        if e.is_null() {
            return;
        }
        (*e).type_0 = VAL as libc::c_int as libc::c_char;
        sym = ps_sym_def(s, e);
        if sym.is_null() {
            return;
        }
    }
    (*(*sym).e).u.v = v;
}
unsafe extern "C" fn setxysym(mut op: *mut libc::c_char, mut use_0: libc::c_int) {
    let mut x: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    y = pop_free_val();
    x = pop_free_val();
    setxory(b"x\0" as *const u8 as *const libc::c_char as *mut libc::c_char, x);
    setxory(b"y\0" as *const u8 as *const libc::c_char as *mut libc::c_char, y);
    def_use(use_0);
    fprintf(
        fout,
        b"<use x=\"%.2f\" y=\"%.2f\" xlink:href=\"#%s\"/>\n\0" as *const u8
            as *const libc::c_char,
        (gcur.xoffs + x) as libc::c_double,
        (gcur.yoffs - y) as libc::c_double,
        op,
    );
}
unsafe extern "C" fn acciac(mut op: *mut libc::c_char) {
    let mut sym: *mut ps_sym_s = 0 as *mut ps_sym_s;
    let mut x: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    let mut dx: libc::c_float = 0.;
    let mut dy: libc::c_float = 0.;
    setg(1 as libc::c_int);
    dy = pop_free_val();
    dx = pop_free_val();
    sym = ps_sym_lookup(b"x\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    x = gcur.xoffs + (*(*sym).e).u.v;
    sym = ps_sym_lookup(b"y\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    y = gcur.yoffs - (*(*sym).e).u.v;
    if *op.offset(1 as libc::c_int as isize) as libc::c_int == 'u' as i32 {
        x -= 1 as libc::c_int as libc::c_float;
        y -= 4 as libc::c_int as libc::c_float;
    } else {
        x -= 5 as libc::c_int as libc::c_float;
        y += 4 as libc::c_int as libc::c_float;
    }
    fprintf(
        fout,
        b"<path d=\"M%.2f %.2fl%.2f %.2f\" class=\"stroke\"/>\n\0" as *const u8
            as *const libc::c_char,
        x as libc::c_double,
        y as libc::c_double,
        dx as libc::c_double,
        -dy as libc::c_double,
    );
}
unsafe extern "C" fn arp_ltr(mut type_0: libc::c_char) {
    let mut x: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    let mut t: libc::c_float = 0.;
    let mut n: libc::c_int = 0;
    def_use(83 as libc::c_int);
    y = gcur.yoffs - pop_free_val();
    x = gcur.xoffs + pop_free_val();
    n = ((pop_free_val() + 5 as libc::c_int as libc::c_float)
        / 6 as libc::c_int as libc::c_float) as libc::c_int;
    if type_0 as libc::c_int == 'a' as i32 {
        fprintf(
            fout,
            b"<g transform=\"rotate(270)\">\n\0" as *const u8 as *const libc::c_char,
        );
        t = x;
        x = -y;
        y = t;
    }
    y -= 4 as libc::c_int as libc::c_float;
    loop {
        n -= 1;
        if !(n >= 0 as libc::c_int) {
            break;
        }
        fprintf(
            fout,
            b"<use x=\"%.2f\" y=\"%.2f\" xlink:href=\"#ltr\"/>\n\0" as *const u8
                as *const libc::c_char,
            x as libc::c_double,
            y as libc::c_double,
        );
        x += 6 as libc::c_int as libc::c_float;
    }
    if type_0 as libc::c_int == 'a' as i32 {
        fprintf(fout, b"</g>\n\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn gliss(mut squiggle: libc::c_int) {
    let mut x1: libc::c_float = 0.;
    let mut y1: libc::c_float = 0.;
    let mut x2: libc::c_float = 0.;
    let mut y2: libc::c_float = 0.;
    let mut ar: libc::c_float = 0.;
    let mut a: libc::c_float = 0.;
    let mut len: libc::c_float = 0.;
    let mut n: libc::c_int = 0;
    if squiggle != 0 {
        def_use(83 as libc::c_int);
    }
    y1 = gcur.yoffs - pop_free_val();
    x1 = gcur.xoffs + pop_free_val();
    y2 = gcur.yoffs - pop_free_val();
    x2 = gcur.xoffs + pop_free_val();
    ar = atan(((y2 - y1) / (x2 - x1)) as libc::c_double) as libc::c_float;
    a = (ar as libc::c_double / 3.14159265358979323846264338327950288f64
        * 180 as libc::c_int as libc::c_double) as libc::c_float;
    len = ((x2 - x1 - 14 as libc::c_int as libc::c_float) as libc::c_double
        / cos(ar as libc::c_double)) as libc::c_float;
    fprintf(
        fout,
        b"<g transform=\"translate(%.2f,%.2f) rotate(%.2f)\">\n\0" as *const u8
            as *const libc::c_char,
        x1 as libc::c_double,
        y1 as libc::c_double,
        a as libc::c_double,
    );
    if squiggle != 0 {
        n = ((len + 2 as libc::c_int as libc::c_float)
            / 6 as libc::c_int as libc::c_float) as libc::c_int;
        x1 = 8 as libc::c_int as libc::c_float;
        loop {
            n -= 1;
            if !(n >= 0 as libc::c_int) {
                break;
            }
            fprintf(
                fout,
                b"<use x=\"%.2f\" xlink:href=\"#ltr\"/>\n\0" as *const u8
                    as *const libc::c_char,
                x1 as libc::c_double,
            );
            x1 += 6 as libc::c_int as libc::c_float;
        }
    } else {
        fprintf(
            fout,
            b"<path class=\"stroke\" stroke-width=\"1\"\n\td=\"M8 0l%.2f 0\"/>\n\0"
                as *const u8 as *const libc::c_char,
            len as libc::c_double,
        );
    }
    fprintf(fout, b"</g>\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn stem(mut op: *mut libc::c_char) {
    let mut sym: *mut ps_sym_s = 0 as *mut ps_sym_s;
    let mut x: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    let mut dx: libc::c_float = 0.;
    let mut h: libc::c_float = 0.;
    ps_exec(b"dlw\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    setg(1 as libc::c_int);
    h = pop_free_val();
    if *op.offset(0 as libc::c_int as isize) as libc::c_int == 's' as i32 {
        dx = 3.5f64 as libc::c_float;
    } else {
        dx = 2.3f64 as libc::c_float;
    }
    if *op.offset(1 as libc::c_int as isize) as libc::c_int == 'd' as i32 {
        dx = -dx;
    }
    sym = ps_sym_lookup(b"x\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    x = gcur.xoffs + (*(*sym).e).u.v + dx;
    sym = ps_sym_lookup(b"y\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    y = gcur.yoffs - (*(*sym).e).u.v;
    fprintf(
        fout,
        b"<path d=\"M%.2f %.2fv%.2f\" class=\"stroke\"/>\n\0" as *const u8
            as *const libc::c_char,
        x as libc::c_double,
        y as libc::c_double,
        -h as libc::c_double,
    );
}
unsafe extern "C" fn show(mut type_0: libc::c_char) {
    let mut x: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    let mut w: libc::c_float = 0.;
    let mut tmp: [libc::c_char; 4] = [0; 4];
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut span: libc::c_int = 0;
    span = 0 as libc::c_int;
    gold.cx = gcur.cx;
    gold.cy = gcur.cy;
    if memcmp(
        &mut gcur as *mut gc as *const libc::c_void,
        &mut gold as *mut gc as *const libc::c_void,
        ::core::mem::size_of::<gc>() as libc::c_ulong,
    ) != 0 as libc::c_int
    {
        if g == 2 as libc::c_int {
            span = 1 as libc::c_int;
        } else {
            defg1();
        }
    }
    x = gcur.cx;
    y = gcur.cy;
    match type_0 as libc::c_int {
        106 => {
            w = pop_free_val();
            p = tmp.as_mut_ptr();
            tmp[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
            s = 0 as *mut libc::c_char;
        }
        _ => {
            if stack.is_null() {
                fprintf(
                    __stderrp,
                    b"svg top: Stack empty\n\0" as *const u8 as *const libc::c_char,
                );
                ps_error = 1 as libc::c_int;
                return;
            }
            if (*stack).type_0 as libc::c_int == STR as libc::c_int {
                s = pop_free_str();
                if s.is_null()
                    || *s.offset(0 as libc::c_int as isize) as libc::c_int != '(' as i32
                {
                    fprintf(
                        __stderrp,
                        b"svg: No string\n\0" as *const u8 as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                p = s.offset(1 as libc::c_int as isize);
            } else {
                p = tmp.as_mut_ptr();
                tmp[0 as libc::c_int as usize] = pop_free_val() as libc::c_char;
                tmp[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                s = 0 as *mut libc::c_char;
            }
            w = strw(p);
            if type_0 as libc::c_int == 'x' as i32 {
                w = pop_free_val();
                q = strchr(p, '\t' as i32);
                *q = '\0' as i32 as libc::c_char;
            }
        }
    }
    if span != 0 {
        fprintf(fout, b"<tspan\n\t\0" as *const u8 as *const libc::c_char);
        output_font(1 as libc::c_int);
        fprintf(fout, b">\0" as *const u8 as *const libc::c_char);
    } else if g != 2 as libc::c_int {
        fprintf(
            fout,
            b"<text x=\"%.2f\" y=\"%.2f\"\0" as *const u8 as *const libc::c_char,
            (gcur.xoffs + x) as libc::c_double,
            (gcur.yoffs - y) as libc::c_double,
        );
        match type_0 as libc::c_int {
            99 => {
                fprintf(
                    fout,
                    b" text-anchor=\"middle\"\0" as *const u8 as *const libc::c_char,
                );
                w /= 2 as libc::c_int as libc::c_float;
            }
            114 => {
                fprintf(
                    fout,
                    b" text-anchor=\"end\"\0" as *const u8 as *const libc::c_char,
                );
                w = 0 as libc::c_int as libc::c_float;
            }
            106 => {
                fprintf(
                    fout,
                    b" textLength=\"%.2f\"\0" as *const u8 as *const libc::c_char,
                    w as libc::c_double,
                );
            }
            _ => {}
        }
        fputs(b">\0" as *const u8 as *const libc::c_char, fout);
        g = 2 as libc::c_int;
    }
    loop {
        xml_str_out(p);
        if span != 0 {
            fprintf(fout, b"</tspan>\0" as *const u8 as *const libc::c_char);
        }
        if !(type_0 as libc::c_int == 'x' as i32) {
            break;
        }
        p = p.offset(strlen(p) as isize).offset(1 as libc::c_int as isize);
        q = strchr(p, '\t' as i32);
        if !q.is_null() {
            *q = '\0' as i32 as libc::c_char;
        } else {
            w = (*free_elt).u.v;
            type_0 = 's' as i32 as libc::c_char;
        }
        fprintf(
            fout,
            b"<tspan dx=\"%.2f\">\0" as *const u8 as *const libc::c_char,
            w as libc::c_double,
        );
        span = 1 as libc::c_int;
    }
    if type_0 as libc::c_int == 'b' as i32 {
        setg(1 as libc::c_int);
        fprintf(
            fout,
            b"<rect class=\"stroke\" stroke-width=\"0.6\"\n\tx=\"%.2f\" y=\"%.2f\" width=\"%.2f\" height=\"%.2f\"/>\n\0"
                as *const u8 as *const libc::c_char,
            (gcur.xoffs + gcur.cx - 2 as libc::c_int as libc::c_float) as libc::c_double,
            (gcur.yoffs - y - gcur.font_s + 2 as libc::c_int as libc::c_float)
                as libc::c_double,
            (w + 4 as libc::c_int as libc::c_float) as libc::c_double,
            (gcur.font_s + 1 as libc::c_int as libc::c_float) as libc::c_double,
        );
    }
    gcur.cx = x + w;
    if !s.is_null() {
        free(s as *mut libc::c_void);
    }
}
unsafe extern "C" fn seq_exec(mut e: *mut elt_s) -> libc::c_int {
    let mut e2: *mut elt_s = 0 as *mut elt_s;
    's_49: {
        match (*e).type_0 as libc::c_int {
            1 => {
                if *((*e).u.s).offset(0 as libc::c_int as isize) as libc::c_int
                    != '/' as i32
                    && *((*e).u.s).offset(0 as libc::c_int as isize) as libc::c_int
                        != '(' as i32
                {
                    if strcmp((*e).u.s, b"exit\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                        return 1 as libc::c_int;
                    }
                    ps_exec((*e).u.s);
                    return 0 as libc::c_int;
                }
            }
            0 | 3 => {}
            _ => {
                break 's_49;
            }
        }
        e = elt_dup(e);
        if e.is_null() {
            return 1 as libc::c_int;
        }
        push(e);
        return 0 as libc::c_int;
    }
    e = (*e).u.e;
    while !e.is_null() {
        let mut current_block_19: u64;
        match (*e).type_0 as libc::c_int {
            1 => {
                if strcmp((*e).u.s, b"exit\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    return 1 as libc::c_int;
                }
                if *((*e).u.s).offset(0 as libc::c_int as isize) as libc::c_int
                    != '(' as i32
                    && *((*e).u.s).offset(0 as libc::c_int as isize) as libc::c_int
                        != '/' as i32
                {
                    ps_exec((*e).u.s);
                    current_block_19 = 13242334135786603907;
                } else {
                    current_block_19 = 2085934361839010130;
                }
            }
            _ => {
                current_block_19 = 2085934361839010130;
            }
        }
        match current_block_19 {
            2085934361839010130 => {
                e2 = elt_dup(e);
                if e2.is_null() {
                    return 1 as libc::c_int;
                }
                push(e2);
            }
            _ => {}
        }
        e = (*e).next;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn ps_exec(mut op: *mut libc::c_char) {
    let mut sym: *mut ps_sym_s = 0 as *mut ps_sym_s;
    let mut e: *mut elt_s = 0 as *mut elt_s;
    let mut e2: *mut elt_s = 0 as *mut elt_s;
    let mut x: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    let mut w: libc::c_float = 0.;
    let mut h: libc::c_float = 0.;
    let mut n: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if ps_error != 0 {
        return;
    }
    sym = ps_sym_lookup(op);
    if !sym.is_null() {
        (*sym).exec += 1;
        if (*sym).exec > 2 as libc::c_int {
            fprintf(
                __stderrp,
                b"svg: Too many recursions of '%s'\n\0" as *const u8
                    as *const libc::c_char,
                op,
            );
            ps_error = 1 as libc::c_int;
            return;
        }
        seq_exec((*sym).e);
        (*sym).exec -= 1;
        (*sym).exec;
        return;
    }
    if *op as libc::c_int == ' ' as i32 {
        op = op.offset(1);
        op;
    }
    let mut c1_0: libc::c_float = 0.;
    let mut c2_0: libc::c_float = 0.;
    let mut c3_0: libc::c_float = 0.;
    let mut c4_0: libc::c_float = 0.;
    let mut c1: libc::c_float = 0.;
    let mut c2: libc::c_float = 0.;
    let mut c3: libc::c_float = 0.;
    let mut c4: libc::c_float = 0.;
    let mut current_block_2019: u64;
    match *op as libc::c_int {
        33 => {
            if *op.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
                if stack.is_null() {
                    fprintf(
                        __stderrp,
                        b"svg def: Stack empty\n\0" as *const u8 as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                e = pop((*stack).type_0 as libc::c_int);
                s = pop_free_str();
                if s.is_null() || *s as libc::c_int != '/' as i32 {
                    fprintf(
                        __stderrp,
                        b"svg def: No / bad symbol\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    if !s.is_null() {
                        free(s as *mut libc::c_void);
                    }
                    ps_error = 1 as libc::c_int;
                    return;
                }
                ps_sym_def(&mut *s.offset(1 as libc::c_int as isize), e);
                free(s as *mut libc::c_void);
                return;
            }
            current_block_2019 = 18125716024132132232;
        }
        97 => {
            if strcmp(op, b"accent\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 60 as libc::c_int);
                return;
            }
            if strcmp(op, b"abs\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                if stack.is_null()
                    || (*stack).type_0 as libc::c_int != VAL as libc::c_int
                {
                    fprintf(
                        __stderrp,
                        b"svg abs: Bad value\n\0" as *const u8 as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                if (*stack).u.v < 0 as libc::c_int as libc::c_float {
                    (*stack).u.v = -(*stack).u.v;
                }
                return;
            }
            if strcmp(op, b"add\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                x = pop_free_val();
                if stack.is_null()
                    || (*stack).type_0 as libc::c_int != VAL as libc::c_int
                {
                    fprintf(
                        __stderrp,
                        b"svg add: Bad value\n\0" as *const u8 as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                (*stack).u.v += x;
                return;
            }
            if strcmp(op, b"and\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                x = pop_free_val();
                if stack.is_null()
                    || (*stack).type_0 as libc::c_int != VAL as libc::c_int
                {
                    fprintf(
                        __stderrp,
                        b"svg and: Bad value\n\0" as *const u8 as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                (*stack)
                    .u
                    .v = (x as libc::c_int & (*stack).u.v as libc::c_int)
                    as libc::c_float;
                return;
            }
            if strcmp(op, b"anshow\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                show('s' as i32 as libc::c_char);
                return;
            }
            if strcmp(op, b"arc\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
                || strcmp(op, b"arcn\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                let mut r: libc::c_float = 0.;
                let mut a1: libc::c_float = 0.;
                let mut a2: libc::c_float = 0.;
                let mut x1: libc::c_float = 0.;
                let mut y1: libc::c_float = 0.;
                let mut x2: libc::c_float = 0.;
                let mut y2: libc::c_float = 0.;
                a2 = pop_free_val();
                a1 = pop_free_val();
                r = pop_free_val();
                if r < 0 as libc::c_int as libc::c_float {
                    fprintf(
                        __stderrp,
                        b"svg arc: Bad value\n\0" as *const u8 as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                if a1 >= 360 as libc::c_int as libc::c_float {
                    a1 -= 360 as libc::c_int as libc::c_float;
                }
                if a2 >= 360 as libc::c_int as libc::c_float {
                    a2 -= 360 as libc::c_int as libc::c_float;
                }
                y = pop_free_val();
                x = pop_free_val();
                x1 = (x as libc::c_double
                    + r as libc::c_double
                        * cos(
                            a1 as libc::c_double
                                * 3.14159265358979323846264338327950288f64
                                / 180 as libc::c_int as libc::c_double,
                        )) as libc::c_float;
                y1 = y
                    + r
                        * sinf(
                            (a1 as libc::c_double
                                * 3.14159265358979323846264338327950288f64
                                / 180 as libc::c_int as libc::c_double) as libc::c_float,
                        );
                if gcur.cx != ((2 as libc::c_int) << 22 as libc::c_int) as libc::c_float
                {
                    if !path.is_null() {
                        path_print(
                            b"\n\t%c%.2f %.2f\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            if x1 != gcur.cx || y1 != gcur.cy {
                                'l' as i32
                            } else {
                                'm' as i32
                            },
                            (x1 - gcur.cx) as libc::c_double,
                            -(y1 - gcur.cy) as libc::c_double,
                        );
                    } else {
                        gcur.cx = x1;
                        gcur.cy = y1;
                        path_def();
                    }
                } else {
                    gcur.cx = x1;
                    gcur.cy = y1;
                    path_def();
                }
                if a1 == a2 {
                    a2 = 180 as libc::c_int as libc::c_float - a1;
                    x2 = x
                        + r
                            * cosf(
                                (a2 as libc::c_double
                                    * 3.14159265358979323846264338327950288f64
                                    / 180 as libc::c_int as libc::c_double) as libc::c_float,
                            );
                    y2 = y
                        + r
                            * sinf(
                                (a2 as libc::c_double
                                    * 3.14159265358979323846264338327950288f64
                                    / 180 as libc::c_int as libc::c_double) as libc::c_float,
                            );
                    path_print(
                        b"\n\ta%.2f %.2f 0 0 %d %.2f %.2f %.2f %.2f 0 0 %d %.2f %.2f\n\0"
                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                        r as libc::c_double,
                        r as libc::c_double,
                        (*op.offset(3 as libc::c_int as isize) as libc::c_int
                            == 'n' as i32) as libc::c_int,
                        (x2 - x1) as libc::c_double,
                        -(y2 - y1) as libc::c_double,
                        r as libc::c_double,
                        r as libc::c_double,
                        (*op.offset(3 as libc::c_int as isize) as libc::c_int
                            == 'n' as i32) as libc::c_int,
                        (x1 - x2) as libc::c_double,
                        -(y1 - y2) as libc::c_double,
                    );
                    gcur.cx = x1;
                    gcur.cy = y1;
                } else {
                    x2 = x
                        + r
                            * cosf(
                                (a2 as libc::c_double
                                    * 3.14159265358979323846264338327950288f64
                                    / 180 as libc::c_int as libc::c_double) as libc::c_float,
                            );
                    y2 = y
                        + r
                            * sinf(
                                (a2 as libc::c_double
                                    * 3.14159265358979323846264338327950288f64
                                    / 180 as libc::c_int as libc::c_double) as libc::c_float,
                            );
                    path_print(
                        b"\n\ta%.2f %.2f 0 0 %d %.2f %.2f\n\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        r as libc::c_double,
                        r as libc::c_double,
                        (*op.offset(3 as libc::c_int as isize) as libc::c_int
                            == 'n' as i32) as libc::c_int,
                        (x2 - x1) as libc::c_double,
                        -(y2 - y1) as libc::c_double,
                    );
                    gcur.cx = x2;
                    gcur.cy = y2;
                }
                return;
            }
            if strcmp(op, b"arp\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                arp_ltr('a' as i32 as libc::c_char);
                return;
            }
            if strcmp(op, b"atan\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                x = pop_free_val();
                if stack.is_null()
                    || (*stack).type_0 as libc::c_int != VAL as libc::c_int
                    || x == 0 as libc::c_int as libc::c_float
                {
                    fprintf(
                        __stderrp,
                        b"svg atan: Bad value\n\0" as *const u8 as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                y = (*stack).u.v;
                (*stack)
                    .u
                    .v = (atan((y / x) as libc::c_double)
                    / 3.14159265358979323846264338327950288f64
                    * 180 as libc::c_int as libc::c_double) as libc::c_float;
                return;
            }
            current_block_2019 = 18125716024132132232;
        }
        98 => {
            if strcmp(op, b"bar\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setg(1 as libc::c_int);
                y = gcur.yoffs - pop_free_val();
                x = gcur.xoffs + pop_free_val();
                h = pop_free_val();
                fprintf(
                    fout,
                    b"<path class=\"stroke\" stroke-width=\"1\"\n\td=\"M%.2f %.2fv%.2f\"/>\n\0"
                        as *const u8 as *const libc::c_char,
                    x as libc::c_double,
                    y as libc::c_double,
                    -h as libc::c_double,
                );
                return;
            }
            if strcmp(op, b"bclef\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 5 as libc::c_int);
                return;
            }
            if strcmp(op, b"bdef\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                ps_exec(b"!\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
                return;
            }
            if strcmp(op, b"bind\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                return;
            }
            if strcmp(op, b"bitshift\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                let mut shift: libc::c_int = 0;
                shift = pop_free_val() as libc::c_int;
                if stack.is_null()
                    || (*stack).type_0 as libc::c_int != VAL as libc::c_int
                    || shift >= 32 as libc::c_int || shift < -(32 as libc::c_int)
                {
                    fprintf(
                        __stderrp,
                        b"svg: Bad value for bitshift\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                if shift > 0 as libc::c_int {
                    n = ((*stack).u.v as libc::c_int) << shift;
                } else {
                    n = (*stack).u.v as libc::c_int >> -shift;
                }
                (*stack).u.v = n as libc::c_float;
                return;
            }
            if strcmp(op, b"bm\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                let mut dx: libc::c_float = 0.;
                let mut dy: libc::c_float = 0.;
                setg(1 as libc::c_int);
                y = gcur.yoffs - pop_free_val();
                x = gcur.xoffs + pop_free_val();
                dy = pop_free_val();
                dx = pop_free_val();
                h = pop_free_val();
                fprintf(
                    fout,
                    b"<path class=\"fill\"\n\td=\"M%.2f %.2fl%.2f %.2fv%.2fl%.2f %.2f\"/>\n\0"
                        as *const u8 as *const libc::c_char,
                    x as libc::c_double,
                    y as libc::c_double,
                    dx as libc::c_double,
                    -dy as libc::c_double,
                    h as libc::c_double,
                    -dx as libc::c_double,
                    dy as libc::c_double,
                );
                return;
            }
            if strcmp(op, b"bnum\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
                || strcmp(op, b"bnumb\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                setg(1 as libc::c_int);
                y = gcur.yoffs - pop_free_val();
                x = gcur.xoffs + pop_free_val();
                s = pop_free_str();
                if s.is_null() {
                    fprintf(
                        __stderrp,
                        b"svg: No string\n\0" as *const u8 as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                if *op.offset(4 as libc::c_int as isize) as libc::c_int == 'b' as i32 {
                    w = (7 as libc::c_int as libc::c_ulong).wrapping_mul(strlen(s))
                        as libc::c_float;
                    fprintf(
                        fout,
                        b"<rect x=\"%.2f\" y=\"%.2f\" width=\"%.2f\" height=\"12\" fill=\"white\"/>\n\0"
                            as *const u8 as *const libc::c_char,
                        (x - w / 2 as libc::c_int as libc::c_float) as libc::c_double,
                        (y - 10 as libc::c_int as libc::c_float) as libc::c_double,
                        w as libc::c_double,
                    );
                }
                fprintf(
                    fout,
                    b"<text style=\"font:italic 12px serif\"\n\tx=\"%.2f\" y=\"%.2f\" text-anchor=\"middle\">%s</text>\n\0"
                        as *const u8 as *const libc::c_char,
                    x as libc::c_double,
                    y as libc::c_double,
                    s.offset(1 as libc::c_int as isize),
                );
                free(s as *mut libc::c_void);
                return;
            }
            if strcmp(op, b"box\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setg(1 as libc::c_int);
                h = pop_free_val();
                w = pop_free_val();
                y = gcur.yoffs - pop_free_val();
                x = gcur.xoffs + pop_free_val();
                fprintf(
                    fout,
                    b"<rect class=\"stroke\"\n\tx=\"%.2f\" y=\"%.2f\" width=\"%.2f\" height=\"%.2f\"/>\n\0"
                        as *const u8 as *const libc::c_char,
                    x as libc::c_double,
                    (y - h) as libc::c_double,
                    w as libc::c_double,
                    h as libc::c_double,
                );
                return;
            }
            if strcmp(op, b"boxdraw\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setg(1 as libc::c_int);
                h = pop_free_val();
                y = gcur.yoffs - pop_free_val();
                x = gcur.xoffs + pop_free_val();
                fprintf(
                    fout,
                    b"<rect class=\"stroke\"\n\tx=\"%.2f\" y=\"%.2f\" width=\"%.2f\" height=\"%.2f\"/>\n\0"
                        as *const u8 as *const libc::c_char,
                    x as libc::c_double,
                    (y - h) as libc::c_double,
                    (boxend as libc::c_float - (x - gcur.xoffs)
                        + 2 as libc::c_int as libc::c_float) as libc::c_double,
                    h as libc::c_double,
                );
                return;
            }
            if strcmp(op, b"boxmark\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                if gcur.cx > boxend as libc::c_float {
                    boxend = gcur.cx as libc::c_int;
                }
                return;
            }
            if strcmp(op, b"boxend\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                boxend = gcur.cx as libc::c_int;
                return;
            }
            if strcmp(op, b"brace\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                def_use(0 as libc::c_int);
                y = gcur.yoffs - pop_free_val();
                x = gcur.xoffs + pop_free_val();
                h = (pop_free_val() as libc::c_double * 0.01f64) as libc::c_float;
                fprintf(
                    fout,
                    b"<g transform=\"translate(%.2f,%.2f) scale(1,%.2f)\">\n\t<use xlink:href=\"#brace\"/>\n</g>\n\0"
                        as *const u8 as *const libc::c_char,
                    x as libc::c_double,
                    y as libc::c_double,
                    h as libc::c_double,
                );
                return;
            }
            if strcmp(op, b"bracket\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setg(1 as libc::c_int);
                y = gcur.yoffs - pop_free_val() - 3 as libc::c_int as libc::c_float;
                x = gcur.xoffs + pop_free_val() - 5 as libc::c_int as libc::c_float;
                h = pop_free_val() + 2 as libc::c_int as libc::c_float;
                fprintf(
                    fout,
                    b"<path class=\"fill\"\n\td=\"M%.2f %.2f\n\tc10.5 1 12 -4.5 12 -3.5c0 1 -3.5 5.5 -8.5 5.5\n\tv%.2f\n\tc5 0 8.5 4.5 8.5 5.5c0 1 -1.5 -4.5 -12 -3.5\"/>\n\0"
                        as *const u8 as *const libc::c_char,
                    x as libc::c_double,
                    y as libc::c_double,
                    h as libc::c_double,
                );
                return;
            }
            if strcmp(op, b"breve\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setxysym(op, 15 as libc::c_int);
                return;
            }
            if strcmp(op, b"brth\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setg(1 as libc::c_int);
                y = gcur.yoffs - pop_free_val() - 6 as libc::c_int as libc::c_float;
                x = gcur.xoffs + pop_free_val();
                fprintf(
                    fout,
                    b"<text x=\"%.2f\" y=\"%.2f\" style=\"font:bold italic 30px serif\">,</text>\n\0"
                        as *const u8 as *const libc::c_char,
                    x as libc::c_double,
                    y as libc::c_double,
                );
                return;
            }
            current_block_2019 = 18125716024132132232;
        }
        67 => {
            if strcmp(op, b"C\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            {
                c1 = 0.;
                c2 = 0.;
                c3 = 0.;
                c4 = 0.;
                current_block_2019 = 45961622177256480;
            } else {
                current_block_2019 = 18125716024132132232;
            }
        }
        99 => {
            if strcmp(op, b"cclef\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 8 as libc::c_int);
                return;
            }
            if strcmp(op, b"csig\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 46 as libc::c_int);
                return;
            }
            if strcmp(op, b"ctsig\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 47 as libc::c_int);
                return;
            }
            if strcmp(op, b"coda\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 72 as libc::c_int);
                return;
            }
            if strcmp(op, b"closepath\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                if !path.is_null() {
                    path_print(
                        b"\tz\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                }
                return;
            }
            if strcmp(op, b"composefont\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                pop(BRK as libc::c_int);
                pop(STR as libc::c_int);
                return;
            }
            if strcmp(op, b"copy\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                let mut e3: *mut elt_s = 0 as *mut elt_s;
                n = pop_free_val() as libc::c_int;
                if n as libc::c_uint > 10 as libc::c_int as libc::c_uint {
                    fprintf(
                        __stderrp,
                        b"svg copy: Too wide\n\0" as *const u8 as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                e = stack;
                e2 = 0 as *mut elt_s;
                loop {
                    n -= 1;
                    if !(n >= 0 as libc::c_int) {
                        break;
                    }
                    if e.is_null() {
                        break;
                    }
                    e3 = elt_dup(e);
                    if e3.is_null() {
                        return;
                    }
                    (*e3).next = e2;
                    e2 = e3;
                    e = (*e).next;
                }
                if n >= 0 as libc::c_int {
                    fprintf(
                        __stderrp,
                        b"svg copy: Stack empty\n\0" as *const u8 as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                while !e2.is_null() {
                    e3 = (*e2).next;
                    push(e2);
                    e2 = e3;
                }
                return;
            }
            if strcmp(op, b"cos\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                if stack.is_null()
                    || (*stack).type_0 as libc::c_int != VAL as libc::c_int
                {
                    fprintf(
                        __stderrp,
                        b"svg cos: Bad value\n\0" as *const u8 as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                (*stack)
                    .u
                    .v = cos(
                    (*stack).u.v as libc::c_double
                        * 3.14159265358979323846264338327950288f64
                        / 180 as libc::c_int as libc::c_double,
                ) as libc::c_float;
                return;
            }
            if strcmp(op, b"cpu\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 68 as libc::c_int);
                return;
            }
            if strcmp(op, b"crdc\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setg(1 as libc::c_int);
                y = gcur.yoffs - pop_free_val() - 5 as libc::c_int as libc::c_float;
                x = gcur.xoffs + pop_free_val();
                s = pop_free_str();
                if s.is_null() {
                    fprintf(
                        __stderrp,
                        b"svg crdc: No string\n\0" as *const u8 as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                fprintf(
                    fout,
                    b"<text style=\"font:italic 16px serif\"\n\tx=\"%.2f\" y=\"%.2f\" text-anchor=\"left\">%s</text>\n\0"
                        as *const u8 as *const libc::c_char,
                    x as libc::c_double,
                    y as libc::c_double,
                    s.offset(1 as libc::c_int as isize),
                );
                free(s as *mut libc::c_void);
                return;
            }
            if strcmp(op, b"cresc\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setg(1 as libc::c_int);
                y = gcur.yoffs - pop_free_val() - 5 as libc::c_int as libc::c_float;
                x = gcur.xoffs + pop_free_val();
                w = pop_free_val();
                sym = ps_sym_lookup(
                    b"defl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                x += w;
                if (*(*sym).e).u.v as libc::c_int & 1 as libc::c_int != 0 {
                    fprintf(
                        fout,
                        b"<path class=\"stroke\"\nd=\"M%.2f %.2fl%.2f -2.2m0 -3.6l%.2f -2.2\"/>\n\0"
                            as *const u8 as *const libc::c_char,
                        x as libc::c_double,
                        y as libc::c_double,
                        -w as libc::c_double,
                        w as libc::c_double,
                    );
                } else {
                    fprintf(
                        fout,
                        b"<path class=\"stroke\"\nd=\"M%.2f %.2fl%.2f -4l%.2f -4\"/>\n\0"
                            as *const u8 as *const libc::c_char,
                        x as libc::c_double,
                        y as libc::c_double,
                        -w as libc::c_double,
                        w as libc::c_double,
                    );
                }
                return;
            }
            if strcmp(op, b"custos\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 84 as libc::c_int);
                return;
            }
            if strcmp(op, b"currentgray\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                e = elt_new();
                if e.is_null() {
                    return;
                }
                (*e).type_0 = VAL as libc::c_int as libc::c_char;
                (*e)
                    .u
                    .v = gcur.rgb as libc::c_float
                    / 0xffffff as libc::c_int as libc::c_float;
                push(e);
                return;
            }
            if strcmp(op, b"currentpoint\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                e = elt_new();
                if e.is_null() {
                    return;
                }
                (*e).type_0 = VAL as libc::c_int as libc::c_char;
                (*e).u.v = gcur.cx;
                push(e);
                e = elt_new();
                if e.is_null() {
                    return;
                }
                (*e).type_0 = VAL as libc::c_int as libc::c_char;
                (*e).u.v = gcur.cy;
                push(e);
                return;
            }
            if strcmp(op, b"curveto\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                current_block_2019 = 45961622177256480;
            } else {
                if strcmp(op, b"cvi\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    if stack.is_null()
                        || (*stack).type_0 as libc::c_int != VAL as libc::c_int
                    {
                        fprintf(
                            __stderrp,
                            b"svg cvi: Bad value\n\0" as *const u8 as *const libc::c_char,
                        );
                        ps_error = 1 as libc::c_int;
                        return;
                    }
                    n = (*stack).u.v as libc::c_int;
                    (*stack).u.v = n as libc::c_float;
                    return;
                }
                if strcmp(op, b"cvx\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    s = pop_free_str();
                    if s.is_null()
                        || *s as libc::c_int != '/' as i32
                            && *s as libc::c_int != '(' as i32
                    {
                        fprintf(
                            __stderrp,
                            b"svg cvx: No / bad string\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        if !s.is_null() {
                            free(s as *mut libc::c_void);
                        }
                        ps_error = 1 as libc::c_int;
                        return;
                    }
                    *s = '{' as i32 as libc::c_char;
                    svg_write(s, strlen(s) as libc::c_int);
                    svg_write(
                        b"}\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        1 as libc::c_int,
                    );
                    free(s as *mut libc::c_void);
                    return;
                }
                current_block_2019 = 18125716024132132232;
            }
        }
        100 => {
            if strcmp(op, b"dacs\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setg(1 as libc::c_int);
                y = gcur.yoffs - pop_free_val() - 3 as libc::c_int as libc::c_float;
                x = gcur.xoffs + pop_free_val();
                s = pop_free_str();
                if s.is_null() {
                    fprintf(
                        __stderrp,
                        b"svg dacs: No string\n\0" as *const u8 as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                fprintf(
                    fout,
                    b"<text style=\"font:16px serif\"\n\tx=\"%.2f\" y=\"%.2f\" text-anchor=\"middle\">%s</text>\n\0"
                        as *const u8 as *const libc::c_char,
                    x as libc::c_double,
                    y as libc::c_double,
                    s.offset(1 as libc::c_int as isize),
                );
                free(s as *mut libc::c_void);
                return;
            }
            if strcmp(op, b"dacoda\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setg(1 as libc::c_int);
                e = elt_dup(stack);
                y = gcur.yoffs - pop_free_val() - 7 as libc::c_int as libc::c_float;
                e2 = elt_dup(stack);
                (*e2).u.v += 10 as libc::c_int as libc::c_float;
                x = gcur.xoffs + pop_free_val() - 10 as libc::c_int as libc::c_float;
                fprintf(
                    fout,
                    b"<text style=\"font:16px serif\"\n\tx=\"%.2f\" y=\"%.2f\" text-anchor=\"middle\">Da</text>\n\0"
                        as *const u8 as *const libc::c_char,
                    x as libc::c_double,
                    y as libc::c_double,
                );
                push(e2);
                push(e);
                xysym(
                    b"coda\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    72 as libc::c_int,
                );
                return;
            }
            if strcmp(op, b"def\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                ps_exec(b"!\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
                return;
            }
            if strcmp(op, b"dim\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setg(1 as libc::c_int);
                y = gcur.yoffs - pop_free_val() - 5 as libc::c_int as libc::c_float;
                x = gcur.xoffs + pop_free_val();
                w = pop_free_val();
                sym = ps_sym_lookup(
                    b"defl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                if (*(*sym).e).u.v as libc::c_int & 2 as libc::c_int != 0 {
                    fprintf(
                        fout,
                        b"<path class=\"stroke\"\nd=\"M%.2f %.2fl%.2f -2.2m0 -3.6l%.2f -2.2\"/>\n\0"
                            as *const u8 as *const libc::c_char,
                        x as libc::c_double,
                        y as libc::c_double,
                        w as libc::c_double,
                        -w as libc::c_double,
                    );
                } else {
                    fprintf(
                        fout,
                        b"<path class=\"stroke\"\nd=\"M%.2f %.2fl%.2f -4l%.2f -4\"/>\n\0"
                            as *const u8 as *const libc::c_char,
                        x as libc::c_double,
                        y as libc::c_double,
                        w as libc::c_double,
                        -w as libc::c_double,
                    );
                }
                return;
            }
            if strcmp(op, b"div\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                x = pop_free_val();
                if stack.is_null()
                    || (*stack).type_0 as libc::c_int != VAL as libc::c_int
                    || x == 0 as libc::c_int as libc::c_float
                {
                    fprintf(
                        __stderrp,
                        b"svg: Bad value for div\n\0" as *const u8 as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                (*stack).u.v /= x;
                return;
            }
            if strcmp(op, b"dnb\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 70 as libc::c_int);
                return;
            }
            if strcmp(op, b"dplus\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 73 as libc::c_int);
                return;
            }
            if strcmp(op, b"dSL\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                let mut a1_0: libc::c_float = 0.;
                let mut a2_0: libc::c_float = 0.;
                let mut a3: libc::c_float = 0.;
                let mut a4: libc::c_float = 0.;
                let mut a5: libc::c_float = 0.;
                let mut a6: libc::c_float = 0.;
                let mut m1: libc::c_float = 0.;
                let mut m2: libc::c_float = 0.;
                setg(1 as libc::c_int);
                m2 = gcur.yoffs - pop_free_val();
                m1 = gcur.xoffs + pop_free_val();
                a6 = pop_free_val();
                a5 = pop_free_val();
                a4 = pop_free_val();
                a3 = pop_free_val();
                a2_0 = pop_free_val();
                a1_0 = pop_free_val();
                fprintf(
                    fout,
                    b"<path class=\"stroke\" stroke-dasharray=\"5,5\"\n\td=\"M%.2f %.2fc%.2f %.2f %.2f %.2f %.2f %.2f\"/>\n\0"
                        as *const u8 as *const libc::c_char,
                    m1 as libc::c_double,
                    m2 as libc::c_double,
                    a1_0 as libc::c_double,
                    -a2_0 as libc::c_double,
                    a3 as libc::c_double,
                    -a4 as libc::c_double,
                    a5 as libc::c_double,
                    -a6 as libc::c_double,
                );
                return;
            }
            if strcmp(op, b"dt\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setg(1 as libc::c_int);
                sym = ps_sym_lookup(
                    b"x\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                x = gcur.xoffs + (*(*sym).e).u.v;
                sym = ps_sym_lookup(
                    b"y\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                y = gcur.yoffs - (*(*sym).e).u.v;
                y -= pop_free_val();
                x += pop_free_val();
                fprintf(
                    fout,
                    b"<circle class=\"fill\" cx=\"%.2f\" cy=\"%.2f\" r=\"1.2\"/>\n\0"
                        as *const u8 as *const libc::c_char,
                    x as libc::c_double,
                    y as libc::c_double,
                );
                return;
            }
            if strcmp(op, b"dotbar\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setg(1 as libc::c_int);
                y = gcur.yoffs - pop_free_val();
                x = gcur.xoffs + pop_free_val();
                h = pop_free_val();
                fprintf(
                    fout,
                    b"<path class=\"stroke\" stroke-dasharray=\"5,5\"\n\td=\"M%.2f %.2fv%.2f\"/>\n\0"
                        as *const u8 as *const libc::c_char,
                    x as libc::c_double,
                    y as libc::c_double,
                    -h as libc::c_double,
                );
                return;
            }
            if strcmp(op, b"dup\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                if stack.is_null() {
                    fprintf(
                        __stderrp,
                        b"svg dup: Stack empty\n\0" as *const u8 as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                e = elt_dup(stack);
                if !e.is_null() {
                    push(e);
                }
                return;
            }
            if strcmp(op, b"dft0\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 39 as libc::c_int);
                return;
            }
            if strcmp(op, b"dsh0\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 38 as libc::c_int);
                return;
            }
            current_block_2019 = 18125716024132132232;
        }
        101 => {
            if strcmp(op, b"emb\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 66 as libc::c_int);
                return;
            }
            if strcmp(op, b"eofill\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                if path.is_null() {
                    fprintf(
                        __stderrp,
                        b"svg eofill: No path\n\0" as *const u8 as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                path_end();
                fprintf(
                    fout,
                    b"\t\" fill-rule=\"evenodd\" class=\"fill\"/>\n\0" as *const u8
                        as *const libc::c_char,
                );
                return;
            }
            if strcmp(op, b"eq\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                cond(0 as libc::c_int);
                return;
            }
            if strcmp(op, b"exch\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                if stack.is_null() || ((*stack).next).is_null() {
                    fprintf(
                        __stderrp,
                        b"svg exch: Stack empty\n\0" as *const u8 as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                e = (*stack).next;
                (*stack).next = (*e).next;
                (*e).next = stack;
                stack = e;
                return;
            }
            if strcmp(op, b"exec\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                e = pop(SEQ as libc::c_int);
                if e.is_null() {
                    return;
                }
                seq_exec(e);
                elt_free(e);
                return;
            }
            current_block_2019 = 18125716024132132232;
        }
        70 => {
            if sscanf(
                op,
                b"F%d\0" as *const u8 as *const libc::c_char,
                &mut n as *mut libc::c_int,
            ) == 1 as libc::c_int
            {
                h = pop_free_val();
                if (fontnames[n as usize]).is_null() {
                    current_block_2019 = 18125716024132132232;
                } else {
                    if gcur.font_s != h
                        || strcmp(fontnames[n as usize], gcur.font_n) != 0 as libc::c_int
                    {
                        free(gcur.font_n_old as *mut libc::c_void);
                        gcur.font_n_old = gcur.font_n;
                        gcur.font_n = strdup(fontnames[n as usize]);
                        gcur.font_s = h;
                        gold.font_n = 0 as *mut libc::c_char;
                    }
                    return;
                }
            } else {
                current_block_2019 = 18125716024132132232;
            }
        }
        102 => {
            if strcmp(op, b"false\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                e = elt_new();
                if e.is_null() {
                    return;
                }
                (*e).type_0 = VAL as libc::c_int as libc::c_char;
                (*e).u.v = 0 as libc::c_int as libc::c_float;
                push(e);
                return;
            }
            if strcmp(op, b"fill\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                if path.is_null() {
                    fprintf(
                        __stderrp,
                        b"svg fill: No path\n\0" as *const u8 as *const libc::c_char,
                    );
                    return;
                }
                path_end();
                fprintf(
                    fout,
                    b"\t\" class=\"fill\"/>\n\0" as *const u8 as *const libc::c_char,
                );
                return;
            }
            if strcmp(op, b"findfont\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                s = pop_free_str();
                if s.is_null() || *s as libc::c_int != '/' as i32 {
                    fprintf(
                        __stderrp,
                        b"svg findfont: No / bad font\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    if !s.is_null() {
                        free(s as *mut libc::c_void);
                    }
                    ps_error = 1 as libc::c_int;
                    return;
                }
                if strcmp(s, gcur.font_n) != 0 as libc::c_int {
                    free(gcur.font_n_old as *mut libc::c_void);
                    gcur.font_n_old = gcur.font_n;
                    gcur.font_n = s;
                    gold.font_n = 0 as *mut libc::c_char;
                } else {
                    free(s as *mut libc::c_void);
                }
                return;
            }
            if strcmp(op, b"fng\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setg(1 as libc::c_int);
                y = gcur.yoffs - pop_free_val() - 1 as libc::c_int as libc::c_float;
                x = gcur.xoffs + pop_free_val() - 3 as libc::c_int as libc::c_float;
                s = pop_free_str();
                if s.is_null() {
                    fprintf(
                        __stderrp,
                        b"svg fng: No string\n\0" as *const u8 as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                fprintf(
                    fout,
                    b"<text style=\"font:8px Bookman\"\n\tx=\"%.2f\" y=\"%.2f\">%s</text>\n\0"
                        as *const u8 as *const libc::c_char,
                    x as libc::c_double,
                    y as libc::c_double,
                    s.offset(1 as libc::c_int as isize),
                );
                free(s as *mut libc::c_void);
                return;
            }
            if strcmp(op, b"for\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                let mut init: libc::c_float = 0.;
                let mut incr: libc::c_float = 0.;
                let mut limit: libc::c_float = 0.;
                e = pop(SEQ as libc::c_int);
                if e.is_null() {
                    return;
                }
                limit = pop_free_val();
                incr = pop_free_val();
                init = pop_free_val();
                if incr == 0 as libc::c_int as libc::c_float
                    || (limit - init) / incr > 100 as libc::c_int as libc::c_float
                {
                    fprintf(
                        __stderrp,
                        b"svg for: Bad values\n\0" as *const u8 as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                if incr > 0 as libc::c_int as libc::c_float {
                    while init <= limit {
                        e2 = elt_new();
                        if e2.is_null() {
                            break;
                        }
                        (*e2).type_0 = VAL as libc::c_int as libc::c_char;
                        (*e2).u.v = init;
                        push(e2);
                        if seq_exec(e) != 0 as libc::c_int {
                            break;
                        }
                        init += incr;
                    }
                } else {
                    while init >= limit {
                        e2 = elt_new();
                        if e2.is_null() {
                            break;
                        }
                        (*e2).type_0 = VAL as libc::c_int as libc::c_char;
                        (*e2).u.v = init;
                        push(e2);
                        if seq_exec(e) != 0 as libc::c_int {
                            break;
                        }
                        init += incr;
                    }
                }
                elt_free(e);
                return;
            }
            if strcmp(op, b"forall\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                let mut e3_0: *mut elt_s = 0 as *mut elt_s;
                let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                e = pop(SEQ as libc::c_int);
                if e.is_null() {
                    return;
                }
                e2 = stack;
                if e2.is_null() {
                    fprintf(
                        __stderrp,
                        b"svg forall: Stack empty\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                stack = (*e2).next;
                match (*e2).type_0 as libc::c_int {
                    1 => {
                        p = &mut *((*e2).u.s).offset(1 as libc::c_int as isize)
                            as *mut libc::c_char as *mut libc::c_uchar;
                        while *p as libc::c_int != '\0' as i32 {
                            e3_0 = elt_new();
                            if e3_0.is_null() {
                                return;
                            }
                            let fresh6 = p;
                            p = p.offset(1);
                            (*e3_0).u.v = *fresh6 as libc::c_float;
                            push(e3_0);
                            if seq_exec(e) != 0 as libc::c_int {
                                break;
                            }
                        }
                    }
                    3 => {
                        e3_0 = (*e2).u.e;
                        while !e3_0.is_null() {
                            let mut e4: *mut elt_s = 0 as *mut elt_s;
                            e4 = elt_dup(e3_0);
                            push(e4);
                            if seq_exec(e) != 0 as libc::c_int {
                                break;
                            }
                            e3_0 = (*e3_0).next;
                        }
                    }
                    _ => {
                        fprintf(
                            __stderrp,
                            b"svg forall: Bad any\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        ps_error = 1 as libc::c_int;
                        return;
                    }
                }
                elt_free(e);
                elt_free(e2);
                return;
            }
            if strcmp(op, b"ft0\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 36 as libc::c_int);
                return;
            }
            if strcmp(op, b"ft1\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 42 as libc::c_int);
                return;
            }
            if strcmp(op, b"ft4\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                n = pop_free_val() as libc::c_int;
                match n {
                    1 => {
                        xysym(
                            b"ft1\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            42 as libc::c_int,
                        );
                    }
                    2 => {
                        xysym(
                            b"ft0\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            36 as libc::c_int,
                        );
                    }
                    3 => {
                        xysym(
                            b"ft513\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            43 as libc::c_int,
                        );
                    }
                    _ => {
                        xysym(
                            b"dft0\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            39 as libc::c_int,
                        );
                    }
                }
                return;
            }
            if strcmp(op, b"ft513\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 43 as libc::c_int);
                return;
            }
            current_block_2019 = 18125716024132132232;
        }
        103 => {
            if strcmp(op, b"gcshow\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                show('s' as i32 as libc::c_char);
                return;
            }
            if strcmp(op, b"ge\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                cond(3 as libc::c_int);
                return;
            }
            if strcmp(op, b"get\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                n = pop_free_val() as libc::c_int;
                if stack.is_null() {
                    fprintf(
                        __stderrp,
                        b"svg get: Stack empty\n\0" as *const u8 as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                match (*stack).type_0 as libc::c_int {
                    0 => {
                        if n != 0 as libc::c_int {
                            fprintf(
                                __stderrp,
                                b"svg get: Out of bounds\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                            ps_error = 1 as libc::c_int;
                            return;
                        }
                        return;
                    }
                    1 => {
                        s = (*stack).u.s;
                        if s.is_null() || *s as libc::c_int != '(' as i32 {
                            fprintf(
                                __stderrp,
                                b"svg get: Not a string\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                            if !s.is_null() {
                                free(s as *mut libc::c_void);
                            }
                            ps_error = 1 as libc::c_int;
                            return;
                        }
                        if n as libc::c_uint as libc::c_ulong
                            >= (strlen(s))
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        {
                            fprintf(
                                __stderrp,
                                b"svg get: Out of bounds\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                            ps_error = 1 as libc::c_int;
                            return;
                        }
                        (*stack).type_0 = VAL as libc::c_int as libc::c_char;
                        (*stack)
                            .u
                            .v = *s.offset((n + 1 as libc::c_int) as isize)
                            as libc::c_float;
                        free(s as *mut libc::c_void);
                        return;
                    }
                    _ => {}
                }
                e = (*stack).u.e;
                e2 = 0 as *mut elt_s;
                loop {
                    n -= 1;
                    if !(n >= 0 as libc::c_int) {
                        break;
                    }
                    if e.is_null() {
                        break;
                    }
                    e2 = e;
                    e = (*e).next;
                }
                if e.is_null() {
                    fprintf(
                        __stderrp,
                        b"svg get: Out of bounds\n\0" as *const u8 as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                if e2.is_null() {
                    (*stack).u.e = (*e).next;
                } else {
                    (*e2).next = (*e).next;
                }
                (*e).next = (*stack).next;
                elt_free(stack);
                stack = e;
                return;
            }
            if strcmp(op, b"getinterval\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                let mut count: libc::c_int = 0;
                count = pop_free_val() as libc::c_int;
                n = pop_free_val() as libc::c_int;
                s = pop_free_str();
                if s.is_null() || *s as libc::c_int != '(' as i32 {
                    fprintf(
                        __stderrp,
                        b"svg getinterval: No string\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    if !s.is_null() {
                        free(s as *mut libc::c_void);
                    }
                    ps_error = 1 as libc::c_int;
                    return;
                }
                if n as libc::c_uint as libc::c_ulong >= strlen(s)
                    || count as libc::c_uint as libc::c_ulong
                        >= (strlen(s)).wrapping_sub(n as libc::c_ulong)
                {
                    fprintf(
                        __stderrp,
                        b"svg getinterval: Out of bounds\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                e = elt_new();
                if e.is_null() {
                    return;
                }
                (*e).type_0 = STR as libc::c_int as libc::c_char;
                (*e)
                    .u
                    .s = malloc((count + 2 as libc::c_int) as libc::c_ulong)
                    as *mut libc::c_char;
                *((*e).u.s)
                    .offset(0 as libc::c_int as isize) = '(' as i32 as libc::c_char;
                memcpy(
                    &mut *((*e).u.s).offset(1 as libc::c_int as isize)
                        as *mut libc::c_char as *mut libc::c_void,
                    &mut *s.offset((n + 1 as libc::c_int) as isize) as *mut libc::c_char
                        as *const libc::c_void,
                    count as libc::c_ulong,
                );
                *((*e).u.s)
                    .offset(
                        (count + 1 as libc::c_int) as isize,
                    ) = '\0' as i32 as libc::c_char;
                push(e);
                free(s as *mut libc::c_void);
                return;
            }
            if strcmp(op, b"ghd\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setxysym(op, 17 as libc::c_int);
                return;
            }
            if strcmp(op, b"ghl\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 55 as libc::c_int);
                return;
            }
            if strcmp(op, b"glisq\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
                || strcmp(op, b"gliss\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                gliss(
                    (*op.offset(4 as libc::c_int as isize) as libc::c_int == 'q' as i32)
                        as libc::c_int,
                );
                return;
            }
            if strcmp(op, b"gt\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                cond(2 as libc::c_int);
                return;
            }
            if strcmp(op, b"gu\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
                || strcmp(op, b"gd\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                stem(op);
                return;
            }
            if strcmp(op, b"gua\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
                || strcmp(op, b"gda\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                acciac(op);
                return;
            }
            if strcmp(op, b"grestore\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                if nsave <= 0 as libc::c_int {
                    fprintf(
                        __stderrp,
                        b"svg grestore: No gsave\n\0" as *const u8 as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                setg(1 as libc::c_int);
                free(gcur.font_n as *mut libc::c_void);
                free(gcur.font_n_old as *mut libc::c_void);
                nsave -= 1;
                memcpy(
                    &mut gcur as *mut gc as *mut libc::c_void,
                    &mut *gsave.as_mut_ptr().offset(nsave as isize) as *mut gc
                        as *const libc::c_void,
                    ::core::mem::size_of::<gc>() as libc::c_ulong,
                );
                return;
            }
            if strcmp(op, b"grm\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 63 as libc::c_int);
                return;
            }
            if strcmp(op, b"gsave\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                if nsave
                    >= (::core::mem::size_of::<[gc; 8]>() as libc::c_ulong)
                        .wrapping_div(::core::mem::size_of::<gc>() as libc::c_ulong)
                        as libc::c_int
                {
                    fprintf(
                        __stderrp,
                        b"svg grestore: Too many gsave's\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                let fresh7 = nsave;
                nsave = nsave + 1;
                memcpy(
                    &mut *gsave.as_mut_ptr().offset(fresh7 as isize) as *mut gc
                        as *mut libc::c_void,
                    &mut gcur as *mut gc as *const libc::c_void,
                    ::core::mem::size_of::<gc>() as libc::c_ulong,
                );
                gcur.font_n = strdup(gcur.font_n);
                gcur.font_n_old = strdup(gcur.font_n_old);
                return;
            }
            if strcmp(op, b"gsl\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                let mut a1_1: libc::c_float = 0.;
                let mut a2_1: libc::c_float = 0.;
                let mut a3_0: libc::c_float = 0.;
                let mut a4_0: libc::c_float = 0.;
                let mut a5_0: libc::c_float = 0.;
                let mut a6_0: libc::c_float = 0.;
                let mut m1_0: libc::c_float = 0.;
                let mut m2_0: libc::c_float = 0.;
                setg(1 as libc::c_int);
                m2_0 = gcur.yoffs - pop_free_val();
                m1_0 = gcur.xoffs + pop_free_val();
                a6_0 = pop_free_val();
                a5_0 = pop_free_val();
                a4_0 = pop_free_val();
                a3_0 = pop_free_val();
                a2_1 = pop_free_val();
                a1_1 = pop_free_val();
                fprintf(
                    fout,
                    b"<path class=\"stroke\"\n\td=\"M%.2f %.2fc%.2f %.2f %.2f %.2f %.2f %.2f\"/>\n\0"
                        as *const u8 as *const libc::c_char,
                    m1_0 as libc::c_double,
                    m2_0 as libc::c_double,
                    a1_1 as libc::c_double,
                    -a2_1 as libc::c_double,
                    a3_0 as libc::c_double,
                    -a4_0 as libc::c_double,
                    a5_0 as libc::c_double,
                    -a6_0 as libc::c_double,
                );
                return;
            }
            if strcmp(op, b"gxshow\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                show('x' as i32 as libc::c_char);
                return;
            }
            current_block_2019 = 18125716024132132232;
        }
        72 => {
            if strcmp(op, b"Hd\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setxysym(op, 12 as libc::c_int);
                return;
            }
            if strcmp(op, b"HD\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setxysym(op, 13 as libc::c_int);
                return;
            }
            if strcmp(op, b"HDD\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setxysym(op, 14 as libc::c_int);
                return;
            }
            current_block_2019 = 18125716024132132232;
        }
        104 => {
            if strcmp(op, b"hd\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setxysym(op, 11 as libc::c_int);
                return;
            }
            if strcmp(op, b"hl\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 52 as libc::c_int);
                return;
            }
            if strcmp(op, b"hl1\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 53 as libc::c_int);
                return;
            }
            if strcmp(op, b"hl2\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 54 as libc::c_int);
                return;
            }
            if strcmp(op, b"hld\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 67 as libc::c_int);
                return;
            }
            if strcmp(op, b"hyph\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                let mut d: libc::c_int = 0;
                setg(1 as libc::c_int);
                y = pop_free_val();
                x = pop_free_val();
                w = pop_free_val();
                d = 25 as libc::c_int
                    + w as libc::c_int / 20 as libc::c_int * 3 as libc::c_int;
                n = ((w as libc::c_double - 15.0f64) / d as libc::c_double)
                    as libc::c_int;
                x
                    += (w - (d * n) as libc::c_float - 5 as libc::c_int as libc::c_float)
                        / 2 as libc::c_int as libc::c_float;
                fprintf(
                    fout,
                    b"<path class=\"stroke\" stroke-width=\"1.2\"\n\tstroke-dasharray=\"5,%d\"\n\td=\"M%.2f %.2fh%d\"/>\n\0"
                        as *const u8 as *const libc::c_char,
                    d - 5 as libc::c_int,
                    (gcur.xoffs + x) as libc::c_double,
                    (gcur.yoffs - y) as libc::c_double
                        - gcur.font_s as libc::c_double * 0.3f64,
                    d * n + 5 as libc::c_int,
                );
                return;
            }
            current_block_2019 = 18125716024132132232;
        }
        105 => {
            if strcmp(op, b"idiv\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                n = pop_free_val() as libc::c_int;
                if stack.is_null()
                    || (*stack).type_0 as libc::c_int != VAL as libc::c_int
                    || n == 0 as libc::c_int
                {
                    fprintf(
                        __stderrp,
                        b"svg idiv: Bad value\n\0" as *const u8 as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                n = (*stack).u.v as libc::c_int / n;
                (*stack).u.v = n as libc::c_float;
                return;
            }
            if strcmp(op, b"if\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                e = pop(SEQ as libc::c_int);
                if e.is_null() {
                    return;
                }
                n = pop_free_val() as libc::c_int;
                if n != 0 as libc::c_int {
                    seq_exec(e);
                }
                elt_free(e);
                return;
            }
            if strcmp(op, b"ifelse\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                e2 = pop(SEQ as libc::c_int);
                e = pop(SEQ as libc::c_int);
                if e.is_null() || e2.is_null() {
                    return;
                }
                n = pop_free_val() as libc::c_int;
                if n != 0 as libc::c_int {
                    seq_exec(e);
                } else {
                    seq_exec(e2);
                }
                elt_free(e);
                elt_free(e2);
                return;
            }
            if strcmp(op, b"imsig\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 50 as libc::c_int);
                return;
            }
            if strcmp(op, b"iMsig\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 51 as libc::c_int);
                return;
            }
            if strcmp(op, b"index\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                n = pop_free_val() as libc::c_int;
                e = stack;
                loop {
                    n -= 1;
                    if !(n >= 0 as libc::c_int) {
                        break;
                    }
                    if e.is_null() {
                        break;
                    }
                    e = (*e).next;
                }
                if e.is_null() {
                    fprintf(
                        __stderrp,
                        b"svg index: Stack empty\n\0" as *const u8 as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                e = elt_dup(e);
                if e.is_null() {
                    return;
                }
                push(e);
                return;
            }
            current_block_2019 = 18125716024132132232;
        }
        106 => {
            if strcmp(op, b"jshow\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                show('j' as i32 as libc::c_char);
                return;
            }
            current_block_2019 = 18125716024132132232;
        }
        76 => {
            if strcmp(op, b"L\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            {
                current_block_2019 = 7375030214633881222;
            } else {
                current_block_2019 = 10963369432928671529;
            }
        }
        108 => {
            current_block_2019 = 10963369432928671529;
        }
        77 => {
            if strcmp(op, b"M\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            {
                current_block_2019 = 3045894779153661330;
            } else {
                current_block_2019 = 18125716024132132232;
            }
        }
        109 => {
            if strcmp(op, b"marcato\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 88 as libc::c_int);
                return;
            }
            if strcmp(op, b"moveto\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                current_block_2019 = 3045894779153661330;
            } else {
                if strcmp(op, b"mphr\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    xysym(op, 75 as libc::c_int);
                    return;
                }
                if strcmp(op, b"mod\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    x = pop_free_val();
                    if stack.is_null()
                        || (*stack).type_0 as libc::c_int != VAL as libc::c_int
                        || x == 0 as libc::c_int as libc::c_float
                    {
                        fprintf(
                            __stderrp,
                            b"svg: Bad value for mod\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        ps_error = 1 as libc::c_int;
                        return;
                    }
                    n = (*stack).u.v as libc::c_int % x as libc::c_int;
                    (*stack).u.v = n as libc::c_float;
                    return;
                }
                if strcmp(op, b"mrep\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    xysym(op, 58 as libc::c_int);
                    return;
                }
                if strcmp(op, b"mrep2\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    xysym(op, 59 as libc::c_int);
                    return;
                }
                if strcmp(op, b"mrest\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    xysym(op, 29 as libc::c_int);
                    return;
                }
                if strcmp(op, b"mul\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    x = pop_free_val();
                    if stack.is_null()
                        || (*stack).type_0 as libc::c_int != VAL as libc::c_int
                    {
                        fprintf(
                            __stderrp,
                            b"svg: Bad value for mul\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        ps_error = 1 as libc::c_int;
                        return;
                    }
                    (*stack).u.v *= x;
                    return;
                }
                current_block_2019 = 18125716024132132232;
            }
        }
        110 => {
            if strcmp(op, b"ne\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                cond(1 as libc::c_int);
                return;
            }
            if strcmp(op, b"neg\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                if stack.is_null()
                    || (*stack).type_0 as libc::c_int != VAL as libc::c_int
                {
                    fprintf(
                        __stderrp,
                        b"svg: Bad value for neg\n\0" as *const u8 as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                (*stack).u.v = -(*stack).u.v;
                return;
            }
            if strcmp(op, b"newpath\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                gcur.cx = ((2 as libc::c_int) << 22 as libc::c_int) as libc::c_float;
                return;
            }
            if strcmp(op, b"nt0\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 37 as libc::c_int);
                return;
            }
            current_block_2019 = 18125716024132132232;
        }
        111 => {
            if strcmp(op, b"o8va\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setg(1 as libc::c_int);
                y = gcur.yoffs - pop_free_val() - 5 as libc::c_int as libc::c_float;
                x = gcur.xoffs + pop_free_val();
                w = pop_free_val();
                sym = ps_sym_lookup(
                    b"defl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                if (*(*sym).e).u.v as libc::c_int & 1 as libc::c_int == 0 {
                    fprintf(
                        fout,
                        b"<text x=\"%.2f\" y=\"%.2f\" style=\"font:italic bold 12px serif\">8<tspan dy=\"-4\" style=\"font-size:10px\">va</tspan></text>\n\0"
                            as *const u8 as *const libc::c_char,
                        (x - 5 as libc::c_int as libc::c_float) as libc::c_double,
                        y as libc::c_double,
                    );
                    x += 14 as libc::c_int as libc::c_float;
                    w -= 14 as libc::c_int as libc::c_float;
                } else {
                    w -= 5 as libc::c_int as libc::c_float;
                }
                y -= 6 as libc::c_int as libc::c_float;
                fprintf(
                    fout,
                    b"<path class=\"stroke\" stroke-dasharray=\"6,6\" d=\"M%.2f %.2fh%.2f\"/>\n\0"
                        as *const u8 as *const libc::c_char,
                    x as libc::c_double,
                    y as libc::c_double,
                    w as libc::c_double,
                );
                if (*(*sym).e).u.v as libc::c_int & 2 as libc::c_int == 0 {
                    fprintf(
                        fout,
                        b"<path class=\"stroke\" d=\"m%.2f %.2fv6\"/>\n\0" as *const u8
                            as *const libc::c_char,
                        (x + w) as libc::c_double,
                        y as libc::c_double,
                    );
                }
                return;
            }
            if strcmp(op, b"o8vb\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setg(1 as libc::c_int);
                y = gcur.yoffs - pop_free_val() - 5 as libc::c_int as libc::c_float;
                x = gcur.xoffs + pop_free_val();
                w = pop_free_val();
                sym = ps_sym_lookup(
                    b"defl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                if (*(*sym).e).u.v as libc::c_int & 1 as libc::c_int == 0 {
                    fprintf(
                        fout,
                        b"<text x=\"%.2f\" y=\"%.2f\" style=\"font:italic bold 12px serif\">8<tspan dy=\"-4\" style=\"font-size:10px\">vb</tspan></text>\n\0"
                            as *const u8 as *const libc::c_char,
                        (x - 5 as libc::c_int as libc::c_float) as libc::c_double,
                        y as libc::c_double,
                    );
                    x += 8 as libc::c_int as libc::c_float;
                    w -= 8 as libc::c_int as libc::c_float;
                } else {
                    w -= 5 as libc::c_int as libc::c_float;
                }
                fprintf(
                    fout,
                    b"<path class=\"stroke\" stroke-dasharray=\"6,6\" d=\"M%.2f %.2fh%.2f\"/>\n\0"
                        as *const u8 as *const libc::c_char,
                    x as libc::c_double,
                    y as libc::c_double,
                    w as libc::c_double,
                );
                if (*(*sym).e).u.v as libc::c_int & 2 as libc::c_int == 0 {
                    fprintf(
                        fout,
                        b"<path class=\"stroke\" d=\"m%.2f %.2fv-6\"/>\n\0" as *const u8
                            as *const libc::c_char,
                        (x + w) as libc::c_double,
                        y as libc::c_double,
                    );
                }
                return;
            }
            if strcmp(op, b"oct\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setg(1 as libc::c_int);
                y = gcur.yoffs - pop_free_val();
                x = gcur.xoffs + pop_free_val();
                fprintf(
                    fout,
                    b"<text style=\"font:12px serif\"\n\tx=\"%.2f\" y=\"%.2f\">8</text>\n\0"
                        as *const u8 as *const libc::c_char,
                    x as libc::c_double,
                    y as libc::c_double,
                );
                return;
            }
            if strcmp(op, b"opend\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 77 as libc::c_int);
                return;
            }
            if strcmp(op, b"or\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                x = pop_free_val();
                if stack.is_null()
                    || (*stack).type_0 as libc::c_int != VAL as libc::c_int
                {
                    fprintf(
                        __stderrp,
                        b"svg or: Bad value\n\0" as *const u8 as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                (*stack)
                    .u
                    .v = (x as libc::c_int & (*stack).u.v as libc::c_int)
                    as libc::c_float;
                return;
            }
            current_block_2019 = 18125716024132132232;
        }
        112 => {
            if strcmp(op, b"pclef\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 10 as libc::c_int);
                return;
            }
            if strcmp(op, b"ped\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 89 as libc::c_int);
                return;
            }
            if strcmp(op, b"pedoff\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 90 as libc::c_int);
                return;
            }
            if strcmp(op, b"pf\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setg(1 as libc::c_int);
                y = gcur.yoffs - pop_free_val() - 5 as libc::c_int as libc::c_float;
                x = gcur.xoffs + pop_free_val();
                s = pop_free_str();
                if s.is_null() {
                    fprintf(
                        __stderrp,
                        b"svg pf: No string\n\0" as *const u8 as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                fprintf(
                    fout,
                    b"<text style=\"font:bold italic 16px serif\"\n\tx=\"%.2f\" y=\"%.2f\">%s</text>\n\0"
                        as *const u8 as *const libc::c_char,
                    x as libc::c_double,
                    y as libc::c_double,
                    s.offset(1 as libc::c_int as isize),
                );
                free(s as *mut libc::c_void);
                return;
            }
            if strcmp(op, b"pmsig\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 48 as libc::c_int);
                return;
            }
            if strcmp(op, b"pMsig\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 49 as libc::c_int);
                return;
            }
            if strcmp(op, b"pop\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                if stack.is_null() {
                    fprintf(
                        __stderrp,
                        b"svg pop: Stack empty\n\0" as *const u8 as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                e = pop((*stack).type_0 as libc::c_int);
                elt_free(e);
                return;
            }
            if strcmp(op, b"pshhd\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setxysym(op, 44 as libc::c_int);
                return;
            }
            if strcmp(op, b"pdshhd\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setxysym(
                    b"pshhd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    44 as libc::c_int,
                );
                return;
            }
            if strcmp(op, b"pfthd\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setxysym(op, 45 as libc::c_int);
                return;
            }
            if strcmp(op, b"pdfthd\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setxysym(
                    b"pfthd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    45 as libc::c_int,
                );
                return;
            }
            current_block_2019 = 18125716024132132232;
        }
        82 => {
            if strcmp(op, b"RC\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                c1_0 = 0.;
                c2_0 = 0.;
                c3_0 = 0.;
                c4_0 = 0.;
                current_block_2019 = 1622577410548098949;
            } else if strcmp(op, b"RL\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                current_block_2019 = 1919975960644342517;
            } else if strcmp(op, b"RM\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                current_block_2019 = 15885057193543236896;
            } else {
                current_block_2019 = 18125716024132132232;
            }
        }
        114 => {
            if strcmp(op, b"r00\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setxysym(op, 18 as libc::c_int);
                return;
            }
            if strcmp(op, b"r0\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setxysym(op, 19 as libc::c_int);
                return;
            }
            if strcmp(op, b"r1\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setxysym(op, 20 as libc::c_int);
                return;
            }
            if strcmp(op, b"r2\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setxysym(op, 21 as libc::c_int);
                return;
            }
            if strcmp(op, b"r4\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setxysym(op, 22 as libc::c_int);
                return;
            }
            if strcmp(op, b"r8\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setxysym(op, 24 as libc::c_int);
                return;
            }
            if strcmp(op, b"r16\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setxysym(op, 25 as libc::c_int);
                return;
            }
            if strcmp(op, b"r32\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setxysym(op, 26 as libc::c_int);
                return;
            }
            if strcmp(op, b"r64\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setxysym(op, 27 as libc::c_int);
                return;
            }
            if strcmp(op, b"r128\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setxysym(op, 28 as libc::c_int);
                return;
            }
            if strcmp(op, b"rdots\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 56 as libc::c_int);
                return;
            }
            if strcmp(op, b"rcurveto\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                current_block_2019 = 1622577410548098949;
            } else if strcmp(op, b"rlineto\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                current_block_2019 = 1919975960644342517;
            } else if strcmp(op, b"rmoveto\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                current_block_2019 = 15885057193543236896;
            } else {
                if strcmp(op, b"roll\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    let mut i: libc::c_int = 0;
                    let mut j: libc::c_int = 0;
                    j = pop_free_val() as libc::c_int;
                    n = pop_free_val() as libc::c_int;
                    if n <= 0 as libc::c_int {
                        fprintf(
                            __stderrp,
                            b"svg roll: Invalid value\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        ps_error = 1 as libc::c_int;
                        return;
                    }
                    if j > 0 as libc::c_int {
                        j = j % n;
                        if j > n / 2 as libc::c_int {
                            j -= n;
                        }
                    } else if j < 0 as libc::c_int {
                        j = -(-j % n);
                        if j < -n / 2 as libc::c_int {
                            j += n;
                        }
                    }
                    if j == 0 as libc::c_int {
                        return;
                    }
                    e2 = stack;
                    i = n;
                    loop {
                        if e2.is_null() {
                            fprintf(
                                __stderrp,
                                b"svg roll: Stack empty\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                            ps_error = 1 as libc::c_int;
                            return;
                        }
                        i -= 1;
                        if i <= 0 as libc::c_int {
                            break;
                        }
                        e2 = (*e2).next;
                    }
                    if j > 0 as libc::c_int {
                        loop {
                            let fresh8 = j;
                            j = j - 1;
                            if !(fresh8 > 0 as libc::c_int) {
                                break;
                            }
                            e = stack;
                            stack = (*e).next;
                            (*e).next = (*e2).next;
                            (*e2).next = e;
                            e2 = e;
                        }
                        return;
                    }
                    loop {
                        let fresh9 = j;
                        j = j + 1;
                        if !(fresh9 < 0 as libc::c_int) {
                            break;
                        }
                        e = stack;
                        i = 0 as libc::c_int;
                        while i < n - 2 as libc::c_int {
                            e = (*e).next;
                            i += 1;
                            i;
                        }
                        e2 = (*e).next;
                        (*e).next = (*e2).next;
                        (*e2).next = stack;
                        stack = e2;
                    }
                    return;
                }
                if strcmp(op, b"repbra\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    let mut i_0: libc::c_int = 0;
                    setg(1 as libc::c_int);
                    y = gcur.yoffs - pop_free_val();
                    x = gcur.xoffs + pop_free_val();
                    w = pop_free_val();
                    i_0 = pop_free_val() as libc::c_int;
                    h = pop_free_val();
                    s = pop_free_str();
                    if s.is_null() {
                        fprintf(
                            __stderrp,
                            b"svg repbra: No string\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        ps_error = 1 as libc::c_int;
                        return;
                    }
                    fprintf(
                        fout,
                        b"<text x=\"%.2f\" y=\"%.2f\">\0" as *const u8
                            as *const libc::c_char,
                        (x + 4 as libc::c_int as libc::c_float) as libc::c_double,
                        (y - h) as libc::c_double,
                    );
                    xml_str_out(s.offset(1 as libc::c_int as isize));
                    fprintf(
                        fout,
                        b"</text>\n<path class=\"stroke\"\n\td=\"M%.2f %.2f\0"
                            as *const u8 as *const libc::c_char,
                        x as libc::c_double,
                        y as libc::c_double,
                    );
                    if i_0 & 1 as libc::c_int != 0 {
                        fprintf(
                            fout,
                            b"m0 20v-20\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    fprintf(
                        fout,
                        b"h%.2f\0" as *const u8 as *const libc::c_char,
                        w as libc::c_double,
                    );
                    if i_0 & 2 as libc::c_int != 0 {
                        fprintf(fout, b"v20\0" as *const u8 as *const libc::c_char);
                    }
                    fprintf(fout, b"\"/>\n\0" as *const u8 as *const libc::c_char);
                    free(s as *mut libc::c_void);
                    return;
                }
                if strcmp(op, b"repeat\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    e = pop(SEQ as libc::c_int);
                    if e.is_null() {
                        return;
                    }
                    n = pop_free_val() as libc::c_int;
                    if n as libc::c_uint >= 100 as libc::c_int as libc::c_uint {
                        fprintf(
                            __stderrp,
                            b"svg repeat: Too high value\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        ps_error = 1 as libc::c_int;
                    }
                    loop {
                        n -= 1;
                        if !(n >= 0 as libc::c_int) {
                            break;
                        }
                        if seq_exec(e) != 0 {
                            break;
                        }
                        if ps_error != 0 {
                            break;
                        }
                    }
                    elt_free(e);
                    return;
                }
                if strcmp(op, b"rotate\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    let mut x_0: libc::c_float = 0.;
                    let mut y_0: libc::c_float = 0.;
                    let mut _sin: libc::c_float = 0.;
                    let mut _cos: libc::c_float = 0.;
                    setg(0 as libc::c_int);
                    x_0 = gcur.xoffs;
                    y_0 = -gcur.yoffs;
                    _sin = gcur.sin;
                    _cos = gcur.cos;
                    gcur.xoffs = x_0 * _cos + y_0 * _sin;
                    gcur.yoffs = -x_0 * _sin + y_0 * _cos;
                    x_0 = gcur.cx * _cos + gcur.cy * _sin;
                    y_0 = -gcur.cx * _sin + gcur.cy * _cos;
                    gcur.rotate -= pop_free_val();
                    if gcur.rotate > 180 as libc::c_int as libc::c_float {
                        gcur.rotate -= 360 as libc::c_int as libc::c_float;
                    } else if gcur.rotate <= -(180 as libc::c_int) as libc::c_float {
                        gcur.rotate += 360 as libc::c_int as libc::c_float;
                    }
                    h = (gcur.rotate as libc::c_double
                        * 3.14159265358979323846264338327950288f64
                        / 180 as libc::c_int as libc::c_double) as libc::c_float;
                    _sin = sin(h as libc::c_double) as libc::c_float;
                    gcur.sin = _sin;
                    _cos = cos(h as libc::c_double) as libc::c_float;
                    gcur.cos = _cos;
                    gcur.cx = x_0 * _cos - y_0 * _sin;
                    gcur.cy = x_0 * _sin + y_0 * _cos;
                    x_0 = gcur.xoffs;
                    y_0 = gcur.yoffs;
                    gcur.xoffs = x_0 * _cos - y_0 * _sin;
                    gcur.yoffs = -(x_0 * _sin + y_0 * _cos);
                    return;
                }
                current_block_2019 = 18125716024132132232;
            }
        }
        83 => {
            if strcmp(op, b"SL\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                let mut c1_1: libc::c_float = 0.;
                let mut c2_1: libc::c_float = 0.;
                let mut c3_1: libc::c_float = 0.;
                let mut c4_1: libc::c_float = 0.;
                let mut c5: libc::c_float = 0.;
                let mut c6: libc::c_float = 0.;
                let mut l2: libc::c_float = 0.;
                let mut a1_2: libc::c_float = 0.;
                let mut a2_2: libc::c_float = 0.;
                let mut a3_1: libc::c_float = 0.;
                let mut a4_1: libc::c_float = 0.;
                let mut a5_1: libc::c_float = 0.;
                let mut a6_1: libc::c_float = 0.;
                let mut m1_1: libc::c_float = 0.;
                let mut m2_1: libc::c_float = 0.;
                setg(1 as libc::c_int);
                m2_1 = gcur.yoffs - pop_free_val();
                m1_1 = gcur.xoffs + pop_free_val();
                a6_1 = pop_free_val();
                a5_1 = pop_free_val();
                a4_1 = pop_free_val();
                a3_1 = pop_free_val();
                a2_2 = pop_free_val();
                a1_2 = pop_free_val();
                l2 = pop_free_val();
                pop_free_val();
                c6 = pop_free_val();
                c5 = pop_free_val();
                c4_1 = pop_free_val();
                c3_1 = pop_free_val();
                c2_1 = pop_free_val();
                c1_1 = pop_free_val();
                fprintf(
                    fout,
                    b"<path class=\"fill\"\n\td=\"M%.2f %.2fc%.2f %.2f %.2f %.2f %.2f %.2f\n\tv%.2fc%.2f %.2f %.2f %.2f %.2f %.2f\"/>\n\0"
                        as *const u8 as *const libc::c_char,
                    m1_1 as libc::c_double,
                    m2_1 as libc::c_double,
                    a1_2 as libc::c_double,
                    -a2_2 as libc::c_double,
                    a3_1 as libc::c_double,
                    -a4_1 as libc::c_double,
                    a5_1 as libc::c_double,
                    -a6_1 as libc::c_double,
                    -l2 as libc::c_double,
                    c1_1 as libc::c_double,
                    -c2_1 as libc::c_double,
                    c3_1 as libc::c_double,
                    -c4_1 as libc::c_double,
                    c5 as libc::c_double,
                    -c6 as libc::c_double,
                );
                return;
            }
            if strcmp(op, b"SLW\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                gcur.linewidth = pop_free_val();
                return;
            }
            current_block_2019 = 18125716024132132232;
        }
        115 => {
            if strcmp(op, b"scale\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                y = pop_free_val();
                x = pop_free_val();
                gcur.xoffs /= x;
                gcur.yoffs /= y;
                gcur.cx /= x;
                gcur.cy /= y;
                gcur.xscale *= x;
                gcur.yscale *= y;
                return;
            }
            if strcmp(op, b"scalefont\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                gcur.font_s = pop_free_val();
                return;
            }
            if strcmp(op, b"search\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                let mut p_0: *mut libc::c_char = 0 as *mut libc::c_char;
                e = pop(STR as libc::c_int);
                e2 = pop(STR as libc::c_int);
                if e.is_null() || e2.is_null()
                    || *((*e).u.s).offset(0 as libc::c_int as isize) as libc::c_int
                        != '(' as i32
                    || *((*e2).u.s).offset(0 as libc::c_int as isize) as libc::c_int
                        != '(' as i32
                {
                    fprintf(
                        __stderrp,
                        b"svg search: No string\n\0" as *const u8 as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                p_0 = strstr(
                    &mut *((*e2).u.s).offset(1 as libc::c_int as isize),
                    &mut *((*e).u.s).offset(1 as libc::c_int as isize),
                );
                if !p_0.is_null() {
                    let mut e3_1: *mut elt_s = 0 as *mut elt_s;
                    let mut l1: libc::c_int = 0;
                    let mut l2_0: libc::c_int = 0;
                    let mut l3: libc::c_int = 0;
                    l1 = p_0.offset_from((*e2).u.s) as libc::c_long as libc::c_int;
                    l2_0 = strlen((*e).u.s) as libc::c_int;
                    l3 = (strlen((*e2).u.s))
                        .wrapping_sub(l2_0 as libc::c_ulong)
                        .wrapping_sub(l1 as libc::c_ulong)
                        .wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_int;
                    e3_1 = elt_new();
                    if e3_1.is_null() {
                        return;
                    }
                    (*e3_1).type_0 = STR as libc::c_int as libc::c_char;
                    (*e3_1).u.s = malloc(l3 as libc::c_ulong) as *mut libc::c_char;
                    *((*e3_1).u.s)
                        .offset(0 as libc::c_int as isize) = '(' as i32 as libc::c_char;
                    memcpy(
                        &mut *((*e3_1).u.s).offset(1 as libc::c_int as isize)
                            as *mut libc::c_char as *mut libc::c_void,
                        &mut *((*e2).u.s).offset((l1 + l2_0 - 2 as libc::c_int) as isize)
                            as *mut libc::c_char as *const libc::c_void,
                        (l3 - 1 as libc::c_int) as libc::c_ulong,
                    );
                    *((*e3_1).u.s)
                        .offset(
                            (l1 + l2_0 - 1 as libc::c_int) as isize,
                        ) = '\0' as i32 as libc::c_char;
                    push(e3_1);
                    push(e);
                    *((*e2).u.s).offset(l1 as isize) = '\0' as i32 as libc::c_char;
                    push(e2);
                    e = elt_new();
                    if e.is_null() {
                        return;
                    }
                    (*e).type_0 = VAL as libc::c_int as libc::c_char;
                    (*e).u.v = 1 as libc::c_int as libc::c_float;
                } else {
                    push(e2);
                    free((*e).u.s as *mut libc::c_void);
                    (*e).type_0 = VAL as libc::c_int as libc::c_char;
                    (*e).u.v = 0 as libc::c_int as libc::c_float;
                }
                push(e);
                return;
            }
            if strcmp(op, b"selectfont\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                h = pop_free_val();
                s = pop_free_str();
                if s.is_null() || *s as libc::c_int != '/' as i32 {
                    fprintf(
                        __stderrp,
                        b"svg selectfont: No / bad font\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    if !s.is_null() {
                        free(s as *mut libc::c_void);
                    }
                    ps_error = 1 as libc::c_int;
                    return;
                }
                if gcur.font_s != h || strcmp(s, gcur.font_n) != 0 as libc::c_int {
                    free(gcur.font_n_old as *mut libc::c_void);
                    gcur.font_n_old = gcur.font_n;
                    gcur.font_n = strdup(s);
                    gcur.font_s = h;
                    gold.font_n = 0 as *mut libc::c_char;
                } else {
                    free(s as *mut libc::c_void);
                }
                return;
            }
            if strcmp(op, b"sep0\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                x = pop_free_val();
                w = pop_free_val();
                fprintf(
                    fout,
                    b"<path class=\"stroke\"\n\td=\"M%.2f %.2fh%.2f\"/>\n\0" as *const u8
                        as *const libc::c_char,
                    (gcur.xoffs + x) as libc::c_double,
                    gcur.yoffs as libc::c_double,
                    w as libc::c_double,
                );
                return;
            }
            if strcmp(op, b"setdash\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                let mut p_1: *mut libc::c_char = 0 as *mut libc::c_char;
                n = pop_free_val() as libc::c_int;
                e = pop(BRK as libc::c_int);
                if e.is_null() {
                    fprintf(
                        __stderrp,
                        b"svg setdash: Bad pattern\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                e = (*e).u.e;
                if e.is_null() {
                    gcur.dash[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                    return;
                }
                p_1 = (gcur.dash).as_mut_ptr();
                if n != 0 as libc::c_int {
                    p_1 = p_1
                        .offset(
                            sprintf(
                                p_1,
                                b" stroke-dashoffset=\"%d\"\0" as *const u8
                                    as *const libc::c_char,
                                n,
                            ) as isize,
                        );
                }
                p_1 = p_1
                    .offset(
                        sprintf(
                            p_1,
                            b" stroke-dasharray=\"\0" as *const u8 as *const libc::c_char,
                        ) as isize,
                    );
                loop {
                    if (*e).type_0 as libc::c_int != VAL as libc::c_int {
                        fprintf(
                            __stderrp,
                            b"svg setdash: Bad pattern type\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        ps_error = 1 as libc::c_int;
                        return;
                    }
                    if p_1
                        >= (&mut *(gcur.dash)
                            .as_mut_ptr()
                            .offset(
                                ::core::mem::size_of::<[libc::c_char; 64]>()
                                    as libc::c_ulong as isize,
                            ) as *mut libc::c_char)
                            .offset(-(10 as libc::c_int as isize))
                    {
                        fprintf(
                            __stderrp,
                            b"svg setdash: Pattern too wide\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        ps_error = 1 as libc::c_int;
                        return;
                    }
                    p_1 = p_1
                        .offset(
                            sprintf(
                                p_1,
                                b"%d,\0" as *const u8 as *const libc::c_char,
                                (*e).u.v as libc::c_int,
                            ) as isize,
                        );
                    e = (*e).next;
                    if e.is_null() {
                        break;
                    }
                }
                p_1 = p_1.offset(-1);
                p_1;
                sprintf(p_1, b"\"\0" as *const u8 as *const libc::c_char);
                return;
            }
            if strcmp(op, b"setfont\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                return;
            }
            if strcmp(op, b"setgray\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                n = (pop_free_val() * 255 as libc::c_int as libc::c_float)
                    as libc::c_int;
                gcur.rgb = n << 16 as libc::c_int | n << 8 as libc::c_int | n;
                return;
            }
            if strcmp(op, b"setlinewidth\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                gcur.linewidth = pop_free_val();
                return;
            }
            if strcmp(op, b"sfu\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setg(1 as libc::c_int);
                h = pop_free_val();
                n = pop_free_val() as libc::c_int;
                sym = ps_sym_lookup(
                    b"x\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                x = ((gcur.xoffs + (*(*sym).e).u.v) as libc::c_double + 3.5f64)
                    as libc::c_float;
                sym = ps_sym_lookup(
                    b"y\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                y = gcur.yoffs - (*(*sym).e).u.v;
                fprintf(
                    fout,
                    b"<path d=\"M%.2f %.2fv%.2f\" class=\"stroke\"/>\n<path class=\"fill\"\n\td=\"\0"
                        as *const u8 as *const libc::c_char,
                    x as libc::c_double,
                    y as libc::c_double,
                    -h as libc::c_double,
                );
                y -= h;
                if n == 1 as libc::c_int {
                    fprintf(
                        fout,
                        b"M%.2f %.2fc0.6 5.6 9.6 9 5.6 18.4\n\t1.6 -6 -1.3 -11.6 -5.6 -12.8\n\0"
                            as *const u8 as *const libc::c_char,
                        x as libc::c_double,
                        y as libc::c_double,
                    );
                } else {
                    loop {
                        n -= 1;
                        if !(n >= 0 as libc::c_int) {
                            break;
                        }
                        fprintf(
                            fout,
                            b"M%.2f %.2fc0.9 3.7 9.1 6.4 6 12.4\n\t1 -5.4 -4.2 -8.4 -6 -8.4\n\0"
                                as *const u8 as *const libc::c_char,
                            x as libc::c_double,
                            y as libc::c_double,
                        );
                        y = (y as libc::c_double + 5.4f64) as libc::c_float;
                    }
                }
                fprintf(fout, b"\"/>\n\0" as *const u8 as *const libc::c_char);
                return;
            }
            if strcmp(op, b"sfd\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setg(1 as libc::c_int);
                h = pop_free_val();
                n = pop_free_val() as libc::c_int;
                sym = ps_sym_lookup(
                    b"x\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                x = ((gcur.xoffs + (*(*sym).e).u.v) as libc::c_double - 3.5f64)
                    as libc::c_float;
                sym = ps_sym_lookup(
                    b"y\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                y = gcur.yoffs - (*(*sym).e).u.v;
                fprintf(
                    fout,
                    b"<path d=\"M%.2f %.2fv%.2f\" class=\"stroke\"/>\n<path class=\"fill\"\n\td=\"\0"
                        as *const u8 as *const libc::c_char,
                    x as libc::c_double,
                    y as libc::c_double,
                    -h as libc::c_double,
                );
                y -= h;
                if n == 1 as libc::c_int {
                    fprintf(
                        fout,
                        b"M%.2f %.2fc0.6 -5.6 9.6 -9 5.6 -18.4\n\t1.6 6 -1.3 11.6 -5.6 12.8\n\0"
                            as *const u8 as *const libc::c_char,
                        x as libc::c_double,
                        y as libc::c_double,
                    );
                } else {
                    loop {
                        n -= 1;
                        if !(n >= 0 as libc::c_int) {
                            break;
                        }
                        fprintf(
                            fout,
                            b"M%.2f %.2fc0.9 -3.7 9.1 -6.4 6 -12.4\n\t1 5.4 -4.2 8.4 -6 8.4\n\0"
                                as *const u8 as *const libc::c_char,
                            x as libc::c_double,
                            y as libc::c_double,
                        );
                        y = (y as libc::c_double - 5.4f64) as libc::c_float;
                    }
                }
                fprintf(fout, b"\"/>\n\0" as *const u8 as *const libc::c_char);
                return;
            }
            if strcmp(op, b"sfs\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setg(1 as libc::c_int);
                h = pop_free_val();
                n = pop_free_val() as libc::c_int;
                sym = ps_sym_lookup(
                    b"x\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                x = gcur.xoffs + (*(*sym).e).u.v;
                sym = ps_sym_lookup(
                    b"y\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                y = gcur.yoffs - (*(*sym).e).u.v - 1 as libc::c_int as libc::c_float;
                if h > 0 as libc::c_int as libc::c_float {
                    x = (x as libc::c_double + 3.5f64) as libc::c_float;
                    y -= 1 as libc::c_int as libc::c_float;
                    fprintf(
                        fout,
                        b"<path d=\"M%.2f %.2fv%.2f\" class=\"stroke\"/>\n<path class=\"fill\"\n\td=\"\0"
                            as *const u8 as *const libc::c_char,
                        x as libc::c_double,
                        y as libc::c_double,
                        (-h + 1 as libc::c_int as libc::c_float) as libc::c_double,
                    );
                    y -= h - 1 as libc::c_int as libc::c_float;
                    loop {
                        n -= 1;
                        if !(n >= 0 as libc::c_int) {
                            break;
                        }
                        fprintf(
                            fout,
                            b"M%.2f %.2fl7 3.2 0 3.2 -7 -3.2z\n\0" as *const u8
                                as *const libc::c_char,
                            x as libc::c_double,
                            y as libc::c_double,
                        );
                        y = (y as libc::c_double + 5.4f64) as libc::c_float;
                    }
                } else {
                    x = (x as libc::c_double - 3.5f64) as libc::c_float;
                    y += 1 as libc::c_int as libc::c_float;
                    fprintf(
                        fout,
                        b"<path d=\"M%.2f %.2fv%.2f\" class=\"stroke\"/>\n<path class=\"fill\"\n\td=\"\0"
                            as *const u8 as *const libc::c_char,
                        x as libc::c_double,
                        y as libc::c_double,
                        (-h - 1 as libc::c_int as libc::c_float) as libc::c_double,
                    );
                    y -= h + 1 as libc::c_int as libc::c_float;
                    loop {
                        n -= 1;
                        if !(n >= 0 as libc::c_int) {
                            break;
                        }
                        fprintf(
                            fout,
                            b"M%.2f %.2fl7 -3.2 0 -3.2 -7 3.2z\n\0" as *const u8
                                as *const libc::c_char,
                            x as libc::c_double,
                            y as libc::c_double,
                        );
                        y = (y as libc::c_double - 5.4f64) as libc::c_float;
                    }
                }
                fprintf(fout, b"\"/>\n\0" as *const u8 as *const libc::c_char);
                return;
            }
            if strcmp(op, b"sgu\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setg(1 as libc::c_int);
                h = pop_free_val();
                n = pop_free_val() as libc::c_int;
                sym = ps_sym_lookup(
                    b"x\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                x = ((gcur.xoffs + (*(*sym).e).u.v) as libc::c_double + 2.3f64)
                    as libc::c_float;
                sym = ps_sym_lookup(
                    b"y\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                y = gcur.yoffs - (*(*sym).e).u.v;
                fprintf(
                    fout,
                    b"<path d=\"M%.2f %.2fv%.2f\" class=\"stroke\"/>\n<path class=\"fill\"\n\td=\"\0"
                        as *const u8 as *const libc::c_char,
                    x as libc::c_double,
                    y as libc::c_double,
                    -h as libc::c_double,
                );
                y -= h;
                if n == 1 as libc::c_int {
                    fprintf(
                        fout,
                        b"M%.2f %.2fc0.6 3.4 5.6 3.8 3 10\n\t1.2 -4.4 -1.4 -7 -3 -7\n\0"
                            as *const u8 as *const libc::c_char,
                        x as libc::c_double,
                        y as libc::c_double,
                    );
                } else {
                    loop {
                        n -= 1;
                        if !(n >= 0 as libc::c_int) {
                            break;
                        }
                        fprintf(
                            fout,
                            b"M%.2f %.2fc1 3.2 5.6 2.8 3.2 8\n\t1.4 -4.8 -2.4 -5.4 -3.2 -5.2\n\0"
                                as *const u8 as *const libc::c_char,
                            x as libc::c_double,
                            y as libc::c_double,
                        );
                        y = (y as libc::c_double + 3.5f64) as libc::c_float;
                    }
                }
                fprintf(fout, b"\"/>\n\0" as *const u8 as *const libc::c_char);
                return;
            }
            if strcmp(op, b"sgd\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setg(1 as libc::c_int);
                h = pop_free_val();
                n = pop_free_val() as libc::c_int;
                sym = ps_sym_lookup(
                    b"x\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                x = ((gcur.xoffs + (*(*sym).e).u.v) as libc::c_double - 2.3f64)
                    as libc::c_float;
                sym = ps_sym_lookup(
                    b"y\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                y = gcur.yoffs - (*(*sym).e).u.v;
                fprintf(
                    fout,
                    b"<path d=\"M%.2f %.2fv%.2f\" class=\"stroke\"/>\n<path class=\"fill\"\n\td=\"\0"
                        as *const u8 as *const libc::c_char,
                    x as libc::c_double,
                    y as libc::c_double,
                    -h as libc::c_double,
                );
                y -= h;
                if n == 1 as libc::c_int {
                    fprintf(
                        fout,
                        b"M%.2f %.2fc0.6 -3.4 5.6 -3.8 3 -10\n\t1.2 4.4 -1.4 7 -3 7\n\0"
                            as *const u8 as *const libc::c_char,
                        x as libc::c_double,
                        y as libc::c_double,
                    );
                } else {
                    loop {
                        n -= 1;
                        if !(n >= 0 as libc::c_int) {
                            break;
                        }
                        fprintf(
                            fout,
                            b"M%.2f %.2fc1 -3.2 5.6 -2.8 3.2 -8\n\t1.4 4.8 -2.4 5.4 -3.2 5.2\n\0"
                                as *const u8 as *const libc::c_char,
                            x as libc::c_double,
                            y as libc::c_double,
                        );
                        y = (y as libc::c_double - 3.5f64) as libc::c_float;
                    }
                }
                fprintf(fout, b"\"/>\n\0" as *const u8 as *const libc::c_char);
                return;
            }
            if strcmp(op, b"sgs\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setg(1 as libc::c_int);
                h = pop_free_val();
                n = pop_free_val() as libc::c_int;
                sym = ps_sym_lookup(
                    b"x\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                x = ((gcur.xoffs + (*(*sym).e).u.v) as libc::c_double + 2.3f64)
                    as libc::c_float;
                sym = ps_sym_lookup(
                    b"y\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                y = gcur.yoffs - (*(*sym).e).u.v;
                fprintf(
                    fout,
                    b"<path d=\"M%.2f %.2fv%.2f\" class=\"stroke\"/>\n<path class=\"fill\"\n\td=\"\0"
                        as *const u8 as *const libc::c_char,
                    x as libc::c_double,
                    y as libc::c_double,
                    -h as libc::c_double,
                );
                y -= h;
                loop {
                    n -= 1;
                    if !(n >= 0 as libc::c_int) {
                        break;
                    }
                    fprintf(
                        fout,
                        b"M%.2f %.2fl3 1.5 0 2 -3 -1.5z\n\0" as *const u8
                            as *const libc::c_char,
                        x as libc::c_double,
                        y as libc::c_double,
                    );
                    y += 3 as libc::c_int as libc::c_float;
                }
                fprintf(fout, b"\"/>\n\0" as *const u8 as *const libc::c_char);
                return;
            }
            if strcmp(op, b"sfz\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 86 as libc::c_int);
                s = pop_free_str();
                if !s.is_null() {
                    free(s as *mut libc::c_void);
                }
                return;
            }
            if strcmp(op, b"sgno\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 71 as libc::c_int);
                return;
            }
            if strcmp(op, b"show\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                show('s' as i32 as libc::c_char);
                return;
            }
            if strcmp(op, b"showb\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                show('b' as i32 as libc::c_char);
                return;
            }
            if strcmp(op, b"showc\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                show('c' as i32 as libc::c_char);
                return;
            }
            if strcmp(op, b"showr\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                show('r' as i32 as libc::c_char);
                return;
            }
            if strcmp(op, b"showerror\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 85 as libc::c_int);
                return;
            }
            if strcmp(op, b"sld\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 65 as libc::c_int);
                return;
            }
            if strcmp(op, b"snap\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 78 as libc::c_int);
                return;
            }
            if strcmp(op, b"sphr\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 76 as libc::c_int);
                return;
            }
            if strcmp(op, b"spclef\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op.offset(1 as libc::c_int as isize), 10 as libc::c_int);
                return;
            }
            if strcmp(op, b"setrgbcolor\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                let mut rgb: libc::c_int = 0;
                rgb = (pop_free_val() * 255 as libc::c_int as libc::c_float)
                    as libc::c_int;
                rgb
                    += ((pop_free_val() * 255 as libc::c_int as libc::c_float)
                        as libc::c_int) << 8 as libc::c_int;
                rgb
                    += ((pop_free_val() * 255 as libc::c_int as libc::c_float)
                        as libc::c_int) << 16 as libc::c_int;
                gcur.rgb = rgb;
                return;
            }
            if strcmp(op, b"stc\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 64 as libc::c_int);
                return;
            }
            if strcmp(op, b"stroke\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                if path.is_null() {
                    fprintf(
                        __stderrp,
                        b"svg: 'stroke' with no path\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    return;
                }
                path_end();
                fprintf(
                    fout,
                    b"\t\" class=\"stroke\"%s/>\n\0" as *const u8 as *const libc::c_char,
                    (gcur.dash).as_mut_ptr(),
                );
                return;
            }
            if strcmp(op, b"su\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
                || strcmp(op, b"sd\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                stem(op);
                return;
            }
            if strcmp(op, b"stsig\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setg(1 as libc::c_int);
                y = gcur.yoffs - pop_free_val();
                x = gcur.xoffs + pop_free_val();
                s = pop_free_str();
                if s.is_null() {
                    fprintf(
                        __stderrp,
                        b"svg: No string\n\0" as *const u8 as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                fprintf(
                    fout,
                    b"<g style=\"font:bold 18px serif\"\n\ttransform=\"translate(%.2f,%.2f) scale(1.2,1)\">\n\t<text y=\"-7\" text-anchor=\"middle\">%s</text>\n</g>\n\0"
                        as *const u8 as *const libc::c_char,
                    x as libc::c_double,
                    y as libc::c_double,
                    s.offset(1 as libc::c_int as isize),
                );
                free(s as *mut libc::c_void);
                return;
            }
            if strcmp(op, b"sub\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                x = pop_free_val();
                if stack.is_null()
                    || (*stack).type_0 as libc::c_int != VAL as libc::c_int
                {
                    fprintf(
                        __stderrp,
                        b"svg: Bad value for sub\n\0" as *const u8 as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                (*stack).u.v -= x;
                return;
            }
            if strcmp(op, b"sbclef\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 6 as libc::c_int);
                return;
            }
            if strcmp(op, b"scclef\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 9 as libc::c_int);
                return;
            }
            if strcmp(op, b"sh0\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 35 as libc::c_int);
                return;
            }
            if strcmp(op, b"sh1\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 40 as libc::c_int);
                return;
            }
            if strcmp(op, b"sh4\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                n = pop_free_val() as libc::c_int;
                match n {
                    1 => {
                        xysym(
                            b"sh1\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            40 as libc::c_int,
                        );
                    }
                    2 => {
                        xysym(
                            b"sh0\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            35 as libc::c_int,
                        );
                    }
                    3 => {
                        xysym(
                            b"sh513\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            41 as libc::c_int,
                        );
                    }
                    _ => {
                        xysym(
                            b"dsh0\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            38 as libc::c_int,
                        );
                    }
                }
                return;
            }
            if strcmp(op, b"sh513\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 41 as libc::c_int);
                return;
            }
            if strcmp(op, b"srep\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 57 as libc::c_int);
                return;
            }
            if strcmp(op, b"stclef\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 3 as libc::c_int);
                return;
            }
            if strcmp(op, b"stringwidth\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                s = pop_free_str();
                if s.is_null() || *s as libc::c_int != '(' as i32 {
                    fprintf(
                        __stderrp,
                        b"svg stringwidth: No string\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                e = elt_new();
                if e.is_null() {
                    return;
                }
                (*e).type_0 = VAL as libc::c_int as libc::c_char;
                (*e).u.v = strw(s.offset(1 as libc::c_int as isize));
                push(e);
                e = elt_new();
                if e.is_null() {
                    return;
                }
                (*e).type_0 = VAL as libc::c_int as libc::c_char;
                (*e).u.v = gcur.font_s;
                push(e);
                return;
            }
            if strcmp(op, b"svg\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                e = elt_new();
                if e.is_null() {
                    return;
                }
                (*e).type_0 = VAL as libc::c_int as libc::c_char;
                (*e).u.v = 1 as libc::c_int as libc::c_float;
                push(e);
                return;
            }
            current_block_2019 = 18125716024132132232;
        }
        84 => {
            if strcmp(op, b"T\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            {
                current_block_2019 = 15740281602351198340;
            } else {
                current_block_2019 = 18125716024132132232;
            }
        }
        116 => {
            if strcmp(op, b"tclef\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 2 as libc::c_int);
                return;
            }
            if strcmp(op, b"thbar\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setg(1 as libc::c_int);
                y = gcur.yoffs - pop_free_val();
                x = ((gcur.xoffs + pop_free_val()) as libc::c_double + 1.5f64)
                    as libc::c_float;
                h = pop_free_val();
                fprintf(
                    fout,
                    b"<path class=\"stroke\" stroke-width=\"3\"\n\td=\"M%.2f %.2fv%.2f\"/>\n\0"
                        as *const u8 as *const libc::c_char,
                    x as libc::c_double,
                    y as libc::c_double,
                    -h as libc::c_double,
                );
                return;
            }
            if strcmp(op, b"thumb\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 79 as libc::c_int);
                return;
            }
            if strcmp(op, b"translate\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                current_block_2019 = 15740281602351198340;
            } else {
                if strcmp(op, b"trem\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    setg(1 as libc::c_int);
                    y = gcur.yoffs - pop_free_val();
                    x = ((gcur.xoffs + pop_free_val()) as libc::c_double - 4.5f64)
                        as libc::c_float;
                    n = pop_free_val() as libc::c_int;
                    fprintf(
                        fout,
                        b"<path class=\"fill\" d=\"m%.2f %.2f\n\t\0" as *const u8
                            as *const libc::c_char,
                        x as libc::c_double,
                        y as libc::c_double,
                    );
                    loop {
                        fputs(
                            b"l9 -3v3l-9 3z\0" as *const u8 as *const libc::c_char,
                            fout,
                        );
                        n -= 1;
                        if n <= 0 as libc::c_int {
                            break;
                        }
                        fputs(b"m0 5.4\0" as *const u8 as *const libc::c_char, fout);
                    }
                    fputs(b"\"/>\0" as *const u8 as *const libc::c_char, fout);
                    return;
                }
                if strcmp(op, b"trl\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    xysym(op, 87 as libc::c_int);
                    return;
                }
                if strcmp(op, b"true\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    e = elt_new();
                    if e.is_null() {
                        return;
                    }
                    (*e).type_0 = VAL as libc::c_int as libc::c_char;
                    (*e).u.v = 1 as libc::c_int as libc::c_float;
                    push(e);
                    return;
                }
                if strcmp(op, b"tsig\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    let mut d_0: *mut libc::c_char = 0 as *mut libc::c_char;
                    setg(1 as libc::c_int);
                    y = ((gcur.yoffs - pop_free_val()) as libc::c_double - 0.5f64)
                        as libc::c_float;
                    x = gcur.xoffs + pop_free_val();
                    d_0 = pop_free_str();
                    s = pop_free_str();
                    if d_0.is_null() || s.is_null() {
                        fprintf(
                            __stderrp,
                            b"svg: No string\n\0" as *const u8 as *const libc::c_char,
                        );
                        if !d_0.is_null() {
                            free(d_0 as *mut libc::c_void);
                        }
                        if !s.is_null() {
                            free(s as *mut libc::c_void);
                        }
                        ps_error = 1 as libc::c_int;
                        return;
                    }
                    fprintf(
                        fout,
                        b"<g style=\"font:bold 16px serif\"\n\ttransform=\"translate(%.2f,%.2f) scale(1.2,1)\">\n\t<text text-anchor=\"middle\">%s</text>\n\t<text y=\"-12\" text-anchor=\"middle\">%s</text>\n</g>\n\0"
                            as *const u8 as *const libc::c_char,
                        x as libc::c_double,
                        y as libc::c_double,
                        d_0.offset(1 as libc::c_int as isize),
                        s.offset(1 as libc::c_int as isize),
                    );
                    free(d_0 as *mut libc::c_void);
                    free(s as *mut libc::c_void);
                    return;
                }
                if strcmp(op, b"tubr\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                    || strcmp(op, b"tubrl\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                {
                    let mut dx_0: libc::c_float = 0.;
                    let mut dy_0: libc::c_float = 0.;
                    let mut h_0: libc::c_int = 0;
                    setg(1 as libc::c_int);
                    y = gcur.yoffs - pop_free_val();
                    x = gcur.xoffs + pop_free_val();
                    dy_0 = pop_free_val();
                    dx_0 = pop_free_val();
                    if *op.offset(4 as libc::c_int as isize) as libc::c_int == 'l' as i32
                    {
                        h_0 = 3 as libc::c_int;
                        y -= 3 as libc::c_int as libc::c_float;
                    } else {
                        h_0 = -(3 as libc::c_int);
                        y += 3 as libc::c_int as libc::c_float;
                    }
                    fprintf(
                        fout,
                        b"<path class=\"stroke\"\n\td=\"M%.2f %.2fv%dl%.2f %.2fv%d\"/>\n\0"
                            as *const u8 as *const libc::c_char,
                        x as libc::c_double,
                        y as libc::c_double,
                        h_0,
                        dx_0 as libc::c_double,
                        -dy_0 as libc::c_double,
                        -h_0,
                    );
                    return;
                }
                if strcmp(op, b"turn\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    xysym(op, 80 as libc::c_int);
                    return;
                }
                if strcmp(op, b"turnx\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    xysym(op, 81 as libc::c_int);
                    return;
                }
                current_block_2019 = 18125716024132132232;
            }
        }
        117 => {
            if strcmp(op, b"upb\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 69 as libc::c_int);
                return;
            }
            if strcmp(op, b"umrd\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 61 as libc::c_int);
                return;
            }
            current_block_2019 = 18125716024132132232;
        }
        119 => {
            if strcmp(op, b"wedge\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                xysym(op, 82 as libc::c_int);
                return;
            }
            if strcmp(op, b"wln\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                setg(1 as libc::c_int);
                y = pop_free_val();
                x = pop_free_val();
                w = pop_free_val();
                fprintf(
                    fout,
                    b"<path class=\"stroke\" stroke-width=\"0.8\"\n\td=\"M%.2f %.2fh%.2f\"/>\n\0"
                        as *const u8 as *const libc::c_char,
                    (gcur.xoffs + x) as libc::c_double,
                    (gcur.yoffs - y) as libc::c_double,
                    w as libc::c_double,
                );
                return;
            }
            if strcmp(op, b"where\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                s = pop_free_str();
                if s.is_null() || *s as libc::c_int != '/' as i32 {
                    fprintf(
                        __stderrp,
                        b"svg where: No / bad symbol\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    if !s.is_null() {
                        free(s as *mut libc::c_void);
                    }
                    ps_error = 1 as libc::c_int;
                    return;
                }
                e = elt_new();
                if e.is_null() {
                    return;
                }
                (*e).type_0 = VAL as libc::c_int as libc::c_char;
                sym = ps_sym_lookup(&mut *s.offset(1 as libc::c_int as isize));
                if sym.is_null() {
                    (*e).u.v = 0 as libc::c_int as libc::c_float;
                } else {
                    (*e).u.v = 1 as libc::c_int as libc::c_float;
                    e2 = elt_new();
                    if e2.is_null() {
                        return;
                    }
                    (*e2).type_0 = VAL as libc::c_int as libc::c_char;
                    (*e2).u.v = 0 as libc::c_int as libc::c_float;
                    push(e2);
                }
                free(s as *mut libc::c_void);
                push(e);
                return;
            }
            current_block_2019 = 18125716024132132232;
        }
        120 => {
            if strcmp(op, b"xydef\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                y = pop_free_val();
                x = pop_free_val();
                setxory(
                    b"x\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    x,
                );
                setxory(
                    b"y\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    y,
                );
                return;
            }
            if strcmp(op, b"xymove\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                gcur.cy = pop_free_val();
                gcur.cx = pop_free_val();
                setxory(
                    b"x\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    gcur.cx,
                );
                setxory(
                    b"y\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    gcur.cy,
                );
                return;
            }
            current_block_2019 = 18125716024132132232;
        }
        _ => {
            current_block_2019 = 18125716024132132232;
        }
    }
    match current_block_2019 {
        15740281602351198340 => {
            y = pop_free_val();
            x = pop_free_val();
            gcur.xoffs += x;
            gcur.yoffs -= y;
            gcur.cx -= x;
            gcur.cy -= y;
            return;
        }
        15885057193543236896 => {
            y = pop_free_val();
            x = pop_free_val();
            if !path.is_null() {
                path_print(
                    b"\tm%.2f %.2f\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    x as libc::c_double,
                    -y as libc::c_double,
                );
            } else if g == 2 as libc::c_int {
                fputs(b"</text>\n\0" as *const u8 as *const libc::c_char, fout);
                g = 1 as libc::c_int;
            }
            gcur.cx += x;
            gcur.cy += y;
            return;
        }
        1919975960644342517 => {
            path_def();
            y = pop_free_val();
            x = pop_free_val();
            if x == 0 as libc::c_int as libc::c_float {
                path_print(
                    b"\tv%.2f\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    -y as libc::c_double,
                );
            } else if y == 0 as libc::c_int as libc::c_float {
                path_print(
                    b"\th%.2f\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    x as libc::c_double,
                );
            } else {
                path_print(
                    b"\tl%.2f %.2f\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    x as libc::c_double,
                    -y as libc::c_double,
                );
            }
            gcur.cx += x;
            gcur.cy += y;
            return;
        }
        1622577410548098949 => {
            path_def();
            y = pop_free_val();
            x = pop_free_val();
            c4_0 = pop_free_val();
            c3_0 = pop_free_val();
            c2_0 = pop_free_val();
            c1_0 = pop_free_val();
            path_print(
                b"\tc%.2f %.2f %.2f %.2f %.2f %.2f\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                c1_0 as libc::c_double,
                -c2_0 as libc::c_double,
                c3_0 as libc::c_double,
                -c4_0 as libc::c_double,
                x as libc::c_double,
                -y as libc::c_double,
            );
            gcur.cx += x;
            gcur.cy += y;
            return;
        }
        3045894779153661330 => {
            gcur.cy = pop_free_val();
            gcur.cx = pop_free_val();
            if !path.is_null() {
                path_print(
                    b"\tM%.2f %.2f\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (gcur.xoffs + gcur.cx) as libc::c_double,
                    (gcur.yoffs - gcur.cy) as libc::c_double,
                );
            } else if g == 2 as libc::c_int {
                fputs(b"</text>\n\0" as *const u8 as *const libc::c_char, fout);
                g = 1 as libc::c_int;
            }
            return;
        }
        10963369432928671529 => {
            if strcmp(op, b"le\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                cond(5 as libc::c_int);
                return;
            }
            if strcmp(op, b"lt\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                cond(4 as libc::c_int);
                return;
            }
            if strcmp(op, b"length\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                s = pop_free_str();
                if s.is_null() || *s as libc::c_int != '(' as i32 {
                    fprintf(
                        __stderrp,
                        b"svg length: No string\n\0" as *const u8 as *const libc::c_char,
                    );
                    if !s.is_null() {
                        free(s as *mut libc::c_void);
                    }
                    ps_error = 1 as libc::c_int;
                    return;
                }
                e = elt_new();
                if e.is_null() {
                    return;
                }
                (*e).type_0 = VAL as libc::c_int as libc::c_char;
                (*e).u.v = strlen(s.offset(1 as libc::c_int as isize)) as libc::c_float;
                push(e);
                free(s as *mut libc::c_void);
                return;
            }
            if strcmp(op, b"lineto\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                current_block_2019 = 7375030214633881222;
            } else {
                if strcmp(op, b"lmrd\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    xysym(op, 62 as libc::c_int);
                    return;
                }
                if strcmp(op, b"load\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    s = pop_free_str();
                    if s.is_null() || *s as libc::c_int != '/' as i32 {
                        fprintf(
                            __stderrp,
                            b"svg load: No / bad symbol\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        if !s.is_null() {
                            free(s as *mut libc::c_void);
                        }
                        ps_error = 1 as libc::c_int;
                        return;
                    }
                    sym = ps_sym_lookup(s.offset(1 as libc::c_int as isize));
                    if sym.is_null() {
                        e = elt_new();
                        if e.is_null() {
                            return;
                        }
                        (*e).type_0 = STR as libc::c_int as libc::c_char;
                        (*e).u.s = strdup(s);
                        *((*e).u.s)
                            .offset(
                                0 as libc::c_int as isize,
                            ) = ' ' as i32 as libc::c_char;
                    } else {
                        e = elt_dup((*sym).e);
                        if e.is_null() {
                            return;
                        }
                    }
                    free(s as *mut libc::c_void);
                    push(e);
                    return;
                }
                if strcmp(op, b"longa\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    setxysym(op, 16 as libc::c_int);
                    return;
                }
                if strcmp(op, b"lphr\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    xysym(op, 74 as libc::c_int);
                    return;
                }
                if strcmp(op, b"ltr\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    arp_ltr('l' as i32 as libc::c_char);
                    return;
                }
                if strcmp(op, b"lyshow\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    show('s' as i32 as libc::c_char);
                    return;
                }
                current_block_2019 = 18125716024132132232;
            }
        }
        45961622177256480 => {
            path_def();
            y = pop_free_val();
            x = pop_free_val();
            c4 = gcur.yoffs - pop_free_val();
            c3 = gcur.xoffs + pop_free_val();
            c2 = gcur.yoffs - pop_free_val();
            c1 = gcur.xoffs + pop_free_val();
            path_print(
                b"\tC%.2f %.2f %.2f %.2f %.2f %.2f\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                c1 as libc::c_double,
                c2 as libc::c_double,
                c3 as libc::c_double,
                c4 as libc::c_double,
                (gcur.xoffs + x) as libc::c_double,
                (gcur.yoffs - y) as libc::c_double,
            );
            gcur.cx = x;
            gcur.cy = y;
            return;
        }
        _ => {}
    }
    match current_block_2019 {
        18125716024132132232 => {}
        _ => {
            path_def();
            y = pop_free_val();
            x = pop_free_val();
            if x == gcur.cx {
                path_print(
                    b"\tv%.2f\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (gcur.cy - y) as libc::c_double,
                );
            } else if y == gcur.cy {
                path_print(
                    b"\th%.2f\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (x - gcur.cx) as libc::c_double,
                );
            } else {
                path_print(
                    b"\tl%.2f %.2f\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (x - gcur.cx) as libc::c_double,
                    (gcur.cy - y) as libc::c_double,
                );
            }
            gcur.cx = x;
            gcur.cy = y;
            return;
        }
    }
    if !defs.is_null() {
        s = strstr(defs, op);
        if !s.is_null()
            && *s.offset(-(1 as libc::c_int) as isize) as libc::c_int == '"' as i32
            && *s.offset(strlen(op) as isize) as libc::c_int == '"' as i32
        {
            xysym(op, -(1 as libc::c_int));
            return;
        }
    }
    fprintf(
        __stderrp,
        b"svg: Symbol '%s' not defined\n\0" as *const u8 as *const libc::c_char,
        op,
    );
    ps_error = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn svg_write(mut buf: *mut libc::c_char, mut len: libc::c_int) {
    let mut l: libc::c_int = 0;
    let mut e: *mut elt_s = 0 as *mut elt_s;
    let mut e2: *mut elt_s = 0 as *mut elt_s;
    let mut c: libc::c_uchar = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut q: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut r: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if ps_error != 0 {
        return;
    }
    p = buf as *mut libc::c_uchar;
    loop {
        len -= 1;
        if !(len >= 0 as libc::c_int) {
            break;
        }
        let fresh10 = p;
        p = p.offset(1);
        c = *fresh10;
        match c as libc::c_int {
            32 | 9 | 10 => {}
            123 | 91 => {
                e = elt_new();
                if e.is_null() {
                    return;
                }
                in_cnt += 1;
                in_cnt;
                (*e).type_0 = STR as libc::c_int as libc::c_char;
                (*e)
                    .u
                    .s = strdup(
                    if c as libc::c_int == '{' as i32 {
                        b"{\0" as *const u8 as *const libc::c_char
                    } else {
                        b"[\0" as *const u8 as *const libc::c_char
                    },
                );
                push(e);
            }
            125 | 93 => {
                in_cnt -= 1;
                in_cnt;
                if in_cnt < 0 as libc::c_int {
                    if c as libc::c_int == '}' as i32 {
                        fprintf(
                            __stderrp,
                            b"svg: '}' without '{'\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else {
                        fprintf(
                            __stderrp,
                            b"svg: ']' without '['\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    ps_error = 1 as libc::c_int;
                    return;
                }
                e = elt_new();
                if e.is_null() {
                    return;
                }
                (*e).u.e = 0 as *mut elt_s;
                if c as libc::c_int == '}' as i32 {
                    (*e).type_0 = SEQ as libc::c_int as libc::c_char;
                    c = '{' as i32 as libc::c_uchar;
                } else {
                    (*e).type_0 = BRK as libc::c_int as libc::c_char;
                    c = '[' as i32 as libc::c_uchar;
                }
                loop {
                    e2 = stack;
                    stack = (*stack).next;
                    if (*e2).type_0 as libc::c_int == STR as libc::c_int
                        && (*((*e2).u.s).offset(0 as libc::c_int as isize) as libc::c_int
                            == '[' as i32
                            || *((*e2).u.s).offset(0 as libc::c_int as isize)
                                as libc::c_int == '{' as i32)
                    {
                        break;
                    }
                    (*e2).next = (*e).u.e;
                    (*e).u.e = e2;
                }
                if *((*e2).u.s).offset(0 as libc::c_int as isize) as libc::c_int
                    != c as libc::c_int
                {
                    fprintf(
                        __stderrp,
                        b"svg: '%c' found before '%c'\n\0" as *const u8
                            as *const libc::c_char,
                        *((*e2).u.s).offset(0 as libc::c_int as isize) as libc::c_int,
                        c as libc::c_int,
                    );
                    ps_error = 1 as libc::c_int;
                    return;
                }
                elt_free(e2);
                push(e);
            }
            37 => {
                q = p;
                loop {
                    len -= 1;
                    if !(len >= 0 as libc::c_int) {
                        break;
                    }
                    let fresh11 = p;
                    p = p.offset(1);
                    c = *fresh11;
                    if c as libc::c_int == '\n' as i32 {
                        break;
                    }
                }
                if q as *mut libc::c_char
                    != &mut *buf.offset(1 as libc::c_int as isize) as *mut libc::c_char
                    && *q.offset(-(2 as libc::c_int) as isize) as libc::c_int
                        != '\n' as i32
                {
                    continue;
                }
                if strncmp(
                    q as *mut libc::c_char,
                    b"A \0" as *const u8 as *const libc::c_char,
                    2 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    let mut type_0: libc::c_char = 0;
                    let mut row: libc::c_int = 0;
                    let mut col: libc::c_int = 0;
                    let mut h: libc::c_int = 0;
                    let mut x: libc::c_float = 0.;
                    let mut y: libc::c_float = 0.;
                    let mut w: libc::c_float = 0.;
                    q = q.offset(2 as libc::c_int as isize);
                    let fresh12 = q;
                    q = q.offset(1);
                    type_0 = *fresh12 as libc::c_char;
                    if type_0 as libc::c_int != 'b' as i32
                        && type_0 as libc::c_int != 'e' as i32
                    {
                        sscanf(
                            (q as *mut libc::c_char).offset(1 as libc::c_int as isize),
                            b"%d %d %f %f %f %d\0" as *const u8 as *const libc::c_char,
                            &mut row as *mut libc::c_int,
                            &mut col as *mut libc::c_int,
                            &mut x as *mut libc::c_float,
                            &mut y as *mut libc::c_float,
                            &mut w as *mut libc::c_float,
                            &mut h as *mut libc::c_int,
                        );
                    } else {
                        sscanf(
                            (q as *mut libc::c_char).offset(1 as libc::c_int as isize),
                            b"%d %d %f %f\0" as *const u8 as *const libc::c_char,
                            &mut row as *mut libc::c_int,
                            &mut col as *mut libc::c_int,
                            &mut x as *mut libc::c_float,
                            &mut y as *mut libc::c_float,
                        );
                        h = 6 as libc::c_int;
                        w = h as libc::c_float;
                    }
                    fprintf(
                        fout,
                        b"<abc type=\"%c\" row=\"%d\" col=\"%d\" x=\"%.2f\" y=\"%.2f\" width=\"%.2f\" height=\"%d\"/>\n\0"
                            as *const u8 as *const libc::c_char,
                        type_0 as libc::c_int,
                        row,
                        col,
                        (gcur.xoffs + x) as libc::c_double,
                        (gcur.yoffs - y - h as libc::c_float) as libc::c_double,
                        w as libc::c_double,
                        h,
                    );
                } else {
                    if !(strncmp(
                        q as *mut libc::c_char,
                        b" --- title\0" as *const u8 as *const libc::c_char,
                        10 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int)
                    {
                        continue;
                    }
                    if strstr(
                        (q as *mut libc::c_char).offset(10 as libc::c_int as isize),
                        b"--\0" as *const u8 as *const libc::c_char,
                    ) < p as *mut libc::c_char
                    {
                        continue;
                    }
                    setg(1 as libc::c_int);
                    if *q.offset(10 as libc::c_int as isize) as libc::c_int == 's' as i32
                    {
                        q = q.offset(14 as libc::c_int as isize);
                        fprintf(
                            fout,
                            b"<!-- subtitle: %.*s -->\n\0" as *const u8
                                as *const libc::c_char,
                            (p.offset_from(q) as libc::c_long
                                - 1 as libc::c_int as libc::c_long) as libc::c_int,
                            q,
                        );
                    } else {
                        q = q.offset(11 as libc::c_int as isize);
                        fprintf(
                            fout,
                            b"<!-- title: %.*s -->\n\0" as *const u8
                                as *const libc::c_char,
                            (p.offset_from(q) as libc::c_long
                                - 1 as libc::c_int as libc::c_long) as libc::c_int,
                            q,
                        );
                    }
                }
            }
            40 => {
                q = p.offset(-(1 as libc::c_int as isize));
                l = 1 as libc::c_int;
                loop {
                    let fresh13 = p;
                    p = p.offset(1);
                    match *fresh13 as libc::c_int {
                        92 => {}
                        41 => {
                            break;
                        }
                        _ => {
                            continue;
                        }
                    }
                    p = p.offset(1);
                    p;
                    l -= 1;
                    l;
                }
                len = (len as libc::c_long
                    - (p.offset_from(q) as libc::c_long
                        - 1 as libc::c_int as libc::c_long)) as libc::c_int;
                l = (l as libc::c_long
                    + (p.offset_from(q) as libc::c_long
                        - 1 as libc::c_int as libc::c_long)) as libc::c_int;
                p = q;
                e = elt_new();
                if e.is_null() {
                    return;
                }
                (*e).type_0 = STR as libc::c_int as libc::c_char;
                r = malloc(l as libc::c_ulong) as *mut libc::c_uchar;
                (*e).u.s = r as *mut libc::c_char;
                loop {
                    let fresh14 = p;
                    p = p.offset(1);
                    c = *fresh14;
                    match c as libc::c_int {
                        92 => {
                            let fresh15 = p;
                            p = p.offset(1);
                            let fresh16 = r;
                            r = r.offset(1);
                            *fresh16 = *fresh15;
                        }
                        41 => {
                            break;
                        }
                        _ => {
                            let fresh17 = r;
                            r = r.offset(1);
                            *fresh17 = c;
                        }
                    }
                }
                *r = '\0' as i32 as libc::c_uchar;
                push(e);
            }
            _ => {
                q = p.offset(-(1 as libc::c_int as isize));
                loop {
                    len -= 1;
                    if !(len >= 0 as libc::c_int) {
                        break;
                    }
                    let fresh18 = p;
                    p = p.offset(1);
                    c = *fresh18;
                    match c as libc::c_int {
                        40 | 32 | 9 | 10 | 123 | 125 | 91 | 93 | 37 | 47 => {
                            break;
                        }
                        _ => {}
                    }
                }
                if len >= 0 as libc::c_int {
                    p = p.offset(-1);
                    p;
                    len += 1;
                    len;
                }
                if isdigit(*q as libc::c_uint as libc::c_int) != 0
                    || *q as libc::c_int == '-' as i32 || *q as libc::c_int == '.' as i32
                {
                    let mut i: libc::c_int = 0;
                    let mut v: libc::c_float = 0.;
                    e = elt_new();
                    if e.is_null() {
                        return;
                    }
                    (*e).type_0 = VAL as libc::c_int as libc::c_char;
                    c = *p;
                    *p = '\0' as i32 as libc::c_uchar;
                    if *q.offset(1 as libc::c_int as isize) as libc::c_int == '#' as i32
                    {
                        i = strtol(
                            (q as *mut libc::c_char).offset(2 as libc::c_int as isize),
                            0 as *mut *mut libc::c_char,
                            8 as libc::c_int,
                        ) as libc::c_int;
                        (*e).u.v = i as libc::c_float;
                    } else if *q.offset(2 as libc::c_int as isize) as libc::c_int
                        == '#' as i32
                    {
                        i = strtol(
                            (q as *mut libc::c_char).offset(3 as libc::c_int as isize),
                            0 as *mut *mut libc::c_char,
                            16 as libc::c_int,
                        ) as libc::c_int;
                        (*e).u.v = i as libc::c_float;
                    } else {
                        if sscanf(
                            q as *mut libc::c_char,
                            b"%f\0" as *const u8 as *const libc::c_char,
                            &mut v as *mut libc::c_float,
                        ) != 1 as libc::c_int
                        {
                            fprintf(
                                __stderrp,
                                b"svg: Bad numeric value in '%s'\n\0" as *const u8
                                    as *const libc::c_char,
                                buf,
                            );
                            v = 0 as libc::c_int as libc::c_float;
                        }
                        (*e).u.v = v;
                    }
                    *p = c;
                } else {
                    if in_cnt == 0 {
                        if *q as libc::c_int != '/' as i32 {
                            c = *p;
                            *p = '\0' as i32 as libc::c_uchar;
                            ps_exec(q as *mut libc::c_char);
                            if ps_error != 0 {
                                return;
                            }
                            *p = c;
                            continue;
                        }
                    } else if strncmp(
                        q as *mut libc::c_char,
                        b"pdfmark\0" as *const u8 as *const libc::c_char,
                        7 as libc::c_int as libc::c_ulong,
                    ) == 0 as libc::c_int
                    {
                        in_cnt -= 1;
                        in_cnt;
                        loop {
                            e = pop((*stack).type_0 as libc::c_int);
                            if (*e).type_0 as libc::c_int == STR as libc::c_int
                                && (*((*e).u.s).offset(0 as libc::c_int as isize)
                                    as libc::c_int == '[' as i32
                                    || *((*e).u.s).offset(0 as libc::c_int as isize)
                                        as libc::c_int == '{' as i32)
                            {
                                break;
                            }
                            elt_free(e);
                        }
                        elt_free(e);
                        continue;
                    }
                    l = p.offset_from(q) as libc::c_long as libc::c_int;
                    r = malloc((l + 1 as libc::c_int) as libc::c_ulong)
                        as *mut libc::c_uchar;
                    memcpy(
                        r as *mut libc::c_void,
                        q as *const libc::c_void,
                        l as libc::c_ulong,
                    );
                    *r.offset(l as isize) = '\0' as i32 as libc::c_uchar;
                    e = elt_new();
                    if e.is_null() {
                        free(r as *mut libc::c_void);
                        return;
                    }
                    (*e).type_0 = STR as libc::c_int as libc::c_char;
                    (*e).u.s = r as *mut libc::c_char;
                }
                push(e);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn svg_output(
    mut out: *mut FILE,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut args_0: va_list = 0 as *mut libc::c_char;
    let mut tmp: [libc::c_char; 128] = [0; 128];
    va_start(args_0, fmt);
    vsnprintf(
        tmp.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        fmt,
        args_0,
    );
    va_end(args_0);
    svg_write(tmp.as_mut_ptr(), strlen(tmp.as_mut_ptr()) as libc::c_int);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn svg_close() {
    let mut e: *mut elt_s = 0 as *mut elt_s;
    let mut e2: *mut elt_s = 0 as *mut elt_s;
    setg(0 as libc::c_int);
    fputs(b"</svg>\n\0" as *const u8 as *const libc::c_char, fout);
    e = stack;
    if !e.is_null() {
        stack = 0 as *mut elt_s;
        fprintf(
            __stderrp,
            b"svg close: stack not empty \0" as *const u8 as *const libc::c_char,
        );
        elt_lst_dump(e);
        fprintf(__stderrp, b"\n\0" as *const u8 as *const libc::c_char);
        loop {
            e2 = (*e).next;
            elt_free(e);
            e = e2;
            if e.is_null() {
                break;
            }
        }
    }
}
