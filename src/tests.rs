use super::{decode_mnemonic_a64, decode_a32, decode_a64, Instr, Mnemonic};

#[test]
fn test_ldxrb() {
    // ldxrb w9, [x0]
    assert_eq!(
        decode_a64(0x85F7C09).unwrap(),
        Instr::LdxrbLr32Ldstexcl {
            Rs: 31,
            Rn: 0,
            Rt: 9
        }
    );
    assert_eq!(
        decode_mnemonic_a64(0x85F7C09).unwrap(),
        Mnemonic::LdxrbLr32Ldstexcl
    );
    // ldxrb w8, [x0]
    assert_eq!(
        decode_a64(0x85F7C08).unwrap(),
        Instr::LdxrbLr32Ldstexcl {
            Rs: 31,
            Rn: 0,
            Rt: 8
        }
    );
}

#[test]
fn test_ldaxrb() {
    // ldaxrb w8, [x0]
    assert_eq!(
        decode_a64(0x85FFC08).unwrap(),
        Instr::LdaxrbLr32Ldstexcl {
            Rs: 31,
            Rn: 0,
            Rt: 8
        }
    );
    // ldaxrb w9, [x0]
    assert_eq!(
        decode_a64(0x85FFC09).unwrap(),
        Instr::LdaxrbLr32Ldstexcl {
            Rs: 31,
            Rn: 0,
            Rt: 9
        }
    );
    // ldaxrb w10, [x0]
    assert_eq!(
        decode_a64(0x85FFC0A).unwrap(),
        Instr::LdaxrbLr32Ldstexcl {
            Rs: 31,
            Rn: 0,
            Rt: 10
        }
    );
    // ldaxrb w10, [x8]
    assert_eq!(
        decode_a64(0x85FFD0A).unwrap(),
        Instr::LdaxrbLr32Ldstexcl {
            Rs: 31,
            Rn: 8,
            Rt: 10
        }
    );
}

#[test]
fn test_ldarb() {
    // ldarb w8, [x0]
    assert_eq!(
        decode_a64(0x8DFFC08).unwrap(),
        Instr::LdarbLr32Ldstexcl {
            Rs: 31,
            Rn: 0,
            Rt: 8
        }
    );
    // ldarb w9, [x8]
    assert_eq!(
        decode_a64(0x8DFFD09).unwrap(),
        Instr::LdarbLr32Ldstexcl {
            Rs: 31,
            Rn: 8,
            Rt: 9
        }
    );
    // ldarb w10, [x10]
    assert_eq!(
        decode_a64(0x8DFFD4A).unwrap(),
        Instr::LdarbLr32Ldstexcl {
            Rs: 31,
            Rn: 10,
            Rt: 10
        }
    );
    // ldarb w8, [x22]
    assert_eq!(
        decode_a64(0x8DFFEC8).unwrap(),
        Instr::LdarbLr32Ldstexcl {
            Rs: 31,
            Rn: 22,
            Rt: 8
        }
    );
    // ldarb w8, [x26]
    assert_eq!(
        decode_a64(0x8DFFF48).unwrap(),
        Instr::LdarbLr32Ldstexcl {
            Rs: 31,
            Rn: 26,
            Rt: 8
        }
    );
    // ldarb w8, [x27]
    assert_eq!(
        decode_a64(0x8DFFF68).unwrap(),
        Instr::LdarbLr32Ldstexcl {
            Rs: 31,
            Rn: 27,
            Rt: 8
        }
    );
}

#[test]
fn test_bic() {
    // bic w2, w6, w2
    assert_eq!(
        decode_a64(0xA2200C2).unwrap(),
        Instr::Bic32LogShift {
            shift: 0,
            Rm: 2,
            Rn: 6,
            Rd: 2
        }
    );
    // bic w8, w1, w8
    assert_eq!(
        decode_a64(0xA280028).unwrap(),
        Instr::Bic32LogShift {
            shift: 0,
            Rm: 8,
            Rn: 1,
            Rd: 8
        }
    );
    // bic w2, w13, w2, asr #31
    assert_eq!(
        decode_a64(0xAA27DA2).unwrap(),
        Instr::Bic32LogShift {
            shift: 2,
            Rm: 2,
            Rn: 13,
            Rd: 2
        }
    );
    // bic w7, w12, w7, asr #31
    assert_eq!(
        decode_a64(0xAA77D87).unwrap(),
        Instr::Bic32LogShift {
            shift: 2,
            Rm: 7,
            Rn: 12,
            Rd: 7
        }
    );
    // bic v2.16b, v2.16b, v0.16b
    assert_eq!(
        decode_a64(0x4E601C42).unwrap(),
        Instr::BicAsimdsameOnly {
            Q: 1,
            Rm: 0,
            Rn: 2,
            Rd: 2
        }
    );
    // bic v30.16b, v30.16b, v27.16b
    assert_eq!(
        decode_a64(0x4E7B1FDE).unwrap(),
        Instr::BicAsimdsameOnly {
            Q: 1,
            Rm: 27,
            Rn: 30,
            Rd: 30
        }
    );
}

#[test]
fn test_add() {
    // add w6, w5, w0
    assert_eq!(
        decode_a64(0xB0000A6).unwrap(),
        Instr::Add32AddsubShift {
            Rm: 0,
            Rn: 5,
            Rd: 6
        }
    );
    // add w18, w18, #0x100
    assert_eq!(
        decode_a64(0x11040252).unwrap(),
        Instr::Add32AddsubImm {
            sh: 0,
            imm12: 256,
            Rn: 18,
            Rd: 18
        }
    );
    // add w13, w21, #0x770, lsl #12
    assert_eq!(
        decode_a64(0x115DC2AD).unwrap(),
        Instr::Add32AddsubImm {
            sh: 1,
            imm12: 1904,
            Rn: 21,
            Rd: 13
        }
    );
    // add w9, w8, #0x900, lsl #12
    assert_eq!(
        decode_a64(0x11640109).unwrap(),
        Instr::Add32AddsubImm {
            sh: 1,
            imm12: 2304,
            Rn: 8,
            Rd: 9
        }
    );
    // add x9, x11, w10, sxtw
    assert_eq!(
        decode_a64(0x8B2AC169).unwrap(),
        Instr::Add64AddsubExt {
            Rm: 10,
            option: 6,
            Rn: 11,
            Rd: 9
        }
    );
    // add x8, x8, w11, sxtw #1
    assert_eq!(
        decode_a64(0x8B2BC508).unwrap(),
        Instr::Add64AddsubExt {
            Rm: 11,
            option: 6,
            Rn: 8,
            Rd: 8
        }
    );
    // add x29, sp, #0x10
    assert_eq!(
        decode_a64(0x910043FD).unwrap(),
        Instr::Add64AddsubImm {
            sh: 0,
            imm12: 16,
            Rn: 31,
            Rd: 29
        }
    );
    // add x10, x8, #0xb8
    assert_eq!(
        decode_a64(0x9102E10A).unwrap(),
        Instr::Add64AddsubImm {
            sh: 0,
            imm12: 184,
            Rn: 8,
            Rd: 10
        }
    );
    // add x25, x26, #0x190
    assert_eq!(
        decode_a64(0x91064359).unwrap(),
        Instr::Add64AddsubImm {
            sh: 0,
            imm12: 400,
            Rn: 26,
            Rd: 25
        }
    );
}

#[test]
fn test_ld2() {
    // ld2 {v3.2s, v4.2s}, [x15]
    assert_eq!(
        decode_a64(0xC4089E3).unwrap(),
        Instr::Ld2AsisdlseR2 {
            Q: 0,
            size: 2,
            Rn: 15,
            Rt: 3
        }
    );
    // ld2 {v6.2d, v7.2d}, [x13]
    assert_eq!(
        decode_a64(0x4C408DA6).unwrap(),
        Instr::Ld2AsisdlseR2 {
            Q: 1,
            size: 3,
            Rn: 13,
            Rt: 6
        }
    );
    // ld2 {v4.2d, v5.2d}, [x15]
    assert_eq!(
        decode_a64(0x4C408DE4).unwrap(),
        Instr::Ld2AsisdlseR2 {
            Q: 1,
            size: 3,
            Rn: 15,
            Rt: 4
        }
    );
}

#[test]
fn test_ld1() {
    // ld1 {v5.s}[0], [x0]
    assert_eq!(
        decode_a64(0xD408005).unwrap(),
        Instr::Ld1AsisdlsoS11S { Q: 0, Rn: 0, Rt: 5 }
    );
    // ld1 {v1.s}[0], [x11]
    assert_eq!(
        decode_a64(0xD408161).unwrap(),
        Instr::Ld1AsisdlsoS11S {
            Q: 0,
            Rn: 11,
            Rt: 1
        }
    );
    // ld1 {v3.s}[0], [x18]
    assert_eq!(
        decode_a64(0xD408243).unwrap(),
        Instr::Ld1AsisdlsoS11S {
            Q: 0,
            Rn: 18,
            Rt: 3
        }
    );
    // ld1 {v5.s}[1], [x11]
    assert_eq!(
        decode_a64(0xD409165).unwrap(),
        Instr::Ld1AsisdlsoS11S {
            Q: 0,
            Rn: 11,
            Rt: 5
        }
    );
    // ld1 {v1.s}[0], [x1], #4
    assert_eq!(
        decode_a64(0xDDF8021).unwrap(),
        Instr::Ld1AsisdlsopS1I1S { Q: 0, Rn: 1, Rt: 1 }
    );
    // ld1 {v0.s}[0], [x8], #4
    assert_eq!(
        decode_a64(0xDDF8100).unwrap(),
        Instr::Ld1AsisdlsopS1I1S { Q: 0, Rn: 8, Rt: 0 }
    );
    // ld1 {v3.s}[1], [x12], #4
    assert_eq!(
        decode_a64(0xDDF9183).unwrap(),
        Instr::Ld1AsisdlsopS1I1S {
            Q: 0,
            Rn: 12,
            Rt: 3
        }
    );
    // ld1 {v1.s}[3], [x17]
    assert_eq!(
        decode_a64(0x4D409221).unwrap(),
        Instr::Ld1AsisdlsoS11S {
            Q: 1,
            Rn: 17,
            Rt: 1
        }
    );
    // ld1 {v1.s}[3], [x27], x25
    assert_eq!(
        decode_a64(0x4DD99361).unwrap(),
        Instr::Ld1AsisdlsopSx1R1S {
            Q: 1,
            Rn: 27,
            Rt: 1
        }
    );
    // ld1 {v0.s}[3], [x8], #4
    assert_eq!(
        decode_a64(0x4DDF9100).unwrap(),
        Instr::Ld1AsisdlsopS1I1S { Q: 1, Rn: 8, Rt: 0 }
    );
    // ld1 {v0.s}[3], [x17], #4
    assert_eq!(
        decode_a64(0x4DDF9220).unwrap(),
        Instr::Ld1AsisdlsopS1I1S {
            Q: 1,
            Rn: 17,
            Rt: 0
        }
    );
    // ld1 {v16.s}[3], [x21], #4
    assert_eq!(
        decode_a64(0x4DDF92B0).unwrap(),
        Instr::Ld1AsisdlsopS1I1S {
            Q: 1,
            Rn: 21,
            Rt: 16
        }
    );
}

#[test]
fn test_ld1r() {
    // ld1r {v5.2s}, [x1], #4
    assert_eq!(
        decode_a64(0xDDFC825).unwrap(),
        Instr::Ld1RAsisdlsopR1I { Q: 0, Rn: 1, Rt: 5 }
    );
    // ld1r {v7.16b}, [x8]
    assert_eq!(
        decode_a64(0x4D40C107).unwrap(),
        Instr::Ld1RAsisdlsoR1 { Q: 1, Rn: 8, Rt: 7 }
    );
    // ld1r {v6.4s}, [x8]
    assert_eq!(
        decode_a64(0x4D40C906).unwrap(),
        Instr::Ld1RAsisdlsoR1 { Q: 1, Rn: 8, Rt: 6 }
    );
    // ld1r {v19.4s}, [x10]
    assert_eq!(
        decode_a64(0x4D40C953).unwrap(),
        Instr::Ld1RAsisdlsoR1 {
            Q: 1,
            Rn: 10,
            Rt: 19
        }
    );
    // ld1r {v25.4s}, [x10]
    assert_eq!(
        decode_a64(0x4D40C959).unwrap(),
        Instr::Ld1RAsisdlsoR1 {
            Q: 1,
            Rn: 10,
            Rt: 25
        }
    );
    // ld1r {v0.4s}, [x11]
    assert_eq!(
        decode_a64(0x4D40C960).unwrap(),
        Instr::Ld1RAsisdlsoR1 {
            Q: 1,
            Rn: 11,
            Rt: 0
        }
    );
    // ld1r {v5.4s}, [x11]
    assert_eq!(
        decode_a64(0x4D40C965).unwrap(),
        Instr::Ld1RAsisdlsoR1 {
            Q: 1,
            Rn: 11,
            Rt: 5
        }
    );
    // ld1r {v0.16b}, [x8], #1
    assert_eq!(
        decode_a64(0x4DDFC100).unwrap(),
        Instr::Ld1RAsisdlsopR1I { Q: 1, Rn: 8, Rt: 0 }
    );
    // ld1r {v1.4s}, [x8], #4
    assert_eq!(
        decode_a64(0x4DDFC901).unwrap(),
        Instr::Ld1RAsisdlsopR1I { Q: 1, Rn: 8, Rt: 1 }
    );
    // ld1r {v27.4s}, [x8], #4
    assert_eq!(
        decode_a64(0x4DDFC91B).unwrap(),
        Instr::Ld1RAsisdlsopR1I {
            Q: 1,
            Rn: 8,
            Rt: 27
        }
    );
    // ld1r {v9.4s}, [x12], #4
    assert_eq!(
        decode_a64(0x4DDFC989).unwrap(),
        Instr::Ld1RAsisdlsopR1I {
            Q: 1,
            Rn: 12,
            Rt: 9
        }
    );
    // ld1r {v6.4s}, [x16], #4
    assert_eq!(
        decode_a64(0x4DDFCA06).unwrap(),
        Instr::Ld1RAsisdlsopR1I {
            Q: 1,
            Rn: 16,
            Rt: 6
        }
    );
}

#[test]
fn test_tbl() {
    // tbl v1.8b, {v5.16b}, v1.8b
    assert_eq!(
        decode_a64(0xE0100A1).unwrap(),
        Instr::TblAsimdtblL11 {
            Q: 0,
            Rm: 1,
            Rn: 5,
            Rd: 1
        }
    );
    // tbl v4.8b, {v7.16b}, v1.8b
    assert_eq!(
        decode_a64(0xE0100E4).unwrap(),
        Instr::TblAsimdtblL11 {
            Q: 0,
            Rm: 1,
            Rn: 7,
            Rd: 4
        }
    );
    // tbl v5.8b, {v23.16b}, v16.8b
    assert_eq!(
        decode_a64(0xE1002E5).unwrap(),
        Instr::TblAsimdtblL11 {
            Q: 0,
            Rm: 16,
            Rn: 23,
            Rd: 5
        }
    );
    // tbl v28.8b, {v27.16b}, v16.8b
    assert_eq!(
        decode_a64(0xE10037C).unwrap(),
        Instr::TblAsimdtblL11 {
            Q: 0,
            Rm: 16,
            Rn: 27,
            Rd: 28
        }
    );
    // tbl v12.8b, {v8.16b, v9.16b}, v18.8b
    assert_eq!(
        decode_a64(0xE12210C).unwrap(),
        Instr::TblAsimdtblL22 {
            Q: 0,
            Rm: 18,
            Rn: 8,
            Rd: 12
        }
    );
    // tbl v26.8b, {v26.16b, v27.16b}, v31.8b
    assert_eq!(
        decode_a64(0xE1F235A).unwrap(),
        Instr::TblAsimdtblL22 {
            Q: 0,
            Rm: 31,
            Rn: 26,
            Rd: 26
        }
    );
}

#[test]
fn test_umov() {
    // umov w8, v0.h[0]
    assert_eq!(
        decode_a64(0xE023C08).unwrap(),
        Instr::UmovAsimdinsWW { Rn: 0, Rd: 8 }
    );
    // umov w9, v0.h[0]
    assert_eq!(
        decode_a64(0xE023C09).unwrap(),
        Instr::UmovAsimdinsWW { Rn: 0, Rd: 9 }
    );
    // umov w9, v1.h[0]
    assert_eq!(
        decode_a64(0xE023C29).unwrap(),
        Instr::UmovAsimdinsWW { Rn: 1, Rd: 9 }
    );
    // umov w10, v2.h[0]
    assert_eq!(
        decode_a64(0xE023C4A).unwrap(),
        Instr::UmovAsimdinsWW { Rn: 2, Rd: 10 }
    );
    // umov w8, v5.h[0]
    assert_eq!(
        decode_a64(0xE023CA8).unwrap(),
        Instr::UmovAsimdinsWW { Rn: 5, Rd: 8 }
    );
    // umov w16, v16.h[0]
    assert_eq!(
        decode_a64(0xE023E10).unwrap(),
        Instr::UmovAsimdinsWW { Rn: 16, Rd: 16 }
    );
}

#[test]
fn test_dup() {
    // dup v31.2s, v24.s[1]
    assert_eq!(
        decode_a64(0xE0C071F).unwrap(),
        Instr::DupAsimdinsDvV { Rn: 24, Rd: 31 }
    );
    // dup v17.4s, v17.s[0]
    assert_eq!(
        decode_a64(0x4E040631).unwrap(),
        Instr::DupAsimdinsDvV { Rn: 17, Rd: 17 }
    );
    // dup v21.4s, v21.s[0]
    assert_eq!(
        decode_a64(0x4E0406B5).unwrap(),
        Instr::DupAsimdinsDvV { Rn: 21, Rd: 21 }
    );
    // dup v20.4s, v25.s[0]
    assert_eq!(
        decode_a64(0x4E040734).unwrap(),
        Instr::DupAsimdinsDvV { Rn: 25, Rd: 20 }
    );
    // dup v0.4s, w1
    assert_eq!(
        decode_a64(0x4E040C20).unwrap(),
        Instr::DupAsimdinsDrR { Rn: 1, Rd: 0 }
    );
    // dup v3.2d, v2.d[1]
    assert_eq!(
        decode_a64(0x4E180443).unwrap(),
        Instr::DupAsimdinsDvV { Rn: 2, Rd: 3 }
    );
}

#[test]
fn test_scvtf() {
    // scvtf v27.2s, v27.2s
    assert_eq!(
        decode_a64(0xE21DB7B).unwrap(),
        Instr::ScvtfAsimdmiscR {
            Q: 0,
            Rn: 27,
            Rd: 27
        }
    );
    // scvtf s5, w8
    assert_eq!(
        decode_a64(0x1E220105).unwrap(),
        Instr::ScvtfS32Float2Int { Rn: 8, Rd: 5 }
    );
    // scvtf s30, w10
    assert_eq!(
        decode_a64(0x1E22015E).unwrap(),
        Instr::ScvtfS32Float2Int { Rn: 10, Rd: 30 }
    );
    // scvtf s21, w21
    assert_eq!(
        decode_a64(0x1E2202B5).unwrap(),
        Instr::ScvtfS32Float2Int { Rn: 21, Rd: 21 }
    );
    // scvtf d0, w2
    assert_eq!(
        decode_a64(0x1E620040).unwrap(),
        Instr::ScvtfD32Float2Int { Rn: 2, Rd: 0 }
    );
    // scvtf s1, s0
    assert_eq!(
        decode_a64(0x5E21D801).unwrap(),
        Instr::ScvtfAsisdmiscR { Rn: 0, Rd: 1 }
    );
}

#[test]
fn test_fadd() {
    // fadd v3.2s, v5.2s, v3.2s
    assert_eq!(
        decode_a64(0xE23D4A3).unwrap(),
        Instr::FaddAsimdsameOnly {
            Q: 0,
            Rm: 3,
            Rn: 5,
            Rd: 3
        }
    );
    // fadd s2, s0, s0
    assert_eq!(
        decode_a64(0x1E202802).unwrap(),
        Instr::FaddSFloatdp2 {
            Rm: 0,
            Rn: 0,
            Rd: 2
        }
    );
    // fadd s16, s16, s3
    assert_eq!(
        decode_a64(0x1E232A10).unwrap(),
        Instr::FaddSFloatdp2 {
            Rm: 3,
            Rn: 16,
            Rd: 16
        }
    );
    // fadd s22, s17, s4
    assert_eq!(
        decode_a64(0x1E242A36).unwrap(),
        Instr::FaddSFloatdp2 {
            Rm: 4,
            Rn: 17,
            Rd: 22
        }
    );
    // fadd s3, s4, s18
    assert_eq!(
        decode_a64(0x1E322883).unwrap(),
        Instr::FaddSFloatdp2 {
            Rm: 18,
            Rn: 4,
            Rd: 3
        }
    );
    // fadd d1, d19, d1
    assert_eq!(
        decode_a64(0x1E612A61).unwrap(),
        Instr::FaddDFloatdp2 {
            Rm: 1,
            Rn: 19,
            Rd: 1
        }
    );
    // fadd v30.4s, v31.4s, v31.4s
    assert_eq!(
        decode_a64(0x4E3FD7FE).unwrap(),
        Instr::FaddAsimdsameOnly {
            Q: 1,
            Rm: 31,
            Rn: 31,
            Rd: 30
        }
    );
}

#[test]
fn test_addv() {
    // addv b1, v0.8b
    assert_eq!(
        decode_a64(0xE31B801).unwrap(),
        Instr::AddvAsimdallOnly { Q: 0, Rn: 0, Rd: 1 }
    );
    // addv s1, v1.4s
    assert_eq!(
        decode_a64(0x4EB1B821).unwrap(),
        Instr::AddvAsimdallOnly { Q: 1, Rn: 1, Rd: 1 }
    );
    // addv s3, v3.4s
    assert_eq!(
        decode_a64(0x4EB1B863).unwrap(),
        Instr::AddvAsimdallOnly { Q: 1, Rn: 3, Rd: 3 }
    );
    // addv s6, v6.4s
    assert_eq!(
        decode_a64(0x4EB1B8C6).unwrap(),
        Instr::AddvAsimdallOnly { Q: 1, Rn: 6, Rd: 6 }
    );
    // addv s7, v7.4s
    assert_eq!(
        decode_a64(0x4EB1B8E7).unwrap(),
        Instr::AddvAsimdallOnly { Q: 1, Rn: 7, Rd: 7 }
    );
    // addv s17, v17.4s
    assert_eq!(
        decode_a64(0x4EB1BA31).unwrap(),
        Instr::AddvAsimdallOnly {
            Q: 1,
            Rn: 17,
            Rd: 17
        }
    );
}

#[test]
fn test_xtn() {
    // xtn v3.4h, v3.4s
    assert_eq!(
        decode_a64(0xE612863).unwrap(),
        Instr::XtnAsimdmiscN { Q: 0, Rn: 3, Rd: 3 }
    );
    // xtn v21.4h, v7.4s
    assert_eq!(
        decode_a64(0xE6128F5).unwrap(),
        Instr::XtnAsimdmiscN {
            Q: 0,
            Rn: 7,
            Rd: 21
        }
    );
    // xtn v13.4h, v10.4s
    assert_eq!(
        decode_a64(0xE61294D).unwrap(),
        Instr::XtnAsimdmiscN {
            Q: 0,
            Rn: 10,
            Rd: 13
        }
    );
    // xtn v0.4h, v13.4s
    assert_eq!(
        decode_a64(0xE6129A0).unwrap(),
        Instr::XtnAsimdmiscN {
            Q: 0,
            Rn: 13,
            Rd: 0
        }
    );
    // xtn v1.2s, v0.2d
    assert_eq!(
        decode_a64(0xEA12801).unwrap(),
        Instr::XtnAsimdmiscN { Q: 0, Rn: 0, Rd: 1 }
    );
    // xtn v4.2s, v1.2d
    assert_eq!(
        decode_a64(0xEA12824).unwrap(),
        Instr::XtnAsimdmiscN { Q: 0, Rn: 1, Rd: 4 }
    );
}

#[test]
fn test_zip1() {
    // zip1 v30.2s, v28.2s, v29.2s
    assert_eq!(
        decode_a64(0xE9D3B9E).unwrap(),
        Instr::Zip1AsimdpermOnly {
            Q: 0,
            size: 2,
            Rm: 29,
            Rn: 28,
            Rd: 30
        }
    );
    // zip1 v0.4s, v2.4s, v1.4s
    assert_eq!(
        decode_a64(0x4E813840).unwrap(),
        Instr::Zip1AsimdpermOnly {
            Q: 1,
            size: 2,
            Rm: 1,
            Rn: 2,
            Rd: 0
        }
    );
    // zip1 v5.4s, v0.4s, v2.4s
    assert_eq!(
        decode_a64(0x4E823805).unwrap(),
        Instr::Zip1AsimdpermOnly {
            Q: 1,
            size: 2,
            Rm: 2,
            Rn: 0,
            Rd: 5
        }
    );
    // zip1 v2.4s, v1.4s, v4.4s
    assert_eq!(
        decode_a64(0x4E843822).unwrap(),
        Instr::Zip1AsimdpermOnly {
            Q: 1,
            size: 2,
            Rm: 4,
            Rn: 1,
            Rd: 2
        }
    );
    // zip1 v3.4s, v9.4s, v11.4s
    assert_eq!(
        decode_a64(0x4E8B3923).unwrap(),
        Instr::Zip1AsimdpermOnly {
            Q: 1,
            size: 2,
            Rm: 11,
            Rn: 9,
            Rd: 3
        }
    );
    // zip1 v5.4s, v16.4s, v17.4s
    assert_eq!(
        decode_a64(0x4E913A05).unwrap(),
        Instr::Zip1AsimdpermOnly {
            Q: 1,
            size: 2,
            Rm: 17,
            Rn: 16,
            Rd: 5
        }
    );
}

#[test]
fn test_sqadd() {
    // sqadd v0.2s, v1.2s, v0.2s
    assert_eq!(
        decode_a64(0xEA00C20).unwrap(),
        Instr::SqaddAsimdsameOnly {
            Q: 0,
            Rm: 0,
            Rn: 1,
            Rd: 0
        }
    );
    // sqadd v1.2s, v1.2s, v0.2s
    assert_eq!(
        decode_a64(0xEA00C21).unwrap(),
        Instr::SqaddAsimdsameOnly {
            Q: 0,
            Rm: 0,
            Rn: 1,
            Rd: 1
        }
    );
    // sqadd v2.2s, v2.2s, v0.2s
    assert_eq!(
        decode_a64(0xEA00C42).unwrap(),
        Instr::SqaddAsimdsameOnly {
            Q: 0,
            Rm: 0,
            Rn: 2,
            Rd: 2
        }
    );
    // sqadd v1.2s, v2.2s, v1.2s
    assert_eq!(
        decode_a64(0xEA10C41).unwrap(),
        Instr::SqaddAsimdsameOnly {
            Q: 0,
            Rm: 1,
            Rn: 2,
            Rd: 1
        }
    );
    // sqadd v1.2s, v3.2s, v1.2s
    assert_eq!(
        decode_a64(0xEA10C61).unwrap(),
        Instr::SqaddAsimdsameOnly {
            Q: 0,
            Rm: 1,
            Rn: 3,
            Rd: 1
        }
    );
    // sqadd v2.2s, v3.2s, v2.2s
    assert_eq!(
        decode_a64(0xEA20C62).unwrap(),
        Instr::SqaddAsimdsameOnly {
            Q: 0,
            Rm: 2,
            Rn: 3,
            Rd: 2
        }
    );
}

#[test]
fn test_smull() {
    // smull v2.2d, v2.2s, v0.2s
    assert_eq!(
        decode_a64(0xEA0C042).unwrap(),
        Instr::SmullAsimddiffL {
            Q: 0,
            size: 2,
            Rm: 0,
            Rn: 2,
            Rd: 2
        }
    );
    // smull x9, w9, w0
    assert_eq!(
        decode_a64(0x9B207D29).unwrap(),
        Instr::Smaddl64WaDp3Src {
            Rm: 0,
            Ra: 31,
            Rn: 9,
            Rd: 9
        }
    );
    // smull x4, w4, w4
    assert_eq!(
        decode_a64(0x9B247C84).unwrap(),
        Instr::Smaddl64WaDp3Src {
            Rm: 4,
            Ra: 31,
            Rn: 4,
            Rd: 4
        }
    );
    // smull x7, w1, w6
    assert_eq!(
        decode_a64(0x9B267C27).unwrap(),
        Instr::Smaddl64WaDp3Src {
            Rm: 6,
            Ra: 31,
            Rn: 1,
            Rd: 7
        }
    );
    // smull x10, w10, w10
    assert_eq!(
        decode_a64(0x9B2A7D4A).unwrap(),
        Instr::Smaddl64WaDp3Src {
            Rm: 10,
            Ra: 31,
            Rn: 10,
            Rd: 10
        }
    );
    // smull x11, w0, w11
    assert_eq!(
        decode_a64(0x9B2B7C0B).unwrap(),
        Instr::Smaddl64WaDp3Src {
            Rm: 11,
            Ra: 31,
            Rn: 0,
            Rd: 11
        }
    );
    // smull x9, w1, w22
    assert_eq!(
        decode_a64(0x9B367C29).unwrap(),
        Instr::Smaddl64WaDp3Src {
            Rm: 22,
            Ra: 31,
            Rn: 1,
            Rd: 9
        }
    );
}

#[test]
fn test_smlal() {
    // smlal v2.2d, v0.2s, v5.2s
    assert_eq!(
        decode_a64(0xEA58002).unwrap(),
        Instr::SmlalAsimddiffL {
            Q: 0,
            size: 2,
            Rm: 5,
            Rn: 0,
            Rd: 2
        }
    );
    // smlal v2.2d, v1.2s, v5.2s
    assert_eq!(
        decode_a64(0xEA58022).unwrap(),
        Instr::SmlalAsimddiffL {
            Q: 0,
            size: 2,
            Rm: 5,
            Rn: 1,
            Rd: 2
        }
    );
    // smlal v2.2d, v3.2s, v6.s[0]
    assert_eq!(
        decode_a64(0xF862062).unwrap(),
        Instr::SmlalAsimdelemL {
            Q: 0,
            L: 0,
            M: 0,
            Rm: 6,
            H: 0,
            Rn: 3,
            Rd: 2
        }
    );
}

#[test]
fn test_pmull() {
    // pmull v0.1q, v1.1d, v0.1d
    assert_eq!(
        decode_a64(0xEE0E020).unwrap(),
        Instr::PmullAsimddiffL {
            Q: 0,
            size: 3,
            Rm: 0,
            Rn: 1,
            Rd: 0
        }
    );
    // pmull v2.1q, v0.1d, v1.1d
    assert_eq!(
        decode_a64(0xEE1E002).unwrap(),
        Instr::PmullAsimddiffL {
            Q: 0,
            size: 3,
            Rm: 1,
            Rn: 0,
            Rd: 2
        }
    );
    // pmull v31.1q, v19.1d, v1.1d
    assert_eq!(
        decode_a64(0xEE1E27F).unwrap(),
        Instr::PmullAsimddiffL {
            Q: 0,
            size: 3,
            Rm: 1,
            Rn: 19,
            Rd: 31
        }
    );
    // pmull v21.1q, v17.1d, v2.1d
    assert_eq!(
        decode_a64(0xEE2E235).unwrap(),
        Instr::PmullAsimddiffL {
            Q: 0,
            size: 3,
            Rm: 2,
            Rn: 17,
            Rd: 21
        }
    );
    // pmull v20.1q, v16.1d, v3.1d
    assert_eq!(
        decode_a64(0xEE3E214).unwrap(),
        Instr::PmullAsimddiffL {
            Q: 0,
            size: 3,
            Rm: 3,
            Rn: 16,
            Rd: 20
        }
    );
    // pmull v22.1q, v18.1d, v8.1d
    assert_eq!(
        decode_a64(0xEE8E256).unwrap(),
        Instr::PmullAsimddiffL {
            Q: 0,
            size: 3,
            Rm: 8,
            Rn: 18,
            Rd: 22
        }
    );
}

#[test]
fn test_sshll() {
    // sshll v2.4s, v2.4h, #0
    assert_eq!(
        decode_a64(0xF10A442).unwrap(),
        Instr::SshllAsimdshfL {
            Q: 0,
            immh: 2,
            immb: 0,
            Rn: 2,
            Rd: 2
        }
    );
    // sshll v3.4s, v3.4h, #0
    assert_eq!(
        decode_a64(0xF10A463).unwrap(),
        Instr::SshllAsimdshfL {
            Q: 0,
            immh: 2,
            immb: 0,
            Rn: 3,
            Rd: 3
        }
    );
    // sshll v5.4s, v5.4h, #0
    assert_eq!(
        decode_a64(0xF10A4A5).unwrap(),
        Instr::SshllAsimdshfL {
            Q: 0,
            immh: 2,
            immb: 0,
            Rn: 5,
            Rd: 5
        }
    );
    // sshll v2.2d, v2.2s, #0
    assert_eq!(
        decode_a64(0xF20A442).unwrap(),
        Instr::SshllAsimdshfL {
            Q: 0,
            immh: 4,
            immb: 0,
            Rn: 2,
            Rd: 2
        }
    );
    // sshll v3.2d, v3.2s, #0
    assert_eq!(
        decode_a64(0xF20A463).unwrap(),
        Instr::SshllAsimdshfL {
            Q: 0,
            immh: 4,
            immb: 0,
            Rn: 3,
            Rd: 3
        }
    );
    // sshll v4.2d, v4.2s, #0
    assert_eq!(
        decode_a64(0xF20A484).unwrap(),
        Instr::SshllAsimdshfL {
            Q: 0,
            immh: 4,
            immb: 0,
            Rn: 4,
            Rd: 4
        }
    );
}

