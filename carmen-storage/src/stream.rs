use s3::request::DataStream;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

pub enum Stream {
    FS(ReaderStream<File>),
    S3(DataStream),
}
