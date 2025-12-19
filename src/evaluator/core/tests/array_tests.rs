use crate::object::Object;
use crate::test_support::eval_input;

#[test]
fn test_array_map() {
    let input = r#"
        let xs = [1, 2, 3];
        let ys = Array::map(xs, fn(x) { x + 1; });
        ys;
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 3);
            assert_eq!(vals[0], Object::Integer(2));
            assert_eq!(vals[1], Object::Integer(3));
            assert_eq!(vals[2], Object::Integer(4));
        }
        other => panic!("expected array from Array::map, got {:?}", other),
    }
}

#[test]
fn test_array_filter() {
    let input = r#"
        let xs = [1, 2, 3, 4];
        let ys = Array::filter(xs, fn(x) { x % 2 == 0; });
        ys;
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 2);
            assert_eq!(vals[0], Object::Integer(2));
            assert_eq!(vals[1], Object::Integer(4));
        }
        other => panic!("expected array from Array::filter, got {:?}", other),
    }
}

#[test]
fn test_array_reduce() {
    let input = r#"
        let xs = [1, 2, 3, 4];
        Array::reduce(xs, 0, fn(acc, x) { acc + x; });
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Integer(i) => assert_eq!(i, 10),
        other => panic!("expected integer from Array::reduce, got {:?}", other),
    }
}

#[test]
fn test_array_find_some_every_flat_map() {
    let input = r#"
        let xs = [1, 2, 3, 4, 5];

        let found = Array::find(xs, fn(x) { x % 2 == 0; });
        let someEven = Array::some(xs, fn(x) { x % 2 == 0; });
        let someGtFive = Array::some(xs, fn(x) { x > 5; });
        let allPositive = Array::every(xs, fn(x) { x > 0; });
        let allEven = Array::every(xs, fn(x) { x % 2 == 0; });

        let pairs = Array::flatMap(xs, fn(x) { [x, x * 10]; });

        [found, someEven, someGtFive, allPositive, allEven, pairs];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 6);

            match &vals[0] {
                Object::OptionSome(inner) => assert_eq!(**inner, Object::Integer(2)),
                other => panic!("expected Option::Some(2) from Array::find, got {:?}", other),
            }

            assert_eq!(vals[1], Object::Boolean(true));
            assert_eq!(vals[2], Object::Boolean(false));
            assert_eq!(vals[3], Object::Boolean(true));
            assert_eq!(vals[4], Object::Boolean(false));

            match &vals[5] {
                Object::Array(pairs) => {
                    assert_eq!(pairs.len(), 10);
                    assert_eq!(pairs[0], Object::Integer(1));
                    assert_eq!(pairs[1], Object::Integer(10));
                    assert_eq!(pairs[8], Object::Integer(5));
                    assert_eq!(pairs[9], Object::Integer(50));
                }
                other => panic!("expected array from Array::flatMap, got {:?}", other),
            }
        }
        other => panic!("expected array from Array::find/some/every/flatMap test, got {:?}", other),
    }
}

#[test]
fn test_array_sort() {
    let input = r#"
        let nums = [3, 1, 4, 1, 5, 9, 2, 6];
        let sorted = Array::sort(nums);

        let strs = ["banana", "apple", "cherry"];
        let sortedStrs = Array::sort(strs);

        [sorted[0], sorted[1], sorted[2], sortedStrs[0], sortedStrs[1]];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 5);
            assert_eq!(vals[0], Object::Integer(1));
            assert_eq!(vals[1], Object::Integer(1));
            assert_eq!(vals[2], Object::Integer(2));
            assert_eq!(vals[3], Object::String("apple".to_string()));
            assert_eq!(vals[4], Object::String("banana".to_string()));
        }
        other => panic!("expected array from Array::sort test, got {:?}", other),
    }
}

#[test]
fn test_array_sort_by() {
    let input = r#"
        let nums = [3, 1, 4, 1, 5];

        let desc = Array::sortBy(nums, fn(a, b) { b - a; });

        let byDiff = Array::sortBy(nums, fn(a, b) {
            let diffA = a - 3;
            let diffB = b - 3;
            if (diffA < 0) { diffA = 0 - diffA; }
            if (diffB < 0) { diffB = 0 - diffB; }
            diffA - diffB;
        });

        [desc[0], desc[1], desc[2], byDiff[0], byDiff[1]];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 5);
            assert_eq!(vals[0], Object::Integer(5));
            assert_eq!(vals[1], Object::Integer(4));
            assert_eq!(vals[2], Object::Integer(3));
            assert_eq!(vals[3], Object::Integer(3));
            assert_eq!(vals[4], Object::Integer(4));
        }
        other => panic!("expected array from Array::sortBy test, got {:?}", other),
    }
}

