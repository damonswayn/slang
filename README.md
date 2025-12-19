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

### Type casting

```
int("42");          // 42 (Integer)
float("3.14");      // 3.14 (Float)
str(123);           // "123" (String)
bool(0);            // false
bool("true");       // true

// Result-based variants
let r1 = Type::int("99");     // Result::Ok(99)
let r2 = Type::float("oops"); // Result::Err("float(): could not parse float from \"oops\"")
if (Result::isErr(r2)) { print("bad cast"); }
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

### Namespaces and imports

```
namespace SomeNamespace {
    function add(a, b) {
        a + b;
    }
}

SomeNamespace::add(5, 7); // 12
```

Code inside a namespace is exported as a single object. Top-level code outside
any namespace remains private to the file. You can import another file with a
string path (absolute or relative to the current working directory):

```
import "lib/math.sl";
Math::sqrt(9);
```

If multiple files declare the same namespace, their members are merged; later
imports override earlier definitions of the same member name.

### Built-in pub/sub tags and chaining

You can tag functions and publish values to all subscribers of a tag. Tags are
declared before a function with `(:TagOne, :TagTwo)`; publishing uses `-> :Tag`.
Publishes can be chained; each stage receives the return values from the previous
stage (with `null` values dropped).

Argument shaping per subscriber:
- 0 params → no arguments passed
- 1 param  → a single array of all non-null values from the previous stage
- >1 params → values are packed positionally; if more values than params, the last
  param receives the remaining values as an array; if fewer, missing params get `null`.

Example:
```
(:Square)
function square(values) {           // one-arg subscriber gets an array
    let n = values[0];
    let s = n * n;
    print(s);
    return s;
}

(:Print)
function output(values) {           // also one-arg -> array
    print(values[0]);
}

let a = 5;
let b = 7;

a + b -> :Square -> :Print          // prints 144 twice in the sample test
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

Slang ships with a comprehensive standard library that is preloaded into the global
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

- **Type**
  - Safe, Result-wrapped casts:
    - `Type::int(value)` → `Result::Ok(Integer)` or `Result::Err("...")`
    - `Type::float(value)` → `Result::Ok(Float)` or `Result::Err("...")`
    - `Type::str(value)` → `Result::Ok(String)` (never errors)
    - `Type::bool(value)` → `Result::Ok(Boolean)` or `Result::Err("...")`
  - Accepted inputs:
    - `int`: integers, finite floats (truncates), booleans (1/0), strings parsable as i64.
    - `float`: floats, integers, booleans (1/0), strings parsable as f64.
    - `str`: any value via `to_string`.
    - `bool`: booleans, integers/floats (non-zero => true), strings `"true"/"false"/"1"/"0"`, `null` → false.
  - Free-function aliases `int(value)`, `float(value)`, `str(value)`, `bool(value)` perform the same conversions but raise an error instead of returning `Result`.
  - Type introspection:
    - `Type::of(value)` – returns a string describing the type: `"integer"`, `"float"`, `"boolean"`, `"string"`, `"array"`, `"object"`, `"function"`, `"option"`, `"result"`, `"null"`.
    - `Type::isInt(value)` / `Type::isFloat(value)` / `Type::isNumber(value)` – numeric type checks.
    - `Type::isBool(value)` / `Type::isString(value)` – primitive type checks.
    - `Type::isArray(value)` / `Type::isObject(value)` – compound type checks.
    - `Type::isCallable(value)` – returns `true` for functions and builtins.
    - `Type::isIterable(value)` – returns `true` for arrays and strings.
    - `Type::isNull(value)` – returns `true` if value is `null`.
    - `Type::isOption(value)` / `Type::isResult(value)` – monad type checks.

