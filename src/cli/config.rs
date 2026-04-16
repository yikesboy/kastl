use crate::cli::error::AppError;
use crate::cli::util::with_spinner;
use crate::ha::HaClient;
use std::slice::from_ref;
use tabled::Table;

use tabled::settings::Style;

pub async fn handle(ha: &HaClient) -> Result<(), AppError> {
    retrieve_config(ha).await
}

async fn retrieve_config(ha: &HaClient) -> Result<(), AppError> {
    let ha_config = with_spinner("Retrieving config...", ha.get_config()).await?;
    let result_table = Table::new(from_ref(&ha_config))
        .with(Style::ascii_rounded())
        .to_string();
    println!("{}", result_table);

    let unitsystem_table = Table::new(from_ref(&ha_config.unit_system))
        .with(Style::ascii_rounded())
        .to_string();
    println!("{}", unitsystem_table);

    Ok(())
}
