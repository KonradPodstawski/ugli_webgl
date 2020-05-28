extern crate ugli_webgl;
use std::cell::RefCell;
use std::rc::Rc;

use stdweb::console;
use stdweb::js;
use stdweb::unstable::TryInto;
use stdweb::web::{document, window, IEventTarget, IHtmlElement, IParentNode, TypedArray};

use stdweb::web::event::KeyDownEvent;
use stdweb::web::event::{IEvent, IKeyboardEvent, KeyUpEvent, KeyboardLocation, ResizeEvent};
use stdweb::web::html_element::ImageElement;

use stdweb::web::html_element::CanvasElement;

use futures::executor::block_on;
use futures::executor::*;
use futures::*;
use std::thread::JoinHandle;
use ugli_webgl::WebGL2RenderingContext as gl;
use ugli_webgl::WebGLBuffer;

use crate::shaders;
use crate::units;
use ugli_webgl::WebGLUniformLocation;
pub mod camera;
pub mod sprite;

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

struct Engine {
    canvas: CanvasElement,
    context: gl,
    state_camera: camera::Camera,

    obj: sprite::Sprite,
    obj2: sprite::Sprite,

    test: Rc<RefCell<i32>>,
}

trait BasicEngine<T> {
    fn init(&mut self, _: T);
    fn update(&mut self, _: T);
    fn draw();
    fn test();
}

static mut STATIC: i32 = 1;

impl BasicEngine<Rc<RefCell<Self>>> for Engine {
    fn test() {}

    fn init(&mut self, _rc: Rc<RefCell<Self>>) {
        let h = Rc::clone(&self.test);
        let w = window().add_event_listener(|_event: KeyDownEvent| {
            let z = _event.code();

            // let nowy = Rc::clone(h);

            // *nowy.borrow_mut() += 1;
            console!(log, z);
            if _event.code() == "KeyS" {
                // STATIC += 1;

                console!(log, "sterowanie:  S");
            }
            if _event.code() == "KeyW" {
                console!(log, "sterowanie:  W");
            }
            if _event.code() == "KeyA" {
                console!(log, "sterowanie: A");
            }
            if _event.code() == "KeyD" {
                console!(log, "sterowanie: D");
            }
        });
    }

    fn update(&mut self, _rc: Rc<RefCell<Self>>) {
        let (w, h) = (self.canvas.width(), self.canvas.height());
        self.context
            .clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

        let a = (w as f32) / (h as f32);

        self.state_camera
            .config_projectcion(&self.context, 10., a, 1., 200.);

        self.context
            .viewport(w as i32 * -1, h as i32 * -1, w as i32 * 2, h as i32 * 2);

        self.state_camera.update(&self.context);

        self.obj.update(&self.context);
        let vec: units::Vector2D<f32> = units::Vector2D { x: 0., y: 0.05 };
        self.obj.move_sprite(vec);
        self.obj2.update(&self.context);

        window().request_animation_frame(move |_time| {
            _rc.borrow_mut().update(_rc.clone());
        });
    }

    fn draw() {}
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

pub fn test() {
    init();

    let window_color = units::Color {
        red: 1.,
        green: 1.,
        blue: 0.,
        alfa: 1.,
    };

    let (canvas, context) = create_ugli_window(window_color);
    let shader_program = shaders::create_texture_shaders(&context);
    let mut state_camera = camera::Camera::init(&context, &shader_program);

    state_camera.zoom(20.);

    let url = "paletka.png";
    let _url2 = "paletka.png";

    let (mut obj, context, shader_program) = sprite::Sprite::new(context, url, shader_program);
    let (mut obj2, context, shader_program) = sprite::Sprite::new(context, _url2, shader_program);

    let vec = units::Vector2D {
        x: 1.,
        y: (9) as f32,
    };
    let vec2 = units::Vector2D {
        x: (32) as f32,
        y: (9) as f32,
    };

    obj.set_position_sprite(vec);
    obj.set_scale_sprite(10.);
    obj2.set_position_sprite(vec2);
    obj2.set_scale_sprite(10.);

    camera::matrix(&context, &shader_program);

    context.use_program(Some(&shader_program));

    let color = units::Color {
        red: 0.5,
        green: 0.5,
        blue: 0.5,
        alfa: 0.9,
    };
    clear_color(&context, color);

    let test = Rc::new(RefCell::new(1));

    let state = Rc::new(RefCell::new(Engine {
        canvas,
        context,
        state_camera,
        obj,
        obj2,
        test,
    }));

    let w = state.borrow_mut().init(state.clone());
    let w2 = state.borrow_mut().update(state.clone());

    stdweb::event_loop();
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

pub fn on_key(key: &str, location: KeyboardLocation, is_pressed: bool) -> bool {
    let location = format!("{:?}", location);
    console!(
        log,
        "Key: ",
        key,
        ", location: ",
        location,
        ", pressed: ",
        is_pressed
    );
    true
}

fn actual_key_debug() {
    document().add_event_listener(move |event: KeyDownEvent| {
        if on_key(&event.key(), event.location(), true) {
            event.prevent_default();
        };
    });
}
