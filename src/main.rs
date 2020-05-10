mod language;

use chord_composer::{
    performance::performance_engine::PerformanceState,
    theory::{
        composition::{Composition, Pattern, PatternEvent},
        notes,
    },
    FailResult, SuccessResult,
};
use clap::{App, Arg, SubCommand};
use language::strings;
use music_timer::music_time::MusicTime;

/// Parse the results of a chord composer interaction.
///
/// # Arguments
/// * `result` - The result from the interaction.
fn parse_result(result: &Result<SuccessResult, FailResult>) {
    match result {
        Ok(SuccessResult::Export(exported_files)) => {
            for exported_file in exported_files {
                println!("{}{}", strings::NOTE_EXPORTED, exported_file);
            }
            println!("{}", strings::NOTE_EXPORT_SUCCESS)
        }
        Ok(SuccessResult::Playback) => println!("{}", strings::NOTE_PLAYBACK_COMPLETE),
        Ok(SuccessResult::ExportTemplate) => {
            println!("{}", strings::NOTE_TEMPLATE_EXPORTED)
        }
        Err(FailResult::Deserialize) => println!("{}", strings::ERROR_DESERIALIZE_FILE),
        Err(FailResult::EmptyPatterns) => println!("{}", strings::ERROR_EMPTY_PATTERNS),
        Err(FailResult::NoPatterns) => println!("{}", strings::ERROR_NO_FOUND_PATTERNS),
        Err(FailResult::ExportMIDI) => println!("{}", strings::ERROR_MIDI_EXPORT),
        Err(FailResult::ExportTemplate) => println!("{}", strings::ERROR_EXPORT_TEMPLATE),
        Err(FailResult::LoadSampler) => println!("{}", strings::ERROR_LOAD_INSTRUMENTS),
        Err(FailResult::TimeReverse(music_time, index, chord)) => {
            println!(
                "{} - [{}] {} @ {}.{}.{}",
                strings::ERROR_TIME_REVERSED,
                index,
                chord,
                music_time.get_bar(),
                music_time.get_beat(),
                music_time.get_beat_interval()
            );
        }
        Err(FailResult::TimeSignature(time_signature)) => println!(
            "{} - {}/{}",
            strings::ERROR_BAD_TIME_SIGNATURE,
            time_signature.get_numerator(),
            time_signature.get_denominator(),
        ),
        Err(FailResult::NoFoundPattern(file)) => {
            println!("{} - {}", strings::ERROR_NO_FOUND_PATTERNS, file)
        }
        Err(FailResult::UnreachableTime(music_time, index, chord)) => {
            println!(
                "{} - [{}] {} @ {}.{}.{}",
                strings::ERROR_UNREACHABLE_EVENT,
                index,
                chord,
                music_time.get_bar(),
                music_time.get_beat(),
                music_time.get_beat_interval()
            );
        }
    }
}

/// Export the patterns from a composition file to midi files.
///
/// # Arguments
/// * `file_path` - The path of the composition parameter yaml file to export.
fn export(file_path: &str) {
    println!("{}{}", strings::NOTE_EXPORTING, file_path);
    parse_result(&chord_composer::export_file_to_midi(file_path));
}

