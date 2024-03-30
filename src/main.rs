// use rand::Rng;
use std::env;
use std::process::{Child, Command};
use std::{thread, time::Duration};

fn main() {
    // test_helper();

    let check_interval = 5;

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Please input your command, for example: 'bash a.sh'");
        return;
    }

    let cmd = args[1].to_string();

    let mut child = exec_command(&cmd);

    loop {
        match child.try_wait() {
            Ok(Some(_)) => {
                // 已退出
                println!(
                    "\n\n=====================================================\n\
                Target child process has exited, it will be restart.\n\
                =====================================================\n\n"
                );
                child = exec_command(&cmd);
            }
            Ok(None) => {
                // println!("Target child process is running.");
            }
            Err(_) => {
                // 无法检查子进程状态
                println!("Can't get status of child process, exit.");
                std::process::exit(1);
            }
        }

        thread::sleep(Duration::from_secs(check_interval));
    }
}

fn exec_command(cmd: &String) -> Child {
    let child = Command::new(cmd)
        .spawn()
        .expect("Failed to execute the command.");

    return child;
}

// fn test_helper() {
//     let mut i = 0;
//     loop {
//         println!("Test info i: {i}");
//         i += 1;

//         let mut rng = rand::thread_rng();
//         let num = rng.gen_range(0..10);
//         if num > 8 {
//             println!("Exit.");
//             break;
//         }

//         thread::sleep(Duration::from_secs(1));
//     }
// }
