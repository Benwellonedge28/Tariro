#!/bin/bash
# Tariro AGI Docker Entrypoint Script
# Production-ready entrypoint with health checks and graceful shutdown

set -euo pipefail

# Configuration
readonly APP_NAME="tariro"
readonly APP_USER="tariro"
readonly LOG_LEVEL="${LOG_LEVEL:-info}"

# Signal handling for graceful shutdown
cleanup() {
    echo "[$APP_NAME] Received shutdown signal, cleaning up..."
    # Add cleanup logic here (close connections, flush buffers, etc.)
    exit 0
}

trap cleanup SIGTERM SIGINT SIGHUP

# Initialize logging
init_logging() {
    echo "[$APP_NAME] Starting $APP_NAME v${VERSION:-unknown}"
    echo "[$APP_NAME] Log level: $LOG_LEVEL"
}

# Health check function
health_check() {
    local host="${1:-localhost}"
    local port="${2:-8080}"
    local timeout="${3:-5}"

    if command -v curl >/dev/null 2>&1; then
        curl -sf "http://${host}:${port}/health" > /dev/null 2>&1
    elif command -v wget >/dev/null 2>&1; then
        wget -q -O - "http://${host}:${port}/health" > /dev/null 2>&1
    else
        # Fallback: check if process is running
        pgrep -f "$APP_NAME" > /dev/null 2>&1
    fi
}

# Wait for dependencies (if any)
wait_for_dependencies() {
    local max_wait="${DEPENDENCY_WAIT_TIME:-60}"
    local waited=0

    # Example: Wait for database
    # if [[ -n "${DATABASE_URL:-}" ]]; then
    #     echo "[$APP_NAME] Waiting for database..."
    #     while ! health_check_db; do
    #         sleep 1
    #         waited=$((waited + 1))
    #         if [[ $waited -ge $max_wait ]]; then
    #             echo "[$APP_NAME] Timeout waiting for database"
    #             exit 1
    #         fi
    #     done
    #     echo "[$APP_NAME] Database is ready"
    # fi
}

# Validate configuration
validate_config() {
    # Check for required environment variables
    if [[ -z "${TARIRO_CONFIG_PATH:-}" ]]; then
        echo "[$APP_NAME] Warning: TARIRO_CONFIG_PATH not set, using default"
    fi

    # Validate TLS settings
    if [[ "${TLS_ENABLED:-false}" == "true" ]]; then
        if [[ -z "${TLS_CERT_PATH:-}" ]] || [[ -z "${TLS_KEY_PATH:-}" ]]; then
            echo "[$APP_NAME] Error: TLS enabled but certificates not provided"
            exit 1
        fi
    fi

    echo "[$APP_NAME] Configuration validated"
}

# Run pre-start checks
pre_start_checks() {
    echo "[$APP_NAME] Running pre-start checks..."

    # Check if running as correct user
    if [[ "$(id -u)" == "0" ]]; then
        echo "[$APP_NAME] Warning: Running as root is not recommended"
    fi

    # Check disk space
    local available_space
    available_space=$(df -BG /app 2>/dev/null | awk 'NR==2 {print $4}' | sed 's/G//')
    if [[ -n "$available_space" ]] && [[ "$available_space" -lt 1 ]]; then
        echo "[$APP_NAME] Error: Insufficient disk space"
        exit 1
    fi

    echo "[$APP_NAME] Pre-start checks passed"
}

# Main execution
main() {
    init_logging
    validate_config
    pre_start_checks
    wait_for_dependencies

    echo "[$APP_NAME] Starting application..."

    # Set runtime environment
    export RUST_LOG="${RUST_LOG:-$LOG_LEVEL}"
    export RUST_BACKTRACE="${RUST_BACKTRACE:-1}"

    # Execute the main application
    exec "$@"  # Pass all arguments to the application
}

# Run as non-root user if needed
switch_user() {
    if [[ "$(id -u)" == "0" ]] && [[ -n "$APP_USER" ]]; then
        echo "[$APP_NAME] Switching to user: $APP_USER"
        exec su-exec "$APP_USER" "$0" "$@"
    fi
}

# Health endpoint (for Kubernetes probes)
if [[ "${1:-}" == "health" ]]; then
    exec /app/tariro --health-check
fi

# Main entry point
main "$@"
