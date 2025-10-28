use crate::Video;
use axum::{Router, extract::Path, response::Html, routing};
use std::sync::Arc;

pub struct VideoServer {
    pub router: Router,
}

const PREVIEW_HTML: &str = include_str!("preview.html");

impl VideoServer {
    pub fn new<C: 'static + Default>(video: Arc<Video<C>>) -> Self {
        let _ = video.progress.clear();

        let router = Router::new()
        .route("/", routing::get(async || Html(PREVIEW_HTML)))
        .route("/frame/{number_dot_svg}", 
            routing::get(async move |Path(number_dot_svg): Path<String>| {
                let number: usize = number_dot_svg
                    .strip_suffix(".svg")
                    .expect("Expecting /frame/{number}.svg, didn't find .svg at the end")
                    .parse()
                    .expect("Expecting /frame/{number}.svg, couldn't parse {number} to an integer");

                println!("");
                println!("Frame number requested: {number}");

                match video.render_single_frame(number) {
                    // Ok((timecode, svg)) => svg.to_string().replace(
                    //     "</svg>", 
                    //     &format!(r#"<meta name="shapemaker:timecode" content="{timecode}" /></svg>"#)
                    // ),
                    Ok(svg) => svg.to_string(),
                    Err(err) => format!("{err:?}"),
                }
            }),
        );

        Self { router }
    }

    pub async fn start(self, address: &str) {
        axum::serve(
            tokio::net::TcpListener::bind(address).await.unwrap(),
            self.router,
        )
        .await
        .unwrap();
    }
}

impl<C: 'static + Default> Video<C> {
    pub async fn serve(self, address: &str) {
        VideoServer::new(Arc::new(self)).start(address).await;
    }
}
