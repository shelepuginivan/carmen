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
// @success 204
// @failure 400
func (wc *WorkspaceController) CreateWorkspace(c *gin.Context) {
	var workspace dto.WorkspaceCreate
	if err := c.ShouldBind(&workspace); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"detail": err.Error()})
		return
	}

	err := wc.srv.CreateWorkspace(c.Request.Context(), workspace.Name, workspace.Description)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"detail": err.Error()})
		return
	}

	c.AbortWithStatus(http.StatusNoContent)
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
