pub struct Point3D {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

fn main() {
    let p3d = Point3D {
        x: 7,
        y: 9,
        z: -1,
    };
    
    let result = is_first_octant(p3d);
    println!("{}", result);
}

pub fn is_first_octant(p3d: Point3D) -> bool {
    p3d.x >= 0 && p3d.y >= 0 && p3d.z >= 0
}