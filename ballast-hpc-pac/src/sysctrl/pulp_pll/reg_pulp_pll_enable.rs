#[doc = "Register `REG_PULP_PLL_ENABLE` reader"]
pub struct R(crate::R<REG_PULP_PLL_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_PULP_PLL_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_PULP_PLL_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_PULP_PLL_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG_PULP_PLL_ENABLE` writer"]
pub struct W(crate::W<REG_PULP_PLL_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_PULP_PLL_ENABLE_SPEC>;
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
impl From<crate::W<REG_PULP_PLL_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_PULP_PLL_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `spare_ctrl` reader - "]
pub struct SPARE_CTRL_R(crate::FieldReader<u32>);
impl SPARE_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SPARE_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPARE_CTRL_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `spare_ctrl` writer - "]
pub struct SPARE_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
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
    pub fn spare_ctrl(&mut self) -> SPARE_CTRL_W {
        SPARE_CTRL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_pulp_pll_enable](index.html) module"]
pub struct REG_PULP_PLL_ENABLE_SPEC;
impl crate::RegisterSpec for REG_PULP_PLL_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_pulp_pll_enable::R](R) reader structure"]
impl crate::Readable for REG_PULP_PLL_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg_pulp_pll_enable::W](W) writer structure"]
impl crate::Writable for REG_PULP_PLL_ENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG_PULP_PLL_ENABLE to value 0"]
impl crate::Resettable for REG_PULP_PLL_ENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
