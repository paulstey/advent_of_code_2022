/// Module to wrap the parsers needed to parse the input file into commands
use anyhow::{anyhow, Result};
use nom::{
    branch,
    bytes::complete,
    character::complete::{alpha1, space1, u32},
    combinator, multi, sequence, Finish, IResult,
};
use std::{cell::RefCell, rc::Rc};

use crate::directory::{Cmd, Directory, DirectoryRef, File, FileSystemObj};

/// Nom parser for "dir bacon" -> Rc<RefCell<Directory { label: "bacon" }>>
fn dir(s: &str) -> IResult<&str, DirectoryRef> {
    let (s, label) = sequence::preceded(complete::tag("dir "), alpha1)(s)?;
    let dir = Directory::from(label);
    Ok((s, Rc::new(RefCell::new(dir))))
}

/// Nom parser for "123 eggs.txt" -> File { size: 123, label: "eggs.txt" }
fn file(s: &str) -> IResult<&str, File> {
    let (s, (size, label)) = sequence::separated_pair(u32, space1, complete::take_until("\n"))(s)?;
    let file = File { size, label };
    Ok((s, file))
}

/// Nom parser for parsing one of the file objects listed from an `ls` command
fn fs_obj(s: &str) -> IResult<&str, FileSystemObj> {
    branch::alt((
        combinator::map(dir, FileSystemObj::Directory),
        combinator::map(file, FileSystemObj::File),
    ))(s)
}

/// Nom parser for a list of newline separated results from an `ls` command
fn contents(s: &str) -> IResult<&str, Vec<FileSystemObj>> {
    multi::separated_list1(complete::tag("\n"), fs_obj)(s)
}

/// Nom parser for the various `cd` commands
fn cd_cmd(s: &str) -> IResult<&str, Cmd> {
    let (s, cmd_str) = sequence::preceded(complete::tag("$ cd "), complete::take_until("\n"))(s)?;
    let cmd = match cmd_str {
        "/" => Cmd::MoveRoot,
        ".." => Cmd::MoveUp,
        _ => Cmd::MoveIn(Directory::from(cmd_str)),
    };
    Ok((s, cmd))
}

/// Nom parser for the `ls` command. Grabs the command line and the lines
/// that follow listing the files and directories.
fn ls_cmd(s: &str) -> IResult<&str, Cmd> {
    let (s, listed) = sequence::preceded(complete::tag("$ ls\n"), contents)(s)?;
    Ok((s, Cmd::List(listed)))
}

/// Nom parser for either a `cd` or `ls` command
fn command(s: &str) -> IResult<&str, Cmd> {
    branch::alt((cd_cmd, ls_cmd))(s)
}

/// Nom parser to parse all commands from the input into a list of Cmd
pub fn commands(s: &str) -> Result<Vec<Cmd>> {
    let result = multi::separated_list1(complete::tag("\n"), command)(s).finish();
    let (_, cmds) = result.map_err(|_| anyhow!("Could not parse commands!"))?;
    Ok(cmds)
}
