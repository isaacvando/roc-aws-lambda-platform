use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde_json::Value;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

async fn function_handler(event: LambdaEvent<Value>) -> Result<String, Error> {
    let payload = event.into_parts().0;
    let bytes = serde_json::to_vec(&payload).unwrap();
    println!("before call");
    unsafe {
        let response = host::mainForHost(bytes).force_thunk();
        // Ok(serde_json::from_str(&response).unwrap())
        // let r = *response;
        // println!("{}", r);
        Ok(response)
    }
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
