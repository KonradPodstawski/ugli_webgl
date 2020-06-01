extern crate ugli_webgl;

extern crate serde_derive;
extern crate stdweb;
extern crate stdweb_derive;

mod engine;
pub mod units;

pub mod app;

fn main() {
    app::init();
}
