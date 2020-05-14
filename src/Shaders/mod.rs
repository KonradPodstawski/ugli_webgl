extern crate ugli_webgl;

#[macro_use]
use std::cell::RefCell;
use std::rc::Rc;

use stdweb::unstable::TryInto;
use stdweb::web::{document, window, IEventTarget, IHtmlElement, IParentNode, TypedArray};

use stdweb::web::html_element::ImageElement;

use stdweb::web::event::{IEvent, IKeyboardEvent, KeyDownEvent, KeyboardLocation, ResizeEvent};

use stdweb::web::html_element::CanvasElement;

use ugli_webgl::WebGL2RenderingContext as gl;
use ugli_webgl::WebGLBuffer;
use ugli_webgl::WebGLProgram;

use ugli_webgl::WebGLUniformLocation;

pub mod buffers;
pub mod fragment;
pub mod vertex;
use crate::matrix;

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

fn init_program(context: &ugli_webgl::WebGL2RenderingContext) -> ugli_webgl::WebGLProgram {
    let vert_shader = context.create_shader(gl::VERTEX_SHADER).unwrap();
    context.shader_source(&vert_shader, vertex::get_vertex());
    context.compile_shader(&vert_shader);

    let frag_shader = context.create_shader(gl::FRAGMENT_SHADER).unwrap();
    context.shader_source(&frag_shader, fragment::get_fragment());
    context.compile_shader(&frag_shader);

    let shader_program = context.create_program().unwrap();
    context.attach_shader(&shader_program, &vert_shader);
    context.attach_shader(&shader_program, &frag_shader);
    context.link_program(&shader_program);

    shader_program
}

fn bind_buffers(
    context: &ugli_webgl::WebGL2RenderingContext,
    shader_program: &ugli_webgl::WebGLProgram,
) {
    context.bind_buffer(
        gl::ARRAY_BUFFER,
        Some(&buffers::get_vertex_buffer(&context)),
    );
    let position = context.get_attrib_location(&shader_program, "position") as u32;
    context.vertex_attrib_pointer(position, 3, gl::FLOAT, false, 0, 0);

    context.enable_vertex_attrib_array(position);
    context.bind_buffer(gl::ARRAY_BUFFER, Some(&buffers::get_color_buffer(&context)));
    let color = context.get_attrib_location(&shader_program, "color") as u32;
    context.vertex_attrib_pointer(color, 3, gl::FLOAT, false, 0, 0);

    context.enable_vertex_attrib_array(color);

    context.bind_buffer(
        gl::ARRAY_BUFFER,
        Some(&buffers::get_texture_coord_buffer(&context)),
    );

    let uv = context.get_attrib_location(&shader_program, "a_uv") as u32;
    context.vertex_attrib_pointer(uv, 3, gl::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(uv);

    context.use_program(Some(&shader_program));
}

pub fn init_shader(
    context: ugli_webgl::WebGL2RenderingContext,
    canvas: CanvasElement,
) -> ugli_webgl::WebGL2RenderingContext {
    let _program = init_program(&context);
    bind_buffers(&context, &_program);
    create(&context, &canvas, &_program);
    create(&context, &canvas, &_program);
    create(&context, &canvas, &_program);
    create(&context, &canvas, &_program);
    create(&context, &canvas, &_program);

    context
}

fn get_projection(angle: f32, a: f32, z_min: f32, z_max: f32) -> [f32; 16] {
    let ang = (angle * 0.5).to_radians().tan();
    return [
        0.5 / ang,
        0.,
        0.,
        0.,
        0.,
        0.5 * a / ang,
        0.,
        0.,
        0.,
        0.,
        -(z_max + z_min) / (z_max - z_min),
        -1.,
        0.,
        0.,
        (-2. * z_max * z_min) / (z_max - z_min),
        0.,
    ];
}

fn create(
    context: &ugli_webgl::WebGL2RenderingContext,
    canvas: &CanvasElement,
    shader_program: &ugli_webgl::WebGLProgram,
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
    let textures = context
        .get_uniform_location(&shader_program, "tex")
        .unwrap();

    context.enable(gl::DEPTH_TEST);
    context.depth_func(gl::LEQUAL);
    context.clear_color(0.5, 0.5, 0.5, 0.9);
    context.clear_depth(1.0);

    let (w, h) = (canvas.width(), canvas.height());
    let proj_matrix = get_projection(20., (w as f32) / (h as f32), 1., 100.);

    context.viewport(-500, -150, w as i32, h as i32);
    context.clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    context.uniform_matrix4fv(Some(&p_matrix), false, &proj_matrix[..], 0, 0);

    unsafe {
        matrix::VIEW_MATRIX[14] -= 8.;
        context.uniform_matrix4fv(Some(&v_matrix), false, &matrix::VIEW_MATRIX[..], 0, 0);
        context.uniform_matrix4fv(Some(&m_matrix), false, &matrix::MOV_MATRIX[..], 0, 0);
    }
    context.bind_buffer(
        gl::ELEMENT_ARRAY_BUFFER,
        Some(&buffers::get_index_buffer(&context)),
    );
    context.draw_elements(gl::TRIANGLES, 6, gl::UNSIGNED_SHORT, 0);

    let ref tex = context.create_texture();
    // context.bind_texture(gl::TEXTURE_2D, tex.as_ref());

    let url = "sprite.bmp";

    let img = ImageElement::new();
    img.set_src(&url);

    context.bind_texture(gl::TEXTURE_2D, tex.as_ref());
    context.tex_image2_d_1(
        gl::TEXTURE_2D,
        0,
        gl::RGBA as i32,
        gl::RGBA,
        gl::UNSIGNED_BYTE,
        img,
    );

    context.generate_mipmap(gl::TEXTURE_2D);

    context.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    context.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    context.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
    context.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
    // window().request_animation_frame(move |time| {
    //     rc.borrow_mut().create(rc.clone());
    // });
    // TODO auto odtwoarzanie funkcji;
}
