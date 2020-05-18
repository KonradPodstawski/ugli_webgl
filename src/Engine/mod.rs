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
    mov_matrix: [f32; 16],
    view_matrix: [f32; 16],
    canvas: CanvasElement,
    context: gl,
    p_matrix: WebGLUniformLocation,
    v_matrix: WebGLUniformLocation,
    m_matrix: WebGLUniformLocation,
    index_buffer: WebGLBuffer,
    test_one: i32,

    obj: sprite::Sprite,
    // TESTOWE ZMIENNE
    // img: ImageElement,
    // url: &'static str,
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
        //self.context.blend_func_separate(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA, gl::ONE, gl::ONE_MINUS_SRC_ALPHA);
        //self.context.depth_func(gl::LEQUAL);
        self.context.clear_color(0.5, 0.5, 0.5, 0.9);
        //self.context.clear_depth(1.0);

        // //==============================================TEST================================================//

        //==================================================================================================//
    }

    fn update(&mut self, _rc: Rc<RefCell<Self>>) {
        let (w, h) = (self.canvas.width(), self.canvas.height());
        let proj_matrix = get_projection(20., (w as f32) / (h as f32), 1., 100.);

        self.test_one -= 1;

        self.context
            .viewport(-self.test_one, -150, w as i32, h as i32);

        self.context
            .clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        self.context
            .uniform_matrix4fv(Some(&self.p_matrix), false, &proj_matrix[..], 0, 0);
        self.context
            .uniform_matrix4fv(Some(&self.v_matrix), false, &self.view_matrix[..], 0, 0);
        self.context
            .uniform_matrix4fv(Some(&self.m_matrix), false, &self.mov_matrix[..], 0, 0);
        self.context
            .bind_buffer(gl::ELEMENT_ARRAY_BUFFER, Some(&self.index_buffer));

        self.obj.update(&self.context);

        // self.context
        //     .draw_elements(gl::TRIANGLES, 6, gl::UNSIGNED_SHORT, 0);

        // //self.context.bind_texture(gl::TEXTURE_2D, tex.as_ref());
        // self.context.tex_image2_d_1(
        //     gl::TEXTURE_2D,
        //     0,
        //     gl::RGBA as i32,
        //     gl::RGBA,
        //     gl::UNSIGNED_BYTE,
        //     &self.img,
        // );

        // self.context.generate_mipmap(gl::TEXTURE_2D);

        // self.context
        //     .tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        // self.context
        //     .tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        // self.context
        //     .tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        // self.context
        //     .tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

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

    let vertices =
        TypedArray::<f32>::from(&[0.5, 0.5, 0., 0.5, -0.5, 0., -0.5, -0.5, 0., -0.5, 0.5, 0.][..])
            .buffer();

    let colors =
        TypedArray::<f32>::from(&[0., 3., 0., 0., 3., 0., 0., 3., 0., 0., 3., 0.][..]).buffer();

    let indices = TypedArray::<u16>::from(&[0, 1, 2, 0, 2, 3][..]).buffer();

    let texture_coordinates =
        TypedArray::<f32>::from(&[1., 0., 0., 1., 1., 0., 0., 1., 0., 0., 0., 0.][..]).buffer();

    let texture_coord_buffer = context.create_buffer().unwrap();
    context.bind_buffer(gl::ARRAY_BUFFER, Some(&texture_coord_buffer));
    context.buffer_data_1(
        gl::ARRAY_BUFFER,
        Some(&texture_coordinates),
        gl::STATIC_DRAW,
    );

    // Create and store data into vertex buffer
    let vertex_buffer = context.create_buffer().unwrap();
    context.bind_buffer(gl::ARRAY_BUFFER, Some(&vertex_buffer));
    context.buffer_data_1(gl::ARRAY_BUFFER, Some(&vertices), gl::STATIC_DRAW);

    // Create and store data into color buffer
    let color_buffer = context.create_buffer().unwrap();
    context.bind_buffer(gl::ARRAY_BUFFER, Some(&color_buffer));
    context.buffer_data_1(gl::ARRAY_BUFFER, Some(&colors), gl::STATIC_DRAW);

    // Create and store data into index buffer
    let index_buffer = context.create_buffer().unwrap();
    context.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
    context.buffer_data_1(gl::ELEMENT_ARRAY_BUFFER, Some(&indices), gl::STATIC_DRAW);

    /*=================== Shaders =========================*/
    let vert_code = r#"
        attribute vec3 position;
        uniform mat4 Pmatrix;
        uniform mat4 Vmatrix;
        uniform mat4 Mmatrix;
        attribute vec3 color;
        varying vec3 vColor;
        attribute vec2 a_uv;
        varying vec2 uv;

        void main() {
            gl_Position = Pmatrix*Vmatrix*Mmatrix*vec4(position, 1.);
            vColor = color;
            uv = a_uv;
        }
    "#;

    let frag_code = r#"
        precision mediump float;
        varying vec3 vColor;
        uniform sampler2D tex;
        varying vec2 uv;

        void main() {
            gl_FragColor = texture2D(tex,uv);
        }
    "#;

    let vert_shader = context.create_shader(gl::VERTEX_SHADER).unwrap();
    context.shader_source(&vert_shader, vert_code);
    context.compile_shader(&vert_shader);

    let frag_shader = context.create_shader(gl::FRAGMENT_SHADER).unwrap();
    context.shader_source(&frag_shader, frag_code);
    context.compile_shader(&frag_shader);

    let shader_program = context.create_program().unwrap();
    context.attach_shader(&shader_program, &vert_shader);
    context.attach_shader(&shader_program, &frag_shader);
    context.link_program(&shader_program);

    /* ====== Associating attributes to vertex shader =====*/
    let p_matrix = context
        .get_uniform_location(&shader_program, "Pmatrix")
        .unwrap();
    let v_matrix = context
        .get_uniform_location(&shader_program, "Vmatrix")
        .unwrap();
    let m_matrix = context
        .get_uniform_location(&shader_program, "Mmatrix")
        .unwrap();
    let _textur = context
        .get_uniform_location(&shader_program, "tex")
        .unwrap();

    context.bind_buffer(gl::ARRAY_BUFFER, Some(&vertex_buffer));
    let position = context.get_attrib_location(&shader_program, "position") as u32;
    context.vertex_attrib_pointer(position, 3, gl::FLOAT, false, 0, 0);

    // Position
    context.enable_vertex_attrib_array(position);
    context.bind_buffer(gl::ARRAY_BUFFER, Some(&color_buffer));
    let color = context.get_attrib_location(&shader_program, "color") as u32;
    context.vertex_attrib_pointer(color, 3, gl::FLOAT, false, 0, 0);

    // Color
    context.enable_vertex_attrib_array(color);

    context.bind_buffer(gl::ARRAY_BUFFER, Some(&texture_coord_buffer));
    let uv = context.get_attrib_location(&shader_program, "a_uv") as u32;
    context.vertex_attrib_pointer(uv, 3, gl::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(uv);

    context.use_program(Some(&shader_program));

    let mov_matrix = [
        1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 1., 1., 0., 5.,
    ];
    let mut view_matrix = [
        1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
    ];

    // translating z
    view_matrix[14] -= 8.; //zoom

    let url = "sprite.png";

    // let ref tex = context.create_texture();
    // context.bind_texture(gl::TEXTURE_2D, tex.as_ref());

    // let img = ImageElement::new();
    // img.set_src(&url);

    let (obj, context) = sprite::Sprite::new(context, url);

    let state = Rc::new(RefCell::new(Engine {
        mov_matrix,
        view_matrix,
        canvas,
        context,
        p_matrix,
        v_matrix,
        m_matrix,
        index_buffer,
        test_one: 500,
        // img,
        // url,
        obj,
    }));

    state.borrow_mut().init(state.clone());
    state.borrow_mut().update(state.clone());

    // context
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
