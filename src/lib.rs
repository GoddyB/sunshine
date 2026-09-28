use worker::*;
use switchyard_translation::{decode_request, WireFormat, LlmRequest};

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_log!("Incoming request: {} {}", req.method(), req.path());

    match req.path().as_str() {
        "/" => Response::ok("Hello from sunshine!"),
        "/hello" => Response::ok("Hello there!"),
        "/models/sunshine" => {
            let body = req.text().await?;
            let ir: LlmRequest = decode_request(&body, WireFormat::OpenAiChat).map_err(|e| worker::Error::RustError(e.to_string()))?;
            Response::ok(serde_json::to_string(&ir).unwrap_or_default())
        }
        _ => Response::error("Not found", 404),
    }
}
