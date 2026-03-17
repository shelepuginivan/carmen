package dto

type WorkspaceCreate struct {
	Name        string `binding:"required" json:"name"`
	Description string `binding:"required" json:"description"`
}
