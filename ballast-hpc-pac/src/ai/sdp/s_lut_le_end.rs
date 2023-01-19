#[doc = "Register `S_LUT_LE_END` reader"]
pub struct R(crate::R<S_LUT_LE_END_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S_LUT_LE_END_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S_LUT_LE_END_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S_LUT_LE_END_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S_LUT_LE_END` writer"]
pub struct W(crate::W<S_LUT_LE_END_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S_LUT_LE_END_SPEC>;
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
impl From<crate::W<S_LUT_LE_END_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S_LUT_LE_END_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LUT_LE_END` reader - "]
pub type LUT_LE_END_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LUT_LE_END` writer - "]
pub type LUT_LE_END_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, S_LUT_LE_END_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn lut_le_end(&self) -> LUT_LE_END_R {
        LUT_LE_END_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn lut_le_end(&mut self) -> LUT_LE_END_W<0> {
        LUT_LE_END_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "End of LE LUT’s range\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s_lut_le_end](index.html) module"]
pub struct S_LUT_LE_END_SPEC;
impl crate::RegisterSpec for S_LUT_LE_END_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s_lut_le_end::R](R) reader structure"]
impl crate::Readable for S_LUT_LE_END_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s_lut_le_end::W](W) writer structure"]
impl crate::Writable for S_LUT_LE_END_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets S_LUT_LE_END to value 0"]
impl crate::Resettable for S_LUT_LE_END_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
