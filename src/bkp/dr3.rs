#[doc = "Reader of register DR3"]
pub type R = crate::R<u32, super::DR3>;
#[doc = "Writer for register DR3"]
pub type W = crate::W<u32, super::DR3>;
#[doc = "Register DR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::DR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `D3`"]
pub type D3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `D3`"]
pub struct D3_W<'a> {
    w: &'a mut W,
}
impl<'a> D3_W<'a> {
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
    pub fn d3(&self) -> D3_R {
        D3_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d3(&mut self) -> D3_W {
        D3_W { w: self }
    }
}
