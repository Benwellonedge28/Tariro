#!/usr/bin/env python3
"""Fix serde attribute issues by adding derive macros to structs."""

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

print(f"Found {len(files_with_serde)} files needing serde derive")

fixed_count = 0
for filename in files_with_serde:
    filepath = os.path.join(src_dir, filename)
    with open(filepath, 'r') as f:
        content = f.read()

    original = content

    # Find all struct definitions with serde(with) attribute
    # Pattern: struct_name followed by #[serde(with...)] blocks before { or ;
    pattern = r'(#[serde\(with = "chrono::serde::ts_seconds"\)\][\s\n]*)(struct \w+)'

    def add_derive(match):
        serde_attr = match.group(1)
        struct_def = match.group(2)
        # Add derive before the serde attribute
        return f'#[derive(serde::Serialize, serde::Deserialize)]\n{serde_attr}{struct_def}'

    content = re.sub(pattern, add_derive, content)

    # Also handle cases where serde comes first (derive not yet added)
    # Look for struct blocks that have serde with but no derive
    lines = content.split('\n')
    new_lines = []
    i = 0
    while i < len(lines):
        line = lines[i]

        # Check if this line has serde(with) attribute
        if 'serde(with = "chrono::serde::ts_seconds")' in line:
            # Look back to see if derive was already added
            look_back = 2
            found_derive = False
            for j in range(max(0, i-2), i):
                if 'derive(serde::Serialize' in lines[j]:
                    found_derive = True
                    break

            # If no derive found, insert it
            if not found_derive:
                # Find the struct keyword above
                struct_idx = -1
                for j in range(max(0, i-5), i):
                    if lines[j].strip().startswith('struct '):
                        struct_idx = j
                        break

                if struct_idx >= 0:
                    # Check if derive already exists on the line with struct
                    if 'derive(' not in lines[struct_idx]:
                        lines[struct_idx] = lines[struct_idx].replace(
                            'struct ',
                            '#[derive(serde::Serialize, serde::Deserialize)]\nstruct '
                        )
        i += 1

    content = '\n'.join(lines)

    # Alternative approach: add derive before each struct that has serde(with)
    # Split by struct keyword and process
    parts = content.split('\n')
    new_parts = []
    i = 0
    while i < len(parts):
        line = parts[i]

        # Check if this is a struct definition
        if line.strip().startswith('struct ') and not line.strip().startswith('struct &'):
            # Check if derive is already there
            if i > 0 and 'derive(serde::Serialize' in parts[i-1]:
                new_parts.append(line)
            elif '#[derive(serde::Serialize' in line:
                new_parts.append(line)
            else:
                # Check next few lines for serde with attribute
                has_serde = False
                for j in range(i+1, min(i+5, len(parts))):
                    if 'serde(with = "chrono::serde::ts_seconds")' in parts[j]:
                        has_serde = True
                        break

                if has_serde:
                    # Insert derive before struct
                    indent = len(line) - len(line.lstrip())
                    derive_line = ' ' * indent + '#[derive(serde::Serialize, serde::Deserialize)]'
                    new_parts.append(derive_line)
                    new_parts.append(line)
                else:
                    new_parts.append(line)
        else:
            new_parts.append(line)
        i += 1

    content = '\n'.join(new_parts)

    if content != original:
        with open(filepath, 'w') as f:
            f.write(content)
        fixed_count += 1
        print(f"Fixed {filename}")

print(f"Done! Fixed {fixed_count} files.")
