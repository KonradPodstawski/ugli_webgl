extern crate ugli_webgl;
// use std::cell::RefCell;
// use std::rc::Rc;

// use stdweb::console;
// use stdweb::js;
use stdweb::unstable::TryInto;
// use stdweb::web::TypedArray;
use stdweb::web::{document, window, IEventTarget, IHtmlElement, IParentNode};
// use stdweb::web::event::KeyDownEvent;
// use stdweb::web::event::{IEvent, IKeyboardEvent, KeyUpEvent, KeyboardLocation};
// use stdweb::web::html_element::ImageElement;
use stdweb::web::event::ResizeEvent;
use stdweb::web::html_element::CanvasElement;

// use futures::executor::block_on;
// use futures::executor::*;
// use futures::*;
// use std::thread::JoinHandle;
// use stdweb::web::Date;
use ugli_webgl::WebGL2RenderingContext as gl;
// use ugli_webgl::WebGLBuffer;

// use crate::shaders;
use crate::units;
// use ugli_webgl::WebGLUniformLocation;
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

// struct Engine {
//     canvas: CanvasElement,
//     context: gl,
//     state_camera: camera::Camera,

//     left_score: i32,
//     right_score: i32,

//     obj: sprite::Sprite,
//     obj2: sprite::Sprite,
//     obj3: sprite::Sprite,

//     velocity: units::Vector2D<f32>,
//     ball_x: f32,
//     ball_y: f32,

//     gracz1: Rc<RefCell<f32>>,
//     gracz2: Rc<RefCell<f32>>,
// }

// trait BasicEngine<T> {
//     fn init(&mut self, _: T);
//     fn update(&mut self, _: T);
//     fn draw();
//     fn test();
// }

// impl BasicEngine<Rc<RefCell<Self>>> for Engine {
//     fn test() {}

//     fn init(&mut self, _rc: Rc<RefCell<Self>>) {
//         let player1 = Rc::clone(&self.gracz1);
//         let player2 = Rc::clone(&self.gracz2);

//         window().add_event_listener(enclose!((player1, player2) move |_event: KeyDownEvent| {
//             let z = _event.code();

//             //console!(log, z);
//             if _event.code() == "KeyS" {
//                 *player1.borrow_mut() -= 0.25 ;
//                 // console!(log, "sterowanie:  S");
//             }
//             if _event.code() == "KeyW" {
//                 *player1.borrow_mut() += 0.25 ;
//                 // console!(log, "sterowanie:  W");
//             }

//             if _event.code() == "ArrowUp" {
//                 *player2.borrow_mut() += 0.25 ;
//                 // console!(log, "sterowanie:  Up");
//             }
//             if _event.code() == "ArrowDown" {
//                 *player2.borrow_mut() -= 0.25 ;
//                 // console!(log, "sterowanie:  Down");
//             }
//         }));
//     }

//     fn update(&mut self, _rc: Rc<RefCell<Self>>) {
//         let (w, h) = (self.canvas.width(), self.canvas.height());
//         self.context
//             .clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

//         self.state_camera
//             .config_projectcion(&self.context, 10., (w as f32) / (h as f32), 1., 200.);

//         self.context
//             .viewport(w as i32 * -1, h as i32 * -1, w as i32 * 2, h as i32 * 2);

//         self.state_camera.update(&self.context);

//         range(&mut (*self.gracz1.borrow_mut()));
//         range(&mut (*self.gracz2.borrow_mut()));

//         //Lewa paletka
//         self.obj.update(&self.context);
//         let vec: units::Vector2D<f32> = units::Vector2D {
//             x: 1.,
//             y: *self.gracz1.borrow_mut(),
//         };
//         self.obj.set_position_sprite(vec);

//         // Prawa Paletka
//         self.obj2.update(&self.context);
//         let vec2: units::Vector2D<f32> = units::Vector2D {
//             x: 32.,
//             y: *self.gracz2.borrow_mut(),
//         };
//         self.obj2.set_position_sprite(vec2);

//         self.obj3.update(&self.context);

