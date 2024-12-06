/*
    [WEB (IPC)]       -> our REST, handler, middleware
 [CONTENT] [EVENT]
      [MODEL]           
      [STORE]

*/

//! Simplistic model layer
//! (with mock-store layer)

use crate::{ctx::{self, Ctx}, Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// region:    --- Ticket Types
#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub cid : u64, // creater user_id
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}
// endregion: --- Ticket Types

// region:    --- Model Controller
// this should be a db conexion, using for example sqlx, or diesel, but for now we will use a simple mock-store
#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>, // we are going to use the vector id as a sequential id, that doesnt work in production
                                    // ^ ONLY for quick local prototype. will grow infinitely
}

// Constructor
impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }
}

//CRUD implementation
impl ModelController {
    pub async fn create_ticket(
        &self,
        ctx: Ctx,
        ticket_fc: TicketForCreate
    ) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let id = store.len() as u64;
        let ticket = Ticket {
            id,
            cid: ctx.user_id(),
            title: ticket_fc.title,
        };
        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    pub async fn list_tickets(&self, _ctx: Ctx) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();

        let tickets = store.iter().filter_map(|t| t.clone()).collect();
        
        Ok(tickets)
    }

    pub async fn delete_ticket(&self, id: u64, _ctx: Ctx) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|t| t.take()); 

        ticket.ok_or(Error::TicketDeleteFailIdNotFound { id })
    }
}

// endregion: --- Model Controller