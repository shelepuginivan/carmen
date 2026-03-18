package controller

import (
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/shelepuginivan/carmen/search/pkg/dto"
	"github.com/shelepuginivan/carmen/search/pkg/service"
)

type WorkspaceController struct {
	srv *service.WorkspaceService
}

func NewWorkspace(srv *service.WorkspaceService) *WorkspaceController {
	return &WorkspaceController{srv}
}

// CreateWorkspace godoc
//
// @summary Create workspace
// @router /workspace [post]
// @tags workspace
// @accept json
// @param workspace body dto.WorkspaceCreate true "New workspace metadata"
// @produce json
// @success 201 {object} dto.WorkspaceGet
// @failure 400
func (wc *WorkspaceController) CreateWorkspace(c *gin.Context) {
	var params dto.WorkspaceCreate
	if err := c.ShouldBind(&params); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"detail": err.Error()})
		return
	}

	workspace, err := wc.srv.CreateWorkspace(c.Request.Context(), params.Name, params.Description)
	if err != nil {
		respondWithError(c, http.StatusInternalServerError, err)
		return
	}

	c.JSON(http.StatusCreated, &dto.WorkspaceGet{
		ID:          workspace.ID,
		Name:        workspace.Name,
		Description: workspace.Description,
	})
}

// GetWorkspace godoc
//
// @summary Get workspace metadata
// @router /workspace/{id-or-name} [get]
// @tags workspace
// @param id-or-name path string true "ID or name of the workspace"
// @produce json
// @success 200 {object} dto.WorkspaceGet
// @failure 404
func (wc *WorkspaceController) GetWorkspace(c *gin.Context) {
	ws, err := wc.srv.GetWorkspace(c.Request.Context(), c.Param("id-or-name"))
	if err != nil {
		c.AbortWithStatus(http.StatusNotFound)
		return
	}

	c.JSON(http.StatusOK, &dto.WorkspaceGet{
		ID:          ws.ID,
		Name:        ws.Name,
		Description: ws.Description,
	})
}

// GetWorkspaceDocuments godoc
//
// @summary Get all documents in workspaces
// @router /workspace/{id-or-name}/document/all [get]
// @tags workspace
// @param id-or-name path string true "ID or name of the workspace"
// @produce json
// @success 200 {array} dto.DocumentMetadata
// @failure 404
func (wc *WorkspaceController) GetWorkspaceDocuments(c *gin.Context) {
	documents, err := wc.srv.GetWorkspaceDocuments(c.Request.Context(), c.Param("id-or-name"))
	if err != nil {
		respondWithError(c, http.StatusNotFound, err)
		return
	}

	result := make([]*dto.DocumentMetadata, len(documents))

	for idx, doc := range documents {
		result[idx] = &dto.DocumentMetadata{
			ID:       doc.ID,
			Filename: doc.Filename,
		}
	}

	c.JSON(http.StatusOK, result)
}

// ListWorkspaces godoc
//
// @summary Get all workspaces
// @router /workspace/all [get]
// @tags workspace
// @produce json
// @success 200 {array} dto.WorkspaceGet
// @failure 500
func (wc *WorkspaceController) ListWorkspaces(c *gin.Context) {
	workspaces, err := wc.srv.ListWorkspaces(c.Request.Context())
	if err != nil {
		respondWithError(c, http.StatusInternalServerError, err)
		return
	}

	result := make([]*dto.WorkspaceGet, len(workspaces))

	for idx, ws := range workspaces {
		result[idx] = &dto.WorkspaceGet{
			ID:          ws.ID,
			Name:        ws.Name,
			Description: ws.Description,
		}
	}

	c.JSON(http.StatusOK, result)
}

// PaginateWorkspaces godoc
//
// @summary Get workspaces with pagination
// @router /workspace/all/page/{page} [get]
// @tags workspace
// @param page path int true "Page" minimum(1)
// @param limit query int false "Page size limit" minimum(10) maximum(100) default(10)
// @produce json
// @success 200 {array} dto.WorkspaceGet
// @failure 400
// @failure 500
func (wc *WorkspaceController) PaginateWorkspaces(c *gin.Context) {
	pagination, err := paginate(c)
	if err != nil {
		respondWithError(c, http.StatusBadRequest, err)
		return
	}

	workspaces, err := wc.srv.PaginateWorkspaces(
		c.Request.Context(),
		pagination.Page,
		pagination.Limit,
	)
	if err != nil {
		respondWithError(c, http.StatusInternalServerError, err)
		return
	}

	result := make([]*dto.WorkspaceGet, len(workspaces))

	for idx, ws := range workspaces {
		result[idx] = &dto.WorkspaceGet{
			ID:          ws.ID,
			Name:        ws.Name,
			Description: ws.Description,
		}
	}

	c.JSON(http.StatusOK, result)
}

// DeleteWorkspace godoc
//
// @summary Delete workspace
// @router /workspace/{id-or-name} [delete]
// @tags workspace
// @param id-or-name path string true "ID or name of the workspace"
// @produce json
// @success 200 {object} dto.WorkspaceGet
// @failure 500
func (wc *WorkspaceController) DeleteWorkspace(c *gin.Context) {
	err := wc.srv.DeleteWorkspace(c.Request.Context(), c.Param("id-or-name"))
	if err != nil {
		respondWithError(c, http.StatusInternalServerError, err)
		return
	}

	c.JSON(http.StatusOK, gin.H{"ok": true})
}
