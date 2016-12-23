
#[macro_use]
extern crate nom;
use nom::{IResult,ErrorKind,Needed};



///All AMD64 registers
#[derive(Copy,Clone,Debug,PartialEq)]
#[allow(non_camel_case_types)]
pub enum Register {
    rip,eip,
    rflags,eflags,
    rax,eax,ax,al,
    rbx,ebx,bx,bl,
    rcx,ecx,cx,cl,
    rdx,edx,dx,dl,
    rbp,ebp,bp,bpl,
    rsp,esp,sp,spl,
    rsi,esi,si,sil,
    rdi,edi,di,dil,
    r8,r8d,r8w,r8b,
    r9,r9d,r9w,r9b,
    r10,r10d,r10w,r10b,
    r11,r11d,r11w,r11b,
    r12,r12d,r12w,r12b,
    r13,r13d,r13w,r13b,
    r14,r14d,r14w,r14b,
    r15,r15d,r15w,r15b,
    zmm0,ymm0,xmm0,
    zmm1,ymm1,xmm1,
    zmm2,ymm2,xmm2,
    zmm3,ymm3,xmm3,
    zmm4,ymm4,xmm4,
    zmm5,ymm5,xmm5,
    zmm6,ymm6,xmm6,
    zmm7,ymm7,xmm7,
    zmm8,ymm8,xmm8,
    zmm9,ymm9,xmm9,
    zmm10,ymm10,xmm10,
    zmm11,ymm11,xmm11,
    zmm12,ymm12,xmm12,
    zmm13,ymm13,xmm13,
    zmm14,ymm14,xmm14,
    zmm15,ymm15,xmm15,
    zmm16,ymm16,xmm16,
    zmm17,ymm17,xmm17,
    zmm18,ymm18,xmm18,
    zmm19,ymm19,xmm19,
    zmm20,ymm20,xmm20,
    zmm21,ymm21,xmm21,
    zmm22,ymm22,xmm22,
    zmm23,ymm23,xmm23,
    zmm24,ymm24,xmm24,
    zmm25,ymm25,xmm25,
    zmm26,ymm26,xmm26,
    zmm27,ymm27,xmm27,
    zmm28,ymm28,xmm28,
    zmm29,ymm29,xmm29,
    zmm30,ymm30,xmm30,
    zmm31,ymm31,xmm31,
    mmx0,
    mmx1,
    mmx2,
    mmx3,
    mmx4,
    mmx5,
    mmx6,
    mmx7,
    fpr0,
    fpr1,
    fpr2,
    fpr3,
    fpr4,
    fpr5,
    fpr6,
    fpr7
}



