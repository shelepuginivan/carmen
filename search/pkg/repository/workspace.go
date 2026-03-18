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
	if err != nil {
		return nil, err
	}

	return &workspace, nil
}

func (wr *WorkspaceRepository) GetWorkspace(ctx context.Context, identifier string) (*model.Workspace, error) {
	var workspace model.Workspace

	err := wr.db.
		WithContext(ctx).
		Where("id = ?", identifier).
		Or("name = ?", identifier).
		First(&workspace).
		Error

	return &workspace, err
}

func (wr *WorkspaceRepository) ListWorkspaces(ctx context.Context) ([]*model.Workspace, error) {
	var workspaces []*model.Workspace

	res := wr.db.
		WithContext(ctx).
		Select("id", "name", "description").
		Order("name").
		Find(&workspaces)

	return workspaces, res.Error
}

func (wr *WorkspaceRepository) PaginateWorkspaces(
	ctx context.Context,
	page int,
	limit int,
) ([]*model.Workspace, error) {
	var workspaces []*model.Workspace

	res := wr.db.
		WithContext(ctx).
		Scopes(paginate(page, limit)).
		Select("id", "name", "description").
		Order("name").
		Find(&workspaces)

	return workspaces, res.Error
}

func (wr *WorkspaceRepository) DeleteWorkspace(ctx context.Context, identifier string) error {
	var workspace model.Workspace

	err := wr.db.
		WithContext(ctx).
		Preload("Documents", func(db *gorm.DB) *gorm.DB {
			return db.Select("workspace_id", "filename")
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
