#![no_std]
#![no_main]

use core::{convert::TryInto, panic, str};

extern crate alloc;

#[macro_use]
extern crate user_lib;

const LF: u8 = 0x0au8;
const CR: u8 = 0x0du8;
const DL: u8 = 0x7fu8;
const BS: u8 = 0x08u8;
const CSI:u8 = 0x1bu8;
const DEL:u8 = 0x7eu8;

macro_rules! color_text {
    ($text:expr, $color:expr) => {{
        format_args!("\x1b[{}m{}\x1b[0m", $color, $text)
    }};
}

macro_rules! cursor_move_left {
    ($x:literal ) => {
        if $x > 0{
            print!("\x1b[{}D", ($x));//when $x=0, it will still view $x as 1
        }
    };

    ($x:expr ) => {
        if $x > 0{
            print!("\x1b[{}D", ($x));
        }
    };
}
macro_rules! cursor_move_right {
    ($x:literal ) => {
        if $x > 0{
            print!("\x1b[{}C",($x))//when $x=0, it will still view $x as 1
        }
    };
    ($x:expr ) => {
        if $x > 0{
            print!("\x1b[{}C",($x))
        }
    };
}

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use user_lib::*;
use user_lib::console::getchar;


const STATE_IDLE:   u8 = 0;
const STATE_PANIC:  u8 = 1;
const STATE_CSI:    u8 = 2;
const STATE_CSI_1:  u8 = 3;
const STATE_CSI_2:  u8 = 4;
pub struct InputMachine{
    state: u8,
    p: usize, // position of input string 
    cmd: String,
    parent_dir_inode_id: usize
}




/// detail of InputMachine is in doc "shell.md"
impl InputMachine{

    pub fn new() -> Self{
        Self{
            state: STATE_IDLE,
            p: 0,
            cmd: String::new(),
            // path: Vec::new(),
            parent_dir_inode_id: 0
        }
    }

    pub fn get_cmd(&mut self) -> &str{
        self.cmd.as_str()
    }

    pub fn clear(&mut self){
        self.state = STATE_IDLE;
        self.p = 0;
        self.cmd.clear();
        // self.path.clear();
        self.parent_dir_inode_id = 0;
    }

    pub fn operate(&mut self, c: char) -> bool{
        // println!("(get char {})",c as u8);
        match self.state {
            STATE_IDLE=>{
                match c as u8 {
                    CSI => {
                        self.state = STATE_CSI;
                    }
                    LF | CR=>{
                        println!("");
                        return true;
                    }
                    BS | DL => {
                        if self.p > 0 {// delete char
                            self.p -= 1;
                            self.cmd.remove(self.p);

                            cursor_move_left!(self.p+1);
                            print!("{} ",self.cmd.as_str());
                            cursor_move_left!(self.cmd.len() - self.p +1);//assert len>=p

                        }
                    }
                    _ =>{
                        if self.cmd.len() == self.p{
                            self.cmd.insert(self.p, c);
                            print!("{}",c);

                        }
                        else{
                            self.cmd.insert(self.p, c);
                            cursor_move_left!(self.p);
                            
                            // cursor_move_right!(1);
                            print!("{}",self.cmd.as_str());
                            cursor_move_left!(self.cmd.len() - self.p - 1);//assert len>p
                        }
                        self.p += 1;
                    }
                }
            }
            STATE_CSI=>{
                if c == '['{
                    self.state = STATE_CSI_1;
                }
                else{
                    self.state = STATE_PANIC;
                    panic!("Shell input not recognized!(STATE_CSI)");
                }
            }
            STATE_CSI_1=>{
                match c as u8 {
                    51 => {
                        self.state = STATE_CSI_2;
                    }
                    68 => {// KEY LEFT
                        self.state = STATE_IDLE;
                        if self.p > 0 {
                            print!("{}{}{}", CSI as char, '[', c);
                            self.p -= 1;
                        }
                    }
                    67 => {// KEY RIGHT
                        self.state = STATE_IDLE;
                        if self.p < self.cmd.len() {
                            print!("{}{}{}", CSI as char, '[', c);
                            self.p += 1;
                        }
                    }
                    65 | 66 =>{//KEY UP/DOWN not support now
                        self.state = STATE_IDLE;
                    }
                    _ =>{
                        self.state = STATE_PANIC;
                        panic!("Shell input not recognized!(STATE_CSI_1)");
                    }
                }
            }
            STATE_CSI_2=>{
                if c == DEL as char{
                    self.state = STATE_IDLE;
                    if self.cmd.len() == self.p{
                        cursor_move_left!(1);
                        print!(" ");
                        cursor_move_left!(1);
                    }
                    else{
                        cursor_move_left!(self.p+1);
                        print!("{} ",self.cmd.as_str());
                        cursor_move_left!(self.cmd.len() - self.p +1);//assert len>=p
                    }
                    self.cmd.remove(self.p);
                    self.p -= 1;
                }
                else{
                    self.state = STATE_PANIC;
                    panic!("Shell input not recognized!(STATE_CSI_2)");
                }
            }
            _ =>{
                panic!("Shell machine state not recognized!");
            }
        }
        return false;
    }
    
    

}

