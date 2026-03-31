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
	language string,
) (*model.Chunk, error) {
	chunk := model.Chunk{
		DocumentID: documentID,
		Text:       text,
		Language:   language,
		Embedding:  pgvector.NewVector(embedding),
	}

	err := cr.db.WithContext(ctx).Create(&chunk).Error
	if err != nil {
		return nil, err
	}

	return &chunk, nil
}

func (cr *ChunksRepository) FullTextSearch(
	ctx context.Context,
	workspaceID string,
	query string,
	queryLang string,
	limit int,
) ([]*model.Chunk, error) {
	var chunks []*model.Chunk

	err := cr.db.
		WithContext(ctx).
		Limit(limit).
		Select(
			"chunks.id, chunks.document_id, chunks.text, chunks.fts_vector, ts_rank(chunks.fts_vector, websearch_to_tsquery(?, ?)) AS relevance",
			queryLang,
			query,
		).
		Joins("JOIN documents ON documents.id = chunks.document_id").
		Where("documents.workspace_id = ?", workspaceID).
		Order("relevance DESC").
		Find(&chunks).
		Error

	return chunks, err
}

func (cr *ChunksRepository) SemanticSearch(
	ctx context.Context,
	workspaceID string,
	vec []float32,
	limit int,
) ([]*model.Chunk, error) {
	var chunks []*model.Chunk

	err := cr.db.
		WithContext(ctx).
		Limit(limit).
		Select(
			"chunks.id, chunks.document_id, chunks.text, 1 - (chunks.embedding <=> ?) AS relevance",
			pgvector.NewVector(vec),
		).
		Joins("JOIN documents ON documents.id = chunks.document_id").
		Where("documents.workspace_id = ?", workspaceID).
		Order("relevance DESC").
		Find(&chunks).
		Error

	return chunks, err
}

func (cr *ChunksRepository) SimilaritySearch(
	ctx context.Context,
	workspaceID string,
	query string,
	limit int,
) ([]*model.Chunk, error) {
	var chunks []*model.Chunk

	err := cr.db.
		WithContext(ctx).
		Limit(limit).
		Select(
			"chunks.id, chunks.document_id, chunks.text, word_similarity(?, chunks.text) AS relevance",
			query,
		).
		Joins("JOIN documents ON documents.id = chunks.document_id").
		Where("documents.workspace_id = ?", workspaceID).
		Order("relevance DESC").
		Find(&chunks).
		Error

	return chunks, err
}
