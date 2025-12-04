# Slang

Slang (Simple LANGuage) is a simple interpreted language
built in Rust as a means of experimenting with Rust.

Slang is a C-like language with a syntax that should be very
familiar to JavaScript programmers.

Slang is not meant to be used for serious programming, it is
a toy language built as a means to learn Rust.

## Running Slang

You can run Slang either in REPL mode or by executing a script file.

```
# runs REPL mode
$ ./slang

# runs script file
$ ./slang script.sl
```

## Debug mode

Slang supports debug mode, this basically just vomits the internal
processing out to your console as the program runs.

This can be enabled by using the `debug(true);` function, or can
be switched off via `debug(false);`.

## Features

The following is a non-exhaustive list of features that Slang
supports.

### Defining variables

```
let x = 5;
let y = 10;
// x == 5, y == 10
```

### Arithmetic operations

```
let x = 5;
let y = 10;
let z = x + y;
// z == 15
```

### If statements

```
let x = 5;
let y = 10;
let z = x + y;
if (z == 15) { z } else { -1 }
// outputs 15
```

### Proper precedence

```
let x = 5;
let y = 10;
let z = x + y * 2;
// z == 25
```

### Functions

```
let add = function(x, y) { x + y };
add(5, 10); // outputs 15

// same as
function add(x, y) { 
    x + y 
}

add(5, 10); // outputs 15
```

### While loops

```
let f = fn() {
    let x = 0;
    while (x < 5) {
        if (x == 3) {
            return x;
        }
        let x = x + 1;
    }
    99;
};

f();
```

### String literals

```
"hello" + " " + "world";
// outputs "hello world"
```

### For loops (also arrays)

```
let a = [1, 2, 3, 4, 5];
for (let i = 0; i < len(a); i = i + 1) {
	print(a[i]);
}

// outputs 1, 2, 3, 4, 5

for (let i = 0; i < len(a); i++) {
	print(a[i]);
}

// also outputs 1, 2, 3, 4, 5
```

### file operations

```
let f = file_open("test.txt", "w+");
file_write(f, "Hello, world!");
file_seek(f, 0, "start");
print(file_read(f));
file_close(f);
```

### objects

```
let obj = {
    x: 5,
    y: 10,
    add: fn() {
        return this.x + this.y;
    },
    inner: {
        z: [1, 2, 3],
        sum: function () {
            let sum = 0;
            for (let i = 0; i < len(this.z); i++) {
                sum = sum + this.z[i];
            }
            
            return sum;
        }
    }
};

print(obj.add());
// outputs 15

print(obj.inner.sum());
// outputs 6
```

### Higher order functions

```
let makeAdder = fn(x) {
    function(y) { x + y; }; // this inner fn closes over x
};

let addTwo = makeAdder(2);
print(addTwo(3)); // => 5

let addTen = makeAdder(10);
print(addTen(7)); // => 17
```

### Monads

```
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
```

### Regex

```
let t1 = regexIsMatch("hello123", "[a-z]+[0-9]+"); // true
let t2 = regexIsMatch("hello", "[0-9]+"); // false

let m1 = regexFind("abc123xyz", "\d+"); // Some("123")
let m2 = regexFind("no-digits-here", "\d+"); // None()

let r = regexReplace("foo 123 bar 456", "\d+", "X"); // foo X bar X

let c1 = regexMatch("abc123", "([a-z]+)(\d+)"); // Some(["abc123", "abc", "123"])
let c2 = regexMatch("no-digits", "(\d+)"); // None()
```

### Monadic results for file operations

```
let opened = File::open("test.txt", "w+");
let f = Result::unwrapOr(opened, 0);

let res1 = File::read(123); // Result::Err()
let res2 = File::write(123, "data"); // Result::Err()
let res3 = File::write(f, 42); // Result::Err()
let _ = File::close(f);
let res4 = File::read(f); // Result::Err()

let a = Result::isErr(res1);
let b = Result::isErr(res2);
let c = Result::isErr(res3);
let d = Result::isErr(res4);
```

