use axum::{
    extract::State,
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Json, Router,
};
use color_eyre::eyre::eyre;
use serde::{Deserialize, Serialize};
use tower::ServiceBuilder;
use tower_http::{catch_panic::CatchPanicLayer, trace, trace::TraceLayer};
use tracing::Level;
use tracing_subscriber::{
    fmt::writer::MakeWriterExt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

/// curl http://localhost:8000/html
/// curl http://localhost:8000/text
/// curl http://localhost:8000/json
/// curl -H "Content-Type: application/json" -X POST -d '{"username":"心斋、坐忘"}' http://localhost:8000/post_json

struct Service {}

impl Service {
    /// 静态字段
    const VERSION: i32 = 123;

    /// 静态方法
    async fn test() -> String {
        println!("Service::VERSION = {}", Service::VERSION);
        Service::VERSION.to_string()
        // "testing...".to_string()
    }
}

#[derive(Clone)]
struct AppVersion {
    app: String,
    version: &'static str,
}

#[tokio::test]
async fn run_server() {
    // tracing_subscriber::fmt()
    //     .with_max_level(Level::INFO)
    //     .with_target(false)
    //     .compact()
    //     .pretty()
    //     .init();
    let tracing_layer = tracing_subscriber::fmt::layer().with_target(false);
    let collector = tracing_subscriber::registry()
        // .with(EnvFilter::from_default_env().add_directive(Level::TRACE.into()))
        // .with(EnvFilter::from_default_env().add_directive(Level::DEBUG.into()))
        .with(EnvFilter::from_default_env().add_directive(Level::INFO.into()))
        .with(tracing_layer);
    tracing::subscriber::set_global_default(collector).expect("Could not set global default");

    let app_version = AppVersion {
        app: "demo-app".to_string(),
        version: "1.2.3",
    };

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        .on_request(trace::DefaultOnRequest::new().level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO));

    let app = Router::new()
        .route("/html", get(html))
        .route("/text", get(text))
        .route("/json", get(json))
        .route("/version", get(version))
        .route("/service", get(Service::test))
        .route("/error", get(error))
        .route("/panic", get(panic))
        .route("/post_json", post(post_json))
        .route("/before", get(before))
        .route("/after", get(after))
        .layer(
            ServiceBuilder::new()
                .layer(trace_layer)
                // .layer(TraceLayer::new_for_http())
                .layer(CatchPanicLayer::new()),
        )
        .with_state(app_version);

    let addr = "0.0.0.0:8888".parse().unwrap();
    println!("listening on: {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn html() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn text() -> &'static str {
    "Hello, World!"
}

async fn json() -> Json<User> {
    let user = User {
        id: 888,
        username: "hi, hang zhou!".to_string(),
    };
    Json(user)
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

async fn post_json(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 888,
        username: format!("明玉功十二重楼: {}、婴宁", payload.username),
    };

    (StatusCode::CREATED, Json(user))
}

async fn version(State(app_version): State<AppVersion>) -> String {
    format!("{} {}\n", app_version.app, app_version.version)
}

async fn panic() -> String {
    std::panic::set_hook(Box::new(|msg| {
        tracing::info!("Custom panic hook: {}", msg);
    }));

    //捕获panic
    let result = std::panic::catch_unwind(|| {
        panic!("This is a panic message");
    });
    if result.is_err() {
        tracing::error!("A panic was caught!");
    }

    panic!("panic: test");
}

async fn before() -> &'static str {
    "before"
}

async fn after() -> &'static str {
    "after"
}

async fn error() -> &'static str {
    let err = eyre!("test err");
    tracing::error!("{:?}", err);
    "error"
}

fn layer_handler() {}

fn route_layer_handler() {}
