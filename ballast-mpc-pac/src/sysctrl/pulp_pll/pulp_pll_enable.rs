#[doc = "Register `PULP_PLL_ENABLE` reader"]
pub struct R(crate::R<PULP_PLL_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PULP_PLL_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PULP_PLL_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PULP_PLL_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PULP_PLL_ENABLE` writer"]
pub struct W(crate::W<PULP_PLL_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PULP_PLL_ENABLE_SPEC>;
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
impl From<crate::W<PULP_PLL_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PULP_PLL_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `spare_ctrl` reader - "]
pub type SPARE_CTRL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `spare_ctrl` writer - "]
pub type SPARE_CTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PULP_PLL_ENABLE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn spare_ctrl(&self) -> SPARE_CTRL_R {
        SPARE_CTRL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn spare_ctrl(&mut self) -> SPARE_CTRL_W<0> {
        SPARE_CTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pulp_pll_enable](index.html) module"]
pub struct PULP_PLL_ENABLE_SPEC;
impl crate::RegisterSpec for PULP_PLL_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pulp_pll_enable::R](R) reader structure"]
impl crate::Readable for PULP_PLL_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pulp_pll_enable::W](W) writer structure"]
impl crate::Writable for PULP_PLL_ENABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PULP_PLL_ENABLE to value 0"]
impl crate::Resettable for PULP_PLL_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
