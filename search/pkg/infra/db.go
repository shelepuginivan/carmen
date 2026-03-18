// Package infra provides utilities for communicating with infrastructure.
package infra

import (
	"fmt"
	"time"

	"github.com/shelepuginivan/carmen/search/pkg/config"
	"github.com/shelepuginivan/carmen/search/pkg/model"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

func connectWithBackoff(
	cfg *config.Config,
	initDelay time.Duration,
	delayScaleFactor int,
	retries int,
) (*gorm.DB, error) {
	var (
		dsn    = cfg.PostgresDSN()
		delay  = initDelay
		factor = time.Duration(delayScaleFactor)

		db  *gorm.DB
		err error
	)

	for range retries {
		db, err = gorm.Open(postgres.Open(dsn))
		if err == nil {
			return db, nil
		}

		time.Sleep(delay)
		delay *= factor
	}

	return nil, fmt.Errorf("db: connection failed after %d retries: %w", retries, err)
}

// NewDBConnection connects to the database and performs necessary setup.
func NewDBConnection(cfg *config.Config) (*gorm.DB, error) {
	db, err := connectWithBackoff(cfg, time.Second, 2, 5)
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
