use {
    rsass::{self, compile_scss_file, OutputStyle},
    std::{
        collections::HashMap,
        convert::{AsRef, From},
        env,
        fs::{self, File},
        io::{self, Write},
        path::{self, Path, PathBuf},
    },
    walkdir::{self, WalkDir},
};

#[derive(Debug)]
enum Error {
    Io(io::Error),
    WalkDir(walkdir::Error),
    Rsass(rsass::Error),
    StripPrefix(path::StripPrefixError),
    EnvVar(env::VarError),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<walkdir::Error> for Error {
    fn from(error: walkdir::Error) -> Self {
        Error::WalkDir(error)
    }
}

impl From<rsass::Error> for Error {
    fn from(error: rsass::Error) -> Self {
        Error::Rsass(error)
    }
}

impl From<path::StripPrefixError> for Error {
    fn from(error: path::StripPrefixError) -> Self {
        Error::StripPrefix(error)
    }
}

impl From<env::VarError> for Error {
    fn from(error: env::VarError) -> Self {
        Error::EnvVar(error)
    }
}

struct StaticBuilder {
    output: PathBuf,
    scss: PathBuf,
    verbatim: PathBuf,
    verbatim_path: PathBuf,
    local_from_external: HashMap<String, String>,
}

impl StaticBuilder {
    pub fn create<O, S, I>(output: O, scss: S, verbatim: I) -> Result<(), Error>
    where
        O: AsRef<Path>,
        S: AsRef<Path>,
        I: AsRef<Path>,
    {
        let output = output.as_ref().to_path_buf();
        let scss = scss.as_ref().to_path_buf();
        let verbatim = verbatim.as_ref().to_path_buf();
        let mut verbatim_path = env::current_dir()?.to_path_buf();
        verbatim_path.push("app/assets/verbatim");
        let mut builder = Self {
            output,
            scss,
            verbatim,
            verbatim_path,
            local_from_external: HashMap::new(),
        };
        builder.create_css()?;
        builder.find_verbatims()?;
        builder.write_output()
    }

    fn create_css(&mut self) -> Result<(), Error> {
        let walker = WalkDir::new(&self.scss).into_iter();

        for entry in walker.filter_entry(|e| {
            !e.file_type().is_file() || e.file_name().to_string_lossy().ends_with(".css.scss")
        }) {
            let entry = entry?;

            if entry.file_type().is_file() {
                let full_path = entry.path();
                let css = compile_scss_file(&full_path, OutputStyle::Compressed)?;
                let mut relative_path = full_path.strip_prefix(&self.scss)?.to_path_buf();
                let stem = relative_path.file_stem().unwrap().to_os_string();
                relative_path.set_file_name(stem);
                let relative_string = relative_path.to_str().unwrap().to_string();

                let mut css_path = PathBuf::from("css");
                css_path = css_path.join(&relative_path);
                self.local_from_external
                    .insert(relative_string, css_path.to_str().unwrap().to_string());

                let mut out_path = self.output.to_path_buf();
                out_path.push("css");

                out_path = out_path.join(relative_path);

                if let Some(directories) = out_path.parent() {
                    // We'll repeatedly be trying to recreate pre-existing
                    // directories, but it doesn't seem like keeping a set
                    // of what we've created around as worth it, although
                    // that may just be my newness to Rust.  TODO: contemplate
                    fs::create_dir_all(directories)?;
                }
                let mut file = File::create(&out_path)?;
                file.write_all(&css)?;
            }
        }
        Ok(())
    }

    fn find_verbatims(&mut self) -> Result<(), Error> {
        for entry in WalkDir::new(&self.verbatim) {
            let entry = entry?;

            if entry.file_type().is_file() {
                let full_path = entry.path();
                let relative_path = full_path.strip_prefix(&self.verbatim)?.to_path_buf();
                let relative_string = relative_path.to_str().unwrap().to_string();
                let mut verbatim_path = self.verbatim_path.clone();
                verbatim_path = verbatim_path.join(relative_path);
                self.local_from_external
                    .insert(relative_string, verbatim_path.to_str().unwrap().to_string());
            }
        }
        Ok(())
    }

    fn write_output(&self) -> Result<(), Error> {
        let mut out_path = self.output.to_path_buf();
        out_path.push("statics.rs");
        let mut file = File::create(&out_path)?;
        file.write_all(
            br"use {
    phf:: {
        self,
        Map,
        phf_map,
    },
};

static STATICS: Map<&'static str, &'static [u8]> = phf_map! {
",
        )?;

        for (external, local) in &self.local_from_external {
            writeln!(
                file,
                r#"    "{}" => include_bytes!("{}"),"#,
                external, local
            )?;
        }

        file.write_all(
            br"};
",
        )?;
        Ok(())
    }
}

fn main() -> Result<(), Error> {
    let output_path = PathBuf::from(env::var("OUT_DIR")?);

    StaticBuilder::create(output_path, "app/assets/stylesheets", "app/assets/verbatim")
}
