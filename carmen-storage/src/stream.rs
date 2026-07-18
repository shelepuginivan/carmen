use s3::request::DataStream;

pub enum Stream {
    S3(DataStream),
    FS(),
}
