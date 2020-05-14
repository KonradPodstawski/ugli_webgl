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

pub fn get_vertices() -> stdweb::web::ArrayBuffer {
    TypedArray::<f32>::from(&[0.5, 0.5, 0., 0.5, -0.5, 0., -0.5, -0.5, 0., -0.5, 0.5, 0.][..])
        .buffer()
}

pub fn get_indices() -> stdweb::web::ArrayBuffer {
    TypedArray::<u16>::from(&[0, 1, 2, 0, 2, 3][..]).buffer()
}

pub fn get_color_array() -> stdweb::web::ArrayBuffer {
    TypedArray::<f32>::from(&[0., 3., 0., 0., 3., 0., 0., 3., 0., 0., 3., 0.][..]).buffer()
}

pub fn get_texture_coords() -> stdweb::web::ArrayBuffer {
    TypedArray::<f32>::from(&[1., 0., 0., 1., 1., 0., 0., 1., 0., 0., 0., 0.][..]).buffer()
}

pub fn get_texture_coord_buffer(
    context: &ugli_webgl::WebGL2RenderingContext,
) -> ugli_webgl::WebGLBuffer {
    let textureCoordBuffer = context.create_buffer().unwrap();
    context.bind_buffer(gl::ARRAY_BUFFER, Some(&textureCoordBuffer));
    context.buffer_data_1(
        gl::ARRAY_BUFFER,
        Some(&get_texture_coords()),
        gl::STATIC_DRAW,
    );

    textureCoordBuffer
}

pub fn get_vertex_buffer(context: &ugli_webgl::WebGL2RenderingContext) -> ugli_webgl::WebGLBuffer {
    let vertex_buffer = context.create_buffer().unwrap();
    context.bind_buffer(gl::ARRAY_BUFFER, Some(&vertex_buffer));
    context.buffer_data_1(gl::ARRAY_BUFFER, Some(&get_vertices()), gl::STATIC_DRAW);

    vertex_buffer
}

pub fn get_color_buffer(context: &ugli_webgl::WebGL2RenderingContext) -> ugli_webgl::WebGLBuffer {
    let color_buffer = context.create_buffer().unwrap();
    context.bind_buffer(gl::ARRAY_BUFFER, Some(&color_buffer));
    context.buffer_data_1(gl::ARRAY_BUFFER, Some(&get_color_array()), gl::STATIC_DRAW);

    color_buffer
}

pub fn get_index_buffer(context: &ugli_webgl::WebGL2RenderingContext) -> ugli_webgl::WebGLBuffer {
    let index_buffer = context.create_buffer().unwrap();
    context.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
    context.buffer_data_1(
        gl::ELEMENT_ARRAY_BUFFER,
        Some(&get_indices()),
        gl::STATIC_DRAW,
    );

    index_buffer
}