const STATE_ARGS:   u8 = 2;
pub struct ArgMachine{
    args: Vec<String>,
    argc: usize,
    state: u8,
    path: Vec<String>
}

impl ArgMachine{

    fn print_root(&mut self){
        print!("{}@UltraOS: /",color_text!("root",32));
        self.path.iter().for_each(|string|
            print!("{}/", string)
        );
        print!(" >>");
    }

    pub fn new () -> Self{
        let mut new_self = Self{
            args: Vec::new(),
            argc: 0,
            state: 0,
            path: Vec::new()
        };
        new_self.print_root();
        new_self
    }

    // not clear path

    pub fn clear(&mut self){
        // println!{"<<<<<<<<<<<<<<<<pin1"}
        self.args.clear();
        // println!{"<<<<<<<<<<<<<<<<pin2"}
        self.argc = 0;
        // println!{"<<<<<<<<<<<<<<<<pin3"}
        self.state = STATE_IDLE;
        // println!{"<<<<<<<<<<<<<<<<pin4"}
        self.print_root();
    }

    pub fn operate(&mut self, c:char){
        match self.state {
            STATE_IDLE=>{
                match c {
                    ' ' => {
                        self.state = STATE_IDLE;
                    }
                    _ =>{//start of an arg
                        self.args.push(String::new());
                        self.args[self.argc].push(c);
                        self.argc += 1;
                        self.state = STATE_ARGS;
                    }
                }
            }
            STATE_ARGS=>{
                match c {
                    ' ' => {// end of one arg
                        self.args[self.argc-1].push(0 as char);
                        self.state = STATE_IDLE;
                    }
                    _ =>{
                        self.args[self.argc-1].push(c);
                        self.state = STATE_ARGS;
                    }
                }
            }
            _ =>{
                panic!("Arg machine state not recognized!");
            }
        }
    }

    // @return: true -> valid "exec" argc (op "cd" not included)
    pub fn operate_str(&mut self, str: &str) -> bool {
        for c in str.chars() {
            self.operate(c);
        }
        if self.state == STATE_ARGS{
            self.args[self.argc-1].push(0 as char);
        }
        // self.print_state();
        // \0 indicates the end of str while rust doesn't do so
        if self.args.is_empty(){
            return false;
        }
        // cd
        if self.args[0].clone().as_str() == "cd\0" {
            unimplemented!();
        }
        return true;
    }

