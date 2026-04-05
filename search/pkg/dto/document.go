package dto

import "mime/multipart"

type DocumentMetadata struct {
	ID       string `json:"id"`
	Filename string `json:"filename"`
}

type DocumentUpload struct {
	File *multipart.FileHeader `form:"file" binding:"required"`
}

type DocumentUploadMany struct {
	Files []*multipart.FileHeader `form:"files" binding:"required"`
}

type DocumentUploadManyResult struct {
	Ok     bool     `json:"ok"`
	Failed []string `json:"failed,omitempty"`
}