#[test]
fn test_shrn() {
    // shrn v2.2s, v0.2d, #0x20
    assert_eq!(
        decode_a64(0xF208402).unwrap(),
        Instr::ShrnAsimdshfN {
            Q: 0,
            immh: 4,
            immb: 0,
            Rn: 0,
            Rd: 2
        }
    );
    // shrn v3.2s, v0.2d, #0x20
    assert_eq!(
        decode_a64(0xF208403).unwrap(),
        Instr::ShrnAsimdshfN {
            Q: 0,
            immh: 4,
            immb: 0,
            Rn: 0,
            Rd: 3
        }
    );
    // shrn v5.2s, v1.2d, #0x20
    assert_eq!(
        decode_a64(0xF208425).unwrap(),
        Instr::ShrnAsimdshfN {
            Q: 0,
            immh: 4,
            immb: 0,
            Rn: 1,
            Rd: 5
        }
    );
    // shrn v1.2s, v4.2d, #0x20
    assert_eq!(
        decode_a64(0xF208481).unwrap(),
        Instr::ShrnAsimdshfN {
            Q: 0,
            immh: 4,
            immb: 0,
            Rn: 4,
            Rd: 1
        }
    );
    // shrn v17.2s, v6.2d, #0x20
    assert_eq!(
        decode_a64(0xF2084D1).unwrap(),
        Instr::ShrnAsimdshfN {
            Q: 0,
            immh: 4,
            immb: 0,
            Rn: 6,
            Rd: 17
        }
    );
}

#[test]
fn test_sqrshrn() {
    // sqrshrn v0.2s, v0.2d, #0x1e
    assert_eq!(
        decode_a64(0xF229C00).unwrap(),
        Instr::SqrshrnAsimdshfN {
            Q: 0,
            immh: 4,
            immb: 2,
            Rn: 0,
            Rd: 0
        }
    );
    // sqrshrn v0.2s, v0.2d, #0xf
    assert_eq!(
        decode_a64(0xF319C00).unwrap(),
        Instr::SqrshrnAsimdshfN {
            Q: 0,
            immh: 6,
            immb: 1,
            Rn: 0,
            Rd: 0
        }
    );
    // sqrshrn v1.2s, v1.2d, #0xf
    assert_eq!(
        decode_a64(0xF319C21).unwrap(),
        Instr::SqrshrnAsimdshfN {
            Q: 0,
            immh: 6,
            immb: 1,
            Rn: 1,
            Rd: 1
        }
    );
    // sqrshrn v2.2s, v2.2d, #0xf
    assert_eq!(
        decode_a64(0xF319C42).unwrap(),
        Instr::SqrshrnAsimdshfN {
            Q: 0,
            immh: 6,
            immb: 1,
            Rn: 2,
            Rd: 2
        }
    );
    // sqrshrn v6.2s, v2.2d, #0xe
    assert_eq!(
        decode_a64(0xF329C46).unwrap(),
        Instr::SqrshrnAsimdshfN {
            Q: 0,
            immh: 6,
            immb: 2,
            Rn: 2,
            Rd: 6
        }
    );
}

#[test]
fn test_and() {
    // and w2, w8, #1
    assert_eq!(
        decode_a64(0x12000102).unwrap(),
        Instr::And32LogImm {
            immr: 0,
            imms: 0,
            Rn: 8,
            Rd: 2
        }
    );
    // and w11, w0, #3
    assert_eq!(
        decode_a64(0x1200040B).unwrap(),
        Instr::And32LogImm {
            immr: 0,
            imms: 1,
            Rn: 0,
            Rd: 11
        }
    );
    // and w18, w4, #0xffff
    assert_eq!(
        decode_a64(0x12003C92).unwrap(),
        Instr::And32LogImm {
            immr: 0,
            imms: 15,
            Rn: 4,
            Rd: 18
        }
    );
    // and w17, w2, #0xff0000
    assert_eq!(
        decode_a64(0x12101C51).unwrap(),
        Instr::And32LogImm {
            immr: 16,
            imms: 7,
            Rn: 2,
            Rd: 17
        }
    );
    // and w8, w10, #2
    assert_eq!(
        decode_a64(0x121F0148).unwrap(),
        Instr::And32LogImm {
            immr: 31,
            imms: 0,
            Rn: 10,
            Rd: 8
        }
    );
    // and x26, x26, #0xfffffffffffffffe
    assert_eq!(
        decode_a64(0x927FFB5A).unwrap(),
        Instr::And64LogImm {
            immr: 63,
            imms: 62,
            Rn: 26,
            Rd: 26
        }
    );
}

#[test]
fn test_mov() {
    // mov r0, #1
    assert_eq!(
        decode_a32(0xe3a00001).unwrap(),
        Instr::MovsIA1 {
            cond: 14,
            S: 0,
            Rn: 0,
            Rd: 0,
            imm12: 1
        }
    );

    // mov w11, #-0x14
    assert_eq!(
        decode_a64(0x1280026B).unwrap(),
        Instr::Movn32Movewide { imm16: 19, Rd: 11 }
    );
    // mov w9, #-0x1c
    assert_eq!(
        decode_a64(0x12800369).unwrap(),
        Instr::Movn32Movewide { imm16: 27, Rd: 9 }
    );
    // mov w0, #-0xfde8
    assert_eq!(
        decode_a64(0x129FBCE0).unwrap(),
        Instr::Movn32Movewide {
            imm16: 64999,
            Rd: 0
        }
    );
    // mov w10, #0x10ffff
    assert_eq!(
        decode_a64(0x12BFFDEA).unwrap(),
        Instr::Movn32Movewide {
            imm16: 65519,
            Rd: 10
        }
    );
    // mov w14, w15
    assert_eq!(
        decode_a64(0x2A0F03EE).unwrap(),
        Instr::Orr32LogShift {
            shift: 0,
            Rm: 15,
            Rn: 31,
            Rd: 14
        }
    );
    // mov w1, w28
    assert_eq!(
        decode_a64(0x2A1C03E1).unwrap(),
        Instr::Orr32LogShift {
            shift: 0,
            Rm: 28,
            Rn: 31,
            Rd: 1
        }
    );
    // mov v0.s[0], w8
    assert_eq!(
        decode_a64(0x4E041D00).unwrap(),
        Instr::InsAsimdinsIrR { Rn: 8, Rd: 0 }
    );
    // mov v24.16b, v31.16b
    assert_eq!(
        decode_a64(0x4EBF1FF8).unwrap(),
        Instr::OrrAsimdsameOnly {
            Q: 1,
            Rm: 31,
            Rn: 31,
            Rd: 24
        }
    );
    // mov w27, #0x1a
    assert_eq!(
        decode_a64(0x5280035B).unwrap(),
        Instr::Movz32Movewide { imm16: 26, Rd: 27 }
    );
    // mov w3, #0x39e8
    assert_eq!(
        decode_a64(0x52873D03).unwrap(),
        Instr::Movz32Movewide {
            imm16: 14824,
            Rd: 3
        }
    );
    // mov w8, #0x8a48
    assert_eq!(
        decode_a64(0x52914908).unwrap(),
        Instr::Movz32Movewide {
            imm16: 35400,
            Rd: 8
        }
    );
    // mov w8, #0xebe4
    assert_eq!(
        decode_a64(0x529D7C88).unwrap(),
        Instr::Movz32Movewide {
            imm16: 60388,
            Rd: 8
        }
    );
    // mov w10, #0x55550000
    assert_eq!(
        decode_a64(0x52AAAAAA).unwrap(),
        Instr::Movz32Movewide {
            imm16: 21845,
            Rd: 10
        }
    );
    // mov v3.s[0], v2.s[0]
    assert_eq!(
        decode_a64(0x6E040443).unwrap(),
        Instr::InsAsimdinsIvV { Rn: 2, Rd: 3 }
    );
    // mov v16.s[0], v7.s[0]
    assert_eq!(
        decode_a64(0x6E0404F0).unwrap(),
        Instr::InsAsimdinsIvV { Rn: 7, Rd: 16 }
    );
    // mov v31.s[1], v24.s[1]
    assert_eq!(
        decode_a64(0x6E0C271F).unwrap(),
        Instr::InsAsimdinsIvV { Rn: 24, Rd: 31 }
    );
    // mov v4.d[1], v1.d[0]
    assert_eq!(
        decode_a64(0x6E180424).unwrap(),
        Instr::InsAsimdinsIvV { Rn: 1, Rd: 4 }
    );
    // mov v19.s[3], v18.s[3]
    assert_eq!(
        decode_a64(0x6E1C6653).unwrap(),
        Instr::InsAsimdinsIvV { Rn: 18, Rd: 19 }
    );
    // mov x3, sp
    assert_eq!(
        decode_a64(0x910003E3).unwrap(),
        Instr::Add64AddsubImm {
            sh: 0,
            imm12: 0,
            Rn: 31,
            Rd: 3
        }
    );
    // mov x1, #-1
    assert_eq!(
        decode_a64(0x92800001).unwrap(),
        Instr::Movn64Movewide { imm16: 0, Rd: 1 }
    );
    // mov x10, #-0x2711
    assert_eq!(
        decode_a64(0x9284E20A).unwrap(),
        Instr::Movn64Movewide {
            imm16: 10000,
            Rd: 10
        }
    );
    // mov x2, x13
    assert_eq!(
        decode_a64(0xAA0D03E2).unwrap(),
        Instr::Orr64LogShift {
            shift: 0,
            Rm: 13,
            Rn: 31,
            Rd: 2
        }
    );
    // mov x18, x25
    assert_eq!(
        decode_a64(0xAA1903F2).unwrap(),
        Instr::Orr64LogShift {
            shift: 0,
            Rm: 25,
            Rn: 31,
            Rd: 18
        }
    );
    // mov x4, #0
    assert_eq!(
        decode_a64(0xD2800004).unwrap(),
        Instr::Movz64Movewide { imm16: 0, Rd: 4 }
    );
}

#[test]
fn test_sxtb() {
    // sxtb w4, w6
    assert_eq!(
        decode_a64(0x13001CC4).unwrap(),
        Instr::Sbfm32MBitfield {
            immr: 0,
            imms: 7,
            Rn: 6,
            Rd: 4
        }
    );
    // sxtb w9, w8
    assert_eq!(
        decode_a64(0x13001D09).unwrap(),
        Instr::Sbfm32MBitfield {
            immr: 0,
            imms: 7,
            Rn: 8,
            Rd: 9
        }
    );
    // sxtb w17, w8
    assert_eq!(
        decode_a64(0x13001D11).unwrap(),
        Instr::Sbfm32MBitfield {
            immr: 0,
            imms: 7,
            Rn: 8,
            Rd: 17
        }
    );
    // sxtb w18, w15
    assert_eq!(
        decode_a64(0x13001DF2).unwrap(),
        Instr::Sbfm32MBitfield {
            immr: 0,
            imms: 7,
            Rn: 15,
            Rd: 18
        }
    );
    // sxtb x8, w16
    assert_eq!(
        decode_a64(0x93401E08).unwrap(),
        Instr::Sbfm64MBitfield {
            immr: 0,
            imms: 7,
            Rn: 16,
            Rd: 8
        }
    );
    // sxtb x20, w19
    assert_eq!(
        decode_a64(0x93401E74).unwrap(),
        Instr::Sbfm64MBitfield {
            immr: 0,
            imms: 7,
            Rn: 19,
            Rd: 20
        }
    );
}

#[test]
fn test_sxth() {
    // sxth w2, w0
    assert_eq!(
        decode_a64(0x13003C02).unwrap(),
        Instr::Sbfm32MBitfield {
            immr: 0,
            imms: 15,
            Rn: 0,
            Rd: 2
        }
    );
    // sxth w21, w0
    assert_eq!(
        decode_a64(0x13003C15).unwrap(),
        Instr::Sbfm32MBitfield {
            immr: 0,
            imms: 15,
            Rn: 0,
            Rd: 21
        }
    );
    // sxth w5, w3
    assert_eq!(
        decode_a64(0x13003C65).unwrap(),
        Instr::Sbfm32MBitfield {
            immr: 0,
            imms: 15,
            Rn: 3,
            Rd: 5
        }
    );
    // sxth w23, w26
    assert_eq!(
        decode_a64(0x13003F57).unwrap(),
        Instr::Sbfm32MBitfield {
            immr: 0,
            imms: 15,
            Rn: 26,
            Rd: 23
        }
    );
    // sxth w25, w30
    assert_eq!(
        decode_a64(0x13003FD9).unwrap(),
        Instr::Sbfm32MBitfield {
            immr: 0,
            imms: 15,
            Rn: 30,
            Rd: 25
        }
    );
    // sxth x4, w14
    assert_eq!(
        decode_a64(0x93403DC4).unwrap(),
        Instr::Sbfm64MBitfield {
            immr: 0,
            imms: 15,
            Rn: 14,
            Rd: 4
        }
    );
}

#[test]
fn test_sbfx() {
    // sbfx w17, w17, #1, #0xf
    assert_eq!(
        decode_a64(0x13013E31).unwrap(),
        Instr::Sbfm32MBitfield {
            immr: 1,
            imms: 15,
            Rn: 17,
            Rd: 17
        }
    );
    // sbfx w0, w8, #4, #1
    assert_eq!(
        decode_a64(0x13041100).unwrap(),
        Instr::Sbfm32MBitfield {
            immr: 4,
            imms: 4,
            Rn: 8,
            Rd: 0
        }
    );
    // sbfx x23, x2, #1, #0x1f
    assert_eq!(
        decode_a64(0x93417C57).unwrap(),
        Instr::Sbfm64MBitfield {
            immr: 1,
            imms: 31,
            Rn: 2,
            Rd: 23
        }
    );
    // sbfx x7, x3, #1, #0x1f
    assert_eq!(
        decode_a64(0x93417C67).unwrap(),
        Instr::Sbfm64MBitfield {
            immr: 1,
            imms: 31,
            Rn: 3,
            Rd: 7
        }
    );
    // sbfx x18, x10, #3, #0x1d
    assert_eq!(
        decode_a64(0x93437D52).unwrap(),
        Instr::Sbfm64MBitfield {
            immr: 3,
            imms: 31,
            Rn: 10,
            Rd: 18
        }
    );
    // sbfx x11, x11, #0x1e, #0x20
    assert_eq!(
        decode_a64(0x935EF56B).unwrap(),
        Instr::Sbfm64MBitfield {
            immr: 30,
            imms: 61,
            Rn: 11,
            Rd: 11
        }
    );
}

#[test]
fn test_asr() {
    // asr w4, w2, #1
    assert_eq!(
        decode_a64(0x13017C44).unwrap(),
        Instr::Sbfm32MBitfield {
            immr: 1,
            imms: 31,
            Rn: 2,
            Rd: 4
        }
    );
    // asr w2, w2, #3
    assert_eq!(
        decode_a64(0x13037C42).unwrap(),
        Instr::Sbfm32MBitfield {
            immr: 3,
            imms: 31,
            Rn: 2,
            Rd: 2
        }
    );
    // asr w15, w15, #3
    assert_eq!(
        decode_a64(0x13037DEF).unwrap(),
        Instr::Sbfm32MBitfield {
            immr: 3,
            imms: 31,
            Rn: 15,
            Rd: 15
        }
    );
    // asr w23, w23, w1
    assert_eq!(
        decode_a64(0x1AC12AF7).unwrap(),
        Instr::Asrv32Dp2Src {
            Rm: 1,
            Rn: 23,
            Rd: 23
        }
    );
    // asr x10, x10, #0x3f
    assert_eq!(
        decode_a64(0x937FFD4A).unwrap(),
        Instr::Sbfm64MBitfield {
            immr: 63,
            imms: 63,
            Rn: 10,
            Rd: 10
        }
    );
    // asr x14, x8, x11
    assert_eq!(
        decode_a64(0x9ACB290E).unwrap(),
        Instr::Asrv64Dp2Src {
            Rm: 11,
            Rn: 8,
            Rd: 14
        }
    );
}

#[test]
fn test_extr() {
    // extr w4, w3, w2, #8
    assert_eq!(
        decode_a64(0x13822064).unwrap(),
        Instr::Extr32Extract {
            Rm: 2,
            Rn: 3,
            Rd: 4
        }
    );
    // extr w10, w13, w10, #0x18
    assert_eq!(
        decode_a64(0x138A61AA).unwrap(),
        Instr::Extr32Extract {
            Rm: 10,
            Rn: 13,
            Rd: 10
        }
    );
    // extr x2, x4, x2, #0x20
    assert_eq!(
        decode_a64(0x93C28082).unwrap(),
        Instr::Extr64Extract {
            Rm: 2,
            Rn: 4,
            Rd: 2
        }
    );
    // extr x10, x9, x8, #0x28
    assert_eq!(
        decode_a64(0x93C8A12A).unwrap(),
        Instr::Extr64Extract {
            Rm: 8,
            Rn: 9,
            Rd: 10
        }
    );
    // extr x12, x21, x23, #0x3f
    assert_eq!(
        decode_a64(0x93D7FEAC).unwrap(),
        Instr::Extr64Extract {
            Rm: 23,
            Rn: 21,
            Rd: 12
        }
    );
    // extr x10, x21, x24, #0x3e
    assert_eq!(
        decode_a64(0x93D8FAAA).unwrap(),
        Instr::Extr64Extract {
            Rm: 24,
            Rn: 21,
            Rd: 10
        }
    );
}

#[test]
fn test_ror() {
    // ror w10, w10, #0x11
    assert_eq!(
        decode_a64(0x138A454A).unwrap(),
        Instr::Extr32Extract {
            Rm: 10,
            Rn: 10,
            Rd: 10
        }
    );
    // ror w10, w10, #0x19
    assert_eq!(
        decode_a64(0x138A654A).unwrap(),
        Instr::Extr32Extract {
            Rm: 10,
            Rn: 10,
            Rd: 10
        }
    );
    // ror w14, w14, #0xb
    assert_eq!(
        decode_a64(0x138E2DCE).unwrap(),
        Instr::Extr32Extract {
            Rm: 14,
            Rn: 14,
            Rd: 14
        }
    );
    // ror w17, w17, #0x1a
    assert_eq!(
        decode_a64(0x13916A31).unwrap(),
        Instr::Extr32Extract {
            Rm: 17,
            Rn: 17,
            Rd: 17
        }
    );
    // ror w18, w18, #0xb
    assert_eq!(
        decode_a64(0x13922E52).unwrap(),
        Instr::Extr32Extract {
            Rm: 18,
            Rn: 18,
            Rd: 18
        }
    );
    // ror w5, w1, w3
    assert_eq!(
        decode_a64(0x1AC32C25).unwrap(),
        Instr::Rorv32Dp2Src {
            Rm: 3,
            Rn: 1,
            Rd: 5
        }
    );
}

#[test]
fn test_cinc() {
    // cinc w8, w0, eq
    assert_eq!(
        decode_a64(0x1A801408).unwrap(),
        Instr::Csinc32Condsel {
            Rm: 0,
            cond: 1,
            Rn: 0,
            Rd: 8
        }
    );
    // cinc w12, w10, lt
    assert_eq!(
        decode_a64(0x1A8AA54C).unwrap(),
        Instr::Csinc32Condsel {
            Rm: 10,
            cond: 10,
            Rn: 10,
            Rd: 12
        }
    );
    // cinc w8, w17, lt
    assert_eq!(
        decode_a64(0x1A91A628).unwrap(),
        Instr::Csinc32Condsel {
            Rm: 17,
            cond: 10,
            Rn: 17,
            Rd: 8
        }
    );
    // cinc w19, w19, ne
    assert_eq!(
        decode_a64(0x1A930673).unwrap(),
        Instr::Csinc32Condsel {
            Rm: 19,
            cond: 0,
            Rn: 19,
            Rd: 19
        }
    );
    // cinc w5, w23, ne
    assert_eq!(
        decode_a64(0x1A9706E5).unwrap(),
        Instr::Csinc32Condsel {
            Rm: 23,
            cond: 0,
            Rn: 23,
            Rd: 5
        }
    );
    // cinc x7, x2, lt
    assert_eq!(
        decode_a64(0x9A82A447).unwrap(),
        Instr::Csinc64Condsel {
            Rm: 2,
            cond: 10,
            Rn: 2,
            Rd: 7
        }
    );
}

#[test]
fn test_csel() {
    // csel w0, w2, w0, lo
    assert_eq!(
        decode_a64(0x1A803040).unwrap(),
        Instr::Csel32Condsel {
            Rm: 0,
            cond: 3,
            Rn: 2,
            Rd: 0
        }
    );
    // csel w4, w25, w4, lo
    assert_eq!(
        decode_a64(0x1A843324).unwrap(),
        Instr::Csel32Condsel {
            Rm: 4,
            cond: 3,
            Rn: 25,
            Rd: 4
        }
    );
    // csel w0, wzr, w9, eq
    assert_eq!(
        decode_a64(0x1A8903E0).unwrap(),
        Instr::Csel32Condsel {
            Rm: 9,
            cond: 0,
            Rn: 31,
            Rd: 0
        }
    );
    // csel w30, wzr, w17, lt
    assert_eq!(
        decode_a64(0x1A91B3FE).unwrap(),
        Instr::Csel32Condsel {
            Rm: 17,
            cond: 11,
            Rn: 31,
            Rd: 30
        }
    );
    // csel w16, w16, w18, hi
    assert_eq!(
        decode_a64(0x1A928210).unwrap(),
        Instr::Csel32Condsel {
            Rm: 18,
            cond: 8,
            Rn: 16,
            Rd: 16
        }
    );
    // csel x9, x8, x10, ne
    assert_eq!(
        decode_a64(0x9A8A1109).unwrap(),
        Instr::Csel64Condsel {
            Rm: 10,
            cond: 1,
            Rn: 8,
            Rd: 9
        }
    );
    // csel x1, x25, x23, eq
    assert_eq!(
        decode_a64(0x9A970321).unwrap(),
        Instr::Csel64Condsel {
            Rm: 23,
            cond: 0,
            Rn: 25,
            Rd: 1
        }
    );
}

#[test]
fn test_csinc() {
    // csinc w12, w13, w8, lt
    assert_eq!(
        decode_a64(0x1A88B5AC).unwrap(),
        Instr::Csinc32Condsel {
            Rm: 8,
            cond: 11,
            Rn: 13,
            Rd: 12
        }
    );
    // csinc w9, w15, w9, lt
    assert_eq!(
        decode_a64(0x1A89B5E9).unwrap(),
        Instr::Csinc32Condsel {
            Rm: 9,
            cond: 11,
            Rn: 15,
            Rd: 9
        }
    );
    // csinc w10, wzr, w10, ge
    assert_eq!(
        decode_a64(0x1A8AA7EA).unwrap(),
        Instr::Csinc32Condsel {
            Rm: 10,
            cond: 10,
            Rn: 31,
            Rd: 10
        }
    );
    // csinc x6, x9, x0, ne
    assert_eq!(
        decode_a64(0x9A801526).unwrap(),
        Instr::Csinc64Condsel {
            Rm: 0,
            cond: 1,
            Rn: 9,
            Rd: 6
        }
    );
    // csinc x1, x9, x11, ne
    assert_eq!(
        decode_a64(0x9A8B1521).unwrap(),
        Instr::Csinc64Condsel {
            Rm: 11,
            cond: 1,
            Rn: 9,
            Rd: 1
        }
    );
    // csinc x8, x9, x25, ne
    assert_eq!(
        decode_a64(0x9A991528).unwrap(),
        Instr::Csinc64Condsel {
            Rm: 25,
            cond: 1,
            Rn: 9,
            Rd: 8
        }
    );
}

#[test]
fn test_cset() {
    // cset w13, lo
    assert_eq!(
        decode_a64(0x1A9F27ED).unwrap(),
        Instr::Csinc32Condsel {
            Rm: 31,
            cond: 2,
            Rn: 31,
            Rd: 13
        }
    );
    // cset w0, mi
    assert_eq!(
        decode_a64(0x1A9F57E0).unwrap(),
        Instr::Csinc32Condsel {
            Rm: 31,
            cond: 5,
            Rn: 31,
            Rd: 0
        }
    );
    // cset w4, hi
    assert_eq!(
        decode_a64(0x1A9F97E4).unwrap(),
        Instr::Csinc32Condsel {
            Rm: 31,
            cond: 9,
            Rn: 31,
            Rd: 4
        }
    );
    // cset w26, lt
    assert_eq!(
        decode_a64(0x1A9FA7FA).unwrap(),
        Instr::Csinc32Condsel {
            Rm: 31,
            cond: 10,
            Rn: 31,
            Rd: 26
        }
    );
    // cset w18, ge
    assert_eq!(
        decode_a64(0x1A9FB7F2).unwrap(),
        Instr::Csinc32Condsel {
            Rm: 31,
            cond: 11,
            Rn: 31,
            Rd: 18
        }
    );
    // cset w28, gt
    assert_eq!(
        decode_a64(0x1A9FD7FC).unwrap(),
        Instr::Csinc32Condsel {
            Rm: 31,
            cond: 13,
            Rn: 31,
            Rd: 28
        }
    );
}

#[test]
fn test_lsr() {
    // lsr w7, w2, w0
    assert_eq!(
        decode_a64(0x1AC02447).unwrap(),
        Instr::Lsrv32Dp2Src {
            Rm: 0,
            Rn: 2,
            Rd: 7
        }
    );
    // lsr w11, w9, #1
    assert_eq!(
        decode_a64(0x53017D2B).unwrap(),
        Instr::Ubfm32MBitfield {
            immr: 1,
            imms: 31,
            Rn: 9,
            Rd: 11
        }
    );
    // lsr w19, w19, #7
    assert_eq!(
        decode_a64(0x53077E73).unwrap(),
        Instr::Ubfm32MBitfield {
            immr: 7,
            imms: 31,
            Rn: 19,
            Rd: 19
        }
    );
    // lsr x19, x10, #0x16
    assert_eq!(
        decode_a64(0xD356FD53).unwrap(),
        Instr::Ubfm64MBitfield {
            immr: 22,
            imms: 63,
            Rn: 10,
            Rd: 19
        }
    );
    // lsr x26, x27, #0x1f
    assert_eq!(
        decode_a64(0xD35FFF7A).unwrap(),
        Instr::Ubfm64MBitfield {
            immr: 31,
            imms: 63,
            Rn: 27,
            Rd: 26
        }
    );
    // lsr x28, x19, #0x3f
    assert_eq!(
        decode_a64(0xD37FFE7C).unwrap(),
        Instr::Ubfm64MBitfield {
            immr: 63,
            imms: 63,
            Rn: 19,
            Rd: 28
        }
    );
}

#[test]
fn test_sdiv() {
    // sdiv w8, w2, w1
    assert_eq!(
        decode_a64(0x1AC10C48).unwrap(),
        Instr::Sdiv32Dp2Src {
            Rm: 1,
            Rn: 2,
            Rd: 8
        }
    );
    // sdiv w12, w10, w8
    assert_eq!(
        decode_a64(0x1AC80D4C).unwrap(),
        Instr::Sdiv32Dp2Src {
            Rm: 8,
            Rn: 10,
            Rd: 12
        }
    );
    // sdiv w0, w8, w9
    assert_eq!(
        decode_a64(0x1AC90D00).unwrap(),
        Instr::Sdiv32Dp2Src {
            Rm: 9,
            Rn: 8,
            Rd: 0
        }
    );
    // sdiv w12, w13, w12
    assert_eq!(
        decode_a64(0x1ACC0DAC).unwrap(),
        Instr::Sdiv32Dp2Src {
            Rm: 12,
            Rn: 13,
            Rd: 12
        }
    );
    // sdiv w9, w10, w24
    assert_eq!(
        decode_a64(0x1AD80D49).unwrap(),
        Instr::Sdiv32Dp2Src {
            Rm: 24,
            Rn: 10,
            Rd: 9
        }
    );
    // sdiv x10, x11, x10
    assert_eq!(
        decode_a64(0x9ACA0D6A).unwrap(),
        Instr::Sdiv64Dp2Src {
            Rm: 10,
            Rn: 11,
            Rd: 10
        }
    );
}

#[test]
fn test_udiv() {
    // udiv w5, w5, w7
    assert_eq!(
        decode_a64(0x1AC708A5).unwrap(),
        Instr::Udiv32Dp2Src {
            Rm: 7,
            Rn: 5,
            Rd: 5
        }
    );
    // udiv w1, w1, w9
    assert_eq!(
        decode_a64(0x1AC90821).unwrap(),
        Instr::Udiv32Dp2Src {
            Rm: 9,
            Rn: 1,
            Rd: 1
        }
    );
    // udiv w15, w19, w10
    assert_eq!(
        decode_a64(0x1ACA0A6F).unwrap(),
        Instr::Udiv32Dp2Src {
            Rm: 10,
            Rn: 19,
            Rd: 15
        }
    );
    // udiv w12, w13, w12
    assert_eq!(
        decode_a64(0x1ACC09AC).unwrap(),
        Instr::Udiv32Dp2Src {
            Rm: 12,
            Rn: 13,
            Rd: 12
        }
    );
    // udiv w18, w18, w17
    assert_eq!(
        decode_a64(0x1AD10A52).unwrap(),
        Instr::Udiv32Dp2Src {
            Rm: 17,
            Rn: 18,
            Rd: 18
        }
    );
    // udiv w14, w12, w21
    assert_eq!(
        decode_a64(0x1AD5098E).unwrap(),
        Instr::Udiv32Dp2Src {
            Rm: 21,
            Rn: 12,
            Rd: 14
        }
    );
    // udiv x10, x22, x9
    assert_eq!(
        decode_a64(0x9AC90ACA).unwrap(),
        Instr::Udiv64Dp2Src {
            Rm: 9,
            Rn: 22,
            Rd: 10
        }
    );
}

#[test]
fn test_lsl() {
    // lsl w27, w8, w26
    assert_eq!(
        decode_a64(0x1ADA211B).unwrap(),
        Instr::Lslv32Dp2Src {
            Rm: 26,
            Rn: 8,
            Rd: 27
        }
    );
    // lsl w27, w8, w28
    assert_eq!(
        decode_a64(0x1ADC211B).unwrap(),
        Instr::Lslv32Dp2Src {
            Rm: 28,
            Rn: 8,
            Rd: 27
        }
    );
    // lsl w30, w27, #0x10
    assert_eq!(
        decode_a64(0x53103F7E).unwrap(),
        Instr::Ubfm32MBitfield {
            immr: 16,
            imms: 15,
            Rn: 27,
            Rd: 30
        }
    );
    // lsl w25, w20, #1
    assert_eq!(
        decode_a64(0x531F7A99).unwrap(),
        Instr::Ubfm32MBitfield {
            immr: 31,
            imms: 30,
            Rn: 20,
            Rd: 25
        }
    );
    // lsl x9, x9, x0
    assert_eq!(
        decode_a64(0x9AC02129).unwrap(),
        Instr::Lslv64Dp2Src {
            Rm: 0,
            Rn: 9,
            Rd: 9
        }
    );
    // lsl x9, x25, #3
    assert_eq!(
        decode_a64(0xD37DF329).unwrap(),
        Instr::Ubfm64MBitfield {
            immr: 61,
            imms: 60,
            Rn: 25,
            Rd: 9
        }
    );
}

#[test]
fn test_msub() {
    // msub w0, w2, w0, w1
    assert_eq!(
        decode_a64(0x1B008440).unwrap(),
        Instr::Msub32ADp3Src {
            Rm: 0,
            Ra: 1,
            Rn: 2,
            Rd: 0
        }
    );
    // msub w9, w12, w8, w9
    assert_eq!(
        decode_a64(0x1B08A589).unwrap(),
        Instr::Msub32ADp3Src {
            Rm: 8,
            Ra: 9,
            Rn: 12,
            Rd: 9
        }
    );
    // msub w10, w14, w10, w13
    assert_eq!(
        decode_a64(0x1B0AB5CA).unwrap(),
        Instr::Msub32ADp3Src {
            Rm: 10,
            Ra: 13,
            Rn: 14,
            Rd: 10
        }
    );
    // msub w14, w14, w13, w9
    assert_eq!(
        decode_a64(0x1B0DA5CE).unwrap(),
        Instr::Msub32ADp3Src {
            Rm: 13,
            Ra: 9,
            Rn: 14,
            Rd: 14
        }
    );
    // msub w15, w18, w15, w19
    assert_eq!(
        decode_a64(0x1B0FCE4F).unwrap(),
        Instr::Msub32ADp3Src {
            Rm: 15,
            Ra: 19,
            Rn: 18,
            Rd: 15
        }
    );
    // msub w9, w9, w26, w8
    assert_eq!(
        decode_a64(0x1B1AA129).unwrap(),
        Instr::Msub32ADp3Src {
            Rm: 26,
            Ra: 8,
            Rn: 9,
            Rd: 9
        }
    );
}

#[test]
fn test_madd() {
    // madd w0, w1, w1, w0
    assert_eq!(
        decode_a64(0x1B010020).unwrap(),
        Instr::Madd32ADp3Src {
            Rm: 1,
            Ra: 0,
            Rn: 1,
            Rd: 0
        }
    );
    // madd w6, w6, w10, w19
    assert_eq!(
        decode_a64(0x1B0A4CC6).unwrap(),
        Instr::Madd32ADp3Src {
            Rm: 10,
            Ra: 19,
            Rn: 6,
            Rd: 6
        }
    );
    // madd w8, w14, w14, w8
    assert_eq!(
        decode_a64(0x1B0E21C8).unwrap(),
        Instr::Madd32ADp3Src {
            Rm: 14,
            Ra: 8,
            Rn: 14,
            Rd: 8
        }
    );
    // madd w14, w14, w26, w16
    assert_eq!(
        decode_a64(0x1B1A41CE).unwrap(),
        Instr::Madd32ADp3Src {
            Rm: 26,
            Ra: 16,
            Rn: 14,
            Rd: 14
        }
    );
    // madd x2, x23, x8, x0
    assert_eq!(
        decode_a64(0x9B0802E2).unwrap(),
        Instr::Madd64ADp3Src {
            Rm: 8,
            Ra: 0,
            Rn: 23,
            Rd: 2
        }
    );
    // madd x16, x10, x16, x15
    assert_eq!(
        decode_a64(0x9B103D50).unwrap(),
        Instr::Madd64ADp3Src {
            Rm: 16,
            Ra: 15,
            Rn: 10,
            Rd: 16
        }
    );
}