    pub fn get_args(&mut self) -> (Vec<String>, String, String){
        //copy args
        let mut args_copy: Vec<String> = Vec::new();
        self.args
            .iter_mut()
            .for_each(|string| {
                args_copy.push(string.clone());
            });

        // redirect input
        let mut input = String::new();
        if let Some((idx, _)) = args_copy
        .iter()
        .enumerate()
        .find(|(_, arg)| arg.as_str() == "<\0") {
            input = args_copy[idx + 1].clone();
            args_copy.drain(idx..=idx + 1);
        }

        // redirect output
        let mut output = String::new();
        if let Some((idx, _)) = args_copy
        .iter()
        .enumerate()
        .find(|(_, arg)| arg.as_str() == ">\0") {
            output = args_copy[idx + 1].clone();
            args_copy.drain(idx..=idx + 1);
        };

        (args_copy, input, output)
    }

    // Assume "self.args' meet "cd" requirements
    pub fn change_dir(&mut self) {
        unimplemented!();
    //     let mut cd_path = self.args[1].clone();
    //     if chdir(cd_path.as_str()) == -1{
    //         println!("cd: No such directory!");
    //         return;
    //     }

    //     cd_path.pop();// clear '\0' at the end of str
    //     let mut cd_path_vec:Vec<String> = Vec::new();
    //     cd_path.as_str().split('/').for_each(
    //         |str| cd_path_vec.push(String::from(str))
    //     );

    //     let is_from_root = cd_path_vec[0].is_empty(); // start from '/' root
    //     if is_from_root {
    //         self.path.clear();
    //     }
    //     cd_path_vec.iter().for_each(
    //         |string| // name of every single directory entry(eg. /hello/world -> "hello","world")
    //         if !string.is_empty() && string.as_str() != "."{
    //             if string.as_str() == ".."{
    //                 self.path.pop();
    //             }
    //             else{
    //                 self.path.push(string.clone());
    //             }
    //         }
    //     );
    }
}


fn get_args_addr(op:&String)->Vec<*const u8>{
    let args: Vec<&str> = op.as_str().split(' ').collect();
    // for i in 0..args.len() {
    //     args[i].push('\0');
    // }
    let mut args_addr: Vec<*const u8> = Vec::new();
    for i in 0..args.len() {
        //println!("{:?}", args[i]);
        args_addr.push(args[i].as_ptr() as usize as *const u8);
    }
    args_addr.push(0 as *const u8 );
    args_addr
}


#[no_mangle]
pub fn main() -> i32 {
    let mut line: String;
    let mut shellmachine = InputMachine::new();
    let mut arg_machine = ArgMachine::new();
    loop {
        // println!{"<<<<<<<<<entering the loop of input"}
        let c = getchar();
        let is_exec = shellmachine.operate(c as char);
        if is_exec {
            line = String::from(shellmachine.get_cmd());
            let is_exec = arg_machine.operate_str(shellmachine.get_cmd());
            if line.is_empty() || !is_exec{
                shellmachine.clear();
                arg_machine.clear();
                continue;
            }
            println!("Input:{}",line);
            let (args_copy,input,output) = arg_machine.get_args();
            // println!{"args_copying..."}
            let mut args_addr: Vec<*const u8> = args_copy
            .iter()
            .map(|arg| arg.as_ptr())
            .collect();

            args_addr.push(0 as *const u8);
            // println!{"pid forking..."}
            let pid = fork();
            if pid == 0 {
                // child process
                if exec(args_copy[0].as_str(), args_addr.as_slice()) == -1 {
                    println!("Error when executing!");
                    return -4;
                }
                unreachable!();
            } else {
                let mut exit_code: i32 = 0;
                // println!{"<<<<<<<<<waiting pid of exec"}
                let exit_pid = waitpid(pid as usize, &mut exit_code);
                // println!{"<<<<<<<<<back of the pid exec"}
                assert_eq!(pid, exit_pid);
                println!("Shell: Process {} exited with code {}", pid, exit_code);
                shellmachine.clear();
                // println!{"<<<<<<<<<end of the shell cleaning"}
                arg_machine.clear();
                // println!{"<<<<<<<<<end of the arg cleaning"}
            }
        }
    }
}