use std::os;
use std::io::fs::PathExtensions;

/// Probe the system for the directory in which CA certificates should likely be
/// found.
///
/// This will only search known system locations.
pub fn find_certs_dirs() -> Vec<Path> {
    // see http://gagravarr.org/writing/openssl-certs/others.shtml
    [
        "/var/ssl",
        "/usr/share/ssl",
        "/usr/local/ssl",
        "/usr/local/openssl",
        "/usr/local/share",
        "/usr/lib/ssl",
        "/usr/ssl",
        "/etc/openssl",
        "/etc/pki/tls",
        "/etc/ssl",
    ].iter().map(|s| Path::new(*s)).filter(|p| {
        p.exists()
    }).collect()
}

pub fn init_ssl_cert_env_vars() {
    for certs_dir in find_certs_dirs().iter() {
        // cert.pem looks to be an openssl 1.0.1 thing, while
        // certs/ca-certificates.crt appears to be a 0.9.8 thing
        try("SSL_CERT_FILE", certs_dir.join("cert.pem"));
        try("SSL_CERT_FILE", certs_dir.join("certs/ca-certificates.crt"));
        try("SSL_CERT_FILE", certs_dir.join("certs/ca-root-nss.crt"));

        try("SSL_CERT_DIR", certs_dir.join("certs"));
    }
}

fn try(var: &str, val: Path) {
    if !val.exists() { return }
    match os::getenv(var) {
        // Someone else has already got this, they probably know what
        // they're doing more than we do
        Some(..) => {},
        None => os::setenv(var, val),
    }
}
