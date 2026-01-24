
use iced::Subscription;
use iced::futures::{self, StreamExt};
use futures::stream::BoxStream;
use std::sync::Arc;


#[derive(Debug, Clone)]
pub struct Progress {
    pub percent: f32,
}

#[derive(Debug, Clone)]
pub enum Error {
    RequestFailed(Arc<reqwest::Error>),
    NoContentLength,
}

#[derive(Debug, Clone)]
pub enum Message {
    DownloadProgress(Progress),
    DownloadFinished(Result<(), Error>),
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::RequestFailed(Arc::new(error))
    }
}



pub fn download_subscription(
    url: String,
) -> Subscription<Message> {
    Subscription::run_with(
        url.clone(),
        move |url| download_stream(url.clone()),
    )
}

struct Download;

fn download_stream(
    url: String,
) -> BoxStream<'static, Message> {
    futures::stream::unfold(
        (url, None::<reqwest::Client>, None::<(reqwest::Response, u64)>, 0usize),
        |(url, _, mut response_data, mut downloaded)| async move {
            if response_data.is_none() {
                let client = reqwest::Client::new();
                let resp = match client.get(&url).send().await {
                    Ok(r) => r,
                    Err(e) => {
                        return Some((
                            Message::DownloadFinished(Err(Error::RequestFailed(Arc::new(e)))),
                            (url, None, None, downloaded),
                        ));
                    }
                };

                let total = match resp.content_length() {
                    Some(t) => t,
                    None => {
                        return Some((
                            Message::DownloadFinished(Err(Error::NoContentLength)),
                            (url, None, None, downloaded),
                        ));
                    }
                };

                return Some((
                    Message::DownloadProgress(Progress { percent: 0.0 }),
                    (url, None, Some((resp, total)), downloaded),
                ));
            }

            let (mut resp, total) = response_data.take().unwrap();

            match resp.chunk().await {
                Ok(Some(bytes)) => {
                    downloaded += bytes.len();
                    let percent = 100.0 * downloaded as f32 / total as f32;

                    Some((
                        Message::DownloadProgress(Progress { percent }),
                        (url, None, Some((resp, total)), downloaded),
                    ))
                }
                Ok(None) => Some((
                    Message::DownloadFinished(Ok(())),
                    (url, None, None, downloaded),
                )),
                Err(e) => Some((
                    Message::DownloadFinished(Err(Error::RequestFailed(Arc::new(e)))),
                    (url, None, None, downloaded),
                )),
            }
        },
    )
    .boxed()
}






// use iced::futures::StreamExt;
// use iced::task::{Straw, sipper};

// use std::sync::Arc;

// pub fn download(url: impl AsRef<str>) -> impl Straw<(), Progress, Error> {
//     sipper(async move |mut progress| {
//         let response = reqwest::get(url.as_ref()).await?;
//         let total = response.content_length().ok_or(Error::NoContentLength)?;

//         let _ = progress.send(Progress { percent: 0.0 }).await;

//         let mut byte_stream = response.bytes_stream();
//         let mut downloaded = 0;

//         while let Some(next_bytes) = byte_stream.next().await {
//             let bytes = next_bytes?;
//             downloaded += bytes.len();

//             let _ = progress
//                 .send(Progress {
//                     percent: 100.0 * downloaded as f32 / total as f32,
//                 })
//                 .await;
//         }

//         Ok(())
//     })
// }

// #[derive(Debug, Clone)]
// pub struct Progress {
//     pub percent: f32,
// }

// #[derive(Debug, Clone)]
// pub enum Error {
//     RequestFailed(Arc<reqwest::Error>),
//     NoContentLength,
// }

// impl From<reqwest::Error> for Error {
//     fn from(error: reqwest::Error) -> Self {
//         Error::RequestFailed(Arc::new(error))
//     }
// }