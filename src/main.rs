extern crate ugli_webgl;

extern crate serde_derive;
extern crate stdweb;
extern crate stdweb_derive;

mod engine;
// mod Shaders;
pub mod matrix;

fn main() {
    engine::init();

    //Shaders::init_shader(_ctx, _canvas);

    engine::end();
}
