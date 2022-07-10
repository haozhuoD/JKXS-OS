MUSL_LIB?=/home/user/new-os-test/riscv64--musl--bleeding-edge-2020.08-1/riscv64-buildroot-linux-musl/sysroot/lib64/
PREFIX?=/home/user/new-os-test/riscv64--musl--bleeding-edge-2020.08-1/bin/riscv64-buildroot-linux-musl-
CC=$(PREFIX)gcc
OBJCOPY=$(PREFIX)objcopy
# CC=riscv64-linux-gnu-gcc
# CC=musl-gcc
# OBJCOPY=objcopy

# CFLAGS += -pipe -std=c99 -D_POSIX_C_SOURCE=200809L -Wall -Wno-unused-function -Wno-missing-braces -Wno-unused -Wno-overflow
# CFLAGS += -Wno-unknown-pragmas -fno-builtin -frounding-math
# CFLAGS += -Werror=implicit-function-declaration -Werror=implicit-int -Werror=pointer-sign -Werror=pointer-arith
# CFLAGS += -O3 -g -s -static

CFLAGS += -Og -static
# CFLAGS += -static -g3
LDLIBS += -Os -s -lpthread -lm -lrt

# CFLAGS += -D_FILE_OFFSET_BITS=64
# LDLIBS += -lcrypt -ldl -lresolv -lutil -lpthread

signal_test:
	$(CC) $(CFLAGS) signal_test.c  $(LDLIBS) -o signal_test
	riscv64-unknown-elf-objdump -d signal_test > signal.S
	cp signal_test ../busybox_lua_testsuites