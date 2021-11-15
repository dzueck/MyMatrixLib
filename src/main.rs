use my_matrix_lib::vectors::VecN;
use my_matrix_lib::matrixs::Mat;
use std::time::Instant;

fn main() {
    let x = Vec3::from(5.0, 2.0, 1.0);
    let y = Vec3::from(-5.0, 8.0, 1.0);

    let start = Instant::now();   
    for i in 0..99999999 {
        x.dot2(&y);
    }
    let time = start.elapsed().as_nanos();
    println!("{}", time);

    let x = VecN::from([5.0, 2.0, 1.0]);
    let y = VecN::from([-5.0, 8.0, 1.0]);

    let start = Instant::now();   
    for i in 0..99999999 {
        let z = VecN::from([-5.0, 8.0, 1.0]);
        x.dot(&y);
    }
    let time = start.elapsed().as_nanos();
    println!("{}", time);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    
    use my_matrix_lib::vectors::Vec2;
    use my_matrix_lib::vectors::Vec3;
    use my_matrix_lib::vectors::VecN;


    #[test]
    fn test_add() {
        assert_eq!(0, 3);
    }

}
