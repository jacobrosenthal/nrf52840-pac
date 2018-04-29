#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ISOOUT {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct SIZER {
    bits: u16,
}
impl SIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `ZERO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZEROR {
    #[doc = "No zero-length data received, use value in SIZE"]
    NORMAL,
    #[doc = "Zero-length data received, ignore value in SIZE"]
    ZERODATA,
}
impl ZEROR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ZEROR::NORMAL => false,
            ZEROR::ZERODATA => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ZEROR {
        match value {
            false => ZEROR::NORMAL,
            true => ZEROR::ZERODATA,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == ZEROR::NORMAL
    }
    #[doc = "Checks if the value of the field is `ZERODATA`"]
    #[inline]
    pub fn is_zero_data(&self) -> bool {
        *self == ZEROR::ZERODATA
    }
}
#[doc = r" Proxy"]
pub struct _SIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _SIZEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ZERO`"]
pub enum ZEROW {
    #[doc = "No zero-length data received, use value in SIZE"]
    NORMAL,
    #[doc = "Zero-length data received, ignore value in SIZE"]
    ZERODATA,
}
impl ZEROW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ZEROW::NORMAL => false,
            ZEROW::ZERODATA => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ZEROW<'a> {
    w: &'a mut W,
}
impl<'a> _ZEROW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ZEROW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No zero-length data received, use value in SIZE"]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(ZEROW::NORMAL)
    }
    #[doc = "Zero-length data received, ignore value in SIZE"]
    #[inline]
    pub fn zero_data(self) -> &'a mut W {
        self.variant(ZEROW::ZERODATA)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:9 - Amount of bytes received last on this iso OUT data endpoint"]
    #[inline]
    pub fn size(&self) -> SIZER {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SIZER { bits }
    }
    #[doc = "Bit 16 - Zero-length data packet received"]
    #[inline]
    pub fn zero(&self) -> ZEROR {
        ZEROR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:9 - Amount of bytes received last on this iso OUT data endpoint"]
    #[inline]
    pub fn size(&mut self) -> _SIZEW {
        _SIZEW { w: self }
    }
    #[doc = "Bit 16 - Zero-length data packet received"]
    #[inline]
    pub fn zero(&mut self) -> _ZEROW {
        _ZEROW { w: self }
    }
}