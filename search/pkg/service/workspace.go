package service

import (
	"context"

	"github.com/shelepuginivan/carmen/search/pkg/model"
	"github.com/shelepuginivan/carmen/search/pkg/repository"
)

type WorkspaceService struct {
	repo *repository.WorkspaceRepository
}

func NewWorkspace(repo *repository.WorkspaceRepository) *WorkspaceService {
	return &WorkspaceService{repo}
}

func (ws *WorkspaceService) CreateWorkspace(ctx context.Context, name string, description string) (*model.Workspace, error) {
	return ws.repo.CreateWorkspace(ctx, name, description)
}

func (ws *WorkspaceService) GetWorkspace(ctx context.Context, identifier string) (*model.Workspace, error) {
	return ws.repo.GetWorkspace(ctx, identifier)
}

func (ws *WorkspaceService) ListWorkspaces(ctx context.Context) ([]*model.Workspace, error) {
	return ws.repo.ListWorkspaces(ctx)
}

func (ws *WorkspaceService) PaginateWorkspaces(ctx context.Context, page, limit int) ([]*model.Workspace, error) {
	return ws.repo.PaginateWorkspaces(ctx, page, limit)
}

func (ws *WorkspaceService) DeleteWorkspace(ctx context.Context, identifier string) error {
	return ws.repo.DeleteWorkspace(ctx, identifier)
}