#[test]
fn test_mul() {
    // mul w3, w12, w3
    assert_eq!(
        decode_a64(0x1B037D83).unwrap(),
        Instr::Madd32ADp3Src {
            Rm: 3,
            Ra: 31,
            Rn: 12,
            Rd: 3
        }
    );
    // mul w8, w12, w13
    assert_eq!(
        decode_a64(0x1B0D7D88).unwrap(),
        Instr::Madd32ADp3Src {
            Rm: 13,
            Ra: 31,
            Rn: 12,
            Rd: 8
        }
    );
    // mul v0.4s, v0.4s, v0.s[1]
    assert_eq!(
        decode_a64(0x4FA08000).unwrap(),
        Instr::MulAsimdelemR {
            Q: 1,
            L: 1,
            M: 0,
            Rm: 0,
            H: 0,
            Rn: 0,
            Rd: 0
        }
    );
    // mul x0, x28, x0
    assert_eq!(
        decode_a64(0x9B007F80).unwrap(),
        Instr::Madd64ADp3Src {
            Rm: 0,
            Ra: 31,
            Rn: 28,
            Rd: 0
        }
    );
    // mul x0, x0, x4
    assert_eq!(
        decode_a64(0x9B047C00).unwrap(),
        Instr::Madd64ADp3Src {
            Rm: 4,
            Ra: 31,
            Rn: 0,
            Rd: 0
        }
    );
    // mul x19, x21, x9
    assert_eq!(
        decode_a64(0x9B097EB3).unwrap(),
        Instr::Madd64ADp3Src {
            Rm: 9,
            Ra: 31,
            Rn: 21,
            Rd: 19
        }
    );
}

#[test]
fn test_fcvtzu() {
    // fcvtzu w16, s0, #0x10
    assert_eq!(
        decode_a64(0x1E19C010).unwrap(),
        Instr::Fcvtzu32SFloat2Fix { Rn: 0, Rd: 16 }
    );
    // fcvtzu w3, s0
    assert_eq!(
        decode_a64(0x1E390003).unwrap(),
        Instr::Fcvtzu32SFloat2Int { Rn: 0, Rd: 3 }
    );
    // fcvtzu w10, s0
    assert_eq!(
        decode_a64(0x1E39000A).unwrap(),
        Instr::Fcvtzu32SFloat2Int { Rn: 0, Rd: 10 }
    );
    // fcvtzu w11, s8
    assert_eq!(
        decode_a64(0x1E39010B).unwrap(),
        Instr::Fcvtzu32SFloat2Int { Rn: 8, Rd: 11 }
    );
    // fcvtzu x9, d0, #0xe
    assert_eq!(
        decode_a64(0x9E59C809).unwrap(),
        Instr::Fcvtzu64DFloat2Fix { Rn: 0, Rd: 9 }
    );
    // fcvtzu x0, d0
    assert_eq!(
        decode_a64(0x9E790000).unwrap(),
        Instr::Fcvtzu64DFloat2Int { Rn: 0, Rd: 0 }
    );
}

#[test]
fn test_fmul() {
    // fmul s6, s0, s0
    assert_eq!(
        decode_a64(0x1E200806).unwrap(),
        Instr::FmulSFloatdp2 {
            Rm: 0,
            Rn: 0,
            Rd: 6
        }
    );
    // fmul s16, s15, s2
    assert_eq!(
        decode_a64(0x1E2209F0).unwrap(),
        Instr::FmulSFloatdp2 {
            Rm: 2,
            Rn: 15,
            Rd: 16
        }
    );
    // fmul s7, s13, s3
    assert_eq!(
        decode_a64(0x1E2309A7).unwrap(),
        Instr::FmulSFloatdp2 {
            Rm: 3,
            Rn: 13,
            Rd: 7
        }
    );
    // fmul s0, s0, s5
    assert_eq!(
        decode_a64(0x1E250800).unwrap(),
        Instr::FmulSFloatdp2 {
            Rm: 5,
            Rn: 0,
            Rd: 0
        }
    );
    // fmul s20, s6, s24
    assert_eq!(
        decode_a64(0x1E3808D4).unwrap(),
        Instr::FmulSFloatdp2 {
            Rm: 24,
            Rn: 6,
            Rd: 20
        }
    );
    // fmul v31.4s, v21.4s, v31.s[0]
    assert_eq!(
        decode_a64(0x4F9F92BF).unwrap(),
        Instr::FmulAsimdelemRSd {
            Q: 1,
            L: 0,
            M: 1,
            Rm: 15,
            H: 0,
            Rn: 21,
            Rd: 31
        }
    );
}

#[test]
fn test_fmov() {
    // fmov s0, #2.00000000
    assert_eq!(
        decode_a64(0x1E201000).unwrap(),
        Instr::FmovSFloatimm { imm8: 0, Rd: 0 }
    );
    // fmov s0, #6.00000000
    assert_eq!(
        decode_a64(0x1E231000).unwrap(),
        Instr::FmovSFloatimm { imm8: 24, Rd: 0 }
    );
    // fmov w9, s0
    assert_eq!(
        decode_a64(0x1E260009).unwrap(),
        Instr::Fmov32SFloat2Int { Rn: 0, Rd: 9 }
    );
    // fmov w0, s8
    assert_eq!(
        decode_a64(0x1E260100).unwrap(),
        Instr::Fmov32SFloat2Int { Rn: 8, Rd: 0 }
    );
    // fmov s13, w27
    assert_eq!(
        decode_a64(0x1E27036D).unwrap(),
        Instr::FmovS32Float2Int { Rn: 27, Rd: 13 }
    );
    // fmov v27.4s, #-1.00000000
    assert_eq!(
        decode_a64(0x4F07F61B).unwrap(),
        Instr::FmovAsimdimmSS {
            a: 1,
            b: 1,
            c: 1,
            d: 1,
            e: 0,
            f: 0,
            g: 0,
            h: 0,
            Rd: 27
        }
    );
    // fmov x11, d8
    assert_eq!(
        decode_a64(0x9E66010B).unwrap(),
        Instr::Fmov64DFloat2Int { Rn: 8, Rd: 11 }
    );
}

#[test]
fn test_fdiv() {
    // fdiv s5, s2, s0
    assert_eq!(
        decode_a64(0x1E201845).unwrap(),
        Instr::FdivSFloatdp2 {
            Rm: 0,
            Rn: 2,
            Rd: 5
        }
    );
    // fdiv s2, s7, s0
    assert_eq!(
        decode_a64(0x1E2018E2).unwrap(),
        Instr::FdivSFloatdp2 {
            Rm: 0,
            Rn: 7,
            Rd: 2
        }
    );
    // fdiv s20, s7, s3
    assert_eq!(
        decode_a64(0x1E2318F4).unwrap(),
        Instr::FdivSFloatdp2 {
            Rm: 3,
            Rn: 7,
            Rd: 20
        }
    );
    // fdiv s15, s0, s11
    assert_eq!(
        decode_a64(0x1E2B180F).unwrap(),
        Instr::FdivSFloatdp2 {
            Rm: 11,
            Rn: 0,
            Rd: 15
        }
    );
    // fdiv s21, s22, s21
    assert_eq!(
        decode_a64(0x1E351AD5).unwrap(),
        Instr::FdivSFloatdp2 {
            Rm: 21,
            Rn: 22,
            Rd: 21
        }
    );
    // fdiv d10, d5, d6
    assert_eq!(
        decode_a64(0x1E6618AA).unwrap(),
        Instr::FdivDFloatdp2 {
            Rm: 6,
            Rn: 5,
            Rd: 10
        }
    );
}

#[test]
fn test_fcmp() {
    // fcmp s1, s0
    assert_eq!(
        decode_a64(0x1E202020).unwrap(),
        Instr::FcmpSFloatcmp { Rm: 0, Rn: 1 }
    );
    // fcmp s25, #0.0
    assert_eq!(
        decode_a64(0x1E202328).unwrap(),
        Instr::FcmpSzFloatcmp { Rm: 0, Rn: 25 }
    );
    // fcmp s17, s8
    assert_eq!(
        decode_a64(0x1E282220).unwrap(),
        Instr::FcmpSFloatcmp { Rm: 8, Rn: 17 }
    );
    // fcmp s0, s13
    assert_eq!(
        decode_a64(0x1E2D2000).unwrap(),
        Instr::FcmpSFloatcmp { Rm: 13, Rn: 0 }
    );
    // fcmp s3, s14
    assert_eq!(
        decode_a64(0x1E2E2060).unwrap(),
        Instr::FcmpSFloatcmp { Rm: 14, Rn: 3 }
    );
    // fcmp d11, #0.0
    assert_eq!(
        decode_a64(0x1E602168).unwrap(),
        Instr::FcmpDzFloatcmp { Rm: 0, Rn: 11 }
    );
}

#[test]
fn test_fsub() {
    // fsub s4, s1, s0
    assert_eq!(
        decode_a64(0x1E203824).unwrap(),
        Instr::FsubSFloatdp2 {
            Rm: 0,
            Rn: 1,
            Rd: 4
        }
    );
    // fsub s2, s4, s8
    assert_eq!(
        decode_a64(0x1E283882).unwrap(),
        Instr::FsubSFloatdp2 {
            Rm: 8,
            Rn: 4,
            Rd: 2
        }
    );
    // fsub s3, s0, s11
    assert_eq!(
        decode_a64(0x1E2B3803).unwrap(),
        Instr::FsubSFloatdp2 {
            Rm: 11,
            Rn: 0,
            Rd: 3
        }
    );
    // fsub s3, s18, s17
    assert_eq!(
        decode_a64(0x1E313A43).unwrap(),
        Instr::FsubSFloatdp2 {
            Rm: 17,
            Rn: 18,
            Rd: 3
        }
    );
    // fsub v4.4s, v4.4s, v3.4s
    assert_eq!(
        decode_a64(0x4EA3D484).unwrap(),
        Instr::FsubAsimdsameOnly {
            Q: 1,
            Rm: 3,
            Rn: 4,
            Rd: 4
        }
    );
    // fsub v21.4s, v26.4s, v30.4s
    assert_eq!(
        decode_a64(0x4EBED755).unwrap(),
        Instr::FsubAsimdsameOnly {
            Q: 1,
            Rm: 30,
            Rn: 26,
            Rd: 21
        }
    );
}

#[test]
fn test_fccmp() {
    // fccmp s8, s0, #4, mi
    assert_eq!(
        decode_a64(0x1E204504).unwrap(),
        Instr::FccmpSFloatccmp {
            Rm: 0,
            cond: 4,
            Rn: 8,
            nzcv: 4
        }
    );
    // fccmp s2, s0, #0, le
    assert_eq!(
        decode_a64(0x1E20D440).unwrap(),
        Instr::FccmpSFloatccmp {
            Rm: 0,
            cond: 13,
            Rn: 2,
            nzcv: 0
        }
    );
    // fccmp s3, s7, #8, pl
    assert_eq!(
        decode_a64(0x1E275468).unwrap(),
        Instr::FccmpSFloatccmp {
            Rm: 7,
            cond: 5,
            Rn: 3,
            nzcv: 8
        }
    );
    // fccmp s1, s9, #8, ls
    assert_eq!(
        decode_a64(0x1E299428).unwrap(),
        Instr::FccmpSFloatccmp {
            Rm: 9,
            cond: 9,
            Rn: 1,
            nzcv: 8
        }
    );
    // fccmp s0, s9, #2, ge
    assert_eq!(
        decode_a64(0x1E29A402).unwrap(),
        Instr::FccmpSFloatccmp {
            Rm: 9,
            cond: 10,
            Rn: 0,
            nzcv: 2
        }
    );
    // fccmp s28, s23, #8, ls
    assert_eq!(
        decode_a64(0x1E379788).unwrap(),
        Instr::FccmpSFloatccmp {
            Rm: 23,
            cond: 9,
            Rn: 28,
            nzcv: 8
        }
    );
}

#[test]
fn test_fmax() {
    // fmax s1, s1, s0
    assert_eq!(
        decode_a64(0x1E204821).unwrap(),
        Instr::FmaxSFloatdp2 {
            Rm: 0,
            Rn: 1,
            Rd: 1
        }
    );
    // fmax s4, s0, s1
    assert_eq!(
        decode_a64(0x1E214804).unwrap(),
        Instr::FmaxSFloatdp2 {
            Rm: 1,
            Rn: 0,
            Rd: 4
        }
    );
    // fmax s6, s1, s2
    assert_eq!(
        decode_a64(0x1E224826).unwrap(),
        Instr::FmaxSFloatdp2 {
            Rm: 2,
            Rn: 1,
            Rd: 6
        }
    );
    // fmax s1, s1, s12
    assert_eq!(
        decode_a64(0x1E2C4821).unwrap(),
        Instr::FmaxSFloatdp2 {
            Rm: 12,
            Rn: 1,
            Rd: 1
        }
    );
    // fmax s0, s0, s15
    assert_eq!(
        decode_a64(0x1E2F4800).unwrap(),
        Instr::FmaxSFloatdp2 {
            Rm: 15,
            Rn: 0,
            Rd: 0
        }
    );
    // fmax s22, s20, s21
    assert_eq!(
        decode_a64(0x1E354A96).unwrap(),
        Instr::FmaxSFloatdp2 {
            Rm: 21,
            Rn: 20,
            Rd: 22
        }
    );
}

#[test]
fn test_fcsel() {
    // fcsel s0, s1, s0, mi
    assert_eq!(
        decode_a64(0x1E204C20).unwrap(),
        Instr::FcselSFloatsel {
            Rm: 0,
            cond: 4,
            Rn: 1,
            Rd: 0
        }
    );
    // fcsel s3, s10, s3, gt
    assert_eq!(
        decode_a64(0x1E23CD43).unwrap(),
        Instr::FcselSFloatsel {
            Rm: 3,
            cond: 12,
            Rn: 10,
            Rd: 3
        }
    );
    // fcsel s3, s1, s4, gt
    assert_eq!(
        decode_a64(0x1E24CC23).unwrap(),
        Instr::FcselSFloatsel {
            Rm: 4,
            cond: 12,
            Rn: 1,
            Rd: 3
        }
    );
    // fcsel s2, s1, s13, eq
    assert_eq!(
        decode_a64(0x1E2D0C22).unwrap(),
        Instr::FcselSFloatsel {
            Rm: 13,
            cond: 0,
            Rn: 1,
            Rd: 2
        }
    );
    // fcsel s20, s20, s21, gt
    assert_eq!(
        decode_a64(0x1E35CE94).unwrap(),
        Instr::FcselSFloatsel {
            Rm: 21,
            cond: 12,
            Rn: 20,
            Rd: 20
        }
    );
    // fcsel d3, d20, d3, vs
    assert_eq!(
        decode_a64(0x1E636E83).unwrap(),
        Instr::FcselDFloatsel {
            Rm: 3,
            cond: 6,
            Rn: 20,
            Rd: 3
        }
    );
}

#[test]
fn test_fmin() {
    // fmin s0, s1, s0
    assert_eq!(
        decode_a64(0x1E205820).unwrap(),
        Instr::FminSFloatdp2 {
            Rm: 0,
            Rn: 1,
            Rd: 0
        }
    );
    // fmin s2, s3, s2
    assert_eq!(
        decode_a64(0x1E225862).unwrap(),
        Instr::FminSFloatdp2 {
            Rm: 2,
            Rn: 3,
            Rd: 2
        }
    );
    // fmin s9, s1, s5
    assert_eq!(
        decode_a64(0x1E255829).unwrap(),
        Instr::FminSFloatdp2 {
            Rm: 5,
            Rn: 1,
            Rd: 9
        }
    );
    // fmin s2, s11, s10
    assert_eq!(
        decode_a64(0x1E2A5962).unwrap(),
        Instr::FminSFloatdp2 {
            Rm: 10,
            Rn: 11,
            Rd: 2
        }
    );
    // fmin s0, s0, s15
    assert_eq!(
        decode_a64(0x1E2F5800).unwrap(),
        Instr::FminSFloatdp2 {
            Rm: 15,
            Rn: 0,
            Rd: 0
        }
    );
    // fmin s2, s17, s16
    assert_eq!(
        decode_a64(0x1E305A22).unwrap(),
        Instr::FminSFloatdp2 {
            Rm: 16,
            Rn: 17,
            Rd: 2
        }
    );
}

#[test]
fn test_fmaxnm() {
    // fmaxnm s0, s3, s0
    assert_eq!(
        decode_a64(0x1E206860).unwrap(),
        Instr::FmaxnmSFloatdp2 {
            Rm: 0,
            Rn: 3,
            Rd: 0
        }
    );
    // fmaxnm s1, s8, s0
    assert_eq!(
        decode_a64(0x1E206901).unwrap(),
        Instr::FmaxnmSFloatdp2 {
            Rm: 0,
            Rn: 8,
            Rd: 1
        }
    );
    // fmaxnm s8, s8, s0
    assert_eq!(
        decode_a64(0x1E206908).unwrap(),
        Instr::FmaxnmSFloatdp2 {
            Rm: 0,
            Rn: 8,
            Rd: 8
        }
    );
    // fmaxnm s2, s9, s2
    assert_eq!(
        decode_a64(0x1E226922).unwrap(),
        Instr::FmaxnmSFloatdp2 {
            Rm: 2,
            Rn: 9,
            Rd: 2
        }
    );
    // fmaxnm s2, s2, s17
    assert_eq!(
        decode_a64(0x1E316842).unwrap(),
        Instr::FmaxnmSFloatdp2 {
            Rm: 17,
            Rn: 2,
            Rd: 2
        }
    );
    // fmaxnm s18, s18, s20
    assert_eq!(
        decode_a64(0x1E346A52).unwrap(),
        Instr::FmaxnmSFloatdp2 {
            Rm: 20,
            Rn: 18,
            Rd: 18
        }
    );
}

#[test]
fn test_fminnm() {
    // fminnm s1, s1, s0
    assert_eq!(
        decode_a64(0x1E207821).unwrap(),
        Instr::FminnmSFloatdp2 {
            Rm: 0,
            Rn: 1,
            Rd: 1
        }
    );
    // fminnm s1, s8, s1
    assert_eq!(
        decode_a64(0x1E217901).unwrap(),
        Instr::FminnmSFloatdp2 {
            Rm: 1,
            Rn: 8,
            Rd: 1
        }
    );
    // fminnm s9, s0, s2
    assert_eq!(
        decode_a64(0x1E227809).unwrap(),
        Instr::FminnmSFloatdp2 {
            Rm: 2,
            Rn: 0,
            Rd: 9
        }
    );
    // fminnm s1, s0, s10
    assert_eq!(
        decode_a64(0x1E2A7801).unwrap(),
        Instr::FminnmSFloatdp2 {
            Rm: 10,
            Rn: 0,
            Rd: 1
        }
    );
    // fminnm s3, s16, s17
    assert_eq!(
        decode_a64(0x1E317A03).unwrap(),
        Instr::FminnmSFloatdp2 {
            Rm: 17,
            Rn: 16,
            Rd: 3
        }
    );
    // fminnm s24, s23, s24
    assert_eq!(
        decode_a64(0x1E387AF8).unwrap(),
        Instr::FminnmSFloatdp2 {
            Rm: 24,
            Rn: 23,
            Rd: 24
        }
    );
}

#[test]
fn test_fnmul() {
    // fnmul s2, s2, s0
    assert_eq!(
        decode_a64(0x1E208842).unwrap(),
        Instr::FnmulSFloatdp2 {
            Rm: 0,
            Rn: 2,
            Rd: 2
        }
    );
    // fnmul s6, s4, s2
    assert_eq!(
        decode_a64(0x1E228886).unwrap(),
        Instr::FnmulSFloatdp2 {
            Rm: 2,
            Rn: 4,
            Rd: 6
        }
    );
    // fnmul s1, s1, s3
    assert_eq!(
        decode_a64(0x1E238821).unwrap(),
        Instr::FnmulSFloatdp2 {
            Rm: 3,
            Rn: 1,
            Rd: 1
        }
    );
    // fnmul s0, s0, s6
    assert_eq!(
        decode_a64(0x1E268800).unwrap(),
        Instr::FnmulSFloatdp2 {
            Rm: 6,
            Rn: 0,
            Rd: 0
        }
    );
    // fnmul s3, s6, s8
    assert_eq!(
        decode_a64(0x1E2888C3).unwrap(),
        Instr::FnmulSFloatdp2 {
            Rm: 8,
            Rn: 6,
            Rd: 3
        }
    );
    // fnmul s20, s23, s21
    assert_eq!(
        decode_a64(0x1E358AF4).unwrap(),
        Instr::FnmulSFloatdp2 {
            Rm: 21,
            Rn: 23,
            Rd: 20
        }
    );
}

#[test]
fn test_fabs() {
    // fabs s2, s0
    assert_eq!(
        decode_a64(0x1E20C002).unwrap(),
        Instr::FabsSFloatdp1 { Rn: 0, Rd: 2 }
    );
    // fabs s0, s1
    assert_eq!(
        decode_a64(0x1E20C020).unwrap(),
        Instr::FabsSFloatdp1 { Rn: 1, Rd: 0 }
    );
    // fabs s3, s3
    assert_eq!(
        decode_a64(0x1E20C063).unwrap(),
        Instr::FabsSFloatdp1 { Rn: 3, Rd: 3 }
    );
    // fabs s0, s6
    assert_eq!(
        decode_a64(0x1E20C0C0).unwrap(),
        Instr::FabsSFloatdp1 { Rn: 6, Rd: 0 }
    );
    // fabs s1, s9
    assert_eq!(
        decode_a64(0x1E20C121).unwrap(),
        Instr::FabsSFloatdp1 { Rn: 9, Rd: 1 }
    );
    // fabs v25.4s, v29.4s
    assert_eq!(
        decode_a64(0x4EA0FBB9).unwrap(),
        Instr::FabsAsimdmiscR {
            Q: 1,
            Rn: 29,
            Rd: 25
        }
    );
}

#[test]
fn test_fneg() {
    // fneg s11, s6
    assert_eq!(
        decode_a64(0x1E2140CB).unwrap(),
        Instr::FnegSFloatdp1 { Rn: 6, Rd: 11 }
    );
    // fneg s0, s11
    assert_eq!(
        decode_a64(0x1E214160).unwrap(),
        Instr::FnegSFloatdp1 { Rn: 11, Rd: 0 }
    );
    // fneg s4, s12
    assert_eq!(
        decode_a64(0x1E214184).unwrap(),
        Instr::FnegSFloatdp1 { Rn: 12, Rd: 4 }
    );
    // fneg s14, s14
    assert_eq!(
        decode_a64(0x1E2141CE).unwrap(),
        Instr::FnegSFloatdp1 { Rn: 14, Rd: 14 }
    );
    // fneg d0, d0
    assert_eq!(
        decode_a64(0x1E614000).unwrap(),
        Instr::FnegDFloatdp1 { Rn: 0, Rd: 0 }
    );
    // fneg v24.4s, v16.4s
    assert_eq!(
        decode_a64(0x6EA0FA18).unwrap(),
        Instr::FnegAsimdmiscR {
            Q: 1,
            Rn: 16,
            Rd: 24
        }
    );
}

#[test]
fn test_fsqrt() {
    // fsqrt s3, s0
    assert_eq!(
        decode_a64(0x1E21C003).unwrap(),
        Instr::FsqrtSFloatdp1 { Rn: 0, Rd: 3 }
    );
    // fsqrt s8, s0
    assert_eq!(
        decode_a64(0x1E21C008).unwrap(),
        Instr::FsqrtSFloatdp1 { Rn: 0, Rd: 8 }
    );
    // fsqrt s12, s0
    assert_eq!(
        decode_a64(0x1E21C00C).unwrap(),
        Instr::FsqrtSFloatdp1 { Rn: 0, Rd: 12 }
    );
    // fsqrt s0, s1
    assert_eq!(
        decode_a64(0x1E21C020).unwrap(),
        Instr::FsqrtSFloatdp1 { Rn: 1, Rd: 0 }
    );
    // fsqrt s10, s11
    assert_eq!(
        decode_a64(0x1E21C16A).unwrap(),
        Instr::FsqrtSFloatdp1 { Rn: 11, Rd: 10 }
    );
    // fsqrt d0, d1
    assert_eq!(
        decode_a64(0x1E61C020).unwrap(),
        Instr::FsqrtDFloatdp1 { Rn: 1, Rd: 0 }
    );
}

#[test]
fn test_fcvt() {
    // fcvt d16, s3
    assert_eq!(
        decode_a64(0x1E22C070).unwrap(),
        Instr::FcvtDsFloatdp1 { Rn: 3, Rd: 16 }
    );
    // fcvt d6, s5
    assert_eq!(
        decode_a64(0x1E22C0A6).unwrap(),
        Instr::FcvtDsFloatdp1 { Rn: 5, Rd: 6 }
    );
    // fcvt s2, d0
    assert_eq!(
        decode_a64(0x1E624002).unwrap(),
        Instr::FcvtSdFloatdp1 { Rn: 0, Rd: 2 }
    );
    // fcvt s14, d0
    assert_eq!(
        decode_a64(0x1E62400E).unwrap(),
        Instr::FcvtSdFloatdp1 { Rn: 0, Rd: 14 }
    );
    // fcvt s5, d5
    assert_eq!(
        decode_a64(0x1E6240A5).unwrap(),
        Instr::FcvtSdFloatdp1 { Rn: 5, Rd: 5 }
    );
    // fcvt s16, d16
    assert_eq!(
        decode_a64(0x1E624210).unwrap(),
        Instr::FcvtSdFloatdp1 { Rn: 16, Rd: 16 }
    );
}

#[test]
fn test_ucvtf() {
    // ucvtf s5, w8
    assert_eq!(
        decode_a64(0x1E230105).unwrap(),
        Instr::UcvtfS32Float2Int { Rn: 8, Rd: 5 }
    );
    // ucvtf s8, w24
    assert_eq!(
        decode_a64(0x1E230308).unwrap(),
        Instr::UcvtfS32Float2Int { Rn: 24, Rd: 8 }
    );
    // ucvtf s8, w27
    assert_eq!(
        decode_a64(0x1E230368).unwrap(),
        Instr::UcvtfS32Float2Int { Rn: 27, Rd: 8 }
    );
    // ucvtf s10, w27
    assert_eq!(
        decode_a64(0x1E23036A).unwrap(),
        Instr::UcvtfS32Float2Int { Rn: 27, Rd: 10 }
    );
    // ucvtf s9, w28
    assert_eq!(
        decode_a64(0x1E230389).unwrap(),
        Instr::UcvtfS32Float2Int { Rn: 28, Rd: 9 }
    );
    // ucvtf s4, s1
    assert_eq!(
        decode_a64(0x7E21D824).unwrap(),
        Instr::UcvtfAsisdmiscR { Rn: 1, Rd: 4 }
    );
}

#[test]
fn test_frintp() {
    // frintp s0, s0
    assert_eq!(
        decode_a64(0x1E24C000).unwrap(),
        Instr::FrintpSFloatdp1 { Rn: 0, Rd: 0 }
    );
    // frintp s3, s0
    assert_eq!(
        decode_a64(0x1E24C003).unwrap(),
        Instr::FrintpSFloatdp1 { Rn: 0, Rd: 3 }
    );
    // frintp s1, s1
    assert_eq!(
        decode_a64(0x1E24C021).unwrap(),
        Instr::FrintpSFloatdp1 { Rn: 1, Rd: 1 }
    );
    // frintp s3, s1
    assert_eq!(
        decode_a64(0x1E24C023).unwrap(),
        Instr::FrintpSFloatdp1 { Rn: 1, Rd: 3 }
    );
    // frintp s2, s2
    assert_eq!(
        decode_a64(0x1E24C042).unwrap(),
        Instr::FrintpSFloatdp1 { Rn: 2, Rd: 2 }
    );
    // frintp d0, d0
    assert_eq!(
        decode_a64(0x1E64C000).unwrap(),
        Instr::FrintpDFloatdp1 { Rn: 0, Rd: 0 }
    );
}

#[test]
fn test_frintm() {
    // frintm s2, s0
    assert_eq!(
        decode_a64(0x1E254002).unwrap(),
        Instr::FrintmSFloatdp1 { Rn: 0, Rd: 2 }
    );
    // frintm s1, s1
    assert_eq!(
        decode_a64(0x1E254021).unwrap(),
        Instr::FrintmSFloatdp1 { Rn: 1, Rd: 1 }
    );
    // frintm s20, s19
    assert_eq!(
        decode_a64(0x1E254274).unwrap(),
        Instr::FrintmSFloatdp1 { Rn: 19, Rd: 20 }
    );
    // frintm d0, d0
    assert_eq!(
        decode_a64(0x1E654000).unwrap(),
        Instr::FrintmDFloatdp1 { Rn: 0, Rd: 0 }
    );
    // frintm d1, d1
    assert_eq!(
        decode_a64(0x1E654021).unwrap(),
        Instr::FrintmDFloatdp1 { Rn: 1, Rd: 1 }
    );
    // frintm d3, d3
    assert_eq!(
        decode_a64(0x1E654063).unwrap(),
        Instr::FrintmDFloatdp1 { Rn: 3, Rd: 3 }
    );
}

#[test]
fn test_frintx() {
    // frintx s0, s0
    assert_eq!(
        decode_a64(0x1E274000).unwrap(),
        Instr::FrintxSFloatdp1 { Rn: 0, Rd: 0 }
    );
    // frintx s2, s1
    assert_eq!(
        decode_a64(0x1E274022).unwrap(),
        Instr::FrintxSFloatdp1 { Rn: 1, Rd: 2 }
    );
    // frintx s8, s8
    assert_eq!(
        decode_a64(0x1E274108).unwrap(),
        Instr::FrintxSFloatdp1 { Rn: 8, Rd: 8 }
    );
    // frintx d0, d0
    assert_eq!(
        decode_a64(0x1E674000).unwrap(),
        Instr::FrintxDFloatdp1 { Rn: 0, Rd: 0 }
    );
    // frintx d2, d1
    assert_eq!(
        decode_a64(0x1E674022).unwrap(),
        Instr::FrintxDFloatdp1 { Rn: 1, Rd: 2 }
    );
    // frintx d8, d8
    assert_eq!(
        decode_a64(0x1E674108).unwrap(),
        Instr::FrintxDFloatdp1 { Rn: 8, Rd: 8 }
    );
}

#[test]
fn test_fcvtps() {
    // fcvtps w1, s0
    assert_eq!(
        decode_a64(0x1E280001).unwrap(),
        Instr::Fcvtps32SFloat2Int { Rn: 0, Rd: 1 }
    );
    // fcvtps w8, s0
    assert_eq!(
        decode_a64(0x1E280008).unwrap(),
        Instr::Fcvtps32SFloat2Int { Rn: 0, Rd: 8 }
    );
    // fcvtps w12, s1
    assert_eq!(
        decode_a64(0x1E28002C).unwrap(),
        Instr::Fcvtps32SFloat2Int { Rn: 1, Rd: 12 }
    );
    // fcvtps w8, s2
    assert_eq!(
        decode_a64(0x1E280048).unwrap(),
        Instr::Fcvtps32SFloat2Int { Rn: 2, Rd: 8 }
    );
    // fcvtps w18, s6
    assert_eq!(
        decode_a64(0x1E2800D2).unwrap(),
        Instr::Fcvtps32SFloat2Int { Rn: 6, Rd: 18 }
    );
    // fcvtps w15, s17
    assert_eq!(
        decode_a64(0x1E28022F).unwrap(),
        Instr::Fcvtps32SFloat2Int { Rn: 17, Rd: 15 }
    );
}

#[test]
fn test_fcvtpu() {
    // fcvtpu w8, s0
    assert_eq!(
        decode_a64(0x1E290008).unwrap(),
        Instr::Fcvtpu32SFloat2Int { Rn: 0, Rd: 8 }
    );
    // fcvtpu x0, s0
    assert_eq!(
        decode_a64(0x9E290000).unwrap(),
        Instr::Fcvtpu64SFloat2Int { Rn: 0, Rd: 0 }
    );
    // fcvtpu x9, s0
    assert_eq!(
        decode_a64(0x9E290009).unwrap(),
        Instr::Fcvtpu64SFloat2Int { Rn: 0, Rd: 9 }
    );
}

#[test]
fn test_fcvtms() {
    // fcvtms w0, s0
    assert_eq!(
        decode_a64(0x1E300000).unwrap(),
        Instr::Fcvtms32SFloat2Int { Rn: 0, Rd: 0 }
    );
    // fcvtms w8, s0
    assert_eq!(
        decode_a64(0x1E300008).unwrap(),
        Instr::Fcvtms32SFloat2Int { Rn: 0, Rd: 8 }
    );
    // fcvtms w19, s0
    assert_eq!(
        decode_a64(0x1E300013).unwrap(),
        Instr::Fcvtms32SFloat2Int { Rn: 0, Rd: 19 }
    );
    // fcvtms w9, s16
    assert_eq!(
        decode_a64(0x1E300209).unwrap(),
        Instr::Fcvtms32SFloat2Int { Rn: 16, Rd: 9 }
    );
    // fcvtms w21, s16
    assert_eq!(
        decode_a64(0x1E300215).unwrap(),
        Instr::Fcvtms32SFloat2Int { Rn: 16, Rd: 21 }
    );
    // fcvtms w22, s19
    assert_eq!(
        decode_a64(0x1E300276).unwrap(),
        Instr::Fcvtms32SFloat2Int { Rn: 19, Rd: 22 }
    );
}

