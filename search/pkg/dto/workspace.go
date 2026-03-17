package dto

type WorkspaceCreate struct {
	Name        string `binding:"required" json:"name" form:"name"`
	Description string `binding:"required" json:"description" form:"description"`
}
