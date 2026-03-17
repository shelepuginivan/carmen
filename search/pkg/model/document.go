package model

import (
	"github.com/google/uuid"
	"gorm.io/gorm"
)

type Document struct {
	gorm.Model `json:"-"`

	ID          string `gorm:"primaryKey" json:"id,omitempty"`
	WorkspaceID string
	Filename    string `json:"filename,omitempty"`

	Chunks []Chunk `gorm:"constraint:OnDelete:CASCADE"`
}

func (d *Document) BeforeCreate(tx *gorm.DB) (err error) {
	d.ID = uuid.New().String()
	return
}
