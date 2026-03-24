package dto

type SemanticSearchResponse struct {
	Embedding []float32 `json:"embedding"`
}

type SearchRequest struct {
	Query string `form:"q" binding:"required,min=1"`
	Limit int    `form:"limit,default=5" binding:"min=1"`
}
