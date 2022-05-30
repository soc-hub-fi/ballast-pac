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
pub struct PAD_15_DRIVE_STRENGTH_R(crate::FieldReader<u8>);
impl PAD_15_DRIVE_STRENGTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_15_DRIVE_STRENGTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_15_DRIVE_STRENGTH_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_15_drive_strength` writer - "]
pub struct PAD_15_DRIVE_STRENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_15_DRIVE_STRENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
#[doc = "Field `PAD_15_trigger` reader - "]
pub struct PAD_15_TRIGGER_R(crate::FieldReader<u8>);
impl PAD_15_TRIGGER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_15_TRIGGER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_15_TRIGGER_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_15_trigger` writer - "]
pub struct PAD_15_TRIGGER_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_15_TRIGGER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 2)) | ((value as u32 & 3) << 2);
        self.w
    }
}
#[doc = "Field `PAD_15_rate` reader - "]
pub struct PAD_15_RATE_R(crate::FieldReader<bool>);
impl PAD_15_RATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAD_15_RATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_15_RATE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_15_rate` writer - "]
pub struct PAD_15_RATE_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_15_RATE_W<'a> {
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
#[doc = "Field `PAD_15_output_en` reader - 0: Enable"]
pub struct PAD_15_OUTPUT_EN_R(crate::FieldReader<bool>);
impl PAD_15_OUTPUT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAD_15_OUTPUT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_15_OUTPUT_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_15_output_en` writer - 0: Enable"]
pub struct PAD_15_OUTPUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_15_OUTPUT_EN_W<'a> {
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
#[doc = "Field `PAD_15_hold` reader - "]
pub struct PAD_15_HOLD_R(crate::FieldReader<bool>);
impl PAD_15_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAD_15_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_15_HOLD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_15_hold` writer - "]
pub struct PAD_15_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_15_HOLD_W<'a> {
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
#[doc = "Field `PAD_15_pull_enable` reader - "]
pub struct PAD_15_PULL_ENABLE_R(crate::FieldReader<bool>);
impl PAD_15_PULL_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAD_15_PULL_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_15_PULL_ENABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_15_pull_enable` writer - "]
pub struct PAD_15_PULL_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_15_PULL_ENABLE_W<'a> {
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
#[doc = "Field `PAD_15_pd_pu` reader - Pull down or pull up register, default pull down 0: pull down 1: pull up"]
pub struct PAD_15_PD_PU_R(crate::FieldReader<bool>);
impl PAD_15_PD_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAD_15_PD_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_15_PD_PU_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_15_pd_pu` writer - Pull down or pull up register, default pull down 0: pull down 1: pull up"]
pub struct PAD_15_PD_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_15_PD_PU_W<'a> {
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
#[doc = "Field `PAD_15_input_en` reader - "]
pub struct PAD_15_INPUT_EN_R(crate::FieldReader<bool>);
impl PAD_15_INPUT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAD_15_INPUT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_15_INPUT_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_15_input_en` writer - "]
pub struct PAD_15_INPUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_15_INPUT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u32 & 1) << 9);
        self.w
    }
}
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
    pub fn pad_15_drive_strength(&mut self) -> PAD_15_DRIVE_STRENGTH_W {
        PAD_15_DRIVE_STRENGTH_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn pad_15_trigger(&mut self) -> PAD_15_TRIGGER_W {
        PAD_15_TRIGGER_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pad_15_rate(&mut self) -> PAD_15_RATE_W {
        PAD_15_RATE_W { w: self }
    }
    #[doc = "Bit 5 - 0: Enable"]
    #[inline(always)]
    pub fn pad_15_output_en(&mut self) -> PAD_15_OUTPUT_EN_W {
        PAD_15_OUTPUT_EN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pad_15_hold(&mut self) -> PAD_15_HOLD_W {
        PAD_15_HOLD_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pad_15_pull_enable(&mut self) -> PAD_15_PULL_ENABLE_W {
        PAD_15_PULL_ENABLE_W { w: self }
    }
    #[doc = "Bit 8 - Pull down or pull up register, default pull down 0: pull down 1: pull up"]
    #[inline(always)]
    pub fn pad_15_pd_pu(&mut self) -> PAD_15_PD_PU_W {
        PAD_15_PD_PU_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pad_15_input_en(&mut self) -> PAD_15_INPUT_EN_W {
        PAD_15_INPUT_EN_W { w: self }
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
}
#[doc = "`reset()` method sets PAD_CFG_5 to value 0x0002_0034"]
impl crate::Resettable for PAD_CFG_5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0002_0034
    }
}
