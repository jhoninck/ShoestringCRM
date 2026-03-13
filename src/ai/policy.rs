pub struct AiPolicy {}

impl AiPolicy {

    pub fn new() -> Self {
        Self {}
    }

    pub fn check_ai_access(
        &self,
        user_id: &str,
        org_id: &str
    ) -> Result<(), String> {

        if user_id.is_empty() {
            return Err("Unauthorized AI access".into());
        }

        if org_id.is_empty() {
            return Err("Tenant missing".into());
        }

        Ok(())
    }
}
