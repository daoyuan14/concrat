/* Generated by CIL v. 1.8.2 */
/* print_CIL_Input is true */

typedef unsigned long size_t;
typedef unsigned int __uint32_t;
typedef unsigned long __dev_t;
typedef unsigned int __uid_t;
typedef unsigned int __gid_t;
typedef unsigned long __ino_t;
typedef unsigned int __mode_t;
typedef unsigned long __nlink_t;
typedef long __off_t;
typedef long __off64_t;
typedef int __pid_t;
typedef long __clock_t;
typedef long __time_t;
typedef unsigned int __useconds_t;
typedef long __suseconds_t;
typedef int __clockid_t;
typedef void *__timer_t;
typedef long __blksize_t;
typedef long __blkcnt_t;
typedef long __syscall_slong_t;
struct _IO_FILE ;
typedef struct _IO_FILE FILE;
struct _IO_marker ;
struct _IO_codecvt ;
struct _IO_wide_data ;
typedef void _IO_lock_t;
struct _IO_FILE {
   int _flags ;
   char *_IO_read_ptr ;
   char *_IO_read_end ;
   char *_IO_read_base ;
   char *_IO_write_base ;
   char *_IO_write_ptr ;
   char *_IO_write_end ;
   char *_IO_buf_base ;
   char *_IO_buf_end ;
   char *_IO_save_base ;
   char *_IO_backup_base ;
   char *_IO_save_end ;
   struct _IO_marker *_markers ;
   struct _IO_FILE *_chain ;
   int _fileno ;
   int _flags2 ;
   __off_t _old_offset ;
   unsigned short _cur_column ;
   signed char _vtable_offset ;
   char _shortbuf[1] ;
   _IO_lock_t *_lock ;
   __off64_t _offset ;
   struct _IO_codecvt *_codecvt ;
   struct _IO_wide_data *_wide_data ;
   struct _IO_FILE *_freeres_list ;
   void *_freeres_buf ;
   size_t __pad5 ;
   int _mode ;
   char _unused2[(15UL * sizeof(int ) - 4UL * sizeof(void *)) - sizeof(size_t )] ;
};
typedef __clockid_t clockid_t;
typedef __time_t time_t;
typedef __timer_t timer_t;
struct __anonstruct___sigset_t_991265788 {
   unsigned long __val[1024UL / (8UL * sizeof(unsigned long ))] ;
};
typedef struct __anonstruct___sigset_t_991265788 __sigset_t;
typedef __sigset_t sigset_t;
struct timeval {
   __time_t tv_sec ;
   __suseconds_t tv_usec ;
};
struct timespec {
   __time_t tv_sec ;
   __syscall_slong_t tv_nsec ;
};
struct __pthread_internal_list {
   struct __pthread_internal_list *__prev ;
   struct __pthread_internal_list *__next ;
};
typedef struct __pthread_internal_list __pthread_list_t;
struct __pthread_mutex_s {
   int __lock ;
   unsigned int __count ;
   int __owner ;
   unsigned int __nusers ;
   int __kind ;
   short __spins ;
   short __elision ;
   __pthread_list_t __list ;
};
struct __anonstruct___wseq32_112954846 {
   unsigned int __low ;
   unsigned int __high ;
};
union __anonunion____missing_field_name_648058633 {
   unsigned long long __wseq ;
   struct __anonstruct___wseq32_112954846 __wseq32 ;
};
struct __anonstruct___g1_start32_366146944 {
   unsigned int __low ;
   unsigned int __high ;
};
union __anonunion____missing_field_name_366146943 {
   unsigned long long __g1_start ;
   struct __anonstruct___g1_start32_366146944 __g1_start32 ;
};
struct __pthread_cond_s {
   union __anonunion____missing_field_name_648058633 __annonCompField1 ;
   union __anonunion____missing_field_name_366146943 __annonCompField2 ;
   unsigned int __g_refs[2] ;
   unsigned int __g_size[2] ;
   unsigned int __g1_orig_size ;
   unsigned int __wrefs ;
   unsigned int __g_signals[2] ;
};
typedef unsigned long pthread_t;
union __anonunion_pthread_condattr_t_488594145 {
   char __size[4] ;
   int __align ;
};
typedef union __anonunion_pthread_condattr_t_488594145 pthread_condattr_t;
union pthread_attr_t {
   char __size[56] ;
   long __align ;
};
typedef union pthread_attr_t pthread_attr_t;
union __anonunion_pthread_mutex_t_335460617 {
   struct __pthread_mutex_s __data ;
   char __size[40] ;
   long __align ;
};
typedef union __anonunion_pthread_mutex_t_335460617 pthread_mutex_t;
union __anonunion_pthread_cond_t_951761805 {
   struct __pthread_cond_s __data ;
   char __size[48] ;
   long long __align ;
};
typedef union __anonunion_pthread_cond_t_951761805 pthread_cond_t;
struct itimerspec {
   struct timespec it_interval ;
   struct timespec it_value ;
};
struct sigevent ;
struct timeb {
   time_t time ;
   unsigned short millitm ;
   short timezone ;
   short dstflag ;
};
struct stat {
   __dev_t st_dev ;
   __ino_t st_ino ;
   __nlink_t st_nlink ;
   __mode_t st_mode ;
   __uid_t st_uid ;
   __gid_t st_gid ;
   int __pad0 ;
   __dev_t st_rdev ;
   __off_t st_size ;
   __blksize_t st_blksize ;
   __blkcnt_t st_blocks ;
   struct timespec st_atim ;
   struct timespec st_mtim ;
   struct timespec st_ctim ;
   __syscall_slong_t __glibc_reserved[3] ;
};
union sigval {
   int sival_int ;
   void *sival_ptr ;
};
typedef union sigval __sigval_t;
struct __anonstruct__kill_244518854 {
   __pid_t si_pid ;
   __uid_t si_uid ;
};
struct __anonstruct__timer_490064738 {
   int si_tid ;
   int si_overrun ;
   __sigval_t si_sigval ;
};
struct __anonstruct__rt_619254530 {
   __pid_t si_pid ;
   __uid_t si_uid ;
   __sigval_t si_sigval ;
};
struct __anonstruct__sigchld_284671705 {
   __pid_t si_pid ;
   __uid_t si_uid ;
   int si_status ;
   __clock_t si_utime ;
   __clock_t si_stime ;
};
struct __anonstruct__addr_bnd_5259977 {
   void *_lower ;
   void *_upper ;
};
union __anonunion__bounds_445958387 {
   struct __anonstruct__addr_bnd_5259977 _addr_bnd ;
   __uint32_t _pkey ;
};
struct __anonstruct__sigfault_2227063 {
   void *si_addr ;
   short si_addr_lsb ;
   union __anonunion__bounds_445958387 _bounds ;
};
struct __anonstruct__sigpoll_386613454 {
   long si_band ;
   int si_fd ;
};
struct __anonstruct__sigsys_44812255 {
   void *_call_addr ;
   int _syscall ;
   unsigned int _arch ;
};
union __anonunion__sifields_838399793 {
   int _pad[128UL / sizeof(int ) - 4UL] ;
   struct __anonstruct__kill_244518854 _kill ;
   struct __anonstruct__timer_490064738 _timer ;
   struct __anonstruct__rt_619254530 _rt ;
   struct __anonstruct__sigchld_284671705 _sigchld ;
   struct __anonstruct__sigfault_2227063 _sigfault ;
   struct __anonstruct__sigpoll_386613454 _sigpoll ;
   struct __anonstruct__sigsys_44812255 _sigsys ;
};
struct __anonstruct_siginfo_t_1066041944 {
   int si_signo ;
   int si_errno ;
   int si_code ;
   int __pad0 ;
   union __anonunion__sifields_838399793 _sifields ;
};
typedef struct __anonstruct_siginfo_t_1066041944 siginfo_t;
struct __anonstruct__sigev_thread_746770901 {
   void (*_function)(__sigval_t  ) ;
   pthread_attr_t *_attribute ;
};
union __anonunion__sigev_un_577170306 {
   int _pad[64UL / sizeof(int ) - 4UL] ;
   __pid_t _tid ;
   struct __anonstruct__sigev_thread_746770901 _sigev_thread ;
};
struct sigevent {
   __sigval_t sigev_value ;
   int sigev_signo ;
   int sigev_notify ;
   union __anonunion__sigev_un_577170306 _sigev_un ;
};
union __anonunion___sigaction_handler_363639592 {
   void (*sa_handler)(int  ) ;
   void (*sa_sigaction)(int  , siginfo_t * , void * ) ;
};
struct sigaction {
   union __anonunion___sigaction_handler_363639592 __sigaction_handler ;
   __sigset_t sa_mask ;
   int sa_flags ;
   void (*sa_restorer)(void) ;
};
#pragma merger("0","/tmp/cil-WKl8_rUC.i","-std=gnu99")
extern FILE *stdout ;
extern int fflush(FILE *__stream ) ;
extern int printf(char const   * __restrict  __format  , ...) ;
extern void perror(char const   *__s ) ;
extern  __attribute__((__nothrow__, __noreturn__)) void exit(int __status ) ;
extern  __attribute__((__nothrow__)) time_t time(time_t *__timer ) ;
extern  __attribute__((__nothrow__)) char *ctime(time_t const   *__timer ) ;
extern  __attribute__((__nothrow__)) int clock_gettime(clockid_t __clock_id , struct timespec *__tp ) ;
extern  __attribute__((__nothrow__)) int timer_create(clockid_t __clock_id , struct sigevent * __restrict  __evp ,
                                                      timer_t * __restrict  __timerid ) ;
