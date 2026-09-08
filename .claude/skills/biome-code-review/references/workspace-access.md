# Workspace Access Review

Check affected workspace, service, CLI/LSP, database, Salsa, and cancellation contracts only.

## Two Execution Models

The `Workspace` interface hides two database modes:

| Client | Storage model | Required behavior |
| --- | --- | --- |
| CLI | Shared, read-only after project scan | Workers read snapshots; filesystem writes happen outside the database |
| LSP | Owned, mutable | Reads are cancellable when a write is pending |

Verify these claims against the current constructors and call sites before citing them because workspace internals change.

## CLI

After scanning, per-file workers must not publish workspace state while others hold snapshots. Trace changed publishing calls' snapshot lifetimes and write ordering to establish reachable races or deadlocks; location alone is insufficient.

For changed synchronization, check read/write and write/write overlap against current execution contracts, not scheduling preferences.

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

These are candidates, not automatic findings. Establish reachability and affected behavior; grade by impact:

- CLI workers publishing state during parallel processing;
- LSP reads bypassing cancellation handling;
- cancellation converted into a panic or terminal error;
- a database read handle held across a write;
- a Salsa query omitting a dependency that can change its result.
