package model

import (
	"github.com/google/uuid"
	"github.com/pgvector/pgvector-go"
	"gorm.io/gorm"
)

// Chunk represents chunk of a document that is indexed for hybrid search.
type Chunk struct {
	ID         string `gorm:"primaryKey"`
	DocumentID string `gorm:"index"`
	Text       string
	Language   string          `gorm:"type:regconfig"`
	Embedding  pgvector.Vector `gorm:"type:vector(1024)"`
	FTSVector  string          `gorm:"->;type:tsvector GENERATED ALWAYS AS (to_tsvector(coalesce(language, 'english'), text)) STORED"`
	Relevance  float64         `gorm:"->"`
}

func (c *Chunk) BeforeCreate(tx *gorm.DB) (err error) {
	c.ID = uuid.New().String()
	return
}
