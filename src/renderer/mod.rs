mod camera;
use camera::Camera;
use nalgebra::{Point3, Matrix4};

use gl::types::{GLfloat, GLuint};
use std::mem;
use std::ptr;
use std::os::raw::c_void;

// Struct to manage the renderer
pub struct Renderer {
    vao: GLuint,
    vbo: GLuint,
    shader_program: GLuint,
    pub camera: Camera,
}

impl Renderer {
    pub fn new(aspect_ratio: f32) -> Self {
        // Triangle vertices (x, y, z)
        let vertices: [f32; 9] = [
            0.0,  0.5, 0.0,  // Top vertex
            -0.5, -0.5, 0.0,  // Bottom-left vertex
            0.5, -0.5, 0.0,  // Bottom-right vertex
        ];

        // Create and compile shaders
        let vertex_shader_src = std::fs::read_to_string("assets/shaders/vertex.glsl")
            .expect("Vertex shader file missing");
        let fragment_shader_src = std::fs::read_to_string("assets/shaders/fragment.glsl")
            .expect("Fragment shader file missing");

        let shader_program = create_shader_program(&vertex_shader_src, &fragment_shader_src);

        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 0;

        // Set up VAO and VBO
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            // Fill VBO with vertex data
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as isize,
                vertices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            // Define vertex attribute layout in the shader
            gl::VertexAttribPointer(
                0,  // location in the shader
                3,  // number of components per vertex (x, y, z)
                gl::FLOAT,
                gl::FALSE,
                3 * mem::size_of::<GLfloat>() as i32,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            // Unbind VAO
            gl::BindVertexArray(0);
        }

        let camera = Camera::new(
            Point3::new(0.0, 0.0, 3.0), // Position the camera slightly back
            Point3::origin(),           // Look towards the origin
            aspect_ratio,
        );

        Renderer {
            vao,
            vbo,
            shader_program,
            camera,
        }
    }

    // Clears the screen
    pub fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    // Renders the triangle
    pub fn render(&self) {
        unsafe {
            gl::UseProgram(self.shader_program);

            // Set the view and projection matrices in the shader
            let view = self.camera.view_matrix();
            let projection = self.camera.projection_matrix();

            let view_loc = gl::GetUniformLocation(self.shader_program, "view\0".as_ptr() as *const i8);
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, view.as_ptr());

            let projection_loc = gl::GetUniformLocation(self.shader_program, "projection\0".as_ptr() as *const i8);
            gl::UniformMatrix4fv(projection_loc, 1, gl::FALSE, projection.as_ptr());

            // Draw the triangle
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::BindVertexArray(0);
        }
    }
}

// Helper function to create and link a shader program
fn create_shader_program(vertex_src: &str, fragment_src: &str) -> GLuint {
    unsafe {
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(vertex_shader, 1, [vertex_src.as_ptr() as *const i8].as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);

        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(fragment_shader, 1, [fragment_src.as_ptr() as *const i8].as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);

        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        shader_program
    }
}
