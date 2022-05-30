#[doc = "Register `REG_SS_CLK_CTRL3` reader"]
pub struct R(crate::R<REG_SS_CLK_CTRL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_SS_CLK_CTRL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_SS_CLK_CTRL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_SS_CLK_CTRL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG_SS_CLK_CTRL3` writer"]
pub struct W(crate::W<REG_SS_CLK_CTRL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_SS_CLK_CTRL3_SPEC>;
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
impl From<crate::W<REG_SS_CLK_CTRL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_SS_CLK_CTRL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `top_periph_sel_cka` reader - Select CKA"]
pub struct TOP_PERIPH_SEL_CKA_R(crate::FieldReader<bool>);
impl TOP_PERIPH_SEL_CKA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOP_PERIPH_SEL_CKA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOP_PERIPH_SEL_CKA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `top_periph_sel_cka` writer - Select CKA"]
pub struct TOP_PERIPH_SEL_CKA_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP_PERIPH_SEL_CKA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
#[doc = "Field `top_periph_force_cka` reader - Force CKA"]
pub struct TOP_PERIPH_FORCE_CKA_R(crate::FieldReader<bool>);
impl TOP_PERIPH_FORCE_CKA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOP_PERIPH_FORCE_CKA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOP_PERIPH_FORCE_CKA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `top_periph_force_cka` writer - Force CKA"]
pub struct TOP_PERIPH_FORCE_CKA_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP_PERIPH_FORCE_CKA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `top_periph_force_ckb` reader - Force CKB"]
pub struct TOP_PERIPH_FORCE_CKB_R(crate::FieldReader<bool>);
impl TOP_PERIPH_FORCE_CKB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOP_PERIPH_FORCE_CKB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOP_PERIPH_FORCE_CKB_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `top_periph_force_ckb` writer - Force CKB"]
pub struct TOP_PERIPH_FORCE_CKB_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP_PERIPH_FORCE_CKB_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `top_periph_subsys_clkena` reader - Subsystem clock enable"]
pub struct TOP_PERIPH_SUBSYS_CLKENA_R(crate::FieldReader<bool>);
impl TOP_PERIPH_SUBSYS_CLKENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOP_PERIPH_SUBSYS_CLKENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOP_PERIPH_SUBSYS_CLKENA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `top_periph_subsys_clkena` writer - Subsystem clock enable"]
pub struct TOP_PERIPH_SUBSYS_CLKENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP_PERIPH_SUBSYS_CLKENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `top_periph_pll_ctrl_valid` reader - PLL Control valid"]
pub struct TOP_PERIPH_PLL_CTRL_VALID_R(crate::FieldReader<bool>);
impl TOP_PERIPH_PLL_CTRL_VALID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOP_PERIPH_PLL_CTRL_VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOP_PERIPH_PLL_CTRL_VALID_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `top_periph_pll_ctrl_valid` writer - PLL Control valid"]
pub struct TOP_PERIPH_PLL_CTRL_VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP_PERIPH_PLL_CTRL_VALID_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Select CKA"]
    #[inline(always)]
    pub fn top_periph_sel_cka(&self) -> TOP_PERIPH_SEL_CKA_R {
        TOP_PERIPH_SEL_CKA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force CKA"]
    #[inline(always)]
    pub fn top_periph_force_cka(&self) -> TOP_PERIPH_FORCE_CKA_R {
        TOP_PERIPH_FORCE_CKA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force CKB"]
    #[inline(always)]
    pub fn top_periph_force_ckb(&self) -> TOP_PERIPH_FORCE_CKB_R {
        TOP_PERIPH_FORCE_CKB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Subsystem clock enable"]
    #[inline(always)]
    pub fn top_periph_subsys_clkena(&self) -> TOP_PERIPH_SUBSYS_CLKENA_R {
        TOP_PERIPH_SUBSYS_CLKENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - PLL Control valid"]
    #[inline(always)]
    pub fn top_periph_pll_ctrl_valid(&self) -> TOP_PERIPH_PLL_CTRL_VALID_R {
        TOP_PERIPH_PLL_CTRL_VALID_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select CKA"]
    #[inline(always)]
    pub fn top_periph_sel_cka(&mut self) -> TOP_PERIPH_SEL_CKA_W {
        TOP_PERIPH_SEL_CKA_W { w: self }
    }
    #[doc = "Bit 1 - Force CKA"]
    #[inline(always)]
    pub fn top_periph_force_cka(&mut self) -> TOP_PERIPH_FORCE_CKA_W {
        TOP_PERIPH_FORCE_CKA_W { w: self }
    }
    #[doc = "Bit 2 - Force CKB"]
    #[inline(always)]
    pub fn top_periph_force_ckb(&mut self) -> TOP_PERIPH_FORCE_CKB_W {
        TOP_PERIPH_FORCE_CKB_W { w: self }
    }
    #[doc = "Bit 3 - Subsystem clock enable"]
    #[inline(always)]
    pub fn top_periph_subsys_clkena(&mut self) -> TOP_PERIPH_SUBSYS_CLKENA_W {
        TOP_PERIPH_SUBSYS_CLKENA_W { w: self }
    }
    #[doc = "Bit 7 - PLL Control valid"]
    #[inline(always)]
    pub fn top_periph_pll_ctrl_valid(&mut self) -> TOP_PERIPH_PLL_CTRL_VALID_W {
        TOP_PERIPH_PLL_CTRL_VALID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Subsystem Clock selection. Bit definition for Top peripheral subsystem. *_CLK_CTRL Bitfield definitions \\[bit\\]:\\[field\\]
0: sel_cka 1: force_cka 2: force_ckb 3: subsys_clkena 4: - 5: - 6: - 7: pll_ctrl_valid\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_ss_clk_ctrl3](index.html) module"]
pub struct REG_SS_CLK_CTRL3_SPEC;
impl crate::RegisterSpec for REG_SS_CLK_CTRL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_ss_clk_ctrl3::R](R) reader structure"]
impl crate::Readable for REG_SS_CLK_CTRL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg_ss_clk_ctrl3::W](W) writer structure"]
impl crate::Writable for REG_SS_CLK_CTRL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG_SS_CLK_CTRL3 to value 0"]
impl crate::Resettable for REG_SS_CLK_CTRL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
