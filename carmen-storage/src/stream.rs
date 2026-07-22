use s3::request::DataStream;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

pub enum Stream {
    FS(ReaderStream<File>),
    S3(DataStream),
}

#[cfg(feature = "axum")]
impl From<Stream> for axum::body::Body {
    fn from(value: Stream) -> Self {
        match value {
            Stream::FS(s) => axum::body::Body::from_stream(s),
            Stream::S3(s) => axum::body::Body::from_stream(s),
        }
    }
}