## Standard library

Slang ships with a small standard library that is preloaded into the global
environment whenever you run a script or the REPL.

### Namespaces

- **Option**
  - Represents optional values: `Some(v)` or `None`.
  - Exposed as a namespace object with the following helpers:
    - `Option::Some(value)` – wrap a value in an `Option`.
    - `Option::None()` – create an empty option.
    - `Option::isSome(opt)` / `Option::isNone(opt)` – boolean checks.
    - `Option::unwrapOr(opt, default)` – returns the inner value or a default.
    - `Option::map(opt, fn)` / `Option::fmap(opt, fn)` – transform the inner value if present.
    - `Option::andThen(opt, fn)` / `Option::bind(opt, fn)` – monadic bind; `fn` should return an `Option`.

- **Result**
  - Represents the outcome of computations that can succeed or fail: `Ok(v)` or `Err(e)`.
  - Exposed as a namespace object with helpers:
    - `Result::Ok(value)` / `Result::Err(errorValue)`.
    - `Result::isOk(res)` / `Result::isErr(res)`.
    - `Result::unwrapOr(res, default)` – returns inner value on `Ok`, default on `Err`.
    - `Result::map(res, fn)` / `Result::fmap(res, fn)` – transform the success value.
    - `Result::andThen(res, fn)` / `Result::bind(res, fn)` – monadic bind; `fn` should return a `Result`.

- **Array**
  - Provides higher-order helpers for working with arrays:
    - `Array::map(arr, fn)` – returns a new array with `fn(element)` applied to each element.
    - `Array::filter(arr, fn)` – returns a new array containing only elements where `fn(element)` is `true`.
    - `Array::reduce(arr, initial, fn)` – folds the array from left to right, calling `fn(acc, element)`.
  - These complement the lower-level builtins like `len`, `first`, `last`, `rest`, and `push`.

- **Regex**
  - Regex helpers are available both as free functions and under the `Regex` namespace:
    - `Regex::isMatch(text, pattern)` – boolean match test.
    - `Regex::find(text, pattern)` – returns `Option::Some(match)` or `Option::None()`.
    - `Regex::replace(text, pattern, replacement)` – returns a new string with replacements.
    - `Regex::match(text, pattern)` – returns `Option::Some(arrayOfGroups)` or `Option::None()`.
  - The free-function aliases (`regexIsMatch`, `regexFind`, `regexReplace`, `regexMatch`) remain available for convenience.

- **File**
  - Low-level file operations exist as free functions (`file_open`, `file_read`, `file_write`, etc.), but the
    preferred interface is the `File` namespace, which wraps results in `Result`:
    - `File::open(path, mode)` – returns `Result::Ok(file)` or `Result::Err(error)`.
    - `File::read(file)` – returns `Result::Ok(string)` or `Result::Err(error)`.
    - `File::write(file, string)` – returns `Result::Ok(unit)` or `Result::Err(error)`.
    - `File::seek(file, offset, origin)` – returns `Result::Ok(unit)` or `Result::Err(error)`.
    - `File::close(file)` – returns `Result::Ok(unit)` or `Result::Err(error)`.

- **Test**
  - The `Test` namespace provides basic assertion helpers designed for writing test scripts:
    - `Test::assert(condition)` – fails if `condition` is false.
    - `Test::assertEq(expected, actual)` – equality assertion.
    - `Test::assertNotEq(expected, actual)` – inequality assertion.

### Top-level builtins

In addition to the namespaced modules above, a handful of helpers are exposed
as top-level builtins:

- **len(x)** – length of a string or array.
- **first(arr)** / **last(arr)** / **rest(arr)** / **push(arr, value)** – basic array helpers.
- **print(...args)** – print values to stdout (used throughout the examples).
- **debug(bool)** – enable or disable Slang's internal debug logging.

Over time, more functionality may move into namespaced modules for better
organization, but the existing free functions will continue to work for
backwards compatibility.
