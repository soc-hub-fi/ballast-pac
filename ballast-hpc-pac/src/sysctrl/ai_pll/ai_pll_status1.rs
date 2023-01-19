#[doc = "Register `AI_PLL_STATUS1` reader"]
pub struct R(crate::R<AI_PLL_STATUS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AI_PLL_STATUS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AI_PLL_STATUS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AI_PLL_STATUS1_SPEC>) -> Self {
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
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ai_pll_status1](index.html) module"]
pub struct AI_PLL_STATUS1_SPEC;
impl crate::RegisterSpec for AI_PLL_STATUS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ai_pll_status1::R](R) reader structure"]
impl crate::Readable for AI_PLL_STATUS1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AI_PLL_STATUS1 to value 0"]
impl crate::Resettable for AI_PLL_STATUS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
