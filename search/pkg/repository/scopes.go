package repository

import "gorm.io/gorm"

type Scope = func(db *gorm.DB) *gorm.DB

func Paginate(page, limit int) func(db *gorm.DB) *gorm.DB {
	page = max(page, 1)
	limit = min(max(limit, 10), 100)

	offset := (page - 1) * limit

	return func(db *gorm.DB) *gorm.DB {
		return db.Offset(offset).Limit(limit)
	}
}
