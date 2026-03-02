use std::io::{stdout, Write};

/// Clear the terminal screen and move cursor to top left.
/// Works on ANSI-compatible consoles.
pub fn clear_screen() {
    // CSI 2J clears the screen; CSI H moves the cursor to home position.
    print!("\x1B[2J\x1B[1;1H");
    let _ = stdout().flush();
}
