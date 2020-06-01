extern crate ugli_webgl;

use std::cell::RefCell;
use std::rc::Rc;

use stdweb::console;
use stdweb::web::{window, IEventTarget};

use stdweb::web::event::IKeyboardEvent;
use stdweb::web::event::KeyDownEvent;
use stdweb::web::event::KeyUpEvent;

use stdweb::web::html_element::CanvasElement;

use ugli_webgl::WebGL2RenderingContext as gl;

use crate::engine::camera;
use crate::engine::shaders;
use crate::engine::sprite;
use crate::units;

use crate::engine;

#[derive(Debug)]
struct Applicatiom {
    canvas: CanvasElement,
    context: gl,

    state_camera: camera::Camera,

    left_score: i32,
    right_score: i32,

    player_1: sprite::Sprite,
    player_2: sprite::Sprite,
    ball: sprite::Sprite,

    velocity: units::Vector2D<f32>,

    down_pressed: Rc<RefCell<bool>>,
    top_pressed: Rc<RefCell<bool>>,

    ball_x: f32,
    ball_y: f32,

    axis_y_one: Rc<RefCell<f32>>,
    axis_y_two: Rc<RefCell<f32>>,
}

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

impl Applicatiom {
    fn player_one_update(&mut self, _rc: Rc<RefCell<Self>>) {
        let top = Rc::clone(&self.top_pressed);
        let down = Rc::clone(&self.down_pressed);

        window().add_event_listener(enclose!((top,down) move |_event: KeyDownEvent| {

            if _event.code() == "KeyS" {
                *top.borrow_mut() = true ;
            }
            if _event.code() == "KeyW" {
                *down.borrow_mut() = true ;
            }

        }));

        window().add_event_listener(enclose!((top,down) move |_event: KeyUpEvent| {

            if _event.code() == "KeyS" {
                *top.borrow_mut() = false;
            }
            if _event.code() == "KeyW" {
                *down.borrow_mut() = false;
            }

        }));
    }

    fn player_two_update(&mut self, _rc: Rc<RefCell<Self>>) {
        let player2 = Rc::clone(&self.axis_y_two);

        if *player2.borrow_mut() > self.ball_y {
            *player2.borrow_mut() -= 0.5;
        } else if *player2.borrow_mut() < self.ball_y {
            *player2.borrow_mut() += 0.5;
        }
    }

    fn update(&mut self, _rc: Rc<RefCell<Self>>) {
        //====================================== Update Window  ======================================//
        let (w, h) = (self.canvas.width(), self.canvas.height());
        self.context
            .clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

        self.state_camera
            .config_projectcion(&self.context, 10., (w as f32) / (h as f32), 1., 200.);

        self.context
            .viewport(w as i32 * -1, h as i32 * -1, w as i32 * 2, h as i32 * 2);

        self.state_camera.update(&self.context);

        //========================================== Enemy ===========================================//

        if *self.axis_y_two.borrow_mut() > self.ball_y {
            *self.axis_y_two.borrow_mut() -= 0.12;
        } else if *self.axis_y_two.borrow_mut() < self.ball_y {
            *self.axis_y_two.borrow_mut() += 0.12;
        }

        self.player_1.update(&self.context);
        self.player_2.update(&self.context);

        //==================================== Player update position =================================//

        if *self.top_pressed.borrow_mut() {
            *self.axis_y_one.borrow_mut() -= 0.25;
        } else if *self.down_pressed.borrow_mut() {
            *self.axis_y_one.borrow_mut() += 0.25;
        }

        //======================================== Add range area =====================================//

        engine::range(&mut (*self.axis_y_one.borrow_mut()));
        engine::range(&mut (*self.axis_y_two.borrow_mut()));

        //====================================== Player set position ==================================//

        let vec: units::Vector2D<f32> = units::Vector2D {
            x: 1.,
            y: *self.axis_y_one.borrow_mut(),
        };
        self.player_1.set_position_sprite(vec);

        let vec2: units::Vector2D<f32> = units::Vector2D {
            x: 32.,
            y: *self.axis_y_two.borrow_mut(),
        };
        self.player_2.set_position_sprite(vec2);

        //======================================== Ball update position ===================================//

        self.ball.update(&self.context);

        self.ball_x += self.velocity.x;
        if self.ball_x > 32. - self.ball.get_width() {
            self.velocity.x = -self.velocity.x;
            if self.ball_y - 1.4 > self.player_2.get_y()
                || self.ball_y - 0.1 < self.player_2.get_y() - 1.6
            {
                self.right_score += 1;
                self.ball_y = 9.;
                self.ball_x = 16.;

                self.velocity.x = 0.1;
                self.velocity.y = 0.1;

                console!(
                    log,
                    "LEFT SCORE: ",
                    self.left_score,
                    "RIGHT SCORE: ",
                    self.right_score
                );
            } else {
                self.velocity.x -= 0.025;
                self.velocity.y -= 0.025;
            }
        }
        if self.ball_x < 1. + self.ball.get_width() {
            self.velocity.x = -self.velocity.x;
            if self.ball_y - 1.4 > self.player_1.get_y()
                || self.ball_y - 0.1 < self.player_1.get_y() - 1.6
            {
                self.left_score += 1;
                self.ball_y = 9.;
                self.ball_x = 16.;

                self.velocity.x = 0.1;
                self.velocity.y = 0.1;

                console!(
                    log,
                    "LEFT SCORE: ",
                    self.left_score,
                    "RIGHT SCORE: ",
                    self.right_score
                );
            } else {
                self.velocity.x += 0.025;
                self.velocity.y += 0.025;
            }
        }

        self.ball_y += self.velocity.y;
        if self.ball_y < 1. || self.ball_y > 16. {
            self.velocity.y = -self.velocity.y;
        }

        //====================================== Player set position ==================================//

        let vec3: units::Vector2D<f32> = units::Vector2D {
            x: self.ball_x,
            y: self.ball_y,
        };
        self.ball.set_position_sprite(vec3);

        //====================================== Animation callback ===================================//

        window().request_animation_frame(move |_time| {
            _rc.borrow_mut().update(_rc.clone());
        });

        //=============================================================================================//
    }
}

