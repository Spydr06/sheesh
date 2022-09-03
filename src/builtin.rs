use crate::{
    command::Command,
    environment::{
        Environment, 
        Identifier::Alias
    },
    shell::{
        self, 
        Error
    }
};

use std::slice::Iter;

use phf::phf_map;

type EvalFn = fn(&Command, &mut Iter<Command>, &mut Environment) -> Result<i32, Error>;

pub static BUILTINS: phf::Map<&'static str, EvalFn> = phf_map! {
    "exit"    => Command::eval_exit,
    "echo"    => Command::eval_echo,
    "alias"   => Command::eval_alias,
    "export"  => Command::eval_export,
    "require" => Command::eval_require,
    "cd"      => Command::eval_cd,
};

impl Command {
    fn eval_exit(&self, _commands: &mut Iter<Command>, env: &mut Environment) -> Result<i32, Error> {
        match self.args.len() {
            0 => Err(Error::EarlyExit(0)),
            1 => {
                let str = self.args.get(0).unwrap().eval(env)?;
                let arg = str.parse::<i32>();
                if let Ok(exit_code) = arg {
                    Err(Error::EarlyExit(exit_code))
                }
                else {
                    eprintln!("exit: {}: numeric argument required", str);
                    Err(Error::EarlyExit(2))
                }
            }
            _ => {
                eprintln!("exit: too many arguments");
                Ok(1)
            }
        }
    }

    fn eval_echo(&self, _commands: &mut Iter<Command>, env: &mut Environment) -> Result<i32, Error> {
        for (i, arg) in self.args.iter().enumerate() {
            print!("{}", arg.eval(env)?);
            if i != self.args.len() - 1 {
                print!(" ");
            }
        }

        println!("");

        Ok(0)
    }

    fn eval_alias(&self, _commands: &mut Iter<Command>, env: &mut Environment) -> Result<i32, Error> {
        if self.args.len() < 3 || &*self.args.get(1).unwrap().eval(env)? != "=" {
            eprintln!("alias: expect matching arguments: alias <name> = <value...>");
            Ok(2)
        }
        else { 
            let alias = Alias { substitute: Command::from_args(&self.args[2..], env)? };
            let name = self.args.get(0).unwrap().eval(env)?;
            env.add_ident(name, alias);

            Ok(0)
        }
    }

    fn eval_export(&self, _commands: &mut Iter<Command>, env: &mut Environment) -> Result<i32, Error> {
        if self.args.len() != 3 || &*self.args.get(1).unwrap().eval(env)? != "=" {
            eprintln!("export: expect matching arguments: export <name> = <value...>");
            Ok(2)
        }
        else {
            let value = self.args.get(2).unwrap().eval(env)?;
            let name = self.args.get(0).unwrap().eval(env)?;
            env.add_var(name, value);

            Ok(0)
        }
    }

    fn eval_require(&self, _commands: &mut Iter<Command>, env: &mut Environment) -> Result<i32, Error> {
        if self.args.len() != 1 {
            eprintln!("require: expect arguments matching: require <file.sh>");
            Ok(2)
        }
        else {
            let path = self.args.get(0).unwrap().eval(env)?;
            shell::run_script(path, env)
        }
    }

    fn eval_cd(&self, _commands: &mut Iter<Command>, env: &mut Environment) -> Result<i32, Error> {
        match self.args.len() {
            0 => {
                if let Some(path) = env.find_var(&String::from("HOME")) {
                    shell::set_directory(&path.value())
                }
                else {
                    eprintln!("cd: $HOME environment variable not set.");
                    Ok(2)
                }
            },
            1 => {
                let path = self.args.get(0).unwrap().eval(env)?;
                shell::set_directory(&path)
            }
            _ => {
                eprintln!("cd: too many arguments");
                Ok(2)
            }
        }
    }
}