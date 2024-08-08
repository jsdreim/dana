#!/bin/bash

if read -p "Publish package? [y/N] " confirm \
&& [[ "$confirm" == "y" ]]
then
	cargo publish -p dana_macros "$@" || exit
	cargo publish -p dana "$@" || exit
else
	echo "Cancelled."
fi