#[test]
fn test_array_reverse() {
    let input = r#"
        let arr = [1, 2, 3, 4, 5];
        let rev = Array::reverse(arr);

        let empty = Array::reverse([]);

        [rev[0], rev[1], rev[4], len(empty)];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 4);
            assert_eq!(vals[0], Object::Integer(5));
            assert_eq!(vals[1], Object::Integer(4));
            assert_eq!(vals[2], Object::Integer(1));
            assert_eq!(vals[3], Object::Integer(0));
        }
        other => panic!("expected array from Array::reverse test, got {:?}", other),
    }
}

#[test]
fn test_array_index_of_and_includes() {
    let input = r#"
        let arr = [10, 20, 30, 20, 40];

        let idx1 = Array::indexOf(arr, 20);
        let idx2 = Array::indexOf(arr, 99);

        let val1 = Option::unwrapOr(idx1, -1);
        let val2 = Option::unwrapOr(idx2, -1);

        let inc1 = Array::includes(arr, 30);
        let inc2 = Array::includes(arr, 99);

        [val1, val2, inc1, inc2, Option::isNone(idx2)];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 5);
            assert_eq!(vals[0], Object::Integer(1));
            assert_eq!(vals[1], Object::Integer(-1));
            assert_eq!(vals[2], Object::Boolean(true));
            assert_eq!(vals[3], Object::Boolean(false));
            assert_eq!(vals[4], Object::Boolean(true));
        }
        other => panic!("expected array from Array::indexOf/includes test, got {:?}", other),
    }
}

#[test]
fn test_array_concat() {
    let input = r#"
        let a = [1, 2, 3];
        let b = [4, 5, 6];
        let c = Array::concat(a, b);

        let d = Array::concat([], [1, 2]);
        let e = Array::concat([1, 2], []);

        [len(c), c[0], c[5], len(d), len(e)];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 5);
            assert_eq!(vals[0], Object::Integer(6));
            assert_eq!(vals[1], Object::Integer(1));
            assert_eq!(vals[2], Object::Integer(6));
            assert_eq!(vals[3], Object::Integer(2));
            assert_eq!(vals[4], Object::Integer(2));
        }
        other => panic!("expected array from Array::concat test, got {:?}", other),
    }
}

#[test]
fn test_array_slice() {
    let input = r#"
        let arr = [0, 1, 2, 3, 4, 5];

        let s1 = Array::slice(arr, 1, 4);
        let s2 = Array::slice(arr, -3, 6);
        let s3 = Array::slice(arr, 0, -2);
        let s4 = Array::slice(arr, 3, 3);
        let s5 = Array::slice(arr, 5, 2);

        [len(s1), s1[0], s1[2], len(s2), s2[0], len(s3), len(s4), len(s5)];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 8);
            assert_eq!(vals[0], Object::Integer(3));
            assert_eq!(vals[1], Object::Integer(1));
            assert_eq!(vals[2], Object::Integer(3));
            assert_eq!(vals[3], Object::Integer(3));
            assert_eq!(vals[4], Object::Integer(3));
            assert_eq!(vals[5], Object::Integer(4));
            assert_eq!(vals[6], Object::Integer(0));
            assert_eq!(vals[7], Object::Integer(0));
        }
        other => panic!("expected array from Array::slice test, got {:?}", other),
    }
}

#[test]
fn test_array_take_and_drop() {
    let input = r#"
        let arr = [1, 2, 3, 4, 5];

        let t1 = Array::take(arr, 3);
        let t2 = Array::take(arr, 10);
        let t3 = Array::take(arr, 0);

        let d1 = Array::drop(arr, 2);
        let d2 = Array::drop(arr, 10);
        let d3 = Array::drop(arr, 0);

        [len(t1), t1[0], t1[2], len(t2), len(t3), len(d1), d1[0], len(d2), len(d3)];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 9);
            assert_eq!(vals[0], Object::Integer(3));
            assert_eq!(vals[1], Object::Integer(1));
            assert_eq!(vals[2], Object::Integer(3));
            assert_eq!(vals[3], Object::Integer(5));
            assert_eq!(vals[4], Object::Integer(0));
            assert_eq!(vals[5], Object::Integer(3));
            assert_eq!(vals[6], Object::Integer(3));
            assert_eq!(vals[7], Object::Integer(0));
            assert_eq!(vals[8], Object::Integer(5));
        }
        other => panic!("expected array from Array::take/drop test, got {:?}", other),
    }
}

