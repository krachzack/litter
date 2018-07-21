# litter
Simple command line tool to take simple scenes and make them into complex scenes.
This is mainly of use for stress tests, less for artistic purposes. Having said
that, go forth and throw ridiculous amounts of geometry at your tool and see how
it handles it!

## Examples
This example takes the two scenes in `scene1.obj` and `scene2.obj` and combines them
into one. While doing that, it applies some filters to make a large scene:

    litter scene1.obj scene2.obj \
    # Ok merge into this file
        -o littered.obj \
    # Arrange a 10x10 grid centered at the old center point
        --filter grid,10x1x10 \
    # Then take the huge grid thing and center it at the origin,
    # except the Y axis, everything sits on top of Y = 0
        --filter align,c,+,c \
    # This is equivalent to the above, there are default values
        --filter

The output should appear in `littered.obj` and `littered.mtl`.

## Installation
You currently need to build it yourself, which requires a local rust installation.
Luckily for you, [rustup](https://rustup.rs/) makes this super easy. After you
are done installing rust, check out this repository and `cargo install` so you can
use it from your path:

    git clone https://github.com/krachzack/litter.git
    cd litter
    cargo install

    # This should print some verison information now,
    # you are good to go!
    litter -V

## Usage
`litter --help` might be of help.

    rosa% litter --help
    litter 0.1.0
    krachzack <hello@phstadler.com>
    Takes scenes and combines them into more complex scenes

    USAGE:
        litter [OPTIONS] <INPUT_FILE>... --output <OUTPUT_FILE>

    FLAGS:
        -h, --help       
                Prints help information

        -V, --version    
                Prints version information


    OPTIONS:
        -f, --filter <FILTER_SPECIFICATION>...    
                Adds transformations on top of the combined input scenes. For example, use --filter=grid,10x10x1 to
                duplicate the scene 100 times in a ten by ten grid. --filter=align,+,+,c will put all vertices in the
                positive X and Y range and puts half of the scene in negative Z and the rest in positive Z, default is
                c,+,c.
        -o, --output <OUTPUT_FILE>                
                Sets the output file path for the combined scene.


    ARGS:
        <INPUT_FILE>...    
                One or more paths to OBJ files

## Limitations
`litter` currently only supports the OBJ file format for both input and output.
