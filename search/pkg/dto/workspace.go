package dto

type WorkspaceCreate struct {
	Name        string `binding:"required" json:"name"`
	Description string `binding:"required" json:"description"`
}

type WorkspaceGet struct {
	ID          string `json:"id"`
	Name        string `json:"name"`
	Description string `json:"description"`
}
