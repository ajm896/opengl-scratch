extern crate gl;
extern crate sdl2;
use std::ffi::{CStr, CString};

const WIDTH: i32 = 900;
const HEIGHT: i32 = 700;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    gl_init(WIDTH, HEIGHT);

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
            clear();
            window.gl_swap_window();
        }
    }
}

fn gl_init(width: i32, height: i32) {
    unsafe {
        gl::ClearColor(0.3, 0.3, 0.3, 1.0);
        gl::Viewport(0, 0, width, height);
    }
}

fn clear() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

fn shader_from_source(
    source: &CString,
    kind: gl::types::GLenum,
) -> Result<gl::types::GLuint, String> {
    let id = unsafe { gl::CreateShader(kind) };

    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;

    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        // allocate buffer of correct size
        let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
        // fill it with len spaces
        buffer.extend([b' '].iter().cycle().take(len as usize));
        // convert buffer to CString
        let error: CString = unsafe { CString::from_vec_unchecked(buffer) };

        
    }

    return Ok(id);
}
