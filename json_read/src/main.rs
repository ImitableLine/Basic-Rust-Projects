// import serde deserialize and Serialize for transfering data from Rust types to JSON in this instance.
use serde::{Deserialize, Serialize};
// Imports "from_str" from serde_json crate, later used to convert json strings to a vector
use serde_json::from_str;

// /* Important disclaimer */
// This code is taken from a github repo
// Link: https://gist.github.com/JosiahParry/cd1e132e91603df468b7043a52b26e88

// Derive is Rust's automatic way to implement common traits (like enums and structs) with automatic code generation
// by placing this here, it will implement the code to use the three macros.
// It is called "attribute macro" or plural macros.
#[derive(Deserialize, Serialize, Debug)]
struct Todo {
    // making a struct called Todo
    #[serde(rename = "userId")] // automatically rename user_id to userId
    user_id: usize, // user_id is now userId which contains usize, the platforms pointer type, (e.g.. 32 bit or 64 bit so u32 or u64)
    id: usize,       // id is also usize
    title: String,   // setting a full String
    completed: bool, // has completed task or not, so boolean true/false
}

// main function, it is run automatically when the compiled program is run
fn main() {
    // immutable variable called json
    // from standard library file system, using the read_to_string method
    // unwrap is a method that will panic if there is *None* or *Error* types returned.
    // is it taking the data in data.json and reading it as a string, storing it inside "json"
    let json = std::fs::read_to_string("./data.json").unwrap();

    // immutable var called todos
    // from_str method to take the reference to json, and convert it to a vector with the Todo struct format and storing it inside "todos"
    //  error pattern matching for from_str "Result<(T), (E)>"
    let todos = from_str::<Vec<Todo>>(&json);
    // using the #? is a macro from above, "Debug", this will allow easy printing of the Todo struct with todos data.
    println!("{:#?}", todos);
}
