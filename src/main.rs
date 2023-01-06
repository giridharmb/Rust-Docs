mod dcode;

use std::fs::File; // for reading a file
use std::io::prelude::*; // for reading a file

// use std::io; // for reading user input

use std::collections::HashMap;

use std::{env, fmt};
use std::fmt::Formatter;
use std::future::Future; // for CLI

extern crate rand; // for random number , make sure you add (rand = "0.8.5") in Cargo.toml first
use rand::Rng;

// regex
extern crate regex; // make sure you add (regex = "1.7.0") in Cargo.toml first
use regex::{Regex};

extern crate reqwest;


use std::process::Command; // for executing external commands

use std::io::{stdin, self, Write, Read};
use std::num::ParseFloatError; // for reading user's input

extern crate serde_json; // for parsing json
use serde_json::Value as JsonValue; // for parsing json
use serde_json::Result as JsonResult;
use tokio::io::AsyncReadExt;

extern crate serde; // for parsing json

#[macro_use]
extern crate serde_derive;
extern crate core; // for parsing json


use std::io::ErrorKind;
use serde_json::Value::Object; // for handling errors below


#[derive(Serialize,Deserialize)] // derive attribute -> Serialize,Deserialize (traits)
struct Customer {
    customerid: String,
    age: u32,
    eyecolor: String,
    name: String,
    gender: String,
    company: String,
    email: String,
    phone: String,
    address: String,
}

#[allow(dead_code)] // this will suppress unused variable warnings
enum Direction {
    Up,
    Down,
    Left,
    Right
}

mod my_module {
    fn print_another_message() {
        println!("this is another message !");
    }
    pub fn print_message() {
        println!("this is an example of a function inside a module !");
        print_another_message();
    }

    pub mod my_test_module {
        pub fn print_test_message() {
            println!("this is a test message !");
        }
    }
}

// enum methods

#[allow(dead_code)]
enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday
}

// enum methods
impl Day {
    fn is_weekday(&self) -> bool {
        return match self {
            &Day::Saturday | &Day::Sunday => false,
            _ => true
        }
    }
}

