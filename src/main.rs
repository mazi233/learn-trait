use std::time::SystemTime;

struct Circle;
struct Square;

trait Draw {
    fn draw(&self);
}

impl Draw for Circle {
    fn draw(&self) {
        // println!("circle");
    }
}

impl Draw for Square {
    fn draw(&self) {
        // println!("square");
    }
}

fn f<T: Draw>(shape1: &T, shape2: &T) {
    shape1.draw();
    shape2.draw();
}

fn main() {
    // trait object的创建：指针（eg：引用，智能指针）+ dyn关键字 + 要实现的trait（一个或多个，trait之间用+连接）
    // eg：&dyn Trait、Box<dyn Trait>。
    // 意思就是：只要一个类型实现了指定的trait(s)，即满足该约束

    let start_time = SystemTime::now();

    let box_circle: Box<dyn Draw> = Box::new(Circle);
    let box_square: Box<dyn Draw> = Box::new(Square);

    let shapes: Vec<Box<dyn Draw>> = vec![box_circle, box_square];

    for _ in 0..20_000_000 as u64 {
        for shape in shapes.iter() {
            shape.draw();
        }
    }

    println!("use time: {}", start_time.elapsed().unwrap().as_millis());
}
