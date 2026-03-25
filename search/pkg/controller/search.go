package controller

import (
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/shelepuginivan/carmen/search/pkg/dto"
	"github.com/shelepuginivan/carmen/search/pkg/service"
)

type SearchController struct {
	srv *service.SearchService
}

func NewSearch(srv *service.SearchService) *SearchController {
	return &SearchController{srv}
}

// SemanticSearch godoc
//
// @summary Semantic search
// @router /search/semantic [get]
// @tags search
// @param q query string true "Search query" minlength(1)
// @param workspace query string true "Workspace ID"
// @param limit query int false "Search result limit" minimum(1) default(5)
// @produce json
// @success 200 {array} dto.ChunkMetadata
// @failure 400
// @failure 500
func (dc *SearchController) SemanticSearch(c *gin.Context) {
	var req dto.SearchRequest
	if err := c.ShouldBindQuery(&req); err != nil {
		respondWithError(c, http.StatusBadRequest, err)
		return
	}

	chunks, err := dc.srv.SemanticSearch(c.Request.Context(), req.Workspace, req.Query, req.Limit)
	if err != nil {
		respondWithError(c, http.StatusInternalServerError, err)
		return
	}

	result := make([]*dto.ChunkMetadata, len(chunks))

	for i, c := range chunks {
		result[i] = &dto.ChunkMetadata{
			ID:         c.ID,
			DocumentID: c.DocumentID,
			Text:       c.Text,
		}
	}

	c.JSON(http.StatusOK, result)
}
