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
