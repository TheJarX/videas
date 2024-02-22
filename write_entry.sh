#!/bin/bash

slugify() {
  text="$1"
  echo "$text" | iconv -t ascii//TRANSLIT | sed -E -e 's/[^[:alnum:]]+/-/g' -e 's/^-+|-+$//g' | tr '[:upper:]' '[:lower:]'
}

file="./tmp.md"

vim "$file"

# Check if file exists
if [ -f "$file" ]; then
    # xargs just in case (to trim spaces)
    title=$(awk 'NR==1' $file | sed 's!^[\t+\s+#]*!!' | xargs)
    # TODO: Extract subtract of entry to use in index
    # TODO: Add support for more metadata (covers, tags, links, etc)
    slug="$(slugify "$title")"
    
    sqlite3 -line db.sqlite3 \
      "INSERT INTO post VALUES( \
        null, \
        '$title', \
        readfile('$file'), \
        datetime('now'), \
        '$slug' \
      );"
    rm "$file"
fi

