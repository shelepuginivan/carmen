package dto

type ChunkReady struct {
	DocumentID string    `json:"document_id"`
	Text       string    `json:"text"`
	Embedding  []float32 `json:"embedding"`
}

type ChunkMetadata struct {
	ID         string `json:"id"`
	DocumentID string `json:"document_id"`
	Text       string `json:"text"`
}
