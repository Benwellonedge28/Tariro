#!/bin/bash

# Tariro AGI Deployment Script
# This script automates the build, containerization, and deployment process for Tariro AGI.
# It supports local Docker deployment and cloud-agnostic Kubernetes deployment.

# --- Configuration --- 
REPO_NAME="tariro-agi"
IMAGE_NAME="tariro-agi-image"
DOCKER_REGISTRY="your-docker-registry.com" # e.g., ghcr.io/your-username or public.ecr.aws/your-repo
KUBE_NAMESPACE="tariro-agi"

# --- Colors for better output --- 
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# --- Functions --- 

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

check_command() {
    command -v "$1" >/dev/null 2>&1 || log_error "$1 is not installed. Please install it to proceed."
}

build_rust_project() {
    log_info "Building Rust project in release mode..."
    cargo build --release || log_error "Rust project build failed."
    log_info "Rust project built successfully."
}

build_docker_image() {
    log_info "Building Docker image: ${IMAGE_NAME} ..."
    docker build -t "${IMAGE_NAME}" . || log_error "Docker image build failed."
    log_info "Docker image built successfully."
}

tag_and_push_docker_image() {
    if [ -z "$DOCKER_REGISTRY" ]; then
        log_warn "DOCKER_REGISTRY is not set. Skipping tagging and pushing to remote registry."
        return
    fi

    log_info "Tagging Docker image for registry: ${DOCKER_REGISTRY}/${IMAGE_NAME}:latest ..."
    docker tag "${IMAGE_NAME}" "${DOCKER_REGISTRY}/${IMAGE_NAME}:latest" || log_error "Docker image tagging failed."
    log_info "Pushing Docker image to registry: ${DOCKER_REGISTRY}/${IMAGE_NAME}:latest ..."
    docker push "${DOCKER_REGISTRY}/${IMAGE_NAME}:latest" || log_error "Docker image push failed."
    log_info "Docker image pushed successfully."
}

deploy_to_docker_local() {
    log_info "Deploying to local Docker..."
    # Stop and remove existing container if it exists
    if docker ps -a --format '{{.Names}}' | grep -q "${REPO_NAME}-local"; then
        log_info "Stopping existing container ${REPO_NAME}-local..."
        docker stop "${REPO_NAME}-local" >/dev/null
        log_info "Removing existing container ${REPO_NAME}-local..."
        docker rm "${REPO_NAME}-local" >/dev/null
    fi

    # Run new container
    docker run -d --name "${REPO_NAME}-local" -p 8080:8080 "${IMAGE_NAME}" || log_error "Local Docker deployment failed."
    log_info "Application deployed locally via Docker. Access at http://localhost:8080"
}

deploy_to_kubernetes() {
    log_info "Deploying to Kubernetes..."
    check_command "kubectl"

    log_info "Creating/updating Kubernetes namespace ${KUBE_NAMESPACE}..."
    kubectl create namespace "${KUBE_NAMESPACE}" --dry-run=client -o yaml | kubectl apply -f - || log_error "Failed to create/update Kubernetes namespace."

    # Ensure the registry is set for K8s deployment
    if [ -z "$DOCKER_REGISTRY" ]; then
        log_error "DOCKER_REGISTRY must be set for Kubernetes deployment."
    }

    # Apply Kubernetes deployments and services
    log_info "Applying Kubernetes configurations..."
    # Replace image name in k8s manifests, assuming manifests are in a 'kubernetes/' directory
    # This is a simplified example; in a real scenario, you'd use Helm or Kustomize
    find kubernetes -name "*.yaml" -print0 | xargs -0 sed -i "s|${IMAGE_NAME}|${DOCKER_REGISTRY}/${IMAGE_NAME}:latest|g"

    kubectl apply -f kubernetes/deployment.yaml -n "${KUBE_NAMESPACE}" || log_error "Kubernetes deployment failed."
    kubectl apply -f kubernetes/service.yaml -n "${KUBE_NAMESPACE}" || log_error "Kubernetes service deployment failed."

    log_info "Kubernetes deployment initiated successfully."
    log_info "Check deployment status with: kubectl get pods -n ${KUBE_NAMESPACE}"
    log_info "Access service with: kubectl get svc -n ${KUBE_NAMESPACE}"
}

show_help() {
    echo "Usage: $0 {build|docker-local|kubernetes|all|help}"
    echo "  build         - Builds the Rust project and Docker image."
    echo "  docker-local  - Deploys the application to a local Docker container."
    echo "  kubernetes    - Deploys the application to a Kubernetes cluster."
    echo "  all           - Performs build, Docker image push (if registry set), and then deploys to local Docker or Kubernetes."
    echo "  help          - Shows this help message."
}

# --- Main Script Logic --- 

ACTION="$1"

case "$ACTION" in
    build)
        check_command "cargo"
        check_command "docker"
        build_rust_project
        build_docker_image
        ;;
    docker-local)
        check_command "docker"
        deploy_to_docker_local
        ;;
    kubernetes)
        deploy_to_kubernetes
        ;;
    all)
        check_command "cargo"
        check_command "docker"
        build_rust_project
        build_docker_image
        tag_and_push_docker_image
        
        # Decide whether to deploy to local Docker or Kubernetes
        if [ -f "kubernetes/deployment.yaml" ] && [ -n "$KUBECONFIG" ]; then
            deploy_to_kubernetes
        else
            deploy_to_docker_local
        fi
        ;;
    help)
        show_help
        ;;
    *)
        log_error "Invalid action: $ACTION. Use 'help' to see available actions."
        ;;
esac
