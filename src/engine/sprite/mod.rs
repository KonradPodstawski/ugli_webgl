extern crate ugli_webgl;

use std::cell::RefCell;
use std::rc::Rc;

use stdweb::console;
use stdweb::unstable::TryInto;
use stdweb::web::{document, window, IEventTarget, IHtmlElement, IParentNode, TypedArray};

use stdweb::web::html_element::ImageElement;

use stdweb::web::event::ResizeEvent;

use stdweb::web::html_element::CanvasElement;

use ugli_webgl::WebGL2RenderingContext as gl;
use ugli_webgl::WebGLBuffer as Buffer;

use crate::shaders;

use ugli_webgl::WebGLUniformLocation;

use crate::units;

#[derive(Debug)]
pub struct Sprite {
    url: &'static str,
    img: ImageElement,
    vertex_buffer: Buffer,
    color_buffer: Buffer,
    index_buffer: Buffer,
    p_matrix: ugli_webgl::WebGLUniformLocation,
    v_matrix: ugli_webgl::WebGLUniformLocation,
    m_matrix: ugli_webgl::WebGLUniformLocation,
    mov_matrix: [f32; 16],
}

impl Sprite {
    pub fn new(context: gl, url: &'static str) -> (Self, gl, ugli_webgl::WebGLProgram) {
        let (context, vertex_buffer, color_buffer, index_buffer, texture_coord_buffer) =
            bind_buffers(context);
        let shader_program = init_shaders(&context);
        let (p_matrix, v_matrix, m_matrix) = init_matrix(&context, &shader_program);
        shader_buffers(
            &context,
            &shader_program,
            &vertex_buffer,
            &color_buffer,
            &texture_coord_buffer,
        );

        let mov_matrix = [
            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 1., 5., 0., 7.,
        ];
        // let w = units::Vector2D { x: 4, y: 4 };

        let ref tex = context.create_texture();
        context.bind_texture(gl::TEXTURE_2D, tex.as_ref());

        let img = ImageElement::new();
        img.set_src(&url);

        (
            Sprite {
                url,
                img,
                vertex_buffer,
                color_buffer,
                index_buffer,
                p_matrix,
                v_matrix,
                m_matrix,
                mov_matrix,
            },
            context,
            shader_program,
        )
    }

    pub fn update(&self, context: &gl) {
        // let mut mov_matrix = [
        //     1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 1., TEST, 0., 5.,
        // ];

        context.draw_elements(gl::TRIANGLES, 6, gl::UNSIGNED_SHORT, 0);
        context.uniform_matrix4fv(Some(&self.m_matrix), false, &self.mov_matrix[..], 0, 0);
        context.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, Some(&self.index_buffer));

        context.tex_image2_d_1(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            &self.img,
        );

        context.generate_mipmap(gl::TEXTURE_2D);

        context.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        context.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        context.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        context.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
    }

    pub fn set_position_sprite(&mut self, vec: units::Vector2D<f32>) {
        let (x, y) = vec.get();
        self.mov_matrix = [
            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., x, y, 0., 20.,
        ];
    }
}

fn bind_buffers(context: gl) -> (gl, Buffer, Buffer, Buffer, Buffer) {
    let vertices =
        TypedArray::<f32>::from(&[1., 1., 0., 1., -1., 0., -1., -1., 0., -1., 1., 0.][..]).buffer();

    let colors =
        TypedArray::<f32>::from(&[0., 3., 0., 0., 3., 0., 0., 3., 0., 0., 3., 0.][..]).buffer();

    let indices = TypedArray::<u16>::from(&[0, 1, 2, 0, 2, 3][..]).buffer();

    let texture_coordinates =
        TypedArray::<f32>::from(&[1., 0., 0., 1., 1., 0., 0., 1., 0., 0., 0., 0.][..]).buffer();

    let texture_coord_buffer = context.create_buffer().unwrap();
    context.bind_buffer(gl::ARRAY_BUFFER, Some(&texture_coord_buffer));
    context.buffer_data_1(
        gl::ARRAY_BUFFER,
        Some(&texture_coordinates),
        gl::STATIC_DRAW,
    );

    let vertex_buffer = context.create_buffer().unwrap();
    context.bind_buffer(gl::ARRAY_BUFFER, Some(&vertex_buffer));
    context.buffer_data_1(gl::ARRAY_BUFFER, Some(&vertices), gl::STATIC_DRAW);

    let color_buffer = context.create_buffer().unwrap();
    context.bind_buffer(gl::ARRAY_BUFFER, Some(&color_buffer));
    context.buffer_data_1(gl::ARRAY_BUFFER, Some(&colors), gl::STATIC_DRAW);

    let index_buffer = context.create_buffer().unwrap();
    context.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
    context.buffer_data_1(gl::ELEMENT_ARRAY_BUFFER, Some(&indices), gl::STATIC_DRAW);

    (
        context,
        vertex_buffer,
        color_buffer,
        index_buffer,
        texture_coord_buffer,
    )
}

fn init_matrix(
    context: &gl,
    shader_program: &ugli_webgl::WebGLProgram,
) -> (
    ugli_webgl::WebGLUniformLocation,
    ugli_webgl::WebGLUniformLocation,
    ugli_webgl::WebGLUniformLocation,
) {
    let p_matrix = context
        .get_uniform_location(&shader_program, "Pmatrix")
        .unwrap();
    let v_matrix = context
        .get_uniform_location(&shader_program, "Vmatrix")
        .unwrap();
    let m_matrix = context
        .get_uniform_location(&shader_program, "Mmatrix")
        .unwrap();
    let _textur = context
        .get_uniform_location(&shader_program, "tex")
        .unwrap();

    (p_matrix, v_matrix, m_matrix)
}

fn init_shaders(context: &gl) -> ugli_webgl::WebGLProgram {
    let vert_shader = context.create_shader(gl::VERTEX_SHADER).unwrap();
    context.shader_source(&vert_shader, shaders::texture::get_vertex());
    context.compile_shader(&vert_shader);

    let frag_shader = context.create_shader(gl::FRAGMENT_SHADER).unwrap();
    context.shader_source(&frag_shader, shaders::texture::get_fragment());
    context.compile_shader(&frag_shader);

    let shader_program = context.create_program().unwrap();
    context.attach_shader(&shader_program, &vert_shader);
    context.attach_shader(&shader_program, &frag_shader);
    context.link_program(&shader_program);

    shader_program
}

fn shader_buffers(
    context: &gl,
    shader_program: &ugli_webgl::WebGLProgram,
    vertex_buffer: &Buffer,
    color_buffer: &Buffer,
    texture_coord_buffer: &Buffer,
) {
    context.bind_buffer(gl::ARRAY_BUFFER, Some(&vertex_buffer));
    let position = context.get_attrib_location(&shader_program, "position") as u32;
    context.vertex_attrib_pointer(position, 3, gl::FLOAT, false, 0, 0);

    // Position
    context.enable_vertex_attrib_array(position);
    context.bind_buffer(gl::ARRAY_BUFFER, Some(&color_buffer));
    let color = context.get_attrib_location(&shader_program, "color") as u32;
    context.vertex_attrib_pointer(color, 3, gl::FLOAT, false, 0, 0);

    // Color
    context.enable_vertex_attrib_array(color);

    context.bind_buffer(gl::ARRAY_BUFFER, Some(&texture_coord_buffer));
    let uv = context.get_attrib_location(&shader_program, "a_uv") as u32;
    context.vertex_attrib_pointer(uv, 3, gl::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(uv);
}
