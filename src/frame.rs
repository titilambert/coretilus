/// A single frame of an animation, containing ASCII art and the number of ticks to display it.
///
/// The `ascii` field holds the ASCII art string, and `ticks` determines how many animation
/// ticks this frame should be shown. If `ticks` is zero, the default from the Animation is used.
///
/// # Examples
///
/// ```
/// use coretilus::frame::Frame;
/// let frame1 = Frame::new("Hello, world!");
/// assert_eq!(frame1.ticks(), 0);
/// assert_eq!(frame1.content(), "Hello, world!");
///
/// let frame2 = Frame::new_ticks("Hello, world!", 200);
/// assert_eq!(frame2.ticks(), 200);
/// ```
#[derive(Clone)]
pub struct Frame {
    ascii: &'static str,
    // Number of ticks to keep this frame showed
    // Zero means use default from Animation
    ticks: usize,
}

impl Frame {
    /// Creates a new `Frame` with the specified ASCII art and a default tick count of 0.
    pub fn new(ascii: &'static str) -> Self {
        Self { ascii, ticks: 0 }
    }

    /// Creates a new `Frame` with the specified ASCII art and the number of ticks that this frame should be displayed for.
    pub fn new_ticks(ascii: &'static str, ticks: usize) -> Self {
        Self { ascii, ticks }
    }

    /// Returns a reference to the ASCII art content of the frame.
    pub fn content(&self) -> &str {
        self.ascii
    }

    /// Returns the number of ticks that this frame should be displayed for.
    pub fn ticks(&self) -> usize {
        self.ticks
    }

    /// Splits the ASCII art into individual lines and returns them as a `Vec<String>`.
    pub fn get_lines(&self) -> Vec<String> {
        self.ascii.lines().map(|line| line.to_string()).collect()
    }

    /// Returns the number of lines (height) in the frame's ASCII art.
    pub fn get_height(&self) -> u32 {
        self.get_lines().len() as u32
    }

    /// Returns the width (in characters) in the frame's ASCII art.
    pub fn get_width(&self) -> u32 {
        self.ascii
            .lines()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0) as u32
    }
}
