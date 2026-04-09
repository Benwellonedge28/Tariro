# Secure External Connectors and Event Mechanisms Design

This document outlines the architectural design for implementing secure external connectors and event mechanisms between Tariro_ide_idpte and TUMC, leveraging the generated Python SDKs and OpenAPI specifications.

## 1. Connector Modules (Adapters)

### 1.1. Tariro_ide_idpte Connector (TUMC Client)
*   **Purpose**: To make API calls to TUMC based on IDE events and workflow rules, and to receive notifications/results from TUMC.
*   **Location**: Integrated within the IDE's core event handling system.
*   **Key Components**:
    *   **SDK Wrapper**: A thin layer encapsulating `sdks/python/tumc_client.py`.
    *   **Event Listener**: Listens for internal IDE events (e.g., file save, build command) that map to `WorkflowRules`.
    *   **API Invoker**: Calls TUMC APIs via the SDK wrapper, interpreting `WorkflowRule` actions.
    *   **Notification Receiver (Webhook Endpoint)**: A secure endpoint within the IDE to receive asynchronous callbacks (notifications, job status updates) from TUMC. This endpoint must be exposed securely.

### 1.2. TUMC Connector (IDE Client & Event Publisher)
*   **Purpose**: To receive API calls from the IDE, execute compilation/analysis, and publish events/notifications back to the IDE.
*   **Location**: Integrated within TUMC's job processing and result delivery system.
*   **Key Components**:
    *   **API Endpoint Handler**: Processes incoming API calls from the IDE (e.g., `/api/tumc/v1/compile`). Validates requests against OpenAPI schema.
    *   **SDK Wrapper**: A thin layer encapsulating `sdks/python/tariro_ide_client.py`.
    *   **Event Publisher**: Publishes events (e.g., job completion, error) to a pre-configured IDE webhook endpoint.
    *   **Result Formatter**: Formats compilation/analysis results into a structure consumable by the IDE's `display_analysis_report` API.

## 2. Security Considerations

### 2.1. Authentication & Authorization
*   **API Keys/Tokens**: Both systems will use robust API keys/tokens for mutual authentication.
    *   **IDE -> TUMC**: IDE includes its API key in `Authorization` header for TUMC API calls.
    *   **TUMC -> IDE**: TUMC includes its API key in `Authorization` header when sending notifications/calling IDE APIs.
*   **Secure Storage**: API keys/tokens will be stored securely in environment variables or a dedicated secrets management service, NOT hardcoded.
*   **Rotation**: Mechanisms for regular key rotation will be supported.
*   **Least Privilege**: Access tokens will be scoped to the minimum necessary permissions.

### 2.2. Secure Communication
*   **HTTPS/TLS**: All communication between IDE and TUMC MUST be encrypted using HTTPS (TLS 1.2 or higher). Certificates will be properly managed and validated.
*   **Network Segmentation**: Deploy connectors in appropriately segmented network zones.

### 2.3. Input Validation & Output Sanitization
*   **Schema Validation**: All incoming API requests and outgoing responses will be validated against their respective OpenAPI schemas to prevent malformed data and potential injection attacks.
*   **Content Sanitization**: Any user-generated content (e.g., file content, commit messages, analysis reports) passed between systems will be sanitized to neutralize executable code or dangerous HTML/JS, especially before being displayed in the IDE.

### 2.4. Event Mechanisms (Webhooks)
*   **Signed Payloads**: Webhook payloads (notifications from TUMC to IDE, or vice versa) will be signed using a shared secret and HMAC. The receiver will verify the signature to ensure message integrity and authenticity.
*   **Idempotency**: Webhook endpoints will be idempotent to handle duplicate delivery attempts gracefully.
*   **Rate Limiting**: Implement rate limiting on webhook endpoints to prevent abuse.

### 2.5. Error Handling & Logging
*   **Secure Logging**: Sensitive data (API keys, PII) will be scrubbed from logs. Log levels will be configurable.
*   **Detailed Error Messages**: Error messages returned to the caller will be informative for debugging but generic enough not to leak system internals.
*   **Circuit Breakers/Retries**: Implement robust retry mechanisms with exponential backoff and circuit breakers to handle transient network issues or service unavailability.

## 3. Integration with Workflow Engine

*   The `WorkflowRules` defined (e.g., "IDE Save -> TUMC Compile & IDE Notify") will be triggered by events published by these connector modules.
*   The connector modules will effectively act as the "API call" and "event publication" actions dictated by the `WorkflowRule` engine.
