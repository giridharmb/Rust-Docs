#### Notes

Rust Playground

https://play.rust-lang.org/

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

### First Things First :)

For getting Rust standard library documentation, run `rustup doc`

Generating/Viewing Documentation For Your Own Code

- Use `///` To write documentation for the code blocks
- To build and open documentation, run `cargo doc --open`

Try generating the documentation for the below code

```rust
/// This a gropcery item struct, having name (String), quantity (number of items) and price (price of the item)
struct GroceryItem {
    name: String,
    quantity: i32,
    price: f64,
}

/// This function returns an Option which is actually a tuple (quantity & price)
fn find_quantity(grocery_name: &str) -> Option<(i32, f64)> {
    let groceries = vec![
        GroceryItem { name: "bananas".to_owned() , quantity: 4 , price: 3.5 },
        GroceryItem { name: "eggs".to_owned() , quantity: 12 , price : 2.0 },
        GroceryItem { name: "bread".to_owned() , quantity: 1 , price : 1.5 },
    ];

    for item in groceries {
        if item.name == grocery_name {
            return Some((item.quantity, item.price))
        }
    }
    /// If there is no item with the specified name, a (None) is returned
    return None;
}

/// calcualte function will accept an item name as input and then find out the quantity
/// And price of each that item, return the result as (quantity * price)
fn calculate(item_name: &str) -> f64 {
    return match find_quantity(item_name) {
        None => {
            println!("quantity for {:?} : missing", item_name);
            0.0
        }
        Some(d) => {
            println!("quantity for {:?} : {:?}", item_name, d);
            let (quantity, price) = d;
            quantity as f64 * price
        }
    };

}

/// Entry point of the program
/// This will query couple of items where one item is present and another one which is not present
/// And then print out the result returned from calculate function
fn main() {
    let item_data_1 = calculate("bananas");
    println!("item_data_1 : {}", item_data_1);

    let item_data_2 = calculate("carrots");
    println!("item_data_2 : {}", item_data_2);

}
```

After you run `cargo doc --open` , the documentation will open up on the browser.

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

`Result` Type : Is of type `enum`

- `Result` : A data type that contains one of two types of data
    - "`successful`" data
    - "`error`" data

- `Result` represents `success` or `failure`
- `Ok(variable_name)` : operation was completed
- `Err(variable_name)` : operation failed
- Useful when working with functionality that can potentially fail
- Use `Results<T,E>` when working with results

```rust
fn get_sound(name: &str) -> Result<SoundData, String> {
    if name == "alert" {
        Ok(SoundData::new("alert"))
    } else {
        Err("unable to find the sound data".to_owned())
    }
}

fn main() {
    let sound = get_sound("alert");
    match sound {
        Ok(_) => println!("sound data located !"),
        Err(e) => println!("error : {:?}", e),
    }
}
```

Another usage of `Result` & `Err`

```rust
#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "main_menu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("menu choice is not valid".to_owned()),
    }
}

fn print_choice(choice: &Result<MenuChoice, String>) {
    println!("choice : {:?}", choice)
}

fn main() {
    let choice1 = get_choice("main_menu");
    print_choice(&choice1);

    let choice2 = get_choice("start");
    print_choice(&choice2);

    let choice3 = get_choice("whats_this");
    print_choice(&choice3);
}
```

Output

```bash
choice : Ok(MainMenu)
choice : Ok(Start)
choice : Err("menu choice is not valid")
```

Another usage of `Result` & `Err`

```rust
#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "main_menu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("menu choice is not valid".to_owned()),
    }
}

fn print_choice(choice: &Result<MenuChoice, String>) {
    println!("choice : {:?}", choice)
}

fn print_choice_v2(choice: &MenuChoice) {
    println!("your choice is : {:?}", choice)
}

fn main() {
    let choice1 = get_choice("main_menu");
    print_choice(&choice1);

    let choice2 = get_choice("start");
    print_choice(&choice2);

    let choice3 = get_choice("whats_this");
    print_choice(&choice3);

    match choice1 {
        Ok(d) => {
            print_choice_v2(&d);
        }
        Err(e) => {
            println!("error : {:?}", e);
        }
    }

    match choice2 {
        Ok(d) => {
            print_choice_v2(&d);
        }
        Err(e) => {
            println!("error : {:?}", e);
        }
    }

    match choice3 {
        Ok(d) => {
            print_choice_v2(&d);
        }
        Err(e) => {
            println!("error : {:?}", e);
        }
    }
}
```

Output

```bash
choice : Ok(MainMenu)
choice : Ok(Start)
choice : Err("menu choice is not valid")
your choice is : MainMenu
your choice is : Start
error : "menu choice is not valid"
```

Another usage of `Result` & `Err`

Check the function `pick_choice(input: &str) -> Result<(), String>` below , which uses `?`

```rust
#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "main_menu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("menu choice is not valid".to_owned()),
    }
}

fn print_choice(choice: &Result<MenuChoice, String>) {
    println!("choice : {:?}", choice)
}

fn print_choice_v2(choice: &MenuChoice) {
    println!("your choice is : {:?}", choice)
}

// this function returns Result<(),String>
// () : is a unit type, which means "Nothing"
// and in case of Error, a String will be returned
// Since return type is of type Result, the signature is Result<T,E>
// That is why we are setting the Ok() data type to be ()
// And Error to be of type String
fn pick_choice(input: &str) -> Result<(), String> {
    let choice = get_choice(input)?;
    print_choice_v2(&choice);
    Ok(())
}

fn main() {
    let choice = pick_choice("main_menu");
    println!("result of your choice : {:?}", choice);

    let choice = pick_choice("start");
    println!("result of your choice : {:?}", choice);

    let choice = pick_choice("whats_this");
    println!("result of your choice : {:?}", choice);
}
```

Output

```bash
your choice is : MainMenu
result of your choice : Ok(())
your choice is : Start
result of your choice : Ok(())
result of your choice : Err("menu choice is not valid")
```

Another usage of `Result` & `Err`

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

#### Result Type and ? Operator

`?` operator (below) : This expression is evaluated : `try_building_access(employee)?`

If try_building_access(employee) returns an error
    Then String is returned with error message
Otherwise
    If it was successful , variable 'attempt_access' will have a valid value

If you are using a `?` operator, your function must return a `Result<T,E>`

```rust
enum Position {
    Maintenance,
    Marketing,
    Manager,
    LineSupervisor,
    KitchenStaff,
    AssemblyTech,
}

enum Status {
    Active,
    Terminated,
}

struct Employee {
    position: Position,
    status: Status,
}

fn try_building_access(employee: &Employee) -> Result<(), String> {
    match employee.status {
        Status::Terminated => {
            return Err("employee terminated".to_owned())
        },
        _ => (),
    }

    match employee.position {
        Position::Maintenance => {
            Ok(())
        },
        Position::Marketing => {
            Ok(())
        }
        Position::Manager => {
            Ok(())
        }
        // --- these positions will not have access to the building ---
        // Position::LineSupervisor => {}
        // Position::KitchenStaff => {}
        // Position::AssemblyTech => {}
        _ => Err("invalid position for building access".to_owned())
    }
}

fn print_access(employee: &Employee) -> Result<(), String> {
    /*
    '?' operator : this expression is evaluated : try_building_access(employee)?
    If try_building_access(employee) returns an error
        -> Then String is returned with error message
    Otherwise
        -> If it was successful , variable 'attempt_access' will have a valid value

    If you are using a '?' operator, your function must return a Result<T,E>
    */

    let attempt_access = try_building_access(employee)?;
    println!("access ok");
    Ok(())
}

fn main() {
    let emp_1 = Employee {
        position: Position::Manager,
        status: Status::Active,
    };

    let emp_2 = Employee {
        position: Position::AssemblyTech,
        status: Status::Active,
    };

    let emp_3 = Employee {
        position: Position::LineSupervisor,
        status: Status::Terminated,
    };

    let emp_4 = Employee {
        position: Position::Marketing,
        status: Status::Terminated,
    };

    let emp_5 = Employee {
        position: Position::Manager,
        status: Status::Terminated,
    };

    //----------------------------------------------------------------

    match print_access(&emp_1) {
        Err(e) => {
            println!("emp_1 : access denied : {:?}", e)
        },
        _ => (),
    }

    match print_access(&emp_2) {
        Err(e) => {
            println!("emp_2 : access denied : {:?}", e)
        },
        _ => (),
    }

    match print_access(&emp_3) {
        Err(e) => {
            println!("emp_3 : access denied : {:?}", e)
        },
        _ => (),
    }

    match print_access(&emp_4) {
        Err(e) => {
            println!("emp_4 : access denied : {:?}", e)
        },
        _ => (),
    }

    match print_access(&emp_5) {
        Err(e) => {
            println!("emp_5 : access denied : {:?}", e)
        },
        _ => (),
    }

    //----------------------------------------------------------------
}
```

Output

```bash
access ok
emp_2 : access denied : "invalid position for building access"
emp_3 : access denied : "employee terminated"
emp_4 : access denied : "employee terminated"
emp_5 : access denied : "employee terminated"
```

Result -> enum : it represents success or failure

```rust
enum Result<T, E>
{
    Ok(T),
    Err(E),
}
```

#### Replacing Characters In A String

```rust
let s:String = "Hello, world!".chars()
    .map(|x| match x { 
        '!' => '?', 
        'A'..='Z' => 'X', 
        'a'..='z' => 'x',
        _ => x
    }).collect();
println!("{}", s);// Xxxxx, xxxxx?
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

- Option represents `Some(..)` data or `None` (which is nothing)
- Useful when working with optional data (ex: form fields)
- Use `Option<type>` to declare an optional type

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

#### Date Time

```rust
use chrono::prelude::*;

fn main() {
    let now = Utc::now();
    let ts: i64 = now.timestamp();
    
    println!("Current timestamp is: {}", ts);
    
    let nt = NaiveDateTime::from_timestamp(ts, 0);
    let dt: DateTime<Utc> = DateTime::from_utc(nt, Utc);
    let res = dt.format("%Y-%m-%d %H:%M:%S");
    
    println!("Date time: {}", res);
}
```

Output

```bash
Current timestamp is: 1675811880
Date time: 2023-02-07 23:18:00
```

#### Option Example

```rust
struct Customer {
    age: Option<i32>,
    email: String,
}

fn main() {
    let mark = Customer {
        age: Some(55),
        email: "mark@company1.com".to_owned(),
    };

    let becky = Customer {
        age: None,
        email: "becky@company2.com".to_owned(),
    };

    match mark.age {
        Some(d) => {
            println!("customer is {:?} years old", d);
        },
        None => {
            println!("customer age is not provided");
        }
    }

    match becky.age {
        Some(d) => {
            println!("customer is {:?} years old", d);
        },
        None => {
            println!("customer age is not provided");
        }
    }
}
```

Output

```bash
customer is 55 years old
customer age is not provided
```

Another Usage of `Option`

```rust
struct GroceryItem {
    name: String,
    quantity: i32,
}

fn find_quantity(grocery_name: &str) -> Option<i32> {
    let groceries = vec![
        GroceryItem { name: "bananas".to_owned() , quantity: 4 },
        GroceryItem { name: "eggs".to_owned() , quantity: 12 },
        GroceryItem { name: "bread".to_owned() , quantity: 1 },
    ];

    for item in groceries {
        if item.name == grocery_name {
            return Some(item.quantity)
        }
    }
    // if name matches nothing, return None
    return None;
}

fn main() {
    let item_1 = match find_quantity("bananas") {
        None => {
            println!("quantity for bananas : missing");
        }
        Some(d) => {
            println!("quantity for bananas : {:?}", d);
        }
    };


    let item_2 = match find_quantity("carrots") {
        None => {
            println!("quantity for carrots : missing");
        }
        Some(d) => {
            println!("quantity for carrots : {:?}", d);
        }
    };
}
```

Output

```bash
quantity for bananas : 4
quantity for carrots : missing
```

Slightly Expanding Above Option Example

```rust
struct GroceryItem {
    name: String,
    quantity: i32,
    price: f64,
}

// this function returns quantity & price
fn find_quantity(grocery_name: &str) -> Option<(i32, f64)> {
    let groceries = vec![
        GroceryItem { name: "bananas".to_owned() , quantity: 4 , price: 3.5 },
        GroceryItem { name: "eggs".to_owned() , quantity: 12 , price : 2.0 },
        GroceryItem { name: "bread".to_owned() , quantity: 1 , price : 1.5 },
    ];

    for item in groceries {
        if item.name == grocery_name {
            return Some((item.quantity, item.price))
        }
    }
    // if name matches nothing, return None
    return None;
}

