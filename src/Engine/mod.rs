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

use crate::units;
use ugli_webgl::WebGLUniformLocation;
mod sprite;

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

struct Engine {
    view_matrix: [f32; 16],
    canvas: CanvasElement,
    context: gl,
    p_matrix: WebGLUniformLocation,
    v_matrix: WebGLUniformLocation,
    test_one: f32,

    obj: sprite::Sprite,
    // obj2: sprite::Sprite,
}

trait BasicEngine<T> {
    fn init(&mut self, _: T);
    fn update(&mut self, _: T);
    fn draw();
}

impl BasicEngine<Rc<RefCell<Self>>> for Engine {
    fn init(&mut self, _rc: Rc<RefCell<Self>>) {
        self.context.enable(gl::BLEND);
        self.context.blend_func(gl::ONE, gl::ONE_MINUS_SRC_ALPHA);
        self.context.clear_color(0.5, 0.5, 0.5, 0.9);
    }

    fn update(&mut self, _rc: Rc<RefCell<Self>>) {
        let (w, h) = (self.canvas.width(), self.canvas.height());
        let proj_matrix = get_projection(10., (w as f32) / (h as f32), 1., 200.);

        self.test_one += 0.05;

        self.context
            .viewport(w as i32 * -1, h as i32 * -1, w as i32 * 2, h as i32 * 2);

        self.context
            .clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        self.context
            .uniform_matrix4fv(Some(&self.p_matrix), false, &proj_matrix[..], 0, 0);
        self.context
            .uniform_matrix4fv(Some(&self.v_matrix), false, &self.view_matrix[..], 0, 0);

        self.obj.update(&self.context);
        let vec = units::Vector2D {
            x: 1.,
            y: self.test_one,
        };
        self.obj.set_position_sprite(vec);
        // self.obj2.update(&self.context);

        window().request_animation_frame(move |_time| {
            _rc.borrow_mut().update(_rc.clone());
        });
    }

    fn draw() {}
}

pub fn init() {
    stdweb::initialize();

    let canvas: CanvasElement = document()
        .query_selector("#canvas")
        .unwrap()
        .unwrap()
        .try_into()
        .unwrap();

    let context: gl = canvas.get_context().unwrap();

    canvas.set_width(canvas.offset_width() as u32);
    canvas.set_height(canvas.offset_height() as u32);

    context.clear_color(1.0, 1.0, 0.0, 1.0);
    context.clear(gl::COLOR_BUFFER_BIT);

    window().add_event_listener(enclose!( (canvas) move |_: ResizeEvent| {
        canvas.set_width(canvas.offset_width() as u32);
        canvas.set_height(canvas.offset_height() as u32);
    }));

    let mov_matrix = [
        1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 1., 1., 0., 20.,
    ];

    let mut view_matrix = [
        1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 1., 1.,
    ];

    // translating z
    view_matrix[14] -= 20.; //zoom

    let url = "sprite.png";
    // let url2 = "sprite.png";

    let (mut obj, context, shader_program) = sprite::Sprite::new(context, url);
    //  let (obj2, context, shader_program) = sprite::Sprite::new(context, url2);
    let vec = units::Vector2D { x: 1., y: 1. };
    obj.set_position_sprite(vec);

    let p_matrix = context
        .get_uniform_location(&shader_program, "Pmatrix")
        .unwrap();
    let v_matrix = context
        .get_uniform_location(&shader_program, "Vmatrix")
        .unwrap();

    context.use_program(Some(&shader_program));

    let state = Rc::new(RefCell::new(Engine {
        view_matrix,
        canvas,
        context,
        p_matrix,
        v_matrix,
        test_one: 1.,
        // img,
        // url,
        obj,
        // obj2,
    }));

    state.borrow_mut().init(state.clone());
    state.borrow_mut().update(state.clone());
}

pub fn end() {
    stdweb::event_loop();
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
