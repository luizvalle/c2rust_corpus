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

map_symbols_to_dependencies() {
    local dir_to_search="$1"
    for object_filepath in "$dir_to_search"/*.o; do
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
        deps="$(echo "$deps" | sort -u | sed "s@^@$COREUTILS_DIR/@" | tr '\n' ' ')"
        for sym in "${syms[@]}"; do
            echo "$sym:$deps"
        done
    done
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
cp -f "$MAIN_SOURCE_FILE" "${main_header_files[@]}" -t "$C_DIR"
cd -

echoerr "Mapping symbols in coreutils to their dependencies..."
#while IFS=: read -r sym deps; do
#    symbol_to_deps["$sym"]="$deps"
#done <<< "$(map_symbols_to_dependencies "$COREUTILS_DIR/lib")"
#while IFS=: read -r sym deps; do
#    symbol_to_deps["$sym"]="$deps"
#done <<< "$(map_symbols_to_dependencies "$COREUTILS_DIR/src")"
echoerr "\tMapping $COREUTILS_DIR/lib..."
lib_syms_map="$(map_symbols_to_dependencies "$COREUTILS_DIR/lib")"
echoerr "\tMapping $COREUTILS_DIR/src..."
src_syms_map="$(map_symbols_to_dependencies "$COREUTILS_DIR/src")"
syms_map="$(printf "%s\n%s" "$lib_syms_map" "$src_syms_map")"

declare -A symbol_to_deps
while IFS=: read -r sym deps; do
    symbol_to_deps["$sym"]="$deps"
done <<< "$syms_map"

echoerr "Copying the remaining .c and .h files..."
i=1
while [ 1 ]; do
    make_output="$(make --directory="$C_DIR" "$PROG_NAME" -i 2>&1)"
    missing_symbols=($(echo "$make_output" \
        | sed -n "s/.*undefined reference to \`\(.*\)'.*/\1/p"))
    num_missing_symbols=${#missing_symbols}
    if [ $num_missing_symbols -eq 0 ]; then
        break
    fi
    echoerr "\tIteration $((i++)): Number of missing symbols: $num_missing_symbols"
    for missing_symbol in "${missing_symbols[@]}"; do
        deps=(${symbol_to_deps["$missing_symbol"]})
        if [ ${#deps[@]} -eq 0 ]; then
            echoerr "\tError: Did not find dependencies for the missing symbol '$missing_symbol'"
            exit 1
        fi
        cp -f "${deps[@]}" -t "$C_DIR"
    done
done

echoerr "Successfully built the C version of $PROG_NAME in $C_DIR"

echoerr "Creating the Rust version of $PROG_NAME..."

echoerr "Done!"
