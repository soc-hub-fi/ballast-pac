#[doc = "Register `clint_base_addr` reader"]
pub struct R(crate::R<CLINT_BASE_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLINT_BASE_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLINT_BASE_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLINT_BASE_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `clint_base_addr` writer"]
pub struct W(crate::W<CLINT_BASE_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLINT_BASE_ADDR_SPEC>;
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
impl From<crate::W<CLINT_BASE_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLINT_BASE_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `clint_base_addr` reader - "]
pub type CLINT_BASE_ADDR_R = crate::FieldReader<u64, u64>;
#[doc = "Field `clint_base_addr` writer - "]
pub type CLINT_BASE_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u64, CLINT_BASE_ADDR_SPEC, u64, u64, 64, O>;
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn clint_base_addr(&self) -> CLINT_BASE_ADDR_R {
        CLINT_BASE_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    #[must_use]
    pub fn clint_base_addr(&mut self) -> CLINT_BASE_ADDR_W<0> {
        CLINT_BASE_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CLINT base address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clint_base_addr](index.html) module"]
pub struct CLINT_BASE_ADDR_SPEC;
impl crate::RegisterSpec for CLINT_BASE_ADDR_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [clint_base_addr::R](R) reader structure"]
impl crate::Readable for CLINT_BASE_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clint_base_addr::W](W) writer structure"]
impl crate::Writable for CLINT_BASE_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets clint_base_addr to value 0"]
impl crate::Resettable for CLINT_BASE_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
