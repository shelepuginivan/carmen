package repository

import (
	"github.com/pgvector/pgvector-go"
	"gorm.io/gorm"
	"gorm.io/gorm/clause"
)

type Scope = func(db *gorm.DB) *gorm.DB

func Paginate(page, limit int) Scope {
	page = max(page, 1)
	limit = min(max(limit, 10), 100)

	offset := (page - 1) * limit

	return func(db *gorm.DB) *gorm.DB {
		return db.Offset(offset).Limit(limit)
	}
}

func VectorSearch(field string, vec []float32) Scope {
	return func(db *gorm.DB) *gorm.DB {
		return db.Clauses(clause.OrderBy{
			Expression: clause.Expr{
				SQL: "embedding <=> ?",
				Vars: []any{
					pgvector.NewVector(vec),
				},
			},
		})
	}
}
