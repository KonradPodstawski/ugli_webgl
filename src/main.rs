extern crate ugli_webgl;

extern crate serde_derive;
extern crate stdweb;
extern crate stdweb_derive;

mod engine;
pub mod matrix;
pub mod shaders;
pub mod units;

fn main() {
    engine::init();

    //Shaders::init_shader(_ctx, _canvas);

    engine::end();
}
