package search

import "gorm.io/gorm"

type WorkspaceService struct {
	db *gorm.DB
}

func NewWorkspaceService(db *gorm.DB) *WorkspaceService {
	return &WorkspaceService{
		db: db,
	}
}

func (ws *WorkspaceService) CreateWorkspace(name string, description string) error {
	return ws.db.Create(&Workspace{
		Name:        name,
		Description: description,
	}).Error
}

func (ws *WorkspaceService) GetWorkspace(identifier string) (*Workspace, error) {
	var workspace Workspace
	res := ws.db.Where("id = ?", identifier).Or("name = ?", identifier).First(&workspace)
	return &workspace, res.Error
}

func (ws *WorkspaceService) ListWorkspaces() ([]*Workspace, error) {
	var workspaces []*Workspace
	res := ws.db.Select("name", "description").Order("name").Find(&workspaces)
	return workspaces, res.Error
}

func (ws *WorkspaceService) DeleteWorkspace(identifier string) error {
	return ws.db.
		Unscoped().
		Where("id = ?", identifier).
		Or("name = ?", identifier).
		Delete(&Workspace{}).
		Error
}
