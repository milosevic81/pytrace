use rust_tracer::Vector3;

#[test]
fn test_vector_creation() {
    let v = Vector3::new(1.0, 2.0, 3.0);
    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 2.0);
    assert_eq!(v.z, 3.0);
}

#[test]
fn test_magnitude() {
    let test_cases = vec![
        (Vector3::new(0.0, 0.0, 0.0), 0.0),
        (Vector3::new(-2.0, 2.0, 1.0), 3.0),
    ];

    for (vector, expected) in test_cases {
        assert_eq!(vector.magnitude(), expected);
    }
}

#[test]
fn test_add() {
    let test_cases = vec![
        (Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0)),
        (Vector3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0)),
        (Vector3::new(1.0, 1.0, 1.0), Vector3::new(1.0, 1.0, 1.0), Vector3::new(2.0, 2.0, 2.0)),
        (Vector3::new(1.0, 1.0, 1.0), Vector3::new(-1.0, -1.0, -1.0), Vector3::new(0.0, 0.0, 0.0)),
    ];

    for (v1, v2, expected) in test_cases {
        assert_eq!(v1 + v2, expected);
    }
}

#[test]
fn test_multiply() {
    let test_cases = vec![
        (Vector3::new(0.0, 0.0, 0.0), 3.0, Vector3::new(0.0, 0.0, 0.0)),
        (Vector3::new(1.0, 0.0, 0.0), 3.0, Vector3::new(3.0, 0.0, 0.0)),
        (Vector3::new(1.0, 1.0, 1.0), 3.0, Vector3::new(3.0, 3.0, 3.0)),
        (Vector3::new(1.0, 1.0, 1.0), 0.0, Vector3::new(0.0, 0.0, 0.0)),
    ];

    for (v1, scalar, expected) in test_cases {
        assert_eq!(v1 * scalar, expected);
    }
}

#[test]
fn test_divide() {
    let test_cases = vec![
        (Vector3::new(0.0, 0.0, 0.0), 3.0, Vector3::new(0.0, 0.0, 0.0)),
        (Vector3::new(3.0, 0.0, 0.0), 3.0, Vector3::new(1.0, 0.0, 0.0)),
        (Vector3::new(9.0, 9.0, 9.0), 3.0, Vector3::new(3.0, 3.0, 3.0)),
    ];

    for (v1, scalar, expected) in test_cases {
        assert_eq!(v1 / scalar, expected);
    }
}

#[test]
fn test_normalize() {
    let test_cases = vec![
        (Vector3::new(0.0, 0.0, 0.0), Vector3::zero()),
        (Vector3::new(1.0, 1.0, 1.0), Vector3::new(2.0, 2.0, 2.0)),
    ];

    for (v1, v2) in test_cases {
        assert_eq!(v1.normalize(), v2.normalize());
    }
}