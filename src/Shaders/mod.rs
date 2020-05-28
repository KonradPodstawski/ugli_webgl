extern crate ugli_webgl;
use crate::shaders;
use ugli_webgl::WebGL2RenderingContext as gl;

pub fn get_fragment() -> &'static str {
    let frag_code = r#"
        precision mediump float;
        varying vec3 vColor;
        uniform sampler2D tex;
        varying vec2 uv;

        void main() {
            gl_FragColor = texture2D(tex,uv);
        }
    "#;

    frag_code
}

pub fn get_vertex() -> &'static str {
    let vert_code = r#"
        attribute vec3 position;
        uniform mat4 Pmatrix;
        uniform mat4 Vmatrix;
        uniform mat4 Mmatrix;
        attribute vec3 color;
        varying vec3 vColor;
        attribute vec2 a_uv;
        varying vec2 uv;


        void main() {
            gl_Position = Pmatrix*Vmatrix*Mmatrix*vec4(position, 1.);
            vColor = color;
            uv = a_uv;
        }
    "#;

    vert_code
}

pub fn create_texture_vertex_shader(context: &gl) -> ugli_webgl::WebGLShader {
    let shader_id = context.create_shader(gl::VERTEX_SHADER).unwrap();
    context.shader_source(&shader_id, shaders::get_vertex());
    context.compile_shader(&shader_id);

    shader_id
}

pub fn create_texture_fragment_shader(context: &gl) -> ugli_webgl::WebGLShader {
    let shader_id = context.create_shader(gl::FRAGMENT_SHADER).unwrap();
    context.shader_source(&shader_id, shaders::get_fragment());
    context.compile_shader(&shader_id);

    shader_id
}

pub fn shader_program(
    vertex_shader: &ugli_webgl::WebGLShader,
    fragment_shader: &ugli_webgl::WebGLShader,
    context: &gl,
) -> ugli_webgl::WebGLProgram {
    let shader_program = context.create_program().unwrap();
    context.attach_shader(&shader_program, &vertex_shader);
    context.attach_shader(&shader_program, &fragment_shader);
    context.link_program(&shader_program);

    shader_program
}

pub fn create_texture_shaders(context: &gl) -> ugli_webgl::WebGLProgram {
    let vert_id = shaders::create_texture_vertex_shader(&context);
    let frag_id = shaders::create_texture_fragment_shader(&context);
    let shader_id = shader_program(&vert_id, &frag_id, &context);

    shader_id
}
