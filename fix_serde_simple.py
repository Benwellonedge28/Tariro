#!/usr/bin/env python3
"""Fix serde issues with a simple, reliable approach."""

import os
import re

src_dir = "/workspace/Tariro/src"

# Get list of all files that need fixing (have #[serde])
files = []
for filename in os.listdir(src_dir):
    if filename.endswith(".rs") and filename != "lib.rs":
        filepath = os.path.join(src_dir, filename)
        with open(filepath, 'r') as f:
            content = f.read()
            if '#[serde(' in content:
                files.append(filename)

print(f"Found {len(files)} files with serde")

for filename in files:
    filepath = os.path.join(src_dir, filename)
    with open(filepath, 'r') as f:
        content = f.read()

    original = content

    # Step 1: Remove any misplaced serde import (not at top level)
    lines = content.split('\n')
    fixed_lines = []
    serde_import = None

    for i, line in enumerate(lines):
        stripped = line.strip()
        if stripped == 'use serde::{Serialize, Deserialize};':
            # Check if this comes after derive or struct - it's misplaced
            prev = ''
            for j in range(i-1, -1, -1):
                if lines[j].strip() and not lines[j].strip().startswith('//'):
                    prev = lines[j].strip()
                    break
            if prev.startswith('#[derive(') or prev.startswith('pub struct') or prev.startswith('struct '):
                serde_import = line  # Save for later
                continue  # Skip this line
        fixed_lines.append(line)

    content = '\n'.join(fixed_lines)

    # Step 2: Add serde import at top if not present
    if 'use serde::{' not in content and '#[macro_use] extern crate serde' not in content:
        lines = content.split('\n')
        # Find where to insert (after module docs and attributes, before first use statement)
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

    # Step 3: Add serde traits to derive macros that have serde fields
    lines = content.split('\n')
    fixed_lines = []
    i = 0

    while i < len(lines):
        line = lines[i]
        stripped = line.strip()

        # Check for derive macro
        derive_match = re.match(r'(#\[derive\()([^)]+)(\)\])', stripped)
        if derive_match:
            # Look ahead for serde attribute
            has_serde = False
            for j in range(i+1, min(i+15, len(lines))):
                if '#[serde(' in lines[j]:
                    has_serde = True
                    break

            if has_serde:
                traits = derive_match.group(2)
                if 'serde::Serialize' not in traits:
                    new_traits = traits.strip()
                    if new_traits:
                        new_traits += ', '
                    new_traits += 'serde::Serialize, serde::Deserialize'
                    new_derive = f'#[derive({new_traits})]'
                    # Preserve indentation
                    indent = len(line) - len(line.lstrip())
                    line = ' ' * indent + new_derive
                    print(f"Fixed derive in {filename}")

        fixed_lines.append(line)
        i += 1

    content = '\n'.join(fixed_lines)

    if content != original:
        with open(filepath, 'w') as f:
            f.write(content)
        print(f"Fixed {filename}")

print("Done!")
