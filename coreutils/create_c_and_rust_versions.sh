#!/bin/bash
PROG_NAME=$1

CORPUS_DIR=$(pwd)
COREUTILS_DIR=/tmp/coreutils
PROG_DIR=src

PROG_DIR="$CORPUS_DIR/$PROG_NAME"
C_DIR="$PROG_DIR/c"
RUST_DIR="$PROG_DIR/rust"

echoerr() { printf "$@\n" 1>&2;  }

get_deps() {
    local filepath="$1"
    dir="${filepath%/*}"
    deps_filename="${filepath##*/}"
    deps_filename="${deps_filename%.*}"
    deps_filepath="$dir/.deps/$deps_filename.Po"
    if [ ! -e "$deps_filepath" ]; then
        echoerr "\tError: Could not find the dependency file '$deps_filepath'"
        exit 1
    fi
    deps="$(cat "$deps_filepath" | sed -E 's/[^[:space:]]+/&\n/g')"
    deps="$(echo "$deps" | sed '/^$/d')"
    deps="$(echo "$deps" | sed -E '/^\s*\//d; /:$/d; /\\/d')"
    deps="$(echo "$deps" | sed -E 's/^[[:space:]]+//; s/[[:space:]]+$//')"
    deps="$(echo "$deps" | sort -u | sed "s@^@$COREUTILS_DIR/@" | tr '\n' ' ')"
    echo "${deps[@]}"
}

map_symbols_to_dependencies() {
    local dir_to_search="$1"
    for object_filepath in "$dir_to_search"/*.o; do
        syms=($(nm --defined-only --extern-only --format="just-symbols" "$object_filepath"))
        if [ ${#syms[@]} -eq 0 ]; then
            continue
        fi
        deps="$(get_deps "$object_filepath")"
        for sym in "${syms[@]}"; do
            echo "$sym:$deps"
        done
    done
}

copy_files() {
    local filepaths=($@)
    for filepath in "${filepaths[@]}"; do
        if [[ "$filepath" =~ (lib|src)/([^/]+)/ ]]; then
            # Deal with subdirs like lib/sys
            dest_dir="$C_DIR/${BASH_REMATCH[2]}"
            mkdir -p "$dest_dir"
        else
            dest_dir="$C_DIR"
        fi
        cp -f "$filepath" -t "$dest_dir"
    done
}

if [ -z "$PROG_NAME" ]; then
    echoerr "Usage: $0 <program_name>"
    exit 1
fi

echoerr "Creating the $C_DIR directory..."
rm -rf "$C_DIR"
mkdir -p "$C_DIR"

echoerr "Creating the Makefile in $C_DIR"
echo "\
CFLAGS=-I./ -g
SRCS := \$(wildcard *.c)
OBJS := \$(SRCS:.c=.o)

.PHONY: all
all: clean $PROG_NAME

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
main_deps=($(get_deps "$MAIN_SOURCE_FILE"))
copy_files "${main_deps[@]}"
cd "$COREUTILS_DIR"
cp -f "$MAIN_SOURCE_FILE" "${main_header_files[@]}" -t "$C_DIR"
cd - &> /dev/null

echoerr "Mapping symbols in coreutils to their dependencies..."
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
            echoerr "\tWarning: Did not find dependencies for the missing symbol '$missing_symbol'"
            continue
        fi
        copy_files "${deps[@]}"
    done
done

echoerr "Successfully built the C version of $PROG_NAME in $C_DIR!"

echoerr "Creating the JSON database file for C2Rust..."
sb_install_msg="$(pip install scan-build 2>&1)"
if [ $? -ne 0 ]; then
    echoerr "\tError: Chould not install scan-build, which is used to create the compilation database: $sb_install_msg"
    exit 1
fi
make -s --directory="$C_DIR" clean
COMPILATION_DB_FILEPATH="$C_DIR/compile_commands.json"

ib_msg="$(intercept-build --cdb "$COMPILATION_DB_FILEPATH" make -s --directory="$C_DIR" "$PROG_NAME" 2>&1)"
if [ $? -ne 0 ]; then
    echoerr "\tError: Could not create the compilation database: $ib_msg"
    exit 1
fi
make -s --directory="$C_DIR" clean &> /dev/null

echoerr "Installing C2Rust, you may need to provide your password..."
c2r_deps_install_msg="$(sudo apt install build-essential llvm clang libclang-dev cmake libssl-dev pkg-config python3 git 2>&1)"
if [ $? -ne 0 ]; then
    echoerr "\tError: Could not install the dependencies for C2Rust: $c2r_deps_install_msg"
    exit 1
fi
c2r_install_msg="$(cargo install c2rust 2>&1)"
if [ $? -ne 0 ]; then
    echoerr "\tError: Could not install C2Rust: $c2r_install_msg"
fi

echoerr "Transpiling the C code to Rust and placing it in $RUST_DIR..."
rm -rf "$RUST_DIR"
c2r_msg="$(c2rust transpile \
    --binary $PROG_NAME \
    --overwrite-existing \
    --output-dir="$RUST_DIR" \
    "$COMPILATION_DB_FILEPATH" 2>&1)"
if [ $? -ne 0 ]; then
    echoerr "\tError: Could not transpile the C code: $c2r_msg"
    exit 1
fi
sed -i 's/^\(#!\[allow([^\)]*\))/\1, unused_imports)/' "$RUST_DIR/src/$PROG_NAME.rs"
find "$RUST_DIR/src" \
    -type f \
    -exec perl -0777 -pi -e 's/compile_error!\((.|\n)*?\)//g' {} +
cd "$RUST_DIR"
cf_msg="$(cargo fix \
    -Z unstable-options \
    --keep-going \
    --no-default-features \
    --broken-code \
    --allow-no-vcs \
    --allow-dirty \
    --allow-staged \
    --quiet 2>&1)"
if [ $? -ne 0 ]; then
    echoerr "\tError: Could not fix the errors in the Rust code: $cf_msg"
    exit 1
fi
cb_msg="$(cargo build 2>&1)"
if [ $? -ne 0 ]; then
    echoerr "\tError: Building the Rust version failed: $cb_msg"
    exit 1
fi
cargo clean &> /dev/null
cd - &> /dev/null

echoerr "Successfully built the Rust version of $PROG_NAME in $RUST_DIR!"

echoerr "Done!"
