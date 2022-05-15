#![no_std]
#![no_main]
#![allow(clippy::println_empty_string)]

extern crate alloc;

#[macro_use]
extern crate user_lib;

const LF: u8 = 0x0au8;
const CR: u8 = 0x0du8;
const DL: u8 = 0x7fu8;
const BS: u8 = 0x08u8;
const HT: u8 = 0x09u8;
const SP: u8 = 0x20u8;
const ESC: u8 = 0x1bu8;
const SBK: u8 = 0x5bu8;

const KEY_UP: u8 = 65u8;
const KEY_DOWN: u8 = 66u8;
const KEY_RIGHT: u8 = 67u8;
const KEY_LEFT: u8 = 68u8;

const CMD_HISTORY_SIZE: usize = 5;

use alloc::string::String;
use alloc::vec::Vec;
use user_lib::console::getchar;
use user_lib::{
    chdir, close, dup, exec, fork, longest_common_prefix, open, pipe, preliminary_test, readcwd,
    readdir, toggle_trace, waitpid, OpenFlags, change_cwd,
};

#[derive(Debug)]
struct ProcessArguments {
    input: String,
    output: String,
    args_copy: Vec<String>,
    args_addr: Vec<*const u8>,
}

impl ProcessArguments {
    pub fn new(command: &str) -> Self {
        let args: Vec<_> = command.split(' ').collect();
        let mut args_copy: Vec<String> = args
            .iter()
            .filter(|&arg| !arg.is_empty())
            .map(|&arg| {
                let mut string = String::new();
                string.push_str(arg);
                string.push('\0');
                string
            })
            .collect();

        // redirect input
        let mut input = String::new();
        if let Some((idx, _)) = args_copy
            .iter()
            .enumerate()
            .find(|(_, arg)| arg.as_str() == "<\0")
        {
            input = args_copy[idx + 1].clone();
            args_copy.drain(idx..=idx + 1);
        }

        // redirect output
        let mut output = String::new();
        if let Some((idx, _)) = args_copy
            .iter()
            .enumerate()
            .find(|(_, arg)| arg.as_str() == ">\0")
        {
            output = args_copy[idx + 1].clone();
            args_copy.drain(idx..=idx + 1);
        }

        let mut args_addr: Vec<*const u8> = args_copy.iter().map(|arg| arg.as_ptr()).collect();
        args_addr.push(core::ptr::null::<u8>());

        Self {
            input,
            output,
            args_copy,
            args_addr,
        }
    }
}

pub fn print_line_start(cwd: &str) {
    print!("root@JKXS-OS:{} # ", cwd);
}

pub fn start_new_line(line: &mut String, pos: &mut usize, cwd: &str) {
    line.clear();
    *pos = 0;
    print_line_start(cwd);
}

pub fn reprint_line(line: &str, line_len_inc: isize, former_pos: usize, pos: usize) {
    for _ in 0..former_pos {
        print!("{}", BS as char);
    }
    print!("{}", line);
    let final_pos = line.len() + (0.max(-line_len_inc) as usize);
    for _ in line.len()..final_pos {
        print!(" ");
    }
    for _ in 0..final_pos - pos {
        print!("{}", BS as char);
    }
}

// #[no_mangle]
// pub fn main() -> i32 {
//     preliminary_test();
//     shutdown()
// }

