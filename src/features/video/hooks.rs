use super::model::Video;

use reqwasm::http::Request;
use yew::prelude::*;

pub fn use_videos() -> Vec<Video> {
    let videos = use_state(|| vec![]);
    {
        let videos = videos.clone();
        use_effect_with_deps(
            move |_| {
                let videos = videos.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let result = Request::get("/tutorial/data.json")
                        .send()
                        .await
                        .expect("cannot fetch data.");

                    let fetched_videos: Vec<Video> =
                        result.json().await.expect("cannot serialize into json.");

                    videos.set(fetched_videos);
                });
                || ()
            },
            (),
        );
    }
    (*videos).clone()
}
