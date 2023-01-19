#[doc = "Register `S_LUT_ACCESS_DATA` reader"]
pub struct R(crate::R<S_LUT_ACCESS_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S_LUT_ACCESS_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S_LUT_ACCESS_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S_LUT_ACCESS_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S_LUT_ACCESS_DATA` writer"]
pub struct W(crate::W<S_LUT_ACCESS_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S_LUT_ACCESS_DATA_SPEC>;
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
impl From<crate::W<S_LUT_ACCESS_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S_LUT_ACCESS_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LUT_DATA` reader - "]
pub type LUT_DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LUT_DATA` writer - "]
pub type LUT_DATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, S_LUT_ACCESS_DATA_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn lut_data(&self) -> LUT_DATA_R {
        LUT_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn lut_data(&mut self) -> LUT_DATA_W<0> {
        LUT_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data register of read or write LUT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s_lut_access_data](index.html) module"]
pub struct S_LUT_ACCESS_DATA_SPEC;
impl crate::RegisterSpec for S_LUT_ACCESS_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s_lut_access_data::R](R) reader structure"]
impl crate::Readable for S_LUT_ACCESS_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s_lut_access_data::W](W) writer structure"]
impl crate::Writable for S_LUT_ACCESS_DATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets S_LUT_ACCESS_DATA to value 0"]
impl crate::Resettable for S_LUT_ACCESS_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
