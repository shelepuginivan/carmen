package controller

import "github.com/gin-gonic/gin"

func respondWithError(c *gin.Context, status int, err error) {
	c.JSON(status, gin.H{
		"ok":     false,
		"detail": err.Error(),
	})
}