fn calculate(item_name: &str) -> f64 {
    return match find_quantity(item_name) {
        None => {
            println!("quantity for {:?} : missing", item_name);
            0.0
        }
        Some(d) => {
            println!("quantity for {:?} : {:?}", item_name, d);
            let (quantity, price) = d;
            quantity as f64 * price
        }
    };

}
fn main() {
    let item_data_1 = calculate("bananas");
    println!("item_data_1 : {}", item_data_1);

    let item_data_2 = calculate("carrots");
    println!("item_data_2 : {}", item_data_2);

}
```

Output

```bash
quantity for "bananas" : (4, 3.5)
item_data_1 : 14
quantity for "carrots" : missing
item_data_2 : 0
```

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

There are two main types of thread in programming

- 1: `Operating System Thread`
- 2: `Green Threads` (abstraction which sits on top of OS thread)
- Golang for example uses Green Threads (which allows lot more threads to be created )
- Rust uses OS thread (for the sake of lower runtime, that is lower amount of code in each binary)
- Green Threads can share data memory amongst themselves directly (although synchronization is required of course)

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

Another example of advanced match

```rust
enum Ticket {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),
    Free(f64, String),
}

fn main() {
    let mut tickets = vec![];
    tickets.push(Ticket::Backstage(50.0, "Billy".to_owned()));
    tickets.push(Ticket::Standard(15.0));
    tickets.push(Ticket::Vip(30.0, "Amy".to_owned()));
    tickets.push(Ticket::Free(0.0, "Billy".to_owned()));

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(price, holder) => {
                println!("Backstage : Holder : {:?} , Price {:?}", holder, price);
            },
            Ticket::Standard(price) => {
                println!("Standard : Price {:?}", price);
            },
            Ticket::Vip(price, holder) => {
                println!("Vip : Holder : {:?} , Price {:?}", holder, price);
            },
            _ => {
                println!("Unknown Ticket Type");
            }
        }
    }
}
```

Output

```bash
Backstage : Holder : "Billy" , Price 50.0
Standard : Price 15.0
Vip : Holder : "Amy" , Price 30.0
Unknown Ticket Type
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
    ...
    ...
    <TRUNCATED>
    ...
    ...
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

#### Pasring JSON


```rust
use serde_json::json;

fn main() {
    let glenn = json!({
        "name": "Glenn Gillen",
        "age": 40,
        "phones": [
            "+61 448612567"
        ],
        "address": {
            "line1": "123 Main St",
            "state": "VIC",
            "country": "AU"
        }
    });

    println!("first phone number: {}", glenn["phones"][0]);

    println!("state: {}", glenn["address"]["state"]);

    let age  = &glenn["age"];
    
    let age_float = match age.as_f64() {
        None => {
            println!("could not get age as float (just for experimentation)");
            0.0
        }
        Some(data) => {
            data
        }
    };

    println!("age_float (just for experimentation) : {:?}", age_float);

    println!("{}", glenn.to_string());
}
```

Output

```bash
first phone number: "+61 448612567"
state: "VIC"
age_float (just for experimentation) : 40.0
{"address":{"country":"AU","line1":"123 Main St","state":"VIC"},"age":40,"name":"Glenn Gillen","phones":["+61 448612567"]}
```

#### Hashmap

```rust
use std::collections::HashMap;

fn get_hashmap() -> HashMap<&'static str, i32> {
    let mut people = HashMap::new();
    people.insert("Susan", 21);
    people.insert("Ed", 13);
    people.insert("Will", 14);
    people.insert("Kathy", 44);
    people.insert("Susan", 22);

    return people
}

fn main() {
    let my_hashmap = get_hashmap();

    let key1 = "Ed";
    let key2 = "Peter";

    match my_hashmap.get(key1) {
        None => {
            println!("could not find {:?} in map", key1);
        }
        Some(d) => {
            println!("found data for key {:?} : {:?}", key1, d);
        }
    }

    match my_hashmap.get(key2) {
        None => {
            println!("could not find {:?} in map", key2);
        }
        Some(d) => {
            println!("found data for key {:?} : {:?}", key2, d);
        }
    }

    //----------------------------------------------------------------

    // Note: Iteration order can change - each time we iterate through the map

    for (person, age) in my_hashmap.iter() {
        println!("person : {:?} , age : {:?}", person, age);
    }

    for person in my_hashmap.keys() {
        println!("person : {:?}", person);
    }

    for age in my_hashmap.values() {
        println!("age : {:?}", age);
    }
}
```

Output

```bash
found data for key "Ed" : 13
could not find "Peter" in map
person : "Kathy" , age : 44
person : "Ed" , age : 13
person : "Will" , age : 14
person : "Susan" , age : 22
person : "Kathy"
person : "Ed"
person : "Will"
person : "Susan"
age : 44
age : 13
age : 14
age : 22
```

Get User Input (CLI)

```rust
use std::io;

fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?; // ? would mean, this line may fail (throw error)
    Ok(buffer.trim().to_owned())
}

fn main() {
    let mut all_input = vec![];
    let mut times_input = 0;
    while times_input < 2 {
        match get_input() {
            Ok(words) => {
                all_input.push(words);
                times_input += 1;
            }
            Err(e) => {
                println!("error : {:?}", e);
            }
        }
    }

    for input in all_input {
        println!("original : {:?} , upper-case : {:?}", input, input.to_uppercase());
    }
}
```

Output

```bash
hello there !
this is some random test.
original : "hello there !" , upper-case : "HELLO THERE !"
original : "this is some random test." , upper-case : "THIS IS SOME RANDOM TEST."
```

Another CLI Program

```rust
use std::io;

enum PowerStates {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl PowerStates {
    fn new(state: &str) -> Option<PowerStates> {
        let binding = state.trim().to_lowercase();
        let power_state = binding.as_str();
        match power_state {
            "off" => {
                Some(PowerStates::Off)
            }
            "sleep" => {
                Some(PowerStates::Sleep)
            }
            "reboot" => {
                Some(PowerStates::Reboot)
            }
            "shutdown" => {
                Some(PowerStates::Shutdown)
            }
            "hibernate" => {
                Some(PowerStates::Hibernate)
            }
            _ => {
                None
            }
        }
    }
}

fn print_power_action(state: PowerStates) {
    match state {
        PowerStates::Off => {
            println!("powering off...");
        },
        PowerStates::Sleep => {
            println!("sleeping now...");
        },
        PowerStates::Reboot => {
            println!("rebooting now...");
        },
        PowerStates::Shutdown => {
            println!("shutting down now...");
        },
        PowerStates::Hibernate => {
            println!("hibernating now...");
        },
    }
}

fn main() {
    let mut buffer = String::new();

    let user_input_status = io::stdin().read_line(&mut buffer); // ? would mean, this line may fail (throw error)

    if user_input_status.is_ok() {
        match PowerStates::new(&buffer) {
            None => {
                println!("invalid power state.");
            },
            Some(d) => {
                print_power_action(d);
            },
        }
    } else {
        println!("error reading input.");
    }
}
```

CLI Driven Billing Application

```rust
use std::collections::HashMap;
use std::hash::Hash;
use std::io;
use std::num::ParseFloatError;

#[derive(Debug, Clone)]
struct Bill {
    name: String,
    amount: f64
}

struct Bills {
    inner: HashMap<String, Bill>
}

impl Bills {
    fn new() -> Self {
        Self {
            inner: HashMap::new()
        }
    }

    fn add(&mut self, bill: Bill) {
        // using .clone() , because we need to have an owned string
        self.inner.insert(bill.name.clone(), bill );
    }

    fn get_all(&self) -> Vec<Bill> {
        let mut bills = vec![];
        for bill in self.inner.values() {
            bills.push(bill.clone());
        }
        return bills;
    }

    fn remove(&mut self, name: &str) -> bool {
        self.inner.remove(name).is_some()
    }

    fn update(&mut self, name: &str, amount: f64) -> bool {
        match self.inner.get_mut(name) {
            None => {
                false
            }
            Some(bill) => {
                bill.amount = amount;
                true
            }
        }
    }
}

fn get_input() -> String  {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("please enter valid data again")
    }
    buffer.trim().to_owned()
}

fn main_menu() {
    fn show() { // this fn is available only within main_menu() fn
        println!();
        println!("[ Manage Bills ]");
        println!("1. Add Bill");
        println!("2. View Bills");
        println!("3. Remove Bill");
        println!("4. Update Bill");
        println!();
        println!("Enter Selection:");
    }

    let mut bills = Bills::new();

    loop {
        show();
        let input = get_input();
        match input.as_str() {
            "1" => {
                add_bill_menu(&mut bills);
            }
            "2" => {
                view_bills_menu(&bills);
            }
            "3" => {
                remove_bill_menu(&mut bills);
            }
            "4" => {
                update_bill_menu(&mut bills);
            }
            _ => {
                break
            }
        }
    }
}

fn get_bill_amount() -> f64 {
    println!("Amount:");
    loop {
        let input: String = get_input();
        let parsed_input: Result<f64, _> = input.parse();
        match parsed_input {
            Ok(data) => {
                return data
            }
            Err(_) => {
                println!("Please enter a valid number")
            }
        }
    }
}

fn add_bill_menu(bills: &mut Bills) {
    println!("Bill name:");
    // get the bill name
    let name = get_input();
    // get the bill amount
    let amount = get_bill_amount();

    let bill = Bill { name, amount };

    bills.add(bill);

    println!("Bill was successfully added.")
}

fn view_bills_menu(bills: &Bills) {
    for bill in bills.get_all() {
        println!("{:?}", bill);
    }
}

fn remove_bill_menu(bills: &mut Bills) {
    for bill in bills.get_all() {
        println!("{:?}", bill);
    }
    println!("Enter bill name to remove:");
    let input = get_input();

    if bills.remove(&input) {
        println!("bill removed.")
    } else {
        println!("bill not found.")
    }
}

fn update_bill_menu(bills: &mut Bills) {
    for bill in bills.get_all() {
        println!("{:?}", bill);
    }
    println!("Enter bill name to update:");
    let name = get_input();

    let amount = get_bill_amount();

    if bills.update(&name, amount) {
        println!("Bill with name {:?} updated with new value {:?}", name, amount);
    } else {
        println!("Bill with name {:?} could not be updated.", name);
    }
}

fn main() {
    main_menu();
}
```

Sample Run

```bash
[ Manage Bills ]
1. Add Bill
2. View Bills
3. Remove Bill
4. Update Bill

Enter Selection:
1
Bill name:
bill-01
Amount:
10
Bill was successfully added.

[ Manage Bills ]
1. Add Bill
2. View Bills
3. Remove Bill
4. Update Bill

Enter Selection:
1
Bill name:
bill-02
Amount:
100
Bill was successfully added.

[ Manage Bills ]
1. Add Bill
2. View Bills
3. Remove Bill
4. Update Bill

Enter Selection:
2
Bill { name: "bill-02", amount: 100.0 }
Bill { name: "bill-01", amount: 10.0 }

[ Manage Bills ]
1. Add Bill
2. View Bills
3. Remove Bill
4. Update Bill

Enter Selection:
4
Bill { name: "bill-02", amount: 100.0 }
Bill { name: "bill-01", amount: 10.0 }
Enter bill name to update:
bill-01
Amount:
555
Bill with name "bill-01" updated with new value 555.0

[ Manage Bills ]
1. Add Bill
2. View Bills
3. Remove Bill
4. Update Bill

Enter Selection:
2
Bill { name: "bill-02", amount: 100.0 }
Bill { name: "bill-01", amount: 555.0 }

[ Manage Bills ]
1. Add Bill
2. View Bills
3. Remove Bill
4. Update Bill

Enter Selection:
3
Bill { name: "bill-02", amount: 100.0 }
Bill { name: "bill-01", amount: 555.0 }
Enter bill name to remove:
bill-01
bill removed.

[ Manage Bills ]
1. Add Bill
2. View Bills
3. Remove Bill
4. Update Bill

Enter Selection:
2
Bill { name: "bill-02", amount: 100.0 }
```

#### Closure : They are anonymous functions

```rust
fn add_fn(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let sum = add(1, 5);

    // Closures : They are anonymous functions
    
    // closure
    let add = |a: i32, b: i32| -> i32 {
        a + b
    };
    let result = add(5,6);

    //closure
    let add = |a,b| a + b;
    let result = add(10,15);
}
```

#### Modules

Think of `mod { ... }` (module) as an individual file

```rust
mod messaging { 

    use std::collections::HashMap;
    use serde_json::json;

    pub(crate) fn hello() {
        println!("hello");
    }

    pub(crate) fn goodbye() {
        println!("goodbye !");
    }
}

mod math {
    pub(crate) fn add_nums(a: i32, b: i32) -> i32 {
        a + b
    }

    pub(crate) fn multiply_nums(a: i32, b: i32) -> i32 {
        a * b
    }
}

fn main() {
    messaging::hello();
    messaging::goodbye();

    println!("{:?}" , math::add_nums(10,20));
    println!("{:?}" , math::multiply_nums(4, 5));
}
```

#### Testing

```rust
fn all_caps(word: &str) -> String {
    word.to_uppercase()
}

fn main() {

}

#[cfg(test)]
mod test {
    use crate::all_caps;
    #[test]
    fn check_all_caps() {
        let result = all_caps("hello");
        let expected = String::from("HELLO");
        assert_eq!(result, expected, "string should have been in uppercase")
    }

    #[test]
    fn check_all_caps_failure() {
        let result = all_caps("hello");
        let expected = String::from("hello");
        assert_ne!(result, expected, "string should have been in lowercase")
    }
}
```

Output of `cargo test`

