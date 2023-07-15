use std::io::{self, Write, stdout};
use rand::Rng;
use std::time::{Duration, Instant};
use std::thread;


fn default(){

    loop{
        let mut rng = rand::thread_rng();
        let mut usr = String::new();
        let mut usr1 = String::new();
        let mut stdout = io::stdout();
        write!(stdout, "\x1b[2J").unwrap(); 
        write!(stdout, "\x1b[1;1H").unwrap();
        stdout.flush().unwrap();

        let mut stdout = io::stdout();

        let banner = format!{
            "\n
            ********** **                    *******     ********  ******         *******                   **     
            /////**/// /**                   /**////**   **//////**/*////**       /**////**                 /**     
                /**    /**       *****       /**   /**  **      // /*   /**       /**   /**  **   **  ******/**     
                /**    /******  **///**      /*******  /**         /******        /*******  /**  /** **//// /****** 
                /**    /**///**/*******      /**///**  /**    *****/*//// **      /**///**  /**  /**//***** /**///**
                /**    /**  /**/**////       /**  //** //**  ////**/*    /**      /**  //** /**  /** /////**/**  /**
                /**    /**  /**//******      /**   //** //******** /*******       /**   //**//****** ****** /**  /**
                //     //   //  //////       //     //   ////////  ///////        //     //  ////// //////  //   // 
            
            \n\n
                sdSS_SSSSSSbs   .S_sSSs     .S   .S_sSSs     .S  sdSS_SSSSSSbs   .S S.   
        YSSS~S%SSSSSP  .SS~YS%%b   .SS  .SS~YS%%b   .SS  YSSS~S%SSSSSP  .SS SS.  
            S%S       S%S   `S%b  S%S  S%S   `S%b  S%S       S%S       S%S S%S  
            S%S       S%S    S%S  S%S  S%S    S%S  S%S       S%S       S%S S%S  
            S&S       S%S    d*S  S&S  S%S    S&S  S&S       S&S       S%S S%S  
            S&S       S&S   .S*S  S&S  S&S    S&S  S&S       S&S        SS SS   
            S&S       S&S_sdSSS   S&S  S&S    S&S  S&S       S&S         S S    
            S&S       S&S~YSY%b   S&S  S&S    S&S  S&S       S&S         SSS    
            S*S       S*S   `S%b  S*S  S*S    S*S  S*S       S*S         S*S    
            S*S       S*S    S%S  S*S  S*S    S*S  S*S       S*S         S*S    
            S*S       S*S    S&S  S*S  S*S    S*S  S*S       S*S         S*S    
            S*S       S*S    SSS  S*S  S*S    SSS  S*S       S*S         S*S    
            SP        SP          SP   SP          SP        SP          SP     
            Y         Y           Y    Y           Y         Y           Y              By: Vynndorix     
                                                                            
            "
        };


        for _ in 0..120{
            write!(stdout, "\x1b[38;2;{};{};{}m", rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>()).unwrap();
            stdout.flush().unwrap();
            thread::sleep(Duration::from_millis(50));
            
            write!(stdout, "\x1b[2J").unwrap();
            write!(stdout, "\x1b[1;1H").unwrap();
            stdout.flush().unwrap();
                    
            let r: u8 = rand::thread_rng().gen_range(0..=255);
            let g: u8 = rand::thread_rng().gen_range(0..=255);
            let b: u8 = rand::thread_rng().gen_range(0..=255);
            print!("\x1b[38;2;{};{};{}m{banner}\x1b[0m\n", r, g, b);
            
        }
        let mut stdout = io::stdout();
        write!(stdout, "\x1b[2J").unwrap(); 
        write!(stdout, "\x1b[1;1H").unwrap(); 
        stdout.flush().unwrap();

        write!(stdout, "\x1b[38;2;{};{};{}m", rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>()).unwrap();
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(50));

                    
        let r: u8 = rand::thread_rng().gen_range(0..=255);
        let g: u8 = rand::thread_rng().gen_range(0..=255);
        let b: u8 = rand::thread_rng().gen_range(0..=255);
        print!("\x1b[38;2;{};{};{}m|\x1b[0m\n", r, g, b);

        print!("\x1b[38;2;{};{};{}m\t\t\t\t\t\t\t\t\t*Note*  All Of These Might Be Chaotic Because It Needs To Clear The Screen Lol  *Note*\n\n\x1b[0m", r, g, b);

        write!(stdout, "\x1b[38;2;{};{};{}m", rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>()).unwrap();
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(50));  
                
        let r: u8 = rand::thread_rng().gen_range(0..=255);
        let g: u8 = rand::thread_rng().gen_range(0..=255);
        let b: u8 = rand::thread_rng().gen_range(0..=255);
        
        print!("\x1b[38;2;{};{};{}m\n\n\n\t\t\t\t\t\t\t1: Longest Delay\n\n\t\t\t\t\t\t\t2: Semi Long Delay\n\n\t\t\t\t\t\t\t3: Zero Delay * Might Be The Chillest *\n\n\n\n \x1b[0m", r, g, b);
        
        write!(stdout, "\x1b[38;2;{};{};{}m", rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>()).unwrap();
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(50));    

        let r: u8 = rand::thread_rng().gen_range(0..=255);
        let g: u8 = rand::thread_rng().gen_range(0..=255);
        let b: u8 = rand::thread_rng().gen_range(0..=255);


        print!("\x1b[38;2;{};{};{}mYour Input Goes Here: \x1b[0m", r, g, b);

        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut usr).expect("failed to read line put something vaild in");

        let usr_c: u8 = match usr.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input.");
                return; 
            }
        };
        let mut number:u8 = 0;
        match usr_c{
            1 => number = 100,
            2 => number = 30,
            3 => number = 0,
            _ => println!("Put a vaild number in from 1-3"), 
            
        };

        let mut stdout = io::stdout();
        write!(stdout, "\x1b[2J").unwrap(); 
        write!(stdout, "\x1b[1;1H").unwrap(); 
        stdout.flush().unwrap();

        write!(stdout, "\x1b[38;2;{};{};{}m", rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>()).unwrap();
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(50));    

        let r: u8 = rand::thread_rng().gen_range(0..=255);
        let g: u8 = rand::thread_rng().gen_range(0..=255);
        let b: u8 = rand::thread_rng().gen_range(0..=255);

        println!("\x1b[38;2;{};{};{}m\n\n\n\t\t\t\t\t\t\t1: Just Amazing RGB\n\n\t\t\t\t\t\t\t2: If You Want Just Amazing RGB With A Name :D\n\n\t\t\t\t\t\t\t3: If You Want Pure Randomness But With RGB\n\n\x1b[0m", r, g, b);
        write!(stdout, "\x1b[38;2;{};{};{}m", rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>()).unwrap();
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(50));    

        let r: u8 = rand::thread_rng().gen_range(0..=255);
        let g: u8 = rand::thread_rng().gen_range(0..=255);
        let b: u8 = rand::thread_rng().gen_range(0..=255);

        print!("\x1b[38;2;{};{};{}mYour Input Goes Here: \x1b[0m", r, g, b);


        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut usr1).expect("failed to read line put something vaild in");

        let usr_cc: u8 = match usr1.trim().parse() {
            Ok(numm) => numm,
            Err(_) => {
                println!("Invalid input.");
                return; 
            }
        };

        let name = format!{
            "
            \n
            ██╗   ██╗██╗   ██╗███╗   ██╗███╗   ██╗██████╗  ██████╗ ██████╗ ██╗██╗  ██╗
            ██║   ██║╚██╗ ██╔╝████╗  ██║████╗  ██║██╔══██╗██╔═══██╗██╔══██╗██║╚██╗██╔╝
            ██║   ██║ ╚████╔╝ ██╔██╗ ██║██╔██╗ ██║██║  ██║██║   ██║██████╔╝██║ ╚███╔╝                ██   ████╗ 
            ╚██╗ ██╔╝  ╚██╔╝  ██║╚██╗██║██║╚██╗██║██║  ██║██║   ██║██╔══██╗██║ ██╔██╗                     ██ ██
            ╚████╔╝    ██║   ██║ ╚████║██║ ╚████║██████╔╝╚██████╔╝██║  ██║██║██╔╝ ██╗                    ██ ██
            ╚═══╝     ╚═╝   ╚═╝  ╚═══╝╚═╝  ╚═══╝╚═════╝  ╚═════╝ ╚═╝  ╚═╝╚═╝╚═╝    ██╗             ██   ████╗ 
            ██████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████
            "
        };

        let not_a_name = format!{
            "
            \n
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            "
        };

        match usr_cc{
            1 =>{
                for _ in 0..400 {
                    write!(stdout, "\x1b[38;2;{};{};{}m", rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>()).unwrap();
                    stdout.flush().unwrap();
                    thread::sleep(Duration::from_millis(50));
            
                    write!(stdout, "\x1b[2J").unwrap();
                    write!(stdout, "\x1b[1;1H").unwrap();
                    stdout.flush().unwrap();
                    
                    let r: u8 = rand::thread_rng().gen_range(0..=255);
                    let g: u8 = rand::thread_rng().gen_range(0..=255);
                    let b: u8 = rand::thread_rng().gen_range(0..=255);
                    print!("\x1b[38;2;{};{};{}m{not_a_name}\x1b[0m\n", r, g, b);
                    thread::sleep(Duration::from_millis(number.into()));
                
                }
            },
            2 =>{
                for _ in 0.. 400 {
                    write!(stdout, "\x1b[38;2;{};{};{}m", rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>()).unwrap();
                    stdout.flush().unwrap();
                    thread::sleep(Duration::from_millis(50));
            
                    write!(stdout, "\x1b[2J").unwrap();
                    write!(stdout, "\x1b[1;1H").unwrap();
                    stdout.flush().unwrap();
                    
                    let r: u8 = rand::thread_rng().gen_range(0..=255);
                    let g: u8 = rand::thread_rng().gen_range(0..=255);
                    let b: u8 = rand::thread_rng().gen_range(0..=255);
                    print!("\x1b[38;2;{};{};{}m{name}\x1b[0m\n", r, g, b);
                    thread::sleep(Duration::from_millis(number.into()));
                }
            },
            3 =>{
                for _ in 0..400 {
                    let random_data: String = (0..10000).map(|_| {
                        let choice = rng.gen_range(0..3);
                        match choice {
                            0 => rng.gen_range(b'0'..=b'9') as char,
                            1 => rng.gen_range(b'a'..=b'z') as char,
                            2 => rng.gen_range(b'A'..=b'Z') as char, 
                            _ => char::from_u32(rng.gen_range(33..=126)).unwrap(), 
                        }
                    }).collect();
            
                    write!(stdout, "\x1b[38;2;{};{};{}m", rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>()).unwrap();
                    stdout.flush().unwrap();
                    thread::sleep(Duration::from_millis(50));
            
                    write!(stdout, "\x1b[2J").unwrap();
                    write!(stdout, "\x1b[1;1H").unwrap();
                    stdout.flush().unwrap();
                    
                    let r: u8 = rand::thread_rng().gen_range(0..=255);
                    let g: u8 = rand::thread_rng().gen_range(0..=255);
                    let b: u8 = rand::thread_rng().gen_range(0..=255);

                    print!("\x1b[38;2;{};{};{}m{random_data}\x1b[0m", r, g, b);
                    thread::sleep(Duration::from_millis(number.into()));

                }
            }
            _ => println!("Choose 1 or 2")
        }
    }

}

fn manual_m(){
    loop{
        let mut rng = rand::thread_rng();
        let mut usr = String::new();
        let mut usr1 = String::new();
        let mut stdout = io::stdout();
        write!(stdout, "\x1b[2J").unwrap(); 
        write!(stdout, "\x1b[1;1H").unwrap();
        stdout.flush().unwrap();

        let mut stdout = io::stdout();

        let banner = format!{
            "\n
            ********** **                    *******     ********  ******         *******                   **     
            /////**/// /**                   /**////**   **//////**/*////**       /**////**                 /**     
                /**    /**       *****       /**   /**  **      // /*   /**       /**   /**  **   **  ******/**     
                /**    /******  **///**      /*******  /**         /******        /*******  /**  /** **//// /****** 
                /**    /**///**/*******      /**///**  /**    *****/*//// **      /**///**  /**  /**//***** /**///**
                /**    /**  /**/**////       /**  //** //**  ////**/*    /**      /**  //** /**  /** /////**/**  /**
                /**    /**  /**//******      /**   //** //******** /*******       /**   //**//****** ****** /**  /**
                //     //   //  //////       //     //   ////////  ///////        //     //  ////// //////  //   // 
            
            \n\n
                sdSS_SSSSSSbs   .S_sSSs     .S   .S_sSSs     .S  sdSS_SSSSSSbs   .S S.   
        YSSS~S%SSSSSP  .SS~YS%%b   .SS  .SS~YS%%b   .SS  YSSS~S%SSSSSP  .SS SS.  
            S%S       S%S   `S%b  S%S  S%S   `S%b  S%S       S%S       S%S S%S  
            S%S       S%S    S%S  S%S  S%S    S%S  S%S       S%S       S%S S%S  
            S&S       S%S    d*S  S&S  S%S    S&S  S&S       S&S       S%S S%S  
            S&S       S&S   .S*S  S&S  S&S    S&S  S&S       S&S        SS SS   
            S&S       S&S_sdSSS   S&S  S&S    S&S  S&S       S&S         S S    
            S&S       S&S~YSY%b   S&S  S&S    S&S  S&S       S&S         SSS    
            S*S       S*S   `S%b  S*S  S*S    S*S  S*S       S*S         S*S    
            S*S       S*S    S%S  S*S  S*S    S*S  S*S       S*S         S*S    
            S*S       S*S    S&S  S*S  S*S    S*S  S*S       S*S         S*S    
            S*S       S*S    SSS  S*S  S*S    SSS  S*S       S*S         S*S    
            SP        SP          SP   SP          SP        SP          SP     
            Y         Y           Y    Y           Y         Y           Y              By: Vynndorix     
                                                                            
            "
        };


        for _ in 0..120{
            write!(stdout, "\x1b[38;2;{};{};{}m", rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>()).unwrap();
            stdout.flush().unwrap();
            thread::sleep(Duration::from_millis(50));
            
            write!(stdout, "\x1b[2J").unwrap();
            write!(stdout, "\x1b[1;1H").unwrap();
            stdout.flush().unwrap();
                    
            let r: u8 = rand::thread_rng().gen_range(0..=255);
            let g: u8 = rand::thread_rng().gen_range(0..=255);
            let b: u8 = rand::thread_rng().gen_range(0..=255);
            print!("\x1b[38;2;{};{};{}m{banner}\x1b[0m\n", r, g, b);
            
        }
        let mut stdout = io::stdout();
        write!(stdout, "\x1b[2J").unwrap(); 
        write!(stdout, "\x1b[1;1H").unwrap(); 
        stdout.flush().unwrap();

        write!(stdout, "\x1b[38;2;{};{};{}m", rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>()).unwrap();
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(50));

                    
        let r: u8 = rand::thread_rng().gen_range(0..=255);
        let g: u8 = rand::thread_rng().gen_range(0..=255);
        let b: u8 = rand::thread_rng().gen_range(0..=255);
        print!("\x1b[38;2;{};{};{}m|\x1b[0m\n", r, g, b);

        print!("\x1b[38;2;{};{};{}m\t\t\t\t\t\t\t\t\t*Note*  All Of These Might Be Chaotic Because It Needs To Clear The Screen Lol  *Note*\n\n\x1b[0m", r, g, b);
        print!("\x1b[38;2;{};{};{}m\t\t\t\t\t\t\t\t\tPress Enter to Change The Colors\n\n\x1b[0m", r, g, b);

        write!(stdout, "\x1b[38;2;{};{};{}m", rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>()).unwrap();
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(50));  
                
        let r: u8 = rand::thread_rng().gen_range(0..=255);
        let g: u8 = rand::thread_rng().gen_range(0..=255);
        let b: u8 = rand::thread_rng().gen_range(0..=255);
        
        print!("\x1b[38;2;{};{};{}m\n\n\n\t\t\t\t\t\t\t1: Longest Delay\n\n\t\t\t\t\t\t\t2: Semi Long Delay\n\n\t\t\t\t\t\t\t3: Zero Delay * Might Be The Chillest *\n\n\n\n \x1b[0m", r, g, b);
        
        write!(stdout, "\x1b[38;2;{};{};{}m", rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>()).unwrap();
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(50));    

        let r: u8 = rand::thread_rng().gen_range(0..=255);
        let g: u8 = rand::thread_rng().gen_range(0..=255);
        let b: u8 = rand::thread_rng().gen_range(0..=255);


        print!("\x1b[38;2;{};{};{}mYour Input Goes Here: \x1b[0m", r, g, b);

        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut usr).expect("failed to read line put something vaild in");

        let usr_c: u8 = match usr.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input.");
                return; 
            }
        };
        let mut number:u8 = 0;
        match usr_c{
            1 => number = 100,
            2 => number = 30,
            3 => number = 0,
            _ => println!("Put a vaild number in from 1-3"), 
            
        };

        let mut stdout = io::stdout();
        write!(stdout, "\x1b[2J").unwrap(); 
        write!(stdout, "\x1b[1;1H").unwrap(); 
        stdout.flush().unwrap();

        write!(stdout, "\x1b[38;2;{};{};{}m", rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>()).unwrap();
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(50));    

        let r: u8 = rand::thread_rng().gen_range(0..=255);
        let g: u8 = rand::thread_rng().gen_range(0..=255);
        let b: u8 = rand::thread_rng().gen_range(0..=255);

        println!("\x1b[38;2;{};{};{}m\n\n\n\t\t\t\t\t\t\t1: Just Amazing RGB\n\n\t\t\t\t\t\t\t2: If You Want Just Amazing RGB With A Name :D\n\n\t\t\t\t\t\t\t3: If You Want Pure Randomness But With RGB\n\n\x1b[0m", r, g, b);
        write!(stdout, "\x1b[38;2;{};{};{}m", rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>()).unwrap();
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(50));    

        let r: u8 = rand::thread_rng().gen_range(0..=255);
        let g: u8 = rand::thread_rng().gen_range(0..=255);
        let b: u8 = rand::thread_rng().gen_range(0..=255);

        print!("\x1b[38;2;{};{};{}mYour Input Goes Here: \x1b[0m", r, g, b);


        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut usr1).expect("failed to read line put something vaild in");

        let usr_cc: u8 = match usr1.trim().parse() {
            Ok(numm) => numm,
            Err(_) => {
                println!("Invalid input.");
                return; 
            }
        };

        let name = format!{
            "
            \n
            ██╗   ██╗██╗   ██╗███╗   ██╗███╗   ██╗██████╗  ██████╗ ██████╗ ██╗██╗  ██╗
            ██║   ██║╚██╗ ██╔╝████╗  ██║████╗  ██║██╔══██╗██╔═══██╗██╔══██╗██║╚██╗██╔╝
            ██║   ██║ ╚████╔╝ ██╔██╗ ██║██╔██╗ ██║██║  ██║██║   ██║██████╔╝██║ ╚███╔╝                ██   ████╗ 
            ╚██╗ ██╔╝  ╚██╔╝  ██║╚██╗██║██║╚██╗██║██║  ██║██║   ██║██╔══██╗██║ ██╔██╗                     ██ ██
            ╚████╔╝    ██║   ██║ ╚████║██║ ╚████║██████╔╝╚██████╔╝██║  ██║██║██╔╝ ██╗                    ██ ██
            ╚═══╝     ╚═╝   ╚═╝  ╚═══╝╚═╝  ╚═══╝╚═════╝  ╚═════╝ ╚═╝  ╚═╝╚═╝╚═╝    ██╗             ██   ████╗ 
            ██████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████
            "
        };

        let not_a_name = format!{
            "
            \n
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n
            !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            "
        };

        match usr_cc{
            1 =>{
                for _ in 0..400 {
                    write!(stdout, "\x1b[38;2;{};{};{}m", rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>()).unwrap();
                    stdout.flush().unwrap();
                    thread::sleep(Duration::from_millis(50));
            
                    write!(stdout, "\x1b[2J").unwrap();
                    write!(stdout, "\x1b[1;1H").unwrap();
                    stdout.flush().unwrap();
                    
                    let r: u8 = rand::thread_rng().gen_range(0..=255);
                    let g: u8 = rand::thread_rng().gen_range(0..=255);
                    let b: u8 = rand::thread_rng().gen_range(0..=255);
                    print!("\x1b[38;2;{};{};{}m{not_a_name}\x1b[0m\n", r, g, b);
                    thread::sleep(Duration::from_millis(number.into()));
                    let mut input = String::new();
                    let start_time = Instant::now();
                    loop {
                        let elapsed = start_time.elapsed();
                        if elapsed > Duration::from_millis(100) {
                            break; 
                        }
                        if io::stdin().read_line(&mut input).is_ok() {
                            break; 
                        }
                    }
                    if !input.trim().is_empty() {
                        break; 
                    }
                
                }
            },
            2 =>{
                for _ in 0.. 400 {
                    write!(stdout, "\x1b[38;2;{};{};{}m", rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>()).unwrap();
                    stdout.flush().unwrap();
                    thread::sleep(Duration::from_millis(50));
            
                    write!(stdout, "\x1b[2J").unwrap();
                    write!(stdout, "\x1b[1;1H").unwrap();
                    stdout.flush().unwrap();
                    
                    let r: u8 = rand::thread_rng().gen_range(0..=255);
                    let g: u8 = rand::thread_rng().gen_range(0..=255);
                    let b: u8 = rand::thread_rng().gen_range(0..=255);
                    print!("\x1b[38;2;{};{};{}m{name}\x1b[0m\n", r, g, b);
                    thread::sleep(Duration::from_millis(number.into()));
                    let mut input = String::new();
                    let start_time = Instant::now();
                    loop {
                        let elapsed = start_time.elapsed();
                        if elapsed > Duration::from_millis(100) {
                            break; 
                        }
                        if io::stdin().read_line(&mut input).is_ok() {
                            break; 
                        }
                    }
                    if !input.trim().is_empty() {
                        break; 
                    }
                }
            },
            3 =>{
                for _ in 0..400 {
                    let random_data: String = (0..10000).map(|_| {
                        let choice = rng.gen_range(0..3);
                        match choice {
                            0 => rng.gen_range(b'0'..=b'9') as char,
                            1 => rng.gen_range(b'a'..=b'z') as char,
                            2 => rng.gen_range(b'A'..=b'Z') as char, 
                            _ => char::from_u32(rng.gen_range(33..=126)).unwrap(), 
                        }
                    }).collect();
            
                    write!(stdout, "\x1b[38;2;{};{};{}m", rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>()).unwrap();
                    stdout.flush().unwrap();
                    thread::sleep(Duration::from_millis(50));
            
                    write!(stdout, "\x1b[2J").unwrap();
                    write!(stdout, "\x1b[1;1H").unwrap();
                    stdout.flush().unwrap();
                    
                    let r: u8 = rand::thread_rng().gen_range(0..=255);
                    let g: u8 = rand::thread_rng().gen_range(0..=255);
                    let b: u8 = rand::thread_rng().gen_range(0..=255);

                    print!("\x1b[38;2;{};{};{}m{random_data}\x1b[0m", r, g, b);
                    thread::sleep(Duration::from_millis(number.into()));
                    let mut input = String::new();
                    let start_time = Instant::now();
                    loop {
                        let elapsed = start_time.elapsed();
                        if elapsed > Duration::from_millis(100) {
                            break; 
                        }
                        if io::stdin().read_line(&mut input).is_ok() {
                            break; 
                        }
                    }
                    if !input.trim().is_empty() {
                        break; 
                    }
                }
            }
            _ => println!("Choose 1 or 2")
        }
    }
}


fn main() {
    let mut stdout = io::stdout();
    write!(stdout, "\x1b[2J").unwrap();
    write!(stdout, "\x1b[1;1H").unwrap();
    stdout.flush().unwrap();

    let mut stdout = io::stdout();

    write!(
        stdout,
        "\x1b[38;2;{};{};{}m",
        rand::random::<u8>(),
        rand::random::<u8>(),
        rand::random::<u8>()
    )
    .unwrap();
    stdout.flush().unwrap();
    thread::sleep(Duration::from_millis(50));

    let r: u8 = rand::thread_rng().gen_range(0..=255);
    let g: u8 = rand::thread_rng().gen_range(0..=255);
    let b: u8 = rand::thread_rng().gen_range(0..=255);

    print!(
        "\x1b[38;2;{};{};{}m\n\n\n\t\t\t\t\t\t\t1: Default\n\n\t\t\t\t\t\t\t2: Manual Color Changing Mode\n\n\n\n \x1b[0m",
        r, g, b
    );
    print!("\n\nYour Input here: ");
    io::stdout().flush().unwrap();

    let mut usr = String::new();
    io::stdin()
        .read_line(&mut usr)
        .expect("Failed to read line");

    let usr_c: u8 = match usr.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input.");
            return;
        }
    };

    match usr_c {
        1 => default(),
        2 => manual_m(),
        _ => println!("Put in a value that is from 1-2"),
    }

}
