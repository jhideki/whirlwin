# Whilwin

A simple and lightweight alternative to the Windows Alt+Tab switcher, built in Rust.

**Features:**

* Allows user to quickly switch between windows using vim-like keybinds
* Lightweight and resource-efficient.


**Installation**

1. Download the latest release from the Releases tab.
2. Extract the archive and run the executable file.

**Installation**
   
## Usage
The leader key allows the user to enter hotkey mode providing access to other keybinds.
- **Key:** Shift + Enter

Once the leader key has been pressed, the following keybinds can be accessed:
- **Key:** H <br>
  Switch focus to the window to the left.
- **Key:** L <br>
  Switch focus to the window to the right.
- **Key:** J <br>
  Switch focus to the window above.
- **Key:** K <br>
  Switch focus to the window below.
- **Key:** N <br>
  Switch focus to the next window.
- **Key:** D <br>
  Close the current window.
- **Key:** P <br>
 Switch focus to the previous window.
- **Key:** ESC <br>
  Exits hotkey mode and unregisters all active keybinds.

### Additional Notes:

- Keybinds are registered using the `RegisterHotKey` function.
- Keybinds can be unregistered using the `UnregisterHotKey` function.
- The `handle_hotkey` function handles incoming hotkey events.
- The `WindowManagerMessage` enum is used to communicate between the hotkey handler and the window manager.

**Development**

1. Ensure you have Rust and Cargo installed (https://www.rust-lang.org/tools/install).
2. Clone the repository:

```bash
git clone https://github.com/jhideki/whirlwin.git