#[test]
fn test_fcvtzs() {
    // fcvtzs w10, s0
    assert_eq!(
        decode_a64(0x1E38000A).unwrap(),
        Instr::Fcvtzs32SFloat2Int { Rn: 0, Rd: 10 }
    );
    // fcvtzs w10, d0, #0x18
    assert_eq!(
        decode_a64(0x1E58A00A).unwrap(),
        Instr::Fcvtzs32DFloat2Fix { Rn: 0, Rd: 10 }
    );
    // fcvtzs w9, d0, #8
    assert_eq!(
        decode_a64(0x1E58E009).unwrap(),
        Instr::Fcvtzs32DFloat2Fix { Rn: 0, Rd: 9 }
    );
    // fcvtzs w8, d2, #8
    assert_eq!(
        decode_a64(0x1E58E048).unwrap(),
        Instr::Fcvtzs32DFloat2Fix { Rn: 2, Rd: 8 }
    );
    // fcvtzs x8, s0
    assert_eq!(
        decode_a64(0x9E380008).unwrap(),
        Instr::Fcvtzs64SFloat2Int { Rn: 0, Rd: 8 }
    );
    // fcvtzs x13, s4
    assert_eq!(
        decode_a64(0x9E38008D).unwrap(),
        Instr::Fcvtzs64SFloat2Int { Rn: 4, Rd: 13 }
    );
}

#[test]
fn test_ldp() {
    // ldp w8, w9, [x1], #8
    assert_eq!(
        decode_a64(0x28C12428).unwrap(),
        Instr::Ldp32LdstpairPost {
            imm7: 2,
            Rt2: 9,
            Rn: 1,
            Rt: 8
        }
    );
    // ldp w11, w12, [x9], #8
    assert_eq!(
        decode_a64(0x28C1312B).unwrap(),
        Instr::Ldp32LdstpairPost {
            imm7: 2,
            Rt2: 12,
            Rn: 9,
            Rt: 11
        }
    );
    // ldp w11, w13, [x11, #4]
    assert_eq!(
        decode_a64(0x2940B56B).unwrap(),
        Instr::Ldp32LdstpairOff {
            imm7: 1,
            Rt2: 13,
            Rn: 11,
            Rt: 11
        }
    );
    // ldp w19, w17, [x17, #8]
    assert_eq!(
        decode_a64(0x29414633).unwrap(),
        Instr::Ldp32LdstpairOff {
            imm7: 2,
            Rt2: 17,
            Rn: 17,
            Rt: 19
        }
    );
    // ldp w9, w12, [sp, #0x44]
    assert_eq!(
        decode_a64(0x2948B3E9).unwrap(),
        Instr::Ldp32LdstpairOff {
            imm7: 17,
            Rt2: 12,
            Rn: 31,
            Rt: 9
        }
    );
    // ldp s16, s17, [x16], #0x10
    assert_eq!(
        decode_a64(0x2CC24610).unwrap(),
        Instr::LdpSLdstpairPost {
            imm7: 4,
            Rt2: 17,
            Rn: 16,
            Rt: 16
        }
    );
    // ldp d13, d12, [sp], #0x40
    assert_eq!(
        decode_a64(0x6CC433ED).unwrap(),
        Instr::LdpDLdstpairPost {
            imm7: 8,
            Rt2: 12,
            Rn: 31,
            Rt: 13
        }
    );
    // ldp x14, x15, [x2], #0x10
    assert_eq!(
        decode_a64(0xA8C13C4E).unwrap(),
        Instr::Ldp64LdstpairPost {
            imm7: 2,
            Rt2: 15,
            Rn: 2,
            Rt: 14
        }
    );
    // ldp x24, x23, [sp], #0x40
    assert_eq!(
        decode_a64(0xA8C45FF8).unwrap(),
        Instr::Ldp64LdstpairPost {
            imm7: 8,
            Rt2: 23,
            Rn: 31,
            Rt: 24
        }
    );
    // ldp x8, x2, [x0]
    assert_eq!(
        decode_a64(0xA9400808).unwrap(),
        Instr::Ldp64LdstpairOff {
            imm7: 0,
            Rt2: 2,
            Rn: 0,
            Rt: 8
        }
    );
    // ldp x14, x12, [sp, #0x60]
    assert_eq!(
        decode_a64(0xA94633EE).unwrap(),
        Instr::Ldp64LdstpairOff {
            imm7: 12,
            Rt2: 12,
            Rn: 31,
            Rt: 14
        }
    );
    // ldp x1, x3, [x29, #-0x60]
    assert_eq!(
        decode_a64(0xA97A0FA1).unwrap(),
        Instr::Ldp64LdstpairOff {
            imm7: 116,
            Rt2: 3,
            Rn: 29,
            Rt: 1
        }
    );
    // ldp x23, x22, [x27, #-0x10]
    assert_eq!(
        decode_a64(0xA97F5B77).unwrap(),
        Instr::Ldp64LdstpairOff {
            imm7: 126,
            Rt2: 22,
            Rn: 27,
            Rt: 23
        }
    );
    // ldp q1, q0, [x29, #-0x30]
    assert_eq!(
        decode_a64(0xAD7E83A1).unwrap(),
        Instr::LdpQLdstpairOff {
            imm7: 125,
            Rt2: 0,
            Rn: 29,
            Rt: 1
        }
    );
}

#[test]
fn test_orn() {
    // orn w8, w26, w0
    assert_eq!(
        decode_a64(0x2A200348).unwrap(),
        Instr::Orn32LogShift {
            shift: 0,
            Rm: 0,
            Rn: 26,
            Rd: 8
        }
    );
    // orn w3, w1, w2
    assert_eq!(
        decode_a64(0x2A220023).unwrap(),
        Instr::Orn32LogShift {
            shift: 0,
            Rm: 2,
            Rn: 1,
            Rd: 3
        }
    );
    // orn w10, w11, w10
    assert_eq!(
        decode_a64(0x2A2A016A).unwrap(),
        Instr::Orn32LogShift {
            shift: 0,
            Rm: 10,
            Rn: 11,
            Rd: 10
        }
    );
    // orn w18, w10, w11
    assert_eq!(
        decode_a64(0x2A2B0152).unwrap(),
        Instr::Orn32LogShift {
            shift: 0,
            Rm: 11,
            Rn: 10,
            Rd: 18
        }
    );
    // orn w8, w17, w13
    assert_eq!(
        decode_a64(0x2A2D0228).unwrap(),
        Instr::Orn32LogShift {
            shift: 0,
            Rm: 13,
            Rn: 17,
            Rd: 8
        }
    );
    // orn w1, w17, w18
    assert_eq!(
        decode_a64(0x2A320221).unwrap(),
        Instr::Orn32LogShift {
            shift: 0,
            Rm: 18,
            Rn: 17,
            Rd: 1
        }
    );
}

#[test]
fn test_mvn() {
    // mvn w17, w1
    assert_eq!(
        decode_a64(0x2A2103F1).unwrap(),
        Instr::Orn32LogShift {
            shift: 0,
            Rm: 1,
            Rn: 31,
            Rd: 17
        }
    );
    // mvn w0, w8
    assert_eq!(
        decode_a64(0x2A2803E0).unwrap(),
        Instr::Orn32LogShift {
            shift: 0,
            Rm: 8,
            Rn: 31,
            Rd: 0
        }
    );
    // mvn w0, w13
    assert_eq!(
        decode_a64(0x2A2D03E0).unwrap(),
        Instr::Orn32LogShift {
            shift: 0,
            Rm: 13,
            Rn: 31,
            Rd: 0
        }
    );
    // mvn w15, w13
    assert_eq!(
        decode_a64(0x2A2D03EF).unwrap(),
        Instr::Orn32LogShift {
            shift: 0,
            Rm: 13,
            Rn: 31,
            Rd: 15
        }
    );
    // mvn v16.16b, v16.16b
    assert_eq!(
        decode_a64(0x6E205A10).unwrap(),
        Instr::NotAsimdmiscR {
            Q: 1,
            Rn: 16,
            Rd: 16
        }
    );
    // mvn x11, x9
    assert_eq!(
        decode_a64(0xAA2903EB).unwrap(),
        Instr::Orn64LogShift {
            shift: 0,
            Rm: 9,
            Rn: 31,
            Rd: 11
        }
    );
}

#[test]
fn test_adds() {
    // adds w8, w8, w0
    assert_eq!(
        decode_a64(0x2B000108).unwrap(),
        Instr::Adds32AddsubShift {
            Rm: 0,
            Rn: 8,
            Rd: 8
        }
    );
    // adds w13, w10, #0x10, lsl #12
    assert_eq!(
        decode_a64(0x3140414D).unwrap(),
        Instr::Adds32SAddsubImm {
            sh: 1,
            imm12: 16,
            Rn: 10,
            Rd: 13
        }
    );
    // adds x14, x14, x3
    assert_eq!(
        decode_a64(0xAB0301CE).unwrap(),
        Instr::Adds64AddsubShift {
            Rm: 3,
            Rn: 14,
            Rd: 14
        }
    );
    // adds x13, x13, x5
    assert_eq!(
        decode_a64(0xAB0501AD).unwrap(),
        Instr::Adds64AddsubShift {
            Rm: 5,
            Rn: 13,
            Rd: 13
        }
    );
    // adds x14, x14, x6
    assert_eq!(
        decode_a64(0xAB0601CE).unwrap(),
        Instr::Adds64AddsubShift {
            Rm: 6,
            Rn: 14,
            Rd: 14
        }
    );
    // adds x11, x8, x23
    assert_eq!(
        decode_a64(0xAB17010B).unwrap(),
        Instr::Adds64AddsubShift {
            Rm: 23,
            Rn: 8,
            Rd: 11
        }
    );
    // adds x4, x4, #1
    assert_eq!(
        decode_a64(0xB1000484).unwrap(),
        Instr::Adds64SAddsubImm {
            sh: 0,
            imm12: 1,
            Rn: 4,
            Rd: 4
        }
    );
}

#[test]
fn test_ext() {
    // ext v1.8b, v2.8b, v3.8b, #4
    assert_eq!(
        decode_a64(0x2E032041).unwrap(),
        Instr::ExtAsimdextOnly {
            Q: 0,
            Rm: 3,
            imm4: 4,
            Rn: 2,
            Rd: 1
        }
    );
    // ext v2.16b, v2.16b, v2.16b, #8
    assert_eq!(
        decode_a64(0x6E024042).unwrap(),
        Instr::ExtAsimdextOnly {
            Q: 1,
            Rm: 2,
            imm4: 8,
            Rn: 2,
            Rd: 2
        }
    );
    // ext v28.16b, v3.16b, v3.16b, #8
    assert_eq!(
        decode_a64(0x6E03407C).unwrap(),
        Instr::ExtAsimdextOnly {
            Q: 1,
            Rm: 3,
            imm4: 8,
            Rn: 3,
            Rd: 28
        }
    );
    // ext v18.16b, v17.16b, v17.16b, #8
    assert_eq!(
        decode_a64(0x6E114232).unwrap(),
        Instr::ExtAsimdextOnly {
            Q: 1,
            Rm: 17,
            imm4: 8,
            Rn: 17,
            Rd: 18
        }
    );
    // ext v26.16b, v21.16b, v21.16b, #8
    assert_eq!(
        decode_a64(0x6E1542BA).unwrap(),
        Instr::ExtAsimdextOnly {
            Q: 1,
            Rm: 21,
            imm4: 8,
            Rn: 21,
            Rd: 26
        }
    );
    // ext v27.16b, v25.16b, v25.16b, #8
    assert_eq!(
        decode_a64(0x6E19433B).unwrap(),
        Instr::ExtAsimdextOnly {
            Q: 1,
            Rm: 25,
            imm4: 8,
            Rn: 25,
            Rd: 27
        }
    );
}

#[test]
fn test_faddp() {
    // faddp v2.2s, v1.2s, v1.2s
    assert_eq!(
        decode_a64(0x2E21D422).unwrap(),
        Instr::FaddpAsimdsameOnly {
            Q: 0,
            Rm: 1,
            Rn: 1,
            Rd: 2
        }
    );
    // faddp v17.2s, v17.2s, v18.2s
    assert_eq!(
        decode_a64(0x2E32D631).unwrap(),
        Instr::FaddpAsimdsameOnly {
            Q: 0,
            Rm: 18,
            Rn: 17,
            Rd: 17
        }
    );
    // faddp v18.2s, v18.2s, v19.2s
    assert_eq!(
        decode_a64(0x2E33D652).unwrap(),
        Instr::FaddpAsimdsameOnly {
            Q: 0,
            Rm: 19,
            Rn: 18,
            Rd: 18
        }
    );
    // faddp v21.2s, v21.2s, v22.2s
    assert_eq!(
        decode_a64(0x2E36D6B5).unwrap(),
        Instr::FaddpAsimdsameOnly {
            Q: 0,
            Rm: 22,
            Rn: 21,
            Rd: 21
        }
    );
    // faddp s3, v5.2s
    assert_eq!(
        decode_a64(0x7E30D8A3).unwrap(),
        Instr::FaddpAsisdpairOnlySd { Rn: 5, Rd: 3 }
    );
    // faddp s23, v20.2s
    assert_eq!(
        decode_a64(0x7E30DA97).unwrap(),
        Instr::FaddpAsisdpairOnlySd { Rn: 20, Rd: 23 }
    );
}

#[test]
fn test_bsl() {
    // bsl v5.8b, v3.8b, v0.8b
    assert_eq!(
        decode_a64(0x2E601C65).unwrap(),
        Instr::BslAsimdsameOnly {
            Q: 0,
            Rm: 0,
            Rn: 3,
            Rd: 5
        }
    );
    // bsl v5.16b, v6.16b, v7.16b
    assert_eq!(
        decode_a64(0x6E671CC5).unwrap(),
        Instr::BslAsimdsameOnly {
            Q: 1,
            Rm: 7,
            Rn: 6,
            Rd: 5
        }
    );
    // bsl v6.16b, v7.16b, v17.16b
    assert_eq!(
        decode_a64(0x6E711CE6).unwrap(),
        Instr::BslAsimdsameOnly {
            Q: 1,
            Rm: 17,
            Rn: 7,
            Rd: 6
        }
    );
    // bsl v25.16b, v30.16b, v27.16b
    assert_eq!(
        decode_a64(0x6E7B1FD9).unwrap(),
        Instr::BslAsimdsameOnly {
            Q: 1,
            Rm: 27,
            Rn: 30,
            Rd: 25
        }
    );
    // bsl v26.16b, v31.16b, v30.16b
    assert_eq!(
        decode_a64(0x6E7E1FFA).unwrap(),
        Instr::BslAsimdsameOnly {
            Q: 1,
            Rm: 30,
            Rn: 31,
            Rd: 26
        }
    );
    // bsl v27.16b, v30.16b, v31.16b
    assert_eq!(
        decode_a64(0x6E7F1FDB).unwrap(),
        Instr::BslAsimdsameOnly {
            Q: 1,
            Rm: 31,
            Rn: 30,
            Rd: 27
        }
    );
}

#[test]
fn test_cmhs() {
    // cmhs v3.2s, v1.2s, v3.2s
    assert_eq!(
        decode_a64(0x2EA33C23).unwrap(),
        Instr::CmhsAsimdsameOnly {
            Q: 0,
            Rm: 3,
            Rn: 1,
            Rd: 3
        }
    );
    // cmhs v4.2s, v1.2s, v4.2s
    assert_eq!(
        decode_a64(0x2EA43C24).unwrap(),
        Instr::CmhsAsimdsameOnly {
            Q: 0,
            Rm: 4,
            Rn: 1,
            Rd: 4
        }
    );
    // cmhs v5.2s, v1.2s, v5.2s
    assert_eq!(
        decode_a64(0x2EA53C25).unwrap(),
        Instr::CmhsAsimdsameOnly {
            Q: 0,
            Rm: 5,
            Rn: 1,
            Rd: 5
        }
    );
}

#[test]
fn test_movi() {
    // movi d29, #0000000000000000
    assert_eq!(
        decode_a64(0x2F00E41D).unwrap(),
        Instr::MoviAsimdimmDDs {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            g: 0,
            h: 0,
            Rd: 29
        }
    );
    // movi v0.4s, #1
    assert_eq!(
        decode_a64(0x4F000420).unwrap(),
        Instr::MoviAsimdimmLSl {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            g: 0,
            h: 1,
            Rd: 0
        }
    );
    // movi v3.4s, #0x80, lsl #24
    assert_eq!(
        decode_a64(0x4F046403).unwrap(),
        Instr::MoviAsimdimmLSl {
            a: 1,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            g: 0,
            h: 0,
            Rd: 3
        }
    );
    // movi v1.4s, #0xbf, lsl #24
    assert_eq!(
        decode_a64(0x4F0567E1).unwrap(),
        Instr::MoviAsimdimmLSl {
            a: 1,
            b: 0,
            c: 1,
            d: 1,
            e: 1,
            f: 1,
            g: 1,
            h: 1,
            Rd: 1
        }
    );
    // movi v7.2d, #0000000000000000
    assert_eq!(
        decode_a64(0x6F00E407).unwrap(),
        Instr::MoviAsimdimmD2D {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            g: 0,
            h: 0,
            Rd: 7
        }
    );
    // movi v1.2d, #0xffffffffffffffff
    assert_eq!(
        decode_a64(0x6F07E7E1).unwrap(),
        Instr::MoviAsimdimmD2D {
            a: 1,
            b: 1,
            c: 1,
            d: 1,
            e: 1,
            f: 1,
            g: 1,
            h: 1,
            Rd: 1
        }
    );
}

#[test]
fn test_ushll() {
    // ushll v2.8h, v1.8b, #0
    assert_eq!(
        decode_a64(0x2F08A422).unwrap(),
        Instr::UshllAsimdshfL {
            Q: 0,
            immh: 1,
            immb: 0,
            Rn: 1,
            Rd: 2
        }
    );
    // ushll v0.4s, v0.4h, #0
    assert_eq!(
        decode_a64(0x2F10A400).unwrap(),
        Instr::UshllAsimdshfL {
            Q: 0,
            immh: 2,
            immb: 0,
            Rn: 0,
            Rd: 0
        }
    );
    // ushll v3.4s, v2.4h, #0
    assert_eq!(
        decode_a64(0x2F10A443).unwrap(),
        Instr::UshllAsimdshfL {
            Q: 0,
            immh: 2,
            immb: 0,
            Rn: 2,
            Rd: 3
        }
    );
    // ushll v6.4s, v6.4h, #0
    assert_eq!(
        decode_a64(0x2F10A4C6).unwrap(),
        Instr::UshllAsimdshfL {
            Q: 0,
            immh: 2,
            immb: 0,
            Rn: 6,
            Rd: 6
        }
    );
    // ushll v13.4s, v13.4h, #0
    assert_eq!(
        decode_a64(0x2F10A5AD).unwrap(),
        Instr::UshllAsimdshfL {
            Q: 0,
            immh: 2,
            immb: 0,
            Rn: 13,
            Rd: 13
        }
    );
    // ushll v21.4s, v21.4h, #0
    assert_eq!(
        decode_a64(0x2F10A6B5).unwrap(),
        Instr::UshllAsimdshfL {
            Q: 0,
            immh: 2,
            immb: 0,
            Rn: 21,
            Rd: 21
        }
    );
}

#[test]
fn test_cmn() {
    // cmn w1, #1
    assert_eq!(
        decode_a64(0x3100043F).unwrap(),
        Instr::Adds32SAddsubImm {
            sh: 0,
            imm12: 1,
            Rn: 1,
            Rd: 31
        }
    );
    // cmn w9, #1
    assert_eq!(
        decode_a64(0x3100053F).unwrap(),
        Instr::Adds32SAddsubImm {
            sh: 0,
            imm12: 1,
            Rn: 9,
            Rd: 31
        }
    );
    // cmn w9, #0xf
    assert_eq!(
        decode_a64(0x31003D3F).unwrap(),
        Instr::Adds32SAddsubImm {
            sh: 0,
            imm12: 15,
            Rn: 9,
            Rd: 31
        }
    );
    // cmn w11, #0x380, lsl #12
    assert_eq!(
        decode_a64(0x314E017F).unwrap(),
        Instr::Adds32SAddsubImm {
            sh: 1,
            imm12: 896,
            Rn: 11,
            Rd: 31
        }
    );
    // cmn x10, #1
    assert_eq!(
        decode_a64(0xB100055F).unwrap(),
        Instr::Adds64SAddsubImm {
            sh: 0,
            imm12: 1,
            Rn: 10,
            Rd: 31
        }
    );
    // cmn x1, #3
    assert_eq!(
        decode_a64(0xB1000C3F).unwrap(),
        Instr::Adds64SAddsubImm {
            sh: 0,
            imm12: 3,
            Rn: 1,
            Rd: 31
        }
    );
    // cmn x9, #0x10
    assert_eq!(
        decode_a64(0xB100413F).unwrap(),
        Instr::Adds64SAddsubImm {
            sh: 0,
            imm12: 16,
            Rn: 9,
            Rd: 31
        }
    );
}

#[test]
fn test_orr() {
    // orr w11, w11, #0x80000
    assert_eq!(
        decode_a64(0x320D016B).unwrap(),
        Instr::Orr32LogImm {
            immr: 13,
            imms: 0,
            Rn: 11,
            Rd: 11
        }
    );
    // orr x14, x4, x2
    assert_eq!(
        decode_a64(0xAA02008E).unwrap(),
        Instr::Orr64LogShift {
            shift: 0,
            Rm: 2,
            Rn: 4,
            Rd: 14
        }
    );
    // orr x21, x21, x7
    assert_eq!(
        decode_a64(0xAA0702B5).unwrap(),
        Instr::Orr64LogShift {
            shift: 0,
            Rm: 7,
            Rn: 21,
            Rd: 21
        }
    );
    // orr x1, x13, x10
    assert_eq!(
        decode_a64(0xAA0A01A1).unwrap(),
        Instr::Orr64LogShift {
            shift: 0,
            Rm: 10,
            Rn: 13,
            Rd: 1
        }
    );
    // orr x9, x9, #1
    assert_eq!(
        decode_a64(0xB2400129).unwrap(),
        Instr::Orr64LogImm {
            immr: 0,
            imms: 0,
            Rn: 9,
            Rd: 9
        }
    );
    // orr x22, xzr, #0xfffffffffffffffe
    assert_eq!(
        decode_a64(0xB27FFBF6).unwrap(),
        Instr::Orr64LogImm {
            immr: 63,
            imms: 62,
            Rn: 31,
            Rd: 22
        }
    );
}

#[test]
fn test_bfxil() {
    // bfxil w0, w17, #0, #1
    assert_eq!(
        decode_a64(0x33000220).unwrap(),
        Instr::Bfm32MBitfield {
            immr: 0,
            imms: 0,
            Rn: 17,
            Rd: 0
        }
    );
    // bfxil w22, w8, #0, #3
    assert_eq!(
        decode_a64(0x33000916).unwrap(),
        Instr::Bfm32MBitfield {
            immr: 0,
            imms: 2,
            Rn: 8,
            Rd: 22
        }
    );
    // bfxil w6, w1, #0, #6
    assert_eq!(
        decode_a64(0x33001426).unwrap(),
        Instr::Bfm32MBitfield {
            immr: 0,
            imms: 5,
            Rn: 1,
            Rd: 6
        }
    );
    // bfxil w12, w9, #2, #0x1e
    assert_eq!(
        decode_a64(0x33027D2C).unwrap(),
        Instr::Bfm32MBitfield {
            immr: 2,
            imms: 31,
            Rn: 9,
            Rd: 12
        }
    );
    // bfxil w8, w1, #6, #5
    assert_eq!(
        decode_a64(0x33062828).unwrap(),
        Instr::Bfm32MBitfield {
            immr: 6,
            imms: 10,
            Rn: 1,
            Rd: 8
        }
    );
    // bfxil x16, x11, #0x15, #0x1f
    assert_eq!(
        decode_a64(0xB355CD70).unwrap(),
        Instr::Bfm64MBitfield {
            immr: 21,
            imms: 51,
            Rn: 11,
            Rd: 16
        }
    );
}

#[test]
fn test_bfi() {
    // bfi w9, w8, #0x18, #4
    assert_eq!(
        decode_a64(0x33080D09).unwrap(),
        Instr::Bfm32MBitfield {
            immr: 8,
            imms: 3,
            Rn: 8,
            Rd: 9
        }
    );
    // bfi w26, w26, #0x10, #0x10
    assert_eq!(
        decode_a64(0x33103F5A).unwrap(),
        Instr::Bfm32MBitfield {
            immr: 16,
            imms: 15,
            Rn: 26,
            Rd: 26
        }
    );
    // bfi w10, w11, #8, #6
    assert_eq!(
        decode_a64(0x3318156A).unwrap(),
        Instr::Bfm32MBitfield {
            immr: 24,
            imms: 5,
            Rn: 11,
            Rd: 10
        }
    );
    // bfi w1, w9, #5, #1
    assert_eq!(
        decode_a64(0x331B0121).unwrap(),
        Instr::Bfm32MBitfield {
            immr: 27,
            imms: 0,
            Rn: 9,
            Rd: 1
        }
    );
    // bfi w9, w11, #3, #1
    assert_eq!(
        decode_a64(0x331D0169).unwrap(),
        Instr::Bfm32MBitfield {
            immr: 29,
            imms: 0,
            Rn: 11,
            Rd: 9
        }
    );
    // bfi w10, w9, #1, #0x1f
    assert_eq!(
        decode_a64(0x331F792A).unwrap(),
        Instr::Bfm32MBitfield {
            immr: 31,
            imms: 30,
            Rn: 9,
            Rd: 10
        }
    );
}

#[test]
fn test_ldrb() {
    // ldrb w8, [x0], #1
    assert_eq!(
        decode_a64(0x38401408).unwrap(),
        Instr::Ldrb32LdstImmpost {
            imm9: 1,
            Rn: 0,
            Rt: 8
        }
    );
    // ldrb w0, [x17], #1
    assert_eq!(
        decode_a64(0x38401620).unwrap(),
        Instr::Ldrb32LdstImmpost {
            imm9: 1,
            Rn: 17,
            Rt: 0
        }
    );
    // ldrb w24, [x18], #1
    assert_eq!(
        decode_a64(0x38401658).unwrap(),
        Instr::Ldrb32LdstImmpost {
            imm9: 1,
            Rn: 18,
            Rt: 24
        }
    );
    // ldrb w4, [x5, #1]!
    assert_eq!(
        decode_a64(0x38401CA4).unwrap(),
        Instr::Ldrb32LdstImmpre {
            imm9: 1,
            Rn: 5,
            Rt: 4
        }
    );
    // ldrb w13, [x11], #2
    assert_eq!(
        decode_a64(0x3840256D).unwrap(),
        Instr::Ldrb32LdstImmpost {
            imm9: 2,
            Rn: 11,
            Rt: 13
        }
    );
    // ldrb w10, [x9, #8]!
    assert_eq!(
        decode_a64(0x38408D2A).unwrap(),
        Instr::Ldrb32LdstImmpre {
            imm9: 8,
            Rn: 9,
            Rt: 10
        }
    );
    // ldrb w12, [x8], #0x70
    assert_eq!(
        decode_a64(0x3847050C).unwrap(),
        Instr::Ldrb32LdstImmpost {
            imm9: 112,
            Rn: 8,
            Rt: 12
        }
    );
    // ldrb w11, [x10, #0x81]!
    assert_eq!(
        decode_a64(0x38481D4B).unwrap(),
        Instr::Ldrb32LdstImmpre {
            imm9: 129,
            Rn: 10,
            Rt: 11
        }
    );
    // ldrb w8, [x0, #0xb0]!
    assert_eq!(
        decode_a64(0x384B0C08).unwrap(),
        Instr::Ldrb32LdstImmpre {
            imm9: 176,
            Rn: 0,
            Rt: 8
        }
    );
    // ldrb w11, [x10, #-0x10]!
    assert_eq!(
        decode_a64(0x385F0D4B).unwrap(),
        Instr::Ldrb32LdstImmpre {
            imm9: 496,
            Rn: 10,
            Rt: 11
        }
    );
    // ldrb w18, [x11], #0xffffffffffffffff
    assert_eq!(
        decode_a64(0x385FF572).unwrap(),
        Instr::Ldrb32LdstImmpost {
            imm9: 511,
            Rn: 11,
            Rt: 18
        }
    );
    // ldrb w14, [x13, #-1]!
    assert_eq!(
        decode_a64(0x385FFDAE).unwrap(),
        Instr::Ldrb32LdstImmpre {
            imm9: 511,
            Rn: 13,
            Rt: 14
        }
    );
    // ldrb w14, [x13, w14, sxtw]
    assert_eq!(
        decode_a64(0x386EC9AE).unwrap(),
        Instr::Ldrb32BLdstRegoff {
            Rm: 14,
            S: 0,
            Rn: 13,
            Rt: 14
        }
    );
    // ldrb w9, [x1]
    assert_eq!(
        decode_a64(0x39400029).unwrap(),
        Instr::Ldrb32LdstPos {
            imm12: 0,
            Rn: 1,
            Rt: 9
        }
    );
    // ldrb w9, [x0, #6]
    assert_eq!(
        decode_a64(0x39401809).unwrap(),
        Instr::Ldrb32LdstPos {
            imm12: 6,
            Rn: 0,
            Rt: 9
        }
    );
    // ldrb w9, [x2, #0x1c]
    assert_eq!(
        decode_a64(0x39407049).unwrap(),
        Instr::Ldrb32LdstPos {
            imm12: 28,
            Rn: 2,
            Rt: 9
        }
    );
    // ldrb w10, [sp, #0xc7]
    assert_eq!(
        decode_a64(0x39431FEA).unwrap(),
        Instr::Ldrb32LdstPos {
            imm12: 199,
            Rn: 31,
            Rt: 10
        }
    );
    // ldrb w8, [x19, #0xbea]
    assert_eq!(
        decode_a64(0x396FAA68).unwrap(),
        Instr::Ldrb32LdstPos {
            imm12: 3050,
            Rn: 19,
            Rt: 8
        }
    );
}

#[test]
fn test_ldurb() {
    // ldurb w8, [x24, #-0x3f]
    assert_eq!(
        decode_a64(0x385C1308).unwrap(),
        Instr::Ldurb32LdstUnscaled {
            imm9: 449,
            Rn: 24,
            Rt: 8
        }
    );
    // ldurb w22, [x5, #-0x1b]
    assert_eq!(
        decode_a64(0x385E50B6).unwrap(),
        Instr::Ldurb32LdstUnscaled {
            imm9: 485,
            Rn: 5,
            Rt: 22
        }
    );
    // ldurb w21, [x28, #-0x10]
    assert_eq!(
        decode_a64(0x385F0395).unwrap(),
        Instr::Ldurb32LdstUnscaled {
            imm9: 496,
            Rn: 28,
            Rt: 21
        }
    );
    // ldurb w0, [x15, #-5]
    assert_eq!(
        decode_a64(0x385FB1E0).unwrap(),
        Instr::Ldurb32LdstUnscaled {
            imm9: 507,
            Rn: 15,
            Rt: 0
        }
    );
    // ldurb w1, [x0, #-1]
    assert_eq!(
        decode_a64(0x385FF001).unwrap(),
        Instr::Ldurb32LdstUnscaled {
            imm9: 511,
            Rn: 0,
            Rt: 1
        }
    );
    // ldurb w6, [x5, #-1]
    assert_eq!(
        decode_a64(0x385FF0A6).unwrap(),
        Instr::Ldurb32LdstUnscaled {
            imm9: 511,
            Rn: 5,
            Rt: 6
        }
    );
}

