/* Generated by CIL v. 1.8.2 */
/* print_CIL_Input is true */

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
typedef unsigned long pthread_t;
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
#pragma merger("0","/tmp/cil-fWMbt7H4.i","-pthread")
extern int ( __attribute__((__nonnull__(1,3))) pthread_create)(pthread_t * __restrict  __newthread ,
                                                               pthread_attr_t const   * __restrict  __attr ,
                                                               void *(*__start_routine)(void * ) ,
                                                               void * __restrict  __arg )  __attribute__((__nothrow__)) ;
extern int pthread_join(pthread_t __th , void **__thread_return ) ;
extern int ( __attribute__((__nonnull__(1))) pthread_mutex_lock)(pthread_mutex_t *__mutex )  __attribute__((__nothrow__)) ;
extern int ( __attribute__((__nonnull__(1))) pthread_mutex_unlock)(pthread_mutex_t *__mutex )  __attribute__((__nothrow__)) ;
int n1  =    0;
int n2  =    0;
int n3  =    1;
pthread_mutex_t num_mutex  =    {{0, 0U, 0, 0U, 0, (short)0, (short)0, {(struct __pthread_internal_list *)0, (struct __pthread_internal_list *)0}}};
void f1(void) 
{ 
  int x ;

  {
  x = n3;
  pthread_mutex_lock(& num_mutex);
  n1 += x;
  n2 += x;
  pthread_mutex_unlock(& num_mutex);
  return;
}
}
void *t_fun(void *arg ) 
{ 


  {
  f1();
  return ((void *)0);
}
}
int main(void) 
{ 
  pthread_t id1 ;
  pthread_t id2 ;

  {
  pthread_create((pthread_t * __restrict  )(& id1), (pthread_attr_t const   * __restrict  )((void *)0),
                 & t_fun, (void * __restrict  )((void *)0));
  pthread_create((pthread_t * __restrict  )(& id2), (pthread_attr_t const   * __restrict  )((void *)0),
                 & t_fun, (void * __restrict  )((void *)0));
  pthread_join(id1, (void **)((void *)0));
  pthread_join(id2, (void **)((void *)0));
  return (0);
}
}
#pragma merger("0","/tmp/cil-fiscXdF1.i","-pthread")
