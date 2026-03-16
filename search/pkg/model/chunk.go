package model

import (
	"github.com/google/uuid"
	"github.com/pgvector/pgvector-go"
	"gorm.io/gorm"
)

// Chunk represents chunk of a document that is indexed for hybrid search.
type Chunk struct {
	gorm.Model

	ID         string `gorm:"primaryKey"`
	DocumentID string
	Text       string
	Embedding  pgvector.Vector `gorm:"type:vector(1024)"`
}

func (c *Chunk) BeforeCreate(tx *gorm.DB) (err error) {
	c.ID = uuid.New().String()
	return
}
