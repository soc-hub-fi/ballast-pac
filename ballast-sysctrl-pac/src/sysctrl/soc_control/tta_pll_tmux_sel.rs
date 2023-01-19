#[doc = "Register `TTA_PLL_TMUX_SEL` reader"]
pub struct R(crate::R<TTA_PLL_TMUX_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTA_PLL_TMUX_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTA_PLL_TMUX_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTA_PLL_TMUX_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TTA_PLL_TMUX_SEL` writer"]
pub struct W(crate::W<TTA_PLL_TMUX_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TTA_PLL_TMUX_SEL_SPEC>;
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
impl From<crate::W<TTA_PLL_TMUX_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TTA_PLL_TMUX_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Tmux_1` reader - "]
pub type TMUX_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Tmux_1` writer - "]
pub type TMUX_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TTA_PLL_TMUX_SEL_SPEC, u8, u8, 4, O>;
#[doc = "Field `Tmux_2` reader - "]
pub type TMUX_2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Tmux_2` writer - "]
pub type TMUX_2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TTA_PLL_TMUX_SEL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn tmux_1(&self) -> TMUX_1_R {
        TMUX_1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn tmux_2(&self) -> TMUX_2_R {
        TMUX_2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn tmux_1(&mut self) -> TMUX_1_W<0> {
        TMUX_1_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn tmux_2(&mut self) -> TMUX_2_W<4> {
        TMUX_2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tta_pll_tmux_sel](index.html) module"]
pub struct TTA_PLL_TMUX_SEL_SPEC;
impl crate::RegisterSpec for TTA_PLL_TMUX_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tta_pll_tmux_sel::R](R) reader structure"]
impl crate::Readable for TTA_PLL_TMUX_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tta_pll_tmux_sel::W](W) writer structure"]
impl crate::Writable for TTA_PLL_TMUX_SEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TTA_PLL_TMUX_SEL to value 0"]
impl crate::Resettable for TTA_PLL_TMUX_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