- **Array**
  - Higher-order functions:
    - `Array::map(arr, fn)` – returns a new array with `fn(element)` applied to each element.
    - `Array::filter(arr, fn)` – returns a new array containing only elements where `fn(element)` is `true`.
    - `Array::reduce(arr, initial, fn)` – folds the array from left to right, calling `fn(acc, element)`.
    - `Array::find(arr, fn)` – returns `Option::Some(element)` for the first element where `fn(element)` is `true`, or `Option::None()` if none match.
    - `Array::some(arr, fn)` – returns `true` if any element matches the predicate.
    - `Array::every(arr, fn)` – returns `true` if all elements match the predicate.
    - `Array::flatMap(arr, fn)` – maps each element to an array and concatenates the results.
    - `Array::forEach(arr, fn)` – executes `fn(element)` for each element (side effects only, returns `null`).
    - `Array::groupBy(arr, fn)` – groups elements by the string key returned by `fn(element)`.
    - `Array::partition(arr, fn)` – splits into `[matching, non-matching]` based on predicate.
  - Sorting:
    - `Array::sort(arr)` – returns a new sorted array (ascending order, homogeneous types only).
    - `Array::sortBy(arr, compareFn)` – custom sort using `compareFn(a, b)` returning negative/zero/positive.
  - Searching:
    - `Array::indexOf(arr, value)` – returns `Option::Some(index)` or `Option::None()`.
    - `Array::includes(arr, value)` – returns `true` if the value exists in the array.
  - Slicing and combining:
    - `Array::slice(arr, start, end)` – extracts a portion (supports negative indices).
    - `Array::take(arr, n)` – returns the first `n` elements.
    - `Array::drop(arr, n)` – returns all elements after the first `n`.
    - `Array::concat(arr1, arr2)` – concatenates two arrays.
    - `Array::reverse(arr)` – returns a new reversed array.
    - `Array::flatten(arr)` – flattens one level of nested arrays.
    - `Array::unique(arr)` – returns a new array with duplicates removed.
  - Zipping:
    - `Array::zip(arr1, arr2)` – combines into array of pairs `[[a1, b1], [a2, b2], ...]`.
    - `Array::unzip(arr)` – splits array of pairs into `[[a1, a2, ...], [b1, b2, ...]]`.
  - Creation:
    - `Array::range(start, end[, step])` – creates an array of integers from `start` to `end` (exclusive).
    - `Array::fill(value, count)` – creates an array with `count` copies of `value`.
  - Utilities:
    - `Array::isEmpty(arr)` – returns `true` if the array has no elements.
    - `Array::len(arr)` – returns the number of elements.
  - These complement the lower-level builtins like `len`, `first`, `last`, `rest`, and `push`.

- **Obj**
  - Object (hash map) manipulation utilities:
    - `Obj::keys(obj)` – returns an array of all keys.
    - `Obj::values(obj)` – returns an array of all values.
    - `Obj::entries(obj)` – returns an array of `[key, value]` pairs.
    - `Obj::fromEntries(arr)` – creates an object from an array of `[key, value]` pairs.
    - `Obj::has(obj, key)` – returns `true` if the key exists.
    - `Obj::get(obj, key)` – returns `Option::Some(value)` or `Option::None()`.
    - `Obj::set(obj, key, value)` – returns a new object with the key set (immutable).
    - `Obj::delete(obj, key)` – returns a new object with the key removed (immutable).
    - `Obj::merge(obj1, obj2)` – returns a new object combining both (obj2 values override obj1).
    - `Obj::isEmpty(obj)` – returns `true` if the object has no keys.
    - `Obj::len(obj)` – returns the number of key-value pairs.

