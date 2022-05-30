#[doc = "Register `CTRL_CFG_CG` reader"]
pub struct R(crate::R<CTRL_CFG_CG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_CFG_CG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_CFG_CG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_CFG_CG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL_CFG_CG` writer"]
pub struct W(crate::W<CTRL_CFG_CG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_CFG_CG_SPEC>;
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
impl From<crate::W<CTRL_CFG_CG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_CFG_CG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CG_UART` reader - "]
pub struct CG_UART_R(crate::FieldReader<bool>);
impl CG_UART_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CG_UART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CG_UART_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CG_UART` writer - "]
pub struct CG_UART_W<'a> {
    w: &'a mut W,
}
impl<'a> CG_UART_W<'a> {
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
#[doc = "Field `CG_SPIM` reader - "]
pub struct CG_SPIM_R(crate::FieldReader<bool>);
impl CG_SPIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CG_SPIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CG_SPIM_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CG_SPIM` writer - "]
pub struct CG_SPIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CG_SPIM_W<'a> {
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
#[doc = "Field `CG_I2C0` reader - "]
pub struct CG_I2C0_R(crate::FieldReader<bool>);
impl CG_I2C0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CG_I2C0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CG_I2C0_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CG_I2C0` writer - "]
pub struct CG_I2C0_W<'a> {
    w: &'a mut W,
}
impl<'a> CG_I2C0_W<'a> {
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
#[doc = "Field `CG_I2C1` reader - "]
pub struct CG_I2C1_R(crate::FieldReader<bool>);
impl CG_I2C1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CG_I2C1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CG_I2C1_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CG_I2C1` writer - "]
pub struct CG_I2C1_W<'a> {
    w: &'a mut W,
}
impl<'a> CG_I2C1_W<'a> {
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
#[doc = "Field `CG_SDIO` reader - "]
pub struct CG_SDIO_R(crate::FieldReader<bool>);
impl CG_SDIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CG_SDIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CG_SDIO_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CG_SDIO` writer - "]
pub struct CG_SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> CG_SDIO_W<'a> {
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
#[doc = "Field `CG_I2S` reader - "]
pub struct CG_I2S_R(crate::FieldReader<bool>);
impl CG_I2S_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CG_I2S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CG_I2S_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CG_I2S` writer - "]
pub struct CG_I2S_W<'a> {
    w: &'a mut W,
}
impl<'a> CG_I2S_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Field `CG_CAM` reader - "]
pub struct CG_CAM_R(crate::FieldReader<bool>);
impl CG_CAM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CG_CAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CG_CAM_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CG_CAM` writer - "]
pub struct CG_CAM_W<'a> {
    w: &'a mut W,
}
impl<'a> CG_CAM_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Field `CG_FILTER` reader - "]
pub struct CG_FILTER_R(crate::FieldReader<bool>);
impl CG_FILTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CG_FILTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CG_FILTER_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CG_FILTER` writer - "]
pub struct CG_FILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> CG_FILTER_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cg_uart(&self) -> CG_UART_R {
        CG_UART_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cg_spim(&self) -> CG_SPIM_R {
        CG_SPIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cg_i2c0(&self) -> CG_I2C0_R {
        CG_I2C0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cg_i2c1(&self) -> CG_I2C1_R {
        CG_I2C1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cg_sdio(&self) -> CG_SDIO_R {
        CG_SDIO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cg_i2s(&self) -> CG_I2S_R {
        CG_I2S_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cg_cam(&self) -> CG_CAM_R {
        CG_CAM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cg_filter(&self) -> CG_FILTER_R {
        CG_FILTER_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cg_uart(&mut self) -> CG_UART_W {
        CG_UART_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cg_spim(&mut self) -> CG_SPIM_W {
        CG_SPIM_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cg_i2c0(&mut self) -> CG_I2C0_W {
        CG_I2C0_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cg_i2c1(&mut self) -> CG_I2C1_W {
        CG_I2C1_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cg_sdio(&mut self) -> CG_SDIO_W {
        CG_SDIO_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cg_i2s(&mut self) -> CG_I2S_W {
        CG_I2S_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cg_cam(&mut self) -> CG_CAM_W {
        CG_CAM_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cg_filter(&mut self) -> CG_FILTER_W {
        CG_FILTER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uDMA peripherals clock gate configuration Bitfields Defines uDMA peripherals clock gate configuration for corresponding peripheral\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_cfg_cg](index.html) module"]
pub struct CTRL_CFG_CG_SPEC;
impl crate::RegisterSpec for CTRL_CFG_CG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl_cfg_cg::R](R) reader structure"]
impl crate::Readable for CTRL_CFG_CG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl_cfg_cg::W](W) writer structure"]
impl crate::Writable for CTRL_CFG_CG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL_CFG_CG to value 0"]
impl crate::Resettable for CTRL_CFG_CG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
