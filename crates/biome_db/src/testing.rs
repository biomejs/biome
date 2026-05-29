//! Test helpers for verifying Salsa incremental computation.
//!
//! Use [`Events`] to capture Salsa events in a test database, then use the
//! assertion helpers to check whether a tracked function was recomputed.
//!
//! # Setup
//!
//! Wire [`Events`] into your `TestDb` via the `Storage::new` callback:
//!
//! ```ignore
//! use biome_db::testing::Events;
//!
//! struct TestDb {
//!     storage: salsa::Storage<Self>,
//!     events: Events,
//! }
//!
//! impl TestDb {
//!     fn new() -> Self {
//!         let events = Events::default();
//!         Self {
//!             storage: salsa::Storage::new(Some(Box::new({
//!                 let events = events.clone();
//!                 move |event| { events.0.lock().unwrap().push(event); }
//!             }))),
//!             events,
//!         }
//!     }
//!
//!     fn take_salsa_events(&mut self) -> Vec<salsa::Event> {
//!         std::mem::take(&mut *self.events.0.lock().unwrap())
//!     }
//!
//!     fn clear_salsa_events(&mut self) { self.take_salsa_events(); }
//! }
//! ```
//!
//! # Usage
//!
//! ```ignore
//! // Run the query once to populate the cache
//! let _model = js_semantic_model(&db, file);
//!
//! // Mutate an input
//! salsa::Setter::to(file.set_parsed(&mut db), new_parsed);
//!
//! // Clear events, run the query again, then check
//! db.clear_salsa_events();
//! let _model = js_semantic_model(&db, file);
//! let events = db.take_salsa_events();
//!
//! assert_function_query_was_run(&db, js_semantic_model, file, &events);
//! ```

use salsa::Event;
use std::sync::{Arc, Mutex};

/// Shared event log for capturing [`salsa::Event`]s during test execution.
///
/// Clone this into the `Storage::new` callback so events are recorded
/// automatically. Drain with `take()` or `clear()` via your `TestDb` methods.
#[derive(Default, Clone)]
pub struct Events(pub Arc<Mutex<Vec<Event>>>);

/// Assert that a tracked function **was** recomputed for the given input.
///
/// Pass the tracked function by name (e.g. `js_semantic_model`) and the
/// Salsa input it was called with. Panics if no matching `WillExecute`
/// event is found.
pub fn assert_function_query_was_run<Db, Q, QDb, I, R>(
    db: &Db,
    query: Q,
    input: I,
    events: &[Event],
) where
    Db: salsa::Database,
    Q: Fn(QDb, I) -> R,
    I: salsa::plumbing::AsId + std::fmt::Debug + Copy,
{
    let id = input.as_id();
    let (query_name, will_execute_event) = find_will_execute_event(db, query, input, events);
    assert!(
        will_execute_event.is_some(),
        "Expected query {query_name}({id:?}) to have run but it did not:\n{events:#?}"
    );
}

/// Assert that a tracked function **was not** recomputed for the given input.
///
/// This succeeds when Salsa either returned a memoized result (input unchanged)
/// or applied early termination (input changed but output was `Eq`-equal,
/// so downstream queries were skipped).
pub fn assert_function_query_was_not_run<Db, Q, QDb, I, R>(
    db: &Db,
    query: Q,
    input: I,
    events: &[Event],
) where
    Db: salsa::Database,
    Q: Fn(QDb, I) -> R,
    I: salsa::plumbing::AsId + std::fmt::Debug + Copy,
{
    let id = input.as_id();
    let (query_name, will_execute_event) = find_will_execute_event(db, query, input, events);
    if let Some(will_execute_event) = will_execute_event {
        panic!(
            "Expected query {query_name}({id:?}) not to have run but it did: {will_execute_event:?}\n\n{events:#?}"
        );
    }
}

fn find_will_execute_event<'a, Q, I>(
    db: &dyn salsa::Database,
    query: Q,
    input: I,
    events: &'a [Event],
) -> (&'static str, Option<&'a Event>)
where
    I: salsa::plumbing::AsId,
{
    let query_name = query_name(&query);
    let event = events.iter().find(|event| {
        if let salsa::EventKind::WillExecute { database_key } = event.kind {
            db.ingredient_debug_name(database_key.ingredient_index()) == query_name
                && database_key.key_index() == input.as_id()
        } else {
            false
        }
    });
    (query_name, event)
}

fn query_name<Q>(_query: &Q) -> &'static str {
    let full_qualified_query_name = std::any::type_name::<Q>();
    full_qualified_query_name
        .rsplit_once("::")
        .map_or(full_qualified_query_name, |(_, name)| name)
}
