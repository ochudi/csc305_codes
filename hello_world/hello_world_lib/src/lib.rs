pub fn greeting_from_lib() -> String {
    let message = String::from("Hello from lib");
    println!("{}", message);
    message
}

trait Shape {
    fn area(&self) -> f32;
    fn new(length: f32, width: f32, name: &'static str) -> Self;
    fn get_length(&self) -> f32;
    fn set_length(&mut self, length: f32);
    fn get_width(&self) -> f32;
    fn set_width(&mut self, width: f32);
    fn get_name(&self) -> &'static str;
    fn set_name(&mut self, name: &'static str);
}

#[derive(Debug, Clone, Copy)]
struct Rect {
    length: f32,
    width: f32,
    name: &'static str,
}

impl Rect {
    fn default() -> Self {
        Rect {
            length: 1f32,
            width: 1f32,
            name: "default_name",
        }
    }
}

impl Shape for Rect {
    ///Associated function used to create a new Shape
    fn new(length: f32, width: f32, name: &'static str) -> Self {
        Rect {
            length,
            width,
            name,
        }
    }

    ///Area method
    fn area(&self) -> f32 {
        self.length * self.width
    }

    fn get_length(&self) -> f32 {
        self.length
    }

    fn set_length(&mut self, length: f32) {
        self.length = length;
    }

    fn get_width(&self) -> f32 {
        self.width
    }

    fn set_width(&mut self, width: f32) {
        self.width = width;
    }

    fn get_name(&self) -> &'static str {
        self.name
    }

    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }
}

impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        // self.length == other.length && self.width == other.width && self.name == other.name
        self.area() == other.area()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

trait Shape2 {
    fn area(&self) -> f32;
    // fn new(length: f32, width: f32, name: &str) -> Self;
    fn get_length(&self) -> f32;
    fn set_length(&mut self, length: f32);
    fn get_width(&self) -> f32;
    fn set_width(&mut self, width: f32);
    fn get_name(&self) -> &str;
    fn set_name(&mut self, name: &str);
}

#[derive(Debug)]
struct Rect2 {
    length: f32,
    width: f32,
    name: String, // on
}

// Implement a From trait for Rect2 that takes a string slice with format "length"
impl Rect2 {
    fn _default() -> Self {
        Rect2 {
            length: 1f32,
            width: 1f32,
            name: String::from("default_name"),
        }
    }
}

impl From<&str> for Rect2 {
    fn from(s: &str) -> Self {
        let mut parts = s.split(',');
        let length = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0f32,
        };
        let width = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0f32,
        };
        let name = match parts.next() {
            Some(val) => val,
            None => "",
        };

        Rect2 {
            length,
            width,
            name: name.to_owned(),
        }
    }
}

// Implement Into trait for Rect2
impl Into<String> for Rect2 {
    fn into(self) -> String {
        // let's return a template string template literal
        format!("My name is {}, and my area is {}.", self.name, self.area())
    }
}

impl Shape2 for Rect2 {
    ///Area method
    fn area(&self) -> f32 {
        self.length * self.width
    }

    fn get_length(&self) -> f32 {
        self.length
    }

    fn set_length(&mut self, length: f32) {
        self.length = length;
    }

    fn get_width(&self) -> f32 {
        self.width
    }

    fn set_width(&mut self, width: f32) {
        self.width = width;
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }
}

pub fn run() {
    let rectangle1 = Rect {
        length: 2f32,
        width: 6f32,
        name: "Rectangle 1",
    };
    let mut rectangle2 = Rect::default();
    rectangle2.set_length(10f32);
    rectangle2.set_width(5f32);
    let rectangle3 = rectangle1.clone();
    let rectangle4 = rectangle2.clone();
    let _rectangle5 = Rect {
        length: 12f32,
        ..rectangle1
    };

    println!(
        "rectangle 1 is {:#?} \nrectangle 2 is {:#?}",
        rectangle1, rectangle2
    );
    println!("Area of rectangle 1 is {}", rectangle1.area());
    assert_eq!(rectangle1, rectangle3);
    assert_eq!(rectangle2, rectangle4);
    //assert_eq!(rectangle1, _rectangle5);
    println!("If you can see this, your two triangles are equal.");
}

pub fn run2() {
    let mut _rectangle1 = Box::new(Rect2 {
        length: 12f32,
        width: 9f32,
        name: "Rectangle 1".to_owned(),
    });
    let rectangle2 = Rect2::from("20.0,30.0,Rectangle3");
    let rectangle3: Rect2 = "25.0,7.0,Rectangle3".into();

    let s: String = rectangle3.into();

    println!("Rectangle 1 = {:#?}", _rectangle1);
    println!("Area of Rectangle 2= {}", rectangle2.area());
    println!("About me: {}", s);
}

///Functions and Closures

//In Rust, functions have their own types

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn apply(f: fn(i32, i32) -> i32, x: i32, y: i32) -> i32 {
    f(x, y)
}

///let's define another function that handles straight line graph formula
///Assuminng that m, c and x have to be passed.
///Here you can use a normal function.
///Below, we have to use array slice as x, otherwise, we will need to specify a size.

fn _straight_line_function(m: i32, c: i32, xses: &[i32]) -> Vec<(i32, i32)> {
    let mut output: Vec<(i32, i32)> = Vec::new(); //you could also use vec![] to bring in initial arguments
    for x in xses {
        let y = (m * x) + c;
        output.push((*x, y)) //here we have to dereference the borrowed x, to get the value
    }
    output
}

pub fn run3() {
    let f = add;
    let x = 7;
    let y = 8;
    let z = apply(f, x, y);

    println!("The result of applying f to {} and {} is {}", x, y, z);
}

