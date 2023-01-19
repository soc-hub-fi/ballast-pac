#[doc = "Register `PULP_PLL_STATUS1` reader"]
pub struct R(crate::R<PULP_PLL_STATUS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PULP_PLL_STATUS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PULP_PLL_STATUS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PULP_PLL_STATUS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `status1` reader - "]
pub type STATUS1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn status1(&self) -> STATUS1_R {
        STATUS1_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pulp_pll_status1](index.html) module"]
pub struct PULP_PLL_STATUS1_SPEC;
impl crate::RegisterSpec for PULP_PLL_STATUS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pulp_pll_status1::R](R) reader structure"]
impl crate::Readable for PULP_PLL_STATUS1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PULP_PLL_STATUS1 to value 0"]
impl crate::Resettable for PULP_PLL_STATUS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
