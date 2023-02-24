# place-folder-cli

 A way to easily navigate to projects in the terminal.

 Quickly switch to the right project.
 

 ## Installation:

```sh
cargo build
sudo install target/debug/.place_folder /usr/local/bin
```

```sh
/etc/run.sh 
# Add function in .bashrc OR .zshrc


pf(){
    place_folder $@
    if [[ -z $1 || "$1" == "go" ]]; then
        if -x "$(command -v python3)"; then
            local path=$(python3 -c "import json;f=open('$HOME/.pf.conf.json');print(json.loads(f.read())['goto_path'])")
            cd $path
        else
            echo "python3 is not install"
        fi
    fi
    return
}

```

A configuration file will be created automatically in your home directory `~/.pf.conf.json`

## Usage:

```sh
pf ls
```


## Commands:

 command | description
|:---|:---|
ls   | show a list of projects
add  | add a project to the list
rm   | remove project from list
go   | go to project dir
