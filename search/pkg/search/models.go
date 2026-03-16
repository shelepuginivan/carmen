package search

import (
	"github.com/google/uuid"
	"github.com/pgvector/pgvector-go"
	"gorm.io/gorm"
)

// Workspace represents a collection of related documents.
type Workspace struct {
	gorm.Model `json:"-"`

	ID          string `gorm:"primaryKey" json:"id,omitempty"`
	Name        string `gorm:"uniqueIndex" json:"name,omitempty"`
	Description string `json:"description,omitempty"`

	Documents []Document `json:"documents,omitempty"`
}

func (w *Workspace) BeforeCreate(tx *gorm.DB) (err error) {
	w.ID = uuid.New().String()
	return
}

type Document struct {
	gorm.Model

	ID          string `gorm:"primaryKey"`
	WorkspaceID string
	Name        string
	ObjectKey   string

	Chunks []Chunk
}

func (d *Document) BeforeCreate(tx *gorm.DB) (err error) {
	d.ID = uuid.New().String()
	return
}

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