extern  __attribute__((__nothrow__)) int timer_settime(timer_t __timerid , int __flags ,
                                                       struct itimerspec  const  * __restrict  __value ,
                                                       struct itimerspec * __restrict  __ovalue ) ;
extern  __attribute__((__nothrow__)) int timer_gettime(timer_t __timerid , struct itimerspec *__value ) ;
extern  __attribute__((__nothrow__)) int timer_getoverrun(timer_t __timerid ) ;
extern  __attribute__((__nothrow__)) int ( __attribute__((__nonnull__(1))) gettimeofday)(struct timeval * __restrict  __tv ,
                                                                                         void * __restrict  __tz ) ;
extern int ( __attribute__((__nonnull__(1))) ftime)(struct timeb *__timebuf )  __attribute__((__deprecated__)) ;
extern  __attribute__((__nothrow__)) int ( __attribute__((__nonnull__(1,2))) lstat)(char const   * __restrict  __file ,
                                                                                    struct stat * __restrict  __buf ) ;
extern unsigned int sleep(unsigned int __seconds ) ;
extern int usleep(__useconds_t __useconds ) ;
extern  __attribute__((__nothrow__)) int ( __attribute__((__nonnull__(1,3))) pthread_create)(pthread_t * __restrict  __newthread ,
                                                                                             pthread_attr_t const   * __restrict  __attr ,
                                                                                             void *(*__start_routine)(void * ) ,
                                                                                             void * __restrict  __arg ) ;
