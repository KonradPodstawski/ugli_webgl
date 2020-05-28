use ugli_webgl::WebGL2RenderingContext as gl;

#[derive(Debug)]
pub struct Camera {
    view_matrix: [f32; 16],
    p_matrix: ugli_webgl::WebGLUniformLocation,
    v_matrix: ugli_webgl::WebGLUniformLocation,
}

impl Camera {
    pub fn init(context: &gl, shader_program: &ugli_webgl::WebGLProgram) -> Self {
        let view_matrix = [
            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 1., 1.,
        ];

        let (p_matrix, v_matrix) = matrix(&context, &shader_program);

        Camera {
            view_matrix,
            p_matrix,
            v_matrix,
        }
    }

    pub fn zoom(&mut self, zoom_size: f32) {
        self.view_matrix[14] -= zoom_size;
    }

    pub fn update(&mut self, context: &gl) {
        context.uniform_matrix4fv(Some(&self.v_matrix), false, &self.view_matrix[..], 0, 0);
    }

    pub fn config_projectcion(&self, context: &gl, angle: f32, a: f32, z_min: f32, z_max: f32) {
        let proj_matrix = get_projection(angle, a, z_min, z_max);
        context.uniform_matrix4fv(Some(&self.p_matrix), false, &proj_matrix[..], 0, 0);
    }
}

pub fn matrix(
    context: &gl,
    shader_program: &ugli_webgl::WebGLProgram,
) -> (
    ugli_webgl::WebGLUniformLocation,
    ugli_webgl::WebGLUniformLocation,
) {
    let p_matrix = context
        .get_uniform_location(&shader_program, "Pmatrix")
        .unwrap();
    let v_matrix = context
        .get_uniform_location(&shader_program, "Vmatrix")
        .unwrap();
    (p_matrix, v_matrix)
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
