use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;

use fs_extra::dir::CopyOptions;
use fs_extra::copy_items;
use anyhow::{Result, anyhow};
use walkdir::WalkDir;

// usage: agda-driver path-in path-out
fn main()
{
    let agda = std::env!("AGDA");

    let args: Vec<String> = std::env::args().collect();

    match args.len()
    {
        // 1 arguments
        2 =>
        {
            let path_in = &args[1];
            // let path_out = &args[2];
            let path_out_maybe = std::env::var("out");

            let path_out = match path_out_maybe
            {
                Ok(x) => x,
                Err(e) =>
                {
                    eprintln!("err {e}");
                    return;
                }
            };

            let result = copy_dir(path_in, &path_out)
                .and_then(|_| agda_build(agda, &path_out));
            match result
            {
                Ok(()) => (),
                Err(e) => eprintln!("Couldnt copy. Error: {e}")
            }
        }

        // wrong number of args
        _ =>
        {
            eprintln!("Wrong number of args!");
        }
    }

    println!("Hello, world!");
    println!("agda is: {agda}");
}

fn copy_dir(from_path: &str, to_path: &str) -> Result<()>
{
    //Initialize default values for CopyOptions
    let options = CopyOptions::new()
        .copy_inside(true);

    // copy all these items with `fs_extra` crate
    copy_items(&[from_path], to_path, &options)?;

    Ok(())
}

fn agda_build(agda: &str, root: &str) -> Result<()>
{
    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let x = entry.file_name();
        // let x = x.to_string_lossy();
        let a = Path::new(&x).extension().and_then(OsStr::to_str);

        if let Some(a) = a
        {
            println!("got file {a}");

            if let Some(x) = entry.path().extension()
            {
                if x.to_str().unwrap_or_default() == "agda"
                {
                    println!("running agda!");
                    let status = Command::new(agda)
                        .arg(entry.path().to_str().unwrap())
                        .current_dir(root)
                        .status()
                        .expect("failed to run agda!");
                    if !status.success()
                    {
                        return Err(anyhow!("typechecking failed."))
                    }
                }
            }
        }

    }
    Ok(())
}



