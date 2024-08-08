#!/bin/bash

if files=$(
	cargo package --allow-dirty --list "$@" \
	| tr '\n' '|' \
	| sed 's:|$::g'
); then
	dir=$(tree -C -f --gitignore --prune)
	num=$(grep -P "$files" <<<"$dir" | wc -l)
	grep -C 10 -P "$files" <<<"$dir" --color=always
	echo "$num files to be packaged"
fi
