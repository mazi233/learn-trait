trait Animal {
    fn new(name: &'static str) -> Self;

    fn name(&self) -> &'static str;

    fn noise(&self) -> &'static str;

    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

struct Cat {
    name: &'static str,
    age: i32
}

impl Animal for Cat {
    fn new(name: &'static str) -> Self {
        Cat { name, age: 1 }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        "Meowww"
    }

    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.age)
    }
}

struct Dog {
    name: &'static str
}

impl Animal for Dog {
    fn new(name: &'static str) -> Self {
        Dog { name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        "<random factory noise>"
    }
}

fn main() {
    let c: Cat = Animal::new("小猫");
    println!("Cat's name is {} {}", c.name(), c.age);
    c.talk();

    let d: Dog = Animal::new("小狗xx");
    d.talk();
}
