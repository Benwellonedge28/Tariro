import re, sys, os

files = sys.argv[1:]
for f in files:
    src = open(f).read()
    names = []
    for m in re.finditer(r'^pub (?:struct|enum|trait|type|fn|const)\s+([A-Za-z_][A-Za-z0-9_]*)', src, re.M):
        names.append(m.group(1))
    for m in re.finditer(r'^pub use ([^;]+);', src, re.M):
        names.append("USE:" + m.group(1).strip())
    print("==", f)
    print("   ", ", ".join(names))
