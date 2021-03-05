pub fn init_log() -> anyhow::Result<()> {
    simple_log::quick().map_err(|e| anyhow!(e))?;
    Ok(())
}
