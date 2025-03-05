mod audio;

fn main() {
    audio::record_audio(5).expect("Failed in main");
}
