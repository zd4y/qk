use std::collections::HashSet;

use anyhow::{bail, ensure, Context, Result};

pub type Command = Vec<Unit>;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Unit {
    Text(String),
    Positional(ClapPositional),
    Option(ClapOption),
    Flag(ClapFlag),
}

impl Unit {
    pub fn to_clap_args(units: HashSet<&Unit>) -> Vec<clap::Arg> {
        units
            .iter()
            .filter_map(|arg| match arg {
                Unit::Positional(unit) => {
                    let mut arg = clap::Arg::new(&unit.name)
                        .action(clap::ArgAction::Set)
                        .required(unit.required)
                        .index(unit.index);
                    if !unit.allow_empty_values {
                        arg = arg.value_parser(clap::builder::NonEmptyStringValueParser::new())
                    }
                    Some(arg)
                }
                Unit::Option(unit) => {
                    let mut arg = clap::Arg::new(&unit.name)
                        .action(clap::ArgAction::Set)
                        .required(unit.required);
                    if !unit.allow_empty_values {
                        arg = arg.value_parser(clap::builder::NonEmptyStringValueParser::new())
                    }
                    if let Some(long) = &unit.long {
                        arg = arg.long(long);
                    }
                    if let Some(short) = unit.short {
                        arg = arg.short(short)
                    }
                    Some(arg)
                }
                Unit::Flag(unit) => {
                    let mut arg = clap::Arg::new(&unit.name).action(clap::ArgAction::SetTrue);
                    if let Some(long) = &unit.long {
                        arg = arg.long(long);
                    }
                    if let Some(short) = unit.short {
                        arg = arg.short(short)
                    }
                    Some(arg)
                }
                _ => None,
            })
            .collect()
    }

