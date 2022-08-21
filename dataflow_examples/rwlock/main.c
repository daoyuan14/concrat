/* Generated by CIL v. 1.8.2 */
/* print_CIL_Input is true */

struct __pthread_rwlock_arch_t {
   unsigned int __readers ;
   unsigned int __writers ;
   unsigned int __wrphase_futex ;
   unsigned int __writers_futex ;
   unsigned int __pad3 ;
   unsigned int __pad4 ;
   int __cur_writer ;
   int __shared ;
   signed char __rwelision ;
   unsigned char __pad1[7] ;
   unsigned long __pad2 ;
   unsigned int __flags ;
};
union __anonunion_pthread_rwlock_t_656928968 {
   struct __pthread_rwlock_arch_t __data ;
   char __size[56] ;
   long __align ;
};
typedef union __anonunion_pthread_rwlock_t_656928968 pthread_rwlock_t;
#pragma merger("0","/tmp/cil-RSG5uVFz.i","-pthread")
extern  __attribute__((__nothrow__)) int ( __attribute__((__nonnull__(1))) pthread_rwlock_rdlock)(pthread_rwlock_t *__rwlock ) ;
extern  __attribute__((__nothrow__)) int ( __attribute__((__nonnull__(1))) pthread_rwlock_wrlock)(pthread_rwlock_t *__rwlock ) ;
extern  __attribute__((__nothrow__)) int ( __attribute__((__nonnull__(1))) pthread_rwlock_unlock)(pthread_rwlock_t *__rwlock ) ;
int n  =    0;
pthread_rwlock_t lock  =    {{0U, 0U, 0U, 0U, 0U, 0U, 0, 0, (signed char)0, {(unsigned char)0, (unsigned char)0,
                                                    (unsigned char)0, (unsigned char)0,
                                                    (unsigned char)0, (unsigned char)0,
                                                    (unsigned char)0}, 0UL, 0U}};
void rdlock(void) 
{ 


  {
  pthread_rwlock_rdlock(& lock);
  return;
}
}
void wrlock(void) 
{ 


  {
  pthread_rwlock_wrlock(& lock);
  return;
}
}
void unlock(void) 
{ 


  {
  pthread_rwlock_unlock(& lock);
  return;
}
}
void f1(void) 
{ 
  int x ;

  {
  pthread_rwlock_rdlock(& lock);
  x = n;
  pthread_rwlock_unlock(& lock);
  x ++;
  pthread_rwlock_wrlock(& lock);
  n += x;
  pthread_rwlock_unlock(& lock);
  rdlock();
  x = n;
  unlock();
  x ++;
  wrlock();
  n += x;
  unlock();
  return;
}
}
int main(void) 
{ 


  {
  return (0);
}
}
#pragma merger("0","/tmp/cil-bHGg0qKC.i","-pthread")
