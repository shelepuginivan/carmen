// Package chunk implements chunk business logic.
package chunk

import (
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
