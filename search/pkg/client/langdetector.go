package client

import (
	"bytes"
	"fmt"
	"io"
	"net/http"
	"time"
)

type LangdetectorClient struct {
	client *http.Client
	url    string
}

func NewLangdetector(url string) *LangdetectorClient {
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

	return &LangdetectorClient{client, url}
}

func (lds *LangdetectorClient) DetectLanguage(text string) (string, error) {
	res, err := lds.client.Post(lds.url, "text/plain", bytes.NewBufferString(text))
	if err != nil {
		return "", fmt.Errorf("langdetect: %w", err)
	}
	defer res.Body.Close()

	if res.StatusCode != http.StatusOK {
		return "", fmt.Errorf("langdetect: cannot detect language reliably")
	}

	lang, err := io.ReadAll(res.Body)
	if err != nil {
		return "", fmt.Errorf("langdetect: %w", err)
	}

	return string(lang), nil
}
