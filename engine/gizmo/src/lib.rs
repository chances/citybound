extern crate compact;
#[macro_use]
extern crate compact_macros;
extern crate kay;
extern crate monet;
extern crate descartes;

extern crate webrender;
extern crate webrender_api;
extern crate winit;
extern crate euclid;

mod notifier;

// user_interface/mod.rs parked_frame has the GL context

// Using webrender without servo
// https://github.com/servo/webrender/issues/205

// https://github.com/servo/webrender/blob/master/examples/common/boilerplate.rs
// https://github.com/glennw/wr-sample/blob/master/src/main.rs

pub mod webrender_gizmo;
