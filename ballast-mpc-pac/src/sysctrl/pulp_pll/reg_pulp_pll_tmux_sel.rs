#[doc = "Register `REG_PULP_PLL_TMUX_SEL` reader"]
pub struct R(crate::R<REG_PULP_PLL_TMUX_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_PULP_PLL_TMUX_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_PULP_PLL_TMUX_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_PULP_PLL_TMUX_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG_PULP_PLL_TMUX_SEL` writer"]
pub struct W(crate::W<REG_PULP_PLL_TMUX_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_PULP_PLL_TMUX_SEL_SPEC>;
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
impl From<crate::W<REG_PULP_PLL_TMUX_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_PULP_PLL_TMUX_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Tmux_1` reader - "]
pub struct TMUX_1_R(crate::FieldReader<u8, u8>);
impl TMUX_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TMUX_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMUX_1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Tmux_1` writer - "]
pub struct TMUX_1_W<'a> {
    w: &'a mut W,
}
impl<'a> TMUX_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `Tmux_2` reader - "]
pub struct TMUX_2_R(crate::FieldReader<u8, u8>);
impl TMUX_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TMUX_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMUX_2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Tmux_2` writer - "]
pub struct TMUX_2_W<'a> {
    w: &'a mut W,
}
impl<'a> TMUX_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
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
    pub fn tmux_1(&mut self) -> TMUX_1_W {
        TMUX_1_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn tmux_2(&mut self) -> TMUX_2_W {
        TMUX_2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_pulp_pll_tmux_sel](index.html) module"]
pub struct REG_PULP_PLL_TMUX_SEL_SPEC;
impl crate::RegisterSpec for REG_PULP_PLL_TMUX_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_pulp_pll_tmux_sel::R](R) reader structure"]
impl crate::Readable for REG_PULP_PLL_TMUX_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg_pulp_pll_tmux_sel::W](W) writer structure"]
impl crate::Writable for REG_PULP_PLL_TMUX_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG_PULP_PLL_TMUX_SEL to value 0"]
impl crate::Resettable for REG_PULP_PLL_TMUX_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