fn main() {
    println!("Hello, world!");

    let my_strings = vec!["x", "y", "z"];

    let number_iterator = 1..11;

    for i in number_iterator {
        println!("value of i is {}", i);
    }

    for (index,value) in my_strings.iter().enumerate() {
        println!("index : {} , value : {}", index, value);
    }

    let player_direction:Direction = Direction::Up;

    match player_direction {
        Direction::Up => println!("we are heading up !"),
        Direction::Down => println!("we are heading down !"),
        Direction::Left => println!("we are heading left !"),
        Direction::Right => println!("we are heading right !"),
    }

    let tup1 = (20, 4.44, "Rust", false);
    let tup2 = (20, 4.44, "Rust", false, ("aa", "bb"));

    println!("{}", tup1.0);
    println!("{}", tup1.1);
    println!("{}", tup1.2);
    println!("{}", tup1.3);

    println!("{}", (tup2.4).1);

    let (v1, v2, v3, v4) = tup1;

    println!("{}", v1);
    println!("{}", v2);
    println!("{}", v3);
    println!("{}", v4);

    print_numbers(10, 21);

    // shadowing

    let mut x = 10;
    println!("outside scope : value of x is -> {}", x);
    {
        let x = 15;
        println!("inside scope : value of x is -> {}", x);
    }
    x = 11;
    println!("outside scope : value of x is -> {}", x);

    let mut x = "this is a string-01";
    println!("value of x is -> {}", x);

    x = "this is a string-02";
    println!("value of x is -> {}", x);

    // references
    let mut x = 55.55;
    // immutable reference
    let xr = &x;
    println!("xr is => {}", xr);

    // mutable reference (value can be changed through the reference
    let mut_ref = &mut x;
    println!("mut_ref is => {}", mut_ref);

    *mut_ref += 100 as f64;
    println!("mut_ref is => {}", mut_ref);

    // x is now changed
    println!("x is => {}", x);

    // structs

    let background_color = Color{
        red: 255,
        green: 70,
        blue: 15
    };

    println!("color is -> {} {} {}", background_color.red, background_color.green, background_color.blue);

    // tuple struct
    let red = TupleColor(255,0,0);
    println!("red is {}, {}, {}", red.0, red.1, red.2);

    print_color(&background_color);

    // arrays

    let nums = [0,1,2,3,4,5];
    println!("arrays : nums[1] => {}", nums[1]);

    for n in nums.iter() {
        println!("looping through array, value => {}", n)
    }

    for i in 0..nums.len() {
        println!("looping through array, value (i) => {}", nums[i])
    }

    // this is what happens internally, that is : nums2:[i32;6] , but not required
    let nums2:[i32;6] = [0,1,2,3,4,5];
    for n in nums2.iter() {
        println!("looping through array, value => {}", n)
    }

    // array of 20 items, default value of 0
    let nums3 = [0; 20];
    for n in nums3.iter() {
        println!("looping through array, value => {}", n)
    }

    let my_rectangle = Rectangle {width: 10, height: 10};
    println!("{:?}", my_rectangle);

    // impl
    my_rectangle.print_description();

    println!("Rectangle is square ? {}", my_rectangle.is_square());

    let mut my_string = String::from("the quick brown fox jumps over the lazy dog");

    println!("length of my_string : {}", my_string.len());
    println!("is my_string empty ? {}", my_string.is_empty());

    let split_words = my_string.split_whitespace();

    for word in split_words {
        println!("> {}", word);
    }

    println!("{}", my_string.contains("jumps"));
    println!("{}", my_string.contains("jump"));
    println!("{}", my_string.contains("jumps "));
    println!("{}", my_string.contains("jumps over_"));

    my_string.push_str(" XYZ");

    println!("my_string : {}", my_string);

    // implementing traits

    let person = Person{
        name: String::from("Giridhar"),
        age: 42
    };

    println!("{}", person.to_string());

    // one way of creating a vector
    let mut my_vector1: Vec<i32> = Vec::new();

    // another way of creating a vector
    let mut my_vector2 = vec![1, 2, 3, 4];

    my_vector1.push(10);
    my_vector1.push(100);
    my_vector1.push(1000);

    my_vector2.push(20);

    let mut index1 = 0;
    let mut index2 = 0;

    for i in my_vector1.iter() {
        println!("my_vector1 : index : {} , element => {}", index1, i);
        index1 = index1 + 1;
    }

    for i in my_vector2.iter() {
        println!("my_vector2 : index : {} , element => {}", index2, i);
        index2 = index2 + 1;
    }

    my_vector1.remove(0);
    my_vector2.remove(0);

    println!("after removing an element from the vector...");

    index1 = 0;
    index2 = 0;

    for i in my_vector1.iter() {
        println!("my_vector1 : index : {} , element => {}", index1, i);
        index1 = index1 + 1;
    }

    for i in my_vector2.iter() {
        println!("my_vector2 : index : {} , element => {}", index2, i);
        index2 = index2 + 1;
    }

    // reading a file

    let mut file = File::open("info.txt").expect("cannot open the file !");

    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("could not read the file");

    println!("\nfile contents >>\n");

    println!("{}", contents);

    // ------------ using reference ------------

    struct MyObject {
        width: u32,
        height: u32
    }

    fn calculate_area(obj: &MyObject) -> u32 {
        obj.width * obj.height
    }

    let my_object = MyObject {
        width:15,
        height:20
    };

    /*
        Important (use of Reference):

        If you calculate area this way >

        fn calculate_area(obj: MyObject) -> u32 {
            obj.width * obj.height
        }

        calculate_area(my_object)

        Then, you will see an error in the compiler this way:

        error[E0505]: cannot move out of `my_object` because it is borrowed
        move out of `my_object` occurs here

        Instead, you should caluclate the area this way:

        fn calculate_area(obj: &MyObject) -> u32 {
            obj.width * obj.height
        }

        calculate_area(&my_object)

    */

    println!("Area of my_object with dimensions {} x {} => {}", my_object.width, my_object.height, calculate_area(&my_object));

    // The above area calculation can be done in a better way >

    impl MyObject {
        fn calculate_area(&self) -> u32 {
            self.width * self.height
        }

        fn show(&self) {
            println!("Area of the specified object with dimensions {} x {} => {}", self.width, self.height, self.calculate_area());
        }
    }

    let my_object_v2 = MyObject {
        width:35,
        height:70
    };

    println!("Area of my_object_v2 with dimensions {} x {} => {}", my_object_v2.width, my_object_v2.height, my_object_v2.calculate_area());

    // creating new objects using 'MyObject::new_object(...)'

    impl MyObject {
        fn new_object(width: u32, height: u32) -> MyObject {
            MyObject {
                width,
                height
            }
        }
    }

    let my_object_v3 = MyObject::new_object(35,45);

    my_object_v3.show();

    /*
        FYI:
        In this print statement >
        println!("{:?}", my_data);
        {:?} -> is debug flag/trait
        {:#?} -> is pretty debug flag/trait

        To apply debug flag/trait, we need to add this:
        #[derive(Debug)] -> this is an annotation
    */

    // example of Debug Flag / Debug Trait

    #[derive(Debug)] // this is an annotation
    struct MyObjectTest {
        width: u32,
        height: u32
    }

    impl MyObjectTest {
        fn calculate_area(&self) -> u32 {
            self.width * self.height
        }

        fn new_object(width: u32, height: u32) -> MyObjectTest {
            MyObjectTest {
                width,
                height
            }
        }

        fn show(&self) {
            println!("Area of the specified object with dimensions {} x {} => {}", self.width, self.height, self.calculate_area());
        }
    }

    let my_object_test = MyObjectTest::new_object(99,67);
    my_object_test.show();

    // using debug flag/trait using {:?}
    // pretty debug flag/trait {:#?}
    println!("my_object_test : {:?}", my_object_test);
    println!("my_object_test : {:#?}", my_object_test);

    // ----------- applying Display trait ----------
    // import : use std::fmt;

    // the below impl , will enable us to do something like : println!("{}", my_object_test);

    impl fmt::Display for MyObjectTest {
        /*
            this peice of code is auto-completed by the IDE

            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                todo!()
            }

            We have to replace todo!() -> with out logic
        */

        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "(width is {} , height is {}) , Area : {}", self.width, self.height, self.calculate_area())
        }
    }

    println!("Here are the dimensions and area for my_object_test : {}", my_object_test);


    // ----------- CLI ------------

    println!("\nCLI >>\n");

    let args: Vec<String> = env::args().collect();

    for argument in args.iter() {
        println!("{}", argument);
    }

    // creating a file

    let mut file = File::create("out.txt").expect("could not create text file");

    file.write_all(b"welcome to rust programming, its awesome !").expect("cannot write to file, there was an error");

    // implement 'HasVoiceBox' trait, on 'Person' struct

    let person1 = Person {
        name: String::from("Giri1"),
        age: 1,
    };
    person1.speak();
    println!("person1 can speak ? {}", person1.can_speak());

    let person2 = Person {
        name: String::from("Giri2"),
        age: 3,
    };
    person2.speak();
    println!("person2 can speak ? {}", person2.can_speak());

    // switch / match
    let mut my_name = "linus";
    check(my_name);
    my_name = "giridhar";
    check(my_name);
    my_name = "abc";
    check(my_name);
    my_name = "clinton";
    check(my_name);

    // reading user input
    /*
    let mut input = String::new();
    println!("Waiting for user input: ");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Success: you said : {}", input.to_uppercase());
        }
        Err(e) => {
            println!("Error: something went wrong : {}", e);
        }
    }
    */

    // hash-maps

    let mut my_map = HashMap::new();

    my_map.insert("rust programming", 100);
    my_map.insert("golang programming", 90);
    my_map.insert("web development", 90);
    my_map.insert("grpc development", 95);

    println!("total elements in hashmap : {}", my_map.len());

    // get a single value from hashmap

    match my_map.get("web development"){
        None => {
            println!("nothing found for key !");
        }
        Some(data) => {
            println!("got this data from hash-map : {}", data);
        }
    }

    // remove a value from hash-map
    my_map.remove("grpc development");
    println!("total elements in hashmap : {}", my_map.len());

    // loop through hashmap
    for (key, value) in &my_map {
        println!("key : {}", key);
        println!("value : {}", value);
    }
    // check for key in map
    println!("my_map contains key 'xyz' : {}", my_map.contains_key("xyz"));
    println!("my_map contains key 'rust programming' : {}", my_map.contains_key("rust programming"));

    // random number
    let random_number = rand::thread_rng().gen_range(1..11);
    println!("random_number : {}", random_number);

    // rand : flipping a coin
    let rand_bool = rand::thread_rng().gen_bool(0.5);
    println!("rand_bool : {}", rand_bool);

    // ------------- [ string methods ] -------------

    println!("\n");

    // ------------- replace -------------
    {
        let my_str = String::from("rust is fantastic , must check it out please");
        println!("After Replace : {}\n", my_str.replace("fantastic", "great"));
    }

    // ------------- lines -------------
    {

        let my_str = String::from("rust\nis\nfantastic,\nmust\ncheck\nit\nout\nplease");
        println!("After Replace : \n\n{}\n", my_str.replace("fantastic", "great"));

        for line in my_str.lines() {
            println!("[ {} ]", line);
        }
    }

    println!("\n");

    // ------------- split -------------
    {
        let my_str = String::from("rust + is + fantastic, + must + check + it + out + please");
        let tokens: Vec<&str> = my_str.split("+").collect();

        for token in tokens.iter() {
            println!("'{}'", token);
        }
    }

    println!("\n");

    // ------------- split -------------
    {
        let my_str = String::from("rust###is###fantastic,###must###check###it###out###please");
        let tokens: Vec<&str> = my_str.split("###").collect();

        for token in tokens.iter() {
            println!("'{}'", token);
        }
    }

    println!("\n");

    // ------------- trim -------------

    {
        let my_str = String::from("   rust   is   fantastic,   must   check   it   out   please   \n\r");
        println!("Before Trim : \n\n#{}#\n", my_str);
        println!("After Trim : \n\n#{}#\n", my_str.trim());
    }

    // ------------- chars -------------
    {
        let my_str = String::from("rust is fantastic , must check it out please");
        println!("{}", my_str);
        // get char at index
        match my_str.chars().nth(5) { // character at index 5 (starting from 0)
            None => {
                println!("char not found");
            }
            Some(c) => {
                println!("char at index 5 : {}", c);
            }
        }

    }

    println!("\n");

    // external modules : 'dcode.rs'
    // make sure you include : 'mod dcode;' in main.rs , and
    // function is public : pub fn print_message() { ... } in decode.rs
    dcode::print_message();

    // regex

    {
        let re = Regex::new(r"\w{5}").unwrap();
        let text = "the quick brown fox jumps over the lazy dog 5837535 39h3yr3t8 v9d9";
        println!("Found Match ? {}", re.is_match(text));
    }

    println!("\n");

    {
        let re = Regex::new(r"[a-zA-Z]{5}").unwrap();
        let text = "the quick brown fox jumps over the lazy dog 5837535 39h3yr3t8 v9d9";
        for cap in re.find_iter(text) {
            println!("regex : found match : {:#?}", cap.as_str());
        }
    }

    println!("\n");

    {
        let re = Regex::new(r"[0-9]{5}").unwrap();
        let text = "64646 642436221 744 643466 66002 46642632 33556";
        for cap in re.find_iter(text) {
            println!("regex : found match : {:#?}", cap.as_str());
        }
    }

    println!("\n");

    // -------- module -------

    my_module::print_message();
    my_module::my_test_module::print_test_message();

    // option - enum

    let name = String::from("0123456789");

    println!("character at index 10 : {}", match name.chars().nth(9) {
        None => String::from("no character found at given index !").to_string(),
        Some(c) => c.to_string()
    });

    println!("1 > occupation is : {}", match get_occupation("giridhar") {
        None => "no occupation found.",
        Some(d) => d
    });

    println!("2 > occupation is : {}", match get_occupation("bhujanga") {
        None => "no occupation found.",
        Some(d) => d
    });

    println!("3 > occupation is : {}", match get_occupation("linus") {
        None => "no occupation found.",
        Some(d) => d
    });

    // --------- http request ----------

    make_http_request();

    // -------- enum methods -----------

    let d1 = Day::Tuesday;
    let d2 = Day::Saturday;

    println!("Tuesday is weekday ? : {}", d1.is_weekday());
    println!("Saturday is weekday ? : {}", d2.is_weekday());

    // -------- execute external commands ---------

    let mut cmd = Command::new("uptime");
    // cmd.arg("-h");

    match cmd.output() {
        Ok(o) => {
            unsafe {
                println!("{}", String::from_utf8_unchecked(o.stdout));
            }
        }
        Err(e) => {
            println!("there was an error execiting the command : {}", e);
        }
    }

    // -------------- parsing json ----------------

    let json_str = r#"
        {
            "customerid": "630c2272eabd3d30fe44d139",
            "age": 28,
            "eyecolor": "brown",
            "name": "Mabel Haley",
            "gender": "female",
            "company": "ENOMEN",
            "email": "mabelhaley@enomen.com",
            "phone": "+1 (880) 516-2365",
            "address": "184 Bergen Court, Gorham, American Samoa, 8722"
        }
    "#;

    let res = serde_json::from_str(json_str);

    // method-1 of getting json data

    if res.is_ok() { // if json_str is a valid json
        // JsonValue -> is a struct
        let deserialized_json: JsonValue = res.unwrap();
        println!("method-1 : customerid : {}", deserialized_json["customerid"].as_str().unwrap());
        println!("method-1 : age : {}", deserialized_json["age"].as_i64().unwrap());
    } else {
        println!("sorry, could not parse json string !");
    }

    // method-2 of getting json data (using structs)

    let res2:JsonResult<JsonValue> = serde_json::from_str(json_str);

    if res2.is_ok() {
        let c: Customer = serde_json::from_str(json_str).unwrap();
        println!("method-2 : customerid : {}", c.customerid);
        println!("method-2 : age : {}", c.age);
    }

    // --------- Result type -------------

    let mut my_input_string = String::new();
    println!("enter a number: ");

    io::stdout().flush().unwrap();

    // method-1 (not safe)
    // completely commented out

    /*
        unwrap() will extract data from Result
        FYI : unwrap() : This is not safe to do : User can input text instead of numbers and program can crash
     */

    // method_1_start
    /*

    {
        stdin().read_line(&mut my_input_string).expect("Did not enter valid input !");
        let my_number: f64 = my_input_string.trim().parse().unwrap();
        let my_number: f64 = my_input_string.trim().parse().expect("invalid input ! you probably did not enter a number !");
        println!("Yay ! you entered a number : {:?}", my_number);
    }

    */
    // method_1_end

    // method-2 (safe) -> use it this way !

    // what ever value loop returns, that will be stored in my_num
    // whatever value comes after 'break', that value will be returned by loop

    // method_2_start
    /*

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

    */

    // method_2_end

    let my_num = 51;
    /*
        func returns this : Result<u32, &'static str>
        If it reaches Ok(..) , it will return : u32
        If it reaches Err(..) , it will return : &'static str (borrowed static string)
    */

    /*
    Result -> enum : it represents success or failure

        enum Result<T, E>
        {
            Ok(T),
            Err(E),
        }
    */


    fn is_it_fifty(num: i32) -> Result<u32, &'static str> {
        let error = "oops it did not work !";

        if num == 50 {
            Ok(num as u32)
        } else {
            Err(error)
        }
    }

    match is_it_fifty(my_num) {
        Ok(_v) => {
            println!("Good ! my_num is 50 !");
        }
        Err(_e) => {
            println!("Error ! my_num is not 50 , it is actually : {}", my_num);
        }
    }

    let f = File::open("hello.txt");

    // method-1 : simple case

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("problem opening the file : {:?}", error),
    // };

    // method-2 : little more detailed way of handling error

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // recover from the error and create the file when it originally was not found.
            // creating the file could also fail
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("problem creating the file : {:?}", e),
            },
            other_error => {
                panic!("problem opening the file : {:?}", other_error)
            }
        },
    };
}

