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
#[doc = "Field `Top_Periph_CLK_CTRL` reader - "]
pub struct TOP_PERIPH_CLK_CTRL_R(crate::FieldReader<u8, u8>);
impl TOP_PERIPH_CLK_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TOP_PERIPH_CLK_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOP_PERIPH_CLK_CTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Top_Periph_CLK_CTRL` writer - "]
pub struct TOP_PERIPH_CLK_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP_PERIPH_CLK_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn top_periph_clk_ctrl(&self) -> TOP_PERIPH_CLK_CTRL_R {
        TOP_PERIPH_CLK_CTRL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn top_periph_clk_ctrl(&mut self) -> TOP_PERIPH_CLK_CTRL_W {
        TOP_PERIPH_CLK_CTRL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Subsystem Clock selection. Bit definition *_CLK_CTRL Bitfield definitions \\[bit\\]:\\[field\\]
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
