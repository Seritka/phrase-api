use rand::Rng;
use worker::*;

mod utils;

const ADVICES: &str = include_str!("advices.json");

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    utils::set_panic_hook();

    let router = Router::new();

    router
        .get("/", |_, _| {
            let advices: Vec<String> = serde_json::from_str(ADVICES).unwrap();
            let advice = advices.get(rand::thread_rng().gen_range(0..advices.len())).expect("No advice found");
            Response::ok_utf8(advice)
        })
        .get("/worker-version", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            Response::ok(version)
        })
        .run(req, env)
        .await
}

trait ResponseExt {
   fn ok_utf8(body: impl Into<String>) -> Result<Self> where Self: Sized;
}

impl ResponseExt for Response {
    fn ok_utf8(body: impl Into<String>) -> Result<Self> {

        let mut headers = Headers::new();
        headers.set("content-type", "text/plain; charset=utf-8")?;

        Ok(Self::from_body(ResponseBody::Body(body.into().into_bytes()))?.with_headers(headers))
    }
}