// extern crate ugli_webgl;

// use stdweb::web::TypedArray;

// use stdweb::web::html_element::ImageElement;

// use ugli_webgl::WebGL2RenderingContext as gl;
// use ugli_webgl::WebGLBuffer as Buffer;

// use crate::units;

// #[derive(Debug)]
// pub struct Animation {
//     url: &'static str,
//     img: ImageElement,
//     vertex_buffer: Buffer,
//     color_buffer: Buffer,
//     index_buffer: Buffer,
//     m_matrix: ugli_webgl::WebGLUniformLocation,
//     uFrameLoc: ugli_webgl::WebGLUniformLocation,
//     mov_matrix: [f32; 16],
// }

// impl Animation {
//     pub fn new(
//         context: gl,
//         url: &'static str,
//         shader_program: ugli_webgl::WebGLProgram,
//     ) -> (Self, gl, ugli_webgl::WebGLProgram) {
//         let (context, vertex_buffer, color_buffer, index_buffer) =
//             bind_buffers(context, &shader_program);

//         let (m_matrix, uFrameLoc) = init_matrix(&context, &shader_program);

//         let mov_matrix = [
//             1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 1., 5., 0., 7.,
//         ];

//         let sprite_frame = units::Point { x: 0., y: 0. };

//         let ref tex = context.create_texture();
//         context.bind_texture(gl::TEXTURE_2D, tex.as_ref());

//         // let img = ImageElement::new();
//         // img.set_src(&url);

//         (
//             Animation {
//                 url,
//                 img,
//                 vertex_buffer,
//                 color_buffer,
//                 index_buffer,
//                 m_matrix,
//                 uFrameLoc,
//                 mov_matrix,
//             },
//             context,
//             shader_program,
//         )
//     }

//     pub fn update(&self, context: &gl) {
//         //todo ==================== naprawic po testach ==============================================================//

//         context.draw_elements(gl::TRIANGLES, 6, gl::UNSIGNED_SHORT, 0);
//         context.uniform_matrix4fv(Some(&self.m_matrix), false, &self.mov_matrix[..], 0, 0);
//         context.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, Some(&self.index_buffer));

//         context.tex_image2_d_1(
//             gl::TEXTURE_2D,
//             0,
//             gl::RGBA as i32,
//             gl::RGBA,
//             gl::UNSIGNED_BYTE,
//             &self.img,
//         );

//         context.generate_mipmap(gl::TEXTURE_2D);
//         context.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
//         context.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
//         context.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
//         context.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
//     }

//     pub fn set_position_sprite(&mut self, vec: units::Vector2D<f32>) {
//         let (x, y) = vec.get();
//         self.mov_matrix[12] = x;
//         self.mov_matrix[13] = y;
//     }

//     pub fn move_sprite(&mut self, vec: units::Vector2D<f32>) {
//         let (x, y) = vec.get();
//         self.mov_matrix[12] += x;
//         self.mov_matrix[13] += y;
//     }

//     pub fn set_scale_sprite(&mut self, scale: f32) {
//         self.mov_matrix[15] = scale;
//     }
// }

// fn bind_buffers(
//     context: gl,
//     shader_program: &ugli_webgl::WebGLProgram,
// ) -> (gl, Buffer, Buffer, Buffer) {
//     let vertices =
//         TypedArray::<f32>::from(&[1., 1., 0., 1., -1., 0., -1., -1., 0., -1., 1., 0.][..]).buffer();

//     let colors =
//         TypedArray::<f32>::from(&[0., 3., 0., 0., 3., 0., 0., 3., 0., 0., 3., 0.][..]).buffer();

//     let indices = TypedArray::<u16>::from(&[0, 1, 2, 0, 2, 3][..]).buffer();

//     let texture_coordinates =
//         TypedArray::<f32>::from(&[1., 0., 0., 1., 1., 0., 0., 1., 0., 0., 0., 0.][..]).buffer();

//     let texture_coord_buffer = context.create_buffer().unwrap();

//     context.bind_buffer(gl::ARRAY_BUFFER, Some(&texture_coord_buffer));
//     context.buffer_data_1(
//         gl::ARRAY_BUFFER,
//         Some(&texture_coordinates),
//         gl::STATIC_DRAW,
//     );

//     let vertex_buffer = context.create_buffer().unwrap();
//     context.bind_buffer(gl::ARRAY_BUFFER, Some(&vertex_buffer));
//     context.buffer_data_1(gl::ARRAY_BUFFER, Some(&vertices), gl::STATIC_DRAW);

//     let color_buffer = context.create_buffer().unwrap();
//     context.bind_buffer(gl::ARRAY_BUFFER, Some(&color_buffer));
//     context.buffer_data_1(gl::ARRAY_BUFFER, Some(&colors), gl::STATIC_DRAW);

//     let index_buffer = context.create_buffer().unwrap();
//     context.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
//     context.buffer_data_1(gl::ELEMENT_ARRAY_BUFFER, Some(&indices), gl::STATIC_DRAW);

//     context.bind_buffer(gl::ARRAY_BUFFER, Some(&vertex_buffer));
//     let position = context.get_attrib_location(&shader_program, "position") as u32;
//     context.vertex_attrib_pointer(position, 3, gl::FLOAT, false, 0, 0);
//     context.enable_vertex_attrib_array(position);

//     context.bind_buffer(gl::ARRAY_BUFFER, Some(&color_buffer));
//     let color = context.get_attrib_location(&shader_program, "color") as u32;
//     context.vertex_attrib_pointer(color, 3, gl::FLOAT, false, 0, 0);
//     context.enable_vertex_attrib_array(color);

//     context.bind_buffer(gl::ARRAY_BUFFER, Some(&texture_coord_buffer));
//     let uv = context.get_attrib_location(&shader_program, "a_uv") as u32;
//     context.vertex_attrib_pointer(uv, 3, gl::FLOAT, false, 0, 0);
//     context.enable_vertex_attrib_array(uv);

//     (context, vertex_buffer, color_buffer, index_buffer)
// }

// fn init_matrix(
//     context: &gl,
//     shader_program: &ugli_webgl::WebGLProgram,
// ) -> (
//     ugli_webgl::WebGLUniformLocation,
//     ugli_webgl::WebGLUniformLocation,
// ) {
//     let m_matrix = context
//         .get_uniform_location(&shader_program, "Mmatrix")
//         .unwrap();
//     let _textur = context
//         .get_uniform_location(&shader_program, "tex")
//         .unwrap();
//     let uFrameLoc = context
//         .get_uniform_location(&shader_program, "u_frame")
//         .unwrap();

//     (m_matrix, uFrameLoc)
// }

// pub fn createRectArray(x: i32, y: i32, w: i32, h: i32) -> stdweb::web::ArrayBuffer {
//     TypedArray::<i32>::from(&[x, y, x + w, y, x, y + h, x, y + h, x + w, y, x + w, y + h][..])
//         .buffer()
// }
