extern crate ugli_webgl;

#[macro_use]
extern crate stdweb;
extern crate serde_derive;
extern crate stdweb_derive;

use std::cell::RefCell;
use std::rc::Rc;

use stdweb::unstable::TryInto;
use stdweb::web::{document, window, IEventTarget, IHtmlElement, IParentNode, TypedArray};

use stdweb::web::html_element::ImageElement;

use stdweb::web::event::{IEvent, IKeyboardEvent, KeyDownEvent, KeyboardLocation, ResizeEvent};

use stdweb::web::html_element::CanvasElement;

use ugli_webgl::WebGL2RenderingContext as gl;
use ugli_webgl::WebGLBuffer;

use ugli_webgl::WebGLUniformLocation;

mod Engine;
mod Shaders;
pub mod matrix;

fn main() {
    Engine::init();

    //Shaders::init_shader(_ctx, _canvas);

    Engine::end();
}
