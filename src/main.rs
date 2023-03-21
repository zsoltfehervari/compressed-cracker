use std::fs;
use std::ffi::OsStr;

//TODO: rar
// extern crate unrar;
// use unrar::Archive;

//TODO: pdf
// use pdf::file::File as PdfFile;
// use pdf::password::PdfPassword;

use bruteforce::BruteForce;
use bruteforce::charset::Charset;

// symbols: "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_+=~`[]{}|\\:;\'<>,.?/"

//extern crate rar;

fn main() {

    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() == 0 {
        println!("No Archive (.zip, .rar, .pdf) path specified!");
        return;
    };

    let f_name = std::path::Path::new(&*args[0]);
    if !f_name.exists() {
        return println!("File does not exist!");
    }

    const ALLOWED_EXTENSIONS: [&str; 3] = ["zip","rar","pdf"];
    let f_extension = f_name.extension().and_then(OsStr::to_str).unwrap();
    if !ALLOWED_EXTENSIONS.contains(&f_extension) {
        return println!("File extension is not allowed!");
    }

    const _CHARSET: Charset = Charset::new(&['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '+', '=', '~', '`', '[', ']', '{', '}', '|', '\\', ':', ';', '"','<', '>', ',', '.', '?', '/']);
    let brute_forcer = if args.len() > 1 { BruteForce::new_by_start_string(_CHARSET,args[1].clone())}
        else { BruteForce::new(_CHARSET) };

    let file = fs::File::open(f_name);

    // zip
    if f_extension == ALLOWED_EXTENSIONS[0] {

        let mut archive = zip::ZipArchive::new(file.unwrap()).unwrap();
        for s in brute_forcer {

            let file = archive.by_index_decrypt(0,s.as_bytes()).unwrap();

            match file {
                Ok(_v) => {
                    println!("================================================");
                    println!("Password found: {}", s);
                    println!("================================================");
                    break;
                },
                Err(_e) => println!("{}", s),
            };

        }

        // rar
    }else if  f_extension == ALLOWED_EXTENSIONS[1] {

        // for s in brute_forcer {
        //
        //     match Archive::with_password(String::from(&*args[0]),String::from(&s))
        //     {
        //         Ok(Archive) => {
        //             println!("================================================");
        //             println!("Password found: {}", &s);
        //             println!("================================================");
        //             break;
        //         },
        //         Err(Archive) => {
        //             println!("{}", &s);
        //         },
        //         _ => {
        //             println!("{}", &s);
        //         }
        //
        //     }
        //
        // }

        // pfd
    }else{

        // let pdf_file = PdfFile::new(file);
        //
        // for s in brute_forcer {
        //
        //     let pdf_password = PdfPassword::from_str(s);
        //     if pdf_file.check_password(&pdf_password)? {
        //
        //         println!("================================================");
        //         println!("Password found: {}", &s);
        //         println!("================================================");
        //         break;
        //
        //     }else{
        //         println!("{}", &s);
        //     }
        //
        //
        // }

    }




}
