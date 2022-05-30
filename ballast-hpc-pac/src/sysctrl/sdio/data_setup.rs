#[doc = "Register `DATA_SETUP` writer"]
pub struct W(crate::W<DATA_SETUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_SETUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DATA_SETUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_SETUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` writer - "]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
#[doc = "Field `RWN` writer - "]
pub struct RWN_W<'a> {
    w: &'a mut W,
}
impl<'a> RWN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `QUAD` writer - "]
pub struct QUAD_W<'a> {
    w: &'a mut W,
}
impl<'a> QUAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | ((value as u32 & 0x3f) << 2);
        self.w
    }
}
#[doc = "Field `Block_Num` writer - "]
pub struct BLOCK_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCK_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `BLOCK_SIZE` writer - "]
pub struct BLOCK_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCK_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | ((value as u32 & 0x03ff) << 16);
        self.w
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rwn(&mut self) -> RWN_W {
        RWN_W { w: self }
    }
    #[doc = "Bits 2:7"]
    #[inline(always)]
    pub fn quad(&mut self) -> QUAD_W {
        QUAD_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn block_num(&mut self) -> BLOCK_NUM_W {
        BLOCK_NUM_W { w: self }
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn block_size(&mut self) -> BLOCK_SIZE_W {
        BLOCK_SIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_setup](index.html) module"]
pub struct DATA_SETUP_SPEC;
impl crate::RegisterSpec for DATA_SETUP_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [data_setup::W](W) writer structure"]
impl crate::Writable for DATA_SETUP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATA_SETUP to value 0"]
impl crate::Resettable for DATA_SETUP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
