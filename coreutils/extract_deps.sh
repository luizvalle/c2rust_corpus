#!/bin/bash
CORPUS_DIR=$(pwd)
COREUTILS_DIR=/tmp/coreutils
PROG_DIR=src
PROG_NAME=echo
C_DIR=c
OUT_DIR="$PROG_NAME/$C_DIR"

echoerr() { echo "$@" 1>&2;  }

echoerr "Creating the $OUT_DIR directory..."
rm -rf $PROG_NAME
mkdir -p $OUT_DIR

cd $COREUTILS_DIR
#echoerr "Making $PROG_NAME..."
#make -s clean &> /dev/null  # Silence the output
#make -s $PROG_DIR/$PROG_NAME &> /dev/null  # Silence the output

# Extract the .c files
echoerr "Copying the .c files to $OUT_DIR..."
c_source_filepaths=()
while IFS= read -r line; do
    c_source_file="$(echo "$line" | awk '{print $3}')"
    c_source_filepath=$(find ./ -type f -name "$c_source_file" | head -n 1)
    if [ -z "$c_source_filepath" ]; then
        continue
    fi
    cp -f "$c_source_filepath" -t "$CORPUS_DIR/$OUT_DIR/"
    c_source_filepaths+=( "$c_source_filepath" )
done < <(nm -a src/$PROG_NAME | grep ".*\.c$")

# Extract the .h files
echoerr "Copying the .h files to $OUT_DIR..."
for filepath in "${c_source_filepaths[@]}"; do
    directory=${filepath#./}  # Remove the leading './'
    directory=${directory%/*}
    filename=${filepath##*/}
    filename=${filename%%.*}  # Remove extension (e.g. .c and .h)
    if [ "$directory" = "gnulib/lib" ]; then
        deps_filepath="./lib/.deps/libcoreutils_a-$filename.Po"
    elif [ "$directory" = "src" ]; then
        deps_filepath="./src/.deps/$filename.Po"
    else
        echoerr "File in an unknown directory: $filepath"
        exit 1
    fi
    if [ ! -e "$deps_filepath" ]; then
        echoerr "Dependency file $deps_filepath does not exist"
        exit 1
    fi
    readarray -t header_files <<< \
        "$(grep -E '^[^/][^:]+/[^:]+:$' "$deps_filepath" | sed 's/:$//' | sort | uniq)"
    cp -f "${header_files[@]}" -t "$CORPUS_DIR/$OUT_DIR/" &> /dev/null
done

# Create the makefile
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
