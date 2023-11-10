#[cfg(test)]
mod tests {
    use crate::geometry::{Point3, Vector3};

    #[test]
    fn test_vector_length() {
        let v = Vector3(3.0, 4.0, 0.0);
        assert_eq!(v.length(), 5.0);
    }

    #[test]
    fn test_vector_length_squared() {
        let v = Vector3(3.0, 4.0, 0.0);
        assert_eq!(v.length_squared(), 25.0);
    }

    #[test]
    fn test_vector_normalize() {
        let v = Vector3(3.0, 4.0, 0.0);
        let normalized = v.normalize();
        assert_eq!(normalized.length(), 1.0);
    }

    #[test]
    fn test_vector_dot() {
        let v1 = Vector3(1.0, 2.0, 3.0);
        let v2 = Vector3(4.0, 5.0, 6.0);
        assert_eq!(Vector3::dot(v1, v2), 32.0);
    }

    #[test]
    fn test_vector_cross() {
        let v1 = Vector3(1.0, 2.0, 3.0);
        let v2 = Vector3(4.0, 5.0, 6.0);
        let cross = Vector3::cross(&v1, &v2);
        assert_eq!(cross, Vector3(-3.0, 6.0, -3.0));
    }

    #[test]
    fn test_point_add_vector() {
        let p = Point3(1.0, 2.0, 3.0);
        let v = Vector3(4.0, 5.0, 6.0);
        let result = p + v;
        assert_eq!(result, Point3(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_point_sub_point() {
        let p1 = Point3(1.0, 2.0, 3.0);
        let p2 = Point3(4.0, 5.0, 6.0);
        let result = p1 - p2;
        assert_eq!(result, Vector3(-3.0, -3.0, -3.0));
    }
}
