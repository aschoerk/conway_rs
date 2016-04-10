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
	
	fn add(&mut self, x: i32, y: i32) {		
		self.points.insert(Seed::hash(x,y));
	}
	
	pub fn clip_and_centralize(&self, width: u32, height: u32) -> Seed {
		let mut maxx = 0;
		let mut minx = 999999999999;
		let mut maxy = 0;
		let mut miny = 999999999999;
		for p in self.points.iter() {
			let x: u32 = (p & 0xFFFFFFFF) as u32;
			let y: u32 = (p >> 32) as u32;
			minx = if x >= minx { minx } else { x };
			miny = if y >= miny { miny } else { y };
			maxx = if x > maxx { x } else { maxx };
			maxy = if y > maxy { y } else { maxy };		
		}
		let halfx = (maxx as i32 - minx as i32) >> 1;
		let halfy = (maxy as i32 - miny as i32) >> 1;
		let movex = (width >> 1) as i32 - halfx;
		let movey = (height >> 1) as i32 - halfy;
		let mut result = Seed::new();
		for p in self.points.iter() {
			let x: i32 = (p & 0xFFFFFFFF) as i32;
			let y: i32 = (p >> 32) as i32;
			result.add(x + movex, y + movey);
		} 
		result
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
		match c {
			'0' ... '9' => {
				self.char_count *= 10;
				self.char_count += c.to_digit(10).unwrap() as i32;
			},
			'!' => return,
			'$' => { 
				if self.char_count == 0 {
					self.char_count = 1;
				}
				self.current_x = 0; 
				self.current_y += self.char_count; 
				self.char_count = 0;
				}, 
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
        "10enginecordership" => Some(read_pattern("#N 10-engine Cordership\n
#O Dean Hickerson\n
#C A c/12 period 96 diagonal Cordership that uses 10 switch engines, which was the fewest possible known at the time of its discovery.\n
#C www.conwaylife.com/wiki/index.php?title=10-engine_Cordership\n
x = 88, y = 88, rule = 23/3\n
42bo45b$42bo45b$44bo5bo37b$43bo6bobo35b$42bo3bo2bo38b$43bo2bobob2o36b$\n
48bob2o36b$62b2o24b$62b2o24b7$70b2o16b$26b2o2bo39b2o16b$29bobo56b$28bo\n
59b2$30b2o56b2$31b2o55b$30bo47b2o8b$28b2ob2o45b2o8b$31b2o55b$16bo12bo\n
58b$16bo22b3o46b$18bo5bo13bo49b$17bo6bobo10bo4b2o44b$16bo3bo2bo12bo3bo\n
47b$17bo2bobob2o10bo2bo4bo41b2o$22bob2o10bo3bo3bo32bo8b2o$37b2obob3o\n
31bobo9b$40bo47b$41b4o31bo2bo8b$30b3o10b2o33b2o8b$29bo3bo45bo8b$28bo4b\n
o54b$27bo3bo56b$27bo2bob3o53b$27bo7bo52b$2o2bo24bo3bobo40b2o10b$3bobo\n
23bo3bob2o41bo9b$2bo28b3ob2o39b2o10b2$4b2o82b2$5b2o81b$4bo83b$2b2ob2o\n
52bobo6bobo17b$5b2o51bo9bobo17b$3bo55bo2bo6bo18b$61b3o24b5$51bo36b$50b\n
obo35b2$50bo2bo34b$7b2o43b2o34b$7b2o44bo34b5$50b2o36b$52bo35b$15b2o33b\n
2o36b$15b2o71b5$33bobo6bobo43b$32bo9bobo43b$23b2o8bo2bo6bo44b$23b2o10b\n
3o50b7$31b2o55b$31b2o!\n")),
        "Cis-beacon" => Some(read_pattern("#N Cis-beacon up and long hook\n
#C A period 2 oscillator composed of a beacon and a long hook.\n
#C www.conwaylife.com/wiki/index.php?title=Beacon_and_long_hook\n
x = 5, y = 8, rule = B3/S23\n
2b2ob$3bob$o4b$2o3b2$4ob$o3bo$3b2o!\n")),

         "4enginecordership" => Some(read_pattern("#N 4-engine Cordership\n
#O David Bell\n
#C A 4-engine c/12 period 96 diagonal Cordership found on July 9, 2005.\n
#C www.conwaylife.com/wiki/index.php?title=4-engine_Cordership\n
x = 76, y = 76, rule = 23/3\n
46bo29b$46bo29b$48bo5bo21b$47bo6bobo19b$46bo3bo2bo22b$47bo2bobob2o20b$\n
52bob2o20b$66b2o8b$66b2o8b7$74b2o$74b2o$55b2o3bo15b$53b3o4b2o14b$48b3o\n
bo8b2o13b$52bobo2b5o14b$53bo3b3o16b3$40bo35b$40bo7bo27b$40bo5b3o27b$\n
39b2o3bobo29b$45bo2bo27b$44bo2b2o27b10$27bo48b$24b4o48b4$27bobo46b$28b\n
o47b$2o2bo21b2o48b$3bobo20bo2bo46b$2bo16bo5b2ob2o46b$19bo56b$4b2o13bo\n
56b2$5b2o12b2o55b$4bo13bo2bo54b$2b2ob2o11bobo55b$5b2o10b2o57b$3bo13bo\n
58b$20b2o54b$20b2o54b$20b2o54b$17b2obo55b$18b3o55b$19bo56b4$7b2o67b$7b\n
2o67b7$15b2o59b$15b2o!\n")),
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
