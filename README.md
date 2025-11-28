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