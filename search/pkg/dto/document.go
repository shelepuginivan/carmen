package dto

import "mime/multipart"

type DocumentMetadata struct {
	ID       string `json:"id"`
	Filename string `json:"filename"`
}

type DocumentUpload struct {
	File *multipart.FileHeader `form:"file" binding:"required"`
}