pub fn init() {
    engine::init();

    //========================================== Set clear color ======================================//

    let window_color = units::Color {
        red: 1.,
        green: 1.,
        blue: 0.,
        alfa: 1.,
    };

    //================================== Create obligatory parameters ==================================//

    let (canvas, context) = engine::create_ugli_window(window_color);
    let shader_program = shaders::create_texture_shaders(&context);

    let mut state_camera = camera::Camera::init(&context, &shader_program);

    state_camera.zoom(20.);

    let url_1 = "paletka.png";
    let url_2 = "paletka.png";
    let url_3 = "ball.png";

    let (mut player_1, context, shader_program) =
        sprite::Sprite::new(context, url_1, shader_program, 0.5, 2.);

    let (mut player_2, context, shader_program) =
        sprite::Sprite::new(context, url_2, shader_program, 0.5, 2.);

    let (mut ball, context, shader_program) =
        sprite::Sprite::new(context, url_3, shader_program, 1., 1.);

    let init_position_1 = units::Vector2D {
        x: 1.,
        y: (9) as f32,
    };
    let init_position_2 = units::Vector2D {
        x: (32) as f32,
        y: (9) as f32,
    };

    let init_position_3 = units::Vector2D {
        x: 16.,
        y: (9) as f32,
    };

    let velocity = units::Vector2D { x: 0.15, y: 0.15 };

    player_1.set_position_sprite(init_position_1);
    player_1.set_scale_sprite(10.);

    player_2.set_position_sprite(init_position_2);
    player_2.set_scale_sprite(10.);

    ball.set_position_sprite(init_position_3);
    ball.set_scale_sprite(10.);

    context.use_program(Some(&shader_program));

    let color = units::Color {
        red: 0.5,
        green: 0.5,
        blue: 0.5,
        alfa: 0.9,
    };
    engine::clear_color(&context, color);

    let down_pressed = Rc::new(RefCell::new(false));
    let top_pressed = Rc::new(RefCell::new(false));

    let axis_y_one = Rc::new(RefCell::new(9.));
    let axis_y_two = Rc::new(RefCell::new(9.));

    let state = Rc::new(RefCell::new(Applicatiom {
        canvas,
        context,

        state_camera,

        left_score: 0,
        right_score: 0,

        player_1,
        player_2,
        ball,

        velocity,

        down_pressed,
        top_pressed,

        ball_x: 18.,
        ball_y: 15.,

        axis_y_one,
        axis_y_two,
    }));

    state.borrow_mut().update(state.clone());
    state.borrow_mut().player_two_update(state.clone());
    state.borrow_mut().player_one_update(state.clone());

    engine::end();
}
