// src/agi_project_manager.rs

//! Manages projects and tasks for the Tariro AGI, enabling autonomous goal achievement.

use crate::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use async_trait::async_trait;

use crate::config::AGIConfig;
use crate::telemetry::{TelemetryEvent, TelemetryLogger};

/// Represents a high-level project managed by the AGI.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: ProjectStatus,
    pub creation_date: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub due_date: Option<DateTime<Utc>>,
    pub assigned_agi_modules: Vec<String>,
    pub associated_meta_goal_id: Option<String>,
    pub progress_percentage: f32,
}

/// Status of a project.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ProjectStatus {
    Planning,
    InProgress,
    Paused,
    Completed,
    Archived,
    Failed,
}

impl std::fmt::Display for ProjectStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectStatus::Planning => write!(f, "Planning"),
            ProjectStatus::InProgress => write!(f, "InProgress"),
            ProjectStatus::Paused => write!(f, "Paused"),
            ProjectStatus::Completed => write!(f, "Completed"),
            ProjectStatus::Archived => write!(f, "Archived"),
            ProjectStatus::Failed => write!(f, "Failed"),
        }
    }
}

/// Represents a discrete task within a project.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectTask {
    pub id: String,
    pub project_id: String,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub assigned_to: Option<String>, // AGI module name or specific agent ID
    pub creation_date: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub due_date: Option<DateTime<Utc>>,
    pub dependencies: Vec<String>, // Other task IDs this one depends on
    pub estimated_effort_hours: Option<f32>,
    pub actual_effort_hours: Option<f32>,
    pub progress_percentage: f32,
}

/// Status of a task.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Blocked,
    Review,
    Done,
    Failed,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Todo => write!(f, "Todo"),
            TaskStatus::InProgress => write!(f, "InProgress"),
            TaskStatus::Blocked => write!(f, "Blocked"),
            TaskStatus::Review => write!(f, "Review"),
            TaskStatus::Done => write!(f, "Done"),
            TaskStatus::Failed => write!(f, "Failed"),
        }
    }
}

/// Priority of a task.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[async_trait]
pub trait ProjectManagerServiceTrait: Send + Sync {
    fn new(config: AGIConfig, telemetry_logger: TelemetryLogger) -> Self;
    async fn initialize(&self) -> Result<(), String>;
    async fn shutdown(&self) -> Result<(), String>;
    async fn create_project(&mut self, project: Project) -> Result<String, String>;
    async fn get_project(&self, project_id: &str) -> Option<&Project>;
    async fn update_project(&mut self, updated_project: Project) -> Result<(), String>;
    async fn create_project_task(&mut self, task: ProjectTask) -> Result<String, String>;
    async fn get_task(&self, task_id: &str) -> Option<&ProjectTask>;
    async fn update_task(&mut self, updated_task: ProjectTask) -> Result<(), String>;
    async fn list_projects(&self, status: Option<ProjectStatus>) -> Vec<&Project>;
    async fn list_tasks_for_project(&self, project_id: &str, status: Option<TaskStatus>) -> Vec<&ProjectTask>;
    async fn check_for_new_tasks(&self);
    async fn analyze_task_dependencies(&self, project_id: &str) -> HashMap<String, Vec<String>>; 
    async fn propose_task_schedule(&self, project_id: &str) -> Result<Vec<String>, String>;
    async fn estimate_project_completion(&self, project_id: &str) -> Option<DateTime<Utc>>; 
}

/// The `ProjectManagerService` is responsible for managing Tariro's projects and tasks.
pub struct ProjectManagerService {
    config: AGIConfig,
    telemetry_logger: TelemetryLogger,
    projects: HashMap<String, Project>,
    tasks: HashMap<String, ProjectTask>,
}

#[async_trait]
impl ProjectManagerServiceTrait for ProjectManagerService {
    /// Creates a new `ProjectManagerService` instance.
    fn new(config: AGIConfig, telemetry_logger: TelemetryLogger) -> Self {
        ProjectManagerService {
            config,
            telemetry_logger,
            projects: HashMap::new(),
            tasks: HashMap::new(),
        }
    }

