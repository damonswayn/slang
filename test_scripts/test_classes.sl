// Test: Class definitions and instantiation

// Basic class with methods
class Calculator {
    function add(a, b) { a + b; }
    function subtract(a, b) { a - b; }
    function multiply(a, b) { a * b; }
    function divide(a, b) { a / b; }
}

let calc = new Calculator();
print("Calculator tests:");
print("  2 + 3 =");
print(calc.add(2, 3));
print("  10 - 4 =");
print(calc.subtract(10, 4));
print("  5 * 6 =");
print(calc.multiply(5, 6));
print("  20 / 4 =");
print(calc.divide(20, 4));

// Class with constructor
class Point {
    function construct(x, y) {
        this.x = x;
        this.y = y;
    }

    function getX() { this.x; }
    function getY() { this.y; }

    function distance(other) {
        let dx = this.x - other.x;
        let dy = this.y - other.y;
        Math::sqrt(dx * dx + dy * dy);
    }
}

print("\nPoint tests:");
let p1 = new Point(0, 0);
let p2 = new Point(3, 4);
print("  p1.x =");
print(p1.getX());
print("  p1.y =");
print(p1.getY());
print("  p2.x =");
print(p2.getX());
print("  p2.y =");
print(p2.getY());
print("  distance(p1, p2) =");
print(p1.distance(p2));

// Class with state mutation (functional style - methods return this)
class Counter {
    function construct(start) {
        this.value = start;
    }

    function increment() {
        this.value = this.value + 1;
        this;
    }

    function decrement() {
        this.value = this.value - 1;
        this;
    }

    function getValue() {
        this.value;
    }

    function reset() {
        this.value = 0;
        this;
    }
}

print("\nCounter tests:");
let counter = new Counter(10);
print("  Initial value:");
print(counter.getValue());
counter = counter.increment();
counter = counter.increment();
counter = counter.increment();
print("  After 3 increments:");
print(counter.getValue());
counter = counter.decrement();
print("  After 1 decrement:");
print(counter.getValue());
counter = counter.reset();
print("  After reset:");
print(counter.getValue());

// Multiple independent instances
print("\nMultiple instances test:");
let box1 = new Counter(100);
let box2 = new Counter(200);
box1 = box1.increment();
box2 = box2.decrement();
print("  box1:");
print(box1.getValue());
print("  box2:");
print(box2.getValue());

// Method chaining
class StringBuilder {
    function construct() {
        this.value = "";
    }

    function append(s) {
        this.value = this.value + s;
        this;
    }

    function build() {
        this.value;
    }
}

print("\nStringBuilder test (method chaining):");
let sb = new StringBuilder();
let result = sb.append("Hello").append(" ").append("World").append("!").build();
print("  Result:");
print(result);

// Recursive method
class MathUtils {
    function factorial(n) {
        if (n <= 1) {
            1;
        } else {
            n * this.factorial(n - 1);
        }
    }

    function fibonacci(n) {
        if (n <= 1) {
            n;
        } else {
            this.fibonacci(n - 1) + this.fibonacci(n - 2);
        }
    }
}

print("\nMathUtils tests (recursive methods):");
let math = new MathUtils();
print("  5! =");
print(math.factorial(5));
print("  10! =");
print(math.factorial(10));
print("  fib(10) =");
print(math.fibonacci(10));

print("\nAll class tests completed!");
