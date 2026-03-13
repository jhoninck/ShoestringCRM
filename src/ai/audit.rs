pub struct AuditLogger {}

impl AuditLogger {

    pub fn new() -> Self {
        Self {}
    }

    pub fn log(
        &self,
        user_id: &str,
        org_id: &str,
        prompt: &str,
    ) {

        println!(
            "[AI AUDIT] user={} org={} prompt={}",
            user_id,
            org_id,
            prompt
        );
    }
}
