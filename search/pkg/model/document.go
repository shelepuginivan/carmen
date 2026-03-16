package model

import (
	"github.com/google/uuid"
	"gorm.io/gorm"
)

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