pub fn parse_register<'a>(i: &'a [u8]) -> IResult<&'a [u8], Register> {
    if i.len() < 2 {
        return IResult::Incomplete(Needed::Size(2));
    }
    let temp = &i[0..2];
    match temp {
        b"al" => IResult::Done(&i[2..], Register::al),
        b"ax" => IResult::Done(&i[2..], Register::ax),
        b"bl" => IResult::Done(&i[2..], Register::bl),
        b"bx" => IResult::Done(&i[2..], Register::bx),
        b"cl" => IResult::Done(&i[2..], Register::cl),
        b"cx" => IResult::Done(&i[2..], Register::cx),
        b"dx" => IResult::Done(&i[2..], Register::dx),
        b"dl" => IResult::Done(&i[2..], Register::dl),
        b"bp" => if i.len() >= 3 {
            match &i[0..3] {
                b"bpl" => IResult::Done(&i[3..], Register::bpl),
                _ => IResult::Done(&i[2..], Register::bp)
            }} else {
                IResult::Done(&i[2..], Register::bp)
        },
        b"di" => if i.len() >= 3 {
            match &i[0..3] {
                b"dil" => IResult::Done(&i[3..], Register::dil),
                _ => IResult::Done(&i[2..], Register::di)
            }} else {
                IResult::Done(&i[2..], Register::di)
        },
        b"si" => if i.len() >= 3 {
            match &i[0..3] {
                b"sil" => IResult::Done(&i[3..], Register::sil),
                _ => IResult::Done(&i[2..], Register::si)
            }} else {
                IResult::Done(&i[2..], Register::si)
        },
        b"sp" => if i.len() >= 3 {
            match &i[0..3] {
                b"spl" => IResult::Done(&i[3..], Register::spl),
                _ => IResult::Done(&i[2..], Register::sp)
            }} else {
                IResult::Done(&i[2..], Register::sp)
        },
        b"r8" => if i.len() >= 3 {
            match &i[0..3] {
                b"r8d" => IResult::Done(&i[3..], Register::r8d),
                b"r8w" => IResult::Done(&i[3..], Register::r8w),
                b"r8b" => IResult::Done(&i[3..], Register::r8b),
                _ => IResult::Done(&i[2..], Register::r8)
            }} else {
                IResult::Done(&i[2..], Register::r8)
        },
        b"r9" => if i.len() >= 3 {
            match &i[0..3] {
                b"r9d" => IResult::Done(&i[3..], Register::r9d),
                b"r9w" => IResult::Done(&i[3..], Register::r9w),
                b"r9b" => IResult::Done(&i[3..], Register::r9b),
                _ => IResult::Done(&i[2..], Register::r9)
            }} else {
            IResult::Done(&i[2..], Register::r9)
        },
        _ => if i.len() >= 3 {
            match &i[0..3] {
                b"mmx" => if i.len() >= 4 {
                        match &i[0..4] {
                            b"mmx0" => IResult::Done(&i[4..], Register::mmx0),
                            b"mmx1" => IResult::Done(&i[4..], Register::mmx1),
                            b"mmx2" => IResult::Done(&i[4..], Register::mmx2),
                            b"mmx3" => IResult::Done(&i[4..], Register::mmx3),
                            b"mmx4" => IResult::Done(&i[4..], Register::mmx4),
                            b"mmx5" => IResult::Done(&i[4..], Register::mmx5),
                            b"mmx6" => IResult::Done(&i[4..], Register::mmx6),
                            b"mmx7" => IResult::Done(&i[4..], Register::mmx7),
                            _ => IResult::Error(ErrorKind::Tag)
                        }} else {
                            IResult::Error(ErrorKind::Tag)
                },
                b"fpr" => if i.len() >= 4 {
                        match &i[0..4] {
                            b"fpr0" => IResult::Done(&i[4..], Register::fpr0),
                            b"fpr1" => IResult::Done(&i[4..], Register::fpr1),
                            b"fpr2" => IResult::Done(&i[4..], Register::fpr2),
                            b"fpr3" => IResult::Done(&i[4..], Register::fpr3),
                            b"fpr4" => IResult::Done(&i[4..], Register::fpr4),
                            b"fpr5" => IResult::Done(&i[4..], Register::fpr5),
                            b"fpr6" => IResult::Done(&i[4..], Register::fpr6),
                            b"fpr7" => IResult::Done(&i[4..], Register::fpr7),
                            _ => IResult::Error(ErrorKind::Tag)
                        }} else {
                            IResult::Error(ErrorKind::Tag)
                },
                b"rip" => IResult::Done(&i[3..], Register::rip),
                b"eip" => IResult::Done(&i[3..], Register::eip),
                b"eax" => IResult::Done(&i[3..], Register::eax),
                b"ebp" => IResult::Done(&i[3..], Register::ebp),
                b"ebx" => IResult::Done(&i[3..], Register::ebx),
                b"ecx" => IResult::Done(&i[3..], Register::ecx),
                b"edi" => IResult::Done(&i[3..], Register::edi),
                b"edx" => IResult::Done(&i[3..], Register::edx),
                b"esi" => IResult::Done(&i[3..], Register::esi),
                b"esp" => IResult::Done(&i[3..], Register::esp),
                b"rax" => IResult::Done(&i[3..], Register::rax),
                b"rbp" => IResult::Done(&i[3..], Register::rbp),
                b"rbx" => IResult::Done(&i[3..], Register::rbx),
                b"rcx" => IResult::Done(&i[3..], Register::rcx),
                b"rdi" => IResult::Done(&i[3..], Register::rdi),
                b"rdx" => IResult::Done(&i[3..], Register::rdx),
                b"rsi" => IResult::Done(&i[3..], Register::rsi),
                b"rsp" => IResult::Done(&i[3..], Register::rsp),
                b"xmm" => if i.len() >= 4 {
                    match &i[0..4] {
                        b"xmm0" => IResult::Done(&i[4..], Register::xmm0),
                        b"xmm4" => IResult::Done(&i[4..], Register::xmm4),
                        b"xmm5" => IResult::Done(&i[4..], Register::xmm5),
                        b"xmm6" => IResult::Done(&i[4..], Register::xmm6),
                        b"xmm7" => IResult::Done(&i[4..], Register::xmm7),
                        b"xmm8" => IResult::Done(&i[4..], Register::xmm8),
                        b"xmm9" => IResult::Done(&i[4..], Register::xmm9),
                        b"xmm1" => if i.len() >= 5 {
                            match &i[0..5]{
                                b"xmm10" => IResult::Done(&i[3..], Register::xmm10),
                                b"xmm11" => IResult::Done(&i[3..], Register::xmm11),
                                b"xmm12" => IResult::Done(&i[3..], Register::xmm12),
                                b"xmm13" => IResult::Done(&i[3..], Register::xmm13),
                                b"xmm14" => IResult::Done(&i[3..], Register::xmm14),
                                b"xmm15" => IResult::Done(&i[3..], Register::xmm15),
                                b"xmm16" => IResult::Done(&i[3..], Register::xmm16),
                                b"xmm17" => IResult::Done(&i[3..], Register::xmm17),
                                b"xmm18" => IResult::Done(&i[3..], Register::xmm18),
                                b"xmm19" => IResult::Done(&i[3..], Register::xmm19),
                                _ => IResult::Done(&i[4..], Register::xmm1)
                            }}else{
                                IResult::Done(&i[4..], Register::xmm1)
                        },
                        b"xmm2" => if i.len() >= 5 {
                            match &i[0..5]{
                                b"xmm20" => IResult::Done(&i[3..], Register::xmm20),
                                b"xmm21" => IResult::Done(&i[3..], Register::xmm21),
                                b"xmm22" => IResult::Done(&i[3..], Register::xmm22),
                                b"xmm23" => IResult::Done(&i[3..], Register::xmm23),
                                b"xmm24" => IResult::Done(&i[3..], Register::xmm24),
                                b"xmm25" => IResult::Done(&i[3..], Register::xmm25),
                                b"xmm26" => IResult::Done(&i[3..], Register::xmm26),
                                b"xmm27" => IResult::Done(&i[3..], Register::xmm27),
                                b"xmm28" => IResult::Done(&i[3..], Register::xmm28),
                                b"xmm29" => IResult::Done(&i[3..], Register::xmm29),
                                b"xmm30" => IResult::Done(&i[3..], Register::xmm30),
                                b"xmm31" => IResult::Done(&i[3..], Register::xmm31),
                                _ => IResult::Done(&i[4..], Register::xmm2)
                            }}else{
                                IResult::Done(&i[4..], Register::xmm2)
                        },
                        b"xmm3" => if i.len() >= 5 {
                            match &i[0..5]{
                                b"xmm30" => IResult::Done(&i[3..], Register::xmm30),
                                b"xmm31" => IResult::Done(&i[3..], Register::xmm31),
                                _ => IResult::Done(&i[4..], Register::xmm3)
                            }}else{
                                IResult::Done(&i[4..], Register::xmm3)
                        },
                        _ => IResult::Error(ErrorKind::Tag)
                    }}else {
                        IResult::Error(ErrorKind::Tag)
                },
                b"ymm" => if i.len() >= 4 {
                    match &i[0..4] {
                        b"ymm0" => IResult::Done(&i[4..], Register::ymm0),
                        b"ymm4" => IResult::Done(&i[4..], Register::ymm4),
                        b"ymm5" => IResult::Done(&i[4..], Register::ymm5),
                        b"ymm6" => IResult::Done(&i[4..], Register::ymm6),
                        b"ymm7" => IResult::Done(&i[4..], Register::ymm7),
                        b"ymm8" => IResult::Done(&i[4..], Register::ymm8),
                        b"ymm9" => IResult::Done(&i[4..], Register::ymm9),
                        b"ymm1" => if i.len() >= 5 {
                            match &i[0..5]{
                                b"ymm10" => IResult::Done(&i[3..], Register::ymm10),
                                b"ymm11" => IResult::Done(&i[3..], Register::ymm11),
                                b"ymm12" => IResult::Done(&i[3..], Register::ymm12),
                                b"ymm13" => IResult::Done(&i[3..], Register::ymm13),
                                b"ymm14" => IResult::Done(&i[3..], Register::ymm14),
                                b"ymm15" => IResult::Done(&i[3..], Register::ymm15),
                                b"ymm16" => IResult::Done(&i[3..], Register::ymm16),
                                b"ymm17" => IResult::Done(&i[3..], Register::ymm17),
                                b"ymm18" => IResult::Done(&i[3..], Register::ymm18),
                                b"ymm19" => IResult::Done(&i[3..], Register::ymm19),
                                _ => IResult::Done(&i[4..], Register::ymm1)
                            }}else{
                                IResult::Done(&i[4..], Register::ymm1)
                        },
                        b"ymm2" => if i.len() >= 5 {
                            match &i[0..5]{
                                b"ymm20" => IResult::Done(&i[3..], Register::ymm20),
                                b"ymm21" => IResult::Done(&i[3..], Register::ymm21),
                                b"ymm22" => IResult::Done(&i[3..], Register::ymm22),
                                b"ymm23" => IResult::Done(&i[3..], Register::ymm23),
                                b"ymm24" => IResult::Done(&i[3..], Register::ymm24),
                                b"ymm25" => IResult::Done(&i[3..], Register::ymm25),
                                b"ymm26" => IResult::Done(&i[3..], Register::ymm26),
                                b"ymm27" => IResult::Done(&i[3..], Register::ymm27),
                                b"ymm28" => IResult::Done(&i[3..], Register::ymm28),
                                b"ymm29" => IResult::Done(&i[3..], Register::ymm29),
                                b"ymm30" => IResult::Done(&i[3..], Register::ymm30),
                                b"ymm31" => IResult::Done(&i[3..], Register::ymm31),
                                _ => IResult::Done(&i[4..], Register::ymm2)
                            }}else{
                                IResult::Done(&i[4..], Register::ymm2)
                        },
                        b"ymm3" => if i.len() >= 5 {
                            match &i[0..5]{
                                b"ymm30" => IResult::Done(&i[3..], Register::ymm30),
                                b"ymm31" => IResult::Done(&i[3..], Register::ymm31),
                                _ => IResult::Done(&i[4..], Register::ymm3)
                            }}else{
                                IResult::Done(&i[4..], Register::ymm3)
                        },
                        _ => IResult::Error(ErrorKind::Tag)
                    }}else {
                        IResult::Error(ErrorKind::Tag)
                },
                b"zmm" => if i.len() >= 4 {
                    match &i[0..4] {
                        b"zmm0" => IResult::Done(&i[4..], Register::zmm0),
                        b"zmm4" => IResult::Done(&i[4..], Register::zmm4),
                        b"zmm5" => IResult::Done(&i[4..], Register::zmm5),
                        b"zmm6" => IResult::Done(&i[4..], Register::zmm6),
                        b"zmm7" => IResult::Done(&i[4..], Register::zmm7),
                        b"zmm8" => IResult::Done(&i[4..], Register::zmm8),
                        b"zmm9" => IResult::Done(&i[4..], Register::zmm9),
                        b"zmm1" => if i.len() >= 5 {
                            match &i[0..5]{
                                b"zmm10" => IResult::Done(&i[3..], Register::zmm10),
                                b"zmm11" => IResult::Done(&i[3..], Register::zmm11),
                                b"zmm12" => IResult::Done(&i[3..], Register::zmm12),
                                b"zmm13" => IResult::Done(&i[3..], Register::zmm13),
                                b"zmm14" => IResult::Done(&i[3..], Register::zmm14),
                                b"zmm15" => IResult::Done(&i[3..], Register::zmm15),
                                b"zmm16" => IResult::Done(&i[3..], Register::zmm16),
                                b"zmm17" => IResult::Done(&i[3..], Register::zmm17),
                                b"zmm18" => IResult::Done(&i[3..], Register::zmm18),
                                b"zmm19" => IResult::Done(&i[3..], Register::zmm19),
                                _ => IResult::Done(&i[4..], Register::zmm1)
                            }}else{
                                IResult::Done(&i[4..], Register::zmm1)
                        },
                        b"zmm2" => if i.len() >= 5 {
                            match &i[0..5]{
                                b"zmm20" => IResult::Done(&i[3..], Register::zmm20),
                                b"zmm21" => IResult::Done(&i[3..], Register::zmm21),
                                b"zmm22" => IResult::Done(&i[3..], Register::zmm22),
                                b"zmm23" => IResult::Done(&i[3..], Register::zmm23),
                                b"zmm24" => IResult::Done(&i[3..], Register::zmm24),
                                b"zmm25" => IResult::Done(&i[3..], Register::zmm25),
                                b"zmm26" => IResult::Done(&i[3..], Register::zmm26),
                                b"zmm27" => IResult::Done(&i[3..], Register::zmm27),
                                b"zmm28" => IResult::Done(&i[3..], Register::zmm28),
                                b"zmm29" => IResult::Done(&i[3..], Register::zmm29),
                                b"zmm30" => IResult::Done(&i[3..], Register::zmm30),
                                b"zmm31" => IResult::Done(&i[3..], Register::zmm31),
                                _ => IResult::Done(&i[4..], Register::zmm2)
                            }}else{
                                IResult::Done(&i[4..], Register::zmm2)
                        },
                        b"zmm3" => if i.len() >= 5 {
                            match &i[0..5]{
                                b"zmm30" => IResult::Done(&i[3..], Register::zmm30),
                                b"zmm31" => IResult::Done(&i[3..], Register::zmm31),
                                _ => IResult::Done(&i[4..], Register::zmm3)
                            }}else{
                                IResult::Done(&i[4..], Register::zmm3)
                        },
                        _ => IResult::Error(ErrorKind::Tag)
                    }}else {
                        IResult::Error(ErrorKind::Tag)
                },
                b"r10" => if i.len() >= 4 {
                    match &i[0..4] {
                        b"r10d" => IResult::Done(&i[4..], Register::r10d),
                        b"r10w" => IResult::Done(&i[4..], Register::r10w),
                        b"r10b" => IResult::Done(&i[4..], Register::r10b),
                        _ => IResult::Done(&i[3..], Register::r10)
                    }} else {
                    IResult::Done(&i[3..], Register::r10)
                },
                b"r11" => if i.len() >= 4 {
                    match &i[0..4] {
                        b"r11d" => IResult::Done(&i[4..], Register::r11d),
                        b"r11w" => IResult::Done(&i[4..], Register::r11w),
                        b"r11b" => IResult::Done(&i[4..], Register::r11b),
                        _ => IResult::Done(&i[3..], Register::r11)
                    }} else {
                    IResult::Done(&i[3..], Register::r11)
                },
                b"r12" => if i.len() >= 4 {
                    match &i[0..4] {
                        b"r12d" => IResult::Done(&i[4..], Register::r12d),
                        b"r12w" => IResult::Done(&i[4..], Register::r12w),
                        b"r12b" => IResult::Done(&i[4..], Register::r12b),
                        _ => IResult::Done(&i[3..], Register::r12)
                    }} else {
                    IResult::Done(&i[3..], Register::r12)
                },
                b"r13" => if i.len() >= 4 {
                    match &i[0..4] {
                        b"r13d" => IResult::Done(&i[4..], Register::r13d),
                        b"r13w" => IResult::Done(&i[4..], Register::r13w),
                        b"r13b" => IResult::Done(&i[4..], Register::r13b),
                        _ => IResult::Done(&i[3..], Register::r13)
                    }} else {
                    IResult::Done(&i[3..], Register::r13)
                },
                b"r14" => if i.len() >= 4 {
                    match &i[0..4] {
                        b"r14d" => IResult::Done(&i[4..], Register::r14d),
                        b"r14w" => IResult::Done(&i[4..], Register::r14w),
                        b"r14b" => IResult::Done(&i[4..], Register::r14b),
                        _ => IResult::Done(&i[3..], Register::r14)
                    }} else {
                    IResult::Done(&i[3..], Register::r14)
                },
                b"r15" => if i.len() >= 4 {
                    match &i[0..4] {
                        b"r15d" => IResult::Done(&i[4..], Register::r15d),
                        b"r15w" => IResult::Done(&i[4..], Register::r15w),
                        b"r15b" => IResult::Done(&i[4..], Register::r15b),
                        _ => IResult::Done(&i[3..], Register::r15)
                    }} else {
                    IResult::Done(&i[3..], Register::r15)
                },
                _ => if i.len() >= 6 {
                    match &i[0..6]{
                        b"rflags" => IResult::Done(&i[6..], Register::rflags),
                        b"eflags" => IResult::Done(&i[6..], Register::eflags),
                        _ => IResult::Error(ErrorKind::Tag)
                    }} else {
                        IResult::Error(ErrorKind::Tag)
                }
            }} else {
                IResult::Error(ErrorKind::Tag)
        }
    }
}
#[test]
fn test_parse_registers() {
    let reg_names: [&'static [u8];180] = [
    b"rip",b"eip",
    b"rflags",b"eflags",
    b"rax",b"eax",b"ax",b"al",
    b"rbx",b"ebx",b"bx",b"bl",
    b"rcx",b"ecx",b"cx",b"cl",
    b"rdx",b"edx",b"dx",b"dl",
    b"rbp",b"ebp",b"bp",b"bpl",
    b"rsp",b"esp",b"sp",b"spl",
    b"rsi",b"esi",b"si",b"sil",
    b"rdi",b"edi",b"di",b"dil",
    b"r8",b"r8d",b"r8w",b"r8b",
    b"r9",b"r9d",b"r9w",b"r9b",
    b"r10",b"r10d",b"r10w",b"r10b",
    b"r11",b"r11d",b"r11w",b"r11b",
    b"r12",b"r12d",b"r12w",b"r12b",
    b"r13",b"r13d",b"r13w",b"r13b",
    b"r14",b"r14d",b"r14w",b"r14b",
    b"r15",b"r15d",b"r15w",b"r15b",
    b"zmm0",b"ymm0",b"xmm0",
    b"zmm1",b"ymm1",b"xmm1",
    b"zmm2",b"ymm2",b"xmm2",
    b"zmm3",b"ymm3",b"xmm3",
    b"zmm4",b"ymm4",b"xmm4",
    b"zmm5",b"ymm5",b"xmm5",
    b"zmm6",b"ymm6",b"xmm6",
    b"zmm7",b"ymm7",b"xmm7",
    b"zmm8",b"ymm8",b"xmm8",
    b"zmm9",b"ymm9",b"xmm9",
    b"zmm10",b"ymm10",b"xmm10",
    b"zmm11",b"ymm11",b"xmm11",
    b"zmm12",b"ymm12",b"xmm12",
    b"zmm13",b"ymm13",b"xmm13",
    b"zmm14",b"ymm14",b"xmm14",
    b"zmm15",b"ymm15",b"xmm15",
    b"zmm16",b"ymm16",b"xmm16",
    b"zmm17",b"ymm17",b"xmm17",
    b"zmm18",b"ymm18",b"xmm18",
    b"zmm19",b"ymm19",b"xmm19",
    b"zmm20",b"ymm20",b"xmm20",
    b"zmm21",b"ymm21",b"xmm21",
    b"zmm22",b"ymm22",b"xmm22",
    b"zmm23",b"ymm23",b"xmm23",
    b"zmm24",b"ymm24",b"xmm24",
    b"zmm25",b"ymm25",b"xmm25",
    b"zmm26",b"ymm26",b"xmm26",
    b"zmm27",b"ymm27",b"xmm27",
    b"zmm28",b"ymm28",b"xmm28",
    b"zmm29",b"ymm29",b"xmm29",
    b"zmm30",b"ymm30",b"xmm30",
    b"zmm31",b"ymm31",b"xmm31",
    b"mmx0",
    b"mmx1",
    b"mmx2",
    b"mmx3",
    b"mmx4",
    b"mmx5",
    b"mmx6",
    b"mmx7",
    b"fpr0",
    b"fpr1",
    b"fpr2",
    b"fpr3",
    b"fpr4",
    b"fpr5",
    b"fpr6",
    b"fpr7"];
    let reg_types: Vec<Register> =
    vec![Register::rip,Register::eip,
    Register::rflags,Register::eflags,
    Register::rax,Register::eax,Register::ax,Register::al,
    Register::rbx,Register::ebx,Register::bx,Register::bl,
    Register::rcx,Register::ecx,Register::cx,Register::cl,
    Register::rdx,Register::edx,Register::dx,Register::dl,
    Register::rbp,Register::ebp,Register::bp,Register::bpl,
    Register::rsp,Register::esp,Register::sp,Register::spl,
    Register::rsi,Register::esi,Register::si,Register::sil,
    Register::rdi,Register::edi,Register::di,Register::dil,
    Register::r8,Register::r8d,Register::r8w,Register::r8b,
    Register::r9,Register::r9d,Register::r9w,Register::r9b,
    Register::r10,Register::r10d,Register::r10w,Register::r10b,
    Register::r11,Register::r11d,Register::r11w,Register::r11b,
    Register::r12,Register::r12d,Register::r12w,Register::r12b,
    Register::r13,Register::r13d,Register::r13w,Register::r13b,
    Register::r14,Register::r14d,Register::r14w,Register::r14b,
    Register::r15,Register::r15d,Register::r15w,Register::r15b,
    Register::zmm0,Register::ymm0,Register::xmm0,
    Register::zmm1,Register::ymm1,Register::xmm1,
    Register::zmm2,Register::ymm2,Register::xmm2,
    Register::zmm3,Register::ymm3,Register::xmm3,
    Register::zmm4,Register::ymm4,Register::xmm4,
    Register::zmm5,Register::ymm5,Register::xmm5,
    Register::zmm6,Register::ymm6,Register::xmm6,
    Register::zmm7,Register::ymm7,Register::xmm7,
    Register::zmm8,Register::ymm8,Register::xmm8,
    Register::zmm9,Register::ymm9,Register::xmm9,
    Register::zmm10,Register::ymm10,Register::xmm10,
    Register::zmm11,Register::ymm11,Register::xmm11,
    Register::zmm12,Register::ymm12,Register::xmm12,
    Register::zmm13,Register::ymm13,Register::xmm13,
    Register::zmm14,Register::ymm14,Register::xmm14,
    Register::zmm15,Register::ymm15,Register::xmm15,
    Register::zmm16,Register::ymm16,Register::xmm16,
    Register::zmm17,Register::ymm17,Register::xmm17,
    Register::zmm18,Register::ymm18,Register::xmm18,
    Register::zmm19,Register::ymm19,Register::xmm19,
    Register::zmm20,Register::ymm20,Register::xmm20,
    Register::zmm21,Register::ymm21,Register::xmm21,
    Register::zmm22,Register::ymm22,Register::xmm22,
    Register::zmm23,Register::ymm23,Register::xmm23,
    Register::zmm24,Register::ymm24,Register::xmm24,
    Register::zmm25,Register::ymm25,Register::xmm25,
    Register::zmm26,Register::ymm26,Register::xmm26,
    Register::zmm27,Register::ymm27,Register::xmm27,
    Register::zmm28,Register::ymm28,Register::xmm28,
    Register::zmm29,Register::ymm29,Register::xmm29,
    Register::zmm30,Register::ymm30,Register::xmm30,
    Register::zmm31,Register::ymm31,Register::xmm31,
    Register::mmx0,
    Register::mmx1,
    Register::mmx2,
    Register::mmx3,
    Register::mmx4,
    Register::mmx5,
    Register::mmx6,
    Register::mmx7,
    Register::fpr0,
    Register::fpr1,
    Register::fpr2,
    Register::fpr3,
    Register::fpr4,
    Register::fpr5,
    Register::fpr6,
    Register::fpr7
    ];
    let parse_lambda = | x: &&[u8] | -> Register {
        match parse_register(x) {
            IResult::Done(_,y) => y,
            _ => panic!("Failed to parse {}", String::from_utf8_lossy(x))
        }
    };
    let folder_lambda = |x: bool, y: (Register,&Register) | -> bool {
        if x {
            if y.0 != *y.1 {
                panic!("parse returned {:?} while {:?} is expected", y.0, y.1)
            }
            return true;
        }
        return false;
    };
    //sanity check
    assert_eq!( reg_names.len(), reg_types.len());
    assert!(
        reg_names.iter()
            .map(parse_lambda)
            .zip(reg_types.iter())
            .fold(true,folder_lambda)
    );
}

