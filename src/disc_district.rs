struct Point {
    x: i64,
    y: i64,
}

pub fn solve() {
    let point = find_closest_district(1);
    println!("{} {}", point.x, point.y);
}

fn find_closest_district(radius: i64) -> Point {
    let mut closest_point = Point {
        x: 0,
        y: radius + 1000,
    };
    let mut closest_distance = distance(&closest_point);
    for x in 0..=radius {
        let point = closest_point_outside(radius, x);
        let distance = distance(&point);
        if distance < closest_distance {
            closest_point = point;
            closest_distance = distance;
        }
    }
    closest_point
}

fn closest_point_outside(radius: i64, x: i64) -> Point {
    let x_square = x * x;
    let r_square = radius * radius;
    let mut y = ((r_square - x_square) as f64).sqrt() as i64;
    if distance_cords(x, y) <= radius as f64 {
        y += 1;
    }
    Point {
        x,
        y
    }
}

fn distance_cords(x: i64, y: i64) -> f64 {
    ((x * x + y * y) as f64).sqrt()
}

fn distance(point: &Point) -> f64 {
    distance_cords(point.x, point.y)
}