#[doc = "Register `PADOUT_00_31` reader"]
pub struct R(crate::R<PADOUT_00_31_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADOUT_00_31_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADOUT_00_31_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADOUT_00_31_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADOUT_00_31` writer"]
pub struct W(crate::W<PADOUT_00_31_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADOUT_00_31_SPEC>;
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
impl From<crate::W<PADOUT_00_31_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADOUT_00_31_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_OUT` reader - "]
pub type DATA_OUT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATA_OUT` writer - "]
pub type DATA_OUT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PADOUT_00_31_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn data_out(&self) -> DATA_OUT_R {
        DATA_OUT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn data_out(&mut self) -> DATA_OUT_W<0> {
        DATA_OUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bit 31 - 0 DATA_OUT (R/W) GPIO\\[31:0\\]
output data read bitfield. DATA_OUT\\[i\\]
corresponds to output data set on GPIO\\[i\\].\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padout_00_31](index.html) module"]
pub struct PADOUT_00_31_SPEC;
impl crate::RegisterSpec for PADOUT_00_31_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [padout_00_31::R](R) reader structure"]
impl crate::Readable for PADOUT_00_31_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [padout_00_31::W](W) writer structure"]
impl crate::Writable for PADOUT_00_31_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PADOUT_00_31 to value 0"]
impl crate::Resettable for PADOUT_00_31_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
