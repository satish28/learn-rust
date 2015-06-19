struct Point {
    x: f32,
    y: f32,
    }

//calculate slope of a line given its coordinates
fn slope(a: &Point,  b: &Point) -> f32{
    let ans  = (a.y - b.y) / (a.x - b.x);
    return ans;
}

//Calculate the mid-point of a line
fn midpoint(a :&Point, b: &Point) -> Point{
    let mid = Point{x: (a.x + b.x)/2.0, y: (a.y + b.y)/2.0};
    return mid;
    }

fn main(){
 
    // coordinates of a line
    let a = Point{x : 10.0, y : 20.0};
    let b = Point{x : 40.0, y : 5.0};
    
    println!("Slope of the given line is {}", slope(&a, &b));
    
    let mid = midpoint(&a, &b);

    //Destructuring a struct data type using match
    match mid{
        Point {x: mid_x, y: mid_y} => println!("the mid point of the given line is ({}, {})", mid_x, mid_y),
        }
}
