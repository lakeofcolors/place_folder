use std::env;
use std::process;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::fs::File;
use std::fs::write;
use std::fs::read_to_string;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
extern crate dirs;

type Callback = fn(args: &[String], config: &mut ConfigManager) -> Result<(), &'static str>;
type ProjectPath = String;
type ProjectName = String;

struct Application{
    config: ConfigManager
}

struct EventHandler{
    funcs: HashMap<String, Callback>
}

pub fn help_message() {
    println!("\nUsage:  pf [COMMAND] args\n");
    println!("Commands:\n");
    println!("  add <name> <path>\tAdd project to list");
    println!("  rm <name> \tRemove project from list");
    println!("  ls \tDisplay project list");
    println!("  go \tGo to project dir\n");
}


impl EventHandler{
    fn add_event(&mut self, name: String, func: Callback) {
        self.funcs.insert(name, func);
    }

    fn on_event_call(&mut self, args: &[String], config: &mut ConfigManager){
        let name = match args.get(1) {
            Some(name) => name,
            None => {
                help_message();
                process::exit(1);
            },
        };
        match self.funcs.get(name){
            Some(func) => func(&args, config).unwrap_or_else(|err|{println!("Err: {}", err)}),
            None => {
                help_message();
                process::exit(1);
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigManager{
    goto_path: Option<String>,
    projects: HashMap<ProjectName, ProjectPath>
}

impl ConfigManager{
    pub fn build() -> ConfigManager{
        let config_path = match dirs::home_dir(){
            Some(i) => i.join(".pf.conf.json"),
            None => {
                panic!("Can not find home dir...");
            }
        };
        let path_exists: bool = Path::new(&config_path).exists();
        if !path_exists {
            let mut file = match File::create(&config_path) {
                Err(why) => panic!("couldn create config file: {}", why),
                Ok(f) => f,
            };
            let data = r#"{"goto_path": null, "projects": {}}"#;
            file.write_all(data.as_bytes());

            let config = ConfigManager{
                goto_path: None,
                projects: HashMap::new(),
            };
            return config
        }
        return ConfigManager::build_from_config(config_path)
    }

    fn build_from_config(config_path: PathBuf) -> ConfigManager{
        let mut data = String::new();
        let mut file = match File::open(config_path){
            Err(why) => panic!("couldn open config file: {}", why),
            Ok(f) => f,
        };
        file.read_to_string(&mut data);
        let config: ConfigManager = serde_json::from_str(&mut data).unwrap();
        config
    }

    pub fn update_config(&self){
        let serialized = serde_json::to_string(self).unwrap();
        let config_path = match dirs::home_dir(){
            Some(i) => i.join(".pf.conf.json"),
            None => {
                panic!("Can not find home dir...");
            }
        };
        let mut file = match File::options().write(true).truncate(true).open(config_path){
            Err(why) => panic!("couldn open config file: {}", why),
            Ok(f) => f,
        };
        file.write_all(serialized.as_bytes());
    }
}


impl Application{
    pub fn rm(args: &[String], config: &mut ConfigManager) -> Result<(), &'static str>{
        if args.len() < 3{
            return Err("not enough arguments");
        }
        let name = &args[2];
        config.projects.remove(name);
        config.update_config();
        println!("\x1b[41mRemove project: {} \x1b[0m", name);
        Ok(())
    }


    pub fn go(args: &[String], config: &mut ConfigManager) -> Result<(), &'static str>{
        if args.len() < 3{
            return Err("not enough arguments");
        }
        let name = &args[2];
        let data = match config.projects.get(name){
            Some(i) => i.clone(),
            None => String::from(dirs::home_dir().unwrap().to_string_lossy()),
        };

        config.goto_path = Some(data.clone());
        config.update_config();
        println!("\x1b[42mGo to dir: {} \x1b[0m", data);
        Ok(())
    }

    pub fn new(args: &[String], config: &mut ConfigManager) -> Result<(), &'static str>{
        if args.len() < 4 {
            return Err("not enough arguments");
        }
        let dir_path = {
            if args[3].clone() == "." {
                String::from(env::current_dir().unwrap().to_string_lossy())
            }else{
                args[3].clone()
            }
        };
        let name = args[2].clone();
        config.projects.insert(name, dir_path);
        config.update_config();
        println!("\x1b[42mAdd project: {} \x1b[0m", args[2].clone());
        Ok(())
    }

    pub fn list(args: &[String], config: &mut ConfigManager) -> Result<(), &'static str>{
        println!(
            "{0: <20}  {1: <20}",
            "PROJECT", "PATH"
        );
        for (name, path) in &config.projects{
            println!("{0: <20}  {1: <20}", name, path);
        }
        Ok(())
    }
}


fn run(args: &[String]) {
    let mut config = ConfigManager::build();
    let mut handler = EventHandler{funcs: HashMap::new()};
    handler.add_event("ls".to_string(), Application::list);
    handler.add_event("rm".to_string(), Application::rm);
    handler.add_event("add".to_string(), Application::new);
    handler.add_event("go".to_string(), Application::go);
    handler.on_event_call(&args, &mut config)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    run(&args);
}
