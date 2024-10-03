package main

import "github.com/gin-gonic/gin"

// setupRouter creates a gin Engine instance, assigns the api routes, and returns the engine
func setupRouter() *gin.Engine {
	r := gin.Default()

	return r
}
