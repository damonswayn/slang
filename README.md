# Slang

Slang (Simple LANGuage) is a simple interpreted language
built in Rust as a means of experimenting with Rust.

## Features

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