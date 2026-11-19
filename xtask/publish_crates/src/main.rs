use xtask_publish_crates::{command, run};

fn main() -> anyhow::Result<()> {
    run(command().fallback_to_usage().run())
}
