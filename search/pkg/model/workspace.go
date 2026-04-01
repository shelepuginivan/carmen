package model

import (
	"github.com/google/uuid"
	"gorm.io/gorm"
)

// Workspace represents a collection of related documents.
type Workspace struct {
	ID          string `gorm:"primaryKey"`
	Name        string `gorm:"uniqueIndex"`
	Description string

	Documents []Document `gorm:"constraint:OnDelete:CASCADE"`
}

func (w *Workspace) BeforeCreate(tx *gorm.DB) (err error) {
	w.ID = uuid.New().String()
	return
}
