#[allow(unused_variables)]
#[allow(non_snake_case)]
pub fn decode_a64(instr: u32) {
    match ((instr >> 29) & 5, (instr >> 24) & 9, instr & 47) {
        (_, x1, _) if x1 & 30 == 0 => {
            match ((instr >> 29) & 5, (instr >> 25) & 7, (instr >> 16) & 17, instr & 31) {
                (x0, _, x2, _) if x0 == 0 && x2 == 0 => {
                    let imm16 = instr & 31;
                    match () {
                        () => {
                            println!("encoding: aarch64_udf");
                        }
                    }
                }
                (_, _, x2, _) if x2 != 0 => {
                }
                (x0, _, _, _) if x0 != 0 => {
                }
                _ => unreachable!()
            }
        }
        (_, x1, _) if x1 == 3 => {
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
                                    println!("encoding: MLA_Z_P_ZZZ__");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: MLS_Z_P_ZZZ__");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: MAD_Z_P_ZZZ__");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: MSB_Z_P_ZZZ__");
                                }
                                _ => unreachable!()
                            }
                        }
                        _ => unreachable!()
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
                                    println!("encoding: ADD_Z_P_ZZ__");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: SUB_Z_P_ZZ__");
                                }
                                x0 if x0 == 2 => {
                                    println!("unallocated");
                                }
                                x0 if x0 == 3 => {
                                    println!("encoding: SUBR_Z_P_ZZ__");
                                }
                                x0 if x0 & 4 == 4 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: SMAX_Z_P_ZZ__");
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    println!("encoding: UMAX_Z_P_ZZ__");
                                }
                                (x0, x1) if x0 == 1 && x1 == 0 => {
                                    println!("encoding: SMIN_Z_P_ZZ__");
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    println!("encoding: UMIN_Z_P_ZZ__");
                                }
                                (x0, x1) if x0 == 2 && x1 == 0 => {
                                    println!("encoding: SABD_Z_P_ZZ__");
                                }
                                (x0, x1) if x0 == 2 && x1 == 1 => {
                                    println!("encoding: UABD_Z_P_ZZ__");
                                }
                                (x0, _) if x0 == 3 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: MUL_Z_P_ZZ__");
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    println!("unallocated");
                                }
                                (x0, x1) if x0 == 1 && x1 == 0 => {
                                    println!("encoding: SMULH_Z_P_ZZ__");
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    println!("encoding: UMULH_Z_P_ZZ__");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: SDIV_Z_P_ZZ__");
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    println!("encoding: UDIV_Z_P_ZZ__");
                                }
                                (x0, x1) if x0 == 1 && x1 == 0 => {
                                    println!("encoding: SDIVR_Z_P_ZZ__");
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    println!("encoding: UDIVR_Z_P_ZZ__");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: ORR_Z_P_ZZ__");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: EOR_Z_P_ZZ__");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: AND_Z_P_ZZ__");
                                }
                                x0 if x0 == 3 => {
                                    println!("encoding: BIC_Z_P_ZZ__");
                                }
                                x0 if x0 & 4 == 4 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
                            }
                        }
                        _ => unreachable!()
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
                                    println!("encoding: SADDV_R_P_Z__");
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    println!("encoding: UADDV_R_P_Z__");
                                }
                                (x0, _) if x0 == 1 => {
                                    println!("unallocated");
                                }
                                (x0, _) if x0 & 2 == 2 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: SMAXV_R_P_Z__");
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    println!("encoding: UMAXV_R_P_Z__");
                                }
                                (x0, x1) if x0 == 1 && x1 == 0 => {
                                    println!("encoding: SMINV_R_P_Z__");
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    println!("encoding: UMINV_R_P_Z__");
                                }
                                (x0, _) if x0 & 2 == 2 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: MOVPRFX_Z_P_Z__");
                                }
                                x0 if x0 == 1 => {
                                    println!("unallocated");
                                }
                                x0 if x0 & 2 == 2 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: ORV_R_P_Z__");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: EORV_R_P_Z__");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: ANDV_R_P_Z__");
                                }
                                x0 if x0 == 3 => {
                                    println!("unallocated");
                                }
                                x0 if x0 & 4 == 4 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
                            }
                        }
                        _ => unreachable!()
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
                                    println!("encoding: ASR_Z_P_ZI__");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                                    println!("encoding: LSR_Z_P_ZI__");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    println!("unallocated");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: LSL_Z_P_ZI__");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    println!("encoding: ASRD_Z_P_ZI__");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    println!("unallocated");
                                }
                                (x0, x1, _) if x0 == 1 && x1 == 1 => {
                                    println!("unallocated");
                                }
                                (x0, _, _) if x0 & 2 == 2 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
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
                                    println!("unallocated");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                                    println!("encoding: ASR_Z_P_ZZ__");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                                    println!("encoding: LSR_Z_P_ZZ__");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: LSL_Z_P_ZZ__");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    println!("encoding: ASRR_Z_P_ZZ__");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    println!("encoding: LSRR_Z_P_ZZ__");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: LSLR_Z_P_ZZ__");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: ASR_Z_P_ZW__");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                                    println!("encoding: LSR_Z_P_ZW__");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    println!("unallocated");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: LSL_Z_P_ZW__");
                                }
                                (x0, _, _) if x0 == 1 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
                            }
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 0 && x2 & 2 == 0 && x4 & 16 == 0 && x6 & 56 == 40 => {
                    match ((instr >> 24) & 15, (instr >> 22) & 3, (instr >> 21) & 1, (instr >> 19) & 3, (instr >> 16) & 5, (instr >> 13) & 5, instr & 25) {
                        (_, _, _, x3, _, _, _) if x3 & 2 == 0 => {
                        }
                        (_, _, _, x3, _, _, _) if x3 == 2 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 5;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match opc {
                                x0 if x0 == 0 => {
                                    println!("encoding: SXTB_Z_P_Z__");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: UXTB_Z_P_Z__");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: SXTH_Z_P_Z__");
                                }
                                x0 if x0 == 3 => {
                                    println!("encoding: UXTH_Z_P_Z__");
                                }
                                x0 if x0 == 4 => {
                                    println!("encoding: SXTW_Z_P_Z__");
                                }
                                x0 if x0 == 5 => {
                                    println!("encoding: UXTW_Z_P_Z__");
                                }
                                x0 if x0 == 6 => {
                                    println!("encoding: ABS_Z_P_Z__");
                                }
                                x0 if x0 == 7 => {
                                    println!("encoding: NEG_Z_P_Z__");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: CLS_Z_P_Z__");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: CLZ_Z_P_Z__");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: CNT_Z_P_Z__");
                                }
                                x0 if x0 == 3 => {
                                    println!("encoding: CNOT_Z_P_Z__");
                                }
                                x0 if x0 == 4 => {
                                    println!("encoding: FABS_Z_P_Z__");
                                }
                                x0 if x0 == 5 => {
                                    println!("encoding: FNEG_Z_P_Z__");
                                }
                                x0 if x0 == 6 => {
                                    println!("encoding: NOT_Z_P_Z__");
                                }
                                x0 if x0 == 7 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
                            }
                        }
                        _ => unreachable!()
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
                            println!("encoding: ADD_Z_ZZ__");
                        }
                        x0 if x0 == 1 => {
                            println!("encoding: SUB_Z_ZZ__");
                        }
                        x0 if x0 & 6 == 2 => {
                            println!("unallocated");
                        }
                        x0 if x0 == 4 => {
                            println!("encoding: SQADD_Z_ZZ__");
                        }
                        x0 if x0 == 5 => {
                            println!("encoding: UQADD_Z_ZZ__");
                        }
                        x0 if x0 == 6 => {
                            println!("encoding: SQSUB_Z_ZZ__");
                        }
                        x0 if x0 == 7 => {
                            println!("encoding: UQSUB_Z_ZZ__");
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 0 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 56 == 8 => {
                    match ((instr >> 24) & 15, (instr >> 22) & 3, (instr >> 21) & 1, (instr >> 16) & 9, (instr >> 13) & 5, (instr >> 12) & 1, (instr >> 10) & 3, instr & 19) {
                        (_, _, _, _, _, x5, _, _) if x5 == 0 => {
                        }
                        (_, _, _, _, _, x5, x6, _) if x5 == 1 && x6 == 0 => {
                            let opc = (instr >> 22) & 3;
                            let Zm = (instr >> 16) & 9;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match opc {
                                x0 if x0 == 0 => {
                                    println!("encoding: AND_Z_ZZ__");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: ORR_Z_ZZ__");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: EOR_Z_ZZ__");
                                }
                                x0 if x0 == 3 => {
                                    println!("encoding: BIC_Z_ZZ__");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, _, _, _, x5, x6, _) if x5 == 1 && x6 != 0 => {
                        }
                        _ => unreachable!()
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
                                    println!("encoding: INDEX_Z_II__");
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
                                    println!("encoding: INDEX_Z_RI__");
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
                                    println!("encoding: INDEX_Z_IR__");
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
                                    println!("encoding: INDEX_Z_RR__");
                                }
                            }
                        }
                        _ => unreachable!()
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
                                    println!("encoding: ADDVL_R_RI__");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: ADDPL_R_RI__");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, x1, _, _, _, _, x6, _) if x1 == 1 && x6 == 0 => {
                            let op = (instr >> 22) & 1;
                            let opc2 = (instr >> 16) & 9;
                            let imm6 = (instr >> 5) & 11;
                            let Rd = instr & 9;
                            match (op, opc2) {
                                (x0, x1) if x0 == 0 && x1 & 16 == 0 => {
                                    println!("unallocated");
                                }
                                (x0, x1) if x0 == 0 && x1 & 24 == 16 => {
                                    println!("unallocated");
                                }
                                (x0, x1) if x0 == 0 && x1 & 28 == 24 => {
                                    println!("unallocated");
                                }
                                (x0, x1) if x0 == 0 && x1 & 30 == 28 => {
                                    println!("unallocated");
                                }
                                (x0, x1) if x0 == 0 && x1 == 30 => {
                                    println!("unallocated");
                                }
                                (x0, x1) if x0 == 0 && x1 == 31 => {
                                    println!("encoding: RDVL_R_I__");
                                }
                                (x0, _) if x0 == 1 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, _, _, _, _, x6, _) if x6 == 1 => {
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 0 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 56 == 24 => {
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
                                    println!("encoding: ASR_Z_ZW__");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: LSR_Z_ZW__");
                                }
                                x0 if x0 == 2 => {
                                    println!("unallocated");
                                }
                                x0 if x0 == 3 => {
                                    println!("encoding: LSL_Z_ZW__");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: ASR_Z_ZI__");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: LSR_Z_ZI__");
                                }
                                x0 if x0 == 2 => {
                                    println!("unallocated");
                                }
                                x0 if x0 == 3 => {
                                    println!("encoding: LSL_Z_ZI__");
                                }
                                _ => unreachable!()
                            }
                        }
                        _ => unreachable!()
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
                            println!("encoding: ADR_Z_AZ_D_s32_scaled");
                        }
                        x0 if x0 == 1 => {
                            println!("encoding: ADR_Z_AZ_D_u32_scaled");
                        }
                        x0 if x0 & 2 == 2 => {
                            println!("encoding: ADR_Z_AZ_SD_same_scaled");
                        }
                        _ => unreachable!()
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
                                    println!("encoding: FTSSEL_Z_ZZ__");
                                }
                                x0 if x0 == 1 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, _, _, _, x5, _) if x5 == 2 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 9;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match opc {
                                x0 if x0 == 0 => {
                                    println!("encoding: FEXPA_Z_Z__");
                                }
                                x0 if x0 == 1 => {
                                    println!("unallocated");
                                }
                                x0 if x0 & 30 == 2 => {
                                    println!("unallocated");
                                }
                                x0 if x0 & 28 == 4 => {
                                    println!("unallocated");
                                }
                                x0 if x0 & 24 == 8 => {
                                    println!("unallocated");
                                }
                                x0 if x0 & 16 == 16 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, _, _, _, x5, _) if x5 == 3 => {
                            let opc = (instr >> 22) & 3;
                            let opc2 = (instr >> 16) & 9;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match (opc, opc2) {
                                (x0, x1) if x0 == 0 && x1 == 0 => {
                                    println!("encoding: MOVPRFX_Z_Z__");
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    println!("unallocated");
                                }
                                (x0, x1) if x0 == 0 && x1 & 30 == 2 => {
                                    println!("unallocated");
                                }
                                (x0, x1) if x0 == 0 && x1 & 28 == 4 => {
                                    println!("unallocated");
                                }
                                (x0, x1) if x0 == 0 && x1 & 24 == 8 => {
                                    println!("unallocated");
                                }
                                (x0, x1) if x0 == 0 && x1 & 16 == 16 => {
                                    println!("unallocated");
                                }
                                (x0, _) if x0 == 1 => {
                                    println!("unallocated");
                                }
                                (x0, _) if x0 & 2 == 2 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
                            }
                        }
                        _ => unreachable!()
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
                                    println!("unallocated");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    println!("encoding: SQINCH_Z_ZS__");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    println!("encoding: UQINCH_Z_ZS__");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: SQDECH_Z_ZS__");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: UQDECH_Z_ZS__");
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                                    println!("encoding: SQINCW_Z_ZS__");
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                                    println!("encoding: UQINCW_Z_ZS__");
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: SQDECW_Z_ZS__");
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: UQDECW_Z_ZS__");
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 0 => {
                                    println!("encoding: SQINCD_Z_ZS__");
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 1 => {
                                    println!("encoding: UQINCD_Z_ZS__");
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: SQDECD_Z_ZS__");
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: UQDECD_Z_ZS__");
                                }
                                _ => unreachable!()
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
                                    println!("unallocated");
                                }
                                (x0, x1) if x0 == 0 && x1 == 0 => {
                                    println!("encoding: CNTB_R_S__");
                                }
                                (x0, x1) if x0 == 1 && x1 == 0 => {
                                    println!("encoding: CNTH_R_S__");
                                }
                                (x0, x1) if x0 == 2 && x1 == 0 => {
                                    println!("encoding: CNTW_R_S__");
                                }
                                (x0, x1) if x0 == 3 && x1 == 0 => {
                                    println!("encoding: CNTD_R_S__");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, _, x3, _, _, x6, _) if x3 == 0 && x6 == 5 => {
                        }
                        (_, _, _, x3, _, _, x6, _) if x3 == 1 && x6 == 0 => {
                            let size = (instr >> 22) & 3;
                            let imm4 = (instr >> 16) & 7;
                            let D = (instr >> 10) & 1;
                            let pattern = (instr >> 5) & 9;
                            let Zdn = instr & 9;
                            match (size, D) {
                                (x0, _) if x0 == 0 => {
                                    println!("unallocated");
                                }
                                (x0, x1) if x0 == 1 && x1 == 0 => {
                                    println!("encoding: INCH_Z_ZS__");
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    println!("encoding: DECH_Z_ZS__");
                                }
                                (x0, x1) if x0 == 2 && x1 == 0 => {
                                    println!("encoding: INCW_Z_ZS__");
                                }
                                (x0, x1) if x0 == 2 && x1 == 1 => {
                                    println!("encoding: DECW_Z_ZS__");
                                }
                                (x0, x1) if x0 == 3 && x1 == 0 => {
                                    println!("encoding: INCD_Z_ZS__");
                                }
                                (x0, x1) if x0 == 3 && x1 == 1 => {
                                    println!("encoding: DECD_Z_ZS__");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: INCB_R_RS__");
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    println!("encoding: DECB_R_RS__");
                                }
                                (x0, x1) if x0 == 1 && x1 == 0 => {
                                    println!("encoding: INCH_R_RS__");
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    println!("encoding: DECH_R_RS__");
                                }
                                (x0, x1) if x0 == 2 && x1 == 0 => {
                                    println!("encoding: INCW_R_RS__");
                                }
                                (x0, x1) if x0 == 2 && x1 == 1 => {
                                    println!("encoding: DECW_R_RS__");
                                }
                                (x0, x1) if x0 == 3 && x1 == 0 => {
                                    println!("encoding: INCD_R_RS__");
                                }
                                (x0, x1) if x0 == 3 && x1 == 1 => {
                                    println!("encoding: DECD_R_RS__");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, _, x3, _, _, x6, _) if x3 == 1 && x6 & 3 == 1 => {
                        }
                        (_, _, _, _, _, _, x6, _) if x6 & 6 == 2 => {
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
                                    println!("encoding: SQINCB_R_RS_SX");
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 => {
                                    println!("encoding: UQINCB_R_RS_UW");
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => {
                                    println!("encoding: SQDECB_R_RS_SX");
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 => {
                                    println!("encoding: UQDECB_R_RS_UW");
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 0 => {
                                    println!("encoding: SQINCB_R_RS_X");
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 1 => {
                                    println!("encoding: UQINCB_R_RS_X");
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 0 => {
                                    println!("encoding: SQDECB_R_RS_X");
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 1 => {
                                    println!("encoding: UQDECB_R_RS_X");
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 => {
                                    println!("encoding: SQINCH_R_RS_SX");
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 => {
                                    println!("encoding: UQINCH_R_RS_UW");
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 => {
                                    println!("encoding: SQDECH_R_RS_SX");
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 => {
                                    println!("encoding: UQDECH_R_RS_UW");
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 0 => {
                                    println!("encoding: SQINCH_R_RS_X");
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 1 => {
                                    println!("encoding: UQINCH_R_RS_X");
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 0 => {
                                    println!("encoding: SQDECH_R_RS_X");
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 1 => {
                                    println!("encoding: UQDECH_R_RS_X");
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 => {
                                    println!("encoding: SQINCW_R_RS_SX");
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 => {
                                    println!("encoding: UQINCW_R_RS_UW");
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 => {
                                    println!("encoding: SQDECW_R_RS_SX");
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 => {
                                    println!("encoding: UQDECW_R_RS_UW");
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 1 && x2 == 0 && x3 == 0 => {
                                    println!("encoding: SQINCW_R_RS_X");
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 1 && x2 == 0 && x3 == 1 => {
                                    println!("encoding: UQINCW_R_RS_X");
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 1 && x2 == 1 && x3 == 0 => {
                                    println!("encoding: SQDECW_R_RS_X");
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 1 && x2 == 1 && x3 == 1 => {
                                    println!("encoding: UQDECW_R_RS_X");
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 => {
                                    println!("encoding: SQINCD_R_RS_SX");
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 => {
                                    println!("encoding: UQINCD_R_RS_UW");
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 => {
                                    println!("encoding: SQDECD_R_RS_SX");
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 => {
                                    println!("encoding: UQDECD_R_RS_UW");
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 1 && x2 == 0 && x3 == 0 => {
                                    println!("encoding: SQINCD_R_RS_X");
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 1 && x2 == 0 && x3 == 1 => {
                                    println!("encoding: UQINCD_R_RS_X");
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 1 && x2 == 1 && x3 == 0 => {
                                    println!("encoding: SQDECD_R_RS_X");
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 1 && x2 == 1 && x3 == 1 => {
                                    println!("encoding: UQDECD_R_RS_X");
                                }
                                _ => unreachable!()
                            }
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, _, x4, _, _, _) if x0 == 0 && x2 & 2 == 2 && x4 & 24 == 0 => {
                    match ((instr >> 24) & 15, (instr >> 22) & 3, (instr >> 20) & 3, (instr >> 18) & 3, instr & 35) {
                        (_, x1, _, x3, _) if x1 == 3 && x3 == 0 => {
                            let imm13 = (instr >> 5) & 25;
                            let Zd = instr & 9;
                            match () {
                                () => {
                                    println!("encoding: DUPM_Z_I__");
                                }
                            }
                        }
                        (_, x1, _, x3, _) if x1 != 3 && x3 == 0 => {
                            let opc = (instr >> 22) & 3;
                            let imm13 = (instr >> 5) & 25;
                            let Zdn = instr & 9;
                            match opc {
                                x0 if x0 == 0 => {
                                    println!("encoding: ORR_Z_ZI__");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: EOR_Z_ZI__");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: AND_Z_ZI__");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, _, x3, _) if x3 != 0 => {
                        }
                        _ => unreachable!()
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
                                    println!("encoding: CPY_Z_O_I__");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: CPY_Z_P_I__");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, _, _, x4, _) if x4 & 6 == 4 => {
                        }
                        (_, _, _, _, x4, _) if x4 == 6 => {
                            let size = (instr >> 22) & 3;
                            let Pg = (instr >> 16) & 7;
                            let imm8 = (instr >> 5) & 15;
                            let Zd = instr & 9;
                            match () {
                                () => {
                                    println!("encoding: FCPY_Z_P_I__");
                                }
                            }
                        }
                        (_, _, _, _, x4, _) if x4 == 7 => {
                        }
                        _ => unreachable!()
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
                                    println!("encoding: DUP_Z_R__");
                                }
                            }
                        }
                        (_, _, _, x3, x4, x5, _, x7, x8, _) if x3 == 0 && x4 == 2 && x5 == 0 && x7 == 1 && x8 == 2 => {
                            let size = (instr >> 22) & 3;
                            let Rm = (instr >> 5) & 9;
                            let Zdn = instr & 9;
                            match () {
                                () => {
                                    println!("encoding: INSR_Z_R__");
                                }
                            }
                        }
                        (_, _, _, x3, x4, x5, _, x7, x8, _) if x3 == 0 && x4 & 1 == 0 && x5 == 0 && x7 == 0 && x8 == 1 => {
                        }
                        (_, _, _, x3, x4, x5, _, x7, x8, _) if x3 == 0 && x4 & 1 == 0 && x5 == 0 && x7 == 1 && x8 & 1 == 1 => {
                        }
                        (_, _, _, x3, x4, _, _, x7, x8, _) if x3 == 0 && x4 & 1 == 1 && x7 == 1 && x8 & 2 == 2 => {
                        }
                        (_, _, _, x3, x4, _, _, _, x8, _) if x3 == 0 && x4 & 1 == 1 && x8 == 1 => {
                        }
                        (_, _, _, x3, _, x5, _, x7, x8, _) if x3 == 0 && x5 == 1 && x7 == 1 && x8 & 2 == 2 => {
                        }
                        (_, _, _, x3, _, x5, _, _, x8, _) if x3 == 0 && x5 == 1 && x8 == 1 => {
                        }
                        (_, _, _, x3, _, _, _, x7, x8, _) if x3 == 0 && x7 == 0 && x8 & 2 == 2 => {
                        }
                        (_, _, _, x3, _, _, _, _, x8, _) if x3 == 1 && x8 != 0 => {
                        }
                        (_, _, _, x3, x4, _, _, x7, x8, _) if x3 == 2 && x4 & 2 == 0 && x7 == 0 && x8 == 1 => {
                        }
                        (_, _, _, x3, x4, _, _, x7, x8, _) if x3 == 2 && x4 & 2 == 0 && x7 == 1 && x8 == 2 => {
                            let size = (instr >> 22) & 3;
                            let U = (instr >> 17) & 1;
                            let H = (instr >> 16) & 1;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match (U, H) {
                                (x0, x1) if x0 == 0 && x1 == 0 => {
                                    println!("encoding: SUNPKLO_Z_Z__");
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    println!("encoding: SUNPKHI_Z_Z__");
                                }
                                (x0, x1) if x0 == 1 && x1 == 0 => {
                                    println!("encoding: UUNPKLO_Z_Z__");
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    println!("encoding: UUNPKHI_Z_Z__");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, _, x3, x4, _, _, x7, x8, _) if x3 == 2 && x4 & 2 == 0 && x7 == 1 && x8 & 1 == 1 => {
                        }
                        (_, _, _, x3, x4, x5, _, x7, x8, _) if x3 == 2 && x4 == 2 && x5 == 0 && x7 == 0 && x8 == 1 => {
                        }
                        (_, _, _, x3, x4, x5, _, x7, x8, _) if x3 == 2 && x4 == 2 && x5 == 0 && x7 == 1 && x8 == 2 => {
                            let size = (instr >> 22) & 3;
                            let Vm = (instr >> 5) & 9;
                            let Zdn = instr & 9;
                            match () {
                                () => {
                                    println!("encoding: INSR_Z_V__");
                                }
                            }
                        }
                        (_, _, _, x3, x4, x5, _, x7, x8, _) if x3 == 2 && x4 == 2 && x5 == 0 && x7 == 1 && x8 & 1 == 1 => {
                        }
                        (_, _, _, x3, x4, _, _, x7, x8, _) if x3 == 2 && x4 == 3 && x7 == 1 && x8 & 2 == 2 => {
                        }
                        (_, _, _, x3, x4, _, _, _, x8, _) if x3 == 2 && x4 == 3 && x8 == 1 => {
                        }
                        (_, _, _, x3, x4, x5, _, x7, x8, _) if x3 == 2 && x4 & 2 == 2 && x5 == 1 && x7 == 1 && x8 & 2 == 2 => {
                        }
                        (_, _, _, x3, x4, x5, _, _, x8, _) if x3 == 2 && x4 & 2 == 2 && x5 == 1 && x8 == 1 => {
                        }
                        (_, _, _, x3, x4, x5, _, x7, x8, _) if x3 == 3 && x4 == 0 && x5 == 0 && x7 == 0 && x8 == 1 => {
                        }
                        (_, _, _, x3, x4, x5, _, x7, x8, _) if x3 == 3 && x4 == 0 && x5 == 0 && x7 == 1 && x8 == 2 => {
                            let size = (instr >> 22) & 3;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match () {
                                () => {
                                    println!("encoding: REV_Z_Z__");
                                }
                            }
                        }
                        (_, _, _, x3, x4, x5, _, x7, x8, _) if x3 == 3 && x4 == 0 && x5 == 0 && x7 == 1 && x8 & 1 == 1 => {
                        }
                        (_, _, _, x3, x4, x5, _, x7, x8, _) if x3 == 3 && x4 & 2 == 0 && x5 == 1 && x7 == 1 && x8 & 2 == 2 => {
                        }
                        (_, _, _, x3, x4, x5, _, _, x8, _) if x3 == 3 && x4 & 2 == 0 && x5 == 1 && x8 == 1 => {
                        }
                        (_, _, _, x3, x4, _, _, x7, x8, _) if x3 == 3 && x4 != 0 && x7 == 1 && x8 & 2 == 2 => {
                        }
                        (_, _, _, x3, x4, _, _, _, x8, _) if x3 == 3 && x4 != 0 && x8 == 1 => {
                        }
                        (_, _, _, x3, _, _, _, x7, x8, _) if x3 & 2 == 2 && x7 == 0 && x8 & 2 == 2 => {
                        }
                        (_, _, _, _, _, _, _, x7, x8, _) if x7 == 0 && x8 == 0 => {
                            let imm2 = (instr >> 22) & 3;
                            let tsz = (instr >> 16) & 9;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match () {
                                () => {
                                    println!("encoding: DUP_Z_Zi__");
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
                                    println!("encoding: TBL_Z_ZZ_1");
                                }
                            }
                        }
                        _ => unreachable!()
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
                                    println!("encoding: PUNPKLO_P_P__");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: PUNPKHI_P_P__");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, x1, _, x3, _, x5, _, x7, _) if x1 == 1 && x3 & 30 == 16 && x5 == 0 && x7 == 0 => {
                        }
                        (_, x1, _, x3, _, x5, _, x7, _) if x1 == 2 && x3 & 30 == 16 && x5 == 0 && x7 == 0 => {
                        }
                        (_, x1, _, x3, _, x5, _, x7, _) if x1 == 3 && x3 & 30 == 16 && x5 == 0 && x7 == 0 => {
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
                                    println!("encoding: ZIP1_P_PP__");
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    println!("encoding: ZIP2_P_PP__");
                                }
                                (x0, x1) if x0 == 1 && x1 == 0 => {
                                    println!("encoding: UZP1_P_PP__");
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    println!("encoding: UZP2_P_PP__");
                                }
                                (x0, x1) if x0 == 2 && x1 == 0 => {
                                    println!("encoding: TRN1_P_PP__");
                                }
                                (x0, x1) if x0 == 2 && x1 == 1 => {
                                    println!("encoding: TRN2_P_PP__");
                                }
                                (x0, _) if x0 == 3 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, _, x3, _, x5, _, x7, _) if x3 & 16 == 0 && x5 & 1 == 1 && x7 == 0 => {
                        }
                        (_, _, _, x3, _, x5, _, x7, _) if x3 == 20 && x5 == 0 && x7 == 0 => {
                            let size = (instr >> 22) & 3;
                            let Pn = (instr >> 5) & 7;
                            let Pd = instr & 7;
                            match () {
                                () => {
                                    println!("encoding: REV_P_P__");
                                }
                            }
                        }
                        (_, _, _, x3, _, x5, _, x7, _) if x3 == 21 && x5 == 0 && x7 == 0 => {
                        }
                        (_, _, _, x3, _, x5, _, x7, _) if x3 & 26 == 16 && x5 == 8 && x7 == 0 => {
                        }
                        (_, _, _, x3, _, x5, _, x7, _) if x3 & 26 == 16 && x5 & 7 == 4 && x7 == 0 => {
                        }
                        (_, _, _, x3, _, x5, _, x7, _) if x3 & 26 == 16 && x5 & 3 == 2 && x7 == 0 => {
                        }
                        (_, _, _, x3, _, x5, _, x7, _) if x3 & 26 == 16 && x5 & 1 == 1 && x7 == 0 => {
                        }
                        (_, _, _, x3, _, _, _, x7, _) if x3 & 26 == 18 && x7 == 0 => {
                        }
                        (_, _, _, x3, _, _, _, x7, _) if x3 & 24 == 24 && x7 == 0 => {
                        }
                        (_, _, _, _, _, _, _, x7, _) if x7 == 1 => {
                        }
                        _ => unreachable!()
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
                            println!("encoding: ZIP1_Z_ZZ__");
                        }
                        x0 if x0 == 1 => {
                            println!("encoding: ZIP2_Z_ZZ__");
                        }
                        x0 if x0 == 2 => {
                            println!("encoding: UZP1_Z_ZZ__");
                        }
                        x0 if x0 == 3 => {
                            println!("encoding: UZP2_Z_ZZ__");
                        }
                        x0 if x0 == 4 => {
                            println!("encoding: TRN1_Z_ZZ__");
                        }
                        x0 if x0 == 5 => {
                            println!("encoding: TRN2_Z_ZZ__");
                        }
                        x0 if x0 & 6 == 6 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
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
                                    println!("encoding: CPY_Z_P_V__");
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
                                    println!("encoding: COMPACT_Z_P_Z__");
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
                                    println!("encoding: LASTA_R_P_Z__");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: LASTB_R_P_Z__");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: LASTA_V_P_Z__");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: LASTB_V_P_Z__");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: REVB_Z_Z__");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: REVH_Z_Z__");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: REVW_Z_Z__");
                                }
                                x0 if x0 == 3 => {
                                    println!("encoding: RBIT_Z_P_Z__");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, _, x3, x4, _, _, x7, _) if x3 == 0 && x4 & 6 == 2 && x7 == 1 => {
                        }
                        (_, _, _, x3, x4, x5, _, x7, _) if x3 == 0 && x4 == 4 && x5 == 0 && x7 == 1 => {
                            let size = (instr >> 22) & 3;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match () {
                                () => {
                                    println!("encoding: CPY_Z_P_R__");
                                }
                            }
                        }
                        (_, _, _, x3, x4, x5, _, x7, _) if x3 == 0 && x4 == 4 && x5 == 1 && x7 == 1 => {
                        }
                        (_, _, _, x3, x4, _, _, x7, _) if x3 == 0 && x4 == 4 && x7 == 0 => {
                            let size = (instr >> 22) & 3;
                            let B = (instr >> 16) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Zm = (instr >> 5) & 9;
                            let Zdn = instr & 9;
                            match B {
                                x0 if x0 == 0 => {
                                    println!("encoding: CLASTA_Z_P_ZZ__");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: CLASTB_Z_P_ZZ__");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: CLASTA_V_P_Z__");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: CLASTB_V_P_Z__");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, _, x3, x4, x5, _, x7, _) if x3 == 0 && x4 == 6 && x5 == 0 && x7 == 0 => {
                            let size = (instr >> 22) & 3;
                            let Pg = (instr >> 10) & 5;
                            let Zm = (instr >> 5) & 9;
                            let Zdn = instr & 9;
                            match () {
                                () => {
                                    println!("encoding: SPLICE_Z_P_ZZ_Des");
                                }
                            }
                        }
                        (_, _, _, x3, x4, x5, _, x7, _) if x3 == 0 && x4 == 6 && x5 == 0 && x7 == 1 => {
                        }
                        (_, _, _, x3, x4, x5, _, _, _) if x3 == 0 && x4 == 6 && x5 == 1 => {
                        }
                        (_, _, _, x3, x4, x5, _, _, _) if x3 == 0 && x4 == 7 && x5 == 0 => {
                        }
                        (_, _, _, x3, x4, x5, _, _, _) if x3 == 0 && x4 == 7 && x5 == 1 => {
                        }
                        (_, _, _, x3, x4, _, _, x7, _) if x3 == 0 && x4 & 3 == 1 && x7 == 1 => {
                        }
                        (_, _, _, x3, x4, _, _, x7, _) if x3 == 1 && x4 == 0 && x7 == 0 => {
                        }
                        (_, _, _, x3, x4, _, _, x7, _) if x3 == 1 && x4 == 0 && x7 == 1 => {
                            let size = (instr >> 22) & 3;
                            let B = (instr >> 16) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Zm = (instr >> 5) & 9;
                            let Rdn = instr & 9;
                            match B {
                                x0 if x0 == 0 => {
                                    println!("encoding: CLASTA_R_P_Z__");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: CLASTB_R_P_Z__");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, _, x3, x4, _, _, _, _) if x3 == 1 && x4 != 0 => {
                        }
                        _ => unreachable!()
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
                            println!("encoding: SEL_Z_P_ZZ__");
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
                                    println!("encoding: EXT_Z_ZI_Des");
                                }
                            }
                        }
                        (_, x1, _, _, _, _) if x1 == 1 => {
                        }
                        _ => unreachable!()
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
                            println!("encoding: ZIP1_Z_ZZ_Q");
                        }
                        (x0, x1) if x0 == 0 && x1 == 1 => {
                            println!("encoding: ZIP2_Z_ZZ_Q");
                        }
                        (x0, x1) if x0 == 0 && x1 == 2 => {
                            println!("encoding: UZP1_Z_ZZ_Q");
                        }
                        (x0, x1) if x0 == 0 && x1 == 3 => {
                            println!("encoding: UZP2_Z_ZZ_Q");
                        }
                        (x0, x1) if x0 == 0 && x1 & 6 == 4 => {
                            println!("unallocated");
                        }
                        (x0, x1) if x0 == 0 && x1 == 6 => {
                            println!("encoding: TRN1_Z_ZZ_Q");
                        }
                        (x0, x1) if x0 == 0 && x1 == 7 => {
                            println!("encoding: TRN2_Z_ZZ_Q");
                        }
                        (x0, _) if x0 == 1 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
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
                                    println!("encoding: CMPHS_P_P_ZZ__");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                                    println!("encoding: CMPHI_P_P_ZZ__");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: CMPEQ_P_P_ZW__");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: CMPNE_P_P_ZW__");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    println!("encoding: CMPGE_P_P_ZZ__");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    println!("encoding: CMPGT_P_P_ZZ__");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: CMPEQ_P_P_ZZ__");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: CMPNE_P_P_ZZ__");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: CMPGE_P_P_ZW__");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                                    println!("encoding: CMPGT_P_P_ZW__");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: CMPLT_P_P_ZW__");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: CMPLE_P_P_ZW__");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    println!("encoding: CMPHS_P_P_ZW__");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    println!("encoding: CMPHI_P_P_ZW__");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: CMPLO_P_P_ZW__");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: CMPLS_P_P_ZW__");
                                }
                                _ => unreachable!()
                            }
                        }
                        _ => unreachable!()
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
                            println!("encoding: CMPHS_P_P_ZI__");
                        }
                        (x0, x1) if x0 == 0 && x1 == 1 => {
                            println!("encoding: CMPHI_P_P_ZI__");
                        }
                        (x0, x1) if x0 == 1 && x1 == 0 => {
                            println!("encoding: CMPLO_P_P_ZI__");
                        }
                        (x0, x1) if x0 == 1 && x1 == 1 => {
                            println!("encoding: CMPLS_P_P_ZI__");
                        }
                        _ => unreachable!()
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
                            println!("encoding: CMPGE_P_P_ZI__");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            println!("encoding: CMPGT_P_P_ZI__");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            println!("encoding: CMPLT_P_P_ZI__");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            println!("encoding: CMPLE_P_P_ZI__");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            println!("encoding: CMPEQ_P_P_ZI__");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            println!("encoding: CMPNE_P_P_ZI__");
                        }
                        (x0, x1, _) if x0 == 1 && x1 == 1 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
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
                            println!("encoding: AND_P_P_PP_Z");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            println!("encoding: BIC_P_P_PP_Z");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            println!("encoding: EOR_P_P_PP_Z");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 => {
                            println!("encoding: SEL_P_P_PP__");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 0 => {
                            println!("encoding: ANDS_P_P_PP_Z");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 1 => {
                            println!("encoding: BICS_P_P_PP_Z");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 0 => {
                            println!("encoding: EORS_P_P_PP_Z");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            println!("encoding: ORR_P_P_PP_Z");
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            println!("encoding: ORN_P_P_PP_Z");
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            println!("encoding: NOR_P_P_PP_Z");
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 => {
                            println!("encoding: NAND_P_P_PP_Z");
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 0 => {
                            println!("encoding: ORRS_P_P_PP_Z");
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 1 => {
                            println!("encoding: ORNS_P_P_PP_Z");
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 0 => {
                            println!("encoding: NORS_P_P_PP_Z");
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 1 => {
                            println!("encoding: NANDS_P_P_PP_Z");
                        }
                        _ => unreachable!()
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
                                    println!("encoding: BRKPA_P_P_PP__");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                                    println!("encoding: BRKPB_P_P_PP__");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: BRKPAS_P_P_PP__");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: BRKPBS_P_P_PP__");
                                }
                                (x0, _, _) if x0 == 1 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, _, _, _, _, x6, _) if x6 == 1 => {
                        }
                        _ => unreachable!()
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
                                    println!("encoding: BRKN_P_P_PP__");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: BRKNS_P_P_PP__");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, x1, _, _, x4, _, _, x7, _, x9, _) if x1 == 0 && x4 == 8 && x7 == 0 && x9 == 1 => {
                        }
                        (_, x1, _, _, x4, _, _, x7, _, _, _) if x1 == 0 && x4 & 7 == 0 && x7 == 1 => {
                        }
                        (_, x1, _, _, x4, _, _, _, _, _, _) if x1 == 0 && x4 & 4 == 4 => {
                        }
                        (_, x1, _, _, x4, _, _, _, _, _, _) if x1 == 0 && x4 & 2 == 2 => {
                        }
                        (_, x1, _, _, x4, _, _, _, _, _, _) if x1 == 0 && x4 & 1 == 1 => {
                        }
                        (_, x1, _, _, x4, _, _, x7, _, _, _) if x1 == 1 && x4 == 0 && x7 == 1 => {
                        }
                        (_, x1, _, _, x4, _, _, _, _, _, _) if x1 == 1 && x4 != 0 => {
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
                                    println!("unallocated");
                                }
                                (x0, x1, _) if x0 == 0 && x1 == 0 => {
                                    println!("encoding: BRKA_P_P_P__");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: BRKAS_P_P_P_Z");
                                }
                                (x0, x1, _) if x0 == 1 && x1 == 0 => {
                                    println!("encoding: BRKB_P_P_P__");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: BRKBS_P_P_P_Z");
                                }
                                _ => unreachable!()
                            }
                        }
                        _ => unreachable!()
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
                                    println!("unallocated");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: PTEST__P_P__");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    println!("unallocated");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 & 14 == 2 => {
                                    println!("unallocated");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 & 12 == 4 => {
                                    println!("unallocated");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 & 8 == 8 => {
                                    println!("unallocated");
                                }
                                (x0, _, _) if x0 == 1 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, _, x3, _, _, x6, _, x8, _) if x3 == 4 && x6 & 1 == 0 && x8 == 0 => {
                        }
                        (_, _, _, x3, _, _, x6, _, x8, _) if x3 & 11 == 2 && x6 & 1 == 0 && x8 == 0 => {
                        }
                        (_, _, _, x3, _, _, x6, _, x8, _) if x3 & 9 == 1 && x6 & 1 == 0 && x8 == 0 => {
                        }
                        (_, _, _, x3, _, _, x6, _, x8, _) if x3 & 8 == 0 && x6 & 1 == 1 && x8 == 0 => {
                        }
                        (_, _, _, x3, _, x5, x6, _, x8, _) if x3 == 8 && x5 == 0 && x6 == 0 && x8 == 0 => {
                            let op = (instr >> 23) & 1;
                            let S = (instr >> 22) & 1;
                            let Pg = (instr >> 5) & 7;
                            let Pdn = instr & 7;
                            match (op, S) {
                                (x0, x1) if x0 == 0 && x1 == 0 => {
                                    println!("unallocated");
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    println!("encoding: PFIRST_P_P_P__");
                                }
                                (x0, _) if x0 == 1 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, _, x3, _, x5, x6, _, x8, _) if x3 == 8 && x5 == 0 && x6 != 0 && x8 == 0 => {
                        }
                        (_, _, _, x3, _, x5, x6, x7, x8, _) if x3 == 8 && x5 == 4 && x6 == 2 && x7 == 0 && x8 == 0 => {
                            let op = (instr >> 23) & 1;
                            let S = (instr >> 22) & 1;
                            let Pd = instr & 7;
                            match (op, S) {
                                (x0, x1) if x0 == 0 && x1 == 0 => {
                                    println!("encoding: PFALSE_P__");
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    println!("unallocated");
                                }
                                (x0, _) if x0 == 1 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, _, x3, _, x5, x6, x7, x8, _) if x3 == 8 && x5 == 4 && x6 == 2 && x7 != 0 && x8 == 0 => {
                        }
                        (_, _, _, x3, _, x5, x6, _, x8, _) if x3 == 8 && x5 == 6 && x6 == 0 && x8 == 0 => {
                            let op = (instr >> 23) & 1;
                            let S = (instr >> 22) & 1;
                            let Pg = (instr >> 5) & 7;
                            let Pd = instr & 7;
                            match (op, S) {
                                (x0, x1) if x0 == 0 && x1 == 0 => {
                                    println!("encoding: RDFFR_P_P_F__");
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    println!("encoding: RDFFRS_P_P_F__");
                                }
                                (x0, _) if x0 == 1 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, _, x3, _, x5, x6, _, x8, _) if x3 == 9 && x5 == 0 && x6 & 2 == 0 && x8 == 0 => {
                        }
                        (_, _, _, x3, _, x5, x6, _, x8, _) if x3 == 9 && x5 == 0 && x6 == 2 && x8 == 0 => {
                            let size = (instr >> 22) & 3;
                            let Pg = (instr >> 5) & 7;
                            let Pdn = instr & 7;
                            match () {
                                () => {
                                    println!("encoding: PNEXT_P_P_P__");
                                }
                            }
                        }
                        (_, _, _, x3, _, x5, x6, _, x8, _) if x3 == 9 && x5 == 0 && x6 == 3 && x8 == 0 => {
                        }
                        (_, _, _, x3, _, x5, x6, _, x8, _) if x3 == 9 && x5 == 4 && x6 == 2 && x8 == 0 => {
                        }
                        (_, _, _, x3, _, x5, x6, x7, x8, _) if x3 == 9 && x5 == 6 && x6 == 0 && x7 == 0 && x8 == 0 => {
                            let op = (instr >> 23) & 1;
                            let S = (instr >> 22) & 1;
                            let Pd = instr & 7;
                            match (op, S) {
                                (x0, x1) if x0 == 0 && x1 == 0 => {
                                    println!("encoding: RDFFR_P_F__");
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    println!("unallocated");
                                }
                                (x0, _) if x0 == 1 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, _, x3, _, x5, x6, x7, x8, _) if x3 == 9 && x5 == 6 && x6 == 0 && x7 != 0 && x8 == 0 => {
                        }
                        (_, _, _, x3, _, x5, _, _, x8, _) if x3 & 14 == 8 && x5 == 2 && x8 == 0 => {
                        }
                        (_, _, _, x3, _, x5, x6, _, x8, _) if x3 & 14 == 8 && x5 == 4 && x6 & 2 == 0 && x8 == 0 => {
                            let size = (instr >> 22) & 3;
                            let S = (instr >> 16) & 1;
                            let pattern = (instr >> 5) & 9;
                            let Pd = instr & 7;
                            match S {
                                x0 if x0 == 0 => {
                                    println!("encoding: PTRUE_P_S__");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: PTRUES_P_S__");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, _, x3, _, x5, x6, _, x8, _) if x3 & 14 == 8 && x5 == 4 && x6 == 3 && x8 == 0 => {
                        }
                        (_, _, _, x3, _, x5, x6, _, x8, _) if x3 & 14 == 8 && x5 == 6 && x6 != 0 && x8 == 0 => {
                        }
                        (_, _, _, x3, _, x5, _, _, x8, _) if x3 & 14 == 8 && x5 & 1 == 1 && x8 == 0 => {
                        }
                        (_, _, _, x3, _, _, _, _, x8, _) if x3 & 14 == 12 && x8 == 0 => {
                        }
                        (_, _, _, x3, _, _, _, _, x8, _) if x3 & 10 == 10 && x8 == 0 => {
                        }
                        (_, _, _, _, _, _, _, _, x8, _) if x8 == 1 => {
                        }
                        _ => unreachable!()
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
                                    println!("unallocated");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: WHILELT_P_P_RR__");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: WHILELE_P_P_RR__");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: WHILELO_P_P_RR__");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: WHILELS_P_P_RR__");
                                }
                                _ => unreachable!()
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
                                    println!("unallocated");
                                }
                                (x0, x1) if x0 == 1 && x1 == 0 => {
                                    println!("encoding: CTERMEQ_RR__");
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    println!("encoding: CTERMNE_RR__");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, _, _, _, x5, x6, _, x8) if x5 == 1 && x6 == 0 && x8 != 0 => {
                        }
                        (_, _, _, _, _, x5, x6, _, _) if x5 == 1 && x6 != 0 => {
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 1 && x2 & 2 == 2 && x4 & 16 == 16 && x6 & 48 == 16 => {
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
                                    println!("encoding: ADD_Z_ZI__");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: SUB_Z_ZI__");
                                }
                                x0 if x0 == 2 => {
                                    println!("unallocated");
                                }
                                x0 if x0 == 3 => {
                                    println!("encoding: SUBR_Z_ZI__");
                                }
                                x0 if x0 == 4 => {
                                    println!("encoding: SQADD_Z_ZI__");
                                }
                                x0 if x0 == 5 => {
                                    println!("encoding: UQADD_Z_ZI__");
                                }
                                x0 if x0 == 6 => {
                                    println!("encoding: SQSUB_Z_ZI__");
                                }
                                x0 if x0 == 7 => {
                                    println!("encoding: UQSUB_Z_ZI__");
                                }
                                _ => unreachable!()
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
                                    println!("unallocated");
                                }
                                (x0, x1) if x0 == 0 && x1 == 0 => {
                                    println!("encoding: SMAX_Z_ZI__");
                                }
                                (x0, x1) if x0 == 1 && x1 == 0 => {
                                    println!("encoding: UMAX_Z_ZI__");
                                }
                                (x0, x1) if x0 == 2 && x1 == 0 => {
                                    println!("encoding: SMIN_Z_ZI__");
                                }
                                (x0, x1) if x0 == 3 && x1 == 0 => {
                                    println!("encoding: UMIN_Z_ZI__");
                                }
                                (x0, _) if x0 & 4 == 4 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: MUL_Z_ZI__");
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    println!("unallocated");
                                }
                                (x0, _) if x0 == 1 => {
                                    println!("unallocated");
                                }
                                (x0, _) if x0 & 6 == 2 => {
                                    println!("unallocated");
                                }
                                (x0, _) if x0 & 4 == 4 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: DUP_Z_I__");
                                }
                                x0 if x0 == 1 => {
                                    println!("unallocated");
                                }
                                x0 if x0 & 2 == 2 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: FDUP_Z_I__");
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    println!("unallocated");
                                }
                                (x0, _) if x0 == 1 => {
                                    println!("unallocated");
                                }
                                (x0, _) if x0 & 2 == 2 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
                            }
                        }
                        _ => unreachable!()
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
                            println!("encoding: CNTP_R_P_P__");
                        }
                        (x0, x1) if x0 == 0 && x1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, _) if x0 == 1 => {
                            println!("unallocated");
                        }
                        (x0, _) if x0 & 6 == 2 => {
                            println!("unallocated");
                        }
                        (x0, _) if x0 & 4 == 4 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
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
                                    println!("unallocated");
                                }
                                (_, _, x2) if x2 & 2 == 2 => {
                                    println!("unallocated");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                                    println!("encoding: SQINCP_Z_P_Z__");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: UQINCP_Z_P_Z__");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    println!("encoding: SQDECP_Z_P_Z__");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: UQDECP_Z_P_Z__");
                                }
                                _ => unreachable!()
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
                                    println!("unallocated");
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 => {
                                    println!("encoding: SQINCP_R_P_R_SX");
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => {
                                    println!("encoding: SQINCP_R_P_R_X");
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 0 => {
                                    println!("encoding: UQINCP_R_P_R_UW");
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 0 => {
                                    println!("encoding: UQINCP_R_P_R_X");
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 => {
                                    println!("encoding: SQDECP_R_P_R_SX");
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 => {
                                    println!("encoding: SQDECP_R_P_R_X");
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 0 => {
                                    println!("encoding: UQDECP_R_P_R_UW");
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 0 => {
                                    println!("encoding: UQDECP_R_P_R_X");
                                }
                                _ => unreachable!()
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
                                    println!("unallocated");
                                }
                                (x0, _, x2) if x0 == 0 && x2 & 2 == 2 => {
                                    println!("unallocated");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                                    println!("encoding: INCP_Z_P_Z__");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: DECP_Z_P_Z__");
                                }
                                (x0, _, _) if x0 == 1 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
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
                                    println!("unallocated");
                                }
                                (x0, _, x2) if x0 == 0 && x2 & 2 == 2 => {
                                    println!("unallocated");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                                    println!("encoding: INCP_R_P_R__");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: DECP_R_P_R__");
                                }
                                (x0, _, _) if x0 == 1 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
                            }
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 1 && x2 & 2 == 2 && x4 & 28 == 20 && x6 & 60 == 36 => {
                    match ((instr >> 24) & 15, (instr >> 22) & 3, (instr >> 19) & 5, (instr >> 18) & 1, (instr >> 16) & 3, (instr >> 12) & 7, (instr >> 9) & 5, (instr >> 5) & 7, instr & 9) {
                        (_, _, _, x3, x4, _, x6, _, x8) if x3 == 0 && x4 == 0 && x6 == 0 && x8 == 0 => {
                            let opc = (instr >> 22) & 3;
                            let Pn = (instr >> 5) & 7;
                            match opc {
                                x0 if x0 == 0 => {
                                    println!("encoding: WRFFR_F_P__");
                                }
                                x0 if x0 == 1 => {
                                    println!("unallocated");
                                }
                                x0 if x0 & 2 == 2 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, _, x3, x4, _, x6, x7, x8) if x3 == 1 && x4 == 0 && x6 == 0 && x7 == 0 && x8 == 0 => {
                            let opc = (instr >> 22) & 3;
                            match opc {
                                x0 if x0 == 0 => {
                                    println!("encoding: SETFFR_F__");
                                }
                                x0 if x0 == 1 => {
                                    println!("unallocated");
                                }
                                x0 if x0 & 2 == 2 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, _, x3, x4, _, x6, x7, x8) if x3 == 1 && x4 == 0 && x6 == 0 && x7 & 8 == 8 && x8 == 0 => {
                        }
                        (_, _, _, x3, x4, _, x6, x7, x8) if x3 == 1 && x4 == 0 && x6 == 0 && x7 & 4 == 4 && x8 == 0 => {
                        }
                        (_, _, _, x3, x4, _, x6, x7, x8) if x3 == 1 && x4 == 0 && x6 == 0 && x7 & 2 == 2 && x8 == 0 => {
                        }
                        (_, _, _, x3, x4, _, x6, x7, x8) if x3 == 1 && x4 == 0 && x6 == 0 && x7 & 1 == 1 && x8 == 0 => {
                        }
                        (_, _, _, _, x4, _, x6, _, x8) if x4 == 0 && x6 == 0 && x8 != 0 => {
                        }
                        (_, _, _, _, x4, _, x6, _, _) if x4 == 0 && x6 != 0 => {
                        }
                        (_, _, _, _, x4, _, _, _, _) if x4 != 0 => {
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 1 && x2 & 2 == 2 && x4 & 28 == 20 && x6 & 56 == 40 => {
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 1 && x2 & 2 == 2 && x4 & 24 == 24 && x6 & 48 == 32 => {
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
                                    println!("encoding: SDOT_Z_ZZZ__");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: UDOT_Z_ZZZ__");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, _, _, _, x5, x6, _, _) if x5 == 0 && x6 != 0 => {
                        }
                        (_, _, _, _, _, x5, x6, _, _) if x5 == 1 && x6 & 4 == 0 => {
                        }
                        (_, _, _, _, _, x5, x6, _, _) if x5 == 1 && x6 & 6 == 4 => {
                        }
                        (_, _, _, _, _, x5, x6, _, _) if x5 == 1 && x6 == 6 => {
                        }
                        (_, _, _, _, _, x5, x6, x7, _) if x5 == 1 && x6 == 7 && x7 == 0 => {
                            let size = (instr >> 22) & 3;
                            let Zm = (instr >> 16) & 9;
                            let Zn = (instr >> 5) & 9;
                            let Zda = instr & 9;
                            match size {
                                x0 if x0 & 2 == 0 => {
                                    println!("unallocated");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: USDOT_Z_ZZZ_S");
                                }
                                x0 if x0 == 3 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, _, _, _, x5, x6, x7, _) if x5 == 1 && x6 == 7 && x7 == 1 => {
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 2 && x2 & 2 == 0 && x4 & 16 == 0 && x6 & 32 == 32 => {
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
                                    println!("unallocated");
                                }
                                (x0, x1) if x0 == 2 && x1 == 0 => {
                                    println!("encoding: SDOT_Z_ZZZi_S");
                                }
                                (x0, x1) if x0 == 2 && x1 == 1 => {
                                    println!("encoding: UDOT_Z_ZZZi_S");
                                }
                                (x0, x1) if x0 == 3 && x1 == 0 => {
                                    println!("encoding: SDOT_Z_ZZZi_D");
                                }
                                (x0, x1) if x0 == 3 && x1 == 1 => {
                                    println!("encoding: UDOT_Z_ZZZi_D");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, _, _, x4, x5, _) if x4 == 0 && x5 == 1 => {
                        }
                        (_, _, _, _, x4, x5, _) if x4 == 0 && x5 == 2 => {
                        }
                        (_, _, _, _, x4, x5, _) if x4 == 0 && x5 == 3 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 9;
                            let U = (instr >> 10) & 1;
                            let Zn = (instr >> 5) & 9;
                            let Zda = instr & 9;
                            match (size, U) {
                                (x0, _) if x0 & 2 == 0 => {
                                    println!("unallocated");
                                }
                                (x0, x1) if x0 == 2 && x1 == 0 => {
                                    println!("encoding: USDOT_Z_ZZZi_S");
                                }
                                (x0, x1) if x0 == 2 && x1 == 1 => {
                                    println!("encoding: SUDOT_Z_ZZZi_S");
                                }
                                (x0, _) if x0 == 3 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, _, _, x4, _, _) if x4 != 0 => {
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 2 && x2 & 2 == 2 && x4 & 16 == 0 && x6 & 32 == 0 => {
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 2 && x2 & 2 == 2 && x4 & 16 == 0 && x6 & 48 == 32 => {
                    match ((instr >> 24) & 15, (instr >> 22) & 3, (instr >> 21) & 1, (instr >> 16) & 9, (instr >> 14) & 3, (instr >> 10) & 7, instr & 19) {
                        (_, _, _, _, _, x5, _) if x5 & 12 == 0 => {
                        }
                        (_, _, _, _, _, x5, _) if x5 & 14 == 4 => {
                        }
                        (_, _, _, _, _, x5, _) if x5 == 6 => {
                            let uns = (instr >> 22) & 3;
                            let Zm = (instr >> 16) & 9;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match uns {
                                x0 if x0 == 0 => {
                                    println!("encoding: SMMLA_Z_ZZZ__");
                                }
                                x0 if x0 == 1 => {
                                    println!("unallocated");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: USMMLA_Z_ZZZ__");
                                }
                                x0 if x0 == 3 => {
                                    println!("encoding: UMMLA_Z_ZZZ__");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, _, _, _, x5, _) if x5 == 7 => {
                        }
                        (_, _, _, _, _, x5, _) if x5 & 8 == 8 => {
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 2 && x2 & 2 == 2 && x4 & 16 == 0 && x6 & 48 == 48 => {
                }
                (x0, _, x2, _, x4, _, _, _) if x0 == 2 && x2 & 2 == 2 && x4 & 16 == 16 => {
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
                            println!("encoding: FCMLA_Z_P_ZZZ__");
                        }
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 26 == 2 && x6 & 32 == 32 => {
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 == 0 && x6 & 56 == 32 => {
                    let size = (instr >> 22) & 3;
                    let rot = (instr >> 16) & 1;
                    let Pg = (instr >> 10) & 5;
                    let Zm = (instr >> 5) & 9;
                    let Zdn = instr & 9;
                    match () {
                        () => {
                            println!("encoding: FCADD_Z_P_ZZ__");
                        }
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 == 0 && x6 & 56 == 40 => {
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 == 0 && x6 & 48 == 48 => {
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 == 1 && x6 & 32 == 32 => {
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 30 == 4 && x6 & 56 == 32 => {
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 30 == 4 && x6 & 56 == 40 => {
                    let opc = (instr >> 22) & 3;
                    let opc2 = (instr >> 16) & 3;
                    let Pg = (instr >> 10) & 5;
                    let Zn = (instr >> 5) & 9;
                    let Zd = instr & 9;
                    match (opc, opc2) {
                        (x0, _) if x0 & 2 == 0 => {
                            println!("unallocated");
                        }
                        (x0, x1) if x0 == 2 && x1 & 2 == 0 => {
                            println!("unallocated");
                        }
                        (x0, x1) if x0 == 2 && x1 == 2 => {
                            println!("encoding: BFCVTNT_Z_P_Z_S2BF");
                        }
                        (x0, x1) if x0 == 2 && x1 == 3 => {
                            println!("unallocated");
                        }
                        (x0, _) if x0 == 3 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 30 == 4 && x6 & 48 == 48 => {
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 24 == 8 && x6 & 32 == 32 => {
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 22 == 2 => {
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 62 == 0 => {
                    let size = (instr >> 22) & 3;
                    let opc = (instr >> 16) & 9;
                    let op = (instr >> 10) & 1;
                    let Zn = (instr >> 5) & 9;
                    let Zda = instr & 9;
                    match (size, op) {
                        (x0, x1) if x0 & 2 == 0 && x1 == 0 => {
                            println!("encoding: FMLA_Z_ZZZi_H");
                        }
                        (x0, x1) if x0 & 2 == 0 && x1 == 1 => {
                            println!("encoding: FMLS_Z_ZZZi_H");
                        }
                        (x0, x1) if x0 == 2 && x1 == 0 => {
                            println!("encoding: FMLA_Z_ZZZi_S");
                        }
                        (x0, x1) if x0 == 2 && x1 == 1 => {
                            println!("encoding: FMLS_Z_ZZZi_S");
                        }
                        (x0, x1) if x0 == 3 && x1 == 0 => {
                            println!("encoding: FMLA_Z_ZZZi_D");
                        }
                        (x0, x1) if x0 == 3 && x1 == 1 => {
                            println!("encoding: FMLS_Z_ZZZi_D");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        x0 if x0 == 2 => {
                            println!("encoding: FCMLA_Z_ZZZi_H");
                        }
                        x0 if x0 == 3 => {
                            println!("encoding: FCMLA_Z_ZZZi_S");
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 == 8 => {
                    let size = (instr >> 22) & 3;
                    let opc = (instr >> 16) & 9;
                    let Zn = (instr >> 5) & 9;
                    let Zd = instr & 9;
                    match size {
                        x0 if x0 & 2 == 0 => {
                            println!("encoding: FMUL_Z_ZZi_H");
                        }
                        x0 if x0 == 2 => {
                            println!("encoding: FMUL_Z_ZZi_S");
                        }
                        x0 if x0 == 3 => {
                            println!("encoding: FMUL_Z_ZZi_D");
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 == 9 => {
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 60 == 12 => {
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
                                    println!("unallocated");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: BFDOT_Z_ZZZi__");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, x1, _, _, _, _, x6, _, x8, _) if x1 == 0 && x6 == 0 && x8 != 0 => {
                        }
                        (_, x1, _, _, _, _, x6, _, _, _) if x1 == 0 && x6 == 1 => {
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
                                    println!("unallocated");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    println!("encoding: BFMLALB_Z_ZZZi__");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    println!("encoding: BFMLALT_Z_ZZZi__");
                                }
                                (x0, x1, _) if x0 == 1 && x1 == 1 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
                            }
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 52 == 20 => {
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
                                    println!("unallocated");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: BFDOT_Z_ZZZ__");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, x1, _, _, _, _, x6, _, x8, _) if x1 == 0 && x6 == 0 && x8 == 1 => {
                        }
                        (_, x1, _, _, _, _, x6, _, _, _) if x1 == 0 && x6 == 1 => {
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
                                    println!("unallocated");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    println!("encoding: BFMLALB_Z_ZZZ__");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    println!("encoding: BFMLALT_Z_ZZZ__");
                                }
                                (x0, x1, _) if x0 == 1 && x1 == 1 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
                            }
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 52 == 36 => {
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 56 == 48 => {
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 == 56 => {
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 == 57 => {
                    let opc = (instr >> 22) & 3;
                    let Zm = (instr >> 16) & 9;
                    let Zn = (instr >> 5) & 9;
                    let Zda = instr & 9;
                    match opc {
                        x0 if x0 == 0 => {
                            println!("unallocated");
                        }
                        x0 if x0 == 1 => {
                            println!("encoding: BFMMLA_Z_ZZZ__");
                        }
                        x0 if x0 == 2 => {
                            println!("encoding: FMMLA_Z_ZZZ_S");
                        }
                        x0 if x0 == 3 => {
                            println!("encoding: FMMLA_Z_ZZZ_D");
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 62 == 58 => {
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 60 == 60 => {
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
                            println!("encoding: FCMGE_P_P_ZZ__");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            println!("encoding: FCMGT_P_P_ZZ__");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            println!("encoding: FCMEQ_P_P_ZZ__");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            println!("encoding: FCMNE_P_P_ZZ__");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            println!("encoding: FCMUO_P_P_ZZ__");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            println!("encoding: FACGE_P_P_ZZ__");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            println!("encoding: FACGT_P_P_ZZ__");
                        }
                        _ => unreachable!()
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
                            println!("encoding: FADD_Z_ZZ__");
                        }
                        x0 if x0 == 1 => {
                            println!("encoding: FSUB_Z_ZZ__");
                        }
                        x0 if x0 == 2 => {
                            println!("encoding: FMUL_Z_ZZ__");
                        }
                        x0 if x0 == 3 => {
                            println!("encoding: FTSMUL_Z_ZZ__");
                        }
                        x0 if x0 & 6 == 4 => {
                            println!("unallocated");
                        }
                        x0 if x0 == 6 => {
                            println!("encoding: FRECPS_Z_ZZ__");
                        }
                        x0 if x0 == 7 => {
                            println!("encoding: FRSQRTS_Z_ZZ__");
                        }
                        _ => unreachable!()
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
                                    println!("encoding: FADD_Z_P_ZZ__");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: FSUB_Z_P_ZZ__");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: FMUL_Z_P_ZZ__");
                                }
                                x0 if x0 == 3 => {
                                    println!("encoding: FSUBR_Z_P_ZZ__");
                                }
                                x0 if x0 == 4 => {
                                    println!("encoding: FMAXNM_Z_P_ZZ__");
                                }
                                x0 if x0 == 5 => {
                                    println!("encoding: FMINNM_Z_P_ZZ__");
                                }
                                x0 if x0 == 6 => {
                                    println!("encoding: FMAX_Z_P_ZZ__");
                                }
                                x0 if x0 == 7 => {
                                    println!("encoding: FMIN_Z_P_ZZ__");
                                }
                                x0 if x0 == 8 => {
                                    println!("encoding: FABD_Z_P_ZZ__");
                                }
                                x0 if x0 == 9 => {
                                    println!("encoding: FSCALE_Z_P_ZZ__");
                                }
                                x0 if x0 == 10 => {
                                    println!("encoding: FMULX_Z_P_ZZ__");
                                }
                                x0 if x0 == 11 => {
                                    println!("unallocated");
                                }
                                x0 if x0 == 12 => {
                                    println!("encoding: FDIVR_Z_P_ZZ__");
                                }
                                x0 if x0 == 13 => {
                                    println!("encoding: FDIV_Z_P_ZZ__");
                                }
                                x0 if x0 & 14 == 14 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, _, x3, _, _, x6, _, _) if x3 == 2 && x6 == 0 => {
                            let size = (instr >> 22) & 3;
                            let imm3 = (instr >> 16) & 5;
                            let Zm = (instr >> 5) & 9;
                            let Zdn = instr & 9;
                            match () {
                                () => {
                                    println!("encoding: FTMAD_Z_ZZI__");
                                }
                            }
                        }
                        (_, _, _, x3, _, _, x6, _, _) if x3 == 2 && x6 != 0 => {
                        }
                        (_, _, _, x3, _, _, _, x7, _) if x3 == 3 && x7 == 0 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 5;
                            let Pg = (instr >> 10) & 5;
                            let i1 = (instr >> 5) & 1;
                            let Zdn = instr & 9;
                            match opc {
                                x0 if x0 == 0 => {
                                    println!("encoding: FADD_Z_P_ZS__");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: FSUB_Z_P_ZS__");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: FMUL_Z_P_ZS__");
                                }
                                x0 if x0 == 3 => {
                                    println!("encoding: FSUBR_Z_P_ZS__");
                                }
                                x0 if x0 == 4 => {
                                    println!("encoding: FMAXNM_Z_P_ZS__");
                                }
                                x0 if x0 == 5 => {
                                    println!("encoding: FMINNM_Z_P_ZS__");
                                }
                                x0 if x0 == 6 => {
                                    println!("encoding: FMAX_Z_P_ZS__");
                                }
                                x0 if x0 == 7 => {
                                    println!("encoding: FMIN_Z_P_ZS__");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, _, x3, _, _, _, x7, _) if x3 == 3 && x7 != 0 => {
                        }
                        _ => unreachable!()
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
                                    println!("encoding: FRINTN_Z_P_Z__");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: FRINTP_Z_P_Z__");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: FRINTM_Z_P_Z__");
                                }
                                x0 if x0 == 3 => {
                                    println!("encoding: FRINTZ_Z_P_Z__");
                                }
                                x0 if x0 == 4 => {
                                    println!("encoding: FRINTA_Z_P_Z__");
                                }
                                x0 if x0 == 5 => {
                                    println!("unallocated");
                                }
                                x0 if x0 == 6 => {
                                    println!("encoding: FRINTX_Z_P_Z__");
                                }
                                x0 if x0 == 7 => {
                                    println!("encoding: FRINTI_Z_P_Z__");
                                }
                                _ => unreachable!()
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
                                    println!("unallocated");
                                }
                                (x0, x1) if x0 == 2 && x1 == 0 => {
                                    println!("encoding: FCVT_Z_P_Z_S2H");
                                }
                                (x0, x1) if x0 == 2 && x1 == 1 => {
                                    println!("encoding: FCVT_Z_P_Z_H2S");
                                }
                                (x0, x1) if x0 == 2 && x1 == 2 => {
                                    println!("encoding: BFCVT_Z_P_Z_S2BF");
                                }
                                (x0, x1) if x0 == 2 && x1 == 3 => {
                                    println!("unallocated");
                                }
                                (x0, x1) if x0 == 3 && x1 == 0 => {
                                    println!("encoding: FCVT_Z_P_Z_D2H");
                                }
                                (x0, x1) if x0 == 3 && x1 == 1 => {
                                    println!("encoding: FCVT_Z_P_Z_H2D");
                                }
                                (x0, x1) if x0 == 3 && x1 == 2 => {
                                    println!("encoding: FCVT_Z_P_Z_D2S");
                                }
                                (x0, x1) if x0 == 3 && x1 == 3 => {
                                    println!("encoding: FCVT_Z_P_Z_S2D");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: FRECPX_Z_P_Z__");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: FSQRT_Z_P_Z__");
                                }
                                x0 if x0 & 2 == 2 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
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
                                    println!("unallocated");
                                }
                                (x0, x1, _) if x0 == 1 && x1 == 0 => {
                                    println!("unallocated");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: SCVTF_Z_P_Z_H2FP16");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: UCVTF_Z_P_Z_H2FP16");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 == 0 => {
                                    println!("encoding: SCVTF_Z_P_Z_W2FP16");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 == 1 => {
                                    println!("encoding: UCVTF_Z_P_Z_W2FP16");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 0 => {
                                    println!("encoding: SCVTF_Z_P_Z_X2FP16");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 1 => {
                                    println!("encoding: UCVTF_Z_P_Z_X2FP16");
                                }
                                (x0, x1, _) if x0 == 2 && x1 & 2 == 0 => {
                                    println!("unallocated");
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 2 && x2 == 0 => {
                                    println!("encoding: SCVTF_Z_P_Z_W2S");
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 2 && x2 == 1 => {
                                    println!("encoding: UCVTF_Z_P_Z_W2S");
                                }
                                (x0, x1, _) if x0 == 2 && x1 == 3 => {
                                    println!("unallocated");
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 0 => {
                                    println!("encoding: SCVTF_Z_P_Z_W2D");
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 1 => {
                                    println!("encoding: UCVTF_Z_P_Z_W2D");
                                }
                                (x0, x1, _) if x0 == 3 && x1 == 1 => {
                                    println!("unallocated");
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 2 && x2 == 0 => {
                                    println!("encoding: SCVTF_Z_P_Z_X2S");
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 2 && x2 == 1 => {
                                    println!("encoding: UCVTF_Z_P_Z_X2S");
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 3 && x2 == 0 => {
                                    println!("encoding: SCVTF_Z_P_Z_X2D");
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 3 && x2 == 1 => {
                                    println!("encoding: UCVTF_Z_P_Z_X2D");
                                }
                                _ => unreachable!()
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
                                    println!("unallocated");
                                }
                                (x0, x1, _) if x0 == 1 && x1 == 0 => {
                                    println!("unallocated");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: FCVTZS_Z_P_Z_FP162H");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: FCVTZU_Z_P_Z_FP162H");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 == 0 => {
                                    println!("encoding: FCVTZS_Z_P_Z_FP162W");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 == 1 => {
                                    println!("encoding: FCVTZU_Z_P_Z_FP162W");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 0 => {
                                    println!("encoding: FCVTZS_Z_P_Z_FP162X");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 1 => {
                                    println!("encoding: FCVTZU_Z_P_Z_FP162X");
                                }
                                (x0, x1, _) if x0 == 2 && x1 & 2 == 0 => {
                                    println!("unallocated");
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 2 && x2 == 0 => {
                                    println!("encoding: FCVTZS_Z_P_Z_S2W");
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 2 && x2 == 1 => {
                                    println!("encoding: FCVTZU_Z_P_Z_S2W");
                                }
                                (x0, x1, _) if x0 == 2 && x1 == 3 => {
                                    println!("unallocated");
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 0 => {
                                    println!("encoding: FCVTZS_Z_P_Z_D2W");
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 1 => {
                                    println!("encoding: FCVTZU_Z_P_Z_D2W");
                                }
                                (x0, x1, _) if x0 == 3 && x1 == 1 => {
                                    println!("unallocated");
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 2 && x2 == 0 => {
                                    println!("encoding: FCVTZS_Z_P_Z_S2X");
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 2 && x2 == 1 => {
                                    println!("encoding: FCVTZU_Z_P_Z_S2X");
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 3 && x2 == 0 => {
                                    println!("encoding: FCVTZS_Z_P_Z_D2X");
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 3 && x2 == 1 => {
                                    println!("encoding: FCVTZU_Z_P_Z_D2X");
                                }
                                _ => unreachable!()
                            }
                        }
                        _ => unreachable!()
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
                            println!("encoding: FADDV_V_P_Z__");
                        }
                        x0 if x0 == 1 => {
                            println!("unallocated");
                        }
                        x0 if x0 & 6 == 2 => {
                            println!("unallocated");
                        }
                        x0 if x0 == 4 => {
                            println!("encoding: FMAXNMV_V_P_Z__");
                        }
                        x0 if x0 == 5 => {
                            println!("encoding: FMINNMV_V_P_Z__");
                        }
                        x0 if x0 == 6 => {
                            println!("encoding: FMAXV_V_P_Z__");
                        }
                        x0 if x0 == 7 => {
                            println!("encoding: FMINV_V_P_Z__");
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 == 3 && x2 & 2 == 2 && x4 & 28 == 4 && x6 & 60 == 8 => {
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
                                    println!("unallocated");
                                }
                                x0 if x0 & 6 == 4 => {
                                    println!("unallocated");
                                }
                                x0 if x0 == 6 => {
                                    println!("encoding: FRECPE_Z_Z__");
                                }
                                x0 if x0 == 7 => {
                                    println!("encoding: FRSQRTE_Z_Z__");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, _, _, _, x5, _) if x5 != 0 => {
                        }
                        _ => unreachable!()
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
                                    println!("encoding: FCMGE_P_P_Z0__");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                                    println!("encoding: FCMGT_P_P_Z0__");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: FCMLT_P_P_Z0__");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: FCMLE_P_P_Z0__");
                                }
                                (x0, _, x2) if x0 == 1 && x2 == 1 => {
                                    println!("unallocated");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    println!("encoding: FCMEQ_P_P_Z0__");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: FCMNE_P_P_Z0__");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, _, x3, _, _, _) if x3 == 1 => {
                        }
                        _ => unreachable!()
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
                            println!("encoding: FADDA_V_P_Z__");
                        }
                        x0 if x0 == 1 => {
                            println!("unallocated");
                        }
                        x0 if x0 & 6 == 2 => {
                            println!("unallocated");
                        }
                        x0 if x0 & 4 == 4 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
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
                                    println!("encoding: FMLA_Z_P_ZZZ__");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: FMLS_Z_P_ZZZ__");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: FNMLA_Z_P_ZZZ__");
                                }
                                x0 if x0 == 3 => {
                                    println!("encoding: FNMLS_Z_P_ZZZ__");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: FMAD_Z_P_ZZZ__");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: FMSB_Z_P_ZZZ__");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: FNMAD_Z_P_ZZZ__");
                                }
                                x0 if x0 == 3 => {
                                    println!("encoding: FNMSB_Z_P_ZZZ__");
                                }
                                _ => unreachable!()
                            }
                        }
                        _ => unreachable!()
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
                                    println!("encoding: PRFB_I_P_BZ_S_x32_scaled");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: PRFH_I_P_BZ_S_x32_scaled");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: PRFW_I_P_BZ_S_x32_scaled");
                                }
                                x0 if x0 == 3 => {
                                    println!("encoding: PRFD_I_P_BZ_S_x32_scaled");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, x1, x2, _, x4, _, x6, _) if x1 == 0 && x2 & 1 == 1 && x4 & 4 == 0 && x6 == 1 => {
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
                                    println!("encoding: LD1SH_Z_P_BZ_S_x32_scaled");
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    println!("encoding: LDFF1SH_Z_P_BZ_S_x32_scaled");
                                }
                                (x0, x1) if x0 == 1 && x1 == 0 => {
                                    println!("encoding: LD1H_Z_P_BZ_S_x32_scaled");
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    println!("encoding: LDFF1H_Z_P_BZ_S_x32_scaled");
                                }
                                _ => unreachable!()
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
                                    println!("unallocated");
                                }
                                (x0, x1) if x0 == 1 && x1 == 0 => {
                                    println!("encoding: LD1W_Z_P_BZ_S_x32_scaled");
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    println!("encoding: LDFF1W_Z_P_BZ_S_x32_scaled");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, x1, x2, _, x4, _, x6, _) if x1 == 3 && x2 & 2 == 0 && x4 == 0 && x6 == 0 => {
                            let imm9h = (instr >> 16) & 11;
                            let imm9l = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Pt = instr & 7;
                            match () {
                                () => {
                                    println!("encoding: LDR_P_BI__");
                                }
                            }
                        }
                        (_, x1, x2, _, x4, _, x6, _) if x1 == 3 && x2 & 2 == 0 && x4 == 0 && x6 == 1 => {
                        }
                        (_, x1, x2, _, x4, _, _, _) if x1 == 3 && x2 & 2 == 0 && x4 == 2 => {
                            let imm9h = (instr >> 16) & 11;
                            let imm9l = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match () {
                                () => {
                                    println!("encoding: LDR_Z_BI__");
                                }
                            }
                        }
                        (_, x1, x2, _, x4, _, _, _) if x1 == 3 && x2 & 2 == 0 && x4 & 5 == 1 => {
                        }
                        (_, x1, x2, _, x4, _, x6, _) if x1 == 3 && x2 & 2 == 2 && x4 & 4 == 0 && x6 == 0 => {
                            let imm6 = (instr >> 16) & 11;
                            let msz = (instr >> 13) & 3;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let prfop = instr & 7;
                            match msz {
                                x0 if x0 == 0 => {
                                    println!("encoding: PRFB_I_P_BI_S");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: PRFH_I_P_BI_S");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: PRFW_I_P_BI_S");
                                }
                                x0 if x0 == 3 => {
                                    println!("encoding: PRFD_I_P_BI_S");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, x1, x2, _, x4, _, x6, _) if x1 == 3 && x2 & 2 == 2 && x4 & 4 == 0 && x6 == 1 => {
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
                                    println!("encoding: LD1SB_Z_P_BZ_S_x32_unscaled");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                                    println!("encoding: LDFF1SB_Z_P_BZ_S_x32_unscaled");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: LD1B_Z_P_BZ_S_x32_unscaled");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: LDFF1B_Z_P_BZ_S_x32_unscaled");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    println!("encoding: LD1SH_Z_P_BZ_S_x32_unscaled");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    println!("encoding: LDFF1SH_Z_P_BZ_S_x32_unscaled");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: LD1H_Z_P_BZ_S_x32_unscaled");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: LDFF1H_Z_P_BZ_S_x32_unscaled");
                                }
                                (x0, x1, _) if x0 == 2 && x1 == 0 => {
                                    println!("unallocated");
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: LD1W_Z_P_BZ_S_x32_unscaled");
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: LDFF1W_Z_P_BZ_S_x32_unscaled");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, x2, _, x4, _, _, _) if x2 == 0 && x4 & 6 == 4 => {
                        }
                        (_, _, x2, _, x4, _, x6, _) if x2 == 0 && x4 == 6 && x6 == 0 => {
                            let msz = (instr >> 23) & 3;
                            let Rm = (instr >> 16) & 9;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let prfop = instr & 7;
                            match msz {
                                x0 if x0 == 0 => {
                                    println!("encoding: PRFB_I_P_BR_S");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: PRFH_I_P_BR_S");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: PRFW_I_P_BR_S");
                                }
                                x0 if x0 == 3 => {
                                    println!("encoding: PRFD_I_P_BR_S");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: PRFB_I_P_AI_S");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: PRFH_I_P_AI_S");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: PRFW_I_P_AI_S");
                                }
                                x0 if x0 == 3 => {
                                    println!("encoding: PRFD_I_P_AI_S");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, x2, _, x4, _, x6, _) if x2 == 0 && x4 & 6 == 6 && x6 == 1 => {
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
                                    println!("encoding: LD1SB_Z_P_AI_S");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                                    println!("encoding: LDFF1SB_Z_P_AI_S");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: LD1B_Z_P_AI_S");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: LDFF1B_Z_P_AI_S");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    println!("encoding: LD1SH_Z_P_AI_S");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    println!("encoding: LDFF1SH_Z_P_AI_S");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: LD1H_Z_P_AI_S");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: LDFF1H_Z_P_AI_S");
                                }
                                (x0, x1, _) if x0 == 2 && x1 == 0 => {
                                    println!("unallocated");
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: LD1W_Z_P_AI_S");
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: LDFF1W_Z_P_AI_S");
                                }
                                (x0, _, _) if x0 == 3 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: LD1RB_Z_P_BI_U8");
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    println!("encoding: LD1RB_Z_P_BI_U16");
                                }
                                (x0, x1) if x0 == 0 && x1 == 2 => {
                                    println!("encoding: LD1RB_Z_P_BI_U32");
                                }
                                (x0, x1) if x0 == 0 && x1 == 3 => {
                                    println!("encoding: LD1RB_Z_P_BI_U64");
                                }
                                (x0, x1) if x0 == 1 && x1 == 0 => {
                                    println!("encoding: LD1RSW_Z_P_BI_S64");
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    println!("encoding: LD1RH_Z_P_BI_U16");
                                }
                                (x0, x1) if x0 == 1 && x1 == 2 => {
                                    println!("encoding: LD1RH_Z_P_BI_U32");
                                }
                                (x0, x1) if x0 == 1 && x1 == 3 => {
                                    println!("encoding: LD1RH_Z_P_BI_U64");
                                }
                                (x0, x1) if x0 == 2 && x1 == 0 => {
                                    println!("encoding: LD1RSH_Z_P_BI_S64");
                                }
                                (x0, x1) if x0 == 2 && x1 == 1 => {
                                    println!("encoding: LD1RSH_Z_P_BI_S32");
                                }
                                (x0, x1) if x0 == 2 && x1 == 2 => {
                                    println!("encoding: LD1RW_Z_P_BI_U32");
                                }
                                (x0, x1) if x0 == 2 && x1 == 3 => {
                                    println!("encoding: LD1RW_Z_P_BI_U64");
                                }
                                (x0, x1) if x0 == 3 && x1 == 0 => {
                                    println!("encoding: LD1RSB_Z_P_BI_S64");
                                }
                                (x0, x1) if x0 == 3 && x1 == 1 => {
                                    println!("encoding: LD1RSB_Z_P_BI_S32");
                                }
                                (x0, x1) if x0 == 3 && x1 == 2 => {
                                    println!("encoding: LD1RSB_Z_P_BI_S16");
                                }
                                (x0, x1) if x0 == 3 && x1 == 3 => {
                                    println!("encoding: LD1RD_Z_P_BI_U64");
                                }
                                _ => unreachable!()
                            }
                        }
                        _ => unreachable!()
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
                                    println!("encoding: LDNT1B_Z_P_BI_Contiguous");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: LDNT1H_Z_P_BI_Contiguous");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: LDNT1W_Z_P_BI_Contiguous");
                                }
                                x0 if x0 == 3 => {
                                    println!("encoding: LDNT1D_Z_P_BI_Contiguous");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: LDNT1B_Z_P_BR_Contiguous");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: LDNT1H_Z_P_BR_Contiguous");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: LDNT1W_Z_P_BR_Contiguous");
                                }
                                x0 if x0 == 3 => {
                                    println!("encoding: LDNT1D_Z_P_BR_Contiguous");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: LD2B_Z_P_BI_Contiguous");
                                }
                                (x0, x1) if x0 == 0 && x1 == 2 => {
                                    println!("encoding: LD3B_Z_P_BI_Contiguous");
                                }
                                (x0, x1) if x0 == 0 && x1 == 3 => {
                                    println!("encoding: LD4B_Z_P_BI_Contiguous");
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    println!("encoding: LD2H_Z_P_BI_Contiguous");
                                }
                                (x0, x1) if x0 == 1 && x1 == 2 => {
                                    println!("encoding: LD3H_Z_P_BI_Contiguous");
                                }
                                (x0, x1) if x0 == 1 && x1 == 3 => {
                                    println!("encoding: LD4H_Z_P_BI_Contiguous");
                                }
                                (x0, x1) if x0 == 2 && x1 == 1 => {
                                    println!("encoding: LD2W_Z_P_BI_Contiguous");
                                }
                                (x0, x1) if x0 == 2 && x1 == 2 => {
                                    println!("encoding: LD3W_Z_P_BI_Contiguous");
                                }
                                (x0, x1) if x0 == 2 && x1 == 3 => {
                                    println!("encoding: LD4W_Z_P_BI_Contiguous");
                                }
                                (x0, x1) if x0 == 3 && x1 == 1 => {
                                    println!("encoding: LD2D_Z_P_BI_Contiguous");
                                }
                                (x0, x1) if x0 == 3 && x1 == 2 => {
                                    println!("encoding: LD3D_Z_P_BI_Contiguous");
                                }
                                (x0, x1) if x0 == 3 && x1 == 3 => {
                                    println!("encoding: LD4D_Z_P_BI_Contiguous");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: LD2B_Z_P_BR_Contiguous");
                                }
                                (x0, x1) if x0 == 0 && x1 == 2 => {
                                    println!("encoding: LD3B_Z_P_BR_Contiguous");
                                }
                                (x0, x1) if x0 == 0 && x1 == 3 => {
                                    println!("encoding: LD4B_Z_P_BR_Contiguous");
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    println!("encoding: LD2H_Z_P_BR_Contiguous");
                                }
                                (x0, x1) if x0 == 1 && x1 == 2 => {
                                    println!("encoding: LD3H_Z_P_BR_Contiguous");
                                }
                                (x0, x1) if x0 == 1 && x1 == 3 => {
                                    println!("encoding: LD4H_Z_P_BR_Contiguous");
                                }
                                (x0, x1) if x0 == 2 && x1 == 1 => {
                                    println!("encoding: LD2W_Z_P_BR_Contiguous");
                                }
                                (x0, x1) if x0 == 2 && x1 == 2 => {
                                    println!("encoding: LD3W_Z_P_BR_Contiguous");
                                }
                                (x0, x1) if x0 == 2 && x1 == 3 => {
                                    println!("encoding: LD4W_Z_P_BR_Contiguous");
                                }
                                (x0, x1) if x0 == 3 && x1 == 1 => {
                                    println!("encoding: LD2D_Z_P_BR_Contiguous");
                                }
                                (x0, x1) if x0 == 3 && x1 == 2 => {
                                    println!("encoding: LD3D_Z_P_BR_Contiguous");
                                }
                                (x0, x1) if x0 == 3 && x1 == 3 => {
                                    println!("encoding: LD4D_Z_P_BR_Contiguous");
                                }
                                _ => unreachable!()
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
                                    println!("unallocated");
                                }
                                (x0, x1) if x0 == 0 && x1 == 0 => {
                                    println!("encoding: LD1RQB_Z_P_BI_U8");
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    println!("encoding: LD1ROB_Z_P_BI_U8");
                                }
                                (x0, x1) if x0 == 1 && x1 == 0 => {
                                    println!("encoding: LD1RQH_Z_P_BI_U16");
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    println!("encoding: LD1ROH_Z_P_BI_U16");
                                }
                                (x0, x1) if x0 == 2 && x1 == 0 => {
                                    println!("encoding: LD1RQW_Z_P_BI_U32");
                                }
                                (x0, x1) if x0 == 2 && x1 == 1 => {
                                    println!("encoding: LD1ROW_Z_P_BI_U32");
                                }
                                (x0, x1) if x0 == 3 && x1 == 0 => {
                                    println!("encoding: LD1RQD_Z_P_BI_U64");
                                }
                                (x0, x1) if x0 == 3 && x1 == 1 => {
                                    println!("encoding: LD1ROD_Z_P_BI_U64");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: LD1B_Z_P_BI_U8");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: LD1B_Z_P_BI_U16");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: LD1B_Z_P_BI_U32");
                                }
                                x0 if x0 == 3 => {
                                    println!("encoding: LD1B_Z_P_BI_U64");
                                }
                                x0 if x0 == 4 => {
                                    println!("encoding: LD1SW_Z_P_BI_S64");
                                }
                                x0 if x0 == 5 => {
                                    println!("encoding: LD1H_Z_P_BI_U16");
                                }
                                x0 if x0 == 6 => {
                                    println!("encoding: LD1H_Z_P_BI_U32");
                                }
                                x0 if x0 == 7 => {
                                    println!("encoding: LD1H_Z_P_BI_U64");
                                }
                                x0 if x0 == 8 => {
                                    println!("encoding: LD1SH_Z_P_BI_S64");
                                }
                                x0 if x0 == 9 => {
                                    println!("encoding: LD1SH_Z_P_BI_S32");
                                }
                                x0 if x0 == 10 => {
                                    println!("encoding: LD1W_Z_P_BI_U32");
                                }
                                x0 if x0 == 11 => {
                                    println!("encoding: LD1W_Z_P_BI_U64");
                                }
                                x0 if x0 == 12 => {
                                    println!("encoding: LD1SB_Z_P_BI_S64");
                                }
                                x0 if x0 == 13 => {
                                    println!("encoding: LD1SB_Z_P_BI_S32");
                                }
                                x0 if x0 == 14 => {
                                    println!("encoding: LD1SB_Z_P_BI_S16");
                                }
                                x0 if x0 == 15 => {
                                    println!("encoding: LD1D_Z_P_BI_U64");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, _, x3, _, x5, _) if x3 == 1 && x5 == 1 => {
                        }
                        (_, _, _, x3, _, x5, _) if x3 == 1 && x5 == 5 => {
                            let dtype = (instr >> 21) & 7;
                            let imm4 = (instr >> 16) & 7;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match dtype {
                                x0 if x0 == 0 => {
                                    println!("encoding: LDNF1B_Z_P_BI_U8");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: LDNF1B_Z_P_BI_U16");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: LDNF1B_Z_P_BI_U32");
                                }
                                x0 if x0 == 3 => {
                                    println!("encoding: LDNF1B_Z_P_BI_U64");
                                }
                                x0 if x0 == 4 => {
                                    println!("encoding: LDNF1SW_Z_P_BI_S64");
                                }
                                x0 if x0 == 5 => {
                                    println!("encoding: LDNF1H_Z_P_BI_U16");
                                }
                                x0 if x0 == 6 => {
                                    println!("encoding: LDNF1H_Z_P_BI_U32");
                                }
                                x0 if x0 == 7 => {
                                    println!("encoding: LDNF1H_Z_P_BI_U64");
                                }
                                x0 if x0 == 8 => {
                                    println!("encoding: LDNF1SH_Z_P_BI_S64");
                                }
                                x0 if x0 == 9 => {
                                    println!("encoding: LDNF1SH_Z_P_BI_S32");
                                }
                                x0 if x0 == 10 => {
                                    println!("encoding: LDNF1W_Z_P_BI_U32");
                                }
                                x0 if x0 == 11 => {
                                    println!("encoding: LDNF1W_Z_P_BI_U64");
                                }
                                x0 if x0 == 12 => {
                                    println!("encoding: LDNF1SB_Z_P_BI_S64");
                                }
                                x0 if x0 == 13 => {
                                    println!("encoding: LDNF1SB_Z_P_BI_S32");
                                }
                                x0 if x0 == 14 => {
                                    println!("encoding: LDNF1SB_Z_P_BI_S16");
                                }
                                x0 if x0 == 15 => {
                                    println!("encoding: LDNF1D_Z_P_BI_U64");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, _, x3, _, x5, _) if x3 == 1 && x5 == 7 => {
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
                                    println!("unallocated");
                                }
                                (x0, x1) if x0 == 0 && x1 == 0 => {
                                    println!("encoding: LD1RQB_Z_P_BR_Contiguous");
                                }
                                (x0, x1) if x0 == 0 && x1 == 1 => {
                                    println!("encoding: LD1ROB_Z_P_BR_Contiguous");
                                }
                                (x0, x1) if x0 == 1 && x1 == 0 => {
                                    println!("encoding: LD1RQH_Z_P_BR_Contiguous");
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    println!("encoding: LD1ROH_Z_P_BR_Contiguous");
                                }
                                (x0, x1) if x0 == 2 && x1 == 0 => {
                                    println!("encoding: LD1RQW_Z_P_BR_Contiguous");
                                }
                                (x0, x1) if x0 == 2 && x1 == 1 => {
                                    println!("encoding: LD1ROW_Z_P_BR_Contiguous");
                                }
                                (x0, x1) if x0 == 3 && x1 == 0 => {
                                    println!("encoding: LD1RQD_Z_P_BR_Contiguous");
                                }
                                (x0, x1) if x0 == 3 && x1 == 1 => {
                                    println!("encoding: LD1ROD_Z_P_BR_Contiguous");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: LD1B_Z_P_BR_U8");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: LD1B_Z_P_BR_U16");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: LD1B_Z_P_BR_U32");
                                }
                                x0 if x0 == 3 => {
                                    println!("encoding: LD1B_Z_P_BR_U64");
                                }
                                x0 if x0 == 4 => {
                                    println!("encoding: LD1SW_Z_P_BR_S64");
                                }
                                x0 if x0 == 5 => {
                                    println!("encoding: LD1H_Z_P_BR_U16");
                                }
                                x0 if x0 == 6 => {
                                    println!("encoding: LD1H_Z_P_BR_U32");
                                }
                                x0 if x0 == 7 => {
                                    println!("encoding: LD1H_Z_P_BR_U64");
                                }
                                x0 if x0 == 8 => {
                                    println!("encoding: LD1SH_Z_P_BR_S64");
                                }
                                x0 if x0 == 9 => {
                                    println!("encoding: LD1SH_Z_P_BR_S32");
                                }
                                x0 if x0 == 10 => {
                                    println!("encoding: LD1W_Z_P_BR_U32");
                                }
                                x0 if x0 == 11 => {
                                    println!("encoding: LD1W_Z_P_BR_U64");
                                }
                                x0 if x0 == 12 => {
                                    println!("encoding: LD1SB_Z_P_BR_S64");
                                }
                                x0 if x0 == 13 => {
                                    println!("encoding: LD1SB_Z_P_BR_S32");
                                }
                                x0 if x0 == 14 => {
                                    println!("encoding: LD1SB_Z_P_BR_S16");
                                }
                                x0 if x0 == 15 => {
                                    println!("encoding: LD1D_Z_P_BR_U64");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: LDFF1B_Z_P_BR_U8");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: LDFF1B_Z_P_BR_U16");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: LDFF1B_Z_P_BR_U32");
                                }
                                x0 if x0 == 3 => {
                                    println!("encoding: LDFF1B_Z_P_BR_U64");
                                }
                                x0 if x0 == 4 => {
                                    println!("encoding: LDFF1SW_Z_P_BR_S64");
                                }
                                x0 if x0 == 5 => {
                                    println!("encoding: LDFF1H_Z_P_BR_U16");
                                }
                                x0 if x0 == 6 => {
                                    println!("encoding: LDFF1H_Z_P_BR_U32");
                                }
                                x0 if x0 == 7 => {
                                    println!("encoding: LDFF1H_Z_P_BR_U64");
                                }
                                x0 if x0 == 8 => {
                                    println!("encoding: LDFF1SH_Z_P_BR_S64");
                                }
                                x0 if x0 == 9 => {
                                    println!("encoding: LDFF1SH_Z_P_BR_S32");
                                }
                                x0 if x0 == 10 => {
                                    println!("encoding: LDFF1W_Z_P_BR_U32");
                                }
                                x0 if x0 == 11 => {
                                    println!("encoding: LDFF1W_Z_P_BR_U64");
                                }
                                x0 if x0 == 12 => {
                                    println!("encoding: LDFF1SB_Z_P_BR_S64");
                                }
                                x0 if x0 == 13 => {
                                    println!("encoding: LDFF1SB_Z_P_BR_S32");
                                }
                                x0 if x0 == 14 => {
                                    println!("encoding: LDFF1SB_Z_P_BR_S16");
                                }
                                x0 if x0 == 15 => {
                                    println!("encoding: LDFF1D_Z_P_BR_U64");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, _, _, _, x5, _) if x5 == 4 => {
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, _, _, _, _, _, _) if x0 == 6 => {
                    match ((instr >> 25) & 13, (instr >> 23) & 3, (instr >> 21) & 3, (instr >> 16) & 9, (instr >> 13) & 5, (instr >> 5) & 15, (instr >> 4) & 1, instr & 7) {
                        (_, x1, x2, _, x4, _, x6, _) if x1 == 0 && x2 == 1 && x4 & 4 == 0 && x6 == 1 => {
                        }
                        (_, x1, x2, _, x4, _, x6, _) if x1 == 0 && x2 == 3 && x4 & 4 == 4 && x6 == 0 => {
                            let Zm = (instr >> 16) & 9;
                            let msz = (instr >> 13) & 3;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let prfop = instr & 7;
                            match msz {
                                x0 if x0 == 0 => {
                                    println!("encoding: PRFB_I_P_BZ_D_64_scaled");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: PRFH_I_P_BZ_D_64_scaled");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: PRFW_I_P_BZ_D_64_scaled");
                                }
                                x0 if x0 == 3 => {
                                    println!("encoding: PRFD_I_P_BZ_D_64_scaled");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, x1, x2, _, _, _, x6, _) if x1 == 0 && x2 == 3 && x6 == 1 => {
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
                                    println!("encoding: PRFB_I_P_BZ_D_x32_scaled");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: PRFH_I_P_BZ_D_x32_scaled");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: PRFW_I_P_BZ_D_x32_scaled");
                                }
                                x0 if x0 == 3 => {
                                    println!("encoding: PRFD_I_P_BZ_D_x32_scaled");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: LD1SH_Z_P_BZ_D_64_scaled");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    println!("encoding: LDFF1SH_Z_P_BZ_D_64_scaled");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: LD1H_Z_P_BZ_D_64_scaled");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: LDFF1H_Z_P_BZ_D_64_scaled");
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                                    println!("encoding: LD1SW_Z_P_BZ_D_64_scaled");
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                                    println!("encoding: LDFF1SW_Z_P_BZ_D_64_scaled");
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: LD1W_Z_P_BZ_D_64_scaled");
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: LDFF1W_Z_P_BZ_D_64_scaled");
                                }
                                (x0, x1, _) if x0 == 3 && x1 == 0 => {
                                    println!("unallocated");
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: LD1D_Z_P_BZ_D_64_scaled");
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: LDFF1D_Z_P_BZ_D_64_scaled");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: LD1SH_Z_P_BZ_D_x32_scaled");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    println!("encoding: LDFF1SH_Z_P_BZ_D_x32_scaled");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: LD1H_Z_P_BZ_D_x32_scaled");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: LDFF1H_Z_P_BZ_D_x32_scaled");
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                                    println!("encoding: LD1SW_Z_P_BZ_D_x32_scaled");
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                                    println!("encoding: LDFF1SW_Z_P_BZ_D_x32_scaled");
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: LD1W_Z_P_BZ_D_x32_scaled");
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: LDFF1W_Z_P_BZ_D_x32_scaled");
                                }
                                (x0, x1, _) if x0 == 3 && x1 == 0 => {
                                    println!("unallocated");
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: LD1D_Z_P_BZ_D_x32_scaled");
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: LDFF1D_Z_P_BZ_D_x32_scaled");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, x2, _, x4, _, _, _) if x2 == 0 && x4 & 6 == 4 => {
                        }
                        (_, _, x2, _, x4, _, _, _) if x2 == 0 && x4 == 6 => {
                        }
                        (_, _, x2, _, x4, _, x6, _) if x2 == 0 && x4 == 7 && x6 == 0 => {
                            let msz = (instr >> 23) & 3;
                            let imm5 = (instr >> 16) & 9;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let prfop = instr & 7;
                            match msz {
                                x0 if x0 == 0 => {
                                    println!("encoding: PRFB_I_P_AI_D");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: PRFH_I_P_AI_D");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: PRFW_I_P_AI_D");
                                }
                                x0 if x0 == 3 => {
                                    println!("encoding: PRFD_I_P_AI_D");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, x2, _, x4, _, x6, _) if x2 == 0 && x4 == 7 && x6 == 1 => {
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
                                    println!("encoding: LD1SB_Z_P_AI_D");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                                    println!("encoding: LDFF1SB_Z_P_AI_D");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: LD1B_Z_P_AI_D");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: LDFF1B_Z_P_AI_D");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    println!("encoding: LD1SH_Z_P_AI_D");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    println!("encoding: LDFF1SH_Z_P_AI_D");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: LD1H_Z_P_AI_D");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: LDFF1H_Z_P_AI_D");
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                                    println!("encoding: LD1SW_Z_P_AI_D");
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                                    println!("encoding: LDFF1SW_Z_P_AI_D");
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: LD1W_Z_P_AI_D");
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: LDFF1W_Z_P_AI_D");
                                }
                                (x0, x1, _) if x0 == 3 && x1 == 0 => {
                                    println!("unallocated");
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: LD1D_Z_P_AI_D");
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: LDFF1D_Z_P_AI_D");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: LD1SB_Z_P_BZ_D_64_unscaled");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                                    println!("encoding: LDFF1SB_Z_P_BZ_D_64_unscaled");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: LD1B_Z_P_BZ_D_64_unscaled");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: LDFF1B_Z_P_BZ_D_64_unscaled");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    println!("encoding: LD1SH_Z_P_BZ_D_64_unscaled");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    println!("encoding: LDFF1SH_Z_P_BZ_D_64_unscaled");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: LD1H_Z_P_BZ_D_64_unscaled");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: LDFF1H_Z_P_BZ_D_64_unscaled");
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                                    println!("encoding: LD1SW_Z_P_BZ_D_64_unscaled");
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                                    println!("encoding: LDFF1SW_Z_P_BZ_D_64_unscaled");
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: LD1W_Z_P_BZ_D_64_unscaled");
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: LDFF1W_Z_P_BZ_D_64_unscaled");
                                }
                                (x0, x1, _) if x0 == 3 && x1 == 0 => {
                                    println!("unallocated");
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: LD1D_Z_P_BZ_D_64_unscaled");
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: LDFF1D_Z_P_BZ_D_64_unscaled");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: LD1SB_Z_P_BZ_D_x32_unscaled");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                                    println!("encoding: LDFF1SB_Z_P_BZ_D_x32_unscaled");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: LD1B_Z_P_BZ_D_x32_unscaled");
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: LDFF1B_Z_P_BZ_D_x32_unscaled");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    println!("encoding: LD1SH_Z_P_BZ_D_x32_unscaled");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    println!("encoding: LDFF1SH_Z_P_BZ_D_x32_unscaled");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: LD1H_Z_P_BZ_D_x32_unscaled");
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: LDFF1H_Z_P_BZ_D_x32_unscaled");
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                                    println!("encoding: LD1SW_Z_P_BZ_D_x32_unscaled");
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                                    println!("encoding: LDFF1SW_Z_P_BZ_D_x32_unscaled");
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: LD1W_Z_P_BZ_D_x32_unscaled");
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: LDFF1W_Z_P_BZ_D_x32_unscaled");
                                }
                                (x0, x1, _) if x0 == 3 && x1 == 0 => {
                                    println!("unallocated");
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 0 => {
                                    println!("encoding: LD1D_Z_P_BZ_D_x32_unscaled");
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 1 => {
                                    println!("encoding: LDFF1D_Z_P_BZ_D_x32_unscaled");
                                }
                                _ => unreachable!()
                            }
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, _, _, _, _, x6, _) if x0 == 7 && x6 & 40 == 0 => {
                    match ((instr >> 25) & 13, (instr >> 22) & 5, (instr >> 16) & 11, (instr >> 15) & 1, (instr >> 14) & 1, (instr >> 13) & 1, (instr >> 5) & 15, (instr >> 4) & 1, instr & 7) {
                        (_, x1, _, _, x4, _, _, _, _) if x1 & 4 == 0 && x4 == 0 => {
                        }
                        (_, x1, _, _, x4, _, _, _, _) if x1 & 6 == 4 && x4 == 0 => {
                        }
                        (_, x1, _, _, x4, _, _, x7, _) if x1 == 6 && x4 == 0 && x7 == 0 => {
                            let imm9h = (instr >> 16) & 11;
                            let imm9l = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Pt = instr & 7;
                            match () {
                                () => {
                                    println!("encoding: STR_P_BI__");
                                }
                            }
                        }
                        (_, x1, _, _, x4, _, _, x7, _) if x1 == 6 && x4 == 0 && x7 == 1 => {
                        }
                        (_, x1, _, _, x4, _, _, _, _) if x1 == 6 && x4 == 1 => {
                            let imm9h = (instr >> 16) & 11;
                            let imm9l = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match () {
                                () => {
                                    println!("encoding: STR_Z_BI__");
                                }
                            }
                        }
                        (_, x1, _, _, x4, _, _, _, _) if x1 == 7 && x4 == 0 => {
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
                                    println!("encoding: ST1B_Z_P_BR__");
                                }
                                (x0, _) if x0 & 6 == 2 => {
                                    println!("encoding: ST1H_Z_P_BR__");
                                }
                                (x0, _) if x0 & 6 == 4 => {
                                    println!("encoding: ST1W_Z_P_BR__");
                                }
                                (x0, x1) if x0 == 7 && x1 == 0 => {
                                    println!("unallocated");
                                }
                                (x0, x1) if x0 == 7 && x1 == 1 => {
                                    println!("encoding: ST1D_Z_P_BR__");
                                }
                                _ => unreachable!()
                            }
                        }
                        _ => unreachable!()
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
                                    println!("encoding: STNT1B_Z_P_BR_Contiguous");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: STNT1H_Z_P_BR_Contiguous");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: STNT1W_Z_P_BR_Contiguous");
                                }
                                x0 if x0 == 3 => {
                                    println!("encoding: STNT1D_Z_P_BR_Contiguous");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: ST2B_Z_P_BR_Contiguous");
                                }
                                (x0, x1) if x0 == 0 && x1 == 2 => {
                                    println!("encoding: ST3B_Z_P_BR_Contiguous");
                                }
                                (x0, x1) if x0 == 0 && x1 == 3 => {
                                    println!("encoding: ST4B_Z_P_BR_Contiguous");
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    println!("encoding: ST2H_Z_P_BR_Contiguous");
                                }
                                (x0, x1) if x0 == 1 && x1 == 2 => {
                                    println!("encoding: ST3H_Z_P_BR_Contiguous");
                                }
                                (x0, x1) if x0 == 1 && x1 == 3 => {
                                    println!("encoding: ST4H_Z_P_BR_Contiguous");
                                }
                                (x0, x1) if x0 == 2 && x1 == 1 => {
                                    println!("encoding: ST2W_Z_P_BR_Contiguous");
                                }
                                (x0, x1) if x0 == 2 && x1 == 2 => {
                                    println!("encoding: ST3W_Z_P_BR_Contiguous");
                                }
                                (x0, x1) if x0 == 2 && x1 == 3 => {
                                    println!("encoding: ST4W_Z_P_BR_Contiguous");
                                }
                                (x0, x1) if x0 == 3 && x1 == 1 => {
                                    println!("encoding: ST2D_Z_P_BR_Contiguous");
                                }
                                (x0, x1) if x0 == 3 && x1 == 2 => {
                                    println!("encoding: ST3D_Z_P_BR_Contiguous");
                                }
                                (x0, x1) if x0 == 3 && x1 == 3 => {
                                    println!("encoding: ST4D_Z_P_BR_Contiguous");
                                }
                                _ => unreachable!()
                            }
                        }
                        (_, _, _, _, _, x5, _, _) if x5 == 0 => {
                        }
                        _ => unreachable!()
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
                                    println!("encoding: ST1B_Z_P_BZ_D_x32_unscaled");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: ST1H_Z_P_BZ_D_x32_unscaled");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: ST1W_Z_P_BZ_D_x32_unscaled");
                                }
                                x0 if x0 == 3 => {
                                    println!("encoding: ST1D_Z_P_BZ_D_x32_unscaled");
                                }
                                _ => unreachable!()
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
                                    println!("unallocated");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: ST1H_Z_P_BZ_D_x32_scaled");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: ST1W_Z_P_BZ_D_x32_scaled");
                                }
                                x0 if x0 == 3 => {
                                    println!("encoding: ST1D_Z_P_BZ_D_x32_scaled");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: ST1B_Z_P_BZ_S_x32_unscaled");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: ST1H_Z_P_BZ_S_x32_unscaled");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: ST1W_Z_P_BZ_S_x32_unscaled");
                                }
                                x0 if x0 == 3 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
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
                                    println!("unallocated");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: ST1H_Z_P_BZ_S_x32_scaled");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: ST1W_Z_P_BZ_S_x32_scaled");
                                }
                                x0 if x0 == 3 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
                            }
                        }
                        _ => unreachable!()
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
                                    println!("encoding: ST1B_Z_P_BZ_D_64_unscaled");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: ST1H_Z_P_BZ_D_64_unscaled");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: ST1W_Z_P_BZ_D_64_unscaled");
                                }
                                x0 if x0 == 3 => {
                                    println!("encoding: ST1D_Z_P_BZ_D_64_unscaled");
                                }
                                _ => unreachable!()
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
                                    println!("unallocated");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: ST1H_Z_P_BZ_D_64_scaled");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: ST1W_Z_P_BZ_D_64_scaled");
                                }
                                x0 if x0 == 3 => {
                                    println!("encoding: ST1D_Z_P_BZ_D_64_scaled");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: ST1B_Z_P_AI_D");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: ST1H_Z_P_AI_D");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: ST1W_Z_P_AI_D");
                                }
                                x0 if x0 == 3 => {
                                    println!("encoding: ST1D_Z_P_AI_D");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: ST1B_Z_P_AI_S");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: ST1H_Z_P_AI_S");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: ST1W_Z_P_AI_S");
                                }
                                x0 if x0 == 3 => {
                                    println!("unallocated");
                                }
                                _ => unreachable!()
                            }
                        }
                        _ => unreachable!()
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
                                    println!("encoding: STNT1B_Z_P_BI_Contiguous");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: STNT1H_Z_P_BI_Contiguous");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: STNT1W_Z_P_BI_Contiguous");
                                }
                                x0 if x0 == 3 => {
                                    println!("encoding: STNT1D_Z_P_BI_Contiguous");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: ST2B_Z_P_BI_Contiguous");
                                }
                                (x0, x1) if x0 == 0 && x1 == 2 => {
                                    println!("encoding: ST3B_Z_P_BI_Contiguous");
                                }
                                (x0, x1) if x0 == 0 && x1 == 3 => {
                                    println!("encoding: ST4B_Z_P_BI_Contiguous");
                                }
                                (x0, x1) if x0 == 1 && x1 == 1 => {
                                    println!("encoding: ST2H_Z_P_BI_Contiguous");
                                }
                                (x0, x1) if x0 == 1 && x1 == 2 => {
                                    println!("encoding: ST3H_Z_P_BI_Contiguous");
                                }
                                (x0, x1) if x0 == 1 && x1 == 3 => {
                                    println!("encoding: ST4H_Z_P_BI_Contiguous");
                                }
                                (x0, x1) if x0 == 2 && x1 == 1 => {
                                    println!("encoding: ST2W_Z_P_BI_Contiguous");
                                }
                                (x0, x1) if x0 == 2 && x1 == 2 => {
                                    println!("encoding: ST3W_Z_P_BI_Contiguous");
                                }
                                (x0, x1) if x0 == 2 && x1 == 3 => {
                                    println!("encoding: ST4W_Z_P_BI_Contiguous");
                                }
                                (x0, x1) if x0 == 3 && x1 == 1 => {
                                    println!("encoding: ST2D_Z_P_BI_Contiguous");
                                }
                                (x0, x1) if x0 == 3 && x1 == 2 => {
                                    println!("encoding: ST3D_Z_P_BI_Contiguous");
                                }
                                (x0, x1) if x0 == 3 && x1 == 3 => {
                                    println!("encoding: ST4D_Z_P_BI_Contiguous");
                                }
                                _ => unreachable!()
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
                                    println!("encoding: ST1B_Z_P_BI__");
                                }
                                x0 if x0 == 1 => {
                                    println!("encoding: ST1H_Z_P_BI__");
                                }
                                x0 if x0 == 2 => {
                                    println!("encoding: ST1W_Z_P_BI__");
                                }
                                x0 if x0 == 3 => {
                                    println!("encoding: ST1D_Z_P_BI__");
                                }
                                _ => unreachable!()
                            }
                        }
                        _ => unreachable!()
                    }
                }
                _ => unreachable!()
            }
        }
        (_, x1, _) if x1 & 30 == 6 => {
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
                            println!("encoding: aarch64_integer_arithmetic_address_pc_rel");
                        }
                        x0 if x0 == 1 => {
                            println!("encoding: aarch64_integer_arithmetic_address_pc_rel");
                        }
                        _ => unreachable!()
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
                            println!("encoding: aarch64_integer_arithmetic_add_sub_immediate");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_integer_arithmetic_add_sub_immediate");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_integer_arithmetic_add_sub_immediate");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_integer_arithmetic_add_sub_immediate");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_integer_arithmetic_add_sub_immediate");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_integer_arithmetic_add_sub_immediate");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_integer_arithmetic_add_sub_immediate");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_integer_arithmetic_add_sub_immediate");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (x0, _, _, x3) if x0 == 0 && x3 == 0 => {
                            println!("unallocated");
                        }
                        (x0, _, x2, x3) if x0 == 1 && x2 == 1 && x3 == 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            println!("encoding: aarch64_integer_tags_mcaddtag");
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 0 => {
                            println!("encoding: aarch64_integer_tags_mcsubtag");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_integer_logical_immediate");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_integer_logical_immediate");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 0 => {
                            println!("encoding: aarch64_integer_logical_immediate");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 0 => {
                            println!("encoding: aarch64_integer_logical_immediate");
                        }
                        (x0, x1, _) if x0 == 1 && x1 == 0 => {
                            println!("encoding: aarch64_integer_logical_immediate");
                        }
                        (x0, x1, _) if x0 == 1 && x1 == 1 => {
                            println!("encoding: aarch64_integer_logical_immediate");
                        }
                        (x0, x1, _) if x0 == 1 && x1 == 2 => {
                            println!("encoding: aarch64_integer_logical_immediate");
                        }
                        (x0, x1, _) if x0 == 1 && x1 == 3 => {
                            println!("encoding: aarch64_integer_logical_immediate");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 0 && x2 & 2 == 2 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 & 2 == 0 => {
                            println!("encoding: aarch64_integer_ins_ext_insert_movewide");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 & 2 == 0 => {
                            println!("encoding: aarch64_integer_ins_ext_insert_movewide");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 & 2 == 0 => {
                            println!("encoding: aarch64_integer_ins_ext_insert_movewide");
                        }
                        (x0, x1, _) if x0 == 1 && x1 == 0 => {
                            println!("encoding: aarch64_integer_ins_ext_insert_movewide");
                        }
                        (x0, x1, _) if x0 == 1 && x1 == 2 => {
                            println!("encoding: aarch64_integer_ins_ext_insert_movewide");
                        }
                        (x0, x1, _) if x0 == 1 && x1 == 3 => {
                            println!("encoding: aarch64_integer_ins_ext_insert_movewide");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_integer_bitfield");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_integer_bitfield");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 0 => {
                            println!("encoding: aarch64_integer_bitfield");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_integer_bitfield");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_integer_bitfield");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 == 1 => {
                            println!("encoding: aarch64_integer_bitfield");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (_, x1, _, x3, _) if x1 == 0 && x3 == 1 => {
                            println!("unallocated");
                        }
                        (_, x1, _, _, _) if x1 & 2 == 2 => {
                            println!("unallocated");
                        }
                        (x0, _, _, _, x4) if x0 == 0 && x4 & 32 == 32 => {
                            println!("unallocated");
                        }
                        (x0, _, x2, _, _) if x0 == 0 && x2 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 & 32 == 0 => {
                            println!("encoding: aarch64_integer_ins_ext_extract_immediate");
                        }
                        (x0, _, x2, _, _) if x0 == 1 && x2 == 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            println!("encoding: aarch64_integer_ins_ext_extract_immediate");
                        }
                        _ => unreachable!()
                    }
                }
                _ => unreachable!()
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
                            println!("encoding: aarch64_branch_conditional_cond");
                        }
                        (x0, x1) if x0 == 0 && x1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, _) if x0 == 1 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, _, _) if x0 == 6 && x2 & 12288 == 0 => {
                    let opc = (instr >> 21) & 5;
                    let imm16 = (instr >> 5) & 31;
                    let op2 = (instr >> 2) & 5;
                    let LL = instr & 3;
                    match (opc, op2, LL) {
                        (_, x1, _) if x1 == 1 => {
                            println!("unallocated");
                        }
                        (_, x1, _) if x1 & 6 == 2 => {
                            println!("unallocated");
                        }
                        (_, x1, _) if x1 & 4 == 4 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_system_exceptions_runtime_svc");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 2 => {
                            println!("encoding: aarch64_system_exceptions_runtime_hvc");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 3 => {
                            println!("encoding: aarch64_system_exceptions_runtime_smc");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 & 1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_system_exceptions_debug_breakpoint");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 & 2 == 2 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 & 1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_system_exceptions_debug_halt");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 & 2 == 2 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 & 2 == 2 => {
                            println!("unallocated");
                        }
                        (x0, x1, _) if x0 == 4 && x1 == 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 5 && x1 == 0 && x2 == 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 5 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_system_exceptions_debug_exception");
                        }
                        (x0, x1, x2) if x0 == 5 && x1 == 0 && x2 == 2 => {
                            println!("encoding: aarch64_system_exceptions_debug_exception");
                        }
                        (x0, x1, x2) if x0 == 5 && x1 == 0 && x2 == 3 => {
                            println!("encoding: aarch64_system_exceptions_debug_exception");
                        }
                        (x0, x1, _) if x0 == 6 && x1 == 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, _) if x0 == 7 && x1 == 0 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, _, x4) if x0 == 6 && x2 == 4146 && x4 == 31 => {
                    let CRm = (instr >> 8) & 7;
                    let op2 = (instr >> 5) & 5;
                    match (CRm, op2) {
                        (_, _) => {
                            println!("encoding: aarch64_system_hints");
                        }
                        (x0, x1) if x0 == 0 && x1 == 0 => {
                            println!("encoding: aarch64_system_hints");
                        }
                        (x0, x1) if x0 == 0 && x1 == 1 => {
                            println!("encoding: aarch64_system_hints");
                        }
                        (x0, x1) if x0 == 0 && x1 == 2 => {
                            println!("encoding: aarch64_system_hints");
                        }
                        (x0, x1) if x0 == 0 && x1 == 3 => {
                            println!("encoding: aarch64_system_hints");
                        }
                        (x0, x1) if x0 == 0 && x1 == 4 => {
                            println!("encoding: aarch64_system_hints");
                        }
                        (x0, x1) if x0 == 0 && x1 == 5 => {
                            println!("encoding: aarch64_system_hints");
                        }
                        (x0, x1) if x0 == 0 && x1 == 6 => {
                            println!("encoding: aarch64_system_hints");
                        }
                        (x0, x1) if x0 == 0 && x1 == 7 => {
                            println!("encoding: aarch64_integer_pac_strip_hint");
                        }
                        (x0, x1) if x0 == 1 && x1 == 0 => {
                            println!("encoding: aarch64_integer_pac_pacia_hint");
                        }
                        (x0, x1) if x0 == 1 && x1 == 2 => {
                            println!("encoding: aarch64_integer_pac_pacib_hint");
                        }
                        (x0, x1) if x0 == 1 && x1 == 4 => {
                            println!("encoding: aarch64_integer_pac_autia_hint");
                        }
                        (x0, x1) if x0 == 1 && x1 == 6 => {
                            println!("encoding: aarch64_integer_pac_autib_hint");
                        }
                        (x0, x1) if x0 == 2 && x1 == 0 => {
                            println!("encoding: aarch64_system_hints");
                        }
                        (x0, x1) if x0 == 2 && x1 == 1 => {
                            println!("encoding: aarch64_system_hints");
                        }
                        (x0, x1) if x0 == 2 && x1 == 2 => {
                            println!("encoding: aarch64_system_hints");
                        }
                        (x0, x1) if x0 == 2 && x1 == 4 => {
                            println!("encoding: aarch64_system_hints");
                        }
                        (x0, x1) if x0 == 3 && x1 == 0 => {
                            println!("encoding: aarch64_integer_pac_pacia_hint");
                        }
                        (x0, x1) if x0 == 3 && x1 == 1 => {
                            println!("encoding: aarch64_integer_pac_pacia_hint");
                        }
                        (x0, x1) if x0 == 3 && x1 == 2 => {
                            println!("encoding: aarch64_integer_pac_pacib_hint");
                        }
                        (x0, x1) if x0 == 3 && x1 == 3 => {
                            println!("encoding: aarch64_integer_pac_pacib_hint");
                        }
                        (x0, x1) if x0 == 3 && x1 == 4 => {
                            println!("encoding: aarch64_integer_pac_autia_hint");
                        }
                        (x0, x1) if x0 == 3 && x1 == 5 => {
                            println!("encoding: aarch64_integer_pac_autia_hint");
                        }
                        (x0, x1) if x0 == 3 && x1 == 6 => {
                            println!("encoding: aarch64_integer_pac_autib_hint");
                        }
                        (x0, x1) if x0 == 3 && x1 == 7 => {
                            println!("encoding: aarch64_integer_pac_autib_hint");
                        }
                        (x0, x1) if x0 == 4 && x1 & 1 == 0 => {
                            println!("encoding: aarch64_system_hints");
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, _, _) if x0 == 6 && x2 == 4147 => {
                    let CRm = (instr >> 8) & 7;
                    let op2 = (instr >> 5) & 5;
                    let Rt = instr & 9;
                    match (CRm, op2, Rt) {
                        (_, x1, _) if x1 == 0 => {
                            println!("unallocated");
                        }
                        (_, x1, _) if x1 == 1 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 == 2 && x2 == 31 => {
                            println!("encoding: aarch64_system_monitors");
                        }
                        (_, x1, x2) if x1 == 5 && x2 == 31 => {
                            println!("encoding: aarch64_system_barriers_dmb");
                        }
                        (_, x1, x2) if x1 == 6 && x2 == 31 => {
                            println!("encoding: aarch64_system_barriers_isb");
                        }
                        (_, x1, x2) if x1 == 7 && x2 != 31 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 == 7 && x2 == 31 => {
                            println!("encoding: aarch64_system_barriers_sb");
                        }
                        (x0, x1, x2) if x0 & 11 != 0 && x1 == 4 && x2 == 31 => {
                            println!("encoding: aarch64_system_barriers_dsb");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 4 && x2 == 31 => {
                            println!("encoding: aarch64_system_barriers_ssbb");
                        }
                        (x0, x1, _) if x0 == 1 && x1 == 3 => {
                            println!("unallocated");
                        }
                        (x0, x1, _) if x0 & 14 == 2 && x1 == 3 => {
                            println!("unallocated");
                        }
                        (x0, x1, _) if x0 & 12 == 4 && x1 == 3 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 4 && x1 == 4 && x2 == 31 => {
                            println!("encoding: aarch64_system_barriers_pssbb");
                        }
                        (x0, x1, _) if x0 & 8 == 8 && x1 == 3 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, _, _) if x0 == 6 && x2 & 16271 == 4100 => {
                    let op1 = (instr >> 16) & 5;
                    let CRm = (instr >> 8) & 7;
                    let op2 = (instr >> 5) & 5;
                    let Rt = instr & 9;
                    match (op1, op2, Rt) {
                        (_, _, x2) if x2 != 31 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 == 31 => {
                            println!("encoding: aarch64_system_register_cpsr");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 31 => {
                            println!("encoding: aarch64_integer_flags_cfinv");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 31 => {
                            println!("encoding: aarch64_integer_flags_xaflag");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 31 => {
                            println!("encoding: aarch64_integer_flags_axflag");
                        }
                        _ => unreachable!()
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
                            println!("encoding: aarch64_system_sysops");
                        }
                        x0 if x0 == 1 => {
                            println!("encoding: aarch64_system_sysops");
                        }
                        _ => unreachable!()
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
                            println!("encoding: aarch64_system_register_system");
                        }
                        x0 if x0 == 1 => {
                            println!("encoding: aarch64_system_register_system");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 31 && x2 == 0 && x4 != 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 31 && x2 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_branch_unconditional_register");
                        }
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 31 && x2 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 31 && x2 == 2 && x4 != 31 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 31 && x2 == 2 && x4 == 31 => {
                            println!("encoding: aarch64_branch_unconditional_register");
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 31 && x2 == 3 && x4 != 31 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 31 && x2 == 3 && x4 == 31 => {
                            println!("encoding: aarch64_branch_unconditional_register");
                        }
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 31 && x2 & 60 == 4 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 31 && x2 & 56 == 8 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 31 && x2 & 48 == 16 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 31 && x2 & 32 == 32 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 31 && x2 == 0 && x4 != 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 31 && x2 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_branch_unconditional_register");
                        }
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 31 && x2 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 31 && x2 == 2 && x4 != 31 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 31 && x2 == 2 && x4 == 31 => {
                            println!("encoding: aarch64_branch_unconditional_register");
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 31 && x2 == 3 && x4 != 31 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 31 && x2 == 3 && x4 == 31 => {
                            println!("encoding: aarch64_branch_unconditional_register");
                        }
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 31 && x2 & 60 == 4 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 31 && x2 & 56 == 8 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 31 && x2 & 48 == 16 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 31 && x2 & 32 == 32 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, x4) if x0 == 2 && x1 == 31 && x2 == 0 && x4 != 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, x4) if x0 == 2 && x1 == 31 && x2 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_branch_unconditional_register");
                        }
                        (x0, x1, x2, _, _) if x0 == 2 && x1 == 31 && x2 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 2 && x1 == 31 && x2 == 2 && x3 != 31 && x4 != 31 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 2 && x1 == 31 && x2 == 2 && x3 == 31 && x4 == 31 => {
                            println!("encoding: aarch64_branch_unconditional_register");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 2 && x1 == 31 && x2 == 3 && x3 != 31 && x4 != 31 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 2 && x1 == 31 && x2 == 3 && x3 == 31 && x4 == 31 => {
                            println!("encoding: aarch64_branch_unconditional_register");
                        }
                        (x0, x1, x2, _, _) if x0 == 2 && x1 == 31 && x2 & 60 == 4 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, _) if x0 == 2 && x1 == 31 && x2 & 56 == 8 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, _) if x0 == 2 && x1 == 31 && x2 & 48 == 16 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, _) if x0 == 2 && x1 == 31 && x2 & 32 == 32 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, _, _) if x0 == 3 && x1 == 31 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 4 && x1 == 31 && x2 == 0 && x3 != 31 && x4 != 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 4 && x1 == 31 && x2 == 0 && x3 != 31 && x4 == 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 4 && x1 == 31 && x2 == 0 && x3 == 31 && x4 != 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 4 && x1 == 31 && x2 == 0 && x3 == 31 && x4 == 0 => {
                            println!("encoding: aarch64_branch_unconditional_eret");
                        }
                        (x0, x1, x2, _, _) if x0 == 4 && x1 == 31 && x2 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 4 && x1 == 31 && x2 == 2 && x3 != 31 && x4 != 31 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 4 && x1 == 31 && x2 == 2 && x3 != 31 && x4 == 31 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 4 && x1 == 31 && x2 == 2 && x3 == 31 && x4 != 31 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 4 && x1 == 31 && x2 == 2 && x3 == 31 && x4 == 31 => {
                            println!("encoding: aarch64_branch_unconditional_eret");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 4 && x1 == 31 && x2 == 3 && x3 != 31 && x4 != 31 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 4 && x1 == 31 && x2 == 3 && x3 != 31 && x4 == 31 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 4 && x1 == 31 && x2 == 3 && x3 == 31 && x4 != 31 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 4 && x1 == 31 && x2 == 3 && x3 == 31 && x4 == 31 => {
                            println!("encoding: aarch64_branch_unconditional_eret");
                        }
                        (x0, x1, x2, _, _) if x0 == 4 && x1 == 31 && x2 & 60 == 4 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, _) if x0 == 4 && x1 == 31 && x2 & 56 == 8 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, _) if x0 == 4 && x1 == 31 && x2 & 48 == 16 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, _) if x0 == 4 && x1 == 31 && x2 & 32 == 32 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, _) if x0 == 5 && x1 == 31 && x2 != 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 5 && x1 == 31 && x2 == 0 && x3 != 31 && x4 != 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 5 && x1 == 31 && x2 == 0 && x3 != 31 && x4 == 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 5 && x1 == 31 && x2 == 0 && x3 == 31 && x4 != 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 5 && x1 == 31 && x2 == 0 && x3 == 31 && x4 == 0 => {
                            println!("encoding: aarch64_branch_unconditional_dret");
                        }
                        (x0, x1, _, _, _) if x0 & 14 == 6 && x1 == 31 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, _) if x0 == 8 && x1 == 31 && x2 & 62 == 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, _) if x0 == 8 && x1 == 31 && x2 == 2 => {
                            println!("encoding: aarch64_branch_unconditional_register");
                        }
                        (x0, x1, x2, _, _) if x0 == 8 && x1 == 31 && x2 == 3 => {
                            println!("encoding: aarch64_branch_unconditional_register");
                        }
                        (x0, x1, x2, _, _) if x0 == 8 && x1 == 31 && x2 & 60 == 4 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, _) if x0 == 8 && x1 == 31 && x2 & 56 == 8 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, _) if x0 == 8 && x1 == 31 && x2 & 48 == 16 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, _) if x0 == 8 && x1 == 31 && x2 & 32 == 32 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, _) if x0 == 9 && x1 == 31 && x2 & 62 == 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, _) if x0 == 9 && x1 == 31 && x2 == 2 => {
                            println!("encoding: aarch64_branch_unconditional_register");
                        }
                        (x0, x1, x2, _, _) if x0 == 9 && x1 == 31 && x2 == 3 => {
                            println!("encoding: aarch64_branch_unconditional_register");
                        }
                        (x0, x1, x2, _, _) if x0 == 9 && x1 == 31 && x2 & 60 == 4 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, _) if x0 == 9 && x1 == 31 && x2 & 56 == 8 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, _) if x0 == 9 && x1 == 31 && x2 & 48 == 16 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, _) if x0 == 9 && x1 == 31 && x2 & 32 == 32 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, _, _) if x0 & 14 == 10 && x1 == 31 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, _, _) if x0 & 12 == 12 && x1 == 31 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, _, _, _) if x0 & 3 == 0 => {
                    let op = (instr >> 31) & 1;
                    let imm26 = instr & 51;
                    match op {
                        x0 if x0 == 0 => {
                            println!("encoding: aarch64_branch_unconditional_immediate");
                        }
                        x0 if x0 == 1 => {
                            println!("encoding: aarch64_branch_unconditional_immediate");
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, _, _) if x0 & 3 == 1 && x2 & 8192 == 0 => {
                    let sf = (instr >> 31) & 1;
                    let op = (instr >> 24) & 1;
                    let imm19 = (instr >> 5) & 37;
                    let Rt = instr & 9;
                    match (sf, op) {
                        (x0, x1) if x0 == 0 && x1 == 0 => {
                            println!("encoding: aarch64_branch_conditional_compare");
                        }
                        (x0, x1) if x0 == 0 && x1 == 1 => {
                            println!("encoding: aarch64_branch_conditional_compare");
                        }
                        (x0, x1) if x0 == 1 && x1 == 0 => {
                            println!("encoding: aarch64_branch_conditional_compare");
                        }
                        (x0, x1) if x0 == 1 && x1 == 1 => {
                            println!("encoding: aarch64_branch_conditional_compare");
                        }
                        _ => unreachable!()
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
                            println!("encoding: aarch64_branch_conditional_test");
                        }
                        x0 if x0 == 1 => {
                            println!("encoding: aarch64_branch_conditional_test");
                        }
                        _ => unreachable!()
                    }
                }
                _ => unreachable!()
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
                            println!("encoding: aarch64_memory_vector_multiple_no_wb");
                        }
                        (x0, x1) if x0 == 0 && x1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1) if x0 == 0 && x1 == 2 => {
                            println!("encoding: aarch64_memory_vector_multiple_no_wb");
                        }
                        (x0, x1) if x0 == 0 && x1 == 3 => {
                            println!("unallocated");
                        }
                        (x0, x1) if x0 == 0 && x1 == 4 => {
                            println!("encoding: aarch64_memory_vector_multiple_no_wb");
                        }
                        (x0, x1) if x0 == 0 && x1 == 5 => {
                            println!("unallocated");
                        }
                        (x0, x1) if x0 == 0 && x1 == 6 => {
                            println!("encoding: aarch64_memory_vector_multiple_no_wb");
                        }
                        (x0, x1) if x0 == 0 && x1 == 7 => {
                            println!("encoding: aarch64_memory_vector_multiple_no_wb");
                        }
                        (x0, x1) if x0 == 0 && x1 == 8 => {
                            println!("encoding: aarch64_memory_vector_multiple_no_wb");
                        }
                        (x0, x1) if x0 == 0 && x1 == 9 => {
                            println!("unallocated");
                        }
                        (x0, x1) if x0 == 0 && x1 == 10 => {
                            println!("encoding: aarch64_memory_vector_multiple_no_wb");
                        }
                        (x0, x1) if x0 == 0 && x1 == 11 => {
                            println!("unallocated");
                        }
                        (x0, x1) if x0 == 0 && x1 & 12 == 12 => {
                            println!("unallocated");
                        }
                        (x0, x1) if x0 == 1 && x1 == 0 => {
                            println!("encoding: aarch64_memory_vector_multiple_no_wb");
                        }
                        (x0, x1) if x0 == 1 && x1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1) if x0 == 1 && x1 == 2 => {
                            println!("encoding: aarch64_memory_vector_multiple_no_wb");
                        }
                        (x0, x1) if x0 == 1 && x1 == 3 => {
                            println!("unallocated");
                        }
                        (x0, x1) if x0 == 1 && x1 == 4 => {
                            println!("encoding: aarch64_memory_vector_multiple_no_wb");
                        }
                        (x0, x1) if x0 == 1 && x1 == 5 => {
                            println!("unallocated");
                        }
                        (x0, x1) if x0 == 1 && x1 == 6 => {
                            println!("encoding: aarch64_memory_vector_multiple_no_wb");
                        }
                        (x0, x1) if x0 == 1 && x1 == 7 => {
                            println!("encoding: aarch64_memory_vector_multiple_no_wb");
                        }
                        (x0, x1) if x0 == 1 && x1 == 8 => {
                            println!("encoding: aarch64_memory_vector_multiple_no_wb");
                        }
                        (x0, x1) if x0 == 1 && x1 == 9 => {
                            println!("unallocated");
                        }
                        (x0, x1) if x0 == 1 && x1 == 10 => {
                            println!("encoding: aarch64_memory_vector_multiple_no_wb");
                        }
                        (x0, x1) if x0 == 1 && x1 == 11 => {
                            println!("unallocated");
                        }
                        (x0, x1) if x0 == 1 && x1 & 12 == 12 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 3 => {
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 5 => {
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 9 => {
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 11 => {
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 0 && x2 & 12 == 12 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 31 && x2 == 0 => {
                            println!("encoding: aarch64_memory_vector_multiple_post_inc");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 31 && x2 == 2 => {
                            println!("encoding: aarch64_memory_vector_multiple_post_inc");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 31 && x2 == 4 => {
                            println!("encoding: aarch64_memory_vector_multiple_post_inc");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 31 && x2 == 6 => {
                            println!("encoding: aarch64_memory_vector_multiple_post_inc");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 31 && x2 == 7 => {
                            println!("encoding: aarch64_memory_vector_multiple_post_inc");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 31 && x2 == 8 => {
                            println!("encoding: aarch64_memory_vector_multiple_post_inc");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 31 && x2 == 10 => {
                            println!("encoding: aarch64_memory_vector_multiple_post_inc");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 31 && x2 == 0 => {
                            println!("encoding: aarch64_memory_vector_multiple_post_inc");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 31 && x2 == 2 => {
                            println!("encoding: aarch64_memory_vector_multiple_post_inc");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 31 && x2 == 4 => {
                            println!("encoding: aarch64_memory_vector_multiple_post_inc");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 31 && x2 == 6 => {
                            println!("encoding: aarch64_memory_vector_multiple_post_inc");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 31 && x2 == 7 => {
                            println!("encoding: aarch64_memory_vector_multiple_post_inc");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 31 && x2 == 8 => {
                            println!("encoding: aarch64_memory_vector_multiple_post_inc");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 31 && x2 == 10 => {
                            println!("encoding: aarch64_memory_vector_multiple_post_inc");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 1 => {
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 3 => {
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 5 => {
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 9 => {
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 11 => {
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 1 && x2 & 12 == 12 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 31 && x2 == 0 => {
                            println!("encoding: aarch64_memory_vector_multiple_post_inc");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 31 && x2 == 2 => {
                            println!("encoding: aarch64_memory_vector_multiple_post_inc");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 31 && x2 == 4 => {
                            println!("encoding: aarch64_memory_vector_multiple_post_inc");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 31 && x2 == 6 => {
                            println!("encoding: aarch64_memory_vector_multiple_post_inc");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 31 && x2 == 7 => {
                            println!("encoding: aarch64_memory_vector_multiple_post_inc");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 31 && x2 == 8 => {
                            println!("encoding: aarch64_memory_vector_multiple_post_inc");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 31 && x2 == 10 => {
                            println!("encoding: aarch64_memory_vector_multiple_post_inc");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 31 && x2 == 0 => {
                            println!("encoding: aarch64_memory_vector_multiple_post_inc");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 31 && x2 == 2 => {
                            println!("encoding: aarch64_memory_vector_multiple_post_inc");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 31 && x2 == 4 => {
                            println!("encoding: aarch64_memory_vector_multiple_post_inc");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 31 && x2 == 6 => {
                            println!("encoding: aarch64_memory_vector_multiple_post_inc");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 31 && x2 == 7 => {
                            println!("encoding: aarch64_memory_vector_multiple_post_inc");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 31 && x2 == 8 => {
                            println!("encoding: aarch64_memory_vector_multiple_post_inc");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 31 && x2 == 10 => {
                            println!("encoding: aarch64_memory_vector_multiple_post_inc");
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, _, x4, _, x6, _, _, _) if x0 & 11 == 0 && x2 == 1 && x4 & 2 == 0 && x6 & 32 == 32 => {
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
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_no_wb");
                        }
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_memory_vector_single_no_wb");
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 0 && x2 == 2 && x4 & 1 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_no_wb");
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 0 && x2 == 2 && x4 & 1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x4 & 1 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_no_wb");
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x4 & 1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 0 && x2 == 4 && x4 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_no_wb");
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 0 && x2 == 4 && x4 & 2 == 2 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 4 && x3 == 0 && x4 == 1 => {
                            println!("encoding: aarch64_memory_vector_single_no_wb");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 4 && x3 == 1 && x4 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 0 && x2 == 5 && x4 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_no_wb");
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 0 && x2 == 5 && x4 == 2 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 5 && x3 == 0 && x4 == 1 => {
                            println!("encoding: aarch64_memory_vector_single_no_wb");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 5 && x3 == 0 && x4 == 3 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 5 && x3 == 1 && x4 & 1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_no_wb");
                        }
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_memory_vector_single_no_wb");
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 1 && x2 == 2 && x4 & 1 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_no_wb");
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 1 && x2 == 2 && x4 & 1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 1 && x2 == 3 && x4 & 1 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_no_wb");
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 1 && x2 == 3 && x4 & 1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 1 && x2 == 4 && x4 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_no_wb");
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 1 && x2 == 4 && x4 == 2 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 1 && x2 == 4 && x3 == 0 && x4 == 1 => {
                            println!("encoding: aarch64_memory_vector_single_no_wb");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 1 && x2 == 4 && x3 == 0 && x4 == 3 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 1 && x2 == 4 && x3 == 1 && x4 & 1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 1 && x2 == 5 && x4 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_no_wb");
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 1 && x2 == 5 && x4 == 2 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 1 && x2 == 5 && x3 == 0 && x4 == 1 => {
                            println!("encoding: aarch64_memory_vector_single_no_wb");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 1 && x2 == 5 && x3 == 0 && x4 == 3 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 1 && x2 == 5 && x3 == 1 && x4 & 1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_no_wb");
                        }
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_memory_vector_single_no_wb");
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 0 && x2 == 2 && x4 & 1 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_no_wb");
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 0 && x2 == 2 && x4 & 1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 0 && x2 == 3 && x4 & 1 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_no_wb");
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 0 && x2 == 3 && x4 & 1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 0 && x2 == 4 && x4 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_no_wb");
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 0 && x2 == 4 && x4 & 2 == 2 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 4 && x3 == 0 && x4 == 1 => {
                            println!("encoding: aarch64_memory_vector_single_no_wb");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 4 && x3 == 1 && x4 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 0 && x2 == 5 && x4 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_no_wb");
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 0 && x2 == 5 && x4 == 2 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 5 && x3 == 0 && x4 == 1 => {
                            println!("encoding: aarch64_memory_vector_single_no_wb");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 5 && x3 == 0 && x4 == 3 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 5 && x3 == 1 && x4 & 1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 6 && x3 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_no_wb");
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 6 && x3 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 7 && x3 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_no_wb");
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 7 && x3 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_no_wb");
                        }
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_memory_vector_single_no_wb");
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 1 && x2 == 2 && x4 & 1 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_no_wb");
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 1 && x2 == 2 && x4 & 1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 1 && x2 == 3 && x4 & 1 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_no_wb");
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 1 && x2 == 3 && x4 & 1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 1 && x2 == 4 && x4 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_no_wb");
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 1 && x2 == 4 && x4 == 2 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 1 && x2 == 4 && x3 == 0 && x4 == 1 => {
                            println!("encoding: aarch64_memory_vector_single_no_wb");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 1 && x2 == 4 && x3 == 0 && x4 == 3 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 1 && x2 == 4 && x3 == 1 && x4 & 1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 1 && x2 == 5 && x4 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_no_wb");
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 1 && x2 == 5 && x4 == 2 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 1 && x2 == 5 && x3 == 0 && x4 == 1 => {
                            println!("encoding: aarch64_memory_vector_single_no_wb");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 1 && x2 == 5 && x3 == 0 && x4 == 3 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 1 && x2 == 5 && x3 == 1 && x4 & 1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 1 && x2 == 6 && x3 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_no_wb");
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 1 && x2 == 6 && x3 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 1 && x2 == 7 && x3 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_no_wb");
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 1 && x2 == 7 && x3 == 1 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 0 && x1 == 0 && x3 == 2 && x5 & 1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 0 && x1 == 0 && x3 == 3 && x5 & 1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 0 && x1 == 0 && x3 == 4 && x5 & 2 == 2 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, x3, x4, x5) if x0 == 0 && x1 == 0 && x3 == 4 && x4 == 1 && x5 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 0 && x1 == 0 && x3 == 5 && x5 == 2 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, x3, x4, x5) if x0 == 0 && x1 == 0 && x3 == 5 && x4 == 0 && x5 == 3 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, x3, x4, x5) if x0 == 0 && x1 == 0 && x3 == 5 && x4 == 1 && x5 & 1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 0 && x2 != 31 && x3 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 0 && x2 != 31 && x3 == 1 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 0 && x2 != 31 && x3 == 2 && x5 & 1 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 0 && x2 != 31 && x3 == 3 && x5 & 1 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 0 && x2 != 31 && x3 == 4 && x5 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 != 31 && x3 == 4 && x4 == 0 && x5 == 1 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 0 && x2 != 31 && x3 == 5 && x5 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 != 31 && x3 == 5 && x4 == 0 && x5 == 1 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 0 && x2 == 31 && x3 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 0 && x2 == 31 && x3 == 1 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 0 && x2 == 31 && x3 == 2 && x5 & 1 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 0 && x2 == 31 && x3 == 3 && x5 & 1 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 0 && x2 == 31 && x3 == 4 && x5 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 31 && x3 == 4 && x4 == 0 && x5 == 1 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 0 && x2 == 31 && x3 == 5 && x5 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 31 && x3 == 5 && x4 == 0 && x5 == 1 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 0 && x1 == 1 && x3 == 2 && x5 & 1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 0 && x1 == 1 && x3 == 3 && x5 & 1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 0 && x1 == 1 && x3 == 4 && x5 == 2 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, x3, x4, x5) if x0 == 0 && x1 == 1 && x3 == 4 && x4 == 0 && x5 == 3 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, x3, x4, x5) if x0 == 0 && x1 == 1 && x3 == 4 && x4 == 1 && x5 & 1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 0 && x1 == 1 && x3 == 5 && x5 == 2 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, x3, x4, x5) if x0 == 0 && x1 == 1 && x3 == 5 && x4 == 0 && x5 == 3 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, x3, x4, x5) if x0 == 0 && x1 == 1 && x3 == 5 && x4 == 1 && x5 & 1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 1 && x2 != 31 && x3 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 1 && x2 != 31 && x3 == 1 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 1 && x2 != 31 && x3 == 2 && x5 & 1 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 1 && x2 != 31 && x3 == 3 && x5 & 1 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 1 && x2 != 31 && x3 == 4 && x5 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 1 && x2 != 31 && x3 == 4 && x4 == 0 && x5 == 1 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 1 && x2 != 31 && x3 == 5 && x5 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 1 && x2 != 31 && x3 == 5 && x4 == 0 && x5 == 1 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 1 && x2 == 31 && x3 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 1 && x2 == 31 && x3 == 1 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 1 && x2 == 31 && x3 == 2 && x5 & 1 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 1 && x2 == 31 && x3 == 3 && x5 & 1 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 1 && x2 == 31 && x3 == 4 && x5 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 1 && x2 == 31 && x3 == 4 && x4 == 0 && x5 == 1 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 0 && x1 == 1 && x2 == 31 && x3 == 5 && x5 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 1 && x2 == 31 && x3 == 5 && x4 == 0 && x5 == 1 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 1 && x1 == 0 && x3 == 2 && x5 & 1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 1 && x1 == 0 && x3 == 3 && x5 & 1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 1 && x1 == 0 && x3 == 4 && x5 & 2 == 2 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, x3, x4, x5) if x0 == 1 && x1 == 0 && x3 == 4 && x4 == 1 && x5 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 1 && x1 == 0 && x3 == 5 && x5 == 2 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, x3, x4, x5) if x0 == 1 && x1 == 0 && x3 == 5 && x4 == 0 && x5 == 3 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, x3, x4, x5) if x0 == 1 && x1 == 0 && x3 == 5 && x4 == 1 && x5 & 1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, x3, x4, _) if x0 == 1 && x1 == 0 && x3 == 6 && x4 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, x3, x4, _) if x0 == 1 && x1 == 0 && x3 == 7 && x4 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 1 && x1 == 0 && x2 != 31 && x3 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 1 && x1 == 0 && x2 != 31 && x3 == 1 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 1 && x1 == 0 && x2 != 31 && x3 == 2 && x5 & 1 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 1 && x1 == 0 && x2 != 31 && x3 == 3 && x5 & 1 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 1 && x1 == 0 && x2 != 31 && x3 == 4 && x5 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 != 31 && x3 == 4 && x4 == 0 && x5 == 1 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 1 && x1 == 0 && x2 != 31 && x3 == 5 && x5 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 != 31 && x3 == 5 && x4 == 0 && x5 == 1 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 != 31 && x3 == 6 && x4 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 != 31 && x3 == 7 && x4 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 1 && x1 == 0 && x2 == 31 && x3 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 1 && x1 == 0 && x2 == 31 && x3 == 1 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 1 && x1 == 0 && x2 == 31 && x3 == 2 && x5 & 1 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 1 && x1 == 0 && x2 == 31 && x3 == 3 && x5 & 1 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 1 && x1 == 0 && x2 == 31 && x3 == 4 && x5 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 31 && x3 == 4 && x4 == 0 && x5 == 1 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 1 && x1 == 0 && x2 == 31 && x3 == 5 && x5 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 31 && x3 == 5 && x4 == 0 && x5 == 1 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 == 31 && x3 == 6 && x4 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 == 31 && x3 == 7 && x4 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 1 && x1 == 1 && x3 == 2 && x5 & 1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 1 && x1 == 1 && x3 == 3 && x5 & 1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 1 && x1 == 1 && x3 == 4 && x5 == 2 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, x3, x4, x5) if x0 == 1 && x1 == 1 && x3 == 4 && x4 == 0 && x5 == 3 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, x3, x4, x5) if x0 == 1 && x1 == 1 && x3 == 4 && x4 == 1 && x5 & 1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 1 && x1 == 1 && x3 == 5 && x5 == 2 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, x3, x4, x5) if x0 == 1 && x1 == 1 && x3 == 5 && x4 == 0 && x5 == 3 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, x3, x4, x5) if x0 == 1 && x1 == 1 && x3 == 5 && x4 == 1 && x5 & 1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, x3, x4, _) if x0 == 1 && x1 == 1 && x3 == 6 && x4 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, x3, x4, _) if x0 == 1 && x1 == 1 && x3 == 7 && x4 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 1 && x1 == 1 && x2 != 31 && x3 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 1 && x1 == 1 && x2 != 31 && x3 == 1 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 1 && x1 == 1 && x2 != 31 && x3 == 2 && x5 & 1 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 1 && x1 == 1 && x2 != 31 && x3 == 3 && x5 & 1 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 1 && x1 == 1 && x2 != 31 && x3 == 4 && x5 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 1 && x2 != 31 && x3 == 4 && x4 == 0 && x5 == 1 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 1 && x1 == 1 && x2 != 31 && x3 == 5 && x5 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 1 && x2 != 31 && x3 == 5 && x4 == 0 && x5 == 1 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 1 && x2 != 31 && x3 == 6 && x4 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 1 && x2 != 31 && x3 == 7 && x4 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 1 && x1 == 1 && x2 == 31 && x3 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 1 && x1 == 1 && x2 == 31 && x3 == 1 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 1 && x1 == 1 && x2 == 31 && x3 == 2 && x5 & 1 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 1 && x1 == 1 && x2 == 31 && x3 == 3 && x5 & 1 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 1 && x1 == 1 && x2 == 31 && x3 == 4 && x5 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 1 && x2 == 31 && x3 == 4 && x4 == 0 && x5 == 1 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, _, x5) if x0 == 1 && x1 == 1 && x2 == 31 && x3 == 5 && x5 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 1 && x2 == 31 && x3 == 5 && x4 == 0 && x5 == 1 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 1 && x2 == 31 && x3 == 6 && x4 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 1 && x2 == 31 && x3 == 7 && x4 == 0 => {
                            println!("encoding: aarch64_memory_vector_single_post_inc");
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, _, x4, _, x6, _, _, _) if x0 & 11 == 0 && x2 == 1 && x4 & 1 == 0 && x6 & 16 == 16 => {
                }
                (x0, _, x2, _, x4, _, x6, _, _, _) if x0 & 11 == 0 && x2 == 1 && x4 & 1 == 0 && x6 & 8 == 8 => {
                }
                (x0, _, x2, _, x4, _, x6, _, _, _) if x0 & 11 == 0 && x2 == 1 && x4 & 1 == 0 && x6 & 4 == 4 => {
                }
                (x0, _, x2, _, x4, _, x6, _, _, _) if x0 & 11 == 0 && x2 == 1 && x4 & 1 == 0 && x6 & 2 == 2 => {
                }
                (x0, _, x2, _, x4, _, x6, _, _, _) if x0 & 11 == 0 && x2 == 1 && x4 & 1 == 0 && x6 & 1 == 1 => {
                }
                (x0, _, x2, _, x4, _, x6, _, _, _) if x0 == 13 && x2 == 0 && x4 & 2 == 2 && x6 & 32 == 32 => {
                    let opc = (instr >> 22) & 3;
                    let imm9 = (instr >> 12) & 17;
                    let op2 = (instr >> 10) & 3;
                    let Rn = (instr >> 5) & 9;
                    let Rt = instr & 9;
                    match (opc, imm9, op2) {
                        (x0, _, x2) if x0 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_integer_tags_mcsettagpost");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 2 => {
                            println!("encoding: aarch64_integer_tags_mcsettag");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 3 => {
                            println!("encoding: aarch64_integer_tags_mcsettagpre");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_integer_tags_mcsettagandzeroarray");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_integer_tags_mcgettag");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_integer_tags_mcsettagandzerodatapost");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 2 => {
                            println!("encoding: aarch64_integer_tags_mcsettagandzerodata");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 3 => {
                            println!("encoding: aarch64_integer_tags_mcsettagandzerodatapre");
                        }
                        (x0, _, x2) if x0 == 2 && x2 == 1 => {
                            println!("encoding: aarch64_integer_tags_mcsettagpairpost");
                        }
                        (x0, _, x2) if x0 == 2 && x2 == 2 => {
                            println!("encoding: aarch64_integer_tags_mcsettagpair");
                        }
                        (x0, _, x2) if x0 == 2 && x2 == 3 => {
                            println!("encoding: aarch64_integer_tags_mcsettagpairpre");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 != 0 && x2 == 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_integer_tags_mcsettagarray");
                        }
                        (x0, _, x2) if x0 == 3 && x2 == 1 => {
                            println!("encoding: aarch64_integer_tags_mcsettagpairandzerodatapost");
                        }
                        (x0, _, x2) if x0 == 3 && x2 == 2 => {
                            println!("encoding: aarch64_integer_tags_mcsettagpairandzerodata");
                        }
                        (x0, _, x2) if x0 == 3 && x2 == 3 => {
                            println!("encoding: aarch64_integer_tags_mcsettagpairandzerodatapre");
                        }
                        (x0, x1, x2) if x0 == 3 && x1 != 0 && x2 == 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_integer_tags_mcgettagarray");
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, _, _, _, _, _, _, _) if x0 & 11 == 8 && x2 == 1 => {
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
                            println!("unallocated");
                        }
                        (x0, x1, _, x3, _, x5) if x0 & 2 == 0 && x1 == 0 && x3 == 1 && x5 != 31 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_memory_exclusive_single");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 1 => {
                            println!("encoding: aarch64_memory_exclusive_single");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 31 => {
                            println!("encoding: aarch64_memory_atomicops_cas_pair");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 1 && x5 == 31 => {
                            println!("encoding: aarch64_memory_atomicops_cas_pair");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_memory_exclusive_single");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 => {
                            println!("encoding: aarch64_memory_exclusive_single");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 31 => {
                            println!("encoding: aarch64_memory_atomicops_cas_pair");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 31 => {
                            println!("encoding: aarch64_memory_atomicops_cas_pair");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_memory_ordered");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 0 && x4 == 1 => {
                            println!("encoding: aarch64_memory_ordered");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 31 => {
                            println!("encoding: aarch64_memory_atomicops_cas_single");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 1 && x4 == 1 && x5 == 31 => {
                            println!("encoding: aarch64_memory_atomicops_cas_single");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_memory_ordered");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 0 && x4 == 1 => {
                            println!("encoding: aarch64_memory_ordered");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 31 => {
                            println!("encoding: aarch64_memory_atomicops_cas_single");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 31 => {
                            println!("encoding: aarch64_memory_atomicops_cas_single");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_memory_exclusive_single");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 1 => {
                            println!("encoding: aarch64_memory_exclusive_single");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 31 => {
                            println!("encoding: aarch64_memory_atomicops_cas_pair");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 1 && x5 == 31 => {
                            println!("encoding: aarch64_memory_atomicops_cas_pair");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_memory_exclusive_single");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 => {
                            println!("encoding: aarch64_memory_exclusive_single");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 31 => {
                            println!("encoding: aarch64_memory_atomicops_cas_pair");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 31 => {
                            println!("encoding: aarch64_memory_atomicops_cas_pair");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_memory_ordered");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 0 && x4 == 1 => {
                            println!("encoding: aarch64_memory_ordered");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 31 => {
                            println!("encoding: aarch64_memory_atomicops_cas_single");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 1 && x4 == 1 && x5 == 31 => {
                            println!("encoding: aarch64_memory_atomicops_cas_single");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_memory_ordered");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 0 && x4 == 1 => {
                            println!("encoding: aarch64_memory_ordered");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 31 => {
                            println!("encoding: aarch64_memory_atomicops_cas_single");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 31 => {
                            println!("encoding: aarch64_memory_atomicops_cas_single");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_memory_exclusive_single");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 1 => {
                            println!("encoding: aarch64_memory_exclusive_single");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 => {
                            println!("encoding: aarch64_memory_exclusive_pair");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 1 => {
                            println!("encoding: aarch64_memory_exclusive_pair");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_memory_exclusive_single");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 => {
                            println!("encoding: aarch64_memory_exclusive_single");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 => {
                            println!("encoding: aarch64_memory_exclusive_pair");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 => {
                            println!("encoding: aarch64_memory_exclusive_pair");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 2 && x1 == 1 && x2 == 0 && x3 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_memory_ordered");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 2 && x1 == 1 && x2 == 0 && x3 == 0 && x4 == 1 => {
                            println!("encoding: aarch64_memory_ordered");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 1 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 31 => {
                            println!("encoding: aarch64_memory_atomicops_cas_single");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 1 && x2 == 0 && x3 == 1 && x4 == 1 && x5 == 31 => {
                            println!("encoding: aarch64_memory_atomicops_cas_single");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 2 && x1 == 1 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_memory_ordered");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 2 && x1 == 1 && x2 == 1 && x3 == 0 && x4 == 1 => {
                            println!("encoding: aarch64_memory_ordered");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 1 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 31 => {
                            println!("encoding: aarch64_memory_atomicops_cas_single");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 1 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 31 => {
                            println!("encoding: aarch64_memory_atomicops_cas_single");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_memory_exclusive_single");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 1 => {
                            println!("encoding: aarch64_memory_exclusive_single");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 => {
                            println!("encoding: aarch64_memory_exclusive_pair");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 1 => {
                            println!("encoding: aarch64_memory_exclusive_pair");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_memory_exclusive_single");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 => {
                            println!("encoding: aarch64_memory_exclusive_single");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 => {
                            println!("encoding: aarch64_memory_exclusive_pair");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 => {
                            println!("encoding: aarch64_memory_exclusive_pair");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 3 && x1 == 1 && x2 == 0 && x3 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_memory_ordered");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 3 && x1 == 1 && x2 == 0 && x3 == 0 && x4 == 1 => {
                            println!("encoding: aarch64_memory_ordered");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 1 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 31 => {
                            println!("encoding: aarch64_memory_atomicops_cas_single");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 1 && x2 == 0 && x3 == 1 && x4 == 1 && x5 == 31 => {
                            println!("encoding: aarch64_memory_atomicops_cas_single");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 3 && x1 == 1 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_memory_ordered");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 3 && x1 == 1 && x2 == 1 && x3 == 0 && x4 == 1 => {
                            println!("encoding: aarch64_memory_ordered");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 1 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 31 => {
                            println!("encoding: aarch64_memory_atomicops_cas_single");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 1 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 31 => {
                            println!("encoding: aarch64_memory_atomicops_cas_single");
                        }
                        _ => unreachable!()
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
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_lda_stl");
                        }
                        (x0, x1) if x0 == 0 && x1 == 1 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_lda_stl");
                        }
                        (x0, x1) if x0 == 0 && x1 == 2 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_lda_stl");
                        }
                        (x0, x1) if x0 == 0 && x1 == 3 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_lda_stl");
                        }
                        (x0, x1) if x0 == 1 && x1 == 0 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_lda_stl");
                        }
                        (x0, x1) if x0 == 1 && x1 == 1 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_lda_stl");
                        }
                        (x0, x1) if x0 == 1 && x1 == 2 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_lda_stl");
                        }
                        (x0, x1) if x0 == 1 && x1 == 3 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_lda_stl");
                        }
                        (x0, x1) if x0 == 2 && x1 == 0 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_lda_stl");
                        }
                        (x0, x1) if x0 == 2 && x1 == 1 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_lda_stl");
                        }
                        (x0, x1) if x0 == 2 && x1 == 2 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_lda_stl");
                        }
                        (x0, x1) if x0 == 2 && x1 == 3 => {
                            println!("unallocated");
                        }
                        (x0, x1) if x0 == 3 && x1 == 0 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_lda_stl");
                        }
                        (x0, x1) if x0 == 3 && x1 == 1 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_lda_stl");
                        }
                        (x0, x1) if x0 == 3 && x1 == 2 => {
                            println!("unallocated");
                        }
                        (x0, x1) if x0 == 3 && x1 == 3 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, _, _, x4, _, _, _, _, _) if x0 & 3 == 1 && x4 & 2 == 0 => {
                    let opc = (instr >> 30) & 3;
                    let V = (instr >> 26) & 1;
                    let imm19 = (instr >> 5) & 37;
                    let Rt = instr & 9;
                    match (opc, V) {
                        (x0, x1) if x0 == 0 && x1 == 0 => {
                            println!("encoding: aarch64_memory_literal_general");
                        }
                        (x0, x1) if x0 == 0 && x1 == 1 => {
                            println!("encoding: aarch64_memory_literal_simdfp");
                        }
                        (x0, x1) if x0 == 1 && x1 == 0 => {
                            println!("encoding: aarch64_memory_literal_general");
                        }
                        (x0, x1) if x0 == 1 && x1 == 1 => {
                            println!("encoding: aarch64_memory_literal_simdfp");
                        }
                        (x0, x1) if x0 == 2 && x1 == 0 => {
                            println!("encoding: aarch64_memory_literal_general");
                        }
                        (x0, x1) if x0 == 2 && x1 == 1 => {
                            println!("encoding: aarch64_memory_literal_simdfp");
                        }
                        (x0, x1) if x0 == 3 && x1 == 0 => {
                            println!("encoding: aarch64_memory_literal_general");
                        }
                        (x0, x1) if x0 == 3 && x1 == 1 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
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
                            println!("encoding: aarch64_memory_pair_general_no_alloc");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_memory_pair_general_no_alloc");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_memory_pair_simdfp_no_alloc");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_memory_pair_simdfp_no_alloc");
                        }
                        (x0, x1, _) if x0 == 1 && x1 == 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_memory_pair_simdfp_no_alloc");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_memory_pair_simdfp_no_alloc");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_memory_pair_general_no_alloc");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_memory_pair_general_no_alloc");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_memory_pair_simdfp_no_alloc");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_memory_pair_simdfp_no_alloc");
                        }
                        (x0, _, _) if x0 == 3 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
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
                            println!("encoding: aarch64_memory_pair_general_post_idx");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_memory_pair_general_post_idx");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_memory_pair_simdfp_post_idx");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_memory_pair_simdfp_post_idx");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_integer_tags_mcsettaganddatapairpost");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_memory_pair_general_post_idx");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_memory_pair_simdfp_post_idx");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_memory_pair_simdfp_post_idx");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_memory_pair_general_post_idx");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_memory_pair_general_post_idx");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_memory_pair_simdfp_post_idx");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_memory_pair_simdfp_post_idx");
                        }
                        (x0, _, _) if x0 == 3 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
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
                            println!("encoding: aarch64_memory_pair_general_offset");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_memory_pair_general_offset");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_memory_pair_simdfp_offset");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_memory_pair_simdfp_offset");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_integer_tags_mcsettaganddatapair");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_memory_pair_general_offset");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_memory_pair_simdfp_offset");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_memory_pair_simdfp_offset");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_memory_pair_general_offset");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_memory_pair_general_offset");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_memory_pair_simdfp_offset");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_memory_pair_simdfp_offset");
                        }
                        (x0, _, _) if x0 == 3 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
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
                            println!("encoding: aarch64_memory_pair_general_pre_idx");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_memory_pair_general_pre_idx");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_memory_pair_simdfp_pre_idx");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_memory_pair_simdfp_pre_idx");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_integer_tags_mcsettaganddatapairpre");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_memory_pair_general_pre_idx");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_memory_pair_simdfp_pre_idx");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_memory_pair_simdfp_pre_idx");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_memory_pair_general_pre_idx");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_memory_pair_general_pre_idx");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_memory_pair_simdfp_pre_idx");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_memory_pair_simdfp_pre_idx");
                        }
                        (x0, _, _) if x0 == 3 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_normal");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_normal");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 2 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_normal");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 3 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_normal");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_signed_offset_normal");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_signed_offset_normal");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 2 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_signed_offset_normal");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 3 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_signed_offset_normal");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_normal");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_normal");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 2 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_normal");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 3 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_normal");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_signed_offset_normal");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_signed_offset_normal");
                        }
                        (x0, x1, x2) if x0 & 2 == 2 && x1 == 0 && x2 == 3 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 & 2 == 2 && x1 == 1 && x2 & 2 == 2 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_normal");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_normal");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 2 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_normal");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_signed_offset_normal");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_signed_offset_normal");
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_normal");
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_normal");
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 2 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_normal");
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_signed_offset_normal");
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_signed_offset_normal");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_post_idx");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_post_idx");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 2 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_post_idx");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 3 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_post_idx");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_signed_post_idx");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_signed_post_idx");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 2 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_signed_post_idx");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 3 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_signed_post_idx");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_post_idx");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_post_idx");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 2 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_post_idx");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 3 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_post_idx");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_signed_post_idx");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_signed_post_idx");
                        }
                        (x0, x1, x2) if x0 & 2 == 2 && x1 == 0 && x2 == 3 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 & 2 == 2 && x1 == 1 && x2 & 2 == 2 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_post_idx");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_post_idx");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 2 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_post_idx");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_signed_post_idx");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_signed_post_idx");
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_post_idx");
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_post_idx");
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 2 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_signed_post_idx");
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_signed_post_idx");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_unpriv");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_unpriv");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 2 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_unpriv");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 3 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_unpriv");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_unpriv");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_unpriv");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 2 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_unpriv");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 3 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_unpriv");
                        }
                        (x0, x1, x2) if x0 & 2 == 2 && x1 == 0 && x2 == 3 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_unpriv");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_unpriv");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 2 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_unpriv");
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_unpriv");
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_offset_unpriv");
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 2 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_pre_idx");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_pre_idx");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 2 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_pre_idx");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 3 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_pre_idx");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_signed_pre_idx");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_signed_pre_idx");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 2 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_signed_pre_idx");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 3 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_signed_pre_idx");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_pre_idx");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_pre_idx");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 2 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_pre_idx");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 3 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_pre_idx");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_signed_pre_idx");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_signed_pre_idx");
                        }
                        (x0, x1, x2) if x0 & 2 == 2 && x1 == 0 && x2 == 3 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 & 2 == 2 && x1 == 1 && x2 & 2 == 2 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_pre_idx");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_pre_idx");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 2 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_pre_idx");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_signed_pre_idx");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_signed_pre_idx");
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_pre_idx");
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_pre_idx");
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 2 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_signed_pre_idx");
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_signed_pre_idx");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (_, x1, _, _, x4, x5) if x1 == 0 && x4 == 1 && x5 & 6 == 2 => {
                            println!("unallocated");
                        }
                        (_, x1, _, _, x4, x5) if x1 == 0 && x4 == 1 && x5 == 5 => {
                            println!("unallocated");
                        }
                        (_, x1, _, _, x4, x5) if x1 == 0 && x4 == 1 && x5 & 6 == 6 => {
                            println!("unallocated");
                        }
                        (_, x1, x2, _, x4, x5) if x1 == 0 && x2 == 0 && x4 == 1 && x5 == 4 => {
                            println!("unallocated");
                        }
                        (_, x1, x2, x3, x4, x5) if x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 4 => {
                            println!("unallocated");
                        }
                        (_, x1, _, _, _, _) if x1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 0 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 1 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 2 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 3 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 4 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 5 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 6 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 7 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 1 && x5 == 0 => {
                            println!("encoding: aarch64_memory_atomicops_swp");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 0 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 1 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 2 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 3 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 4 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 5 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 6 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 7 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 1 && x5 == 0 => {
                            println!("encoding: aarch64_memory_atomicops_swp");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 0 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 1 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 2 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 3 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 4 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 5 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 6 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 7 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 0 => {
                            println!("encoding: aarch64_memory_atomicops_swp");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 4 => {
                            println!("encoding: aarch64_memory_ordered_rcpc");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 0 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 1 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 2 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 3 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 4 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 5 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 6 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 7 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 0 => {
                            println!("encoding: aarch64_memory_atomicops_swp");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 0 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 1 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 2 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 3 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 4 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 5 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 6 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 7 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 1 && x5 == 0 => {
                            println!("encoding: aarch64_memory_atomicops_swp");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 0 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 1 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 2 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 3 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 4 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 5 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 6 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 7 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 1 && x5 == 0 => {
                            println!("encoding: aarch64_memory_atomicops_swp");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 0 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 1 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 2 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 3 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 4 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 5 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 6 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 7 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 0 => {
                            println!("encoding: aarch64_memory_atomicops_swp");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 4 => {
                            println!("encoding: aarch64_memory_ordered_rcpc");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 0 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 1 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 2 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 3 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 4 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 5 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 6 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 7 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 0 => {
                            println!("encoding: aarch64_memory_atomicops_swp");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 0 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 1 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 2 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 3 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 4 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 5 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 6 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 7 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 1 && x5 == 0 => {
                            println!("encoding: aarch64_memory_atomicops_swp");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 0 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 1 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 2 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 3 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 4 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 5 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 6 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 7 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 1 && x5 == 0 => {
                            println!("encoding: aarch64_memory_atomicops_swp");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 0 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 1 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 2 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 3 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 4 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 5 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 6 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 7 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 0 => {
                            println!("encoding: aarch64_memory_atomicops_swp");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 4 => {
                            println!("encoding: aarch64_memory_ordered_rcpc");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 0 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 1 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 2 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 3 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 4 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 5 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 6 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 7 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 0 => {
                            println!("encoding: aarch64_memory_atomicops_swp");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 0 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 1 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 2 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 3 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 4 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 5 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 6 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 7 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 1 && x5 == 0 => {
                            println!("encoding: aarch64_memory_atomicops_swp");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 0 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 1 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 2 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 3 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 4 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 5 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 6 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 7 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 1 && x5 == 0 => {
                            println!("encoding: aarch64_memory_atomicops_swp");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 0 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 1 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 2 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 3 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 4 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 5 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 6 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 7 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 0 => {
                            println!("encoding: aarch64_memory_atomicops_swp");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 4 => {
                            println!("encoding: aarch64_memory_ordered_rcpc");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 0 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 1 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 2 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 3 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 4 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 5 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 6 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 7 => {
                            println!("encoding: aarch64_memory_atomicops_ld");
                        }
                        (x0, x1, x2, x3, x4, x5) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 0 => {
                            println!("encoding: aarch64_memory_atomicops_swp");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (x0, x1, x2, _) if x0 & 1 == 1 && x1 == 1 && x2 & 2 == 2 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 != 3 => {
                            println!("encoding: aarch64_memory_single_general_register");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 3 => {
                            println!("encoding: aarch64_memory_single_general_register");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 != 3 => {
                            println!("encoding: aarch64_memory_single_general_register");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 3 => {
                            println!("encoding: aarch64_memory_single_general_register");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 2 && x3 != 3 => {
                            println!("encoding: aarch64_memory_single_general_register");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 2 && x3 == 3 => {
                            println!("encoding: aarch64_memory_single_general_register");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 != 3 => {
                            println!("encoding: aarch64_memory_single_general_register");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 3 => {
                            println!("encoding: aarch64_memory_single_general_register");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 != 3 => {
                            println!("encoding: aarch64_memory_single_simdfp_register");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 3 => {
                            println!("encoding: aarch64_memory_single_simdfp_register");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 != 3 => {
                            println!("encoding: aarch64_memory_single_simdfp_register");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 3 => {
                            println!("encoding: aarch64_memory_single_simdfp_register");
                        }
                        (x0, x1, x2, _) if x0 == 0 && x1 == 1 && x2 == 2 => {
                            println!("encoding: aarch64_memory_single_simdfp_register");
                        }
                        (x0, x1, x2, _) if x0 == 0 && x1 == 1 && x2 == 3 => {
                            println!("encoding: aarch64_memory_single_simdfp_register");
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_general_register");
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_general_register");
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 0 && x2 == 2 => {
                            println!("encoding: aarch64_memory_single_general_register");
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 0 && x2 == 3 => {
                            println!("encoding: aarch64_memory_single_general_register");
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_simdfp_register");
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_simdfp_register");
                        }
                        (x0, x1, x2, _) if x0 & 2 == 2 && x1 == 0 && x2 == 3 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _) if x0 & 2 == 2 && x1 == 1 && x2 & 2 == 2 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _) if x0 == 2 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_general_register");
                        }
                        (x0, x1, x2, _) if x0 == 2 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_general_register");
                        }
                        (x0, x1, x2, _) if x0 == 2 && x1 == 0 && x2 == 2 => {
                            println!("encoding: aarch64_memory_single_general_register");
                        }
                        (x0, x1, x2, _) if x0 == 2 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_simdfp_register");
                        }
                        (x0, x1, x2, _) if x0 == 2 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_simdfp_register");
                        }
                        (x0, x1, x2, _) if x0 == 3 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_general_register");
                        }
                        (x0, x1, x2, _) if x0 == 3 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_general_register");
                        }
                        (x0, x1, x2, _) if x0 == 3 && x1 == 0 && x2 == 2 => {
                            println!("encoding: aarch64_memory_single_general_register");
                        }
                        (x0, x1, x2, _) if x0 == 3 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_simdfp_register");
                        }
                        (x0, x1, x2, _) if x0 == 3 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_simdfp_register");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_pac");
                        }
                        (x0, x1, x2, x3) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_pac");
                        }
                        (x0, x1, x2, x3) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_pac");
                        }
                        (x0, x1, x2, x3) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 => {
                            println!("encoding: aarch64_memory_single_general_immediate_signed_pac");
                        }
                        (x0, x1, _, _) if x0 == 3 && x1 == 1 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_general_immediate_unsigned");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_general_immediate_unsigned");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 2 => {
                            println!("encoding: aarch64_memory_single_general_immediate_unsigned");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 3 => {
                            println!("encoding: aarch64_memory_single_general_immediate_unsigned");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_unsigned");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_unsigned");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 2 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_unsigned");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 3 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_unsigned");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_general_immediate_unsigned");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_general_immediate_unsigned");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 2 => {
                            println!("encoding: aarch64_memory_single_general_immediate_unsigned");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 3 => {
                            println!("encoding: aarch64_memory_single_general_immediate_unsigned");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_unsigned");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_unsigned");
                        }
                        (x0, x1, x2) if x0 & 2 == 2 && x1 == 0 && x2 == 3 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 & 2 == 2 && x1 == 1 && x2 & 2 == 2 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_general_immediate_unsigned");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_general_immediate_unsigned");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 2 => {
                            println!("encoding: aarch64_memory_single_general_immediate_unsigned");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_unsigned");
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_unsigned");
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_general_immediate_unsigned");
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_general_immediate_unsigned");
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 2 => {
                            println!("encoding: aarch64_memory_single_general_immediate_unsigned");
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_unsigned");
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_memory_single_simdfp_immediate_unsigned");
                        }
                        _ => unreachable!()
                    }
                }
                _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 & 56 == 24 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 & 32 == 32 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 == 0 && x2 & 62 == 6 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 == 0 && x2 == 13 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 == 0 && x2 & 62 == 14 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 == 1 && x2 & 62 == 2 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 == 1 && x2 & 60 == 4 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 == 1 && x2 & 56 == 8 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 == 1 && x2 & 48 == 16 => {
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 2 => {
                            println!("encoding: aarch64_integer_arithmetic_div");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 3 => {
                            println!("encoding: aarch64_integer_arithmetic_div");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 & 62 == 4 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 8 => {
                            println!("encoding: aarch64_integer_shift_variable");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 9 => {
                            println!("encoding: aarch64_integer_shift_variable");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 10 => {
                            println!("encoding: aarch64_integer_shift_variable");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 11 => {
                            println!("encoding: aarch64_integer_shift_variable");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 12 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 & 59 == 19 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 16 => {
                            println!("encoding: aarch64_integer_crc");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 17 => {
                            println!("encoding: aarch64_integer_crc");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 18 => {
                            println!("encoding: aarch64_integer_crc");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 20 => {
                            println!("encoding: aarch64_integer_crc");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 21 => {
                            println!("encoding: aarch64_integer_crc");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 22 => {
                            println!("encoding: aarch64_integer_crc");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 2 => {
                            println!("encoding: aarch64_integer_arithmetic_div");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 3 => {
                            println!("encoding: aarch64_integer_arithmetic_div");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 4 => {
                            println!("encoding: aarch64_integer_tags_mcinsertrandomtag");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 5 => {
                            println!("encoding: aarch64_integer_tags_mcinserttagmask");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 8 => {
                            println!("encoding: aarch64_integer_shift_variable");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 9 => {
                            println!("encoding: aarch64_integer_shift_variable");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 10 => {
                            println!("encoding: aarch64_integer_shift_variable");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 11 => {
                            println!("encoding: aarch64_integer_shift_variable");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 12 => {
                            println!("encoding: aarch64_integer_pac_pacga_dp_2src");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 & 57 == 16 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 & 58 == 16 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 19 => {
                            println!("encoding: aarch64_integer_crc");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 23 => {
                            println!("encoding: aarch64_integer_crc");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (_, _, x2, _, _) if x2 & 2 == 2 => {
                            println!("unallocated");
                        }
                        (_, _, x2, _, _) if x2 & 4 == 4 => {
                            println!("unallocated");
                        }
                        (_, _, x2, _, _) if x2 & 8 == 8 => {
                            println!("unallocated");
                        }
                        (_, _, x2, _, _) if x2 & 16 == 16 => {
                            println!("unallocated");
                        }
                        (_, x1, x2, x3, _) if x1 == 0 && x2 == 0 && x3 & 62 == 6 => {
                            println!("unallocated");
                        }
                        (_, x1, x2, x3, _) if x1 == 0 && x2 == 0 && x3 & 56 == 8 => {
                            println!("unallocated");
                        }
                        (_, x1, x2, x3, _) if x1 == 0 && x2 == 0 && x3 & 48 == 16 => {
                            println!("unallocated");
                        }
                        (_, x1, _, _, _) if x1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, _, x2, _, _) if x0 == 0 && x2 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            println!("encoding: aarch64_integer_arithmetic_rbit");
                        }
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            println!("encoding: aarch64_integer_arithmetic_rev");
                        }
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 2 => {
                            println!("encoding: aarch64_integer_arithmetic_rev");
                        }
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 3 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 4 => {
                            println!("encoding: aarch64_integer_arithmetic_cnt");
                        }
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 5 => {
                            println!("encoding: aarch64_integer_arithmetic_cnt");
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            println!("encoding: aarch64_integer_arithmetic_rbit");
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            println!("encoding: aarch64_integer_arithmetic_rev");
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 2 => {
                            println!("encoding: aarch64_integer_arithmetic_rev");
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 3 => {
                            println!("encoding: aarch64_integer_arithmetic_rev");
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 4 => {
                            println!("encoding: aarch64_integer_arithmetic_cnt");
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 5 => {
                            println!("encoding: aarch64_integer_arithmetic_cnt");
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            println!("encoding: aarch64_integer_pac_pacia_dp_1src");
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 => {
                            println!("encoding: aarch64_integer_pac_pacib_dp_1src");
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 2 => {
                            println!("encoding: aarch64_integer_pac_pacda_dp_1src");
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 3 => {
                            println!("encoding: aarch64_integer_pac_pacdb_dp_1src");
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 4 => {
                            println!("encoding: aarch64_integer_pac_autia_dp_1src");
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 5 => {
                            println!("encoding: aarch64_integer_pac_autib_dp_1src");
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 6 => {
                            println!("encoding: aarch64_integer_pac_autda_dp_1src");
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 7 => {
                            println!("encoding: aarch64_integer_pac_autdb_dp_1src");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 8 && x4 == 31 => {
                            println!("encoding: aarch64_integer_pac_pacia_dp_1src");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 9 && x4 == 31 => {
                            println!("encoding: aarch64_integer_pac_pacib_dp_1src");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 10 && x4 == 31 => {
                            println!("encoding: aarch64_integer_pac_pacda_dp_1src");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 11 && x4 == 31 => {
                            println!("encoding: aarch64_integer_pac_pacdb_dp_1src");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 12 && x4 == 31 => {
                            println!("encoding: aarch64_integer_pac_autia_dp_1src");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 13 && x4 == 31 => {
                            println!("encoding: aarch64_integer_pac_autib_dp_1src");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 14 && x4 == 31 => {
                            println!("encoding: aarch64_integer_pac_autda_dp_1src");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 15 && x4 == 31 => {
                            println!("encoding: aarch64_integer_pac_autdb_dp_1src");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 16 && x4 == 31 => {
                            println!("encoding: aarch64_integer_pac_strip_dp_1src");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 17 && x4 == 31 => {
                            println!("encoding: aarch64_integer_pac_strip_dp_1src");
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 & 62 == 18 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 & 60 == 20 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 & 56 == 24 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (x0, x1, x2, _) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_integer_logical_shiftedreg");
                        }
                        (x0, x1, x2, _) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_integer_logical_shiftedreg");
                        }
                        (x0, x1, x2, _) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_integer_logical_shiftedreg");
                        }
                        (x0, x1, x2, _) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_integer_logical_shiftedreg");
                        }
                        (x0, x1, x2, _) if x0 == 0 && x1 == 2 && x2 == 0 => {
                            println!("encoding: aarch64_integer_logical_shiftedreg");
                        }
                        (x0, x1, x2, _) if x0 == 0 && x1 == 2 && x2 == 1 => {
                            println!("encoding: aarch64_integer_logical_shiftedreg");
                        }
                        (x0, x1, x2, _) if x0 == 0 && x1 == 3 && x2 == 0 => {
                            println!("encoding: aarch64_integer_logical_shiftedreg");
                        }
                        (x0, x1, x2, _) if x0 == 0 && x1 == 3 && x2 == 1 => {
                            println!("encoding: aarch64_integer_logical_shiftedreg");
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_integer_logical_shiftedreg");
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_integer_logical_shiftedreg");
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_integer_logical_shiftedreg");
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_integer_logical_shiftedreg");
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 2 && x2 == 0 => {
                            println!("encoding: aarch64_integer_logical_shiftedreg");
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 2 && x2 == 1 => {
                            println!("encoding: aarch64_integer_logical_shiftedreg");
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 3 && x2 == 0 => {
                            println!("encoding: aarch64_integer_logical_shiftedreg");
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 3 && x2 == 1 => {
                            println!("encoding: aarch64_integer_logical_shiftedreg");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (x0, _, _, _, x4) if x0 == 0 && x4 & 32 == 32 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_integer_arithmetic_add_sub_shiftedreg");
                        }
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_integer_arithmetic_add_sub_shiftedreg");
                        }
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_integer_arithmetic_add_sub_shiftedreg");
                        }
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_integer_arithmetic_add_sub_shiftedreg");
                        }
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_integer_arithmetic_add_sub_shiftedreg");
                        }
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_integer_arithmetic_add_sub_shiftedreg");
                        }
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_integer_arithmetic_add_sub_shiftedreg");
                        }
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_integer_arithmetic_add_sub_shiftedreg");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (_, _, _, _, x4) if x4 & 6 == 6 => {
                            println!("unallocated");
                        }
                        (_, _, _, x3, _) if x3 & 1 == 1 => {
                            println!("unallocated");
                        }
                        (_, _, _, x3, _) if x3 & 2 == 2 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            println!("encoding: aarch64_integer_arithmetic_add_sub_extendedreg");
                        }
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            println!("encoding: aarch64_integer_arithmetic_add_sub_extendedreg");
                        }
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 0 => {
                            println!("encoding: aarch64_integer_arithmetic_add_sub_extendedreg");
                        }
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 0 => {
                            println!("encoding: aarch64_integer_arithmetic_add_sub_extendedreg");
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            println!("encoding: aarch64_integer_arithmetic_add_sub_extendedreg");
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            println!("encoding: aarch64_integer_arithmetic_add_sub_extendedreg");
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 0 => {
                            println!("encoding: aarch64_integer_arithmetic_add_sub_extendedreg");
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 0 => {
                            println!("encoding: aarch64_integer_arithmetic_add_sub_extendedreg");
                        }
                        _ => unreachable!()
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
                            println!("encoding: aarch64_integer_arithmetic_add_sub_carry");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_integer_arithmetic_add_sub_carry");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_integer_arithmetic_add_sub_carry");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_integer_arithmetic_add_sub_carry");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_integer_arithmetic_add_sub_carry");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_integer_arithmetic_add_sub_carry");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_integer_arithmetic_add_sub_carry");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_integer_arithmetic_add_sub_carry");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            println!("encoding: aarch64_integer_flags_rmif");
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, _) if x0 == 1 && x1 == 1 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, _, _, _) if x0 == 0 && x1 == 0 && x2 == 1 && x3 != 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, _, x5, x6) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x5 == 0 && x6 != 13 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, _, x5, _) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x5 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4, x5, x6) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 0 && x6 == 13 => {
                            println!("encoding: aarch64_integer_flags_setf");
                        }
                        (x0, x1, x2, x3, x4, x5, x6) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 0 && x6 == 13 => {
                            println!("encoding: aarch64_integer_flags_setf");
                        }
                        (x0, x1, _, _, _, _, _) if x0 == 0 && x1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, _, _, _, _, _, _) if x0 == 1 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (_, _, _, x3, _) if x3 == 1 => {
                            println!("unallocated");
                        }
                        (_, _, x2, _, _) if x2 == 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_integer_conditional_compare_register");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_integer_conditional_compare_register");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_integer_conditional_compare_register");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_integer_conditional_compare_register");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (_, _, _, x3, _) if x3 == 1 => {
                            println!("unallocated");
                        }
                        (_, _, x2, _, _) if x2 == 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_integer_conditional_compare_immediate");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_integer_conditional_compare_immediate");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_integer_conditional_compare_immediate");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_integer_conditional_compare_immediate");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (_, _, x2, _) if x2 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            println!("encoding: aarch64_integer_conditional_select");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            println!("encoding: aarch64_integer_conditional_select");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 0 => {
                            println!("encoding: aarch64_integer_conditional_select");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 1 => {
                            println!("encoding: aarch64_integer_conditional_select");
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            println!("encoding: aarch64_integer_conditional_select");
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            println!("encoding: aarch64_integer_conditional_select");
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 0 => {
                            println!("encoding: aarch64_integer_conditional_select");
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 1 => {
                            println!("encoding: aarch64_integer_conditional_select");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (_, x1, x2, _) if x1 == 0 && x2 == 3 => {
                            println!("unallocated");
                        }
                        (_, x1, x2, _) if x1 == 0 && x2 == 4 => {
                            println!("unallocated");
                        }
                        (_, x1, x2, x3) if x1 == 0 && x2 == 6 && x3 == 1 => {
                            println!("unallocated");
                        }
                        (_, x1, x2, _) if x1 == 0 && x2 == 7 => {
                            println!("unallocated");
                        }
                        (_, x1, _, _) if x1 == 1 => {
                            println!("unallocated");
                        }
                        (_, x1, _, _) if x1 & 2 == 2 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            println!("encoding: aarch64_integer_arithmetic_mul_uniform_add_sub");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            println!("encoding: aarch64_integer_arithmetic_mul_uniform_add_sub");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 2 && x3 == 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 5 && x3 == 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 5 && x3 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 6 && x3 == 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            println!("encoding: aarch64_integer_arithmetic_mul_uniform_add_sub");
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            println!("encoding: aarch64_integer_arithmetic_mul_uniform_add_sub");
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            println!("encoding: aarch64_integer_arithmetic_mul_widening_32_64");
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 => {
                            println!("encoding: aarch64_integer_arithmetic_mul_widening_32_64");
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 2 && x3 == 0 => {
                            println!("encoding: aarch64_integer_arithmetic_mul_widening_64_128hi");
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 5 && x3 == 0 => {
                            println!("encoding: aarch64_integer_arithmetic_mul_widening_32_64");
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 5 && x3 == 1 => {
                            println!("encoding: aarch64_integer_arithmetic_mul_widening_32_64");
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 6 && x3 == 0 => {
                            println!("encoding: aarch64_integer_arithmetic_mul_widening_64_128hi");
                        }
                        _ => unreachable!()
                    }
                }
                _ => unreachable!()
            }
        }
        (_, x1, _) if x1 & 14 == 14 => {
            match ((instr >> 28) & 7, (instr >> 25) & 5, (instr >> 23) & 3, (instr >> 19) & 7, (instr >> 10) & 17, instr & 19) {
                (x0, _, x2, x3, x4, _) if x0 == 0 && x2 & 2 == 0 && x3 & 7 == 5 && x4 & 387 == 2 => {
                }
                (x0, _, x2, x3, x4, _) if x0 == 2 && x2 & 2 == 0 && x3 & 7 == 5 && x4 & 387 == 2 => {
                }
                (x0, _, x2, x3, x4, _) if x0 == 4 && x2 & 2 == 0 && x3 & 7 == 5 && x4 & 387 == 2 => {
                    let size = (instr >> 22) & 3;
                    let opcode = (instr >> 12) & 9;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (size, opcode) {
                        (_, x1) if x1 & 8 == 8 => {
                            println!("unallocated");
                        }
                        (_, x1) if x1 & 28 == 0 => {
                            println!("unallocated");
                        }
                        (_, x1) if x1 & 16 == 16 => {
                            println!("unallocated");
                        }
                        (x0, _) if x0 & 1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1) if x0 == 0 && x1 == 4 => {
                            println!("encoding: aarch64_vector_crypto_aes_round");
                        }
                        (x0, x1) if x0 == 0 && x1 == 5 => {
                            println!("encoding: aarch64_vector_crypto_aes_round");
                        }
                        (x0, x1) if x0 == 0 && x1 == 6 => {
                            println!("encoding: aarch64_vector_crypto_aes_mix");
                        }
                        (x0, x1) if x0 == 0 && x1 == 7 => {
                            println!("encoding: aarch64_vector_crypto_aes_mix");
                        }
                        (x0, _) if x0 & 2 == 2 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (x0, _) if x0 & 1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1) if x0 == 0 && x1 == 0 => {
                            println!("encoding: aarch64_vector_crypto_sha3op_sha1_hash_choose");
                        }
                        (x0, x1) if x0 == 0 && x1 == 1 => {
                            println!("encoding: aarch64_vector_crypto_sha3op_sha1_hash_parity");
                        }
                        (x0, x1) if x0 == 0 && x1 == 2 => {
                            println!("encoding: aarch64_vector_crypto_sha3op_sha1_hash_majority");
                        }
                        (x0, x1) if x0 == 0 && x1 == 3 => {
                            println!("encoding: aarch64_vector_crypto_sha3op_sha1_sched0");
                        }
                        (x0, x1) if x0 == 0 && x1 == 4 => {
                            println!("encoding: aarch64_vector_crypto_sha3op_sha256_hash");
                        }
                        (x0, x1) if x0 == 0 && x1 == 5 => {
                            println!("encoding: aarch64_vector_crypto_sha3op_sha256_hash");
                        }
                        (x0, x1) if x0 == 0 && x1 == 6 => {
                            println!("encoding: aarch64_vector_crypto_sha3op_sha256_sched1");
                        }
                        (x0, _) if x0 & 2 == 2 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 == 5 && x2 & 2 == 0 && x3 & 4 == 0 && x4 & 35 == 2 => {
                }
                (x0, _, x2, x3, x4, _) if x0 == 5 && x2 & 2 == 0 && x3 & 7 == 5 && x4 & 387 == 2 => {
                    let size = (instr >> 22) & 3;
                    let opcode = (instr >> 12) & 9;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (size, opcode) {
                        (_, x1) if x1 & 4 == 4 => {
                            println!("unallocated");
                        }
                        (_, x1) if x1 & 8 == 8 => {
                            println!("unallocated");
                        }
                        (_, x1) if x1 & 16 == 16 => {
                            println!("unallocated");
                        }
                        (x0, _) if x0 & 1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1) if x0 == 0 && x1 == 0 => {
                            println!("encoding: aarch64_vector_crypto_sha2op_sha1_hash");
                        }
                        (x0, x1) if x0 == 0 && x1 == 1 => {
                            println!("encoding: aarch64_vector_crypto_sha2op_sha1_sched1");
                        }
                        (x0, x1) if x0 == 0 && x1 == 2 => {
                            println!("encoding: aarch64_vector_crypto_sha2op_sha256_sched0");
                        }
                        (x0, x1) if x0 == 0 && x1 == 3 => {
                            println!("unallocated");
                        }
                        (x0, _) if x0 & 2 == 2 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 == 6 && x2 & 2 == 0 && x3 & 7 == 5 && x4 & 387 == 2 => {
                }
                (x0, _, x2, x3, x4, _) if x0 == 7 && x2 & 2 == 0 && x3 & 4 == 0 && x4 & 33 == 0 => {
                }
                (x0, _, x2, x3, x4, _) if x0 == 7 && x2 & 2 == 0 && x3 & 7 == 5 && x4 & 387 == 2 => {
                }
                (x0, _, x2, x3, x4, _) if x0 & 13 == 5 && x2 == 0 && x3 & 12 == 0 && x4 & 33 == 1 => {
                    let op = (instr >> 29) & 1;
                    let imm5 = (instr >> 16) & 9;
                    let imm4 = (instr >> 11) & 7;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (op, imm5, imm4) {
                        (x0, _, x2) if x0 == 0 && x2 & 1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 0 && x2 & 2 == 2 => {
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 0 && x2 & 4 == 4 => {
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_vector_transfer_vector_cpy_dup_sisd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 & 8 == 8 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 15 == 0 && x2 == 0 => {
                            println!("unallocated");
                        }
                        (x0, _, _) if x0 == 1 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 & 13 == 5 && x2 == 1 && x3 & 12 == 0 && x4 & 33 == 1 => {
                }
                (x0, _, x2, x3, x4, _) if x0 & 13 == 5 && x2 & 2 == 0 && x3 == 7 && x4 & 387 == 2 => {
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
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 == 1 && x2 == 3 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 3 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_mul_fp16_extended_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 4 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 5 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 7 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_recps_fp16_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 4 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 5 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 7 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_rsqrts_fp16_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 3 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 4 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 5 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 7 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 2 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 4 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 5 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 7 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 & 13 == 5 && x2 & 2 == 0 && x3 & 12 == 8 && x4 & 49 == 17 => {
                }
                (x0, _, x2, x3, x4, _) if x0 & 13 == 5 && x2 & 2 == 0 && x3 == 15 && x4 & 387 == 2 => {
                    let U = (instr >> 29) & 1;
                    let a = (instr >> 23) & 1;
                    let opcode = (instr >> 12) & 9;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (U, a, opcode) {
                        (_, _, x2) if x2 & 24 == 0 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 & 28 == 8 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 & 24 == 16 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 & 30 == 24 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 == 30 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 == 0 && x2 & 28 == 12 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 == 0 && x2 == 31 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 == 1 && x2 == 15 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 == 1 && x2 == 28 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 26 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 27 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 28 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_fp16_conv_float_tieaway_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 29 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_fp16_conv_int_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 12 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 13 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 14 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 26 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 27 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 29 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_special_recip_fp16_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 31 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_special_frecpx_fp16");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 26 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 27 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 28 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_fp16_conv_float_tieaway_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 29 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_fp16_conv_int_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 12 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 13 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 14 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 26 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 27 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 29 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_special_sqrt_est_fp16_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 31 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 & 13 == 5 && x2 & 2 == 0 && x3 & 4 == 0 && x4 & 33 == 32 => {
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
                            println!("unallocated");
                        }
                        (_, x1) if x1 & 12 == 4 => {
                            println!("unallocated");
                        }
                        (_, x1) if x1 & 8 == 8 => {
                            println!("unallocated");
                        }
                        (x0, x1) if x0 == 0 && x1 == 0 => {
                            println!("unallocated");
                        }
                        (x0, x1) if x0 == 0 && x1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1) if x0 == 1 && x1 == 0 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd");
                        }
                        (x0, x1) if x0 == 1 && x1 == 1 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 == 2 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 & 30 == 4 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 == 6 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 == 15 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 & 30 == 16 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 == 19 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 == 21 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 == 23 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 & 30 == 24 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 == 30 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 & 2 == 0 && x2 & 28 == 12 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 & 2 == 0 && x2 == 31 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 & 2 == 2 && x2 == 22 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 & 2 == 2 && x2 == 28 => {
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 3 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_add_saturating_sisd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 7 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_diff_neg_sat_sisd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 8 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 9 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 10 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 11 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_diff_neg_int_sisd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 18 => {
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 20 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_extract_sat_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 22 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 26 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_conv_float_bulk_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 27 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_conv_float_bulk_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 28 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_conv_float_tieaway_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 29 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_conv_int_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 12 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 13 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 14 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 26 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_conv_float_bulk_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 27 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_conv_float_bulk_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 29 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_special_recip_float_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 31 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_special_frecpx");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 3 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_add_saturating_sisd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 7 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_diff_neg_sat_sisd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 8 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 9 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 10 => {
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 11 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_diff_neg_int_sisd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 18 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_extract_sqxtun_sisd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 20 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_extract_sat_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 22 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_xtn_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 26 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_conv_float_bulk_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 27 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_conv_float_bulk_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 28 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_conv_float_tieaway_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 29 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_conv_int_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 12 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 13 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 14 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 26 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_conv_float_bulk_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 27 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_conv_float_bulk_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 29 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_special_sqrt_est_float_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 31 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 & 28 == 8 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 == 14 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 & 24 == 16 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 & 30 == 24 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 == 26 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 & 28 == 28 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 & 2 == 2 && x2 == 13 => {
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 27 => {
                            println!("encoding: aarch64_vector_reduce_add_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 12 => {
                            println!("encoding: aarch64_vector_reduce_fp16_maxnm_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 13 => {
                            println!("encoding: aarch64_vector_reduce_fp16_add_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 15 => {
                            println!("encoding: aarch64_vector_reduce_fp16_max_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 12 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 13 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 15 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 12 => {
                            println!("encoding: aarch64_vector_reduce_fp16_maxnm_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 15 => {
                            println!("encoding: aarch64_vector_reduce_fp16_max_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 12 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 15 => {
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 27 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 12 => {
                            println!("encoding: aarch64_vector_reduce_fp_maxnm_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 13 => {
                            println!("encoding: aarch64_vector_reduce_fp_add_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 15 => {
                            println!("encoding: aarch64_vector_reduce_fp_max_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 12 => {
                            println!("encoding: aarch64_vector_reduce_fp_maxnm_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 15 => {
                            println!("encoding: aarch64_vector_reduce_fp_max_sisd");
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 & 13 == 5 && x2 & 2 == 0 && x3 & 4 == 4 && x4 & 259 == 258 => {
                }
                (x0, _, x2, x3, x4, _) if x0 & 13 == 5 && x2 & 2 == 0 && x3 & 4 == 4 && x4 & 131 == 130 => {
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
                            println!("unallocated");
                        }
                        (_, x1) if x1 & 12 == 4 => {
                            println!("unallocated");
                        }
                        (_, x1) if x1 == 8 => {
                            println!("unallocated");
                        }
                        (_, x1) if x1 == 10 => {
                            println!("unallocated");
                        }
                        (_, x1) if x1 == 12 => {
                            println!("unallocated");
                        }
                        (_, x1) if x1 & 14 == 14 => {
                            println!("unallocated");
                        }
                        (x0, x1) if x0 == 0 && x1 == 9 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_disparate_mul_dmacc_sisd");
                        }
                        (x0, x1) if x0 == 0 && x1 == 11 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_disparate_mul_dmacc_sisd");
                        }
                        (x0, x1) if x0 == 0 && x1 == 13 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_disparate_mul_double_sisd");
                        }
                        (x0, x1) if x0 == 1 && x1 == 9 => {
                            println!("unallocated");
                        }
                        (x0, x1) if x0 == 1 && x1 == 11 => {
                            println!("unallocated");
                        }
                        (x0, x1) if x0 == 1 && x1 == 13 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 & 30 == 2 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 == 4 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 & 28 == 12 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 & 30 == 18 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 & 2 == 2 && x2 == 27 => {
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 5 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 6 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 7 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 8 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_shift_sisd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 9 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_shift_sisd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 10 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_shift_sisd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 11 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_shift_sisd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 16 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 17 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 20 => {
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 21 => {
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 22 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_sisd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 23 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 24 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 25 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 26 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 27 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_mul_fp_extended_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 28 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 29 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 30 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 31 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_recps_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 24 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 25 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 26 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 28 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 29 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 30 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 31 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_rsqrts_sisd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 5 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 6 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 7 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 8 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_shift_sisd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 9 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_shift_sisd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 10 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_shift_sisd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 11 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_shift_sisd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 16 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 17 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 20 => {
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 21 => {
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 22 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_sisd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 23 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 24 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 25 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 26 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 27 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 28 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 29 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 30 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 31 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 24 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 25 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 26 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 28 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 29 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 30 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 31 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 != 0 && x2 == 3 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 != 0 && x2 == 5 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 != 0 && x2 == 7 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 != 0 && x2 == 9 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 != 0 && x2 == 11 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 != 0 && x2 == 13 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 != 0 && x2 == 15 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 != 0 && x2 & 28 == 20 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 != 0 && x2 & 28 == 24 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 != 0 && x2 == 29 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 != 0 && x2 == 30 => {
                            println!("unallocated");
                        }
                        (_, x1, _) if x1 == 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 0 => {
                            println!("encoding: aarch64_vector_shift_right_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 2 => {
                            println!("encoding: aarch64_vector_shift_right_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 4 => {
                            println!("encoding: aarch64_vector_shift_right_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 6 => {
                            println!("encoding: aarch64_vector_shift_right_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 8 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 10 => {
                            println!("encoding: aarch64_vector_shift_left_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 12 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 14 => {
                            println!("encoding: aarch64_vector_shift_left_sat_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 16 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 17 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 18 => {
                            println!("encoding: aarch64_vector_shift_right_narrow_uniform_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 19 => {
                            println!("encoding: aarch64_vector_shift_right_narrow_uniform_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 28 => {
                            println!("encoding: aarch64_vector_shift_conv_int_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 31 => {
                            println!("encoding: aarch64_vector_shift_conv_float_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 0 => {
                            println!("encoding: aarch64_vector_shift_right_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 2 => {
                            println!("encoding: aarch64_vector_shift_right_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 4 => {
                            println!("encoding: aarch64_vector_shift_right_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 6 => {
                            println!("encoding: aarch64_vector_shift_right_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 8 => {
                            println!("encoding: aarch64_vector_shift_right_insert_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 10 => {
                            println!("encoding: aarch64_vector_shift_left_insert_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 12 => {
                            println!("encoding: aarch64_vector_shift_left_sat_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 14 => {
                            println!("encoding: aarch64_vector_shift_left_sat_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 16 => {
                            println!("encoding: aarch64_vector_shift_right_narrow_nonuniform_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 17 => {
                            println!("encoding: aarch64_vector_shift_right_narrow_nonuniform_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 18 => {
                            println!("encoding: aarch64_vector_shift_right_narrow_uniform_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 19 => {
                            println!("encoding: aarch64_vector_shift_right_narrow_uniform_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 28 => {
                            println!("encoding: aarch64_vector_shift_conv_int_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 31 => {
                            println!("encoding: aarch64_vector_shift_conv_float_sisd");
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, _, x4, _) if x0 & 13 == 5 && x2 == 3 && x4 & 1 == 1 => {
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
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 == 2 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 == 4 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 == 6 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 == 8 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 == 10 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 == 14 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 == 1 && x2 == 1 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 == 1 && x2 == 5 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 == 1 && x2 == 9 => {
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 3 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_acc_double_sisd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 7 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_acc_double_sisd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 11 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_double_sisd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 12 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_high_sisd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 13 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_high_sisd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 15 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_acc_fp16_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 5 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_acc_fp16_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 9 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_fp16_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 1 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_acc_fp_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 5 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_acc_fp_sisd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 9 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_fp_sisd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 3 => {
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 7 => {
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 11 => {
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 12 => {
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 13 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_acc_high_sisd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 15 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_acc_high_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 5 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 9 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_fp16_sisd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 5 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 9 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_fp_sisd");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_vector_transfer_vector_table");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_vector_transfer_vector_table");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_vector_transfer_vector_table");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_vector_transfer_vector_table");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 0 => {
                            println!("encoding: aarch64_vector_transfer_vector_table");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 1 => {
                            println!("encoding: aarch64_vector_transfer_vector_table");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 0 => {
                            println!("encoding: aarch64_vector_transfer_vector_table");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 1 => {
                            println!("encoding: aarch64_vector_transfer_vector_table");
                        }
                        (x0, _, _) if x0 & 2 == 2 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        x0 if x0 == 1 => {
                            println!("encoding: aarch64_vector_transfer_vector_permute_unzip");
                        }
                        x0 if x0 == 2 => {
                            println!("encoding: aarch64_vector_transfer_vector_permute_transpose");
                        }
                        x0 if x0 == 3 => {
                            println!("encoding: aarch64_vector_transfer_vector_permute_zip");
                        }
                        x0 if x0 == 4 => {
                            println!("unallocated");
                        }
                        x0 if x0 == 5 => {
                            println!("encoding: aarch64_vector_transfer_vector_permute_unzip");
                        }
                        x0 if x0 == 6 => {
                            println!("encoding: aarch64_vector_transfer_vector_permute_transpose");
                        }
                        x0 if x0 == 7 => {
                            println!("encoding: aarch64_vector_transfer_vector_permute_zip");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        x0 if x0 == 0 => {
                            println!("encoding: aarch64_vector_transfer_vector_extract");
                        }
                        x0 if x0 & 2 == 2 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (_, x1, _, x3) if x1 == 0 && x3 == 0 => {
                            println!("encoding: aarch64_vector_transfer_vector_cpy_dup_simd");
                        }
                        (_, x1, _, x3) if x1 == 0 && x3 == 1 => {
                            println!("encoding: aarch64_vector_transfer_integer_dup");
                        }
                        (_, x1, _, x3) if x1 == 0 && x3 == 2 => {
                            println!("unallocated");
                        }
                        (_, x1, _, x3) if x1 == 0 && x3 == 4 => {
                            println!("unallocated");
                        }
                        (_, x1, _, x3) if x1 == 0 && x3 == 6 => {
                            println!("unallocated");
                        }
                        (_, x1, _, x3) if x1 == 0 && x3 & 8 == 8 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, x3) if x0 == 0 && x1 == 0 && x3 == 3 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, x3) if x0 == 0 && x1 == 0 && x3 == 5 => {
                            println!("encoding: aarch64_vector_transfer_integer_move_signed");
                        }
                        (x0, x1, _, x3) if x0 == 0 && x1 == 0 && x3 == 7 => {
                            println!("encoding: aarch64_vector_transfer_integer_move_unsigned");
                        }
                        (x0, x1, _, _) if x0 == 0 && x1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, _, x3) if x0 == 1 && x1 == 0 && x3 == 3 => {
                            println!("encoding: aarch64_vector_transfer_integer_insert");
                        }
                        (x0, x1, _, x3) if x0 == 1 && x1 == 0 && x3 == 5 => {
                            println!("encoding: aarch64_vector_transfer_integer_move_signed");
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 & 15 == 8 && x3 == 7 => {
                            println!("encoding: aarch64_vector_transfer_integer_move_unsigned");
                        }
                        (x0, x1, _, _) if x0 == 1 && x1 == 1 => {
                            println!("encoding: aarch64_vector_transfer_vector_insert");
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 & 9 == 0 && x2 == 1 && x3 & 12 == 0 && x4 & 33 == 1 => {
                }
                (x0, _, x2, x3, x4, _) if x0 & 9 == 0 && x2 & 2 == 0 && x3 == 7 && x4 & 387 == 2 => {
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
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_max_min_fp16_2008");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_mul_fp16_fused");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 2 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_add_fp16");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 3 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_mul_fp16_extended_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 4 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 5 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 6 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_max_min_fp16_1985");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 7 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_recps_fp16_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_max_min_fp16_2008");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_mul_fp16_fused");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 2 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 3 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 4 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 5 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 6 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_max_min_fp16_1985");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 7 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_rsqrts_fp16_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_max_min_fp16_2008");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 2 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_add_fp16");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 3 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_mul_fp16_product");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 4 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 5 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 6 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_max_min_fp16_1985");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 7 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_div_fp16");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_max_min_fp16_2008");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 2 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 3 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 4 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 5 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 6 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_max_min_fp16_1985");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 7 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 & 9 == 0 && x2 & 2 == 0 && x3 & 12 == 8 && x4 & 49 == 17 => {
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
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 & 28 == 8 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 & 24 == 16 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 == 30 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 == 0 && x2 & 28 == 12 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 == 0 && x2 == 31 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 == 1 && x2 == 28 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 24 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_fp16_round");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 25 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_fp16_round");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 26 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 27 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 28 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_fp16_conv_float_tieaway_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 29 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_fp16_conv_int_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 12 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 13 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 14 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 15 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_diff_neg_fp16");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 24 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_fp16_round");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 25 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_fp16_round");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 26 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 27 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 29 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_special_recip_fp16_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 31 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 24 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_fp16_round");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 25 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_fp16_round");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 26 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 27 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 28 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_fp16_conv_float_tieaway_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 29 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_fp16_conv_int_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 12 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 13 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 14 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 15 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_diff_neg_fp16");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 24 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 25 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_fp16_round");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 26 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 27 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 29 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_special_sqrt_est_fp16_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 31 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_special_sqrt_fp16");
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 & 9 == 0 && x2 & 2 == 0 && x3 & 4 == 0 && x4 & 33 == 32 => {
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
                            println!("unallocated");
                        }
                        (_, _, x2, x3) if x2 == 3 && x3 == 3 => {
                            println!("unallocated");
                        }
                        (_, x1, _, x3) if x1 == 0 && x3 == 0 => {
                            println!("unallocated");
                        }
                        (_, x1, _, x3) if x1 == 0 && x3 == 1 => {
                            println!("unallocated");
                        }
                        (_, x1, _, x3) if x1 == 0 && x3 == 2 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_mul_int_dotp");
                        }
                        (_, x1, _, x3) if x1 == 0 && x3 & 8 == 8 => {
                            println!("unallocated");
                        }
                        (_, x1, x2, x3) if x1 == 0 && x2 == 2 && x3 == 3 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_mat_mul_int_usdot");
                        }
                        (_, x1, _, x3) if x1 == 1 && x3 == 0 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_accum_simd");
                        }
                        (_, x1, _, x3) if x1 == 1 && x3 == 1 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_accum_simd");
                        }
                        (_, x1, _, x3) if x1 == 1 && x3 == 2 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_mul_int_dotp");
                        }
                        (_, x1, _, x3) if x1 == 1 && x3 & 12 == 8 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_mul_fp_complex");
                        }
                        (_, x1, _, x3) if x1 == 1 && x3 & 13 == 12 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_add_fp_complex");
                        }
                        (_, x1, x2, x3) if x1 == 1 && x2 == 0 && x3 == 13 => {
                            println!("unallocated");
                        }
                        (_, x1, x2, x3) if x1 == 1 && x2 == 0 && x3 == 15 => {
                            println!("unallocated");
                        }
                        (_, x1, x2, x3) if x1 == 1 && x2 == 1 && x3 == 15 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_mul_int_bfdot");
                        }
                        (_, x1, x2, x3) if x1 == 1 && x2 & 2 == 2 && x3 == 13 => {
                            println!("unallocated");
                        }
                        (_, x1, x2, x3) if x1 == 1 && x2 == 2 && x3 == 3 => {
                            println!("unallocated");
                        }
                        (_, x1, x2, x3) if x1 == 1 && x2 == 2 && x3 == 15 => {
                            println!("unallocated");
                        }
                        (_, x1, x2, x3) if x1 == 1 && x2 == 3 && x3 == 15 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_mul_acc_bf16_long");
                        }
                        (x0, _, _, x3) if x0 == 0 && x3 & 12 == 4 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 13 => {
                            println!("unallocated");
                        }
                        (x0, _, x2, x3) if x0 == 1 && x2 & 2 == 0 && x3 & 12 == 4 => {
                            println!("unallocated");
                        }
                        (x0, _, x2, x3) if x0 == 1 && x2 & 2 == 2 && x3 & 14 == 6 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 2 && x3 == 4 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_mat_mul_int_mla");
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 2 && x3 == 5 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_mat_mul_int_mla");
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 13 => {
                            println!("encoding: aarch64_vector_bfmmla");
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 2 && x3 == 4 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_mat_mul_int_mla");
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 2 && x3 == 5 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 == 21 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 & 2 == 0 && x2 & 28 == 12 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 & 2 == 2 && x2 == 23 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 & 2 == 2 && x2 == 30 => {
                            println!("unallocated");
                        }
                        (_, x1, x2) if x1 == 3 && x2 == 22 => {
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_rev");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_rev");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 2 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_add_pairwise");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 3 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_add_saturating_simd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 4 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_clsz");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 5 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_cnt");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 6 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_add_pairwise");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 7 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_diff_neg_sat_simd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 8 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 9 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 10 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 11 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_diff_neg_int_simd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 18 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_extract_nosat");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 19 => {
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 20 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_extract_sat_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 22 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_narrow");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 23 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_widen");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 24 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_round");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 25 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_round");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 26 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_conv_float_bulk_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 27 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_conv_float_bulk_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 28 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_conv_float_tieaway_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 29 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_conv_int_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 30 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_round_frint_32_64");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 31 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_round_frint_32_64");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 12 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 13 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 14 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 15 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_diff_neg_float");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 24 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_round");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 25 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_round");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 26 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_conv_float_bulk_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 27 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_conv_float_bulk_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 28 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_special_recip_int");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 29 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_special_recip_float_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 31 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 22 => {
                            println!("encoding: aarch64_vector_cvt_bf16_vector");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_rev");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 1 => {
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 2 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_add_pairwise");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 3 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_add_saturating_simd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 4 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_clsz");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 6 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_add_pairwise");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 7 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_diff_neg_sat_simd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 8 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 9 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_cmp_int_bulk_simd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 10 => {
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 11 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_diff_neg_int_simd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 18 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_extract_sqxtun_simd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 19 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_shift");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 20 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_extract_sat_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 22 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_xtn_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 23 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 24 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_round");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 25 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_round");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 26 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_conv_float_bulk_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 27 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_conv_float_bulk_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 28 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_conv_float_tieaway_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 29 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_conv_int_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 30 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_round_frint_32_64");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 31 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_round_frint_32_64");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 5 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_not");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 5 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_rbit");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 5 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 12 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 13 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_cmp_float_bulk_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 14 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 15 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_diff_neg_float");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 24 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 25 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_round");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 26 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_conv_float_bulk_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 27 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_float_conv_float_bulk_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 28 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_special_sqrt_est_int");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 29 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_special_sqrt_est_float_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 31 => {
                            println!("encoding: aarch64_vector_arithmetic_unary_special_sqrt");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 == 22 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 == 2 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 & 28 == 4 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 & 30 == 8 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 == 11 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 == 13 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 == 14 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 & 24 == 16 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 & 30 == 24 => {
                            println!("unallocated");
                        }
                        (_, _, x2) if x2 & 28 == 28 => {
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 3 => {
                            println!("encoding: aarch64_vector_reduce_add_long");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 10 => {
                            println!("encoding: aarch64_vector_reduce_int_max");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 26 => {
                            println!("encoding: aarch64_vector_reduce_int_max");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 27 => {
                            println!("encoding: aarch64_vector_reduce_add_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 12 => {
                            println!("encoding: aarch64_vector_reduce_fp16_maxnm_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 15 => {
                            println!("encoding: aarch64_vector_reduce_fp16_max_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 12 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 15 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 12 => {
                            println!("encoding: aarch64_vector_reduce_fp16_maxnm_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 15 => {
                            println!("encoding: aarch64_vector_reduce_fp16_max_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 12 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 15 => {
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 3 => {
                            println!("encoding: aarch64_vector_reduce_add_long");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 10 => {
                            println!("encoding: aarch64_vector_reduce_int_max");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 26 => {
                            println!("encoding: aarch64_vector_reduce_int_max");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 27 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 12 => {
                            println!("encoding: aarch64_vector_reduce_fp_maxnm_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 15 => {
                            println!("encoding: aarch64_vector_reduce_fp_max_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 12 => {
                            println!("encoding: aarch64_vector_reduce_fp_maxnm_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 15 => {
                            println!("encoding: aarch64_vector_reduce_fp_max_simd");
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 & 9 == 0 && x2 & 2 == 0 && x3 & 4 == 4 && x4 & 259 == 258 => {
                }
                (x0, _, x2, x3, x4, _) if x0 & 9 == 0 && x2 & 2 == 0 && x3 & 4 == 4 && x4 & 131 == 130 => {
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
                            println!("unallocated");
                        }
                        (x0, x1) if x0 == 0 && x1 == 0 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_disparate_add_sub_long");
                        }
                        (x0, x1) if x0 == 0 && x1 == 1 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_disparate_add_sub_wide");
                        }
                        (x0, x1) if x0 == 0 && x1 == 2 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_disparate_add_sub_long");
                        }
                        (x0, x1) if x0 == 0 && x1 == 3 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_disparate_add_sub_wide");
                        }
                        (x0, x1) if x0 == 0 && x1 == 4 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow");
                        }
                        (x0, x1) if x0 == 0 && x1 == 5 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_disparate_diff");
                        }
                        (x0, x1) if x0 == 0 && x1 == 6 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow");
                        }
                        (x0, x1) if x0 == 0 && x1 == 7 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_disparate_diff");
                        }
                        (x0, x1) if x0 == 0 && x1 == 8 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_disparate_mul_accum");
                        }
                        (x0, x1) if x0 == 0 && x1 == 9 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_disparate_mul_dmacc_simd");
                        }
                        (x0, x1) if x0 == 0 && x1 == 10 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_disparate_mul_accum");
                        }
                        (x0, x1) if x0 == 0 && x1 == 11 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_disparate_mul_dmacc_simd");
                        }
                        (x0, x1) if x0 == 0 && x1 == 12 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_disparate_mul_product");
                        }
                        (x0, x1) if x0 == 0 && x1 == 13 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_disparate_mul_double_simd");
                        }
                        (x0, x1) if x0 == 0 && x1 == 14 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_disparate_mul_poly");
                        }
                        (x0, x1) if x0 == 1 && x1 == 0 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_disparate_add_sub_long");
                        }
                        (x0, x1) if x0 == 1 && x1 == 1 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_disparate_add_sub_wide");
                        }
                        (x0, x1) if x0 == 1 && x1 == 2 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_disparate_add_sub_long");
                        }
                        (x0, x1) if x0 == 1 && x1 == 3 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_disparate_add_sub_wide");
                        }
                        (x0, x1) if x0 == 1 && x1 == 4 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow");
                        }
                        (x0, x1) if x0 == 1 && x1 == 5 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_disparate_diff");
                        }
                        (x0, x1) if x0 == 1 && x1 == 6 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_disparate_add_sub_narrow");
                        }
                        (x0, x1) if x0 == 1 && x1 == 7 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_disparate_diff");
                        }
                        (x0, x1) if x0 == 1 && x1 == 8 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_disparate_mul_accum");
                        }
                        (x0, x1) if x0 == 1 && x1 == 9 => {
                            println!("unallocated");
                        }
                        (x0, x1) if x0 == 1 && x1 == 10 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_disparate_mul_accum");
                        }
                        (x0, x1) if x0 == 1 && x1 == 11 => {
                            println!("unallocated");
                        }
                        (x0, x1) if x0 == 1 && x1 == 12 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_disparate_mul_product");
                        }
                        (x0, x1) if x0 == 1 && x1 == 13 => {
                            println!("unallocated");
                        }
                        (x0, x1) if x0 == 1 && x1 == 14 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
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
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 2 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 4 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_sub_int");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 5 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 6 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 7 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 8 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_shift_simd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 9 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_shift_simd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 10 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_shift_simd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 11 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_shift_simd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 12 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_max_min_single");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 13 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_max_min_single");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 14 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_diff");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 15 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_diff");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 16 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 17 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 18 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_mul_int_accum");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 19 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_mul_int_product");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 20 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_max_min_pair");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 21 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_max_min_pair");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 22 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_simd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 23 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 24 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_max_min_fp_2008");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 25 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_mul_fp_fused");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 26 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_add_fp");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 27 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_mul_fp_extended_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 28 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 30 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_max_min_fp_1985");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 31 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_recps_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 3 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_logical_and_orr");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 29 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_lower");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 3 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_logical_and_orr");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 29 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 24 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_max_min_fp_2008");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 25 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_mul_fp_fused");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 26 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 27 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 28 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 30 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_max_min_fp_1985");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 31 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_rsqrts_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 3 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_logical_and_orr");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 29 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_lower");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 3 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_logical_and_orr");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 29 => {
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_add_halving_truncating");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 1 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_add_saturating_simd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 2 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_add_halving_rounding");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 4 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_sub_int");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 5 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 6 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 7 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_cmp_int_simd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 8 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_shift_simd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 9 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_shift_simd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 10 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_shift_simd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 11 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_shift_simd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 12 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_max_min_single");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 13 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_max_min_single");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 14 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_diff");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 15 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_diff");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 16 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 17 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 18 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_mul_int_accum");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 19 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_mul_int_product");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 20 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_max_min_pair");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 21 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_max_min_pair");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 22 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_simd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 23 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 24 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_max_min_fp_2008");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 26 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_add_fp");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 27 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_mul_fp_product");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 28 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 29 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 30 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_max_min_fp_1985");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 31 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_div");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 3 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_logical_bsl_eor");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 25 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_upper");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 3 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_logical_bsl_eor");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 25 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 24 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_max_min_fp_2008");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 26 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_sub_fp_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 27 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 28 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 29 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 30 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_max_min_fp_1985");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 31 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 == 3 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_logical_bsl_eor");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 == 25 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_upper");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 3 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_uniform_logical_bsl_eor");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 25 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (_, x1, x2, x3) if x1 == 0 && x2 & 9 == 0 && x3 == 0 => {
                            println!("encoding: aarch64_vector_logical");
                        }
                        (_, x1, x2, x3) if x1 == 0 && x2 & 9 == 1 && x3 == 0 => {
                            println!("encoding: aarch64_vector_logical");
                        }
                        (_, x1, x2, x3) if x1 == 0 && x2 & 12 == 8 && x3 == 1 => {
                            println!("unallocated");
                        }
                        (_, x1, x2, x3) if x1 == 0 && x2 & 13 == 8 && x3 == 0 => {
                            println!("encoding: aarch64_vector_logical");
                        }
                        (_, x1, x2, x3) if x1 == 0 && x2 & 13 == 9 && x3 == 0 => {
                            println!("encoding: aarch64_vector_logical");
                        }
                        (_, x1, x2, x3) if x1 == 0 && x2 & 14 == 12 && x3 == 0 => {
                            println!("encoding: aarch64_vector_logical");
                        }
                        (_, x1, x2, x3) if x1 == 0 && x2 & 14 == 12 && x3 == 1 => {
                            println!("unallocated");
                        }
                        (_, x1, x2, x3) if x1 == 0 && x2 == 14 && x3 == 0 => {
                            println!("encoding: aarch64_vector_logical");
                        }
                        (_, x1, x2, x3) if x1 == 0 && x2 == 14 && x3 == 1 => {
                            println!("unallocated");
                        }
                        (_, x1, x2, x3) if x1 == 0 && x2 == 15 && x3 == 0 => {
                            println!("encoding: aarch64_vector_logical");
                        }
                        (_, x1, x2, x3) if x1 == 0 && x2 == 15 && x3 == 1 => {
                            println!("encoding: aarch64_vector_fp16_movi");
                        }
                        (_, x1, _, x3) if x1 == 1 && x3 == 1 => {
                            println!("unallocated");
                        }
                        (_, x1, x2, x3) if x1 == 1 && x2 & 9 == 0 && x3 == 0 => {
                            println!("encoding: aarch64_vector_logical");
                        }
                        (_, x1, x2, x3) if x1 == 1 && x2 & 9 == 1 && x3 == 0 => {
                            println!("encoding: aarch64_vector_logical");
                        }
                        (_, x1, x2, x3) if x1 == 1 && x2 & 13 == 8 && x3 == 0 => {
                            println!("encoding: aarch64_vector_logical");
                        }
                        (_, x1, x2, x3) if x1 == 1 && x2 & 13 == 9 && x3 == 0 => {
                            println!("encoding: aarch64_vector_logical");
                        }
                        (_, x1, x2, x3) if x1 == 1 && x2 & 14 == 12 && x3 == 0 => {
                            println!("encoding: aarch64_vector_logical");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 14 && x3 == 0 => {
                            println!("encoding: aarch64_vector_logical");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 15 && x3 == 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 14 && x3 == 0 => {
                            println!("encoding: aarch64_vector_logical");
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 15 && x3 == 0 => {
                            println!("encoding: aarch64_vector_logical");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (_, x1) if x1 == 3 => {
                            println!("unallocated");
                        }
                        (_, x1) if x1 == 5 => {
                            println!("unallocated");
                        }
                        (_, x1) if x1 == 7 => {
                            println!("unallocated");
                        }
                        (_, x1) if x1 == 9 => {
                            println!("unallocated");
                        }
                        (_, x1) if x1 == 11 => {
                            println!("unallocated");
                        }
                        (_, x1) if x1 == 13 => {
                            println!("unallocated");
                        }
                        (_, x1) if x1 == 15 => {
                            println!("unallocated");
                        }
                        (_, x1) if x1 == 21 => {
                            println!("unallocated");
                        }
                        (_, x1) if x1 & 30 == 22 => {
                            println!("unallocated");
                        }
                        (_, x1) if x1 & 28 == 24 => {
                            println!("unallocated");
                        }
                        (_, x1) if x1 == 29 => {
                            println!("unallocated");
                        }
                        (_, x1) if x1 == 30 => {
                            println!("unallocated");
                        }
                        (x0, x1) if x0 == 0 && x1 == 0 => {
                            println!("encoding: aarch64_vector_shift_right_simd");
                        }
                        (x0, x1) if x0 == 0 && x1 == 2 => {
                            println!("encoding: aarch64_vector_shift_right_simd");
                        }
                        (x0, x1) if x0 == 0 && x1 == 4 => {
                            println!("encoding: aarch64_vector_shift_right_simd");
                        }
                        (x0, x1) if x0 == 0 && x1 == 6 => {
                            println!("encoding: aarch64_vector_shift_right_simd");
                        }
                        (x0, x1) if x0 == 0 && x1 == 8 => {
                            println!("unallocated");
                        }
                        (x0, x1) if x0 == 0 && x1 == 10 => {
                            println!("encoding: aarch64_vector_shift_left_simd");
                        }
                        (x0, x1) if x0 == 0 && x1 == 12 => {
                            println!("unallocated");
                        }
                        (x0, x1) if x0 == 0 && x1 == 14 => {
                            println!("encoding: aarch64_vector_shift_left_sat_simd");
                        }
                        (x0, x1) if x0 == 0 && x1 == 16 => {
                            println!("encoding: aarch64_vector_shift_right_narrow_logical");
                        }
                        (x0, x1) if x0 == 0 && x1 == 17 => {
                            println!("encoding: aarch64_vector_shift_right_narrow_logical");
                        }
                        (x0, x1) if x0 == 0 && x1 == 18 => {
                            println!("encoding: aarch64_vector_shift_right_narrow_uniform_simd");
                        }
                        (x0, x1) if x0 == 0 && x1 == 19 => {
                            println!("encoding: aarch64_vector_shift_right_narrow_uniform_simd");
                        }
                        (x0, x1) if x0 == 0 && x1 == 20 => {
                            println!("encoding: aarch64_vector_shift_left_long");
                        }
                        (x0, x1) if x0 == 0 && x1 == 28 => {
                            println!("encoding: aarch64_vector_shift_conv_int_simd");
                        }
                        (x0, x1) if x0 == 0 && x1 == 31 => {
                            println!("encoding: aarch64_vector_shift_conv_float_simd");
                        }
                        (x0, x1) if x0 == 1 && x1 == 0 => {
                            println!("encoding: aarch64_vector_shift_right_simd");
                        }
                        (x0, x1) if x0 == 1 && x1 == 2 => {
                            println!("encoding: aarch64_vector_shift_right_simd");
                        }
                        (x0, x1) if x0 == 1 && x1 == 4 => {
                            println!("encoding: aarch64_vector_shift_right_simd");
                        }
                        (x0, x1) if x0 == 1 && x1 == 6 => {
                            println!("encoding: aarch64_vector_shift_right_simd");
                        }
                        (x0, x1) if x0 == 1 && x1 == 8 => {
                            println!("encoding: aarch64_vector_shift_right_insert_simd");
                        }
                        (x0, x1) if x0 == 1 && x1 == 10 => {
                            println!("encoding: aarch64_vector_shift_left_insert_simd");
                        }
                        (x0, x1) if x0 == 1 && x1 == 12 => {
                            println!("encoding: aarch64_vector_shift_left_sat_simd");
                        }
                        (x0, x1) if x0 == 1 && x1 == 14 => {
                            println!("encoding: aarch64_vector_shift_left_sat_simd");
                        }
                        (x0, x1) if x0 == 1 && x1 == 16 => {
                            println!("encoding: aarch64_vector_shift_right_narrow_nonuniform_simd");
                        }
                        (x0, x1) if x0 == 1 && x1 == 17 => {
                            println!("encoding: aarch64_vector_shift_right_narrow_nonuniform_simd");
                        }
                        (x0, x1) if x0 == 1 && x1 == 18 => {
                            println!("encoding: aarch64_vector_shift_right_narrow_uniform_simd");
                        }
                        (x0, x1) if x0 == 1 && x1 == 19 => {
                            println!("encoding: aarch64_vector_shift_right_narrow_uniform_simd");
                        }
                        (x0, x1) if x0 == 1 && x1 == 20 => {
                            println!("encoding: aarch64_vector_shift_left_long");
                        }
                        (x0, x1) if x0 == 1 && x1 == 28 => {
                            println!("encoding: aarch64_vector_shift_conv_int_simd");
                        }
                        (x0, x1) if x0 == 1 && x1 == 31 => {
                            println!("encoding: aarch64_vector_shift_conv_float_simd");
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, _, x4, _) if x0 & 9 == 0 && x2 == 3 && x4 & 1 == 1 => {
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
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 2 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_acc_long");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 3 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_acc_double_simd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 6 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_acc_long");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 7 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_acc_double_simd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 8 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_int");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 10 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_long");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 11 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_double_simd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 12 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_high_simd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 13 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_high_simd");
                        }
                        (x0, _, x2) if x0 == 0 && x2 == 14 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_dotp");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 4 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_acc_fp16_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 5 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_acc_fp16_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 9 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_fp16_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 15 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mat_mul_int_dotp");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 5 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 15 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_bfdot");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 1 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_acc_fp_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 5 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_acc_fp_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 9 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_fp_simd");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 0 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_acc_mul_norounding_i_lower");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 4 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_acc_mul_norounding_i_lower");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 15 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mat_mul_int_dotp");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 4 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 15 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_acc_bf16_long");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 0 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_acc_int");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 2 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_acc_long");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 4 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_acc_int");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 6 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_acc_long");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 10 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_long");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 11 => {
                            println!("unallocated");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 13 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_acc_high_simd");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 14 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_dotp");
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 15 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_acc_high_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 8 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 12 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 3 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 5 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 7 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 9 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_fp16_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 & 9 == 1 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_acc_complex");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 9 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_fp_simd");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 & 9 == 1 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_acc_complex");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 == 8 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_acc_mul_norounding_i_upper");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 == 12 => {
                            println!("encoding: aarch64_vector_arithmetic_binary_element_mul_acc_mul_norounding_i_upper");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 3 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 5 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 7 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 8 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 12 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
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
                            println!("encoding: aarch64_vector_crypto_sm3_sm3tt1a");
                        }
                        x0 if x0 == 1 => {
                            println!("encoding: aarch64_vector_crypto_sm3_sm3tt1b");
                        }
                        x0 if x0 == 2 => {
                            println!("encoding: aarch64_vector_crypto_sm3_sm3tt2a");
                        }
                        x0 if x0 == 3 => {
                            println!("encoding: aarch64_vector_crypto_sm3_sm3tt2b");
                        }
                        _ => unreachable!()
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
                            println!("encoding: aarch64_vector_crypto_sha512_sha512h");
                        }
                        (x0, x1) if x0 == 0 && x1 == 1 => {
                            println!("encoding: aarch64_vector_crypto_sha512_sha512h2");
                        }
                        (x0, x1) if x0 == 0 && x1 == 2 => {
                            println!("encoding: aarch64_vector_crypto_sha512_sha512su1");
                        }
                        (x0, x1) if x0 == 0 && x1 == 3 => {
                            println!("encoding: aarch64_vector_crypto_sha3_rax1");
                        }
                        (x0, x1) if x0 == 1 && x1 == 0 => {
                            println!("encoding: aarch64_vector_crypto_sm3_sm3partw1");
                        }
                        (x0, x1) if x0 == 1 && x1 == 1 => {
                            println!("encoding: aarch64_vector_crypto_sm3_sm3partw2");
                        }
                        (x0, x1) if x0 == 1 && x1 == 2 => {
                            println!("encoding: aarch64_vector_crypto_sm4_sm4enckey");
                        }
                        (x0, x1) if x0 == 1 && x1 == 3 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
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
                            println!("encoding: aarch64_vector_crypto_sha3_eor3");
                        }
                        x0 if x0 == 1 => {
                            println!("encoding: aarch64_vector_crypto_sha3_bcax");
                        }
                        x0 if x0 == 2 => {
                            println!("encoding: aarch64_vector_crypto_sm3_sm3ss1");
                        }
                        x0 if x0 == 3 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, x3, _, _) if x0 == 12 && x2 == 1 && x3 & 12 == 0 => {
                    let Rm = (instr >> 16) & 9;
                    let imm6 = (instr >> 10) & 11;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match () {
                        () => {
                            println!("encoding: aarch64_vector_crypto_sha3_xar");
                        }
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 == 12 && x2 == 1 && x3 == 8 && x4 & 508 == 32 => {
                    let opcode = (instr >> 10) & 3;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match opcode {
                        x0 if x0 == 0 => {
                            println!("encoding: aarch64_vector_crypto_sha512_sha512su0");
                        }
                        x0 if x0 == 1 => {
                            println!("encoding: aarch64_vector_crypto_sm4_sm4enc");
                        }
                        x0 if x0 & 2 == 2 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
                    }
                }
                (x0, _, x2, _, _, _) if x0 & 9 == 8 && x2 & 2 == 2 => {
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
                            println!("unallocated");
                        }
                        (_, _, _, x3, x4, _) if x3 & 1 == 0 && x4 & 6 == 0 => {
                            println!("unallocated");
                        }
                        (_, _, _, x3, x4, _) if x3 & 1 == 1 && x4 & 6 == 2 => {
                            println!("unallocated");
                        }
                        (_, _, _, x3, x4, _) if x3 & 2 == 0 && x4 & 6 == 0 => {
                            println!("unallocated");
                        }
                        (_, _, _, x3, x4, _) if x3 & 2 == 2 && x4 & 6 == 2 => {
                            println!("unallocated");
                        }
                        (_, _, x2, _, _, _) if x2 == 2 => {
                            println!("unallocated");
                        }
                        (_, x1, _, _, _, _) if x1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, _, _, _, _, x5) if x0 == 0 && x5 & 32 == 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 2 => {
                            println!("encoding: aarch64_float_convert_fix");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 3 => {
                            println!("encoding: aarch64_float_convert_fix");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 3 && x4 == 0 => {
                            println!("encoding: aarch64_float_convert_fix");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 3 && x4 == 1 => {
                            println!("encoding: aarch64_float_convert_fix");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 2 => {
                            println!("encoding: aarch64_float_convert_fix");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 3 => {
                            println!("encoding: aarch64_float_convert_fix");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 3 && x4 == 0 => {
                            println!("encoding: aarch64_float_convert_fix");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 3 && x4 == 1 => {
                            println!("encoding: aarch64_float_convert_fix");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 2 => {
                            println!("encoding: aarch64_float_convert_fix");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 3 => {
                            println!("encoding: aarch64_float_convert_fix");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 3 && x4 == 0 => {
                            println!("encoding: aarch64_float_convert_fix");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 3 && x4 == 1 => {
                            println!("encoding: aarch64_float_convert_fix");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 2 => {
                            println!("encoding: aarch64_float_convert_fix");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 3 => {
                            println!("encoding: aarch64_float_convert_fix");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 3 && x4 == 0 => {
                            println!("encoding: aarch64_float_convert_fix");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 3 && x4 == 1 => {
                            println!("encoding: aarch64_float_convert_fix");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 2 => {
                            println!("encoding: aarch64_float_convert_fix");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 3 => {
                            println!("encoding: aarch64_float_convert_fix");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 3 && x4 == 0 => {
                            println!("encoding: aarch64_float_convert_fix");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 3 && x4 == 1 => {
                            println!("encoding: aarch64_float_convert_fix");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 2 => {
                            println!("encoding: aarch64_float_convert_fix");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 3 => {
                            println!("encoding: aarch64_float_convert_fix");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 3 && x4 == 0 => {
                            println!("encoding: aarch64_float_convert_fix");
                        }
                        (x0, x1, x2, x3, x4, _) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 3 && x4 == 1 => {
                            println!("encoding: aarch64_float_convert_fix");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (_, _, _, x3, x4) if x3 & 1 == 1 && x4 & 6 == 4 => {
                            println!("unallocated");
                        }
                        (_, _, _, x3, x4) if x3 & 2 == 2 && x4 & 6 == 2 => {
                            println!("unallocated");
                        }
                        (_, _, _, x3, x4) if x3 & 2 == 2 && x4 & 6 == 4 => {
                            println!("unallocated");
                        }
                        (_, x1, x2, _, x4) if x1 == 0 && x2 == 2 && x4 & 4 == 0 => {
                            println!("unallocated");
                        }
                        (_, x1, x2, _, x4) if x1 == 0 && x2 == 2 && x4 & 6 == 4 => {
                            println!("unallocated");
                        }
                        (_, x1, _, _, _) if x1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 & 1 == 1 && x4 & 6 == 6 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 1 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 2 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 3 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 4 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 5 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 6 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 7 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 1 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 & 2 == 2 && x4 & 6 == 6 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 2 && x4 == 0 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 2 && x4 == 1 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 3 && x4 == 0 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 3 && x4 == 1 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 & 2 == 0 && x4 & 6 == 6 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 2 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 3 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 4 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 5 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 2 && x4 == 0 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 2 && x4 == 1 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 2 && x4 & 6 == 6 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 3 && x4 == 0 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 3 && x4 == 1 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 3 && x4 == 6 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 3 && x4 == 7 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 0 && x2 == 2 && x4 & 6 == 6 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 1 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 2 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 3 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 4 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 5 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 6 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 7 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 1 && x4 == 0 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 1 && x4 == 1 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 2 && x4 == 0 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 2 && x4 == 1 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 3 && x4 == 0 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 3 && x4 == 1 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 0 && x2 == 0 && x4 & 6 == 6 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 1 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 2 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 3 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 4 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 5 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 1 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 2 && x4 == 0 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 2 && x4 == 1 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 3 && x4 == 0 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 3 && x4 == 1 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 & 1 == 1 && x4 & 6 == 6 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 2 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 3 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 4 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 5 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 6 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 7 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 & 2 == 2 && x4 & 6 == 6 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 2 && x4 == 0 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 2 && x4 == 1 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 3 && x4 == 0 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 3 && x4 == 1 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 2 && x3 & 1 == 0 && x4 & 6 == 6 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 2 && x3 == 1 && x4 == 6 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 2 && x3 == 1 && x4 == 7 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 2 && x3 & 2 == 2 && x4 & 6 == 6 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 1 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 2 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 3 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 4 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 5 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 6 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 7 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 1 && x4 == 0 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 1 && x4 == 1 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 2 && x4 == 0 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 2 && x4 == 1 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 3 && x4 == 0 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 3 && x4 == 1 => {
                            println!("encoding: aarch64_float_convert_int");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (_, x1, _, _) if x1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            println!("encoding: aarch64_float_arithmetic_unary");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            println!("encoding: aarch64_float_arithmetic_unary");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 2 => {
                            println!("encoding: aarch64_float_arithmetic_unary");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 3 => {
                            println!("encoding: aarch64_float_arithmetic_unary");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 4 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 5 => {
                            println!("encoding: aarch64_float_convert_fp");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 6 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 7 => {
                            println!("encoding: aarch64_float_convert_fp");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 8 => {
                            println!("encoding: aarch64_float_arithmetic_round_frint");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 9 => {
                            println!("encoding: aarch64_float_arithmetic_round_frint");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 10 => {
                            println!("encoding: aarch64_float_arithmetic_round_frint");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 11 => {
                            println!("encoding: aarch64_float_arithmetic_round_frint");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 12 => {
                            println!("encoding: aarch64_float_arithmetic_round_frint");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 13 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 14 => {
                            println!("encoding: aarch64_float_arithmetic_round_frint");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 15 => {
                            println!("encoding: aarch64_float_arithmetic_round_frint");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 16 => {
                            println!("encoding: aarch64_float_arithmetic_round_frint_32_64");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 17 => {
                            println!("encoding: aarch64_float_arithmetic_round_frint_32_64");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 18 => {
                            println!("encoding: aarch64_float_arithmetic_round_frint_32_64");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 19 => {
                            println!("encoding: aarch64_float_arithmetic_round_frint_32_64");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 & 60 == 20 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 & 56 == 24 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            println!("encoding: aarch64_float_arithmetic_unary");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 => {
                            println!("encoding: aarch64_float_arithmetic_unary");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 2 => {
                            println!("encoding: aarch64_float_arithmetic_unary");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 3 => {
                            println!("encoding: aarch64_float_arithmetic_unary");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 4 => {
                            println!("encoding: aarch64_float_convert_fp");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 5 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 6 => {
                            println!("encoding: aarch64_vector_cvt_bf16_scalar");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 7 => {
                            println!("encoding: aarch64_float_convert_fp");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 8 => {
                            println!("encoding: aarch64_float_arithmetic_round_frint");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 9 => {
                            println!("encoding: aarch64_float_arithmetic_round_frint");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 10 => {
                            println!("encoding: aarch64_float_arithmetic_round_frint");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 11 => {
                            println!("encoding: aarch64_float_arithmetic_round_frint");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 12 => {
                            println!("encoding: aarch64_float_arithmetic_round_frint");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 13 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 14 => {
                            println!("encoding: aarch64_float_arithmetic_round_frint");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 15 => {
                            println!("encoding: aarch64_float_arithmetic_round_frint");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 16 => {
                            println!("encoding: aarch64_float_arithmetic_round_frint_32_64");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 17 => {
                            println!("encoding: aarch64_float_arithmetic_round_frint_32_64");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 18 => {
                            println!("encoding: aarch64_float_arithmetic_round_frint_32_64");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 19 => {
                            println!("encoding: aarch64_float_arithmetic_round_frint_32_64");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 & 60 == 20 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 & 56 == 24 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 2 && x3 & 32 == 0 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 => {
                            println!("encoding: aarch64_float_arithmetic_unary");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 1 => {
                            println!("encoding: aarch64_float_arithmetic_unary");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 2 => {
                            println!("encoding: aarch64_float_arithmetic_unary");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 3 => {
                            println!("encoding: aarch64_float_arithmetic_unary");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 4 => {
                            println!("encoding: aarch64_float_convert_fp");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 5 => {
                            println!("encoding: aarch64_float_convert_fp");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 & 62 == 6 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 8 => {
                            println!("encoding: aarch64_float_arithmetic_round_frint");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 9 => {
                            println!("encoding: aarch64_float_arithmetic_round_frint");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 10 => {
                            println!("encoding: aarch64_float_arithmetic_round_frint");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 11 => {
                            println!("encoding: aarch64_float_arithmetic_round_frint");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 12 => {
                            println!("encoding: aarch64_float_arithmetic_round_frint");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 13 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 14 => {
                            println!("encoding: aarch64_float_arithmetic_round_frint");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 15 => {
                            println!("encoding: aarch64_float_arithmetic_round_frint");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 & 48 == 16 => {
                            println!("unallocated");
                        }
                        (x0, _, _, _) if x0 == 1 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (_, _, _, _, x4) if x4 & 2 == 2 => {
                            println!("unallocated");
                        }
                        (_, _, _, _, x4) if x4 & 4 == 4 => {
                            println!("unallocated");
                        }
                        (_, _, _, x3, _) if x3 & 1 == 1 => {
                            println!("unallocated");
                        }
                        (_, _, _, x3, _) if x3 & 2 == 2 => {
                            println!("unallocated");
                        }
                        (_, _, x2, _, _) if x2 == 2 => {
                            println!("unallocated");
                        }
                        (_, x1, _, _, _) if x1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_float_compare_uncond");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 8 => {
                            println!("encoding: aarch64_float_compare_uncond");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 16 => {
                            println!("encoding: aarch64_float_compare_uncond");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 24 => {
                            println!("encoding: aarch64_float_compare_uncond");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_float_compare_uncond");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 8 => {
                            println!("encoding: aarch64_float_compare_uncond");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 16 => {
                            println!("encoding: aarch64_float_compare_uncond");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 24 => {
                            println!("encoding: aarch64_float_compare_uncond");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_float_compare_uncond");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 8 => {
                            println!("encoding: aarch64_float_compare_uncond");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 16 => {
                            println!("encoding: aarch64_float_compare_uncond");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 24 => {
                            println!("encoding: aarch64_float_compare_uncond");
                        }
                        (x0, _, _, _, _) if x0 == 1 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (_, _, _, x3) if x3 & 2 == 2 => {
                            println!("unallocated");
                        }
                        (_, _, _, x3) if x3 & 4 == 4 => {
                            println!("unallocated");
                        }
                        (_, _, _, x3) if x3 & 8 == 8 => {
                            println!("unallocated");
                        }
                        (_, _, _, x3) if x3 & 16 == 16 => {
                            println!("unallocated");
                        }
                        (_, _, x2, _) if x2 == 2 => {
                            println!("unallocated");
                        }
                        (_, x1, _, _) if x1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            println!("encoding: aarch64_float_move_fp_imm");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            println!("encoding: aarch64_float_move_fp_imm");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 => {
                            println!("encoding: aarch64_float_move_fp_imm");
                        }
                        (x0, _, _, _) if x0 == 1 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (_, x1, _, _) if x1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            println!("encoding: aarch64_float_compare_cond");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            println!("encoding: aarch64_float_compare_cond");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            println!("encoding: aarch64_float_compare_cond");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 => {
                            println!("encoding: aarch64_float_compare_cond");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 => {
                            println!("encoding: aarch64_float_compare_cond");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 1 => {
                            println!("encoding: aarch64_float_compare_cond");
                        }
                        (x0, _, _, _) if x0 == 1 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (_, _, _, x3) if x3 & 10 == 10 => {
                            println!("unallocated");
                        }
                        (_, _, _, x3) if x3 & 12 == 12 => {
                            println!("unallocated");
                        }
                        (_, _, x2, _) if x2 == 2 => {
                            println!("unallocated");
                        }
                        (_, x1, _, _) if x1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            println!("encoding: aarch64_float_arithmetic_mul_product");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            println!("encoding: aarch64_float_arithmetic_div");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 2 => {
                            println!("encoding: aarch64_float_arithmetic_add_sub");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 3 => {
                            println!("encoding: aarch64_float_arithmetic_add_sub");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 4 => {
                            println!("encoding: aarch64_float_arithmetic_max_min");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 5 => {
                            println!("encoding: aarch64_float_arithmetic_max_min");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 6 => {
                            println!("encoding: aarch64_float_arithmetic_max_min");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 7 => {
                            println!("encoding: aarch64_float_arithmetic_max_min");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 8 => {
                            println!("encoding: aarch64_float_arithmetic_mul_product");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            println!("encoding: aarch64_float_arithmetic_mul_product");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 => {
                            println!("encoding: aarch64_float_arithmetic_div");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 2 => {
                            println!("encoding: aarch64_float_arithmetic_add_sub");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 3 => {
                            println!("encoding: aarch64_float_arithmetic_add_sub");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 4 => {
                            println!("encoding: aarch64_float_arithmetic_max_min");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 5 => {
                            println!("encoding: aarch64_float_arithmetic_max_min");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 6 => {
                            println!("encoding: aarch64_float_arithmetic_max_min");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 7 => {
                            println!("encoding: aarch64_float_arithmetic_max_min");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 8 => {
                            println!("encoding: aarch64_float_arithmetic_mul_product");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 => {
                            println!("encoding: aarch64_float_arithmetic_mul_product");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 1 => {
                            println!("encoding: aarch64_float_arithmetic_div");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 2 => {
                            println!("encoding: aarch64_float_arithmetic_add_sub");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 3 => {
                            println!("encoding: aarch64_float_arithmetic_add_sub");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 4 => {
                            println!("encoding: aarch64_float_arithmetic_max_min");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 5 => {
                            println!("encoding: aarch64_float_arithmetic_max_min");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 6 => {
                            println!("encoding: aarch64_float_arithmetic_max_min");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 7 => {
                            println!("encoding: aarch64_float_arithmetic_max_min");
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 8 => {
                            println!("encoding: aarch64_float_arithmetic_mul_product");
                        }
                        (x0, _, _, _) if x0 == 1 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (_, x1, _) if x1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            println!("encoding: aarch64_float_move_fp_select");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            println!("encoding: aarch64_float_move_fp_select");
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 3 => {
                            println!("encoding: aarch64_float_move_fp_select");
                        }
                        (x0, _, _) if x0 == 1 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
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
                            println!("unallocated");
                        }
                        (_, x1, _, _, _) if x1 == 1 => {
                            println!("unallocated");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_float_arithmetic_mul_add_sub");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 1 => {
                            println!("encoding: aarch64_float_arithmetic_mul_add_sub");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 => {
                            println!("encoding: aarch64_float_arithmetic_mul_add_sub");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 1 => {
                            println!("encoding: aarch64_float_arithmetic_mul_add_sub");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_float_arithmetic_mul_add_sub");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 => {
                            println!("encoding: aarch64_float_arithmetic_mul_add_sub");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 => {
                            println!("encoding: aarch64_float_arithmetic_mul_add_sub");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 => {
                            println!("encoding: aarch64_float_arithmetic_mul_add_sub");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 0 => {
                            println!("encoding: aarch64_float_arithmetic_mul_add_sub");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 1 => {
                            println!("encoding: aarch64_float_arithmetic_mul_add_sub");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 1 && x4 == 0 => {
                            println!("encoding: aarch64_float_arithmetic_mul_add_sub");
                        }
                        (x0, x1, x2, x3, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 1 && x4 == 1 => {
                            println!("encoding: aarch64_float_arithmetic_mul_add_sub");
                        }
                        (x0, _, _, _, _) if x0 == 1 => {
                            println!("unallocated");
                        }
                        _ => unreachable!()
                    }
                }
                _ => unreachable!()
            }
        }
        _ => unreachable!()
    }
} // end of decoding A64

#[cfg(test)]
mod tests {
    use super::decode_a64;

    #[test]
    fn test() {
        decode_a64(0x30_00_00_00);
    }
}
