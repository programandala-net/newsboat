extern crate libnewsboat;
extern crate tempfile;

use self::libnewsboat::cliargsparser::CliArgsParser;
use self::tempfile::TempDir;
use std::{env, path::PathBuf};

#[test]
fn t_cliargsparser_dash_i_resolves_tilde_to_homedir() {
    let tmp = TempDir::new().unwrap();

    env::set_var("HOME", tmp.path());

    let filename = "feedlist.opml";
    let arg = format!("~/{}", filename);

    let check = |opts| {
        let args = CliArgsParser::new(opts);
        assert_eq!(
            args.importfile,
            Some(PathBuf::from(tmp.path().join(filename)))
        );
    };

    check(vec![
        "newsboat".to_string(),
        "-i".to_string(),
        arg.to_string(),
    ]);

    check(vec![
        "newsboat".to_string(),
        "--import-from-opml=".to_string() + &arg,
    ]);
}
