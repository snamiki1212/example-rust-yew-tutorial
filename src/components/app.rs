use crate::components::video_details::VideoDetails;
use crate::components::video_list::VideoList;
use crate::features::video::hooks::use_videos;
use crate::features::video::model::Video;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let videos = use_videos();
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
                <VideoList videos={videos} on_click={on_video_select.clone()} />
            </div>
            {for details}
        </>
    }
}
