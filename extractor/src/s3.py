import boto3

from models import Config


class DocumentsBucket:
    def __init__(self, config: Config) -> None:
        self._bucket = config.s3_bucket
        self._client = boto3.client(
            "s3",
            endpoint_url=config.s3_endpoint,
            aws_access_key_id=config.s3_access_key,
            aws_secret_access_key=config.s3_secret_key,
            region_name=config.s3_region,
        )

    def get_object(self, key: str) -> bytes:
        res = self._client.get_object(Bucket=self._bucket, Key=key)
        return res["Body"].read()
