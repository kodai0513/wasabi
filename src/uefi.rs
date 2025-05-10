use crate::graphics::draw_font_fg;
use crate::graphics::Bitmap;
use crate::result::Result;
use core::fmt;
use core::mem::offset_of;
use core::mem::size_of;
use core::ptr::null_mut;

type EfiVoid = u8;
type EfiHandle = u64;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct EfiGuid {
    pub data0: u32,
    pub data1: u16,
    pub data2: u16,
    pub data3: [u8; 8],
}