package infra

import (
	"context"
	"io"

	"github.com/aws/aws-sdk-go-v2/credentials"
	"github.com/aws/aws-sdk-go-v2/service/s3"
	"github.com/shelepuginivan/carmen/search/pkg/config"
)

type S3 struct {
	client *s3.Client
	bucket string
}

func NewS3(cfg *config.S3) *S3 {
	client := s3.New(s3.Options{
		Region:       cfg.Region,
		Credentials:  credentials.NewStaticCredentialsProvider(cfg.AccessKey, cfg.SecretKey, ""),
		BaseEndpoint: &cfg.Endpoint,
		UsePathStyle: true,
	})

	return &S3{
		client: client,
		bucket: cfg.Bucket,
	}
}

func (ds *S3) GetDocument(ctx context.Context, key string) (io.ReadCloser, error) {
	res, err := ds.client.GetObject(ctx, &s3.GetObjectInput{
		Bucket: &ds.bucket,
		Key:    &key,
	})
	if err != nil {
		return nil, err
	}

	return res.Body, err
}

func (ds *S3) PutDocument(ctx context.Context, key string, body io.Reader) error {
	_, err := ds.client.PutObject(ctx, &s3.PutObjectInput{
		Bucket: &ds.bucket,
		Key:    &key,
		Body:   body,
	})
	return err
}

func (ds *S3) DeleteDocument(ctx context.Context, key string) error {
	_, err := ds.client.DeleteObject(ctx, &s3.DeleteObjectInput{
		Bucket: &ds.bucket,
		Key:    &key,
	})
	return err
}
