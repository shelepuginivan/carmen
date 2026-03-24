package repository

import (
	"context"

	"github.com/pgvector/pgvector-go"
	"github.com/shelepuginivan/carmen/search/pkg/model"
	"gorm.io/gorm"
)

type ChunksRepository struct {
	db *gorm.DB
}

func NewChunk(db *gorm.DB) *ChunksRepository {
	return &ChunksRepository{db}
}

func (cr *ChunksRepository) Create(
	ctx context.Context,
	documentID string,
	text string,
	embedding []float32,
) (*model.Chunk, error) {
	chunk := model.Chunk{
		DocumentID: documentID,
		Text:       text,
		Embedding:  pgvector.NewVector(embedding),
	}

	err := cr.db.WithContext(ctx).Create(&chunk).Error
	if err != nil {
		return nil, err
	}

	return &chunk, nil
}

func (cr *ChunksRepository) SemanticSearch(ctx context.Context, vec []float32, limit int) ([]*model.Chunk, error) {
	var chunks []*model.Chunk

	err := cr.db.
		WithContext(ctx).
		Scopes(VectorSearch("embedding", vec)).
		Limit(limit).
		Select("id", "document_id", "text").
		Find(&chunks).
		Error

	return chunks, err
}
