package search

import (
	"context"
	"io"

	"github.com/aws/aws-sdk-go-v2/credentials"
	"github.com/aws/aws-sdk-go-v2/service/s3"
	"github.com/shelepuginivan/carmen/search/pkg/config"
)

type DocumentStorage struct {
	client *s3.Client
	bucket string
}

func NewDocumentStorage(cfg *config.Config) *DocumentStorage {
	client := s3.New(s3.Options{
		Region:       cfg.S3Region,
		Credentials:  credentials.NewStaticCredentialsProvider(cfg.S3AccessKey, cfg.S3SecretKey, ""),
		BaseEndpoint: &cfg.S3Endpoint,
		UsePathStyle: true,
	})

	return &DocumentStorage{
		client: client,
		bucket: cfg.S3Bucket,
	}
}

func (ds *DocumentStorage) GetDocument(ctx context.Context, key string) ([]byte, error) {
	res, err := ds.client.GetObject(ctx, &s3.GetObjectInput{
		Bucket: &ds.bucket,
		Key:    &key,
	})
	if err != nil {
		return nil, err
	}
	defer res.Body.Close()

	return io.ReadAll(res.Body)
}

func (ds *DocumentStorage) PutDocument(ctx context.Context, key string, body io.Reader) error {
	_, err := ds.client.PutObject(ctx, &s3.PutObjectInput{
		Bucket: &ds.bucket,
		Key:    &key,
		Body:   body,
	})
	return err
}

func (ds *DocumentStorage) DeleteDocument(ctx context.Context, key string) error {
	_, err := ds.client.DeleteObject(ctx, &s3.DeleteObjectInput{
		Bucket: &ds.bucket,
		Key:    &key,
	})
	return err
}
