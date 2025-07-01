use std::collections::HashSet;
use std::env;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;

use cliclack::confirm;
use rust_translate::supported_languages::get_languages;
use rust_translate::translate;
use serde_json::Value;
use serde_json::from_str;

#[tokio::main]
async fn main() {
    if let Err(e) = sync_languages().await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

async fn sync_languages() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: transl <header> <key> <value> <current_lang>");
        std::process::exit(2);
    }

    let supported_langs: HashSet<&str> = get_languages().into_iter().collect();

    let mut found_langs: Vec<String> = Vec::new();
    let paths = std::fs::read_dir(std::env::current_dir()?)?;

    let mut base_lang_found = false;

    for path in paths {
        let path = path?.path(); // PathBuf

        match path.file_stem().and_then(|s| s.to_str()) {
            Some(stem) if supported_langs.contains(stem) => {
                found_langs.push(stem.to_owned());
                if stem == args[4] {
                    base_lang_found = true;
                }
                println!("Found lang: {}", path.display())
            }
            Some(_) => println!(
                "unsupported language: {}. Use two-letter codes like 'en'",
                path.display()
            ),
            None => println!("invalid file: {}", path.display()),
        }
    }

    if !base_lang_found {
        return Err(format!("Could not find base language {} in dir", args[4]).into());
    }

    // traverse all files
    for lang in found_langs {
        let file = std::env::current_dir()?.join(format!("{lang}.json"));

        let mut f = OpenOptions::new().read(true).write(true).open(file)?;
        let mut buf = String::new();
        f.read_to_string(&mut buf)?;
        let mut j: Value = from_str(&buf)?;

        if let Some(obj) = j.get_mut(&args[1]).and_then(Value::as_object_mut) {
            // compute the new value from args[3]
            let val = if lang == args[4] {
                args[3].clone()
            } else {
                translate(&args[3], &args[4], &lang)
                    .await
                    .expect("translation failed")
            };
            // insert under key from args[2]
            obj.insert(args[2].clone(), Value::String(val));

            f.set_len(0)?;
            f.seek(SeekFrom::Start(0))?;
            f.write_all(serde_json::to_string_pretty(&j)?.as_bytes())?;
        } else {
            let prompt = format!("The header was not found in {lang}, do you want to abbort");
            let proceed = confirm(&prompt).interact()?;

            if proceed {
                println!("Process abborted.");
                break;
            } else {
                println!("Process continued.");
            }
        }
    }

    Ok(())
}