- **String**
  - Basic utilities:
    - `String::trim(s)` – trims leading and trailing whitespace.
    - `String::toUpper(s)` / `String::toLower(s)` – case conversion.
    - `String::split(s, sep)` – splits into an array of strings (`sep == ""` splits into characters).
    - `String::join(arr, sep)` – joins an array of strings with a separator.
    - `String::len(s)` – returns the number of characters (not bytes).
    - `String::isEmpty(s)` – returns `true` if the string has no characters.
  - Searching:
    - `String::contains(s, substr)` – returns `true` if `substr` is found.
    - `String::startsWith(s, prefix)` / `String::endsWith(s, suffix)` – prefix/suffix checks.
    - `String::indexOf(s, substr)` – returns `Option::Some(index)` or `Option::None()`.
    - `String::lastIndexOf(s, substr)` – returns index of last occurrence.
  - Slicing and manipulation:
    - `String::slice(s, start, end)` – extracts a substring (supports negative indices).
    - `String::replace(s, from, to)` – replaces the first occurrence.
    - `String::replaceAll(s, from, to)` – replaces all occurrences.
    - `String::repeat(s, count)` – repeats the string `count` times.
    - `String::reverse(s)` – reverses the string.
    - `String::padLeft(s, length, char)` / `String::padRight(s, length, char)` – pads to target length.
  - Character utilities:
    - `String::chars(s)` – returns an array of single-character strings.
    - `String::charCodeAt(s, index)` – returns the Unicode code point at the index.
    - `String::charCodes(s)` – returns an array of all character codes.
    - `String::fromCharCode(code)` – creates a single-character string from a code point.
    - `String::fromCharCodes(arr)` – creates a string from an array of code points.

- **Math**
  - Basic operations:
    - `Math::abs(x)` – absolute value.
    - `Math::floor(x)`, `Math::ceil(x)`, `Math::round(x)` – integer rounding.
    - `Math::min(a, b)`, `Math::max(a, b)` – minimum/maximum of two numbers.
    - `Math::pow(base, exp)` – exponentiation.
    - `Math::sqrt(x)` – square root.
    - `Math::sign(x)` – returns -1, 0, or 1.
    - `Math::clamp(x, min, max)` – constrains a value to a range.
  - Trigonometry:
    - `Math::sin(x)`, `Math::cos(x)`, `Math::tan(x)` – basic trig (radians).
    - `Math::asin(x)`, `Math::acos(x)`, `Math::atan(x)` – inverse trig.
    - `Math::atan2(y, x)` – two-argument arctangent.
    - `Math::sinh(x)`, `Math::cosh(x)`, `Math::tanh(x)` – hyperbolic functions.
  - Logarithms and exponentials:
    - `Math::log(x)` – natural logarithm (base e).
    - `Math::log10(x)` – base-10 logarithm.
    - `Math::log2(x)` – base-2 logarithm.
    - `Math::exp(x)` – e raised to the power x.
  - Constants:
    - `Math::PI()` – π (3.14159...).
    - `Math::E()` – Euler's number e (2.71828...).
    - `Math::TAU()` – τ = 2π (6.28318...).
  - Random numbers:
    - `Math::random()` – returns a random float in [0, 1).
    - `Math::randomInt(min, max)` – returns a random integer in [min, max].

- **Time**
  - Current time:
    - `Time::now()` – returns the current Unix timestamp in milliseconds.
    - `Time::nowSecs()` – returns the current Unix timestamp in seconds.
  - Date/time components (all take a timestamp in milliseconds):
    - `Time::year(ts)`, `Time::month(ts)`, `Time::day(ts)` – date components (UTC).
    - `Time::hour(ts)`, `Time::minute(ts)`, `Time::second(ts)` – time components (UTC).
    - `Time::dayOfWeek(ts)` – returns 0-6 (Sunday = 0).
  - Formatting and conversion:
    - `Time::format(ts, formatStr)` – formats using strftime syntax (e.g., `"%Y-%m-%d %H:%M:%S"`).
    - `Time::toObject(ts)` – returns an object with `year`, `month`, `day`, `hour`, `minute`, `second`, `dayOfWeek`.
  - Utility:
    - `Time::sleep(ms)` – pauses execution for the specified milliseconds.

- **Sys**
  - Environment variables:
    - `Sys::env()` – returns an object with all environment variables.
    - `Sys::env(name)` – returns `Option::Some(value)` or `Option::None()` for a specific variable.
    - `Sys::setEnv(name, value)` – sets an environment variable.
  - Process information:
    - `Sys::args()` – returns an array of command-line arguments.
    - `Sys::cwd()` – returns the current working directory.
    - `Sys::setCwd(path)` – changes the current working directory.
    - `Sys::platform()` – returns the OS name (e.g., `"macos"`, `"linux"`, `"windows"`).
    - `Sys::arch()` – returns the CPU architecture (e.g., `"x86_64"`, `"aarch64"`).
  - Process control:
    - `Sys::exit(code)` – exits the process with the given status code.
    - `Sys::exec(command)` – executes a shell command, returns `Result::Ok({ code, stdout, stderr })` or `Result::Err(error)`.

