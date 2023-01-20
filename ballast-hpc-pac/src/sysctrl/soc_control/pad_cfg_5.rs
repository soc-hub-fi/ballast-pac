#[doc = "Register `PAD_CFG_5` reader"]
pub struct R(crate::R<PAD_CFG_5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_CFG_5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_CFG_5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_CFG_5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_CFG_5` writer"]
pub struct W(crate::W<PAD_CFG_5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_CFG_5_SPEC>;
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
impl From<crate::W<PAD_CFG_5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_CFG_5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAD_15_drive_strength` reader - "]
pub type PAD_15_DRIVE_STRENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAD_15_drive_strength` writer - "]
pub type PAD_15_DRIVE_STRENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PAD_CFG_5_SPEC, u8, u8, 2, O>;
#[doc = "Field `PAD_15_trigger` reader - "]
pub type PAD_15_TRIGGER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAD_15_trigger` writer - "]
pub type PAD_15_TRIGGER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PAD_CFG_5_SPEC, u8, u8, 2, O>;
#[doc = "Field `PAD_15_rate` reader - "]
pub type PAD_15_RATE_R = crate::BitReader<bool>;
#[doc = "Field `PAD_15_rate` writer - "]
pub type PAD_15_RATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_CFG_5_SPEC, bool, O>;
#[doc = "Field `PAD_15_output_en` reader - 0: Enable"]
pub type PAD_15_OUTPUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `PAD_15_output_en` writer - 0: Enable"]
pub type PAD_15_OUTPUT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_CFG_5_SPEC, bool, O>;
#[doc = "Field `PAD_15_hold` reader - "]
pub type PAD_15_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `PAD_15_hold` writer - "]
pub type PAD_15_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_CFG_5_SPEC, bool, O>;
#[doc = "Field `PAD_15_pull_enable` reader - "]
pub type PAD_15_PULL_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `PAD_15_pull_enable` writer - "]
pub type PAD_15_PULL_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_CFG_5_SPEC, bool, O>;
#[doc = "Field `PAD_15_pd_pu` reader - Pull down or pull up register, default pull down 0: pull down 1: pull up"]
pub type PAD_15_PD_PU_R = crate::BitReader<bool>;
#[doc = "Field `PAD_15_pd_pu` writer - Pull down or pull up register, default pull down 0: pull down 1: pull up"]
pub type PAD_15_PD_PU_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_CFG_5_SPEC, bool, O>;
#[doc = "Field `PAD_15_input_en` reader - "]
pub type PAD_15_INPUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `PAD_15_input_en` writer - "]
pub type PAD_15_INPUT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_CFG_5_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pad_15_drive_strength(&self) -> PAD_15_DRIVE_STRENGTH_R {
        PAD_15_DRIVE_STRENGTH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn pad_15_trigger(&self) -> PAD_15_TRIGGER_R {
        PAD_15_TRIGGER_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pad_15_rate(&self) -> PAD_15_RATE_R {
        PAD_15_RATE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 0: Enable"]
    #[inline(always)]
    pub fn pad_15_output_en(&self) -> PAD_15_OUTPUT_EN_R {
        PAD_15_OUTPUT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pad_15_hold(&self) -> PAD_15_HOLD_R {
        PAD_15_HOLD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pad_15_pull_enable(&self) -> PAD_15_PULL_ENABLE_R {
        PAD_15_PULL_ENABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pull down or pull up register, default pull down 0: pull down 1: pull up"]
    #[inline(always)]
    pub fn pad_15_pd_pu(&self) -> PAD_15_PD_PU_R {
        PAD_15_PD_PU_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pad_15_input_en(&self) -> PAD_15_INPUT_EN_R {
        PAD_15_INPUT_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn pad_15_drive_strength(&mut self) -> PAD_15_DRIVE_STRENGTH_W<0> {
        PAD_15_DRIVE_STRENGTH_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn pad_15_trigger(&mut self) -> PAD_15_TRIGGER_W<2> {
        PAD_15_TRIGGER_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pad_15_rate(&mut self) -> PAD_15_RATE_W<4> {
        PAD_15_RATE_W::new(self)
    }
    #[doc = "Bit 5 - 0: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pad_15_output_en(&mut self) -> PAD_15_OUTPUT_EN_W<5> {
        PAD_15_OUTPUT_EN_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn pad_15_hold(&mut self) -> PAD_15_HOLD_W<6> {
        PAD_15_HOLD_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pad_15_pull_enable(&mut self) -> PAD_15_PULL_ENABLE_W<7> {
        PAD_15_PULL_ENABLE_W::new(self)
    }
    #[doc = "Bit 8 - Pull down or pull up register, default pull down 0: pull down 1: pull up"]
    #[inline(always)]
    #[must_use]
    pub fn pad_15_pd_pu(&mut self) -> PAD_15_PD_PU_W<8> {
        PAD_15_PD_PU_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pad_15_input_en(&mut self) -> PAD_15_INPUT_EN_W<9> {
        PAD_15_INPUT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_cfg_5](index.html) module"]
pub struct PAD_CFG_5_SPEC;
impl crate::RegisterSpec for PAD_CFG_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad_cfg_5::R](R) reader structure"]
impl crate::Readable for PAD_CFG_5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_cfg_5::W](W) writer structure"]
impl crate::Writable for PAD_CFG_5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAD_CFG_5 to value 0x0002_0034"]
impl crate::Resettable for PAD_CFG_5_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0034;
}
