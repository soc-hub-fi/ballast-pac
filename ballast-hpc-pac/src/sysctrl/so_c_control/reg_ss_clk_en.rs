#[doc = "Register `REG_SS_CLK_EN` reader"]
pub struct R(crate::R<REG_SS_CLK_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_SS_CLK_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_SS_CLK_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_SS_CLK_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG_SS_CLK_EN` writer"]
pub struct W(crate::W<REG_SS_CLK_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_SS_CLK_EN_SPEC>;
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
impl From<crate::W<REG_SS_CLK_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_SS_CLK_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pulpissimo` reader - Pulpissimo subsystem clock enable"]
pub struct PULPISSIMO_R(crate::FieldReader<bool>);
impl PULPISSIMO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PULPISSIMO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULPISSIMO_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pulpissimo` writer - Pulpissimo subsystem clock enable"]
pub struct PULPISSIMO_W<'a> {
    w: &'a mut W,
}
impl<'a> PULPISSIMO_W<'a> {
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
#[doc = "Field `interconnect` reader - interconnect subsystem clock enable"]
pub struct INTERCONNECT_R(crate::FieldReader<bool>);
impl INTERCONNECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTERCONNECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERCONNECT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `interconnect` writer - interconnect subsystem clock enable"]
pub struct INTERCONNECT_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERCONNECT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `top_peripheral` reader - Top Peripheral subsystem clock enable"]
pub struct TOP_PERIPHERAL_R(crate::FieldReader<bool>);
impl TOP_PERIPHERAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOP_PERIPHERAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOP_PERIPHERAL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `top_peripheral` writer - Top Peripheral subsystem clock enable"]
pub struct TOP_PERIPHERAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP_PERIPHERAL_W<'a> {
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
#[doc = "Field `c2c` reader - C2C clock enable"]
pub struct C2C_R(crate::FieldReader<bool>);
impl C2C_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        C2C_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C2C_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `c2c` writer - C2C clock enable"]
pub struct C2C_W<'a> {
    w: &'a mut W,
}
impl<'a> C2C_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
#[doc = "Field `coreHW` reader - coreHW subsystem clock enable"]
pub struct COREHW_R(crate::FieldReader<bool>);
impl COREHW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COREHW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COREHW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `coreHW` writer - coreHW subsystem clock enable"]
pub struct COREHW_W<'a> {
    w: &'a mut W,
}
impl<'a> COREHW_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 12)) | ((value as u32 & 1) << 12);
        self.w
    }
}
#[doc = "Field `TTA` reader - TTA subsystem clock enable"]
pub struct TTA_R(crate::FieldReader<bool>);
impl TTA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TTA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TTA_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TTA` writer - TTA subsystem clock enable"]
pub struct TTA_W<'a> {
    w: &'a mut W,
}
impl<'a> TTA_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
#[doc = "Field `ethernet` reader - Ethernet subsystem clock enable"]
pub struct ETHERNET_R(crate::FieldReader<bool>);
impl ETHERNET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ETHERNET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETHERNET_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ethernet` writer - Ethernet subsystem clock enable"]
pub struct ETHERNET_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHERNET_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 20)) | ((value as u32 & 1) << 20);
        self.w
    }
}
#[doc = "Field `AI` reader - AI subsystem clock enable"]
pub struct AI_R(crate::FieldReader<bool>);
impl AI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AI_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AI` writer - AI subsystem clock enable"]
pub struct AI_W<'a> {
    w: &'a mut W,
}
impl<'a> AI_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 24)) | ((value as u32 & 1) << 24);
        self.w
    }
}
#[doc = "Field `HPC` reader - HPC subsystem clock enable"]
pub struct HPC_R(crate::FieldReader<bool>);
impl HPC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HPC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPC_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPC` writer - HPC subsystem clock enable"]
pub struct HPC_W<'a> {
    w: &'a mut W,
}
impl<'a> HPC_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 28)) | ((value as u32 & 1) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Pulpissimo subsystem clock enable"]
    #[inline(always)]
    pub fn pulpissimo(&self) -> PULPISSIMO_R {
        PULPISSIMO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - interconnect subsystem clock enable"]
    #[inline(always)]
    pub fn interconnect(&self) -> INTERCONNECT_R {
        INTERCONNECT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Top Peripheral subsystem clock enable"]
    #[inline(always)]
    pub fn top_peripheral(&self) -> TOP_PERIPHERAL_R {
        TOP_PERIPHERAL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - C2C clock enable"]
    #[inline(always)]
    pub fn c2c(&self) -> C2C_R {
        C2C_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - coreHW subsystem clock enable"]
    #[inline(always)]
    pub fn core_hw(&self) -> COREHW_R {
        COREHW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - TTA subsystem clock enable"]
    #[inline(always)]
    pub fn tta(&self) -> TTA_R {
        TTA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Ethernet subsystem clock enable"]
    #[inline(always)]
    pub fn ethernet(&self) -> ETHERNET_R {
        ETHERNET_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - AI subsystem clock enable"]
    #[inline(always)]
    pub fn ai(&self) -> AI_R {
        AI_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - HPC subsystem clock enable"]
    #[inline(always)]
    pub fn hpc(&self) -> HPC_R {
        HPC_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pulpissimo subsystem clock enable"]
    #[inline(always)]
    pub fn pulpissimo(&mut self) -> PULPISSIMO_W {
        PULPISSIMO_W { w: self }
    }
    #[doc = "Bit 4 - interconnect subsystem clock enable"]
    #[inline(always)]
    pub fn interconnect(&mut self) -> INTERCONNECT_W {
        INTERCONNECT_W { w: self }
    }
    #[doc = "Bit 7 - Top Peripheral subsystem clock enable"]
    #[inline(always)]
    pub fn top_peripheral(&mut self) -> TOP_PERIPHERAL_W {
        TOP_PERIPHERAL_W { w: self }
    }
    #[doc = "Bit 8 - C2C clock enable"]
    #[inline(always)]
    pub fn c2c(&mut self) -> C2C_W {
        C2C_W { w: self }
    }
    #[doc = "Bit 12 - coreHW subsystem clock enable"]
    #[inline(always)]
    pub fn core_hw(&mut self) -> COREHW_W {
        COREHW_W { w: self }
    }
    #[doc = "Bit 16 - TTA subsystem clock enable"]
    #[inline(always)]
    pub fn tta(&mut self) -> TTA_W {
        TTA_W { w: self }
    }
    #[doc = "Bit 20 - Ethernet subsystem clock enable"]
    #[inline(always)]
    pub fn ethernet(&mut self) -> ETHERNET_W {
        ETHERNET_W { w: self }
    }
    #[doc = "Bit 24 - AI subsystem clock enable"]
    #[inline(always)]
    pub fn ai(&mut self) -> AI_W {
        AI_W { w: self }
    }
    #[doc = "Bit 28 - HPC subsystem clock enable"]
    #[inline(always)]
    pub fn hpc(&mut self) -> HPC_W {
        HPC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Subsystem clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_ss_clk_en](index.html) module"]
pub struct REG_SS_CLK_EN_SPEC;
impl crate::RegisterSpec for REG_SS_CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_ss_clk_en::R](R) reader structure"]
impl crate::Readable for REG_SS_CLK_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg_ss_clk_en::W](W) writer structure"]
impl crate::Writable for REG_SS_CLK_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG_SS_CLK_EN to value 0"]
impl crate::Resettable for REG_SS_CLK_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
