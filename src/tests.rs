use super::{decode_a32, Instr};

#[test]
fn test() {
    assert_eq!(
        decode_a32(0xe3a00001).unwrap(),
        Instr::MovsIA1 {
            cond: 6,
            S: 0,
            Rn: 0,
            Rd: 0,
            imm12: 1
        }
    );
}