```bash
running 2 tests
test test::check_all_caps_failure ... ok
test test::check_all_caps ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

#### Measure Time Taken By A Function / Execution Time

```rust
fn main() {
    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    {
        my_function_to_measure();
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
```

#### Openstack API : Fetch Compute Service List

This piece of code

- Makes HTTP Request To Keystone Endpoint To Get Token (Through Headers)
- Then make HTTP (Blocking) Request (GET) Call To Nova API Endpoint
- Fetches The Results & Stores As A JSON File

`Cargo.toml`

```toml
[package]
name = "openstack_worker_pool"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
reqwest = { version = "0.11.13" , features = ["blocking"] } # reqwest with JSON parsing support
tokio = { version = "1.24.1", features = ["full"] } # for our async runtime
futures = "0.3.4" # for our async / await blocks
serde = "1.0.152"
serde_json = "1.0.91"
serde_derive = "1.0.152"
rand = "0.8.5"
rayon = "1.6.1"
sha256 = "1.1.1"
```

How To Run

```bash
OUSER="<USER>" OPASS="<PASS>" cargo run
```

`src/main.rs`

```rust
use std::borrow::Borrow;
use std::collections::HashMap;

use serde::Deserialize;
use serde_json::{json, Value};
use std::env;
use std::fmt::{Display, Formatter};
use std::future::Future;
use futures::future::err;
use reqwest::{Error};

use reqwest::{Response, StatusCode};
use reqwest::header::{ACCEPT, CONTENT_TYPE, HeaderValue, ToStrError};
use reqwest::blocking::Client;

use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::time::Duration;
use rand::{distributions::Alphanumeric};
use sha256::digest;
use std::time::{SystemTime, UNIX_EPOCH};

use std::fs::File;

use std::time::Instant;

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
struct Cloud {
    region_name: String,
    region_endpoint: String
}

impl Cloud {

    fn get_region_endpoint(&self) -> String {
        let (_, my_map) = get_cloud_mapping_v2();

        match my_map.get(&self.region_name) {
            None => { "missing".to_owned() },
            Some(d) => {
                match my_map.get(&self.region_name) {
                    None => { "missing".to_owned() }
                    Some(d) => { d.to_string() },
                }
            }
        }
    }

    fn get_keystone_token(&self) -> String {
        return match self.exec_keystone_request() {
            Some(d) => {
                d
            }
            None => {
                println!(">> no token present");
                "".to_owned()
            }
        };
    }

    fn get_creds_payload(&self, user: &str, pass: &str) -> serde_json::Value {
        let payload_body = json!(
            {
                "auth":
                {
                    "identity":
                    {
                        "methods":
                        [
                            "password"
                        ],
                        "password":
                        {
                            "user":
                            {
                                "name": user,
                                "domain":
                                {
                                    "name": "Default"
                                },
                                "password": pass
                            }
                        }
                    }
                }
            }
        );
        return payload_body;
    }

    fn exec_keystone_request(&self) -> Option<String> {

        let ouser = get_openstack_user();
        if ouser == "" {
            return None
        }

        let opass = get_openstack_pass();
        if opass == "" {
            return None
        }

        let payload_body = self.get_creds_payload(ouser.as_str(), opass.as_str());

        let cloud_endpoint = self.get_region_endpoint();

        let keystone_url = format!("https://{}:5000/v3/auth/tokens", cloud_endpoint);

        let client = reqwest::blocking::Client::new();

        let payload_body_str = payload_body.to_string();

        let resp = client.post(keystone_url)
            .body(payload_body_str)
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/json")
            .send();

        return match resp {
            Ok(d) => {
                Some(d.headers().get("X-Subject-Token").unwrap().to_str().unwrap().to_string())
            }
            Err(e) => {
                println!("error : {:?}", e);
                None
            }
        };
    }

    fn exec_compute_service_list_request(&self) -> Option<Value> {

        let token = self.get_keystone_token();
        if token == "" {
            println!("token missing, skipping this request...");
            return None
        }

        let cloud_endpoint = self.get_region_endpoint();

        let compute_service_request_url = format!("https://{}:8774/v2.1/os-services", cloud_endpoint);

        let client = reqwest::blocking::Client::new();

        let resp = client.get(compute_service_request_url)
            .header("X-Auth-Token", token)
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/json")
            .send();

        return match resp {
            Ok(d) => {
                println!("data : {:#?}", d);
                let result_str = d.text().unwrap();
                let v: Value = serde_json::from_str(result_str.as_str()).unwrap();
                Some(v)
            }
            Err(e) => {
                println!("error : {:?}", e);
                None
            }
        };
    }
}

fn compute_job(cloud: Cloud) -> String {
    return match cloud.exec_keystone_request() {
        Some(d) => {
            d
        }
        None => {
            println!(">> no token present <<");
            "".to_owned()
        }
    };
}

fn compute_job_compute_service_list(cloud: Cloud) -> Value {
    return match cloud.exec_compute_service_list_request() {
        Some(d) => {
            d
        }
        None => {
            println!(">> no token present <<");
            let data = r#"{}"#;
            // Parse the string of data into serde_json::Value.
            let v: Value = serde_json::from_str(data).unwrap();
            v
        }
    };
}


fn generate_jobs() -> Vec<Cloud> {

    let mut jobs = vec![];

    let (cloud_region_mappings, _) = get_cloud_mapping_v2();

    for item in cloud_region_mappings {
        jobs.push(item);
    }
    return jobs;
}


fn get_cloud_mapping_v2() -> (Vec<Cloud>, HashMap<String, String>) {
    let vec_data = get_cloud_vector();
    let my_map = get_hashmap_v2(&vec_data);
    return (vec_data, my_map)
}

fn get_cloud_vector() -> Vec<Cloud> {

    let mut cloud_region_mappings = vec![];

    // replace this with your region endpoints

    cloud_region_mappings.push(Cloud{ region_name: "region01".parse().unwrap(), region_endpoint: "region01.endpoint.company.com".parse().unwrap() });
    cloud_region_mappings.push(Cloud{ region_name: "region02".parse().unwrap(), region_endpoint: "region02.endpoint.company.com".parse().unwrap() });
    cloud_region_mappings.push(Cloud{ region_name: "region03".parse().unwrap(), region_endpoint: "region03.endpoint.company.com".parse().unwrap() });
    cloud_region_mappings.push(Cloud{ region_name: "region04".parse().unwrap(), region_endpoint: "region04.endpoint.company.com".parse().unwrap() });
    cloud_region_mappings.push(Cloud{ region_name: "region05".parse().unwrap(), region_endpoint: "region05.endpoint.company.com".parse().unwrap() });

    return cloud_region_mappings
}

fn get_hashmap_v2(vec_data: &Vec<Cloud>) -> HashMap<String, String> {
    let mut my_map: HashMap<String, String> = HashMap::new();

    for data in vec_data {
        let (region_name, endpoint) = get_regionname_and_andpoint(data);
        my_map.insert(region_name, endpoint);
    }
    return my_map
}

fn get_regionname_and_andpoint(mapping: &Cloud) -> (String, String) {
    return (mapping.region_name.to_string(), mapping.region_endpoint.to_string())
}


fn get_openstack_user() -> String {
    let uname = "OUSER";
    return match env::var(uname) {
        Ok(v) => return v,
        Err(e) => {
            println!("env variable 'OUSER' is not set !");
            "".to_owned()
        }
    }
}

fn get_openstack_pass() -> String {
    let opass = "OPASS";
    return match env::var(opass) {
        Ok(v) => v,
        Err(e) => {
            println!("env variable 'OPASS' is not set !");
            "".to_owned()
        },
    };
}

fn main() {
    println!("OpenStack Test");

    rayon::ThreadPoolBuilder::new().num_threads(30).build_global().unwrap();

    let mut my_jobs = generate_jobs();

    let now = Instant::now();

    let results: Vec<Value> = my_jobs.into_par_iter()
        .map(compute_job_compute_service_list)
        .collect();

    let elapsed = now.elapsed();

    println!("results : {:#?}", results);

    println!("Elapsed: {:.2?}", elapsed);

    let f = match File::create("results.json") {
        Ok(d) => {
            let write_data = match serde_json::to_writer_pretty(&d, &results) {
                Ok(d) => {
                    println!("data successfully written to json file !");
                },
                Err(e) => {
                    println!("data could not be written to json file : {:?}", e);
                },
            };
        },
        Err(e) => {
            println!("json file could not be created : {:?}", e);
        },
    };
}
```

#### Azure : Get API & Graph API Token

How To Make API Calls To Public Azure Endpoints (Still, Work In Progress)

`Cargo.toml`

```toml
[package]
name = "azure-rust"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rand = "0.8.5"
regex = "1.7.0"
reqwest = { version = "0.11.13" , features = ["blocking"] } # reqwest with JSON parsing support
tokio = { version = "1.23.0", features = ["full"] }
serde = "1.0.152"
serde_json = "1.0.91"
serde_derive = "1.0.152"
rayon = "1.6.1"
futures = "0.3.4" # for our async / await blocks
```

File `src/main.rs`

```rust
use std::collections::HashMap;
use reqwest::{Response, StatusCode};
use serde_json::{Map, Value};


#[derive(Debug)]
pub enum GenericError {
    InvalidServicePrincipal,
    MissingDataInResponse,
}

#[derive(Debug)]
struct CustomError {
    err_type: GenericError,
    err_msg: String
}

enum TokenType {
    AzureApiToken,
    AzureGraphToken,
}

// pub type ResultToken = Result<String, CustomError>;

struct ServicePrincipal {
    client_id: String,
    client_secret: String,
    tenant_id: String,
    region: String,
    token_type: TokenType
}

impl ServicePrincipal {
    fn get_token(&self, token_type: TokenType) -> Result<String, CustomError> {
        let region = &self.region;
        let client_id = &self.client_id;
        let client_secret = &self.client_secret;
        let region = &self.region;
        let tenant_id = &self.tenant_id;

        let azure_region_details = get_azure_region_details(region);

        let authority_url = match token_type {
            TokenType::AzureApiToken => {
                let authority_url: String = azure_region_details.get("authorityURL").unwrap().to_string() + "/" + tenant_id +  "/oauth2/token";
                authority_url
            },
            TokenType::AzureGraphToken => {
                let authority_url: String = azure_region_details.get("authorityURL").unwrap().to_string() + "/" + tenant_id +  "/oauth2/v2.0/token";
                authority_url
            },
        };

        println!("authority_url : {:?}", authority_url);

        let resource_api: String = azure_region_details.get("resourceAPI").unwrap().to_string();

        let scope = azure_region_details.get("resourceGraphURL").unwrap().to_string();

        println!("resource_api : {:?}", resource_api);

        let params = match token_type {
            TokenType::AzureApiToken => {
                let params = [
                    ("grant_type", "client_credentials"),
                    ("client_id", client_id),
                    ("client_secret", client_secret),
                    ("resource", ""),
                ];
                params
            },
            TokenType::AzureGraphToken => {
                let params = [
                    ("grant_type", "client_credentials"),
                    ("client_id", client_id),
                    ("client_secret", client_secret),
                    ("scope", scope.as_str()),
                ];
                params
            }
        };


        let client = reqwest::blocking::Client::new();

        let res = client.post(authority_url).form(&params).send();

        let result =  match res {
            Ok(res) => {
                println!("http response : {:?}", res);
                let data_str = res.text().unwrap_or("N/A".to_string());
                let value: Value = serde_json::from_str(data_str.as_str()).unwrap();

                let my_object = match value.as_object() {
                    None => {
                        println!("no object found")
                    },
                    Some(d) => {
                        if d.contains_key("token_type") == false || d.contains_key("access_token") == false {
                            let msg:String = format!("http response does not contains either the follow keys : (token_type|access_token)");
                            let custom_err = CustomError {
                                err_msg: String::from(msg),
                                err_type: GenericError::MissingDataInResponse
                            };
                            return Err(custom_err)
                        }
                    }
                };

                let token = format!("{} {}", &value["token_type"].as_str().unwrap(), &value["access_token"].as_str().unwrap());
                token
            },
            Err(err) => {
                let msg:String = format!("there was an error http form submit to get token {:?}", err);
                let custom_err = CustomError {
                    err_msg: String::from(msg),
                    err_type: GenericError::InvalidServicePrincipal
                };
                return Err(custom_err)
            }
        };
        Ok(result)
    }
}


fn get_azure_region_details(region: &str) -> HashMap<String, String> {
    let mut azure_region: HashMap<String, String> = HashMap::new();

    return match region {
        "america" => {
            azure_region.insert("resourceAPI".to_owned(), "https://management.azure.com".to_owned());
            azure_region.insert("authorityURL".to_owned(), "https://login.microsoftonline.com".to_owned());
            azure_region.insert("resourceGraphURL".to_owned(), "https://graph.microsoft.com/.default".to_owned());
            azure_region
        },
        "china" => {
            azure_region.insert("resourceAPI".to_owned(), "https://management.chinacloudapi.cn".to_owned());
            azure_region.insert("authorityURL".to_owned(), "https://login.partner.microsoftonline.cn".to_owned());
            azure_region.insert("resourceGraphURL".to_owned(), "https://microsoftgraph.chinacloudapi.cn/.default".to_owned());
            azure_region
        },
        _ => {
            azure_region.insert("resourceAPI".to_owned(), "https://management.azure.com".to_owned());
            azure_region.insert("authorityURL".to_owned(), "https://login.microsoftonline.com".to_owned());
            azure_region.insert("resourceGraphURL".to_owned(), "https://graph.microsoft.com/.default".to_owned());
            azure_region
        }
    }
}

fn main() {
    println!("Azure Data >");

    let service_principal = ServicePrincipal {
        client_id: "00000000-0000-0000-0000-000000000000".to_owned(),
        client_secret: "<INSERT_SECRET_HERE>".to_owned(),
        region: "america".to_owned(),
        tenant_id: "00000000-0000-0000-0000-000000000000".to_owned(),
        token_type: TokenType::AzureApiToken,
    };

    // api token
    let api_token = match service_principal.get_token(TokenType::AzureApiToken) {
        Ok(data) => {
            println!("\ntoken\n");
            println!("{}\n", data);
        }
        Err(e) => {
            println!("error : {:#?}", e);
        }
    };

    // graph token
    let graph_token = match service_principal.get_token(TokenType::AzureGraphToken) {
        Ok(data) => {
            println!("\ntoken\n");
            println!("{}\n", data);
        }
        Err(e) => {
            println!("error : {:#?}", e);
        }
    };
}
```

#### Public Azure Sample

Fetching Virtual Machines : Azure Resource Graph Query (ADX) : (FYI : Still, Work In Progress) : Part-1

```rust
use std::collections::HashMap;
use reqwest::{Response, StatusCode};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::{json, Map, Value};
use std::{thread, time};

#[derive(Debug)]
pub enum GenericError {
    InvalidServicePrincipal,
    MissingDataInResponse,
}

#[derive(Debug)]
struct CustomError {
    err_type: GenericError,
    err_msg: String
}

enum TokenType {
    AzureApiToken,
    AzureGraphToken,
}

// pub type ResultToken = Result<String, CustomError>;

struct ServicePrincipal {
    client_id: String,
    client_secret: String,
    tenant_id: String,
    region: String,
    token_type: TokenType
}

// microsoft public api endpoints

fn get_azure_region_details(region: &str) -> HashMap<String, String> {
    let mut azure_region: HashMap<String, String> = HashMap::new();

    return match region {
        "america" => {
            azure_region.insert("resourceAPI".to_owned(), "https://management.azure.com".to_owned());
            azure_region.insert("authorityURL".to_owned(), "https://login.microsoftonline.com".to_owned());
            azure_region.insert("resourceGraphURL".to_owned(), "https://graph.microsoft.com/.default".to_owned());
            azure_region
        },
        "china" => {
            azure_region.insert("resourceAPI".to_owned(), "https://management.chinacloudapi.cn".to_owned());
            azure_region.insert("authorityURL".to_owned(), "https://login.partner.microsoftonline.cn".to_owned());
            azure_region.insert("resourceGraphURL".to_owned(), "https://microsoftgraph.chinacloudapi.cn/.default".to_owned());
            azure_region
        },
        _ => {
            azure_region.insert("resourceAPI".to_owned(), "https://management.azure.com".to_owned());
            azure_region.insert("authorityURL".to_owned(), "https://login.microsoftonline.com".to_owned());
            azure_region.insert("resourceGraphURL".to_owned(), "https://graph.microsoft.com/.default".to_owned());
            azure_region
        }
    }
}


impl ServicePrincipal {

    fn get_params_and_url(&self, token_type: TokenType, azure_scope: Option<String>) -> ([(String, String); 4], String, String, String) {
        let region = &self.region;
        let client_id = &self.client_id;
        let tenant_id = &self.tenant_id;
        let client_secret = &self.client_secret;

        let azure_region_details = get_azure_region_details(region.as_str());

        let authority_url = match token_type {
            TokenType::AzureApiToken => {
                let authority_url: String = azure_region_details.get("authorityURL").unwrap().to_string() + "/" + tenant_id +  "/oauth2/token";
                authority_url
            },
            TokenType::AzureGraphToken => {
                let authority_url: String = azure_region_details.get("authorityURL").unwrap().to_string() + "/" + tenant_id +  "/oauth2/v2.0/token";
                authority_url
            },
        };

        let resource_graph_url = azure_region_details.get("resourceGraphURL").unwrap().to_string();

        let resource_api_url = azure_region_details.get("resourceAPI").unwrap().to_string();

        // println!("authority_url      : {:?}", authority_url);
        // println!("resource_graph_url : {:?}", resource_graph_url);
        // println!("resource_api_url   : {:?}", resource_api_url);

        let my_scope = match azure_scope {
            None => {
                println!("azure_scope is None !");
                "https://graph.microsoft.com/.default".to_owned()
            },
            Some(d) => {
                d.to_owned()
            },
        };

        let params = match token_type {
            TokenType::AzureApiToken => {
                let params = [
                    ("grant_type".to_string(), "client_credentials".to_string()),
                    ("client_id".to_string(), client_id.to_string()),
                    ("client_secret".to_string(), client_secret.to_string()),
                    ("resource".to_string(), "".to_string()),
                ];
                params
            },
            TokenType::AzureGraphToken => {
                let params = [
                    ("grant_type".to_string(), "client_credentials".to_string()),
                    ("client_id".to_string(), client_id.to_string()),
                    ("client_secret".to_string(), client_secret.to_string()),
                    ("scope".to_string(), my_scope.to_string()),
                ];
                params
            }
        };
        let return_data = (params, authority_url, resource_graph_url, resource_api_url);
        // println!("{:#?}", &return_data);
        return return_data;
    }

    fn get_token(&self, token_type: TokenType, azure_scope: Option<String>) -> Result<String, CustomError> {
        let region = &self.region;
        let client_id = &self.client_id;
        let client_secret = &self.client_secret;
        let region = &self.region;
        let tenant_id = &self.tenant_id;

        let my_scope = Some("https://management.azure.com/.default".to_string());

        let my_scope = match azure_scope {
            None => {
                println!("azure_scope is None !");
                "https://graph.microsoft.com/.default".to_owned()
            },
            Some(d) => {
                d.to_owned()
            },
        };

        let (
            params,
            authority_url,
            resource_graph_url,
            resource_api_url
        ) = self.get_params_and_url(token_type, Some(my_scope));

        let client = reqwest::blocking::Client::new();

        let res = client.post(authority_url).form(&params).send();

        let result =  match res {
            Ok(res) => {
                println!("http response : {:?}", res);
                let data_str = res.text().unwrap_or("N/A".to_string());
                let value: Value = serde_json::from_str(data_str.as_str()).unwrap();

                let my_object = match value.as_object() {
                    None => {
                        println!("no object found");
                    },
                    Some(d) => {
                        if d.contains_key("token_type") == false || d.contains_key("access_token") == false {
                            let msg:String = format!("http response does not contains either the follow keys : (token_type|access_token)");
                            let custom_err = CustomError {
                                err_msg: String::from(msg),
                                err_type: GenericError::MissingDataInResponse
                            };
                            return Err(custom_err)
                        }
                    }
                };

                let token = format!("{} {}", &value["token_type"].as_str().unwrap(), &value["access_token"].as_str().unwrap());
                token
            },
            Err(err) => {
                let msg:String = format!("there was an error http form submit to get token {:?}", err);
                let custom_err = CustomError {
                    err_msg: String::from(msg),
                    err_type: GenericError::InvalidServicePrincipal
                };
                return Err(custom_err)
            }
        };
        Ok(result)
    }

    fn get_vms(&self) {

        let my_scope = Some("https://management.azure.com/.default".to_owned());

        let (
            params,
            authority_url,
            resource_graph_url,
            resource_api_url
        ) = self.get_params_and_url(TokenType::AzureGraphToken, my_scope.to_owned());

        let api_url = resource_api_url.to_owned() + "/providers/Microsoft.ResourceGraph/resources?api-version=2020-04-01-preview";

        println!("api_url for azure_data_explorer : {}", api_url.as_str());

        let empty:Vec<String> = vec![];

        /*
            0 - 999      -> first page
            1000 - 1999  -> second page
            2000 - 2999  -> third page
            3000 - 3999  -> fourth page

            $top -> The maximum number of rows that the query should return. Overrides the page size when $skipToken property is present.
            $skip -> The number of rows to skip from the beginning of the results. Overrides the next page offset when $skipToken property is present.
        */

        let json_payload = json!({
            "subscriptions": empty,
            "options" : {
                "$top" : 1000, // usually fixed at 1000
                "$skip" : 0
            },
            "query": "Resources | where type =~ 'Microsoft.Compute/virtualMachines'"

            // "query": "Resources | project id, name, type, location | where type =~ 'Microsoft.Compute/virtualMachines' | limit 10"
        });

        println!("json_payload:");
        println!("{:#?}", json_payload);

        let token_data = match self.get_token(TokenType::AzureGraphToken, my_scope.to_owned()) {
            Ok(d) => {
                let client = reqwest::blocking::Client::new();

                let token = d.to_owned();
                println!("token for azure_data_explorer : {}", token.as_str());

                let now = time::Instant::now();

                let res = match client.post(api_url.as_str()).body(json_payload.to_string()).header(CONTENT_TYPE, "application/json").header(AUTHORIZATION, token.as_str()).send() {
                    Ok(d) => {
                        let elapsed = now.elapsed();

                        println!("azure_data_explorer query result:");
                        let json_str = d.text().unwrap();
                        let json_data: Value = serde_json::from_str(json_str.as_str()).unwrap();

                        // uncomment this to see the records
                        // println!("{:#?}", json_data["data"]["rows"]);

                        let total_records = &json_data["data"]["rows"].as_array().unwrap().len();
                        println!("Total Records Fetched : {}",  total_records);
                        println!("Total Time Taken : {:.2?}", elapsed);
                    },
                    Err(e) => {
                        println!("error : could not make http request for adx query : {:?}", e);
                    },
                };
            },
            Err(e) => {
                println!("error : could not fetch token : {:#?}", e);
            },
        };
    }
}

fn main() {
    println!("Azure Data >");

    let service_principal = ServicePrincipal {
        client_id: "00000000-0000-0000-0000-000000000000".to_owned(),
        client_secret: "<INSERT_SECRET_HERE>".to_owned(),
        region: "america".to_owned(),
        tenant_id: "00000000-0000-0000-0000-000000000000".to_owned(),
        token_type: TokenType::AzureApiToken,
    };

    // api token
    let api_token = match service_principal.get_token(TokenType::AzureApiToken, None) {
        Ok(data) => {
            println!("\ntoken\n");
            println!("{}\n", data);
        }
        Err(e) => {
            println!("error : {:#?}", e);
        }
    };

    // graph token
    let graph_token = match service_principal.get_token(TokenType::AzureGraphToken, None) {
        Ok(data) => {
            println!("\ntoken\n");
            println!("{}\n", data);
        }
        Err(e) => {
            println!("error : {:#?}", e);
        }
    };

    service_principal.get_vms();
}
```

#### Split Into Multiple Chunks

```rust
fn main() {
    let input: Vec<_> = (0..27).collect();
    let result: Vec<_> = input.chunks(5).collect();
    println!("{:?}", result);
}
```

Output

```bash
[
    [
        0,
        1,
        2,
        3,
        4,
    ],
    [
        5,
        6,
        7,
        8,
        9,
    ],
    [
        10,
        11,
        12,
        13,
        14,
    ],
    [
        15,
        16,
        17,
        18,
        19,
    ],
    [
        20,
        21,
        22,
        23,
        24,
    ],
    [
        25,
        26,
    ],
]
```

#### Public Azure Sample

Fetching Virtual Machines : Azure Resource Graph Query (ADX) : (FYI : Still, Work In Progress) : Part-2

How To Run

```bash
cargo build --release
```

```bash
CLIENT_ID="00000000-0000-0000-0000-000000000000" CLIENT_SECRET="<INSERT_SECRET_HERE>" TENANT_ID="00000000-0000-0000-0000-000000000000" REGION="america" target/release/azure-rust
```

`src/main.rs`

```rust
use std::collections::HashMap;
use reqwest::{Response, StatusCode};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::{json, Map, Value};
use std::{env, thread, time};
use std::slice::Chunks;

#[derive(Debug)]
pub enum GenericError {
    InvalidServicePrincipal,
    MissingDataInResponse,
}

#[derive(Debug)]
struct CustomError {
    err_type: GenericError,
    err_msg: String
}

enum TokenType {
    AzureApiToken,
    AzureGraphToken,
}

#[derive(Clone, Copy)]
enum AzureRecord {
    VirtualMachine
}

// pub type ResultToken = Result<String, CustomError>;

struct ServicePrincipal {
    client_id: String,
    client_secret: String,
    tenant_id: String,
    region: String,
    token_type: TokenType
}

// microsoft public api endpoints

fn get_azure_region_details(region: &str) -> HashMap<String, String> {
    let mut azure_region: HashMap<String, String> = HashMap::new();

    return match region {
        "america" => {
            azure_region.insert("resourceAPI".to_owned(), "https://management.azure.com".to_owned());
            azure_region.insert("authorityURL".to_owned(), "https://login.microsoftonline.com".to_owned());
            azure_region.insert("resourceGraphURL".to_owned(), "https://graph.microsoft.com/.default".to_owned());
            azure_region
        },
        "china" => {
            azure_region.insert("resourceAPI".to_owned(), "https://management.chinacloudapi.cn".to_owned());
            azure_region.insert("authorityURL".to_owned(), "https://login.partner.microsoftonline.cn".to_owned());
            azure_region.insert("resourceGraphURL".to_owned(), "https://microsoftgraph.chinacloudapi.cn/.default".to_owned());
            azure_region
        },
        _ => {
            azure_region.insert("resourceAPI".to_owned(), "https://management.azure.com".to_owned());
            azure_region.insert("authorityURL".to_owned(), "https://login.microsoftonline.com".to_owned());
            azure_region.insert("resourceGraphURL".to_owned(), "https://graph.microsoft.com/.default".to_owned());
            azure_region
        }
    }
}

fn get_azure_record(azure_record: AzureRecord) -> String {
    return match azure_record {
        AzureRecord::VirtualMachine => "microsoft.compute/virtualmachines".to_owned(),
    }
}

impl ServicePrincipal {

    fn get_params_and_url(&self, token_type: TokenType, azure_scope: Option<String>) -> ([(String, String); 4], String, String, String) {
        let region = &self.region;
        let client_id = &self.client_id;
        let tenant_id = &self.tenant_id;
        let client_secret = &self.client_secret;

        let azure_region_details = get_azure_region_details(region.as_str());

        let authority_url = match token_type {
            TokenType::AzureApiToken => {
                let authority_url: String = azure_region_details.get("authorityURL").unwrap().to_string() + "/" + tenant_id +  "/oauth2/token";
                authority_url
            },
            TokenType::AzureGraphToken => {
                let authority_url: String = azure_region_details.get("authorityURL").unwrap().to_string() + "/" + tenant_id +  "/oauth2/v2.0/token";
                authority_url
            },
        };

        let resource_graph_url = azure_region_details.get("resourceGraphURL").unwrap().to_string();

        let resource_api_url = azure_region_details.get("resourceAPI").unwrap().to_string();

        // println!("authority_url      : {:?}", authority_url);
        // println!("resource_graph_url : {:?}", resource_graph_url);
        // println!("resource_api_url   : {:?}", resource_api_url);

        let my_scope = match azure_scope {
            None => {
                println!("azure_scope is None !");
                "https://graph.microsoft.com/.default".to_owned()
            },
            Some(d) => {
                d.to_owned()
            },
        };

        let params = match token_type {
            TokenType::AzureApiToken => {
                let params = [
                    ("grant_type".to_string(), "client_credentials".to_string()),
                    ("client_id".to_string(), client_id.to_string()),
                    ("client_secret".to_string(), client_secret.to_string()),
                    ("resource".to_string(), "".to_string()),
                ];
                params
            },
            TokenType::AzureGraphToken => {
                let params = [
                    ("grant_type".to_string(), "client_credentials".to_string()),
                    ("client_id".to_string(), client_id.to_string()),
                    ("client_secret".to_string(), client_secret.to_string()),
                    ("scope".to_string(), my_scope.to_string()),
                ];
                params
            }
        };
        let return_data = (params, authority_url, resource_graph_url, resource_api_url);
        // println!("{:#?}", &return_data);
        return return_data;
    }

    fn get_token(&self, token_type: TokenType, azure_scope: Option<String>) -> Result<String, CustomError> {
        let region = &self.region;
        let client_id = &self.client_id;
        let client_secret = &self.client_secret;
        let region = &self.region;
        let tenant_id = &self.tenant_id;

        let my_scope = Some("https://management.azure.com/.default".to_string());

        let my_scope = match azure_scope {
            None => {
                println!("azure_scope is None !");
                "https://graph.microsoft.com/.default".to_owned()
            },
            Some(d) => {
                d.to_owned()
            },
        };

        let (
            params,
            authority_url,
            resource_graph_url,
            resource_api_url
        ) = self.get_params_and_url(token_type, Some(my_scope));

        let client = reqwest::blocking::Client::new();

        let res = client.post(authority_url).form(&params).send();

        let result =  match res {
            Ok(res) => {
                println!("http response : {:?}", res);
                let data_str = res.text().unwrap_or("N/A".to_string());
                let value: Value = serde_json::from_str(data_str.as_str()).unwrap();

                let my_object = match value.as_object() {
                    None => {
                        println!("no object found");
                    },
                    Some(d) => {
                        if d.contains_key("token_type") == false || d.contains_key("access_token") == false {
                            let msg:String = format!("http response does not contains either the follow keys : (token_type|access_token)");
                            let custom_err = CustomError {
                                err_msg: String::from(msg),
                                err_type: GenericError::MissingDataInResponse
                            };
                            return Err(custom_err)
                        }
                    }
                };

                let token = format!("{} {}", &value["token_type"].as_str().unwrap(), &value["access_token"].as_str().unwrap());
                token
            },
            Err(err) => {
                let msg:String = format!("there was an error http form submit to get token {:?}", err);
                let custom_err = CustomError {
                    err_msg: String::from(msg),
                    err_type: GenericError::InvalidServicePrincipal
                };
                return Err(custom_err)
            }
        };
        Ok(result)
    }

    fn get_total_records(&self, azure_record: AzureRecord) -> u64 {
        let my_scope = Some("https://management.azure.com/.default".to_owned());

        let (
            params,
            authority_url,
            resource_graph_url,
            resource_api_url
        ) = self.get_params_and_url(TokenType::AzureGraphToken, my_scope.to_owned());

        let api_url = resource_api_url.to_owned() + "/providers/Microsoft.ResourceGraph/resources?api-version=2020-04-01-preview";

        println!("api_url for azure_data_explorer : {}", api_url.as_str());

        let empty:Vec<String> = vec![];

        let record_name = get_azure_record(azure_record);

        let total_records_query = format!("Resources | where type == '{}' | summarize count()", record_name);

        let json_payload = json!({
            "subscriptions": empty,
            "query": total_records_query
        });

        println!("json_payload:");
        println!("{:#?}", json_payload);

        let total_records = return match self.get_token(TokenType::AzureGraphToken, my_scope.to_owned()) {
            Ok(d) => {
                let client = reqwest::blocking::Client::new();
                let token = d.to_owned();
                println!("token for azure_data_explorer : {}", token.as_str());
                let now = time::Instant::now();
                let res = match client.post(api_url.as_str()).body(json_payload.to_string()).header(CONTENT_TYPE, "application/json").header(AUTHORIZATION, token.as_str()).send() {
                    Ok(d) => {
                        let elapsed = now.elapsed();
                        let json_str = d.text().unwrap();
                        let json_data: Value = serde_json::from_str(json_str.as_str()).unwrap();
                        println!("{}", serde_json::to_string_pretty(&json_data).unwrap());
                        let total_records = &json_data["data"]["rows"][0][0].as_u64().unwrap();
                        println!("total_records : {}", total_records);
                        println!("TTT : {:.2?}", elapsed);
                        total_records.to_owned()
                    },
                    Err(e) => {
                        println!("error : could not make http request for adx query : {:?}", e);
                        0 as u64
                    },
                };
                res
            }, // Ok
            Err(e) => {
                println!("error : could not fetch token : {:#?}", e);
                0
            }, // Err
        };
        total_records
    }

    fn get_azure_records(&self, azure_record: AzureRecord, top: i32, skip: i32) -> Vec<Value> {

        let my_scope = Some("https://management.azure.com/.default".to_owned());

        let empty_vec:Vec<Value> = Vec::new();

        let (
            params,
            authority_url,
            resource_graph_url,
            resource_api_url
        ) = self.get_params_and_url(TokenType::AzureGraphToken, my_scope.to_owned());

        let api_url = resource_api_url.to_owned() + "/providers/Microsoft.ResourceGraph/resources?api-version=2020-04-01-preview";

        println!("api_url for azure_data_explorer : {}", api_url.as_str());

        let empty:Vec<String> = vec![];

        /*
            0 - 999      -> first page
            1000 - 1999  -> second page
            2000 - 2999  -> third page
            3000 - 3999  -> fourth page

            $top -> The maximum number of rows that the query should return. Overrides the page size when $skipToken property is present.
            $skip -> The number of rows to skip from the beginning of the results. Overrides the next page offset when $skipToken property is present.
        */

        let azure_entity = get_azure_record(azure_record);

        let query = format!("Resources | where type =~ '{}'", azure_entity);

        let json_payload = json!({
            "subscriptions": empty,
            "options" : {
                "$top" : top, // usually fixed at 1000
                "$skip" : skip
            },
            "query": query
        });

        println!("{}", serde_json::to_string_pretty(&json_payload).unwrap());

        let azure_data = match self.get_token(TokenType::AzureGraphToken, my_scope.to_owned()) {
            Ok(d) => {
                let client = reqwest::blocking::Client::new();
                let token = d.to_owned();
                let now = time::Instant::now();
                let res = match client.post(api_url.as_str()).body(json_payload.to_string()).header(CONTENT_TYPE, "application/json").header(AUTHORIZATION, token.as_str()).send() {
                    Ok(d) => {
                        let elapsed = now.elapsed();

                        println!("azure_data_explorer query result:");
                        let json_str = d.text().unwrap();
                        let json_data: Value = serde_json::from_str(json_str.as_str()).unwrap();
                        let rows = &json_data["data"]["rows"];


                        println!("TTT (top : {} , skip : {}) : {:.2?}", top, skip, elapsed);
                        rows.as_array().unwrap().to_vec()
                    },
                    Err(e) => {
                        println!("error : could not make http request for adx query : {:?}", e);
                        empty_vec
                    },
                };
                res.to_owned()
            },
            Err(e) => {
                println!("error : could not fetch token : {:#?}", e);
                empty_vec
            },
        };
        azure_data
    }

    fn get_all_azure_records(&self, azure_record: AzureRecord, records_per_page: i32, write_json_to_file: bool) -> Vec<Value> {
        let total_records_for_virtual_machines = self.get_total_records(azure_record);

        // let page_number = 1;

        // let records_per_page = 1000;

        let total_pages = (total_records_for_virtual_machines as f32 / records_per_page as f32).ceil() as i32;

        println!("records_per_page : {}", records_per_page);
        println!("total_pages : {}", total_pages);

        let mut all_records: Vec<Value> = Vec::new();

        let now = time::Instant::now();

        let mut page_list: Vec<i32> = Vec::new();

        for i in 1..=total_pages {
            page_list.push(i);
        }

        let result: Vec<_> = page_list.chunks(8).collect();
        println!("{:#?}", result);

        for page_number in 1..=total_pages {
            println!("fetching data for page_number : {}...", page_number);

            let skip = get_skip_number(records_per_page, page_number);

            let records = self.get_azure_records(AzureRecord::VirtualMachine, records_per_page, skip);
            println!("total records for page_number {} : {}", page_number, records.len());
            for item in records.iter() {
                &all_records.push(item.to_owned());
            }
            println!("iteration in progress : length of all_records : {}", &all_records.len());
            let elapsed = now.elapsed();
            println!("total time taken so far : {:.2?}", elapsed);
        }

        println!("length of all_records : {}", &all_records.len());

        if write_json_to_file {
            let azure_entity = get_azure_record(azure_record);
            let output_file = azure_entity.replace("/", "_").replace(".", "_") + ".json";

            std::fs::write(
            output_file,
        serde_json::to_string_pretty(&all_records).unwrap(),
            ).unwrap();
        }
        all_records
    }
}

enum EnvironmentVarible {
    ClientID,
    ClientSecret,
    TenantID,
    Region,
}

fn get_env_var(env_var: EnvironmentVarible) -> String {
    return match env_var {
        EnvironmentVarible::ClientID => {
            match env::var("CLIENT_ID") {
                Ok(v) => v,
                Err(e) => {
                    println!("environment variable 'CLIENT_ID' is not set !");
                    "".to_owned()
                },
            }
        },
        EnvironmentVarible::ClientSecret => {
            match env::var("CLIENT_SECRET") {
                Ok(v) => v,
                Err(e) => {
                    println!("environment variable 'CLIENT_SECRET' is not set !");
                    "".to_owned()
                },
            }
        },
        EnvironmentVarible::TenantID => {
            match env::var("TENANT_ID") {
                Ok(v) => v,
                Err(e) => {
                    println!("environment variable 'TENANT_ID' is not set !");
                    "".to_owned()
                },
            }
        },
        EnvironmentVarible::Region => {
            match env::var("REGION") {
                Ok(v) => v,
                Err(e) => {
                    println!("environment variable 'TENANT_ID' is not set !");
                    "".to_owned()
                },
            }
        }
    };
}

fn main() {
    println!("Azure Data >");

    let client_id = get_env_var(EnvironmentVarible::ClientID);
    let client_secret = get_env_var(EnvironmentVarible::ClientSecret);
    let tenant_id = get_env_var(EnvironmentVarible::TenantID);
    let region = get_env_var(EnvironmentVarible::Region);

    let service_principal = ServicePrincipal {
        client_id,
        client_secret,
        region,
        tenant_id,
        token_type: TokenType::AzureApiToken,
    };

    // api token
    let api_token = match service_principal.get_token(TokenType::AzureApiToken, None) {
        Ok(data) => {
            println!("\ntoken\n");
            println!("{}\n", data);
        }
        Err(e) => {
            println!("error : {:#?}", e);
        }
    };

    // graph token
    let graph_token = match service_principal.get_token(TokenType::AzureGraphToken, None) {
        Ok(data) => {
            println!("\ntoken\n");
            println!("{}\n", data);
        }
        Err(e) => {
            println!("error : {:#?}", e);
        }
    };

    let all_recs = service_principal.get_all_azure_records(AzureRecord::VirtualMachine,1000,true);
    println!("all_recs.len() : {}", all_recs.len())

}

fn get_skip_number(records_per_page: i32, page_number: i32) -> i32 {
    (page_number-1) * records_per_page
}
```

Fetching Virtual Machines : Azure Resource Graph Query (ADX) , Using Cache : (FYI : Still, Work In Progress) : Part-3

`src/main.rs`

```rust
use std::collections::HashMap;
use reqwest::{Response, StatusCode};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::{json, Map, Value};
use std::{env, thread, time};
use std::borrow::Borrow;
use std::slice::Chunks;

use moka::sync::Cache;
use std::time::Duration;

use lazy_static::lazy_static;


#[derive(Debug)]
pub enum GenericError {
    InvalidServicePrincipal,
    MissingDataInResponse,
}

#[derive(Debug)]
struct CustomError {
    err_type: GenericError,
    err_msg: String
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum TokenType {
    AzureApiToken,
    AzureGraphToken,
}

#[derive(Clone, Copy)]
enum AzureRecord {
    VirtualMachine
}

// pub type ResultToken = Result<String, CustomError>;

struct ServicePrincipal {
    client_id: String,
    client_secret: String,
    tenant_id: String,
    region: String,
    token_type: TokenType
}

// global variable

lazy_static! {
    static ref APP_CACHE:Cache<TokenAndScope, String> = Cache::builder()
        // Time to live (TTL): 45 minutes
        .time_to_live(Duration::from_secs(45 * 60))
        // Time to idle (TTI):  45 minutes
        .time_to_idle(Duration::from_secs(45 * 60))
        // Create the cache.
        .build();
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct TokenAndScope {
    token_type: TokenType,
    azure_scope: Option<String>
}

// microsoft public api endpoints

fn get_azure_region_details(region: &str) -> HashMap<String, String> {
    let mut azure_region: HashMap<String, String> = HashMap::new();

    return match region {
        "america" => {
            azure_region.insert("resourceAPI".to_owned(), "https://management.azure.com".to_owned());
            azure_region.insert("authorityURL".to_owned(), "https://login.microsoftonline.com".to_owned());
            azure_region.insert("resourceGraphURL".to_owned(), "https://graph.microsoft.com/.default".to_owned());
            azure_region
        },
        "china" => {
            azure_region.insert("resourceAPI".to_owned(), "https://management.chinacloudapi.cn".to_owned());
            azure_region.insert("authorityURL".to_owned(), "https://login.partner.microsoftonline.cn".to_owned());
            azure_region.insert("resourceGraphURL".to_owned(), "https://microsoftgraph.chinacloudapi.cn/.default".to_owned());
            azure_region
        },
        _ => {
            azure_region.insert("resourceAPI".to_owned(), "https://management.azure.com".to_owned());
            azure_region.insert("authorityURL".to_owned(), "https://login.microsoftonline.com".to_owned());
            azure_region.insert("resourceGraphURL".to_owned(), "https://graph.microsoft.com/.default".to_owned());
            azure_region
        }
    }
}

fn get_azure_record(azure_record: AzureRecord) -> String {
    return match azure_record {
        AzureRecord::VirtualMachine => "microsoft.compute/virtualmachines".to_owned(),
    }
}

impl ServicePrincipal {

    fn get_params_and_url(&self, token_type: TokenType, azure_scope: Option<String>) -> ([(String, String); 4], String, String, String) {
        let region = &self.region;
        let client_id = &self.client_id;
        let tenant_id = &self.tenant_id;
        let client_secret = &self.client_secret;

        let azure_region_details = get_azure_region_details(region.as_str());

        let authority_url = match token_type {
            TokenType::AzureApiToken => {
                let authority_url: String = azure_region_details.get("authorityURL").unwrap().to_string() + "/" + tenant_id +  "/oauth2/token";
                authority_url
            },
            TokenType::AzureGraphToken => {
                let authority_url: String = azure_region_details.get("authorityURL").unwrap().to_string() + "/" + tenant_id +  "/oauth2/v2.0/token";
                authority_url
            },
        };

        let resource_graph_url = azure_region_details.get("resourceGraphURL").unwrap().to_string();

        let resource_api_url = azure_region_details.get("resourceAPI").unwrap().to_string();

        // println!("authority_url      : {:?}", authority_url);
        // println!("resource_graph_url : {:?}", resource_graph_url);
        // println!("resource_api_url   : {:?}", resource_api_url);

        let my_scope = match azure_scope {
            None => {
                println!("azure_scope is None !");
                "https://graph.microsoft.com/.default".to_owned()
            },
            Some(d) => {
                d.to_owned()
            },
        };

        let params = match token_type {
            TokenType::AzureApiToken => {
                let params = [
                    ("grant_type".to_string(), "client_credentials".to_string()),
                    ("client_id".to_string(), client_id.to_string()),
                    ("client_secret".to_string(), client_secret.to_string()),
                    ("resource".to_string(), "".to_string()),
                ];
                params
            },
            TokenType::AzureGraphToken => {
                let params = [
                    ("grant_type".to_string(), "client_credentials".to_string()),
                    ("client_id".to_string(), client_id.to_string()),
                    ("client_secret".to_string(), client_secret.to_string()),
                    ("scope".to_string(), my_scope.to_string()),
                ];
                params
            }
        };
        let return_data = (params, authority_url, resource_graph_url, resource_api_url);
        // println!("{:#?}", &return_data);
        return return_data;
    }

    fn get_token(&self, token_type: TokenType, azure_scope: Option<String>) -> Result<String, CustomError> {

        let token_type_and_scope = TokenAndScope { token_type: token_type.to_owned(), azure_scope: azure_scope.to_owned() };

        let my_token = match APP_CACHE.get(&token_type_and_scope) {
            None => {
                println!("MISSING_TOKEN in APP_CACHE");
                "".to_owned()
            }
            Some(d) => {
                println!("APP_CACHE : token : {}", d);
                d
            }
        };

        if my_token != "" {
            return Ok(my_token)
        }

        let region = &self.region;
        let client_id = &self.client_id;
        let client_secret = &self.client_secret;
        let region = &self.region;
        let tenant_id = &self.tenant_id;

        let my_scope = Some("https://management.azure.com/.default".to_string());

        let my_scope = match azure_scope {
            None => {
                println!("azure_scope is None !");
                "https://graph.microsoft.com/.default".to_owned()
            },
            Some(d) => {
                d.to_owned()
            },
        };

        let (
            params,
            authority_url,
            resource_graph_url,
            resource_api_url
        ) = self.get_params_and_url(token_type, Some(my_scope));

        let client = reqwest::blocking::Client::new();

        let res = client.post(authority_url).form(&params).send();

        let result =  match res {
            Ok(res) => {
                println!("http response : {:?}", res);
                let data_str = res.text().unwrap_or("N/A".to_string());
                let value: Value = serde_json::from_str(data_str.as_str()).unwrap();

                let my_object = match value.as_object() {
                    None => {
                        println!("no object found");
                    },
                    Some(d) => {
                        if d.contains_key("token_type") == false || d.contains_key("access_token") == false {
                            let msg:String = format!("http response does not contains either the follow keys : (token_type|access_token)");
                            let custom_err = CustomError {
                                err_msg: String::from(msg),
                                err_type: GenericError::MissingDataInResponse
                            };
                            return Err(custom_err)
                        }
                    }
                };

                let token = format!("{} {}", &value["token_type"].as_str().unwrap(), &value["access_token"].as_str().unwrap());
                token
            },
            Err(err) => {
                let msg:String = format!("there was an error http form submit to get token {:?}", err);
                let custom_err = CustomError {
                    err_msg: String::from(msg),
                    err_type: GenericError::InvalidServicePrincipal
                };
                return Err(custom_err)
            }
        };
        println!("Storing api_token : {} , in APP_CACHE", result.to_owned());
        APP_CACHE.insert(token_type_and_scope, result.to_owned());
        Ok(result)
    }

    fn get_total_records(&self, azure_record: AzureRecord) -> u64 {
        let my_scope = Some("https://management.azure.com/.default".to_owned());

        let (
            params,
            authority_url,
            resource_graph_url,
            resource_api_url
        ) = self.get_params_and_url(TokenType::AzureGraphToken, my_scope.to_owned());

        let api_url = resource_api_url.to_owned() + "/providers/Microsoft.ResourceGraph/resources?api-version=2020-04-01-preview";

        println!("api_url for azure_data_explorer : {}", api_url.as_str());

        let empty:Vec<String> = vec![];

        let record_name = get_azure_record(azure_record);

        let total_records_query = format!("Resources | where type == '{}' | summarize count()", record_name);

        let json_payload = json!({
            "subscriptions": empty,
            "query": total_records_query
        });

        println!("json_payload:");
        println!("{:#?}", json_payload);

        let total_records = return match self.get_token(TokenType::AzureGraphToken, my_scope.to_owned()) {
            Ok(d) => {
                let client = reqwest::blocking::Client::new();
                let token = d.to_owned();
                println!("token for azure_data_explorer : {}", token.as_str());
                let now = time::Instant::now();
                let res = match client.post(api_url.as_str()).body(json_payload.to_string()).header(CONTENT_TYPE, "application/json").header(AUTHORIZATION, token.as_str()).send() {
                    Ok(d) => {
                        let elapsed = now.elapsed();
                        let json_str = d.text().unwrap();
                        let json_data: Value = serde_json::from_str(json_str.as_str()).unwrap();
                        println!("{}", serde_json::to_string_pretty(&json_data).unwrap());
                        let total_records = &json_data["data"]["rows"][0][0].as_u64().unwrap();
                        println!("total_records : {}", total_records);
                        println!("TTT : {:.2?}", elapsed);
                        total_records.to_owned()
                    },
                    Err(e) => {
                        println!("error : could not make http request for adx query : {:?}", e);
                        0 as u64
                    },
                };
                res
            }, // Ok
            Err(e) => {
                println!("error : could not fetch token : {:#?}", e);
                0
            }, // Err
        };
        total_records
    }

    fn get_azure_records(&self, azure_record: AzureRecord, top: i32, skip: i32) -> Vec<Value> {

        let my_scope = Some("https://management.azure.com/.default".to_owned());

        let empty_vec:Vec<Value> = Vec::new();

        let (
            params,
            authority_url,
            resource_graph_url,
            resource_api_url
        ) = self.get_params_and_url(TokenType::AzureGraphToken, my_scope.to_owned());

        let api_url = resource_api_url.to_owned() + "/providers/Microsoft.ResourceGraph/resources?api-version=2020-04-01-preview";

        println!("api_url for azure_data_explorer : {}", api_url.as_str());

        let empty:Vec<String> = vec![];

        /*
            0 - 999      -> first page
            1000 - 1999  -> second page
            2000 - 2999  -> third page
            3000 - 3999  -> fourth page

            $top -> The maximum number of rows that the query should return. Overrides the page size when $skipToken property is present.
            $skip -> The number of rows to skip from the beginning of the results. Overrides the next page offset when $skipToken property is present.
        */

        let azure_entity = get_azure_record(azure_record);

        let query = format!("Resources | where type =~ '{}'", azure_entity);

        let json_payload = json!({
            "subscriptions": empty,
            "options" : {
                "$top" : top, // usually fixed at 1000
                "$skip" : skip
            },
            "query": query
        });

        println!("{}", serde_json::to_string_pretty(&json_payload).unwrap());

        let azure_data = match self.get_token(TokenType::AzureGraphToken, my_scope.to_owned()) {
            Ok(d) => {
                let client = reqwest::blocking::Client::new();
                let token = d.to_owned();
                let now = time::Instant::now();
                let res = match client.post(api_url.as_str()).body(json_payload.to_string()).header(CONTENT_TYPE, "application/json").header(AUTHORIZATION, token.as_str()).send() {
                    Ok(d) => {
                        let elapsed = now.elapsed();

                        println!("azure_data_explorer query result:");
                        let json_str = d.text().unwrap();
                        let json_data: Value = serde_json::from_str(json_str.as_str()).unwrap();
                        let rows = &json_data["data"]["rows"];

                        println!("TTT (top : {} , skip : {}) : {:.2?}", top, skip, elapsed);

                        let row_data = return match rows.as_array() {
                            None => {
                                println!("oops : could not find any row data !");
                                empty_vec
                            },
                            Some(d) => {
                                println!("found rows !");
                                d.to_vec()
                            },
                        };
                        row_data
                    },
                    Err(e) => {
                        println!("error : could not make http request for adx query : {:?}", e);
                        empty_vec
                    },
                };
                res.to_owned()
            },
            Err(e) => {
                println!("error : could not fetch token : {:#?}", e);
                empty_vec
            },
        };
        azure_data
    }

    fn get_all_azure_records(&self, azure_record: AzureRecord, records_per_page: i32, write_json_to_file: bool) -> Vec<Value> {
        let total_records_for_virtual_machines = self.get_total_records(azure_record);

        let total_pages = (total_records_for_virtual_machines as f32 / records_per_page as f32).ceil() as i32;

        println!("records_per_page : {}", records_per_page);
        println!("total_pages : {}", total_pages);

        let mut all_records: Vec<Value> = Vec::new();

        let now = time::Instant::now();

        let mut page_list: Vec<i32> = Vec::new();

        for i in 1..=total_pages {
            page_list.push(i);
        }

        let result: Vec<_> = page_list.chunks(8).collect();
        println!("{:#?}", result);

        for page_number in 1..=total_pages {
            println!("fetching data for page_number : {}...", page_number);

            let skip = get_skip_number(records_per_page, page_number);

            let records = self.get_azure_records(AzureRecord::VirtualMachine, records_per_page, skip);
            println!("total records for page_number {} : {}", page_number, records.len());
            for item in records.iter() {
                &all_records.push(item.to_owned());
            }
            println!("iteration in progress : length of all_records : {}", &all_records.len());
            let elapsed = now.elapsed();
            println!("total time taken so far : {:.2?}", elapsed);
        }

        println!("length of all_records : {}", &all_records.len());

        if write_json_to_file {
            let azure_entity = get_azure_record(azure_record);
            let output_file = azure_entity.replace("/", "_").replace(".", "_") + ".json";

            std::fs::write(
            output_file,
        serde_json::to_string_pretty(&all_records).unwrap(),
            ).unwrap();
        }
        all_records
    }
}

enum EnvironmentVarible {
    ClientID,
    ClientSecret,
    TenantID,
    Region,
}

fn get_env_var(env_var: EnvironmentVarible) -> String {
    return match env_var {
        EnvironmentVarible::ClientID => {
            match env::var("CLIENT_ID") {
                Ok(v) => v,
                Err(e) => {
                    println!("environment variable 'CLIENT_ID' is not set !");
                    "".to_owned()
                },
            }
        },
        EnvironmentVarible::ClientSecret => {
            match env::var("CLIENT_SECRET") {
                Ok(v) => v,
                Err(e) => {
                    println!("environment variable 'CLIENT_SECRET' is not set !");
                    "".to_owned()
                },
            }
        },
        EnvironmentVarible::TenantID => {
            match env::var("TENANT_ID") {
                Ok(v) => v,
                Err(e) => {
                    println!("environment variable 'TENANT_ID' is not set !");
                    "".to_owned()
                },
            }
        },
        EnvironmentVarible::Region => {
            match env::var("REGION") {
                Ok(v) => v,
                Err(e) => {
                    println!("environment variable 'TENANT_ID' is not set !");
                    "".to_owned()
                },
            }
        }
    };
}

fn main() {
    println!("Azure Data >");

    let client_id = get_env_var(EnvironmentVarible::ClientID);
    let client_secret = get_env_var(EnvironmentVarible::ClientSecret);
    let tenant_id = get_env_var(EnvironmentVarible::TenantID);
    let region = get_env_var(EnvironmentVarible::Region);

    let service_principal = ServicePrincipal {
        client_id,
        client_secret,
        region,
        tenant_id,
        token_type: TokenType::AzureApiToken,
    };

    // api token
    let api_token = service_principal.get_token(TokenType::AzureApiToken, None).unwrap();
    // graph token
    let graph_token = service_principal.get_token(TokenType::AzureGraphToken, None).unwrap();

    let all_recs = service_principal.get_all_azure_records(AzureRecord::VirtualMachine,1000,true);
    println!("all_recs.len() : {}", all_recs.len())

}

fn get_skip_number(records_per_page: i32, page_number: i32) -> i32 {
    (page_number-1) * records_per_page
}
```

Fetching Virtual Machines : Azure Resource Graph Query (ADX) , Using Cache & Multiple Threads : (FYI : Still, Work In Progress) : Part-4

```rust
use std::collections::HashMap;
use reqwest::{Response, StatusCode};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::{json, Map, Value};
use std::{env,time};
use std::borrow::Borrow;
use std::slice::Chunks;
use moka::sync::Cache;
use std::time::Duration;
use std::sync::mpsc;
use lazy_static::lazy_static;
use std::{thread};
use std::sync::mpsc::SendError;
// use crossbeam_utils::thread;
use crate::AzureRecord::VirtualMachine;


#[derive(Debug)]
pub enum GenericError {
    InvalidServicePrincipal,
    MissingDataInResponse,
}

#[derive(Debug)]
struct CustomError {
    err_type: GenericError,
    err_msg: String
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum TokenType {
    AzureApiToken,
    AzureGraphToken,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum AzureRecord {
    VirtualMachine
}

#[derive(Clone)]
struct ServicePrincipal {
    client_id: String,
    client_secret: String,
    tenant_id: String,
    region: String,
    token_type: TokenType
}

// global variable

lazy_static! {
    static ref APP_CACHE:Cache<TokenAndScope, String> = Cache::builder()
        // Time to live (TTL): 45 minutes
        .time_to_live(Duration::from_secs(45 * 60))
        // Time to idle (TTI):  45 minutes
        .time_to_idle(Duration::from_secs(45 * 60))
        // Create the cache.
        .build();

    static ref APP_CACHE_TOTAL_RECORDS:Cache<AzRecord, u64> = Cache::builder()
        // Time to live (TTL): 45 minutes
        .time_to_live(Duration::from_secs(45 * 60))
        // Time to idle (TTI):  45 minutes
        .time_to_idle(Duration::from_secs(45 * 60))
        // Create the cache.
        .build();
}

impl PartialEq<Self> for AzRecord {
    fn eq(&self, other: &Self) -> bool {
        self.azure_record == other.azure_record
    }
}

#[derive(Hash, Clone, Eq)]
struct AzRecord {
    azure_record: AzureRecord
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct TokenAndScope {
    token_type: TokenType,
    azure_scope: Option<String>
}

// microsoft public api endpoints

fn get_azure_region_details(region: &str) -> HashMap<String, String> {
    let mut azure_region: HashMap<String, String> = HashMap::new();

    return match region {
        "america" => {
            azure_region.insert("resourceAPI".to_owned(), "https://management.azure.com".to_owned());
            azure_region.insert("authorityURL".to_owned(), "https://login.microsoftonline.com".to_owned());
            azure_region.insert("resourceGraphURL".to_owned(), "https://graph.microsoft.com/.default".to_owned());
            azure_region
        },
        "china" => {
            azure_region.insert("resourceAPI".to_owned(), "https://management.chinacloudapi.cn".to_owned());
            azure_region.insert("authorityURL".to_owned(), "https://login.partner.microsoftonline.cn".to_owned());
            azure_region.insert("resourceGraphURL".to_owned(), "https://microsoftgraph.chinacloudapi.cn/.default".to_owned());
            azure_region
        },
        _ => {
            azure_region.insert("resourceAPI".to_owned(), "https://management.azure.com".to_owned());
            azure_region.insert("authorityURL".to_owned(), "https://login.microsoftonline.com".to_owned());
            azure_region.insert("resourceGraphURL".to_owned(), "https://graph.microsoft.com/.default".to_owned());
            azure_region
        }
    }
}

fn get_azure_record(azure_record: AzureRecord) -> String {
    return match azure_record {
        AzureRecord::VirtualMachine => "microsoft.compute/virtualmachines".to_owned(),
    }
}

impl ServicePrincipal {

    fn get_params_and_url(&self, token_type: TokenType, azure_scope: Option<String>) -> ([(String, String); 4], String, String, String) {
        let region = &self.region;
        let client_id = &self.client_id;
        let tenant_id = &self.tenant_id;
        let client_secret = &self.client_secret;

        let azure_region_details = get_azure_region_details(region.as_str());

        let authority_url = match token_type {
            TokenType::AzureApiToken => {
                let authority_url: String = azure_region_details.get("authorityURL").unwrap().to_string() + "/" + tenant_id +  "/oauth2/token";
                authority_url
            },
            TokenType::AzureGraphToken => {
                let authority_url: String = azure_region_details.get("authorityURL").unwrap().to_string() + "/" + tenant_id +  "/oauth2/v2.0/token";
                authority_url
            },
        };

        let resource_graph_url = azure_region_details.get("resourceGraphURL").unwrap().to_string();

        let resource_api_url = azure_region_details.get("resourceAPI").unwrap().to_string();

        // println!("authority_url      : {:?}", authority_url);
        // println!("resource_graph_url : {:?}", resource_graph_url);
        // println!("resource_api_url   : {:?}", resource_api_url);

        let my_scope = match azure_scope {
            None => {
                println!("azure_scope is None !");
                "https://graph.microsoft.com/.default".to_owned()
            },
            Some(d) => {
                d.to_owned()
            },
        };

        let params = match token_type {
            TokenType::AzureApiToken => {
                let params = [
                    ("grant_type".to_string(), "client_credentials".to_string()),
                    ("client_id".to_string(), client_id.to_string()),
                    ("client_secret".to_string(), client_secret.to_string()),
                    ("resource".to_string(), "".to_string()),
                ];
                params
            },
            TokenType::AzureGraphToken => {
                let params = [
                    ("grant_type".to_string(), "client_credentials".to_string()),
                    ("client_id".to_string(), client_id.to_string()),
                    ("client_secret".to_string(), client_secret.to_string()),
                    ("scope".to_string(), my_scope.to_string()),
                ];
                params
            }
        };
        let return_data = (params, authority_url, resource_graph_url, resource_api_url);
        // println!("{:#?}", &return_data);
        return return_data;
    }

    fn get_token(&self, token_type: TokenType, azure_scope: Option<String>) -> Result<String, CustomError> {

        let token_type_and_scope = TokenAndScope { token_type: token_type.to_owned(), azure_scope: azure_scope.to_owned() };

        let my_token = match APP_CACHE.get(&token_type_and_scope) {
            None => {
                println!("MISSING_TOKEN in APP_CACHE");
                "".to_owned()
            }
            Some(d) => {
                // println!("APP_CACHE : token : {}", d);
                d
            }
        };

        if my_token != "" {
            return Ok(my_token)
        }

        let region = &self.region;
        let client_id = &self.client_id;
        let client_secret = &self.client_secret;
        let region = &self.region;
        let tenant_id = &self.tenant_id;

        let my_scope = Some("https://management.azure.com/.default".to_string());

        let my_scope = match azure_scope {
            None => {
                println!("azure_scope is None !");
                "https://graph.microsoft.com/.default".to_owned()
            },
            Some(d) => {
                d.to_owned()
            },
        };

        let (
            params,
            authority_url,
            resource_graph_url,
            resource_api_url
        ) = self.get_params_and_url(token_type, Some(my_scope));

        let client = reqwest::blocking::Client::new();

        let res = client.post(authority_url).form(&params).send();

        let result =  match res {
            Ok(res) => {
                println!("http response : {:?}", res);
                let data_str = res.text().unwrap_or("N/A".to_string());
                let value: Value = serde_json::from_str(data_str.as_str()).unwrap();

                let my_object = match value.as_object() {
                    None => {
                        println!("no object found");
                    },
                    Some(d) => {
                        if d.contains_key("token_type") == false || d.contains_key("access_token") == false {
                            let msg:String = format!("http response does not contains either the follow keys : (token_type|access_token)");
                            let custom_err = CustomError {
                                err_msg: String::from(msg),
                                err_type: GenericError::MissingDataInResponse
                            };
                            return Err(custom_err)
                        }
                    }
                };

                let token = format!("{} {}", &value["token_type"].as_str().unwrap(), &value["access_token"].as_str().unwrap());
                token
            },
            Err(err) => {
                let msg:String = format!("there was an error http form submit to get token {:?}", err);
                let custom_err = CustomError {
                    err_msg: String::from(msg),
                    err_type: GenericError::InvalidServicePrincipal
                };
                return Err(custom_err)
            }
        };
        println!("Storing api_token : {} , in APP_CACHE", result.to_owned());
        APP_CACHE.insert(token_type_and_scope, result.to_owned());
        Ok(result)
    }

    fn get_total_records(&self, azure_record: AzureRecord) -> u64 {

        let az_record = AzRecord { azure_record: azure_record.to_owned() };

        let total_records_from_cache = match APP_CACHE_TOTAL_RECORDS.get(&az_record) {
            None => {
                println!("__FUNC__ : get_total_records() : MISSING_TOTAL_RECORDS in APP_CACHE_TOTAL_RECORDS");
                0
            }
            Some(d) => {
                println!("__FUNC__ : get_total_records() : APP_CACHE_TOTAL_RECORDS : total_records_from_cache : {}", d);
                d
            }
        };

        if total_records_from_cache != 0 {
            println!("__FUNC__ : get_total_records() : total_records_from_cache : {}", total_records_from_cache);
            return total_records_from_cache
        }

        println!("__FUNC__ : data not found in cache !");

        //----------------------------------------------------------------------------------------

        let my_scope = Some("https://management.azure.com/.default".to_owned());

        let (
            params,
            authority_url,
            resource_graph_url,
            resource_api_url
        ) = self.get_params_and_url(TokenType::AzureGraphToken, my_scope.to_owned());

        let api_url = resource_api_url.to_owned() + "/providers/Microsoft.ResourceGraph/resources?api-version=2020-04-01-preview";

        println!("api_url for azure_data_explorer : {}", api_url.as_str());

        let empty:Vec<String> = vec![];

        let record_name = get_azure_record(azure_record);

        let total_records_query = format!("Resources | where type == '{}' | summarize count()", record_name);

        let json_payload = json!({
            "subscriptions": empty,
            "query": total_records_query
        });

        println!("json_payload:");
        println!("{:#?}", json_payload);

        let total_records = match self.get_token(TokenType::AzureGraphToken, my_scope.to_owned()) {
            Ok(d) => {
                let client = reqwest::blocking::Client::new();
                let token = d.to_owned();
                println!("token for azure_data_explorer : {}", token.as_str());
                let now = time::Instant::now();
                let res = match client.post(api_url.as_str()).body(json_payload.to_string()).header(CONTENT_TYPE, "application/json").header(AUTHORIZATION, token.as_str()).send() {
                    Ok(d) => {
                        let elapsed = now.elapsed();
                        let json_str = d.text().unwrap();
                        let json_data: Value = serde_json::from_str(json_str.as_str()).unwrap();
                        println!("{}", serde_json::to_string_pretty(&json_data).unwrap());
                        let total_records = &json_data["data"]["rows"][0][0].as_u64().unwrap();
                        println!("total_records : {}", total_records);
                        println!("TTT : {:.2?}", elapsed);
                        total_records.to_owned()
                    },
                    Err(e) => {
                        println!("error : could not make http request for adx query : {:?}", e);
                        0 as u64
                    },
                };
                res
            }, // Ok
            Err(e) => {
                println!("error : could not fetch token : {:#?}", e);
                0
            }, // Err
        };
        APP_CACHE_TOTAL_RECORDS.insert(az_record, total_records.to_owned());
        println!("__FUNC__ : get_total_records() : total_records : {}", total_records_from_cache);
        total_records
    }

    fn get_azure_records(&self, azure_record: AzureRecord, top: i32, skip: i32) -> Vec<Value> {

        let my_scope = Some("https://management.azure.com/.default".to_owned());

        let empty_vec:Vec<Value> = Vec::new();

        let (
            params,
            authority_url,
            resource_graph_url,
            resource_api_url
        ) = self.get_params_and_url(TokenType::AzureGraphToken, my_scope.to_owned());

        let api_url = resource_api_url.to_owned() + "/providers/Microsoft.ResourceGraph/resources?api-version=2020-04-01-preview";

        println!("api_url for azure_data_explorer : {}", api_url.as_str());

        let empty:Vec<String> = vec![];

        /*
            0 - 999      -> first page
            1000 - 1999  -> second page
            2000 - 2999  -> third page
            3000 - 3999  -> fourth page

            $top -> The maximum number of rows that the query should return. Overrides the page size when $skipToken property is present.
            $skip -> The number of rows to skip from the beginning of the results. Overrides the next page offset when $skipToken property is present.
        */

        let azure_entity = get_azure_record(azure_record);

        let query = format!("Resources | where type =~ '{}'", azure_entity);

        let json_payload = json!({
            "subscriptions": empty,
            "options" : {
                "$top" : top, // usually fixed at 1000
                "$skip" : skip
            },
            "query": query
        });

        println!("{}", serde_json::to_string_pretty(&json_payload).unwrap());

        let azure_data = match self.get_token(TokenType::AzureGraphToken, my_scope.to_owned()) {
            Ok(d) => {
                let client = reqwest::blocking::Client::new();
                let token = d.to_owned();
                let now = time::Instant::now();
                let res = match client.post(api_url.as_str()).body(json_payload.to_string()).header(CONTENT_TYPE, "application/json").header(AUTHORIZATION, token.as_str()).send() {
                    Ok(d) => {
                        let elapsed = now.elapsed();

                        println!("azure_data_explorer query result:");
                        let json_str = d.text().unwrap();
                        let json_data: Value = serde_json::from_str(json_str.as_str()).unwrap();
                        let rows = &json_data["data"]["rows"];

                        println!("TTT (top : {} , skip : {}) : {:.2?}", top, skip, elapsed);

                        let row_data = return match rows.as_array() {
                            None => {
                                println!("oops : could not find any row data !");
                                empty_vec
                            },
                            Some(d) => {
                                println!("found rows !");
                                d.to_vec()
                            },
                        };
                        row_data
                    },
                    Err(e) => {
                        println!("error : could not make http request for adx query : {:?}", e);
                        empty_vec
                    },
                };
                res.to_owned()
            },
            Err(e) => {
                println!("error : could not fetch token : {:#?}", e);
                empty_vec
            },
        };
        azure_data
    }

    fn get_total_pages_for_all_records(&self, azure_record: AzureRecord, records_per_page: i32) -> i32 {
        let total_records_for_virtual_machines = self.get_total_records(azure_record);
        let total_pages = (total_records_for_virtual_machines as f32 / records_per_page as f32).ceil() as i32;
        println!("__FUNC__ : get_total_pages_for_all_records() : {}", total_pages);
        total_pages
    }

    fn get_all_azure_records_for_page(&self, page_number: i32, azure_record: AzureRecord, records_per_page: i32) -> Vec<Value> {
        let total_pages = self.get_total_pages_for_all_records(azure_record, records_per_page);
        let mut all_records: Vec<Value> = Vec::new();
        let now = time::Instant::now();
        println!("__FUNC__: get_all_azure_records_for_page() : fetching data for page_number : {}...", page_number);
        let skip = get_skip_number(records_per_page, page_number);
        let records = self.get_azure_records(AzureRecord::VirtualMachine, records_per_page, skip);
        println!("total records for page_number {} : {}", page_number, records.len());
        for item in records.iter() {
            &all_records.push(item.to_owned());
        }
        let elapsed = now.elapsed();
        println!("__FUNC__ : get_all_azure_records_for_page() : total time taken for page_number {} : {:.2?}", page_number, elapsed);
        println!("__FUNC__ : get_all_azure_records_for_page() : all_records.len() : {}", &all_records.len());
        all_records
    }

    fn get_all_azure_records_for_pages(&self, page_numbers: Vec<i32>, azure_record: AzureRecord, records_per_page: i32) -> Vec<Value> {
        let total_pages = self.get_total_pages_for_all_records(azure_record, records_per_page);
        let mut all_records: Vec<Value> = Vec::new();
        let now = time::Instant::now();

        for page_number in page_numbers {
            println!("__FUNC__ : get_all_azure_records_for_pages() : fetching data for page_number : {}...", page_number);
            let skip = get_skip_number(records_per_page, page_number);
            let records = self.get_azure_records(AzureRecord::VirtualMachine, records_per_page, skip);

            println!("__FUNC__ : get_all_azure_records_for_pages() : total records for page_number {} : {}", page_number, records.len());
            for item in records.iter() {
                &all_records.push(item.to_owned());
            }
            let elapsed = now.elapsed();
            println!("__FUNC__ : get_all_azure_records_for_pages() : total time taken so far : {:.2?}", elapsed);
        }
        println!("__FUNC__ : get_all_azure_records_for_pages() : length of all_records : {}", &all_records.len());
        all_records
    }

    fn get_all_azure_records(&self, azure_record: AzureRecord, records_per_page: i32, write_json_to_file: bool) -> Vec<Value> {

        let total_pages = self.get_total_pages_for_all_records(azure_record, records_per_page);

        let mut all_records: Vec<Value> = Vec::new();

        let now = time::Instant::now();

        for page_number in 1..=total_pages {
            let records = self.get_all_azure_records_for_page(page_number, azure_record, records_per_page);
            println!("total records for page_number {} : {}", page_number, records.len());
            for item in records.iter() {
                &all_records.push(item.to_owned());
            }
            let elapsed = now.elapsed();
            println!("__FUNC__ : get_all_azure_records() : total time taken so far : {:.2?}", elapsed);
        }

        println!("__FUNC__ : get_all_azure_records() : length of all_records : {}", &all_records.len());

        if write_json_to_file {
            let azure_entity = get_azure_record(azure_record);
            let output_file = azure_entity.replace("/", "_").replace(".", "_") + ".json";

            std::fs::write(
            output_file,
        serde_json::to_string_pretty(&all_records).unwrap(),
            ).unwrap();
        }
        all_records
    }
}

enum EnvironmentVarible {
    ClientID,
    ClientSecret,
    TenantID,
    Region,
}

fn get_env_var(env_var: EnvironmentVarible) -> String {
    return match env_var {
        EnvironmentVarible::ClientID => {
            match env::var("CLIENT_ID") {
                Ok(v) => v,
                Err(e) => {
                    println!("environment variable 'CLIENT_ID' is not set !");
                    "".to_owned()
                },
            }
        },
        EnvironmentVarible::ClientSecret => {
            match env::var("CLIENT_SECRET") {
                Ok(v) => v,
                Err(e) => {
                    println!("environment variable 'CLIENT_SECRET' is not set !");
                    "".to_owned()
                },
            }
        },
        EnvironmentVarible::TenantID => {
            match env::var("TENANT_ID") {
                Ok(v) => v,
                Err(e) => {
                    println!("environment variable 'TENANT_ID' is not set !");
                    "".to_owned()
                },
            }
        },
        EnvironmentVarible::Region => {
            match env::var("REGION") {
                Ok(v) => v,
                Err(e) => {
                    println!("environment variable 'TENANT_ID' is not set !");
                    "".to_owned()
                },
            }
        }
    };
}

fn main() {
    println!("Azure Data >");

    let client_id = get_env_var(EnvironmentVarible::ClientID);
    let client_secret = get_env_var(EnvironmentVarible::ClientSecret);
    let tenant_id = get_env_var(EnvironmentVarible::TenantID);
    let region = get_env_var(EnvironmentVarible::Region);

    let mut service_principal = ServicePrincipal {
        client_id,
        client_secret,
        region,
        tenant_id,
        token_type: TokenType::AzureApiToken,
    };

    // api token
    let api_token = service_principal.get_token(TokenType::AzureApiToken, None).unwrap();
    // graph token
    let graph_token = service_principal.get_token(TokenType::AzureGraphToken, None).unwrap();

    let all_recs = get_all_azure_records_in_parallel(&service_principal, AzureRecord::VirtualMachine, 1000, true);

}

fn get_skip_number(records_per_page: i32, page_number: i32) -> i32 {
    (page_number-1) * records_per_page
}


fn get_all_azure_records_in_parallel(service_principal: &ServicePrincipal, azure_record: AzureRecord, records_per_page: i32, write_json_to_file: bool) -> Vec<Value> {

    println!("__FUNC__ : get_all_azure_records_in_parallel() ...");

    let now = time::Instant::now();

    let total_pages = service_principal.get_total_pages_for_all_records(azure_record, records_per_page);

    let mut page_numbers: Vec<i32> = Vec::new();

    for i in 1..=total_pages {
        page_numbers.push(i);
    }

    let (sender, receiver) = mpsc::channel();

    // let result_pages: Vec<_> = page_numbers.chunks(8).collect();

    let v_chunked: Vec<Vec<i32>> = page_numbers.chunks(8).map(|x| x.to_vec()).collect();

    println!("v_chunked : {:?}", v_chunked);

    let mut all_records: Vec<Value> = Vec::new();

    // let mut threads = vec![];

    let total_count_of_chunks_of_vectors = v_chunked.len();

    for page_numbers in v_chunked {

        let sender = sender.clone();

        // let spn = service_principal.clone();

        let spn = service_principal.to_owned();

        thread::spawn(move || {

            let records = spn.get_all_azure_records_for_pages(page_numbers, azure_record, records_per_page);

            println!("__VEC_LENGTH__ : {}", records.len());

            let send_data = match sender.send(records) {
                Ok(d) => {
                    println!("__FUNC__ : get_all_azure_records_in_parallel() : data was sent from sender to receiver.")
                },
                Err(e) => {
                    println!("__FUNC__ : error : get_all_azure_records_in_parallel() : data send failed : {:#?}", e);
                }
            };

        });
    }

    drop(sender);

    for _ in 0..total_count_of_chunks_of_vectors {
        let records = receiver.recv().unwrap();

        println!("__FUNC__ : _received {}  records from (receiver) ", records.len());
        for rec in records.iter() {
            &all_records.push(rec.to_owned());
        }
        println!("__FUNC__ : _total Length of all_records {}", all_records.len());
    }

    println!("__FUNC__ : _final : length of all_records : {}", &all_records.len());


    if write_json_to_file {
        let azure_entity = get_azure_record(azure_record);
        let output_file = azure_entity.replace("/", "_").replace(".", "_") + ".json";

        std::fs::write(
        output_file,
    serde_json::to_string_pretty(&all_records).unwrap(),
        ).unwrap();
    }

    let elapsed = now.elapsed();

    println!("__FUNC__ : get_all_azure_records_in_parallel() : final_time : {:.2?}", elapsed);

    all_records
}
```