#!/bin/bash
# Auto-generate all module declarations for lib.rs

echo "Generating module declarations..."

# Get all .rs files in src/ excluding lib.rs and main.rs
find src -name "*.rs" -type f ! -name "lib.rs" ! -name "main.rs" | sort | while read file; do
    # Convert path to module name
    # e.g., src/admin_omniversal_control/advanced_interface_ux/augmented_reality_ar_admin_interface.rs
    # becomes: pub mod admin_omniversal_control::advanced_interface_ux::augmented_reality_ar_admin_interface;
    
    rel_path="${file#src/}"
    rel_path="${rel_path%.rs}"
    
    # Replace / with ::
    module_path="${rel_path//\//::}"
    
    echo "pub mod ${module_path};"
done > all_modules.txt

echo "Generated $(wc -l < all_modules.txt) module declarations"
