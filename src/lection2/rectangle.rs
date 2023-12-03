pub struct Rectangle<T> {
    x: T,
    y: T,
    width: T,
    height: T
}

pub struct Point<T> {
    x: T,
    y: T,
}

pub trait Rect<T> : Default {
    // fn new(x: T, y : T, width: T, height: T) -> Self;
    
    // fn adjust(&mut self, dx1: T, dy1: T, dx2: T, dy2: T);
    // fn adjust_const(&self, dx1: T, dy1: T, dx2: T, dy2: T) -> Self;

    // fn left(&self) -> T;
    // fn right(&self) -> T;
    // fn top(&self) -> T;
    // fn bottom(&self) -> T;

    fn x(&self) -> T;
    fn y(&self) -> T;

    // fn height(&self) -> T;
    // fn width(&self) -> T;

    // fn bottom_left(&self) -> Point<T>;
    // fn bottom_right(&self) -> Point<T>;
    // fn top_left(&self) -> Point<T>;
    // fn top_right(&self) -> Point<T>;

    // fn center(&self) -> Point<T>;
    // fn contains_point(&self, point: &Point<T>) -> bool;
    // fn contains_rect(&self, point: &Rectangle<T>) -> bool;

    // fn is_intersected(&self, rect: &Rectangle<T>) -> bool;
    // fn intersected(&self, rect: &Rectangle<T>) -> Rectangle<T>;
    // fn united(&self, rect: &Rectangle<T>) -> Rectangle<T>;

    // fn transposed(&self) -> Rectangle<T>;
}
