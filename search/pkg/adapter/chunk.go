package adapter

import (
	"context"
	"encoding/json"
	"log"

	"github.com/segmentio/kafka-go"
	"github.com/shelepuginivan/carmen/search/pkg/config"
	"github.com/shelepuginivan/carmen/search/pkg/dto"
	"github.com/shelepuginivan/carmen/search/pkg/repository"
)

type ChunkAdapter struct {
	r  *kafka.Reader
	cr *repository.ChunksRepository
}

func NewChunk(cfg *config.Kafka, cr *repository.ChunksRepository) *ChunkAdapter {
	r := kafka.NewReader(kafka.ReaderConfig{
		Brokers: []string{cfg.URI},
		Topic:   cfg.TopicChunksReady,
		GroupID: cfg.ConsumerGroup,
	})

	return &ChunkAdapter{r, cr}
}

func (cc *ChunkAdapter) Handle(ctx context.Context) {
	for {
		chunk, err := cc.ReadReadyChunk(ctx)
		if err != nil {
			log.Println(err)
			break
		}

		if _, err = cc.cr.Create(ctx, chunk.DocumentID, chunk.Text, chunk.Embedding); err != nil {
			log.Println(err)
		}
	}
}

func (cc *ChunkAdapter) ReadReadyChunk(ctx context.Context) (*dto.ChunkReady, error) {
	msg, err := cc.r.ReadMessage(ctx)
	if err != nil {
		return nil, err
	}

	var chunk dto.ChunkReady

	if err := json.Unmarshal(msg.Value, &chunk); err != nil {
		return nil, err
	}

	return &chunk, nil
}

func (cc *ChunkAdapter) Close() error {
	return cc.r.Close()
}
