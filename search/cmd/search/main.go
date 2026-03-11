package main

import (
	"fmt"
	"log"

	"github.com/shelepuginivan/carmen/search/pkg/config"
	"github.com/shelepuginivan/carmen/search/pkg/db"
)

func main() {
	cfg := config.Load()

	db, err := db.Connect(cfg)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println(db)
}