    async fn initialize(&self) -> Result<(), String> {
        println!("[ProjectManagerService] Initializing...");
        self.telemetry_logger.log_event(
            TelemetryEvent::ServiceInitialized { service_name: "ProjectManagerService".to_string() }
        ).await;
        Ok(())
    }

    async fn shutdown(&self) -> Result<(), String> {
        println!("[ProjectManagerService] Shutting down...");
        self.telemetry_logger.log_event(
            TelemetryEvent::ServiceShutdown { service_name: "ProjectManagerService".to_string() }
        ).await;
        Ok(())
    }

    /// Creates a new project.
    async fn create_project(&mut self, mut project: Project) -> Result<String, String> {
        project.id = format!("proj_{}", Uuid::new_v4());
        project.creation_date = Utc::now();
        project.last_updated = Utc::now();
        project.status = ProjectStatus::Planning;
        project.progress_percentage = 0.0;

        if self.projects.contains_key(&project.id) {
            return Err(format!("Project with ID {} already exists.", project.id));
        }
        println!("[ProjectManagerService] Creating project: {}", project.name);
        self.telemetry_logger.log_event(TelemetryEvent::ProjectCreated {
            project_id: project.id.clone(),
            project_name: project.name.clone(),
        }).await;
        self.projects.insert(project.id.clone(), project.clone());
        Ok(project.id)
    }

    /// Retrieves a project by its ID.
    async fn get_project(&self, project_id: &str) -> Option<&Project> {
        self.projects.get(project_id)
    }

    /// Updates an existing project.
    async fn update_project(&mut self, mut updated_project: Project) -> Result<(), String> {
        if let Some(project) = self.projects.get_mut(&updated_project.id) {
            project.name = updated_project.name;
            project.description = updated_project.description;
            project.status = updated_project.status;
            project.last_updated = Utc::now();
            project.due_date = updated_project.due_date;
            project.assigned_agi_modules = updated_project.assigned_agi_modules;
            project.associated_meta_goal_id = updated_project.associated_meta_goal_id;
            project.progress_percentage = updated_project.progress_percentage;

            self.telemetry_logger.log_event(TelemetryEvent::ProjectUpdated {
                project_id: project.id.clone(),
                project_name: project.name.clone(),
                new_status: Some(project.status.to_string()),
            }).await;
            Ok(())
        } else {
            Err(format!("Project with ID {} not found.", updated_project.id))
        }
    }

    /// Creates a new task within a project.
    async fn create_project_task(&mut self, mut task: ProjectTask) -> Result<String, String> {
        if !self.projects.contains_key(&task.project_id) {
            return Err(format!("Project with ID {} not found. Cannot create task.", task.project_id));
        }
        task.id = format!("task_{}", Uuid::new_v4());
        task.creation_date = Utc::now();
        task.last_updated = Utc::now();
        task.status = TaskStatus::Todo;
        task.progress_percentage = 0.0;

        if self.tasks.contains_key(&task.id) {
            return Err(format!("Task with ID {} already exists.", task.id));
        }
        println!("[ProjectManagerService] Creating task: {}", task.title);
        self.telemetry_logger.log_event(TelemetryEvent::TaskCreated {
            task_id: task.id.clone(),
            task_title: task.title.clone(),
            project_id: task.project_id.clone(),
        }).await;
        self.tasks.insert(task.id.clone(), task.clone());
        Ok(task.id)
    }

    /// Retrieves a task by its ID.
    async fn get_task(&self, task_id: &str) -> Option<&ProjectTask> {
        self.tasks.get(task_id)
    }

