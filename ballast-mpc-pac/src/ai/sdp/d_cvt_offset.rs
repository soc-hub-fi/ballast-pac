#[doc = "Register `D_CVT_OFFSET` reader"]
pub struct R(crate::R<D_CVT_OFFSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_CVT_OFFSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_CVT_OFFSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_CVT_OFFSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_CVT_OFFSET` writer"]
pub struct W(crate::W<D_CVT_OFFSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_CVT_OFFSET_SPEC>;
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
impl From<crate::W<D_CVT_OFFSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_CVT_OFFSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CVT_OFFSET` reader - "]
pub type CVT_OFFSET_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CVT_OFFSET` writer - "]
pub type CVT_OFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_CVT_OFFSET_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cvt_offset(&self) -> CVT_OFFSET_R {
        CVT_OFFSET_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn cvt_offset(&mut self) -> CVT_OFFSET_W<0> {
        CVT_OFFSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output converter offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_cvt_offset](index.html) module"]
pub struct D_CVT_OFFSET_SPEC;
impl crate::RegisterSpec for D_CVT_OFFSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_cvt_offset::R](R) reader structure"]
impl crate::Readable for D_CVT_OFFSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_cvt_offset::W](W) writer structure"]
impl crate::Writable for D_CVT_OFFSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_CVT_OFFSET to value 0"]
impl crate::Resettable for D_CVT_OFFSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
