# litter
Simple command line tool to take simple scenes and make them into complex scenes.
This is mainly of use for stress tests, less for artistic purposes. Having said
that, go forth and throw ridiculous amounts of geometry at your tool and see how
it handles it!

## Examples
This example takes the two scenes in `scene1.obj` and `scene2.obj` and combines them
into one. After that, it will arrange 100 copies on a ten by ten grid. The output
of this transformation is then again arranged in a grid of four objects, that is,
two by two:

    litter scene1.obj scene2.obj \
        -o littered.obj \
        --filter grid,10x10 \
        --filter grid,4

The output should appear in `littered.obj` and `littered.mtl`.

## Limitations
`litter` currently only supports the OBJ file format for both input and output.
