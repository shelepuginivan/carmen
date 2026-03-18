package main

import (
	"log"

	"github.com/gin-gonic/gin"
	"github.com/shelepuginivan/carmen/search/pkg/config"
	"github.com/shelepuginivan/carmen/search/pkg/controller"
	"github.com/shelepuginivan/carmen/search/pkg/infra"
	"github.com/shelepuginivan/carmen/search/pkg/repository"
	"github.com/shelepuginivan/carmen/search/pkg/service"

	_ "github.com/shelepuginivan/carmen/search/swagger"
	swaggerfiles "github.com/swaggo/files"
	ginSwagger "github.com/swaggo/gin-swagger"
)

func main() {
	cfg := config.Load()

	s3 := infra.NewS3(cfg)
	db, err := infra.NewDBConnection(cfg)
	if err != nil {
		log.Fatal(err)
	}

	srv := gin.Default()

	workspaceRepo := repository.NewWorkspace(db, s3)
	documentsRepo := repository.NewDocument(db, s3)
	workspaceService := service.NewWorkspace(workspaceRepo, documentsRepo)
	workspaceController := controller.NewWorkspace(workspaceService)

	workspaces := srv.Group("/workspace")
	workspaces.POST("/", workspaceController.CreateWorkspace)
	workspaces.GET("/:id-or-name", workspaceController.GetWorkspace)
	workspaces.GET("/:id-or-name/document/all", workspaceController.GetWorkspaceDocuments)
	workspaces.POST("/:id-or-name/document", workspaceController.UploadDocument)
	workspaces.GET("/all", workspaceController.ListWorkspaces)
	workspaces.GET("/all/page/:page", workspaceController.PaginateWorkspaces)
	workspaces.DELETE("/:id-or-name", workspaceController.DeleteWorkspace)

	srv.GET("/swagger/*any", ginSwagger.WrapHandler(swaggerfiles.Handler))

	if err := srv.Run(cfg.Addr); err != nil {
		log.Fatal(err)
	}
}
