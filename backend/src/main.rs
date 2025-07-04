use poem::{EndpointExt, Result, Route, Server, listener::TcpListener, middleware::Cors};
use poem_openapi::{ApiResponse, Object, OpenApi, OpenApiService, param::Query, payload::Json};
use std::fs;

#[derive(Object)]
struct Hello {
    message: String,
}

#[derive(ApiResponse)]
enum HelloResponse {
    #[oai(status = 200)]
    Ok(Json<Hello>),
}

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>) -> Result<HelloResponse> {
        println!("requested!");
        Ok(HelloResponse::Ok(Json(match name.0 {
            Some(name) => Hello {
                message: format!("hello, {name}!"),
            },
            None => Hello {
                message: "hello!".to_string(),
            },
        })))
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let port = env!("API_PORT");
    println!("{:#?}", port);
    let origin = format!("http://localhost:{}", port);

    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "poem=debug");
        }
    }
    tracing_subscriber::fmt::init();

    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server(format!("{}/api", origin));

    let spec = api_service.spec_yaml();
    fs::write("openapi/schema.yaml", spec).expect("Unable to write file");

    let ui = api_service.swagger_ui();

    Server::new(TcpListener::bind(format!("0.0.0.0:{}", port)))
        .run(
            Route::new()
                .nest("/api", api_service)
                .nest("/", ui)
                .with(Cors::new()),
        )
        .await
}
