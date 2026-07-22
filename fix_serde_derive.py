#!/usr/bin/env python3
"""Fix serde derive issues by adding Serialize, Deserialize to struct derives."""

import os
import re

src_dir = "/workspace/Tariro/src"

# Find all .rs files with serde(with = ...) attribute
files_with_serde = []
for filename in os.listdir(src_dir):
    if filename.endswith(".rs") and filename != "lib.rs":
        filepath = os.path.join(src_dir, filename)
        with open(filepath, 'r') as f:
            content = f.read()
            if 'serde(with = "chrono::serde::ts_seconds")' in content:
                files_with_serde.append(filename)

print(f"Found {len(files_with_serde)} files needing serde derives")

fixed_count = 0
for filename in files_with_serde:
    filepath = os.path.join(src_dir, filename)
    with open(filepath, 'r') as f:
        content = f.read()

    original = content

    # Find all struct definitions and their derives
    # Match structs that have serde(with) in their fields
    lines = content.split('\n')
    new_lines = []
    i = 0

    while i < len(lines):
        line = lines[i]
        stripped = line.strip()

        # Check if this is a derive line for a struct
        if stripped.startswith('#[derive('):
            # Look ahead for struct with serde fields
            struct_idx = -1
            for j in range(i+1, min(i+10, len(lines))):
                if 'serde(with = "chrono::serde::ts_seconds")' in lines[j]:
                    struct_idx = j
                    break

            if struct_idx > 0:
                # This derive needs Serialize, Deserialize
                if 'Serialize' not in line or 'Deserialize' not in line:
                    # Add to existing derive
                    if line.rstrip().endswith(')'):
                        # It's a single line derive
                        derive_content = line[line.index('(')+1:line.rindex(')')]
                        parts = [p.strip() for p in derive_content.split(',')]
                        new_parts = []
                        for p in parts:
                            if 'serde' not in p.lower():
                                new_parts.append(p)
                        new_parts.extend(['serde::Serialize', 'serde::Deserialize'])
                        new_line = stripped.split('(')[0] + '(' + ', '.join(new_parts) + ')'
                        line = new_line
                        fixed_count += 1
                        print(f"Fixed derive in {filename}")

        new_lines.append(line)
        i += 1

    content = '\n'.join(new_lines)

    if content != original:
        with open(filepath, 'w') as f:
            f.write(content)

print(f"Done! Fixed {fixed_count} struct derives.")