    /// Updates an existing task.
    async fn update_task(&mut self, mut updated_task: ProjectTask) -> Result<(), String> {
        if let Some(task) = self.tasks.get_mut(&updated_task.id) {
            task.project_id = updated_task.project_id;
            task.title = updated_task.title;
            task.description = updated_task.description;
            task.status = updated_task.status;
            task.priority = updated_task.priority;
            task.assigned_to = updated_task.assigned_to;
            task.last_updated = Utc::now();
            task.due_date = updated_task.due_date;
            task.dependencies = updated_task.dependencies;
            task.estimated_effort_hours = updated_task.estimated_effort_hours;
            task.actual_effort_hours = updated_task.actual_effort_hours;
            task.progress_percentage = updated_task.progress_percentage;

            self.telemetry_logger.log_event(TelemetryEvent::TaskUpdated {
                task_id: task.id.clone(),
                task_title: task.title.clone(),
                new_status: Some(task.status.to_string()),
            }).await;
            Ok(())
        } else {
            Err(format!("Task with ID {} not found.", updated_task.id))
        }
    }

    /// Lists projects, optionally filtered by status.
    async fn list_projects(&self, status: Option<ProjectStatus>) -> Vec<&Project> {
        self.projects.values().filter(|p| {
            status.as_ref().map_or(true, |s| &p.status == s)
        }).collect()
    }

    /// Lists tasks for a given project, optionally filtered by status.
    async fn list_tasks_for_project(&self, project_id: &str, status: Option<TaskStatus>) -> Vec<&ProjectTask> {
        self.tasks.values().filter(|t| {
            t.project_id == project_id && status.as_ref().map_or(true, |s| &t.status == s)
        }).collect()
    }

