use transcribot_whisper_wrapper::WhisperArgs;

fn main() {
    let w_args = WhisperArgs::from_cmd();
    let res = w_args.run_whisper().unwrap_or_else(|err| {
        panic!("{err}");
    });
    println!("{res}")
}
