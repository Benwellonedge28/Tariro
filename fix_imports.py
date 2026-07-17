#!/usr/bin/env python3
"""Add prelude imports to Rust files that need chrono types."""
import os
import re

src_dir = "/workspace/Tariro/src"
prelude_import = "use crate::prelude::*;"

# Find all .rs files that use DateTime<Utc>
files_with_datetime = []
for filename in os.listdir(src_dir):
    if filename.endswith(".rs"):
        filepath = os.path.join(src_dir, filename)
        with open(filepath, 'r') as f:
            content = f.read()
            if "DateTime<Utc>" in content or "Utc::now()" in content:
                files_with_datetime.append(filename)

print(f"Found {len(files_with_datetime)} files needing chrono imports")

for filename in files_with_datetime:
    filepath = os.path.join(src_dir, filename)
    with open(filepath, 'r') as f:
        content = f.read()

    # Skip if prelude already imported
    if "use crate::prelude::" in content:
        continue

    # Skip lib.rs (already has prelude)
    if filename == "lib.rs":
        continue

    # Find where to insert - after the module doc or after existing use statements
    # Look for the first non-doc, non-comment line after any doc comments
    lines = content.split('\n')

    # Find insertion point (after doc comments and attributes)
    insert_idx = 0
    for i, line in enumerate(lines):
        stripped = line.strip()
        # Stop at first non-doc, non-attribute, non-empty line
        if stripped and not stripped.startswith('//') and not stripped.startswith('#') and not stripped.startswith('/*') and not stripped.startswith('*'):
            insert_idx = i
            break

    # Insert the prelude import
    if insert_idx == 0:
        lines.insert(0, prelude_import)
    else:
        lines.insert(insert_idx, prelude_import)

    new_content = '\n'.join(lines)

    with open(filepath, 'w') as f:
        f.write(new_content)
    print(f"Added prelude import to {filename}")

print("Done!")
