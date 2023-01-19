#[doc = "Register `AI_PLL_STATUS2` reader"]
pub struct R(crate::R<AI_PLL_STATUS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AI_PLL_STATUS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AI_PLL_STATUS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AI_PLL_STATUS2_SPEC>) -> Self {
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
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ai_pll_status2](index.html) module"]
pub struct AI_PLL_STATUS2_SPEC;
impl crate::RegisterSpec for AI_PLL_STATUS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ai_pll_status2::R](R) reader structure"]
impl crate::Readable for AI_PLL_STATUS2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AI_PLL_STATUS2 to value 0"]
impl crate::Resettable for AI_PLL_STATUS2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
