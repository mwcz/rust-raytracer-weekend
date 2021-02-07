class Point {
    constructor(x, y) {
        this.x = x;
        this.y = y;
    }

    add(other) {
        return new Point(this.x + other.x, this.y + other.y);
    }

    toString() {
        return `(${this.x}, ${this.y})`;
    }
}

for (let i = 0; i < 10000000; ++i) {
    let a = new Point( 100, 100 );
    let b = new Point( 200, 200 );
    let c = a.add(b);
    // console.log(c.toString());
}
