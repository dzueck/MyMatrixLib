use my_vector_math::Vec2;
use my_vector_math::Vec3;
use my_vector_math::VecN;

fn main() {
    let d = Vec2::new(1.0,0.0);
    let x = Vec2::new(0.0,1.0);

    println!("{:?}", d.dot(x + d));
}

