import sys

BASE = "/workspace/Tariro/"

PATCHES = [
    # --- simple missing imports ---
    ("src/tumc_compiler/package_manager.rs",
     "use std::path::PathBuf;",
     "use std::path::{Path, PathBuf};"),
    ("src/tumc_compiler/threading.rs",
     "use std::future::Future;",
     "use std::future::Future;\nuse std::thread::JoinHandle;"),
    ("src/tumc_compiler/ir/dialects/gpu.rs",
     "Shuffle(Bool, ValueId, ValueId, ValueId), // width, mask, src_lane",
     "Shuffle(IrType, ValueId, ValueId, ValueId), // width, mask, src_lane"),
    ("src/tumc_compiler/wasi.rs",
     "        use preview2::WasiTable;",
     "        use preview2::{ResourceKind, WasiTable};"),
]

for path, old, new in PATCHES:
    full = BASE + path
    src = open(full).read()
    if old not in src:
        print("MISS", path, "|", old[:60])
        continue
    if src.count(old) != 1:
        print("DUP(%d)" % src.count(old), path, "|", old[:60])
    src = src.replace(old, new, 1)
    open(full, "w").write(src)
    print("OK", path)
