#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn point_add(&self, p2: &Point) -> Point {
        // your code here
        
        let mut final_value_x: f32 = 0.0;
        let mut final_value_y: f32 = 0.0;

        if self.x.is_nan() && p2.x.is_nan() {
            final_value_x = 0.0;
        }
        else if self.x.is_nan() {
            final_value_x += p2.x;
        }
        else if p2.x.is_nan() {
            final_value_x += self.x;
        }
        else {
            final_value_x += self.x + p2.x;
        }

        if self.y.is_nan() && p2.y.is_nan() {
            final_value_y = 0.0;
        }
        else if self.y.is_nan() {
            final_value_y += p2.y;
        }
        else if p2.y.is_nan() {
            final_value_y += self.y;
        }
        else {
            final_value_y += self.y + p2.y;
        }

        Point {
            x: final_value_x,
            y: final_value_y,
        }

    }
}

fn main() {
    let p1 = Point { x: 5.0, y: f32::NAN };
    let p2 = Point { x: 1.0, y: 1.0 };
    let result = p1.point_add(&p2);
    println!("{:?}", result); // Point { x: 6.0, y: 1.0 }
}
