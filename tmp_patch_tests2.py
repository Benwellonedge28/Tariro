import sys

path = "/workspace/Tariro/src/knowledge/tests.rs"
src = open(path).read()

mods = {
    "knowledge_graph_tests": "knowledge_graph",
    "reasoning_tests": "reasoning",
    "logic_tests": "logic",
    "causal_tests": "causal",
    "commonsense_tests": "commonsense",
    "formal_tests": "formal",
    "mind_tests": "mind",
}

for tname, target in mods.items():
    old = "mod %s {\n    use super::super::*;" % tname
    new = ("mod %s {\n"
           "    use super::super::*;\n"
           "    use super::super::%s::*;" % (tname, target))
    if old not in src:
        print("MISS", tname)
        continue
    src = src.replace(old, new, 1)
    print("OK", tname)

open(path, "w").write(src)
