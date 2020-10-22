from aslutils.parse_asl_file import *

TAB = "    "

# remove prefix
def r(text, prefix):
    if text.startswith(prefix):
        return text[len(prefix):]
    return text

def to_name(sym):
    return r(r(sym, "aarch32_"), "aarch64_").rstrip("_").replace("_", " ").title().replace(" ", "")

class Decoder(NopDecodeListener):
    def __init__(self, include_fields=True):
        self.include_fields = include_fields
        self.tabs = 0
        self.field_sizes = {}
        self.variants = set(["Nop"])
        self.unused_var_stack = []
        self.variant_fields = {}

    def unused_in_scope_vars(self):
        x = []
        for y in self.unused_var_stack:
            x += y
        return x

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
        self.unused_var_stack.pop()
        print(f"{TAB * self.tabs}}}")

    def listen_case(self, fields):
        def f(field):
            if field.name:
                try:
                    self.unused_var_stack[-1].remove(field.name)
                except:
                    pass
                return field.name
            elif field.concat_names:
                shift = self.field_sizes[field.concat_names[1]]
                return f"({field.concat_names[0]} << {shift}) | {field.concat_names[1]}"
            else:
                mask = (1 << field.run) - 1
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
            print("#[allow(unreachable_patterns)]")
            if self.include_fields:
                print(f"pub fn decode_{name.lower()}(instr: u32) -> Option<Instr> {{")
            else:
                print(f"pub fn decode_mnemonic_{name.lower()}(instr: u32) -> Option<Mnemonic> {{")
            self.tabs += 1
            return True

    def listen_encoding(self, name, comment):
        if comment:
            name = comment
        name = to_name(name)
        if self.include_fields:
            self.variants.add(name)
            fields = self.unused_in_scope_vars()

            def to_field(x):
                bits = self.field_sizes[x]
                if bits <= 8:
                    ty = "u8"
                elif bits <= 16:
                    ty = "u16"
                else:
                    ty = "u32"
                return (x, ty)

            self.variant_fields[name] = [to_field(x) for x in fields]
            fields = "" if len(fields) == 0 else (" {" + ", ".join([f"{x}: {x} as _" for x in fields]) + "}")
            print(f"{TAB * self.tabs}Some(Instr::{name}{fields})")
        else:
            print(f"{TAB * self.tabs}Some(Mnemonic::{name})")

    def listen_field(self, name, start, run):
        self.unused_var_stack[-1].append(name)
        mask = (1 << run) - 1
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
        if self.include_fields:
            print(f"{TAB * self.tabs}Some(Instr::Nop)")
        else:
            print(f"{TAB * self.tabs}Some(Mnemonic::Nop)")

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
        self.unused_var_stack.append([])
        return True

dec = Decoder()
parse_asl_decoder_file("../mra_tools/arch/arch_decode.asl", dec)
dec2 = Decoder(include_fields=False)
parse_asl_decoder_file("../mra_tools/arch/arch_decode.asl", dec2)
print()
print("#[allow(non_snake_case)]")
print("#[derive(Debug, PartialEq, Clone)]")
print("pub enum Mnemonic {")
for op in dec.variants:
    print(f"    {op},")
print("}")
print()
print("#[allow(non_snake_case)]")
print("#[derive(Debug, PartialEq, Clone)]")
print("pub enum Instr {")
for op in dec.variants:
    fields = dec.variant_fields.get(op, [])
    if len(fields) == 0:
        print(f"    {op},")
    else:
        print(f"    {op} {{")
        for field, ty in fields:
            print(f"        {field}: {ty},")
        print(f"    }},")
print("}")
print("""
#[cfg(test)]
mod tests;
""")
