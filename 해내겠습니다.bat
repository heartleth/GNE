@echo off
cargo run --quiet %* > NUL
node l.js
del l.js