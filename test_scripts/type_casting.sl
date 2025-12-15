print(int("42"));
print(float("3.14"));
print(str(123));
print(bool(0));
print(bool(1));
print(bool("true"));
print(bool("FALSE"));

let okFloat = Type::float("2.5");
Result::map(okFloat, fn (v) {
    print(v);
    v;
});

let badInt = Type::int("abc");
if (Result::isErr(badInt)) {
    print("type error");
}

let someStr = Type::str(Option::Some(5));
let extracted = Result::unwrapOr(someStr, "nope");
print(extracted);

