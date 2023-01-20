#[doc = "Register `INTER_PLL_STATUS2` reader"]
pub struct R(crate::R<INTER_PLL_STATUS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTER_PLL_STATUS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTER_PLL_STATUS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTER_PLL_STATUS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `status2` reader - "]
pub type STATUS2_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn status2(&self) -> STATUS2_R {
        STATUS2_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inter_pll_status2](index.html) module"]
pub struct INTER_PLL_STATUS2_SPEC;
impl crate::RegisterSpec for INTER_PLL_STATUS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inter_pll_status2::R](R) reader structure"]
impl crate::Readable for INTER_PLL_STATUS2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTER_PLL_STATUS2 to value 0"]
impl crate::Resettable for INTER_PLL_STATUS2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
