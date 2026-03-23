package service

import (
	"context"
	"io"

	"github.com/gabriel-vasile/mimetype"
	"github.com/shelepuginivan/carmen/search/pkg/infra"
	"github.com/shelepuginivan/carmen/search/pkg/model"
	"github.com/shelepuginivan/carmen/search/pkg/repository"
)

type WorkspaceService struct {
	wr *repository.WorkspaceRepository
	dr *repository.DocumentRepository
	ep *infra.ExtractorProducer
}

func NewWorkspace(
	wr *repository.WorkspaceRepository,
	dr *repository.DocumentRepository,
	ep *infra.ExtractorProducer,
) *WorkspaceService {
	return &WorkspaceService{wr, dr, ep}
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

func (ws *WorkspaceService) PaginateWorkspaceDocuments(
	ctx context.Context,
	identifier string,
	page int,
	limit int,
) ([]*model.Document, error) {
	workspace, err := ws.wr.GetWorkspace(ctx, identifier)
	if err != nil {
		return nil, err
	}

	documents, err := ws.dr.ListDocumentsInWorkspace(
		ctx,
		workspace.ID,
		repository.Paginate(page, limit),
	)
	if err != nil {
		return nil, err
	}

	return documents, nil
}

func (ws *WorkspaceService) ListWorkspaces(ctx context.Context) ([]*model.Workspace, error) {
	return ws.wr.ListWorkspaces(ctx)
}

func (ws *WorkspaceService) PaginateWorkspaces(ctx context.Context, page, limit int) ([]*model.Workspace, error) {
	return ws.wr.ListWorkspaces(ctx, repository.Paginate(page, limit))
}

func (ws *WorkspaceService) DeleteWorkspace(ctx context.Context, identifier string) error {
	return ws.wr.DeleteWorkspace(ctx, identifier)
}

func (ws *WorkspaceService) UploadDocumentToWorkspace(
	ctx context.Context,
	identifier string,
	filename string,
	content io.ReadSeeker,
) (*model.Document, error) {
	workspace, err := ws.wr.GetWorkspace(ctx, identifier)
	if err != nil {
		return nil, err
	}

	document, err := ws.dr.CreateDocument(ctx, workspace.ID, filename, content)
	if err != nil {
		return nil, err
	}

	mime, err := mimetype.DetectReader(content)
	if err != nil {
		return nil, err
	}
	content.Seek(0, io.SeekStart)

	err = ws.ep.EnqueueDocumentForExtraction(ctx, document.ID, document.Filename, mime.String())
	if err != nil {
		return nil, err
	}

	return document, nil
}
