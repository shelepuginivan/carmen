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

// FullTextSearch godoc
//
// @summary Full text search
// @router /search/fulltext [get]
// @tags search
// @param q query string true "Search query" minlength(1)
// @param workspace query string true "Workspace ID"
// @param limit query int false "Search result limit" minimum(1) default(5)
// @param threshold query number false "Search result relevance threshold" default(0.0)
// @produce json
// @success 200 {array} dto.SearchResponse
// @failure 400
// @failure 500
func (dc *SearchController) FullTextSearch(c *gin.Context) {
	var req dto.SearchRequest
	if err := c.ShouldBindQuery(&req); err != nil {
		respondWithError(c, http.StatusBadRequest, err)
		return
	}

	chunks, err := dc.srv.FullTextSearch(
		c.Request.Context(),
		req.Workspace,
		req.Query,
		req.Limit,
		req.Threshold,
	)
	if err != nil {
		respondWithError(c, http.StatusInternalServerError, err)
		return
	}

	result := make([]*dto.SearchResponse, len(chunks))

	for i, c := range chunks {
		result[i] = &dto.SearchResponse{
			ID:         c.ID,
			DocumentID: c.DocumentID,
			Text:       c.Text,
			Relevance:  c.Relevance,
		}
	}

	c.JSON(http.StatusOK, result)
}

// SemanticSearch godoc
//
// @summary Semantic search
// @router /search/semantic [get]
// @tags search
// @param q query string true "Search query" minlength(1)
// @param workspace query string true "Workspace ID"
// @param limit query int false "Search result limit" minimum(1) default(5)
// @param threshold query number false "Search result relevance threshold" default(0.0)
// @produce json
// @success 200 {array} dto.SearchResponse
// @failure 400
// @failure 500
func (dc *SearchController) SemanticSearch(c *gin.Context) {
	var req dto.SearchRequest
	if err := c.ShouldBindQuery(&req); err != nil {
		respondWithError(c, http.StatusBadRequest, err)
		return
	}

	chunks, err := dc.srv.SemanticSearch(
		c.Request.Context(),
		req.Workspace,
		req.Query,
		req.Limit,
		req.Threshold,
	)
	if err != nil {
		respondWithError(c, http.StatusInternalServerError, err)
		return
	}

	result := make([]*dto.SearchResponse, len(chunks))

	for i, c := range chunks {
		result[i] = &dto.SearchResponse{
			ID:         c.ID,
			DocumentID: c.DocumentID,
			Text:       c.Text,
			Relevance:  c.Relevance,
		}
	}

	c.JSON(http.StatusOK, result)
}

// SimilaritySearch godoc
//
// @summary Similarity search
// @router /search/similarity [get]
// @tags search
// @param q query string true "Search query" minlength(1)
// @param workspace query string true "Workspace ID"
// @param limit query int false "Search result limit" minimum(1) default(5)
// @param threshold query number false "Search result relevance threshold" default(0.0)
// @produce json
// @success 200 {array} dto.SearchResponse
// @failure 400
// @failure 500
func (dc *SearchController) SimilaritySearch(c *gin.Context) {
	var req dto.SearchRequest
	if err := c.ShouldBindQuery(&req); err != nil {
		respondWithError(c, http.StatusBadRequest, err)
		return
	}

	chunks, err := dc.srv.SimilaritySearch(
		c.Request.Context(),
		req.Workspace,
		req.Query,
		req.Limit,
		req.Threshold,
	)
	if err != nil {
		respondWithError(c, http.StatusInternalServerError, err)
		return
	}

	result := make([]*dto.SearchResponse, len(chunks))

	for i, c := range chunks {
		result[i] = &dto.SearchResponse{
			ID:         c.ID,
			DocumentID: c.DocumentID,
			Text:       c.Text,
			Relevance:  c.Relevance,
		}
	}

	c.JSON(http.StatusOK, result)
}
