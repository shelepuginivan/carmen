package model

import (
	"github.com/google/uuid"
	"gorm.io/gorm"
)

type Document struct {
	ID          string `gorm:"primaryKey"`
	WorkspaceID string `gorm:"index"`
	Filename    string

	Chunks []Chunk `gorm:"constraint:OnDelete:CASCADE"`
}

func (d *Document) BeforeCreate(tx *gorm.DB) (err error) {
	d.ID = uuid.New().String()
	return
}
