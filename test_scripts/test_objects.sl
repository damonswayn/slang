let obj = {
    x: 10,
    y: 15,
    z: [1, 2, 3, 4, 5, 6],
    add: fn () { this.x + this.y; }
};

print(obj.x);
print(obj.y);
print(obj.add());

obj.x = 15;

print(obj.add());

for (let i = 0; i < len(obj.z); i = i + 1) {
    print(obj.z[i]);
}