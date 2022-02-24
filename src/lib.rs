pub mod interface;
pub mod prelude;
mod registers;

use interface::WriteFrame;

#[doc(inline)]
pub use registers::line_in::InVoldB;

use registers::line_in::LeftLineIn;
use registers::line_in::RightLineIn;

pub struct Wm8731<I>
where
    I: WriteFrame,
{
    interface: I,
    left_line_in: LeftLineIn,
    right_line_in: RightLineIn,
}

impl<I> Wm8731<I>
where
    I: WriteFrame,
{
    ///Instantiate a driver. This also reset the codec to guarantee a known state.
    pub fn new(interface: I) -> Self {
        let mut codec = Self {
            interface,
            left_line_in: Default::default(),
            right_line_in: Default::default(),
        };
        codec.reset();
        codec
    }

    /// Reset the codec. All configuration is lost.
    pub fn reset(&mut self) {
        todo!();
    }
}

impl<I> Wm8731<I>
where
    I: WriteFrame,
{
    pub fn set_left_line_in_vol(&mut self, volume: InVoldB) -> &mut Self {
        self.left_line_in.set_vol(volume);
        self.left_line_in.set_both(false);
        self.interface.write(self.left_line_in.to_frame());
        self
    }

    pub fn set_right_line_in_vol(&mut self, volume: InVoldB) -> &mut Self {
        self.right_line_in.set_vol(volume);
        self.right_line_in.set_both(false);
        self.interface.write(self.right_line_in.to_frame());
        self
    }

    pub fn set_both_line_in_vol(&mut self, volume: InVoldB) -> &mut Self {
        self.left_line_in.set_vol(volume);
        self.right_line_in.set_vol(volume);
        self.left_line_in.set_both(true);
        self.interface.write(self.left_line_in.to_frame());
        self
    }

    pub fn set_left_line_in_mute(&mut self, mute: bool) -> &mut Self {
        self.left_line_in.set_mute(mute);
        self.left_line_in.set_both(false);
        self.interface.write(self.left_line_in.to_frame());
        self
    }

    pub fn set_right_line_in_mute(&mut self, mute: bool) -> &mut Self {
        self.right_line_in.set_mute(mute);
        self.right_line_in.set_both(false);
        self.interface.write(self.right_line_in.to_frame());
        self
    }

    pub fn set_both_line_in_mute(&mut self, mute: bool) -> &mut Self {
        self.left_line_in.set_mute(mute);
        self.right_line_in.set_mute(mute);
        self.left_line_in.set_both(true);
        self.interface.write(self.left_line_in.to_frame());
        self
    }
}
