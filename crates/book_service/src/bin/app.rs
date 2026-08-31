use book_service::{AppConf, AppState, routes};
use tracing::info;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        "info,tower_http=warn,axum=warn,toasty=warn,tokio-postgres=warn".into()
    });

    tracing_subscriber::registry()
        .with(filter)
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_target(false)
                .with_current_span(false),
        )
        .init();

    let conf = AppConf::init();
    let addr = conf.server.to_addr();
    let server_conf = conf.server;
    let state = AppState { server_conf };
    info!(addr=%addr, "Starting server");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    let app = routes::init(state);
    axum::serve(listener, app).await?;

    Ok(())
}
