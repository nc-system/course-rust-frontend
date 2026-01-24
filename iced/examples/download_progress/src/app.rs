use crate::download;

#[derive(Debug, Clone)]
pub enum Message {
    DownloadProgress(download::Progress),
    DownloadFinished(Result<(), download::Error>),
    StartDownload,
}
