use anyhow::{anyhow, Result};
use std::{path::Path, process::Command};
pub mod acm_rand;

fn compile(std_exe: impl AsRef<Path>) -> Result<()> {
    let std_exe = std_exe.as_ref();
    let compiling = Command::new("g++")
        .arg(&std_exe.with_extension("cpp"))
        .arg("-o")
        .arg(&std_exe)
        .arg("-std=c++17")
        .arg("-Wall")
        .arg("-Wl,--stack=268435456")
        .arg("-O2")
        .status()?;
    if !compiling.success() {
        return Err(anyhow!("Failed to compile {}", std_exe.display()));
    }
    Ok(())
}
fn zip_data(name: impl AsRef<str>, data_dir: impl AsRef<Path>) -> Result<()> {
    let name = name.as_ref();
    let data_dir = data_dir.as_ref();
    let zip_file = data_dir.join("data.zip");
    if zip_file.is_file() {
        std::fs::remove_file(&zip_file)?;
    }
    let packing = Command::new("7z")
        .current_dir(&data_dir)
        .arg("a")
        .arg(format!("{}-data.zip", name))
        .arg("*.in")
        .arg("*.out")
        .status()?;
    if !packing.success() {
        return Err(anyhow!("Failed to pack data"));
    }
    Ok(())
}
pub trait Generator {
    fn generate(&mut self, id: usize, file: &mut std::fs::File) -> Result<()>;
}
pub fn build_data(
    name: impl AsRef<str>,
    ids: impl Iterator<Item = usize>,
    mut generator: impl Generator,
) -> Result<()> {
    let name = name.as_ref();
    let root_dir = Path::new(".").join("problems").join(name);
    if !root_dir.is_dir() {
        return Err(anyhow!("Problem {} does not exist", name));
    }
    let std_exe = root_dir.join("std.exe");
    compile(&std_exe)?;
    let data_dir = root_dir.join("data");
    if !data_dir.is_dir() {
        std::fs::create_dir(&data_dir)?;
    }
    for id in ids {
        let fixed_input_file = root_dir.join(format!("in-{}.txt", id));
        let input_file = data_dir.join(format!("{}.in", id));
        let output_file = data_dir.join(format!("{}.out", id));

        if fixed_input_file.is_file() {
            std::fs::copy(&fixed_input_file, &input_file)?;
            println!("Using fixed input file {}", fixed_input_file.display());
        } else {
            generator.generate(id, &mut std::fs::File::create(&input_file)?)?;
            println!("Created input file {}", input_file.display());
        };

        let std_input = std::fs::File::open(&input_file)?;
        let std_output = std::fs::File::create(&output_file)?;

        let std_running = Command::new(&std_exe)
            .stdin(std_input)
            .stdout(std_output)
            .status()?;
        if !std_running.success() {
            return Err(anyhow!("Failed to run {}", std_exe.display()));
        }
        println!("Created output file {}", output_file.display());
    }
    zip_data(name, &data_dir)
}