    /// Periodically checks for new tasks or updates to existing tasks from external sources or internal modules.
    async fn check_for_new_tasks(&self) {
        println!("[ProjectManagerService] Checking for new tasks...");
        // This method would integrate with other AGI modules or external systems
        // to discover new work, sync project states, etc.
        self.telemetry_logger.log_event(
            TelemetryEvent::PeriodicCheck { check_name: "NewTasksCheck".to_string() }
        ).await;
        // Simulate finding a new task or update
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    /// Analyzes task dependencies for a given project.
    async fn analyze_task_dependencies(&self, project_id: &str) -> HashMap<String, Vec<String>> {
        let mut dependencies_map = HashMap::new();
        for task in self.tasks.values() {
            if task.project_id == project_id {
                dependencies_map.insert(task.id.clone(), task.dependencies.clone());
            }
        }
        // Further analysis could detect circular dependencies, critical paths, etc.
        dependencies_map
    }

    /// Proposes an optimal schedule for tasks within a project based on dependencies and priorities.
    async fn propose_task_schedule(&self, project_id: &str) -> Result<Vec<String>, String> {
        let project_tasks = self.list_tasks_for_project(project_id, None).await;
        if project_tasks.is_empty() {
            return Err(format!("No tasks found for project {}.", project_id));
        }

        // Simple topological sort for demonstration. Real scheduling would be more complex.
        let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
        let mut in_degree: HashMap<&str, usize> = HashMap::new();

        for task in &project_tasks {
            in_degree.entry(&task.id).or_insert(0);
            for dep_id in &task.dependencies {
                graph.entry(dep_id).or_insert_with(Vec::new).push(&task.id);
                *in_degree.entry(&task.id).or_insert(0) += 1;
            }
        }

        let mut queue: Vec<&str> = in_degree.iter()
            .filter(|(_id, &degree)| degree == 0)
            .map(|(id, _degree)| *id)
            .collect();

        let mut schedule = Vec::new();
        while let Some(task_id) = queue.pop() {
            schedule.push(task_id.to_string());
            if let Some(dependents) = graph.get(task_id) {
                for &dependent_id in dependents {
                    let count = in_degree.get_mut(dependent_id).unwrap();
                    *count -= 1;
                    if *count == 0 {
                        queue.push(dependent_id);
                    }
                }
            }
        }

        if schedule.len() != project_tasks.len() {
            Err("Circular dependency detected or some tasks are isolated.".to_string())
        } else {
            Ok(schedule)
        }
    }

    /// Estimates the completion date for a project based on current progress, estimated efforts, and dependencies.
    async fn estimate_project_completion(&self, project_id: &str) -> Option<DateTime<Utc>> {
        let tasks = self.list_tasks_for_project(project_id, None).await;
        if tasks.is_empty() { return None; }

        let now = Utc::now();
        let mut max_completion_time = now;

        // This is a very simplified estimation.
        // A real system would use critical path analysis, resource availability, etc.
        for task in tasks {
            if task.status == TaskStatus::Done { continue; }

            let remaining_effort = task.estimated_effort_hours.unwrap_or(0.0) * (1.0 - task.progress_percentage / 100.0);
            // Assuming 8 hours/day for simplicity
            let days_remaining = (remaining_effort / 8.0).ceil(); 

            if days_remaining > 0.0 {
                let estimated_task_completion = now + chrono::Duration::days(days_remaining as i64);
                if estimated_task_completion > max_completion_time {
                    max_completion_time = estimated_task_completion;
                }
            }
        }

        if max_completion_time == now { 
            // All tasks are done or have no estimated effort
            Some(now) 
        } else {
            Some(max_completion_time)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AGIConfig;
    use crate::telemetry::TelemetryEvent;
    use tokio::sync::mpsc;

    fn setup_services() -> (ProjectManagerService, mpsc::Receiver<TelemetryEvent>) {
        let config = AGIConfig::default();
        let (tx, rx) = mpsc::channel(10);
        let telemetry_logger = TelemetryLogger::new(tx);
        (ProjectManagerService::new(config, telemetry_logger), rx)
    }

    #[tokio::test]
    async fn test_create_project() {
        let (mut service, mut rx) = setup_services();
        let project = Project {
            id: String::new(), // Will be overwritten
            name: "Test Project".to_string(),
            description: "A test project for AGI management.".to_string(),
            status: ProjectStatus::Planning,
            creation_date: Utc::now(),
            last_updated: Utc::now(),
            due_date: None,
            assigned_agi_modules: vec!["agi_core".to_string()],
            associated_meta_goal_id: None,
            progress_percentage: 0.0,
        };

        let project_id = service.create_project(project.clone()).await.unwrap();
        assert!(!project_id.is_empty());
        assert!(service.get_project(&project_id).await.is_some());

        let event = rx.recv().await.unwrap();
        match event {
            TelemetryEvent::ProjectCreated { project_id: event_proj_id, project_name } => {
                assert_eq!(event_proj_id, project_id);
                assert_eq!(project_name, "Test Project");
            },
            _ => panic!("Unexpected telemetry event"),
        }
    }

    #[tokio::test]
    async fn test_update_project() {
        let (mut service, mut rx) = setup_services();
        let project = Project {
            id: String::new(),
            name: "Original Name".to_string(),
            description: "Original Description".to_string(),
            status: ProjectStatus::Planning,
            creation_date: Utc::now(),
            last_updated: Utc::now(),
            due_date: None,
            assigned_agi_modules: vec![],
            associated_meta_goal_id: None,
            progress_percentage: 0.0,
        };
        let project_id = service.create_project(project.clone()).await.unwrap();

        let mut updated_project = service.get_project(&project_id).await.unwrap().clone();
        updated_project.name = "Updated Project Name".to_string();
        updated_project.status = ProjectStatus::InProgress;

        service.update_project(updated_project.clone()).await.unwrap();
        let retrieved_project = service.get_project(&project_id).await.unwrap();
        assert_eq!(retrieved_project.name, "Updated Project Name");
        assert_eq!(retrieved_project.status, ProjectStatus::InProgress);

        let _ = rx.recv().await.unwrap(); // Consume ProjectCreated event
        let event = rx.recv().await.unwrap();
        match event {
            TelemetryEvent::ProjectUpdated { project_id: event_proj_id, project_name, new_status } => {
                assert_eq!(event_proj_id, project_id);
                assert_eq!(project_name, "Updated Project Name");
                assert_eq!(new_status, Some("InProgress".to_string()));
            },
            _ => panic!("Unexpected telemetry event"),
        }
    }

    #[tokio::test]
    async fn test_create_project_task() {
        let (mut service, mut rx) = setup_services();
        let project = Project {
            id: String::new(),
            name: "Task Project".to_string(),
            description: "".to_string(),
            status: ProjectStatus::Planning,
            creation_date: Utc::now(),
            last_updated: Utc::now(),
            due_date: None,
            assigned_agi_modules: vec![],
            associated_meta_goal_id: None,
            progress_percentage: 0.0,
        };
        let project_id = service.create_project(project.clone()).await.unwrap();

        let task = ProjectTask {
            id: String::new(),
            project_id: project_id.clone(),
            title: "First Task".to_string(),
            description: "".to_string(),
            status: TaskStatus::Todo,
            priority: TaskPriority::Medium,
            assigned_to: None,
            creation_date: Utc::now(),
            last_updated: Utc::now(),
            due_date: None,
            dependencies: vec![],
            estimated_effort_hours: Some(8.0),
            actual_effort_hours: None,
            progress_percentage: 0.0,
        };

        let task_id = service.create_project_task(task.clone()).await.unwrap();
        assert!(!task_id.is_empty());
        assert!(service.get_task(&task_id).await.is_some());

        let _ = rx.recv().await.unwrap(); // Consume ProjectCreated event
        let event = rx.recv().await.unwrap();
        match event {
            TelemetryEvent::TaskCreated { task_id: event_task_id, task_title, project_id: event_proj_id } => {
                assert_eq!(event_task_id, task_id);
                assert_eq!(task_title, "First Task");
                assert_eq!(event_proj_id, project_id);
            },
            _ => panic!("Unexpected telemetry event"),
        }
    }

    #[tokio::test]
    async fn test_propose_task_schedule_simple() {
        let (mut service, _rx) = setup_services();
        let project = Project {
            id: String::new(), name: "Schedule Project".to_string(), description: "".to_string(), status: ProjectStatus::Planning,
            creation_date: Utc::now(), last_updated: Utc::now(), due_date: None, assigned_agi_modules: vec![],
            associated_meta_goal_id: None, progress_percentage: 0.0,
        };
        let project_id = service.create_project(project.clone()).await.unwrap();

        let task_a = ProjectTask {
            id: String::new(), project_id: project_id.clone(), title: "Task A".to_string(), description: "".to_string(),
            status: TaskStatus::Todo, priority: TaskPriority::Medium, assigned_to: None,
            creation_date: Utc::now(), last_updated: Utc::now(), due_date: None, dependencies: vec![],
            estimated_effort_hours: Some(4.0), actual_effort_hours: None, progress_percentage: 0.0,
        };
        let task_a_id = service.create_project_task(task_a.clone()).await.unwrap();

        let task_b = ProjectTask {
            id: String::new(), project_id: project_id.clone(), title: "Task B".to_string(), description: "".to_string(),
            status: TaskStatus::Todo, priority: TaskPriority::Medium, assigned_to: None,
            creation_date: Utc::now(), last_updated: Utc::now(), due_date: None, dependencies: vec![task_a_id.clone()],
            estimated_effort_hours: Some(6.0), actual_effort_hours: None, progress_percentage: 0.0,
        };
        let task_b_id = service.create_project_task(task_b.clone()).await.unwrap();

        let task_c = ProjectTask {
            id: String::new(), project_id: project_id.clone(), title: "Task C".to_string(), description: "".to_string(),
            status: TaskStatus::Todo, priority: TaskPriority::Medium, assigned_to: None,
            creation_date: Utc::now(), last_updated: Utc::now(), due_date: None, dependencies: vec![task_a_id.clone()],
            estimated_effort_hours: Some(3.0), actual_effort_hours: None, progress_percentage: 0.0,
        };
        let task_c_id = service.create_project_task(task_c.clone()).await.unwrap();

        let schedule = service.propose_task_schedule(&project_id).await.unwrap();
        
        // A should be first, then B and C can be in any order relative to each other but after A
        assert_eq!(schedule[0], task_a_id);
        assert!(schedule[1] == task_b_id || schedule[1] == task_c_id);
        assert!(schedule[2] == task_b_id || schedule[2] == task_c_id);
        assert_ne!(schedule[1], schedule[2]);
    }
}
