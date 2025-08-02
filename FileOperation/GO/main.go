package main

import (
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
)

type Endpoint struct {
	Url string `json:"url"`
}

func readEndpointConfig(filePath string) (Endpoint, error) {
	var endpoint Endpoint
	content, err := os.ReadFile(filePath)
	if err != nil {
		return endpoint, fmt.Errorf("error reading file %s: %w", filePath, err)
	}
	if err := json.Unmarshal(content, &endpoint); err != nil {
		return endpoint, fmt.Errorf("error parsing JSON: %w", err)
	}
	return endpoint, nil
}

func fetchAndSaveResponse(url, outputFile string) error {
	response, err := http.Get(url)
	if err != nil {
		return fmt.Errorf("error making HTTP request: %w", err)
	}
	defer response.Body.Close()

	file, err := os.Create(outputFile)
	if err != nil {
		return fmt.Errorf("error creating file: %w", err)
	}
	defer file.Close()

	_, err = io.Copy(file, response.Body)
	if err != nil {
		return fmt.Errorf("error writing response to file: %w", err)
	}
	return nil
}

func main() {
	endpoint, err := readEndpointConfig("../endpoint.json")
	if err != nil {
		log.Fatalf("Failed to load endpoint: %v", err)
	}

	if err := fetchAndSaveResponse(endpoint.Url, "response.json"); err != nil {
		log.Fatalf("Failed to fetch data: %v", err)
	}
}
