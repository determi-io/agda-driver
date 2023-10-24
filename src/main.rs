use std::{ffi::OsStr, process::exit};
use std::path::Path;
use std::process::Command;

use fs_extra::dir::CopyOptions;
use fs_extra::copy_items;
use anyhow::{Result, anyhow};
use walkdir::WalkDir;

// usage: agda-driver path-in path-out
fn main()
{


    /////////////////////////////////////////////
    // parse environment vars
    let agda = std::env!("AGDA");
    let agda_includes = match std::env::var("AGDA_INCLUDES")
    {
        Ok(agda_includes) => {
            agda_includes
                .split(":")
                .map(|s| String::from(s))
                .collect()
        },
        Err(_) => Vec::new(),
    };


    /////////////////////////////////////////////
    // match args
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
                .and_then(|_| agda_build(agda, &path_out, agda_includes));
            match result
            {
                Ok(()) => (),
                Err(e) =>
                {
                    eprintln!("Couldnt copy. Error: {e}");
                    exit(-1);
                }
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

fn agda_build(agda: &str, root: &str, includes: Vec<String>) -> Result<()>
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

                    // prepare command to run
                    let mut cmd = Command::new(agda);

                    // add all includes
                    for include in &includes
                    {
                        cmd.arg("-i").arg(include);
                    }

                    // add all other arguments and run
                    let status = cmd.arg("--transliterate")
                        .arg(entry.path().to_str().unwrap())
                        .current_dir(root)
                        .status()
                        .expect("failed to run agda!");

                    // check result
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