#[test]
fn test_ldrsb() {
    // ldrsb x12, [x10], #1
    assert_eq!(
        decode_a64(0x3880154C).unwrap(),
        Instr::Ldrsb64LdstImmpost {
            imm9: 1,
            Rn: 10,
            Rt: 12
        }
    );
    // ldrsb x8, [x20], #1
    assert_eq!(
        decode_a64(0x38801688).unwrap(),
        Instr::Ldrsb64LdstImmpost {
            imm9: 1,
            Rn: 20,
            Rt: 8
        }
    );
    // ldrsb x11, [x10, #0x11]!
    assert_eq!(
        decode_a64(0x38811D4B).unwrap(),
        Instr::Ldrsb64LdstImmpre {
            imm9: 17,
            Rn: 10,
            Rt: 11
        }
    );
    // ldrsb w9, [x8], #1
    assert_eq!(
        decode_a64(0x38C01509).unwrap(),
        Instr::Ldrsb32LdstImmpost {
            imm9: 1,
            Rn: 8,
            Rt: 9
        }
    );
    // ldrsb w11, [x9], #1
    assert_eq!(
        decode_a64(0x38C0152B).unwrap(),
        Instr::Ldrsb32LdstImmpost {
            imm9: 1,
            Rn: 9,
            Rt: 11
        }
    );
    // ldrsb w26, [x20], #1
    assert_eq!(
        decode_a64(0x38C0169A).unwrap(),
        Instr::Ldrsb32LdstImmpost {
            imm9: 1,
            Rn: 20,
            Rt: 26
        }
    );
    // ldrsb w9, [x0, #1]!
    assert_eq!(
        decode_a64(0x38C01C09).unwrap(),
        Instr::Ldrsb32LdstImmpre {
            imm9: 1,
            Rn: 0,
            Rt: 9
        }
    );
    // ldrsb w10, [x9, #1]!
    assert_eq!(
        decode_a64(0x38C01D2A).unwrap(),
        Instr::Ldrsb32LdstImmpre {
            imm9: 1,
            Rn: 9,
            Rt: 10
        }
    );
    // ldrsb w8, [x21, #1]!
    assert_eq!(
        decode_a64(0x38C01EA8).unwrap(),
        Instr::Ldrsb32LdstImmpre {
            imm9: 1,
            Rn: 21,
            Rt: 8
        }
    );
    // ldrsb w27, [x11, #2]!
    assert_eq!(
        decode_a64(0x38C02D7B).unwrap(),
        Instr::Ldrsb32LdstImmpre {
            imm9: 2,
            Rn: 11,
            Rt: 27
        }
    );
    // ldrsb w11, [x8], #0xffffffffffffffff
    assert_eq!(
        decode_a64(0x38DFF50B).unwrap(),
        Instr::Ldrsb32LdstImmpost {
            imm9: 511,
            Rn: 8,
            Rt: 11
        }
    );
    // ldrsb w13, [x9, #-1]!
    assert_eq!(
        decode_a64(0x38DFFD2D).unwrap(),
        Instr::Ldrsb32LdstImmpre {
            imm9: 511,
            Rn: 9,
            Rt: 13
        }
    );
    // ldrsb w22, [x21, x9]
    assert_eq!(
        decode_a64(0x38E96AB6).unwrap(),
        Instr::Ldrsb32BlLdstRegoff {
            Rm: 9,
            S: 0,
            Rn: 21,
            Rt: 22
        }
    );
    // ldrsb w15, [x10, w15, sxtw]
    assert_eq!(
        decode_a64(0x38EFC94F).unwrap(),
        Instr::Ldrsb32BLdstRegoff {
            Rm: 15,
            S: 0,
            Rn: 10,
            Rt: 15
        }
    );
    // ldrsb w9, [x25, x28]
    assert_eq!(
        decode_a64(0x38FC6B29).unwrap(),
        Instr::Ldrsb32BlLdstRegoff {
            Rm: 28,
            S: 0,
            Rn: 25,
            Rt: 9
        }
    );
    // ldrsb x8, [x8]
    assert_eq!(
        decode_a64(0x39800108).unwrap(),
        Instr::Ldrsb64LdstPos {
            imm12: 0,
            Rn: 8,
            Rt: 8
        }
    );
    // ldrsb w12, [x0, #3]
    assert_eq!(
        decode_a64(0x39C00C0C).unwrap(),
        Instr::Ldrsb32LdstPos {
            imm12: 3,
            Rn: 0,
            Rt: 12
        }
    );
    // ldrsb w8, [x0, #0x84]
    assert_eq!(
        decode_a64(0x39C21008).unwrap(),
        Instr::Ldrsb32LdstPos {
            imm12: 132,
            Rn: 0,
            Rt: 8
        }
    );
    // ldrsb w8, [x19, #0x19a]
    assert_eq!(
        decode_a64(0x39C66A68).unwrap(),
        Instr::Ldrsb32LdstPos {
            imm12: 410,
            Rn: 19,
            Rt: 8
        }
    );
}

#[test]
fn test_ldursb() {
    // ldursb w18, [x15, #-0x30]
    assert_eq!(
        decode_a64(0x38DD01F2).unwrap(),
        Instr::Ldursb32LdstUnscaled {
            imm9: 464,
            Rn: 15,
            Rt: 18
        }
    );
    // ldursb w1, [x29, #-0x1d]
    assert_eq!(
        decode_a64(0x38DE33A1).unwrap(),
        Instr::Ldursb32LdstUnscaled {
            imm9: 483,
            Rn: 29,
            Rt: 1
        }
    );
    // ldursb w12, [x10, #-2]
    assert_eq!(
        decode_a64(0x38DFE14C).unwrap(),
        Instr::Ldursb32LdstUnscaled {
            imm9: 510,
            Rn: 10,
            Rt: 12
        }
    );
    // ldursb w18, [x17, #-2]
    assert_eq!(
        decode_a64(0x38DFE232).unwrap(),
        Instr::Ldursb32LdstUnscaled {
            imm9: 510,
            Rn: 17,
            Rt: 18
        }
    );
    // ldursb w9, [x1, #-1]
    assert_eq!(
        decode_a64(0x38DFF029).unwrap(),
        Instr::Ldursb32LdstUnscaled {
            imm9: 511,
            Rn: 1,
            Rt: 9
        }
    );
    // ldursb w1, [x8, #-1]
    assert_eq!(
        decode_a64(0x38DFF101).unwrap(),
        Instr::Ldursb32LdstUnscaled {
            imm9: 511,
            Rn: 8,
            Rt: 1
        }
    );
}

#[test]
fn test_adcs() {
    // adcs w4, w4, w8
    assert_eq!(
        decode_a64(0x3A080084).unwrap(),
        Instr::Adcs32AddsubCarry {
            Rm: 8,
            Rn: 4,
            Rd: 4
        }
    );
    // adcs x13, x3, x13
    assert_eq!(
        decode_a64(0xBA0D006D).unwrap(),
        Instr::Adcs64AddsubCarry {
            Rm: 13,
            Rn: 3,
            Rd: 13
        }
    );
    // adcs x6, x6, x14
    assert_eq!(
        decode_a64(0xBA0E00C6).unwrap(),
        Instr::Adcs64AddsubCarry {
            Rm: 14,
            Rn: 6,
            Rd: 6
        }
    );
    // adcs x9, x9, x17
    assert_eq!(
        decode_a64(0xBA110129).unwrap(),
        Instr::Adcs64AddsubCarry {
            Rm: 17,
            Rn: 9,
            Rd: 9
        }
    );
    // adcs x11, x21, xzr
    assert_eq!(
        decode_a64(0xBA1F02AB).unwrap(),
        Instr::Adcs64AddsubCarry {
            Rm: 31,
            Rn: 21,
            Rd: 11
        }
    );
    // adcs x0, xzr, xzr
    assert_eq!(
        decode_a64(0xBA1F03E0).unwrap(),
        Instr::Adcs64AddsubCarry {
            Rm: 31,
            Rn: 31,
            Rd: 0
        }
    );
}

#[test]
fn test_ccmn() {
    // ccmn w8, #1, #4, eq
    assert_eq!(
        decode_a64(0x3A410904).unwrap(),
        Instr::Ccmn32CondcmpImm {
            imm5: 1,
            cond: 0,
            Rn: 8,
            nzcv: 4
        }
    );
    // ccmn w9, #1, #4, ne
    assert_eq!(
        decode_a64(0x3A411924).unwrap(),
        Instr::Ccmn32CondcmpImm {
            imm5: 1,
            cond: 1,
            Rn: 9,
            nzcv: 4
        }
    );
    // ccmn w13, #1, #0, ne
    assert_eq!(
        decode_a64(0x3A4119A0).unwrap(),
        Instr::Ccmn32CondcmpImm {
            imm5: 1,
            cond: 1,
            Rn: 13,
            nzcv: 0
        }
    );
    // ccmn w8, #1, #4, lo
    assert_eq!(
        decode_a64(0x3A413904).unwrap(),
        Instr::Ccmn32CondcmpImm {
            imm5: 1,
            cond: 3,
            Rn: 8,
            nzcv: 4
        }
    );
    // ccmn w22, #0x11, #4, ne
    assert_eq!(
        decode_a64(0x3A511AC4).unwrap(),
        Instr::Ccmn32CondcmpImm {
            imm5: 17,
            cond: 1,
            Rn: 22,
            nzcv: 4
        }
    );
    // ccmn x1, #1, #4, ne
    assert_eq!(
        decode_a64(0xBA411824).unwrap(),
        Instr::Ccmn64CondcmpImm {
            imm5: 1,
            cond: 1,
            Rn: 1,
            nzcv: 4
        }
    );
}

#[test]
fn test_ldur() {
    // ldur q0, [x8, #0x4c]
    assert_eq!(
        decode_a64(0x3CC4C100).unwrap(),
        Instr::LdurQLdstUnscaled {
            imm9: 76,
            Rn: 8,
            Rt: 0
        }
    );
    // ldur w11, [x29, #-0xf0]
    assert_eq!(
        decode_a64(0xB85103AB).unwrap(),
        Instr::Ldur32LdstUnscaled {
            imm9: 272,
            Rn: 29,
            Rt: 11
        }
    );
    // ldur x7, [x4, #6]
    assert_eq!(
        decode_a64(0xF8406087).unwrap(),
        Instr::Ldur64LdstUnscaled {
            imm9: 6,
            Rn: 4,
            Rt: 7
        }
    );
    // ldur x11, [sp, #0x26]
    assert_eq!(
        decode_a64(0xF84263EB).unwrap(),
        Instr::Ldur64LdstUnscaled {
            imm9: 38,
            Rn: 31,
            Rt: 11
        }
    );
    // ldur x25, [x29, #-0xf0]
    assert_eq!(
        decode_a64(0xF85103B9).unwrap(),
        Instr::Ldur64LdstUnscaled {
            imm9: 272,
            Rn: 29,
            Rt: 25
        }
    );
    // ldur x21, [x21, #-0x10]
    assert_eq!(
        decode_a64(0xF85F02B5).unwrap(),
        Instr::Ldur64LdstUnscaled {
            imm9: 496,
            Rn: 21,
            Rt: 21
        }
    );
}

#[test]
fn test_ldxrh() {
    // ldxrh w8, [x0]
    assert_eq!(
        decode_a64(0x485F7C08).unwrap(),
        Instr::LdxrhLr32Ldstexcl {
            Rs: 31,
            Rn: 0,
            Rt: 8
        }
    );
    // ldxrh w9, [x0]
    assert_eq!(
        decode_a64(0x485F7C09).unwrap(),
        Instr::LdxrhLr32Ldstexcl {
            Rs: 31,
            Rn: 0,
            Rt: 9
        }
    );
    // ldxrh w9, [x20]
    assert_eq!(
        decode_a64(0x485F7E89).unwrap(),
        Instr::LdxrhLr32Ldstexcl {
            Rs: 31,
            Rn: 20,
            Rt: 9
        }
    );
    // ldxrh w10, [x20]
    assert_eq!(
        decode_a64(0x485F7E8A).unwrap(),
        Instr::LdxrhLr32Ldstexcl {
            Rs: 31,
            Rn: 20,
            Rt: 10
        }
    );
}

#[test]
fn test_ldaxrh() {
    // ldaxrh w8, [x0]
    assert_eq!(
        decode_a64(0x485FFC08).unwrap(),
        Instr::LdaxrhLr32Ldstexcl {
            Rs: 31,
            Rn: 0,
            Rt: 8
        }
    );
    // ldaxrh w9, [x0]
    assert_eq!(
        decode_a64(0x485FFC09).unwrap(),
        Instr::LdaxrhLr32Ldstexcl {
            Rs: 31,
            Rn: 0,
            Rt: 9
        }
    );
    // ldaxrh w10, [x0]
    assert_eq!(
        decode_a64(0x485FFC0A).unwrap(),
        Instr::LdaxrhLr32Ldstexcl {
            Rs: 31,
            Rn: 0,
            Rt: 10
        }
    );
    // ldaxrh w9, [x20]
    assert_eq!(
        decode_a64(0x485FFE89).unwrap(),
        Instr::LdaxrhLr32Ldstexcl {
            Rs: 31,
            Rn: 20,
            Rt: 9
        }
    );
    // ldaxrh w10, [x20]
    assert_eq!(
        decode_a64(0x485FFE8A).unwrap(),
        Instr::LdaxrhLr32Ldstexcl {
            Rs: 31,
            Rn: 20,
            Rt: 10
        }
    );
}

#[test]
fn test_ldarh() {
    // ldarh w0, [x0]
    assert_eq!(
        decode_a64(0x48DFFC00).unwrap(),
        Instr::LdarhLr32Ldstexcl {
            Rs: 31,
            Rn: 0,
            Rt: 0
        }
    );
    // ldarh w8, [x8]
    assert_eq!(
        decode_a64(0x48DFFD08).unwrap(),
        Instr::LdarhLr32Ldstexcl {
            Rs: 31,
            Rn: 8,
            Rt: 8
        }
    );
    // ldarh w8, [x20]
    assert_eq!(
        decode_a64(0x48DFFE88).unwrap(),
        Instr::LdarhLr32Ldstexcl {
            Rs: 31,
            Rn: 20,
            Rt: 8
        }
    );
}

#[test]
fn test_eor() {
    // eor w8, w1, w0
    assert_eq!(
        decode_a64(0x4A000028).unwrap(),
        Instr::Eor32LogShift {
            shift: 0,
            Rm: 0,
            Rn: 1,
            Rd: 8
        }
    );
    // eor w18, w18, w6
    assert_eq!(
        decode_a64(0x4A060252).unwrap(),
        Instr::Eor32LogShift {
            shift: 0,
            Rm: 6,
            Rn: 18,
            Rd: 18
        }
    );
    // eor w18, w0, w18
    assert_eq!(
        decode_a64(0x4A120012).unwrap(),
        Instr::Eor32LogShift {
            shift: 0,
            Rm: 18,
            Rn: 0,
            Rd: 18
        }
    );
    // eor w27, w27, #0x80000000
    assert_eq!(
        decode_a64(0x5201037B).unwrap(),
        Instr::Eor32LogImm {
            immr: 1,
            imms: 0,
            Rn: 27,
            Rd: 27
        }
    );
    // eor w0, w8, #0x7fff0000
    assert_eq!(
        decode_a64(0x52103900).unwrap(),
        Instr::Eor32LogImm {
            immr: 16,
            imms: 14,
            Rn: 8,
            Rd: 0
        }
    );
    // eor x15, x12, #0x7fff000000000000
    assert_eq!(
        decode_a64(0xD250398F).unwrap(),
        Instr::Eor64LogImm {
            immr: 16,
            imms: 14,
            Rn: 12,
            Rd: 15
        }
    );
}

#[test]
fn test_eon() {
    // eon w10, w10, w9
    assert_eq!(
        decode_a64(0x4A29014A).unwrap(),
        Instr::Eon32LogShift {
            shift: 0,
            Rm: 9,
            Rn: 10,
            Rd: 10
        }
    );
    // eon w12, w12, w11
    assert_eq!(
        decode_a64(0x4A2B018C).unwrap(),
        Instr::Eon32LogShift {
            shift: 0,
            Rm: 11,
            Rn: 12,
            Rd: 12
        }
    );
    // eon w14, w14, w13
    assert_eq!(
        decode_a64(0x4A2D01CE).unwrap(),
        Instr::Eon32LogShift {
            shift: 0,
            Rm: 13,
            Rn: 14,
            Rd: 14
        }
    );
    // eon w15, w15, w14
    assert_eq!(
        decode_a64(0x4A2E01EF).unwrap(),
        Instr::Eon32LogShift {
            shift: 0,
            Rm: 14,
            Rn: 15,
            Rd: 15
        }
    );
    // eon w16, w17, w16
    assert_eq!(
        decode_a64(0x4A300230).unwrap(),
        Instr::Eon32LogShift {
            shift: 0,
            Rm: 16,
            Rn: 17,
            Rd: 16
        }
    );
    // eon w17, w18, w17
    assert_eq!(
        decode_a64(0x4A310251).unwrap(),
        Instr::Eon32LogShift {
            shift: 0,
            Rm: 17,
            Rn: 18,
            Rd: 17
        }
    );
}

#[test]
fn test_neg() {
    // neg w12, w3
    assert_eq!(
        decode_a64(0x4B0303EC).unwrap(),
        Instr::Sub32AddsubShift {
            Rm: 3,
            Rn: 31,
            Rd: 12
        }
    );
    // neg w9, w8, lsl #7
    assert_eq!(
        decode_a64(0x4B081FE9).unwrap(),
        Instr::Sub32AddsubShift {
            Rm: 8,
            Rn: 31,
            Rd: 9
        }
    );
    // neg w4, w19
    assert_eq!(
        decode_a64(0x4B1303E4).unwrap(),
        Instr::Sub32AddsubShift {
            Rm: 19,
            Rn: 31,
            Rd: 4
        }
    );
    // neg x0, x2
    assert_eq!(
        decode_a64(0xCB0203E0).unwrap(),
        Instr::Sub64AddsubShift {
            Rm: 2,
            Rn: 31,
            Rd: 0
        }
    );
    // neg x12, x10, lsl #29
    assert_eq!(
        decode_a64(0xCB0A77EC).unwrap(),
        Instr::Sub64AddsubShift {
            Rm: 10,
            Rn: 31,
            Rd: 12
        }
    );
    // neg x23, x22
    assert_eq!(
        decode_a64(0xCB1603F7).unwrap(),
        Instr::Sub64AddsubShift {
            Rm: 22,
            Rn: 31,
            Rd: 23
        }
    );
}

#[test]
fn test_sub() {
    // sub w7, w7, w21
    assert_eq!(
        decode_a64(0x4B1500E7).unwrap(),
        Instr::Sub32AddsubShift {
            Rm: 21,
            Rn: 7,
            Rd: 7
        }
    );
    // sub w0, w8, w9, uxtb
    assert_eq!(
        decode_a64(0x4B290100).unwrap(),
        Instr::Sub32AddsubExt {
            Rm: 9,
            option: 0,
            Rn: 8,
            Rd: 0
        }
    );
    // sub w10, w22, #0x10, lsl #12
    assert_eq!(
        decode_a64(0x514042CA).unwrap(),
        Instr::Sub32AddsubImm {
            sh: 1,
            imm12: 16,
            Rn: 22,
            Rd: 10
        }
    );
    // sub w10, w10, #0xa00, lsl #12
    assert_eq!(
        decode_a64(0x5168014A).unwrap(),
        Instr::Sub32AddsubImm {
            sh: 1,
            imm12: 2560,
            Rn: 10,
            Rd: 10
        }
    );
    // sub x2, x9, x0
    assert_eq!(
        decode_a64(0xCB000122).unwrap(),
        Instr::Sub64AddsubShift {
            Rm: 0,
            Rn: 9,
            Rd: 2
        }
    );
    // sub x3, x29, #0x20
    assert_eq!(
        decode_a64(0xD10083A3).unwrap(),
        Instr::Sub64AddsubImm {
            sh: 0,
            imm12: 32,
            Rn: 29,
            Rd: 3
        }
    );
    // sub x4, x29, #0x5c
    assert_eq!(
        decode_a64(0xD10173A4).unwrap(),
        Instr::Sub64AddsubImm {
            sh: 0,
            imm12: 92,
            Rn: 29,
            Rd: 4
        }
    );
    // sub sp, sp, #0x70
    assert_eq!(
        decode_a64(0xD101C3FF).unwrap(),
        Instr::Sub64AddsubImm {
            sh: 0,
            imm12: 112,
            Rn: 31,
            Rd: 31
        }
    );
}

#[test]
fn test_fmla() {
    // fmla v3.4s, v1.4s, v0.4s
    assert_eq!(
        decode_a64(0x4E20CC23).unwrap(),
        Instr::FmlaAsimdsameOnly {
            Q: 1,
            Rm: 0,
            Rn: 1,
            Rd: 3
        }
    );
    // fmla v1.4s, v6.4s, v0.s[2]
    assert_eq!(
        decode_a64(0x4F8018C1).unwrap(),
        Instr::FmlaAsimdelemRSd {
            Q: 1,
            L: 0,
            M: 0,
            Rm: 0,
            H: 1,
            Rn: 6,
            Rd: 1
        }
    );
    // fmla v31.4s, v2.4s, v3.s[2]
    assert_eq!(
        decode_a64(0x4F83185F).unwrap(),
        Instr::FmlaAsimdelemRSd {
            Q: 1,
            L: 0,
            M: 0,
            Rm: 3,
            H: 1,
            Rn: 2,
            Rd: 31
        }
    );
    // fmla v16.4s, v17.4s, v3.s[2]
    assert_eq!(
        decode_a64(0x4F831A30).unwrap(),
        Instr::FmlaAsimdelemRSd {
            Q: 1,
            L: 0,
            M: 0,
            Rm: 3,
            H: 1,
            Rn: 17,
            Rd: 16
        }
    );
    // fmla v18.4s, v3.4s, v16.s[2]
    assert_eq!(
        decode_a64(0x4F901872).unwrap(),
        Instr::FmlaAsimdelemRSd {
            Q: 1,
            L: 0,
            M: 1,
            Rm: 0,
            H: 1,
            Rn: 3,
            Rd: 18
        }
    );
    // fmla v22.4s, v20.4s, v16.s[2]
    assert_eq!(
        decode_a64(0x4F901A96).unwrap(),
        Instr::FmlaAsimdelemRSd {
            Q: 1,
            L: 0,
            M: 1,
            Rm: 0,
            H: 1,
            Rn: 20,
            Rd: 22
        }
    );
}

#[test]
fn test_xtn2() {
    // xtn2 v4.16b, v3.8h
    assert_eq!(
        decode_a64(0x4E212864).unwrap(),
        Instr::XtnAsimdmiscN { Q: 1, Rn: 3, Rd: 4 }
    );
    // xtn2 v6.16b, v5.8h
    assert_eq!(
        decode_a64(0x4E2128A6).unwrap(),
        Instr::XtnAsimdmiscN { Q: 1, Rn: 5, Rd: 6 }
    );
    // xtn2 v7.16b, v17.8h
    assert_eq!(
        decode_a64(0x4E212A27).unwrap(),
        Instr::XtnAsimdmiscN {
            Q: 1,
            Rn: 17,
            Rd: 7
        }
    );
    // xtn2 v7.8h, v6.4s
    assert_eq!(
        decode_a64(0x4E6128C7).unwrap(),
        Instr::XtnAsimdmiscN { Q: 1, Rn: 6, Rd: 7 }
    );
    // xtn2 v5.8h, v16.4s
    assert_eq!(
        decode_a64(0x4E612A05).unwrap(),
        Instr::XtnAsimdmiscN {
            Q: 1,
            Rn: 16,
            Rd: 5
        }
    );
    // xtn2 v7.8h, v16.4s
    assert_eq!(
        decode_a64(0x4E612A07).unwrap(),
        Instr::XtnAsimdmiscN {
            Q: 1,
            Rn: 16,
            Rd: 7
        }
    );
}

#[test]
fn test_frecps() {
    // frecps v1.4s, v2.4s, v1.4s
    assert_eq!(
        decode_a64(0x4E21FC41).unwrap(),
        Instr::FrecpsAsimdsameOnly {
            Q: 1,
            Rm: 1,
            Rn: 2,
            Rd: 1
        }
    );
    // frecps v26.4s, v23.4s, v3.4s
    assert_eq!(
        decode_a64(0x4E23FEFA).unwrap(),
        Instr::FrecpsAsimdsameOnly {
            Q: 1,
            Rm: 3,
            Rn: 23,
            Rd: 26
        }
    );
    // frecps v18.4s, v17.4s, v6.4s
    assert_eq!(
        decode_a64(0x4E26FE32).unwrap(),
        Instr::FrecpsAsimdsameOnly {
            Q: 1,
            Rm: 6,
            Rn: 17,
            Rd: 18
        }
    );
    // frecps v25.4s, v21.4s, v18.4s
    assert_eq!(
        decode_a64(0x4E32FEB9).unwrap(),
        Instr::FrecpsAsimdsameOnly {
            Q: 1,
            Rm: 18,
            Rn: 21,
            Rd: 25
        }
    );
    // frecps v12.4s, v11.4s, v30.4s
    assert_eq!(
        decode_a64(0x4E3EFD6C).unwrap(),
        Instr::FrecpsAsimdsameOnly {
            Q: 1,
            Rm: 30,
            Rn: 11,
            Rd: 12
        }
    );
    // frecps v30.4s, v31.4s, v30.4s
    assert_eq!(
        decode_a64(0x4E3EFFFE).unwrap(),
        Instr::FrecpsAsimdsameOnly {
            Q: 1,
            Rm: 30,
            Rn: 31,
            Rd: 30
        }
    );
}

#[test]
fn test_zip2() {
    // zip2 v0.4s, v0.4s, v1.4s
    assert_eq!(
        decode_a64(0x4E817800).unwrap(),
        Instr::Zip2AsimdpermOnly {
            Q: 1,
            size: 2,
            Rm: 1,
            Rn: 0,
            Rd: 0
        }
    );
    // zip2 v1.4s, v1.4s, v2.4s
    assert_eq!(
        decode_a64(0x4E827821).unwrap(),
        Instr::Zip2AsimdpermOnly {
            Q: 1,
            size: 2,
            Rm: 2,
            Rn: 1,
            Rd: 1
        }
    );
    // zip2 v2.4s, v4.4s, v2.4s
    assert_eq!(
        decode_a64(0x4E827882).unwrap(),
        Instr::Zip2AsimdpermOnly {
            Q: 1,
            size: 2,
            Rm: 2,
            Rn: 4,
            Rd: 2
        }
    );
    // zip2 v0.4s, v0.4s, v4.4s
    assert_eq!(
        decode_a64(0x4E847800).unwrap(),
        Instr::Zip2AsimdpermOnly {
            Q: 1,
            size: 2,
            Rm: 4,
            Rn: 0,
            Rd: 0
        }
    );
    // zip2 v3.4s, v3.4s, v4.4s
    assert_eq!(
        decode_a64(0x4E847863).unwrap(),
        Instr::Zip2AsimdpermOnly {
            Q: 1,
            size: 2,
            Rm: 4,
            Rn: 3,
            Rd: 3
        }
    );
    // zip2 v27.4s, v27.4s, v29.4s
    assert_eq!(
        decode_a64(0x4E9D7B7B).unwrap(),
        Instr::Zip2AsimdpermOnly {
            Q: 1,
            size: 2,
            Rm: 29,
            Rn: 27,
            Rd: 27
        }
    );
}

#[test]
fn test_rev64() {
    // rev64 v0.4s, v0.4s
    assert_eq!(
        decode_a64(0x4EA00800).unwrap(),
        Instr::Rev64AsimdmiscR { Q: 1, Rn: 0, Rd: 0 }
    );
    // rev64 v1.4s, v1.4s
    assert_eq!(
        decode_a64(0x4EA00821).unwrap(),
        Instr::Rev64AsimdmiscR { Q: 1, Rn: 1, Rd: 1 }
    );
    // rev64 v2.4s, v2.4s
    assert_eq!(
        decode_a64(0x4EA00842).unwrap(),
        Instr::Rev64AsimdmiscR { Q: 1, Rn: 2, Rd: 2 }
    );
    // rev64 v3.4s, v3.4s
    assert_eq!(
        decode_a64(0x4EA00863).unwrap(),
        Instr::Rev64AsimdmiscR { Q: 1, Rn: 3, Rd: 3 }
    );
    // rev64 v4.4s, v4.4s
    assert_eq!(
        decode_a64(0x4EA00884).unwrap(),
        Instr::Rev64AsimdmiscR { Q: 1, Rn: 4, Rd: 4 }
    );
    // rev64 v7.4s, v7.4s
    assert_eq!(
        decode_a64(0x4EA008E7).unwrap(),
        Instr::Rev64AsimdmiscR { Q: 1, Rn: 7, Rd: 7 }
    );
}

#[test]
fn test_cmtst() {
    // cmtst v18.4s, v17.4s, v0.4s
    assert_eq!(
        decode_a64(0x4EA08E32).unwrap(),
        Instr::CmtstAsimdsameOnly {
            Q: 1,
            Rm: 0,
            Rn: 17,
            Rd: 18
        }
    );
    // cmtst v19.4s, v17.4s, v1.4s
    assert_eq!(
        decode_a64(0x4EA18E33).unwrap(),
        Instr::CmtstAsimdsameOnly {
            Q: 1,
            Rm: 1,
            Rn: 17,
            Rd: 19
        }
    );
    // cmtst v19.4s, v17.4s, v3.4s
    assert_eq!(
        decode_a64(0x4EA38E33).unwrap(),
        Instr::CmtstAsimdsameOnly {
            Q: 1,
            Rm: 3,
            Rn: 17,
            Rd: 19
        }
    );
    // cmtst v19.4s, v17.4s, v5.4s
    assert_eq!(
        decode_a64(0x4EA58E33).unwrap(),
        Instr::CmtstAsimdsameOnly {
            Q: 1,
            Rm: 5,
            Rn: 17,
            Rd: 19
        }
    );
    // cmtst v19.4s, v17.4s, v6.4s
    assert_eq!(
        decode_a64(0x4EA68E33).unwrap(),
        Instr::CmtstAsimdsameOnly {
            Q: 1,
            Rm: 6,
            Rn: 17,
            Rd: 19
        }
    );
    // cmtst v17.4s, v17.4s, v7.4s
    assert_eq!(
        decode_a64(0x4EA78E31).unwrap(),
        Instr::CmtstAsimdsameOnly {
            Q: 1,
            Rm: 7,
            Rn: 17,
            Rd: 17
        }
    );
}

#[test]
fn test_cmeq() {
    // cmeq v1.4s, v0.4s, #0
    assert_eq!(
        decode_a64(0x4EA09801).unwrap(),
        Instr::CmeqAsimdmiscZ { Q: 1, Rn: 0, Rd: 1 }
    );
    // cmeq v16.4s, v7.4s, #0
    assert_eq!(
        decode_a64(0x4EA098F0).unwrap(),
        Instr::CmeqAsimdmiscZ {
            Q: 1,
            Rn: 7,
            Rd: 16
        }
    );
    // cmeq v19.16b, v16.16b, v2.16b
    assert_eq!(
        decode_a64(0x6E228E13).unwrap(),
        Instr::CmeqAsimdsameOnly {
            Q: 1,
            Rm: 2,
            Rn: 16,
            Rd: 19
        }
    );
    // cmeq v4.16b, v4.16b, v3.16b
    assert_eq!(
        decode_a64(0x6E238C84).unwrap(),
        Instr::CmeqAsimdsameOnly {
            Q: 1,
            Rm: 3,
            Rn: 4,
            Rd: 4
        }
    );
    // cmeq v16.16b, v5.16b, v3.16b
    assert_eq!(
        decode_a64(0x6E238CB0).unwrap(),
        Instr::CmeqAsimdsameOnly {
            Q: 1,
            Rm: 3,
            Rn: 5,
            Rd: 16
        }
    );
    // cmeq v18.16b, v7.16b, v5.16b
    assert_eq!(
        decode_a64(0x6E258CF2).unwrap(),
        Instr::CmeqAsimdsameOnly {
            Q: 1,
            Rm: 5,
            Rn: 7,
            Rd: 18
        }
    );
}

#[test]
fn test_fmls() {
    // fmls v6.4s, v2.4s, v0.4s
    assert_eq!(
        decode_a64(0x4EA0CC46).unwrap(),
        Instr::FmlsAsimdsameOnly {
            Q: 1,
            Rm: 0,
            Rn: 2,
            Rd: 6
        }
    );
    // fmls v2.4s, v16.4s, v4.4s
    assert_eq!(
        decode_a64(0x4EA4CE02).unwrap(),
        Instr::FmlsAsimdsameOnly {
            Q: 1,
            Rm: 4,
            Rn: 16,
            Rd: 2
        }
    );
    // fmls v3.4s, v0.4s, v2.s[2]
    assert_eq!(
        decode_a64(0x4F825803).unwrap(),
        Instr::FmlsAsimdelemRSd {
            Q: 1,
            L: 0,
            M: 0,
            Rm: 2,
            H: 1,
            Rn: 0,
            Rd: 3
        }
    );
    // fmls v20.4s, v18.4s, v5.s[0]
    assert_eq!(
        decode_a64(0x4F855254).unwrap(),
        Instr::FmlsAsimdelemRSd {
            Q: 1,
            L: 0,
            M: 0,
            Rm: 5,
            H: 0,
            Rn: 18,
            Rd: 20
        }
    );
    // fmls v30.4s, v24.4s, v8.s[2]
    assert_eq!(
        decode_a64(0x4F885B1E).unwrap(),
        Instr::FmlsAsimdelemRSd {
            Q: 1,
            L: 0,
            M: 0,
            Rm: 8,
            H: 1,
            Rn: 24,
            Rd: 30
        }
    );
    // fmls v27.4s, v26.4s, v28.s[0]
    assert_eq!(
        decode_a64(0x4F9C535B).unwrap(),
        Instr::FmlsAsimdelemRSd {
            Q: 1,
            L: 0,
            M: 1,
            Rm: 12,
            H: 0,
            Rn: 26,
            Rd: 27
        }
    );
}

#[test]
fn test_fcmeq() {
    // fcmeq v2.4s, v1.4s, #0.0
    assert_eq!(
        decode_a64(0x4EA0D822).unwrap(),
        Instr::FcmeqAsimdmiscFz { Q: 1, Rn: 1, Rd: 2 }
    );
    // fcmeq v0.4s, v2.4s, #0.0
    assert_eq!(
        decode_a64(0x4EA0D840).unwrap(),
        Instr::FcmeqAsimdmiscFz { Q: 1, Rn: 2, Rd: 0 }
    );
    // fcmeq v2.4s, v2.4s, #0.0
    assert_eq!(
        decode_a64(0x4EA0D842).unwrap(),
        Instr::FcmeqAsimdmiscFz { Q: 1, Rn: 2, Rd: 2 }
    );
    // fcmeq v1.4s, v4.4s, #0.0
    assert_eq!(
        decode_a64(0x4EA0D881).unwrap(),
        Instr::FcmeqAsimdmiscFz { Q: 1, Rn: 4, Rd: 1 }
    );
    // fcmeq v2.4s, v7.4s, #0.0
    assert_eq!(
        decode_a64(0x4EA0D8E2).unwrap(),
        Instr::FcmeqAsimdmiscFz { Q: 1, Rn: 7, Rd: 2 }
    );
    // fcmeq v23.4s, v25.4s, #0.0
    assert_eq!(
        decode_a64(0x4EA0DB37).unwrap(),
        Instr::FcmeqAsimdmiscFz {
            Q: 1,
            Rn: 25,
            Rd: 23
        }
    );
}

