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

We can slightly modify the error handling this way and avoid program crashing via panic

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