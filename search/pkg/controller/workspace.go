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
		c.JSON(http.StatusInternalServerError, gin.H{"detail": err.Error()})
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
