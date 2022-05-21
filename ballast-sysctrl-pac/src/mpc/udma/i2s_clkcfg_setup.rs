#[doc = "Register `I2S_CLKCFG_SETUP` reader"]
pub struct R(crate::R<I2S_CLKCFG_SETUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_CLKCFG_SETUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_CLKCFG_SETUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_CLKCFG_SETUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2S_CLKCFG_SETUP` writer"]
pub struct W(crate::W<I2S_CLKCFG_SETUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_CLKCFG_SETUP_SPEC>;
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
impl From<crate::W<I2S_CLKCFG_SETUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_CLKCFG_SETUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASTER_CLK_DIV` reader - LSB of master clock divider"]
pub struct MASTER_CLK_DIV_R(crate::FieldReader<u8, u8>);
impl MASTER_CLK_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MASTER_CLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASTER_CLK_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASTER_CLK_DIV` writer - LSB of master clock divider"]
pub struct MASTER_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `SLAVE_CLK_DIV` reader - LSB of slave clock divider"]
pub struct SLAVE_CLK_DIV_R(crate::FieldReader<u8, u8>);
impl SLAVE_CLK_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLAVE_CLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_CLK_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_CLK_DIV` writer - LSB of slave clock divider"]
pub struct SLAVE_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `COMMON_CLK_DIV` reader - MSBs of both master and slave clock divider"]
pub struct COMMON_CLK_DIV_R(crate::FieldReader<u8, u8>);
impl COMMON_CLK_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        COMMON_CLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMMON_CLK_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMMON_CLK_DIV` writer - MSBs of both master and slave clock divider"]
pub struct COMMON_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMON_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `SLAVE_CLK_EN` reader - Enables Slave clock"]
pub struct SLAVE_CLK_EN_R(crate::FieldReader<bool, bool>);
impl SLAVE_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_CLK_EN` writer - Enables Slave clock"]
pub struct SLAVE_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_CLK_EN_W<'a> {
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
#[doc = "Field `MASTER_CLK_EN` reader - Enables Master clock"]
pub struct MASTER_CLK_EN_R(crate::FieldReader<bool, bool>);
impl MASTER_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MASTER_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASTER_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASTER_CLK_EN` writer - Enables Master clock"]
pub struct MASTER_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 25)) | ((value as u32 & 1) << 25);
        self.w
    }
}
#[doc = "Field `PDM_CLK_EN` reader - When enabled slave output clock is taken from PDM module."]
pub struct PDM_CLK_EN_R(crate::FieldReader<bool, bool>);
impl PDM_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDM_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDM_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDM_CLK_EN` writer - When enabled slave output clock is taken from PDM module."]
pub struct PDM_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDM_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 26)) | ((value as u32 & 1) << 26);
        self.w
    }
}
#[doc = "Field `SLAVE_EXT` reader - When set uses external clock for slave"]
pub struct SLAVE_EXT_R(crate::FieldReader<bool, bool>);
impl SLAVE_EXT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_EXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_EXT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_EXT` writer - When set uses external clock for slave"]
pub struct SLAVE_EXT_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_EXT_W<'a> {
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
#[doc = "Field `SLAVE_NUM` reader - Selects slave clock source(either ext or generated): -1'b0:selects master -1'b1:selects slave"]
pub struct SLAVE_NUM_R(crate::FieldReader<bool, bool>);
impl SLAVE_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_NUM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_NUM` writer - Selects slave clock source(either ext or generated): -1'b0:selects master -1'b1:selects slave"]
pub struct SLAVE_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_NUM_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 29)) | ((value as u32 & 1) << 29);
        self.w
    }
}
#[doc = "Field `MASTER_EXT` reader - When set uses external clock for master"]
pub struct MASTER_EXT_R(crate::FieldReader<bool, bool>);
impl MASTER_EXT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MASTER_EXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASTER_EXT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASTER_EXT` writer - When set uses external clock for master"]
pub struct MASTER_EXT_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_EXT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 30)) | ((value as u32 & 1) << 30);
        self.w
    }
}
#[doc = "Field `MASTER_NUM` reader - Selects master clock source(either ext or generated): -1'b0:selects master -1'b1:selects slave"]
pub struct MASTER_NUM_R(crate::FieldReader<bool, bool>);
impl MASTER_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MASTER_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASTER_NUM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASTER_NUM` writer - Selects master clock source(either ext or generated): -1'b0:selects master -1'b1:selects slave"]
pub struct MASTER_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_NUM_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - LSB of master clock divider"]
    #[inline(always)]
    pub fn master_clk_div(&self) -> MASTER_CLK_DIV_R {
        MASTER_CLK_DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - LSB of slave clock divider"]
    #[inline(always)]
    pub fn slave_clk_div(&self) -> SLAVE_CLK_DIV_R {
        SLAVE_CLK_DIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - MSBs of both master and slave clock divider"]
    #[inline(always)]
    pub fn common_clk_div(&self) -> COMMON_CLK_DIV_R {
        COMMON_CLK_DIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Enables Slave clock"]
    #[inline(always)]
    pub fn slave_clk_en(&self) -> SLAVE_CLK_EN_R {
        SLAVE_CLK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enables Master clock"]
    #[inline(always)]
    pub fn master_clk_en(&self) -> MASTER_CLK_EN_R {
        MASTER_CLK_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - When enabled slave output clock is taken from PDM module."]
    #[inline(always)]
    pub fn pdm_clk_en(&self) -> PDM_CLK_EN_R {
        PDM_CLK_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - When set uses external clock for slave"]
    #[inline(always)]
    pub fn slave_ext(&self) -> SLAVE_EXT_R {
        SLAVE_EXT_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Selects slave clock source(either ext or generated): -1'b0:selects master -1'b1:selects slave"]
    #[inline(always)]
    pub fn slave_num(&self) -> SLAVE_NUM_R {
        SLAVE_NUM_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - When set uses external clock for master"]
    #[inline(always)]
    pub fn master_ext(&self) -> MASTER_EXT_R {
        MASTER_EXT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Selects master clock source(either ext or generated): -1'b0:selects master -1'b1:selects slave"]
    #[inline(always)]
    pub fn master_num(&self) -> MASTER_NUM_R {
        MASTER_NUM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - LSB of master clock divider"]
    #[inline(always)]
    pub fn master_clk_div(&mut self) -> MASTER_CLK_DIV_W {
        MASTER_CLK_DIV_W { w: self }
    }
    #[doc = "Bits 8:15 - LSB of slave clock divider"]
    #[inline(always)]
    pub fn slave_clk_div(&mut self) -> SLAVE_CLK_DIV_W {
        SLAVE_CLK_DIV_W { w: self }
    }
    #[doc = "Bits 16:23 - MSBs of both master and slave clock divider"]
    #[inline(always)]
    pub fn common_clk_div(&mut self) -> COMMON_CLK_DIV_W {
        COMMON_CLK_DIV_W { w: self }
    }
    #[doc = "Bit 24 - Enables Slave clock"]
    #[inline(always)]
    pub fn slave_clk_en(&mut self) -> SLAVE_CLK_EN_W {
        SLAVE_CLK_EN_W { w: self }
    }
    #[doc = "Bit 25 - Enables Master clock"]
    #[inline(always)]
    pub fn master_clk_en(&mut self) -> MASTER_CLK_EN_W {
        MASTER_CLK_EN_W { w: self }
    }
    #[doc = "Bit 26 - When enabled slave output clock is taken from PDM module."]
    #[inline(always)]
    pub fn pdm_clk_en(&mut self) -> PDM_CLK_EN_W {
        PDM_CLK_EN_W { w: self }
    }
    #[doc = "Bit 28 - When set uses external clock for slave"]
    #[inline(always)]
    pub fn slave_ext(&mut self) -> SLAVE_EXT_W {
        SLAVE_EXT_W { w: self }
    }
    #[doc = "Bit 29 - Selects slave clock source(either ext or generated): -1'b0:selects master -1'b1:selects slave"]
    #[inline(always)]
    pub fn slave_num(&mut self) -> SLAVE_NUM_W {
        SLAVE_NUM_W { w: self }
    }
    #[doc = "Bit 30 - When set uses external clock for master"]
    #[inline(always)]
    pub fn master_ext(&mut self) -> MASTER_EXT_W {
        MASTER_EXT_W { w: self }
    }
    #[doc = "Bit 31 - Selects master clock source(either ext or generated): -1'b0:selects master -1'b1:selects slave"]
    #[inline(always)]
    pub fn master_num(&mut self) -> MASTER_NUM_W {
        MASTER_NUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock configuration for both master, slave and pdm\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_clkcfg_setup](index.html) module"]
pub struct I2S_CLKCFG_SETUP_SPEC;
impl crate::RegisterSpec for I2S_CLKCFG_SETUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_clkcfg_setup::R](R) reader structure"]
impl crate::Readable for I2S_CLKCFG_SETUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_clkcfg_setup::W](W) writer structure"]
impl crate::Writable for I2S_CLKCFG_SETUP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2S_CLKCFG_SETUP to value 0"]
impl crate::Resettable for I2S_CLKCFG_SETUP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
