use crate::config::error::ConfigError;
use crate::config::{SERVICE_IDENTIFIER, Secret, SecretIdentifier};
use keyring::Entry;
use strum::IntoEnumIterator;

pub trait SecretStorage {
    fn save(&self, secret: &Secret) -> Result<(), ConfigError>;
    fn load(&self, secret_identifier: SecretIdentifier) -> Result<Secret, ConfigError>;
    fn delete(&self, secret_identifier: SecretIdentifier) -> Result<(), ConfigError>;
    fn delete_all(&self) -> Result<(), ConfigError>;
}

pub struct KeyringSecretStorage;

impl SecretStorage for KeyringSecretStorage {
    fn save(&self, secret: &Secret) -> Result<(), ConfigError> {
        let entry = Entry::new(SERVICE_IDENTIFIER, &secret.identifier.to_string())?;
        entry.set_password(&secret.value)?;
        Ok(())
    }

    fn load(&self, secret_identifier: SecretIdentifier) -> Result<Secret, ConfigError> {
        let entry = Entry::new(SERVICE_IDENTIFIER, &secret_identifier.to_string())?;
        let value = entry.get_password()?;
        Ok(Secret {
            identifier: secret_identifier,
            value: value,
        })
    }

    fn delete(&self, secret_identifier: SecretIdentifier) -> Result<(), ConfigError> {
        let entry = Entry::new(SERVICE_IDENTIFIER, &secret_identifier.to_string())?;
        let deletion_result = entry.delete_credential();
        match deletion_result {
            Ok(_) => return Ok(()),
            Err(keyring::Error::NoEntry) => return Ok(()),
            Err(e) => return Err(ConfigError::Keyring(e)),
        }
    }

    fn delete_all(&self) -> Result<(), ConfigError> {
        SecretIdentifier::iter().try_for_each(|id| self.delete(id))
    }
}
