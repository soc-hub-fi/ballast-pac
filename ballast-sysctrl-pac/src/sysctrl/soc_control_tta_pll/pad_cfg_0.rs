#[doc = "Register `PAD_CFG_0` reader"]
pub struct R(crate::R<PAD_CFG_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_CFG_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_CFG_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_CFG_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_CFG_0` writer"]
pub struct W(crate::W<PAD_CFG_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_CFG_0_SPEC>;
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
impl From<crate::W<PAD_CFG_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_CFG_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAD_0_drive_strength` reader - "]
pub struct PAD_0_DRIVE_STRENGTH_R(crate::FieldReader<u8>);
impl PAD_0_DRIVE_STRENGTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_0_DRIVE_STRENGTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_0_DRIVE_STRENGTH_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_0_drive_strength` writer - "]
pub struct PAD_0_DRIVE_STRENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_0_DRIVE_STRENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
#[doc = "Field `PAD_0_trigger` reader - "]
pub struct PAD_0_TRIGGER_R(crate::FieldReader<u8>);
impl PAD_0_TRIGGER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_0_TRIGGER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_0_TRIGGER_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_0_trigger` writer - "]
pub struct PAD_0_TRIGGER_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_0_TRIGGER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 2)) | ((value as u32 & 3) << 2);
        self.w
    }
}
#[doc = "Field `PAD_0_rate` reader - "]
pub struct PAD_0_RATE_R(crate::FieldReader<bool>);
impl PAD_0_RATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAD_0_RATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_0_RATE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_0_rate` writer - "]
pub struct PAD_0_RATE_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_0_RATE_W<'a> {
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
#[doc = "Field `PAD_0_output_en` reader - 0: Enable"]
pub struct PAD_0_OUTPUT_EN_R(crate::FieldReader<bool>);
impl PAD_0_OUTPUT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAD_0_OUTPUT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_0_OUTPUT_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_0_output_en` writer - 0: Enable"]
pub struct PAD_0_OUTPUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_0_OUTPUT_EN_W<'a> {
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
#[doc = "Field `PAD_0_hold` reader - "]
pub struct PAD_0_HOLD_R(crate::FieldReader<bool>);
impl PAD_0_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAD_0_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_0_HOLD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_0_hold` writer - "]
pub struct PAD_0_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_0_HOLD_W<'a> {
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
#[doc = "Field `PAD_0_pull_enable` reader - "]
pub struct PAD_0_PULL_ENABLE_R(crate::FieldReader<bool>);
impl PAD_0_PULL_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAD_0_PULL_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_0_PULL_ENABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_0_pull_enable` writer - "]
pub struct PAD_0_PULL_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_0_PULL_ENABLE_W<'a> {
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
#[doc = "Field `PAD_0_pd_pu` reader - Pull down or pull up register, default pull down 0: pull down 1: pull up"]
pub struct PAD_0_PD_PU_R(crate::FieldReader<bool>);
impl PAD_0_PD_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAD_0_PD_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_0_PD_PU_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_0_pd_pu` writer - Pull down or pull up register, default pull down 0: pull down 1: pull up"]
pub struct PAD_0_PD_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_0_PD_PU_W<'a> {
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
#[doc = "Field `PAD_0_input_en` reader - "]
pub struct PAD_0_INPUT_EN_R(crate::FieldReader<bool>);
impl PAD_0_INPUT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAD_0_INPUT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_0_INPUT_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_0_input_en` writer - "]
pub struct PAD_0_INPUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_0_INPUT_EN_W<'a> {
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
#[doc = "Field `PAD_1_drive_strength` reader - "]
pub struct PAD_1_DRIVE_STRENGTH_R(crate::FieldReader<u8>);
impl PAD_1_DRIVE_STRENGTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_1_DRIVE_STRENGTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_1_DRIVE_STRENGTH_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_1_drive_strength` writer - "]
pub struct PAD_1_DRIVE_STRENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_1_DRIVE_STRENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 10)) | ((value as u32 & 3) << 10);
        self.w
    }
}
#[doc = "Field `PAD_1_trigger` reader - "]
pub struct PAD_1_TRIGGER_R(crate::FieldReader<u8>);
impl PAD_1_TRIGGER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_1_TRIGGER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_1_TRIGGER_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_1_trigger` writer - "]
pub struct PAD_1_TRIGGER_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_1_TRIGGER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 12)) | ((value as u32 & 3) << 12);
        self.w
    }
}
#[doc = "Field `PAD_1_rate` reader - "]
pub struct PAD_1_RATE_R(crate::FieldReader<bool>);
impl PAD_1_RATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAD_1_RATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_1_RATE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_1_rate` writer - "]
pub struct PAD_1_RATE_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_1_RATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 14)) | ((value as u32 & 1) << 14);
        self.w
    }
}
#[doc = "Field `PAD_1_output_en` reader - 0: Enable"]
pub struct PAD_1_OUTPUT_EN_R(crate::FieldReader<bool>);
impl PAD_1_OUTPUT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAD_1_OUTPUT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_1_OUTPUT_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_1_output_en` writer - 0: Enable"]
pub struct PAD_1_OUTPUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_1_OUTPUT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u32 & 1) << 15);
        self.w
    }
}
#[doc = "Field `PAD_1_hold` reader - "]
pub struct PAD_1_HOLD_R(crate::FieldReader<bool>);
impl PAD_1_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAD_1_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_1_HOLD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_1_hold` writer - "]
pub struct PAD_1_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_1_HOLD_W<'a> {
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
#[doc = "Field `PAD_1_pull_enable` reader - "]
pub struct PAD_1_PULL_ENABLE_R(crate::FieldReader<bool>);
impl PAD_1_PULL_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAD_1_PULL_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_1_PULL_ENABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_1_pull_enable` writer - "]
pub struct PAD_1_PULL_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_1_PULL_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 17)) | ((value as u32 & 1) << 17);
        self.w
    }
}
#[doc = "Field `PAD_1_pd_pu` reader - Pull down or pull up register, default pull down 0: pull down 1: pull up"]
pub struct PAD_1_PD_PU_R(crate::FieldReader<bool>);
impl PAD_1_PD_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAD_1_PD_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_1_PD_PU_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_1_pd_pu` writer - Pull down or pull up register, default pull down 0: pull down 1: pull up"]
pub struct PAD_1_PD_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_1_PD_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 18)) | ((value as u32 & 1) << 18);
        self.w
    }
}
#[doc = "Field `PAD_1_input_en` reader - "]
pub struct PAD_1_INPUT_EN_R(crate::FieldReader<bool>);
impl PAD_1_INPUT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAD_1_INPUT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_1_INPUT_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_1_input_en` writer - "]
pub struct PAD_1_INPUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_1_INPUT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 19)) | ((value as u32 & 1) << 19);
        self.w
    }
}
#[doc = "Field `PAD_2_drive_strength` reader - "]
pub struct PAD_2_DRIVE_STRENGTH_R(crate::FieldReader<u8>);
impl PAD_2_DRIVE_STRENGTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_2_DRIVE_STRENGTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_2_DRIVE_STRENGTH_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_2_drive_strength` writer - "]
pub struct PAD_2_DRIVE_STRENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_2_DRIVE_STRENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 20)) | ((value as u32 & 3) << 20);
        self.w
    }
}
#[doc = "Field `PAD_2_trigger` reader - "]
pub struct PAD_2_TRIGGER_R(crate::FieldReader<u8>);
impl PAD_2_TRIGGER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_2_TRIGGER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_2_TRIGGER_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_2_trigger` writer - "]
pub struct PAD_2_TRIGGER_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_2_TRIGGER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 22)) | ((value as u32 & 3) << 22);
        self.w
    }
}
#[doc = "Field `PAD_2_rate` reader - "]
pub struct PAD_2_RATE_R(crate::FieldReader<bool>);
impl PAD_2_RATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAD_2_RATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_2_RATE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_2_rate` writer - "]
pub struct PAD_2_RATE_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_2_RATE_W<'a> {
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
#[doc = "Field `PAD_2_output_en` reader - 0: Enable"]
pub struct PAD_2_OUTPUT_EN_R(crate::FieldReader<bool>);
impl PAD_2_OUTPUT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAD_2_OUTPUT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_2_OUTPUT_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_2_output_en` writer - 0: Enable"]
pub struct PAD_2_OUTPUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_2_OUTPUT_EN_W<'a> {
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
#[doc = "Field `PAD_2_hold` reader - "]
pub struct PAD_2_HOLD_R(crate::FieldReader<bool>);
impl PAD_2_HOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAD_2_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_2_HOLD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_2_hold` writer - "]
pub struct PAD_2_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_2_HOLD_W<'a> {
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
#[doc = "Field `PAD_2_pull_enable` reader - "]
pub struct PAD_2_PULL_ENABLE_R(crate::FieldReader<bool>);
impl PAD_2_PULL_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAD_2_PULL_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_2_PULL_ENABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_2_pull_enable` writer - "]
pub struct PAD_2_PULL_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_2_PULL_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 27)) | ((value as u32 & 1) << 27);
        self.w
    }
}
#[doc = "Field `PAD_2_pd_pu` reader - Pull down or pull up register, default pull down 0: pull down 1: pull up"]
pub struct PAD_2_PD_PU_R(crate::FieldReader<bool>);
impl PAD_2_PD_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAD_2_PD_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_2_PD_PU_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_2_pd_pu` writer - Pull down or pull up register, default pull down 0: pull down 1: pull up"]
pub struct PAD_2_PD_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_2_PD_PU_W<'a> {
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
#[doc = "Field `PAD_2_input_en` reader - "]
pub struct PAD_2_INPUT_EN_R(crate::FieldReader<bool>);
impl PAD_2_INPUT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAD_2_INPUT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_2_INPUT_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_2_input_en` writer - "]
pub struct PAD_2_INPUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_2_INPUT_EN_W<'a> {
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
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pad_0_drive_strength(&self) -> PAD_0_DRIVE_STRENGTH_R {
        PAD_0_DRIVE_STRENGTH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn pad_0_trigger(&self) -> PAD_0_TRIGGER_R {
        PAD_0_TRIGGER_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pad_0_rate(&self) -> PAD_0_RATE_R {
        PAD_0_RATE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 0: Enable"]
    #[inline(always)]
    pub fn pad_0_output_en(&self) -> PAD_0_OUTPUT_EN_R {
        PAD_0_OUTPUT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pad_0_hold(&self) -> PAD_0_HOLD_R {
        PAD_0_HOLD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pad_0_pull_enable(&self) -> PAD_0_PULL_ENABLE_R {
        PAD_0_PULL_ENABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pull down or pull up register, default pull down 0: pull down 1: pull up"]
    #[inline(always)]
    pub fn pad_0_pd_pu(&self) -> PAD_0_PD_PU_R {
        PAD_0_PD_PU_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pad_0_input_en(&self) -> PAD_0_INPUT_EN_R {
        PAD_0_INPUT_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn pad_1_drive_strength(&self) -> PAD_1_DRIVE_STRENGTH_R {
        PAD_1_DRIVE_STRENGTH_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pad_1_trigger(&self) -> PAD_1_TRIGGER_R {
        PAD_1_TRIGGER_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pad_1_rate(&self) -> PAD_1_RATE_R {
        PAD_1_RATE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 0: Enable"]
    #[inline(always)]
    pub fn pad_1_output_en(&self) -> PAD_1_OUTPUT_EN_R {
        PAD_1_OUTPUT_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pad_1_hold(&self) -> PAD_1_HOLD_R {
        PAD_1_HOLD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pad_1_pull_enable(&self) -> PAD_1_PULL_ENABLE_R {
        PAD_1_PULL_ENABLE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Pull down or pull up register, default pull down 0: pull down 1: pull up"]
    #[inline(always)]
    pub fn pad_1_pd_pu(&self) -> PAD_1_PD_PU_R {
        PAD_1_PD_PU_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pad_1_input_en(&self) -> PAD_1_INPUT_EN_R {
        PAD_1_INPUT_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn pad_2_drive_strength(&self) -> PAD_2_DRIVE_STRENGTH_R {
        PAD_2_DRIVE_STRENGTH_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn pad_2_trigger(&self) -> PAD_2_TRIGGER_R {
        PAD_2_TRIGGER_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pad_2_rate(&self) -> PAD_2_RATE_R {
        PAD_2_RATE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 0: Enable"]
    #[inline(always)]
    pub fn pad_2_output_en(&self) -> PAD_2_OUTPUT_EN_R {
        PAD_2_OUTPUT_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn pad_2_hold(&self) -> PAD_2_HOLD_R {
        PAD_2_HOLD_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn pad_2_pull_enable(&self) -> PAD_2_PULL_ENABLE_R {
        PAD_2_PULL_ENABLE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Pull down or pull up register, default pull down 0: pull down 1: pull up"]
    #[inline(always)]
    pub fn pad_2_pd_pu(&self) -> PAD_2_PD_PU_R {
        PAD_2_PD_PU_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pad_2_input_en(&self) -> PAD_2_INPUT_EN_R {
        PAD_2_INPUT_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pad_0_drive_strength(&mut self) -> PAD_0_DRIVE_STRENGTH_W {
        PAD_0_DRIVE_STRENGTH_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn pad_0_trigger(&mut self) -> PAD_0_TRIGGER_W {
        PAD_0_TRIGGER_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pad_0_rate(&mut self) -> PAD_0_RATE_W {
        PAD_0_RATE_W { w: self }
    }
    #[doc = "Bit 5 - 0: Enable"]
    #[inline(always)]
    pub fn pad_0_output_en(&mut self) -> PAD_0_OUTPUT_EN_W {
        PAD_0_OUTPUT_EN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pad_0_hold(&mut self) -> PAD_0_HOLD_W {
        PAD_0_HOLD_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pad_0_pull_enable(&mut self) -> PAD_0_PULL_ENABLE_W {
        PAD_0_PULL_ENABLE_W { w: self }
    }
    #[doc = "Bit 8 - Pull down or pull up register, default pull down 0: pull down 1: pull up"]
    #[inline(always)]
    pub fn pad_0_pd_pu(&mut self) -> PAD_0_PD_PU_W {
        PAD_0_PD_PU_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pad_0_input_en(&mut self) -> PAD_0_INPUT_EN_W {
        PAD_0_INPUT_EN_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn pad_1_drive_strength(&mut self) -> PAD_1_DRIVE_STRENGTH_W {
        PAD_1_DRIVE_STRENGTH_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pad_1_trigger(&mut self) -> PAD_1_TRIGGER_W {
        PAD_1_TRIGGER_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pad_1_rate(&mut self) -> PAD_1_RATE_W {
        PAD_1_RATE_W { w: self }
    }
    #[doc = "Bit 15 - 0: Enable"]
    #[inline(always)]
    pub fn pad_1_output_en(&mut self) -> PAD_1_OUTPUT_EN_W {
        PAD_1_OUTPUT_EN_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pad_1_hold(&mut self) -> PAD_1_HOLD_W {
        PAD_1_HOLD_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pad_1_pull_enable(&mut self) -> PAD_1_PULL_ENABLE_W {
        PAD_1_PULL_ENABLE_W { w: self }
    }
    #[doc = "Bit 18 - Pull down or pull up register, default pull down 0: pull down 1: pull up"]
    #[inline(always)]
    pub fn pad_1_pd_pu(&mut self) -> PAD_1_PD_PU_W {
        PAD_1_PD_PU_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pad_1_input_en(&mut self) -> PAD_1_INPUT_EN_W {
        PAD_1_INPUT_EN_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn pad_2_drive_strength(&mut self) -> PAD_2_DRIVE_STRENGTH_W {
        PAD_2_DRIVE_STRENGTH_W { w: self }
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn pad_2_trigger(&mut self) -> PAD_2_TRIGGER_W {
        PAD_2_TRIGGER_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pad_2_rate(&mut self) -> PAD_2_RATE_W {
        PAD_2_RATE_W { w: self }
    }
    #[doc = "Bit 25 - 0: Enable"]
    #[inline(always)]
    pub fn pad_2_output_en(&mut self) -> PAD_2_OUTPUT_EN_W {
        PAD_2_OUTPUT_EN_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn pad_2_hold(&mut self) -> PAD_2_HOLD_W {
        PAD_2_HOLD_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn pad_2_pull_enable(&mut self) -> PAD_2_PULL_ENABLE_W {
        PAD_2_PULL_ENABLE_W { w: self }
    }
    #[doc = "Bit 28 - Pull down or pull up register, default pull down 0: pull down 1: pull up"]
    #[inline(always)]
    pub fn pad_2_pd_pu(&mut self) -> PAD_2_PD_PU_W {
        PAD_2_PD_PU_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pad_2_input_en(&mut self) -> PAD_2_INPUT_EN_W {
        PAD_2_INPUT_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "All 10 bit fields have reset value of 10'b10_0011_0100: 0 drive strenght 1 drive strenght 2 trigger 3 trigger 4 rate 5 output en(0) 6 hold 7 pull enable 8 pd(0)/pu(1) 9 input en(1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_cfg_0](index.html) module"]
pub struct PAD_CFG_0_SPEC;
impl crate::RegisterSpec for PAD_CFG_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad_cfg_0::R](R) reader structure"]
impl crate::Readable for PAD_CFG_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_cfg_0::W](W) writer structure"]
impl crate::Writable for PAD_CFG_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAD_CFG_0 to value 0x8c01_2003"]
impl crate::Resettable for PAD_CFG_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8c01_2003
    }
}
