# fraktalgen
A program to look at fraktals interaktively. Right now only the
Mandelbrotset can be generated

![alt text](https://github.com/AndreasMuehlmann/fraktalgen/blob/main/pictures/mandelbrotset-green.png)

![alt text](https://github.com/AndreasMuehlmann/fraktalgen/blob/main/pictures/top-mandelbrotset-purple.png)

Author: Andreas Mühlmann

GitHub repository: "https://github.com/AndreasMuehlmann/fraktalgen"

## Quickstart
Clone the git repository with
"git clone https://github.com/AndreasMuehlmann/fraktalgen.git".

if git is not installed download the
repository from the website via the zip-archive (click on the green button).

## Usage
Run the executable for your suited for your OS.

When your on Windows run: ".\target\release\fraktalgen.exe"
When your on Linux run: ".\target\release\fraktalgen"

You can then move around with "w", "a", "s" and "d".

To zoom in press "i". To zoom out press "o".

When zooming in you will notice that that the Mandelbrotset
actually isn't accuratly generated. This is so inital movement is quick.
To make the generation more accurate press "m". The tradeoff is that 
calculation will take longer so you might not be able to move as quick.
To make the generation less accurate press "l".

## Playing with the source code
The project is written in rust so you need to install the rust compiler.

To do that visit this website: "https://www.rust-lang.org/learn/get-started"

If that is done run "cargo run", to compile and run the program.
