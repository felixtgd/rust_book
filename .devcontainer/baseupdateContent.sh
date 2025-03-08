function preConfig() {
    sudo apt -y update
    sudo apt -y upgrade
    sudo apt-get install fzf -y
}


function postConfig(){

    # Zsh plugins for syntax highlighting and autosuggestions
    git clone https://github.com/zsh-users/zsh-syntax-highlighting.git ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-syntax-highlighting
    git clone https://github.com/zsh-users/zsh-autosuggestions ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-autosuggestions

    # Neovim text editor
    # curl -LO https://github.com/neovim/neovim/releases/latest/download/nvim-linux-x86_64.tar.gz
    # sudo rm -rf /opt/nvim
    # sudo tar -C /opt -xzf nvim-linux-x86_64.tar.gz
    # rm nvim-linux-x86_64.tar.gz
    # export PATH=$PATH:/opt/nvim-linux-x86_64/bin

    # fzf-tab for improved tab completion in Zsh
    git clone https://github.com/Aloxaf/fzf-tab ~/fzf-tab

    # LazyGit for terminal-based Git UI
    LAZYGIT_VERSION=$(curl -s "https://api.github.com/repos/jesseduffield/lazygit/releases/latest" | grep -Po '"tag_name": "v\K[^"]*')
    curl -Lo lazygit.tar.gz "https://github.com/jesseduffield/lazygit/releases/latest/download/lazygit_${LAZYGIT_VERSION}_Linux_x86_64.tar.gz"
    tar xf lazygit.tar.gz lazygit
    sudo install lazygit /usr/local/bin
    rm lazygit.tar.gz lazygit

}
