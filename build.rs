use std::env;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn write(in_dir: &OsStr, out_file: &mut File, num_colours: u8) {
    let base_dir = Path::new(&in_dir).join(format!("base{num_colours}"));
    for entry in std::fs::read_dir(&base_dir).expect("dir exists") {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension() == Some(OsStr::new("yaml")) {
            let f_name = {
                let base = path
                    .as_path()
                    .file_name()
                    .expect("has filename")
                    .to_str()
                    .expect("to string");
                let mut a = base.split('.');
                a.next().unwrap()
            };
            let contents = std::fs::read_to_string(&path).expect("Unable to read base16 file");
            let yaml: serde_yaml::Value = serde_yaml::from_str(&contents).unwrap();
            let palette: Vec<[u8; 3]> = yaml
                .get("palette")
                .and_then(|v| v.as_mapping())
                .unwrap()
                .iter()
                .filter_map(|(_, v)| {
                    v.as_str().map(|i| {
                        let i = i.strip_prefix('#').expect("missing #");
                        let x = u32::from_str_radix(i.trim(), 16).unwrap().to_be_bytes();
                        [x[1], x[2], x[3]]
                    })
                })
                .collect();

            writeln!(
                out_file,
                "pub const BASE{}_{}: [[u8; 3]; {}] = {:?};",
                num_colours,
                f_name.replace('-', "_").to_uppercase(),
                num_colours,
                palette
            )
            .unwrap();
        }
    }
}

fn main() {
    let in_dir = env::var_os("THEME_DIR").unwrap_or("./tinted-schemes/".into());
    if !Path::new(&in_dir).exists() {
        let _ = Command::new("git")
            .args(["submodule", "update", "--init", "tinted-schemes"])
            .status();
    }

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("theme.rs");
    let mut f = File::create(&dest_path).unwrap();
    write(&in_dir, &mut f, 16);
    write(&in_dir, &mut f, 24);

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=tinted-schemes/");
}
