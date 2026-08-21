# Workspace Access Review

Load this reference for changes to workspace methods, service execution, CLI/LSP paths, database storage, Salsa queries, or cancellation.

## Two Execution Models

The `Workspace` interface hides two database modes:

| Client | Storage model | Required behavior |
| --- | --- | --- |
| CLI | Shared, read-only after project scan | Workers read snapshots; filesystem writes happen outside the database |
| LSP | Owned, mutable | Reads are cancellable when a write is pending |

Verify these claims against the current constructors and call sites before citing them because workspace internals change.

## CLI

After scanning, per-file workers must not publish workspace state while other workers hold snapshots. A new `change_file` or equivalent state-publishing call inside a worker, crawl, or parallel iterator is a high-severity race.

Server-backed synchronization, when present, must remain deferred until parallel workers finish and then run sequentially. Do not accept a change that moves synchronization into the crawl or parallelizes the deferred commit without a new architecture that proves safety.

## LSP Cancellation

Pending-write cancellation is normal control flow. Check that:

- read handlers run under the current cancellation boundary;
- cancellation maps to the editor's content-modified response or the established retry path;
- no new `unwrap`, panic, log-and-continue, or generic hard error intercepts cancellation;
- callers do not retain a database fork while initiating a write.

## Read, Resolve, Commit

A function that reads through a database fork and writes through the same database in one call stack can deadlock waiting for its own read handle. The safe shape is:

1. Extract owned input while holding the read fork.
2. Drop the fork by leaving its scope.
3. Resolve or transform the owned data.
4. Commit through the write API.

Search the current workspace implementation for the established example rather than relying on a historical function name.

## Review Severity

Treat these as high-severity correctness findings:

- CLI workers publishing state during parallel processing;
- LSP reads bypassing cancellation handling;
- cancellation converted into a panic or terminal error;
- a database read handle held across a write;
- a Salsa query omitting a dependency that can change its result.
