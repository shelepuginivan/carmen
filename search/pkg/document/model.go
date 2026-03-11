// Package document implements document business logic.
package document

import (
	"github.com/google/uuid"
	"github.com/shelepuginivan/carmen/search/pkg/chunk"
	"gorm.io/gorm"
)

type Document struct {
	gorm.Model

	ID          string `gorm:"primaryKey"`
	WorkspaceID string
	Name        string
	ObjectKey   string

	Chunks []chunk.Chunk
}

func (d *Document) BeforeCreate(tx *gorm.DB) (err error) {
	d.ID = uuid.New().String()
	return
}
