#!/bin/bash

# Function to extract the block wrapped with ====
extractMetadataBlock() {
  local filePath="$1"
  local fileContent=""
  local data=""

  # Read the file content
  fileContent=$(<"$filePath")

  # Extract the data between the ==== markers
  data=$(echo "$fileContent" | sed -n '/^====/,/^====/p')
  sed -i '' '/====/,/====/d' "$filePath"

  # Return the extracted block
  echo "$data"
}

extractTitle() {
  local block="$1"
  local title=""

  title=$(echo "$block" | sed -n 's!^title:\s*!!p' | xargs)
  echo "$title"
}

extractSlug() {
  local block="$1"
  local slug=""

  slug=$(echo "$block" | sed -n 's!^slug:\s*!!p' | xargs)
  echo "$slug"
}

extractTags() {
  local block="$1"
  # Extract tags using awk (expected format: "tags: Tag1, Tag2, ..., TagN")

  tagsContent=$(echo "$block" | sed -n 's!^tags:\s*!!p' | xargs)
  # Create an array from the tags
  IFS=', ' read -ra tagsArray <<< "$tagsContent"

  echo "${tagsArray[@]}"
}

