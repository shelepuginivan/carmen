package client

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"time"
)

type EmbeddingResponse struct {
	Language  string    `json:"language"`
	Embedding []float32 `json:"embedding"`
}

type EmbeddingClient struct {
	client *http.Client
	url    string
}

func NewEmbedding(url string) *EmbeddingClient {
	transport := &http.Transport{
		MaxIdleConns:        10,
		MaxIdleConnsPerHost: 10,
		IdleConnTimeout:     30 * time.Second,
		DisableKeepAlives:   false,
	}
	client := &http.Client{
		Transport: transport,
		Timeout:   30 * time.Second,
	}

	return &EmbeddingClient{client, url}
}

func (lds *EmbeddingClient) GenerateEmbedding(query string) (*EmbeddingResponse, error) {
	res, err := lds.client.Post(lds.url, "text/plain", bytes.NewBufferString(query))
	if err != nil {
		return nil, fmt.Errorf("embedding: %w", err)
	}
	defer res.Body.Close()

	payload, err := io.ReadAll(res.Body)
	if err != nil {
		return nil, fmt.Errorf("embedding: %w", err)
	}

	var er EmbeddingResponse

	if err := json.Unmarshal(payload, &er); err != nil {
		return nil, fmt.Errorf("embedding: %w", err)
	}

	return &er, nil
}
