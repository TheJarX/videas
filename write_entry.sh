#!/bin/bash

file="tmp.md"
# To provide flexibility and make it easier to test
dbPath="${1:-db.sqlite3}"

vim "$file"

# Check if file exists
if [ -f "$file" ]; then
  # import metadata helpers
  . "./bash/metadata.sh"
  metadataBlock=$(extractMetadataBlock "$file")

  abstract=$(extractDescription "$metadataBlock")
  title=$(extractTitle "$metadataBlock")
  slug=$(extractSlug "$metadataBlock")
  tags=$(extractTags "$metadataBlock")

  sqlite3 -line "$dbPath" \
    "INSERT INTO post VALUES( \
    null, \
    '$title', \
    readfile('$file'), \
    datetime('now'), \
    '$slug', \
    '$tags', \
    '$abstract' \
    );"
fi

trap 'rm -f "$file"' EXIT
