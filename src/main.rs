extern crate ugli_webgl;

extern crate serde_derive;
extern crate stdweb;
extern crate stdweb_derive;

pub mod app;
mod engine;

fn main() {
    app::init();
}
