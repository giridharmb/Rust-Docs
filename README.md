#### Notes

Please have a look the following file for code snippets/samples

#### [src/main.rs](https://github.com/giridharmb/rust-app1/blob/master/src/main.rs)

How To Create A New Cargo Project (Executable App) ?

```bash
cargo new test_app --bin
```

How To Run The Application ?

```bash
cargo run
```

How To Run The Unit Tests ?

```bash
cargo test
```

```rust
// In this print statement >
println!("{:?}", my_data);

{:?} -> is debug flag/trait
{:#?} -> is pretty debug flag/trait

// To apply debug flag/trait, we need to add this >

#[derive(Debug)] -> this is an annotation
```

#### Ownership (Borrowing Data)

Notes-1:

- Programs must track memory
- If they fail to do so, a memory leak occurs
- Rust utilizes an "ownership" model to manage memory
- The owner of the memory is responsible for cleaning up the memory
- Memory can either be "moved" or "borrowed"

Notes-2:

- Memory must be managed in some way to prevent leaks
- Rust uses "ownership" to accomplish memory management
- The "owner" of the data must clean up the memory
- This occurs automatically at the end of the scope
- Default behaviour is to "move" memory to a new owner
- Use an `&` to allow the code to "borrow" memory

The below program *will not* compile

```rust
enum Light {
    Bright,
    Dull,
}

fn display_light(light: Light) {
    match light {
        Light::Bright => println!("bright !"),
        Light::Dull => println!("dull !"),
    }
}

fn main() {
    let my_light = Light::Dull;
    display_light(my_light); // ---> my_light gets moved (or owned) to (display_light) function
    display_light(my_light); // --------> this is an error (program wont compile)
}
```

To fix the above program, do it this way

```rust

enum Light {
    Bright,
    Dull,
}

fn display_light(light: &Light) {
    match light {
        Light::Bright => println!("bright !"),
        Light::Dull => println!("dull !"),
    }
}

fn main() {
    let my_light = Light::Dull;
    display_light(&my_light); // ---> here we are (borrowing) my_light
    display_light(&my_light); // ---> here we are (borrowing) my_light

    // FYI : main() function is still the owner of my_light
    // display_light(...) function -> cannot delete my_light, as it is borrowing it.
}
```

#### Strings

`String` -> Owned (Must be used in Structs)

`&str` -> borrowed String slice (Use this when passing data to a function)

- Strings are automatically borrowed
- Use `.to_owned()` or `String::from()` to create an owned copy of a string slice
- Use an owned String when storing in struct

Also, `#[derive(...)` macro is usually applied to `enums` & `structs`

For instance >

```rust
#[derive(Debug, Clone, Copy)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}

#[derive(Debug, Clone, Copy)]
struct Employee {
    position: Position,
    work_hours: i64,
}
```

Also note, since we have `position: Position` in struct `Employee`, <br/>
same derive `#[derive(Debug, Clone, Copy)]` must be applied to both the places.<br/>
Otherwise, we will get compilation error.

If we apply `Copy` derive -> a `copy` is made, instead of `move` when passed to a function

So if we expand the above piece of code,

```rust
fn print_employee(emp: Employee) {
    println!("{:?}", emp);
}

fn main() {
    let me = Employee {
        position: Position::worker,
        work_hours: 40,
    }
    println!(me); // because of `Copy` derive, ownership * is not * transferred
    println!(me); // because of `Copy` derive, ownership * is not * transferred
}
```

*Important Note*:

Apply `Clone` & `Copy` derive for `enum` and `structs` which are small in size.

If they are large, then expensive copies will made each time functions are called.

Without `#[derive(... , Clone, Copy, ...)]` (that is `Copy` & `Clone`), the above<br/>
program will usually throw error.

Since we are making a `copy` of the piece of data, program will run without errors.

```rust
fn print_it(data: &str) {
    println!("{:?}", data);
}

fn main() {
    print_it("a string slice"); // using it this way -> is automatically borrowed
    let owned_string = "owner string".to_owned();
    let another_owned = String::from("another");
    print_it(owned_string);
    print_it(another_owned);
}
```

This *will not* work

```rust
struct Employee {
    name: &str,
}
```

This is the way to use a (string) in a struct

```rust
struct Employee {
    name: String,
}
```

#### Impl (Implementation) | Structs

```rust
struct Temperature {
    degrees_f: f64,
}

impl Temperature {
    fn freezing() -> Self { // 'Self' -> refers to 'Temperature' here
        return Self {
            degrees_f: 32.0
        };
    }

    fn show_temp(&self) {
        println!("{:?} degrees F", self.degrees_f);
    }
}

fn main() {
    let hot = Temperature { degrees_f: 99.9 };
    hot.show_temp();

    let cold = Temperature::freezing();
    cold.show_temp();
}
```

Output

```bash
99.9 degrees F
32.0 degrees F
```

Another Impl (Implementation)

```rust
enum Color {
    Brown,
    Red,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Brown => println!("brown"),
            Color::Red => println!("red"),
        }
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("width : {:?}", self.width);
        println!("height : {:?}", self.height);
        println!("depth : {:?}", self.depth);
    }
}

struct ShippingBox {
    color: Color,
    weight: f64,
    dimensions: Dimensions,
}

impl ShippingBox {
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }

    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("weight : {:?}", self.weight);
    }
}

fn main() {
    let my_dimensions  = Dimensions {
        width: 2.0,
        height: 3.5,
        depth: 4.5,
    };

    let my_color = Color::Brown;
    let my_box = ShippingBox::new(99.00, my_color, my_dimensions);
    my_box.print();
}
```

Output

```bash
brown
width : 2.0
height : 3.5
depth : 4.5
weight : 99.0
```

`ownership` | `borrowing` | `references`

Important : Use of Reference

```rust
struct MyObject {
    width: u32,
    height: u32
}

let my_object = MyObject {
    width:15,
    height:20
};

// If you calculate area this way >

fn calculate_area(obj: MyObject) -> u32 {
    obj.width * obj.height
}

calculate_area(my_object); // -> this will result in error !

// You will see an error in the compiler this way:

// error[E0505]: cannot move out of `my_object` because it is borrowed
// move out of `my_object` occurs here

// Instead, you should calculate the area this way:

fn calculate_area(obj: &MyObject) -> u32 { // note that it is (obj: &MyObject)
    obj.width * obj.height
}

calculate_area(&my_object); // this will work as expected
```

Only one reference can own one peice of data at any given time.

```rust
let s = String::from("hello");
let y = s;
println!("{}", s); // -> this is NOT VALID , since y owns s now
```

To fix it

```rust
let s = String::from("hello");
let y = &s; // this is called "borrowing" , y is borrowing reference of s
println!("{}", s); // -> now, this is perfectly fine
```

HEAP (for complex data types)

```rust
fn take_data(v: Vec<i32>) {
    println!("We took v : sum of v[0] + v[1] is equal to : {}", v[0] + v[1]);
}

fn main() {
    let mut v = Vec::new();

    for i in 1..100 {
        v.push(i);
    }

    take_data(v); // this is called 'moving'

    // take_data(...)
    // above this peice of code, will transfer the
    // ownership of 'v' from main() function to take_data() function

    // println!("We took v : sum of v[0] + v[1] is equal to : {}", v[0] + v[1]);

    println("finished !");
}
```

Copying

```rust
fn copy_data(a: i32, b: i32) {
    println!("{}", a+b);
}

fn main() {
    let a = 32;
    let b = 45;

    // a and b -> exist on the stack, and not the heap,
    // so in the below call function call 'copy_data(a,b)'
    // they are 'copied' and not 'moved'
    // so, they will still remain in the scope of main()

    copy_data(a,b);

    println!("we have a: {} , b: {}", a,b);
}
```

Borrowing

- borrowing lets us have multiple references to a single resource
- a reference is also an object in rust
- `mutable` references -> are `MOVED`
- `immutable` references -> are `COPIED`

<hr/>

`mutable` vs `immutable` references

```rust
fn re(v: Vec<i32>) -> Vec<i32> {
    println!("{}", v[15] + v[25]);
    return v
}

fn borrow_1(v: &Vec<i32>) {
    println!("{}", (*v)[10] + (*v)[20]); // note : this is '(*v)' and not 'v'
}

fn borrow_2(v: &Vec<i32>) {
    println!("{}", v[30] + v[40]); // note : this is 'v' and not '(*v)'
}

fn main() {
    let mut v = Vec::new();

    for i in 1..1000 {
        v.push(i);
    }

    v = re(v); // re() function will transfer ownership of 'v' back to main() function

    println!("still own v : {} {}", v[0], v[1]);

    borrow_1(&v);
    println!("still own v : {} {}", v[0], v[1]);

    borrow_2(&v);
    println!("still own v : {} {}", v[0], v[1]);
}
```

Result Type : Is of type `enum`

```rust
let mut my_input_string = String::new();

println!("enter a number: ");

io::stdout().flush().unwrap();

// method-1 (not safe)
// completely commented out

/*
unwrap() will extract data from Result
unwrap() : This is not safe to do : user can input text instead of numbers == crash
*/

// method_1_start
{
    stdin().read_line(&mut my_input_string).expect("did not enter valid input !");
    let my_number: f64 = my_input_string.trim().parse().unwrap();
    let my_number: f64 = my_input_string.trim().parse().expect("invalid input, enter a number !");
    println!("Yay ! you entered a number : {:?}", my_number);
}
// method_1_end


// method-2 (safe) -> use it this way !

// what ever value loop returns, that will be stored in my_num
// whatever value comes after 'break', that value will be returned by loop

// method_2_start
let my_num = loop {
    my_input_string.clear();

    stdin().read_line(&mut my_input_string).expect("did not enter a correct string !");

    match my_input_string.trim().parse::<f64>() {
        Ok(_s) => {
            break _s;
        }
        Err(_err) => {
            println!("Try again, invalid input string.");
        }
    }
};
println!("Yay ! you entered a number : {:?}", my_num);
// method_2_end
```

Result -> enum : it represents success or failure

```rust
enum Result<T, E>
{
    Ok(T),
    Err(E),
}
```

Rust's named lifetimes

