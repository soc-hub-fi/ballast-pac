#[doc = "Register `timer_base_addr` reader"]
pub struct R(crate::R<TIMER_BASE_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER_BASE_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER_BASE_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER_BASE_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `timer_base_addr` writer"]
pub struct W(crate::W<TIMER_BASE_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER_BASE_ADDR_SPEC>;
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
impl From<crate::W<TIMER_BASE_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER_BASE_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `timer_base_addr` reader - "]
pub struct TIMER_BASE_ADDR_R(crate::FieldReader<u64>);
impl TIMER_BASE_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u64) -> Self {
        TIMER_BASE_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_BASE_ADDR_R {
    type Target = crate::FieldReader<u64>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `timer_base_addr` writer - "]
pub struct TIMER_BASE_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_BASE_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u64) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn timer_base_addr(&self) -> TIMER_BASE_ADDR_R {
        TIMER_BASE_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn timer_base_addr(&mut self) -> TIMER_BASE_ADDR_W {
        TIMER_BASE_ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer base address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_base_addr](index.html) module"]
pub struct TIMER_BASE_ADDR_SPEC;
impl crate::RegisterSpec for TIMER_BASE_ADDR_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [timer_base_addr::R](R) reader structure"]
impl crate::Readable for TIMER_BASE_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer_base_addr::W](W) writer structure"]
impl crate::Writable for TIMER_BASE_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets timer_base_addr to value 0"]
impl crate::Resettable for TIMER_BASE_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
