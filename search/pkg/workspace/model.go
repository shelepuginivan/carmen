// Package workspace implements workspace business logic.
package workspace

import (
	"github.com/google/uuid"
	"github.com/shelepuginivan/carmen/search/pkg/document"
	"gorm.io/gorm"
)

// Workspace represents a collection of related documents.
type Workspace struct {
	gorm.Model

	ID          string `gorm:"primaryKey"`
	Name        string
	Description string

	Documents []document.Document
}

func (w *Workspace) BeforeCreate(tx *gorm.DB) (err error) {
	w.ID = uuid.New().String()
	return
}