```
Every reference in Rust has a lifetime, which is the scope for which that 
reference is valid. Most of the time lifetimes are implicit and inferred, 
just like most of the time types are inferred. 
Similarly to when we have to annotate types because multiple types are 
possible, there are cases where the lifetimes of references could be related 
in a few different ways, so Rust needs us to annotate the relationships 
using generic lifetime parameters so that it can make sure the actual 
references used at runtime will definitely be valid.

Lifetime annotations don’t change how long any of the references involved live. 
In the same way that functions can accept any type when the signature specifies a 
generic type parameter, functions can accept references with any lifetime when the 
signature specifies a generic lifetime parameter. What lifetime annotations do is 
relate the lifetimes of multiple references to each other.

Lifetime annotations have a slightly unusual syntax: the names of lifetime parameters 
must start with an apostrophe '. The names of lifetime parameters are usually all lowercase, 
and like generic types, their names are usually very short. 'a is the name most people 
use as a default. Lifetime parameter annotations go after the & of a reference, 
and a space separates the lifetime annotation from the reference’s type.
```

```rust
pub fn func_test(a: &str, b: &str) -> &str {
    return b
}
```

Above function won't compile because it returns a borrowed value, <br/>
but does not specify whether it borrowed it from `a` or `b`.

`func_test(...)` throws the following error:

```
error[E0106]: missing lifetime specifier

XXX | pub fn func_test(a: &str, b: &str) -> &str {
    |                     ----     ----     ^ expected named lifetime parameter

help: this function's return type contains a borrowed value, but the signature
does not say whether it is borrowed from `a` or `b`

help: consider introducing a named lifetime parameter

XXX | pub fn func_test<'a>(a: &'a str, b: &'a str) -> &'a str {
    |                 ++++     ++          ++          ++
```

To understand it better , run this 

`rustc --explain E0106`

To fix the issue, you'd declare a named lifetime and use the same lifetime for `a` or `b` and the return type:

Replace the above function with this

```rust
pub fn func_test<'r>(a: &'r str, b: &'r str) -> &'r str {
    return b
}
```

And use it as expected

```rust
func_test("a", "b")
```

#### Option

The below code will not compile, because `id` can be anything apart from value of `1`

```rust
fn lookup_player(id: u32) -> String {
    if id == 1 {
        return "Giridhar Bhujanga".to_string()
    }
}
```

To fix this, we can use `Option`

```rust
fn lookup_player(id: u32) -> Option<String> {
    if id == 1 {
        let data = "Giridhar Bhujanga".to_string();
        return Option::Some(data);
    }
    return Option::None;
}
```

This function return signature : `lookup_player(id: u32) -> Option<String>` <br/>
means that we either return some `String` or `None`.

We can simplify it further by using `enum`:

```rust
fn lookup_player(id: u32) -> Option<String> {
    use Option::Some;
    use Option::None;

    if id == 1 {
        let data = "Giridhar Bhujanga".to_string();
        return Some(data);
    }
    return None;
}
```

Now we can call the above function `lookup_player`:

```rust
fn run_game() {
    let player = match lookup_player(1) {
        Some(p) => p,
        None => return
    };
}
```

FYI : An `empty tuple` is `()`, is also called a `unit`.

```rust
fn run_game() -> Option<()> {
    let player = lookup_player(1)?;
    println!("Player : {}", player);
    Some(())
}
```

Note : `?` above, that is <br/>
`lookup_player(1)?` : This will return `None`, If `lookup_player(1)` returns `None`.

#### Error Handling

Error Handling : Example-1

```rust
// Error Handling

use std::fs::File;

fn main() {
    println!("this is an example of error handling");
    let f = File::open("random.txt");

    let f = match f {
        Ok(file) => file,
        Err(err) => panic!("there was an error opening the file : {:?}", err), 
    };
}
```

In the above example, there was an error and output of `cargo run` was

```bash
cargo run
```

```bash
    Finished dev [unoptimized + debuginfo] target(s) in 0.19s
     Running `target/debug/test_app`
this is an example of error handling
thread 'main' panicked at 'there was an error opening the file : Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:11:21
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

```bash
RUST_BACKTRACE=1 cargo run
```

```bash
this is an example of error handling
thread 'main' panicked at 'there was an error opening the file : Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:11:21
stack backtrace:
   0: rust_begin_unwind
             at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/std/src/panicking.rs:575:5
   1: core::panicking::panic_fmt
             at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/core/src/panicking.rs:65:14
   2: test_app::main
             at ./src/main.rs:11:21
   3: core::ops::function::FnOnce::call_once
             at /rustc/69f9c33d71c871fc16ac445211281c6e7a340943/library/core/src/ops/function.rs:251:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

Error Handling : Example-2

```rust
// Error Handling

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    println!("this is an example of error handling");
    let f = File::open("random.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.Kind() {
            // recover from the error and try to create the file
            // also note, trying to create the file could also fail
            ErrorKind::NotFound => match File::create("random.txt") {
                Ok(fc) => fc, // just return fc if file could be created
                Err(e) => panic!("there was a problem in trying to create the file : {:?}", e),
            },
            other_error => {
                panic!("there was a different problem in opening the file : {:?}", other_error)
            }
        },
    };
}
```

Error Handling : Example-3

```rust
// Error Handling

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    println!("this is an example of error handling");
    let f = File::open("random.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("random.txt").unwrap_or_else(|error| {
                panic!("could not create the file : {:?}", error);
            }) // unwrap_or_else
        } else {
            panic!("could not open the file : {:?}", error);
        }
    }); //unwrap_or_else
}
```

Error Handling : Example-4

```rust
// Error Handling

use std::net::{AddrParseError, IpAddr};
use std::net::{Ipv4Addr};
use std::str::FromStr;


#[derive(Debug)]
pub enum GenericError {
    InvalidInput,
}

#[derive(Debug)]
struct CustomError {
    err_type: GenericError,
    err_msg: String
}

pub type IpCheckResult = Result<bool, CustomError>;

fn validate_ip_address(my_ip: &String) -> IpCheckResult {
    let ip_addr = Ipv4Addr::from_str(my_ip);
    let resp = match ip_addr {
        Ok(ip) => {
            println!("ip-address : {} , is valid !", my_ip);
            true
        },
        Err(e) => {
            println!("ip-address : {} , is *not* valid !", my_ip);
            let custom_err = CustomError{
                err_msg: String::from("there was an error in parsing the ipaddress"),
                err_type: GenericError::InvalidInput
            };
            return Err(custom_err)
        },
    };
    Ok(resp)
}

fn main() {
    println!("this is an example of error handling");

    // valid input
    let my_str = String::from("127.0.0.77");
    let _ = match validate_ip_address(&my_str) {
        Ok(data) => {
            println!("success : {:?}", data)
        }
        Err(e) => {
            println!("error : ip address validation failed : {:#?}", e)
        }
    };

    // invalid input
    let my_str = String::from("127.0.0.777");
    let _ = match validate_ip_address(&my_str) {
        Ok(data) => {
            println!("success : {:?}", data)
        }
        Err(e) => {
            println!("error : ip address validation failed : {:#?}", e)
        }
    };
}
```

Output of Example-4

```bash
cargo run
```
```bash
this is an example of error handling
ip-address : 127.0.0.77 , is valid !
success : true
ip-address : 127.0.0.777 , is *not* valid !
error : ip address validation failed : CustomError {
    err_type: InvalidInput,
    err_msg: "there was an error in parsing the ipaddress",
}
```

Error Handling : Example-5 (Without Error)

```rust
use std::collections::HashMap;
use reqwest::{Response, StatusCode};


fn main() {
    match get_current_date() {
        Ok(date) => println!("We've time travelled to {} !", date),
        Err(e) => eprintln!("Oh noes, we don't know which era we're in ! :( \n  {}", e),
    }
}

fn get_current_date() -> Result<String, reqwest::Error> {
    let url = "https://postman-echo.com/time/object";

    let client = reqwest::blocking::Client::new();

    let res = client.get(url).send();

    let response = match res {
        Ok(res) => res,
        Err(err) => return Err(err),
    };

    let body = response.json::<HashMap<String, i32>>();

    let json = match body {
        Ok(json) => json,
        Err(err) => return Err(err),
    };

    let date = json["years"].to_string();

    Ok(date)
}
```

Output of Example-5 (Without Error)

```bash
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/test_app_v2`
We've time travelled to 2023 !
```

Error Handling : Example-5 (*With* Error , Invalid URL Passed)

```rust
use std::collections::HashMap;
use reqwest::{Response, StatusCode};


fn main() {
    match get_current_date() {
        Ok(date) => println!("We've time travelled to {} !", date),
        Err(e) => eprintln!("Oh noes, we don't know which era we're in ! :( \n  {}", e),
    }
}

fn get_current_date() -> Result<String, reqwest::Error> {
    let url = "https://postman-echo.com/time/object-XXX";

    let client = reqwest::blocking::Client::new();

    let res = client.get(url).send();

    let response = match res {
        Ok(res) => res,
        Err(err) => return Err(err),
    };

    let body = response.json::<HashMap<String, i32>>();

    let json = match body {
        Ok(json) => json,
        Err(err) => return Err(err),
    };

    let date = json["years"].to_string();

    Ok(date)
}
```

Output of Example-5 (*With* Error , Invalid URL Passed)

```bash
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/test_app_v2`
Oh noes, we don't know which era we're in ! :(
  error decoding response body: EOF while parsing a value at line 1 column 0
```

#### Option

Rust avoids the billion dollar mistake of including nulls in the language. <br/>
Instead, we can represent a value that might or might not exist with the Option type.<br/>
This is similar to Java 8 Optional or Haskell’s Maybe. There is plenty of material out<br/>
there detailing why an Option type is better than null.<br/>

In Rust, `Option<T>` is an enum that can either be `None` = `(no value present)` <br/>
or `Some(x)` = `(some value present)`.<br/>

Please see the `// comments` in the code below.<br/>

```rust
struct User {
    name: String,
    car: Option<String>,
}

fn print_user_details(u: &User) {
    println!("user name : {}", u.name);

    // Important !

    // In the above struct (User), value for car is Option<String>
    // So, to avoid panic, we have to check if the value for car is
    // Some(x) or None , as show below in match

    match &u.car {
        None => {
            println!("user ( {} ) has no car !", u.name);
        }
        Some(d) => {
            println!("user car : {}", d);
        }
    }

}

fn main() {
    println!("Usage of Option");

    let user1 = User {
        name: String::from("Giridhar Bhujanga"),
        car: Some(String::from("Audi S4")),
    };

    let user2 = User {
        name: String::from("Sumeet Singh"),
        car: None, // ---> Important ! This is None for user2
    };

    print_user_details(&user1);
    print_user_details(&user2);

}
```

Output

```bash
cargo run
```