extern int pthread_join(pthread_t __th , void **__thread_return ) ;
extern  __attribute__((__nothrow__)) int ( __attribute__((__nonnull__(1))) pthread_mutex_lock)(pthread_mutex_t *__mutex ) ;
extern  __attribute__((__nothrow__)) int ( __attribute__((__nonnull__(1))) pthread_mutex_unlock)(pthread_mutex_t *__mutex ) ;
extern  __attribute__((__nothrow__)) int ( __attribute__((__nonnull__(1))) pthread_cond_init)(pthread_cond_t * __restrict  __cond ,
                                                                                              pthread_condattr_t const   * __restrict  __cond_attr ) ;
extern  __attribute__((__nothrow__)) int ( __attribute__((__nonnull__(1))) pthread_cond_destroy)(pthread_cond_t *__cond ) ;
extern int ( __attribute__((__nonnull__(1,2,3))) pthread_cond_timedwait)(pthread_cond_t * __restrict  __cond ,
                                                                         pthread_mutex_t * __restrict  __mutex ,
                                                                         struct timespec  const  * __restrict  __abstime ) ;
extern  __attribute__((__nothrow__)) int ( __attribute__((__nonnull__(1))) pthread_condattr_init)(pthread_condattr_t *__attr ) ;
extern  __attribute__((__nothrow__)) int ( __attribute__((__nonnull__(1))) pthread_condattr_setclock)(pthread_condattr_t *__attr ,
                                                                                                      __clockid_t __clock_id ) ;
