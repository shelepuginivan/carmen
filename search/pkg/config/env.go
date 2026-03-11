package config

import (
	"log"
	"os"
	"strconv"
)

const envPrefix = "CARMEN_SEARCH_"

func requiredEnvStr(env string) string {
	envName := envPrefix + env

	v, ok := os.LookupEnv(envName)
	if !ok {
		log.Fatalf("required env variable '%s' is not set", envName)
	}

	return v
}

func requiredEnvInt(env string) int {
	v, err := strconv.Atoi(requiredEnvStr(env))
	if err != nil {
		log.Fatalf("env variable '%s' is not an int", envPrefix+env)
	}

	return v
}