```bash
   Compiling test_option v0.1.0 (/Users/giri/git/rust/test_option)
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/test_option`
Usage of Option
user name : Giridhar Bhujanga
user car : Audi S4
user name : Sumeet Singh
user ( Sumeet Singh ) has no car !
```

#### Result & Option (Bubble Up Errors)

In the below code:

`print_user_details()` -> Returns a Result < `empty tuple` & `error` >

`u.car.as_ref().ok_or` -> This returns an error, in case (`None`) is the value for `car`

As per documentation in Rust:

```
ok_or : Transforms the Option<T> into a Result<T, E>, mapping Some(v) to Ok(v) and None to Err(err)
```

```rust
use std::error::Error;

struct User {
    name: String,
    car: Option<String>,
}

fn print_user_details(u: &User) -> Result<(),Box<dyn std::error::Error>>{
    println!("user name : {}", u.name);

    let user_car = match u.car.as_ref().ok_or("problem : could not get car details") {
        Ok(_) => {}
        Err(e) => {
            return Err(Box::try_from(String::from("( yikes : could not find car )")).unwrap());
        }
    };

    Ok(()) // if there was no error above, then code reaches here

}

fn main() {
    println!("Usage of Option");

    let user1 = User {
        name: String::from("Giridhar Bhujanga"),
        car: Some(String::from("Audi S4")),
    };

    let user2 = User {
        name: String::from("Sumeet Singh"),
        car: None, // ---> Important ! this is None
    };

    let result1 = match print_user_details(&user1) {
        Ok(_) => {}
        Err(e) => {
            println!("[ oops ] : there was an error while getting user details : {}", e);
        }
    };

    let result2 = match print_user_details(&user2) {
        Ok(_) => {}
        Err(e) => {
            println!("[ oops ] : there was an error while getting user details : {}", e);
        }
    };

}
```

Output

```bash
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/test_option`
Usage of Option
user name : Giridhar Bhujanga
user name : Sumeet Singh
[ oops ] : there was an error while getting user details : ( yikes : could not find car )
```

Another Use Case of Error Handling

```rust
fn describe_car(my_car: Option<&str>) {
    // Specify a course of action for each case.
    match my_car {
        Some("bmw") => println!("1> oh damn, this is sporty and a drivers car !"),
        Some("audi") => println!("2> sorry to hear this, I know it breaks down a lot"),
        Some("porsche") => println!("3> omg ! you must be rich to own this car for daily use !"),
        Some(other) => println!("4> hey, you have this *{}* ? I don't know much about this :(", other),
        None => println!("5> IS THIS EVEN A CAR ?"),
    }
}

fn does_it_need_servicing(mileage: Option<i32>) {

    // `unwrap` returns a `panic` when it receives a `None`.
    let car_miles = mileage.unwrap();

    if car_miles >= 150000 {
        panic!("[1] (Please sell the car) Panic*");
    } else {
        println!("[2] Hey Buddy, your car is fine , it only has {} miles.", car_miles);
    }
}

fn main() {
    let car1 = Some("bmw");
    let car2 = Some("audi");
    let car3 = Some("porsche");
    let car4 = Some("toyota");
    let car5 = None;

    describe_car(car1);
    describe_car(car2);
    describe_car(car3);
    describe_car(car4);
    describe_car(car5);

    let car_1_mileage = Some(10000);
    let car_2_mileage = Some(20000);
    let car_3_mileage = Some(250000);

    does_it_need_servicing(car_1_mileage);
    does_it_need_servicing(car_2_mileage);
    does_it_need_servicing(car_3_mileage);
}
```

The above program panics. The Output looks like this

```bash
1> oh damn, this is sporty and a drivers car !
2> sorry to hear this, I know it breaks down a lot
3> omg ! you must be rich to own this car for daily use !
4> hey, you have this *toyota* ? I don't know much about this :(
5> IS THIS EVEN A CAR ?
[2] Hey Buddy, your car is fine , it only has 10000 miles.
[2] Hey Buddy, your car is fine , it only has 20000 miles.
thread 'main' panicked at '[1] (Please sell the car) Panic*', src/main.rs:18:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

We can slightly modify the `error handling` this way and avoid program `crashing` via `panic`

```rust
use std::error::Error;

fn describe_car(my_car: Option<&str>) {
    // Specify a course of action for each case.
    match my_car {
        Some("bmw") => println!("1> oh damn, this is sporty and a drivers car !"),
        Some("audi") => println!("2> sorry to hear this, I know it breaks down a lot"),
        Some("porsche") => println!("3> omg ! you must be rich to own this car for daily use !"),
        Some(other) => println!("4> hey, you have this *{}* ? I don't know much about this :(", other),
        None => println!("5> IS THIS EVEN A CAR ?"),
    }
}

fn does_it_need_servicing(mileage: Option<i32>) -> Result<(), Box<dyn std::error::Error>> {

    // `unwrap` returns a `panic` when it receives a `None`.
    let car_miles = mileage.unwrap();

    if car_miles >= 150000 {
        return Err(Box::try_from("[1] (Please sell the car) Panic*").unwrap())
    } else {
        println!("[2] Hey Buddy, your car is fine , it only has {} miles.", car_miles);
    }

    Ok(())
}

fn main() {
    let car1 = Some("bmw");
    let car2 = Some("audi");
    let car3 = Some("porsche");
    let car4 = Some("toyota");
    let car5 = None;

    describe_car(car1);
    describe_car(car2);
    describe_car(car3);
    describe_car(car4);
    describe_car(car5);

    let car_1_mileage = Some(10000);
    let car_2_mileage = Some(20000);
    let car_3_mileage = Some(250000);

    let s1 = match does_it_need_servicing(car_1_mileage) {
        Ok(_) => {}
        Err(e) => {
            println!("oops : (s1) there was an error : {}", e)
        }
    };

    let s2 = match does_it_need_servicing(car_2_mileage) {
        Ok(_) => {}
        Err(e) => {
            println!("oops : (s2) there was an error : {}", e)
        }
    };

    let s3 = match does_it_need_servicing(car_3_mileage) {
        Ok(_) => {}
        Err(e) => {
            println!("oops : (s3) there was an error : {}", e)
        }
    };
}
```

Output

```bash
1> oh damn, this is sporty and a drivers car !
2> sorry to hear this, I know it breaks down a lot
3> omg ! you must be rich to own this car for daily use !
4> hey, you have this *toyota* ? I don't know much about this :(
5> IS THIS EVEN A CAR ?
[2] Hey Buddy, your car is fine , it only has 10000 miles.
[2] Hey Buddy, your car is fine , it only has 20000 miles.
oops : (s3) there was an error : [1] (Please sell the car) Panic*
```

Basically, If You Have A Look At This Snippet Below:

```rust
fn work_with_text() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("text.txt")?;
    // do something with content that may cause another type of error (rusqlite error)
    Ok(())
}
```

Having a `Box<dyn std::error::Error>` also allows you to return a wide variety of errors<br/>
from your function since most error types can be automatically converted<br/>
into a `Box<dyn std::error::Error>` via the `?` operator.

#### Rust Threading & Usage of `move` keyword

```rust
use std::thread;

