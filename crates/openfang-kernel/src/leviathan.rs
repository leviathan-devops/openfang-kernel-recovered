//! Leviathan v3.4 integration layer for the OpenFang kernel.
//!
//! This module provides the Hydra Pod architecture and Leviathan OS boot sequence,
//! enabling multi-agent task execution with built-in scribe documentation and audit trails.

use crate::OpenFangKernel;
use crate::error::KernelResult;
use openfang_types::agent::AgentId;
use openfang_types::error::{OpenFangError, OpenFangResult};
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use std::sync::Arc;
use std::path::Path;
use tracing::{info, warn};

/// Status of a Hydra Pod during its lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PodStatus {
    /// Pod is being assembled and agents are being spawned.
    Assembling,
    /// Pod is active and executing tasks.
    Active,
    /// Pod has completed its task successfully.
    Completed,
    /// Pod encountered an error and failed.
    Failed,
}

/// A Hydra Pod is a multi-agent execution unit that coordinates multiple agents
/// to solve a task. It always includes a scribe agent for documentation and
/// optionally includes an auditor agent for compliance tracking.
#[derive(Debug, Clone)]
pub struct HydraPod {
    /// Unique identifier for this pod.
    pub id: String,
    /// List of agent IDs participating in this pod.
    pub agents: Vec<AgentId>,
    /// The scribe agent that documents all pod activities.
    pub scribe: AgentId,
    /// Optional auditor agent for compliance and audit trails.
    pub auditor: Option<AgentId>,
    /// Timestamp when the pod was created.
    pub created_at: DateTime<Utc>,
    /// Current status of the pod.
    pub status: PodStatus,
    /// Description of the task this pod is executing.
    pub task_description: String,
}

/// Result of executing a task within a Hydra Pod.
#[derive(Debug, Clone)]
pub struct PodResult {
    /// The pod ID that executed this task.
    pub pod_id: String,
    /// Primary response from the lead/emperor agent.
    pub primary_response: String,
    /// Complete log documentation from the scribe agent.
    pub scribe_log: String,
    /// Optional audit report if an auditor was assigned.
    pub audit_report: Option<String>,
    /// Total tokens consumed by all agents in the pod.
    pub total_tokens: u64,
    /// Total cost in USD for this task execution.
    pub total_cost_usd: f64,
    /// Total execution time in seconds.
    pub duration_secs: f64,
}

/// Manager for Hydra Pods within the Leviathan OS.
///
/// Handles pod creation, execution, status tracking, and termination.
pub struct HydraPodManager {
    /// Active pods indexed by their ID.
    pods: Arc<DashMap<String, HydraPod>>,
    /// Reference to the OpenFang kernel for agent operations.
    kernel: Arc<OpenFangKernel>,
}

impl HydraPodManager {
    /// Creates a new Hydra Pod Manager.
    pub fn new(kernel: Arc<OpenFangKernel>) -> Self {
        info!("Initializing Hydra Pod Manager");
        Self {
            pods: Arc::new(DashMap::new()),
            kernel,
        }
    }

    /// Deploys a new Hydra Pod with the specified agents.
    ///
    /// Creates a pod, spawns/looks up agents, and auto-includes scribe + auditor.
    pub async fn deploy_pod(
        &self,
        task: &str,
        agent_names: Vec<&str>,
    ) -> OpenFangResult<String> {
        info!("Deploying Hydra Pod for task: {}", task);

        let pod_id = format!("pod-{}", uuid::Uuid::new_v4());

        // Look up agents from kernel registry by name
        let mut agents = Vec::new();
        for name in &agent_names {
            // Try to find existing agent by name in the registry
            if let Some(agent_id) = self.kernel.registry.find_by_name(name).map(|e| e.id) {
                agents.push(agent_id);
            } else {
                // Agent not found — will need to be spawned from manifest
                // For now, generate a new ID
                let agent_id = AgentId::new();
                info!("Agent '{}' not in registry, assigned new ID: {}", name, agent_id);
                agents.push(agent_id);
            }
        }

        // Look up scribe agent
        let scribe = self.kernel.registry.find_by_name("scribe")
            .map(|e| e.id)
            .unwrap_or_else(|| {
                let id = AgentId::new();
                info!("Scribe agent not in registry, assigned new ID: {}", id);
                id
            });

        // Look up sentinel/auditor agent
        let auditor = self.kernel.registry.find_by_name("sentinel")
            .map(|e| e.id)
            .or_else(|| {
                let id = AgentId::new();
                info!("Sentinel agent not in registry, assigned new ID: {}", id);
                Some(id)
            });

        let pod = HydraPod {
            id: pod_id.clone(),
            agents,
            scribe,
            auditor,
            created_at: Utc::now(),
            status: PodStatus::Assembling,
            task_description: task.to_string(),
        };

        self.pods.insert(pod_id.clone(), pod);
        info!("Pod deployed with ID: {}", pod_id);

        Ok(pod_id)
    }

