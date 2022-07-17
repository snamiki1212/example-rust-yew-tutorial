use crate::components::video_details::VideoDetails;
use crate::components::video_list::VideoList;
use crate::models::video::Video;
use reqwasm::http::Request;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
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

    let selected_video = use_state(|| None);
    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| selected_video.set(Some(video)))
    };
    let details = selected_video.as_ref().map(|video| {
        html! {
            <VideoDetails video={video.clone()} />
        }
    });

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
                <VideoList videos={(*videos).clone()} on_click={on_video_select.clone()} />
            </div>
            {for details}
        </>
    }
}
