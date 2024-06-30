#!/bin/sh
# Make sure touch can operate on a directory.
# This was broken in the 4.0[efg] test releases.

. "${srcdir=.}/tests/init.sh"; path_prepend_ $1

touch . || fail=1
Exit $fail
