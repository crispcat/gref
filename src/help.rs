use const_format::formatcp;

pub(crate) const HELP: &str = formatcp!("\
PICOGREP! SMALLEST POSSIBLE GREP!\n\
\n\
SYNOPSIS:\n\
picogrep [OPTIONS...] REGEXPRESSION [SOURCES...]\n\
\n\
OPTIONS:\n\
-h  -  Prints help\n\
-l  -  Lines to show around search matches\n\
\n\
OPTIONS:\
\n
-l\
\n
{OPTION_HELP_L}
\n\
SOURCES:
file path, directory path, plain text or stdin
\n\
EXAPMLES:\n\
cat log.txt | picogrep ERROR\n\
picogrep \"ERR(OR)*\\|WARN(ING)*/i\" another_log.txt ../more_logs_dir");

pub(crate) const OPTION_HELP_L: &str = "\
You can provide number or number with explicit display method modifier.\n\
Where:\n\
Nu - Show N lines up\n\
Nd - Show N lines down\n\
No modifier - match is centered\n\
For example: -l 5d - show 5 lines down each match.";