pub const COMMAND_LIST: &str = "--COMMAND LIST--
Many commands can be combined; e.g., :qp prints the current package list and exits the program.
:q,:quit                Exit pacbrow
:p,:print               Print current package list
:c,:commands            Display list of commands
:h,:help                Open help page
-------
";

pub const HELP_TEXT: &str = "--PACBROW HELP--
This help screen can be exited at any time by pressing the <Esc> key. pacbrow can be exited by typing \":q\" then pressing <Enter>.

pacbrow is a tool that allows you to browse the pacman/AUR packages you have installed on your device.

pacbrow is controlled entirely through the keyboard using Vim-inspired controls.


--MODES--
You navigate the pacbrow interface by switching between different modes. You can see the mode you're currently in by looking at the box labelled \"Mode\" in the bottom-right of the interface.

NORMAL MODE: This is, generally speaking, the mode which you use to switch between modes. You can also scroll through the current list of packages this way.

COMMAND MODE: You enter command mode when you need to give pacbrow instructions, such as displaying this help page or exiting the program.

SEARCH MODE: You enter search mode when you want to filter the list of packages or search for a specific package.

INFO MODE: You enter info mode when you want to read more details about the currently-selected package.

DISPLAY MODE: You enter display mode when you are viewing a non-package-related display message, such as this help page.
-------


--CONTROLS--
NORMAL MODE
:                       Enter command mode
s                       Enter search mode
r                       Enter search mode, clearing whatever was previously typed
k,<Up>                  Scroll up the list of packages
j,<Down>                Scroll down the list of packages
l,i,<Right>,<Enter>     Enter info mode for the currently selected package

COMMAND MODE
The list of commands can be found in the \"--COMMANDS--\" section below.
<Esc>                   Cancel current command and return to normal mode
<Enter>                 Submit command
<Left>                  Move cursor left
<Right>                 Move cursor right

SEARCH MODE
:                       Enter command mode (can be disabled in configuration)
<Left>                  Move cursor left
<Right>                 Move cursor right
<Esc>,<Enter>,<Down>    Return to normal mode

INFO MODE
:                       Enter command mode
s                       Enter search mode
r                       Enter search mode, clearing whatever was previously typed
h,n,<Esc>,<Left>        Exit info mode, returning to normal mode and the package list
k,<Up>                  Scroll up this package's information
j,<Down>                Scroll down this package's information

DISPLAY MODE
<Esc>                   Enter normal mode, closing the display
:                       Enter command mode, closing the display
k,<Up>                  Scroll up the displayed text
j,<Down>                Scroll down the displayed text
-------


--COMMANDS--
Many commands can be combined; e.g., :qp prints the current package list and exits the program.
:q,:quit                Exit pacbrow
:p,:print               Print current package list
:c,:commands            Display list of commands
:h,:help                Open help page
-------


--CONFIGURATION--
Your configuration file is located in ~/.config/pacbrow/config.toml. You may edit these values as much as you wish- any missing or malformed fields will simply be overwritten by the default value, stored in ~/.config/pacbrow/default-config.toml. You can't edit the default config, but it's a good reference for your own config, especially if you forget any values you've deleted.

[colours]
Valid colour options are: Black, Red, Green, Yellow, Blue, Magenta, Cyan, Gray, DarkGray, LightRed, LightGreen, LightYellow, LightBlue, LightMagenta, LightCyan, & White.
Please make sure that you've enclosed your chosen colour in double quotes- reference ~/.config/pacbrow/default-config.toml for an example.
normal = the colour associated with normal mode in the UI.
info =  the colour associated with info mode in the UI.
search =  the colour associated with search mode in the UI.
command =  the colour associated with command mode in the UI.
display =  the colour associated with display mode in the UI.
text =  the default text colour.

[operation]
starting_mode = valid options: \"normal\", \"command\", \"search\", or \"info\". Denotes the mode into which pacbrow boots.
allow_colon_in_search = valid options: true or false. If true, then you will be able to type ':' whilst in search mode, meaning that you can't jump directly to command mode from search mode.
-------
";
