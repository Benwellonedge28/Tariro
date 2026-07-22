#!/usr/bin/env python3
"""Fix serde attribute issues by adding serde imports."""

import os

src_dir = "/workspace/Tariro/src"
serde_import = "use serde::{Serialize, Deserialize};"

# Find all .rs files with serde(with = ...) attribute
files_with_serde = []
for filename in os.listdir(src_dir):
    if filename.endswith(".rs") and filename != "lib.rs":
        filepath = os.path.join(src_dir, filename)
        with open(filepath, 'r') as f:
            content = f.read()
            if 'serde(with = "chrono::serde::ts_seconds")' in content:
                files_with_serde.append(filename)

print(f"Found {len(files_with_serde)} files needing serde imports")

fixed_count = 0
for filename in files_with_serde:
    filepath = os.path.join(src_dir, filename)
    with open(filepath, 'r') as f:
        content = f.read()

    # Skip if serde already imported
    if "use serde::" in content or "#[macro_use] extern crate serde" in content:
        continue

    original = content

    # Find insertion point - after any existing use statements
    lines = content.split('\n')

    # Find insertion point (after doc comments, attributes, and module declarations)
    insert_idx = 0
    for i, line in enumerate(lines):
        stripped = line.strip()
        # Stop at first non-doc, non-attribute, non-empty line that's not a use statement
        if stripped and not stripped.startswith('//') and not stripped.startswith('#') and not stripped.startswith('/*') and not stripped.startswith('*'):
            # Skip use statements
            if not stripped.startswith('use '):
                insert_idx = i
                break
            else:
                insert_idx = i + 1

    # Insert the serde import
    lines.insert(insert_idx, serde_import)

    content = '\n'.join(lines)

    if content != original:
        with open(filepath, 'w') as f:
            f.write(content)
        fixed_count += 1
        print(f"Added serde import to {filename}")

print(f"Done! Fixed {fixed_count} files.")
