# Rustlings
This Repository contains all the examples from the Rust Book. This is going to be a REPO for me to store my Rust documentation and the programs I am creating in between. 

I am following Let's get Rusty Channel on youtube that will take us through the RUST Documentation and coding at the same time. 

## YouTube links to Learn: 
    - https://www.youtube.com/watch?v=BpPEoZW5IiY
    - https://www.youtube.com/watch?v=un6ZyFkqFKo
    - https://www.youtube.com/playlist?list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8   (Lets' get RUSTY)
    - https://www.youtube.com/watch?v=-lYeJeQ11OI ( Easy Rust, this is what I am following)
    - Crust Of Rust [https://www.youtube.com/playlist?list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa](https://www.youtube.com/playlist?list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa)  - Must watch - by Jon Gjengset



## RUST INSTALLATION
### Linux: Simple just on single line on LINUX 
```
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
source "$HOME/.cargo/env"
rustc -V  
cargo -V
```
### Windows: Its quite complex, lets leave it. 


## Important commands to remember
- cargo new package  -> Create new project (--lib for library)
- cargo test (npm test)
- cargo run (npm run index.js)
- cargo publish
- cargo bench
- cargo build   (--release for release builds
- cargo doc

## Important Cargo extensions
- cargo install just -> install a better makefile tool
- cargo install cargo-edit -> Add commands to the cargo exe
- cargo install cargo-workspaces 
- cargo install cargo-expand
- cargo install cargo-watch -> Used like nodemon


## VS Code Extension to install for RUST
- Rust-analyzer (language server)
- cargo
- linter
- codelldb
- BetterTOML 
- Crates
- Error Lens

## Rust PlayGround
- https://play.rust-lang.org

--------------------------------------------------------------------

Chapter 1: [Video_1_Guessing_Game](Video_1_Guessing_game/src/main.rs)

--------------------------------------------------------------------
## Interesting Blogs: 
- https://rauljordan.com/rust-concepts-i-wish-i-learned-earlier/ 
- RUst Module System Explained [RUST_MODULE_SYSTEM](https://www.sheshbabu.com/posts/rust-module-system/)
- Rust - Simplifying the Toughest parts [https://www.youtube.com/watch?v=TJTDTyNdJdY](https://www.youtube.com/watch?v=TJTDTyNdJdY)


--------------------------------------------------------------
# WEB FRAMEWORKS
--------------------------------
## AXUM
- [https://www.youtube.com/playlist?list=PLrmY5pVcnuE-_CP7XZ_44HN-mDrLQV4nS](https://www.youtube.com/playlist?list=PLrmY5pVcnuE-_CP7XZ_44HN-mDrLQV4nS) 
- cargo watch -x run -> for continuously running the project.   
- cargo doc --open -> it will open a browser and show the offline documentation of all the crates that the project is using. 