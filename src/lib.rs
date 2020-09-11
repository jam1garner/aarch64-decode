#[allow(unused_variables)]
#[allow(non_snake_case)]
#[allow(unreachable_patterns)]
pub fn decode_a64(instr: u32) -> Option<Op> {
    match ((instr >> 29) & 5, (instr >> 24) & 9, instr & 47) {
        (_, x1, _) if x1 & 30 == 0 => {
            match (
                (instr >> 29) & 5,
                (instr >> 25) & 7,
                (instr >> 16) & 17,
                instr & 31,
            ) {
                (x0, _, x2, _) if x0 == 0 && x2 == 0 => {
                    let imm16 = instr & 31;
                    match () {
                        () => Some(Op::UdfOnlyPermUndef),
                    }
                }
                (_, _, x2, _) if x2 != 0 => None,
                (x0, _, _, _) if x0 != 0 => None,
                _ => None,
            }
        }
        (_, x1, _) if x1 == 3 => None,
        (_, x1, _) if x1 & 30 == 4 => {
            match (
                (instr >> 29) & 5,
                (instr >> 25) & 7,
                (instr >> 23) & 3,
                (instr >> 22) & 1,
                (instr >> 17) & 9,
                (instr >> 16) & 1,
                (instr >> 10) & 11,
                instr & 19,
            ) {
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 0 && x2 & 2 == 0 && x4 & 16 == 0 && x6 & 16 == 16 =>
                {
                    match (
                        (instr >> 24) & 15,
                        (instr >> 22) & 3,
                        (instr >> 21) & 1,
                        (instr >> 16) & 9,
                        (instr >> 15) & 1,
                        (instr >> 14) & 1,
                        instr & 27,
                    ) {
                        (_, _, _, _, x4, _, _) if x4 == 0 => {
                            let size = (instr >> 22) & 3;
                            let Zm = (instr >> 16) & 9;
                            let op = (instr >> 13) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Zda = instr & 9;
                            match op {
                                x0 if x0 == 0 => Some(Op::MlaZPZzz),
                                x0 if x0 == 1 => Some(Op::MlsZPZzz),
                                _ => None,
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
                                x0 if x0 == 0 => Some(Op::MadZPZzz),
                                x0 if x0 == 1 => Some(Op::MsbZPZzz),
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 0 && x2 & 2 == 0 && x4 & 16 == 0 && x6 & 56 == 0 =>
                {
                    match (
                        (instr >> 24) & 15,
                        (instr >> 22) & 3,
                        (instr >> 21) & 1,
                        (instr >> 18) & 5,
                        (instr >> 16) & 3,
                        (instr >> 13) & 5,
                        instr & 25,
                    ) {
                        (_, _, _, x3, _, _, _) if x3 & 6 == 0 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 5;
                            let Pg = (instr >> 10) & 5;
                            let Zm = (instr >> 5) & 9;
                            let Zdn = instr & 9;
                            match opc {
                                x0 if x0 == 0 => Some(Op::DdZPZz),
                                x0 if x0 == 1 => Some(Op::SubZPZz),
                                x0 if x0 == 2 => None,
                                x0 if x0 == 3 => Some(Op::SubrZPZz),
                                x0 if x0 & 4 == 4 => None,
                                _ => None,
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
                                (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::SmaxZPZz),
                                (x0, x1) if x0 == 0 && x1 == 1 => Some(Op::UmaxZPZz),
                                (x0, x1) if x0 == 1 && x1 == 0 => Some(Op::SminZPZz),
                                (x0, x1) if x0 == 1 && x1 == 1 => Some(Op::UminZPZz),
                                (x0, x1) if x0 == 2 && x1 == 0 => Some(Op::SabdZPZz),
                                (x0, x1) if x0 == 2 && x1 == 1 => Some(Op::UabdZPZz),
                                (x0, _) if x0 == 3 => None,
                                _ => None,
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
                                (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::MulZPZz),
                                (x0, x1) if x0 == 0 && x1 == 1 => None,
                                (x0, x1) if x0 == 1 && x1 == 0 => Some(Op::SmulhZPZz),
                                (x0, x1) if x0 == 1 && x1 == 1 => Some(Op::UmulhZPZz),
                                _ => None,
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
                                (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::SdivZPZz),
                                (x0, x1) if x0 == 0 && x1 == 1 => Some(Op::UdivZPZz),
                                (x0, x1) if x0 == 1 && x1 == 0 => Some(Op::SdivrZPZz),
                                (x0, x1) if x0 == 1 && x1 == 1 => Some(Op::UdivrZPZz),
                                _ => None,
                            }
                        }
                        (_, _, _, x3, _, _, _) if x3 & 6 == 6 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 5;
                            let Pg = (instr >> 10) & 5;
                            let Zm = (instr >> 5) & 9;
                            let Zdn = instr & 9;
                            match opc {
                                x0 if x0 == 0 => Some(Op::OrrZPZz),
                                x0 if x0 == 1 => Some(Op::EorZPZz),
                                x0 if x0 == 2 => Some(Op::NdZPZz),
                                x0 if x0 == 3 => Some(Op::BicZPZz),
                                x0 if x0 & 4 == 4 => None,
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 0 && x2 & 2 == 0 && x4 & 16 == 0 && x6 & 56 == 8 =>
                {
                    match (
                        (instr >> 24) & 15,
                        (instr >> 22) & 3,
                        (instr >> 21) & 1,
                        (instr >> 19) & 3,
                        (instr >> 16) & 5,
                        (instr >> 13) & 5,
                        instr & 25,
                    ) {
                        (_, _, _, x3, _, _, _) if x3 == 0 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 17) & 3;
                            let U = (instr >> 16) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Vd = instr & 9;
                            match (opc, U) {
                                (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::SaddvRPZ),
                                (x0, x1) if x0 == 0 && x1 == 1 => Some(Op::UaddvRPZ),
                                (x0, _) if x0 == 1 => None,
                                (x0, _) if x0 & 2 == 2 => None,
                                _ => None,
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
                                (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::SmaxvRPZ),
                                (x0, x1) if x0 == 0 && x1 == 1 => Some(Op::UmaxvRPZ),
                                (x0, x1) if x0 == 1 && x1 == 0 => Some(Op::SminvRPZ),
                                (x0, x1) if x0 == 1 && x1 == 1 => Some(Op::UminvRPZ),
                                (x0, _) if x0 & 2 == 2 => None,
                                _ => None,
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
                                x0 if x0 == 0 => Some(Op::MovprfxZPZ),
                                x0 if x0 == 1 => None,
                                x0 if x0 & 2 == 2 => None,
                                _ => None,
                            }
                        }
                        (_, _, _, x3, _, _, _) if x3 == 3 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 5;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Vd = instr & 9;
                            match opc {
                                x0 if x0 == 0 => Some(Op::OrvRPZ),
                                x0 if x0 == 1 => Some(Op::EorvRPZ),
                                x0 if x0 == 2 => Some(Op::NdvRPZ),
                                x0 if x0 == 3 => None,
                                x0 if x0 & 4 == 4 => None,
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 0 && x2 & 2 == 0 && x4 & 16 == 0 && x6 & 56 == 32 =>
                {
                    match (
                        (instr >> 24) & 15,
                        (instr >> 22) & 3,
                        (instr >> 21) & 1,
                        (instr >> 19) & 3,
                        (instr >> 16) & 5,
                        (instr >> 13) & 5,
                        instr & 25,
                    ) {
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
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => Some(Op::SrZPZi),
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => Some(Op::LsrZPZi),
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => None,
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => Some(Op::LslZPZi),
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => Some(Op::SrdZPZi),
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => None,
                                (x0, x1, _) if x0 == 1 && x1 == 1 => None,
                                (x0, _, _) if x0 & 2 == 2 => None,
                                _ => None,
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
                                (_, x1, x2) if x1 == 1 && x2 == 0 => None,
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => Some(Op::SrZPZz),
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => Some(Op::LsrZPZz),
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => Some(Op::LslZPZz),
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => Some(Op::SrrZPZz),
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => Some(Op::LsrrZPZz),
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => Some(Op::LslrZPZz),
                                _ => None,
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
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => Some(Op::SrZPZw),
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => Some(Op::LsrZPZw),
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => None,
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => Some(Op::LslZPZw),
                                (x0, _, _) if x0 == 1 => None,
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 0 && x2 & 2 == 0 && x4 & 16 == 0 && x6 & 56 == 40 =>
                {
                    match (
                        (instr >> 24) & 15,
                        (instr >> 22) & 3,
                        (instr >> 21) & 1,
                        (instr >> 19) & 3,
                        (instr >> 16) & 5,
                        (instr >> 13) & 5,
                        instr & 25,
                    ) {
                        (_, _, _, x3, _, _, _) if x3 & 2 == 0 => None,
                        (_, _, _, x3, _, _, _) if x3 == 2 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 5;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match opc {
                                x0 if x0 == 0 => Some(Op::SxtbZPZ),
                                x0 if x0 == 1 => Some(Op::UxtbZPZ),
                                x0 if x0 == 2 => Some(Op::SxthZPZ),
                                x0 if x0 == 3 => Some(Op::UxthZPZ),
                                x0 if x0 == 4 => Some(Op::SxtwZPZ),
                                x0 if x0 == 5 => Some(Op::UxtwZPZ),
                                x0 if x0 == 6 => Some(Op::BsZPZ),
                                x0 if x0 == 7 => Some(Op::NegZPZ),
                                _ => None,
                            }
                        }
                        (_, _, _, x3, _, _, _) if x3 == 3 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 5;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match opc {
                                x0 if x0 == 0 => Some(Op::LsZPZ),
                                x0 if x0 == 1 => Some(Op::LzZPZ),
                                x0 if x0 == 2 => Some(Op::NtZPZ),
                                x0 if x0 == 3 => Some(Op::NotZPZ),
                                x0 if x0 == 4 => Some(Op::FabsZPZ),
                                x0 if x0 == 5 => Some(Op::FnegZPZ),
                                x0 if x0 == 6 => Some(Op::NotZPZ),
                                x0 if x0 == 7 => None,
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 0 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 56 == 0 =>
                {
                    let size = (instr >> 22) & 3;
                    let Zm = (instr >> 16) & 9;
                    let opc = (instr >> 10) & 5;
                    let Zn = (instr >> 5) & 9;
                    let Zd = instr & 9;
                    match opc {
                        x0 if x0 == 0 => Some(Op::DdZZz),
                        x0 if x0 == 1 => Some(Op::SubZZz),
                        x0 if x0 & 6 == 2 => None,
                        x0 if x0 == 4 => Some(Op::SqaddZZz),
                        x0 if x0 == 5 => Some(Op::UqaddZZz),
                        x0 if x0 == 6 => Some(Op::SqsubZZz),
                        x0 if x0 == 7 => Some(Op::UqsubZZz),
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 0 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 56 == 8 =>
                {
                    match (
                        (instr >> 24) & 15,
                        (instr >> 22) & 3,
                        (instr >> 21) & 1,
                        (instr >> 16) & 9,
                        (instr >> 13) & 5,
                        (instr >> 12) & 1,
                        (instr >> 10) & 3,
                        instr & 19,
                    ) {
                        (_, _, _, _, _, x5, _, _) if x5 == 0 => None,
                        (_, _, _, _, _, x5, x6, _) if x5 == 1 && x6 == 0 => {
                            let opc = (instr >> 22) & 3;
                            let Zm = (instr >> 16) & 9;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match opc {
                                x0 if x0 == 0 => Some(Op::NdZZz),
                                x0 if x0 == 1 => Some(Op::OrrZZz),
                                x0 if x0 == 2 => Some(Op::EorZZz),
                                x0 if x0 == 3 => Some(Op::BicZZz),
                                _ => None,
                            }
                        }
                        (_, _, _, _, _, x5, x6, _) if x5 == 1 && x6 != 0 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 0 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 60 == 16 =>
                {
                    match (
                        (instr >> 24) & 15,
                        (instr >> 22) & 3,
                        (instr >> 21) & 1,
                        (instr >> 16) & 9,
                        (instr >> 12) & 7,
                        (instr >> 10) & 3,
                        instr & 19,
                    ) {
                        (_, _, _, _, _, x5, _) if x5 == 0 => {
                            let size = (instr >> 22) & 3;
                            let imm5b = (instr >> 16) & 9;
                            let imm5 = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match () {
                                () => Some(Op::IndexZIi),
                            }
                        }
                        (_, _, _, _, _, x5, _) if x5 == 1 => {
                            let size = (instr >> 22) & 3;
                            let imm5 = (instr >> 16) & 9;
                            let Rn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match () {
                                () => Some(Op::IndexZRi),
                            }
                        }
                        (_, _, _, _, _, x5, _) if x5 == 2 => {
                            let size = (instr >> 22) & 3;
                            let Rm = (instr >> 16) & 9;
                            let imm5 = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match () {
                                () => Some(Op::IndexZIr),
                            }
                        }
                        (_, _, _, _, _, x5, _) if x5 == 3 => {
                            let size = (instr >> 22) & 3;
                            let Rm = (instr >> 16) & 9;
                            let Rn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match () {
                                () => Some(Op::IndexZRr),
                            }
                        }
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 0 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 60 == 20 =>
                {
                    match (
                        (instr >> 24) & 15,
                        (instr >> 23) & 1,
                        (instr >> 22) & 1,
                        (instr >> 21) & 1,
                        (instr >> 16) & 9,
                        (instr >> 12) & 7,
                        (instr >> 11) & 1,
                        instr & 21,
                    ) {
                        (_, x1, _, _, _, _, x6, _) if x1 == 0 && x6 == 0 => {
                            let op = (instr >> 22) & 1;
                            let Rn = (instr >> 16) & 9;
                            let imm6 = (instr >> 5) & 11;
                            let Rd = instr & 9;
                            match op {
                                x0 if x0 == 0 => Some(Op::DdvlRRi),
                                x0 if x0 == 1 => Some(Op::DdplRRi),
                                _ => None,
                            }
                        }
                        (_, x1, _, _, _, _, x6, _) if x1 == 1 && x6 == 0 => {
                            let op = (instr >> 22) & 1;
                            let opc2 = (instr >> 16) & 9;
                            let imm6 = (instr >> 5) & 11;
                            let Rd = instr & 9;
                            match (op, opc2) {
                                (x0, x1) if x0 == 0 && x1 & 16 == 0 => None,
                                (x0, x1) if x0 == 0 && x1 & 24 == 16 => None,
                                (x0, x1) if x0 == 0 && x1 & 28 == 24 => None,
                                (x0, x1) if x0 == 0 && x1 & 30 == 28 => None,
                                (x0, x1) if x0 == 0 && x1 == 30 => None,
                                (x0, x1) if x0 == 0 && x1 == 31 => Some(Op::DvlRI),
                                (x0, _) if x0 == 1 => None,
                                _ => None,
                            }
                        }
                        (_, _, _, _, _, _, x6, _) if x6 == 1 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 0 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 56 == 24 =>
                {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 0 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 56 == 32 =>
                {
                    match (
                        (instr >> 24) & 15,
                        (instr >> 22) & 3,
                        (instr >> 21) & 1,
                        (instr >> 16) & 9,
                        (instr >> 13) & 5,
                        (instr >> 12) & 1,
                        instr & 23,
                    ) {
                        (_, _, _, _, _, x5, _) if x5 == 0 => {
                            let size = (instr >> 22) & 3;
                            let Zm = (instr >> 16) & 9;
                            let opc = (instr >> 10) & 3;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match opc {
                                x0 if x0 == 0 => Some(Op::SrZZw),
                                x0 if x0 == 1 => Some(Op::LsrZZw),
                                x0 if x0 == 2 => None,
                                x0 if x0 == 3 => Some(Op::LslZZw),
                                _ => None,
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
                                x0 if x0 == 0 => Some(Op::SrZZi),
                                x0 if x0 == 1 => Some(Op::LsrZZi),
                                x0 if x0 == 2 => None,
                                x0 if x0 == 3 => Some(Op::LslZZi),
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 0 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 60 == 40 =>
                {
                    let opc = (instr >> 22) & 3;
                    let Zm = (instr >> 16) & 9;
                    let msz = (instr >> 10) & 3;
                    let Zn = (instr >> 5) & 9;
                    let Zd = instr & 9;
                    match opc {
                        x0 if x0 == 0 => Some(Op::DrZAzDS32Scaled),
                        x0 if x0 == 1 => Some(Op::DrZAzDU32Scaled),
                        x0 if x0 & 2 == 2 => Some(Op::DrZAzSdSameScaled),
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 0 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 60 == 44 =>
                {
                    match (
                        (instr >> 24) & 15,
                        (instr >> 22) & 3,
                        (instr >> 21) & 1,
                        (instr >> 16) & 9,
                        (instr >> 12) & 7,
                        (instr >> 10) & 3,
                        instr & 19,
                    ) {
                        (_, _, _, _, _, x5, _) if x5 & 2 == 0 => {
                            let size = (instr >> 22) & 3;
                            let Zm = (instr >> 16) & 9;
                            let op = (instr >> 10) & 1;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match op {
                                x0 if x0 == 0 => Some(Op::FtsselZZz),
                                x0 if x0 == 1 => None,
                                _ => None,
                            }
                        }
                        (_, _, _, _, _, x5, _) if x5 == 2 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 9;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match opc {
                                x0 if x0 == 0 => Some(Op::FexpaZZ),
                                x0 if x0 == 1 => None,
                                x0 if x0 & 30 == 2 => None,
                                x0 if x0 & 28 == 4 => None,
                                x0 if x0 & 24 == 8 => None,
                                x0 if x0 & 16 == 16 => None,
                                _ => None,
                            }
                        }
                        (_, _, _, _, _, x5, _) if x5 == 3 => {
                            let opc = (instr >> 22) & 3;
                            let opc2 = (instr >> 16) & 9;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match (opc, opc2) {
                                (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::MovprfxZZ),
                                (x0, x1) if x0 == 0 && x1 == 1 => None,
                                (x0, x1) if x0 == 0 && x1 & 30 == 2 => None,
                                (x0, x1) if x0 == 0 && x1 & 28 == 4 => None,
                                (x0, x1) if x0 == 0 && x1 & 24 == 8 => None,
                                (x0, x1) if x0 == 0 && x1 & 16 == 16 => None,
                                (x0, _) if x0 == 1 => None,
                                (x0, _) if x0 & 2 == 2 => None,
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 0 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 48 == 48 =>
                {
                    match (
                        (instr >> 24) & 15,
                        (instr >> 22) & 3,
                        (instr >> 21) & 1,
                        (instr >> 20) & 1,
                        (instr >> 16) & 7,
                        (instr >> 14) & 3,
                        (instr >> 11) & 5,
                        instr & 21,
                    ) {
                        (_, _, _, x3, _, _, x6, _) if x3 == 0 && x6 & 6 == 0 => {
                            let size = (instr >> 22) & 3;
                            let imm4 = (instr >> 16) & 7;
                            let D = (instr >> 11) & 1;
                            let U = (instr >> 10) & 1;
                            let pattern = (instr >> 5) & 9;
                            let Zdn = instr & 9;
                            match (size, D, U) {
                                (x0, _, _) if x0 == 0 => None,
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    Some(Op::SqinchZZs)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    Some(Op::UqinchZZs)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    Some(Op::SqdechZZs)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    Some(Op::UqdechZZs)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                                    Some(Op::SqincwZZs)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                                    Some(Op::UqincwZZs)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                                    Some(Op::SqdecwZZs)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                                    Some(Op::UqdecwZZs)
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 0 => {
                                    Some(Op::SqincdZZs)
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 1 => {
                                    Some(Op::UqincdZZs)
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 0 => {
                                    Some(Op::SqdecdZZs)
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 1 => {
                                    Some(Op::UqdecdZZs)
                                }
                                _ => None,
                            }
                        }
                        (_, _, _, x3, _, _, x6, _) if x3 == 0 && x6 == 4 => {
                            let size = (instr >> 22) & 3;
                            let imm4 = (instr >> 16) & 7;
                            let op = (instr >> 10) & 1;
                            let pattern = (instr >> 5) & 9;
                            let Rd = instr & 9;
                            match (size, op) {
                                (_, x1) if x1 == 1 => None,
                                (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::NtbRS),
                                (x0, x1) if x0 == 1 && x1 == 0 => Some(Op::NthRS),
                                (x0, x1) if x0 == 2 && x1 == 0 => Some(Op::NtwRS),
                                (x0, x1) if x0 == 3 && x1 == 0 => Some(Op::NtdRS),
                                _ => None,
                            }
                        }
                        (_, _, _, x3, _, _, x6, _) if x3 == 0 && x6 == 5 => None,
                        (_, _, _, x3, _, _, x6, _) if x3 == 1 && x6 == 0 => {
                            let size = (instr >> 22) & 3;
                            let imm4 = (instr >> 16) & 7;
                            let D = (instr >> 10) & 1;
                            let pattern = (instr >> 5) & 9;
                            let Zdn = instr & 9;
                            match (size, D) {
                                (x0, _) if x0 == 0 => None,
                                (x0, x1) if x0 == 1 && x1 == 0 => Some(Op::InchZZs),
                                (x0, x1) if x0 == 1 && x1 == 1 => Some(Op::DechZZs),
                                (x0, x1) if x0 == 2 && x1 == 0 => Some(Op::IncwZZs),
                                (x0, x1) if x0 == 2 && x1 == 1 => Some(Op::DecwZZs),
                                (x0, x1) if x0 == 3 && x1 == 0 => Some(Op::IncdZZs),
                                (x0, x1) if x0 == 3 && x1 == 1 => Some(Op::DecdZZs),
                                _ => None,
                            }
                        }
                        (_, _, _, x3, _, _, x6, _) if x3 == 1 && x6 == 4 => {
                            let size = (instr >> 22) & 3;
                            let imm4 = (instr >> 16) & 7;
                            let D = (instr >> 10) & 1;
                            let pattern = (instr >> 5) & 9;
                            let Rdn = instr & 9;
                            match (size, D) {
                                (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::IncbRRs),
                                (x0, x1) if x0 == 0 && x1 == 1 => Some(Op::DecbRRs),
                                (x0, x1) if x0 == 1 && x1 == 0 => Some(Op::InchRRs),
                                (x0, x1) if x0 == 1 && x1 == 1 => Some(Op::DechRRs),
                                (x0, x1) if x0 == 2 && x1 == 0 => Some(Op::IncwRRs),
                                (x0, x1) if x0 == 2 && x1 == 1 => Some(Op::DecwRRs),
                                (x0, x1) if x0 == 3 && x1 == 0 => Some(Op::IncdRRs),
                                (x0, x1) if x0 == 3 && x1 == 1 => Some(Op::DecdRRs),
                                _ => None,
                            }
                        }
                        (_, _, _, x3, _, _, x6, _) if x3 == 1 && x6 & 3 == 1 => None,
                        (_, _, _, _, _, _, x6, _) if x6 & 6 == 2 => None,
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
                                    Some(Op::SqincbRRsSx)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 => {
                                    Some(Op::UqincbRRsUw)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => {
                                    Some(Op::SqdecbRRsSx)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 => {
                                    Some(Op::UqdecbRRsUw)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 0 => {
                                    Some(Op::SqincbRRsX)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 1 => {
                                    Some(Op::UqincbRRsX)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 0 => {
                                    Some(Op::SqdecbRRsX)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 1 => {
                                    Some(Op::UqdecbRRsX)
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 => {
                                    Some(Op::SqinchRRsSx)
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 => {
                                    Some(Op::UqinchRRsUw)
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 => {
                                    Some(Op::SqdechRRsSx)
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 => {
                                    Some(Op::UqdechRRsUw)
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 0 => {
                                    Some(Op::SqinchRRsX)
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 1 => {
                                    Some(Op::UqinchRRsX)
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 0 => {
                                    Some(Op::SqdechRRsX)
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 1 => {
                                    Some(Op::UqdechRRsX)
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 => {
                                    Some(Op::SqincwRRsSx)
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 => {
                                    Some(Op::UqincwRRsUw)
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 => {
                                    Some(Op::SqdecwRRsSx)
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 => {
                                    Some(Op::UqdecwRRsUw)
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 1 && x2 == 0 && x3 == 0 => {
                                    Some(Op::SqincwRRsX)
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 1 && x2 == 0 && x3 == 1 => {
                                    Some(Op::UqincwRRsX)
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 1 && x2 == 1 && x3 == 0 => {
                                    Some(Op::SqdecwRRsX)
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 1 && x2 == 1 && x3 == 1 => {
                                    Some(Op::UqdecwRRsX)
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 => {
                                    Some(Op::SqincdRRsSx)
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 => {
                                    Some(Op::UqincdRRsUw)
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 => {
                                    Some(Op::SqdecdRRsSx)
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 => {
                                    Some(Op::UqdecdRRsUw)
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 1 && x2 == 0 && x3 == 0 => {
                                    Some(Op::SqincdRRsX)
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 1 && x2 == 0 && x3 == 1 => {
                                    Some(Op::UqincdRRsX)
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 1 && x2 == 1 && x3 == 0 => {
                                    Some(Op::SqdecdRRsX)
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 1 && x2 == 1 && x3 == 1 => {
                                    Some(Op::UqdecdRRsX)
                                }
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, _, _) if x0 == 0 && x2 & 2 == 2 && x4 & 24 == 0 => {
                    match (
                        (instr >> 24) & 15,
                        (instr >> 22) & 3,
                        (instr >> 20) & 3,
                        (instr >> 18) & 3,
                        instr & 35,
                    ) {
                        (_, x1, _, x3, _) if x1 == 3 && x3 == 0 => {
                            let imm13 = (instr >> 5) & 25;
                            let Zd = instr & 9;
                            match () {
                                () => Some(Op::DupmZI),
                            }
                        }
                        (_, x1, _, x3, _) if x1 != 3 && x3 == 0 => {
                            let opc = (instr >> 22) & 3;
                            let imm13 = (instr >> 5) & 25;
                            let Zdn = instr & 9;
                            match opc {
                                x0 if x0 == 0 => Some(Op::OrrZZi),
                                x0 if x0 == 1 => Some(Op::EorZZi),
                                x0 if x0 == 2 => Some(Op::NdZZi),
                                _ => None,
                            }
                        }
                        (_, _, _, x3, _) if x3 != 0 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, _, _) if x0 == 0 && x2 & 2 == 2 && x4 & 24 == 8 => {
                    match (
                        (instr >> 24) & 15,
                        (instr >> 22) & 3,
                        (instr >> 20) & 3,
                        (instr >> 16) & 7,
                        (instr >> 13) & 5,
                        instr & 25,
                    ) {
                        (_, _, _, _, x4, _) if x4 & 4 == 0 => {
                            let size = (instr >> 22) & 3;
                            let Pg = (instr >> 16) & 7;
                            let M = (instr >> 14) & 1;
                            let sh = (instr >> 13) & 1;
                            let imm8 = (instr >> 5) & 15;
                            let Zd = instr & 9;
                            match M {
                                x0 if x0 == 0 => Some(Op::PyZOI),
                                x0 if x0 == 1 => Some(Op::PyZPI),
                                _ => None,
                            }
                        }
                        (_, _, _, _, x4, _) if x4 & 6 == 4 => None,
                        (_, _, _, _, x4, _) if x4 == 6 => {
                            let size = (instr >> 22) & 3;
                            let Pg = (instr >> 16) & 7;
                            let imm8 = (instr >> 5) & 15;
                            let Zd = instr & 9;
                            match () {
                                () => Some(Op::FcpyZPI),
                            }
                        }
                        (_, _, _, _, x4, _) if x4 == 7 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 0 && x2 & 2 == 2 && x4 & 16 == 16 && x6 & 56 == 8 =>
                {
                    match (
                        (instr >> 24) & 15,
                        (instr >> 22) & 3,
                        (instr >> 21) & 1,
                        (instr >> 19) & 3,
                        (instr >> 17) & 3,
                        (instr >> 16) & 1,
                        (instr >> 13) & 5,
                        (instr >> 12) & 1,
                        (instr >> 10) & 3,
                        instr & 19,
                    ) {
                        (_, _, _, x3, x4, x5, _, x7, x8, _)
                            if x3 == 0 && x4 == 0 && x5 == 0 && x7 == 1 && x8 == 2 =>
                        {
                            let size = (instr >> 22) & 3;
                            let Rn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match () {
                                () => Some(Op::DupZR),
                            }
                        }
                        (_, _, _, x3, x4, x5, _, x7, x8, _)
                            if x3 == 0 && x4 == 2 && x5 == 0 && x7 == 1 && x8 == 2 =>
                        {
                            let size = (instr >> 22) & 3;
                            let Rm = (instr >> 5) & 9;
                            let Zdn = instr & 9;
                            match () {
                                () => Some(Op::InsrZR),
                            }
                        }
                        (_, _, _, x3, x4, x5, _, x7, x8, _)
                            if x3 == 0 && x4 & 1 == 0 && x5 == 0 && x7 == 0 && x8 == 1 =>
                        {
                            None
                        }
                        (_, _, _, x3, x4, x5, _, x7, x8, _)
                            if x3 == 0 && x4 & 1 == 0 && x5 == 0 && x7 == 1 && x8 & 1 == 1 =>
                        {
                            None
                        }
                        (_, _, _, x3, x4, _, _, x7, x8, _)
                            if x3 == 0 && x4 & 1 == 1 && x7 == 1 && x8 & 2 == 2 =>
                        {
                            None
                        }
                        (_, _, _, x3, x4, _, _, _, x8, _) if x3 == 0 && x4 & 1 == 1 && x8 == 1 => {
                            None
                        }
                        (_, _, _, x3, _, x5, _, x7, x8, _)
                            if x3 == 0 && x5 == 1 && x7 == 1 && x8 & 2 == 2 =>
                        {
                            None
                        }
                        (_, _, _, x3, _, x5, _, _, x8, _) if x3 == 0 && x5 == 1 && x8 == 1 => None,
                        (_, _, _, x3, _, _, _, x7, x8, _) if x3 == 0 && x7 == 0 && x8 & 2 == 2 => {
                            None
                        }
                        (_, _, _, x3, _, _, _, _, x8, _) if x3 == 1 && x8 != 0 => None,
                        (_, _, _, x3, x4, _, _, x7, x8, _)
                            if x3 == 2 && x4 & 2 == 0 && x7 == 0 && x8 == 1 =>
                        {
                            None
                        }
                        (_, _, _, x3, x4, _, _, x7, x8, _)
                            if x3 == 2 && x4 & 2 == 0 && x7 == 1 && x8 == 2 =>
                        {
                            let size = (instr >> 22) & 3;
                            let U = (instr >> 17) & 1;
                            let H = (instr >> 16) & 1;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match (U, H) {
                                (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::SunpkloZZ),
                                (x0, x1) if x0 == 0 && x1 == 1 => Some(Op::SunpkhiZZ),
                                (x0, x1) if x0 == 1 && x1 == 0 => Some(Op::UunpkloZZ),
                                (x0, x1) if x0 == 1 && x1 == 1 => Some(Op::UunpkhiZZ),
                                _ => None,
                            }
                        }
                        (_, _, _, x3, x4, _, _, x7, x8, _)
                            if x3 == 2 && x4 & 2 == 0 && x7 == 1 && x8 & 1 == 1 =>
                        {
                            None
                        }
                        (_, _, _, x3, x4, x5, _, x7, x8, _)
                            if x3 == 2 && x4 == 2 && x5 == 0 && x7 == 0 && x8 == 1 =>
                        {
                            None
                        }
                        (_, _, _, x3, x4, x5, _, x7, x8, _)
                            if x3 == 2 && x4 == 2 && x5 == 0 && x7 == 1 && x8 == 2 =>
                        {
                            let size = (instr >> 22) & 3;
                            let Vm = (instr >> 5) & 9;
                            let Zdn = instr & 9;
                            match () {
                                () => Some(Op::InsrZV),
                            }
                        }
                        (_, _, _, x3, x4, x5, _, x7, x8, _)
                            if x3 == 2 && x4 == 2 && x5 == 0 && x7 == 1 && x8 & 1 == 1 =>
                        {
                            None
                        }
                        (_, _, _, x3, x4, _, _, x7, x8, _)
                            if x3 == 2 && x4 == 3 && x7 == 1 && x8 & 2 == 2 =>
                        {
                            None
                        }
                        (_, _, _, x3, x4, _, _, _, x8, _) if x3 == 2 && x4 == 3 && x8 == 1 => None,
                        (_, _, _, x3, x4, x5, _, x7, x8, _)
                            if x3 == 2 && x4 & 2 == 2 && x5 == 1 && x7 == 1 && x8 & 2 == 2 =>
                        {
                            None
                        }
                        (_, _, _, x3, x4, x5, _, _, x8, _)
                            if x3 == 2 && x4 & 2 == 2 && x5 == 1 && x8 == 1 =>
                        {
                            None
                        }
                        (_, _, _, x3, x4, x5, _, x7, x8, _)
                            if x3 == 3 && x4 == 0 && x5 == 0 && x7 == 0 && x8 == 1 =>
                        {
                            None
                        }
                        (_, _, _, x3, x4, x5, _, x7, x8, _)
                            if x3 == 3 && x4 == 0 && x5 == 0 && x7 == 1 && x8 == 2 =>
                        {
                            let size = (instr >> 22) & 3;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match () {
                                () => Some(Op::EvZZ),
                            }
                        }
                        (_, _, _, x3, x4, x5, _, x7, x8, _)
                            if x3 == 3 && x4 == 0 && x5 == 0 && x7 == 1 && x8 & 1 == 1 =>
                        {
                            None
                        }
                        (_, _, _, x3, x4, x5, _, x7, x8, _)
                            if x3 == 3 && x4 & 2 == 0 && x5 == 1 && x7 == 1 && x8 & 2 == 2 =>
                        {
                            None
                        }
                        (_, _, _, x3, x4, x5, _, _, x8, _)
                            if x3 == 3 && x4 & 2 == 0 && x5 == 1 && x8 == 1 =>
                        {
                            None
                        }
                        (_, _, _, x3, x4, _, _, x7, x8, _)
                            if x3 == 3 && x4 != 0 && x7 == 1 && x8 & 2 == 2 =>
                        {
                            None
                        }
                        (_, _, _, x3, x4, _, _, _, x8, _) if x3 == 3 && x4 != 0 && x8 == 1 => None,
                        (_, _, _, x3, _, _, _, x7, x8, _)
                            if x3 & 2 == 2 && x7 == 0 && x8 & 2 == 2 =>
                        {
                            None
                        }
                        (_, _, _, _, _, _, _, x7, x8, _) if x7 == 0 && x8 == 0 => {
                            let imm2 = (instr >> 22) & 3;
                            let tsz = (instr >> 16) & 9;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match () {
                                () => Some(Op::DupZZi),
                            }
                        }
                        (_, _, _, _, _, _, _, x7, x8, _) if x7 == 1 && x8 == 0 => {
                            let size = (instr >> 22) & 3;
                            let Zm = (instr >> 16) & 9;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match () {
                                () => Some(Op::TblZZz1),
                            }
                        }
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 0 && x2 & 2 == 2 && x4 & 16 == 16 && x6 & 56 == 16 =>
                {
                    match (
                        (instr >> 24) & 15,
                        (instr >> 22) & 3,
                        (instr >> 21) & 1,
                        (instr >> 16) & 9,
                        (instr >> 13) & 5,
                        (instr >> 9) & 7,
                        (instr >> 5) & 7,
                        (instr >> 4) & 1,
                        instr & 7,
                    ) {
                        (_, x1, _, x3, _, x5, _, x7, _)
                            if x1 == 0 && x3 & 30 == 16 && x5 == 0 && x7 == 0 =>
                        {
                            let H = (instr >> 16) & 1;
                            let Pn = (instr >> 5) & 7;
                            let Pd = instr & 7;
                            match H {
                                x0 if x0 == 0 => Some(Op::PunpkloPP),
                                x0 if x0 == 1 => Some(Op::PunpkhiPP),
                                _ => None,
                            }
                        }
                        (_, x1, _, x3, _, x5, _, x7, _)
                            if x1 == 1 && x3 & 30 == 16 && x5 == 0 && x7 == 0 =>
                        {
                            None
                        }
                        (_, x1, _, x3, _, x5, _, x7, _)
                            if x1 == 2 && x3 & 30 == 16 && x5 == 0 && x7 == 0 =>
                        {
                            None
                        }
                        (_, x1, _, x3, _, x5, _, x7, _)
                            if x1 == 3 && x3 & 30 == 16 && x5 == 0 && x7 == 0 =>
                        {
                            None
                        }
                        (_, _, _, x3, _, x5, _, x7, _)
                            if x3 & 16 == 0 && x5 & 1 == 0 && x7 == 0 =>
                        {
                            let size = (instr >> 22) & 3;
                            let Pm = (instr >> 16) & 7;
                            let opc = (instr >> 11) & 3;
                            let H = (instr >> 10) & 1;
                            let Pn = (instr >> 5) & 7;
                            let Pd = instr & 7;
                            match (opc, H) {
                                (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::Zip1PPp),
                                (x0, x1) if x0 == 0 && x1 == 1 => Some(Op::Zip2PPp),
                                (x0, x1) if x0 == 1 && x1 == 0 => Some(Op::Uzp1PPp),
                                (x0, x1) if x0 == 1 && x1 == 1 => Some(Op::Uzp2PPp),
                                (x0, x1) if x0 == 2 && x1 == 0 => Some(Op::Trn1PPp),
                                (x0, x1) if x0 == 2 && x1 == 1 => Some(Op::Trn2PPp),
                                (x0, _) if x0 == 3 => None,
                                _ => None,
                            }
                        }
                        (_, _, _, x3, _, x5, _, x7, _)
                            if x3 & 16 == 0 && x5 & 1 == 1 && x7 == 0 =>
                        {
                            None
                        }
                        (_, _, _, x3, _, x5, _, x7, _) if x3 == 20 && x5 == 0 && x7 == 0 => {
                            let size = (instr >> 22) & 3;
                            let Pn = (instr >> 5) & 7;
                            let Pd = instr & 7;
                            match () {
                                () => Some(Op::EvPP),
                            }
                        }
                        (_, _, _, x3, _, x5, _, x7, _) if x3 == 21 && x5 == 0 && x7 == 0 => None,
                        (_, _, _, x3, _, x5, _, x7, _) if x3 & 26 == 16 && x5 == 8 && x7 == 0 => {
                            None
                        }
                        (_, _, _, x3, _, x5, _, x7, _)
                            if x3 & 26 == 16 && x5 & 7 == 4 && x7 == 0 =>
                        {
                            None
                        }
                        (_, _, _, x3, _, x5, _, x7, _)
                            if x3 & 26 == 16 && x5 & 3 == 2 && x7 == 0 =>
                        {
                            None
                        }
                        (_, _, _, x3, _, x5, _, x7, _)
                            if x3 & 26 == 16 && x5 & 1 == 1 && x7 == 0 =>
                        {
                            None
                        }
                        (_, _, _, x3, _, _, _, x7, _) if x3 & 26 == 18 && x7 == 0 => None,
                        (_, _, _, x3, _, _, _, x7, _) if x3 & 24 == 24 && x7 == 0 => None,
                        (_, _, _, _, _, _, _, x7, _) if x7 == 1 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 0 && x2 & 2 == 2 && x4 & 16 == 16 && x6 & 56 == 24 =>
                {
                    let size = (instr >> 22) & 3;
                    let Zm = (instr >> 16) & 9;
                    let opc = (instr >> 10) & 5;
                    let Zn = (instr >> 5) & 9;
                    let Zd = instr & 9;
                    match opc {
                        x0 if x0 == 0 => Some(Op::Zip1ZZz),
                        x0 if x0 == 1 => Some(Op::Zip2ZZz),
                        x0 if x0 == 2 => Some(Op::Uzp1ZZz),
                        x0 if x0 == 3 => Some(Op::Uzp2ZZz),
                        x0 if x0 == 4 => Some(Op::Trn1ZZz),
                        x0 if x0 == 5 => Some(Op::Trn2ZZz),
                        x0 if x0 & 6 == 6 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 0 && x2 & 2 == 2 && x4 & 16 == 16 && x6 & 48 == 32 =>
                {
                    match (
                        (instr >> 24) & 15,
                        (instr >> 22) & 3,
                        (instr >> 21) & 1,
                        (instr >> 20) & 1,
                        (instr >> 17) & 5,
                        (instr >> 16) & 1,
                        (instr >> 14) & 3,
                        (instr >> 13) & 1,
                        instr & 25,
                    ) {
                        (_, _, _, x3, x4, x5, _, x7, _)
                            if x3 == 0 && x4 == 0 && x5 == 0 && x7 == 0 =>
                        {
                            let size = (instr >> 22) & 3;
                            let Pg = (instr >> 10) & 5;
                            let Vn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match () {
                                () => Some(Op::PyZPV),
                            }
                        }
                        (_, _, _, x3, x4, x5, _, x7, _)
                            if x3 == 0 && x4 == 0 && x5 == 1 && x7 == 0 =>
                        {
                            let size = (instr >> 22) & 3;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match () {
                                () => Some(Op::OmpactZPZ),
                            }
                        }
                        (_, _, _, x3, x4, _, _, x7, _) if x3 == 0 && x4 == 0 && x7 == 1 => {
                            let size = (instr >> 22) & 3;
                            let B = (instr >> 16) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Rd = instr & 9;
                            match B {
                                x0 if x0 == 0 => Some(Op::LastaRPZ),
                                x0 if x0 == 1 => Some(Op::LastbRPZ),
                                _ => None,
                            }
                        }
                        (_, _, _, x3, x4, _, _, x7, _) if x3 == 0 && x4 == 1 && x7 == 0 => {
                            let size = (instr >> 22) & 3;
                            let B = (instr >> 16) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Vd = instr & 9;
                            match B {
                                x0 if x0 == 0 => Some(Op::LastaVPZ),
                                x0 if x0 == 1 => Some(Op::LastbVPZ),
                                _ => None,
                            }
                        }
                        (_, _, _, x3, x4, _, _, x7, _) if x3 == 0 && x4 & 6 == 2 && x7 == 0 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 3;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match opc {
                                x0 if x0 == 0 => Some(Op::EvbZZ),
                                x0 if x0 == 1 => Some(Op::EvhZZ),
                                x0 if x0 == 2 => Some(Op::EvwZZ),
                                x0 if x0 == 3 => Some(Op::BitZPZ),
                                _ => None,
                            }
                        }
                        (_, _, _, x3, x4, _, _, x7, _) if x3 == 0 && x4 & 6 == 2 && x7 == 1 => None,
                        (_, _, _, x3, x4, x5, _, x7, _)
                            if x3 == 0 && x4 == 4 && x5 == 0 && x7 == 1 =>
                        {
                            let size = (instr >> 22) & 3;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match () {
                                () => Some(Op::PyZPR),
                            }
                        }
                        (_, _, _, x3, x4, x5, _, x7, _)
                            if x3 == 0 && x4 == 4 && x5 == 1 && x7 == 1 =>
                        {
                            None
                        }
                        (_, _, _, x3, x4, _, _, x7, _) if x3 == 0 && x4 == 4 && x7 == 0 => {
                            let size = (instr >> 22) & 3;
                            let B = (instr >> 16) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Zm = (instr >> 5) & 9;
                            let Zdn = instr & 9;
                            match B {
                                x0 if x0 == 0 => Some(Op::LastaZPZz),
                                x0 if x0 == 1 => Some(Op::LastbZPZz),
                                _ => None,
                            }
                        }
                        (_, _, _, x3, x4, _, _, x7, _) if x3 == 0 && x4 == 5 && x7 == 0 => {
                            let size = (instr >> 22) & 3;
                            let B = (instr >> 16) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Zm = (instr >> 5) & 9;
                            let Vdn = instr & 9;
                            match B {
                                x0 if x0 == 0 => Some(Op::LastaVPZ),
                                x0 if x0 == 1 => Some(Op::LastbVPZ),
                                _ => None,
                            }
                        }
                        (_, _, _, x3, x4, x5, _, x7, _)
                            if x3 == 0 && x4 == 6 && x5 == 0 && x7 == 0 =>
                        {
                            let size = (instr >> 22) & 3;
                            let Pg = (instr >> 10) & 5;
                            let Zm = (instr >> 5) & 9;
                            let Zdn = instr & 9;
                            match () {
                                () => Some(Op::SpliceZPZzDes),
                            }
                        }
                        (_, _, _, x3, x4, x5, _, x7, _)
                            if x3 == 0 && x4 == 6 && x5 == 0 && x7 == 1 =>
                        {
                            None
                        }
                        (_, _, _, x3, x4, x5, _, _, _) if x3 == 0 && x4 == 6 && x5 == 1 => None,
                        (_, _, _, x3, x4, x5, _, _, _) if x3 == 0 && x4 == 7 && x5 == 0 => None,
                        (_, _, _, x3, x4, x5, _, _, _) if x3 == 0 && x4 == 7 && x5 == 1 => None,
                        (_, _, _, x3, x4, _, _, x7, _) if x3 == 0 && x4 & 3 == 1 && x7 == 1 => None,
                        (_, _, _, x3, x4, _, _, x7, _) if x3 == 1 && x4 == 0 && x7 == 0 => None,
                        (_, _, _, x3, x4, _, _, x7, _) if x3 == 1 && x4 == 0 && x7 == 1 => {
                            let size = (instr >> 22) & 3;
                            let B = (instr >> 16) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Zm = (instr >> 5) & 9;
                            let Rdn = instr & 9;
                            match B {
                                x0 if x0 == 0 => Some(Op::LastaRPZ),
                                x0 if x0 == 1 => Some(Op::LastbRPZ),
                                _ => None,
                            }
                        }
                        (_, _, _, x3, x4, _, _, _, _) if x3 == 1 && x4 != 0 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 0 && x2 & 2 == 2 && x4 & 16 == 16 && x6 & 48 == 48 =>
                {
                    let size = (instr >> 22) & 3;
                    let Zm = (instr >> 16) & 9;
                    let Pg = (instr >> 10) & 7;
                    let Zn = (instr >> 5) & 9;
                    let Zd = instr & 9;
                    match () {
                        () => Some(Op::SelZPZz),
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 0 && x2 == 2 && x4 & 16 == 16 && x6 & 56 == 0 =>
                {
                    match (
                        (instr >> 23) & 17,
                        (instr >> 22) & 1,
                        (instr >> 21) & 1,
                        (instr >> 16) & 9,
                        (instr >> 13) & 5,
                        instr & 25,
                    ) {
                        (_, x1, _, _, _, _) if x1 == 0 => {
                            let imm8h = (instr >> 16) & 9;
                            let imm8l = (instr >> 10) & 5;
                            let Zm = (instr >> 5) & 9;
                            let Zdn = instr & 9;
                            match () {
                                () => Some(Op::ExtZZiDes),
                            }
                        }
                        (_, x1, _, _, _, _) if x1 == 1 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 0 && x2 == 3 && x4 & 16 == 16 && x6 & 56 == 0 =>
                {
                    let op = (instr >> 22) & 1;
                    let Zm = (instr >> 16) & 9;
                    let opc2 = (instr >> 10) & 5;
                    let Zn = (instr >> 5) & 9;
                    let Zd = instr & 9;
                    match (op, opc2) {
                        (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::Zip1ZZzQ),
                        (x0, x1) if x0 == 0 && x1 == 1 => Some(Op::Zip2ZZzQ),
                        (x0, x1) if x0 == 0 && x1 == 2 => Some(Op::Uzp1ZZzQ),
                        (x0, x1) if x0 == 0 && x1 == 3 => Some(Op::Uzp2ZZzQ),
                        (x0, x1) if x0 == 0 && x1 & 6 == 4 => None,
                        (x0, x1) if x0 == 0 && x1 == 6 => Some(Op::Trn1ZZzQ),
                        (x0, x1) if x0 == 0 && x1 == 7 => Some(Op::Trn2ZZzQ),
                        (x0, _) if x0 == 1 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, _, _) if x0 == 1 && x2 & 2 == 0 && x4 & 16 == 0 => {
                    match (
                        (instr >> 24) & 15,
                        (instr >> 22) & 3,
                        (instr >> 21) & 1,
                        (instr >> 15) & 11,
                        (instr >> 14) & 1,
                        instr & 27,
                    ) {
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
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => Some(Op::MphsPPZz),
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => Some(Op::MphiPPZz),
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => Some(Op::MpeqPPZw),
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => Some(Op::MpnePPZw),
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => Some(Op::MpgePPZz),
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => Some(Op::MpgtPPZz),
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => Some(Op::MpeqPPZz),
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => Some(Op::MpnePPZz),
                                _ => None,
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
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => Some(Op::MpgePPZw),
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => Some(Op::MpgtPPZw),
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => Some(Op::MpltPPZw),
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => Some(Op::MplePPZw),
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => Some(Op::MphsPPZw),
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => Some(Op::MphiPPZw),
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => Some(Op::MploPPZw),
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => Some(Op::MplsPPZw),
                                _ => None,
                            }
                        }
                        _ => None,
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
                        (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::MphsPPZi),
                        (x0, x1) if x0 == 0 && x1 == 1 => Some(Op::MphiPPZi),
                        (x0, x1) if x0 == 1 && x1 == 0 => Some(Op::MploPPZi),
                        (x0, x1) if x0 == 1 && x1 == 1 => Some(Op::MplsPPZi),
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 1 && x2 & 2 == 2 && x4 & 16 == 0 && x6 & 16 == 0 =>
                {
                    let size = (instr >> 22) & 3;
                    let imm5 = (instr >> 16) & 9;
                    let op = (instr >> 15) & 1;
                    let o2 = (instr >> 13) & 1;
                    let Pg = (instr >> 10) & 5;
                    let Zn = (instr >> 5) & 9;
                    let ne = (instr >> 4) & 1;
                    let Pd = instr & 7;
                    match (op, o2, ne) {
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => Some(Op::MpgePPZi),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => Some(Op::MpgtPPZi),
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => Some(Op::MpltPPZi),
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => Some(Op::MplePPZi),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => Some(Op::MpeqPPZi),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => Some(Op::MpnePPZi),
                        (x0, x1, _) if x0 == 1 && x1 == 1 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 1 && x2 & 2 == 2 && x4 & 24 == 0 && x6 & 48 == 16 =>
                {
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
                            Some(Op::NdPPPpZ)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            Some(Op::BicPPPpZ)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            Some(Op::EorPPPpZ)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 => {
                            Some(Op::SelPPPp)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 0 => {
                            Some(Op::NdsPPPpZ)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 1 => {
                            Some(Op::BicsPPPpZ)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 0 => {
                            Some(Op::EorsPPPpZ)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 1 => None,
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            Some(Op::OrrPPPpZ)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            Some(Op::OrnPPPpZ)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            Some(Op::NorPPPpZ)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 => {
                            Some(Op::NandPPPpZ)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 0 => {
                            Some(Op::OrrsPPPpZ)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 1 => {
                            Some(Op::OrnsPPPpZ)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 0 => {
                            Some(Op::NorsPPPpZ)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 1 => {
                            Some(Op::NandsPPPpZ)
                        }
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 1 && x2 & 2 == 2 && x4 & 24 == 0 && x6 & 48 == 48 =>
                {
                    match (
                        (instr >> 24) & 15,
                        (instr >> 22) & 3,
                        (instr >> 20) & 3,
                        (instr >> 16) & 7,
                        (instr >> 14) & 3,
                        (instr >> 10) & 7,
                        (instr >> 9) & 1,
                        instr & 17,
                    ) {
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
                                    Some(Op::BrkpaPPPp)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                                    Some(Op::BrkpbPPPp)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    Some(Op::BrkpasPPPp)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    Some(Op::BrkpbsPPPp)
                                }
                                (x0, _, _) if x0 == 1 => None,
                                _ => None,
                            }
                        }
                        (_, _, _, _, _, _, x6, _) if x6 == 1 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 1 && x2 & 2 == 2 && x4 & 24 == 8 && x6 & 48 == 16 =>
                {
                    match (
                        (instr >> 24) & 15,
                        (instr >> 23) & 1,
                        (instr >> 22) & 1,
                        (instr >> 20) & 3,
                        (instr >> 16) & 7,
                        (instr >> 14) & 3,
                        (instr >> 10) & 7,
                        (instr >> 9) & 1,
                        (instr >> 5) & 7,
                        (instr >> 4) & 1,
                        instr & 7,
                    ) {
                        (_, x1, _, _, x4, _, _, x7, _, x9, _)
                            if x1 == 0 && x4 == 8 && x7 == 0 && x9 == 0 =>
                        {
                            let S = (instr >> 22) & 1;
                            let Pg = (instr >> 10) & 7;
                            let Pn = (instr >> 5) & 7;
                            let Pdm = instr & 7;
                            match S {
                                x0 if x0 == 0 => Some(Op::BrknPPPp),
                                x0 if x0 == 1 => Some(Op::BrknsPPPp),
                                _ => None,
                            }
                        }
                        (_, x1, _, _, x4, _, _, x7, _, x9, _)
                            if x1 == 0 && x4 == 8 && x7 == 0 && x9 == 1 =>
                        {
                            None
                        }
                        (_, x1, _, _, x4, _, _, x7, _, _, _)
                            if x1 == 0 && x4 & 7 == 0 && x7 == 1 =>
                        {
                            None
                        }
                        (_, x1, _, _, x4, _, _, _, _, _, _) if x1 == 0 && x4 & 4 == 4 => None,
                        (_, x1, _, _, x4, _, _, _, _, _, _) if x1 == 0 && x4 & 2 == 2 => None,
                        (_, x1, _, _, x4, _, _, _, _, _, _) if x1 == 0 && x4 & 1 == 1 => None,
                        (_, x1, _, _, x4, _, _, x7, _, _, _) if x1 == 1 && x4 == 0 && x7 == 1 => {
                            None
                        }
                        (_, x1, _, _, x4, _, _, _, _, _, _) if x1 == 1 && x4 != 0 => None,
                        (_, _, _, _, x4, _, _, x7, _, _, _) if x4 == 0 && x7 == 0 => {
                            let B = (instr >> 23) & 1;
                            let S = (instr >> 22) & 1;
                            let Pg = (instr >> 10) & 7;
                            let Pn = (instr >> 5) & 7;
                            let M = (instr >> 4) & 1;
                            let Pd = instr & 7;
                            match (B, S, M) {
                                (_, x1, x2) if x1 == 1 && x2 == 1 => None,
                                (x0, x1, _) if x0 == 0 && x1 == 0 => Some(Op::BrkaPPP),
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    Some(Op::BrkasPPPZ)
                                }
                                (x0, x1, _) if x0 == 1 && x1 == 0 => Some(Op::BrkbPPP),
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    Some(Op::BrkbsPPPZ)
                                }
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 1 && x2 & 2 == 2 && x4 & 24 == 8 && x6 & 48 == 48 =>
                {
                    match (
                        (instr >> 24) & 15,
                        (instr >> 22) & 3,
                        (instr >> 20) & 3,
                        (instr >> 16) & 7,
                        (instr >> 14) & 3,
                        (instr >> 11) & 5,
                        (instr >> 9) & 3,
                        (instr >> 5) & 7,
                        (instr >> 4) & 1,
                        instr & 7,
                    ) {
                        (_, _, _, x3, _, _, x6, _, x8, _) if x3 == 0 && x6 & 1 == 0 && x8 == 0 => {
                            let op = (instr >> 23) & 1;
                            let S = (instr >> 22) & 1;
                            let Pg = (instr >> 10) & 7;
                            let Pn = (instr >> 5) & 7;
                            let opc2 = instr & 7;
                            match (op, S, opc2) {
                                (x0, x1, _) if x0 == 0 && x1 == 0 => None,
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => Some(Op::PtestPP),
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => None,
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 & 14 == 2 => None,
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 & 12 == 4 => None,
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 & 8 == 8 => None,
                                (x0, _, _) if x0 == 1 => None,
                                _ => None,
                            }
                        }
                        (_, _, _, x3, _, _, x6, _, x8, _) if x3 == 4 && x6 & 1 == 0 && x8 == 0 => {
                            None
                        }
                        (_, _, _, x3, _, _, x6, _, x8, _)
                            if x3 & 11 == 2 && x6 & 1 == 0 && x8 == 0 =>
                        {
                            None
                        }
                        (_, _, _, x3, _, _, x6, _, x8, _)
                            if x3 & 9 == 1 && x6 & 1 == 0 && x8 == 0 =>
                        {
                            None
                        }
                        (_, _, _, x3, _, _, x6, _, x8, _)
                            if x3 & 8 == 0 && x6 & 1 == 1 && x8 == 0 =>
                        {
                            None
                        }
                        (_, _, _, x3, _, x5, x6, _, x8, _)
                            if x3 == 8 && x5 == 0 && x6 == 0 && x8 == 0 =>
                        {
                            let op = (instr >> 23) & 1;
                            let S = (instr >> 22) & 1;
                            let Pg = (instr >> 5) & 7;
                            let Pdn = instr & 7;
                            match (op, S) {
                                (x0, x1) if x0 == 0 && x1 == 0 => None,
                                (x0, x1) if x0 == 0 && x1 == 1 => Some(Op::PfirstPPP),
                                (x0, _) if x0 == 1 => None,
                                _ => None,
                            }
                        }
                        (_, _, _, x3, _, x5, x6, _, x8, _)
                            if x3 == 8 && x5 == 0 && x6 != 0 && x8 == 0 =>
                        {
                            None
                        }
                        (_, _, _, x3, _, x5, x6, x7, x8, _)
                            if x3 == 8 && x5 == 4 && x6 == 2 && x7 == 0 && x8 == 0 =>
                        {
                            let op = (instr >> 23) & 1;
                            let S = (instr >> 22) & 1;
                            let Pd = instr & 7;
                            match (op, S) {
                                (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::PfalseP),
                                (x0, x1) if x0 == 0 && x1 == 1 => None,
                                (x0, _) if x0 == 1 => None,
                                _ => None,
                            }
                        }
                        (_, _, _, x3, _, x5, x6, x7, x8, _)
                            if x3 == 8 && x5 == 4 && x6 == 2 && x7 != 0 && x8 == 0 =>
                        {
                            None
                        }
                        (_, _, _, x3, _, x5, x6, _, x8, _)
                            if x3 == 8 && x5 == 6 && x6 == 0 && x8 == 0 =>
                        {
                            let op = (instr >> 23) & 1;
                            let S = (instr >> 22) & 1;
                            let Pg = (instr >> 5) & 7;
                            let Pd = instr & 7;
                            match (op, S) {
                                (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::DffrPPF),
                                (x0, x1) if x0 == 0 && x1 == 1 => Some(Op::DffrsPPF),
                                (x0, _) if x0 == 1 => None,
                                _ => None,
                            }
                        }
                        (_, _, _, x3, _, x5, x6, _, x8, _)
                            if x3 == 9 && x5 == 0 && x6 & 2 == 0 && x8 == 0 =>
                        {
                            None
                        }
                        (_, _, _, x3, _, x5, x6, _, x8, _)
                            if x3 == 9 && x5 == 0 && x6 == 2 && x8 == 0 =>
                        {
                            let size = (instr >> 22) & 3;
                            let Pg = (instr >> 5) & 7;
                            let Pdn = instr & 7;
                            match () {
                                () => Some(Op::PnextPPP),
                            }
                        }
                        (_, _, _, x3, _, x5, x6, _, x8, _)
                            if x3 == 9 && x5 == 0 && x6 == 3 && x8 == 0 =>
                        {
                            None
                        }
                        (_, _, _, x3, _, x5, x6, _, x8, _)
                            if x3 == 9 && x5 == 4 && x6 == 2 && x8 == 0 =>
                        {
                            None
                        }
                        (_, _, _, x3, _, x5, x6, x7, x8, _)
                            if x3 == 9 && x5 == 6 && x6 == 0 && x7 == 0 && x8 == 0 =>
                        {
                            let op = (instr >> 23) & 1;
                            let S = (instr >> 22) & 1;
                            let Pd = instr & 7;
                            match (op, S) {
                                (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::DffrPF),
                                (x0, x1) if x0 == 0 && x1 == 1 => None,
                                (x0, _) if x0 == 1 => None,
                                _ => None,
                            }
                        }
                        (_, _, _, x3, _, x5, x6, x7, x8, _)
                            if x3 == 9 && x5 == 6 && x6 == 0 && x7 != 0 && x8 == 0 =>
                        {
                            None
                        }
                        (_, _, _, x3, _, x5, _, _, x8, _) if x3 & 14 == 8 && x5 == 2 && x8 == 0 => {
                            None
                        }
                        (_, _, _, x3, _, x5, x6, _, x8, _)
                            if x3 & 14 == 8 && x5 == 4 && x6 & 2 == 0 && x8 == 0 =>
                        {
                            let size = (instr >> 22) & 3;
                            let S = (instr >> 16) & 1;
                            let pattern = (instr >> 5) & 9;
                            let Pd = instr & 7;
                            match S {
                                x0 if x0 == 0 => Some(Op::PtruePS),
                                x0 if x0 == 1 => Some(Op::PtruesPS),
                                _ => None,
                            }
                        }
                        (_, _, _, x3, _, x5, x6, _, x8, _)
                            if x3 & 14 == 8 && x5 == 4 && x6 == 3 && x8 == 0 =>
                        {
                            None
                        }
                        (_, _, _, x3, _, x5, x6, _, x8, _)
                            if x3 & 14 == 8 && x5 == 6 && x6 != 0 && x8 == 0 =>
                        {
                            None
                        }
                        (_, _, _, x3, _, x5, _, _, x8, _)
                            if x3 & 14 == 8 && x5 & 1 == 1 && x8 == 0 =>
                        {
                            None
                        }
                        (_, _, _, x3, _, _, _, _, x8, _) if x3 & 14 == 12 && x8 == 0 => None,
                        (_, _, _, x3, _, _, _, _, x8, _) if x3 & 10 == 10 && x8 == 0 => None,
                        (_, _, _, _, _, _, _, _, x8, _) if x8 == 1 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 1 && x2 & 2 == 2 && x4 & 16 == 16 && x6 & 48 == 0 =>
                {
                    match (
                        (instr >> 24) & 15,
                        (instr >> 22) & 3,
                        (instr >> 21) & 1,
                        (instr >> 16) & 9,
                        (instr >> 14) & 3,
                        (instr >> 13) & 1,
                        (instr >> 10) & 5,
                        (instr >> 4) & 11,
                        instr & 7,
                    ) {
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
                                (_, x1, _) if x1 == 0 => None,
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    Some(Op::WhileltPPRr)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    Some(Op::WhilelePPRr)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    Some(Op::WhileloPPRr)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    Some(Op::WhilelsPPRr)
                                }
                                _ => None,
                            }
                        }
                        (_, _, _, _, _, x5, x6, _, x8) if x5 == 1 && x6 == 0 && x8 == 0 => {
                            let op = (instr >> 23) & 1;
                            let sz = (instr >> 22) & 1;
                            let Rm = (instr >> 16) & 9;
                            let Rn = (instr >> 5) & 9;
                            let ne = (instr >> 4) & 1;
                            match (op, ne) {
                                (x0, _) if x0 == 0 => None,
                                (x0, x1) if x0 == 1 && x1 == 0 => Some(Op::TermeqRr),
                                (x0, x1) if x0 == 1 && x1 == 1 => Some(Op::TermneRr),
                                _ => None,
                            }
                        }
                        (_, _, _, _, _, x5, x6, _, x8) if x5 == 1 && x6 == 0 && x8 != 0 => None,
                        (_, _, _, _, _, x5, x6, _, _) if x5 == 1 && x6 != 0 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 1 && x2 & 2 == 2 && x4 & 16 == 16 && x6 & 48 == 16 =>
                {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 1 && x2 & 2 == 2 && x4 & 16 == 16 && x6 & 48 == 48 =>
                {
                    match (
                        (instr >> 24) & 15,
                        (instr >> 22) & 3,
                        (instr >> 21) & 1,
                        (instr >> 19) & 3,
                        (instr >> 17) & 3,
                        (instr >> 16) & 1,
                        (instr >> 14) & 3,
                        instr & 27,
                    ) {
                        (_, _, _, x3, _, _, _, _) if x3 == 0 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 5;
                            let sh = (instr >> 13) & 1;
                            let imm8 = (instr >> 5) & 15;
                            let Zdn = instr & 9;
                            match opc {
                                x0 if x0 == 0 => Some(Op::DdZZi),
                                x0 if x0 == 1 => Some(Op::SubZZi),
                                x0 if x0 == 2 => None,
                                x0 if x0 == 3 => Some(Op::SubrZZi),
                                x0 if x0 == 4 => Some(Op::SqaddZZi),
                                x0 if x0 == 5 => Some(Op::UqaddZZi),
                                x0 if x0 == 6 => Some(Op::SqsubZZi),
                                x0 if x0 == 7 => Some(Op::UqsubZZi),
                                _ => None,
                            }
                        }
                        (_, _, _, x3, _, _, _, _) if x3 == 1 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 5;
                            let o2 = (instr >> 13) & 1;
                            let imm8 = (instr >> 5) & 15;
                            let Zdn = instr & 9;
                            match (opc, o2) {
                                (x0, x1) if x0 & 4 == 0 && x1 == 1 => None,
                                (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::SmaxZZi),
                                (x0, x1) if x0 == 1 && x1 == 0 => Some(Op::UmaxZZi),
                                (x0, x1) if x0 == 2 && x1 == 0 => Some(Op::SminZZi),
                                (x0, x1) if x0 == 3 && x1 == 0 => Some(Op::UminZZi),
                                (x0, _) if x0 & 4 == 4 => None,
                                _ => None,
                            }
                        }
                        (_, _, _, x3, _, _, _, _) if x3 == 2 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 5;
                            let o2 = (instr >> 13) & 1;
                            let imm8 = (instr >> 5) & 15;
                            let Zdn = instr & 9;
                            match (opc, o2) {
                                (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::MulZZi),
                                (x0, x1) if x0 == 0 && x1 == 1 => None,
                                (x0, _) if x0 == 1 => None,
                                (x0, _) if x0 & 6 == 2 => None,
                                (x0, _) if x0 & 4 == 4 => None,
                                _ => None,
                            }
                        }
                        (_, _, _, x3, _, x5, _, _) if x3 == 3 && x5 == 0 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 17) & 3;
                            let sh = (instr >> 13) & 1;
                            let imm8 = (instr >> 5) & 15;
                            let Zd = instr & 9;
                            match opc {
                                x0 if x0 == 0 => Some(Op::DupZI),
                                x0 if x0 == 1 => None,
                                x0 if x0 & 2 == 2 => None,
                                _ => None,
                            }
                        }
                        (_, _, _, x3, _, x5, _, _) if x3 == 3 && x5 == 1 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 17) & 3;
                            let o2 = (instr >> 13) & 1;
                            let imm8 = (instr >> 5) & 15;
                            let Zd = instr & 9;
                            match (opc, o2) {
                                (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::FdupZI),
                                (x0, x1) if x0 == 0 && x1 == 1 => None,
                                (x0, _) if x0 == 1 => None,
                                (x0, _) if x0 & 2 == 2 => None,
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 1 && x2 & 2 == 2 && x4 & 28 == 16 && x6 & 48 == 32 =>
                {
                    let size = (instr >> 22) & 3;
                    let opc = (instr >> 16) & 5;
                    let Pg = (instr >> 10) & 7;
                    let o2 = (instr >> 9) & 1;
                    let Pn = (instr >> 5) & 7;
                    let Rd = instr & 9;
                    match (opc, o2) {
                        (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::NtpRPP),
                        (x0, x1) if x0 == 0 && x1 == 1 => None,
                        (x0, _) if x0 == 1 => None,
                        (x0, _) if x0 & 6 == 2 => None,
                        (x0, _) if x0 & 4 == 4 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 1 && x2 & 2 == 2 && x4 & 28 == 20 && x6 & 60 == 32 =>
                {
                    match (
                        (instr >> 24) & 15,
                        (instr >> 22) & 3,
                        (instr >> 19) & 5,
                        (instr >> 18) & 1,
                        (instr >> 16) & 3,
                        (instr >> 12) & 7,
                        (instr >> 11) & 1,
                        instr & 21,
                    ) {
                        (_, _, _, x3, _, _, x6, _) if x3 == 0 && x6 == 0 => {
                            let size = (instr >> 22) & 3;
                            let D = (instr >> 17) & 1;
                            let U = (instr >> 16) & 1;
                            let opc = (instr >> 9) & 3;
                            let Pm = (instr >> 5) & 7;
                            let Zdn = instr & 9;
                            match (D, U, opc) {
                                (_, _, x2) if x2 == 1 => None,
                                (_, _, x2) if x2 & 2 == 2 => None,
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                                    Some(Op::SqincpZPZ)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    Some(Op::UqincpZPZ)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    Some(Op::SqdecpZPZ)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    Some(Op::UqdecpZPZ)
                                }
                                _ => None,
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
                                (_, _, _, x3) if x3 == 1 => None,
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 => {
                                    Some(Op::SqincpRPRSx)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => {
                                    Some(Op::SqincpRPRX)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 0 => {
                                    Some(Op::UqincpRPRUw)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 0 => {
                                    Some(Op::UqincpRPRX)
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 => {
                                    Some(Op::SqdecpRPRSx)
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 => {
                                    Some(Op::SqdecpRPRX)
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 0 => {
                                    Some(Op::UqdecpRPRUw)
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 0 => {
                                    Some(Op::UqdecpRPRX)
                                }
                                _ => None,
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
                                (x0, _, x2) if x0 == 0 && x2 == 1 => None,
                                (x0, _, x2) if x0 == 0 && x2 & 2 == 2 => None,
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => Some(Op::IncpZPZ),
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => Some(Op::DecpZPZ),
                                (x0, _, _) if x0 == 1 => None,
                                _ => None,
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
                                (x0, _, x2) if x0 == 0 && x2 == 1 => None,
                                (x0, _, x2) if x0 == 0 && x2 & 2 == 2 => None,
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => Some(Op::IncpRPR),
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => Some(Op::DecpRPR),
                                (x0, _, _) if x0 == 1 => None,
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 1 && x2 & 2 == 2 && x4 & 28 == 20 && x6 & 60 == 36 =>
                {
                    match (
                        (instr >> 24) & 15,
                        (instr >> 22) & 3,
                        (instr >> 19) & 5,
                        (instr >> 18) & 1,
                        (instr >> 16) & 3,
                        (instr >> 12) & 7,
                        (instr >> 9) & 5,
                        (instr >> 5) & 7,
                        instr & 9,
                    ) {
                        (_, _, _, x3, x4, _, x6, _, x8)
                            if x3 == 0 && x4 == 0 && x6 == 0 && x8 == 0 =>
                        {
                            let opc = (instr >> 22) & 3;
                            let Pn = (instr >> 5) & 7;
                            match opc {
                                x0 if x0 == 0 => Some(Op::WrffrFP),
                                x0 if x0 == 1 => None,
                                x0 if x0 & 2 == 2 => None,
                                _ => None,
                            }
                        }
                        (_, _, _, x3, x4, _, x6, x7, x8)
                            if x3 == 1 && x4 == 0 && x6 == 0 && x7 == 0 && x8 == 0 =>
                        {
                            let opc = (instr >> 22) & 3;
                            match opc {
                                x0 if x0 == 0 => Some(Op::SetffrF),
                                x0 if x0 == 1 => None,
                                x0 if x0 & 2 == 2 => None,
                                _ => None,
                            }
                        }
                        (_, _, _, x3, x4, _, x6, x7, x8)
                            if x3 == 1 && x4 == 0 && x6 == 0 && x7 & 8 == 8 && x8 == 0 =>
                        {
                            None
                        }
                        (_, _, _, x3, x4, _, x6, x7, x8)
                            if x3 == 1 && x4 == 0 && x6 == 0 && x7 & 4 == 4 && x8 == 0 =>
                        {
                            None
                        }
                        (_, _, _, x3, x4, _, x6, x7, x8)
                            if x3 == 1 && x4 == 0 && x6 == 0 && x7 & 2 == 2 && x8 == 0 =>
                        {
                            None
                        }
                        (_, _, _, x3, x4, _, x6, x7, x8)
                            if x3 == 1 && x4 == 0 && x6 == 0 && x7 & 1 == 1 && x8 == 0 =>
                        {
                            None
                        }
                        (_, _, _, _, x4, _, x6, _, x8) if x4 == 0 && x6 == 0 && x8 != 0 => None,
                        (_, _, _, _, x4, _, x6, _, _) if x4 == 0 && x6 != 0 => None,
                        (_, _, _, _, x4, _, _, _, _) if x4 != 0 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 1 && x2 & 2 == 2 && x4 & 28 == 20 && x6 & 56 == 40 =>
                {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 1 && x2 & 2 == 2 && x4 & 24 == 24 && x6 & 48 == 32 =>
                {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 2 && x2 & 2 == 0 && x4 & 16 == 0 && x6 & 32 == 0 =>
                {
                    match (
                        (instr >> 24) & 15,
                        (instr >> 22) & 3,
                        (instr >> 21) & 1,
                        (instr >> 16) & 9,
                        (instr >> 15) & 1,
                        (instr >> 14) & 1,
                        (instr >> 11) & 5,
                        (instr >> 10) & 1,
                        instr & 19,
                    ) {
                        (_, _, _, _, _, x5, x6, _, _) if x5 == 0 && x6 == 0 => {
                            let size = (instr >> 22) & 3;
                            let Zm = (instr >> 16) & 9;
                            let U = (instr >> 10) & 1;
                            let Zn = (instr >> 5) & 9;
                            let Zda = instr & 9;
                            match U {
                                x0 if x0 == 0 => Some(Op::SdotZZzz),
                                x0 if x0 == 1 => Some(Op::UdotZZzz),
                                _ => None,
                            }
                        }
                        (_, _, _, _, _, x5, x6, _, _) if x5 == 0 && x6 != 0 => None,
                        (_, _, _, _, _, x5, x6, _, _) if x5 == 1 && x6 & 4 == 0 => None,
                        (_, _, _, _, _, x5, x6, _, _) if x5 == 1 && x6 & 6 == 4 => None,
                        (_, _, _, _, _, x5, x6, _, _) if x5 == 1 && x6 == 6 => None,
                        (_, _, _, _, _, x5, x6, x7, _) if x5 == 1 && x6 == 7 && x7 == 0 => {
                            let size = (instr >> 22) & 3;
                            let Zm = (instr >> 16) & 9;
                            let Zn = (instr >> 5) & 9;
                            let Zda = instr & 9;
                            match size {
                                x0 if x0 & 2 == 0 => None,
                                x0 if x0 == 2 => Some(Op::UsdotZZzzS),
                                x0 if x0 == 3 => None,
                                _ => None,
                            }
                        }
                        (_, _, _, _, _, x5, x6, x7, _) if x5 == 1 && x6 == 7 && x7 == 1 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 2 && x2 & 2 == 0 && x4 & 16 == 0 && x6 & 32 == 32 =>
                {
                    None
                }
                (x0, _, x2, _, x4, _, _, _) if x0 == 2 && x2 & 2 == 0 && x4 & 16 == 16 => {
                    match (
                        (instr >> 24) & 15,
                        (instr >> 22) & 3,
                        (instr >> 21) & 1,
                        (instr >> 16) & 9,
                        (instr >> 13) & 5,
                        (instr >> 11) & 3,
                        instr & 21,
                    ) {
                        (_, _, _, _, x4, x5, _) if x4 == 0 && x5 == 0 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 9;
                            let U = (instr >> 10) & 1;
                            let Zn = (instr >> 5) & 9;
                            let Zda = instr & 9;
                            match (size, U) {
                                (x0, _) if x0 & 2 == 0 => None,
                                (x0, x1) if x0 == 2 && x1 == 0 => Some(Op::SdotZZzziS),
                                (x0, x1) if x0 == 2 && x1 == 1 => Some(Op::UdotZZzziS),
                                (x0, x1) if x0 == 3 && x1 == 0 => Some(Op::SdotZZzziD),
                                (x0, x1) if x0 == 3 && x1 == 1 => Some(Op::UdotZZzziD),
                                _ => None,
                            }
                        }
                        (_, _, _, _, x4, x5, _) if x4 == 0 && x5 == 1 => None,
                        (_, _, _, _, x4, x5, _) if x4 == 0 && x5 == 2 => None,
                        (_, _, _, _, x4, x5, _) if x4 == 0 && x5 == 3 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 9;
                            let U = (instr >> 10) & 1;
                            let Zn = (instr >> 5) & 9;
                            let Zda = instr & 9;
                            match (size, U) {
                                (x0, _) if x0 & 2 == 0 => None,
                                (x0, x1) if x0 == 2 && x1 == 0 => Some(Op::UsdotZZzziS),
                                (x0, x1) if x0 == 2 && x1 == 1 => Some(Op::SudotZZzziS),
                                (x0, _) if x0 == 3 => None,
                                _ => None,
                            }
                        }
                        (_, _, _, _, x4, _, _) if x4 != 0 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 2 && x2 & 2 == 2 && x4 & 16 == 0 && x6 & 32 == 0 =>
                {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 2 && x2 & 2 == 2 && x4 & 16 == 0 && x6 & 48 == 32 =>
                {
                    match (
                        (instr >> 24) & 15,
                        (instr >> 22) & 3,
                        (instr >> 21) & 1,
                        (instr >> 16) & 9,
                        (instr >> 14) & 3,
                        (instr >> 10) & 7,
                        instr & 19,
                    ) {
                        (_, _, _, _, _, x5, _) if x5 & 12 == 0 => None,
                        (_, _, _, _, _, x5, _) if x5 & 14 == 4 => None,
                        (_, _, _, _, _, x5, _) if x5 == 6 => {
                            let uns = (instr >> 22) & 3;
                            let Zm = (instr >> 16) & 9;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match uns {
                                x0 if x0 == 0 => Some(Op::SmmlaZZzz),
                                x0 if x0 == 1 => None,
                                x0 if x0 == 2 => Some(Op::UsmmlaZZzz),
                                x0 if x0 == 3 => Some(Op::UmmlaZZzz),
                                _ => None,
                            }
                        }
                        (_, _, _, _, _, x5, _) if x5 == 7 => None,
                        (_, _, _, _, _, x5, _) if x5 & 8 == 8 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 2 && x2 & 2 == 2 && x4 & 16 == 0 && x6 & 48 == 48 =>
                {
                    None
                }
                (x0, _, x2, _, x4, _, _, _) if x0 == 2 && x2 & 2 == 2 && x4 & 16 == 16 => None,
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 0 && x6 & 32 == 0 =>
                {
                    let size = (instr >> 22) & 3;
                    let Zm = (instr >> 16) & 9;
                    let rot = (instr >> 13) & 3;
                    let Pg = (instr >> 10) & 5;
                    let Zn = (instr >> 5) & 9;
                    let Zda = instr & 9;
                    match () {
                        () => Some(Op::FcmlaZPZzz),
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 3 && x2 & 2 == 0 && x4 & 26 == 2 && x6 & 32 == 32 =>
                {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 3 && x2 & 2 == 0 && x4 == 0 && x6 & 56 == 32 =>
                {
                    let size = (instr >> 22) & 3;
                    let rot = (instr >> 16) & 1;
                    let Pg = (instr >> 10) & 5;
                    let Zm = (instr >> 5) & 9;
                    let Zdn = instr & 9;
                    match () {
                        () => Some(Op::FcaddZPZz),
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 3 && x2 & 2 == 0 && x4 == 0 && x6 & 56 == 40 =>
                {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 3 && x2 & 2 == 0 && x4 == 0 && x6 & 48 == 48 =>
                {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 3 && x2 & 2 == 0 && x4 == 1 && x6 & 32 == 32 =>
                {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 3 && x2 & 2 == 0 && x4 & 30 == 4 && x6 & 56 == 32 =>
                {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 3 && x2 & 2 == 0 && x4 & 30 == 4 && x6 & 56 == 40 =>
                {
                    let opc = (instr >> 22) & 3;
                    let opc2 = (instr >> 16) & 3;
                    let Pg = (instr >> 10) & 5;
                    let Zn = (instr >> 5) & 9;
                    let Zd = instr & 9;
                    match (opc, opc2) {
                        (x0, _) if x0 & 2 == 0 => None,
                        (x0, x1) if x0 == 2 && x1 & 2 == 0 => None,
                        (x0, x1) if x0 == 2 && x1 == 2 => Some(Op::BfcvtntZPZS2Bf),
                        (x0, x1) if x0 == 2 && x1 == 3 => None,
                        (x0, _) if x0 == 3 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 3 && x2 & 2 == 0 && x4 & 30 == 4 && x6 & 48 == 48 =>
                {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 3 && x2 & 2 == 0 && x4 & 24 == 8 && x6 & 32 == 32 =>
                {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 22 == 2 =>
                {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 62 == 0 =>
                {
                    let size = (instr >> 22) & 3;
                    let opc = (instr >> 16) & 9;
                    let op = (instr >> 10) & 1;
                    let Zn = (instr >> 5) & 9;
                    let Zda = instr & 9;
                    match (size, op) {
                        (x0, x1) if x0 & 2 == 0 && x1 == 0 => Some(Op::FmlaZZzziH),
                        (x0, x1) if x0 & 2 == 0 && x1 == 1 => Some(Op::FmlsZZzziH),
                        (x0, x1) if x0 == 2 && x1 == 0 => Some(Op::FmlaZZzziS),
                        (x0, x1) if x0 == 2 && x1 == 1 => Some(Op::FmlsZZzziS),
                        (x0, x1) if x0 == 3 && x1 == 0 => Some(Op::FmlaZZzziD),
                        (x0, x1) if x0 == 3 && x1 == 1 => Some(Op::FmlsZZzziD),
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 60 == 4 =>
                {
                    let size = (instr >> 22) & 3;
                    let opc = (instr >> 16) & 9;
                    let rot = (instr >> 10) & 3;
                    let Zn = (instr >> 5) & 9;
                    let Zda = instr & 9;
                    match size {
                        x0 if x0 & 2 == 0 => None,
                        x0 if x0 == 2 => Some(Op::FcmlaZZzziH),
                        x0 if x0 == 3 => Some(Op::FcmlaZZzziS),
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 == 8 =>
                {
                    let size = (instr >> 22) & 3;
                    let opc = (instr >> 16) & 9;
                    let Zn = (instr >> 5) & 9;
                    let Zd = instr & 9;
                    match size {
                        x0 if x0 & 2 == 0 => Some(Op::FmulZZziH),
                        x0 if x0 == 2 => Some(Op::FmulZZziS),
                        x0 if x0 == 3 => Some(Op::FmulZZziD),
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 == 9 =>
                {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 60 == 12 =>
                {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 52 == 16 =>
                {
                    match (
                        (instr >> 24) & 15,
                        (instr >> 23) & 1,
                        (instr >> 22) & 1,
                        (instr >> 21) & 1,
                        (instr >> 16) & 9,
                        (instr >> 14) & 3,
                        (instr >> 13) & 1,
                        (instr >> 12) & 1,
                        (instr >> 10) & 3,
                        instr & 19,
                    ) {
                        (_, x1, _, _, _, _, x6, _, x8, _) if x1 == 0 && x6 == 0 && x8 == 0 => {
                            let op = (instr >> 22) & 1;
                            let i2 = (instr >> 19) & 3;
                            let Zm = (instr >> 16) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Zda = instr & 9;
                            match op {
                                x0 if x0 == 0 => None,
                                x0 if x0 == 1 => Some(Op::BfdotZZzzi),
                                _ => None,
                            }
                        }
                        (_, x1, _, _, _, _, x6, _, x8, _) if x1 == 0 && x6 == 0 && x8 != 0 => None,
                        (_, x1, _, _, _, _, x6, _, _, _) if x1 == 0 && x6 == 1 => None,
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
                                (x0, _, _) if x0 == 0 => None,
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    Some(Op::BfmlalbZZzzi)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    Some(Op::BfmlaltZZzzi)
                                }
                                (x0, x1, _) if x0 == 1 && x1 == 1 => None,
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 52 == 20 =>
                {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 54 == 32 =>
                {
                    match (
                        (instr >> 24) & 15,
                        (instr >> 23) & 1,
                        (instr >> 22) & 1,
                        (instr >> 21) & 1,
                        (instr >> 16) & 9,
                        (instr >> 14) & 3,
                        (instr >> 13) & 1,
                        (instr >> 11) & 3,
                        (instr >> 10) & 1,
                        instr & 19,
                    ) {
                        (_, x1, _, _, _, _, x6, _, x8, _) if x1 == 0 && x6 == 0 && x8 == 0 => {
                            let op = (instr >> 22) & 1;
                            let Zm = (instr >> 16) & 9;
                            let Zn = (instr >> 5) & 9;
                            let Zda = instr & 9;
                            match op {
                                x0 if x0 == 0 => None,
                                x0 if x0 == 1 => Some(Op::BfdotZZzz),
                                _ => None,
                            }
                        }
                        (_, x1, _, _, _, _, x6, _, x8, _) if x1 == 0 && x6 == 0 && x8 == 1 => None,
                        (_, x1, _, _, _, _, x6, _, _, _) if x1 == 0 && x6 == 1 => None,
                        (_, x1, _, _, _, _, _, _, _, _) if x1 == 1 => {
                            let o2 = (instr >> 22) & 1;
                            let Zm = (instr >> 16) & 9;
                            let op = (instr >> 13) & 1;
                            let T = (instr >> 10) & 1;
                            let Zn = (instr >> 5) & 9;
                            let Zda = instr & 9;
                            match (o2, op, T) {
                                (x0, _, _) if x0 == 0 => None,
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    Some(Op::BfmlalbZZzz)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    Some(Op::BfmlaltZZzz)
                                }
                                (x0, x1, _) if x0 == 1 && x1 == 1 => None,
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 52 == 36 =>
                {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 56 == 48 =>
                {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 == 56 =>
                {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 == 57 =>
                {
                    let opc = (instr >> 22) & 3;
                    let Zm = (instr >> 16) & 9;
                    let Zn = (instr >> 5) & 9;
                    let Zda = instr & 9;
                    match opc {
                        x0 if x0 == 0 => None,
                        x0 if x0 == 1 => Some(Op::BfmmlaZZzz),
                        x0 if x0 == 2 => Some(Op::FmmlaZZzzS),
                        x0 if x0 == 3 => Some(Op::FmmlaZZzzD),
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 62 == 58 =>
                {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 3 && x2 & 2 == 0 && x4 & 16 == 16 && x6 & 60 == 60 =>
                {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 3 && x2 & 2 == 2 && x4 & 16 == 0 && x6 & 16 == 16 =>
                {
                    let size = (instr >> 22) & 3;
                    let Zm = (instr >> 16) & 9;
                    let op = (instr >> 15) & 1;
                    let o2 = (instr >> 13) & 1;
                    let Pg = (instr >> 10) & 5;
                    let Zn = (instr >> 5) & 9;
                    let o3 = (instr >> 4) & 1;
                    let Pd = instr & 7;
                    match (op, o2, o3) {
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => Some(Op::FcmgePPZz),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => Some(Op::FcmgtPPZz),
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => Some(Op::FcmeqPPZz),
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => Some(Op::FcmnePPZz),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => Some(Op::FcmuoPPZz),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => Some(Op::FacgePPZz),
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => None,
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => Some(Op::FacgtPPZz),
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 3 && x2 & 2 == 2 && x4 & 16 == 0 && x6 & 56 == 0 =>
                {
                    let size = (instr >> 22) & 3;
                    let Zm = (instr >> 16) & 9;
                    let opc = (instr >> 10) & 5;
                    let Zn = (instr >> 5) & 9;
                    let Zd = instr & 9;
                    match opc {
                        x0 if x0 == 0 => Some(Op::FaddZZz),
                        x0 if x0 == 1 => Some(Op::FsubZZz),
                        x0 if x0 == 2 => Some(Op::FmulZZz),
                        x0 if x0 == 3 => Some(Op::FtsmulZZz),
                        x0 if x0 & 6 == 4 => None,
                        x0 if x0 == 6 => Some(Op::FrecpsZZz),
                        x0 if x0 == 7 => Some(Op::FrsqrtsZZz),
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 3 && x2 & 2 == 2 && x4 & 16 == 0 && x6 & 56 == 32 =>
                {
                    match (
                        (instr >> 24) & 15,
                        (instr >> 22) & 3,
                        (instr >> 21) & 1,
                        (instr >> 19) & 3,
                        (instr >> 16) & 5,
                        (instr >> 13) & 5,
                        (instr >> 10) & 5,
                        (instr >> 6) & 7,
                        instr & 11,
                    ) {
                        (_, _, _, x3, _, _, _, _, _) if x3 & 2 == 0 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 7;
                            let Pg = (instr >> 10) & 5;
                            let Zm = (instr >> 5) & 9;
                            let Zdn = instr & 9;
                            match opc {
                                x0 if x0 == 0 => Some(Op::FaddZPZz),
                                x0 if x0 == 1 => Some(Op::FsubZPZz),
                                x0 if x0 == 2 => Some(Op::FmulZPZz),
                                x0 if x0 == 3 => Some(Op::FsubrZPZz),
                                x0 if x0 == 4 => Some(Op::FmaxnmZPZz),
                                x0 if x0 == 5 => Some(Op::FminnmZPZz),
                                x0 if x0 == 6 => Some(Op::FmaxZPZz),
                                x0 if x0 == 7 => Some(Op::FminZPZz),
                                x0 if x0 == 8 => Some(Op::FabdZPZz),
                                x0 if x0 == 9 => Some(Op::FscaleZPZz),
                                x0 if x0 == 10 => Some(Op::FmulxZPZz),
                                x0 if x0 == 11 => None,
                                x0 if x0 == 12 => Some(Op::FdivrZPZz),
                                x0 if x0 == 13 => Some(Op::FdivZPZz),
                                x0 if x0 & 14 == 14 => None,
                                _ => None,
                            }
                        }
                        (_, _, _, x3, _, _, x6, _, _) if x3 == 2 && x6 == 0 => {
                            let size = (instr >> 22) & 3;
                            let imm3 = (instr >> 16) & 5;
                            let Zm = (instr >> 5) & 9;
                            let Zdn = instr & 9;
                            match () {
                                () => Some(Op::FtmadZZzi),
                            }
                        }
                        (_, _, _, x3, _, _, x6, _, _) if x3 == 2 && x6 != 0 => None,
                        (_, _, _, x3, _, _, _, x7, _) if x3 == 3 && x7 == 0 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 5;
                            let Pg = (instr >> 10) & 5;
                            let i1 = (instr >> 5) & 1;
                            let Zdn = instr & 9;
                            match opc {
                                x0 if x0 == 0 => Some(Op::FaddZPZs),
                                x0 if x0 == 1 => Some(Op::FsubZPZs),
                                x0 if x0 == 2 => Some(Op::FmulZPZs),
                                x0 if x0 == 3 => Some(Op::FsubrZPZs),
                                x0 if x0 == 4 => Some(Op::FmaxnmZPZs),
                                x0 if x0 == 5 => Some(Op::FminnmZPZs),
                                x0 if x0 == 6 => Some(Op::FmaxZPZs),
                                x0 if x0 == 7 => Some(Op::FminZPZs),
                                _ => None,
                            }
                        }
                        (_, _, _, x3, _, _, _, x7, _) if x3 == 3 && x7 != 0 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 3 && x2 & 2 == 2 && x4 & 16 == 0 && x6 & 56 == 40 =>
                {
                    match (
                        (instr >> 24) & 15,
                        (instr >> 22) & 3,
                        (instr >> 21) & 1,
                        (instr >> 18) & 5,
                        (instr >> 16) & 3,
                        (instr >> 13) & 5,
                        instr & 25,
                    ) {
                        (_, _, _, x3, _, _, _) if x3 & 6 == 0 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 5;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match opc {
                                x0 if x0 == 0 => Some(Op::FrintnZPZ),
                                x0 if x0 == 1 => Some(Op::FrintpZPZ),
                                x0 if x0 == 2 => Some(Op::FrintmZPZ),
                                x0 if x0 == 3 => Some(Op::FrintzZPZ),
                                x0 if x0 == 4 => Some(Op::FrintaZPZ),
                                x0 if x0 == 5 => None,
                                x0 if x0 == 6 => Some(Op::FrintxZPZ),
                                x0 if x0 == 7 => Some(Op::FrintiZPZ),
                                _ => None,
                            }
                        }
                        (_, _, _, x3, _, _, _) if x3 == 2 => {
                            let opc = (instr >> 22) & 3;
                            let opc2 = (instr >> 16) & 3;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match (opc, opc2) {
                                (x0, _) if x0 & 2 == 0 => None,
                                (x0, x1) if x0 == 2 && x1 == 0 => Some(Op::FcvtZPZS2H),
                                (x0, x1) if x0 == 2 && x1 == 1 => Some(Op::FcvtZPZH2S),
                                (x0, x1) if x0 == 2 && x1 == 2 => Some(Op::BfcvtZPZS2Bf),
                                (x0, x1) if x0 == 2 && x1 == 3 => None,
                                (x0, x1) if x0 == 3 && x1 == 0 => Some(Op::FcvtZPZD2H),
                                (x0, x1) if x0 == 3 && x1 == 1 => Some(Op::FcvtZPZH2D),
                                (x0, x1) if x0 == 3 && x1 == 2 => Some(Op::FcvtZPZD2S),
                                (x0, x1) if x0 == 3 && x1 == 3 => Some(Op::FcvtZPZS2D),
                                _ => None,
                            }
                        }
                        (_, _, _, x3, _, _, _) if x3 == 3 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 3;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match opc {
                                x0 if x0 == 0 => Some(Op::FrecpxZPZ),
                                x0 if x0 == 1 => Some(Op::FsqrtZPZ),
                                x0 if x0 & 2 == 2 => None,
                                _ => None,
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
                                (x0, _, _) if x0 == 0 => None,
                                (x0, x1, _) if x0 == 1 && x1 == 0 => None,
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    Some(Op::ScvtfZPZH2Fp16)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    Some(Op::UcvtfZPZH2Fp16)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 == 0 => {
                                    Some(Op::ScvtfZPZW2Fp16)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 == 1 => {
                                    Some(Op::UcvtfZPZW2Fp16)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 0 => {
                                    Some(Op::ScvtfZPZX2Fp16)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 1 => {
                                    Some(Op::UcvtfZPZX2Fp16)
                                }
                                (x0, x1, _) if x0 == 2 && x1 & 2 == 0 => None,
                                (x0, x1, x2) if x0 == 2 && x1 == 2 && x2 == 0 => {
                                    Some(Op::ScvtfZPZW2S)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 2 && x2 == 1 => {
                                    Some(Op::UcvtfZPZW2S)
                                }
                                (x0, x1, _) if x0 == 2 && x1 == 3 => None,
                                (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 0 => {
                                    Some(Op::ScvtfZPZW2D)
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 1 => {
                                    Some(Op::UcvtfZPZW2D)
                                }
                                (x0, x1, _) if x0 == 3 && x1 == 1 => None,
                                (x0, x1, x2) if x0 == 3 && x1 == 2 && x2 == 0 => {
                                    Some(Op::ScvtfZPZX2S)
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 2 && x2 == 1 => {
                                    Some(Op::UcvtfZPZX2S)
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 3 && x2 == 0 => {
                                    Some(Op::ScvtfZPZX2D)
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 3 && x2 == 1 => {
                                    Some(Op::UcvtfZPZX2D)
                                }
                                _ => None,
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
                                (x0, _, _) if x0 == 0 => None,
                                (x0, x1, _) if x0 == 1 && x1 == 0 => None,
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    Some(Op::FcvtzsZPZFp162H)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    Some(Op::FcvtzuZPZFp162H)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 == 0 => {
                                    Some(Op::FcvtzsZPZFp162W)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 == 1 => {
                                    Some(Op::FcvtzuZPZFp162W)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 0 => {
                                    Some(Op::FcvtzsZPZFp162X)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 1 => {
                                    Some(Op::FcvtzuZPZFp162X)
                                }
                                (x0, x1, _) if x0 == 2 && x1 & 2 == 0 => None,
                                (x0, x1, x2) if x0 == 2 && x1 == 2 && x2 == 0 => {
                                    Some(Op::FcvtzsZPZS2W)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 2 && x2 == 1 => {
                                    Some(Op::FcvtzuZPZS2W)
                                }
                                (x0, x1, _) if x0 == 2 && x1 == 3 => None,
                                (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 0 => {
                                    Some(Op::FcvtzsZPZD2W)
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 1 => {
                                    Some(Op::FcvtzuZPZD2W)
                                }
                                (x0, x1, _) if x0 == 3 && x1 == 1 => None,
                                (x0, x1, x2) if x0 == 3 && x1 == 2 && x2 == 0 => {
                                    Some(Op::FcvtzsZPZS2X)
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 2 && x2 == 1 => {
                                    Some(Op::FcvtzuZPZS2X)
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 3 && x2 == 0 => {
                                    Some(Op::FcvtzsZPZD2X)
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 3 && x2 == 1 => {
                                    Some(Op::FcvtzuZPZD2X)
                                }
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 3 && x2 & 2 == 2 && x4 & 28 == 0 && x6 & 56 == 8 =>
                {
                    let size = (instr >> 22) & 3;
                    let opc = (instr >> 16) & 5;
                    let Pg = (instr >> 10) & 5;
                    let Zn = (instr >> 5) & 9;
                    let Vd = instr & 9;
                    match opc {
                        x0 if x0 == 0 => Some(Op::FaddvVPZ),
                        x0 if x0 == 1 => None,
                        x0 if x0 & 6 == 2 => None,
                        x0 if x0 == 4 => Some(Op::FmaxnmvVPZ),
                        x0 if x0 == 5 => Some(Op::FminnmvVPZ),
                        x0 if x0 == 6 => Some(Op::FmaxvVPZ),
                        x0 if x0 == 7 => Some(Op::FminvVPZ),
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 3 && x2 & 2 == 2 && x4 & 28 == 4 && x6 & 60 == 8 =>
                {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 3 && x2 & 2 == 2 && x4 & 28 == 4 && x6 & 60 == 12 =>
                {
                    match (
                        (instr >> 24) & 15,
                        (instr >> 22) & 3,
                        (instr >> 19) & 5,
                        (instr >> 16) & 5,
                        (instr >> 12) & 7,
                        (instr >> 10) & 3,
                        instr & 19,
                    ) {
                        (_, _, _, _, _, x5, _) if x5 == 0 => {
                            let size = (instr >> 22) & 3;
                            let opc = (instr >> 16) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Zd = instr & 9;
                            match opc {
                                x0 if x0 & 4 == 0 => None,
                                x0 if x0 & 6 == 4 => None,
                                x0 if x0 == 6 => Some(Op::FrecpeZZ),
                                x0 if x0 == 7 => Some(Op::FrsqrteZZ),
                                _ => None,
                            }
                        }
                        (_, _, _, _, _, x5, _) if x5 != 0 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 3 && x2 & 2 == 2 && x4 & 28 == 8 && x6 & 56 == 8 =>
                {
                    match (
                        (instr >> 24) & 15,
                        (instr >> 22) & 3,
                        (instr >> 19) & 5,
                        (instr >> 18) & 1,
                        (instr >> 16) & 3,
                        (instr >> 13) & 5,
                        instr & 25,
                    ) {
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
                                    Some(Op::FcmgePPZ0)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                                    Some(Op::FcmgtPPZ0)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    Some(Op::FcmltPPZ0)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    Some(Op::FcmlePPZ0)
                                }
                                (x0, _, x2) if x0 == 1 && x2 == 1 => None,
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    Some(Op::FcmeqPPZ0)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    Some(Op::FcmnePPZ0)
                                }
                                _ => None,
                            }
                        }
                        (_, _, _, x3, _, _, _) if x3 == 1 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _)
                    if x0 == 3 && x2 & 2 == 2 && x4 & 28 == 12 && x6 & 56 == 8 =>
                {
                    let size = (instr >> 22) & 3;
                    let opc = (instr >> 16) & 5;
                    let Pg = (instr >> 10) & 5;
                    let Zm = (instr >> 5) & 9;
                    let Vdn = instr & 9;
                    match opc {
                        x0 if x0 == 0 => Some(Op::FaddaVPZ),
                        x0 if x0 == 1 => None,
                        x0 if x0 & 6 == 2 => None,
                        x0 if x0 & 4 == 4 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, _, _) if x0 == 3 && x2 & 2 == 2 && x4 & 16 == 16 => {
                    match (
                        (instr >> 24) & 15,
                        (instr >> 22) & 3,
                        (instr >> 21) & 1,
                        (instr >> 16) & 9,
                        (instr >> 15) & 1,
                        instr & 29,
                    ) {
                        (_, _, _, _, x4, _) if x4 == 0 => {
                            let size = (instr >> 22) & 3;
                            let Zm = (instr >> 16) & 9;
                            let opc = (instr >> 13) & 3;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Zda = instr & 9;
                            match opc {
                                x0 if x0 == 0 => Some(Op::FmlaZPZzz),
                                x0 if x0 == 1 => Some(Op::FmlsZPZzz),
                                x0 if x0 == 2 => Some(Op::FnmlaZPZzz),
                                x0 if x0 == 3 => Some(Op::FnmlsZPZzz),
                                _ => None,
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
                                x0 if x0 == 0 => Some(Op::FmadZPZzz),
                                x0 if x0 == 1 => Some(Op::FmsbZPZzz),
                                x0 if x0 == 2 => Some(Op::FnmadZPZzz),
                                x0 if x0 == 3 => Some(Op::FnmsbZPZzz),
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                (x0, _, _, _, _, _, _, _) if x0 == 4 => {
                    match (
                        (instr >> 25) & 13,
                        (instr >> 23) & 3,
                        (instr >> 21) & 3,
                        (instr >> 16) & 9,
                        (instr >> 13) & 5,
                        (instr >> 5) & 15,
                        (instr >> 4) & 1,
                        instr & 7,
                    ) {
                        (_, x1, x2, _, x4, _, x6, _)
                            if x1 == 0 && x2 & 1 == 1 && x4 & 4 == 0 && x6 == 0 =>
                        {
                            let xs = (instr >> 22) & 1;
                            let Zm = (instr >> 16) & 9;
                            let msz = (instr >> 13) & 3;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let prfop = instr & 7;
                            match msz {
                                x0 if x0 == 0 => Some(Op::PrfbIPBzSX32Scaled),
                                x0 if x0 == 1 => Some(Op::PrfhIPBzSX32Scaled),
                                x0 if x0 == 2 => Some(Op::PrfwIPBzSX32Scaled),
                                x0 if x0 == 3 => Some(Op::PrfdIPBzSX32Scaled),
                                _ => None,
                            }
                        }
                        (_, x1, x2, _, x4, _, x6, _)
                            if x1 == 0 && x2 & 1 == 1 && x4 & 4 == 0 && x6 == 1 =>
                        {
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
                                (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::Ld1ShZPBzSX32Scaled),
                                (x0, x1) if x0 == 0 && x1 == 1 => Some(Op::Ldff1ShZPBzSX32Scaled),
                                (x0, x1) if x0 == 1 && x1 == 0 => Some(Op::Ld1HZPBzSX32Scaled),
                                (x0, x1) if x0 == 1 && x1 == 1 => Some(Op::Ldff1HZPBzSX32Scaled),
                                _ => None,
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
                                (x0, _) if x0 == 0 => None,
                                (x0, x1) if x0 == 1 && x1 == 0 => Some(Op::Ld1WZPBzSX32Scaled),
                                (x0, x1) if x0 == 1 && x1 == 1 => Some(Op::Ldff1WZPBzSX32Scaled),
                                _ => None,
                            }
                        }
                        (_, x1, x2, _, x4, _, x6, _)
                            if x1 == 3 && x2 & 2 == 0 && x4 == 0 && x6 == 0 =>
                        {
                            let imm9h = (instr >> 16) & 11;
                            let imm9l = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Pt = instr & 7;
                            match () {
                                () => Some(Op::LdrPBi),
                            }
                        }
                        (_, x1, x2, _, x4, _, x6, _)
                            if x1 == 3 && x2 & 2 == 0 && x4 == 0 && x6 == 1 =>
                        {
                            None
                        }
                        (_, x1, x2, _, x4, _, _, _) if x1 == 3 && x2 & 2 == 0 && x4 == 2 => {
                            let imm9h = (instr >> 16) & 11;
                            let imm9l = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match () {
                                () => Some(Op::LdrZBi),
                            }
                        }
                        (_, x1, x2, _, x4, _, _, _) if x1 == 3 && x2 & 2 == 0 && x4 & 5 == 1 => {
                            None
                        }
                        (_, x1, x2, _, x4, _, x6, _)
                            if x1 == 3 && x2 & 2 == 2 && x4 & 4 == 0 && x6 == 0 =>
                        {
                            let imm6 = (instr >> 16) & 11;
                            let msz = (instr >> 13) & 3;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let prfop = instr & 7;
                            match msz {
                                x0 if x0 == 0 => Some(Op::PrfbIPBiS),
                                x0 if x0 == 1 => Some(Op::PrfhIPBiS),
                                x0 if x0 == 2 => Some(Op::PrfwIPBiS),
                                x0 if x0 == 3 => Some(Op::PrfdIPBiS),
                                _ => None,
                            }
                        }
                        (_, x1, x2, _, x4, _, x6, _)
                            if x1 == 3 && x2 & 2 == 2 && x4 & 4 == 0 && x6 == 1 =>
                        {
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
                                    Some(Op::Ld1SbZPBzSX32Unscaled)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                                    Some(Op::Ldff1SbZPBzSX32Unscaled)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    Some(Op::Ld1BZPBzSX32Unscaled)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    Some(Op::Ldff1BZPBzSX32Unscaled)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    Some(Op::Ld1ShZPBzSX32Unscaled)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    Some(Op::Ldff1ShZPBzSX32Unscaled)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    Some(Op::Ld1HZPBzSX32Unscaled)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    Some(Op::Ldff1HZPBzSX32Unscaled)
                                }
                                (x0, x1, _) if x0 == 2 && x1 == 0 => None,
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                                    Some(Op::Ld1WZPBzSX32Unscaled)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                                    Some(Op::Ldff1WZPBzSX32Unscaled)
                                }
                                _ => None,
                            }
                        }
                        (_, _, x2, _, x4, _, _, _) if x2 == 0 && x4 & 6 == 4 => None,
                        (_, _, x2, _, x4, _, x6, _) if x2 == 0 && x4 == 6 && x6 == 0 => {
                            let msz = (instr >> 23) & 3;
                            let Rm = (instr >> 16) & 9;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let prfop = instr & 7;
                            match msz {
                                x0 if x0 == 0 => Some(Op::PrfbIPBrS),
                                x0 if x0 == 1 => Some(Op::PrfhIPBrS),
                                x0 if x0 == 2 => Some(Op::PrfwIPBrS),
                                x0 if x0 == 3 => Some(Op::PrfdIPBrS),
                                _ => None,
                            }
                        }
                        (_, _, x2, _, x4, _, x6, _) if x2 == 0 && x4 == 7 && x6 == 0 => {
                            let msz = (instr >> 23) & 3;
                            let imm5 = (instr >> 16) & 9;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let prfop = instr & 7;
                            match msz {
                                x0 if x0 == 0 => Some(Op::PrfbIPAiS),
                                x0 if x0 == 1 => Some(Op::PrfhIPAiS),
                                x0 if x0 == 2 => Some(Op::PrfwIPAiS),
                                x0 if x0 == 3 => Some(Op::PrfdIPAiS),
                                _ => None,
                            }
                        }
                        (_, _, x2, _, x4, _, x6, _) if x2 == 0 && x4 & 6 == 6 && x6 == 1 => None,
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
                                    Some(Op::Ld1SbZPAiS)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                                    Some(Op::Ldff1SbZPAiS)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    Some(Op::Ld1BZPAiS)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    Some(Op::Ldff1BZPAiS)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    Some(Op::Ld1ShZPAiS)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    Some(Op::Ldff1ShZPAiS)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    Some(Op::Ld1HZPAiS)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    Some(Op::Ldff1HZPAiS)
                                }
                                (x0, x1, _) if x0 == 2 && x1 == 0 => None,
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                                    Some(Op::Ld1WZPAiS)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                                    Some(Op::Ldff1WZPAiS)
                                }
                                (x0, _, _) if x0 == 3 => None,
                                _ => None,
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
                                (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::Ld1RbZPBiU8),
                                (x0, x1) if x0 == 0 && x1 == 1 => Some(Op::Ld1RbZPBiU16),
                                (x0, x1) if x0 == 0 && x1 == 2 => Some(Op::Ld1RbZPBiU32),
                                (x0, x1) if x0 == 0 && x1 == 3 => Some(Op::Ld1RbZPBiU64),
                                (x0, x1) if x0 == 1 && x1 == 0 => Some(Op::Ld1RswZPBiS64),
                                (x0, x1) if x0 == 1 && x1 == 1 => Some(Op::Ld1RhZPBiU16),
                                (x0, x1) if x0 == 1 && x1 == 2 => Some(Op::Ld1RhZPBiU32),
                                (x0, x1) if x0 == 1 && x1 == 3 => Some(Op::Ld1RhZPBiU64),
                                (x0, x1) if x0 == 2 && x1 == 0 => Some(Op::Ld1RshZPBiS64),
                                (x0, x1) if x0 == 2 && x1 == 1 => Some(Op::Ld1RshZPBiS32),
                                (x0, x1) if x0 == 2 && x1 == 2 => Some(Op::Ld1RwZPBiU32),
                                (x0, x1) if x0 == 2 && x1 == 3 => Some(Op::Ld1RwZPBiU64),
                                (x0, x1) if x0 == 3 && x1 == 0 => Some(Op::Ld1RsbZPBiS64),
                                (x0, x1) if x0 == 3 && x1 == 1 => Some(Op::Ld1RsbZPBiS32),
                                (x0, x1) if x0 == 3 && x1 == 2 => Some(Op::Ld1RsbZPBiS16),
                                (x0, x1) if x0 == 3 && x1 == 3 => Some(Op::Ld1RdZPBiU64),
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                (x0, _, _, _, _, _, _, _) if x0 == 5 => {
                    match (
                        (instr >> 25) & 13,
                        (instr >> 23) & 3,
                        (instr >> 21) & 3,
                        (instr >> 20) & 1,
                        (instr >> 16) & 7,
                        (instr >> 13) & 5,
                        instr & 25,
                    ) {
                        (_, _, x2, x3, _, x5, _) if x2 == 0 && x3 == 0 && x5 == 7 => {
                            let msz = (instr >> 23) & 3;
                            let imm4 = (instr >> 16) & 7;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match msz {
                                x0 if x0 == 0 => Some(Op::Ldnt1BZPBiContiguous),
                                x0 if x0 == 1 => Some(Op::Ldnt1HZPBiContiguous),
                                x0 if x0 == 2 => Some(Op::Ldnt1WZPBiContiguous),
                                x0 if x0 == 3 => Some(Op::Ldnt1DZPBiContiguous),
                                _ => None,
                            }
                        }
                        (_, _, x2, _, _, x5, _) if x2 == 0 && x5 == 6 => {
                            let msz = (instr >> 23) & 3;
                            let Rm = (instr >> 16) & 9;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match msz {
                                x0 if x0 == 0 => Some(Op::Ldnt1BZPBrContiguous),
                                x0 if x0 == 1 => Some(Op::Ldnt1HZPBrContiguous),
                                x0 if x0 == 2 => Some(Op::Ldnt1WZPBrContiguous),
                                x0 if x0 == 3 => Some(Op::Ldnt1DZPBrContiguous),
                                _ => None,
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
                                (x0, x1) if x0 == 0 && x1 == 1 => Some(Op::Ld2BZPBiContiguous),
                                (x0, x1) if x0 == 0 && x1 == 2 => Some(Op::Ld3BZPBiContiguous),
                                (x0, x1) if x0 == 0 && x1 == 3 => Some(Op::Ld4BZPBiContiguous),
                                (x0, x1) if x0 == 1 && x1 == 1 => Some(Op::Ld2HZPBiContiguous),
                                (x0, x1) if x0 == 1 && x1 == 2 => Some(Op::Ld3HZPBiContiguous),
                                (x0, x1) if x0 == 1 && x1 == 3 => Some(Op::Ld4HZPBiContiguous),
                                (x0, x1) if x0 == 2 && x1 == 1 => Some(Op::Ld2WZPBiContiguous),
                                (x0, x1) if x0 == 2 && x1 == 2 => Some(Op::Ld3WZPBiContiguous),
                                (x0, x1) if x0 == 2 && x1 == 3 => Some(Op::Ld4WZPBiContiguous),
                                (x0, x1) if x0 == 3 && x1 == 1 => Some(Op::Ld2DZPBiContiguous),
                                (x0, x1) if x0 == 3 && x1 == 2 => Some(Op::Ld3DZPBiContiguous),
                                (x0, x1) if x0 == 3 && x1 == 3 => Some(Op::Ld4DZPBiContiguous),
                                _ => None,
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
                                (x0, x1) if x0 == 0 && x1 == 1 => Some(Op::Ld2BZPBrContiguous),
                                (x0, x1) if x0 == 0 && x1 == 2 => Some(Op::Ld3BZPBrContiguous),
                                (x0, x1) if x0 == 0 && x1 == 3 => Some(Op::Ld4BZPBrContiguous),
                                (x0, x1) if x0 == 1 && x1 == 1 => Some(Op::Ld2HZPBrContiguous),
                                (x0, x1) if x0 == 1 && x1 == 2 => Some(Op::Ld3HZPBrContiguous),
                                (x0, x1) if x0 == 1 && x1 == 3 => Some(Op::Ld4HZPBrContiguous),
                                (x0, x1) if x0 == 2 && x1 == 1 => Some(Op::Ld2WZPBrContiguous),
                                (x0, x1) if x0 == 2 && x1 == 2 => Some(Op::Ld3WZPBrContiguous),
                                (x0, x1) if x0 == 2 && x1 == 3 => Some(Op::Ld4WZPBrContiguous),
                                (x0, x1) if x0 == 3 && x1 == 1 => Some(Op::Ld2DZPBrContiguous),
                                (x0, x1) if x0 == 3 && x1 == 2 => Some(Op::Ld3DZPBrContiguous),
                                (x0, x1) if x0 == 3 && x1 == 3 => Some(Op::Ld4DZPBrContiguous),
                                _ => None,
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
                                (_, x1) if x1 & 2 == 2 => None,
                                (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::Ld1RqbZPBiU8),
                                (x0, x1) if x0 == 0 && x1 == 1 => Some(Op::Ld1RobZPBiU8),
                                (x0, x1) if x0 == 1 && x1 == 0 => Some(Op::Ld1RqhZPBiU16),
                                (x0, x1) if x0 == 1 && x1 == 1 => Some(Op::Ld1RohZPBiU16),
                                (x0, x1) if x0 == 2 && x1 == 0 => Some(Op::Ld1RqwZPBiU32),
                                (x0, x1) if x0 == 2 && x1 == 1 => Some(Op::Ld1RowZPBiU32),
                                (x0, x1) if x0 == 3 && x1 == 0 => Some(Op::Ld1RqdZPBiU64),
                                (x0, x1) if x0 == 3 && x1 == 1 => Some(Op::Ld1RodZPBiU64),
                                _ => None,
                            }
                        }
                        (_, _, _, x3, _, x5, _) if x3 == 0 && x5 == 5 => {
                            let dtype = (instr >> 21) & 7;
                            let imm4 = (instr >> 16) & 7;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match dtype {
                                x0 if x0 == 0 => Some(Op::Ld1BZPBiU8),
                                x0 if x0 == 1 => Some(Op::Ld1BZPBiU16),
                                x0 if x0 == 2 => Some(Op::Ld1BZPBiU32),
                                x0 if x0 == 3 => Some(Op::Ld1BZPBiU64),
                                x0 if x0 == 4 => Some(Op::Ld1SwZPBiS64),
                                x0 if x0 == 5 => Some(Op::Ld1HZPBiU16),
                                x0 if x0 == 6 => Some(Op::Ld1HZPBiU32),
                                x0 if x0 == 7 => Some(Op::Ld1HZPBiU64),
                                x0 if x0 == 8 => Some(Op::Ld1ShZPBiS64),
                                x0 if x0 == 9 => Some(Op::Ld1ShZPBiS32),
                                x0 if x0 == 10 => Some(Op::Ld1WZPBiU32),
                                x0 if x0 == 11 => Some(Op::Ld1WZPBiU64),
                                x0 if x0 == 12 => Some(Op::Ld1SbZPBiS64),
                                x0 if x0 == 13 => Some(Op::Ld1SbZPBiS32),
                                x0 if x0 == 14 => Some(Op::Ld1SbZPBiS16),
                                x0 if x0 == 15 => Some(Op::Ld1DZPBiU64),
                                _ => None,
                            }
                        }
                        (_, _, _, x3, _, x5, _) if x3 == 1 && x5 == 1 => None,
                        (_, _, _, x3, _, x5, _) if x3 == 1 && x5 == 5 => {
                            let dtype = (instr >> 21) & 7;
                            let imm4 = (instr >> 16) & 7;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match dtype {
                                x0 if x0 == 0 => Some(Op::Ldnf1BZPBiU8),
                                x0 if x0 == 1 => Some(Op::Ldnf1BZPBiU16),
                                x0 if x0 == 2 => Some(Op::Ldnf1BZPBiU32),
                                x0 if x0 == 3 => Some(Op::Ldnf1BZPBiU64),
                                x0 if x0 == 4 => Some(Op::Ldnf1SwZPBiS64),
                                x0 if x0 == 5 => Some(Op::Ldnf1HZPBiU16),
                                x0 if x0 == 6 => Some(Op::Ldnf1HZPBiU32),
                                x0 if x0 == 7 => Some(Op::Ldnf1HZPBiU64),
                                x0 if x0 == 8 => Some(Op::Ldnf1ShZPBiS64),
                                x0 if x0 == 9 => Some(Op::Ldnf1ShZPBiS32),
                                x0 if x0 == 10 => Some(Op::Ldnf1WZPBiU32),
                                x0 if x0 == 11 => Some(Op::Ldnf1WZPBiU64),
                                x0 if x0 == 12 => Some(Op::Ldnf1SbZPBiS64),
                                x0 if x0 == 13 => Some(Op::Ldnf1SbZPBiS32),
                                x0 if x0 == 14 => Some(Op::Ldnf1SbZPBiS16),
                                x0 if x0 == 15 => Some(Op::Ldnf1DZPBiU64),
                                _ => None,
                            }
                        }
                        (_, _, _, x3, _, x5, _) if x3 == 1 && x5 == 7 => None,
                        (_, _, _, _, _, x5, _) if x5 == 0 => {
                            let msz = (instr >> 23) & 3;
                            let ssz = (instr >> 21) & 3;
                            let Rm = (instr >> 16) & 9;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match (msz, ssz) {
                                (_, x1) if x1 & 2 == 2 => None,
                                (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::Ld1RqbZPBrContiguous),
                                (x0, x1) if x0 == 0 && x1 == 1 => Some(Op::Ld1RobZPBrContiguous),
                                (x0, x1) if x0 == 1 && x1 == 0 => Some(Op::Ld1RqhZPBrContiguous),
                                (x0, x1) if x0 == 1 && x1 == 1 => Some(Op::Ld1RohZPBrContiguous),
                                (x0, x1) if x0 == 2 && x1 == 0 => Some(Op::Ld1RqwZPBrContiguous),
                                (x0, x1) if x0 == 2 && x1 == 1 => Some(Op::Ld1RowZPBrContiguous),
                                (x0, x1) if x0 == 3 && x1 == 0 => Some(Op::Ld1RqdZPBrContiguous),
                                (x0, x1) if x0 == 3 && x1 == 1 => Some(Op::Ld1RodZPBrContiguous),
                                _ => None,
                            }
                        }
                        (_, _, _, _, _, x5, _) if x5 == 2 => {
                            let dtype = (instr >> 21) & 7;
                            let Rm = (instr >> 16) & 9;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match dtype {
                                x0 if x0 == 0 => Some(Op::Ld1BZPBrU8),
                                x0 if x0 == 1 => Some(Op::Ld1BZPBrU16),
                                x0 if x0 == 2 => Some(Op::Ld1BZPBrU32),
                                x0 if x0 == 3 => Some(Op::Ld1BZPBrU64),
                                x0 if x0 == 4 => Some(Op::Ld1SwZPBrS64),
                                x0 if x0 == 5 => Some(Op::Ld1HZPBrU16),
                                x0 if x0 == 6 => Some(Op::Ld1HZPBrU32),
                                x0 if x0 == 7 => Some(Op::Ld1HZPBrU64),
                                x0 if x0 == 8 => Some(Op::Ld1ShZPBrS64),
                                x0 if x0 == 9 => Some(Op::Ld1ShZPBrS32),
                                x0 if x0 == 10 => Some(Op::Ld1WZPBrU32),
                                x0 if x0 == 11 => Some(Op::Ld1WZPBrU64),
                                x0 if x0 == 12 => Some(Op::Ld1SbZPBrS64),
                                x0 if x0 == 13 => Some(Op::Ld1SbZPBrS32),
                                x0 if x0 == 14 => Some(Op::Ld1SbZPBrS16),
                                x0 if x0 == 15 => Some(Op::Ld1DZPBrU64),
                                _ => None,
                            }
                        }
                        (_, _, _, _, _, x5, _) if x5 == 3 => {
                            let dtype = (instr >> 21) & 7;
                            let Rm = (instr >> 16) & 9;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match dtype {
                                x0 if x0 == 0 => Some(Op::Ldff1BZPBrU8),
                                x0 if x0 == 1 => Some(Op::Ldff1BZPBrU16),
                                x0 if x0 == 2 => Some(Op::Ldff1BZPBrU32),
                                x0 if x0 == 3 => Some(Op::Ldff1BZPBrU64),
                                x0 if x0 == 4 => Some(Op::Ldff1SwZPBrS64),
                                x0 if x0 == 5 => Some(Op::Ldff1HZPBrU16),
                                x0 if x0 == 6 => Some(Op::Ldff1HZPBrU32),
                                x0 if x0 == 7 => Some(Op::Ldff1HZPBrU64),
                                x0 if x0 == 8 => Some(Op::Ldff1ShZPBrS64),
                                x0 if x0 == 9 => Some(Op::Ldff1ShZPBrS32),
                                x0 if x0 == 10 => Some(Op::Ldff1WZPBrU32),
                                x0 if x0 == 11 => Some(Op::Ldff1WZPBrU64),
                                x0 if x0 == 12 => Some(Op::Ldff1SbZPBrS64),
                                x0 if x0 == 13 => Some(Op::Ldff1SbZPBrS32),
                                x0 if x0 == 14 => Some(Op::Ldff1SbZPBrS16),
                                x0 if x0 == 15 => Some(Op::Ldff1DZPBrU64),
                                _ => None,
                            }
                        }
                        (_, _, _, _, _, x5, _) if x5 == 4 => None,
                        _ => None,
                    }
                }
                (x0, _, _, _, _, _, _, _) if x0 == 6 => {
                    match (
                        (instr >> 25) & 13,
                        (instr >> 23) & 3,
                        (instr >> 21) & 3,
                        (instr >> 16) & 9,
                        (instr >> 13) & 5,
                        (instr >> 5) & 15,
                        (instr >> 4) & 1,
                        instr & 7,
                    ) {
                        (_, x1, x2, _, x4, _, x6, _)
                            if x1 == 0 && x2 == 1 && x4 & 4 == 0 && x6 == 1 =>
                        {
                            None
                        }
                        (_, x1, x2, _, x4, _, x6, _)
                            if x1 == 0 && x2 == 3 && x4 & 4 == 4 && x6 == 0 =>
                        {
                            let Zm = (instr >> 16) & 9;
                            let msz = (instr >> 13) & 3;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let prfop = instr & 7;
                            match msz {
                                x0 if x0 == 0 => Some(Op::PrfbIPBzD64Scaled),
                                x0 if x0 == 1 => Some(Op::PrfhIPBzD64Scaled),
                                x0 if x0 == 2 => Some(Op::PrfwIPBzD64Scaled),
                                x0 if x0 == 3 => Some(Op::PrfdIPBzD64Scaled),
                                _ => None,
                            }
                        }
                        (_, x1, x2, _, _, _, x6, _) if x1 == 0 && x2 == 3 && x6 == 1 => None,
                        (_, x1, x2, _, x4, _, x6, _)
                            if x1 == 0 && x2 & 1 == 1 && x4 & 4 == 0 && x6 == 0 =>
                        {
                            let xs = (instr >> 22) & 1;
                            let Zm = (instr >> 16) & 9;
                            let msz = (instr >> 13) & 3;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let prfop = instr & 7;
                            match msz {
                                x0 if x0 == 0 => Some(Op::PrfbIPBzDX32Scaled),
                                x0 if x0 == 1 => Some(Op::PrfhIPBzDX32Scaled),
                                x0 if x0 == 2 => Some(Op::PrfwIPBzDX32Scaled),
                                x0 if x0 == 3 => Some(Op::PrfdIPBzDX32Scaled),
                                _ => None,
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
                                    Some(Op::Ld1ShZPBzD64Scaled)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    Some(Op::Ldff1ShZPBzD64Scaled)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    Some(Op::Ld1HZPBzD64Scaled)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    Some(Op::Ldff1HZPBzD64Scaled)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                                    Some(Op::Ld1SwZPBzD64Scaled)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                                    Some(Op::Ldff1SwZPBzD64Scaled)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                                    Some(Op::Ld1WZPBzD64Scaled)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                                    Some(Op::Ldff1WZPBzD64Scaled)
                                }
                                (x0, x1, _) if x0 == 3 && x1 == 0 => None,
                                (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 0 => {
                                    Some(Op::Ld1DZPBzD64Scaled)
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 1 => {
                                    Some(Op::Ldff1DZPBzD64Scaled)
                                }
                                _ => None,
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
                                    Some(Op::Ld1ShZPBzDX32Scaled)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    Some(Op::Ldff1ShZPBzDX32Scaled)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    Some(Op::Ld1HZPBzDX32Scaled)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    Some(Op::Ldff1HZPBzDX32Scaled)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                                    Some(Op::Ld1SwZPBzDX32Scaled)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                                    Some(Op::Ldff1SwZPBzDX32Scaled)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                                    Some(Op::Ld1WZPBzDX32Scaled)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                                    Some(Op::Ldff1WZPBzDX32Scaled)
                                }
                                (x0, x1, _) if x0 == 3 && x1 == 0 => None,
                                (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 0 => {
                                    Some(Op::Ld1DZPBzDX32Scaled)
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 1 => {
                                    Some(Op::Ldff1DZPBzDX32Scaled)
                                }
                                _ => None,
                            }
                        }
                        (_, _, x2, _, x4, _, _, _) if x2 == 0 && x4 & 6 == 4 => None,
                        (_, _, x2, _, x4, _, _, _) if x2 == 0 && x4 == 6 => None,
                        (_, _, x2, _, x4, _, x6, _) if x2 == 0 && x4 == 7 && x6 == 0 => {
                            let msz = (instr >> 23) & 3;
                            let imm5 = (instr >> 16) & 9;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let prfop = instr & 7;
                            match msz {
                                x0 if x0 == 0 => Some(Op::PrfbIPAiD),
                                x0 if x0 == 1 => Some(Op::PrfhIPAiD),
                                x0 if x0 == 2 => Some(Op::PrfwIPAiD),
                                x0 if x0 == 3 => Some(Op::PrfdIPAiD),
                                _ => None,
                            }
                        }
                        (_, _, x2, _, x4, _, x6, _) if x2 == 0 && x4 == 7 && x6 == 1 => None,
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
                                    Some(Op::Ld1SbZPAiD)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                                    Some(Op::Ldff1SbZPAiD)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    Some(Op::Ld1BZPAiD)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    Some(Op::Ldff1BZPAiD)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    Some(Op::Ld1ShZPAiD)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    Some(Op::Ldff1ShZPAiD)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    Some(Op::Ld1HZPAiD)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    Some(Op::Ldff1HZPAiD)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                                    Some(Op::Ld1SwZPAiD)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                                    Some(Op::Ldff1SwZPAiD)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                                    Some(Op::Ld1WZPAiD)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                                    Some(Op::Ldff1WZPAiD)
                                }
                                (x0, x1, _) if x0 == 3 && x1 == 0 => None,
                                (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 0 => {
                                    Some(Op::Ld1DZPAiD)
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 1 => {
                                    Some(Op::Ldff1DZPAiD)
                                }
                                _ => None,
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
                                    Some(Op::Ld1SbZPBzD64Unscaled)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                                    Some(Op::Ldff1SbZPBzD64Unscaled)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    Some(Op::Ld1BZPBzD64Unscaled)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    Some(Op::Ldff1BZPBzD64Unscaled)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    Some(Op::Ld1ShZPBzD64Unscaled)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    Some(Op::Ldff1ShZPBzD64Unscaled)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    Some(Op::Ld1HZPBzD64Unscaled)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    Some(Op::Ldff1HZPBzD64Unscaled)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                                    Some(Op::Ld1SwZPBzD64Unscaled)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                                    Some(Op::Ldff1SwZPBzD64Unscaled)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                                    Some(Op::Ld1WZPBzD64Unscaled)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                                    Some(Op::Ldff1WZPBzD64Unscaled)
                                }
                                (x0, x1, _) if x0 == 3 && x1 == 0 => None,
                                (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 0 => {
                                    Some(Op::Ld1DZPBzD64Unscaled)
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 1 => {
                                    Some(Op::Ldff1DZPBzD64Unscaled)
                                }
                                _ => None,
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
                                    Some(Op::Ld1SbZPBzDX32Unscaled)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                                    Some(Op::Ldff1SbZPBzDX32Unscaled)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    Some(Op::Ld1BZPBzDX32Unscaled)
                                }
                                (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    Some(Op::Ldff1BZPBzDX32Unscaled)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    Some(Op::Ld1ShZPBzDX32Unscaled)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    Some(Op::Ldff1ShZPBzDX32Unscaled)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    Some(Op::Ld1HZPBzDX32Unscaled)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    Some(Op::Ldff1HZPBzDX32Unscaled)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                                    Some(Op::Ld1SwZPBzDX32Unscaled)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                                    Some(Op::Ldff1SwZPBzDX32Unscaled)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                                    Some(Op::Ld1WZPBzDX32Unscaled)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                                    Some(Op::Ldff1WZPBzDX32Unscaled)
                                }
                                (x0, x1, _) if x0 == 3 && x1 == 0 => None,
                                (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 0 => {
                                    Some(Op::Ld1DZPBzDX32Unscaled)
                                }
                                (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 1 => {
                                    Some(Op::Ldff1DZPBzDX32Unscaled)
                                }
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                (x0, _, _, _, _, _, x6, _) if x0 == 7 && x6 & 40 == 0 => {
                    match (
                        (instr >> 25) & 13,
                        (instr >> 22) & 5,
                        (instr >> 16) & 11,
                        (instr >> 15) & 1,
                        (instr >> 14) & 1,
                        (instr >> 13) & 1,
                        (instr >> 5) & 15,
                        (instr >> 4) & 1,
                        instr & 7,
                    ) {
                        (_, x1, _, _, x4, _, _, _, _) if x1 & 4 == 0 && x4 == 0 => None,
                        (_, x1, _, _, x4, _, _, _, _) if x1 & 6 == 4 && x4 == 0 => None,
                        (_, x1, _, _, x4, _, _, x7, _) if x1 == 6 && x4 == 0 && x7 == 0 => {
                            let imm9h = (instr >> 16) & 11;
                            let imm9l = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Pt = instr & 7;
                            match () {
                                () => Some(Op::StrPBi),
                            }
                        }
                        (_, x1, _, _, x4, _, _, x7, _) if x1 == 6 && x4 == 0 && x7 == 1 => None,
                        (_, x1, _, _, x4, _, _, _, _) if x1 == 6 && x4 == 1 => {
                            let imm9h = (instr >> 16) & 11;
                            let imm9l = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match () {
                                () => Some(Op::StrZBi),
                            }
                        }
                        (_, x1, _, _, x4, _, _, _, _) if x1 == 7 && x4 == 0 => None,
                        (_, x1, _, _, x4, _, _, _, _) if x1 != 6 && x4 == 1 => {
                            let opc = (instr >> 22) & 5;
                            let o2 = (instr >> 21) & 1;
                            let Rm = (instr >> 16) & 9;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match (opc, o2) {
                                (x0, _) if x0 & 6 == 0 => Some(Op::St1BZPBr),
                                (x0, _) if x0 & 6 == 2 => Some(Op::St1HZPBr),
                                (x0, _) if x0 & 6 == 4 => Some(Op::St1WZPBr),
                                (x0, x1) if x0 == 7 && x1 == 0 => None,
                                (x0, x1) if x0 == 7 && x1 == 1 => Some(Op::St1DZPBr),
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                (x0, _, _, _, _, _, x6, _) if x0 == 7 && x6 & 40 == 8 => {
                    match (
                        (instr >> 25) & 13,
                        (instr >> 23) & 3,
                        (instr >> 21) & 3,
                        (instr >> 16) & 9,
                        (instr >> 15) & 1,
                        (instr >> 14) & 1,
                        (instr >> 13) & 1,
                        instr & 25,
                    ) {
                        (_, _, x2, _, _, x5, _, _) if x2 == 0 && x5 == 1 => {
                            let msz = (instr >> 23) & 3;
                            let Rm = (instr >> 16) & 9;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match msz {
                                x0 if x0 == 0 => Some(Op::Stnt1BZPBrContiguous),
                                x0 if x0 == 1 => Some(Op::Stnt1HZPBrContiguous),
                                x0 if x0 == 2 => Some(Op::Stnt1WZPBrContiguous),
                                x0 if x0 == 3 => Some(Op::Stnt1DZPBrContiguous),
                                _ => None,
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
                                (x0, x1) if x0 == 0 && x1 == 1 => Some(Op::St2BZPBrContiguous),
                                (x0, x1) if x0 == 0 && x1 == 2 => Some(Op::St3BZPBrContiguous),
                                (x0, x1) if x0 == 0 && x1 == 3 => Some(Op::St4BZPBrContiguous),
                                (x0, x1) if x0 == 1 && x1 == 1 => Some(Op::St2HZPBrContiguous),
                                (x0, x1) if x0 == 1 && x1 == 2 => Some(Op::St3HZPBrContiguous),
                                (x0, x1) if x0 == 1 && x1 == 3 => Some(Op::St4HZPBrContiguous),
                                (x0, x1) if x0 == 2 && x1 == 1 => Some(Op::St2WZPBrContiguous),
                                (x0, x1) if x0 == 2 && x1 == 2 => Some(Op::St3WZPBrContiguous),
                                (x0, x1) if x0 == 2 && x1 == 3 => Some(Op::St4WZPBrContiguous),
                                (x0, x1) if x0 == 3 && x1 == 1 => Some(Op::St2DZPBrContiguous),
                                (x0, x1) if x0 == 3 && x1 == 2 => Some(Op::St3DZPBrContiguous),
                                (x0, x1) if x0 == 3 && x1 == 3 => Some(Op::St4DZPBrContiguous),
                                _ => None,
                            }
                        }
                        (_, _, _, _, _, x5, _, _) if x5 == 0 => None,
                        _ => None,
                    }
                }
                (x0, _, _, _, _, _, x6, _) if x0 == 7 && x6 & 40 == 32 => {
                    match (
                        (instr >> 25) & 13,
                        (instr >> 23) & 3,
                        (instr >> 21) & 3,
                        (instr >> 16) & 9,
                        (instr >> 15) & 1,
                        (instr >> 14) & 1,
                        (instr >> 13) & 1,
                        instr & 25,
                    ) {
                        (_, _, x2, _, _, _, _, _) if x2 == 0 => {
                            let msz = (instr >> 23) & 3;
                            let Zm = (instr >> 16) & 9;
                            let xs = (instr >> 14) & 1;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match msz {
                                x0 if x0 == 0 => Some(Op::St1BZPBzDX32Unscaled),
                                x0 if x0 == 1 => Some(Op::St1HZPBzDX32Unscaled),
                                x0 if x0 == 2 => Some(Op::St1WZPBzDX32Unscaled),
                                x0 if x0 == 3 => Some(Op::St1DZPBzDX32Unscaled),
                                _ => None,
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
                                x0 if x0 == 0 => None,
                                x0 if x0 == 1 => Some(Op::St1HZPBzDX32Scaled),
                                x0 if x0 == 2 => Some(Op::St1WZPBzDX32Scaled),
                                x0 if x0 == 3 => Some(Op::St1DZPBzDX32Scaled),
                                _ => None,
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
                                x0 if x0 == 0 => Some(Op::St1BZPBzSX32Unscaled),
                                x0 if x0 == 1 => Some(Op::St1HZPBzSX32Unscaled),
                                x0 if x0 == 2 => Some(Op::St1WZPBzSX32Unscaled),
                                x0 if x0 == 3 => None,
                                _ => None,
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
                                x0 if x0 == 0 => None,
                                x0 if x0 == 1 => Some(Op::St1HZPBzSX32Scaled),
                                x0 if x0 == 2 => Some(Op::St1WZPBzSX32Scaled),
                                x0 if x0 == 3 => None,
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                (x0, _, _, _, _, _, x6, _) if x0 == 7 && x6 & 56 == 40 => {
                    match (
                        (instr >> 25) & 13,
                        (instr >> 23) & 3,
                        (instr >> 21) & 3,
                        (instr >> 16) & 9,
                        (instr >> 13) & 5,
                        instr & 25,
                    ) {
                        (_, _, x2, _, _, _) if x2 == 0 => {
                            let msz = (instr >> 23) & 3;
                            let Zm = (instr >> 16) & 9;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match msz {
                                x0 if x0 == 0 => Some(Op::St1BZPBzD64Unscaled),
                                x0 if x0 == 1 => Some(Op::St1HZPBzD64Unscaled),
                                x0 if x0 == 2 => Some(Op::St1WZPBzD64Unscaled),
                                x0 if x0 == 3 => Some(Op::St1DZPBzD64Unscaled),
                                _ => None,
                            }
                        }
                        (_, _, x2, _, _, _) if x2 == 1 => {
                            let msz = (instr >> 23) & 3;
                            let Zm = (instr >> 16) & 9;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match msz {
                                x0 if x0 == 0 => None,
                                x0 if x0 == 1 => Some(Op::St1HZPBzD64Scaled),
                                x0 if x0 == 2 => Some(Op::St1WZPBzD64Scaled),
                                x0 if x0 == 3 => Some(Op::St1DZPBzD64Scaled),
                                _ => None,
                            }
                        }
                        (_, _, x2, _, _, _) if x2 == 2 => {
                            let msz = (instr >> 23) & 3;
                            let imm5 = (instr >> 16) & 9;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match msz {
                                x0 if x0 == 0 => Some(Op::St1BZPAiD),
                                x0 if x0 == 1 => Some(Op::St1HZPAiD),
                                x0 if x0 == 2 => Some(Op::St1WZPAiD),
                                x0 if x0 == 3 => Some(Op::St1DZPAiD),
                                _ => None,
                            }
                        }
                        (_, _, x2, _, _, _) if x2 == 3 => {
                            let msz = (instr >> 23) & 3;
                            let imm5 = (instr >> 16) & 9;
                            let Pg = (instr >> 10) & 5;
                            let Zn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match msz {
                                x0 if x0 == 0 => Some(Op::St1BZPAiS),
                                x0 if x0 == 1 => Some(Op::St1HZPAiS),
                                x0 if x0 == 2 => Some(Op::St1WZPAiS),
                                x0 if x0 == 3 => None,
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                (x0, _, _, _, _, _, x6, _) if x0 == 7 && x6 & 56 == 56 => {
                    match (
                        (instr >> 25) & 13,
                        (instr >> 23) & 3,
                        (instr >> 21) & 3,
                        (instr >> 20) & 1,
                        (instr >> 16) & 7,
                        (instr >> 13) & 5,
                        instr & 25,
                    ) {
                        (_, _, x2, x3, _, _, _) if x2 == 0 && x3 == 1 => {
                            let msz = (instr >> 23) & 3;
                            let imm4 = (instr >> 16) & 7;
                            let Pg = (instr >> 10) & 5;
                            let Rn = (instr >> 5) & 9;
                            let Zt = instr & 9;
                            match msz {
                                x0 if x0 == 0 => Some(Op::Stnt1BZPBiContiguous),
                                x0 if x0 == 1 => Some(Op::Stnt1HZPBiContiguous),
                                x0 if x0 == 2 => Some(Op::Stnt1WZPBiContiguous),
                                x0 if x0 == 3 => Some(Op::Stnt1DZPBiContiguous),
                                _ => None,
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
                                (x0, x1) if x0 == 0 && x1 == 1 => Some(Op::St2BZPBiContiguous),
                                (x0, x1) if x0 == 0 && x1 == 2 => Some(Op::St3BZPBiContiguous),
                                (x0, x1) if x0 == 0 && x1 == 3 => Some(Op::St4BZPBiContiguous),
                                (x0, x1) if x0 == 1 && x1 == 1 => Some(Op::St2HZPBiContiguous),
                                (x0, x1) if x0 == 1 && x1 == 2 => Some(Op::St3HZPBiContiguous),
                                (x0, x1) if x0 == 1 && x1 == 3 => Some(Op::St4HZPBiContiguous),
                                (x0, x1) if x0 == 2 && x1 == 1 => Some(Op::St2WZPBiContiguous),
                                (x0, x1) if x0 == 2 && x1 == 2 => Some(Op::St3WZPBiContiguous),
                                (x0, x1) if x0 == 2 && x1 == 3 => Some(Op::St4WZPBiContiguous),
                                (x0, x1) if x0 == 3 && x1 == 1 => Some(Op::St2DZPBiContiguous),
                                (x0, x1) if x0 == 3 && x1 == 2 => Some(Op::St3DZPBiContiguous),
                                (x0, x1) if x0 == 3 && x1 == 3 => Some(Op::St4DZPBiContiguous),
                                _ => None,
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
                                x0 if x0 == 0 => Some(Op::St1BZPBi),
                                x0 if x0 == 1 => Some(Op::St1HZPBi),
                                x0 if x0 == 2 => Some(Op::St1WZPBi),
                                x0 if x0 == 3 => Some(Op::St1DZPBi),
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                _ => None,
            }
        }
        (_, x1, _) if x1 & 30 == 6 => None,
        (_, x1, _) if x1 & 28 == 16 => {
            match (
                (instr >> 29) & 5,
                (instr >> 26) & 5,
                (instr >> 23) & 5,
                instr & 45,
            ) {
                (_, _, x2, _) if x2 & 6 == 0 => {
                    let op = (instr >> 31) & 1;
                    let immlo = (instr >> 29) & 3;
                    let immhi = (instr >> 5) & 37;
                    let Rd = instr & 9;
                    match op {
                        x0 if x0 == 0 => Some(Op::AdrpOnlyPcreladdr),
                        x0 if x0 == 1 => Some(Op::AdrpOnlyPcreladdr),
                        _ => None,
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
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => Some(Op::Subs64SAddsubImm),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => Some(Op::Subs64SAddsubImm),
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => Some(Op::Subs64SAddsubImm),
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => Some(Op::Subs64SAddsubImm),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => Some(Op::Subs64SAddsubImm),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => Some(Op::Subs64SAddsubImm),
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => Some(Op::Subs64SAddsubImm),
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => Some(Op::Subs64SAddsubImm),
                        _ => None,
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
                        (_, _, _, x3) if x3 == 1 => None,
                        (x0, _, _, x3) if x0 == 0 && x3 == 0 => None,
                        (x0, _, x2, x3) if x0 == 1 && x2 == 1 && x3 == 0 => None,
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            Some(Op::Addg64AddsubImmtags)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 0 => {
                            Some(Op::Subg64AddsubImmtags)
                        }
                        _ => None,
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
                        (x0, _, x2) if x0 == 0 && x2 == 1 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => Some(Op::Ands64SLogImm),
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => Some(Op::Ands64SLogImm),
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 0 => Some(Op::Ands64SLogImm),
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 0 => Some(Op::Ands64SLogImm),
                        (x0, x1, _) if x0 == 1 && x1 == 0 => Some(Op::Ands64SLogImm),
                        (x0, x1, _) if x0 == 1 && x1 == 1 => Some(Op::Ands64SLogImm),
                        (x0, x1, _) if x0 == 1 && x1 == 2 => Some(Op::Ands64SLogImm),
                        (x0, x1, _) if x0 == 1 && x1 == 3 => Some(Op::Ands64SLogImm),
                        _ => None,
                    }
                }
                (_, _, x2, _) if x2 == 5 => {
                    let sf = (instr >> 31) & 1;
                    let opc = (instr >> 29) & 3;
                    let hw = (instr >> 21) & 3;
                    let imm16 = (instr >> 5) & 31;
                    let Rd = instr & 9;
                    match (sf, opc, hw) {
                        (_, x1, _) if x1 == 1 => None,
                        (x0, _, x2) if x0 == 0 && x2 & 2 == 2 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 & 2 == 0 => {
                            Some(Op::Movk64Movewide)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 & 2 == 0 => {
                            Some(Op::Movk64Movewide)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 & 2 == 0 => {
                            Some(Op::Movk64Movewide)
                        }
                        (x0, x1, _) if x0 == 1 && x1 == 0 => Some(Op::Movk64Movewide),
                        (x0, x1, _) if x0 == 1 && x1 == 2 => Some(Op::Movk64Movewide),
                        (x0, x1, _) if x0 == 1 && x1 == 3 => Some(Op::Movk64Movewide),
                        _ => None,
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
                        (_, x1, _) if x1 == 3 => None,
                        (x0, _, x2) if x0 == 0 && x2 == 1 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => Some(Op::Ubfm64MBitfield),
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => Some(Op::Ubfm64MBitfield),
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 0 => Some(Op::Ubfm64MBitfield),
                        (x0, _, x2) if x0 == 1 && x2 == 0 => None,
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => Some(Op::Ubfm64MBitfield),
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => Some(Op::Ubfm64MBitfield),
                        (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 == 1 => Some(Op::Ubfm64MBitfield),
                        _ => None,
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
                        (_, x1, _, _, _) if x1 & 1 == 1 => None,
                        (_, x1, _, x3, _) if x1 == 0 && x3 == 1 => None,
                        (_, x1, _, _, _) if x1 & 2 == 2 => None,
                        (x0, _, _, _, x4) if x0 == 0 && x4 & 32 == 32 => None,
                        (x0, _, x2, _, _) if x0 == 0 && x2 == 1 => None,
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 & 32 == 0 =>
                        {
                            Some(Op::Extr64Extract)
                        }
                        (x0, _, x2, _, _) if x0 == 1 && x2 == 0 => None,
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            Some(Op::Extr64Extract)
                        }
                        _ => None,
                    }
                }
                _ => None,
            }
        }
        (_, x1, _) if x1 & 28 == 20 => {
            match (
                (instr >> 29) & 5,
                (instr >> 26) & 5,
                (instr >> 12) & 27,
                (instr >> 5) & 13,
                instr & 9,
            ) {
                (x0, _, x2, _, _) if x0 == 2 && x2 & 8192 == 0 => {
                    let o1 = (instr >> 24) & 1;
                    let imm19 = (instr >> 5) & 37;
                    let o0 = (instr >> 4) & 1;
                    let cond = instr & 7;
                    match (o1, o0) {
                        (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::BOnlyCondbranch),
                        (x0, x1) if x0 == 0 && x1 == 1 => None,
                        (x0, _) if x0 == 1 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, _, _) if x0 == 6 && x2 & 12288 == 0 => {
                    let opc = (instr >> 21) & 5;
                    let imm16 = (instr >> 5) & 31;
                    let op2 = (instr >> 2) & 5;
                    let LL = instr & 3;
                    match (opc, op2, LL) {
                        (_, x1, _) if x1 == 1 => None,
                        (_, x1, _) if x1 & 6 == 2 => None,
                        (_, x1, _) if x1 & 4 == 4 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => Some(Op::SvcExException),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 2 => Some(Op::HvcExException),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 3 => Some(Op::SmcExException),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 & 1 == 1 => None,
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => Some(Op::BrkExException),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 & 2 == 2 => None,
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 & 1 == 1 => None,
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => Some(Op::HltExException),
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 & 2 == 2 => None,
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 1 => None,
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 & 2 == 2 => None,
                        (x0, x1, _) if x0 == 4 && x1 == 0 => None,
                        (x0, x1, x2) if x0 == 5 && x1 == 0 && x2 == 0 => None,
                        (x0, x1, x2) if x0 == 5 && x1 == 0 && x2 == 1 => Some(Op::Dcps3DcException),
                        (x0, x1, x2) if x0 == 5 && x1 == 0 && x2 == 2 => Some(Op::Dcps3DcException),
                        (x0, x1, x2) if x0 == 5 && x1 == 0 && x2 == 3 => Some(Op::Dcps3DcException),
                        (x0, x1, _) if x0 == 6 && x1 == 0 => None,
                        (x0, x1, _) if x0 == 7 && x1 == 0 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4) if x0 == 6 && x2 == 4146 && x4 == 31 => {
                    let CRm = (instr >> 8) & 7;
                    let op2 = (instr >> 5) & 5;
                    match (CRm, op2) {
                        (_, _) => Some(Op::BtiHbHints),
                        (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::BtiHbHints),
                        (x0, x1) if x0 == 0 && x1 == 1 => Some(Op::BtiHbHints),
                        (x0, x1) if x0 == 0 && x1 == 2 => Some(Op::BtiHbHints),
                        (x0, x1) if x0 == 0 && x1 == 3 => Some(Op::BtiHbHints),
                        (x0, x1) if x0 == 0 && x1 == 4 => Some(Op::BtiHbHints),
                        (x0, x1) if x0 == 0 && x1 == 5 => Some(Op::BtiHbHints),
                        (x0, x1) if x0 == 0 && x1 == 6 => Some(Op::BtiHbHints),
                        (x0, x1) if x0 == 0 && x1 == 7 => Some(Op::XpaclriHiHints),
                        (x0, x1) if x0 == 1 && x1 == 0 => Some(Op::PaciaspHiHints),
                        (x0, x1) if x0 == 1 && x1 == 2 => Some(Op::PacibspHiHints),
                        (x0, x1) if x0 == 1 && x1 == 4 => Some(Op::AutiaspHiHints),
                        (x0, x1) if x0 == 1 && x1 == 6 => Some(Op::AutibspHiHints),
                        (x0, x1) if x0 == 2 && x1 == 0 => Some(Op::BtiHbHints),
                        (x0, x1) if x0 == 2 && x1 == 1 => Some(Op::BtiHbHints),
                        (x0, x1) if x0 == 2 && x1 == 2 => Some(Op::BtiHbHints),
                        (x0, x1) if x0 == 2 && x1 == 4 => Some(Op::BtiHbHints),
                        (x0, x1) if x0 == 3 && x1 == 0 => Some(Op::PaciaspHiHints),
                        (x0, x1) if x0 == 3 && x1 == 1 => Some(Op::PaciaspHiHints),
                        (x0, x1) if x0 == 3 && x1 == 2 => Some(Op::PacibspHiHints),
                        (x0, x1) if x0 == 3 && x1 == 3 => Some(Op::PacibspHiHints),
                        (x0, x1) if x0 == 3 && x1 == 4 => Some(Op::AutiaspHiHints),
                        (x0, x1) if x0 == 3 && x1 == 5 => Some(Op::AutiaspHiHints),
                        (x0, x1) if x0 == 3 && x1 == 6 => Some(Op::AutibspHiHints),
                        (x0, x1) if x0 == 3 && x1 == 7 => Some(Op::AutibspHiHints),
                        (x0, x1) if x0 == 4 && x1 & 1 == 0 => Some(Op::BtiHbHints),
                        _ => None,
                    }
                }
                (x0, _, x2, _, _) if x0 == 6 && x2 == 4147 => {
                    let CRm = (instr >> 8) & 7;
                    let op2 = (instr >> 5) & 5;
                    let Rt = instr & 9;
                    match (CRm, op2, Rt) {
                        (_, x1, _) if x1 == 0 => None,
                        (_, x1, _) if x1 == 1 => None,
                        (_, x1, x2) if x1 == 2 && x2 == 31 => Some(Op::ClrexBnBarriers),
                        (_, x1, x2) if x1 == 5 && x2 == 31 => Some(Op::DmbBoBarriers),
                        (_, x1, x2) if x1 == 6 && x2 == 31 => Some(Op::IsbBiBarriers),
                        (_, x1, x2) if x1 == 7 && x2 != 31 => None,
                        (_, x1, x2) if x1 == 7 && x2 == 31 => Some(Op::SbOnlyBarriers),
                        (x0, x1, x2) if x0 & 11 != 0 && x1 == 4 && x2 == 31 => {
                            Some(Op::DsbBoBarriers)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 4 && x2 == 31 => {
                            Some(Op::SsbbOnlyBarriers)
                        }
                        (x0, x1, _) if x0 == 1 && x1 == 3 => None,
                        (x0, x1, _) if x0 & 14 == 2 && x1 == 3 => None,
                        (x0, x1, _) if x0 & 12 == 4 && x1 == 3 => None,
                        (x0, x1, x2) if x0 == 4 && x1 == 4 && x2 == 31 => {
                            Some(Op::PssbbOnlyBarriers)
                        }
                        (x0, x1, _) if x0 & 8 == 8 && x1 == 3 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, _, _) if x0 == 6 && x2 & 16271 == 4100 => {
                    let op1 = (instr >> 16) & 5;
                    let CRm = (instr >> 8) & 7;
                    let op2 = (instr >> 5) & 5;
                    let Rt = instr & 9;
                    match (op1, op2, Rt) {
                        (_, _, x2) if x2 != 31 => None,
                        (_, _, x2) if x2 == 31 => Some(Op::MsrSiPstate),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 31 => Some(Op::CfinvMPstate),
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 31 => Some(Op::XaflagMPstate),
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 31 => Some(Op::AxflagMPstate),
                        _ => None,
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
                        x0 if x0 == 0 => Some(Op::SyslRcSysteminstrs),
                        x0 if x0 == 1 => Some(Op::SyslRcSysteminstrs),
                        _ => None,
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
                        x0 if x0 == 0 => Some(Op::MrsRsSystemmove),
                        x0 if x0 == 1 => Some(Op::MrsRsSystemmove),
                        _ => None,
                    }
                }
                (x0, _, x2, _, _) if x0 == 6 && x2 & 8192 == 8192 => {
                    let opc = (instr >> 21) & 7;
                    let op2 = (instr >> 16) & 9;
                    let op3 = (instr >> 10) & 11;
                    let Rn = (instr >> 5) & 9;
                    let op4 = instr & 9;
                    match (opc, op2, op3, Rn, op4) {
                        (_, x1, _, _, _) if x1 != 31 => None,
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 31 && x2 == 0 && x4 != 0 => None,
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 31 && x2 == 0 && x4 == 0 => {
                            Some(Op::Blrab64PBranchReg)
                        }
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 31 && x2 == 1 => None,
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 31 && x2 == 2 && x4 != 31 => None,
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 31 && x2 == 2 && x4 == 31 => {
                            Some(Op::Blrab64PBranchReg)
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 31 && x2 == 3 && x4 != 31 => None,
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 31 && x2 == 3 && x4 == 31 => {
                            Some(Op::Blrab64PBranchReg)
                        }
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 31 && x2 & 60 == 4 => None,
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 31 && x2 & 56 == 8 => None,
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 31 && x2 & 48 == 16 => None,
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 31 && x2 & 32 == 32 => None,
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 31 && x2 == 0 && x4 != 0 => None,
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 31 && x2 == 0 && x4 == 0 => {
                            Some(Op::Blrab64PBranchReg)
                        }
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 31 && x2 == 1 => None,
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 31 && x2 == 2 && x4 != 31 => None,
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 31 && x2 == 2 && x4 == 31 => {
                            Some(Op::Blrab64PBranchReg)
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 31 && x2 == 3 && x4 != 31 => None,
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 31 && x2 == 3 && x4 == 31 => {
                            Some(Op::Blrab64PBranchReg)
                        }
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 31 && x2 & 60 == 4 => None,
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 31 && x2 & 56 == 8 => None,
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 31 && x2 & 48 == 16 => None,
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 31 && x2 & 32 == 32 => None,
                        (x0, x1, x2, _, x4) if x0 == 2 && x1 == 31 && x2 == 0 && x4 != 0 => None,
                        (x0, x1, x2, _, x4) if x0 == 2 && x1 == 31 && x2 == 0 && x4 == 0 => {
                            Some(Op::Blrab64PBranchReg)
                        }
                        (x0, x1, x2, _, _) if x0 == 2 && x1 == 31 && x2 == 1 => None,
                        (x0, x1, x2, x3, x4)
                            if x0 == 2 && x1 == 31 && x2 == 2 && x3 != 31 && x4 != 31 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 2 && x1 == 31 && x2 == 2 && x3 == 31 && x4 == 31 =>
                        {
                            Some(Op::Blrab64PBranchReg)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 2 && x1 == 31 && x2 == 3 && x3 != 31 && x4 != 31 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 2 && x1 == 31 && x2 == 3 && x3 == 31 && x4 == 31 =>
                        {
                            Some(Op::Blrab64PBranchReg)
                        }
                        (x0, x1, x2, _, _) if x0 == 2 && x1 == 31 && x2 & 60 == 4 => None,
                        (x0, x1, x2, _, _) if x0 == 2 && x1 == 31 && x2 & 56 == 8 => None,
                        (x0, x1, x2, _, _) if x0 == 2 && x1 == 31 && x2 & 48 == 16 => None,
                        (x0, x1, x2, _, _) if x0 == 2 && x1 == 31 && x2 & 32 == 32 => None,
                        (x0, x1, _, _, _) if x0 == 3 && x1 == 31 => None,
                        (x0, x1, x2, x3, x4)
                            if x0 == 4 && x1 == 31 && x2 == 0 && x3 != 31 && x4 != 0 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 4 && x1 == 31 && x2 == 0 && x3 != 31 && x4 == 0 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 4 && x1 == 31 && x2 == 0 && x3 == 31 && x4 != 0 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 4 && x1 == 31 && x2 == 0 && x3 == 31 && x4 == 0 =>
                        {
                            Some(Op::Eretab64EBranchReg)
                        }
                        (x0, x1, x2, _, _) if x0 == 4 && x1 == 31 && x2 == 1 => None,
                        (x0, x1, x2, x3, x4)
                            if x0 == 4 && x1 == 31 && x2 == 2 && x3 != 31 && x4 != 31 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 4 && x1 == 31 && x2 == 2 && x3 != 31 && x4 == 31 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 4 && x1 == 31 && x2 == 2 && x3 == 31 && x4 != 31 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 4 && x1 == 31 && x2 == 2 && x3 == 31 && x4 == 31 =>
                        {
                            Some(Op::Eretab64EBranchReg)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 4 && x1 == 31 && x2 == 3 && x3 != 31 && x4 != 31 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 4 && x1 == 31 && x2 == 3 && x3 != 31 && x4 == 31 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 4 && x1 == 31 && x2 == 3 && x3 == 31 && x4 != 31 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 4 && x1 == 31 && x2 == 3 && x3 == 31 && x4 == 31 =>
                        {
                            Some(Op::Eretab64EBranchReg)
                        }
                        (x0, x1, x2, _, _) if x0 == 4 && x1 == 31 && x2 & 60 == 4 => None,
                        (x0, x1, x2, _, _) if x0 == 4 && x1 == 31 && x2 & 56 == 8 => None,
                        (x0, x1, x2, _, _) if x0 == 4 && x1 == 31 && x2 & 48 == 16 => None,
                        (x0, x1, x2, _, _) if x0 == 4 && x1 == 31 && x2 & 32 == 32 => None,
                        (x0, x1, x2, _, _) if x0 == 5 && x1 == 31 && x2 != 0 => None,
                        (x0, x1, x2, x3, x4)
                            if x0 == 5 && x1 == 31 && x2 == 0 && x3 != 31 && x4 != 0 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 5 && x1 == 31 && x2 == 0 && x3 != 31 && x4 == 0 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 5 && x1 == 31 && x2 == 0 && x3 == 31 && x4 != 0 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 5 && x1 == 31 && x2 == 0 && x3 == 31 && x4 == 0 =>
                        {
                            Some(Op::Drps64EBranchReg)
                        }
                        (x0, x1, _, _, _) if x0 & 14 == 6 && x1 == 31 => None,
                        (x0, x1, x2, _, _) if x0 == 8 && x1 == 31 && x2 & 62 == 0 => None,
                        (x0, x1, x2, _, _) if x0 == 8 && x1 == 31 && x2 == 2 => {
                            Some(Op::Blrab64PBranchReg)
                        }
                        (x0, x1, x2, _, _) if x0 == 8 && x1 == 31 && x2 == 3 => {
                            Some(Op::Blrab64PBranchReg)
                        }
                        (x0, x1, x2, _, _) if x0 == 8 && x1 == 31 && x2 & 60 == 4 => None,
                        (x0, x1, x2, _, _) if x0 == 8 && x1 == 31 && x2 & 56 == 8 => None,
                        (x0, x1, x2, _, _) if x0 == 8 && x1 == 31 && x2 & 48 == 16 => None,
                        (x0, x1, x2, _, _) if x0 == 8 && x1 == 31 && x2 & 32 == 32 => None,
                        (x0, x1, x2, _, _) if x0 == 9 && x1 == 31 && x2 & 62 == 0 => None,
                        (x0, x1, x2, _, _) if x0 == 9 && x1 == 31 && x2 == 2 => {
                            Some(Op::Blrab64PBranchReg)
                        }
                        (x0, x1, x2, _, _) if x0 == 9 && x1 == 31 && x2 == 3 => {
                            Some(Op::Blrab64PBranchReg)
                        }
                        (x0, x1, x2, _, _) if x0 == 9 && x1 == 31 && x2 & 60 == 4 => None,
                        (x0, x1, x2, _, _) if x0 == 9 && x1 == 31 && x2 & 56 == 8 => None,
                        (x0, x1, x2, _, _) if x0 == 9 && x1 == 31 && x2 & 48 == 16 => None,
                        (x0, x1, x2, _, _) if x0 == 9 && x1 == 31 && x2 & 32 == 32 => None,
                        (x0, x1, _, _, _) if x0 & 14 == 10 && x1 == 31 => None,
                        (x0, x1, _, _, _) if x0 & 12 == 12 && x1 == 31 => None,
                        _ => None,
                    }
                }
                (x0, _, _, _, _) if x0 & 3 == 0 => {
                    let op = (instr >> 31) & 1;
                    let imm26 = instr & 51;
                    match op {
                        x0 if x0 == 0 => Some(Op::BlOnlyBranchImm),
                        x0 if x0 == 1 => Some(Op::BlOnlyBranchImm),
                        _ => None,
                    }
                }
                (x0, _, x2, _, _) if x0 & 3 == 1 && x2 & 8192 == 0 => {
                    let sf = (instr >> 31) & 1;
                    let op = (instr >> 24) & 1;
                    let imm19 = (instr >> 5) & 37;
                    let Rt = instr & 9;
                    match (sf, op) {
                        (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::Cbnz64Compbranch),
                        (x0, x1) if x0 == 0 && x1 == 1 => Some(Op::Cbnz64Compbranch),
                        (x0, x1) if x0 == 1 && x1 == 0 => Some(Op::Cbnz64Compbranch),
                        (x0, x1) if x0 == 1 && x1 == 1 => Some(Op::Cbnz64Compbranch),
                        _ => None,
                    }
                }
                (x0, _, x2, _, _) if x0 & 3 == 1 && x2 & 8192 == 8192 => {
                    let b5 = (instr >> 31) & 1;
                    let op = (instr >> 24) & 1;
                    let b40 = (instr >> 19) & 9;
                    let imm14 = (instr >> 5) & 27;
                    let Rt = instr & 9;
                    match op {
                        x0 if x0 == 0 => Some(Op::TbnzOnlyTestbranch),
                        x0 if x0 == 1 => Some(Op::TbnzOnlyTestbranch),
                        _ => None,
                    }
                }
                _ => None,
            }
        }
        (_, x1, _) if x1 & 10 == 8 => {
            match (
                (instr >> 28) & 7,
                (instr >> 27) & 1,
                (instr >> 26) & 1,
                (instr >> 25) & 1,
                (instr >> 23) & 3,
                (instr >> 22) & 1,
                (instr >> 16) & 11,
                (instr >> 12) & 7,
                (instr >> 10) & 3,
                instr & 19,
            ) {
                (x0, _, x2, _, x4, _, x6, _, _, _)
                    if x0 & 11 == 0 && x2 == 1 && x4 == 0 && x6 == 0 =>
                {
                    let Q = (instr >> 30) & 1;
                    let L = (instr >> 22) & 1;
                    let opcode = (instr >> 12) & 7;
                    let size = (instr >> 10) & 3;
                    let Rn = (instr >> 5) & 9;
                    let Rt = instr & 9;
                    match (L, opcode) {
                        (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::Ld1AsisdlseR22V),
                        (x0, x1) if x0 == 0 && x1 == 1 => None,
                        (x0, x1) if x0 == 0 && x1 == 2 => Some(Op::Ld1AsisdlseR22V),
                        (x0, x1) if x0 == 0 && x1 == 3 => None,
                        (x0, x1) if x0 == 0 && x1 == 4 => Some(Op::Ld1AsisdlseR22V),
                        (x0, x1) if x0 == 0 && x1 == 5 => None,
                        (x0, x1) if x0 == 0 && x1 == 6 => Some(Op::Ld1AsisdlseR22V),
                        (x0, x1) if x0 == 0 && x1 == 7 => Some(Op::Ld1AsisdlseR22V),
                        (x0, x1) if x0 == 0 && x1 == 8 => Some(Op::Ld1AsisdlseR22V),
                        (x0, x1) if x0 == 0 && x1 == 9 => None,
                        (x0, x1) if x0 == 0 && x1 == 10 => Some(Op::Ld1AsisdlseR22V),
                        (x0, x1) if x0 == 0 && x1 == 11 => None,
                        (x0, x1) if x0 == 0 && x1 & 12 == 12 => None,
                        (x0, x1) if x0 == 1 && x1 == 0 => Some(Op::Ld1AsisdlseR22V),
                        (x0, x1) if x0 == 1 && x1 == 1 => None,
                        (x0, x1) if x0 == 1 && x1 == 2 => Some(Op::Ld1AsisdlseR22V),
                        (x0, x1) if x0 == 1 && x1 == 3 => None,
                        (x0, x1) if x0 == 1 && x1 == 4 => Some(Op::Ld1AsisdlseR22V),
                        (x0, x1) if x0 == 1 && x1 == 5 => None,
                        (x0, x1) if x0 == 1 && x1 == 6 => Some(Op::Ld1AsisdlseR22V),
                        (x0, x1) if x0 == 1 && x1 == 7 => Some(Op::Ld1AsisdlseR22V),
                        (x0, x1) if x0 == 1 && x1 == 8 => Some(Op::Ld1AsisdlseR22V),
                        (x0, x1) if x0 == 1 && x1 == 9 => None,
                        (x0, x1) if x0 == 1 && x1 == 10 => Some(Op::Ld1AsisdlseR22V),
                        (x0, x1) if x0 == 1 && x1 == 11 => None,
                        (x0, x1) if x0 == 1 && x1 & 12 == 12 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _, _, _)
                    if x0 & 11 == 0 && x2 == 1 && x4 == 1 && x6 & 32 == 0 =>
                {
                    let Q = (instr >> 30) & 1;
                    let L = (instr >> 22) & 1;
                    let Rm = (instr >> 16) & 9;
                    let opcode = (instr >> 12) & 7;
                    let size = (instr >> 10) & 3;
                    let Rn = (instr >> 5) & 9;
                    let Rt = instr & 9;
                    match (L, Rm, opcode) {
                        (x0, _, x2) if x0 == 0 && x2 == 1 => None,
                        (x0, _, x2) if x0 == 0 && x2 == 3 => None,
                        (x0, _, x2) if x0 == 0 && x2 == 5 => None,
                        (x0, _, x2) if x0 == 0 && x2 == 9 => None,
                        (x0, _, x2) if x0 == 0 && x2 == 11 => None,
                        (x0, _, x2) if x0 == 0 && x2 & 12 == 12 => None,
                        (x0, x1, x2) if x0 == 0 && x1 != 31 && x2 == 0 => {
                            Some(Op::Ld1AsisdlsepI2I2)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 31 && x2 == 2 => {
                            Some(Op::Ld1AsisdlsepI2I2)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 31 && x2 == 4 => {
                            Some(Op::Ld1AsisdlsepI2I2)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 31 && x2 == 6 => {
                            Some(Op::Ld1AsisdlsepI2I2)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 31 && x2 == 7 => {
                            Some(Op::Ld1AsisdlsepI2I2)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 31 && x2 == 8 => {
                            Some(Op::Ld1AsisdlsepI2I2)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 31 && x2 == 10 => {
                            Some(Op::Ld1AsisdlsepI2I2)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 31 && x2 == 0 => {
                            Some(Op::Ld1AsisdlsepI2I2)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 31 && x2 == 2 => {
                            Some(Op::Ld1AsisdlsepI2I2)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 31 && x2 == 4 => {
                            Some(Op::Ld1AsisdlsepI2I2)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 31 && x2 == 6 => {
                            Some(Op::Ld1AsisdlsepI2I2)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 31 && x2 == 7 => {
                            Some(Op::Ld1AsisdlsepI2I2)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 31 && x2 == 8 => {
                            Some(Op::Ld1AsisdlsepI2I2)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 31 && x2 == 10 => {
                            Some(Op::Ld1AsisdlsepI2I2)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 1 => None,
                        (x0, _, x2) if x0 == 1 && x2 == 3 => None,
                        (x0, _, x2) if x0 == 1 && x2 == 5 => None,
                        (x0, _, x2) if x0 == 1 && x2 == 9 => None,
                        (x0, _, x2) if x0 == 1 && x2 == 11 => None,
                        (x0, _, x2) if x0 == 1 && x2 & 12 == 12 => None,
                        (x0, x1, x2) if x0 == 1 && x1 != 31 && x2 == 0 => {
                            Some(Op::Ld1AsisdlsepI2I2)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 31 && x2 == 2 => {
                            Some(Op::Ld1AsisdlsepI2I2)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 31 && x2 == 4 => {
                            Some(Op::Ld1AsisdlsepI2I2)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 31 && x2 == 6 => {
                            Some(Op::Ld1AsisdlsepI2I2)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 31 && x2 == 7 => {
                            Some(Op::Ld1AsisdlsepI2I2)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 31 && x2 == 8 => {
                            Some(Op::Ld1AsisdlsepI2I2)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 31 && x2 == 10 => {
                            Some(Op::Ld1AsisdlsepI2I2)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 31 && x2 == 0 => {
                            Some(Op::Ld1AsisdlsepI2I2)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 31 && x2 == 2 => {
                            Some(Op::Ld1AsisdlsepI2I2)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 31 && x2 == 4 => {
                            Some(Op::Ld1AsisdlsepI2I2)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 31 && x2 == 6 => {
                            Some(Op::Ld1AsisdlsepI2I2)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 31 && x2 == 7 => {
                            Some(Op::Ld1AsisdlsepI2I2)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 31 && x2 == 8 => {
                            Some(Op::Ld1AsisdlsepI2I2)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 31 && x2 == 10 => {
                            Some(Op::Ld1AsisdlsepI2I2)
                        }
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _, _, _)
                    if x0 & 11 == 0 && x2 == 1 && x4 & 2 == 0 && x6 & 32 == 32 =>
                {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _, _, _)
                    if x0 & 11 == 0 && x2 == 1 && x4 == 2 && x6 & 31 == 0 =>
                {
                    let Q = (instr >> 30) & 1;
                    let L = (instr >> 22) & 1;
                    let R = (instr >> 21) & 1;
                    let opcode = (instr >> 13) & 5;
                    let S = (instr >> 12) & 1;
                    let size = (instr >> 10) & 3;
                    let Rn = (instr >> 5) & 9;
                    let Rt = instr & 9;
                    match (L, R, opcode, S, size) {
                        (x0, _, x2, _, _) if x0 == 0 && x2 & 6 == 6 => None,
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            Some(Op::Ld4RAsisdlsoR4)
                        }
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            Some(Op::Ld4RAsisdlsoR4)
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 0 && x2 == 2 && x4 & 1 == 0 => {
                            Some(Op::Ld4RAsisdlsoR4)
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 0 && x2 == 2 && x4 & 1 == 1 => None,
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x4 & 1 == 0 => {
                            Some(Op::Ld4RAsisdlsoR4)
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 0 && x2 == 3 && x4 & 1 == 1 => None,
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 0 && x2 == 4 && x4 == 0 => {
                            Some(Op::Ld4RAsisdlsoR4)
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 0 && x2 == 4 && x4 & 2 == 2 => None,
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 4 && x3 == 0 && x4 == 1 =>
                        {
                            Some(Op::Ld4RAsisdlsoR4)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 4 && x3 == 1 && x4 == 1 =>
                        {
                            None
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 0 && x2 == 5 && x4 == 0 => {
                            Some(Op::Ld4RAsisdlsoR4)
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 0 && x2 == 5 && x4 == 2 => None,
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 5 && x3 == 0 && x4 == 1 =>
                        {
                            Some(Op::Ld4RAsisdlsoR4)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 5 && x3 == 0 && x4 == 3 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 5 && x3 == 1 && x4 & 1 == 1 =>
                        {
                            None
                        }
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            Some(Op::Ld4RAsisdlsoR4)
                        }
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            Some(Op::Ld4RAsisdlsoR4)
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 1 && x2 == 2 && x4 & 1 == 0 => {
                            Some(Op::Ld4RAsisdlsoR4)
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 1 && x2 == 2 && x4 & 1 == 1 => None,
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 1 && x2 == 3 && x4 & 1 == 0 => {
                            Some(Op::Ld4RAsisdlsoR4)
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 1 && x2 == 3 && x4 & 1 == 1 => None,
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 1 && x2 == 4 && x4 == 0 => {
                            Some(Op::Ld4RAsisdlsoR4)
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 1 && x2 == 4 && x4 == 2 => None,
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 1 && x2 == 4 && x3 == 0 && x4 == 1 =>
                        {
                            Some(Op::Ld4RAsisdlsoR4)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 1 && x2 == 4 && x3 == 0 && x4 == 3 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 1 && x2 == 4 && x3 == 1 && x4 & 1 == 1 =>
                        {
                            None
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 1 && x2 == 5 && x4 == 0 => {
                            Some(Op::Ld4RAsisdlsoR4)
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 1 && x2 == 5 && x4 == 2 => None,
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 1 && x2 == 5 && x3 == 0 && x4 == 1 =>
                        {
                            Some(Op::Ld4RAsisdlsoR4)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 1 && x2 == 5 && x3 == 0 && x4 == 3 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 1 && x2 == 5 && x3 == 1 && x4 & 1 == 1 =>
                        {
                            None
                        }
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            Some(Op::Ld4RAsisdlsoR4)
                        }
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            Some(Op::Ld4RAsisdlsoR4)
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 0 && x2 == 2 && x4 & 1 == 0 => {
                            Some(Op::Ld4RAsisdlsoR4)
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 0 && x2 == 2 && x4 & 1 == 1 => None,
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 0 && x2 == 3 && x4 & 1 == 0 => {
                            Some(Op::Ld4RAsisdlsoR4)
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 0 && x2 == 3 && x4 & 1 == 1 => None,
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 0 && x2 == 4 && x4 == 0 => {
                            Some(Op::Ld4RAsisdlsoR4)
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 0 && x2 == 4 && x4 & 2 == 2 => None,
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 4 && x3 == 0 && x4 == 1 =>
                        {
                            Some(Op::Ld4RAsisdlsoR4)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 4 && x3 == 1 && x4 == 1 =>
                        {
                            None
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 0 && x2 == 5 && x4 == 0 => {
                            Some(Op::Ld4RAsisdlsoR4)
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 0 && x2 == 5 && x4 == 2 => None,
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 5 && x3 == 0 && x4 == 1 =>
                        {
                            Some(Op::Ld4RAsisdlsoR4)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 5 && x3 == 0 && x4 == 3 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 5 && x3 == 1 && x4 & 1 == 1 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 6 && x3 == 0 => {
                            Some(Op::Ld4RAsisdlsoR4)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 6 && x3 == 1 => None,
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 7 && x3 == 0 => {
                            Some(Op::Ld4RAsisdlsoR4)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 7 && x3 == 1 => None,
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            Some(Op::Ld4RAsisdlsoR4)
                        }
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            Some(Op::Ld4RAsisdlsoR4)
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 1 && x2 == 2 && x4 & 1 == 0 => {
                            Some(Op::Ld4RAsisdlsoR4)
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 1 && x2 == 2 && x4 & 1 == 1 => None,
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 1 && x2 == 3 && x4 & 1 == 0 => {
                            Some(Op::Ld4RAsisdlsoR4)
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 1 && x2 == 3 && x4 & 1 == 1 => None,
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 1 && x2 == 4 && x4 == 0 => {
                            Some(Op::Ld4RAsisdlsoR4)
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 1 && x2 == 4 && x4 == 2 => None,
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 1 && x2 == 4 && x3 == 0 && x4 == 1 =>
                        {
                            Some(Op::Ld4RAsisdlsoR4)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 1 && x2 == 4 && x3 == 0 && x4 == 3 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 1 && x2 == 4 && x3 == 1 && x4 & 1 == 1 =>
                        {
                            None
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 1 && x2 == 5 && x4 == 0 => {
                            Some(Op::Ld4RAsisdlsoR4)
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 1 && x2 == 5 && x4 == 2 => None,
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 1 && x2 == 5 && x3 == 0 && x4 == 1 =>
                        {
                            Some(Op::Ld4RAsisdlsoR4)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 1 && x2 == 5 && x3 == 0 && x4 == 3 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 1 && x2 == 5 && x3 == 1 && x4 & 1 == 1 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 1 && x2 == 6 && x3 == 0 => {
                            Some(Op::Ld4RAsisdlsoR4)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 1 && x2 == 6 && x3 == 1 => None,
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 1 && x2 == 7 && x3 == 0 => {
                            Some(Op::Ld4RAsisdlsoR4)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 1 && x2 == 7 && x3 == 1 => None,
                        _ => None,
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
                        (x0, _, _, x3, _, _) if x0 == 0 && x3 & 6 == 6 => None,
                        (x0, x1, _, x3, _, x5) if x0 == 0 && x1 == 0 && x3 == 2 && x5 & 1 == 1 => {
                            None
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 0 && x1 == 0 && x3 == 3 && x5 & 1 == 1 => {
                            None
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 0 && x1 == 0 && x3 == 4 && x5 & 2 == 2 => {
                            None
                        }
                        (x0, x1, _, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x3 == 4 && x4 == 1 && x5 == 1 =>
                        {
                            None
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 0 && x1 == 0 && x3 == 5 && x5 == 2 => None,
                        (x0, x1, _, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x3 == 5 && x4 == 0 && x5 == 3 =>
                        {
                            None
                        }
                        (x0, x1, _, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x3 == 5 && x4 == 1 && x5 & 1 == 1 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 0 && x2 != 31 && x3 == 0 => {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 0 && x2 != 31 && x3 == 1 => {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 0 && x1 == 0 && x2 != 31 && x3 == 2 && x5 & 1 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 0 && x1 == 0 && x2 != 31 && x3 == 3 && x5 & 1 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 0 && x1 == 0 && x2 != 31 && x3 == 4 && x5 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 != 31 && x3 == 4 && x4 == 0 && x5 == 1 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 0 && x1 == 0 && x2 != 31 && x3 == 5 && x5 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 != 31 && x3 == 5 && x4 == 0 && x5 == 1 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 0 && x2 == 31 && x3 == 0 => {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 0 && x2 == 31 && x3 == 1 => {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 0 && x1 == 0 && x2 == 31 && x3 == 2 && x5 & 1 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 0 && x1 == 0 && x2 == 31 && x3 == 3 && x5 & 1 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 0 && x1 == 0 && x2 == 31 && x3 == 4 && x5 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 31 && x3 == 4 && x4 == 0 && x5 == 1 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 0 && x1 == 0 && x2 == 31 && x3 == 5 && x5 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 31 && x3 == 5 && x4 == 0 && x5 == 1 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 0 && x1 == 1 && x3 == 2 && x5 & 1 == 1 => {
                            None
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 0 && x1 == 1 && x3 == 3 && x5 & 1 == 1 => {
                            None
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 0 && x1 == 1 && x3 == 4 && x5 == 2 => None,
                        (x0, x1, _, x3, x4, x5)
                            if x0 == 0 && x1 == 1 && x3 == 4 && x4 == 0 && x5 == 3 =>
                        {
                            None
                        }
                        (x0, x1, _, x3, x4, x5)
                            if x0 == 0 && x1 == 1 && x3 == 4 && x4 == 1 && x5 & 1 == 1 =>
                        {
                            None
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 0 && x1 == 1 && x3 == 5 && x5 == 2 => None,
                        (x0, x1, _, x3, x4, x5)
                            if x0 == 0 && x1 == 1 && x3 == 5 && x4 == 0 && x5 == 3 =>
                        {
                            None
                        }
                        (x0, x1, _, x3, x4, x5)
                            if x0 == 0 && x1 == 1 && x3 == 5 && x4 == 1 && x5 & 1 == 1 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 1 && x2 != 31 && x3 == 0 => {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 1 && x2 != 31 && x3 == 1 => {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 0 && x1 == 1 && x2 != 31 && x3 == 2 && x5 & 1 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 0 && x1 == 1 && x2 != 31 && x3 == 3 && x5 & 1 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 0 && x1 == 1 && x2 != 31 && x3 == 4 && x5 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 1 && x2 != 31 && x3 == 4 && x4 == 0 && x5 == 1 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 0 && x1 == 1 && x2 != 31 && x3 == 5 && x5 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 1 && x2 != 31 && x3 == 5 && x4 == 0 && x5 == 1 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 1 && x2 == 31 && x3 == 0 => {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 1 && x2 == 31 && x3 == 1 => {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 0 && x1 == 1 && x2 == 31 && x3 == 2 && x5 & 1 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 0 && x1 == 1 && x2 == 31 && x3 == 3 && x5 & 1 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 0 && x1 == 1 && x2 == 31 && x3 == 4 && x5 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 1 && x2 == 31 && x3 == 4 && x4 == 0 && x5 == 1 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 0 && x1 == 1 && x2 == 31 && x3 == 5 && x5 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 1 && x2 == 31 && x3 == 5 && x4 == 0 && x5 == 1 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
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
                        (x0, x1, _, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x3 == 4 && x4 == 1 && x5 == 1 =>
                        {
                            None
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 1 && x1 == 0 && x3 == 5 && x5 == 2 => None,
                        (x0, x1, _, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x3 == 5 && x4 == 0 && x5 == 3 =>
                        {
                            None
                        }
                        (x0, x1, _, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x3 == 5 && x4 == 1 && x5 & 1 == 1 =>
                        {
                            None
                        }
                        (x0, x1, _, x3, x4, _) if x0 == 1 && x1 == 0 && x3 == 6 && x4 == 1 => None,
                        (x0, x1, _, x3, x4, _) if x0 == 1 && x1 == 0 && x3 == 7 && x4 == 1 => None,
                        (x0, x1, x2, x3, _, _) if x0 == 1 && x1 == 0 && x2 != 31 && x3 == 0 => {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 1 && x1 == 0 && x2 != 31 && x3 == 1 => {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 1 && x1 == 0 && x2 != 31 && x3 == 2 && x5 & 1 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 1 && x1 == 0 && x2 != 31 && x3 == 3 && x5 & 1 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 1 && x1 == 0 && x2 != 31 && x3 == 4 && x5 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 != 31 && x3 == 4 && x4 == 0 && x5 == 1 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 1 && x1 == 0 && x2 != 31 && x3 == 5 && x5 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 != 31 && x3 == 5 && x4 == 0 && x5 == 1 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 1 && x1 == 0 && x2 != 31 && x3 == 6 && x4 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 1 && x1 == 0 && x2 != 31 && x3 == 7 && x4 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 1 && x1 == 0 && x2 == 31 && x3 == 0 => {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 1 && x1 == 0 && x2 == 31 && x3 == 1 => {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 1 && x1 == 0 && x2 == 31 && x3 == 2 && x5 & 1 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 1 && x1 == 0 && x2 == 31 && x3 == 3 && x5 & 1 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 1 && x1 == 0 && x2 == 31 && x3 == 4 && x5 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 31 && x3 == 4 && x4 == 0 && x5 == 1 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 1 && x1 == 0 && x2 == 31 && x3 == 5 && x5 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 31 && x3 == 5 && x4 == 0 && x5 == 1 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 1 && x1 == 0 && x2 == 31 && x3 == 6 && x4 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 1 && x1 == 0 && x2 == 31 && x3 == 7 && x4 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 1 && x1 == 1 && x3 == 2 && x5 & 1 == 1 => {
                            None
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 1 && x1 == 1 && x3 == 3 && x5 & 1 == 1 => {
                            None
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 1 && x1 == 1 && x3 == 4 && x5 == 2 => None,
                        (x0, x1, _, x3, x4, x5)
                            if x0 == 1 && x1 == 1 && x3 == 4 && x4 == 0 && x5 == 3 =>
                        {
                            None
                        }
                        (x0, x1, _, x3, x4, x5)
                            if x0 == 1 && x1 == 1 && x3 == 4 && x4 == 1 && x5 & 1 == 1 =>
                        {
                            None
                        }
                        (x0, x1, _, x3, _, x5) if x0 == 1 && x1 == 1 && x3 == 5 && x5 == 2 => None,
                        (x0, x1, _, x3, x4, x5)
                            if x0 == 1 && x1 == 1 && x3 == 5 && x4 == 0 && x5 == 3 =>
                        {
                            None
                        }
                        (x0, x1, _, x3, x4, x5)
                            if x0 == 1 && x1 == 1 && x3 == 5 && x4 == 1 && x5 & 1 == 1 =>
                        {
                            None
                        }
                        (x0, x1, _, x3, x4, _) if x0 == 1 && x1 == 1 && x3 == 6 && x4 == 1 => None,
                        (x0, x1, _, x3, x4, _) if x0 == 1 && x1 == 1 && x3 == 7 && x4 == 1 => None,
                        (x0, x1, x2, x3, _, _) if x0 == 1 && x1 == 1 && x2 != 31 && x3 == 0 => {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 1 && x1 == 1 && x2 != 31 && x3 == 1 => {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 1 && x1 == 1 && x2 != 31 && x3 == 2 && x5 & 1 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 1 && x1 == 1 && x2 != 31 && x3 == 3 && x5 & 1 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 1 && x1 == 1 && x2 != 31 && x3 == 4 && x5 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 1 && x2 != 31 && x3 == 4 && x4 == 0 && x5 == 1 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 1 && x1 == 1 && x2 != 31 && x3 == 5 && x5 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 1 && x2 != 31 && x3 == 5 && x4 == 0 && x5 == 1 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 1 && x1 == 1 && x2 != 31 && x3 == 6 && x4 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 1 && x1 == 1 && x2 != 31 && x3 == 7 && x4 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 1 && x1 == 1 && x2 == 31 && x3 == 0 => {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 1 && x1 == 1 && x2 == 31 && x3 == 1 => {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 1 && x1 == 1 && x2 == 31 && x3 == 2 && x5 & 1 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 1 && x1 == 1 && x2 == 31 && x3 == 3 && x5 & 1 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 1 && x1 == 1 && x2 == 31 && x3 == 4 && x5 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 1 && x2 == 31 && x3 == 4 && x4 == 0 && x5 == 1 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 1 && x1 == 1 && x2 == 31 && x3 == 5 && x5 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 1 && x2 == 31 && x3 == 5 && x4 == 0 && x5 == 1 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 1 && x1 == 1 && x2 == 31 && x3 == 6 && x4 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 1 && x1 == 1 && x2 == 31 && x3 == 7 && x4 == 0 =>
                        {
                            Some(Op::Ld4RAsisdlsopR4I)
                        }
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _, _, _)
                    if x0 & 11 == 0 && x2 == 1 && x4 & 1 == 0 && x6 & 16 == 16 =>
                {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _, _, _)
                    if x0 & 11 == 0 && x2 == 1 && x4 & 1 == 0 && x6 & 8 == 8 =>
                {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _, _, _)
                    if x0 & 11 == 0 && x2 == 1 && x4 & 1 == 0 && x6 & 4 == 4 =>
                {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _, _, _)
                    if x0 & 11 == 0 && x2 == 1 && x4 & 1 == 0 && x6 & 2 == 2 =>
                {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _, _, _)
                    if x0 & 11 == 0 && x2 == 1 && x4 & 1 == 0 && x6 & 1 == 1 =>
                {
                    None
                }
                (x0, _, x2, _, x4, _, x6, _, _, _)
                    if x0 == 13 && x2 == 0 && x4 & 2 == 2 && x6 & 32 == 32 =>
                {
                    let opc = (instr >> 22) & 3;
                    let imm9 = (instr >> 12) & 17;
                    let op2 = (instr >> 10) & 3;
                    let Rn = (instr >> 5) & 9;
                    let Rt = instr & 9;
                    match (opc, imm9, op2) {
                        (x0, _, x2) if x0 == 0 && x2 == 1 => Some(Op::Stg64SpostLdsttags),
                        (x0, _, x2) if x0 == 0 && x2 == 2 => Some(Op::Stg64SoffsetLdsttags),
                        (x0, _, x2) if x0 == 0 && x2 == 3 => Some(Op::Stg64SpreLdsttags),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            Some(Op::Stzgm64BulkLdsttags)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 0 => Some(Op::Ldg64LoffsetLdsttags),
                        (x0, _, x2) if x0 == 1 && x2 == 1 => Some(Op::Stzg64SpostLdsttags),
                        (x0, _, x2) if x0 == 1 && x2 == 2 => Some(Op::Stzg64SoffsetLdsttags),
                        (x0, _, x2) if x0 == 1 && x2 == 3 => Some(Op::Stzg64SpreLdsttags),
                        (x0, _, x2) if x0 == 2 && x2 == 1 => Some(Op::St2G64SpostLdsttags),
                        (x0, _, x2) if x0 == 2 && x2 == 2 => Some(Op::St2G64SoffsetLdsttags),
                        (x0, _, x2) if x0 == 2 && x2 == 3 => Some(Op::St2G64SpreLdsttags),
                        (x0, x1, x2) if x0 == 2 && x1 != 0 && x2 == 0 => None,
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                            Some(Op::Stgm64BulkLdsttags)
                        }
                        (x0, _, x2) if x0 == 3 && x2 == 1 => Some(Op::Stz2G64SpostLdsttags),
                        (x0, _, x2) if x0 == 3 && x2 == 2 => Some(Op::Stz2G64SoffsetLdsttags),
                        (x0, _, x2) if x0 == 3 && x2 == 3 => Some(Op::Stz2G64SpreLdsttags),
                        (x0, x1, x2) if x0 == 3 && x1 != 0 && x2 == 0 => None,
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 0 => {
                            Some(Op::Ldgm64BulkLdsttags)
                        }
                        _ => None,
                    }
                }
                (x0, _, x2, _, _, _, _, _, _, _) if x0 & 11 == 8 && x2 == 1 => None,
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
                        (_, x1, _, x3, _, x5) if x1 == 1 && x3 == 1 && x5 != 31 => None,
                        (x0, x1, _, x3, _, x5) if x0 & 2 == 0 && x1 == 0 && x3 == 1 && x5 != 31 => {
                            None
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 =>
                        {
                            Some(Op::LdaxrLr64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 1 =>
                        {
                            Some(Op::LdaxrLr64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 31 =>
                        {
                            Some(Op::CaspalCp64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 1 && x5 == 31 =>
                        {
                            Some(Op::CaspalCp64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 =>
                        {
                            Some(Op::LdaxrLr64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 =>
                        {
                            Some(Op::LdaxrLr64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 31 =>
                        {
                            Some(Op::CaspalCp64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 31 =>
                        {
                            Some(Op::CaspalCp64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 0 && x4 == 0 =>
                        {
                            Some(Op::LdarLr64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 0 && x4 == 1 =>
                        {
                            Some(Op::LdarLr64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 31 =>
                        {
                            Some(Op::CasalC64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 1 && x4 == 1 && x5 == 31 =>
                        {
                            Some(Op::CasalC64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 0 && x4 == 0 =>
                        {
                            Some(Op::LdarLr64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 0 && x4 == 1 =>
                        {
                            Some(Op::LdarLr64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 31 =>
                        {
                            Some(Op::CasalC64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 31 =>
                        {
                            Some(Op::CasalC64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 =>
                        {
                            Some(Op::LdaxrLr64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 1 =>
                        {
                            Some(Op::LdaxrLr64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 31 =>
                        {
                            Some(Op::CaspalCp64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 1 && x5 == 31 =>
                        {
                            Some(Op::CaspalCp64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 =>
                        {
                            Some(Op::LdaxrLr64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 =>
                        {
                            Some(Op::LdaxrLr64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 31 =>
                        {
                            Some(Op::CaspalCp64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 31 =>
                        {
                            Some(Op::CaspalCp64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 0 && x4 == 0 =>
                        {
                            Some(Op::LdarLr64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 0 && x4 == 1 =>
                        {
                            Some(Op::LdarLr64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 31 =>
                        {
                            Some(Op::CasalC64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 1 && x4 == 1 && x5 == 31 =>
                        {
                            Some(Op::CasalC64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 0 && x4 == 0 =>
                        {
                            Some(Op::LdarLr64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 0 && x4 == 1 =>
                        {
                            Some(Op::LdarLr64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 31 =>
                        {
                            Some(Op::CasalC64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 31 =>
                        {
                            Some(Op::CasalC64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 =>
                        {
                            Some(Op::LdaxrLr64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 1 =>
                        {
                            Some(Op::LdaxrLr64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 =>
                        {
                            Some(Op::LdaxpLp64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 1 =>
                        {
                            Some(Op::LdaxpLp64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 =>
                        {
                            Some(Op::LdaxrLr64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 =>
                        {
                            Some(Op::LdaxrLr64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 =>
                        {
                            Some(Op::LdaxpLp64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 =>
                        {
                            Some(Op::LdaxpLp64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 2 && x1 == 1 && x2 == 0 && x3 == 0 && x4 == 0 =>
                        {
                            Some(Op::LdarLr64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 2 && x1 == 1 && x2 == 0 && x3 == 0 && x4 == 1 =>
                        {
                            Some(Op::LdarLr64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 1 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 31 =>
                        {
                            Some(Op::CasalC64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 1 && x2 == 0 && x3 == 1 && x4 == 1 && x5 == 31 =>
                        {
                            Some(Op::CasalC64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 2 && x1 == 1 && x2 == 1 && x3 == 0 && x4 == 0 =>
                        {
                            Some(Op::LdarLr64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 2 && x1 == 1 && x2 == 1 && x3 == 0 && x4 == 1 =>
                        {
                            Some(Op::LdarLr64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 1 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 31 =>
                        {
                            Some(Op::CasalC64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 1 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 31 =>
                        {
                            Some(Op::CasalC64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 =>
                        {
                            Some(Op::LdaxrLr64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 1 =>
                        {
                            Some(Op::LdaxrLr64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 =>
                        {
                            Some(Op::LdaxpLp64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 1 =>
                        {
                            Some(Op::LdaxpLp64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 =>
                        {
                            Some(Op::LdaxrLr64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 =>
                        {
                            Some(Op::LdaxrLr64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 =>
                        {
                            Some(Op::LdaxpLp64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 =>
                        {
                            Some(Op::LdaxpLp64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 3 && x1 == 1 && x2 == 0 && x3 == 0 && x4 == 0 =>
                        {
                            Some(Op::LdarLr64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 3 && x1 == 1 && x2 == 0 && x3 == 0 && x4 == 1 =>
                        {
                            Some(Op::LdarLr64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 1 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 31 =>
                        {
                            Some(Op::CasalC64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 1 && x2 == 0 && x3 == 1 && x4 == 1 && x5 == 31 =>
                        {
                            Some(Op::CasalC64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 3 && x1 == 1 && x2 == 1 && x3 == 0 && x4 == 0 =>
                        {
                            Some(Op::LdarLr64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 3 && x1 == 1 && x2 == 1 && x3 == 0 && x4 == 1 =>
                        {
                            Some(Op::LdarLr64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 1 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 31 =>
                        {
                            Some(Op::CasalC64Ldstexcl)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 1 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 31 =>
                        {
                            Some(Op::CasalC64Ldstexcl)
                        }
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _, x8, _)
                    if x0 & 3 == 1 && x2 == 0 && x4 & 2 == 2 && x6 & 32 == 0 && x8 == 0 =>
                {
                    let size = (instr >> 30) & 3;
                    let opc = (instr >> 22) & 3;
                    let imm9 = (instr >> 12) & 17;
                    let Rn = (instr >> 5) & 9;
                    let Rt = instr & 9;
                    match (size, opc) {
                        (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::Ldapur64LdapstlUnscaled),
                        (x0, x1) if x0 == 0 && x1 == 1 => Some(Op::Ldapur64LdapstlUnscaled),
                        (x0, x1) if x0 == 0 && x1 == 2 => Some(Op::Ldapur64LdapstlUnscaled),
                        (x0, x1) if x0 == 0 && x1 == 3 => Some(Op::Ldapur64LdapstlUnscaled),
                        (x0, x1) if x0 == 1 && x1 == 0 => Some(Op::Ldapur64LdapstlUnscaled),
                        (x0, x1) if x0 == 1 && x1 == 1 => Some(Op::Ldapur64LdapstlUnscaled),
                        (x0, x1) if x0 == 1 && x1 == 2 => Some(Op::Ldapur64LdapstlUnscaled),
                        (x0, x1) if x0 == 1 && x1 == 3 => Some(Op::Ldapur64LdapstlUnscaled),
                        (x0, x1) if x0 == 2 && x1 == 0 => Some(Op::Ldapur64LdapstlUnscaled),
                        (x0, x1) if x0 == 2 && x1 == 1 => Some(Op::Ldapur64LdapstlUnscaled),
                        (x0, x1) if x0 == 2 && x1 == 2 => Some(Op::Ldapur64LdapstlUnscaled),
                        (x0, x1) if x0 == 2 && x1 == 3 => None,
                        (x0, x1) if x0 == 3 && x1 == 0 => Some(Op::Ldapur64LdapstlUnscaled),
                        (x0, x1) if x0 == 3 && x1 == 1 => Some(Op::Ldapur64LdapstlUnscaled),
                        (x0, x1) if x0 == 3 && x1 == 2 => None,
                        (x0, x1) if x0 == 3 && x1 == 3 => None,
                        _ => None,
                    }
                }
                (x0, _, _, _, x4, _, _, _, _, _) if x0 & 3 == 1 && x4 & 2 == 0 => {
                    let opc = (instr >> 30) & 3;
                    let V = (instr >> 26) & 1;
                    let imm19 = (instr >> 5) & 37;
                    let Rt = instr & 9;
                    match (opc, V) {
                        (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::PrfmPLoadlit),
                        (x0, x1) if x0 == 0 && x1 == 1 => Some(Op::LdrQLoadlit),
                        (x0, x1) if x0 == 1 && x1 == 0 => Some(Op::PrfmPLoadlit),
                        (x0, x1) if x0 == 1 && x1 == 1 => Some(Op::LdrQLoadlit),
                        (x0, x1) if x0 == 2 && x1 == 0 => Some(Op::PrfmPLoadlit),
                        (x0, x1) if x0 == 2 && x1 == 1 => Some(Op::LdrQLoadlit),
                        (x0, x1) if x0 == 3 && x1 == 0 => Some(Op::PrfmPLoadlit),
                        (x0, x1) if x0 == 3 && x1 == 1 => None,
                        _ => None,
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
                            Some(Op::Ldnp64LdstnapairOffs)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            Some(Op::Ldnp64LdstnapairOffs)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            Some(Op::LdnpQLdstnapairOffs)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            Some(Op::LdnpQLdstnapairOffs)
                        }
                        (x0, x1, _) if x0 == 1 && x1 == 0 => None,
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            Some(Op::LdnpQLdstnapairOffs)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            Some(Op::LdnpQLdstnapairOffs)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                            Some(Op::Ldnp64LdstnapairOffs)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                            Some(Op::Ldnp64LdstnapairOffs)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                            Some(Op::LdnpQLdstnapairOffs)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                            Some(Op::LdnpQLdstnapairOffs)
                        }
                        (x0, _, _) if x0 == 3 => None,
                        _ => None,
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
                            Some(Op::Ldp64LdstpairPost)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            Some(Op::Ldp64LdstpairPost)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => Some(Op::LdpQLdstpairPost),
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => Some(Op::LdpQLdstpairPost),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            Some(Op::Stgp64LdstpairPost)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            Some(Op::Ldp64LdstpairPost)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => Some(Op::LdpQLdstpairPost),
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => Some(Op::LdpQLdstpairPost),
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                            Some(Op::Ldp64LdstpairPost)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                            Some(Op::Ldp64LdstpairPost)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => Some(Op::LdpQLdstpairPost),
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => Some(Op::LdpQLdstpairPost),
                        (x0, _, _) if x0 == 3 => None,
                        _ => None,
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
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => Some(Op::Ldp64LdstpairOff),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => Some(Op::Ldp64LdstpairOff),
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => Some(Op::LdpQLdstpairOff),
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => Some(Op::LdpQLdstpairOff),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            Some(Op::Stgp64LdstpairOff)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => Some(Op::Ldp64LdstpairOff),
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => Some(Op::LdpQLdstpairOff),
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => Some(Op::LdpQLdstpairOff),
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => Some(Op::Ldp64LdstpairOff),
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => Some(Op::Ldp64LdstpairOff),
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => Some(Op::LdpQLdstpairOff),
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => Some(Op::LdpQLdstpairOff),
                        (x0, _, _) if x0 == 3 => None,
                        _ => None,
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
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => Some(Op::Ldp64LdstpairPre),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => Some(Op::Ldp64LdstpairPre),
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => Some(Op::LdpQLdstpairPre),
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => Some(Op::LdpQLdstpairPre),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            Some(Op::Stgp64LdstpairPre)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => Some(Op::Ldp64LdstpairPre),
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => Some(Op::LdpQLdstpairPre),
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => Some(Op::LdpQLdstpairPre),
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => Some(Op::Ldp64LdstpairPre),
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => Some(Op::Ldp64LdstpairPre),
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => Some(Op::LdpQLdstpairPre),
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => Some(Op::LdpQLdstpairPre),
                        (x0, _, _) if x0 == 3 => None,
                        _ => None,
                    }
                }
                (x0, _, _, _, x4, _, x6, _, x8, _)
                    if x0 & 3 == 3 && x4 & 2 == 0 && x6 & 32 == 0 && x8 == 0 =>
                {
                    let size = (instr >> 30) & 3;
                    let V = (instr >> 26) & 1;
                    let opc = (instr >> 22) & 3;
                    let imm9 = (instr >> 12) & 17;
                    let Rn = (instr >> 5) & 9;
                    let Rt = instr & 9;
                    match (size, V, opc) {
                        (x0, x1, x2) if x0 & 1 == 1 && x1 == 1 && x2 & 2 == 2 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            Some(Op::PrfumPLdstUnscaled)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            Some(Op::PrfumPLdstUnscaled)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 2 => {
                            Some(Op::PrfumPLdstUnscaled)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 3 => {
                            Some(Op::PrfumPLdstUnscaled)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            Some(Op::LdurDLdstUnscaled)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            Some(Op::LdurDLdstUnscaled)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 2 => {
                            Some(Op::LdurDLdstUnscaled)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 3 => {
                            Some(Op::LdurDLdstUnscaled)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            Some(Op::PrfumPLdstUnscaled)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            Some(Op::PrfumPLdstUnscaled)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 2 => {
                            Some(Op::PrfumPLdstUnscaled)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 3 => {
                            Some(Op::PrfumPLdstUnscaled)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            Some(Op::LdurDLdstUnscaled)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            Some(Op::LdurDLdstUnscaled)
                        }
                        (x0, x1, x2) if x0 & 2 == 2 && x1 == 0 && x2 == 3 => None,
                        (x0, x1, x2) if x0 & 2 == 2 && x1 == 1 && x2 & 2 == 2 => None,
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => {
                            Some(Op::PrfumPLdstUnscaled)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => {
                            Some(Op::PrfumPLdstUnscaled)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 2 => {
                            Some(Op::PrfumPLdstUnscaled)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => {
                            Some(Op::LdurDLdstUnscaled)
                        }
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => {
                            Some(Op::LdurDLdstUnscaled)
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 0 => {
                            Some(Op::PrfumPLdstUnscaled)
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 1 => {
                            Some(Op::PrfumPLdstUnscaled)
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 2 => {
                            Some(Op::PrfumPLdstUnscaled)
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 0 => {
                            Some(Op::LdurDLdstUnscaled)
                        }
                        (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 1 => {
                            Some(Op::LdurDLdstUnscaled)
                        }
                        _ => None,
                    }
                }
                (x0, _, _, _, x4, _, x6, _, x8, _)
                    if x0 & 3 == 3 && x4 & 2 == 0 && x6 & 32 == 0 && x8 == 1 =>
                {
                    let size = (instr >> 30) & 3;
                    let V = (instr >> 26) & 1;
                    let opc = (instr >> 22) & 3;
                    let imm9 = (instr >> 12) & 17;
                    let Rn = (instr >> 5) & 9;
                    let Rt = instr & 9;
                    match (size, V, opc) {
                        (x0, x1, x2) if x0 & 1 == 1 && x1 == 1 && x2 & 2 == 2 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => Some(Op::Ldr64LdstImmpost),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => Some(Op::Ldr64LdstImmpost),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 2 => Some(Op::Ldr64LdstImmpost),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 3 => Some(Op::Ldr64LdstImmpost),
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => Some(Op::LdrDLdstImmpost),
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => Some(Op::LdrDLdstImmpost),
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 2 => Some(Op::LdrDLdstImmpost),
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 3 => Some(Op::LdrDLdstImmpost),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => Some(Op::Ldr64LdstImmpost),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => Some(Op::Ldr64LdstImmpost),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 2 => Some(Op::Ldr64LdstImmpost),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 3 => Some(Op::Ldr64LdstImmpost),
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => Some(Op::LdrDLdstImmpost),
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => Some(Op::LdrDLdstImmpost),
                        (x0, x1, x2) if x0 & 2 == 2 && x1 == 0 && x2 == 3 => None,
                        (x0, x1, x2) if x0 & 2 == 2 && x1 == 1 && x2 & 2 == 2 => None,
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => Some(Op::Ldr64LdstImmpost),
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => Some(Op::Ldr64LdstImmpost),
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 2 => Some(Op::Ldr64LdstImmpost),
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => Some(Op::LdrDLdstImmpost),
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => Some(Op::LdrDLdstImmpost),
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 0 => Some(Op::Ldr64LdstImmpost),
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 1 => Some(Op::Ldr64LdstImmpost),
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 2 => None,
                        (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 0 => Some(Op::LdrDLdstImmpost),
                        (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 1 => Some(Op::LdrDLdstImmpost),
                        _ => None,
                    }
                }
                (x0, _, _, _, x4, _, x6, _, x8, _)
                    if x0 & 3 == 3 && x4 & 2 == 0 && x6 & 32 == 0 && x8 == 2 =>
                {
                    let size = (instr >> 30) & 3;
                    let V = (instr >> 26) & 1;
                    let opc = (instr >> 22) & 3;
                    let imm9 = (instr >> 12) & 17;
                    let Rn = (instr >> 5) & 9;
                    let Rt = instr & 9;
                    match (size, V, opc) {
                        (_, x1, _) if x1 == 1 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => Some(Op::Ldtr64LdstUnpriv),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => Some(Op::Ldtr64LdstUnpriv),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 2 => Some(Op::Ldtr64LdstUnpriv),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 3 => Some(Op::Ldtr64LdstUnpriv),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => Some(Op::Ldtr64LdstUnpriv),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => Some(Op::Ldtr64LdstUnpriv),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 2 => Some(Op::Ldtr64LdstUnpriv),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 3 => Some(Op::Ldtr64LdstUnpriv),
                        (x0, x1, x2) if x0 & 2 == 2 && x1 == 0 && x2 == 3 => None,
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => Some(Op::Ldtr64LdstUnpriv),
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => Some(Op::Ldtr64LdstUnpriv),
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 2 => Some(Op::Ldtr64LdstUnpriv),
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 0 => Some(Op::Ldtr64LdstUnpriv),
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 1 => Some(Op::Ldtr64LdstUnpriv),
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 2 => None,
                        _ => None,
                    }
                }
                (x0, _, _, _, x4, _, x6, _, x8, _)
                    if x0 & 3 == 3 && x4 & 2 == 0 && x6 & 32 == 0 && x8 == 3 =>
                {
                    let size = (instr >> 30) & 3;
                    let V = (instr >> 26) & 1;
                    let opc = (instr >> 22) & 3;
                    let imm9 = (instr >> 12) & 17;
                    let Rn = (instr >> 5) & 9;
                    let Rt = instr & 9;
                    match (size, V, opc) {
                        (x0, x1, x2) if x0 & 1 == 1 && x1 == 1 && x2 & 2 == 2 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => Some(Op::Ldr64LdstImmpre),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => Some(Op::Ldr64LdstImmpre),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 2 => Some(Op::Ldr64LdstImmpre),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 3 => Some(Op::Ldr64LdstImmpre),
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => Some(Op::LdrDLdstImmpre),
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => Some(Op::LdrDLdstImmpre),
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 2 => Some(Op::LdrDLdstImmpre),
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 3 => Some(Op::LdrDLdstImmpre),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => Some(Op::Ldr64LdstImmpre),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => Some(Op::Ldr64LdstImmpre),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 2 => Some(Op::Ldr64LdstImmpre),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 3 => Some(Op::Ldr64LdstImmpre),
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => Some(Op::LdrDLdstImmpre),
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => Some(Op::LdrDLdstImmpre),
                        (x0, x1, x2) if x0 & 2 == 2 && x1 == 0 && x2 == 3 => None,
                        (x0, x1, x2) if x0 & 2 == 2 && x1 == 1 && x2 & 2 == 2 => None,
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => Some(Op::Ldr64LdstImmpre),
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => Some(Op::Ldr64LdstImmpre),
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 2 => Some(Op::Ldr64LdstImmpre),
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => Some(Op::LdrDLdstImmpre),
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => Some(Op::LdrDLdstImmpre),
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 0 => Some(Op::Ldr64LdstImmpre),
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 1 => Some(Op::Ldr64LdstImmpre),
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 2 => None,
                        (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 0 => Some(Op::LdrDLdstImmpre),
                        (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 1 => Some(Op::LdrDLdstImmpre),
                        _ => None,
                    }
                }
                (x0, _, _, _, x4, _, x6, _, x8, _)
                    if x0 & 3 == 3 && x4 & 2 == 0 && x6 & 32 == 32 && x8 == 0 =>
                {
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
                        (_, x1, _, _, x4, x5) if x1 == 0 && x4 == 1 && x5 == 1 => None,
                        (_, x1, _, _, x4, x5) if x1 == 0 && x4 == 1 && x5 & 6 == 2 => None,
                        (_, x1, _, _, x4, x5) if x1 == 0 && x4 == 1 && x5 == 5 => None,
                        (_, x1, _, _, x4, x5) if x1 == 0 && x4 == 1 && x5 & 6 == 6 => None,
                        (_, x1, x2, _, x4, x5) if x1 == 0 && x2 == 0 && x4 == 1 && x5 == 4 => None,
                        (_, x1, x2, x3, x4, x5)
                            if x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 4 =>
                        {
                            None
                        }
                        (_, x1, _, _, _, _) if x1 == 1 => None,
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 0 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 1 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 2 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 3 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 4 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 5 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 6 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 7 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 1 && x5 == 0 =>
                        {
                            Some(Op::Swpal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 0 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 1 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 2 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 3 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 4 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 5 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 6 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 7 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 1 && x5 == 0 =>
                        {
                            Some(Op::Swpal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 0 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 1 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 2 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 3 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 4 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 5 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 6 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 7 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 0 =>
                        {
                            Some(Op::Swpal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 4 =>
                        {
                            Some(Op::Ldapr64LMemop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 0 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 1 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 2 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 3 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 4 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 5 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 6 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 7 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 0 =>
                        {
                            Some(Op::Swpal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 0 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 1 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 2 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 3 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 4 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 5 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 6 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 7 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 1 && x5 == 0 =>
                        {
                            Some(Op::Swpal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 0 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 1 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 2 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 3 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 4 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 5 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 6 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 7 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 1 && x5 == 0 =>
                        {
                            Some(Op::Swpal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 0 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 1 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 2 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 3 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 4 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 5 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 6 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 7 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 0 =>
                        {
                            Some(Op::Swpal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 4 =>
                        {
                            Some(Op::Ldapr64LMemop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 0 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 1 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 2 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 3 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 4 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 5 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 6 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 7 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 0 =>
                        {
                            Some(Op::Swpal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 0 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 1 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 2 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 3 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 4 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 5 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 6 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 7 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 1 && x5 == 0 =>
                        {
                            Some(Op::Swpal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 0 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 1 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 2 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 3 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 4 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 5 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 6 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 7 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 1 && x5 == 0 =>
                        {
                            Some(Op::Swpal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 0 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 1 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 2 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 3 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 4 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 5 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 6 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 7 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 0 =>
                        {
                            Some(Op::Swpal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 4 =>
                        {
                            Some(Op::Ldapr64LMemop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 0 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 1 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 2 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 3 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 4 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 5 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 6 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 7 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 0 =>
                        {
                            Some(Op::Swpal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 0 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 1 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 2 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 3 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 4 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 5 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 6 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 7 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 1 && x5 == 0 =>
                        {
                            Some(Op::Swpal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 0 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 1 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 2 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 3 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 4 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 5 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 6 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 && x5 == 7 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 1 && x5 == 0 =>
                        {
                            Some(Op::Swpal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 0 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 1 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 2 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 3 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 4 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 5 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 6 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 7 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 0 =>
                        {
                            Some(Op::Swpal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 4 =>
                        {
                            Some(Op::Ldapr64LMemop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 0 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 1 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 2 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 3 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 4 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 5 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 6 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 7 =>
                        {
                            Some(Op::Lduminal64Memop)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 0 =>
                        {
                            Some(Op::Swpal64Memop)
                        }
                        _ => None,
                    }
                }
                (x0, _, _, _, x4, _, x6, _, x8, _)
                    if x0 & 3 == 3 && x4 & 2 == 0 && x6 & 32 == 32 && x8 == 2 =>
                {
                    let size = (instr >> 30) & 3;
                    let V = (instr >> 26) & 1;
                    let opc = (instr >> 22) & 3;
                    let Rm = (instr >> 16) & 9;
                    let option = (instr >> 13) & 5;
                    let S = (instr >> 12) & 1;
                    let Rn = (instr >> 5) & 9;
                    let Rt = instr & 9;
                    match (size, V, opc, option) {
                        (_, _, _, x3) if x3 & 2 == 0 => None,
                        (x0, x1, x2, _) if x0 & 1 == 1 && x1 == 1 && x2 & 2 == 2 => None,
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 != 3 => {
                            Some(Op::PrfmPLdstRegoff)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 3 => {
                            Some(Op::PrfmPLdstRegoff)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 != 3 => {
                            Some(Op::PrfmPLdstRegoff)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 3 => {
                            Some(Op::PrfmPLdstRegoff)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 2 && x3 != 3 => {
                            Some(Op::PrfmPLdstRegoff)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 2 && x3 == 3 => {
                            Some(Op::PrfmPLdstRegoff)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 != 3 => {
                            Some(Op::PrfmPLdstRegoff)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 3 => {
                            Some(Op::PrfmPLdstRegoff)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 != 3 => {
                            Some(Op::LdrDLdstRegoff)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 3 => {
                            Some(Op::LdrDLdstRegoff)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 != 3 => {
                            Some(Op::LdrDLdstRegoff)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 3 => {
                            Some(Op::LdrDLdstRegoff)
                        }
                        (x0, x1, x2, _) if x0 == 0 && x1 == 1 && x2 == 2 => {
                            Some(Op::LdrDLdstRegoff)
                        }
                        (x0, x1, x2, _) if x0 == 0 && x1 == 1 && x2 == 3 => {
                            Some(Op::LdrDLdstRegoff)
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            Some(Op::PrfmPLdstRegoff)
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            Some(Op::PrfmPLdstRegoff)
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 0 && x2 == 2 => {
                            Some(Op::PrfmPLdstRegoff)
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 0 && x2 == 3 => {
                            Some(Op::PrfmPLdstRegoff)
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            Some(Op::LdrDLdstRegoff)
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            Some(Op::LdrDLdstRegoff)
                        }
                        (x0, x1, x2, _) if x0 & 2 == 2 && x1 == 0 && x2 == 3 => None,
                        (x0, x1, x2, _) if x0 & 2 == 2 && x1 == 1 && x2 & 2 == 2 => None,
                        (x0, x1, x2, _) if x0 == 2 && x1 == 0 && x2 == 0 => {
                            Some(Op::PrfmPLdstRegoff)
                        }
                        (x0, x1, x2, _) if x0 == 2 && x1 == 0 && x2 == 1 => {
                            Some(Op::PrfmPLdstRegoff)
                        }
                        (x0, x1, x2, _) if x0 == 2 && x1 == 0 && x2 == 2 => {
                            Some(Op::PrfmPLdstRegoff)
                        }
                        (x0, x1, x2, _) if x0 == 2 && x1 == 1 && x2 == 0 => {
                            Some(Op::LdrDLdstRegoff)
                        }
                        (x0, x1, x2, _) if x0 == 2 && x1 == 1 && x2 == 1 => {
                            Some(Op::LdrDLdstRegoff)
                        }
                        (x0, x1, x2, _) if x0 == 3 && x1 == 0 && x2 == 0 => {
                            Some(Op::PrfmPLdstRegoff)
                        }
                        (x0, x1, x2, _) if x0 == 3 && x1 == 0 && x2 == 1 => {
                            Some(Op::PrfmPLdstRegoff)
                        }
                        (x0, x1, x2, _) if x0 == 3 && x1 == 0 && x2 == 2 => {
                            Some(Op::PrfmPLdstRegoff)
                        }
                        (x0, x1, x2, _) if x0 == 3 && x1 == 1 && x2 == 0 => {
                            Some(Op::LdrDLdstRegoff)
                        }
                        (x0, x1, x2, _) if x0 == 3 && x1 == 1 && x2 == 1 => {
                            Some(Op::LdrDLdstRegoff)
                        }
                        _ => None,
                    }
                }
                (x0, _, _, _, x4, _, x6, _, x8, _)
                    if x0 & 3 == 3 && x4 & 2 == 0 && x6 & 32 == 32 && x8 & 1 == 1 =>
                {
                    let size = (instr >> 30) & 3;
                    let V = (instr >> 26) & 1;
                    let M = (instr >> 23) & 1;
                    let S = (instr >> 22) & 1;
                    let imm9 = (instr >> 12) & 17;
                    let W = (instr >> 11) & 1;
                    let Rn = (instr >> 5) & 9;
                    let Rt = instr & 9;
                    match (size, V, M, W) {
                        (x0, _, _, _) if x0 != 3 => None,
                        (x0, x1, x2, x3) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            Some(Op::Ldrab64WLdstPac)
                        }
                        (x0, x1, x2, x3) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            Some(Op::Ldrab64WLdstPac)
                        }
                        (x0, x1, x2, x3) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            Some(Op::Ldrab64WLdstPac)
                        }
                        (x0, x1, x2, x3) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 => {
                            Some(Op::Ldrab64WLdstPac)
                        }
                        (x0, x1, _, _) if x0 == 3 && x1 == 1 => None,
                        _ => None,
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
                        (x0, x1, x2) if x0 & 1 == 1 && x1 == 1 && x2 & 2 == 2 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => Some(Op::PrfmPLdstPos),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => Some(Op::PrfmPLdstPos),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 2 => Some(Op::PrfmPLdstPos),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 3 => Some(Op::PrfmPLdstPos),
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => Some(Op::LdrDLdstPos),
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => Some(Op::LdrDLdstPos),
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 2 => Some(Op::LdrDLdstPos),
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 3 => Some(Op::LdrDLdstPos),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => Some(Op::PrfmPLdstPos),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => Some(Op::PrfmPLdstPos),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 2 => Some(Op::PrfmPLdstPos),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 3 => Some(Op::PrfmPLdstPos),
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => Some(Op::LdrDLdstPos),
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => Some(Op::LdrDLdstPos),
                        (x0, x1, x2) if x0 & 2 == 2 && x1 == 0 && x2 == 3 => None,
                        (x0, x1, x2) if x0 & 2 == 2 && x1 == 1 && x2 & 2 == 2 => None,
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => Some(Op::PrfmPLdstPos),
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => Some(Op::PrfmPLdstPos),
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 2 => Some(Op::PrfmPLdstPos),
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => Some(Op::LdrDLdstPos),
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => Some(Op::LdrDLdstPos),
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 0 => Some(Op::PrfmPLdstPos),
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 1 => Some(Op::PrfmPLdstPos),
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 2 => Some(Op::PrfmPLdstPos),
                        (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 0 => Some(Op::LdrDLdstPos),
                        (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 1 => Some(Op::LdrDLdstPos),
                        _ => None,
                    }
                }
                _ => None,
            }
        }
        (_, x1, _) if x1 & 14 == 10 => {
            match (
                (instr >> 31) & 1,
                (instr >> 30) & 1,
                (instr >> 29) & 1,
                (instr >> 28) & 1,
                (instr >> 25) & 5,
                (instr >> 21) & 7,
                (instr >> 16) & 9,
                (instr >> 10) & 11,
                instr & 19,
            ) {
                (_, x1, _, x3, _, x5, _, _, _) if x1 == 0 && x3 == 1 && x5 == 6 => {
                    let sf = (instr >> 31) & 1;
                    let S = (instr >> 29) & 1;
                    let Rm = (instr >> 16) & 9;
                    let opcode = (instr >> 10) & 11;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (sf, S, opcode) {
                        (_, _, x2) if x2 == 1 => None,
                        (_, _, x2) if x2 & 56 == 24 => None,
                        (_, _, x2) if x2 & 32 == 32 => None,
                        (_, x1, x2) if x1 == 0 && x2 & 62 == 6 => None,
                        (_, x1, x2) if x1 == 0 && x2 == 13 => None,
                        (_, x1, x2) if x1 == 0 && x2 & 62 == 14 => None,
                        (_, x1, x2) if x1 == 1 && x2 & 62 == 2 => None,
                        (_, x1, x2) if x1 == 1 && x2 & 60 == 4 => None,
                        (_, x1, x2) if x1 == 1 && x2 & 56 == 8 => None,
                        (_, x1, x2) if x1 == 1 && x2 & 48 == 16 => None,
                        (x0, _, x2) if x0 == 0 && x2 == 0 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 2 => Some(Op::Sdiv64Dp2Src),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 3 => Some(Op::Sdiv64Dp2Src),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 & 62 == 4 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 8 => Some(Op::Rorv64Dp2Src),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 9 => Some(Op::Rorv64Dp2Src),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 10 => Some(Op::Rorv64Dp2Src),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 11 => Some(Op::Rorv64Dp2Src),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 12 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 & 59 == 19 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 16 => {
                            Some(Op::Crc32Cx64CDp2Src)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 17 => {
                            Some(Op::Crc32Cx64CDp2Src)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 18 => {
                            Some(Op::Crc32Cx64CDp2Src)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 20 => {
                            Some(Op::Crc32Cx64CDp2Src)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 21 => {
                            Some(Op::Crc32Cx64CDp2Src)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 22 => {
                            Some(Op::Crc32Cx64CDp2Src)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => Some(Op::Subp64SDp2Src),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 2 => Some(Op::Sdiv64Dp2Src),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 3 => Some(Op::Sdiv64Dp2Src),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 4 => Some(Op::Irg64IDp2Src),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 5 => Some(Op::Gmi64GDp2Src),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 8 => Some(Op::Rorv64Dp2Src),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 9 => Some(Op::Rorv64Dp2Src),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 10 => Some(Op::Rorv64Dp2Src),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 11 => Some(Op::Rorv64Dp2Src),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 12 => Some(Op::Pacga64PDp2Src),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 & 57 == 16 => None,
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 & 58 == 16 => None,
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 19 => {
                            Some(Op::Crc32Cx64CDp2Src)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 23 => {
                            Some(Op::Crc32Cx64CDp2Src)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => Some(Op::Subps64SDp2Src),
                        _ => None,
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
                        (_, _, _, x3, _) if x3 & 32 == 32 => None,
                        (_, _, x2, _, _) if x2 & 2 == 2 => None,
                        (_, _, x2, _, _) if x2 & 4 == 4 => None,
                        (_, _, x2, _, _) if x2 & 8 == 8 => None,
                        (_, _, x2, _, _) if x2 & 16 == 16 => None,
                        (_, x1, x2, x3, _) if x1 == 0 && x2 == 0 && x3 & 62 == 6 => None,
                        (_, x1, x2, x3, _) if x1 == 0 && x2 == 0 && x3 & 56 == 8 => None,
                        (_, x1, x2, x3, _) if x1 == 0 && x2 == 0 && x3 & 48 == 16 => None,
                        (_, x1, _, _, _) if x1 == 1 => None,
                        (x0, _, x2, _, _) if x0 == 0 && x2 == 1 => None,
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            Some(Op::Rbit64Dp1Src)
                        }
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            Some(Op::Rev64Dp1Src)
                        }
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 2 => {
                            Some(Op::Rev64Dp1Src)
                        }
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 3 => None,
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 4 => {
                            Some(Op::Cls64Dp1Src)
                        }
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 5 => {
                            Some(Op::Cls64Dp1Src)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            Some(Op::Rbit64Dp1Src)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            Some(Op::Rev64Dp1Src)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 2 => {
                            Some(Op::Rev64Dp1Src)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 3 => {
                            Some(Op::Rev64Dp1Src)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 4 => {
                            Some(Op::Cls64Dp1Src)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 5 => {
                            Some(Op::Cls64Dp1Src)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            Some(Op::Paciza64ZDp1Src)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 => {
                            Some(Op::Pacizb64ZDp1Src)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 2 => {
                            Some(Op::Pacdza64ZDp1Src)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 3 => {
                            Some(Op::Pacdzb64ZDp1Src)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 4 => {
                            Some(Op::Autiza64ZDp1Src)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 5 => {
                            Some(Op::Autizb64ZDp1Src)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 6 => {
                            Some(Op::Autdza64ZDp1Src)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 7 => {
                            Some(Op::Autdzb64ZDp1Src)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 8 && x4 == 31 =>
                        {
                            Some(Op::Paciza64ZDp1Src)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 9 && x4 == 31 =>
                        {
                            Some(Op::Pacizb64ZDp1Src)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 10 && x4 == 31 =>
                        {
                            Some(Op::Pacdza64ZDp1Src)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 11 && x4 == 31 =>
                        {
                            Some(Op::Pacdzb64ZDp1Src)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 12 && x4 == 31 =>
                        {
                            Some(Op::Autiza64ZDp1Src)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 13 && x4 == 31 =>
                        {
                            Some(Op::Autizb64ZDp1Src)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 14 && x4 == 31 =>
                        {
                            Some(Op::Autdza64ZDp1Src)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 15 && x4 == 31 =>
                        {
                            Some(Op::Autdzb64ZDp1Src)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 16 && x4 == 31 =>
                        {
                            Some(Op::Xpacd64ZDp1Src)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 17 && x4 == 31 =>
                        {
                            Some(Op::Xpacd64ZDp1Src)
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
                        _ => None,
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
                        (x0, _, _, x3) if x0 == 0 && x3 & 32 == 32 => None,
                        (x0, x1, x2, _) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            Some(Op::Bics64LogShift)
                        }
                        (x0, x1, x2, _) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            Some(Op::Bics64LogShift)
                        }
                        (x0, x1, x2, _) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            Some(Op::Bics64LogShift)
                        }
                        (x0, x1, x2, _) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            Some(Op::Bics64LogShift)
                        }
                        (x0, x1, x2, _) if x0 == 0 && x1 == 2 && x2 == 0 => {
                            Some(Op::Bics64LogShift)
                        }
                        (x0, x1, x2, _) if x0 == 0 && x1 == 2 && x2 == 1 => {
                            Some(Op::Bics64LogShift)
                        }
                        (x0, x1, x2, _) if x0 == 0 && x1 == 3 && x2 == 0 => {
                            Some(Op::Bics64LogShift)
                        }
                        (x0, x1, x2, _) if x0 == 0 && x1 == 3 && x2 == 1 => {
                            Some(Op::Bics64LogShift)
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            Some(Op::Bics64LogShift)
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            Some(Op::Bics64LogShift)
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            Some(Op::Bics64LogShift)
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            Some(Op::Bics64LogShift)
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 2 && x2 == 0 => {
                            Some(Op::Bics64LogShift)
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 2 && x2 == 1 => {
                            Some(Op::Bics64LogShift)
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 3 && x2 == 0 => {
                            Some(Op::Bics64LogShift)
                        }
                        (x0, x1, x2, _) if x0 == 1 && x1 == 3 && x2 == 1 => {
                            Some(Op::Bics64LogShift)
                        }
                        _ => None,
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
                        (_, _, _, x3, _) if x3 == 3 => None,
                        (x0, _, _, _, x4) if x0 == 0 && x4 & 32 == 32 => None,
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            Some(Op::Subs64AddsubShift)
                        }
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            Some(Op::Subs64AddsubShift)
                        }
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            Some(Op::Subs64AddsubShift)
                        }
                        (x0, x1, x2, _, _) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            Some(Op::Subs64AddsubShift)
                        }
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            Some(Op::Subs64AddsubShift)
                        }
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            Some(Op::Subs64AddsubShift)
                        }
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            Some(Op::Subs64AddsubShift)
                        }
                        (x0, x1, x2, _, _) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            Some(Op::Subs64AddsubShift)
                        }
                        _ => None,
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
                        (_, _, _, _, x4) if x4 & 5 == 5 => None,
                        (_, _, _, _, x4) if x4 & 6 == 6 => None,
                        (_, _, _, x3, _) if x3 & 1 == 1 => None,
                        (_, _, _, x3, _) if x3 & 2 == 2 => None,
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            Some(Op::Subs64SAddsubExt)
                        }
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            Some(Op::Subs64SAddsubExt)
                        }
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 0 => {
                            Some(Op::Subs64SAddsubExt)
                        }
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 0 => {
                            Some(Op::Subs64SAddsubExt)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            Some(Op::Subs64SAddsubExt)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            Some(Op::Subs64SAddsubExt)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 0 => {
                            Some(Op::Subs64SAddsubExt)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 0 => {
                            Some(Op::Subs64SAddsubExt)
                        }
                        _ => None,
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
                            Some(Op::Sbcs64AddsubCarry)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            Some(Op::Sbcs64AddsubCarry)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            Some(Op::Sbcs64AddsubCarry)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            Some(Op::Sbcs64AddsubCarry)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            Some(Op::Sbcs64AddsubCarry)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => {
                            Some(Op::Sbcs64AddsubCarry)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            Some(Op::Sbcs64AddsubCarry)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => {
                            Some(Op::Sbcs64AddsubCarry)
                        }
                        _ => None,
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
                        (x0, _, _, _) if x0 == 0 => None,
                        (x0, x1, x2, _) if x0 == 1 && x1 == 0 && x2 == 0 => None,
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            Some(Op::RmifOnlyRmif)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 => None,
                        (x0, x1, _, _) if x0 == 1 && x1 == 1 => None,
                        _ => None,
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
                        (x0, x1, x2, _, _, _, _) if x0 == 0 && x1 == 0 && x2 == 0 => None,
                        (x0, x1, x2, x3, _, _, _) if x0 == 0 && x1 == 0 && x2 == 1 && x3 != 0 => {
                            None
                        }
                        (x0, x1, x2, x3, _, x5, x6)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x5 == 0 && x6 != 13 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, _, x5, _)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x5 == 1 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4, x5, x6)
                            if x0 == 0
                                && x1 == 0
                                && x2 == 1
                                && x3 == 0
                                && x4 == 0
                                && x5 == 0
                                && x6 == 13 =>
                        {
                            Some(Op::Setf16OnlySetf)
                        }
                        (x0, x1, x2, x3, x4, x5, x6)
                            if x0 == 0
                                && x1 == 0
                                && x2 == 1
                                && x3 == 0
                                && x4 == 1
                                && x5 == 0
                                && x6 == 13 =>
                        {
                            Some(Op::Setf16OnlySetf)
                        }
                        (x0, x1, _, _, _, _, _) if x0 == 0 && x1 == 1 => None,
                        (x0, _, _, _, _, _, _) if x0 == 1 => None,
                        _ => None,
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
                        (_, _, _, _, x4) if x4 == 1 => None,
                        (_, _, _, x3, _) if x3 == 1 => None,
                        (_, _, x2, _, _) if x2 == 0 => None,
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 =>
                        {
                            Some(Op::Ccmp64CondcmpReg)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 0 && x4 == 0 =>
                        {
                            Some(Op::Ccmp64CondcmpReg)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 =>
                        {
                            Some(Op::Ccmp64CondcmpReg)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 0 && x4 == 0 =>
                        {
                            Some(Op::Ccmp64CondcmpReg)
                        }
                        _ => None,
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
                        (_, _, _, _, x4) if x4 == 1 => None,
                        (_, _, _, x3, _) if x3 == 1 => None,
                        (_, _, x2, _, _) if x2 == 0 => None,
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 =>
                        {
                            Some(Op::Ccmp64CondcmpImm)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 0 && x4 == 0 =>
                        {
                            Some(Op::Ccmp64CondcmpImm)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 =>
                        {
                            Some(Op::Ccmp64CondcmpImm)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 0 && x4 == 0 =>
                        {
                            Some(Op::Ccmp64CondcmpImm)
                        }
                        _ => None,
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
                        (_, _, _, x3) if x3 & 2 == 2 => None,
                        (_, _, x2, _) if x2 == 1 => None,
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            Some(Op::Csneg64Condsel)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            Some(Op::Csneg64Condsel)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 0 => {
                            Some(Op::Csneg64Condsel)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 1 => {
                            Some(Op::Csneg64Condsel)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            Some(Op::Csneg64Condsel)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            Some(Op::Csneg64Condsel)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 0 => {
                            Some(Op::Csneg64Condsel)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 1 => {
                            Some(Op::Csneg64Condsel)
                        }
                        _ => None,
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
                        (_, x1, x2, x3) if x1 == 0 && x2 == 2 && x3 == 1 => None,
                        (_, x1, x2, _) if x1 == 0 && x2 == 3 => None,
                        (_, x1, x2, _) if x1 == 0 && x2 == 4 => None,
                        (_, x1, x2, x3) if x1 == 0 && x2 == 6 && x3 == 1 => None,
                        (_, x1, x2, _) if x1 == 0 && x2 == 7 => None,
                        (_, x1, _, _) if x1 == 1 => None,
                        (_, x1, _, _) if x1 & 2 == 2 => None,
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            Some(Op::Msub64ADp3Src)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            Some(Op::Msub64ADp3Src)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => None,
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 => None,
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 2 && x3 == 0 => None,
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 5 && x3 == 0 => None,
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 5 && x3 == 1 => None,
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 6 && x3 == 0 => None,
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            Some(Op::Msub64ADp3Src)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            Some(Op::Msub64ADp3Src)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            Some(Op::Umsubl64WaDp3Src)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 => {
                            Some(Op::Umsubl64WaDp3Src)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 2 && x3 == 0 => {
                            Some(Op::Umulh64Dp3Src)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 5 && x3 == 0 => {
                            Some(Op::Umsubl64WaDp3Src)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 5 && x3 == 1 => {
                            Some(Op::Umsubl64WaDp3Src)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 6 && x3 == 0 => {
                            Some(Op::Umulh64Dp3Src)
                        }
                        _ => None,
                    }
                }
                _ => None,
            }
        }
        (_, x1, _) if x1 & 14 == 14 => {
            match (
                (instr >> 28) & 7,
                (instr >> 25) & 5,
                (instr >> 23) & 3,
                (instr >> 19) & 7,
                (instr >> 10) & 17,
                instr & 19,
            ) {
                (x0, _, x2, x3, x4, _)
                    if x0 == 0 && x2 & 2 == 0 && x3 & 7 == 5 && x4 & 387 == 2 =>
                {
                    None
                }
                (x0, _, x2, x3, x4, _)
                    if x0 == 2 && x2 & 2 == 0 && x3 & 7 == 5 && x4 & 387 == 2 =>
                {
                    None
                }
                (x0, _, x2, x3, x4, _)
                    if x0 == 4 && x2 & 2 == 0 && x3 & 7 == 5 && x4 & 387 == 2 =>
                {
                    let size = (instr >> 22) & 3;
                    let opcode = (instr >> 12) & 9;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (size, opcode) {
                        (_, x1) if x1 & 8 == 8 => None,
                        (_, x1) if x1 & 28 == 0 => None,
                        (_, x1) if x1 & 16 == 16 => None,
                        (x0, _) if x0 & 1 == 1 => None,
                        (x0, x1) if x0 == 0 && x1 == 4 => Some(Op::AesdBCryptoaes),
                        (x0, x1) if x0 == 0 && x1 == 5 => Some(Op::AesdBCryptoaes),
                        (x0, x1) if x0 == 0 && x1 == 6 => Some(Op::AesimcBCryptoaes),
                        (x0, x1) if x0 == 0 && x1 == 7 => Some(Op::AesimcBCryptoaes),
                        (x0, _) if x0 & 2 == 2 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 == 5 && x2 & 2 == 0 && x3 & 4 == 0 && x4 & 35 == 0 => {
                    let size = (instr >> 22) & 3;
                    let Rm = (instr >> 16) & 9;
                    let opcode = (instr >> 12) & 5;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (size, opcode) {
                        (_, x1) if x1 == 7 => None,
                        (x0, _) if x0 & 1 == 1 => None,
                        (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::Sha1CQsvCryptosha3),
                        (x0, x1) if x0 == 0 && x1 == 1 => Some(Op::Sha1PQsvCryptosha3),
                        (x0, x1) if x0 == 0 && x1 == 2 => Some(Op::Sha1MQsvCryptosha3),
                        (x0, x1) if x0 == 0 && x1 == 3 => Some(Op::Sha1Su0VvvCryptosha3),
                        (x0, x1) if x0 == 0 && x1 == 4 => Some(Op::Sha256H2QqvCryptosha3),
                        (x0, x1) if x0 == 0 && x1 == 5 => Some(Op::Sha256H2QqvCryptosha3),
                        (x0, x1) if x0 == 0 && x1 == 6 => Some(Op::Sha256Su1VvvCryptosha3),
                        (x0, _) if x0 & 2 == 2 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 == 5 && x2 & 2 == 0 && x3 & 4 == 0 && x4 & 35 == 2 => {
                    None
                }
                (x0, _, x2, x3, x4, _)
                    if x0 == 5 && x2 & 2 == 0 && x3 & 7 == 5 && x4 & 387 == 2 =>
                {
                    let size = (instr >> 22) & 3;
                    let opcode = (instr >> 12) & 9;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (size, opcode) {
                        (_, x1) if x1 & 4 == 4 => None,
                        (_, x1) if x1 & 8 == 8 => None,
                        (_, x1) if x1 & 16 == 16 => None,
                        (x0, _) if x0 & 1 == 1 => None,
                        (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::Sha1HSsCryptosha2),
                        (x0, x1) if x0 == 0 && x1 == 1 => Some(Op::Sha1Su1VvCryptosha2),
                        (x0, x1) if x0 == 0 && x1 == 2 => Some(Op::Sha256Su0VvCryptosha2),
                        (x0, x1) if x0 == 0 && x1 == 3 => None,
                        (x0, _) if x0 & 2 == 2 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, x3, x4, _)
                    if x0 == 6 && x2 & 2 == 0 && x3 & 7 == 5 && x4 & 387 == 2 =>
                {
                    None
                }
                (x0, _, x2, x3, x4, _) if x0 == 7 && x2 & 2 == 0 && x3 & 4 == 0 && x4 & 33 == 0 => {
                    None
                }
                (x0, _, x2, x3, x4, _)
                    if x0 == 7 && x2 & 2 == 0 && x3 & 7 == 5 && x4 & 387 == 2 =>
                {
                    None
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 13 == 5 && x2 == 0 && x3 & 12 == 0 && x4 & 33 == 1 =>
                {
                    let op = (instr >> 29) & 1;
                    let imm5 = (instr >> 16) & 9;
                    let imm4 = (instr >> 11) & 7;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (op, imm5, imm4) {
                        (x0, _, x2) if x0 == 0 && x2 & 1 == 1 => None,
                        (x0, _, x2) if x0 == 0 && x2 & 2 == 2 => None,
                        (x0, _, x2) if x0 == 0 && x2 & 4 == 4 => None,
                        (x0, _, x2) if x0 == 0 && x2 == 0 => Some(Op::DupAsisdoneOnly),
                        (x0, _, x2) if x0 == 0 && x2 & 8 == 8 => None,
                        (x0, x1, x2) if x0 == 0 && x1 & 15 == 0 && x2 == 0 => None,
                        (x0, _, _) if x0 == 1 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 13 == 5 && x2 == 1 && x3 & 12 == 0 && x4 & 33 == 1 =>
                {
                    None
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 13 == 5 && x2 & 2 == 0 && x3 == 7 && x4 & 387 == 2 =>
                {
                    None
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 13 == 5 && x2 & 2 == 0 && x3 & 12 == 8 && x4 & 49 == 1 =>
                {
                    let U = (instr >> 29) & 1;
                    let a = (instr >> 23) & 1;
                    let Rm = (instr >> 16) & 9;
                    let opcode = (instr >> 11) & 5;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (U, a, opcode) {
                        (_, _, x2) if x2 == 6 => None,
                        (_, x1, x2) if x1 == 1 && x2 == 3 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 3 => {
                            Some(Op::FmulxAsisdsamefp16Only)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 4 => {
                            Some(Op::FacgtAsisdsamefp16Only)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 5 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 7 => {
                            Some(Op::FrecpsAsisdsamefp16Only)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 4 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 5 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 7 => {
                            Some(Op::FrsqrtsAsisdsamefp16Only)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 3 => None,
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 4 => {
                            Some(Op::FacgtAsisdsamefp16Only)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 5 => {
                            Some(Op::FacgtAsisdsamefp16Only)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 7 => None,
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 2 => {
                            Some(Op::FabdAsisdsamefp16Only)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 4 => {
                            Some(Op::FacgtAsisdsamefp16Only)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 5 => {
                            Some(Op::FacgtAsisdsamefp16Only)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 7 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 13 == 5 && x2 & 2 == 0 && x3 & 12 == 8 && x4 & 49 == 17 =>
                {
                    None
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 13 == 5 && x2 & 2 == 0 && x3 == 15 && x4 & 387 == 2 =>
                {
                    let U = (instr >> 29) & 1;
                    let a = (instr >> 23) & 1;
                    let opcode = (instr >> 12) & 9;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (U, a, opcode) {
                        (_, _, x2) if x2 & 24 == 0 => None,
                        (_, _, x2) if x2 & 28 == 8 => None,
                        (_, _, x2) if x2 & 24 == 16 => None,
                        (_, _, x2) if x2 & 30 == 24 => None,
                        (_, _, x2) if x2 == 30 => None,
                        (_, x1, x2) if x1 == 0 && x2 & 28 == 12 => None,
                        (_, x1, x2) if x1 == 0 && x2 == 31 => None,
                        (_, x1, x2) if x1 == 1 && x2 == 15 => None,
                        (_, x1, x2) if x1 == 1 && x2 == 28 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 26 => {
                            Some(Op::FcvtzuAsisdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 27 => {
                            Some(Op::FcvtzuAsisdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 28 => {
                            Some(Op::FcvtauAsisdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 29 => {
                            Some(Op::UcvtfAsisdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 12 => {
                            Some(Op::FcmleAsisdmiscfp16Fz)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 13 => {
                            Some(Op::FcmleAsisdmiscfp16Fz)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 14 => {
                            Some(Op::FcmltAsisdmiscfp16Fz)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 26 => {
                            Some(Op::FcvtzuAsisdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 27 => {
                            Some(Op::FcvtzuAsisdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 29 => {
                            Some(Op::FrecpeAsisdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 31 => {
                            Some(Op::FrecpxAsisdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 26 => {
                            Some(Op::FcvtzuAsisdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 27 => {
                            Some(Op::FcvtzuAsisdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 28 => {
                            Some(Op::FcvtauAsisdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 29 => {
                            Some(Op::UcvtfAsisdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 12 => {
                            Some(Op::FcmleAsisdmiscfp16Fz)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 13 => {
                            Some(Op::FcmleAsisdmiscfp16Fz)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 14 => None,
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 26 => {
                            Some(Op::FcvtzuAsisdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 27 => {
                            Some(Op::FcvtzuAsisdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 29 => {
                            Some(Op::FrsqrteAsisdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 31 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 13 == 5 && x2 & 2 == 0 && x3 & 4 == 0 && x4 & 33 == 32 =>
                {
                    None
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 13 == 5 && x2 & 2 == 0 && x3 & 4 == 0 && x4 & 33 == 33 =>
                {
                    let U = (instr >> 29) & 1;
                    let size = (instr >> 22) & 3;
                    let Rm = (instr >> 16) & 9;
                    let opcode = (instr >> 11) & 7;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (U, opcode) {
                        (_, x1) if x1 & 14 == 2 => None,
                        (_, x1) if x1 & 12 == 4 => None,
                        (_, x1) if x1 & 8 == 8 => None,
                        (x0, x1) if x0 == 0 && x1 == 0 => None,
                        (x0, x1) if x0 == 0 && x1 == 1 => None,
                        (x0, x1) if x0 == 1 && x1 == 0 => Some(Op::SqrdmlshAsisdsame2Only),
                        (x0, x1) if x0 == 1 && x1 == 1 => Some(Op::SqrdmlshAsisdsame2Only),
                        _ => None,
                    }
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 13 == 5 && x2 & 2 == 0 && x3 & 7 == 4 && x4 & 387 == 2 =>
                {
                    let U = (instr >> 29) & 1;
                    let size = (instr >> 22) & 3;
                    let opcode = (instr >> 12) & 9;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (U, size, opcode) {
                        (_, _, x2) if x2 & 30 == 0 => None,
                        (_, _, x2) if x2 == 2 => None,
                        (_, _, x2) if x2 & 30 == 4 => None,
                        (_, _, x2) if x2 == 6 => None,
                        (_, _, x2) if x2 == 15 => None,
                        (_, _, x2) if x2 & 30 == 16 => None,
                        (_, _, x2) if x2 == 19 => None,
                        (_, _, x2) if x2 == 21 => None,
                        (_, _, x2) if x2 == 23 => None,
                        (_, _, x2) if x2 & 30 == 24 => None,
                        (_, _, x2) if x2 == 30 => None,
                        (_, x1, x2) if x1 & 2 == 0 && x2 & 28 == 12 => None,
                        (_, x1, x2) if x1 & 2 == 0 && x2 == 31 => None,
                        (_, x1, x2) if x1 & 2 == 2 && x2 == 22 => None,
                        (_, x1, x2) if x1 & 2 == 2 && x2 == 28 => None,
                        (x0, _, x2) if x0 == 0 && x2 == 3 => Some(Op::UsqaddAsisdmiscR),
                        (x0, _, x2) if x0 == 0 && x2 == 7 => Some(Op::SqnegAsisdmiscR),
                        (x0, _, x2) if x0 == 0 && x2 == 8 => Some(Op::CmleAsisdmiscZ),
                        (x0, _, x2) if x0 == 0 && x2 == 9 => Some(Op::CmleAsisdmiscZ),
                        (x0, _, x2) if x0 == 0 && x2 == 10 => Some(Op::CmltAsisdmiscZ),
                        (x0, _, x2) if x0 == 0 && x2 == 11 => Some(Op::NegAsisdmiscR),
                        (x0, _, x2) if x0 == 0 && x2 == 18 => None,
                        (x0, _, x2) if x0 == 0 && x2 == 20 => Some(Op::UqxtnAsisdmiscN),
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 22 => None,
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 26 => {
                            Some(Op::FcvtzuAsisdmiscR)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 27 => {
                            Some(Op::FcvtzuAsisdmiscR)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 28 => {
                            Some(Op::FcvtauAsisdmiscR)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 29 => {
                            Some(Op::UcvtfAsisdmiscR)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 12 => {
                            Some(Op::FcmleAsisdmiscFz)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 13 => {
                            Some(Op::FcmleAsisdmiscFz)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 14 => {
                            Some(Op::FcmltAsisdmiscFz)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 26 => {
                            Some(Op::FcvtzuAsisdmiscR)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 27 => {
                            Some(Op::FcvtzuAsisdmiscR)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 29 => {
                            Some(Op::FrecpeAsisdmiscR)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 31 => {
                            Some(Op::FrecpxAsisdmiscR)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 3 => Some(Op::UsqaddAsisdmiscR),
                        (x0, _, x2) if x0 == 1 && x2 == 7 => Some(Op::SqnegAsisdmiscR),
                        (x0, _, x2) if x0 == 1 && x2 == 8 => Some(Op::CmleAsisdmiscZ),
                        (x0, _, x2) if x0 == 1 && x2 == 9 => Some(Op::CmleAsisdmiscZ),
                        (x0, _, x2) if x0 == 1 && x2 == 10 => None,
                        (x0, _, x2) if x0 == 1 && x2 == 11 => Some(Op::NegAsisdmiscR),
                        (x0, _, x2) if x0 == 1 && x2 == 18 => Some(Op::SqxtunAsisdmiscN),
                        (x0, _, x2) if x0 == 1 && x2 == 20 => Some(Op::UqxtnAsisdmiscN),
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 22 => {
                            Some(Op::FcvtxnAsisdmiscN)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 26 => {
                            Some(Op::FcvtzuAsisdmiscR)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 27 => {
                            Some(Op::FcvtzuAsisdmiscR)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 28 => {
                            Some(Op::FcvtauAsisdmiscR)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 29 => {
                            Some(Op::UcvtfAsisdmiscR)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 12 => {
                            Some(Op::FcmleAsisdmiscFz)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 13 => {
                            Some(Op::FcmleAsisdmiscFz)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 14 => None,
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 26 => {
                            Some(Op::FcvtzuAsisdmiscR)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 27 => {
                            Some(Op::FcvtzuAsisdmiscR)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 29 => {
                            Some(Op::FrsqrteAsisdmiscR)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 31 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 13 == 5 && x2 & 2 == 0 && x3 & 7 == 6 && x4 & 387 == 2 =>
                {
                    let U = (instr >> 29) & 1;
                    let size = (instr >> 22) & 3;
                    let opcode = (instr >> 12) & 9;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (U, size, opcode) {
                        (_, _, x2) if x2 & 24 == 0 => None,
                        (_, _, x2) if x2 & 28 == 8 => None,
                        (_, _, x2) if x2 == 14 => None,
                        (_, _, x2) if x2 & 24 == 16 => None,
                        (_, _, x2) if x2 & 30 == 24 => None,
                        (_, _, x2) if x2 == 26 => None,
                        (_, _, x2) if x2 & 28 == 28 => None,
                        (_, x1, x2) if x1 & 2 == 2 && x2 == 13 => None,
                        (x0, _, x2) if x0 == 0 && x2 == 27 => Some(Op::AddpAsisdpairOnly),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 12 => {
                            Some(Op::FminnmpAsisdpairOnlyH)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 13 => {
                            Some(Op::FaddpAsisdpairOnlyH)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 15 => {
                            Some(Op::FminpAsisdpairOnlyH)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 12 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 13 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 15 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 12 => {
                            Some(Op::FminnmpAsisdpairOnlyH)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 15 => {
                            Some(Op::FminpAsisdpairOnlyH)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 12 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 15 => None,
                        (x0, _, x2) if x0 == 1 && x2 == 27 => None,
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 12 => {
                            Some(Op::FminnmpAsisdpairOnlySd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 13 => {
                            Some(Op::FaddpAsisdpairOnlySd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 15 => {
                            Some(Op::FminpAsisdpairOnlySd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 12 => {
                            Some(Op::FminnmpAsisdpairOnlySd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 15 => {
                            Some(Op::FminpAsisdpairOnlySd)
                        }
                        _ => None,
                    }
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 13 == 5 && x2 & 2 == 0 && x3 & 4 == 4 && x4 & 259 == 258 =>
                {
                    None
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 13 == 5 && x2 & 2 == 0 && x3 & 4 == 4 && x4 & 131 == 130 =>
                {
                    None
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 13 == 5 && x2 & 2 == 0 && x3 & 4 == 4 && x4 & 3 == 0 =>
                {
                    let U = (instr >> 29) & 1;
                    let size = (instr >> 22) & 3;
                    let Rm = (instr >> 16) & 9;
                    let opcode = (instr >> 12) & 7;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (U, opcode) {
                        (_, x1) if x1 & 12 == 0 => None,
                        (_, x1) if x1 & 12 == 4 => None,
                        (_, x1) if x1 == 8 => None,
                        (_, x1) if x1 == 10 => None,
                        (_, x1) if x1 == 12 => None,
                        (_, x1) if x1 & 14 == 14 => None,
                        (x0, x1) if x0 == 0 && x1 == 9 => Some(Op::SqdmlslAsisddiffOnly),
                        (x0, x1) if x0 == 0 && x1 == 11 => Some(Op::SqdmlslAsisddiffOnly),
                        (x0, x1) if x0 == 0 && x1 == 13 => Some(Op::SqdmullAsisddiffOnly),
                        (x0, x1) if x0 == 1 && x1 == 9 => None,
                        (x0, x1) if x0 == 1 && x1 == 11 => None,
                        (x0, x1) if x0 == 1 && x1 == 13 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 13 == 5 && x2 & 2 == 0 && x3 & 4 == 4 && x4 & 1 == 1 =>
                {
                    let U = (instr >> 29) & 1;
                    let size = (instr >> 22) & 3;
                    let Rm = (instr >> 16) & 9;
                    let opcode = (instr >> 11) & 9;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (U, size, opcode) {
                        (_, _, x2) if x2 == 0 => None,
                        (_, _, x2) if x2 & 30 == 2 => None,
                        (_, _, x2) if x2 == 4 => None,
                        (_, _, x2) if x2 & 28 == 12 => None,
                        (_, _, x2) if x2 & 30 == 18 => None,
                        (_, x1, x2) if x1 & 2 == 2 && x2 == 27 => None,
                        (x0, _, x2) if x0 == 0 && x2 == 1 => Some(Op::UqaddAsisdsameOnly),
                        (x0, _, x2) if x0 == 0 && x2 == 5 => Some(Op::UqsubAsisdsameOnly),
                        (x0, _, x2) if x0 == 0 && x2 == 6 => Some(Op::CmhsAsisdsameOnly),
                        (x0, _, x2) if x0 == 0 && x2 == 7 => Some(Op::CmhsAsisdsameOnly),
                        (x0, _, x2) if x0 == 0 && x2 == 8 => Some(Op::UqrshlAsisdsameOnly),
                        (x0, _, x2) if x0 == 0 && x2 == 9 => Some(Op::UqrshlAsisdsameOnly),
                        (x0, _, x2) if x0 == 0 && x2 == 10 => Some(Op::UqrshlAsisdsameOnly),
                        (x0, _, x2) if x0 == 0 && x2 == 11 => Some(Op::UqrshlAsisdsameOnly),
                        (x0, _, x2) if x0 == 0 && x2 == 16 => Some(Op::SubAsisdsameOnly),
                        (x0, _, x2) if x0 == 0 && x2 == 17 => Some(Op::CmeqAsisdsameOnly),
                        (x0, _, x2) if x0 == 0 && x2 == 20 => None,
                        (x0, _, x2) if x0 == 0 && x2 == 21 => None,
                        (x0, _, x2) if x0 == 0 && x2 == 22 => Some(Op::SqrdmulhAsisdsameOnly),
                        (x0, _, x2) if x0 == 0 && x2 == 23 => None,
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 24 => None,
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 25 => None,
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 26 => None,
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 27 => {
                            Some(Op::FmulxAsisdsameOnly)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 28 => {
                            Some(Op::FacgtAsisdsameOnly)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 29 => None,
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 30 => None,
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 31 => {
                            Some(Op::FrecpsAsisdsameOnly)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 24 => None,
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 25 => None,
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 26 => None,
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 28 => None,
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 29 => None,
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 30 => None,
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 31 => {
                            Some(Op::FrsqrtsAsisdsameOnly)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 1 => Some(Op::UqaddAsisdsameOnly),
                        (x0, _, x2) if x0 == 1 && x2 == 5 => Some(Op::UqsubAsisdsameOnly),
                        (x0, _, x2) if x0 == 1 && x2 == 6 => Some(Op::CmhsAsisdsameOnly),
                        (x0, _, x2) if x0 == 1 && x2 == 7 => Some(Op::CmhsAsisdsameOnly),
                        (x0, _, x2) if x0 == 1 && x2 == 8 => Some(Op::UqrshlAsisdsameOnly),
                        (x0, _, x2) if x0 == 1 && x2 == 9 => Some(Op::UqrshlAsisdsameOnly),
                        (x0, _, x2) if x0 == 1 && x2 == 10 => Some(Op::UqrshlAsisdsameOnly),
                        (x0, _, x2) if x0 == 1 && x2 == 11 => Some(Op::UqrshlAsisdsameOnly),
                        (x0, _, x2) if x0 == 1 && x2 == 16 => Some(Op::SubAsisdsameOnly),
                        (x0, _, x2) if x0 == 1 && x2 == 17 => Some(Op::CmeqAsisdsameOnly),
                        (x0, _, x2) if x0 == 1 && x2 == 20 => None,
                        (x0, _, x2) if x0 == 1 && x2 == 21 => None,
                        (x0, _, x2) if x0 == 1 && x2 == 22 => Some(Op::SqrdmulhAsisdsameOnly),
                        (x0, _, x2) if x0 == 1 && x2 == 23 => None,
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 24 => None,
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 25 => None,
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 26 => None,
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 27 => None,
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 28 => {
                            Some(Op::FacgtAsisdsameOnly)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 29 => {
                            Some(Op::FacgtAsisdsameOnly)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 30 => None,
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 31 => None,
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 24 => None,
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 25 => None,
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 26 => {
                            Some(Op::FabdAsisdsameOnly)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 28 => {
                            Some(Op::FacgtAsisdsameOnly)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 29 => {
                            Some(Op::FacgtAsisdsameOnly)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 30 => None,
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 31 => None,
                        _ => None,
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
                        (_, x1, x2) if x1 != 0 && x2 == 1 => None,
                        (_, x1, x2) if x1 != 0 && x2 == 3 => None,
                        (_, x1, x2) if x1 != 0 && x2 == 5 => None,
                        (_, x1, x2) if x1 != 0 && x2 == 7 => None,
                        (_, x1, x2) if x1 != 0 && x2 == 9 => None,
                        (_, x1, x2) if x1 != 0 && x2 == 11 => None,
                        (_, x1, x2) if x1 != 0 && x2 == 13 => None,
                        (_, x1, x2) if x1 != 0 && x2 == 15 => None,
                        (_, x1, x2) if x1 != 0 && x2 & 28 == 20 => None,
                        (_, x1, x2) if x1 != 0 && x2 & 28 == 24 => None,
                        (_, x1, x2) if x1 != 0 && x2 == 29 => None,
                        (_, x1, x2) if x1 != 0 && x2 == 30 => None,
                        (_, x1, _) if x1 == 0 => None,
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 0 => Some(Op::UrsraAsisdshfR),
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 2 => Some(Op::UrsraAsisdshfR),
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 4 => Some(Op::UrsraAsisdshfR),
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 6 => Some(Op::UrsraAsisdshfR),
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 8 => None,
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 10 => Some(Op::ShlAsisdshfR),
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 12 => None,
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 14 => Some(Op::UqshlAsisdshfR),
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 16 => None,
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 17 => None,
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 18 => {
                            Some(Op::UqrshrnAsisdshfN)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 19 => {
                            Some(Op::UqrshrnAsisdshfN)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 28 => Some(Op::UcvtfAsisdshfC),
                        (x0, x1, x2) if x0 == 0 && x1 != 0 && x2 == 31 => Some(Op::FcvtzuAsisdshfC),
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 0 => Some(Op::UrsraAsisdshfR),
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 2 => Some(Op::UrsraAsisdshfR),
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 4 => Some(Op::UrsraAsisdshfR),
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 6 => Some(Op::UrsraAsisdshfR),
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 8 => Some(Op::SriAsisdshfR),
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 10 => Some(Op::SliAsisdshfR),
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 12 => Some(Op::UqshlAsisdshfR),
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 14 => Some(Op::UqshlAsisdshfR),
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 16 => {
                            Some(Op::SqrshrunAsisdshfN)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 17 => {
                            Some(Op::SqrshrunAsisdshfN)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 18 => {
                            Some(Op::UqrshrnAsisdshfN)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 19 => {
                            Some(Op::UqrshrnAsisdshfN)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 28 => Some(Op::UcvtfAsisdshfC),
                        (x0, x1, x2) if x0 == 1 && x1 != 0 && x2 == 31 => Some(Op::FcvtzuAsisdshfC),
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _) if x0 & 13 == 5 && x2 == 3 && x4 & 1 == 1 => None,
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
                        (_, _, x2) if x2 == 0 => None,
                        (_, _, x2) if x2 == 2 => None,
                        (_, _, x2) if x2 == 4 => None,
                        (_, _, x2) if x2 == 6 => None,
                        (_, _, x2) if x2 == 8 => None,
                        (_, _, x2) if x2 == 10 => None,
                        (_, _, x2) if x2 == 14 => None,
                        (_, x1, x2) if x1 == 1 && x2 == 1 => None,
                        (_, x1, x2) if x1 == 1 && x2 == 5 => None,
                        (_, x1, x2) if x1 == 1 && x2 == 9 => None,
                        (x0, _, x2) if x0 == 0 && x2 == 3 => Some(Op::SqdmlslAsisdelemL),
                        (x0, _, x2) if x0 == 0 && x2 == 7 => Some(Op::SqdmlslAsisdelemL),
                        (x0, _, x2) if x0 == 0 && x2 == 11 => Some(Op::SqdmullAsisdelemL),
                        (x0, _, x2) if x0 == 0 && x2 == 12 => Some(Op::SqrdmulhAsisdelemR),
                        (x0, _, x2) if x0 == 0 && x2 == 13 => Some(Op::SqrdmulhAsisdelemR),
                        (x0, _, x2) if x0 == 0 && x2 == 15 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => Some(Op::FmlsAsisdelemRhH),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 5 => Some(Op::FmlsAsisdelemRhH),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 9 => {
                            Some(Op::FmulxAsisdelemRhH)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 1 => {
                            Some(Op::FmlsAsisdelemRSd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 5 => {
                            Some(Op::FmlsAsisdelemRSd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 9 => {
                            Some(Op::FmulxAsisdelemRSd)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 3 => None,
                        (x0, _, x2) if x0 == 1 && x2 == 7 => None,
                        (x0, _, x2) if x0 == 1 && x2 == 11 => None,
                        (x0, _, x2) if x0 == 1 && x2 == 12 => None,
                        (x0, _, x2) if x0 == 1 && x2 == 13 => Some(Op::SqrdmlshAsisdelemR),
                        (x0, _, x2) if x0 == 1 && x2 == 15 => Some(Op::SqrdmlshAsisdelemR),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => None,
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 5 => None,
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 9 => {
                            Some(Op::FmulxAsisdelemRhH)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 1 => None,
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 5 => None,
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 9 => {
                            Some(Op::FmulxAsisdelemRSd)
                        }
                        _ => None,
                    }
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 11 == 0 && x2 & 2 == 0 && x3 & 4 == 0 && x4 & 35 == 0 =>
                {
                    let Q = (instr >> 30) & 1;
                    let op2 = (instr >> 22) & 3;
                    let Rm = (instr >> 16) & 9;
                    let len = (instr >> 13) & 3;
                    let op = (instr >> 12) & 1;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (op2, len, op) {
                        (x0, _, _) if x0 & 1 == 1 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => Some(Op::TbxAsimdtblL44),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => Some(Op::TbxAsimdtblL44),
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => Some(Op::TbxAsimdtblL44),
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => Some(Op::TbxAsimdtblL44),
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 0 => Some(Op::TbxAsimdtblL44),
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 1 => Some(Op::TbxAsimdtblL44),
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 0 => Some(Op::TbxAsimdtblL44),
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 1 => Some(Op::TbxAsimdtblL44),
                        (x0, _, _) if x0 & 2 == 2 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 11 == 0 && x2 & 2 == 0 && x3 & 4 == 0 && x4 & 35 == 2 =>
                {
                    let Q = (instr >> 30) & 1;
                    let size = (instr >> 22) & 3;
                    let Rm = (instr >> 16) & 9;
                    let opcode = (instr >> 12) & 5;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match opcode {
                        x0 if x0 == 0 => None,
                        x0 if x0 == 1 => Some(Op::Uzp2AsimdpermOnly),
                        x0 if x0 == 2 => Some(Op::Trn2AsimdpermOnly),
                        x0 if x0 == 3 => Some(Op::Zip2AsimdpermOnly),
                        x0 if x0 == 4 => None,
                        x0 if x0 == 5 => Some(Op::Uzp2AsimdpermOnly),
                        x0 if x0 == 6 => Some(Op::Trn2AsimdpermOnly),
                        x0 if x0 == 7 => Some(Op::Zip2AsimdpermOnly),
                        _ => None,
                    }
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 11 == 2 && x2 & 2 == 0 && x3 & 4 == 0 && x4 & 33 == 0 =>
                {
                    let Q = (instr >> 30) & 1;
                    let op2 = (instr >> 22) & 3;
                    let Rm = (instr >> 16) & 9;
                    let imm4 = (instr >> 11) & 7;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match op2 {
                        x0 if x0 & 1 == 1 => None,
                        x0 if x0 == 0 => Some(Op::ExtAsimdextOnly),
                        x0 if x0 & 2 == 2 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 9 == 0 && x2 == 0 && x3 & 12 == 0 && x4 & 33 == 1 =>
                {
                    let Q = (instr >> 30) & 1;
                    let op = (instr >> 29) & 1;
                    let imm5 = (instr >> 16) & 9;
                    let imm4 = (instr >> 11) & 7;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (Q, op, imm5, imm4) {
                        (_, _, x2, _) if x2 & 15 == 0 => None,
                        (_, x1, _, x3) if x1 == 0 && x3 == 0 => Some(Op::DupAsimdinsDvV),
                        (_, x1, _, x3) if x1 == 0 && x3 == 1 => Some(Op::DupAsimdinsDrR),
                        (_, x1, _, x3) if x1 == 0 && x3 == 2 => None,
                        (_, x1, _, x3) if x1 == 0 && x3 == 4 => None,
                        (_, x1, _, x3) if x1 == 0 && x3 == 6 => None,
                        (_, x1, _, x3) if x1 == 0 && x3 & 8 == 8 => None,
                        (x0, x1, _, x3) if x0 == 0 && x1 == 0 && x3 == 3 => None,
                        (x0, x1, _, x3) if x0 == 0 && x1 == 0 && x3 == 5 => {
                            Some(Op::SmovAsimdinsXX)
                        }
                        (x0, x1, _, x3) if x0 == 0 && x1 == 0 && x3 == 7 => {
                            Some(Op::UmovAsimdinsXX)
                        }
                        (x0, x1, _, _) if x0 == 0 && x1 == 1 => None,
                        (x0, x1, _, x3) if x0 == 1 && x1 == 0 && x3 == 3 => {
                            Some(Op::InsAsimdinsIrR)
                        }
                        (x0, x1, _, x3) if x0 == 1 && x1 == 0 && x3 == 5 => {
                            Some(Op::SmovAsimdinsXX)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 & 15 == 8 && x3 == 7 => {
                            Some(Op::UmovAsimdinsXX)
                        }
                        (x0, x1, _, _) if x0 == 1 && x1 == 1 => Some(Op::InsAsimdinsIvV),
                        _ => None,
                    }
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 9 == 0 && x2 == 1 && x3 & 12 == 0 && x4 & 33 == 1 =>
                {
                    None
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 9 == 0 && x2 & 2 == 0 && x3 == 7 && x4 & 387 == 2 =>
                {
                    None
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 9 == 0 && x2 & 2 == 0 && x3 & 12 == 8 && x4 & 49 == 1 =>
                {
                    let Q = (instr >> 30) & 1;
                    let U = (instr >> 29) & 1;
                    let a = (instr >> 23) & 1;
                    let Rm = (instr >> 16) & 9;
                    let opcode = (instr >> 11) & 5;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (U, a, opcode) {
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => {
                            Some(Op::FminnmpAsimdsamefp16Only)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => {
                            Some(Op::FmlsAsimdsamefp16Only)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 2 => {
                            Some(Op::FaddpAsimdsamefp16Only)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 3 => {
                            Some(Op::FmulxAsimdsamefp16Only)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 4 => {
                            Some(Op::FacgtAsimdsamefp16Only)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 5 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 6 => {
                            Some(Op::FminpAsimdsamefp16Only)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 7 => {
                            Some(Op::FrecpsAsimdsamefp16Only)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 0 => {
                            Some(Op::FminnmpAsimdsamefp16Only)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => {
                            Some(Op::FmlsAsimdsamefp16Only)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 2 => {
                            Some(Op::FabdAsimdsamefp16Only)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 3 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 4 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 5 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 6 => {
                            Some(Op::FminpAsimdsamefp16Only)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 7 => {
                            Some(Op::FrsqrtsAsimdsamefp16Only)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => {
                            Some(Op::FminnmpAsimdsamefp16Only)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => None,
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 2 => {
                            Some(Op::FaddpAsimdsamefp16Only)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 3 => {
                            Some(Op::FmulAsimdsamefp16Only)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 4 => {
                            Some(Op::FacgtAsimdsamefp16Only)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 5 => {
                            Some(Op::FacgtAsimdsamefp16Only)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 6 => {
                            Some(Op::FminpAsimdsamefp16Only)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 7 => {
                            Some(Op::FdivAsimdsamefp16Only)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => {
                            Some(Op::FminnmpAsimdsamefp16Only)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => None,
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 2 => {
                            Some(Op::FabdAsimdsamefp16Only)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 3 => None,
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 4 => {
                            Some(Op::FacgtAsimdsamefp16Only)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 5 => {
                            Some(Op::FacgtAsimdsamefp16Only)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 6 => {
                            Some(Op::FminpAsimdsamefp16Only)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 7 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 9 == 0 && x2 & 2 == 0 && x3 & 12 == 8 && x4 & 49 == 17 =>
                {
                    None
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 9 == 0 && x2 & 2 == 0 && x3 == 15 && x4 & 387 == 2 =>
                {
                    let Q = (instr >> 30) & 1;
                    let U = (instr >> 29) & 1;
                    let a = (instr >> 23) & 1;
                    let opcode = (instr >> 12) & 9;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (U, a, opcode) {
                        (_, _, x2) if x2 & 24 == 0 => None,
                        (_, _, x2) if x2 & 28 == 8 => None,
                        (_, _, x2) if x2 & 24 == 16 => None,
                        (_, _, x2) if x2 == 30 => None,
                        (_, x1, x2) if x1 == 0 && x2 & 28 == 12 => None,
                        (_, x1, x2) if x1 == 0 && x2 == 31 => None,
                        (_, x1, x2) if x1 == 1 && x2 == 28 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 24 => {
                            Some(Op::FrintiAsimdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 25 => {
                            Some(Op::FrintiAsimdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 26 => {
                            Some(Op::FcvtzuAsimdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 27 => {
                            Some(Op::FcvtzuAsimdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 28 => {
                            Some(Op::FcvtauAsimdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 29 => {
                            Some(Op::UcvtfAsimdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 12 => {
                            Some(Op::FcmleAsimdmiscfp16Fz)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 13 => {
                            Some(Op::FcmleAsimdmiscfp16Fz)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 14 => {
                            Some(Op::FcmltAsimdmiscfp16Fz)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 15 => {
                            Some(Op::FnegAsimdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 24 => {
                            Some(Op::FrintiAsimdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 25 => {
                            Some(Op::FrintiAsimdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 26 => {
                            Some(Op::FcvtzuAsimdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 27 => {
                            Some(Op::FcvtzuAsimdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 29 => {
                            Some(Op::FrecpeAsimdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 31 => None,
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 24 => {
                            Some(Op::FrintiAsimdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 25 => {
                            Some(Op::FrintiAsimdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 26 => {
                            Some(Op::FcvtzuAsimdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 27 => {
                            Some(Op::FcvtzuAsimdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 28 => {
                            Some(Op::FcvtauAsimdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 29 => {
                            Some(Op::UcvtfAsimdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 12 => {
                            Some(Op::FcmleAsimdmiscfp16Fz)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 13 => {
                            Some(Op::FcmleAsimdmiscfp16Fz)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 14 => None,
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 15 => {
                            Some(Op::FnegAsimdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 24 => None,
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 25 => {
                            Some(Op::FrintiAsimdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 26 => {
                            Some(Op::FcvtzuAsimdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 27 => {
                            Some(Op::FcvtzuAsimdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 29 => {
                            Some(Op::FrsqrteAsimdmiscfp16R)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 31 => {
                            Some(Op::FsqrtAsimdmiscfp16R)
                        }
                        _ => None,
                    }
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 9 == 0 && x2 & 2 == 0 && x3 & 4 == 0 && x4 & 33 == 32 =>
                {
                    None
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 9 == 0 && x2 & 2 == 0 && x3 & 4 == 0 && x4 & 33 == 33 =>
                {
                    let Q = (instr >> 30) & 1;
                    let U = (instr >> 29) & 1;
                    let size = (instr >> 22) & 3;
                    let Rm = (instr >> 16) & 9;
                    let opcode = (instr >> 11) & 7;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (Q, U, size, opcode) {
                        (_, _, x2, x3) if x2 & 2 == 0 && x3 == 3 => None,
                        (_, _, x2, x3) if x2 == 3 && x3 == 3 => None,
                        (_, x1, _, x3) if x1 == 0 && x3 == 0 => None,
                        (_, x1, _, x3) if x1 == 0 && x3 == 1 => None,
                        (_, x1, _, x3) if x1 == 0 && x3 == 2 => Some(Op::UdotAsimdsame2D),
                        (_, x1, _, x3) if x1 == 0 && x3 & 8 == 8 => None,
                        (_, x1, x2, x3) if x1 == 0 && x2 == 2 && x3 == 3 => {
                            Some(Op::UsdotAsimdsame2D)
                        }
                        (_, x1, _, x3) if x1 == 1 && x3 == 0 => Some(Op::SqrdmlshAsimdsame2Only),
                        (_, x1, _, x3) if x1 == 1 && x3 == 1 => Some(Op::SqrdmlshAsimdsame2Only),
                        (_, x1, _, x3) if x1 == 1 && x3 == 2 => Some(Op::UdotAsimdsame2D),
                        (_, x1, _, x3) if x1 == 1 && x3 & 12 == 8 => Some(Op::FcmlaAsimdsame2C),
                        (_, x1, _, x3) if x1 == 1 && x3 & 13 == 12 => Some(Op::FcaddAsimdsame2C),
                        (_, x1, x2, x3) if x1 == 1 && x2 == 0 && x3 == 13 => None,
                        (_, x1, x2, x3) if x1 == 1 && x2 == 0 && x3 == 15 => None,
                        (_, x1, x2, x3) if x1 == 1 && x2 == 1 && x3 == 15 => {
                            Some(Op::BfdotAsimdsame2D)
                        }
                        (_, x1, x2, x3) if x1 == 1 && x2 & 2 == 2 && x3 == 13 => None,
                        (_, x1, x2, x3) if x1 == 1 && x2 == 2 && x3 == 3 => None,
                        (_, x1, x2, x3) if x1 == 1 && x2 == 2 && x3 == 15 => None,
                        (_, x1, x2, x3) if x1 == 1 && x2 == 3 && x3 == 15 => {
                            Some(Op::BfmlalAsimdsame2F)
                        }
                        (x0, _, _, x3) if x0 == 0 && x3 & 12 == 4 => None,
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 13 => None,
                        (x0, _, x2, x3) if x0 == 1 && x2 & 2 == 0 && x3 & 12 == 4 => None,
                        (x0, _, x2, x3) if x0 == 1 && x2 & 2 == 2 && x3 & 14 == 6 => None,
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 2 && x3 == 4 => {
                            Some(Op::UmmlaAsimdsame2G)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 2 && x3 == 5 => {
                            Some(Op::UmmlaAsimdsame2G)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 13 => {
                            Some(Op::BfmmlaAsimdsame2E)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 2 && x3 == 4 => {
                            Some(Op::UmmlaAsimdsame2G)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 2 && x3 == 5 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 9 == 0 && x2 & 2 == 0 && x3 & 7 == 4 && x4 & 387 == 2 =>
                {
                    let Q = (instr >> 30) & 1;
                    let U = (instr >> 29) & 1;
                    let size = (instr >> 22) & 3;
                    let opcode = (instr >> 12) & 9;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (U, size, opcode) {
                        (_, _, x2) if x2 & 30 == 16 => None,
                        (_, _, x2) if x2 == 21 => None,
                        (_, x1, x2) if x1 & 2 == 0 && x2 & 28 == 12 => None,
                        (_, x1, x2) if x1 & 2 == 2 && x2 == 23 => None,
                        (_, x1, x2) if x1 & 2 == 2 && x2 == 30 => None,
                        (_, x1, x2) if x1 == 3 && x2 == 22 => None,
                        (x0, _, x2) if x0 == 0 && x2 == 0 => Some(Op::Rev32AsimdmiscR),
                        (x0, _, x2) if x0 == 0 && x2 == 1 => Some(Op::Rev32AsimdmiscR),
                        (x0, _, x2) if x0 == 0 && x2 == 2 => Some(Op::UadalpAsimdmiscP),
                        (x0, _, x2) if x0 == 0 && x2 == 3 => Some(Op::UsqaddAsimdmiscR),
                        (x0, _, x2) if x0 == 0 && x2 == 4 => Some(Op::ClzAsimdmiscR),
                        (x0, _, x2) if x0 == 0 && x2 == 5 => Some(Op::CntAsimdmiscR),
                        (x0, _, x2) if x0 == 0 && x2 == 6 => Some(Op::UadalpAsimdmiscP),
                        (x0, _, x2) if x0 == 0 && x2 == 7 => Some(Op::SqnegAsimdmiscR),
                        (x0, _, x2) if x0 == 0 && x2 == 8 => Some(Op::CmleAsimdmiscZ),
                        (x0, _, x2) if x0 == 0 && x2 == 9 => Some(Op::CmleAsimdmiscZ),
                        (x0, _, x2) if x0 == 0 && x2 == 10 => Some(Op::CmltAsimdmiscZ),
                        (x0, _, x2) if x0 == 0 && x2 == 11 => Some(Op::NegAsimdmiscR),
                        (x0, _, x2) if x0 == 0 && x2 == 18 => Some(Op::XtnAsimdmiscN),
                        (x0, _, x2) if x0 == 0 && x2 == 19 => None,
                        (x0, _, x2) if x0 == 0 && x2 == 20 => Some(Op::UqxtnAsimdmiscN),
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 22 => {
                            Some(Op::FcvtnAsimdmiscN)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 23 => {
                            Some(Op::FcvtlAsimdmiscL)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 24 => {
                            Some(Op::FrintiAsimdmiscR)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 25 => {
                            Some(Op::FrintiAsimdmiscR)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 26 => {
                            Some(Op::FcvtzuAsimdmiscR)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 27 => {
                            Some(Op::FcvtzuAsimdmiscR)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 28 => {
                            Some(Op::FcvtauAsimdmiscR)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 29 => {
                            Some(Op::UcvtfAsimdmiscR)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 30 => {
                            Some(Op::Frint64XAsimdmiscR)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 31 => {
                            Some(Op::Frint64XAsimdmiscR)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 12 => {
                            Some(Op::FcmleAsimdmiscFz)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 13 => {
                            Some(Op::FcmleAsimdmiscFz)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 14 => {
                            Some(Op::FcmltAsimdmiscFz)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 15 => {
                            Some(Op::FnegAsimdmiscR)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 24 => {
                            Some(Op::FrintiAsimdmiscR)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 25 => {
                            Some(Op::FrintiAsimdmiscR)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 26 => {
                            Some(Op::FcvtzuAsimdmiscR)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 27 => {
                            Some(Op::FcvtzuAsimdmiscR)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 28 => {
                            Some(Op::UrecpeAsimdmiscR)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 29 => {
                            Some(Op::FrecpeAsimdmiscR)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 31 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 22 => {
                            Some(Op::BfcvtnAsimdmisc4S)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 0 => Some(Op::Rev32AsimdmiscR),
                        (x0, _, x2) if x0 == 1 && x2 == 1 => None,
                        (x0, _, x2) if x0 == 1 && x2 == 2 => Some(Op::UadalpAsimdmiscP),
                        (x0, _, x2) if x0 == 1 && x2 == 3 => Some(Op::UsqaddAsimdmiscR),
                        (x0, _, x2) if x0 == 1 && x2 == 4 => Some(Op::ClzAsimdmiscR),
                        (x0, _, x2) if x0 == 1 && x2 == 6 => Some(Op::UadalpAsimdmiscP),
                        (x0, _, x2) if x0 == 1 && x2 == 7 => Some(Op::SqnegAsimdmiscR),
                        (x0, _, x2) if x0 == 1 && x2 == 8 => Some(Op::CmleAsimdmiscZ),
                        (x0, _, x2) if x0 == 1 && x2 == 9 => Some(Op::CmleAsimdmiscZ),
                        (x0, _, x2) if x0 == 1 && x2 == 10 => None,
                        (x0, _, x2) if x0 == 1 && x2 == 11 => Some(Op::NegAsimdmiscR),
                        (x0, _, x2) if x0 == 1 && x2 == 18 => Some(Op::SqxtunAsimdmiscN),
                        (x0, _, x2) if x0 == 1 && x2 == 19 => Some(Op::ShllAsimdmiscS),
                        (x0, _, x2) if x0 == 1 && x2 == 20 => Some(Op::UqxtnAsimdmiscN),
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 22 => {
                            Some(Op::FcvtxnAsimdmiscN)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 23 => None,
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 24 => {
                            Some(Op::FrintiAsimdmiscR)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 25 => {
                            Some(Op::FrintiAsimdmiscR)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 26 => {
                            Some(Op::FcvtzuAsimdmiscR)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 27 => {
                            Some(Op::FcvtzuAsimdmiscR)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 28 => {
                            Some(Op::FcvtauAsimdmiscR)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 29 => {
                            Some(Op::UcvtfAsimdmiscR)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 30 => {
                            Some(Op::Frint64XAsimdmiscR)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 31 => {
                            Some(Op::Frint64XAsimdmiscR)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 5 => Some(Op::NotAsimdmiscR),
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 5 => Some(Op::RbitAsimdmiscR),
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 5 => None,
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 12 => {
                            Some(Op::FcmleAsimdmiscFz)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 13 => {
                            Some(Op::FcmleAsimdmiscFz)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 14 => None,
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 15 => {
                            Some(Op::FnegAsimdmiscR)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 24 => None,
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 25 => {
                            Some(Op::FrintiAsimdmiscR)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 26 => {
                            Some(Op::FcvtzuAsimdmiscR)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 27 => {
                            Some(Op::FcvtzuAsimdmiscR)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 28 => {
                            Some(Op::UrsqrteAsimdmiscR)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 29 => {
                            Some(Op::FrsqrteAsimdmiscR)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 31 => {
                            Some(Op::FsqrtAsimdmiscR)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 == 22 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 9 == 0 && x2 & 2 == 0 && x3 & 7 == 6 && x4 & 387 == 2 =>
                {
                    let Q = (instr >> 30) & 1;
                    let U = (instr >> 29) & 1;
                    let size = (instr >> 22) & 3;
                    let opcode = (instr >> 12) & 9;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (U, size, opcode) {
                        (_, _, x2) if x2 & 30 == 0 => None,
                        (_, _, x2) if x2 == 2 => None,
                        (_, _, x2) if x2 & 28 == 4 => None,
                        (_, _, x2) if x2 & 30 == 8 => None,
                        (_, _, x2) if x2 == 11 => None,
                        (_, _, x2) if x2 == 13 => None,
                        (_, _, x2) if x2 == 14 => None,
                        (_, _, x2) if x2 & 24 == 16 => None,
                        (_, _, x2) if x2 & 30 == 24 => None,
                        (_, _, x2) if x2 & 28 == 28 => None,
                        (x0, _, x2) if x0 == 0 && x2 == 3 => Some(Op::UaddlvAsimdallOnly),
                        (x0, _, x2) if x0 == 0 && x2 == 10 => Some(Op::UminvAsimdallOnly),
                        (x0, _, x2) if x0 == 0 && x2 == 26 => Some(Op::UminvAsimdallOnly),
                        (x0, _, x2) if x0 == 0 && x2 == 27 => Some(Op::AddvAsimdallOnly),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 12 => {
                            Some(Op::FminnmvAsimdallOnlyH)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 15 => {
                            Some(Op::FminvAsimdallOnlyH)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 12 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 15 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 12 => {
                            Some(Op::FminnmvAsimdallOnlyH)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 15 => {
                            Some(Op::FminvAsimdallOnlyH)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 12 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 15 => None,
                        (x0, _, x2) if x0 == 1 && x2 == 3 => Some(Op::UaddlvAsimdallOnly),
                        (x0, _, x2) if x0 == 1 && x2 == 10 => Some(Op::UminvAsimdallOnly),
                        (x0, _, x2) if x0 == 1 && x2 == 26 => Some(Op::UminvAsimdallOnly),
                        (x0, _, x2) if x0 == 1 && x2 == 27 => None,
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 12 => {
                            Some(Op::FminnmvAsimdallOnlySd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 15 => {
                            Some(Op::FminvAsimdallOnlySd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 12 => {
                            Some(Op::FminnmvAsimdallOnlySd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 15 => {
                            Some(Op::FminvAsimdallOnlySd)
                        }
                        _ => None,
                    }
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 9 == 0 && x2 & 2 == 0 && x3 & 4 == 4 && x4 & 259 == 258 =>
                {
                    None
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 9 == 0 && x2 & 2 == 0 && x3 & 4 == 4 && x4 & 131 == 130 =>
                {
                    None
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 9 == 0 && x2 & 2 == 0 && x3 & 4 == 4 && x4 & 3 == 0 =>
                {
                    let Q = (instr >> 30) & 1;
                    let U = (instr >> 29) & 1;
                    let size = (instr >> 22) & 3;
                    let Rm = (instr >> 16) & 9;
                    let opcode = (instr >> 12) & 7;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (U, opcode) {
                        (_, x1) if x1 == 15 => None,
                        (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::UsublAsimddiffL),
                        (x0, x1) if x0 == 0 && x1 == 1 => Some(Op::UsubwAsimddiffW),
                        (x0, x1) if x0 == 0 && x1 == 2 => Some(Op::UsublAsimddiffL),
                        (x0, x1) if x0 == 0 && x1 == 3 => Some(Op::UsubwAsimddiffW),
                        (x0, x1) if x0 == 0 && x1 == 4 => Some(Op::RsubhnAsimddiffN),
                        (x0, x1) if x0 == 0 && x1 == 5 => Some(Op::UabdlAsimddiffL),
                        (x0, x1) if x0 == 0 && x1 == 6 => Some(Op::RsubhnAsimddiffN),
                        (x0, x1) if x0 == 0 && x1 == 7 => Some(Op::UabdlAsimddiffL),
                        (x0, x1) if x0 == 0 && x1 == 8 => Some(Op::UmlslAsimddiffL),
                        (x0, x1) if x0 == 0 && x1 == 9 => Some(Op::SqdmlslAsimddiffL),
                        (x0, x1) if x0 == 0 && x1 == 10 => Some(Op::UmlslAsimddiffL),
                        (x0, x1) if x0 == 0 && x1 == 11 => Some(Op::SqdmlslAsimddiffL),
                        (x0, x1) if x0 == 0 && x1 == 12 => Some(Op::UmullAsimddiffL),
                        (x0, x1) if x0 == 0 && x1 == 13 => Some(Op::SqdmullAsimddiffL),
                        (x0, x1) if x0 == 0 && x1 == 14 => Some(Op::PmullAsimddiffL),
                        (x0, x1) if x0 == 1 && x1 == 0 => Some(Op::UsublAsimddiffL),
                        (x0, x1) if x0 == 1 && x1 == 1 => Some(Op::UsubwAsimddiffW),
                        (x0, x1) if x0 == 1 && x1 == 2 => Some(Op::UsublAsimddiffL),
                        (x0, x1) if x0 == 1 && x1 == 3 => Some(Op::UsubwAsimddiffW),
                        (x0, x1) if x0 == 1 && x1 == 4 => Some(Op::RsubhnAsimddiffN),
                        (x0, x1) if x0 == 1 && x1 == 5 => Some(Op::UabdlAsimddiffL),
                        (x0, x1) if x0 == 1 && x1 == 6 => Some(Op::RsubhnAsimddiffN),
                        (x0, x1) if x0 == 1 && x1 == 7 => Some(Op::UabdlAsimddiffL),
                        (x0, x1) if x0 == 1 && x1 == 8 => Some(Op::UmlslAsimddiffL),
                        (x0, x1) if x0 == 1 && x1 == 9 => None,
                        (x0, x1) if x0 == 1 && x1 == 10 => Some(Op::UmlslAsimddiffL),
                        (x0, x1) if x0 == 1 && x1 == 11 => None,
                        (x0, x1) if x0 == 1 && x1 == 12 => Some(Op::UmullAsimddiffL),
                        (x0, x1) if x0 == 1 && x1 == 13 => None,
                        (x0, x1) if x0 == 1 && x1 == 14 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 9 == 0 && x2 & 2 == 0 && x3 & 4 == 4 && x4 & 1 == 1 =>
                {
                    let Q = (instr >> 30) & 1;
                    let U = (instr >> 29) & 1;
                    let size = (instr >> 22) & 3;
                    let Rm = (instr >> 16) & 9;
                    let opcode = (instr >> 11) & 9;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (U, size, opcode) {
                        (x0, _, x2) if x0 == 0 && x2 == 0 => Some(Op::UhaddAsimdsameOnly),
                        (x0, _, x2) if x0 == 0 && x2 == 1 => Some(Op::UqaddAsimdsameOnly),
                        (x0, _, x2) if x0 == 0 && x2 == 2 => Some(Op::UrhaddAsimdsameOnly),
                        (x0, _, x2) if x0 == 0 && x2 == 4 => Some(Op::UhsubAsimdsameOnly),
                        (x0, _, x2) if x0 == 0 && x2 == 5 => Some(Op::UqsubAsimdsameOnly),
                        (x0, _, x2) if x0 == 0 && x2 == 6 => Some(Op::CmhsAsimdsameOnly),
                        (x0, _, x2) if x0 == 0 && x2 == 7 => Some(Op::CmhsAsimdsameOnly),
                        (x0, _, x2) if x0 == 0 && x2 == 8 => Some(Op::UqrshlAsimdsameOnly),
                        (x0, _, x2) if x0 == 0 && x2 == 9 => Some(Op::UqrshlAsimdsameOnly),
                        (x0, _, x2) if x0 == 0 && x2 == 10 => Some(Op::UqrshlAsimdsameOnly),
                        (x0, _, x2) if x0 == 0 && x2 == 11 => Some(Op::UqrshlAsimdsameOnly),
                        (x0, _, x2) if x0 == 0 && x2 == 12 => Some(Op::UminAsimdsameOnly),
                        (x0, _, x2) if x0 == 0 && x2 == 13 => Some(Op::UminAsimdsameOnly),
                        (x0, _, x2) if x0 == 0 && x2 == 14 => Some(Op::UabaAsimdsameOnly),
                        (x0, _, x2) if x0 == 0 && x2 == 15 => Some(Op::UabaAsimdsameOnly),
                        (x0, _, x2) if x0 == 0 && x2 == 16 => Some(Op::SubAsimdsameOnly),
                        (x0, _, x2) if x0 == 0 && x2 == 17 => Some(Op::CmeqAsimdsameOnly),
                        (x0, _, x2) if x0 == 0 && x2 == 18 => Some(Op::MlsAsimdsameOnly),
                        (x0, _, x2) if x0 == 0 && x2 == 19 => Some(Op::PmulAsimdsameOnly),
                        (x0, _, x2) if x0 == 0 && x2 == 20 => Some(Op::UminpAsimdsameOnly),
                        (x0, _, x2) if x0 == 0 && x2 == 21 => Some(Op::UminpAsimdsameOnly),
                        (x0, _, x2) if x0 == 0 && x2 == 22 => Some(Op::SqrdmulhAsimdsameOnly),
                        (x0, _, x2) if x0 == 0 && x2 == 23 => Some(Op::AddpAsimdsameOnly),
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 24 => {
                            Some(Op::FminnmpAsimdsameOnly)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 25 => {
                            Some(Op::FmlsAsimdsameOnly)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 26 => {
                            Some(Op::FaddpAsimdsameOnly)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 27 => {
                            Some(Op::FmulxAsimdsameOnly)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 28 => {
                            Some(Op::FacgtAsimdsameOnly)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 30 => {
                            Some(Op::FminpAsimdsameOnly)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 31 => {
                            Some(Op::FrecpsAsimdsameOnly)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 3 => Some(Op::OrnAsimdsameOnly),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 29 => Some(Op::FmlslAsimdsameF),
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 3 => Some(Op::OrnAsimdsameOnly),
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 29 => None,
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 24 => {
                            Some(Op::FminnmpAsimdsameOnly)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 25 => {
                            Some(Op::FmlsAsimdsameOnly)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 26 => {
                            Some(Op::FabdAsimdsameOnly)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 27 => None,
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 28 => None,
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 30 => {
                            Some(Op::FminpAsimdsameOnly)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 31 => {
                            Some(Op::FrsqrtsAsimdsameOnly)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 3 => Some(Op::OrnAsimdsameOnly),
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 29 => Some(Op::FmlslAsimdsameF),
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 3 => Some(Op::OrnAsimdsameOnly),
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 29 => None,
                        (x0, _, x2) if x0 == 1 && x2 == 0 => Some(Op::UhaddAsimdsameOnly),
                        (x0, _, x2) if x0 == 1 && x2 == 1 => Some(Op::UqaddAsimdsameOnly),
                        (x0, _, x2) if x0 == 1 && x2 == 2 => Some(Op::UrhaddAsimdsameOnly),
                        (x0, _, x2) if x0 == 1 && x2 == 4 => Some(Op::UhsubAsimdsameOnly),
                        (x0, _, x2) if x0 == 1 && x2 == 5 => Some(Op::UqsubAsimdsameOnly),
                        (x0, _, x2) if x0 == 1 && x2 == 6 => Some(Op::CmhsAsimdsameOnly),
                        (x0, _, x2) if x0 == 1 && x2 == 7 => Some(Op::CmhsAsimdsameOnly),
                        (x0, _, x2) if x0 == 1 && x2 == 8 => Some(Op::UqrshlAsimdsameOnly),
                        (x0, _, x2) if x0 == 1 && x2 == 9 => Some(Op::UqrshlAsimdsameOnly),
                        (x0, _, x2) if x0 == 1 && x2 == 10 => Some(Op::UqrshlAsimdsameOnly),
                        (x0, _, x2) if x0 == 1 && x2 == 11 => Some(Op::UqrshlAsimdsameOnly),
                        (x0, _, x2) if x0 == 1 && x2 == 12 => Some(Op::UminAsimdsameOnly),
                        (x0, _, x2) if x0 == 1 && x2 == 13 => Some(Op::UminAsimdsameOnly),
                        (x0, _, x2) if x0 == 1 && x2 == 14 => Some(Op::UabaAsimdsameOnly),
                        (x0, _, x2) if x0 == 1 && x2 == 15 => Some(Op::UabaAsimdsameOnly),
                        (x0, _, x2) if x0 == 1 && x2 == 16 => Some(Op::SubAsimdsameOnly),
                        (x0, _, x2) if x0 == 1 && x2 == 17 => Some(Op::CmeqAsimdsameOnly),
                        (x0, _, x2) if x0 == 1 && x2 == 18 => Some(Op::MlsAsimdsameOnly),
                        (x0, _, x2) if x0 == 1 && x2 == 19 => Some(Op::PmulAsimdsameOnly),
                        (x0, _, x2) if x0 == 1 && x2 == 20 => Some(Op::UminpAsimdsameOnly),
                        (x0, _, x2) if x0 == 1 && x2 == 21 => Some(Op::UminpAsimdsameOnly),
                        (x0, _, x2) if x0 == 1 && x2 == 22 => Some(Op::SqrdmulhAsimdsameOnly),
                        (x0, _, x2) if x0 == 1 && x2 == 23 => None,
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 24 => {
                            Some(Op::FminnmpAsimdsameOnly)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 26 => {
                            Some(Op::FaddpAsimdsameOnly)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 27 => {
                            Some(Op::FmulAsimdsameOnly)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 28 => {
                            Some(Op::FacgtAsimdsameOnly)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 29 => {
                            Some(Op::FacgtAsimdsameOnly)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 30 => {
                            Some(Op::FminpAsimdsameOnly)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 31 => {
                            Some(Op::FdivAsimdsameOnly)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 3 => Some(Op::BifAsimdsameOnly),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 25 => {
                            Some(Op::Fmlsl2AsimdsameF)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 3 => Some(Op::BifAsimdsameOnly),
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 25 => None,
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 24 => {
                            Some(Op::FminnmpAsimdsameOnly)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 26 => {
                            Some(Op::FabdAsimdsameOnly)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 27 => None,
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 28 => {
                            Some(Op::FacgtAsimdsameOnly)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 29 => {
                            Some(Op::FacgtAsimdsameOnly)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 30 => {
                            Some(Op::FminpAsimdsameOnly)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 31 => None,
                        (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 == 3 => Some(Op::BifAsimdsameOnly),
                        (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 == 25 => {
                            Some(Op::Fmlsl2AsimdsameF)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 3 => Some(Op::BifAsimdsameOnly),
                        (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 25 => None,
                        _ => None,
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
                        (_, x1, x2, x3) if x1 == 0 && x2 & 8 == 0 && x3 == 1 => None,
                        (_, x1, x2, x3) if x1 == 0 && x2 & 9 == 0 && x3 == 0 => {
                            Some(Op::FmovAsimdimmD2D)
                        }
                        (_, x1, x2, x3) if x1 == 0 && x2 & 9 == 1 && x3 == 0 => {
                            Some(Op::FmovAsimdimmD2D)
                        }
                        (_, x1, x2, x3) if x1 == 0 && x2 & 12 == 8 && x3 == 1 => None,
                        (_, x1, x2, x3) if x1 == 0 && x2 & 13 == 8 && x3 == 0 => {
                            Some(Op::FmovAsimdimmD2D)
                        }
                        (_, x1, x2, x3) if x1 == 0 && x2 & 13 == 9 && x3 == 0 => {
                            Some(Op::FmovAsimdimmD2D)
                        }
                        (_, x1, x2, x3) if x1 == 0 && x2 & 14 == 12 && x3 == 0 => {
                            Some(Op::FmovAsimdimmD2D)
                        }
                        (_, x1, x2, x3) if x1 == 0 && x2 & 14 == 12 && x3 == 1 => None,
                        (_, x1, x2, x3) if x1 == 0 && x2 == 14 && x3 == 0 => {
                            Some(Op::FmovAsimdimmD2D)
                        }
                        (_, x1, x2, x3) if x1 == 0 && x2 == 14 && x3 == 1 => None,
                        (_, x1, x2, x3) if x1 == 0 && x2 == 15 && x3 == 0 => {
                            Some(Op::FmovAsimdimmD2D)
                        }
                        (_, x1, x2, x3) if x1 == 0 && x2 == 15 && x3 == 1 => {
                            Some(Op::FmovAsimdimmHH)
                        }
                        (_, x1, _, x3) if x1 == 1 && x3 == 1 => None,
                        (_, x1, x2, x3) if x1 == 1 && x2 & 9 == 0 && x3 == 0 => {
                            Some(Op::FmovAsimdimmD2D)
                        }
                        (_, x1, x2, x3) if x1 == 1 && x2 & 9 == 1 && x3 == 0 => {
                            Some(Op::FmovAsimdimmD2D)
                        }
                        (_, x1, x2, x3) if x1 == 1 && x2 & 13 == 8 && x3 == 0 => {
                            Some(Op::FmovAsimdimmD2D)
                        }
                        (_, x1, x2, x3) if x1 == 1 && x2 & 13 == 9 && x3 == 0 => {
                            Some(Op::FmovAsimdimmD2D)
                        }
                        (_, x1, x2, x3) if x1 == 1 && x2 & 14 == 12 && x3 == 0 => {
                            Some(Op::FmovAsimdimmD2D)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 14 && x3 == 0 => {
                            Some(Op::FmovAsimdimmD2D)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 15 && x3 == 0 => None,
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 14 && x3 == 0 => {
                            Some(Op::FmovAsimdimmD2D)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 15 && x3 == 0 => {
                            Some(Op::FmovAsimdimmD2D)
                        }
                        _ => None,
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
                        (_, x1) if x1 == 1 => None,
                        (_, x1) if x1 == 3 => None,
                        (_, x1) if x1 == 5 => None,
                        (_, x1) if x1 == 7 => None,
                        (_, x1) if x1 == 9 => None,
                        (_, x1) if x1 == 11 => None,
                        (_, x1) if x1 == 13 => None,
                        (_, x1) if x1 == 15 => None,
                        (_, x1) if x1 == 21 => None,
                        (_, x1) if x1 & 30 == 22 => None,
                        (_, x1) if x1 & 28 == 24 => None,
                        (_, x1) if x1 == 29 => None,
                        (_, x1) if x1 == 30 => None,
                        (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::UrsraAsimdshfR),
                        (x0, x1) if x0 == 0 && x1 == 2 => Some(Op::UrsraAsimdshfR),
                        (x0, x1) if x0 == 0 && x1 == 4 => Some(Op::UrsraAsimdshfR),
                        (x0, x1) if x0 == 0 && x1 == 6 => Some(Op::UrsraAsimdshfR),
                        (x0, x1) if x0 == 0 && x1 == 8 => None,
                        (x0, x1) if x0 == 0 && x1 == 10 => Some(Op::ShlAsimdshfR),
                        (x0, x1) if x0 == 0 && x1 == 12 => None,
                        (x0, x1) if x0 == 0 && x1 == 14 => Some(Op::UqshlAsimdshfR),
                        (x0, x1) if x0 == 0 && x1 == 16 => Some(Op::RshrnAsimdshfN),
                        (x0, x1) if x0 == 0 && x1 == 17 => Some(Op::RshrnAsimdshfN),
                        (x0, x1) if x0 == 0 && x1 == 18 => Some(Op::UqrshrnAsimdshfN),
                        (x0, x1) if x0 == 0 && x1 == 19 => Some(Op::UqrshrnAsimdshfN),
                        (x0, x1) if x0 == 0 && x1 == 20 => Some(Op::UshllAsimdshfL),
                        (x0, x1) if x0 == 0 && x1 == 28 => Some(Op::UcvtfAsimdshfC),
                        (x0, x1) if x0 == 0 && x1 == 31 => Some(Op::FcvtzuAsimdshfC),
                        (x0, x1) if x0 == 1 && x1 == 0 => Some(Op::UrsraAsimdshfR),
                        (x0, x1) if x0 == 1 && x1 == 2 => Some(Op::UrsraAsimdshfR),
                        (x0, x1) if x0 == 1 && x1 == 4 => Some(Op::UrsraAsimdshfR),
                        (x0, x1) if x0 == 1 && x1 == 6 => Some(Op::UrsraAsimdshfR),
                        (x0, x1) if x0 == 1 && x1 == 8 => Some(Op::SriAsimdshfR),
                        (x0, x1) if x0 == 1 && x1 == 10 => Some(Op::SliAsimdshfR),
                        (x0, x1) if x0 == 1 && x1 == 12 => Some(Op::UqshlAsimdshfR),
                        (x0, x1) if x0 == 1 && x1 == 14 => Some(Op::UqshlAsimdshfR),
                        (x0, x1) if x0 == 1 && x1 == 16 => Some(Op::SqrshrunAsimdshfN),
                        (x0, x1) if x0 == 1 && x1 == 17 => Some(Op::SqrshrunAsimdshfN),
                        (x0, x1) if x0 == 1 && x1 == 18 => Some(Op::UqrshrnAsimdshfN),
                        (x0, x1) if x0 == 1 && x1 == 19 => Some(Op::UqrshrnAsimdshfN),
                        (x0, x1) if x0 == 1 && x1 == 20 => Some(Op::UshllAsimdshfL),
                        (x0, x1) if x0 == 1 && x1 == 28 => Some(Op::UcvtfAsimdshfC),
                        (x0, x1) if x0 == 1 && x1 == 31 => Some(Op::FcvtzuAsimdshfC),
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _) if x0 & 9 == 0 && x2 == 3 && x4 & 1 == 1 => None,
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
                        (_, x1, x2) if x1 == 1 && x2 == 9 => None,
                        (x0, _, x2) if x0 == 0 && x2 == 2 => Some(Op::UmlslAsimdelemL),
                        (x0, _, x2) if x0 == 0 && x2 == 3 => Some(Op::SqdmlslAsimdelemL),
                        (x0, _, x2) if x0 == 0 && x2 == 6 => Some(Op::UmlslAsimdelemL),
                        (x0, _, x2) if x0 == 0 && x2 == 7 => Some(Op::SqdmlslAsimdelemL),
                        (x0, _, x2) if x0 == 0 && x2 == 8 => Some(Op::MulAsimdelemR),
                        (x0, _, x2) if x0 == 0 && x2 == 10 => Some(Op::UmullAsimdelemL),
                        (x0, _, x2) if x0 == 0 && x2 == 11 => Some(Op::SqdmullAsimdelemL),
                        (x0, _, x2) if x0 == 0 && x2 == 12 => Some(Op::SqrdmulhAsimdelemR),
                        (x0, _, x2) if x0 == 0 && x2 == 13 => Some(Op::SqrdmulhAsimdelemR),
                        (x0, _, x2) if x0 == 0 && x2 == 14 => Some(Op::UdotAsimdelemD),
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 0 => None,
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 0 && x2 == 4 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => Some(Op::FmlsAsimdelemRhH),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 5 => Some(Op::FmlsAsimdelemRhH),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 9 => {
                            Some(Op::FmulxAsimdelemRhH)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 15 => Some(Op::UsdotAsimdelemD),
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 1 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 5 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 1 && x2 == 15 => Some(Op::BfdotAsimdelemE),
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 1 => {
                            Some(Op::FmlsAsimdelemRSd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 5 => {
                            Some(Op::FmlsAsimdelemRSd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 & 2 == 2 && x2 == 9 => {
                            Some(Op::FmulxAsimdelemRSd)
                        }
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 0 => Some(Op::FmlslAsimdelemLh),
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 4 => Some(Op::FmlslAsimdelemLh),
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 15 => Some(Op::UsdotAsimdelemD),
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 0 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 4 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 15 => {
                            Some(Op::BfmlalAsimdelemF)
                        }
                        (x0, _, x2) if x0 == 1 && x2 == 0 => Some(Op::MlsAsimdelemR),
                        (x0, _, x2) if x0 == 1 && x2 == 2 => Some(Op::UmlslAsimdelemL),
                        (x0, _, x2) if x0 == 1 && x2 == 4 => Some(Op::MlsAsimdelemR),
                        (x0, _, x2) if x0 == 1 && x2 == 6 => Some(Op::UmlslAsimdelemL),
                        (x0, _, x2) if x0 == 1 && x2 == 10 => Some(Op::UmullAsimdelemL),
                        (x0, _, x2) if x0 == 1 && x2 == 11 => None,
                        (x0, _, x2) if x0 == 1 && x2 == 13 => Some(Op::SqrdmlshAsimdelemR),
                        (x0, _, x2) if x0 == 1 && x2 == 14 => Some(Op::UdotAsimdelemD),
                        (x0, _, x2) if x0 == 1 && x2 == 15 => Some(Op::SqrdmlshAsimdelemR),
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 8 => None,
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 0 && x2 == 12 => None,
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => None,
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 3 => None,
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 5 => None,
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 7 => None,
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 9 => {
                            Some(Op::FmulxAsimdelemRhH)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 & 9 == 1 => {
                            Some(Op::FcmlaAsimdelemCS)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 & 2 == 2 && x2 == 9 => {
                            Some(Op::FmulxAsimdelemRSd)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 & 9 == 1 => {
                            Some(Op::FcmlaAsimdelemCS)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 == 8 => {
                            Some(Op::Fmlsl2AsimdelemLh)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 == 12 => {
                            Some(Op::Fmlsl2AsimdelemLh)
                        }
                        (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 1 => None,
                        (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 3 => None,
                        (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 5 => None,
                        (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 7 => None,
                        (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 8 => None,
                        (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 12 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 == 12 && x2 == 0 && x3 & 12 == 8 && x4 & 48 == 32 => {
                    let Rm = (instr >> 16) & 9;
                    let imm2 = (instr >> 12) & 3;
                    let opcode = (instr >> 10) & 3;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match opcode {
                        x0 if x0 == 0 => Some(Op::Sm3Tt1AVvv4Crypto3Imm2),
                        x0 if x0 == 1 => Some(Op::Sm3Tt1BVvv4Crypto3Imm2),
                        x0 if x0 == 2 => Some(Op::Sm3Tt2AVvv4Crypto3Imm2),
                        x0 if x0 == 3 => Some(Op::Sm3Tt2BVvvCrypto3Imm2),
                        _ => None,
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 == 12 && x2 == 0 && x3 & 12 == 12 && x4 & 44 == 32 => {
                    let Rm = (instr >> 16) & 9;
                    let O = (instr >> 14) & 1;
                    let opcode = (instr >> 10) & 3;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (O, opcode) {
                        (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::Sha512HQqvCryptosha5123),
                        (x0, x1) if x0 == 0 && x1 == 1 => Some(Op::Sha512H2QqvCryptosha5123),
                        (x0, x1) if x0 == 0 && x1 == 2 => Some(Op::Sha512Su1Vvv2Cryptosha5123),
                        (x0, x1) if x0 == 0 && x1 == 3 => Some(Op::Rax1Vvv2Cryptosha5123),
                        (x0, x1) if x0 == 1 && x1 == 0 => Some(Op::Sm3Partw1Vvv4Cryptosha5123),
                        (x0, x1) if x0 == 1 && x1 == 1 => Some(Op::Sm3Partw2Vvv4Cryptosha5123),
                        (x0, x1) if x0 == 1 && x1 == 2 => Some(Op::Sm4EkeyVvv4Cryptosha5123),
                        (x0, x1) if x0 == 1 && x1 == 3 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _) if x0 == 12 && x2 == 0 && x4 & 32 == 0 => {
                    let Op0 = (instr >> 21) & 3;
                    let Rm = (instr >> 16) & 9;
                    let Ra = (instr >> 10) & 9;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match Op0 {
                        x0 if x0 == 0 => Some(Op::Eor3Vvv16Crypto4),
                        x0 if x0 == 1 => Some(Op::BcaxVvv16Crypto4),
                        x0 if x0 == 2 => Some(Op::Sm3Ss1Vvv4Crypto4),
                        x0 if x0 == 3 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, x3, _, _) if x0 == 12 && x2 == 1 && x3 & 12 == 0 => {
                    let Rm = (instr >> 16) & 9;
                    let imm6 = (instr >> 10) & 11;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match () {
                        () => Some(Op::XarVvv2Crypto3Imm6),
                    }
                }
                (x0, _, x2, x3, x4, _) if x0 == 12 && x2 == 1 && x3 == 8 && x4 & 508 == 32 => {
                    let opcode = (instr >> 10) & 3;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match opcode {
                        x0 if x0 == 0 => Some(Op::Sha512Su0Vv2Cryptosha5122),
                        x0 if x0 == 1 => Some(Op::Sm4EVv4Cryptosha5122),
                        x0 if x0 & 2 == 2 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, _, _, _) if x0 & 9 == 8 && x2 & 2 == 2 => None,
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
                        (_, _, _, _, x4, _) if x4 & 4 == 4 => None,
                        (_, _, _, x3, x4, _) if x3 & 1 == 0 && x4 & 6 == 0 => None,
                        (_, _, _, x3, x4, _) if x3 & 1 == 1 && x4 & 6 == 2 => None,
                        (_, _, _, x3, x4, _) if x3 & 2 == 0 && x4 & 6 == 0 => None,
                        (_, _, _, x3, x4, _) if x3 & 2 == 2 && x4 & 6 == 2 => None,
                        (_, _, x2, _, _, _) if x2 == 2 => None,
                        (_, x1, _, _, _, _) if x1 == 1 => None,
                        (x0, _, _, _, _, x5) if x0 == 0 && x5 & 32 == 0 => None,
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 2 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Fix)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 3 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Fix)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 3 && x4 == 0 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Fix)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 3 && x4 == 1 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Fix)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 2 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Fix)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 3 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Fix)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 3 && x4 == 0 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Fix)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 3 && x4 == 1 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Fix)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 2 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Fix)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 3 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Fix)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 3 && x4 == 0 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Fix)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 3 && x4 == 1 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Fix)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 2 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Fix)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 3 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Fix)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 3 && x4 == 0 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Fix)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 3 && x4 == 1 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Fix)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 2 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Fix)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 3 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Fix)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 3 && x4 == 0 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Fix)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 3 && x4 == 1 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Fix)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 2 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Fix)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 3 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Fix)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 3 && x4 == 0 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Fix)
                        }
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 3 && x4 == 1 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Fix)
                        }
                        _ => None,
                    }
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 5 == 1 && x2 & 2 == 0 && x3 & 4 == 4 && x4 & 63 == 0 =>
                {
                    let sf = (instr >> 31) & 1;
                    let S = (instr >> 29) & 1;
                    let ptype = (instr >> 22) & 3;
                    let rmode = (instr >> 19) & 3;
                    let opcode = (instr >> 16) & 5;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (sf, S, ptype, rmode, opcode) {
                        (_, _, _, x3, x4) if x3 & 1 == 1 && x4 & 6 == 2 => None,
                        (_, _, _, x3, x4) if x3 & 1 == 1 && x4 & 6 == 4 => None,
                        (_, _, _, x3, x4) if x3 & 2 == 2 && x4 & 6 == 2 => None,
                        (_, _, _, x3, x4) if x3 & 2 == 2 && x4 & 6 == 4 => None,
                        (_, x1, x2, _, x4) if x1 == 0 && x2 == 2 && x4 & 4 == 0 => None,
                        (_, x1, x2, _, x4) if x1 == 0 && x2 == 2 && x4 & 6 == 4 => None,
                        (_, x1, _, _, _) if x1 == 1 => None,
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 & 1 == 1 && x4 & 6 == 6 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 1 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 2 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 3 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 4 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 5 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 6 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 7 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 1 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 & 2 == 2 && x4 & 6 == 6 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 2 && x4 == 0 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 2 && x4 == 1 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 3 && x4 == 0 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 3 && x4 == 1 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 & 2 == 0 && x4 & 6 == 6 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 2 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 3 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 4 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 5 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 2 && x4 == 0 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 2 && x4 == 1 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 2 && x4 & 6 == 6 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 3 && x4 == 0 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 3 && x4 == 1 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 3 && x4 == 6 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 3 && x4 == 7 =>
                        {
                            None
                        }
                        (x0, x1, x2, _, x4) if x0 == 0 && x1 == 0 && x2 == 2 && x4 & 6 == 6 => None,
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 0 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 1 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 2 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 3 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 4 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 5 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 6 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 7 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 1 && x4 == 0 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 1 && x4 == 1 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 2 && x4 == 0 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 2 && x4 == 1 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 3 && x4 == 0 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 3 && x4 == 1 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, _, x4) if x0 == 1 && x1 == 0 && x2 == 0 && x4 & 6 == 6 => None,
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 1 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 2 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 3 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 4 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 5 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 1 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 2 && x4 == 0 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 2 && x4 == 1 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 3 && x4 == 0 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 3 && x4 == 1 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 & 1 == 1 && x4 & 6 == 6 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 2 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 3 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 4 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 5 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 6 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 7 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 & 2 == 2 && x4 & 6 == 6 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 2 && x4 == 0 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 2 && x4 == 1 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 3 && x4 == 0 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 3 && x4 == 1 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 2 && x3 & 1 == 0 && x4 & 6 == 6 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 2 && x3 == 1 && x4 == 6 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 2 && x3 == 1 && x4 == 7 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 2 && x3 & 2 == 2 && x4 & 6 == 6 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 0 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 1 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 2 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 3 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 4 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 5 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 6 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 7 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 1 && x4 == 0 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 1 && x4 == 1 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 2 && x4 == 0 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 2 && x4 == 1 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 3 && x4 == 0 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 3 && x4 == 1 =>
                        {
                            Some(Op::Fcvtzu64HFloat2Int)
                        }
                        _ => None,
                    }
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 5 == 1 && x2 & 2 == 0 && x3 & 4 == 4 && x4 & 31 == 16 =>
                {
                    let M = (instr >> 31) & 1;
                    let S = (instr >> 29) & 1;
                    let ptype = (instr >> 22) & 3;
                    let opcode = (instr >> 15) & 11;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (M, S, ptype, opcode) {
                        (_, _, _, x3) if x3 & 32 == 32 => None,
                        (_, x1, _, _) if x1 == 1 => None,
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            Some(Op::FsqrtHFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            Some(Op::FsqrtHFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 2 => {
                            Some(Op::FsqrtHFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 3 => {
                            Some(Op::FsqrtHFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 4 => None,
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 5 => {
                            Some(Op::FcvtDhFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 6 => None,
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 7 => {
                            Some(Op::FcvtDhFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 8 => {
                            Some(Op::FrintiHFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 9 => {
                            Some(Op::FrintiHFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 10 => {
                            Some(Op::FrintiHFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 11 => {
                            Some(Op::FrintiHFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 12 => {
                            Some(Op::FrintiHFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 13 => None,
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 14 => {
                            Some(Op::FrintiHFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 15 => {
                            Some(Op::FrintiHFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 16 => {
                            Some(Op::Frint64XDFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 17 => {
                            Some(Op::Frint64XDFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 18 => {
                            Some(Op::Frint64XDFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 19 => {
                            Some(Op::Frint64XDFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 & 60 == 20 => None,
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 & 56 == 24 => None,
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            Some(Op::FsqrtHFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 => {
                            Some(Op::FsqrtHFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 2 => {
                            Some(Op::FsqrtHFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 3 => {
                            Some(Op::FsqrtHFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 4 => {
                            Some(Op::FcvtDhFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 5 => None,
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 6 => {
                            Some(Op::BfcvtBsFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 7 => {
                            Some(Op::FcvtDhFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 8 => {
                            Some(Op::FrintiHFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 9 => {
                            Some(Op::FrintiHFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 10 => {
                            Some(Op::FrintiHFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 11 => {
                            Some(Op::FrintiHFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 12 => {
                            Some(Op::FrintiHFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 13 => None,
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 14 => {
                            Some(Op::FrintiHFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 15 => {
                            Some(Op::FrintiHFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 16 => {
                            Some(Op::Frint64XDFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 17 => {
                            Some(Op::Frint64XDFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 18 => {
                            Some(Op::Frint64XDFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 19 => {
                            Some(Op::Frint64XDFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 & 60 == 20 => None,
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 & 56 == 24 => None,
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 2 && x3 & 32 == 0 => None,
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 => {
                            Some(Op::FsqrtHFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 1 => {
                            Some(Op::FsqrtHFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 2 => {
                            Some(Op::FsqrtHFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 3 => {
                            Some(Op::FsqrtHFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 4 => {
                            Some(Op::FcvtDhFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 5 => {
                            Some(Op::FcvtDhFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 & 62 == 6 => None,
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 8 => {
                            Some(Op::FrintiHFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 9 => {
                            Some(Op::FrintiHFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 10 => {
                            Some(Op::FrintiHFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 11 => {
                            Some(Op::FrintiHFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 12 => {
                            Some(Op::FrintiHFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 13 => None,
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 14 => {
                            Some(Op::FrintiHFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 15 => {
                            Some(Op::FrintiHFloatdp1)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 & 48 == 16 => None,
                        (x0, _, _, _) if x0 == 1 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 5 == 1 && x2 & 2 == 0 && x3 & 4 == 4 && x4 & 15 == 8 =>
                {
                    let M = (instr >> 31) & 1;
                    let S = (instr >> 29) & 1;
                    let ptype = (instr >> 22) & 3;
                    let Rm = (instr >> 16) & 9;
                    let op = (instr >> 14) & 3;
                    let Rn = (instr >> 5) & 9;
                    let opcode2 = instr & 9;
                    match (M, S, ptype, op, opcode2) {
                        (_, _, _, _, x4) if x4 & 1 == 1 => None,
                        (_, _, _, _, x4) if x4 & 2 == 2 => None,
                        (_, _, _, _, x4) if x4 & 4 == 4 => None,
                        (_, _, _, x3, _) if x3 & 1 == 1 => None,
                        (_, _, _, x3, _) if x3 & 2 == 2 => None,
                        (_, _, x2, _, _) if x2 == 2 => None,
                        (_, x1, _, _, _) if x1 == 1 => None,
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 =>
                        {
                            Some(Op::FcmpeHzFloatcmp)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 8 =>
                        {
                            Some(Op::FcmpeHzFloatcmp)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 16 =>
                        {
                            Some(Op::FcmpeHzFloatcmp)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 24 =>
                        {
                            Some(Op::FcmpeHzFloatcmp)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 =>
                        {
                            Some(Op::FcmpeHzFloatcmp)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 8 =>
                        {
                            Some(Op::FcmpeHzFloatcmp)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 16 =>
                        {
                            Some(Op::FcmpeHzFloatcmp)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 24 =>
                        {
                            Some(Op::FcmpeHzFloatcmp)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 0 =>
                        {
                            Some(Op::FcmpeHzFloatcmp)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 8 =>
                        {
                            Some(Op::FcmpeHzFloatcmp)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 16 =>
                        {
                            Some(Op::FcmpeHzFloatcmp)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 24 =>
                        {
                            Some(Op::FcmpeHzFloatcmp)
                        }
                        (x0, _, _, _, _) if x0 == 1 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 5 == 1 && x2 & 2 == 0 && x3 & 4 == 4 && x4 & 7 == 4 =>
                {
                    let M = (instr >> 31) & 1;
                    let S = (instr >> 29) & 1;
                    let ptype = (instr >> 22) & 3;
                    let imm8 = (instr >> 13) & 15;
                    let imm5 = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (M, S, ptype, imm5) {
                        (_, _, _, x3) if x3 & 1 == 1 => None,
                        (_, _, _, x3) if x3 & 2 == 2 => None,
                        (_, _, _, x3) if x3 & 4 == 4 => None,
                        (_, _, _, x3) if x3 & 8 == 8 => None,
                        (_, _, _, x3) if x3 & 16 == 16 => None,
                        (_, _, x2, _) if x2 == 2 => None,
                        (_, x1, _, _) if x1 == 1 => None,
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            Some(Op::FmovHFloatimm)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            Some(Op::FmovHFloatimm)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 => {
                            Some(Op::FmovHFloatimm)
                        }
                        (x0, _, _, _) if x0 == 1 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 5 == 1 && x2 & 2 == 0 && x3 & 4 == 4 && x4 & 3 == 1 =>
                {
                    let M = (instr >> 31) & 1;
                    let S = (instr >> 29) & 1;
                    let ptype = (instr >> 22) & 3;
                    let Rm = (instr >> 16) & 9;
                    let cond = (instr >> 12) & 7;
                    let Rn = (instr >> 5) & 9;
                    let op = (instr >> 4) & 1;
                    let nzcv = instr & 7;
                    match (M, S, ptype, op) {
                        (_, _, x2, _) if x2 == 2 => None,
                        (_, x1, _, _) if x1 == 1 => None,
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            Some(Op::FccmpeHFloatccmp)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            Some(Op::FccmpeHFloatccmp)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            Some(Op::FccmpeHFloatccmp)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 => {
                            Some(Op::FccmpeHFloatccmp)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 => {
                            Some(Op::FccmpeHFloatccmp)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 1 => {
                            Some(Op::FccmpeHFloatccmp)
                        }
                        (x0, _, _, _) if x0 == 1 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 5 == 1 && x2 & 2 == 0 && x3 & 4 == 4 && x4 & 3 == 2 =>
                {
                    let M = (instr >> 31) & 1;
                    let S = (instr >> 29) & 1;
                    let ptype = (instr >> 22) & 3;
                    let Rm = (instr >> 16) & 9;
                    let opcode = (instr >> 12) & 7;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (M, S, ptype, opcode) {
                        (_, _, _, x3) if x3 & 9 == 9 => None,
                        (_, _, _, x3) if x3 & 10 == 10 => None,
                        (_, _, _, x3) if x3 & 12 == 12 => None,
                        (_, _, x2, _) if x2 == 2 => None,
                        (_, x1, _, _) if x1 == 1 => None,
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            Some(Op::FnmulHFloatdp2)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            Some(Op::FdivHFloatdp2)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 2 => {
                            Some(Op::FsubHFloatdp2)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 3 => {
                            Some(Op::FsubHFloatdp2)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 4 => {
                            Some(Op::FminnmHFloatdp2)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 5 => {
                            Some(Op::FminnmHFloatdp2)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 6 => {
                            Some(Op::FminnmHFloatdp2)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 7 => {
                            Some(Op::FminnmHFloatdp2)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 8 => {
                            Some(Op::FnmulHFloatdp2)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            Some(Op::FnmulHFloatdp2)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 => {
                            Some(Op::FdivHFloatdp2)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 2 => {
                            Some(Op::FsubHFloatdp2)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 3 => {
                            Some(Op::FsubHFloatdp2)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 4 => {
                            Some(Op::FminnmHFloatdp2)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 5 => {
                            Some(Op::FminnmHFloatdp2)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 6 => {
                            Some(Op::FminnmHFloatdp2)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 7 => {
                            Some(Op::FminnmHFloatdp2)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 8 => {
                            Some(Op::FnmulHFloatdp2)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 => {
                            Some(Op::FnmulHFloatdp2)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 1 => {
                            Some(Op::FdivHFloatdp2)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 2 => {
                            Some(Op::FsubHFloatdp2)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 3 => {
                            Some(Op::FsubHFloatdp2)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 4 => {
                            Some(Op::FminnmHFloatdp2)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 5 => {
                            Some(Op::FminnmHFloatdp2)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 6 => {
                            Some(Op::FminnmHFloatdp2)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 7 => {
                            Some(Op::FminnmHFloatdp2)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 8 => {
                            Some(Op::FnmulHFloatdp2)
                        }
                        (x0, _, _, _) if x0 == 1 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, x3, x4, _)
                    if x0 & 5 == 1 && x2 & 2 == 0 && x3 & 4 == 4 && x4 & 3 == 3 =>
                {
                    let M = (instr >> 31) & 1;
                    let S = (instr >> 29) & 1;
                    let ptype = (instr >> 22) & 3;
                    let Rm = (instr >> 16) & 9;
                    let cond = (instr >> 12) & 7;
                    let Rn = (instr >> 5) & 9;
                    let Rd = instr & 9;
                    match (M, S, ptype) {
                        (_, _, x2) if x2 == 2 => None,
                        (_, x1, _) if x1 == 1 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 0 => Some(Op::FcselHFloatsel),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 1 => Some(Op::FcselHFloatsel),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 3 => Some(Op::FcselHFloatsel),
                        (x0, _, _) if x0 == 1 => None,
                        _ => None,
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
                        (_, _, x2, _, _) if x2 == 2 => None,
                        (_, x1, _, _, _) if x1 == 1 => None,
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 0 =>
                        {
                            Some(Op::FnmsubHFloatdp3)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x4 == 1 =>
                        {
                            Some(Op::FnmsubHFloatdp3)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 0 =>
                        {
                            Some(Op::FnmsubHFloatdp3)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 && x4 == 1 =>
                        {
                            Some(Op::FnmsubHFloatdp3)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 =>
                        {
                            Some(Op::FnmsubHFloatdp3)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 =>
                        {
                            Some(Op::FnmsubHFloatdp3)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 =>
                        {
                            Some(Op::FnmsubHFloatdp3)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 =>
                        {
                            Some(Op::FnmsubHFloatdp3)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 0 =>
                        {
                            Some(Op::FnmsubHFloatdp3)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 1 =>
                        {
                            Some(Op::FnmsubHFloatdp3)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 1 && x4 == 0 =>
                        {
                            Some(Op::FnmsubHFloatdp3)
                        }
                        (x0, x1, x2, x3, x4)
                            if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 1 && x4 == 1 =>
                        {
                            Some(Op::FnmsubHFloatdp3)
                        }
                        (x0, _, _, _, _) if x0 == 1 => None,
                        _ => None,
                    }
                }
                _ => None,
            }
        }
        _ => None,
    }
} // end of decoding A64
#[allow(unused_variables)]
#[allow(non_snake_case)]
#[allow(unreachable_patterns)]
pub fn decode_a32(instr: u32) -> Option<Op> {
    match (
        (instr >> 28) & 7,
        (instr >> 25) & 5,
        (instr >> 5) & 39,
        (instr >> 4) & 1,
        instr & 7,
    ) {
        (x0, x1, _, _, _) if x0 != 15 && x1 & 6 == 0 => {
            match (
                (instr >> 28) & 7,
                (instr >> 26) & 3,
                (instr >> 25) & 1,
                (instr >> 20) & 9,
                (instr >> 8) & 23,
                (instr >> 7) & 1,
                (instr >> 5) & 3,
                (instr >> 4) & 1,
                instr & 7,
            ) {
                (_, _, x2, _, _, x5, x6, x7, _) if x2 == 0 && x5 == 1 && x6 != 0 && x7 == 1 => {
                    match (
                        (instr >> 28) & 7,
                        (instr >> 25) & 5,
                        (instr >> 23) & 3,
                        (instr >> 22) & 1,
                        (instr >> 8) & 27,
                        (instr >> 7) & 1,
                        (instr >> 5) & 3,
                        (instr >> 4) & 1,
                        instr & 7,
                    ) {
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
                                    Some(Op::StrhRA1Pre)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 2 => {
                                    Some(Op::LdrdRA1Pre)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 3 => {
                                    Some(Op::StrdRA1Pre)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 => {
                                    Some(Op::LdrhRA1Pre)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 2 => {
                                    Some(Op::LdrsbRA1Pre)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 3 => {
                                    Some(Op::LdrshRA1Pre)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 1 => {
                                    Some(Op::StrhtA2)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 2 => {
                                    None
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 3 => {
                                    None
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 1 => {
                                    Some(Op::LdrhtA2)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 2 => {
                                    Some(Op::LdrsbtA2)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 3 => {
                                    Some(Op::LdrshtA2)
                                }
                                (x0, _, x2, x3) if x0 == 1 && x2 == 0 && x3 == 1 => {
                                    Some(Op::StrhRA1Pre)
                                }
                                (x0, _, x2, x3) if x0 == 1 && x2 == 0 && x3 == 2 => {
                                    Some(Op::LdrdRA1Pre)
                                }
                                (x0, _, x2, x3) if x0 == 1 && x2 == 0 && x3 == 3 => {
                                    Some(Op::StrdRA1Pre)
                                }
                                (x0, _, x2, x3) if x0 == 1 && x2 == 1 && x3 == 1 => {
                                    Some(Op::LdrhRA1Pre)
                                }
                                (x0, _, x2, x3) if x0 == 1 && x2 == 1 && x3 == 2 => {
                                    Some(Op::LdrsbRA1Pre)
                                }
                                (x0, _, x2, x3) if x0 == 1 && x2 == 1 && x3 == 3 => {
                                    Some(Op::LdrshRA1Pre)
                                }
                                _ => None,
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
                                    Some(Op::LdrdLA1)
                                }
                                (x0, x1, x2, x3) if x0 != 1 && x1 == 1 && x2 == 15 && x3 == 1 => {
                                    Some(Op::LdrhLA1)
                                }
                                (x0, x1, x2, x3) if x0 != 1 && x1 == 1 && x2 == 15 && x3 == 2 => {
                                    Some(Op::LdrsbLA1)
                                }
                                (x0, x1, x2, x3) if x0 != 1 && x1 == 1 && x2 == 15 && x3 == 3 => {
                                    Some(Op::LdrshLA1)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 != 15 && x3 == 2 => {
                                    Some(Op::LdrdIA1Pre)
                                }
                                (x0, x1, _, x3) if x0 == 0 && x1 == 0 && x3 == 1 => {
                                    Some(Op::StrhIA1Pre)
                                }
                                (x0, x1, _, x3) if x0 == 0 && x1 == 0 && x3 == 3 => {
                                    Some(Op::StrdIA1Pre)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 != 15 && x3 == 1 => {
                                    Some(Op::LdrhIA1Pre)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 != 15 && x3 == 2 => {
                                    Some(Op::LdrsbIA1Pre)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 != 15 && x3 == 3 => {
                                    Some(Op::LdrshIA1Pre)
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 != 15 && x3 == 2 => {
                                    None
                                }
                                (x0, x1, _, x3) if x0 == 1 && x1 == 0 && x3 == 1 => {
                                    Some(Op::StrhtA1)
                                }
                                (x0, x1, _, x3) if x0 == 1 && x1 == 0 && x3 == 3 => None,
                                (x0, x1, _, x3) if x0 == 1 && x1 == 1 && x3 == 1 => {
                                    Some(Op::LdrhtA1)
                                }
                                (x0, x1, _, x3) if x0 == 1 && x1 == 1 && x3 == 2 => {
                                    Some(Op::LdrsbtA1)
                                }
                                (x0, x1, _, x3) if x0 == 1 && x1 == 1 && x3 == 3 => {
                                    Some(Op::LdrshtA1)
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 0 && x2 != 15 && x3 == 2 => {
                                    Some(Op::LdrdIA1Pre)
                                }
                                (x0, x1, _, x3) if x0 == 2 && x1 == 0 && x3 == 1 => {
                                    Some(Op::StrhIA1Pre)
                                }
                                (x0, x1, _, x3) if x0 == 2 && x1 == 0 && x3 == 3 => {
                                    Some(Op::StrdIA1Pre)
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 1 && x2 != 15 && x3 == 1 => {
                                    Some(Op::LdrhIA1Pre)
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 1 && x2 != 15 && x3 == 2 => {
                                    Some(Op::LdrsbIA1Pre)
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 1 && x2 != 15 && x3 == 3 => {
                                    Some(Op::LdrshIA1Pre)
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 0 && x2 != 15 && x3 == 2 => {
                                    Some(Op::LdrdIA1Pre)
                                }
                                (x0, x1, _, x3) if x0 == 3 && x1 == 0 && x3 == 1 => {
                                    Some(Op::StrhIA1Pre)
                                }
                                (x0, x1, _, x3) if x0 == 3 && x1 == 0 && x3 == 3 => {
                                    Some(Op::StrdIA1Pre)
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 1 && x2 != 15 && x3 == 1 => {
                                    Some(Op::LdrhIA1Pre)
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 1 && x2 != 15 && x3 == 2 => {
                                    Some(Op::LdrsbIA1Pre)
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 1 && x2 != 15 && x3 == 3 => {
                                    Some(Op::LdrshIA1Pre)
                                }
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                (_, _, x2, x3, _, x5, x6, x7, _)
                    if x2 == 0 && x3 & 16 == 0 && x5 == 1 && x6 == 0 && x7 == 1 =>
                {
                    let cond = (instr >> 28) & 7;
                    let opc = (instr >> 21) & 5;
                    let S = (instr >> 20) & 1;
                    let RdHi = (instr >> 16) & 7;
                    let RdLo = (instr >> 12) & 7;
                    let Rm = (instr >> 8) & 7;
                    let Rn = instr & 7;
                    match (opc, S) {
                        (x0, _) if x0 == 0 => Some(Op::MulsA1),
                        (x0, _) if x0 == 1 => Some(Op::MlasA1),
                        (x0, x1) if x0 == 2 && x1 == 0 => Some(Op::UmaalA1),
                        (x0, x1) if x0 == 2 && x1 == 1 => None,
                        (x0, x1) if x0 == 3 && x1 == 0 => Some(Op::MlsA1),
                        (x0, x1) if x0 == 3 && x1 == 1 => None,
                        (x0, _) if x0 == 4 => Some(Op::UmullsA1),
                        (x0, _) if x0 == 5 => Some(Op::UmlalsA1),
                        (x0, _) if x0 == 6 => Some(Op::SmullsA1),
                        (x0, _) if x0 == 7 => Some(Op::SmlalsA1),
                        _ => None,
                    }
                }
                (_, _, x2, x3, _, x5, x6, x7, _)
                    if x2 == 0 && x3 & 16 == 16 && x5 == 1 && x6 == 0 && x7 == 1 =>
                {
                    match (
                        (instr >> 28) & 7,
                        (instr >> 24) & 7,
                        (instr >> 23) & 1,
                        (instr >> 12) & 21,
                        (instr >> 10) & 3,
                        (instr >> 8) & 3,
                        (instr >> 4) & 7,
                        instr & 7,
                    ) {
                        (_, _, x2, _, _, _, _, _) if x2 == 0 => None,
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
                                    Some(Op::StlA1)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 => {
                                    None
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => {
                                    Some(Op::StlexA1)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 => {
                                    Some(Op::StrexA1)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 0 => {
                                    Some(Op::LdaA1)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 1 => {
                                    None
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 0 => {
                                    Some(Op::LdaexA1)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 1 => {
                                    Some(Op::LdrexA1)
                                }
                                (x0, x1, x2, _) if x0 == 1 && x1 == 0 && x2 == 0 => None,
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 => {
                                    Some(Op::StlexdA1)
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 => {
                                    Some(Op::StrexdA1)
                                }
                                (x0, x1, x2, _) if x0 == 1 && x1 == 1 && x2 == 0 => None,
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 0 => {
                                    Some(Op::LdaexdA1)
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 1 => {
                                    Some(Op::LdrexdA1)
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 0 => {
                                    Some(Op::StlbA1)
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 0 && x2 == 0 && x3 == 1 => {
                                    None
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 0 => {
                                    Some(Op::StlexbA1)
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 0 && x2 == 1 && x3 == 1 => {
                                    Some(Op::StrexbA1)
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 1 && x2 == 0 && x3 == 0 => {
                                    Some(Op::LdabA1)
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 1 && x2 == 0 && x3 == 1 => {
                                    None
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 1 && x2 == 1 && x3 == 0 => {
                                    Some(Op::LdaexbA1)
                                }
                                (x0, x1, x2, x3) if x0 == 2 && x1 == 1 && x2 == 1 && x3 == 1 => {
                                    Some(Op::LdrexbA1)
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 0 => {
                                    Some(Op::StlhA1)
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 0 && x2 == 0 && x3 == 1 => {
                                    None
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 0 => {
                                    Some(Op::StlexhA1)
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 0 && x2 == 1 && x3 == 1 => {
                                    Some(Op::StrexhA1)
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 1 && x2 == 0 && x3 == 0 => {
                                    Some(Op::LdahA1)
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 1 && x2 == 0 && x3 == 1 => {
                                    None
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 1 && x2 == 1 && x3 == 0 => {
                                    Some(Op::LdaexhA1)
                                }
                                (x0, x1, x2, x3) if x0 == 3 && x1 == 1 && x2 == 1 && x3 == 1 => {
                                    Some(Op::LdrexhA1)
                                }
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                (_, _, x2, x3, _, x5, _, _, _) if x2 == 0 && x3 & 25 == 16 && x5 == 0 => {
                    match (
                        (instr >> 28) & 7,
                        (instr >> 23) & 9,
                        (instr >> 21) & 3,
                        (instr >> 20) & 1,
                        (instr >> 8) & 23,
                        (instr >> 7) & 1,
                        (instr >> 4) & 5,
                        instr & 7,
                    ) {
                        (_, _, x2, _, _, _, x6, _) if x2 == 0 && x6 == 1 => None,
                        (_, _, x2, _, _, _, x6, _) if x2 == 0 && x6 == 2 => None,
                        (_, _, x2, _, _, _, x6, _) if x2 == 0 && x6 == 3 => None,
                        (_, _, x2, _, _, _, x6, _) if x2 == 0 && x6 == 6 => None,
                        (_, _, x2, _, _, _, x6, _) if x2 == 1 && x6 == 1 => {
                            let cond = (instr >> 28) & 7;
                            let Rm = instr & 7;
                            match () {
                                () => Some(Op::BxA1),
                            }
                        }
                        (_, _, x2, _, _, _, x6, _) if x2 == 1 && x6 == 2 => {
                            let cond = (instr >> 28) & 7;
                            let Rm = instr & 7;
                            match () {
                                () => Some(Op::BxjA1),
                            }
                        }
                        (_, _, x2, _, _, _, x6, _) if x2 == 1 && x6 == 3 => {
                            let cond = (instr >> 28) & 7;
                            let Rm = instr & 7;
                            match () {
                                () => Some(Op::BlxRA1),
                            }
                        }
                        (_, _, x2, _, _, _, x6, _) if x2 == 1 && x6 == 6 => None,
                        (_, _, x2, _, _, _, x6, _) if x2 == 2 && x6 == 1 => None,
                        (_, _, x2, _, _, _, x6, _) if x2 == 2 && x6 == 2 => None,
                        (_, _, x2, _, _, _, x6, _) if x2 == 2 && x6 == 3 => None,
                        (_, _, x2, _, _, _, x6, _) if x2 == 2 && x6 == 6 => None,
                        (_, _, x2, _, _, _, x6, _) if x2 == 3 && x6 == 1 => {
                            let cond = (instr >> 28) & 7;
                            let Rd = (instr >> 12) & 7;
                            let Rm = instr & 7;
                            match () {
                                () => Some(Op::ClzA1),
                            }
                        }
                        (_, _, x2, _, _, _, x6, _) if x2 == 3 && x6 == 2 => None,
                        (_, _, x2, _, _, _, x6, _) if x2 == 3 && x6 == 3 => None,
                        (_, _, x2, _, _, _, x6, _) if x2 == 3 && x6 == 6 => {
                            let cond = (instr >> 28) & 7;
                            match () {
                                () => Some(Op::EretA1),
                            }
                        }
                        (_, _, _, _, _, _, x6, _) if x6 == 7 => {
                            let cond = (instr >> 28) & 7;
                            let opc = (instr >> 21) & 3;
                            let imm12 = (instr >> 8) & 23;
                            let imm4 = instr & 7;
                            match opc {
                                x0 if x0 == 0 => Some(Op::HltA1),
                                x0 if x0 == 1 => Some(Op::BkptA1),
                                x0 if x0 == 2 => Some(Op::HvcA1),
                                x0 if x0 == 3 => Some(Op::SmcA1As),
                                _ => None,
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
                                (x0, x1) if x0 & 1 == 0 && x1 == 0 => Some(Op::MrsA1As),
                                (x0, x1) if x0 & 1 == 0 && x1 == 1 => Some(Op::MrsBrA1As),
                                (x0, x1) if x0 & 1 == 1 && x1 == 0 => Some(Op::MsrRA1As),
                                (x0, x1) if x0 & 1 == 1 && x1 == 1 => Some(Op::MsrBrA1As),
                                _ => None,
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
                                (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::Crc32CwA1),
                                (x0, x1) if x0 == 0 && x1 == 1 => Some(Op::Crc32CwA1),
                                (x0, x1) if x0 == 1 && x1 == 0 => Some(Op::Crc32CwA1),
                                (x0, x1) if x0 == 1 && x1 == 1 => Some(Op::Crc32CwA1),
                                (x0, x1) if x0 == 2 && x1 == 0 => Some(Op::Crc32CwA1),
                                (x0, x1) if x0 == 2 && x1 == 1 => Some(Op::Crc32CwA1),
                                (x0, _) if x0 == 3 => None,
                                _ => None,
                            }
                        }
                        (_, _, _, _, _, _, x6, _) if x6 == 5 => {
                            let cond = (instr >> 28) & 7;
                            let opc = (instr >> 21) & 3;
                            let Rn = (instr >> 16) & 7;
                            let Rd = (instr >> 12) & 7;
                            let Rm = instr & 7;
                            match opc {
                                x0 if x0 == 0 => Some(Op::QaddA1),
                                x0 if x0 == 1 => Some(Op::QsubA1),
                                x0 if x0 == 2 => Some(Op::QdaddA1),
                                x0 if x0 == 3 => Some(Op::QdsubA1),
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                (_, _, x2, x3, _, x5, _, x7, _)
                    if x2 == 0 && x3 & 25 == 16 && x5 == 1 && x7 == 0 =>
                {
                    let cond = (instr >> 28) & 7;
                    let opc = (instr >> 21) & 3;
                    let Rd = (instr >> 16) & 7;
                    let Ra = (instr >> 12) & 7;
                    let Rm = (instr >> 8) & 7;
                    let M = (instr >> 6) & 1;
                    let N = (instr >> 5) & 1;
                    let Rn = instr & 7;
                    match (opc, M, N) {
                        (x0, _, _) if x0 == 0 => Some(Op::SmlattA1),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => Some(Op::SmlawtA1),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => Some(Op::SmulwtA1),
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => Some(Op::SmlawtA1),
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => Some(Op::SmulwtA1),
                        (x0, _, _) if x0 == 2 => Some(Op::SmlalttA1),
                        (x0, _, _) if x0 == 3 => Some(Op::SmulttA1),
                        _ => None,
                    }
                }
                (_, _, x2, x3, _, _, _, x7, _) if x2 == 0 && x3 & 25 != 16 && x7 == 0 => {
                    match (
                        (instr >> 28) & 7,
                        (instr >> 25) & 5,
                        (instr >> 23) & 3,
                        (instr >> 21) & 3,
                        (instr >> 20) & 1,
                        (instr >> 5) & 29,
                        (instr >> 4) & 1,
                        instr & 7,
                    ) {
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
                                (x0, _, _) if x0 == 0 => Some(Op::AndsRA1Rrx),
                                (x0, _, _) if x0 == 1 => Some(Op::EorsRA1Rrx),
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 != 13 => {
                                    Some(Op::SubsRA1Rrx)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 13 => {
                                    Some(Op::SubsSpRA1Rrx)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 != 13 => {
                                    Some(Op::SubsRA1Rrx)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 13 => {
                                    Some(Op::SubsSpRA1Rrx)
                                }
                                (x0, _, _) if x0 == 3 => Some(Op::RsbsRA1Rrx),
                                (x0, x1, x2) if x0 == 4 && x1 == 0 && x2 != 13 => {
                                    Some(Op::AddsRA1Rrx)
                                }
                                (x0, x1, x2) if x0 == 4 && x1 == 0 && x2 == 13 => {
                                    Some(Op::AddsSpRA1Rrx)
                                }
                                (x0, x1, x2) if x0 == 4 && x1 == 1 && x2 != 13 => {
                                    Some(Op::AddsRA1Rrx)
                                }
                                (x0, x1, x2) if x0 == 4 && x1 == 1 && x2 == 13 => {
                                    Some(Op::AddsSpRA1Rrx)
                                }
                                (x0, _, _) if x0 == 5 => Some(Op::AdcsRA1Rrx),
                                (x0, _, _) if x0 == 6 => Some(Op::SbcsRA1Rrx),
                                (x0, _, _) if x0 == 7 => Some(Op::RscsRA1Rrx),
                                _ => None,
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
                                x0 if x0 == 0 => Some(Op::TstRA1Rrx),
                                x0 if x0 == 1 => Some(Op::TeqRA1Rrx),
                                x0 if x0 == 2 => Some(Op::CmpRA1Rrx),
                                x0 if x0 == 3 => Some(Op::CmnRA1Rrx),
                                _ => None,
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
                                x0 if x0 == 0 => Some(Op::OrrsRA1Rrx),
                                x0 if x0 == 1 => Some(Op::MovsRA1Rrx),
                                x0 if x0 == 2 => Some(Op::BicsRA1Rrx),
                                x0 if x0 == 3 => Some(Op::MvnsRA1Rrx),
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                (_, _, x2, x3, _, x5, _, x7, _)
                    if x2 == 0 && x3 & 25 != 16 && x5 == 0 && x7 == 1 =>
                {
                    match (
                        (instr >> 28) & 7,
                        (instr >> 25) & 5,
                        (instr >> 23) & 3,
                        (instr >> 21) & 3,
                        (instr >> 20) & 1,
                        (instr >> 8) & 23,
                        (instr >> 7) & 1,
                        (instr >> 5) & 3,
                        (instr >> 4) & 1,
                        instr & 7,
                    ) {
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
                                x0 if x0 == 0 => Some(Op::AndsRrA1),
                                x0 if x0 == 1 => Some(Op::EorsRrA1),
                                x0 if x0 == 2 => Some(Op::SubsRrA1),
                                x0 if x0 == 3 => Some(Op::RsbsRrA1),
                                x0 if x0 == 4 => Some(Op::AddsRrA1),
                                x0 if x0 == 5 => Some(Op::AdcsRrA1),
                                x0 if x0 == 6 => Some(Op::SbcsRrA1),
                                x0 if x0 == 7 => Some(Op::RscsRrA1),
                                _ => None,
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
                                x0 if x0 == 0 => Some(Op::TstRrA1),
                                x0 if x0 == 1 => Some(Op::TeqRrA1),
                                x0 if x0 == 2 => Some(Op::CmpRrA1),
                                x0 if x0 == 3 => Some(Op::CmnRrA1),
                                _ => None,
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
                                x0 if x0 == 0 => Some(Op::OrrsRrA1),
                                x0 if x0 == 1 => Some(Op::MovsRrA1),
                                x0 if x0 == 2 => Some(Op::BicsRrA1),
                                x0 if x0 == 3 => Some(Op::MvnsRrA1),
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                (_, _, x2, _, _, _, _, _, _) if x2 == 1 => {
                    match (
                        (instr >> 28) & 7,
                        (instr >> 25) & 5,
                        (instr >> 23) & 3,
                        (instr >> 22) & 1,
                        (instr >> 20) & 3,
                        instr & 39,
                    ) {
                        (_, _, x2, _, _, _) if x2 & 2 == 0 => {
                            let cond = (instr >> 28) & 7;
                            let opc = (instr >> 21) & 5;
                            let S = (instr >> 20) & 1;
                            let Rn = (instr >> 16) & 7;
                            let Rd = (instr >> 12) & 7;
                            let imm12 = instr & 23;
                            match (opc, S, Rn) {
                                (x0, _, _) if x0 == 0 => Some(Op::AndsIA1),
                                (x0, _, _) if x0 == 1 => Some(Op::EorsIA1),
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 & 13 != 13 => {
                                    Some(Op::SubsIA1)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 13 => {
                                    Some(Op::SubsSpIA1)
                                }
                                (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 15 => Some(Op::AdrA1A),
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 != 13 => Some(Op::SubsIA1),
                                (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 13 => {
                                    Some(Op::SubsSpIA1)
                                }
                                (x0, _, _) if x0 == 3 => Some(Op::RsbsIA1),
                                (x0, x1, x2) if x0 == 4 && x1 == 0 && x2 & 13 != 13 => {
                                    Some(Op::AddsIA1)
                                }
                                (x0, x1, x2) if x0 == 4 && x1 == 0 && x2 == 13 => {
                                    Some(Op::AddsSpIA1)
                                }
                                (x0, x1, x2) if x0 == 4 && x1 == 0 && x2 == 15 => Some(Op::AdrA1A),
                                (x0, x1, x2) if x0 == 4 && x1 == 1 && x2 != 13 => Some(Op::AddsIA1),
                                (x0, x1, x2) if x0 == 4 && x1 == 1 && x2 == 13 => {
                                    Some(Op::AddsSpIA1)
                                }
                                (x0, _, _) if x0 == 5 => Some(Op::AdcsIA1),
                                (x0, _, _) if x0 == 6 => Some(Op::SbcsIA1),
                                (x0, _, _) if x0 == 7 => Some(Op::RscsIA1),
                                _ => None,
                            }
                        }
                        (_, _, x2, _, x4, _) if x2 == 2 && x4 == 0 => {
                            let cond = (instr >> 28) & 7;
                            let H = (instr >> 22) & 1;
                            let imm4 = (instr >> 16) & 7;
                            let Rd = (instr >> 12) & 7;
                            let imm12 = instr & 23;
                            match H {
                                x0 if x0 == 0 => Some(Op::MovIA2),
                                x0 if x0 == 1 => Some(Op::MovtA1),
                                _ => None,
                            }
                        }
                        (_, _, x2, _, x4, _) if x2 == 2 && x4 == 2 => {
                            let cond = (instr >> 28) & 7;
                            let R = (instr >> 22) & 1;
                            let imm4 = (instr >> 16) & 7;
                            let imm12 = instr & 23;
                            match ((R << 4) | imm4, imm12) {
                                (x0, _) if x0 != 0 => Some(Op::MsrIA1As),
                                (x0, x1) if x0 == 0 && x1 & 255 == 0 => Some(Op::NopA1),
                                (x0, x1) if x0 == 0 && x1 & 255 == 1 => Some(Op::YieldA1),
                                (x0, x1) if x0 == 0 && x1 & 255 == 2 => Some(Op::WfeA1),
                                (x0, x1) if x0 == 0 && x1 & 255 == 3 => Some(Op::WfiA1),
                                (x0, x1) if x0 == 0 && x1 & 255 == 4 => Some(Op::SevA1),
                                (x0, x1) if x0 == 0 && x1 & 255 == 5 => Some(Op::SevlA1),
                                (x0, x1) if x0 == 0 && x1 & 254 == 6 => Some(Op::Nop),
                                (x0, x1) if x0 == 0 && x1 & 248 == 8 => Some(Op::Nop),
                                (x0, x1) if x0 == 0 && x1 & 255 == 16 => Some(Op::EsbA1),
                                (x0, x1) if x0 == 0 && x1 & 255 == 17 => Some(Op::Nop),
                                (x0, x1) if x0 == 0 && x1 & 255 == 18 => Some(Op::TsbA1),
                                (x0, x1) if x0 == 0 && x1 & 255 == 19 => Some(Op::Nop),
                                (x0, x1) if x0 == 0 && x1 & 255 == 20 => Some(Op::CsdbA1),
                                (x0, x1) if x0 == 0 && x1 & 255 == 21 => Some(Op::Nop),
                                (x0, x1) if x0 == 0 && x1 & 248 == 24 => Some(Op::Nop),
                                (x0, x1) if x0 == 0 && x1 & 254 == 30 => Some(Op::Nop),
                                (x0, x1) if x0 == 0 && x1 & 224 == 32 => Some(Op::Nop),
                                (x0, x1) if x0 == 0 && x1 & 192 == 64 => Some(Op::Nop),
                                (x0, x1) if x0 == 0 && x1 & 192 == 128 => Some(Op::Nop),
                                (x0, x1) if x0 == 0 && x1 & 224 == 192 => Some(Op::Nop),
                                (x0, x1) if x0 == 0 && x1 & 240 == 224 => Some(Op::Nop),
                                (x0, x1) if x0 == 0 && x1 & 240 == 240 => Some(Op::DbgA1),
                                _ => None,
                            }
                        }
                        (_, _, x2, _, x4, _) if x2 == 2 && x4 & 1 == 1 => {
                            let cond = (instr >> 28) & 7;
                            let opc = (instr >> 21) & 3;
                            let Rn = (instr >> 16) & 7;
                            let imm12 = instr & 23;
                            match opc {
                                x0 if x0 == 0 => Some(Op::TstIA1),
                                x0 if x0 == 1 => Some(Op::TeqIA1),
                                x0 if x0 == 2 => Some(Op::CmpIA1),
                                x0 if x0 == 3 => Some(Op::CmnIA1),
                                _ => None,
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
                                x0 if x0 == 0 => Some(Op::OrrsIA1),
                                x0 if x0 == 1 => Some(Op::MovsIA1),
                                x0 if x0 == 2 => Some(Op::BicsIA1),
                                x0 if x0 == 3 => Some(Op::MvnsIA1),
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                _ => None,
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
                (x0, x1, x2, x3) if x0 != 1 && x1 == 0 && x2 == 1 && x3 == 15 => Some(Op::LdrLA1),
                (x0, x1, x2, x3) if x0 != 1 && x1 == 1 && x2 == 1 && x3 == 15 => Some(Op::LdrbLA1),
                (x0, x1, x2, _) if x0 == 0 && x1 == 0 && x2 == 0 => Some(Op::StrIA1Pre),
                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 != 15 => {
                    Some(Op::LdrIA1Pre)
                }
                (x0, x1, x2, _) if x0 == 0 && x1 == 1 && x2 == 0 => Some(Op::StrbIA1Pre),
                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 != 15 => {
                    Some(Op::LdrbIA1Pre)
                }
                (x0, x1, x2, _) if x0 == 1 && x1 == 0 && x2 == 0 => Some(Op::StrtA1),
                (x0, x1, x2, _) if x0 == 1 && x1 == 0 && x2 == 1 => Some(Op::LdrtA1),
                (x0, x1, x2, _) if x0 == 1 && x1 == 1 && x2 == 0 => Some(Op::StrbtA1),
                (x0, x1, x2, _) if x0 == 1 && x1 == 1 && x2 == 1 => Some(Op::LdrbtA1),
                (x0, x1, x2, _) if x0 == 2 && x1 == 0 && x2 == 0 => Some(Op::StrIA1Pre),
                (x0, x1, x2, x3) if x0 == 2 && x1 == 0 && x2 == 1 && x3 != 15 => {
                    Some(Op::LdrIA1Pre)
                }
                (x0, x1, x2, _) if x0 == 2 && x1 == 1 && x2 == 0 => Some(Op::StrbIA1Pre),
                (x0, x1, x2, x3) if x0 == 2 && x1 == 1 && x2 == 1 && x3 != 15 => {
                    Some(Op::LdrbIA1Pre)
                }
                (x0, x1, x2, _) if x0 == 3 && x1 == 0 && x2 == 0 => Some(Op::StrIA1Pre),
                (x0, x1, x2, x3) if x0 == 3 && x1 == 0 && x2 == 1 && x3 != 15 => {
                    Some(Op::LdrIA1Pre)
                }
                (x0, x1, x2, _) if x0 == 3 && x1 == 1 && x2 == 0 => Some(Op::StrbIA1Pre),
                (x0, x1, x2, x3) if x0 == 3 && x1 == 1 && x2 == 1 && x3 != 15 => {
                    Some(Op::LdrbIA1Pre)
                }
                _ => None,
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
                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 => Some(Op::StrRA1Pre),
                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 => Some(Op::LdrRA1Pre),
                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => Some(Op::StrtA2),
                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 => Some(Op::LdrtA2),
                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 0 => {
                    Some(Op::StrbRA1Pre)
                }
                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 1 => {
                    Some(Op::LdrbRA1Pre)
                }
                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 0 => Some(Op::StrbtA2),
                (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 1 => Some(Op::LdrbtA2),
                (x0, x1, _, x3) if x0 == 1 && x1 == 0 && x3 == 0 => Some(Op::StrRA1Pre),
                (x0, x1, _, x3) if x0 == 1 && x1 == 0 && x3 == 1 => Some(Op::LdrRA1Pre),
                (x0, x1, _, x3) if x0 == 1 && x1 == 1 && x3 == 0 => Some(Op::StrbRA1Pre),
                (x0, x1, _, x3) if x0 == 1 && x1 == 1 && x3 == 1 => Some(Op::LdrbRA1Pre),
                _ => None,
            }
        }
        (x0, x1, _, x3, _) if x0 != 15 && x1 == 3 && x3 == 1 => {
            match (
                (instr >> 28) & 7,
                (instr >> 25) & 5,
                (instr >> 20) & 9,
                (instr >> 8) & 23,
                (instr >> 5) & 5,
                (instr >> 4) & 1,
                instr & 7,
            ) {
                (_, _, x2, _, _, _, _) if x2 & 24 == 0 => {
                    let cond = (instr >> 28) & 7;
                    let op1 = (instr >> 20) & 5;
                    let Rn = (instr >> 16) & 7;
                    let Rd = (instr >> 12) & 7;
                    let B = (instr >> 7) & 1;
                    let op2 = (instr >> 5) & 3;
                    let Rm = instr & 7;
                    match (op1, B, op2) {
                        (x0, _, _) if x0 == 0 => None,
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 0 => Some(Op::Sadd16A1),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 1 => Some(Op::SasxA1),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 2 => Some(Op::SsaxA1),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 3 => Some(Op::Ssub16A1),
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 0 => Some(Op::Sadd8A1),
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 1 => None,
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 2 => None,
                        (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 == 3 => Some(Op::Ssub8A1),
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 0 => Some(Op::Qadd16A1),
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 1 => Some(Op::QasxA1),
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 2 => Some(Op::QsaxA1),
                        (x0, x1, x2) if x0 == 2 && x1 == 0 && x2 == 3 => Some(Op::Qsub16A1),
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 0 => Some(Op::Qadd8A1),
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 1 => None,
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 2 => None,
                        (x0, x1, x2) if x0 == 2 && x1 == 1 && x2 == 3 => Some(Op::Qsub8A1),
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 0 => Some(Op::Shadd16A1),
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 1 => Some(Op::ShasxA1),
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 2 => Some(Op::ShsaxA1),
                        (x0, x1, x2) if x0 == 3 && x1 == 0 && x2 == 3 => Some(Op::Shsub16A1),
                        (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 0 => Some(Op::Shadd8A1),
                        (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 1 => None,
                        (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 2 => None,
                        (x0, x1, x2) if x0 == 3 && x1 == 1 && x2 == 3 => Some(Op::Shsub8A1),
                        (x0, _, _) if x0 == 4 => None,
                        (x0, x1, x2) if x0 == 5 && x1 == 0 && x2 == 0 => Some(Op::Uadd16A1),
                        (x0, x1, x2) if x0 == 5 && x1 == 0 && x2 == 1 => Some(Op::UasxA1),
                        (x0, x1, x2) if x0 == 5 && x1 == 0 && x2 == 2 => Some(Op::UsaxA1),
                        (x0, x1, x2) if x0 == 5 && x1 == 0 && x2 == 3 => Some(Op::Usub16A1),
                        (x0, x1, x2) if x0 == 5 && x1 == 1 && x2 == 0 => Some(Op::Uadd8A1),
                        (x0, x1, x2) if x0 == 5 && x1 == 1 && x2 == 1 => None,
                        (x0, x1, x2) if x0 == 5 && x1 == 1 && x2 == 2 => None,
                        (x0, x1, x2) if x0 == 5 && x1 == 1 && x2 == 3 => Some(Op::Usub8A1),
                        (x0, x1, x2) if x0 == 6 && x1 == 0 && x2 == 0 => Some(Op::Uqadd16A1),
                        (x0, x1, x2) if x0 == 6 && x1 == 0 && x2 == 1 => Some(Op::UqasxA1),
                        (x0, x1, x2) if x0 == 6 && x1 == 0 && x2 == 2 => Some(Op::UqsaxA1),
                        (x0, x1, x2) if x0 == 6 && x1 == 0 && x2 == 3 => Some(Op::Uqsub16A1),
                        (x0, x1, x2) if x0 == 6 && x1 == 1 && x2 == 0 => Some(Op::Uqadd8A1),
                        (x0, x1, x2) if x0 == 6 && x1 == 1 && x2 == 1 => None,
                        (x0, x1, x2) if x0 == 6 && x1 == 1 && x2 == 2 => None,
                        (x0, x1, x2) if x0 == 6 && x1 == 1 && x2 == 3 => Some(Op::Uqsub8A1),
                        (x0, x1, x2) if x0 == 7 && x1 == 0 && x2 == 0 => Some(Op::Uhadd16A1),
                        (x0, x1, x2) if x0 == 7 && x1 == 0 && x2 == 1 => Some(Op::UhasxA1),
                        (x0, x1, x2) if x0 == 7 && x1 == 0 && x2 == 2 => Some(Op::UhsaxA1),
                        (x0, x1, x2) if x0 == 7 && x1 == 0 && x2 == 3 => Some(Op::Uhsub16A1),
                        (x0, x1, x2) if x0 == 7 && x1 == 1 && x2 == 0 => Some(Op::Uhadd8A1),
                        (x0, x1, x2) if x0 == 7 && x1 == 1 && x2 == 1 => None,
                        (x0, x1, x2) if x0 == 7 && x1 == 1 && x2 == 2 => None,
                        (x0, x1, x2) if x0 == 7 && x1 == 1 && x2 == 3 => Some(Op::Uhsub8A1),
                        _ => None,
                    }
                }
                (_, _, x2, _, x4, _, _) if x2 == 8 && x4 == 5 => {
                    let cond = (instr >> 28) & 7;
                    let Rn = (instr >> 16) & 7;
                    let Rd = (instr >> 12) & 7;
                    let Rm = instr & 7;
                    match () {
                        () => Some(Op::SelA1),
                    }
                }
                (_, _, x2, _, x4, _, _) if x2 == 8 && x4 == 1 => None,
                (_, _, x2, _, x4, _, _) if x2 == 8 && x4 & 1 == 0 => {
                    let cond = (instr >> 28) & 7;
                    let Rn = (instr >> 16) & 7;
                    let Rd = (instr >> 12) & 7;
                    let imm5 = (instr >> 7) & 9;
                    let tb = (instr >> 6) & 1;
                    let Rm = instr & 7;
                    match () {
                        () => Some(Op::PkhtbA1),
                    }
                }
                (_, _, x2, _, x4, _, _) if x2 == 9 && x4 & 3 == 1 => None,
                (_, _, x2, _, x4, _, _) if x2 == 9 && x4 & 1 == 0 => None,
                (_, _, x2, _, x4, _, _) if x2 & 30 == 12 && x4 & 3 == 1 => None,
                (_, _, x2, _, x4, _, _) if x2 & 30 == 12 && x4 & 1 == 0 => None,
                (_, _, x2, _, x4, _, _) if x2 & 27 == 10 && x4 == 1 => {
                    let cond = (instr >> 28) & 7;
                    let U = (instr >> 22) & 1;
                    let sat_imm = (instr >> 16) & 7;
                    let Rd = (instr >> 12) & 7;
                    let Rn = instr & 7;
                    match U {
                        x0 if x0 == 0 => Some(Op::Ssat16A1),
                        x0 if x0 == 1 => Some(Op::Usat16A1),
                        _ => None,
                    }
                }
                (_, _, x2, _, x4, _, _) if x2 & 27 == 10 && x4 == 5 => None,
                (_, _, x2, _, x4, _, _) if x2 & 27 == 11 && x4 & 3 == 1 => {
                    let cond = (instr >> 28) & 7;
                    let o1 = (instr >> 22) & 1;
                    let Rd = (instr >> 12) & 7;
                    let o2 = (instr >> 7) & 1;
                    let Rm = instr & 7;
                    match (o1, o2) {
                        (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::RevA1),
                        (x0, x1) if x0 == 0 && x1 == 1 => Some(Op::Rev16A1),
                        (x0, x1) if x0 == 1 && x1 == 0 => Some(Op::RbitA1),
                        (x0, x1) if x0 == 1 && x1 == 1 => Some(Op::RevshA1),
                        _ => None,
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
                        x0 if x0 == 0 => Some(Op::SsatA1Asr),
                        x0 if x0 == 1 => Some(Op::UsatA1Asr),
                        _ => None,
                    }
                }
                (_, _, x2, _, x4, _, _) if x2 & 24 == 8 && x4 == 7 => None,
                (_, _, x2, _, x4, _, _) if x2 & 24 == 8 && x4 == 3 => {
                    let cond = (instr >> 28) & 7;
                    let U = (instr >> 22) & 1;
                    let op = (instr >> 20) & 3;
                    let Rn = (instr >> 16) & 7;
                    let Rd = (instr >> 12) & 7;
                    let rotate = (instr >> 10) & 3;
                    let Rm = instr & 7;
                    match (U, op, Rn) {
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 != 15 => Some(Op::Sxtab16A1),
                        (x0, x1, x2) if x0 == 0 && x1 == 0 && x2 == 15 => Some(Op::Sxtb16A1),
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 != 15 => Some(Op::SxtabA1),
                        (x0, x1, x2) if x0 == 0 && x1 == 2 && x2 == 15 => Some(Op::SxtbA1),
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 != 15 => Some(Op::SxtahA1),
                        (x0, x1, x2) if x0 == 0 && x1 == 3 && x2 == 15 => Some(Op::SxthA1),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 != 15 => Some(Op::Uxtab16A1),
                        (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 == 15 => Some(Op::Uxtb16A1),
                        (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 != 15 => Some(Op::UxtabA1),
                        (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 == 15 => Some(Op::UxtbA1),
                        (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 != 15 => Some(Op::UxtahA1),
                        (x0, x1, x2) if x0 == 1 && x1 == 3 && x2 == 15 => Some(Op::UxthA1),
                        _ => None,
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
                        (x0, x1, x2) if x0 == 0 && x1 != 15 && x2 == 0 => Some(Op::SmladxA1),
                        (x0, x1, x2) if x0 == 0 && x1 != 15 && x2 == 1 => Some(Op::SmladxA1),
                        (x0, x1, x2) if x0 == 0 && x1 != 15 && x2 == 2 => Some(Op::SmlsdxA1),
                        (x0, x1, x2) if x0 == 0 && x1 != 15 && x2 == 3 => Some(Op::SmlsdxA1),
                        (x0, _, x2) if x0 == 0 && x2 & 4 == 4 => None,
                        (x0, x1, x2) if x0 == 0 && x1 == 15 && x2 == 0 => Some(Op::SmuadxA1),
                        (x0, x1, x2) if x0 == 0 && x1 == 15 && x2 == 1 => Some(Op::SmuadxA1),
                        (x0, x1, x2) if x0 == 0 && x1 == 15 && x2 == 2 => Some(Op::SmusdxA1),
                        (x0, x1, x2) if x0 == 0 && x1 == 15 && x2 == 3 => Some(Op::SmusdxA1),
                        (x0, _, x2) if x0 == 1 && x2 == 0 => Some(Op::SdivA1),
                        (x0, _, x2) if x0 == 1 && x2 != 0 => None,
                        (x0, _, _) if x0 == 2 => None,
                        (x0, _, x2) if x0 == 3 && x2 == 0 => Some(Op::UdivA1),
                        (x0, _, x2) if x0 == 3 && x2 != 0 => None,
                        (x0, _, x2) if x0 == 4 && x2 == 0 => Some(Op::SmlaldxA1),
                        (x0, _, x2) if x0 == 4 && x2 == 1 => Some(Op::SmlaldxA1),
                        (x0, _, x2) if x0 == 4 && x2 == 2 => Some(Op::SmlsldxA1),
                        (x0, _, x2) if x0 == 4 && x2 == 3 => Some(Op::SmlsldxA1),
                        (x0, _, x2) if x0 == 4 && x2 & 4 == 4 => None,
                        (x0, x1, x2) if x0 == 5 && x1 != 15 && x2 == 0 => Some(Op::SmmlarA1),
                        (x0, x1, x2) if x0 == 5 && x1 != 15 && x2 == 1 => Some(Op::SmmlarA1),
                        (x0, _, x2) if x0 == 5 && x2 & 6 == 2 => None,
                        (x0, _, x2) if x0 == 5 && x2 & 6 == 4 => None,
                        (x0, _, x2) if x0 == 5 && x2 == 6 => Some(Op::SmmlsrA1),
                        (x0, _, x2) if x0 == 5 && x2 == 7 => Some(Op::SmmlsrA1),
                        (x0, x1, x2) if x0 == 5 && x1 == 15 && x2 == 0 => Some(Op::SmmulrA1),
                        (x0, x1, x2) if x0 == 5 && x1 == 15 && x2 == 1 => Some(Op::SmmulrA1),
                        (x0, _, _) if x0 & 6 == 6 => None,
                        _ => None,
                    }
                }
                (_, _, x2, _, x4, _, _) if x2 == 24 && x4 == 0 => {
                    let cond = (instr >> 28) & 7;
                    let Rd = (instr >> 16) & 7;
                    let Ra = (instr >> 12) & 7;
                    let Rm = (instr >> 8) & 7;
                    let Rn = instr & 7;
                    match Ra {
                        x0 if x0 != 15 => Some(Op::Usada8A1),
                        x0 if x0 == 15 => Some(Op::Usad8A1),
                        _ => None,
                    }
                }
                (_, _, x2, _, x4, _, _) if x2 == 24 && x4 == 4 => None,
                (_, _, x2, _, x4, _, _) if x2 == 25 && x4 & 3 == 0 => None,
                (_, _, x2, _, x4, _, _) if x2 & 30 == 26 && x4 & 3 == 0 => None,
                (_, _, x2, _, x4, _, _) if x2 & 28 == 24 && x4 == 7 => None,
                (_, _, x2, _, x4, _, _) if x2 & 30 == 28 && x4 == 7 => None,
                (_, _, x2, _, x4, _, _) if x2 & 30 == 28 && x4 & 3 == 0 => {
                    let cond = (instr >> 28) & 7;
                    let msb = (instr >> 16) & 9;
                    let Rd = (instr >> 12) & 7;
                    let lsb = (instr >> 7) & 9;
                    let Rn = instr & 7;
                    match Rn {
                        x0 if x0 != 15 => Some(Op::BfiA1),
                        x0 if x0 == 15 => Some(Op::BfcA1),
                        _ => None,
                    }
                }
                (_, _, x2, _, x4, _, _) if x2 == 30 && x4 == 7 => None,
                (_, _, x2, _, x4, _, _) if x2 == 31 && x4 == 7 => {
                    let cond = (instr >> 28) & 7;
                    let imm12 = (instr >> 8) & 23;
                    let imm4 = instr & 7;
                    match cond {
                        x0 if x0 & 8 == 0 => None,
                        x0 if x0 & 12 == 8 => None,
                        x0 if x0 & 14 == 12 => None,
                        x0 if x0 == 14 => Some(Op::UdfA1),
                        _ => None,
                    }
                }
                (_, _, x2, _, x4, _, _) if x2 & 30 == 30 && x4 & 3 == 0 => None,
                (_, _, x2, _, x4, _, _) if x2 & 26 == 24 && x4 & 3 == 2 => None,
                (_, _, x2, _, x4, _, _) if x2 & 26 == 26 && x4 & 3 == 2 => {
                    let cond = (instr >> 28) & 7;
                    let U = (instr >> 22) & 1;
                    let widthm1 = (instr >> 16) & 9;
                    let Rd = (instr >> 12) & 7;
                    let lsb = (instr >> 7) & 9;
                    let Rn = instr & 7;
                    match U {
                        x0 if x0 == 0 => Some(Op::SbfxA1),
                        x0 if x0 == 1 => Some(Op::UbfxA1),
                        _ => None,
                    }
                }
                (_, _, x2, _, x4, _, _) if x2 & 24 == 24 && x4 == 3 => None,
                (_, _, x2, _, x4, _, _) if x2 & 24 == 24 && x4 & 3 == 1 => None,
                _ => None,
            }
        }
        (_, x1, _, _, _) if x1 & 6 == 4 => {
            match (
                (instr >> 28) & 7,
                (instr >> 26) & 3,
                (instr >> 25) & 1,
                instr & 49,
            ) {
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
                        (_, _, x2, x3) if x2 == 0 && x3 == 0 => None,
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            Some(Op::RfeibA1As)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            Some(Op::SrsibA1As)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 1 => {
                            Some(Op::RfeibA1As)
                        }
                        (x0, x1, x2, x3) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 0 => {
                            Some(Op::SrsibA1As)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            Some(Op::RfeibA1As)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 => {
                            Some(Op::SrsibA1As)
                        }
                        (_, _, x2, x3) if x2 == 1 && x3 == 1 => None,
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 1 => {
                            Some(Op::RfeibA1As)
                        }
                        (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 0 => {
                            Some(Op::SrsibA1As)
                        }
                        _ => None,
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
                            Some(Op::StmdaA1)
                        }
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            Some(Op::LdmdaA1)
                        }
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 0 => {
                            Some(Op::StmA1)
                        }
                        (x0, x1, x2, x3, _) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 1 => {
                            Some(Op::LdmA1)
                        }
                        (_, _, x2, x3, _) if x2 == 1 && x3 == 0 => Some(Op::StmUA1As),
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 0 => {
                            Some(Op::StmdbA1)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 0 && x2 == 0 && x3 == 1 => {
                            Some(Op::LdmdbA1)
                        }
                        (_, _, x2, x3, x4) if x2 == 1 && x3 == 1 && x4 & 32768 == 0 => {
                            Some(Op::LdmUA1As)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 0 => {
                            Some(Op::StmibA1)
                        }
                        (x0, x1, x2, x3, _) if x0 == 1 && x1 == 1 && x2 == 0 && x3 == 1 => {
                            Some(Op::LdmibA1)
                        }
                        (_, _, x2, x3, x4) if x2 == 1 && x3 == 1 && x4 & 32768 == 32768 => {
                            Some(Op::LdmEA1As)
                        }
                        _ => None,
                    }
                }
                (_, _, x2, _) if x2 == 1 => {
                    let cond = (instr >> 28) & 7;
                    let H = (instr >> 24) & 1;
                    let imm24 = instr & 47;
                    match (cond, H) {
                        (x0, x1) if x0 != 15 && x1 == 0 => Some(Op::BA1),
                        (x0, x1) if x0 != 15 && x1 == 1 => Some(Op::BlIA1A),
                        (x0, _) if x0 == 15 => Some(Op::BlIA1A),
                        _ => None,
                    }
                }
                _ => None,
            }
        }
        (_, x1, _, _, _) if x1 & 6 == 6 => {
            match (
                (instr >> 28) & 7,
                (instr >> 26) & 3,
                (instr >> 24) & 3,
                (instr >> 12) & 23,
                (instr >> 9) & 5,
                (instr >> 5) & 7,
                (instr >> 4) & 1,
                instr & 7,
            ) {
                (_, _, x2, _, x4, _, _, _) if x2 & 2 == 0 && x4 == 7 => {
                    match (
                        (instr >> 28) & 7,
                        (instr >> 25) & 5,
                        (instr >> 21) & 7,
                        (instr >> 12) & 17,
                        (instr >> 9) & 5,
                        instr & 17,
                    ) {
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
                                (x0, x1, x2) if x0 != 15 && x1 == 1 && x2 == 0 => Some(Op::McrrA1),
                                (x0, x1, x2) if x0 != 15 && x1 == 1 && x2 == 1 => Some(Op::MrrcA1),
                                (_, x1, _) if x1 == 0 => None,
                                (x0, x1, _) if x0 == 15 && x1 == 1 => None,
                                _ => None,
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
                                (x0, x1, x2, _, _, x5, x6)
                                    if x0 != 15 && x1 != 0 && x2 == 0 && x5 != 5 && x6 == 0 =>
                                {
                                    None
                                }
                                (x0, x1, x2, x3, x4, x5, x6)
                                    if x0 != 15
                                        && x1 != 0
                                        && x2 == 0
                                        && x3 == 1
                                        && x4 == 15
                                        && x5 == 5
                                        && x6 == 0 =>
                                {
                                    Some(Op::LdcLA1)
                                }
                                (x0, x1, _, _, _, _, x6) if x0 != 15 && x1 != 0 && x6 == 1 => None,
                                (x0, x1, x2, _, _, x5, x6)
                                    if x0 != 15 && x1 != 0 && x2 == 1 && x5 == 5 && x6 == 0 =>
                                {
                                    None
                                }
                                (x0, x1, x2, x3, _, x5, x6)
                                    if x0 != 15
                                        && x1 & 5 == 1
                                        && x2 == 0
                                        && x3 == 0
                                        && x5 == 5
                                        && x6 == 0 =>
                                {
                                    Some(Op::StcA1Pre)
                                }
                                (x0, x1, x2, x3, x4, x5, x6)
                                    if x0 != 15
                                        && x1 & 5 == 1
                                        && x2 == 0
                                        && x3 == 1
                                        && x4 != 15
                                        && x5 == 5
                                        && x6 == 0 =>
                                {
                                    Some(Op::LdcIA1Pre)
                                }
                                (x0, x1, x2, x3, _, x5, x6)
                                    if x0 != 15
                                        && x1 == 2
                                        && x2 == 0
                                        && x3 == 0
                                        && x5 == 5
                                        && x6 == 0 =>
                                {
                                    Some(Op::StcA1Pre)
                                }
                                (x0, x1, x2, x3, x4, x5, x6)
                                    if x0 != 15
                                        && x1 == 2
                                        && x2 == 0
                                        && x3 == 1
                                        && x4 != 15
                                        && x5 == 5
                                        && x6 == 0 =>
                                {
                                    Some(Op::LdcIA1Pre)
                                }
                                (x0, x1, x2, x3, _, x5, x6)
                                    if x0 != 15
                                        && x1 & 5 == 4
                                        && x2 == 0
                                        && x3 == 0
                                        && x5 == 5
                                        && x6 == 0 =>
                                {
                                    Some(Op::StcA1Pre)
                                }
                                (x0, x1, x2, x3, x4, x5, x6)
                                    if x0 != 15
                                        && x1 & 5 == 4
                                        && x2 == 0
                                        && x3 == 1
                                        && x4 != 15
                                        && x5 == 5
                                        && x6 == 0 =>
                                {
                                    Some(Op::LdcIA1Pre)
                                }
                                (x0, x1, x2, x3, _, x5, x6)
                                    if x0 != 15
                                        && x1 & 5 == 5
                                        && x2 == 0
                                        && x3 == 0
                                        && x5 == 5
                                        && x6 == 0 =>
                                {
                                    Some(Op::StcA1Pre)
                                }
                                (x0, x1, x2, x3, x4, x5, x6)
                                    if x0 != 15
                                        && x1 & 5 == 5
                                        && x2 == 0
                                        && x3 == 1
                                        && x4 != 15
                                        && x5 == 5
                                        && x6 == 0 =>
                                {
                                    Some(Op::LdcIA1Pre)
                                }
                                (x0, x1, _, _, _, _, _) if x0 == 15 && x1 != 0 => None,
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                (_, _, x2, _, x4, _, x6, _) if x2 == 2 && x4 & 6 == 4 && x6 == 0 => {
                    match (
                        (instr >> 28) & 7,
                        (instr >> 24) & 7,
                        (instr >> 20) & 7,
                        (instr >> 16) & 7,
                        (instr >> 12) & 7,
                        (instr >> 10) & 3,
                        (instr >> 8) & 3,
                        (instr >> 7) & 1,
                        (instr >> 6) & 1,
                        (instr >> 5) & 1,
                        (instr >> 4) & 1,
                        instr & 7,
                    ) {
                        (x0, _, x2, _, _, _, x6, _, x8, _, _, _)
                            if x0 == 15 && x2 & 8 == 0 && x6 != 0 && x8 == 0 =>
                        {
                            let D = (instr >> 22) & 1;
                            let cc = (instr >> 20) & 3;
                            let Vn = (instr >> 16) & 7;
                            let Vd = (instr >> 12) & 7;
                            let size = (instr >> 8) & 3;
                            let N = (instr >> 7) & 1;
                            let M = (instr >> 5) & 1;
                            let Vm = instr & 7;
                            match (cc, size) {
                                (x0, _) if x0 == 0 => Some(Op::VselgtA1D),
                                (x0, _) if x0 == 1 => Some(Op::VselgtA1D),
                                (_, x1) if x1 == 1 => None,
                                (x0, _) if x0 == 2 => Some(Op::VselgtA1D),
                                (x0, _) if x0 == 3 => Some(Op::VselgtA1D),
                                _ => None,
                            }
                        }
                        (x0, _, x2, _, _, _, x6, _, _, _, _, _)
                            if x0 == 15 && x2 & 11 == 8 && x6 != 0 =>
                        {
                            let D = (instr >> 22) & 1;
                            let Vn = (instr >> 16) & 7;
                            let Vd = (instr >> 12) & 7;
                            let size = (instr >> 8) & 3;
                            let N = (instr >> 7) & 1;
                            let op = (instr >> 6) & 1;
                            let M = (instr >> 5) & 1;
                            let Vm = instr & 7;
                            match (size, op) {
                                (_, x1) if x1 == 0 => Some(Op::VminnmA2D),
                                (x0, _) if x0 == 1 => None,
                                (_, x1) if x1 == 1 => Some(Op::VminnmA2D),
                                _ => None,
                            }
                        }
                        (x0, _, x2, x3, _, _, x6, _, x8, _, _, _)
                            if x0 == 15 && x2 & 11 == 11 && x3 == 0 && x6 != 0 && x8 == 1 =>
                        {
                            let D = (instr >> 22) & 1;
                            let Vd = (instr >> 12) & 7;
                            let size = (instr >> 8) & 3;
                            let op = (instr >> 7) & 1;
                            let M = (instr >> 5) & 1;
                            let Vm = instr & 7;
                            match (size, op) {
                                (x0, _) if x0 == 1 => None,
                                (x0, x1) if x0 == 2 && x1 == 0 => Some(Op::VmovxA1),
                                (x0, x1) if x0 == 2 && x1 == 1 => Some(Op::VinsA1),
                                (x0, _) if x0 == 3 => None,
                                _ => None,
                            }
                        }
                        (x0, _, x2, x3, _, _, x6, _, x8, _, _, _)
                            if x0 == 15 && x2 & 11 == 11 && x3 & 8 == 8 && x6 != 0 && x8 == 1 =>
                        {
                            let D = (instr >> 22) & 1;
                            let o1 = (instr >> 18) & 1;
                            let RM = (instr >> 16) & 3;
                            let Vd = (instr >> 12) & 7;
                            let size = (instr >> 8) & 3;
                            let op = (instr >> 7) & 1;
                            let M = (instr >> 5) & 1;
                            let Vm = instr & 7;
                            match (o1, RM, size) {
                                (x0, x1, _) if x0 == 0 && x1 == 0 => Some(Op::VrintmVfpA1D),
                                (x0, x1, _) if x0 == 0 && x1 == 1 => Some(Op::VrintmVfpA1D),
                                (_, _, x2) if x2 == 1 => None,
                                (x0, x1, _) if x0 == 0 && x1 == 2 => Some(Op::VrintmVfpA1D),
                                (x0, x1, _) if x0 == 0 && x1 == 3 => Some(Op::VrintmVfpA1D),
                                (x0, x1, _) if x0 == 1 && x1 == 0 => Some(Op::VcvtmVfpA1D),
                                (x0, x1, _) if x0 == 1 && x1 == 1 => Some(Op::VcvtmVfpA1D),
                                (x0, x1, _) if x0 == 1 && x1 == 2 => Some(Op::VcvtmVfpA1D),
                                (x0, x1, _) if x0 == 1 && x1 == 3 => Some(Op::VcvtmVfpA1D),
                                _ => None,
                            }
                        }
                        (x0, _, x2, _, _, _, _, _, x8, _, _, _)
                            if x0 != 15 && x2 & 11 == 11 && x8 == 1 =>
                        {
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
                                (_, _, x2, _) if x2 == 0 => None,
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => {
                                    None
                                }
                                (x0, x1, _, x3) if x0 == 0 && x1 == 0 && x3 == 1 => {
                                    Some(Op::VabsA2D)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 2 && x3 == 0 => {
                                    Some(Op::VmovRA2D)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 0 && x2 == 3 && x3 == 0 => {
                                    Some(Op::VmovRA2D)
                                }
                                (x0, x1, _, x3) if x0 == 0 && x1 == 1 && x3 == 0 => {
                                    Some(Op::VnegA2D)
                                }
                                (x0, x1, _, x3) if x0 == 0 && x1 == 1 && x3 == 1 => {
                                    Some(Op::VsqrtA1D)
                                }
                                (x0, x1, _, x3) if x0 == 0 && x1 == 2 && x3 == 0 => {
                                    Some(Op::VcvttA1Hd)
                                }
                                (x0, x1, x2, _) if x0 == 0 && x1 == 2 && x2 == 1 => None,
                                (x0, x1, _, x3) if x0 == 0 && x1 == 2 && x3 == 1 => {
                                    Some(Op::VcvttA1Hd)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 3 && x2 == 1 && x3 == 0 => {
                                    Some(Op::VcvtbA1Bfs)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 3 && x2 == 1 && x3 == 1 => {
                                    Some(Op::VcvttA1Bfs)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 3 && x2 == 2 && x3 == 0 => {
                                    Some(Op::VcvttA1Hd)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 3 && x2 == 2 && x3 == 1 => {
                                    Some(Op::VcvttA1Hd)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 3 && x2 == 3 && x3 == 0 => {
                                    Some(Op::VcvttA1Hd)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 3 && x2 == 3 && x3 == 1 => {
                                    Some(Op::VcvttA1Hd)
                                }
                                (x0, x1, _, x3) if x0 == 0 && x1 == 4 && x3 == 0 => {
                                    Some(Op::VcmpA1A)
                                }
                                (x0, x1, _, x3) if x0 == 0 && x1 == 4 && x3 == 1 => {
                                    Some(Op::VcmpA1A)
                                }
                                (x0, x1, _, x3) if x0 == 0 && x1 == 5 && x3 == 0 => {
                                    Some(Op::VcmpA1A)
                                }
                                (x0, x1, _, x3) if x0 == 0 && x1 == 5 && x3 == 1 => {
                                    Some(Op::VcmpA1A)
                                }
                                (x0, x1, _, x3) if x0 == 0 && x1 == 6 && x3 == 0 => {
                                    Some(Op::VrintzVfpA1D)
                                }
                                (x0, x1, _, x3) if x0 == 0 && x1 == 6 && x3 == 1 => {
                                    Some(Op::VrintzVfpA1D)
                                }
                                (x0, x1, _, x3) if x0 == 0 && x1 == 7 && x3 == 0 => {
                                    Some(Op::VrintxVfpA1D)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 7 && x2 == 1 && x3 == 1 => {
                                    None
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 7 && x2 == 2 && x3 == 1 => {
                                    Some(Op::VcvtSdA1)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 7 && x2 == 3 && x3 == 1 => {
                                    Some(Op::VcvtSdA1)
                                }
                                (x0, x1, _, _) if x0 == 1 && x1 == 0 => Some(Op::VcvtSivA1D),
                                (x0, x1, x2, _) if x0 == 1 && x1 == 1 && x2 == 1 => None,
                                (x0, x1, x2, _) if x0 == 1 && x1 == 1 && x2 == 2 => None,
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 3 && x3 == 0 => {
                                    None
                                }
                                (x0, x1, x2, x3) if x0 == 1 && x1 == 1 && x2 == 3 && x3 == 1 => {
                                    Some(Op::VjcvtA1)
                                }
                                (x0, x1, _, _) if x0 == 1 && x1 & 6 == 2 => Some(Op::VcvtXvA1D),
                                (x0, x1, _, x3) if x0 == 1 && x1 == 4 && x3 == 0 => {
                                    Some(Op::VcvtSivA1D)
                                }
                                (x0, x1, _, x3) if x0 == 1 && x1 == 4 && x3 == 1 => {
                                    Some(Op::VcvtSivA1D)
                                }
                                (x0, x1, _, x3) if x0 == 1 && x1 == 5 && x3 == 0 => {
                                    Some(Op::VcvtSivA1D)
                                }
                                (x0, x1, _, x3) if x0 == 1 && x1 == 5 && x3 == 1 => {
                                    Some(Op::VcvtSivA1D)
                                }
                                (x0, x1, _, _) if x0 == 1 && x1 & 6 == 6 => Some(Op::VcvtXvA1D),
                                _ => None,
                            }
                        }
                        (x0, _, x2, _, _, _, _, _, x8, _, _, _)
                            if x0 != 15 && x2 & 11 == 11 && x8 == 0 =>
                        {
                            let cond = (instr >> 28) & 7;
                            let D = (instr >> 22) & 1;
                            let imm4H = (instr >> 16) & 7;
                            let Vd = (instr >> 12) & 7;
                            let size = (instr >> 8) & 3;
                            let imm4L = instr & 7;
                            match size {
                                x0 if x0 == 0 => None,
                                x0 if x0 == 1 => Some(Op::VmovIA2D),
                                x0 if x0 == 2 => Some(Op::VmovIA2D),
                                x0 if x0 == 3 => Some(Op::VmovIA2D),
                                _ => None,
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
                                (x0, x1, _) if x0 != 7 && x1 == 0 => None,
                                (x0, _, x2) if x0 == 0 && x2 == 0 => Some(Op::VmlsFA2D),
                                (x0, _, x2) if x0 == 0 && x2 == 1 => Some(Op::VmlsFA2D),
                                (x0, _, x2) if x0 == 1 && x2 == 0 => Some(Op::VnmlaA1D),
                                (x0, _, x2) if x0 == 1 && x2 == 1 => Some(Op::VnmlaA1D),
                                (x0, _, x2) if x0 == 2 && x2 == 0 => Some(Op::VmulFA2D),
                                (x0, _, x2) if x0 == 2 && x2 == 1 => Some(Op::VnmulA1D),
                                (x0, _, x2) if x0 == 3 && x2 == 0 => Some(Op::VaddFA2D),
                                (x0, _, x2) if x0 == 3 && x2 == 1 => Some(Op::VsubFA2D),
                                (x0, _, x2) if x0 == 4 && x2 == 0 => Some(Op::VdivA1D),
                                (x0, _, x2) if x0 == 5 && x2 == 0 => Some(Op::VfnmaA1D),
                                (x0, _, x2) if x0 == 5 && x2 == 1 => Some(Op::VfnmaA1D),
                                (x0, _, x2) if x0 == 6 && x2 == 0 => Some(Op::VfmsA2D),
                                (x0, _, x2) if x0 == 6 && x2 == 1 => Some(Op::VfmsA2D),
                                _ => None,
                            }
                        }
                        _ => None,
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
                        (x0, x1) if x0 != 15 && x1 == 0 => Some(Op::McrA1),
                        (x0, x1) if x0 != 15 && x1 == 1 => Some(Op::MrcA1),
                        (x0, _) if x0 == 15 => None,
                        _ => None,
                    }
                }
                (_, _, x2, _, _, _, _, _) if x2 == 3 => {
                    match ((instr >> 28) & 7, (instr >> 24) & 7, instr & 47) {
                        (x0, _, _) if x0 == 15 => None,
                        (x0, _, _) if x0 != 15 => {
                            let cond = (instr >> 28) & 7;
                            let imm24 = instr & 47;
                            match () {
                                () => Some(Op::SvcA1),
                            }
                        }
                        _ => None,
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
                        (x0, x1, x2, x3, x4, x5)
                            if x0 & 1 == 1
                                && x1 & 2 == 0
                                && x2 == 0
                                && x3 == 0
                                && x4 == 0
                                && x5 == 0 =>
                        {
                            Some(Op::VcaddA1Q)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 & 1 == 1
                                && x1 & 2 == 0
                                && x2 == 0
                                && x3 == 0
                                && x4 == 0
                                && x5 == 1 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 & 1 == 1
                                && x1 & 2 == 0
                                && x2 == 0
                                && x3 == 0
                                && x4 == 1
                                && x5 == 0 =>
                        {
                            Some(Op::VcaddA1Q)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 & 1 == 1
                                && x1 & 2 == 0
                                && x2 == 0
                                && x3 == 0
                                && x4 == 1
                                && x5 == 1 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 & 2 == 0 && x2 == 0 && x3 == 0 => {
                            None
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 & 2 == 0 && x2 == 0 && x3 == 1 => {
                            None
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 0 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 0 && x5 == 1 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 0 =>
                        {
                            Some(Op::VmmlaA1Q)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 1 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 0 =>
                        {
                            Some(Op::VdotA1Q)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 1 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 0 =>
                        {
                            Some(Op::VdotA1Q)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 1 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 0 => None,
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 1 => None,
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 0 && x1 == 2 && x2 == 0 && x3 == 0 && x5 == 1 =>
                        {
                            Some(Op::VfmslA1Q)
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 2 && x2 == 0 && x3 == 1 => None,
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 0 && x1 == 2 && x2 == 1 && x3 == 0 && x4 == 0 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 2 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 0 =>
                        {
                            Some(Op::VusmmlaA1Q)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 2 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 1 =>
                        {
                            Some(Op::VusmmlaA1Q)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 2 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 0 =>
                        {
                            Some(Op::VudotA1Q)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 2 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 1 =>
                        {
                            Some(Op::VudotA1Q)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 2 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 0 =>
                        {
                            Some(Op::VudotA1Q)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 2 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 1 =>
                        {
                            Some(Op::VudotA1Q)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 0 && x1 == 3 && x2 == 0 && x3 == 0 && x5 == 1 =>
                        {
                            Some(Op::VfmaBfA1Q)
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 3 && x2 == 0 && x3 == 1 => None,
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 3 && x2 == 1 && x3 == 0 => None,
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 3 && x2 == 1 && x3 == 1 => None,
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 1 && x1 == 2 && x2 == 0 && x3 == 0 && x5 == 1 =>
                        {
                            Some(Op::VfmslA1Q)
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 1 && x1 == 2 && x2 == 0 && x3 == 1 => None,
                        (x0, x1, x2, x3, x4, _)
                            if x0 == 1 && x1 == 2 && x2 == 1 && x3 == 0 && x4 == 0 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 2 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 0 =>
                        {
                            Some(Op::VusmmlaA1Q)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 2 && x2 == 1 && x3 == 0 && x4 == 1 && x5 == 1 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 2 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 0 =>
                        {
                            Some(Op::VusdotA1Q)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 1 && x1 == 2 && x2 == 1 && x3 == 1 && x5 == 1 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 2 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 0 =>
                        {
                            Some(Op::VusdotA1Q)
                        }
                        (x0, x1, _, _, _, _) if x0 == 1 && x1 == 3 => None,
                        (_, x1, x2, x3, _, x5) if x1 & 2 == 2 && x2 == 0 && x3 == 0 && x5 == 0 => {
                            Some(Op::VcmlaA1Q)
                        }
                        (x0, x1, _, _, _, _) if x0 == 2 && x1 == 3 => None,
                        (x0, x1, _, _, _, _) if x0 == 3 && x1 == 3 => None,
                        _ => None,
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
                            Some(Op::VcmlaSA1Qs)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 0 && x5 == 1 =>
                        {
                            Some(Op::VfmslSA1Q)
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 0 && x2 == 0 && x3 == 1 => None,
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 0 => None,
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 0 =>
                        {
                            Some(Op::VdotSA1Q)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x5 == 1 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 0 =>
                        {
                            Some(Op::VdotSA1Q)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 0 && x5 == 0 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 0 && x4 == 0 && x5 == 1 =>
                        {
                            Some(Op::VfmslSA1Q)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 0 && x4 == 1 && x5 == 1 =>
                        {
                            Some(Op::VfmslSA1Q)
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 1 && x2 == 0 && x3 == 1 => None,
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 1 && x2 == 1 && x3 == 0 => None,
                        (x0, x1, x2, _, _, _) if x0 == 0 && x1 == 2 && x2 == 0 => None,
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 2 && x2 == 1 && x3 == 0 => None,
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 2 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 0 =>
                        {
                            Some(Op::VudotSA1Q)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 2 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 1 =>
                        {
                            Some(Op::VudotSA1Q)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 2 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 0 =>
                        {
                            Some(Op::VudotSA1Q)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 0 && x1 == 2 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 1 =>
                        {
                            Some(Op::VudotSA1Q)
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 0 && x1 == 3 && x2 == 0 && x3 == 0 && x5 == 0 =>
                        {
                            None
                        }
                        (x0, x1, x2, x3, _, x5)
                            if x0 == 0 && x1 == 3 && x2 == 0 && x3 == 0 && x5 == 1 =>
                        {
                            Some(Op::VfmaBfsA1Q)
                        }
                        (x0, x1, x2, x3, _, _) if x0 == 0 && x1 == 3 && x2 == 0 && x3 == 1 => None,
                        (x0, x1, x2, _, _, _) if x0 == 0 && x1 == 3 && x2 == 1 => None,
                        (x0, _, x2, x3, _, x5) if x0 == 1 && x2 == 0 && x3 == 0 && x5 == 0 => {
                            Some(Op::VcmlaSA1Qs)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 0 =>
                        {
                            Some(Op::VsudotSA1Q)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 0 && x5 == 1 =>
                        {
                            Some(Op::VsudotSA1Q)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 0 =>
                        {
                            Some(Op::VsudotSA1Q)
                        }
                        (x0, x1, x2, x3, x4, x5)
                            if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x4 == 1 && x5 == 1 =>
                        {
                            Some(Op::VsudotSA1Q)
                        }
                        (x0, _, x2, x3, _, _) if x0 == 1 && x2 == 0 && x3 == 1 => None,
                        (x0, x1, x2, x3, _, _) if x0 == 1 && x1 == 1 && x2 == 1 && x3 == 1 => None,
                        (x0, x1, x2, x3, _, _) if x0 == 1 && x1 & 2 == 2 && x2 == 1 && x3 == 1 => {
                            None
                        }
                        (x0, _, x2, x3, _, _) if x0 == 1 && x2 == 1 && x3 == 0 => None,
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, _, _) if x0 != 15 && x2 & 2 == 0 && x4 & 6 == 4 => {
                    match (
                        (instr >> 28) & 7,
                        (instr >> 25) & 5,
                        (instr >> 21) & 7,
                        (instr >> 12) & 17,
                        (instr >> 10) & 3,
                        instr & 19,
                    ) {
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
                                (x0, _, _, _, _) if x0 == 0 => None,
                                (x0, _, _, _, x4) if x0 == 1 && x4 == 0 => None,
                                (x0, _, x2, x3, x4)
                                    if x0 == 1 && x2 & 2 == 0 && x3 == 0 && x4 == 1 =>
                                {
                                    None
                                }
                                (x0, _, _, x3, _) if x0 == 1 && x3 == 1 => None,
                                (x0, x1, x2, x3, x4)
                                    if x0 == 1 && x1 == 0 && x2 == 2 && x3 == 0 && x4 == 1 =>
                                {
                                    Some(Op::VmovSsA1)
                                }
                                (x0, x1, x2, x3, x4)
                                    if x0 == 1 && x1 == 0 && x2 == 3 && x3 == 0 && x4 == 1 =>
                                {
                                    Some(Op::VmovDA1)
                                }
                                (x0, _, _, x3, _) if x0 == 1 && x3 & 2 == 2 => None,
                                (x0, x1, x2, x3, x4)
                                    if x0 == 1 && x1 == 1 && x2 == 2 && x3 == 0 && x4 == 1 =>
                                {
                                    Some(Op::VmovSsA1)
                                }
                                (x0, x1, x2, x3, x4)
                                    if x0 == 1 && x1 == 1 && x2 == 3 && x3 == 0 && x4 == 1 =>
                                {
                                    Some(Op::VmovDA1)
                                }
                                _ => None,
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
                                (x0, x1, x2, _, _, _, _) if x0 == 0 && x1 == 0 && x2 == 1 => None,
                                (x0, x1, _, _, _, x5, _) if x0 == 0 && x1 == 1 && x5 & 2 == 0 => {
                                    None
                                }
                                (x0, x1, _, x3, _, x5, _)
                                    if x0 == 0 && x1 == 1 && x3 == 0 && x5 == 2 =>
                                {
                                    Some(Op::VstmdbA2)
                                }
                                (x0, x1, _, x3, _, x5, x6)
                                    if x0 == 0 && x1 == 1 && x3 == 0 && x5 == 3 && x6 & 1 == 0 =>
                                {
                                    Some(Op::FstmdbxA1)
                                }
                                (x0, x1, _, x3, _, x5, x6)
                                    if x0 == 0 && x1 == 1 && x3 == 0 && x5 == 3 && x6 & 1 == 1 =>
                                {
                                    Some(Op::FstmdbxA1)
                                }
                                (x0, x1, _, x3, _, x5, _)
                                    if x0 == 0 && x1 == 1 && x3 == 1 && x5 == 2 =>
                                {
                                    Some(Op::VldmdbA2)
                                }
                                (x0, x1, _, x3, _, x5, x6)
                                    if x0 == 0 && x1 == 1 && x3 == 1 && x5 == 3 && x6 & 1 == 0 =>
                                {
                                    Some(Op::FldmdbxA1)
                                }
                                (x0, x1, _, x3, _, x5, x6)
                                    if x0 == 0 && x1 == 1 && x3 == 1 && x5 == 3 && x6 & 1 == 1 =>
                                {
                                    Some(Op::FldmdbxA1)
                                }
                                (x0, _, x2, x3, _, _, _) if x0 == 1 && x2 == 0 && x3 == 0 => {
                                    Some(Op::VstrA1D)
                                }
                                (x0, _, x2, _, _, x5, _) if x0 == 1 && x2 == 0 && x5 == 0 => None,
                                (x0, _, x2, x3, x4, _, _)
                                    if x0 == 1 && x2 == 0 && x3 == 1 && x4 != 15 =>
                                {
                                    Some(Op::VldrLA1D)
                                }
                                (x0, x1, x2, _, _, x5, _)
                                    if x0 == 1 && x1 == 0 && x2 == 1 && x5 & 2 == 0 =>
                                {
                                    None
                                }
                                (x0, x1, x2, x3, _, x5, _)
                                    if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 0 && x5 == 2 =>
                                {
                                    Some(Op::VstmdbA2)
                                }
                                (x0, x1, x2, x3, _, x5, x6)
                                    if x0 == 1
                                        && x1 == 0
                                        && x2 == 1
                                        && x3 == 0
                                        && x5 == 3
                                        && x6 & 1 == 0 =>
                                {
                                    Some(Op::FstmdbxA1)
                                }
                                (x0, x1, x2, x3, _, x5, x6)
                                    if x0 == 1
                                        && x1 == 0
                                        && x2 == 1
                                        && x3 == 0
                                        && x5 == 3
                                        && x6 & 1 == 1 =>
                                {
                                    Some(Op::FstmdbxA1)
                                }
                                (x0, x1, x2, x3, _, x5, _)
                                    if x0 == 1 && x1 == 0 && x2 == 1 && x3 == 1 && x5 == 2 =>
                                {
                                    Some(Op::VldmdbA2)
                                }
                                (x0, x1, x2, x3, _, x5, x6)
                                    if x0 == 1
                                        && x1 == 0
                                        && x2 == 1
                                        && x3 == 1
                                        && x5 == 3
                                        && x6 & 1 == 0 =>
                                {
                                    Some(Op::FldmdbxA1)
                                }
                                (x0, x1, x2, x3, _, x5, x6)
                                    if x0 == 1
                                        && x1 == 0
                                        && x2 == 1
                                        && x3 == 1
                                        && x5 == 3
                                        && x6 & 1 == 1 =>
                                {
                                    Some(Op::FldmdbxA1)
                                }
                                (x0, _, x2, x3, x4, _, _)
                                    if x0 == 1 && x2 == 0 && x3 == 1 && x4 == 15 =>
                                {
                                    Some(Op::VldrLA1D)
                                }
                                (x0, x1, x2, _, _, _, _) if x0 == 1 && x1 == 1 && x2 == 1 => None,
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                (x0, _, x2, _, x4, _, x6, _) if x0 != 15 && x2 == 2 && x4 & 6 == 4 && x6 == 1 => {
                    match (
                        (instr >> 28) & 7,
                        (instr >> 24) & 7,
                        (instr >> 21) & 5,
                        (instr >> 12) & 17,
                        (instr >> 10) & 3,
                        (instr >> 8) & 3,
                        (instr >> 5) & 5,
                        instr & 9,
                    ) {
                        (_, _, x2, _, _, x5, _, _) if x2 == 0 && x5 == 1 => {
                            let cond = (instr >> 28) & 7;
                            let op = (instr >> 20) & 1;
                            let Vn = (instr >> 16) & 7;
                            let Rt = (instr >> 12) & 7;
                            let N = (instr >> 7) & 1;
                            match () {
                                () => Some(Op::VmovHA1),
                            }
                        }
                        (_, _, x2, _, _, x5, _, _) if x2 == 0 && x5 == 2 => {
                            let cond = (instr >> 28) & 7;
                            let op = (instr >> 20) & 1;
                            let Vn = (instr >> 16) & 7;
                            let Rt = (instr >> 12) & 7;
                            let N = (instr >> 7) & 1;
                            match () {
                                () => Some(Op::VmovSA1),
                            }
                        }
                        (_, _, x2, _, _, x5, _, _) if x2 == 7 && x5 == 2 => {
                            let cond = (instr >> 28) & 7;
                            let L = (instr >> 20) & 1;
                            let reg = (instr >> 16) & 7;
                            let Rt = (instr >> 12) & 7;
                            match L {
                                x0 if x0 == 0 => Some(Op::VmsrA1As),
                                x0 if x0 == 1 => Some(Op::VmrsA1As),
                                _ => None,
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
                                (x0, x1, _) if x0 & 4 == 0 && x1 == 0 => Some(Op::VmovRsA1),
                                (_, x1, _) if x1 == 1 => Some(Op::VmovSrA1),
                                (x0, x1, x2) if x0 & 4 == 4 && x1 == 0 && x2 & 2 == 0 => {
                                    Some(Op::VdupRA1)
                                }
                                (x0, x1, x2) if x0 & 4 == 4 && x1 == 0 && x2 & 2 == 2 => None,
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                _ => None,
            }
        }
        (x0, x1, _, _, _) if x0 == 15 && x1 & 4 == 0 => {
            match (
                (instr >> 27) & 9,
                (instr >> 25) & 3,
                (instr >> 21) & 7,
                (instr >> 20) & 1,
                instr & 39,
            ) {
                (_, x1, _, _, _) if x1 == 0 => {
                    match (
                        (instr >> 25) & 13,
                        (instr >> 20) & 9,
                        (instr >> 8) & 23,
                        (instr >> 4) & 7,
                        instr & 7,
                    ) {
                        (_, x1, _, _, _) if x1 & 16 == 0 => None,
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
                                (_, _, x2, x3) if x2 == 1 && x3 & 16 == 0 => Some(Op::SetendA1),
                                (_, _, x2, _) if x2 == 0 => Some(Op::CpsidA1Asm),
                                (_, _, x2, x3) if x2 == 1 && x3 & 16 == 16 => None,
                                _ => None,
                            }
                        }
                        (_, x1, _, x3, _) if x1 == 17 && x3 == 8 => None,
                        (_, x1, _, x3, _) if x1 == 17 && x3 & 7 == 4 => None,
                        (_, x1, _, x3, _) if x1 == 17 && x3 & 3 == 1 => None,
                        (_, x1, _, x3, _) if x1 == 17 && x3 == 0 => {
                            let imm1 = (instr >> 9) & 1;
                            match () {
                                () => Some(Op::SetpanA1),
                            }
                        }
                        (_, x1, _, x3, _) if x1 & 30 == 16 && x3 == 7 => None,
                        (_, x1, _, x3, _) if x1 == 18 && x3 == 7 => None,
                        (_, x1, _, x3, _) if x1 == 19 && x3 == 7 => None,
                        (_, x1, _, x3, _) if x1 & 30 == 18 && x3 & 2 == 0 => None,
                        (_, x1, _, x3, _) if x1 & 28 == 16 && x3 == 3 => None,
                        (_, x1, _, x3, _) if x1 & 28 == 16 && x3 & 11 == 2 => None,
                        (_, x1, _, x3, _) if x1 & 28 == 16 && x3 & 10 == 10 => None,
                        (_, x1, _, _, _) if x1 & 28 == 20 => None,
                        (_, x1, _, _, _) if x1 & 24 == 24 => None,
                        _ => None,
                    }
                }
                (_, x1, _, _, _) if x1 == 1 => {
                    match (
                        (instr >> 25) & 13,
                        (instr >> 24) & 1,
                        (instr >> 23) & 1,
                        (instr >> 5) & 35,
                        (instr >> 4) & 1,
                        instr & 7,
                    ) {
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
                                (x0, x1, x2, _, x4)
                                    if x0 == 0 && x1 & 2 == 0 && x2 == 12 && x4 == 1 =>
                                {
                                    Some(Op::VfmsA1Q)
                                }
                                (x0, x1, x2, _, x4)
                                    if x0 == 0 && x1 & 2 == 0 && x2 == 13 && x4 == 0 =>
                                {
                                    Some(Op::VaddFA1Q)
                                }
                                (x0, x1, x2, _, x4)
                                    if x0 == 0 && x1 & 2 == 0 && x2 == 13 && x4 == 1 =>
                                {
                                    Some(Op::VmlsFA1Q)
                                }
                                (x0, x1, x2, _, x4)
                                    if x0 == 0 && x1 & 2 == 0 && x2 == 14 && x4 == 0 =>
                                {
                                    Some(Op::VceqRT1A1A)
                                }
                                (x0, x1, x2, _, x4)
                                    if x0 == 0 && x1 & 2 == 0 && x2 == 15 && x4 == 0 =>
                                {
                                    Some(Op::VminFA1Q)
                                }
                                (x0, x1, x2, _, x4)
                                    if x0 == 0 && x1 & 2 == 0 && x2 == 15 && x4 == 1 =>
                                {
                                    Some(Op::VrecpsA1Q)
                                }
                                (_, _, x2, _, x4) if x2 == 0 && x4 == 0 => Some(Op::VhsubA1Q),
                                (x0, x1, x2, _, x4) if x0 == 0 && x1 == 0 && x2 == 1 && x4 == 1 => {
                                    Some(Op::VandRA1Q)
                                }
                                (_, _, x2, _, x4) if x2 == 0 && x4 == 1 => Some(Op::VqaddA1Q),
                                (_, _, x2, _, x4) if x2 == 1 && x4 == 0 => Some(Op::VrhaddA1Q),
                                (x0, x1, x2, _, x4)
                                    if x0 == 0 && x1 == 0 && x2 == 12 && x4 == 0 =>
                                {
                                    Some(Op::Sha1CA1)
                                }
                                (_, _, x2, _, x4) if x2 == 2 && x4 == 0 => Some(Op::VhsubA1Q),
                                (x0, x1, x2, _, x4) if x0 == 0 && x1 == 1 && x2 == 1 && x4 == 1 => {
                                    Some(Op::VbicRA1Q)
                                }
                                (_, _, x2, _, x4) if x2 == 2 && x4 == 1 => Some(Op::VqsubA1Q),
                                (_, _, x2, _, x4) if x2 == 3 && x4 == 0 => Some(Op::VcgtRT1A1A),
                                (_, _, x2, _, x4) if x2 == 3 && x4 == 1 => Some(Op::VcgeRT1A1A),
                                (x0, x1, x2, _, x4)
                                    if x0 == 0 && x1 == 1 && x2 == 12 && x4 == 0 =>
                                {
                                    Some(Op::Sha1PA1)
                                }
                                (x0, x1, x2, _, x4)
                                    if x0 == 0 && x1 & 2 == 2 && x2 == 12 && x4 == 1 =>
                                {
                                    Some(Op::VfmsA1Q)
                                }
                                (x0, x1, x2, _, x4)
                                    if x0 == 0 && x1 & 2 == 2 && x2 == 13 && x4 == 0 =>
                                {
                                    Some(Op::VsubFA1Q)
                                }
                                (x0, x1, x2, _, x4)
                                    if x0 == 0 && x1 & 2 == 2 && x2 == 13 && x4 == 1 =>
                                {
                                    Some(Op::VmlsFA1Q)
                                }
                                (x0, x1, x2, _, x4)
                                    if x0 == 0 && x1 & 2 == 2 && x2 == 14 && x4 == 0 =>
                                {
                                    None
                                }
                                (x0, x1, x2, _, x4)
                                    if x0 == 0 && x1 & 2 == 2 && x2 == 15 && x4 == 0 =>
                                {
                                    Some(Op::VminFA1Q)
                                }
                                (x0, x1, x2, _, x4)
                                    if x0 == 0 && x1 & 2 == 2 && x2 == 15 && x4 == 1 =>
                                {
                                    Some(Op::VrsqrtsA1Q)
                                }
                                (_, _, x2, _, x4) if x2 == 4 && x4 == 0 => Some(Op::VshlRA1Q),
                                (x0, _, x2, _, x4) if x0 == 0 && x2 == 8 && x4 == 0 => {
                                    Some(Op::VaddIA1Q)
                                }
                                (x0, x1, x2, _, x4) if x0 == 0 && x1 == 2 && x2 == 1 && x4 == 1 => {
                                    Some(Op::VorrRA1Q)
                                }
                                (x0, _, x2, _, x4) if x0 == 0 && x2 == 8 && x4 == 1 => {
                                    Some(Op::VtstA1Q)
                                }
                                (_, _, x2, _, x4) if x2 == 4 && x4 == 1 => Some(Op::VqshlRA1Q),
                                (x0, _, x2, _, x4) if x0 == 0 && x2 == 9 && x4 == 0 => {
                                    Some(Op::VmlsIA1Q)
                                }
                                (_, _, x2, _, x4) if x2 == 5 && x4 == 0 => Some(Op::VrshlA1Q),
                                (_, _, x2, _, x4) if x2 == 5 && x4 == 1 => Some(Op::VqrshlA1Q),
                                (x0, _, x2, _, x4) if x0 == 0 && x2 == 11 && x4 == 0 => {
                                    Some(Op::VqdmulhA1Q)
                                }
                                (x0, x1, x2, _, x4)
                                    if x0 == 0 && x1 == 2 && x2 == 12 && x4 == 0 =>
                                {
                                    Some(Op::Sha1MA1)
                                }
                                (x0, _, x2, _, x4) if x0 == 0 && x2 == 11 && x4 == 1 => {
                                    Some(Op::VpaddIA1)
                                }
                                (_, _, x2, _, x4) if x2 == 6 && x4 == 0 => Some(Op::VminIA1Q),
                                (x0, x1, x2, _, x4) if x0 == 0 && x1 == 3 && x2 == 1 && x4 == 1 => {
                                    Some(Op::VornRA1Q)
                                }
                                (_, _, x2, _, x4) if x2 == 6 && x4 == 1 => Some(Op::VminIA1Q),
                                (_, _, x2, _, x4) if x2 == 7 && x4 == 0 => Some(Op::VabdIA1Q),
                                (_, _, x2, _, x4) if x2 == 7 && x4 == 1 => Some(Op::VabaA1Q),
                                (x0, x1, x2, _, x4)
                                    if x0 == 0 && x1 == 3 && x2 == 12 && x4 == 0 =>
                                {
                                    Some(Op::Sha1Su0A1)
                                }
                                (x0, x1, x2, _, x4)
                                    if x0 == 1 && x1 & 2 == 0 && x2 == 13 && x4 == 0 =>
                                {
                                    Some(Op::VpaddFA1)
                                }
                                (x0, x1, x2, _, x4)
                                    if x0 == 1 && x1 & 2 == 0 && x2 == 13 && x4 == 1 =>
                                {
                                    Some(Op::VmulFA1Q)
                                }
                                (x0, x1, x2, _, x4)
                                    if x0 == 1 && x1 & 2 == 0 && x2 == 14 && x4 == 0 =>
                                {
                                    Some(Op::VcgeRT1A1A)
                                }
                                (x0, x1, x2, _, x4)
                                    if x0 == 1 && x1 & 2 == 0 && x2 == 14 && x4 == 1 =>
                                {
                                    Some(Op::VacgtA1Q)
                                }
                                (x0, x1, x2, x3, x4)
                                    if x0 == 1 && x1 & 2 == 0 && x2 == 15 && x3 == 0 && x4 == 0 =>
                                {
                                    Some(Op::VpminFA1)
                                }
                                (x0, x1, x2, _, x4)
                                    if x0 == 1 && x1 & 2 == 0 && x2 == 15 && x4 == 1 =>
                                {
                                    Some(Op::VminnmA1Q)
                                }
                                (x0, x1, x2, _, x4) if x0 == 1 && x1 == 0 && x2 == 1 && x4 == 1 => {
                                    Some(Op::VeorA1Q)
                                }
                                (_, _, x2, _, x4) if x2 == 9 && x4 == 1 => Some(Op::VmulIA1Q),
                                (x0, x1, x2, _, x4)
                                    if x0 == 1 && x1 == 0 && x2 == 12 && x4 == 0 =>
                                {
                                    Some(Op::Sha256HA1)
                                }
                                (_, _, x2, x3, x4) if x2 == 10 && x3 == 0 && x4 == 0 => {
                                    Some(Op::VpminIA1)
                                }
                                (x0, x1, x2, _, x4) if x0 == 1 && x1 == 1 && x2 == 1 && x4 == 1 => {
                                    Some(Op::VbifA1Q)
                                }
                                (_, _, x2, x3, x4) if x2 == 10 && x3 == 0 && x4 == 1 => {
                                    Some(Op::VpminIA1)
                                }
                                (_, _, x2, x3, _) if x2 == 10 && x3 == 1 => None,
                                (x0, x1, x2, _, x4)
                                    if x0 == 1 && x1 == 1 && x2 == 12 && x4 == 0 =>
                                {
                                    Some(Op::Sha256H2A1)
                                }
                                (x0, x1, x2, _, x4)
                                    if x0 == 1 && x1 & 2 == 2 && x2 == 13 && x4 == 0 =>
                                {
                                    Some(Op::VabdFA1Q)
                                }
                                (x0, x1, x2, _, x4)
                                    if x0 == 1 && x1 & 2 == 2 && x2 == 14 && x4 == 0 =>
                                {
                                    Some(Op::VcgtRT1A1A)
                                }
                                (x0, x1, x2, _, x4)
                                    if x0 == 1 && x1 & 2 == 2 && x2 == 14 && x4 == 1 =>
                                {
                                    Some(Op::VacgtA1Q)
                                }
                                (x0, x1, x2, x3, x4)
                                    if x0 == 1 && x1 & 2 == 2 && x2 == 15 && x3 == 0 && x4 == 0 =>
                                {
                                    Some(Op::VpminFA1)
                                }
                                (x0, x1, x2, _, x4)
                                    if x0 == 1 && x1 & 2 == 2 && x2 == 15 && x4 == 1 =>
                                {
                                    Some(Op::VminnmA1Q)
                                }
                                (x0, _, x2, _, x4) if x0 == 1 && x2 == 8 && x4 == 0 => {
                                    Some(Op::VsubIA1Q)
                                }
                                (x0, x1, x2, _, x4) if x0 == 1 && x1 == 2 && x2 == 1 && x4 == 1 => {
                                    Some(Op::VbifA1Q)
                                }
                                (x0, _, x2, _, x4) if x0 == 1 && x2 == 8 && x4 == 1 => {
                                    Some(Op::VceqRT1A1A)
                                }
                                (x0, _, x2, _, x4) if x0 == 1 && x2 == 9 && x4 == 0 => {
                                    Some(Op::VmlsIA1Q)
                                }
                                (x0, _, x2, _, x4) if x0 == 1 && x2 == 11 && x4 == 0 => {
                                    Some(Op::VqrdmulhA1Q)
                                }
                                (x0, x1, x2, _, x4)
                                    if x0 == 1 && x1 == 2 && x2 == 12 && x4 == 0 =>
                                {
                                    Some(Op::Sha256Su1A1)
                                }
                                (x0, _, x2, _, x4) if x0 == 1 && x2 == 11 && x4 == 1 => {
                                    Some(Op::VqrdmlahA1Q)
                                }
                                (x0, x1, x2, _, x4) if x0 == 1 && x1 == 3 && x2 == 1 && x4 == 1 => {
                                    Some(Op::VbifA1Q)
                                }
                                (x0, _, x2, _, x4) if x0 == 1 && x2 == 12 && x4 == 1 => {
                                    Some(Op::VqrdmlshA1Q)
                                }
                                (x0, _, x2, x3, x4)
                                    if x0 == 1 && x2 == 15 && x3 == 1 && x4 == 0 =>
                                {
                                    None
                                }
                                _ => None,
                            }
                        }
                        (_, _, x2, _, x4, _) if x2 == 1 && x4 == 0 => {
                            match (
                                (instr >> 25) & 13,
                                (instr >> 24) & 1,
                                (instr >> 23) & 1,
                                (instr >> 22) & 1,
                                (instr >> 20) & 3,
                                (instr >> 12) & 15,
                                (instr >> 10) & 3,
                                (instr >> 7) & 5,
                                (instr >> 6) & 1,
                                (instr >> 5) & 1,
                                (instr >> 4) & 1,
                                instr & 7,
                            ) {
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
                                        () => Some(Op::VextA1Q),
                                    }
                                }
                                (_, x1, _, _, x4, _, x6, _, _, _, _, _)
                                    if x1 == 1 && x4 == 3 && x6 & 2 == 0 =>
                                {
                                    let D = (instr >> 22) & 1;
                                    let size = (instr >> 18) & 3;
                                    let opc1 = (instr >> 16) & 3;
                                    let Vd = (instr >> 12) & 7;
                                    let opc2 = (instr >> 7) & 7;
                                    let Q = (instr >> 6) & 1;
                                    let M = (instr >> 5) & 1;
                                    let Vm = instr & 7;
                                    match (size, opc1, opc2, Q) {
                                        (_, x1, x2, _) if x1 == 0 && x2 == 0 => Some(Op::Vrev16A1Q),
                                        (_, x1, x2, _) if x1 == 0 && x2 == 1 => Some(Op::Vrev16A1Q),
                                        (_, x1, x2, _) if x1 == 0 && x2 == 2 => Some(Op::Vrev16A1Q),
                                        (_, x1, x2, _) if x1 == 0 && x2 == 3 => None,
                                        (_, x1, x2, _) if x1 == 0 && x2 & 14 == 4 => {
                                            Some(Op::VpaddlA1Q)
                                        }
                                        (_, x1, x2, x3) if x1 == 0 && x2 == 6 && x3 == 0 => {
                                            Some(Op::AeseA1)
                                        }
                                        (_, x1, x2, x3) if x1 == 0 && x2 == 6 && x3 == 1 => {
                                            Some(Op::AesdA1)
                                        }
                                        (_, x1, x2, x3) if x1 == 0 && x2 == 7 && x3 == 0 => {
                                            Some(Op::AesmcA1)
                                        }
                                        (_, x1, x2, x3) if x1 == 0 && x2 == 7 && x3 == 1 => {
                                            Some(Op::AesimcA1)
                                        }
                                        (_, x1, x2, _) if x1 == 0 && x2 == 8 => Some(Op::VclsA1Q),
                                        (x0, x1, x2, _) if x0 == 0 && x1 == 2 && x2 == 0 => {
                                            Some(Op::VswpA1Q)
                                        }
                                        (_, x1, x2, _) if x1 == 0 && x2 == 9 => Some(Op::VclzA1Q),
                                        (_, x1, x2, _) if x1 == 0 && x2 == 10 => Some(Op::VcntA1Q),
                                        (_, x1, x2, _) if x1 == 0 && x2 == 11 => Some(Op::VmvnRA1Q),
                                        (x0, x1, x2, x3)
                                            if x0 == 0 && x1 == 2 && x2 == 12 && x3 == 1 =>
                                        {
                                            None
                                        }
                                        (_, x1, x2, _) if x1 == 0 && x2 & 14 == 12 => {
                                            Some(Op::VpadalA1Q)
                                        }
                                        (_, x1, x2, _) if x1 == 0 && x2 == 14 => Some(Op::VqabsA1Q),
                                        (_, x1, x2, _) if x1 == 0 && x2 == 15 => Some(Op::VqnegA1Q),
                                        (_, x1, x2, _) if x1 == 1 && x2 & 7 == 0 => {
                                            Some(Op::VcgtIA1Q)
                                        }
                                        (_, x1, x2, _) if x1 == 1 && x2 & 7 == 1 => {
                                            Some(Op::VcgeIA1Q)
                                        }
                                        (_, x1, x2, _) if x1 == 1 && x2 & 7 == 2 => {
                                            Some(Op::VceqIA1Q)
                                        }
                                        (_, x1, x2, _) if x1 == 1 && x2 & 7 == 3 => {
                                            Some(Op::VcleIA1Q)
                                        }
                                        (_, x1, x2, _) if x1 == 1 && x2 & 7 == 4 => {
                                            Some(Op::VcltIA1Q)
                                        }
                                        (_, x1, x2, _) if x1 == 1 && x2 & 7 == 6 => {
                                            Some(Op::VabsA1Q)
                                        }
                                        (_, x1, x2, _) if x1 == 1 && x2 & 7 == 7 => {
                                            Some(Op::VnegA1Q)
                                        }
                                        (_, x1, x2, x3) if x1 == 1 && x2 == 5 && x3 == 1 => {
                                            Some(Op::Sha1HA1)
                                        }
                                        (x0, x1, x2, x3)
                                            if x0 == 1 && x1 == 2 && x2 == 12 && x3 == 1 =>
                                        {
                                            Some(Op::VcvtBfsA1)
                                        }
                                        (_, x1, x2, _) if x1 == 2 && x2 == 1 => Some(Op::VtrnA1Q),
                                        (_, x1, x2, _) if x1 == 2 && x2 == 2 => Some(Op::VuzpA1Q),
                                        (_, x1, x2, _) if x1 == 2 && x2 == 3 => Some(Op::VzipA1Q),
                                        (_, x1, x2, x3) if x1 == 2 && x2 == 4 && x3 == 0 => {
                                            Some(Op::VmovnA1)
                                        }
                                        (_, x1, x2, x3) if x1 == 2 && x2 == 4 && x3 == 1 => {
                                            Some(Op::VqmovnA1)
                                        }
                                        (_, x1, x2, _) if x1 == 2 && x2 == 5 => Some(Op::VqmovnA1),
                                        (_, x1, x2, x3) if x1 == 2 && x2 == 6 && x3 == 0 => {
                                            Some(Op::VshllA2)
                                        }
                                        (_, x1, x2, x3) if x1 == 2 && x2 == 7 && x3 == 0 => {
                                            Some(Op::Sha1Su1A1)
                                        }
                                        (_, x1, x2, x3) if x1 == 2 && x2 == 7 && x3 == 1 => {
                                            Some(Op::Sha256Su0A1)
                                        }
                                        (_, x1, x2, _) if x1 == 2 && x2 == 8 => {
                                            Some(Op::VrintpAsimdA1Q)
                                        }
                                        (_, x1, x2, _) if x1 == 2 && x2 == 9 => {
                                            Some(Op::VrintxAsimdA1Q)
                                        }
                                        (_, x1, x2, _) if x1 == 2 && x2 == 10 => {
                                            Some(Op::VrintpAsimdA1Q)
                                        }
                                        (_, x1, x2, _) if x1 == 2 && x2 == 11 => {
                                            Some(Op::VrintzAsimdA1Q)
                                        }
                                        (x0, x1, x2, x3)
                                            if x0 == 2 && x1 == 2 && x2 == 12 && x3 == 1 =>
                                        {
                                            None
                                        }
                                        (_, x1, x2, x3) if x1 == 2 && x2 == 12 && x3 == 0 => {
                                            Some(Op::VcvtShA1)
                                        }
                                        (_, x1, x2, _) if x1 == 2 && x2 == 13 => {
                                            Some(Op::VrintpAsimdA1Q)
                                        }
                                        (_, x1, x2, x3) if x1 == 2 && x2 == 14 && x3 == 0 => {
                                            Some(Op::VcvtShA1)
                                        }
                                        (_, x1, x2, x3) if x1 == 2 && x2 == 14 && x3 == 1 => None,
                                        (_, x1, x2, _) if x1 == 2 && x2 == 15 => {
                                            Some(Op::VrintpAsimdA1Q)
                                        }
                                        (_, x1, x2, _) if x1 == 3 && x2 & 14 == 0 => {
                                            Some(Op::VcvtmAsimdA1Q)
                                        }
                                        (_, x1, x2, _) if x1 == 3 && x2 & 14 == 2 => {
                                            Some(Op::VcvtmAsimdA1Q)
                                        }
                                        (_, x1, x2, _) if x1 == 3 && x2 & 14 == 4 => {
                                            Some(Op::VcvtmAsimdA1Q)
                                        }
                                        (_, x1, x2, _) if x1 == 3 && x2 & 14 == 6 => {
                                            Some(Op::VcvtmAsimdA1Q)
                                        }
                                        (_, x1, x2, _) if x1 == 3 && x2 & 13 == 8 => {
                                            Some(Op::VrecpeA1Q)
                                        }
                                        (_, x1, x2, _) if x1 == 3 && x2 & 13 == 9 => {
                                            Some(Op::VrsqrteA1Q)
                                        }
                                        (x0, x1, x2, x3)
                                            if x0 == 3 && x1 == 2 && x2 == 12 && x3 == 1 =>
                                        {
                                            None
                                        }
                                        (_, x1, x2, _) if x1 == 3 && x2 & 12 == 12 => {
                                            Some(Op::VcvtIsA1Q)
                                        }
                                        _ => None,
                                    }
                                }
                                (_, x1, _, _, x4, _, x6, _, _, _, _, _)
                                    if x1 == 1 && x4 == 3 && x6 == 2 =>
                                {
                                    let D = (instr >> 22) & 1;
                                    let Vn = (instr >> 16) & 7;
                                    let Vd = (instr >> 12) & 7;
                                    let len = (instr >> 8) & 3;
                                    let N = (instr >> 7) & 1;
                                    let op = (instr >> 6) & 1;
                                    let M = (instr >> 5) & 1;
                                    let Vm = instr & 7;
                                    match () {
                                        () => Some(Op::VtbxA1),
                                    }
                                }
                                (_, x1, _, _, x4, _, x6, _, _, _, _, _)
                                    if x1 == 1 && x4 == 3 && x6 == 3 =>
                                {
                                    let D = (instr >> 22) & 1;
                                    let imm4 = (instr >> 16) & 7;
                                    let Vd = (instr >> 12) & 7;
                                    let opc = (instr >> 7) & 5;
                                    let Q = (instr >> 6) & 1;
                                    let M = (instr >> 5) & 1;
                                    let Vm = instr & 7;
                                    match opc {
                                        x0 if x0 == 0 => Some(Op::VdupSA1Q),
                                        x0 if x0 == 1 => None,
                                        x0 if x0 & 6 == 2 => None,
                                        x0 if x0 & 4 == 4 => None,
                                        _ => None,
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
                                        (_, x1) if x1 == 0 => Some(Op::VaddwA1),
                                        (_, x1) if x1 == 1 => Some(Op::VaddwA1),
                                        (_, x1) if x1 == 2 => Some(Op::VsubwA1),
                                        (x0, x1) if x0 == 0 && x1 == 4 => Some(Op::VaddhnA1),
                                        (_, x1) if x1 == 3 => Some(Op::VsubwA1),
                                        (x0, x1) if x0 == 0 && x1 == 6 => Some(Op::VsubhnA1),
                                        (x0, x1) if x0 == 0 && x1 == 9 => Some(Op::VqdmlalA1),
                                        (_, x1) if x1 == 5 => Some(Op::VabalA1),
                                        (x0, x1) if x0 == 0 && x1 == 11 => Some(Op::VqdmlslA1),
                                        (x0, x1) if x0 == 0 && x1 == 13 => Some(Op::VqdmullA1),
                                        (_, x1) if x1 == 7 => Some(Op::VabdlIA1),
                                        (_, x1) if x1 == 8 => Some(Op::VmlslIA1),
                                        (_, x1) if x1 == 10 => Some(Op::VmlslIA1),
                                        (x0, x1) if x0 == 1 && x1 == 4 => Some(Op::VraddhnA1),
                                        (x0, x1) if x0 == 1 && x1 == 6 => Some(Op::VrsubhnA1),
                                        (_, x1) if x1 & 13 == 12 => Some(Op::VmullIA1),
                                        (x0, x1) if x0 == 1 && x1 == 9 => None,
                                        (x0, x1) if x0 == 1 && x1 == 11 => None,
                                        (x0, x1) if x0 == 1 && x1 == 13 => None,
                                        (_, x1) if x1 == 15 => None,
                                        _ => None,
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
                                        (_, x1) if x1 & 14 == 0 => Some(Op::VmlsSA1Q),
                                        (x0, x1) if x0 == 0 && x1 == 3 => Some(Op::VqdmlalA2),
                                        (_, x1) if x1 == 2 => Some(Op::VmlslSA1),
                                        (x0, x1) if x0 == 0 && x1 == 7 => Some(Op::VqdmlslA2),
                                        (_, x1) if x1 & 14 == 4 => Some(Op::VmlsSA1Q),
                                        (x0, x1) if x0 == 0 && x1 == 11 => Some(Op::VqdmullA2),
                                        (_, x1) if x1 == 6 => Some(Op::VmlslSA1),
                                        (_, x1) if x1 & 14 == 8 => Some(Op::VmulSA1Q),
                                        (x0, x1) if x0 == 1 && x1 == 3 => None,
                                        (_, x1) if x1 == 10 => Some(Op::VmullSA1),
                                        (x0, x1) if x0 == 1 && x1 == 7 => None,
                                        (_, x1) if x1 == 12 => Some(Op::VqdmulhA2Q),
                                        (_, x1) if x1 == 13 => Some(Op::VqrdmulhA2Q),
                                        (x0, x1) if x0 == 1 && x1 == 11 => None,
                                        (_, x1) if x1 == 14 => Some(Op::VqrdmlahA2Q),
                                        (_, x1) if x1 == 15 => Some(Op::VqrdmlshA2Q),
                                        _ => None,
                                    }
                                }
                                _ => None,
                            }
                        }
                        (_, _, x2, _, x4, _) if x2 == 1 && x4 == 1 => {
                            match (
                                (instr >> 25) & 13,
                                (instr >> 24) & 1,
                                (instr >> 23) & 1,
                                (instr >> 22) & 1,
                                (instr >> 7) & 29,
                                (instr >> 5) & 3,
                                (instr >> 4) & 1,
                                instr & 7,
                            ) {
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
                                        (x0, x1) if x0 & 9 == 0 && x1 == 0 => Some(Op::VmovIT1A1A),
                                        (x0, x1) if x0 & 9 == 0 && x1 == 1 => Some(Op::VmvnIT1A1A),
                                        (x0, x1) if x0 & 9 == 1 && x1 == 0 => Some(Op::VorrIT1A1A),
                                        (x0, x1) if x0 & 9 == 1 && x1 == 1 => Some(Op::VbicIT1A1A),
                                        (x0, x1) if x0 & 13 == 8 && x1 == 0 => Some(Op::VmovIT1A1A),
                                        (x0, x1) if x0 & 13 == 8 && x1 == 1 => Some(Op::VmvnIT1A1A),
                                        (x0, x1) if x0 & 13 == 9 && x1 == 0 => Some(Op::VorrIT1A1A),
                                        (x0, x1) if x0 & 13 == 9 && x1 == 1 => Some(Op::VbicIT1A1A),
                                        (x0, x1) if x0 & 12 == 12 && x1 == 0 => {
                                            Some(Op::VmovIT1A1A)
                                        }
                                        (x0, x1) if x0 & 14 == 12 && x1 == 1 => {
                                            Some(Op::VmvnIT1A1A)
                                        }
                                        (x0, x1) if x0 == 14 && x1 == 1 => Some(Op::VmovIT1A1A),
                                        (x0, x1) if x0 == 15 && x1 == 1 => None,
                                        _ => None,
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
                                            Some(Op::VshrA1Q)
                                        }
                                        (_, x1, _, x3, _) if x1 != 0 && x3 == 1 => {
                                            Some(Op::VsraA1Q)
                                        }
                                        (_, x1, x2, x3, x4)
                                            if x1 != 0 && x2 == 0 && x3 == 10 && x4 == 0 =>
                                        {
                                            Some(Op::VmovlA1)
                                        }
                                        (_, x1, _, x3, _) if x1 != 0 && x3 == 2 => {
                                            Some(Op::VrshrA1Q)
                                        }
                                        (_, x1, _, x3, _) if x1 != 0 && x3 == 3 => {
                                            Some(Op::VrsraA1D)
                                        }
                                        (_, x1, _, x3, _) if x1 != 0 && x3 == 7 => {
                                            Some(Op::VqshluIA1Q)
                                        }
                                        (_, x1, _, x3, x4) if x1 != 0 && x3 == 9 && x4 == 0 => {
                                            Some(Op::VqshrunA1)
                                        }
                                        (_, x1, _, x3, x4) if x1 != 0 && x3 == 9 && x4 == 1 => {
                                            Some(Op::VqrshrunA1)
                                        }
                                        (_, x1, _, x3, x4) if x1 != 0 && x3 == 10 && x4 == 0 => {
                                            Some(Op::VshllA1)
                                        }
                                        (_, x1, _, x3, _) if x1 != 0 && x3 & 12 == 12 => {
                                            Some(Op::VcvtXsA1Q)
                                        }
                                        (x0, x1, _, x3, _) if x0 == 0 && x1 != 0 && x3 == 5 => {
                                            Some(Op::VshlIA1Q)
                                        }
                                        (x0, x1, _, x3, x4)
                                            if x0 == 0 && x1 != 0 && x3 == 8 && x4 == 0 =>
                                        {
                                            Some(Op::VshrnA1)
                                        }
                                        (x0, x1, _, x3, x4)
                                            if x0 == 0 && x1 != 0 && x3 == 8 && x4 == 1 =>
                                        {
                                            Some(Op::VrshrnA1)
                                        }
                                        (x0, x1, _, x3, _) if x0 == 1 && x1 != 0 && x3 == 4 => {
                                            Some(Op::VsriA1Q)
                                        }
                                        (x0, x1, _, x3, _) if x0 == 1 && x1 != 0 && x3 == 5 => {
                                            Some(Op::VsliA1Q)
                                        }
                                        (x0, x1, _, x3, _) if x0 == 1 && x1 != 0 && x3 == 6 => {
                                            Some(Op::VqshluIA1Q)
                                        }
                                        (x0, x1, _, x3, x4)
                                            if x0 == 1 && x1 != 0 && x3 == 8 && x4 == 0 =>
                                        {
                                            Some(Op::VqshrunA1)
                                        }
                                        (x0, x1, _, x3, x4)
                                            if x0 == 1 && x1 != 0 && x3 == 8 && x4 == 1 =>
                                        {
                                            Some(Op::VqrshrunA1)
                                        }
                                        _ => None,
                                    }
                                }
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                (_, x1, _, x3, _) if x1 & 2 == 2 && x3 == 1 => {
                    match (
                        (instr >> 26) & 11,
                        (instr >> 21) & 9,
                        (instr >> 20) & 1,
                        (instr >> 5) & 29,
                        (instr >> 4) & 1,
                        instr & 7,
                    ) {
                        (_, x1, _, _, _, _) if x1 & 25 == 1 => None,
                        (_, x1, _, _, _, _) if x1 == 9 => None,
                        (_, x1, _, _, _, _) if x1 == 11 => {
                            let opcode = (instr >> 4) & 7;
                            let option = instr & 7;
                            match (opcode, option) {
                                (x0, _) if x0 == 0 => None,
                                (x0, _) if x0 == 1 => Some(Op::ClrexA1),
                                (x0, _) if x0 & 14 == 2 => None,
                                (x0, x1) if x0 == 4 && x1 & 11 != 0 => Some(Op::DsbA1),
                                (x0, x1) if x0 == 4 && x1 == 0 => Some(Op::SsbbA1),
                                (x0, x1) if x0 == 4 && x1 == 4 => Some(Op::PssbbA1),
                                (x0, _) if x0 == 5 => Some(Op::DmbA1),
                                (x0, _) if x0 == 6 => Some(Op::IsbA1),
                                (x0, _) if x0 == 7 => Some(Op::SbA1),
                                (x0, _) if x0 & 8 == 8 => None,
                                _ => None,
                            }
                        }
                        (_, x1, _, _, _, _) if x1 & 29 == 13 => None,
                        (_, x1, _, _, _, _) if x1 & 17 == 0 => {
                            let D = (instr >> 24) & 1;
                            let U = (instr >> 23) & 1;
                            let R = (instr >> 22) & 1;
                            let Rn = (instr >> 16) & 7;
                            let imm12 = instr & 23;
                            match (D, R, Rn) {
                                (x0, x1, _) if x0 == 0 && x1 == 0 => Some(Op::Nop),
                                (x0, x1, _) if x0 == 0 && x1 == 1 => Some(Op::PliIA1),
                                (x0, _, x2) if x0 == 1 && x2 == 15 => Some(Op::PldLA1),
                                (x0, x1, x2) if x0 == 1 && x1 == 0 && x2 != 15 => Some(Op::PldIA1),
                                (x0, x1, x2) if x0 == 1 && x1 == 1 && x2 != 15 => Some(Op::PldIA1),
                                _ => None,
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
                                (x0, x1) if x0 == 0 && x1 == 0 => Some(Op::Nop),
                                (x0, x1) if x0 == 0 && x1 == 1 => Some(Op::PliRA1Rrx),
                                (x0, x1) if x0 == 1 && x1 == 0 => Some(Op::PldRA1Rrx),
                                (x0, x1) if x0 == 1 && x1 == 1 => Some(Op::PldRA1Rrx),
                                _ => None,
                            }
                        }
                        (_, x1, _, _, x4, _) if x1 & 17 == 17 && x4 == 0 => None,
                        (_, x1, _, _, x4, _) if x1 & 16 == 16 && x4 == 1 => None,
                        _ => None,
                    }
                }
                (_, x1, _, x3, _) if x1 == 2 && x3 == 0 => {
                    match (
                        (instr >> 24) & 15,
                        (instr >> 23) & 1,
                        (instr >> 21) & 3,
                        (instr >> 20) & 1,
                        (instr >> 12) & 15,
                        (instr >> 10) & 3,
                        instr & 19,
                    ) {
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
                                (x0, x1) if x0 == 0 && x1 & 14 == 0 => Some(Op::Vst4MA1Posti),
                                (x0, x1) if x0 == 0 && x1 == 2 => Some(Op::Vst1MT1A1A),
                                (x0, x1) if x0 == 0 && x1 == 3 => Some(Op::Vst2MT1A1A),
                                (x0, x1) if x0 == 0 && x1 & 14 == 4 => Some(Op::Vst3MA1Posti),
                                (x0, x1) if x0 == 0 && x1 == 6 => Some(Op::Vst1MT1A1A),
                                (x0, x1) if x0 == 0 && x1 == 7 => Some(Op::Vst1MT1A1A),
                                (x0, x1) if x0 == 0 && x1 & 14 == 8 => Some(Op::Vst2MT1A1A),
                                (x0, x1) if x0 == 0 && x1 == 10 => Some(Op::Vst1MT1A1A),
                                (x0, x1) if x0 == 1 && x1 & 14 == 0 => Some(Op::Vld4MA1Posti),
                                (x0, x1) if x0 == 1 && x1 == 2 => Some(Op::Vld1MT1A1A),
                                (x0, x1) if x0 == 1 && x1 == 3 => Some(Op::Vld2MT1A1A),
                                (x0, x1) if x0 == 1 && x1 & 14 == 4 => Some(Op::Vld3MA1Posti),
                                (_, x1) if x1 == 11 => None,
                                (x0, x1) if x0 == 1 && x1 == 6 => Some(Op::Vld1MT1A1A),
                                (x0, x1) if x0 == 1 && x1 == 7 => Some(Op::Vld1MT1A1A),
                                (_, x1) if x1 & 12 == 12 => None,
                                (x0, x1) if x0 == 1 && x1 & 14 == 8 => Some(Op::Vld2MT1A1A),
                                (x0, x1) if x0 == 1 && x1 == 10 => Some(Op::Vld1MT1A1A),
                                _ => None,
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
                                (x0, _, _) if x0 == 0 => None,
                                (x0, x1, _) if x0 == 1 && x1 == 0 => Some(Op::Vld1AA1Posti),
                                (x0, x1, _) if x0 == 1 && x1 == 1 => Some(Op::Vld2AA1Posti),
                                (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 == 0 => {
                                    Some(Op::Vld3AA1Posti)
                                }
                                (x0, x1, x2) if x0 == 1 && x1 == 2 && x2 == 1 => None,
                                (x0, x1, _) if x0 == 1 && x1 == 3 => Some(Op::Vld4AA1Posti),
                                _ => None,
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
                                    Some(Op::Vst11T1A1A)
                                }
                                (x0, x1, x2, _) if x0 == 0 && x1 == 0 && x2 == 1 => {
                                    Some(Op::Vst21T1A1A)
                                }
                                (x0, x1, x2, _) if x0 == 0 && x1 == 0 && x2 == 2 => {
                                    Some(Op::Vst31T1A1A)
                                }
                                (x0, x1, x2, _) if x0 == 0 && x1 == 0 && x2 == 3 => {
                                    Some(Op::Vst41A1Posti)
                                }
                                (x0, x1, x2, _) if x0 == 0 && x1 == 1 && x2 == 0 => {
                                    Some(Op::Vst11T1A1A)
                                }
                                (x0, x1, x2, _) if x0 == 0 && x1 == 1 && x2 == 1 => {
                                    Some(Op::Vst21T1A1A)
                                }
                                (x0, x1, x2, _) if x0 == 0 && x1 == 1 && x2 == 2 => {
                                    Some(Op::Vst31T1A1A)
                                }
                                (x0, x1, x2, _) if x0 == 0 && x1 == 1 && x2 == 3 => {
                                    Some(Op::Vst41A2Posti)
                                }
                                (x0, x1, x2, _) if x0 == 0 && x1 == 2 && x2 == 0 => {
                                    Some(Op::Vst11T1A1A)
                                }
                                (x0, x1, x2, _) if x0 == 0 && x1 == 2 && x2 == 1 => {
                                    Some(Op::Vst21T1A1A)
                                }
                                (x0, x1, x2, _) if x0 == 0 && x1 == 2 && x2 == 2 => {
                                    Some(Op::Vst31T1A1A)
                                }
                                (x0, x1, x2, x3)
                                    if x0 == 0 && x1 == 2 && x2 == 3 && x3 & 13 != 13 =>
                                {
                                    Some(Op::Vst41A3Nowb)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 2 && x2 == 3 && x3 == 13 => {
                                    Some(Op::Vst41A3Nowb)
                                }
                                (x0, x1, x2, x3) if x0 == 0 && x1 == 2 && x2 == 3 && x3 == 15 => {
                                    Some(Op::Vst41A3Nowb)
                                }
                                (x0, x1, x2, _) if x0 == 1 && x1 == 0 && x2 == 0 => {
                                    Some(Op::Vld11T1A1A)
                                }
                                (x0, x1, x2, _) if x0 == 1 && x1 == 0 && x2 == 1 => {
                                    Some(Op::Vld21T1A1A)
                                }
                                (x0, x1, x2, _) if x0 == 1 && x1 == 0 && x2 == 2 => {
                                    Some(Op::Vld31T1A1A)
                                }
                                (x0, x1, x2, _) if x0 == 1 && x1 == 0 && x2 == 3 => {
                                    Some(Op::Vld41T1A1A)
                                }
                                (x0, x1, x2, _) if x0 == 1 && x1 == 1 && x2 == 0 => {
                                    Some(Op::Vld11T1A1A)
                                }
                                (x0, x1, x2, _) if x0 == 1 && x1 == 1 && x2 == 1 => {
                                    Some(Op::Vld21T1A1A)
                                }
                                (x0, x1, x2, _) if x0 == 1 && x1 == 1 && x2 == 2 => {
                                    Some(Op::Vld31T1A1A)
                                }
                                (x0, x1, x2, _) if x0 == 1 && x1 == 1 && x2 == 3 => {
                                    Some(Op::Vld41T1A1A)
                                }
                                (x0, x1, x2, _) if x0 == 1 && x1 == 2 && x2 == 0 => {
                                    Some(Op::Vld11T1A1A)
                                }
                                (x0, x1, x2, _) if x0 == 1 && x1 == 2 && x2 == 1 => {
                                    Some(Op::Vld21T1A1A)
                                }
                                (x0, x1, x2, _) if x0 == 1 && x1 == 2 && x2 == 2 => {
                                    Some(Op::Vld31T1A1A)
                                }
                                (x0, x1, x2, _) if x0 == 1 && x1 == 2 && x2 == 3 => {
                                    Some(Op::Vld41T1A1A)
                                }
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                (_, x1, _, x3, _) if x1 == 3 && x3 == 0 => None,
                _ => None,
            }
        }
        _ => None,
    }
} // end of decoding A32

#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    Sm3Tt1AVvv4Crypto3Imm2,
    UcvtfZPZX2Fp16,
    LsrZZw,
    UxtahA1,
    SbcsIA1,
    UdivrZPZz,
    VfmaBfsA1Q,
    BlIA1A,
    Ld1SwZPAiD,
    FnmsubHFloatdp3,
    Ldff1HZPBzD64Unscaled,
    VtrnA1Q,
    Ldff1ShZPAiD,
    FmlaZZzziH,
    Autdzb64ZDp1Src,
    Fcvtzu64HFloat2Int,
    AutiaspHiHints,
    SrZZi,
    Ldff1SwZPBzD64Scaled,
    Ldff1SbZPBrS32,
    Ld1RwZPBiU64,
    Ldnf1DZPBiU64,
    Eretab64EBranchReg,
    Ld1HZPBrU16,
    LdmEA1As,
    Ldff1BZPBzD64Unscaled,
    VzipA1Q,
    FcvtzuAsisdmiscfp16R,
    Ld1HZPBzDX32Scaled,
    McrrA1,
    Ldff1WZPBzD64Unscaled,
    FrintaZPZ,
    Ld4RAsisdlsoR4,
    UqdecwRRsX,
    Subps64SDp2Src,
    DechRRs,
    St4HZPBiContiguous,
    Ldff1ShZPBzSX32Unscaled,
    VqnegA1Q,
    StlbA1,
    Sadd16A1,
    SubrZPZz,
    Ldff1WZPBzDX32Unscaled,
    Ssat16A1,
    FcmltAsimdmiscFz,
    YieldA1,
    TblZZz1,
    St4HZPBrContiguous,
    UqinchZZs,
    FrsqrteAsisdmiscR,
    Ldff1WZPBzSX32Scaled,
    St2HZPBiContiguous,
    UqaddAsisdsameOnly,
    St4WZPBiContiguous,
    LslZPZw,
    St1BZPAiS,
    FcmeqPPZ0,
    VclzA1Q,
    Sha1HA1,
    SmlattA1,
    LdrsbRA1Pre,
    Pacga64PDp2Src,
    SsbbA1,
    FcmltPPZ0,
    UsdotZZzzS,
    UqincwZZs,
    Ldp64LdstpairOff,
    LslZZi,
    VqrdmlahA1Q,
    Ld2DZPBrContiguous,
    Ld1RodZPBrContiguous,
    FrsqrtsAsisdsameOnly,
    Sha512H2QqvCryptosha5123,
    VcmlaA1Q,
    InsrZV,
    VrshlA1Q,
    MlasA1,
    Ld1HZPBrU32,
    St3DZPBrContiguous,
    WhilelsPPRr,
    St1BZPBi,
    MpgePPZi,
    UrsraAsisdshfR,
    UabdZPZz,
    VcvtSivA1D,
    FdivHFloatdp2,
    VminnmA1Q,
    Sha512Su1Vvv2Cryptosha5123,
    DdplRRi,
    Ld1WZPBiU32,
    FnmulHFloatdp2,
    FacgtAsisdsameOnly,
    BrknPPPp,
    WrffrFP,
    Ld1RodZPBiU64,
    Bics64LogShift,
    HvcA1,
    MadZPZzz,
    FminZPZs,
    UdfOnlyPermUndef,
    VnegA2D,
    Sha1Su0A1,
    ScvtfZPZH2Fp16,
    BitZPZ,
    UhasxA1,
    Ld1SwZPBzD64Unscaled,
    XpaclriHiHints,
    Ldff1BZPBrU64,
    SubsIA1,
    QdaddA1,
    VinsA1,
    Ld2HZPBiContiguous,
    RsbsRA1Rrx,
    NegAsimdmiscR,
    LdaexA1,
    AxflagMPstate,
    Ldff1WZPBzSX32Unscaled,
    St2DZPBrContiguous,
    UqdecdRRsX,
    MpltPPZw,
    PrfhIPBzDX32Scaled,
    FmulxZPZz,
    MrsBrA1As,
    Stnt1DZPBiContiguous,
    LdabA1,
    Sm3Partw2Vvv4Cryptosha5123,
    BrkbPPP,
    Vld1MT1A1A,
    SqsubZZz,
    St1HZPBzD64Scaled,
    SqdmullAsisdelemL,
    VmulIA1Q,
    StlexA1,
    CmnRA1Rrx,
    PrfumPLdstUnscaled,
    Ld1BZPBzD64Unscaled,
    LdrhLA1,
    LsrZPZi,
    VldrLA1D,
    QsubA1,
    Sha1CQsvCryptosha3,
    FexpaZZ,
    SmmlaZZzz,
    FnegAsimdmiscfp16R,
    FrecpsZZz,
    SubsRA1Rrx,
    TermeqRr,
    LdrdLA1,
    MulZZi,
    SdotZZzz,
    Sxtb16A1,
    MsrIA1As,
    Ld1BZPBiU64,
    UminZPZz,
    DffrPF,
    EorsRrA1,
    Vst4MA1Posti,
    Vst41A2Posti,
    UmovAsimdinsXX,
    Ubfm64MBitfield,
    FcvtDhFloatdp1,
    LslZPZi,
    AesmcA1,
    VqdmlalA1,
    BrkpaPPPp,
    MsrRA1As,
    FminnmZPZs,
    Ldff1HZPBzSX32Scaled,
    Umulh64Dp3Src,
    Uzp2ZZz,
    Sha1PA1,
    MovsRrA1,
    St1DZPBi,
    SelZPZz,
    LdrhRA1Pre,
    FrintiAsimdmiscR,
    LastaVPZ,
    St3HZPBiContiguous,
    Ldff1WZPBrU64,
    FcvtzsZPZD2W,
    SqdecbRRsSx,
    Ld1AsisdlsepI2I2,
    RsubhnAsimddiffN,
    Ldnf1SbZPBiS32,
    NegZPZ,
    SxtbZPZ,
    Ldnf1ShZPBiS64,
    VsubwA1,
    Sha256H2A1,
    St1DZPBzD64Scaled,
    UxthZPZ,
    AdcsRrA1,
    VqdmlslA2,
    SqinchRRsX,
    FmlsZZzziS,
    Ldp64LdstpairPost,
    St1WZPAiD,
    VmovIA2D,
    Fcvtzu64HFloat2Fix,
    Sha1MA1,
    Zip2AsimdpermOnly,
    Ld1HZPBzSX32Scaled,
    UqdecpRPRX,
    FaddpAsisdpairOnlySd,
    SmlsldxA1,
    FmaxnmvVPZ,
    RevA1,
    LdrDLdstImmpost,
    SqdmlslAsisdelemL,
    VsudotSA1Q,
    SevlA1,
    St1DZPAiD,
    VcgeIA1Q,
    Uqadd16A1,
    NdsPPPpZ,
    VcvtXsA1Q,
    Ld4DZPBrContiguous,
    FmlsAsimdsamefp16Only,
    SqdmlslAsimdelemL,
    FcvtauAsimdmiscfp16R,
    MplePPZw,
    Rax1Vvv2Cryptosha5123,
    VaddFA1Q,
    VeorA1Q,
    SqrdmulhAsisdelemR,
    FabsZPZ,
    VmlsFA1Q,
    UrsraAsimdshfR,
    LdrsbIA1Pre,
    FnmsbZPZzz,
    Ld1RbZPBiU32,
    FaddaVPZ,
    LdaA1,
    StrIA1Pre,
    UqrshlAsisdsameOnly,
    Ld1RswZPBiS64,
    Sm4EkeyVvv4Cryptosha5123,
    SqxtunAsisdmiscN,
    Sha256Su0A1,
    UqincwRRsUw,
    VrshrnA1,
    Vld2AA1Posti,
    PyZOI,
    SmusdxA1,
    Ldnf1BZPBiU64,
    FmulxAsimdelemRhH,
    AddpAsimdsameOnly,
    Ldff1DZPBrU64,
    SqdechRRsSx,
    SsatA1Asr,
    FacgtPPZz,
    Stg64SoffsetLdsttags,
    FcvtzuAsimdmiscR,
    Ssub8A1,
    UqrshrnAsimdshfN,
    VstrA1D,
    St1WZPBzDX32Scaled,
    VaddwA1,
    St4WZPBrContiguous,
    FcvtzsZPZS2X,
    VshlIA1Q,
    PrfhIPBzSX32Scaled,
    St3WZPBiContiguous,
    Ldnf1BZPBiU8,
    FmovAsimdimmHH,
    St1HZPBi,
    Ldnf1ShZPBiS32,
    UrecpeAsimdmiscR,
    FmulZPZs,
    BfmmlaZZzz,
    SrZPZi,
    FcpyZPI,
    PrfhIPBrS,
    VqshlRA1Q,
    SrZZw,
    Uxtb16A1,
    Ldnf1HZPBiU64,
    FcvtzuAsimdmiscfp16R,
    Ldff1WZPBrU32,
    Ld1SbZPBzDX32Unscaled,
    UsqaddAsimdmiscR,
    ClrexBnBarriers,
    HltExException,
    SelA1,
    SubZZi,
    PrfdIPAiS,
    Ld1ShZPBzD64Unscaled,
    VrintxAsimdA1Q,
    VfmslA1Q,
    EorZZi,
    Pacdza64ZDp1Src,
    VmovHA1,
    VmovRA2D,
    DecwZZs,
    Ld1RshZPBiS64,
    Stnt1BZPBiContiguous,
    FabdAsimdsameOnly,
    PrfwIPBzD64Scaled,
    VmullIA1,
    Ldff1DZPBzD64Unscaled,
    SliAsisdshfR,
    FrintzZPZ,
    BfmlalbZZzz,
    Qsub16A1,
    FsubrZPZs,
    PnextPPP,
    PrfwIPBiS,
    SdotZZzziS,
    FcvtzuAsimdshfC,
    Ldff1DZPBzDX32Scaled,
    FmaxZPZz,
    IncdZZs,
    StlexdA1,
    SbcsRA1Rrx,
    Ldnt1DZPBiContiguous,
    Usad8A1,
    SdivrZPZz,
    FacgePPZz,
    UminpAsimdsameOnly,
    SrdZPZi,
    VqrdmulhA2Q,
    FdivAsimdsameOnly,
    NotAsimdmiscR,
    VqrdmlahA2Q,
    Trn1ZZzQ,
    FmulxAsimdelemRSd,
    Ld1AsisdlseR22V,
    VshllA2,
    EorPPPpZ,
    Subp64SDp2Src,
    SqincdZZs,
    FmlaZZzziS,
    Stgp64LdstpairPre,
    MvnsRrA1,
    EvZZ,
    BfcvtBsFloatdp1,
    SubAsisdsameOnly,
    SxtbA1,
    MsrSiPstate,
    AddvAsimdallOnly,
    FcselHFloatsel,
    Stnt1BZPBrContiguous,
    LdrPBi,
    LdrtA1,
    UrhaddAsimdsameOnly,
    Ldff1SbZPBrS16,
    MrsRsSystemmove,
    CmhsAsisdsameOnly,
    VceqRT1A1A,
    Nop,
    Ld1DZPBiU64,
    NandPPPpZ,
    AndsRA1Rrx,
    PrfhIPAiD,
    Ldff1SbZPBzSX32Unscaled,
    Ld4BZPBiContiguous,
    VqrdmulhA1Q,
    DdZZi,
    FcvtzuZPZS2W,
    IncwRRs,
    VudotA1Q,
    SqdecpRPRSx,
    SqrdmlshAsimdelemR,
    AutibspHiHints,
    UqincwRRsX,
    FcvtzuAsisdshfC,
    St1WZPBzSX32Scaled,
    VorrIT1A1A,
    BxjA1,
    PrfbIPBzSX32Scaled,
    LdrdRA1Pre,
    StrZBi,
    St1WZPBr,
    Ld1RqwZPBrContiguous,
    Subs64SAddsubImm,
    Ld3BZPBrContiguous,
    PrfhIPAiS,
    BfcA1,
    St1HZPAiD,
    Ldff1ShZPBzDX32Scaled,
    VextA1Q,
    VrintxVfpA1D,
    SubsRrA1,
    Ldff1HZPBzDX32Scaled,
    Trn1PPp,
    FcvtzuZPZFp162X,
    FminnmpAsimdsamefp16Only,
    FmulZZziH,
    VqrshrunA1,
    Ld1ShZPBzD64Scaled,
    Rev32AsimdmiscR,
    TermneRr,
    FmlsZZzziD,
    VaddFA2D,
    CmpIA1,
    Ldff1HZPAiD,
    Vld3MA1Posti,
    FrecpeAsisdmiscR,
    UqaddZZi,
    RmifOnlyRmif,
    AddsSpIA1,
    DecpZPZ,
    RfeibA1As,
    FsqrtAsimdmiscfp16R,
    FrintiAsimdmiscfp16R,
    FcmuoPPZz,
    Ld1WZPBzSX32Scaled,
    LdmibA1,
    FrintpZPZ,
    UqdecdRRsUw,
    CsdbA1,
    Setf16OnlySetf,
    IncbRRs,
    Ldff1WZPBzDX32Scaled,
    UxtbZPZ,
    FmlaZZzziD,
    UqdechRRsX,
    Ldnt1HZPBiContiguous,
    FcvtZPZD2S,
    SqdecpRPRX,
    Ldff1DZPBzDX32Unscaled,
    VsubIA1Q,
    UrsqrteAsimdmiscR,
    Ld2WZPBrContiguous,
    PmulAsimdsameOnly,
    IsbA1,
    UqsubZZz,
    SqinchZZs,
    FldmdbxA1,
    Ld1RqdZPBrContiguous,
    VqsubA1Q,
    VrsraA1D,
    BlxRA1,
    DrZAzDS32Scaled,
    Vst11T1A1A,
    MpeqPPZz,
    UcvtfZPZX2D,
    Stnt1DZPBrContiguous,
    LdmdaA1,
    SriAsimdshfR,
    MphiPPZi,
    MphiPPZw,
    FsubHFloatdp2,
    SetpanA1,
    VmovxA1,
    LdaexhA1,
    DupmZI,
    Xpacd64ZDp1Src,
    SmaxZZi,
    FccmpeHFloatccmp,
    UdotAsimdsame2D,
    MrsA1As,
    BfiA1,
    FnegAsimdmiscR,
    Ld1RhZPBiU32,
    VbifA1Q,
    Ldnf1HZPBiU32,
    PyZPV,
    UdotZZzziS,
    UqdecbRRsUw,
    Cls64Dp1Src,
    IndexZRi,
    St1HZPBzSX32Unscaled,
    UqdecdZZs,
    UxtbA1,
    UcvtfZPZH2Fp16,
    SbOnlyBarriers,
    OrrPPPpZ,
    SmaxZPZz,
    FcmlePPZ0,
    UasxA1,
    SmlalsA1,
    StrhtA2,
    BlOnlyBranchImm,
    VmmlaA1Q,
    Ld1HZPBiU16,
    SrsibA1As,
    Sha256Su1A1,
    VmovDA1,
    FrintmZPZ,
    VsubFA2D,
    MpnePPZz,
    LdmUA1As,
    UxthA1,
    St1DZPBzDX32Unscaled,
    Ldff1ShZPAiS,
    SqinchRRsSx,
    LdcLA1,
    VusmmlaA1Q,
    Ldff1BZPAiS,
    BicsRA1Rrx,
    Ld3WZPBrContiguous,
    TsbA1,
    Ld4DZPBiContiguous,
    St1WZPBzD64Scaled,
    FcmlaAsimdelemCS,
    CmpRrA1,
    Ld1HZPBzD64Unscaled,
    Ld1RohZPBiU16,
    Ldff1SwZPAiD,
    Ld1RbZPBiU8,
    Stzg64SpostLdsttags,
    SqdecdRRsX,
    VrhaddA1Q,
    Uxtab16A1,
    IndexZIr,
    SrZPZw,
    St1BZPBr,
    FrecpeAsimdmiscR,
    UcvtfZPZX2S,
    Ld3DZPBrContiguous,
    AdcsIA1,
    VqdmullA2,
    SxtabA1,
    PrfbIPAiS,
    St1WZPBzD64Unscaled,
    WfeA1,
    VandRA1Q,
    CmltAsimdmiscZ,
    SqrshrunAsimdshfN,
    StmUA1As,
    EorsIA1,
    NtZPZ,
    UqincpRPRX,
    SdivZPZz,
    Rev64Dp1Src,
    Sha1HSsCryptosha2,
    MplePPZi,
    FcvtauAsisdmiscfp16R,
    Sadd8A1,
    SqrdmulhAsisdsameOnly,
    FcmleAsimdmiscFz,
    Autdza64ZDp1Src,
    SqxtunAsimdmiscN,
    Ld1BZPAiD,
    Stg64SpreLdsttags,
    ShllAsimdmiscS,
    MphsPPZw,
    MovIA2,
    UdfA1,
    FcvtzsZPZFp162H,
    Lduminal64Memop,
    FrecpeAsimdmiscfp16R,
    SmlaldxA1,
    Ld1HZPBzD64Scaled,
    FcvtzuZPZD2W,
    TbxAsimdtblL44,
    Ldff1ShZPBrS32,
    Ldff1BZPAiD,
    BfmlalbZZzzi,
    UmaalA1,
    VhsubA1Q,
    BsZPZ,
    SubrZZi,
    LslrZPZz,
    Stz2G64SpreLdsttags,
    VfmslSA1Q,
    LdrshtA1,
    UminZZi,
    StrexA1,
    FmlsAsimdsameOnly,
    PrfdIPBzDX32Scaled,
    Ld1WZPBrU32,
    FmovHFloatimm,
    UcvtfZPZW2S,
    UdotZZzz,
    Shadd8A1,
    BfcvtntZPZS2Bf,
    PrfdIPBrS,
    VsubFA1Q,
    StrexhA1,
    Sha1Su1A1,
    DmbA1,
    PrfdIPAiD,
    VrsqrtsA1Q,
    St1HZPBzDX32Scaled,
    FcvtzuZPZFp162W,
    FcvtnAsimdmiscN,
    FcvtzuZPZFp162H,
    WfiA1,
    BcaxVvv16Crypto4,
    SqrdmlshAsimdsame2Only,
    VqdmulhA1Q,
    FrecpsAsimdsameOnly,
    AddsSpRA1Rrx,
    UqdechRRsUw,
    UqincdZZs,
    BrkpbPPPp,
    UmullsA1,
    Ld1WZPBrU64,
    FmovAsimdimmD2D,
    FnmlaZPZzz,
    SvcExException,
    UminAsimdsameOnly,
    DecwRRs,
    Sha1Su1VvCryptosha2,
    QdsubA1,
    StrtA1,
    VrsqrteA1Q,
    VrecpeA1Q,
    FmulxAsisdsamefp16Only,
    StmdbA1,
    UdotAsimdelemD,
    DrZAzSdSameScaled,
    Cbnz64Compbranch,
    UshllAsimdshfL,
    Ldgm64BulkLdsttags,
    EsbA1,
    ClzA1,
    ShlAsimdshfR,
    ClzAsimdmiscR,
    IndexZRr,
    St2BZPBiContiguous,
    Frint64XAsimdmiscR,
    PrfmPLdstRegoff,
    Fmlsl2AsimdelemLh,
    Qsub8A1,
    Fmlsl2AsimdsameF,
    FrecpeZZ,
    FminvVPZ,
    EorsPPPpZ,
    UdivZPZz,
    SqincdRRsX,
    Stgm64BulkLdsttags,
    UmlalsA1,
    Subg64AddsubImmtags,
    UxtwZPZ,
    AdrA1A,
    VqrdmlshA2Q,
    VselgtA1D,
    SminZPZz,
    TbnzOnlyTestbranch,
    FcmpeHzFloatcmp,
    LsrZZi,
    VcmlaSA1Qs,
    SpliceZPZzDes,
    LdrshIA1Pre,
    BrkpbsPPPp,
    UqshlAsimdshfR,
    Ld1SbZPBrS32,
    Ld1SbZPBrS64,
    FrintiHFloatdp1,
    VqdmullA1,
    MvnsIA1,
    FmulxAsimdsameOnly,
    FminZPZz,
    VshrA1Q,
    DecbRRs,
    Gmi64GDp2Src,
    SqrshrunAsisdshfN,
    Ld1HZPBiU32,
    FminvAsimdallOnlySd,
    FsqrtZPZ,
    UxtabA1,
    LdaexdA1,
    InsAsimdinsIrR,
    VmlslSA1,
    UdotZZzziD,
    MlaZPZzz,
    DupZI,
    FcmltAsisdmiscfp16Fz,
    FcmleAsisdmiscFz,
    Ld1RbZPBiU64,
    VcntA1Q,
    VmlslIA1,
    AddsRA1Rrx,
    Ldff1HZPBzD64Scaled,
    IncwZZs,
    UqincpRPRUw,
    MvnsRA1Rrx,
    FcmltAsimdmiscfp16Fz,
    Ld1RobZPBrContiguous,
    UsdotAsimdelemD,
    VaddIA1Q,
    Ld1BZPBrU64,
    Ld1RhZPBiU64,
    FtmadZZzi,
    XaflagMPstate,
    CmnRrA1,
    FrsqrtsAsisdsamefp16Only,
    UqinchRRsUw,
    UqdecpZPZ,
    VnmulA1D,
    Uqsub16A1,
    FaddZPZs,
    VdotSA1Q,
    Rbit64Dp1Src,
    Ldnf1BZPBiU32,
    Ld1RqhZPBrContiguous,
    OrrsRrA1,
    NthRS,
    FcaddAsimdsame2C,
    VbicRA1Q,
    FsubZPZs,
    Sha512Su0Vv2Cryptosha5122,
    VabsA2D,
    Ld1SbZPBzD64Unscaled,
    CfinvMPstate,
    FmsbZPZzz,
    Vst3MA1Posti,
    NdZZi,
    FrecpxZPZ,
    FminnmpAsisdpairOnlySd,
    FmlslAsimdsameF,
    LsZPZ,
    StrhtA1,
    Ld1ShZPBrS64,
    CmeqAsisdsameOnly,
    Ldnf1SbZPBiS64,
    VpaddFA1,
    UqasxA1,
    SubAsimdsameOnly,
    St4DZPBrContiguous,
    UsubwAsimddiffW,
    SmovAsimdinsXX,
    BrkExException,
    VqabsA1Q,
    Zip1ZZzQ,
    LdrLA1,
    StlexbA1,
    PrfwIPBrS,
    LastaZPZz,
    Uzp1ZZz,
    SbcsRrA1,
    LdrexbA1,
    SsaxA1,
    Ld1RqwZPBiU32,
    FmulxAsimdsamefp16Only,
    FabdAsisdsamefp16Only,
    Ld1RqbZPBiU8,
    FcvtZPZD2H,
    LastaRPZ,
    VtstA1Q,
    St1BZPBzDX32Unscaled,
    Ldff1ShZPBzSX32Scaled,
    MulsA1,
    Stg64SpostLdsttags,
    Ssub16A1,
    CmpRA1Rrx,
    UaddlvAsimdallOnly,
    Ld1DZPBzDX32Unscaled,
    Paciza64ZDp1Src,
    LsrrZPZz,
    MulAsimdelemR,
    BfdotZZzz,
    XtnAsimdmiscN,
    ShsaxA1,
    UcvtfAsimdmiscR,
    SyslRcSysteminstrs,
    VabdFA1Q,
    VshlRA1Q,
    UqincbRRsUw,
    Ld1BZPBiU32,
    SmulwtA1,
    PrfhIPBiS,
    FtsmulZZz,
    FacgtAsisdsamefp16Only,
    VqshluIA1Q,
    CntAsimdmiscR,
    CmeqAsimdsameOnly,
    Ld1RobZPBiU8,
    RscsIA1,
    OrnPPPpZ,
    LdrZBi,
    Ldff1HZPAiS,
    Ld1ShZPAiD,
    Autiza64ZDp1Src,
    LdrIA1Pre,
    Sha1Su0VvvCryptosha3,
    VaddhnA1,
    Ldapr64LMemop,
    FrsqrteAsimdmiscfp16R,
    FaddvVPZ,
    SqincbRRsSx,
    UhsaxA1,
    Crc32CwA1,
    Ld1BZPBrU32,
    PrfbIPBzD64Scaled,
    Ld4BZPBrContiguous,
    Ldff1HZPBzDX32Unscaled,
    Ld1RsbZPBiS64,
    LdaxpLp64Ldstexcl,
    MrrcA1,
    PunpkloPP,
    DdZPZz,
    FminnmHFloatdp2,
    StcA1Pre,
    Umsubl64WaDp3Src,
    NtdRS,
    BOnlyCondbranch,
    NtwRS,
    LslZZw,
    SunpkhiZZ,
    VmovSrA1,
    Vld21T1A1A,
    BrkbsPPPZ,
    LdrDLdstImmpre,
    AndsRrA1,
    FcmlaZPZzz,
    SqincwZZs,
    Swpal64Memop,
    LdaxrLr64Ldstexcl,
    LdrsbtA2,
    Zip1ZZz,
    AeseA1,
    Ldff1HZPBzSX32Unscaled,
    BxA1,
    VfnmaA1D,
    FcvtxnAsimdmiscN,
    Ld1RowZPBrContiguous,
    Sm4EVv4Cryptosha5122,
    BrkaPPP,
    Ld1ShZPBiS32,
    SbA1,
    FminnmvVPZ,
    Ldff1BZPBzSX32Unscaled,
    Trn2ZZz,
    SudotZZzziS,
    UqdechZZs,
    MsrBrA1As,
    SdivA1,
    SelPPPp,
    FcvtzsZPZD2X,
    Ldff1ShZPBrS64,
    McrA1,
    VminnmA2D,
    FrintnZPZ,
    FmlslAsimdelemLh,
    FcvtZPZH2D,
    Stnt1HZPBrContiguous,
    VmovIT1A1A,
    VrshrA1Q,
    FnmlsZPZzz,
    UmmlaZZzz,
    DsbA1,
    Ld1BZPBiU8,
    OrvRPZ,
    VusdotA1Q,
    Ldff1DZPBzD64Scaled,
    UqsubZZi,
    ScvtfZPZW2S,
    FcaddZPZz,
    VsubhnA1,
    FabdAsisdsameOnly,
    FrsqrteAsimdmiscR,
    UqdecpRPRUw,
    St4BZPBiContiguous,
    Vst2MT1A1A,
    St2G64SoffsetLdsttags,
    St1WZPBzDX32Unscaled,
    FacgtAsimdsameOnly,
    SabdZPZz,
    Sbcs64AddsubCarry,
    AddsRrA1,
    FnmadZPZzz,
    PtruesPS,
    FcmlaZZzziS,
    FmulxAsisdsameOnly,
    SxthA1,
    Zip1PPp,
    PmullAsimddiffL,
    SaddvRPZ,
    EorZZz,
    EvbZZ,
    DupZR,
    Ldp64LdstpairPre,
    SbfxA1,
    VabsA1Q,
    Ld3WZPBiContiguous,
    Ld1BZPBrU16,
    Ld1HZPAiS,
    UqaddAsimdsameOnly,
    IncpZPZ,
    EorZPZz,
    BrknsPPPp,
    St1HZPBzSX32Scaled,
    Ldtr64LdstUnpriv,
    Irg64IDp2Src,
    SubsSpIA1,
    SqdmullAsimddiffL,
    FminpAsisdpairOnlySd,
    Ld1ShZPAiS,
    VpadalA1Q,
    BtiHbHints,
    SmmulrA1,
    PyZPR,
    DsbBoBarriers,
    VshrnA1,
    DffrsPPF,
    VfmsA1Q,
    LdrbRA1Pre,
    OrrsIA1,
    FcvtZPZS2D,
    Ld1ShZPBrS32,
    Rorv64Dp2Src,
    UsublAsimddiffL,
    IncpRPR,
    Ld1ShZPBiS64,
    Ld1BZPBzDX32Unscaled,
    MlsAsimdsameOnly,
    St1HZPAiS,
    VraddhnA1,
    ScvtfZPZX2Fp16,
    SqdecbRRsX,
    DupAsimdinsDrR,
    MsbZPZzz,
    BfmmlaAsimdsame2E,
    NorPPPpZ,
    Ldnf1WZPBiU64,
    AddpAsisdpairOnly,
    Sha256H2QqvCryptosha3,
    LdrhIA1Pre,
    VsliA1Q,
    PrfbIPAiD,
    MphsPPZi,
    Zip2PPp,
    Ld1BZPAiS,
    Uzp1ZZzQ,
    FcmgtPPZz,
    SasxA1,
    InchZZs,
    FcmgtPPZ0,
    FcvtauAsisdmiscR,
    VfmaBfA1Q,
    IsbBiBarriers,
    LdrbtA2,
    FrintiZPZ,
    Uqadd8A1,
    Usat16A1,
    BkptA1,
    MploPPZi,
    PrfmPLoadlit,
    Ld3HZPBrContiguous,
    PtruePS,
    Frint64XDFloatdp1,
    VmovnA1,
    VswpA1Q,
    Rev16A1,
    PldRA1Rrx,
    Subs64SAddsubExt,
    FdivrZPZz,
    LdrshRA1Pre,
    Ldnf1SbZPBiS16,
    SqdmullAsisddiffOnly,
    NegAsisdmiscR,
    Ldff1HZPBrU16,
    LdrhtA2,
    Vld41T1A1A,
    UqdecwRRsUw,
    RshrnAsimdshfN,
    AddsIA1,
    SqdecpZPZ,
    SqdecwRRsSx,
    Ld1DZPBzD64Scaled,
    St4BZPBrContiguous,
    Sha1CA1,
    St3WZPBrContiguous,
    VcaddA1Q,
    Ldnt1HZPBrContiguous,
    LdrshLA1,
    ScvtfZPZX2S,
    Ldnp64LdstnapairOffs,
    Uadd8A1,
    VrintzVfpA1D,
    UhsubAsimdsameOnly,
    UqincpZPZ,
    NdZZz,
    Ld1WZPBzD64Scaled,
    PtestPP,
    Ld2WZPBiContiguous,
    LdrDLdstRegoff,
    SqdechZZs,
    UqshlAsisdshfR,
    FrsqrtsZZz,
    UqxtnAsisdmiscN,
    St3BZPBrContiguous,
    Sha1PQsvCryptosha3,
    SqdecdZZs,
    Ldff1SbZPBzDX32Unscaled,
    VmulFA2D,
    LdcIA1Pre,
    FmulAsimdsameOnly,
    VabaA1Q,
    SvcA1,
    DupZZi,
    LdpQLdstpairPre,
    VdupSA1Q,
    OrrZZi,
    SmmlarA1,
    BfdotAsimdelemE,
    OrrZPZz,
    BfmlalAsimdelemF,
    SmcExException,
    Sm3Tt2BVvvCrypto3Imm2,
    LsrZPZz,
    Dcps3DcException,
    StrexbA1,
    BicsRrA1,
    LdrbLA1,
    Pacizb64ZDp1Src,
    SevA1,
    FmulZZz,
    UmaxvRPZ,
    VcvtmAsimdA1Q,
    FcmlaZZzziH,
    SxthZPZ,
    Stgp64LdstpairPost,
    EvPP,
    FminnmpAsisdpairOnlyH,
    Ld1HZPBrU64,
    Ld1DZPBzDX32Scaled,
    SqsubZZi,
    UminvAsimdallOnly,
    StlhA1,
    St1WZPBi,
    Drps64EBranchReg,
    SqincbRRsX,
    Ld1SbZPBiS32,
    LastbVPZ,
    DecpRPR,
    DbgA1,
    FdupZI,
    Ld1SwZPBzDX32Scaled,
    RscsRA1Rrx,
    LslZPZz,
    WhileloPPRr,
    UmlslAsimddiffL,
    LdrexA1,
    Ld1RshZPBiS32,
    FcvtZPZH2S,
    MpgePPZw,
    VnmlaA1D,
    FrecpxAsisdmiscR,
    St1HZPBzD64Unscaled,
    StrtA2,
    FminpAsisdpairOnlyH,
    UqaddZZz,
    UsatA1Asr,
    LdpQLdstpairPost,
    Ld1DZPBrU64,
    LdrexdA1,
    FminnmvAsimdallOnlySd,
    StrRA1Pre,
    PrfmPLdstPos,
    VrecpsA1Q,
    VcgtIA1Q,
    FmlsAsisdelemRhH,
    MlsA1,
    DdZZz,
    Ld1RsbZPBiS16,
    Uhsub8A1,
    FmaxvVPZ,
    VmovRsA1,
    MplsPPZw,
    FsubZZz,
    FmaxZPZs,
    UmaxZPZz,
    NopA1,
    PrfdIPBiS,
    QsaxA1,
    Qadd8A1,
    BfcvtZPZS2Bf,
    VcleIA1Q,
    VceqIA1Q,
    InchRRs,
    Ldff1SbZPBrS64,
    NdvRPZ,
    FmulAsimdsamefp16Only,
    CaspalCp64Ldstexcl,
    Ld1SwZPBzD64Scaled,
    DffrPPF,
    FcmleAsisdmiscfp16Fz,
    StrdIA1Pre,
    MplsPPZi,
    Ldnf1WZPBiU32,
    St4DZPBiContiguous,
    VmovSA1,
    VtbxA1,
    VmvnIT1A1A,
    Sha256Su0VvCryptosha2,
    Ldnt1WZPBrContiguous,
    FrsqrtsAsimdsamefp16Only,
    Ld1RqhZPBiU16,
    BfcvtnAsimdmisc4S,
    Uhadd8A1,
    FmmlaZZzzD,
    AesdBCryptoaes,
    BfdotAsimdsame2D,
    FrecpsAsimdsamefp16Only,
    Ldnt1BZPBiContiguous,
    DecdZZs,
    MovprfxZZ,
    Ld1WZPBzDX32Unscaled,
    FcvtzuAsisdmiscR,
    TstRA1Rrx,
    LdurDLdstUnscaled,
    SqnegAsimdmiscR,
    Stgp64LdstpairOff,
    St1HZPBr,
    Ldff1SwZPBzDX32Scaled,
    UmlslAsimdelemL,
    Ldnf1BZPBiU16,
    BrkpasPPPp,
    Uzp1PPp,
    PrfbIPBiS,
    St1BZPAiD,
    St2DZPBiContiguous,
    Zip2ZZzQ,
    UmullAsimdelemL,
    LdrhtA1,
    PssbbOnlyBarriers,
    MpgePPZz,
    Ld1BZPBiU16,
    LdrQLoadlit,
    St1DZPBzD64Unscaled,
    ShlAsisdshfR,
    SqincdRRsSx,
    LdmA1,
    Uadd16A1,
    VjcvtA1,
    FmlsAsisdelemRSd,
    Msub64ADp3Src,
    UcvtfAsimdmiscfp16R,
    UunpkhiZZ,
    Ld1DZPBzD64Unscaled,
    FcmgePPZz,
    Shsub16A1,
    FsqrtAsimdmiscR,
    VqdmlalA2,
    StrbtA2,
    MphiPPZz,
    VrintmVfpA1D,
    SqincpRPRSx,
    MphsPPZz,
    NtpRPP,
    SmlawtA1,
    UsmmlaZZzz,
    Addg64AddsubImmtags,
    BicZZz,
    Vld31T1A1A,
    Ld4WZPBrContiguous,
    Stnt1WZPBiContiguous,
    VdotA1Q,
    AesdA1,
    BA1,
    PliIA1,
    UqrshrnAsisdshfN,
    RbitA1,
    Usub16A1,
    Sm3Tt1BVvv4Crypto3Imm2,
    Subs64AddsubShift,
    UcvtfZPZW2D,
    VcvtSdA1,
    Ld1HZPAiD,
    VcvtShA1,
    Ldrab64WLdstPac,
    LdahA1,
    UqdecbRRsX,
    StrPBi,
    St2G64SpreLdsttags,
    ExtZZiDes,
    VshllA1,
    MovtA1,
    UmulhZPZz,
    PkhtbA1,
    SmuadxA1,
    Stzgm64BulkLdsttags,
    UqincdRRsUw,
    CmhsAsimdsameOnly,
    Ld3HZPBiContiguous,
    SubZPZz,
    SmaxvRPZ,
    FcvtzsZPZFp162X,
    FrsqrteZZ,
    LdrdIA1Pre,
    Pacdzb64ZDp1Src,
    Ld2DZPBiContiguous,
    PyZPI,
    Stzg64SpreLdsttags,
    FcvtxnAsisdmiscN,
    FaddpAsisdpairOnlyH,
    Ld4HZPBiContiguous,
    MpltPPZi,
    PrfwIPAiS,
    EorvRPZ,
    St2G64SpostLdsttags,
    UsqaddAsisdmiscR,
    SqdecwRRsX,
    UbfxA1,
    UqdecwZZs,
    CasalC64Ldstexcl,
    FmmlaZZzzS,
    MlsZPZzz,
    Vld11T1A1A,
    NandsPPPpZ,
    UaddvRPZ,
    StrbtA1,
    MovprfxZPZ,
    Ld1WZPBzD64Unscaled,
    SetffrF,
    Ldff1BZPBrU8,
    MpnePPZi,
    PssbbA1,
    SrrZPZz,
    FnegZPZ,
    OrrsPPPpZ,
    Ld4HZPBrContiguous,
    FstmdbxA1,
    BicZPZz,
    FcvtauAsimdmiscR,
    SminZZi,
    UqincdRRsX,
    FacgtAsimdsamefp16Only,
    UqrshlAsimdsameOnly,
    Ldr64LdstImmpre,
    Stz2G64SoffsetLdsttags,
    DecdRRs,
    Ld1BZPBzSX32Unscaled,
    Csneg64Condsel,
    LdrbIA1Pre,
    St1WZPAiS,
    VcvtXvA1D,
    NdPPPpZ,
    FrecpxAsisdmiscfp16R,
    DupAsisdoneOnly,
    HvcExException,
    Uzp2PPp,
    OrrZZz,
    StrexdA1,
    FmulZPZz,
    Ld1HZPBzSX32Unscaled,
    SminvRPZ,
    FmlsZZzziH,
    RscsRrA1,
    LdnpQLdstnapairOffs,
    VrintpAsimdA1Q,
    Ld1RhZPBiU16,
    PliRA1Rrx,
    VcgeRT1A1A,
    FaddZZz,
    Uzp2AsimdpermOnly,
    UabdlAsimddiffL,
    NotZPZ,
    Sm3Ss1Vvv4Crypto4,
    AesimcA1,
    Ldff1BZPBrU16,
    VabdlIA1,
    FrsqrteAsisdmiscfp16R,
    SqrdmlshAsisdelemR,
    Ld1RowZPBiU32,
    FcmnePPZz,
    Ld1SbZPBrS16,
    PrfdIPBzD64Scaled,
    OmpactZPZ,
    Stnt1HZPBiContiguous,
    Ccmp64CondcmpImm,
    VldmdbA2,
    VcvttA1Hd,
    Ld1RqbZPBrContiguous,
    Ldff1WZPAiS,
    LdarLr64Ldstexcl,
    FmulZZziD,
    Ld1WZPBzSX32Unscaled,
    VdupRA1,
    Ldnf1HZPBiU16,
    VmsrA1As,
    HltA1,
    LastbZPZz,
    Ld4WZPBiContiguous,
    SmlsdxA1,
    VcvtmVfpA1D,
    ClrexA1,
    TstRrA1,
    Ld1SwZPBiS64,
    ShasxA1,
    Ldr64LdstImmpost,
    AndsIA1,
    ExtAsimdextOnly,
    EorsRA1Rrx,
    LzZPZ,
    VmullSA1,
    DrZAzDU32Scaled,
    LdrsbtA1,
    Ldff1SbZPAiS,
    Vld4AA1Posti,
    ScvtfZPZW2Fp16,
    Ld1HZPBzDX32Unscaled,
    Vld3AA1Posti,
    DvlRI,
    St3HZPBrContiguous,
    WhilelePPRr,
    CmltAsisdmiscZ,
    FcvtlAsimdmiscL,
    UqsaxA1,
    StmdaA1,
    PunpkhiPP,
    FabdZPZz,
    SmcA1As,
    ScvtfZPZW2D,
    VcmpA1A,
    LdrsbLA1,
    PrfdIPBzSX32Scaled,
    Ldff1HZPBrU32,
    FdivZPZz,
    VqaddA1Q,
    Usub8A1,
    LsrZPZw,
    Ldff1BZPBrU32,
    Stnt1WZPBrContiguous,
    VcvtBfsA1,
    VrsubhnA1,
    FmlsAsimdelemRSd,
    UabaAsimdsameOnly,
    VudotSA1Q,
    FscaleZPZz,
    VpminIA1,
    VmovlA1,
    EvwZZ,
    Ldff1SbZPAiD,
    VminIA1Q,
    BifAsimdsameOnly,
    St1DZPBr,
    OrnsPPPpZ,
    Shsub8A1,
    Trn2AsimdpermOnly,
    SubZZz,
    SqaddZZi,
    FminpAsimdsameOnly,
    VcvttA1Bfs,
    Movk64Movewide,
    RsbsRrA1,
    SxtwZPZ,
    AesimcBCryptoaes,
    TstIA1,
    IncdRRs,
    FaddpAsimdsameOnly,
    VnegA1Q,
    ScvtfZPZX2D,
    DdvlRRi,
    SqaddZZz,
    FmulZZziS,
    Ld1WZPAiD,
    Ld1ShZPBzDX32Scaled,
    VpaddIA1,
    PacibspHiHints,
    VpaddlA1Q,
    VsqrtA1D,
    VmlsFA2D,
    Sha1MQsvCryptosha3,
    UmaxZZi,
    Vst1MT1A1A,
    UqsubAsisdsameOnly,
    LdrDLdstPos,
    Ld1DZPAiD,
    FminvAsimdallOnlyH,
    NorsPPPpZ,
    TeqRA1Rrx,
    UcvtfAsimdshfC,
    VmulFA1Q,
    FrintxZPZ,
    NtbRS,
    Ld1RwZPBiU32,
    UcvtfAsisdmiscR,
    SqincpRPRX,
    Ld1RbZPBiU16,
    StrbRA1Pre,
    UdivA1,
    Ld1SbZPBiS64,
    FrecpeAsisdmiscfp16R,
    DupAsimdinsDvV,
    FcvtzuZPZS2X,
    Ldff1ShZPBzD64Scaled,
    VstmdbA2,
    Ld1SbZPBiS16,
    Ldff1DZPAiD,
    SmullsA1,
    Ld1RsbZPBiS32,
    Ld1HZPBiU64,
    FrecpsAsisdsameOnly,
    PrfwIPAiD,
    PfalseP,
    FcvtzsZPZFp162W,
    VbicIT1A1A,
    PfirstPPP,
    SmlalttA1,
    CmleAsisdmiscZ,
    Ld2HZPBrContiguous,
    St1BZPBzD64Unscaled,
    FmulxAsisdelemRhH,
    UmmlaAsimdsame2G,
    Ldff1ShZPBzDX32Unscaled,
    StrhIA1Pre,
    UcvtfAsisdmiscfp16R,
    VabdIA1Q,
    EretA1,
    VmlsIA1Q,
    SqrdmulhAsimdsameOnly,
    Ldff1ShZPBzD64Unscaled,
    WhileltPPRr,
    Ld1WZPBiU64,
    BicsPPPpZ,
    FabdAsimdsamefp16Only,
    Ld3DZPBiContiguous,
    CmnIA1,
    SxtahA1,
    UhaddAsimdsameOnly,
    PrfbIPBzDX32Scaled,
    MpgtPPZz,
    SmulhZPZz,
    St3BZPBiContiguous,
    SmulttA1,
    VmrsA1As,
    VpminFA1,
    Vrev16A1Q,
    UqincbRRsX,
    CpsidA1Asm,
    St2HZPBrContiguous,
    TeqRrA1,
    Stzg64SoffsetLdsttags,
    VcvtIsA1Q,
    Uqsub8A1,
    BicPPPpZ,
    LdrbtA1,
    FaddZPZz,
    SqdmullAsimdelemL,
    PldIA1,
    VacgtA1Q,
    Trn2PPp,
    FminnmpAsimdsameOnly,
    Ld3BZPBiContiguous,
    LdrRA1Pre,
    UmullAsimddiffL,
    PrfbIPBrS,
    StrhRA1Pre,
    FaddpAsimdsamefp16Only,
    PrfhIPBzD64Scaled,
    Ldff1HZPBrU64,
    MovsIA1,
    SqdechRRsX,
    PrfwIPBzSX32Scaled,
    Ldg64LoffsetLdsttags,
    Ldff1SwZPBrS64,
    UqsubAsimdsameOnly,
    Vst21T1A1A,
    VcgtRT1A1A,
    FsubZPZz,
    Eor3Vvv16Crypto4,
    Ld1SwZPBrS64,
    Ldff1SbZPBzD64Unscaled,
    Ldnf1SwZPBiS64,
    StlA1,
    Sha256HA1,
    FcvtZPZS2H,
    Vst41A3Nowb,
    MlsAsimdelemR,
    FminnmZPZz,
    Vld1AA1Posti,
    Ldff1WZPBzD64Scaled,
    St2WZPBrContiguous,
    OrnAsimdsameOnly,
    FsubrZPZz,
    Ldnt1BZPBrContiguous,
    Stz2G64SpostLdsttags,
    SmmlsrA1,
    PrfwIPBzDX32Scaled,
    Uhadd16A1,
    Ld2BZPBrContiguous,
    FtsselZZz,
    Sdiv64Dp2Src,
    CmleAsimdmiscZ,
    VfmsA2D,
    St1HZPBzDX32Unscaled,
    StlexhA1,
    BicsIA1,
    Trn2ZZzQ,
    RevshA1,
    FcmgePPZ0,
    Ld1WZPAiS,
    UadalpAsimdmiscP,
    FmlsAsimdelemRhH,
    Blrab64PBranchReg,
    VmvnRA1Q,
    VcvtbA1Bfs,
    PaciaspHiHints,
    VornRA1Q,
    Ld4RAsisdlsopR4I,
    FcmeqPPZz,
    VorrRA1Q,
    MrcA1,
    Ldff1SwZPBzDX32Unscaled,
    Ld1RdZPBiU64,
    EvhZZ,
    Ldff1BZPBzDX32Unscaled,
    StrbIA1Pre,
    FrsqrtsAsimdsameOnly,
    InsrZR,
    SqnegAsisdmiscR,
    UcvtfAsisdshfC,
    MploPPZw,
    FcmlaAsimdsame2C,
    MulZPZz,
    VdivA1D,
    Trn1ZZz,
    Ldff1WZPAiD,
    Sm3Partw1Vvv4Cryptosha5123,
    SqincwRRsX,
    UsaxA1,
    LdmdbA1,
    FsqrtHFloatdp1,
    Vst41A1Posti,
    VqdmulhA2Q,
    Ldapur64LdapstlUnscaled,
    FcvtzuZPZD2X,
    VqrdmlshA1Q,
    Ld1WZPBzDX32Scaled,
    UunpkloZZ,
    SqrdmlshAsisdsame2Only,
    Ld1ShZPBzSX32Scaled,
    UqxtnAsimdmiscN,
    Ld1ShZPBzDX32Unscaled,
    LdrshtA2,
    Ld1SbZPAiS,
    VqshrunA1,
    UqinchRRsX,
    MpeqPPZi,
    BfmlaltZZzzi,
    FrecpsAsisdsamefp16Only,
    Sxtab16A1,
    FmaxnmZPZz,
    FcmltAsisdmiscFz,
    MpnePPZw,
    RbitAsimdmiscR,
    Sha512HQqvCryptosha5123,
    VclsA1Q,
    VsraA1Q,
    Ld1SwZPBzDX32Unscaled,
    SunpkloZZ,
    Shadd16A1,
    FminnmvAsimdallOnlyH,
    FmlsZPZzz,
    FcmnePPZ0,
    SqdmlslAsisddiffOnly,
    UsdotZZzziS,
    UminvRPZ,
    SsbbOnlyBarriers,
    InsAsimdinsIvV,
    SetendA1,
    St1WZPBzSX32Unscaled,
    LdaexbA1,
    Autizb64ZDp1Src,
    VabalA1,
    SliAsimdshfR,
    Sha256Su1VvvCryptosha3,
    Crc32Cx64CDp2Src,
    Ldnt1WZPBiContiguous,
    FcvtzsZPZS2W,
    Sm3Tt2AVvv4Crypto3Imm2,
    VrintzAsimdA1Q,
    VsriA1Q,
    FminpAsimdsamefp16Only,
    UcvtfZPZW2Fp16,
    Ldff1SwZPBzD64Unscaled,
    BfmlalAsimdsame2F,
    MpgtPPZw,
    Ldnt1DZPBrContiguous,
    SqrdmulhAsimdelemR,
    VqrshlA1Q,
    Ld1RqdZPBiU64,
    Uhsub16A1,
    BrkasPPPZ,
    BfdotZZzzi,
    Vld2MT1A1A,
    LdrexhA1,
    Ccmp64CondcmpReg,
    FdivAsimdsamefp16Only,
    SqdmlslAsimddiffL,
    Extr64Extract,
    FmadZPZzz,
    SqincpZPZ,
    Ld1BZPBrU8,
    UsdotAsimdsame2D,
    LdpQLdstpairOff,
    St2BZPBrContiguous,
    VcltIA1Q,
    VminFA1Q,
    VuzpA1Q,
    OrrsRA1Rrx,
    QaddA1,
    Ld1RohZPBrContiguous,
    SmladxA1,
    VmlsSA1Q,
    LastbRPZ,
    StmA1,
    XarVvv2Crypto3Imm6,
    MpeqPPZw,
    Ld1SbZPBzSX32Unscaled,
    AdcsRA1Rrx,
    St3DZPBiContiguous,
    RsbsIA1,
    IndexZIi,
    Ands64SLogImm,
    MovsRA1Rrx,
    DechZZs,
    SdotZZzziD,
    VmulSA1Q,
    Vst31T1A1A,
    St1DZPBzDX32Scaled,
    FcmleAsimdmiscfp16Fz,
    Usada8A1,
    SqincwRRsSx,
    SriAsisdshfR,
    FmulxAsisdelemRSd,
    SqdecwZZs,
    VqmovnA1,
    SqdecdRRsSx,
    NdZPZz,
    St1BZPBzSX32Unscaled,
    Ld1SbZPAiD,
    Vld4MA1Posti,
    VqdmlslA1,
    VmovSsA1,
    St2WZPBiContiguous,
    StrdRA1Pre,
    TeqIA1,
    Zip2ZZz,
    Ld2BZPBiContiguous,
    PldLA1,
    FmaxnmZPZs,
    MpgtPPZi,
    DmbBoBarriers,
    LdrtA2,
    Qadd16A1,
    Ld1ShZPBzSX32Unscaled,
    StmibA1,
    QasxA1,
    SubsSpRA1Rrx,
    Uzp2ZZzQ,
    FmlaZPZzz,
    SrZPZz,
    AdrpOnlyPcreladdr,
    BfmlaltZZzz,
}

#[cfg(test)]
mod tests {
    use super::{decode_a32, Op};

    #[test]
    fn test() {
        assert_eq!(decode_a32(0xe3a00001).unwrap(), Op::MovsIA1);
    }
}
