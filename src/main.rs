use glfw;
use glfw::{Action, Context, Key};

use gl;
use gl::types::*;

mod shader;
use shader::Shader;

use std::os::raw::c_void;
use std::sync::mpsc::Receiver;
use std::{mem, ptr};

mod common;
use common::load_texture;

#[macro_use]
mod macros;

const SCR_WIDTH: u32 = 600;
const SCR_HEIGHT: u32 = 600;

pub fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) = glfw
        .create_window(
            SCR_WIDTH,
            SCR_HEIGHT,
            "LearnOpenGL",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window");
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    let (our_shader, vbo, vao, ebo, texture) = unsafe {
        let our_shader = Shader::new(
            "src/shaders/texture.vert",
            "src/shaders/texture.frag",
        );
        #[cfg_attr(rustfmt, rustfmt_skip)]
            let vertices: [f32; 32] = [
            0.5, 0.5, 0., 1., 0., 0., 1., 1.,
            0.5, -0.5, 0., 0., 1., 0., 1., 0.,
            -0.5, -0.5, 0., 0., 0., 1., 0., 0.,
            -0.5, 0.5, 0., 1., 1., 0., 0., 1.,
        ];

        let indices = [0, 1, 3, 1, 2, 3];
        let (mut vbo, mut vao, mut ebo) = (0, 0, 0);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &vertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &indices[0] as *const i32 as *const c_void,
            gl::STATIC_DRAW,
        );
        let stride = 8 * mem::size_of::<GLfloat>() as GLsizei;

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (3 * mem::size_of::<GLfloat>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);

        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (6 * mem::size_of::<GLfloat>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(2);

        let texture = load_texture("resources/textures/dog-face.png");

        (our_shader, vbo, vao, ebo, texture)
    };

    while !window.should_close() {
        process_events(&mut window, &events);
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            our_shader.use_program();
            let time = glfw.get_time() as f32;
            let time_loc = gl::GetUniformLocation(our_shader.id, c_str!("time").as_ptr());
            gl::Uniform1f(time_loc, time);

            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
        window.swap_buffers();
        glfw.poll_events();
    }

    unsafe {
        gl::DeleteVertexArrays(1, &vao);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteBuffers(1, &ebo);
    }
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                gl::Viewport(0, 0, width, height)
            },
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }
}
