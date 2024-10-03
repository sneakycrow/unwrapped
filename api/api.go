package main

import (
	"log/slog"

	"github.com/gin-gonic/gin"
)

// The version of the api running. Expected to be set at build time with ldflags
var Version string

type Server struct {
	logger *slog.Logger
	config *Config
	router *gin.Engine
}

// TODO: Login with Spotify OAuth
// TODO: DB structures (gorm)
func SetupServer() *gin.Engine {
	// Setup the logger

	// Setup the router engine
	router := setupRouter()
	slog.Debug("Router successfully setup")
	return router
}

// Setup the logger for the entire API
func SetupLogger() *slog.Logger {
	logger := setupLogger()
	slog.Debug("Logger successfully initialized")
	return logger
}

// Initialize the configuration
func SetupConfig() *Config {
	config := setupConfig()
	// Set the gin mode based on the config
	gin.SetMode(config.Mode.GinMode())
	slog.Debug("Config successfully setup", config.IntoLog())
	return config
}
