pub fn forward(record: &log::Record) {
    tracing::info!("{}", record.args());
}
