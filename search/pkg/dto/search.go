package dto

type SemanticSearchResponse struct {
	Embedding []float32 `json:"embedding"`
	Language  string    `json:"language"`
}

type SearchRequest struct {
	Workspace string  `form:"workspace" binding:"uuid,required"`
	Query     string  `form:"q" binding:"required,min=1"`
	Limit     int     `form:"limit,default=5" binding:"min=1"`
	Threshold float64 `form:"threshold,default=0.0"`
}

type SearchResponse struct {
	ID         string  `json:"id"`
	DocumentID string  `json:"document_id"`
	Text       string  `json:"text"`
	Relevance  float64 `json:"relevance"`
}
