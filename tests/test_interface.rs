extern crate chord_composer;

#[test]
fn chord_to_string_array() {
    let chords = chord_composer::get_chord_keywords();
    assert_eq!(chords.len(), 73);
    assert_eq!(chords[0], "AUGMENTED = [0, 4, 8]");
}

#[test]
fn export_midi_missing_file() {
    let file = "./tests/no_file.gone";

    assert_eq!(
        chord_composer::export_file_to_midi(file),
        Err(chord_composer::FailResult::Deserialize)
    );
}

#[test]
fn export_midi_no_patterns() {
    let file = "./tests/export_test_no_patterns.yaml";

    assert_eq!(
        chord_composer::export_file_to_midi(file),
        Err(chord_composer::FailResult::NoPatterns)
    );
}

#[test]
fn test_export_template() {
    assert_eq!(
        chord_composer::export_template("./tests/test_template_export.yaml"),
        Ok(chord_composer::SuccessResult::ExportTemplate)
    );
}

#[test]
fn export_midi() {
    let file = "./tests/export_test.yaml";

    let files = vec![
        "./tests/bc_000_a/part_a.mid".to_string(),
        "./tests/bc_000_a/part_b.mid".to_string(),
    ];

    assert_eq!(
        chord_composer::export_file_to_midi(file),
        Ok(chord_composer::SuccessResult::Export(files.clone())),
    );

    use std::fs::File;
    use std::io::prelude::*;

    let file1_bin = vec![
        0x4D, 0x54, 0x68, 0x64, 0x00, 0x00, 0x00, 0x06, 0x00, 0x01, 0x00, 0x02, 0x01, 0xE0, 0x4D,
        0x54, 0x72, 0x6B, 0x00, 0x00, 0x00, 0x13, 0x00, 0xFF, 0x51, 0x03, 0x07, 0x27, 0x0E, 0x00,
        0xFF, 0x58, 0x04, 0x03, 0x02, 0x08, 0x18, 0x00, 0xFF, 0x2F, 0x00, 0x4D, 0x54, 0x72, 0x6B,
        0x00, 0x00, 0x01, 0x07, 0x00, 0xFF, 0x03, 0x06, 0x70, 0x61, 0x72, 0x74, 0x5F, 0x61, 0x00,
        0x90, 0x3E, 0x40, 0x00, 0x90, 0x42, 0x40, 0x00, 0x90, 0x45, 0x40, 0x00, 0x90, 0x49, 0x40,
        0x87, 0x40, 0x90, 0x3E, 0x00, 0x00, 0x90, 0x42, 0x00, 0x00, 0x90, 0x45, 0x00, 0x00, 0x90,
        0x49, 0x00, 0x00, 0x90, 0x3E, 0x40, 0x00, 0x90, 0x41, 0x40, 0x00, 0x90, 0x46, 0x40, 0x83,
        0x60, 0x90, 0x3E, 0x00, 0x00, 0x90, 0x41, 0x00, 0x00, 0x90, 0x46, 0x00, 0x00, 0x90, 0x3E,
        0x40, 0x00, 0x90, 0x42, 0x40, 0x00, 0x90, 0x45, 0x40, 0x00, 0x90, 0x49, 0x40, 0x00, 0x90,
        0x40, 0x40, 0x87, 0x40, 0x90, 0x3E, 0x00, 0x00, 0x90, 0x42, 0x00, 0x00, 0x90, 0x45, 0x00,
        0x00, 0x90, 0x49, 0x00, 0x00, 0x90, 0x40, 0x00, 0x00, 0x90, 0x3E, 0x40, 0x00, 0x90, 0x41,
        0x40, 0x00, 0x90, 0x46, 0x40, 0x83, 0x60, 0x90, 0x3E, 0x00, 0x00, 0x90, 0x41, 0x00, 0x00,
        0x90, 0x46, 0x00, 0x00, 0x90, 0x41, 0x40, 0x00, 0x90, 0x45, 0x40, 0x00, 0x90, 0x48, 0x40,
        0x00, 0x90, 0x4C, 0x40, 0x83, 0x60, 0x90, 0x41, 0x00, 0x00, 0x90, 0x45, 0x00, 0x00, 0x90,
        0x48, 0x00, 0x00, 0x90, 0x4C, 0x00, 0x00, 0x90, 0x3E, 0x40, 0x00, 0x90, 0x41, 0x40, 0x00,
        0x90, 0x46, 0x40, 0x83, 0x60, 0x90, 0x3E, 0x00, 0x00, 0x90, 0x41, 0x00, 0x00, 0x90, 0x46,
        0x00, 0x83, 0x60, 0x90, 0x3B, 0x40, 0x00, 0x90, 0x3F, 0x40, 0x00, 0x90, 0x42, 0x40, 0x00,
        0x90, 0x46, 0x40, 0x00, 0x90, 0x3D, 0x40, 0x83, 0x60, 0x90, 0x3B, 0x00, 0x00, 0x90, 0x3F,
        0x00, 0x00, 0x90, 0x42, 0x00, 0x00, 0x90, 0x46, 0x00, 0x00, 0x90, 0x3D, 0x00, 0x00, 0x90,
        0x3E, 0x40, 0x00, 0x90, 0x41, 0x40, 0x00, 0x90, 0x46, 0x40, 0x87, 0x40, 0x90, 0x3E, 0x00,
        0x00, 0x90, 0x41, 0x00, 0x00, 0x90, 0x46, 0x00, 0x00, 0xFF, 0x2F, 0x00,
    ];

    let buffer = {
        let mut f = File::open(&files[0]).unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();
        buffer
    };

    assert_eq!(buffer, file1_bin);

    let file2_bin = vec![
        0x4D, 0x54, 0x68, 0x64, 0x00, 0x00, 0x00, 0x06, 0x00, 0x01, 0x00, 0x02, 0x01, 0xE0, 0x4D,
        0x54, 0x72, 0x6B, 0x00, 0x00, 0x00, 0x13, 0x00, 0xFF, 0x51, 0x03, 0x0D, 0x44, 0xBD, 0x00,
        0xFF, 0x58, 0x04, 0x04, 0x03, 0x08, 0x18, 0x00, 0xFF, 0x2F, 0x00, 0x4D, 0x54, 0x72, 0x6B,
        0x00, 0x00, 0x01, 0x06, 0x00, 0xFF, 0x03, 0x06, 0x70, 0x61, 0x72, 0x74, 0x5F, 0x62, 0x00,
        0x90, 0x3D, 0x40, 0x00, 0x90, 0x41, 0x40, 0x00, 0x90, 0x44, 0x40, 0x00, 0x90, 0x48, 0x40,
        0x83, 0x60, 0x90, 0x3D, 0x00, 0x00, 0x90, 0x41, 0x00, 0x00, 0x90, 0x44, 0x00, 0x00, 0x90,
        0x48, 0x00, 0x00, 0x90, 0x3D, 0x40, 0x00, 0x90, 0x40, 0x40, 0x00, 0x90, 0x45, 0x40, 0x8B,
        0x20, 0x90, 0x3D, 0x00, 0x00, 0x90, 0x40, 0x00, 0x00, 0x90, 0x45, 0x00, 0x00, 0x90, 0x3D,
        0x40, 0x00, 0x90, 0x41, 0x40, 0x00, 0x90, 0x44, 0x40, 0x00, 0x90, 0x48, 0x40, 0x00, 0x90,
        0x3F, 0x40, 0x83, 0x60, 0x90, 0x3D, 0x00, 0x00, 0x90, 0x41, 0x00, 0x00, 0x90, 0x44, 0x00,
        0x00, 0x90, 0x48, 0x00, 0x00, 0x90, 0x3F, 0x00, 0x00, 0x90, 0x3D, 0x40, 0x00, 0x90, 0x40,
        0x40, 0x00, 0x90, 0x45, 0x40, 0x8B, 0x20, 0x90, 0x3D, 0x00, 0x00, 0x90, 0x40, 0x00, 0x00,
        0x90, 0x45, 0x00, 0x00, 0x90, 0x40, 0x40, 0x00, 0x90, 0x44, 0x40, 0x00, 0x90, 0x47, 0x40,
        0x00, 0x90, 0x4B, 0x40, 0x83, 0x60, 0x90, 0x40, 0x00, 0x00, 0x90, 0x44, 0x00, 0x00, 0x90,
        0x47, 0x00, 0x00, 0x90, 0x4B, 0x00, 0x00, 0x90, 0x3D, 0x40, 0x00, 0x90, 0x40, 0x40, 0x00,
        0x90, 0x45, 0x40, 0x8B, 0x20, 0x90, 0x3D, 0x00, 0x00, 0x90, 0x40, 0x00, 0x00, 0x90, 0x45,
        0x00, 0x00, 0x90, 0x3A, 0x40, 0x00, 0x90, 0x3E, 0x40, 0x00, 0x90, 0x41, 0x40, 0x00, 0x90,
        0x45, 0x40, 0x00, 0x90, 0x3C, 0x40, 0x83, 0x60, 0x90, 0x3A, 0x00, 0x00, 0x90, 0x3E, 0x00,
        0x00, 0x90, 0x41, 0x00, 0x00, 0x90, 0x45, 0x00, 0x00, 0x90, 0x3C, 0x00, 0x00, 0x90, 0x3D,
        0x40, 0x00, 0x90, 0x40, 0x40, 0x00, 0x90, 0x45, 0x40, 0x8B, 0x20, 0x90, 0x3D, 0x00, 0x00,
        0x90, 0x40, 0x00, 0x00, 0x90, 0x45, 0x00, 0x00, 0xFF, 0x2F, 0x00,
    ];

    let buffer = {
        let mut f = File::open(&files[1]).unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();
        buffer
    };

    assert_eq!(buffer, file2_bin);
}

