pub fn get_vertex() -> &'static str {
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

    vert_code
}
