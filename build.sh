rm -r ./docs
dx bundle
mv ./target/dx/penguin-addition/release/web/public ./docs
cp ./docs/index.html ./docs/404.html