./runtest.exe -w entry-static.exe dlopen
./runtest.exe -w entry-static.exe env
./runtest.exe -w entry-static.exe search_hsearch
./runtest.exe -w entry-static.exe setjmp
./runtest.exe -w entry-static.exe sscanf_long
./runtest.exe -w entry-static.exe strftime
./runtest.exe -w entry-static.exe strptime
./runtest.exe -w entry-static.exe putenv_doublefree
./runtest.exe -w entry-static.exe syscall_sign_extend

./runtest.exe -w entry-dynamic.exe dlopen
./runtest.exe -w entry-dynamic.exe env
./runtest.exe -w entry-dynamic.exe search_hsearch
./runtest.exe -w entry-dynamic.exe setjmp
./runtest.exe -w entry-dynamic.exe sscanf_long
./runtest.exe -w entry-dynamic.exe strftime
./runtest.exe -w entry-dynamic.exe strptime
./runtest.exe -w entry-dynamic.exe putenv_doublefree
./runtest.exe -w entry-dynamic.exe syscall_sign_extend

# ========== START entry-dynamic.exe dlopen ==========
# src/functional/dlopen.c:18: dlopen ./dlopen_dso.so failed: Error loading shared library ./dlopen_dso.so: No such file or directory
# src/functional/dlopen.c:37: dlsym i should have failed
# src/functional/dlopen.c:39: dlsym main failed: Symbol not found: main
# src/functional/dlopen.c:53: dlclose failed: Invalid library handle 0
# FAIL dlopen [status 1]
# ========== END entry-dynamic.exe dlopen ==========

# ========== START entry-dynamic.exe env ==========
# src/regression/flockfile-list.c:55: 0 is not an allocated pointer
# FAIL env [status 1]
# ========== END entry-dynamic.exe env ==========

# ========== START entry-dynamic.exe search_hsearch ==========
# src/functional/search_hsearch.c:26: hcreate((size_t)-1) should fail with ENOMEM got No error information
# FAIL search_hsearch [status 1]
# ========== END entry-dynamic.exe search_hsearch ==========

# ========== START entry-dynamic.exe setjmp ==========
# src/functional/setjmp.c:62: sigismember(&set2, SIGUSR1)==1 failed: siglongjmp incorrectly restored mask
# FAIL setjmp [status 1]
# ========== END entry-dynamic.exe setjmp ==========

# ========== START entry-dynamic.exe sscanf_long ==========
# src/regression/flockfile-list.c:28: test buffer is small, pos: 0, need: 8388608
# FAIL sscanf_long [status 1]
# ========== END entry-dynamic.exe sscanf_long ==========

# ========== START entry-dynamic.exe strftime ==========
# src/regression/flockfile-list.c:55: 0 is not an allocated pointer
# FAIL strftime [status 1]
# ========== END entry-dynamic.exe strftime ==========

# ========== START entry-dynamic.exe strptime ==========
# src/regression/flockfile-list.c:55: 0 is not an allocated pointer
# FAIL strptime [status 1]
# ========== END entry-dynamic.exe strptime ==========

# ========== START entry-dynamic.exe putenv_doublefree ==========
# src/regression/flockfile-list.c:55: 0 is not an allocated pointer
# FAIL putenv_doublefree [status 1]
# ========== END entry-dynamic.exe putenv_doublefree ==========

# ========== START entry-dynamic.exe syscall_sign_extend ==========
# src/regression/syscall-sign-extend.c:21: (r = syscall(SYS_read, fd, buf, 1)) == 1 failed: No error information
# src/regression/syscall-sign-extend.c:23: read 1 instead of 0
# FAIL syscall_sign_extend [status 1]
# ========== END entry-dynamic.exe syscall_sign_extend ==========

