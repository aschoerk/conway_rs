#[macro_use]
extern crate glium;
extern crate glutin;
extern crate rand;

use glium::DisplayBuild;
use glium::Surface;
use glium::Program;
use std::time::Duration;
use std::env;
use std::thread;
use std::sync::{Arc, Mutex};
use grid::Grid;

mod shaders;
mod square;
mod grid;
mod cell;
mod seeds;

const UPDATES_PER_SECOND: u64 = 100;

fn main() {
    let width = 1024.0;
    let height = 768.0;
    let mut gen = 0;
    let mut chksum1 = 0;
    let mut chksum2 = 0;
    let mut render_gen = 0;

    let seed = env::args().nth(1).map(|s|
        seeds::named(&s).expect("Invalid seed name! Valid seeds are random or gosper_glider")
    ).unwrap_or(seeds::random);

    let display = glutin::WindowBuilder::new()
        .with_dimensions(width as u32, height as u32)
        .with_title(format!("Conways \"Game of Life\" by Rust"))
        .with_vsync()
        .build_glium()
        .unwrap();

    let (vertices, indices) = square::geometry(&display);
    let program = Program::from_source(&display, shaders::VERTEX, shaders::FRAGMENT, None).unwrap();

    let uniforms = uniform! {
        view_transform: [
            [ 1.0 / width, 0.0         , 0.0, 0.0],
            [ 0.0        , 1.0 / height, 0.0, 0.0],
            [ 0.0        , 0.0         , 1.0, 0.0],
            [-1.0        , 1.0         , 0.0, 1.0f32]
        ]
    };

    let square_size = 8.0;

    // Arc is needed until thread::scoped is stable
    let grid = Arc::new(Mutex::new(Grid::new(seed, 256, 192, square_size)));

    {
        let grid = grid.clone();
        // Spawn off thread to update the grid. Main thread will be in charge of rendering
        thread::spawn(move || {
            loop {
                std::thread::sleep(Duration::from_millis(1000 / UPDATES_PER_SECOND));
                grid.lock().unwrap().update();
                let chksum = grid.lock().unwrap().checksum;
                if gen > 2 {
                    if chksum1 == chksum || chksum2 == chksum {
                        println!("duplicate chksum found");
                        return;
                    }                    
                    if gen % 2 == 0 {
                        chksum1 = chksum;
                    } else {
                        chksum2 = chksum;
                    }
                    if gen % 100 == 0 {
                        println!("gen: {} chksum1: {} chksum2 {}", gen, chksum1, chksum2);
                    }
                }
                gen += 1;
            }
        });
    }

    loop {
        render_gen += 1;
        std::thread::sleep(Duration::from_millis(100));
        if render_gen % 100 == 0 {
            println!("render_gen: {} chksum1: {} chksum2 {}", render_gen, chksum1, chksum2);
        }
        let instances = {
            let grid = grid.lock().unwrap();
            square::instances(&display, &grid.cells)
        };

        let mut frame = display.draw();
        frame.clear_color(1.0, 1.0, 1.0, 1.0);
        frame.draw((&vertices, instances.per_instance().unwrap()), &indices, &program, &uniforms, &std::default::Default::default()).unwrap();
        frame.finish();

        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
