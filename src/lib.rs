use async_trait::async_trait;
use ldap3::{exop::WhoAmI, Ldap, LdapConnAsync, LdapError};

pub struct Manager(String);
pub type Pool = deadpool::managed::Pool<Manager>;

impl Manager {
    pub fn new<S: Into<String>>(ldap_url: S) -> Self {
        Self(ldap_url.into())
    }
}

#[async_trait]
impl deadpool::managed::Manager for Manager {
    type Type = Ldap;
    type Error = LdapError;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        let (conn, ldap) = LdapConnAsync::new(&self.0).await?;
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