fn main() {
    let v = vec![1,2,3];

    let handle = thread::spawn(move || {
        // due to 'move' above
        // the below line will have complete ownership of 'v'
        println!("vector : {:?}", v);
    });

    // due to 'move' above
    // this will throw error in the below print statement
    println!("{:?}", v);

    let join_result = match handle.join() {
        Ok(_) => {
            println!("thread successfully joined.");
        }
        Err(_) => {
            println!("failed to join the thread to the main thread");
        }
    };
}
```

The above program will throw the below error because we have used `move` (which is actually needed)

```bash
|     let v = vec![1,2,3];
|         - move occurs because `v` has type `Vec<i32>`, which does not implement the `Copy` trait
|
|     let handle = thread::spawn(move || {
|                                ------- value moved into closure here
|         println!("vector : {:?}", v);
|                                   - variable moved due to use in closure
| 
|     println!("{:?}", v);
|                      ^ value borrowed here after move
```

#### Advanced Match

```rust
enum Discount {
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: i32,
}

fn main() {
    let n = 3;
    match n {
        3 => println!("value is three !"),
        other => println!("number : {:?}", other),
    }

    let flat = Discount::flat(3);

    match flat {
        Discount::flat(2) => println!("flat 2"),
        Discount::flat(amount) => println!("flat discount of {:?}", amount),
        _ => (),
    }

    //---

    let concert = Ticket {
        event: "concert".to_owned(), // since "concert" is a slice and we need String as datatype
        price: 50,
    }

    // --- Matching on Struct ---
    // Note
    // below , we are using '..', which means -> ingore everything else in the Struct
    match concert {
        Ticket {price, ..} => println!("price : {:?}", price),
        Ticket {50, event} => println!("event is priced @ 50 : {:?}", event),
    }
}
```

Threading Example

```rust
use std::thread;

fn main() {
    println!("rust threading !");

    let mut c = vec![];

    // move keyword allows the closure
    // to use data from one thread to another thread
    for i in 0..10 {
        c.push(thread::spawn(move || {
            println!("thread number {}", i);
        }));
    }

    for my_thread in c {
        my_thread.join().expect("failed to join the thread to the main thread");
    }
}
```

Output

```bash
rust threading !
thread number 1
thread number 0
thread number 2
thread number 3
thread number 4
thread number 5
thread number 6
thread number 7
thread number 8
thread number 9
```

Async Functions

`Cargo.toml`

```toml
[package]
name = "async_functions"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
simple_logger = { version = "4.0.0", features = ["colors", "timestamps"] }
tokio = { version = "1.24.1", features = ["full"] }
log = { version = "0.4", features = ["std", "serde"] }
```

Code

```rust
use std::{thread, time};
use simple_logger::SimpleLogger;
use log::{info, warn, debug, error};


fn main() {
    println!("async function test !");
    SimpleLogger::new().init().unwrap();
    execute_async_function()
}

#[tokio::main]
async fn execute_async_function() {
    log::debug!("<START>");
    let my_future = my_function();
    log::debug!("<BEFORE AWAIT>");
    my_future.await;
    log::debug!("<AFTER AWAIT>");
}

async fn my_function() {
    log::debug!("my_function() ...");

    let now = time::Instant::now();

    let my_sleep_duration = time::Duration::from_millis(3000);

    thread::sleep(my_sleep_duration);
    let s1 = read_from_database().await;
    log::debug!("s1 : {}", s1);

    thread::sleep(my_sleep_duration);
    let s2 = read_from_database().await;
    log::debug!("s2 : {}", s2);

    log::debug!("my_function() is done.")

}


async fn read_from_database() -> String {
    return "DB Data".to_owned();
}
```

Output (Make a note of the timestamps, as sleep is used in the above code)

```bash
async function test !
2023-01-12T20:10:40.780Z TRACE [mio::poll] registering event source with poller: token=Token(2147483649), interests=READABLE
2023-01-12T20:10:40.781Z DEBUG [async_functions] <START>
2023-01-12T20:10:40.781Z DEBUG [async_functions] <BEFORE AWAIT>
2023-01-12T20:10:40.781Z DEBUG [async_functions] my_function() ...
2023-01-12T20:10:43.787Z DEBUG [async_functions] s1 : DB Data
2023-01-12T20:10:46.788Z DEBUG [async_functions] s2 : DB Data
2023-01-12T20:10:46.788Z DEBUG [async_functions] my_function() is done.
2023-01-12T20:10:46.788Z DEBUG [async_functions] <AFTER AWAIT>
```

#### Async Functions : Tasks (Green Threads)

To run `async` code concurrently, we can use `tokio-tasks`.

A `task` is a `light weight` , `non blocking` unit of execution.

It is a `green thread` (similar to a go-routine).

It allows top level `Futures` to be executed `concurrently`.

Note :
By default, TOKIO uses `thread pool` to execute tasks.
We can tell TOKIO to use one thread by changing this

`#[tokio::main]`
to this
`#[tokio::main(flavor = "current_thread")]`

This will make TOKIO to execute `tasks` `concurrently` - using `time slicing` instead of `threads`.

Note : to communicate between tasks,
We have to use `message passing` through a `channel`
Or
`Shared state` through a `mutex`.

Unlike threads, `async` code uses `cooperative scheduling`, instead of `preemptive scheduling`.

```rust
use std::{thread, time};
use simple_logger::SimpleLogger;
use log::{info, warn, debug, error};
use tokio::time::sleep;
use std::time::Duration;


fn main() {
    println!("async function test !");
    SimpleLogger::new().init().unwrap();
    execute_async_function()
}

// Note :
// By default, TOKIO uses thread pool to execute tasks
// We can tell TOKIO to use 1 thread by changing this
// #[tokio::main]
// to this
// #[tokio::main(flavor = "current_thread")]
// this will make TOKIO to execute tasks concurrently - using time slicing
// instead of threads

// Note : to communicate between tasks >
// we have to use message passing through a channel
// Or
// shared state through a mutex

// Unlike threads, async code uses cooperative scheduling, instead of preemptive scheduling

#[tokio::main]
async fn execute_async_function() {

    log::debug!("<START>");

    let mut handles = vec![]; // task handles

    for i in 0..2 {
        let handle = tokio::spawn(async move {
            my_function(i).await;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
    log::debug!("<END>");
}

async fn my_function(i: i32) {
    log::debug!("[{i}] my_function() ...");

    let s1 = read_from_database().await;
    log::debug!("[{i}] s1 : {}", s1);

    let s2 = read_from_database().await;
    log::debug!("[{i}] s2 : {}", s2);

    log::debug!("[{i}] my_function() is done.")

}

// Note : we should not be putting CPU intensive operations on async function

async fn read_from_database() -> String {
    sleep(Duration::from_millis(3000)).await;
    return "DB Data".to_owned();
}
```

Output

```bash
async function test !
2023-01-12T21:30:05.512Z TRACE [mio::poll] registering event source with poller: token=Token(2147483649), interests=READABLE
2023-01-12T21:30:05.513Z DEBUG [async_functions] <START>
2023-01-12T21:30:05.513Z DEBUG [async_functions] [0] my_function() ...
2023-01-12T21:30:05.513Z DEBUG [async_functions] [1] my_function() ...
2023-01-12T21:30:08.516Z DEBUG [async_functions] [1] s1 : DB Data
2023-01-12T21:30:08.516Z DEBUG [async_functions] [0] s1 : DB Data
2023-01-12T21:30:11.519Z DEBUG [async_functions] [0] s2 : DB Data
2023-01-12T21:30:11.519Z DEBUG [async_functions] [0] my_function() is done.
2023-01-12T21:30:11.519Z DEBUG [async_functions] [1] s2 : DB Data
2023-01-12T21:30:11.519Z DEBUG [async_functions] [1] my_function() is done.
2023-01-12T21:30:11.520Z DEBUG [async_functions] <END>
```

Channels (Sender and Receiver) With Multithreading

`Cargo.toml`

```toml
[package]
name = "multi_threading"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
log = { version = "0.4", features = ["std", "serde"] }
simple_logger = { version = "4.0.0", features = ["colors", "timestamps"] }
```

```rust
use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use simple_logger::SimpleLogger;

fn main() {
    println!("rust channels !");
    SimpleLogger::new().init().unwrap();
    let (tx, rx) = mpsc::channel();

    log::debug!("| START |");

    log::debug!("t1 starting...");
    let t1= thread::spawn(move || {
        //
        for i in 0..10 {
            thread::sleep(Duration::from_millis(1000));
            log::debug!("tx | sending data {}", i);
            tx.send(i).unwrap();
        }
    });
    log::debug!("t1 started !");

    log::debug!("t2 starting...");
    let t2 = thread::spawn(move || {
        for data in &rx {
            log::debug!("rx | received data : {}", data);
        }
    });
    log::debug!("t2 started !");

    t1.join().expect("could not join the thread t1 to main thread");
    t2.join().expect("could not join the thread t2 to main thread");

    log::debug!("| END |");
    println!("done processing tx (transmitter) & rx (receiver).")

}
```

Output

```bash
rust channels !
2023-01-12T22:57:06.247Z DEBUG [multi_threading] | START |
2023-01-12T22:57:06.247Z DEBUG [multi_threading] t1 starting...
2023-01-12T22:57:06.247Z DEBUG [multi_threading] t1 started !
2023-01-12T22:57:06.247Z DEBUG [multi_threading] t2 starting...
2023-01-12T22:57:06.247Z DEBUG [multi_threading] t2 started !
2023-01-12T22:57:07.252Z DEBUG [multi_threading] tx | sending data 0
2023-01-12T22:57:07.252Z DEBUG [multi_threading] rx | received data : 0
2023-01-12T22:57:08.258Z DEBUG [multi_threading] tx | sending data 1
2023-01-12T22:57:08.258Z DEBUG [multi_threading] rx | received data : 1
2023-01-12T22:57:09.262Z DEBUG [multi_threading] tx | sending data 2
2023-01-12T22:57:09.262Z DEBUG [multi_threading] rx | received data : 2
2023-01-12T22:57:10.268Z DEBUG [multi_threading] tx | sending data 3
2023-01-12T22:57:10.268Z DEBUG [multi_threading] rx | received data : 3
2023-01-12T22:57:11.270Z DEBUG [multi_threading] tx | sending data 4
2023-01-12T22:57:11.271Z DEBUG [multi_threading] rx | received data : 4
2023-01-12T22:57:12.276Z DEBUG [multi_threading] tx | sending data 5
2023-01-12T22:57:12.276Z DEBUG [multi_threading] rx | received data : 5
2023-01-12T22:57:13.279Z DEBUG [multi_threading] tx | sending data 6
2023-01-12T22:57:13.279Z DEBUG [multi_threading] rx | received data : 6
2023-01-12T22:57:14.284Z DEBUG [multi_threading] tx | sending data 7
2023-01-12T22:57:14.284Z DEBUG [multi_threading] rx | received data : 7
2023-01-12T22:57:15.287Z DEBUG [multi_threading] tx | sending data 8
2023-01-12T22:57:15.287Z DEBUG [multi_threading] rx | received data : 8
2023-01-12T22:57:16.292Z DEBUG [multi_threading] tx | sending data 9
2023-01-12T22:57:16.292Z DEBUG [multi_threading] rx | received data : 9
2023-01-12T22:57:16.295Z DEBUG [multi_threading] | END |
done processing tx (transmitter) & rx (receiver).
```

#### Worker Pool

Note : By default, the thread pool has a size equal to the number of logical CPUs of the machine.

`Cargo.toml`

```toml
[package]
name = "worker_pool"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rand = "0.8.5"
rayon = "1.6.1"
sha256 = "1.1.1"
```

```rust
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::time::Duration;
use rand::{distributions::Alphanumeric};
use sha256::digest;

fn compute_job(my_input: String) -> String {
    std::thread::sleep(Duration::from_millis(1000));
    let val = digest(my_input); // this will calcuate sha256 of given input (my_input)
    return val;
}

fn process_result(my_result: String) {
    println!("sha256 : {}", my_result);
}

fn generate_random_string() -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(9)
        .map(char::from)
        .collect();
    return s;
}

fn generate_jobs(number_of_inputs: i32) -> Vec<String> {
    let mut jobs = vec![];
    for i in 0..number_of_inputs {
        let my_rand_str = generate_random_string();
        jobs.push(my_rand_str);
    }
    return jobs;
}

fn main() {
    let mut my_jobs = generate_jobs(30);

    println!("my_jobs : {:?}", my_jobs);

    my_jobs.into_par_iter()
        .map(compute_job)
        .for_each(process_result);

}
```

Output

```bash
my_jobs : ["uJBjhL1FK", "EaslDWXWi", "FcPD4fVGY", "6GAc15IGM", "rIRcrfKlj", "uUKBVMrgU", "iRT26258z", "l59Njh4KC", "kL2IHssph", "DPInKHZGv", "nFOYEQ1Y5", "5Sq8tGH20", "a9V7bQEsq", "zIJAPrhEH", "AqfuQBxk5", "xujodgTYr", "4kZ4vA7TY", "evHkQgOqa", "WTYfmOuGM", "E6xBM9HQm", "q4nkmA1eQ", "r1FlON9Z0", "MG3zr3944", "wY3wQHs6g", "QsfTKOpuD", "NKo5QXI9n", "CE8PeKKdE", "tEvVP6INb", "em5Es3BXx", "4D5hzuMi0"]
sha256 : 478b13673b38d95669ec20758168f85fc8eab62a6269af57820441c5541c7f81
sha256 : 1a6a99cc6a319ee9ae418db8ecc1c8d68e861111e9b2bbbb9ed5c204b9834656
sha256 : f7fa1ad6bd2ce721a015777b2c89ca248b3763d2f755e787bd88cf23e447a0a8
sha256 : b3df070dc24dde5fb45958ebabdf6c801ff47d093d40d120d91340c14a69ded2
sha256 : 0c9b3cf09b1896c06cc598af7d466cec701b50dbc1e2c79f2e24c9e2e8e1f23b
sha256 : e6913feb6658dc987af882376412d0d4f4eb141a8f5c5142784f83c2d94c55e4
sha256 : dd9f0b3a156f4fbe66b2896ce233b9dd51832cd3f2620693d7be0ab6b335149d
sha256 : 0e31478dfc7b6deecc067ae0b966256f6149257316b7c0c9f39d9cff7a0b32e5
sha256 : c3887f280f8912bad998b4b26ab9d34ccf50b9eaead44eb898b25df566f6562e
sha256 : c3c2690850b758e4ed8d92295cde689ad2a6b44535d66a6d0300ba4a2ce2868b
sha256 : 6eddaf7b5e882bcb064f93027b6b672f7b7af982edd2818c92ba671df5be39f8
sha256 : 09f093bfe7acc2ed1b25517436f0cbf5a559d39c85a673421bfa3d94d3844e40
sha256 : db535aad5fccb36f6eeea8371ad8f5943c6f731e9eb60d0aa61c801b526dda1f
sha256 : 6fac7a6e65e03e905faf68d24d7c8d087131293149a92dfe18aacfcdc223eb5d
sha256 : 6a151da389c8dce46431052a25f65c2af9429488e166b3366bf27bcfc6f17ace
sha256 : 3a5ee44e6565c0c9203946b6a0f8c9411b2015ecdb7ee5b91503ef2d900f2796
sha256 : 49c217555674ba87a183877735440e2053b5dc0b61ce25aeafe33260297bed07
sha256 : 47da2dfc12cd77039c5e632a0b5a0ef57fcb6cbf10a8894a1e89819aef742714
sha256 : e8b7b1817c454adee627ffc127a75efdd03adf3a67cdad179215aeed1d74b538
sha256 : 9d21a645f4544205150323bdc5175c1bf870b3c7005d2904035928ad128419dd
sha256 : 5f86acf3e41f228a05c8aa4c18328beb3a12cbddebb2bbde7ebb87f493b87559
sha256 : ba8ad379772aeb150d476980e646f811e5dba69efa0d7e21522df732fca4ff59
sha256 : 89f6fc8b96aa12822e180c98504256468d2df020b2d48f54db2b65847047dd66
sha256 : b0cf3e1d355fc02131d07ac098e3795df2eaa29b02354f1ec7586ffb62cada68
sha256 : c25e181f50c4eb00b8c141131842bfdd720447eadf0f49813182ed3166d25564
sha256 : c4ef565821c862ca84ab0b3bcddd5598b6089f19ba5d89e89d16c0ca76bbe18d
sha256 : 0d3cda1809c2320052c8bb11901007501384530325d6795a75c16ca0efc3cc08
sha256 : 482e97d2f29eea78231442d622d0a97901000c083101c9760229065f1a5dcbca
sha256 : 608a3da14c19db267d24514b80b432a5cdf8c28daad94bcbbbca460d9d743d93
sha256 : c3b734502da38a5cf47abd565208e44b0b2903f5cdd817f0393bcbf2afb8c765
```

Worker Pool : Returning Tuple

The below program will calculate sha256 of a given string & return both sha256 & epoch time at which sha256 was calculated.

Also, we are collecting the results (output) of worker pool jobs using `.collect()` : The results are in the same order in which inputs were sent.

```rust
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::time::Duration;
use rand::{distributions::Alphanumeric};
use sha256::digest;
use std::time::{SystemTime, UNIX_EPOCH};

fn get_epoch_ms() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}

fn compute_job(my_input: String) -> (String, f64) {
    let mut range = rand::thread_rng();
    let random_sleep_secs = range.gen_range(1.0..3.0);
    std::thread::sleep(Duration::from_secs_f64(random_sleep_secs));
    let sha_256_value = digest(my_input.as_str()); // this will calcuate sha256 of given input (my_input)
    let epoch_time = get_epoch_ms();
    println!("compute_job() : my_input : {} | sha_256_value : {} , epoch_time : {}", my_input, sha_256_value, epoch_time);
    return (sha_256_value, epoch_time)
}

fn generate_random_string() -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(9)
        .map(char::from)
        .collect();
    return s;
}

fn generate_jobs(number_of_inputs: i32) -> Vec<String> {
    let mut jobs = vec![];
    for i in 0..number_of_inputs {
        let my_rand_str = generate_random_string();
        jobs.push(my_rand_str);
    }
    return jobs;
}

fn main() {
    let mut my_jobs = generate_jobs(30);

    println!("my_jobs : {:?}", my_jobs);

    let results: Vec<(String, f64)> = my_jobs.clone().into_par_iter()
        .map(compute_job)
        .collect();

    println!("results : {:#?}", results);
}
```

Output

```bash
my_jobs : ["zJ91dbOAv", "6rr5Mb1Gv", "XYJCnwVsM", "ffg5KfRVe", "1V6Ch08zs", "nKhTvgrqJ", "CLz0oFYDb", "aubjor5Gg", "uICIgQ75z", "u83dW6PUr", "t61FtzRN9", "PJ6pIJV44", "OmB2WMYu6", "VeqyQKtQD", "uFF7lvidh", "cG7YOYwS1", "42K4i17vv", "oOtA9Tzt4", "sCvBrvksx", "LOQ6fLMV5", "ZQyeVn7yT", "mnEsMZ3A0", "pMDkrLOan", "C8AD9VMbN", "0AuB45Lbd", "u7Rx3Jp1B", "YcgOTvecX", "ipLbLHEzv", "JB14msrMr", "3ThaCH3G4"]

compute_job() : my_input : XYJCnwVsM | sha_256_value : d77b3ae212445533b49720b8b3658d336d41524e955cadb266ad576dbfe9ff2e , epoch_time : 1673747079.365396
compute_job() : my_input : pMDkrLOan | sha_256_value : 88225bb8b16912775bfd3381a1b62b7044d04c5a7a0a2205dfc16beb33945c46 , epoch_time : 1673747079.919968
compute_job() : my_input : PJ6pIJV44 | sha_256_value : d317d1a79d4b347cad4478f264b01df9db2d250037c7b65bbd4dd67c393cdded , epoch_time : 1673747079.960817
compute_job() : my_input : cG7YOYwS1 | sha_256_value : 4c6d92e921b27d314d6da9bace68580db7ae36ce6a070c5d5e6a0626dba9c94f , epoch_time : 1673747079.993854
compute_job() : my_input : sCvBrvksx | sha_256_value : 1a3de567c5fe3486d493602ef235c972bde8c83b2bae670d79f48163ff4438f2 , epoch_time : 1673747080.059119
compute_job() : my_input : YcgOTvecX | sha_256_value : 7b3952cf2718c29ff6df6641f91bd2c6fa2198fc6a0535881525b0b4357043ea , epoch_time : 1673747080.112505
compute_job() : my_input : aubjor5Gg | sha_256_value : e501d8279162bfe05419a05cfd11df42e6965d4595458da0820a220ffac7fcbe , epoch_time : 1673747080.294094
compute_job() : my_input : ffg5KfRVe | sha_256_value : 0cb1ae96bc82ad00f5e765f54b6fec5c0d593ff10f77ecdabe48781a0419821d , epoch_time : 1673747080.39971
compute_job() : my_input : 42K4i17vv | sha_256_value : 1502a11cbac9a986e6b56c8e92821b2fca2e55f0beafe9e38b524488f9d7c482 , epoch_time : 1673747080.696017
compute_job() : my_input : 6rr5Mb1Gv | sha_256_value : 6fe58d7dd2d6e75cc8acbf4881a36b3f9e9547aac9a5fcbdc816f4bd0670461f , epoch_time : 1673747080.907202
compute_job() : my_input : zJ91dbOAv | sha_256_value : 4732d8d51dff3218810bad5cb05e5ee750058e893d243dcd308720f74943f122 , epoch_time : 1673747081.079455
compute_job() : my_input : C8AD9VMbN | sha_256_value : 54fe54da4caccd7011836f315e751e9d5c536ade924910169f1f6da4ee5690dc , epoch_time : 1673747081.309109
compute_job() : my_input : LOQ6fLMV5 | sha_256_value : 14f72f3cde1fe77d1e8f30bd151b4440d71469f73b1c9c22257b0bfac0201819 , epoch_time : 1673747081.496496
compute_job() : my_input : OmB2WMYu6 | sha_256_value : 3bc6672d451e14e937b438aefbb64f1c2c84d9bbd7d21ead9dbb3ba139f7afef , epoch_time : 1673747082.02419
compute_job() : my_input : VeqyQKtQD | sha_256_value : c4b3ee13fe7b12a3cb722ae4d2ba324bb13ee167dbbb126deb25b72503fcf9c8 , epoch_time : 1673747082.144759
compute_job() : my_input : ipLbLHEzv | sha_256_value : 40e3ae0b94f6629ede43f2b3ec4b263d09ac59d7defc3e79a781bbf50f269059 , epoch_time : 1673747082.300534
compute_job() : my_input : uICIgQ75z | sha_256_value : 8317929f6a9f8d04504a837af2e1b9ae429d3be48e5fb8c21f4cc7cd2b4f3788 , epoch_time : 1673747082.457795
compute_job() : my_input : 0AuB45Lbd | sha_256_value : 7872395297bc7ea1223ddae1bcdb3fbceb4d78e954440a425ac8562df537dfae , epoch_time : 1673747082.735495
compute_job() : my_input : oOtA9Tzt4 | sha_256_value : e4c00fed9bbe7a18130c9d7852b322e69b7880b56e7baef7c74a9f8a7261a186 , epoch_time : 1673747082.773733
compute_job() : my_input : u7Rx3Jp1B | sha_256_value : 236ef101d511d0e157b4c19cf9dd1f90b7b454168d93d52c11a9f17dc0592f34 , epoch_time : 1673747083.009317
compute_job() : my_input : 1V6Ch08zs | sha_256_value : 68363ac3c1b2a0e2f0f375ff15b3ef10d8d7ea9d03646a3ffd5ca14505f52092 , epoch_time : 1673747083.234678
compute_job() : my_input : uFF7lvidh | sha_256_value : 96c11616847e17d1dd2f20115184646a07a87d13d615a929f6d0a40add376a19 , epoch_time : 1673747083.290868
compute_job() : my_input : nKhTvgrqJ | sha_256_value : d8fb09d5ca4f1ef12e79db8797d609facc601b3dbd7edc05a97b29855c47545a , epoch_time : 1673747083.926569
compute_job() : my_input : ZQyeVn7yT | sha_256_value : 1c1a008c0c5e6a418d25a0a92c79213646b12a3bff7600e2f5c4b52944e88108 , epoch_time : 1673747084.104349
compute_job() : my_input : CLz0oFYDb | sha_256_value : 215632807dbd00a8ed4f516dd8bc6c63425df67700245994cd64454092da5fa4 , epoch_time : 1673747084.19506
compute_job() : my_input : t61FtzRN9 | sha_256_value : 2d89fdb82a619086f74a70cb863b44ba2b867d3ca50fb0740bacb3f919bb1d73 , epoch_time : 1673747084.234326
compute_job() : my_input : JB14msrMr | sha_256_value : 5de48a3521ad2a2853c913008fdf1678f3a8313e3eef0e34ff2a297db3be7ac5 , epoch_time : 1673747084.405927
compute_job() : my_input : 3ThaCH3G4 | sha_256_value : fcbd2407fdf11afe6861935c6e4af27dac9bc6a41409a6ec11e36a9ff7a26d17 , epoch_time : 1673747085.112416
compute_job() : my_input : u83dW6PUr | sha_256_value : 089bbf796a6d2559432e1064d71ecafdf0286242c3473c1b9cfbb6612ec06494 , epoch_time : 1673747085.153877
compute_job() : my_input : mnEsMZ3A0 | sha_256_value : 944af9d520bb12710b32bcb80dcb84cf741da7755f9081a0d7eed48091169a9b , epoch_time : 1673747085.675663
results : [
    (
        "0888630fd4940cf3de0396d06e3d71f3129d10d27dcb0badea1250fad313dd41",
        1673747272.324538,
    ),
    (
        "d128de919b16f5b0aea9fe08660d84232f34e910c24d92e75cf215f009c92e19",
        1673747271.591321,
    ),
    (
        "9eb5a78fb2aa79935101b57b656340fe392a067925a8e5d07725e235b9f6d398",
        1673747274.572332,
    ),
    (
        "554fbe3489ed57ca4969ed9d532012bb544760392d616126904f67051c261f59",
        1673747271.498116,
    ),
    (
        "06c5a9960da840f45b36950dfec3631c09f5f7165584fe70b7ddfadaedc7f1f0",
        1673747272.860816,
    ),
    (
        "dc41fbc8d09d82694573214722657b06be64b0d0370ae580c16daf18bae3c60a",
        1673747271.416253,
    ),
    (
        "3c0c9d928f631d2b1593645376cf4b9e6937a2090ea6a96900afbae5d5dbcc06",
        1673747272.66621,
    ),
    (
        "0c223d6f1312b3cad997b50c513d04bcf53dd9c3bc382270c4a7bc9e48d39cee",
        1673747272.201742,
    ),
    (
        "42817fee5e19cb2a7d144e24940afb723f6e3dafccc558a62ca984f42cd6ffce",
        1673747274.874443,
    ),
    (
        "3df66ca4d166f4c6fde65cbc5741bc6fe972d0777b22244d1b1041dd6ebf3c9e",
        1673747273.769186,
    ),
    (
        "740d18eb2e03e774e16b28b8370113f68ba374cba91b71bbecd1338803687a72",
        1673747276.487745,
    ),
    (
        "6d12ff07e30394da7915e22ff1851d3da547fc00cb4ac9249ea5fb31a59cdd19",
        1673747273.452667,
    ),
    (
        "483fa44fe13c7b4c79f150ff5fb38142ccf41f229da10db01677d4f8fcc9864f",
        1673747275.640469,
    ),
    (
        "027e76305c164be774d6fd73188b6587221637944a493f72c83cbb86c69b6c35",
        1673747274.047715,
    ),
    (
        "45be3d5dabc0b6ae9e0730e94588a9a5cb37b627b3de0a4137837d15506c1305",
        1673747274.502549,
    ),
    (
        "4d2b8db1e9a278020eb3fede08857a0de21125857d36bf200ec26673b546a2fd",
        1673747272.15114,
    ),
    (
        "6fba11c9c9c83ece689485d7de3185e08422a756892a36b6badcaaec2367739e",
        1673747274.17036,
    ),
    (
        "dbee14487963d9c6e85ddf4f7f4d73e34a13a92422dbd5a652b785ceb80c5939",
        1673747275.296178,
    ),
    (
        "ab66104d8cfcdafd1a73638eef49dc0b26a5214fc86f12d1786fa8d687b01440",
        1673747275.452,
    ),
    (
        "bc25a03a3cad13dda28b8433a082f6ced7e9608a40ac31034a0b2f9cc25d69fd",
        1673747276.396313,
    ),
    (
        "0d96566b16ab61d8f8b9504d62c6e8e501913fc69adc6e8aeec0cd4a523dbd51",
        1673747275.733711,
    ),
    (
        "ae6bd8b3f1d3eadaa290893e0922dee6f57e9e329abcd09fa7612d8ecd453383",
        1673747276.013697,
    ),
    (
        "b829123bbf2ea693804c2ba38b1bfcbf783cdf3a76c4876e1624ba375831f2d0",
        1673747272.654842,
    ),
    (
        "7441298fbdfb12cb114a2297f0cf9950cc8dc755c6dfb209abfd2e4e7fdfec7d",
        1673747274.236779,
    ),
    (
        "7d50522ff378805adfc59431c24793947a11843595962735a4004eab9e403955",
        1673747273.247425,
    ),
    (
        "fbf8ce22831aea43d24b84592722c4bfcc3be08837b82aa29c184036c715ba74",
        1673747274.277669,
    ),
    (
        "c11cc2c1cf37e9dcf0de1acab84b2606bba5fa77bf22dbaca3e3574dd25e97ce",
        1673747272.731439,
    ),
    (
        "7c2ad62657128d6fdc0b35b6c1b20ed91eb4132fc2952c5e679453727cd09c87",
        1673747274.838332,
    ),
    (
        "8314eaed61fee90e3244356314a2c307a28a10eb12e6bc54bf298ad368ca43c1",
        1673747277.11155,
    ),
    (
        "f6dfb67a8028b3f2e08f42ebc9f1932e9e5f6c811623b05a53954713ff7b2549",
        1673747276.435732,
    ),
]
```

Thread Pools : With Fixed Thread Pool Size & `mpsc` | `channels`

FYI : This needs refactoring.

```rust
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::time::Duration;
use rand::{distributions::Alphanumeric};
use sha256::digest;
use std::time::{SystemTime, UNIX_EPOCH};

fn get_epoch_ms() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}

fn compute_job(my_input: String) -> (String, f64) {
    let mut range = rand::thread_rng();
    let random_sleep_secs = range.gen_range(1.0..2.0);
    std::thread::sleep(Duration::from_secs_f64(random_sleep_secs));
    let sha_256_value = digest(my_input.as_str()); // this will calcuate sha256 of given input (my_input)
    let epoch_time = get_epoch_ms();
    println!("compute_job() : my_input : {} | sha_256_value : {} , epoch_time : {}", my_input, sha_256_value, epoch_time);
    return (sha_256_value, epoch_time)
}

fn generate_random_string() -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(9)
        .map(char::from)
        .collect();
    return s;
}

fn generate_jobs(number_of_inputs: i32) -> Vec<String> {
    let mut jobs = vec![];
    for i in 0..number_of_inputs {
        let my_rand_str = generate_random_string();
        jobs.push(my_rand_str);
    }
    return jobs;
}

fn main() {

    let pool = rayon::ThreadPoolBuilder::new()
    .num_threads(4)
    .build()
    .unwrap();

    let (tx, rx) = std::sync::mpsc::channel();

    let mut my_jobs = generate_jobs(18);

    for job in my_jobs {
        let tx = tx.clone();
        pool.spawn(move || {
           tx.send(compute_job(job)).unwrap();
        });
    }
    drop(tx); // close all senders
    let results: Vec<(String, f64)> = rx.into_iter().collect(); // ... this would block
    println!("results : {:?}", results);
}
```

Output

```bash
compute_job() : my_input : w6MXXCY5G | sha_256_value : 63cfb36a203fa0483b8e34a1967dcd13b7d60f1f39904d84a23e914587cc45d8 , epoch_time : 1673821750.955093
compute_job() : my_input : 4xYe0sBZt | sha_256_value : 0b814e0fe547506e2ce19d28178f15068eb549c98eed47809795ba0bf4836da8 , epoch_time : 1673821751.085114
compute_job() : my_input : PUwW0yCkK | sha_256_value : 8a034040e2ef749ee815cec6cc3dcb0ee41f3b0961b1cd29b8c6ea4ff2fb6ffc , epoch_time : 1673821751.32368
compute_job() : my_input : BqPdHtDUN | sha_256_value : c08b279b2ffadeab7a5bcd2bf3be08210d31a18b98325801381d6cbb3cee1fad , epoch_time : 1673821751.684953
compute_job() : my_input : KgiaSir6W | sha_256_value : c3e75358acdf293b3719fea7ca750229d9d4ea2671a7d259b73e63b39d9644db , epoch_time : 1673821752.629222
compute_job() : my_input : Vtu3DZxuh | sha_256_value : 03d56649b196e3bfcf1a75c75310d907dfb11229a835d80e04d20fd13a0d9e11 , epoch_time : 1673821752.721017
compute_job() : my_input : fS5BNcUL3 | sha_256_value : 893a57e85e28863700910eec965826a742939a15f45b7b1edaca0d17d81d1c0b , epoch_time : 1673821752.834414
compute_job() : my_input : XytMR9v0B | sha_256_value : fff4010cd6a9d8dcd5a5fd3d8e8342d9b842c95af11569d2b4575ea764f2eb97 , epoch_time : 1673821753.449123
compute_job() : my_input : DtwzLggNM | sha_256_value : b174433a034659c33085ed41d6ac8eacf7ba2cfa2fc0d8b769b709362d685726 , epoch_time : 1673821753.755912
compute_job() : my_input : szyqJHMJy | sha_256_value : ff9bb3409379acb8f087d4a11ef6694edd76853ad98642543a756a68275a126c , epoch_time : 1673821754.107252
compute_job() : my_input : Ms93WRzkQ | sha_256_value : b8c807a56fe167d9778af2eff520195c856ab451d6f2d760280ee56996ff80d6 , epoch_time : 1673821754.310042
compute_job() : my_input : WeFAI3Ocs | sha_256_value : 821618243fc4e71de39c91bcc123ead15c2bbc5dabe581c77b98dd21c8c029f7 , epoch_time : 1673821755.184058
compute_job() : my_input : CupPUfMhB | sha_256_value : 4f1b31224b342651c7f447dd63b031e6dda355df0f4a551b46f5635c29b31a48 , epoch_time : 1673821755.356287
compute_job() : my_input : 1ei4MHii0 | sha_256_value : 8d8e601c86f2d4be9b1edb0fc347ad0c937d2537b7e30f7b5b16fa0d061a5067 , epoch_time : 1673821755.427253
compute_job() : my_input : M9vA1g0cL | sha_256_value : 79c43fdd8039c03586901b61ca422b8feeffb1d878ff14037836b84035161d87 , epoch_time : 1673821756.023228
compute_job() : my_input : ef61OtdEx | sha_256_value : 413dc63eb69e8c584faf4c2a27fc6a8dab30ce5835838a53a73c0ffc0421d637 , epoch_time : 1673821756.593555
compute_job() : my_input : RmNYAMuyF | sha_256_value : de777de9530c981a757bf5eece673c4532988de190a939de15ad0af3687961ce , epoch_time : 1673821756.80075
compute_job() : my_input : MpEe8bHJv | sha_256_value : f82a94c1ee3973cd8c709bb6d6941458287d7698da0cb5bdd6bfe3ce2ba52205 , epoch_time : 1673821756.960486
results : [("63cfb36a203fa0483b8e34a1967dcd13b7d60f1f39904d84a23e914587cc45d8", 1673821750.955093), ("0b814e0fe547506e2ce19d28178f15068eb549c98eed47809795ba0bf4836da8", 1673821751.085114), ("8a034040e2ef749ee815cec6cc3dcb0ee41f3b0961b1cd29b8c6ea4ff2fb6ffc", 1673821751.32368), ("c08b279b2ffadeab7a5bcd2bf3be08210d31a18b98325801381d6cbb3cee1fad", 1673821751.684953), ("c3e75358acdf293b3719fea7ca750229d9d4ea2671a7d259b73e63b39d9644db", 1673821752.629222), ("03d56649b196e3bfcf1a75c75310d907dfb11229a835d80e04d20fd13a0d9e11", 1673821752.721017), ("893a57e85e28863700910eec965826a742939a15f45b7b1edaca0d17d81d1c0b", 1673821752.834414), ("fff4010cd6a9d8dcd5a5fd3d8e8342d9b842c95af11569d2b4575ea764f2eb97", 1673821753.449123), ("b174433a034659c33085ed41d6ac8eacf7ba2cfa2fc0d8b769b709362d685726", 1673821753.755912), ("ff9bb3409379acb8f087d4a11ef6694edd76853ad98642543a756a68275a126c", 1673821754.107252), ("b8c807a56fe167d9778af2eff520195c856ab451d6f2d760280ee56996ff80d6", 1673821754.310042), ("821618243fc4e71de39c91bcc123ead15c2bbc5dabe581c77b98dd21c8c029f7", 1673821755.184058), ("4f1b31224b342651c7f447dd63b031e6dda355df0f4a551b46f5635c29b31a48", 1673821755.356287), ("8d8e601c86f2d4be9b1edb0fc347ad0c937d2537b7e30f7b5b16fa0d061a5067", 1673821755.427253), ("79c43fdd8039c03586901b61ca422b8feeffb1d878ff14037836b84035161d87", 1673821756.023228), ("413dc63eb69e8c584faf4c2a27fc6a8dab30ce5835838a53a73c0ffc0421d637", 1673821756.593555), ("de777de9530c981a757bf5eece673c4532988de190a939de15ad0af3687961ce", 1673821756.80075), ("f82a94c1ee3973cd8c709bb6d6941458287d7698da0cb5bdd6bfe3ce2ba52205", 1673821756.960486)]
```

Multiple Thread Pools : With Fixed Thread Pool Size & `mpsc` | `channels`

FYI : This needs refactoring.

`Cargo.toml`

```toml
[package]
name = "worker_pool"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rand = "0.8.5"
rayon = "1.6.1"
sha256 = "1.1.1"
```

`main.rs`

```rust
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::time::Duration;
use rand::{distributions::Alphanumeric};
use sha256::digest;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use std::thread;
use rayon::ThreadPool;

fn get_epoch_ms() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}

fn compute_job(thread_name: String, my_input: String) -> (String, f64) {
    let mut range = rand::thread_rng();
    let random_sleep_secs = range.gen_range(1.0..2.0);
    std::thread::sleep(Duration::from_secs_f64(random_sleep_secs));
    let sha_256_value = digest(my_input.clone()); // this will calcuate sha256 of given input (my_input)
    let epoch_time = get_epoch_ms();
    println!("compute_job() : thread_name : {} , my_input : {:?} | sha_256_value : {} , epoch_time : {}", thread_name, my_input, sha_256_value, epoch_time);
    return (sha_256_value, epoch_time)
}

fn generate_random_string() -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(9)
        .map(char::from)
        .collect();
    return s;
}

fn generate_jobs(number_of_inputs: i32) -> Vec<String> {
    let mut jobs = vec![];
    for i in 0..number_of_inputs {
        let my_rand_str = generate_random_string();
        jobs.push(my_rand_str);
    }
    return jobs;
}

fn main() {

    let mut my_jobs1 = generate_jobs(12);
    let mut my_jobs2 = generate_jobs(18);

    let pool1 = rayon::ThreadPoolBuilder::new()
        .num_threads(3)
        .build()
        .unwrap();

    let pool2 = rayon::ThreadPoolBuilder::new()
        .num_threads(6)
        .build()
        .unwrap();

    let mut jobs_vec = vec![];

    jobs_vec.push(thread::spawn(move || {
        return execute_job(String::from("thread_1"), my_jobs1, pool1);
    }));

    jobs_vec.push(thread::spawn(move || {
        return execute_job(String::from("thread_2"), my_jobs2, pool2);
    }));

    let mut final_results = vec![];
    for my_thread in jobs_vec {
        let r = my_thread.join().unwrap();
        final_results.push(r);
    }
    println!("\nfinal_results : {:?}", final_results);

}

fn execute_job(thread_name: String, input_jobs: Vec<String>, thread_pool: ThreadPool) -> Vec<(String, f64)> {
    println!("execute_job()...");
    let (tx, rx) = std::sync::mpsc::channel();
    for input_job in input_jobs {
        let tx = tx.clone();
        let thread_name = thread_name.clone();
        thread_pool.spawn(move || {
            tx.send(compute_job(thread_name, input_job)).unwrap();
        });
    }
    drop(tx); // close all senders
    let results: Vec<(String, f64)> = rx.into_iter().collect(); // ... this would block
    println!("\nresults : {:?}", results);
    return results;
}
```

Output

```bash
execute_job()...
execute_job()...
compute_job() : thread_name : thread_1 , my_input : "kXfJQKQmH" | sha_256_value : f25afb33e9276de6ebce2f674188a436001e2ae74418d4c116fe152080ae1eb5 , epoch_time : 1673842721.365629
compute_job() : thread_name : thread_2 , my_input : "aAVJypBog" | sha_256_value : e5d980a3c60e90d979d603b8b2a6dba9a10474a5d0fbb968f85220b48844c74f , epoch_time : 1673842721.371302
compute_job() : thread_name : thread_2 , my_input : "3iTFSR1l4" | sha_256_value : 31d1b41225fca0ede031667ddfe1cc1660d3577909bd69d9ad7d733b0a424406 , epoch_time : 1673842721.501707
compute_job() : thread_name : thread_2 , my_input : "blq3amlun" | sha_256_value : 57e5929d3597161839fb4f01807747ccb30244eef621e75e3a39b62ce7603636 , epoch_time : 1673842721.648848
compute_job() : thread_name : thread_1 , my_input : "PPABsySqL" | sha_256_value : 69679b87bc85902fc5522f36f0f5df1357a3b57d5c5fa889ad3981b2cdd779cd , epoch_time : 1673842721.737256
compute_job() : thread_name : thread_1 , my_input : "rGPEOCiTt" | sha_256_value : 9749387dfe52ee588d1677c786ebbd87a26e7c34f0e73eba7ffaf013cec96d94 , epoch_time : 1673842721.818902
compute_job() : thread_name : thread_2 , my_input : "hFOGeC7kr" | sha_256_value : 6c87b0a15880ddeaa4c86c12a70990b1c463067c1c0871170309cc96e53f06a9 , epoch_time : 1673842722.003265
compute_job() : thread_name : thread_2 , my_input : "i1HHJyk8z" | sha_256_value : cacd91b2ace979f0e158b0e41ac51373f98550398f2765fa3af0542a4eee7654 , epoch_time : 1673842722.003497
compute_job() : thread_name : thread_2 , my_input : "ba02FBtpu" | sha_256_value : 45e6094966be2f9504b0f506d22053990f77f40f7dd54707bc8baf6a5a8a5011 , epoch_time : 1673842722.321547
compute_job() : thread_name : thread_2 , my_input : "9hf5wAnFT" | sha_256_value : bbe98a4df0b43da46e24b7cc72e044e562acb51f189754a7306216716559573c , epoch_time : 1673842722.637512
compute_job() : thread_name : thread_2 , my_input : "cLmAb0kIw" | sha_256_value : 0b0a0524216b68bddda144242e7fbf3e6539c0fc1f40cd1049f362ef8c6e9f29 , epoch_time : 1673842722.777773
compute_job() : thread_name : thread_1 , my_input : "HjA7Kr5DD" | sha_256_value : 7646a0c229b4a340541fca4819c52747990b6fcdd64fee9a118b7c5f9ea2fc46 , epoch_time : 1673842722.942442
compute_job() : thread_name : thread_2 , my_input : "6iOB4ScN6" | sha_256_value : e4dbae2e0b2aa268ec0f2a87e59e84a00bc69983664497a106e2b2d6567638a2 , epoch_time : 1673842723.228651
compute_job() : thread_name : thread_1 , my_input : "XgY8DXmVo" | sha_256_value : 953559a71c5635467fc16e92434ee95e946e8a25055a07831f6eecd7d4f3c263 , epoch_time : 1673842723.260944
compute_job() : thread_name : thread_2 , my_input : "9hsk9U7PZ" | sha_256_value : 5b8687be3ff9316419aca2a3b827975d6c0c58258789ffdda21ebbf4547efc96 , epoch_time : 1673842723.317163
compute_job() : thread_name : thread_1 , my_input : "TMq7gJWTL" | sha_256_value : a729a3ed0d4e656c9359d5bc124891068edb68f56c5356ff86007571bd7b2fbb , epoch_time : 1673842723.434574
compute_job() : thread_name : thread_2 , my_input : "c1GfywuMQ" | sha_256_value : 224c97d8fe95375946117757d3c317877d3d3e8581030076f30cf56876eb41fd , epoch_time : 1673842723.64329
compute_job() : thread_name : thread_2 , my_input : "CGceEAfhA" | sha_256_value : aff249d37df04ec2762c3edb2e4f1b8cd0d6d6c63568ef6621653d793265de79 , epoch_time : 1673842723.776862
compute_job() : thread_name : thread_2 , my_input : "b1BcqhhYS" | sha_256_value : cc4929d2b2019e60eeb6277aa29d4aa23da3e4dc468ed8cb37ac993491afa471 , epoch_time : 1673842724.176338
compute_job() : thread_name : thread_2 , my_input : "dOGYIsQUN" | sha_256_value : 5abe34fb3bf16180189f68b844559c9acd4135b8bf0df41ff0bfc28acbe02a7c , epoch_time : 1673842724.278151
compute_job() : thread_name : thread_1 , my_input : "QzBIG6rFW" | sha_256_value : a5307ccebf3254091002bb528e89aed541ed21b7f419d44d311f9d45b3babd2b , epoch_time : 1673842724.382228
compute_job() : thread_name : thread_2 , my_input : "RjMiq0DuL" | sha_256_value : 3795411902b343b9e32a459a292f9c2105e3e5ef8cb8b6f9f16b19510a438326 , epoch_time : 1673842724.619284
compute_job() : thread_name : thread_2 , my_input : "KMehvvMlN" | sha_256_value : f3d5f6e856f04ad1efe4dc77892473f6fb96a2cb2a464959022274de4d8f9d7f , epoch_time : 1673842724.648562
compute_job() : thread_name : thread_1 , my_input : "to6WiEVz8" | sha_256_value : b810c3f004e876dc847ace912e5294b953ce66a3fa40390016fe9d269ceb5a37 , epoch_time : 1673842724.70955
compute_job() : thread_name : thread_1 , my_input : "918oiAZQV" | sha_256_value : 198681c0492a15db84d980d9e4f7948806df2fec4b9eb3ca419055214b36cca2 , epoch_time : 1673842724.864037
compute_job() : thread_name : thread_2 , my_input : "S5Fs0Jl1G" | sha_256_value : 9d1cb4f38342ac786f73d1137e8c8403471f240517f540e645be9ef536a90c19 , epoch_time : 1673842725.348332
compute_job() : thread_name : thread_2 , my_input : "iJ8pTrbrF" | sha_256_value : ec2f240f2029fd7cb3df641f1e2bff79a0e28d4b61a16c856764fae3a5f38f05 , epoch_time : 1673842725.493735

results : [("e5d980a3c60e90d979d603b8b2a6dba9a10474a5d0fbb968f85220b48844c74f", 1673842721.371302), ("31d1b41225fca0ede031667ddfe1cc1660d3577909bd69d9ad7d733b0a424406", 1673842721.501707), ("57e5929d3597161839fb4f01807747ccb30244eef621e75e3a39b62ce7603636", 1673842721.648848), ("6c87b0a15880ddeaa4c86c12a70990b1c463067c1c0871170309cc96e53f06a9", 1673842722.003265), ("cacd91b2ace979f0e158b0e41ac51373f98550398f2765fa3af0542a4eee7654", 1673842722.003497), ("45e6094966be2f9504b0f506d22053990f77f40f7dd54707bc8baf6a5a8a5011", 1673842722.321547), ("bbe98a4df0b43da46e24b7cc72e044e562acb51f189754a7306216716559573c", 1673842722.637512), ("0b0a0524216b68bddda144242e7fbf3e6539c0fc1f40cd1049f362ef8c6e9f29", 1673842722.777773), ("e4dbae2e0b2aa268ec0f2a87e59e84a00bc69983664497a106e2b2d6567638a2", 1673842723.228651), ("5b8687be3ff9316419aca2a3b827975d6c0c58258789ffdda21ebbf4547efc96", 1673842723.317163), ("224c97d8fe95375946117757d3c317877d3d3e8581030076f30cf56876eb41fd", 1673842723.64329), ("aff249d37df04ec2762c3edb2e4f1b8cd0d6d6c63568ef6621653d793265de79", 1673842723.776862), ("cc4929d2b2019e60eeb6277aa29d4aa23da3e4dc468ed8cb37ac993491afa471", 1673842724.176338), ("5abe34fb3bf16180189f68b844559c9acd4135b8bf0df41ff0bfc28acbe02a7c", 1673842724.278151), ("3795411902b343b9e32a459a292f9c2105e3e5ef8cb8b6f9f16b19510a438326", 1673842724.619284), ("f3d5f6e856f04ad1efe4dc77892473f6fb96a2cb2a464959022274de4d8f9d7f", 1673842724.648562), ("9d1cb4f38342ac786f73d1137e8c8403471f240517f540e645be9ef536a90c19", 1673842725.348332), ("ec2f240f2029fd7cb3df641f1e2bff79a0e28d4b61a16c856764fae3a5f38f05", 1673842725.493735)]
compute_job() : thread_name : thread_1 , my_input : "lvQX8pe6m" | sha_256_value : e07e1207d80d970cb7bb4ad5df8afc0087a2d2b3483c8c91b3f6ba836dafb3fe , epoch_time : 1673842726.10374
compute_job() : thread_name : thread_1 , my_input : "Fv6obTVNk" | sha_256_value : 7c21017f339bb50e492ec1976af67a3239e24345af9a53ae46e82c9fc5253a72 , epoch_time : 1673842726.223604
compute_job() : thread_name : thread_1 , my_input : "FIo8B9SiN" | sha_256_value : b67ef75c68ac03da78eb638a85af557db3a94718dcc29c35f4511708062a6bf7 , epoch_time : 1673842726.361256

results : [("f25afb33e9276de6ebce2f674188a436001e2ae74418d4c116fe152080ae1eb5", 1673842721.365629), ("69679b87bc85902fc5522f36f0f5df1357a3b57d5c5fa889ad3981b2cdd779cd", 1673842721.737256), ("9749387dfe52ee588d1677c786ebbd87a26e7c34f0e73eba7ffaf013cec96d94", 1673842721.818902), ("7646a0c229b4a340541fca4819c52747990b6fcdd64fee9a118b7c5f9ea2fc46", 1673842722.942442), ("953559a71c5635467fc16e92434ee95e946e8a25055a07831f6eecd7d4f3c263", 1673842723.260944), ("a729a3ed0d4e656c9359d5bc124891068edb68f56c5356ff86007571bd7b2fbb", 1673842723.434574), ("a5307ccebf3254091002bb528e89aed541ed21b7f419d44d311f9d45b3babd2b", 1673842724.382228), ("b810c3f004e876dc847ace912e5294b953ce66a3fa40390016fe9d269ceb5a37", 1673842724.70955), ("198681c0492a15db84d980d9e4f7948806df2fec4b9eb3ca419055214b36cca2", 1673842724.864037), ("e07e1207d80d970cb7bb4ad5df8afc0087a2d2b3483c8c91b3f6ba836dafb3fe", 1673842726.10374), ("7c21017f339bb50e492ec1976af67a3239e24345af9a53ae46e82c9fc5253a72", 1673842726.223604), ("b67ef75c68ac03da78eb638a85af557db3a94718dcc29c35f4511708062a6bf7", 1673842726.361256)]

final_results : [[("f25afb33e9276de6ebce2f674188a436001e2ae74418d4c116fe152080ae1eb5", 1673842721.365629), ("69679b87bc85902fc5522f36f0f5df1357a3b57d5c5fa889ad3981b2cdd779cd", 1673842721.737256), ("9749387dfe52ee588d1677c786ebbd87a26e7c34f0e73eba7ffaf013cec96d94", 1673842721.818902), ("7646a0c229b4a340541fca4819c52747990b6fcdd64fee9a118b7c5f9ea2fc46", 1673842722.942442), ("953559a71c5635467fc16e92434ee95e946e8a25055a07831f6eecd7d4f3c263", 1673842723.260944), ("a729a3ed0d4e656c9359d5bc124891068edb68f56c5356ff86007571bd7b2fbb", 1673842723.434574), ("a5307ccebf3254091002bb528e89aed541ed21b7f419d44d311f9d45b3babd2b", 1673842724.382228), ("b810c3f004e876dc847ace912e5294b953ce66a3fa40390016fe9d269ceb5a37", 1673842724.70955), ("198681c0492a15db84d980d9e4f7948806df2fec4b9eb3ca419055214b36cca2", 1673842724.864037), ("e07e1207d80d970cb7bb4ad5df8afc0087a2d2b3483c8c91b3f6ba836dafb3fe", 1673842726.10374), ("7c21017f339bb50e492ec1976af67a3239e24345af9a53ae46e82c9fc5253a72", 1673842726.223604), ("b67ef75c68ac03da78eb638a85af557db3a94718dcc29c35f4511708062a6bf7", 1673842726.361256)], [("e5d980a3c60e90d979d603b8b2a6dba9a10474a5d0fbb968f85220b48844c74f", 1673842721.371302), ("31d1b41225fca0ede031667ddfe1cc1660d3577909bd69d9ad7d733b0a424406", 1673842721.501707), ("57e5929d3597161839fb4f01807747ccb30244eef621e75e3a39b62ce7603636", 1673842721.648848), ("6c87b0a15880ddeaa4c86c12a70990b1c463067c1c0871170309cc96e53f06a9", 1673842722.003265), ("cacd91b2ace979f0e158b0e41ac51373f98550398f2765fa3af0542a4eee7654", 1673842722.003497), ("45e6094966be2f9504b0f506d22053990f77f40f7dd54707bc8baf6a5a8a5011", 1673842722.321547), ("bbe98a4df0b43da46e24b7cc72e044e562acb51f189754a7306216716559573c", 1673842722.637512), ("0b0a0524216b68bddda144242e7fbf3e6539c0fc1f40cd1049f362ef8c6e9f29", 1673842722.777773), ("e4dbae2e0b2aa268ec0f2a87e59e84a00bc69983664497a106e2b2d6567638a2", 1673842723.228651), ("5b8687be3ff9316419aca2a3b827975d6c0c58258789ffdda21ebbf4547efc96", 1673842723.317163), ("224c97d8fe95375946117757d3c317877d3d3e8581030076f30cf56876eb41fd", 1673842723.64329), ("aff249d37df04ec2762c3edb2e4f1b8cd0d6d6c63568ef6621653d793265de79", 1673842723.776862), ("cc4929d2b2019e60eeb6277aa29d4aa23da3e4dc468ed8cb37ac993491afa471", 1673842724.176338), ("5abe34fb3bf16180189f68b844559c9acd4135b8bf0df41ff0bfc28acbe02a7c", 1673842724.278151), ("3795411902b343b9e32a459a292f9c2105e3e5ef8cb8b6f9f16b19510a438326", 1673842724.619284), ("f3d5f6e856f04ad1efe4dc77892473f6fb96a2cb2a464959022274de4d8f9d7f", 1673842724.648562), ("9d1cb4f38342ac786f73d1137e8c8403471f240517f540e645be9ef536a90c19", 1673842725.348332), ("ec2f240f2029fd7cb3df641f1e2bff79a0e28d4b61a16c856764fae3a5f38f05", 1673842725.493735)]]
```