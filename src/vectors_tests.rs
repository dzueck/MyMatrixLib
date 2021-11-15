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

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(bad_add(1, 2), 3);
    }
}
