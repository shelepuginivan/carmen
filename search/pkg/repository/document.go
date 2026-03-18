package repository

import (
	"context"
	"io"

	"github.com/shelepuginivan/carmen/search/pkg/infra"
	"github.com/shelepuginivan/carmen/search/pkg/model"
	"gorm.io/gorm"
	"gorm.io/gorm/clause"
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
		return nil, err
	}

	if err := dr.s3.PutDocument(ctx, filename, body); err != nil {
		return nil, err
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
		Preload("Chunks", func(db *gorm.DB) *gorm.DB {
			return db.Select("id", "text")
		}).
		Select("filename").
		First(&document, documentID).
		Error

	if err != nil {
		return nil, err
	}

	return &document, nil
}

func (dr *DocumentRepository) GetDocumentContents(
	ctx context.Context,
	documentID string,
) (io.Reader, error) {
	var document model.Document

	err := dr.db.Select("filename").First(&document, documentID).Error
	if err != nil {
		return nil, err
	}

	return dr.s3.GetDocument(ctx, document.Filename)
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
		return nil, err
	}

	return documents, nil
}

func (dr *DocumentRepository) DeleteDocument(ctx context.Context, documentID string) error {
	var document model.Document

	err := dr.db.
		WithContext(ctx).
		Clauses(clause.Returning{Columns: []clause.Column{{Name: "filename"}}}).
		Delete(&document, documentID).
		Error
	if err != nil {
		return err
	}

	return dr.s3.DeleteDocument(ctx, document.Filename)
}