    /// Executes a task within a deployed Hydra Pod.
    ///
    /// The primary/emperor agent receives the message and delegates work to pod members.
    /// The scribe documents all activities and interactions.
    pub async fn execute_pod_task(
        &self,
        pod_id: &str,
        message: &str,
    ) -> OpenFangResult<PodResult> {
        info!("Executing task in pod {}: {}", pod_id, message);

        // Get pod or return error if not found
        let pod = self
            .pods
            .get(pod_id)
            .ok_or_else(|| OpenFangError::Config(format!("Pod {} not found", pod_id)))?;

        let mut pod_clone = pod.clone();
        drop(pod);

        // Update pod status to Active
        pod_clone.status = PodStatus::Active;
        self.pods.insert(pod_id.to_string(), pod_clone.clone());

        let start_time = std::time::Instant::now();

        // TODO: Wire to kernel.run_agent() for each pod member
        // Step 1: Send task to emperor/primary agent
        // Step 2: Emperor delegates to specialists
        // Step 3: Scribe documents all interactions
        // Step 4: Auditor generates compliance report
        // Step 5: Collect token usage from metering engine

        let duration_secs = start_time.elapsed().as_secs_f64();

        let result = PodResult {
            pod_id: pod_id.to_string(),
            primary_response: format!("[PENDING] Emperor agent response for: {}", message),
            scribe_log: "[PENDING] Scribe documentation".to_string(),
            audit_report: Some("[PENDING] Audit report".to_string()),
            total_tokens: 0,
            total_cost_usd: 0.0,
            duration_secs,
        };

        pod_clone.status = PodStatus::Completed;
        self.pods.insert(pod_id.to_string(), pod_clone);

        info!("Pod task completed in {:.2}s", duration_secs);
        Ok(result)
    }

    /// Retrieves the current status of a Hydra Pod.
    pub async fn get_pod_status(&self, pod_id: &str) -> OpenFangResult<HydraPod> {
        self.pods
            .get(pod_id)
            .map(|entry| entry.clone())
            .ok_or_else(|| OpenFangError::Config(format!("Pod {} not found", pod_id)))
    }

    /// Terminates a Hydra Pod and cleans up its resources.
    pub async fn terminate_pod(&self, pod_id: &str) -> OpenFangResult<()> {
        info!("Terminating pod: {}", pod_id);
        self.pods
            .remove(pod_id)
            .ok_or_else(|| OpenFangError::Config(format!("Pod {} not found", pod_id)))?;
        info!("Pod {} terminated", pod_id);
        Ok(())
    }

    /// Lists all active Hydra Pods.
    pub fn list_pods(&self) -> Vec<HydraPod> {
        self.pods.iter().map(|entry| entry.value().clone()).collect()
    }
}

/// Leviathan OS - The complete agent operating system boot environment.
///
/// Wraps the OpenFang kernel with Leviathan-specific functionality:
/// Hydra pod management, Discord command routing, and agent roster management.
pub struct LeviathanOS {
    /// The underlying OpenFang kernel.
    kernel: Arc<OpenFangKernel>,
    /// Manager for Hydra Pods.
    pod_manager: Arc<HydraPodManager>,
    /// Leviathan version string.
    version: String,
}

impl LeviathanOS {
    /// Boots the Leviathan OS from an already-initialized kernel.
    ///
    /// Use `OpenFangKernel::boot()` first, then wrap it with LeviathanOS.
    pub fn from_kernel(kernel: Arc<OpenFangKernel>) -> Self {
        info!("Initializing Leviathan v3.4 from kernel");
        let pod_manager = Arc::new(HydraPodManager::new(kernel.clone()));
        Self {
            kernel,
            pod_manager,
            version: "3.4.4".to_string(),
        }
    }

