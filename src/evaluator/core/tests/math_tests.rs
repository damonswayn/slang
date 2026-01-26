use crate::object::Object;
use crate::test_support::eval_input;

#[test]
fn test_math_namespace_basic() {
    let input = r#"
        let a = Math::abs(-5);
        let b = Math::abs(-1.5);
        let c = Math::floor(1.9);
        let d = Math::ceil(1.1);
        let e = Math::round(1.6);
        let f = Math::min(2, 3.5);
        let g = Math::max(2, 3.5);
        let h = Math::pow(2, 3);

        [a, b, c, d, e, f, g, h];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 8);
            assert_eq!(vals[0], Object::Integer(5));
            assert_eq!(vals[1], Object::Float(1.5));
            assert_eq!(vals[2], Object::Integer(1));
            assert_eq!(vals[3], Object::Integer(2));
            assert_eq!(vals[4], Object::Integer(2));
            assert_eq!(vals[5], Object::Float(2.0));
            assert_eq!(vals[6], Object::Float(3.5));
            assert_eq!(vals[7], Object::Float(8.0));
        }
        other => panic!("expected array from Math namespace test, got {:?}", other),
    }
}

#[test]
fn test_math_logarithms() {
    let input = r#"
        let logE = Math::log(2.718281828459045);
        let log10_100 = Math::log10(100);
        let log2_8 = Math::log2(8);
        let exp1 = Math::exp(1);
        let exp0 = Math::exp(0);

        [logE, log10_100, log2_8, exp1, exp0];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 5);

            match &vals[0] {
                Object::Float(f) => assert!((*f - 1.0).abs() < 0.0001, "log(e) should be ~1"),
                other => panic!("expected float, got {:?}", other),
            }

            match &vals[1] {
                Object::Float(f) => assert!((*f - 2.0).abs() < 0.0001, "log10(100) should be 2"),
                other => panic!("expected float, got {:?}", other),
            }

            match &vals[2] {
                Object::Float(f) => assert!((*f - 3.0).abs() < 0.0001, "log2(8) should be 3"),
                other => panic!("expected float, got {:?}", other),
            }

            match &vals[3] {
                Object::Float(f) => assert!((*f - 2.718281828).abs() < 0.0001, "exp(1) should be ~e"),
                other => panic!("expected float, got {:?}", other),
            }

            match &vals[4] {
                Object::Float(f) => assert!((*f - 1.0).abs() < 0.0001, "exp(0) should be 1"),
                other => panic!("expected float, got {:?}", other),
            }
        }
        other => panic!("expected array from Math logarithms test, got {:?}", other),
    }
}

#[test]
fn test_math_inverse_trig() {
    let input = r#"
        let asin0 = Math::asin(0);
        let asin1 = Math::asin(1);
        let acos1 = Math::acos(1);
        let acos0 = Math::acos(0);
        let atan0 = Math::atan(0);
        let atan1 = Math::atan(1);
        let atan2_11 = Math::atan2(1, 1);
        let atan2_10 = Math::atan2(1, 0);

        [asin0, asin1, acos1, acos0, atan0, atan1, atan2_11, atan2_10];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 8);

            let pi = std::f64::consts::PI;

            match &vals[0] {
                Object::Float(f) => assert!(f.abs() < 0.0001, "asin(0) should be 0"),
                other => panic!("expected float, got {:?}", other),
            }

            match &vals[1] {
                Object::Float(f) => assert!((*f - pi / 2.0).abs() < 0.0001, "asin(1) should be π/2"),
                other => panic!("expected float, got {:?}", other),
            }

            match &vals[2] {
                Object::Float(f) => assert!(f.abs() < 0.0001, "acos(1) should be 0"),
                other => panic!("expected float, got {:?}", other),
            }

            match &vals[3] {
                Object::Float(f) => assert!((*f - pi / 2.0).abs() < 0.0001, "acos(0) should be π/2"),
                other => panic!("expected float, got {:?}", other),
            }

            match &vals[4] {
                Object::Float(f) => assert!(f.abs() < 0.0001, "atan(0) should be 0"),
                other => panic!("expected float, got {:?}", other),
            }

            match &vals[5] {
                Object::Float(f) => assert!((*f - pi / 4.0).abs() < 0.0001, "atan(1) should be π/4"),
                other => panic!("expected float, got {:?}", other),
            }

            match &vals[6] {
                Object::Float(f) => assert!((*f - pi / 4.0).abs() < 0.0001, "atan2(1,1) should be π/4"),
                other => panic!("expected float, got {:?}", other),
            }

            match &vals[7] {
                Object::Float(f) => assert!((*f - pi / 2.0).abs() < 0.0001, "atan2(1,0) should be π/2"),
                other => panic!("expected float, got {:?}", other),
            }
        }
        other => panic!("expected array from Math inverse trig test, got {:?}", other),
    }
}

