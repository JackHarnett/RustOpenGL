#[derive(Copy, Clone, Debug)]
pub struct Vertex { 
	pub position : [f32; 2],
	pub tex_coords : [f32; 2],
}

glium::implement_vertex!(Vertex, position, tex_coords);