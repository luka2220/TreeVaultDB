package server

// HTTP server
// JSON serialization
// POST request handlers

import (
	"fmt"
	"log"
	"net/http"
)

// Handles the creation of a new DB Server
// Will hold some type of authentication in the future
type DBServer struct {
	port     string // port number of the server
	endpoint string // endpoint name
}

func (s DBServer) Start() {
	fmt.Println("Starting server...")

	dbHandler := handleDBPost

	http.HandleFunc(s.endpoint, dbHandler)
	log.Fatal(http.ListenAndServe(s.port, nil))
}

func NewDBServer(port string, endpoint string) *DBServer {
	port = fmt.Sprintf(":%s", port)
	endpoint = fmt.Sprintf("/%s", endpoint)

	return &DBServer{
		port,
		endpoint,
	}
}

func handleDBPost(w http.ResponseWriter, req *http.Request) {
	// TODO: Process the request based on the DB call
	// Read the db operation from X-Amz-Target i.e TreeVaultDB.GetItem
	// Parse the request body based on table data and db operation specific data

	var err = validateRequestMetaData(req)
	if err != nil {
		w.Write([]byte(err.Error()))
		w.WriteHeader(err.ResponseCode)
		req.Close = true
		return
	}

	w.Write([]byte("DB Running"))
	w.WriteHeader(http.StatusOK)
}

// DB Server error with message and response code
// Implements the error interface
type dbServerError struct {
	Message      string
	ResponseCode int
}

func (d dbServerError) Error() string {
	return d.Message
}

// Validates the requests method, headers
// Returns a dbServer error that implements the Error interface
func validateRequestMetaData(req *http.Request) *dbServerError {
	if req.Method != "POST" {
		return &dbServerError{
			Message:      "This route accepts only POST requests",
			ResponseCode: http.StatusMethodNotAllowed,
		}
	}

	dbOperation := req.Header.Get("X-Amz-Target")
	if dbOperation == "" {
		return &dbServerError{
			Message:      "Invalid DB operation on header X-Amz-Target",
			ResponseCode: http.StatusBadRequest,
		}
	}

	return nil
}
