#[doc = "Register `plic_base_addr` reader"]
pub struct R(crate::R<PLIC_BASE_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLIC_BASE_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLIC_BASE_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLIC_BASE_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `plic_base_addr` writer"]
pub struct W(crate::W<PLIC_BASE_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLIC_BASE_ADDR_SPEC>;
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
impl From<crate::W<PLIC_BASE_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLIC_BASE_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `plic_base_addr` reader - "]
pub struct PLIC_BASE_ADDR_R(crate::FieldReader<u64, u64>);
impl PLIC_BASE_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u64) -> Self {
        PLIC_BASE_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLIC_BASE_ADDR_R {
    type Target = crate::FieldReader<u64, u64>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `plic_base_addr` writer - "]
pub struct PLIC_BASE_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> PLIC_BASE_ADDR_W<'a> {
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
    pub fn plic_base_addr(&self) -> PLIC_BASE_ADDR_R {
        PLIC_BASE_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn plic_base_addr(&mut self) -> PLIC_BASE_ADDR_W {
        PLIC_BASE_ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLIC base address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [plic_base_addr](index.html) module"]
pub struct PLIC_BASE_ADDR_SPEC;
impl crate::RegisterSpec for PLIC_BASE_ADDR_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [plic_base_addr::R](R) reader structure"]
impl crate::Readable for PLIC_BASE_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [plic_base_addr::W](W) writer structure"]
impl crate::Writable for PLIC_BASE_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets plic_base_addr to value 0"]
impl crate::Resettable for PLIC_BASE_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
