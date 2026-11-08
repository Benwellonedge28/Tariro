import re, collections, sys

LOG = sys.argv[1] if len(sys.argv) > 1 else '/tmp/check3.log'
pat = sys.argv[2] if len(sys.argv) > 2 else ''
limit = int(sys.argv[3]) if len(sys.argv) > 3 else 200
lines = open(LOG).read().splitlines()
out = []
for l in lines:
    m = re.match(r'^(src/[^:]+):(\d+):(\d+): error\[(E\d+)\]: (.*)$', l)
    if m and (pat in m.group(1)):
        out.append((int(m.group(2)), m.group(1), m.group(4), m.group(5)))
out.sort()
cnt = collections.Counter((x[3][:70]) for x in out)
print("FILE", pat, "errors", len(out))
for k, v in cnt.most_common(40):
    print(v, "|", k)
print("--- details ---")
for x in out[:limit]:
    print("%s:%d [%s] %s" % (x[1], x[0], x[2], x[3]))
