use {
    super::CERT_DIR,
    actix_web::dev::Server,
    futures::executor,
    std::{fs::File, io::Write, path::PathBuf, thread},
};

// NOTE: This code blindly requests a cert, but it really should check
// our existing cert to see when it expires.  Also, it should schedule
// checks and requests into the future.  However, I don't want to take
// the time to figure out the correct semantics for all of that.  I
// can revisit it later and possibly bundle it into
// actix-web-lets-encrypt (which is currently useless) or at least
// package it up in some way to share with others.  This code,
// however, is way to ugly to share.

#[must_use]
pub fn start(server: &Server) -> Option<(PathBuf, PathBuf)> {
    crate::acme_lib::request_cert()
        .ok()
        .and_then(|(pkey, cert)| {
            let key_path: PathBuf = [CERT_DIR, "devctm_com.key"].iter().collect();
            let mut key_file = File::create(&key_path).ok()?;
            key_file
                .write_all(&pkey.private_key_to_pem_pkcs8().ok()?)
                .ok()?;

            let cert_path: PathBuf = [CERT_DIR, "devctm_com.crt"].iter().collect();
            let mut cert_file = File::create(&cert_path).ok()?;
            cert_file.write_all(cert.certificate().as_bytes()).ok()?;

            let server = server.clone();
            thread::spawn(move || executor::block_on(server.stop(true)));
            Some((key_path, cert_path))
        })
}
