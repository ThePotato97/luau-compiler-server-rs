#[macro_use] extern crate rocket;
use mlua::{Compiler};

#[post("/", data = "<code>")]
fn index(code: String) -> String {
    let bytecode = Compiler::new().compile(&code);
    println!("{:?}", code);

    let formatted_string = bytecode.iter()
        .map(|byte| format!("\\{}", byte))
        .collect::<String>();

    formatted_string
}
#[get("/")]
fn instruct() -> &'static str {
    "pls send text/plain post only to this endpoint :3"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, instruct])
}