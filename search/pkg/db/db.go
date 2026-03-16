// Package db provides database utilities.
package db

import (
	"github.com/shelepuginivan/carmen/search/pkg/config"
	"github.com/shelepuginivan/carmen/search/pkg/search"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

// Connect connects to the database and performs necessary setup.
func Connect(cfg *config.Config) (*gorm.DB, error) {
	db, err := gorm.Open(postgres.Open(cfg.PostgresDSN()), &gorm.Config{})
	if err != nil {
		return nil, err
	}

	if err := db.Exec("CREATE EXTENSION IF NOT EXISTS vector").Error; err != nil {
		return nil, err
	}

	if err := db.AutoMigrate(
		&search.Workspace{},
		&search.Document{},
		&search.Chunk{},
	); err != nil {
		return nil, err
	}

	return db, nil
}