#[test]
fn test_array_range() {
    let input = r#"
        let r1 = Array::range(0, 5);
        let r2 = Array::range(1, 10, 2);
        let r3 = Array::range(5, 0, -1);
        let r4 = Array::range(0, 0);

        [len(r1), r1[0], r1[4], len(r2), r2[0], r2[4], len(r3), r3[0], r3[4], len(r4)];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 10);
            assert_eq!(vals[0], Object::Integer(5));
            assert_eq!(vals[1], Object::Integer(0));
            assert_eq!(vals[2], Object::Integer(4));
            assert_eq!(vals[3], Object::Integer(5));
            assert_eq!(vals[4], Object::Integer(1));
            assert_eq!(vals[5], Object::Integer(9));
            assert_eq!(vals[6], Object::Integer(5));
            assert_eq!(vals[7], Object::Integer(5));
            assert_eq!(vals[8], Object::Integer(1));
            assert_eq!(vals[9], Object::Integer(0));
        }
        other => panic!("expected array from Array::range test, got {:?}", other),
    }
}

#[test]
fn test_array_unique() {
    let input = r#"
        let arr = [1, 2, 2, 3, 1, 4, 3, 5];
        let uniq = Array::unique(arr);

        let strs = ["a", "b", "a", "c", "b"];
        let uniqStrs = Array::unique(strs);

        [len(uniq), uniq[0], uniq[1], uniq[4], len(uniqStrs)];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 5);
            assert_eq!(vals[0], Object::Integer(5));
            assert_eq!(vals[1], Object::Integer(1));
            assert_eq!(vals[2], Object::Integer(2));
            assert_eq!(vals[3], Object::Integer(5));
            assert_eq!(vals[4], Object::Integer(3));
        }
        other => panic!("expected array from Array::unique test, got {:?}", other),
    }
}

#[test]
fn test_array_expansions_error_handling() {
    let input1 = r#"Array::sort("not an array");"#;
    let obj1 = eval_input(input1);
    assert!(obj1.is_error(), "Array::sort on non-array should error");

    let input2 = r#"Array::slice([1,2,3], "a", 2);"#;
    let obj2 = eval_input(input2);
    assert!(obj2.is_error(), "Array::slice with non-integer should error");

    let input3 = r#"Array::take([1,2,3], -1);"#;
    let obj3 = eval_input(input3);
    assert!(obj3.is_error(), "Array::take with negative should error");

    let input4 = r#"Array::range(0, 10, 0);"#;
    let obj4 = eval_input(input4);
    assert!(obj4.is_error(), "Array::range with zero step should error");
}

#[test]
fn test_array_flatten() {
    let input = r#"
        let nested = [[1, 2], [3, 4], [5]];
        let flat = Array::flatten(nested);

        let mixed = [1, [2, 3], 4, [5, 6]];
        let flatMixed = Array::flatten(mixed);

        let deep = [[[1, 2]], [[3, 4]]];
        let flatDeep = Array::flatten(deep);

        [len(flat), flat[0], flat[4], len(flatMixed), len(flatDeep)];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 5);
            assert_eq!(vals[0], Object::Integer(5));
            assert_eq!(vals[1], Object::Integer(1));
            assert_eq!(vals[2], Object::Integer(5));
            assert_eq!(vals[3], Object::Integer(6));
            assert_eq!(vals[4], Object::Integer(2));
        }
        other => panic!("expected array from Array::flatten test, got {:?}", other),
    }
}

#[test]
fn test_array_zip_and_unzip() {
    let input = r#"
        let a = [1, 2, 3];
        let b = ["a", "b", "c"];

        let zipped = Array::zip(a, b);
        let unzipped = Array::unzip(zipped);

        let z0 = zipped[0];
        let z1 = zipped[1];

        let u0 = unzipped[0];
        let u1 = unzipped[1];

        [len(zipped), z0[0], z0[1], z1[0], len(u0), u0[0], u1[1]];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 7);
            assert_eq!(vals[0], Object::Integer(3));
            assert_eq!(vals[1], Object::Integer(1));
            assert_eq!(vals[2], Object::String("a".to_string()));
            assert_eq!(vals[3], Object::Integer(2));
            assert_eq!(vals[4], Object::Integer(3));
            assert_eq!(vals[5], Object::Integer(1));
            assert_eq!(vals[6], Object::String("b".to_string()));
        }
        other => panic!("expected array from Array::zip/unzip test, got {:?}", other),
    }
}

