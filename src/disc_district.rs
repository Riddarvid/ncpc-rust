pub fn solve() {
    let (x, y) = find_closest_district(1);
    println!("{} {}", x, y);
}

fn find_closest_district(radius: u64) -> (u64, u64) {
    let mut closest_point = (0, 0);
    let mut closest_distance: f64 = (radius + 1000) as f64;
    for x in 0..=radius {
        let  y = closest_y_outside(radius, x);
        let distance = distance(x, y);
        println!("x: {x}, y: {y}, distance: {distance}");
        if distance < closest_distance {
            closest_point = (x, y);
            closest_distance = distance;
        }
    }
    closest_point
}

fn closest_y_outside(radius: u64, x: u64) -> u64 {
    let mut  y = ((radius * radius - x * x) as f64).sqrt() as u64;
    if distance(x, y) <= radius as f64 {
        y += 1;
    }
    y
}

fn distance(x: u64, y: u64) -> f64 {
    ((x * x + y * y) as f64).sqrt()
}