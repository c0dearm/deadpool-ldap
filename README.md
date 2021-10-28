# Deadpool for LDAP

Deadpool is a dead simple async pool for connections and objects
of any type.

This crate implements a [`deadpool`](https://crates.io/crates/deadpool)
manager for [`ldap3`](https://crates.io/crates/ldap3)

## Features

| Feature      | Description                                                 | Default |
| -------------| ----------------------------------------------------------- | ------- |
| `tls-native` | Enable support for TLS connections using `tokio-native-tls` | no      |
| `tls-rustls` | Enable support for TLS connections using `tokio-rustls`     | no      |

## Basic usage

```rust,ignore
use deadpool_ldap::{Manager, Pool};

#[tokio::main]
async fn main() {
    let manager = Manager::new("ldap://example.org");
    let pool = Pool::new(manager, 5);

    let mut client = pool.get().await.unwrap();
    result = client.simple_bind("uid=user,dc=example,dc=org", "password").await;
    assert!(result.is_ok());
}
```

### Sending a custom LdapConnSettings

To send custom ldap connection settings use .with_connection_settings() on the manager.

```rust,ignore
use deadpool_ldap::{Manager, Pool};
use ldap3::LdapConnSettings;

#[tokio::main]
async fn main() {
    let manager = Manager::new("ldap://example.org")
        .with_connection_settings(
            LdapConnSettings::new()
                .set_conn_timeout(Duration::from_secs(30))
        );
    let pool = Pool::new(manager, 5);

    let mut client = pool.get().await.unwrap();
    result = client.simple_bind("uid=user,dc=example,dc=org", "password").await;
    assert!(result.is_ok());
}
```

## License

Licensed under either of

* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
* APACHE 2.0 license ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

Choose at your option!