#[test]
fn test_play_file() {
    let file = "./tests/middle_c.yaml";

    use chord_composer::{
        performance::performance_engine::PerformanceState,
        theory::composition::{Composition, Pattern, PatternEvent},
    };
    use music_timer::music_time::MusicTime;

    struct MyState {
        callback_calls: u16,
        current_time: MusicTime,
    }

    impl PerformanceState for MyState {
        fn on_ready(&mut self, composition: &Composition) {
            self.callback_calls += 1;
            assert_eq!(composition.get_name(), "middle_c");
        }
        fn on_beat_interval_change(&mut self, current_time: &MusicTime) {
            self.callback_calls += 1;
            self.current_time = current_time.clone();
        }
        fn on_beat_change(&mut self, _current_time: &MusicTime) {
            self.callback_calls += 1;
        }
        fn on_bar_change(&mut self, _current_time: &MusicTime) {
            self.callback_calls += 1;
        }
        fn on_event(&mut self, event: &PatternEvent) {
            self.callback_calls += 1;
            let (event_time, event_notes) = event;
            assert_eq!(event_time, &MusicTime::default());
            assert_eq!(event_notes, &vec![60]);
        }
        fn on_pattern_playback_begin(&mut self, _pattern: &Pattern) {
            self.callback_calls += 1;
        }
        fn on_pattern_playback_end(&mut self, _pattern: &Pattern) {
            self.callback_calls += 1;
        }
        fn on_completed(&mut self, composition: &Composition) {
            self.callback_calls += 1;
            assert_eq!(composition.get_name(), "middle_c");
        }
    }

    let mut my_state = MyState {
        callback_calls: 0,
        current_time: MusicTime::default(),
    };

    // Load audio assets
    let sample_paths_metronome = vec![
        "./audio_assets/metronome/tock.ogg".to_string(),
        "./audio_assets/metronome/tick.ogg".to_string(),
    ];

    let sample_paths_piano = {
        let mut sample_paths = Vec::new();
        for i in 1..61 {
            let path = format!("audio_assets/piano_instrument/piano ({}).ogg", i);
            sample_paths.push(path);
        }
        sample_paths
    };
    assert_eq!(
        chord_composer::play_file(
            file,
            &mut my_state,
            true,
            &sample_paths_metronome,
            &sample_paths_piano
        ),
        Ok(chord_composer::SuccessResult::Playback)
    );

    assert_eq!(my_state.callback_calls, 42);
    assert_eq!(my_state.current_time, MusicTime::new(1, 4, 8));
}
