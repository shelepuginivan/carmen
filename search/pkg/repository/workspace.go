package repository

import (
	"context"

	"github.com/shelepuginivan/carmen/search/pkg/infra"
	"github.com/shelepuginivan/carmen/search/pkg/model"
	"gorm.io/gorm"
)

type WorkspaceRepository struct {
	db *gorm.DB
	s3 *infra.S3
}

func NewWorkspace(db *gorm.DB, s3 *infra.S3) *WorkspaceRepository {
	return &WorkspaceRepository{
		db: db,
		s3: s3,
	}
}

func (wr *WorkspaceRepository) CreateWorkspace(ctx context.Context, name string, description string) (*model.Workspace, error) {
	workspace := model.Workspace{
		Name:        name,
		Description: description,
	}

	err := wr.db.WithContext(ctx).Create(&workspace).Error

	return &workspace, wrapErr(err)
}

func (wr *WorkspaceRepository) GetWorkspace(ctx context.Context, identifier string) (*model.Workspace, error) {
	var workspace model.Workspace

	err := wr.db.
		WithContext(ctx).
		Where("id = ?", identifier).
		Or("name = ?", identifier).
		First(&workspace).
		Error

	return &workspace, wrapErr(err)
}

func (wr *WorkspaceRepository) ListWorkspaces(ctx context.Context, scopes ...Scope) ([]*model.Workspace, error) {
	var workspaces []*model.Workspace

	err := wr.db.
		WithContext(ctx).
		Scopes(scopes...).
		Select("id", "name", "description").
		Order("name").
		Find(&workspaces).
		Error

	return workspaces, wrapErr(err)
}

func (wr *WorkspaceRepository) DeleteWorkspace(ctx context.Context, identifier string) error {
	var workspace model.Workspace

	err := wr.db.
		WithContext(ctx).
		Preload("Documents", func(db *gorm.DB) *gorm.DB {
			return db.Select("id", "workspace_id")
		}).
		Where("id = ?", identifier).
		Or("name = ?", identifier).
		First(&workspace).
		Error
	if err != nil {
		return wrapErr(err)
	}

	for _, document := range workspace.Documents {
		_ = wr.s3.DeleteDocument(ctx, document.ID)
	}

	err = wr.db.
		WithContext(ctx).
		Unscoped().
		Delete(workspace).
		Error

	return wrapErr(err)
}
