extern crate custom_egl as egl;
extern crate custom_gl as gl;

use std::mem;
use std::ffi::{CStr, CString};
use egl::types::*;

fn main()
{
    let disp = unsafe { egl::GetDisplay(egl::DEFAULT_DISPLAY) };
    if disp == egl::NO_DISPLAY
    {
        panic!("error opening EGL display");
    }

    let mut egl_major = 0;
    let mut egl_minor = 0;
    if unsafe { egl::Initialize(disp, &mut egl_major, &mut egl_minor) } == 0
    {
        panic!("error initializing EGL");
    }
    println!("loaded EGL {}.{}", egl_major, egl_minor);

    unsafe
    {
        let vendor = CStr::from_ptr(egl::QueryString(disp, egl::VENDOR as EGLint));
        let version = CStr::from_ptr(egl::QueryString(disp, egl::VERSION as EGLint));
        let apis = CStr::from_ptr(egl::QueryString(disp, egl::CLIENT_APIS as EGLint));
        let exts = CStr::from_ptr(egl::QueryString(disp, egl::EXTENSIONS as EGLint));
        println!("EGL vendor: {:?}\nEGL version: {:?}\nEGL apis: {:?}\nEGL extensions: {:?}",
            vendor, version, apis, exts);
    }

    let cfg_attribs = [
        egl::RED_SIZE, 8,
        egl::GREEN_SIZE, 8,
        egl::BLUE_SIZE, 8,
        egl::CONFORMANT, egl::OPENGL_ES2_BIT,
        egl::RENDERABLE_TYPE, egl::OPENGL_ES2_BIT,
        egl::NONE
    ];
    let configs: [EGLConfig; 1] = unsafe{ mem::zeroed() };
    let mut num_cfg = 0;
    if unsafe { egl::ChooseConfig(disp, cfg_attribs.as_ptr() as _, configs.as_ptr() as *mut _, configs.len() as EGLint, &mut num_cfg) } == 0
    {
        panic!("error choosing EGL config");
    }
    if num_cfg == 0
    {
        panic!("no compatible EGL configs found");
    }

    let ctx_attribs = [
        egl::CONTEXT_CLIENT_VERSION, 2,
        egl::NONE
    ];
    let context = unsafe { egl::CreateContext(disp, configs[0], egl::NO_CONTEXT, ctx_attribs.as_ptr() as _) };
    if context == egl::NO_CONTEXT
    {
        panic!("error creating GL context");
    }

    if unsafe { egl::MakeCurrent(disp, egl::NO_SURFACE, egl::NO_SURFACE, context) } == 0
    {
        panic!("error binding GL context");
    };

    gl::load_with(|name| unsafe { egl::GetProcAddress(CString::new(name).unwrap().as_ptr()) } as *const _);

    unsafe
    {
        let vendor = CStr::from_ptr(gl::GetString(gl::VENDOR) as *const _);
        let renderer = CStr::from_ptr(gl::GetString(gl::RENDERER) as *const _);
        let version = CStr::from_ptr(gl::GetString(gl::VERSION) as *const _);
        let exts = CStr::from_ptr(gl::GetString(gl::EXTENSIONS) as *const _);
        println!("GL vendor: {:?}\nGL renderer: {:?}\nGL version: {:?}\nGL extensions: {:?}",
            vendor, renderer, version, exts);
    }
}
