use crate::ai::ollama_client::OllamaClient;
use crate::ai::policy::AiPolicy;
use crate::ai::redaction::Redactor;
use crate::ai::audit::AuditLogger;

pub struct AiService {
    ollama: OllamaClient,
    policy: AiPolicy,
    redactor: Redactor,
    audit: AuditLogger,
}

impl AiService {
    pub fn new() -> Self {
        Self {
            ollama: OllamaClient::new("http://localhost:11434"),
            policy: AiPolicy::new(),
            redactor: Redactor::new(),
            audit: AuditLogger::new(),
        }
    }

    pub async fn ask(
        &self,
        user_id: &str,
        org_id: &str,
        prompt: &str,
    ) -> Result<String, String> {

        self.policy.check_ai_access(user_id, org_id)?;

        let safe_prompt = self.redactor.clean(prompt);

        let response = self.ollama.generate(&safe_prompt).await?;

        self.audit.log(user_id, org_id, &safe_prompt);

        Ok(response)
    }
}
