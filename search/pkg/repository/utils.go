package repository

import (
	"errors"

	"github.com/shelepuginivan/carmen/search/pkg/apperror"
	"gorm.io/gorm"
)

func wrapErr(err error) error {
	switch {
	case errors.Is(err, gorm.ErrRecordNotFound):
		return apperror.ErrNotFound
	default:
		return apperror.ErrInternal
	}
}
