use const_format::formatcp;

pub const HELP: &str = formatcp!("\n\
PICOGREP! SMALLEST POSSIBLE GREP.\n\
\n\
SYNOPSIS:\n\
\n\
\tpicogrep [OPTIONS...] REGEXPRESSION [SOURCES...]\n\
\n\
SOURCES:\n\
\n\
\tFile path, directory path (search all files) or plain text.
\tAlso accepts stdin.
\n\
OPTIONS BRIEF:\n\
\n\
\t-h\t-\tPrints help\n\
\t-l\t-\tLines to show around search matches\n\
\n\
OPTIONS:\n\
\n\
{OPTION_HELP_L}\n\
\n\
EXAPMLES:\n\
\n\
\tcat log.txt | picogrep ERROR\n\
\n\
\tpicogrep \"ERR(OR)*\\|WARN(ING)*/i\" another_log.txt ../more_logs_dir\n\
\n");

pub const OPTION_HELP_L: &str = "\
\t-l\n\
\t\tLines to show around search matches.\n\
\t\tYou can provide number or number with explicit display method modifier.\n\
\t\tNu - Show N lines up\n\
\t\tNd - Show N lines down\n\
\t\tNo modifier - match is centered\n\
\t\tFor example: -l 5d - show 5 lines down each match.";