    pub fn to_value(&self, matches: &clap::ArgMatches) -> Option<String> {
        match self {
            Unit::Text(text) => Some(text.to_owned()),
            Unit::Positional(unit) => matches.get_one::<String>(&unit.name).cloned(),
            Unit::Option(unit) => matches.get_one::<String>(&unit.name).cloned(),
            Unit::Flag(unit) => {
                if *matches.get_one::<bool>(&unit.name).unwrap() {
                    let prefix = if unit.long.is_none() { "-" } else { "--" };
                    let unit = format!("{}{}", prefix, unit.name);
                    Some(unit)
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ClapPositional {
    name: String,
    allow_empty_values: bool,
    required: bool,
    index: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ClapOption {
    name: String,
    long: Option<String>,
    short: Option<char>,
    allow_empty_values: bool,
    required: bool,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ClapFlag {
    name: String,
    long: Option<String>,
    short: Option<char>,
}

pub fn parse(command: &str) -> Result<Command> {
    match_args(command)
        .map(|(_, result)| result)
        .context("failed parsing commands")
}

fn match_literal(literal: &'static str) -> impl Fn(&str) -> Result<(&str, ())> {
    move |input: &str| {
        if let Some(matched) = input.strip_prefix(literal) {
            Ok((matched, ()))
        } else {
            bail!("expected literal {}", literal)
        }
    }
}

/// Matches the text until one of these characters:
/// - '!'
/// - '?'
/// - '*'
/// - ','
/// - '}'
///
fn match_name(input: &str) -> (&str, String) {
    let mut name = String::new();

    for char in input.chars() {
        match char {
            '!' | '?' | '*' | ',' | '}' => break,
            _ => name.push(char),
        }
    }

    (&input[name.len()..], name)
}

fn match_symbol(input: &str) -> Result<(&str, char), &str> {
    match input.chars().next() {
        Some('!') => Ok((&input['!'.len_utf8()..], '!')),
        Some('?') => Ok((&input['?'.len_utf8()..], '?')),
        Some('*') => Ok((&input['*'.len_utf8()..], '*')),
        _ => Err(input),
    }
}

/// Returns the input until: `#{`, `{{` or `\`
fn match_until_custom_arg_start(input: &str) -> (&str, &str) {
    for (index, c) in input.chars().enumerate() {
        if c == '\\' || ((c == '#' || c == '{') && input.get(index + 1..index + 2) == Some("{")) {
            return (&input[index..], &input[..index]);
        }
    }
    ("", input)
}

fn match_usize(input: &str) -> Result<(&str, usize)> {
    let mut number_str = String::new();
    for (index, c) in input.chars().enumerate() {
        if c.is_numeric() {
            number_str.push(c);
        } else if number_str.is_empty() {
            bail!("expecting usize number, found {:?}", input);
        } else {
            return Ok((
                &input[index..],
                number_str.parse().context("failed parsing usize number")?,
            ));
        }
    }
    bail!("expecting usize number, found {:?}", input);
}

/// Matches `<usize>:`
/// Example: `1:`
fn match_num(input: &str) -> Result<(&str, usize)> {
    let (next, num) = match_usize(input)?;
    let (next, _) = match_literal(":")(next)?;
    Ok((next, num))
}

fn match_custom_arg(input: &str) -> Result<(&str, Unit)> {
    if let Ok((next, _)) = match_literal("\\#{")(input) {
        let (next, text) = match_until_custom_arg_start(next);
        Ok((next, Unit::Text(format!("#{{{}", text))))
    } else {
        let (next, _) = match_literal("#{")(input)?;
        let (next, index) = match match_num(next) {
            Ok((next, index)) => (next, Some(index)),
            Err(_) => (next, None),
        };
        let (next, long) = match_name(next);
        let (next, _) = match_literal(",")(next).unwrap_or((next, ()));
        let (mut next, short) = match_name(next);

        if index.is_some() && !short.is_empty() {
            bail!("short not allowed in positional arguments");
        }

        let name = if long.is_empty() {
            short.clone()
        } else {
            long.clone()
        };
        let mut allow_empty_values = false;
        let mut required = false;
        let mut flag = false;

        while let Ok((n, symbol)) = match_symbol(next) {
            match symbol {
                '!' => {
                    ensure!(!flag, "incompatible symbols: `!` and `?`");
                    required = true;
                }
                '?' => {
                    ensure!(!required, "incompatible symbols: `?` and `!`");
                    ensure!(!allow_empty_values, "incompatible symbols: `?` and `*`");
                    ensure!(index.is_none(), "incompatible symbols: `?` and `<num>:`");
                    flag = true;
                }
                '*' => {
                    ensure!(!flag, "incompatible symbols: `*` and `?`");
                    allow_empty_values = true;
                }
                _ => unreachable!(),
            };
            next = n;
        }

        let (next, _) = match_literal("}")(next)?;

        let long = if long.is_empty() { None } else { Some(long) };
        let short = short.chars().next();

        let arg = {
            if flag {
                Unit::Flag(ClapFlag { name, long, short })
            } else if let Some(index) = index {
                Unit::Positional(ClapPositional {
                    name,
                    allow_empty_values,
                    required,
                    index,
                })
            } else {
                Unit::Option(ClapOption {
                    name,
                    long,
                    short,
                    allow_empty_values,
                    required,
                })
            }
        };

        Ok((next, arg))
    }
}

fn match_unit(input: &str) -> Result<(&str, Unit)> {
    let (next, text) = match_until_custom_arg_start(input);

    if text.is_empty() {
        if let Ok((next, _)) = match_literal("\\\\")(next) {
            Ok((next, Unit::Text("\\".to_string())))
        } else {
            match_custom_arg(next)
        }
    } else {
        let unit = Unit::Text(text.to_string());
        Ok((next, unit))
    }
}

fn match_args(input: &str) -> Result<((), Command)> {
    let mut result = Vec::new();
    let mut next = input;
    while !next.is_empty() {
        let (inner_next, unit) = match_unit(next)?;
        result.push(unit);
        next = inner_next;
    }
    Ok(((), result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_unit_error() {
        assert!(match_unit("").is_err());
        assert_eq!(
            match_unit("\\hello").map_err(|err| err.to_string()),
            Err("expected literal #{".to_string())
        );
        assert_eq!(
            match_unit("\\").map_err(|err| err.to_string()),
            Err("expected literal #{".to_string())
        );
    }

    #[test]
    fn test_match_unit() {
        assert_eq!(
            match_unit("hello").unwrap(),
            ("", Unit::Text("hello".to_string()))
        );
        assert_eq!(
            match_unit("hello world").unwrap(),
            ("", Unit::Text("hello world".to_string()))
        );
        assert_eq!(
            match_unit("#{hello}").unwrap(),
            (
                "",
                Unit::Option(ClapOption {
                    allow_empty_values: false,
                    long: Some("hello".to_string()),
                    short: None,
                    name: "hello".to_string(),
                    required: false
                })
            )
        );
        assert_eq!(
            match_unit("\\\\").unwrap(),
            ("", Unit::Text("\\".to_string()))
        );
        assert_eq!(
            match_unit("\\#{hello}").unwrap(),
            ("", Unit::Text("#{hello}".to_string()))
        );
        assert_eq!(
            match_unit("hello #{world}").unwrap(),
            ("#{world}", Unit::Text("hello ".to_string()))
        );
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("cargo new $QK_PROJECT_NAME #{lib?}").unwrap(),
            vec![
                Unit::Text("cargo new $QK_PROJECT_NAME ".to_string()),
                Unit::Flag(ClapFlag {
                    name: "lib".to_string(),
                    long: Some("lib".to_string()),
                    short: None,
                })
            ]
        );
        assert_eq!(
            parse("echo one #{one} two #{two!} three #{three*} four #{four!*} five #{five*!} six #{six?} seven #{1:seven!} eight #{2:eight!*} nine #{3:nine*!} ten #{4:ten} eleven #{5:eleven*} twelve #{twelve,t} thirteen #{thirteen,h!} fourteen #{fourteen,f*} fifteen #{fifteen,i*!} sixteen #{sixteen,s!*} seventeen #{seventeen,e?} eighteen #{,g} nineteen #{,n!} twenty #{,w*} twenty-one #{,y!*} twenty-two #{,o*!} twenty-three #{,x?}").unwrap(),
            vec![
                Unit::Text("echo one ".to_string()),
                Unit::Option(ClapOption {
                    name:"one".to_string(),
                    long:Some("one".to_string()),
                    short:None,
                    allow_empty_values: false,
                    required: false
                }),
                Unit::Text(" two ".to_string()),
                Unit::Option(ClapOption {
                    name:"two".to_string(),
                    long:Some("two".to_string()),
                    short:None,
                    allow_empty_values: false,
                    required: true
                }),
                Unit::Text(" three ".to_string()),
                Unit::Option(ClapOption {
                    name:"three".to_string(),
                    long:Some("three".to_string()),
                    short:None,
                    allow_empty_values: true,
                    required: false
                }),
                Unit::Text(" four ".to_string()),
                Unit::Option(ClapOption {
                    name:"four".to_string(),
                    long:Some("four".to_string()),
                    short:None,
                    allow_empty_values: true,
                    required: true
                }),
                Unit::Text(" five ".to_string()),
                Unit::Option(ClapOption {
                    name:"five".to_string(),
                    long:Some("five".to_string()),
                    short:None,
                    allow_empty_values: true,
                    required: true
                }),
                Unit::Text(" six ".to_string()),
                Unit::Flag(ClapFlag {
                    name:"six".to_string(),
                    long:Some("six".to_string()),
                    short:None,
                }),
                Unit::Text(" seven ".to_string()),
                Unit::Positional(ClapPositional {
                    name:"seven".to_string(),
                    allow_empty_values:false,
                    required:true,
                    index: 1
                }),
                Unit::Text(" eight ".to_string()),
                Unit::Positional(ClapPositional {
                    name:"eight".to_string(),
                    allow_empty_values:true,
                    required:true,
                    index: 2
                }),
                Unit::Text(" nine ".to_string()),
                Unit::Positional(ClapPositional {
                    name:"nine".to_string(),
                    allow_empty_values:true,
                    required:true,
                    index: 3
                }),
                Unit::Text(" ten ".to_string()),
                Unit::Positional(ClapPositional {
                    name:"ten".to_string(),
                    allow_empty_values:false,
                    required:false,
                    index: 4
                }),
                Unit::Text(" eleven ".to_string()),
                Unit::Positional(ClapPositional {
                    name:"eleven".to_string(),
                    allow_empty_values:true,
                    required:false,
                    index: 5
                }),
                Unit::Text(" twelve ".to_string()),
                Unit::Option(ClapOption {
                    name:"twelve".to_string(),
                    long: Some("twelve".to_string()),
                    short: Some('t'),
                    allow_empty_values:false,
                    required:false,
                }),
                Unit::Text(" thirteen ".to_string()),
                Unit::Option(ClapOption {
                    name:"thirteen".to_string(),
                    long: Some("thirteen".to_string()),
                    short: Some('h'),
                    allow_empty_values:false,
                    required:true,
                }),
                Unit::Text(" fourteen ".to_string()),
                Unit::Option(ClapOption {
                    name:"fourteen".to_string(),
                    long: Some("fourteen".to_string()),
                    short: Some('f'),
                    allow_empty_values:true,
                    required:false,
                }),
                Unit::Text(" fifteen ".to_string()),
                Unit::Option(ClapOption {
                    name:"fifteen".to_string(),
                    long: Some("fifteen".to_string()),
                    short: Some('i'),
                    allow_empty_values:true,
                    required:true,
                }),
                Unit::Text(" sixteen ".to_string()),
                Unit::Option(ClapOption {
                    name:"sixteen".to_string(),
                    long: Some("sixteen".to_string()),
                    short: Some('s'),
                    allow_empty_values:true,
                    required:true,
                }),
                Unit::Text(" seventeen ".to_string()),
                Unit::Flag(ClapFlag {
                    name:"seventeen".to_string(),
                    long: Some("seventeen".to_string()),
                    short: Some('e'),
                }),
                Unit::Text(" eighteen ".to_string()),
                Unit::Option(ClapOption {
                    name:"g".to_string(),
                    long: None,
                    short: Some('g'),
                    allow_empty_values:false,
                    required:false,
                }),
                Unit::Text(" nineteen ".to_string()),
                Unit::Option(ClapOption {
                    name:"n".to_string(),
                    long: None,
                    short: Some('n'),
                    allow_empty_values:false,
                    required:true,
                }),
                Unit::Text(" twenty ".to_string()),
                Unit::Option(ClapOption {
                    name:"w".to_string(),
                    long: None,
                    short: Some('w'),
                    allow_empty_values:true,
                    required:false,
                }),
                Unit::Text(" twenty-one ".to_string()),
                Unit::Option(ClapOption {
                    name:"y".to_string(),
                    long: None,
                    short: Some('y'),
                    allow_empty_values:true,
                    required:true,
                }),
                Unit::Text(" twenty-two ".to_string()),
                Unit::Option(ClapOption {
                    name:"o".to_string(),
                    long: None,
                    short: Some('o'),
                    allow_empty_values:true,
                    required:true,
                }),
                Unit::Text(" twenty-three ".to_string()),
                Unit::Flag(ClapFlag {
                    name:"x".to_string(),
                    long: None,
                    short: Some('x'),
                }),
            ]
        );
    }

    #[test]
    fn test_parse_with_simple_command() {
        assert_eq!(
            parse("echo hello # world").unwrap(),
            vec![Unit::Text("echo hello # world".to_string()),]
        );
    }

    #[test]
    fn test_parse_multi_line() {
        assert_eq!(
            parse("echo one\necho two\necho three").unwrap(),
            vec![Unit::Text("echo one\necho two\necho three".to_string())]
        );
    }

    #[test]
    fn test_match_simple_args_multi_line() {
        assert_eq!(
            match_until_custom_arg_start("echo one\necho two\necho three"),
            ("", "echo one\necho two\necho three")
        )
    }

    #[test]
    fn test_parse_with_custom_args() {
        assert_eq!(
            parse("echo my name is #{1:first!} and my last name is #{2:last!}.").unwrap(),
            vec![
                Unit::Text("echo my name is ".to_string()),
                Unit::Positional(ClapPositional {
                    name: "first".to_string(),
                    required: true,
                    allow_empty_values: false,
                    index: 1
                }),
                Unit::Text(" and my last name is ".to_string()),
                Unit::Positional(ClapPositional {
                    name: "last".to_string(),
                    required: true,
                    allow_empty_values: false,
                    index: 2
                }),
                Unit::Text(".".to_string()),
            ]
        );
    }

    #[test]
    fn test_match_simple_args() {
        assert_eq!(
            match_until_custom_arg_start("hello world"),
            ("", "hello world")
        );
        assert_eq!(
            match_until_custom_arg_start("hello #{world."),
            ("#{world.", "hello ")
        );
        assert_eq!(match_until_custom_arg_start("#{hello"), ("#{hello", ""));
        assert_eq!(
            match_until_custom_arg_start("hello #world"),
            ("", "hello #world")
        );
        assert_eq!(
            match_until_custom_arg_start("hello {world"),
            ("", "hello {world")
        );
        assert_eq!(match_until_custom_arg_start("{{hello"), ("{{hello", ""));
        assert_eq!(
            match_until_custom_arg_start("{{hello world"),
            ("{{hello world", "")
        );
        assert_eq!(
            match_until_custom_arg_start("hello {{world"),
            ("{{world", "hello ")
        );
        assert_eq!(
            match_until_custom_arg_start("{hello world"),
            ("", "{hello world")
        );
        assert_eq!(
            match_until_custom_arg_start("hello \\world"),
            ("\\world", "hello ")
        );
        assert_eq!(
            match_until_custom_arg_start("\\#{hello}"),
            ("\\#{hello}", "")
        );
        assert_eq!(
            match_until_custom_arg_start("hello \\#{world}"),
            ("\\#{world}", "hello ")
        );
        assert_eq!(
            match_until_custom_arg_start("\\{{hello}}"),
            ("\\{{hello}}", "")
        );
        assert_eq!(
            match_until_custom_arg_start("hello \\{{world}}"),
            ("\\{{world}}", "hello ")
        );
    }

    #[test]
    fn test_match_custom_arg() {
        assert_eq!(
            match_custom_arg("#{color}").unwrap(),
            (
                "",
                Unit::Option(ClapOption {
                    allow_empty_values: false,
                    long: Some("color".to_string()),
                    name: "color".to_string(),
                    short: None,
                    required: false
                })
            )
        );
        assert_eq!(
            match_custom_arg("\\#{color}").unwrap(),
            ("", Unit::Text("#{color}".to_string()))
        );
        assert_eq!(
            match_custom_arg("\\#{hello} world").unwrap(),
            ("", Unit::Text("#{hello} world".to_string()))
        );
        assert_eq!(
            match_custom_arg("\\#{hello} #{world}").unwrap(),
            ("#{world}", Unit::Text("#{hello} ".to_string()))
        );
    }

    #[test]
    fn test_match_custom_arg_starting_with_number() {
        assert_eq!(
            match_custom_arg("#{1color}").unwrap(),
            (
                "",
                Unit::Option(ClapOption {
                    allow_empty_values: false,
                    long: Some("1color".to_string()),
                    name: "1color".to_string(),
                    short: None,
                    required: false
                })
            )
        );
    }

    #[test]
    fn test_match_name() {
        assert_eq!(match_name("hello"), ("", "hello".to_string()));
        assert_eq!(match_name("hello world"), ("", "hello world".to_string()));
        assert_eq!(match_name("1hello"), ("", "1hello".to_string()));
        assert_eq!(match_name("!hello"), ("!hello", "".to_string()));
        assert_eq!(match_name("hello? world"), ("? world", "hello".to_string()));
        assert_eq!(match_name("hello! world"), ("! world", "hello".to_string()));
        assert_eq!(match_name("hello *world"), ("*world", "hello ".to_string()));
        assert_eq!(match_name("hello,world"), (",world", "hello".to_string()));
        assert_eq!(match_name("hello}world"), ("}world", "hello".to_string()));
    }

    #[test]
    fn test_match_name_with_empty_string() {
        assert_eq!(match_name(""), ("", String::new()));
    }

    #[test]
    fn test_match_literal_with_empty_string() {
        assert!(match_literal(":")("").is_err())
    }
}
