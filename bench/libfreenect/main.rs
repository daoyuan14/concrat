use ::libc;
extern "C" {
    pub type _freenect_context;
    pub type _freenect_device;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn freenect_init(
        ctx: *mut *mut freenect_context,
        usb_ctx: *mut libc::c_void,
    ) -> libc::c_int;
    fn freenect_shutdown(ctx: *mut freenect_context) -> libc::c_int;
    fn freenect_set_log_level(ctx: *mut freenect_context, level: freenect_loglevel);
    fn freenect_process_events(ctx: *mut freenect_context) -> libc::c_int;
    fn freenect_num_devices(ctx: *mut freenect_context) -> libc::c_int;
    fn freenect_select_subdevices(
        ctx: *mut freenect_context,
        subdevs: freenect_device_flags,
    );
    fn freenect_open_device(
        ctx: *mut freenect_context,
        dev: *mut *mut freenect_device,
        index: libc::c_int,
    ) -> libc::c_int;
    fn freenect_close_device(dev: *mut freenect_device) -> libc::c_int;
    fn freenect_set_depth_callback(
        dev: *mut freenect_device,
        cb: Option::<
            unsafe extern "C" fn(*mut freenect_device, *mut libc::c_void, uint32_t) -> (),
        >,
    );
    fn freenect_set_video_callback(
        dev: *mut freenect_device,
        cb: Option::<
            unsafe extern "C" fn(*mut freenect_device, *mut libc::c_void, uint32_t) -> (),
        >,
    );
    fn freenect_set_video_buffer(
        dev: *mut freenect_device,
        buf: *mut libc::c_void,
    ) -> libc::c_int;
    fn freenect_start_depth(dev: *mut freenect_device) -> libc::c_int;
    fn freenect_start_video(dev: *mut freenect_device) -> libc::c_int;
    fn freenect_stop_depth(dev: *mut freenect_device) -> libc::c_int;
    fn freenect_stop_video(dev: *mut freenect_device) -> libc::c_int;
    fn freenect_find_video_mode(
        res: freenect_resolution,
        fmt: freenect_video_format,
    ) -> freenect_frame_mode;
    fn freenect_set_video_mode(
        dev: *mut freenect_device,
        mode: freenect_frame_mode,
    ) -> libc::c_int;
    fn freenect_find_depth_mode(
        res: freenect_resolution,
        fmt: freenect_depth_format,
    ) -> freenect_frame_mode;
    fn freenect_set_depth_mode(
        dev: *mut freenect_device,
        mode: freenect_frame_mode,
    ) -> libc::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
    fn powf(_: libc::c_float, _: libc::c_float) -> libc::c_float;
    fn glClearColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf);
    fn glBlendFunc(sfactor: GLenum, dfactor: GLenum);
    fn glEnable(cap: GLenum);
    fn glDisable(cap: GLenum);
    fn glClearDepth(depth: GLclampd);
    fn glDepthFunc(func: GLenum);
    fn glDepthMask(flag: GLboolean);
    fn glMatrixMode(mode: GLenum);
    fn glOrtho(
        left: GLdouble,
        right: GLdouble,
        bottom: GLdouble,
        top: GLdouble,
        near_val: GLdouble,
        far_val: GLdouble,
    );
    fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
    fn glLoadIdentity();
    fn glBegin(mode: GLenum);
    fn glEnd();
    fn glVertex3f(x: GLfloat, y: GLfloat, z: GLfloat);
    fn glColor4f(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
    fn glTexCoord2f(s: GLfloat, t: GLfloat);
    fn glShadeModel(mode: GLenum);
    fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint);
    fn glTexImage2D(
        target: GLenum,
        level: GLint,
        internalFormat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_0: GLenum,
        pixels: *const libc::c_void,
    );
    fn glGenTextures(n: GLsizei, textures: *mut GLuint);
    fn glBindTexture(target: GLenum, texture: GLuint);
    fn glutInit(pargc: *mut libc::c_int, argv: *mut *mut libc::c_char);
    fn glutInitWindowPosition(x: libc::c_int, y: libc::c_int);
    fn glutInitWindowSize(width: libc::c_int, height: libc::c_int);
    fn glutInitDisplayMode(displayMode: libc::c_uint);
    fn glutMainLoop();
    fn glutCreateWindow(title: *const libc::c_char) -> libc::c_int;
    fn glutDestroyWindow(window_0: libc::c_int);
    fn glutPostRedisplay();
    fn glutSwapBuffers();
    fn glutIdleFunc(callback: Option::<unsafe extern "C" fn() -> ()>);
    fn glutKeyboardFunc(
        callback: Option::<
            unsafe extern "C" fn(libc::c_uchar, libc::c_int, libc::c_int) -> (),
        >,
    );
    fn glutReshapeFunc(
        callback: Option::<unsafe extern "C" fn(libc::c_int, libc::c_int) -> ()>,
    );
    fn glutDisplayFunc(callback: Option::<unsafe extern "C" fn() -> ()>);
    fn glutGet(query: GLenum) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
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
pub struct __anonstruct___wseq32_112954846 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_937450124 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: __anonstruct___wseq32_112954846,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct___g1_start32_677907541 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_677907540 {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: __anonstruct___g1_start32_677907541,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __annonCompField1: __anonunion____missing_field_name_937450124,
    pub __annonCompField2: __anonunion____missing_field_name_677907540,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_pthread_mutex_t_335460617 {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type pthread_mutex_t = __anonunion_pthread_mutex_t_335460617;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_pthread_cond_t_951761805 {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
pub type pthread_cond_t = __anonunion_pthread_cond_t_951761805;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type __anonenum_freenect_device_flags_931223624 = libc::c_uint;
pub const FREENECT_DEVICE_AUDIO: __anonenum_freenect_device_flags_931223624 = 4;
pub const FREENECT_DEVICE_CAMERA: __anonenum_freenect_device_flags_931223624 = 2;
pub const FREENECT_DEVICE_MOTOR: __anonenum_freenect_device_flags_931223624 = 1;
pub type freenect_device_flags = __anonenum_freenect_device_flags_931223624;
pub type __anonenum_freenect_resolution_1036235832 = libc::c_uint;
pub const FREENECT_RESOLUTION_DUMMY: __anonenum_freenect_resolution_1036235832 = 2147483647;
pub const FREENECT_RESOLUTION_HIGH: __anonenum_freenect_resolution_1036235832 = 2;
pub const FREENECT_RESOLUTION_MEDIUM: __anonenum_freenect_resolution_1036235832 = 1;
pub const FREENECT_RESOLUTION_LOW: __anonenum_freenect_resolution_1036235832 = 0;
pub type freenect_resolution = __anonenum_freenect_resolution_1036235832;
pub type __anonenum_freenect_video_format_922200655 = libc::c_uint;
pub const FREENECT_VIDEO_DUMMY: __anonenum_freenect_video_format_922200655 = 2147483647;
pub const FREENECT_VIDEO_YUV_RAW: __anonenum_freenect_video_format_922200655 = 6;
pub const FREENECT_VIDEO_YUV_RGB: __anonenum_freenect_video_format_922200655 = 5;
pub const FREENECT_VIDEO_IR_10BIT_PACKED: __anonenum_freenect_video_format_922200655 = 4;
pub const FREENECT_VIDEO_IR_10BIT: __anonenum_freenect_video_format_922200655 = 3;
pub const FREENECT_VIDEO_IR_8BIT: __anonenum_freenect_video_format_922200655 = 2;
pub const FREENECT_VIDEO_BAYER: __anonenum_freenect_video_format_922200655 = 1;
pub const FREENECT_VIDEO_RGB: __anonenum_freenect_video_format_922200655 = 0;
pub type freenect_video_format = __anonenum_freenect_video_format_922200655;
pub type __anonenum_freenect_depth_format_454778101 = libc::c_uint;
pub const FREENECT_DEPTH_DUMMY: __anonenum_freenect_depth_format_454778101 = 2147483647;
pub const FREENECT_DEPTH_MM: __anonenum_freenect_depth_format_454778101 = 5;
pub const FREENECT_DEPTH_REGISTERED: __anonenum_freenect_depth_format_454778101 = 4;
pub const FREENECT_DEPTH_10BIT_PACKED: __anonenum_freenect_depth_format_454778101 = 3;
pub const FREENECT_DEPTH_11BIT_PACKED: __anonenum_freenect_depth_format_454778101 = 2;
pub const FREENECT_DEPTH_10BIT: __anonenum_freenect_depth_format_454778101 = 1;
pub const FREENECT_DEPTH_11BIT: __anonenum_freenect_depth_format_454778101 = 0;
pub type freenect_depth_format = __anonenum_freenect_depth_format_454778101;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_633756896 {
    pub dummy: int32_t,
    pub video_format: freenect_video_format,
    pub depth_format: freenect_depth_format,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_freenect_frame_mode_402645106 {
    pub reserved: uint32_t,
    pub resolution: freenect_resolution,
    pub __annonCompField3: __anonunion____missing_field_name_633756896,
    pub bytes: int32_t,
    pub width: int16_t,
    pub height: int16_t,
    pub data_bits_per_pixel: int8_t,
    pub padding_bits_per_pixel: int8_t,
    pub framerate: int8_t,
    pub is_valid: int8_t,
}
pub type freenect_frame_mode = __anonstruct_freenect_frame_mode_402645106;
pub type freenect_context = _freenect_context;
pub type freenect_device = _freenect_device;
pub type freenect_usb_context = ();
pub type __anonenum_freenect_loglevel_817188290 = libc::c_uint;
pub const FREENECT_LOG_FLOOD: __anonenum_freenect_loglevel_817188290 = 7;
pub const FREENECT_LOG_SPEW: __anonenum_freenect_loglevel_817188290 = 6;
pub const FREENECT_LOG_DEBUG: __anonenum_freenect_loglevel_817188290 = 5;
pub const FREENECT_LOG_INFO: __anonenum_freenect_loglevel_817188290 = 4;
pub const FREENECT_LOG_NOTICE: __anonenum_freenect_loglevel_817188290 = 3;
pub const FREENECT_LOG_WARNING: __anonenum_freenect_loglevel_817188290 = 2;
pub const FREENECT_LOG_ERROR: __anonenum_freenect_loglevel_817188290 = 1;
pub const FREENECT_LOG_FATAL: __anonenum_freenect_loglevel_817188290 = 0;
pub type freenect_loglevel = __anonenum_freenect_loglevel_817188290;
pub type GLenum = libc::c_uint;
pub type GLboolean = libc::c_uchar;
pub type GLvoid = ();
pub type GLint = libc::c_int;
pub type GLuint = libc::c_uint;
pub type GLsizei = libc::c_int;
pub type GLfloat = libc::c_float;
pub type GLclampf = libc::c_float;
pub type GLdouble = libc::c_double;
pub type GLclampd = libc::c_double;
pub static mut freenect_thread: pthread_t = 0;
pub static mut die: libc::c_int = 0 as libc::c_int;
pub static mut g_argc: libc::c_int = 0;
pub static mut g_argv: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
pub static mut window: libc::c_int = 0;
pub static mut gl_backbuf_mutex: pthread_mutex_t = __anonunion_pthread_mutex_t_335460617 {
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
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
pub static mut depth_mid: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
pub static mut depth_front: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
pub static mut rgb_back: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
pub static mut rgb_mid: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
pub static mut rgb_front: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
pub static mut gl_depth_tex: GLuint = 0;
pub static mut gl_rgb_tex: GLuint = 0;
pub static mut f_ctx: *mut freenect_context = 0 as *const freenect_context
    as *mut freenect_context;
pub static mut f_dev: *mut freenect_device = 0 as *const freenect_device
    as *mut freenect_device;
pub static mut freenect_angle: libc::c_int = 0 as libc::c_int;
pub static mut freenect_led: libc::c_int = 0;
pub static mut gl_frame_cond: pthread_cond_t = __anonunion_pthread_cond_t_951761805 {
    __data: {
        let mut init = __pthread_cond_s {
            __annonCompField1: __anonunion____missing_field_name_937450124 {
                __wseq: 0 as libc::c_ulonglong,
            },
            __annonCompField2: __anonunion____missing_field_name_677907540 {
                __g1_start: 0 as libc::c_ulonglong,
            },
            __g_refs: [0 as libc::c_uint, 0 as libc::c_uint],
            __g_size: [0 as libc::c_uint, 0 as libc::c_uint],
            __g1_orig_size: 0 as libc::c_uint,
            __wrefs: 0 as libc::c_uint,
            __g_signals: [0 as libc::c_uint, 0 as libc::c_uint],
        };
        init
    },
};
pub static mut got_rgb: libc::c_int = 0 as libc::c_int;
pub static mut got_depth: libc::c_int = 0 as libc::c_int;
pub static mut frame: libc::c_int = 0 as libc::c_int;
pub static mut my_ftime: libc::c_int = 0 as libc::c_int;
pub static mut fps: libc::c_double = 0 as libc::c_int as libc::c_double;
pub unsafe extern "C" fn idle() {
    pthread_mutex_lock(&mut gl_backbuf_mutex);
    while got_depth == 0 {
        if got_rgb != 0 {
            break;
        }
        pthread_cond_wait(
            &mut gl_frame_cond as *mut pthread_cond_t,
            &mut gl_backbuf_mutex as *mut pthread_mutex_t,
        );
    }
    if got_depth == 0 {
        pthread_mutex_unlock(&mut gl_backbuf_mutex);
        return;
    } else {
        if got_rgb == 0 {
            pthread_mutex_unlock(&mut gl_backbuf_mutex);
            return;
        }
    }
    pthread_mutex_unlock(&mut gl_backbuf_mutex);
    glutPostRedisplay();
}
pub unsafe extern "C" fn DrawGLScene() {
    let mut tmp: *mut uint8_t = 0 as *mut uint8_t;
    let mut ms: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    pthread_mutex_lock(&mut gl_backbuf_mutex);
    if got_depth != 0 {
        tmp = depth_front;
        depth_front = depth_mid;
        depth_mid = tmp;
        got_depth = 0 as libc::c_int;
    }
    if got_rgb != 0 {
        tmp = rgb_front;
        rgb_front = rgb_mid;
        rgb_mid = tmp;
        got_rgb = 0 as libc::c_int;
    }
    pthread_mutex_unlock(&mut gl_backbuf_mutex);
    glBindTexture(3553 as libc::c_int as GLenum, gl_rgb_tex);
    glTexImage2D(
        3553 as libc::c_int as GLenum,
        0 as libc::c_int,
        3 as libc::c_int,
        640 as libc::c_int,
        480 as libc::c_int,
        0 as libc::c_int,
        6407 as libc::c_int as GLenum,
        5121 as libc::c_int as GLenum,
        rgb_front as *const libc::c_void,
    );
    glBegin(6 as libc::c_int as GLenum);
    glColor4f(1.0f32, 1.0f32, 1.0f32, 1.0f32);
    glTexCoord2f(0 as libc::c_int as GLfloat, 0 as libc::c_int as GLfloat);
    glVertex3f(
        0 as libc::c_int as GLfloat,
        0 as libc::c_int as GLfloat,
        0 as libc::c_int as GLfloat,
    );
    glTexCoord2f(1 as libc::c_int as GLfloat, 0 as libc::c_int as GLfloat);
    glVertex3f(
        640 as libc::c_int as GLfloat,
        0 as libc::c_int as GLfloat,
        0 as libc::c_int as GLfloat,
    );
    glTexCoord2f(1 as libc::c_int as GLfloat, 1 as libc::c_int as GLfloat);
    glVertex3f(
        640 as libc::c_int as GLfloat,
        480 as libc::c_int as GLfloat,
        0 as libc::c_int as GLfloat,
    );
    glTexCoord2f(0 as libc::c_int as GLfloat, 1 as libc::c_int as GLfloat);
    glVertex3f(
        0 as libc::c_int as GLfloat,
        480 as libc::c_int as GLfloat,
        0 as libc::c_int as GLfloat,
    );
    glEnd();
    glBindTexture(3553 as libc::c_int as GLenum, gl_depth_tex);
    glTexImage2D(
        3553 as libc::c_int as GLenum,
        0 as libc::c_int,
        4 as libc::c_int,
        640 as libc::c_int,
        480 as libc::c_int,
        0 as libc::c_int,
        6408 as libc::c_int as GLenum,
        5121 as libc::c_int as GLenum,
        depth_front as *const libc::c_void,
    );
    glBegin(6 as libc::c_int as GLenum);
    glColor4f(1.0f32, 1.0f32, 1.0f32, 1.0f32);
    glTexCoord2f(0 as libc::c_int as GLfloat, 0 as libc::c_int as GLfloat);
    glVertex3f(
        0 as libc::c_int as GLfloat,
        0 as libc::c_int as GLfloat,
        0 as libc::c_int as GLfloat,
    );
    glTexCoord2f(1 as libc::c_int as GLfloat, 0 as libc::c_int as GLfloat);
    glVertex3f(
        640 as libc::c_int as GLfloat,
        0 as libc::c_int as GLfloat,
        0 as libc::c_int as GLfloat,
    );
    glTexCoord2f(1 as libc::c_int as GLfloat, 1 as libc::c_int as GLfloat);
    glVertex3f(
        640 as libc::c_int as GLfloat,
        480 as libc::c_int as GLfloat,
        0 as libc::c_int as GLfloat,
    );
    glTexCoord2f(0 as libc::c_int as GLfloat, 1 as libc::c_int as GLfloat);
    glVertex3f(
        0 as libc::c_int as GLfloat,
        480 as libc::c_int as GLfloat,
        0 as libc::c_int as GLfloat,
    );
    glEnd();
    glutSwapBuffers();
    frame += 1;
    if frame % 30 as libc::c_int == 0 as libc::c_int {
        tmp___0 = glutGet(700 as libc::c_int as GLenum);
        ms = tmp___0;
        fps = 30.0f64 / ((ms - my_ftime) as libc::c_double / 1000.0f64);
        my_ftime = ms;
    }
}
pub unsafe extern "C" fn keyPressed(
    mut key: libc::c_uchar,
    mut x: libc::c_int,
    mut y: libc::c_int,
) {
    's_46: {
        if !(key as libc::c_int == 27 as libc::c_int) {
            if !(key as libc::c_int == 113 as libc::c_int) {
                break 's_46;
            }
        }
        die = 1 as libc::c_int;
        pthread_join(freenect_thread, 0 as *mut libc::c_void as *mut *mut libc::c_void);
        pthread_cond_signal(&mut gl_frame_cond);
        glutDestroyWindow(window);
        free(depth_mid as *mut libc::c_void);
        free(depth_front as *mut libc::c_void);
        free(rgb_back as *mut libc::c_void);
        free(rgb_mid as *mut libc::c_void);
        free(rgb_front as *mut libc::c_void);
        exit(0 as libc::c_int);
    };
}
pub unsafe extern "C" fn ReSizeGLScene(mut Width: libc::c_int, mut Height: libc::c_int) {
    glViewport(0 as libc::c_int, 0 as libc::c_int, Width, Height);
    glMatrixMode(5889 as libc::c_int as GLenum);
    glLoadIdentity();
    glOrtho(
        0 as libc::c_int as GLdouble,
        640 as libc::c_int as GLdouble,
        480 as libc::c_int as GLdouble,
        0 as libc::c_int as GLdouble,
        -1.0f32 as GLdouble,
        1.0f32 as GLdouble,
    );
    glMatrixMode(5888 as libc::c_int as GLenum);
    glLoadIdentity();
}
pub unsafe extern "C" fn InitGL(mut Width: libc::c_int, mut Height: libc::c_int) {
    glClearColor(0.0f32, 0.0f32, 0.0f32, 0.0f32);
    glClearDepth(1.0f64);
    glDepthFunc(513 as libc::c_int as GLenum);
    glDepthMask(0 as libc::c_int as GLboolean);
    glDisable(2929 as libc::c_int as GLenum);
    glEnable(3042 as libc::c_int as GLenum);
    glDisable(3008 as libc::c_int as GLenum);
    glEnable(3553 as libc::c_int as GLenum);
    glBlendFunc(770 as libc::c_int as GLenum, 771 as libc::c_int as GLenum);
    glShadeModel(7424 as libc::c_int as GLenum);
    glGenTextures(1 as libc::c_int, &mut gl_depth_tex);
    glBindTexture(3553 as libc::c_int as GLenum, gl_depth_tex);
    glTexParameteri(
        3553 as libc::c_int as GLenum,
        10241 as libc::c_int as GLenum,
        9729 as libc::c_int,
    );
    glTexParameteri(
        3553 as libc::c_int as GLenum,
        10240 as libc::c_int as GLenum,
        9729 as libc::c_int,
    );
    glGenTextures(1 as libc::c_int, &mut gl_rgb_tex);
    glBindTexture(3553 as libc::c_int as GLenum, gl_rgb_tex);
    glTexParameteri(
        3553 as libc::c_int as GLenum,
        10241 as libc::c_int as GLenum,
        9729 as libc::c_int,
    );
    glTexParameteri(
        3553 as libc::c_int as GLenum,
        10240 as libc::c_int as GLenum,
        9729 as libc::c_int,
    );
    ReSizeGLScene(Width, Height);
}
pub unsafe extern "C" fn gl_threadfunc(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    printf(b"GL thread\n\0" as *const u8 as *const libc::c_char);
    glutInit(&mut g_argc, g_argv);
    glutInitDisplayMode(26 as libc::c_uint);
    glutInitWindowSize(640 as libc::c_int, 480 as libc::c_int);
    glutInitWindowPosition(0 as libc::c_int, 0 as libc::c_int);
    window = glutCreateWindow(
        b"libfreenect Registration viewer\0" as *const u8 as *const libc::c_char,
    );
    glutDisplayFunc(Some(DrawGLScene as unsafe extern "C" fn() -> ()));
    glutIdleFunc(Some(idle as unsafe extern "C" fn() -> ()));
    glutReshapeFunc(
        Some(ReSizeGLScene as unsafe extern "C" fn(libc::c_int, libc::c_int) -> ()),
    );
    glutKeyboardFunc(
        Some(
            keyPressed
                as unsafe extern "C" fn(libc::c_uchar, libc::c_int, libc::c_int) -> (),
        ),
    );
    InitGL(640 as libc::c_int, 480 as libc::c_int);
    glutMainLoop();
    return 0 as *mut libc::c_void;
}
pub static mut t_gamma: [uint16_t; 10000] = [0; 10000];
pub unsafe extern "C" fn depth_cb(
    mut dev: *mut freenect_device,
    mut v_depth: *mut libc::c_void,
    mut timestamp: uint32_t,
) {
    let mut i: libc::c_int = 0;
    let mut depth: *mut uint16_t = 0 as *mut uint16_t;
    let mut pval: libc::c_int = 0;
    let mut lb: libc::c_int = 0;
    depth = v_depth as *mut uint16_t;
    pthread_mutex_lock(&mut gl_backbuf_mutex);
    i = 0 as libc::c_int;
    while i < 307200 as libc::c_int {
        pval = t_gamma[*depth.offset(i as isize) as usize] as libc::c_int
            / 4 as libc::c_int;
        lb = pval & 255 as libc::c_int;
        *depth_mid
            .offset(
                (4 as libc::c_int * i + 3 as libc::c_int) as isize,
            ) = 128 as libc::c_int as uint8_t;
        if *depth.offset(i as isize) as libc::c_int == 0 as libc::c_int {
            *depth_mid
                .offset(
                    (4 as libc::c_int * i + 3 as libc::c_int) as isize,
                ) = 0 as libc::c_int as uint8_t;
        }
        match pval >> 8 as libc::c_int {
            0 => {
                *depth_mid
                    .offset(
                        (4 as libc::c_int * i) as isize,
                    ) = 255 as libc::c_int as uint8_t;
                *depth_mid
                    .offset(
                        (4 as libc::c_int * i + 1 as libc::c_int) as isize,
                    ) = (255 as libc::c_int - lb) as uint8_t;
                *depth_mid
                    .offset(
                        (4 as libc::c_int * i + 2 as libc::c_int) as isize,
                    ) = (255 as libc::c_int - lb) as uint8_t;
            }
            1 => {
                *depth_mid
                    .offset(
                        (4 as libc::c_int * i) as isize,
                    ) = 255 as libc::c_int as uint8_t;
                *depth_mid
                    .offset(
                        (4 as libc::c_int * i + 1 as libc::c_int) as isize,
                    ) = lb as uint8_t;
                *depth_mid
                    .offset(
                        (4 as libc::c_int * i + 2 as libc::c_int) as isize,
                    ) = 0 as libc::c_int as uint8_t;
            }
            2 => {
                *depth_mid
                    .offset(
                        (4 as libc::c_int * i) as isize,
                    ) = (255 as libc::c_int - lb) as uint8_t;
                *depth_mid
                    .offset(
                        (4 as libc::c_int * i + 1 as libc::c_int) as isize,
                    ) = 255 as libc::c_int as uint8_t;
                *depth_mid
                    .offset(
                        (4 as libc::c_int * i + 2 as libc::c_int) as isize,
                    ) = 0 as libc::c_int as uint8_t;
            }
            3 => {
                *depth_mid
                    .offset(
                        (4 as libc::c_int * i) as isize,
                    ) = 0 as libc::c_int as uint8_t;
                *depth_mid
                    .offset(
                        (4 as libc::c_int * i + 1 as libc::c_int) as isize,
                    ) = 255 as libc::c_int as uint8_t;
                *depth_mid
                    .offset(
                        (4 as libc::c_int * i + 2 as libc::c_int) as isize,
                    ) = lb as uint8_t;
            }
            4 => {
                *depth_mid
                    .offset(
                        (4 as libc::c_int * i) as isize,
                    ) = 0 as libc::c_int as uint8_t;
                *depth_mid
                    .offset(
                        (4 as libc::c_int * i + 1 as libc::c_int) as isize,
                    ) = (255 as libc::c_int - lb) as uint8_t;
                *depth_mid
                    .offset(
                        (4 as libc::c_int * i + 2 as libc::c_int) as isize,
                    ) = 255 as libc::c_int as uint8_t;
            }
            5 => {
                *depth_mid
                    .offset(
                        (4 as libc::c_int * i) as isize,
                    ) = 0 as libc::c_int as uint8_t;
                *depth_mid
                    .offset(
                        (4 as libc::c_int * i + 1 as libc::c_int) as isize,
                    ) = 0 as libc::c_int as uint8_t;
                *depth_mid
                    .offset(
                        (4 as libc::c_int * i + 2 as libc::c_int) as isize,
                    ) = (255 as libc::c_int - lb) as uint8_t;
            }
            _ => {
                *depth_mid
                    .offset(
                        (4 as libc::c_int * i) as isize,
                    ) = 0 as libc::c_int as uint8_t;
                *depth_mid
                    .offset(
                        (4 as libc::c_int * i + 1 as libc::c_int) as isize,
                    ) = 0 as libc::c_int as uint8_t;
                *depth_mid
                    .offset(
                        (4 as libc::c_int * i + 2 as libc::c_int) as isize,
                    ) = 0 as libc::c_int as uint8_t;
                *depth_mid
                    .offset(
                        (4 as libc::c_int * i + 3 as libc::c_int) as isize,
                    ) = 0 as libc::c_int as uint8_t;
            }
        }
        i += 1;
    }
    got_depth += 1;
    pthread_cond_signal(&mut gl_frame_cond);
    pthread_mutex_unlock(&mut gl_backbuf_mutex);
}
pub unsafe extern "C" fn rgb_cb(
    mut dev: *mut freenect_device,
    mut rgb: *mut libc::c_void,
    mut timestamp: uint32_t,
) {
    pthread_mutex_lock(&mut gl_backbuf_mutex);
    if !(rgb_back as libc::c_ulong == rgb as libc::c_ulong) {
        __assert_fail(
            b"rgb_back == rgb\0" as *const u8 as *const libc::c_char,
            b"regview.c\0" as *const u8 as *const libc::c_char,
            286 as libc::c_uint,
            b"rgb_cb\0" as *const u8 as *const libc::c_char,
        );
    }
    rgb_back = rgb_mid;
    freenect_set_video_buffer(dev, rgb_back as *mut libc::c_void);
    rgb_mid = rgb as *mut uint8_t;
    got_rgb += 1;
    pthread_cond_signal(&mut gl_frame_cond);
    pthread_mutex_unlock(&mut gl_backbuf_mutex);
}
pub unsafe extern "C" fn freenect_threadfunc(
    mut arg: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut tmp: freenect_frame_mode = freenect_frame_mode {
        reserved: 0,
        resolution: FREENECT_RESOLUTION_LOW,
        __annonCompField3: __anonunion____missing_field_name_633756896 {
            dummy: 0,
        },
        bytes: 0,
        width: 0,
        height: 0,
        data_bits_per_pixel: 0,
        padding_bits_per_pixel: 0,
        framerate: 0,
        is_valid: 0,
    };
    let mut tmp___0: freenect_frame_mode = freenect_frame_mode {
        reserved: 0,
        resolution: FREENECT_RESOLUTION_LOW,
        __annonCompField3: __anonunion____missing_field_name_633756896 {
            dummy: 0,
        },
        bytes: 0,
        width: 0,
        height: 0,
        data_bits_per_pixel: 0,
        padding_bits_per_pixel: 0,
        framerate: 0,
        is_valid: 0,
    };
    let mut res: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    freenect_set_depth_callback(
        f_dev,
        Some(
            depth_cb
                as unsafe extern "C" fn(
                    *mut freenect_device,
                    *mut libc::c_void,
                    uint32_t,
                ) -> (),
        ),
    );
    freenect_set_video_callback(
        f_dev,
        Some(
            rgb_cb
                as unsafe extern "C" fn(
                    *mut freenect_device,
                    *mut libc::c_void,
                    uint32_t,
                ) -> (),
        ),
    );
    tmp = freenect_find_video_mode(FREENECT_RESOLUTION_MEDIUM, FREENECT_VIDEO_RGB);
    freenect_set_video_mode(f_dev, tmp);
    tmp___0 = freenect_find_depth_mode(
        FREENECT_RESOLUTION_MEDIUM,
        FREENECT_DEPTH_REGISTERED,
    );
    freenect_set_depth_mode(f_dev, tmp___0);
    freenect_set_video_buffer(f_dev, rgb_back as *mut libc::c_void);
    freenect_start_depth(f_dev);
    freenect_start_video(f_dev);
    printf(
        b"'w'-tilt up, 's'-level, 'x'-tilt down, '0'-'6'-select LED mode, 'f'-video format\n\0"
            as *const u8 as *const libc::c_char,
    );
    while die == 0 {
        tmp___1 = freenect_process_events(f_ctx);
        res = tmp___1;
        if !(res < 0 as libc::c_int) {
            continue;
        }
        if !(res != -(10 as libc::c_int)) {
            continue;
        }
        printf(
            b"\nError %d received from libusb - aborting.\n\0" as *const u8
                as *const libc::c_char,
            res,
        );
        break;
    }
    printf(b"\nshutting down streams...\n\0" as *const u8 as *const libc::c_char);
    freenect_stop_depth(f_dev);
    freenect_stop_video(f_dev);
    freenect_close_device(f_dev);
    freenect_shutdown(f_ctx);
    printf(b"-- done!\n\0" as *const u8 as *const libc::c_char);
    return 0 as *mut libc::c_void;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: libc::c_int = 0;
    let mut v: libc::c_float = 0.;
    let mut tmp___4: libc::c_float = 0.;
    let mut tmp___5: libc::c_int = 0;
    let mut nr_devices: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut user_device_number: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    tmp = malloc(1228800 as libc::c_int as size_t);
    depth_mid = tmp as *mut uint8_t;
    tmp___0 = malloc(1228800 as libc::c_int as size_t);
    depth_front = tmp___0 as *mut uint8_t;
    tmp___1 = malloc(921600 as libc::c_int as size_t);
    rgb_back = tmp___1 as *mut uint8_t;
    tmp___2 = malloc(921600 as libc::c_int as size_t);
    rgb_mid = tmp___2 as *mut uint8_t;
    tmp___3 = malloc(921600 as libc::c_int as size_t);
    rgb_front = tmp___3 as *mut uint8_t;
    printf(b"Kinect camera test\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < 10000 as libc::c_int {
        v = (i as libc::c_double / 2048.0f64) as libc::c_float;
        tmp___4 = powf(v, 3 as libc::c_int as libc::c_float);
        v = tmp___4 * 6 as libc::c_int as libc::c_float;
        t_gamma[i
            as usize] = (v * 6 as libc::c_int as libc::c_float
            * 256 as libc::c_int as libc::c_float) as uint16_t;
        i += 1;
    }
    g_argc = argc;
    g_argv = argv;
    tmp___5 = freenect_init(&mut f_ctx, 0 as *mut libc::c_void);
    if tmp___5 < 0 as libc::c_int {
        printf(b"freenect_init() failed\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    freenect_set_log_level(f_ctx, FREENECT_LOG_DEBUG);
    freenect_select_subdevices(f_ctx, FREENECT_DEVICE_CAMERA);
    tmp___6 = freenect_num_devices(f_ctx);
    nr_devices = tmp___6;
    printf(
        b"Number of devices found: %d\n\0" as *const u8 as *const libc::c_char,
        nr_devices,
    );
    user_device_number = 0 as libc::c_int;
    if argc > 1 as libc::c_int {
        user_device_number = atoi(
            *argv.offset(1 as libc::c_int as isize) as *const libc::c_char,
        );
    }
    if nr_devices < 1 as libc::c_int {
        freenect_shutdown(f_ctx);
        return 1 as libc::c_int;
    }
    tmp___7 = freenect_open_device(f_ctx, &mut f_dev, user_device_number);
    if tmp___7 < 0 as libc::c_int {
        printf(b"Could not open device\n\0" as *const u8 as *const libc::c_char);
        freenect_shutdown(f_ctx);
        return 1 as libc::c_int;
    }
    res = pthread_create(
        &mut freenect_thread as *mut pthread_t,
        0 as *mut libc::c_void as *const pthread_attr_t,
        Some(
            freenect_threadfunc
                as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        0 as *mut libc::c_void,
    );
    if res != 0 {
        printf(b"pthread_create failed\n\0" as *const u8 as *const libc::c_char);
        freenect_shutdown(f_ctx);
        return 1 as libc::c_int;
    }
    gl_threadfunc(0 as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
