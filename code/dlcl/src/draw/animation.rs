use embedded_graphics::pixelcolor::Rgb888;

///
/// The single frame of an animation
///
#[derive(Clone, PartialEq)]
pub struct Frame {
    pixels: Vec<Rgb888>
}

impl Frame {
    pub fn new(pixels: Vec<Rgb888>) -> Self {
        let frame = Frame {
            pixels
        };
        frame
    }

    pub fn pixels(&self) -> &Vec<Rgb888> {
        &self.pixels
    }
}

///
/// An animation consisting of multiple frames
///
#[derive(Clone, PartialEq)]
pub struct Animation {
    frames: Vec<Frame>,
    frame_index: usize
}

impl Animation {

    pub fn from_frames(frames: Vec<Frame>) -> Self {
        let anim = Animation {
            frames,
            frame_index: 0,
        };
        anim
    }

    pub fn frame_index(&self) -> usize {
        self.frame_index
    }

    pub fn len(&self) -> usize {
        self.frames.len()
    }

    pub fn next_frame(&mut self) -> &Frame {
        let frame = &self.frames[self.frame_index];
        if self.frame_index < self.frames.len() - 1 {
            self.frame_index += 1;
        }
        frame
    }
}