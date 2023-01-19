#[doc = "Register `D_ZERO_PADDING_VALUE` reader"]
pub struct R(crate::R<D_ZERO_PADDING_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_ZERO_PADDING_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_ZERO_PADDING_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_ZERO_PADDING_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_ZERO_PADDING_VALUE` writer"]
pub struct W(crate::W<D_ZERO_PADDING_VALUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_ZERO_PADDING_VALUE_SPEC>;
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
impl From<crate::W<D_ZERO_PADDING_VALUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_ZERO_PADDING_VALUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAD_VALUE` reader - "]
pub type PAD_VALUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PAD_VALUE` writer - "]
pub type PAD_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_ZERO_PADDING_VALUE_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pad_value(&self) -> PAD_VALUE_R {
        PAD_VALUE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn pad_value(&mut self) -> PAD_VALUE_W<0> {
        PAD_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Padding value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_zero_padding_value](index.html) module"]
pub struct D_ZERO_PADDING_VALUE_SPEC;
impl crate::RegisterSpec for D_ZERO_PADDING_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_zero_padding_value::R](R) reader structure"]
impl crate::Readable for D_ZERO_PADDING_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_zero_padding_value::W](W) writer structure"]
impl crate::Writable for D_ZERO_PADDING_VALUE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_ZERO_PADDING_VALUE to value 0"]
impl crate::Resettable for D_ZERO_PADDING_VALUE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
