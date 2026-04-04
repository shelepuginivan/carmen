package service

import (
	"context"

	"github.com/shelepuginivan/carmen/search/pkg/apperror"
	"github.com/shelepuginivan/carmen/search/pkg/client"
	"github.com/shelepuginivan/carmen/search/pkg/model"
	"github.com/shelepuginivan/carmen/search/pkg/repository"
)

type SearchService struct {
	cr *repository.ChunksRepository
	ec *client.EmbeddingClient
	lc *client.LangdetectorClient
}

func NewSearch(
	cr *repository.ChunksRepository,
	ec *client.EmbeddingClient,
	lc *client.LangdetectorClient,
) *SearchService {
	return &SearchService{cr, ec, lc}
}

func (ss *SearchService) FullTextSearch(
	ctx context.Context,
	workspaceID string,
	query string,
	limit int,
	threshold float64,
) ([]*model.Chunk, error) {
	lang, err := ss.lc.DetectLanguage(query)
	if err != nil {
		return nil, err
	}

	return ss.cr.FullTextSearch(ctx, workspaceID, query, lang, limit, threshold)
}

func (ss *SearchService) SemanticSearch(
	ctx context.Context,
	workspaceID string,
	query string,
	limit int,
	threshold float64,
) ([]*model.Chunk, error) {
	res, err := ss.ec.GenerateEmbedding(query)
	if err != nil {
		return nil, apperror.ErrInternal
	}

	return ss.cr.SemanticSearch(ctx, workspaceID, res.Embedding, limit, threshold)
}

func (ss *SearchService) SimilaritySearch(
	ctx context.Context,
	workspaceID string,
	query string,
	limit int,
	threshold float64,
) ([]*model.Chunk, error) {
	return ss.cr.SimilaritySearch(ctx, workspaceID, query, limit, threshold)
}

func (ss *SearchService) FullTextSearchDocuments(
	ctx context.Context,
	workspaceID string,
	query string,
	limit int,
	threshold float64,
) ([]string, error) {
	lang, err := ss.lc.DetectLanguage(query)
	if err != nil {
		return nil, err
	}

	return ss.cr.FullTextSearchDocuments(ctx, workspaceID, query, lang, limit, threshold)
}

func (ss *SearchService) SemanticSearchDocuments(
	ctx context.Context,
	workspaceID string,
	query string,
	limit int,
	threshold float64,
) ([]string, error) {
	res, err := ss.ec.GenerateEmbedding(query)
	if err != nil {
		return nil, apperror.ErrInternal
	}

	return ss.cr.SemanticSearchDocuments(ctx, workspaceID, res.Embedding, limit, threshold)
}

func (ss *SearchService) SimilaritySearchDocuments(
	ctx context.Context,
	workspaceID string,
	query string,
	limit int,
	threshold float64,
) ([]string, error) {
	return ss.cr.SimilaritySearchDocuments(ctx, workspaceID, query, limit, threshold)
}