#[test]
fn test_array_group_by() {
    let input = r#"
        let items = [
            { name: "apple", type: "fruit" },
            { name: "carrot", type: "vegetable" },
            { name: "banana", type: "fruit" },
            { name: "broccoli", type: "vegetable" }
        ];

        let grouped = Array::groupBy(items, fn(item) { item.type; });

        let fruits = grouped.fruit;
        let veggies = grouped.vegetable;

        [len(fruits), len(veggies), fruits[0].name, veggies[1].name];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 4);
            assert_eq!(vals[0], Object::Integer(2));
            assert_eq!(vals[1], Object::Integer(2));
            assert_eq!(vals[2], Object::String("apple".to_string()));
            assert_eq!(vals[3], Object::String("broccoli".to_string()));
        }
        other => panic!("expected array from Array::groupBy test, got {:?}", other),
    }
}

#[test]
fn test_array_partition() {
    let input = r#"
        let nums = [1, 2, 3, 4, 5, 6, 7, 8];
        let parts = Array::partition(nums, fn(x) { x % 2 == 0; });

        let evens = parts[0];
        let odds = parts[1];

        [len(evens), len(odds), evens[0], evens[1], odds[0], odds[1]];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 6);
            assert_eq!(vals[0], Object::Integer(4));
            assert_eq!(vals[1], Object::Integer(4));
            assert_eq!(vals[2], Object::Integer(2));
            assert_eq!(vals[3], Object::Integer(4));
            assert_eq!(vals[4], Object::Integer(1));
            assert_eq!(vals[5], Object::Integer(3));
        }
        other => panic!("expected array from Array::partition test, got {:?}", other),
    }
}

#[test]
fn test_array_fill() {
    let input = r#"
        let f1 = Array::fill(0, 5);
        let f2 = Array::fill("x", 3);
        let f3 = Array::fill(true, 0);

        [len(f1), f1[0], f1[4], len(f2), f2[1], len(f3)];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 6);
            assert_eq!(vals[0], Object::Integer(5));
            assert_eq!(vals[1], Object::Integer(0));
            assert_eq!(vals[2], Object::Integer(0));
            assert_eq!(vals[3], Object::Integer(3));
            assert_eq!(vals[4], Object::String("x".to_string()));
            assert_eq!(vals[5], Object::Integer(0));
        }
        other => panic!("expected array from Array::fill test, got {:?}", other),
    }
}

#[test]
fn test_array_is_empty_and_len() {
    let input = r#"
        let empty = [];
        let nonEmpty = [1, 2, 3];

        let ie1 = Array::isEmpty(empty);
        let ie2 = Array::isEmpty(nonEmpty);

        let l1 = Array::len(empty);
        let l2 = Array::len(nonEmpty);

        [ie1, ie2, l1, l2];
    "#;

    let obj = eval_input(input);
    match obj {
        Object::Array(vals) => {
            assert_eq!(vals.len(), 4);
            assert_eq!(vals[0], Object::Boolean(true));
            assert_eq!(vals[1], Object::Boolean(false));
            assert_eq!(vals[2], Object::Integer(0));
            assert_eq!(vals[3], Object::Integer(3));
        }
        other => panic!("expected array from Array::isEmpty/len test, got {:?}", other),
    }
}

#[test]
fn test_array_for_each() {
    let input = r#"
        let arr = [1, 2, 3, 4, 5];

        let result = Array::forEach(arr, fn(x) { x * 2; });

        result;
    "#;

    let obj = eval_input(input);
    assert_eq!(obj, Object::Null);
}

#[test]
fn test_array_extras_error_handling() {
    let input1 = r#"Array::flatten("not array");"#;
    let obj1 = eval_input(input1);
    assert!(obj1.is_error(), "Array::flatten on non-array should error");

    let input2 = r#"Array::unzip([[1, 2], "not pair"]);"#;
    let obj2 = eval_input(input2);
    assert!(obj2.is_error(), "Array::unzip with invalid pairs should error");

    let input3 = r#"Array::fill("x", -1);"#;
    let obj3 = eval_input(input3);
    assert!(obj3.is_error(), "Array::fill with negative count should error");

    let input4 = r#"Array::partition([1,2,3], fn(x) { "not bool"; });"#;
    let obj4 = eval_input(input4);
    assert!(obj4.is_error(), "Array::partition with non-bool predicate should error");
}

