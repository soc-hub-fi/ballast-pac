#[doc = "Register `core0_hart_id` reader"]
pub struct R(crate::R<CORE0_HART_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE0_HART_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE0_HART_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE0_HART_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `core0_hart_id` writer"]
pub struct W(crate::W<CORE0_HART_ID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE0_HART_ID_SPEC>;
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
impl From<crate::W<CORE0_HART_ID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE0_HART_ID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `core0_hart_id` reader - "]
pub type CORE0_HART_ID_R = crate::FieldReader<u64, u64>;
#[doc = "Field `core0_hart_id` writer - "]
pub type CORE0_HART_ID_W<'a, const O: u8> =
    crate::FieldWriter<'a, u64, CORE0_HART_ID_SPEC, u64, u64, 64, O>;
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn core0_hart_id(&self) -> CORE0_HART_ID_R {
        CORE0_HART_ID_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    #[must_use]
    pub fn core0_hart_id(&mut self) -> CORE0_HART_ID_W<0> {
        CORE0_HART_ID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core#0 hart ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core0_hart_id](index.html) module"]
pub struct CORE0_HART_ID_SPEC;
impl crate::RegisterSpec for CORE0_HART_ID_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [core0_hart_id::R](R) reader structure"]
impl crate::Readable for CORE0_HART_ID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core0_hart_id::W](W) writer structure"]
impl crate::Writable for CORE0_HART_ID_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets core0_hart_id to value 0"]
impl crate::Resettable for CORE0_HART_ID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
