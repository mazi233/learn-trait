struct Circle;
struct Square;

trait Draw {
    fn draw(&self);
}

impl Draw for Circle {
    fn draw(&self) {
        println!("circle");
    }
}

impl Draw for Square {
    fn draw(&self) {
        println!("square");
    }
}

fn f(shape: &impl Draw) {
    shape.draw();
}

fn main() {
    let circle = Circle;
    let square = Square;

    f(&circle);
    f(&square);
}
