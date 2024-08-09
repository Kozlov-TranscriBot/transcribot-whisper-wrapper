pub mod model;

use std::{env::args, fs::{remove_file, File}, io::Read, process::{Command, Stdio}};
use model::ModelSize;

pub struct WhisperArgs {
    model: ModelSize,
    lang: String,
    file_path: String,
    filename: String
}

impl WhisperArgs {
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

    pub fn from_cmd() -> Result<Self, String> {
        let args: Vec<String> = args().collect();
        match args.len() {
            4 => Self::new(
                args[1].clone(), 
                args[2].clone(), 
                args[3].clone()
            ),
            _ => Err(String::from("TODO")) // TODO: Extend description, add help arm
        }
    }

    pub fn new(model_name: String, lang: String, file_path: String) -> Result<Self, String> {
        let model_size = ModelSize::from(model_name.as_str());
        match model_size {
            Ok(size) => Ok(WhisperArgs {
                model: size,
                lang: lang,
                file_path: file_path.clone(),
                filename: String::from(file_path.split('/').last().expect(
                    "There's not a valid path to file"
                ))
            }),
            Err(error_message) => Err(error_message)
        }
        
    }

    pub fn run_whisper(&self) -> Result<String, String> {
        let output_dir = "/tmp/whisper";
        let whisper_exit_status = Command::new("whisper")
            .stdout(Stdio::null())
            .args(["--model", self.model.as_str()])
            .args(["--verbose", "False", "-f", "txt", "--word_timestamps", "True", "-o", output_dir])
            .args(["--language", self.lang.as_str()])
            .args(["--", self.file_path.as_str()])
            .status()
            .expect("Error in whisper");
        if whisper_exit_status.success() {
            Ok(Self::read_file(format!("{}/{}.txt", output_dir, self.filename).as_str()))
        } else {
            Err(format!("Failed to process audio. Exit code {}", whisper_exit_status.to_string()))
        }
    }
}