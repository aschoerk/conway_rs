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
use std::collections::HashSet;


pub struct Seed {
	points: HashSet<u64>,
}

impl Seed {
	fn new() -> Seed {		
		Seed {
			points: HashSet::new(),
		}
	}
	fn hash(x: i32, y:i32) -> u64 {
		x as u64 | ((y as u64) << 32)
	}
	
	fn add(&mut self, x: i32, y: i32) -> &Seed {		
		self.points.insert(Seed::hash(x,y));
		self
	}
	
	pub fn clip_and_centralize(&mut self, width: i32, height: i32) {
		
	}
	
	pub fn contains(&self, x: i32, y:i32) -> bool {		
		self.points.contains(&Seed::hash(x,y))
	}
	
	pub fn containsi16(&self, x: i16, y:i16) -> bool {		
		self.points.contains(&Seed::hash(x as i32, y as i32))
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
		fn points_as_set(&self) -> Seed {
			let mut seed = Seed::new();
			for p in &self.points {
				seed.add(p.x, p.y);
			}
			return seed;
		}
}

pub fn named<'b>(seed: &str) -> Option<Seed> {
    match seed {
        "random" => Some(random()),
        "gosper_glider" => Some(gosper_glider()),
        "binary_adder" => Some(binary_adder()),     
        "44P5H2V0" => Some(read_pattern("#N 44P5H2V0\n
#O Dean Hickerson\n
#C The first 2c/5 spaceship to be discovered\n
#C http://www.conwaylife.com/wiki/index.php?title=44P5H2V0\n
x = 15, y = 11, rule = b3/s23\n
4bo5bo4b$3b3o3b3o3b$2bo2bo3bo2bo2b$b3o7b3ob$2bobo5bobo2b$4b2o3b2o4b$o\n
4bo3bo4bo$5bo3bo5b$2o3bo3bo3b2o$2bo2bo3bo2bo2b$4bo5bo!\n")),
        _ => Some(random()),
    }
}

pub fn read_pattern(pattern: &str) -> Seed {
	let mut lexer = Lexer::new_by_str(pattern);
	lexer.read_lines();
	lexer.points_as_set()
}


pub fn random() -> Seed {
	let mut res = Seed::new();
	for i in 0..100 {
		for j in 0..100 {
            if rand::random() {
			  	res.add(i,j);            	
            }
		}
	}
	res
}

pub fn binary_adder() -> Seed {
	let mut res = Seed::new();
	res.add(111, 111); res.add(112,111);res.add(113,112);res.add(114,113);
	res
}

pub fn gosper_glider() -> Seed {
    let mut res = Seed::new();
    res.add(1, 6);
    res.add(1, 5);
    res.add(2, 6);
    res.add(2, 5);

    res.add(11, 7);
    res.add(11, 6);
    res.add(11, 5);

    res.add(12, 8);
    res.add(12, 4);

    res.add(13, 9);
    res.add(13, 3);
    res.add(14, 9);
    res.add(14, 3);

    res.add(15, 6);

    res.add(16, 8);
    res.add(16, 4);

    res.add(17, 7);
    res.add(17, 6);
    res.add(17, 5);

    res.add(18, 6);

    res.add(21, 5);
    res.add(21, 4);
    res.add(21, 3);

    res.add(22, 5);
    res.add(22, 4);
    res.add(22, 3);

    res.add(23, 6);
    res.add(23, 2);

    res.add(25, 7);
    res.add(25, 6);
    res.add(25, 2);
    res.add(25, 1);

    res.add(35, 4);
    res.add(35, 3);
    res.add(36, 4);
    res.add(36, 3);

    res
}
