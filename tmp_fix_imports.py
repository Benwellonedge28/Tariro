import re, os, collections, sys

LOG = sys.argv[1] if len(sys.argv) > 1 else "/tmp/check2.log"
REPO = "/workspace/Tariro"

# ---- collect public type names defined per module ----
def pub_types(path):
    names = set()
    if not os.path.exists(path):
        return names
    src = open(path).read()
    for m in re.finditer(r'^pub (?:struct|enum|trait|type) ([A-Za-z_][A-Za-z0-9_]*)', src, re.M):
        names.add(m.group(1))
    return names

IR_NAMES = pub_types(REPO + "/src/tumc_compiler/ir/mod.rs")
AST_NAMES = pub_types(REPO + "/src/tumc_compiler/ast.rs")
# dialects
for d in ["linalg", "gpu", "typed_ptr", "quantum"]:
    IR_NAMES |= pub_types(REPO + "/src/tumc_compiler/ir/dialects/%s.rs" % d)

# ---- parse errors ----
missing = collections.defaultdict(set)   # file -> set of names
err_re = re.compile(r'^(src/[^:]+\.rs):\d+:\d+: error\[E0(425|433|422)\]: cannot find (?:type|struct, variant or union type|function, tuple struct or tuple variant) `([A-Za-z_][A-Za-z0-9_]*)`')
for line in open(LOG):
    m = err_re.match(line)
    if m:
        missing[m.group(1)].add(m.group(3))

report = []
for f in sorted(missing):
    src = open(REPO + "/" + f).read()
    names = missing[f]
    # what does the file already import from tumc_compiler?
    has_ir = re.search(r'use crate::tumc_compiler::ir\b', src) or re.search(r'use super::ir\b', src) or 'use super::*;' in src and 'ir' in src
    has_ast = re.search(r'use crate::tumc_compiler::ast\b', src)
    # names already imported
    imported = set(re.findall(r'\b[A-Z][A-Za-z0-9_]*\b', "\n".join(re.findall(r'use [^;]+;', src))))
    local_defs = set(re.findall(r'^(?:pub )?(?:struct|enum|trait|type) ([A-Za-z_][A-Za-z0-9_]*)', src, re.M))
    todo = [n for n in names if n not in imported and n not in local_defs]
    ir_needed = [n for n in todo if n in IR_NAMES]
    ast_needed = [n for n in todo if n in AST_NAMES]
    report.append((f, len(names), sorted(todo), ir_needed, ast_needed))

for f, cnt, todo, ir_needed, ast_needed in report:
    print("%-55s errors=%-4d" % (f, cnt))
    if todo:
        print("    missing: %s" % ", ".join(todo[:20]))
    if ir_needed:
        print("    IR has  : %s" % ", ".join(ir_needed[:20]))
    if ast_needed:
        print("    AST has : %s" % ", ".join(ast_needed[:20]))
