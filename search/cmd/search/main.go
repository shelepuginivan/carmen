package main

import (
	"context"
	"log"

	"github.com/gin-gonic/gin"
	"github.com/shelepuginivan/carmen/search/pkg/adapter"
	"github.com/shelepuginivan/carmen/search/pkg/client"
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

	s3 := infra.NewS3(cfg.S3)
	db, err := infra.NewDBConnection(cfg.Postgres)
	if err != nil {
		log.Fatal(err)
	}

	ep := infra.NewExtractorProducer(cfg.Kafka)
	defer ep.Close()

	srv := gin.Default()

	workspaceRepo := repository.NewWorkspace(db, s3)
	documentsRepo := repository.NewDocument(db, s3)
	workspaceService := service.NewWorkspace(workspaceRepo, documentsRepo, ep)
	workspaceController := controller.NewWorkspace(workspaceService)

	workspaces := srv.Group("/workspace")
	workspaces.POST("/", workspaceController.CreateWorkspace)
	workspaces.GET("/:id-or-name", workspaceController.GetWorkspace)
	workspaces.GET("/:id-or-name/document/all", workspaceController.GetWorkspaceDocuments)
	workspaces.GET("/:id-or-name/document/page/:page", workspaceController.PaginateWorkspaceDocuments)
	workspaces.POST("/:id-or-name/document", workspaceController.UploadDocument)
	workspaces.GET("/all", workspaceController.ListWorkspaces)
	workspaces.GET("/all/page/:page", workspaceController.PaginateWorkspaces)
	workspaces.DELETE("/:id-or-name", workspaceController.DeleteWorkspace)

	documentsService := service.NewDocument(documentsRepo)
	documentController := controller.NewDocument(documentsService)

	documents := srv.Group("/document")
	documents.GET("/:id", documentController.GetDocumentMetadata)
	documents.GET("/:id/content", documentController.GetDocumentContents)
	documents.DELETE("/:id", documentController.DeleteDocument)

	chunksRepo := repository.NewChunk(db)
	langdetectorClient := client.NewLangdetector(cfg.Service.Langdetector)
	embeddingClient := client.NewEmbedding(cfg.Service.Embedding)
	searchService := service.NewSearch(chunksRepo, embeddingClient, langdetectorClient)
	searchController := controller.NewSearch(searchService)

	search := srv.Group("/search")
	search.GET("/fulltext", searchController.FullTextSearch)
	search.GET("/hybrid", searchController.HybridSearch)
	search.GET("/semantic", searchController.SemanticSearch)
	search.GET("/similarity", searchController.SimilaritySearch)
	search.GET("/fulltext/docs", searchController.FullTextSearchDocuments)
	search.GET("/semantic/docs", searchController.SemanticSearchDocuments)
	search.GET("/similarity/docs", searchController.SimilaritySearchDocuments)
	search.GET("/hybrid/docs", searchController.HybridSearchDocuments)

	chunksAdapter := adapter.NewChunk(cfg.Kafka, chunksRepo)

	go func() {
		chunksAdapter.Handle(context.Background())
	}()
	defer chunksAdapter.Close()

	srv.GET("/swagger/*any", ginSwagger.WrapHandler(swaggerfiles.Handler))

	if err := srv.Run(cfg.Server.Addr); err != nil {
		log.Fatal(err)
	}
}
