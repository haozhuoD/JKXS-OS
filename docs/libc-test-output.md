<!-- ========== START entry-static.exe env ==========
src/regression/flockfile-list.c:55: 0 is not an allocated pointer
FAIL env [status 1]
========== END entry-static.exe env ========== -->

========== START entry-static.exe search_hsearch ==========
src/functional/search_hsearch.c:26: hcreate((size_t)-1) should fail with ENOMEM got No error information
FAIL search_hsearch [status 1]
========== END entry-static.exe search_hsearch ==========

========== START entry-static.exe sscanf_long ==========
src/regression/flockfile-list.c:28: test buffer is small, pos: 0, need: 8388608
FAIL sscanf_long [status 1]
========== END entry-static.exe sscanf_long ==========

<!-- ========== START entry-static.exe strftime ==========
src/regression/flockfile-list.c:55: 0 is not an allocated pointer
FAIL strftime [status 1]
========== END entry-static.exe strftime ========== -->

<!-- ========== START entry-static.exe strptime ==========
src/regression/flockfile-list.c:55: 0 is not an allocated pointer
FAIL strptime [status 1]
========== END entry-static.exe strptime ========== -->

<!-- ========== START entry-static.exe putenv_doublefree ==========
src/regression/flockfile-list.c:55: 0 is not an allocated pointer
FAIL putenv_doublefree [status 1]
========== END entry-static.exe putenv_doublefree ========== -->

========== START entry-dynamic.exe dlopen ==========
src/functional/dlopen.c:18: dlopen ./dlopen_dso.so failed: Error loading shared library ./dlopen_dso.so: No such file or directory
src/functional/dlopen.c:37: dlsym i should have failed
src/functional/dlopen.c:39: dlsym main failed: Symbol not found: main
src/functional/dlopen.c:53: dlclose failed: Invalid library handle 0
FAIL dlopen [status 1]
========== END entry-dynamic.exe dlopen ==========

<!-- ========== START entry-dynamic.exe env ==========
src/regression/flockfile-list.c:55: 0 is not an allocated pointer
FAIL env [status 1]
========== END entry-dynamic.exe env ========== -->

========== START entry-dynamic.exe search_hsearch ==========
src/functional/search_hsearch.c:26: hcreate((size_t)-1) should fail with ENOMEM got No error information
FAIL search_hsearch [status 1]
========== END entry-dynamic.exe search_hsearch ==========

========== START entry-dynamic.exe sscanf_long ==========
src/regression/flockfile-list.c:28: test buffer is small, pos: 0, need: 8388608
FAIL sscanf_long [status 1]
========== END entry-dynamic.exe sscanf_long ==========

<!-- ========== START entry-dynamic.exe strftime ==========
src/regression/flockfile-list.c:55: 0 is not an allocated pointer
FAIL strftime [status 1]
========== END entry-dynamic.exe strftime ========== -->

<!-- ========== START entry-dynamic.exe strptime ==========
src/regression/flockfile-list.c:55: 0 is not an allocated pointer
FAIL strptime [status 1]
========== END entry-dynamic.exe strptime ==========

========== START entry-dynamic.exe putenv_doublefree ==========
src/regression/flockfile-list.c:55: 0 is not an allocated pointer
FAIL putenv_doublefree [status 1]
========== END entry-dynamic.exe putenv_doublefree ========== -->

========== START entry-dynamic.exe tls_get_new_dtv ==========
src/regression/tls_get_new-dtv.c:32: mod = dlopen("tls_get_new-dtv_dso.so", RTLD_NOW) failed
FAIL tls_get_new_dtv [status 1]
========== END entry-dynamic.exe tls_get_new_dtv ==========
