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
const LINE_START: &str = ">> ";

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use user_lib::console::getchar;
use user_lib::{close, dup, exec, fork, open, pipe, waitpid, shutdown, OpenFlags, toggle_trace, chdir};

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

fn preliminary_test() {
    let mut preliminary_apps = Vec::new();
    preliminary_apps.push("times\0");
    preliminary_apps.push("gettimeofday\0");
    preliminary_apps.push("sleep\0");
    preliminary_apps.push("brk\0");
    preliminary_apps.push("clone\0");
    // preliminary_apps.push("close\0");
    preliminary_apps.push("dup2\0");
    preliminary_apps.push("dup\0");
    preliminary_apps.push("execve\0");
    preliminary_apps.push("exit\0");
    preliminary_apps.push("fork\0");
    preliminary_apps.push("fstat\0");
    preliminary_apps.push("getcwd\0");
    preliminary_apps.push("getdents\0");
    preliminary_apps.push("getpid\0");
    preliminary_apps.push("getppid\0");
    preliminary_apps.push("mkdir_\0");
    preliminary_apps.push("mmap\0");
    preliminary_apps.push("munmap\0");
    preliminary_apps.push("mount\0");
    preliminary_apps.push("openat\0");
    preliminary_apps.push("open\0");
    preliminary_apps.push("pipe\0");
    preliminary_apps.push("read\0");
    preliminary_apps.push("umount\0");
    preliminary_apps.push("uname\0");
    preliminary_apps.push("wait\0");
    preliminary_apps.push("waitpid\0");
    preliminary_apps.push("write\0");
    preliminary_apps.push("yield\0");
    preliminary_apps.push("unlink\0");
    preliminary_apps.push("chdir\0");
    preliminary_apps.push("close\0");

    for app_name in preliminary_apps {
        let pid = fork();
        if pid == 0 {
            exec(app_name, &[core::ptr::null::<u8>()]);
        } else {
            let mut exit_code = 0;
            waitpid(pid as usize, &mut exit_code);
        }
    };
}

pub fn print_linestart(cwd: &str) {
    print!("root@JKXS-OS:{} # ",cwd);
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
    let mut cwd = String::from("/");
    print_linestart(cwd.as_str());
    loop {
        let c = getchar();
        match c {
            LF | CR => {
                println!("");
                if !line.is_empty() {
                    if line == "trace" {
                        toggle_trace();
                        line.clear();
                        print_linestart(cwd.as_str());
                        continue;
                    } else if line == "usertests" {
                        preliminary_test();
                        line.clear();
                        print_linestart(cwd.as_str());
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
                                },
                                2 => arg_copy[1].as_str(),
                                _ => {
                                    println!("cd: too many arguments");
                                    line.clear();
                                    print_linestart(cwd.as_str());
                                    continue;
                                }
                            };
                            if chdir(path) == -1 {
                                println!("cd: {}: No such file or directory", path);
                            } else {
                                let old_cwd = cwd.clone();
                                let mut cwdv: Vec<&str> = old_cwd.as_str().split("/").filter(|x| *x != "").collect();
                                let pathv: Vec<&str> = path.split("/")
                                    .map(|x| x.trim_end_matches("\0"))
                                    .filter(|x| *x != "").collect();
                                for &path_element in pathv.iter() {
                                    if path_element == "." {
                                        continue;
                                    } else if path_element == ".." {
                                        cwdv.pop();
                                    } else {
                                        cwdv.push(path_element);
                                    }
                                }
                                cwd = String::from("/");
                                for &cwd_element in cwdv.iter() {
                                    cwd.push_str(cwd_element);
                                    cwd.push('/');
                                }
                            }
                            line.clear();
                            print_linestart(cwd.as_str());
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
                    }
                    line.clear();
                }
                print_linestart(cwd.as_str());
            }
            BS | DL => {
                if !line.is_empty() {
                    print!("{}", BS as char);
                    print!(" ");
                    print!("{}", BS as char);
                    line.pop();
                }
            }
            HT => {
                if !line.is_empty() && "busybox".starts_with(&line) {
                    let line_add = "busybox".trim_start_matches(&line);
                    print!("{}", line_add);
                    line.push_str(line_add);
                }
            }
            _ => {
                print!("{}", c as char);
                line.push(c as char);
            }
        }
    }
}
