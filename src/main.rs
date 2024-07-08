use std::{env::args, fs::{remove_file, File}, io::Read, process::{Command, Stdio}};

struct WhisperArgs {
    lang: String,
    file_path: String
}

fn read_file(path: &str) -> String {
    let file_str = File::open(path)
        .and_then(|mut file| {
            let mut internal_str = String::new();
            file.read_to_string(&mut internal_str)?;
            Ok(internal_str)
        })
        .or_else(|err| {
            Err(err.to_string())
        });
    remove_file(path).expect("Failed to remove file");
    match file_str {
        Ok(result_str) => result_str,
        Err(error) => error
    }
}

impl WhisperArgs {
    fn new() -> Self {
        let args: Vec<String> = args().collect();
        if args.len() < 3 {
            panic!("Specify language code and audio file path")
        }
        WhisperArgs {
            lang: args[1].clone(),
            file_path: args[2].clone()
        }
    }

    fn run_whisper(&self) -> Result<String, String> {
        let output_dir = "/tmp/whisper";
        let whisper_exit_status = Command::new("whisper")
            .stdout(Stdio::null())
            .args(["--model", "medium"])
            .args(["--verbose", "False", "-f", "txt", "--word_timestamps", "True", "-o", output_dir])
            .args(["--language", self.lang.as_str()])
            .args(["--", self.file_path.as_str()])
            .status()
            .expect("Error in whisper");
        if whisper_exit_status.success() {
            Ok(read_file(format!("{}/{}.txt", output_dir, self.file_path).as_str()))
        } else {
            Err(format!("Failed to process audio. Exit code {}", whisper_exit_status.to_string()))
        }
    }
}

fn main() {
    let w_args = WhisperArgs::new();
    let res = w_args.run_whisper().unwrap_or_else(|err| {
        panic!("{err}");
    });
    println!("{res}")
}
