use crate::config::error::ConfigError;
use crate::config::{SERVICE_IDENTIFIER, Secret, SecretIdentifier};
use keyring::Entry;
use strum::IntoEnumIterator;
use tokio::task::spawn_blocking;

pub trait SecretStorage {
    async fn save(&self, secret: &Secret) -> Result<(), ConfigError>;
    async fn load(&self, secret_identifier: SecretIdentifier) -> Result<Secret, ConfigError>;
    async fn delete(&self, secret_identifier: SecretIdentifier) -> Result<(), ConfigError>;
    async fn delete_all(&self) -> Result<(), ConfigError>;
}

pub struct KeyringSecretStorage;

impl SecretStorage for KeyringSecretStorage {
    async fn save(&self, secret: &Secret) -> Result<(), ConfigError> {
        let identifier = secret.identifier.clone();
        let value = secret.value.clone();
        spawn_blocking(move || {
            let entry = Entry::new(SERVICE_IDENTIFIER, &identifier.to_string())?;
            entry.set_password(&value)?;
            Ok(())
        }).await?
    }

    async fn load(&self, secret_identifier: SecretIdentifier) -> Result<Secret, ConfigError> {
        spawn_blocking(move || {
            let entry = Entry::new(SERVICE_IDENTIFIER, &secret_identifier.to_string())?;
            let value = entry.get_password()?;
            let secret = Secret { identifier: secret_identifier, value: value };
            Ok(secret)
        }).await?
    }

    async fn delete(&self, secret_identifier: SecretIdentifier) -> Result<(), ConfigError> {
        spawn_blocking(move || {
            let entry = Entry::new(SERVICE_IDENTIFIER, &secret_identifier.to_string())?;
            let deletion_result = entry.delete_credential();
            match deletion_result {
                Ok(_) => return Ok(()),
                Err(keyring::Error::NoEntry) => return Ok(()),
                Err(e) => return Err(ConfigError::Keyring(e)),
            }
        }).await?
    }

    async fn delete_all(&self) -> Result<(), ConfigError> {
        for id in SecretIdentifier::iter() {
            self.delete(id).await?;
        }
        Ok(())
    }
}
