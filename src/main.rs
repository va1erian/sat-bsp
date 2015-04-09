#![feature(convert)]
extern crate cgmath;

mod bsp;
mod geometry;
mod map;

fn main() {
    println!("Hello, world!");

    let p = geometry::Pt::new(2.4f32, 3f32 ,4f32);
    let w = geometry::Polygon {vert : vec![p, p, p] };

    for seg in w.seg_iterator().take(9) {
    	println!("{:?}", seg);
    }

    println!("{:?}", w);

    bsp::print_kek();
}
