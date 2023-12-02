#!/bin/bash

readme_file="README.md"
tag="\`\`\`"

# Extract content until the tag from the README
existing_content=$(sed -n "/$tag/q;p" "$readme_file")

# Run your command and store the output in a variable
new_output=$(cargo run --release)

# Update the README file with the existing content and new output
echo -e "$existing_content\n\n$tag\n$new_output\n$tag" > "$readme_file"

echo "README updated successfully!"
