function isNumberGreaterThanZero(num) {
    if (num > 0) {
        Result::Ok(num);
    } else {
        Result::Err("Number less than zero");
    }
}

let resultOne = isNumberGreaterThanZero(5);
Result::andThen(resultOne, fn (num) {
    print(num);
});

let resultTwo = isNumberGreaterThanZero(-12);
Result::andThen(resultTwo, fn (num) {
    print("This shouldn't happen");
});

if (Result::isErr(resultTwo)) {
    print("failure");
}

function findValueInList(val) {
    let list = [1,2,3,4,5];
    for (let i = 0; i < len(list); i++) {
        if (list[i] == val) {
            return Option::Some(i);
        }
    }

    Option::None();
}

let optA = findValueInList(3);
Option::andThen(optA, fn (idx) {
    print("Found value in list at index");
    print(idx);
    Option::Some(idx);
});

let optB = findValueInList(-2);
Option::andThen(optB, fn (val) {
    print("Should not happen");
});

if (Option::isNone(optB)) {
    print("Value not in list");
}