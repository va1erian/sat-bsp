use cgmath::{Point, Vector, Point3, Vector3, Intersect};

pub type SCALAR = f32;

pub type Vect = Vector3<SCALAR>;
pub type Pt   = Point3<SCALAR>;

#[derive(Debug)]
pub struct Polygon {
	pub vert : Vec<Pt>
}

impl Polygon {
	pub fn seg_iterator(&self) -> PolygonSegIterator {
		assert!(self.vert.len() > 2);
		PolygonSegIterator{ start: 0, end : 1, poly : self }
	}
}

struct PolygonSegIterator<'a> {
 	start : usize,
 	end   : usize,
	poly  : &'a Polygon
}

impl<'a> Iterator for PolygonSegIterator<'a> {
	type Item = (Pt, Pt);

	fn next(&mut self) -> Option<(Pt, Pt)> {
		let start = self.poly.vert[self.start];
		let end   = self.poly.vert[self.end];

		Some((start, end))
	}
}



#[derive(Debug)]
pub struct Face {
	poly  : Box<Polygon>,
	norm  : Vect
} 

impl Face {
	pub fn new(polygon : Box<Polygon> ) -> Face {
		assert!(polygon.vert.len() > 2);

		Face{ norm: Face::mk_normal(&polygon), poly : polygon }
	}


	fn intersection(&self, face : &Face) -> Option<(Face, Face)> {
		let mut on_back  = Vec::new();
		let mut on_front = Vec::new();

		let v = self.poly.vert[0];

		match incidence(v, face) {
			Incidence::Back  => { on_back.push(v);  }
			Incidence::Front => { on_front.push(v); }
			Incidence::Coincident => {
				on_back.push(v);
				on_front.push(v);
			}
		}

		None
	}

	fn mk_normal( poly: &Polygon ) -> Vect {
		Vect::new (0.0f32, 0.0f32 , 0.0f32)
	}

}


enum Incidence {
	Back,
	Front,
	Coincident
}


fn incidence(p: Pt, f : &Face) -> Incidence {
	let face_pt = f.poly.vert[0];
	let dir   = face_pt.sub_p(&p);
	let res   = dir.dot(&f.norm);

	if res < -0.001f32
		{ Incidence::Back } 
	else if res > 0.001f32 
		{ Incidence::Front }
	else 
		{ Incidence::Coincident }
}
