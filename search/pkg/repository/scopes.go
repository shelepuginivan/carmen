package repository

import (
	"fmt"

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

func FullTextSearch(field string, query, language string) Scope {
	return func(db *gorm.DB) *gorm.DB {
		return db.Clauses(clause.OrderBy{
			Expression: clause.Expr{
				SQL:  fmt.Sprintf("ts_rank(%s, websearch_to_query(?, ?)) DESC", field),
				Vars: []any{language, query},
			},
		})
	}
}

func VectorSearch(field string, vec []float32) Scope {
	return func(db *gorm.DB) *gorm.DB {
		return db.Clauses(clause.OrderBy{
			Expression: clause.Expr{
				SQL: field + " <=> ?",
				Vars: []any{
					pgvector.NewVector(vec),
				},
			},
		})
	}
}

func SimilaritySearch(field string, query string) Scope {
	return func(db *gorm.DB) *gorm.DB {
		return db.Clauses(clause.OrderBy{
			Expression: clause.Expr{
				SQL:  fmt.Sprintf("similarity(%s, ?) DESC", field),
				Vars: []any{query},
			},
		})
	}
}
