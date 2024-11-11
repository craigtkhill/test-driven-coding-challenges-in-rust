pub fn sum_polygon_angles(n: i32) -> i32 {
    return (n - 2) * 180;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_polygons() {
        assert_eq!(sum_polygon_angles(3), 180); // triangle
        assert_eq!(sum_polygon_angles(4), 360); // square
        assert_eq!(sum_polygon_angles(5), 540); // pentagon
        assert_eq!(sum_polygon_angles(6), 720); // hexagon
    }

    #[test]
    fn test_larger_polygons() {
        assert_eq!(sum_polygon_angles(7), 900);
        assert_eq!(sum_polygon_angles(8), 1080);
        assert_eq!(sum_polygon_angles(9), 1260);
        assert_eq!(sum_polygon_angles(10), 1440);
    }

    #[test]
    fn test_random_polygons() {
        assert_eq!(sum_polygon_angles(15), 2340);
        assert_eq!(sum_polygon_angles(20), 3240);
        assert_eq!(sum_polygon_angles(25), 4140);
        assert_eq!(sum_polygon_angles(30), 5040);
    }

    #[test]
    fn test_large_polygons() {
        assert_eq!(sum_polygon_angles(50), 8640);
        assert_eq!(sum_polygon_angles(100), 17640);
        assert_eq!(sum_polygon_angles(500), 89640);
        assert_eq!(sum_polygon_angles(1000), 179640);
    }

    #[test]
    fn test_pattern() {
        // Test that the difference between consecutive polygons is constant
        assert_eq!(sum_polygon_angles(51) - sum_polygon_angles(50), 180);
        assert_eq!(sum_polygon_angles(101) - sum_polygon_angles(100), 180);
        assert_eq!(sum_polygon_angles(501) - sum_polygon_angles(500), 180);
    }
}
