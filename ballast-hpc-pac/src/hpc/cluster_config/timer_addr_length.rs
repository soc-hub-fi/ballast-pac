#[doc = "Register `timer_addr_length` reader"]
pub struct R(crate::R<TIMER_ADDR_LENGTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER_ADDR_LENGTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER_ADDR_LENGTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER_ADDR_LENGTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `timer_addr_length` writer"]
pub struct W(crate::W<TIMER_ADDR_LENGTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER_ADDR_LENGTH_SPEC>;
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
impl From<crate::W<TIMER_ADDR_LENGTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER_ADDR_LENGTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `timer_addr_length` reader - "]
pub type TIMER_ADDR_LENGTH_R = crate::FieldReader<u64, u64>;
#[doc = "Field `timer_addr_length` writer - "]
pub type TIMER_ADDR_LENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u64, TIMER_ADDR_LENGTH_SPEC, u64, u64, 64, O>;
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn timer_addr_length(&self) -> TIMER_ADDR_LENGTH_R {
        TIMER_ADDR_LENGTH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    #[must_use]
    pub fn timer_addr_length(&mut self) -> TIMER_ADDR_LENGTH_W<0> {
        TIMER_ADDR_LENGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer address space length\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_addr_length](index.html) module"]
pub struct TIMER_ADDR_LENGTH_SPEC;
impl crate::RegisterSpec for TIMER_ADDR_LENGTH_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [timer_addr_length::R](R) reader structure"]
impl crate::Readable for TIMER_ADDR_LENGTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer_addr_length::W](W) writer structure"]
impl crate::Writable for TIMER_ADDR_LENGTH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets timer_addr_length to value 0"]
impl crate::Resettable for TIMER_ADDR_LENGTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
