#[doc = "Register `timer0_cmp` reader"]
pub struct R(crate::R<TIMER0_CMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER0_CMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER0_CMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER0_CMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `timer0_cmp` writer"]
pub struct W(crate::W<TIMER0_CMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER0_CMP_SPEC>;
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
impl From<crate::W<TIMER0_CMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER0_CMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `timer0_cmp` reader - "]
pub type TIMER0_CMP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `timer0_cmp` writer - "]
pub type TIMER0_CMP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMER0_CMP_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn timer0_cmp(&self) -> TIMER0_CMP_R {
        TIMER0_CMP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn timer0_cmp(&mut self) -> TIMER0_CMP_W<0> {
        TIMER0_CMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer0_cmp](index.html) module"]
pub struct TIMER0_CMP_SPEC;
impl crate::RegisterSpec for TIMER0_CMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer0_cmp::R](R) reader structure"]
impl crate::Readable for TIMER0_CMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer0_cmp::W](W) writer structure"]
impl crate::Writable for TIMER0_CMP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets timer0_cmp to value 0"]
impl crate::Resettable for TIMER0_CMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