//         self.ball_x += self.velocity.x;
//         if self.ball_x > 32. - self.obj3.get_width() {
//             self.velocity.x = -self.velocity.x;
//             if self.ball_y < self.obj.get_y() || self.ball_y > self.obj.get_y() + 7. {
//                 self.right_score += 1;

//                 console!(
//                     log,
//                     "LEFT SCORE: ",
//                     self.left_score,
//                     "RIGHT SCORE: ",
//                     self.right_score
//                 );
//             }
//         }
//         if self.ball_x < 1. + self.obj3.get_width() {
//             self.velocity.x = -self.velocity.x;
//             if self.ball_y < self.obj2.get_y() || self.ball_y > self.obj2.get_y() + 7. {
//                 self.left_score += 1;

//                 console!(
//                     log,
//                     "LEFT SCORE: ",
//                     self.left_score,
//                     "RIGHT SCORE: ",
//                     self.right_score
//                 );
//             }
//         }

//         self.ball_y += self.velocity.y;
//         if self.ball_y < 1. || self.ball_y > 16. {
//             self.velocity.y = -self.velocity.y;
//         }

//         let vec3: units::Vector2D<f32> = units::Vector2D {
//             x: self.ball_x,
//             y: self.ball_y,
//         };
//         self.obj3.set_position_sprite(vec3);

//         window().request_animation_frame(move |_time| {
//             _rc.borrow_mut().update(_rc.clone());
//         });
//     }

//     fn draw() {}
// }

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

// fn test() {
//     init();

//     let window_color = units::Color {
//         red: 1.,
//         green: 1.,
//         blue: 0.,
//         alfa: 1.,
//     };

//     let (canvas, context) = create_ugli_window(window_color);
//     let shader_program = shaders::create_texture_shaders(&context);
//     let mut state_camera = camera::Camera::init(&context, &shader_program);

//     state_camera.zoom(20.);

//     let url = "paletka.png";
//     let _url2 = "paletka.png";
//     let url3 = "ball.png";

//     let (mut obj, context, shader_program) =
//         sprite::Sprite::new(context, url, shader_program, 0.5, 2.);

//     let (mut obj2, context, shader_program) =
//         sprite::Sprite::new(context, _url2, shader_program, 0.5, 2.);

//     let (mut obj3, context, shader_program) =
//         sprite::Sprite::new(context, url3, shader_program, 1., 1.);

//     let vec = units::Vector2D {
//         x: 1.,
//         y: (9) as f32,
//     };
//     let vec2 = units::Vector2D {
//         x: (32) as f32,
//         y: (9) as f32,
//     };

//     let vec3 = units::Vector2D {
//         x: 16.,
//         y: (9) as f32,
//     };

//     let velocity = units::Vector2D { x: 0.1, y: 0.1 };

//     obj.set_position_sprite(vec);
//     obj.set_scale_sprite(10.);

//     obj2.set_position_sprite(vec2);
//     obj2.set_scale_sprite(10.);

//     obj3.set_position_sprite(vec3);
//     obj3.set_scale_sprite(10.);

//     camera::matrix(&context, &shader_program);

//     context.use_program(Some(&shader_program));

//     let color = units::Color {
//         red: 0.5,
//         green: 0.5,
//         blue: 0.5,
//         alfa: 0.9,
//     };
//     clear_color(&context, color);

//     let gracz1 = Rc::new(RefCell::new(9.));
//     let gracz2 = Rc::new(RefCell::new(9.));

//     //graniczne wartocis wyskosc : 21
//     //graniczne wartocis wyskosc : 1.5

//     let state = Rc::new(RefCell::new(Engine {
//         canvas,
//         context,
//         state_camera,

//         left_score: 0,
//         right_score: 0,

//         obj,
//         obj2,
//         obj3,

//         velocity,

//         ball_x: 18.,
//         ball_y: 15.,

//         gracz1,
//         gracz2,
//     }));

//     let _time = Date::new();
//     let testwooew = _time.get_time();
//     console!(log, testwooew);

//     state.borrow_mut().init(state.clone());
//     state.borrow_mut().update(state.clone());

//     stdweb::event_loop();
// }

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
    if *y >= 21. {
        *y -= 0.5;
    }
    if *y <= 1. {
        *y += 0.5;
    }
}
