#[doc = "Register `REG_SS_CLK_CTRL1` reader"]
pub struct R(crate::R<REG_SS_CLK_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_SS_CLK_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_SS_CLK_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_SS_CLK_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG_SS_CLK_CTRL1` writer"]
pub struct W(crate::W<REG_SS_CLK_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_SS_CLK_CTRL1_SPEC>;
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
impl From<crate::W<REG_SS_CLK_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_SS_CLK_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TTA_CLK_CTRL` reader - "]
pub struct TTA_CLK_CTRL_R(crate::FieldReader<u8>);
impl TTA_CLK_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TTA_CLK_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TTA_CLK_CTRL_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TTA_CLK_CTRL` writer - "]
pub struct TTA_CLK_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> TTA_CLK_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `Ethernet_CLK_CTRL` reader - "]
pub struct ETHERNET_CLK_CTRL_R(crate::FieldReader<u8>);
impl ETHERNET_CLK_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ETHERNET_CLK_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETHERNET_CLK_CTRL_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Ethernet_CLK_CTRL` writer - "]
pub struct ETHERNET_CLK_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHERNET_CLK_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `AI_CLK_CTRL` reader - "]
pub struct AI_CLK_CTRL_R(crate::FieldReader<u8>);
impl AI_CLK_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AI_CLK_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AI_CLK_CTRL_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AI_CLK_CTRL` writer - "]
pub struct AI_CLK_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> AI_CLK_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `HPC_CLK_CTRL` reader - "]
pub struct HPC_CLK_CTRL_R(crate::FieldReader<u8>);
impl HPC_CLK_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HPC_CLK_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPC_CLK_CTRL_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPC_CLK_CTRL` writer - "]
pub struct HPC_CLK_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> HPC_CLK_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn tta_clk_ctrl(&self) -> TTA_CLK_CTRL_R {
        TTA_CLK_CTRL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn ethernet_clk_ctrl(&self) -> ETHERNET_CLK_CTRL_R {
        ETHERNET_CLK_CTRL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn ai_clk_ctrl(&self) -> AI_CLK_CTRL_R {
        AI_CLK_CTRL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn hpc_clk_ctrl(&self) -> HPC_CLK_CTRL_R {
        HPC_CLK_CTRL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn tta_clk_ctrl(&mut self) -> TTA_CLK_CTRL_W {
        TTA_CLK_CTRL_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn ethernet_clk_ctrl(&mut self) -> ETHERNET_CLK_CTRL_W {
        ETHERNET_CLK_CTRL_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn ai_clk_ctrl(&mut self) -> AI_CLK_CTRL_W {
        AI_CLK_CTRL_W { w: self }
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn hpc_clk_ctrl(&mut self) -> HPC_CLK_CTRL_W {
        HPC_CLK_CTRL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Subsystem Clock selection. Bit definition *_CLK_CTRL Bitfield definitions \\[bit\\]:\\[field\\]
0: sel_cka 1: force_cka 2: force_ckb 3: subsys_clkena 4: - 5: - 6: - 7: pll_ctrl_valid\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_ss_clk_ctrl1](index.html) module"]
pub struct REG_SS_CLK_CTRL1_SPEC;
impl crate::RegisterSpec for REG_SS_CLK_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_ss_clk_ctrl1::R](R) reader structure"]
impl crate::Readable for REG_SS_CLK_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg_ss_clk_ctrl1::W](W) writer structure"]
impl crate::Writable for REG_SS_CLK_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG_SS_CLK_CTRL1 to value 0"]
impl crate::Resettable for REG_SS_CLK_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
