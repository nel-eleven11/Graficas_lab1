#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub x: isize,
    pub y: isize,
}

impl Vertex {
    pub fn new(x: isize, y: isize) -> Self {
        Vertex { x, y }
    }
}
