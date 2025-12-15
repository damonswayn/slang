(:TagOne, :TagTwo, :TagThree)
function add(a, b) {
    print(a + b);
}

(:TagOne, :TagThree)
function mul(a, b) {
    print(a * b);
}

(:TagTwo)
function sub(a, b) {
    print(a - b);
}

(:Square)
function square(arr) {
    let o = arr[0];
    let s = o * o;
    print(s);
    return s;
}

(:Print)
function output(arr) {
    // The pub/sub pipeline sends a single array when a stage has one param.
    print(arr[0]);
}

let a = 5;
let b = 7;

a, b -> :TagOne

a + b -> :Square 
      -> :Print
