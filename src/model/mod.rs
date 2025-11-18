pub mod types;
pub mod account;
pub mod contact;
pub mod ticket;
pub mod social_event;
pub mod lead;
pub mod opportunity;

pub use types::*;
pub use account::ACCOUNT_MODEL;
pub use contact::CONTACT_MODEL;
pub use ticket::TICKET_MODEL;
pub use social_event::SOCIAL_EVENT_MODEL;
pub use lead::LEAD_MODEL;
pub use opportunity::OPPORTUNITY_MODEL;

pub static ALL_MODELS: [&ModelDef; 6] = [
    ACCOUNT_MODEL,
    CONTACT_MODEL,
    TICKET_MODEL,
    SOCIAL_EVENT_MODEL,
    LEAD_MODEL,
    OPPORTUNITY_MODEL,
];

pub fn get_model(name: &str) -> Option<&'static ModelDef> {
    ALL_MODELS.iter().find(|m| m.name == name).copied()
}
