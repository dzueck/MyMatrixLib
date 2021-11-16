
fn main() {
}

#[cfg(test)]
mod vector_tests {
    use my_matrix_lib::vectors::VecN;

    #[test]
    fn vector_contains_values() {
        let mut x = VecN::from([2.0,4.0,5.0,3.0,6.0,0.0,888.0]);
        assert_eq!(x[0], 2.0);
        assert_eq!(x[1], 4.0);
        assert_eq!(x[2], 5.0);
        assert_eq!(x[3], 3.0);
        assert_eq!(x[4], 6.0);
        assert_eq!(x[5], 0.0);
        assert_eq!(x[6], 888.0);
        x[4] = 444.0;
        assert_eq!(x[0], 2.0);
        assert_eq!(x[1], 4.0);
        assert_eq!(x[2], 5.0);
        assert_eq!(x[3], 3.0);
        assert_eq!(x[4], 444.0);
        assert_eq!(x[5], 0.0);
        assert_eq!(x[6], 888.0);
    }

    #[test]
    fn vector_project() {
        let x = VecN::from([2.0,4.0,5.0]);
        let y = VecN::from([0.0,1.0,0.0]);
        assert_eq!(x.project(&y), VecN::from([0.0,4.0,0.0]));
        assert_eq!(y.project(&x), VecN::from([8.0/45.0,16.0/45.0,4.0/9.0]));
        let y = VecN::from([0.0,3904.0,0.0]);
        assert_eq!(x.project(&y), VecN::from([0.0,4.0,0.0]));
    }

    #[test]
    fn vector_cross() {
        let x = VecN::from([2.0, 3.0]);
        assert_eq!(x.cross(), VecN::from([3.0,-2.0]));
        let x = VecN::from([2.0, 3.0, -5.0]);
        let y = VecN::from([1.0, 0.0, 8.4]);
        assert_eq!(x.cross(&y), VecN::from([25.2,-21.8,-3.0]));
    }

    #[test]
    fn vector_length() {
        let x = VecN::from([2.0,4.0,5.0]);
        let y = VecN::from([0.0,1.0,0.0]);
        assert_eq!(x.length(), 6.708203932);
        assert_eq!(y.length(), 1.0);
    }

    #[test]
    fn vector_normalize() {
        let x = VecN::from([2.0,4.0,5.0]);
        let y = VecN::from([0.0,1.0,0.0]);
        let z = VecN::from([0.0,0.2,0.0]);
        assert_eq!(x.normalize(), VecN::from([0.29814239699997197, 0.5962847939999439, 0.7453559929]));
        assert_eq!(y.normalize(), y);
        assert_eq!(z.normalize(), VecN::from([0.0, 1.0, 0.0]));
    }

    #[test]
    fn vector_scales() {
        let x = VecN::from([2.0,4.0,5.0,3.0,6.0,0.0,888.0]);
        let x = x * 4.0;
        assert_eq!(x[0], 2.0 * 4.0);
        assert_eq!(x[1], 4.0 * 4.0);
        assert_eq!(x[2], 5.0 * 4.0);
        assert_eq!(x[3], 3.0 * 4.0);
        assert_eq!(x[4], 6.0 * 4.0);
        assert_eq!(x[5], 0.0 * 4.0);
        assert_eq!(x[6], 888.0 * 4.0);
        let x = VecN::from([2.0,4.0,5.0,3.0,6.0,0.0,888.0]);
        let x = x / 4.0;
        assert_eq!(x[0], 2.0 / 4.0);
        assert_eq!(x[1], 4.0 / 4.0);
        assert_eq!(x[2], 5.0 / 4.0);
        assert_eq!(x[3], 3.0 / 4.0);
        assert_eq!(x[4], 6.0 / 4.0);
        assert_eq!(x[5], 0.0 / 4.0);
        assert_eq!(x[6], 888.0 / 4.0);
    }

    #[test]
    fn vector_eq() {
        let x = VecN::from([2.0,4.0,5.0,3.0,6.0,0.0,888.0]);
        let y = VecN::from([2.0,4.0,5.0,3.0,6.00000002,0.0,888.0]);
        assert_eq!(x, y);
        let x = VecN::from([2.0,4.0,5.0,3.0,6.0,0.0,888.0]);
        let y = VecN::from([2.0,4.0,5.0,3.0,6.00010002,0.0,888.0]);
        assert_ne!(x, y);
    }

    #[test]
    fn dot_product() {
        let x = VecN::from([2.0,4.0]);
        let y = VecN::from([2.0,4.0]);
        assert_eq!(x * y, 2.0 * 2.0 + 4.0 * 4.0);
        let x = VecN::from([3.0,0.0]);
        let y = VecN::from([0.0,5.0]);
        assert_eq!(x * y, 0.0);
        let x = VecN::from([3.0,0.0,6.0]);
        let y = VecN::from([0.5,5.0,9.0]);
        assert_eq!(x * y, 3.0*0.5 + 6.0*9.0);
    }

