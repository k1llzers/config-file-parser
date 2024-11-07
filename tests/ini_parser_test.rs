use pest::Parser;
use anyhow::anyhow;
use config_file_parser::*;

#[test]
fn comment_test() -> anyhow::Result< () > {
    let pair = INIGrammar::parse(Rule::comment, ";comment")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), ";comment" );
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 8 );

    let pair = INIGrammar::parse(Rule::comment, "");
    assert!(pair.is_err());

    let pair = INIGrammar::parse(Rule::comment, "comment");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn value_test() -> anyhow::Result< () > {
    let pair = INIGrammar::parse(Rule::value, "aswrdsf%$#sd123")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), "aswrdsf%$#sd123" );
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 15 );

    let pair = INIGrammar::parse(Rule::value, "");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn key_test() -> anyhow::Result< () > {
    let pair = INIGrammar::parse(Rule::key, "AaasSss")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), "AaasSss" );
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 7 );

    let pair = INIGrammar::parse(Rule::key, "AAA$%#")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), "AAA" ); // correct for single key test because key is only sequence of alphabetic
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 3 );

    let pair = INIGrammar::parse(Rule::key, "111$%#");
    assert!(pair.is_err());

    let pair = INIGrammar::parse(Rule::key, "");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn pair_test() -> anyhow::Result< () > {
    let pair = INIGrammar::parse(Rule::pair, "key=value")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), "key=value" );
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 9 );

    let pair = INIGrammar::parse(Rule::pair, "keyvalue");
    assert!(pair.is_err());

    let pair = INIGrammar::parse(Rule::pair, "key=");
    assert!(pair.is_err());

    let pair = INIGrammar::parse(Rule::pair, "=value");
    assert!(pair.is_err());

    let pair = INIGrammar::parse(Rule::pair, "key123=value");
    assert!(pair.is_err());

    let pair = INIGrammar::parse(Rule::pair, "key%^%@=value");
    assert!(pair.is_err());

    let pair = INIGrammar::parse(Rule::pair, "");
    assert!(pair.is_err());

    let pair = INIGrammar::parse(Rule::pair, " ");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn name_test() -> anyhow::Result< () > {
    let pair = INIGrammar::parse(Rule::name, "SECTIONNAME")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), "SECTIONNAME" );
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 11 );

    let pair = INIGrammar::parse(Rule::name, "SECTIONname")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), "SECTION" );
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 7 );

    let pair = INIGrammar::parse(Rule::name, "asdasd");
    assert!(pair.is_err());

    let pair = INIGrammar::parse(Rule::name, "123412");
    assert!(pair.is_err());

    let pair = INIGrammar::parse(Rule::name, "%$&^#");
    assert!(pair.is_err());

    let pair = INIGrammar::parse(Rule::name, "");
    assert!(pair.is_err());

    let pair = INIGrammar::parse(Rule::name, " ");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn section_test() -> anyhow::Result< () > {
    let pair = INIGrammar::parse(Rule::section, "[SECTION]\nkey=value\n")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), "[SECTION]\nkey=value\n" );
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 20 );

    let pair = INIGrammar::parse(Rule::section, "[SECTION]\n key=value\n keyTwo=valueTwo\n")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), "[SECTION]\n key=value\n keyTwo=valueTwo\n" );
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 38 );

    let pair = INIGrammar::parse(Rule::section, "[SECTION]\n\n\n\n ;aaaa \n key=value\n keyTwo=valueTwo\n")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), "[SECTION]\n\n\n\n ;aaaa \n key=value\n keyTwo=valueTwo\n" );
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 49 );

    let pair = INIGrammar::parse(Rule::section, "[SECTION]\n key=valuekeyTwo=valueTwo\n")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), "[SECTION]\n key=valuekeyTwo=valueTwo\n" );
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 36 );

    let pair = INIGrammar::parse(Rule::section, "[SECTION] key=valuekeyTwo=valueTwo\n");
    assert!(pair.is_err());

    let pair = INIGrammar::parse(Rule::section, "");
    assert!(pair.is_err());

    let pair = INIGrammar::parse(Rule::section, " ");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn file_test() -> anyhow::Result< () > {
    let pair = INIGrammar::parse(Rule::file, "[SECTION]\nkey=value \n [SECTIONTWO]\nkey=value\n")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), "[SECTION]\nkey=value \n [SECTIONTWO]\nkey=value\n" );
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 45 );

    let pair = INIGrammar::parse(Rule::file, "[SECTION]\nkey=value\n;com \n [SECTIONTWO]\nkey=value\n")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), "[SECTION]\nkey=value\n;com \n [SECTIONTWO]\nkey=value\n" );
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 50 );

    let pair = INIGrammar::parse(Rule::file, "[SECTION]\nkey=value [SECTIONTWO]\nkey=value\n");
    assert!(pair.is_err());

    let pair = INIGrammar::parse(Rule::file, "")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), "" );
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 0 );

    let pair = INIGrammar::parse(Rule::file, " ")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), " " );
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 1 );

    Ok(())
}

