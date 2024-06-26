//! `/thread` endpoints for managing grouped user messages.
use crate::model::{Comment, K7Response, MessageInsights, Thread, ThreadDebug, ThreadEscalation};
use crate::service::AgentService;
use rocket::serde::json::Json;

/// All endpoints to be served by Rocket
pub fn routes() -> Vec<rocket::Route> {
    routes![
        get_thread,
        get_thread_comments,
        get_thread_debug,
        get_thread_insights,
        escalate_thread
    ]
}

/// Load the thread using the Kindness ID or Cerb ticket mask
#[get("/thread/<kid_or_ticket_mask>", format = "json")]
async fn get_thread(kid_or_ticket_mask: &str, service: AgentService) -> K7Response<Thread> {
    service.get_thread(kid_or_ticket_mask).await.into()
}

/// A debug page for inspecting the exact prompt that would be sent to OpenAI
/// for generating a reply.
#[get("/debug/thread/<kid>", format = "json")]
pub async fn get_thread_debug(kid: &str, service: AgentService) -> K7Response<ThreadDebug> {
    service.get_thread_debug(kid).await.into()
}

#[get("/thread/<thread_id>/comments", format = "json")]
pub async fn get_thread_comments(
    thread_id: i32,
    service: AgentService,
) -> K7Response<Vec<Comment>> {
    service.get_thread_comments(thread_id).await.into()
}

#[get("/thread/<thread_id>/insights", format = "json")]
pub async fn get_thread_insights(
    thread_id: i32,
    service: AgentService,
) -> K7Response<MessageInsights> {
    service.get_thread_insights(thread_id).await.into()
}

#[post("/thread/escalate", format = "json", data = "<escalation>")]
pub async fn escalate_thread(
    escalation: Json<ThreadEscalation>,
    service: AgentService,
) -> K7Response<()> {
    service.escalate_thread(escalation.0).await.into()
}
