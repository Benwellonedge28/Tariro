#!/usr/bin/env python3
"""Fix duplicate serde derives (Serialize, Deserialize, serde::Serialize, serde::Deserialize)."""

import os
import re

src_dir = "/workspace/Tariro/src"

# Find all .rs files
fixed_count = 0
for filename in os.listdir(src_dir):
    if filename.endswith(".rs"):
        filepath = os.path.join(src_dir, filename)
        with open(filepath, 'r') as f:
            content = f.read()

        original = content

        # Fix duplicate serde in derives: "Serialize, Deserialize, serde::Serialize, serde::Deserialize"
        # should be just "Serialize, Deserialize"
        content = re.sub(
            r'Serialize,\s*Deserialize,\s*serde::Serialize,\s*serde::Deserialize',
            'Serialize, Deserialize',
            content
        )

        # Also handle the reverse order
        content = re.sub(
            r'serde::Serialize,\s*serde::Deserialize,\s*Serialize,\s*Deserialize',
            'Serialize, Deserialize',
            content
        )

        if content != original:
            with open(filepath, 'w') as f:
                f.write(content)
            fixed_count += 1
            print(f"Fixed {filename}")

print(f"Done! Fixed {fixed_count} files.")
