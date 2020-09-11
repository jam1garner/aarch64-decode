#[allow(unused_variables)]
#[allow(non_snake_case)]
pub fn decode_a64(instr: u32) -> Option<Op> {
    match ((instr >> 29) & 5, (instr >> 24) & 9, instr & 47) {
        (_, x1, _) if x1 & 30 == 0 => {
            match ((instr >> 29) & 5, (instr >> 25) & 7, (instr >> 16) & 17, instr & 31) {
                (x0, _, x2, _) if x0 == 0 && x2 == 0 => {
                    let imm16 = instr & 31;
                    match () {
                        () => {
                            Some(Op::aarch64_udf)
                        }
                    }
                }
                (_, _, x2, _) if x2 != 0 => {
                    None
                }
                (x0, _, _, _) if x0 != 0 => {
                    None
                }
                _ => None
            }
        }
        (_, x1, _) if x1 == 3 => {
            None
        }
        (_, x1, _) if x1 & 30 == 4 => {
            match ((instr >> 29) & 5, (instr >> 25) & 7, (instr >> 23) & 3, (instr >> 22) & 1, (instr >> 17) & 9, (instr >> 16) & 1, (instr >> 10) & 11, instr & 19) {
                (x0, _, x2, _, x4, _, x6, _) if x0 == 0 && x2 & 2 == 0 && x4 & 16 == 0 && x6 & 16 == 16 => {
                    match ((instr >> 24) & 15, (instr >> 22) & 3, (instr >> 21) & 1, (instr >> 16) & 9, (instr >> 15) & 1, (instr >> 14) & 1, instr & 27) {
                        (_, _, _, _, x4, _, _) if x4 == 0 => {
                            let size = (instr >> 22) & 3;
                            let Zm = (instr >> 16) & 9;
                            let op = (instr >> 13) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Zda = instr & 9;
                            match op {
                                x0 if x0 == 0 => {
                                    Some(Op::MLA_Z_P_ZZZ__)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::MLS_Z_P_ZZZ__)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, _, x4, _, _) if x4 == 1 => {
                            let size = (instr >> 22) & 3;
                            let Zm = (instr >> 16) & 9;
                            let op = (instr >> 13) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Za = (instr >> 5) & 9;
                            let Zdn = instr & 9;
                            match op {
                                x0 if x0 == 0 => {
                                    Some(Op::MAD_Z_P_ZZZ__)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::MSB_Z_P_ZZZ__)
                                }
                                _ => None
                            }
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 0 && x2 & 2 == 0 && x4 & 16 == 0 && x6 & 56 == 0 => {
                    match ((instr >> 24) & 15, (instr >> 22) & 3, (instr >> 21) & 1, (instr >> 18) & 5, (instr >> 16) & 3, (instr >> 13) & 5, instr & 25) {
                        (_, _, _, x3, _, _, _) if x3 & 6 == 0 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 5;
                            let Pg = (instr >> 10) & 5;
                            let Zm = (instr >> 5) & 9;
                            let Zdn = instr & 9;
                            match opc {
                                x0 if x0 == 0 => {
                                    Some(Op::ADD_Z_P_ZZ__)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::SUB_Z_P_ZZ__)
                                }
                                x0 if x0 == 2 => {
                                    None
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::SUBR_Z_P_ZZ__)
                                }
                                x0 if x0 & 4 == 4 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, _, _) if x3 & 6 == 2 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 17) & 3;
                            let U = (instr >> 16) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Zm = (instr >> 5) & 9;
                            let Zdn = instr & 9;
                            match (opc, U) {
                                (x0, x1) if x0 == 0 && x1 == 0 => {
                                    Some(Op::SMAX_Z_P_ZZ__)
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    Some(Op::UMAX_Z_P_ZZ__)
                                }
                                (x0, x1) if x0 == 1 && x1 == 0 => {
                                    Some(Op::SMIN_Z_P_ZZ__)
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    Some(Op::UMIN_Z_P_ZZ__)
                                }
                                (x0, x1) if x0 == 2 && x1 == 0 => {
                                    Some(Op::SABD_Z_P_ZZ__)
                                }
                                (x0, x1) if x0 == 2 && x1 == 1 => {
                                    Some(Op::UABD_Z_P_ZZ__)
                                }
                                (x0, _) if x0 == 3 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, _, _) if x3 == 4 => {
                            let size = (instr >> 22) & 3;
                            let H = (instr >> 17) & 1;
                            let U = (instr >> 16) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Zm = (instr >> 5) & 9;
                            let Zdn = instr & 9;
                            match (H, U) {
                                (x0, x1) if x0 == 0 && x1 == 0 => {
                                    Some(Op::MUL_Z_P_ZZ__)
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    None
                                }
                                (x0, x1) if x0 == 1 && x1 == 0 => {
                                    Some(Op::SMULH_Z_P_ZZ__)
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    Some(Op::UMULH_Z_P_ZZ__)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, _, _) if x3 == 5 => {
                            let size = (instr >> 22) & 3;
                            let R = (instr >> 17) & 1;
                            let U = (instr >> 16) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Zm = (instr >> 5) & 9;
                            let Zdn = instr & 9;
                            match (R, U) {
                                (x0, x1) if x0 == 0 && x1 == 0 => {
                                    Some(Op::SDIV_Z_P_ZZ__)
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    Some(Op::UDIV_Z_P_ZZ__)
                                }
                                (x0, x1) if x0 == 1 && x1 == 0 => {
                                    Some(Op::SDIVR_Z_P_ZZ__)
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    Some(Op::UDIVR_Z_P_ZZ__)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, _, _) if x3 & 6 == 6 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 5;
                            let Pg = (instr >> 10) & 5;
                            let Zm = (instr >> 5) & 9;
                            let Zdn = instr & 9;
                            match opc {
                                x0 if x0 == 0 => {
                                    Some(Op::ORR_Z_P_ZZ__)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::EOR_Z_P_ZZ__)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::AND_Z_P_ZZ__)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::BIC_Z_P_ZZ__)
                                }
                                x0 if x0 & 4 == 4 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 0 && x2 & 2 == 0 && x4 & 16 == 0 && x6 & 56 == 8 => {
                    match ((instr >> 24) & 15, (instr >> 22) & 3, (instr >> 21) & 1, (instr >> 19) & 3, (instr >> 16) & 5, (instr >> 13) & 5, instr & 25) {
                        (_, _, _, x3, _, _, _) if x3 == 0 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 17) & 3;
                            let U = (instr >> 16) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Vd = instr & 9;
                            match (opc, U) {
                                (x0, x1) if x0 == 0 && x1 == 0 => {
                                    Some(Op::SADDV_R_P_Z__)
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    Some(Op::UADDV_R_P_Z__)
                                }
                                (x0, _) if x0 == 1 => {
                                    None
                                }
                                (x0, _) if x0 & 2 == 2 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, _, _) if x3 == 1 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 17) & 3;
                            let U = (instr >> 16) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Vd = instr & 9;
                            match (opc, U) {
                                (x0, x1) if x0 == 0 && x1 == 0 => {
                                    Some(Op::SMAXV_R_P_Z__)
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    Some(Op::UMAXV_R_P_Z__)
                                }
                                (x0, x1) if x0 == 1 && x1 == 0 => {
                                    Some(Op::SMINV_R_P_Z__)
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    Some(Op::UMINV_R_P_Z__)
                                }
                                (x0, _) if x0 & 2 == 2 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, _, _) if x3 == 2 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 17) & 3;
                            let M = (instr >> 16) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match opc {
                                x0 if x0 == 0 => {
                                    Some(Op::MOVPRFX_Z_P_Z__)
                                }
                                x0 if x0 == 1 => {
                                    None
                                }
                                x0 if x0 & 2 == 2 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, _, _) if x3 == 3 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 5;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Vd = instr & 9;
                            match opc {
                                x0 if x0 == 0 => {
                                    Some(Op::ORV_R_P_Z__)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::EORV_R_P_Z__)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::ANDV_R_P_Z__)
                                }
                                x0 if x0 == 3 => {
                                    None
                                }
                                x0 if x0 & 4 == 4 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 0 && x2 & 2 == 0 && x4 & 16 == 0 && x6 & 56 == 32 => {
                    match ((instr >> 24) & 15, (instr >> 22) & 3, (instr >> 21) & 1, (instr >> 19) & 3, (instr >> 16) & 5, (instr >> 13) & 5, instr & 25) {
                        (_, _, _, x3, _, _, _) if x3 & 2 == 0 => {
                            let tszh = (instr >> 22) & 3;
                            let opc = (instr >> 18) & 3;
                            let L = (instr >> 17) & 1;
                            let U = (instr >> 16) & 1;
                            let Pg = (instr >> 10) & 5;
                            let tszl = (instr >> 8) & 3;
                            let imm3 = (instr >> 5) & 5;
                            let Zdn = instr & 9;
                            match (opc, L, U) {
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                                    Some(Op::ASR_Z_P_ZI__)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                                    Some(Op::LSR_Z_P_ZI__)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    None
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    Some(Op::LSL_Z_P_ZI__)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    Some(Op::ASRD_Z_P_ZI__)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    None
                                }
                                (x0, x1, _) if x0 == 1 && x1 == 1 => {
                                    None
                                }
                                (x0, _, _) if x0 & 2 == 2 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, _, _) if x3 == 2 => {
                            let size = (instr >> 22) & 3;
                            let R = (instr >> 18) & 1;
                            let L = (instr >> 17) & 1;
                            let U = (instr >> 16) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Zm = (instr >> 5) & 9;
                            let Zdn = instr & 9;
                            match (R, L, U) {
                                (_, x1, x2) if x1 == 1 && x2 == 0 => {
                                    None
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                                    Some(Op::ASR_Z_P_ZZ__)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                                    Some(Op::LSR_Z_P_ZZ__)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    Some(Op::LSL_Z_P_ZZ__)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    Some(Op::ASRR_Z_P_ZZ__)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    Some(Op::LSRR_Z_P_ZZ__)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    Some(Op::LSLR_Z_P_ZZ__)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, _, _) if x3 == 3 => {
                            let size = (instr >> 22) & 3;
                            let R = (instr >> 18) & 1;
                            let L = (instr >> 17) & 1;
                            let U = (instr >> 16) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Zm = (instr >> 5) & 9;
                            let Zdn = instr & 9;
                            match (R, L, U) {
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                                    Some(Op::ASR_Z_P_ZW__)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                                    Some(Op::LSR_Z_P_ZW__)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    None
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    Some(Op::LSL_Z_P_ZW__)
                                }
                                (x0, _, _) if x0 == 1 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 0 && x2 & 2 == 0 && x4 & 16 == 0 && x6 & 56 == 40 => {
                    match ((instr >> 24) & 15, (instr >> 22) & 3, (instr >> 21) & 1, (instr >> 19) & 3, (instr >> 16) & 5, (instr >> 13) & 5, instr & 25) {
                        (_, _, _, x3, _, _, _) if x3 & 2 == 0 => {
                            None
                        }
                        (_, _, _, x3, _, _, _) if x3 == 2 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 5;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match opc {
                                x0 if x0 == 0 => {
                                    Some(Op::SXTB_Z_P_Z__)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::UXTB_Z_P_Z__)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::SXTH_Z_P_Z__)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::UXTH_Z_P_Z__)
                                }
                                x0 if x0 == 4 => {
                                    Some(Op::SXTW_Z_P_Z__)
                                }
                                x0 if x0 == 5 => {
                                    Some(Op::UXTW_Z_P_Z__)
                                }
                                x0 if x0 == 6 => {
                                    Some(Op::ABS_Z_P_Z__)
                                }
                                x0 if x0 == 7 => {
                                    Some(Op::NEG_Z_P_Z__)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, _, _) if x3 == 3 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 5;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match opc {
                                x0 if x0 == 0 => {
                                    Some(Op::CLS_Z_P_Z__)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::CLZ_Z_P_Z__)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::CNT_Z_P_Z__)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::CNOT_Z_P_Z__)
                                }
                                x0 if x0 == 4 => {
                                    Some(Op::FABS_Z_P_Z__)
                                }
                                x0 if x0 == 5 => {
                                    Some(Op::FNEG_Z_P_Z__)
                                }
                                x0 if x0 == 6 => {
                                    Some(Op::NOT_Z_P_Z__)
                                }
                                x0 if x0 == 7 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 0 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 56 == 0 => {
                    let size = (instr >> 22) & 3;
                    let Zm = (instr >> 16) & 9;
                    let opc = (instr >> 10) & 5;
                    let Zn = (instr >> 5) & 9;
                    let Zd = instr & 9;
                    match opc {
                        x0 if x0 == 0 => {
                            Some(Op::ADD_Z_ZZ__)
                        }
                        x0 if x0 == 1 => {
                            Some(Op::SUB_Z_ZZ__)
                        }
                        x0 if x0 & 6 == 2 => {
                            None
                        }
                        x0 if x0 == 4 => {
                            Some(Op::SQADD_Z_ZZ__)
                        }
                        x0 if x0 == 5 => {
                            Some(Op::UQADD_Z_ZZ__)
                        }
                        x0 if x0 == 6 => {
                            Some(Op::SQSUB_Z_ZZ__)
                        }
                        x0 if x0 == 7 => {
                            Some(Op::UQSUB_Z_ZZ__)
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 0 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 56 == 8 => {
                    match ((instr >> 24) & 15, (instr >> 22) & 3, (instr >> 21) & 1, (instr >> 16) & 9, (instr >> 13) & 5, (instr >> 12) & 1, (instr >> 10) & 3, instr & 19) {
                        (_, _, _, _, _, x5, _, _) if x5 == 0 => {
                            None
                        }
                        (_, _, _, _, _, x5, x6, _) if x5 == 1 && x6 == 0 => {
                            let opc = (instr >> 22) & 3;
                            let Zm = (instr >> 16) & 9;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match opc {
                                x0 if x0 == 0 => {
                                    Some(Op::AND_Z_ZZ__)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::ORR_Z_ZZ__)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::EOR_Z_ZZ__)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::BIC_Z_ZZ__)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, _, _, x5, x6, _) if x5 == 1 && x6 != 0 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 0 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 60 == 16 => {
                    match ((instr >> 24) & 15, (instr >> 22) & 3, (instr >> 21) & 1, (instr >> 16) & 9, (instr >> 12) & 7, (instr >> 10) & 3, instr & 19) {
                        (_, _, _, _, _, x5, _) if x5 == 0 => {
                            let size = (instr >> 22) & 3;
                            let imm5b = (instr >> 16) & 9;
                            let imm5 = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match () {
                                () => {
                                    Some(Op::INDEX_Z_II__)
                                }
                            }
                        }
                        (_, _, _, _, _, x5, _) if x5 == 1 => {
                            let size = (instr >> 22) & 3;
                            let imm5 = (instr >> 16) & 9;
                            let Rn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match () {
                                () => {
                                    Some(Op::INDEX_Z_RI__)
                                }
                            }
                        }
                        (_, _, _, _, _, x5, _) if x5 == 2 => {
                            let size = (instr >> 22) & 3;
                            let Rm = (instr >> 16) & 9;
                            let imm5 = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match () {
                                () => {
                                    Some(Op::INDEX_Z_IR__)
                                }
                            }
                        }
                        (_, _, _, _, _, x5, _) if x5 == 3 => {
                            let size = (instr >> 22) & 3;
                            let Rm = (instr >> 16) & 9;
                            let Rn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match () {
                                () => {
                                    Some(Op::INDEX_Z_RR__)
                                }
                            }
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 0 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 60 == 20 => {
                    match ((instr >> 24) & 15, (instr >> 23) & 1, (instr >> 22) & 1, (instr >> 21) & 1, (instr >> 16) & 9, (instr >> 12) & 7, (instr >> 11) & 1, instr & 21) {
                        (_, x1, _, _, _, _, x6, _) if x1 == 0 && x6 == 0 => {
                            let op = (instr >> 22) & 1;
                            let Rn = (instr >> 16) & 9;
                            let imm6 = (instr >> 5) & 11;
                            let Rd = instr & 9;
                            match op {
                                x0 if x0 == 0 => {
                                    Some(Op::ADDVL_R_RI__)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::ADDPL_R_RI__)
                                }
                                _ => None
                            }
                        }
                        (_, x1, _, _, _, _, x6, _) if x1 == 1 && x6 == 0 => {
                            let op = (instr >> 22) & 1;
                            let opc2 = (instr >> 16) & 9;
                            let imm6 = (instr >> 5) & 11;
                            let Rd = instr & 9;
                            match (op, opc2) {
                                (x0, x1) if x0 == 0 && x1 & 16 == 0 => {
                                    None
                                }
                                (x0, x1) if x0 == 0 && x1 & 24 == 16 => {
                                    None
                                }
                                (x0, x1) if x0 == 0 && x1 & 28 == 24 => {
                                    None
                                }
                                (x0, x1) if x0 == 0 && x1 & 30 == 28 => {
                                    None
                                }
                                (x0, x1) if x0 == 0 && x1 == 30 => {
                                    None
                                }
                                (x0, x1) if x0 == 0 && x1 == 31 => {
                                    Some(Op::RDVL_R_I__)
                                }
                                (x0, _) if x0 == 1 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        (_, _, _, _, _, _, x6, _) if x6 == 1 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 0 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 56 == 24 => {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 0 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 56 == 32 => {
                    match ((instr >> 24) & 15, (instr >> 22) & 3, (instr >> 21) & 1, (instr >> 16) & 9, (instr >> 13) & 5, (instr >> 12) & 1, instr & 23) {
                        (_, _, _, _, _, x5, _) if x5 == 0 => {
                            let size = (instr >> 22) & 3;
                            let Zm = (instr >> 16) & 9;
                            let opc = (instr >> 10) & 3;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match opc {
                                x0 if x0 == 0 => {
                                    Some(Op::ASR_Z_ZW__)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::LSR_Z_ZW__)
                                }
                                x0 if x0 == 2 => {
                                    None
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::LSL_Z_ZW__)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, _, _, x5, _) if x5 == 1 => {
                            let tszh = (instr >> 22) & 3;
                            let tszl = (instr >> 19) & 3;
                            let imm3 = (instr >> 16) & 5;
                            let opc = (instr >> 10) & 3;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match opc {
                                x0 if x0 == 0 => {
                                    Some(Op::ASR_Z_ZI__)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::LSR_Z_ZI__)
                                }
                                x0 if x0 == 2 => {
                                    None
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::LSL_Z_ZI__)
                                }
                                _ => None
                            }
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 0 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 60 == 40 => {
                    let opc = (instr >> 22) & 3;
                    let Zm = (instr >> 16) & 9;
                    let msz = (instr >> 10) & 3;
                    let Zn = (instr >> 5) & 9;
                    let Zd = instr & 9;
                    match opc {
                        x0 if x0 == 0 => {
                            Some(Op::ADR_Z_AZ_D_s32_scaled)
                        }
                        x0 if x0 == 1 => {
                            Some(Op::ADR_Z_AZ_D_u32_scaled)
                        }
                        x0 if x0 & 2 == 2 => {
                            Some(Op::ADR_Z_AZ_SD_same_scaled)
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 0 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 60 == 44 => {
                    match ((instr >> 24) & 15, (instr >> 22) & 3, (instr >> 21) & 1, (instr >> 16) & 9, (instr >> 12) & 7, (instr >> 10) & 3, instr & 19) {
                        (_, _, _, _, _, x5, _) if x5 & 2 == 0 => {
                            let size = (instr >> 22) & 3;
                            let Zm = (instr >> 16) & 9;
                            let op = (instr >> 10) & 1;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match op {
                                x0 if x0 == 0 => {
                                    Some(Op::FTSSEL_Z_ZZ__)
                                }
                                x0 if x0 == 1 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        (_, _, _, _, _, x5, _) if x5 == 2 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 9;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match opc {
                                x0 if x0 == 0 => {
                                    Some(Op::FEXPA_Z_Z__)
                                }
                                x0 if x0 == 1 => {
                                    None
                                }
                                x0 if x0 & 30 == 2 => {
                                    None
                                }
                                x0 if x0 & 28 == 4 => {
                                    None
                                }
                                x0 if x0 & 24 == 8 => {
                                    None
                                }
                                x0 if x0 & 16 == 16 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        (_, _, _, _, _, x5, _) if x5 == 3 => {
                            let opc = (instr >> 22) & 3;
                            let opc2 = (instr >> 16) & 9;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match (opc, opc2) {
                                (x0, x1) if x0 == 0 && x1 == 0 => {
                                    Some(Op::MOVPRFX_Z_Z__)
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    None
                                }
                                (x0, x1) if x0 == 0 && x1 & 30 == 2 => {
                                    None
                                }
                                (x0, x1) if x0 == 0 && x1 & 28 == 4 => {
                                    None
                                }
                                (x0, x1) if x0 == 0 && x1 & 24 == 8 => {
                                    None
                                }
                                (x0, x1) if x0 == 0 && x1 & 16 == 16 => {
                                    None
                                }
                                (x0, _) if x0 == 1 => {
                                    None
                                }
                                (x0, _) if x0 & 2 == 2 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 0 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 48 == 48 => {
                    match ((instr >> 24) & 15, (instr >> 22) & 3, (instr >> 21) & 1, (instr >> 20) & 1, (instr >> 16) & 7, (instr >> 14) & 3, (instr >> 11) & 5, instr & 21) {
                        (_, _, _, x3, _, _, x6, _) if x3 == 0 && x6 & 6 == 0 => {
                            let size = (instr >> 22) & 3;
                            let imm4 = (instr >> 16) & 7;
                            let D = (instr >> 11) & 1;
                            let U = (instr >> 10) & 1;
                            let pattern = (instr >> 5) & 9;
                            let Zdn = instr & 9;
                            match (size, D, U) {
                                (x0, _, _) if x0 == 0 => {
                                    None
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    Some(Op::SQINCH_Z_ZS__)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    Some(Op::UQINCH_Z_ZS__)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    Some(Op::SQDECH_Z_ZS__)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    Some(Op::UQDECH_Z_ZS__)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                                    Some(Op::SQINCW_Z_ZS__)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                                    Some(Op::UQINCW_Z_ZS__)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                                    Some(Op::SQDECW_Z_ZS__)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                                    Some(Op::UQDECW_Z_ZS__)
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 0 => {
                                    Some(Op::SQINCD_Z_ZS__)
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 1 => {
                                    Some(Op::UQINCD_Z_ZS__)
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 0 => {
                                    Some(Op::SQDECD_Z_ZS__)
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 1 => {
                                    Some(Op::UQDECD_Z_ZS__)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, _, x6, _) if x3 == 0 && x6 == 4 => {
                            let size = (instr >> 22) & 3;
                            let imm4 = (instr >> 16) & 7;
                            let op = (instr >> 10) & 1;
                            let pattern = (instr >> 5) & 9;
                            let Rd = instr & 9;
                            match (size, op) {
                                (_, x1) if x1 == 1 => {
                                    None
                                }
                                (x0, x1) if x0 == 0 && x1 == 0 => {
                                    Some(Op::CNTB_R_S__)
                                }
                                (x0, x1) if x0 == 1 && x1 == 0 => {
                                    Some(Op::CNTH_R_S__)
                                }
                                (x0, x1) if x0 == 2 && x1 == 0 => {
                                    Some(Op::CNTW_R_S__)
                                }
                                (x0, x1) if x0 == 3 && x1 == 0 => {
                                    Some(Op::CNTD_R_S__)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, _, x6, _) if x3 == 0 && x6 == 5 => {
                            None
                        }
                        (_, _, _, x3, _, _, x6, _) if x3 == 1 && x6 == 0 => {
                            let size = (instr >> 22) & 3;
                            let imm4 = (instr >> 16) & 7;
                            let D = (instr >> 10) & 1;
                            let pattern = (instr >> 5) & 9;
                            let Zdn = instr & 9;
                            match (size, D) {
                                (x0, _) if x0 == 0 => {
                                    None
                                }
                                (x0, x1) if x0 == 1 && x1 == 0 => {
                                    Some(Op::INCH_Z_ZS__)
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    Some(Op::DECH_Z_ZS__)
                                }
                                (x0, x1) if x0 == 2 && x1 == 0 => {
                                    Some(Op::INCW_Z_ZS__)
                                }
                                (x0, x1) if x0 == 2 && x1 == 1 => {
                                    Some(Op::DECW_Z_ZS__)
                                }
                                (x0, x1) if x0 == 3 && x1 == 0 => {
                                    Some(Op::INCD_Z_ZS__)
                                }
                                (x0, x1) if x0 == 3 && x1 == 1 => {
                                    Some(Op::DECD_Z_ZS__)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, _, x6, _) if x3 == 1 && x6 == 4 => {
                            let size = (instr >> 22) & 3;
                            let imm4 = (instr >> 16) & 7;
                            let D = (instr >> 10) & 1;
                            let pattern = (instr >> 5) & 9;
                            let Rdn = instr & 9;
                            match (size, D) {
                                (x0, x1) if x0 == 0 && x1 == 0 => {
                                    Some(Op::INCB_R_RS__)
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    Some(Op::DECB_R_RS__)
                                }
                                (x0, x1) if x0 == 1 && x1 == 0 => {
                                    Some(Op::INCH_R_RS__)
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    Some(Op::DECH_R_RS__)
                                }
                                (x0, x1) if x0 == 2 && x1 == 0 => {
                                    Some(Op::INCW_R_RS__)
                                }
                                (x0, x1) if x0 == 2 && x1 == 1 => {
                                    Some(Op::DECW_R_RS__)
                                }
                                (x0, x1) if x0 == 3 && x1 == 0 => {
                                    Some(Op::INCD_R_RS__)
                                }
                                (x0, x1) if x0 == 3 && x1 == 1 => {
                                    Some(Op::DECD_R_RS__)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, _, x6, _) if x3 == 1 && x6 & 3 == 1 => {
                            None
                        }
                        (_, _, _, _, _, _, x6, _) if x6 & 6 == 2 => {
                            None
                        }
                        (_, _, _, _, _, _, x6, _) if x6 & 6 == 6 => {
                            let size = (instr >> 22) & 3;
                            let sf = (instr >> 20) & 1;
                            let imm4 = (instr >> 16) & 7;
                            let D = (instr >> 11) & 1;
                            let U = (instr >> 10) & 1;
                            let pattern = (instr >> 5) & 9;
                            let Rdn = instr & 9;
                            match (size, sf, D, U) {
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 => {
                                    Some(Op::SQINCB_R_RS_SX)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 => {
                                    Some(Op::UQINCB_R_RS_UW)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => {
                                    Some(Op::SQDECB_R_RS_SX)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 => {
                                    Some(Op::UQDECB_R_RS_UW)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 0 => {
                                    Some(Op::SQINCB_R_RS_X)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 1 => {
                                    Some(Op::UQINCB_R_RS_X)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 0 => {
                                    Some(Op::SQDECB_R_RS_X)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 1 => {
                                    Some(Op::UQDECB_R_RS_X)
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 => {
                                    Some(Op::SQINCH_R_RS_SX)
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 => {
                                    Some(Op::UQINCH_R_RS_UW)
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 => {
                                    Some(Op::SQDECH_R_RS_SX)
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 => {
                                    Some(Op::UQDECH_R_RS_UW)
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 0 => {
                                    Some(Op::SQINCH_R_RS_X)
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 1 => {
                                    Some(Op::UQINCH_R_RS_X)
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 0 => {
                                    Some(Op::SQDECH_R_RS_X)
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 1 => {
                                    Some(Op::UQDECH_R_RS_X)
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 => {
                                    Some(Op::SQINCW_R_RS_SX)
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 => {
                                    Some(Op::UQINCW_R_RS_UW)
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 => {
                                    Some(Op::SQDECW_R_RS_SX)
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 => {
                                    Some(Op::UQDECW_R_RS_UW)
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 1 && x2 == 0 && x3 == 0 => {
                                    Some(Op::SQINCW_R_RS_X)
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 1 && x2 == 0 && x3 == 1 => {
                                    Some(Op::UQINCW_R_RS_X)
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 1 && x2 == 1 && x3 == 0 => {
                                    Some(Op::SQDECW_R_RS_X)
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 1 && x2 == 1 && x3 == 1 => {
                                    Some(Op::UQDECW_R_RS_X)
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 => {
                                    Some(Op::SQINCD_R_RS_SX)
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 => {
                                    Some(Op::UQINCD_R_RS_UW)
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 => {
                                    Some(Op::SQDECD_R_RS_SX)
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 => {
                                    Some(Op::UQDECD_R_RS_UW)
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 1 && x2 == 0 && x3 == 0 => {
                                    Some(Op::SQINCD_R_RS_X)
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 1 && x2 == 0 && x3 == 1 => {
                                    Some(Op::UQINCD_R_RS_X)
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 1 && x2 == 1 && x3 == 0 => {
                                    Some(Op::SQDECD_R_RS_X)
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 1 && x2 == 1 && x3 == 1 => {
                                    Some(Op::UQDECD_R_RS_X)
                                }
                                _ => None
                            }
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, _, _) if x0 == 0 && x2 & 2 == 2 && x4 & 24 == 0 => {
                    match ((instr >> 24) & 15, (instr >> 22) & 3, (instr >> 20) & 3, (instr >> 18) & 3, instr & 35) {
                        (_, x1, _, x3, _) if x1 == 3 && x3 == 0 => {
                            let imm13 = (instr >> 5) & 25;
                            let Zd = instr & 9;
                            match () {
                                () => {
                                    Some(Op::DUPM_Z_I__)
                                }
                            }
                        }
                        (_, x1, _, x3, _) if x1 != 3 && x3 == 0 => {
                            let opc = (instr >> 22) & 3;
                            let imm13 = (instr >> 5) & 25;
                            let Zdn = instr & 9;
                            match opc {
                                x0 if x0 == 0 => {
                                    Some(Op::ORR_Z_ZI__)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::EOR_Z_ZI__)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::AND_Z_ZI__)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _) if x3 != 0 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, _, _) if x0 == 0 && x2 & 2 == 2 && x4 & 24 == 8 => {
                    match ((instr >> 24) & 15, (instr >> 22) & 3, (instr >> 20) & 3, (instr >> 16) & 7, (instr >> 13) & 5, instr & 25) {
                        (_, _, _, _, x4, _) if x4 & 4 == 0 => {
                            let size = (instr >> 22) & 3;
                            let Pg = (instr >> 16) & 7;
                            let M = (instr >> 14) & 1;
                            let sh = (instr >> 13) & 1;
                            let imm8 = (instr >> 5) & 15;
                            let Zd = instr & 9;
                            match M {
                                x0 if x0 == 0 => {
                                    Some(Op::CPY_Z_O_I__)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::CPY_Z_P_I__)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, _, x4, _) if x4 & 6 == 4 => {
                            None
                        }
                        (_, _, _, _, x4, _) if x4 == 6 => {
                            let size = (instr >> 22) & 3;
                            let Pg = (instr >> 16) & 7;
                            let imm8 = (instr >> 5) & 15;
                            let Zd = instr & 9;
                            match () {
                                () => {
                                    Some(Op::FCPY_Z_P_I__)
                                }
                            }
                        }
                        (_, _, _, _, x4, _) if x4 == 7 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 0 && x2 & 2 == 2 && x4 & 16 == 16 && x6 & 56 == 8 => {
                    match ((instr >> 24) & 15, (instr >> 22) & 3, (instr >> 21) & 1, (instr >> 19) & 3, (instr >> 17) & 3, (instr >> 16) & 1, (instr >> 13) & 5, (instr >> 12) & 1, (instr >> 10) & 3, instr & 19) {
                        (_, _, _, x3, x4, x5, _, x7, x8, _) if x3 == 0 && x4 == 0 && x5 == 0 && x7 == 1 && x8 == 2 => {
                            let size = (instr >> 22) & 3;
                            let Rn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match () {
                                () => {
                                    Some(Op::DUP_Z_R__)
                                }
                            }
                        }
                        (_, _, _, x3, x4, x5, _, x7, x8, _) if x3 == 0 && x4 == 2 && x5 == 0 && x7 == 1 && x8 == 2 => {
                            let size = (instr >> 22) & 3;
                            let Rm = (instr >> 5) & 9;
                            let Zdn = instr & 9;
                            match () {
                                () => {
                                    Some(Op::INSR_Z_R__)
                                }
                            }
                        }
                        (_, _, _, x3, x4, x5, _, x7, x8, _) if x3 == 0 && x4 & 1 == 0 && x5 == 0 && x7 == 0 && x8 == 1 => {
                            None
                        }
                        (_, _, _, x3, x4, x5, _, x7, x8, _) if x3 == 0 && x4 & 1 == 0 && x5 == 0 && x7 == 1 && x8 & 1 == 1 => {
                            None
                        }
                        (_, _, _, x3, x4, _, _, x7, x8, _) if x3 == 0 && x4 & 1 == 1 && x7 == 1 && x8 & 2 == 2 => {
                            None
                        }
                        (_, _, _, x3, x4, _, _, _, x8, _) if x3 == 0 && x4 & 1 == 1 && x8 == 1 => {
                            None
                        }
                        (_, _, _, x3, _, x5, _, x7, x8, _) if x3 == 0 && x5 == 1 && x7 == 1 && x8 & 2 == 2 => {
                            None
                        }
                        (_, _, _, x3, _, x5, _, _, x8, _) if x3 == 0 && x5 == 1 && x8 == 1 => {
                            None
                        }
                        (_, _, _, x3, _, _, _, x7, x8, _) if x3 == 0 && x7 == 0 && x8 & 2 == 2 => {
                            None
                        }
                        (_, _, _, x3, _, _, _, _, x8, _) if x3 == 1 && x8 != 0 => {
                            None
                        }
                        (_, _, _, x3, x4, _, _, x7, x8, _) if x3 == 2 && x4 & 2 == 0 && x7 == 0 && x8 == 1 => {
                            None
                        }
                        (_, _, _, x3, x4, _, _, x7, x8, _) if x3 == 2 && x4 & 2 == 0 && x7 == 1 && x8 == 2 => {
                            let size = (instr >> 22) & 3;
                            let U = (instr >> 17) & 1;
                            let H = (instr >> 16) & 1;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match (U, H) {
                                (x0, x1) if x0 == 0 && x1 == 0 => {
                                    Some(Op::SUNPKLO_Z_Z__)
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    Some(Op::SUNPKHI_Z_Z__)
                                }
                                (x0, x1) if x0 == 1 && x1 == 0 => {
                                    Some(Op::UUNPKLO_Z_Z__)
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    Some(Op::UUNPKHI_Z_Z__)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, x4, _, _, x7, x8, _) if x3 == 2 && x4 & 2 == 0 && x7 == 1 && x8 & 1 == 1 => {
                            None
                        }
                        (_, _, _, x3, x4, x5, _, x7, x8, _) if x3 == 2 && x4 == 2 && x5 == 0 && x7 == 0 && x8 == 1 => {
                            None
                        }
                        (_, _, _, x3, x4, x5, _, x7, x8, _) if x3 == 2 && x4 == 2 && x5 == 0 && x7 == 1 && x8 == 2 => {
                            let size = (instr >> 22) & 3;
                            let Vm = (instr >> 5) & 9;
                            let Zdn = instr & 9;
                            match () {
                                () => {
                                    Some(Op::INSR_Z_V__)
                                }
                            }
                        }
                        (_, _, _, x3, x4, x5, _, x7, x8, _) if x3 == 2 && x4 == 2 && x5 == 0 && x7 == 1 && x8 & 1 == 1 => {
                            None
                        }
                        (_, _, _, x3, x4, _, _, x7, x8, _) if x3 == 2 && x4 == 3 && x7 == 1 && x8 & 2 == 2 => {
                            None
                        }
                        (_, _, _, x3, x4, _, _, _, x8, _) if x3 == 2 && x4 == 3 && x8 == 1 => {
                            None
                        }
                        (_, _, _, x3, x4, x5, _, x7, x8, _) if x3 == 2 && x4 & 2 == 2 && x5 == 1 && x7 == 1 && x8 & 2 == 2 => {
                            None
                        }
                        (_, _, _, x3, x4, x5, _, _, x8, _) if x3 == 2 && x4 & 2 == 2 && x5 == 1 && x8 == 1 => {
                            None
                        }
                        (_, _, _, x3, x4, x5, _, x7, x8, _) if x3 == 3 && x4 == 0 && x5 == 0 && x7 == 0 && x8 == 1 => {
                            None
                        }
                        (_, _, _, x3, x4, x5, _, x7, x8, _) if x3 == 3 && x4 == 0 && x5 == 0 && x7 == 1 && x8 == 2 => {
                            let size = (instr >> 22) & 3;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match () {
                                () => {
                                    Some(Op::REV_Z_Z__)
                                }
                            }
                        }
                        (_, _, _, x3, x4, x5, _, x7, x8, _) if x3 == 3 && x4 == 0 && x5 == 0 && x7 == 1 && x8 & 1 == 1 => {
                            None
                        }
                        (_, _, _, x3, x4, x5, _, x7, x8, _) if x3 == 3 && x4 & 2 == 0 && x5 == 1 && x7 == 1 && x8 & 2 == 2 => {
                            None
                        }
                        (_, _, _, x3, x4, x5, _, _, x8, _) if x3 == 3 && x4 & 2 == 0 && x5 == 1 && x8 == 1 => {
                            None
                        }
                        (_, _, _, x3, x4, _, _, x7, x8, _) if x3 == 3 && x4 != 0 && x7 == 1 && x8 & 2 == 2 => {
                            None
                        }
                        (_, _, _, x3, x4, _, _, _, x8, _) if x3 == 3 && x4 != 0 && x8 == 1 => {
                            None
                        }
                        (_, _, _, x3, _, _, _, x7, x8, _) if x3 & 2 == 2 && x7 == 0 && x8 & 2 == 2 => {
                            None
                        }
                        (_, _, _, _, _, _, _, x7, x8, _) if x7 == 0 && x8 == 0 => {
                            let imm2 = (instr >> 22) & 3;
                            let tsz = (instr >> 16) & 9;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match () {
                                () => {
                                    Some(Op::DUP_Z_Zi__)
                                }
                            }
                        }
                        (_, _, _, _, _, _, _, x7, x8, _) if x7 == 1 && x8 == 0 => {
                            let size = (instr >> 22) & 3;
                            let Zm = (instr >> 16) & 9;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match () {
                                () => {
                                    Some(Op::TBL_Z_ZZ_1)
                                }
                            }
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 0 && x2 & 2 == 2 && x4 & 16 == 16 && x6 & 56 == 16 => {
                    match ((instr >> 24) & 15, (instr >> 22) & 3, (instr >> 21) & 1, (instr >> 16) & 9, (instr >> 13) & 5, (instr >> 9) & 7, (instr >> 5) & 7, (instr >> 4) & 1, instr & 7) {
                        (_, x1, _, x3, _, x5, _, x7, _) if x1 == 0 && x3 & 30 == 16 && x5 == 0 && x7 == 0 => {
                            let H = (instr >> 16) & 1;
                            let Pn = (instr >> 5) & 7;
                            let Pd = instr & 7;
                            match H {
                                x0 if x0 == 0 => {
                                    Some(Op::PUNPKLO_P_P__)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::PUNPKHI_P_P__)
                                }
                                _ => None
                            }
                        }
                        (_, x1, _, x3, _, x5, _, x7, _) if x1 == 1 && x3 & 30 == 16 && x5 == 0 && x7 == 0 => {
                            None
                        }
                        (_, x1, _, x3, _, x5, _, x7, _) if x1 == 2 && x3 & 30 == 16 && x5 == 0 && x7 == 0 => {
                            None
                        }
                        (_, x1, _, x3, _, x5, _, x7, _) if x1 == 3 && x3 & 30 == 16 && x5 == 0 && x7 == 0 => {
                            None
                        }
                        (_, _, _, x3, _, x5, _, x7, _) if x3 & 16 == 0 && x5 & 1 == 0 && x7 == 0 => {
                            let size = (instr >> 22) & 3;
                            let Pm = (instr >> 16) & 7;
                            let opc = (instr >> 11) & 3;
                            let H = (instr >> 10) & 1;
                            let Pn = (instr >> 5) & 7;
                            let Pd = instr & 7;
                            match (opc, H) {
                                (x0, x1) if x0 == 0 && x1 == 0 => {
                                    Some(Op::ZIP1_P_PP__)
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    Some(Op::ZIP2_P_PP__)
                                }
                                (x0, x1) if x0 == 1 && x1 == 0 => {
                                    Some(Op::UZP1_P_PP__)
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    Some(Op::UZP2_P_PP__)
                                }
                                (x0, x1) if x0 == 2 && x1 == 0 => {
                                    Some(Op::TRN1_P_PP__)
                                }
                                (x0, x1) if x0 == 2 && x1 == 1 => {
                                    Some(Op::TRN2_P_PP__)
                                }
                                (x0, _) if x0 == 3 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, x5, _, x7, _) if x3 & 16 == 0 && x5 & 1 == 1 && x7 == 0 => {
                            None
                        }
                        (_, _, _, x3, _, x5, _, x7, _) if x3 == 20 && x5 == 0 && x7 == 0 => {
                            let size = (instr >> 22) & 3;
                            let Pn = (instr >> 5) & 7;
                            let Pd = instr & 7;
                            match () {
                                () => {
                                    Some(Op::REV_P_P__)
                                }
                            }
                        }
                        (_, _, _, x3, _, x5, _, x7, _) if x3 == 21 && x5 == 0 && x7 == 0 => {
                            None
                        }
                        (_, _, _, x3, _, x5, _, x7, _) if x3 & 26 == 16 && x5 == 8 && x7 == 0 => {
                            None
                        }
                        (_, _, _, x3, _, x5, _, x7, _) if x3 & 26 == 16 && x5 & 7 == 4 && x7 == 0 => {
                            None
                        }
                        (_, _, _, x3, _, x5, _, x7, _) if x3 & 26 == 16 && x5 & 3 == 2 && x7 == 0 => {
                            None
                        }
                        (_, _, _, x3, _, x5, _, x7, _) if x3 & 26 == 16 && x5 & 1 == 1 && x7 == 0 => {
                            None
                        }
                        (_, _, _, x3, _, _, _, x7, _) if x3 & 26 == 18 && x7 == 0 => {
                            None
                        }
                        (_, _, _, x3, _, _, _, x7, _) if x3 & 24 == 24 && x7 == 0 => {
                            None
                        }
                        (_, _, _, _, _, _, _, x7, _) if x7 == 1 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 0 && x2 & 2 == 2 && x4 & 16 == 16 && x6 & 56 == 24 => {
                    let size = (instr >> 22) & 3;
                    let Zm = (instr >> 16) & 9;
                    let opc = (instr >> 10) & 5;
                    let Zn = (instr >> 5) & 9;
                    let Zd = instr & 9;
                    match opc {
                        x0 if x0 == 0 => {
                            Some(Op::ZIP1_Z_ZZ__)
                        }
                        x0 if x0 == 1 => {
                            Some(Op::ZIP2_Z_ZZ__)
                        }
                        x0 if x0 == 2 => {
                            Some(Op::UZP1_Z_ZZ__)
                        }
                        x0 if x0 == 3 => {
                            Some(Op::UZP2_Z_ZZ__)
                        }
                        x0 if x0 == 4 => {
                            Some(Op::TRN1_Z_ZZ__)
                        }
                        x0 if x0 == 5 => {
                            Some(Op::TRN2_Z_ZZ__)
                        }
                        x0 if x0 & 6 == 6 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 0 && x2 & 2 == 2 && x4 & 16 == 16 && x6 & 48 == 32 => {
                    match ((instr >> 24) & 15, (instr >> 22) & 3, (instr >> 21) & 1, (instr >> 20) & 1, (instr >> 17) & 5, (instr >> 16) & 1, (instr >> 14) & 3, (instr >> 13) & 1, instr & 25) {
                        (_, _, _, x3, x4, x5, _, x7, _) if x3 == 0 && x4 == 0 && x5 == 0 && x7 == 0 => {
                            let size = (instr >> 22) & 3;
                            let Pg = (instr >> 10) & 5;
                            let Vn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match () {
                                () => {
                                    Some(Op::CPY_Z_P_V__)
                                }
                            }
                        }
                        (_, _, _, x3, x4, x5, _, x7, _) if x3 == 0 && x4 == 0 && x5 == 1 && x7 == 0 => {
                            let size = (instr >> 22) & 3;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match () {
                                () => {
                                    Some(Op::COMPACT_Z_P_Z__)
                                }
                            }
                        }
                        (_, _, _, x3, x4, _, _, x7, _) if x3 == 0 && x4 == 0 && x7 == 1 => {
                            let size = (instr >> 22) & 3;
                            let B = (instr >> 16) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Rd = instr & 9;
                            match B {
                                x0 if x0 == 0 => {
                                    Some(Op::LASTA_R_P_Z__)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::LASTB_R_P_Z__)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, x4, _, _, x7, _) if x3 == 0 && x4 == 1 && x7 == 0 => {
                            let size = (instr >> 22) & 3;
                            let B = (instr >> 16) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Vd = instr & 9;
                            match B {
                                x0 if x0 == 0 => {
                                    Some(Op::LASTA_V_P_Z__)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::LASTB_V_P_Z__)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, x4, _, _, x7, _) if x3 == 0 && x4 & 6 == 2 && x7 == 0 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 3;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match opc {
                                x0 if x0 == 0 => {
                                    Some(Op::REVB_Z_Z__)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::REVH_Z_Z__)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::REVW_Z_Z__)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::RBIT_Z_P_Z__)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, x4, _, _, x7, _) if x3 == 0 && x4 & 6 == 2 && x7 == 1 => {
                            None
                        }
                        (_, _, _, x3, x4, x5, _, x7, _) if x3 == 0 && x4 == 4 && x5 == 0 && x7 == 1 => {
                            let size = (instr >> 22) & 3;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match () {
                                () => {
                                    Some(Op::CPY_Z_P_R__)
                                }
                            }
                        }
                        (_, _, _, x3, x4, x5, _, x7, _) if x3 == 0 && x4 == 4 && x5 == 1 && x7 == 1 => {
                            None
                        }
                        (_, _, _, x3, x4, _, _, x7, _) if x3 == 0 && x4 == 4 && x7 == 0 => {
                            let size = (instr >> 22) & 3;
                            let B = (instr >> 16) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Zm = (instr >> 5) & 9;
                            let Zdn = instr & 9;
                            match B {
                                x0 if x0 == 0 => {
                                    Some(Op::CLASTA_Z_P_ZZ__)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::CLASTB_Z_P_ZZ__)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, x4, _, _, x7, _) if x3 == 0 && x4 == 5 && x7 == 0 => {
                            let size = (instr >> 22) & 3;
                            let B = (instr >> 16) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Zm = (instr >> 5) & 9;
                            let Vdn = instr & 9;
                            match B {
                                x0 if x0 == 0 => {
                                    Some(Op::CLASTA_V_P_Z__)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::CLASTB_V_P_Z__)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, x4, x5, _, x7, _) if x3 == 0 && x4 == 6 && x5 == 0 && x7 == 0 => {
                            let size = (instr >> 22) & 3;
                            let Pg = (instr >> 10) & 5;
                            let Zm = (instr >> 5) & 9;
                            let Zdn = instr & 9;
                            match () {
                                () => {
                                    Some(Op::SPLICE_Z_P_ZZ_Des)
                                }
                            }
                        }
                        (_, _, _, x3, x4, x5, _, x7, _) if x3 == 0 && x4 == 6 && x5 == 0 && x7 == 1 => {
                            None
                        }
                        (_, _, _, x3, x4, x5, _, _, _) if x3 == 0 && x4 == 6 && x5 == 1 => {
                            None
                        }
                        (_, _, _, x3, x4, x5, _, _, _) if x3 == 0 && x4 == 7 && x5 == 0 => {
                            None
                        }
                        (_, _, _, x3, x4, x5, _, _, _) if x3 == 0 && x4 == 7 && x5 == 1 => {
                            None
                        }
                        (_, _, _, x3, x4, _, _, x7, _) if x3 == 0 && x4 & 3 == 1 && x7 == 1 => {
                            None
                        }
                        (_, _, _, x3, x4, _, _, x7, _) if x3 == 1 && x4 == 0 && x7 == 0 => {
                            None
                        }
                        (_, _, _, x3, x4, _, _, x7, _) if x3 == 1 && x4 == 0 && x7 == 1 => {
                            let size = (instr >> 22) & 3;
                            let B = (instr >> 16) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Zm = (instr >> 5) & 9;
                            let Rdn = instr & 9;
                            match B {
                                x0 if x0 == 0 => {
                                    Some(Op::CLASTA_R_P_Z__)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::CLASTB_R_P_Z__)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, x4, _, _, _, _) if x3 == 1 && x4 != 0 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 0 && x2 & 2 == 2 && x4 & 16 == 16 && x6 & 48 == 48 => {
                    let size = (instr >> 22) & 3;
                    let Zm = (instr >> 16) & 9;
                    let Pg = (instr >> 10) & 7;
                    let Zn = (instr >> 5) & 9;
                    let Zd = instr & 9;
                    match () {
                        () => {
                            Some(Op::SEL_Z_P_ZZ__)
                        }
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 0 && x2 == 2 && x4 & 16 == 16 && x6 & 56 == 0 => {
                    match ((instr >> 23) & 17, (instr >> 22) & 1, (instr >> 21) & 1, (instr >> 16) & 9, (instr >> 13) & 5, instr & 25) {
                        (_, x1, _, _, _, _) if x1 == 0 => {
                            let imm8h = (instr >> 16) & 9;
                            let imm8l = (instr >> 10) & 5;
                            let Zm = (instr >> 5) & 9;
                            let Zdn = instr & 9;
                            match () {
                                () => {
                                    Some(Op::EXT_Z_ZI_Des)
                                }
                            }
                        }
                        (_, x1, _, _, _, _) if x1 == 1 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 0 && x2 == 3 && x4 & 16 == 16 && x6 & 56 == 0 => {
                    let op = (instr >> 22) & 1;
                    let Zm = (instr >> 16) & 9;
                    let opc2 = (instr >> 10) & 5;
                    let Zn = (instr >> 5) & 9;
                    let Zd = instr & 9;
                    match (op, opc2) {
                        (x0, x1) if x0 == 0 && x1 == 0 => {
                            Some(Op::ZIP1_Z_ZZ_Q)
                        }
                        (x0, x1) if x0 == 0 && x1 == 1 => {
                            Some(Op::ZIP2_Z_ZZ_Q)
                        }
                        (x0, x1) if x0 == 0 && x1 == 2 => {
                            Some(Op::UZP1_Z_ZZ_Q)
                        }
                        (x0, x1) if x0 == 0 && x1 == 3 => {
                            Some(Op::UZP2_Z_ZZ_Q)
                        }
                        (x0, x1) if x0 == 0 && x1 & 6 == 4 => {
                            None
                        }
                        (x0, x1) if x0 == 0 && x1 == 6 => {
                            Some(Op::TRN1_Z_ZZ_Q)
                        }
                        (x0, x1) if x0 == 0 && x1 == 7 => {
                            Some(Op::TRN2_Z_ZZ_Q)
                        }
                        (x0, _) if x0 == 1 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, _, _) if x0 == 1 && x2 & 2 == 0 && x4 & 16 == 0 => {
                    match ((instr >> 24) & 15, (instr >> 22) & 3, (instr >> 21) & 1, (instr >> 15) & 11, (instr >> 14) & 1, instr & 27) {
                        (_, _, _, _, x4, _) if x4 == 0 => {
                            let size = (instr >> 22) & 3;
                            let Zm = (instr >> 16) & 9;
                            let op = (instr >> 15) & 1;
                            let o2 = (instr >> 13) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let ne = (instr >> 4) & 1;
                            let Pd = instr & 7;
                            match (op, o2, ne) {
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                                    Some(Op::CMPHS_P_P_ZZ__)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                                    Some(Op::CMPHI_P_P_ZZ__)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    Some(Op::CMPEQ_P_P_ZW__)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    Some(Op::CMPNE_P_P_ZW__)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    Some(Op::CMPGE_P_P_ZZ__)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    Some(Op::CMPGT_P_P_ZZ__)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    Some(Op::CMPEQ_P_P_ZZ__)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    Some(Op::CMPNE_P_P_ZZ__)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, _, x4, _) if x4 == 1 => {
                            let size = (instr >> 22) & 3;
                            let Zm = (instr >> 16) & 9;
                            let U = (instr >> 15) & 1;
                            let lt = (instr >> 13) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let ne = (instr >> 4) & 1;
                            let Pd = instr & 7;
                            match (U, lt, ne) {
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                                    Some(Op::CMPGE_P_P_ZW__)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                                    Some(Op::CMPGT_P_P_ZW__)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    Some(Op::CMPLT_P_P_ZW__)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    Some(Op::CMPLE_P_P_ZW__)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    Some(Op::CMPHS_P_P_ZW__)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    Some(Op::CMPHI_P_P_ZW__)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    Some(Op::CMPLO_P_P_ZW__)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    Some(Op::CMPLS_P_P_ZW__)
                                }
                                _ => None
                            }
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, _, _) if x0 == 1 && x2 & 2 == 0 && x4 & 16 == 16 => {
                    let size = (instr >> 22) & 3;
                    let imm7 = (instr >> 14) & 13;
                    let lt = (instr >> 13) & 1;
                    let Pg = (instr >> 10) & 5;
                    let Zn = (instr >> 5) & 9;
                    let ne = (instr >> 4) & 1;
                    let Pd = instr & 7;
                    match (lt, ne) {
                        (x0, x1) if x0 == 0 && x1 == 0 => {
                            Some(Op::CMPHS_P_P_ZI__)
                        }
                        (x0, x1) if x0 == 0 && x1 == 1 => {
                            Some(Op::CMPHI_P_P_ZI__)
                        }
                        (x0, x1) if x0 == 1 && x1 == 0 => {
                            Some(Op::CMPLO_P_P_ZI__)
                        }
                        (x0, x1) if x0 == 1 && x1 == 1 => {
                            Some(Op::CMPLS_P_P_ZI__)
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 1 && x2 & 2 == 2 && x4 & 16 == 0 && x6 & 16 == 0 => {
                    let size = (instr >> 22) & 3;
                    let imm5 = (instr >> 16) & 9;
                    let op = (instr >> 15) & 1;
                    let o2 = (instr >> 13) & 1;
                    let Pg = (instr >> 10) & 5;
                    let Zn = (instr >> 5) & 9;
                    let ne = (instr >> 4) & 1;
                    let Pd = instr & 7;
                    match (op, o2, ne) {
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            Some(Op::CMPGE_P_P_ZI__)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            Some(Op::CMPGT_P_P_ZI__)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            Some(Op::CMPLT_P_P_ZI__)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            Some(Op::CMPLE_P_P_ZI__)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            Some(Op::CMPEQ_P_P_ZI__)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            Some(Op::CMPNE_P_P_ZI__)
                        }
                        (x0, x1, _) if x0 == 1 && x1 == 1 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 1 && x2 & 2 == 2 && x4 & 24 == 0 && x6 & 48 == 16 => {
                    let op = (instr >> 23) & 1;
                    let S = (instr >> 22) & 1;
                    let Pm = (instr >> 16) & 7;
                    let Pg = (instr >> 10) & 7;
                    let o2 = (instr >> 9) & 1;
                    let Pn = (instr >> 5) & 7;
                    let o3 = (instr >> 4) & 1;
                    let Pd = instr & 7;
                    match (op, S, o2, o3) {
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            Some(Op::AND_P_P_PP_Z)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            Some(Op::BIC_P_P_PP_Z)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            Some(Op::EOR_P_P_PP_Z)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 => {
                            Some(Op::SEL_P_P_PP__)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 0 => {
                            Some(Op::ANDS_P_P_PP_Z)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 1 => {
                            Some(Op::BICS_P_P_PP_Z)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 0 => {
                            Some(Op::EORS_P_P_PP_Z)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            Some(Op::ORR_P_P_PP_Z)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            Some(Op::ORN_P_P_PP_Z)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            Some(Op::NOR_P_P_PP_Z)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 => {
                            Some(Op::NAND_P_P_PP_Z)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 0 => {
                            Some(Op::ORRS_P_P_PP_Z)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 1 => {
                            Some(Op::ORNS_P_P_PP_Z)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 0 => {
                            Some(Op::NORS_P_P_PP_Z)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 1 => {
                            Some(Op::NANDS_P_P_PP_Z)
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 1 && x2 & 2 == 2 && x4 & 24 == 0 && x6 & 48 == 48 => {
                    match ((instr >> 24) & 15, (instr >> 22) & 3, (instr >> 20) & 3, (instr >> 16) & 7, (instr >> 14) & 3, (instr >> 10) & 7, (instr >> 9) & 1, instr & 17) {
                        (_, _, _, _, _, _, x6, _) if x6 == 0 => {
                            let op = (instr >> 23) & 1;
                            let S = (instr >> 22) & 1;
                            let Pm = (instr >> 16) & 7;
                            let Pg = (instr >> 10) & 7;
                            let Pn = (instr >> 5) & 7;
                            let B = (instr >> 4) & 1;
                            let Pd = instr & 7;
                            match (op, S, B) {
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                                    Some(Op::BRKPA_P_P_PP__)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                                    Some(Op::BRKPB_P_P_PP__)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    Some(Op::BRKPAS_P_P_PP__)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    Some(Op::BRKPBS_P_P_PP__)
                                }
                                (x0, _, _) if x0 == 1 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        (_, _, _, _, _, _, x6, _) if x6 == 1 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 1 && x2 & 2 == 2 && x4 & 24 == 8 && x6 & 48 == 16 => {
                    match ((instr >> 24) & 15, (instr >> 23) & 1, (instr >> 22) & 1, (instr >> 20) & 3, (instr >> 16) & 7, (instr >> 14) & 3, (instr >> 10) & 7, (instr >> 9) & 1, (instr >> 5) & 7, (instr >> 4) & 1, instr & 7) {
                        (_, x1, _, _, x4, _, _, x7, _, x9, _) if x1 == 0 && x4 == 8 && x7 == 0 && x9 == 0 => {
                            let S = (instr >> 22) & 1;
                            let Pg = (instr >> 10) & 7;
                            let Pn = (instr >> 5) & 7;
                            let Pdm = instr & 7;
                            match S {
                                x0 if x0 == 0 => {
                                    Some(Op::BRKN_P_P_PP__)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::BRKNS_P_P_PP__)
                                }
                                _ => None
                            }
                        }
                        (_, x1, _, _, x4, _, _, x7, _, x9, _) if x1 == 0 && x4 == 8 && x7 == 0 && x9 == 1 => {
                            None
                        }
                        (_, x1, _, _, x4, _, _, x7, _, _, _) if x1 == 0 && x4 & 7 == 0 && x7 == 1 => {
                            None
                        }
                        (_, x1, _, _, x4, _, _, _, _, _, _) if x1 == 0 && x4 & 4 == 4 => {
                            None
                        }
                        (_, x1, _, _, x4, _, _, _, _, _, _) if x1 == 0 && x4 & 2 == 2 => {
                            None
                        }
                        (_, x1, _, _, x4, _, _, _, _, _, _) if x1 == 0 && x4 & 1 == 1 => {
                            None
                        }
                        (_, x1, _, _, x4, _, _, x7, _, _, _) if x1 == 1 && x4 == 0 && x7 == 1 => {
                            None
                        }
                        (_, x1, _, _, x4, _, _, _, _, _, _) if x1 == 1 && x4 != 0 => {
                            None
                        }
                        (_, _, _, _, x4, _, _, x7, _, _, _) if x4 == 0 && x7 == 0 => {
                            let B = (instr >> 23) & 1;
                            let S = (instr >> 22) & 1;
                            let Pg = (instr >> 10) & 7;
                            let Pn = (instr >> 5) & 7;
                            let M = (instr >> 4) & 1;
                            let Pd = instr & 7;
                            match (B, S, M) {
                                (_, x1, x2) if x1 == 1 && x2 == 1 => {
                                    None
                                }
                                (x0, x1, _) if x0 == 0 && x1 == 0 => {
                                    Some(Op::BRKA_P_P_P__)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    Some(Op::BRKAS_P_P_P_Z)
                                }
                                (x0, x1, _) if x0 == 1 && x1 == 0 => {
                                    Some(Op::BRKB_P_P_P__)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    Some(Op::BRKBS_P_P_P_Z)
                                }
                                _ => None
                            }
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 1 && x2 & 2 == 2 && x4 & 24 == 8 && x6 & 48 == 48 => {
                    match ((instr >> 24) & 15, (instr >> 22) & 3, (instr >> 20) & 3, (instr >> 16) & 7, (instr >> 14) & 3, (instr >> 11) & 5, (instr >> 9) & 3, (instr >> 5) & 7, (instr >> 4) & 1, instr & 7) {
                        (_, _, _, x3, _, _, x6, _, x8, _) if x3 == 0 && x6 & 1 == 0 && x8 == 0 => {
                            let op = (instr >> 23) & 1;
                            let S = (instr >> 22) & 1;
                            let Pg = (instr >> 10) & 7;
                            let Pn = (instr >> 5) & 7;
                            let opc2 = instr & 7;
                            match (op, S, opc2) {
                                (x0, x1, _) if x0 == 0 && x1 == 0 => {
                                    None
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    Some(Op::PTEST__P_P__)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    None
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 & 14 == 2 => {
                                    None
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 & 12 == 4 => {
                                    None
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 & 8 == 8 => {
                                    None
                                }
                                (x0, _, _) if x0 == 1 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, _, x6, _, x8, _) if x3 == 4 && x6 & 1 == 0 && x8 == 0 => {
                            None
                        }
                        (_, _, _, x3, _, _, x6, _, x8, _) if x3 & 11 == 2 && x6 & 1 == 0 && x8 == 0 => {
                            None
                        }
                        (_, _, _, x3, _, _, x6, _, x8, _) if x3 & 9 == 1 && x6 & 1 == 0 && x8 == 0 => {
                            None
                        }
                        (_, _, _, x3, _, _, x6, _, x8, _) if x3 & 8 == 0 && x6 & 1 == 1 && x8 == 0 => {
                            None
                        }
                        (_, _, _, x3, _, x5, x6, _, x8, _) if x3 == 8 && x5 == 0 && x6 == 0 && x8 == 0 => {
                            let op = (instr >> 23) & 1;
                            let S = (instr >> 22) & 1;
                            let Pg = (instr >> 5) & 7;
                            let Pdn = instr & 7;
                            match (op, S) {
                                (x0, x1) if x0 == 0 && x1 == 0 => {
                                    None
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    Some(Op::PFIRST_P_P_P__)
                                }
                                (x0, _) if x0 == 1 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, x5, x6, _, x8, _) if x3 == 8 && x5 == 0 && x6 != 0 && x8 == 0 => {
                            None
                        }
                        (_, _, _, x3, _, x5, x6, x7, x8, _) if x3 == 8 && x5 == 4 && x6 == 2 && x7 == 0 && x8 == 0 => {
                            let op = (instr >> 23) & 1;
                            let S = (instr >> 22) & 1;
                            let Pd = instr & 7;
                            match (op, S) {
                                (x0, x1) if x0 == 0 && x1 == 0 => {
                                    Some(Op::PFALSE_P__)
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    None
                                }
                                (x0, _) if x0 == 1 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, x5, x6, x7, x8, _) if x3 == 8 && x5 == 4 && x6 == 2 && x7 != 0 && x8 == 0 => {
                            None
                        }
                        (_, _, _, x3, _, x5, x6, _, x8, _) if x3 == 8 && x5 == 6 && x6 == 0 && x8 == 0 => {
                            let op = (instr >> 23) & 1;
                            let S = (instr >> 22) & 1;
                            let Pg = (instr >> 5) & 7;
                            let Pd = instr & 7;
                            match (op, S) {
                                (x0, x1) if x0 == 0 && x1 == 0 => {
                                    Some(Op::RDFFR_P_P_F__)
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    Some(Op::RDFFRS_P_P_F__)
                                }
                                (x0, _) if x0 == 1 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, x5, x6, _, x8, _) if x3 == 9 && x5 == 0 && x6 & 2 == 0 && x8 == 0 => {
                            None
                        }
                        (_, _, _, x3, _, x5, x6, _, x8, _) if x3 == 9 && x5 == 0 && x6 == 2 && x8 == 0 => {
                            let size = (instr >> 22) & 3;
                            let Pg = (instr >> 5) & 7;
                            let Pdn = instr & 7;
                            match () {
                                () => {
                                    Some(Op::PNEXT_P_P_P__)
                                }
                            }
                        }
                        (_, _, _, x3, _, x5, x6, _, x8, _) if x3 == 9 && x5 == 0 && x6 == 3 && x8 == 0 => {
                            None
                        }
                        (_, _, _, x3, _, x5, x6, _, x8, _) if x3 == 9 && x5 == 4 && x6 == 2 && x8 == 0 => {
                            None
                        }
                        (_, _, _, x3, _, x5, x6, x7, x8, _) if x3 == 9 && x5 == 6 && x6 == 0 && x7 == 0 && x8 == 0 => {
                            let op = (instr >> 23) & 1;
                            let S = (instr >> 22) & 1;
                            let Pd = instr & 7;
                            match (op, S) {
                                (x0, x1) if x0 == 0 && x1 == 0 => {
                                    Some(Op::RDFFR_P_F__)
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    None
                                }
                                (x0, _) if x0 == 1 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, x5, x6, x7, x8, _) if x3 == 9 && x5 == 6 && x6 == 0 && x7 != 0 && x8 == 0 => {
                            None
                        }
                        (_, _, _, x3, _, x5, _, _, x8, _) if x3 & 14 == 8 && x5 == 2 && x8 == 0 => {
                            None
                        }
                        (_, _, _, x3, _, x5, x6, _, x8, _) if x3 & 14 == 8 && x5 == 4 && x6 & 2 == 0 && x8 == 0 => {
                            let size = (instr >> 22) & 3;
                            let S = (instr >> 16) & 1;
                            let pattern = (instr >> 5) & 9;
                            let Pd = instr & 7;
                            match S {
                                x0 if x0 == 0 => {
                                    Some(Op::PTRUE_P_S__)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::PTRUES_P_S__)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, x5, x6, _, x8, _) if x3 & 14 == 8 && x5 == 4 && x6 == 3 && x8 == 0 => {
                            None
                        }
                        (_, _, _, x3, _, x5, x6, _, x8, _) if x3 & 14 == 8 && x5 == 6 && x6 != 0 && x8 == 0 => {
                            None
                        }
                        (_, _, _, x3, _, x5, _, _, x8, _) if x3 & 14 == 8 && x5 & 1 == 1 && x8 == 0 => {
                            None
                        }
                        (_, _, _, x3, _, _, _, _, x8, _) if x3 & 14 == 12 && x8 == 0 => {
                            None
                        }
                        (_, _, _, x3, _, _, _, _, x8, _) if x3 & 10 == 10 && x8 == 0 => {
                            None
                        }
                        (_, _, _, _, _, _, _, _, x8, _) if x8 == 1 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 1 && x2 & 2 == 2 && x4 & 16 == 16 && x6 & 48 == 0 => {
                    match ((instr >> 24) & 15, (instr >> 22) & 3, (instr >> 21) & 1, (instr >> 16) & 9, (instr >> 14) & 3, (instr >> 13) & 1, (instr >> 10) & 5, (instr >> 4) & 11, instr & 7) {
                        (_, _, _, _, _, x5, _, _, _) if x5 == 0 => {
                            let size = (instr >> 22) & 3;
                            let Rm = (instr >> 16) & 9;
                            let sf = (instr >> 12) & 1;
                            let U = (instr >> 11) & 1;
                            let lt = (instr >> 10) & 1;
                            let Rn = (instr >> 5) & 9;
                            let eq = (instr >> 4) & 1;
                            let Pd = instr & 7;
                            match (U, lt, eq) {
                                (_, x1, _) if x1 == 0 => {
                                    None
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    Some(Op::WHILELT_P_P_RR__)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    Some(Op::WHILELE_P_P_RR__)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    Some(Op::WHILELO_P_P_RR__)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    Some(Op::WHILELS_P_P_RR__)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, _, _, x5, x6, _, x8) if x5 == 1 && x6 == 0 && x8 == 0 => {
                            let op = (instr >> 23) & 1;
                            let sz = (instr >> 22) & 1;
                            let Rm = (instr >> 16) & 9;
                            let Rn = (instr >> 5) & 9;
                            let ne = (instr >> 4) & 1;
                            match (op, ne) {
                                (x0, _) if x0 == 0 => {
                                    None
                                }
                                (x0, x1) if x0 == 1 && x1 == 0 => {
                                    Some(Op::CTERMEQ_RR__)
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    Some(Op::CTERMNE_RR__)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, _, _, x5, x6, _, x8) if x5 == 1 && x6 == 0 && x8 != 0 => {
                            None
                        }
                        (_, _, _, _, _, x5, x6, _, _) if x5 == 1 && x6 != 0 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 1 && x2 & 2 == 2 && x4 & 16 == 16 && x6 & 48 == 16 => {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 1 && x2 & 2 == 2 && x4 & 16 == 16 && x6 & 48 == 48 => {
                    match ((instr >> 24) & 15, (instr >> 22) & 3, (instr >> 21) & 1, (instr >> 19) & 3, (instr >> 17) & 3, (instr >> 16) & 1, (instr >> 14) & 3, instr & 27) {
                        (_, _, _, x3, _, _, _, _) if x3 == 0 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 5;
                            let sh = (instr >> 13) & 1;
                            let imm8 = (instr >> 5) & 15;
                            let Zdn = instr & 9;
                            match opc {
                                x0 if x0 == 0 => {
                                    Some(Op::ADD_Z_ZI__)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::SUB_Z_ZI__)
                                }
                                x0 if x0 == 2 => {
                                    None
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::SUBR_Z_ZI__)
                                }
                                x0 if x0 == 4 => {
                                    Some(Op::SQADD_Z_ZI__)
                                }
                                x0 if x0 == 5 => {
                                    Some(Op::UQADD_Z_ZI__)
                                }
                                x0 if x0 == 6 => {
                                    Some(Op::SQSUB_Z_ZI__)
                                }
                                x0 if x0 == 7 => {
                                    Some(Op::UQSUB_Z_ZI__)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, _, _, _) if x3 == 1 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 5;
                            let o2 = (instr >> 13) & 1;
                            let imm8 = (instr >> 5) & 15;
                            let Zdn = instr & 9;
                            match (opc, o2) {
                                (x0, x1) if x0 & 4 == 0 && x1 == 1 => {
                                    None
                                }
                                (x0, x1) if x0 == 0 && x1 == 0 => {
                                    Some(Op::SMAX_Z_ZI__)
                                }
                                (x0, x1) if x0 == 1 && x1 == 0 => {
                                    Some(Op::UMAX_Z_ZI__)
                                }
                                (x0, x1) if x0 == 2 && x1 == 0 => {
                                    Some(Op::SMIN_Z_ZI__)
                                }
                                (x0, x1) if x0 == 3 && x1 == 0 => {
                                    Some(Op::UMIN_Z_ZI__)
                                }
                                (x0, _) if x0 & 4 == 4 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, _, _, _) if x3 == 2 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 5;
                            let o2 = (instr >> 13) & 1;
                            let imm8 = (instr >> 5) & 15;
                            let Zdn = instr & 9;
                            match (opc, o2) {
                                (x0, x1) if x0 == 0 && x1 == 0 => {
                                    Some(Op::MUL_Z_ZI__)
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    None
                                }
                                (x0, _) if x0 == 1 => {
                                    None
                                }
                                (x0, _) if x0 & 6 == 2 => {
                                    None
                                }
                                (x0, _) if x0 & 4 == 4 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, x5, _, _) if x3 == 3 && x5 == 0 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 17) & 3;
                            let sh = (instr >> 13) & 1;
                            let imm8 = (instr >> 5) & 15;
                            let Zd = instr & 9;
                            match opc {
                                x0 if x0 == 0 => {
                                    Some(Op::DUP_Z_I__)
                                }
                                x0 if x0 == 1 => {
                                    None
                                }
                                x0 if x0 & 2 == 2 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, x5, _, _) if x3 == 3 && x5 == 1 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 17) & 3;
                            let o2 = (instr >> 13) & 1;
                            let imm8 = (instr >> 5) & 15;
                            let Zd = instr & 9;
                            match (opc, o2) {
                                (x0, x1) if x0 == 0 && x1 == 0 => {
                                    Some(Op::FDUP_Z_I__)
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    None
                                }
                                (x0, _) if x0 == 1 => {
                                    None
                                }
                                (x0, _) if x0 & 2 == 2 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 1 && x2 & 2 == 2 && x4 & 28 == 16 && x6 & 48 == 32 => {
                    let size = (instr >> 22) & 3;
                    let opc = (instr >> 16) & 5;
                    let Pg = (instr >> 10) & 7;
                    let o2 = (instr >> 9) & 1;
                    let Pn = (instr >> 5) & 7;
                    let Rd = instr & 9;
                    match (opc, o2) {
                        (x0, x1) if x0 == 0 && x1 == 0 => {
                            Some(Op::CNTP_R_P_P__)
                        }
                        (x0, x1) if x0 == 0 && x1 == 1 => {
                            None
                        }
                        (x0, _) if x0 == 1 => {
                            None
                        }
                        (x0, _) if x0 & 6 == 2 => {
                            None
                        }
                        (x0, _) if x0 & 4 == 4 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 1 && x2 & 2 == 2 && x4 & 28 == 20 && x6 & 60 == 32 => {
                    match ((instr >> 24) & 15, (instr >> 22) & 3, (instr >> 19) & 5, (instr >> 18) & 1, (instr >> 16) & 3, (instr >> 12) & 7, (instr >> 11) & 1, instr & 21) {
                        (_, _, _, x3, _, _, x6, _) if x3 == 0 && x6 == 0 => {
                            let size = (instr >> 22) & 3;
                            let D = (instr >> 17) & 1;
                            let U = (instr >> 16) & 1;
                            let opc = (instr >> 9) & 3;
                            let Pm = (instr >> 5) & 7;
                            let Zdn = instr & 9;
                            match (D, U, opc) {
                                (_, _, x2) if x2 == 1 => {
                                    None
                                }
                                (_, _, x2) if x2 & 2 == 2 => {
                                    None
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                                    Some(Op::SQINCP_Z_P_Z__)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    Some(Op::UQINCP_Z_P_Z__)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    Some(Op::SQDECP_Z_P_Z__)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    Some(Op::UQDECP_Z_P_Z__)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, _, x6, _) if x3 == 0 && x6 == 1 => {
                            let size = (instr >> 22) & 3;
                            let D = (instr >> 17) & 1;
                            let U = (instr >> 16) & 1;
                            let sf = (instr >> 10) & 1;
                            let op = (instr >> 9) & 1;
                            let Pm = (instr >> 5) & 7;
                            let Rdn = instr & 9;
                            match (D, U, sf, op) {
                                (_, _, _, x3) if x3 == 1 => {
                                    None
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 => {
                                    Some(Op::SQINCP_R_P_R_SX)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => {
                                    Some(Op::SQINCP_R_P_R_X)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 0 => {
                                    Some(Op::UQINCP_R_P_R_UW)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 0 => {
                                    Some(Op::UQINCP_R_P_R_X)
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 => {
                                    Some(Op::SQDECP_R_P_R_SX)
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 => {
                                    Some(Op::SQDECP_R_P_R_X)
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 0 => {
                                    Some(Op::UQDECP_R_P_R_UW)
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 0 => {
                                    Some(Op::UQDECP_R_P_R_X)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, _, x6, _) if x3 == 1 && x6 == 0 => {
                            let size = (instr >> 22) & 3;
                            let op = (instr >> 17) & 1;
                            let D = (instr >> 16) & 1;
                            let opc2 = (instr >> 9) & 3;
                            let Pm = (instr >> 5) & 7;
                            let Zdn = instr & 9;
                            match (op, D, opc2) {
                                (x0, _, x2) if x0 == 0 && x2 == 1 => {
                                    None
                                }
                                (x0, _, x2) if x0 == 0 && x2 & 2 == 2 => {
                                    None
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                                    Some(Op::INCP_Z_P_Z__)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    Some(Op::DECP_Z_P_Z__)
                                }
                                (x0, _, _) if x0 == 1 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, _, x6, _) if x3 == 1 && x6 == 1 => {
                            let size = (instr >> 22) & 3;
                            let op = (instr >> 17) & 1;
                            let D = (instr >> 16) & 1;
                            let opc2 = (instr >> 9) & 3;
                            let Pm = (instr >> 5) & 7;
                            let Rdn = instr & 9;
                            match (op, D, opc2) {
                                (x0, _, x2) if x0 == 0 && x2 == 1 => {
                                    None
                                }
                                (x0, _, x2) if x0 == 0 && x2 & 2 == 2 => {
                                    None
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                                    Some(Op::INCP_R_P_R__)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    Some(Op::DECP_R_P_R__)
                                }
                                (x0, _, _) if x0 == 1 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 1 && x2 & 2 == 2 && x4 & 28 == 20 && x6 & 60 == 36 => {
                    match ((instr >> 24) & 15, (instr >> 22) & 3, (instr >> 19) & 5, (instr >> 18) & 1, (instr >> 16) & 3, (instr >> 12) & 7, (instr >> 9) & 5, (instr >> 5) & 7, instr & 9) {
                        (_, _, _, x3, x4, _, x6, _, x8) if x3 == 0 && x4 == 0 && x6 == 0 && x8 == 0 => {
                            let opc = (instr >> 22) & 3;
                            let Pn = (instr >> 5) & 7;
                            match opc {
                                x0 if x0 == 0 => {
                                    Some(Op::WRFFR_F_P__)
                                }
                                x0 if x0 == 1 => {
                                    None
                                }
                                x0 if x0 & 2 == 2 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, x4, _, x6, x7, x8) if x3 == 1 && x4 == 0 && x6 == 0 && x7 == 0 && x8 == 0 => {
                            let opc = (instr >> 22) & 3;
                            match opc {
                                x0 if x0 == 0 => {
                                    Some(Op::SETFFR_F__)
                                }
                                x0 if x0 == 1 => {
                                    None
                                }
                                x0 if x0 & 2 == 2 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, x4, _, x6, x7, x8) if x3 == 1 && x4 == 0 && x6 == 0 && x7 & 8 == 8 && x8 == 0 => {
                            None
                        }
                        (_, _, _, x3, x4, _, x6, x7, x8) if x3 == 1 && x4 == 0 && x6 == 0 && x7 & 4 == 4 && x8 == 0 => {
                            None
                        }
                        (_, _, _, x3, x4, _, x6, x7, x8) if x3 == 1 && x4 == 0 && x6 == 0 && x7 & 2 == 2 && x8 == 0 => {
                            None
                        }
                        (_, _, _, x3, x4, _, x6, x7, x8) if x3 == 1 && x4 == 0 && x6 == 0 && x7 & 1 == 1 && x8 == 0 => {
                            None
                        }
                        (_, _, _, _, x4, _, x6, _, x8) if x4 == 0 && x6 == 0 && x8 != 0 => {
                            None
                        }
                        (_, _, _, _, x4, _, x6, _, _) if x4 == 0 && x6 != 0 => {
                            None
                        }
                        (_, _, _, _, x4, _, _, _, _) if x4 != 0 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 1 && x2 & 2 == 2 && x4 & 28 == 20 && x6 & 56 == 40 => {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 1 && x2 & 2 == 2 && x4 & 24 == 24 && x6 & 48 == 32 => {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 2 && x2 & 2 == 0 && x4 & 16 == 0 && x6 & 32 == 0 => {
                    match ((instr >> 24) & 15, (instr >> 22) & 3, (instr >> 21) & 1, (instr >> 16) & 9, (instr >> 15) & 1, (instr >> 14) & 1, (instr >> 11) & 5, (instr >> 10) & 1, instr & 19) {
                        (_, _, _, _, _, x5, x6, _, _) if x5 == 0 && x6 == 0 => {
                            let size = (instr >> 22) & 3;
                            let Zm = (instr >> 16) & 9;
                            let U = (instr >> 10) & 1;
                            let Zn = (instr >> 5) & 9;
                            let Zda = instr & 9;
                            match U {
                                x0 if x0 == 0 => {
                                    Some(Op::SDOT_Z_ZZZ__)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::UDOT_Z_ZZZ__)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, _, _, x5, x6, _, _) if x5 == 0 && x6 != 0 => {
                            None
                        }
                        (_, _, _, _, _, x5, x6, _, _) if x5 == 1 && x6 & 4 == 0 => {
                            None
                        }
                        (_, _, _, _, _, x5, x6, _, _) if x5 == 1 && x6 & 6 == 4 => {
                            None
                        }
                        (_, _, _, _, _, x5, x6, _, _) if x5 == 1 && x6 == 6 => {
                            None
                        }
                        (_, _, _, _, _, x5, x6, x7, _) if x5 == 1 && x6 == 7 && x7 == 0 => {
                            let size = (instr >> 22) & 3;
                            let Zm = (instr >> 16) & 9;
                            let Zn = (instr >> 5) & 9;
                            let Zda = instr & 9;
                            match size {
                                x0 if x0 & 2 == 0 => {
                                    None
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::USDOT_Z_ZZZ_S)
                                }
                                x0 if x0 == 3 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        (_, _, _, _, _, x5, x6, x7, _) if x5 == 1 && x6 == 7 && x7 == 1 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 2 && x2 & 2 == 0 && x4 & 16 == 0 && x6 & 32 == 32 => {
                    None
                }
                (x0, _, x2, _, x4, _, _, _) if x0 == 2 && x2 & 2 == 0 && x4 & 16 == 16 => {
                    match ((instr >> 24) & 15, (instr >> 22) & 3, (instr >> 21) & 1, (instr >> 16) & 9, (instr >> 13) & 5, (instr >> 11) & 3, instr & 21) {
                        (_, _, _, _, x4, x5, _) if x4 == 0 && x5 == 0 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 9;
                            let U = (instr >> 10) & 1;
                            let Zn = (instr >> 5) & 9;
                            let Zda = instr & 9;
                            match (size, U) {
                                (x0, _) if x0 & 2 == 0 => {
                                    None
                                }
                                (x0, x1) if x0 == 2 && x1 == 0 => {
                                    Some(Op::SDOT_Z_ZZZi_S)
                                }
                                (x0, x1) if x0 == 2 && x1 == 1 => {
                                    Some(Op::UDOT_Z_ZZZi_S)
                                }
                                (x0, x1) if x0 == 3 && x1 == 0 => {
                                    Some(Op::SDOT_Z_ZZZi_D)
                                }
                                (x0, x1) if x0 == 3 && x1 == 1 => {
                                    Some(Op::UDOT_Z_ZZZi_D)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, _, x4, x5, _) if x4 == 0 && x5 == 1 => {
                            None
                        }
                        (_, _, _, _, x4, x5, _) if x4 == 0 && x5 == 2 => {
                            None
                        }
                        (_, _, _, _, x4, x5, _) if x4 == 0 && x5 == 3 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 9;
                            let U = (instr >> 10) & 1;
                            let Zn = (instr >> 5) & 9;
                            let Zda = instr & 9;
                            match (size, U) {
                                (x0, _) if x0 & 2 == 0 => {
                                    None
                                }
                                (x0, x1) if x0 == 2 && x1 == 0 => {
                                    Some(Op::USDOT_Z_ZZZi_S)
                                }
                                (x0, x1) if x0 == 2 && x1 == 1 => {
                                    Some(Op::SUDOT_Z_ZZZi_S)
                                }
                                (x0, _) if x0 == 3 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        (_, _, _, _, x4, _, _) if x4 != 0 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 2 && x2 & 2 == 2 && x4 & 16 == 0 && x6 & 32 == 0 => {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 2 && x2 & 2 == 2 && x4 & 16 == 0 && x6 & 48 == 32 => {
                    match ((instr >> 24) & 15, (instr >> 22) & 3, (instr >> 21) & 1, (instr >> 16) & 9, (instr >> 14) & 3, (instr >> 10) & 7, instr & 19) {
                        (_, _, _, _, _, x5, _) if x5 & 12 == 0 => {
                            None
                        }
                        (_, _, _, _, _, x5, _) if x5 & 14 == 4 => {
                            None
                        }
                        (_, _, _, _, _, x5, _) if x5 == 6 => {
                            let uns = (instr >> 22) & 3;
                            let Zm = (instr >> 16) & 9;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match uns {
                                x0 if x0 == 0 => {
                                    Some(Op::SMMLA_Z_ZZZ__)
                                }
                                x0 if x0 == 1 => {
                                    None
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::USMMLA_Z_ZZZ__)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::UMMLA_Z_ZZZ__)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, _, _, x5, _) if x5 == 7 => {
                            None
                        }
                        (_, _, _, _, _, x5, _) if x5 & 8 == 8 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 2 && x2 & 2 == 2 && x4 & 16 == 0 && x6 & 48 == 48 => {
                    None
                }
                (x0, _, x2, _, x4, _, _, _) if x0 == 2 && x2 & 2 == 2 && x4 & 16 == 16 => {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 0 && x6 & 32 == 0 => {
                    let size = (instr >> 22) & 3;
                    let Zm = (instr >> 16) & 9;
                    let rot = (instr >> 13) & 3;
                    let Pg = (instr >> 10) & 5;
                    let Zn = (instr >> 5) & 9;
                    let Zda = instr & 9;
                    match () {
                        () => {
                            Some(Op::FCMLA_Z_P_ZZZ__)
                        }
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 26 == 2 && x6 & 32 == 32 => {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 == 0 && x6 & 56 == 32 => {
                    let size = (instr >> 22) & 3;
                    let rot = (instr >> 16) & 1;
                    let Pg = (instr >> 10) & 5;
                    let Zm = (instr >> 5) & 9;
                    let Zdn = instr & 9;
                    match () {
                        () => {
                            Some(Op::FCADD_Z_P_ZZ__)
                        }
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 == 0 && x6 & 56 == 40 => {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 == 0 && x6 & 48 == 48 => {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 == 1 && x6 & 32 == 32 => {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 30 == 4 && x6 & 56 == 32 => {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 30 == 4 && x6 & 56 == 40 => {
                    let opc = (instr >> 22) & 3;
                    let opc2 = (instr >> 16) & 3;
                    let Pg = (instr >> 10) & 5;
                    let Zn = (instr >> 5) & 9;
                    let Zd = instr & 9;
                    match (opc, opc2) {
                        (x0, _) if x0 & 2 == 0 => {
                            None
                        }
                        (x0, x1) if x0 == 2 && x1 & 2 == 0 => {
                            None
                        }
                        (x0, x1) if x0 == 2 && x1 == 2 => {
                            Some(Op::BFCVTNT_Z_P_Z_S2BF)
                        }
                        (x0, x1) if x0 == 2 && x1 == 3 => {
                            None
                        }
                        (x0, _) if x0 == 3 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 30 == 4 && x6 & 48 == 48 => {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 24 == 8 && x6 & 32 == 32 => {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 22 == 2 => {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 62 == 0 => {
                    let size = (instr >> 22) & 3;
                    let opc = (instr >> 16) & 9;
                    let op = (instr >> 10) & 1;
                    let Zn = (instr >> 5) & 9;
                    let Zda = instr & 9;
                    match (size, op) {
                        (x0, x1) if x0 & 2 == 0 && x1 == 0 => {
                            Some(Op::FMLA_Z_ZZZi_H)
                        }
                        (x0, x1) if x0 & 2 == 0 && x1 == 1 => {
                            Some(Op::FMLS_Z_ZZZi_H)
                        }
                        (x0, x1) if x0 == 2 && x1 == 0 => {
                            Some(Op::FMLA_Z_ZZZi_S)
                        }
                        (x0, x1) if x0 == 2 && x1 == 1 => {
                            Some(Op::FMLS_Z_ZZZi_S)
                        }
                        (x0, x1) if x0 == 3 && x1 == 0 => {
                            Some(Op::FMLA_Z_ZZZi_D)
                        }
                        (x0, x1) if x0 == 3 && x1 == 1 => {
                            Some(Op::FMLS_Z_ZZZi_D)
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 60 == 4 => {
                    let size = (instr >> 22) & 3;
                    let opc = (instr >> 16) & 9;
                    let rot = (instr >> 10) & 3;
                    let Zn = (instr >> 5) & 9;
                    let Zda = instr & 9;
                    match size {
                        x0 if x0 & 2 == 0 => {
                            None
                        }
                        x0 if x0 == 2 => {
                            Some(Op::FCMLA_Z_ZZZi_H)
                        }
                        x0 if x0 == 3 => {
                            Some(Op::FCMLA_Z_ZZZi_S)
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 == 8 => {
                    let size = (instr >> 22) & 3;
                    let opc = (instr >> 16) & 9;
                    let Zn = (instr >> 5) & 9;
                    let Zd = instr & 9;
                    match size {
                        x0 if x0 & 2 == 0 => {
                            Some(Op::FMUL_Z_ZZi_H)
                        }
                        x0 if x0 == 2 => {
                            Some(Op::FMUL_Z_ZZi_S)
                        }
                        x0 if x0 == 3 => {
                            Some(Op::FMUL_Z_ZZi_D)
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 == 9 => {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 60 == 12 => {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 52 == 16 => {
                    match ((instr >> 24) & 15, (instr >> 23) & 1, (instr >> 22) & 1, (instr >> 21) & 1, (instr >> 16) & 9, (instr >> 14) & 3, (instr >> 13) & 1, (instr >> 12) & 1, (instr >> 10) & 3, instr & 19) {
                        (_, x1, _, _, _, _, x6, _, x8, _) if x1 == 0 && x6 == 0 && x8 == 0 => {
                            let op = (instr >> 22) & 1;
                            let i2 = (instr >> 19) & 3;
                            let Zm = (instr >> 16) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Zda = instr & 9;
                            match op {
                                x0 if x0 == 0 => {
                                    None
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::BFDOT_Z_ZZZi__)
                                }
                                _ => None
                            }
                        }
                        (_, x1, _, _, _, _, x6, _, x8, _) if x1 == 0 && x6 == 0 && x8 != 0 => {
                            None
                        }
                        (_, x1, _, _, _, _, x6, _, _, _) if x1 == 0 && x6 == 1 => {
                            None
                        }
                        (_, x1, _, _, _, _, _, _, _, _) if x1 == 1 => {
                            let o2 = (instr >> 22) & 1;
                            let i3h = (instr >> 19) & 3;
                            let Zm = (instr >> 16) & 5;
                            let op = (instr >> 13) & 1;
                            let i3l = (instr >> 11) & 1;
                            let T = (instr >> 10) & 1;
                            let Zn = (instr >> 5) & 9;
                            let Zda = instr & 9;
                            match (o2, op, T) {
                                (x0, _, _) if x0 == 0 => {
                                    None
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    Some(Op::BFMLALB_Z_ZZZi__)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    Some(Op::BFMLALT_Z_ZZZi__)
                                }
                                (x0, x1, _) if x0 == 1 && x1 == 1 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 52 == 20 => {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 54 == 32 => {
                    match ((instr >> 24) & 15, (instr >> 23) & 1, (instr >> 22) & 1, (instr >> 21) & 1, (instr >> 16) & 9, (instr >> 14) & 3, (instr >> 13) & 1, (instr >> 11) & 3, (instr >> 10) & 1, instr & 19) {
                        (_, x1, _, _, _, _, x6, _, x8, _) if x1 == 0 && x6 == 0 && x8 == 0 => {
                            let op = (instr >> 22) & 1;
                            let Zm = (instr >> 16) & 9;
                            let Zn = (instr >> 5) & 9;
                            let Zda = instr & 9;
                            match op {
                                x0 if x0 == 0 => {
                                    None
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::BFDOT_Z_ZZZ__)
                                }
                                _ => None
                            }
                        }
                        (_, x1, _, _, _, _, x6, _, x8, _) if x1 == 0 && x6 == 0 && x8 == 1 => {
                            None
                        }
                        (_, x1, _, _, _, _, x6, _, _, _) if x1 == 0 && x6 == 1 => {
                            None
                        }
                        (_, x1, _, _, _, _, _, _, _, _) if x1 == 1 => {
                            let o2 = (instr >> 22) & 1;
                            let Zm = (instr >> 16) & 9;
                            let op = (instr >> 13) & 1;
                            let T = (instr >> 10) & 1;
                            let Zn = (instr >> 5) & 9;
                            let Zda = instr & 9;
                            match (o2, op, T) {
                                (x0, _, _) if x0 == 0 => {
                                    None
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    Some(Op::BFMLALB_Z_ZZZ__)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    Some(Op::BFMLALT_Z_ZZZ__)
                                }
                                (x0, x1, _) if x0 == 1 && x1 == 1 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 52 == 36 => {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 56 == 48 => {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 == 56 => {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 == 57 => {
                    let opc = (instr >> 22) & 3;
                    let Zm = (instr >> 16) & 9;
                    let Zn = (instr >> 5) & 9;
                    let Zda = instr & 9;
                    match opc {
                        x0 if x0 == 0 => {
                            None
                        }
                        x0 if x0 == 1 => {
                            Some(Op::BFMMLA_Z_ZZZ__)
                        }
                        x0 if x0 == 2 => {
                            Some(Op::FMMLA_Z_ZZZ_S)
                        }
                        x0 if x0 == 3 => {
                            Some(Op::FMMLA_Z_ZZZ_D)
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 62 == 58 => {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 60 == 60 => {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 2 && x4 & 16 == 0 && x6 & 16 == 16 => {
                    let size = (instr >> 22) & 3;
                    let Zm = (instr >> 16) & 9;
                    let op = (instr >> 15) & 1;
                    let o2 = (instr >> 13) & 1;
                    let Pg = (instr >> 10) & 5;
                    let Zn = (instr >> 5) & 9;
                    let o3 = (instr >> 4) & 1;
                    let Pd = instr & 7;
                    match (op, o2, o3) {
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            Some(Op::FCMGE_P_P_ZZ__)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            Some(Op::FCMGT_P_P_ZZ__)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            Some(Op::FCMEQ_P_P_ZZ__)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            Some(Op::FCMNE_P_P_ZZ__)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            Some(Op::FCMUO_P_P_ZZ__)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            Some(Op::FACGE_P_P_ZZ__)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            Some(Op::FACGT_P_P_ZZ__)
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 2 && x4 & 16 == 0 && x6 & 56 == 0 => {
                    let size = (instr >> 22) & 3;
                    let Zm = (instr >> 16) & 9;
                    let opc = (instr >> 10) & 5;
                    let Zn = (instr >> 5) & 9;
                    let Zd = instr & 9;
                    match opc {
                        x0 if x0 == 0 => {
                            Some(Op::FADD_Z_ZZ__)
                        }
                        x0 if x0 == 1 => {
                            Some(Op::FSUB_Z_ZZ__)
                        }
                        x0 if x0 == 2 => {
                            Some(Op::FMUL_Z_ZZ__)
                        }
                        x0 if x0 == 3 => {
                            Some(Op::FTSMUL_Z_ZZ__)
                        }
                        x0 if x0 & 6 == 4 => {
                            None
                        }
                        x0 if x0 == 6 => {
                            Some(Op::FRECPS_Z_ZZ__)
                        }
                        x0 if x0 == 7 => {
                            Some(Op::FRSQRTS_Z_ZZ__)
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 2 && x4 & 16 == 0 && x6 & 56 == 32 => {
                    match ((instr >> 24) & 15, (instr >> 22) & 3, (instr >> 21) & 1, (instr >> 19) & 3, (instr >> 16) & 5, (instr >> 13) & 5, (instr >> 10) & 5, (instr >> 6) & 7, instr & 11) {
                        (_, _, _, x3, _, _, _, _, _) if x3 & 2 == 0 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 7;
                            let Pg = (instr >> 10) & 5;
                            let Zm = (instr >> 5) & 9;
                            let Zdn = instr & 9;
                            match opc {
                                x0 if x0 == 0 => {
                                    Some(Op::FADD_Z_P_ZZ__)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::FSUB_Z_P_ZZ__)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::FMUL_Z_P_ZZ__)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::FSUBR_Z_P_ZZ__)
                                }
                                x0 if x0 == 4 => {
                                    Some(Op::FMAXNM_Z_P_ZZ__)
                                }
                                x0 if x0 == 5 => {
                                    Some(Op::FMINNM_Z_P_ZZ__)
                                }
                                x0 if x0 == 6 => {
                                    Some(Op::FMAX_Z_P_ZZ__)
                                }
                                x0 if x0 == 7 => {
                                    Some(Op::FMIN_Z_P_ZZ__)
                                }
                                x0 if x0 == 8 => {
                                    Some(Op::FABD_Z_P_ZZ__)
                                }
                                x0 if x0 == 9 => {
                                    Some(Op::FSCALE_Z_P_ZZ__)
                                }
                                x0 if x0 == 10 => {
                                    Some(Op::FMULX_Z_P_ZZ__)
                                }
                                x0 if x0 == 11 => {
                                    None
                                }
                                x0 if x0 == 12 => {
                                    Some(Op::FDIVR_Z_P_ZZ__)
                                }
                                x0 if x0 == 13 => {
                                    Some(Op::FDIV_Z_P_ZZ__)
                                }
                                x0 if x0 & 14 == 14 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, _, x6, _, _) if x3 == 2 && x6 == 0 => {
                            let size = (instr >> 22) & 3;
                            let imm3 = (instr >> 16) & 5;
                            let Zm = (instr >> 5) & 9;
                            let Zdn = instr & 9;
                            match () {
                                () => {
                                    Some(Op::FTMAD_Z_ZZI__)
                                }
                            }
                        }
                        (_, _, _, x3, _, _, x6, _, _) if x3 == 2 && x6 != 0 => {
                            None
                        }
                        (_, _, _, x3, _, _, _, x7, _) if x3 == 3 && x7 == 0 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 5;
                            let Pg = (instr >> 10) & 5;
                            let i1 = (instr >> 5) & 1;
                            let Zdn = instr & 9;
                            match opc {
                                x0 if x0 == 0 => {
                                    Some(Op::FADD_Z_P_ZS__)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::FSUB_Z_P_ZS__)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::FMUL_Z_P_ZS__)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::FSUBR_Z_P_ZS__)
                                }
                                x0 if x0 == 4 => {
                                    Some(Op::FMAXNM_Z_P_ZS__)
                                }
                                x0 if x0 == 5 => {
                                    Some(Op::FMINNM_Z_P_ZS__)
                                }
                                x0 if x0 == 6 => {
                                    Some(Op::FMAX_Z_P_ZS__)
                                }
                                x0 if x0 == 7 => {
                                    Some(Op::FMIN_Z_P_ZS__)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, _, _, x7, _) if x3 == 3 && x7 != 0 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 2 && x4 & 16 == 0 && x6 & 56 == 40 => {
                    match ((instr >> 24) & 15, (instr >> 22) & 3, (instr >> 21) & 1, (instr >> 18) & 5, (instr >> 16) & 3, (instr >> 13) & 5, instr & 25) {
                        (_, _, _, x3, _, _, _) if x3 & 6 == 0 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 5;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match opc {
                                x0 if x0 == 0 => {
                                    Some(Op::FRINTN_Z_P_Z__)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::FRINTP_Z_P_Z__)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::FRINTM_Z_P_Z__)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::FRINTZ_Z_P_Z__)
                                }
                                x0 if x0 == 4 => {
                                    Some(Op::FRINTA_Z_P_Z__)
                                }
                                x0 if x0 == 5 => {
                                    None
                                }
                                x0 if x0 == 6 => {
                                    Some(Op::FRINTX_Z_P_Z__)
                                }
                                x0 if x0 == 7 => {
                                    Some(Op::FRINTI_Z_P_Z__)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, _, _) if x3 == 2 => {
                            let opc = (instr >> 22) & 3;
                            let opc2 = (instr >> 16) & 3;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match (opc, opc2) {
                                (x0, _) if x0 & 2 == 0 => {
                                    None
                                }
                                (x0, x1) if x0 == 2 && x1 == 0 => {
                                    Some(Op::FCVT_Z_P_Z_S2H)
                                }
                                (x0, x1) if x0 == 2 && x1 == 1 => {
                                    Some(Op::FCVT_Z_P_Z_H2S)
                                }
                                (x0, x1) if x0 == 2 && x1 == 2 => {
                                    Some(Op::BFCVT_Z_P_Z_S2BF)
                                }
                                (x0, x1) if x0 == 2 && x1 == 3 => {
                                    None
                                }
                                (x0, x1) if x0 == 3 && x1 == 0 => {
                                    Some(Op::FCVT_Z_P_Z_D2H)
                                }
                                (x0, x1) if x0 == 3 && x1 == 1 => {
                                    Some(Op::FCVT_Z_P_Z_H2D)
                                }
                                (x0, x1) if x0 == 3 && x1 == 2 => {
                                    Some(Op::FCVT_Z_P_Z_D2S)
                                }
                                (x0, x1) if x0 == 3 && x1 == 3 => {
                                    Some(Op::FCVT_Z_P_Z_S2D)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, _, _) if x3 == 3 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 3;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match opc {
                                x0 if x0 == 0 => {
                                    Some(Op::FRECPX_Z_P_Z__)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::FSQRT_Z_P_Z__)
                                }
                                x0 if x0 & 2 == 2 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, _, _) if x3 & 6 == 4 => {
                            let opc = (instr >> 22) & 3;
                            let opc2 = (instr >> 17) & 3;
                            let U = (instr >> 16) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match (opc, opc2, U) {
                                (x0, _, _) if x0 == 0 => {
                                    None
                                }
                                (x0, x1, _) if x0 == 1 && x1 == 0 => {
                                    None
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    Some(Op::SCVTF_Z_P_Z_H2FP16)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    Some(Op::UCVTF_Z_P_Z_H2FP16)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 == 0 => {
                                    Some(Op::SCVTF_Z_P_Z_W2FP16)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 == 1 => {
                                    Some(Op::UCVTF_Z_P_Z_W2FP16)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 0 => {
                                    Some(Op::SCVTF_Z_P_Z_X2FP16)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 1 => {
                                    Some(Op::UCVTF_Z_P_Z_X2FP16)
                                }
                                (x0, x1, _) if x0 == 2 && x1 & 2 == 0 => {
                                    None
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 2 && x2 == 0 => {
                                    Some(Op::SCVTF_Z_P_Z_W2S)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 2 && x2 == 1 => {
                                    Some(Op::UCVTF_Z_P_Z_W2S)
                                }
                                (x0, x1, _) if x0 == 2 && x1 == 3 => {
                                    None
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 0 => {
                                    Some(Op::SCVTF_Z_P_Z_W2D)
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 1 => {
                                    Some(Op::UCVTF_Z_P_Z_W2D)
                                }
                                (x0, x1, _) if x0 == 3 && x1 == 1 => {
                                    None
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 2 && x2 == 0 => {
                                    Some(Op::SCVTF_Z_P_Z_X2S)
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 2 && x2 == 1 => {
                                    Some(Op::UCVTF_Z_P_Z_X2S)
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 3 && x2 == 0 => {
                                    Some(Op::SCVTF_Z_P_Z_X2D)
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 3 && x2 == 1 => {
                                    Some(Op::UCVTF_Z_P_Z_X2D)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, _, _) if x3 & 6 == 6 => {
                            let opc = (instr >> 22) & 3;
                            let opc2 = (instr >> 17) & 3;
                            let U = (instr >> 16) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match (opc, opc2, U) {
                                (x0, _, _) if x0 == 0 => {
                                    None
                                }
                                (x0, x1, _) if x0 == 1 && x1 == 0 => {
                                    None
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    Some(Op::FCVTZS_Z_P_Z_FP162H)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    Some(Op::FCVTZU_Z_P_Z_FP162H)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 == 0 => {
                                    Some(Op::FCVTZS_Z_P_Z_FP162W)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 == 1 => {
                                    Some(Op::FCVTZU_Z_P_Z_FP162W)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 0 => {
                                    Some(Op::FCVTZS_Z_P_Z_FP162X)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 1 => {
                                    Some(Op::FCVTZU_Z_P_Z_FP162X)
                                }
                                (x0, x1, _) if x0 == 2 && x1 & 2 == 0 => {
                                    None
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 2 && x2 == 0 => {
                                    Some(Op::FCVTZS_Z_P_Z_S2W)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 2 && x2 == 1 => {
                                    Some(Op::FCVTZU_Z_P_Z_S2W)
                                }
                                (x0, x1, _) if x0 == 2 && x1 == 3 => {
                                    None
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 0 => {
                                    Some(Op::FCVTZS_Z_P_Z_D2W)
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 1 => {
                                    Some(Op::FCVTZU_Z_P_Z_D2W)
                                }
                                (x0, x1, _) if x0 == 3 && x1 == 1 => {
                                    None
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 2 && x2 == 0 => {
                                    Some(Op::FCVTZS_Z_P_Z_S2X)
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 2 && x2 == 1 => {
                                    Some(Op::FCVTZU_Z_P_Z_S2X)
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 3 && x2 == 0 => {
                                    Some(Op::FCVTZS_Z_P_Z_D2X)
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 3 && x2 == 1 => {
                                    Some(Op::FCVTZU_Z_P_Z_D2X)
                                }
                                _ => None
                            }
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 2 && x4 & 28 == 0 && x6 & 56 == 8 => {
                    let size = (instr >> 22) & 3;
                    let opc = (instr >> 16) & 5;
                    let Pg = (instr >> 10) & 5;
                    let Zn = (instr >> 5) & 9;
                    let Vd = instr & 9;
                    match opc {
                        x0 if x0 == 0 => {
                            Some(Op::FADDV_V_P_Z__)
                        }
                        x0 if x0 == 1 => {
                            None
                        }
                        x0 if x0 & 6 == 2 => {
                            None
                        }
                        x0 if x0 == 4 => {
                            Some(Op::FMAXNMV_V_P_Z__)
                        }
                        x0 if x0 == 5 => {
                            Some(Op::FMINNMV_V_P_Z__)
                        }
                        x0 if x0 == 6 => {
                            Some(Op::FMAXV_V_P_Z__)
                        }
                        x0 if x0 == 7 => {
                            Some(Op::FMINV_V_P_Z__)
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 2 && x4 & 28 == 4 && x6 & 60 == 8 => {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 2 && x4 & 28 == 4 && x6 & 60 == 12 => {
                    match ((instr >> 24) & 15, (instr >> 22) & 3, (instr >> 19) & 5, (instr >> 16) & 5, (instr >> 12) & 7, (instr >> 10) & 3, instr & 19) {
                        (_, _, _, _, _, x5, _) if x5 == 0 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match opc {
                                x0 if x0 & 4 == 0 => {
                                    None
                                }
                                x0 if x0 & 6 == 4 => {
                                    None
                                }
                                x0 if x0 == 6 => {
                                    Some(Op::FRECPE_Z_Z__)
                                }
                                x0 if x0 == 7 => {
                                    Some(Op::FRSQRTE_Z_Z__)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, _, _, x5, _) if x5 != 0 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 2 && x4 & 28 == 8 && x6 & 56 == 8 => {
                    match ((instr >> 24) & 15, (instr >> 22) & 3, (instr >> 19) & 5, (instr >> 18) & 1, (instr >> 16) & 3, (instr >> 13) & 5, instr & 25) {
                        (_, _, _, x3, _, _, _) if x3 == 0 => {
                            let size = (instr >> 22) & 3;
                            let eq = (instr >> 17) & 1;
                            let lt = (instr >> 16) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let ne = (instr >> 4) & 1;
                            let Pd = instr & 7;
                            match (eq, lt, ne) {
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                                    Some(Op::FCMGE_P_P_Z0__)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                                    Some(Op::FCMGT_P_P_Z0__)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    Some(Op::FCMLT_P_P_Z0__)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    Some(Op::FCMLE_P_P_Z0__)
                                }
                                (x0, _, x2) if x0 == 1 && x2 == 1 => {
                                    None
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    Some(Op::FCMEQ_P_P_Z0__)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    Some(Op::FCMNE_P_P_Z0__)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, _, _) if x3 == 1 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 2 && x4 & 28 == 12 && x6 & 56 == 8 => {
                    let size = (instr >> 22) & 3;
                    let opc = (instr >> 16) & 5;
                    let Pg = (instr >> 10) & 5;
                    let Zm = (instr >> 5) & 9;
                    let Vdn = instr & 9;
                    match opc {
                        x0 if x0 == 0 => {
                            Some(Op::FADDA_V_P_Z__)
                        }
                        x0 if x0 == 1 => {
                            None
                        }
                        x0 if x0 & 6 == 2 => {
                            None
                        }
                        x0 if x0 & 4 == 4 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, _, _) if x0 == 3 && x2 & 2 == 2 && x4 & 16 == 16 => {
                    match ((instr >> 24) & 15, (instr >> 22) & 3, (instr >> 21) & 1, (instr >> 16) & 9, (instr >> 15) & 1, instr & 29) {
                        (_, _, _, _, x4, _) if x4 == 0 => {
                            let size = (instr >> 22) & 3;
                            let Zm = (instr >> 16) & 9;
                            let opc = (instr >> 13) & 3;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Zda = instr & 9;
                            match opc {
                                x0 if x0 == 0 => {
                                    Some(Op::FMLA_Z_P_ZZZ__)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::FMLS_Z_P_ZZZ__)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::FNMLA_Z_P_ZZZ__)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::FNMLS_Z_P_ZZZ__)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, _, x4, _) if x4 == 1 => {
                            let size = (instr >> 22) & 3;
                            let Za = (instr >> 16) & 9;
                            let opc = (instr >> 13) & 3;
                            let Pg = (instr >> 10) & 5;
                            let Zm = (instr >> 5) & 9;
                            let Zdn = instr & 9;
                            match opc {
                                x0 if x0 == 0 => {
                                    Some(Op::FMAD_Z_P_ZZZ__)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::FMSB_Z_P_ZZZ__)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::FNMAD_Z_P_ZZZ__)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::FNMSB_Z_P_ZZZ__)
                                }
                                _ => None
                            }
                        }
                        _ => None
                    }
                }
                (x0, _, _, _, _, _, _, _) if x0 == 4 => {
                    match ((instr >> 25) & 13, (instr >> 23) & 3, (instr >> 21) & 3, (instr >> 16) & 9, (instr >> 13) & 5, (instr >> 5) & 15, (instr >> 4) & 1, instr & 7) {
                        (_, x1, x2, _, x4, _, x6, _) if x1 == 0 && x2 & 1 == 1 && x4 & 4 == 0 && x6 == 0 => {
                            let xs = (instr >> 22) & 1;
                            let Zm = (instr >> 16) & 9;
                            let msz = (instr >> 13) & 3;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let prfop = instr & 7;
                            match msz {
                                x0 if x0 == 0 => {
                                    Some(Op::PRFB_I_P_BZ_S_x32_scaled)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::PRFH_I_P_BZ_S_x32_scaled)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::PRFW_I_P_BZ_S_x32_scaled)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::PRFD_I_P_BZ_S_x32_scaled)
                                }
                                _ => None
                            }
                        }
                        (_, x1, x2, _, x4, _, x6, _) if x1 == 0 && x2 & 1 == 1 && x4 & 4 == 0 && x6 == 1 => {
                            None
                        }
                        (_, x1, x2, _, x4, _, _, _) if x1 == 1 && x2 & 1 == 1 && x4 & 4 == 0 => {
                            let xs = (instr >> 22) & 1;
                            let Zm = (instr >> 16) & 9;
                            let U = (instr >> 14) & 1;
                            let ff = (instr >> 13) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match (U, ff) {
                                (x0, x1) if x0 == 0 && x1 == 0 => {
                                    Some(Op::LD1SH_Z_P_BZ_S_x32_scaled)
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    Some(Op::LDFF1SH_Z_P_BZ_S_x32_scaled)
                                }
                                (x0, x1) if x0 == 1 && x1 == 0 => {
                                    Some(Op::LD1H_Z_P_BZ_S_x32_scaled)
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    Some(Op::LDFF1H_Z_P_BZ_S_x32_scaled)
                                }
                                _ => None
                            }
                        }
                        (_, x1, x2, _, x4, _, _, _) if x1 == 2 && x2 & 1 == 1 && x4 & 4 == 0 => {
                            let xs = (instr >> 22) & 1;
                            let Zm = (instr >> 16) & 9;
                            let U = (instr >> 14) & 1;
                            let ff = (instr >> 13) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match (U, ff) {
                                (x0, _) if x0 == 0 => {
                                    None
                                }
                                (x0, x1) if x0 == 1 && x1 == 0 => {
                                    Some(Op::LD1W_Z_P_BZ_S_x32_scaled)
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    Some(Op::LDFF1W_Z_P_BZ_S_x32_scaled)
                                }
                                _ => None
                            }
                        }
                        (_, x1, x2, _, x4, _, x6, _) if x1 == 3 && x2 & 2 == 0 && x4 == 0 && x6 == 0 => {
                            let imm9h = (instr >> 16) & 11;
                            let imm9l = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Pt = instr & 7;
                            match () {
                                () => {
                                    Some(Op::LDR_P_BI__)
                                }
                            }
                        }
                        (_, x1, x2, _, x4, _, x6, _) if x1 == 3 && x2 & 2 == 0 && x4 == 0 && x6 == 1 => {
                            None
                        }
                        (_, x1, x2, _, x4, _, _, _) if x1 == 3 && x2 & 2 == 0 && x4 == 2 => {
                            let imm9h = (instr >> 16) & 11;
                            let imm9l = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match () {
                                () => {
                                    Some(Op::LDR_Z_BI__)
                                }
                            }
                        }
                        (_, x1, x2, _, x4, _, _, _) if x1 == 3 && x2 & 2 == 0 && x4 & 5 == 1 => {
                            None
                        }
                        (_, x1, x2, _, x4, _, x6, _) if x1 == 3 && x2 & 2 == 2 && x4 & 4 == 0 && x6 == 0 => {
                            let imm6 = (instr >> 16) & 11;
                            let msz = (instr >> 13) & 3;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let prfop = instr & 7;
                            match msz {
                                x0 if x0 == 0 => {
                                    Some(Op::PRFB_I_P_BI_S)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::PRFH_I_P_BI_S)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::PRFW_I_P_BI_S)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::PRFD_I_P_BI_S)
                                }
                                _ => None
                            }
                        }
                        (_, x1, x2, _, x4, _, x6, _) if x1 == 3 && x2 & 2 == 2 && x4 & 4 == 0 && x6 == 1 => {
                            None
                        }
                        (_, x1, x2, _, x4, _, _, _) if x1 != 3 && x2 & 1 == 0 && x4 & 4 == 0 => {
                            let opc = (instr >> 23) & 3;
                            let xs = (instr >> 22) & 1;
                            let Zm = (instr >> 16) & 9;
                            let U = (instr >> 14) & 1;
                            let ff = (instr >> 13) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match (opc, U, ff) {
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                                    Some(Op::LD1SB_Z_P_BZ_S_x32_unscaled)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                                    Some(Op::LDFF1SB_Z_P_BZ_S_x32_unscaled)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    Some(Op::LD1B_Z_P_BZ_S_x32_unscaled)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    Some(Op::LDFF1B_Z_P_BZ_S_x32_unscaled)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    Some(Op::LD1SH_Z_P_BZ_S_x32_unscaled)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    Some(Op::LDFF1SH_Z_P_BZ_S_x32_unscaled)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    Some(Op::LD1H_Z_P_BZ_S_x32_unscaled)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    Some(Op::LDFF1H_Z_P_BZ_S_x32_unscaled)
                                }
                                (x0, x1, _) if x0 == 2 && x1 == 0 => {
                                    None
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                                    Some(Op::LD1W_Z_P_BZ_S_x32_unscaled)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                                    Some(Op::LDFF1W_Z_P_BZ_S_x32_unscaled)
                                }
                                _ => None
                            }
                        }
                        (_, _, x2, _, x4, _, _, _) if x2 == 0 && x4 & 6 == 4 => {
                            None
                        }
                        (_, _, x2, _, x4, _, x6, _) if x2 == 0 && x4 == 6 && x6 == 0 => {
                            let msz = (instr >> 23) & 3;
                            let Rm = (instr >> 16) & 9;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let prfop = instr & 7;
                            match msz {
                                x0 if x0 == 0 => {
                                    Some(Op::PRFB_I_P_BR_S)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::PRFH_I_P_BR_S)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::PRFW_I_P_BR_S)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::PRFD_I_P_BR_S)
                                }
                                _ => None
                            }
                        }
                        (_, _, x2, _, x4, _, x6, _) if x2 == 0 && x4 == 7 && x6 == 0 => {
                            let msz = (instr >> 23) & 3;
                            let imm5 = (instr >> 16) & 9;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let prfop = instr & 7;
                            match msz {
                                x0 if x0 == 0 => {
                                    Some(Op::PRFB_I_P_AI_S)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::PRFH_I_P_AI_S)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::PRFW_I_P_AI_S)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::PRFD_I_P_AI_S)
                                }
                                _ => None
                            }
                        }
                        (_, _, x2, _, x4, _, x6, _) if x2 == 0 && x4 & 6 == 6 && x6 == 1 => {
                            None
                        }
                        (_, _, x2, _, x4, _, _, _) if x2 == 1 && x4 & 4 == 4 => {
                            let msz = (instr >> 23) & 3;
                            let imm5 = (instr >> 16) & 9;
                            let U = (instr >> 14) & 1;
                            let ff = (instr >> 13) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match (msz, U, ff) {
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                                    Some(Op::LD1SB_Z_P_AI_S)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                                    Some(Op::LDFF1SB_Z_P_AI_S)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    Some(Op::LD1B_Z_P_AI_S)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    Some(Op::LDFF1B_Z_P_AI_S)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    Some(Op::LD1SH_Z_P_AI_S)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    Some(Op::LDFF1SH_Z_P_AI_S)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    Some(Op::LD1H_Z_P_AI_S)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    Some(Op::LDFF1H_Z_P_AI_S)
                                }
                                (x0, x1, _) if x0 == 2 && x1 == 0 => {
                                    None
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                                    Some(Op::LD1W_Z_P_AI_S)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                                    Some(Op::LDFF1W_Z_P_AI_S)
                                }
                                (x0, _, _) if x0 == 3 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        (_, _, x2, _, x4, _, _, _) if x2 & 2 == 2 && x4 & 4 == 4 => {
                            let dtypeh = (instr >> 23) & 3;
                            let imm6 = (instr >> 16) & 11;
                            let dtypel = (instr >> 13) & 3;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match (dtypeh, dtypel) {
                                (x0, x1) if x0 == 0 && x1 == 0 => {
                                    Some(Op::LD1RB_Z_P_BI_U8)
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    Some(Op::LD1RB_Z_P_BI_U16)
                                }
                                (x0, x1) if x0 == 0 && x1 == 2 => {
                                    Some(Op::LD1RB_Z_P_BI_U32)
                                }
                                (x0, x1) if x0 == 0 && x1 == 3 => {
                                    Some(Op::LD1RB_Z_P_BI_U64)
                                }
                                (x0, x1) if x0 == 1 && x1 == 0 => {
                                    Some(Op::LD1RSW_Z_P_BI_S64)
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    Some(Op::LD1RH_Z_P_BI_U16)
                                }
                                (x0, x1) if x0 == 1 && x1 == 2 => {
                                    Some(Op::LD1RH_Z_P_BI_U32)
                                }
                                (x0, x1) if x0 == 1 && x1 == 3 => {
                                    Some(Op::LD1RH_Z_P_BI_U64)
                                }
                                (x0, x1) if x0 == 2 && x1 == 0 => {
                                    Some(Op::LD1RSH_Z_P_BI_S64)
                                }
                                (x0, x1) if x0 == 2 && x1 == 1 => {
                                    Some(Op::LD1RSH_Z_P_BI_S32)
                                }
                                (x0, x1) if x0 == 2 && x1 == 2 => {
                                    Some(Op::LD1RW_Z_P_BI_U32)
                                }
                                (x0, x1) if x0 == 2 && x1 == 3 => {
                                    Some(Op::LD1RW_Z_P_BI_U64)
                                }
                                (x0, x1) if x0 == 3 && x1 == 0 => {
                                    Some(Op::LD1RSB_Z_P_BI_S64)
                                }
                                (x0, x1) if x0 == 3 && x1 == 1 => {
                                    Some(Op::LD1RSB_Z_P_BI_S32)
                                }
                                (x0, x1) if x0 == 3 && x1 == 2 => {
                                    Some(Op::LD1RSB_Z_P_BI_S16)
                                }
                                (x0, x1) if x0 == 3 && x1 == 3 => {
                                    Some(Op::LD1RD_Z_P_BI_U64)
                                }
                                _ => None
                            }
                        }
                        _ => None
                    }
                }
                (x0, _, _, _, _, _, _, _) if x0 == 5 => {
                    match ((instr >> 25) & 13, (instr >> 23) & 3, (instr >> 21) & 3, (instr >> 20) & 1, (instr >> 16) & 7, (instr >> 13) & 5, instr & 25) {
                        (_, _, x2, x3, _, x5, _) if x2 == 0 && x3 == 0 && x5 == 7 => {
                            let msz = (instr >> 23) & 3;
                            let imm4 = (instr >> 16) & 7;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match msz {
                                x0 if x0 == 0 => {
                                    Some(Op::LDNT1B_Z_P_BI_Contiguous)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::LDNT1H_Z_P_BI_Contiguous)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::LDNT1W_Z_P_BI_Contiguous)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::LDNT1D_Z_P_BI_Contiguous)
                                }
                                _ => None
                            }
                        }
                        (_, _, x2, _, _, x5, _) if x2 == 0 && x5 == 6 => {
                            let msz = (instr >> 23) & 3;
                            let Rm = (instr >> 16) & 9;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match msz {
                                x0 if x0 == 0 => {
                                    Some(Op::LDNT1B_Z_P_BR_Contiguous)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::LDNT1H_Z_P_BR_Contiguous)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::LDNT1W_Z_P_BR_Contiguous)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::LDNT1D_Z_P_BR_Contiguous)
                                }
                                _ => None
                            }
                        }
                        (_, _, x2, x3, _, x5, _) if x2 != 0 && x3 == 0 && x5 == 7 => {
                            let msz = (instr >> 23) & 3;
                            let opc = (instr >> 21) & 3;
                            let imm4 = (instr >> 16) & 7;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match (msz, opc) {
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    Some(Op::LD2B_Z_P_BI_Contiguous)
                                }
                                (x0, x1) if x0 == 0 && x1 == 2 => {
                                    Some(Op::LD3B_Z_P_BI_Contiguous)
                                }
                                (x0, x1) if x0 == 0 && x1 == 3 => {
                                    Some(Op::LD4B_Z_P_BI_Contiguous)
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    Some(Op::LD2H_Z_P_BI_Contiguous)
                                }
                                (x0, x1) if x0 == 1 && x1 == 2 => {
                                    Some(Op::LD3H_Z_P_BI_Contiguous)
                                }
                                (x0, x1) if x0 == 1 && x1 == 3 => {
                                    Some(Op::LD4H_Z_P_BI_Contiguous)
                                }
                                (x0, x1) if x0 == 2 && x1 == 1 => {
                                    Some(Op::LD2W_Z_P_BI_Contiguous)
                                }
                                (x0, x1) if x0 == 2 && x1 == 2 => {
                                    Some(Op::LD3W_Z_P_BI_Contiguous)
                                }
                                (x0, x1) if x0 == 2 && x1 == 3 => {
                                    Some(Op::LD4W_Z_P_BI_Contiguous)
                                }
                                (x0, x1) if x0 == 3 && x1 == 1 => {
                                    Some(Op::LD2D_Z_P_BI_Contiguous)
                                }
                                (x0, x1) if x0 == 3 && x1 == 2 => {
                                    Some(Op::LD3D_Z_P_BI_Contiguous)
                                }
                                (x0, x1) if x0 == 3 && x1 == 3 => {
                                    Some(Op::LD4D_Z_P_BI_Contiguous)
                                }
                                _ => None
                            }
                        }
                        (_, _, x2, _, _, x5, _) if x2 != 0 && x5 == 6 => {
                            let msz = (instr >> 23) & 3;
                            let opc = (instr >> 21) & 3;
                            let Rm = (instr >> 16) & 9;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match (msz, opc) {
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    Some(Op::LD2B_Z_P_BR_Contiguous)
                                }
                                (x0, x1) if x0 == 0 && x1 == 2 => {
                                    Some(Op::LD3B_Z_P_BR_Contiguous)
                                }
                                (x0, x1) if x0 == 0 && x1 == 3 => {
                                    Some(Op::LD4B_Z_P_BR_Contiguous)
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    Some(Op::LD2H_Z_P_BR_Contiguous)
                                }
                                (x0, x1) if x0 == 1 && x1 == 2 => {
                                    Some(Op::LD3H_Z_P_BR_Contiguous)
                                }
                                (x0, x1) if x0 == 1 && x1 == 3 => {
                                    Some(Op::LD4H_Z_P_BR_Contiguous)
                                }
                                (x0, x1) if x0 == 2 && x1 == 1 => {
                                    Some(Op::LD2W_Z_P_BR_Contiguous)
                                }
                                (x0, x1) if x0 == 2 && x1 == 2 => {
                                    Some(Op::LD3W_Z_P_BR_Contiguous)
                                }
                                (x0, x1) if x0 == 2 && x1 == 3 => {
                                    Some(Op::LD4W_Z_P_BR_Contiguous)
                                }
                                (x0, x1) if x0 == 3 && x1 == 1 => {
                                    Some(Op::LD2D_Z_P_BR_Contiguous)
                                }
                                (x0, x1) if x0 == 3 && x1 == 2 => {
                                    Some(Op::LD3D_Z_P_BR_Contiguous)
                                }
                                (x0, x1) if x0 == 3 && x1 == 3 => {
                                    Some(Op::LD4D_Z_P_BR_Contiguous)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, x5, _) if x3 == 0 && x5 == 1 => {
                            let msz = (instr >> 23) & 3;
                            let ssz = (instr >> 21) & 3;
                            let imm4 = (instr >> 16) & 7;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match (msz, ssz) {
                                (_, x1) if x1 & 2 == 2 => {
                                    None
                                }
                                (x0, x1) if x0 == 0 && x1 == 0 => {
                                    Some(Op::LD1RQB_Z_P_BI_U8)
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    Some(Op::LD1ROB_Z_P_BI_U8)
                                }
                                (x0, x1) if x0 == 1 && x1 == 0 => {
                                    Some(Op::LD1RQH_Z_P_BI_U16)
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    Some(Op::LD1ROH_Z_P_BI_U16)
                                }
                                (x0, x1) if x0 == 2 && x1 == 0 => {
                                    Some(Op::LD1RQW_Z_P_BI_U32)
                                }
                                (x0, x1) if x0 == 2 && x1 == 1 => {
                                    Some(Op::LD1ROW_Z_P_BI_U32)
                                }
                                (x0, x1) if x0 == 3 && x1 == 0 => {
                                    Some(Op::LD1RQD_Z_P_BI_U64)
                                }
                                (x0, x1) if x0 == 3 && x1 == 1 => {
                                    Some(Op::LD1ROD_Z_P_BI_U64)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, x5, _) if x3 == 0 && x5 == 5 => {
                            let dtype = (instr >> 21) & 7;
                            let imm4 = (instr >> 16) & 7;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match dtype {
                                x0 if x0 == 0 => {
                                    Some(Op::LD1B_Z_P_BI_U8)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::LD1B_Z_P_BI_U16)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::LD1B_Z_P_BI_U32)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::LD1B_Z_P_BI_U64)
                                }
                                x0 if x0 == 4 => {
                                    Some(Op::LD1SW_Z_P_BI_S64)
                                }
                                x0 if x0 == 5 => {
                                    Some(Op::LD1H_Z_P_BI_U16)
                                }
                                x0 if x0 == 6 => {
                                    Some(Op::LD1H_Z_P_BI_U32)
                                }
                                x0 if x0 == 7 => {
                                    Some(Op::LD1H_Z_P_BI_U64)
                                }
                                x0 if x0 == 8 => {
                                    Some(Op::LD1SH_Z_P_BI_S64)
                                }
                                x0 if x0 == 9 => {
                                    Some(Op::LD1SH_Z_P_BI_S32)
                                }
                                x0 if x0 == 10 => {
                                    Some(Op::LD1W_Z_P_BI_U32)
                                }
                                x0 if x0 == 11 => {
                                    Some(Op::LD1W_Z_P_BI_U64)
                                }
                                x0 if x0 == 12 => {
                                    Some(Op::LD1SB_Z_P_BI_S64)
                                }
                                x0 if x0 == 13 => {
                                    Some(Op::LD1SB_Z_P_BI_S32)
                                }
                                x0 if x0 == 14 => {
                                    Some(Op::LD1SB_Z_P_BI_S16)
                                }
                                x0 if x0 == 15 => {
                                    Some(Op::LD1D_Z_P_BI_U64)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, x5, _) if x3 == 1 && x5 == 1 => {
                            None
                        }
                        (_, _, _, x3, _, x5, _) if x3 == 1 && x5 == 5 => {
                            let dtype = (instr >> 21) & 7;
                            let imm4 = (instr >> 16) & 7;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match dtype {
                                x0 if x0 == 0 => {
                                    Some(Op::LDNF1B_Z_P_BI_U8)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::LDNF1B_Z_P_BI_U16)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::LDNF1B_Z_P_BI_U32)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::LDNF1B_Z_P_BI_U64)
                                }
                                x0 if x0 == 4 => {
                                    Some(Op::LDNF1SW_Z_P_BI_S64)
                                }
                                x0 if x0 == 5 => {
                                    Some(Op::LDNF1H_Z_P_BI_U16)
                                }
                                x0 if x0 == 6 => {
                                    Some(Op::LDNF1H_Z_P_BI_U32)
                                }
                                x0 if x0 == 7 => {
                                    Some(Op::LDNF1H_Z_P_BI_U64)
                                }
                                x0 if x0 == 8 => {
                                    Some(Op::LDNF1SH_Z_P_BI_S64)
                                }
                                x0 if x0 == 9 => {
                                    Some(Op::LDNF1SH_Z_P_BI_S32)
                                }
                                x0 if x0 == 10 => {
                                    Some(Op::LDNF1W_Z_P_BI_U32)
                                }
                                x0 if x0 == 11 => {
                                    Some(Op::LDNF1W_Z_P_BI_U64)
                                }
                                x0 if x0 == 12 => {
                                    Some(Op::LDNF1SB_Z_P_BI_S64)
                                }
                                x0 if x0 == 13 => {
                                    Some(Op::LDNF1SB_Z_P_BI_S32)
                                }
                                x0 if x0 == 14 => {
                                    Some(Op::LDNF1SB_Z_P_BI_S16)
                                }
                                x0 if x0 == 15 => {
                                    Some(Op::LDNF1D_Z_P_BI_U64)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, x5, _) if x3 == 1 && x5 == 7 => {
                            None
                        }
                        (_, _, _, _, _, x5, _) if x5 == 0 => {
                            let msz = (instr >> 23) & 3;
                            let ssz = (instr >> 21) & 3;
                            let Rm = (instr >> 16) & 9;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match (msz, ssz) {
                                (_, x1) if x1 & 2 == 2 => {
                                    None
                                }
                                (x0, x1) if x0 == 0 && x1 == 0 => {
                                    Some(Op::LD1RQB_Z_P_BR_Contiguous)
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    Some(Op::LD1ROB_Z_P_BR_Contiguous)
                                }
                                (x0, x1) if x0 == 1 && x1 == 0 => {
                                    Some(Op::LD1RQH_Z_P_BR_Contiguous)
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    Some(Op::LD1ROH_Z_P_BR_Contiguous)
                                }
                                (x0, x1) if x0 == 2 && x1 == 0 => {
                                    Some(Op::LD1RQW_Z_P_BR_Contiguous)
                                }
                                (x0, x1) if x0 == 2 && x1 == 1 => {
                                    Some(Op::LD1ROW_Z_P_BR_Contiguous)
                                }
                                (x0, x1) if x0 == 3 && x1 == 0 => {
                                    Some(Op::LD1RQD_Z_P_BR_Contiguous)
                                }
                                (x0, x1) if x0 == 3 && x1 == 1 => {
                                    Some(Op::LD1ROD_Z_P_BR_Contiguous)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, _, _, x5, _) if x5 == 2 => {
                            let dtype = (instr >> 21) & 7;
                            let Rm = (instr >> 16) & 9;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match dtype {
                                x0 if x0 == 0 => {
                                    Some(Op::LD1B_Z_P_BR_U8)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::LD1B_Z_P_BR_U16)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::LD1B_Z_P_BR_U32)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::LD1B_Z_P_BR_U64)
                                }
                                x0 if x0 == 4 => {
                                    Some(Op::LD1SW_Z_P_BR_S64)
                                }
                                x0 if x0 == 5 => {
                                    Some(Op::LD1H_Z_P_BR_U16)
                                }
                                x0 if x0 == 6 => {
                                    Some(Op::LD1H_Z_P_BR_U32)
                                }
                                x0 if x0 == 7 => {
                                    Some(Op::LD1H_Z_P_BR_U64)
                                }
                                x0 if x0 == 8 => {
                                    Some(Op::LD1SH_Z_P_BR_S64)
                                }
                                x0 if x0 == 9 => {
                                    Some(Op::LD1SH_Z_P_BR_S32)
                                }
                                x0 if x0 == 10 => {
                                    Some(Op::LD1W_Z_P_BR_U32)
                                }
                                x0 if x0 == 11 => {
                                    Some(Op::LD1W_Z_P_BR_U64)
                                }
                                x0 if x0 == 12 => {
                                    Some(Op::LD1SB_Z_P_BR_S64)
                                }
                                x0 if x0 == 13 => {
                                    Some(Op::LD1SB_Z_P_BR_S32)
                                }
                                x0 if x0 == 14 => {
                                    Some(Op::LD1SB_Z_P_BR_S16)
                                }
                                x0 if x0 == 15 => {
                                    Some(Op::LD1D_Z_P_BR_U64)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, _, _, x5, _) if x5 == 3 => {
                            let dtype = (instr >> 21) & 7;
                            let Rm = (instr >> 16) & 9;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match dtype {
                                x0 if x0 == 0 => {
                                    Some(Op::LDFF1B_Z_P_BR_U8)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::LDFF1B_Z_P_BR_U16)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::LDFF1B_Z_P_BR_U32)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::LDFF1B_Z_P_BR_U64)
                                }
                                x0 if x0 == 4 => {
                                    Some(Op::LDFF1SW_Z_P_BR_S64)
                                }
                                x0 if x0 == 5 => {
                                    Some(Op::LDFF1H_Z_P_BR_U16)
                                }
                                x0 if x0 == 6 => {
                                    Some(Op::LDFF1H_Z_P_BR_U32)
                                }
                                x0 if x0 == 7 => {
                                    Some(Op::LDFF1H_Z_P_BR_U64)
                                }
                                x0 if x0 == 8 => {
                                    Some(Op::LDFF1SH_Z_P_BR_S64)
                                }
                                x0 if x0 == 9 => {
                                    Some(Op::LDFF1SH_Z_P_BR_S32)
                                }
                                x0 if x0 == 10 => {
                                    Some(Op::LDFF1W_Z_P_BR_U32)
                                }
                                x0 if x0 == 11 => {
                                    Some(Op::LDFF1W_Z_P_BR_U64)
                                }
                                x0 if x0 == 12 => {
                                    Some(Op::LDFF1SB_Z_P_BR_S64)
                                }
                                x0 if x0 == 13 => {
                                    Some(Op::LDFF1SB_Z_P_BR_S32)
                                }
                                x0 if x0 == 14 => {
                                    Some(Op::LDFF1SB_Z_P_BR_S16)
                                }
                                x0 if x0 == 15 => {
                                    Some(Op::LDFF1D_Z_P_BR_U64)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, _, _, x5, _) if x5 == 4 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, _, _, _, _, _, _) if x0 == 6 => {
                    match ((instr >> 25) & 13, (instr >> 23) & 3, (instr >> 21) & 3, (instr >> 16) & 9, (instr >> 13) & 5, (instr >> 5) & 15, (instr >> 4) & 1, instr & 7) {
                        (_, x1, x2, _, x4, _, x6, _) if x1 == 0 && x2 == 1 && x4 & 4 == 0 && x6 == 1 => {
                            None
                        }
                        (_, x1, x2, _, x4, _, x6, _) if x1 == 0 && x2 == 3 && x4 & 4 == 4 && x6 == 0 => {
                            let Zm = (instr >> 16) & 9;
                            let msz = (instr >> 13) & 3;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let prfop = instr & 7;
                            match msz {
                                x0 if x0 == 0 => {
                                    Some(Op::PRFB_I_P_BZ_D_64_scaled)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::PRFH_I_P_BZ_D_64_scaled)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::PRFW_I_P_BZ_D_64_scaled)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::PRFD_I_P_BZ_D_64_scaled)
                                }
                                _ => None
                            }
                        }
                        (_, x1, x2, _, _, _, x6, _) if x1 == 0 && x2 == 3 && x6 == 1 => {
                            None
                        }
                        (_, x1, x2, _, x4, _, x6, _) if x1 == 0 && x2 & 1 == 1 && x4 & 4 == 0 && x6 == 0 => {
                            let xs = (instr >> 22) & 1;
                            let Zm = (instr >> 16) & 9;
                            let msz = (instr >> 13) & 3;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let prfop = instr & 7;
                            match msz {
                                x0 if x0 == 0 => {
                                    Some(Op::PRFB_I_P_BZ_D_x32_scaled)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::PRFH_I_P_BZ_D_x32_scaled)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::PRFW_I_P_BZ_D_x32_scaled)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::PRFD_I_P_BZ_D_x32_scaled)
                                }
                                _ => None
                            }
                        }
                        (_, x1, x2, _, x4, _, _, _) if x1 != 0 && x2 == 3 && x4 & 4 == 4 => {
                            let opc = (instr >> 23) & 3;
                            let Zm = (instr >> 16) & 9;
                            let U = (instr >> 14) & 1;
                            let ff = (instr >> 13) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match (opc, U, ff) {
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    Some(Op::LD1SH_Z_P_BZ_D_64_scaled)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    Some(Op::LDFF1SH_Z_P_BZ_D_64_scaled)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    Some(Op::LD1H_Z_P_BZ_D_64_scaled)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    Some(Op::LDFF1H_Z_P_BZ_D_64_scaled)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                                    Some(Op::LD1SW_Z_P_BZ_D_64_scaled)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                                    Some(Op::LDFF1SW_Z_P_BZ_D_64_scaled)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                                    Some(Op::LD1W_Z_P_BZ_D_64_scaled)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                                    Some(Op::LDFF1W_Z_P_BZ_D_64_scaled)
                                }
                                (x0, x1, _) if x0 == 3 && x1 == 0 => {
                                    None
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 0 => {
                                    Some(Op::LD1D_Z_P_BZ_D_64_scaled)
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 1 => {
                                    Some(Op::LDFF1D_Z_P_BZ_D_64_scaled)
                                }
                                _ => None
                            }
                        }
                        (_, x1, x2, _, x4, _, _, _) if x1 != 0 && x2 & 1 == 1 && x4 & 4 == 0 => {
                            let opc = (instr >> 23) & 3;
                            let xs = (instr >> 22) & 1;
                            let Zm = (instr >> 16) & 9;
                            let U = (instr >> 14) & 1;
                            let ff = (instr >> 13) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match (opc, U, ff) {
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    Some(Op::LD1SH_Z_P_BZ_D_x32_scaled)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    Some(Op::LDFF1SH_Z_P_BZ_D_x32_scaled)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    Some(Op::LD1H_Z_P_BZ_D_x32_scaled)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    Some(Op::LDFF1H_Z_P_BZ_D_x32_scaled)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                                    Some(Op::LD1SW_Z_P_BZ_D_x32_scaled)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                                    Some(Op::LDFF1SW_Z_P_BZ_D_x32_scaled)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                                    Some(Op::LD1W_Z_P_BZ_D_x32_scaled)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                                    Some(Op::LDFF1W_Z_P_BZ_D_x32_scaled)
                                }
                                (x0, x1, _) if x0 == 3 && x1 == 0 => {
                                    None
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 0 => {
                                    Some(Op::LD1D_Z_P_BZ_D_x32_scaled)
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 1 => {
                                    Some(Op::LDFF1D_Z_P_BZ_D_x32_scaled)
                                }
                                _ => None
                            }
                        }
                        (_, _, x2, _, x4, _, _, _) if x2 == 0 && x4 & 6 == 4 => {
                            None
                        }
                        (_, _, x2, _, x4, _, _, _) if x2 == 0 && x4 == 6 => {
                            None
                        }
                        (_, _, x2, _, x4, _, x6, _) if x2 == 0 && x4 == 7 && x6 == 0 => {
                            let msz = (instr >> 23) & 3;
                            let imm5 = (instr >> 16) & 9;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let prfop = instr & 7;
                            match msz {
                                x0 if x0 == 0 => {
                                    Some(Op::PRFB_I_P_AI_D)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::PRFH_I_P_AI_D)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::PRFW_I_P_AI_D)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::PRFD_I_P_AI_D)
                                }
                                _ => None
                            }
                        }
                        (_, _, x2, _, x4, _, x6, _) if x2 == 0 && x4 == 7 && x6 == 1 => {
                            None
                        }
                        (_, _, x2, _, x4, _, _, _) if x2 == 1 && x4 & 4 == 4 => {
                            let msz = (instr >> 23) & 3;
                            let imm5 = (instr >> 16) & 9;
                            let U = (instr >> 14) & 1;
                            let ff = (instr >> 13) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match (msz, U, ff) {
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                                    Some(Op::LD1SB_Z_P_AI_D)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                                    Some(Op::LDFF1SB_Z_P_AI_D)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    Some(Op::LD1B_Z_P_AI_D)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    Some(Op::LDFF1B_Z_P_AI_D)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    Some(Op::LD1SH_Z_P_AI_D)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    Some(Op::LDFF1SH_Z_P_AI_D)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    Some(Op::LD1H_Z_P_AI_D)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    Some(Op::LDFF1H_Z_P_AI_D)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                                    Some(Op::LD1SW_Z_P_AI_D)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                                    Some(Op::LDFF1SW_Z_P_AI_D)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                                    Some(Op::LD1W_Z_P_AI_D)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                                    Some(Op::LDFF1W_Z_P_AI_D)
                                }
                                (x0, x1, _) if x0 == 3 && x1 == 0 => {
                                    None
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 0 => {
                                    Some(Op::LD1D_Z_P_AI_D)
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 1 => {
                                    Some(Op::LDFF1D_Z_P_AI_D)
                                }
                                _ => None
                            }
                        }
                        (_, _, x2, _, x4, _, _, _) if x2 == 2 && x4 & 4 == 4 => {
                            let msz = (instr >> 23) & 3;
                            let Zm = (instr >> 16) & 9;
                            let U = (instr >> 14) & 1;
                            let ff = (instr >> 13) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match (msz, U, ff) {
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                                    Some(Op::LD1SB_Z_P_BZ_D_64_unscaled)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                                    Some(Op::LDFF1SB_Z_P_BZ_D_64_unscaled)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    Some(Op::LD1B_Z_P_BZ_D_64_unscaled)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    Some(Op::LDFF1B_Z_P_BZ_D_64_unscaled)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    Some(Op::LD1SH_Z_P_BZ_D_64_unscaled)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    Some(Op::LDFF1SH_Z_P_BZ_D_64_unscaled)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    Some(Op::LD1H_Z_P_BZ_D_64_unscaled)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    Some(Op::LDFF1H_Z_P_BZ_D_64_unscaled)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                                    Some(Op::LD1SW_Z_P_BZ_D_64_unscaled)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                                    Some(Op::LDFF1SW_Z_P_BZ_D_64_unscaled)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                                    Some(Op::LD1W_Z_P_BZ_D_64_unscaled)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                                    Some(Op::LDFF1W_Z_P_BZ_D_64_unscaled)
                                }
                                (x0, x1, _) if x0 == 3 && x1 == 0 => {
                                    None
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 0 => {
                                    Some(Op::LD1D_Z_P_BZ_D_64_unscaled)
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 1 => {
                                    Some(Op::LDFF1D_Z_P_BZ_D_64_unscaled)
                                }
                                _ => None
                            }
                        }
                        (_, _, x2, _, x4, _, _, _) if x2 & 1 == 0 && x4 & 4 == 0 => {
                            let msz = (instr >> 23) & 3;
                            let xs = (instr >> 22) & 1;
                            let Zm = (instr >> 16) & 9;
                            let U = (instr >> 14) & 1;
                            let ff = (instr >> 13) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match (msz, U, ff) {
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                                    Some(Op::LD1SB_Z_P_BZ_D_x32_unscaled)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                                    Some(Op::LDFF1SB_Z_P_BZ_D_x32_unscaled)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    Some(Op::LD1B_Z_P_BZ_D_x32_unscaled)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    Some(Op::LDFF1B_Z_P_BZ_D_x32_unscaled)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    Some(Op::LD1SH_Z_P_BZ_D_x32_unscaled)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    Some(Op::LDFF1SH_Z_P_BZ_D_x32_unscaled)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    Some(Op::LD1H_Z_P_BZ_D_x32_unscaled)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    Some(Op::LDFF1H_Z_P_BZ_D_x32_unscaled)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                                    Some(Op::LD1SW_Z_P_BZ_D_x32_unscaled)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                                    Some(Op::LDFF1SW_Z_P_BZ_D_x32_unscaled)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                                    Some(Op::LD1W_Z_P_BZ_D_x32_unscaled)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                                    Some(Op::LDFF1W_Z_P_BZ_D_x32_unscaled)
                                }
                                (x0, x1, _) if x0 == 3 && x1 == 0 => {
                                    None
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 0 => {
                                    Some(Op::LD1D_Z_P_BZ_D_x32_unscaled)
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 1 => {
                                    Some(Op::LDFF1D_Z_P_BZ_D_x32_unscaled)
                                }
                                _ => None
                            }
                        }
                        _ => None
                    }
                }
                (x0, _, _, _, _, _, x6, _) if x0 == 7 && x6 & 40 == 0 => {
                    match ((instr >> 25) & 13, (instr >> 22) & 5, (instr >> 16) & 11, (instr >> 15) & 1, (instr >> 14) & 1, (instr >> 13) & 1, (instr >> 5) & 15, (instr >> 4) & 1, instr & 7) {
                        (_, x1, _, _, x4, _, _, _, _) if x1 & 4 == 0 && x4 == 0 => {
                            None
                        }
                        (_, x1, _, _, x4, _, _, _, _) if x1 & 6 == 4 && x4 == 0 => {
                            None
                        }
                        (_, x1, _, _, x4, _, _, x7, _) if x1 == 6 && x4 == 0 && x7 == 0 => {
                            let imm9h = (instr >> 16) & 11;
                            let imm9l = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Pt = instr & 7;
                            match () {
                                () => {
                                    Some(Op::STR_P_BI__)
                                }
                            }
                        }
                        (_, x1, _, _, x4, _, _, x7, _) if x1 == 6 && x4 == 0 && x7 == 1 => {
                            None
                        }
                        (_, x1, _, _, x4, _, _, _, _) if x1 == 6 && x4 == 1 => {
                            let imm9h = (instr >> 16) & 11;
                            let imm9l = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match () {
                                () => {
                                    Some(Op::STR_Z_BI__)
                                }
                            }
                        }
                        (_, x1, _, _, x4, _, _, _, _) if x1 == 7 && x4 == 0 => {
                            None
                        }
                        (_, x1, _, _, x4, _, _, _, _) if x1 != 6 && x4 == 1 => {
                            let opc = (instr >> 22) & 5;
                            let o2 = (instr >> 21) & 1;
                            let Rm = (instr >> 16) & 9;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match (opc, o2) {
                                (x0, _) if x0 & 6 == 0 => {
                                    Some(Op::ST1B_Z_P_BR__)
                                }
                                (x0, _) if x0 & 6 == 2 => {
                                    Some(Op::ST1H_Z_P_BR__)
                                }
                                (x0, _) if x0 & 6 == 4 => {
                                    Some(Op::ST1W_Z_P_BR__)
                                }
                                (x0, x1) if x0 == 7 && x1 == 0 => {
                                    None
                                }
                                (x0, x1) if x0 == 7 && x1 == 1 => {
                                    Some(Op::ST1D_Z_P_BR__)
                                }
                                _ => None
                            }
                        }
                        _ => None
                    }
                }
                (x0, _, _, _, _, _, x6, _) if x0 == 7 && x6 & 40 == 8 => {
                    match ((instr >> 25) & 13, (instr >> 23) & 3, (instr >> 21) & 3, (instr >> 16) & 9, (instr >> 15) & 1, (instr >> 14) & 1, (instr >> 13) & 1, instr & 25) {
                        (_, _, x2, _, _, x5, _, _) if x2 == 0 && x5 == 1 => {
                            let msz = (instr >> 23) & 3;
                            let Rm = (instr >> 16) & 9;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match msz {
                                x0 if x0 == 0 => {
                                    Some(Op::STNT1B_Z_P_BR_Contiguous)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::STNT1H_Z_P_BR_Contiguous)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::STNT1W_Z_P_BR_Contiguous)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::STNT1D_Z_P_BR_Contiguous)
                                }
                                _ => None
                            }
                        }
                        (_, _, x2, _, _, x5, _, _) if x2 != 0 && x5 == 1 => {
                            let msz = (instr >> 23) & 3;
                            let opc = (instr >> 21) & 3;
                            let Rm = (instr >> 16) & 9;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match (msz, opc) {
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    Some(Op::ST2B_Z_P_BR_Contiguous)
                                }
                                (x0, x1) if x0 == 0 && x1 == 2 => {
                                    Some(Op::ST3B_Z_P_BR_Contiguous)
                                }
                                (x0, x1) if x0 == 0 && x1 == 3 => {
                                    Some(Op::ST4B_Z_P_BR_Contiguous)
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    Some(Op::ST2H_Z_P_BR_Contiguous)
                                }
                                (x0, x1) if x0 == 1 && x1 == 2 => {
                                    Some(Op::ST3H_Z_P_BR_Contiguous)
                                }
                                (x0, x1) if x0 == 1 && x1 == 3 => {
                                    Some(Op::ST4H_Z_P_BR_Contiguous)
                                }
                                (x0, x1) if x0 == 2 && x1 == 1 => {
                                    Some(Op::ST2W_Z_P_BR_Contiguous)
                                }
                                (x0, x1) if x0 == 2 && x1 == 2 => {
                                    Some(Op::ST3W_Z_P_BR_Contiguous)
                                }
                                (x0, x1) if x0 == 2 && x1 == 3 => {
                                    Some(Op::ST4W_Z_P_BR_Contiguous)
                                }
                                (x0, x1) if x0 == 3 && x1 == 1 => {
                                    Some(Op::ST2D_Z_P_BR_Contiguous)
                                }
                                (x0, x1) if x0 == 3 && x1 == 2 => {
                                    Some(Op::ST3D_Z_P_BR_Contiguous)
                                }
                                (x0, x1) if x0 == 3 && x1 == 3 => {
                                    Some(Op::ST4D_Z_P_BR_Contiguous)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, _, _, x5, _, _) if x5 == 0 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, _, _, _, _, x6, _) if x0 == 7 && x6 & 40 == 32 => {
                    match ((instr >> 25) & 13, (instr >> 23) & 3, (instr >> 21) & 3, (instr >> 16) & 9, (instr >> 15) & 1, (instr >> 14) & 1, (instr >> 13) & 1, instr & 25) {
                        (_, _, x2, _, _, _, _, _) if x2 == 0 => {
                            let msz = (instr >> 23) & 3;
                            let Zm = (instr >> 16) & 9;
                            let xs = (instr >> 14) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match msz {
                                x0 if x0 == 0 => {
                                    Some(Op::ST1B_Z_P_BZ_D_x32_unscaled)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::ST1H_Z_P_BZ_D_x32_unscaled)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::ST1W_Z_P_BZ_D_x32_unscaled)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::ST1D_Z_P_BZ_D_x32_unscaled)
                                }
                                _ => None
                            }
                        }
                        (_, _, x2, _, _, _, _, _) if x2 == 1 => {
                            let msz = (instr >> 23) & 3;
                            let Zm = (instr >> 16) & 9;
                            let xs = (instr >> 14) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match msz {
                                x0 if x0 == 0 => {
                                    None
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::ST1H_Z_P_BZ_D_x32_scaled)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::ST1W_Z_P_BZ_D_x32_scaled)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::ST1D_Z_P_BZ_D_x32_scaled)
                                }
                                _ => None
                            }
                        }
                        (_, _, x2, _, _, _, _, _) if x2 == 2 => {
                            let msz = (instr >> 23) & 3;
                            let Zm = (instr >> 16) & 9;
                            let xs = (instr >> 14) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match msz {
                                x0 if x0 == 0 => {
                                    Some(Op::ST1B_Z_P_BZ_S_x32_unscaled)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::ST1H_Z_P_BZ_S_x32_unscaled)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::ST1W_Z_P_BZ_S_x32_unscaled)
                                }
                                x0 if x0 == 3 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        (_, _, x2, _, _, _, _, _) if x2 == 3 => {
                            let msz = (instr >> 23) & 3;
                            let Zm = (instr >> 16) & 9;
                            let xs = (instr >> 14) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match msz {
                                x0 if x0 == 0 => {
                                    None
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::ST1H_Z_P_BZ_S_x32_scaled)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::ST1W_Z_P_BZ_S_x32_scaled)
                                }
                                x0 if x0 == 3 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        _ => None
                    }
                }
                (x0, _, _, _, _, _, x6, _) if x0 == 7 && x6 & 56 == 40 => {
                    match ((instr >> 25) & 13, (instr >> 23) & 3, (instr >> 21) & 3, (instr >> 16) & 9, (instr >> 13) & 5, instr & 25) {
                        (_, _, x2, _, _, _) if x2 == 0 => {
                            let msz = (instr >> 23) & 3;
                            let Zm = (instr >> 16) & 9;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match msz {
                                x0 if x0 == 0 => {
                                    Some(Op::ST1B_Z_P_BZ_D_64_unscaled)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::ST1H_Z_P_BZ_D_64_unscaled)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::ST1W_Z_P_BZ_D_64_unscaled)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::ST1D_Z_P_BZ_D_64_unscaled)
                                }
                                _ => None
                            }
                        }
                        (_, _, x2, _, _, _) if x2 == 1 => {
                            let msz = (instr >> 23) & 3;
                            let Zm = (instr >> 16) & 9;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match msz {
                                x0 if x0 == 0 => {
                                    None
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::ST1H_Z_P_BZ_D_64_scaled)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::ST1W_Z_P_BZ_D_64_scaled)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::ST1D_Z_P_BZ_D_64_scaled)
                                }
                                _ => None
                            }
                        }
                        (_, _, x2, _, _, _) if x2 == 2 => {
                            let msz = (instr >> 23) & 3;
                            let imm5 = (instr >> 16) & 9;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match msz {
                                x0 if x0 == 0 => {
                                    Some(Op::ST1B_Z_P_AI_D)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::ST1H_Z_P_AI_D)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::ST1W_Z_P_AI_D)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::ST1D_Z_P_AI_D)
                                }
                                _ => None
                            }
                        }
                        (_, _, x2, _, _, _) if x2 == 3 => {
                            let msz = (instr >> 23) & 3;
                            let imm5 = (instr >> 16) & 9;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match msz {
                                x0 if x0 == 0 => {
                                    Some(Op::ST1B_Z_P_AI_S)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::ST1H_Z_P_AI_S)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::ST1W_Z_P_AI_S)
                                }
                                x0 if x0 == 3 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        _ => None
                    }
                }
                (x0, _, _, _, _, _, x6, _) if x0 == 7 && x6 & 56 == 56 => {
                    match ((instr >> 25) & 13, (instr >> 23) & 3, (instr >> 21) & 3, (instr >> 20) & 1, (instr >> 16) & 7, (instr >> 13) & 5, instr & 25) {
                        (_, _, x2, x3, _, _, _) if x2 == 0 && x3 == 1 => {
                            let msz = (instr >> 23) & 3;
                            let imm4 = (instr >> 16) & 7;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match msz {
                                x0 if x0 == 0 => {
                                    Some(Op::STNT1B_Z_P_BI_Contiguous)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::STNT1H_Z_P_BI_Contiguous)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::STNT1W_Z_P_BI_Contiguous)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::STNT1D_Z_P_BI_Contiguous)
                                }
                                _ => None
                            }
                        }
                        (_, _, x2, x3, _, _, _) if x2 != 0 && x3 == 1 => {
                            let msz = (instr >> 23) & 3;
                            let opc = (instr >> 21) & 3;
                            let imm4 = (instr >> 16) & 7;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match (msz, opc) {
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    Some(Op::ST2B_Z_P_BI_Contiguous)
                                }
                                (x0, x1) if x0 == 0 && x1 == 2 => {
                                    Some(Op::ST3B_Z_P_BI_Contiguous)
                                }
                                (x0, x1) if x0 == 0 && x1 == 3 => {
                                    Some(Op::ST4B_Z_P_BI_Contiguous)
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    Some(Op::ST2H_Z_P_BI_Contiguous)
                                }
                                (x0, x1) if x0 == 1 && x1 == 2 => {
                                    Some(Op::ST3H_Z_P_BI_Contiguous)
                                }
                                (x0, x1) if x0 == 1 && x1 == 3 => {
                                    Some(Op::ST4H_Z_P_BI_Contiguous)
                                }
                                (x0, x1) if x0 == 2 && x1 == 1 => {
                                    Some(Op::ST2W_Z_P_BI_Contiguous)
                                }
                                (x0, x1) if x0 == 2 && x1 == 2 => {
                                    Some(Op::ST3W_Z_P_BI_Contiguous)
                                }
                                (x0, x1) if x0 == 2 && x1 == 3 => {
                                    Some(Op::ST4W_Z_P_BI_Contiguous)
                                }
                                (x0, x1) if x0 == 3 && x1 == 1 => {
                                    Some(Op::ST2D_Z_P_BI_Contiguous)
                                }
                                (x0, x1) if x0 == 3 && x1 == 2 => {
                                    Some(Op::ST3D_Z_P_BI_Contiguous)
                                }
                                (x0, x1) if x0 == 3 && x1 == 3 => {
                                    Some(Op::ST4D_Z_P_BI_Contiguous)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, _, _) if x3 == 0 => {
                            let msz = (instr >> 23) & 3;
                            let size = (instr >> 21) & 3;
                            let imm4 = (instr >> 16) & 7;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match msz {
                                x0 if x0 == 0 => {
                                    Some(Op::ST1B_Z_P_BI__)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::ST1H_Z_P_BI__)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::ST1W_Z_P_BI__)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::ST1D_Z_P_BI__)
                                }
                                _ => None
                            }
                        }
                        _ => None
                    }
                }
                _ => None
            }
        }
        (_, x1, _) if x1 & 30 == 6 => {
            None
        }
        (_, x1, _) if x1 & 28 == 16 => {
            match ((instr >> 29) & 5, (instr >> 26) & 5, (instr >> 23) & 5, instr & 45) {
                (_, _, x2, _) if x2 & 6 == 0 => {
                    let op = (instr >> 31) & 1;
                    let immlo = (instr >> 29) & 3;
                    let immhi = (instr >> 5) & 37;
                    let Rd = instr & 9;
                    match op {
                        x0 if x0 == 0 => {
                            Some(Op::aarch64_integer_arithmetic_address_pc_rel)
                        }
                        x0 if x0 == 1 => {
                            Some(Op::aarch64_integer_arithmetic_address_pc_rel)
                        }
                        _ => None
                    }
                }
                (_, _, x2, _) if x2 == 2 => {
                    let sf = (instr >> 31) & 1;
                    let op = (instr >> 30) & 1;
                    let S = (instr >> 29) & 1;
                    let sh = (instr >> 22) & 1;
                    let imm12 = (instr >> 10) & 23;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (sf, op, S) {
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_integer_arithmetic_add_sub_immediate)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_integer_arithmetic_add_sub_immediate)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_integer_arithmetic_add_sub_immediate)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_integer_arithmetic_add_sub_immediate)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_integer_arithmetic_add_sub_immediate)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_integer_arithmetic_add_sub_immediate)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_integer_arithmetic_add_sub_immediate)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_integer_arithmetic_add_sub_immediate)
                        }
                        _ => None
                    }
                }
                (_, _, x2, _) if x2 == 3 => {
                    let sf = (instr >> 31) & 1;
                    let op = (instr >> 30) & 1;
                    let S = (instr >> 29) & 1;
                    let o2 = (instr >> 22) & 1;
                    let uimm6 = (instr >> 16) & 11;
                    let op3 = (instr >> 14) & 3;
                    let uimm4 = (instr >> 10) & 7;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (sf, op, S, o2) {
                        (_, _, _, x3) if x3 == 1 => {
                            None
                        }
                        (x0, _, _, x3) if x0 == 0 && x3 == 0 => {
                            None
                        }
                        (x0, _, x2, x3) if x0 == 1 && x2 == 1 && x3 == 0 => {
                            None
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            Some(Op::aarch64_integer_tags_mcaddtag)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 0 => {
                            Some(Op::aarch64_integer_tags_mcsubtag)
                        }
                        _ => None
                    }
                }
                (_, _, x2, _) if x2 == 4 => {
                    let sf = (instr >> 31) & 1;
                    let opc = (instr >> 29) & 3;
                    let N = (instr >> 22) & 1;
                    let immr = (instr >> 16) & 11;
                    let imms = (instr >> 10) & 11;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (sf, opc, N) {
                        (x0, _, x2) if x0 == 0 && x2 == 1 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_integer_logical_immediate)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_integer_logical_immediate)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 0 => {
                            Some(Op::aarch64_integer_logical_immediate)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 0 => {
                            Some(Op::aarch64_integer_logical_immediate)
                        }
                        (x0, x1, _) if x0 == 1 && x1 == 0 => {
                            Some(Op::aarch64_integer_logical_immediate)
                        }
                        (x0, x1, _) if x0 == 1 && x1 == 1 => {
                            Some(Op::aarch64_integer_logical_immediate)
                        }
                        (x0, x1, _) if x0 == 1 && x1 == 2 => {
                            Some(Op::aarch64_integer_logical_immediate)
                        }
                        (x0, x1, _) if x0 == 1 && x1 == 3 => {
                            Some(Op::aarch64_integer_logical_immediate)
                        }
                        _ => None
                    }
                }
                (_, _, x2, _) if x2 == 5 => {
                    let sf = (instr >> 31) & 1;
                    let opc = (instr >> 29) & 3;
                    let hw = (instr >> 21) & 3;
                    let imm16 = (instr >> 5) & 31;
                    let Rd = instr & 9;
                    match (sf, opc, hw) {
                        (_, x1, _) if x1 == 1 => {
                            None
                        }
                        (x0, _, x2) if x0 == 0 && x2 & 2 == 2 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 & 2 == 0 => {
                            Some(Op::aarch64_integer_ins_ext_insert_movewide)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 & 2 == 0 => {
                            Some(Op::aarch64_integer_ins_ext_insert_movewide)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 & 2 == 0 => {
                            Some(Op::aarch64_integer_ins_ext_insert_movewide)
                        }
                        (x0, x1, _) if x0 == 1 && x1 == 0 => {
                            Some(Op::aarch64_integer_ins_ext_insert_movewide)
                        }
                        (x0, x1, _) if x0 == 1 && x1 == 2 => {
                            Some(Op::aarch64_integer_ins_ext_insert_movewide)
                        }
                        (x0, x1, _) if x0 == 1 && x1 == 3 => {
                            Some(Op::aarch64_integer_ins_ext_insert_movewide)
                        }
                        _ => None
                    }
                }
                (_, _, x2, _) if x2 == 6 => {
                    let sf = (instr >> 31) & 1;
                    let opc = (instr >> 29) & 3;
                    let N = (instr >> 22) & 1;
                    let immr = (instr >> 16) & 11;
                    let imms = (instr >> 10) & 11;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (sf, opc, N) {
                        (_, x1, _) if x1 == 3 => {
                            None
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 1 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_integer_bitfield)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_integer_bitfield)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 0 => {
                            Some(Op::aarch64_integer_bitfield)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 0 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_integer_bitfield)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_integer_bitfield)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 == 1 => {
                            Some(Op::aarch64_integer_bitfield)
                        }
                        _ => None
                    }
                }
                (_, _, x2, _) if x2 == 7 => {
                    let sf = (instr >> 31) & 1;
                    let op21 = (instr >> 29) & 3;
                    let N = (instr >> 22) & 1;
                    let o0 = (instr >> 21) & 1;
                    let Rm = (instr >> 16) & 9;
                    let imms = (instr >> 10) & 11;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (sf, op21, N, o0, imms) {
                        (_, x1, _, _, _) if x1 & 1 == 1 => {
                            None
                        }
                        (_, x1, _, x3, _) if x1 == 0 && x3 == 1 => {
                            None
                        }
                        (_, x1, _, _, _) if x1 & 2 == 2 => {
                            None
                        }
                        (x0, _, _, _, x4) if x0 == 0 && x4 & 32 == 32 => {
                            None
                        }
                        (x0, _, x2, _, _) if x0 == 0 && x2 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 & 32 == 0 => {
                            Some(Op::aarch64_integer_ins_ext_extract_immediate)
                        }
                        (x0, _, x2, _, _) if x0 == 1 && x2 == 0 => {
                            None
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            Some(Op::aarch64_integer_ins_ext_extract_immediate)
                        }
                        _ => None
                    }
                }
                _ => None
            }
        }
        (_, x1, _) if x1 & 28 == 20 => {
            match ((instr >> 29) & 5, (instr >> 26) & 5, (instr >> 12) & 27, (instr >> 5) & 13, instr & 9) {
                (x0, _, x2, _, _) if x0 == 2 && x2 & 8192 == 0 => {
                    let o1 = (instr >> 24) & 1;
                    let imm19 = (instr >> 5) & 37;
                    let o0 = (instr >> 4) & 1;
                    let cond = instr & 7;
                    match (o1, o0) {
                        (x0, x1) if x0 == 0 && x1 == 0 => {
                            Some(Op::aarch64_branch_conditional_cond)
                        }
                        (x0, x1) if x0 == 0 && x1 == 1 => {
                            None
                        }
                        (x0, _) if x0 == 1 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, _) if x0 == 6 && x2 & 12288 == 0 => {
                    let opc = (instr >> 21) & 5;
                    let imm16 = (instr >> 5) & 31;
                    let op2 = (instr >> 2) & 5;
                    let LL = instr & 3;
                    match (opc, op2, LL) {
                        (_, x1, _) if x1 == 1 => {
                            None
                        }
                        (_, x1, _) if x1 & 6 == 2 => {
                            None
                        }
                        (_, x1, _) if x1 & 4 == 4 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_system_exceptions_runtime_svc)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 2 => {
                            Some(Op::aarch64_system_exceptions_runtime_hvc)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 3 => {
                            Some(Op::aarch64_system_exceptions_runtime_smc)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 & 1 == 1 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_system_exceptions_debug_breakpoint)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 & 2 == 2 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 & 1 == 1 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_system_exceptions_debug_halt)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 & 2 == 2 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 1 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 & 2 == 2 => {
                            None
                        }
                        (x0, x1, _) if x0 == 4 && x1 == 0 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 5 && x1 == 0 && x2 == 0 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 5 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_system_exceptions_debug_exception)
                        }
                        (x0, x1, x2) if x0 == 5 && x1 == 0 && x2 == 2 => {
                            Some(Op::aarch64_system_exceptions_debug_exception)
                        }
                        (x0, x1, x2) if x0 == 5 && x1 == 0 && x2 == 3 => {
                            Some(Op::aarch64_system_exceptions_debug_exception)
                        }
                        (x0, x1, _) if x0 == 6 && x1 == 0 => {
                            None
                        }
                        (x0, x1, _) if x0 == 7 && x1 == 0 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4) if x0 == 6 && x2 == 4146 && x4 == 31 => {
                    let CRm = (instr >> 8) & 7;
                    let op2 = (instr >> 5) & 5;
                    match (CRm, op2) {
                        (_, _) => {
                            Some(Op::aarch64_system_hints)
                        }
                        (x0, x1) if x0 == 0 && x1 == 0 => {
                            Some(Op::aarch64_system_hints)
                        }
                        (x0, x1) if x0 == 0 && x1 == 1 => {
                            Some(Op::aarch64_system_hints)
                        }
                        (x0, x1) if x0 == 0 && x1 == 2 => {
                            Some(Op::aarch64_system_hints)
                        }
                        (x0, x1) if x0 == 0 && x1 == 3 => {
                            Some(Op::aarch64_system_hints)
                        }
                        (x0, x1) if x0 == 0 && x1 == 4 => {
                            Some(Op::aarch64_system_hints)
                        }
                        (x0, x1) if x0 == 0 && x1 == 5 => {
                            Some(Op::aarch64_system_hints)
                        }
                        (x0, x1) if x0 == 0 && x1 == 6 => {
                            Some(Op::aarch64_system_hints)
                        }
                        (x0, x1) if x0 == 0 && x1 == 7 => {
                            Some(Op::aarch64_integer_pac_strip_hint)
                        }
                        (x0, x1) if x0 == 1 && x1 == 0 => {
                            Some(Op::aarch64_integer_pac_pacia_hint)
                        }
                        (x0, x1) if x0 == 1 && x1 == 2 => {
                            Some(Op::aarch64_integer_pac_pacib_hint)
                        }
                        (x0, x1) if x0 == 1 && x1 == 4 => {
                            Some(Op::aarch64_integer_pac_autia_hint)
                        }
                        (x0, x1) if x0 == 1 && x1 == 6 => {
                            Some(Op::aarch64_integer_pac_autib_hint)
                        }
                        (x0, x1) if x0 == 2 && x1 == 0 => {
                            Some(Op::aarch64_system_hints)
                        }
                        (x0, x1) if x0 == 2 && x1 == 1 => {
                            Some(Op::aarch64_system_hints)
                        }
                        (x0, x1) if x0 == 2 && x1 == 2 => {
                            Some(Op::aarch64_system_hints)
                        }
                        (x0, x1) if x0 == 2 && x1 == 4 => {
                            Some(Op::aarch64_system_hints)
                        }
                        (x0, x1) if x0 == 3 && x1 == 0 => {
                            Some(Op::aarch64_integer_pac_pacia_hint)
                        }
                        (x0, x1) if x0 == 3 && x1 == 1 => {
                            Some(Op::aarch64_integer_pac_pacia_hint)
                        }
                        (x0, x1) if x0 == 3 && x1 == 2 => {
                            Some(Op::aarch64_integer_pac_pacib_hint)
                        }
                        (x0, x1) if x0 == 3 && x1 == 3 => {
                            Some(Op::aarch64_integer_pac_pacib_hint)
                        }
                        (x0, x1) if x0 == 3 && x1 == 4 => {
                            Some(Op::aarch64_integer_pac_autia_hint)
                        }
                        (x0, x1) if x0 == 3 && x1 == 5 => {
                            Some(Op::aarch64_integer_pac_autia_hint)
                        }
                        (x0, x1) if x0 == 3 && x1 == 6 => {
                            Some(Op::aarch64_integer_pac_autib_hint)
                        }
                        (x0, x1) if x0 == 3 && x1 == 7 => {
                            Some(Op::aarch64_integer_pac_autib_hint)
                        }
                        (x0, x1) if x0 == 4 && x1 & 1 == 0 => {
                            Some(Op::aarch64_system_hints)
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, _) if x0 == 6 && x2 == 4147 => {
                    let CRm = (instr >> 8) & 7;
                    let op2 = (instr >> 5) & 5;
                    let Rt = instr & 9;
                    match (CRm, op2, Rt) {
                        (_, x1, _) if x1 == 0 => {
                            None
                        }
                        (_, x1, _) if x1 == 1 => {
                            None
                        }
                        (_, x1, x2) if x1 == 2 && x2 == 31 => {
                            Some(Op::aarch64_system_monitors)
                        }
                        (_, x1, x2) if x1 == 5 && x2 == 31 => {
                            Some(Op::aarch64_system_barriers_dmb)
                        }
                        (_, x1, x2) if x1 == 6 && x2 == 31 => {
                            Some(Op::aarch64_system_barriers_isb)
                        }
                        (_, x1, x2) if x1 == 7 && x2 != 31 => {
                            None
                        }
                        (_, x1, x2) if x1 == 7 && x2 == 31 => {
                            Some(Op::aarch64_system_barriers_sb)
                        }
                        (x0, x1, x2) if x0 & 11 != 0 && x1 == 4 && x2 == 31 => {
                            Some(Op::aarch64_system_barriers_dsb)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 4 && x2 == 31 => {
                            Some(Op::aarch64_system_barriers_ssbb)
                        }
                        (x0, x1, _) if x0 == 1 && x1 == 3 => {
                            None
                        }
                        (x0, x1, _) if x0 & 14 == 2 && x1 == 3 => {
                            None
                        }
                        (x0, x1, _) if x0 & 12 == 4 && x1 == 3 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 4 && x1 == 4 && x2 == 31 => {
                            Some(Op::aarch64_system_barriers_pssbb)
                        }
                        (x0, x1, _) if x0 & 8 == 8 && x1 == 3 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, _) if x0 == 6 && x2 & 16271 == 4100 => {
                    let op1 = (instr >> 16) & 5;
                    let CRm = (instr >> 8) & 7;
                    let op2 = (instr >> 5) & 5;
                    let Rt = instr & 9;
                    match (op1, op2, Rt) {
                        (_, _, x2) if x2 != 31 => {
                            None
                        }
                        (_, _, x2) if x2 == 31 => {
                            Some(Op::aarch64_system_register_cpsr)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 31 => {
                            Some(Op::aarch64_integer_flags_cfinv)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 31 => {
                            Some(Op::aarch64_integer_flags_xaflag)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 31 => {
                            Some(Op::aarch64_integer_flags_axflag)
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, _) if x0 == 6 && x2 & 15744 == 4224 => {
                    let L = (instr >> 21) & 1;
                    let op1 = (instr >> 16) & 5;
                    let CRn = (instr >> 12) & 7;
                    let CRm = (instr >> 8) & 7;
                    let op2 = (instr >> 5) & 5;
                    let Rt = instr & 9;
                    match L {
                        x0 if x0 == 0 => {
                            Some(Op::aarch64_system_sysops)
                        }
                        x0 if x0 == 1 => {
                            Some(Op::aarch64_system_sysops)
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, _) if x0 == 6 && x2 & 15616 == 4352 => {
                    let L = (instr >> 21) & 1;
                    let o0 = (instr >> 19) & 1;
                    let op1 = (instr >> 16) & 5;
                    let CRn = (instr >> 12) & 7;
                    let CRm = (instr >> 8) & 7;
                    let op2 = (instr >> 5) & 5;
                    let Rt = instr & 9;
                    match L {
                        x0 if x0 == 0 => {
                            Some(Op::aarch64_system_register_system)
                        }
                        x0 if x0 == 1 => {
                            Some(Op::aarch64_system_register_system)
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, _) if x0 == 6 && x2 & 8192 == 8192 => {
                    let opc = (instr >> 21) & 7;
                    let op2 = (instr >> 16) & 9;
                    let op3 = (instr >> 10) & 11;
                    let Rn = (instr >> 5) & 9;
                    let op4 = instr & 9;
                    match (opc, op2, op3, Rn, op4) {
                        (_, x1, _, _, _) if x1 != 31 => {
                            None
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 31 && x2 == 0 && x4 != 0 => {
                            None
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 31 && x2 == 0 && x4 == 0 => {
                            Some(Op::aarch64_branch_unconditional_register)
                        }
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 31 && x2 == 1 => {
                            None
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 31 && x2 == 2 && x4 != 31 => {
                            None
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 31 && x2 == 2 && x4 == 31 => {
                            Some(Op::aarch64_branch_unconditional_register)
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 31 && x2 == 3 && x4 != 31 => {
                            None
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 31 && x2 == 3 && x4 == 31 => {
                            Some(Op::aarch64_branch_unconditional_register)
                        }
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 31 && x2 & 60 == 4 => {
                            None
                        }
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 31 && x2 & 56 == 8 => {
                            None
                        }
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 31 && x2 & 48 == 16 => {
                            None
                        }
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 31 && x2 & 32 == 32 => {
                            None
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 31 && x2 == 0 && x4 != 0 => {
                            None
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 31 && x2 == 0 && x4 == 0 => {
                            Some(Op::aarch64_branch_unconditional_register)
                        }
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 31 && x2 == 1 => {
                            None
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 31 && x2 == 2 && x4 != 31 => {
                            None
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 31 && x2 == 2 && x4 == 31 => {
                            Some(Op::aarch64_branch_unconditional_register)
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 31 && x2 == 3 && x4 != 31 => {
                            None
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 31 && x2 == 3 && x4 == 31 => {
                            Some(Op::aarch64_branch_unconditional_register)
                        }
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 31 && x2 & 60 == 4 => {
                            None
                        }
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 31 && x2 & 56 == 8 => {
                            None
                        }
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 31 && x2 & 48 == 16 => {
                            None
                        }
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 31 && x2 & 32 == 32 => {
                            None
                        }
                        (x0, x1, x2, _, x4) if x0 == 2 && x1 == 31 && x2 == 0 && x4 != 0 => {
                            None
                        }
                        (x0, x1, x2, _, x4) if x0 == 2 && x1 == 31 && x2 == 0 && x4 == 0 => {
                            Some(Op::aarch64_branch_unconditional_register)
                        }
                        (x0, x1, x2, _, _) if x0 == 2 && x1 == 31 && x2 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 2 && x1 == 31 && x2 == 2 && x3 != 31 && x4 != 31 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 2 && x1 == 31 && x2 == 2 && x3 == 31 && x4 == 31 => {
                            Some(Op::aarch64_branch_unconditional_register)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 2 && x1 == 31 && x2 == 3 && x3 != 31 && x4 != 31 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 2 && x1 == 31 && x2 == 3 && x3 == 31 && x4 == 31 => {
                            Some(Op::aarch64_branch_unconditional_register)
                        }
                        (x0, x1, x2, _, _) if x0 == 2 && x1 == 31 && x2 & 60 == 4 => {
                            None
                        }
                        (x0, x1, x2, _, _) if x0 == 2 && x1 == 31 && x2 & 56 == 8 => {
                            None
                        }
                        (x0, x1, x2, _, _) if x0 == 2 && x1 == 31 && x2 & 48 == 16 => {
                            None
                        }
                        (x0, x1, x2, _, _) if x0 == 2 && x1 == 31 && x2 & 32 == 32 => {
                            None
                        }
                        (x0, x1, _, _, _) if x0 == 3 && x1 == 31 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 4 && x1 == 31 && x2 == 0 && x3 != 31 && x4 != 0 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 4 && x1 == 31 && x2 == 0 && x3 != 31 && x4 == 0 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 4 && x1 == 31 && x2 == 0 && x3 == 31 && x4 != 0 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 4 && x1 == 31 && x2 == 0 && x3 == 31 && x4 == 0 => {
                            Some(Op::aarch64_branch_unconditional_eret)
                        }
                        (x0, x1, x2, _, _) if x0 == 4 && x1 == 31 && x2 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 4 && x1 == 31 && x2 == 2 && x3 != 31 && x4 != 31 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 4 && x1 == 31 && x2 == 2 && x3 != 31 && x4 == 31 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 4 && x1 == 31 && x2 == 2 && x3 == 31 && x4 != 31 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 4 && x1 == 31 && x2 == 2 && x3 == 31 && x4 == 31 => {
                            Some(Op::aarch64_branch_unconditional_eret)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 4 && x1 == 31 && x2 == 3 && x3 != 31 && x4 != 31 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 4 && x1 == 31 && x2 == 3 && x3 != 31 && x4 == 31 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 4 && x1 == 31 && x2 == 3 && x3 == 31 && x4 != 31 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 4 && x1 == 31 && x2 == 3 && x3 == 31 && x4 == 31 => {
                            Some(Op::aarch64_branch_unconditional_eret)
                        }
                        (x0, x1, x2, _, _) if x0 == 4 && x1 == 31 && x2 & 60 == 4 => {
                            None
                        }
                        (x0, x1, x2, _, _) if x0 == 4 && x1 == 31 && x2 & 56 == 8 => {
                            None
                        }
                        (x0, x1, x2, _, _) if x0 == 4 && x1 == 31 && x2 & 48 == 16 => {
                            None
                        }
                        (x0, x1, x2, _, _) if x0 == 4 && x1 == 31 && x2 & 32 == 32 => {
                            None
                        }
                        (x0, x1, x2, _, _) if x0 == 5 && x1 == 31 && x2 != 0 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 5 && x1 == 31 && x2 == 0 && x3 != 31 && x4 != 0 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 5 && x1 == 31 && x2 == 0 && x3 != 31 && x4 == 0 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 5 && x1 == 31 && x2 == 0 && x3 == 31 && x4 != 0 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 5 && x1 == 31 && x2 == 0 && x3 == 31 && x4 == 0 => {
                            Some(Op::aarch64_branch_unconditional_dret)
                        }
                        (x0, x1, _, _, _) if x0 & 14 == 6 && x1 == 31 => {
                            None
                        }
                        (x0, x1, x2, _, _) if x0 == 8 && x1 == 31 && x2 & 62 == 0 => {
                            None
                        }
                        (x0, x1, x2, _, _) if x0 == 8 && x1 == 31 && x2 == 2 => {
                            Some(Op::aarch64_branch_unconditional_register)
                        }
                        (x0, x1, x2, _, _) if x0 == 8 && x1 == 31 && x2 == 3 => {
                            Some(Op::aarch64_branch_unconditional_register)
                        }
                        (x0, x1, x2, _, _) if x0 == 8 && x1 == 31 && x2 & 60 == 4 => {
                            None
                        }
                        (x0, x1, x2, _, _) if x0 == 8 && x1 == 31 && x2 & 56 == 8 => {
                            None
                        }
                        (x0, x1, x2, _, _) if x0 == 8 && x1 == 31 && x2 & 48 == 16 => {
                            None
                        }
                        (x0, x1, x2, _, _) if x0 == 8 && x1 == 31 && x2 & 32 == 32 => {
                            None
                        }
                        (x0, x1, x2, _, _) if x0 == 9 && x1 == 31 && x2 & 62 == 0 => {
                            None
                        }
                        (x0, x1, x2, _, _) if x0 == 9 && x1 == 31 && x2 == 2 => {
                            Some(Op::aarch64_branch_unconditional_register)
                        }
                        (x0, x1, x2, _, _) if x0 == 9 && x1 == 31 && x2 == 3 => {
                            Some(Op::aarch64_branch_unconditional_register)
                        }
                        (x0, x1, x2, _, _) if x0 == 9 && x1 == 31 && x2 & 60 == 4 => {
                            None
                        }
                        (x0, x1, x2, _, _) if x0 == 9 && x1 == 31 && x2 & 56 == 8 => {
                            None
                        }
                        (x0, x1, x2, _, _) if x0 == 9 && x1 == 31 && x2 & 48 == 16 => {
                            None
                        }
                        (x0, x1, x2, _, _) if x0 == 9 && x1 == 31 && x2 & 32 == 32 => {
                            None
                        }
                        (x0, x1, _, _, _) if x0 & 14 == 10 && x1 == 31 => {
                            None
                        }
                        (x0, x1, _, _, _) if x0 & 12 == 12 && x1 == 31 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, _, _, _) if x0 & 3 == 0 => {
                    let op = (instr >> 31) & 1;
                    let imm26 = instr & 51;
                    match op {
                        x0 if x0 == 0 => {
                            Some(Op::aarch64_branch_unconditional_immediate)
                        }
                        x0 if x0 == 1 => {
                            Some(Op::aarch64_branch_unconditional_immediate)
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, _) if x0 & 3 == 1 && x2 & 8192 == 0 => {
                    let sf = (instr >> 31) & 1;
                    let op = (instr >> 24) & 1;
                    let imm19 = (instr >> 5) & 37;
                    let Rt = instr & 9;
                    match (sf, op) {
                        (x0, x1) if x0 == 0 && x1 == 0 => {
                            Some(Op::aarch64_branch_conditional_compare)
                        }
                        (x0, x1) if x0 == 0 && x1 == 1 => {
                            Some(Op::aarch64_branch_conditional_compare)
                        }
                        (x0, x1) if x0 == 1 && x1 == 0 => {
                            Some(Op::aarch64_branch_conditional_compare)
                        }
                        (x0, x1) if x0 == 1 && x1 == 1 => {
                            Some(Op::aarch64_branch_conditional_compare)
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, _) if x0 & 3 == 1 && x2 & 8192 == 8192 => {
                    let b5 = (instr >> 31) & 1;
                    let op = (instr >> 24) & 1;
                    let b40 = (instr >> 19) & 9;
                    let imm14 = (instr >> 5) & 27;
                    let Rt = instr & 9;
                    match op {
                        x0 if x0 == 0 => {
                            Some(Op::aarch64_branch_conditional_test)
                        }
                        x0 if x0 == 1 => {
                            Some(Op::aarch64_branch_conditional_test)
                        }
                        _ => None
                    }
                }
                _ => None
            }
        }
        (_, x1, _) if x1 & 10 == 8 => {
            match ((instr >> 28) & 7, (instr >> 27) & 1, (instr >> 26) & 1, (instr >> 25) & 1, (instr >> 23) & 3, (instr >> 22) & 1, (instr >> 16) & 11, (instr >> 12) & 7, (instr >> 10) & 3, instr & 19) {
                (x0, _, x2, _, x4, _, x6, _, _, _) if x0 & 11 == 0 && x2 == 1 && x4 == 0 && x6 == 0 => {
                    let Q = (instr >> 30) & 1;
                    let L = (instr >> 22) & 1;
                    let opcode = (instr >> 12) & 7;
                    let size = (instr >> 10) & 3;
                    let Rn = (instr >> 5) & 9;
                    let Rt = instr & 9;
                    match (L, opcode) {
                        (x0, x1) if x0 == 0 && x1 == 0 => {
                            Some(Op::aarch64_memory_vector_multiple_no_wb)
                        }
                        (x0, x1) if x0 == 0 && x1 == 1 => {
                            None
                        }
                        (x0, x1) if x0 == 0 && x1 == 2 => {
                            Some(Op::aarch64_memory_vector_multiple_no_wb)
                        }
                        (x0, x1) if x0 == 0 && x1 == 3 => {
                            None
                        }
                        (x0, x1) if x0 == 0 && x1 == 4 => {
                            Some(Op::aarch64_memory_vector_multiple_no_wb)
                        }
                        (x0, x1) if x0 == 0 && x1 == 5 => {
                            None
                        }
                        (x0, x1) if x0 == 0 && x1 == 6 => {
                            Some(Op::aarch64_memory_vector_multiple_no_wb)
                        }
                        (x0, x1) if x0 == 0 && x1 == 7 => {
                            Some(Op::aarch64_memory_vector_multiple_no_wb)
                        }
                        (x0, x1) if x0 == 0 && x1 == 8 => {
                            Some(Op::aarch64_memory_vector_multiple_no_wb)
                        }
                        (x0, x1) if x0 == 0 && x1 == 9 => {
                            None
                        }
                        (x0, x1) if x0 == 0 && x1 == 10 => {
                            Some(Op::aarch64_memory_vector_multiple_no_wb)
                        }
                        (x0, x1) if x0 == 0 && x1 == 11 => {
                            None
                        }
                        (x0, x1) if x0 == 0 && x1 & 12 == 12 => {
                            None
                        }
                        (x0, x1) if x0 == 1 && x1 == 0 => {
                            Some(Op::aarch64_memory_vector_multiple_no_wb)
                        }
                        (x0, x1) if x0 == 1 && x1 == 1 => {
                            None
                        }
                        (x0, x1) if x0 == 1 && x1 == 2 => {
                            Some(Op::aarch64_memory_vector_multiple_no_wb)
                        }
                        (x0, x1) if x0 == 1 && x1 == 3 => {
                            None
                        }
                        (x0, x1) if x0 == 1 && x1 == 4 => {
                            Some(Op::aarch64_memory_vector_multiple_no_wb)
                        }
                        (x0, x1) if x0 == 1 && x1 == 5 => {
                            None
                        }
                        (x0, x1) if x0 == 1 && x1 == 6 => {
                            Some(Op::aarch64_memory_vector_multiple_no_wb)
                        }
                        (x0, x1) if x0 == 1 && x1 == 7 => {
                            Some(Op::aarch64_memory_vector_multiple_no_wb)
                        }
                        (x0, x1) if x0 == 1 && x1 == 8 => {
                            Some(Op::aarch64_memory_vector_multiple_no_wb)
                        }
                        (x0, x1) if x0 == 1 && x1 == 9 => {
                            None
                        }
                        (x0, x1) if x0 == 1 && x1 == 10 => {
                            Some(Op::aarch64_memory_vector_multiple_no_wb)
                        }
                        (x0, x1) if x0 == 1 && x1 == 11 => {
                            None
                        }
                        (x0, x1) if x0 == 1 && x1 & 12 == 12 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _, _, _) if x0 & 11 == 0 && x2 == 1 && x4 == 1 && x6 & 32 == 0 => {
                    let Q = (instr >> 30) & 1;
                    let L = (instr >> 22) & 1;
                    let Rm = (instr >> 16) & 9;
                    let opcode = (instr >> 12) & 7;
                    let size = (instr >> 10) & 3;
                    let Rn = (instr >> 5) & 9;
                    let Rt = instr & 9;
                    match (L, Rm, opcode) {
                        (x0, _, x2) if x0 == 0 && x2 == 1 => {
                            None
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 3 => {
                            None
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 5 => {
                            None
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 9 => {
                            None
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 11 => {
                            None
                        }
                        (x0, _, x2) if x0 == 0 && x2 & 12 == 12 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 31 && x2 == 0 => {
                            Some(Op::aarch64_memory_vector_multiple_post_inc)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 31 && x2 == 2 => {
                            Some(Op::aarch64_memory_vector_multiple_post_inc)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 31 && x2 == 4 => {
                            Some(Op::aarch64_memory_vector_multiple_post_inc)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 31 && x2 == 6 => {
                            Some(Op::aarch64_memory_vector_multiple_post_inc)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 31 && x2 == 7 => {
                            Some(Op::aarch64_memory_vector_multiple_post_inc)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 31 && x2 == 8 => {
                            Some(Op::aarch64_memory_vector_multiple_post_inc)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 31 && x2 == 10 => {
                            Some(Op::aarch64_memory_vector_multiple_post_inc)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 31 && x2 == 0 => {
                            Some(Op::aarch64_memory_vector_multiple_post_inc)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 31 && x2 == 2 => {
                            Some(Op::aarch64_memory_vector_multiple_post_inc)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 31 && x2 == 4 => {
                            Some(Op::aarch64_memory_vector_multiple_post_inc)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 31 && x2 == 6 => {
                            Some(Op::aarch64_memory_vector_multiple_post_inc)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 31 && x2 == 7 => {
                            Some(Op::aarch64_memory_vector_multiple_post_inc)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 31 && x2 == 8 => {
                            Some(Op::aarch64_memory_vector_multiple_post_inc)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 31 && x2 == 10 => {
                            Some(Op::aarch64_memory_vector_multiple_post_inc)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 1 => {
                            None
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 3 => {
                            None
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 5 => {
                            None
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 9 => {
                            None
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 11 => {
                            None
                        }
                        (x0, _, x2) if x0 == 1 && x2 & 12 == 12 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 31 && x2 == 0 => {
                            Some(Op::aarch64_memory_vector_multiple_post_inc)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 31 && x2 == 2 => {
                            Some(Op::aarch64_memory_vector_multiple_post_inc)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 31 && x2 == 4 => {
                            Some(Op::aarch64_memory_vector_multiple_post_inc)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 31 && x2 == 6 => {
                            Some(Op::aarch64_memory_vector_multiple_post_inc)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 31 && x2 == 7 => {
                            Some(Op::aarch64_memory_vector_multiple_post_inc)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 31 && x2 == 8 => {
                            Some(Op::aarch64_memory_vector_multiple_post_inc)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 31 && x2 == 10 => {
                            Some(Op::aarch64_memory_vector_multiple_post_inc)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 31 && x2 == 0 => {
                            Some(Op::aarch64_memory_vector_multiple_post_inc)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 31 && x2 == 2 => {
                            Some(Op::aarch64_memory_vector_multiple_post_inc)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 31 && x2 == 4 => {
                            Some(Op::aarch64_memory_vector_multiple_post_inc)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 31 && x2 == 6 => {
                            Some(Op::aarch64_memory_vector_multiple_post_inc)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 31 && x2 == 7 => {
                            Some(Op::aarch64_memory_vector_multiple_post_inc)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 31 && x2 == 8 => {
                            Some(Op::aarch64_memory_vector_multiple_post_inc)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 31 && x2 == 10 => {
                            Some(Op::aarch64_memory_vector_multiple_post_inc)
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _, _, _) if x0 & 11 == 0 && x2 == 1 && x4 & 2 == 0 && x6 & 32 == 32 => {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _, _, _) if x0 & 11 == 0 && x2 == 1 && x4 == 2 && x6 & 31 == 0 => {
                    let Q = (instr >> 30) & 1;
                    let L = (instr >> 22) & 1;
                    let R = (instr >> 21) & 1;
                    let opcode = (instr >> 13) & 5;
                    let S = (instr >> 12) & 1;
                    let size = (instr >> 10) & 3;
                    let Rn = (instr >> 5) & 9;
                    let Rt = instr & 9;
                    match (L, R, opcode, S, size) {
                        (x0, _, x2, _, _) if x0 == 0 && x2 & 6 == 6 => {
                            None
                        }
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_memory_vector_single_no_wb)
                        }
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_memory_vector_single_no_wb)
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 0 && x2 == 2 && x4 & 1 == 0 => {
                            Some(Op::aarch64_memory_vector_single_no_wb)
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 0 && x2 == 2 && x4 & 1 == 1 => {
                            None
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x4 & 1 == 0 => {
                            Some(Op::aarch64_memory_vector_single_no_wb)
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x4 & 1 == 1 => {
                            None
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 0 && x2 == 4 && x4 == 0 => {
                            Some(Op::aarch64_memory_vector_single_no_wb)
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 0 && x2 == 4 && x4 & 2 == 2 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 4 && x3 == 0 && x4 == 1 => {
                            Some(Op::aarch64_memory_vector_single_no_wb)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 4 && x3 == 1 && x4 == 1 => {
                            None
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 0 && x2 == 5 && x4 == 0 => {
                            Some(Op::aarch64_memory_vector_single_no_wb)
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 0 && x2 == 5 && x4 == 2 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 5 && x3 == 0 && x4 == 1 => {
                            Some(Op::aarch64_memory_vector_single_no_wb)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 5 && x3 == 0 && x4 == 3 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 5 && x3 == 1 && x4 & 1 == 1 => {
                            None
                        }
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_memory_vector_single_no_wb)
                        }
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_memory_vector_single_no_wb)
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 1 && x2 == 2 && x4 & 1 == 0 => {
                            Some(Op::aarch64_memory_vector_single_no_wb)
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 1 && x2 == 2 && x4 & 1 == 1 => {
                            None
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 1 && x2 == 3 && x4 & 1 == 0 => {
                            Some(Op::aarch64_memory_vector_single_no_wb)
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 1 && x2 == 3 && x4 & 1 == 1 => {
                            None
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 1 && x2 == 4 && x4 == 0 => {
                            Some(Op::aarch64_memory_vector_single_no_wb)
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 1 && x2 == 4 && x4 == 2 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 1 && x2 == 4 && x3 == 0 && x4 == 1 => {
                            Some(Op::aarch64_memory_vector_single_no_wb)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 1 && x2 == 4 && x3 == 0 && x4 == 3 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 1 && x2 == 4 && x3 == 1 && x4 & 1 == 1 => {
                            None
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 1 && x2 == 5 && x4 == 0 => {
                            Some(Op::aarch64_memory_vector_single_no_wb)
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 1 && x2 == 5 && x4 == 2 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 1 && x2 == 5 && x3 == 0 && x4 == 1 => {
                            Some(Op::aarch64_memory_vector_single_no_wb)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 1 && x2 == 5 && x3 == 0 && x4 == 3 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 1 && x2 == 5 && x3 == 1 && x4 & 1 == 1 => {
                            None
                        }
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_memory_vector_single_no_wb)
                        }
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_memory_vector_single_no_wb)
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 0 && x2 == 2 && x4 & 1 == 0 => {
                            Some(Op::aarch64_memory_vector_single_no_wb)
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 0 && x2 == 2 && x4 & 1 == 1 => {
                            None
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 0 && x2 == 3 && x4 & 1 == 0 => {
                            Some(Op::aarch64_memory_vector_single_no_wb)
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 0 && x2 == 3 && x4 & 1 == 1 => {
                            None
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 0 && x2 == 4 && x4 == 0 => {
                            Some(Op::aarch64_memory_vector_single_no_wb)
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 0 && x2 == 4 && x4 & 2 == 2 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 4 && x3 == 0 && x4 == 1 => {
                            Some(Op::aarch64_memory_vector_single_no_wb)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 4 && x3 == 1 && x4 == 1 => {
                            None
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 0 && x2 == 5 && x4 == 0 => {
                            Some(Op::aarch64_memory_vector_single_no_wb)
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 0 && x2 == 5 && x4 == 2 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 5 && x3 == 0 && x4 == 1 => {
                            Some(Op::aarch64_memory_vector_single_no_wb)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 5 && x3 == 0 && x4 == 3 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 5 && x3 == 1 && x4 & 1 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 6 && x3 == 0 => {
                            Some(Op::aarch64_memory_vector_single_no_wb)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 6 && x3 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 7 && x3 == 0 => {
                            Some(Op::aarch64_memory_vector_single_no_wb)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 7 && x3 == 1 => {
                            None
                        }
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_memory_vector_single_no_wb)
                        }
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_memory_vector_single_no_wb)
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 1 && x2 == 2 && x4 & 1 == 0 => {
                            Some(Op::aarch64_memory_vector_single_no_wb)
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 1 && x2 == 2 && x4 & 1 == 1 => {
                            None
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 1 && x2 == 3 && x4 & 1 == 0 => {
                            Some(Op::aarch64_memory_vector_single_no_wb)
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 1 && x2 == 3 && x4 & 1 == 1 => {
                            None
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 1 && x2 == 4 && x4 == 0 => {
                            Some(Op::aarch64_memory_vector_single_no_wb)
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 1 && x2 == 4 && x4 == 2 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 1 && x2 == 4 && x3 == 0 && x4 == 1 => {
                            Some(Op::aarch64_memory_vector_single_no_wb)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 1 && x2 == 4 && x3 == 0 && x4 == 3 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 1 && x2 == 4 && x3 == 1 && x4 & 1 == 1 => {
                            None
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 1 && x2 == 5 && x4 == 0 => {
                            Some(Op::aarch64_memory_vector_single_no_wb)
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 1 && x2 == 5 && x4 == 2 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 1 && x2 == 5 && x3 == 0 && x4 == 1 => {
                            Some(Op::aarch64_memory_vector_single_no_wb)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 1 && x2 == 5 && x3 == 0 && x4 == 3 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 1 && x2 == 5 && x3 == 1 && x4 & 1 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 1 && x2 == 6 && x3 == 0 => {
                            Some(Op::aarch64_memory_vector_single_no_wb)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 1 && x2 == 6 && x3 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 1 && x2 == 7 && x3 == 0 => {
                            Some(Op::aarch64_memory_vector_single_no_wb)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 1 && x2 == 7 && x3 == 1 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, _, _, _, _) if x0 & 11 == 0 && x2 == 1 && x4 == 3 => {
                    let Q = (instr >> 30) & 1;
                    let L = (instr >> 22) & 1;
                    let R = (instr >> 21) & 1;
                    let Rm = (instr >> 16) & 9;
                    let opcode = (instr >> 13) & 5;
                    let S = (instr >> 12) & 1;
                    let size = (instr >> 10) & 3;
                    let Rn = (instr >> 5) & 9;
                    let Rt = instr & 9;
                    match (L, R, Rm, opcode, S, size) {
                        (x0, _, _, x3, _, _) if x0 == 0 && x3 & 6 == 6 => {
                            None
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 0 && x1 == 0 && x3 == 2 && x5 & 1 == 1 => {
                            None
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 0 && x1 == 0 && x3 == 3 && x5 & 1 == 1 => {
                            None
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 0 && x1 == 0 && x3 == 4 && x5 & 2 == 2 => {
                            None
                        }
                        (x0, x1, _, x3, x4, x5) if x0 == 0 && x1 == 0 && x3 == 4 && x4 == 1 && x5 == 1 => {
                            None
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 0 && x1 == 0 && x3 == 5 && x5 == 2 => {
                            None
                        }
                        (x0, x1, _, x3, x4, x5) if x0 == 0 && x1 == 0 && x3 == 5 && x4 == 0 && x5 == 3 => {
                            None
                        }
                        (x0, x1, _, x3, x4, x5) if x0 == 0 && x1 == 0 && x3 == 5 && x4 == 1 && x5 & 1 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 0 && x2 != 31 && x3 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 0 && x2 != 31 && x3 == 1 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 0 && x2 != 31 && x3 == 2 && x5 & 1 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 0 && x2 != 31 && x3 == 3 && x5 & 1 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 0 && x2 != 31 && x3 == 4 && x5 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 != 31 && x3 == 4 && x4 == 0 && x5 == 1 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 0 && x2 != 31 && x3 == 5 && x5 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 != 31 && x3 == 5 && x4 == 0 && x5 == 1 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 0 && x2 == 31 && x3 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 0 && x2 == 31 && x3 == 1 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 0 && x2 == 31 && x3 == 2 && x5 & 1 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 0 && x2 == 31 && x3 == 3 && x5 & 1 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 0 && x2 == 31 && x3 == 4 && x5 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 31 && x3 == 4 && x4 == 0 && x5 == 1 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 0 && x2 == 31 && x3 == 5 && x5 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 31 && x3 == 5 && x4 == 0 && x5 == 1 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 0 && x1 == 1 && x3 == 2 && x5 & 1 == 1 => {
                            None
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 0 && x1 == 1 && x3 == 3 && x5 & 1 == 1 => {
                            None
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 0 && x1 == 1 && x3 == 4 && x5 == 2 => {
                            None
                        }
                        (x0, x1, _, x3, x4, x5) if x0 == 0 && x1 == 1 && x3 == 4 && x4 == 0 && x5 == 3 => {
                            None
                        }
                        (x0, x1, _, x3, x4, x5) if x0 == 0 && x1 == 1 && x3 == 4 && x4 == 1 && x5 & 1 == 1 => {
                            None
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 0 && x1 == 1 && x3 == 5 && x5 == 2 => {
                            None
                        }
                        (x0, x1, _, x3, x4, x5) if x0 == 0 && x1 == 1 && x3 == 5 && x4 == 0 && x5 == 3 => {
                            None
                        }
                        (x0, x1, _, x3, x4, x5) if x0 == 0 && x1 == 1 && x3 == 5 && x4 == 1 && x5 & 1 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 1 && x2 != 31 && x3 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 1 && x2 != 31 && x3 == 1 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 1 && x2 != 31 && x3 == 2 && x5 & 1 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 1 && x2 != 31 && x3 == 3 && x5 & 1 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 1 && x2 != 31 && x3 == 4 && x5 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 1 && x2 != 31 && x3 == 4 && x4 == 0 && x5 == 1 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 1 && x2 != 31 && x3 == 5 && x5 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 1 && x2 != 31 && x3 == 5 && x4 == 0 && x5 == 1 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 1 && x2 == 31 && x3 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 1 && x2 == 31 && x3 == 1 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 1 && x2 == 31 && x3 == 2 && x5 & 1 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 1 && x2 == 31 && x3 == 3 && x5 & 1 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 1 && x2 == 31 && x3 == 4 && x5 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 1 && x2 == 31 && x3 == 4 && x4 == 0 && x5 == 1 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 1 && x2 == 31 && x3 == 5 && x5 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 1 && x2 == 31 && x3 == 5 && x4 == 0 && x5 == 1 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 1 && x1 == 0 && x3 == 2 && x5 & 1 == 1 => {
                            None
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 1 && x1 == 0 && x3 == 3 && x5 & 1 == 1 => {
                            None
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 1 && x1 == 0 && x3 == 4 && x5 & 2 == 2 => {
                            None
                        }
                        (x0, x1, _, x3, x4, x5) if x0 == 1 && x1 == 0 && x3 == 4 && x4 == 1 && x5 == 1 => {
                            None
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 1 && x1 == 0 && x3 == 5 && x5 == 2 => {
                            None
                        }
                        (x0, x1, _, x3, x4, x5) if x0 == 1 && x1 == 0 && x3 == 5 && x4 == 0 && x5 == 3 => {
                            None
                        }
                        (x0, x1, _, x3, x4, x5) if x0 == 1 && x1 == 0 && x3 == 5 && x4 == 1 && x5 & 1 == 1 => {
                            None
                        }
                        (x0, x1, _, x3, x4, _) if x0 == 1 && x1 == 0 && x3 == 6 && x4 == 1 => {
                            None
                        }
                        (x0, x1, _, x3, x4, _) if x0 == 1 && x1 == 0 && x3 == 7 && x4 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 1 && x1 == 0 && x2 != 31 && x3 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 1 && x1 == 0 && x2 != 31 && x3 == 1 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 1 && x1 == 0 && x2 != 31 && x3 == 2 && x5 & 1 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 1 && x1 == 0 && x2 != 31 && x3 == 3 && x5 & 1 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 1 && x1 == 0 && x2 != 31 && x3 == 4 && x5 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 != 31 && x3 == 4 && x4 == 0 && x5 == 1 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 1 && x1 == 0 && x2 != 31 && x3 == 5 && x5 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 != 31 && x3 == 5 && x4 == 0 && x5 == 1 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 != 31 && x3 == 6 && x4 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 != 31 && x3 == 7 && x4 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 1 && x1 == 0 && x2 == 31 && x3 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 1 && x1 == 0 && x2 == 31 && x3 == 1 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 1 && x1 == 0 && x2 == 31 && x3 == 2 && x5 & 1 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 1 && x1 == 0 && x2 == 31 && x3 == 3 && x5 & 1 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 1 && x1 == 0 && x2 == 31 && x3 == 4 && x5 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 31 && x3 == 4 && x4 == 0 && x5 == 1 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 1 && x1 == 0 && x2 == 31 && x3 == 5 && x5 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 31 && x3 == 5 && x4 == 0 && x5 == 1 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 == 31 && x3 == 6 && x4 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 == 31 && x3 == 7 && x4 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 1 && x1 == 1 && x3 == 2 && x5 & 1 == 1 => {
                            None
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 1 && x1 == 1 && x3 == 3 && x5 & 1 == 1 => {
                            None
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 1 && x1 == 1 && x3 == 4 && x5 == 2 => {
                            None
                        }
                        (x0, x1, _, x3, x4, x5) if x0 == 1 && x1 == 1 && x3 == 4 && x4 == 0 && x5 == 3 => {
                            None
                        }
                        (x0, x1, _, x3, x4, x5) if x0 == 1 && x1 == 1 && x3 == 4 && x4 == 1 && x5 & 1 == 1 => {
                            None
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 1 && x1 == 1 && x3 == 5 && x5 == 2 => {
                            None
                        }
                        (x0, x1, _, x3, x4, x5) if x0 == 1 && x1 == 1 && x3 == 5 && x4 == 0 && x5 == 3 => {
                            None
                        }
                        (x0, x1, _, x3, x4, x5) if x0 == 1 && x1 == 1 && x3 == 5 && x4 == 1 && x5 & 1 == 1 => {
                            None
                        }
                        (x0, x1, _, x3, x4, _) if x0 == 1 && x1 == 1 && x3 == 6 && x4 == 1 => {
                            None
                        }
                        (x0, x1, _, x3, x4, _) if x0 == 1 && x1 == 1 && x3 == 7 && x4 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 1 && x1 == 1 && x2 != 31 && x3 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 1 && x1 == 1 && x2 != 31 && x3 == 1 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 1 && x1 == 1 && x2 != 31 && x3 == 2 && x5 & 1 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 1 && x1 == 1 && x2 != 31 && x3 == 3 && x5 & 1 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 1 && x1 == 1 && x2 != 31 && x3 == 4 && x5 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 1 && x2 != 31 && x3 == 4 && x4 == 0 && x5 == 1 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 1 && x1 == 1 && x2 != 31 && x3 == 5 && x5 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 1 && x2 != 31 && x3 == 5 && x4 == 0 && x5 == 1 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 1 && x2 != 31 && x3 == 6 && x4 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 1 && x2 != 31 && x3 == 7 && x4 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 1 && x1 == 1 && x2 == 31 && x3 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 1 && x1 == 1 && x2 == 31 && x3 == 1 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 1 && x1 == 1 && x2 == 31 && x3 == 2 && x5 & 1 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 1 && x1 == 1 && x2 == 31 && x3 == 3 && x5 & 1 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 1 && x1 == 1 && x2 == 31 && x3 == 4 && x5 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 1 && x2 == 31 && x3 == 4 && x4 == 0 && x5 == 1 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 1 && x1 == 1 && x2 == 31 && x3 == 5 && x5 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 1 && x2 == 31 && x3 == 5 && x4 == 0 && x5 == 1 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 1 && x2 == 31 && x3 == 6 && x4 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 1 && x2 == 31 && x3 == 7 && x4 == 0 => {
                            Some(Op::aarch64_memory_vector_single_post_inc)
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _, _, _) if x0 & 11 == 0 && x2 == 1 && x4 & 1 == 0 && x6 & 16 == 16 => {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _, _, _) if x0 & 11 == 0 && x2 == 1 && x4 & 1 == 0 && x6 & 8 == 8 => {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _, _, _) if x0 & 11 == 0 && x2 == 1 && x4 & 1 == 0 && x6 & 4 == 4 => {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _, _, _) if x0 & 11 == 0 && x2 == 1 && x4 & 1 == 0 && x6 & 2 == 2 => {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _, _, _) if x0 & 11 == 0 && x2 == 1 && x4 & 1 == 0 && x6 & 1 == 1 => {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _, _, _) if x0 == 13 && x2 == 0 && x4 & 2 == 2 && x6 & 32 == 32 => {
                    let opc = (instr >> 22) & 3;
                    let imm9 = (instr >> 12) & 17;
                    let op2 = (instr >> 10) & 3;
                    let Rn = (instr >> 5) & 9;
                    let Rt = instr & 9;
                    match (opc, imm9, op2) {
                        (x0, _, x2) if x0 == 0 && x2 == 1 => {
                            Some(Op::aarch64_integer_tags_mcsettagpost)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 2 => {
                            Some(Op::aarch64_integer_tags_mcsettag)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 3 => {
                            Some(Op::aarch64_integer_tags_mcsettagpre)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_integer_tags_mcsettagandzeroarray)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 0 => {
                            Some(Op::aarch64_integer_tags_mcgettag)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 1 => {
                            Some(Op::aarch64_integer_tags_mcsettagandzerodatapost)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 2 => {
                            Some(Op::aarch64_integer_tags_mcsettagandzerodata)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 3 => {
                            Some(Op::aarch64_integer_tags_mcsettagandzerodatapre)
                        }
                        (x0, _, x2) if x0 == 2 && x2 == 1 => {
                            Some(Op::aarch64_integer_tags_mcsettagpairpost)
                        }
                        (x0, _, x2) if x0 == 2 && x2 == 2 => {
                            Some(Op::aarch64_integer_tags_mcsettagpair)
                        }
                        (x0, _, x2) if x0 == 2 && x2 == 3 => {
                            Some(Op::aarch64_integer_tags_mcsettagpairpre)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 != 0 && x2 == 0 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_integer_tags_mcsettagarray)
                        }
                        (x0, _, x2) if x0 == 3 && x2 == 1 => {
                            Some(Op::aarch64_integer_tags_mcsettagpairandzerodatapost)
                        }
                        (x0, _, x2) if x0 == 3 && x2 == 2 => {
                            Some(Op::aarch64_integer_tags_mcsettagpairandzerodata)
                        }
                        (x0, _, x2) if x0 == 3 && x2 == 3 => {
                            Some(Op::aarch64_integer_tags_mcsettagpairandzerodatapre)
                        }
                        (x0, x1, x2) if x0 == 3 && x1 != 0 && x2 == 0 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_integer_tags_mcgettagarray)
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, _, _, _, _, _, _) if x0 & 11 == 8 && x2 == 1 => {
                    None
                }
                (x0, _, x2, _, x4, _, _, _, _, _) if x0 & 3 == 0 && x2 == 0 && x4 & 2 == 0 => {
                    let size = (instr >> 30) & 3;
                    let o2 = (instr >> 23) & 1;
                    let L = (instr >> 22) & 1;
                    let o1 = (instr >> 21) & 1;
                    let Rs = (instr >> 16) & 9;
                    let o0 = (instr >> 15) & 1;
                    let Rt2 = (instr >> 10) & 9;
                    let Rn = (instr >> 5) & 9;
                    let Rt = instr & 9;
                    match (size, o2, L, o1, o0, Rt2) {
                        (_, x1, _, x3, _, x5) if x1 == 1 && x3 == 1 && x5 != 31 => {
                            None
                        }
                        (x0, x1, _, x3, _, x5) if x0 & 2 == 0 && x1 == 0 && x3 == 1 && x5 != 31 => {
                            None
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 => {
                            Some(Op::aarch64_memory_exclusive_single)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 1 => {
                            Some(Op::aarch64_memory_exclusive_single)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 31 => {
                            Some(Op::aarch64_memory_atomicops_cas_pair)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 1 && x5 == 31 => {
                            Some(Op::aarch64_memory_atomicops_cas_pair)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            Some(Op::aarch64_memory_exclusive_single)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 => {
                            Some(Op::aarch64_memory_exclusive_single)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 31 => {
                            Some(Op::aarch64_memory_atomicops_cas_pair)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 31 => {
                            Some(Op::aarch64_memory_atomicops_cas_pair)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 0 && x4 == 0 => {
                            Some(Op::aarch64_memory_ordered)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 0 && x4 == 1 => {
                            Some(Op::aarch64_memory_ordered)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 31 => {
                            Some(Op::aarch64_memory_atomicops_cas_single)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 1 && x4 == 1 && x5 == 31 => {
                            Some(Op::aarch64_memory_atomicops_cas_single)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            Some(Op::aarch64_memory_ordered)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 0 && x4 == 1 => {
                            Some(Op::aarch64_memory_ordered)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 31 => {
                            Some(Op::aarch64_memory_atomicops_cas_single)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 31 => {
                            Some(Op::aarch64_memory_atomicops_cas_single)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 => {
                            Some(Op::aarch64_memory_exclusive_single)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 1 => {
                            Some(Op::aarch64_memory_exclusive_single)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 31 => {
                            Some(Op::aarch64_memory_atomicops_cas_pair)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 1 && x5 == 31 => {
                            Some(Op::aarch64_memory_atomicops_cas_pair)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            Some(Op::aarch64_memory_exclusive_single)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 => {
                            Some(Op::aarch64_memory_exclusive_single)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 31 => {
                            Some(Op::aarch64_memory_atomicops_cas_pair)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 31 => {
                            Some(Op::aarch64_memory_atomicops_cas_pair)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 0 && x4 == 0 => {
                            Some(Op::aarch64_memory_ordered)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 0 && x4 == 1 => {
                            Some(Op::aarch64_memory_ordered)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 31 => {
                            Some(Op::aarch64_memory_atomicops_cas_single)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 1 && x4 == 1 && x5 == 31 => {
                            Some(Op::aarch64_memory_atomicops_cas_single)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            Some(Op::aarch64_memory_ordered)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 0 && x4 == 1 => {
                            Some(Op::aarch64_memory_ordered)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 31 => {
                            Some(Op::aarch64_memory_atomicops_cas_single)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 31 => {
                            Some(Op::aarch64_memory_atomicops_cas_single)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 => {
                            Some(Op::aarch64_memory_exclusive_single)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 1 => {
                            Some(Op::aarch64_memory_exclusive_single)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 => {
                            Some(Op::aarch64_memory_exclusive_pair)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 1 => {
                            Some(Op::aarch64_memory_exclusive_pair)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            Some(Op::aarch64_memory_exclusive_single)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 => {
                            Some(Op::aarch64_memory_exclusive_single)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 => {
                            Some(Op::aarch64_memory_exclusive_pair)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 => {
                            Some(Op::aarch64_memory_exclusive_pair)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 2 && x1 == 1 && x2 == 0 && x3 == 0 && x4 == 0 => {
                            Some(Op::aarch64_memory_ordered)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 2 && x1 == 1 && x2 == 0 && x3 == 0 && x4 == 1 => {
                            Some(Op::aarch64_memory_ordered)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 1 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 31 => {
                            Some(Op::aarch64_memory_atomicops_cas_single)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 1 && x2 == 0 && x3 == 1 && x4 == 1 && x5 == 31 => {
                            Some(Op::aarch64_memory_atomicops_cas_single)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 2 && x1 == 1 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            Some(Op::aarch64_memory_ordered)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 2 && x1 == 1 && x2 == 1 && x3 == 0 && x4 == 1 => {
                            Some(Op::aarch64_memory_ordered)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 1 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 31 => {
                            Some(Op::aarch64_memory_atomicops_cas_single)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 1 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 31 => {
                            Some(Op::aarch64_memory_atomicops_cas_single)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 => {
                            Some(Op::aarch64_memory_exclusive_single)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 1 => {
                            Some(Op::aarch64_memory_exclusive_single)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 => {
                            Some(Op::aarch64_memory_exclusive_pair)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 1 => {
                            Some(Op::aarch64_memory_exclusive_pair)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            Some(Op::aarch64_memory_exclusive_single)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 => {
                            Some(Op::aarch64_memory_exclusive_single)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 => {
                            Some(Op::aarch64_memory_exclusive_pair)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 => {
                            Some(Op::aarch64_memory_exclusive_pair)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 3 && x1 == 1 && x2 == 0 && x3 == 0 && x4 == 0 => {
                            Some(Op::aarch64_memory_ordered)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 3 && x1 == 1 && x2 == 0 && x3 == 0 && x4 == 1 => {
                            Some(Op::aarch64_memory_ordered)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 1 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 31 => {
                            Some(Op::aarch64_memory_atomicops_cas_single)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 1 && x2 == 0 && x3 == 1 && x4 == 1 && x5 == 31 => {
                            Some(Op::aarch64_memory_atomicops_cas_single)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 3 && x1 == 1 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            Some(Op::aarch64_memory_ordered)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 3 && x1 == 1 && x2 == 1 && x3 == 0 && x4 == 1 => {
                            Some(Op::aarch64_memory_ordered)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 1 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 31 => {
                            Some(Op::aarch64_memory_atomicops_cas_single)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 1 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 31 => {
                            Some(Op::aarch64_memory_atomicops_cas_single)
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _, x8, _) if x0 & 3 == 1 && x2 == 0 && x4 & 2 == 2 && x6 & 32 == 0 && x8 == 0 => {
                    let size = (instr >> 30) & 3;
                    let opc = (instr >> 22) & 3;
                    let imm9 = (instr >> 12) & 17;
                    let Rn = (instr >> 5) & 9;
                    let Rt = instr & 9;
                    match (size, opc) {
                        (x0, x1) if x0 == 0 && x1 == 0 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_lda_stl)
                        }
                        (x0, x1) if x0 == 0 && x1 == 1 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_lda_stl)
                        }
                        (x0, x1) if x0 == 0 && x1 == 2 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_lda_stl)
                        }
                        (x0, x1) if x0 == 0 && x1 == 3 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_lda_stl)
                        }
                        (x0, x1) if x0 == 1 && x1 == 0 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_lda_stl)
                        }
                        (x0, x1) if x0 == 1 && x1 == 1 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_lda_stl)
                        }
                        (x0, x1) if x0 == 1 && x1 == 2 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_lda_stl)
                        }
                        (x0, x1) if x0 == 1 && x1 == 3 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_lda_stl)
                        }
                        (x0, x1) if x0 == 2 && x1 == 0 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_lda_stl)
                        }
                        (x0, x1) if x0 == 2 && x1 == 1 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_lda_stl)
                        }
                        (x0, x1) if x0 == 2 && x1 == 2 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_lda_stl)
                        }
                        (x0, x1) if x0 == 2 && x1 == 3 => {
                            None
                        }
                        (x0, x1) if x0 == 3 && x1 == 0 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_lda_stl)
                        }
                        (x0, x1) if x0 == 3 && x1 == 1 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_lda_stl)
                        }
                        (x0, x1) if x0 == 3 && x1 == 2 => {
                            None
                        }
                        (x0, x1) if x0 == 3 && x1 == 3 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, _, _, x4, _, _, _, _, _) if x0 & 3 == 1 && x4 & 2 == 0 => {
                    let opc = (instr >> 30) & 3;
                    let V = (instr >> 26) & 1;
                    let imm19 = (instr >> 5) & 37;
                    let Rt = instr & 9;
                    match (opc, V) {
                        (x0, x1) if x0 == 0 && x1 == 0 => {
                            Some(Op::aarch64_memory_literal_general)
                        }
                        (x0, x1) if x0 == 0 && x1 == 1 => {
                            Some(Op::aarch64_memory_literal_simdfp)
                        }
                        (x0, x1) if x0 == 1 && x1 == 0 => {
                            Some(Op::aarch64_memory_literal_general)
                        }
                        (x0, x1) if x0 == 1 && x1 == 1 => {
                            Some(Op::aarch64_memory_literal_simdfp)
                        }
                        (x0, x1) if x0 == 2 && x1 == 0 => {
                            Some(Op::aarch64_memory_literal_general)
                        }
                        (x0, x1) if x0 == 2 && x1 == 1 => {
                            Some(Op::aarch64_memory_literal_simdfp)
                        }
                        (x0, x1) if x0 == 3 && x1 == 0 => {
                            Some(Op::aarch64_memory_literal_general)
                        }
                        (x0, x1) if x0 == 3 && x1 == 1 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, _, _, x4, _, _, _, _, _) if x0 & 3 == 2 && x4 == 0 => {
                    let opc = (instr >> 30) & 3;
                    let V = (instr >> 26) & 1;
                    let L = (instr >> 22) & 1;
                    let imm7 = (instr >> 15) & 13;
                    let Rt2 = (instr >> 10) & 9;
                    let Rn = (instr >> 5) & 9;
                    let Rt = instr & 9;
                    match (opc, V, L) {
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_memory_pair_general_no_alloc)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_memory_pair_general_no_alloc)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_memory_pair_simdfp_no_alloc)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_memory_pair_simdfp_no_alloc)
                        }
                        (x0, x1, _) if x0 == 1 && x1 == 0 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_memory_pair_simdfp_no_alloc)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_memory_pair_simdfp_no_alloc)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_memory_pair_general_no_alloc)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_memory_pair_general_no_alloc)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_memory_pair_simdfp_no_alloc)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_memory_pair_simdfp_no_alloc)
                        }
                        (x0, _, _) if x0 == 3 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, _, _, x4, _, _, _, _, _) if x0 & 3 == 2 && x4 == 1 => {
                    let opc = (instr >> 30) & 3;
                    let V = (instr >> 26) & 1;
                    let L = (instr >> 22) & 1;
                    let imm7 = (instr >> 15) & 13;
                    let Rt2 = (instr >> 10) & 9;
                    let Rn = (instr >> 5) & 9;
                    let Rt = instr & 9;
                    match (opc, V, L) {
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_memory_pair_general_post_idx)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_memory_pair_general_post_idx)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_memory_pair_simdfp_post_idx)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_memory_pair_simdfp_post_idx)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_integer_tags_mcsettaganddatapairpost)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_memory_pair_general_post_idx)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_memory_pair_simdfp_post_idx)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_memory_pair_simdfp_post_idx)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_memory_pair_general_post_idx)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_memory_pair_general_post_idx)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_memory_pair_simdfp_post_idx)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_memory_pair_simdfp_post_idx)
                        }
                        (x0, _, _) if x0 == 3 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, _, _, x4, _, _, _, _, _) if x0 & 3 == 2 && x4 == 2 => {
                    let opc = (instr >> 30) & 3;
                    let V = (instr >> 26) & 1;
                    let L = (instr >> 22) & 1;
                    let imm7 = (instr >> 15) & 13;
                    let Rt2 = (instr >> 10) & 9;
                    let Rn = (instr >> 5) & 9;
                    let Rt = instr & 9;
                    match (opc, V, L) {
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_memory_pair_general_offset)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_memory_pair_general_offset)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_memory_pair_simdfp_offset)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_memory_pair_simdfp_offset)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_integer_tags_mcsettaganddatapair)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_memory_pair_general_offset)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_memory_pair_simdfp_offset)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_memory_pair_simdfp_offset)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_memory_pair_general_offset)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_memory_pair_general_offset)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_memory_pair_simdfp_offset)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_memory_pair_simdfp_offset)
                        }
                        (x0, _, _) if x0 == 3 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, _, _, x4, _, _, _, _, _) if x0 & 3 == 2 && x4 == 3 => {
                    let opc = (instr >> 30) & 3;
                    let V = (instr >> 26) & 1;
                    let L = (instr >> 22) & 1;
                    let imm7 = (instr >> 15) & 13;
                    let Rt2 = (instr >> 10) & 9;
                    let Rn = (instr >> 5) & 9;
                    let Rt = instr & 9;
                    match (opc, V, L) {
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_memory_pair_general_pre_idx)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_memory_pair_general_pre_idx)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_memory_pair_simdfp_pre_idx)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_memory_pair_simdfp_pre_idx)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_integer_tags_mcsettaganddatapairpre)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_memory_pair_general_pre_idx)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_memory_pair_simdfp_pre_idx)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_memory_pair_simdfp_pre_idx)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_memory_pair_general_pre_idx)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_memory_pair_general_pre_idx)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_memory_pair_simdfp_pre_idx)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_memory_pair_simdfp_pre_idx)
                        }
                        (x0, _, _) if x0 == 3 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, _, _, x4, _, x6, _, x8, _) if x0 & 3 == 3 && x4 & 2 == 0 && x6 & 32 == 0 && x8 == 0 => {
                    let size = (instr >> 30) & 3;
                    let V = (instr >> 26) & 1;
                    let opc = (instr >> 22) & 3;
                    let imm9 = (instr >> 12) & 17;
                    let Rn = (instr >> 5) & 9;
                    let Rt = instr & 9;
                    match (size, V, opc) {
                        (x0, x1, x2) if x0 & 1 == 1 && x1 == 1 && x2 & 2 == 2 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_normal)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_normal)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 2 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_normal)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 3 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_normal)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_signed_offset_normal)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_signed_offset_normal)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 2 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_signed_offset_normal)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 3 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_signed_offset_normal)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_normal)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_normal)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 2 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_normal)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 3 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_normal)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_signed_offset_normal)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_signed_offset_normal)
                        }
                        (x0, x1, x2) if x0 & 2 == 2 && x1 == 0 && x2 == 3 => {
                            None
                        }
                        (x0, x1, x2) if x0 & 2 == 2 && x1 == 1 && x2 & 2 == 2 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_normal)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_normal)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 2 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_normal)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_signed_offset_normal)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_signed_offset_normal)
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_normal)
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_normal)
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 2 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_normal)
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_signed_offset_normal)
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_signed_offset_normal)
                        }
                        _ => None
                    }
                }
                (x0, _, _, _, x4, _, x6, _, x8, _) if x0 & 3 == 3 && x4 & 2 == 0 && x6 & 32 == 0 && x8 == 1 => {
                    let size = (instr >> 30) & 3;
                    let V = (instr >> 26) & 1;
                    let opc = (instr >> 22) & 3;
                    let imm9 = (instr >> 12) & 17;
                    let Rn = (instr >> 5) & 9;
                    let Rt = instr & 9;
                    match (size, V, opc) {
                        (x0, x1, x2) if x0 & 1 == 1 && x1 == 1 && x2 & 2 == 2 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_post_idx)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_post_idx)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 2 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_post_idx)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 3 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_post_idx)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_signed_post_idx)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_signed_post_idx)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 2 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_signed_post_idx)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 3 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_signed_post_idx)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_post_idx)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_post_idx)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 2 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_post_idx)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 3 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_post_idx)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_signed_post_idx)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_signed_post_idx)
                        }
                        (x0, x1, x2) if x0 & 2 == 2 && x1 == 0 && x2 == 3 => {
                            None
                        }
                        (x0, x1, x2) if x0 & 2 == 2 && x1 == 1 && x2 & 2 == 2 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_post_idx)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_post_idx)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 2 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_post_idx)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_signed_post_idx)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_signed_post_idx)
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_post_idx)
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_post_idx)
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 2 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_signed_post_idx)
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_signed_post_idx)
                        }
                        _ => None
                    }
                }
                (x0, _, _, _, x4, _, x6, _, x8, _) if x0 & 3 == 3 && x4 & 2 == 0 && x6 & 32 == 0 && x8 == 2 => {
                    let size = (instr >> 30) & 3;
                    let V = (instr >> 26) & 1;
                    let opc = (instr >> 22) & 3;
                    let imm9 = (instr >> 12) & 17;
                    let Rn = (instr >> 5) & 9;
                    let Rt = instr & 9;
                    match (size, V, opc) {
                        (_, x1, _) if x1 == 1 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_unpriv)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_unpriv)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 2 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_unpriv)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 3 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_unpriv)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_unpriv)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_unpriv)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 2 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_unpriv)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 3 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_unpriv)
                        }
                        (x0, x1, x2) if x0 & 2 == 2 && x1 == 0 && x2 == 3 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_unpriv)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_unpriv)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 2 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_unpriv)
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_unpriv)
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_offset_unpriv)
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 2 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, _, _, x4, _, x6, _, x8, _) if x0 & 3 == 3 && x4 & 2 == 0 && x6 & 32 == 0 && x8 == 3 => {
                    let size = (instr >> 30) & 3;
                    let V = (instr >> 26) & 1;
                    let opc = (instr >> 22) & 3;
                    let imm9 = (instr >> 12) & 17;
                    let Rn = (instr >> 5) & 9;
                    let Rt = instr & 9;
                    match (size, V, opc) {
                        (x0, x1, x2) if x0 & 1 == 1 && x1 == 1 && x2 & 2 == 2 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_pre_idx)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_pre_idx)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 2 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_pre_idx)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 3 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_pre_idx)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_signed_pre_idx)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_signed_pre_idx)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 2 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_signed_pre_idx)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 3 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_signed_pre_idx)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_pre_idx)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_pre_idx)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 2 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_pre_idx)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 3 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_pre_idx)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_signed_pre_idx)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_signed_pre_idx)
                        }
                        (x0, x1, x2) if x0 & 2 == 2 && x1 == 0 && x2 == 3 => {
                            None
                        }
                        (x0, x1, x2) if x0 & 2 == 2 && x1 == 1 && x2 & 2 == 2 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_pre_idx)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_pre_idx)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 2 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_pre_idx)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_signed_pre_idx)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_signed_pre_idx)
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_pre_idx)
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_pre_idx)
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 2 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_signed_pre_idx)
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_signed_pre_idx)
                        }
                        _ => None
                    }
                }
                (x0, _, _, _, x4, _, x6, _, x8, _) if x0 & 3 == 3 && x4 & 2 == 0 && x6 & 32 == 32 && x8 == 0 => {
                    let size = (instr >> 30) & 3;
                    let V = (instr >> 26) & 1;
                    let A = (instr >> 23) & 1;
                    let R = (instr >> 22) & 1;
                    let Rs = (instr >> 16) & 9;
                    let o3 = (instr >> 15) & 1;
                    let opc = (instr >> 12) & 5;
                    let Rn = (instr >> 5) & 9;
                    let Rt = instr & 9;
                    match (size, V, A, R, o3, opc) {
                        (_, x1, _, _, x4, x5) if x1 == 0 && x4 == 1 && x5 == 1 => {
                            None
                        }
                        (_, x1, _, _, x4, x5) if x1 == 0 && x4 == 1 && x5 & 6 == 2 => {
                            None
                        }
                        (_, x1, _, _, x4, x5) if x1 == 0 && x4 == 1 && x5 == 5 => {
                            None
                        }
                        (_, x1, _, _, x4, x5) if x1 == 0 && x4 == 1 && x5 & 6 == 6 => {
                            None
                        }
                        (_, x1, x2, _, x4, x5) if x1 == 0 && x2 == 0 && x4 == 1 && x5 == 4 => {
                            None
                        }
                        (_, x1, x2, x3, x4, x5) if x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 4 => {
                            None
                        }
                        (_, x1, _, _, _, _) if x1 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 0 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 1 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 2 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 3 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 4 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 5 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 6 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 7 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 1 && x5 == 0 => {
                            Some(Op::aarch64_memory_atomicops_swp)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 0 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 1 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 2 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 3 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 4 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 5 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 6 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 7 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 1 && x5 == 0 => {
                            Some(Op::aarch64_memory_atomicops_swp)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 0 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 1 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 2 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 3 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 4 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 5 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 6 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 7 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 0 => {
                            Some(Op::aarch64_memory_atomicops_swp)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 4 => {
                            Some(Op::aarch64_memory_ordered_rcpc)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 0 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 1 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 2 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 3 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 4 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 5 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 6 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 7 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 0 => {
                            Some(Op::aarch64_memory_atomicops_swp)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 0 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 1 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 2 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 3 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 4 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 5 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 6 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 7 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 1 && x5 == 0 => {
                            Some(Op::aarch64_memory_atomicops_swp)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 0 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 1 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 2 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 3 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 4 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 5 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 6 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 7 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 1 && x5 == 0 => {
                            Some(Op::aarch64_memory_atomicops_swp)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 0 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 1 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 2 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 3 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 4 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 5 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 6 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 7 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 0 => {
                            Some(Op::aarch64_memory_atomicops_swp)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 4 => {
                            Some(Op::aarch64_memory_ordered_rcpc)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 0 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 1 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 2 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 3 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 4 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 5 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 6 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 7 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 0 => {
                            Some(Op::aarch64_memory_atomicops_swp)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 0 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 1 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 2 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 3 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 4 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 5 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 6 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 7 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 1 && x5 == 0 => {
                            Some(Op::aarch64_memory_atomicops_swp)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 0 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 1 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 2 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 3 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 4 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 5 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 6 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 7 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 1 && x5 == 0 => {
                            Some(Op::aarch64_memory_atomicops_swp)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 0 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 1 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 2 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 3 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 4 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 5 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 6 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 7 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 0 => {
                            Some(Op::aarch64_memory_atomicops_swp)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 4 => {
                            Some(Op::aarch64_memory_ordered_rcpc)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 0 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 1 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 2 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 3 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 4 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 5 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 6 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 7 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 0 => {
                            Some(Op::aarch64_memory_atomicops_swp)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 0 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 1 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 2 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 3 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 4 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 5 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 6 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 7 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 1 && x5 == 0 => {
                            Some(Op::aarch64_memory_atomicops_swp)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 0 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 1 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 2 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 3 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 4 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 5 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 6 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 7 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 1 && x5 == 0 => {
                            Some(Op::aarch64_memory_atomicops_swp)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 0 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 1 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 2 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 3 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 4 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 5 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 6 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 7 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 0 => {
                            Some(Op::aarch64_memory_atomicops_swp)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 4 => {
                            Some(Op::aarch64_memory_ordered_rcpc)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 0 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 1 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 2 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 3 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 4 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 5 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 6 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 7 => {
                            Some(Op::aarch64_memory_atomicops_ld)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 0 => {
                            Some(Op::aarch64_memory_atomicops_swp)
                        }
                        _ => None
                    }
                }
                (x0, _, _, _, x4, _, x6, _, x8, _) if x0 & 3 == 3 && x4 & 2 == 0 && x6 & 32 == 32 && x8 == 2 => {
                    let size = (instr >> 30) & 3;
                    let V = (instr >> 26) & 1;
                    let opc = (instr >> 22) & 3;
                    let Rm = (instr >> 16) & 9;
                    let option = (instr >> 13) & 5;
                    let S = (instr >> 12) & 1;
                    let Rn = (instr >> 5) & 9;
                    let Rt = instr & 9;
                    match (size, V, opc, option) {
                        (_, _, _, x3) if x3 & 2 == 0 => {
                            None
                        }
                        (x0, x1, x2, _) if x0 & 1 == 1 && x1 == 1 && x2 & 2 == 2 => {
                            None
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 != 3 => {
                            Some(Op::aarch64_memory_single_general_register)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 3 => {
                            Some(Op::aarch64_memory_single_general_register)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 != 3 => {
                            Some(Op::aarch64_memory_single_general_register)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 3 => {
                            Some(Op::aarch64_memory_single_general_register)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 2 && x3 != 3 => {
                            Some(Op::aarch64_memory_single_general_register)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 2 && x3 == 3 => {
                            Some(Op::aarch64_memory_single_general_register)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 != 3 => {
                            Some(Op::aarch64_memory_single_general_register)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 3 => {
                            Some(Op::aarch64_memory_single_general_register)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 != 3 => {
                            Some(Op::aarch64_memory_single_simdfp_register)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 3 => {
                            Some(Op::aarch64_memory_single_simdfp_register)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 != 3 => {
                            Some(Op::aarch64_memory_single_simdfp_register)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 3 => {
                            Some(Op::aarch64_memory_single_simdfp_register)
                        }
                        (x0, x1, x2, _) if x0 == 0 && x1 == 1 && x2 == 2 => {
                            Some(Op::aarch64_memory_single_simdfp_register)
                        }
                        (x0, x1, x2, _) if x0 == 0 && x1 == 1 && x2 == 3 => {
                            Some(Op::aarch64_memory_single_simdfp_register)
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_general_register)
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_general_register)
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 0 && x2 == 2 => {
                            Some(Op::aarch64_memory_single_general_register)
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 0 && x2 == 3 => {
                            Some(Op::aarch64_memory_single_general_register)
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_simdfp_register)
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_simdfp_register)
                        }
                        (x0, x1, x2, _) if x0 & 2 == 2 && x1 == 0 && x2 == 3 => {
                            None
                        }
                        (x0, x1, x2, _) if x0 & 2 == 2 && x1 == 1 && x2 & 2 == 2 => {
                            None
                        }
                        (x0, x1, x2, _) if x0 == 2 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_general_register)
                        }
                        (x0, x1, x2, _) if x0 == 2 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_general_register)
                        }
                        (x0, x1, x2, _) if x0 == 2 && x1 == 0 && x2 == 2 => {
                            Some(Op::aarch64_memory_single_general_register)
                        }
                        (x0, x1, x2, _) if x0 == 2 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_simdfp_register)
                        }
                        (x0, x1, x2, _) if x0 == 2 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_simdfp_register)
                        }
                        (x0, x1, x2, _) if x0 == 3 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_general_register)
                        }
                        (x0, x1, x2, _) if x0 == 3 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_general_register)
                        }
                        (x0, x1, x2, _) if x0 == 3 && x1 == 0 && x2 == 2 => {
                            Some(Op::aarch64_memory_single_general_register)
                        }
                        (x0, x1, x2, _) if x0 == 3 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_simdfp_register)
                        }
                        (x0, x1, x2, _) if x0 == 3 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_simdfp_register)
                        }
                        _ => None
                    }
                }
                (x0, _, _, _, x4, _, x6, _, x8, _) if x0 & 3 == 3 && x4 & 2 == 0 && x6 & 32 == 32 && x8 & 1 == 1 => {
                    let size = (instr >> 30) & 3;
                    let V = (instr >> 26) & 1;
                    let M = (instr >> 23) & 1;
                    let S = (instr >> 22) & 1;
                    let imm9 = (instr >> 12) & 17;
                    let W = (instr >> 11) & 1;
                    let Rn = (instr >> 5) & 9;
                    let Rt = instr & 9;
                    match (size, V, M, W) {
                        (x0, _, _, _) if x0 != 3 => {
                            None
                        }
                        (x0, x1, x2, x3) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_pac)
                        }
                        (x0, x1, x2, x3) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_pac)
                        }
                        (x0, x1, x2, x3) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_pac)
                        }
                        (x0, x1, x2, x3) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 => {
                            Some(Op::aarch64_memory_single_general_immediate_signed_pac)
                        }
                        (x0, x1, _, _) if x0 == 3 && x1 == 1 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, _, _, x4, _, _, _, _, _) if x0 & 3 == 3 && x4 & 2 == 2 => {
                    let size = (instr >> 30) & 3;
                    let V = (instr >> 26) & 1;
                    let opc = (instr >> 22) & 3;
                    let imm12 = (instr >> 10) & 23;
                    let Rn = (instr >> 5) & 9;
                    let Rt = instr & 9;
                    match (size, V, opc) {
                        (x0, x1, x2) if x0 & 1 == 1 && x1 == 1 && x2 & 2 == 2 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_general_immediate_unsigned)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_general_immediate_unsigned)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 2 => {
                            Some(Op::aarch64_memory_single_general_immediate_unsigned)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 3 => {
                            Some(Op::aarch64_memory_single_general_immediate_unsigned)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_unsigned)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_unsigned)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 2 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_unsigned)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 3 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_unsigned)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_general_immediate_unsigned)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_general_immediate_unsigned)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 2 => {
                            Some(Op::aarch64_memory_single_general_immediate_unsigned)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 3 => {
                            Some(Op::aarch64_memory_single_general_immediate_unsigned)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_unsigned)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_unsigned)
                        }
                        (x0, x1, x2) if x0 & 2 == 2 && x1 == 0 && x2 == 3 => {
                            None
                        }
                        (x0, x1, x2) if x0 & 2 == 2 && x1 == 1 && x2 & 2 == 2 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_general_immediate_unsigned)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_general_immediate_unsigned)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 2 => {
                            Some(Op::aarch64_memory_single_general_immediate_unsigned)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_unsigned)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_unsigned)
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_general_immediate_unsigned)
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_general_immediate_unsigned)
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 2 => {
                            Some(Op::aarch64_memory_single_general_immediate_unsigned)
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_unsigned)
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_memory_single_simdfp_immediate_unsigned)
                        }
                        _ => None
                    }
                }
                _ => None
            }
        }
        (_, x1, _) if x1 & 14 == 10 => {
            match ((instr >> 31) & 1, (instr >> 30) & 1, (instr >> 29) & 1, (instr >> 28) & 1, (instr >> 25) & 5, (instr >> 21) & 7, (instr >> 16) & 9, (instr >> 10) & 11, instr & 19) {
                (_, x1, _, x3, _, x5, _, _, _) if x1 == 0 && x3 == 1 && x5 == 6 => {
                    let sf = (instr >> 31) & 1;
                    let S = (instr >> 29) & 1;
                    let Rm = (instr >> 16) & 9;
                    let opcode = (instr >> 10) & 11;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (sf, S, opcode) {
                        (_, _, x2) if x2 == 1 => {
                            None
                        }
                        (_, _, x2) if x2 & 56 == 24 => {
                            None
                        }
                        (_, _, x2) if x2 & 32 == 32 => {
                            None
                        }
                        (_, x1, x2) if x1 == 0 && x2 & 62 == 6 => {
                            None
                        }
                        (_, x1, x2) if x1 == 0 && x2 == 13 => {
                            None
                        }
                        (_, x1, x2) if x1 == 0 && x2 & 62 == 14 => {
                            None
                        }
                        (_, x1, x2) if x1 == 1 && x2 & 62 == 2 => {
                            None
                        }
                        (_, x1, x2) if x1 == 1 && x2 & 60 == 4 => {
                            None
                        }
                        (_, x1, x2) if x1 == 1 && x2 & 56 == 8 => {
                            None
                        }
                        (_, x1, x2) if x1 == 1 && x2 & 48 == 16 => {
                            None
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 0 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 2 => {
                            Some(Op::aarch64_integer_arithmetic_div)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 3 => {
                            Some(Op::aarch64_integer_arithmetic_div)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 & 62 == 4 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 8 => {
                            Some(Op::aarch64_integer_shift_variable)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 9 => {
                            Some(Op::aarch64_integer_shift_variable)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 10 => {
                            Some(Op::aarch64_integer_shift_variable)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 11 => {
                            Some(Op::aarch64_integer_shift_variable)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 12 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 & 59 == 19 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 16 => {
                            Some(Op::aarch64_integer_crc)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 17 => {
                            Some(Op::aarch64_integer_crc)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 18 => {
                            Some(Op::aarch64_integer_crc)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 20 => {
                            Some(Op::aarch64_integer_crc)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 21 => {
                            Some(Op::aarch64_integer_crc)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 22 => {
                            Some(Op::aarch64_integer_crc)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 2 => {
                            Some(Op::aarch64_integer_arithmetic_div)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 3 => {
                            Some(Op::aarch64_integer_arithmetic_div)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 4 => {
                            Some(Op::aarch64_integer_tags_mcinsertrandomtag)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 5 => {
                            Some(Op::aarch64_integer_tags_mcinserttagmask)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 8 => {
                            Some(Op::aarch64_integer_shift_variable)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 9 => {
                            Some(Op::aarch64_integer_shift_variable)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 10 => {
                            Some(Op::aarch64_integer_shift_variable)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 11 => {
                            Some(Op::aarch64_integer_shift_variable)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 12 => {
                            Some(Op::aarch64_integer_pac_pacga_dp_2src)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 & 57 == 16 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 & 58 == 16 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 19 => {
                            Some(Op::aarch64_integer_crc)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 23 => {
                            Some(Op::aarch64_integer_crc)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags)
                        }
                        _ => None
                    }
                }
                (_, x1, _, x3, _, x5, _, _, _) if x1 == 1 && x3 == 1 && x5 == 6 => {
                    let sf = (instr >> 31) & 1;
                    let S = (instr >> 29) & 1;
                    let opcode2 = (instr >> 16) & 9;
                    let opcode = (instr >> 10) & 11;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (sf, S, opcode2, opcode, Rn) {
                        (_, _, _, x3, _) if x3 & 32 == 32 => {
                            None
                        }
                        (_, _, x2, _, _) if x2 & 2 == 2 => {
                            None
                        }
                        (_, _, x2, _, _) if x2 & 4 == 4 => {
                            None
                        }
                        (_, _, x2, _, _) if x2 & 8 == 8 => {
                            None
                        }
                        (_, _, x2, _, _) if x2 & 16 == 16 => {
                            None
                        }
                        (_, x1, x2, x3, _) if x1 == 0 && x2 == 0 && x3 & 62 == 6 => {
                            None
                        }
                        (_, x1, x2, x3, _) if x1 == 0 && x2 == 0 && x3 & 56 == 8 => {
                            None
                        }
                        (_, x1, x2, x3, _) if x1 == 0 && x2 == 0 && x3 & 48 == 16 => {
                            None
                        }
                        (_, x1, _, _, _) if x1 == 1 => {
                            None
                        }
                        (x0, _, x2, _, _) if x0 == 0 && x2 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            Some(Op::aarch64_integer_arithmetic_rbit)
                        }
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            Some(Op::aarch64_integer_arithmetic_rev)
                        }
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 2 => {
                            Some(Op::aarch64_integer_arithmetic_rev)
                        }
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 3 => {
                            None
                        }
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 4 => {
                            Some(Op::aarch64_integer_arithmetic_cnt)
                        }
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 5 => {
                            Some(Op::aarch64_integer_arithmetic_cnt)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            Some(Op::aarch64_integer_arithmetic_rbit)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            Some(Op::aarch64_integer_arithmetic_rev)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 2 => {
                            Some(Op::aarch64_integer_arithmetic_rev)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 3 => {
                            Some(Op::aarch64_integer_arithmetic_rev)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 4 => {
                            Some(Op::aarch64_integer_arithmetic_cnt)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 5 => {
                            Some(Op::aarch64_integer_arithmetic_cnt)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            Some(Op::aarch64_integer_pac_pacia_dp_1src)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 => {
                            Some(Op::aarch64_integer_pac_pacib_dp_1src)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 2 => {
                            Some(Op::aarch64_integer_pac_pacda_dp_1src)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 3 => {
                            Some(Op::aarch64_integer_pac_pacdb_dp_1src)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 4 => {
                            Some(Op::aarch64_integer_pac_autia_dp_1src)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 5 => {
                            Some(Op::aarch64_integer_pac_autib_dp_1src)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 6 => {
                            Some(Op::aarch64_integer_pac_autda_dp_1src)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 7 => {
                            Some(Op::aarch64_integer_pac_autdb_dp_1src)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 8 && x4 == 31 => {
                            Some(Op::aarch64_integer_pac_pacia_dp_1src)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 9 && x4 == 31 => {
                            Some(Op::aarch64_integer_pac_pacib_dp_1src)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 10 && x4 == 31 => {
                            Some(Op::aarch64_integer_pac_pacda_dp_1src)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 11 && x4 == 31 => {
                            Some(Op::aarch64_integer_pac_pacdb_dp_1src)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 12 && x4 == 31 => {
                            Some(Op::aarch64_integer_pac_autia_dp_1src)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 13 && x4 == 31 => {
                            Some(Op::aarch64_integer_pac_autib_dp_1src)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 14 && x4 == 31 => {
                            Some(Op::aarch64_integer_pac_autda_dp_1src)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 15 && x4 == 31 => {
                            Some(Op::aarch64_integer_pac_autdb_dp_1src)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 16 && x4 == 31 => {
                            Some(Op::aarch64_integer_pac_strip_dp_1src)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 17 && x4 == 31 => {
                            Some(Op::aarch64_integer_pac_strip_dp_1src)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 & 62 == 18 => {
                            None
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 & 60 == 20 => {
                            None
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 & 56 == 24 => {
                            None
                        }
                        _ => None
                    }
                }
                (_, _, _, x3, _, x5, _, _, _) if x3 == 0 && x5 & 8 == 0 => {
                    let sf = (instr >> 31) & 1;
                    let opc = (instr >> 29) & 3;
                    let shift = (instr >> 22) & 3;
                    let N = (instr >> 21) & 1;
                    let Rm = (instr >> 16) & 9;
                    let imm6 = (instr >> 10) & 11;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (sf, opc, N, imm6) {
                        (x0, _, _, x3) if x0 == 0 && x3 & 32 == 32 => {
                            None
                        }
                        (x0, x1, x2, _) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_integer_logical_shiftedreg)
                        }
                        (x0, x1, x2, _) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_integer_logical_shiftedreg)
                        }
                        (x0, x1, x2, _) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_integer_logical_shiftedreg)
                        }
                        (x0, x1, x2, _) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_integer_logical_shiftedreg)
                        }
                        (x0, x1, x2, _) if x0 == 0 && x1 == 2 && x2 == 0 => {
                            Some(Op::aarch64_integer_logical_shiftedreg)
                        }
                        (x0, x1, x2, _) if x0 == 0 && x1 == 2 && x2 == 1 => {
                            Some(Op::aarch64_integer_logical_shiftedreg)
                        }
                        (x0, x1, x2, _) if x0 == 0 && x1 == 3 && x2 == 0 => {
                            Some(Op::aarch64_integer_logical_shiftedreg)
                        }
                        (x0, x1, x2, _) if x0 == 0 && x1 == 3 && x2 == 1 => {
                            Some(Op::aarch64_integer_logical_shiftedreg)
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_integer_logical_shiftedreg)
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_integer_logical_shiftedreg)
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_integer_logical_shiftedreg)
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_integer_logical_shiftedreg)
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 2 && x2 == 0 => {
                            Some(Op::aarch64_integer_logical_shiftedreg)
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 2 && x2 == 1 => {
                            Some(Op::aarch64_integer_logical_shiftedreg)
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 3 && x2 == 0 => {
                            Some(Op::aarch64_integer_logical_shiftedreg)
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 3 && x2 == 1 => {
                            Some(Op::aarch64_integer_logical_shiftedreg)
                        }
                        _ => None
                    }
                }
                (_, _, _, x3, _, x5, _, _, _) if x3 == 0 && x5 & 9 == 8 => {
                    let sf = (instr >> 31) & 1;
                    let op = (instr >> 30) & 1;
                    let S = (instr >> 29) & 1;
                    let shift = (instr >> 22) & 3;
                    let Rm = (instr >> 16) & 9;
                    let imm6 = (instr >> 10) & 11;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (sf, op, S, shift, imm6) {
                        (_, _, _, x3, _) if x3 == 3 => {
                            None
                        }
                        (x0, _, _, _, x4) if x0 == 0 && x4 & 32 == 32 => {
                            None
                        }
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_integer_arithmetic_add_sub_shiftedreg)
                        }
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_integer_arithmetic_add_sub_shiftedreg)
                        }
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_integer_arithmetic_add_sub_shiftedreg)
                        }
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_integer_arithmetic_add_sub_shiftedreg)
                        }
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_integer_arithmetic_add_sub_shiftedreg)
                        }
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_integer_arithmetic_add_sub_shiftedreg)
                        }
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_integer_arithmetic_add_sub_shiftedreg)
                        }
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_integer_arithmetic_add_sub_shiftedreg)
                        }
                        _ => None
                    }
                }
                (_, _, _, x3, _, x5, _, _, _) if x3 == 0 && x5 & 9 == 9 => {
                    let sf = (instr >> 31) & 1;
                    let op = (instr >> 30) & 1;
                    let S = (instr >> 29) & 1;
                    let opt = (instr >> 22) & 3;
                    let Rm = (instr >> 16) & 9;
                    let option = (instr >> 13) & 5;
                    let imm3 = (instr >> 10) & 5;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (sf, op, S, opt, imm3) {
                        (_, _, _, _, x4) if x4 & 5 == 5 => {
                            None
                        }
                        (_, _, _, _, x4) if x4 & 6 == 6 => {
                            None
                        }
                        (_, _, _, x3, _) if x3 & 1 == 1 => {
                            None
                        }
                        (_, _, _, x3, _) if x3 & 2 == 2 => {
                            None
                        }
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            Some(Op::aarch64_integer_arithmetic_add_sub_extendedreg)
                        }
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            Some(Op::aarch64_integer_arithmetic_add_sub_extendedreg)
                        }
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 0 => {
                            Some(Op::aarch64_integer_arithmetic_add_sub_extendedreg)
                        }
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 0 => {
                            Some(Op::aarch64_integer_arithmetic_add_sub_extendedreg)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            Some(Op::aarch64_integer_arithmetic_add_sub_extendedreg)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            Some(Op::aarch64_integer_arithmetic_add_sub_extendedreg)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 0 => {
                            Some(Op::aarch64_integer_arithmetic_add_sub_extendedreg)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 0 => {
                            Some(Op::aarch64_integer_arithmetic_add_sub_extendedreg)
                        }
                        _ => None
                    }
                }
                (_, _, _, x3, _, x5, _, x7, _) if x3 == 1 && x5 == 0 && x7 == 0 => {
                    let sf = (instr >> 31) & 1;
                    let op = (instr >> 30) & 1;
                    let S = (instr >> 29) & 1;
                    let Rm = (instr >> 16) & 9;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (sf, op, S) {
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_integer_arithmetic_add_sub_carry)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_integer_arithmetic_add_sub_carry)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_integer_arithmetic_add_sub_carry)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_integer_arithmetic_add_sub_carry)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_integer_arithmetic_add_sub_carry)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_integer_arithmetic_add_sub_carry)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_integer_arithmetic_add_sub_carry)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_integer_arithmetic_add_sub_carry)
                        }
                        _ => None
                    }
                }
                (_, _, _, x3, _, x5, _, x7, _) if x3 == 1 && x5 == 0 && x7 & 31 == 1 => {
                    let sf = (instr >> 31) & 1;
                    let op = (instr >> 30) & 1;
                    let S = (instr >> 29) & 1;
                    let imm6 = (instr >> 15) & 11;
                    let Rn = (instr >> 5) & 9;
                    let o2 = (instr >> 4) & 1;
                    let mask = instr & 7;
                    match (sf, op, S, o2) {
                        (x0, _, _, _) if x0 == 0 => {
                            None
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            None
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            Some(Op::aarch64_integer_flags_rmif)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 => {
                            None
                        }
                        (x0, x1, _, _) if x0 == 1 && x1 == 1 => {
                            None
                        }
                        _ => None
                    }
                }
                (_, _, _, x3, _, x5, _, x7, _) if x3 == 1 && x5 == 0 && x7 & 15 == 2 => {
                    let sf = (instr >> 31) & 1;
                    let op = (instr >> 30) & 1;
                    let S = (instr >> 29) & 1;
                    let opcode2 = (instr >> 15) & 11;
                    let sz = (instr >> 14) & 1;
                    let Rn = (instr >> 5) & 9;
                    let o3 = (instr >> 4) & 1;
                    let mask = instr & 7;
                    match (sf, op, S, opcode2, sz, o3, mask) {
                        (x0, x1, x2, _, _, _, _) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            None
                        }
                        (x0, x1, x2, x3, _, _, _) if x0 == 0 && x1 == 0 && x2 == 1 && x3 != 0 => {
                            None
                        }
                        (x0, x1, x2, x3, _, x5, x6) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x5 == 0 && x6 != 13 => {
                            None
                        }
                        (x0, x1, x2, x3, _, x5, _) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x5 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, x4, x5, x6) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 0 && x6 == 13 => {
                            Some(Op::aarch64_integer_flags_setf)
                        }
                        (x0, x1, x2, x3, x4, x5, x6) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 0 && x6 == 13 => {
                            Some(Op::aarch64_integer_flags_setf)
                        }
                        (x0, x1, _, _, _, _, _) if x0 == 0 && x1 == 1 => {
                            None
                        }
                        (x0, _, _, _, _, _, _) if x0 == 1 => {
                            None
                        }
                        _ => None
                    }
                }
                (_, _, _, x3, _, x5, _, x7, _) if x3 == 1 && x5 == 2 && x7 & 2 == 0 => {
                    let sf = (instr >> 31) & 1;
                    let op = (instr >> 30) & 1;
                    let S = (instr >> 29) & 1;
                    let Rm = (instr >> 16) & 9;
                    let cond = (instr >> 12) & 7;
                    let o2 = (instr >> 10) & 1;
                    let Rn = (instr >> 5) & 9;
                    let o3 = (instr >> 4) & 1;
                    let nzcv = instr & 7;
                    match (sf, op, S, o2, o3) {
                        (_, _, _, _, x4) if x4 == 1 => {
                            None
                        }
                        (_, _, _, x3, _) if x3 == 1 => {
                            None
                        }
                        (_, _, x2, _, _) if x2 == 0 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            Some(Op::aarch64_integer_conditional_compare_register)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            Some(Op::aarch64_integer_conditional_compare_register)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            Some(Op::aarch64_integer_conditional_compare_register)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            Some(Op::aarch64_integer_conditional_compare_register)
                        }
                        _ => None
                    }
                }
                (_, _, _, x3, _, x5, _, x7, _) if x3 == 1 && x5 == 2 && x7 & 2 == 2 => {
                    let sf = (instr >> 31) & 1;
                    let op = (instr >> 30) & 1;
                    let S = (instr >> 29) & 1;
                    let imm5 = (instr >> 16) & 9;
                    let cond = (instr >> 12) & 7;
                    let o2 = (instr >> 10) & 1;
                    let Rn = (instr >> 5) & 9;
                    let o3 = (instr >> 4) & 1;
                    let nzcv = instr & 7;
                    match (sf, op, S, o2, o3) {
                        (_, _, _, _, x4) if x4 == 1 => {
                            None
                        }
                        (_, _, _, x3, _) if x3 == 1 => {
                            None
                        }
                        (_, _, x2, _, _) if x2 == 0 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            Some(Op::aarch64_integer_conditional_compare_immediate)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            Some(Op::aarch64_integer_conditional_compare_immediate)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            Some(Op::aarch64_integer_conditional_compare_immediate)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            Some(Op::aarch64_integer_conditional_compare_immediate)
                        }
                        _ => None
                    }
                }
                (_, _, _, x3, _, x5, _, _, _) if x3 == 1 && x5 == 4 => {
                    let sf = (instr >> 31) & 1;
                    let op = (instr >> 30) & 1;
                    let S = (instr >> 29) & 1;
                    let Rm = (instr >> 16) & 9;
                    let cond = (instr >> 12) & 7;
                    let op2 = (instr >> 10) & 3;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (sf, op, S, op2) {
                        (_, _, _, x3) if x3 & 2 == 2 => {
                            None
                        }
                        (_, _, x2, _) if x2 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            Some(Op::aarch64_integer_conditional_select)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            Some(Op::aarch64_integer_conditional_select)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 0 => {
                            Some(Op::aarch64_integer_conditional_select)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 1 => {
                            Some(Op::aarch64_integer_conditional_select)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            Some(Op::aarch64_integer_conditional_select)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            Some(Op::aarch64_integer_conditional_select)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 0 => {
                            Some(Op::aarch64_integer_conditional_select)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 1 => {
                            Some(Op::aarch64_integer_conditional_select)
                        }
                        _ => None
                    }
                }
                (_, _, _, x3, _, x5, _, _, _) if x3 == 1 && x5 & 8 == 8 => {
                    let sf = (instr >> 31) & 1;
                    let op54 = (instr >> 29) & 3;
                    let op31 = (instr >> 21) & 5;
                    let Rm = (instr >> 16) & 9;
                    let o0 = (instr >> 15) & 1;
                    let Ra = (instr >> 10) & 9;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (sf, op54, op31, o0) {
                        (_, x1, x2, x3) if x1 == 0 && x2 == 2 && x3 == 1 => {
                            None
                        }
                        (_, x1, x2, _) if x1 == 0 && x2 == 3 => {
                            None
                        }
                        (_, x1, x2, _) if x1 == 0 && x2 == 4 => {
                            None
                        }
                        (_, x1, x2, x3) if x1 == 0 && x2 == 6 && x3 == 1 => {
                            None
                        }
                        (_, x1, x2, _) if x1 == 0 && x2 == 7 => {
                            None
                        }
                        (_, x1, _, _) if x1 == 1 => {
                            None
                        }
                        (_, x1, _, _) if x1 & 2 == 2 => {
                            None
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            Some(Op::aarch64_integer_arithmetic_mul_uniform_add_sub)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            Some(Op::aarch64_integer_arithmetic_mul_uniform_add_sub)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            None
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 2 && x3 == 0 => {
                            None
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 5 && x3 == 0 => {
                            None
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 5 && x3 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 6 && x3 == 0 => {
                            None
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            Some(Op::aarch64_integer_arithmetic_mul_uniform_add_sub)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            Some(Op::aarch64_integer_arithmetic_mul_uniform_add_sub)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            Some(Op::aarch64_integer_arithmetic_mul_widening_32_64)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 => {
                            Some(Op::aarch64_integer_arithmetic_mul_widening_32_64)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 2 && x3 == 0 => {
                            Some(Op::aarch64_integer_arithmetic_mul_widening_64_128hi)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 5 && x3 == 0 => {
                            Some(Op::aarch64_integer_arithmetic_mul_widening_32_64)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 5 && x3 == 1 => {
                            Some(Op::aarch64_integer_arithmetic_mul_widening_32_64)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 6 && x3 == 0 => {
                            Some(Op::aarch64_integer_arithmetic_mul_widening_64_128hi)
                        }
                        _ => None
                    }
                }
                _ => None
            }
        }
        (_, x1, _) if x1 & 14 == 14 => {
            match ((instr >> 28) & 7, (instr >> 25) & 5, (instr >> 23) & 3, (instr >> 19) & 7, (instr >> 10) & 17, instr & 19) {
                (x0, _, x2, x3, x4, _) if x0 == 0 && x2 & 2 == 0 && x3 & 7 == 5 && x4 & 387 == 2 => {
                    None
                }
                (x0, _, x2, x3, x4, _) if x0 == 2 && x2 & 2 == 0 && x3 & 7 == 5 && x4 & 387 == 2 => {
                    None
                }
                (x0, _, x2, x3, x4, _) if x0 == 4 && x2 & 2 == 0 && x3 & 7 == 5 && x4 & 387 == 2 => {
                    let size = (instr >> 22) & 3;
                    let opcode = (instr >> 12) & 9;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (size, opcode) {
                        (_, x1) if x1 & 8 == 8 => {
                            None
                        }
                        (_, x1) if x1 & 28 == 0 => {
                            None
                        }
                        (_, x1) if x1 & 16 == 16 => {
                            None
                        }
                        (x0, _) if x0 & 1 == 1 => {
                            None
                        }
                        (x0, x1) if x0 == 0 && x1 == 4 => {
                            Some(Op::aarch64_vector_crypto_aes_round)
                        }
                        (x0, x1) if x0 == 0 && x1 == 5 => {
                            Some(Op::aarch64_vector_crypto_aes_round)
                        }
                        (x0, x1) if x0 == 0 && x1 == 6 => {
                            Some(Op::aarch64_vector_crypto_aes_mix)
                        }
                        (x0, x1) if x0 == 0 && x1 == 7 => {
                            Some(Op::aarch64_vector_crypto_aes_mix)
                        }
                        (x0, _) if x0 & 2 == 2 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 == 5 && x2 & 2 == 0 && x3 & 4 == 0 && x4 & 35 == 0 => {
                    let size = (instr >> 22) & 3;
                    let Rm = (instr >> 16) & 9;
                    let opcode = (instr >> 12) & 5;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (size, opcode) {
                        (_, x1) if x1 == 7 => {
                            None
                        }
                        (x0, _) if x0 & 1 == 1 => {
                            None
                        }
                        (x0, x1) if x0 == 0 && x1 == 0 => {
                            Some(Op::aarch64_vector_crypto_sha3op_sha1_hash_choose)
                        }
                        (x0, x1) if x0 == 0 && x1 == 1 => {
                            Some(Op::aarch64_vector_crypto_sha3op_sha1_hash_parity)
                        }
                        (x0, x1) if x0 == 0 && x1 == 2 => {
                            Some(Op::aarch64_vector_crypto_sha3op_sha1_hash_majority)
                        }
                        (x0, x1) if x0 == 0 && x1 == 3 => {
                            Some(Op::aarch64_vector_crypto_sha3op_sha1_sched0)
                        }
                        (x0, x1) if x0 == 0 && x1 == 4 => {
                            Some(Op::aarch64_vector_crypto_sha3op_sha256_hash)
                        }
                        (x0, x1) if x0 == 0 && x1 == 5 => {
                            Some(Op::aarch64_vector_crypto_sha3op_sha256_hash)
                        }
                        (x0, x1) if x0 == 0 && x1 == 6 => {
                            Some(Op::aarch64_vector_crypto_sha3op_sha256_sched1)
                        }
                        (x0, _) if x0 & 2 == 2 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 == 5 && x2 & 2 == 0 && x3 & 4 == 0 && x4 & 35 == 2 => {
                    None
                }
                (x0, _, x2, x3, x4, _) if x0 == 5 && x2 & 2 == 0 && x3 & 7 == 5 && x4 & 387 == 2 => {
                    let size = (instr >> 22) & 3;
                    let opcode = (instr >> 12) & 9;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (size, opcode) {
                        (_, x1) if x1 & 4 == 4 => {
                            None
                        }
                        (_, x1) if x1 & 8 == 8 => {
                            None
                        }
                        (_, x1) if x1 & 16 == 16 => {
                            None
                        }
                        (x0, _) if x0 & 1 == 1 => {
                            None
                        }
                        (x0, x1) if x0 == 0 && x1 == 0 => {
                            Some(Op::aarch64_vector_crypto_sha2op_sha1_hash)
                        }
                        (x0, x1) if x0 == 0 && x1 == 1 => {
                            Some(Op::aarch64_vector_crypto_sha2op_sha1_sched1)
                        }
                        (x0, x1) if x0 == 0 && x1 == 2 => {
                            Some(Op::aarch64_vector_crypto_sha2op_sha256_sched0)
                        }
                        (x0, x1) if x0 == 0 && x1 == 3 => {
                            None
                        }
                        (x0, _) if x0 & 2 == 2 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 == 6 && x2 & 2 == 0 && x3 & 7 == 5 && x4 & 387 == 2 => {
                    None
                }
                (x0, _, x2, x3, x4, _) if x0 == 7 && x2 & 2 == 0 && x3 & 4 == 0 && x4 & 33 == 0 => {
                    None
                }
                (x0, _, x2, x3, x4, _) if x0 == 7 && x2 & 2 == 0 && x3 & 7 == 5 && x4 & 387 == 2 => {
                    None
                }
                (x0, _, x2, x3, x4, _) if x0 & 13 == 5 && x2 == 0 && x3 & 12 == 0 && x4 & 33 == 1 => {
                    let op = (instr >> 29) & 1;
                    let imm5 = (instr >> 16) & 9;
                    let imm4 = (instr >> 11) & 7;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (op, imm5, imm4) {
                        (x0, _, x2) if x0 == 0 && x2 & 1 == 1 => {
                            None
                        }
                        (x0, _, x2) if x0 == 0 && x2 & 2 == 2 => {
                            None
                        }
                        (x0, _, x2) if x0 == 0 && x2 & 4 == 4 => {
                            None
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 0 => {
                            Some(Op::aarch64_vector_transfer_vector_cpy_dup_sisd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 & 8 == 8 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 15 == 0 && x2 == 0 => {
                            None
                        }
                        (x0, _, _) if x0 == 1 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 & 13 == 5 && x2 == 1 && x3 & 12 == 0 && x4 & 33 == 1 => {
                    None
                }
                (x0, _, x2, x3, x4, _) if x0 & 13 == 5 && x2 & 2 == 0 && x3 == 7 && x4 & 387 == 2 => {
                    None
                }
                (x0, _, x2, x3, x4, _) if x0 & 13 == 5 && x2 & 2 == 0 && x3 & 12 == 8 && x4 & 49 == 1 => {
                    let U = (instr >> 29) & 1;
                    let a = (instr >> 23) & 1;
                    let Rm = (instr >> 16) & 9;
                    let opcode = (instr >> 11) & 5;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (U, a, opcode) {
                        (_, _, x2) if x2 == 6 => {
                            None
                        }
                        (_, x1, x2) if x1 == 1 && x2 == 3 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 3 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_mul_fp16_extended_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 4 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 5 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 7 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_recps_fp16_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 4 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 5 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 7 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_rsqrts_fp16_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 3 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 4 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 5 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 7 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 2 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 4 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 5 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 7 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 & 13 == 5 && x2 & 2 == 0 && x3 & 12 == 8 && x4 & 49 == 17 => {
                    None
                }
                (x0, _, x2, x3, x4, _) if x0 & 13 == 5 && x2 & 2 == 0 && x3 == 15 && x4 & 387 == 2 => {
                    let U = (instr >> 29) & 1;
                    let a = (instr >> 23) & 1;
                    let opcode = (instr >> 12) & 9;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (U, a, opcode) {
                        (_, _, x2) if x2 & 24 == 0 => {
                            None
                        }
                        (_, _, x2) if x2 & 28 == 8 => {
                            None
                        }
                        (_, _, x2) if x2 & 24 == 16 => {
                            None
                        }
                        (_, _, x2) if x2 & 30 == 24 => {
                            None
                        }
                        (_, _, x2) if x2 == 30 => {
                            None
                        }
                        (_, x1, x2) if x1 == 0 && x2 & 28 == 12 => {
                            None
                        }
                        (_, x1, x2) if x1 == 0 && x2 == 31 => {
                            None
                        }
                        (_, x1, x2) if x1 == 1 && x2 == 15 => {
                            None
                        }
                        (_, x1, x2) if x1 == 1 && x2 == 28 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 26 => {
                            Some(Op::aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 27 => {
                            Some(Op::aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 28 => {
                            Some(Op::aarch64_vector_arithmetic_unary_fp16_conv_float_tieaway_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 29 => {
                            Some(Op::aarch64_vector_arithmetic_unary_fp16_conv_int_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 12 => {
                            Some(Op::aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 13 => {
                            Some(Op::aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 14 => {
                            Some(Op::aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 26 => {
                            Some(Op::aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 27 => {
                            Some(Op::aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 29 => {
                            Some(Op::aarch64_vector_arithmetic_unary_special_recip_fp16_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 31 => {
                            Some(Op::aarch64_vector_arithmetic_unary_special_frecpx_fp16)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 26 => {
                            Some(Op::aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 27 => {
                            Some(Op::aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 28 => {
                            Some(Op::aarch64_vector_arithmetic_unary_fp16_conv_float_tieaway_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 29 => {
                            Some(Op::aarch64_vector_arithmetic_unary_fp16_conv_int_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 12 => {
                            Some(Op::aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 13 => {
                            Some(Op::aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 14 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 26 => {
                            Some(Op::aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 27 => {
                            Some(Op::aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 29 => {
                            Some(Op::aarch64_vector_arithmetic_unary_special_sqrt_est_fp16_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 31 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 & 13 == 5 && x2 & 2 == 0 && x3 & 4 == 0 && x4 & 33 == 32 => {
                    None
                }
                (x0, _, x2, x3, x4, _) if x0 & 13 == 5 && x2 & 2 == 0 && x3 & 4 == 0 && x4 & 33 == 33 => {
                    let U = (instr >> 29) & 1;
                    let size = (instr >> 22) & 3;
                    let Rm = (instr >> 16) & 9;
                    let opcode = (instr >> 11) & 7;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (U, opcode) {
                        (_, x1) if x1 & 14 == 2 => {
                            None
                        }
                        (_, x1) if x1 & 12 == 4 => {
                            None
                        }
                        (_, x1) if x1 & 8 == 8 => {
                            None
                        }
                        (x0, x1) if x0 == 0 && x1 == 0 => {
                            None
                        }
                        (x0, x1) if x0 == 0 && x1 == 1 => {
                            None
                        }
                        (x0, x1) if x0 == 1 && x1 == 0 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd)
                        }
                        (x0, x1) if x0 == 1 && x1 == 1 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd)
                        }
                        _ => None
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 & 13 == 5 && x2 & 2 == 0 && x3 & 7 == 4 && x4 & 387 == 2 => {
                    let U = (instr >> 29) & 1;
                    let size = (instr >> 22) & 3;
                    let opcode = (instr >> 12) & 9;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (U, size, opcode) {
                        (_, _, x2) if x2 & 30 == 0 => {
                            None
                        }
                        (_, _, x2) if x2 == 2 => {
                            None
                        }
                        (_, _, x2) if x2 & 30 == 4 => {
                            None
                        }
                        (_, _, x2) if x2 == 6 => {
                            None
                        }
                        (_, _, x2) if x2 == 15 => {
                            None
                        }
                        (_, _, x2) if x2 & 30 == 16 => {
                            None
                        }
                        (_, _, x2) if x2 == 19 => {
                            None
                        }
                        (_, _, x2) if x2 == 21 => {
                            None
                        }
                        (_, _, x2) if x2 == 23 => {
                            None
                        }
                        (_, _, x2) if x2 & 30 == 24 => {
                            None
                        }
                        (_, _, x2) if x2 == 30 => {
                            None
                        }
                        (_, x1, x2) if x1 & 2 == 0 && x2 & 28 == 12 => {
                            None
                        }
                        (_, x1, x2) if x1 & 2 == 0 && x2 == 31 => {
                            None
                        }
                        (_, x1, x2) if x1 & 2 == 2 && x2 == 22 => {
                            None
                        }
                        (_, x1, x2) if x1 & 2 == 2 && x2 == 28 => {
                            None
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 3 => {
                            Some(Op::aarch64_vector_arithmetic_unary_add_saturating_sisd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 7 => {
                            Some(Op::aarch64_vector_arithmetic_unary_diff_neg_sat_sisd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 8 => {
                            Some(Op::aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 9 => {
                            Some(Op::aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 10 => {
                            Some(Op::aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 11 => {
                            Some(Op::aarch64_vector_arithmetic_unary_diff_neg_int_sisd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 18 => {
                            None
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 20 => {
                            Some(Op::aarch64_vector_arithmetic_unary_extract_sat_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 22 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 26 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_conv_float_bulk_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 27 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_conv_float_bulk_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 28 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_conv_float_tieaway_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 29 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_conv_int_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 12 => {
                            Some(Op::aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 13 => {
                            Some(Op::aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 14 => {
                            Some(Op::aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 26 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_conv_float_bulk_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 27 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_conv_float_bulk_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 29 => {
                            Some(Op::aarch64_vector_arithmetic_unary_special_recip_float_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 31 => {
                            Some(Op::aarch64_vector_arithmetic_unary_special_frecpx)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 3 => {
                            Some(Op::aarch64_vector_arithmetic_unary_add_saturating_sisd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 7 => {
                            Some(Op::aarch64_vector_arithmetic_unary_diff_neg_sat_sisd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 8 => {
                            Some(Op::aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 9 => {
                            Some(Op::aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 10 => {
                            None
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 11 => {
                            Some(Op::aarch64_vector_arithmetic_unary_diff_neg_int_sisd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 18 => {
                            Some(Op::aarch64_vector_arithmetic_unary_extract_sqxtun_sisd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 20 => {
                            Some(Op::aarch64_vector_arithmetic_unary_extract_sat_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 22 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_xtn_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 26 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_conv_float_bulk_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 27 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_conv_float_bulk_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 28 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_conv_float_tieaway_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 29 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_conv_int_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 12 => {
                            Some(Op::aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 13 => {
                            Some(Op::aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 14 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 26 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_conv_float_bulk_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 27 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_conv_float_bulk_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 29 => {
                            Some(Op::aarch64_vector_arithmetic_unary_special_sqrt_est_float_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 31 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 & 13 == 5 && x2 & 2 == 0 && x3 & 7 == 6 && x4 & 387 == 2 => {
                    let U = (instr >> 29) & 1;
                    let size = (instr >> 22) & 3;
                    let opcode = (instr >> 12) & 9;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (U, size, opcode) {
                        (_, _, x2) if x2 & 24 == 0 => {
                            None
                        }
                        (_, _, x2) if x2 & 28 == 8 => {
                            None
                        }
                        (_, _, x2) if x2 == 14 => {
                            None
                        }
                        (_, _, x2) if x2 & 24 == 16 => {
                            None
                        }
                        (_, _, x2) if x2 & 30 == 24 => {
                            None
                        }
                        (_, _, x2) if x2 == 26 => {
                            None
                        }
                        (_, _, x2) if x2 & 28 == 28 => {
                            None
                        }
                        (_, x1, x2) if x1 & 2 == 2 && x2 == 13 => {
                            None
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 27 => {
                            Some(Op::aarch64_vector_reduce_add_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 12 => {
                            Some(Op::aarch64_vector_reduce_fp16_maxnm_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 13 => {
                            Some(Op::aarch64_vector_reduce_fp16_add_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 15 => {
                            Some(Op::aarch64_vector_reduce_fp16_max_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 12 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 13 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 15 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 12 => {
                            Some(Op::aarch64_vector_reduce_fp16_maxnm_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 15 => {
                            Some(Op::aarch64_vector_reduce_fp16_max_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 12 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 15 => {
                            None
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 27 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 12 => {
                            Some(Op::aarch64_vector_reduce_fp_maxnm_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 13 => {
                            Some(Op::aarch64_vector_reduce_fp_add_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 15 => {
                            Some(Op::aarch64_vector_reduce_fp_max_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 12 => {
                            Some(Op::aarch64_vector_reduce_fp_maxnm_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 15 => {
                            Some(Op::aarch64_vector_reduce_fp_max_sisd)
                        }
                        _ => None
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 & 13 == 5 && x2 & 2 == 0 && x3 & 4 == 4 && x4 & 259 == 258 => {
                    None
                }
                (x0, _, x2, x3, x4, _) if x0 & 13 == 5 && x2 & 2 == 0 && x3 & 4 == 4 && x4 & 131 == 130 => {
                    None
                }
                (x0, _, x2, x3, x4, _) if x0 & 13 == 5 && x2 & 2 == 0 && x3 & 4 == 4 && x4 & 3 == 0 => {
                    let U = (instr >> 29) & 1;
                    let size = (instr >> 22) & 3;
                    let Rm = (instr >> 16) & 9;
                    let opcode = (instr >> 12) & 7;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (U, opcode) {
                        (_, x1) if x1 & 12 == 0 => {
                            None
                        }
                        (_, x1) if x1 & 12 == 4 => {
                            None
                        }
                        (_, x1) if x1 == 8 => {
                            None
                        }
                        (_, x1) if x1 == 10 => {
                            None
                        }
                        (_, x1) if x1 == 12 => {
                            None
                        }
                        (_, x1) if x1 & 14 == 14 => {
                            None
                        }
                        (x0, x1) if x0 == 0 && x1 == 9 => {
                            Some(Op::aarch64_vector_arithmetic_binary_disparate_mul_dmacc_sisd)
                        }
                        (x0, x1) if x0 == 0 && x1 == 11 => {
                            Some(Op::aarch64_vector_arithmetic_binary_disparate_mul_dmacc_sisd)
                        }
                        (x0, x1) if x0 == 0 && x1 == 13 => {
                            Some(Op::aarch64_vector_arithmetic_binary_disparate_mul_double_sisd)
                        }
                        (x0, x1) if x0 == 1 && x1 == 9 => {
                            None
                        }
                        (x0, x1) if x0 == 1 && x1 == 11 => {
                            None
                        }
                        (x0, x1) if x0 == 1 && x1 == 13 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 & 13 == 5 && x2 & 2 == 0 && x3 & 4 == 4 && x4 & 1 == 1 => {
                    let U = (instr >> 29) & 1;
                    let size = (instr >> 22) & 3;
                    let Rm = (instr >> 16) & 9;
                    let opcode = (instr >> 11) & 9;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (U, size, opcode) {
                        (_, _, x2) if x2 == 0 => {
                            None
                        }
                        (_, _, x2) if x2 & 30 == 2 => {
                            None
                        }
                        (_, _, x2) if x2 == 4 => {
                            None
                        }
                        (_, _, x2) if x2 & 28 == 12 => {
                            None
                        }
                        (_, _, x2) if x2 & 30 == 18 => {
                            None
                        }
                        (_, x1, x2) if x1 & 2 == 2 && x2 == 27 => {
                            None
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 1 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 5 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 6 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 7 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 8 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_shift_sisd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 9 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_shift_sisd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 10 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_shift_sisd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 11 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_shift_sisd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 16 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 17 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 20 => {
                            None
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 21 => {
                            None
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 22 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_sisd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 23 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 24 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 25 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 26 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 27 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_mul_fp_extended_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 28 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 29 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 30 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 31 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_recps_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 24 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 25 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 26 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 28 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 29 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 30 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 31 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_rsqrts_sisd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 1 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 5 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 6 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 7 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 8 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_shift_sisd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 9 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_shift_sisd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 10 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_shift_sisd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 11 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_shift_sisd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 16 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 17 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 20 => {
                            None
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 21 => {
                            None
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 22 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_sisd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 23 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 24 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 25 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 26 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 27 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 28 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 29 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 30 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 31 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 24 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 25 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 26 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 28 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 29 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 30 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 31 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _) if x0 & 13 == 5 && x2 == 2 && x4 & 1 == 1 => {
                    let U = (instr >> 29) & 1;
                    let immh = (instr >> 19) & 7;
                    let immb = (instr >> 16) & 5;
                    let opcode = (instr >> 11) & 9;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (U, immh, opcode) {
                        (_, x1, x2) if x1 != 0 && x2 == 1 => {
                            None
                        }
                        (_, x1, x2) if x1 != 0 && x2 == 3 => {
                            None
                        }
                        (_, x1, x2) if x1 != 0 && x2 == 5 => {
                            None
                        }
                        (_, x1, x2) if x1 != 0 && x2 == 7 => {
                            None
                        }
                        (_, x1, x2) if x1 != 0 && x2 == 9 => {
                            None
                        }
                        (_, x1, x2) if x1 != 0 && x2 == 11 => {
                            None
                        }
                        (_, x1, x2) if x1 != 0 && x2 == 13 => {
                            None
                        }
                        (_, x1, x2) if x1 != 0 && x2 == 15 => {
                            None
                        }
                        (_, x1, x2) if x1 != 0 && x2 & 28 == 20 => {
                            None
                        }
                        (_, x1, x2) if x1 != 0 && x2 & 28 == 24 => {
                            None
                        }
                        (_, x1, x2) if x1 != 0 && x2 == 29 => {
                            None
                        }
                        (_, x1, x2) if x1 != 0 && x2 == 30 => {
                            None
                        }
                        (_, x1, _) if x1 == 0 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 0 => {
                            Some(Op::aarch64_vector_shift_right_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 2 => {
                            Some(Op::aarch64_vector_shift_right_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 4 => {
                            Some(Op::aarch64_vector_shift_right_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 6 => {
                            Some(Op::aarch64_vector_shift_right_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 8 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 10 => {
                            Some(Op::aarch64_vector_shift_left_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 12 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 14 => {
                            Some(Op::aarch64_vector_shift_left_sat_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 16 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 17 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 18 => {
                            Some(Op::aarch64_vector_shift_right_narrow_uniform_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 19 => {
                            Some(Op::aarch64_vector_shift_right_narrow_uniform_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 28 => {
                            Some(Op::aarch64_vector_shift_conv_int_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 31 => {
                            Some(Op::aarch64_vector_shift_conv_float_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 0 => {
                            Some(Op::aarch64_vector_shift_right_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 2 => {
                            Some(Op::aarch64_vector_shift_right_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 4 => {
                            Some(Op::aarch64_vector_shift_right_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 6 => {
                            Some(Op::aarch64_vector_shift_right_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 8 => {
                            Some(Op::aarch64_vector_shift_right_insert_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 10 => {
                            Some(Op::aarch64_vector_shift_left_insert_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 12 => {
                            Some(Op::aarch64_vector_shift_left_sat_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 14 => {
                            Some(Op::aarch64_vector_shift_left_sat_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 16 => {
                            Some(Op::aarch64_vector_shift_right_narrow_nonuniform_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 17 => {
                            Some(Op::aarch64_vector_shift_right_narrow_nonuniform_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 18 => {
                            Some(Op::aarch64_vector_shift_right_narrow_uniform_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 19 => {
                            Some(Op::aarch64_vector_shift_right_narrow_uniform_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 28 => {
                            Some(Op::aarch64_vector_shift_conv_int_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 31 => {
                            Some(Op::aarch64_vector_shift_conv_float_sisd)
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _) if x0 & 13 == 5 && x2 == 3 && x4 & 1 == 1 => {
                    None
                }
                (x0, _, x2, _, x4, _) if x0 & 13 == 5 && x2 & 2 == 2 && x4 & 1 == 0 => {
                    let U = (instr >> 29) & 1;
                    let size = (instr >> 22) & 3;
                    let L = (instr >> 21) & 1;
                    let M = (instr >> 20) & 1;
                    let Rm = (instr >> 16) & 7;
                    let opcode = (instr >> 12) & 7;
                    let H = (instr >> 11) & 1;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (U, size, opcode) {
                        (_, _, x2) if x2 == 0 => {
                            None
                        }
                        (_, _, x2) if x2 == 2 => {
                            None
                        }
                        (_, _, x2) if x2 == 4 => {
                            None
                        }
                        (_, _, x2) if x2 == 6 => {
                            None
                        }
                        (_, _, x2) if x2 == 8 => {
                            None
                        }
                        (_, _, x2) if x2 == 10 => {
                            None
                        }
                        (_, _, x2) if x2 == 14 => {
                            None
                        }
                        (_, x1, x2) if x1 == 1 && x2 == 1 => {
                            None
                        }
                        (_, x1, x2) if x1 == 1 && x2 == 5 => {
                            None
                        }
                        (_, x1, x2) if x1 == 1 && x2 == 9 => {
                            None
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 3 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_acc_double_sisd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 7 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_acc_double_sisd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 11 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_double_sisd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 12 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_high_sisd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 13 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_high_sisd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 15 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_acc_fp16_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 5 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_acc_fp16_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 9 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_fp16_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 1 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_acc_fp_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 5 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_acc_fp_sisd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 9 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_fp_sisd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 3 => {
                            None
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 7 => {
                            None
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 11 => {
                            None
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 12 => {
                            None
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 13 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_acc_high_sisd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 15 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_acc_high_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 5 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 9 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_fp16_sisd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 1 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 5 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 9 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_fp_sisd)
                        }
                        _ => None
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 & 11 == 0 && x2 & 2 == 0 && x3 & 4 == 0 && x4 & 35 == 0 => {
                    let Q = (instr >> 30) & 1;
                    let op2 = (instr >> 22) & 3;
                    let Rm = (instr >> 16) & 9;
                    let len = (instr >> 13) & 3;
                    let op = (instr >> 12) & 1;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (op2, len, op) {
                        (x0, _, _) if x0 & 1 == 1 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_vector_transfer_vector_table)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_vector_transfer_vector_table)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_vector_transfer_vector_table)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_vector_transfer_vector_table)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 0 => {
                            Some(Op::aarch64_vector_transfer_vector_table)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 1 => {
                            Some(Op::aarch64_vector_transfer_vector_table)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 0 => {
                            Some(Op::aarch64_vector_transfer_vector_table)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 1 => {
                            Some(Op::aarch64_vector_transfer_vector_table)
                        }
                        (x0, _, _) if x0 & 2 == 2 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 & 11 == 0 && x2 & 2 == 0 && x3 & 4 == 0 && x4 & 35 == 2 => {
                    let Q = (instr >> 30) & 1;
                    let size = (instr >> 22) & 3;
                    let Rm = (instr >> 16) & 9;
                    let opcode = (instr >> 12) & 5;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match opcode {
                        x0 if x0 == 0 => {
                            None
                        }
                        x0 if x0 == 1 => {
                            Some(Op::aarch64_vector_transfer_vector_permute_unzip)
                        }
                        x0 if x0 == 2 => {
                            Some(Op::aarch64_vector_transfer_vector_permute_transpose)
                        }
                        x0 if x0 == 3 => {
                            Some(Op::aarch64_vector_transfer_vector_permute_zip)
                        }
                        x0 if x0 == 4 => {
                            None
                        }
                        x0 if x0 == 5 => {
                            Some(Op::aarch64_vector_transfer_vector_permute_unzip)
                        }
                        x0 if x0 == 6 => {
                            Some(Op::aarch64_vector_transfer_vector_permute_transpose)
                        }
                        x0 if x0 == 7 => {
                            Some(Op::aarch64_vector_transfer_vector_permute_zip)
                        }
                        _ => None
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 & 11 == 2 && x2 & 2 == 0 && x3 & 4 == 0 && x4 & 33 == 0 => {
                    let Q = (instr >> 30) & 1;
                    let op2 = (instr >> 22) & 3;
                    let Rm = (instr >> 16) & 9;
                    let imm4 = (instr >> 11) & 7;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match op2 {
                        x0 if x0 & 1 == 1 => {
                            None
                        }
                        x0 if x0 == 0 => {
                            Some(Op::aarch64_vector_transfer_vector_extract)
                        }
                        x0 if x0 & 2 == 2 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 & 9 == 0 && x2 == 0 && x3 & 12 == 0 && x4 & 33 == 1 => {
                    let Q = (instr >> 30) & 1;
                    let op = (instr >> 29) & 1;
                    let imm5 = (instr >> 16) & 9;
                    let imm4 = (instr >> 11) & 7;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (Q, op, imm5, imm4) {
                        (_, _, x2, _) if x2 & 15 == 0 => {
                            None
                        }
                        (_, x1, _, x3) if x1 == 0 && x3 == 0 => {
                            Some(Op::aarch64_vector_transfer_vector_cpy_dup_simd)
                        }
                        (_, x1, _, x3) if x1 == 0 && x3 == 1 => {
                            Some(Op::aarch64_vector_transfer_integer_dup)
                        }
                        (_, x1, _, x3) if x1 == 0 && x3 == 2 => {
                            None
                        }
                        (_, x1, _, x3) if x1 == 0 && x3 == 4 => {
                            None
                        }
                        (_, x1, _, x3) if x1 == 0 && x3 == 6 => {
                            None
                        }
                        (_, x1, _, x3) if x1 == 0 && x3 & 8 == 8 => {
                            None
                        }
                        (x0, x1, _, x3) if x0 == 0 && x1 == 0 && x3 == 3 => {
                            None
                        }
                        (x0, x1, _, x3) if x0 == 0 && x1 == 0 && x3 == 5 => {
                            Some(Op::aarch64_vector_transfer_integer_move_signed)
                        }
                        (x0, x1, _, x3) if x0 == 0 && x1 == 0 && x3 == 7 => {
                            Some(Op::aarch64_vector_transfer_integer_move_unsigned)
                        }
                        (x0, x1, _, _) if x0 == 0 && x1 == 1 => {
                            None
                        }
                        (x0, x1, _, x3) if x0 == 1 && x1 == 0 && x3 == 3 => {
                            Some(Op::aarch64_vector_transfer_integer_insert)
                        }
                        (x0, x1, _, x3) if x0 == 1 && x1 == 0 && x3 == 5 => {
                            Some(Op::aarch64_vector_transfer_integer_move_signed)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 & 15 == 8 && x3 == 7 => {
                            Some(Op::aarch64_vector_transfer_integer_move_unsigned)
                        }
                        (x0, x1, _, _) if x0 == 1 && x1 == 1 => {
                            Some(Op::aarch64_vector_transfer_vector_insert)
                        }
                        _ => None
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 & 9 == 0 && x2 == 1 && x3 & 12 == 0 && x4 & 33 == 1 => {
                    None
                }
                (x0, _, x2, x3, x4, _) if x0 & 9 == 0 && x2 & 2 == 0 && x3 == 7 && x4 & 387 == 2 => {
                    None
                }
                (x0, _, x2, x3, x4, _) if x0 & 9 == 0 && x2 & 2 == 0 && x3 & 12 == 8 && x4 & 49 == 1 => {
                    let Q = (instr >> 30) & 1;
                    let U = (instr >> 29) & 1;
                    let a = (instr >> 23) & 1;
                    let Rm = (instr >> 16) & 9;
                    let opcode = (instr >> 11) & 5;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (U, a, opcode) {
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_max_min_fp16_2008)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_mul_fp16_fused)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 2 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_add_fp16)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 3 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_mul_fp16_extended_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 4 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 5 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 6 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_max_min_fp16_1985)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 7 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_recps_fp16_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_max_min_fp16_2008)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_mul_fp16_fused)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 2 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 3 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 4 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 5 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 6 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_max_min_fp16_1985)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 7 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_rsqrts_fp16_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_max_min_fp16_2008)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 2 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_add_fp16)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 3 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_mul_fp16_product)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 4 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 5 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 6 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_max_min_fp16_1985)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 7 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_div_fp16)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_max_min_fp16_2008)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 2 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 3 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 4 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 5 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 6 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_max_min_fp16_1985)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 7 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 & 9 == 0 && x2 & 2 == 0 && x3 & 12 == 8 && x4 & 49 == 17 => {
                    None
                }
                (x0, _, x2, x3, x4, _) if x0 & 9 == 0 && x2 & 2 == 0 && x3 == 15 && x4 & 387 == 2 => {
                    let Q = (instr >> 30) & 1;
                    let U = (instr >> 29) & 1;
                    let a = (instr >> 23) & 1;
                    let opcode = (instr >> 12) & 9;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (U, a, opcode) {
                        (_, _, x2) if x2 & 24 == 0 => {
                            None
                        }
                        (_, _, x2) if x2 & 28 == 8 => {
                            None
                        }
                        (_, _, x2) if x2 & 24 == 16 => {
                            None
                        }
                        (_, _, x2) if x2 == 30 => {
                            None
                        }
                        (_, x1, x2) if x1 == 0 && x2 & 28 == 12 => {
                            None
                        }
                        (_, x1, x2) if x1 == 0 && x2 == 31 => {
                            None
                        }
                        (_, x1, x2) if x1 == 1 && x2 == 28 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 24 => {
                            Some(Op::aarch64_vector_arithmetic_unary_fp16_round)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 25 => {
                            Some(Op::aarch64_vector_arithmetic_unary_fp16_round)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 26 => {
                            Some(Op::aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 27 => {
                            Some(Op::aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 28 => {
                            Some(Op::aarch64_vector_arithmetic_unary_fp16_conv_float_tieaway_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 29 => {
                            Some(Op::aarch64_vector_arithmetic_unary_fp16_conv_int_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 12 => {
                            Some(Op::aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 13 => {
                            Some(Op::aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 14 => {
                            Some(Op::aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 15 => {
                            Some(Op::aarch64_vector_arithmetic_unary_diff_neg_fp16)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 24 => {
                            Some(Op::aarch64_vector_arithmetic_unary_fp16_round)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 25 => {
                            Some(Op::aarch64_vector_arithmetic_unary_fp16_round)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 26 => {
                            Some(Op::aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 27 => {
                            Some(Op::aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 29 => {
                            Some(Op::aarch64_vector_arithmetic_unary_special_recip_fp16_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 31 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 24 => {
                            Some(Op::aarch64_vector_arithmetic_unary_fp16_round)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 25 => {
                            Some(Op::aarch64_vector_arithmetic_unary_fp16_round)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 26 => {
                            Some(Op::aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 27 => {
                            Some(Op::aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 28 => {
                            Some(Op::aarch64_vector_arithmetic_unary_fp16_conv_float_tieaway_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 29 => {
                            Some(Op::aarch64_vector_arithmetic_unary_fp16_conv_int_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 12 => {
                            Some(Op::aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 13 => {
                            Some(Op::aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 14 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 15 => {
                            Some(Op::aarch64_vector_arithmetic_unary_diff_neg_fp16)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 24 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 25 => {
                            Some(Op::aarch64_vector_arithmetic_unary_fp16_round)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 26 => {
                            Some(Op::aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 27 => {
                            Some(Op::aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 29 => {
                            Some(Op::aarch64_vector_arithmetic_unary_special_sqrt_est_fp16_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 31 => {
                            Some(Op::aarch64_vector_arithmetic_unary_special_sqrt_fp16)
                        }
                        _ => None
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 & 9 == 0 && x2 & 2 == 0 && x3 & 4 == 0 && x4 & 33 == 32 => {
                    None
                }
                (x0, _, x2, x3, x4, _) if x0 & 9 == 0 && x2 & 2 == 0 && x3 & 4 == 0 && x4 & 33 == 33 => {
                    let Q = (instr >> 30) & 1;
                    let U = (instr >> 29) & 1;
                    let size = (instr >> 22) & 3;
                    let Rm = (instr >> 16) & 9;
                    let opcode = (instr >> 11) & 7;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (Q, U, size, opcode) {
                        (_, _, x2, x3) if x2 & 2 == 0 && x3 == 3 => {
                            None
                        }
                        (_, _, x2, x3) if x2 == 3 && x3 == 3 => {
                            None
                        }
                        (_, x1, _, x3) if x1 == 0 && x3 == 0 => {
                            None
                        }
                        (_, x1, _, x3) if x1 == 0 && x3 == 1 => {
                            None
                        }
                        (_, x1, _, x3) if x1 == 0 && x3 == 2 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_mul_int_dotp)
                        }
                        (_, x1, _, x3) if x1 == 0 && x3 & 8 == 8 => {
                            None
                        }
                        (_, x1, x2, x3) if x1 == 0 && x2 == 2 && x3 == 3 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_mat_mul_int_usdot)
                        }
                        (_, x1, _, x3) if x1 == 1 && x3 == 0 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_accum_simd)
                        }
                        (_, x1, _, x3) if x1 == 1 && x3 == 1 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_accum_simd)
                        }
                        (_, x1, _, x3) if x1 == 1 && x3 == 2 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_mul_int_dotp)
                        }
                        (_, x1, _, x3) if x1 == 1 && x3 & 12 == 8 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_mul_fp_complex)
                        }
                        (_, x1, _, x3) if x1 == 1 && x3 & 13 == 12 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_add_fp_complex)
                        }
                        (_, x1, x2, x3) if x1 == 1 && x2 == 0 && x3 == 13 => {
                            None
                        }
                        (_, x1, x2, x3) if x1 == 1 && x2 == 0 && x3 == 15 => {
                            None
                        }
                        (_, x1, x2, x3) if x1 == 1 && x2 == 1 && x3 == 15 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_mul_int_bfdot)
                        }
                        (_, x1, x2, x3) if x1 == 1 && x2 & 2 == 2 && x3 == 13 => {
                            None
                        }
                        (_, x1, x2, x3) if x1 == 1 && x2 == 2 && x3 == 3 => {
                            None
                        }
                        (_, x1, x2, x3) if x1 == 1 && x2 == 2 && x3 == 15 => {
                            None
                        }
                        (_, x1, x2, x3) if x1 == 1 && x2 == 3 && x3 == 15 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_mul_acc_bf16_long)
                        }
                        (x0, _, _, x3) if x0 == 0 && x3 & 12 == 4 => {
                            None
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 13 => {
                            None
                        }
                        (x0, _, x2, x3) if x0 == 1 && x2 & 2 == 0 && x3 & 12 == 4 => {
                            None
                        }
                        (x0, _, x2, x3) if x0 == 1 && x2 & 2 == 2 && x3 & 14 == 6 => {
                            None
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 2 && x3 == 4 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_mat_mul_int_mla)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 2 && x3 == 5 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_mat_mul_int_mla)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 13 => {
                            Some(Op::aarch64_vector_bfmmla)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 2 && x3 == 4 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_mat_mul_int_mla)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 2 && x3 == 5 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 & 9 == 0 && x2 & 2 == 0 && x3 & 7 == 4 && x4 & 387 == 2 => {
                    let Q = (instr >> 30) & 1;
                    let U = (instr >> 29) & 1;
                    let size = (instr >> 22) & 3;
                    let opcode = (instr >> 12) & 9;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (U, size, opcode) {
                        (_, _, x2) if x2 & 30 == 16 => {
                            None
                        }
                        (_, _, x2) if x2 == 21 => {
                            None
                        }
                        (_, x1, x2) if x1 & 2 == 0 && x2 & 28 == 12 => {
                            None
                        }
                        (_, x1, x2) if x1 & 2 == 2 && x2 == 23 => {
                            None
                        }
                        (_, x1, x2) if x1 & 2 == 2 && x2 == 30 => {
                            None
                        }
                        (_, x1, x2) if x1 == 3 && x2 == 22 => {
                            None
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 0 => {
                            Some(Op::aarch64_vector_arithmetic_unary_rev)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 1 => {
                            Some(Op::aarch64_vector_arithmetic_unary_rev)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 2 => {
                            Some(Op::aarch64_vector_arithmetic_unary_add_pairwise)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 3 => {
                            Some(Op::aarch64_vector_arithmetic_unary_add_saturating_simd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 4 => {
                            Some(Op::aarch64_vector_arithmetic_unary_clsz)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 5 => {
                            Some(Op::aarch64_vector_arithmetic_unary_cnt)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 6 => {
                            Some(Op::aarch64_vector_arithmetic_unary_add_pairwise)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 7 => {
                            Some(Op::aarch64_vector_arithmetic_unary_diff_neg_sat_simd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 8 => {
                            Some(Op::aarch64_vector_arithmetic_unary_cmp_int_bulk_simd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 9 => {
                            Some(Op::aarch64_vector_arithmetic_unary_cmp_int_bulk_simd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 10 => {
                            Some(Op::aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 11 => {
                            Some(Op::aarch64_vector_arithmetic_unary_diff_neg_int_simd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 18 => {
                            Some(Op::aarch64_vector_arithmetic_unary_extract_nosat)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 19 => {
                            None
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 20 => {
                            Some(Op::aarch64_vector_arithmetic_unary_extract_sat_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 22 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_narrow)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 23 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_widen)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 24 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_round)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 25 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_round)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 26 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_conv_float_bulk_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 27 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_conv_float_bulk_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 28 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_conv_float_tieaway_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 29 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_conv_int_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 30 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_round_frint_32_64)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 31 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_round_frint_32_64)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 12 => {
                            Some(Op::aarch64_vector_arithmetic_unary_cmp_float_bulk_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 13 => {
                            Some(Op::aarch64_vector_arithmetic_unary_cmp_float_bulk_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 14 => {
                            Some(Op::aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 15 => {
                            Some(Op::aarch64_vector_arithmetic_unary_diff_neg_float)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 24 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_round)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 25 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_round)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 26 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_conv_float_bulk_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 27 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_conv_float_bulk_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 28 => {
                            Some(Op::aarch64_vector_arithmetic_unary_special_recip_int)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 29 => {
                            Some(Op::aarch64_vector_arithmetic_unary_special_recip_float_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 31 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 22 => {
                            Some(Op::aarch64_vector_cvt_bf16_vector)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 0 => {
                            Some(Op::aarch64_vector_arithmetic_unary_rev)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 1 => {
                            None
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 2 => {
                            Some(Op::aarch64_vector_arithmetic_unary_add_pairwise)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 3 => {
                            Some(Op::aarch64_vector_arithmetic_unary_add_saturating_simd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 4 => {
                            Some(Op::aarch64_vector_arithmetic_unary_clsz)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 6 => {
                            Some(Op::aarch64_vector_arithmetic_unary_add_pairwise)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 7 => {
                            Some(Op::aarch64_vector_arithmetic_unary_diff_neg_sat_simd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 8 => {
                            Some(Op::aarch64_vector_arithmetic_unary_cmp_int_bulk_simd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 9 => {
                            Some(Op::aarch64_vector_arithmetic_unary_cmp_int_bulk_simd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 10 => {
                            None
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 11 => {
                            Some(Op::aarch64_vector_arithmetic_unary_diff_neg_int_simd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 18 => {
                            Some(Op::aarch64_vector_arithmetic_unary_extract_sqxtun_simd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 19 => {
                            Some(Op::aarch64_vector_arithmetic_unary_shift)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 20 => {
                            Some(Op::aarch64_vector_arithmetic_unary_extract_sat_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 22 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_xtn_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 23 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 24 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_round)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 25 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_round)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 26 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_conv_float_bulk_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 27 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_conv_float_bulk_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 28 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_conv_float_tieaway_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 29 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_conv_int_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 30 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_round_frint_32_64)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 31 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_round_frint_32_64)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 5 => {
                            Some(Op::aarch64_vector_arithmetic_unary_not)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 5 => {
                            Some(Op::aarch64_vector_arithmetic_unary_rbit)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 5 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 12 => {
                            Some(Op::aarch64_vector_arithmetic_unary_cmp_float_bulk_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 13 => {
                            Some(Op::aarch64_vector_arithmetic_unary_cmp_float_bulk_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 14 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 15 => {
                            Some(Op::aarch64_vector_arithmetic_unary_diff_neg_float)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 24 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 25 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_round)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 26 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_conv_float_bulk_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 27 => {
                            Some(Op::aarch64_vector_arithmetic_unary_float_conv_float_bulk_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 28 => {
                            Some(Op::aarch64_vector_arithmetic_unary_special_sqrt_est_int)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 29 => {
                            Some(Op::aarch64_vector_arithmetic_unary_special_sqrt_est_float_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 31 => {
                            Some(Op::aarch64_vector_arithmetic_unary_special_sqrt)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 == 22 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 & 9 == 0 && x2 & 2 == 0 && x3 & 7 == 6 && x4 & 387 == 2 => {
                    let Q = (instr >> 30) & 1;
                    let U = (instr >> 29) & 1;
                    let size = (instr >> 22) & 3;
                    let opcode = (instr >> 12) & 9;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (U, size, opcode) {
                        (_, _, x2) if x2 & 30 == 0 => {
                            None
                        }
                        (_, _, x2) if x2 == 2 => {
                            None
                        }
                        (_, _, x2) if x2 & 28 == 4 => {
                            None
                        }
                        (_, _, x2) if x2 & 30 == 8 => {
                            None
                        }
                        (_, _, x2) if x2 == 11 => {
                            None
                        }
                        (_, _, x2) if x2 == 13 => {
                            None
                        }
                        (_, _, x2) if x2 == 14 => {
                            None
                        }
                        (_, _, x2) if x2 & 24 == 16 => {
                            None
                        }
                        (_, _, x2) if x2 & 30 == 24 => {
                            None
                        }
                        (_, _, x2) if x2 & 28 == 28 => {
                            None
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 3 => {
                            Some(Op::aarch64_vector_reduce_add_long)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 10 => {
                            Some(Op::aarch64_vector_reduce_int_max)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 26 => {
                            Some(Op::aarch64_vector_reduce_int_max)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 27 => {
                            Some(Op::aarch64_vector_reduce_add_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 12 => {
                            Some(Op::aarch64_vector_reduce_fp16_maxnm_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 15 => {
                            Some(Op::aarch64_vector_reduce_fp16_max_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 12 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 15 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 12 => {
                            Some(Op::aarch64_vector_reduce_fp16_maxnm_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 15 => {
                            Some(Op::aarch64_vector_reduce_fp16_max_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 12 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 15 => {
                            None
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 3 => {
                            Some(Op::aarch64_vector_reduce_add_long)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 10 => {
                            Some(Op::aarch64_vector_reduce_int_max)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 26 => {
                            Some(Op::aarch64_vector_reduce_int_max)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 27 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 12 => {
                            Some(Op::aarch64_vector_reduce_fp_maxnm_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 15 => {
                            Some(Op::aarch64_vector_reduce_fp_max_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 12 => {
                            Some(Op::aarch64_vector_reduce_fp_maxnm_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 15 => {
                            Some(Op::aarch64_vector_reduce_fp_max_simd)
                        }
                        _ => None
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 & 9 == 0 && x2 & 2 == 0 && x3 & 4 == 4 && x4 & 259 == 258 => {
                    None
                }
                (x0, _, x2, x3, x4, _) if x0 & 9 == 0 && x2 & 2 == 0 && x3 & 4 == 4 && x4 & 131 == 130 => {
                    None
                }
                (x0, _, x2, x3, x4, _) if x0 & 9 == 0 && x2 & 2 == 0 && x3 & 4 == 4 && x4 & 3 == 0 => {
                    let Q = (instr >> 30) & 1;
                    let U = (instr >> 29) & 1;
                    let size = (instr >> 22) & 3;
                    let Rm = (instr >> 16) & 9;
                    let opcode = (instr >> 12) & 7;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (U, opcode) {
                        (_, x1) if x1 == 15 => {
                            None
                        }
                        (x0, x1) if x0 == 0 && x1 == 0 => {
                            Some(Op::aarch64_vector_arithmetic_binary_disparate_add_sub_long)
                        }
                        (x0, x1) if x0 == 0 && x1 == 1 => {
                            Some(Op::aarch64_vector_arithmetic_binary_disparate_add_sub_wide)
                        }
                        (x0, x1) if x0 == 0 && x1 == 2 => {
                            Some(Op::aarch64_vector_arithmetic_binary_disparate_add_sub_long)
                        }
                        (x0, x1) if x0 == 0 && x1 == 3 => {
                            Some(Op::aarch64_vector_arithmetic_binary_disparate_add_sub_wide)
                        }
                        (x0, x1) if x0 == 0 && x1 == 4 => {
                            Some(Op::aarch64_vector_arithmetic_binary_disparate_add_sub_narrow)
                        }
                        (x0, x1) if x0 == 0 && x1 == 5 => {
                            Some(Op::aarch64_vector_arithmetic_binary_disparate_diff)
                        }
                        (x0, x1) if x0 == 0 && x1 == 6 => {
                            Some(Op::aarch64_vector_arithmetic_binary_disparate_add_sub_narrow)
                        }
                        (x0, x1) if x0 == 0 && x1 == 7 => {
                            Some(Op::aarch64_vector_arithmetic_binary_disparate_diff)
                        }
                        (x0, x1) if x0 == 0 && x1 == 8 => {
                            Some(Op::aarch64_vector_arithmetic_binary_disparate_mul_accum)
                        }
                        (x0, x1) if x0 == 0 && x1 == 9 => {
                            Some(Op::aarch64_vector_arithmetic_binary_disparate_mul_dmacc_simd)
                        }
                        (x0, x1) if x0 == 0 && x1 == 10 => {
                            Some(Op::aarch64_vector_arithmetic_binary_disparate_mul_accum)
                        }
                        (x0, x1) if x0 == 0 && x1 == 11 => {
                            Some(Op::aarch64_vector_arithmetic_binary_disparate_mul_dmacc_simd)
                        }
                        (x0, x1) if x0 == 0 && x1 == 12 => {
                            Some(Op::aarch64_vector_arithmetic_binary_disparate_mul_product)
                        }
                        (x0, x1) if x0 == 0 && x1 == 13 => {
                            Some(Op::aarch64_vector_arithmetic_binary_disparate_mul_double_simd)
                        }
                        (x0, x1) if x0 == 0 && x1 == 14 => {
                            Some(Op::aarch64_vector_arithmetic_binary_disparate_mul_poly)
                        }
                        (x0, x1) if x0 == 1 && x1 == 0 => {
                            Some(Op::aarch64_vector_arithmetic_binary_disparate_add_sub_long)
                        }
                        (x0, x1) if x0 == 1 && x1 == 1 => {
                            Some(Op::aarch64_vector_arithmetic_binary_disparate_add_sub_wide)
                        }
                        (x0, x1) if x0 == 1 && x1 == 2 => {
                            Some(Op::aarch64_vector_arithmetic_binary_disparate_add_sub_long)
                        }
                        (x0, x1) if x0 == 1 && x1 == 3 => {
                            Some(Op::aarch64_vector_arithmetic_binary_disparate_add_sub_wide)
                        }
                        (x0, x1) if x0 == 1 && x1 == 4 => {
                            Some(Op::aarch64_vector_arithmetic_binary_disparate_add_sub_narrow)
                        }
                        (x0, x1) if x0 == 1 && x1 == 5 => {
                            Some(Op::aarch64_vector_arithmetic_binary_disparate_diff)
                        }
                        (x0, x1) if x0 == 1 && x1 == 6 => {
                            Some(Op::aarch64_vector_arithmetic_binary_disparate_add_sub_narrow)
                        }
                        (x0, x1) if x0 == 1 && x1 == 7 => {
                            Some(Op::aarch64_vector_arithmetic_binary_disparate_diff)
                        }
                        (x0, x1) if x0 == 1 && x1 == 8 => {
                            Some(Op::aarch64_vector_arithmetic_binary_disparate_mul_accum)
                        }
                        (x0, x1) if x0 == 1 && x1 == 9 => {
                            None
                        }
                        (x0, x1) if x0 == 1 && x1 == 10 => {
                            Some(Op::aarch64_vector_arithmetic_binary_disparate_mul_accum)
                        }
                        (x0, x1) if x0 == 1 && x1 == 11 => {
                            None
                        }
                        (x0, x1) if x0 == 1 && x1 == 12 => {
                            Some(Op::aarch64_vector_arithmetic_binary_disparate_mul_product)
                        }
                        (x0, x1) if x0 == 1 && x1 == 13 => {
                            None
                        }
                        (x0, x1) if x0 == 1 && x1 == 14 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 & 9 == 0 && x2 & 2 == 0 && x3 & 4 == 4 && x4 & 1 == 1 => {
                    let Q = (instr >> 30) & 1;
                    let U = (instr >> 29) & 1;
                    let size = (instr >> 22) & 3;
                    let Rm = (instr >> 16) & 9;
                    let opcode = (instr >> 11) & 9;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (U, size, opcode) {
                        (x0, _, x2) if x0 == 0 && x2 == 0 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_add_halving_truncating)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 1 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_add_saturating_simd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 2 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_add_halving_rounding)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 4 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_sub_int)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 5 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 6 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_cmp_int_simd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 7 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_cmp_int_simd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 8 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_shift_simd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 9 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_shift_simd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 10 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_shift_simd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 11 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_shift_simd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 12 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_max_min_single)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 13 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_max_min_single)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 14 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_diff)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 15 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_diff)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 16 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 17 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 18 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_mul_int_accum)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 19 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_mul_int_product)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 20 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_max_min_pair)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 21 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_max_min_pair)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 22 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_simd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 23 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 24 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_max_min_fp_2008)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 25 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_mul_fp_fused)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 26 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_add_fp)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 27 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_mul_fp_extended_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 28 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 30 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_max_min_fp_1985)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 31 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_recps_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 3 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_logical_and_orr)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 29 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_lower)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 3 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_logical_and_orr)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 29 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 24 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_max_min_fp_2008)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 25 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_mul_fp_fused)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 26 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_sub_fp_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 27 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 28 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 30 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_max_min_fp_1985)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 31 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_rsqrts_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 3 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_logical_and_orr)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 29 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_lower)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 3 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_logical_and_orr)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 29 => {
                            None
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 0 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_add_halving_truncating)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 1 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_add_saturating_simd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 2 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_add_halving_rounding)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 4 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_sub_int)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 5 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 6 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_cmp_int_simd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 7 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_cmp_int_simd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 8 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_shift_simd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 9 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_shift_simd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 10 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_shift_simd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 11 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_shift_simd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 12 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_max_min_single)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 13 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_max_min_single)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 14 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_diff)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 15 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_diff)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 16 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 17 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 18 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_mul_int_accum)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 19 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_mul_int_product)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 20 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_max_min_pair)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 21 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_max_min_pair)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 22 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_simd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 23 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 24 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_max_min_fp_2008)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 26 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_add_fp)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 27 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_mul_fp_product)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 28 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 29 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 30 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_max_min_fp_1985)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 31 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_div)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 3 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_logical_bsl_eor)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 25 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_upper)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 3 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_logical_bsl_eor)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 25 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 24 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_max_min_fp_2008)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 26 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_sub_fp_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 27 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 28 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 29 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 30 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_max_min_fp_1985)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 31 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 == 3 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_logical_bsl_eor)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 == 25 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_upper)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 3 => {
                            Some(Op::aarch64_vector_arithmetic_binary_uniform_logical_bsl_eor)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 25 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 & 9 == 0 && x2 == 2 && x3 == 0 && x4 & 1 == 1 => {
                    let Q = (instr >> 30) & 1;
                    let op = (instr >> 29) & 1;
                    let a = (instr >> 18) & 1;
                    let b = (instr >> 17) & 1;
                    let c = (instr >> 16) & 1;
                    let cmode = (instr >> 12) & 7;
                    let o2 = (instr >> 11) & 1;
                    let d = (instr >> 9) & 1;
                    let e = (instr >> 8) & 1;
                    let f = (instr >> 7) & 1;
                    let g = (instr >> 6) & 1;
                    let h = (instr >> 5) & 1;
                    let Rd = instr & 9;
                    match (Q, op, cmode, o2) {
                        (_, x1, x2, x3) if x1 == 0 && x2 & 8 == 0 && x3 == 1 => {
                            None
                        }
                        (_, x1, x2, x3) if x1 == 0 && x2 & 9 == 0 && x3 == 0 => {
                            Some(Op::aarch64_vector_logical)
                        }
                        (_, x1, x2, x3) if x1 == 0 && x2 & 9 == 1 && x3 == 0 => {
                            Some(Op::aarch64_vector_logical)
                        }
                        (_, x1, x2, x3) if x1 == 0 && x2 & 12 == 8 && x3 == 1 => {
                            None
                        }
                        (_, x1, x2, x3) if x1 == 0 && x2 & 13 == 8 && x3 == 0 => {
                            Some(Op::aarch64_vector_logical)
                        }
                        (_, x1, x2, x3) if x1 == 0 && x2 & 13 == 9 && x3 == 0 => {
                            Some(Op::aarch64_vector_logical)
                        }
                        (_, x1, x2, x3) if x1 == 0 && x2 & 14 == 12 && x3 == 0 => {
                            Some(Op::aarch64_vector_logical)
                        }
                        (_, x1, x2, x3) if x1 == 0 && x2 & 14 == 12 && x3 == 1 => {
                            None
                        }
                        (_, x1, x2, x3) if x1 == 0 && x2 == 14 && x3 == 0 => {
                            Some(Op::aarch64_vector_logical)
                        }
                        (_, x1, x2, x3) if x1 == 0 && x2 == 14 && x3 == 1 => {
                            None
                        }
                        (_, x1, x2, x3) if x1 == 0 && x2 == 15 && x3 == 0 => {
                            Some(Op::aarch64_vector_logical)
                        }
                        (_, x1, x2, x3) if x1 == 0 && x2 == 15 && x3 == 1 => {
                            Some(Op::aarch64_vector_fp16_movi)
                        }
                        (_, x1, _, x3) if x1 == 1 && x3 == 1 => {
                            None
                        }
                        (_, x1, x2, x3) if x1 == 1 && x2 & 9 == 0 && x3 == 0 => {
                            Some(Op::aarch64_vector_logical)
                        }
                        (_, x1, x2, x3) if x1 == 1 && x2 & 9 == 1 && x3 == 0 => {
                            Some(Op::aarch64_vector_logical)
                        }
                        (_, x1, x2, x3) if x1 == 1 && x2 & 13 == 8 && x3 == 0 => {
                            Some(Op::aarch64_vector_logical)
                        }
                        (_, x1, x2, x3) if x1 == 1 && x2 & 13 == 9 && x3 == 0 => {
                            Some(Op::aarch64_vector_logical)
                        }
                        (_, x1, x2, x3) if x1 == 1 && x2 & 14 == 12 && x3 == 0 => {
                            Some(Op::aarch64_vector_logical)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 14 && x3 == 0 => {
                            Some(Op::aarch64_vector_logical)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 15 && x3 == 0 => {
                            None
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 14 && x3 == 0 => {
                            Some(Op::aarch64_vector_logical)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 15 && x3 == 0 => {
                            Some(Op::aarch64_vector_logical)
                        }
                        _ => None
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 & 9 == 0 && x2 == 2 && x3 != 0 && x4 & 1 == 1 => {
                    let Q = (instr >> 30) & 1;
                    let U = (instr >> 29) & 1;
                    let immh = (instr >> 19) & 7;
                    let immb = (instr >> 16) & 5;
                    let opcode = (instr >> 11) & 9;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (U, opcode) {
                        (_, x1) if x1 == 1 => {
                            None
                        }
                        (_, x1) if x1 == 3 => {
                            None
                        }
                        (_, x1) if x1 == 5 => {
                            None
                        }
                        (_, x1) if x1 == 7 => {
                            None
                        }
                        (_, x1) if x1 == 9 => {
                            None
                        }
                        (_, x1) if x1 == 11 => {
                            None
                        }
                        (_, x1) if x1 == 13 => {
                            None
                        }
                        (_, x1) if x1 == 15 => {
                            None
                        }
                        (_, x1) if x1 == 21 => {
                            None
                        }
                        (_, x1) if x1 & 30 == 22 => {
                            None
                        }
                        (_, x1) if x1 & 28 == 24 => {
                            None
                        }
                        (_, x1) if x1 == 29 => {
                            None
                        }
                        (_, x1) if x1 == 30 => {
                            None
                        }
                        (x0, x1) if x0 == 0 && x1 == 0 => {
                            Some(Op::aarch64_vector_shift_right_simd)
                        }
                        (x0, x1) if x0 == 0 && x1 == 2 => {
                            Some(Op::aarch64_vector_shift_right_simd)
                        }
                        (x0, x1) if x0 == 0 && x1 == 4 => {
                            Some(Op::aarch64_vector_shift_right_simd)
                        }
                        (x0, x1) if x0 == 0 && x1 == 6 => {
                            Some(Op::aarch64_vector_shift_right_simd)
                        }
                        (x0, x1) if x0 == 0 && x1 == 8 => {
                            None
                        }
                        (x0, x1) if x0 == 0 && x1 == 10 => {
                            Some(Op::aarch64_vector_shift_left_simd)
                        }
                        (x0, x1) if x0 == 0 && x1 == 12 => {
                            None
                        }
                        (x0, x1) if x0 == 0 && x1 == 14 => {
                            Some(Op::aarch64_vector_shift_left_sat_simd)
                        }
                        (x0, x1) if x0 == 0 && x1 == 16 => {
                            Some(Op::aarch64_vector_shift_right_narrow_logical)
                        }
                        (x0, x1) if x0 == 0 && x1 == 17 => {
                            Some(Op::aarch64_vector_shift_right_narrow_logical)
                        }
                        (x0, x1) if x0 == 0 && x1 == 18 => {
                            Some(Op::aarch64_vector_shift_right_narrow_uniform_simd)
                        }
                        (x0, x1) if x0 == 0 && x1 == 19 => {
                            Some(Op::aarch64_vector_shift_right_narrow_uniform_simd)
                        }
                        (x0, x1) if x0 == 0 && x1 == 20 => {
                            Some(Op::aarch64_vector_shift_left_long)
                        }
                        (x0, x1) if x0 == 0 && x1 == 28 => {
                            Some(Op::aarch64_vector_shift_conv_int_simd)
                        }
                        (x0, x1) if x0 == 0 && x1 == 31 => {
                            Some(Op::aarch64_vector_shift_conv_float_simd)
                        }
                        (x0, x1) if x0 == 1 && x1 == 0 => {
                            Some(Op::aarch64_vector_shift_right_simd)
                        }
                        (x0, x1) if x0 == 1 && x1 == 2 => {
                            Some(Op::aarch64_vector_shift_right_simd)
                        }
                        (x0, x1) if x0 == 1 && x1 == 4 => {
                            Some(Op::aarch64_vector_shift_right_simd)
                        }
                        (x0, x1) if x0 == 1 && x1 == 6 => {
                            Some(Op::aarch64_vector_shift_right_simd)
                        }
                        (x0, x1) if x0 == 1 && x1 == 8 => {
                            Some(Op::aarch64_vector_shift_right_insert_simd)
                        }
                        (x0, x1) if x0 == 1 && x1 == 10 => {
                            Some(Op::aarch64_vector_shift_left_insert_simd)
                        }
                        (x0, x1) if x0 == 1 && x1 == 12 => {
                            Some(Op::aarch64_vector_shift_left_sat_simd)
                        }
                        (x0, x1) if x0 == 1 && x1 == 14 => {
                            Some(Op::aarch64_vector_shift_left_sat_simd)
                        }
                        (x0, x1) if x0 == 1 && x1 == 16 => {
                            Some(Op::aarch64_vector_shift_right_narrow_nonuniform_simd)
                        }
                        (x0, x1) if x0 == 1 && x1 == 17 => {
                            Some(Op::aarch64_vector_shift_right_narrow_nonuniform_simd)
                        }
                        (x0, x1) if x0 == 1 && x1 == 18 => {
                            Some(Op::aarch64_vector_shift_right_narrow_uniform_simd)
                        }
                        (x0, x1) if x0 == 1 && x1 == 19 => {
                            Some(Op::aarch64_vector_shift_right_narrow_uniform_simd)
                        }
                        (x0, x1) if x0 == 1 && x1 == 20 => {
                            Some(Op::aarch64_vector_shift_left_long)
                        }
                        (x0, x1) if x0 == 1 && x1 == 28 => {
                            Some(Op::aarch64_vector_shift_conv_int_simd)
                        }
                        (x0, x1) if x0 == 1 && x1 == 31 => {
                            Some(Op::aarch64_vector_shift_conv_float_simd)
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _) if x0 & 9 == 0 && x2 == 3 && x4 & 1 == 1 => {
                    None
                }
                (x0, _, x2, _, x4, _) if x0 & 9 == 0 && x2 & 2 == 2 && x4 & 1 == 0 => {
                    let Q = (instr >> 30) & 1;
                    let U = (instr >> 29) & 1;
                    let size = (instr >> 22) & 3;
                    let L = (instr >> 21) & 1;
                    let M = (instr >> 20) & 1;
                    let Rm = (instr >> 16) & 7;
                    let opcode = (instr >> 12) & 7;
                    let H = (instr >> 11) & 1;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (U, size, opcode) {
                        (_, x1, x2) if x1 == 1 && x2 == 9 => {
                            None
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 2 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_acc_long)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 3 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_acc_double_simd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 6 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_acc_long)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 7 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_acc_double_simd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 8 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_int)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 10 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_long)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 11 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_double_simd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 12 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_high_simd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 13 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_high_simd)
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 14 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_dotp)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 0 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 4 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_acc_fp16_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 5 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_acc_fp16_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 9 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_fp16_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 15 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mat_mul_int_dotp)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 5 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 15 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_bfdot)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 1 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_acc_fp_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 5 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_acc_fp_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 9 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_fp_simd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 0 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_acc_mul_norounding_i_lower)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 4 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_acc_mul_norounding_i_lower)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 15 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mat_mul_int_dotp)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 0 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 4 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 15 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_acc_bf16_long)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 0 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_acc_int)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 2 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_acc_long)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 4 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_acc_int)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 6 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_acc_long)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 10 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_long)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 11 => {
                            None
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 13 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_acc_high_simd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 14 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_dotp)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 15 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_acc_high_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 8 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 12 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 3 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 5 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 7 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 9 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_fp16_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 & 9 == 1 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_acc_complex)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 9 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_fp_simd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 & 9 == 1 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_acc_complex)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 == 8 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_acc_mul_norounding_i_upper)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 == 12 => {
                            Some(Op::aarch64_vector_arithmetic_binary_element_mul_acc_mul_norounding_i_upper)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 1 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 3 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 5 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 7 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 8 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 12 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 == 12 && x2 == 0 && x3 & 12 == 8 && x4 & 48 == 32 => {
                    let Rm = (instr >> 16) & 9;
                    let imm2 = (instr >> 12) & 3;
                    let opcode = (instr >> 10) & 3;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match opcode {
                        x0 if x0 == 0 => {
                            Some(Op::aarch64_vector_crypto_sm3_sm3tt1a)
                        }
                        x0 if x0 == 1 => {
                            Some(Op::aarch64_vector_crypto_sm3_sm3tt1b)
                        }
                        x0 if x0 == 2 => {
                            Some(Op::aarch64_vector_crypto_sm3_sm3tt2a)
                        }
                        x0 if x0 == 3 => {
                            Some(Op::aarch64_vector_crypto_sm3_sm3tt2b)
                        }
                        _ => None
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 == 12 && x2 == 0 && x3 & 12 == 12 && x4 & 44 == 32 => {
                    let Rm = (instr >> 16) & 9;
                    let O = (instr >> 14) & 1;
                    let opcode = (instr >> 10) & 3;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (O, opcode) {
                        (x0, x1) if x0 == 0 && x1 == 0 => {
                            Some(Op::aarch64_vector_crypto_sha512_sha512h)
                        }
                        (x0, x1) if x0 == 0 && x1 == 1 => {
                            Some(Op::aarch64_vector_crypto_sha512_sha512h2)
                        }
                        (x0, x1) if x0 == 0 && x1 == 2 => {
                            Some(Op::aarch64_vector_crypto_sha512_sha512su1)
                        }
                        (x0, x1) if x0 == 0 && x1 == 3 => {
                            Some(Op::aarch64_vector_crypto_sha3_rax1)
                        }
                        (x0, x1) if x0 == 1 && x1 == 0 => {
                            Some(Op::aarch64_vector_crypto_sm3_sm3partw1)
                        }
                        (x0, x1) if x0 == 1 && x1 == 1 => {
                            Some(Op::aarch64_vector_crypto_sm3_sm3partw2)
                        }
                        (x0, x1) if x0 == 1 && x1 == 2 => {
                            Some(Op::aarch64_vector_crypto_sm4_sm4enckey)
                        }
                        (x0, x1) if x0 == 1 && x1 == 3 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _) if x0 == 12 && x2 == 0 && x4 & 32 == 0 => {
                    let Op0 = (instr >> 21) & 3;
                    let Rm = (instr >> 16) & 9;
                    let Ra = (instr >> 10) & 9;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match Op0 {
                        x0 if x0 == 0 => {
                            Some(Op::aarch64_vector_crypto_sha3_eor3)
                        }
                        x0 if x0 == 1 => {
                            Some(Op::aarch64_vector_crypto_sha3_bcax)
                        }
                        x0 if x0 == 2 => {
                            Some(Op::aarch64_vector_crypto_sm3_sm3ss1)
                        }
                        x0 if x0 == 3 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, x3, _, _) if x0 == 12 && x2 == 1 && x3 & 12 == 0 => {
                    let Rm = (instr >> 16) & 9;
                    let imm6 = (instr >> 10) & 11;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match () {
                        () => {
                            Some(Op::aarch64_vector_crypto_sha3_xar)
                        }
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 == 12 && x2 == 1 && x3 == 8 && x4 & 508 == 32 => {
                    let opcode = (instr >> 10) & 3;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match opcode {
                        x0 if x0 == 0 => {
                            Some(Op::aarch64_vector_crypto_sha512_sha512su0)
                        }
                        x0 if x0 == 1 => {
                            Some(Op::aarch64_vector_crypto_sm4_sm4enc)
                        }
                        x0 if x0 & 2 == 2 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, _, _) if x0 & 9 == 8 && x2 & 2 == 2 => {
                    None
                }
                (x0, _, x2, x3, _, _) if x0 & 5 == 1 && x2 & 2 == 0 && x3 & 4 == 0 => {
                    let sf = (instr >> 31) & 1;
                    let S = (instr >> 29) & 1;
                    let ptype = (instr >> 22) & 3;
                    let rmode = (instr >> 19) & 3;
                    let opcode = (instr >> 16) & 5;
                    let scale = (instr >> 10) & 11;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (sf, S, ptype, rmode, opcode, scale) {
                        (_, _, _, _, x4, _) if x4 & 4 == 4 => {
                            None
                        }
                        (_, _, _, x3, x4, _) if x3 & 1 == 0 && x4 & 6 == 0 => {
                            None
                        }
                        (_, _, _, x3, x4, _) if x3 & 1 == 1 && x4 & 6 == 2 => {
                            None
                        }
                        (_, _, _, x3, x4, _) if x3 & 2 == 0 && x4 & 6 == 0 => {
                            None
                        }
                        (_, _, _, x3, x4, _) if x3 & 2 == 2 && x4 & 6 == 2 => {
                            None
                        }
                        (_, _, x2, _, _, _) if x2 == 2 => {
                            None
                        }
                        (_, x1, _, _, _, _) if x1 == 1 => {
                            None
                        }
                        (x0, _, _, _, _, x5) if x0 == 0 && x5 & 32 == 0 => {
                            None
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 2 => {
                            Some(Op::aarch64_float_convert_fix)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 3 => {
                            Some(Op::aarch64_float_convert_fix)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 3 && x4 == 0 => {
                            Some(Op::aarch64_float_convert_fix)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 3 && x4 == 1 => {
                            Some(Op::aarch64_float_convert_fix)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 2 => {
                            Some(Op::aarch64_float_convert_fix)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 3 => {
                            Some(Op::aarch64_float_convert_fix)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 3 && x4 == 0 => {
                            Some(Op::aarch64_float_convert_fix)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 3 && x4 == 1 => {
                            Some(Op::aarch64_float_convert_fix)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 2 => {
                            Some(Op::aarch64_float_convert_fix)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 3 => {
                            Some(Op::aarch64_float_convert_fix)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 3 && x4 == 0 => {
                            Some(Op::aarch64_float_convert_fix)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 3 && x4 == 1 => {
                            Some(Op::aarch64_float_convert_fix)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 2 => {
                            Some(Op::aarch64_float_convert_fix)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 3 => {
                            Some(Op::aarch64_float_convert_fix)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 3 && x4 == 0 => {
                            Some(Op::aarch64_float_convert_fix)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 3 && x4 == 1 => {
                            Some(Op::aarch64_float_convert_fix)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 2 => {
                            Some(Op::aarch64_float_convert_fix)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 3 => {
                            Some(Op::aarch64_float_convert_fix)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 3 && x4 == 0 => {
                            Some(Op::aarch64_float_convert_fix)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 3 && x4 == 1 => {
                            Some(Op::aarch64_float_convert_fix)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 2 => {
                            Some(Op::aarch64_float_convert_fix)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 3 => {
                            Some(Op::aarch64_float_convert_fix)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 3 && x4 == 0 => {
                            Some(Op::aarch64_float_convert_fix)
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 3 && x4 == 1 => {
                            Some(Op::aarch64_float_convert_fix)
                        }
                        _ => None
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 & 5 == 1 && x2 & 2 == 0 && x3 & 4 == 4 && x4 & 63 == 0 => {
                    let sf = (instr >> 31) & 1;
                    let S = (instr >> 29) & 1;
                    let ptype = (instr >> 22) & 3;
                    let rmode = (instr >> 19) & 3;
                    let opcode = (instr >> 16) & 5;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (sf, S, ptype, rmode, opcode) {
                        (_, _, _, x3, x4) if x3 & 1 == 1 && x4 & 6 == 2 => {
                            None
                        }
                        (_, _, _, x3, x4) if x3 & 1 == 1 && x4 & 6 == 4 => {
                            None
                        }
                        (_, _, _, x3, x4) if x3 & 2 == 2 && x4 & 6 == 2 => {
                            None
                        }
                        (_, _, _, x3, x4) if x3 & 2 == 2 && x4 & 6 == 4 => {
                            None
                        }
                        (_, x1, x2, _, x4) if x1 == 0 && x2 == 2 && x4 & 4 == 0 => {
                            None
                        }
                        (_, x1, x2, _, x4) if x1 == 0 && x2 == 2 && x4 & 6 == 4 => {
                            None
                        }
                        (_, x1, _, _, _) if x1 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 & 1 == 1 && x4 & 6 == 6 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 1 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 2 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 3 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 4 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 5 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 6 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 7 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 1 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 & 2 == 2 && x4 & 6 == 6 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 2 && x4 == 0 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 2 && x4 == 1 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 3 && x4 == 0 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 3 && x4 == 1 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 & 2 == 0 && x4 & 6 == 6 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 2 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 3 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 4 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 5 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 2 && x4 == 0 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 2 && x4 == 1 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 2 && x4 & 6 == 6 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 3 && x4 == 0 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 3 && x4 == 1 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 3 && x4 == 6 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 3 && x4 == 7 => {
                            None
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 0 && x2 == 2 && x4 & 6 == 6 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 0 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 1 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 2 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 3 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 4 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 5 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 6 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 7 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 1 && x4 == 0 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 1 && x4 == 1 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 2 && x4 == 0 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 2 && x4 == 1 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 3 && x4 == 0 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 3 && x4 == 1 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 0 && x2 == 0 && x4 & 6 == 6 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 1 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 2 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 3 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 4 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 5 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 1 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 2 && x4 == 0 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 2 && x4 == 1 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 3 && x4 == 0 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 3 && x4 == 1 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 & 1 == 1 && x4 & 6 == 6 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 2 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 3 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 4 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 5 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 6 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 7 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 & 2 == 2 && x4 & 6 == 6 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 2 && x4 == 0 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 2 && x4 == 1 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 3 && x4 == 0 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 3 && x4 == 1 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 2 && x3 & 1 == 0 && x4 & 6 == 6 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 2 && x3 == 1 && x4 == 6 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 2 && x3 == 1 && x4 == 7 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 2 && x3 & 2 == 2 && x4 & 6 == 6 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 0 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 1 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 2 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 3 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 4 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 5 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 6 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 7 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 1 && x4 == 0 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 1 && x4 == 1 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 2 && x4 == 0 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 2 && x4 == 1 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 3 && x4 == 0 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 3 && x4 == 1 => {
                            Some(Op::aarch64_float_convert_int)
                        }
                        _ => None
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 & 5 == 1 && x2 & 2 == 0 && x3 & 4 == 4 && x4 & 31 == 16 => {
                    let M = (instr >> 31) & 1;
                    let S = (instr >> 29) & 1;
                    let ptype = (instr >> 22) & 3;
                    let opcode = (instr >> 15) & 11;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (M, S, ptype, opcode) {
                        (_, _, _, x3) if x3 & 32 == 32 => {
                            None
                        }
                        (_, x1, _, _) if x1 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            Some(Op::aarch64_float_arithmetic_unary)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            Some(Op::aarch64_float_arithmetic_unary)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 2 => {
                            Some(Op::aarch64_float_arithmetic_unary)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 3 => {
                            Some(Op::aarch64_float_arithmetic_unary)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 4 => {
                            None
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 5 => {
                            Some(Op::aarch64_float_convert_fp)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 6 => {
                            None
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 7 => {
                            Some(Op::aarch64_float_convert_fp)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 8 => {
                            Some(Op::aarch64_float_arithmetic_round_frint)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 9 => {
                            Some(Op::aarch64_float_arithmetic_round_frint)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 10 => {
                            Some(Op::aarch64_float_arithmetic_round_frint)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 11 => {
                            Some(Op::aarch64_float_arithmetic_round_frint)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 12 => {
                            Some(Op::aarch64_float_arithmetic_round_frint)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 13 => {
                            None
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 14 => {
                            Some(Op::aarch64_float_arithmetic_round_frint)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 15 => {
                            Some(Op::aarch64_float_arithmetic_round_frint)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 16 => {
                            Some(Op::aarch64_float_arithmetic_round_frint_32_64)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 17 => {
                            Some(Op::aarch64_float_arithmetic_round_frint_32_64)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 18 => {
                            Some(Op::aarch64_float_arithmetic_round_frint_32_64)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 19 => {
                            Some(Op::aarch64_float_arithmetic_round_frint_32_64)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 & 60 == 20 => {
                            None
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 & 56 == 24 => {
                            None
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            Some(Op::aarch64_float_arithmetic_unary)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 => {
                            Some(Op::aarch64_float_arithmetic_unary)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 2 => {
                            Some(Op::aarch64_float_arithmetic_unary)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 3 => {
                            Some(Op::aarch64_float_arithmetic_unary)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 4 => {
                            Some(Op::aarch64_float_convert_fp)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 5 => {
                            None
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 6 => {
                            Some(Op::aarch64_vector_cvt_bf16_scalar)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 7 => {
                            Some(Op::aarch64_float_convert_fp)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 8 => {
                            Some(Op::aarch64_float_arithmetic_round_frint)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 9 => {
                            Some(Op::aarch64_float_arithmetic_round_frint)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 10 => {
                            Some(Op::aarch64_float_arithmetic_round_frint)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 11 => {
                            Some(Op::aarch64_float_arithmetic_round_frint)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 12 => {
                            Some(Op::aarch64_float_arithmetic_round_frint)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 13 => {
                            None
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 14 => {
                            Some(Op::aarch64_float_arithmetic_round_frint)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 15 => {
                            Some(Op::aarch64_float_arithmetic_round_frint)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 16 => {
                            Some(Op::aarch64_float_arithmetic_round_frint_32_64)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 17 => {
                            Some(Op::aarch64_float_arithmetic_round_frint_32_64)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 18 => {
                            Some(Op::aarch64_float_arithmetic_round_frint_32_64)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 19 => {
                            Some(Op::aarch64_float_arithmetic_round_frint_32_64)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 & 60 == 20 => {
                            None
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 & 56 == 24 => {
                            None
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 2 && x3 & 32 == 0 => {
                            None
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 => {
                            Some(Op::aarch64_float_arithmetic_unary)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 1 => {
                            Some(Op::aarch64_float_arithmetic_unary)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 2 => {
                            Some(Op::aarch64_float_arithmetic_unary)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 3 => {
                            Some(Op::aarch64_float_arithmetic_unary)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 4 => {
                            Some(Op::aarch64_float_convert_fp)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 5 => {
                            Some(Op::aarch64_float_convert_fp)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 & 62 == 6 => {
                            None
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 8 => {
                            Some(Op::aarch64_float_arithmetic_round_frint)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 9 => {
                            Some(Op::aarch64_float_arithmetic_round_frint)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 10 => {
                            Some(Op::aarch64_float_arithmetic_round_frint)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 11 => {
                            Some(Op::aarch64_float_arithmetic_round_frint)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 12 => {
                            Some(Op::aarch64_float_arithmetic_round_frint)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 13 => {
                            None
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 14 => {
                            Some(Op::aarch64_float_arithmetic_round_frint)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 15 => {
                            Some(Op::aarch64_float_arithmetic_round_frint)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 & 48 == 16 => {
                            None
                        }
                        (x0, _, _, _) if x0 == 1 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 & 5 == 1 && x2 & 2 == 0 && x3 & 4 == 4 && x4 & 15 == 8 => {
                    let M = (instr >> 31) & 1;
                    let S = (instr >> 29) & 1;
                    let ptype = (instr >> 22) & 3;
                    let Rm = (instr >> 16) & 9;
                    let op = (instr >> 14) & 3;
                    let Rn = (instr >> 5) & 9;
                    let opcode2 = instr & 9;
                    match (M, S, ptype, op, opcode2) {
                        (_, _, _, _, x4) if x4 & 1 == 1 => {
                            None
                        }
                        (_, _, _, _, x4) if x4 & 2 == 2 => {
                            None
                        }
                        (_, _, _, _, x4) if x4 & 4 == 4 => {
                            None
                        }
                        (_, _, _, x3, _) if x3 & 1 == 1 => {
                            None
                        }
                        (_, _, _, x3, _) if x3 & 2 == 2 => {
                            None
                        }
                        (_, _, x2, _, _) if x2 == 2 => {
                            None
                        }
                        (_, x1, _, _, _) if x1 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 => {
                            Some(Op::aarch64_float_compare_uncond)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 8 => {
                            Some(Op::aarch64_float_compare_uncond)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 16 => {
                            Some(Op::aarch64_float_compare_uncond)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 24 => {
                            Some(Op::aarch64_float_compare_uncond)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            Some(Op::aarch64_float_compare_uncond)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 8 => {
                            Some(Op::aarch64_float_compare_uncond)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 16 => {
                            Some(Op::aarch64_float_compare_uncond)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 24 => {
                            Some(Op::aarch64_float_compare_uncond)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 0 => {
                            Some(Op::aarch64_float_compare_uncond)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 8 => {
                            Some(Op::aarch64_float_compare_uncond)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 16 => {
                            Some(Op::aarch64_float_compare_uncond)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 24 => {
                            Some(Op::aarch64_float_compare_uncond)
                        }
                        (x0, _, _, _, _) if x0 == 1 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 & 5 == 1 && x2 & 2 == 0 && x3 & 4 == 4 && x4 & 7 == 4 => {
                    let M = (instr >> 31) & 1;
                    let S = (instr >> 29) & 1;
                    let ptype = (instr >> 22) & 3;
                    let imm8 = (instr >> 13) & 15;
                    let imm5 = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (M, S, ptype, imm5) {
                        (_, _, _, x3) if x3 & 1 == 1 => {
                            None
                        }
                        (_, _, _, x3) if x3 & 2 == 2 => {
                            None
                        }
                        (_, _, _, x3) if x3 & 4 == 4 => {
                            None
                        }
                        (_, _, _, x3) if x3 & 8 == 8 => {
                            None
                        }
                        (_, _, _, x3) if x3 & 16 == 16 => {
                            None
                        }
                        (_, _, x2, _) if x2 == 2 => {
                            None
                        }
                        (_, x1, _, _) if x1 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            Some(Op::aarch64_float_move_fp_imm)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            Some(Op::aarch64_float_move_fp_imm)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 => {
                            Some(Op::aarch64_float_move_fp_imm)
                        }
                        (x0, _, _, _) if x0 == 1 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 & 5 == 1 && x2 & 2 == 0 && x3 & 4 == 4 && x4 & 3 == 1 => {
                    let M = (instr >> 31) & 1;
                    let S = (instr >> 29) & 1;
                    let ptype = (instr >> 22) & 3;
                    let Rm = (instr >> 16) & 9;
                    let cond = (instr >> 12) & 7;
                    let Rn = (instr >> 5) & 9;
                    let op = (instr >> 4) & 1;
                    let nzcv = instr & 7;
                    match (M, S, ptype, op) {
                        (_, _, x2, _) if x2 == 2 => {
                            None
                        }
                        (_, x1, _, _) if x1 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            Some(Op::aarch64_float_compare_cond)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            Some(Op::aarch64_float_compare_cond)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            Some(Op::aarch64_float_compare_cond)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 => {
                            Some(Op::aarch64_float_compare_cond)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 => {
                            Some(Op::aarch64_float_compare_cond)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 1 => {
                            Some(Op::aarch64_float_compare_cond)
                        }
                        (x0, _, _, _) if x0 == 1 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 & 5 == 1 && x2 & 2 == 0 && x3 & 4 == 4 && x4 & 3 == 2 => {
                    let M = (instr >> 31) & 1;
                    let S = (instr >> 29) & 1;
                    let ptype = (instr >> 22) & 3;
                    let Rm = (instr >> 16) & 9;
                    let opcode = (instr >> 12) & 7;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (M, S, ptype, opcode) {
                        (_, _, _, x3) if x3 & 9 == 9 => {
                            None
                        }
                        (_, _, _, x3) if x3 & 10 == 10 => {
                            None
                        }
                        (_, _, _, x3) if x3 & 12 == 12 => {
                            None
                        }
                        (_, _, x2, _) if x2 == 2 => {
                            None
                        }
                        (_, x1, _, _) if x1 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            Some(Op::aarch64_float_arithmetic_mul_product)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            Some(Op::aarch64_float_arithmetic_div)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 2 => {
                            Some(Op::aarch64_float_arithmetic_add_sub)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 3 => {
                            Some(Op::aarch64_float_arithmetic_add_sub)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 4 => {
                            Some(Op::aarch64_float_arithmetic_max_min)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 5 => {
                            Some(Op::aarch64_float_arithmetic_max_min)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 6 => {
                            Some(Op::aarch64_float_arithmetic_max_min)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 7 => {
                            Some(Op::aarch64_float_arithmetic_max_min)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 8 => {
                            Some(Op::aarch64_float_arithmetic_mul_product)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            Some(Op::aarch64_float_arithmetic_mul_product)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 => {
                            Some(Op::aarch64_float_arithmetic_div)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 2 => {
                            Some(Op::aarch64_float_arithmetic_add_sub)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 3 => {
                            Some(Op::aarch64_float_arithmetic_add_sub)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 4 => {
                            Some(Op::aarch64_float_arithmetic_max_min)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 5 => {
                            Some(Op::aarch64_float_arithmetic_max_min)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 6 => {
                            Some(Op::aarch64_float_arithmetic_max_min)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 7 => {
                            Some(Op::aarch64_float_arithmetic_max_min)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 8 => {
                            Some(Op::aarch64_float_arithmetic_mul_product)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 => {
                            Some(Op::aarch64_float_arithmetic_mul_product)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 1 => {
                            Some(Op::aarch64_float_arithmetic_div)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 2 => {
                            Some(Op::aarch64_float_arithmetic_add_sub)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 3 => {
                            Some(Op::aarch64_float_arithmetic_add_sub)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 4 => {
                            Some(Op::aarch64_float_arithmetic_max_min)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 5 => {
                            Some(Op::aarch64_float_arithmetic_max_min)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 6 => {
                            Some(Op::aarch64_float_arithmetic_max_min)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 7 => {
                            Some(Op::aarch64_float_arithmetic_max_min)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 8 => {
                            Some(Op::aarch64_float_arithmetic_mul_product)
                        }
                        (x0, _, _, _) if x0 == 1 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 & 5 == 1 && x2 & 2 == 0 && x3 & 4 == 4 && x4 & 3 == 3 => {
                    let M = (instr >> 31) & 1;
                    let S = (instr >> 29) & 1;
                    let ptype = (instr >> 22) & 3;
                    let Rm = (instr >> 16) & 9;
                    let cond = (instr >> 12) & 7;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (M, S, ptype) {
                        (_, _, x2) if x2 == 2 => {
                            None
                        }
                        (_, x1, _) if x1 == 1 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch64_float_move_fp_select)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch64_float_move_fp_select)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 3 => {
                            Some(Op::aarch64_float_move_fp_select)
                        }
                        (x0, _, _) if x0 == 1 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, _, _) if x0 & 5 == 1 && x2 & 2 == 2 => {
                    let M = (instr >> 31) & 1;
                    let S = (instr >> 29) & 1;
                    let ptype = (instr >> 22) & 3;
                    let o1 = (instr >> 21) & 1;
                    let Rm = (instr >> 16) & 9;
                    let o0 = (instr >> 15) & 1;
                    let Ra = (instr >> 10) & 9;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (M, S, ptype, o1, o0) {
                        (_, _, x2, _, _) if x2 == 2 => {
                            None
                        }
                        (_, x1, _, _, _) if x1 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 => {
                            Some(Op::aarch64_float_arithmetic_mul_add_sub)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 1 => {
                            Some(Op::aarch64_float_arithmetic_mul_add_sub)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 => {
                            Some(Op::aarch64_float_arithmetic_mul_add_sub)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 1 => {
                            Some(Op::aarch64_float_arithmetic_mul_add_sub)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            Some(Op::aarch64_float_arithmetic_mul_add_sub)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 => {
                            Some(Op::aarch64_float_arithmetic_mul_add_sub)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 => {
                            Some(Op::aarch64_float_arithmetic_mul_add_sub)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 => {
                            Some(Op::aarch64_float_arithmetic_mul_add_sub)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 0 => {
                            Some(Op::aarch64_float_arithmetic_mul_add_sub)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 1 => {
                            Some(Op::aarch64_float_arithmetic_mul_add_sub)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 1 && x4 == 0 => {
                            Some(Op::aarch64_float_arithmetic_mul_add_sub)
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 1 && x4 == 1 => {
                            Some(Op::aarch64_float_arithmetic_mul_add_sub)
                        }
                        (x0, _, _, _, _) if x0 == 1 => {
                            None
                        }
                        _ => None
                    }
                }
                _ => None
            }
        }
        _ => None
    }
} // end of decoding A64
#[allow(unused_variables)]
#[allow(non_snake_case)]
pub fn decode_a32(instr: u32) -> Option<Op> {
    match ((instr >> 28) & 7, (instr >> 25) & 5, (instr >> 5) & 39, (instr >> 4) & 1, instr & 7) {
        (x0, x1, _, _, _) if x0 != 15 && x1 & 6 == 0 => {
            match ((instr >> 28) & 7, (instr >> 26) & 3, (instr >> 25) & 1, (instr >> 20) & 9, (instr >> 8) & 23, (instr >> 7) & 1, (instr >> 5) & 3, (instr >> 4) & 1, instr & 7) {
                (_, _, x2, _, _, x5, x6, x7, _) if x2 == 0 && x5 == 1 && x6 != 0 && x7 == 1 => {
                    match ((instr >> 28) & 7, (instr >> 25) & 5, (instr >> 23) & 3, (instr >> 22) & 1, (instr >> 8) & 27, (instr >> 7) & 1, (instr >> 5) & 3, (instr >> 4) & 1, instr & 7) {
                        (_, _, _, x3, _, _, _, _, _) if x3 == 0 => {
                            let cond = (instr >> 28) & 7;
                            let P = (instr >> 24) & 1;
                            let U = (instr >> 23) & 1;
                            let W = (instr >> 21) & 1;
                            let o1 = (instr >> 20) & 1;
                            let Rn = (instr >> 16) & 7;
                            let Rt = (instr >> 12) & 7;
                            let op2 = (instr >> 5) & 3;
                            let Rm = instr & 7;
                            match (P, W, o1, op2) {
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 => {
                                    Some(Op::aarch32_STRH_r_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 2 => {
                                    Some(Op::aarch32_LDRD_r_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 3 => {
                                    Some(Op::aarch32_STRD_r_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 => {
                                    Some(Op::aarch32_LDRH_r_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 2 => {
                                    Some(Op::aarch32_LDRSB_r_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 3 => {
                                    Some(Op::aarch32_LDRSH_r_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 1 => {
                                    Some(Op::aarch32_STRHT_A2_A)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 2 => {
                                    None
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 3 => {
                                    None
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 1 => {
                                    Some(Op::aarch32_LDRHT_A2_A)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 2 => {
                                    Some(Op::aarch32_LDRSBT_A2_A)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 3 => {
                                    Some(Op::aarch32_LDRSHT_A2_A)
                                }
                                (x0, _, x2, x3) if x0 == 1 && x2 == 0 && x3 == 1 => {
                                    Some(Op::aarch32_STRH_r_A1_A)
                                }
                                (x0, _, x2, x3) if x0 == 1 && x2 == 0 && x3 == 2 => {
                                    Some(Op::aarch32_LDRD_r_A1_A)
                                }
                                (x0, _, x2, x3) if x0 == 1 && x2 == 0 && x3 == 3 => {
                                    Some(Op::aarch32_STRD_r_A1_A)
                                }
                                (x0, _, x2, x3) if x0 == 1 && x2 == 1 && x3 == 1 => {
                                    Some(Op::aarch32_LDRH_r_A1_A)
                                }
                                (x0, _, x2, x3) if x0 == 1 && x2 == 1 && x3 == 2 => {
                                    Some(Op::aarch32_LDRSB_r_A1_A)
                                }
                                (x0, _, x2, x3) if x0 == 1 && x2 == 1 && x3 == 3 => {
                                    Some(Op::aarch32_LDRSH_r_A1_A)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, x3, _, _, _, _, _) if x3 == 1 => {
                            let cond = (instr >> 28) & 7;
                            let P = (instr >> 24) & 1;
                            let U = (instr >> 23) & 1;
                            let W = (instr >> 21) & 1;
                            let o1 = (instr >> 20) & 1;
                            let Rn = (instr >> 16) & 7;
                            let Rt = (instr >> 12) & 7;
                            let imm4H = (instr >> 8) & 7;
                            let op2 = (instr >> 5) & 3;
                            let imm4L = instr & 7;
                            match ((P << 1) | W, o1, Rn, op2) {
                                (_, x1, x2, x3) if x1 == 0 && x2 == 15 && x3 == 2 => {
                                    Some(Op::aarch32_LDRD_l_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 != 1 && x1 == 1 && x2 == 15 && x3 == 1 => {
                                    Some(Op::aarch32_LDRH_l_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 != 1 && x1 == 1 && x2 == 15 && x3 == 2 => {
                                    Some(Op::aarch32_LDRSB_l_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 != 1 && x1 == 1 && x2 == 15 && x3 == 3 => {
                                    Some(Op::aarch32_LDRSH_l_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 != 15 && x3 == 2 => {
                                    Some(Op::aarch32_LDRD_i_A1_A)
                                }
                                (x0, x1, _, x3) if x0 == 0 && x1 == 0 && x3 == 1 => {
                                    Some(Op::aarch32_STRH_i_A1_A)
                                }
                                (x0, x1, _, x3) if x0 == 0 && x1 == 0 && x3 == 3 => {
                                    Some(Op::aarch32_STRD_i_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 != 15 && x3 == 1 => {
                                    Some(Op::aarch32_LDRH_i_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 != 15 && x3 == 2 => {
                                    Some(Op::aarch32_LDRSB_i_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 != 15 && x3 == 3 => {
                                    Some(Op::aarch32_LDRSH_i_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 != 15 && x3 == 2 => {
                                    None
                                }
                                (x0, x1, _, x3) if x0 == 1 && x1 == 0 && x3 == 1 => {
                                    Some(Op::aarch32_STRHT_A1_A)
                                }
                                (x0, x1, _, x3) if x0 == 1 && x1 == 0 && x3 == 3 => {
                                    None
                                }
                                (x0, x1, _, x3) if x0 == 1 && x1 == 1 && x3 == 1 => {
                                    Some(Op::aarch32_LDRHT_A1_A)
                                }
                                (x0, x1, _, x3) if x0 == 1 && x1 == 1 && x3 == 2 => {
                                    Some(Op::aarch32_LDRSBT_A1_A)
                                }
                                (x0, x1, _, x3) if x0 == 1 && x1 == 1 && x3 == 3 => {
                                    Some(Op::aarch32_LDRSHT_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 0 && x2 != 15 && x3 == 2 => {
                                    Some(Op::aarch32_LDRD_i_A1_A)
                                }
                                (x0, x1, _, x3) if x0 == 2 && x1 == 0 && x3 == 1 => {
                                    Some(Op::aarch32_STRH_i_A1_A)
                                }
                                (x0, x1, _, x3) if x0 == 2 && x1 == 0 && x3 == 3 => {
                                    Some(Op::aarch32_STRD_i_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 1 && x2 != 15 && x3 == 1 => {
                                    Some(Op::aarch32_LDRH_i_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 1 && x2 != 15 && x3 == 2 => {
                                    Some(Op::aarch32_LDRSB_i_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 1 && x2 != 15 && x3 == 3 => {
                                    Some(Op::aarch32_LDRSH_i_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 0 && x2 != 15 && x3 == 2 => {
                                    Some(Op::aarch32_LDRD_i_A1_A)
                                }
                                (x0, x1, _, x3) if x0 == 3 && x1 == 0 && x3 == 1 => {
                                    Some(Op::aarch32_STRH_i_A1_A)
                                }
                                (x0, x1, _, x3) if x0 == 3 && x1 == 0 && x3 == 3 => {
                                    Some(Op::aarch32_STRD_i_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 1 && x2 != 15 && x3 == 1 => {
                                    Some(Op::aarch32_LDRH_i_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 1 && x2 != 15 && x3 == 2 => {
                                    Some(Op::aarch32_LDRSB_i_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 1 && x2 != 15 && x3 == 3 => {
                                    Some(Op::aarch32_LDRSH_i_A1_A)
                                }
                                _ => None
                            }
                        }
                        _ => None
                    }
                }
                (_, _, x2, x3, _, x5, x6, x7, _) if x2 == 0 && x3 & 16 == 0 && x5 == 1 && x6 == 0 && x7 == 1 => {
                    let cond = (instr >> 28) & 7;
                    let opc = (instr >> 21) & 5;
                    let S = (instr >> 20) & 1;
                    let RdHi = (instr >> 16) & 7;
                    let RdLo = (instr >> 12) & 7;
                    let Rm = (instr >> 8) & 7;
                    let Rn = instr & 7;
                    match (opc, S) {
                        (x0, _) if x0 == 0 => {
                            Some(Op::aarch32_MUL_A1_A)
                        }
                        (x0, _) if x0 == 1 => {
                            Some(Op::aarch32_MLA_A1_A)
                        }
                        (x0, x1) if x0 == 2 && x1 == 0 => {
                            Some(Op::aarch32_UMAAL_A1_A)
                        }
                        (x0, x1) if x0 == 2 && x1 == 1 => {
                            None
                        }
                        (x0, x1) if x0 == 3 && x1 == 0 => {
                            Some(Op::aarch32_MLS_A1_A)
                        }
                        (x0, x1) if x0 == 3 && x1 == 1 => {
                            None
                        }
                        (x0, _) if x0 == 4 => {
                            Some(Op::aarch32_UMULL_A1_A)
                        }
                        (x0, _) if x0 == 5 => {
                            Some(Op::aarch32_UMLAL_A1_A)
                        }
                        (x0, _) if x0 == 6 => {
                            Some(Op::aarch32_SMULL_A1_A)
                        }
                        (x0, _) if x0 == 7 => {
                            Some(Op::aarch32_SMLAL_A1_A)
                        }
                        _ => None
                    }
                }
                (_, _, x2, x3, _, x5, x6, x7, _) if x2 == 0 && x3 & 16 == 16 && x5 == 1 && x6 == 0 && x7 == 1 => {
                    match ((instr >> 28) & 7, (instr >> 24) & 7, (instr >> 23) & 1, (instr >> 12) & 21, (instr >> 10) & 3, (instr >> 8) & 3, (instr >> 4) & 7, instr & 7) {
                        (_, _, x2, _, _, _, _, _) if x2 == 0 => {
                            None
                        }
                        (_, _, x2, _, _, _, _, _) if x2 == 1 => {
                            let cond = (instr >> 28) & 7;
                            let size = (instr >> 21) & 3;
                            let L = (instr >> 20) & 1;
                            let Rn = (instr >> 16) & 7;
                            let xRd = (instr >> 12) & 7;
                            let ex = (instr >> 9) & 1;
                            let ord = (instr >> 8) & 1;
                            let xRt = instr & 7;
                            match (size, L, ex, ord) {
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 => {
                                    Some(Op::aarch32_STL_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 => {
                                    None
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => {
                                    Some(Op::aarch32_STLEX_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 => {
                                    Some(Op::aarch32_STREX_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 0 => {
                                    Some(Op::aarch32_LDA_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 1 => {
                                    None
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 0 => {
                                    Some(Op::aarch32_LDAEX_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 1 => {
                                    Some(Op::aarch32_LDREX_A1_A)
                                }
                                (x0, x1, x2, _) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    None
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 => {
                                    Some(Op::aarch32_STLEXD_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 => {
                                    Some(Op::aarch32_STREXD_A1_A)
                                }
                                (x0, x1, x2, _) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    None
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 0 => {
                                    Some(Op::aarch32_LDAEXD_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 1 => {
                                    Some(Op::aarch32_LDREXD_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 => {
                                    Some(Op::aarch32_STLB_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 => {
                                    None
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 => {
                                    Some(Op::aarch32_STLEXB_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 => {
                                    Some(Op::aarch32_STREXB_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 1 && x2 == 0 && x3 == 0 => {
                                    Some(Op::aarch32_LDAB_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 1 && x2 == 0 && x3 == 1 => {
                                    None
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 1 && x2 == 1 && x3 == 0 => {
                                    Some(Op::aarch32_LDAEXB_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 1 && x2 == 1 && x3 == 1 => {
                                    Some(Op::aarch32_LDREXB_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 => {
                                    Some(Op::aarch32_STLH_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 => {
                                    None
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 => {
                                    Some(Op::aarch32_STLEXH_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 => {
                                    Some(Op::aarch32_STREXH_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 1 && x2 == 0 && x3 == 0 => {
                                    Some(Op::aarch32_LDAH_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 1 && x2 == 0 && x3 == 1 => {
                                    None
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 1 && x2 == 1 && x3 == 0 => {
                                    Some(Op::aarch32_LDAEXH_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 1 && x2 == 1 && x3 == 1 => {
                                    Some(Op::aarch32_LDREXH_A1_A)
                                }
                                _ => None
                            }
                        }
                        _ => None
                    }
                }
                (_, _, x2, x3, _, x5, _, _, _) if x2 == 0 && x3 & 25 == 16 && x5 == 0 => {
                    match ((instr >> 28) & 7, (instr >> 23) & 9, (instr >> 21) & 3, (instr >> 20) & 1, (instr >> 8) & 23, (instr >> 7) & 1, (instr >> 4) & 5, instr & 7) {
                        (_, _, x2, _, _, _, x6, _) if x2 == 0 && x6 == 1 => {
                            None
                        }
                        (_, _, x2, _, _, _, x6, _) if x2 == 0 && x6 == 2 => {
                            None
                        }
                        (_, _, x2, _, _, _, x6, _) if x2 == 0 && x6 == 3 => {
                            None
                        }
                        (_, _, x2, _, _, _, x6, _) if x2 == 0 && x6 == 6 => {
                            None
                        }
                        (_, _, x2, _, _, _, x6, _) if x2 == 1 && x6 == 1 => {
                            let cond = (instr >> 28) & 7;
                            let Rm = instr & 7;
                            match () {
                                () => {
                                    Some(Op::aarch32_BX_A1_A)
                                }
                            }
                        }
                        (_, _, x2, _, _, _, x6, _) if x2 == 1 && x6 == 2 => {
                            let cond = (instr >> 28) & 7;
                            let Rm = instr & 7;
                            match () {
                                () => {
                                    Some(Op::aarch32_BXJ_A1_A)
                                }
                            }
                        }
                        (_, _, x2, _, _, _, x6, _) if x2 == 1 && x6 == 3 => {
                            let cond = (instr >> 28) & 7;
                            let Rm = instr & 7;
                            match () {
                                () => {
                                    Some(Op::aarch32_BLX_r_A1_A)
                                }
                            }
                        }
                        (_, _, x2, _, _, _, x6, _) if x2 == 1 && x6 == 6 => {
                            None
                        }
                        (_, _, x2, _, _, _, x6, _) if x2 == 2 && x6 == 1 => {
                            None
                        }
                        (_, _, x2, _, _, _, x6, _) if x2 == 2 && x6 == 2 => {
                            None
                        }
                        (_, _, x2, _, _, _, x6, _) if x2 == 2 && x6 == 3 => {
                            None
                        }
                        (_, _, x2, _, _, _, x6, _) if x2 == 2 && x6 == 6 => {
                            None
                        }
                        (_, _, x2, _, _, _, x6, _) if x2 == 3 && x6 == 1 => {
                            let cond = (instr >> 28) & 7;
                            let Rd = (instr >> 12) & 7;
                            let Rm = instr & 7;
                            match () {
                                () => {
                                    Some(Op::aarch32_CLZ_A1_A)
                                }
                            }
                        }
                        (_, _, x2, _, _, _, x6, _) if x2 == 3 && x6 == 2 => {
                            None
                        }
                        (_, _, x2, _, _, _, x6, _) if x2 == 3 && x6 == 3 => {
                            None
                        }
                        (_, _, x2, _, _, _, x6, _) if x2 == 3 && x6 == 6 => {
                            let cond = (instr >> 28) & 7;
                            match () {
                                () => {
                                    Some(Op::aarch32_ERET_A1_A)
                                }
                            }
                        }
                        (_, _, _, _, _, _, x6, _) if x6 == 7 => {
                            let cond = (instr >> 28) & 7;
                            let opc = (instr >> 21) & 3;
                            let imm12 = (instr >> 8) & 23;
                            let imm4 = instr & 7;
                            match opc {
                                x0 if x0 == 0 => {
                                    Some(Op::aarch32_HLT_A1_A)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::aarch32_BKPT_A1_A)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::aarch32_HVC_A1_A)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::aarch32_SMC_A1_AS)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, _, _, _, x6, _) if x6 == 0 => {
                            let cond = (instr >> 28) & 7;
                            let opc = (instr >> 21) & 3;
                            let mask = (instr >> 16) & 7;
                            let Rd = (instr >> 12) & 7;
                            let B = (instr >> 9) & 1;
                            let m = (instr >> 8) & 1;
                            let Rn = instr & 7;
                            match (opc, B) {
                                (x0, x1) if x0 & 1 == 0 && x1 == 0 => {
                                    Some(Op::aarch32_MRS_A1_AS)
                                }
                                (x0, x1) if x0 & 1 == 0 && x1 == 1 => {
                                    Some(Op::aarch32_MRS_br_A1_AS)
                                }
                                (x0, x1) if x0 & 1 == 1 && x1 == 0 => {
                                    Some(Op::aarch32_MSR_r_A1_AS)
                                }
                                (x0, x1) if x0 & 1 == 1 && x1 == 1 => {
                                    Some(Op::aarch32_MSR_br_A1_AS)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, _, _, _, x6, _) if x6 == 4 => {
                            let cond = (instr >> 28) & 7;
                            let sz = (instr >> 21) & 3;
                            let Rn = (instr >> 16) & 7;
                            let Rd = (instr >> 12) & 7;
                            let C = (instr >> 9) & 1;
                            let Rm = instr & 7;
                            match (sz, C) {
                                (x0, x1) if x0 == 0 && x1 == 0 => {
                                    Some(Op::aarch32_CRC32_A1_A)
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    Some(Op::aarch32_CRC32_A1_A)
                                }
                                (x0, x1) if x0 == 1 && x1 == 0 => {
                                    Some(Op::aarch32_CRC32_A1_A)
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    Some(Op::aarch32_CRC32_A1_A)
                                }
                                (x0, x1) if x0 == 2 && x1 == 0 => {
                                    Some(Op::aarch32_CRC32_A1_A)
                                }
                                (x0, x1) if x0 == 2 && x1 == 1 => {
                                    Some(Op::aarch32_CRC32_A1_A)
                                }
                                (x0, _) if x0 == 3 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        (_, _, _, _, _, _, x6, _) if x6 == 5 => {
                            let cond = (instr >> 28) & 7;
                            let opc = (instr >> 21) & 3;
                            let Rn = (instr >> 16) & 7;
                            let Rd = (instr >> 12) & 7;
                            let Rm = instr & 7;
                            match opc {
                                x0 if x0 == 0 => {
                                    Some(Op::aarch32_QADD_A1_A)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::aarch32_QSUB_A1_A)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::aarch32_QDADD_A1_A)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::aarch32_QDSUB_A1_A)
                                }
                                _ => None
                            }
                        }
                        _ => None
                    }
                }
                (_, _, x2, x3, _, x5, _, x7, _) if x2 == 0 && x3 & 25 == 16 && x5 == 1 && x7 == 0 => {
                    let cond = (instr >> 28) & 7;
                    let opc = (instr >> 21) & 3;
                    let Rd = (instr >> 16) & 7;
                    let Ra = (instr >> 12) & 7;
                    let Rm = (instr >> 8) & 7;
                    let M = (instr >> 6) & 1;
                    let N = (instr >> 5) & 1;
                    let Rn = instr & 7;
                    match (opc, M, N) {
                        (x0, _, _) if x0 == 0 => {
                            Some(Op::aarch32_SMLABB_A1_A)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch32_SMLAWB_A1_A)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch32_SMULWB_A1_A)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch32_SMLAWB_A1_A)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            Some(Op::aarch32_SMULWB_A1_A)
                        }
                        (x0, _, _) if x0 == 2 => {
                            Some(Op::aarch32_SMLALBB_A1_A)
                        }
                        (x0, _, _) if x0 == 3 => {
                            Some(Op::aarch32_SMULBB_A1_A)
                        }
                        _ => None
                    }
                }
                (_, _, x2, x3, _, _, _, x7, _) if x2 == 0 && x3 & 25 != 16 && x7 == 0 => {
                    match ((instr >> 28) & 7, (instr >> 25) & 5, (instr >> 23) & 3, (instr >> 21) & 3, (instr >> 20) & 1, (instr >> 5) & 29, (instr >> 4) & 1, instr & 7) {
                        (_, _, x2, _, _, _, _, _) if x2 & 2 == 0 => {
                            let cond = (instr >> 28) & 7;
                            let opc = (instr >> 21) & 5;
                            let S = (instr >> 20) & 1;
                            let Rn = (instr >> 16) & 7;
                            let Rd = (instr >> 12) & 7;
                            let imm5 = (instr >> 7) & 9;
                            let stype = (instr >> 5) & 3;
                            let Rm = instr & 7;
                            match (opc, S, Rn) {
                                (x0, _, _) if x0 == 0 => {
                                    Some(Op::aarch32_AND_r_A1_A)
                                }
                                (x0, _, _) if x0 == 1 => {
                                    Some(Op::aarch32_EOR_r_A1_A)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 != 13 => {
                                    Some(Op::aarch32_SUB_r_A1_A)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 13 => {
                                    Some(Op::aarch32_SUB_SP_r_A1_A)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 != 13 => {
                                    Some(Op::aarch32_SUB_r_A1_A)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 13 => {
                                    Some(Op::aarch32_SUB_SP_r_A1_A)
                                }
                                (x0, _, _) if x0 == 3 => {
                                    Some(Op::aarch32_RSB_r_A1_A)
                                }
                                (x0, x1, x2) if x0 == 4 && x1 == 0 && x2 != 13 => {
                                    Some(Op::aarch32_ADD_r_A1_A)
                                }
                                (x0, x1, x2) if x0 == 4 && x1 == 0 && x2 == 13 => {
                                    Some(Op::aarch32_ADD_SP_r_A1_A)
                                }
                                (x0, x1, x2) if x0 == 4 && x1 == 1 && x2 != 13 => {
                                    Some(Op::aarch32_ADD_r_A1_A)
                                }
                                (x0, x1, x2) if x0 == 4 && x1 == 1 && x2 == 13 => {
                                    Some(Op::aarch32_ADD_SP_r_A1_A)
                                }
                                (x0, _, _) if x0 == 5 => {
                                    Some(Op::aarch32_ADC_r_A1_A)
                                }
                                (x0, _, _) if x0 == 6 => {
                                    Some(Op::aarch32_SBC_r_A1_A)
                                }
                                (x0, _, _) if x0 == 7 => {
                                    Some(Op::aarch32_RSC_r_A1_A)
                                }
                                _ => None
                            }
                        }
                        (_, _, x2, _, x4, _, _, _) if x2 == 2 && x4 == 1 => {
                            let cond = (instr >> 28) & 7;
                            let opc = (instr >> 21) & 3;
                            let Rn = (instr >> 16) & 7;
                            let imm5 = (instr >> 7) & 9;
                            let stype = (instr >> 5) & 3;
                            let Rm = instr & 7;
                            match opc {
                                x0 if x0 == 0 => {
                                    Some(Op::aarch32_TST_r_A1_A)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::aarch32_TEQ_r_A1_A)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::aarch32_CMP_r_A1_A)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::aarch32_CMN_r_A1_A)
                                }
                                _ => None
                            }
                        }
                        (_, _, x2, _, _, _, _, _) if x2 == 3 => {
                            let cond = (instr >> 28) & 7;
                            let opc = (instr >> 21) & 3;
                            let S = (instr >> 20) & 1;
                            let Rn = (instr >> 16) & 7;
                            let Rd = (instr >> 12) & 7;
                            let imm5 = (instr >> 7) & 9;
                            let stype = (instr >> 5) & 3;
                            let Rm = instr & 7;
                            match opc {
                                x0 if x0 == 0 => {
                                    Some(Op::aarch32_ORR_r_A1_A)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::aarch32_MOV_r_A1_A)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::aarch32_BIC_r_A1_A)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::aarch32_MVN_r_A1_A)
                                }
                                _ => None
                            }
                        }
                        _ => None
                    }
                }
                (_, _, x2, x3, _, x5, _, x7, _) if x2 == 0 && x3 & 25 != 16 && x5 == 0 && x7 == 1 => {
                    match ((instr >> 28) & 7, (instr >> 25) & 5, (instr >> 23) & 3, (instr >> 21) & 3, (instr >> 20) & 1, (instr >> 8) & 23, (instr >> 7) & 1, (instr >> 5) & 3, (instr >> 4) & 1, instr & 7) {
                        (_, _, x2, _, _, _, _, _, _, _) if x2 & 2 == 0 => {
                            let cond = (instr >> 28) & 7;
                            let opc = (instr >> 21) & 5;
                            let S = (instr >> 20) & 1;
                            let Rn = (instr >> 16) & 7;
                            let Rd = (instr >> 12) & 7;
                            let Rs = (instr >> 8) & 7;
                            let stype = (instr >> 5) & 3;
                            let Rm = instr & 7;
                            match opc {
                                x0 if x0 == 0 => {
                                    Some(Op::aarch32_AND_rr_A1_A)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::aarch32_EOR_rr_A1_A)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::aarch32_SUB_rr_A1_A)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::aarch32_RSB_rr_A1_A)
                                }
                                x0 if x0 == 4 => {
                                    Some(Op::aarch32_ADD_rr_A1_A)
                                }
                                x0 if x0 == 5 => {
                                    Some(Op::aarch32_ADC_rr_A1_A)
                                }
                                x0 if x0 == 6 => {
                                    Some(Op::aarch32_SBC_rr_A1_A)
                                }
                                x0 if x0 == 7 => {
                                    Some(Op::aarch32_RSC_rr_A1_A)
                                }
                                _ => None
                            }
                        }
                        (_, _, x2, _, x4, _, _, _, _, _) if x2 == 2 && x4 == 1 => {
                            let cond = (instr >> 28) & 7;
                            let opc = (instr >> 21) & 3;
                            let Rn = (instr >> 16) & 7;
                            let Rs = (instr >> 8) & 7;
                            let stype = (instr >> 5) & 3;
                            let Rm = instr & 7;
                            match opc {
                                x0 if x0 == 0 => {
                                    Some(Op::aarch32_TST_rr_A1_A)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::aarch32_TEQ_rr_A1_A)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::aarch32_CMP_rr_A1_A)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::aarch32_CMN_rr_A1_A)
                                }
                                _ => None
                            }
                        }
                        (_, _, x2, _, _, _, _, _, _, _) if x2 == 3 => {
                            let cond = (instr >> 28) & 7;
                            let opc = (instr >> 21) & 3;
                            let S = (instr >> 20) & 1;
                            let Rn = (instr >> 16) & 7;
                            let Rd = (instr >> 12) & 7;
                            let Rs = (instr >> 8) & 7;
                            let stype = (instr >> 5) & 3;
                            let Rm = instr & 7;
                            match opc {
                                x0 if x0 == 0 => {
                                    Some(Op::aarch32_ORR_rr_A1_A)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::aarch32_MOV_rr_A1_A)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::aarch32_BIC_rr_A1_A)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::aarch32_MVN_rr_A1_A)
                                }
                                _ => None
                            }
                        }
                        _ => None
                    }
                }
                (_, _, x2, _, _, _, _, _, _) if x2 == 1 => {
                    match ((instr >> 28) & 7, (instr >> 25) & 5, (instr >> 23) & 3, (instr >> 22) & 1, (instr >> 20) & 3, instr & 39) {
                        (_, _, x2, _, _, _) if x2 & 2 == 0 => {
                            let cond = (instr >> 28) & 7;
                            let opc = (instr >> 21) & 5;
                            let S = (instr >> 20) & 1;
                            let Rn = (instr >> 16) & 7;
                            let Rd = (instr >> 12) & 7;
                            let imm12 = instr & 23;
                            match (opc, S, Rn) {
                                (x0, _, _) if x0 == 0 => {
                                    Some(Op::aarch32_AND_i_A1_A)
                                }
                                (x0, _, _) if x0 == 1 => {
                                    Some(Op::aarch32_EOR_i_A1_A)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 & 13 != 13 => {
                                    Some(Op::aarch32_SUB_i_A1_A)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 13 => {
                                    Some(Op::aarch32_SUB_SP_i_A1_A)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 15 => {
                                    Some(Op::aarch32_ADR_A1_A)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 != 13 => {
                                    Some(Op::aarch32_SUB_i_A1_A)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 13 => {
                                    Some(Op::aarch32_SUB_SP_i_A1_A)
                                }
                                (x0, _, _) if x0 == 3 => {
                                    Some(Op::aarch32_RSB_i_A1_A)
                                }
                                (x0, x1, x2) if x0 == 4 && x1 == 0 && x2 & 13 != 13 => {
                                    Some(Op::aarch32_ADD_i_A1_A)
                                }
                                (x0, x1, x2) if x0 == 4 && x1 == 0 && x2 == 13 => {
                                    Some(Op::aarch32_ADD_SP_i_A1_A)
                                }
                                (x0, x1, x2) if x0 == 4 && x1 == 0 && x2 == 15 => {
                                    Some(Op::aarch32_ADR_A1_A)
                                }
                                (x0, x1, x2) if x0 == 4 && x1 == 1 && x2 != 13 => {
                                    Some(Op::aarch32_ADD_i_A1_A)
                                }
                                (x0, x1, x2) if x0 == 4 && x1 == 1 && x2 == 13 => {
                                    Some(Op::aarch32_ADD_SP_i_A1_A)
                                }
                                (x0, _, _) if x0 == 5 => {
                                    Some(Op::aarch32_ADC_i_A1_A)
                                }
                                (x0, _, _) if x0 == 6 => {
                                    Some(Op::aarch32_SBC_i_A1_A)
                                }
                                (x0, _, _) if x0 == 7 => {
                                    Some(Op::aarch32_RSC_i_A1_A)
                                }
                                _ => None
                            }
                        }
                        (_, _, x2, _, x4, _) if x2 == 2 && x4 == 0 => {
                            let cond = (instr >> 28) & 7;
                            let H = (instr >> 22) & 1;
                            let imm4 = (instr >> 16) & 7;
                            let Rd = (instr >> 12) & 7;
                            let imm12 = instr & 23;
                            match H {
                                x0 if x0 == 0 => {
                                    Some(Op::aarch32_MOV_i_A2_A)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::aarch32_MOVT_A1_A)
                                }
                                _ => None
                            }
                        }
                        (_, _, x2, _, x4, _) if x2 == 2 && x4 == 2 => {
                            let cond = (instr >> 28) & 7;
                            let R = (instr >> 22) & 1;
                            let imm4 = (instr >> 16) & 7;
                            let imm12 = instr & 23;
                            match ((R << 4) | imm4, imm12) {
                                (x0, _) if x0 != 0 => {
                                    Some(Op::aarch32_MSR_i_A1_AS)
                                }
                                (x0, x1) if x0 == 0 && x1 & 255 == 0 => {
                                    Some(Op::aarch32_NOP_A1_A)
                                }
                                (x0, x1) if x0 == 0 && x1 & 255 == 1 => {
                                    Some(Op::aarch32_YIELD_A1_A)
                                }
                                (x0, x1) if x0 == 0 && x1 & 255 == 2 => {
                                    Some(Op::aarch32_WFE_A1_A)
                                }
                                (x0, x1) if x0 == 0 && x1 & 255 == 3 => {
                                    Some(Op::aarch32_WFI_A1_A)
                                }
                                (x0, x1) if x0 == 0 && x1 & 255 == 4 => {
                                    Some(Op::aarch32_SEV_A1_A)
                                }
                                (x0, x1) if x0 == 0 && x1 & 255 == 5 => {
                                    Some(Op::aarch32_SEVL_A1_A)
                                }
                                (x0, x1) if x0 == 0 && x1 & 254 == 6 => {
                                    Some(Op::Nop)
                                }
                                (x0, x1) if x0 == 0 && x1 & 248 == 8 => {
                                    Some(Op::Nop)
                                }
                                (x0, x1) if x0 == 0 && x1 & 255 == 16 => {
                                    Some(Op::aarch32_ESB_A1_A)
                                }
                                (x0, x1) if x0 == 0 && x1 & 255 == 17 => {
                                    Some(Op::Nop)
                                }
                                (x0, x1) if x0 == 0 && x1 & 255 == 18 => {
                                    Some(Op::aarch32_TSB_A1_A)
                                }
                                (x0, x1) if x0 == 0 && x1 & 255 == 19 => {
                                    Some(Op::Nop)
                                }
                                (x0, x1) if x0 == 0 && x1 & 255 == 20 => {
                                    Some(Op::aarch32_CSDB_A1_A)
                                }
                                (x0, x1) if x0 == 0 && x1 & 255 == 21 => {
                                    Some(Op::Nop)
                                }
                                (x0, x1) if x0 == 0 && x1 & 248 == 24 => {
                                    Some(Op::Nop)
                                }
                                (x0, x1) if x0 == 0 && x1 & 254 == 30 => {
                                    Some(Op::Nop)
                                }
                                (x0, x1) if x0 == 0 && x1 & 224 == 32 => {
                                    Some(Op::Nop)
                                }
                                (x0, x1) if x0 == 0 && x1 & 192 == 64 => {
                                    Some(Op::Nop)
                                }
                                (x0, x1) if x0 == 0 && x1 & 192 == 128 => {
                                    Some(Op::Nop)
                                }
                                (x0, x1) if x0 == 0 && x1 & 224 == 192 => {
                                    Some(Op::Nop)
                                }
                                (x0, x1) if x0 == 0 && x1 & 240 == 224 => {
                                    Some(Op::Nop)
                                }
                                (x0, x1) if x0 == 0 && x1 & 240 == 240 => {
                                    Some(Op::aarch32_DBG_A1_A)
                                }
                                _ => None
                            }
                        }
                        (_, _, x2, _, x4, _) if x2 == 2 && x4 & 1 == 1 => {
                            let cond = (instr >> 28) & 7;
                            let opc = (instr >> 21) & 3;
                            let Rn = (instr >> 16) & 7;
                            let imm12 = instr & 23;
                            match opc {
                                x0 if x0 == 0 => {
                                    Some(Op::aarch32_TST_i_A1_A)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::aarch32_TEQ_i_A1_A)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::aarch32_CMP_i_A1_A)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::aarch32_CMN_i_A1_A)
                                }
                                _ => None
                            }
                        }
                        (_, _, x2, _, _, _) if x2 == 3 => {
                            let cond = (instr >> 28) & 7;
                            let opc = (instr >> 21) & 3;
                            let S = (instr >> 20) & 1;
                            let Rn = (instr >> 16) & 7;
                            let Rd = (instr >> 12) & 7;
                            let imm12 = instr & 23;
                            match opc {
                                x0 if x0 == 0 => {
                                    Some(Op::aarch32_ORR_i_A1_A)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::aarch32_MOV_i_A1_A)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::aarch32_BIC_i_A1_A)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::aarch32_MVN_i_A1_A)
                                }
                                _ => None
                            }
                        }
                        _ => None
                    }
                }
                _ => None
            }
        }
        (x0, x1, _, _, _) if x0 != 15 && x1 == 2 => {
            let cond = (instr >> 28) & 7;
            let P = (instr >> 24) & 1;
            let U = (instr >> 23) & 1;
            let o2 = (instr >> 22) & 1;
            let W = (instr >> 21) & 1;
            let o1 = (instr >> 20) & 1;
            let Rn = (instr >> 16) & 7;
            let Rt = (instr >> 12) & 7;
            let imm12 = instr & 23;
            match ((P << 1) | W, o2, o1, Rn) {
                (x0, x1, x2, x3) if x0 != 1 && x1 == 0 && x2 == 1 && x3 == 15 => {
                    Some(Op::aarch32_LDR_l_A1_A)
                }
                (x0, x1, x2, x3) if x0 != 1 && x1 == 1 && x2 == 1 && x3 == 15 => {
                    Some(Op::aarch32_LDRB_l_A1_A)
                }
                (x0, x1, x2, _) if x0 == 0 && x1 == 0 && x2 == 0 => {
                    Some(Op::aarch32_STR_i_A1_A)
                }
                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 != 15 => {
                    Some(Op::aarch32_LDR_i_A1_A)
                }
                (x0, x1, x2, _) if x0 == 0 && x1 == 1 && x2 == 0 => {
                    Some(Op::aarch32_STRB_i_A1_A)
                }
                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 != 15 => {
                    Some(Op::aarch32_LDRB_i_A1_A)
                }
                (x0, x1, x2, _) if x0 == 1 && x1 == 0 && x2 == 0 => {
                    Some(Op::aarch32_STRT_A1_A)
                }
                (x0, x1, x2, _) if x0 == 1 && x1 == 0 && x2 == 1 => {
                    Some(Op::aarch32_LDRT_A1_A)
                }
                (x0, x1, x2, _) if x0 == 1 && x1 == 1 && x2 == 0 => {
                    Some(Op::aarch32_STRBT_A1_A)
                }
                (x0, x1, x2, _) if x0 == 1 && x1 == 1 && x2 == 1 => {
                    Some(Op::aarch32_LDRBT_A1_A)
                }
                (x0, x1, x2, _) if x0 == 2 && x1 == 0 && x2 == 0 => {
                    Some(Op::aarch32_STR_i_A1_A)
                }
                (x0, x1, x2, x3) if x0 == 2 && x1 == 0 && x2 == 1 && x3 != 15 => {
                    Some(Op::aarch32_LDR_i_A1_A)
                }
                (x0, x1, x2, _) if x0 == 2 && x1 == 1 && x2 == 0 => {
                    Some(Op::aarch32_STRB_i_A1_A)
                }
                (x0, x1, x2, x3) if x0 == 2 && x1 == 1 && x2 == 1 && x3 != 15 => {
                    Some(Op::aarch32_LDRB_i_A1_A)
                }
                (x0, x1, x2, _) if x0 == 3 && x1 == 0 && x2 == 0 => {
                    Some(Op::aarch32_STR_i_A1_A)
                }
                (x0, x1, x2, x3) if x0 == 3 && x1 == 0 && x2 == 1 && x3 != 15 => {
                    Some(Op::aarch32_LDR_i_A1_A)
                }
                (x0, x1, x2, _) if x0 == 3 && x1 == 1 && x2 == 0 => {
                    Some(Op::aarch32_STRB_i_A1_A)
                }
                (x0, x1, x2, x3) if x0 == 3 && x1 == 1 && x2 == 1 && x3 != 15 => {
                    Some(Op::aarch32_LDRB_i_A1_A)
                }
                _ => None
            }
        }
        (x0, x1, _, x3, _) if x0 != 15 && x1 == 3 && x3 == 0 => {
            let cond = (instr >> 28) & 7;
            let P = (instr >> 24) & 1;
            let U = (instr >> 23) & 1;
            let o2 = (instr >> 22) & 1;
            let W = (instr >> 21) & 1;
            let o1 = (instr >> 20) & 1;
            let Rn = (instr >> 16) & 7;
            let Rt = (instr >> 12) & 7;
            let imm5 = (instr >> 7) & 9;
            let stype = (instr >> 5) & 3;
            let Rm = instr & 7;
            match (P, o2, W, o1) {
                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 => {
                    Some(Op::aarch32_STR_r_A1_A)
                }
                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 => {
                    Some(Op::aarch32_LDR_r_A1_A)
                }
                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => {
                    Some(Op::aarch32_STRT_A2_A)
                }
                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 => {
                    Some(Op::aarch32_LDRT_A2_A)
                }
                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 0 => {
                    Some(Op::aarch32_STRB_r_A1_A)
                }
                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 1 => {
                    Some(Op::aarch32_LDRB_r_A1_A)
                }
                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 0 => {
                    Some(Op::aarch32_STRBT_A2_A)
                }
                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 1 => {
                    Some(Op::aarch32_LDRBT_A2_A)
                }
                (x0, x1, _, x3) if x0 == 1 && x1 == 0 && x3 == 0 => {
                    Some(Op::aarch32_STR_r_A1_A)
                }
                (x0, x1, _, x3) if x0 == 1 && x1 == 0 && x3 == 1 => {
                    Some(Op::aarch32_LDR_r_A1_A)
                }
                (x0, x1, _, x3) if x0 == 1 && x1 == 1 && x3 == 0 => {
                    Some(Op::aarch32_STRB_r_A1_A)
                }
                (x0, x1, _, x3) if x0 == 1 && x1 == 1 && x3 == 1 => {
                    Some(Op::aarch32_LDRB_r_A1_A)
                }
                _ => None
            }
        }
        (x0, x1, _, x3, _) if x0 != 15 && x1 == 3 && x3 == 1 => {
            match ((instr >> 28) & 7, (instr >> 25) & 5, (instr >> 20) & 9, (instr >> 8) & 23, (instr >> 5) & 5, (instr >> 4) & 1, instr & 7) {
                (_, _, x2, _, _, _, _) if x2 & 24 == 0 => {
                    let cond = (instr >> 28) & 7;
                    let op1 = (instr >> 20) & 5;
                    let Rn = (instr >> 16) & 7;
                    let Rd = (instr >> 12) & 7;
                    let B = (instr >> 7) & 1;
                    let op2 = (instr >> 5) & 3;
                    let Rm = instr & 7;
                    match (op1, B, op2) {
                        (x0, _, _) if x0 == 0 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch32_SADD16_A1_A)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch32_SASX_A1_A)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 2 => {
                            Some(Op::aarch32_SSAX_A1_A)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 3 => {
                            Some(Op::aarch32_SSUB16_A1_A)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch32_SADD8_A1_A)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 2 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 3 => {
                            Some(Op::aarch32_SSUB8_A1_A)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch32_QADD16_A1_A)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch32_QASX_A1_A)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 2 => {
                            Some(Op::aarch32_QSAX_A1_A)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 3 => {
                            Some(Op::aarch32_QSUB16_A1_A)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch32_QADD8_A1_A)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 2 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 3 => {
                            Some(Op::aarch32_QSUB8_A1_A)
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch32_SHADD16_A1_A)
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch32_SHASX_A1_A)
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 2 => {
                            Some(Op::aarch32_SHSAX_A1_A)
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 3 => {
                            Some(Op::aarch32_SHSUB16_A1_A)
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch32_SHADD8_A1_A)
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 1 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 2 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 3 => {
                            Some(Op::aarch32_SHSUB8_A1_A)
                        }
                        (x0, _, _) if x0 == 4 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 5 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch32_UADD16_A1_A)
                        }
                        (x0, x1, x2) if x0 == 5 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch32_UASX_A1_A)
                        }
                        (x0, x1, x2) if x0 == 5 && x1 == 0 && x2 == 2 => {
                            Some(Op::aarch32_USAX_A1_A)
                        }
                        (x0, x1, x2) if x0 == 5 && x1 == 0 && x2 == 3 => {
                            Some(Op::aarch32_USUB16_A1_A)
                        }
                        (x0, x1, x2) if x0 == 5 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch32_UADD8_A1_A)
                        }
                        (x0, x1, x2) if x0 == 5 && x1 == 1 && x2 == 1 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 5 && x1 == 1 && x2 == 2 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 5 && x1 == 1 && x2 == 3 => {
                            Some(Op::aarch32_USUB8_A1_A)
                        }
                        (x0, x1, x2) if x0 == 6 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch32_UQADD16_A1_A)
                        }
                        (x0, x1, x2) if x0 == 6 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch32_UQASX_A1_A)
                        }
                        (x0, x1, x2) if x0 == 6 && x1 == 0 && x2 == 2 => {
                            Some(Op::aarch32_UQSAX_A1_A)
                        }
                        (x0, x1, x2) if x0 == 6 && x1 == 0 && x2 == 3 => {
                            Some(Op::aarch32_UQSUB16_A1_A)
                        }
                        (x0, x1, x2) if x0 == 6 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch32_UQADD8_A1_A)
                        }
                        (x0, x1, x2) if x0 == 6 && x1 == 1 && x2 == 1 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 6 && x1 == 1 && x2 == 2 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 6 && x1 == 1 && x2 == 3 => {
                            Some(Op::aarch32_UQSUB8_A1_A)
                        }
                        (x0, x1, x2) if x0 == 7 && x1 == 0 && x2 == 0 => {
                            Some(Op::aarch32_UHADD16_A1_A)
                        }
                        (x0, x1, x2) if x0 == 7 && x1 == 0 && x2 == 1 => {
                            Some(Op::aarch32_UHASX_A1_A)
                        }
                        (x0, x1, x2) if x0 == 7 && x1 == 0 && x2 == 2 => {
                            Some(Op::aarch32_UHSAX_A1_A)
                        }
                        (x0, x1, x2) if x0 == 7 && x1 == 0 && x2 == 3 => {
                            Some(Op::aarch32_UHSUB16_A1_A)
                        }
                        (x0, x1, x2) if x0 == 7 && x1 == 1 && x2 == 0 => {
                            Some(Op::aarch32_UHADD8_A1_A)
                        }
                        (x0, x1, x2) if x0 == 7 && x1 == 1 && x2 == 1 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 7 && x1 == 1 && x2 == 2 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 7 && x1 == 1 && x2 == 3 => {
                            Some(Op::aarch32_UHSUB8_A1_A)
                        }
                        _ => None
                    }
                }
                (_, _, x2, _, x4, _, _) if x2 == 8 && x4 == 5 => {
                    let cond = (instr >> 28) & 7;
                    let Rn = (instr >> 16) & 7;
                    let Rd = (instr >> 12) & 7;
                    let Rm = instr & 7;
                    match () {
                        () => {
                            Some(Op::aarch32_SEL_A1_A)
                        }
                    }
                }
                (_, _, x2, _, x4, _, _) if x2 == 8 && x4 == 1 => {
                    None
                }
                (_, _, x2, _, x4, _, _) if x2 == 8 && x4 & 1 == 0 => {
                    let cond = (instr >> 28) & 7;
                    let Rn = (instr >> 16) & 7;
                    let Rd = (instr >> 12) & 7;
                    let imm5 = (instr >> 7) & 9;
                    let tb = (instr >> 6) & 1;
                    let Rm = instr & 7;
                    match () {
                        () => {
                            Some(Op::aarch32_PKH_A1_A)
                        }
                    }
                }
                (_, _, x2, _, x4, _, _) if x2 == 9 && x4 & 3 == 1 => {
                    None
                }
                (_, _, x2, _, x4, _, _) if x2 == 9 && x4 & 1 == 0 => {
                    None
                }
                (_, _, x2, _, x4, _, _) if x2 & 30 == 12 && x4 & 3 == 1 => {
                    None
                }
                (_, _, x2, _, x4, _, _) if x2 & 30 == 12 && x4 & 1 == 0 => {
                    None
                }
                (_, _, x2, _, x4, _, _) if x2 & 27 == 10 && x4 == 1 => {
                    let cond = (instr >> 28) & 7;
                    let U = (instr >> 22) & 1;
                    let sat_imm = (instr >> 16) & 7;
                    let Rd = (instr >> 12) & 7;
                    let Rn = instr & 7;
                    match U {
                        x0 if x0 == 0 => {
                            Some(Op::aarch32_SSAT16_A1_A)
                        }
                        x0 if x0 == 1 => {
                            Some(Op::aarch32_USAT16_A1_A)
                        }
                        _ => None
                    }
                }
                (_, _, x2, _, x4, _, _) if x2 & 27 == 10 && x4 == 5 => {
                    None
                }
                (_, _, x2, _, x4, _, _) if x2 & 27 == 11 && x4 & 3 == 1 => {
                    let cond = (instr >> 28) & 7;
                    let o1 = (instr >> 22) & 1;
                    let Rd = (instr >> 12) & 7;
                    let o2 = (instr >> 7) & 1;
                    let Rm = instr & 7;
                    match (o1, o2) {
                        (x0, x1) if x0 == 0 && x1 == 0 => {
                            Some(Op::aarch32_REV_A1_A)
                        }
                        (x0, x1) if x0 == 0 && x1 == 1 => {
                            Some(Op::aarch32_REV16_A1_A)
                        }
                        (x0, x1) if x0 == 1 && x1 == 0 => {
                            Some(Op::aarch32_RBIT_A1_A)
                        }
                        (x0, x1) if x0 == 1 && x1 == 1 => {
                            Some(Op::aarch32_REVSH_A1_A)
                        }
                        _ => None
                    }
                }
                (_, _, x2, _, x4, _, _) if x2 & 26 == 10 && x4 & 1 == 0 => {
                    let cond = (instr >> 28) & 7;
                    let U = (instr >> 22) & 1;
                    let sat_imm = (instr >> 16) & 9;
                    let Rd = (instr >> 12) & 7;
                    let imm5 = (instr >> 7) & 9;
                    let sh = (instr >> 6) & 1;
                    let Rn = instr & 7;
                    match U {
                        x0 if x0 == 0 => {
                            Some(Op::aarch32_SSAT_A1_A)
                        }
                        x0 if x0 == 1 => {
                            Some(Op::aarch32_USAT_A1_A)
                        }
                        _ => None
                    }
                }
                (_, _, x2, _, x4, _, _) if x2 & 24 == 8 && x4 == 7 => {
                    None
                }
                (_, _, x2, _, x4, _, _) if x2 & 24 == 8 && x4 == 3 => {
                    let cond = (instr >> 28) & 7;
                    let U = (instr >> 22) & 1;
                    let op = (instr >> 20) & 3;
                    let Rn = (instr >> 16) & 7;
                    let Rd = (instr >> 12) & 7;
                    let rotate = (instr >> 10) & 3;
                    let Rm = instr & 7;
                    match (U, op, Rn) {
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 != 15 => {
                            Some(Op::aarch32_SXTAB16_A1_A)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 15 => {
                            Some(Op::aarch32_SXTB16_A1_A)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 != 15 => {
                            Some(Op::aarch32_SXTAB_A1_A)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 15 => {
                            Some(Op::aarch32_SXTB_A1_A)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 != 15 => {
                            Some(Op::aarch32_SXTAH_A1_A)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 15 => {
                            Some(Op::aarch32_SXTH_A1_A)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 != 15 => {
                            Some(Op::aarch32_UXTAB16_A1_A)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 15 => {
                            Some(Op::aarch32_UXTB16_A1_A)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 != 15 => {
                            Some(Op::aarch32_UXTAB_A1_A)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 == 15 => {
                            Some(Op::aarch32_UXTB_A1_A)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 != 15 => {
                            Some(Op::aarch32_UXTAH_A1_A)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 15 => {
                            Some(Op::aarch32_UXTH_A1_A)
                        }
                        _ => None
                    }
                }
                (_, _, x2, _, _, _, _) if x2 & 24 == 16 => {
                    let cond = (instr >> 28) & 7;
                    let op1 = (instr >> 20) & 5;
                    let Rd = (instr >> 16) & 7;
                    let Ra = (instr >> 12) & 7;
                    let Rm = (instr >> 8) & 7;
                    let op2 = (instr >> 5) & 5;
                    let Rn = instr & 7;
                    match (op1, Ra, op2) {
                        (x0, x1, x2) if x0 == 0 && x1 != 15 && x2 == 0 => {
                            Some(Op::aarch32_SMLAD_A1_A)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 15 && x2 == 1 => {
                            Some(Op::aarch32_SMLAD_A1_A)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 15 && x2 == 2 => {
                            Some(Op::aarch32_SMLSD_A1_A)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 15 && x2 == 3 => {
                            Some(Op::aarch32_SMLSD_A1_A)
                        }
                        (x0, _, x2) if x0 == 0 && x2 & 4 == 4 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 15 && x2 == 0 => {
                            Some(Op::aarch32_SMUAD_A1_A)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 15 && x2 == 1 => {
                            Some(Op::aarch32_SMUAD_A1_A)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 15 && x2 == 2 => {
                            Some(Op::aarch32_SMUSD_A1_A)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 15 && x2 == 3 => {
                            Some(Op::aarch32_SMUSD_A1_A)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 0 => {
                            Some(Op::aarch32_SDIV_A1_A)
                        }
                        (x0, _, x2) if x0 == 1 && x2 != 0 => {
                            None
                        }
                        (x0, _, _) if x0 == 2 => {
                            None
                        }
                        (x0, _, x2) if x0 == 3 && x2 == 0 => {
                            Some(Op::aarch32_UDIV_A1_A)
                        }
                        (x0, _, x2) if x0 == 3 && x2 != 0 => {
                            None
                        }
                        (x0, _, x2) if x0 == 4 && x2 == 0 => {
                            Some(Op::aarch32_SMLALD_A1_A)
                        }
                        (x0, _, x2) if x0 == 4 && x2 == 1 => {
                            Some(Op::aarch32_SMLALD_A1_A)
                        }
                        (x0, _, x2) if x0 == 4 && x2 == 2 => {
                            Some(Op::aarch32_SMLSLD_A1_A)
                        }
                        (x0, _, x2) if x0 == 4 && x2 == 3 => {
                            Some(Op::aarch32_SMLSLD_A1_A)
                        }
                        (x0, _, x2) if x0 == 4 && x2 & 4 == 4 => {
                            None
                        }
                        (x0, x1, x2) if x0 == 5 && x1 != 15 && x2 == 0 => {
                            Some(Op::aarch32_SMMLA_A1_A)
                        }
                        (x0, x1, x2) if x0 == 5 && x1 != 15 && x2 == 1 => {
                            Some(Op::aarch32_SMMLA_A1_A)
                        }
                        (x0, _, x2) if x0 == 5 && x2 & 6 == 2 => {
                            None
                        }
                        (x0, _, x2) if x0 == 5 && x2 & 6 == 4 => {
                            None
                        }
                        (x0, _, x2) if x0 == 5 && x2 == 6 => {
                            Some(Op::aarch32_SMMLS_A1_A)
                        }
                        (x0, _, x2) if x0 == 5 && x2 == 7 => {
                            Some(Op::aarch32_SMMLS_A1_A)
                        }
                        (x0, x1, x2) if x0 == 5 && x1 == 15 && x2 == 0 => {
                            Some(Op::aarch32_SMMUL_A1_A)
                        }
                        (x0, x1, x2) if x0 == 5 && x1 == 15 && x2 == 1 => {
                            Some(Op::aarch32_SMMUL_A1_A)
                        }
                        (x0, _, _) if x0 & 6 == 6 => {
                            None
                        }
                        _ => None
                    }
                }
                (_, _, x2, _, x4, _, _) if x2 == 24 && x4 == 0 => {
                    let cond = (instr >> 28) & 7;
                    let Rd = (instr >> 16) & 7;
                    let Ra = (instr >> 12) & 7;
                    let Rm = (instr >> 8) & 7;
                    let Rn = instr & 7;
                    match Ra {
                        x0 if x0 != 15 => {
                            Some(Op::aarch32_USADA8_A1_A)
                        }
                        x0 if x0 == 15 => {
                            Some(Op::aarch32_USAD8_A1_A)
                        }
                        _ => None
                    }
                }
                (_, _, x2, _, x4, _, _) if x2 == 24 && x4 == 4 => {
                    None
                }
                (_, _, x2, _, x4, _, _) if x2 == 25 && x4 & 3 == 0 => {
                    None
                }
                (_, _, x2, _, x4, _, _) if x2 & 30 == 26 && x4 & 3 == 0 => {
                    None
                }
                (_, _, x2, _, x4, _, _) if x2 & 28 == 24 && x4 == 7 => {
                    None
                }
                (_, _, x2, _, x4, _, _) if x2 & 30 == 28 && x4 == 7 => {
                    None
                }
                (_, _, x2, _, x4, _, _) if x2 & 30 == 28 && x4 & 3 == 0 => {
                    let cond = (instr >> 28) & 7;
                    let msb = (instr >> 16) & 9;
                    let Rd = (instr >> 12) & 7;
                    let lsb = (instr >> 7) & 9;
                    let Rn = instr & 7;
                    match Rn {
                        x0 if x0 != 15 => {
                            Some(Op::aarch32_BFI_A1_A)
                        }
                        x0 if x0 == 15 => {
                            Some(Op::aarch32_BFC_A1_A)
                        }
                        _ => None
                    }
                }
                (_, _, x2, _, x4, _, _) if x2 == 30 && x4 == 7 => {
                    None
                }
                (_, _, x2, _, x4, _, _) if x2 == 31 && x4 == 7 => {
                    let cond = (instr >> 28) & 7;
                    let imm12 = (instr >> 8) & 23;
                    let imm4 = instr & 7;
                    match cond {
                        x0 if x0 & 8 == 0 => {
                            None
                        }
                        x0 if x0 & 12 == 8 => {
                            None
                        }
                        x0 if x0 & 14 == 12 => {
                            None
                        }
                        x0 if x0 == 14 => {
                            Some(Op::aarch32_UDF_A1_A)
                        }
                        _ => None
                    }
                }
                (_, _, x2, _, x4, _, _) if x2 & 30 == 30 && x4 & 3 == 0 => {
                    None
                }
                (_, _, x2, _, x4, _, _) if x2 & 26 == 24 && x4 & 3 == 2 => {
                    None
                }
                (_, _, x2, _, x4, _, _) if x2 & 26 == 26 && x4 & 3 == 2 => {
                    let cond = (instr >> 28) & 7;
                    let U = (instr >> 22) & 1;
                    let widthm1 = (instr >> 16) & 9;
                    let Rd = (instr >> 12) & 7;
                    let lsb = (instr >> 7) & 9;
                    let Rn = instr & 7;
                    match U {
                        x0 if x0 == 0 => {
                            Some(Op::aarch32_SBFX_A1_A)
                        }
                        x0 if x0 == 1 => {
                            Some(Op::aarch32_UBFX_A1_A)
                        }
                        _ => None
                    }
                }
                (_, _, x2, _, x4, _, _) if x2 & 24 == 24 && x4 == 3 => {
                    None
                }
                (_, _, x2, _, x4, _, _) if x2 & 24 == 24 && x4 & 3 == 1 => {
                    None
                }
                _ => None
            }
        }
        (_, x1, _, _, _) if x1 & 6 == 4 => {
            match ((instr >> 28) & 7, (instr >> 26) & 3, (instr >> 25) & 1, instr & 49) {
                (x0, _, x2, _) if x0 == 15 && x2 == 0 => {
                    let P = (instr >> 24) & 1;
                    let U = (instr >> 23) & 1;
                    let S = (instr >> 22) & 1;
                    let W = (instr >> 21) & 1;
                    let L = (instr >> 20) & 1;
                    let Rn = (instr >> 16) & 7;
                    let op = (instr >> 5) & 21;
                    let mode = instr & 9;
                    match (P, U, S, L) {
                        (_, _, x2, x3) if x2 == 0 && x3 == 0 => {
                            None
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            Some(Op::aarch32_RFE_A1_AS)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            Some(Op::aarch32_SRS_A1_AS)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 1 => {
                            Some(Op::aarch32_RFE_A1_AS)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 0 => {
                            Some(Op::aarch32_SRS_A1_AS)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            Some(Op::aarch32_RFE_A1_AS)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            Some(Op::aarch32_SRS_A1_AS)
                        }
                        (_, _, x2, x3) if x2 == 1 && x3 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 1 => {
                            Some(Op::aarch32_RFE_A1_AS)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 0 => {
                            Some(Op::aarch32_SRS_A1_AS)
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _) if x0 != 15 && x2 == 0 => {
                    let cond = (instr >> 28) & 7;
                    let P = (instr >> 24) & 1;
                    let U = (instr >> 23) & 1;
                    let op = (instr >> 22) & 1;
                    let W = (instr >> 21) & 1;
                    let L = (instr >> 20) & 1;
                    let Rn = (instr >> 16) & 7;
                    let register_list = instr & 31;
                    match (P, U, op, L, register_list) {
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            Some(Op::aarch32_STMDA_A1_A)
                        }
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            Some(Op::aarch32_LDMDA_A1_A)
                        }
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 0 => {
                            Some(Op::aarch32_STM_A1_A)
                        }
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 1 => {
                            Some(Op::aarch32_LDM_A1_A)
                        }
                        (_, _, x2, x3, _) if x2 == 1 && x3 == 0 => {
                            Some(Op::aarch32_STM_u_A1_AS)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            Some(Op::aarch32_STMDB_A1_A)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            Some(Op::aarch32_LDMDB_A1_A)
                        }
                        (_, _, x2, x3, x4) if x2 == 1 && x3 == 1 && x4 & 32768 == 0 => {
                            Some(Op::aarch32_LDM_u_A1_AS)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 0 => {
                            Some(Op::aarch32_STMIB_A1_A)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 1 => {
                            Some(Op::aarch32_LDMIB_A1_A)
                        }
                        (_, _, x2, x3, x4) if x2 == 1 && x3 == 1 && x4 & 32768 == 32768 => {
                            Some(Op::aarch32_LDM_e_A1_AS)
                        }
                        _ => None
                    }
                }
                (_, _, x2, _) if x2 == 1 => {
                    let cond = (instr >> 28) & 7;
                    let H = (instr >> 24) & 1;
                    let imm24 = instr & 47;
                    match (cond, H) {
                        (x0, x1) if x0 != 15 && x1 == 0 => {
                            Some(Op::aarch32_B_A1_A)
                        }
                        (x0, x1) if x0 != 15 && x1 == 1 => {
                            Some(Op::aarch32_BL_i_A1_A)
                        }
                        (x0, _) if x0 == 15 => {
                            Some(Op::aarch32_BL_i_A1_A)
                        }
                        _ => None
                    }
                }
                _ => None
            }
        }
        (_, x1, _, _, _) if x1 & 6 == 6 => {
            match ((instr >> 28) & 7, (instr >> 26) & 3, (instr >> 24) & 3, (instr >> 12) & 23, (instr >> 9) & 5, (instr >> 5) & 7, (instr >> 4) & 1, instr & 7) {
                (_, _, x2, _, x4, _, _, _) if x2 & 2 == 0 && x4 == 7 => {
                    match ((instr >> 28) & 7, (instr >> 25) & 5, (instr >> 21) & 7, (instr >> 12) & 17, (instr >> 9) & 5, instr & 17) {
                        (_, _, x2, _, _, _) if x2 & 13 == 0 => {
                            let cond = (instr >> 28) & 7;
                            let D = (instr >> 22) & 1;
                            let L = (instr >> 20) & 1;
                            let Rt2 = (instr >> 16) & 7;
                            let Rt = (instr >> 12) & 7;
                            let cp15 = (instr >> 8) & 1;
                            let opc1 = (instr >> 4) & 7;
                            let CRm = instr & 7;
                            match (cond, D, L) {
                                (x0, x1, x2) if x0 != 15 && x1 == 1 && x2 == 0 => {
                                    Some(Op::aarch32_MCRR_T1A1_A)
                                }
                                (x0, x1, x2) if x0 != 15 && x1 == 1 && x2 == 1 => {
                                    Some(Op::aarch32_MRRC_T1A1_A)
                                }
                                (_, x1, _) if x1 == 0 => {
                                    None
                                }
                                (x0, x1, _) if x0 == 15 && x1 == 1 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        (_, _, x2, _, _, _) if x2 & 13 != 0 => {
                            let cond = (instr >> 28) & 7;
                            let P = (instr >> 24) & 1;
                            let U = (instr >> 23) & 1;
                            let D = (instr >> 22) & 1;
                            let W = (instr >> 21) & 1;
                            let L = (instr >> 20) & 1;
                            let Rn = (instr >> 16) & 7;
                            let CRd = (instr >> 12) & 7;
                            let cp15 = (instr >> 8) & 1;
                            let imm8 = instr & 15;
                            match (cond, (P << 1) | U, D, L, Rn, CRd, cp15) {
                                (x0, x1, x2, _, _, x5, x6) if x0 != 15 && x1 != 0 && x2 == 0 && x5 != 5 && x6 == 0 => {
                                    None
                                }
                                (x0, x1, x2, x3, x4, x5, x6) if x0 != 15 && x1 != 0 && x2 == 0 && x3 == 1 && x4 == 15 && x5 == 5 && x6 == 0 => {
                                    Some(Op::aarch32_LDC_l_T1A1_A)
                                }
                                (x0, x1, _, _, _, _, x6) if x0 != 15 && x1 != 0 && x6 == 1 => {
                                    None
                                }
                                (x0, x1, x2, _, _, x5, x6) if x0 != 15 && x1 != 0 && x2 == 1 && x5 == 5 && x6 == 0 => {
                                    None
                                }
                                (x0, x1, x2, x3, _, x5, x6) if x0 != 15 && x1 & 5 == 1 && x2 == 0 && x3 == 0 && x5 == 5 && x6 == 0 => {
                                    Some(Op::aarch32_STC_T1A1_A)
                                }
                                (x0, x1, x2, x3, x4, x5, x6) if x0 != 15 && x1 & 5 == 1 && x2 == 0 && x3 == 1 && x4 != 15 && x5 == 5 && x6 == 0 => {
                                    Some(Op::aarch32_LDC_i_T1A1_A)
                                }
                                (x0, x1, x2, x3, _, x5, x6) if x0 != 15 && x1 == 2 && x2 == 0 && x3 == 0 && x5 == 5 && x6 == 0 => {
                                    Some(Op::aarch32_STC_T1A1_A)
                                }
                                (x0, x1, x2, x3, x4, x5, x6) if x0 != 15 && x1 == 2 && x2 == 0 && x3 == 1 && x4 != 15 && x5 == 5 && x6 == 0 => {
                                    Some(Op::aarch32_LDC_i_T1A1_A)
                                }
                                (x0, x1, x2, x3, _, x5, x6) if x0 != 15 && x1 & 5 == 4 && x2 == 0 && x3 == 0 && x5 == 5 && x6 == 0 => {
                                    Some(Op::aarch32_STC_T1A1_A)
                                }
                                (x0, x1, x2, x3, x4, x5, x6) if x0 != 15 && x1 & 5 == 4 && x2 == 0 && x3 == 1 && x4 != 15 && x5 == 5 && x6 == 0 => {
                                    Some(Op::aarch32_LDC_i_T1A1_A)
                                }
                                (x0, x1, x2, x3, _, x5, x6) if x0 != 15 && x1 & 5 == 5 && x2 == 0 && x3 == 0 && x5 == 5 && x6 == 0 => {
                                    Some(Op::aarch32_STC_T1A1_A)
                                }
                                (x0, x1, x2, x3, x4, x5, x6) if x0 != 15 && x1 & 5 == 5 && x2 == 0 && x3 == 1 && x4 != 15 && x5 == 5 && x6 == 0 => {
                                    Some(Op::aarch32_LDC_i_T1A1_A)
                                }
                                (x0, x1, _, _, _, _, _) if x0 == 15 && x1 != 0 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        _ => None
                    }
                }
                (_, _, x2, _, x4, _, x6, _) if x2 == 2 && x4 & 6 == 4 && x6 == 0 => {
                    match ((instr >> 28) & 7, (instr >> 24) & 7, (instr >> 20) & 7, (instr >> 16) & 7, (instr >> 12) & 7, (instr >> 10) & 3, (instr >> 8) & 3, (instr >> 7) & 1, (instr >> 6) & 1, (instr >> 5) & 1, (instr >> 4) & 1, instr & 7) {
                        (x0, _, x2, _, _, _, x6, _, x8, _, _, _) if x0 == 15 && x2 & 8 == 0 && x6 != 0 && x8 == 0 => {
                            let D = (instr >> 22) & 1;
                            let cc = (instr >> 20) & 3;
                            let Vn = (instr >> 16) & 7;
                            let Vd = (instr >> 12) & 7;
                            let size = (instr >> 8) & 3;
                            let N = (instr >> 7) & 1;
                            let M = (instr >> 5) & 1;
                            let Vm = instr & 7;
                            match (cc, size) {
                                (x0, _) if x0 == 0 => {
                                    Some(Op::aarch32_VSEL_A1_A)
                                }
                                (x0, _) if x0 == 1 => {
                                    Some(Op::aarch32_VSEL_A1_A)
                                }
                                (_, x1) if x1 == 1 => {
                                    None
                                }
                                (x0, _) if x0 == 2 => {
                                    Some(Op::aarch32_VSEL_A1_A)
                                }
                                (x0, _) if x0 == 3 => {
                                    Some(Op::aarch32_VSEL_A1_A)
                                }
                                _ => None
                            }
                        }
                        (x0, _, x2, _, _, _, x6, _, _, _, _, _) if x0 == 15 && x2 & 11 == 8 && x6 != 0 => {
                            let D = (instr >> 22) & 1;
                            let Vn = (instr >> 16) & 7;
                            let Vd = (instr >> 12) & 7;
                            let size = (instr >> 8) & 3;
                            let N = (instr >> 7) & 1;
                            let op = (instr >> 6) & 1;
                            let M = (instr >> 5) & 1;
                            let Vm = instr & 7;
                            match (size, op) {
                                (_, x1) if x1 == 0 => {
                                    Some(Op::aarch32_VMAXNM_A2_A)
                                }
                                (x0, _) if x0 == 1 => {
                                    None
                                }
                                (_, x1) if x1 == 1 => {
                                    Some(Op::aarch32_VMAXNM_A2_A)
                                }
                                _ => None
                            }
                        }
                        (x0, _, x2, x3, _, _, x6, _, x8, _, _, _) if x0 == 15 && x2 & 11 == 11 && x3 == 0 && x6 != 0 && x8 == 1 => {
                            let D = (instr >> 22) & 1;
                            let Vd = (instr >> 12) & 7;
                            let size = (instr >> 8) & 3;
                            let op = (instr >> 7) & 1;
                            let M = (instr >> 5) & 1;
                            let Vm = instr & 7;
                            match (size, op) {
                                (x0, _) if x0 == 1 => {
                                    None
                                }
                                (x0, x1) if x0 == 2 && x1 == 0 => {
                                    Some(Op::aarch32_VMOVX_A1_A)
                                }
                                (x0, x1) if x0 == 2 && x1 == 1 => {
                                    Some(Op::aarch32_VINS_A1_A)
                                }
                                (x0, _) if x0 == 3 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        (x0, _, x2, x3, _, _, x6, _, x8, _, _, _) if x0 == 15 && x2 & 11 == 11 && x3 & 8 == 8 && x6 != 0 && x8 == 1 => {
                            let D = (instr >> 22) & 1;
                            let o1 = (instr >> 18) & 1;
                            let RM = (instr >> 16) & 3;
                            let Vd = (instr >> 12) & 7;
                            let size = (instr >> 8) & 3;
                            let op = (instr >> 7) & 1;
                            let M = (instr >> 5) & 1;
                            let Vm = instr & 7;
                            match (o1, RM, size) {
                                (x0, x1, _) if x0 == 0 && x1 == 0 => {
                                    Some(Op::aarch32_VRINTA_vfp_A1_A)
                                }
                                (x0, x1, _) if x0 == 0 && x1 == 1 => {
                                    Some(Op::aarch32_VRINTA_vfp_A1_A)
                                }
                                (_, _, x2) if x2 == 1 => {
                                    None
                                }
                                (x0, x1, _) if x0 == 0 && x1 == 2 => {
                                    Some(Op::aarch32_VRINTA_vfp_A1_A)
                                }
                                (x0, x1, _) if x0 == 0 && x1 == 3 => {
                                    Some(Op::aarch32_VRINTA_vfp_A1_A)
                                }
                                (x0, x1, _) if x0 == 1 && x1 == 0 => {
                                    Some(Op::aarch32_VCVTA_vfp_A1_A)
                                }
                                (x0, x1, _) if x0 == 1 && x1 == 1 => {
                                    Some(Op::aarch32_VCVTA_vfp_A1_A)
                                }
                                (x0, x1, _) if x0 == 1 && x1 == 2 => {
                                    Some(Op::aarch32_VCVTA_vfp_A1_A)
                                }
                                (x0, x1, _) if x0 == 1 && x1 == 3 => {
                                    Some(Op::aarch32_VCVTA_vfp_A1_A)
                                }
                                _ => None
                            }
                        }
                        (x0, _, x2, _, _, _, _, _, x8, _, _, _) if x0 != 15 && x2 & 11 == 11 && x8 == 1 => {
                            let cond = (instr >> 28) & 7;
                            let D = (instr >> 22) & 1;
                            let o1 = (instr >> 19) & 1;
                            let opc2 = (instr >> 16) & 5;
                            let Vd = (instr >> 12) & 7;
                            let size = (instr >> 8) & 3;
                            let o3 = (instr >> 7) & 1;
                            let M = (instr >> 5) & 1;
                            let Vm = instr & 7;
                            match (o1, opc2, size, o3) {
                                (_, _, x2, _) if x2 == 0 => {
                                    None
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => {
                                    None
                                }
                                (x0, x1, _, x3) if x0 == 0 && x1 == 0 && x3 == 1 => {
                                    Some(Op::aarch32_VABS_A2_A)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 2 && x3 == 0 => {
                                    Some(Op::aarch32_VMOV_r_T2A2_A)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 => {
                                    Some(Op::aarch32_VMOV_r_T2A2_A)
                                }
                                (x0, x1, _, x3) if x0 == 0 && x1 == 1 && x3 == 0 => {
                                    Some(Op::aarch32_VNEG_A2_A)
                                }
                                (x0, x1, _, x3) if x0 == 0 && x1 == 1 && x3 == 1 => {
                                    Some(Op::aarch32_VSQRT_A1_A)
                                }
                                (x0, x1, _, x3) if x0 == 0 && x1 == 2 && x3 == 0 => {
                                    Some(Op::aarch32_VCVTB_T1A1_A)
                                }
                                (x0, x1, x2, _) if x0 == 0 && x1 == 2 && x2 == 1 => {
                                    None
                                }
                                (x0, x1, _, x3) if x0 == 0 && x1 == 2 && x3 == 1 => {
                                    Some(Op::aarch32_VCVTB_T1A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 3 && x2 == 1 && x3 == 0 => {
                                    Some(Op::aarch32_VCVTB_bf16_T1A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 3 && x2 == 1 && x3 == 1 => {
                                    Some(Op::aarch32_VCVTT_T1A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 3 && x2 == 2 && x3 == 0 => {
                                    Some(Op::aarch32_VCVTB_T1A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 3 && x2 == 2 && x3 == 1 => {
                                    Some(Op::aarch32_VCVTB_T1A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 3 && x2 == 3 && x3 == 0 => {
                                    Some(Op::aarch32_VCVTB_T1A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 3 && x2 == 3 && x3 == 1 => {
                                    Some(Op::aarch32_VCVTB_T1A1_A)
                                }
                                (x0, x1, _, x3) if x0 == 0 && x1 == 4 && x3 == 0 => {
                                    Some(Op::aarch32_VCMP_A1_A)
                                }
                                (x0, x1, _, x3) if x0 == 0 && x1 == 4 && x3 == 1 => {
                                    Some(Op::aarch32_VCMP_A1_A)
                                }
                                (x0, x1, _, x3) if x0 == 0 && x1 == 5 && x3 == 0 => {
                                    Some(Op::aarch32_VCMP_A1_A)
                                }
                                (x0, x1, _, x3) if x0 == 0 && x1 == 5 && x3 == 1 => {
                                    Some(Op::aarch32_VCMP_A1_A)
                                }
                                (x0, x1, _, x3) if x0 == 0 && x1 == 6 && x3 == 0 => {
                                    Some(Op::aarch32_VRINTZ_vfp_A1_A)
                                }
                                (x0, x1, _, x3) if x0 == 0 && x1 == 6 && x3 == 1 => {
                                    Some(Op::aarch32_VRINTZ_vfp_A1_A)
                                }
                                (x0, x1, _, x3) if x0 == 0 && x1 == 7 && x3 == 0 => {
                                    Some(Op::aarch32_VRINTX_vfp_A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 7 && x2 == 1 && x3 == 1 => {
                                    None
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 7 && x2 == 2 && x3 == 1 => {
                                    Some(Op::aarch32_VCVT_ds_T1A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 7 && x2 == 3 && x3 == 1 => {
                                    Some(Op::aarch32_VCVT_ds_T1A1_A)
                                }
                                (x0, x1, _, _) if x0 == 1 && x1 == 0 => {
                                    Some(Op::aarch32_VCVT_iv_A1_A)
                                }
                                (x0, x1, x2, _) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    None
                                }
                                (x0, x1, x2, _) if x0 == 1 && x1 == 1 && x2 == 2 => {
                                    None
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 3 && x3 == 0 => {
                                    None
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 3 && x3 == 1 => {
                                    Some(Op::aarch32_VJCVT_A1_A)
                                }
                                (x0, x1, _, _) if x0 == 1 && x1 & 6 == 2 => {
                                    Some(Op::aarch32_VCVT_xv_A1_A)
                                }
                                (x0, x1, _, x3) if x0 == 1 && x1 == 4 && x3 == 0 => {
                                    Some(Op::aarch32_VCVT_iv_A1_A)
                                }
                                (x0, x1, _, x3) if x0 == 1 && x1 == 4 && x3 == 1 => {
                                    Some(Op::aarch32_VCVT_iv_A1_A)
                                }
                                (x0, x1, _, x3) if x0 == 1 && x1 == 5 && x3 == 0 => {
                                    Some(Op::aarch32_VCVT_iv_A1_A)
                                }
                                (x0, x1, _, x3) if x0 == 1 && x1 == 5 && x3 == 1 => {
                                    Some(Op::aarch32_VCVT_iv_A1_A)
                                }
                                (x0, x1, _, _) if x0 == 1 && x1 & 6 == 6 => {
                                    Some(Op::aarch32_VCVT_xv_A1_A)
                                }
                                _ => None
                            }
                        }
                        (x0, _, x2, _, _, _, _, _, x8, _, _, _) if x0 != 15 && x2 & 11 == 11 && x8 == 0 => {
                            let cond = (instr >> 28) & 7;
                            let D = (instr >> 22) & 1;
                            let imm4H = (instr >> 16) & 7;
                            let Vd = (instr >> 12) & 7;
                            let size = (instr >> 8) & 3;
                            let imm4L = instr & 7;
                            match size {
                                x0 if x0 == 0 => {
                                    None
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::aarch32_VMOV_i_A2_A)
                                }
                                x0 if x0 == 2 => {
                                    Some(Op::aarch32_VMOV_i_A2_A)
                                }
                                x0 if x0 == 3 => {
                                    Some(Op::aarch32_VMOV_i_A2_A)
                                }
                                _ => None
                            }
                        }
                        (x0, _, x2, _, _, _, _, _, _, _, _, _) if x0 != 15 && x2 & 11 != 11 => {
                            let cond = (instr >> 28) & 7;
                            let o0 = (instr >> 23) & 1;
                            let D = (instr >> 22) & 1;
                            let o1 = (instr >> 20) & 3;
                            let Vn = (instr >> 16) & 7;
                            let Vd = (instr >> 12) & 7;
                            let size = (instr >> 8) & 3;
                            let N = (instr >> 7) & 1;
                            let o2 = (instr >> 6) & 1;
                            let M = (instr >> 5) & 1;
                            let Vm = instr & 7;
                            match ((o0 << 2) | o1, size, o2) {
                                (x0, x1, _) if x0 != 7 && x1 == 0 => {
                                    None
                                }
                                (x0, _, x2) if x0 == 0 && x2 == 0 => {
                                    Some(Op::aarch32_VMLA_f_A2_A)
                                }
                                (x0, _, x2) if x0 == 0 && x2 == 1 => {
                                    Some(Op::aarch32_VMLA_f_A2_A)
                                }
                                (x0, _, x2) if x0 == 1 && x2 == 0 => {
                                    Some(Op::aarch32_VNMLA_A1_A)
                                }
                                (x0, _, x2) if x0 == 1 && x2 == 1 => {
                                    Some(Op::aarch32_VNMLA_A1_A)
                                }
                                (x0, _, x2) if x0 == 2 && x2 == 0 => {
                                    Some(Op::aarch32_VMUL_f_A2_A)
                                }
                                (x0, _, x2) if x0 == 2 && x2 == 1 => {
                                    Some(Op::aarch32_VNMLA_A2_A)
                                }
                                (x0, _, x2) if x0 == 3 && x2 == 0 => {
                                    Some(Op::aarch32_VADD_f_A2_A)
                                }
                                (x0, _, x2) if x0 == 3 && x2 == 1 => {
                                    Some(Op::aarch32_VSUB_f_A2_A)
                                }
                                (x0, _, x2) if x0 == 4 && x2 == 0 => {
                                    Some(Op::aarch32_VDIV_A1_A)
                                }
                                (x0, _, x2) if x0 == 5 && x2 == 0 => {
                                    Some(Op::aarch32_VFNMA_A1_A)
                                }
                                (x0, _, x2) if x0 == 5 && x2 == 1 => {
                                    Some(Op::aarch32_VFNMA_A1_A)
                                }
                                (x0, _, x2) if x0 == 6 && x2 == 0 => {
                                    Some(Op::aarch32_VFMA_A2_A)
                                }
                                (x0, _, x2) if x0 == 6 && x2 == 1 => {
                                    Some(Op::aarch32_VFMA_A2_A)
                                }
                                _ => None
                            }
                        }
                        _ => None
                    }
                }
                (_, _, x2, _, x4, _, x6, _) if x2 == 2 && x4 == 7 && x6 == 1 => {
                    let cond = (instr >> 28) & 7;
                    let opc1 = (instr >> 21) & 5;
                    let L = (instr >> 20) & 1;
                    let CRn = (instr >> 16) & 7;
                    let Rt = (instr >> 12) & 7;
                    let cp15 = (instr >> 8) & 1;
                    let opc2 = (instr >> 5) & 5;
                    let CRm = instr & 7;
                    match (cond, L) {
                        (x0, x1) if x0 != 15 && x1 == 0 => {
                            Some(Op::aarch32_MCR_T1A1_A)
                        }
                        (x0, x1) if x0 != 15 && x1 == 1 => {
                            Some(Op::aarch32_MRC_T1A1_A)
                        }
                        (x0, _) if x0 == 15 => {
                            None
                        }
                        _ => None
                    }
                }
                (_, _, x2, _, _, _, _, _) if x2 == 3 => {
                    match ((instr >> 28) & 7, (instr >> 24) & 7, instr & 47) {
                        (x0, _, _) if x0 == 15 => {
                            None
                        }
                        (x0, _, _) if x0 != 15 => {
                            let cond = (instr >> 28) & 7;
                            let imm24 = instr & 47;
                            match () {
                                () => {
                                    Some(Op::aarch32_SVC_A1_A)
                                }
                            }
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, _, _) if x0 == 15 && x2 & 2 == 0 && x4 & 5 == 4 => {
                    let op1 = (instr >> 23) & 3;
                    let D = (instr >> 22) & 1;
                    let op2 = (instr >> 20) & 3;
                    let Vn = (instr >> 16) & 7;
                    let Vd = (instr >> 12) & 7;
                    let op3 = (instr >> 10) & 1;
                    let op4 = (instr >> 8) & 1;
                    let N = (instr >> 7) & 1;
                    let Q = (instr >> 6) & 1;
                    let M = (instr >> 5) & 1;
                    let U = (instr >> 4) & 1;
                    let Vm = instr & 7;
                    match (op1, op2, op3, op4, Q, U) {
                        (x0, x1, x2, x3, x4, x5) if x0 & 1 == 1 && x1 & 2 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 0 => {
                            Some(Op::aarch32_VCADD_A1_A)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 & 1 == 1 && x1 & 2 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 & 1 == 1 && x1 & 2 == 0 && x2 == 0 && x3 == 0 && x4 == 1 && x5 == 0 => {
                            Some(Op::aarch32_VCADD_A1_A)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 & 1 == 1 && x1 & 2 == 0 && x2 == 0 && x3 == 0 && x4 == 1 && x5 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 & 2 == 0 && x2 == 0 && x3 == 0 => {
                            None
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 & 2 == 0 && x2 == 0 && x3 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 0 => {
                            None
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 0 => {
                            Some(Op::aarch32_VMMLA_A1_A)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 0 => {
                            Some(Op::aarch32_VDOT_bf16_A1_A)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 0 => {
                            Some(Op::aarch32_VDOT_bf16_A1_A)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 0 => {
                            None
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 2 && x2 == 0 && x3 == 0 && x5 == 1 => {
                            Some(Op::aarch32_VFMAL_A1_A)
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 2 && x2 == 0 && x3 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 2 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            None
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 2 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 0 => {
                            Some(Op::aarch32_MMLA_A1_A)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 2 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 1 => {
                            Some(Op::aarch32_MMLA_A1_A)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 2 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 0 => {
                            Some(Op::aarch32_VDOT_A1_A)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 2 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 1 => {
                            Some(Op::aarch32_VDOT_A1_A)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 2 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 0 => {
                            Some(Op::aarch32_VDOT_A1_A)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 2 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 1 => {
                            Some(Op::aarch32_VDOT_A1_A)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 3 && x2 == 0 && x3 == 0 && x5 == 1 => {
                            Some(Op::aarch32_VFMA_bf_A1_A)
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 3 && x2 == 0 && x3 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 3 && x2 == 1 && x3 == 0 => {
                            None
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 3 && x2 == 1 && x3 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 1 && x1 == 2 && x2 == 0 && x3 == 0 && x5 == 1 => {
                            Some(Op::aarch32_VFMAL_A1_A)
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 1 && x1 == 2 && x2 == 0 && x3 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 2 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            None
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 2 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 0 => {
                            Some(Op::aarch32_MMLA_A1_A)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 2 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 2 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 0 => {
                            Some(Op::aarch32_VUSDOT_A1_A)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 1 && x1 == 2 && x2 == 1 && x3 == 1 && x5 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 2 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 0 => {
                            Some(Op::aarch32_VUSDOT_A1_A)
                        }
                        (x0, x1, _, _, _, _) if x0 == 1 && x1 == 3 => {
                            None
                        }
                        (_, x1, x2, x3, _, x5) if x1 & 2 == 2 && x2 == 0 && x3 == 0 && x5 == 0 => {
                            Some(Op::aarch32_VCMLA_A1_A)
                        }
                        (x0, x1, _, _, _, _) if x0 == 2 && x1 == 3 => {
                            None
                        }
                        (x0, x1, _, _, _, _) if x0 == 3 && x1 == 3 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, _, _) if x0 == 15 && x2 == 2 && x4 & 5 == 4 => {
                    let op1 = (instr >> 23) & 1;
                    let D = (instr >> 22) & 1;
                    let op2 = (instr >> 20) & 3;
                    let Vn = (instr >> 16) & 7;
                    let Vd = (instr >> 12) & 7;
                    let op3 = (instr >> 10) & 1;
                    let op4 = (instr >> 8) & 1;
                    let N = (instr >> 7) & 1;
                    let Q = (instr >> 6) & 1;
                    let M = (instr >> 5) & 1;
                    let U = (instr >> 4) & 1;
                    let Vm = instr & 7;
                    match (op1, op2, op3, op4, Q, U) {
                        (x0, _, x2, x3, _, x5) if x0 == 0 && x2 == 0 && x3 == 0 && x5 == 0 => {
                            Some(Op::aarch32_VCMLA_idx_A1_A)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x5 == 1 => {
                            Some(Op::aarch32_VFMAL_i_A1_A)
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            None
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 0 => {
                            Some(Op::aarch32_VDOT_bf16_i_A1_A)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x5 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 0 => {
                            Some(Op::aarch32_VDOT_bf16_i_A1_A)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 0 && x5 == 0 => {
                            None
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 1 => {
                            Some(Op::aarch32_VFMAL_i_A1_A)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 0 && x4 == 1 && x5 == 1 => {
                            Some(Op::aarch32_VFMAL_i_A1_A)
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 0 => {
                            None
                        }
                        (x0, x1, x2, _, _, _) if x0 == 0 && x1 == 2 && x2 == 0 => {
                            None
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 2 && x2 == 1 && x3 == 0 => {
                            None
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 2 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 0 => {
                            Some(Op::aarch32_VDOT_s_A1_A)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 2 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 1 => {
                            Some(Op::aarch32_VDOT_s_A1_A)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 2 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 0 => {
                            Some(Op::aarch32_VDOT_s_A1_A)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 2 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 1 => {
                            Some(Op::aarch32_VDOT_s_A1_A)
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 3 && x2 == 0 && x3 == 0 && x5 == 0 => {
                            None
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 3 && x2 == 0 && x3 == 0 && x5 == 1 => {
                            Some(Op::aarch32_VFMA_bfs_A1_A)
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 3 && x2 == 0 && x3 == 1 => {
                            None
                        }
                        (x0, x1, x2, _, _, _) if x0 == 0 && x1 == 3 && x2 == 1 => {
                            None
                        }
                        (x0, _, x2, x3, _, x5) if x0 == 1 && x2 == 0 && x3 == 0 && x5 == 0 => {
                            Some(Op::aarch32_VCMLA_idx_A1_A)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 0 => {
                            Some(Op::aarch32_DOT_A1_A)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 1 => {
                            Some(Op::aarch32_DOT_A1_A)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 0 => {
                            Some(Op::aarch32_DOT_A1_A)
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 1 => {
                            Some(Op::aarch32_DOT_A1_A)
                        }
                        (x0, _, x2, x3, _, _) if x0 == 1 && x2 == 0 && x3 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 1 && x1 & 2 == 2 && x2 == 1 && x3 == 1 => {
                            None
                        }
                        (x0, _, x2, x3, _, _) if x0 == 1 && x2 == 1 && x3 == 0 => {
                            None
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, _, _) if x0 != 15 && x2 & 2 == 0 && x4 & 6 == 4 => {
                    match ((instr >> 28) & 7, (instr >> 25) & 5, (instr >> 21) & 7, (instr >> 12) & 17, (instr >> 10) & 3, instr & 19) {
                        (_, _, x2, _, _, _) if x2 & 13 == 0 => {
                            let cond = (instr >> 28) & 7;
                            let D = (instr >> 22) & 1;
                            let op = (instr >> 20) & 1;
                            let Rt2 = (instr >> 16) & 7;
                            let Rt = (instr >> 12) & 7;
                            let size = (instr >> 8) & 3;
                            let opc2 = (instr >> 6) & 3;
                            let M = (instr >> 5) & 1;
                            let o3 = (instr >> 4) & 1;
                            let Vm = instr & 7;
                            match (D, op, size, opc2, o3) {
                                (x0, _, _, _, _) if x0 == 0 => {
                                    None
                                }
                                (x0, _, _, _, x4) if x0 == 1 && x4 == 0 => {
                                    None
                                }
                                (x0, _, x2, x3, x4) if x0 == 1 && x2 & 2 == 0 && x3 == 0 && x4 == 1 => {
                                    None
                                }
                                (x0, _, _, x3, _) if x0 == 1 && x3 == 1 => {
                                    None
                                }
                                (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 2 && x3 == 0 && x4 == 1 => {
                                    Some(Op::aarch32_VMOV_ss_T1A1_A)
                                }
                                (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 1 => {
                                    Some(Op::aarch32_VMOV_d_T1A1_A)
                                }
                                (x0, _, _, x3, _) if x0 == 1 && x3 & 2 == 2 => {
                                    None
                                }
                                (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 1 && x2 == 2 && x3 == 0 && x4 == 1 => {
                                    Some(Op::aarch32_VMOV_ss_T1A1_A)
                                }
                                (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 1 && x2 == 3 && x3 == 0 && x4 == 1 => {
                                    Some(Op::aarch32_VMOV_d_T1A1_A)
                                }
                                _ => None
                            }
                        }
                        (_, _, x2, _, _, _) if x2 & 13 != 0 => {
                            let cond = (instr >> 28) & 7;
                            let P = (instr >> 24) & 1;
                            let U = (instr >> 23) & 1;
                            let D = (instr >> 22) & 1;
                            let W = (instr >> 21) & 1;
                            let L = (instr >> 20) & 1;
                            let Rn = (instr >> 16) & 7;
                            let Vd = (instr >> 12) & 7;
                            let size = (instr >> 8) & 3;
                            let imm8 = instr & 15;
                            match (P, U, W, L, Rn, size, imm8) {
                                (x0, x1, x2, _, _, _, _) if x0 == 0 && x1 == 0 && x2 == 1 => {
                                    None
                                }
                                (x0, x1, _, _, _, x5, _) if x0 == 0 && x1 == 1 && x5 & 2 == 0 => {
                                    None
                                }
                                (x0, x1, _, x3, _, x5, _) if x0 == 0 && x1 == 1 && x3 == 0 && x5 == 2 => {
                                    Some(Op::aarch32_VSTM_T2A2_A)
                                }
                                (x0, x1, _, x3, _, x5, x6) if x0 == 0 && x1 == 1 && x3 == 0 && x5 == 3 && x6 & 1 == 0 => {
                                    Some(Op::aarch32_VSTM_T1A1_A)
                                }
                                (x0, x1, _, x3, _, x5, x6) if x0 == 0 && x1 == 1 && x3 == 0 && x5 == 3 && x6 & 1 == 1 => {
                                    Some(Op::aarch32_VSTM_T1A1_A)
                                }
                                (x0, x1, _, x3, _, x5, _) if x0 == 0 && x1 == 1 && x3 == 1 && x5 == 2 => {
                                    Some(Op::aarch32_VLDM_T2A2_A)
                                }
                                (x0, x1, _, x3, _, x5, x6) if x0 == 0 && x1 == 1 && x3 == 1 && x5 == 3 && x6 & 1 == 0 => {
                                    Some(Op::aarch32_VLDM_T1A1_A)
                                }
                                (x0, x1, _, x3, _, x5, x6) if x0 == 0 && x1 == 1 && x3 == 1 && x5 == 3 && x6 & 1 == 1 => {
                                    Some(Op::aarch32_VLDM_T1A1_A)
                                }
                                (x0, _, x2, x3, _, _, _) if x0 == 1 && x2 == 0 && x3 == 0 => {
                                    Some(Op::aarch32_VSTR_A1_A)
                                }
                                (x0, _, x2, _, _, x5, _) if x0 == 1 && x2 == 0 && x5 == 0 => {
                                    None
                                }
                                (x0, _, x2, x3, x4, _, _) if x0 == 1 && x2 == 0 && x3 == 1 && x4 != 15 => {
                                    Some(Op::aarch32_VLDR_A1_A)
                                }
                                (x0, x1, x2, _, _, x5, _) if x0 == 1 && x1 == 0 && x2 == 1 && x5 & 2 == 0 => {
                                    None
                                }
                                (x0, x1, x2, x3, _, x5, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x5 == 2 => {
                                    Some(Op::aarch32_VSTM_T2A2_A)
                                }
                                (x0, x1, x2, x3, _, x5, x6) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x5 == 3 && x6 & 1 == 0 => {
                                    Some(Op::aarch32_VSTM_T1A1_A)
                                }
                                (x0, x1, x2, x3, _, x5, x6) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x5 == 3 && x6 & 1 == 1 => {
                                    Some(Op::aarch32_VSTM_T1A1_A)
                                }
                                (x0, x1, x2, x3, _, x5, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x5 == 2 => {
                                    Some(Op::aarch32_VLDM_T2A2_A)
                                }
                                (x0, x1, x2, x3, _, x5, x6) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x5 == 3 && x6 & 1 == 0 => {
                                    Some(Op::aarch32_VLDM_T1A1_A)
                                }
                                (x0, x1, x2, x3, _, x5, x6) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x5 == 3 && x6 & 1 == 1 => {
                                    Some(Op::aarch32_VLDM_T1A1_A)
                                }
                                (x0, _, x2, x3, x4, _, _) if x0 == 1 && x2 == 0 && x3 == 1 && x4 == 15 => {
                                    Some(Op::aarch32_VLDR_A1_A)
                                }
                                (x0, x1, x2, _, _, _, _) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        _ => None
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 != 15 && x2 == 2 && x4 & 6 == 4 && x6 == 1 => {
                    match ((instr >> 28) & 7, (instr >> 24) & 7, (instr >> 21) & 5, (instr >> 12) & 17, (instr >> 10) & 3, (instr >> 8) & 3, (instr >> 5) & 5, instr & 9) {
                        (_, _, x2, _, _, x5, _, _) if x2 == 0 && x5 == 1 => {
                            let cond = (instr >> 28) & 7;
                            let op = (instr >> 20) & 1;
                            let Vn = (instr >> 16) & 7;
                            let Rt = (instr >> 12) & 7;
                            let N = (instr >> 7) & 1;
                            match () {
                                () => {
                                    Some(Op::aarch32_VMOV_h_A1_A)
                                }
                            }
                        }
                        (_, _, x2, _, _, x5, _, _) if x2 == 0 && x5 == 2 => {
                            let cond = (instr >> 28) & 7;
                            let op = (instr >> 20) & 1;
                            let Vn = (instr >> 16) & 7;
                            let Rt = (instr >> 12) & 7;
                            let N = (instr >> 7) & 1;
                            match () {
                                () => {
                                    Some(Op::aarch32_VMOV_s_A1_A)
                                }
                            }
                        }
                        (_, _, x2, _, _, x5, _, _) if x2 == 7 && x5 == 2 => {
                            let cond = (instr >> 28) & 7;
                            let L = (instr >> 20) & 1;
                            let reg = (instr >> 16) & 7;
                            let Rt = (instr >> 12) & 7;
                            match L {
                                x0 if x0 == 0 => {
                                    Some(Op::aarch32_VMSR_T1A1_AS)
                                }
                                x0 if x0 == 1 => {
                                    Some(Op::aarch32_VMRS_T1A1_AS)
                                }
                                _ => None
                            }
                        }
                        (_, _, _, _, _, x5, _, _) if x5 == 3 => {
                            let cond = (instr >> 28) & 7;
                            let opc1 = (instr >> 21) & 5;
                            let L = (instr >> 20) & 1;
                            let Vn = (instr >> 16) & 7;
                            let Rt = (instr >> 12) & 7;
                            let N = (instr >> 7) & 1;
                            let opc2 = (instr >> 5) & 3;
                            match (opc1, L, opc2) {
                                (x0, x1, _) if x0 & 4 == 0 && x1 == 0 => {
                                    Some(Op::aarch32_VMOV_rs_T1A1_A)
                                }
                                (_, x1, _) if x1 == 1 => {
                                    Some(Op::aarch32_VMOV_sr_T1A1_A)
                                }
                                (x0, x1, x2) if x0 & 4 == 4 && x1 == 0 && x2 & 2 == 0 => {
                                    Some(Op::aarch32_VDUP_r_T1A1_A)
                                }
                                (x0, x1, x2) if x0 & 4 == 4 && x1 == 0 && x2 & 2 == 2 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        _ => None
                    }
                }
                _ => None
            }
        }
        (x0, x1, _, _, _) if x0 == 15 && x1 & 4 == 0 => {
            match ((instr >> 27) & 9, (instr >> 25) & 3, (instr >> 21) & 7, (instr >> 20) & 1, instr & 39) {
                (_, x1, _, _, _) if x1 == 0 => {
                    match ((instr >> 25) & 13, (instr >> 20) & 9, (instr >> 8) & 23, (instr >> 4) & 7, instr & 7) {
                        (_, x1, _, _, _) if x1 & 16 == 0 => {
                            None
                        }
                        (_, x1, _, x3, _) if x1 == 16 && x3 & 2 == 0 => {
                            let imod = (instr >> 18) & 3;
                            let M = (instr >> 17) & 1;
                            let op = (instr >> 16) & 1;
                            let E = (instr >> 9) & 1;
                            let A = (instr >> 8) & 1;
                            let I = (instr >> 7) & 1;
                            let F = (instr >> 6) & 1;
                            let mode = instr & 9;
                            match (imod, M, op, mode) {
                                (_, _, x2, x3) if x2 == 1 && x3 & 16 == 0 => {
                                    Some(Op::aarch32_SETEND_A1_A)
                                }
                                (_, _, x2, _) if x2 == 0 => {
                                    Some(Op::aarch32_CPS_A1_AS)
                                }
                                (_, _, x2, x3) if x2 == 1 && x3 & 16 == 16 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        (_, x1, _, x3, _) if x1 == 17 && x3 == 8 => {
                            None
                        }
                        (_, x1, _, x3, _) if x1 == 17 && x3 & 7 == 4 => {
                            None
                        }
                        (_, x1, _, x3, _) if x1 == 17 && x3 & 3 == 1 => {
                            None
                        }
                        (_, x1, _, x3, _) if x1 == 17 && x3 == 0 => {
                            let imm1 = (instr >> 9) & 1;
                            match () {
                                () => {
                                    Some(Op::aarch32_SETPAN_A1_A)
                                }
                            }
                        }
                        (_, x1, _, x3, _) if x1 & 30 == 16 && x3 == 7 => {
                            None
                        }
                        (_, x1, _, x3, _) if x1 == 18 && x3 == 7 => {
                            None
                        }
                        (_, x1, _, x3, _) if x1 == 19 && x3 == 7 => {
                            None
                        }
                        (_, x1, _, x3, _) if x1 & 30 == 18 && x3 & 2 == 0 => {
                            None
                        }
                        (_, x1, _, x3, _) if x1 & 28 == 16 && x3 == 3 => {
                            None
                        }
                        (_, x1, _, x3, _) if x1 & 28 == 16 && x3 & 11 == 2 => {
                            None
                        }
                        (_, x1, _, x3, _) if x1 & 28 == 16 && x3 & 10 == 10 => {
                            None
                        }
                        (_, x1, _, _, _) if x1 & 28 == 20 => {
                            None
                        }
                        (_, x1, _, _, _) if x1 & 24 == 24 => {
                            None
                        }
                        _ => None
                    }
                }
                (_, x1, _, _, _) if x1 == 1 => {
                    match ((instr >> 25) & 13, (instr >> 24) & 1, (instr >> 23) & 1, (instr >> 5) & 35, (instr >> 4) & 1, instr & 7) {
                        (_, _, x2, _, _, _) if x2 == 0 => {
                            let U = (instr >> 24) & 1;
                            let D = (instr >> 22) & 1;
                            let size = (instr >> 20) & 3;
                            let Vn = (instr >> 16) & 7;
                            let Vd = (instr >> 12) & 7;
                            let opc = (instr >> 8) & 7;
                            let N = (instr >> 7) & 1;
                            let Q = (instr >> 6) & 1;
                            let M = (instr >> 5) & 1;
                            let o1 = (instr >> 4) & 1;
                            let Vm = instr & 7;
                            match (U, size, opc, Q, o1) {
                                (x0, x1, x2, _, x4) if x0 == 0 && x1 & 2 == 0 && x2 == 12 && x4 == 1 => {
                                    Some(Op::aarch32_VFMA_A1_A)
                                }
                                (x0, x1, x2, _, x4) if x0 == 0 && x1 & 2 == 0 && x2 == 13 && x4 == 0 => {
                                    Some(Op::aarch32_VADD_f_A1_A)
                                }
                                (x0, x1, x2, _, x4) if x0 == 0 && x1 & 2 == 0 && x2 == 13 && x4 == 1 => {
                                    Some(Op::aarch32_VMLA_f_A1_A)
                                }
                                (x0, x1, x2, _, x4) if x0 == 0 && x1 & 2 == 0 && x2 == 14 && x4 == 0 => {
                                    Some(Op::aarch32_VCEQ_r_T1A1_A)
                                }
                                (x0, x1, x2, _, x4) if x0 == 0 && x1 & 2 == 0 && x2 == 15 && x4 == 0 => {
                                    Some(Op::aarch32_VMAX_f_A1_A)
                                }
                                (x0, x1, x2, _, x4) if x0 == 0 && x1 & 2 == 0 && x2 == 15 && x4 == 1 => {
                                    Some(Op::aarch32_VRECPS_A1_A)
                                }
                                (_, _, x2, _, x4) if x2 == 0 && x4 == 0 => {
                                    Some(Op::aarch32_VHADD_T1A1_A)
                                }
                                (x0, x1, x2, _, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x4 == 1 => {
                                    Some(Op::aarch32_VAND_r_T1A1_A)
                                }
                                (_, _, x2, _, x4) if x2 == 0 && x4 == 1 => {
                                    Some(Op::aarch32_VQADD_T1A1_A)
                                }
                                (_, _, x2, _, x4) if x2 == 1 && x4 == 0 => {
                                    Some(Op::aarch32_VRHADD_T1A1_A)
                                }
                                (x0, x1, x2, _, x4) if x0 == 0 && x1 == 0 && x2 == 12 && x4 == 0 => {
                                    Some(Op::aarch32_SHA1C_A1_A)
                                }
                                (_, _, x2, _, x4) if x2 == 2 && x4 == 0 => {
                                    Some(Op::aarch32_VHADD_T1A1_A)
                                }
                                (x0, x1, x2, _, x4) if x0 == 0 && x1 == 1 && x2 == 1 && x4 == 1 => {
                                    Some(Op::aarch32_VBIC_r_T1A1_A)
                                }
                                (_, _, x2, _, x4) if x2 == 2 && x4 == 1 => {
                                    Some(Op::aarch32_VQSUB_T1A1_A)
                                }
                                (_, _, x2, _, x4) if x2 == 3 && x4 == 0 => {
                                    Some(Op::aarch32_VCGT_r_T1A1_A)
                                }
                                (_, _, x2, _, x4) if x2 == 3 && x4 == 1 => {
                                    Some(Op::aarch32_VCGE_r_T1A1_A)
                                }
                                (x0, x1, x2, _, x4) if x0 == 0 && x1 == 1 && x2 == 12 && x4 == 0 => {
                                    Some(Op::aarch32_SHA1P_A1_A)
                                }
                                (x0, x1, x2, _, x4) if x0 == 0 && x1 & 2 == 2 && x2 == 12 && x4 == 1 => {
                                    Some(Op::aarch32_VFMA_A1_A)
                                }
                                (x0, x1, x2, _, x4) if x0 == 0 && x1 & 2 == 2 && x2 == 13 && x4 == 0 => {
                                    Some(Op::aarch32_VSUB_f_A1_A)
                                }
                                (x0, x1, x2, _, x4) if x0 == 0 && x1 & 2 == 2 && x2 == 13 && x4 == 1 => {
                                    Some(Op::aarch32_VMLA_f_A1_A)
                                }
                                (x0, x1, x2, _, x4) if x0 == 0 && x1 & 2 == 2 && x2 == 14 && x4 == 0 => {
                                    None
                                }
                                (x0, x1, x2, _, x4) if x0 == 0 && x1 & 2 == 2 && x2 == 15 && x4 == 0 => {
                                    Some(Op::aarch32_VMAX_f_A1_A)
                                }
                                (x0, x1, x2, _, x4) if x0 == 0 && x1 & 2 == 2 && x2 == 15 && x4 == 1 => {
                                    Some(Op::aarch32_VRSQRTS_A1_A)
                                }
                                (_, _, x2, _, x4) if x2 == 4 && x4 == 0 => {
                                    Some(Op::aarch32_VSHL_r_T1A1_A)
                                }
                                (x0, _, x2, _, x4) if x0 == 0 && x2 == 8 && x4 == 0 => {
                                    Some(Op::aarch32_VADD_i_T1A1_A)
                                }
                                (x0, x1, x2, _, x4) if x0 == 0 && x1 == 2 && x2 == 1 && x4 == 1 => {
                                    Some(Op::aarch32_VORR_r_T1A1_A)
                                }
                                (x0, _, x2, _, x4) if x0 == 0 && x2 == 8 && x4 == 1 => {
                                    Some(Op::aarch32_VTST_T1A1_A)
                                }
                                (_, _, x2, _, x4) if x2 == 4 && x4 == 1 => {
                                    Some(Op::aarch32_VQSHL_r_T1A1_A)
                                }
                                (x0, _, x2, _, x4) if x0 == 0 && x2 == 9 && x4 == 0 => {
                                    Some(Op::aarch32_VMLA_i_T1A1_A)
                                }
                                (_, _, x2, _, x4) if x2 == 5 && x4 == 0 => {
                                    Some(Op::aarch32_VRSHL_T1A1_A)
                                }
                                (_, _, x2, _, x4) if x2 == 5 && x4 == 1 => {
                                    Some(Op::aarch32_VQRSHL_T1A1_A)
                                }
                                (x0, _, x2, _, x4) if x0 == 0 && x2 == 11 && x4 == 0 => {
                                    Some(Op::aarch32_VQDMULH_T1A1_A)
                                }
                                (x0, x1, x2, _, x4) if x0 == 0 && x1 == 2 && x2 == 12 && x4 == 0 => {
                                    Some(Op::aarch32_SHA1M_A1_A)
                                }
                                (x0, _, x2, _, x4) if x0 == 0 && x2 == 11 && x4 == 1 => {
                                    Some(Op::aarch32_VPADD_i_T1A1_A)
                                }
                                (_, _, x2, _, x4) if x2 == 6 && x4 == 0 => {
                                    Some(Op::aarch32_VMAX_i_T1A1_A)
                                }
                                (x0, x1, x2, _, x4) if x0 == 0 && x1 == 3 && x2 == 1 && x4 == 1 => {
                                    Some(Op::aarch32_VORN_r_T1A1_A)
                                }
                                (_, _, x2, _, x4) if x2 == 6 && x4 == 1 => {
                                    Some(Op::aarch32_VMAX_i_T1A1_A)
                                }
                                (_, _, x2, _, x4) if x2 == 7 && x4 == 0 => {
                                    Some(Op::aarch32_VABD_i_T1A1_A)
                                }
                                (_, _, x2, _, x4) if x2 == 7 && x4 == 1 => {
                                    Some(Op::aarch32_VABA_T1A1_A)
                                }
                                (x0, x1, x2, _, x4) if x0 == 0 && x1 == 3 && x2 == 12 && x4 == 0 => {
                                    Some(Op::aarch32_SHA1SU0_A1_A)
                                }
                                (x0, x1, x2, _, x4) if x0 == 1 && x1 & 2 == 0 && x2 == 13 && x4 == 0 => {
                                    Some(Op::aarch32_VPADD_f_A1_A)
                                }
                                (x0, x1, x2, _, x4) if x0 == 1 && x1 & 2 == 0 && x2 == 13 && x4 == 1 => {
                                    Some(Op::aarch32_VMUL_f_A1_A)
                                }
                                (x0, x1, x2, _, x4) if x0 == 1 && x1 & 2 == 0 && x2 == 14 && x4 == 0 => {
                                    Some(Op::aarch32_VCGE_r_T1A1_A)
                                }
                                (x0, x1, x2, _, x4) if x0 == 1 && x1 & 2 == 0 && x2 == 14 && x4 == 1 => {
                                    Some(Op::aarch32_VACGE_A1_A)
                                }
                                (x0, x1, x2, x3, x4) if x0 == 1 && x1 & 2 == 0 && x2 == 15 && x3 == 0 && x4 == 0 => {
                                    Some(Op::aarch32_VPMAX_f_A1_A)
                                }
                                (x0, x1, x2, _, x4) if x0 == 1 && x1 & 2 == 0 && x2 == 15 && x4 == 1 => {
                                    Some(Op::aarch32_VMAXNM_A1_A)
                                }
                                (x0, x1, x2, _, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x4 == 1 => {
                                    Some(Op::aarch32_VEOR_T1A1_A)
                                }
                                (_, _, x2, _, x4) if x2 == 9 && x4 == 1 => {
                                    Some(Op::aarch32_VMUL_i_T1A1_A)
                                }
                                (x0, x1, x2, _, x4) if x0 == 1 && x1 == 0 && x2 == 12 && x4 == 0 => {
                                    Some(Op::aarch32_SHA256H_A1_A)
                                }
                                (_, _, x2, x3, x4) if x2 == 10 && x3 == 0 && x4 == 0 => {
                                    Some(Op::aarch32_VPMAX_i_T1A1_A)
                                }
                                (x0, x1, x2, _, x4) if x0 == 1 && x1 == 1 && x2 == 1 && x4 == 1 => {
                                    Some(Op::aarch32_VBIF_T1A1_A)
                                }
                                (_, _, x2, x3, x4) if x2 == 10 && x3 == 0 && x4 == 1 => {
                                    Some(Op::aarch32_VPMAX_i_T1A1_A)
                                }
                                (_, _, x2, x3, _) if x2 == 10 && x3 == 1 => {
                                    None
                                }
                                (x0, x1, x2, _, x4) if x0 == 1 && x1 == 1 && x2 == 12 && x4 == 0 => {
                                    Some(Op::aarch32_SHA256H2_A1_A)
                                }
                                (x0, x1, x2, _, x4) if x0 == 1 && x1 & 2 == 2 && x2 == 13 && x4 == 0 => {
                                    Some(Op::aarch32_VABD_f_A1_A)
                                }
                                (x0, x1, x2, _, x4) if x0 == 1 && x1 & 2 == 2 && x2 == 14 && x4 == 0 => {
                                    Some(Op::aarch32_VCGT_r_T1A1_A)
                                }
                                (x0, x1, x2, _, x4) if x0 == 1 && x1 & 2 == 2 && x2 == 14 && x4 == 1 => {
                                    Some(Op::aarch32_VACGE_A1_A)
                                }
                                (x0, x1, x2, x3, x4) if x0 == 1 && x1 & 2 == 2 && x2 == 15 && x3 == 0 && x4 == 0 => {
                                    Some(Op::aarch32_VPMAX_f_A1_A)
                                }
                                (x0, x1, x2, _, x4) if x0 == 1 && x1 & 2 == 2 && x2 == 15 && x4 == 1 => {
                                    Some(Op::aarch32_VMAXNM_A1_A)
                                }
                                (x0, _, x2, _, x4) if x0 == 1 && x2 == 8 && x4 == 0 => {
                                    Some(Op::aarch32_VSUB_i_T1A1_A)
                                }
                                (x0, x1, x2, _, x4) if x0 == 1 && x1 == 2 && x2 == 1 && x4 == 1 => {
                                    Some(Op::aarch32_VBIF_T1A1_A)
                                }
                                (x0, _, x2, _, x4) if x0 == 1 && x2 == 8 && x4 == 1 => {
                                    Some(Op::aarch32_VCEQ_r_T1A1_A)
                                }
                                (x0, _, x2, _, x4) if x0 == 1 && x2 == 9 && x4 == 0 => {
                                    Some(Op::aarch32_VMLA_i_T1A1_A)
                                }
                                (x0, _, x2, _, x4) if x0 == 1 && x2 == 11 && x4 == 0 => {
                                    Some(Op::aarch32_VQRDMULH_T1A1_A)
                                }
                                (x0, x1, x2, _, x4) if x0 == 1 && x1 == 2 && x2 == 12 && x4 == 0 => {
                                    Some(Op::aarch32_SHA256SU1_A1_A)
                                }
                                (x0, _, x2, _, x4) if x0 == 1 && x2 == 11 && x4 == 1 => {
                                    Some(Op::aarch32_VQRDMLAH_A1_A)
                                }
                                (x0, x1, x2, _, x4) if x0 == 1 && x1 == 3 && x2 == 1 && x4 == 1 => {
                                    Some(Op::aarch32_VBIF_T1A1_A)
                                }
                                (x0, _, x2, _, x4) if x0 == 1 && x2 == 12 && x4 == 1 => {
                                    Some(Op::aarch32_VQRDMLSH_A1_A)
                                }
                                (x0, _, x2, x3, x4) if x0 == 1 && x2 == 15 && x3 == 1 && x4 == 0 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        (_, _, x2, _, x4, _) if x2 == 1 && x4 == 0 => {
                            match ((instr >> 25) & 13, (instr >> 24) & 1, (instr >> 23) & 1, (instr >> 22) & 1, (instr >> 20) & 3, (instr >> 12) & 15, (instr >> 10) & 3, (instr >> 7) & 5, (instr >> 6) & 1, (instr >> 5) & 1, (instr >> 4) & 1, instr & 7) {
                                (_, x1, _, _, x4, _, _, _, _, _, _, _) if x1 == 0 && x4 == 3 => {
                                    let D = (instr >> 22) & 1;
                                    let Vn = (instr >> 16) & 7;
                                    let Vd = (instr >> 12) & 7;
                                    let imm4 = (instr >> 8) & 7;
                                    let N = (instr >> 7) & 1;
                                    let Q = (instr >> 6) & 1;
                                    let M = (instr >> 5) & 1;
                                    let Vm = instr & 7;
                                    match () {
                                        () => {
                                            Some(Op::aarch32_VEXT_T1A1_A)
                                        }
                                    }
                                }
                                (_, x1, _, _, x4, _, x6, _, _, _, _, _) if x1 == 1 && x4 == 3 && x6 & 2 == 0 => {
                                    let D = (instr >> 22) & 1;
                                    let size = (instr >> 18) & 3;
                                    let opc1 = (instr >> 16) & 3;
                                    let Vd = (instr >> 12) & 7;
                                    let opc2 = (instr >> 7) & 7;
                                    let Q = (instr >> 6) & 1;
                                    let M = (instr >> 5) & 1;
                                    let Vm = instr & 7;
                                    match (size, opc1, opc2, Q) {
                                        (_, x1, x2, _) if x1 == 0 && x2 == 0 => {
                                            Some(Op::aarch32_VREV16_T1A1_A)
                                        }
                                        (_, x1, x2, _) if x1 == 0 && x2 == 1 => {
                                            Some(Op::aarch32_VREV16_T1A1_A)
                                        }
                                        (_, x1, x2, _) if x1 == 0 && x2 == 2 => {
                                            Some(Op::aarch32_VREV16_T1A1_A)
                                        }
                                        (_, x1, x2, _) if x1 == 0 && x2 == 3 => {
                                            None
                                        }
                                        (_, x1, x2, _) if x1 == 0 && x2 & 14 == 4 => {
                                            Some(Op::aarch32_VPADDL_T1A1_A)
                                        }
                                        (_, x1, x2, x3) if x1 == 0 && x2 == 6 && x3 == 0 => {
                                            Some(Op::aarch32_AESE_A1_A)
                                        }
                                        (_, x1, x2, x3) if x1 == 0 && x2 == 6 && x3 == 1 => {
                                            Some(Op::aarch32_AESD_A1_A)
                                        }
                                        (_, x1, x2, x3) if x1 == 0 && x2 == 7 && x3 == 0 => {
                                            Some(Op::aarch32_AESMC_A1_A)
                                        }
                                        (_, x1, x2, x3) if x1 == 0 && x2 == 7 && x3 == 1 => {
                                            Some(Op::aarch32_AESIMC_A1_A)
                                        }
                                        (_, x1, x2, _) if x1 == 0 && x2 == 8 => {
                                            Some(Op::aarch32_VCLS_T1A1_A)
                                        }
                                        (x0, x1, x2, _) if x0 == 0 && x1 == 2 && x2 == 0 => {
                                            Some(Op::aarch32_VSWP_T1A1_A)
                                        }
                                        (_, x1, x2, _) if x1 == 0 && x2 == 9 => {
                                            Some(Op::aarch32_VCLZ_T1A1_A)
                                        }
                                        (_, x1, x2, _) if x1 == 0 && x2 == 10 => {
                                            Some(Op::aarch32_VCNT_T1A1_A)
                                        }
                                        (_, x1, x2, _) if x1 == 0 && x2 == 11 => {
                                            Some(Op::aarch32_VMVN_r_T1A1_A)
                                        }
                                        (x0, x1, x2, x3) if x0 == 0 && x1 == 2 && x2 == 12 && x3 == 1 => {
                                            None
                                        }
                                        (_, x1, x2, _) if x1 == 0 && x2 & 14 == 12 => {
                                            Some(Op::aarch32_VPADAL_T1A1_A)
                                        }
                                        (_, x1, x2, _) if x1 == 0 && x2 == 14 => {
                                            Some(Op::aarch32_VQABS_T1A1_A)
                                        }
                                        (_, x1, x2, _) if x1 == 0 && x2 == 15 => {
                                            Some(Op::aarch32_VQNEG_T1A1_A)
                                        }
                                        (_, x1, x2, _) if x1 == 1 && x2 & 7 == 0 => {
                                            Some(Op::aarch32_VCGT_i_A1_A)
                                        }
                                        (_, x1, x2, _) if x1 == 1 && x2 & 7 == 1 => {
                                            Some(Op::aarch32_VCGE_i_A1_A)
                                        }
                                        (_, x1, x2, _) if x1 == 1 && x2 & 7 == 2 => {
                                            Some(Op::aarch32_VCEQ_i_A1_A)
                                        }
                                        (_, x1, x2, _) if x1 == 1 && x2 & 7 == 3 => {
                                            Some(Op::aarch32_VCLE_i_A1_A)
                                        }
                                        (_, x1, x2, _) if x1 == 1 && x2 & 7 == 4 => {
                                            Some(Op::aarch32_VCLT_i_A1_A)
                                        }
                                        (_, x1, x2, _) if x1 == 1 && x2 & 7 == 6 => {
                                            Some(Op::aarch32_VABS_A1_A)
                                        }
                                        (_, x1, x2, _) if x1 == 1 && x2 & 7 == 7 => {
                                            Some(Op::aarch32_VNEG_A1_A)
                                        }
                                        (_, x1, x2, x3) if x1 == 1 && x2 == 5 && x3 == 1 => {
                                            Some(Op::aarch32_SHA1H_A1_A)
                                        }
                                        (x0, x1, x2, x3) if x0 == 1 && x1 == 2 && x2 == 12 && x3 == 1 => {
                                            Some(Op::aarch32_VCVT_T1A1_A)
                                        }
                                        (_, x1, x2, _) if x1 == 2 && x2 == 1 => {
                                            Some(Op::aarch32_VTRN_T1A1_A)
                                        }
                                        (_, x1, x2, _) if x1 == 2 && x2 == 2 => {
                                            Some(Op::aarch32_VUZP_T1A1_A)
                                        }
                                        (_, x1, x2, _) if x1 == 2 && x2 == 3 => {
                                            Some(Op::aarch32_VZIP_T1A1_A)
                                        }
                                        (_, x1, x2, x3) if x1 == 2 && x2 == 4 && x3 == 0 => {
                                            Some(Op::aarch32_VMOVN_T1A1_A)
                                        }
                                        (_, x1, x2, x3) if x1 == 2 && x2 == 4 && x3 == 1 => {
                                            Some(Op::aarch32_VQMOVN_T1A1_A)
                                        }
                                        (_, x1, x2, _) if x1 == 2 && x2 == 5 => {
                                            Some(Op::aarch32_VQMOVN_T1A1_A)
                                        }
                                        (_, x1, x2, x3) if x1 == 2 && x2 == 6 && x3 == 0 => {
                                            Some(Op::aarch32_VSHLL_T2A2_A)
                                        }
                                        (_, x1, x2, x3) if x1 == 2 && x2 == 7 && x3 == 0 => {
                                            Some(Op::aarch32_SHA1SU1_A1_A)
                                        }
                                        (_, x1, x2, x3) if x1 == 2 && x2 == 7 && x3 == 1 => {
                                            Some(Op::aarch32_SHA256SU0_A1_A)
                                        }
                                        (_, x1, x2, _) if x1 == 2 && x2 == 8 => {
                                            Some(Op::aarch32_VRINTA_asimd_A1_A)
                                        }
                                        (_, x1, x2, _) if x1 == 2 && x2 == 9 => {
                                            Some(Op::aarch32_VRINTX_asimd_A1_A)
                                        }
                                        (_, x1, x2, _) if x1 == 2 && x2 == 10 => {
                                            Some(Op::aarch32_VRINTA_asimd_A1_A)
                                        }
                                        (_, x1, x2, _) if x1 == 2 && x2 == 11 => {
                                            Some(Op::aarch32_VRINTZ_asimd_A1_A)
                                        }
                                        (x0, x1, x2, x3) if x0 == 2 && x1 == 2 && x2 == 12 && x3 == 1 => {
                                            None
                                        }
                                        (_, x1, x2, x3) if x1 == 2 && x2 == 12 && x3 == 0 => {
                                            Some(Op::aarch32_VCVT_hs_T1A1_A)
                                        }
                                        (_, x1, x2, _) if x1 == 2 && x2 == 13 => {
                                            Some(Op::aarch32_VRINTA_asimd_A1_A)
                                        }
                                        (_, x1, x2, x3) if x1 == 2 && x2 == 14 && x3 == 0 => {
                                            Some(Op::aarch32_VCVT_hs_T1A1_A)
                                        }
                                        (_, x1, x2, x3) if x1 == 2 && x2 == 14 && x3 == 1 => {
                                            None
                                        }
                                        (_, x1, x2, _) if x1 == 2 && x2 == 15 => {
                                            Some(Op::aarch32_VRINTA_asimd_A1_A)
                                        }
                                        (_, x1, x2, _) if x1 == 3 && x2 & 14 == 0 => {
                                            Some(Op::aarch32_VCVTA_asimd_A1_A)
                                        }
                                        (_, x1, x2, _) if x1 == 3 && x2 & 14 == 2 => {
                                            Some(Op::aarch32_VCVTA_asimd_A1_A)
                                        }
                                        (_, x1, x2, _) if x1 == 3 && x2 & 14 == 4 => {
                                            Some(Op::aarch32_VCVTA_asimd_A1_A)
                                        }
                                        (_, x1, x2, _) if x1 == 3 && x2 & 14 == 6 => {
                                            Some(Op::aarch32_VCVTA_asimd_A1_A)
                                        }
                                        (_, x1, x2, _) if x1 == 3 && x2 & 13 == 8 => {
                                            Some(Op::aarch32_VRECPE_A1_A)
                                        }
                                        (_, x1, x2, _) if x1 == 3 && x2 & 13 == 9 => {
                                            Some(Op::aarch32_VRSQRTE_A1_A)
                                        }
                                        (x0, x1, x2, x3) if x0 == 3 && x1 == 2 && x2 == 12 && x3 == 1 => {
                                            None
                                        }
                                        (_, x1, x2, _) if x1 == 3 && x2 & 12 == 12 => {
                                            Some(Op::aarch32_VCVT_is_A1_A)
                                        }
                                        _ => None
                                    }
                                }
                                (_, x1, _, _, x4, _, x6, _, _, _, _, _) if x1 == 1 && x4 == 3 && x6 == 2 => {
                                    let D = (instr >> 22) & 1;
                                    let Vn = (instr >> 16) & 7;
                                    let Vd = (instr >> 12) & 7;
                                    let len = (instr >> 8) & 3;
                                    let N = (instr >> 7) & 1;
                                    let op = (instr >> 6) & 1;
                                    let M = (instr >> 5) & 1;
                                    let Vm = instr & 7;
                                    match () {
                                        () => {
                                            Some(Op::aarch32_VTBL_T1A1_A)
                                        }
                                    }
                                }
                                (_, x1, _, _, x4, _, x6, _, _, _, _, _) if x1 == 1 && x4 == 3 && x6 == 3 => {
                                    let D = (instr >> 22) & 1;
                                    let imm4 = (instr >> 16) & 7;
                                    let Vd = (instr >> 12) & 7;
                                    let opc = (instr >> 7) & 5;
                                    let Q = (instr >> 6) & 1;
                                    let M = (instr >> 5) & 1;
                                    let Vm = instr & 7;
                                    match opc {
                                        x0 if x0 == 0 => {
                                            Some(Op::aarch32_VDUP_s_T1A1_A)
                                        }
                                        x0 if x0 == 1 => {
                                            None
                                        }
                                        x0 if x0 & 6 == 2 => {
                                            None
                                        }
                                        x0 if x0 & 4 == 4 => {
                                            None
                                        }
                                        _ => None
                                    }
                                }
                                (_, _, _, _, x4, _, _, _, x8, _, _, _) if x4 != 3 && x8 == 0 => {
                                    let U = (instr >> 24) & 1;
                                    let D = (instr >> 22) & 1;
                                    let size = (instr >> 20) & 3;
                                    let Vn = (instr >> 16) & 7;
                                    let Vd = (instr >> 12) & 7;
                                    let opc = (instr >> 8) & 7;
                                    let N = (instr >> 7) & 1;
                                    let M = (instr >> 5) & 1;
                                    let Vm = instr & 7;
                                    match (U, opc) {
                                        (_, x1) if x1 == 0 => {
                                            Some(Op::aarch32_VADDL_T1A1_A)
                                        }
                                        (_, x1) if x1 == 1 => {
                                            Some(Op::aarch32_VADDL_T1A1_A)
                                        }
                                        (_, x1) if x1 == 2 => {
                                            Some(Op::aarch32_VSUBL_T1A1_A)
                                        }
                                        (x0, x1) if x0 == 0 && x1 == 4 => {
                                            Some(Op::aarch32_VADDHN_T1A1_A)
                                        }
                                        (_, x1) if x1 == 3 => {
                                            Some(Op::aarch32_VSUBL_T1A1_A)
                                        }
                                        (x0, x1) if x0 == 0 && x1 == 6 => {
                                            Some(Op::aarch32_VSUBHN_T1A1_A)
                                        }
                                        (x0, x1) if x0 == 0 && x1 == 9 => {
                                            Some(Op::aarch32_VQDMLAL_T1A1_A)
                                        }
                                        (_, x1) if x1 == 5 => {
                                            Some(Op::aarch32_VABA_T2A2_A)
                                        }
                                        (x0, x1) if x0 == 0 && x1 == 11 => {
                                            Some(Op::aarch32_VQDMLSL_T1A1_A)
                                        }
                                        (x0, x1) if x0 == 0 && x1 == 13 => {
                                            Some(Op::aarch32_VQDMULL_T1A1_A)
                                        }
                                        (_, x1) if x1 == 7 => {
                                            Some(Op::aarch32_VABD_i_T2A2_A)
                                        }
                                        (_, x1) if x1 == 8 => {
                                            Some(Op::aarch32_VMLA_i_T2A2_A)
                                        }
                                        (_, x1) if x1 == 10 => {
                                            Some(Op::aarch32_VMLA_i_T2A2_A)
                                        }
                                        (x0, x1) if x0 == 1 && x1 == 4 => {
                                            Some(Op::aarch32_VRADDHN_T1A1_A)
                                        }
                                        (x0, x1) if x0 == 1 && x1 == 6 => {
                                            Some(Op::aarch32_VRSUBHN_T1A1_A)
                                        }
                                        (_, x1) if x1 & 13 == 12 => {
                                            Some(Op::aarch32_VMUL_i_A2_A)
                                        }
                                        (x0, x1) if x0 == 1 && x1 == 9 => {
                                            None
                                        }
                                        (x0, x1) if x0 == 1 && x1 == 11 => {
                                            None
                                        }
                                        (x0, x1) if x0 == 1 && x1 == 13 => {
                                            None
                                        }
                                        (_, x1) if x1 == 15 => {
                                            None
                                        }
                                        _ => None
                                    }
                                }
                                (_, _, _, _, x4, _, _, _, x8, _, _, _) if x4 != 3 && x8 == 1 => {
                                    let Q = (instr >> 24) & 1;
                                    let D = (instr >> 22) & 1;
                                    let size = (instr >> 20) & 3;
                                    let Vn = (instr >> 16) & 7;
                                    let Vd = (instr >> 12) & 7;
                                    let opc = (instr >> 8) & 7;
                                    let N = (instr >> 7) & 1;
                                    let M = (instr >> 5) & 1;
                                    let Vm = instr & 7;
                                    match (Q, opc) {
                                        (_, x1) if x1 & 14 == 0 => {
                                            Some(Op::aarch32_VMLA_s_A1_A)
                                        }
                                        (x0, x1) if x0 == 0 && x1 == 3 => {
                                            Some(Op::aarch32_VQDMLAL_T2A2_A)
                                        }
                                        (_, x1) if x1 == 2 => {
                                            Some(Op::aarch32_VMLA_s_T2A2_A)
                                        }
                                        (x0, x1) if x0 == 0 && x1 == 7 => {
                                            Some(Op::aarch32_VQDMLSL_T2A2_A)
                                        }
                                        (_, x1) if x1 & 14 == 4 => {
                                            Some(Op::aarch32_VMLA_s_A1_A)
                                        }
                                        (x0, x1) if x0 == 0 && x1 == 11 => {
                                            Some(Op::aarch32_VQDMULL_T2A2_A)
                                        }
                                        (_, x1) if x1 == 6 => {
                                            Some(Op::aarch32_VMLA_s_T2A2_A)
                                        }
                                        (_, x1) if x1 & 14 == 8 => {
                                            Some(Op::aarch32_VMUL_s_A1_A)
                                        }
                                        (x0, x1) if x0 == 1 && x1 == 3 => {
                                            None
                                        }
                                        (_, x1) if x1 == 10 => {
                                            Some(Op::aarch32_VMUL_s_T2A2_A)
                                        }
                                        (x0, x1) if x0 == 1 && x1 == 7 => {
                                            None
                                        }
                                        (_, x1) if x1 == 12 => {
                                            Some(Op::aarch32_VQDMULH_T2A2_A)
                                        }
                                        (_, x1) if x1 == 13 => {
                                            Some(Op::aarch32_VQRDMULH_T2A2_A)
                                        }
                                        (x0, x1) if x0 == 1 && x1 == 11 => {
                                            None
                                        }
                                        (_, x1) if x1 == 14 => {
                                            Some(Op::aarch32_VQRDMLAH_A2_A)
                                        }
                                        (_, x1) if x1 == 15 => {
                                            Some(Op::aarch32_VQRDMLSH_A2_A)
                                        }
                                        _ => None
                                    }
                                }
                                _ => None
                            }
                        }
                        (_, _, x2, _, x4, _) if x2 == 1 && x4 == 1 => {
                            match ((instr >> 25) & 13, (instr >> 24) & 1, (instr >> 23) & 1, (instr >> 22) & 1, (instr >> 7) & 29, (instr >> 5) & 3, (instr >> 4) & 1, instr & 7) {
                                (_, _, _, _, x4, _, _, _) if x4 & 28673 == 0 => {
                                    let i = (instr >> 24) & 1;
                                    let D = (instr >> 22) & 1;
                                    let imm3 = (instr >> 16) & 5;
                                    let Vd = (instr >> 12) & 7;
                                    let cmode = (instr >> 8) & 7;
                                    let Q = (instr >> 6) & 1;
                                    let op = (instr >> 5) & 1;
                                    let imm4 = instr & 7;
                                    match (cmode, op) {
                                        (x0, x1) if x0 & 9 == 0 && x1 == 0 => {
                                            Some(Op::aarch32_VMOV_i_T1A1_A)
                                        }
                                        (x0, x1) if x0 & 9 == 0 && x1 == 1 => {
                                            Some(Op::aarch32_VMVN_i_T1A1_A)
                                        }
                                        (x0, x1) if x0 & 9 == 1 && x1 == 0 => {
                                            Some(Op::aarch32_VORR_i_T1A1_A)
                                        }
                                        (x0, x1) if x0 & 9 == 1 && x1 == 1 => {
                                            Some(Op::aarch32_VBIC_i_T1A1_A)
                                        }
                                        (x0, x1) if x0 & 13 == 8 && x1 == 0 => {
                                            Some(Op::aarch32_VMOV_i_T1A1_A)
                                        }
                                        (x0, x1) if x0 & 13 == 8 && x1 == 1 => {
                                            Some(Op::aarch32_VMVN_i_T1A1_A)
                                        }
                                        (x0, x1) if x0 & 13 == 9 && x1 == 0 => {
                                            Some(Op::aarch32_VORR_i_T1A1_A)
                                        }
                                        (x0, x1) if x0 & 13 == 9 && x1 == 1 => {
                                            Some(Op::aarch32_VBIC_i_T1A1_A)
                                        }
                                        (x0, x1) if x0 & 12 == 12 && x1 == 0 => {
                                            Some(Op::aarch32_VMOV_i_T1A1_A)
                                        }
                                        (x0, x1) if x0 & 14 == 12 && x1 == 1 => {
                                            Some(Op::aarch32_VMVN_i_T1A1_A)
                                        }
                                        (x0, x1) if x0 == 14 && x1 == 1 => {
                                            Some(Op::aarch32_VMOV_i_T1A1_A)
                                        }
                                        (x0, x1) if x0 == 15 && x1 == 1 => {
                                            None
                                        }
                                        _ => None
                                    }
                                }
                                (_, _, _, _, x4, _, _, _) if x4 & 28673 != 0 => {
                                    let U = (instr >> 24) & 1;
                                    let D = (instr >> 22) & 1;
                                    let imm3H = (instr >> 19) & 5;
                                    let imm3L = (instr >> 16) & 5;
                                    let Vd = (instr >> 12) & 7;
                                    let opc = (instr >> 8) & 7;
                                    let L = (instr >> 7) & 1;
                                    let Q = (instr >> 6) & 1;
                                    let M = (instr >> 5) & 1;
                                    let Vm = instr & 7;
                                    match (U, (imm3H << 1) | L, imm3L, opc, Q) {
                                        (_, x1, _, x3, _) if x1 != 0 && x3 == 0 => {
                                            Some(Op::aarch32_VSHR_T1A1_A)
                                        }
                                        (_, x1, _, x3, _) if x1 != 0 && x3 == 1 => {
                                            Some(Op::aarch32_VSRA_T1A1_A)
                                        }
                                        (_, x1, x2, x3, x4) if x1 != 0 && x2 == 0 && x3 == 10 && x4 == 0 => {
                                            Some(Op::aarch32_VMOVL_T1A1_A)
                                        }
                                        (_, x1, _, x3, _) if x1 != 0 && x3 == 2 => {
                                            Some(Op::aarch32_VRSHR_T1A1_A)
                                        }
                                        (_, x1, _, x3, _) if x1 != 0 && x3 == 3 => {
                                            Some(Op::aarch32_VRSRA_T1A1_A)
                                        }
                                        (_, x1, _, x3, _) if x1 != 0 && x3 == 7 => {
                                            Some(Op::aarch32_VQSHL_i_T1A1_A)
                                        }
                                        (_, x1, _, x3, x4) if x1 != 0 && x3 == 9 && x4 == 0 => {
                                            Some(Op::aarch32_VQSHRN_T1A1_A)
                                        }
                                        (_, x1, _, x3, x4) if x1 != 0 && x3 == 9 && x4 == 1 => {
                                            Some(Op::aarch32_VQRSHRN_T1A1_A)
                                        }
                                        (_, x1, _, x3, x4) if x1 != 0 && x3 == 10 && x4 == 0 => {
                                            Some(Op::aarch32_VSHLL_T1A1_A)
                                        }
                                        (_, x1, _, x3, _) if x1 != 0 && x3 & 12 == 12 => {
                                            Some(Op::aarch32_VCVT_xs_A1_A)
                                        }
                                        (x0, x1, _, x3, _) if x0 == 0 && x1 != 0 && x3 == 5 => {
                                            Some(Op::aarch32_VSHL_i_T1A1_A)
                                        }
                                        (x0, x1, _, x3, x4) if x0 == 0 && x1 != 0 && x3 == 8 && x4 == 0 => {
                                            Some(Op::aarch32_VSHRN_T1A1_A)
                                        }
                                        (x0, x1, _, x3, x4) if x0 == 0 && x1 != 0 && x3 == 8 && x4 == 1 => {
                                            Some(Op::aarch32_VRSHRN_T1A1_A)
                                        }
                                        (x0, x1, _, x3, _) if x0 == 1 && x1 != 0 && x3 == 4 => {
                                            Some(Op::aarch32_VSRI_T1A1_A)
                                        }
                                        (x0, x1, _, x3, _) if x0 == 1 && x1 != 0 && x3 == 5 => {
                                            Some(Op::aarch32_VSLI_T1A1_A)
                                        }
                                        (x0, x1, _, x3, _) if x0 == 1 && x1 != 0 && x3 == 6 => {
                                            Some(Op::aarch32_VQSHL_i_T1A1_A)
                                        }
                                        (x0, x1, _, x3, x4) if x0 == 1 && x1 != 0 && x3 == 8 && x4 == 0 => {
                                            Some(Op::aarch32_VQSHRN_T1A1_A)
                                        }
                                        (x0, x1, _, x3, x4) if x0 == 1 && x1 != 0 && x3 == 8 && x4 == 1 => {
                                            Some(Op::aarch32_VQRSHRN_T1A1_A)
                                        }
                                        _ => None
                                    }
                                }
                                _ => None
                            }
                        }
                        _ => None
                    }
                }
                (_, x1, _, x3, _) if x1 & 2 == 2 && x3 == 1 => {
                    match ((instr >> 26) & 11, (instr >> 21) & 9, (instr >> 20) & 1, (instr >> 5) & 29, (instr >> 4) & 1, instr & 7) {
                        (_, x1, _, _, _, _) if x1 & 25 == 1 => {
                            None
                        }
                        (_, x1, _, _, _, _) if x1 == 9 => {
                            None
                        }
                        (_, x1, _, _, _, _) if x1 == 11 => {
                            let opcode = (instr >> 4) & 7;
                            let option = instr & 7;
                            match (opcode, option) {
                                (x0, _) if x0 == 0 => {
                                    None
                                }
                                (x0, _) if x0 == 1 => {
                                    Some(Op::aarch32_CLREX_A1_A)
                                }
                                (x0, _) if x0 & 14 == 2 => {
                                    None
                                }
                                (x0, x1) if x0 == 4 && x1 & 11 != 0 => {
                                    Some(Op::aarch32_DSB_A1_A)
                                }
                                (x0, x1) if x0 == 4 && x1 == 0 => {
                                    Some(Op::aarch32_SSBB_A1_A)
                                }
                                (x0, x1) if x0 == 4 && x1 == 4 => {
                                    Some(Op::aarch32_PSSBB_A1_A)
                                }
                                (x0, _) if x0 == 5 => {
                                    Some(Op::aarch32_DMB_A1_A)
                                }
                                (x0, _) if x0 == 6 => {
                                    Some(Op::aarch32_ISB_A1_A)
                                }
                                (x0, _) if x0 == 7 => {
                                    Some(Op::aarch32_SB_A1_A)
                                }
                                (x0, _) if x0 & 8 == 8 => {
                                    None
                                }
                                _ => None
                            }
                        }
                        (_, x1, _, _, _, _) if x1 & 29 == 13 => {
                            None
                        }
                        (_, x1, _, _, _, _) if x1 & 17 == 0 => {
                            let D = (instr >> 24) & 1;
                            let U = (instr >> 23) & 1;
                            let R = (instr >> 22) & 1;
                            let Rn = (instr >> 16) & 7;
                            let imm12 = instr & 23;
                            match (D, R, Rn) {
                                (x0, x1, _) if x0 == 0 && x1 == 0 => {
                                    Some(Op::Nop)
                                }
                                (x0, x1, _) if x0 == 0 && x1 == 1 => {
                                    Some(Op::aarch32_PLI_i_A1_A)
                                }
                                (x0, _, x2) if x0 == 1 && x2 == 15 => {
                                    Some(Op::aarch32_PLD_l_A1_A)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 != 15 => {
                                    Some(Op::aarch32_PLD_i_A1_A)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 != 15 => {
                                    Some(Op::aarch32_PLD_i_A1_A)
                                }
                                _ => None
                            }
                        }
                        (_, x1, _, _, x4, _) if x1 & 17 == 16 && x4 == 0 => {
                            let D = (instr >> 24) & 1;
                            let U = (instr >> 23) & 1;
                            let o2 = (instr >> 22) & 1;
                            let Rn = (instr >> 16) & 7;
                            let imm5 = (instr >> 7) & 9;
                            let stype = (instr >> 5) & 3;
                            let Rm = instr & 7;
                            match (D, o2) {
                                (x0, x1) if x0 == 0 && x1 == 0 => {
                                    Some(Op::Nop)
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    Some(Op::aarch32_PLI_r_A1_A)
                                }
                                (x0, x1) if x0 == 1 && x1 == 0 => {
                                    Some(Op::aarch32_PLD_r_A1_A)
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    Some(Op::aarch32_PLD_r_A1_A)
                                }
                                _ => None
                            }
                        }
                        (_, x1, _, _, x4, _) if x1 & 17 == 17 && x4 == 0 => {
                            None
                        }
                        (_, x1, _, _, x4, _) if x1 & 16 == 16 && x4 == 1 => {
                            None
                        }
                        _ => None
                    }
                }
                (_, x1, _, x3, _) if x1 == 2 && x3 == 0 => {
                    match ((instr >> 24) & 15, (instr >> 23) & 1, (instr >> 21) & 3, (instr >> 20) & 1, (instr >> 12) & 15, (instr >> 10) & 3, instr & 19) {
                        (_, x1, _, _, _, _, _) if x1 == 0 => {
                            let D = (instr >> 22) & 1;
                            let L = (instr >> 21) & 1;
                            let Rn = (instr >> 16) & 7;
                            let Vd = (instr >> 12) & 7;
                            let itype = (instr >> 8) & 7;
                            let size = (instr >> 6) & 3;
                            let align = (instr >> 4) & 3;
                            let Rm = instr & 7;
                            match (L, itype) {
                                (x0, x1) if x0 == 0 && x1 & 14 == 0 => {
                                    Some(Op::aarch32_VST4_m_T1A1_A)
                                }
                                (x0, x1) if x0 == 0 && x1 == 2 => {
                                    Some(Op::aarch32_VST1_m_T1A1_A)
                                }
                                (x0, x1) if x0 == 0 && x1 == 3 => {
                                    Some(Op::aarch32_VST2_m_T1A1_A)
                                }
                                (x0, x1) if x0 == 0 && x1 & 14 == 4 => {
                                    Some(Op::aarch32_VST3_m_T1A1_A)
                                }
                                (x0, x1) if x0 == 0 && x1 == 6 => {
                                    Some(Op::aarch32_VST1_m_T1A1_A)
                                }
                                (x0, x1) if x0 == 0 && x1 == 7 => {
                                    Some(Op::aarch32_VST1_m_T1A1_A)
                                }
                                (x0, x1) if x0 == 0 && x1 & 14 == 8 => {
                                    Some(Op::aarch32_VST2_m_T1A1_A)
                                }
                                (x0, x1) if x0 == 0 && x1 == 10 => {
                                    Some(Op::aarch32_VST1_m_T1A1_A)
                                }
                                (x0, x1) if x0 == 1 && x1 & 14 == 0 => {
                                    Some(Op::aarch32_VLD4_m_T1A1_A)
                                }
                                (x0, x1) if x0 == 1 && x1 == 2 => {
                                    Some(Op::aarch32_VLD1_m_T1A1_A)
                                }
                                (x0, x1) if x0 == 1 && x1 == 3 => {
                                    Some(Op::aarch32_VLD2_m_T1A1_A)
                                }
                                (x0, x1) if x0 == 1 && x1 & 14 == 4 => {
                                    Some(Op::aarch32_VLD3_m_T1A1_A)
                                }
                                (_, x1) if x1 == 11 => {
                                    None
                                }
                                (x0, x1) if x0 == 1 && x1 == 6 => {
                                    Some(Op::aarch32_VLD1_m_T1A1_A)
                                }
                                (x0, x1) if x0 == 1 && x1 == 7 => {
                                    Some(Op::aarch32_VLD1_m_T1A1_A)
                                }
                                (_, x1) if x1 & 12 == 12 => {
                                    None
                                }
                                (x0, x1) if x0 == 1 && x1 & 14 == 8 => {
                                    Some(Op::aarch32_VLD2_m_T1A1_A)
                                }
                                (x0, x1) if x0 == 1 && x1 == 10 => {
                                    Some(Op::aarch32_VLD1_m_T1A1_A)
                                }
                                _ => None
                            }
                        }
                        (_, x1, _, _, _, x5, _) if x1 == 1 && x5 == 3 => {
                            let D = (instr >> 22) & 1;
                            let L = (instr >> 21) & 1;
                            let Rn = (instr >> 16) & 7;
                            let Vd = (instr >> 12) & 7;
                            let N = (instr >> 8) & 3;
                            let size = (instr >> 6) & 3;
                            let T = (instr >> 5) & 1;
                            let a = (instr >> 4) & 1;
                            let Rm = instr & 7;
                            match (L, N, a) {
                                (x0, _, _) if x0 == 0 => {
                                    None
                                }
                                (x0, x1, _) if x0 == 1 && x1 == 0 => {
                                    Some(Op::aarch32_VLD1_a_T1A1_A)
                                }
                                (x0, x1, _) if x0 == 1 && x1 == 1 => {
                                    Some(Op::aarch32_VLD2_a_T1A1_A)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 == 0 => {
                                    Some(Op::aarch32_VLD3_a_T1A1_A)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 == 1 => {
                                    None
                                }
                                (x0, x1, _) if x0 == 1 && x1 == 3 => {
                                    Some(Op::aarch32_VLD4_a_T1A1_A)
                                }
                                _ => None
                            }
                        }
                        (_, x1, _, _, _, x5, _) if x1 == 1 && x5 != 3 => {
                            let D = (instr >> 22) & 1;
                            let L = (instr >> 21) & 1;
                            let Rn = (instr >> 16) & 7;
                            let Vd = (instr >> 12) & 7;
                            let size = (instr >> 10) & 3;
                            let N = (instr >> 8) & 3;
                            let index_align = (instr >> 4) & 7;
                            let Rm = instr & 7;
                            match (L, size, N, Rm) {
                                (x0, x1, x2, _) if x0 == 0 && x1 == 0 && x2 == 0 => {
                                    Some(Op::aarch32_VST1_1_T1A1_A)
                                }
                                (x0, x1, x2, _) if x0 == 0 && x1 == 0 && x2 == 1 => {
                                    Some(Op::aarch32_VST2_1_T1A1_A)
                                }
                                (x0, x1, x2, _) if x0 == 0 && x1 == 0 && x2 == 2 => {
                                    Some(Op::aarch32_VST3_1_T1A1_A)
                                }
                                (x0, x1, x2, _) if x0 == 0 && x1 == 0 && x2 == 3 => {
                                    Some(Op::aarch32_VST4_1_T1A1_A)
                                }
                                (x0, x1, x2, _) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    Some(Op::aarch32_VST1_1_T1A1_A)
                                }
                                (x0, x1, x2, _) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    Some(Op::aarch32_VST2_1_T1A1_A)
                                }
                                (x0, x1, x2, _) if x0 == 0 && x1 == 1 && x2 == 2 => {
                                    Some(Op::aarch32_VST3_1_T1A1_A)
                                }
                                (x0, x1, x2, _) if x0 == 0 && x1 == 1 && x2 == 3 => {
                                    Some(Op::aarch32_VST4_1_T2A2_A)
                                }
                                (x0, x1, x2, _) if x0 == 0 && x1 == 2 && x2 == 0 => {
                                    Some(Op::aarch32_VST1_1_T1A1_A)
                                }
                                (x0, x1, x2, _) if x0 == 0 && x1 == 2 && x2 == 1 => {
                                    Some(Op::aarch32_VST2_1_T1A1_A)
                                }
                                (x0, x1, x2, _) if x0 == 0 && x1 == 2 && x2 == 2 => {
                                    Some(Op::aarch32_VST3_1_T1A1_A)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 2 && x2 == 3 && x3 & 13 != 13 => {
                                    Some(Op::aarch32_VST4_1_T3A3_A)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 2 && x2 == 3 && x3 == 13 => {
                                    Some(Op::aarch32_VST4_1_T3A3_A)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 2 && x2 == 3 && x3 == 15 => {
                                    Some(Op::aarch32_VST4_1_T3A3_A)
                                }
                                (x0, x1, x2, _) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    Some(Op::aarch32_VLD1_1_T1A1_A)
                                }
                                (x0, x1, x2, _) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    Some(Op::aarch32_VLD2_1_T1A1_A)
                                }
                                (x0, x1, x2, _) if x0 == 1 && x1 == 0 && x2 == 2 => {
                                    Some(Op::aarch32_VLD3_1_T1A1_A)
                                }
                                (x0, x1, x2, _) if x0 == 1 && x1 == 0 && x2 == 3 => {
                                    Some(Op::aarch32_VLD4_1_T1A1_A)
                                }
                                (x0, x1, x2, _) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    Some(Op::aarch32_VLD1_1_T1A1_A)
                                }
                                (x0, x1, x2, _) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    Some(Op::aarch32_VLD2_1_T1A1_A)
                                }
                                (x0, x1, x2, _) if x0 == 1 && x1 == 1 && x2 == 2 => {
                                    Some(Op::aarch32_VLD3_1_T1A1_A)
                                }
                                (x0, x1, x2, _) if x0 == 1 && x1 == 1 && x2 == 3 => {
                                    Some(Op::aarch32_VLD4_1_T1A1_A)
                                }
                                (x0, x1, x2, _) if x0 == 1 && x1 == 2 && x2 == 0 => {
                                    Some(Op::aarch32_VLD1_1_T1A1_A)
                                }
                                (x0, x1, x2, _) if x0 == 1 && x1 == 2 && x2 == 1 => {
                                    Some(Op::aarch32_VLD2_1_T1A1_A)
                                }
                                (x0, x1, x2, _) if x0 == 1 && x1 == 2 && x2 == 2 => {
                                    Some(Op::aarch32_VLD3_1_T1A1_A)
                                }
                                (x0, x1, x2, _) if x0 == 1 && x1 == 2 && x2 == 3 => {
                                    Some(Op::aarch32_VLD4_1_T1A1_A)
                                }
                                _ => None
                            }
                        }
                        _ => None
                    }
                }
                (_, x1, _, x3, _) if x1 == 3 && x3 == 0 => {
                    None
                }
                _ => None
            }
        }
        _ => None
    }
} // end of decoding A32

#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    aarch32_SHA1SU0_A1_A,
    UQINCD_R_RS_X,
    ADR_Z_AZ_D_u32_scaled,
    INCH_R_RS__,
    aarch64_system_barriers_ssbb,
    aarch32_VSRA_T1A1_A,
    aarch32_LDRH_l_A1_A,
    UQDECD_R_RS_X,
    aarch64_integer_pac_autdb_dp_1src,
    aarch64_vector_arithmetic_unary_special_sqrt_est_float_simd,
    ST1B_Z_P_BR__,
    aarch32_VCNT_T1A1_A,
    ANDV_R_P_Z__,
    aarch32_BL_i_A1_A,
    aarch64_vector_shift_conv_int_sisd,
    SUB_Z_P_ZZ__,
    aarch64_memory_pair_simdfp_offset,
    MUL_Z_ZI__,
    aarch64_float_arithmetic_mul_add_sub,
    aarch32_VRADDHN_T1A1_A,
    aarch32_STLEXH_A1_A,
    aarch32_ADD_rr_A1_A,
    aarch64_integer_tags_mcsettaganddatapairpost,
    aarch64_vector_arithmetic_binary_uniform_max_min_fp16_1985,
    CMPNE_P_P_ZW__,
    aarch32_MRRC_T1A1_A,
    FRINTA_Z_P_Z__,
    aarch64_vector_crypto_sm3_sm3tt2b,
    LD4B_Z_P_BI_Contiguous,
    ZIP2_P_PP__,
    aarch64_integer_arithmetic_mul_uniform_add_sub,
    aarch32_VABA_T2A2_A,
    LDFF1B_Z_P_BR_U32,
    aarch32_LDAEXD_A1_A,
    ORV_R_P_Z__,
    ST3D_Z_P_BI_Contiguous,
    aarch64_memory_pair_simdfp_no_alloc,
    LD2H_Z_P_BI_Contiguous,
    aarch32_VUZP_T1A1_A,
    aarch64_vector_crypto_sha2op_sha256_sched0,
    aarch64_memory_literal_general,
    aarch64_vector_cvt_bf16_scalar,
    aarch32_VORR_i_T1A1_A,
    MOVPRFX_Z_P_Z__,
    aarch32_VDOT_bf16_A1_A,
    ST1W_Z_P_BZ_S_x32_scaled,
    aarch32_STM_u_A1_AS,
    LD3H_Z_P_BR_Contiguous,
    aarch32_RSB_rr_A1_A,
    aarch64_system_exceptions_debug_halt,
    aarch32_VSWP_T1A1_A,
    ST1B_Z_P_BZ_S_x32_unscaled,
    aarch64_vector_shift_left_sat_simd,
    CLASTB_V_P_Z__,
    aarch64_vector_arithmetic_unary_special_sqrt,
    DUP_Z_Zi__,
    aarch64_integer_tags_mcsettagandzerodata,
    FMULX_Z_P_ZZ__,
    UQDECB_R_RS_UW,
    FADD_Z_P_ZS__,
    WHILELS_P_P_RR__,
    CNTP_R_P_P__,
    aarch32_STREXD_A1_A,
    PTRUES_P_S__,
    aarch64_vector_arithmetic_unary_special_sqrt_fp16,
    aarch32_VZIP_T1A1_A,
    FRINTI_Z_P_Z__,
    UQINCD_R_RS_UW,
    CPY_Z_P_V__,
    REVH_Z_Z__,
    aarch64_vector_crypto_sha512_sha512h,
    STR_Z_BI__,
    FMINV_V_P_Z__,
    LDFF1W_Z_P_BR_U64,
    aarch32_SHA256SU1_A1_A,
    SUBR_Z_ZI__,
    LSRR_Z_P_ZZ__,
    aarch32_MVN_r_A1_A,
    aarch32_USAD8_A1_A,
    aarch32_LDREXD_A1_A,
    aarch32_SHA1P_A1_A,
    aarch32_VCVT_is_A1_A,
    aarch64_vector_arithmetic_binary_uniform_mul_fp_extended_sisd,
    FMLA_Z_ZZZi_D,
    aarch32_ESB_A1_A,
    aarch32_UMULL_A1_A,
    LDFF1SB_Z_P_BR_S64,
    aarch32_LDRSBT_A2_A,
    aarch32_SMLABB_A1_A,
    aarch32_LDRHT_A2_A,
    aarch32_VQRDMLSH_A2_A,
    aarch32_PLD_r_A1_A,
    LDFF1SH_Z_P_BZ_D_x32_scaled,
    aarch32_SHA256H2_A1_A,
    aarch32_SHA1SU1_A1_A,
    BRKPA_P_P_PP__,
    aarch64_vector_transfer_vector_insert,
    aarch32_VMLA_s_T2A2_A,
    aarch64_vector_arithmetic_binary_disparate_add_sub_narrow,
    aarch32_VABS_A1_A,
    aarch32_STRBT_A2_A,
    aarch64_vector_arithmetic_unary_float_round,
    LD1SB_Z_P_BI_S16,
    PRFW_I_P_AI_D,
    aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd,
    aarch32_ADD_r_A1_A,
    aarch64_memory_single_general_immediate_signed_post_idx,
    aarch64_vector_crypto_sm3_sm3tt1a,
    aarch32_VST4_1_T2A2_A,
    FCVTZS_Z_P_Z_D2W,
    LD1D_Z_P_AI_D,
    LDFF1W_Z_P_BZ_D_64_unscaled,
    LD1SH_Z_P_BI_S32,
    ADDVL_R_RI__,
    aarch32_VBIC_i_T1A1_A,
    aarch32_VRSHL_T1A1_A,
    aarch64_vector_arithmetic_unary_fp16_conv_int_sisd,
    ZIP1_Z_ZZ__,
    aarch32_VST3_m_T1A1_A,
    PRFB_I_P_AI_D,
    aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd,
    aarch32_UHASX_A1_A,
    FNMLA_Z_P_ZZZ__,
    aarch32_SB_A1_A,
    ST1D_Z_P_BZ_D_x32_scaled,
    aarch64_integer_tags_mcsettagpair,
    BRKA_P_P_P__,
    aarch32_CRC32_A1_A,
    aarch64_vector_arithmetic_binary_uniform_mul_int_accum,
    UABD_Z_P_ZZ__,
    CMPHS_P_P_ZW__,
    aarch32_VSHR_T1A1_A,
    aarch32_VRSRA_T1A1_A,
    FRSQRTS_Z_ZZ__,
    LD1RQW_Z_P_BR_Contiguous,
    aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd,
    aarch32_VQRSHL_T1A1_A,
    LDFF1W_Z_P_BZ_D_x32_unscaled,
    aarch32_QSUB_A1_A,
    ST3H_Z_P_BI_Contiguous,
    LDFF1SB_Z_P_BZ_D_64_unscaled,
    aarch64_memory_single_general_immediate_signed_offset_lda_stl,
    aarch32_VQRSHRN_T1A1_A,
    aarch64_integer_flags_rmif,
    aarch64_vector_crypto_sha3_eor3,
    aarch32_VFMA_bf_A1_A,
    aarch64_system_exceptions_debug_exception,
    STNT1D_Z_P_BR_Contiguous,
    aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd,
    aarch32_LDAB_A1_A,
    CMPHS_P_P_ZI__,
    aarch64_integer_conditional_select,
    LDFF1D_Z_P_AI_D,
    FCMUO_P_P_ZZ__,
    BRKN_P_P_PP__,
    INCB_R_RS__,
    aarch32_STRD_i_A1_A,
    ST1D_Z_P_BZ_D_x32_unscaled,
    aarch32_VSUB_f_A1_A,
    aarch64_vector_arithmetic_binary_uniform_rsqrts_fp16_sisd,
    aarch32_LDM_e_A1_AS,
    aarch32_UQADD16_A1_A,
    LD1SH_Z_P_BZ_D_64_scaled,
    LD1RB_Z_P_BI_U16,
    aarch64_vector_arithmetic_binary_disparate_mul_double_sisd,
    aarch64_vector_arithmetic_unary_diff_neg_fp16,
    aarch64_vector_arithmetic_binary_element_mat_mul_int_dotp,
    LD1SH_Z_P_BZ_D_x32_unscaled,
    aarch64_system_barriers_dsb,
    aarch32_VRSUBHN_T1A1_A,
    aarch32_LDC_i_T1A1_A,
    CNT_Z_P_Z__,
    aarch32_VMRS_T1A1_AS,
    ST2B_Z_P_BR_Contiguous,
    UDIV_Z_P_ZZ__,
    aarch32_VRECPS_A1_A,
    CMPNE_P_P_ZI__,
    aarch64_float_arithmetic_add_sub,
    FCMGT_P_P_ZZ__,
    aarch64_vector_shift_right_narrow_nonuniform_simd,
    aarch32_VSEL_A1_A,
    NOT_Z_P_Z__,
    LD1RSW_Z_P_BI_S64,
    aarch32_VMOV_rs_T1A1_A,
    REVW_Z_Z__,
    ST1D_Z_P_BR__,
    aarch64_system_monitors,
    aarch32_UXTAB_A1_A,
    aarch32_SEVL_A1_A,
    aarch32_BLX_r_A1_A,
    aarch32_B_A1_A,
    UDIVR_Z_P_ZZ__,
    aarch64_memory_single_simdfp_immediate_signed_pre_idx,
    aarch32_VCVTB_bf16_T1A1_A,
    aarch32_LDRB_i_A1_A,
    aarch64_vector_arithmetic_binary_disparate_mul_poly,
    FCMGT_P_P_Z0__,
    aarch32_STRB_i_A1_A,
    CLASTA_R_P_Z__,
    USDOT_Z_ZZZi_S,
    UQDECH_R_RS_X,
    LD1H_Z_P_AI_D,
    aarch64_integer_tags_mcsettaganddatapair,
    aarch32_ADC_r_A1_A,
    aarch32_ADD_SP_i_A1_A,
    aarch64_integer_conditional_compare_register,
    aarch32_SXTAH_A1_A,
    aarch64_vector_crypto_sha2op_sha1_hash,
    aarch32_VMUL_s_T2A2_A,
    aarch32_VSHRN_T1A1_A,
    UQINCB_R_RS_X,
    aarch32_REV16_A1_A,
    aarch32_VINS_A1_A,
    aarch32_VSLI_T1A1_A,
    aarch64_vector_arithmetic_binary_disparate_diff,
    aarch64_float_arithmetic_div,
    ST1W_Z_P_BR__,
    DUPM_Z_I__,
    aarch64_vector_arithmetic_unary_rbit,
    aarch32_VORN_r_T1A1_A,
    aarch32_VRINTZ_vfp_A1_A,
    SQINCH_R_RS_X,
    aarch32_BFC_A1_A,
    LDFF1H_Z_P_BR_U16,
    LD1B_Z_P_AI_D,
    LD1H_Z_P_BZ_D_64_scaled,
    TRN1_Z_ZZ__,
    aarch64_vector_arithmetic_binary_uniform_div_fp16,
    NEG_Z_P_Z__,
    UQDECH_R_RS_UW,
    aarch32_VMVN_i_T1A1_A,
    aarch64_vector_arithmetic_unary_float_narrow,
    aarch32_UHADD16_A1_A,
    aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd,
    aarch32_SHADD16_A1_A,
    ST4W_Z_P_BR_Contiguous,
    aarch32_SBC_rr_A1_A,
    aarch64_vector_arithmetic_binary_disparate_add_sub_long,
    CMPLE_P_P_ZI__,
    aarch32_TST_rr_A1_A,
    aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd,
    aarch32_AND_rr_A1_A,
    aarch64_memory_pair_simdfp_post_idx,
    SDOT_Z_ZZZi_S,
    ST2D_Z_P_BI_Contiguous,
    aarch64_integer_tags_mcsettagandzeroarray,
    aarch32_SEV_A1_A,
    aarch32_UHSAX_A1_A,
    LD1B_Z_P_BR_U32,
    SQADD_Z_ZI__,
    FMAXV_V_P_Z__,
    aarch32_VABA_T1A1_A,
    PRFW_I_P_BZ_D_64_scaled,
    CMPHI_P_P_ZI__,
    aarch64_memory_atomicops_cas_single,
    aarch32_VRSQRTE_A1_A,
    ASR_Z_P_ZI__,
    aarch32_SHSUB8_A1_A,
    aarch32_SHA1H_A1_A,
    FDUP_Z_I__,
    LDFF1B_Z_P_AI_D,
    LD1B_Z_P_BI_U8,
    aarch64_vector_arithmetic_binary_uniform_add_fp,
    LD3W_Z_P_BR_Contiguous,
    aarch64_vector_arithmetic_unary_special_frecpx,
    ST1B_Z_P_BI__,
    aarch64_integer_tags_mcsubtag,
    aarch64_integer_pac_pacia_hint,
    SQINCB_R_RS_SX,
    aarch32_SMULBB_A1_A,
    FADDV_V_P_Z__,
    FMUL_Z_ZZi_D,
    aarch32_UHSUB16_A1_A,
    REVB_Z_Z__,
    aarch64_vector_shift_left_simd,
    LSR_Z_ZW__,
    LD1RB_Z_P_BI_U32,
    ST1H_Z_P_BI__,
    aarch64_vector_crypto_sha3_rax1,
    CMPLS_P_P_ZI__,
    aarch32_VABD_f_A1_A,
    ORRS_P_P_PP_Z,
    aarch64_vector_arithmetic_binary_uniform_mul_fp16_product,
    ST1W_Z_P_BZ_S_x32_unscaled,
    ORN_P_P_PP_Z,
    ST3H_Z_P_BR_Contiguous,
    aarch64_memory_literal_simdfp,
    aarch64_system_barriers_dmb,
    aarch64_float_convert_int,
    aarch32_VDOT_bf16_i_A1_A,
    aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd,
    aarch32_VSHL_r_T1A1_A,
    ST2B_Z_P_BI_Contiguous,
    CMPGE_P_P_ZW__,
    aarch64_vector_reduce_fp16_maxnm_simd,
    aarch64_float_move_fp_select,
    aarch64_vector_transfer_vector_permute_transpose,
    aarch64_integer_tags_mcinsertrandomtag,
    aarch64_vector_arithmetic_binary_uniform_add_fp16,
    aarch32_UDIV_A1_A,
    BFMLALT_Z_ZZZi__,
    aarch32_VTST_T1A1_A,
    LD1SH_Z_P_BZ_D_x32_scaled,
    aarch64_system_barriers_isb,
    aarch32_VLD4_a_T1A1_A,
    aarch32_VLD2_a_T1A1_A,
    SQSUB_Z_ZZ__,
    aarch32_VMLA_f_A1_A,
    LD1RSB_Z_P_BI_S64,
    aarch32_VPADAL_T1A1_A,
    aarch32_EOR_rr_A1_A,
    aarch64_vector_crypto_sm3_sm3partw2,
    BICS_P_P_PP_Z,
    aarch32_VQRDMULH_T2A2_A,
    aarch64_vector_arithmetic_binary_element_mul_acc_fp16_simd,
    ASRR_Z_P_ZZ__,
    SQDECH_Z_ZS__,
    LDFF1H_Z_P_BZ_S_x32_unscaled,
    aarch32_QDSUB_A1_A,
    SCVTF_Z_P_Z_H2FP16,
    aarch32_QSUB16_A1_A,
    LD1RSB_Z_P_BI_S16,
    aarch64_vector_arithmetic_binary_uniform_add_halving_truncating,
    aarch32_LDRSBT_A1_A,
    aarch32_LDRT_A1_A,
    FDIVR_Z_P_ZZ__,
    aarch64_integer_tags_mcgettag,
    FCVTZS_Z_P_Z_FP162W,
    FCVT_Z_P_Z_H2D,
    aarch32_VMUL_s_A1_A,
    aarch32_VLD2_m_T1A1_A,
    FSUB_Z_P_ZS__,
    ST1D_Z_P_AI_D,
    DECD_R_RS__,
    aarch64_vector_arithmetic_binary_uniform_recps_sisd,
    SXTH_Z_P_Z__,
    CMPEQ_P_P_ZI__,
    aarch64_integer_arithmetic_mul_widening_32_64,
    CPY_Z_O_I__,
    aarch64_float_convert_fix,
    aarch64_integer_arithmetic_add_sub_immediate,
    aarch32_VQNEG_T1A1_A,
    aarch64_vector_arithmetic_unary_extract_sat_simd,
    ST1W_Z_P_BZ_D_x32_scaled,
    aarch32_UADD8_A1_A,
    aarch64_integer_bitfield,
    LD1RSB_Z_P_BI_S32,
    SDIVR_Z_P_ZZ__,
    BFMLALB_Z_ZZZ__,
    aarch64_memory_pair_simdfp_pre_idx,
    UMIN_Z_P_ZZ__,
    ASR_Z_P_ZZ__,
    FCMLT_P_P_Z0__,
    FMAXNMV_V_P_Z__,
    SUBR_Z_P_ZZ__,
    aarch64_vector_crypto_sha3op_sha1_hash_parity,
    aarch32_PLD_l_A1_A,
    FADD_Z_ZZ__,
    LDFF1B_Z_P_BR_U8,
    aarch64_vector_transfer_integer_insert,
    LD1SB_Z_P_AI_D,
    LDNF1SB_Z_P_BI_S16,
    FCMLA_Z_ZZZi_S,
    aarch32_MVN_rr_A1_A,
    TRN2_Z_ZZ__,
    BFCVT_Z_P_Z_S2BF,
    aarch64_integer_pac_autib_hint,
    aarch64_branch_conditional_test,
    aarch32_MRS_A1_AS,
    aarch32_BKPT_A1_A,
    LDFF1SH_Z_P_AI_D,
    aarch32_LDRD_l_A1_A,
    aarch32_VMOV_sr_T1A1_A,
    FCMGE_P_P_Z0__,
    aarch64_vector_arithmetic_binary_uniform_diff,
    aarch64_integer_ins_ext_insert_movewide,
    aarch32_LDRSH_r_A1_A,
    UDOT_Z_ZZZi_S,
    aarch32_VFMAL_A1_A,
    aarch64_vector_arithmetic_binary_element_mul_acc_mul_norounding_i_upper,
    aarch64_integer_arithmetic_add_sub_shiftedreg,
    LD1SB_Z_P_BZ_D_x32_unscaled,
    ADD_Z_ZI__,
    aarch64_vector_arithmetic_binary_uniform_add_fp_complex,
    aarch64_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_lower,
    aarch32_CMN_rr_A1_A,
    ST4D_Z_P_BR_Contiguous,
    LD1SH_Z_P_AI_S,
    aarch64_vector_arithmetic_binary_uniform_logical_bsl_eor,
    WHILELO_P_P_RR__,
    aarch32_YIELD_A1_A,
    FCVTZU_Z_P_Z_FP162H,
    FSCALE_Z_P_ZZ__,
    aarch64_vector_arithmetic_binary_element_mul_high_sisd,
    REV_P_P__,
    LD3D_Z_P_BR_Contiguous,
    aarch64_integer_arithmetic_div,
    aarch64_vector_arithmetic_unary_float_widen,
    aarch32_ORR_rr_A1_A,
    SQDECW_Z_ZS__,
    aarch64_vector_arithmetic_unary_special_sqrt_est_fp16_simd,
    aarch32_BIC_rr_A1_A,
    LD1SH_Z_P_BI_S64,
    UCVTF_Z_P_Z_X2D,
    LDNF1W_Z_P_BI_U32,
    aarch64_vector_crypto_sha3op_sha256_sched1,
    aarch64_integer_tags_mcgettagarray,
    aarch32_SMLALD_A1_A,
    CLASTA_V_P_Z__,
    aarch32_AND_r_A1_A,
    LD1RQD_Z_P_BR_Contiguous,
    aarch64_integer_tags_mcinserttagmask,
    MAD_Z_P_ZZZ__,
    aarch64_vector_arithmetic_unary_diff_neg_int_simd,
    LDFF1SH_Z_P_BZ_D_64_unscaled,
    CNTW_R_S__,
    CMPNE_P_P_ZZ__,
    ST1H_Z_P_BZ_D_64_scaled,
    aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_sisd,
    FADD_Z_P_ZZ__,
    ST1H_Z_P_BR__,
    aarch32_VNEG_A1_A,
    LD1ROW_Z_P_BI_U32,
    UZP2_P_PP__,
    LSL_Z_P_ZW__,
    aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd,
    RDFFR_P_F__,
    aarch32_SMLSD_A1_A,
    LSLR_Z_P_ZZ__,
    aarch32_HVC_A1_A,
    AND_Z_ZI__,
    SETFFR_F__,
    FCVT_Z_P_Z_S2D,
    LD1SB_Z_P_BI_S32,
    aarch32_SMMLA_A1_A,
    ST1D_Z_P_BZ_D_64_unscaled,
    UZP2_Z_ZZ__,
    LD1ROD_Z_P_BI_U64,
    UQINCD_Z_ZS__,
    aarch32_VFMA_bfs_A1_A,
    aarch32_VQDMLAL_T1A1_A,
    aarch32_PLI_r_A1_A,
    aarch64_vector_arithmetic_binary_disparate_mul_dmacc_simd,
    LDFF1H_Z_P_BZ_D_x32_unscaled,
    UQINCW_R_RS_X,
    aarch64_branch_unconditional_dret,
    aarch32_LDRSHT_A2_A,
    CNTD_R_S__,
    aarch64_vector_transfer_vector_permute_unzip,
    aarch64_memory_single_general_immediate_signed_offset_normal,
    aarch32_SHA256SU0_A1_A,
    ORR_P_P_PP_Z,
    aarch64_vector_arithmetic_binary_disparate_mul_dmacc_sisd,
    SQDECD_Z_ZS__,
    ASR_Z_ZW__,
    FCMNE_P_P_Z0__,
    aarch32_SUB_r_A1_A,
    aarch32_VADDL_T1A1_A,
    aarch64_vector_arithmetic_binary_element_mul_acc_double_simd,
    FMINNMV_V_P_Z__,
    ADR_Z_AZ_SD_same_scaled,
    SQDECD_R_RS_SX,
    PRFW_I_P_BZ_D_x32_scaled,
    aarch32_VCADD_A1_A,
    aarch32_VCMLA_idx_A1_A,
    LDNT1H_Z_P_BR_Contiguous,
    aarch32_VSTM_T2A2_A,
    ADR_Z_AZ_D_s32_scaled,
    LD1SB_Z_P_BR_S64,
    ZIP2_Z_ZZ_Q,
    LDNT1B_Z_P_BI_Contiguous,
    aarch64_vector_reduce_fp_maxnm_simd,
    ST1W_Z_P_BI__,
    FRINTP_Z_P_Z__,
    FNEG_Z_P_Z__,
    aarch64_system_register_system,
    LDNF1SH_Z_P_BI_S32,
    aarch64_vector_crypto_sha512_sha512su1,
    aarch64_memory_single_general_immediate_signed_pac,
    UMULH_Z_P_ZZ__,
    aarch32_LDRD_i_A1_A,
    SQADD_Z_ZZ__,
    LD1D_Z_P_BR_U64,
    BIC_P_P_PP_Z,
    aarch32_VQSUB_T1A1_A,
    LDFF1SB_Z_P_AI_S,
    LD1B_Z_P_BZ_D_x32_unscaled,
    TBL_Z_ZZ_1,
    LD1SH_Z_P_BZ_D_64_unscaled,
    SMIN_Z_ZI__,
    FMLS_Z_ZZZi_H,
    aarch32_VSQRT_A1_A,
    STNT1B_Z_P_BI_Contiguous,
    RBIT_Z_P_Z__,
    aarch32_VSHL_i_T1A1_A,
    LDFF1B_Z_P_BR_U64,
    aarch32_VST1_1_T1A1_A,
    AND_Z_ZZ__,
    STR_P_BI__,
    FMLA_Z_ZZZi_S,
    LDFF1B_Z_P_AI_S,
    aarch32_VMUL_i_A2_A,
    LDNT1W_Z_P_BR_Contiguous,
    LDFF1B_Z_P_BR_U16,
    LD1RSH_Z_P_BI_S32,
    LD4D_Z_P_BR_Contiguous,
    ST1H_Z_P_AI_D,
    aarch32_ERET_A1_A,
    aarch32_VFMA_A1_A,
    LDNF1H_Z_P_BI_U64,
    FSUB_Z_ZZ__,
    aarch64_vector_arithmetic_unary_float_conv_int_sisd,
    aarch32_AESMC_A1_A,
    ST4H_Z_P_BR_Contiguous,
    aarch32_STLEXB_A1_A,
    aarch64_vector_arithmetic_binary_uniform_max_min_fp_1985,
    FNMAD_Z_P_ZZZ__,
    FCADD_Z_P_ZZ__,
    aarch32_STL_A1_A,
    DUP_Z_R__,
    aarch64_vector_arithmetic_unary_cnt,
    aarch64_integer_arithmetic_add_sub_carry,
    aarch64_vector_arithmetic_unary_special_recip_float_sisd,
    aarch32_REVSH_A1_A,
    LD1RQD_Z_P_BI_U64,
    NORS_P_P_PP_Z,
    aarch64_vector_arithmetic_unary_float_conv_float_bulk_simd,
    aarch64_udf,
    LDFF1SW_Z_P_BZ_D_x32_unscaled,
    aarch32_CLZ_A1_A,
    aarch64_vector_crypto_sm3_sm3ss1,
    INCW_Z_ZS__,
    CTERMNE_RR__,
    SMULH_Z_P_ZZ__,
    aarch64_vector_reduce_fp16_maxnm_sisd,
    FCVTZS_Z_P_Z_FP162X,
    aarch32_LDRHT_A1_A,
    UQINCB_R_RS_UW,
    aarch32_VREV16_T1A1_A,
    aarch32_VLDR_A1_A,
    aarch32_MRC_T1A1_A,
    PRFB_I_P_BZ_D_x32_scaled,
    Nop,
    aarch64_memory_exclusive_pair,
    INDEX_Z_RI__,
    UQDECD_Z_ZS__,
    aarch32_VMLA_s_A1_A,
    aarch32_SHA256H_A1_A,
    ST1H_Z_P_AI_S,
    aarch32_CSDB_A1_A,
    aarch64_vector_crypto_sm4_sm4enc,
    aarch64_memory_exclusive_single,
    aarch64_memory_pair_general_offset,
    TRN2_Z_ZZ_Q,
    LD1B_Z_P_BZ_D_64_unscaled,
    LDFF1H_Z_P_BR_U32,
    aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd,
    aarch64_integer_tags_mcsettag,
    aarch64_vector_arithmetic_binary_element_mul_fp16_sisd,
    UQDECP_R_P_R_X,
    aarch64_vector_crypto_sm3_sm3tt1b,
    LD1H_Z_P_AI_S,
    PFIRST_P_P_P__,
    INCD_R_RS__,
    LDFF1W_Z_P_BR_U32,
    ZIP1_P_PP__,
    aarch32_AND_i_A1_A,
    aarch64_vector_arithmetic_unary_diff_neg_float,
    aarch64_memory_atomicops_swp,
    LD4H_Z_P_BR_Contiguous,
    DECB_R_RS__,
    aarch64_system_sysops,
    aarch64_vector_arithmetic_binary_element_mul_fp_simd,
    PRFB_I_P_BI_S,
    FMIN_Z_P_ZS__,
    aarch32_VTBL_T1A1_A,
    LDFF1SH_Z_P_BR_S64,
    UQINCW_Z_ZS__,
    UZP1_P_PP__,
    aarch64_vector_arithmetic_binary_uniform_max_min_fp16_2008,
    LD2B_Z_P_BR_Contiguous,
    LDFF1H_Z_P_BR_U64,
    aarch64_vector_arithmetic_binary_element_mul_fp_sisd,
    aarch64_integer_tags_mcsettagandzerodatapre,
    aarch32_SEL_A1_A,
    LDNF1B_Z_P_BI_U32,
    aarch64_vector_arithmetic_unary_cmp_float_bulk_simd,
    SQDECW_R_RS_SX,
    aarch64_system_exceptions_runtime_smc,
    LDFF1D_Z_P_BZ_D_64_scaled,
    LD1W_Z_P_BI_U32,
    LDNF1SB_Z_P_BI_S64,
    USDOT_Z_ZZZ_S,
    aarch32_VUSDOT_A1_A,
    aarch32_ADC_rr_A1_A,
    aarch64_integer_pac_autda_dp_1src,
    LD3D_Z_P_BI_Contiguous,
    aarch32_SSAT_A1_A,
    LD4H_Z_P_BI_Contiguous,
    aarch32_VST4_m_T1A1_A,
    aarch32_SRS_A1_AS,
    FCMLA_Z_ZZZi_H,
    aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags,
    ST4B_Z_P_BR_Contiguous,
    LDFF1B_Z_P_BZ_D_x32_unscaled,
    LDFF1SB_Z_P_AI_D,
    UQADD_Z_ZI__,
    SUNPKHI_Z_Z__,
    FMUL_Z_P_ZZ__,
    aarch64_memory_single_general_immediate_signed_pre_idx,
    UCVTF_Z_P_Z_W2S,
    SQDECW_R_RS_X,
    aarch32_MLS_A1_A,
    aarch32_VMOV_i_T1A1_A,
    aarch32_VST2_m_T1A1_A,
    aarch32_SSUB16_A1_A,
    aarch32_ADD_i_A1_A,
    aarch64_integer_tags_mcsettagarray,
    FCMEQ_P_P_ZZ__,
    LD1H_Z_P_BR_U64,
    LD1W_Z_P_BZ_D_64_unscaled,
    aarch32_LDM_u_A1_AS,
    aarch32_VPMAX_f_A1_A,
    INCH_Z_ZS__,
    FMIN_Z_P_ZZ__,
    aarch32_VHADD_T1A1_A,
    ST2W_Z_P_BI_Contiguous,
    aarch32_UHADD8_A1_A,
    aarch64_vector_arithmetic_binary_element_bfdot,
    aarch64_vector_arithmetic_binary_disparate_add_sub_wide,
    aarch64_integer_arithmetic_address_pc_rel,
    aarch64_vector_arithmetic_binary_uniform_recps_simd,
    SMIN_Z_P_ZZ__,
    AND_P_P_PP_Z,
    CMPEQ_P_P_ZZ__,
    UCVTF_Z_P_Z_W2FP16,
    FMINNM_Z_P_ZS__,
    MOVPRFX_Z_Z__,
    aarch64_vector_arithmetic_binary_uniform_recps_fp16_simd,
    aarch64_vector_arithmetic_binary_uniform_max_min_pair,
    aarch64_integer_tags_mcsettagpost,
    aarch64_vector_crypto_sm3_sm3partw1,
    aarch32_VQDMULH_T2A2_A,
    aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd,
    WHILELE_P_P_RR__,
    aarch64_integer_tags_mcaddtag,
    aarch64_vector_shift_left_long,
    aarch64_vector_arithmetic_unary_cmp_int_bulk_simd,
    aarch32_LDRH_r_A1_A,
    SQINCW_R_RS_X,
    aarch32_UADD16_A1_A,
    REV_Z_Z__,
    PRFB_I_P_BZ_S_x32_scaled,
    FRINTM_Z_P_Z__,
    TRN1_Z_ZZ_Q,
    aarch64_vector_arithmetic_binary_element_mul_acc_long,
    aarch32_RSB_i_A1_A,
    DECD_Z_ZS__,
    aarch32_LDC_l_T1A1_A,
    PUNPKLO_P_P__,
    PRFD_I_P_BZ_S_x32_scaled,
    aarch32_VSUBL_T1A1_A,
    aarch64_vector_arithmetic_unary_extract_sqxtun_sisd,
    SQDECP_R_P_R_SX,
    LD1H_Z_P_BR_U32,
    aarch64_vector_arithmetic_unary_fp16_round,
    aarch32_VCGT_r_T1A1_A,
    LD1W_Z_P_AI_S,
    aarch32_VQRDMLAH_A1_A,
    aarch32_DOT_A1_A,
    LD1ROW_Z_P_BR_Contiguous,
    aarch32_VRECPE_A1_A,
    aarch32_QADD16_A1_A,
    LDFF1W_Z_P_BZ_D_x32_scaled,
    aarch32_VCVTB_T1A1_A,
    aarch64_vector_shift_right_narrow_logical,
    LD1SW_Z_P_BZ_D_64_unscaled,
    aarch64_vector_arithmetic_binary_uniform_rsqrts_simd,
    UZP1_Z_ZZ__,
    LD3B_Z_P_BI_Contiguous,
    aarch64_vector_crypto_sha3_bcax,
    CMPHS_P_P_ZZ__,
    PRFW_I_P_BI_S,
    LDNT1B_Z_P_BR_Contiguous,
    LD1D_Z_P_BZ_D_x32_unscaled,
    UXTH_Z_P_Z__,
    aarch64_vector_arithmetic_binary_uniform_mul_fp_extended_simd,
    DECW_Z_ZS__,
    FCMLA_Z_P_ZZZ__,
    aarch64_vector_shift_right_insert_sisd,
    LD4D_Z_P_BI_Contiguous,
    aarch32_SSAX_A1_A,
    aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd,
    aarch32_SMULL_A1_A,
    aarch32_VNMLA_A2_A,
    LD1B_Z_P_BI_U32,
    aarch32_SHASX_A1_A,
    aarch64_integer_arithmetic_cnt,
    aarch64_vector_arithmetic_binary_uniform_mul_fp_fused,
    FEXPA_Z_Z__,
    PRFW_I_P_BR_S,
    FCVTZU_Z_P_Z_D2W,
    aarch64_vector_arithmetic_binary_disparate_mul_accum,
    aarch32_EOR_i_A1_A,
    aarch64_vector_arithmetic_binary_uniform_mul_acc_bf16_long,
    aarch32_VLD1_m_T1A1_A,
    aarch32_LDMDA_A1_A,
    aarch64_vector_arithmetic_unary_float_conv_float_bulk_sisd,
    aarch64_vector_fp16_movi,
    aarch32_VST4_1_T3A3_A,
    aarch64_vector_arithmetic_unary_diff_neg_sat_simd,
    aarch64_vector_arithmetic_unary_clsz,
    NANDS_P_P_PP_Z,
    aarch32_SHADD8_A1_A,
    aarch32_VPADD_f_A1_A,
    DUP_Z_I__,
    aarch64_vector_arithmetic_unary_special_frecpx_fp16,
    FNMLS_Z_P_ZZZ__,
    UQINCP_R_P_R_X,
    aarch64_vector_arithmetic_unary_fp16_conv_float_tieaway_simd,
    DECP_R_P_R__,
    aarch32_VCLE_i_A1_A,
    SQINCP_R_P_R_SX,
    BFCVTNT_Z_P_Z_S2BF,
    UQINCH_R_RS_UW,
    CMPGT_P_P_ZW__,
    UDOT_Z_ZZZi_D,
    AND_Z_P_ZZ__,
    SQINCH_Z_ZS__,
    MLS_Z_P_ZZZ__,
    MSB_Z_P_ZZZ__,
    aarch32_STREXB_A1_A,
    aarch32_ISB_A1_A,
    aarch64_vector_arithmetic_binary_uniform_add_halving_rounding,
    SMAXV_R_P_Z__,
    aarch32_LDAEX_A1_A,
    BRKB_P_P_P__,
    ASR_Z_ZI__,
    LD1D_Z_P_BZ_D_x32_scaled,
    ST1W_Z_P_AI_S,
    aarch64_integer_tags_mcsettagandzerodatapost,
    aarch32_VSTM_T1A1_A,
    aarch32_SMMUL_A1_A,
    aarch32_BFI_A1_A,
    aarch32_VADD_f_A2_A,
    aarch64_vector_arithmetic_binary_uniform_mul_int_product,
    FADDA_V_P_Z__,
    LD3B_Z_P_BR_Contiguous,
    aarch32_MOV_rr_A1_A,
    aarch64_vector_crypto_sha3op_sha256_hash,
    FMSB_Z_P_ZZZ__,
    CMPLO_P_P_ZW__,
    FMLA_Z_P_ZZZ__,
    ST1D_Z_P_BZ_D_64_scaled,
    aarch32_SDIV_A1_A,
    aarch32_SETPAN_A1_A,
    TRN1_P_PP__,
    LDNF1SW_Z_P_BI_S64,
    SADDV_R_P_Z__,
    aarch32_STMDA_A1_A,
    SEL_Z_P_ZZ__,
    aarch64_vector_arithmetic_binary_element_mul_acc_complex,
    aarch64_system_register_cpsr,
    LD2D_Z_P_BI_Contiguous,
    SQINCD_R_RS_SX,
    FCPY_Z_P_I__,
    aarch32_SBFX_A1_A,
    aarch64_integer_logical_shiftedreg,
    PRFD_I_P_BZ_D_x32_scaled,
    aarch64_vector_arithmetic_unary_shift,
    ANDS_P_P_PP_Z,
    aarch32_SXTAB_A1_A,
    aarch32_BXJ_A1_A,
    FACGT_P_P_ZZ__,
    LD1RQW_Z_P_BI_U32,
    aarch32_SSUB8_A1_A,
    LD1H_Z_P_BZ_S_x32_scaled,
    aarch32_STLEX_A1_A,
    aarch32_SMLAWB_A1_A,
    CMPGE_P_P_ZI__,
    aarch64_vector_shift_conv_int_simd,
    LDR_P_BI__,
    aarch32_VQSHL_i_T1A1_A,
    SQINCD_R_RS_X,
    aarch32_VLD3_a_T1A1_A,
    aarch32_VPMAX_i_T1A1_A,
    aarch64_vector_crypto_sha512_sha512h2,
    aarch64_memory_single_simdfp_immediate_signed_post_idx,
    aarch32_VEXT_T1A1_A,
    aarch32_SUB_i_A1_A,
    LD1W_Z_P_AI_D,
    LSL_Z_P_ZZ__,
    BRKPBS_P_P_PP__,
    aarch64_vector_reduce_fp_max_simd,
    aarch64_vector_transfer_vector_permute_zip,
    RDFFRS_P_P_F__,
    aarch32_AESIMC_A1_A,
    aarch32_UQSUB16_A1_A,
    aarch64_vector_reduce_fp_max_sisd,
    LD1H_Z_P_BZ_D_x32_scaled,
    aarch32_LDRBT_A2_A,
    aarch64_vector_arithmetic_unary_add_saturating_simd,
    UQDECW_Z_ZS__,
    aarch32_SXTAB16_A1_A,
    LDFF1SW_Z_P_BZ_D_64_unscaled,
    LD1SH_Z_P_BR_S64,
    CPY_Z_P_R__,
    aarch32_VRINTZ_asimd_A1_A,
    LDNT1W_Z_P_BI_Contiguous,
    aarch32_LDRSB_r_A1_A,
    aarch32_SMC_A1_AS,
    FMAX_Z_P_ZS__,
    aarch64_integer_pac_strip_hint,
    aarch32_SUB_SP_r_A1_A,
    ST1H_Z_P_BZ_S_x32_unscaled,
    aarch32_HLT_A1_A,
    PRFD_I_P_AI_S,
    aarch32_VMVN_r_T1A1_A,
    LDNF1H_Z_P_BI_U16,
    aarch64_integer_tags_mcsettagpairandzerodatapost,
    aarch32_BIC_i_A1_A,
    aarch32_LDM_A1_A,
    LD1B_Z_P_BR_U64,
    aarch32_MCRR_T1A1_A,
    aarch32_VNEG_A2_A,
    aarch32_SSBB_A1_A,
    aarch64_integer_shift_variable,
    LASTB_R_P_Z__,
    aarch64_vector_arithmetic_binary_uniform_add_saturating_simd,
    aarch32_VCVT_iv_A1_A,
    FCMEQ_P_P_Z0__,
    aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress,
    aarch32_PLD_i_A1_A,
    aarch64_vector_arithmetic_binary_element_mul_acc_int,
    LDNF1D_Z_P_BI_U64,
    aarch32_DMB_A1_A,
    CLASTB_R_P_Z__,
    UMIN_Z_ZI__,
    aarch32_SETEND_A1_A,
    aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd,
    aarch32_SBC_i_A1_A,
    LD1W_Z_P_BZ_D_x32_scaled,
    aarch32_LDAEXB_A1_A,
    LD1W_Z_P_BR_U32,
    aarch32_STREXH_A1_A,
    aarch32_QSUB8_A1_A,
    aarch32_VST2_1_T1A1_A,
    LD4B_Z_P_BR_Contiguous,
    aarch32_LDRSHT_A1_A,
    aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd,
    aarch32_UASX_A1_A,
    aarch32_SASX_A1_A,
    aarch64_memory_pair_general_pre_idx,
    LDFF1SB_Z_P_BR_S16,
    aarch32_VMOV_r_T2A2_A,
    INCD_Z_ZS__,
    aarch32_UMAAL_A1_A,
    EOR_Z_ZI__,
    aarch32_QASX_A1_A,
    LDNF1B_Z_P_BI_U8,
    SMAX_Z_ZI__,
    aarch32_STRBT_A1_A,
    aarch64_vector_reduce_add_sisd,
    aarch32_QADD8_A1_A,
    aarch32_VNMLA_A1_A,
    aarch32_VSHLL_T1A1_A,
    aarch32_VQRDMLSH_A1_A,
    LDNT1H_Z_P_BI_Contiguous,
    aarch64_vector_arithmetic_unary_special_sqrt_est_fp16_sisd,
    UMAXV_R_P_Z__,
    STNT1W_Z_P_BI_Contiguous,
    FNMSB_Z_P_ZZZ__,
    LD1D_Z_P_BI_U64,
    aarch32_RSC_r_A1_A,
    aarch64_vector_arithmetic_unary_float_conv_int_simd,
    aarch32_VCVTA_asimd_A1_A,
    aarch32_PKH_A1_A,
    aarch32_VQMOVN_T1A1_A,
    UQADD_Z_ZZ__,
    BFMMLA_Z_ZZZ__,
    aarch32_STRT_A1_A,
    aarch32_VMOVL_T1A1_A,
    aarch64_system_barriers_sb,
    aarch64_vector_crypto_aes_mix,
    aarch64_vector_shift_left_insert_simd,
    PRFH_I_P_AI_S,
    STNT1H_Z_P_BR_Contiguous,
    aarch32_CMN_i_A1_A,
    aarch64_integer_ins_ext_extract_immediate,
    SMINV_R_P_Z__,
    LD1W_Z_P_BZ_D_64_scaled,
    aarch64_vector_arithmetic_binary_element_mul_acc_double_sisd,
    aarch32_VCLS_T1A1_A,
    PRFW_I_P_BZ_S_x32_scaled,
    aarch64_vector_transfer_vector_table,
    aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd,
    aarch64_vector_arithmetic_binary_uniform_cmp_int_simd,
    BFDOT_Z_ZZZi__,
    SABD_Z_P_ZZ__,
    LD1W_Z_P_BZ_S_x32_scaled,
    aarch32_RSC_i_A1_A,
    CTERMEQ_RR__,
    aarch64_float_compare_cond,
    LD1RSH_Z_P_BI_S64,
    aarch32_AESD_A1_A,
    UQSUB_Z_ZZ__,
    FMLS_Z_ZZZi_S,
    LD1W_Z_P_BR_U64,
    aarch64_vector_crypto_sha512_sha512su0,
    aarch32_VCGT_i_A1_A,
    aarch64_vector_arithmetic_binary_uniform_shift_sisd,
    aarch32_VMUL_f_A1_A,
    aarch32_VCGE_r_T1A1_A,
    aarch32_VCVT_xv_A1_A,
    FCMNE_P_P_ZZ__,
    aarch64_vector_arithmetic_binary_element_mul_acc_fp16_sisd,
    aarch32_MVN_i_A1_A,
    aarch64_vector_arithmetic_binary_element_mul_fp16_simd,
    PRFB_I_P_BZ_D_64_scaled,
    aarch32_VLD1_a_T1A1_A,
    LD1SH_Z_P_AI_D,
    FCVTZU_Z_P_Z_D2X,
    CMPGT_P_P_ZI__,
    aarch64_vector_arithmetic_unary_float_xtn_sisd,
    SQDECB_R_RS_SX,
    CMPHI_P_P_ZW__,
    CMPLS_P_P_ZW__,
    STNT1B_Z_P_BR_Contiguous,
    aarch32_VRINTA_vfp_A1_A,
    aarch64_float_arithmetic_round_frint,
    SDIV_Z_P_ZZ__,
    ST1W_Z_P_BZ_D_64_unscaled,
    LDFF1D_Z_P_BZ_D_x32_unscaled,
    LDNF1H_Z_P_BI_U32,
    LD1ROB_Z_P_BI_U8,
    aarch32_VMAX_i_T1A1_A,
    aarch64_vector_shift_right_narrow_uniform_simd,
    PTEST__P_P__,
    aarch64_branch_unconditional_immediate,
    SCVTF_Z_P_Z_W2S,
    LD2W_Z_P_BI_Contiguous,
    aarch32_ORR_r_A1_A,
    FMUL_Z_ZZi_S,
    aarch32_VSUBHN_T1A1_A,
    LD1H_Z_P_BR_U16,
    LDNT1D_Z_P_BI_Contiguous,
    aarch64_vector_crypto_sha2op_sha1_sched1,
    ST3W_Z_P_BR_Contiguous,
    ST1H_Z_P_BZ_D_x32_unscaled,
    LDFF1H_Z_P_BZ_D_64_scaled,
    aarch64_vector_arithmetic_unary_diff_neg_int_sisd,
    LSL_Z_P_ZI__,
    CLZ_Z_P_Z__,
    aarch64_vector_transfer_integer_move_unsigned,
    ST3B_Z_P_BR_Contiguous,
    LSR_Z_P_ZW__,
    LD2B_Z_P_BI_Contiguous,
    aarch64_vector_arithmetic_unary_float_conv_float_tieaway_simd,
    aarch32_TST_i_A1_A,
    aarch32_VLD3_m_T1A1_A,
    aarch64_vector_reduce_add_simd,
    aarch32_SUB_rr_A1_A,
    aarch64_vector_arithmetic_binary_uniform_mat_mul_int_mla,
    FMUL_Z_ZZi_H,
    aarch32_VQABS_T1A1_A,
    aarch64_system_exceptions_runtime_svc,
    aarch32_VMOVN_T1A1_A,
    LDFF1SW_Z_P_BR_S64,
    aarch32_STRD_r_A1_A,
    aarch32_STLH_A1_A,
    ADDPL_R_RI__,
    FMAD_Z_P_ZZZ__,
    aarch32_VPADD_i_T1A1_A,
    LD1SB_Z_P_BZ_S_x32_unscaled,
    aarch32_TEQ_rr_A1_A,
    aarch32_STR_r_A1_A,
    ST1D_Z_P_BI__,
    LDFF1H_Z_P_AI_D,
    SXTB_Z_P_Z__,
    aarch64_integer_pac_pacib_dp_1src,
    FMAXNM_Z_P_ZZ__,
    SUB_Z_ZI__,
    aarch64_vector_reduce_add_long,
    FCVTZU_Z_P_Z_S2X,
    LDNF1SH_Z_P_BI_S64,
    UQDECD_R_RS_UW,
    LDFF1SB_Z_P_BR_S32,
    aarch64_integer_logical_immediate,
    UXTW_Z_P_Z__,
    aarch64_vector_arithmetic_binary_element_mul_acc_high_simd,
    aarch64_vector_shift_right_insert_simd,
    aarch64_memory_single_general_immediate_unsigned,
    LDFF1SH_Z_P_BZ_D_64_scaled,
    ST1W_Z_P_AI_D,
    LD3H_Z_P_BI_Contiguous,
    aarch32_LDRH_i_A1_A,
    aarch64_memory_single_general_immediate_signed_offset_unpriv,
    aarch64_vector_arithmetic_binary_uniform_rsqrts_sisd,
    LD1ROB_Z_P_BR_Contiguous,
    CLASTA_Z_P_ZZ__,
    UADDV_R_P_Z__,
    aarch32_VSUB_f_A2_A,
    LSL_Z_ZI__,
    CNTH_R_S__,
    aarch32_VCMP_A1_A,
    aarch64_vector_cvt_bf16_vector,
    LDFF1W_Z_P_AI_D,
    WRFFR_F_P__,
    LDFF1H_Z_P_BZ_S_x32_scaled,
    aarch32_VLDM_T2A2_A,
    FDIV_Z_P_ZZ__,
    FCVTZS_Z_P_Z_S2W,
    LD1SB_Z_P_AI_S,
    aarch32_VABS_A2_A,
    FMLS_Z_P_ZZZ__,
    aarch32_VCVTT_T1A1_A,
    ASR_Z_P_ZW__,
    aarch32_STM_A1_A,
    SDOT_Z_ZZZi_D,
    aarch64_vector_arithmetic_binary_element_mul_acc_mul_norounding_i_lower,
    ST4W_Z_P_BI_Contiguous,
    FMUL_Z_ZZ__,
    aarch64_branch_conditional_compare,
    FTSSEL_Z_ZZ__,
    aarch32_VDUP_r_T1A1_A,
    aarch32_VQDMLSL_T1A1_A,
    aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd,
    LD1H_Z_P_BI_U32,
    aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd,
    aarch32_LDAH_A1_A,
    FCVTZS_Z_P_Z_S2X,
    LD1RW_Z_P_BI_U32,
    SQDECH_R_RS_SX,
    aarch64_vector_arithmetic_binary_disparate_mul_double_simd,
    aarch32_VCLT_i_A1_A,
    aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd,
    aarch32_RSB_r_A1_A,
    FABD_Z_P_ZZ__,
    SQDECP_R_P_R_X,
    LDNF1W_Z_P_BI_U64,
    aarch64_integer_pac_pacda_dp_1src,
    INSR_Z_R__,
    aarch32_LDR_r_A1_A,
    LDNT1D_Z_P_BR_Contiguous,
    aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd,
    aarch32_SHSUB16_A1_A,
    CMPLT_P_P_ZI__,
    LDFF1W_Z_P_AI_S,
    aarch64_vector_arithmetic_unary_fp16_conv_int_simd,
    aarch32_VMSR_T1A1_AS,
    aarch32_VTRN_T1A1_A,
    STNT1D_Z_P_BI_Contiguous,
    aarch32_LDMDB_A1_A,
    LDFF1D_Z_P_BZ_D_64_unscaled,
    aarch64_vector_arithmetic_unary_float_round_frint_32_64,
    aarch32_UDF_A1_A,
    CLS_Z_P_Z__,
    aarch32_SXTB16_A1_A,
    aarch32_UQADD8_A1_A,
    aarch64_float_arithmetic_round_frint_32_64,
    LD1D_Z_P_BZ_D_64_unscaled,
    UMINV_R_P_Z__,
    LD1SB_Z_P_BR_S16,
    FCMGE_P_P_ZZ__,
    aarch64_memory_single_simdfp_immediate_signed_offset_normal,
    aarch64_vector_transfer_integer_move_signed,
    aarch64_memory_vector_single_post_inc,
    aarch64_integer_tags_mcsettagpairpost,
    MLA_Z_P_ZZZ__,
    aarch32_STRH_r_A1_A,
    aarch64_vector_transfer_vector_extract,
    aarch32_PSSBB_A1_A,
    aarch64_vector_crypto_aes_round,
    aarch32_VQSHL_r_T1A1_A,
    aarch64_vector_arithmetic_binary_uniform_mul_fp16_fused,
    aarch32_VRSHR_T1A1_A,
    aarch64_integer_pac_autia_hint,
    LD1RQH_Z_P_BI_U16,
    aarch32_VQDMULH_T1A1_A,
    UQDECH_Z_ZS__,
    LDFF1SH_Z_P_BZ_S_x32_scaled,
    SPLICE_Z_P_ZZ_Des,
    aarch32_STMDB_A1_A,
    aarch64_vector_arithmetic_binary_uniform_mul_fp_complex,
    aarch32_VABD_i_T1A1_A,
    aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_accum_simd,
    aarch64_vector_arithmetic_unary_add_pairwise,
    LDFF1W_Z_P_BZ_S_x32_unscaled,
    aarch32_UBFX_A1_A,
    UUNPKHI_Z_Z__,
    PRFD_I_P_BR_S,
    aarch64_system_exceptions_debug_breakpoint,
    LD1RH_Z_P_BI_U64,
    aarch32_SXTH_A1_A,
    aarch32_SADD16_A1_A,
    aarch32_VEOR_T1A1_A,
    CMPLO_P_P_ZI__,
    LDFF1H_Z_P_BZ_D_x32_scaled,
    STNT1W_Z_P_BR_Contiguous,
    SCVTF_Z_P_Z_W2FP16,
    aarch32_VCEQ_i_A1_A,
    STNT1H_Z_P_BI_Contiguous,
    BRKPB_P_P_PP__,
    aarch64_vector_arithmetic_binary_element_mul_acc_fp_sisd,
    aarch32_VFMA_A2_A,
    UQSUB_Z_ZI__,
    aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd,
    BRKAS_P_P_P_Z,
    LDFF1W_Z_P_BZ_D_64_scaled,
    FCVTZS_Z_P_Z_FP162H,
    aarch64_vector_arithmetic_binary_element_mul_high_simd,
    aarch32_SHSAX_A1_A,
    LDFF1SH_Z_P_BR_S32,
    CMPLE_P_P_ZW__,
    aarch64_branch_unconditional_eret,
    UCVTF_Z_P_Z_W2D,
    LDNF1SB_Z_P_BI_S32,
    aarch32_VRINTA_asimd_A1_A,
    aarch32_VMLA_f_A2_A,
    aarch64_integer_tags_mcsettagpre,
    aarch32_VQDMULL_T1A1_A,
    aarch64_vector_arithmetic_binary_element_mul_acc_bf16_long,
    LDFF1D_Z_P_BR_U64,
    LDFF1SH_Z_P_AI_S,
    DECP_Z_P_Z__,
    LDR_Z_BI__,
    LD4W_Z_P_BR_Contiguous,
    ST3D_Z_P_BR_Contiguous,
    aarch32_VDUP_s_T1A1_A,
    ZIP2_Z_ZZ__,
    aarch32_SMLALBB_A1_A,
    aarch32_VMOVX_A1_A,
    LDNF1B_Z_P_BI_U64,
    PRFH_I_P_BZ_D_x32_scaled,
    ST4B_Z_P_BI_Contiguous,
    aarch32_VBIC_r_T1A1_A,
    PRFH_I_P_AI_D,
    aarch64_vector_arithmetic_binary_uniform_recps_fp16_sisd,
    aarch32_SMLSLD_A1_A,
    SQINCW_Z_ZS__,
    aarch32_VLD4_m_T1A1_A,
    aarch64_integer_flags_setf,
    EOR_Z_ZZ__,
    LD1SB_Z_P_BI_S64,
    aarch32_VMLA_i_T2A2_A,
    FMMLA_Z_ZZZ_S,
    UUNPKLO_Z_Z__,
    LD1B_Z_P_BI_U16,
    aarch32_LDRT_A2_A,
    aarch32_SHA1M_A1_A,
    aarch32_ADC_i_A1_A,
    LD1B_Z_P_BR_U8,
    LD1SW_Z_P_BZ_D_x32_scaled,
    FCVT_Z_P_Z_S2H,
    LDFF1SW_Z_P_BZ_D_x32_scaled,
    UQDECW_R_RS_X,
    UQINCH_Z_ZS__,
    aarch32_VSRI_T1A1_A,
    FCVTZU_Z_P_Z_S2W,
    aarch64_vector_arithmetic_unary_special_sqrt_est_int,
    aarch64_vector_arithmetic_binary_uniform_sub_fp_simd,
    aarch32_STRHT_A2_A,
    LD1H_Z_P_BZ_D_64_unscaled,
    aarch32_AESE_A1_A,
    aarch64_integer_flags_xaflag,
    aarch32_STRHT_A1_A,
    aarch32_TST_r_A1_A,
    FCVTZU_Z_P_Z_FP162X,
    ST1B_Z_P_BZ_D_64_unscaled,
    UMAX_Z_ZI__,
    MUL_Z_P_ZZ__,
    LD1D_Z_P_BZ_D_64_scaled,
    aarch32_VQADD_T1A1_A,
    aarch64_vector_arithmetic_binary_element_mul_long,
    LD1SW_Z_P_BR_S64,
    LD1RB_Z_P_BI_U64,
    aarch32_VMOV_h_A1_A,
    aarch32_LDREXB_A1_A,
    aarch32_VQDMLSL_T2A2_A,
    LD1SW_Z_P_BZ_D_x32_unscaled,
    aarch64_vector_arithmetic_unary_fp16_conv_float_tieaway_sisd,
    aarch64_vector_arithmetic_binary_uniform_mat_mul_int_usdot,
    aarch64_vector_arithmetic_binary_element_dotp,
    aarch32_MSR_r_A1_AS,
    DECW_R_RS__,
    PRFH_I_P_BZ_S_x32_scaled,
    LD1ROD_Z_P_BR_Contiguous,
    TRN2_P_PP__,
    ABS_Z_P_Z__,
    aarch64_vector_crypto_sm3_sm3tt2a,
    aarch32_VFMAL_i_A1_A,
    aarch64_float_arithmetic_max_min,
    aarch64_vector_shift_right_simd,
    LASTA_V_P_Z__,
    LD1SW_Z_P_BZ_D_64_scaled,
    FRSQRTE_Z_Z__,
    PRFB_I_P_BR_S,
    aarch32_MOV_r_A1_A,
    UQINCW_R_RS_UW,
    aarch32_SMULWB_A1_A,
    PRFH_I_P_BR_S,
    WHILELT_P_P_RR__,
    LD1H_Z_P_BZ_S_x32_unscaled,
    aarch32_USUB8_A1_A,
    LD1H_Z_P_BI_U16,
    LD1RW_Z_P_BI_U64,
    USMMLA_Z_ZZZ__,
    aarch32_LDREXH_A1_A,
    aarch32_VCGE_i_A1_A,
    LDFF1SB_Z_P_BZ_D_x32_unscaled,
    aarch32_SMLAD_A1_A,
    INCW_R_RS__,
    aarch32_VLD4_1_T1A1_A,
    aarch32_VLD3_1_T1A1_A,
    ADD_Z_ZZ__,
    aarch64_vector_arithmetic_unary_add_saturating_sisd,
    aarch32_VQDMULL_T2A2_A,
    UZP2_Z_ZZ_Q,
    aarch64_system_exceptions_runtime_hvc,
    aarch32_STLB_A1_A,
    SCVTF_Z_P_Z_X2D,
    aarch64_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_upper,
    CNOT_Z_P_Z__,
    aarch32_VLD1_1_T1A1_A,
    aarch64_vector_reduce_fp_maxnm_sisd,
    aarch32_ORR_i_A1_A,
    aarch32_VQSHRN_T1A1_A,
    FCVTZS_Z_P_Z_D2X,
    aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair,
    aarch32_VCMLA_A1_A,
    aarch64_vector_reduce_int_max,
    aarch32_SBC_r_A1_A,
    aarch32_CMP_rr_A1_A,
    aarch32_VCEQ_r_T1A1_A,
    aarch32_VRHADD_T1A1_A,
    BRKNS_P_P_PP__,
    aarch64_memory_atomicops_cas_pair,
    SQINCH_R_RS_SX,
    EORV_R_P_Z__,
    aarch64_vector_arithmetic_unary_special_recip_fp16_simd,
    SMAX_Z_P_ZZ__,
    BFDOT_Z_ZZZ__,
    LDFF1SB_Z_P_BZ_S_x32_unscaled,
    INSR_Z_V__,
    ST1B_Z_P_BZ_D_x32_unscaled,
    aarch32_CLREX_A1_A,
    aarch64_vector_arithmetic_unary_special_sqrt_est_float_sisd,
    aarch32_VADD_f_A1_A,
    aarch32_STREX_A1_A,
    aarch64_vector_arithmetic_unary_extract_nosat,
    aarch64_vector_transfer_vector_cpy_dup_simd,
    aarch32_MUL_A1_A,
    aarch64_vector_arithmetic_binary_uniform_mul_fp16_extended_simd,
    SUNPKLO_Z_Z__,
    aarch32_RFE_A1_AS,
    aarch32_VMOV_i_A2_A,
    aarch32_VSUB_i_T1A1_A,
    aarch64_integer_arithmetic_mul_widening_64_128hi,
    ST3W_Z_P_BI_Contiguous,
    COMPACT_Z_P_Z__,
    aarch32_VLDM_T1A1_A,
    PUNPKHI_P_P__,
    aarch64_vector_arithmetic_binary_uniform_rsqrts_fp16_simd,
    LD1RD_Z_P_BI_U64,
    UXTB_Z_P_Z__,
    INDEX_Z_RR__,
    aarch32_DBG_A1_A,
    aarch32_SSAT16_A1_A,
    aarch64_vector_arithmetic_binary_element_mul_double_simd,
    CMPLT_P_P_ZW__,
    aarch64_vector_arithmetic_binary_uniform_mul_int_dotp,
    LD1RQH_Z_P_BR_Contiguous,
    LD1SH_Z_P_BZ_S_x32_scaled,
    aarch32_MSR_i_A1_AS,
    aarch32_VAND_r_T1A1_A,
    EOR_Z_P_ZZ__,
    PFALSE_P__,
    aarch64_vector_arithmetic_unary_special_recip_float_simd,
    aarch64_vector_arithmetic_binary_element_mul_acc_fp_simd,
    aarch32_SVC_A1_A,
    ST4H_Z_P_BI_Contiguous,
    aarch32_VQRDMLAH_A2_A,
    LDFF1SW_Z_P_BZ_D_64_scaled,
    FCVT_Z_P_Z_D2S,
    aarch32_SADD8_A1_A,
    aarch32_VADD_i_T1A1_A,
    aarch32_VRSHRN_T1A1_A,
    SXTW_Z_P_Z__,
    aarch64_vector_arithmetic_unary_extract_sqxtun_simd,
    aarch32_STRH_i_A1_A,
    LSR_Z_ZI__,
    aarch64_integer_arithmetic_add_sub_extendedreg,
    aarch32_LDRD_r_A1_A,
    aarch64_vector_crypto_sha3_xar,
    aarch32_LDREX_A1_A,
    SMMLA_Z_ZZZ__,
    aarch32_SMUSD_A1_A,
    FMMLA_Z_ZZZ_D,
    aarch64_vector_arithmetic_unary_special_recip_fp16_sisd,
    aarch32_VST1_m_T1A1_A,
    SQDECH_R_RS_X,
    ST1W_Z_P_BZ_D_x32_unscaled,
    DECH_R_RS__,
    LDFF1D_Z_P_BZ_D_x32_scaled,
    aarch32_CMP_i_A1_A,
    LD1RB_Z_P_BI_U8,
    aarch32_VFNMA_A1_A,
    ST4D_Z_P_BI_Contiguous,
    aarch32_STRB_r_A1_A,
    aarch64_float_arithmetic_mul_product,
    aarch32_LDMIB_A1_A,
    LD2D_Z_P_BR_Contiguous,
    aarch64_integer_tags_mcsettagpairandzerodatapre,
    aarch64_vector_arithmetic_binary_element_mul_double_sisd,
    LD1SH_Z_P_BR_S32,
    aarch32_MOV_i_A1_A,
    LSR_Z_P_ZI__,
    INDEX_Z_IR__,
    LDFF1B_Z_P_BZ_S_x32_unscaled,
    FCMLE_P_P_Z0__,
    EOR_P_P_PP_Z,
    aarch64_vector_shift_right_narrow_nonuniform_sisd,
    aarch64_vector_transfer_vector_cpy_dup_sisd,
    FSUB_Z_P_ZZ__,
    aarch32_VMMLA_A1_A,
    aarch64_integer_pac_pacia_dp_1src,
    aarch32_VLD2_1_T1A1_A,
    aarch32_USUB16_A1_A,
    aarch32_MOVT_A1_A,
    FSUBR_Z_P_ZS__,
    SQDECP_Z_P_Z__,
    PRFD_I_P_AI_D,
    aarch32_DSB_A1_A,
    ST3B_Z_P_BI_Contiguous,
    aarch32_VDOT_s_A1_A,
    PRFD_I_P_BI_S,
    ST2H_Z_P_BI_Contiguous,
    FCVT_Z_P_Z_D2H,
    BFMLALB_Z_ZZZi__,
    FSUBR_Z_P_ZZ__,
    aarch64_vector_arithmetic_unary_diff_neg_sat_sisd,
    BFMLALT_Z_ZZZ__,
    DECH_Z_ZS__,
    UCVTF_Z_P_Z_H2FP16,
    aarch64_integer_pac_strip_dp_1src,
    aarch32_LDRB_r_A1_A,
    ST1W_Z_P_BZ_D_64_scaled,
    aarch32_VSTR_A1_A,
    aarch32_MMLA_A1_A,
    aarch32_LDR_i_A1_A,
    aarch32_VMUL_i_T1A1_A,
    aarch32_BIC_r_A1_A,
    aarch32_CPS_A1_AS,
    ST2D_Z_P_BR_Contiguous,
    aarch32_STLEXD_A1_A,
    LD2W_Z_P_BR_Contiguous,
    LD1W_Z_P_BI_U64,
    FCVTZU_Z_P_Z_FP162W,
    aarch32_QSAX_A1_A,
    LSL_Z_ZW__,
    aarch32_LDA_A1_A,
    aarch32_VQRDMULH_T1A1_A,
    aarch32_VMUL_f_A2_A,
    PRFB_I_P_AI_S,
    CMPGT_P_P_ZZ__,
    ORR_Z_ZI__,
    aarch32_SMLAL_A1_A,
    aarch64_integer_pac_pacga_dp_2src,
    aarch32_WFE_A1_A,
    aarch32_SXTB_A1_A,
    aarch64_memory_single_simdfp_register,
    aarch64_float_move_fp_imm,
    aarch32_ADR_A1_A,
    FRINTX_Z_P_Z__,
    SEL_P_P_PP__,
    aarch64_vector_arithmetic_binary_uniform_mul_int_bfdot,
    aarch64_vector_shift_right_sisd,
    aarch32_LDR_l_A1_A,
    LD3W_Z_P_BI_Contiguous,
    BIC_Z_P_ZZ__,
    CMPEQ_P_P_ZW__,
    LD1B_Z_P_BZ_S_x32_unscaled,
    aarch64_vector_arithmetic_binary_uniform_logical_and_orr,
    aarch32_TEQ_i_A1_A,
    aarch64_memory_single_simdfp_immediate_unsigned,
    LD4W_Z_P_BI_Contiguous,
    FTSMUL_Z_ZZ__,
    PNEXT_P_P_P__,
    aarch64_integer_pac_pacib_hint,
    aarch32_VSHLL_T2A2_A,
    aarch64_vector_logical,
    CMPGE_P_P_ZZ__,
    LD2H_Z_P_BR_Contiguous,
    LD1SB_Z_P_BZ_D_64_unscaled,
    aarch32_USAX_A1_A,
    CLASTB_Z_P_ZZ__,
    SDOT_Z_ZZZ__,
    aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd,
    aarch64_memory_vector_single_no_wb,
    SQDECB_R_RS_X,
    LD1B_Z_P_AI_S,
    FCVT_Z_P_Z_H2S,
    aarch32_STRT_A2_A,
    aarch64_memory_pair_general_post_idx,
    aarch64_vector_shift_left_insert_sisd,
    FRECPX_Z_P_Z__,
    aarch32_RSC_rr_A1_A,
    aarch64_integer_crc,
    aarch32_VST4_1_T1A1_A,
    LD1W_Z_P_BZ_S_x32_unscaled,
    ST2H_Z_P_BR_Contiguous,
    LD1RQB_Z_P_BI_U8,
    PRFH_I_P_BZ_D_64_scaled,
    UCVTF_Z_P_Z_X2FP16,
    aarch64_branch_conditional_cond,
    ORNS_P_P_PP_Z,
    aarch64_vector_arithmetic_binary_disparate_mul_product,
    aarch32_WFI_A1_A,
    UQINCP_R_P_R_UW,
    aarch32_SHA1C_A1_A,
    aarch32_VACGE_A1_A,
    aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_simd,
    aarch64_vector_arithmetic_binary_uniform_max_min_fp_2008,
    aarch32_VMOV_ss_T1A1_A,
    aarch32_VCVT_hs_T1A1_A,
    aarch32_VQDMLAL_T2A2_A,
    aarch32_TSB_A1_A,
    aarch64_integer_pac_autia_dp_1src,
    aarch32_VADDHN_T1A1_A,
    LD1SB_Z_P_BR_S32,
    FSQRT_Z_P_Z__,
    aarch64_float_compare_uncond,
    LD1H_Z_P_BZ_D_x32_unscaled,
    aarch32_VMAXNM_A1_A,
    UMMLA_Z_ZZZ__,
    aarch32_VCVT_T1A1_A,
    aarch32_STR_i_A1_A,
    aarch32_RBIT_A1_A,
    aarch64_vector_arithmetic_unary_rev,
    LDFF1H_Z_P_AI_S,
    SUB_Z_ZZ__,
    LD1RH_Z_P_BI_U16,
    aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_simd,
    aarch32_SMMLS_A1_A,
    aarch64_integer_tags_mcsettaganddatapairpre,
    aarch32_UMLAL_A1_A,
    LDFF1SH_Z_P_BZ_D_x32_unscaled,
    aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd,
    aarch32_VMLA_i_T1A1_A,
    FRINTN_Z_P_Z__,
    aarch64_vector_crypto_sha3op_sha1_sched0,
    LD1ROH_Z_P_BR_Contiguous,
    SQINCD_Z_ZS__,
    aarch32_VDOT_A1_A,
    FRECPE_Z_Z__,
    aarch64_vector_reduce_fp16_max_simd,
    UQDECB_R_RS_X,
    LDFF1SW_Z_P_AI_D,
    aarch32_NOP_A1_A,
    aarch64_vector_shift_left_sisd,
    aarch64_integer_pac_pacdb_dp_1src,
    aarch64_vector_reduce_fp16_max_sisd,
    LDFF1B_Z_P_BZ_D_64_unscaled,
    PRFW_I_P_AI_S,
    ST2W_Z_P_BR_Contiguous,
    INCP_R_P_R__,
    LD1ROH_Z_P_BI_U16,
    aarch32_SMUAD_A1_A,
    aarch32_VMOV_s_A1_A,
    aarch64_float_arithmetic_unary,
    LSR_Z_P_ZZ__,
    aarch64_vector_reduce_fp_add_sisd,
    aarch64_integer_tags_mcsettagpairandzerodata,
    aarch64_vector_arithmetic_unary_extract_sat_sisd,
    aarch64_memory_vector_multiple_post_inc,
    aarch32_MOV_i_A2_A,
    ORR_Z_ZZ__,
    aarch32_CMN_r_A1_A,
    aarch32_UXTAH_A1_A,
    aarch64_vector_shift_conv_float_sisd,
    LD1SH_Z_P_BZ_S_x32_unscaled,
    aarch64_integer_arithmetic_rbit,
    aarch32_VPADDL_T1A1_A,
    LDFF1SH_Z_P_BZ_S_x32_unscaled,
    aarch32_REV_A1_A,
    aarch32_STMIB_A1_A,
    aarch32_VCVT_xs_A1_A,
    LD1B_Z_P_BR_U16,
    aarch64_vector_arithmetic_unary_special_recip_int,
    aarch32_STC_T1A1_A,
    BRKPAS_P_P_PP__,
    aarch64_vector_arithmetic_binary_uniform_div,
    aarch32_MRS_br_A1_AS,
    aarch32_VABD_i_T2A2_A,
    ST1H_Z_P_BZ_S_x32_scaled,
    ST1H_Z_P_BZ_D_64_unscaled,
    UZP1_Z_ZZ_Q,
    aarch32_UXTB16_A1_A,
    SQINCW_R_RS_SX,
    aarch32_VRINTX_asimd_A1_A,
    FMINNM_Z_P_ZZ__,
    SUDOT_Z_ZZZi_S,
    aarch32_VRSQRTS_A1_A,
    aarch64_memory_ordered,
    aarch32_MSR_br_A1_AS,
    aarch32_ADD_SP_r_A1_A,
    aarch64_integer_conditional_compare_immediate,
    LD1H_Z_P_BI_U64,
    FABS_Z_P_Z__,
    UQDECW_R_RS_UW,
    aarch32_VMOV_d_T1A1_A,
    aarch64_system_barriers_pssbb,
    SQINCP_Z_P_Z__,
    aarch64_vector_crypto_sha3op_sha1_hash_majority,
    aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd,
    aarch64_branch_unconditional_register,
    aarch32_MLA_A1_A,
    aarch32_USAT16_A1_A,
    FMUL_Z_P_ZS__,
    aarch32_LDRSB_l_A1_A,
    aarch32_SUB_SP_i_A1_A,
    ST1B_Z_P_AI_D,
    PTRUE_P_S__,
    aarch64_vector_arithmetic_binary_uniform_shift_simd,
    LDNF1B_Z_P_BI_U16,
    aarch32_EOR_r_A1_A,
    aarch64_system_hints,
    aarch64_memory_ordered_rcpc,
    aarch64_memory_vector_multiple_no_wb,
    aarch32_QADD_A1_A,
    SQINCP_R_P_R_X,
    ORR_Z_P_ZZ__,
    UDOT_Z_ZZZ__,
    ST1B_Z_P_AI_S,
    LD1W_Z_P_BZ_D_x32_unscaled,
    aarch64_vector_shift_left_sat_sisd,
    aarch64_memory_pair_general_no_alloc,
    aarch64_vector_transfer_integer_dup,
    aarch32_UQASX_A1_A,
    aarch32_VJCVT_A1_A,
    LDFF1W_Z_P_BZ_S_x32_scaled,
    aarch32_VCLZ_T1A1_A,
    aarch32_UHSUB8_A1_A,
    ADD_Z_P_ZZ__,
    SQSUB_Z_ZI__,
    LASTB_V_P_Z__,
    aarch32_VORR_r_T1A1_A,
    aarch64_vector_shift_right_narrow_uniform_sisd,
    aarch32_MCR_T1A1_A,
    aarch64_vector_arithmetic_binary_uniform_sub_int,
    aarch32_VCVT_ds_T1A1_A,
    aarch64_vector_arithmetic_binary_element_mul_int,
    CMPHI_P_P_ZZ__,
    LD1RQB_Z_P_BR_Contiguous,
    aarch64_vector_arithmetic_binary_uniform_max_min_single,
    aarch32_VMAXNM_A2_A,
    FMLA_Z_ZZZi_H,
    SQDECD_R_RS_X,
    aarch32_LDAEXH_A1_A,
    FRECPS_Z_ZZ__,
    aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd,
    ASRD_Z_P_ZI__,
    aarch32_LDRSH_i_A1_A,
    CPY_Z_P_I__,
    aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd,
    aarch64_vector_arithmetic_binary_uniform_mul_fp_product,
    UQINCH_R_RS_X,
    EXT_Z_ZI_Des,
    aarch64_vector_arithmetic_unary_not,
    aarch32_LDRB_l_A1_A,
    INCP_Z_P_Z__,
    FMAXNM_Z_P_ZS__,
    aarch32_UXTH_A1_A,
    EORS_P_P_PP_Z,
    aarch64_vector_shift_conv_float_simd,
    UQINCP_Z_P_Z__,
    aarch64_vector_arithmetic_binary_element_mul_acc_high_sisd,
    aarch64_vector_reduce_fp16_add_sisd,
    FMAX_Z_P_ZZ__,
    aarch32_VBIF_T1A1_A,
    aarch32_VRINTX_vfp_A1_A,
    FACGE_P_P_ZZ__,
    aarch32_VMAX_f_A1_A,
    aarch32_PLI_i_A1_A,
    aarch64_integer_pac_autib_dp_1src,
    NAND_P_P_PP_Z,
    aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd,
    INDEX_Z_II__,
    ZIP1_Z_ZZ_Q,
    aarch64_integer_arithmetic_rev,
    SQINCB_R_RS_X,
    aarch64_vector_arithmetic_unary_float_conv_float_tieaway_sisd,
    NOR_P_P_PP_Z,
    FTMAD_Z_ZZI__,
    LDFF1H_Z_P_BZ_D_64_unscaled,
    aarch32_UQSUB8_A1_A,
    aarch32_LDRSH_l_A1_A,
    aarch32_VST3_1_T1A1_A,
    PRFH_I_P_BI_S,
    aarch64_vector_bfmmla,
    PRFD_I_P_BZ_D_64_scaled,
    RDVL_R_I__,
    aarch32_BX_A1_A,
    aarch64_vector_arithmetic_binary_uniform_mul_fp16_extended_sisd,
    ST1H_Z_P_BZ_D_x32_scaled,
    aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd,
    UMAX_Z_P_ZZ__,
    aarch64_float_convert_fp,
    aarch32_UXTAB16_A1_A,
    aarch32_VDIV_A1_A,
    SCVTF_Z_P_Z_X2S,
    aarch32_LDRSB_i_A1_A,
    aarch64_integer_flags_axflag,
    aarch64_memory_single_general_register,
    FRINTZ_Z_P_Z__,
    aarch64_memory_atomicops_ld,
    aarch32_USADA8_A1_A,
    aarch32_CMP_r_A1_A,
    aarch32_UQSAX_A1_A,
    LD1RH_Z_P_BI_U32,
    aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd,
    aarch32_VCVTA_vfp_A1_A,
    SCVTF_Z_P_Z_X2FP16,
    LD1SW_Z_P_AI_D,
    aarch32_QDADD_A1_A,
    LD1SW_Z_P_BI_S64,
    LASTA_R_P_Z__,
    UQDECP_R_P_R_UW,
    FMLS_Z_ZZZi_D,
    aarch64_vector_crypto_sha3op_sha1_hash_choose,
    aarch32_TEQ_r_A1_A,
    LD1B_Z_P_BI_U64,
    aarch32_UXTB_A1_A,
    UQDECP_Z_P_Z__,
    aarch32_LDRBT_A1_A,
    aarch64_integer_flags_cfinv,
    RDFFR_P_P_F__,
    BIC_Z_ZZ__,
    aarch32_USAT_A1_A,
    CNTB_R_S__,
    aarch64_vector_crypto_sm4_sm4enckey,
    aarch64_vector_arithmetic_unary_float_xtn_simd,
    aarch64_integer_tags_mcsettagpairpre,
    SCVTF_Z_P_Z_W2D,
    BRKBS_P_P_P_Z,
    UCVTF_Z_P_Z_X2S,
}


#[cfg(test)]
mod tests {
    use super::{decode_a64, decode_a32, Op};

    #[test]
    fn test() {
        assert_eq!(decode_a32(0xe3a00001).unwrap(), Op::aarch32_MOV_i_A1_A);
    }
}
