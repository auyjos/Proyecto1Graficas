#!/bin/bash

# Script to convert all PNG images in assets folder to RGBA format
# This ensures consistent pixel format for the raycaster engine

echo "Converting all PNG images to RGBA format..."

# Function to convert a single image to RGBA
convert_to_rgba() {
    local input_file="$1"
    local output_file="${input_file%.*}_rgba.png"
    
    # Skip if already converted
    if [[ "$input_file" == *"_rgba.png" ]]; then
        echo "  Skipping $input_file (already RGBA)"
        return
    fi
    
    # Skip if RGBA version already exists
    if [[ -f "$output_file" ]]; then
        echo "  Skipping $input_file (RGBA version exists)"
        return
    fi
    
    echo "  Converting: $input_file -> $output_file"
    convert "$input_file" -alpha on "$output_file"
    
    if [[ $? -eq 0 ]]; then
        echo "    ✓ Success"
    else
        echo "    ✗ Failed"
    fi
}

# Convert images in main assets folder
echo "Converting main assets folder..."
for file in assets/*.png; do
    if [[ -f "$file" ]]; then
        convert_to_rgba "$file"
    fi
done

# Convert images in textures subfolder
echo "Converting textures folder..."
for file in assets/textures/*.png; do
    if [[ -f "$file" ]]; then
        convert_to_rgba "$file"
    fi
done

# Convert images in textures/cloth subfolder
echo "Converting textures/cloth folder..."
for file in assets/textures/cloth/*.png; do
    if [[ -f "$file" ]]; then
        convert_to_rgba "$file"
    fi
done

# Convert images in textures/elements subfolder
echo "Converting textures/elements folder..."
for file in assets/textures/elements/*.png; do
    if [[ -f "$file" ]]; then
        convert_to_rgba "$file"
    fi
done

# Convert images in textures/metals subfolder
echo "Converting textures/metals folder..."
for file in assets/textures/metals/*.png; do
    if [[ -f "$file" ]]; then
        convert_to_rgba "$file"
    fi
done

echo ""
echo "Conversion complete!"
echo ""
echo "Summary of RGBA files created:"
find assets -name "*_rgba.png" | sort

echo ""
echo "You can now update your textures.rs to use the _rgba.png versions for consistent rendering."