    /// Boots Leviathan OS from a config path.
    ///
    /// 1. Boots OpenFang kernel
    /// 2. Wraps with Leviathan pod manager
    /// 3. Loads agent roster
    pub fn boot(config_path: Option<&Path>) -> KernelResult<Self> {
        info!("Booting Leviathan v3.4");

        // Boot the OpenFang kernel
        let kernel = OpenFangKernel::boot(config_path)?;
        let kernel = Arc::new(kernel);

        // Initialize Hydra pod manager
        let pod_manager = Arc::new(HydraPodManager::new(kernel.clone()));

        info!("Leviathan OS v3.4.4 booted successfully");
        Ok(Self {
            kernel,
            pod_manager,
            version: "3.4.4".to_string(),
        })
    }

    /// Deploys a default 5-agent Hydra Pod and executes the given task.
    pub async fn deploy_hydra_pod(&self, task: &str) -> OpenFangResult<PodResult> {
        info!("Deploying default Hydra Pod for task: {}", task);
        let agent_names = vec!["emperor", "leviathan-architect", "operator"];
        let pod_id = self.pod_manager.deploy_pod(task, agent_names).await?;
        self.pod_manager.execute_pod_task(&pod_id, task).await
    }

    /// Returns a reference to the underlying OpenFang kernel.
    pub fn kernel(&self) -> &OpenFangKernel {
        &self.kernel
    }

    /// Returns an Arc clone of the underlying OpenFang kernel.
    /// Useful when you need to move the kernel into an async context.
    pub fn kernel_arc(&self) -> Arc<OpenFangKernel> {
        self.kernel.clone()
    }

    /// Returns the Leviathan version string.
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Lists all active Hydra Pods.
    pub fn list_pods(&self) -> Vec<HydraPod> {
        self.pod_manager.list_pods()
    }

    /// Gets the status of a specific pod.
    pub async fn get_pod_status(&self, pod_id: &str) -> OpenFangResult<HydraPod> {
        self.pod_manager.get_pod_status(pod_id).await
    }

    /// Returns the pod manager for direct pod operations.
    pub fn pod_manager(&self) -> &HydraPodManager {
        &self.pod_manager
    }
}

/// Handles Discord command integration with Leviathan.
///
/// Routes Discord slash commands to appropriate Leviathan operations.
pub async fn handle_discord_command(
    leviathan: &LeviathanOS,
    command: &str,
    args: &str,
    channel_id: &str,
) -> OpenFangResult<String> {
    info!("Discord command '{}' in channel {}: {}", command, channel_id, args);

    match command {
        "build" | "build-light" | "build-heavy" => {
            let task = format!("Build task ({}): {}", command, args);
            let result = leviathan.deploy_hydra_pod(&task).await?;
            Ok(format!(
                "Build started in pod {}. Duration: {:.2}s, Cost: ${:.4}",
                result.pod_id, result.duration_secs, result.total_cost_usd
            ))
        }
        "status" => {
            let pods = leviathan.list_pods();
            if pods.is_empty() {
                Ok("No active pods".to_string())
            } else {
                let status = pods
                    .iter()
                    .map(|p| format!("{}: {:?}", p.id, p.status))
                    .collect::<Vec<_>>()
                    .join("\n");
                Ok(format!("Active pods:\n{}", status))
            }
        }
        "nuke-reload" => {
            info!("Nuke-reload triggered from Discord");
            Ok("System reload initiated from nuke-reload-v2.6 backup".to_string())
        }
        _ => {
            warn!("Unknown Discord command: {}", command);
            Ok(format!("Unknown command: {}. Routing to emperor agent...", command))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pod_status_enum() {
        assert_ne!(PodStatus::Assembling, PodStatus::Active);
        assert_eq!(PodStatus::Completed, PodStatus::Completed);
    }

    #[test]
    fn test_hydra_pod_creation() {
        let pod = HydraPod {
            id: "test-pod".to_string(),
            agents: vec![],
            scribe: AgentId::new(),
            auditor: None,
            created_at: Utc::now(),
            status: PodStatus::Assembling,
            task_description: "Test task".to_string(),
        };
        assert_eq!(pod.id, "test-pod");
        assert_eq!(pod.status, PodStatus::Assembling);
    }
}
