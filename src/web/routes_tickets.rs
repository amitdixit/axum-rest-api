use crate::{ctx::Ctx, Result};
use axum::{
    extract::{Path, State},
    routing::{delete, post},
    Json, Router,
};

use crate::model::{ModelController, Ticket, TicketForCreate};

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/ticket/:id", delete(delete_ticket))
        .with_state(mc)
}

async fn create_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - create_ticket", "HANDLER");
    let ticket = mc.create_ticket(ctx, ticket_fc).await?;
    Ok(Json(ticket))
}

async fn list_tickets(mc: State<ModelController>, _ctx: Ctx) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<12} - list_tickets", "HANDLER");
    let tickets = mc.list_tickets().await?;
    Ok(Json(tickets))
}

async fn delete_ticket(
    mc: State<ModelController>,
    ctx: Ctx,
    Path(id): Path<u32>,
) -> Result<Json<()>> {
    println!(">>> {:<12} - delete_ticket", "HANDLER");
    mc.delete_ticket(ctx, id).await?;
    Ok(Json(()))
}
