#[macro_use] extern crate rocket;
use mlua::{Compiler};

#[post("/", data = "<code>")]
fn index(code: String) -> String  {
    let bytecode = Compiler::new().compile(&code);
    let s = String::from_utf8_lossy(&*bytecode);
    let formatted_string = bytecode.iter()
        .map(|byte| format!("\\{}", byte))
        .collect::<String>();
    println!("{:?}", formatted_string);
    s.parse().unwrap()
}

#[post("/bytes", data = "<code>")]
fn bytes(code: String) -> String  {
    let bytecode = Compiler::new().compile(&code);
    let s = String::from_utf8_lossy(&*bytecode);
    let formatted_string = bytecode.iter()
        .map(|byte| format!("\\{}", byte))
        .collect::<String>();
    println!("{:?}", formatted_string);
    formatted_string
}
#[get("/")]
fn instruct() -> &'static str {
    "pls send text/plain post only to this endpoint :3"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, instruct, bytes])
}