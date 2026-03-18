package service

import (
	"context"
	"io"

	"github.com/shelepuginivan/carmen/search/pkg/model"
	"github.com/shelepuginivan/carmen/search/pkg/repository"
)

type DocumentService struct {
	dr *repository.DocumentRepository
}

func NewDocument(dr *repository.DocumentRepository) *DocumentService {
	return &DocumentService{dr}
}

func (ds *DocumentService) GetDocumentMetadata(ctx context.Context, id string) (*model.Document, error) {
	return ds.dr.GetDocumentMetadata(ctx, id)
}

func (ds *DocumentService) GetDocumentContents(ctx context.Context, id string) (io.ReadCloser, error) {
	return ds.dr.GetDocumentContents(ctx, id)
}

func (ds *DocumentService) DeleteDocument(ctx context.Context, id string) error {
	return ds.dr.DeleteDocument(ctx, id)
}
