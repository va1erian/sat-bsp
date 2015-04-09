struct Nil;

use geometry;

pub enum TreeItem {
	TreeNode {
		left  : Option<Box<TreeItem>>,
		right : Option<Box<TreeItem>>
	},
	TreeLeaf {
		solid: bool,
		value: geometry::Face
	}
}


pub fn print_kek() {
	println!("fuck you");
}

