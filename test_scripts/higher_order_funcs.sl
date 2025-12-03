let makeAdder = fn(x) {
    function(y) { x + y; }; // this inner fn closes over x
};

let addTwo = makeAdder(2);
print(addTwo(3)); // => 5

let addTen = makeAdder(10);
print(addTen(7)); // => 17