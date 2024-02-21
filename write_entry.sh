#!/bin/bash

file="tmp.md"

vim "$file"

# Check if file exists
if [ -f "$file" ]; then
    title=$(awk 'NR==1' $file | sed 's!^[\t\s+#]*!!')
    # TODO: Extract subtract of entry to use in index
    # TODO: Add support for tags
    # TODO: Add support for more metadata (covers, tags, links, etc)
    content="$(< $file)"
    
    sqlite3 -line db.sqlite3 "INSERT INTO post VALUES(null,\"$title\", \"$content\",  datetime('now'));"
    rm "$file"
fi

