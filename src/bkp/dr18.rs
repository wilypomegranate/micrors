#[doc = "Reader of register DR18"]
pub type R = crate::R<u32, super::DR18>;
#[doc = "Writer for register DR18"]
pub type W = crate::W<u32, super::DR18>;
#[doc = "Register DR18 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR18 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D18`"]
pub type D18_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D18`"]
pub struct D18_W<'a> {
    w: &'a mut W,
}
impl<'a> D18_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d18(&self) -> D18_R {
        D18_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d18(&mut self) -> D18_W {
        D18_W { w: self }
    }
}
