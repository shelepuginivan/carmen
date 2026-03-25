package adapter

import (
	"context"
	"encoding/json"
	"log"
	"sync"

	"github.com/google/uuid"
	"github.com/segmentio/kafka-go"
	"github.com/shelepuginivan/carmen/search/pkg/config"
	"github.com/shelepuginivan/carmen/search/pkg/dto"
)

type SemanticSearchAdapter struct {
	r  *kafka.Reader
	w  *kafka.Writer
	mu sync.Mutex
	ch map[string]chan<- *dto.SemanticSearchResponse
}

func NewSemanticSearch(cfg *config.Kafka) *SemanticSearchAdapter {
	r := kafka.NewReader(kafka.ReaderConfig{
		Brokers: []string{cfg.URI},
		Topic:   cfg.TopicSearchResponses,
	})

	w := &kafka.Writer{
		Addr:                   kafka.TCP(cfg.URI),
		Topic:                  cfg.TopicSearchRequests,
		Balancer:               &kafka.LeastBytes{},
		AllowAutoTopicCreation: true,
	}

	ch := make(map[string]chan<- *dto.SemanticSearchResponse, 1)

	return &SemanticSearchAdapter{
		r:  r,
		w:  w,
		ch: ch,
	}
}

func (ssa *SemanticSearchAdapter) Query(ctx context.Context, query string) (<-chan *dto.SemanticSearchResponse, error) {
	key := uuid.New().String()

	req := map[string]string{
		"query":          query,
		"response_topic": ssa.r.Config().Topic,
	}

	payload, err := json.Marshal(req)
	if err != nil {
		return nil, err
	}

	msg := kafka.Message{
		Key:   []byte(key),
		Value: payload,
	}

	res := make(chan *dto.SemanticSearchResponse)

	ssa.mu.Lock()
	ssa.ch[key] = res
	ssa.mu.Unlock()

	err = ssa.w.WriteMessages(ctx, msg)
	if err != nil {
		close(res)
		ssa.mu.Lock()
		delete(ssa.ch, key)
		ssa.mu.Unlock()
		return nil, err
	}
	return res, nil
}

func (ssa *SemanticSearchAdapter) Handle(ctx context.Context) {
	for {
		response, key, err := ssa.ReadResponse(ctx)
		if err != nil {
			log.Println(err)
			break
		}

		ssa.mu.Lock()

		res, ok := ssa.ch[key]
		if ok {
			res <- response
			close(res)
		} else {
			log.Printf("warning: unknown response key %s", key)
		}

		delete(ssa.ch, key)
		ssa.mu.Unlock()
	}
}

func (ssa *SemanticSearchAdapter) ReadResponse(ctx context.Context) (*dto.SemanticSearchResponse, string, error) {
	msg, err := ssa.r.ReadMessage(ctx)
	if err != nil {
		return nil, "", err
	}

	var res dto.SemanticSearchResponse

	if err := json.Unmarshal(msg.Value, &res); err != nil {
		return nil, "", err
	}

	return &res, string(msg.Key), nil
}

func (ssa *SemanticSearchAdapter) Close() error {
	ssa.mu.Lock()
	defer ssa.mu.Unlock()

	for key, ch := range ssa.ch {
		close(ch)
		delete(ssa.ch, key)
	}

	err := ssa.r.Close()
	if err != nil {
		return err
	}

	return ssa.w.Close()
}
