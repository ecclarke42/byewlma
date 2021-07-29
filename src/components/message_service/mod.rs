mod agent;
mod consumer;
mod notification;
mod properties;
mod service;

// TODO: Rename parts for message_service instead of Notification

pub use agent::{NotificationAgent, NotificationAgentInput, NotificationAgentOutput};
pub use consumer::NotificationConsumer;
pub use notification::{Notification, NotificationProps};
pub use properties::{Color, Position, Size};
pub use service::NotificationService;
