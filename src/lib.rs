pub mod errors;

use async_trait::async_trait;
use ldap3::{exop::WhoAmI, Ldap, LdapConnAsync, LdapError as Error};

pub type Pool = deadpool::managed::Pool<Ldap, errors::LdapError>;
pub struct Manager(pub &'static str);

#[async_trait]
impl deadpool::managed::Manager<Ldap, Error> for Manager {
    async fn create(&self) -> Result<Ldap, Error> {
        let (conn, ldap) = LdapConnAsync::new(self.0).await?;
        #[cfg(feature = "default")]
        ldap3::drive!(conn);
        #[cfg(feature = "rt-actix")]
        actix_rt::spawn(async move {
            if let Err(e) = conn.drive().await {
                log::warn!("LDAP connection error: {}", e);
            }
        });
        Ok(ldap)
    }
    async fn recycle(&self, conn: &mut Ldap) -> deadpool::managed::RecycleResult<Error> {
        conn.extended(WhoAmI).await?;
        Ok(())
    }
}
