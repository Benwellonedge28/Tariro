import re, sys, collections

files = sys.argv[1:]
variants = collections.defaultdict(set)
for f in files:
    src = open(f).read()
    for m in re.finditer(r'\b([A-Z][A-Za-z0-9_]*)\s*(::\s*([A-Za-z_][A-Za-z0-9_]*)\s*[{,(])', src):
        variants[m.group(1)].add(m.group(3))
for k in sorted(variants):
    vals = sorted(variants[k])
    print("%s: %s" % (k, ", ".join(vals)))
