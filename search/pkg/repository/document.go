package repository

import (
	"context"
	"io"

	"github.com/shelepuginivan/carmen/search/pkg/infra"
	"github.com/shelepuginivan/carmen/search/pkg/model"
	"gorm.io/gorm"
)

type DocumentRepository struct {
	db *gorm.DB
	s3 *infra.S3
}

func NewDocument(db *gorm.DB, s3 *infra.S3) *DocumentRepository {
	return &DocumentRepository{
		db: db,
		s3: s3,
	}
}

func (dr *DocumentRepository) CreateDocument(
	ctx context.Context,
	workspaceID string,
	filename string,
	body io.Reader,
) (*model.Document, error) {
	document := model.Document{
		Filename:    filename,
		WorkspaceID: workspaceID,
	}

	err := dr.db.WithContext(ctx).Create(&document).Error
	if err != nil {
		return nil, wrapErr(err)
	}

	if err := dr.s3.PutDocument(ctx, document.ID, body); err != nil {
		return nil, wrapErr(err)
	}

	return &document, nil
}

func (dr *DocumentRepository) GetDocumentMetadata(
	ctx context.Context,
	documentID string,
) (*model.Document, error) {
	var document model.Document

	err := dr.db.
		WithContext(ctx).
		Select("id", "filename").
		Where("id = ?", documentID).
		First(&document).
		Error

	return &document, wrapErr(err)
}

func (dr *DocumentRepository) GetDocumentContents(
	ctx context.Context,
	documentID string,
) (io.ReadCloser, error) {
	return dr.s3.GetDocument(ctx, documentID)
}

func (dr *DocumentRepository) ListDocumentsInWorkspace(
	ctx context.Context,
	workspaceID string,
	scopes ...Scope,
) ([]*model.Document, error) {
	var documents []*model.Document

	err := dr.db.
		WithContext(ctx).
		Scopes(scopes...).
		Select("id", "filename").
		Where("workspace_id = ?", workspaceID).
		Find(&documents).
		Error
	if err != nil {
		return nil, wrapErr(err)
	}

	return documents, nil
}

func (dr *DocumentRepository) DeleteDocument(ctx context.Context, documentID string) error {
	var document model.Document

	err := dr.db.
		WithContext(ctx).
		Where("id = ?", documentID).
		Delete(&document).
		Error
	if err != nil {
		return wrapErr(err)
	}

	return dr.s3.DeleteDocument(ctx, documentID)
}
