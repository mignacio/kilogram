
```
 ██╗  ██╗██╗██╗      ██████╗  ██████╗ ██████╗  █████╗ ███╗   ███╗
 ██║ ██╔╝██║██║     ██╔═══██╗██╔════╝ ██╔══██╗██╔══██╗████╗ ████║
 █████╔╝ ██║██║     ██║   ██║██║  ███╗██████╔╝███████║██╔████╔██║
 ██╔═██╗ ██║██║     ██║   ██║██║   ██║██╔══██╗██╔══██║██║╚██╔╝██║
 ██║  ██╗██║███████╗╚██████╔╝╚██████╔╝██║  ██║██║  ██║██║ ╚═╝ ██║
 ╚═╝  ╚═╝╚═╝╚══════╝ ╚═════╝  ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝     ╚═╝
```                                                             

If you're lazy or suck at doing math in your head. This is a tool for you.

Given a weight in kilograms it will show you how to properly load the plates on the bar. It will do it with an ASCII art drawing of the bar with the correct plates loaded on it. I used the [IWF reference](https://iwf.sport/weightlifting_/equipment/) for the color and weights of the equipment.

Example: 

```
./kilogram 143 m
      |¦||¦|                  |¦||¦|
    |||¦||¦|                  |¦||¦|||
   ||||¦||¦|                  |¦||¦||||
==Ò||||¦||¦|=————————————————=|¦||¦||||Ò==
   ||||¦||¦|                  |¦||¦||||
    |||¦||¦|                  |¦||¦|||
      |¦||¦|                  |¦||¦|
```
(The actual drawing is colorized)

TODO:

  - Add color indicators for the bars.

  - Option to add o remove collars (the program asumes the bar will always have a 2.5kg collar loaded on either side).

  - Unit testing (so I can learn how unit testing works on Rust).

  - Github actions for release and testing.

  ~ Improve the help or usage message with examples. ~

  ~ Option to use different bars (the program asumes the 20kg bar for male competitions). ~

  ~ Add ASCII art drawings of the collars (now collars are loaded but not displayed in the drawing). ~


(Made with Rust because I wanted to learn)