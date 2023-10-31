use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type __sFILEX;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(_: *const libc::c_char, _: *mut FILE) -> libc::c_int;
    static mut cfmt: FORMAT;
    static mut fout: *mut FILE;
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
static mut ps_head: [libc::c_char; 8777] = unsafe {
    *::core::mem::transmute::<
        &[u8; 8777],
        &mut [libc::c_char; 8777],
    >(
        b"/xydef{/y exch def/x exch def}!\n/xymove{/x 2 index def/y 1 index def M}!\n/showc{dup stringwidth pop .5 mul neg 0 RM show}!\n/showr{dup stringwidth pop neg 0 RM show}!\n/showb{\tdup currentpoint 3 -1 roll show\n\t.6 SLW\n\texch 2 sub exch 3 sub 3 -1 roll\n\tstringwidth pop 4 add\n\tcurrentfont/ScaleMatrix get 0 get .8 mul\n\t4 add rectstroke}!\n/oct{/Times-Roman 12 selectfont M(8)show}!\n/octu{oct}!\n/octl{oct}!\n/bm{\tM 3 copy RL neg 0 exch RL neg exch neg exch RL 0 exch RL fill}!\n/bnum{M/Times-Italic 12 selectfont showc}!\n/bnumb{\tcurrentgray/Times-Italic 12 selectfont\n\t3 index stringwidth pop 4 add\n\tdup .5 mul neg 4 index add 3 index 3 -1 roll 8\n\t1.0 setgray rectfill setgray M showc}!\n/tubr{3 sub M 0 3 RL RL 0 -3 RL dlw stroke}!\n/tubrl{3 add M 0 -3 RL RL 0 3 RL dlw stroke}!\n/dt{x y M RM currentpoint 1.2 0 360 arc fill}!\n/dnb{\tdlw M -3.2 2 RM\n\t0 7.2 RL\n\t6.4 0 RM\n\t0 -7.2 RL\n\tcurrentpoint stroke M\n\t-6.4 4.8 RM\n\t0 2.4 RL\n\t6.4 0 RL\n\t0 -2.4 RL\n\tfill}!\n/upb{\tdlw M -2.6 9.4 RM\n\t2.6 -8.8 RL\n\t2.6 8.8 RL\n\tstroke}!\n/grm{\tM -5 2.5 RM\n\t5 8.5 5.5 -4.5 10 2 RC\n\t-5 -8.5 -5.5 4.5 -10 -2 RC fill}!\n/stc{\t3 add M currentpoint 1.2 0 360 arc fill}!\n/emb{\t1.2 SLW 1 setlinecap M -2.5 3 RM 5 0 RL stroke 0 setlinecap}!\n/cpu{\tM -6 0 RM\n\t0.4 7.3 11.3 7.3 11.7 0 RC\n\t-1.3 6 -10.4 6 -11.7 0 RC fill}!\n/sld{\tM -7.2 -4.8 RM\n\t1.8 -0.7 4.5 0.2 7.2 4.8 RC\n\t-2.1 -5 -5.4 -6.8 -7.6 -6 RC fill}!\n/trl{\t/Times-BoldItalic 16 selectfont M -4 2 RM(tr)show}!\n/fng{/Bookman-Demi 8 selectfont M -3 1 RM show}!\n/dacs{/Times-Roman 16 selectfont 3 add M showc}!\n/crdc{/Times-Italic 14 selectfont 5 add M show}!\n/brth{/Times-BoldItalic 30 selectfont 6 add M(,)show}!\n/pf{/Times-BoldItalic 16 selectfont 5 add M show}!\n/sfz{\tM -7 5 RM pop\n\t/Times-Italic 14 selectfont(s)show\n\t/Times-BoldItalic 16 selectfont(f)show\n\t/Times-Italic 14 selectfont(z)show}!\n/cresc{\t1 SLW M dup 5 RM\n\tdefl 1 and 0 eq\n\t{dup neg 4 RL 4 RL}\n\t{dup neg 2.2 RL 0 3.6 RM 2.2 RL}\n\tifelse stroke}!\n/dim{\t1 SLW 5 add M\n\tdefl 2 and 0 eq\n\t{dup 4 RL neg 4 RL}\n\t{dup 2.2 RL 0 3.6 RM neg 2.2 RL}\n\tifelse stroke}!\n/o8va{\tM\n\tdefl 1 and 0 eq\n\t{14 sub currentpoint\n\t-5 0 RM /Times-BoldItalic 12 selectfont(8)show\n\t0 4 RM /Times-BoldItalic 10 selectfont(va)show\n\tM 14 0 RM}if 0 6 RM\n\t[6] 0 setdash 5 sub 0 RL currentpoint stroke [] 0 setdash\n\tM defl 2 and 0 eq\n\t{0 -6 RL stroke}if}!\n/o8vb{\tM\n\tdefl 1 and 0 eq\n\t{8 sub currentpoint\n\t-5 0 RM /Times-BoldItalic 12 selectfont(8)show\n\t0 4 RM /Times-BoldItalic 10 selectfont(vb)show\n\tM 8 0 RM}if\n\t[6] 0 setdash 5 sub 0 RL currentpoint stroke [] 0 setdash\n\tM defl 2 and 0 eq\n\t{0 6 RL stroke}if}!\n/dplus{\t1.2 SLW 0.5 add M 0 6 RL -3 -3 RM 6 0 RL stroke}!\n/turnx{\t2 copy turn .6 SLW 1.5 add M 0 9 RL stroke}!\n/lphr{1.2 SLW M 0 -18 RL stroke}!\n/mphr{1.2 SLW M 0 -12 RL stroke}!\n/sphr{1.2 SLW M 0 -6 RL stroke}!\n/ltr{\tgsave 4 add T\n\t0 6 3 -1 roll{\n\t\t2{\n\t\t\t0 0.4 M\n\t\t\t2 1.9 3.4 2.3 3.9 0 C\n\t\t\t2.1 0 L\n\t\t\t1.9 0.8 1.4 0.7 0 -0.4 C\n\t\t\tfill\n\t\t\t180 rotate -6 0 T\n\t\t}repeat\n\t\tpop 6 0 T\n\t}for\n\tgrestore}!\n/arp{gsave 90 rotate exch neg ltr grestore}!\n/gliss{\tgsave 2 copy T\nexch 4 -1 roll exch sub 3 1 roll sub\n2 copy exch atan dup rotate\nexch pop cos div\n8 0 M 14 sub 0 RL stroke grestore}!\n/glisq{\tgsave 2 copy T\nexch 4 -1 roll exch sub 3 1 roll sub\n2 copy exch atan dup rotate\nexch pop cos div\n17 sub 8 -4 ltr grestore}!\n/wedge{1 add M -1.5 5 RL 3 0 RL -1.5 -5 RL fill}!\n/opend{dlw M currentpoint 3 add 2.5 -90 270 arc stroke}!\n/snap{\tdlw 2 copy M -3 6 RM\n\t0 5 6 5 6 0 RC\n\t0 -5 -6 -5 -6 0 RC\n\t5 add M 0 -6 RL stroke}!\n/thumb{\tdlw 2 copy M -2.5 7 RM\n\t0 6 5 6 5 0 RC\n\t0 -6 -5 -6 -5 0 RC\n\t2 add M 0 -4 RL stroke}!\n/trem{\tM -4.5 0 RM{\n\t\tcurrentpoint\n\t\t9 3 RL 0 -3 RL -9 -3 RL 0 3 RL\n\t\tfill 5.4 sub M\n\t}repeat}!\n/hl{\t.8 SLW M -6 0 RM 12 0 RL stroke}!\n/hl1{\t.8 SLW M -7 0 RM 14 0 RL stroke}!\n/hl2{\t.7 SLW M -9 0 RM 18 0 RL stroke}!\n/gsc{gsave y T .8 dup scale 0 0}!\n/uflat{<95200028\n\t0064000001b802ee\n\t006402ea\n\t008402ea\n\t0084000c\n\t00640008\n\t00840154\n\t00b2019c011c01ae01540168\n\t01b800fa00dc00220084000c\n\t00840028\n\t00ba0028014c00f60106014a\n\t00d401860084014e00840128\n\t><0b00010303030a0105050105050a>}cvlit def\n/unat{<95200022\n\t003cff42013602ee\n\t006002ee\n\t004002ee\n\t00400022\n\t0060002a\n\t01160060\n\t0116ff46\n\t0136ff46\n\t01360208\n\t011401fe\n\t006001cc\n\t006002ee\n\t0060009e\n\t0060015c\n\t01160190\n\t011600d4\n\t><0b00012a030a0123030a>}cvlit def\n/usharp{<95200024\n\t003cff42019a02ee\n\t008802be\n\t0088ff44\n\t00a8ff44\n\t00a802be\n\t0128ff76\n\t0148ff76\n\t014802ee\n\t012802ee\n\t004001d0\n\t0040015c\n\t019201bc\n\t01920230\n\t00400076\n\t00400002\n\t01920064\n\t019200d6\n\t><0b000123030a0123030a0123030a0123030a>}cvlit def\n/udblesharp{<95200046\n\t003c006e019001c2\n\t00f0011a\n\t01180140013a015e018e015e\n\t018e01be\n\t012e01be\n\t012e016a0110014800ea0122\n\t00c2014800a4016a00a401be\n\t004401be\n\t0044015e\n\t009a015e00bc014000e2011a\n\t00bc00f4009a00d6004400d6\n\t00440076\n\t00a40076\n\t00a400ca00c200ec00ea0112\n\t011000ec012e00ca012e0076\n\t018e0076\n\t018e00d6\n\t013a00d6011800f400f0011a\n\t><0b0001050303050503030505030305050303050a>}cvlit def\n/udbleflat{<9520004c\n\t00140000022602ee\n\t001402ea\n\t002c02ea\n\t002c000c\n\t00140008\n\t002c0154\n\t004e019c009e01ae00c80168\n\t011300fa00660022002c000c\n\t002c0028\n\t0054002800c200f6008d014a\n\t00680186002c014e002c0128\n\t010e02ea\n\t012602ea\n\t0126000c\n\t010e0008\n\t01260154\n\t0148019c019801ae01c20168\n\t020d00fa016000220126000c\n\t01260028\n\t014e002801bc00f60187014a\n\t016201860126014e01260128\n\t><0b000123030a0105050105050a0123030a0105050105050a>}cvlit def\n/sh1{\tgsave T .9 SLW\n\t0 -7.8 M 0 15.4 RL stroke\n\t-1.8 -2.7 M 3.6 1.1 RL 0 -2.2 RL -3.6 -1.1 RL 0 2.2 RL fill\n\t-1.8 3.7 M 3.6 1.1 RL 0 -2.2 RL -3.6 -1.1 RL 0 2.2 RL fill\n\tgrestore}!\n/sh513{\tgsave T .8 SLW\n\t-2.5 -8.7 M 0 15.4 RL\n\t0 -7.8 M 0 15.4 RL\n\t2.5 -6.9 M 0 15.4 RL stroke\n\t-3.7 -3.1 M 7.4 2.2 RL 0 -2.2 RL -7.4 -2.2 RL 0 2.2 RL fill\n\t-3.7 3.2 M 7.4 2.2 RL 0 -2.2 RL -7.4 -2.2 RL 0 2.2 RL fill\n\tgrestore}!\n/ft1{gsave -1 1 scale exch neg exch ft0 grestore}!\n/ftx{\t-1.4 2.7 RM\n\t5.7 3.1 5.7 -3.6 0 -6.7 RC\n\t3.9 4 4 7.6 0 5.8 RC\n\tcurrentpoint fill 7.1 add M\n\tdlw 0 -12.4 RL stroke}!\n/ft513{2 copy gsave -1 1 scale exch neg 3 add exch M ftx grestore\n\tM 1.5 0 RM ftx}!\n/sh4tb[/.notdef/sh1/sh0/sh513/dsh0]def\n/sh4{sh4tb exch get cvx exec}!\n/ft4tb[/.notdef/ft1/ft0/ft513/dft0]def\n/ft4{ft4tb exch get cvx exec}!\n/bar{3 -1 roll 1 exch rectfill}!\n/dotbar{M 1 SLW 0 exch RL [5] 0 setdash stroke [] 0 setdash}!\n/thbar{3 -1 roll 3 exch rectfill}!\n/rdots{\t2 copy 9 add M currentpoint 1.2 0 360 arc\n\t15 add M currentpoint 1.2 0 360 arc fill}!\n/pmsig{0.3 SLW 12 add M currentpoint 5 0 360 arc stroke}!\n/pMsig{2 copy pmsig 12 add M currentpoint 1.3 0 360 arc fill}!\n/imsig{0.3 SLW 12 add 2 copy 5 add M 5 60 300 arc stroke}!\n/iMsig{2 copy imsig 12 add M currentpoint 1.3 0 360 arc fill}!\n/tsig{\t1 add M gsave/Times-Bold 16 selectfont 1.2 1 scale\n\tcurrentpoint 3 -1 roll showc\n\t12 add M showc grestore}!\n/stsig{\t7 add M gsave/Times-Bold 18 selectfont 1.2 1 scale\n\tshowc grestore}!\n/sep0{\tdlw 0 M 0 RL stroke}!\n/bracket{M -5 2 RM currentpoint\n\t-1.7 2 RM 10.5 -1 12 4.5 12 3.5 RC\n\t0 -1 -3.5 -5.5 -8.5 -5.5 RC fill\n\t3 SLW 2 add M\n\t0 exch neg 8 sub RL currentpoint stroke\n\tM -1.7 0 RM\n\t10.5 1 12 -4.5 12 -3.5 RC\n\t0 1 -3.5 5.5 -8.5 5.5 RC fill}!\n/srep{\tM -1 -6 RM 11 12 RL 3 0 RL -11 -12 RL -3 0 RL fill}!\n/repbra{gsave dlw T 0 -20 M\n\t0 20 3 index 1 and 1 eq{RL}{RM}ifelse 0\n\tRL 2 and 2 eq{0 -20 RL}if stroke\n\t4 exch M show grestore}!\n/SL{M RC RL RC closepath fill}!\n/dSL{\tM [4] 0 setdash .8 SLW RC stroke [] 0 setdash}!\n/strw{\tgsave 0 -2000 M/strop/show load def 1 setgray str\n\t0 setgray currentpoint pop/w exch def grestore}!\n/jshow{w 0 32 4 -1 roll widthshow}!\n/strop/show load def\n/arrayshow{{dup type/stringtype eq{strop}{glyphshow}ifelse}forall}def\n/gcshow{show}!\n/agcshow{arrayshow}!\n/box{.6 SLW rectstroke}!\n/boxend{currentpoint pop/x exch def}!\n/boxmark{currentpoint pop dup x gt\n\t{/x exch def}{pop}ifelse}!\n/boxdraw{x 3 index sub 2 add exch box}!\n/gxshow{0 9 3 -1 roll widthshow}!\n/anshow{show}!\n/aanshow{arrayshow}!\n/wln{M .8 SLW 0 RL stroke}!\n/hyph{\t.8 SLW 3 add M\n\tdup cvi 20 idiv 3 mul 25 add\n\t1 index cvi exch idiv 1 add exch 1 index div\n\tdup 4 sub 3 1 roll .5 mul 2 sub 0 RM\n\t{4 0 RL dup 0 RM}repeat stroke pop}!\n/lyshow{show}!\n/alyshow{arrayshow}!\n/pfthd{/x 2 index def/y 1 index def dsh0\n\t.7 SLW x y M x y 4 0 360 arc stroke}!\n/pdshhd{pshhd}!\n/pdfthd{pfthd}!\n/ghd{\txymove\n\t1.7 1.5 RM\n\t-1.32 2.31 -5.94 -0.33 -4.62 -2.64 RC\n\t1.32 -2.31 5.94 0.33 4.62 2.64 RC fill}!\n/gua{x y M -1 4 RM RL stroke}!\n/gda{x y M -5 -4 RM RL stroke}!\n/ghl{\t.6 SLW M -3.5 0 RM 7 0 RL stroke}!\n/gsl{dlw M RC stroke}!\n/custos{2 copy M -4 0 RM 2 2.5 RL 2 -2.5 RL 2 2.5 RL 2 -2.5 RL\n\t-2 -2.5 RL -2 2.5 RL -2 -2.5 RL -2 2.5 RL fill\n\tM 3.5 0 RM 5 7 RL dlw stroke}!\n/glypharray{{glyphshow}forall}!\n/showerror{gsave 1 0.7 0.7 setrgbcolor 2.5 SLW newpath\n\t30 0 360 arc stroke grestore}!\n/pdfmark where{pop}{userdict/pdfmark/cleartomark load put}ifelse\n0 setlinecap 0 setlinejoin\n\0",
    )
};
static mut psdgl: [libc::c_char; 6794] = unsafe {
    *::core::mem::transmute::<
        &[u8; 6794],
        &mut [libc::c_char; 6794],
    >(
        b"/hbrce{\t-2.5 1 RM\n\t-4.5 -4.6 -7.5 -12.2 -4.4 -26.8 RC\n\t3.5 -14.3 3.2 -21.7 -2.1 -24.2 RC\n\t7.4 2.4 7.3 14.2 3.5 29.5 RC\n\t-2.7 9.5 -1.5 16.2 3 21.5 RC\n\tfill}!\n/brace{\tgsave T 0 0 M .01 mul 1 exch scale hbrce\n\t0 -100 M 1 -1 scale hbrce grestore}!\n/sgno{\t3 add M currentpoint currentpoint currentpoint\n\t1.5 -1.7 6.4 0.3 3 3.7 RC\n\t-10.4 7.8 -8 10.6 -6.5 11.9 RC\n\t4 1.9 5.9 -1.7 4.2 -2.6 RC\n\t-1.3 -0.7 -2.9 1.3 -0.7 2 RC\n\t-1.5 1.7 -6.4 -0.3 -3 -3.7 RC\n\t10.4 -7.8 8 -10.6 6.5 -11.9 RC\n\t-4 -1.9 -5.9 1.7 -4.2 2.6 RC\n\t1.3 0.7 2.9 -1.3 0.7 -2 RC\n\tfill\n\tM 0.8 SLW -6 1.2 RM 12.6 12.6 RL stroke\n\t7 add exch 6 sub exch 1.2 0 360 arc fill\n\t8 add exch 6 add exch 1.2 0 360 arc fill}!\n/dacoda{2 1 roll 10 add 2 1 roll dup 3 -1 roll dup 3 1 roll -23 add 4 1 roll\n\t1 SLW 2 add 2 copy M 0 20 RL\n\t2 copy M -10 10 RM 20 0 RL stroke\n\t10 add 6 0 360 arc 1.7 SLW stroke\n\t/Times-Roman 16 selectfont 7 add M (Da) showc}!\n/coda{\t1 SLW 2 add 2 copy M 0 20 RL\n\t2 copy M -10 10 RM 20 0 RL 1.1 SLW stroke\n\texch -7 add exch (O) 3 1 roll /Times-Bold 18 selectfont 4 add M show}!\n/utclef{<95200072\n\t0000ff2e01c2030c\n\t00ac0056\n\t0064007f006400f400e00112\n\t0176011c01bc0056013a0012\n\t00c8ffde002700120015009a\n\t0006014f0072017f00f101e8\n\t0149023f0140026d012f02ba\n\t00fc029900d1025100d60200\n\t00e700f500fa008a0107ffc2\n\t010dff6200f4ff3c00baff3b\n\t006aff3a003cff98007dffc0\n\t00d2ffe90102ff5b009cff57\n\t00b3ff4600f8ff3200f6ffb3\n\t00ec009200cf010900c4021c\n\t00c4027600c402be01240304\n\t015c02bc0163021e013a01e3\n\t00f001790039013b003b00a7\n\t0044000e00cfffee01370022\n\t018d0063015400e200e700d2\n\t00a000c6007e008f00ac0056\n\t><0b000132050a>}cvlit def\n/tclef{gsave T -10 -6 T .045 dup scale utclef ufill grestore}!\n/ucclef{<95200066\n\t006effbe01e70256\n\t00d10108\n\t00d10002\n\t00c40002\n\t00c40213\n\t00d10213\n\t00d10113\n\t00ea012700fa013701100180\n\t011e0161011d014d0148013a\n\t01a2011801a80244011f01f3\n\t015301e0013a01a3011401a6\n\t00ba01cc01350256019f01eb\n\t01e7019c01a000fa01190131\n\t0109010a\n\t011900e4\n\t01a0011b01e70079019f002a\n\t0135ffbe00ba00490114006f\n\t013a007201530035011f0022\n\t01a8ffd101a200fd014800db\n\t011d00c8011b00bd0110009b\n\t00fa00e400ea00f400d10108\n\t006e0213\n\t00a70213\n\t00a70002\n\t006e0002\n\t006e0213\n\t><0b000125032605220326050a0124030a>}cvlit def\n/cclef{gsave T -12 -12 T .045 dup scale ucclef ufill grestore}!\n/ubclef{<95200046\n\t00000050019a0244\n\t00010057\n\t007d007a00df00a500ff0143\n\t012a022700580239003f01aa\n\t007a01fa00dc0194009b015c\n\t005d012d00280172003101b4\n\t00460241013f023c01430180\n\t014200d100d9007800010057\n\t01660151\n\t016601750199017301990151\n\t0199012c0166012d01660151\n\t016401d2\n\t016401f6019701f4019701d2\n\t019701ac016401ad016401d2\n\t><0b000126050a0122050a0122050a>}cvlit def\n/bclef{gsave T -10 -18 T .045 dup scale ubclef ufill grestore}!\n/pclef{\texch 2.7 sub exch -9 add 5.4 18 1.4 SLW rectstroke}!\n/spclef{pclef}!\n/stclef{gsave T -10 -6 T .037 dup scale utclef ufill grestore}!\n/scclef{gsave T -12 -10 T .037 dup scale ucclef ufill grestore}!\n/sbclef{gsave T -10 -15 T .037 dup scale ubclef ufill grestore}!\n/csig{\tM\n\t6 5.3 RM\n\t0.9 0 2.3 -0.7 2.4 -2.2 RC\n\t-1.2 2 -3.6 -0.1 -1.6 -1.7 RC\n\t2 -1 3.8 3.5 -0.8 4.7 RC\n\t-2 0.4 -6.4 -1.3 -5.8 -7 RC\n\t0.4 -6.4 7.9 -6.8 9.1 -0.7 RC\n\t-2.3 -5.6 -6.7 -5.1 -6.8 0 RC\n\t-0.5 4.4 0.7 7.5 3.5 6.9 RC\n\tfill}!\n/ctsig{dlw 2 copy csig M 5 -8 RM 0 16 RL stroke}!\n/HDD{\tdlw HD\n\tx y M -6 -4 RM 0 8 RL\n\t12 0 RM 0 -8 RL stroke}!\n/breve{\txymove\n\t2.5 SLW -6 -2.7 RM 12 0 RL\n\t0 5.4 RM -12 0 RL stroke\n\tdlw x y M -6 -5 RM 0 10 RL\n\t12 0 RM 0 -10 RL stroke}!\n/HD{\txymove\n\t-2.7 1.4 RM\n\t1.5 2.8 6.9 0 5.3 -2.7 RC\n\t-1.5 -2.8 -6.9 0 -5.3 2.7 RC\n\t8.3 -1.4 RM\n\t0 1.5 -2.2 3 -5.6 3 RC\n\t-3.4 0 -5.6 -1.5 -5.6 -3 RC\n\t0 -1.5 2.2 -3 5.6 -3 RC\n\t3.4 0 5.6 1.5 5.6 3 RC fill}!\n/Hd{\txymove\n\t3 1.6 RM\n\t-1 1.8 -7 -1.4 -6 -3.2 RC\n\t1 -1.8 7 1.4 6 3.2 RC\n\t0.5 0.3 RM\n\t2 -3.8 -5 -7.6 -7 -3.8 RC\n\t-2 3.8 5 7.6 7 3.8 RC fill}!\n/uhd{{\t100 -270 640 280\n\t560 82\n\t474 267 105 105 186 -80\n\t267 -265 636 -102 555 82\n\t}<0b000122050a>}cvlit def\n/hd{\t/x 2 index def/y 1 index def\n\tgsave T -7.4 0 T .02 dup scale uhd ufill grestore}!\n/ft0{\tgsave T -3.5 -3.5 T .018 dup scale uflat ufill grestore}!\n/nt0{\tgsave T -3 -5 T .018 dup scale unat ufill grestore}!\n/sh0{\tgsave T -4 -5 T .018 dup scale usharp ufill grestore}!\n/dsh0{\tgsave T -4 -5 T .018 dup scale udblesharp ufill grestore}!\n/pshhd{/x 2 index def/y 1 index def dsh0}!\n/dft0{\tgsave T -4 -3.5 T .018 dup scale udbleflat ufill grestore}!\n/accent{1.2 SLW M -4 1 RM 8 2 RL -8 2 RL stroke}!\n/marcato{M -3 0 RM 3 7 RL 3 -7 RL -1.5 0 RL -1.8 4.2 RL -1.7 -4.2 RL fill}!\n/hld{\t1.5 add 2 copy 1.5 add M currentpoint 1.3 0 360 arc\n\tM -7.5 0 RM\n\t0 11.5 15 11.5 15 0 RC\n\t-0.25 0 RL\n\t-1.25 9 -13.25 9 -14.5 0 RC\n\tfill}!\n/r00{\txymove -1.5 -6 RM currentpoint 3 12 rectfill}!\n/r0{\txymove -1.5 0 RM currentpoint 3 6 rectfill}!\n/r1{\txymove -3.5 3 RM currentpoint 7 3 rectfill}!\n/r2{\txymove -3.5 0 RM currentpoint 7 3 rectfill}!\n/r4{\txymove\n\t-1 8.5 RM\n\t3.6 -5.1 RL\n\t-2.1 -5.2 RL\n\t2.2 -4.3 RL\n\t-2.6 2.3 -5.1 0 -2.4 -2.6 RC\n\t-4.8 3 -1.5 6.9 1.4 4.1 RC\n\t-3.1 4.5 RL\n\t1.9 5.1 RL\n\t-1.5 3.5 RL\n\tfill}!\n/r8e{\t-1.5 -1.5 -2.4 -2 -3.6 -2 RC\n\t2.4 2.8 -2.8 4 -2.8 1.2 RC\n\t0 -2.7 4.3 -2.4 5.9 -0.6 RC\n\tfill}!\n/r8{\txymove\n\t.5 SLW 3.3 4 RM\n\t-3.4 -9.6 RL stroke\n\tx y M 3.4 4 RM r8e}!\n/r16{\txymove\n\t.5 SLW 3.3 4 RM\n\t-4 -15.6 RL stroke\n\tx y M 3.4 4 RM r8e\n\tx y M 1.9 -2 RM r8e}!\n/r32{\txymove\n\t.5 SLW 4.8 10 RM\n\t-5.5 -21.6 RL stroke\n\tx y M 4.9 10 RM r8e\n\tx y M 3.4 4 RM r8e\n\tx y M 1.9 -2 RM r8e}!\n/r64{\txymove\n\t.5 SLW 4.8 10 RM\n\t-7 -27.6 RL stroke\n\tx y M 4.9 10 RM r8e\n\tx y M 3.4 4 RM r8e\n\tx y M 1.9 -2 RM r8e\n\tx y M 0.4 -8 RM r8e}!\n/r128{\txymove\n\t.5 SLW 5.8 16 RM\n\t-8.5 -33.6 RL stroke\n\tx y M 5.9 16 RM r8e\n\tx y M 4.4 10 RM r8e\n\tx y M 2.9 4 RM r8e\n\tx y M 1.4 -2 RM r8e\n\tx y M -0.1 -8 RM r8e}!\n/mrest{\tM currentpoint 1 SLW\n\t-20 -6 RM 0 12 RL 40 0 RM 0 -12 RL stroke\n\tM 5 SLW -20 0 RM 40 0 RL stroke}!\n/mrep{\t2 copy 2 copy\n\tM -5 3 RM currentpoint 1.4 0 360 arc\n\tM 5 -3 RM currentpoint 1.4 0 360 arc\n\tM -7 -6 RM 11 12 RL 3 0 RL -11 -12 RL -3 0 RL fill}!\n/mrep2{\t2 copy 2 copy\n\tM -5 6 RM currentpoint 1.4 0 360 arc\n\tM 5 -6 RM currentpoint 1.4 0 360 arc fill\n\tM 1.8 SLW\n\t-7 -8 RM 14 10 RL -14 -4 RM 14 10 RL stroke}!\n/turn{\tM 5.2 8 RM\n\t1.4 -0.5 0.9 -4.8 -2.2 -2.8 RC\n\t-4.8 3.5 RL\n\t-3 2 -5.8 -1.8 -3.6 -4.4 RC\n\t1 -1.1 2 -0.8 2.1 0.1 RC\n\t0.1 0.9 -0.7 1.2 -1.9 0.6 RC\n\t-1.4 0.5 -0.9 4.8 2.2 2.8 RC\n\t4.8 -3.5 RL\n\t3 -2 5.8 1.8 3.6 4.4 RC\n\t-1 1.1 -2 0.8 -2.1 -0.1 RC\n\t-0.1 -0.9 0.7 -1.2 1.9 -0.6 RC\n\tfill}!\n/umrd{\t4 add M\n\t2.2 2.2 RL 2.1 -2.9 RL 0.7 0.7 RL\n\t-2.2 -2.2 RL -2.1 2.9 RL -0.7 -0.7 RL\n\t-2.2 -2.2 RL -2.1 2.9 RL -0.7 -0.7 RL\n\t2.2 2.2 RL 2.1 -2.9 RL 0.7 0.7 RL fill}!\n/lmrd{\t2 copy umrd M .6 SLW 0 8 RL stroke}!\n/ped{\t/Times-BoldItalic 16 selectfont M -10 2 RM(Ped)show}!\n/pedoff{\t/Times-BoldItalic 16 selectfont M -4 2 RM(*)show}!\n/longa{\txymove\n\t2.5 SLW -6 -2.7 RM 12 0 RL\n\t0 5.4 RM -12 0 RL stroke\n\tdlw x y M -6 -5 RM 0 10 RL\n\t12 0 RM 0 -16 RL stroke}!\n\0",
    )
};
static mut psfgl: [libc::c_char; 1693] = unsafe {
    *::core::mem::transmute::<
        &[u8; 1693],
        &mut [libc::c_char; 1693],
    >(
        b"/musgly{music 24 selectfont glyphshow}!\n/brace{gsave\n\tT -7.5 0 M -.042 mul 3 exch scale/uniE000 musgly\n\tgrestore}!\n/sgno{\tM -6 4 RM/uniE047 musgly}!\n/coda{\tM -12 6 RM/uniE048 musgly}!\n/tclef{M -8 0 RM/uniE050 musgly}!\n/cclef{M -8 0 RM/uniE05C musgly}!\n/bclef{M -8 0 RM/uniE062 musgly}!\n/pclef{M -6 0 RM/uniE069 musgly}!\n/stclef{M -8 0 RM/uniE07A musgly}!\n/scclef{M -8 0 RM/uniE07B musgly}!\n/sbclef{M -7 0 RM/uniE07C musgly}!\n/csig{\tM 0 0 RM/uniE08A musgly}!\n/ctsig{M 0 0 RM/uniE08B musgly}!\n/HDD{\txymove -7 0 RM/uniE0A0 musgly}!\n/breve{xymove -6 0 RM/uniE0A1 musgly}!\n/HD{\txymove -5.2 0 RM/uniE0A2 musgly}!\n/Hd{\txymove -3.8 0 RM/uniE0A3 musgly}!\n/hd{\txymove -3.7 0 RM/uniE0A4 musgly}!\n/ft0{\tM -3 0 RM/uniE260 musgly}!\n/nt0{\tM -2 0 RM/uniE261 musgly}!\n/sh0{\tM -3 0 RM/uniE262 musgly}!\n/dsh0{\tM -3 0 RM/uniE263 musgly}!\n/pshhd{xymove -3 0 RM/uniE263 musgly}!\n/dft0{\tM -3 0 RM/uniE264 musgly}!\n/accent{M -3 0 RM/uniE4A0 musgly}!\n/marcato{M -3 0 RM/uniE4AC musgly}!\n/hld{\tM -7 0 RM/uniE4C0 musgly}!\n/r00{\txymove -1.5 0 RM/uniE4E1 musgly}!\n/r0{\txymove -1.5 0 RM/uniE4E2 musgly}!\n/r1{\txymove -3.5 6 RM/uniE4E3 musgly}!\n/r2{\txymove -3.2 0 RM/uniE4E4 musgly}!\n/r4{\txymove -3 0 RM/uniE4E5 musgly}!\n/r8{\txymove -3 0 RM/uniE4E6 musgly}!\n/r16{\txymove -4 0 RM/uniE4E7 musgly}!\n/r32{\txymove -4 0 RM/uniE4E8 musgly}!\n/r64{\txymove -4 0 RM/uniE4E9 musgly}!\n/r128{\txymove -4 0 RM/uniE4EA musgly}!\n/mrest{M -10 0 RM/uniE4EE musgly}!\n/mrep{\tM -6 0 RM/uniE500 musgly}!\n/mrep2{M -9 0 RM/uniE501 musgly}!\n/turn{\tM -4 0 RM/uniE567 musgly}!\n/umrd{\tM -7 2 RM/uniE56C musgly}!\n/lmrd{\tM -7 2 RM/uniE56D musgly}!\n/ped{\tM -10 0 RM/uniE650 musgly}!\n/pedoff{M -6 0 RM/uniE655 musgly}!\n/longa{xymove -6 0 RM/uniE95C musgly}!\n\0",
    )
};
#[no_mangle]
pub unsafe extern "C" fn define_font(
    mut name: *mut libc::c_char,
    mut num: libc::c_int,
    mut enc: libc::c_int,
) {
    if enc == 0 as libc::c_int {
        fprintf(
            fout,
            b"/%s-utf8/%s mkfont\n/F%d{/%s-utf8 exch selectfont}!\n\0" as *const u8
                as *const libc::c_char,
            name,
            name,
            num,
            name,
        );
    } else {
        fprintf(
            fout,
            b"/F%d{/%s exch selectfont}!\n\0" as *const u8 as *const libc::c_char,
            num,
            name,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn define_symbols() {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    p = cfmt.musicfont;
    fputs(ps_head.as_mut_ptr(), fout);
    fputs(if !p.is_null() { psfgl.as_mut_ptr() } else { psdgl.as_mut_ptr() }, fout);
    if !p.is_null() {
        q = strchr(p, '(' as i32);
        if !q.is_null() {
            q = q.offset(1);
            q;
            r = strrchr(q, '.' as i32);
            if r.is_null() {
                r = q.offset(strlen(p) as isize).offset(-(1 as libc::c_int as isize));
            }
            p = strrchr(q, '/' as i32);
            if !p.is_null() {
                q = p.offset(1 as libc::c_int as isize);
            }
            fprintf(
                fout,
                b"/music/%.*s def\n\0" as *const u8 as *const libc::c_char,
                r.offset_from(q) as libc::c_long as libc::c_int,
                q,
            );
        } else {
            fprintf(fout, b"/music/%s def\n\0" as *const u8 as *const libc::c_char, p);
        }
    }
    fprintf(
        fout,
        b"/su{dlw x y M %.1f %.1f RM %.1f sub 0 exch RL stroke}!\n\0" as *const u8
            as *const libc::c_char,
        3.5f64,
        1.0f64,
        1.0f64,
    );
    fprintf(
        fout,
        b"/sd{dlw x y M %.1f %.1f RM %.1f add 0 exch RL stroke}!\n\0" as *const u8
            as *const libc::c_char,
        -3.5f64,
        -1.0f64,
        1.0f64,
    );
    fprintf(
        fout,
        b"/sfu{\tdlw x y M %.1f %.1f RM\n\t%.1f sub 0 exch RL currentpoint stroke\n\tM dup 1 eq{\n\t\tpop\n\t\t0.6 -5.6 9.6 -9 5.6 -18.4 RC\n\t\t1.6 6 -1.3 11.6 -5.6 12.8 RC fill\n\t  }{\n\t\t1 sub{\tcurrentpoint\n\t\t\t0.9 -3.7 9.1 -6.4 6 -12.4 RC\n\t\t\t1 5.4 -4.2 8.4 -6 8.4 RC\n\t\t\tfill 5.4 sub M\n\t\t}repeat\n\t\t1.2 -3.2 9.6 -5.7 5.6 -14.6 RC\n\t\t1.6 5.4 -1 10.2 -5.6 11.4 RC fill\n\t  }ifelse}!\n\0"
            as *const u8 as *const libc::c_char,
        3.5f64,
        1.0f64,
        1.0f64,
    );
    fprintf(
        fout,
        b"/sfd{\tdlw x y M %.1f %.1f RM\n\t%.1f add 0 exch RL currentpoint stroke\n\tM dup 1 eq{\n\t\tpop\n\t\t0.6 5.6 9.6 9 5.6 18.4 RC\n\t\t1.6 -6 -1.3 -11.6 -5.6 -12.8 RC fill\n\t  }{\n\t\t1 sub{\tcurrentpoint\n\t\t\t0.9 3.7 9.1 6.4 6 12.4 RC\n\t\t\t1 -5.4 -4.2 -8.4 -6 -8.4 RC\n\t\t\tfill 5.4 add M\n\t\t}repeat\n\t\t1.2 3.2 9.6 5.7 5.6 14.6 RC\n\t\t1.6 -5.4 -1 -10.2 -5.6 -11.4 RC fill\n\t  }ifelse}!\n\0"
            as *const u8 as *const libc::c_char,
        -3.5f64,
        -1.0f64,
        1.0f64,
    );
    fprintf(
        fout,
        b"/sfs{\tdup 0 lt{\n\t\tdlw x y M -%.1f -%.1f RM\n\t\t%.1f add 0 exch RL currentpoint stroke\n\t\tM{\tcurrentpoint\n\t\t\t7 %.1f RL\n\t\t\t0 %.1f RL\n\t\t\t-7 -%.1f RL\n\t\t\tfill 5.4 add M\n\t\t}repeat\n\t}{\n\t\tdlw x y M %.1f %.1f RM\n\t\t%.1f sub 0 exch RL currentpoint stroke\n\t\tM{\tcurrentpoint\n\t\t\t7 -%.1f RL\n\t\t\t0 -%.1f RL\n\t\t\t-7 %.1f RL\n\t\t\tfill 5.4 sub M\n\t\t}repeat\n\t}ifelse}!\n\0"
            as *const u8 as *const libc::c_char,
        3.5f64,
        1.0f64,
        1.0f64,
        3.2f64,
        3.2f64,
        3.2f64,
        3.5f64,
        1.0f64,
        1.0f64,
        3.2f64,
        3.2f64,
        3.2f64,
    );
    fprintf(
        fout,
        b"/gu{\t.6 SLW x y M\n\t%.1f 0 RM 0 exch RL stroke}!\n/gd{\t.6 SLW x y M\n\t%.1f 0 RM 0 exch RL stroke}!\n\0"
            as *const u8 as *const libc::c_char,
        2.3f64,
        -2.3f64,
    );
    fprintf(
        fout,
        b"/sgu{\t.6 SLW x y M %.1f 0 RM\n\t0 exch RL currentpoint stroke\n\tM dup 1 eq{\n\t\tpop\n\t\t0.6 -3.4 5.6 -3.8 3 -10 RC\n\t\t1.2 4.4 -1.4 7 -3 7 RC fill\n\t  }{\n\t\t{\tcurrentpoint\n\t\t\t1 -3.2 5.6 -2.8 3.2 -8 RC\n\t\t\t1.4 4.8 -2.4 5.4 -3.2 5.2 RC\n\t\t\tfill 3.5 sub M\n\t\t}repeat\n\t  }ifelse}!\n\0"
            as *const u8 as *const libc::c_char,
        2.3f64,
    );
    fprintf(
        fout,
        b"/sgd{\t.6 SLW x y M %.1f 0 RM\n\t0 exch RL currentpoint stroke\n\tM dup 1 eq{\n\t\tpop\n\t\t0.6 3.4 5.6 3.8 3 10 RC\n\t\t1.2 -4.4 -1.4 -7 -3 -7 RC fill\n\t  }{\n\t\t{\tcurrentpoint\n\t\t\t1 3.2 5.6 2.8 3.2 8 RC\n\t\t\t1.4 -4.8 -2.4 -5.4 -3.2 -5.2 RC\n\t\t\tfill 3.5 add M\n\t\t}repeat\n\t  }ifelse}!\n\0"
            as *const u8 as *const libc::c_char,
        -2.3f64,
    );
    fprintf(
        fout,
        b"/sgs{\t.6 SLW x y M %.1f 0 RM\n\t0 exch RL currentpoint stroke\n\tM{\tcurrentpoint\n\t\t3 -1.5 RL 0 -2 RL -3 1.5 RL\n\t\tclosepath fill 3 sub M\n\t}repeat}!\n\0"
            as *const u8 as *const libc::c_char,
        2.3f64,
    );
}
