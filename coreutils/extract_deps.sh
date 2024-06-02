#!/bin/bash
CORPUS_DIR=$(pwd)
COREUTILS_DIR=/tmp/coreutils
PROG_DIR=src
PROG_NAME=echo

PROG_DIR="$CORPUS_DIR/$PROG_NAME"
C_DIR="$PROG_DIR/c"

echoerr() { printf "$@\n" 1>&2;  }

get_headers() {
    local c_filepath="$1"
    directory="${c_filepath%/*}"
    directory="${directory##*/}"
    filename=${c_filepath##*/}
    filename=${filename%%.*}  # Remove extension (e.g. .c and .h)
    if [ "$directory" = "gnulib/lib" ]; then
        deps_filepath="$COREUTILS_DIR/lib/.deps/libcoreutils_a-$filename.Po"
    elif [ "$directory" = "src" ]; then
        deps_filepath="$COREUTILS_DIR/src/.deps/$filename.Po"
    else
        echoerr "Warning: File in an unknown directory: '$filepath'"
        echo ""
        return
    fi
    if [ ! -e "$deps_filepath" ]; then
        echoerr "Warning: Dependency file $deps_filepath does not exist, skipping..."
        echo ""
        return
    fi
    readarray -t header_files <<< \
        "$(grep -E '^[^/][^:]+/[^:]+:$' "$deps_filepath" | sed 's/:$//' | sort -u)"
    echo "$(printf "%s\n" "${header_files[@]}" | sed '${/^$/d}' | sort -u)"
}

#echoerr "Making $PROG_NAME..."
#
#make -s clean &> /dev/null
#make -s --directory=$COREUTILS_DIR $PROG_DIR/$PROG_NAME -j 6 &> /dev/null

echoerr "Creating the $C_DIR directory..."
rm -rf "$C_DIR"
mkdir -p "$C_DIR"

echoerr "Creating the Makefile in $C_DIR"
echo "\
CFLAGS=-I./
SRCS := \$(wildcard *.c)
OBJS := \$(SRCS:.c=.o)

.PHONY: all
all: clean echo

$PROG_NAME: \$(OBJS)
	gcc $^ -o \$@

%.o: %.c
	gcc \$(CFLAGS) -c $< -o \$@

.PHONY: clean
clean:
	rm -rf \$(OBJS) $PROG_NAME
" > $C_DIR/Makefile

MAIN_SOURCE_FILE="$COREUTILS_DIR/src/$PROG_NAME.c"
echoerr "Copying $MAIN_SOURCE_FILE and the header files for it..."
main_header_files=($(get_headers "$MAIN_SOURCE_FILE"))
cd "$COREUTILS_DIR"
cp "$MAIN_SOURCE_FILE" "${main_header_files[@]}" "$C_DIR"
cd -

echoerr "Mapping symbols in coreutils to .c files"
declare -A symbol_to_source
SEARCH_PATTERN="s@.* \\([^ ]*\\.c\\).*@$COREUTILS_DIR/\1@p"
for object_filepath in "$COREUTILS_DIR/lib"/*.o; do
    syms=($(nm --defined-only --extern-only --format="just-symbols" "$object_filepath"))
    if [ ${#syms[@]} -eq 0 ]; then
        continue
    fi
    dir="${object_filepath%/*}"
    object_file_name="${object_filepath##*/}"
    object_file_name="${object_file_name%.o}"
    deps_filepath="$dir/.deps/$object_file_name.Po"
    if [ ! -e "$deps_filepath" ]; then
        echoerr "\tError: Could not find the dependency file '$deps_filepath'"
        exit 1
    fi
    deps="$(cat "$deps_filepath" | sed -E 's/[^[:space:]]+/&\n/g')"
    deps="$(echo "$deps" | sed '/^$/d')"
    deps="$(echo "$deps" | sed -E '/^\s*\//d; /:$/d; /\\/d')"
    deps="$(echo "$deps" | sed -E 's/^[[:space:]]+//; s/[[:space:]]+$//')"
    deps="$(echo "$deps" | sort -u)"
    for sym in "${syms[@]}"; do
        symbol_to_source["$sym"]="$deps"
    done
done

for key in "${!symbol_to_source[@]}"; do
    arr=${symbol_to_source[$key]}
    echo "$key -> ${arr[@]}"
done

#echoerr "Copying remaining .c and .h files"
#make_output="$(make --directory=$C_DIR $PROG_NAME -i 2>&1)"
#missing_symbols=($(echo "$make_output" \
#    | sed -n "s/.*undefined reference to \`\(.*\)'.*/\1/p"))
#missing_headers=($(echo "$make_output" \
#    | sed -n 's/.*fatal error: \(.*\): No such file or directory$/\1/p'))
#
#echo "Missing symbols: ${missing_symbols[@]}"
#echo "Missing headers: ${missing_headers[@]}"

#echoerr "Collecting all the .h and .c files..."
#echoerr "\tCollecting the initial .c files..."
#c_files=()
#while IFS= read -r line; do
#    c_source_file="$(echo "$line" | awk '{print $3}')"
#    c_source_filepath=$(find ./ -type f -name "$c_source_file" | head -n 1)
#    if [ -z "$c_source_filepath" ]; then
#        continue
#    fi
#    cp -f "$c_source_filepath" -t "$CORPUS_DIR/$OUT_DIR"
#    c_files+=( "$c_source_filepath" )
#done < <(nm -a src/$PROG_NAME | grep ".*\.c$")
#
#new_header_files=($(get_headers "${c_files[@]}"))
#header_files=("${new_header_files[@]}")
#
#echo "${c_files[@]}"
#
#cp "${c_files[@]}" "${header_files[@]}" "$CORPUS_DIR/$OUT_DIR"

#i=1
#while [ 1 ]; do
#    echoerr "\tLoop $((i++))..."
#    new_c_files=($(get_c_files_from_headers "${new_header_files[@]}"))
#    old_len=${#c_files[@]}
#    c_files=($(printf "%s\n" "${c_files[@]}" "${new_c_files[@]}" | sort -u))
#    if [ ${#c_files[@]} -eq $old_len ]; then
#        break
#    fi
#    new_header_files=($(get_headers "${new_c_files[@]}"))
#    old_len=${#header_files[@]}
#    header_files=($(printf "%s\n" "${header_files[@]}" "${new_header_files[@]}" | sort -u))
#    if [ ${#header_files[@]} -eq $old_len ]; then
#        break
#    fi
#    break
#done

#cp -f "${c_files[@]}" -t "$CORPUS_DIR/$OUT_DIR"
#cp -f "${header_files[@]}" -t "$CORPUS_DIR/$OUT_DIR"

echoerr "Done!"
