#!/usr/bin/env python3
"""Add prelude imports to all Rust files in the Tariro project."""
import os
import re

src_dir = "/workspace/tariro/src"
prelude_import = "use crate::prelude::*;"

# Files to skip (already have prelude or are special)
SKIP_FILES = {"lib.rs", "main.rs", "prelude.rs"}

def needs_prelude(filepath):
    """Check if file needs prelude import."""
    with open(filepath, 'r') as f:
        content = f.read()

    # Already has prelude
    if "use crate::prelude" in content:
        return False

    # Check if it's a normal module file (has pub struct, enum, fn definitions)
    has_code = any([
        'pub struct' in content,
        'pub enum' in content,
        'pub fn' in content,
        'pub mod' in content,
        '#[derive(' in content,
        'struct ' in content and 'impl ' in content,
    ])

    return has_code

# Find all .rs files that need prelude
files_to_fix = []
for root, dirs, files in os.walk(src_dir):
    for filename in files:
        if not filename.endswith(".rs"):
            continue
        if filename in SKIP_FILES:
            continue

        filepath = os.path.join(root, filename)
        if needs_prelude(filepath):
            files_to_fix.append(filepath)

print(f"Found {len(files_to_fix)} files needing prelude imports")

modified = 0
for filepath in files_to_fix:
    with open(filepath, 'r') as f:
        content = f.read()

    # Find where to insert
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

    modified += 1
    relpath = os.path.relpath(filepath, src_dir)
    print(f"Added prelude import to {relpath}")

print(f"\nDone! Modified {modified} files")
