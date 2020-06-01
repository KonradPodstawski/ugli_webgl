extern crate ugli_webgl;

use stdweb::unstable::TryInto;
use stdweb::web::event::ResizeEvent;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, window, IEventTarget, IHtmlElement, IParentNode};

use crate::units;
use ugli_webgl::WebGL2RenderingContext as gl;
pub mod camera;
pub mod shaders;
pub mod sprite;

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

pub fn create_ugli_window(color: units::Color) -> (CanvasElement, gl) {
    let canvas: CanvasElement = document()
        .query_selector("#canvas")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();

    let context: gl = canvas.get_context().unwrap();

    canvas.set_width(canvas.offset_width() as u32);
    canvas.set_height(canvas.offset_height() as u32);

    let (red, green, blue, alfa) = color.get();
    context.clear_color(red, green, blue, alfa);
    context.clear(gl::COLOR_BUFFER_BIT);

    window().add_event_listener(enclose!( (canvas) move |_: ResizeEvent| {
        canvas.set_width(canvas.offset_width() as u32);
        canvas.set_height(canvas.offset_height() as u32);
    }));

    (canvas, context)
}

pub fn init() {
    stdweb::initialize();
}

pub fn end() {
    stdweb::event_loop();
}

pub fn clear_color(context: &gl, color: units::Color) {
    let (red, green, blue, alfa) = color.get();

    context.enable(gl::BLEND);
    context.blend_func(gl::ONE, gl::ONE_MINUS_SRC_ALPHA);
    context.clear_color(red, green, blue, alfa);
}

pub fn range(y: &mut f32) {
    if *y >= 16. {
        *y -= 0.5;
    }
    if *y <= 1. {
        *y += 0.5;
    }
}
