#!/usr/bin/env python3
"""Fix serde issues properly - correct placement of imports and derive macros."""

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
            if '#[serde(' in content:
                files_with_serde.append(filename)

print(f"Found {len(files_with_serde)} files with #[serde( attribute")

fixed_count = 0
for filename in files_with_serde:
    filepath = os.path.join(src_dir, filename)
    with open(filepath, 'r') as f:
        content = f.read()

    original = content

    # First, fix the misplaced serde import (move it to top)
    # Find the misplaced use serde line
    lines = content.split('\n')
    misplaced_serde_import = None
    new_lines = []

    for i, line in enumerate(lines):
        stripped = line.strip()
        # Check if this is a misplaced serde import (after derive or module line)
        if stripped == 'use serde::{Serialize, Deserialize};':
            # Check if previous non-empty, non-comment line was a derive or struct
            prev_idx = i - 1
            while prev_idx >= 0 and (lines[prev_idx].strip() == '' or lines[prev_idx].strip().startswith('//')):
                prev_idx -= 1
            if prev_idx >= 0:
                prev = lines[prev_idx].strip()
                if prev.startswith('#[derive(') or prev.startswith('pub struct') or prev.startswith('struct'):
                    # This is misplaced - skip it, will add at top later
                    misplaced_serde_import = line
                    continue

        new_lines.append(line)

    if misplaced_serde_import:
        lines = new_lines
        content = '\n'.join(lines)

    # Now fix: add serde::Serialize, serde::Deserialize to derive macros
    lines = content.split('\n')
    new_lines = []
    i = 0

    while i < len(lines):
        line = lines[i]
        stripped = line.strip()

        # Check if this is a derive macro
        if stripped.startswith('#[derive(') and stripped.endswith(')'):
            # Look ahead for serde attribute
            has_serde = False
            for j in range(i+1, min(i+15, len(lines))):
                if '#[serde(' in lines[j]:
                    has_serde = True
                    break

            if has_serde:
                # Add serde traits if not present
                if 'serde::Serialize' not in stripped:
                    # Parse and modify the derive
                    derive_match = re.match(r'(#\[derive\()(.*)(\)\])', stripped)
                    if derive_match:
                        traits = derive_match.group(2)
                        new_traits = traits.strip()
                        if new_traits:
                            new_traits += ', '
                        new_traits += 'serde::Serialize, serde::Deserialize'
                        new_derive = f'#[derive({new_traits})]'
                        line = line.replace(stripped, new_derive)
                        print(f"Fixed derive in {filename}")

        new_lines.append(line)
        i += 1

    content = '\n'.join(new_lines)

    # Add serde import at the top if not present (and file uses serde)
    if 'use serde::' not in content and '#[macro_use] extern crate serde' not in content:
        lines = content.split('\n')
        new_lines = []
        insert_idx = 0

        for i, line in enumerate(lines):
            stripped = line.strip()
            if stripped and not stripped.startswith('//') and not stripped.startswith('#') and not stripped.startswith('/*') and not stripped.startswith('*'):
                # First non-doc, non-attribute line
                if not stripped.startswith('use '):
                    insert_idx = i
                    break
                else:
                    insert_idx = i + 1

        lines.insert(insert_idx, "use serde::{Serialize, Deserialize};")
        content = '\n'.join(lines)

    if content != original:
        with open(filepath, 'w') as f:
            f.write(content)
        fixed_count += 1
        print(f"Fixed {filename}")

print(f"Done! Fixed {fixed_count} files.")