/// Load a composition from a file and then play it.
///
/// # Arguments
/// * `file_path` - The file path of the composition parameter yaml file.
/// * `play_with_metronome` - Playback the composition with a metronome.
fn load_and_play(
    file_path: &str,
    play_with_metronome: bool,
    ticker_bar: bool,
    ticker_beat: bool,
    ticker_interval: bool,
    pattern: Option<&str>,
    start_bar: u16,
) {
    /// The composition playback performance state.
    struct State {
        ticker_bar: bool,
        ticker_beat: bool,
        ticker_interval: bool,
    }

    impl PerformanceState for State {
        /// Called when the composition is ready for playback.
        ///
        /// # Arguments
        /// * `composition` - The composition to play.
        fn on_ready(&mut self, composition: &Composition) {
            println!("[ {} ]", composition.get_name());
        }

        /// Called when playback has a change in beat interval.
        ///
        /// # Arguments
        /// * `current_time` - The `MusicTime` which the callback has been triggered.
        fn on_beat_interval_change(&mut self, current_time: &MusicTime) {
            if self.ticker_interval {
                println!(
                    "| {:02}.{}.{}",
                    current_time.get_bar(),
                    current_time.get_beat(),
                    current_time.get_beat_interval()
                );
            }
        }

        /// Called when playback has a change in beat.
        ///
        /// # Arguments
        /// * `current_time` - The `MusicTime` which the callback has been triggered.
        fn on_beat_change(&mut self, current_time: &MusicTime) {
            if self.ticker_beat {
                println!(
                    "| {:02}.{}.{}",
                    current_time.get_bar(),
                    current_time.get_beat(),
                    current_time.get_beat_interval()
                );
            }
        }

        /// Called when playback has a change in bar.
        ///
        /// # Arguments
        /// * `current_time` - The `MusicTime` which the callback has been triggered.
        fn on_bar_change(&mut self, current_time: &MusicTime) {
            if self.ticker_bar {
                println!(
                    "| {:02}.{}.{}",
                    current_time.get_bar(),
                    current_time.get_beat(),
                    current_time.get_beat_interval()
                );
            }
        }

        /// Called when playback is triggering an event in the performance.
        ///
        /// # Arguments
        /// * `event` - The `PatternEvent` been triggered at this event.
        fn on_event(&mut self, event: &PatternEvent) {
            let (time, notes) = event;

            let intervals_string = {
                let mut intervals_string = String::new();

                for note in notes {
                    let (octave, interval_enum) = notes::midi_to_note(*note);
                    let interval_string = notes::key_to_string(interval_enum);
                    intervals_string.push_str(&format!("{}{} ", octave, interval_string));
                }
                intervals_string
            };

            println!(
                "| {:02}.{}.{} | {}",
                time.get_bar(),
                time.get_beat(),
                time.get_beat_interval(),
                intervals_string
            );
        }

        /// Called when playback is about the begin to play a pattern.
        ///
        /// # Arguments
        /// * `pattern` - The `Pattern` ready for playback.
        fn on_pattern_playback_begin(&mut self, pattern: &Pattern) {
            println!(
                "<< {} {}/{} @ {}bmp >>",
                pattern.get_name(),
                pattern.get_time_signature().get_numerator(),
                pattern.get_time_signature().get_denominator(),
                pattern.get_bpm()
            );
        }

        /// Called when playback of a pattern has ended.
        ///
        /// # Arguments
        /// * `pattern` - The `Pattern` concluding it's playback.
        fn on_pattern_playback_end(&mut self, _pattern: &Pattern) {}

        /// Called when playback has completed.
        ///
        /// # Arguments
        /// * `composition` - The `Composition` concluding it's playback.
        fn on_completed(&mut self, _composition: &Composition) {}
    }

    // Load and play a composition file
    let mut performance_state = State {
        ticker_bar,
        ticker_beat,
        ticker_interval,
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

    match pattern {
        Some(pattern_name) => {
            parse_result(&chord_composer::play_file_from(
                file_path,
                &mut performance_state,
                play_with_metronome,
                &sample_paths_metronome,
                &sample_paths_piano,
                &MusicTime::new(start_bar, 1, 1),
                pattern_name,
            ));
        }
        None => {
            parse_result(&chord_composer::play_file(
                file_path,
                &mut performance_state,
                play_with_metronome,
                &sample_paths_metronome,
                &sample_paths_piano,
            ));
        }
    }
}

/// Print all the supported internal chords.
fn print_chords() {
    for chord in chord_composer::get_chord_keywords() {
        println!("> {}", chord);
    }
}

/// Export a template composition yaml file.
///
/// # Arguments
/// * `file_path` - The path to export the template to.
fn export_template(file_path: &str) {
    parse_result(&chord_composer::export_template(file_path));
}

fn main() {
    let matches = App::new(strings::TITLE)
        .version("0.1.3")
        .author("Cj <unsignedbytebite@gmail.com>")
        .about(strings::ABOUT)
        .subcommand(
            SubCommand::with_name("play")
                .about(strings::ABOUT_PATTERN_PLAY)
                .arg(
                    Arg::with_name("COMPOSITION_FILE")
                        .help(strings::ABOUT_COMPOSITION_FILE)
                        .required(true),
                )
                .arg(
                    Arg::with_name("metronome")
                        .long("metronome")
                        .help(strings::ABOUT_METRONOME)
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("ticker-bar")
                        .long("ticker-bar")
                        .help(strings::ABOUT_PRINT_BAR)
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("ticker-beat")
                        .long("ticker-beat")
                        .help(strings::ABOUT_PRINT_BEAT)
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("ticker-interval")
                        .long("ticker-interval")
                        .help(strings::ABOUT_PRINT_BEAT_INTERVAL)
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("pattern")
                        .long("pattern")
                        .help(strings::ABOUT_PLAYBACK)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("start-bar")
                        .long("start-bar")
                        .help(strings::ABOUT_PLAYBACK_START)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("template")
                .about(strings::ABOUT_TEMPLATE_EXPORT)
                .arg(
                    Arg::with_name("EXPORT_PATH")
                        .help(strings::ABOUT_TEMPLATE_PATH)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("export")
                .about(strings::ABOUT_MIDI_EXPORT)
                .arg(
                    Arg::with_name("COMPOSITION_FILE")
                        .help(strings::ABOUT_COMPOSITION_FILE)
                        .required(true),
                ),
        )
        .subcommand(SubCommand::with_name("chords").about(strings::ABOUT_CHORDS))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("play") {
        match matches.value_of("COMPOSITION_FILE") {
            Some(file_name) => load_and_play(
                file_name,
                matches.is_present("metronome"),
                matches.is_present("ticker-bar"),
                matches.is_present("ticker-beat"),
                matches.is_present("ticker-interval"),
                matches.value_of("pattern"),
                match matches.value_of("start-bar") {
                    Some(start_bar) => {
                        let has_pattern_tag = matches.is_present("pattern");
                        if !has_pattern_tag {
                            println!(
                                "{} --start-bar : {}",
                                strings::WARNING_PATTERN_OPTION,
                                strings::ABOUT_PLAYBACK_START
                            );
                        }

                        start_bar.parse().unwrap_or(1)
                    }
                    None => 1,
                },
            ),
            None => println!(
                "{} {}",
                strings::ERROR_NEEDS_MORE_COMMANDS,
                strings::ABOUT_HELP
            ),
        }
    } else if let Some(matches) = matches.subcommand_matches("export") {
        match matches.value_of("COMPOSITION_FILE") {
            Some(file_name) => export(file_name),
            None => println!(
                "COMPOSITION_FILE {} {}",
                strings::ERROR_NOT_FOUND,
                strings::ABOUT_HELP
            ),
        }
    } else if let Some(_matches) = matches.subcommand_matches("chords") {
        print_chords();
    } else if let Some(matches) = matches.subcommand_matches("template") {
        match matches.value_of("EXPORT_PATH") {
            Some(file_name) => export_template(file_name),
            None => println!(
                "EXPORT_PATH {} {}",
                strings::ERROR_NOT_FOUND,
                strings::ABOUT_HELP
            ),
        }
    } else {
        println!("{}", strings::ABOUT_HELP)
    }
}
