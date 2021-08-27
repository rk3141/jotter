cd ~
rm -rf .jotter

git clone https://github.com/rishit-khandelwal/jotter .jotter
cd .jotter
cargo build --release

cp target/release/jotter /tmp/jotter
rm -rf *
mv /tmp/jotter .
sudo ln -s jotter /usr/bin/jotter
