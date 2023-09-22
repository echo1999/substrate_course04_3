trait Area {
    fn area(&self) -> f64;
}

// 圆形结构和它对 Area 的实现
struct Circle {
    radius: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        3.141592653589793 * self.radius * self.radius
    }
}

// 正方形结构和它对 Area 的实现
struct Square {
    side: f64,
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// 三角形结构和它对 Area 的实现
struct Triangle {
    base: f64,
    height: f64,
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

// 使用泛型定义打印面积的函数
fn print_area<T: Area>(shape: T) {
    println!("The area is: {}", shape.area());
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let square = Square { side: 4.0 };
    let triangle = Triangle { base: 6.0, height: 3.0 };

    print_area(circle);    
    print_area(square);    
    print_area(triangle);  
}