extern  __attribute__((__nothrow__)) int ( __attribute__((__nonnull__(1))) sigemptyset)(sigset_t *__set ) ;
extern  __attribute__((__nothrow__)) int ( __attribute__((__nonnull__(1))) sigaddset)(sigset_t *__set ,
                                                                                      int __signo ) ;
extern  __attribute__((__nothrow__)) int sigprocmask(int __how , sigset_t const   * __restrict  __set ,
                                                     sigset_t * __restrict  __oset ) ;
extern  __attribute__((__nothrow__)) int sigaction(int __sig , struct sigaction  const  * __restrict  __act ,
                                                   struct sigaction * __restrict  __oact ) ;
static void handler(int sig , siginfo_t *si , void *uc ) 
{ 


  {
  if ((unsigned long )si == (unsigned long )((void *)0)) {
    printf((char const   * __restrict  )"Caught signal %d\n", sig);
  } else
  if ((unsigned long )si != (unsigned long )uc) {
    printf((char const   * __restrict  )"Caught signal %d\n", sig);
  }
  return;
}
}
void *pthread_test(void *args ) 
{ 
  pthread_mutex_t fakeMutex ;
  pthread_cond_t fakeCond ;
  pthread_cond_t monotonic_cond ;
  pthread_condattr_t attr ;
  struct timespec timeToWait ;
  struct timespec now ;
  int rt ;

  {
  fakeMutex.__data.__lock = 0;
  fakeMutex.__data.__count = 0U;
  fakeMutex.__data.__owner = 0;
  fakeMutex.__data.__nusers = 0U;
  fakeMutex.__data.__kind = 0;
  fakeMutex.__data.__spins = (short)0;
  fakeMutex.__data.__elision = (short)0;
  fakeMutex.__data.__list.__prev = (struct __pthread_internal_list *)0;
  fakeMutex.__data.__list.__next = (struct __pthread_internal_list *)0;
  fakeCond.__data.__annonCompField1.__wseq = 0ULL;
  fakeCond.__data.__annonCompField2.__g1_start = 0ULL;
  fakeCond.__data.__g_refs[0] = 0U;
  fakeCond.__data.__g_refs[1] = 0U;
  fakeCond.__data.__g_size[0] = 0U;
  fakeCond.__data.__g_size[1] = 0U;
  fakeCond.__data.__g1_orig_size = 0U;
  fakeCond.__data.__wrefs = 0U;
  fakeCond.__data.__g_signals[0] = 0U;
  fakeCond.__data.__g_signals[1] = 0U;
  args = args;
  clock_gettime(0, & now);
  timeToWait.tv_sec = now.tv_sec + 1L;
  timeToWait.tv_nsec = now.tv_nsec;
  printf((char const   * __restrict  )"pthread_cond_timedwait: CLOCK_REALTIME test\n");
  printf((char const   * __restrict  )"(Intentionally sleeping 1 second...)\n");
  fflush(stdout);
  pthread_mutex_lock(& fakeMutex);
  rt = pthread_cond_timedwait((pthread_cond_t * __restrict  )(& fakeCond), (pthread_mutex_t * __restrict  )(& fakeMutex),
                              (struct timespec  const  * __restrict  )(& timeToWait));
  if (rt != 110) {
    printf((char const   * __restrict  )"pthread_cond_timedwait failed\n");
    pthread_mutex_unlock(& fakeMutex);
    exit(1);
  }
  pthread_mutex_unlock(& fakeMutex);
  pthread_condattr_init(& attr);
  pthread_condattr_setclock(& attr, 1);
  pthread_cond_init((pthread_cond_t * __restrict  )(& monotonic_cond), (pthread_condattr_t const   * __restrict  )(& attr));
  clock_gettime(1, & now);
  timeToWait.tv_sec = now.tv_sec + 1L;
  timeToWait.tv_nsec = now.tv_nsec;
  printf((char const   * __restrict  )"pthread_cond_timedwait: CLOCK_MONOTONIC test\n");
  printf((char const   * __restrict  )"(Intentionally sleeping 1 second...)\n");
  printf((char const   * __restrict  )"(If this test hangs for more than a few seconds, please report\n your glibc version and system details as FORCE_MONOTONIC_FIX\n issue at https://github.com/wolfcw/libfaketime)\n");
  fflush(stdout);
  pthread_mutex_lock(& fakeMutex);
  rt = pthread_cond_timedwait((pthread_cond_t * __restrict  )(& monotonic_cond), (pthread_mutex_t * __restrict  )(& fakeMutex),
                              (struct timespec  const  * __restrict  )(& timeToWait));
  if (rt != 110) {
    printf((char const   * __restrict  )"pthread_cond_timedwait failed\n");
    pthread_mutex_unlock(& fakeMutex);
    exit(1);
  }
  pthread_mutex_unlock(& fakeMutex);
  pthread_cond_destroy(& monotonic_cond);
  return ((void *)0);
}
}
int main(int argc , char **argv ) 
{ 
  time_t now ;
  struct timeb tb ;
  struct timeval tv ;
  struct timespec ts ;
  timer_t timerid1 ;
  timer_t timerid2 ;
  struct sigevent sev ;
  struct itimerspec its ;
  sigset_t mask ;
  struct sigaction sa ;
  struct stat buf ;
  pthread_t thread ;
  void *ret ;
  int tmp ;
  int tmp___0 ;
  int tmp___1 ;
  int tmp___2 ;
  int tmp___3 ;
  int tmp___4 ;
  char *tmp___5 ;
  time_t tmp___6 ;
  char *tmp___7 ;
  char *tmp___8 ;
  int tmp___9 ;
  char *tmp___10 ;
  int timer_getoverrun_timerid1 ;
  int tmp___11 ;
  int timer_getoverrun_timerid2 ;
  int tmp___12 ;
  char *tmp___13 ;

  {
  timerid1 = (timer_t )0;
  printf((char const   * __restrict  )"%s", "");
  pthread_create((pthread_t * __restrict  )(& thread), (pthread_attr_t const   * __restrict  )((void *)0),
                 & pthread_test, (void * __restrict  )((void *)0));
  pthread_join(thread, & ret);
  sa.sa_flags = 4;
  sa.__sigaction_handler.sa_sigaction = & handler;
  sigemptyset(& sa.sa_mask);
  tmp = sigaction(10, (struct sigaction  const  * __restrict  )(& sa), (struct sigaction * __restrict  )((void *)0));
  if (tmp == -1) {
    perror("sigaction");
    exit(1);
  }
  printf((char const   * __restrict  )"Blocking signal %d\n", 10);
  sigemptyset(& mask);
  sigaddset(& mask, 10);
  tmp___0 = sigprocmask(2, (sigset_t const   * __restrict  )(& mask), (sigset_t * __restrict  )((void *)0));
  if (tmp___0 == -1) {
    perror("sigaction");
    exit(1);
  }
  sev.sigev_notify = 0;
  sev.sigev_signo = 10;
  sev.sigev_value.sival_ptr = (void *)(& timerid1);
  tmp___1 = timer_create(0, (struct sigevent * __restrict  )(& sev), (timer_t * __restrict  )(& timerid1));
  if (tmp___1 == -1) {
    perror("timer_create");
    exit(1);
  }
  its.it_value.tv_sec = (__time_t )1;
  its.it_value.tv_nsec = (__syscall_slong_t )0;
  its.it_interval.tv_sec = (__time_t )0;
  its.it_interval.tv_nsec = (__syscall_slong_t )300000000;
  tmp___2 = timer_settime(timerid1, 0, (struct itimerspec  const  * __restrict  )(& its),
                          (struct itimerspec * __restrict  )((void *)0));
  if (tmp___2 == -1) {
    perror("timer_settime");
    exit(1);
  }
  sev.sigev_value.sival_ptr = (void *)(& timerid2);
  tmp___3 = timer_create(0, (struct sigevent * __restrict  )(& sev), (timer_t * __restrict  )(& timerid2));
  if (tmp___3 == -1) {
    perror("timer_create");
    exit(1);
  }
  clock_gettime(0, & its.it_value);
  its.it_value.tv_sec += 3L;
  its.it_interval.tv_sec = (__time_t )0;
  its.it_interval.tv_nsec = (__syscall_slong_t )0;
  tmp___4 = timer_settime(timerid2, 1, (struct itimerspec  const  * __restrict  )(& its),
                          (struct itimerspec * __restrict  )((void *)0));
  if (tmp___4 == -1) {
    perror("timer_settime");
    exit(1);
  }
  time(& now);
  tmp___5 = ctime((time_t const   *)(& now));
  printf((char const   * __restrict  )"time()         : Current date and time: %s",
         tmp___5);
  tmp___6 = time((time_t *)((void *)0));
  printf((char const   * __restrict  )"time(NULL)     : Seconds since Epoch  : %u\n",
         (unsigned int )tmp___6);
  ftime(& tb);
  tmp___7 = ctime((time_t const   *)(& tb.time));
  printf((char const   * __restrict  )"ftime()        : Current date and time: %s",
         tmp___7);
  printf((char const   * __restrict  )"(Intentionally sleeping 2 seconds...)\n");
  fflush(stdout);
  if (argc < 3) {
    sleep(1U);
    usleep((__useconds_t )1000000);
  }
  gettimeofday((struct timeval * __restrict  )(& tv), (void * __restrict  )((void *)0));
  tmp___8 = ctime((time_t const   *)(& tv.tv_sec));
  printf((char const   * __restrict  )"gettimeofday() : Current date and time: %s",
         tmp___8);
  tmp___9 = sigprocmask(1, (sigset_t const   * __restrict  )(& mask), (sigset_t * __restrict  )((void *)0));
  if (tmp___9 == -1) {
    perror("sigprocmask");
    exit(1);
  }
  clock_gettime(0, & ts);
  tmp___10 = ctime((time_t const   *)(& ts.tv_sec));
  printf((char const   * __restrict  )"clock_gettime(): Current date and time: %s",
         tmp___10);
  tmp___11 = timer_getoverrun(timerid1);
  timer_getoverrun_timerid1 = tmp___11;
  if (timer_getoverrun_timerid1 != 3) {
    printf((char const   * __restrict  )"timer_getoverrun(timerid1) FAILED, must be 3 but got: %d\n",
           timer_getoverrun_timerid1);
  }
  timer_gettime(timerid1, & its);
  tmp___12 = timer_getoverrun(timerid2);
  timer_getoverrun_timerid2 = tmp___12;
  if (timer_getoverrun_timerid2 != 0) {
    printf((char const   * __restrict  )"timer_getoverrun(timerid2) FAILED, must be 0 but got: %d\n",
           timer_getoverrun_timerid2);
  }
  timer_gettime(timerid2, & its);
  lstat((char const   * __restrict  )*(argv + 0), (struct stat * __restrict  )(& buf));
  tmp___13 = ctime((time_t const   *)(& buf.st_mtim.tv_sec));
  printf((char const   * __restrict  )"stat(): mod. time of file \'%s\': %s", *(argv + 0),
         tmp___13);
  return (0);
}
}
#pragma merger("0","/tmp/cil-e4LdUfPy.i","-std=gnu99")