#[test]
fn test_fcmlt() {
    // fcmlt v19.4s, v7.4s, #0.0
    assert_eq!(
        decode_a64(0x4EA0E8F3).unwrap(),
        Instr::FcmltAsimdmiscFz {
            Q: 1,
            Rn: 7,
            Rd: 19
        }
    );
    // fcmlt v27.4s, v29.4s, #0.0
    assert_eq!(
        decode_a64(0x4EA0EBBB).unwrap(),
        Instr::FcmltAsimdmiscFz {
            Q: 1,
            Rn: 29,
            Rd: 27
        }
    );
}

#[test]
fn test_smax() {
    // smax v0.4s, v0.4s, v1.4s
    assert_eq!(
        decode_a64(0x4EA16400).unwrap(),
        Instr::SmaxAsimdsameOnly {
            Q: 1,
            Rm: 1,
            Rn: 0,
            Rd: 0
        }
    );
    // smax v0.4s, v0.4s, v2.4s
    assert_eq!(
        decode_a64(0x4EA26400).unwrap(),
        Instr::SmaxAsimdsameOnly {
            Q: 1,
            Rm: 2,
            Rn: 0,
            Rd: 0
        }
    );
    // smax v0.4s, v0.4s, v3.4s
    assert_eq!(
        decode_a64(0x4EA36400).unwrap(),
        Instr::SmaxAsimdsameOnly {
            Q: 1,
            Rm: 3,
            Rn: 0,
            Rd: 0
        }
    );
    // smax v1.4s, v1.4s, v5.4s
    assert_eq!(
        decode_a64(0x4EA56421).unwrap(),
        Instr::SmaxAsimdsameOnly {
            Q: 1,
            Rm: 5,
            Rn: 1,
            Rd: 1
        }
    );
    // smax v2.4s, v2.4s, v6.4s
    assert_eq!(
        decode_a64(0x4EA66442).unwrap(),
        Instr::SmaxAsimdsameOnly {
            Q: 1,
            Rm: 6,
            Rn: 2,
            Rd: 2
        }
    );
    // smax v3.4s, v3.4s, v7.4s
    assert_eq!(
        decode_a64(0x4EA76463).unwrap(),
        Instr::SmaxAsimdsameOnly {
            Q: 1,
            Rm: 7,
            Rn: 3,
            Rd: 3
        }
    );
}

#[test]
fn test_frecpe() {
    // frecpe v3.4s, v2.4s
    assert_eq!(
        decode_a64(0x4EA1D843).unwrap(),
        Instr::FrecpeAsimdmiscR { Q: 1, Rn: 2, Rd: 3 }
    );
    // frecpe v18.4s, v6.4s
    assert_eq!(
        decode_a64(0x4EA1D8D2).unwrap(),
        Instr::FrecpeAsimdmiscR {
            Q: 1,
            Rn: 6,
            Rd: 18
        }
    );
    // frecpe v18.4s, v17.4s
    assert_eq!(
        decode_a64(0x4EA1DA32).unwrap(),
        Instr::FrecpeAsimdmiscR {
            Q: 1,
            Rn: 17,
            Rd: 18
        }
    );
    // frecpe v19.4s, v18.4s
    assert_eq!(
        decode_a64(0x4EA1DA53).unwrap(),
        Instr::FrecpeAsimdmiscR {
            Q: 1,
            Rn: 18,
            Rd: 19
        }
    );
    // frecpe v31.4s, v25.4s
    assert_eq!(
        decode_a64(0x4EA1DB3F).unwrap(),
        Instr::FrecpeAsimdmiscR {
            Q: 1,
            Rn: 25,
            Rd: 31
        }
    );
    // frecpe v5.4s, v28.4s
    assert_eq!(
        decode_a64(0x4EA1DB85).unwrap(),
        Instr::FrecpeAsimdmiscR {
            Q: 1,
            Rn: 28,
            Rd: 5
        }
    );
}

#[test]
fn test_frsqrts() {
    // frsqrts v2.4s, v0.4s, v2.4s
    assert_eq!(
        decode_a64(0x4EA2FC02).unwrap(),
        Instr::FrsqrtsAsimdsameOnly {
            Q: 1,
            Rm: 2,
            Rn: 0,
            Rd: 2
        }
    );
    // frsqrts v3.4s, v4.4s, v3.4s
    assert_eq!(
        decode_a64(0x4EA3FC83).unwrap(),
        Instr::FrsqrtsAsimdsameOnly {
            Q: 1,
            Rm: 3,
            Rn: 4,
            Rd: 3
        }
    );
    // frsqrts v7.4s, v4.4s, v7.4s
    assert_eq!(
        decode_a64(0x4EA7FC87).unwrap(),
        Instr::FrsqrtsAsimdsameOnly {
            Q: 1,
            Rm: 7,
            Rn: 4,
            Rd: 7
        }
    );
    // frsqrts v7.4s, v6.4s, v7.4s
    assert_eq!(
        decode_a64(0x4EA7FCC7).unwrap(),
        Instr::FrsqrtsAsimdsameOnly {
            Q: 1,
            Rm: 7,
            Rn: 6,
            Rd: 7
        }
    );
    // frsqrts v16.4s, v7.4s, v16.4s
    assert_eq!(
        decode_a64(0x4EB0FCF0).unwrap(),
        Instr::FrsqrtsAsimdsameOnly {
            Q: 1,
            Rm: 16,
            Rn: 7,
            Rd: 16
        }
    );
    // frsqrts v31.4s, v30.4s, v31.4s
    assert_eq!(
        decode_a64(0x4EBFFFDF).unwrap(),
        Instr::FrsqrtsAsimdsameOnly {
            Q: 1,
            Rm: 31,
            Rn: 30,
            Rd: 31
        }
    );
}

#[test]
fn test_pmull2() {
    // pmull2 v0.1q, v0.2d, v1.2d
    assert_eq!(
        decode_a64(0x4EE1E000).unwrap(),
        Instr::PmullAsimddiffL {
            Q: 1,
            size: 3,
            Rm: 1,
            Rn: 0,
            Rd: 0
        }
    );
    // pmull2 v19.1q, v19.2d, v1.2d
    assert_eq!(
        decode_a64(0x4EE1E273).unwrap(),
        Instr::PmullAsimddiffL {
            Q: 1,
            size: 3,
            Rm: 1,
            Rn: 19,
            Rd: 19
        }
    );
    // pmull2 v17.1q, v17.2d, v2.2d
    assert_eq!(
        decode_a64(0x4EE2E231).unwrap(),
        Instr::PmullAsimddiffL {
            Q: 1,
            size: 3,
            Rm: 2,
            Rn: 17,
            Rd: 17
        }
    );
    // pmull2 v25.1q, v17.2d, v2.2d
    assert_eq!(
        decode_a64(0x4EE2E239).unwrap(),
        Instr::PmullAsimddiffL {
            Q: 1,
            size: 3,
            Rm: 2,
            Rn: 17,
            Rd: 25
        }
    );
    // pmull2 v16.1q, v16.2d, v3.2d
    assert_eq!(
        decode_a64(0x4EE3E210).unwrap(),
        Instr::PmullAsimddiffL {
            Q: 1,
            size: 3,
            Rm: 3,
            Rn: 16,
            Rd: 16
        }
    );
    // pmull2 v26.1q, v18.2d, v8.2d
    assert_eq!(
        decode_a64(0x4EE8E25A).unwrap(),
        Instr::PmullAsimddiffL {
            Q: 1,
            size: 3,
            Rm: 8,
            Rn: 18,
            Rd: 26
        }
    );
}

#[test]
fn test_sshr() {
    // sshr v0.16b, v1.16b, #7
    assert_eq!(
        decode_a64(0x4F090420).unwrap(),
        Instr::SshrAsimdshfR {
            Q: 1,
            immh: 1,
            immb: 1,
            Rn: 1,
            Rd: 0
        }
    );
    // sshr v0.4s, v0.4s, #0x1f
    assert_eq!(
        decode_a64(0x4F210400).unwrap(),
        Instr::SshrAsimdshfR {
            Q: 1,
            immh: 4,
            immb: 1,
            Rn: 0,
            Rd: 0
        }
    );
    // sshr v17.4s, v6.4s, #0x1f
    assert_eq!(
        decode_a64(0x4F2104D1).unwrap(),
        Instr::SshrAsimdshfR {
            Q: 1,
            immh: 4,
            immb: 1,
            Rn: 6,
            Rd: 17
        }
    );
    // sshr v7.4s, v7.4s, #0x1f
    assert_eq!(
        decode_a64(0x4F2104E7).unwrap(),
        Instr::SshrAsimdshfR {
            Q: 1,
            immh: 4,
            immb: 1,
            Rn: 7,
            Rd: 7
        }
    );
    // sshr v16.4s, v16.4s, #0x1f
    assert_eq!(
        decode_a64(0x4F210610).unwrap(),
        Instr::SshrAsimdshfR {
            Q: 1,
            immh: 4,
            immb: 1,
            Rn: 16,
            Rd: 16
        }
    );
    // sshr v21.4s, v21.4s, #0x1f
    assert_eq!(
        decode_a64(0x4F2106B5).unwrap(),
        Instr::SshrAsimdshfR {
            Q: 1,
            immh: 4,
            immb: 1,
            Rn: 21,
            Rd: 21
        }
    );
}

#[test]
fn test_shl() {
    // shl v16.16b, v16.16b, #7
    assert_eq!(
        decode_a64(0x4F0F5610).unwrap(),
        Instr::ShlAsimdshfR {
            Q: 1,
            immh: 1,
            immb: 7,
            Rn: 16,
            Rd: 16
        }
    );
    // shl v7.4s, v2.4s, #3
    assert_eq!(
        decode_a64(0x4F235447).unwrap(),
        Instr::ShlAsimdshfR {
            Q: 1,
            immh: 4,
            immb: 3,
            Rn: 2,
            Rd: 7
        }
    );
    // shl v6.4s, v3.4s, #6
    assert_eq!(
        decode_a64(0x4F265466).unwrap(),
        Instr::ShlAsimdshfR {
            Q: 1,
            immh: 4,
            immb: 6,
            Rn: 3,
            Rd: 6
        }
    );
    // shl v0.4s, v0.4s, #0x1f
    assert_eq!(
        decode_a64(0x4F3F5400).unwrap(),
        Instr::ShlAsimdshfR {
            Q: 1,
            immh: 7,
            immb: 7,
            Rn: 0,
            Rd: 0
        }
    );
    // shl v6.4s, v6.4s, #0x1f
    assert_eq!(
        decode_a64(0x4F3F54C6).unwrap(),
        Instr::ShlAsimdshfR {
            Q: 1,
            immh: 7,
            immb: 7,
            Rn: 6,
            Rd: 6
        }
    );
    // shl v21.4s, v21.4s, #0x1f
    assert_eq!(
        decode_a64(0x4F3F56B5).unwrap(),
        Instr::ShlAsimdshfR {
            Q: 1,
            immh: 7,
            immb: 7,
            Rn: 21,
            Rd: 21
        }
    );
}

#[test]
fn test_ubfx() {
    // ubfx w9, w0, #1, #1
    assert_eq!(
        decode_a64(0x53010409).unwrap(),
        Instr::Ubfm32MBitfield {
            immr: 1,
            imms: 1,
            Rn: 0,
            Rd: 9
        }
    );
    // ubfx w8, w20, #1, #1
    assert_eq!(
        decode_a64(0x53010688).unwrap(),
        Instr::Ubfm32MBitfield {
            immr: 1,
            imms: 1,
            Rn: 20,
            Rd: 8
        }
    );
    // ubfx w11, w9, #3, #1
    assert_eq!(
        decode_a64(0x53030D2B).unwrap(),
        Instr::Ubfm32MBitfield {
            immr: 3,
            imms: 3,
            Rn: 9,
            Rd: 11
        }
    );
    // ubfx w13, w13, #0xc, #1
    assert_eq!(
        decode_a64(0x530C31AD).unwrap(),
        Instr::Ubfm32MBitfield {
            immr: 12,
            imms: 12,
            Rn: 13,
            Rd: 13
        }
    );
    // ubfx w0, w8, #0xf, #5
    assert_eq!(
        decode_a64(0x530F4D00).unwrap(),
        Instr::Ubfm32MBitfield {
            immr: 15,
            imms: 19,
            Rn: 8,
            Rd: 0
        }
    );
    // ubfx x15, x15, #0x1f, #0x20
    assert_eq!(
        decode_a64(0xD35FF9EF).unwrap(),
        Instr::Ubfm64MBitfield {
            immr: 31,
            imms: 62,
            Rn: 15,
            Rd: 15
        }
    );
}

#[test]
fn test_ubfiz() {
    // ubfiz w21, w0, #8, #6
    assert_eq!(
        decode_a64(0x53181415).unwrap(),
        Instr::Ubfm32MBitfield {
            immr: 24,
            imms: 5,
            Rn: 0,
            Rd: 21
        }
    );
    // ubfiz w10, w10, #8, #8
    assert_eq!(
        decode_a64(0x53181D4A).unwrap(),
        Instr::Ubfm32MBitfield {
            immr: 24,
            imms: 7,
            Rn: 10,
            Rd: 10
        }
    );
    // ubfiz w12, w12, #5, #2
    assert_eq!(
        decode_a64(0x531B058C).unwrap(),
        Instr::Ubfm32MBitfield {
            immr: 27,
            imms: 1,
            Rn: 12,
            Rd: 12
        }
    );
    // ubfiz w10, w8, #4, #2
    assert_eq!(
        decode_a64(0x531C050A).unwrap(),
        Instr::Ubfm32MBitfield {
            immr: 28,
            imms: 1,
            Rn: 8,
            Rd: 10
        }
    );
    // ubfiz w8, w8, #4, #4
    assert_eq!(
        decode_a64(0x531C0D08).unwrap(),
        Instr::Ubfm32MBitfield {
            immr: 28,
            imms: 3,
            Rn: 8,
            Rd: 8
        }
    );
    // ubfiz x12, x13, #0x28, #0x14
    assert_eq!(
        decode_a64(0xD3584DAC).unwrap(),
        Instr::Ubfm64MBitfield {
            immr: 24,
            imms: 19,
            Rn: 13,
            Rd: 12
        }
    );
}

#[test]
fn test_cneg() {
    // cneg w9, w0, mi
    assert_eq!(
        decode_a64(0x5A805409).unwrap(),
        Instr::Csneg32Condsel {
            Rm: 0,
            cond: 5,
            Rn: 0,
            Rd: 9
        }
    );
    // cneg w14, w4, lt
    assert_eq!(
        decode_a64(0x5A84A48E).unwrap(),
        Instr::Csneg32Condsel {
            Rm: 4,
            cond: 10,
            Rn: 4,
            Rd: 14
        }
    );
    // cneg w14, w11, eq
    assert_eq!(
        decode_a64(0x5A8B156E).unwrap(),
        Instr::Csneg32Condsel {
            Rm: 11,
            cond: 1,
            Rn: 11,
            Rd: 14
        }
    );
    // cneg w17, w14, mi
    assert_eq!(
        decode_a64(0x5A8E55D1).unwrap(),
        Instr::Csneg32Condsel {
            Rm: 14,
            cond: 5,
            Rn: 14,
            Rd: 17
        }
    );
    // cneg x8, x8, mi
    assert_eq!(
        decode_a64(0xDA885508).unwrap(),
        Instr::Csneg64Condsel {
            Rm: 8,
            cond: 5,
            Rn: 8,
            Rd: 8
        }
    );
    // cneg x17, x17, mi
    assert_eq!(
        decode_a64(0xDA915631).unwrap(),
        Instr::Csneg64Condsel {
            Rm: 17,
            cond: 5,
            Rn: 17,
            Rd: 17
        }
    );
}

#[test]
fn test_csneg() {
    // csneg w8, w8, w2, ge
    assert_eq!(
        decode_a64(0x5A82A508).unwrap(),
        Instr::Csneg32Condsel {
            Rm: 2,
            cond: 10,
            Rn: 8,
            Rd: 8
        }
    );
    // csneg w8, w9, w8, gt
    assert_eq!(
        decode_a64(0x5A88C528).unwrap(),
        Instr::Csneg32Condsel {
            Rm: 8,
            cond: 12,
            Rn: 9,
            Rd: 8
        }
    );
    // csneg w8, w10, w8, gt
    assert_eq!(
        decode_a64(0x5A88C548).unwrap(),
        Instr::Csneg32Condsel {
            Rm: 8,
            cond: 12,
            Rn: 10,
            Rd: 8
        }
    );
    // csneg w21, w21, w8, le
    assert_eq!(
        decode_a64(0x5A88D6B5).unwrap(),
        Instr::Csneg32Condsel {
            Rm: 8,
            cond: 13,
            Rn: 21,
            Rd: 21
        }
    );
    // csneg w12, w10, w22, gt
    assert_eq!(
        decode_a64(0x5A96C54C).unwrap(),
        Instr::Csneg32Condsel {
            Rm: 22,
            cond: 12,
            Rn: 10,
            Rd: 12
        }
    );
    // csneg w11, w23, w27, lo
    assert_eq!(
        decode_a64(0x5A9B36EB).unwrap(),
        Instr::Csneg32Condsel {
            Rm: 27,
            cond: 3,
            Rn: 23,
            Rd: 11
        }
    );
}

#[test]
fn test_csinv() {
    // csinv w8, w8, w3, ge
    assert_eq!(
        decode_a64(0x5A83A108).unwrap(),
        Instr::Csinv32Condsel {
            Rm: 3,
            cond: 10,
            Rn: 8,
            Rd: 8
        }
    );
    // csinv w8, w9, w8, ls
    assert_eq!(
        decode_a64(0x5A889128).unwrap(),
        Instr::Csinv32Condsel {
            Rm: 8,
            cond: 9,
            Rn: 9,
            Rd: 8
        }
    );
    // csinv w12, w13, w24, le
    assert_eq!(
        decode_a64(0x5A98D1AC).unwrap(),
        Instr::Csinv32Condsel {
            Rm: 24,
            cond: 13,
            Rn: 13,
            Rd: 12
        }
    );
    // csinv w8, w9, wzr, ge
    assert_eq!(
        decode_a64(0x5A9FA128).unwrap(),
        Instr::Csinv32Condsel {
            Rm: 31,
            cond: 10,
            Rn: 9,
            Rd: 8
        }
    );
    // csinv w25, w9, wzr, le
    assert_eq!(
        decode_a64(0x5A9FD139).unwrap(),
        Instr::Csinv32Condsel {
            Rm: 31,
            cond: 13,
            Rn: 9,
            Rd: 25
        }
    );
    // csinv w18, w16, wzr, le
    assert_eq!(
        decode_a64(0x5A9FD212).unwrap(),
        Instr::Csinv32Condsel {
            Rm: 31,
            cond: 13,
            Rn: 16,
            Rd: 18
        }
    );
}

#[test]
fn test_cinv() {
    // cinv w0, w8, ne
    assert_eq!(
        decode_a64(0x5A880100).unwrap(),
        Instr::Csinv32Condsel {
            Rm: 8,
            cond: 0,
            Rn: 8,
            Rd: 0
        }
    );
    // cinv w8, w8, ge
    assert_eq!(
        decode_a64(0x5A88B108).unwrap(),
        Instr::Csinv32Condsel {
            Rm: 8,
            cond: 11,
            Rn: 8,
            Rd: 8
        }
    );
    // cinv w22, w8, le
    assert_eq!(
        decode_a64(0x5A88C116).unwrap(),
        Instr::Csinv32Condsel {
            Rm: 8,
            cond: 12,
            Rn: 8,
            Rd: 22
        }
    );
    // cinv w10, w10, lt
    assert_eq!(
        decode_a64(0x5A8AA14A).unwrap(),
        Instr::Csinv32Condsel {
            Rm: 10,
            cond: 10,
            Rn: 10,
            Rd: 10
        }
    );
    // cinv w11, w11, lt
    assert_eq!(
        decode_a64(0x5A8BA16B).unwrap(),
        Instr::Csinv32Condsel {
            Rm: 11,
            cond: 10,
            Rn: 11,
            Rd: 11
        }
    );
    // cinv x0, x8, ge
    assert_eq!(
        decode_a64(0xDA88B100).unwrap(),
        Instr::Csinv64Condsel {
            Rm: 8,
            cond: 11,
            Rn: 8,
            Rd: 0
        }
    );
}

#[test]
fn test_csetm() {
    // csetm w23, ne
    assert_eq!(
        decode_a64(0x5A9F03F7).unwrap(),
        Instr::Csinv32Condsel {
            Rm: 31,
            cond: 0,
            Rn: 31,
            Rd: 23
        }
    );
    // csetm w0, hi
    assert_eq!(
        decode_a64(0x5A9F93E0).unwrap(),
        Instr::Csinv32Condsel {
            Rm: 31,
            cond: 9,
            Rn: 31,
            Rd: 0
        }
    );
    // csetm w14, hi
    assert_eq!(
        decode_a64(0x5A9F93EE).unwrap(),
        Instr::Csinv32Condsel {
            Rm: 31,
            cond: 9,
            Rn: 31,
            Rd: 14
        }
    );
    // csetm w11, lt
    assert_eq!(
        decode_a64(0x5A9FA3EB).unwrap(),
        Instr::Csinv32Condsel {
            Rm: 31,
            cond: 10,
            Rn: 31,
            Rd: 11
        }
    );
    // csetm w23, le
    assert_eq!(
        decode_a64(0x5A9FC3F7).unwrap(),
        Instr::Csinv32Condsel {
            Rm: 31,
            cond: 12,
            Rn: 31,
            Rd: 23
        }
    );
    // csetm x0, eq
    assert_eq!(
        decode_a64(0xDA9F13E0).unwrap(),
        Instr::Csinv64Condsel {
            Rm: 31,
            cond: 1,
            Rn: 31,
            Rd: 0
        }
    );
}

#[test]
fn test_rbit() {
    // rbit w10, w10
    assert_eq!(
        decode_a64(0x5AC0014A).unwrap(),
        Instr::Rbit32Dp1Src { Rd: 10 }
    );
    // rbit w11, w15
    assert_eq!(
        decode_a64(0x5AC001EB).unwrap(),
        Instr::Rbit32Dp1Src { Rd: 11 }
    );
    // rbit x8, x0
    assert_eq!(
        decode_a64(0xDAC00008).unwrap(),
        Instr::Rbit64Dp1Src { Rd: 8 }
    );
    // rbit x9, x9
    assert_eq!(
        decode_a64(0xDAC00129).unwrap(),
        Instr::Rbit64Dp1Src { Rd: 9 }
    );
    // rbit x11, x11
    assert_eq!(
        decode_a64(0xDAC0016B).unwrap(),
        Instr::Rbit64Dp1Src { Rd: 11 }
    );
    // rbit x11, x12
    assert_eq!(
        decode_a64(0xDAC0018B).unwrap(),
        Instr::Rbit64Dp1Src { Rd: 11 }
    );
}

#[test]
fn test_rev16() {
    // rev16 w5, w3
    assert_eq!(
        decode_a64(0x5AC00465).unwrap(),
        Instr::Rev1632Dp1Src { Rd: 5 }
    );
    // rev16 w10, w9
    assert_eq!(
        decode_a64(0x5AC0052A).unwrap(),
        Instr::Rev1632Dp1Src { Rd: 10 }
    );
    // rev16 w3, w11
    assert_eq!(
        decode_a64(0x5AC00563).unwrap(),
        Instr::Rev1632Dp1Src { Rd: 3 }
    );
    // rev16 w12, w12
    assert_eq!(
        decode_a64(0x5AC0058C).unwrap(),
        Instr::Rev1632Dp1Src { Rd: 12 }
    );
    // rev16 w26, w24
    assert_eq!(
        decode_a64(0x5AC0071A).unwrap(),
        Instr::Rev1632Dp1Src { Rd: 26 }
    );
    // rev16 w11, w28
    assert_eq!(
        decode_a64(0x5AC0078B).unwrap(),
        Instr::Rev1632Dp1Src { Rd: 11 }
    );
}

#[test]
fn test_rev() {
    // rev w4, w5
    assert_eq!(
        decode_a64(0x5AC008A4).unwrap(),
        Instr::Rev32Dp1Src { Rd: 4 }
    );
    // rev w28, w30
    assert_eq!(
        decode_a64(0x5AC00BDC).unwrap(),
        Instr::Rev32Dp1Src { Rd: 28 }
    );
    // rev w30, w30
    assert_eq!(
        decode_a64(0x5AC00BDE).unwrap(),
        Instr::Rev32Dp1Src { Rd: 30 }
    );
    // rev x8, x8
    assert_eq!(
        decode_a64(0xDAC00D08).unwrap(),
        Instr::Rev64Dp1Src { Rd: 8 }
    );
    // rev x11, x11
    assert_eq!(
        decode_a64(0xDAC00D6B).unwrap(),
        Instr::Rev64Dp1Src { Rd: 11 }
    );
    // rev x12, x12
    assert_eq!(
        decode_a64(0xDAC00D8C).unwrap(),
        Instr::Rev64Dp1Src { Rd: 12 }
    );
}

#[test]
fn test_clz() {
    // clz w9, w6
    assert_eq!(
        decode_a64(0x5AC010C9).unwrap(),
        Instr::Clz32Dp1Src { Rd: 9 }
    );
    // clz w21, w6
    assert_eq!(
        decode_a64(0x5AC010D5).unwrap(),
        Instr::Clz32Dp1Src { Rd: 21 }
    );
    // clz w12, w12
    assert_eq!(
        decode_a64(0x5AC0118C).unwrap(),
        Instr::Clz32Dp1Src { Rd: 12 }
    );
    // clz w13, w12
    assert_eq!(
        decode_a64(0x5AC0118D).unwrap(),
        Instr::Clz32Dp1Src { Rd: 13 }
    );
    // clz w19, w19
    assert_eq!(
        decode_a64(0x5AC01273).unwrap(),
        Instr::Clz32Dp1Src { Rd: 19 }
    );
    // clz w10, w25
    assert_eq!(
        decode_a64(0x5AC0132A).unwrap(),
        Instr::Clz32Dp1Src { Rd: 10 }
    );
}

#[test]
fn test_sha1su0() {
    // sha1su0 v5.4s, v6.4s, v7.4s
    assert_eq!(
        decode_a64(0x5E0730C5).unwrap(),
        Instr::Sha1Su0VvvCryptosha3 {
            Rm: 7,
            Rn: 6,
            Rd: 5
        }
    );
    // sha1su0 v6.4s, v7.4s, v16.4s
    assert_eq!(
        decode_a64(0x5E1030E6).unwrap(),
        Instr::Sha1Su0VvvCryptosha3 {
            Rm: 16,
            Rn: 7,
            Rd: 6
        }
    );
}

#[test]
fn test_ldpsw() {
    // ldpsw x6, x5, [x5]
    assert_eq!(
        decode_a64(0x694014A6).unwrap(),
        Instr::Ldpsw64LdstpairOff {
            imm7: 0,
            Rt2: 5,
            Rn: 5,
            Rt: 6
        }
    );
    // ldpsw x18, x14, [x14]
    assert_eq!(
        decode_a64(0x694039D2).unwrap(),
        Instr::Ldpsw64LdstpairOff {
            imm7: 0,
            Rt2: 14,
            Rn: 14,
            Rt: 18
        }
    );
    // ldpsw x10, x8, [x19, #8]
    assert_eq!(
        decode_a64(0x6941226A).unwrap(),
        Instr::Ldpsw64LdstpairOff {
            imm7: 2,
            Rt2: 8,
            Rn: 19,
            Rt: 10
        }
    );
    // ldpsw x1, x14, [x12, #0x30]
    assert_eq!(
        decode_a64(0x69463981).unwrap(),
        Instr::Ldpsw64LdstpairOff {
            imm7: 12,
            Rt2: 14,
            Rn: 12,
            Rt: 1
        }
    );
    // ldpsw x10, x11, [x19, #0xac]
    assert_eq!(
        decode_a64(0x6955AE6A).unwrap(),
        Instr::Ldpsw64LdstpairOff {
            imm7: 43,
            Rt2: 11,
            Rn: 19,
            Rt: 10
        }
    );
    // ldpsw x14, x15, [x11, #-0x10]
    assert_eq!(
        decode_a64(0x697E3D6E).unwrap(),
        Instr::Ldpsw64LdstpairOff {
            imm7: 124,
            Rt2: 15,
            Rn: 11,
            Rt: 14
        }
    );
}

#[test]
fn test_tst() {
    // tst w5, w7
    assert_eq!(
        decode_a64(0x6A0700BF).unwrap(),
        Instr::Ands32LogShift {
            shift: 0,
            Rm: 7,
            Rn: 5,
            Rd: 31
        }
    );
    // tst w13, w12
    assert_eq!(
        decode_a64(0x6A0C01BF).unwrap(),
        Instr::Ands32LogShift {
            shift: 0,
            Rm: 12,
            Rn: 13,
            Rd: 31
        }
    );
    // tst w9, #0x1000000
    assert_eq!(
        decode_a64(0x7208013F).unwrap(),
        Instr::Ands32SLogImm {
            immr: 8,
            imms: 0,
            Rn: 9,
            Rd: 31
        }
    );
    // tst w17, #0x400
    assert_eq!(
        decode_a64(0x7216023F).unwrap(),
        Instr::Ands32SLogImm {
            immr: 22,
            imms: 0,
            Rn: 17,
            Rd: 31
        }
    );
    // tst w25, #4
    assert_eq!(
        decode_a64(0x721E033F).unwrap(),
        Instr::Ands32SLogImm {
            immr: 30,
            imms: 0,
            Rn: 25,
            Rd: 31
        }
    );
    // tst x10, #0x7fffffffffffffff
    assert_eq!(
        decode_a64(0xF240F95F).unwrap(),
        Instr::Ands64SLogImm {
            immr: 0,
            imms: 62,
            Rn: 10,
            Rd: 31
        }
    );
}

#[test]
fn test_cmp() {
    // cmp w2, w8
    assert_eq!(
        decode_a64(0x6B08005F).unwrap(),
        Instr::Subs32AddsubShift {
            Rm: 8,
            Rn: 2,
            Rd: 31
        }
    );
    // cmp w26, w13
    assert_eq!(
        decode_a64(0x6B0D035F).unwrap(),
        Instr::Subs32AddsubShift {
            Rm: 13,
            Rn: 26,
            Rd: 31
        }
    );
    // cmp w18, w4, uxth
    assert_eq!(
        decode_a64(0x6B24225F).unwrap(),
        Instr::Subs32SAddsubExt {
            Rm: 4,
            option: 1,
            Rn: 18,
            Rd: 31
        }
    );
    // cmp w9, w8, asr #3
    assert_eq!(
        decode_a64(0x6B880D3F).unwrap(),
        Instr::Subs32AddsubShift {
            Rm: 8,
            Rn: 9,
            Rd: 31
        }
    );
    // cmp w1, #0
    assert_eq!(
        decode_a64(0x7100003F).unwrap(),
        Instr::Subs32SAddsubImm {
            sh: 0,
            imm12: 0,
            Rn: 1,
            Rd: 31
        }
    );
    // cmp w2, #0x93
    assert_eq!(
        decode_a64(0x71024C5F).unwrap(),
        Instr::Subs32SAddsubImm {
            sh: 0,
            imm12: 147,
            Rn: 2,
            Rd: 31
        }
    );
    // cmp w12, #0x7fe, lsl #12
    assert_eq!(
        decode_a64(0x715FF99F).unwrap(),
        Instr::Subs32SAddsubImm {
            sh: 1,
            imm12: 2046,
            Rn: 12,
            Rd: 31
        }
    );
    // cmp x8, x20
    assert_eq!(
        decode_a64(0xEB14011F).unwrap(),
        Instr::Subs64AddsubShift {
            Rm: 20,
            Rn: 8,
            Rd: 31
        }
    );
    // cmp x6, #0xa
    assert_eq!(
        decode_a64(0xF10028DF).unwrap(),
        Instr::Subs64SAddsubImm {
            sh: 0,
            imm12: 10,
            Rn: 6,
            Rd: 31
        }
    );
    // cmp x19, #1, lsl #12
    assert_eq!(
        decode_a64(0xF140067F).unwrap(),
        Instr::Subs64SAddsubImm {
            sh: 1,
            imm12: 1,
            Rn: 19,
            Rd: 31
        }
    );
    // cmp x8, #0x80, lsl #12
    assert_eq!(
        decode_a64(0xF142011F).unwrap(),
        Instr::Subs64SAddsubImm {
            sh: 1,
            imm12: 128,
            Rn: 8,
            Rd: 31
        }
    );
}

#[test]
fn test_subs() {
    // subs w9, w26, w8
    assert_eq!(
        decode_a64(0x6B080349).unwrap(),
        Instr::Subs32AddsubShift {
            Rm: 8,
            Rn: 26,
            Rd: 9
        }
    );
    // subs w8, w8, w10
    assert_eq!(
        decode_a64(0x6B0A0108).unwrap(),
        Instr::Subs32AddsubShift {
            Rm: 10,
            Rn: 8,
            Rd: 8
        }
    );
    // subs w17, w17, w12
    assert_eq!(
        decode_a64(0x6B0C0231).unwrap(),
        Instr::Subs32AddsubShift {
            Rm: 12,
            Rn: 17,
            Rd: 17
        }
    );
    // subs x25, x20, x21
    assert_eq!(
        decode_a64(0xEB150299).unwrap(),
        Instr::Subs64AddsubShift {
            Rm: 21,
            Rn: 20,
            Rd: 25
        }
    );
    // subs x8, x8, #1
    assert_eq!(
        decode_a64(0xF1000508).unwrap(),
        Instr::Subs64SAddsubImm {
            sh: 0,
            imm12: 1,
            Rn: 8,
            Rd: 8
        }
    );
    // subs x21, x21, #0x600, lsl #12
    assert_eq!(
        decode_a64(0xF15802B5).unwrap(),
        Instr::Subs64SAddsubImm {
            sh: 1,
            imm12: 1536,
            Rn: 21,
            Rd: 21
        }
    );
}

