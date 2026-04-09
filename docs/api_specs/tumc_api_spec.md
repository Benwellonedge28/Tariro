# Tariro Universal Meta-Compiler (TUMC) API Specification

This document outlines the external API endpoints that TUMC will expose for interoperability with external tools like Tariro IDE (IDPTE).

## Principles
*   **Command-Query Responsibility Segregation (CQRS)**: Clear separation between commands (actions that change state) and queries (actions that retrieve data).
*   **Asynchronous Processing**: Long-running operations like compilation will be asynchronous, returning job IDs for status tracking.
*   **Versioned APIs**: APIs will be versioned to manage changes gracefully.

## Endpoints

### 1. `POST /api/tumc/v1/compile`
*   **Description**: Initiate a compilation/transpilation job.
*   **Payload**:
    ```json
    {
      "project_id": "string", // Reference to a CompilerProject
      "source_files": [
        {
          "path": "string",
          "content": "string"
        }
      ],
      "target_platform": "string", // e.g., "WASM", "x86_64", "ARM64"
      "output_format": "string", // e.g., "executable", "library", "bytecode"
      "optimizations": ["string"] // Optional list of optimization flags
    }
    ```
*   **Response**: `202 Accepted` with a `job_id`.
    ```json
    {
      "job_id": "string",
      "status_url": "/api/tumc/v1/job/{job_id}/status"
    }
    ```

### 2. `GET /api/tumc/v1/job/{job_id}/status`
*   **Description**: Get the status of an asynchronous job (compilation, analysis).
*   **Response**: `200 OK`
    ```json
    {
      "job_id": "string",
      "status": "pending" | "in_progress" | "completed" | "failed",
      "progress_pct": "number", // 0-100
      "result_url": "string", // Optional, URL to compiled output or analysis report
      "error_message": "string" // Optional, if job failed
    }
    ```

### 3. `POST /api/tumc/v1/analyze_code`
*   **Description**: Request static analysis (e.g., ATVE scan) for provided code.
*   **Payload**:
    ```json
    {
      "source_files": [
        {
          "path": "string",
          "content": "string"
        }
      ],
      "analysis_type": "performance" | "security" | "compatibility",
      "report_format": "json" | "markdown"
    }
    ```
*   **Response**: `202 Accepted` with a `job_id`.

## Authentication
*   API Key / Token based authentication will be used.
