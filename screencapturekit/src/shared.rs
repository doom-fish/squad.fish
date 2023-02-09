pub type DisplayID = u32;
pub type WindowID = u32;


#[derive(Debug)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

