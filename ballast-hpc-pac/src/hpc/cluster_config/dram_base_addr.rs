#[doc = "Register `dram_base_addr` reader"]
pub struct R(crate::R<DRAM_BASE_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DRAM_BASE_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DRAM_BASE_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DRAM_BASE_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dram_base_addr` writer"]
pub struct W(crate::W<DRAM_BASE_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DRAM_BASE_ADDR_SPEC>;
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
impl From<crate::W<DRAM_BASE_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DRAM_BASE_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dram_base_addr` reader - "]
pub type DRAM_BASE_ADDR_R = crate::FieldReader<u64, u64>;
#[doc = "Field `dram_base_addr` writer - "]
pub type DRAM_BASE_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u64, DRAM_BASE_ADDR_SPEC, u64, u64, 64, O>;
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn dram_base_addr(&self) -> DRAM_BASE_ADDR_R {
        DRAM_BASE_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    #[must_use]
    pub fn dram_base_addr(&mut self) -> DRAM_BASE_ADDR_W<0> {
        DRAM_BASE_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External memory base address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dram_base_addr](index.html) module"]
pub struct DRAM_BASE_ADDR_SPEC;
impl crate::RegisterSpec for DRAM_BASE_ADDR_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [dram_base_addr::R](R) reader structure"]
impl crate::Readable for DRAM_BASE_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dram_base_addr::W](W) writer structure"]
impl crate::Writable for DRAM_BASE_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dram_base_addr to value 0"]
impl crate::Resettable for DRAM_BASE_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
