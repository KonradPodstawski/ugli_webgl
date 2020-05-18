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
use ugli_webgl::WebGLBuffer;

use ugli_webgl::WebGLUniformLocation;

#[derive(Debug)]
pub struct Sprite {
    url: &'static str,
    img: ImageElement,
}

impl Sprite {
    pub fn new(context: gl, url: &'static str) -> (Self, gl) {
        let ref tex = context.create_texture();
        context.bind_texture(gl::TEXTURE_2D, tex.as_ref());

        let img = ImageElement::new();
        img.set_src(&url);

        (Sprite { url, img }, context)
    }

    pub fn update(&self, context: &gl) {
        context.draw_elements(gl::TRIANGLES, 6, gl::UNSIGNED_SHORT, 0);

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
}
