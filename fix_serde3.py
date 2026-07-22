#!/usr/bin/env python3
"""Fix serde derive issues - add Serialize, Deserialize to all derive macros in files using serde."""

import os
import re

src_dir = "/workspace/Tariro/src"

# Find all .rs files with serde attribute
files_with_serde = []
for filename in os.listdir(src_dir):
    if filename.endswith(".rs") and filename != "lib.rs":
        filepath = os.path.join(src_dir, filename)
        with open(filepath, 'r') as f:
            content = f.read()
            if 'serde(with = "chrono::serde::ts_seconds")' in content or '#[derive(' in content and 'serde::' in content:
                files_with_serde.append(filename)

print(f"Found {len(files_with_serde)} files with serde usage")

# Also check for any file that might need it
all_serde_files = []
for filename in os.listdir(src_dir):
    if filename.endswith(".rs") and filename != "lib.rs":
        filepath = os.path.join(src_dir, filename)
        with open(filepath, 'r') as f:
            content = f.read()
            if '#[serde(' in content:
                all_serde_files.append(filename)

print(f"Total files with #[serde( attribute: {len(all_serde_files)}")

fixed_count = 0
for filename in all_serde_files:
    filepath = os.path.join(src_dir, filename)
    with open(filepath, 'r') as f:
        content = f.read()

    original = content

    # Add use serde statement if not present
    if 'use serde::' not in content and '#[macro_use] extern crate serde' not in content:
        # Find insertion point
        lines = content.split('\n')
        insert_idx = 0
        for i, line in enumerate(lines):
            stripped = line.strip()
            if stripped and not stripped.startswith('//') and not stripped.startswith('#') and not stripped.startswith('/*') and not stripped.startswith('*'):
                if not stripped.startswith('use '):
                    insert_idx = i
                    break
                else:
                    insert_idx = i + 1

        lines.insert(insert_idx, "use serde::{Serialize, Deserialize};")
        content = '\n'.join(lines)

    # Now fix all #[derive(...)] that don't have serde traits
    # Find derive lines and add serde::Serialize, serde::Deserialize
    lines = content.split('\n')
    new_lines = []
    i = 0

    while i < len(lines):
        line = lines[i]
        stripped = line.strip()

        # Check for derive macro
        if stripped.startswith('#[derive(') and stripped.endswith(')'):
            # Check if any following lines have serde attribute
            has_serde = False
            for j in range(i+1, min(i+15, len(lines))):
                if '#[serde(' in lines[j]:
                    has_serde = True
                    break

            if has_serde:
                # Add serde traits if not present
                if 'serde::Serialize' not in line and 'Serialize' not in line:
                    derive_start = line.index('(') + 1
                    derive_end = line.rindex(')')
                    current = line[derive_start:derive_end]
                    new_derive = current.rstrip()
                    if new_derive:
                        new_derive += ', '
                    new_derive += 'serde::Serialize, serde::Deserialize'
                    line = line[:derive_start] + new_derive + line[derive_end:]
                    print(f"Fixed derive in {filename}")

        new_lines.append(line)
        i += 1

    content = '\n'.join(new_lines)

    if content != original:
        with open(filepath, 'w') as f:
            f.write(content)
        fixed_count += 1

print(f"Done! Fixed {fixed_count} files.")