#[test]
fn test_rev32() {
    // rev32 v0.16b, v0.16b
    assert_eq!(
        decode_a64(0x6E200800).unwrap(),
        Instr::Rev32AsimdmiscR { Q: 1, Rn: 0, Rd: 0 }
    );
    // rev32 v1.16b, v1.16b
    assert_eq!(
        decode_a64(0x6E200821).unwrap(),
        Instr::Rev32AsimdmiscR { Q: 1, Rn: 1, Rd: 1 }
    );
    // rev32 v3.16b, v3.16b
    assert_eq!(
        decode_a64(0x6E200863).unwrap(),
        Instr::Rev32AsimdmiscR { Q: 1, Rn: 3, Rd: 3 }
    );
    // rev32 v5.16b, v5.16b
    assert_eq!(
        decode_a64(0x6E2008A5).unwrap(),
        Instr::Rev32AsimdmiscR { Q: 1, Rn: 5, Rd: 5 }
    );
    // rev32 v7.16b, v7.16b
    assert_eq!(
        decode_a64(0x6E2008E7).unwrap(),
        Instr::Rev32AsimdmiscR { Q: 1, Rn: 7, Rd: 7 }
    );
    // rev32 v16.16b, v16.16b
    assert_eq!(
        decode_a64(0x6E200A10).unwrap(),
        Instr::Rev32AsimdmiscR {
            Q: 1,
            Rn: 16,
            Rd: 16
        }
    );
}

#[test]
fn test_fcmge() {
    // fcmge v2.4s, v3.4s, v2.4s
    assert_eq!(
        decode_a64(0x6E22E462).unwrap(),
        Instr::FcmgeAsimdsameOnly {
            Q: 1,
            Rm: 2,
            Rn: 3,
            Rd: 2
        }
    );
    // fcmge v27.4s, v25.4s, v8.4s
    assert_eq!(
        decode_a64(0x6E28E73B).unwrap(),
        Instr::FcmgeAsimdsameOnly {
            Q: 1,
            Rm: 8,
            Rn: 25,
            Rd: 27
        }
    );
    // fcmge v24.4s, v24.4s, v28.4s
    assert_eq!(
        decode_a64(0x6E3CE718).unwrap(),
        Instr::FcmgeAsimdsameOnly {
            Q: 1,
            Rm: 28,
            Rn: 24,
            Rd: 24
        }
    );
    // fcmge v4.4s, v3.4s, #0.0
    assert_eq!(
        decode_a64(0x6EA0C864).unwrap(),
        Instr::FcmgeAsimdmiscFz { Q: 1, Rn: 3, Rd: 4 }
    );
    // fcmge v6.4s, v4.4s, #0.0
    assert_eq!(
        decode_a64(0x6EA0C886).unwrap(),
        Instr::FcmgeAsimdmiscFz { Q: 1, Rn: 4, Rd: 6 }
    );
    // fcmge v29.4s, v27.4s, #0.0
    assert_eq!(
        decode_a64(0x6EA0CB7D).unwrap(),
        Instr::FcmgeAsimdmiscFz {
            Q: 1,
            Rn: 27,
            Rd: 29
        }
    );
}

#[test]
fn test_cmhi() {
    // cmhi v2.8h, v2.8h, v0.8h
    assert_eq!(
        decode_a64(0x6E603442).unwrap(),
        Instr::CmhiAsimdsameOnly {
            Q: 1,
            Rm: 0,
            Rn: 2,
            Rd: 2
        }
    );
    // cmhi v3.8h, v3.8h, v0.8h
    assert_eq!(
        decode_a64(0x6E603463).unwrap(),
        Instr::CmhiAsimdsameOnly {
            Q: 1,
            Rm: 0,
            Rn: 3,
            Rd: 3
        }
    );
}

#[test]
fn test_bit() {
    // bit v2.16b, v1.16b, v0.16b
    assert_eq!(
        decode_a64(0x6EA01C22).unwrap(),
        Instr::BitAsimdsameOnly {
            Q: 1,
            Rm: 0,
            Rn: 1,
            Rd: 2
        }
    );
    // bit v2.16b, v1.16b, v3.16b
    assert_eq!(
        decode_a64(0x6EA31C22).unwrap(),
        Instr::BitAsimdsameOnly {
            Q: 1,
            Rm: 3,
            Rn: 1,
            Rd: 2
        }
    );
    // bit v16.16b, v1.16b, v4.16b
    assert_eq!(
        decode_a64(0x6EA41C30).unwrap(),
        Instr::BitAsimdsameOnly {
            Q: 1,
            Rm: 4,
            Rn: 1,
            Rd: 16
        }
    );
    // bit v5.16b, v3.16b, v4.16b
    assert_eq!(
        decode_a64(0x6EA41C65).unwrap(),
        Instr::BitAsimdsameOnly {
            Q: 1,
            Rm: 4,
            Rn: 3,
            Rd: 5
        }
    );
    // bit v7.16b, v0.16b, v5.16b
    assert_eq!(
        decode_a64(0x6EA51C07).unwrap(),
        Instr::BitAsimdsameOnly {
            Q: 1,
            Rm: 5,
            Rn: 0,
            Rd: 7
        }
    );
    // bit v20.16b, v3.16b, v22.16b
    assert_eq!(
        decode_a64(0x6EB61C74).unwrap(),
        Instr::BitAsimdsameOnly {
            Q: 1,
            Rm: 22,
            Rn: 3,
            Rd: 20
        }
    );
}

#[test]
fn test_frsqrte() {
    // frsqrte v0.4s, v1.4s
    assert_eq!(
        decode_a64(0x6EA1D820).unwrap(),
        Instr::FrsqrteAsimdmiscR { Q: 1, Rn: 1, Rd: 0 }
    );
    // frsqrte v2.4s, v1.4s
    assert_eq!(
        decode_a64(0x6EA1D822).unwrap(),
        Instr::FrsqrteAsimdmiscR { Q: 1, Rn: 1, Rd: 2 }
    );
    // frsqrte v6.4s, v2.4s
    assert_eq!(
        decode_a64(0x6EA1D846).unwrap(),
        Instr::FrsqrteAsimdmiscR { Q: 1, Rn: 2, Rd: 6 }
    );
    // frsqrte v1.4s, v5.4s
    assert_eq!(
        decode_a64(0x6EA1D8A1).unwrap(),
        Instr::FrsqrteAsimdmiscR { Q: 1, Rn: 5, Rd: 1 }
    );
    // frsqrte v4.4s, v5.4s
    assert_eq!(
        decode_a64(0x6EA1D8A4).unwrap(),
        Instr::FrsqrteAsimdmiscR { Q: 1, Rn: 5, Rd: 4 }
    );
    // frsqrte v17.4s, v16.4s
    assert_eq!(
        decode_a64(0x6EA1DA11).unwrap(),
        Instr::FrsqrteAsimdmiscR {
            Q: 1,
            Rn: 16,
            Rd: 17
        }
    );
}

#[test]
fn test_fcmgt() {
    // fcmgt v1.4s, v3.4s, v1.4s
    assert_eq!(
        decode_a64(0x6EA1E461).unwrap(),
        Instr::FcmgtAsimdsameOnly {
            Q: 1,
            Rm: 1,
            Rn: 3,
            Rd: 1
        }
    );
    // fcmgt v2.4s, v3.4s, v2.4s
    assert_eq!(
        decode_a64(0x6EA2E462).unwrap(),
        Instr::FcmgtAsimdsameOnly {
            Q: 1,
            Rm: 2,
            Rn: 3,
            Rd: 2
        }
    );
    // fcmgt v26.4s, v8.4s, v5.4s
    assert_eq!(
        decode_a64(0x6EA5E51A).unwrap(),
        Instr::FcmgtAsimdsameOnly {
            Q: 1,
            Rm: 5,
            Rn: 8,
            Rd: 26
        }
    );
    // fcmgt v3.4s, v23.4s, v8.4s
    assert_eq!(
        decode_a64(0x6EA8E6E3).unwrap(),
        Instr::FcmgtAsimdsameOnly {
            Q: 1,
            Rm: 8,
            Rn: 23,
            Rd: 3
        }
    );
    // fcmgt v7.4s, v3.4s, v24.4s
    assert_eq!(
        decode_a64(0x6EB8E467).unwrap(),
        Instr::FcmgtAsimdsameOnly {
            Q: 1,
            Rm: 24,
            Rn: 3,
            Rd: 7
        }
    );
    // fcmgt v30.4s, v17.4s, v28.4s
    assert_eq!(
        decode_a64(0x6EBCE63E).unwrap(),
        Instr::FcmgtAsimdsameOnly {
            Q: 1,
            Rm: 28,
            Rn: 17,
            Rd: 30
        }
    );
}

#[test]
fn test_ushll2() {
    // ushll2 v1.8h, v1.16b, #0
    assert_eq!(
        decode_a64(0x6F08A421).unwrap(),
        Instr::UshllAsimdshfL {
            Q: 1,
            immh: 1,
            immb: 0,
            Rn: 1,
            Rd: 1
        }
    );
    // ushll2 v1.4s, v1.8h, #0
    assert_eq!(
        decode_a64(0x6F10A421).unwrap(),
        Instr::UshllAsimdshfL {
            Q: 1,
            immh: 2,
            immb: 0,
            Rn: 1,
            Rd: 1
        }
    );
    // ushll2 v2.4s, v2.8h, #0
    assert_eq!(
        decode_a64(0x6F10A442).unwrap(),
        Instr::UshllAsimdshfL {
            Q: 1,
            immh: 2,
            immb: 0,
            Rn: 2,
            Rd: 2
        }
    );
}

#[test]
fn test_ands() {
    // ands w10, w10, #1
    assert_eq!(
        decode_a64(0x7200014A).unwrap(),
        Instr::Ands32SLogImm {
            immr: 0,
            imms: 0,
            Rn: 10,
            Rd: 10
        }
    );
    // ands w13, w10, #0x4000
    assert_eq!(
        decode_a64(0x7212014D).unwrap(),
        Instr::Ands32SLogImm {
            immr: 18,
            imms: 0,
            Rn: 10,
            Rd: 13
        }
    );
    // ands w28, w8, #4
    assert_eq!(
        decode_a64(0x721E011C).unwrap(),
        Instr::Ands32SLogImm {
            immr: 30,
            imms: 0,
            Rn: 8,
            Rd: 28
        }
    );
    // ands w11, w11, #0xfffffffe
    assert_eq!(
        decode_a64(0x721F796B).unwrap(),
        Instr::Ands32SLogImm {
            immr: 31,
            imms: 30,
            Rn: 11,
            Rd: 11
        }
    );
    // ands x13, x13, #7
    assert_eq!(
        decode_a64(0xF24009AD).unwrap(),
        Instr::Ands64SLogImm {
            immr: 0,
            imms: 2,
            Rn: 13,
            Rd: 13
        }
    );
}

#[test]
fn test_movk() {
    // movk w25, #0x364
    assert_eq!(
        decode_a64(0x72806C99).unwrap(),
        Instr::Movk32Movewide { imm16: 868, Rd: 25 }
    );
    // movk w19, #0x1006
    assert_eq!(
        decode_a64(0x728200D3).unwrap(),
        Instr::Movk32Movewide {
            imm16: 4102,
            Rd: 19
        }
    );
    // movk w8, #0x1201
    assert_eq!(
        decode_a64(0x72824028).unwrap(),
        Instr::Movk32Movewide { imm16: 4609, Rd: 8 }
    );
    // movk w14, #0xd7d7
    assert_eq!(
        decode_a64(0x729AFAEE).unwrap(),
        Instr::Movk32Movewide {
            imm16: 55255,
            Rd: 14
        }
    );
    // movk x6, #0
    assert_eq!(
        decode_a64(0xF2800006).unwrap(),
        Instr::Movk64Movewide { imm16: 0, Rd: 6 }
    );
    // movk x12, #0x3e00
    assert_eq!(
        decode_a64(0xF287C00C).unwrap(),
        Instr::Movk64Movewide {
            imm16: 15872,
            Rd: 12
        }
    );
    // movk x27, #0xffff, lsl #32
    assert_eq!(
        decode_a64(0xF2DFFFFB).unwrap(),
        Instr::Movk64Movewide {
            imm16: 65535,
            Rd: 27
        }
    );
}

#[test]
fn test_ldurh() {
    // ldurh w0, [x8, #1]
    assert_eq!(
        decode_a64(0x78401100).unwrap(),
        Instr::Ldurh32LdstUnscaled {
            imm9: 1,
            Rn: 8,
            Rt: 0
        }
    );
    // ldurh w18, [x15, #-0x10]
    assert_eq!(
        decode_a64(0x785F01F2).unwrap(),
        Instr::Ldurh32LdstUnscaled {
            imm9: 496,
            Rn: 15,
            Rt: 18
        }
    );
    // ldurh w8, [x22, #-7]
    assert_eq!(
        decode_a64(0x785F92C8).unwrap(),
        Instr::Ldurh32LdstUnscaled {
            imm9: 505,
            Rn: 22,
            Rt: 8
        }
    );
    // ldurh w20, [x8, #-2]
    assert_eq!(
        decode_a64(0x785FE114).unwrap(),
        Instr::Ldurh32LdstUnscaled {
            imm9: 510,
            Rn: 8,
            Rt: 20
        }
    );
    // ldurh w11, [x9, #-2]
    assert_eq!(
        decode_a64(0x785FE12B).unwrap(),
        Instr::Ldurh32LdstUnscaled {
            imm9: 510,
            Rn: 9,
            Rt: 11
        }
    );
    // ldurh w16, [x11, #-2]
    assert_eq!(
        decode_a64(0x785FE170).unwrap(),
        Instr::Ldurh32LdstUnscaled {
            imm9: 510,
            Rn: 11,
            Rt: 16
        }
    );
}

#[test]
fn test_ldrh() {
    // ldrh w10, [x2], #2
    assert_eq!(
        decode_a64(0x7840244A).unwrap(),
        Instr::Ldrh32LdstImmpost {
            imm9: 2,
            Rn: 2,
            Rt: 10
        }
    );
    // ldrh w5, [x6], #2
    assert_eq!(
        decode_a64(0x784024C5).unwrap(),
        Instr::Ldrh32LdstImmpost {
            imm9: 2,
            Rn: 6,
            Rt: 5
        }
    );
    // ldrh w0, [x8], #2
    assert_eq!(
        decode_a64(0x78402500).unwrap(),
        Instr::Ldrh32LdstImmpost {
            imm9: 2,
            Rn: 8,
            Rt: 0
        }
    );
    // ldrh w24, [x19], #2
    assert_eq!(
        decode_a64(0x78402678).unwrap(),
        Instr::Ldrh32LdstImmpost {
            imm9: 2,
            Rn: 19,
            Rt: 24
        }
    );
    // ldrh w8, [x20, #2]!
    assert_eq!(
        decode_a64(0x78402E88).unwrap(),
        Instr::Ldrh32LdstImmpre {
            imm9: 2,
            Rn: 20,
            Rt: 8
        }
    );
    // ldrh w8, [x24, #2]!
    assert_eq!(
        decode_a64(0x78402F08).unwrap(),
        Instr::Ldrh32LdstImmpre {
            imm9: 2,
            Rn: 24,
            Rt: 8
        }
    );
    // ldrh w9, [x8, #6]!
    assert_eq!(
        decode_a64(0x78406D09).unwrap(),
        Instr::Ldrh32LdstImmpre {
            imm9: 6,
            Rn: 8,
            Rt: 9
        }
    );
    // ldrh w3, [x18, #0x34]!
    assert_eq!(
        decode_a64(0x78434E43).unwrap(),
        Instr::Ldrh32LdstImmpre {
            imm9: 52,
            Rn: 18,
            Rt: 3
        }
    );
    // ldrh w8, [x24, #0x6c]!
    assert_eq!(
        decode_a64(0x7846CF08).unwrap(),
        Instr::Ldrh32LdstImmpre {
            imm9: 108,
            Rn: 24,
            Rt: 8
        }
    );
    // ldrh w13, [x10, #-0x20]!
    assert_eq!(
        decode_a64(0x785E0D4D).unwrap(),
        Instr::Ldrh32LdstImmpre {
            imm9: 480,
            Rn: 10,
            Rt: 13
        }
    );
    // ldrh w16, [x15], #0xfffffffffffffff0
    assert_eq!(
        decode_a64(0x785F05F0).unwrap(),
        Instr::Ldrh32LdstImmpost {
            imm9: 496,
            Rn: 15,
            Rt: 16
        }
    );
    // ldrh w13, [x11], #0xfffffffffffffffe
    assert_eq!(
        decode_a64(0x785FE56D).unwrap(),
        Instr::Ldrh32LdstImmpost {
            imm9: 510,
            Rn: 11,
            Rt: 13
        }
    );
    // ldrh w4, [x14, x2]
    assert_eq!(
        decode_a64(0x786269C4).unwrap(),
        Instr::Ldrh32LdstRegoff {
            Rm: 2,
            S: 0,
            Rn: 14,
            Rt: 4
        }
    );
    // ldrh w10, [x11, w10, uxtw #1]
    assert_eq!(
        decode_a64(0x786A596A).unwrap(),
        Instr::Ldrh32LdstRegoff {
            Rm: 10,
            S: 1,
            Rn: 11,
            Rt: 10
        }
    );
    // ldrh w7, [x1]
    assert_eq!(
        decode_a64(0x79400027).unwrap(),
        Instr::Ldrh32LdstPos {
            imm12: 0,
            Rn: 1,
            Rt: 7
        }
    );
    // ldrh w9, [x5, #2]
    assert_eq!(
        decode_a64(0x794004A9).unwrap(),
        Instr::Ldrh32LdstPos {
            imm12: 1,
            Rn: 5,
            Rt: 9
        }
    );
    // ldrh w11, [sp, #0x18]
    assert_eq!(
        decode_a64(0x794033EB).unwrap(),
        Instr::Ldrh32LdstPos {
            imm12: 12,
            Rn: 31,
            Rt: 11
        }
    );
    // ldrh w8, [x8, #0x128]
    assert_eq!(
        decode_a64(0x79425108).unwrap(),
        Instr::Ldrh32LdstPos {
            imm12: 148,
            Rn: 8,
            Rt: 8
        }
    );
}

#[test]
fn test_ldursh() {
    // ldursh x8, [x9, #1]
    assert_eq!(
        decode_a64(0x78801128).unwrap(),
        Instr::Ldursh64LdstUnscaled {
            imm9: 1,
            Rn: 9,
            Rt: 8
        }
    );
    // ldursh x24, [x14, #-2]
    assert_eq!(
        decode_a64(0x789FE1D8).unwrap(),
        Instr::Ldursh64LdstUnscaled {
            imm9: 510,
            Rn: 14,
            Rt: 24
        }
    );
    // ldursh w6, [x15, #-0x1e]
    assert_eq!(
        decode_a64(0x78DE21E6).unwrap(),
        Instr::Ldursh32LdstUnscaled {
            imm9: 482,
            Rn: 15,
            Rt: 6
        }
    );
    // ldursh w14, [x23, #-0x10]
    assert_eq!(
        decode_a64(0x78DF02EE).unwrap(),
        Instr::Ldursh32LdstUnscaled {
            imm9: 496,
            Rn: 23,
            Rt: 14
        }
    );
    // ldursh w5, [x4, #-2]
    assert_eq!(
        decode_a64(0x78DFE085).unwrap(),
        Instr::Ldursh32LdstUnscaled {
            imm9: 510,
            Rn: 4,
            Rt: 5
        }
    );
    // ldursh w12, [x11, #-2]
    assert_eq!(
        decode_a64(0x78DFE16C).unwrap(),
        Instr::Ldursh32LdstUnscaled {
            imm9: 510,
            Rn: 11,
            Rt: 12
        }
    );
}

#[test]
fn test_ldrsh() {
    // ldrsh x14, [x21, x13]
    assert_eq!(
        decode_a64(0x78AD6AAE).unwrap(),
        Instr::Ldrsh64LdstRegoff {
            Rm: 13,
            S: 0,
            Rn: 21,
            Rt: 14
        }
    );
    // ldrsh w12, [x0], #2
    assert_eq!(
        decode_a64(0x78C0240C).unwrap(),
        Instr::Ldrsh32LdstImmpost {
            imm9: 2,
            Rn: 0,
            Rt: 12
        }
    );
    // ldrsh w2, [x1], #2
    assert_eq!(
        decode_a64(0x78C02422).unwrap(),
        Instr::Ldrsh32LdstImmpost {
            imm9: 2,
            Rn: 1,
            Rt: 2
        }
    );
    // ldrsh w9, [x1], #2
    assert_eq!(
        decode_a64(0x78C02429).unwrap(),
        Instr::Ldrsh32LdstImmpost {
            imm9: 2,
            Rn: 1,
            Rt: 9
        }
    );
    // ldrsh w11, [x10], #2
    assert_eq!(
        decode_a64(0x78C0254B).unwrap(),
        Instr::Ldrsh32LdstImmpost {
            imm9: 2,
            Rn: 10,
            Rt: 11
        }
    );
    // ldrsh w10, [x9, #2]!
    assert_eq!(
        decode_a64(0x78C02D2A).unwrap(),
        Instr::Ldrsh32LdstImmpre {
            imm9: 2,
            Rn: 9,
            Rt: 10
        }
    );
    // ldrsh w9, [x22, #4]!
    assert_eq!(
        decode_a64(0x78C04EC9).unwrap(),
        Instr::Ldrsh32LdstImmpre {
            imm9: 4,
            Rn: 22,
            Rt: 9
        }
    );
    // ldrsh w6, [x5, #8]!
    assert_eq!(
        decode_a64(0x78C08CA6).unwrap(),
        Instr::Ldrsh32LdstImmpre {
            imm9: 8,
            Rn: 5,
            Rt: 6
        }
    );
    // ldrsh w13, [x12, #-0x60]!
    assert_eq!(
        decode_a64(0x78DA0D8D).unwrap(),
        Instr::Ldrsh32LdstImmpre {
            imm9: 416,
            Rn: 12,
            Rt: 13
        }
    );
    // ldrsh w13, [x9], #0xfffffffffffffffe
    assert_eq!(
        decode_a64(0x78DFE52D).unwrap(),
        Instr::Ldrsh32LdstImmpost {
            imm9: 510,
            Rn: 9,
            Rt: 13
        }
    );
    // ldrsh w10, [x26], #0xfffffffffffffffe
    assert_eq!(
        decode_a64(0x78DFE74A).unwrap(),
        Instr::Ldrsh32LdstImmpost {
            imm9: 510,
            Rn: 26,
            Rt: 10
        }
    );
    // ldrsh w13, [x9, #-2]!
    assert_eq!(
        decode_a64(0x78DFED2D).unwrap(),
        Instr::Ldrsh32LdstImmpre {
            imm9: 510,
            Rn: 9,
            Rt: 13
        }
    );
    // ldrsh w9, [x11, #-2]!
    assert_eq!(
        decode_a64(0x78DFED69).unwrap(),
        Instr::Ldrsh32LdstImmpre {
            imm9: 510,
            Rn: 11,
            Rt: 9
        }
    );
    // ldrsh w15, [x20, w15, sxtw #1]
    assert_eq!(
        decode_a64(0x78EFDA8F).unwrap(),
        Instr::Ldrsh32LdstRegoff {
            Rm: 15,
            S: 1,
            Rn: 20,
            Rt: 15
        }
    );
    // ldrsh w0, [x0]
    assert_eq!(
        decode_a64(0x79C00000).unwrap(),
        Instr::Ldrsh32LdstPos {
            imm12: 0,
            Rn: 0,
            Rt: 0
        }
    );
    // ldrsh w0, [x8, #4]
    assert_eq!(
        decode_a64(0x79C00900).unwrap(),
        Instr::Ldrsh32LdstPos {
            imm12: 2,
            Rn: 8,
            Rt: 0
        }
    );
    // ldrsh w8, [x8, #6]
    assert_eq!(
        decode_a64(0x79C00D08).unwrap(),
        Instr::Ldrsh32LdstPos {
            imm12: 3,
            Rn: 8,
            Rt: 8
        }
    );
    // ldrsh w9, [x17, #0x80]
    assert_eq!(
        decode_a64(0x79C10229).unwrap(),
        Instr::Ldrsh32LdstPos {
            imm12: 64,
            Rn: 17,
            Rt: 9
        }
    );
}

#[test]
fn test_ccmp() {
    // ccmp w1, w4, #4, lt
    assert_eq!(
        decode_a64(0x7A44B024).unwrap(),
        Instr::Ccmp32CondcmpReg {
            Rm: 4,
            cond: 11,
            Rn: 1,
            nzcv: 4
        }
    );
    // ccmp w23, #0x1f, #2, ne
    assert_eq!(
        decode_a64(0x7A5F1AE2).unwrap(),
        Instr::Ccmp32CondcmpImm {
            imm5: 31,
            cond: 1,
            Rn: 23,
            nzcv: 2
        }
    );
    // ccmp x8, #0, #0, eq
    assert_eq!(
        decode_a64(0xFA400900).unwrap(),
        Instr::Ccmp64CondcmpImm {
            imm5: 0,
            cond: 0,
            Rn: 8,
            nzcv: 0
        }
    );
    // ccmp x9, x2, #2, hs
    assert_eq!(
        decode_a64(0xFA422122).unwrap(),
        Instr::Ccmp64CondcmpReg {
            Rm: 2,
            cond: 2,
            Rn: 9,
            nzcv: 2
        }
    );
    // ccmp x10, x14, #0, lo
    assert_eq!(
        decode_a64(0xFA4E3140).unwrap(),
        Instr::Ccmp64CondcmpReg {
            Rm: 14,
            cond: 3,
            Rn: 10,
            nzcv: 0
        }
    );
    // ccmp x8, x26, #4, ne
    assert_eq!(
        decode_a64(0xFA5A1104).unwrap(),
        Instr::Ccmp64CondcmpReg {
            Rm: 26,
            cond: 1,
            Rn: 8,
            nzcv: 4
        }
    );
}

#[test]
fn test_ldxr() {
    // ldxr w8, [x0]
    assert_eq!(
        decode_a64(0x885F7C08).unwrap(),
        Instr::LdxrLr32Ldstexcl {
            Rs: 31,
            Rn: 0,
            Rt: 8
        }
    );
    // ldxr w11, [x10]
    assert_eq!(
        decode_a64(0x885F7D4B).unwrap(),
        Instr::LdxrLr32Ldstexcl {
            Rs: 31,
            Rn: 10,
            Rt: 11
        }
    );
    // ldxr x0, [x8]
    assert_eq!(
        decode_a64(0xC85F7D00).unwrap(),
        Instr::LdxrLr64Ldstexcl {
            Rs: 31,
            Rn: 8,
            Rt: 0
        }
    );
    // ldxr x9, [x20]
    assert_eq!(
        decode_a64(0xC85F7E89).unwrap(),
        Instr::LdxrLr64Ldstexcl {
            Rs: 31,
            Rn: 20,
            Rt: 9
        }
    );
    // ldxr x8, [x21]
    assert_eq!(
        decode_a64(0xC85F7EA8).unwrap(),
        Instr::LdxrLr64Ldstexcl {
            Rs: 31,
            Rn: 21,
            Rt: 8
        }
    );
    // ldxr x26, [x27]
    assert_eq!(
        decode_a64(0xC85F7F7A).unwrap(),
        Instr::LdxrLr64Ldstexcl {
            Rs: 31,
            Rn: 27,
            Rt: 26
        }
    );
}

#[test]
fn test_ldaxr() {
    // ldaxr w8, [x2]
    assert_eq!(
        decode_a64(0x885FFC48).unwrap(),
        Instr::LdaxrLr32Ldstexcl {
            Rs: 31,
            Rn: 2,
            Rt: 8
        }
    );
    // ldaxr w0, [x8]
    assert_eq!(
        decode_a64(0x885FFD00).unwrap(),
        Instr::LdaxrLr32Ldstexcl {
            Rs: 31,
            Rn: 8,
            Rt: 0
        }
    );
    // ldaxr w10, [x27]
    assert_eq!(
        decode_a64(0x885FFF6A).unwrap(),
        Instr::LdaxrLr32Ldstexcl {
            Rs: 31,
            Rn: 27,
            Rt: 10
        }
    );
    // ldaxr w10, [x28]
    assert_eq!(
        decode_a64(0x885FFF8A).unwrap(),
        Instr::LdaxrLr32Ldstexcl {
            Rs: 31,
            Rn: 28,
            Rt: 10
        }
    );
    // ldaxr x8, [x0]
    assert_eq!(
        decode_a64(0xC85FFC08).unwrap(),
        Instr::LdaxrLr64Ldstexcl {
            Rs: 31,
            Rn: 0,
            Rt: 8
        }
    );
    // ldaxr x0, [x9]
    assert_eq!(
        decode_a64(0xC85FFD20).unwrap(),
        Instr::LdaxrLr64Ldstexcl {
            Rs: 31,
            Rn: 9,
            Rt: 0
        }
    );
}

#[test]
fn test_ldar() {
    // ldar w8, [x8]
    assert_eq!(
        decode_a64(0x88DFFD08).unwrap(),
        Instr::LdarLr32Ldstexcl {
            Rs: 31,
            Rn: 8,
            Rt: 8
        }
    );
    // ldar w9, [x27]
    assert_eq!(
        decode_a64(0x88DFFF69).unwrap(),
        Instr::LdarLr32Ldstexcl {
            Rs: 31,
            Rn: 27,
            Rt: 9
        }
    );
    // ldar x0, [x8]
    assert_eq!(
        decode_a64(0xC8DFFD00).unwrap(),
        Instr::LdarLr64Ldstexcl {
            Rs: 31,
            Rn: 8,
            Rt: 0
        }
    );
    // ldar x9, [x8]
    assert_eq!(
        decode_a64(0xC8DFFD09).unwrap(),
        Instr::LdarLr64Ldstexcl {
            Rs: 31,
            Rn: 8,
            Rt: 9
        }
    );
    // ldar x28, [x27]
    assert_eq!(
        decode_a64(0xC8DFFF7C).unwrap(),
        Instr::LdarLr64Ldstexcl {
            Rs: 31,
            Rn: 27,
            Rt: 28
        }
    );
    // ldar x24, [x28]
    assert_eq!(
        decode_a64(0xC8DFFF98).unwrap(),
        Instr::LdarLr64Ldstexcl {
            Rs: 31,
            Rn: 28,
            Rt: 24
        }
    );
}

#[test]
fn test_adrp() {
    // adrp x2, #0x1140000
    assert_eq!(
        decode_a64(0x90008A02).unwrap(),
        Instr::AdrpOnlyPcreladdr {
            immlo: 0,
            immhi: 1104,
            Rd: 2
        }
    );
    // adrp x10, #0x53d000
    assert_eq!(
        decode_a64(0xB00029EA).unwrap(),
        Instr::AdrpOnlyPcreladdr {
            immlo: 1,
            immhi: 335,
            Rd: 10
        }
    );
    // adrp x9, #0x1895000
    assert_eq!(
        decode_a64(0xB000C4A9).unwrap(),
        Instr::AdrpOnlyPcreladdr {
            immlo: 1,
            immhi: 1573,
            Rd: 9
        }
    );
    // adrp x20, #0x19fd000
    assert_eq!(
        decode_a64(0xB000CFF4).unwrap(),
        Instr::AdrpOnlyPcreladdr {
            immlo: 1,
            immhi: 1663,
            Rd: 20
        }
    );
    // adrp x8, #0xfc2000
    assert_eq!(
        decode_a64(0xD0007E08).unwrap(),
        Instr::AdrpOnlyPcreladdr {
            immlo: 2,
            immhi: 1008,
            Rd: 8
        }
    );
    // adrp x2, #0x3000
    assert_eq!(
        decode_a64(0xF0000002).unwrap(),
        Instr::AdrpOnlyPcreladdr {
            immlo: 3,
            immhi: 0,
            Rd: 2
        }
    );
    // adrp x21, #0xfffffffffffff000
    assert_eq!(
        decode_a64(0xF0FFFFF5).unwrap(),
        Instr::AdrpOnlyPcreladdr {
            immlo: 3,
            immhi: 524287,
            Rd: 21
        }
    );
}

#[test]
fn test_sxtw() {
    // sxtw x1, w1
    assert_eq!(
        decode_a64(0x93407C21).unwrap(),
        Instr::Sbfm64MBitfield {
            immr: 0,
            imms: 31,
            Rn: 1,
            Rd: 1
        }
    );
    // sxtw x14, w4
    assert_eq!(
        decode_a64(0x93407C8E).unwrap(),
        Instr::Sbfm64MBitfield {
            immr: 0,
            imms: 31,
            Rn: 4,
            Rd: 14
        }
    );
    // sxtw x22, w8
    assert_eq!(
        decode_a64(0x93407D16).unwrap(),
        Instr::Sbfm64MBitfield {
            immr: 0,
            imms: 31,
            Rn: 8,
            Rd: 22
        }
    );
    // sxtw x26, w25
    assert_eq!(
        decode_a64(0x93407F3A).unwrap(),
        Instr::Sbfm64MBitfield {
            immr: 0,
            imms: 31,
            Rn: 25,
            Rd: 26
        }
    );
    // sxtw x1, w27
    assert_eq!(
        decode_a64(0x93407F61).unwrap(),
        Instr::Sbfm64MBitfield {
            immr: 0,
            imms: 31,
            Rn: 27,
            Rd: 1
        }
    );
    // sxtw x24, w29
    assert_eq!(
        decode_a64(0x93407FB8).unwrap(),
        Instr::Sbfm64MBitfield {
            immr: 0,
            imms: 31,
            Rn: 29,
            Rd: 24
        }
    );
}

