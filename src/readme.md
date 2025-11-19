
Membranes --> apply rules --> score by constraint satisfaction --> select top membranes --> repeat

[re-install rustup cargo/rustc]
sudo apt remove rustc
sudo apt remove cargo
sudo apt autoremove

rm -rf $HOME/.cargo
rm -rf $HOME/.rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

[Add extra component]
rustup component add rust-src
rustup component add clippy
rustup component add rustfmt
