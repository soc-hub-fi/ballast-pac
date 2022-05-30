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
pub struct CORE_STATUS_R(crate::FieldReader<u8>);
impl CORE_STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CORE_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_STATUS_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_STATUS` writer - "]
pub struct CORE_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
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
    pub fn core_status(&mut self) -> CORE_STATUS_W {
        CORE_STATUS_W { w: self }
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
}
#[doc = "`reset()` method sets CORESTATUS to value 0"]
impl crate::Resettable for CORESTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
