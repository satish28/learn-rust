//slope of a line given the coordinates of the line

struct Point {
    x: f32,
    y: f32,
    }

fn slope(a: Point,  b: Point){
    let ans  = (a.y - b.y) / (a.x - b.x);
    println!("The slope of the given line is {}", ans);
}

fn main(){
 
    let a = Point{x : 10.0, y : 20.0};
    let b = Point{x : 40.0, y : 5.0};

    slope(a, b);
}
