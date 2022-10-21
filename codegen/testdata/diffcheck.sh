#!/bin/bash

set -e

FAILED=false

for dir in */; do
  if ! (
    FAILED=true
    echo "Generating code in $dir"
    mkdir -p "$dir/actual"
    cd "$dir/actual"
    apex generate ../apex.yaml
    cd ..
    echo "Checking $dir for diffs"
    diff -r ./expected ./actual
  ) ; then
    FAILED=true
  fi
  echo "------------------------------------"
done

if [ "$FAILED" = true ] ; then
  echo "Exiting with code -1 due to differences"
  exit -1
fi
