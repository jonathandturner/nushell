#[macro_use]
pub mod macros;

pub mod append;
pub mod args;
pub mod autoenv;
pub mod autoenv_trust;
pub mod autoenv_untrust;
pub mod autoview;
pub mod build_string;
pub mod cd;
pub mod char_;
pub mod classified;
pub mod command;
pub mod compact;
pub mod config;
pub mod constants;
pub mod count;
pub mod cp;
pub mod debug;
pub mod def;
pub mod default;
pub mod describe;
pub mod do_;
pub mod drop;
pub mod du;
pub mod each;
pub mod echo;
pub mod empty;
pub mod enter;
pub mod every;
pub mod exit;
pub mod first;
pub mod flatten;
pub mod format;
pub mod get;
pub mod group_by;
pub mod group_by_date;
pub mod headers;
pub mod help;
pub mod history;
pub mod if_;
pub mod insert;
pub mod into_int;
pub mod keep;
pub mod last;
pub mod let_;
pub mod let_env;
pub mod lines;
pub mod ls;
pub mod merge;
pub mod mkdir;
pub mod move_;
pub mod next;
pub mod nth;
pub mod nu;
pub mod open;
pub mod parse;
pub mod path;
pub mod pivot;
pub mod prepend;
pub mod prev;
pub mod pwd;
pub mod range;
pub mod reduce;
pub mod reject;
pub mod rename;
pub mod reverse;
pub mod rm;
pub mod run_external;
pub mod save;
pub mod select;
pub mod seq;
pub mod seq_dates;
pub mod shells;
pub mod size;
pub mod skip;
pub mod sleep;
pub mod sort_by;
pub mod source;
pub mod split;
pub mod split_by;
pub mod str_;
pub mod table;
pub mod tags;
pub mod uniq;
pub mod update;
pub mod version;
pub mod where_;
pub mod which_;
pub mod with_env;
pub mod wrap;

pub use autoview::Autoview;
pub use cd::Cd;

pub use append::Command as Append;
pub use autoenv::Autoenv;
pub use autoenv_trust::AutoenvTrust;
pub use autoenv_untrust::AutoenvUnTrust;
pub use build_string::BuildString;
pub use char_::Char;
pub use compact::Compact;
pub use config::{
    Config, ConfigClear, ConfigGet, ConfigLoad, ConfigPath, ConfigRemove, ConfigSet, ConfigSetInto,
};
pub use count::Count;
pub use cp::Cpy;
pub use debug::Debug;
pub use def::Def;
pub use default::Default;
pub use describe::Describe;
pub use do_::Do;
pub use drop::Drop;
pub use du::Du;
pub use each::Each;
pub use each::EachGroup;
pub use each::EachWindow;
pub use echo::Echo;
pub use empty::Command as Empty;
pub use if_::If;
pub use nu::NuPlugin;
pub use update::Command as Update;
pub mod clear;
pub use clear::Clear;
pub mod touch;
pub use enter::Enter;
pub use every::Every;
pub use exit::Exit;
pub use first::First;
pub use flatten::Command as Flatten;
pub use format::{FileSize, Format};
pub use get::Get;
pub use group_by::Command as GroupBy;
pub use group_by_date::GroupByDate;
pub use headers::Headers;
pub use help::Help;
pub use history::History;
pub use insert::Command as Insert;
pub use into_int::IntoInt;
pub use keep::{Keep, KeepUntil, KeepWhile};
pub use last::Last;
pub use let_::Let;
pub use let_env::LetEnv;
pub use lines::Lines;
pub use ls::Ls;
pub use merge::Merge;
pub use mkdir::Mkdir;
pub use move_::{Move, Mv};
pub use next::Next;
pub use nth::Nth;
pub use open::Open;
pub use parse::Parse;
pub use path::{
    PathBasename, PathCommand, PathDirname, PathExists, PathExpand, PathExtension, PathFilestem,
    PathType,
};
pub use pivot::Pivot;
pub use prepend::Prepend;
pub use prev::Previous;
pub use pwd::Pwd;
pub use range::Range;
pub use reduce::Reduce;
pub use reject::Reject;
pub use rename::Rename;
pub use reverse::Reverse;
pub use rm::Remove;
pub use run_external::RunExternalCommand;
pub use save::Save;
pub use select::Select;
pub use seq::Seq;
pub use seq_dates::SeqDates;
pub use shells::Shells;
pub use size::Size;
pub use skip::{Skip, SkipUntil, SkipWhile};
pub use sleep::Sleep;
pub use sort_by::SortBy;
pub use source::Source;
pub use split::{Split, SplitChars, SplitColumn, SplitRow};
pub use split_by::SplitBy;
pub use str_::{
    Str, StrCamelCase, StrCapitalize, StrCollect, StrContains, StrDowncase, StrEndsWith,
    StrFindReplace, StrFrom, StrIndexOf, StrKebabCase, StrLPad, StrLength, StrPascalCase, StrRPad,
    StrReverse, StrScreamingSnakeCase, StrSet, StrSnakeCase, StrStartsWith, StrSubstring,
    StrToDatetime, StrToDecimal, StrToInteger, StrTrim, StrTrimLeft, StrTrimRight, StrUpcase,
};
pub use table::Table;
pub use tags::Tags;
pub use touch::Touch;
pub use uniq::Uniq;
pub use version::Version;
pub use where_::Where;
pub use which_::Which;
pub use with_env::WithEnv;
pub use wrap::Wrap;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::examples::{test_anchors, test_examples};
    use nu_engine::{whole_stream_command, Command};
    use nu_errors::ShellError;

    fn full_tests() -> Vec<Command> {
        vec![
            whole_stream_command(Append),
            whole_stream_command(GroupBy),
            whole_stream_command(Insert),
            whole_stream_command(Move),
            whole_stream_command(Update),
            whole_stream_command(Empty),
        ]
    }

    fn only_examples() -> Vec<Command> {
        let mut commands = full_tests();
        commands.extend(vec![whole_stream_command(Flatten)]);
        commands
    }

    #[test]
    fn examples_work_as_expected() -> Result<(), ShellError> {
        for cmd in only_examples() {
            test_examples(cmd)?;
        }

        Ok(())
    }

    #[test]
    fn tracks_metadata() -> Result<(), ShellError> {
        for cmd in full_tests() {
            test_anchors(cmd)?;
        }

        Ok(())
    }
}