// error handling example-1

fn read_data_from_file_v1() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// error handling example-2

/*
In the below example, we have this >
let mut f = File::open("hello.txt")?;
and this >
f.read_to_string(&mut s)?;

? symbol : means, if there is an error, then return it immediately (rust will take care of it)
*/
fn read_data_from_file_v2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_data_from_file_v3() -> Result<String, io::Error> {
    // example of chaining , that is reducing the lines of code in read_data_from_file_v2()
    let mut s = String::new();
    let mut f = File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

/*
In the below example, we have this as the return type:
Result<(), Box<dyn std::error::Error>>

() : empty parentheses : this of this as *void*, or basically *nothing*
*/

#[tokio::main]
async fn make_http_request()  -> Result<(), Box<dyn std::error::Error>> {
    // Build the client using the builder pattern
    let client = reqwest::Client::builder()
        .build()?;

    // Perform the actual execution of the network request
    let res = client
        .get("https://v2.jokeapi.dev/joke/Any")
        .send()
        .await?;

    println!("status : {}", res.status());

    println!("{:?}", res);
    Ok(())
}

fn get_occupation(name: &str) -> Option<&str> {
    match name {
        "giridhar" => Some("software developer"),
        "linus" => Some("linux kernel developer"),
        _ => None
    }
}

fn check(my_name: &str) {
    match my_name {
        "linus" => {
            println!("he created linux !");
        }
        "giridhar" => {
            println!("he is getting better at RUST language");
        }
        "clinton" | "bush" => {
            println!("they were US presidents !");
        }
        _ => {
            println!("oops : does not match anything");
        }
    }
}

struct Color {
    red: u8,
    green: u8,
    blue: u8
}

struct TupleColor(u8, u8, u8);

#[derive(Debug)] // this will allow printing of structs
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn print_description(&self) {
        println!("Rectangle : {} x {}", self.width, self.height)
    }

    fn is_square(&self) -> bool {
        return if self.height == self.width {
            true
        } else {
            false
        }
    }
}

