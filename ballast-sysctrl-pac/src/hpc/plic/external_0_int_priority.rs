#[doc = "Register `external_0_int_priority[%s]` reader"]
pub struct R(crate::R<EXTERNAL_0_INT_PRIORITY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTERNAL_0_INT_PRIORITY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTERNAL_0_INT_PRIORITY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTERNAL_0_INT_PRIORITY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `external_0_int_priority[%s]` writer"]
pub struct W(crate::W<EXTERNAL_0_INT_PRIORITY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTERNAL_0_INT_PRIORITY_SPEC>;
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
impl From<crate::W<EXTERNAL_0_INT_PRIORITY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTERNAL_0_INT_PRIORITY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `external_0_int_priority` reader - "]
pub type EXTERNAL_0_INT_PRIORITY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `external_0_int_priority` writer - "]
pub type EXTERNAL_0_INT_PRIORITY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EXTERNAL_0_INT_PRIORITY_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn external_0_int_priority(&self) -> EXTERNAL_0_INT_PRIORITY_R {
        EXTERNAL_0_INT_PRIORITY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn external_0_int_priority(&mut self) -> EXTERNAL_0_INT_PRIORITY_W<0> {
        EXTERNAL_0_INT_PRIORITY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [external_0_int_priority](index.html) module"]
pub struct EXTERNAL_0_INT_PRIORITY_SPEC;
impl crate::RegisterSpec for EXTERNAL_0_INT_PRIORITY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [external_0_int_priority::R](R) reader structure"]
impl crate::Readable for EXTERNAL_0_INT_PRIORITY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [external_0_int_priority::W](W) writer structure"]
impl crate::Writable for EXTERNAL_0_INT_PRIORITY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets external_0_int_priority[%s]
to value 0"]
impl crate::Resettable for EXTERNAL_0_INT_PRIORITY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
