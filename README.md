![alt text](https://github.com/amixtum/dangerous-deliveries/blob/couch/dd.png?raw=true)

<h1>Dangerous Deliveries</h1>

It is your first day on the job as a delivery skater in the Grind Zone. Grind, skate, and reshape the world around you to deliver all the packages, no matter what.

<h3>How to play:</h3>

If you have Rust installed already just type:

    cargo run

Otherwise install Rust first

<h4>Controls</h4>

* Display/Hide Menu = Esc

* Apply Cellular Automata = G

* Look = Semicolon (Press a direction key afterwards to look in that direction) It's a good idea to look at what you're standing on by pressing Semicolon, then Period or Tab

* Move Left = A or H

* Move Right = D or L 

* Move Up = W or K

* Move Down = S or J 

* Move Northwest = Q or Y

* Move Northeast = E or U

* Move Southwest = Z or B

* Move Southeast = C or N 

* Wait = Period or Tab

* Restart = Enter

* Don't fall over (or into a bottomless pit of spikes)! 

* Tip: You can press G to modify the level while you are playing.

* Movement is calculated by adding an instantaneous velocity to your speed when you press a key.

<h3>Configuring</h3>

The config directory contains all data files necessary to compute the game

If you would like to add an [lsystem](https://en.wikipedia.org/wiki/L-system), or use a different one, create a new file in the config directory, adhering to the format described in lsystem_example.txt, and prefix it with `small_` or `medium_` to make it visible to the level selector for 40x20 or 80x40 maps respectively 

e.g. `small_my_lsystem.txt` will be visible when selecting small maps and viewable as `my_lsystem.txt`.

For other map sizes, edit `window.txt` and change `game_width` and `game_height` to a reasonable number. WARNING: Large maps will be slow.

To use an lsystem without the level selector, change `lsystem` in `window.txt` to the name of your lsystem file in the `config` directory.
