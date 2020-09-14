use super::{decode_a32, decode_a64, Instr};

#[test]
fn test_mov() {
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
}

//   left: `Subs64AddsubShift { Rm: 0, Rn: 5, Rd: 6 }`,
//   right: `AddvlRRi { Rn: 5, imm6: 0, Rd: 6 }`'

#[test]
fn test_add() {
    // add w6, w5, w0
    // |00001011000|00000|000000|00101|00110|
    // |           | Rm | imm6  |  Rn  | Rd |

    assert_eq!(
        decode_a64(0x0b0000a6).unwrap(),
        Instr::Add32AddsubShift {
            Rm: 0,
            Rn: 5,
            Rd: 6
        }
    );

    // add w18, w18, #0x100
    // |000100010|0000100000000|10010|10010|
    // |         |     imm12   |  Rn |  Rd |
    assert_eq!(
        decode_a64(0x11040252).unwrap(),
        Instr::Add32AddsubImm {
            sh: 0,
            imm12: 256,
            Rn: 18,
            Rd: 18
        }
    );
}

#[test]
fn test_and() {
    // and w2, w8, #1
    //  109876543 2 109876 543210 98765 43210
    // |000100100|0|000000|000000|01000|00010|
    // |         |N| immr | imms | Rn  |  Rd |
    assert_eq!(
        decode_a64(0x12000102).unwrap(),
        Instr::And32LogImm {
            immr: 0,
            imms: 0,
            Rn: 8,
            Rd: 2,
        }
    );
}
