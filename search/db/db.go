// Package db provides database utilities.
package db

import (
	"github.com/shelepuginivan/carmen/search/chunk"
	"github.com/shelepuginivan/carmen/search/document"
	"github.com/shelepuginivan/carmen/search/workspace"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

// Connect connects to the database and performs necessary setup.
func Connect() (*gorm.DB, error) {
	// TODO: configure via env vars
	dsn := "host=localhost user=postgres password=postgres dbname=carmen port=5432"
	db, err := gorm.Open(postgres.Open(dsn), &gorm.Config{})
	if err != nil {
		return nil, err
	}

	if err := db.Exec("CREATE EXTENSION IF NOT EXISTS vector").Error; err != nil {
		return nil, err
	}

	if err := db.AutoMigrate(
		&workspace.Workspace{},
		&document.Document{},
		&chunk.Chunk{},
	); err != nil {
		return nil, err
	}

	return db, nil
}
