package controller

import (
	"fmt"
	"strconv"

	"github.com/gin-gonic/gin"
)

type Pagination struct {
	Page  int
	Limit int
}

func paginate(c *gin.Context) (*Pagination, error) {
	page, err := strconv.Atoi(c.Param("page"))
	if err != nil {
		return nil, fmt.Errorf("page should be a positive integer")
	}

	query, err := strconv.Atoi(c.DefaultQuery("limit", "10"))
	if err != nil {
		return nil, fmt.Errorf("query should be an integer between 10 and 100")
	}

	return &Pagination{page, query}, nil
}
