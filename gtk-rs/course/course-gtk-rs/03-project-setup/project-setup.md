
# PROJECT SETUP

    - Let's begin by installing all necessary tools. First, follow the instructions on the GTK website in order to install GTK 4. Then install Rust with rustup.


## Create

    - Now, create a new project and move into the newly created folder by executing:

        $ cargo new my-gtk-app
    
        $ cd my-gtk-app


## Find GTK

    - Find out the GTK 4 version on your machine by running

        $ pkg-config --modversion gtk4

    - Use this information to add the gtk4 crate to your dependencies in Cargo.toml. At the time of this writing the newest version is 4.8.


## Add GTK4 to Project by Crate

    - Add to project the library gtk4

    1 - Add with cargo add

        $ cargo add gtk4 --rename gtk --features v4_8

    or

    2 - Add to file Cargo.toml - dependencies 
        
        [dependencies]
        gtk4 = "0.7.3"


## Run Project

    - cargo run

