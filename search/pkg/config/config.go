// Package config provides global service configuration.
package config

import "fmt"

type Server struct {
	Addr string
}

type Postgres struct {
	User     string
	Password string
	Database string
	Host     string
	Port     int
}

type S3 struct {
	AccessKey string
	SecretKey string
	Region    string
	Endpoint  string
	Bucket    string
}

// DSN returns DSN for PostgreSQL database connection.
func (p *Postgres) DSN() string {
	return fmt.Sprintf(
		"host=%s port=%d user=%s password=%s dbname=%s",
		p.Host,
		p.Port,
		p.User,
		p.Password,
		p.Database,
	)
}

// Config represents service configuration.
type Config struct {
	Server   *Server
	Postgres *Postgres
	S3       *S3
}

// Load loads configuration from env variables.
func Load() *Config {
	return &Config{
		Server: &Server{
			Addr: requiredEnvStr("ADDR"),
		},

		Postgres: &Postgres{
			User:     requiredEnvStr("POSTGRES_USER"),
			Password: requiredEnvStr("POSTGRES_PASSWORD"),
			Database: requiredEnvStr("POSTGRES_DB"),
			Host:     requiredEnvStr("POSTGRES_HOST"),
			Port:     requiredEnvInt("POSTGRES_PORT"),
		},

		S3: &S3{
			AccessKey: requiredEnvStr("S3_ACCESS_KEY"),
			SecretKey: requiredEnvStr("S3_SECRET_KEY"),
			Region:    requiredEnvStr("S3_REGION"),
			Endpoint:  requiredEnvStr("S3_ENDPOINT"),
			Bucket:    requiredEnvStr("S3_BUCKET"),
		},
	}
}
