package dto

type SemanticSearchResponse struct {
	Embedding []float32 `json:"embedding"`
}

type SearchRequest struct {
	Workspace string `form:"workspace" binding:"uuid,required"`
	Query     string `form:"q" binding:"required,min=1"`
	Limit     int    `form:"limit,default=5" binding:"min=1"`
}
