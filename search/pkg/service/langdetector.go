package service

import (
	"bytes"
	"fmt"
	"io"
	"net/http"
	"time"
)

type LangdetectorService struct {
	client *http.Client
	url    string
}

func NewLangdetector(url string) *LangdetectorService {
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

	return &LangdetectorService{client, url}
}

func (lds *LangdetectorService) DetectLanguage(text string) (string, error) {
	res, err := lds.client.Post(lds.url, "text/plain", bytes.NewBufferString(text))
	if err != nil {
		return "", err
	}
	defer res.Body.Close()

	if res.StatusCode != http.StatusOK {
		return "", fmt.Errorf("cannot detect language")
	}

	lang, err := io.ReadAll(res.Body)
	if err != nil {
		return "", err
	}

	return string(lang), nil
}
