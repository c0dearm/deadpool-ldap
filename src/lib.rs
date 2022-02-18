use async_trait::async_trait;
use ldap3::{exop::WhoAmI, Ldap, LdapConnAsync, LdapConnSettings, LdapError};

pub struct Manager(String, LdapConnSettings);
pub type Pool = deadpool::managed::Pool<Manager>;

/// LDAP Manager for the `deadpool` managed connection pool.
impl Manager {
    /// Creates a new manager with the given URL.
    /// URL can be anything that can go Into a String (e.g. String or &str)
    pub fn new<S: Into<String>>(ldap_url: S) -> Self {
        Self(ldap_url.into(), LdapConnSettings::new())
    }

    /// Set a custom LdapConnSettings object on the manager.
    /// Returns a copy of the Manager.
    pub fn with_connection_settings(mut self, settings: LdapConnSettings) -> Self {
        self.1 = settings;
        self
    }
}

#[async_trait]
impl deadpool::managed::Manager for Manager {
    type Type = Ldap;
    type Error = LdapError;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        let (conn, ldap) = LdapConnAsync::with_settings(self.1.clone(), &self.0).await?;
        #[cfg(feature = "default")]
        ldap3::drive!(conn);
        #[cfg(feature = "rt-actix")]
        actix_rt::spawn(async move {
            if let Err(e) = conn.drive().await {
                log::warn!("LDAP connection error: {:?}", e);
            }
        });
        Ok(ldap)
    }
    async fn recycle(&self, conn: &mut Self::Type) -> deadpool::managed::RecycleResult<Self::Error> {
        conn.extended(WhoAmI).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use super::*;

    #[test]
    fn manager_new_sets_url() {
        let manager = Manager::new("my_url");

        assert_eq!(manager.0, "my_url");
    }

    // TODO figure out how to test the LdapConnSettings struct

    #[cfg(any(feature = "tls-native", feature = "tls-rustls"))]
    #[test]
    fn manager_new_sets_default_connection_settings() {
        let manager = Manager::new("my_url");
        let default = LdapConnSettings::new();

        assert_eq!(manager.1.starttls(), default.starttls());
    }

    fn is_manager(s: &dyn std::any::Any) -> bool {
        std::any::TypeId::of::<Manager>() == s.type_id()
    }

    #[test]
    fn manager_with_connection_settings_returns_manager() {
        let manager = Manager::new("my_url")
            .with_connection_settings(
                LdapConnSettings::new()
                    .set_conn_timeout(Duration::from_secs(30))
            );
        assert!(is_manager(&manager));
    }

    #[cfg(any(feature = "tls-native", feature = "tls-rustls"))]
    #[test]
    fn manager_with_connection_settings_updates_settings() {
        let manager = Manager::new("my_url")
            .with_connection_settings(
                LdapConnSettings::new()
                    .set_conn_timeout(Duration::from_secs(30))
                    .set_starttls(true)
            );
        let default = LdapConnSettings::new();

        assert_ne!(manager.1.starttls(), default.starttls());
    }
}
