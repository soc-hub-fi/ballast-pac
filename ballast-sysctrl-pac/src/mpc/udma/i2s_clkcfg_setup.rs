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
pub type MASTER_CLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASTER_CLK_DIV` writer - LSB of master clock divider"]
pub type MASTER_CLK_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_CLKCFG_SETUP_SPEC, u8, u8, 8, O>;
#[doc = "Field `SLAVE_CLK_DIV` reader - LSB of slave clock divider"]
pub type SLAVE_CLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLAVE_CLK_DIV` writer - LSB of slave clock divider"]
pub type SLAVE_CLK_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_CLKCFG_SETUP_SPEC, u8, u8, 8, O>;
#[doc = "Field `COMMON_CLK_DIV` reader - MSBs of both master and slave clock divider"]
pub type COMMON_CLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMMON_CLK_DIV` writer - MSBs of both master and slave clock divider"]
pub type COMMON_CLK_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_CLKCFG_SETUP_SPEC, u8, u8, 8, O>;
#[doc = "Field `SLAVE_CLK_EN` reader - Enables Slave clock"]
pub type SLAVE_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `SLAVE_CLK_EN` writer - Enables Slave clock"]
pub type SLAVE_CLK_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, I2S_CLKCFG_SETUP_SPEC, bool, O>;
#[doc = "Field `MASTER_CLK_EN` reader - Enables Master clock"]
pub type MASTER_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `MASTER_CLK_EN` writer - Enables Master clock"]
pub type MASTER_CLK_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, I2S_CLKCFG_SETUP_SPEC, bool, O>;
#[doc = "Field `PDM_CLK_EN` reader - When enabled slave output clock is taken from PDM module."]
pub type PDM_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `PDM_CLK_EN` writer - When enabled slave output clock is taken from PDM module."]
pub type PDM_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_CLKCFG_SETUP_SPEC, bool, O>;
#[doc = "Field `SLAVE_EXT` reader - When set uses external clock for slave"]
pub type SLAVE_EXT_R = crate::BitReader<bool>;
#[doc = "Field `SLAVE_EXT` writer - When set uses external clock for slave"]
pub type SLAVE_EXT_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_CLKCFG_SETUP_SPEC, bool, O>;
#[doc = "Field `SLAVE_NUM` reader - Selects slave clock source(either ext or generated): -1'b0:selects master -1'b1:selects slave"]
pub type SLAVE_NUM_R = crate::BitReader<bool>;
#[doc = "Field `SLAVE_NUM` writer - Selects slave clock source(either ext or generated): -1'b0:selects master -1'b1:selects slave"]
pub type SLAVE_NUM_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_CLKCFG_SETUP_SPEC, bool, O>;
#[doc = "Field `MASTER_EXT` reader - When set uses external clock for master"]
pub type MASTER_EXT_R = crate::BitReader<bool>;
#[doc = "Field `MASTER_EXT` writer - When set uses external clock for master"]
pub type MASTER_EXT_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_CLKCFG_SETUP_SPEC, bool, O>;
#[doc = "Field `MASTER_NUM` reader - Selects master clock source(either ext or generated): -1'b0:selects master -1'b1:selects slave"]
pub type MASTER_NUM_R = crate::BitReader<bool>;
#[doc = "Field `MASTER_NUM` writer - Selects master clock source(either ext or generated): -1'b0:selects master -1'b1:selects slave"]
pub type MASTER_NUM_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_CLKCFG_SETUP_SPEC, bool, O>;
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
    #[must_use]
    pub fn master_clk_div(&mut self) -> MASTER_CLK_DIV_W<0> {
        MASTER_CLK_DIV_W::new(self)
    }
    #[doc = "Bits 8:15 - LSB of slave clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn slave_clk_div(&mut self) -> SLAVE_CLK_DIV_W<8> {
        SLAVE_CLK_DIV_W::new(self)
    }
    #[doc = "Bits 16:23 - MSBs of both master and slave clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn common_clk_div(&mut self) -> COMMON_CLK_DIV_W<16> {
        COMMON_CLK_DIV_W::new(self)
    }
    #[doc = "Bit 24 - Enables Slave clock"]
    #[inline(always)]
    #[must_use]
    pub fn slave_clk_en(&mut self) -> SLAVE_CLK_EN_W<24> {
        SLAVE_CLK_EN_W::new(self)
    }
    #[doc = "Bit 25 - Enables Master clock"]
    #[inline(always)]
    #[must_use]
    pub fn master_clk_en(&mut self) -> MASTER_CLK_EN_W<25> {
        MASTER_CLK_EN_W::new(self)
    }
    #[doc = "Bit 26 - When enabled slave output clock is taken from PDM module."]
    #[inline(always)]
    #[must_use]
    pub fn pdm_clk_en(&mut self) -> PDM_CLK_EN_W<26> {
        PDM_CLK_EN_W::new(self)
    }
    #[doc = "Bit 28 - When set uses external clock for slave"]
    #[inline(always)]
    #[must_use]
    pub fn slave_ext(&mut self) -> SLAVE_EXT_W<28> {
        SLAVE_EXT_W::new(self)
    }
    #[doc = "Bit 29 - Selects slave clock source(either ext or generated): -1'b0:selects master -1'b1:selects slave"]
    #[inline(always)]
    #[must_use]
    pub fn slave_num(&mut self) -> SLAVE_NUM_W<29> {
        SLAVE_NUM_W::new(self)
    }
    #[doc = "Bit 30 - When set uses external clock for master"]
    #[inline(always)]
    #[must_use]
    pub fn master_ext(&mut self) -> MASTER_EXT_W<30> {
        MASTER_EXT_W::new(self)
    }
    #[doc = "Bit 31 - Selects master clock source(either ext or generated): -1'b0:selects master -1'b1:selects slave"]
    #[inline(always)]
    #[must_use]
    pub fn master_num(&mut self) -> MASTER_NUM_W<31> {
        MASTER_NUM_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2S_CLKCFG_SETUP to value 0"]
impl crate::Resettable for I2S_CLKCFG_SETUP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
