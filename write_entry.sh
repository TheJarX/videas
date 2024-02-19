#!/bin/bash

file="tmp.md"

vim "$file"

# Check if file exists
if [ -f "$file" ]; then
    title=$(awk 'NR==1' $file | sed 's!^[\t\s+#]*!!')
    content="$(< $file)"
    
    sqlite3 -line db.sqlite3 "INSERT INTO post VALUES(null, '$title', '$content', datetime('now'));"
    rm "$file"
fi

