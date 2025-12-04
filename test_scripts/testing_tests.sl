let add = fn(a, b) { a + b; };

test "adds two numbers" {
    Test::assertEq(8, add(5, 3));
}

test "simple boolean assertion" {
    let ok = 2 < 3;
    Test::assert(ok, "2 should be less than 3");
}

test "testing not equals" {
    let sum = add(5, 2);
    Test::assertNotEq(8, sum);
}

test "this test should fail" {
    let failure = 2 == 5;
    Test::assert(failure, "This is expected to fail.");
}