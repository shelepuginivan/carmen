package main

import (
	"log"

	"github.com/gin-gonic/gin"
	"github.com/shelepuginivan/carmen/search/pkg/config"
	"github.com/shelepuginivan/carmen/search/pkg/controller"
	"github.com/shelepuginivan/carmen/search/pkg/dal"
	"github.com/shelepuginivan/carmen/search/pkg/repository"
	"github.com/shelepuginivan/carmen/search/pkg/service"
)

func main() {
	cfg := config.Load()

	s3 := dal.NewS3(cfg)
	db, err := dal.NewDBConnection(cfg)
	if err != nil {
		log.Fatal(err)
	}

	srv := gin.Default()

	workspaceRepo := repository.NewWorkspace(db, s3)
	workspaceService := service.NewWorkspace(workspaceRepo)
	workspaceController := controller.NewWorkspace(workspaceService)

	workspaces := srv.Group("/workspace")
	workspaces.POST("/", workspaceController.CreateWorkspace)

	if err := srv.Run(cfg.Addr); err != nil {
		log.Fatal(err)
	}
}
