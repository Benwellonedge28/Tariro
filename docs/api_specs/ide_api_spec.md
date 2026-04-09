# Tariro IDE (IDPTE) API Specification

This document outlines the external API endpoints that the Tariro IDE (IDPTE) will expose for interoperability with external tools like TUMC (Tariro Universal Meta-Compiler).

## Principles
*   **Loose Coupling**: Interactions are purely via API calls, without direct code modification.
*   **Stateless where possible**: Minimize shared state to simplify integration.
*   **Event-Driven**: Support for events/webhooks to notify external systems of changes within the IDE.

## Endpoints

### 1. `POST /api/ide/display_notification`
*   **Description**: Display a notification message within the IDE.
*   **Payload**:
    ```json
    {
      "type": "info" | "warning" | "error",
      "title": "string",
      "message": "string",
      "duration": "number" // Optional, in milliseconds
    }
    ```
*   **Response**: `200 OK`

### 2. `POST /api/ide/open_file`
*   **Description**: Open a specific file path in the IDE editor.
*   **Payload**:
    ```json
    {
      "path": "string",
      "line": "number" // Optional, line number to navigate to
    }
    ```
*   **Response**: `200 OK`

### 3. `POST /api/ide/display_analysis_report`
*   **Description**: Display a structured analysis report (e.g., from ATVE) within the IDE.
*   **Payload**:
    ```json
    {
      "report_type": "string", // e.g., "ATVE_Performance", "ATVE_Security"
      "title": "string",
      "content": "string", // Markdown or structured JSON report
      "file_path": "string", // Optional, file associated with the report
      "severity": "info" | "warning" | "error"
    }
    ```
*   **Response**: `200 OK`

## Authentication
*   API Key / Token based authentication will be used to secure endpoints.
