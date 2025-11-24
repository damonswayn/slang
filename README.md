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
add(5, 10);
// outputs 15
```