use todo_service::app::common::config::Config;
use todo_service::app::db::create_store;
use todo_service::app::handlers::routes::create_routes;
use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    // 로깅 초기화
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // 설정 로드
    let config = Config::from_env();

    // 데이터베이스(Store) 생성
    let store = create_store(config.database_url);

    // 라우트 생성
    let routes = create_routes(store);

    // 서버 주소 설정
    let addr: SocketAddr = ([127, 0, 0, 1], config.port).into();
    info!("🚀 서버가 다음 주소에서 실행됩니다: {}", addr);

    // 서버 실행
    warp::serve(routes).run(addr).await;
}
