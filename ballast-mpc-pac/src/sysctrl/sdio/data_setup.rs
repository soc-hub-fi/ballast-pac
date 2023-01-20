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
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DATA_SETUP_SPEC, bool, O>;
#[doc = "Field `RWN` writer - "]
pub type RWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DATA_SETUP_SPEC, bool, O>;
#[doc = "Field `QUAD` writer - "]
pub type QUAD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DATA_SETUP_SPEC, u8, u8, 6, O>;
#[doc = "Field `Block_Num` writer - "]
pub type BLOCK_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DATA_SETUP_SPEC, u8, u8, 8, O>;
#[doc = "Field `BLOCK_SIZE` writer - "]
pub type BLOCK_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DATA_SETUP_SPEC, u16, u16, 10, O>;
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rwn(&mut self) -> RWN_W<1> {
        RWN_W::new(self)
    }
    #[doc = "Bits 2:7"]
    #[inline(always)]
    #[must_use]
    pub fn quad(&mut self) -> QUAD_W<2> {
        QUAD_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn block_num(&mut self) -> BLOCK_NUM_W<8> {
        BLOCK_NUM_W::new(self)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    #[must_use]
    pub fn block_size(&mut self) -> BLOCK_SIZE_W<16> {
        BLOCK_SIZE_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA_SETUP to value 0"]
impl crate::Resettable for DATA_SETUP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
