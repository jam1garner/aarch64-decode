from aslutils.parse_asl_file import *

TAB = "    "

class Decoder(NopDecodeListener):
    def __init__(self):
        self.tabs = 0
        self.field_sizes = {}
        self.variants = set(["Nop"])

    def after_listen_case(self, fields):
        if len(fields) != 0:
            print(f"{TAB * self.tabs}_ => None")
        self.tabs -= 1
        print(f"{TAB * self.tabs}}}")

    def after_listen_decode(self, name):
        if name == "A64" or name == "A32":
            self.tabs -= 1
            print("} // end of decoding " + name)

    def after_listen_when(self, _):
        self.tabs -= 1
        print(f"{TAB * self.tabs}}}")

    def listen_case(self, fields):
        def f(field):
            if field.name:
                return field.name
            elif field.concat_names:
                shift = self.field_sizes[field.concat_names[1]]
                return f"({field.concat_names[0]} << {shift}) | {field.concat_names[1]}"
            else:
                mask = (field.run << 1) - 1
                start = field.start
                if start == 0:
                    return f"instr & {mask}"
                else:
                    return f"(instr >> {start}) & {mask}"
        if len(fields) == 1:
            print(f"{TAB * self.tabs}match " + ", ".join([f(field) for field in fields]) + " {")
        else:
            print(f"{TAB * self.tabs}match (" + ", ".join([f(field) for field in fields]) + ") {")
        self.tabs += 1
        return True

    def listen_decode(self, name):
        if name == "A64" or name == "A32":
            print("#[allow(unused_variables)]")
            print("#[allow(non_snake_case)]")
            print(f"pub fn decode_{name.lower()}(instr: u32) -> Option<Op> {{")
            self.tabs += 1
            return True

    def listen_encoding(self, name):
        self.variants.add(name)
        print(f"{TAB * self.tabs}Some(Op::{name})")

    def listen_field(self, name, start, run):
        mask = (run << 1) - 1
        self.field_sizes[name] = run
        if start == 0:
            print(f"{TAB * self.tabs}let {name} = instr & {mask};")
        else:
            print(f"{TAB * self.tabs}let {name} = (instr >> {start}) & {mask};")
        return True

    def listen_unallocated(self):
        print(f"{TAB * self.tabs}None")

    def listen_undocumented(self):
        print(f"{TAB * self.tabs}None")

    def listen_unused(self):
        print(f"{TAB * self.tabs}None")

    def listen_nop(self):
        print(f"{TAB * self.tabs}Some(Op::Nop)")

    def listen_unpredictable(self):
        print(f"{TAB * self.tabs}None")
        return True

    def listen_when(self, values):
        def to_bitmask(x):
            if x.find("x") == -1:
                return None
            return int(x.replace("0", "1").replace("x", "0"), 2)

        def to_maskval(x):
            return int(x.replace("x", "0"), 2)

        def f(val, i):
            if val.empty:
                return ("_", None)
            elif val.notvalue:
                mask = to_bitmask(val.notvalue)
                val = to_maskval(val.notvalue)
                if mask == None:
                    return (f"x{i}", f"x{i} != {val}")
                return (f"x{i}", f"x{i} & {mask} != {val}")
            elif val.value:
                mask = to_bitmask(val.value)
                val = to_maskval(val.value)
                if mask == None:
                    return (f"x{i}", f"x{i} == {val}")
                return (f"x{i}", f"x{i} & {mask} == {val}")
            elif val.range:
                return (f"0b{val.range[0]:b}..=0b{val.range[1]:b}", None)
            else:
                assert False
        vals = [f(val, i) for i, val in enumerate(values)]
        pats = ", ".join([val[0] for val in vals])
        conds = [val[1] for val in vals if val[1]]
        cond = "" if len(conds) == 0 else (" if " + " && ".join(conds))
        if len(vals) == 1:
            print(f"{TAB * self.tabs}{pats}{cond} => {{")
        else:
            print(f"{TAB * self.tabs}({pats}){cond} => {{")
        self.tabs += 1
        return True

dec = Decoder()
parse_asl_decoder_file("../mra_tools/arch/arch_decode.asl", dec)
print()
print("#[derive(Debug, PartialEq, Clone)]")
print("pub enum Op {")
for op in dec.variants:
    print(f"    {op},")
print("}")
print()
print("""
#[cfg(test)]
mod tests {
    use super::{decode_a64, decode_a32, Op};

    #[test]
    fn test() {
        assert_eq!(decode_a32(0xe3a00001).unwrap(), Op::aarch32_MOV_i_A1_A);
    }
}""")
