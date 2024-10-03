package main

import (
	"context"
	"log/slog"
	"os"
	"time"
)

// Sets up the core logger and assigns it as the default
func setupLogger() *slog.Logger {
	opts := DefaultOptions()
	logger := slog.New(NewCustomJSONHandler(os.Stdout, opts))
	slog.SetDefault(logger)
	return logger
}

func DefaultOptions() *slog.HandlerOptions {
	// Get the log level, defaulting to Info if unset
	level := slog.LevelDebug
	if envLevel, ok := os.LookupEnv("LOG_LEVEL"); ok {
		switch envLevel {
		case "DEBUG":
			level = slog.LevelDebug
		case "INFO":
			level = slog.LevelInfo
		case "WARN":
			level = slog.LevelWarn
		case "ERROR":
			level = slog.LevelError
		default:
			level = slog.LevelInfo
		}
	}
	// Construct the options
	opts := &slog.HandlerOptions{
		Level: level,
		// Replace the time formatting with RFC3339
		ReplaceAttr: func(groups []string, a slog.Attr) slog.Attr {
			if a.Key == slog.TimeKey {
				return slog.String("time", a.Value.Time().Format(time.RFC3339))
			}
			return a
		},
	}

	return opts
}

// CustomJSONHandler wraps a JSONHandler to add version information
type CustomJSONHandler struct {
	slog.JSONHandler
	version string
}

// NewCustomJSONHandler creates a new CustomJSONHandler
func NewCustomJSONHandler(out *os.File, opts *slog.HandlerOptions) *CustomJSONHandler {
	return &CustomJSONHandler{
		JSONHandler: *slog.NewJSONHandler(out, opts),
		version:     Version,
	}
}

// Handle adds the version to the log record before passing it to the underlying JSONHandler
func (h *CustomJSONHandler) Handle(ctx context.Context, r slog.Record) error {
	r.AddAttrs(slog.String("version", h.version))
	return h.JSONHandler.Handle(ctx, r)
}

// WithAttrs returns a new CustomJSONHandler with the given attributes added to it
func (h *CustomJSONHandler) WithAttrs(attrs []slog.Attr) slog.Handler {
	return &CustomJSONHandler{
		JSONHandler: *h.JSONHandler.WithAttrs(attrs).(*slog.JSONHandler),
		version:     h.version,
	}
}

// WithGroup returns a new CustomJSONHandler with the given group added to it
func (h *CustomJSONHandler) WithGroup(name string) slog.Handler {
	return &CustomJSONHandler{
		JSONHandler: *h.JSONHandler.WithGroup(name).(*slog.JSONHandler),
		version:     h.version,
	}
}
