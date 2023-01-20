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
pub type CG_UART_R = crate::BitReader<bool>;
#[doc = "Field `CG_UART` writer - "]
pub type CG_UART_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_CFG_CG_SPEC, bool, O>;
#[doc = "Field `CG_SPIM` reader - "]
pub type CG_SPIM_R = crate::BitReader<bool>;
#[doc = "Field `CG_SPIM` writer - "]
pub type CG_SPIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_CFG_CG_SPEC, bool, O>;
#[doc = "Field `CG_I2C0` reader - "]
pub type CG_I2C0_R = crate::BitReader<bool>;
#[doc = "Field `CG_I2C0` writer - "]
pub type CG_I2C0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_CFG_CG_SPEC, bool, O>;
#[doc = "Field `CG_I2C1` reader - "]
pub type CG_I2C1_R = crate::BitReader<bool>;
#[doc = "Field `CG_I2C1` writer - "]
pub type CG_I2C1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_CFG_CG_SPEC, bool, O>;
#[doc = "Field `CG_SDIO` reader - "]
pub type CG_SDIO_R = crate::BitReader<bool>;
#[doc = "Field `CG_SDIO` writer - "]
pub type CG_SDIO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_CFG_CG_SPEC, bool, O>;
#[doc = "Field `CG_I2S` reader - "]
pub type CG_I2S_R = crate::BitReader<bool>;
#[doc = "Field `CG_I2S` writer - "]
pub type CG_I2S_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_CFG_CG_SPEC, bool, O>;
#[doc = "Field `CG_CAM` reader - "]
pub type CG_CAM_R = crate::BitReader<bool>;
#[doc = "Field `CG_CAM` writer - "]
pub type CG_CAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_CFG_CG_SPEC, bool, O>;
#[doc = "Field `CG_FILTER` reader - "]
pub type CG_FILTER_R = crate::BitReader<bool>;
#[doc = "Field `CG_FILTER` writer - "]
pub type CG_FILTER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_CFG_CG_SPEC, bool, O>;
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
    #[must_use]
    pub fn cg_uart(&mut self) -> CG_UART_W<0> {
        CG_UART_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cg_spim(&mut self) -> CG_SPIM_W<1> {
        CG_SPIM_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cg_i2c0(&mut self) -> CG_I2C0_W<2> {
        CG_I2C0_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cg_i2c1(&mut self) -> CG_I2C1_W<3> {
        CG_I2C1_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cg_sdio(&mut self) -> CG_SDIO_W<4> {
        CG_SDIO_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn cg_i2s(&mut self) -> CG_I2S_W<5> {
        CG_I2S_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn cg_cam(&mut self) -> CG_CAM_W<6> {
        CG_CAM_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn cg_filter(&mut self) -> CG_FILTER_W<7> {
        CG_FILTER_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL_CFG_CG to value 0"]
impl crate::Resettable for CTRL_CFG_CG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