type ManyReg = Vec<Register>;
named!(pub multiple_reg<ManyReg>, many1!(ws!(parse_register)));
#[test]
fn test_multiple_registers() {
    let dut = b"zmm0r8r8br10w rflags     eax";
    let (_, val) = multiple_reg(dut).unwrap();
    assert_eq!(val.len(), 6);
    assert_eq!( val[0], Register::zmm0);
    assert_eq!( val[1], Register::r8);
    assert_eq!( val[2], Register::r8b);
    assert_eq!( val[3], Register::r10w);
    assert_eq!( val[4], Register::rflags);
    assert_eq!( val[5], Register::eax);
}

#[derive(Copy,Clone,Debug,PartialEq)]
pub enum PointerOp {
    AddCon(u64),
    SubCon(u64),
    FMAConCon(u64,u64),
}
#[derive(Copy,Clone,Debug,PartialEq)]
pub enum Pointer {
    Simple(Register),
    Operation(Register,PointerOp)
}

///parse a value
#[inline(always)]
fn to_val<'a>(x: &'a [u8]) -> IResult<&'a [u8],u64> {
    use std::str;
    use std::u64;
    match str::from_utf8(x) {
        Ok(y) => match u64::from_str_radix(y,10) {
            Ok(z) => IResult::Done(x,z),
            Err(_) => IResult::Error(ErrorKind::Tag)
        },
        Err(_) => IResult::Error(ErrorKind::Tag)
    }
}

