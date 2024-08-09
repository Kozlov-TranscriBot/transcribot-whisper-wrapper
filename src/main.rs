use whisper_wrapper::WhisperArgs;

fn main() {
    match WhisperArgs::from_cmd() {
        Ok(w_args) => {
            let res = w_args.run_whisper().unwrap_or_else(|err| {
                panic!("{err}");
            });
            println!("{res}") 
        },
        Err(error_message) => panic!("{}", error_message)
    }
}
