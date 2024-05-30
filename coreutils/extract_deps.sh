#!/bin/bash
CORPUS_DIR=$(pwd)
COREUTILS_DIR=/tmp/coreutils
PROG_DIR=src
PROG_NAME=echo
C_DIR=c
OUT_DIR="$PROG_NAME/$C_DIR"

echoerr() { echo "$@" 1>&2;  }

get_headers() {
    local c_files=("$@")
    local header_files=()
    for c_filepath in "${c_files[@]}"; do
        directory=${c_filepath#./}  # Remove the leading './'
        directory=${directory%/*}
        filename=${c_filepath##*/}
        filename=${filename%%.*}  # Remove extension (e.g. .c and .h)
        if [ "$directory" = "gnulib/lib" ]; then
            deps_filepath="./lib/.deps/libcoreutils_a-$filename.Po"
        elif [ "$directory" = "src" ]; then
            deps_filepath="./src/.deps/$filename.Po"
        else
            echoerr "Warning: File in an unknown directory: '$filepath'"
            continue
        fi
        if [ ! -e "$deps_filepath" ]; then
            echoerr "Warning: Dependency file $deps_filepath does not exist, skipping..."
            continue
        fi
        readarray -t cur_header_files <<< \
            "$(grep -E '^[^/][^:]+/[^:]+:$' "$deps_filepath" | sed 's/:$//' | sort -u)"
                    header_files=( "${header_files[@]}" "${cur_header_files[@]}" )
    done
    echo "$(printf "%s\n" "${header_files[@]}" | sort -u)"
}

get_c_files_from_headers() {
    local header_files=("$@")
    local c_files=()
    for header_file in "${header_files[@]}"; do
        filename="${header_file##*/}"
        filename="${filename%.h}"
        c_file=$(find ./ -type f -name "$filename.c" | head -n 1)
        if [ -z "$c_file" ]; then
            continue
        fi
        c_files+=( "$c_file" )
    done
    echo "$(printf "%s\n" "${c_files[@]}" | sort -u)"
}

echoerr "Creating the $OUT_DIR directory..."
rm -rf $PROG_NAME
mkdir -p $OUT_DIR

cd $COREUTILS_DIR
#echoerr "Making $PROG_NAME..."
#make -s clean &> /dev/null  # Silence the output
#make -s $PROG_DIR/$PROG_NAME &> /dev/null  # Silence the output

echoerr "Collecting all the .h and .c files..."
c_files=()
while IFS= read -r line; do
    c_source_file="$(echo "$line" | awk '{print $3}')"
    c_source_filepath=$(find ./ -type f -name "$c_source_file" | head -n 1)
    if [ -z "$c_source_filepath" ]; then
        continue
    fi
    cp -f "$c_source_filepath" -t "$CORPUS_DIR/$OUT_DIR"
    c_files+=( "$c_source_filepath" )
done < <(nm -a src/$PROG_NAME | grep ".*\.c$")

new_header_files=($(get_headers "${c_files[@]}"))
header_files=("${new_header_files[@]}")

i=1
while [ 1 ]; do
    echo "Loop $((i++))..."
    new_c_files=($(get_c_files_from_headers "${new_header_files[@]}"))
    old_len=${#c_files[@]}
    c_files=($(printf "%s\n" "${c_files[@]}" "${new_c_files[@]}" | sort -u))
    if [ ${#c_files[@]} -eq $old_len ]; then
        break
    fi
    new_header_files=($(get_headers "${new_c_files[@]}"))
    old_len=${#header_files[@]}
    header_files=($(printf "%s\n" "${header_files[@]}" "${new_header_files[@]}" | sort -u))
    if [ ${#header_files[@]} -eq $old_len ]; then
        break
    fi
done

cp -f "${c_files[@]}" -t "$CORPUS_DIR/$OUT_DIR"
cp -f "${header_files[@]}" -t "$CORPUS_DIR/$OUT_DIR"

# Create the makefile
echoerr "Creating the Makefile in $CORPUS_DIR/$OUT_DIR..."
cd $CORPUS_DIR
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
" > $OUT_DIR/Makefile
