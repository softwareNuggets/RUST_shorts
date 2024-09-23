trait Shape {
    fn area(&self) -> f64;
    fn display(&self);
}

struct Rectangle {
    width: f64,
    height: f64
}
        
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn display(&self) {
        println!("Rectangle: {}x{}", self.width, self.height);
    }
}

fn main() {
    let my_shape: Box<dyn Shape> = Box::new(Rectangle { width: 7.0, height: 4.0 });
 
    println!("Area: {}", my_shape.area());
    my_shape.display();
}