use axum::{extract::Path, routing::get, Json, Router};
use serde::Serialize;
use tower_http::services::ServeDir;

#[derive(Serialize)]
struct Episode {
    episode_num: u32,
    title: String,
    stream_url: String,
}

#[derive(Serialize)]
struct Season {
    season_num: u32,
    episodes: Vec<Episode>,
}

#[derive(Serialize)]
struct VideoInfo {
    id: String,
    title: String,
    description: String,
    stream_url: String,
    seasons: Vec<Season>,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/video/{id}", get(get_video_info))
        .nest_service("/media", ServeDir::new("example"))
        .fallback_service(ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("Server is running at http://127.0.0.1:8080");

    axum::serve(listener, app).await.unwrap();
}

async fn get_video_info(Path(id): Path<String>) -> Json<VideoInfo> {
    let response = VideoInfo {
        id,
        title: "Love, Death & Robots".to_string(),
        description: "Viewing MP4 Episodes natively over axum HTTP.".to_string(),
        stream_url: "http://127.0.0.1:1300/video/love_death_and_robots_1s_1ep/love_death_and_robots_1s_1ep.m3u8".to_string(),
        seasons: vec![Season {
            season_num: 1,
            episodes: vec![
                Episode {
                    episode_num: 1,
                    title: "Sonnie's Edge".to_string(),
                    stream_url: "http://127.0.0.1:1300/video/love_death_and_robots_1s_1ep/love_death_and_robots_1s_1ep.m3u8".to_string(),
                },
                Episode {
                    episode_num: 2,
                    title: "Three Robots".to_string(),
                    stream_url: "http://127.0.0.1:1300/video/love_death_and_robots_1s_2ep/love_death_and_robots_1s_2ep.m3u8".to_string(),
                },
            ],
        }],
    };

    Json(response)
}
