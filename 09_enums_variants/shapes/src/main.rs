enum Shape {
    Circle(f64),
    Square(f64),
}

fn main() {
    let shapes = vec![Shape::Circle(1.1), Shape::Square(2.2)];
    let total_area: f64 = shapes
        .iter()
        .map(|shape| match shape {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Square(l) => l * l,
        })
        .sum();
    println!("Total Area: {:.3}", total_area);
}
