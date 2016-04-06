use rand;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Lines;
use std::io;
use std::path::Path;
use std::boxed::Box;
use core::iter::Enumerate;

pub type Seed<'b> =  Fn(i16, i16) -> bool;

pub fn named<'b>(seed: &str) -> Option<Box<Seed>> {
	let res = Box::new(random);
    match seed {
        "random" => Some(Box::new(random)),
        "gosper_glider" => Some(Box::new(gosper_glider)),
        "binary_adder" => Some(Box::new(binary_adder)),     
        "44P5H2V0" => Some(Box::new(&read_pattern("#N 44P5H2V0\n
#O Dean Hickerson\n
#C The first 2c/5 spaceship to be discovered\n
#C http://www.conwaylife.com/wiki/index.php?title=44P5H2V0\n
x = 15, y = 11, rule = b3/s23\n
4bo5bo4b$3b3o3b3o3b$2bo2bo3bo2bo2b$b3o7b3ob$2bobo5bobo2b$4b2o3b2o4b$o\n
4bo3bo4bo$5bo3bo5b$2o3bo3bo3b2o$2bo2bo3bo2bo2b$4bo5bo!\n"))),
        _ => Some(Box::new(random)),
    }
}

struct Point {
	x: i32,
	y: i32
}

struct Lexer<'a> {
	lines: Enumerate<Lines<BufReader<&'a [u8]>>>,
	char_count: i32,
	current_x: i32,
	current_y: i32,
	points: Vec<Point>,
}

impl<'a> Lexer<'a> {
	
	fn new_by_str(content: &str) -> Lexer  {
		let mut br = BufReader::new(content.as_bytes()); 
		Lexer {
      		lines: br.lines().enumerate(),
			char_count: 0,
			current_x: 0,
			current_y: 0,
			points: Vec::<Point>::new(),
		}
	}
	
	fn read_next_line(&mut self, linenumber_p: &mut usize, line_p: &mut String) -> bool {
		let (linenumber, line) = self.lines.next().unwrap_or((99999999,Ok("".to_string())));
		let ln: usize = linenumber;
		if linenumber == 99999999 {
				return false;
		} else {
			*linenumber_p = ln;			
			*line_p	 = line.unwrap();
		
			return true;
		}
	}
	
	fn read_lines(&mut self) {
		let mut line = String::new();
		let mut linenumber = 0;
		let mut data_started = false;
		while self.read_next_line(&mut linenumber, &mut line) {
			println!("{}: {}", linenumber, line);
			let mut chars = line.chars();
			match chars.next() {
				Some(c) => {
					if !data_started {
						match c {
							'#' => continue,
							'x' => {
								data_started = true;
								continue;
							},
							_ => {
								println!("unexpected line");
								continue;
							},
						}
					} else {
						self.eat_char(c);
						loop {
							match chars.next() {
								Some('!') =>  {
									self.eat_char('!');
									data_started = false;
								},
								Some(c) => {
									self.eat_char(c);
								}
								None => break, 
							}							
						}
					}
				},
				None => continue,
			}			
		}
	}
	
    
		                	
	fn eat_char(&mut self, c: char) {
		println!("eating: {:?}",c);
		match c {
			'0' ... '9' => {
				self.char_count *= 10;
				self.char_count += c.to_digit(10).unwrap() as i32;
			},
			'!' => return,
			'$' => { self.current_x = 0; self.current_y += 1; }, 
			'b' => { 
				if self.char_count == 0 {
					self.char_count = 1;
				}
				self.current_x += self.char_count; 
				self.char_count = 0; },
			'o' => { 
				if self.char_count == 0 {
					self.char_count = 1;
				}
				for i in 0..self.char_count {
					self.points.push(Point { x: self.current_x, y: self.current_y});
					self.current_x += 1;
				} 
				self.char_count = 0;
			},
			_ => return,
			
		}		
	}
}


pub fn read_pattern(pattern: &str) -> &Seed {
	let mut lexer = Lexer::new_by_str(pattern);
	lexer.read_lines();
	let func: Seed = |x: i16, y: i16| true;
	&func
}

pub fn read_file(name: &str) -> Seed {
    
    &random
}

pub fn random<'b>(_: i16, _: i16) -> bool {
    rand::random()
}

pub fn binary_adder<'b>(x: i16, y: i16) -> bool {
    match (x, y) {
        (111,111) => true,
        (112,111) => true,
        (113,112) => true,
        (114,113) => true,
        _ => false
    }
}

pub fn gosper_glider<'b>(x: i16, y: i16) -> bool {
    match (x, y) {
        (1, 6) => true,
        (1, 5) => true,
        (2, 6) => true,
        (2, 5) => true,

        (11, 7) => true,
        (11, 6) => true,
        (11, 5) => true,

        (12, 8) => true,
        (12, 4) => true,

        (13, 9) => true,
        (13, 3) => true,
        (14, 9) => true,
        (14, 3) => true,

        (15, 6) => true,

        (16, 8) => true,
        (16, 4) => true,

        (17, 7) => true,
        (17, 6) => true,
        (17, 5) => true,

        (18, 6) => true,

        (21, 5) => true,
        (21, 4) => true,
        (21, 3) => true,

        (22, 5) => true,
        (22, 4) => true,
        (22, 3) => true,

        (23, 6) => true,
        (23, 2) => true,

        (25, 7) => true,
        (25, 6) => true,
        (25, 2) => true,
        (25, 1) => true,

        (35, 4) => true,
        (35, 3) => true,
        (36, 4) => true,
        (36, 3) => true,

        _ => false
    }
}
