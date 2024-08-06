pub const HELP: &str = "\n\
GREF.\n\
\n\
A grep-like \"global-search-and-FORMAT\" regular expressions search written in Rust as an educational project.\n\
To read more about regex implementation used under the hood see: https://docs.rs/regex/latest/regex/\n\
\n\
SYNOPSIS:\n\
\n\
\tgref SEARCH_EXPRESSION PATHS... [OPTIONS...]\n\
\n\
PATHS:\n\
\n\
\tUse valid file path or directory path (to search in files recursively).\n\
\tAlso accepts stdin.\n\
\n\
OPTIONS BRIEF:\n\
\n\
\t-h  -  Prints help\n\
\t-e  -  Extract mode. Extract capture group matches. Group can be named like \"(?<name>.*)\" or simple indexed starting from 0. \n\
\t       For example: -e \"message\" -e \"time\" will print both group matches each from new line sequentially.
\t       This is also forces -w mode. See: https://docs.rs/regex/latest/regex/struct.Regex.html#method.captures.\n\
\t-w  -  Show only exact match. You will forced to use something like \"^.*keyword.*$\" to imitate default line-showing behaviour.\n\
\t-f  -  Format mode. Format output using capture groups. For example: -f \"{time}: {message}\". Multiply -f's behaves as -e's. You can mix-up.\n\
\t       Group indexes are also allowed and starting from 0 like {0}. \n\
\t-v  -  Verbose mode. Show source, line, offset and other useful information alongside search results.\n\
\t-i  -  Case insensitive: Makes the regex match case insensitively.\n\
\t-m  -  Multiline mode. Changes the behavior of ^ and $ to match the beginning and end of lines within the input text.\n\
\t       Please be aware that using the mode forces to load entire files into the memory.\n\
\t-s  -  . matches new line (Single line mode): Changes the behavior of . to match any character, including newline characters (\\n).\n\
\t-U  -  Ungreedy (Swap greed): Reverses the greediness of quantifiers (*, +, ?, {{m,n}}) so that they match as few characters as possible.\n\
\t-x  -  Ignore whitespace and comments: Allows the use of whitespace and comments inside the regex pattern for clarity.\n\
\t-u  -  Unicode: Enables full Unicode support, including support for Unicode properties like \\p{Letter} and \\p{L}.\n\
\t-p  -  Pass text as command line argument to search within.\n\
\t-t  -  Do work in n-th threads. Default value is machine multithreading capability defined.
\t-d  -  Debug mode.\n\
\n\
INTERACTIVE MODE:\n\
\n\
\tProvide search expression only to run gref in the interactive mode.\n\
\tThen you can type or paste text directly into your terminal to search in.\n\
\tSend EOF (default Ctrl+D) to end stdin stream and get stdout results.\n\
\n\
EXAMPLES:\n\
\n\
\t## simple search for a keyword in a file and grab entire lines ##\n\
\t>> cat log.txt | gref ERROR\n\
\n\
\t## extract error messages from files with named group match  ##\n\
\t>> gref \"(ERR(OR)?\\|WARN(ING)?)(?<message>.*)\" -e message log.txt ../more_logs_dir\n\
\n";