import io, sys, re

path = "/workspace/Tariro/src/cognition/tests.rs"
src = open(path).read()

mods = {
    "consciousness_tests": "consciousness",
    "working_memory_tests": "working_memory",
    "world_model_tests": "world_model",
    "motivation_tests": "motivation",
    "cognition_engine_tests": "cognition_engine",
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
