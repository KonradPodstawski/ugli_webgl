// use crate::shaders;

// use ugli_webgl::WebGL2RenderingContext as gl;

// pub fn shader_program(
//     vertex_shader: &ugli_webgl::WebGLShader,
//     fragment_shader: &ugli_webgl::WebGLShader,
//     context: &gl,
// ) -> ugli_webgl::WebGLProgram {
//     let shader_program = context.create_program().unwrap();
//     context.attach_shader(&shader_program, &vertex_shader);
//     context.attach_shader(&shader_program, &fragment_shader);
//     context.link_program(&shader_program);

//     shader_program
// }

// pub fn create_texture_shaders(context: &gl) -> ugli_webgl::WebGLProgram {
//     let vert_id = shaders::texture::Create_Texture_Vertex_Shader(&context);
//     let frag_id = shaders::texture::Create_Texture_Fragment_Shader(&context);
//     let shader_id = shader_program(&vert_id, &frag_id, &context);

//     shader_id
// }
