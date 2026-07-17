# Building a Compile-Time Query Engine: Lessons from Three Years of Rust

*Published on February 18, 2026 by Sarah Chen*
*Tags: rust, databases, compilers, performance*

When we started building Meridian, our embedded analytics engine, we chose Rust
for the usual reasons: performance, safety, and a type system that catches bugs
before they reach production. What we did not expect was how profoundly Rust's
ownership model would reshape our entire query architecture.

## The Problem: Analytical Queries at the Edge

Modern applications increasingly need to run complex analytical queries close to
the user. We wanted a query engine that could be **embedded directly into
application processes**, handling thousands of queries per second with
predictable tail latencies.

The challenge was not just speed. We needed the engine to be **safe for
multi-tenant environments**, where a poorly formed query from one tenant must
never crash the process or leak data to another. Traditional C++ engines give
you the speed but make safety a constant battle. Garbage-collected runtimes give
you safety but introduce unpredictable pause times.

Rust offered a third path.

> "The best code is code that the compiler refuses to let you write incorrectly."
> -- Attributed to various Rust advocates

## Architecture Overview

Our engine breaks query execution into four phases, each leveraging Rust's type
system for correctness guarantees.

### Phase 1: Parsing and Validation

Queries arrive as a SQL-like DSL. The parser converts raw text into a typed AST
where every node carries schema information.

```rust
pub enum Expr<'schema> {
    Column(ColumnRef<'schema>),
    Literal(ScalarValue),
    BinaryOp {
        left: Box<Expr<'schema>>,
        op: BinaryOperator,
        right: Box<Expr<'schema>>,
    },
    Aggregate {
        func: AggregateFunc,
        arg: Box<Expr<'schema>>,
        distinct: bool,
    },
}
```

The lifetime parameter `'schema` ties every expression back to the schema
registry that was active when the query was parsed. If the schema changes, all
outstanding parsed queries are invalidated automatically.

### Phase 2: Planning and Optimization

The planner transforms the validated AST into a logical plan, then applies
optimization rules. We borrowed ideas from both the [Cascades
framework][cascades] and [adaptive query processing][adaptive-qp].

Our optimizer applies these rules in order:

1. **Predicate pushdown** -- Move filters close to the data source
2. **Projection pruning** -- Remove unreferenced columns
3. **Join reordering** -- Dynamic programming for small plans, greedy otherwise
4. **Common subexpression elimination** -- Evaluate repeated computations once
5. **Aggregate splitting** -- Partial and final stages for parallel execution

```python
# Pseudocode for a rewrite rule
def pushdown_filter(plan):
    if isinstance(plan, Filter) and isinstance(plan.child, Join):
        join = plan.child
        left_cols = join.left.output_columns()
        if plan.predicate.references().issubset(left_cols):
            return Join(
                Filter(plan.predicate, join.left),
                join.right,
                join.condition
            )
    return plan
```

---

### Phase 3: Code Generation

Rather than interpreting the query plan at runtime, we generate specialized code
and compile it using **Cranelift** as a JIT backend. A query like
`SUM(price * quantity) WHERE region = 'EU'` becomes a tight loop with no
interpretation overhead, no virtual dispatch, and no hash table lookups.

```rust
fn execute_query(batch: &RecordBatch) -> Result<ScalarValue> {
    let price = batch.column_f64("price")?;
    let quantity = batch.column_f64("quantity")?;
    let region = batch.column_str("region")?;

    let mut sum: f64 = 0.0;
    for i in 0..batch.len() {
        if region.value(i) == "EU" {
            sum += price.value(i) * quantity.value(i);
        }
    }
    Ok(ScalarValue::Float64(sum))
}
```

> **Note:** JIT compilation adds roughly 2-5ms of upfront latency per query.
> For simple point lookups, we skip JIT and use an interpreter instead.

### Phase 4: Execution and Resource Management

The final phase executes compiled queries against columnar data inspired by
[Apache Arrow][arrow]. Each query gets an `Arc<Snapshot>` of the data. Snapshots
are immutable, so multiple queries read from the same memory simultaneously.
This gives us **snapshot isolation for free** with no GC pauses.

---

## Performance Results

Here is where Meridian stands on TPC-H at scale factor 10, single node:

- **Q1** (pricing summary): 45ms -- *limited by memory bandwidth*
- **Q6** (forecasting revenue): 12ms -- *benefits most from JIT*
- **Q9** (product profit): 230ms -- *dominated by hash table construction*
- **Q19** (discounted revenue): 38ms -- *predicate pushdown eliminates 97% of rows*

The numbers that matter most are the **tail latencies**. At p99, our query times
are within 1.3x of the median. No GC pauses, no JIT deoptimization cliffs.

## What We Would Do Differently

### Lesson 1: Start with `Arc` Everywhere

We tried to use references and lifetimes for everything, leading to **lifetime
parameter explosion**. We now use `Arc` for anything crossing a module boundary
and reserve lifetimes for hot paths within a single module.

### Lesson 2: Invest in Error Types Early

Our initial error type was a single enum with thirty variants. We should have
designed a **layered error architecture** from day one. We eventually adopted
`thiserror` for defining errors and `miette` for rendering them.

### Lesson 3: Property-Based Testing is Non-Negotiable

**Property-based testing** with `proptest` catches bugs no human would write
tests for -- obscure interactions between null handling, overflow semantics, and
Unicode collation.

```javascript
const query = generateRandomQuery({
  tables: schema.tables,
  maxJoins: 3,
  maxPredicates: 5,
  allowNulls: true,
});

const meridianResult = await meridian.execute(query);
const sqliteResult = await sqlite.execute(query);
assert.deepEqual(meridianResult, sqliteResult);
```

## Looking Ahead

We are working on **distributed query execution**, splitting large queries
across multiple nodes using Rust's async ecosystem with `tokio`. The community
around database engineering in Rust is thriving, and there has never been a
better time to start.

---

*Thanks to Alex Rivera, Priya Sharma, and the Meridian team for reviewing
drafts of this post.*

![Meridian query execution pipeline](https://meridian.dev/images/pipeline-diagram.png)

*The Meridian query pipeline, from SQL text to native machine code.*

---

## Further Reading

- Graefe, G. "The Cascades Framework for Query Optimization." *IEEE Data
  Engineering Bulletin*, 1995.
- Neumann, T. "Efficiently Compiling Efficient Query Plans for Modern
  Hardware." *VLDB*, 2011.

[cascades]: https://www.cse.iitb.ac.in/infolab/Data/Courses/CS632/2006/Papers/Cascades-Graefe.pdf
[adaptive-qp]: https://doi.org/10.1561/1900000026
[arrow]: https://arrow.apache.org/
[vec-eval]: https://meridian.dev/blog/vectorized-evaluation
[rustconf-talk]: https://www.youtube.com/watch?v=dQw4w9WgXcQ
