package repository

import (
	"context"

	"github.com/shelepuginivan/carmen/search/pkg/dal"
	"github.com/shelepuginivan/carmen/search/pkg/model"
	"gorm.io/gorm"
)

type WorkspaceRepository struct {
	db *gorm.DB
	s3 *dal.S3
}

func NewWorkspace(db *gorm.DB, s3 *dal.S3) *WorkspaceRepository {
	return &WorkspaceRepository{
		db: db,
		s3: s3,
	}
}

func (wr *WorkspaceRepository) CreateWorkspace(ctx context.Context, name string, description string) error {
	return wr.db.WithContext(ctx).Create(&model.Workspace{
		Name:        name,
		Description: description,
	}).Error
}

func (wr *WorkspaceRepository) GetWorkspace(ctx context.Context, identifier string) (*model.Workspace, error) {
	var workspace model.Workspace

	res := wr.db.
		WithContext(ctx).
		Where("id = ?", identifier).
		Or("name = ?", identifier).
		First(&workspace)

	return &workspace, res.Error
}

func (wr *WorkspaceRepository) ListWorkspaces(ctx context.Context) ([]*model.Workspace, error) {
	var workspaces []*model.Workspace

	res := wr.db.
		WithContext(ctx).
		Select("id", "name", "description").
		Order("name").Find(&workspaces)

	return workspaces, res.Error
}

func (wr *WorkspaceRepository) DeleteWorkspace(ctx context.Context, identifier string) error {
	var workspace model.Workspace

	err := wr.db.
		WithContext(ctx).
		Preload("Documents", func(db *gorm.DB) *gorm.DB {
			return db.Select("filename")
		}).
		Where("id = ?", identifier).
		Or("name = ?", identifier).
		First(&workspace).
		Error
	if err != nil {
		return err
	}

	for _, document := range workspace.Documents {
		wr.s3.DeleteDocument(ctx, document.Filename)
	}

	return wr.db.
		WithContext(ctx).
		Unscoped().
		Delete(workspace).
		Error
}
