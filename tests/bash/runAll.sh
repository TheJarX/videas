#!/bin/bash

TESTING_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
THIS_FILE=$(basename ${BASH_SOURCE[0]})

echo "Running all tests located in $TESTING_DIR"
for test in $(ls "$TESTING_DIR"/*.sh); do
  if [ "$(basename $test)" == "$THIS_FILE" ]; then
    continue
  fi
  echo "Executing file: $test"
  $test
  echo ""
done