    #[test]
    fn vector_add() {
        let x1 = 4.0;
        let x2 = 9.0;
        let y1 = 6.099;
        let y2 = 2.34;
        let x = VecN::from([x1, x2]);
        let y = VecN::from([y1, y2]);
        assert_eq!(x + y, VecN::from([x1 + y1, x2 + y2]));
        let x1 = 4.3398234;
        let x2 = 9.304034;
        let y1 = -2390.0;
        let y2 = 349.2;
        let x = VecN::from([x1, x2]);
        let y = VecN::from([y1, y2]);
        assert_eq!(x + y, VecN::from([x1 + y1, x2 + y2]));
        let x1 = 0.0;
        let x2 = 0.0;
        let y1 = 0.0;
        let y2 = 0.0;
        let x = VecN::from([x1, x2]);
        let y = VecN::from([y1, y2]);
        assert_eq!(x + y, VecN::from([x1 + y1, x2 + y2]));
        let x1 = 0.0;
        let x2 = 0.0;
        let y1 = 1.0;
        let y2 = 2.34;
        let x = VecN::from([x1, x2]);
        let y = VecN::from([y1, y2]);
        assert_eq!(x + y, VecN::from([x1 + y1, x2 + y2]));
        let x1 = 0.0;
        let x2 = 0.0;
        let x3 = 0.0;
        let y1 = 1.0;
        let y2 = 2.34;
        let y3 = 2.34;
        let x = VecN::from([x1, x2, x3]);
        let y = VecN::from([y1, y2, y3]);
        assert_eq!(x + y, VecN::from([x1 + y1, x2 + y2, x3 + y3]));
        let x1 = 0.0;
        let x2 = 0.0;
        let x3 = 0.0;
        let x4 = 0.0;
        let x5 = 0.0;
        let y1 = 1.0;
        let y2 = 2.34;
        let y3 = 2.34;
        let y4 = 2.34;
        let y5 = 2.34;
        let x = VecN::from([x1, x2, x3, x4, x5]);
        let y = VecN::from([y1, y2, y3, y4, y5]);
        assert_eq!(x + y, VecN::from([x1 + y1, x2 + y2, x3 + y3, x4 + y4, x5 + y5]));
        let x1 = 0.0;
        let x2 = 0.0;
        let y1 = 1.0;
        let y2 = 2.34;
        let x = VecN::from([x1, x2]);
        assert_ne!(x, VecN::from([x1 + y1, x2 + y2]));
    }

    #[test]
    fn vector_sub() {
        let x1 = 4.0;
        let x2 = 9.0;
        let y1 = 6.099;
        let y2 = 2.34;
        let x = VecN::from([x1, x2]);
        let y = VecN::from([y1, y2]);
        assert_eq!(x - y, VecN::from([x1 - y1, x2 - y2]));
        let x1 = 4.3398234;
        let x2 = 9.304034;
        let y1 = -2390.0;
        let y2 = 349.2;
        let x = VecN::from([x1, x2]);
        let y = VecN::from([y1, y2]);
        assert_eq!(x - y, VecN::from([x1 - y1, x2 - y2]));
        let x1 = 0.0;
        let x2 = 0.0;
        let y1 = 0.0;
        let y2 = 0.0;
        let x = VecN::from([x1, x2]);
        let y = VecN::from([y1, y2]);
        assert_eq!(x - y, VecN::from([x1 - y1, x2 - y2]));
        let x1 = 0.0;
        let x2 = 0.0;
        let y1 = 1.0;
        let y2 = 2.34;
        let x = VecN::from([x1, x2]);
        let y = VecN::from([y1, y2]);
        assert_eq!(x - y, VecN::from([x1 - y1, x2 - y2]));
        let x1 = 0.0;
        let x2 = 0.0;
        let x3 = 0.0;
        let y1 = 1.0;
        let y2 = 2.34;
        let y3 = 2.34;
        let x = VecN::from([x1, x2, x3]);
        let y = VecN::from([y1, y2, y3]);
        assert_eq!(x - y, VecN::from([x1 - y1, x2 - y2, x3 - y3]));
        let x1 = 0.0;
        let x2 = 0.0;
        let x3 = 0.0;
        let x4 = 0.0;
        let x5 = 0.0;
        let y1 = 1.0;
        let y2 = 2.34;
        let y3 = 2.34;
        let y4 = 2.34;
        let y5 = 2.34;
        let x = VecN::from([x1, x2, x3, x4, x5]);
        let y = VecN::from([y1, y2, y3, y4, y5]);
        assert_eq!(x - y, VecN::from([x1 - y1, x2 - y2, x3 - y3, x4 - y4, x5 - y5]));
        let x1 = 0.0;
        let x2 = 0.0;
        let y1 = 1.0;
        let y2 = 2.34;
        let x = VecN::from([x1, x2]);
        assert_ne!(x, VecN::from([x1 - y1, x2 - y2]));
    }

}

#[cfg(test)]
mod matrix_tests {
    
}
