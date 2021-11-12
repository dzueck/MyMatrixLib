use my_vector_math::Vec2;
use my_vector_math::Vec3;
use my_vector_math::VecN;

fn main() {
    let d = VecN::new([4.0;5]);
    let x = VecN::new([4.0;5]);

    println!("{:?}", d + x);
}

