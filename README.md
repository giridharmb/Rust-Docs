#### Notes

> FYI : All Of This Code Is `Not Proprietary` & Is Documented Publicly On The Web

Rust Playground

https://play.rust-lang.org/

Please have a look the following file for code snippets/samples

#### [src/main.rs](https://github.com/giridharmb/rust-app1/blob/master/src/main.rs)

#### [Cloud : OpenStack | Azure | GCP](https://github.com/giridharmb/rust-app1/blob/master/CLOUD.md)

#### [File Uploader : Server And Client](https://github.com/giridharmb/rust-app1/blob/master/FileUploader.md)

#### [Actix Web & PostgreSQL DB & jQuery Data Table With Server Side Rendering (Pagination)](https://github.com/giridharmb/rust-app1/blob/master/Rust-Data-Table.md)

[Openstack Hypervisor List With HTTP Retry](#openstack-hypervisor-list-with-http-retry)

[Actix REST API With PostgreSQL Backend](#actix-rest-api-with-postgresql-backend)

[Sanitize String And Split String](#sanitize-string-and-split-string)

[Sanitize String - Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=3b5289c32acfb26e9d92f6e172f6aa97)

[Actix REST API With Advanced Query And PostgreSQL Backend](#actix-rest-api-with-advanced-query-and-postgresql-backend)

[Async Functions And Channels](#async-functions-and-channels)

[Fetch Openstack Keystone Token With HTTP Request Retry](#fetch-openstack-keystone-token-with-http-request-retry)

[Using Map On ASYNC Functions](#using-map-on-async-functions)

[Struct And PostgreSQL Table Mapping](#struct-and-postgresql-table-mapping)

[Using ARC To Clone Objects](#using-arc-to-clone-objects)

[Measure PostgreSQL Read Write Performance](#measure-postgresql-read-write-performance)

[Factory Design Pattern V1](#factory-design-pattern-v1)

[Write CSV File](#write-csv-file)

<hr/>

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

#### The question `?` mark operator

The question mark operator (`?`) unwraps valid values or returns errornous values, <br/>
propagating them to the calling function. It is a unary postfix operator that can <br/>
only be applied to the types `Result<T, E>` and `Option<T>`.<br/>

When applied to values of the `Result<T, E>` type, it propagates errors. <br/>
If the value is `Err(e)`, then it will return `Err(From::from(e))` from the <br/>
enclosing function or closure. <br/>
If applied to `Ok(x)`, then it will unwrap the value to evaulate to `x`.<br/>

```rust
use std::num::ParseIntError;

fn try_to_parse() -> Result<i32, ParseIntError> {
    let x: i32 = "123".parse()?; // x = 123
    let y: i32 = "24a".parse()?; // returns an Err() immediately
    Ok(x + y)                    // Doesn't run.
}

fn main() {
    let res = try_to_parse();
    println!("{:?}", res);
}
```

Output

```bash
Err(ParseIntError { kind: InvalidDigit })
```

#### Another Way To Leverage `?` Question Mark Operator

```rust
use std::error::Error;

fn main() {

    let result_division = match divide(3.0, 4.0) {
        Ok(d) => {
            println!("result of division : {:?}", d);
            d
        }
        Err(e) => {
            println!("error : {:?}", e);
            0.0
        }
    };

    let result_division = match divide(3.0, 0.0) {
        Ok(d) => {
            println!("result of division : {:?}", d);
            d
        }
        Err(e) => {
            println!("error : {:?}", e);
            0.0
        }
    };

    //--------------------------------------------------------------------------------

    let result_square = match square(3) {
        None => {
            println!("result of square : got no data !");
        }
        Some(d) => {
            println!("result of square : {:?}", d);
        }
    };

    let result_square = match square(0) {
        None => {
            println!("result of square : got no data !");
        }
        Some(d) => {
            println!("result of square : {:?}", d);
        }
    };

    //--------------------------------------------------------------------------------
}

fn divide(a: f32, b: f32) -> Result<f32, Box<dyn std::error::Error>> {
    if b <= 0.0 {
        return Err(Box::try_from(String::from("cannot divide by 0")).unwrap());
    }
    Ok(a/b)
}

fn square(a: i32) -> Option<i32> {
    if a == 0 {
        return None
    }
    Some(a * a)
}
```

Output

```bash
result of division : 0.75
error : "cannot divide by 0"
result of square : 9
result of square : got no data !
```

#### Using `?` operator (for `Result` and `Option` Types)

```rust
fn main() {
    println!("result of division-1 : {:?}", perform_math_operation_divide(3.0, 4.0));
    println!("result of division-1 : {:?}", perform_math_operation_divide(3.0, 0.0));

    println!("result of square-1 : {:?}", perform_math_operation_square(2));
    println!("result of square-2 : {:?}", perform_math_operation_square(0));
}

fn perform_math_operation_divide(a: f32, b: f32) -> Result<f32, Box<dyn std::error::Error>> {
    let result_division = divide(a, b)?;
    println!("result_division : {:?}", result_division);
    Ok(result_division)
}

fn perform_math_operation_square(a: i32) -> Option<i32> {
    let result_square = square(a)?;
    println!("result_square : {:?}", result_square);
    Some(result_square)
}

fn divide(a: f32, b: f32) -> Result<f32, Box<dyn std::error::Error>> {
    if b <= 0.0 {
        return Err(Box::try_from(String::from("cannot divide by 0")).unwrap());
    }
    Ok(a/b)
}

fn square(a: i32) -> Option<i32> {
    if a == 0 {
        return None
    }
    Some(a * a)
}
```

Output

```bash
result_division : 0.75
result of division-1 : Ok(0.75)
result of division-1 : Err("cannot divide by 0")
result_square : 4
result of square-1 : Some(4)
result of square-2 : None
```

#### Threads vs Async

`Async` vs `threads` in Rust

```
The primary alternative to async in Rust is  using OS threads, either directly
through  std::thread  or  indirectly  through  a thread  pool. Migrating  from
threads to async  or  vice versa typically  requires  major refactoring  work,
both in  terms  of  implementation  and (if  you are  building  a library) any
exposed public interfaces. As such,  picking  the model  that suits your needs
early can save a lot of development time.

OS threads are  suitable for a small number of tasks, since  threads come with
CPU  and  memory overhead.  Spawning and switching between  threads  is  quite
expensive  as  even  idle threads  consume  system  resources.  A thread  pool
library can help mitigate  some of these costs, but not all. However,  threads
let  you reuse existing synchronous  code without significant code  changes—no
particular programming model is required.  In  some operating systems, you can
also change the priority of a thread,  which is  useful for drivers and  other
latency sensitive applications.

Async  provides significantly  reduced CPU and memory overhead, especially for
workloads  with a  large  amount  of  IO-bound  tasks,  such  as  servers  and
databases.  All else equal, you can have  orders  of magnitude more tasks than
OS threads,  because  an async  runtime  uses  a  small  amount of (expensive)
threads  to handle  a  large  amount  of  (cheap)  tasks. However, async  Rust
results in larger binary blobs  due to the state machines generated from async
functions and since each executable  bundles an async runtime.

On  a  last note, asynchronous programming is not  better  than  threads,  but
different.  If you don't need async for performance reasons, threads can often
be the simpler alternative.
```

#### Boxing : Creating Objects In Heap Using `Box<T>`

```rust
use std::mem;

#[allow(dead_code)]
struct Rectangle {
    width: f64,
    height: f64,
}

#[allow(dead_code)]
fn object() -> Rectangle {
    Rectangle { width: 1.0, height: 1.0}
}

#[allow(dead_code)]
fn new_object() -> Box<Rectangle> {
    // Allocate this on the heap, and return a pointer to it
    Box::new(Rectangle { width: 1.0, height: 1.0})

}

fn main() {
    // Stack allocated variables
    let my_rect: Rectangle = object();

    // Heap allocated rectangle
    let boxed_rectangle: Box<Rectangle> = new_object();

    println!("Rectangle occupies {} bytes on the stack",
             mem::size_of_val(&my_rect));

    println!("Rectangle occupies {} bytes on the heap",
             mem::size_of_val(&boxed_rectangle));
}
```

Output

```bash
Rectangle occupies 16 bytes on the stack
Rectangle occupies 8 bytes on the heap
```

Serialize & Deserialize Data

```toml
[package]
name = "my-crate"
version = "0.1.0"
authors = ["Me <user@rust-lang.org>"]

[dependencies]
serde = { version = "1.0", features = ["derive"] }

# serde_json is just for the example, not required in general
serde_json = "1.0"
```

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };

    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}
```

Output

```bash
$ cargo run
serialized = {"x":1,"y":2}
deserialized = Point { x: 1, y: 2 }
```

Troubleshooting

Sometimes you may see compile-time errors that tell you:

> the trait `serde::ser::Serialize` is not implemented for `...`

Even though the struct or enum clearly has `#[derive(Serialize)]` on it.

This almost always means that you are using libraries that depend on incompatible versions of Serde. <br/>
You may be depending on serde 1.0 in your Cargo.toml but using some other library that depends on serde 0.9. <br/>
So the Serialize trait from serde 1.0 may be implemented, but the library expects an implementation of the <br/>
Serialize trait from serde 0.9. From the Rust compiler's perspective these are totally different traits.<br/>

The fix is to upgrade or downgrade libraries as appropriate until the Serde versions match. <br/>
The cargo tree -d command is helpful for finding all the places that duplicate dependencies are being pulled in.<br/>

#### Map Function & Iter (Iterator)

```rust
fn main() {
    let str_numbers: Vec<&str> = vec!["1", "2", "3"];
    let numbers: Vec<u32> = str_numbers.iter().map(|str_number| str_number.parse::<u32>().unwrap()).collect();
    print!("{:?}", numbers)
}
```

Output

```bash
[1, 2, 3]
```

In Case Of Incorrect Data Types, Use `unwrap_or( <default_value> )`

Using `i32` below, becuase we are returning `-1` (in case of `unwrap()` errors)

```rust
fn main() {
    let str_numbers: Vec<&str> = vec!["1", "2", "3", "xyz"];
    let numbers: Vec<i32> = str_numbers
        .iter()
        .map(|str_number| str_number.parse::<i32>().unwrap_or(-1))
        .collect();
    print!("{:?}", numbers)
}
```

Output

```bash
[1, 2, 3, -1]
```

#### Cross Compile : Build For Linux

```bash
rustup target add x86_64-unknown-linux-gnu
```

Build

```bash
cargo build --release --target=x86_64-unknown-linux-gnu
```

#### Rust Lifetimes Explained

[![Everything Is AWESOME](https://img.youtube.com/vi/juIINGuZyBc/maxresdefault.jpg)](https://www.youtube.com/watch?v=juIINGuZyBc "Rust Lifetimes")

#### Calling Async Functions From Non Async Functions : Tokio Runtime

Make sure you export Google Service Account JSON File (Environment Variable)

```bash
export GOOGLE_APPLICATION_CREDENTIALS="$HOME/my-account.json"
```

`Cargo.toml`

```toml
[package]
name = "gcs"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
google-cloud-storage = "0.10.0"
google-cloud-default = { version = "0.1.1", features = ["storage"] }
simple_logger = { version = "4.0.0", features = ["colors", "timestamps"] }
tokio = { version = "1.24.1", features = ["full"] }
log = { version = "0.4", features = ["std", "serde"] }
futures = "0.3.27"
```

`src/main.rs`

```rust
use google_cloud_storage::client::Client;
use google_cloud_storage::sign::SignedURLOptions;
use google_cloud_storage::sign::SignedURLMethod;
use google_cloud_storage::http::Error;
use google_cloud_storage::http::objects::download::Range;
use google_cloud_storage::http::objects::get::GetObjectRequest;
use google_cloud_storage::http::objects::upload::UploadObjectRequest;
use tokio::task::JoinHandle;
use std::fs::File;
use std::future::Future;
use std::io::BufReader;
use std::io::Read;
use google_cloud_default::WithAuthExt;
use google_cloud_storage::client::ClientConfig;
use google_cloud_storage::http::objects::Object;
use google_cloud_storage::http::objects::upload::{Media, UploadType};
use futures::executor::block_on;
use google_cloud_storage::http::buckets::Bucket;
use tokio::task;
use tokio::runtime::Runtime;

use std::io;


fn upload_data_to_gcs() {
    let rt  = Runtime::new().unwrap();
    rt.block_on(async {
        // Create client.
        let config = ClientConfig::default().with_auth().await.unwrap();
        let mut client = Client::new(config);


        let mut bytes: Vec<u8> = Vec::new();
        for byte in File::open("mac.webp").unwrap().bytes() {
            bytes.push(byte.unwrap());
        }

        let f = File::open("mac.webp").unwrap();
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();

        // Read file into vector.
        reader.read_to_end(&mut buffer).unwrap();

        // Upload the file
        let upload_type = UploadType::Simple(Media::new("mac.webp"));

        let uploaded = client.upload_object(&UploadObjectRequest {
            bucket: "rubucket".to_string(),
            ..Default::default()
        }, buffer, &upload_type).await;
    });
}

fn main() {
    upload_data_to_gcs();
}
```

#### Reading TOML (Config) Files

File : `Cargo.toml`

```toml
[package]
name = "toml-read"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
toml = "0.7.3"
serde = "1.0.158"
serde_derive = "1.0.158"
```

File : `app_config_azure.toml`

```toml
[[azure_entity_spn]]
azure_client_id = "00000000-0000-0000-0000-000000000111"
azure_client_secret = "<RANDOM_STR_001>"
azure_tenant_id = "00000000-0000-0000-0000-000000000999"
azure_region_name = "america"
entity_type = "microsoft.compute/virtualmachines"

[[azure_entity_spn]]
azure_client_id = "00000000-0000-0000-0000-000000000222"
azure_client_secret = "<RANDOM_STR_002>"
azure_tenant_id = "00000000-0000-0000-0000-000000000999"
azure_region_name = "america"
entity_type = "microsoft.compute/virtualmachinescalesets/virtualmachines"
```

File : `app_config_global.toml`

```toml
[gcpbqmetadata]
gcp_project_name = "my-test-gcp-project"
gcp_bq_dataset = "my-dataset"
gcp_bq_table = "my-table-001"

[microsoftcomputevirtualmachines]
azure_client_id = "00000000-0000-0000-0000-000000000111"
azure_client_secret = "<RANDOM_STR_001>"
azure_tenant_id = "00000000-0000-0000-0000-000000000999"
azure_region_name = "america"
```

File : `src/main.rs`

```rust
use std::collections::HashMap;
// Import the required dependencies.
use serde_derive::Deserialize;
use std::fs;
use std::process::exit;
use toml;
use toml::from_str;

// Top level struct to hold the TOML data.
#[derive(Deserialize,Debug)]
struct Data {
    microsoftcomputevirtualmachines: AzureMetadata,
    gcpbqmetadata: GCPMetadata,
}

#[derive(Deserialize,Debug)]
struct AzureMetadata {
    azure_client_id: String,
    azure_client_secret: String,
    azure_tenant_id: String,
    azure_region_name: String,
}

#[derive(Deserialize,Debug)]
struct GCPMetadata {
    gcp_project_name: String,
    gcp_bq_dataset: String,
    gcp_bq_table: String,
}

#[derive(Deserialize,Debug)]
struct AzureMetadataItem {
    azure_client_id: String,
    azure_client_secret: String,
    azure_tenant_id: String,
    azure_region_name: String,
    entity_type: String,
}

fn get_toml_file_contents(file_name: String) -> String {
    // Variable that holds the filename as a `&str`.
    let filename = file_name.as_str();

    // Read the contents of the file using a `match` block
    // to return the `data: Ok(c)` as a `String`
    // or handle any `errors: Err(_)`.
    let contents = match fs::read_to_string(filename) {
        // If successful return the files text as `contents`.
        // `c` is a local variable.
        Ok(c) => c,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Could not read file `{}`", filename);
            // Exit the program with exit code `1`.
            exit(1);
        }
    };
    contents
}

fn main() {

    let file_name_toml_gcp = "app_config_global.toml";

    let contents_toml_config_gcp = get_toml_file_contents(file_name_toml_gcp.to_string());

    // --------- part-1 ----------

    // Use a `match` block to return the
    // file `contents` as a `Data struct: Ok(d)`
    // or handle any `errors: Err(_)`.

    let data: Data = match toml::from_str(&contents_toml_config_gcp) {
        // If successful, return data as `Data` struct.
        // `d` is a local variable.
        Ok(d) => d,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Unable to load data from `{}`", file_name_toml_gcp);
            // Exit the program with exit code `1`.
            exit(1);
        }
    };

    println!("---[part-1]---\n");

    println!("---[gcp]---");

    println!("{}", data.gcpbqmetadata.gcp_project_name);
    println!("{}", data.gcpbqmetadata.gcp_bq_dataset);

    println!("---[azure]---");

    println!("{}", data.microsoftcomputevirtualmachines.azure_client_id);
    println!("{}", data.microsoftcomputevirtualmachines.azure_tenant_id);
    println!("{}", data.microsoftcomputevirtualmachines.azure_region_name);

    // --------- part-2 ----------

    println!("\n---[part-2]---\n");

    let file_name_toml_azure = "app_config_azure.toml";

    let contents_toml_config_azure = get_toml_file_contents(file_name_toml_azure.to_string());

    let items_table: HashMap<String, Vec<AzureMetadataItem>> = from_str(contents_toml_config_azure.as_str()).unwrap();
    let items: &[AzureMetadataItem] = &items_table["azure_entity_spn"];

    println!("---[items_table]---\n");
    println!("{:#?}", items_table);

    println!("---[items]---\n");
    println!("{:#?}", items);

}
```

Output Of : `cargo run`

```bash
---[part-1]---

---[gcp]---
my-test-gcp-project
my-dataset
---[azure]---
00000000-0000-0000-0000-000000000111
00000000-0000-0000-0000-000000000999
america

---[part-2]---

---[items_table]---

{
    "azure_entity_spn": [
        AzureMetadataItem {
            azure_client_id: "00000000-0000-0000-0000-000000000111",
            azure_client_secret: "<RANDOM_STR_001>",
            azure_tenant_id: "00000000-0000-0000-0000-000000000999",
            azure_region_name: "america",
            entity_type: "microsoft.compute/virtualmachines",
        },
        AzureMetadataItem {
            azure_client_id: "00000000-0000-0000-0000-000000000222",
            azure_client_secret: "<RANDOM_STR_002>",
            azure_tenant_id: "00000000-0000-0000-0000-000000000999",
            azure_region_name: "america",
            entity_type: "microsoft.compute/virtualmachinescalesets/virtualmachines",
        },
    ],
}
---[items]---

[
    AzureMetadataItem {
        azure_client_id: "00000000-0000-0000-0000-000000000111",
        azure_client_secret: "<RANDOM_STR_001>",
        azure_tenant_id: "00000000-0000-0000-0000-000000000999",
        azure_region_name: "america",
        entity_type: "microsoft.compute/virtualmachines",
    },
    AzureMetadataItem {
        azure_client_id: "00000000-0000-0000-0000-000000000222",
        azure_client_secret: "<RANDOM_STR_002>",
        azure_tenant_id: "00000000-0000-0000-0000-000000000999",
        azure_region_name: "america",
        entity_type: "microsoft.compute/virtualmachinescalesets/virtualmachines",
    },
]
```

#### Convert String To An Enum

```rust
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Foo {
    Bar,
    Baz,
    Bat,
    Quux,
}

impl FromStr for Foo {

    type Err = ();

    fn from_str(input: &str) -> Result<Foo, Self::Err> {
        match input {
            "Bar"  => Ok(Foo::Bar),
            "Baz"  => Ok(Foo::Baz),
            "Bat"  => Ok(Foo::Bat),
            "Quux" => Ok(Foo::Quux),
            _      => Err(()),
        }
    }
}


fn main() {
    // Use it like this
    let f = Foo::from_str("Baz").unwrap();
    assert_eq!(f, Foo::Baz);
}
```

#### Convert String To An Enum : With Custom Error Type

```rust
use std::error::Error;
use std::str::FromStr;
use std::fmt;
use std::fmt::{Display, Formatter};


#[derive(Debug)]
pub enum MyCustomError {
    MissingRecord,
    NotImplemented,
}

impl Error for MyCustomError {}

impl Display for MyCustomError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            MyCustomError::MissingRecord => write!(f, "record_missing"),
            MyCustomError::NotImplemented => write!(f, "record_not_implemented"),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Foo {
    Bar,
    Baz,
    Bat,
    Quux,
}

fn get_record(input: &str) -> Result<Foo,Box<dyn Error>> {
    match input {
        "Bar"  => Ok(Foo::Bar),
        "Baz"  => Ok(Foo::Baz),
        "Bat"  => Ok(Foo::Bat),
        "Quux" => Ok(Foo::Quux),
        "xyz"  => Err(Box::try_from(MyCustomError::MissingRecord).unwrap()),
        _      => Err(Box::try_from(MyCustomError::NotImplemented).unwrap()),
    }
}

fn main() {
    // case-1
    let f = get_record("Baz");
    match f {
        Ok(d) => {
            println!("{:#?}", d);
        },
        Err(e) => {
            println!("data invalid: {:#?}", e);
        },
    }

    // case-2
    let f = get_record("Baz_");
    match f {
        Ok(d) => {
            println!("{:#?}", d);
        },
        Err(e) => {
            println!("oops : data invalid : {:#?}", e);
        },
    }

    // case-3
    let f = get_record("xyz");
    match f {
        Ok(d) => {
            println!("{:#?}", d);
        },
        Err(e) => {
            println!("oops : data invalid : {:#?}", e);
        },
    }
}
```

Output

```bash
Baz
oops : data invalid : NotImplemented
oops : data invalid : MissingRecord
```

#### Joining Futures (await)

```rust
use async_std::task;
use std::time::Duration;
use futures::{join};
use futures::future::FutureExt;
use futures::try_join;
use async_std;
use log::log;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();

    log::info!("starting main()...");

    let f_1 = do_something_1();
    let f_2 = do_something_2();

    try_join!(f_1, f_2);

    log::info!("end main() !");
}

async fn do_something_1() -> Result<(), String>{
    log::info!(">> do_something_1()");
    async_std::task::sleep(Duration::from_secs(2)).await;
    log::info!("do_something_1() done !");
    Ok(())
}

async fn do_something_2() -> Result<(), String> {
    log::info!(">> do_something_2()");
    async_std::task::sleep(Duration::from_secs(2)).await;
    log::info!("do_something_2() done !");
    Ok(())
}
```

Output

```bash
2023-05-07T18:52:29.998Z INFO  [test_app] starting main()...
2023-05-07T18:52:29.998Z INFO  [test_app] >> do_something_1()
2023-05-07T18:52:29.999Z TRACE [polling::kqueue] new: kqueue_fd=9
2023-05-07T18:52:29.999Z TRACE [polling] Poller::notify()
2023-05-07T18:52:29.999Z TRACE [polling::kqueue] notify: kqueue_fd=9
2023-05-07T18:52:29.999Z INFO  [test_app] >> do_something_2()
2023-05-07T18:52:29.999Z TRACE [polling] Poller::notify()
2023-05-07T18:52:29.999Z TRACE [async_io::driver] main_loop: waiting on I/O
2023-05-07T18:52:29.999Z TRACE [async_io::reactor] process_timers: 0 ready wakers
2023-05-07T18:52:29.999Z TRACE [polling] Poller::wait(_, Some(1.999334042s))
2023-05-07T18:52:29.999Z TRACE [polling::kqueue] wait: kqueue_fd=9, timeout=Some(1.999334042s)
2023-05-07T18:52:29.999Z TRACE [polling::kqueue] new events: kqueue_fd=9, res=1
2023-05-07T18:52:29.999Z TRACE [async_io::reactor] process_timers: 0 ready wakers
2023-05-07T18:52:29.999Z TRACE [async_io::reactor] react: 0 ready wakers
2023-05-07T18:52:29.999Z TRACE [async_io::driver] main_loop: waiting on I/O
2023-05-07T18:52:29.999Z TRACE [async_io::reactor] process_timers: 0 ready wakers
2023-05-07T18:52:29.999Z TRACE [polling] Poller::wait(_, Some(1.999234083s))
2023-05-07T18:52:29.999Z TRACE [polling::kqueue] wait: kqueue_fd=9, timeout=Some(1.999234083s)
2023-05-07T18:52:31.999Z TRACE [polling::kqueue] new events: kqueue_fd=9, res=0
2023-05-07T18:52:32.000Z TRACE [async_io::reactor] process_timers: 2 ready wakers
2023-05-07T18:52:32.000Z TRACE [async_io::reactor] react: 2 ready wakers
2023-05-07T18:52:32.000Z TRACE [async_io::driver] main_loop: waiting on I/O
2023-05-07T18:52:32.000Z TRACE [async_io::reactor] process_timers: 0 ready wakers
2023-05-07T18:52:32.000Z TRACE [polling] Poller::wait(_, None)
2023-05-07T18:52:32.000Z TRACE [polling::kqueue] wait: kqueue_fd=9, timeout=None
2023-05-07T18:52:32.000Z INFO  [test_app] do_something_1() done !
2023-05-07T18:52:32.000Z INFO  [test_app] do_something_2() done !
2023-05-07T18:52:32.000Z INFO  [test_app] end main() !
```

#### Async With Concurrency

```rust
use async_std::task;
use std::time::Duration;
use futures::{join, select, StreamExt};
use futures::future::FutureExt;
use futures::stream;
use futures::pin_mut;
use futures::try_join;
use async_std;
use log::log;
use simple_logger::SimpleLogger;
use rand::{thread_rng, Rng};

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();

    log::info!("starting main()...");

    let jobs = 0..100;
    let concurrency = 5;

    _ = stream::iter(jobs).for_each_concurrent(concurrency, |job| async move {
        let result = do_something(job).await.unwrap();
        process_result(result).await;
    }).await;

    log::info!("end main() !");
}

async fn do_something(job: i64) -> Result<String, String>{
    log::info!(">> do_something()");
    let mut rng = thread_rng();
    let sleep_ms: u64 = rng.gen_range(1000..5000);
    // async_std::task::sleep(Duration::from_millis(sleep_ms)).await;
    tokio::time::sleep(Duration::from_millis(sleep_ms)).await;
    log::info!("<< do_something_1() done !");
    let my_data = format!("do_something:done:{}", sleep_ms);
    Ok(my_data)
}

async fn process_result(data: String) {
    log::info!("process_result -> {}", data);
}
```

#### Fixed Sleep For Each ASYNC Function

```rust
use async_std::task;
use std::time::Duration;
use futures::{join, select, StreamExt};
use futures::future::FutureExt;
use futures::stream;
use futures::pin_mut;
use futures::try_join;
use async_std;
use log::log;
use simple_logger::SimpleLogger;
use rand::{thread_rng, Rng};

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();

    log::info!("starting main()...");

    let jobs = 0..100;
    let concurrency = 7;

    _ = stream::iter(jobs).for_each_concurrent(concurrency, |job| async move {
        let result = do_something(job).await.unwrap();
        process_result(result).await;
    }).await;

    log::info!("end main() !");
}

async fn do_something(job: i64) -> Result<String, String>{
    log::info!(">> do_something()");
    let mut rng = thread_rng();
    let sleep_ms: u64 = 3000;
    // let sleep_ms: u64 = rng.gen_range(1000..5000);
    // async_std::task::sleep(Duration::from_millis(sleep_ms)).await;
    tokio::time::sleep(Duration::from_millis(sleep_ms)).await;
    log::info!("<< do_something_1() done !");
    let my_data = format!("do_something:done:{}", sleep_ms);
    Ok(my_data)
}

async fn process_result(data: String) {
    log::info!("process_result -> {}", data);
}
```

#### Async With Concurrency & Collecting Results

```rust
use async_std::task;
use std::time::Duration;

use futures::{join, select, StreamExt};
use futures::future::FutureExt;
use futures::stream;
use futures::pin_mut;
use futures::try_join;
use async_std;
use log::log;
use simple_logger::SimpleLogger;
use rand::{thread_rng, Rng};

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();

    log::info!("starting main()...");

    let jobs = 0..33;
    let concurrency = 4;

    let results: Vec<String> = stream::iter(jobs).map(do_something).buffer_unordered(concurrency).collect().await;

    log::info!("{:#?}", results);

    log::info!("end main() !");
}

async fn do_something(job: i64) -> String {
    log::info!(">> do_something()");
    let mut rng = thread_rng();
    let sleep_ms: u64 = rng.gen_range(1000..3000);
    tokio::time::sleep(Duration::from_millis(sleep_ms)).await;
    log::info!("<< do_something() done !");
    let my_data = format!("do_something:done:{}", sleep_ms);
    log::info!("{}", &my_data);
    my_data
}
```

Output

```bash
2023-05-09T17:49:09.923Z INFO  [test_app] starting main()...
2023-05-09T17:49:09.923Z INFO  [test_app] >> do_something()
2023-05-09T17:49:09.924Z INFO  [test_app] >> do_something()
2023-05-09T17:49:09.924Z INFO  [test_app] >> do_something()
2023-05-09T17:49:09.924Z INFO  [test_app] >> do_something()
2023-05-09T17:49:11.058Z INFO  [test_app] << do_something() done !
2023-05-09T17:49:11.058Z INFO  [test_app] do_something:done:1133
2023-05-09T17:49:11.058Z INFO  [test_app] >> do_something()
2023-05-09T17:49:12.063Z INFO  [test_app] << do_something() done !
2023-05-09T17:49:12.063Z INFO  [test_app] do_something:done:1004
2023-05-09T17:49:12.063Z INFO  [test_app] >> do_something()
2023-05-09T17:49:12.147Z INFO  [test_app] << do_something() done !
2023-05-09T17:49:12.147Z INFO  [test_app] do_something:done:2220
2023-05-09T17:49:12.147Z INFO  [test_app] >> do_something()
2023-05-09T17:49:12.152Z INFO  [test_app] << do_something() done !
2023-05-09T17:49:12.152Z INFO  [test_app] do_something:done:2225
2023-05-09T17:49:12.152Z INFO  [test_app] >> do_something()
2023-05-09T17:49:12.620Z INFO  [test_app] << do_something() done !
2023-05-09T17:49:12.620Z INFO  [test_app] do_something:done:2695
2023-05-09T17:49:12.621Z INFO  [test_app] >> do_something()
2023-05-09T17:49:13.357Z INFO  [test_app] << do_something() done !
2023-05-09T17:49:13.357Z INFO  [test_app] do_something:done:1207
2023-05-09T17:49:13.357Z INFO  [test_app] >> do_something()
2023-05-09T17:49:13.402Z INFO  [test_app] << do_something() done !
2023-05-09T17:49:13.402Z INFO  [test_app] do_something:done:1248
2023-05-09T17:49:13.402Z INFO  [test_app] >> do_something()
2023-05-09T17:49:14.344Z INFO  [test_app] << do_something() done !
2023-05-09T17:49:14.344Z INFO  [test_app] do_something:done:2278
2023-05-09T17:49:14.344Z INFO  [test_app] >> do_something()
2023-05-09T17:49:14.714Z INFO  [test_app] << do_something() done !
2023-05-09T17:49:14.714Z INFO  [test_app] do_something:done:2088
2023-05-09T17:49:14.714Z INFO  [test_app] >> do_something()
2023-05-09T17:49:14.784Z INFO  [test_app] << do_something() done !
2023-05-09T17:49:14.784Z INFO  [test_app] do_something:done:1379
2023-05-09T17:49:14.784Z INFO  [test_app] >> do_something()
2023-05-09T17:49:14.933Z INFO  [test_app] << do_something() done !
2023-05-09T17:49:14.933Z INFO  [test_app] do_something:done:1572
2023-05-09T17:49:14.933Z INFO  [test_app] >> do_something()
2023-05-09T17:49:15.382Z INFO  [test_app] << do_something() done !
2023-05-09T17:49:15.382Z INFO  [test_app] do_something:done:1036
2023-05-09T17:49:15.382Z INFO  [test_app] >> do_something()
2023-05-09T17:49:16.260Z INFO  [test_app] << do_something() done !
2023-05-09T17:49:16.260Z INFO  [test_app] do_something:done:1474
2023-05-09T17:49:16.260Z INFO  [test_app] >> do_something()
2023-05-09T17:49:16.330Z INFO  [test_app] << do_something() done !
2023-05-09T17:49:16.330Z INFO  [test_app] do_something:done:1396
2023-05-09T17:49:16.330Z INFO  [test_app] >> do_something()
2023-05-09T17:49:16.939Z INFO  [test_app] << do_something() done !
2023-05-09T17:49:16.943Z INFO  [test_app] do_something:done:2224
2023-05-09T17:49:16.943Z INFO  [test_app] >> do_something()
2023-05-09T17:49:17.325Z INFO  [test_app] << do_something() done !
2023-05-09T17:49:17.325Z INFO  [test_app] do_something:done:1941
2023-05-09T17:49:17.325Z INFO  [test_app] >> do_something()
2023-05-09T17:49:18.893Z INFO  [test_app] << do_something() done !
2023-05-09T17:49:18.893Z INFO  [test_app] do_something:done:1567
2023-05-09T17:49:18.894Z INFO  [test_app] >> do_something()
2023-05-09T17:49:18.972Z INFO  [test_app] << do_something() done !
2023-05-09T17:49:18.972Z INFO  [test_app] do_something:done:2638
2023-05-09T17:49:18.973Z INFO  [test_app] >> do_something()
2023-05-09T17:49:19.197Z INFO  [test_app] << do_something() done !
2023-05-09T17:49:19.197Z INFO  [test_app] do_something:done:2935
2023-05-09T17:49:19.197Z INFO  [test_app] >> do_something()
2023-05-09T17:49:19.833Z INFO  [test_app] << do_something() done !
2023-05-09T17:49:19.834Z INFO  [test_app] do_something:done:2888
2023-05-09T17:49:19.834Z INFO  [test_app] >> do_something()
2023-05-09T17:49:20.153Z INFO  [test_app] << do_something() done !
2023-05-09T17:49:20.153Z INFO  [test_app] do_something:done:1255
2023-05-09T17:49:20.153Z INFO  [test_app] >> do_something()
2023-05-09T17:49:20.794Z INFO  [test_app] << do_something() done !
2023-05-09T17:49:20.794Z INFO  [test_app] do_something:done:1819
2023-05-09T17:49:20.794Z INFO  [test_app] >> do_something()
2023-05-09T17:49:21.224Z INFO  [test_app] << do_something() done !
2023-05-09T17:49:21.224Z INFO  [test_app] do_something:done:2024
2023-05-09T17:49:21.224Z INFO  [test_app] >> do_something()
2023-05-09T17:49:21.306Z INFO  [test_app] << do_something() done !
2023-05-09T17:49:21.307Z INFO  [test_app] do_something:done:1151
2023-05-09T17:49:21.308Z INFO  [test_app] >> do_something()
2023-05-09T17:49:21.326Z INFO  [test_app] << do_something() done !
2023-05-09T17:49:21.326Z INFO  [test_app] do_something:done:1488
2023-05-09T17:49:21.326Z INFO  [test_app] >> do_something()
2023-05-09T17:49:22.296Z INFO  [test_app] << do_something() done !
2023-05-09T17:49:22.296Z INFO  [test_app] do_something:done:1497
2023-05-09T17:49:22.296Z INFO  [test_app] >> do_something()
2023-05-09T17:49:22.704Z INFO  [test_app] << do_something() done !
2023-05-09T17:49:22.704Z INFO  [test_app] do_something:done:1479
2023-05-09T17:49:22.704Z INFO  [test_app] >> do_something()
2023-05-09T17:49:23.132Z INFO  [test_app] << do_something() done !
2023-05-09T17:49:23.132Z INFO  [test_app] do_something:done:1823
2023-05-09T17:49:23.132Z INFO  [test_app] >> do_something()
2023-05-09T17:49:23.792Z INFO  [test_app] << do_something() done !
2023-05-09T17:49:23.792Z INFO  [test_app] do_something:done:2464
2023-05-09T17:49:23.792Z INFO  [test_app] >> do_something()
2023-05-09T17:49:23.939Z INFO  [test_app] << do_something() done !
2023-05-09T17:49:23.940Z INFO  [test_app] do_something:done:1233
2023-05-09T17:49:23.999Z INFO  [test_app] << do_something() done !
2023-05-09T17:49:23.999Z INFO  [test_app] do_something:done:1701
2023-05-09T17:49:25.489Z INFO  [test_app] << do_something() done !
2023-05-09T17:49:25.489Z INFO  [test_app] do_something:done:2355
2023-05-09T17:49:26.032Z INFO  [test_app] << do_something() done !
2023-05-09T17:49:26.032Z INFO  [test_app] do_something:done:2238
2023-05-09T17:49:26.032Z INFO  [test_app] [
    "do_something:done:1133",
    "do_something:done:1004",
    "do_something:done:2220",
    "do_something:done:2225",
    "do_something:done:2695",
    "do_something:done:1207",
    "do_something:done:1248",
    "do_something:done:2278",
    "do_something:done:2088",
    "do_something:done:1379",
    "do_something:done:1572",
    "do_something:done:1036",
    "do_something:done:1474",
    "do_something:done:1396",
    "do_something:done:2224",
    "do_something:done:1941",
    "do_something:done:1567",
    "do_something:done:2638",
    "do_something:done:2935",
    "do_something:done:2888",
    "do_something:done:1255",
    "do_something:done:1819",
    "do_something:done:2024",
    "do_something:done:1151",
    "do_something:done:1488",
    "do_something:done:1497",
    "do_something:done:1479",
    "do_something:done:1823",
    "do_something:done:2464",
    "do_something:done:1233",
    "do_something:done:1701",
    "do_something:done:2355",
    "do_something:done:2238",
]
2023-05-09T17:49:26.034Z INFO  [test_app] end main() !
```

#### Stackdriver Logging

`Cargo.toml`

```toml
stackdriver_logger = "0.8.2"
log = "0.4.17"
```

```rust
use async_std::task;
use std::time::Duration;

use futures::{join, select, StreamExt};
use futures::future::FutureExt;
use futures::stream;
use futures::pin_mut;
use futures::try_join;
use async_std;
use rand::{thread_rng, Rng};
use log::{error, info, trace, debug, warn};

#[tokio::main]
async fn main() {
    stackdriver_logger::init_with_cargo!();
    info!("starting main()...");
    let jobs = 0..33;
    let concurrency = 4;
    let results: Vec<String> = stream::iter(jobs).map(do_something).buffer_unordered(concurrency).collect().await;
    info!("{:#?}", results);
    error!("this is a test error message");
    info!("end main() !");
}

async fn do_something(job: i64) -> String {
    info!(">> do_something()");
    let mut rng = thread_rng();
    let sleep_ms: u64 = rng.gen_range(1000..3000);
    tokio::time::sleep(Duration::from_millis(sleep_ms)).await;
    info!("<< do_something() done !");
    let my_data = format!("do_something:done:{}", sleep_ms);
    info!("{}", &my_data);
    my_data
}
```

#### `?` Operator

As you may have noticed, Rust does not have exceptions. It has panics, but their use for error-handling is discouraged (they are meant for unrecoverable errors).

In Rust, error handling uses Result. A typical example would be:

```rust
fn halves_if_even(i: i32) -> Result<i32, Error> {
    if i % 2 == 0 {
        Ok(i / 2)
    } else {
        Err(/* something */)
    }
}

fn do_the_thing(i: i32) -> Result<i32, Error> {
    let i = match halves_if_even(i) {
        Ok(i) => i,
        Err(e) => return Err(e),
    };

    // use `i`
}
```

This is great because:

 - when writing the code you cannot accidentally forget to deal with the error,
 - when reading the code you can immediately see that there is a potential for error right here.

 It's less than ideal, however, in that it is very verbose. This is where the question mark operator `?` comes in.

 The above can be rewritten as:

```rust
fn do_the_thing(i: i32) -> Result<i32, Error> {
    let i = halves_if_even(i)?;

    // use `i`
}
```

which is much more concise.

What `?` does here is equivalent to the match statement above with an addition. In short:

1. It unpacks the Result if OK

2. It returns the error if not, calling From::from on the error value to potentially convert it to another type.

3. It's a bit magic, but error handling needs some magic to cut down the boilerplate, and unlike exceptions it is immediately visible which function calls may or may not error out: those that are adorned with `?`

One example of the magic is that this also works for `Option`:

```rust
// Assume
// fn halves_if_even(i: i32) -> Option<i32>

fn do_the_thing(i: i32) -> Option<i32> {
    let i = halves_if_even(i)?;

    // use `i`
}
```

#### `?` Operator And Cascading Errors : Custom Error Type

```rust
#[derive(Debug)]
enum MathError {
    DivisionByZero,
}

#[derive(Debug)]
struct CustomError {
    err_type: MathError,
    err_message: String,
}


fn div(x: f64, y: f64) -> Result<f64, CustomError> {
    if y == 0.0 {
        let custom_error = CustomError {
            err_message: "cannot divide by zero".to_string(),
            err_type: MathError::DivisionByZero,
        };
        return Err(custom_error);
    } else {
        Ok(x / y)
    }
}

fn main() -> Result<(), CustomError> {
    let result_1 = div(5.34 , 2.5336);
    println!("result_1 : {:#?}", result_1);

    let result_2 = div(5.34 , 2.5336)?;
    println!("result_2 : {}", result_2);

    //-----------------------------------------------------------

    let result_3 = div(5.34, 0 as f64);
    println!("result_3 : {:#?}", result_3);

    let result_4 = div(5.34, 0 as f64)?;
    println!("result_4 : {}", result_4);

    //-----------------------------------------------------------

    Ok(())
}
```

Output

```bash
result_1 : Ok(
    2.107672876539312,
)
result_2 : 2.107672876539312
result_3 : Err(
    CustomError {
        err_type: DivisionByZero,
        err_message: "cannot divide by zero",
    },
)
Error: CustomError { err_type: DivisionByZero, err_message: "cannot divide by zero" }
```

#### Run Scheduled Jobs / Cron Job

`Cargo.toml`

```toml
job_scheduler = "*"
```

`main.rs`

```rust
extern crate job_scheduler;
use job_scheduler::{JobScheduler, Job};
use std::time::Duration;

fn main() {
    let mut sched = JobScheduler::new();

    sched.add(Job::new("1/5 * * * * *".parse().unwrap(), || {
        println!("A: I get executed every 5 seconds!");
    }));

    sched.add(Job::new("1/10 * * * * *".parse().unwrap(), || {
        println!("B: I get executed every 10 seconds!");
    }));

    sched.add(Job::new("1/15 * * * * *".parse().unwrap(), || {
        println!("C: I get executed every 15 seconds!");
    }));

    loop {
        sched.tick();

        std::thread::sleep(Duration::from_millis(500));
    }
}
```

Output

```bash
A: I get executed every 5 seconds!
A: I get executed every 5 seconds!
B: I get executed every 10 seconds!
C: I get executed every 15 seconds!
A: I get executed every 5 seconds!
A: I get executed every 5 seconds!
B: I get executed every 10 seconds!
A: I get executed every 5 seconds!
C: I get executed every 15 seconds!
A: I get executed every 5 seconds!
B: I get executed every 10 seconds!
A: I get executed every 5 seconds!
A: I get executed every 5 seconds!
B: I get executed every 10 seconds!
C: I get executed every 15 seconds!
...
...
...
```

#### Running Scheduled Cron Jobs Using Tokio Runtime

```rust
use tokio::time::interval;
use log;

async fn do_something_1(my_str: String) -> Result<(), String>{
    println!(">> do_something_1() : {}",my_str);
    // async_std::task::sleep(Duration::from_secs(3)).await;
    println!("<< do_something_1() done !");
    Ok(())
}

async fn do_something_2(my_str: String) -> Result<(), String>{
    println!(">> do_something_2() : {}",my_str);
    // async_std::task::sleep(Duration::from_secs(3)).await;
    println!("<< do_something_2() done !");
    Ok(())
}

#[tokio::main]
async fn main()  {
    // let mut interval = std::time::interval(std::time::Duration::from_secs(3));
    let mut interval = interval(std::time::Duration::from_secs(3));

    let mut counter = 1;

    loop {
        interval.tick().await;

        tokio::spawn(async move {
            let _data1 = do_something_1(counter.to_string()).await.unwrap();
        });

        tokio::spawn(async move {
            let _data1 = do_something_2(counter.to_string()).await.unwrap();
        });
        counter = counter + 1;
    }
}
```

Output

```bash
>> do_something_1() : 1
<< do_something_1() done !
>> do_something_2() : 1
<< do_something_2() done !
>> do_something_1() : 2
<< do_something_1() done !
>> do_something_2() : 2
<< do_something_2() done !
>> do_something_1() : 3
<< do_something_1() done !
>> do_something_2() : 3
<< do_something_2() done !
>> do_something_1() : 4
<< do_something_1() done !
>> do_something_2() : 4
<< do_something_2() done !
...
...
```

#### Using Trait Objects in Rust & Dynamic Dispatch

`lib.rs`

```rust
// ------ Using Trait Objects in Rust -------

pub trait Draw {
    fn draw(&self);
}

// list of draw'able components
pub struct Screen {
    // uses Box smart pointer
    // A pointer type that uniquely owns a heap allocation of type T.
    // contains any type , that implements the Draw trait
    // components: it contains a Vector of Trait Objects
    // dyn -> is Dynamic Dispatch
    pub components: Vec< Box<dyn Draw> >,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// why not use Generics, instead of trait objects ?
// when using generics, you are limited to one type
// that is, our Screen component can store buttons, sliders, text input fields etc.
// But, it cannot store a mixture of the three different types.
// that is, the list has to be homogenous

// But, if you are only storing buttons or only text input fields,
// then advantage with Generics (with Trait bounds) is that : there is no run time cost
// However, if you want the flexibility to store any Type that implements a certain Trait,
// then using Trait objects makes more sense


// (static dispatch) : when the compiler knows the concrete functions that you are calling
// at compile time.
// the opposite is (dynamic dispatch)
// dynamic dispatch happens when the compiler does not know the concrete methods you are calling
// at compile time, it figures that out at run time

// when using trait objects, the rust compiler must use dynamic dispatch
// which means there is a run time performance cost
// however we get to write flexible code, which can accept any object,
// which implements a certain trait

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("this is the draw for Button...");
    }
}
```

`main.rs`

```rust
use test_app::{Draw, Screen, Button};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("this is the draw for SelectBox...");
    }
}

fn main() {

    let my_select_box = Box::new(
        SelectBox {
            width: 100,
            height: 100,
            options: vec![
                String::from("yes"),
                String::from("no"),
                String::from("may_be"),
            ],
        }
    );

    let my_button = Box::new(
        Button {
            width: 10,
            height: 15,
            label: "execute".to_string(),
        }
    );

    let screen = Screen {
        components: vec![
            my_select_box,
            my_button
        ],
    };

    screen.run();
}
```

#### Date & Time

```rust
use chrono;


fn main() {
    // returns DateTime<Local>
    println!("{:?}", chrono::offset::Local::now());

    // returns DateTime<Utc>
    // NOTE: Available on crate feature *clock* only.
    println!("{:?}", chrono::offset::Utc::now());
    
    let dt = chrono::Utc::now();
    let timestamp: i64 = dt.timestamp();

    println!("Current timestamp is {}", timestamp);
}
```

Output

```bash
2023-05-25T19:52:48.729841750+00:00
2023-05-25T19:52:48.729991974Z
Current timestamp is 1685044368
```

#### Convert a local time to another timezone

```rust
use chrono::{DateTime, FixedOffset, Local, Utc};

fn main() {
    let local_time = Local::now();
    
    let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
    
    let china_timezone = FixedOffset::east(8 * 3600);
    let rio_timezone = FixedOffset::west(2 * 3600);
    
    println!("Local time now is {}", local_time);
    println!("UTC time now is {}", utc_time);
    
    println!("Time in Hong Kong now is {}", utc_time.with_timezone(&china_timezone));
    println!("Time in Rio de Janeiro now is {}", utc_time.with_timezone(&rio_timezone));
}
```

Output

```bash
Local time now is 2023-05-25 19:58:28.898108863 +00:00
UTC time now is 2023-05-25 19:58:28.898108863 UTC
Time in Hong Kong now is 2023-05-26 03:58:28.898108863 +08:00
Time in Rio de Janeiro now is 2023-05-25 17:58:28.898108863 -02:00
```

#### Display formatted date and time

```rust
use chrono::{DateTime, Utc};

fn main() {
    let now: DateTime<Utc> = Utc::now();

    println!("UTC now is: {}", now);
    println!("UTC now in RFC 2822 is: {}", now.to_rfc2822());
    println!("UTC now in RFC 3339 is: {}", now.to_rfc3339());
    println!("UTC now in a custom format is: {}", now.format("%a %b %e %T %Y"));
}
```

Output

```bash
UTC now is: 2023-05-25 20:03:11.742083840 UTC
UTC now in RFC 2822 is: Thu, 25 May 2023 20:03:11 +0000
UTC now in RFC 3339 is: 2023-05-25T20:03:11.742083840+00:00
UTC now in a custom format is: Thu May 25 20:03:11 2023
```

#### Custom Logging

```rust
use log::{Record, Level, Metadata, LevelFilter, SetLoggerError};

static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("Rust App : {} : {}", record.level(), record.args());
        }
    }
    fn flush(&self) {}
}

fn main() -> Result<(), SetLoggerError> {
    log::set_logger(&CONSOLE_LOGGER)?;
    log::set_max_level(LevelFilter::Info);

    log::info!("hello log");
    log::warn!("warning");
    log::error!("oops");
    Ok(())
}
```

Output

```bash
Rust App : INFO : hello log
Rust App : WARN : warning
Rust App : ERROR : oops
```

#### ASYNC CONCURRENCY : Call async Functions In Parallel

`Cargo.toml`

```toml
[package]
name = "async-concurrency"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
tokio = { version = "1.23.0", features = ["full"] }
rand = "0.8.5"
md5 = "0.7.0"
futures = "0.3.4" # for our async / await blocks
uuid = { version = "1.3.0", features = ["v4", "fast-rng", "macro-diagnostics"] }
```

`src/main.rs`

```rust
use std::fmt::{format, Formatter};
use tokio::time::sleep as tokio_sleep;
use tokio::time::Duration as tokio_duration;
use rand::{Rng, thread_rng};
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};
use md5;
use tokio;
use std::{time};
use futures::{join, select, StreamExt};
use futures::future::FutureExt;
use futures::stream;
use futures::pin_mut;
use futures::try_join;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct InputData {
    pub epoch_time: u64,
    pub uuid_data: String,
    pub id: i32,
}

#[tokio::main]
async fn main() {
    let results = perform_all_calculations().await;
    println!("\nresults : \n\n{:#?}\n", results);
}

async fn perform_all_calculations() -> Vec<String> {
    let now = time::Instant::now();
    let input_vec = generate_input_data();
    let input_vec_length = input_vec.len();
    // at any given time, run these many async functions
    let concurrency = 4;
    let results: Vec<_> = stream::iter(input_vec).map(perform_calculation_for_input).buffer_unordered(concurrency).collect().await;
    let elapsed = now.elapsed().as_secs_f64();
    println!("perform_all_calculations() : total time taken : {}", elapsed);
    results
}

// generate a vector of input data for computation
fn generate_input_data() -> Vec<InputData> {
    let mut input_data = vec![];
    for i in 1..=16 {
        let current_epoch = get_epoch_time();
        let random_uuid = Uuid::new_v4();
        let input = InputData {
            epoch_time: current_epoch,
            uuid_data: random_uuid.to_string(),
            id: i,
        };
        input_data.push(input);
    }
    println!("generate_input_data() : input_data : \n{:#?}\n", input_data);
    input_data
}

// for a given input of type (InputData), compute md5 hash
async fn perform_calculation_for_input(input: InputData) -> String {

    // first sleep for some random seconds (to simulate doing some job)
    let random_number = get_random_number();

    tokio_sleep(tokio_duration::from_secs_f64(random_number)).await;

    // if epoch is 1234 and uuid is a0b1c2d3 , then input_str will be 1234_a0b1c2d3
    let input_str = input.to_string();
    // let input_str = get_input_data_as_string(input.clone());

    // this will compute md5 of 1234_a0b1c2d3
    let computed_md5 = get_md5(input_str);

    // print the output
    println!("perform_calculation_for_input() : input : {:#?} , \ncomputed_md5 : {}\n\n", input, computed_md5);

    // return the computed md5 hash as a string
    computed_md5
}

// helper function : get current epoch time
fn get_epoch_time() -> u64 {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).unwrap();
    since_the_epoch.as_secs()
}

// helper function : get md5 hash of a given input string
fn get_md5(my_str: String) -> String {
    let digest = md5::compute(my_str);
    let computed_hash = format!("{:x}", digest);
    computed_hash
}

// helper function : get InputData as a string
impl std::fmt::Display for InputData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_{}", self.epoch_time, self.uuid_data)
    }
}

// helper function : get random number
fn get_random_number() -> f64 {
    let mut rng = thread_rng();
    let random_number = rng.gen_range(2..5);
    random_number as f64
}
```

Output

```bash
generate_input_data() : input_data :
[
    InputData {
        epoch_time: 1687793303,
        uuid_data: "ea494cd5-4ba3-45e6-aaac-dd5d6471c654",
        id: 1,
    },
    InputData {
        epoch_time: 1687793303,
        uuid_data: "2947b63d-5660-483c-859e-bb7084f3f561",
        id: 2,
    },
    InputData {
        epoch_time: 1687793303,
        uuid_data: "fec80b3d-1310-43e0-9322-c898ebc49211",
        id: 3,
    },
    InputData {
        epoch_time: 1687793303,
        uuid_data: "34905092-0f75-479d-bbef-4d9c381d7ae4",
        id: 4,
    },
    InputData {
        epoch_time: 1687793303,
        uuid_data: "29528d21-44ca-48fa-a4ae-a1ad281a11f8",
        id: 5,
    },
    InputData {
        epoch_time: 1687793303,
        uuid_data: "3c356d6e-a5bd-475d-abb0-8c64fa55c7e9",
        id: 6,
    },
    InputData {
        epoch_time: 1687793303,
        uuid_data: "6054c1a6-84e1-4143-a210-d128fab80fc6",
        id: 7,
    },
    InputData {
        epoch_time: 1687793303,
        uuid_data: "cda88a1b-1954-417e-89c8-6406651ead00",
        id: 8,
    },
    InputData {
        epoch_time: 1687793303,
        uuid_data: "bf13270d-ac9a-4611-902d-9f7cec4783d9",
        id: 9,
    },
    InputData {
        epoch_time: 1687793303,
        uuid_data: "84c4cd51-756d-4442-948b-367be372692d",
        id: 10,
    },
    InputData {
        epoch_time: 1687793303,
        uuid_data: "bd00d879-a1a6-4780-9121-04a33078eb63",
        id: 11,
    },
    InputData {
        epoch_time: 1687793303,
        uuid_data: "95de9f48-3432-4959-bcf0-cb8c7852b512",
        id: 12,
    },
    InputData {
        epoch_time: 1687793303,
        uuid_data: "c9051b6d-f391-4bf2-b5fa-fe52d7a5899e",
        id: 13,
    },
    InputData {
        epoch_time: 1687793303,
        uuid_data: "d19db842-f5e8-46fa-8cd7-b860141609a6",
        id: 14,
    },
    InputData {
        epoch_time: 1687793303,
        uuid_data: "75e35aba-0859-43fd-8584-60b672123a40",
        id: 15,
    },
    InputData {
        epoch_time: 1687793303,
        uuid_data: "64e6a01e-3081-4103-9546-a1a7aacbc4a4",
        id: 16,
    },
]

perform_calculation_for_input() : input : InputData {
    epoch_time: 1687793303,
    uuid_data: "2947b63d-5660-483c-859e-bb7084f3f561",
    id: 2,
} ,
computed_md5 : a25bd0a4e3556a10643ec7e539a50d8e


perform_calculation_for_input() : input : InputData {
    epoch_time: 1687793303,
    uuid_data: "fec80b3d-1310-43e0-9322-c898ebc49211",
    id: 3,
} ,
computed_md5 : 1691c160f304bd8f577d7913e149b503


perform_calculation_for_input() : input : InputData {
    epoch_time: 1687793303,
    uuid_data: "ea494cd5-4ba3-45e6-aaac-dd5d6471c654",
    id: 1,
} ,
computed_md5 : d32f86c1ba9f96ee1922cb454a820096


perform_calculation_for_input() : input : InputData {
    epoch_time: 1687793303,
    uuid_data: "34905092-0f75-479d-bbef-4d9c381d7ae4",
    id: 4,
} ,
computed_md5 : 8c98192db810ac81265e9d8f048d8196


perform_calculation_for_input() : input : InputData {
    epoch_time: 1687793303,
    uuid_data: "29528d21-44ca-48fa-a4ae-a1ad281a11f8",
    id: 5,
} ,
computed_md5 : e4191d9a662666d69d12a4958a291512


perform_calculation_for_input() : input : InputData {
    epoch_time: 1687793303,
    uuid_data: "6054c1a6-84e1-4143-a210-d128fab80fc6",
    id: 7,
} ,
computed_md5 : 441e0e0092dcdc9ba10c3af177568c29


perform_calculation_for_input() : input : InputData {
    epoch_time: 1687793303,
    uuid_data: "cda88a1b-1954-417e-89c8-6406651ead00",
    id: 8,
} ,
computed_md5 : a540bd2b78e7622ce7a9faef6f511150


perform_calculation_for_input() : input : InputData {
    epoch_time: 1687793303,
    uuid_data: "3c356d6e-a5bd-475d-abb0-8c64fa55c7e9",
    id: 6,
} ,
computed_md5 : 6dbadc3f67ebf8389ccc36b2240c5757


perform_calculation_for_input() : input : InputData {
    epoch_time: 1687793303,
    uuid_data: "95de9f48-3432-4959-bcf0-cb8c7852b512",
    id: 12,
} ,
computed_md5 : 5c7571c5b0697c6efbebe5c2ab53347b


perform_calculation_for_input() : input : InputData {
    epoch_time: 1687793303,
    uuid_data: "bf13270d-ac9a-4611-902d-9f7cec4783d9",
    id: 9,
} ,
computed_md5 : 5c778008f9a4c1ac75a319ead6964c5e


perform_calculation_for_input() : input : InputData {
    epoch_time: 1687793303,
    uuid_data: "84c4cd51-756d-4442-948b-367be372692d",
    id: 10,
} ,
computed_md5 : 4881a2c5f8ec5e1133a25b24f1a7b616


perform_calculation_for_input() : input : InputData {
    epoch_time: 1687793303,
    uuid_data: "bd00d879-a1a6-4780-9121-04a33078eb63",
    id: 11,
} ,
computed_md5 : 5cc736a4ffe3b667522b74fbb177bbdc


perform_calculation_for_input() : input : InputData {
    epoch_time: 1687793303,
    uuid_data: "c9051b6d-f391-4bf2-b5fa-fe52d7a5899e",
    id: 13,
} ,
computed_md5 : f83567244f8f660cdbb80d2ac5d40386


perform_calculation_for_input() : input : InputData {
    epoch_time: 1687793303,
    uuid_data: "d19db842-f5e8-46fa-8cd7-b860141609a6",
    id: 14,
} ,
computed_md5 : 96886f3ec45661de862b06f06f3dfb4e


perform_calculation_for_input() : input : InputData {
    epoch_time: 1687793303,
    uuid_data: "64e6a01e-3081-4103-9546-a1a7aacbc4a4",
    id: 16,
} ,
computed_md5 : f7e163dd471411b9790b97f0c6d3edf2


perform_calculation_for_input() : input : InputData {
    epoch_time: 1687793303,
    uuid_data: "75e35aba-0859-43fd-8584-60b672123a40",
    id: 15,
} ,
computed_md5 : ae7f2ed779b1b99fbc5cbfc9aac2b00f


perform_all_calculations() : total time taken : 14.011009708

results :

[
    "a25bd0a4e3556a10643ec7e539a50d8e",
    "1691c160f304bd8f577d7913e149b503",
    "d32f86c1ba9f96ee1922cb454a820096",
    "8c98192db810ac81265e9d8f048d8196",
    "e4191d9a662666d69d12a4958a291512",
    "441e0e0092dcdc9ba10c3af177568c29",
    "a540bd2b78e7622ce7a9faef6f511150",
    "6dbadc3f67ebf8389ccc36b2240c5757",
    "5c7571c5b0697c6efbebe5c2ab53347b",
    "5c778008f9a4c1ac75a319ead6964c5e",
    "4881a2c5f8ec5e1133a25b24f1a7b616",
    "5cc736a4ffe3b667522b74fbb177bbdc",
    "f83567244f8f660cdbb80d2ac5d40386",
    "96886f3ec45661de862b06f06f3dfb4e",
    "f7e163dd471411b9790b97f0c6d3edf2",
    "ae7f2ed779b1b99fbc5cbfc9aac2b00f",
]
```

#### Async Functions And Channels

`Cargo.toml`

```toml
[package]
name = "async-channels"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
tokio = { version = "1.29.1", features = ["full"] }
```

`src/main.rs`

```rust
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
async fn producer(tx: mpsc::Sender<i32>) {
    for i in 0..25 {
        sleep(Duration::from_millis(200)).await;
        let send_result = match tx.send(i).await {
            Ok(d) => {
                println!("tx.send successful");
            }
            Err(e) => {
                println!("tx.send failed : {:#?}", e);
            }
        };
    }
}

async fn consumer(mut rx: mpsc::Receiver<i32>) {
    while let Some(data) = rx.recv().await {
        println!("consumer : rx.recv : data : {:#}", data);
    }
}

#[tokio::main]
async fn main() {

    let (tx, rx) = mpsc::channel(1);

    let producer_handler = tokio::spawn(async move {
        producer(tx).await;
    });

    let consumer_handler = tokio::spawn(async move {
        consumer(rx).await;
    });

    let _ = tokio::try_join!(producer_handler, consumer_handler);
}
```

```bash
cargo build --release
```

Output

```bash
target/release/async-channels

tx.send successful
consumer : rx.recv : data : 0
tx.send successful
consumer : rx.recv : data : 1
tx.send successful
consumer : rx.recv : data : 2
tx.send successful
consumer : rx.recv : data : 3
tx.send successful
consumer : rx.recv : data : 4
tx.send successful
consumer : rx.recv : data : 5
tx.send successful
consumer : rx.recv : data : 6
tx.send successful
consumer : rx.recv : data : 7
tx.send successful
consumer : rx.recv : data : 8
tx.send successful
consumer : rx.recv : data : 9
tx.send successful
consumer : rx.recv : data : 10
tx.send successful
consumer : rx.recv : data : 11
tx.send successful
consumer : rx.recv : data : 12
tx.send successful
consumer : rx.recv : data : 13
tx.send successful
consumer : rx.recv : data : 14
tx.send successful
consumer : rx.recv : data : 15
tx.send successful
consumer : rx.recv : data : 16
tx.send successful
consumer : rx.recv : data : 17
tx.send successful
consumer : rx.recv : data : 18
tx.send successful
consumer : rx.recv : data : 19
tx.send successful
consumer : rx.recv : data : 20
tx.send successful
consumer : rx.recv : data : 21
tx.send successful
consumer : rx.recv : data : 22
tx.send successful
consumer : rx.recv : data : 23
tx.send successful
consumer : rx.recv : data : 24
```

#### [Openstack Hypervisor List With HTTP Retry](#openstack-hypervisor-list-with-http-retry)

Ganerally speaking, every openstack region, will have its own
- region name
- openstack credentials
- rabbitmq credentials
- http endpoint


```rust
#[derive(Debug, Deserialize)]
pub struct OpenstackRegionConfig {
    pub region_name: String,
    pub openstack_user: String,
    pub openstack_pass: String,
    pub http_endpoint: String,
    pub rabbitmq_user: String,
    pub rabbitmq_pass: String,
}
```

```rust
pub async fn get_workspace_data() -> Vec<OpenstackRegionConfig> {
    /*
        openstack_configuration.yaml:

        ---
          - region_name: "region-001"
            openstack_user: "my_user_1"
            openstack_pass: "some_thing_xyz_1"
            http_endpoint: "region-001-endpoint.prod.company.com"
            rabbitmq_user: "r_user_1"
            rabbitmq_pass: "some_thing_p_001"
          - region_name: "cdc-002"
            openstack_user: "my_user_2"
            openstack_pass: "some_thing_xyz_2"
            http_endpoint: "region-002-endpoint.prod.company.com"
            rabbitmq_user: "r_user_2"
            rabbitmq_pass: "some_thing_p_002"
    */

    // Open the YAML file
    let mut file = File::open("openstack_configuration.yaml").unwrap();
    let mut contents = String::new();

    // Read the file content
    file.read_to_string(&mut contents).unwrap();

    // Deserialize YAML into a Vec<User>
    let os_items: Vec<OpenstackRegionConfig> = serde_yaml::from_str(&contents).unwrap();
    os_items
}
```

```rust
pub async fn get_openstack_endpoints() -> Vec<String> {
    let mut endpoints:Vec<String> = vec![];

    let openstack_config = get_workspace_data().await;

    for region_config in openstack_config {
        endpoints.push(region_config.http_endpoint);
    }

    println!("openstack_http_endpoints : {:#?}", endpoints);

    endpoints
}
```

```rust
use std::time;
use reqwest;
use serde_json::{Value};
use crate::{app_error, os_keystone};
use tokio::time::Duration;
use tokio::time::{sleep};
use tokio;
use futures::stream;
use futures::{join, select, StreamExt};

pub const MAX_RETRIES: u32 = 10;

pub async fn get_os_hypervisors(http_endpoint: &str) -> Result<Vec<Value>, app_error::CustomError> {
    let client = reqwest::Client::new();

    let keystone_token = match os_keystone::get_token_with_retry(http_endpoint).await {
        None => {
            let custom_err = app_error::CustomError{
                err_msg: String::from("os_hypervisors : Could Not Make HTTP Get Keystone Token"),
                err_type: app_error::GenericError::HypervisorQueryFailed
            };
            return Err(custom_err)
        },
        Some(d) => {
            d
        },
    };

    let my_url = format!("https://{}:8774/v2.1/os-hypervisors", http_endpoint);

    let mut hypervisors = vec![];

    let response = match client.get(my_url)
        .header("X-Auth-Token", keystone_token)
        .header("accept", "application/json")
        .send()
        .await {
        Ok(d) => {
            let response: serde_json::Value = d.json().await.unwrap();
            // println!("{:#?}", response);
            if let Some(my_items) = response.get("hypervisors").and_then(Value::as_array) {
                hypervisors = my_items.to_vec();
            }
        },
        Err(e) => {
            let custom_err = app_error::CustomError{
                err_msg: String::from("os_hypervisors : Could Not Make HTTP Get Request To Fetch Hypervisor List"),
                err_type: app_error::GenericError::TenantQueryFailed
            };
            return Err(custom_err)
        },
    };
    Ok(hypervisors)
}

pub(crate) async fn get_os_hypervisors_with_retry(http_endpoint: &str) -> Vec<Value> {
    let mut backoff: u64 = 1;
    let mut retries = 20;

    let mut my_list: Vec<Value> = vec![];
    loop {
        match get_os_hypervisors(&http_endpoint).await {
            Ok(d) => {
                my_list = d;
                break;
            }
            Err(e) => {
                println!("_REQUEST_FAILED (OS_HYPERVISORS) : {:#?}", e);
                retries += 1;

                if retries > MAX_RETRIES {
                    println!("_MAX_RETRIES_REACHED (OS_HYPERVISORS) , EXITING...");
                    break;
                }

                println!("_RETRYING (OS_HYPERVISORS) IN {} SECOND(S)...", backoff);
                sleep(Duration::from_secs(backoff)).await;

                // Exponential backoff
                backoff *= 2;
            }
        }
    }
    my_list
}

pub async fn get_os_hypervisors_token_for_all_regions() -> Vec<Vec<Value>> {
    let now = time::Instant::now();
    let input_vec = os_keystone::get_openstack_endpoints().await;
    let input_data: Vec<&str> = input_vec.iter().map(|d| d.as_str()).collect();
    let input_vec_length = input_data.len();
    let concurrency = 50;
    let results: Vec<_> = stream::iter(input_data)
        .map(get_os_hypervisors_with_retry)
        .buffer_unordered(concurrency)
        .collect()
        .await;
    let elapsed = now.elapsed().as_secs_f64();
    println!(
        "get_os_hypervisors_token_for_all_regions() : total time taken : {}",
        elapsed
    );
    results
}
```

`main.rs`

```rust

use std::future::Future;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use std::{thread, time};

let all_hypervisors = os_hypervisor::get_os_hypervisors_token_for_all_regions().await;

let total_hypervisors: usize = all_hypervisors.iter().map(|inner_vec| inner_vec.len()).sum();

println!("total_hypervisors : {}", total_hypervisors);
```

#### [Actix REST API With PostgreSQL Backend](#actix-rest-api-with-postgresql-backend)

FYI : This has some imports that are not needed. Please ignore those.

`Cargo.toml`

```toml
[package]
name = "fastapi"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
actix-web = "4.3.0"
serde_json = "1.0.93"
serde = { version = "1.0.2" , features = ["derive"] }
reqwest = { version = "0.11.18" , features = ["json"] }
serde_yaml = "0.9.25"
serde_derive = "1.0.176"
tokio = { version = "1.29.1", features = ["full"] }
futures = "0.3.28"
lazy_static = "1.4.0"
sqlx = { version = "0.7.3", features = ["postgres"]}
dotenv = "0.15.0"
tokio-postgres = {version = "0.7.10", features = ["with-uuid-0_8", "with-serde_json-1"] }
database = "0.5.0"
apps = "0.2.2"
emoji-logger = "0.1.0"
deadpool-postgres = "0.11.0"
postgres-openssl = "0.5.0"
openssl = "0.10.60"
config = "0.13.4"
uuid = { version = "0.8.2" , features = ["serde"]}
derive_more = { version = "0.99.17", features = [] }
regex = "1.7.1"
dirs = "5.0.1"
async-std = { version = "1.12.0", features = [] }
```

Environment File `app.rust.env`

```ini
DATABASE_URL=postgres://<USERNAME>:<PASSWORD>@<HOSTNAME>/<DBNAME>
PG.USER=<USERNAME>
PG.PASSWORD=<PASSWORD>
PG.HOST=<HOSTNAME>
PG.PORT=5432
PG.DBNAME=<DBNAME>
PG.POOL.MAX_SIZE=16
RUST_LOG=actix_web=info
```

Please read the documentation put inside the `main.rs`, it has >
- SQL Queries
- CURL Requests (HTTP API)
- Outputs
- Verifications


`src/main.rs`

```rust
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use tokio_postgres::{Error, row};
use tokio_postgres::NoTls;
use actix_web::{get, post, web, web::Data, App, HttpResponse, HttpServer};
use actix_web::{Responder, HttpRequest, http, middleware, middleware::Logger, ResponseError};
use futures::future;
use futures::Future;
use uuid::Uuid;
use deadpool_postgres::{Config, Client, Pool};
use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::*;
use tokio::runtime::Runtime;
use ::config::ConfigError;
use dotenv::dotenv;
use tokio_postgres::tls::MakeTlsConnect;
use derive_more::{Display,From};
use tokio_postgres::types::Json;

extern crate emoji_logger;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Data1 {
    // Define your data structure
    random_num: i32,
    random_float: f64,
    md5: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Data2 {
    id: Uuid,
    name: String,
    special_code: String
}

#[derive(Deserialize)]
struct QueryParams {
    string_match: String, // 'like' or 'exact'
    search_string: String, // 'xyz' or '123-xyz' > basically any search string
}

#[derive(Display, From, Debug)]
enum CustomError {
    DatabaseError,
    InvalidData,
    QueryError,
}
impl std::error::Error for CustomError {}
impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            CustomError::DatabaseError => HttpResponse::InternalServerError().finish(),
            CustomError::InvalidData => HttpResponse::BadRequest().finish(),
            CustomError::QueryError => HttpResponse::BadRequest().finish(),
        }
    }
}

/*

Initial Setup

Make sure to create ENV file (app.rust.env)

DATABASE_URL=postgres://<USERNAME>:<PASSWORD>@<HOSTNAME>/<DBNAME>
PG.USER=<USERNAME>
PG.PASSWORD=<PASSWORD>
PG.HOST=<HOSTNAME>
PG.PORT=5432
PG.DBNAME=<DBNAME>
PG.POOL.MAX_SIZE=16
RUST_LOG=actix_web=info
*/

/*
Operations for [ table1 ]

Step-1: Manually Create Table >>

Column-1 : random_num (integer)
Column-2 : random_float (floating point number)
Column-3 : md5 (text/string)

CREATE TABLE IF NOT EXISTS table1(
   random_num INT NOT NULL,
   random_float DOUBLE PRECISION NOT NULL,
   md5 TEXT NOT NULL
);

Confirm On DB >>

# \d table1
                      Table "public.table1"
    Column    |       Type       | Collation | Nullable | Default
--------------+------------------+-----------+----------+---------
 random_num   | integer          |           | not null |
 random_float | double precision |           | not null |
 md5          | text             |           | not null |

Step-2: Manually Insert Records through SQL Query & Verify

INSERT INTO table1 (random_num, random_float, md5)
    SELECT
    floor(random()* (999-100 + 1) + 100),
    random(),
    md5(random()::text)
 from generate_series(1,100);


# select * from table1 limit 3;

 random_num |    random_float    |               md5
------------+--------------------+----------------------------------
        178 | 0.4448593893793138 | b0b71cf1f224cbcea917928eee6f9fd9
        884 | 0.9987171714499112 | 8509ec06ce85f98506f3987ef5ce6db9
        227 | 0.5995229322453817 | 107f165cd7db33830d3967d9fb9cfc04
(3 rows)

Step-3: Query Via HTTP GET Request >>

curl -X GET -H "accept: application/json" -H "content-type: application/json" http://localhost:8080/fetch_1 2>/dev/null | python -m json.tool
[
    {
        "random_num": 178,
        "random_float": 0.4448593893793138,
        "md5": "b0b71cf1f224cbcea917928eee6f9fd9"
    },
    {
        "random_num": 884,
        "random_float": 0.9987171714499112,
        "md5": "8509ec06ce85f98506f3987ef5ce6db9"
    },
    ...
    ...
    ...
    {
        "random_num": 160,
        "random_float": 0.24041538274591545,
        "md5": "ffafdc06f49c26daaf9989631f8e8de3"
    }
]

*/

/*

Operations for [ table2 ]

Step-1: Manually Create Table through SQL Query

Column-1 : id (uuid) -> primary key
Column-2 : json_data (jsonb) , example of data in this column : {"id": "<Random-UUID>", "name": "<SOME_STRING>", "specialCode": "<SOME_NUMBER>"}

CREATE TABLE IF NOT EXISTS public.table2 (
    id uuid NOT NULL,
    json_data jsonb NOT NULL,
    CONSTRAINT my_pk2 PRIMARY KEY (id)
);

Confirm On DB:

#\d public.table2
               Table "public.table2"
  Column   | Type  | Collation | Nullable | Default
-----------+-------+-----------+----------+---------
 id        | uuid  |           | not null |
 json_data | jsonb |           | not null |
Indexes:
    "my_pk2" PRIMARY KEY, btree (id)

Step-2: Insert Some Records via HTTP POST

curl -X POST http://localhost:8080/data \
-H "accept: application/json" \
-H "content-type: application/json" \
-d '{"id":"82115a42-3b60-4adf-b7c8-6a5afe73bc60", "name":"test", "specialCode": "1000"}'

curl -X POST http://localhost:8080/data \
-H "accept: application/json" \
-H "content-type: application/json" \
-d '{"id":"82115a42-3b60-4adf-b7c8-6a5afe73bc61", "name":"test2", "specialCode": "2000"}'

curl -X POST http://localhost:8080/data \
-H "accept: application/json" \
-H "content-type: application/json" \
-d '{"id":"82115a42-3b60-4adf-b7c8-6a5afe73bc62", "name":"test2", "specialCode": "3000"}'

Step-5: Connect to Database, Verify With SQL Query

# select * from public.table2;
                  id                  |                                       json_data
--------------------------------------+----------------------------------------------------------------------------------------
 82115a42-3b60-4adf-b7c8-6a5afe73bc60 | {"id": "82115a42-3b60-4adf-b7c8-6a5afe73bc60", "name": "test", "specialCode": "1000"}
 82115a42-3b60-4adf-b7c8-6a5afe73bc61 | {"id": "82115a42-3b60-4adf-b7c8-6a5afe73bc61", "name": "test2", "specialCode": "2000"}
 82115a42-3b60-4adf-b7c8-6a5afe73bc62 | {"id": "82115a42-3b60-4adf-b7c8-6a5afe73bc62", "name": "test2", "specialCode": "3000"}
(3 rows)

Step-6: Verify/Fetch Records Through HTTP GET API

curl -X GET -H "accept: application/json" -H "content-type: application/json" http://localhost:8080/fetch_2 2>/dev/null | python -m json.tool
[
    {
        "id": "82115a42-3b60-4adf-b7c8-6a5afe73bc60",
        "name": "test",
        "specialCode": "1000"
    },
    {
        "id": "82115a42-3b60-4adf-b7c8-6a5afe73bc61",
        "name": "test2",
        "specialCode": "2000"
    },
    {
        "id": "82115a42-3b60-4adf-b7c8-6a5afe73bc62",
        "name": "test2",
        "specialCode": "3000"
    }
]

*/


/*
    Make sure you populate the (rust.app.env) file
*/
async fn make_db_pool() -> Pool {
    dotenv().ok();
    dotenv::from_filename("app.rust.env").ok();

    let mut cfg = Config::new();
    cfg.host = Option::from(env::var("PG.HOST").unwrap());
    cfg.user = Option::from(env::var("PG.USER").unwrap());
    cfg.password = Option::from(env::var("PG.PASSWORD").unwrap());
    cfg.dbname = Option::from(env::var("PG.DBNAME").unwrap());
    let pool: Pool = cfg.create_pool(None, tokio_postgres::NoTls).unwrap();
    pool
}


async fn insert_2(client: &Client, sample: &Data2) -> Result<u64, tokio_postgres::Error> {
    client
        .execute(
            "INSERT INTO public.table2 (id, json_data) VALUES ($1, $2)",
            &[&sample.id, &Json(&sample)],
        )
        .await
}

async fn get_data_1(db_pool: web::Data<Pool>) -> Result<HttpResponse, CustomError>  {
    let client: Client = db_pool.get().await.unwrap();

    let rows = client.query("SELECT * from table1", &[]).await.map_err(|_| CustomError::DatabaseError)?;

    let mut structs:Vec<Data1> = Vec::new();

    for row in rows {
        let random_num: i32  = row.get("random_num");
        let random_float: f64  = row.get("random_float");
        let md5: String  = row.get("md5");

        let my_struct = Data1 {
            random_num,
            random_float,
            md5,
        };
        structs.push(my_struct)
    }

    match serde_json::to_string(&structs) {
        Ok(my_data) => {
          Ok(HttpResponse::Ok().content_type("application/json").body(my_data))
        },
        Err(_) => {
            Err(CustomError::DatabaseError)
        }
    }
}


async fn get_data_2(db_pool: web::Data<Pool>) -> Result<HttpResponse, CustomError>  {
    let client: Client = db_pool.get().await.unwrap();

    let rows = client.query("SELECT json_data from public.table2", &[]).await.map_err(|_| CustomError::DatabaseError)?;

    let mut structs:Vec<Data2> = Vec::new();

    for row in rows {
        let json_data: Json<Data2> = row.get("json_data");
        println!("{:#?}", json_data);
        let my_struct = Data2 {
          id: json_data.0.id,
          special_code: json_data.0.special_code,
          name: json_data.0.name,
        };
        structs.push(my_struct)
    }

    match serde_json::to_string(&structs) {
        Ok(my_data) => {
          Ok(HttpResponse::Ok().content_type("application/json").body(my_data))
        },
        Err(_) => {
            Err(CustomError::DatabaseError)
        }
    }
}

/*

PostgreSQL Query For Query JSON Fields:

db=# select * from table2;
                  id                  |                                       json_data
--------------------------------------+----------------------------------------------------------------------------------------
 82115a42-3b60-4adf-b7c8-6a5afe73bc60 | {"id": "82115a42-3b60-4adf-b7c8-6a5afe73bc60", "name": "test", "specialCode": "1000"}
 82115a42-3b60-4adf-b7c8-6a5afe73bc61 | {"id": "82115a42-3b60-4adf-b7c8-6a5afe73bc61", "name": "test2", "specialCode": "2000"}
 82115a42-3b60-4adf-b7c8-6a5afe73bc62 | {"id": "82115a42-3b60-4adf-b7c8-6a5afe73bc62", "name": "test2", "specialCode": "3000"}
(3 rows)

db=# select * from table2 where json_data->>'id' = '82115a42-3b60-4adf-b7c8-6a5afe73bc60';
                  id                  |                                       json_data
--------------------------------------+---------------------------------------------------------------------------------------
 82115a42-3b60-4adf-b7c8-6a5afe73bc60 | {"id": "82115a42-3b60-4adf-b7c8-6a5afe73bc60", "name": "test", "specialCode": "1000"}
(1 row)

db=# select * from table2 where json_data->>'id' like '%6a5afe73bc60%';
                  id                  |                                       json_data
--------------------------------------+---------------------------------------------------------------------------------------
 82115a42-3b60-4adf-b7c8-6a5afe73bc60 | {"id": "82115a42-3b60-4adf-b7c8-6a5afe73bc60", "name": "test", "specialCode": "1000"}
(1 row)

*/

/*

Sample Outputs For Below Function : get_data_with_query_2(...) >>

Pattern Match (string_match=like)

curl -X GET -H "accept: application/json" -H "content-type: application/json" "http://localhost:8080/fetch_with_query_2?string_match=like&search_string=6a5afe73bc60" 2>/dev/null | python -m json.tool
[
    {
        "id": "82115a42-3b60-4adf-b7c8-6a5afe73bc60",
        "name": "test",
        "specialCode": "1000"
    }
]

Exact Match (string_match=exact)

curl -X GET -H "accept: application/json" -H "content-type: application/json" "http://localhost:8080/fetch_with_query_2?string_match=exact&search_string=82115a42-3b60-4adf-b7c8-6a5afe73bc60" 2>/dev/null | python -m json.tool
[
    {
        "id": "82115a42-3b60-4adf-b7c8-6a5afe73bc60",
        "name": "test",
        "specialCode": "1000"
    }
]

*/
async fn get_data_with_query_2(db_pool: web::Data<Pool>, query: web::Query<QueryParams>) -> Result<HttpResponse, CustomError>  {
    let client: Client = db_pool.get().await.unwrap();

    println!("query >> \n\nstring_match => [ {} ] , \n\nsearch_string => [ {} ]\n\n", query.string_match, query.search_string);

    if !(query.string_match.to_string() == "exact" || query.string_match.to_string() == "like") {
        return Err(CustomError::QueryError)
    }

    let sanitized_str = sanitize_string(query.search_string.as_str()).await;

    let mut inner_query = "".to_string();

    if query.string_match.as_str() == "exact" { // exact string match
        // search all JSON fields for possible match
        // this can also be applied if there are other columns
        inner_query = inner_query + " ( ";
        inner_query = inner_query + format!(" json_data->>'id' = '{}' ", sanitized_str).as_str();
        inner_query = inner_query + format!(" OR json_data->>'name' = '{}' ", sanitized_str).as_str();
        inner_query = inner_query + format!(" OR json_data->>'specialCode' = '{}' ", sanitized_str).as_str();
        inner_query = inner_query + " ) ";
    } else if query.string_match.as_str() == "like" { // pattern match
        // search all JSON fields for possible match
        // this can also be applied if there are other columns
        inner_query = inner_query + " ( ";
        inner_query = inner_query + format!(" json_data->>'id' like '%{}%' ", sanitized_str).as_str();
        inner_query = inner_query + format!(" OR json_data->>'name' like '%{}%' ", sanitized_str).as_str();
        inner_query = inner_query + format!(" OR json_data->>'specialCode' like '%{}%' ", sanitized_str).as_str();
        inner_query = inner_query + " ) ";
    } else {
        return Err(CustomError::QueryError)
    }

    println!("inner_query >> \n\n[ {} ]\n\n",inner_query);

    let complete_query = format!("SELECT json_data from public.table2 WHERE {}", inner_query);

    let rows = client.query(complete_query.as_str(), &[]).await.map_err(|_| CustomError::DatabaseError)?;

    let mut structs:Vec<Data2> = Vec::new();

    for row in rows {
        let json_data: Json<Data2> = row.get("json_data");
        println!("{:#?}", json_data);
        let my_struct = Data2 {
          id: json_data.0.id,
          special_code: json_data.0.special_code,
          name: json_data.0.name,
        };
        structs.push(my_struct)
    }

    match serde_json::to_string(&structs) {
        Ok(my_data) => {
          Ok(HttpResponse::Ok().content_type("application/json").body(my_data))
        },
        Err(_) => {
            Err(CustomError::DatabaseError)
        }
    }
}


/*
This function will receive data via POST Request
Example:
curl -X POST http://localhost:8080/data \
-H "accept: application/json" \
-H "content-type: application/json" \
-d '{"id":"82115a42-3b60-4adf-b7c8-6a5afe73bc60", "name":"test", "specialCode": "1000"}'
*/
async fn receive_data(record: web::Json<Data2>, db_pool: web::Data<Pool>) -> Result<HttpResponse, CustomError> {
    let client: Client = db_pool
        .get()
        .await
        .map_err(|_| CustomError::DatabaseError)?;

    insert_2(&client, &record)
        .await
        .map_err(|_| CustomError::InvalidData)?;

    Ok(HttpResponse::Ok().body(record.id.to_hyphenated().to_string()))
}

async fn get_connection(db_pool: Data<Pool>) -> Result<Client, String> {
    db_pool.get().await.map_err(|err| err.to_string())
}

#[get("/api/health")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("hi! 👋")
}

/*
Testing sanitize_string(...)

fn main() {
    let original_string = "Hello! This is a test string with various characters: #$%^&*()";
    let sanitized = sanitize_string(original_string);
    println!("Sanitized string: {}", sanitized);
}
*/

async fn sanitize_string(input: &str) -> String {
    input.chars()
        .filter(|&c| c.is_ascii_alphanumeric() || "_./-@,".contains(c))
        .collect()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv::from_filename("app.rust.env").ok();

    println!("🧑‍🔬 Sample Service Starting");

    let pool = make_db_pool().await;

    std::env::set_var("RUST_LOG", "actix_web=debug");
    emoji_logger::init();

    let result = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(pool.clone()))
            .service(hello_world)
            .route("data", web::post().to(receive_data))
            .route("fetch_2", web::get().to(get_data_2))
            .route("fetch_with_query_2", web::get().to(get_data_with_query_2))
            .route("fetch_1", web::get().to(get_data_1))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await;

    println!("🧑‍🔬 Sample Service Stopping");

    result
}
```

#### [Sanitize String And Split String](#sanitize-string-and-split-string)

Please see the output below inside the code. This will >

- Sanitize the string (removed not needed characters)
- Remove `leading` AND `trailing` '#' character
- Replace multiple repetitions of `#` character in-between with only single `#`
- Then return the `Vec<String>` by splitting based of `#` character




```rust
use regex::Regex;

fn sanitize_string(input: &str) -> String {
    input.chars()
        .filter(|&c| c.is_ascii_alphanumeric() || "_./-@,#".contains(c))
        .collect()
}

fn remove_leading_trailing_hashes(input: &str) -> String {
    input.trim_start_matches("#").trim_end_matches("#").to_string()
}

fn replace_multiple_hashes(input: &str) -> String {
    let re = Regex::new(r"#+").unwrap();
    re.replace_all(input, "#").to_string()
}

fn split_string(input: &str) -> Vec<String> {
    input.split("#").map(|s| s.to_string()).collect()
}

fn get_items(my_str: &str) -> Vec<String> {
    let sanitized_str = sanitize_string(my_str);
    
    let updated_str = remove_leading_trailing_hashes(sanitized_str.as_str());
    println!("updated_str : {}", updated_str);
    
    let validated_updated_str = replace_multiple_hashes(updated_str.as_str());
    println!("validated_updated_str : {}", validated_updated_str);
    
    let my_list = split_string(validated_updated_str.as_str());
    println!("my_list : {:?}", my_list);
    
    println!("\n");
    
    my_list
}

fn main() {
    let original_string = "###abc#555#xyz-123###";
    let my_items = get_items(original_string);
    
    let original_string = "abcd";
    let my_items = get_items(original_string);
    
    let original_string = "###abc##555####xyz-123#3-2-1#a-b-c###";
    let my_items = get_items(original_string);
}

/*

Output:

updated_str : abc#555#xyz-123
validated_updated_str : abc#555#xyz-123
my_list : ["abc", "555", "xyz-123"]


updated_str : abcd
validated_updated_str : abcd
my_list : ["abcd"]


updated_str : abc##555####xyz-123#3-2-1#a-b-c
validated_updated_str : abc#555#xyz-123#3-2-1#a-b-c
my_list : ["abc", "555", "xyz-123", "3-2-1", "a-b-c"]

*/
```

#### [Actix REST API With Advanced Query And PostgreSQL Backend](#actix-rest-api-with-advanced-query-and-postgresql-backend)

> Please see the comments inside the code, which shows HTTP GET Request and corresponding output

FYI : If your backend table can have NULL values, then setup your `struct` with `Option<..>` this way >

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Data1 {
    // Define your data structure
    random_num: Option<i32>,
    random_float: Option<f64>,
    md5: Option<String>,
}

```

`src/main.rs`

```rust
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use tokio_postgres::{Error, row};
use tokio_postgres::NoTls;
use actix_web::{get, post, web, web::Data, App, HttpResponse, HttpServer};
use actix_web::{Responder, HttpRequest, http, middleware, middleware::Logger, ResponseError};
use futures::future;
use futures::Future;
use uuid::Uuid;
use deadpool_postgres::{Config, Client, Pool};
use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::*;
use tokio::runtime::Runtime;
use ::config::ConfigError;
use dotenv::dotenv;
use tokio_postgres::tls::MakeTlsConnect;
use derive_more::{Display,From};
use tokio_postgres::types::Json;
use regex::Regex;
use serde_json::to_string;
use dirs;
extern crate emoji_logger;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Data1 {
    // Define your data structure
    random_num: i32,
    random_float: f64,
    md5: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Data2 {
    id: Uuid,
    name: String,
    special_code: String
}

#[derive(Deserialize)]
struct QueryParams {
    string_match: String, // 'like' or 'exact'
    search_string: String, // 'xyz' or '123-xyz' > basically any search string
}

#[derive(Deserialize)]
struct QueryParamsAdvanced {
    //----------------------------------------------------
    // examples of value for (string_match)
    // 'like' or 'exact'
    string_match: String,
    //----------------------------------------------------
    // examples of value for (search_type)
    // this can be 'or' (OR) 'and'
    // 'or' means , search for 'abc' | 'xyz' etc
    // 'and' means , search for 'abc' + 'xyz'
    search_type: String,
    //----------------------------------------------------
    // examples of value for (search_string) > basically any search string
    // 'xyz'
    // '123-xyz'
    // 'xyz,555,abc-123' (',' delimited values : for multiple values)
    // if you gave (search_type=and), then search the table for all columns where : 'xyz' AND '555' AND 'abc-123' is present
    // if you gave (search_type=or), then search the table for all columns where : 'xyz' OR '555' OR 'abc-123' is present
    search_string: String,
    //----------------------------------------------------
    source_table: String, // ex: 't_1'
}

#[derive(Display, From, Debug)]
enum CustomError {
    DatabaseError,
    InvalidData,
    QueryError,
}
impl std::error::Error for CustomError {}
impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            CustomError::DatabaseError => HttpResponse::InternalServerError().finish(),
            CustomError::InvalidData => HttpResponse::BadRequest().finish(),
            CustomError::QueryError => HttpResponse::BadRequest().finish(),
        }
    }
}


impl QueryParamsAdvanced {
    fn validate(&self) -> bool {
        return !self.search_type.is_empty() && !self.string_match.is_empty() && !self.search_string.is_empty()
    }
}

/*

Initial Setup

Make sure to create ENV file (app.rust.env)

DATABASE_URL=postgres://<USERNAME>:<PASSWORD>@<HOSTNAME>/<DBNAME>
PG.USER=<USERNAME>
PG.PASSWORD=<PASSWORD>
PG.HOST=<HOSTNAME>
PG.PORT=5432
PG.DBNAME=<DBNAME>
PG.POOL.MAX_SIZE=16
RUST_LOG=actix_web=info
*/

/*
Operations for [ table1 ]

Step-1: Manually Create Table >>

Column-1 : random_num (integer)
Column-2 : random_float (floating point number)
Column-3 : md5 (text/string)

CREATE TABLE IF NOT EXISTS table1(
   random_num INT NOT NULL,
   random_float DOUBLE PRECISION NOT NULL,
   md5 TEXT NOT NULL
);

Confirm On DB >>

# \d table1
                      Table "public.table1"
    Column    |       Type       | Collation | Nullable | Default
--------------+------------------+-----------+----------+---------
 random_num   | integer          |           | not null |
 random_float | double precision |           | not null |
 md5          | text             |           | not null |

Step-2: Manually Insert Records through SQL Query & Verify

INSERT INTO table1 (random_num, random_float, md5)
    SELECT
    floor(random()* (999-100 + 1) + 100),
    random(),
    md5(random()::text)
 from generate_series(1,100);


# select * from table1 limit 3;

 random_num |    random_float    |               md5
------------+--------------------+----------------------------------
        178 | 0.4448593893793138 | b0b71cf1f224cbcea917928eee6f9fd9
        884 | 0.9987171714499112 | 8509ec06ce85f98506f3987ef5ce6db9
        227 | 0.5995229322453817 | 107f165cd7db33830d3967d9fb9cfc04
(3 rows)

Step-3: Query Via HTTP GET Request >>

curl -X GET -H "accept: application/json" -H "content-type: application/json" http://localhost:8080/fetch_1 2>/dev/null | python -m json.tool
[
    {
        "random_num": 178,
        "random_float": 0.4448593893793138,
        "md5": "b0b71cf1f224cbcea917928eee6f9fd9"
    },
    {
        "random_num": 884,
        "random_float": 0.9987171714499112,
        "md5": "8509ec06ce85f98506f3987ef5ce6db9"
    },
    ...
    ...
    ...
    {
        "random_num": 160,
        "random_float": 0.24041538274591545,
        "md5": "ffafdc06f49c26daaf9989631f8e8de3"
    }
]

*/

/*

Operations for [ table2 ]

Step-1: Manually Create Table through SQL Query

Column-1 : id (uuid) -> primary key
Column-2 : json_data (jsonb) , example of data in this column : {"id": "<Random-UUID>", "name": "<SOME_STRING>", "specialCode": "<SOME_NUMBER>"}

CREATE TABLE IF NOT EXISTS public.table2 (
    id uuid NOT NULL,
    json_data jsonb NOT NULL,
    CONSTRAINT my_pk2 PRIMARY KEY (id)
);

Confirm On DB:

#\d public.table2
               Table "public.table2"
  Column   | Type  | Collation | Nullable | Default
-----------+-------+-----------+----------+---------
 id        | uuid  |           | not null |
 json_data | jsonb |           | not null |
Indexes:
    "my_pk2" PRIMARY KEY, btree (id)

Step-2: Insert Some Records via HTTP POST

curl -X POST http://localhost:8080/data \
-H "accept: application/json" \
-H "content-type: application/json" \
-d '{"id":"82115a42-3b60-4adf-b7c8-6a5afe73bc60", "name":"test", "specialCode": "1000"}'

curl -X POST http://localhost:8080/data \
-H "accept: application/json" \
-H "content-type: application/json" \
-d '{"id":"82115a42-3b60-4adf-b7c8-6a5afe73bc61", "name":"test2", "specialCode": "2000"}'

curl -X POST http://localhost:8080/data \
-H "accept: application/json" \
-H "content-type: application/json" \
-d '{"id":"82115a42-3b60-4adf-b7c8-6a5afe73bc62", "name":"test2", "specialCode": "3000"}'

Step-5: Connect to Database, Verify With SQL Query

# select * from public.table2;
                  id                  |                                       json_data
--------------------------------------+----------------------------------------------------------------------------------------
 82115a42-3b60-4adf-b7c8-6a5afe73bc60 | {"id": "82115a42-3b60-4adf-b7c8-6a5afe73bc60", "name": "test", "specialCode": "1000"}
 82115a42-3b60-4adf-b7c8-6a5afe73bc61 | {"id": "82115a42-3b60-4adf-b7c8-6a5afe73bc61", "name": "test2", "specialCode": "2000"}
 82115a42-3b60-4adf-b7c8-6a5afe73bc62 | {"id": "82115a42-3b60-4adf-b7c8-6a5afe73bc62", "name": "test2", "specialCode": "3000"}
(3 rows)

Step-6: Verify/Fetch Records Through HTTP GET API

curl -X GET -H "accept: application/json" -H "content-type: application/json" http://localhost:8080/fetch_2 2>/dev/null | python -m json.tool
[
    {
        "id": "82115a42-3b60-4adf-b7c8-6a5afe73bc60",
        "name": "test",
        "specialCode": "1000"
    },
    {
        "id": "82115a42-3b60-4adf-b7c8-6a5afe73bc61",
        "name": "test2",
        "specialCode": "2000"
    },
    {
        "id": "82115a42-3b60-4adf-b7c8-6a5afe73bc62",
        "name": "test2",
        "specialCode": "3000"
    }
]

*/


/*
    Make sure you populate the (rust.app.env) file
*/
async fn make_db_pool() -> Pool {
    dotenv().ok();
    dotenv::from_filename("app.rust.env").ok();

    let mut cfg = Config::new();
    cfg.host = Option::from(env::var("PG.HOST").unwrap());
    cfg.user = Option::from(env::var("PG.USER").unwrap());
    cfg.password = Option::from(env::var("PG.PASSWORD").unwrap());
    cfg.dbname = Option::from(env::var("PG.DBNAME").unwrap());
    let pool: Pool = cfg.create_pool(None, tokio_postgres::NoTls).unwrap();
    pool
}


async fn insert_2(client: &Client, sample: &Data2) -> Result<u64, tokio_postgres::Error> {
    client
        .execute(
            "INSERT INTO public.table2 (id, json_data) VALUES ($1, $2)",
            &[&sample.id, &Json(&sample)],
        )
        .await
}

async fn get_data_1(db_pool: web::Data<Pool>) -> Result<HttpResponse, CustomError>  {
    let client: Client = db_pool.get().await.unwrap();

    let rows = client.query("SELECT * from table1", &[]).await.map_err(|_| CustomError::DatabaseError)?;

    let mut structs:Vec<Data1> = Vec::new();

    for row in rows {
        let random_num: i32  = row.get("random_num");
        let random_float: f64  = row.get("random_float");
        let md5: String  = row.get("md5");

        let my_struct = Data1 {
            random_num,
            random_float,
            md5,
        };
        structs.push(my_struct)
    }

    match serde_json::to_string(&structs) {
        Ok(my_data) => {
          Ok(HttpResponse::Ok().content_type("application/json").body(my_data))
        },
        Err(_) => {
            Err(CustomError::DatabaseError)
        }
    }
}


async fn get_data_2(db_pool: web::Data<Pool>) -> Result<HttpResponse, CustomError>  {
    let client: Client = db_pool.get().await.unwrap();

    let rows = client.query("SELECT json_data from public.table2", &[]).await.map_err(|_| CustomError::DatabaseError)?;

    let mut structs:Vec<Data2> = Vec::new();

    for row in rows {
        let json_data: Json<Data2> = row.get("json_data");
        println!("{:#?}", json_data);
        let my_struct = Data2 {
          id: json_data.0.id,
          special_code: json_data.0.special_code,
          name: json_data.0.name,
        };
        structs.push(my_struct)
    }

    match serde_json::to_string(&structs) {
        Ok(my_data) => {
          Ok(HttpResponse::Ok().content_type("application/json").body(my_data))
        },
        Err(_) => {
            Err(CustomError::DatabaseError)
        }
    }
}

/*

PostgreSQL Query For Query JSON Fields:

# select * from table2;
                  id                  |                                       json_data
--------------------------------------+----------------------------------------------------------------------------------------
 82115a42-3b60-4adf-b7c8-6a5afe73bc60 | {"id": "82115a42-3b60-4adf-b7c8-6a5afe73bc60", "name": "test", "specialCode": "1000"}
 82115a42-3b60-4adf-b7c8-6a5afe73bc61 | {"id": "82115a42-3b60-4adf-b7c8-6a5afe73bc61", "name": "test2", "specialCode": "2000"}
 82115a42-3b60-4adf-b7c8-6a5afe73bc62 | {"id": "82115a42-3b60-4adf-b7c8-6a5afe73bc62", "name": "test2", "specialCode": "3000"}
(3 rows)

# select * from table2 where json_data->>'id' = '82115a42-3b60-4adf-b7c8-6a5afe73bc60';
                  id                  |                                       json_data
--------------------------------------+---------------------------------------------------------------------------------------
 82115a42-3b60-4adf-b7c8-6a5afe73bc60 | {"id": "82115a42-3b60-4adf-b7c8-6a5afe73bc60", "name": "test", "specialCode": "1000"}
(1 row)

# select * from table2 where json_data->>'id' like '%6a5afe73bc60%';
                  id                  |                                       json_data
--------------------------------------+---------------------------------------------------------------------------------------
 82115a42-3b60-4adf-b7c8-6a5afe73bc60 | {"id": "82115a42-3b60-4adf-b7c8-6a5afe73bc60", "name": "test", "specialCode": "1000"}
(1 row)

*/

/*

Sample Outputs For Below Function : get_data_with_query_2(...) >>

Pattern Match (string_match=like)

curl -X GET -H "accept: application/json" -H "content-type: application/json" "http://localhost:8080/fetch_with_query_2?string_match=like&search_string=6a5afe73bc60" 2>/dev/null | python -m json.tool
[
    {
        "id": "82115a42-3b60-4adf-b7c8-6a5afe73bc60",
        "name": "test",
        "specialCode": "1000"
    }
]

Exact Match (string_match=exact)

curl -X GET -H "accept: application/json" -H "content-type: application/json" "http://localhost:8080/fetch_with_query_2?string_match=exact&search_string=82115a42-3b60-4adf-b7c8-6a5afe73bc60" 2>/dev/null | python -m json.tool
[
    {
        "id": "82115a42-3b60-4adf-b7c8-6a5afe73bc60",
        "name": "test",
        "specialCode": "1000"
    }
]

*/
async fn get_data_with_query_2(db_pool: web::Data<Pool>, query: web::Query<QueryParams>) -> Result<HttpResponse, CustomError>  {
    let client: Client = db_pool.get().await.unwrap();

    println!("query >> \n\nstring_match => [ {} ] , \n\nsearch_string => [ {} ]\n\n", query.string_match, query.search_string);

    if !(query.string_match.to_string() == "exact" || query.string_match.to_string() == "like") {
        return Err(CustomError::QueryError)
    }

    let sanitized_str = sanitize_string(query.search_string.as_str()).await;

    let mut inner_query = "".to_string();

    if query.string_match.as_str() == "exact" { // exact string match
        // search all JSON fields for possible match
        // this can also be applied if there are other columns
        inner_query = inner_query + " ( ";
        inner_query = inner_query + format!(" json_data->>'id' = '{}' ", sanitized_str).as_str();
        inner_query = inner_query + format!(" OR json_data->>'name' = '{}' ", sanitized_str).as_str();
        inner_query = inner_query + format!(" OR json_data->>'specialCode' = '{}' ", sanitized_str).as_str();
        inner_query = inner_query + " ) ";
    } else if query.string_match.as_str() == "like" { // pattern match
        // search all JSON fields for possible match
        // this can also be applied if there are other columns
        inner_query = inner_query + " ( ";
        inner_query = inner_query + format!(" json_data->>'id' like '%{}%' ", sanitized_str).as_str();
        inner_query = inner_query + format!(" OR json_data->>'name' like '%{}%' ", sanitized_str).as_str();
        inner_query = inner_query + format!(" OR json_data->>'specialCode' like '%{}%' ", sanitized_str).as_str();
        inner_query = inner_query + " ) ";
    } else {
        return Err(CustomError::QueryError)
    }

    println!("inner_query >> \n\n[ {} ]\n\n",inner_query);

    let complete_query = format!("SELECT json_data from public.table2 WHERE {}", inner_query);

    let rows = client.query(complete_query.as_str(), &[]).await.map_err(|_| CustomError::DatabaseError)?;

    let mut structs:Vec<Data2> = Vec::new();

    for row in rows {
        let json_data: Json<Data2> = row.get("json_data");
        println!("{:#?}", json_data);
        let my_struct = Data2 {
          id: json_data.0.id,
          special_code: json_data.0.special_code,
          name: json_data.0.name,
        };
        structs.push(my_struct)
    }

    match serde_json::to_string(&structs) {
        Ok(my_data) => {
          Ok(HttpResponse::Ok().content_type("application/json").body(my_data))
        },
        Err(_) => {
            Err(CustomError::DatabaseError)
        }
    }
}

/*
To test the below function, we will use data from the table which was already populated >>

# select * from public.table1 limit 20;

 random_num |    random_float     |               md5
------------+---------------------+----------------------------------
        178 |  0.4448593893793138 | b0b71cf1f224cbcea917928eee6f9fd9
        884 |  0.9987171714499112 | 8509ec06ce85f98506f3987ef5ce6db9
        227 |  0.5995229322453817 | 107f165cd7db33830d3967d9fb9cfc04
        748 |  0.5057753720795759 | f61298bd0d5c5a6d9221ec23a8f04254
        965 | 0.01908963305890765 | 3be05140d21c1ed4826c048716780d20
        780 |  0.1196140984027636 | 605244cc2944631e428c62452ea503fb
        527 |  0.8780820632179207 | b825cb9a9bbf1af5bab08a0f876b1b9d
        772 |  0.5709867230460972 | 37d27891706dac89e96fe065d0895a20
        835 | 0.08642430332279716 | e08dc8ffb4ada40546b1978c345a7693
        133 |  0.5025188130212683 | 19acaa9451099cb90faa52647f163629
        172 | 0.09138717604981039 | 9718bccd47be4004494082f51694a3fb
        502 |  0.9575964213380033 | 237e13853c28fdd85b19a69e9761c0d4
        696 |  0.1575453174179664 | eb24c501fcd437e51833af91f2b001f7
        934 | 0.10996602314173742 | 1d0ab023f983d0f9839ba03962b23f10
        580 |  0.9346545416868715 | b80147c8dc29ac799973e459d2330e26
        672 | 0.11810111293834069 | 8dfd3cb0a70634cef7eba8ff057e3d4c
        143 | 0.06132282664914257 | 82831fd645a597274ae06fa45a9f785d
        586 |  0.9936290273894173 | a6f23a47f4960358c71d8687c23c37fe
        973 |  0.7346715379786453 | af26aa7f82cbe24760516710edfb296b
        529 |  0.2696997793540632 | 9756fe86e3c903fbf18236d01de02692
(20 rows)

Example of HTTP GET Requests >>

http://localhost:8080/fetch_with_query_advanced_2?string_match=like&search_string=696,586&search_type=or&source_table=t_1

Output >>

table_columns : [
    "random_num",
    "random_float",
    "md5",
]
total_combinations : 6
string_match : like
search_type : or
inner_query >>

 (  (  random_num::text like '%696%' OR  random_float::text like '%696%' OR  md5::text like '%696%'  ) OR  (  random_num::text like '%586%' OR  random_float::text like '%586%' OR  md5::text like '%586%'  )  )


complete_query >>

SELECT * from public.table1 WHERE  (  (  random_num::text like '%696%' OR  random_float::text like '%696%' OR  md5::text like '%696%'  ) OR  (  random_num::text like '%586%' OR  random_float::text like '%586%' OR  md5::text like '%586%'  )  )

---------------------------------------------------------------------------------------------------

http://localhost:8080/fetch_with_query_advanced_2?string_match=like&search_string=e08dc8f,3a8f04254&search_type=or&source_table=t_1

Output >>

table_columns : [
    "random_num",
    "random_float",
    "md5",
]
total_combinations : 6
string_match : like
search_type : or
inner_query >>

 (  (  random_num::text like '%e08dc8f%' OR  random_float::text like '%e08dc8f%' OR  md5::text like '%e08dc8f%'  ) OR  (  random_num::text like '%3a8f04254%' OR  random_float::text like '%3a8f04254%' OR  md5::text like '%3a8f04254%'  )  )


complete_query >>

SELECT * from public.table1 WHERE  (  (  random_num::text like '%e08dc8f%' OR  random_float::text like '%e08dc8f%' OR  md5::text like '%e08dc8f%'  ) OR  (  random_num::text like '%3a8f04254%' OR  random_float::text like '%3a8f04254%' OR  md5::text like '%3a8f04254%'  )  )

---------------------------------------------------------------------------------------------------

http://localhost:8080/fetch_with_query_advanced_2?string_match=like&search_string=9718bccd47be4004494082f51694a3fb&search_type=and&source_table=t_1

Output >>

table_columns : [
    "random_num",
    "random_float",
    "md5",
]
total_combinations : 3
string_match : like
search_type : and
inner_query >>

 (  (  random_num::text like '%9718bccd47be4004494082f51694a3fb%' OR  random_float::text like '%9718bccd47be4004494082f51694a3fb%' OR  md5::text like '%9718bccd47be4004494082f51694a3fb%'  )  )


complete_query >>

SELECT * from public.table1 WHERE  (  (  random_num::text like '%9718bccd47be4004494082f51694a3fb%' OR  random_float::text like '%9718bccd47be4004494082f51694a3fb%' OR  md5::text like '%9718bccd47be4004494082f51694a3fb%'  )  )

---------------------------------------------------------------------------------------------------

http://localhost:8080/fetch_with_query_advanced_2?string_match=like&search_string=696,586&search_type=or&source_table=t_1

Output >>

table_columns : [
    "random_num",
    "random_float",
    "md5",
]
total_combinations : 6
string_match : like
search_type : or
inner_query >>

 (  (  random_num::text like '%696%' OR  random_float::text like '%696%' OR  md5::text like '%696%'  ) OR  (  random_num::text like '%586%' OR  random_float::text like '%586%' OR  md5::text like '%586%'  )  )


complete_query >>

SELECT * from public.table1 WHERE  (  (  random_num::text like '%696%' OR  random_float::text like '%696%' OR  md5::text like '%696%'  ) OR  (  random_num::text like '%586%' OR  random_float::text like '%586%' OR  md5::text like '%586%'  )  )

---------------------------------------------------------------------------------------------------

*/

async fn get_data_with_advanced_query_2(db_pool: web::Data<Pool>, query: web::Query<QueryParamsAdvanced>) -> Result<HttpResponse, CustomError>  {

    if !query.validate() {
        return Ok(HttpResponse::BadRequest().content_type("application/json").body("{\"error\" : \"please pass all the required data : 'string_match' , 'search_string' and 'search_type'\"}"))
    }

    let client: Client = db_pool.get().await.unwrap();

    println!("query >> \n\nstring_match => [ {} ] , \n\nsearch_string => [ {} ]\n\n", query.string_match, query.search_string);



    if query.source_table.is_empty() {
        return Ok(HttpResponse::BadRequest().content_type("application/json").body("{\"error\" : \"source_table cannot be empty !\"}"))
    }

    let mut actual_db_table = "".to_string();
    let query_table = query.source_table.to_owned();
    actual_db_table = match query_table.as_str() {
        "t_1" => "table1".to_string(),
        _ => return Ok(HttpResponse::BadRequest().content_type("application/json").body("{\"error\" : \"source_table is not valid !\"}"))
    };

    if !(query.string_match.to_string() == "exact" || query.string_match.to_string() == "like") {
        return Ok(HttpResponse::BadRequest().content_type("application/json").body("{\"error\" : \"string_match should be 'exact' (OR) 'like'\"}"))
    }

    if !(query.search_type.to_string() == "and" || query.search_type.to_string() == "or") {
        return Ok(HttpResponse::BadRequest().content_type("application/json").body("{\"error\" : \"search_type should be 'and' (OR) 'or'\"}"))
    }

    let search_strings = get_items(query.search_string.as_str()).await;

    let length_of_search_strings = search_strings.len() as i32;

    if length_of_search_strings < 1 {
        return Ok(HttpResponse::BadRequest().content_type("application/json").body("{\"error\" : \"total search string should be greater than or equal to 1 (separated by ',')\"}"))
    }

    let mut table_columns = vec![];

    match query_table.as_str() {
        "t_1" => {
            table_columns.push("random_num");
            table_columns.push("random_float");
            table_columns.push("md5");
        },
        _ => return Ok(HttpResponse::BadRequest().content_type("application/json").body("{\"error\" : \"source_table is not valid !\"}"))
    };

    println!("table_columns : {:#?}", table_columns);

    let mut inner_query = "".to_string();

    let total_combinations = table_columns.len() as i32 * search_strings.len() as i32;

    println!("total_combinations : {}", total_combinations);

    println!("string_match : {}", query.string_match.to_string());
    println!("search_type : {}", query.search_type.to_string());

    let length_of_columns = table_columns.len() as i32;

    if length_of_columns < 1 {
        return Ok(HttpResponse::BadRequest().content_type("application/json").body("{\"error\" : \"total number of columns on PG table should be greater than or equal to 1\"}"))
    }

    if query.string_match.as_str() == "exact" { // exact string match
        // search all JSON fields for possible match
        // this can also be applied if there are other columns
        if query.search_type.to_string() == "and" {
            inner_query = inner_query + " ( ";
            let mut search_string_counter = 1;
            for my_search_str in search_strings.to_owned() {
                inner_query = inner_query + " ( ";
                let mut column_counter = 1;
                // --------------------------------------
                for my_column in table_columns.to_owned() {
                    if column_counter == table_columns.len() as i32 {
                        inner_query = inner_query + format!(" lower({}::text) = lower('{}') ", my_column.to_string(), my_search_str.to_lowercase()).as_str();
                    } else {
                        inner_query = inner_query + format!(" lower({}::text) = lower('{}') OR ", my_column.to_string(), my_search_str.to_lowercase()).as_str();
                    }
                    column_counter += 1;
                }
                if search_string_counter == search_strings.len() as i32 {
                    inner_query = inner_query + " ) ";
                } else {
                    inner_query = inner_query + " ) AND ";
                }
                search_string_counter += 1;
                // --------------------------------------
            }
            inner_query = inner_query + " ) ";
        } else if query.search_type.to_string() == "or" {
            inner_query = inner_query + " ( ";
            let mut search_string_counter = 1;
            for my_search_str in search_strings.to_owned() {
                inner_query = inner_query + " ( ";
                let mut column_counter = 1;
                // --------------------------------------
                for my_column in table_columns.to_owned() {
                    if column_counter == table_columns.len() as i32 {
                        inner_query = inner_query + format!(" lower({}::text) = lower('{}') ", my_column.to_string(), my_search_str.to_lowercase()).as_str();
                    } else {
                        inner_query = inner_query + format!(" lower({}::text) = lower('{}') OR ", my_column.to_string(), my_search_str.to_lowercase()).as_str();
                    }
                    column_counter += 1;
                }
                if search_string_counter == search_strings.len() as i32 {
                    inner_query = inner_query + " ) ";
                } else {
                    inner_query = inner_query + " ) OR ";
                }
                search_string_counter += 1;
                // --------------------------------------
            }
            inner_query = inner_query + " ) ";
        } else {
            return Err(CustomError::QueryError)
        }

    } else if query.string_match.as_str() == "like" { // pattern match
        // search all JSON fields for possible match
        // this can also be applied if there are other columns
        if query.search_type.to_string() == "and" {
            inner_query = inner_query + " ( ";
            let mut search_string_counter = 1;
            for my_search_str in search_strings.to_owned() {
                inner_query = inner_query + " ( ";
                let mut column_counter = 1;
                // --------------------------------------
                for my_column in table_columns.to_owned() {
                    if column_counter == table_columns.len() as i32 {
                        inner_query = inner_query + format!(" lower({}::text) like lower('%{}%') ", my_column.to_string(), my_search_str.to_lowercase()).as_str();
                    } else {
                        inner_query = inner_query + format!(" lower({}::text) like lower('%{}%') OR ", my_column.to_string(), my_search_str.to_lowercase()).as_str();
                    }
                    column_counter += 1;
                }
                if search_string_counter == search_strings.len() as i32 {
                    inner_query = inner_query + " ) ";
                } else {
                    inner_query = inner_query + " ) AND ";
                }
                search_string_counter += 1;
                // --------------------------------------
            }
            inner_query = inner_query + " ) ";
        } else if query.search_type.to_string() == "or" {
            inner_query = inner_query + " ( ";
            let mut search_string_counter = 1;
            for my_search_str in search_strings.to_owned() {
                inner_query = inner_query + " ( ";
                let mut column_counter = 1;
                // --------------------------------------
                for my_column in table_columns.to_owned() {
                    if column_counter == table_columns.len() as i32 {
                        inner_query = inner_query + format!(" lower({}::text) like lower('%{}%') ", my_column.to_string(), my_search_str.to_lowercase()).as_str();
                    } else {
                        inner_query = inner_query + format!(" lower({}::text) like lower('%{}%') OR ", my_column.to_string(), my_search_str.to_lowercase()).as_str();
                    }
                    column_counter += 1;
                }
                if search_string_counter == search_strings.len() as i32 {
                    inner_query = inner_query + " ) ";
                } else {
                    inner_query = inner_query + " ) OR ";
                }
                search_string_counter += 1;
                // --------------------------------------
            }
            inner_query = inner_query + " ) ";
        } else {
            return Err(CustomError::QueryError)
        }
    } else {
        return Err(CustomError::QueryError)
    }

    println!("inner_query >> \n\n{}\n\n",inner_query);

    let complete_query = format!("SELECT * from public.table1 WHERE {}", inner_query);

    println!("complete_query >> \n\n{}\n\n",complete_query);

    let rows = client.query(complete_query.as_str(), &[]).await.map_err(|_| CustomError::DatabaseError)?;

    let mut structs:Vec<Data1> = Vec::new();

    for row in rows {
        let random_num = row.get("random_num");
        let random_float = row.get("random_float");
        let md5 = row.get("md5");

        let my_struct = Data1 {
            random_num,
            random_float,
            md5,
        };
        structs.push(my_struct)
    }

    match serde_json::to_string(&structs) {
        Ok(my_data) => {
          Ok(HttpResponse::Ok().content_type("application/json").body(my_data))
        },
        Err(_) => {
            Err(CustomError::DatabaseError)
        }
    }
}



/*
This function will receive data via POST Request
Example:
curl -X POST http://localhost:8080/data \
-H "accept: application/json" \
-H "content-type: application/json" \
-d '{"id":"82115a42-3b60-4adf-b7c8-6a5afe73bc60", "name":"test", "specialCode": "1000"}'
*/
async fn receive_data(record: web::Json<Data2>, db_pool: web::Data<Pool>) -> Result<HttpResponse, CustomError> {
    let client: Client = db_pool
        .get()
        .await
        .map_err(|_| CustomError::DatabaseError)?;

    insert_2(&client, &record)
        .await
        .map_err(|_| CustomError::InvalidData)?;

    Ok(HttpResponse::Ok().body(record.id.to_hyphenated().to_string()))
}

async fn get_connection(db_pool: Data<Pool>) -> Result<Client, String> {
    db_pool.get().await.map_err(|err| err.to_string())
}

#[get("/api/health")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("hi! 👋")
}

/*
Testing sanitize_string(...)

fn main() {
    let original_string = "Hello! This is a test string with various characters: #$%^&*()";
    let sanitized = sanitize_string(original_string);
    println!("Sanitized string: {}", sanitized);
}
*/

async fn sanitize_string(input: &str) -> String {
    input.chars()
        .filter(|&c| c.is_ascii_alphanumeric() || "_./-@,#".contains(c))
        .collect()
}

async fn remove_leading_trailing_characters(input: &str) -> String {
    input.trim_start_matches(",").trim_end_matches(",").to_string()
}

async fn replace_multiple_characters(input: &str) -> String {
    let re = Regex::new(r",+").unwrap();
    re.replace_all(input, ",").to_string()
}

async fn split_string(input: &str) -> Vec<String> {
    input.split(",").map(|s| s.to_string()).collect()
}

/*
What get_items(...) Does >>

let original_string = "###abc#555#xyz-123###";
let my_items = get_items(original_string);

let original_string = "abcd";
let my_items = get_items(original_string);

let original_string = "###abc##555####xyz-123#3-2-1#a-b-c###";
let my_items = get_items(original_string);

Output:

updated_str : abc#555#xyz-123
validated_updated_str : abc#555#xyz-123
my_list : ["abc", "555", "xyz-123"]


updated_str : abcd
validated_updated_str : abcd
my_list : ["abcd"]


updated_str : abc##555####xyz-123#3-2-1#a-b-c
validated_updated_str : abc#555#xyz-123#3-2-1#a-b-c
my_list : ["abc", "555", "xyz-123", "3-2-1", "a-b-c"]
*/
async fn get_items(my_str: &str) -> Vec<String> {
    let sanitized_str = sanitize_string(my_str).await;

    let updated_str = remove_leading_trailing_characters(sanitized_str.as_str()).await;
    println!("updated_str : {}", updated_str);

    let validated_updated_str = replace_multiple_characters(updated_str.as_str()).await;
    println!("validated_updated_str : {}", validated_updated_str);

    let my_list = split_string(validated_updated_str.as_str()).await;
    println!("my_list : {:?}", my_list);

    println!("\n");

    my_list
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // let environment = Option::from(env::var("ENVIRONMENT").unwrap()).unwrap();

    // let home_dir = dirs::home_dir().unwrap().display().to_string();

    let app_env_file = "app.rust.env".to_string();

    let file_path = format!("/etc/secrets/{}", app_env_file);

    dotenv::from_filename(file_path).ok();

    println!("☝️Service Starting");

    let pool = make_db_pool().await;

    std::env::set_var("RUST_LOG", "actix_web=debug");
    emoji_logger::init();

    let result = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(pool.clone()))
            .service(hello_world)
            .route("data", web::post().to(receive_data))
            .route("fetch_2", web::get().to(get_data_2))
            .route("fetch_with_query_2", web::get().to(get_data_with_query_2))
            .route("fetch_with_query_advanced_2", web::get().to(get_data_with_advanced_query_2))
            .route("fetch_1", web::get().to(get_data_1))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await;

    println!("🚨 Sample Service Stopping");

    result
}
```

#### Customizations & Parallel PostgreSQL Queries

> In my case function `get_data_with_advanced_single_query(...)` returns an JSON Array , but in `String` format

So I'm using `stream::` to parallize the queries across different tables and fetch them

```rust
#[derive(Display, From, Debug)]
enum CustomError {
    DatabaseError,
    InvalidData,
    QueryError,
    InvalidTable,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DataX {
    data_x_column_name_1: Option<String>,
    data_x_column_name_2: Option<String>,
    data_x_column_name_3: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DataY {
    data_y_column_name_1: Option<String>,
    data_y_column_name_2: Option<String>,
    data_y_column_name_3: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DataZ {
    data_z_column_name_1: Option<String>,
    data_z_column_name_2: Option<String>,
    data_z_column_name_3: Option<String>,
}


#[derive(Deserialize)]
struct QueryParamsAdvanced {
    //----------------------------------------------------
    // examples of value for (string_match)
    // 'like' or 'exact'
    string_match: String,
    //----------------------------------------------------
    // examples of value for (search_type)
    // this can be 'or' (OR) 'and'
    // 'or' means , search for 'abc' | 'xyz' etc
    // 'and' means , search for 'abc' + 'xyz'
    search_type: String,
    //----------------------------------------------------
    // examples of value for (search_string) > basically any search string
    // 'xyz'
    // '123-xyz'
    // 'xyz,555,abc-123' (',' delimited values : for multiple values)
    // if you gave (search_type=and), then search the table for all columns where : 'xyz' AND '555' AND 'abc-123' is present
    // if you gave (search_type=or), then search the table for all columns where : 'xyz' OR '555' OR 'abc-123' is present
    search_string: String,
    //----------------------------------------------------
    source_table: String,
}

// used by >> 'my_table_for_data_1', 'my_table_for_data_2'
async fn make_db_pool() -> Pool {
    dotenv().ok();
    dotenv::from_filename("app.rust.env").ok();

    let mut cfg = Config::new();
    cfg.host = Option::from(env::var("PG.HOST").unwrap());
    cfg.user = Option::from(env::var("PG.USER").unwrap());
    cfg.password = Option::from(env::var("PG.PASSWORD").unwrap());
    cfg.dbname = Option::from(env::var("PG.DBNAME").unwrap());
    let pool: Pool = cfg.create_pool(None, tokio_postgres::NoTls).unwrap();
    pool
}

// used by >> 'my_table_for_data_3'
async fn make_db_pool_v2() -> Pool {
    dotenv().ok();
    dotenv::from_filename("app.rust.env").ok();

    let mut cfg = Config::new();
    cfg.host = Option::from(env::var("PG.OS.HOST").unwrap());
    cfg.user = Option::from(env::var("PG.OS.USER").unwrap());
    cfg.password = Option::from(env::var("PG.OS.PASSWORD").unwrap());
    cfg.dbname = Option::from(env::var("PG.OS.DBNAME").unwrap());
    let pool: Pool = cfg.create_pool(None, tokio_postgres::NoTls).unwrap();
    pool
}


async fn get_db_pool_for_table(source_table: &str) -> Result<Pool,CustomError> {
    println!("source_table : {}", source_table.to_string());
    return match source_table {
        "my_table_for_data_1" => {
            Ok(make_db_pool().await)
        },
        "my_table_for_data_2" => {
            Ok(make_db_pool().await)
        },
        "my_table_for_data_3" => {
            Ok(make_db_pool_v2().await)
        },
        _ => {
            return Err(CustomError::InvalidTable)
        }
    };
}

> Important : the below function returns Result<String, CustomError> , Where (String) is an Array of JSON Objects

async fn get_data_with_advanced_single_query(query: QueryParamsAdvanced) -> Result<String, CustomError> {
    if !query.validate() {
        return return Err(CustomError::QueryError);
    }

    let mut actual_db_table = "".to_string();

    let query_table = query.source_table.to_owned();

    actual_db_table = match query_table.as_str() {
        "my_table_for_data_1" => "actual_postgres_table_1".to_string(),
        "my_table_for_data_2" => "actual_postgres_table_2".to_string(),
        "my_table_for_data_3" => "actual_postgres_table_3".to_string(),
        _ => return Err(CustomError::QueryError)
    };

    let my_db_pool = get_db_pool_for_table(query.source_table.as_str()).await.unwrap();

    let client: Client = my_db_pool.get().await.unwrap();

    println!("query >> \n\nstring_match => [ {} ] , \n\nsearch_string => [ {} ]\n\n", query.string_match, query.search_string);

    if query.source_table.is_empty() {
        return return Err(CustomError::QueryError);
    }


    if !(query.string_match.to_string() == "exact" || query.string_match.to_string() == "like") {
        return return Err(CustomError::QueryError);
    }

    if !(query.search_type.to_string() == "and" || query.search_type.to_string() == "or") {
        return return Err(CustomError::QueryError);
    }

    let search_strings = get_items(query.search_string.as_str()).await;

    let length_of_search_strings = search_strings.len() as i32;

    if length_of_search_strings < 1 {
        return return Err(CustomError::QueryError);
    }

    let mut table_columns = vec![];

    match query_table.as_str() {
        "my_table_for_data_1" => {
            table_columns.push("data_x_column_name_1");
            table_columns.push("data_x_column_name_2");
            table_columns.push("data_x_column_name_3");
        },
        "my_table_for_data_2" => {
            table_columns.push("data_y_column_name_1");
            table_columns.push("data_y_column_name_2");
            table_columns.push("data_y_column_name_3");
        },
        "my_table_for_data_3" => {
            table_columns.push("data_z_column_name_1");
            table_columns.push("data_z_column_name_2");
            table_columns.push("data_z_column_name_3");
        },
        _ => return Err(CustomError::QueryError)
    };

    println!("table_columns : {:#?}", table_columns);

    let mut inner_query = "".to_string();

    let total_combinations = table_columns.len() as i32 * search_strings.len() as i32;

    println!("total_combinations : {}", total_combinations);

    println!("string_match : {}", query.string_match.to_string());
    println!("search_type : {}", query.search_type.to_string());

    let length_of_columns = table_columns.len() as i32;

    if length_of_columns < 1 {
        return Err(CustomError::QueryError);
    }

    if query.string_match.as_str() == "exact" { // exact string match
        // search all JSON fields for possible match
        // this can also be applied if there are other columns
        if query.search_type.to_string() == "and" {
            inner_query = inner_query + " ( ";
            let mut search_string_counter = 1;
            for my_search_str in search_strings.to_owned() {
                inner_query = inner_query + " ( ";
                let mut column_counter = 1;
                // --------------------------------------
                for my_column in table_columns.to_owned() {
                    if column_counter == table_columns.len() as i32 {
                        inner_query = inner_query + format!(" lower({}::text) = lower('{}') ", my_column.to_string(), my_search_str.to_lowercase()).as_str();
                    } else {
                        inner_query = inner_query + format!(" lower({}::text) = lower('{}') OR ", my_column.to_string(), my_search_str.to_lowercase()).as_str();
                    }
                    column_counter += 1;
                }
                if search_string_counter == search_strings.len() as i32 {
                    inner_query = inner_query + " ) ";
                } else {
                    inner_query = inner_query + " ) AND ";
                }
                search_string_counter += 1;
                // --------------------------------------
            }
            inner_query = inner_query + " ) ";
        } else if query.search_type.to_string() == "or" {
            inner_query = inner_query + " ( ";
            let mut search_string_counter = 1;
            for my_search_str in search_strings.to_owned() {
                inner_query = inner_query + " ( ";
                let mut column_counter = 1;
                // --------------------------------------
                for my_column in table_columns.to_owned() {
                    if column_counter == table_columns.len() as i32 {
                        inner_query = inner_query + format!(" lower({}::text) = lower('{}') ", my_column.to_string(), my_search_str.to_lowercase()).as_str();
                    } else {
                        inner_query = inner_query + format!(" lower({}::text) = lower('{}') OR ", my_column.to_string(), my_search_str.to_lowercase()).as_str();
                    }
                    column_counter += 1;
                }
                if search_string_counter == search_strings.len() as i32 {
                    inner_query = inner_query + " ) ";
                } else {
                    inner_query = inner_query + " ) OR ";
                }
                search_string_counter += 1;
                // --------------------------------------
            }
            inner_query = inner_query + " ) ";
        } else {
            return Err(CustomError::QueryError)
        }

    } else if query.string_match.as_str() == "like" { // pattern match
        // search all JSON fields for possible match
        // this can also be applied if there are other columns
        if query.search_type.to_string() == "and" {
            inner_query = inner_query + " ( ";
            let mut search_string_counter = 1;
            for my_search_str in search_strings.to_owned() {
                inner_query = inner_query + " ( ";
                let mut column_counter = 1;
                // --------------------------------------
                for my_column in table_columns.to_owned() {
                    if column_counter == table_columns.len() as i32 {
                        inner_query = inner_query + format!(" lower({}::text) like lower('%{}%') ", my_column.to_string(), my_search_str.to_lowercase()).as_str();
                    } else {
                        inner_query = inner_query + format!(" lower({}::text) like lower('%{}%') OR ", my_column.to_string(), my_search_str.to_lowercase()).as_str();
                    }
                    column_counter += 1;
                }
                if search_string_counter == search_strings.len() as i32 {
                    inner_query = inner_query + " ) ";
                } else {
                    inner_query = inner_query + " ) AND ";
                }
                search_string_counter += 1;
                // --------------------------------------
            }
            inner_query = inner_query + " ) ";
        } else if query.search_type.to_string() == "or" {
            inner_query = inner_query + " ( ";
            let mut search_string_counter = 1;
            for my_search_str in search_strings.to_owned() {
                inner_query = inner_query + " ( ";
                let mut column_counter = 1;
                // --------------------------------------
                for my_column in table_columns.to_owned() {
                    if column_counter == table_columns.len() as i32 {
                        inner_query = inner_query + format!(" lower({}::text) like lower('%{}%') ", my_column.to_string(), my_search_str.to_lowercase()).as_str();
                    } else {
                        inner_query = inner_query + format!(" lower({}::text) like lower('%{}%') OR ", my_column.to_string(), my_search_str.to_lowercase()).as_str();
                    }
                    column_counter += 1;
                }
                if search_string_counter == search_strings.len() as i32 {
                    inner_query = inner_query + " ) ";
                } else {
                    inner_query = inner_query + " ) OR ";
                }
                search_string_counter += 1;
                // --------------------------------------
            }
            inner_query = inner_query + " ) ";
        } else {
            return Err(CustomError::QueryError)
        }
    } else {
        return Err(CustomError::QueryError)
    }

    println!("inner_query >> \n\n{}\n\n",inner_query);

    let complete_query = format!("SELECT * from {} WHERE {}", actual_db_table, inner_query);

    println!("complete_query >> \n\n{}\n\n",complete_query);

    let rows = client.query(complete_query.as_str(), &[]).await.map_err(|_| CustomError::DatabaseError)?;

    let mut structs_1:Vec<DataX> = Vec::new();
    let mut structs_2:Vec<DataY> = Vec::new();
    let mut structs_3:Vec<DataZ> = Vec::new();

    match query_table.as_str() {
        "t_1" => {
            for row in rows {
                let random_num = row.get("random_num");
                let random_float = row.get("random_float");
                let md5 = row.get("md5");

                let my_struct = Data1 {
                    random_num,
                    random_float,
                    md5,
                };
                structs_1.push(my_struct)
            }
        },
        "my_table_for_data_1" => {
            for row in rows {
                let data_x_column_name_1 = row.get("data_x_column_name_1");
                let data_x_column_name_2 = row.get("data_x_column_name_2");
                let data_x_column_name_2 = row.get("data_x_column_name_2");
            

                let my_struct = DataX {
                    data_x_column_name_1,
                    data_x_column_name_2,
                    data_x_column_name_2,
                };
                structs_2.push(my_struct)
            }
        },
        "my_table_for_data_2" => {
            for row in rows {
                let data_y_column_name_1 = row.get("data_y_column_name_1");
                let data_y_column_name_2 = row.get("data_y_column_name_2");
                let data_y_column_name_3 = row.get("data_y_column_name_3");
            
                let my_struct = DataY {
                    data_y_column_name_1,
                    data_y_column_name_2,
                    data_y_column_name_3,
                };
                structs_2.push(my_struct);
            }
        },
        "my_table_for_data_3" => {
            for row in rows {
                let data_z_column_name_1 = row.get("data_z_column_name_1");
                let data_z_column_name_2 = row.get("data_z_column_name_2");
                let data_z_column_name_3 = row.get("data_z_column_name_3");

                let my_struct = DataZ {
                    data_z_column_name_1,
                    data_z_column_name_2,
                    data_z_column_name_3,
                };
                structs_3.push(my_struct);
            }
        },
        _ => {
            return Err(CustomError::InvalidTable);
        }
    };


    return match query_table.as_str() {
        "my_table_for_data_1" => {
            Ok(serde_json::to_string(&structs_1).unwrap())
        },
        "my_table_for_data_2" => {
            Ok(serde_json::to_string(&structs_2).unwrap())
        },
        "my_table_for_data_3" => {
            Ok(serde_json::to_string(&structs_3).unwrap())
        },
        _ => {
            return Err(CustomError::InvalidTable);
        }
    };
}


async fn get_data_with_advanced_query_4(query: web::Query<QueryParamsAdvanced>) -> Result<HttpResponse, CustomError>  {

    let my_query1 = QueryParamsAdvanced{
        string_match: query.string_match.to_string(),
        search_type: query.search_type.to_string(),
        search_string: query.search_string.to_string(),
        source_table: "my_table_for_data_1".to_string(),
    };

    let my_query2 = QueryParamsAdvanced{
        string_match: query.string_match.to_string(),
        search_type: query.search_type.to_string(),
        search_string: query.search_string.to_string(),
        source_table: "my_table_for_data_2".to_string(),
    };

    let my_query3 = QueryParamsAdvanced{
        string_match: query.string_match.to_string(),
        search_type: query.search_type.to_string(),
        search_string: query.search_string.to_string(),
        source_table: "my_table_for_data_3".to_string(),
    };

    let mut jobs = vec![];
    jobs.push(my_query1);
    jobs.push(my_query2);
    jobs.push(my_query3);

    let concurrency = 10;

    let results: Vec<Result<String, CustomError>> = stream::iter(jobs).map(get_data_with_advanced_single_query).buffer_unordered(concurrency).collect().await;

    let mut my_results = vec![];

    for item in results {
        let my_result = match item {
            Ok(d) => {
                let json_items: Vec<Value> = serde_json::from_str(d.as_str()).unwrap();
                for item in json_items {
                    my_results.push(item);
                }
            },
            Err(e) => {
                println!("ERROR : {:#?}", e);
            },
        };
    }

    let json_body = json!(my_results);

    Ok(HttpResponse::Ok().content_type("application/json").body(json_body.to_string()))
}
```

> And `main` function looks like this

```rust
#[get("/api/health")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("{\"status\": \"ok\"}")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // let environment = Option::from(env::var("ENVIRONMENT").unwrap()).unwrap();

    // let home_dir = dirs::home_dir().unwrap().display().to_string();

    let app_env_file = "app.rust.env".to_string();

    let file_path = format!("/etc/secrets/{}", app_env_file);

    dotenv::from_filename(file_path).ok();

    println!("☝️Service Starting");

    let pool = make_db_pool().await;

    std::env::set_var("RUST_LOG", "actix_web=debug");
    emoji_logger::init();

    let result = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(pool.clone()))
            .service(hello_world)
            .route("/api/v1/fetch_with_query_advanced", web::get().to(get_data_with_advanced_query_4))
            .route("fetch_1", web::get().to(get_data_1))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await;

    println!("🚨 Sample Service Stopping");

    result
}
```

`Dockerfile`

```dockerfile
FROM DOCKER-HOST.COM/rust:1.70-alpine

### custom repo-setup

RUN echo "http://MY-REPO-SERVER/alpine/3.18/community" > /etc/apk/repositories
RUN echo "http://MY-REPO-SERVER/alpine/3.18/main" >> /etc/apk/repositories

RUN apk update && apk upgrade && apk add curl
RUN apk add pkgconfig
RUN apk add --no-cache musl-dev openssl-dev build-base
RUN mkdir -p /etc/secrets

ENV http_proxy=http://SOME-PROXY.SERVER.COM:5050
ENV https_proxy=http://SOME-PROXY.SERVER.COM:5050

RUN addgroup -S -g 10000 app && adduser -S -D -u 10000 -s /sbin/nologin -h /app/ -G app app

COPY --chown=app:app . /app
RUN chown -R app:app /app
USER 10000

WORKDIR /app

RUN cargo new --bin fastapi

RUN rm -rfv ./src

RUN rm -rfv ./target

COPY ./Cargo.toml ./Cargo.toml

COPY ./src ./src

RUN cargo build --release

CMD ["./target/release/fastapi"]

# On Mac OSX

# https://github.com/messense/homebrew-macos-cross-toolchains
#   brew tap messense/macos-cross-toolchains

# Install aarch64-unknown-linux-gnu Toolchain:
#   brew install aarch64-unknown-linux-gnu

#   brew install x86_64-linux-gnu-binutils
#   brew install x86_64-unknown-linux-gnu

# Build for Mac OSX:
#   cargo build --release

# Export these for compiling for Linux:
#   export CC_x86_64_unknown_linux_gnu=x86_64-unknown-linux-gnu-gcc;export CXX_x86_64_unknown_linux_gnu=x86_64-unknown-linux-gnu-g++;export AR_x86_64_unknown_linux_gnu=x86_64-unknown-linux-gnu-ar;export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-unknown-linux-gnu-gcc;

# Build for Linux (x86-64 Architexture)
#   cargo build --release --target=x86_64-unknown-linux-gnu

# Build Image
#   docker build . -t rust-app:latest

# Stop Running Docker And Remove Docker (Not Image)
#   docker stop rust-docker-app; docker rm rust-docker-app;

# Stop Running Docker And Remove Docker + Image
#   docker stop rust-docker-app; docker rm rust-docker-app; docker rmi rust-app;


# --------------------------------------------------
# 
#  FYI : I'm syncing the code to Linux Machine & Building The Docker Image and Running It There...
#
# cd ~/git/rust/fastapi && rsync -avh --stats --progress Dockerfile Cargo.toml src my_configuration.yaml root@SOME-UBUNTU-HOST:~/git/rust/fastapi/
# cd ~/git/rust/fastapi && rsync -avh --stats --progress ~/vault/* root@SOME-UBUNTU-HOST:~/vault/
#
# docker build . -t rust-app:latest
# docker run --platform linux/amd64 --name=rusty -d -v ~/vault:/etc/secrets -p 8080:8080 rust-app:latest;
#
# docker stop rusty; docker rm rusty; docker rmi rust-app:latest;

# --------------------------------------------------
```

#### [Async Functions And Channels](#async-functions-and-channels)

`Cargo.toml`

```toml
[package]
name = "async-channels"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
tokio = { version = "1.29.1", features = ["full"] }
```

`src/main.rs`

```rust
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
async fn producer(tx: mpsc::Sender<i32>) {
    for i in 0..25 {
        sleep(Duration::from_millis(200)).await;
        let send_result = match tx.send(i).await {
            Ok(d) => {
                println!("tx.send successful");
            }
            Err(e) => {
                println!("tx.send failed : {:#?}", e);
            }
        };
    }
}

async fn consumer(mut rx: mpsc::Receiver<i32>) {
    while let Some(data) = rx.recv().await {
        println!("consumer : rx.recv : data : {:#}", data);
    }
}

#[tokio::main]
async fn main() {

    let (tx, rx) = mpsc::channel(1);

    let producer_handler = tokio::spawn(async move {
        producer(tx).await;
    });

    let consumer_handler = tokio::spawn(async move {
        consumer(rx).await;
    });

    let _ = tokio::try_join!(producer_handler, consumer_handler);
}
```

#### [Fetch Openstack Keystone Token With HTTP Request Retry](#fetch-openstack-keystone-token-with-http-request-retry)

FYI : All of these APIs are documented here
- https://docs.openstack.org/api-quick-start/

- This piece of code makes http request & fetches openstack keystone token
- Has http request-retry (with exponential backoff) logic in place (in case the http request fails)
- Has (async concurrency) for making http requests to multiple-endpoints
- Openstack Configuration is maintained in a YAML file
- In case of errors, has a GENERIC error type, which can be extended/modified

Environment Config File

`app.config.env`

```ini
MY_USER="<SOME_OPENSTACK_USER>"
MY_PASS="<SOME_OPENSTACK_PASS>"
```

`openstack_configuration.yaml`

```yaml
---
  - region_name: "my-region-001"
    openstack_user: "my_os_user"
    openstack_pass: "my_os_pass"
    http_endpoint: "my-region-001-api-endpoint.company.com"
    rabbitmq_user: "my_mq_user"
    rabbitmq_pass: "my_mq_pass"
    galera_user: "my_galera_user"
    galera_pass: "my_galera_pass"

  - region_name: "my-region-002"
    openstack_user: "my_os_user"
    openstack_pass: "my_os_pass"
    http_endpoint: "my-region-002-api-endpoint.company.com"
    rabbitmq_user: "my_mq_user"
    rabbitmq_pass: "my_mq_pass"
    galera_user: "my_galera_user"
    galera_pass: "my_galera_pass"

    ...
    ...
    <INSERT MORE REGIONS>
    ...
    ...
```

`src/main.rs`

```rust
use std::{env, time};
use reqwest;
use serde::Deserialize;
use serde_json::{json, to_string};
use tokio::time::Duration;
use tokio::time::{sleep};
use std::fs::File;
use std::io::prelude::*;
use serde_yaml;
use tokio;
use futures::stream;
use futures::{join, select, StreamExt};
use dotenv;

pub const MAX_RETRIES: u32 = 10;

#[derive(Debug)]
pub enum GenericError {
    InternalServerError,
    SerializationFailed,
    HttpRequestFailed,
    EndpointNotFound,
    TokenNotFound,
    TenantQueryFailed,
    HypervisorQueryFailed,
}

#[derive(Debug)]
pub struct CustomError {
    pub err_type: GenericError,
    pub err_msg: String
}


#[derive(Debug, Deserialize)]
pub struct OpenstackRegionConfig {
    pub region_name: String,
    pub openstack_user: String,
    pub openstack_pass: String,
    pub http_endpoint: String,
    pub rabbitmq_user: String,
    pub rabbitmq_pass: String,
}


pub async fn get_workspace_data() -> Vec<OpenstackRegionConfig> {
    /*
        openstack_configuration.yaml:

        ---
          - region_name: "region-001"
            openstack_user: "my_user_1"
            openstack_pass: "some_thing_xyz_1"
            http_endpoint: "region-001-endpoint.company.com"
            rabbitmq_user: "r_user_1"
            rabbitmq_pass: "some_thing_p_001"
            galera_user: "my_galera_user_1"
            galera_pass: "my_galera_pass_1"

          - region_name: "region-002"
            openstack_user: "my_user_2"
            openstack_pass: "some_thing_xyz_2"
            http_endpoint: "region-002-endpoint.company.com"
            rabbitmq_user: "r_user_2"
            rabbitmq_pass: "some_thing_p_002"
            galera_user: "my_galera_user_2"
            galera_pass: "my_galera_pass_2"
    */

    // Open the YAML file
    let mut file = File::open("openstack_configuration.yaml").unwrap();
    let mut contents = String::new();

    // Read the file content
    file.read_to_string(&mut contents).unwrap();

    // Deserialize YAML into a Vec<User>
    let os_items: Vec<OpenstackRegionConfig> = serde_yaml::from_str(&contents).unwrap();
    os_items
}

pub async fn get_http_endpoint_for_region(region: &str) -> Option<String> {
    let openstack_endpoints = get_workspace_data().await;
    for item in openstack_endpoints {
        if item.region_name.to_string() == region.to_string() {
            return Some(item.http_endpoint);
        }
    }
    None
}

pub async fn get_region_for_http_endpoint(http_endpoint: &str) -> Option<String> {
    let openstack_endpoints = get_workspace_data().await;
    for item in openstack_endpoints {
        if item.http_endpoint.to_string() == http_endpoint.to_string() {
            return Some(item.region_name);
        }
    }
    None
}

pub async fn get_token(http_endpoint: &str) -> Result<(String,String), CustomError> {
    let client = reqwest::Client::new();

    dotenv::from_filename("app.config.env").ok();

    let user = env::var("MY_USER").unwrap();
    let pass = env::var("MY_PASS").unwrap();

    let payload = json!({
        "auth": {
            "identity": {
                "methods": ["password"],
                "password": {
                    "user": {
                        "name": user,
                        "domain": {
                            "id": "default"
                        },
                        "password": pass
                    }
                }
            }
        }
    });

    let json_string = to_string(&payload).unwrap();

    let mut keystone_token = String::from("");

    let keystone_endpoint = format!("https://{}:5000/v3/auth/tokens", http_endpoint);

    let response = match client.post(keystone_endpoint)
        .header("content-type", "application/json")
        .header("accept", "application/json")
        .body(json_string)
        .send()
        .await {
        Ok(d) => {
            println!("http response : {:#?}", d);
            if let Some(header_value) = d.headers().get("X-Subject-Token") {
                keystone_token = header_value.to_str().unwrap().to_string();
            } else {
                let custom_err = CustomError{
                    err_msg: String::from("Token Header (X-Subject-Token) Not Found"),
                    err_type: GenericError::InternalServerError
                };
                return Err(custom_err)
            }
        },
        Err(e) => {
            let custom_err = CustomError{
                err_msg: String::from("Could Not Make HTTP Post Request To Fetch Keystone Token"),
                err_type: GenericError::HttpRequestFailed
            };
            return Err(custom_err)
        },
    };
    Ok((http_endpoint.to_string(),keystone_token))
}

pub(crate) async fn get_token_with_retry(http_endpoint: &str) -> Option<(String,String)> {
    let mut backoff: u64 = 1;
    let mut retries = 20;

    let mut token = String::from("");
    let mut region = String::from("");
    loop {
        match get_token(&http_endpoint).await {
            Ok(d) => {
                (region, token) = d;
                break;
            }
            Err(e) => {
                println!("_REQUEST_KEYSTONE_FAILED (KEYSTONE_TOKEN) : {:#?}", e);
                retries += 1;

                if retries > MAX_RETRIES {
                    println!("_MAX_RETRIES_REACHED (KEYSTONE_TOKEN) , EXITING...");
                    break;
                }

                println!("_RETRYING (KEYSTONE_TOKEN) IN {} SECOND(S)...", backoff);
                sleep(Duration::from_secs(backoff)).await;

                // Exponential backoff
                backoff *= 2;
            }
        }
    }
    if token == "" {
        None
    } else {
        Some((region.to_string(),token.to_string()))
    }
}

pub async fn get_openstack_endpoints() -> Vec<String> {
    let mut endpoints:Vec<String> = vec![];
    let openstack_config = get_workspace_data().await;
    for region_config in openstack_config {
        endpoints.push(region_config.http_endpoint);
    }
    println!("openstack_http_endpoints : {:#?}", endpoints);
    endpoints
}

pub async fn get_keystone_token_for_all_regions() -> Vec<Option<(String,String)>> {
    let now = time::Instant::now();
    let input_vec = get_openstack_endpoints().await;
    let input_data: Vec<&str> = input_vec.iter().map(|d| d.as_str()).collect();
    let input_vec_length = input_data.len();
    // at any given time, run these many async functions
    let concurrency = 50;
    let results: Vec<_> = stream::iter(input_data)
        .map(get_token_with_retry)
        .buffer_unordered(concurrency)
        .collect()
        .await;
    let elapsed = now.elapsed().as_secs_f64();
    println!(
        "get_keystone_token_for_all_regions() : total time taken : {}",
        elapsed
    );
    results
}


#[tokio::main]
async fn main() {
    let keystone_tokens = get_keystone_token_for_all_regions().await;

    for my_token in keystone_tokens {
        match my_token {
            None => {}
            Some(d) => {
                let (region, token) = d;
                println!("Region : {} , Keystone Token : {}", region, token);
            }
        };
    };

}
```

#### [Using Map On ASYNC Functions](#using-map-on-async-functions)

> ASYNC Functions

```rust
fn get_sanitized_string(text: &str) -> String {
    let re = Regex::new(r"[^a-zA-Z0-9-._@/,;:\s]+").unwrap();
    let result = re.replace_all(&text, "");
    result.to_string()
}

async fn sanitize_string(input: &str) -> String {
    input.chars()
        .filter(|&c| c.is_ascii_alphanumeric() || "_./-@,#:;".contains(c))
        .collect()
}

async fn remove_leading_trailing_characters(input: &str) -> String {
    input.trim_start_matches(",").trim_end_matches(",").to_string()
}

async fn replace_multiple_characters(input: &str) -> String {
    let re = Regex::new(r",+").unwrap();
    re.replace_all(input, ",").to_string()
}

async fn split_string(input: &str, split_char: &str) -> Vec<String> {
    input.split(split_char).map(|s| s.to_string()).collect()
}

async fn remove_leading_and_trailing_spaces(my_str: &str) -> String {
    my_str.trim_start().trim_end().to_string()
}
```

> Usage for `map`

```rust
let mut search_strings:Vec<String> = vec![];
let mut sanitized_search_strings: Vec<String> = vec![];
let mut search_string_for_reference = "".to_string();
let mut search_strings_with_leading_and_trailing_spaces_removed = vec![];

/* --------------------------------------------------------------------- */

let futures_1: Vec<_> = search_strings.iter().map(|s| async {
    remove_leading_and_trailing_spaces(s).await
}).collect();
search_strings_with_leading_and_trailing_spaces_removed = futures::future::join_all(futures_1).await;

println!("@ length of search_strings_with_leading_and_trailing_spaces_removed >> {}", search_strings_with_leading_and_trailing_spaces_removed.len());

/* --------------------------------------------------------------------- */

let futures_2: Vec<_> = search_strings_with_leading_and_trailing_spaces_removed.iter().map(|s| async {
    sanitize_string(s).await
}).collect();
sanitized_search_strings = futures::future::join_all(futures_2).await;

println!("@ length of sanitized_search_strings >> {}", sanitized_search_strings.len());

/* --------------------------------------------------------------------- */
```

#### [Struct And PostgreSQL Table Mapping](#struct-and-postgresql-table-mapping)

If the following columns in `PostgreSQL` are of type `column_name::text`,<br/>
then we can create a PostgreSQL `view`, <br/>
which will convert the `text` (in `materialized view`) to `integer` (in the `view`).

- cloud        (this will be text in the view)
- vcpus        (this will be integer in the view)
- vcpus_used   (this will be integer in the view)
- running_vms  (this will be integer in the view)

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    pub cloud: String,
    pub vcpus: i32,
    pub vcpus_used: i32,
    pub running_vms: i32,
}
```

Create PostgreSQL `view` from `materialized view`.

```sql
CREATE VIEW view_table1 AS
    SELECT
        cloud::text as cloud,
        CASE WHEN vcpus ~ '^[0-9]+$' THEN vcpus::integer ELSE 0 END as vcpus,
        CASE WHEN vcpus_used ~ '^[0-9]+$' THEN vcpus_used::integer ELSE 0 END as vcpus_used,
        CASE WHEN running_vms ~ '^[0-9]+$' THEN running_vms::integer ELSE 0 END as running_vms
    FROM public.table1_materialized_view;
```

> Original `materialized view`

```sql
 \d public.table1_materialized_view
          Materialized view "public.table1_materialized_view"
        Column         | Type | Collation | Nullable | Default
-----------------------+------+-----------+----------+---------
 cloud                 | text |           |          |
 vcpus                 | text |           |          |
 vcpus_used            | text |           |          |
 running_vms           | text |           |          |
```

> Newly Created `view` from `materialized view`

```sql
\d view_table1
                 View "public.view_table1"
        Column         |  Type   | Collation | Nullable | Default
-----------------------+---------+-----------+----------+---------
 cloud                 | text    |           |          |
 vcpus                 | integer |           |          |
 vcpus_used            | integer |           |          |
 running_vms           | integer |           |          |
```

Note-1 : `integer` type in `PostgreSQL` will map to `i32` in Rust's `struct {...}`.

Note-2 : In the view `view_table1` above, if the column cannot be converted to `integer`, it will default to integer value of `0`.

#### [Using ARC To Clone Objects](#using-arc-to-clone-objects)

In a scenario where `client.clone()` fails, becuase `client` does not implement `.clone()` method,
then we can do something like this >>

The error you’re encountering likely stems from trying to clone the client object directly, 
which is not possible with the tokio-postgres::Client type because it does not implement 
the Clone trait. This trait is not implemented because each Client instance 
maintains its own connection pool and state.

A common pattern to handle this in Rust when working with tokio-postgres and 
needing to perform concurrent database operations is to use an 
Arc (Atomic Reference Counting) to share the client among multiple tasks. 
Arc safely allows multiple owners of the same data by ensuring memory safety with atomic operations.

Here’s how you can adjust your program to use Arc for sharing the Client across tasks:

Adjusted Program for Concurrent Operations

First, make sure to include Arc in your use declarations:

```rust
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=your_password dbname=testdb", NoTls).await?;

    let client = Arc::new(client); // Wrap the client with Arc

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Now, when you need to use the client in an async task, clone the Arc
    let write_handles: Vec<_> = (0..10).map(|_| {
        let client = Arc::clone(&client); // Clone the Arc, not the client itself
        tokio::spawn(async move {
            // Your database operation using client
        })
    }).collect();

    // Your code for write and read operations follows, making sure to clone the Arc each time you pass the client to a new task
}
```

Complete Code Sample To Measure PostgreSQL Read/Write Performance Below (Uses ARC to clone `client`) >>

#### [Measure PostgreSQL Read Write Performance](#measure-postgresql-read-write-performance)

- Payload Size 10KB
- Parallel Writes
- Parallel Reads (once writes are done)
- 1000 Threads
- ARC to clone `client` object

Create Database And Table First

```bash
psql -P expanded=auto -h PG_IP_ADDRESS -U perfuser perfdb
```

```sql
CREATE DATABASE perfdb;

-- Switch to the created database
\c perfdb;

-- Create a table
CREATE TABLE test_data ( id SERIAL PRIMARY KEY, data TEXT NOT NULL );
```

`Cargo.toml`

```toml
[package]
name = "pg-performance"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
chrono = "0.4.33"
rand = "0.8.5"
tokio = { version = "1.35.1" , features = ["full"]}
tokio-postgres = "0.7.10"
```

`src/main.rs`

```rust
use chrono::prelude::*;
use tokio_postgres::{NoTls, Error};
use rand::{distributions::Alphanumeric, Rng};
use tokio::time::{sleep, Duration};
use tokio::time::interval;
use std::sync::Arc;


#[tokio::main]
async fn main() -> Result<(), Error> {
    let conn_str = "host=PG_IP_ADDRESS user=perfuser password=PG_PASS dbname=perfdb";
    let (client, connection) = tokio_postgres::connect(conn_str, NoTls).await.unwrap();
    let client = Arc::new(client);

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    println!("database connection initialized...");

    let total_operations = 1000; // Number of concurrent operations

    println!("starting write operations...");

    let payload_size = 10240;

    // Concurrent Writes
    let write_start = Utc::now();
    let mut write_handles = Vec::new();
    for iteration in 0..total_operations {
        println!("write >> iteration : {}", iteration);
        let client = Arc::clone(&client);
        let data: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(payload_size) // 5KB of data
            .map(char::from)
            .collect();

        let handle = tokio::spawn(async move {
            let _ = client.execute("INSERT INTO test_data (data) VALUES ($1)", &[&data]).await;
        });
        write_handles.push(handle);
    }

    // Await all write operations to complete
    for handle in write_handles {
        let _ = handle.await;
    }
    let write_end = Utc::now();
    let time_total_writes = (write_end - write_start).num_milliseconds();

    println!("sleeping for sometime...");

    // Delay to ensure writes are committed
    sleep(Duration::from_secs(5)).await;

    println!("starting read operations...");

    // Concurrent Reads
    let read_start = Utc::now();
    let mut read_handles = Vec::new();
    for iteration in 0..total_operations {
        println!("read >> iteration : {}", iteration);
        let client = Arc::clone(&client);
        let handle = tokio::spawn(async move {
            let _ = client.query("SELECT data FROM test_data ORDER BY id DESC LIMIT 1", &[]).await;
        });
        read_handles.push(handle);
    }

    println!("both writes and reads are done.");

    // Await all read operations to complete
    for handle in read_handles {
        let _ = handle.await;
    }
    let read_end = Utc::now();

    let time_total_reads = (read_end - read_start).num_milliseconds();

    println!("\n# Payload (Size) Of Each Record: {} bytes", payload_size);

    println!("\n# Total Parallel Threads (Write+Read): {}", total_operations);

    println!("\n# Concurrent write duration: {} ms", time_total_writes);

    println!("\n# Concurrent read duration: {} ms", time_total_reads);

    Ok(())
}
```

#### Connect To Azure PostgreSQL Over SSL

- Connect to PostgreSQL DB Over `SSL`
- Below code connects to Azure PostgreSQL (without SSL it will `not` connect)

```shell
psql -P expanded=auto -h my-azure-database-region-01.postgres.database.azure.com -U my-read-write-user@my-azure-database-region-01 postgres
```

`Cargo.toml`

```toml
[package]
name = "pg-performance"
version = "0.1.0"
edition = "2021"

[dependencies]
async-trait = "0.1.77"
chrono = "0.4.33"
deadpool-postgres = "0.12.1"
dotenv = "0.15.0"
native-tls = "0.2.11"
openssl = "0.10.63"
postgres-openssl = "0.5.0"
rand = "0.8.5"
derive_more = "0.99.17"
tokio = { version = "1.35.1" , features = ["full"]}
tokio-native-tls = "0.3.1"
tokio-postgres = "0.7.10"
tokio-postgres-native-tls = "0.1.0-rc.1"
tokio-postgres-openssl = "0.1.0-rc.1"
serde = { version = "1.0.196", features = ["derive"] }
tokio-postgres-rustls = "0.11.1"
rustls = "0.22.2"
config = "0.13.4"
anyhow = "1.0.79"
```

`app.config.env`

```ini
USERNAME="my-read-write-user@my-azure-database-region-01"
PASSWORD="some-random-text"
HOSTNAME="my-azure-database-region-01.postgres.database.azure.com"
DBNAME="postgres"
PORT="5432"
```

`src/main.rs`

```rust
use chrono::prelude::*;
use tokio_postgres::{NoTls, Error};
use rand::{distributions::Alphanumeric, Rng};
use tokio::time::{sleep, Duration};
use tokio::time::interval;
use std::sync::Arc;
use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::MakeTlsConnector;


#[tokio::main]
async fn main() -> Result<(), Error> {

    dotenv::from_filename("app.config.env").ok().unwrap();

    let pg_host = std::env::var("HOSTNAME").unwrap();
    let password = std::env::var("PASSWORD").unwrap();
    let username = std::env::var("USERNAME").unwrap();
    let db_name = std::env::var("DBNAME").unwrap();
    let port = std::env::var("PORT").unwrap();


    let conn_str = format!("host={} user={} password={} dbname={} sslmode=require",
        pg_host,
        username,
        password,
        db_name
    );

    let builder = SslConnector::builder(SslMethod::tls()).unwrap();
    let tls_connector = MakeTlsConnector::new(builder.build());

    let (client, connection) = tokio_postgres::connect(conn_str.as_str(), tls_connector).await.unwrap();
    let client = Arc::new(client);

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    println!("database connection initialized...");

    let total_operations = 1000; // Number of concurrent operations

    println!("starting write operations...");

    let payload_size = 10240;

    // Concurrent Writes
    let write_start = Utc::now();
    let mut write_handles = Vec::new();
    for iteration in 0..total_operations {
        println!("write >> iteration : {}", iteration);
        let client = Arc::clone(&client);
        let data: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(payload_size) // 5KB of data
            .map(char::from)
            .collect();

        let handle = tokio::spawn(async move {
            let _ = client.execute("INSERT INTO test_data (data) VALUES ($1)", &[&data]).await;
        });
        write_handles.push(handle);
    }

    // Await all write operations to complete
    for handle in write_handles {
        let _ = handle.await;
    }
    let write_end = Utc::now();
    let time_total_writes = (write_end - write_start).num_milliseconds();

    println!("sleeping for sometime...");

    // Delay to ensure writes are committed
    sleep(Duration::from_secs(5)).await;

    println!("starting read operations...");

    // Concurrent Reads
    let read_start = Utc::now();
    let mut read_handles = Vec::new();
    for iteration in 0..total_operations {
        println!("read >> iteration : {}", iteration);
        let client = Arc::clone(&client);
        let handle = tokio::spawn(async move {
            let _ = client.query("SELECT data FROM test_data ORDER BY id DESC LIMIT 1", &[]).await;
        });
        read_handles.push(handle);
    }

    println!("both writes and reads are done.");

    // Await all read operations to complete
    for handle in read_handles {
        let _ = handle.await;
    }
    let read_end = Utc::now();

    let time_total_reads = (read_end - read_start).num_milliseconds();

    println!("\n# Payload (Size) Of Each Record: {} bytes", payload_size);

    println!("\n# Total Parallel Threads (Write+Read): {}", total_operations);

    println!("\n# Concurrent write duration: {} ms", time_total_writes);

    println!("\n# Concurrent read duration: {} ms", time_total_reads);

    Ok(())
}
```

#### [Factory Design Pattern V1](#factory-design-pattern-v1)

Perform CRUD Operation on MongoDB & PostgreSQL

Notes

```
The factory design pattern in Rust can be used to abstract the 
creation of objects that perform CRUD operations on different 
databases like MongoDB or PostgreSQL. 

The idea is to define a common trait for CRUD operations and 
then implement this trait for each specific database. 
The factory function or method then returns a concrete 
implementation based on the requested database type.
```

- The actual implementation of CRUD operations for MongoDB and PostgreSQL will depend on their respective client libraries and your application's specific requirements.
- This example uses trait objects (`Box<dyn CrudOperations>`) to achieve polymorphism.
- Trait objects are a way to achieve runtime polymorphism in Rust.
- Ensure you have the necessary dependencies for MongoDB and PostgreSQL clients in your Cargo.toml and set up the actual database connections and operations according to your application needs.
- The factory pattern is useful when the creation logic is complex or when you want to abstract away the instantiation details from the client code.
- In this example, it helps to switch between different database backends easily.

```rust
trait CrudOperations {
    fn create(&self, data: &str);
    fn read(&self, id: u32) -> String;
    fn update(&self, id: u32, data: &str);
    fn delete(&self, id: u32);
}
```

```rust
struct MongoDB;

impl CrudOperations for MongoDB {
    fn create(&self, data: &str) {
        println!("MongoDB create: {}", data);
        // Implement MongoDB create operation
    }
    fn read(&self, id: u32) -> String {
        println!("MongoDB read: {}", id);
        // Implement MongoDB read operation
        "MongoDB data".to_string()
    }
    fn update(&self, id: u32, data: &str) {
        println!("MongoDB update: {}", id);
        // Implement MongoDB update operation
    }
    fn delete(&self, id: u32) {
        println!("MongoDB delete: {}", id);
        // Implement MongoDB delete operation
    }
}
```

```rust
struct PostgreSQL;

impl CrudOperations for PostgreSQL {
    fn create(&self, data: &str) {
        println!("PostgreSQL create: {}", data);
        // Implement PostgreSQL create operation
    }
    fn read(&self, id: u32) -> String {
        println!("PostgreSQL read: {}", id);
        // Implement PostgreSQL read operation
        "PostgreSQL data".to_string()
    }
    fn update(&self, id: u32, data: &str) {
        println!("PostgreSQL update: {}", id);
        // Implement PostgreSQL update operation
    }
    fn delete(&self, id: u32) {
        println!("PostgreSQL delete: {}", id);
        // Implement PostgreSQL delete operation
    }
}
```

```rust
enum DbType {
    MongoDB,
    PostgreSQL,
}

fn db_factory(db_type: DbType) -> Box<dyn CrudOperations> {
    match db_type {
        DbType::MongoDB => Box::new(MongoDB),
        DbType::PostgreSQL => Box::new(PostgreSQL),
    }
}
```

```rust
fn main() {
    let db = db_factory(DbType::MongoDB);
    db.create("test data");
    let data = db.read(1);
    db.update(1, "updated data");
    db.delete(1);

    let db = db_factory(DbType::PostgreSQL);
    db.create("test data");
    let data = db.read(1);
    db.update(1, "updated data");
    db.delete(1);
}
```

#### [Write CSV File](#write-csv-file)

`Cargo.toml`

```toml
[dependencies]
csv = "1.1"
serde = { version = "1.0", features = ["derive"] }
```

`src/main.rs`

```rust
use serde::Serialize;

#[derive(Serialize)]
struct Record {
    field1: String,
    field2: u32,
    // Add more fields as needed
}

use csv::Writer;
use std::error::Error;
use std::fs::File;

fn write_csv() -> Result<(), Box<dyn Error>> {
    // Create a writer from a file
    let file = File::create("output.csv")?;
    let mut writer = Writer::from_writer(file);

    // Create some sample data
    let records = vec![
        Record {
            field1: "Value1".to_string(),
            field2: 123,
        },
        Record {
            field1: "Value2".to_string(),
            field2: 456,
        },
        // Add more records as needed
    ];

    // Write records to CSV
    for record in records {
        writer.serialize(record)?;
    }

    writer.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    write_csv()?;
    Ok(())
}
```