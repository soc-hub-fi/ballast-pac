#[doc = "Register `D_POOLING_PADDING_CFG` reader"]
pub struct R(crate::R<D_POOLING_PADDING_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_POOLING_PADDING_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_POOLING_PADDING_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_POOLING_PADDING_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_POOLING_PADDING_CFG` writer"]
pub struct W(crate::W<D_POOLING_PADDING_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_POOLING_PADDING_CFG_SPEC>;
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
impl From<crate::W<D_POOLING_PADDING_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_POOLING_PADDING_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAD_WIDTH` reader - "]
pub type PAD_WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAD_WIDTH` writer - "]
pub type PAD_WIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_POOLING_PADDING_CFG_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn pad_width(&self) -> PAD_WIDTH_R {
        PAD_WIDTH_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn pad_width(&mut self) -> PAD_WIDTH_W<0> {
        PAD_WIDTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_pooling_padding_cfg](index.html) module"]
pub struct D_POOLING_PADDING_CFG_SPEC;
impl crate::RegisterSpec for D_POOLING_PADDING_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_pooling_padding_cfg::R](R) reader structure"]
impl crate::Readable for D_POOLING_PADDING_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_pooling_padding_cfg::W](W) writer structure"]
impl crate::Writable for D_POOLING_PADDING_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_POOLING_PADDING_CFG to value 0"]
impl crate::Resettable for D_POOLING_PADDING_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
