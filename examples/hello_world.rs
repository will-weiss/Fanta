extern crate fanta;
extern crate serde;
extern crate serde_json;
extern crate tokio_proto;
extern crate tokio_service;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;

mod context;

use fanta::{App, MiddlewareChain};
use context::{generate_context, Ctx};

lazy_static! {
  static ref APP: App<Ctx> = {
    let mut _app = App::<Ctx>::create(generate_context);

    _app.get("/json", vec![json]);
    _app.get("/plaintext", vec![plaintext]);

    _app.set404(vec![not_found_404]);

    _app
  };
}

fn not_found_404(context: Ctx, _chain: &MiddlewareChain<Ctx>) -> Ctx {
  let mut context = Ctx::new(context);

  context.body = "<html>
  ( ͡° ͜ʖ ͡°) What're you looking for here?
</html>".to_owned();
  context.set_header("Content-Type", "text/html");
  context.status_code = 404;

  context
}

#[derive(Serialize)]
struct JsonStruct<'a> {
  message: &'a str
}
fn json(mut context: Ctx, _chain: &MiddlewareChain<Ctx>) -> Ctx {
  let json = JsonStruct {
    message: "Hello, World!"
  };

  let val = serde_json::to_string(&json).unwrap();

  context.body = val;
  context.set_header("Server", "fanta");
  context.set_header("Content-Type", "application/json");

  context
}

fn plaintext(mut context: Ctx, _chain: &MiddlewareChain<Ctx>) -> Ctx {
  let val = "Hello, World!".to_owned();

  context.body = val;
  context.set_header("Server", "fanta");
  context.set_header("Content-Type", "text/plain");

  context
}

fn main() {
  println!("Starting server...");

  App::start(&APP, "0.0.0.0".to_string(), "8080".to_string());
}
