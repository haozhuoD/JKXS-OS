./runtest.exe -w entry-static.exe tls_init
./runtest.exe -w entry-static.exe tls_local_exec
./runtest.exe -w entry-static.exe tls_get_new_dtv

# dynamic需要使用futex
# ./runtest.exe -w entry-static.exe sem_init
# ./runtest.exe -w entry-dynamic.exe sem_init

# ./runtest.exe -w entry-static.exe pthread_cancel_points
# ./runtest.exe -w entry-static.exe pthread_cancel
# ./runtest.exe -w entry-static.exe pthread_cond
# ./runtest.exe -w entry-static.exe pthread_tsd

# ./runtest.exe -w entry-static.exe pthread_robust_detach
# ./runtest.exe -w entry-static.exe pthread_cond_smasher
# ./runtest.exe -w entry-static.exe pthread_condattr_setclock
# ./runtest.exe -w entry-static.exe pthread_exit_cancel
# ./runtest.exe -w entry-static.exe pthread_once_deadlock
# ./runtest.exe -w entry-static.exe pthread_rwlock_ebusy




# ./runtest.exe -w entry-dynamic.exe sem_init
# ./runtest.exe -w entry-dynamic.exe tls_init
# ./runtest.exe -w entry-dynamic.exe tls_local_exec
# ./runtest.exe -w entry-dynamic.exe tls_get_new_dtv

# ./runtest.exe -w entry-dynamic.exe pthread_cancel_points
# ./runtest.exe -w entry-dynamic.exe pthread_cancel
# ./runtest.exe -w entry-dynamic.exe pthread_cond
# ./runtest.exe -w entry-dynamic.exe pthread_tsd

# ./runtest.exe -w entry-dynamic.exe pthread_robust_detach
# ./runtest.exe -w entry-dynamic.exe pthread_cond_smasher
# ./runtest.exe -w entry-dynamic.exe pthread_condattr_setclock
# ./runtest.exe -w entry-dynamic.exe pthread_exit_cancel
# ./runtest.exe -w entry-dynamic.exe pthread_once_deadlock
# ./runtest.exe -w entry-dynamic.exe pthread_rwlock_ebusy

