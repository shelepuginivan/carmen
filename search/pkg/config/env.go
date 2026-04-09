package config

import (
	"fmt"
	"log"
	"os"
	"strconv"
)

const envPrefix = "CARMEN_SEARCH_"

func envString(env string) (string, error) {
	varName := envPrefix + env
	v, ok := os.LookupEnv(varName)
	if !ok {
		return "", fmt.Errorf("env variable '%s' is not set", varName)
	}
	return v, nil
}

func envInt(env string) (int, error) {
	varName := envPrefix + env
	v, ok := os.LookupEnv(varName)
	if !ok {
		return 0, fmt.Errorf("env variable '%s' is not set", varName)
	}

	i, err := strconv.Atoi(v)
	if err != nil {
		return 0, fmt.Errorf("env variable '%s' is not a valid integer", varName)
	}

	return i, nil
}

func defaultEnvStr(env string, fallback string) string {
	v, err := envString(env)
	if err != nil {
		return fallback
	}
	return v
}

func defaultEnvInt(env string, fallback int) int {
	v, err := envInt(env)
	if err != nil {
		return fallback
	}
	return v
}

func requiredEnvStr(env string) string {
	v, err := envString(env)
	if err != nil {
		log.Fatal(err)
	}
	return v
}

func requiredEnvInt(env string) int {
	v, err := envInt(env)
	if err != nil {
		log.Fatal(err)
	}
	return v
}
