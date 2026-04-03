// Package apperror provides common errors.
package apperror

import "errors"

var (
	ErrNotFound = errors.New("resource not found")
	ErrInternal = errors.New("operation failed")
)
