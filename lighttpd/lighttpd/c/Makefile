CFLAGS = -I./include -I./ \
    -g -DHAVE_CONFIG_H -DHAVE_VERSIONSTAMP_H \
    -DLIBRARY_DIR="\"/usr/local/lib\"" -DSBIN_DIR="\"/usr/local/sbin\"" \
    -D_REENTRANT  -D_TIME_BITS=64 -D_FILE_OFFSET_BITS=64 -D_LARGEFILE_SOURCE \
    -D_LARGE_FILES -D_DEFAULT_SOURCE
SRCS := $(wildcard *.c)
OBJS := $(SRCS:.c=.o)
LDLIBS = -lpcre2-8

.PHONY: all
all: clean lighttpd

lighttpd: $(OBJS)
	gcc $^ $(LDLIBS) -o $@

%.o: %.c
	gcc $(CFLAGS) -c $< -o $@

.PHONY: clean
clean:
	rm -rf $(OBJS) lighttpd
