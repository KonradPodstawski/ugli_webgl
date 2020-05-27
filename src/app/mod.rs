extern crate ugli_webgl;

use std::cell::RefCell;
use std::rc::Rc;

use stdweb::web::window;

use stdweb::web::html_element::CanvasElement;

use ugli_webgl::WebGL2RenderingContext as gl;

use crate::engine::camera;
use crate::engine::sprite;
use crate::shaders;
use crate::units;

use crate::engine;

#[derive(Debug)]
struct Applicatiom {
    canvas: CanvasElement,
    context: gl,
    state_camera: camera::Camera,
    test_one: f32,
    obj: sprite::Sprite,
}

impl Applicatiom {
    fn update(&mut self, _rc: Rc<RefCell<Self>>) {
        let (w, h) = (self.canvas.width(), self.canvas.height());
        self.context
            .clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        self.test_one += 0.05;

        let a = (w as f32) / (h as f32);

        self.state_camera
            .config_projectcion(&self.context, 10., a, 1., 200.);

        self.context
            .viewport(w as i32 * -1, h as i32 * -1, w as i32 * 2, h as i32 * 2);

        self.state_camera.update(&self.context);

        self.obj.update(&self.context);

        let vec = units::Vector2D {
            x: 1.,
            y: self.test_one,
        };

        self.obj.set_position_sprite(vec);

        window().request_animation_frame(move |_time| {
            _rc.borrow_mut().update(_rc.clone());
        });
    }
}

pub fn init() {
    engine::init();

    let window_color = units::Color {
        red: 1.,
        green: 1.,
        blue: 0.,
        alfa: 1.,
    };

    let (canvas, context) = engine::create_ugli_window(window_color);

    let shader_program = shaders::create_texture_shaders(&context);

    let mut state_camera = camera::Camera::init(&context, &shader_program);

    state_camera.zoom(20.);

    let url = "sprite.png";
    let _url2 = "sprite.png";

    let (mut obj, context, shader_program) = sprite::Sprite::new(context, url, shader_program);

    let vec = units::Vector2D { x: 1., y: 1. };

    obj.set_position_sprite(vec);
    obj.set_scale_sprite(10.);

    camera::matrix(&context, &shader_program);

    context.use_program(Some(&shader_program));

    let color = units::Color {
        red: 0.5,
        green: 0.5,
        blue: 0.5,
        alfa: 0.9,
    };
    engine::clear_color(&context, color);

    let state = Rc::new(RefCell::new(Applicatiom {
        canvas,
        context,
        state_camera,
        test_one: 1.,
        obj,
    }));

    state.borrow_mut().update(state.clone());

    engine::end();
}
