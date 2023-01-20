#[doc = "Register `execute_region_addr_base2` reader"]
pub struct R(crate::R<EXECUTE_REGION_ADDR_BASE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXECUTE_REGION_ADDR_BASE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXECUTE_REGION_ADDR_BASE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXECUTE_REGION_ADDR_BASE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `execute_region_addr_base2` writer"]
pub struct W(crate::W<EXECUTE_REGION_ADDR_BASE2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXECUTE_REGION_ADDR_BASE2_SPEC>;
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
impl From<crate::W<EXECUTE_REGION_ADDR_BASE2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXECUTE_REGION_ADDR_BASE2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `execute_region_addr_base2` reader - "]
pub type EXECUTE_REGION_ADDR_BASE2_R = crate::FieldReader<u64, u64>;
#[doc = "Field `execute_region_addr_base2` writer - "]
pub type EXECUTE_REGION_ADDR_BASE2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u64, EXECUTE_REGION_ADDR_BASE2_SPEC, u64, u64, 64, O>;
impl R {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    pub fn execute_region_addr_base2(&self) -> EXECUTE_REGION_ADDR_BASE2_R {
        EXECUTE_REGION_ADDR_BASE2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63"]
    #[inline(always)]
    #[must_use]
    pub fn execute_region_addr_base2(&mut self) -> EXECUTE_REGION_ADDR_BASE2_W<0> {
        EXECUTE_REGION_ADDR_BASE2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [execute_region_addr_base2](index.html) module"]
pub struct EXECUTE_REGION_ADDR_BASE2_SPEC;
impl crate::RegisterSpec for EXECUTE_REGION_ADDR_BASE2_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [execute_region_addr_base2::R](R) reader structure"]
impl crate::Readable for EXECUTE_REGION_ADDR_BASE2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [execute_region_addr_base2::W](W) writer structure"]
impl crate::Writable for EXECUTE_REGION_ADDR_BASE2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets execute_region_addr_base2 to value 0"]
impl crate::Resettable for EXECUTE_REGION_ADDR_BASE2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
