CFLAGS=-I./include -g
SRCS := $(wildcard *.c)
OBJS := $(SRCS:.c=.o)
LDFLAGS= -lcrypto -lssl

.PHONY: all
all: clean sort

sort: $(OBJS)
	gcc $^ $(LDFLAGS) -o $@

%.o: %.c
	gcc $(CFLAGS) -c $< -o $@

.PHONY: clean
clean:
	rm -rf $(OBJS) sort
