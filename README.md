```
Keyboard Controls:
Left Analog Stick:
Up: W
Down: S
Left: A
Right: D
L3 / Click: X

Right Analog Stick:
Up: T
Down: G
Left: F
Right: H
R3 / Click : B

Left Bumper (L1): E
Left Analog (L2): Q

Right Bumper (R1): R
Right Analog (L2): Y

Digital Pad:
Up: Up Arrow
Down: Down Arrow
Left: Left Arrow
Right: Right Arrow

Buttons:
A Button: U
B Button: I
C Button: J
D Button: K
Start: 5
Select: 6
```

## Quick Start:
```
Required Functions:
init() - Called once when initializing the game

update() - Called once every frame, before draw.

draw() - Called once every frame, after update.

```

TODO (In order of priority):
1. Networking Functionality (WASM):
    1. Test if input for prev and "was pressed" is actually correct handling is actually correct
    1. Check how increasing/decreasing heap mem works (will need another test suite)
1. Example games/wasm folder?
    1. Helper script / build script to automatically place into correct directory
1. Input handling
    1. Do "Emulated gamepad" for keyboard input, include analogs etc
    1. Gamepad support
    1. Support multiple local players
    1. Use SoAs instead of AoS for better perf?
1. Finish "Graphcis" outlined functions
    1. Add "Draw Square" filled?
    1. Add "Draw Circle"
    1. Add "Draw Circle" filled?
    1. Add Write Text function (need a font??)
    1. How to handle lines/rects drawn out of bounds?
1. Add deterministic random number generation function
1. Build showcase projects
1. Start adding "Sprites" and Sprite Drawing
1. Set the user count accessable as a global somewhere within lua
1. Figure out how to save/load/rollback lua state?
    1. See the rlua comment about globals() vs _env
    1. https://github.com/amethyst/rlua/issues/247
1. Lua Refactoring
    1. Remove the inner arcs, use input/graphics context instead?

Research/Thinking Tasks:
1. Investigate how to play sounds
    1. Support wave forms: 
        1. Sin
        1. Sharktooth
        1. Sawtooth
        1. Square
        1. Pulse
        1. Triangle
        1. Double Triangle
        1. Noise
    1. Samples? .wav or .mp3?
1. Mouse/Cursor API? for UI, stick emulation etc? How to handle networking here?
1. Brainstorm "UserApi" for stuff like player names, avatar, meta-data outside the game
1. Cloud "Save/Load" Api - for game saves? Pulling session/game data?
1. Full screen shaders: Bloom, scanlines, etc



How to build WASM Projects:
cargo build --target wasm32-unknown-unknown

If WGPU errors occur, set WGPU_BACKEND=gl via

> export WGPU_BACKEND=gl

Useful Links:
Circle Drawing Algorithm
http://rosettacode.org/wiki/Bitmap/Midpoint_circle_algorithm

Showcase Projects TODO:
Pong 1p - Simple showcase
Pong 2p - Showcase networking multiplayer
Controller Debug - Showcase all controls for local player
Blasters - Twin stick shooter, Showcase two analog stick usage
