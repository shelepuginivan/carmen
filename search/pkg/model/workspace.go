package model

import (
	"github.com/google/uuid"
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
