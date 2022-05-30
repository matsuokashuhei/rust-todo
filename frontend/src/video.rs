use yew::prelude::*;

#[derive(PartialEq)]
pub struct Video {
    pub id: usize,
    pub title: String,
    pub speaker: String,
    pub url: String,
}

#[derive(Properties, PartialEq)]
pub struct VideosListProps {
    pub videos: Vec<Video>,
}

#[function_component(VideoList)]
pub fn video_list(VideosListProps { videos }: &VideosListProps) -> Html {
    // let videos = vec![
    //     Video {
    //         id: 1,
    //         title: "Building and breaking things".to_string(),
    //         speaker: "John Doe".to_string(),
    //         url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    //     },
    //     Video {
    //         id: 2,
    //         title: "The development process".to_string(),
    //         speaker: "Jane Smith".to_string(),
    //         url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    //     },
    //     Video {
    //         id: 3,
    //         title: "The Web 7.0".to_string(),
    //         speaker: "Matt Miller".to_string(),
    //         url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    //     },
    //     Video {
    //         id: 4,
    //         title: "Mouseless development".to_string(),
    //         speaker: "Tom Jerry".to_string(),
    //         url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    //     },
    // ];
    videos
        .iter()
        .map(|video| {
            html! {
                <p>{format!("{}: {}", video.speaker, video.title)}</p>
            }
        })
        .collect()
}
