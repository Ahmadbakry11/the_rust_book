#[derive(Debug, Copy, Clone)]

pub struct Shoe<'a> {
    pub size: u32,
    pub style: &'a str,
}
