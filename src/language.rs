#[cfg(any(feature = "eng", all(not(feature = "zhn"), not(feature = "pt"))))]
pub mod strings {
    pub const ERROR_DESERIALIZE_FILE: &str =
        "Failed to deserialize file! Does it exist or have composition parameters?";
    pub const ERROR_EMPTY_PATTERNS: &str = "Patterns are empty!";
    pub const ERROR_NO_FOUND_PATTERNS: &str = "No patterns found!";
    pub const ERROR_MIDI_EXPORT: &str = "Failed to export MIDI files!";
    pub const ERROR_TIME_REVERSED: &str = "Time cannot reverse!";
    pub const ERROR_BAD_TIME_SIGNATURE:&str = "Bad time signature! Denominator must be at least 1. Only numerators 2, 4, 8, 6,16, 32 are currently supported!";
    pub const ERROR_UNREACHABLE_EVENT: &str = "The event cannot be reached!";
    pub const ERROR_LOAD_INSTRUMENTS: &str =
        "One of the instruments cannot be loaded or created for playback. Do they exist?";
    pub const ERROR_EXPORT_TEMPLATE: &str = "Failed to export template!";
    pub const ERROR_NOT_FOUND: &str = "Cannot be found!";
    pub const ERROR_NEEDS_MORE_COMMANDS: &str = "Additional commands required!";
    pub const NOTE_EXPORTING: &str = "Exporting...";
    pub const NOTE_EXPORTED: &str = "Exported.";
    pub const NOTE_EXPORT_SUCCESS: &str = "Export complete.";
    pub const NOTE_PLAYBACK_COMPLETE: &str = "Playback complete.";
    pub const NOTE_TEMPLATE_EXPORTED: &str = "Exported template.";
    pub const TITLE: &str = "Chord Composer";
    pub const ABOUT: &str =
        "A music composition tool for structuring chord progressions and patterns.";
    pub const ABOUT_PLAYBACK: &str = "Playback patterns in a composition.";
    pub const ABOUT_COMPOSITION_FILE: &str = "The composition arrangement YAML file.";
    pub const ABOUT_METRONOME: &str = "Play a metronome during playback.";
    pub const ABOUT_MIDI_EXPORT: &str = "Export composition patterns to .MID.";
    pub const ABOUT_TEMPLATE_EXPORT: &str = "Export a composition arrangement YAML template.";
    pub const ABOUT_TEMPLATE_PATH: &str = "Export path of the YAML template.";
    pub const ABOUT_CHORDS: &str = "Print the list of supported chords and their intervals.";
    pub const ABOUT_PRINT_BAR: &str = "Prints the current time on each bar change.";
    pub const ABOUT_PRINT_BEAT: &str = "Prints the current time on each beat change.";
    pub const ABOUT_PRINT_BEAT_INTERVAL: &str =
        "Prints the current time on each beat interval change.";
    pub const ABOUT_HELP: &str = "For more, use-help.";
    pub const ABOUT_PATTERN_PLAY: &str = "Play a pattern.";
    pub const ABOUT_PLAYBACK_START: &str = "The bar to start playback from.";
    pub const WARNING_PATTERN_OPTION: &str =
        "Warning, the --pattern option is required for this tag to work.";
}

#[cfg(feature = "pt")]
pub mod strings {
    pub const ERROR_DESERIALIZE_FILE: &str =
        "Falha ao desserializar ficheiro! Existe ou tem parâmetros de composição?";
    pub const ERROR_EMPTY_PATTERNS: &str = "Padrões vazios!";
    pub const ERROR_NO_FOUND_PATTERNS: &str = "Nenhum padrão encontrado!";
    pub const ERROR_MIDI_EXPORT: &str = "Falha ao exportar ficheiros MIDI!";
    pub const ERROR_TIME_REVERSED: &str = "O tempo não pode reverter!";
    pub const ERROR_BAD_TIME_SIGNATURE:&str = "Péssimo horário de assinatura! O denominador deve ser pelo menos 1. Apenas numeradores 2, 4, 8, 6, 16, 32 são suportados atualmente!";
    pub const ERROR_UNREACHABLE_EVENT: &str = "O evento não pode ser alcançado!";
    pub const ERROR_LOAD_INSTRUMENTS: &str =
        "Um dos instrumentos não pode ser carregado ou criado para playback. Será que existem?";
    pub const ERROR_EXPORT_TEMPLATE: &str = "Falha ao exportar modelo!";
    pub const ERROR_NOT_FOUND: &str = "Não foi encontrado!";
    pub const ERROR_NEEDS_MORE_COMMANDS: &str = "Comandos adicionais necessários!";
    pub const NOTE_EXPORTING: &str = "Exportando...";
    pub const NOTE_EXPORTED: &str = "Exportado.";
    pub const NOTE_EXPORT_SUCCESS: &str = "Exportação completa.";
    pub const NOTE_PLAYBACK_COMPLETE: &str = "Playback completo.";
    pub const NOTE_TEMPLATE_EXPORTED: &str = "Modelo exportado.";
    pub const TITLE: &str = "Chord Composer (Compositor de acordes)";
    pub const ABOUT: &str =
        "Uma ferramenta de composição musical para estreturar progressos e padrões de acordes.";
    pub const ABOUT_PLAYBACK: &str = "Padrões de playback numa composição.";
    pub const ABOUT_COMPOSITION_FILE: &str = "A disposição de composição do ficheiro YAML.";
    pub const ABOUT_METRONOME: &str = "Reproduz um metrônomo durante o playback";
    pub const ABOUT_MIDI_EXPORT: &str = "Exportar padrões de composição para .MID.";
    pub const ABOUT_TEMPLATE_EXPORT: &str =
        "Exportar uma disposição de composião em modelo YAML.";
    pub const ABOUT_TEMPLATE_PATH: &str = "Exportar trajeto do modelo YAML.";
    pub const ABOUT_CHORDS: &str = "Imprime a lista de acordes suportados e respetivos intervalos.";
    pub const ABOUT_PRINT_BAR: &str = "Imprime a hora atual em cada alteração de barra.";
    pub const ABOUT_PRINT_BEAT: &str = "Imprime a hora atual em cada alteração de batida.";
    pub const ABOUT_PRINT_BEAT_INTERVAL: &str =
        "Imprime a hora atual em cada alteração de intervalo da batida.";
    pub const ABOUT_HELP: &str = "Para mais, utilize-ajuda.";
    pub const ABOUT_PATTERN_PLAY: &str = "Reproduzir um padrão.";
    pub const ABOUT_PLAYBACK_START: &str = "A barra para retomar playback.";
    pub const WARNING_PATTERN_OPTION: &str =
        "Aviso, a opção de padrão é obrigatória para esta tag funcionar.";
}

