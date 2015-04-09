use std::collections::HashMap;
use std::str::FromStr;

use geometry::Pt;

type MapBrush = Vec<BrushPlane>;

#[derive(Debug)]
struct BrushPlane {
	pub a : Pt,
	pub b : Pt,
	pub c : Pt,
	pub tex : String,
	pub x_offset : f32,
	pub y_offset : f32,
	pub rot      : f32,
	pub x_scale  : f32,
	pub y_scale  : f32
}

#[derive(Debug)]
struct Entity {
	pub properties : HashMap<String, String>,
	pub brushes    : Option<Vec<MapBrush>>
}

enum EntityItem {
	Pair(String, String),
	Brush(MapBrush),
}

#[derive(Debug)]
struct Map {
	//pub world : Entity,
	pub entities : Vec<Entity>
}

struct Parser<'a> {
	src    : &'a Vec<char>,
	cursor : usize,
	line   : i32
}

type ParseResult<T> = Result<T, String>;

impl<'a> Parser<'a> {
	pub fn new(source : &'a Vec<char> ) -> Parser {
		Parser { src : source, line : 1 , cursor : 0}
	}

	pub fn parse(&mut self) -> ParseResult<self::Map> {
		let mut ents = vec![];

		loop {
			if let Some('{') = self.peek() {
				ents.push( try!(self.entity()) );
			} else {
				break
			}
		}

		Ok( Map { entities : ents })
	}

	fn quoted(&mut self) -> ParseResult<String> {		
		try!(self.expect('"'));

		let mut buf = String::new();
		
		while let Some(c) = self.peek() {
			if c == '"' {
				self.next_char();
				return Ok(buf)
			} else {
				buf.push(c);
				self.next_char();
			}
		}

		return Err("Unexpected string end".to_string())
	}

	fn pair(&mut self) -> ParseResult<(String, String)> {
		let k = try!(self.quoted());

		let v = try!(self.quoted());

		Ok((k, v))
	}

	fn point(&mut self) -> ParseResult<Pt> {
		try!(self.expect('('));

		let p = Pt::new(
				try!(self.float()),
				try!(self.float()),
				try!(self.float())
			);

		try!(self.expect(')'));

		Ok(p)
	}

	fn float(&mut self) -> ParseResult<f32> {
		let mut buf = String::new();

		self.next();

		while let Some(c) = self.peek() {
			if c == '-' || c == '.' || c == '+' || c.is_numeric() {
				buf.push(c);
				self.next_char();
			}  else {
				break
			}
		}

		if let Ok(f) = FromStr::from_str(buf.as_str()){
			Ok(f)
		}  else {
			Err(format!("Line {} - Failed to parse {}",self.line, buf))
		}
	}

	fn plane(&mut self) -> ParseResult<BrushPlane> {
		let p = BrushPlane {
			a:        try!(self.point()),
			b:        try!(self.point()),
			c:        try!(self.point()),
			tex:      try!(self.tex_name()),
			x_offset: try!(self.float()),
			y_offset: try!(self.float()),
			rot:      try!(self.float()),
			x_scale:  try!(self.float()),
			y_scale:  try!(self.float())
		};

		self.next_char();
		Ok(p)
	}

	fn tex_name(&mut self) -> ParseResult<String> {
		self.next();

		let mut buf = String::new();

		while let Some(c) = self.peek() {
			if c.is_alphanumeric() || c == '_' || c == '#' 
			|| c ==',' || c == '+' || c == '*' {
				buf.push(c);
				self.next_char();
			} else {
				break
			}
		}

		Ok(buf)
	}
	fn brush(&mut self) -> ParseResult<MapBrush> {
		try!(self.expect('{'));

		let mut planes = vec![];
		
		loop {
			self.next();
			if let Some('}') = self.peek() {
				self.next_char();
				break 
			}

			planes.push(try!(self.plane()));
		}

		Ok(planes)
	}

	fn entity_item(&mut self) -> ParseResult<EntityItem> {
		self.next();

		match self.peek() {
			Some('{') => Ok(EntityItem::Brush(try!(self.brush()))),
			Some('"') => {
				let (k,v) = try!(self.pair());
				Ok(EntityItem::Pair(k,v))
			},
			Some(c)   => Err(format!("Expected pair or brush start, found {}", c)),
			None	  => Err("Unexpected end of file".to_string())
		}
	}	

	fn entity(&mut self) ->ParseResult<Entity> {
		try!(self.expect('{'));

		let mut map = HashMap::new();
		let mut brushes = vec![];

		loop {
			self.next();
			if let Some('}') = self.peek() { 
				self.next_char();
				break; 
			}
			
			match self.entity_item() {
				Ok(EntityItem::Pair( k, v ))   => { map.insert(k,v); },
				Ok(EntityItem::Brush( b ))     => { brushes.push(b); },
				Err(msg) => return Err(msg)
			}
		}

		let ent = Entity {
			properties : map,
			brushes: if brushes.is_empty() {None} else {Some(brushes)}
		};
		Ok(ent)
	}

	fn peek(&self) -> Option<char> {
		match self.src.get(self.cursor) {
			Some( c ) => return Some( *c ),
			None      => return None
		}
	}
	fn next(&mut self)  {
		while let Some(c) = self.peek() {
			if c.is_whitespace() || c.is_control() {
				self.next_char();
			} else {
				break
			}
		}
	}

	fn next_char(&mut self) {
		self.cursor += 1;
		if let Some('\n') = self.peek() {
			self.line += 1
		}
	}

	fn expect(&mut self, c : char) -> Result<(), String> {
		self.next();
		match self.peek() {
			Some(n) if n == c => { self.next_char(); Ok(()) },
			Some(n) => Err(format!("Expected {}, found {}", c, n)),
			None    => Err("Unexpected EOF".to_string())
		}
	}
}

#[test]
fn test_parsing() {
	let test : Vec<char> = "{\"fuck\" \"you\" \"top\" \"kek\" } { \"fuck\" \"you\" \"top\" \"kek\" }".chars().collect();

	let mut parser = Parser::new(&test);
	let result     = parser.parse();

	result.unwrap();
}

#[test]
fn test_file() {
	use std::fs::File;
	use std::path::Path;
	use std::io::Read;

	let path = Path::new("jrbase1.map");

	let mut file = File::open(&path).unwrap();
	let mut content = String::new();

	file.read_to_string(&mut content).unwrap();
	let vec_content = content.chars().collect();

	let mut parser = Parser::new(&vec_content);
	let result = parser.parse();

	result.unwrap();
}