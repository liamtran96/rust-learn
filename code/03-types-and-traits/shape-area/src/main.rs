fn main() {
    let circle = Shape::Circle { radius: 2.0 };
    let triangle = Shape::Triangle {
        a: 3.0,
        b: 4.0,
        c: 5.0,
    };
    let rectangle = Shape::Rectangle { w: 3.0, h: 4.0 };
    println!("Circle: {}", circle.area());
    println!("Triangle: {}", triangle.area());
    println!("Rectangle: {}", rectangle.area());
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rectangle_area_is_width_times_height() {
        let shape = Shape::Rectangle { w: 3.0, h: 4.0 };
        assert_eq!(shape.area(), 12.0);
    }
    #[test]
    fn circle_area_is_radius() {
        let shape = Shape::Circle { radius: 2.0 };
        assert_eq!(shape.area(), 12.566370614359172);
    }
    #[test]
    fn triangle_area() {
        let shape = Shape::Triangle {
            a: 3.0,
            b: 4.0,
            c: 5.0,
        };

        assert_eq!(shape.area(), 6.0);
    }
}

enum Shape {
    Circle { radius: f64 },
    Rectangle { w: f64, h: f64 },
    Triangle { a: f64, b: f64, c: f64 },
}
impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { w, h } => w * h,
            Shape::Triangle { a, b, c } => {
                let s = (a + b + c) / 2.0;
                let area_squared = s * (s - a) * (s - b) * (s - c);
                area_squared.sqrt()
            }
        }
    }
}