#[no_mangle]
pub fn main() -> i32 {
    println!("Rust user shell");
    let mut line: String = String::new();
    let mut pos: usize = 0;
    let mut cwd = String::from("/");
    let mut cwd_wl = readdir(cwd.as_str());
    let mut sub_wl = cwd_wl.clone();
    let mut search_sub_flag = false;

    let mut cmd_history = Vec::<String>::new();
    let mut cmd_history_idx = 0;

    start_new_line(&mut line, &mut pos, cwd.as_str());
    loop {
        let c = getchar();
        if c != ESC {
            cmd_history_idx = cmd_history.len();
        }
        match c {
            LF | CR => {
                println!("");
                search_sub_flag = false;
                if !line.is_empty() {
                    cmd_history.push(line.clone());
                    while cmd_history.len() > CMD_HISTORY_SIZE {
                       cmd_history.remove(0);
                    }
                    cmd_history_idx = cmd_history.len();
                    if line == "trace" {
                        toggle_trace();
                        start_new_line(&mut line, &mut pos, cwd.as_str());
                        continue;
                    } else if line == "usertests" {
                        preliminary_test();
                        start_new_line(&mut line, &mut pos, cwd.as_str());
                        continue;
                    };
                    let splited: Vec<_> = line.as_str().split('|').collect();
                    let process_arguments_list: Vec<_> = splited
                        .iter()
                        .map(|&cmd| ProcessArguments::new(cmd))
                        .collect();
                    // println!("process_arguments_list: {:?}", process_arguments_list);
                    let mut valid = true;
                    for (i, process_args) in process_arguments_list.iter().enumerate() {
                        if i == 0 {
                            if !process_args.output.is_empty() {
                                valid = false;
                            }
                        } else if i == process_arguments_list.len() - 1 {
                            if !process_args.input.is_empty() {
                                valid = false;
                            }
                        } else if !process_args.output.is_empty() || !process_args.input.is_empty()
                        {
                            valid = false;
                        }
                    }
                    if process_arguments_list.len() == 1 {
                        valid = true;
                        let arg_copy = &process_arguments_list[0].args_copy;
                        if arg_copy[0] == "cd\0" {
                            let path = match arg_copy.len() {
                                1 => {
                                    cwd = String::from("/");
                                    "/\0"
                                }
                                2 => arg_copy[1].as_str(),
                                _ => {
                                    println!("cd: too many arguments");
                                    start_new_line(&mut line, &mut pos, cwd.as_str());
                                    continue;
                                }
                            };
                            match chdir(path) {
                                -1 => println!("cd: {}: No such file or directory", path),
                                -20 => println!("cd: {}: Not a directory", path),
                                _ => {
                                    cwd = change_cwd(cwd.as_str(), path);
                                    cwd_wl = readdir(cwd.as_str()); // cd后重新读取工作目录下的wordlist
                                }
                            }
                            start_new_line(&mut line, &mut pos, cwd.as_str());
                            continue;
                        }
                    }
                    if !valid {
                        println!("Invalid command: Inputs/Outputs cannot be correctly binded!");
                    } else {
                        // create pipes
                        let mut pipes_fd: Vec<[usize; 2]> = Vec::new();
                        if !process_arguments_list.is_empty() {
                            for _ in 0..process_arguments_list.len() - 1 {
                                let mut pipe_fd = [0usize; 2];
                                pipe(&mut pipe_fd);
                                pipes_fd.push(pipe_fd);
                            }
                        }
                        let mut children: Vec<_> = Vec::new();
                        for (i, process_argument) in process_arguments_list.iter().enumerate() {
                            let pid = fork();
                            if pid == 0 {
                                let input = &process_argument.input;
                                let output = &process_argument.output;
                                let args_copy = &process_argument.args_copy;
                                let args_addr = &process_argument.args_addr;
                                // redirect input
                                if !input.is_empty() {
                                    let input_fd = open(input.as_str(), OpenFlags::RDONLY);
                                    if input_fd == -1 {
                                        println!("Error when opening file {}", input);
                                        return -4;
                                    }
                                    let input_fd = input_fd as usize;
                                    close(0);
                                    assert_eq!(dup(input_fd), 0);
                                    close(input_fd);
                                }
                                // redirect output
                                if !output.is_empty() {
                                    let output_fd = open(
                                        output.as_str(),
                                        OpenFlags::CREATE | OpenFlags::WRONLY,
                                    );
                                    if output_fd == -1 {
                                        println!("Error when opening file {}", output);
                                        return -4;
                                    }
                                    let output_fd = output_fd as usize;
                                    close(1);
                                    assert_eq!(dup(output_fd), 1);
                                    close(output_fd);
                                }
                                // receive input from the previous process
                                if i > 0 {
                                    close(0);
                                    let read_end = pipes_fd.get(i - 1).unwrap()[0];
                                    assert_eq!(dup(read_end), 0);
                                }
                                // send output to the next process
                                if i < process_arguments_list.len() - 1 {
                                    close(1);
                                    let write_end = pipes_fd.get(i).unwrap()[1];
                                    assert_eq!(dup(write_end), 1);
                                }
                                // close all pipe ends inherited from the parent process
                                for pipe_fd in pipes_fd.iter() {
                                    close(pipe_fd[0]);
                                    close(pipe_fd[1]);
                                }
                                // execute new application
                                if exec(args_copy[0].as_str(), args_addr.as_slice()) == -1 {
                                    println!("Error when executing!");
                                    return -4;
                                }
                                unreachable!();
                            } else {
                                children.push(pid);
                            }
                        }
                        for pipe_fd in pipes_fd.iter() {
                            close(pipe_fd[0]);
                            close(pipe_fd[1]);
                        }
                        let mut exit_code: i32 = 0;
                        for pid in children.into_iter() {
                            let exit_pid = waitpid(pid as usize, &mut exit_code);
                            assert_eq!(pid, exit_pid);
                            //println!("Shell: Process {} exited with code {}", pid, exit_code);
                        }
                        cwd_wl = readdir(cwd.as_str()); // 有可能有更改工作目录下目录项的操作
                    }
                }
                start_new_line(&mut line, &mut pos, cwd.as_str());
            }
            BS | DL => {
                if pos > 0 {
                    search_sub_flag = false;
                    line.remove(pos - 1);
                    pos -= 1;
                    reprint_line(&line, -1, pos + 1, pos);
                    // print!("{}", BS as char);
                    // print!(" ");
                    // print!("{}", BS as char);
                }
            }
            HT => {
                if pos < line.len() {
                    continue;
                }
                let wordv: Vec<&str> = line.as_str().split(' ').collect();
                let space_word = wordv.last().unwrap();
                let wordv: Vec<&str> = space_word.split('/').collect();
                let slash_word = wordv.last().unwrap();
                if !slash_word.is_empty() {
                    if space_word != slash_word {  // 还可以加个search_dir_flag来加快二次tab的搜索效率
                        let search_path = space_word.rsplit_once('/').unwrap().0;
                        let mut search_dir: String;
                        if space_word.starts_with('/') {
                            search_dir = change_cwd("/", search_path);
                        } else {
                            search_dir = change_cwd(cwd.as_str(), search_path);
                        }
                        sub_wl = readdir(search_dir.as_str());
                        search_sub_flag = true;
                    }
                    if search_sub_flag {
                        sub_wl = sub_wl.into_iter().filter(|x| x.starts_with(slash_word)).collect();
                    // 在sub_wl中找以line为首的word
                    } else {
                        sub_wl = cwd_wl
                            .clone()
                            .into_iter()
                            .filter(|x| x.starts_with(slash_word))
                            .collect(); // 在cwd_wl中找以line为首的word
                        search_sub_flag = true;
                    }
                    if sub_wl.len() == 0 {
                        continue;
                    }
                    // 找到最长公共前缀
                    let longest_prefix = longest_common_prefix(&sub_wl);
                    if longest_prefix == *slash_word {
                        // 如果相等，则展示sub_wl
                        println!("\n{:#?}", sub_wl);
                        print_line_start(cwd.as_str());
                        print!("{}", line);
                    } else {
                        // 不相等则补全
                        let word_add = longest_prefix.trim_start_matches(slash_word);
                        print!("{}", word_add);
                        line.push_str(word_add);
                        pos += word_add.len();
                    }
                }
            }
            ESC => {
                if getchar() == SBK {
                    //up 65, down 66, right 67, left 68
                    match getchar() {
                        KEY_UP => {
                            if cmd_history.len() == 0 {
                                continue;
                            }
                            if cmd_history_idx > 0 {
                                cmd_history_idx -= 1;
                            }
                            reprint_line("", -(line.len() as isize), pos, 0);
                            line = cmd_history[cmd_history_idx].clone();
                            pos = line.len();
                            reprint_line(&line, line.len() as isize, 0, pos);
                        }
                        KEY_DOWN => {
                            if cmd_history.len() == 0 {
                                continue;
                            }
                            if cmd_history_idx < cmd_history.len() {
                                cmd_history_idx += 1;
                            }
                            reprint_line("", -(line.len() as isize), pos, 0);
                            if cmd_history_idx < cmd_history.len() {
                                line = cmd_history[cmd_history_idx].clone();
                            } else {
                                line = String::new();
                            }
                            pos = line.len();
                            reprint_line(&line, line.len() as isize, 0, pos);
                        }
                        KEY_LEFT => {
                            if pos > 0 {
                                pos -= 1;
                                print!("{}", BS as char);
                            }
                        }
                        KEY_RIGHT => {
                            if pos < line.len() {
                                print!("{}", line.chars().nth(pos).unwrap() as char);
                                pos += 1;
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {
                if c == SP {
                    search_sub_flag = false;
                }
                line.insert(pos, c as char);
                pos += 1;
                reprint_line(&line, 1, pos - 1, pos);
            }
        }
    }
}
