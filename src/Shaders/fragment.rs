pub fn get_fragment() -> &'static str {
    let frag_code = r#"
        precision mediump float;
        varying vec3 vColor;
        uniform sampler2D tex;
        varying vec2 uv;

        void main() {
            gl_FragColor = texture2D(tex,uv);
        }
    "#;

    frag_code
}
