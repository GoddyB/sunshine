use worker::*;

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_log!("Incoming request: {} {}", req.method(), req.path());

    match req.path().as_str() {
        "/" => Response::ok("Hello from sunshine!"),
        "/hello" => Response::ok("Hello there!"),
        _ => Response::error("Not found", 404),
    }
}
