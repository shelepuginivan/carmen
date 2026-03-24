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
}

func NewSearch(cr *repository.ChunksRepository, ssa *adapter.SemanticSearchAdapter) *SearchService {
	return &SearchService{cr, ssa}
}

func (ss *SearchService) SemanticSearch(ctx context.Context, query string, limit int) ([]*model.Chunk, error) {
	resCh, err := ss.ssa.Query(ctx, query)
	if err != nil {
		return nil, err
	}

	res := <-resCh

	return ss.cr.SemanticSearch(ctx, res.Embedding, limit)
}
