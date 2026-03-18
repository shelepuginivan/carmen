package controller

import (
	"mime"
	"net/http"
	"path/filepath"

	"github.com/gin-gonic/gin"
	"github.com/shelepuginivan/carmen/search/pkg/dto"
	"github.com/shelepuginivan/carmen/search/pkg/service"
)

type DocumentController struct {
	srv *service.DocumentService
}

func NewDocument(srv *service.DocumentService) *DocumentController {
	return &DocumentController{srv}
}

// GetDocumentMetadata godoc
//
// @summary Get document
// @router /document/{id} [get]
// @tags document
// @param id path string true "Document ID"
// @produce json
// @success 200 {object} dto.DocumentMetadata
// @failure 404
func (dc *DocumentController) GetDocumentMetadata(c *gin.Context) {
	doc, err := dc.srv.GetDocumentMetadata(c.Request.Context(), c.Param("id"))
	if err != nil {
		respondWithError(c, http.StatusNotFound, err)
		return
	}

	c.JSON(http.StatusOK, &dto.DocumentMetadata{
		ID:       doc.ID,
		Filename: doc.Filename,
	})
}

// GetDocumentContents godoc
//
// @summary Get document content
// @router /document/{id}/content [get]
// @tags document
// @param id path string true "Document ID"
// @produce octet-stream
// @success 200
// @failure 404
func (dc *DocumentController) GetDocumentContents(c *gin.Context) {
	doc, err := dc.srv.GetDocumentMetadata(c.Request.Context(), c.Param("id"))
	if err != nil {
		respondWithError(c, http.StatusNotFound, err)
		return
	}

	content, err := dc.srv.GetDocumentContents(c.Request.Context(), doc.ID)
	if err != nil {
		respondWithError(c, http.StatusNotFound, err)
		return
	}
	defer content.Close()

	mimetype := mime.TypeByExtension(filepath.Ext(doc.Filename))

	c.DataFromReader(
		http.StatusOK,
		-1,
		mimetype,
		content,
		map[string]string{},
	)
}

// DeleteDocument godoc
//
// @summary Delete document
// @router /document/{id} [delete]
// @tags document
// @param id path string true "Document ID"
// @produce json
// @success 200
// @failure 500
func (dc *DocumentController) DeleteDocument(c *gin.Context) {
	err := dc.srv.DeleteDocument(c.Request.Context(), c.Param("id"))
	if err != nil {
		respondWithError(c, http.StatusInternalServerError, err)
		return
	}

	c.JSON(http.StatusOK, gin.H{"ok": true})
}
