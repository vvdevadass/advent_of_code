#!/bin/bash
template="p20xx_template"
new="$1"

if test ! -d $template -o -z "$new" -o -e "$new"; then
	echo "Usage: new.sh p2024_01"
	exit 1
fi

set -x
cp -fr ./$template ./$new
gfind ./$new -type f -printf "gsed -i \"s/$template/$new/g\" %p\n" | bash
gsed -i "/members/ a \    \"$new\"," ./Cargo.toml
git add .