#[doc = "Register `D_ZERO_PADDING` reader"]
pub struct R(crate::R<D_ZERO_PADDING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_ZERO_PADDING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_ZERO_PADDING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_ZERO_PADDING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_ZERO_PADDING` writer"]
pub struct W(crate::W<D_ZERO_PADDING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_ZERO_PADDING_SPEC>;
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
impl From<crate::W<D_ZERO_PADDING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_ZERO_PADDING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAD_LEFT` reader - "]
pub type PAD_LEFT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAD_LEFT` writer - "]
pub type PAD_LEFT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_ZERO_PADDING_SPEC, u8, u8, 5, O>;
#[doc = "Field `PAD_TOP` reader - "]
pub type PAD_TOP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAD_TOP` writer - "]
pub type PAD_TOP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_ZERO_PADDING_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn pad_left(&self) -> PAD_LEFT_R {
        PAD_LEFT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn pad_top(&self) -> PAD_TOP_R {
        PAD_TOP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn pad_left(&mut self) -> PAD_LEFT_W<0> {
        PAD_LEFT_W::new(self)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    #[must_use]
    pub fn pad_top(&mut self) -> PAD_TOP_W<16> {
        PAD_TOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Left/right/top/bottom padding size\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_zero_padding](index.html) module"]
pub struct D_ZERO_PADDING_SPEC;
impl crate::RegisterSpec for D_ZERO_PADDING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_zero_padding::R](R) reader structure"]
impl crate::Readable for D_ZERO_PADDING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_zero_padding::W](W) writer structure"]
impl crate::Writable for D_ZERO_PADDING_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_ZERO_PADDING to value 0"]
impl crate::Resettable for D_ZERO_PADDING_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
