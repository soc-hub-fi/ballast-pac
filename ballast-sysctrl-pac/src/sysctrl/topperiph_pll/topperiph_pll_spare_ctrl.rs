#[doc = "Register `TOPPERIPH_PLL_SPARE_CTRL` reader"]
pub struct R(crate::R<TOPPERIPH_PLL_SPARE_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOPPERIPH_PLL_SPARE_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOPPERIPH_PLL_SPARE_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOPPERIPH_PLL_SPARE_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOPPERIPH_PLL_SPARE_CTRL` writer"]
pub struct W(crate::W<TOPPERIPH_PLL_SPARE_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOPPERIPH_PLL_SPARE_CTRL_SPEC>;
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
impl From<crate::W<TOPPERIPH_PLL_SPARE_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOPPERIPH_PLL_SPARE_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `spare_ctrl` reader - "]
pub type SPARE_CTRL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `spare_ctrl` writer - "]
pub type SPARE_CTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TOPPERIPH_PLL_SPARE_CTRL_SPEC, u32, u32, 32, O>;
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
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [topperiph_pll_spare_ctrl](index.html) module"]
pub struct TOPPERIPH_PLL_SPARE_CTRL_SPEC;
impl crate::RegisterSpec for TOPPERIPH_PLL_SPARE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [topperiph_pll_spare_ctrl::R](R) reader structure"]
impl crate::Readable for TOPPERIPH_PLL_SPARE_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [topperiph_pll_spare_ctrl::W](W) writer structure"]
impl crate::Writable for TOPPERIPH_PLL_SPARE_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOPPERIPH_PLL_SPARE_CTRL to value 0"]
impl crate::Resettable for TOPPERIPH_PLL_SPARE_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
