use s3::request::DataStream;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

pub enum Stream {
    FS(ReaderStream<File>),
    S3(DataStream),
}

#[cfg(feature = "axum")]
impl Into<axum::body::Body> for Stream {
    fn into(self) -> axum::body::Body {
        match self {
            Self::FS(s) => axum::body::Body::from_stream(s),
            Self::S3(s) => axum::body::Body::from_stream(s),
        }
    }
}