#[test]
fn test_math_hyperbolic() {
    let input = r#"
        let sinh0 = Math::sinh(0);
        let cosh0 = Math::cosh(0);
        let tanh0 = Math::tanh(0);
        let tanhLarge = Math::tanh(10);

        [sinh0, cosh0, tanh0, tanhLarge];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 4);

            match &vals[0] {
                Object::Float(f) => assert!(f.abs() < 0.0001, "sinh(0) should be 0"),
                other => panic!("expected float, got {:?}", other),
            }

            match &vals[1] {
                Object::Float(f) => assert!((*f - 1.0).abs() < 0.0001, "cosh(0) should be 1"),
                other => panic!("expected float, got {:?}", other),
            }

            match &vals[2] {
                Object::Float(f) => assert!(f.abs() < 0.0001, "tanh(0) should be 0"),
                other => panic!("expected float, got {:?}", other),
            }

            match &vals[3] {
                Object::Float(f) => assert!((*f - 1.0).abs() < 0.0001, "tanh(10) should be ~1"),
                other => panic!("expected float, got {:?}", other),
            }
        }
        other => panic!("expected array from Math hyperbolic test, got {:?}", other),
    }
}

#[test]
fn test_math_expansions_error_handling() {
    let input1 = r#"Math::log("not a number");"#;
    let obj1 = eval_input(input1);
    assert!(obj1.is_error(), "Math::log with non-number should error");

    let input2 = r#"Math::atan2("a", 1);"#;
    let obj2 = eval_input(input2);
    assert!(obj2.is_error(), "Math::atan2 with non-number should error");
}

#[test]
fn test_math_constants() {
    let input = r#"
        let pi = Math::PI();
        let e = Math::E();
        let tau = Math::TAU();

        let tauCheck = tau - 2.0 * pi;

        [pi, e, tau, tauCheck];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 4);

            let pi = std::f64::consts::PI;
            let e = std::f64::consts::E;
            let tau = std::f64::consts::TAU;

            match &vals[0] {
                Object::Float(f) => assert!((*f - pi).abs() < 0.0001, "PI should be ~3.14159"),
                other => panic!("expected float for PI, got {:?}", other),
            }

            match &vals[1] {
                Object::Float(f) => assert!((*f - e).abs() < 0.0001, "E should be ~2.71828"),
                other => panic!("expected float for E, got {:?}", other),
            }

            match &vals[2] {
                Object::Float(f) => assert!((*f - tau).abs() < 0.0001, "TAU should be ~6.28318"),
                other => panic!("expected float for TAU, got {:?}", other),
            }

            match &vals[3] {
                Object::Float(f) => assert!(f.abs() < 0.0001, "TAU should equal 2*PI"),
                other => panic!("expected float, got {:?}", other),
            }
        }
        other => panic!("expected array from Math constants test, got {:?}", other),
    }
}

