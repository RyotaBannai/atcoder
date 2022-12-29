#[derive(Debug, Clone)]
pub struct Vertex {
    pub from: usize,
    pub to: usize,
    pub w: isize,
}
impl Vertex {
    pub fn new(from: usize, to: usize, w: isize) -> Self {
        Self { from, to, w }
    }
}
