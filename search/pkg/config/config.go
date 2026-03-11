// Package config provides global service configuration.
package config

import "fmt"

// Config represents service configuration.
type Config struct {
	PostgresUser     string
	PostgresPassword string
	PostgresDatabase string
	PostgresHost     string
	PostgresPort     int
}

// Load loads configuration from env variables.
func Load() *Config {
	return &Config{
		PostgresUser:     requiredEnvStr("POSTGRES_USER"),
		PostgresPassword: requiredEnvStr("POSTGRES_PASSWORD"),
		PostgresDatabase: requiredEnvStr("POSTGRES_DB"),
		PostgresHost:     requiredEnvStr("POSTGRES_HOST"),
		PostgresPort:     requiredEnvInt("POSTGRES_PORT"),
	}
}

// PostgresDSN returns DSN for PostgreSQL database connection.
func (c *Config) PostgresDSN() string {
	return fmt.Sprintf(
		"host=%s port=%d user=%s password=%s dbname=%s",
		c.PostgresHost,
		c.PostgresPort,
		c.PostgresUser,
		c.PostgresPassword,
		c.PostgresDatabase,
	)
}