#[cfg(feature = "zhn")]
pub mod strings {
    pub const ERROR_DESERIALIZE_FILE: &str = "反序列化文件失败！它是否存在或有组成参数？";
    pub const ERROR_EMPTY_PATTERNS: &str = "图案是空的！";
    pub const ERROR_NO_FOUND_PATTERNS: &str = "找不到图案！";
    pub const ERROR_MIDI_EXPORT: &str = "导出MIDI文件失败！";
    pub const ERROR_TIME_REVERSED: &str = "时间不能倒转！";
    pub const ERROR_BAD_TIME_SIGNATURE: &str =
        "时间签名错误！分母必须至少为1。目前只支持分子2、4、8、6、16、32！";
    pub const ERROR_UNREACHABLE_EVENT: &str = "无法访问事件！";
    pub const ERROR_LOAD_INSTRUMENTS: &str = "无法加载或创建其中一个乐器以供播放。它们存在吗？";
    pub const ERROR_EXPORT_TEMPLATE: &str = "导出模板失败！";
    pub const ERROR_NOT_FOUND: &str = "找不到！";
    pub const ERROR_NEEDS_MORE_COMMANDS: &str = "需要其他命令！";
    pub const NOTE_EXPORTING: &str = "正在导出。。。";
    pub const NOTE_EXPORTED: &str = "出口。";
    pub const NOTE_EXPORT_SUCCESS: &str = "导出完成。";
    pub const NOTE_PLAYBACK_COMPLETE: &str = "播放完毕。";
    pub const NOTE_TEMPLATE_EXPORTED: &str = "导出的模板。";
    pub const TITLE: &str = "Chord Composer (和弦作曲家)";
    pub const ABOUT: &str = "一种用于构造和弦进行和模式的音乐创作工具。";
    pub const ABOUT_PLAYBACK: &str = "乐曲中的重放模式。";
    pub const ABOUT_COMPOSITION_FILE: &str = "构图安排YAML文件。";
    pub const ABOUT_METRONOME: &str = "播放时播放节拍器。";
    pub const ABOUT_MIDI_EXPORT: &str = "将合成模式导出到.MID。";
    pub const ABOUT_TEMPLATE_EXPORT: &str = "导出构图排列YAML模板。";
    pub const ABOUT_TEMPLATE_PATH: &str = "YAML模板的导出路径。";
    pub const ABOUT_CHORDS: &str = "打印支持的和弦及其间隔的列表。";
    pub const ABOUT_PRINT_BAR: &str = "打印每个更改栏上的当前时间。";
    pub const ABOUT_PRINT_BEAT: &str = "打印每个节拍变化的当前时间。";
    pub const ABOUT_PRINT_BEAT_INTERVAL: &str = "打印每个拍间隔变化的当前时间。";
    pub const ABOUT_HELP: &str = "更多信息，请使用帮助。";
    pub const ABOUT_PATTERN_PLAY: &str = "演奏一个模式。";
    pub const ABOUT_PLAYBACK_START: &str = "开始播放一栏。";
    pub const WARNING_PATTERN_OPTION: &str = "警告，此标记需要--模式 选项才能工作。";
}

#[test]
fn test_language() {
    if cfg!(any(
        feature = "eng",
        all(not(feature = "zhn"), not(feature = "pt"))
    )) {
        assert_eq!(strings::TITLE, "Chord Composer");
    } else if cfg!(feature = "pt") {
        assert_eq!(strings::TITLE, "Chord Composer (Compositor de acordes)");
    } else if cfg!(feature = "zhn") {
        assert_eq!(strings::TITLE, "Chord Composer (和弦作曲家)");
    } else if cfg!(feature = "pt") {
        assert_eq!(strings::TITLE, "Chord ");
    } else {
        assert!(false, "No language supplied");
    }
}