#[derive(Debug)] // so that we can print it
struct Person {
    name: String,
    age: u8
}

// implementing traits

impl ToString for Person {
    fn to_string(&self) -> String {
        return format!("My Name Is {}, & I am {}", self.name, self.age);
    }
}

fn print_numbers(start: u32, end: u32) {
    for i in start..end {
        let (n, is_even) = is_even(i);
        println!("{} => {}", n, is_even);
    }
}

fn is_even(num: u32) -> (u32, bool) {
    let num_even = num %2 == 0;
    return (num, num_even)
}

fn print_color(c: &Color) {
    println!("color values are : {}, {}, {}", c.red, c.green, c.blue)
}

// implement 'HasVoiceBox' trait, on 'Person' struct
trait HasVoiceBox {
    // speak
    fn speak(&self);
    // can speak
    fn can_speak(&self) -> bool;
}
// implement 'HasVoiceBox' trait, on 'Person' struct
impl HasVoiceBox for Person {
    fn speak(&self) {
        println!("Hello, my name is {} and my age is {}", self.name, self.age);
    }

    fn can_speak(&self) -> bool {
        return if self.age > 2 {
            true
        } else {
            false
        }
    }
}

// for 'cargo test', used in below testing module
fn get_data_for_testing(d: u32) -> u32 {
    return d;
}

// struct - used for testing
struct MyBox {
    width: u32,
    height: u32
}

// used by Box struct - for testing below
impl MyBox {
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

// Run : cargo test
#[cfg(test)] // cargo build will also compile this module, to avoid this , add this line
mod my_testing {
    // use core::panicking::panic;
    #[test]
    fn test_basic() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_basic_no_equal() {
        assert_ne!(super::get_data_for_testing(2), 3);
    }

    #[test]
    #[ignore] // this test will be ignored ('cargo test' will ignore it)
    fn test_basic_ignore() {
        assert_ne!(2, 3);
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("holy shit !");
    }

    #[test]
    #[should_panic]
    fn test_structs_fail() {
        let box1 = super::MyBox{
            width: 50,
            height: 25
        };
        assert!(box1.is_square());
    }

    #[test]
    fn test_structs_pass() {
        let box2 = super::MyBox{
            width: 50,
            height: 50
        };
        assert!(box2.is_square());
    }

}