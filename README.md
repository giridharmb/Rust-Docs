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