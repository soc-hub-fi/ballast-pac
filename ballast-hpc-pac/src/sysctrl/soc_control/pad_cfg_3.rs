#[doc = "Register `PAD_CFG_3` reader"]
pub struct R(crate::R<PAD_CFG_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_CFG_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_CFG_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_CFG_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_CFG_3` writer"]
pub struct W(crate::W<PAD_CFG_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_CFG_3_SPEC>;
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
impl From<crate::W<PAD_CFG_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_CFG_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAD_9_drive_strength` reader - "]
pub type PAD_9_DRIVE_STRENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAD_9_drive_strength` writer - "]
pub type PAD_9_DRIVE_STRENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PAD_CFG_3_SPEC, u8, u8, 2, O>;
#[doc = "Field `PAD_9_trigger` reader - "]
pub type PAD_9_TRIGGER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAD_9_trigger` writer - "]
pub type PAD_9_TRIGGER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PAD_CFG_3_SPEC, u8, u8, 2, O>;
#[doc = "Field `PAD_9_rate` reader - "]
pub type PAD_9_RATE_R = crate::BitReader<bool>;
#[doc = "Field `PAD_9_rate` writer - "]
pub type PAD_9_RATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_CFG_3_SPEC, bool, O>;
#[doc = "Field `PAD_9_output_en` reader - 0: Enable"]
pub type PAD_9_OUTPUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `PAD_9_output_en` writer - 0: Enable"]
pub type PAD_9_OUTPUT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_CFG_3_SPEC, bool, O>;
#[doc = "Field `PAD_9_hold` reader - "]
pub type PAD_9_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `PAD_9_hold` writer - "]
pub type PAD_9_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_CFG_3_SPEC, bool, O>;
#[doc = "Field `PAD_9_pull_enable` reader - "]
pub type PAD_9_PULL_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `PAD_9_pull_enable` writer - "]
pub type PAD_9_PULL_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_CFG_3_SPEC, bool, O>;
#[doc = "Field `PAD_9_pd_pu` reader - Pull down or pull up register, default pull down 0: pull down 1: pull up"]
pub type PAD_9_PD_PU_R = crate::BitReader<bool>;
#[doc = "Field `PAD_9_pd_pu` writer - Pull down or pull up register, default pull down 0: pull down 1: pull up"]
pub type PAD_9_PD_PU_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_CFG_3_SPEC, bool, O>;
#[doc = "Field `PAD_9_input_en` reader - "]
pub type PAD_9_INPUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `PAD_9_input_en` writer - "]
pub type PAD_9_INPUT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_CFG_3_SPEC, bool, O>;
#[doc = "Field `PAD_10_drive_strength` reader - "]
pub type PAD_10_DRIVE_STRENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAD_10_drive_strength` writer - "]
pub type PAD_10_DRIVE_STRENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PAD_CFG_3_SPEC, u8, u8, 2, O>;
#[doc = "Field `PAD_10_trigger` reader - "]
pub type PAD_10_TRIGGER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAD_10_trigger` writer - "]
pub type PAD_10_TRIGGER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PAD_CFG_3_SPEC, u8, u8, 2, O>;
#[doc = "Field `PAD_10_rate` reader - "]
pub type PAD_10_RATE_R = crate::BitReader<bool>;
#[doc = "Field `PAD_10_rate` writer - "]
pub type PAD_10_RATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_CFG_3_SPEC, bool, O>;
#[doc = "Field `PAD_10_output_en` reader - 0: Enable"]
pub type PAD_10_OUTPUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `PAD_10_output_en` writer - 0: Enable"]
pub type PAD_10_OUTPUT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_CFG_3_SPEC, bool, O>;
#[doc = "Field `PAD_10_hold` reader - "]
pub type PAD_10_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `PAD_10_hold` writer - "]
pub type PAD_10_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_CFG_3_SPEC, bool, O>;
#[doc = "Field `PAD_10_pull_enable` reader - "]
pub type PAD_10_PULL_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `PAD_10_pull_enable` writer - "]
pub type PAD_10_PULL_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_CFG_3_SPEC, bool, O>;
#[doc = "Field `PAD_10_pd_pu` reader - Pull down or pull up register, default pull down 0: pull down 1: pull up"]
pub type PAD_10_PD_PU_R = crate::BitReader<bool>;
#[doc = "Field `PAD_10_pd_pu` writer - Pull down or pull up register, default pull down 0: pull down 1: pull up"]
pub type PAD_10_PD_PU_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_CFG_3_SPEC, bool, O>;
#[doc = "Field `PAD_10_input_en` reader - "]
pub type PAD_10_INPUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `PAD_10_input_en` writer - "]
pub type PAD_10_INPUT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_CFG_3_SPEC, bool, O>;
#[doc = "Field `PAD_11_drive_strength` reader - "]
pub type PAD_11_DRIVE_STRENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAD_11_drive_strength` writer - "]
pub type PAD_11_DRIVE_STRENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PAD_CFG_3_SPEC, u8, u8, 2, O>;
#[doc = "Field `PAD_11_trigger` reader - "]
pub type PAD_11_TRIGGER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAD_11_trigger` writer - "]
pub type PAD_11_TRIGGER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PAD_CFG_3_SPEC, u8, u8, 2, O>;
#[doc = "Field `PAD_11_rate` reader - "]
pub type PAD_11_RATE_R = crate::BitReader<bool>;
#[doc = "Field `PAD_11_rate` writer - "]
pub type PAD_11_RATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_CFG_3_SPEC, bool, O>;
#[doc = "Field `PAD_11_output_en` reader - 0: Enable"]
pub type PAD_11_OUTPUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `PAD_11_output_en` writer - 0: Enable"]
pub type PAD_11_OUTPUT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_CFG_3_SPEC, bool, O>;
#[doc = "Field `PAD_11_hold` reader - "]
pub type PAD_11_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `PAD_11_hold` writer - "]
pub type PAD_11_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_CFG_3_SPEC, bool, O>;
#[doc = "Field `PAD_11_pull_enable` reader - "]
pub type PAD_11_PULL_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `PAD_11_pull_enable` writer - "]
pub type PAD_11_PULL_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_CFG_3_SPEC, bool, O>;
#[doc = "Field `PAD_11_pd_pu` reader - Pull down or pull up register, default pull down 0: pull down 1: pull up"]
pub type PAD_11_PD_PU_R = crate::BitReader<bool>;
#[doc = "Field `PAD_11_pd_pu` writer - Pull down or pull up register, default pull down 0: pull down 1: pull up"]
pub type PAD_11_PD_PU_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_CFG_3_SPEC, bool, O>;
#[doc = "Field `PAD_11_input_en` reader - "]
pub type PAD_11_INPUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `PAD_11_input_en` writer - "]
pub type PAD_11_INPUT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_CFG_3_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pad_9_drive_strength(&self) -> PAD_9_DRIVE_STRENGTH_R {
        PAD_9_DRIVE_STRENGTH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn pad_9_trigger(&self) -> PAD_9_TRIGGER_R {
        PAD_9_TRIGGER_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pad_9_rate(&self) -> PAD_9_RATE_R {
        PAD_9_RATE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 0: Enable"]
    #[inline(always)]
    pub fn pad_9_output_en(&self) -> PAD_9_OUTPUT_EN_R {
        PAD_9_OUTPUT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pad_9_hold(&self) -> PAD_9_HOLD_R {
        PAD_9_HOLD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pad_9_pull_enable(&self) -> PAD_9_PULL_ENABLE_R {
        PAD_9_PULL_ENABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pull down or pull up register, default pull down 0: pull down 1: pull up"]
    #[inline(always)]
    pub fn pad_9_pd_pu(&self) -> PAD_9_PD_PU_R {
        PAD_9_PD_PU_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pad_9_input_en(&self) -> PAD_9_INPUT_EN_R {
        PAD_9_INPUT_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn pad_10_drive_strength(&self) -> PAD_10_DRIVE_STRENGTH_R {
        PAD_10_DRIVE_STRENGTH_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pad_10_trigger(&self) -> PAD_10_TRIGGER_R {
        PAD_10_TRIGGER_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pad_10_rate(&self) -> PAD_10_RATE_R {
        PAD_10_RATE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 0: Enable"]
    #[inline(always)]
    pub fn pad_10_output_en(&self) -> PAD_10_OUTPUT_EN_R {
        PAD_10_OUTPUT_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pad_10_hold(&self) -> PAD_10_HOLD_R {
        PAD_10_HOLD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pad_10_pull_enable(&self) -> PAD_10_PULL_ENABLE_R {
        PAD_10_PULL_ENABLE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Pull down or pull up register, default pull down 0: pull down 1: pull up"]
    #[inline(always)]
    pub fn pad_10_pd_pu(&self) -> PAD_10_PD_PU_R {
        PAD_10_PD_PU_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pad_10_input_en(&self) -> PAD_10_INPUT_EN_R {
        PAD_10_INPUT_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn pad_11_drive_strength(&self) -> PAD_11_DRIVE_STRENGTH_R {
        PAD_11_DRIVE_STRENGTH_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn pad_11_trigger(&self) -> PAD_11_TRIGGER_R {
        PAD_11_TRIGGER_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pad_11_rate(&self) -> PAD_11_RATE_R {
        PAD_11_RATE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 0: Enable"]
    #[inline(always)]
    pub fn pad_11_output_en(&self) -> PAD_11_OUTPUT_EN_R {
        PAD_11_OUTPUT_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn pad_11_hold(&self) -> PAD_11_HOLD_R {
        PAD_11_HOLD_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn pad_11_pull_enable(&self) -> PAD_11_PULL_ENABLE_R {
        PAD_11_PULL_ENABLE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Pull down or pull up register, default pull down 0: pull down 1: pull up"]
    #[inline(always)]
    pub fn pad_11_pd_pu(&self) -> PAD_11_PD_PU_R {
        PAD_11_PD_PU_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pad_11_input_en(&self) -> PAD_11_INPUT_EN_R {
        PAD_11_INPUT_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn pad_9_drive_strength(&mut self) -> PAD_9_DRIVE_STRENGTH_W<0> {
        PAD_9_DRIVE_STRENGTH_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn pad_9_trigger(&mut self) -> PAD_9_TRIGGER_W<2> {
        PAD_9_TRIGGER_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pad_9_rate(&mut self) -> PAD_9_RATE_W<4> {
        PAD_9_RATE_W::new(self)
    }
    #[doc = "Bit 5 - 0: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pad_9_output_en(&mut self) -> PAD_9_OUTPUT_EN_W<5> {
        PAD_9_OUTPUT_EN_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn pad_9_hold(&mut self) -> PAD_9_HOLD_W<6> {
        PAD_9_HOLD_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pad_9_pull_enable(&mut self) -> PAD_9_PULL_ENABLE_W<7> {
        PAD_9_PULL_ENABLE_W::new(self)
    }
    #[doc = "Bit 8 - Pull down or pull up register, default pull down 0: pull down 1: pull up"]
    #[inline(always)]
    #[must_use]
    pub fn pad_9_pd_pu(&mut self) -> PAD_9_PD_PU_W<8> {
        PAD_9_PD_PU_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pad_9_input_en(&mut self) -> PAD_9_INPUT_EN_W<9> {
        PAD_9_INPUT_EN_W::new(self)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    #[must_use]
    pub fn pad_10_drive_strength(&mut self) -> PAD_10_DRIVE_STRENGTH_W<10> {
        PAD_10_DRIVE_STRENGTH_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn pad_10_trigger(&mut self) -> PAD_10_TRIGGER_W<12> {
        PAD_10_TRIGGER_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn pad_10_rate(&mut self) -> PAD_10_RATE_W<14> {
        PAD_10_RATE_W::new(self)
    }
    #[doc = "Bit 15 - 0: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pad_10_output_en(&mut self) -> PAD_10_OUTPUT_EN_W<15> {
        PAD_10_OUTPUT_EN_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pad_10_hold(&mut self) -> PAD_10_HOLD_W<16> {
        PAD_10_HOLD_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn pad_10_pull_enable(&mut self) -> PAD_10_PULL_ENABLE_W<17> {
        PAD_10_PULL_ENABLE_W::new(self)
    }
    #[doc = "Bit 18 - Pull down or pull up register, default pull down 0: pull down 1: pull up"]
    #[inline(always)]
    #[must_use]
    pub fn pad_10_pd_pu(&mut self) -> PAD_10_PD_PU_W<18> {
        PAD_10_PD_PU_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn pad_10_input_en(&mut self) -> PAD_10_INPUT_EN_W<19> {
        PAD_10_INPUT_EN_W::new(self)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn pad_11_drive_strength(&mut self) -> PAD_11_DRIVE_STRENGTH_W<20> {
        PAD_11_DRIVE_STRENGTH_W::new(self)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    #[must_use]
    pub fn pad_11_trigger(&mut self) -> PAD_11_TRIGGER_W<22> {
        PAD_11_TRIGGER_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn pad_11_rate(&mut self) -> PAD_11_RATE_W<24> {
        PAD_11_RATE_W::new(self)
    }
    #[doc = "Bit 25 - 0: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pad_11_output_en(&mut self) -> PAD_11_OUTPUT_EN_W<25> {
        PAD_11_OUTPUT_EN_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn pad_11_hold(&mut self) -> PAD_11_HOLD_W<26> {
        PAD_11_HOLD_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn pad_11_pull_enable(&mut self) -> PAD_11_PULL_ENABLE_W<27> {
        PAD_11_PULL_ENABLE_W::new(self)
    }
    #[doc = "Bit 28 - Pull down or pull up register, default pull down 0: pull down 1: pull up"]
    #[inline(always)]
    #[must_use]
    pub fn pad_11_pd_pu(&mut self) -> PAD_11_PD_PU_W<28> {
        PAD_11_PD_PU_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn pad_11_input_en(&mut self) -> PAD_11_INPUT_EN_W<29> {
        PAD_11_INPUT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_cfg_3](index.html) module"]
pub struct PAD_CFG_3_SPEC;
impl crate::RegisterSpec for PAD_CFG_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad_cfg_3::R](R) reader structure"]
impl crate::Readable for PAD_CFG_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_cfg_3::W](W) writer structure"]
impl crate::Writable for PAD_CFG_3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAD_CFG_3 to value 0x8c01_2003"]
impl crate::Resettable for PAD_CFG_3_SPEC {
    const RESET_VALUE: Self::Ux = 0x8c01_2003;
}
