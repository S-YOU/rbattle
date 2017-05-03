#![allow(dead_code)]

// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate glium;

mod drawer;
mod errors;
mod graph;
mod map;
mod math;
mod square;
mod state;
mod visible_graph;

use drawer::Drawer;
use map::Map;
use square::SquareGrid;
use state::State;

use glium::glutin::Event;
use glium::Surface;

use std::rc::Rc;

// This only gives access within this module. Make this `pub use errors::*;`
// instead if the types must be accessible from other modules (e.g., within
// a `links` section).
use errors::*;

#[cfg(test)]
#[macro_use]
mod test_utils;

fn main() {
    if let Err(ref e) = run() {
        use ::std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    use glium::DisplayBuild;

    let display = glium::glutin::WindowBuilder::new()
        .with_title("rbattle".to_string())
        .build_glium()
        .chain_err(|| "unable to open window")?;

    let map = Rc::new(Map {
        graph: SquareGrid::new(30, 40),
        sources: vec![]
    });

    let drawer = Drawer::new(&display, &map)
        .chain_err(|| "failed to construct Drawer for map")?;

    loop {
        let state = State {
            map: map.clone(),
            nodes: vec![]
        };

        let mut frame = display.draw();
        frame.clear_color(1.0, 0.43, 0.0, 1.0);
        let status = drawer.draw(&mut frame, &state);
        frame.finish()
            .chain_err(|| "drawing finish failed")?;

        status?;

        for event in display.poll_events() {
            match event {
                Event::Closed => return Ok(()),
                _ => ()
            }
        }
    }
}
