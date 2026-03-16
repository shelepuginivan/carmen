// Package dal provides data access layer utilities.
package dal

import (
	"github.com/shelepuginivan/carmen/search/pkg/config"
	"github.com/shelepuginivan/carmen/search/pkg/model"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

// NewDBConnection connects to the database and performs necessary setup.
func NewDBConnection(cfg *config.Config) (*gorm.DB, error) {
	db, err := gorm.Open(postgres.Open(cfg.PostgresDSN()), &gorm.Config{})
	if err != nil {
		return nil, err
	}

	if err := db.Exec("CREATE EXTENSION IF NOT EXISTS vector").Error; err != nil {
		return nil, err
	}

	if err := db.AutoMigrate(
		&model.Workspace{},
		&model.Document{},
		&model.Chunk{},
	); err != nil {
		return nil, err
	}

	return db, nil
}