- **HTTP**
  - HTTP client functions (all return `Result::Ok(response)` or `Result::Err(error)`):
    - `HTTP::get(url[, options])` – performs a GET request.
    - `HTTP::post(url, body[, options])` – performs a POST request.
    - `HTTP::put(url, body[, options])` – performs a PUT request.
    - `HTTP::delete(url[, options])` – performs a DELETE request.
    - `HTTP::patch(url, body[, options])` – performs a PATCH request.
    - `HTTP::head(url[, options])` – performs a HEAD request.
  - Options object can include:
    - `headers` – an object of HTTP headers.
    - `timeout` – request timeout in milliseconds.
  - Response object includes:
    - `status` – HTTP status code.
    - `body` – response body as a string.
    - `headers` – response headers as an object.

- **Fn**
  - Functional programming utilities:
    - `Fn::identity(x)` – returns its argument unchanged.
    - `Fn::constant(value)` – returns a function that always returns `value`.
    - `Fn::compose(f, g)` – returns `fn(x) { f(g(x)) }` (right-to-left composition).
    - `Fn::pipe(f, g)` – returns `fn(x) { g(f(x)) }` (left-to-right composition).
    - `Fn::apply(fn, argsArray)` – calls `fn` with arguments from an array.
    - `Fn::call(fn, ...args)` – calls `fn` with the provided arguments.
    - `Fn::negate(predicateFn)` – returns a function that negates the predicate result.
    - `Fn::flip(fn)` – returns a function with the first two arguments swapped.
    - `Fn::partial(fn, ...boundArgs)` – returns a partially applied function.
    - `Fn::isCallable(value)` – returns `true` if the value can be called as a function.

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

- **Json**
  - JSON interop helpers:
    - `Json::parse(s)` – parses a JSON string into Slang values, returning `Result::Ok(value)` or `Result::Err(errorString)`.
    - `Json::stringify(value)` – converts a Slang value back into a JSON string, returning `Result::Ok(string)` or `Result::Err(errorString)`.

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

### Idiomatic stdlib usage examples

```slang
// Array + Option + Result
let xs = [1, 2, 3, 4, 5];
let firstEven = Array::find(xs, fn(x) { x % 2 == 0; }); // Option::Some(2)

// String helpers
let raw = "  hello world  ";
let cleaned = String::toUpper(String::trim(raw)); // "HELLO WORLD"

// Math + Json
let v = { x: 3, y: 4, len: Math::sqrt(3 * 3 + 4 * 4) };
let json = Json::stringify(v);
let jsonStr = Result::unwrapOr(json, "ERR");
print(jsonStr); // {"x":3,"y":4,"len":5}

// Object manipulation
let user = { name: "Alice", age: 30 };
let updated = Obj::set(user, "email", "alice@example.com");
let keys = Obj::keys(updated); // ["name", "age", "email"]

// Functional programming
let double = fn(x) { x * 2 };
let addOne = fn(x) { x + 1 };
let doubleThenAdd = Fn::compose(addOne, double);
print(Fn::call(doubleThenAdd, 5)); // 11

// Time and dates
let now = Time::now();
let formatted = Time::format(now, "%Y-%m-%d %H:%M:%S");
print(formatted); // e.g., "2024-12-19 15:30:45"

// System interaction
let cwd = Sys::cwd();
let result = Sys::exec("echo hello");
let output = Result::unwrapOr(result, { stdout: "" });
print(String::trim(output.stdout)); // "hello"

// HTTP requests (requires network)
let response = HTTP::get("https://api.example.com/data");
if (Result::isOk(response)) {
    let data = Result::unwrapOr(response, {});
    print(data.body);
}

// Type checking
let value = 42;
if (Type::isNumber(value)) {
    print("It's a number!");
}
print(Type::of(value)); // "integer"
```
