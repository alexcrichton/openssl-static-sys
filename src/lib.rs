use std::os;
use std::io::fs::PathExtensions;

/// Probe the system for the directory in which CA certificates should likely be
/// found.
///
/// This will only search known system locations.
pub fn find_certs_dir() -> Option<Path> {
    // see http://gagravarr.org/writing/openssl-certs/others.shtml
    [
        "/var/ssl",
        "/usr/share/ssl",
        "/usr/local/ssl",
        "/usr/local/openssl",
        "/usr/lib/ssl",
        "/usr/ssl",
        "/etc/openssl",
        "/etc/pki/tls",
        "/etc/ssl",
    ].iter().map(|s| Path::new(*s)).find(|p| {
        p.exists()
    })
}

pub fn init_ssl_cert_env_vars() {
    let certs_dir = match find_certs_dir() {
        Some(path) => path,
        None => return,
    };

    if certs_dir.join("cert.pem").exists() {
        match os::getenv("SSL_CERT_FILE") {
            // Someone else has already got this, they probably know what
            // they're doing more than we do
            Some(..) => {}

            None => os::setenv("SSL_CERT_FILE", certs_dir.join("cert.pem")),
        }
    }

    if certs_dir.join("certs").exists() {
        match os::getenv("SSL_CERT_DIR") {
            Some(..) => {}
            None => os::setenv("SSL_CERT_DIR", certs_dir.join("certs")),
        }
    }
}
