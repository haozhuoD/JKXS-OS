MUSL_LIB?=/home/user/new-os-test/riscv64--musl--bleeding-edge-2020.08-1/riscv64-buildroot-linux-musl/sysroot/lib64/
PREFIX?=/home/user/new-os-test/riscv64--musl--bleeding-edge-2020.08-1/bin/riscv64-buildroot-linux-musl-
CC=$(PREFIX)gcc
OBJCOPY=$(PREFIX)objcopy
# CC=riscv64-linux-gnu-gcc
# CC=musl-gcc
# OBJCOPY=objcopy

STATIC_EXES = $(shell cat static.txt)
STATIC_OBJS = $(STATIC_EXES:%.exe=%.obj)
STATIC_PREFIXED_OBJS = $(STATIC_EXES:%.exe=%_prefixed.obj)

DYNAMIC_EXES = $(shell cat dynamic.txt)
DYNAMIC_OBJS = $(DYNAMIC_EXES:%.exe=%.obj)
DYNAMIC_PREFIXED_OBJS = $(DYNAMIC_EXES:%.exe=%_prefixed.obj)

COMMON_SRC_FILES = fdfill.c memfill.c mtest.c path.c print.c rand.c setrlim.c utf8.c vmfill.c
COMMON_SRCS = $(addprefix src/common/,  $(COMMON_SRC_FILES))
COMMON_OBJS = $(COMMON_SRCS:%.c=%.obj)

DSO_SRCS = src/functional/tls_align_dso.c src/functional/tls_init_dso.c src/functional/dlopen_dso.c src/regression/tls_get_new-dtv_dso.c
DSO_SOS = $(DSO_SRCS:%.c=%.so)


# CFLAGS += -pipe -std=c99 -D_POSIX_C_SOURCE=200809L -Wall -Wno-unused-function -Wno-missing-braces -Wno-unused -Wno-overflow
# CFLAGS += -Wno-unknown-pragmas -fno-builtin -frounding-math
# CFLAGS += -Werror=implicit-function-declaration -Werror=implicit-int -Werror=pointer-sign -Werror=pointer-arith
# CFLAGS += -O3 -g -s -static

CFLAGS += -Og -static
# CFLAGS += -static -g3
LDLIBS += -Os -s -lpthread -lm -lrt

# CFLAGS += -D_FILE_OFFSET_BITS=64
# LDLIBS += -lcrypt -ldl -lresolv -lutil -lpthread

all:
	$(CC) $(CFLAGS) signal_test.c -o signal_test
	riscv64-unknown-elf-objdump -d signal_test > signal.S
	cp signal_test ../busybox_lua_testsuites