#[test]
fn test_math_sign() {
    let input = r#"
        let s1 = Math::sign(5);
        let s2 = Math::sign(-10);
        let s3 = Math::sign(0);
        let s4 = Math::sign(3.14);
        let s5 = Math::sign(-2.5);
        let s6 = Math::sign(0.0);

        [s1, s2, s3, s4, s5, s6];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 6);
            assert_eq!(vals[0], Object::Integer(1));
            assert_eq!(vals[1], Object::Integer(-1));
            assert_eq!(vals[2], Object::Integer(0));
            assert_eq!(vals[3], Object::Integer(1));
            assert_eq!(vals[4], Object::Integer(-1));
            assert_eq!(vals[5], Object::Integer(0));
        }
        other => panic!("expected array from Math::sign test, got {:?}", other),
    }
}

#[test]
fn test_math_clamp() {
    let input = r#"
        let c1 = Math::clamp(5, 0, 10);
        let c2 = Math::clamp(-5, 0, 10);
        let c3 = Math::clamp(15, 0, 10);
        let c4 = Math::clamp(5.5, 0, 10);
        let c5 = Math::clamp(0, 0, 10);
        let c6 = Math::clamp(10, 0, 10);

        [c1, c2, c3, c4, c5, c6];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 6);
            assert_eq!(vals[0], Object::Integer(5));
            assert_eq!(vals[1], Object::Integer(0));
            assert_eq!(vals[2], Object::Integer(10));
            match &vals[3] {
                Object::Float(f) => assert!((*f - 5.5).abs() < 0.0001),
                other => panic!("expected float, got {:?}", other),
            }
            assert_eq!(vals[4], Object::Integer(0));
            assert_eq!(vals[5], Object::Integer(10));
        }
        other => panic!("expected array from Math::clamp test, got {:?}", other),
    }
}

#[test]
fn test_math_random() {
    let input = r#"
        let r1 = Math::random();
        let r2 = Math::random();

        let inRange1 = r1 >= 0.0;
        let inRange2 = r1 < 1.0;
        let inRange3 = r2 >= 0.0;
        let inRange4 = r2 < 1.0;

        [inRange1, inRange2, inRange3, inRange4];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 4);
            assert_eq!(vals[0], Object::Boolean(true));
            assert_eq!(vals[1], Object::Boolean(true));
            assert_eq!(vals[2], Object::Boolean(true));
            assert_eq!(vals[3], Object::Boolean(true));
        }
        other => panic!("expected array from Math::random test, got {:?}", other),
    }
}

#[test]
fn test_math_random_int() {
    let input = r#"
        let r1 = Math::randomInt(1, 10);
        let r2 = Math::randomInt(0, 100);
        let r3 = Math::randomInt(5, 5);

        let inRange1 = r1 >= 1;
        let inRange2 = r1 <= 10;
        let inRange3 = r2 >= 0;
        let inRange4 = r2 <= 100;
        let exactVal = r3 == 5;

        [inRange1, inRange2, inRange3, inRange4, exactVal];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 5);
            assert_eq!(vals[0], Object::Boolean(true));
            assert_eq!(vals[1], Object::Boolean(true));
            assert_eq!(vals[2], Object::Boolean(true));
            assert_eq!(vals[3], Object::Boolean(true));
            assert_eq!(vals[4], Object::Boolean(true));
        }
        other => panic!("expected array from Math::randomInt test, got {:?}", other),
    }
}

#[test]
fn test_math_extras_error_handling() {
    let input1 = r#"Math::sign("not a number");"#;
    let obj1 = eval_input(input1);
    assert!(obj1.is_error(), "Math::sign with non-number should error");

    let input2 = r#"Math::clamp(5, "a", 10);"#;
    let obj2 = eval_input(input2);
    assert!(obj2.is_error(), "Math::clamp with non-number should error");

    let input3 = r#"Math::randomInt(10, 5);"#;
    let obj3 = eval_input(input3);
    assert!(obj3.is_error(), "Math::randomInt with min > max should error");

    let input4 = r#"Math::PI(1);"#;
    let obj4 = eval_input(input4);
    assert!(obj4.is_error(), "Math::PI with argument should error");
}




