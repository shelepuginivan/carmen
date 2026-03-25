package repository

import (
	"context"
	"strings"

	"github.com/pemistahl/lingua-go"
	"github.com/pgvector/pgvector-go"
	"github.com/shelepuginivan/carmen/search/pkg/model"
	"gorm.io/gorm"
)

type ChunksRepository struct {
	db *gorm.DB
	ld lingua.LanguageDetector
}

func NewChunk(db *gorm.DB) *ChunksRepository {
	// TODO: support more languages via configuration.
	ld := lingua.NewLanguageDetectorBuilder().
		FromLanguages(
			lingua.English,
			lingua.Chinese,
			lingua.Russian,
		).
		WithLowAccuracyMode().
		Build()

	return &ChunksRepository{db, ld}
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
		Language:   cr.detectLanguage(text),
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
	limit int,
) ([]*model.Chunk, error) {
	var chunks []*model.Chunk

	err := cr.db.
		WithContext(ctx).
		Scopes(FullTextSearch("fts_vector", query)).
		Limit(limit).
		Select("chunks.id, chunks.document_id, chunks.text").
		Joins("JOIN documents ON documents.id = chunks.document_id").
		Where("documents.workspace_id = ?", workspaceID).
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
		Scopes(VectorSearch("embedding", vec)).
		Limit(limit).
		Select("chunks.id, chunks.document_id, chunks.text").
		Joins("JOIN documents ON documents.id = chunks.document_id").
		Where("documents.workspace_id = ?", workspaceID).
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
		Scopes(SimilaritySearch("text", query)).
		Limit(limit).
		Select("chunks.id, chunks.document_id, chunks.text").
		Joins("JOIN documents ON documents.id = chunks.document_id").
		Where("documents.workspace_id = ?", workspaceID).
		Find(&chunks).
		Error

	return chunks, err
}

func (cr *ChunksRepository) detectLanguage(text string) string {
	lang, ok := cr.ld.DetectLanguageOf(text)
	if !ok {
		return "simple"
	}

	return strings.ToLower(lang.String())
}
