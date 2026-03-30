package service

import (
	"context"

	"github.com/shelepuginivan/carmen/search/pkg/adapter"
	"github.com/shelepuginivan/carmen/search/pkg/model"
	"github.com/shelepuginivan/carmen/search/pkg/repository"
)

type SearchService struct {
	cr  *repository.ChunksRepository
	ssa *adapter.SemanticSearchAdapter
	lds *LangdetectorService
}

func NewSearch(
	cr *repository.ChunksRepository,
	ssa *adapter.SemanticSearchAdapter,
	lds *LangdetectorService,
) *SearchService {
	return &SearchService{cr, ssa, lds}
}

func (ss *SearchService) FullTextSearch(
	ctx context.Context,
	workspaceID string,
	query string,
	limit int,
) ([]*model.Chunk, error) {
	lang, err := ss.lds.DetectLanguage(query)
	if err != nil {
		return nil, err
	}

	return ss.cr.FullTextSearch(ctx, workspaceID, query, lang, limit)
}

func (ss *SearchService) SemanticSearch(
	ctx context.Context,
	workspaceID string,
	query string,
	limit int,
) ([]*model.Chunk, error) {
	resCh, err := ss.ssa.Query(ctx, query)
	if err != nil {
		return nil, err
	}

	res := <-resCh

	return ss.cr.SemanticSearch(ctx, workspaceID, res.Embedding, limit)
}

func (ss *SearchService) SimilaritySearch(
	ctx context.Context,
	workspaceID string,
	query string,
	limit int,
) ([]*model.Chunk, error) {
	return ss.cr.SimilaritySearch(ctx, workspaceID, query, limit)
}