pub fn run6() {
    let mut x = 10;

    println!("x before change = {}", x);

    let y = &mut x; // y is a mutable reference to x
    let z: *const u32 = y; // z is an immutable raw pointer to y which references x
    let a = y as *mut u32; // a is a mutable raw pointer to y which references x
                           // let a: *mut u32 = y; // a is a mutable raw pointer to y which references x

    println!("y = {:?}", y); // expect value in x
    println!("z = {:?}", z); // expect memory address
    println!("a = {:?}", a); // expect same memory address as z above

    *y = 11; // expect value in x to change
    println!("x after first change = {:?}", x);

    unsafe {
        *a = 12;
        assert!(x == 12)
    }
}

pub fn run7() {
    //Error handling
    //See his slides for references

    //panic!("Problem. You called panic");

    //Illustrate Some
    let mut v = vec!["a", "b", "c"];

    // pop an element from the vector
    let x = v.pop();

    /*
    println!(
        "x is: {}",
        x.expect("I expect a value from my vector. You messed up")
    );
    */

    //What if we know that there is a possibility of having no Some value

    match x {
        Some(value) => println!("Popped {}", value),
        None => println!("Your vector is empty"),
    }

    //Compare above to:
    let mut v2: Vec<&str> = Vec::new();

    // let mut x2 = v2.pop().unwrap; // Will panic
    // let mut x2 = v2.pop().expect("Do not call pop on an empty vector");

    let x2 = v2.pop();

    match x2 {
        Some(value) => println!("Popped {}", value),
        None => println!("Your vector is empty"),
    }

    //Let's use ? for Option
    let mut v3 = vec![1, 2, 3];

    let mut plus_one = || -> Option<i32> { Some(v3.pop()? + 1) };

    println!("Plus one: {}", plus_one().unwrap());
}

//Let's see Result instead of Option
//Here it returns OK value vs Err, unlike Option that returns Some value vs None

//Adjust the following to return Result
pub fn multiplier(numbers: &[f64]) -> f64 {
    let mut product = 1f64;

    for n in numbers {
        product *= n;
    }

    product
}

//What if we want to return Error to the caller of this function when less than 2 arguments are passed?

#[derive(Debug)]
pub struct ErrorTypes {
    pub number: u8,
    pub message: &'static str,
    pub detail: &'static str,
}

//Let's create static variables for our error types
const INVALID_ARG_LEN_ERR: ErrorTypes = ErrorTypes {
    number: 101,
    message: "Invalid argument length",
    detail: "Two or more arguments are expected.",
};

const _INVALID_ARG_TYPE_ERR: ErrorTypes = ErrorTypes {
    number: 102,
    message: "Invalid argument type. f64 expected",
    detail: "Invalid argument type. f64 expected. You must convert your argument to f64.",
};

pub fn mature_multiplier(numbers: &[f64]) -> Result<f64, ErrorTypes> {
    if numbers.len() < 2 {
        return Err(INVALID_ARG_LEN_ERR);
    };

    let mut product = 1f64;

    for n in numbers {
        product *= n;
    }

    Ok(product)
}

/*
Case 1: We would like to create a macro that would allow us instantiate
one or more rectangles (along with their Shape trait impl) at a go. i.e., :
rectangles!((length:f32, width:f32, name:&str),…,n)
E.g., rectangles!((1, 1, "rect1"), (3.5, 7.0, "rect2"))
 */

#[macro_export] //in-built in Rust
macro_rules! rectangles {
    ($($rect_props_tuple:expr), *) => {
        //I want to return a vector of rectangles.
        {
            let mut rect_vec = Vec::new();
            //take our expression received, get the length, width and name from each
            //and create the appropriate rectangle and push each to our vector.

            $(let (length, width, name) = $rect_props_tuple;
            let rect = Rect{length : length as f32, width : width as f32, name: name as &str};

            rect_vec.push(rect);
            )*

            rect_vec
        }
    };
}

//Try our rectangles! Declarative macro.
pub fn run9() {
    let rects = rectangles!((1, 1, "rect_1"), (3.5, 7.0, "rect_2"));
    println!(
        "Area of rectangle 1 = {}; Area of rectangle 2 = {}",
        rects[0].area(),
        rects[1].area(),
    )
}

//You can also have multiple expressions in a declarative macro.
//What if you want a second expression that contains defaults for length, width and name for the rect.
//This implies that length, width and name will be optional.

#[macro_export]
macro_rules! rectangles_with_default {
    (($($rect_props_tuple:expr), *), $default_tuple:expr) => {
        {
            let mut rect_vec = Vec::new();
            let (default_length, default_width, default_name) = $default_tuple;

            $(
                let (length, width, name) = $rect_props_tuple;
                let rect = Rect{
                    length: if length.is_none() { default_length as f32 } else {length.unwrap() as f32},
                    width: if width.is_none() { default_width as f32 } else {width.unwrap() as f32},
                    name: if name.is_none() { default_name as &str } else {name.unwrap() as &str}
                };
                rect_vec.push(rect);
            )*

            rect_vec
        }
    };
}

pub fn run10() {
    let rects = rectangles_with_default!(
        (
            (None as Option<i32>, Some(1), Some("rect_1")),
            (Some(3.5), Some(7.0), None as Option<&str>)
        ),
        (1, 1, "default_name")
    );

    println!(
        "Area of rectangle 1 = {}; Area of rectangle 2 = {}",
        rects[0].area(),
        rects[1].area(),
    );
    println!(
        "Rectangle 1 is named: {}; Rectangle 2 is named: {}",
        rects[0].name,
        rects[1].name
    );
}
