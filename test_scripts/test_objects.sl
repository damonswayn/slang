let obj = {
    x: 10,
    y: 15,
    z: [1, 2, 3, 4, 5, 6],
    add: fn (a, b) { a + b; }
};

print(obj.x);
print(obj.y);
print(obj.add(obj.x, obj.y));

obj.x = 15;

print(obj.add(obj.x, obj.y));

for (let i = 0; i < len(obj.z); i = i + 1) {
    print(obj.z[i]);
}