#!/bin/bash


main() {
	if command -v "as-tree" >/dev/null; then
		pkg_tree dana_macros
		echo
		pkg_tree dana .
	else
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
	fi
}


color_file() {
#	sed <<<"$1" -rn "s:([^/]+$):\x1B[$2m\1\x1B[m:p"
	sed <<<"$1" -rn "s:([^/.]+)(\.[^/]+)?$:\x1B[$2m\1\x1B[$3m\2\x1B[m:p"
}


pkg_tree() {
	local pkg_name="$1"
	local pkg_dir="${2:-$pkg_name}"

	local IFS=$'\n'
	local files=($(cargo package --allow-dirty --list -p "$pkg_name"))
	local paths=()

	# Process list of files.
	for file in ${files[@]}; do
		paths+=("$pkg_dir/$file")
	done

	# Display packaged files as tree.
	echo -n "Package '$pkg_name': "
	for path in ${paths[@]}; do
		if [[ -e "$path" ]]; then
			# Path exists; Color it green.
			color_file "$path" "92" "32"
		else
			# Path does not exist; Color it yellow.
			color_file "$path" "93" "33"
		fi
	done \
	| sed -r 's:^\./::g' \
	| as-tree -f

	# Report number of packaged files.
	echo "Files in package '$pkg_name': ${#paths[@]}"
}


main
