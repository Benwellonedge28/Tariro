import re, collections, sys

LOG = sys.argv[1] if len(sys.argv) > 1 else '/tmp/check3.log'
lines = open(LOG).read().splitlines()
errs = collections.defaultdict(list)
for l in lines:
    m = re.match(r'^(src/[^:]+):(\d+):(\d+): error\[(E\d+)\]: (.*)$', l)
    if m:
        errs[m.group(1)].append((int(m.group(2)), m.group(4), m.group(5)))
cnt = collections.Counter()
for f, e in errs.items():
    cnt[f] = len(e)
print("TOTAL", sum(cnt.values()), "FILES", len(cnt))
for f, c in cnt.most_common(60):
    print(c, f)
print("---- error codes ----")
cc = collections.Counter()
for f, e in errs.items():
    for x in e:
        cc[x[1]] += 1
for k, v in cc.most_common():
    print(v, k)
