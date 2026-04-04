package controller

import (
	"errors"
	"log"
	"net/http"
	"time"

	"github.com/gin-gonic/gin"
	"github.com/go-playground/validator/v10"
	"github.com/shelepuginivan/carmen/search/pkg/apperror"
)

func respondWithError(c *gin.Context, err error) {
	var (
		status    = http.StatusInternalServerError
		timestamp = time.Now()
		detail    = err.Error()
	)

	log.Println(err)

	switch {
	case errors.Is(err, &validator.ValidationErrors{}):
		status = http.StatusBadRequest
	case errors.Is(err, apperror.ErrNotFound):
		status = http.StatusNotFound
	case errors.Is(err, apperror.ErrInternal):
		status = http.StatusInternalServerError
	default:
		status = http.StatusInternalServerError
		detail = "unknown error"
	}

	c.JSON(status, gin.H{
		"detail":    detail,
		"timestamp": timestamp,
	})
}
