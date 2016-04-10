#[macro_use]
extern crate glium;
extern crate glutin;
extern crate rand;
extern crate time;
extern crate scoped_threadpool;
extern crate core;
extern crate getopts;

use glium::DisplayBuild;
use glium::Surface;
use glium::Program;
use time::PreciseTime;
use std::time::Duration;
use std::env;
use std::process;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::thread;
use std::sync::{Arc, Mutex};
use grid::Grid;
use scoped_threadpool::Pool;
use getopts::Options;
use getopts::Matches;

mod shaders;
mod square;
mod grid;
mod cell;
mod seeds;

const UPDATES_PER_SECOND: u64 = 100;

const THREAD_COUNT: usize = 3usize;                

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn get_options() -> Matches {
	let args: Vec<String> = env::args().collect();
	let program = args[0].clone();

	let mut opts = Options::new();
	opts.optopt("t","time","time to wait in milliseconds between generations <0: no wait","10");
	opts.optopt("q","square","square size","8");
	opts.optopt("g","generations","number of generations to check for repetitions","100");
	opts.optopt("w","width","width of screen","256");
	opts.optopt("h","height","height of screen","192");	
	opts.optopt("s","seed","seed - random gosper_glider ...","");	
	opts.optflag("?","help","print this help menu");	
	let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("?") {
        print_usage(&program, opts);
        process::exit(1);
    }
    matches
}          

fn main() {
	let m = get_options();
    let width = 1024.0;
    let height = 768.0;
    let mut gen = 0;
    let mut render_gen = 0;
    let start = PreciseTime::now();
    
    let mut chksums = HashMap::new();
    let mut chksumv: LinkedList<u64> = LinkedList::new();
    
    let pool = Arc::new(Mutex::new((Pool::new(THREAD_COUNT as u32))));

    let seed = seeds::named(&(m.opt_str("s").unwrap_or(String::from("gosper_glider")))[..]).unwrap_or(seeds::gosper_glider());

    
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

    let square_size = m.opt_str("q").unwrap_or(String::from("8.0")).parse::<f32>().unwrap_or(8.0);
    let xsize = m.opt_str("w").unwrap_or(String::from("256")).parse::<i16>().unwrap_or(256);
    let ysize = m.opt_str("h").unwrap_or(String::from("192")).parse::<i16>().unwrap_or(192);
    
    
    let wait_time = m.opt_str("t").unwrap_or(String::from("10")).parse::<i16>().unwrap_or(10);
    let f = seed.clip_and_centralize(xsize as u32, ysize as u32);
    let mut gen_to_check: usize = m.opt_str("g").unwrap_or(String::from("100")).parse::<i16>().unwrap_or(100) as usize;
    
    use std::sync::atomic::{AtomicBool, AtomicIsize, Ordering};
    
    let pause_generations = Arc::new(AtomicBool::new(false));
    let wait_time = Arc::new(AtomicIsize::new(wait_time as isize));

    // Arc is needed until thread::scoped is stable
    let grid = Arc::new(Mutex::new(Grid::new(f, xsize, ysize, square_size)));

    {
        let grid = grid.clone();
        let pause_generations = pause_generations.clone();
        let wait_time = wait_time.clone();
        // Spawn off thread to update the grid. Main thread will be in charge of rendering
        thread::spawn(move || {
            loop {
            	if pause_generations.load(Ordering::Relaxed) {
            		std::thread::sleep(Duration::from_millis(1000u64))
            	} else {
	                // std::thread::sleep(Duration::from_millis(1000 / UPDATES_PER_SECOND));
	                // grid.lock().unwrap().update1();
	                if wait_time.load(Ordering::Relaxed) >= 0 { std::thread::sleep(Duration::from_millis(wait_time.load(Ordering::Relaxed) as u64)); }
	                grid.lock().unwrap().update2(pool.clone());
	                let chksum = grid.lock().unwrap().checksum;
	                
	                if gen > 1000 {
	                	if chksums.contains_key(&chksum) {
	                		println!("duplicate chksum found in generation {} now I am in {}", chksums.get(&chksum).unwrap(), gen);
	                        let end = PreciseTime::now();
	                        println!("{} seconds for whatever you did.", start.to(end));
	                        return;
	                	}
	                    
	                    if gen % 100 == 0 {
	                        println!("gen: {}", gen);
	                    }
	                }
	                chksums.insert(chksum, gen);
	                chksumv.push_front(chksum);
	                if chksumv.len() > gen_to_check {
	                	let tmp = chksumv.pop_back().unwrap();
	                	chksums.remove(&tmp);
	                }
	                gen += 1;
            	}
            }
        });
    }
    
    let mut act_x = 0;
    let mut act_y = 0;

    loop {
        render_gen += 1;
        std::thread::sleep(Duration::from_millis(50));
        if render_gen % 100 == 0 {
            println!("render_gen: {}", render_gen);
        }
        let instances = {
            let grid = grid.lock().unwrap();
            square::instances(&display, &grid.cells)
        };

        let mut frame = display.draw();
        frame.clear_color(1.0, 1.0, 1.0, 1.0);
        frame.draw((&vertices, instances.per_instance().unwrap()), &indices, &program, &uniforms, &std::default::Default::default()).unwrap();
        frame.finish();
		
		use glium::glutin::ElementState::*;
        use glium::glutin::Event::*;
        use glium::glutin::MouseScrollDelta::*;
        use glium::glutin::MouseButton;
        use glium::glutin::VirtualKeyCode;
        for event in display.poll_events() {
            match event {
            	KeyboardInput(Pressed,_,Some(VirtualKeyCode::P)) => {
            		println!("Key pressed");
            		pause_generations.store(!pause_generations.load(Ordering::Relaxed), Ordering::Relaxed);
            	},
            	KeyboardInput(Pressed,_,Some(VirtualKeyCode::Down)) => {
            		wait_time.store(wait_time.load(Ordering::Relaxed) + 10, Ordering::Relaxed);
            	},
            	KeyboardInput(Pressed,_,Some(VirtualKeyCode::Up)) => {
            		wait_time.store(wait_time.load(Ordering::Relaxed) - 10, Ordering::Relaxed);
            	},
            	MouseInput(Pressed,MouseButton::Left) => {
            		println!("Mouse pressed");
            		let mut grid = grid.lock().unwrap();
            		grid.set(act_x, act_y, true);	
            	},
            	MouseInput(Pressed,MouseButton::Right) => {
            		println!("Mouse pressed");
            		let mut grid = grid.lock().unwrap();
            		grid.set(act_x, act_y, false);	
            	},
            	MouseMoved((x, y))  => {
            		println!("mouse moved: {} {}",x,y);
            		act_x = x / (width / (xsize as f32)) as i32;
            		act_y = y / (height / (ysize as f32)) as i32;
            		println!("mouse moved squares: {} {} ",act_x,act_y);

            		
            	}
                glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
