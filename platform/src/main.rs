// fn main() {
//     std::process::exit(host::rust_main() as _);
// }

use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    command: String,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let roc_str = host::rust_main();

    let resp = Response {
        req_id: event.context.request_id,
        msg: roc_str,
    };

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
