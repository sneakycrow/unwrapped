package main

import (
	"log/slog"
	"os"

	"github.com/gin-gonic/gin"
)

type Config struct {
	Port string
	Mode Mode
}

type Mode string

const (
	Debug      Mode = "debug"
	Production Mode = "production"
)

func isValidMode(mode Mode) bool {
	switch mode {
	case Debug, Production:
		return true
	default:
		return false
	}
}

func (m Mode) GinMode() string {
	switch m {
	case Debug:
		return gin.DebugMode
	case Production:
		return gin.ReleaseMode
	default:
		slog.Debug("No value gin mode for provided mode, defaulting to Debug", "mode", m)
		return gin.DebugMode
	}
}

const DEFAULT_PORT = "3000"

// Creates a default configuration, overrides with any set env vars, and then returns the Config
func setupConfig() *Config {
	c := Default()
	// Check whether the UNWRAPPED_PORT environment variable is configured
	port := os.Getenv("PORT")
	if port != "" {
		c.Port = port
	}
	// Get the mode from UNWRAPPED_MODE environment variable
	// Defaults to debug if not set
	mode := os.Getenv("UNWRAPPED_MODE")
	if mode != "" {
		if !isValidMode(Mode(mode)) {
			slog.Debug("Invalid UNWRAPPED_MODE supplied, defaulting to Debug", "mode", mode)
			mode = string(Debug)
		}
		c.Mode = Mode(mode)
	} else {
		c.Mode = Debug
	}
	return c
}

func (c *Config) IntoLog() slog.Attr {
	return slog.Group("config",
		slog.String("port", c.Port),
		slog.String("mode", string(c.Mode)),
	)
}

func Default() *Config {
	return &Config{
		Port: DEFAULT_PORT,
		Mode: Debug,
	}
}
