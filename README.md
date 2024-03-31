# Trying to recreate [tt](https://github.com/lemnos/tt) in rust

## Install
```
git clone https://github.com/crolbar/tt-rs 
cd tt-rs
cargo build --release
sudo cp target/release/tt-rs /usr/bin
cp -r conf ~/.config/tt-rs
```
words and quotes used have to be in `~/.config/tt-rs`

## Usage
### Arguments
`-q` - test contains quotes instead of words \
`-d` - test will restart if you make an error \
`-w 50` - specify the number of words \
`-t 30` - specify time for the timer in seconds

### Binds / Keys
`ESC`, `ALT + q`, `CTRL + c` - exit \
`ALT + s` - switch between normal and sroller mode \
`ALT + r` - restart the test with the same words \
`ALT + n`, `TAB` - restard the test with different words