named!(pub pointer<Pointer>, alt!(
    chain!(
        tag!("[")~
        take_while!(nom::is_space)?~
        reg: parse_register~
        take_while!(nom::is_space)?~
        tag!("]"),
        || Pointer::Simple(reg)) |
    chain!(
        tag!("[")~
        take_while!(nom::is_space)?~
        reg: parse_register~
        take_while!(nom::is_space)?~
        tag!("+")~
        take_while!(nom::is_space)?~
        val: to_val~
        take_while!(nom::is_space)?~
        tag!("]"),
        ||Pointer::Operation(reg,PointerOp::AddCon(val))) |
    chain!(
        tag!("[")~
        take_while!(nom::is_space)?~
        reg: parse_register~
        take_while!(nom::is_space)?~
        tag!("-")~
        take_while!(nom::is_space)?~
        val: to_val~
        take_while!(nom::is_space)?~
        tag!("]"),
        ||Pointer::Operation(reg,PointerOp::SubCon(val)))
));
#[test]
fn test_pointer() {
    //simplest path
    let dut = b"[ rdx ]";
    let (_,val) = pointer(dut).unwrap();
    assert_eq!(val, Pointer::Simple(Register::rdx));
    let dut = b"[rax]";
    let (_,val) = pointer(dut).unwrap();
    assert_eq!(val, Pointer::Simple(Register::rax));
    //register before value
    let dut = b"[rdx+10]";
    let (_,val) = pointer(dut).unwrap();
    assert_eq!(val, Pointer::Operation(Register::rdx, PointerOp::AddCon(10)));
    //register before value
    let dut = b"[r11-19]";
    let (_,val) = pointer(dut).unwrap();
    assert_eq!(val, Pointer::Operation(Register::r11, PointerOp::SubCon(19)));
}
