#[macro_use] extern crate rocket;
use mlua::{Compiler};
use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
struct Response {
    code: String,
}

#[post("/", data = "<code>")]
fn index(code: String) -> Json<Response>  {
    let bytecode = Compiler::new().compile(&code);
    println!("{:?}", code);

    let formatted_string = bytecode.iter()
        .map(|byte| format!("\\{}", byte))
        .collect::<String>();
    let response = Response {
        code: formatted_string,
    };

    Json(response)
}
#[get("/")]
fn instruct() -> &'static str {
    "pls send text/plain post only to this endpoint :3"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, instruct])
}