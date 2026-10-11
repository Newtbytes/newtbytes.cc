use std::{
    env,
    fs::{self, read_to_string},
    io,
    path::{Path, PathBuf},
};

use pulldown_cmark::{Options, Parser, html};
use walkdir::WalkDir;

fn build_page(page: String) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&page, options);

    let mut output = String::new();
    pulldown_cmark::html::push_html(&mut output, parser);

    output
}

#[derive(Debug)]
enum BuildError {
    IoError(String, io::Error),
    CannotOutput(String),
}

fn get_abs<P: AsRef<Path>>(path: P) -> PathBuf {
    if let Ok(root) = env::current_dir() {
        root.join(path)
    } else {
        path.as_ref().to_owned()
    }
}

struct BuildContext {
    src_dir: PathBuf,
    build_dir: PathBuf,
}

impl BuildContext {
    fn new(src_dir: PathBuf, build_dir: PathBuf) -> Self {
        Self { src_dir, build_dir }
    }

    /// Get the absolute path of a file relative to the source directory
    fn abs_src_path<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        get_abs(self.src_dir.join(path))
    }

    /// Get the absolute path of a file relative to the build directory
    fn abs_build_path<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        get_abs(self.build_dir.join(path))
    }
}

fn build_file<P: AsRef<Path>>(page: P) -> Result<String, BuildError> {
    let page =
        read_to_string(page).map_err(|e| BuildError::IoError("Cannot build file".to_owned(), e))?;
    Ok(build_page(page))
}

fn process_file<P: AsRef<Path>>(ctx: &BuildContext, page: P) -> Result<(), BuildError> {
    if page.as_ref().is_absolute() {
        return Err(BuildError::CannotOutput(
            "Cannot build a source file with an absolute path. The target location to build to cannot be deduced.".to_owned(),
        ));
    }

    let ext = {
        let ext = page.as_ref().extension();
        let ext = ext.unwrap_or_default().to_str();
        ext.unwrap_or_default()
    };

    let (output, ext) = match ext {
        "md" => (build_file(ctx.abs_src_path(&page))?, "html"),
        _ => (
            read_to_string(ctx.abs_src_path(&page))
                .map_err(|e| BuildError::IoError("Could not read source file".to_owned(), e))?,
            page.as_ref()
                .extension()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default(),
        ),
    };

    let output_path = ctx.abs_build_path(&page).with_extension(ext);

    if let Some(parent) = output_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)
                .map_err(|e| BuildError::IoError("Failed to create directory".to_owned(), e))?;
        }
    }

    fs::write(output_path, output)
        .map_err(|e| BuildError::IoError("Could not write source file".to_owned(), e))?;

    Ok(())
}

fn process_dir(src_dir: PathBuf, build_dir: PathBuf) -> Result<(), BuildError> {
    let ctx = BuildContext::new(src_dir.clone(), build_dir);

    for entry in WalkDir::new(src_dir) {
        match entry {
            Ok(entry) => {
                if entry.path().is_dir() {
                    continue;
                }

                println!("{}", entry.path().display());

                let page = PathBuf::from_iter(entry.path().components().skip(1));

                if let Err(err) = process_file(&ctx, page) {
                    println!("{:?}", err);
                }
            }
            Err(err) => {
                println!("{err}");
            }
        }
    }

    Ok(())
}

fn main() {
    let mut args = env::args();
    let src_dir = args.nth(1).expect("Should contain a source directory");
    let build_dir = args.nth(2).unwrap_or("build".to_owned());

    if let Err(err) = process_dir(src_dir.into(), build_dir.into()) {
        println!("{:?}", err);
    }
}
