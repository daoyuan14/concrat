use ::libc;
extern "C" {
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_pthread_mutex_t_335460617 {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type pthread_mutex_t = __anonunion_pthread_mutex_t_335460617;
pub static mut x: libc::c_int = 0 as libc::c_int;
pub static mut n: libc::c_int = 0 as libc::c_int;
pub static mut m: pthread_mutex_t = __anonunion_pthread_mutex_t_335460617 {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_uint,
            __kind: 0 as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
pub unsafe extern "C" fn f1() {
    if x != 0 {
        pthread_mutex_lock(&mut m);
        n += 1;
    }
}
pub unsafe extern "C" fn f2() {
    if x != 0 {
        pthread_mutex_lock(&mut m);
    } else {
        pthread_mutex_lock(&mut m);
    }
    n += 1;
}
pub unsafe extern "C" fn f3() {
    if x != 0 {
        n += 1;
        pthread_mutex_unlock(&mut m);
    }
}
pub unsafe extern "C" fn f4() {
    n += 1;
    if x != 0 {
        pthread_mutex_unlock(&mut m);
    } else {
        pthread_mutex_unlock(&mut m);
    };
}
pub unsafe extern "C" fn f5() {
    pthread_mutex_lock(&mut m);
    n += 1;
    if x != 0 {
        pthread_mutex_unlock(&mut m);
    } else {
        pthread_mutex_unlock(&mut m);
    };
}
pub unsafe extern "C" fn f6() {
    if x != 0 {
        pthread_mutex_lock(&mut m);
    } else {
        pthread_mutex_lock(&mut m);
    }
    n += 1;
    pthread_mutex_unlock(&mut m);
}
pub unsafe extern "C" fn f7() {
    if x != 0 {
        n += 1;
        pthread_mutex_unlock(&mut m);
        pthread_mutex_lock(&mut m);
        n += 1;
    }
}
unsafe fn main_0() -> libc::c_int {
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
