struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let age: u32 = 30; // Declaring an unsigned 32-bit integer variable 'age' with value 30
    let temperature: i8 = -20; // Declaring a signed 8-bit integer variable 'temperature' with value -20

    let pi: f64 = 3.14159; // Declaring a double-precision floating-point variable 'pi' with the value of pi
    let distance: f32 = 10.5; // Declaring a single-precision floating-point variable 'distance' with value 10.5

    let is_night: bool = true; // Declaring a boolean variable 'is_night' with value true
    let is_weekend: bool = false; // Declaring a boolean variable 'is_weekend' with value false

    let initial: char = 'A'; // Declaring a character variable 'initial' with value 'A'
    let symbol: char = '&'; // Declaring a character variable 'symbol' with value '&'

    let colors: [u8; 3] = [255, 0, 0]; // Declaring an array 'colors' of size 3 with values red (255), green (0), and blue (0)
    let first_color = colors[0]; // Accessing the first element (red)

    let person: (String, u32) = ("John Doe".to_string(), 30); // Tuple with name (string) and age (integer)
    let name = person.0; // Accessing the first element (name)
    let age = person.1; // Accessing the second element (age)

    let origin: Point = Point { x: 0, y: 0 }; // Creating a Point struct with x and y coordinates
    println!("Origin coordinates: ({}, {})", origin.x, origin.y);
es