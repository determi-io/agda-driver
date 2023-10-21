use fs_extra::dir::CopyOptions;
use fs_extra::copy_items;
use anyhow::Result;

// usage: agda-driver path-in path-out
fn main()
{
    let agda = std::env!("AGDA");

    let args: Vec<String> = std::env::args().collect();

    match args.len()
    {
        // 2 arguments
        3 =>
        {
            let path_in = &args[1];
            let path_out = &args[2];
            let result = copy_dir(path_in, path_out);
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
    // get all items in existing path
    // let from_paths = fs::read_dir(path_in).unwrap();


    //Initialize default values for CopyOptions
    let options = CopyOptions::new()
        .copy_inside(true);
        // .content_only(true);

    // copy all these items with `fs_extra` crate
    copy_items(&[from_path], to_path, &options)?;

    Ok(())
}


