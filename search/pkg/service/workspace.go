package service

import (
	"context"

	"github.com/shelepuginivan/carmen/search/pkg/model"
	"github.com/shelepuginivan/carmen/search/pkg/repository"
)

type WorkspaceService struct {
	wr *repository.WorkspaceRepository
	dr *repository.DocumentRepository
}

func NewWorkspace(wr *repository.WorkspaceRepository, dr *repository.DocumentRepository) *WorkspaceService {
	return &WorkspaceService{wr, dr}
}

func (ws *WorkspaceService) CreateWorkspace(ctx context.Context, name string, description string) (*model.Workspace, error) {
	return ws.wr.CreateWorkspace(ctx, name, description)
}

func (ws *WorkspaceService) GetWorkspace(ctx context.Context, identifier string) (*model.Workspace, error) {
	return ws.wr.GetWorkspace(ctx, identifier)
}

func (ws *WorkspaceService) GetWorkspaceDocuments(ctx context.Context, identifier string) ([]*model.Document, error) {
	workspace, err := ws.wr.GetWorkspace(ctx, identifier)
	if err != nil {
		return nil, err
	}

	documents, err := ws.dr.ListDocumentsInWorkspace(ctx, workspace.ID)
	if err != nil {
		return nil, err
	}

	return documents, nil
}

func (ws *WorkspaceService) ListWorkspaces(ctx context.Context) ([]*model.Workspace, error) {
	return ws.wr.ListWorkspaces(ctx)
}

func (ws *WorkspaceService) PaginateWorkspaces(ctx context.Context, page, limit int) ([]*model.Workspace, error) {
	return ws.wr.PaginateWorkspaces(ctx, page, limit)
}

func (ws *WorkspaceService) DeleteWorkspace(ctx context.Context, identifier string) error {
	return ws.wr.DeleteWorkspace(ctx, identifier)
}
