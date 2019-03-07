use {
    acme_client::{error::Error, Directory},
    actix_web::{self, fs::NamedFile, HttpRequest},
    std::path::PathBuf,
};

const HOST: &str = "devctm.com";
const NONCE_DIRECTORY: &str = "/var/devctm";

pub fn lets_encrypt_do_not_leave_in_place() -> Result<(), Error> {
    // let directory = Directory::lets_encrypt()?;
    let directory = Directory::from_url("https://acme-staging.api.letsencrypt.org/directory")?;
    let account = directory
        .account_registration()
        .email("ctm@devctm.com")
        .register()?;
    let authorization = account.authorization(HOST)?;
    let http_challenge = authorization
        .get_http_challenge()
        .ok_or("HTTP challenge not found")?;
    http_challenge.save_key_authorization(NONCE_DIRECTORY)?;
    http_challenge.validate()?;

    Ok(())
}

pub fn nonce(req: &HttpRequest) -> actix_web::Result<NamedFile> {
    let token: PathBuf = req.match_info().query("token")?;
    let mut path = PathBuf::from(NONCE_DIRECTORY);
    path.push(".well-known/acme-challenge");
    let path = path.join(token);
    Ok(NamedFile::open(path)?)
}
