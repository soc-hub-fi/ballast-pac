#[doc = "Register `CORESTATUS` reader"]
pub struct R(crate::R<CORESTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORESTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORESTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORESTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORESTATUS` writer"]
pub struct W(crate::W<CORESTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORESTATUS_SPEC>;
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
impl From<crate::W<CORESTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORESTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_STATUS` reader - "]
pub type CORE_STATUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_STATUS` writer - "]
pub type CORE_STATUS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORESTATUS_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn core_status(&self) -> CORE_STATUS_R {
        CORE_STATUS_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn core_status(&mut self) -> CORE_STATUS_W<0> {
        CORE_STATUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "These 2 registers (CORESTATUS, CS_RO) contain the status of the system for testing/verification purposes like End Of Computation. The 0x1A10_40C0 register is read-only.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [corestatus](index.html) module"]
pub struct CORESTATUS_SPEC;
impl crate::RegisterSpec for CORESTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [corestatus::R](R) reader structure"]
impl crate::Readable for CORESTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [corestatus::W](W) writer structure"]
impl crate::Writable for CORESTATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORESTATUS to value 0"]
impl crate::Resettable for CORESTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
