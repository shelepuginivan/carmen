package infra

import (
	"context"
	"encoding/json"

	"github.com/segmentio/kafka-go"
	"github.com/shelepuginivan/carmen/search/pkg/config"
)

type ExtractorProducer struct {
	w *kafka.Writer
}

func NewExtractorProducer(cfg *config.Kafka) *ExtractorProducer {
	w := &kafka.Writer{
		Addr:                   kafka.TCP(cfg.URI),
		Topic:                  cfg.TopicDocumentsQueue,
		Balancer:               &kafka.LeastBytes{},
		AllowAutoTopicCreation: true,
	}

	return &ExtractorProducer{w}
}

func (ep *ExtractorProducer) EnqueueDocumentForExtraction(
	ctx context.Context,
	id string,
	filename string,
	mimetype string,
) error {
	payload := map[string]string{
		"document_id": id,
		"object_key":  filename,
		"mimetype":    mimetype,
	}

	body, err := json.Marshal(payload)
	if err != nil {
		return err
	}

	msg := kafka.Message{Value: body}

	return ep.w.WriteMessages(ctx, msg)
}

func (ep *ExtractorProducer) Close() error {
	return ep.w.Close()
}
