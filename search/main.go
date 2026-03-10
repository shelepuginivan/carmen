package main

import (
	"fmt"
	"log"

	"github.com/shelepuginivan/carmen/search/db"
)

func main() {
	db, err := db.Connect()
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println(db)
}
