package main

import (
	"github.com/luka2220/TreeVaultDB/internal/server"
)

func main() {
 	dbServer := server.NewDBServer("8080", "data")
	dbServer.Start()
}
