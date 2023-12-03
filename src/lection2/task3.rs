//use super::rectangle;
#[derive(Default, Debug)]
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

trait Rect<T> : Default {
    fn new(x: T, y : T, width: T, height: T) -> Self;
    
    // fn adjust(&mut self, dx1: T, dy1: T, dx2: T, dy2: T);
    // fn adjust_const(&self, dx1: T, dy1: T, dx2: T, dy2: T) -> Self;
//почему прямоугольник складывается с чем-то по dx1 и dx2?
//что это за числа?
//как я понимаю, прямоугольник задается точкой
//но если х и у - позиции центра, то это объясяет эти методы  

//что вообще должны делать эти функции?
    // fn left(&self) -> T;
    // fn right(&self) -> T;
    // fn top(&self) -> T;
    // fn bottom(&self) -> T;

    fn x(&self) -> T;
    fn y(&self) -> T;

    fn height(&self) -> T;
    fn width(&self) -> T;

    fn bottom_left(&self) -> Point<T>;
    fn bottom_right(&self) -> Point<T>;
    fn top_left(&self) -> Point<T>;
    fn top_right(&self) -> Point<T>;

// как разделить тип Т на 2.0?
    // fn center(&self) -> Point<T>;
    // fn contains_point(&self, point: &Point<T>) -> bool;
    // fn contains_rect(&self, point: &Rectangle<T>) -> bool;

    // fn is_intersected(&self, rect: &Rectangle<T>) -> bool;
    // fn intersected(&self, rect: &Rectangle<T>) -> Rectangle<T>;
    // fn united(&self, rect: &Rectangle<T>) -> Rectangle<T>;

    // fn transposed(&self) -> Rectangle<T>;
}


impl <T> Rect<T> for Rectangle<T> 
where 
    T:Default,
    T:Copy,
    T:std::ops::Add<Output=T>,
    T:std::ops::Div<Output=T>
{
    fn new(x: T, y : T, width: T, height: T) -> Self{
        Self { x: (x), y: (y), width: (width), height: (height) }
    }
    
    // fn adjust(&mut self, dx1: T, dy1: T, dx2: T, dy2: T);
    // fn adjust_const(&self, dx1: T, dy1: T, dx2: T, dy2: T) -> Self;

    // fn left(&self) -> T;
    // fn right(&self) -> T;
    // fn top(&self) -> T;
    // fn bottom(&self) -> T;

    fn x(&self) -> T {
        return self.x;
    }
    fn y(&self) -> T{
        return self.y;
    }

    fn height(&self) -> T{
        return self.height;
    }
    fn width(&self) -> T{
        return self.width;
    }

    fn bottom_left(&self) -> Point<T>{
        return Point{x:self.x, y:self.y};
    }
    fn bottom_right(&self) -> Point<T>{
        return Point{x:self.x + self.width, y:self.y};
    }
    fn top_left(&self) -> Point<T>{
        return Point{x:self.x, y:self.y + self.height};
    }
    fn top_right(&self) -> Point<T>{
        return Point{x:self.x + self.width, y:self.y + self.height};
    }

    // fn center(&self) -> Point<T>{
    //     return Point{x:self.x + self.width/2.0, y:self.y + self.height/2.0};    
    // }

    // fn contains_point(&self, point: &Point<T>) -> bool;
    // fn contains_rect(&self, point: &Rectangle<T>) -> bool;

    // fn is_intersected(&self, rect: &Rectangle<T>) -> bool;
    // fn intersected(&self, rect: &Rectangle<T>) -> Rectangle<T>;
    // fn united(&self, rect: &Rectangle<T>) -> Rectangle<T>;

    // fn transposed(&self) -> Rectangle<T>;
}
// trait std::ops::BitAnd{}
// trait std::ops::BitAndAssign
// trait std::ops::BitOr
// trait std::ops::BitOrAssign

#[test]
fn test(){
    let r = Rectangle::new(1.0, 2.0, 3.0, 4.0);
    println!("{:?}", r);
}