#[test]
fn test_sbfiz() {
    // sbfiz x13, x13, #0x20, #0x20
    assert_eq!(
        decode_a64(0x93607DAD).unwrap(),
        Instr::Sbfm64MBitfield {
            immr: 32,
            imms: 31,
            Rn: 13,
            Rd: 13
        }
    );
    // sbfiz x12, x8, #3, #0x20
    assert_eq!(
        decode_a64(0x937D7D0C).unwrap(),
        Instr::Sbfm64MBitfield {
            immr: 61,
            imms: 31,
            Rn: 8,
            Rd: 12
        }
    );
    // sbfiz x14, x6, #2, #0x20
    assert_eq!(
        decode_a64(0x937E7CCE).unwrap(),
        Instr::Sbfm64MBitfield {
            immr: 62,
            imms: 31,
            Rn: 6,
            Rd: 14
        }
    );
    // sbfiz x7, x7, #2, #0x20
    assert_eq!(
        decode_a64(0x937E7CE7).unwrap(),
        Instr::Sbfm64MBitfield {
            immr: 62,
            imms: 31,
            Rn: 7,
            Rd: 7
        }
    );
    // sbfiz x2, x21, #2, #0x20
    assert_eq!(
        decode_a64(0x937E7EA2).unwrap(),
        Instr::Sbfm64MBitfield {
            immr: 62,
            imms: 31,
            Rn: 21,
            Rd: 2
        }
    );
    // sbfiz x8, x8, #1, #8
    assert_eq!(
        decode_a64(0x937F1D08).unwrap(),
        Instr::Sbfm64MBitfield {
            immr: 63,
            imms: 7,
            Rn: 8,
            Rd: 8
        }
    );
}

#[test]
fn test_mneg() {
    // mneg x8, x0, x8
    assert_eq!(
        decode_a64(0x9B08FC08).unwrap(),
        Instr::Msub64ADp3Src {
            Rm: 8,
            Ra: 31,
            Rn: 0,
            Rd: 8
        }
    );
    // mneg x14, x10, x12
    assert_eq!(
        decode_a64(0x9B0CFD4E).unwrap(),
        Instr::Msub64ADp3Src {
            Rm: 12,
            Ra: 31,
            Rn: 10,
            Rd: 14
        }
    );
    // mneg x15, x15, x12
    assert_eq!(
        decode_a64(0x9B0CFDEF).unwrap(),
        Instr::Msub64ADp3Src {
            Rm: 12,
            Ra: 31,
            Rn: 15,
            Rd: 15
        }
    );
    // mneg x15, x11, x13
    assert_eq!(
        decode_a64(0x9B0DFD6F).unwrap(),
        Instr::Msub64ADp3Src {
            Rm: 13,
            Ra: 31,
            Rn: 11,
            Rd: 15
        }
    );
    // mneg x0, x0, x17
    assert_eq!(
        decode_a64(0x9B11FC00).unwrap(),
        Instr::Msub64ADp3Src {
            Rm: 17,
            Ra: 31,
            Rn: 0,
            Rd: 0
        }
    );
    // mneg x0, x0, x18
    assert_eq!(
        decode_a64(0x9B12FC00).unwrap(),
        Instr::Msub64ADp3Src {
            Rm: 18,
            Ra: 31,
            Rn: 0,
            Rd: 0
        }
    );
}

#[test]
fn test_smaddl() {
    // smaddl x8, w6, w3, x8
    assert_eq!(
        decode_a64(0x9B2320C8).unwrap(),
        Instr::Smaddl64WaDp3Src {
            Rm: 3,
            Ra: 8,
            Rn: 6,
            Rd: 8
        }
    );
    // smaddl x8, w9, w8, x21
    assert_eq!(
        decode_a64(0x9B285528).unwrap(),
        Instr::Smaddl64WaDp3Src {
            Rm: 8,
            Ra: 21,
            Rn: 9,
            Rd: 8
        }
    );
    // smaddl x8, w2, w9, x8
    assert_eq!(
        decode_a64(0x9B292048).unwrap(),
        Instr::Smaddl64WaDp3Src {
            Rm: 9,
            Ra: 8,
            Rn: 2,
            Rd: 8
        }
    );
    // smaddl x10, w9, w10, x5
    assert_eq!(
        decode_a64(0x9B2A152A).unwrap(),
        Instr::Smaddl64WaDp3Src {
            Rm: 10,
            Ra: 5,
            Rn: 9,
            Rd: 10
        }
    );
    // smaddl x9, w13, w27, x9
    assert_eq!(
        decode_a64(0x9B3B25A9).unwrap(),
        Instr::Smaddl64WaDp3Src {
            Rm: 27,
            Ra: 9,
            Rn: 13,
            Rd: 9
        }
    );
    // smaddl x20, w26, w28, x19
    assert_eq!(
        decode_a64(0x9B3C4F54).unwrap(),
        Instr::Smaddl64WaDp3Src {
            Rm: 28,
            Ra: 19,
            Rn: 26,
            Rd: 20
        }
    );
}

#[test]
fn test_smulh() {
    // smulh x8, x0, x8
    assert_eq!(
        decode_a64(0x9B487C08).unwrap(),
        Instr::Smulh64Dp3Src {
            Rm: 8,
            Ra: 31,
            Rn: 0,
            Rd: 8
        }
    );
    // smulh x8, x22, x8
    assert_eq!(
        decode_a64(0x9B487EC8).unwrap(),
        Instr::Smulh64Dp3Src {
            Rm: 8,
            Ra: 31,
            Rn: 22,
            Rd: 8
        }
    );
    // smulh x8, x8, x9
    assert_eq!(
        decode_a64(0x9B497D08).unwrap(),
        Instr::Smulh64Dp3Src {
            Rm: 9,
            Ra: 31,
            Rn: 8,
            Rd: 8
        }
    );
    // smulh x9, x8, x9
    assert_eq!(
        decode_a64(0x9B497D09).unwrap(),
        Instr::Smulh64Dp3Src {
            Rm: 9,
            Ra: 31,
            Rn: 8,
            Rd: 9
        }
    );
    // smulh x9, x19, x9
    assert_eq!(
        decode_a64(0x9B497E69).unwrap(),
        Instr::Smulh64Dp3Src {
            Rm: 9,
            Ra: 31,
            Rn: 19,
            Rd: 9
        }
    );
    // smulh x8, x8, x11
    assert_eq!(
        decode_a64(0x9B4B7D08).unwrap(),
        Instr::Smulh64Dp3Src {
            Rm: 11,
            Ra: 31,
            Rn: 8,
            Rd: 8
        }
    );
    // smulh x10, x10, x11
    assert_eq!(
        decode_a64(0x9B4B7D4A).unwrap(),
        Instr::Smulh64Dp3Src {
            Rm: 11,
            Ra: 31,
            Rn: 10,
            Rd: 10
        }
    );
}

#[test]
fn test_umaddl() {
    // umaddl x8, w3, w1, x8
    assert_eq!(
        decode_a64(0x9BA12068).unwrap(),
        Instr::Umaddl64WaDp3Src {
            Rm: 1,
            Ra: 8,
            Rn: 3,
            Rd: 8
        }
    );
    // umaddl x22, w22, w7, x19
    assert_eq!(
        decode_a64(0x9BA74ED6).unwrap(),
        Instr::Umaddl64WaDp3Src {
            Rm: 7,
            Ra: 19,
            Rn: 22,
            Rd: 22
        }
    );
    // umaddl x9, w19, w9, x20
    assert_eq!(
        decode_a64(0x9BA95269).unwrap(),
        Instr::Umaddl64WaDp3Src {
            Rm: 9,
            Ra: 20,
            Rn: 19,
            Rd: 9
        }
    );
    // umaddl x17, w17, w20, x9
    assert_eq!(
        decode_a64(0x9BB42631).unwrap(),
        Instr::Umaddl64WaDp3Src {
            Rm: 20,
            Ra: 9,
            Rn: 17,
            Rd: 17
        }
    );
    // umaddl x22, w10, w21, x24
    assert_eq!(
        decode_a64(0x9BB56156).unwrap(),
        Instr::Umaddl64WaDp3Src {
            Rm: 21,
            Ra: 24,
            Rn: 10,
            Rd: 22
        }
    );
    // umaddl x9, w26, w28, x19
    assert_eq!(
        decode_a64(0x9BBC4F49).unwrap(),
        Instr::Umaddl64WaDp3Src {
            Rm: 28,
            Ra: 19,
            Rn: 26,
            Rd: 9
        }
    );
}

#[test]
fn test_umull() {
    // umull x1, w21, w8
    assert_eq!(
        decode_a64(0x9BA87EA1).unwrap(),
        Instr::Umaddl64WaDp3Src {
            Rm: 8,
            Ra: 31,
            Rn: 21,
            Rd: 1
        }
    );
    // umull x4, w8, w9
    assert_eq!(
        decode_a64(0x9BA97D04).unwrap(),
        Instr::Umaddl64WaDp3Src {
            Rm: 9,
            Ra: 31,
            Rn: 8,
            Rd: 4
        }
    );
    // umull x11, w11, w10
    assert_eq!(
        decode_a64(0x9BAA7D6B).unwrap(),
        Instr::Umaddl64WaDp3Src {
            Rm: 10,
            Ra: 31,
            Rn: 11,
            Rd: 11
        }
    );
    // umull x15, w12, w13
    assert_eq!(
        decode_a64(0x9BAD7D8F).unwrap(),
        Instr::Umaddl64WaDp3Src {
            Rm: 13,
            Ra: 31,
            Rn: 12,
            Rd: 15
        }
    );
    // umull x22, w23, w13
    assert_eq!(
        decode_a64(0x9BAD7EF6).unwrap(),
        Instr::Umaddl64WaDp3Src {
            Rm: 13,
            Ra: 31,
            Rn: 23,
            Rd: 22
        }
    );
    // umull x23, w23, w13
    assert_eq!(
        decode_a64(0x9BAD7EF7).unwrap(),
        Instr::Umaddl64WaDp3Src {
            Rm: 13,
            Ra: 31,
            Rn: 23,
            Rd: 23
        }
    );
}

#[test]
fn test_umulh() {
    // umulh x8, x2, x0
    assert_eq!(
        decode_a64(0x9BC07C48).unwrap(),
        Instr::Umulh64Dp3Src {
            Rm: 0,
            Ra: 31,
            Rn: 2,
            Rd: 8
        }
    );
    // umulh x9, x9, x11
    assert_eq!(
        decode_a64(0x9BCB7D29).unwrap(),
        Instr::Umulh64Dp3Src {
            Rm: 11,
            Ra: 31,
            Rn: 9,
            Rd: 9
        }
    );
    // umulh x9, x27, x11
    assert_eq!(
        decode_a64(0x9BCB7F69).unwrap(),
        Instr::Umulh64Dp3Src {
            Rm: 11,
            Ra: 31,
            Rn: 27,
            Rd: 9
        }
    );
    // umulh x3, x2, x12
    assert_eq!(
        decode_a64(0x9BCC7C43).unwrap(),
        Instr::Umulh64Dp3Src {
            Rm: 12,
            Ra: 31,
            Rn: 2,
            Rd: 3
        }
    );
    // umulh x10, x10, x23
    assert_eq!(
        decode_a64(0x9BD77D4A).unwrap(),
        Instr::Umulh64Dp3Src {
            Rm: 23,
            Ra: 31,
            Rn: 10,
            Rd: 10
        }
    );
    // umulh x8, x23, x24
    assert_eq!(
        decode_a64(0x9BD87EE8).unwrap(),
        Instr::Umulh64Dp3Src {
            Rm: 24,
            Ra: 31,
            Rn: 23,
            Rd: 8
        }
    );
}

#[test]
fn test_fcvtas() {
    // fcvtas x0, s0
    assert_eq!(
        decode_a64(0x9E240000).unwrap(),
        Instr::Fcvtas64SFloat2Int { Rn: 0, Rd: 0 }
    );
    // fcvtas x0, d0
    assert_eq!(
        decode_a64(0x9E640000).unwrap(),
        Instr::Fcvtas64DFloat2Int { Rn: 0, Rd: 0 }
    );
}

#[test]
fn test_ldr() {
    // ldr w5, [x0], #4
    assert_eq!(
        decode_a64(0xB8404405).unwrap(),
        Instr::Ldr32LdstImmpost {
            imm9: 4,
            Rn: 0,
            Rt: 5
        }
    );
    // ldr w6, [x0, #4]!
    assert_eq!(
        decode_a64(0xB8404C06).unwrap(),
        Instr::Ldr32LdstImmpre {
            imm9: 4,
            Rn: 0,
            Rt: 6
        }
    );
    // ldr w23, [x20], #0xfffffffffffffffc
    assert_eq!(
        decode_a64(0xB85FC697).unwrap(),
        Instr::Ldr32LdstImmpost {
            imm9: 508,
            Rn: 20,
            Rt: 23
        }
    );
    // ldr w15, [x16, w15, sxtw #2]
    assert_eq!(
        decode_a64(0xB86FDA0F).unwrap(),
        Instr::Ldr32LdstRegoff {
            Rm: 15,
            S: 1,
            Rn: 16,
            Rt: 15
        }
    );
    // ldr w3, [x0]
    assert_eq!(
        decode_a64(0xB9400003).unwrap(),
        Instr::Ldr32LdstPos {
            imm12: 0,
            Rn: 0,
            Rt: 3
        }
    );
    // ldr w0, [x8, #0x73c]
    assert_eq!(
        decode_a64(0xB9473D00).unwrap(),
        Instr::Ldr32LdstPos {
            imm12: 463,
            Rn: 8,
            Rt: 0
        }
    );
    // ldr s2, [x3, #0x6c]
    assert_eq!(
        decode_a64(0xBD406C62).unwrap(),
        Instr::LdrSLdstPos {
            imm12: 27,
            Rn: 3,
            Rt: 2
        }
    );
    // ldr s17, [sp, #0x14c]
    assert_eq!(
        decode_a64(0xBD414FF1).unwrap(),
        Instr::LdrSLdstPos {
            imm12: 83,
            Rn: 31,
            Rt: 17
        }
    );
    // ldr s1, [x8, #0xd08]
    assert_eq!(
        decode_a64(0xBD4D0901).unwrap(),
        Instr::LdrSLdstPos {
            imm12: 834,
            Rn: 8,
            Rt: 1
        }
    );
    // ldr x1, [x22], #8
    assert_eq!(
        decode_a64(0xF84086C1).unwrap(),
        Instr::Ldr64LdstImmpost {
            imm9: 8,
            Rn: 22,
            Rt: 1
        }
    );
    // ldr x0, [x8, #8]!
    assert_eq!(
        decode_a64(0xF8408D00).unwrap(),
        Instr::Ldr64LdstImmpre {
            imm9: 8,
            Rn: 8,
            Rt: 0
        }
    );
    // ldr x22, [x21, #0x10]!
    assert_eq!(
        decode_a64(0xF8410EB6).unwrap(),
        Instr::Ldr64LdstImmpre {
            imm9: 16,
            Rn: 21,
            Rt: 22
        }
    );
    // ldr x0, [x20], #0x30
    assert_eq!(
        decode_a64(0xF8430680).unwrap(),
        Instr::Ldr64LdstImmpost {
            imm9: 48,
            Rn: 20,
            Rt: 0
        }
    );
    // ldr x8, [x22], #0x30
    assert_eq!(
        decode_a64(0xF84306C8).unwrap(),
        Instr::Ldr64LdstImmpost {
            imm9: 48,
            Rn: 22,
            Rt: 8
        }
    );
    // ldr x25, [x22, #0xa0]!
    assert_eq!(
        decode_a64(0xF84A0ED9).unwrap(),
        Instr::Ldr64LdstImmpre {
            imm9: 160,
            Rn: 22,
            Rt: 25
        }
    );
    // ldr x10, [x20, #-0x10]!
    assert_eq!(
        decode_a64(0xF85F0E8A).unwrap(),
        Instr::Ldr64LdstImmpre {
            imm9: 496,
            Rn: 20,
            Rt: 10
        }
    );
    // ldr x8, [x21, #-0x10]!
    assert_eq!(
        decode_a64(0xF85F0EA8).unwrap(),
        Instr::Ldr64LdstImmpre {
            imm9: 496,
            Rn: 21,
            Rt: 8
        }
    );
    // ldr x0, [x8, w19, sxtw #3]
    assert_eq!(
        decode_a64(0xF873D900).unwrap(),
        Instr::Ldr64LdstRegoff {
            Rm: 19,
            S: 1,
            Rn: 8,
            Rt: 0
        }
    );
    // ldr x0, [x0]
    assert_eq!(
        decode_a64(0xF9400000).unwrap(),
        Instr::Ldr64LdstPos {
            imm12: 0,
            Rn: 0,
            Rt: 0
        }
    );
    // ldr d2, [x9], #0xfffffffffffffff8
    assert_eq!(
        decode_a64(0xFC5F8522).unwrap(),
        Instr::LdrDLdstImmpost {
            imm9: 504,
            Rn: 9,
            Rt: 2
        }
    );
}

#[test]
fn test_ldursw() {
    // ldursw x8, [x9, #1]
    assert_eq!(
        decode_a64(0xB8801128).unwrap(),
        Instr::Ldursw64LdstUnscaled {
            imm9: 1,
            Rn: 9,
            Rt: 8
        }
    );
    // ldursw x16, [x10, #-0xc0]
    assert_eq!(
        decode_a64(0xB8940150).unwrap(),
        Instr::Ldursw64LdstUnscaled {
            imm9: 320,
            Rn: 10,
            Rt: 16
        }
    );
    // ldursw x8, [x29, #-0x3c]
    assert_eq!(
        decode_a64(0xB89C43A8).unwrap(),
        Instr::Ldursw64LdstUnscaled {
            imm9: 452,
            Rn: 29,
            Rt: 8
        }
    );
    // ldursw x21, [x29, #-0x14]
    assert_eq!(
        decode_a64(0xB89EC3B5).unwrap(),
        Instr::Ldursw64LdstUnscaled {
            imm9: 492,
            Rn: 29,
            Rt: 21
        }
    );
    // ldursw x10, [x23, #-0x10]
    assert_eq!(
        decode_a64(0xB89F02EA).unwrap(),
        Instr::Ldursw64LdstUnscaled {
            imm9: 496,
            Rn: 23,
            Rt: 10
        }
    );
    // ldursw x13, [x11, #-4]
    assert_eq!(
        decode_a64(0xB89FC16D).unwrap(),
        Instr::Ldursw64LdstUnscaled {
            imm9: 508,
            Rn: 11,
            Rt: 13
        }
    );
}

#[test]
fn test_ldrsw() {
    // ldrsw x7, [x2], #4
    assert_eq!(
        decode_a64(0xB8804447).unwrap(),
        Instr::Ldrsw64LdstImmpost {
            imm9: 4,
            Rn: 2,
            Rt: 7
        }
    );
    // ldrsw x14, [x10], #4
    assert_eq!(
        decode_a64(0xB880454E).unwrap(),
        Instr::Ldrsw64LdstImmpost {
            imm9: 4,
            Rn: 10,
            Rt: 14
        }
    );
    // ldrsw x19, [x18], #4
    assert_eq!(
        decode_a64(0xB8804653).unwrap(),
        Instr::Ldrsw64LdstImmpost {
            imm9: 4,
            Rn: 18,
            Rt: 19
        }
    );
    // ldrsw x23, [x26], #4
    assert_eq!(
        decode_a64(0xB8804757).unwrap(),
        Instr::Ldrsw64LdstImmpost {
            imm9: 4,
            Rn: 26,
            Rt: 23
        }
    );
    // ldrsw x4, [x0, #8]!
    assert_eq!(
        decode_a64(0xB8808C04).unwrap(),
        Instr::Ldrsw64LdstImmpre {
            imm9: 8,
            Rn: 0,
            Rt: 4
        }
    );
    // ldrsw x3, [x18, #8]!
    assert_eq!(
        decode_a64(0xB8808E43).unwrap(),
        Instr::Ldrsw64LdstImmpre {
            imm9: 8,
            Rn: 18,
            Rt: 3
        }
    );
    // ldrsw x8, [x21, #0x1c]!
    assert_eq!(
        decode_a64(0xB881CEA8).unwrap(),
        Instr::Ldrsw64LdstImmpre {
            imm9: 28,
            Rn: 21,
            Rt: 8
        }
    );
    // ldrsw x8, [x28, #0x2c]!
    assert_eq!(
        decode_a64(0xB882CF88).unwrap(),
        Instr::Ldrsw64LdstImmpre {
            imm9: 44,
            Rn: 28,
            Rt: 8
        }
    );
    // ldrsw x9, [x8, #0x68]!
    assert_eq!(
        decode_a64(0xB8868D09).unwrap(),
        Instr::Ldrsw64LdstImmpre {
            imm9: 104,
            Rn: 8,
            Rt: 9
        }
    );
    // ldrsw x8, [x23], #0x84
    assert_eq!(
        decode_a64(0xB88846E8).unwrap(),
        Instr::Ldrsw64LdstImmpost {
            imm9: 132,
            Rn: 23,
            Rt: 8
        }
    );
    // ldrsw x25, [x9, #-0x14]!
    assert_eq!(
        decode_a64(0xB89ECD39).unwrap(),
        Instr::Ldrsw64LdstImmpre {
            imm9: 492,
            Rn: 9,
            Rt: 25
        }
    );
    // ldrsw x15, [x16], #0xfffffffffffffffc
    assert_eq!(
        decode_a64(0xB89FC60F).unwrap(),
        Instr::Ldrsw64LdstImmpost {
            imm9: 508,
            Rn: 16,
            Rt: 15
        }
    );
    // ldrsw x26, [x8, x21, lsl #2]
    assert_eq!(
        decode_a64(0xB8B5791A).unwrap(),
        Instr::Ldrsw64LdstRegoff {
            Rm: 21,
            S: 1,
            Rn: 8,
            Rt: 26
        }
    );
    // ldrsw x12, [x21, w28, sxtw #2]
    assert_eq!(
        decode_a64(0xB8BCDAAC).unwrap(),
        Instr::Ldrsw64LdstRegoff {
            Rm: 28,
            S: 1,
            Rn: 21,
            Rt: 12
        }
    );
    // ldrsw x2, [x8]
    assert_eq!(
        decode_a64(0xB9800102).unwrap(),
        Instr::Ldrsw64LdstPos {
            imm12: 0,
            Rn: 8,
            Rt: 2
        }
    );
    // ldrsw x22, [x22]
    assert_eq!(
        decode_a64(0xB98002D6).unwrap(),
        Instr::Ldrsw64LdstPos {
            imm12: 0,
            Rn: 22,
            Rt: 22
        }
    );
    // ldrsw x13, [x0, #8]
    assert_eq!(
        decode_a64(0xB980080D).unwrap(),
        Instr::Ldrsw64LdstPos {
            imm12: 2,
            Rn: 0,
            Rt: 13
        }
    );
    // ldrsw x8, [x22, #0x1c]
    assert_eq!(
        decode_a64(0xB9801EC8).unwrap(),
        Instr::Ldrsw64LdstPos {
            imm12: 7,
            Rn: 22,
            Rt: 8
        }
    );
}

#[test]
fn test_dmb() {
    // dmb ishld
    assert_eq!(decode_a64(0xD50339BF).unwrap(), Instr::DmbBoBarriers);
    // dmb ishst
    assert_eq!(decode_a64(0xD5033ABF).unwrap(), Instr::DmbBoBarriers);
    // dmb ish
    assert_eq!(decode_a64(0xD5033BBF).unwrap(), Instr::DmbBoBarriers);
}

#[test]
fn test_dsb() {
    // dsb ish
    assert_eq!(decode_a64(0xD5033B9F).unwrap(), Instr::DsbBoBarriers);
    // dsb sy
    assert_eq!(decode_a64(0xD5033F9F).unwrap(), Instr::DsbBoBarriers);
}

#[test]
fn test_dc() {
    // dc cvau, x11
    assert_eq!(
        decode_a64(0xD50B7B2B).unwrap(),
        Instr::SysCrSysteminstrs {
            op1: 3,
            CRn: 7,
            CRm: 11,
            op2: 1,
            Rt: 11
        }
    );
    // dc civac, x10
    assert_eq!(
        decode_a64(0xD50B7E2A).unwrap(),
        Instr::SysCrSysteminstrs {
            op1: 3,
            CRn: 7,
            CRm: 14,
            op2: 1,
            Rt: 10
        }
    );
}

#[test]
fn test_negs() {
    // negs x0, x0
    assert_eq!(
        decode_a64(0xEB0003E0).unwrap(),
        Instr::Subs64AddsubShift {
            Rm: 0,
            Rn: 31,
            Rd: 0
        }
    );
    // negs x2, x2
    assert_eq!(
        decode_a64(0xEB0203E2).unwrap(),
        Instr::Subs64AddsubShift {
            Rm: 2,
            Rn: 31,
            Rd: 2
        }
    );
    // negs x4, x5
    assert_eq!(
        decode_a64(0xEB0503E4).unwrap(),
        Instr::Subs64AddsubShift {
            Rm: 5,
            Rn: 31,
            Rd: 4
        }
    );
    // negs x15, x15
    assert_eq!(
        decode_a64(0xEB0F03EF).unwrap(),
        Instr::Subs64AddsubShift {
            Rm: 15,
            Rn: 31,
            Rd: 15
        }
    );
    // negs x17, x17
    assert_eq!(
        decode_a64(0xEB1103F1).unwrap(),
        Instr::Subs64AddsubShift {
            Rm: 17,
            Rn: 31,
            Rd: 17
        }
    );
}

#[test]
fn test_prfm() {
    // prfm pldl2strm, [x8]
    assert_eq!(
        decode_a64(0xF9800103).unwrap(),
        Instr::PrfmPLdstPos {
            imm12: 0,
            Rn: 8,
            Rt: 3
        }
    );
    // prfm pldl2strm, [x10]
    assert_eq!(
        decode_a64(0xF9800143).unwrap(),
        Instr::PrfmPLdstPos {
            imm12: 0,
            Rn: 10,
            Rt: 3
        }
    );
    // prfm pstl2strm, [x10]
    assert_eq!(
        decode_a64(0xF9800153).unwrap(),
        Instr::PrfmPLdstPos {
            imm12: 0,
            Rn: 10,
            Rt: 19
        }
    );
    // prfm pstl2strm, [x11]
    assert_eq!(
        decode_a64(0xF9800173).unwrap(),
        Instr::PrfmPLdstPos {
            imm12: 0,
            Rn: 11,
            Rt: 19
        }
    );
    // prfm pstl1keep, [x8, #0x40]
    assert_eq!(
        decode_a64(0xF9802110).unwrap(),
        Instr::PrfmPLdstPos {
            imm12: 8,
            Rn: 8,
            Rt: 16
        }
    );
    // prfm pstl1keep, [x9, #0x40]
    assert_eq!(
        decode_a64(0xF9802130).unwrap(),
        Instr::PrfmPLdstPos {
            imm12: 8,
            Rn: 9,
            Rt: 16
        }
    );
}

#[test]
fn test_ngcs() {
    // ngcs x1, x1
    assert_eq!(
        decode_a64(0xFA0103E1).unwrap(),
        Instr::Sbcs64AddsubCarry {
            Rm: 1,
            Rn: 31,
            Rd: 1
        }
    );
    // ngcs x3, x3
    assert_eq!(
        decode_a64(0xFA0303E3).unwrap(),
        Instr::Sbcs64AddsubCarry {
            Rm: 3,
            Rn: 31,
            Rd: 3
        }
    );
    // ngcs x14, x14
    assert_eq!(
        decode_a64(0xFA0E03EE).unwrap(),
        Instr::Sbcs64AddsubCarry {
            Rm: 14,
            Rn: 31,
            Rd: 14
        }
    );
}

#[test]
fn test_sbcs() {
    // sbcs x1, x8, x3
    assert_eq!(
        decode_a64(0xFA030101).unwrap(),
        Instr::Sbcs64AddsubCarry {
            Rm: 3,
            Rn: 8,
            Rd: 1
        }
    );
    // sbcs x4, x4, x8
    assert_eq!(
        decode_a64(0xFA080084).unwrap(),
        Instr::Sbcs64AddsubCarry {
            Rm: 8,
            Rn: 4,
            Rd: 4
        }
    );
    // sbcs x21, x9, x8
    assert_eq!(
        decode_a64(0xFA080135).unwrap(),
        Instr::Sbcs64AddsubCarry {
            Rm: 8,
            Rn: 9,
            Rd: 21
        }
    );
    // sbcs x7, x7, x15
    assert_eq!(
        decode_a64(0xFA0F00E7).unwrap(),
        Instr::Sbcs64AddsubCarry {
            Rm: 15,
            Rn: 7,
            Rd: 7
        }
    );
    // sbcs x18, x10, xzr
    assert_eq!(
        decode_a64(0xFA1F0152).unwrap(),
        Instr::Sbcs64AddsubCarry {
            Rm: 31,
            Rn: 10,
            Rd: 18
        }
    );
    // sbcs x16, x18, xzr
    assert_eq!(
        decode_a64(0xFA1F0250).unwrap(),
        Instr::Sbcs64AddsubCarry {
            Rm: 31,
            Rn: 18,
            Rd: 16
        }
    );
}

#[test]
fn test_crc32ch() {
    // crc32ch w0, w1, w2
    assert_eq!(
        decode_a64(0x1AC25420).unwrap(),
        Instr::Crc32Ch32CDp2Src {
            Rm: 2,
            Rn: 1,
            Rd: 0
        }
    );
}

#[test]
fn test_asrv() {
    // asrv x0, x1, x3
    assert_eq!(
        decode_a64(0x9AC32820).unwrap(),
        Instr::Asrv64Dp2Src {
            Rm: 3,
            Rn: 1,
            Rd: 0
        }
    );
}

#[test]
fn test_bfm() {
    // bfm x5, x7, #4, #8
    assert_eq!(
        decode_a64(0xB34420E5).unwrap(),
        Instr::Bfm64MBitfield {
            immr: 4,
            imms: 8,
            Rn: 7,
            Rd: 5
        }
    );
}

#[test]
fn test_fminv() {
    // fminv s0, v0.4s
    assert_eq!(
        decode_a64(0x6EB0F800).unwrap(),
        Instr::FminvAsimdallOnlySd { Q: 1, Rn: 0, Rd: 0 }
    );
}

#[test]
fn test_cmlt() {
    // cmlt d0, d0, #0
    assert_eq!(
        decode_a64(0x5EE0A800).unwrap(),
        Instr::CmltAsisdmiscZ { Rn: 0, Rd: 0 }
    );
}

#[test]
fn test_sqxtn2_4s() {
    // sqxtn2.4s v0, v1
    assert_eq!(
        decode_a64(0x4EA14820).unwrap(),
        Instr::SqxtnAsimdmiscN { Q: 1, Rn: 1, Rd: 0 }
    );
}

#[test]
fn test_fcmle() {
    // fcmle d3, d1, #0
    assert_eq!(
        decode_a64(0x7EE0D823).unwrap(),
        Instr::FcmleAsisdmiscFz { Rn: 1, Rd: 3 }
    );
}

#[test]
fn test_shll2_4s() {
    // shll2.4s v0, v1, #16
    assert_eq!(
        decode_a64(0x6E613820).unwrap(),
        Instr::ShllAsimdmiscS { Q: 1, Rn: 1, Rd: 0 }
    );
}

#[test]
fn test_stur() {
    // stur h0, [x0, #-20]
    assert_eq!(
        decode_a64(0x7C1EC000).unwrap(),
        Instr::SturHLdstUnscaled {
            imm9: 492,
            Rn: 0,
            Rt: 0
        }
    );
}

#[test]
fn test_stp() {
    // stp w0, w1, [x2]
    assert_eq!(
        decode_a64(0x29000440).unwrap(),
        Instr::Stp32LdstpairOff {
            imm7: 0,
            Rt2: 1,
            Rn: 2,
            Rt: 0
        }
    );
}

#[test]
fn test_ldtrsb() {
    // ldtrsb x1, [x3, #196]
    assert_eq!(
        decode_a64(0x388C4861).unwrap(),
        Instr::Ldtrsb64LdstUnpriv {
            imm9: 196,
            Rn: 3,
            Rt: 1
        }
    );
}

#[test]
fn test_uhsub() {
    // uhsub v0.8b, v1.8b, v3.8b
    assert_eq!(
        decode_a64(0x2E232420).unwrap(),
        Instr::UhsubAsimdsameOnly {
            Q: 0,
            Rm: 3,
            Rn: 1,
            Rd: 0
        }
    );
}

#[test]
fn test_uabdl2() {
    // uabdl2 v4.2d, v1.4s, v3.4s
    assert_eq!(
        decode_a64(0x6EA37024).unwrap(),
        Instr::UabdlAsimddiffL {
            Q: 1,
            size: 2,
            Rm: 3,
            Rn: 1,
            Rd: 4
        }
    );
}

#[test]
fn test_uminp() {
    // uminp v0.8b, v1.8b, v3.8b
    assert_eq!(
        decode_a64(0x2E23AC20).unwrap(),
        Instr::UminpAsimdsameOnly {
            Q: 0,
            Rm: 3,
            Rn: 1,
            Rd: 0
        }
    );
}

#[test]
fn test_subhn2() {
    // subhn2 v1.16B, v3.8H, v5.8H
    assert_eq!(
        decode_a64(0x4E256061).unwrap(),
        Instr::SubhnAsimddiffN {
            Q: 1,
            size: 0,
            Rm: 5,
            Rn: 3,
            Rd: 1
        }
    );
}

#[test]
fn test_shadd() {
    // shadd v1.8B, v3.8B, v5.8B
    assert_eq!(
        decode_a64(0xE250461).unwrap(),
        Instr::ShaddAsimdsameOnly {
            Q: 0,
            Rm: 5,
            Rn: 3,
            Rd: 1
        }
    );
}
