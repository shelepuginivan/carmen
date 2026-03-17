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
