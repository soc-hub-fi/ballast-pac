#[doc = "Register `CAM_CFG_GLOB` reader"]
pub struct R(crate::R<CAM_CFG_GLOB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAM_CFG_GLOB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAM_CFG_GLOB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAM_CFG_GLOB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAM_CFG_GLOB` writer"]
pub struct W(crate::W<CAM_CFG_GLOB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAM_CFG_GLOB_SPEC>;
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
impl From<crate::W<CAM_CFG_GLOB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAM_CFG_GLOB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAMEDROP_EN` reader - Frame dropping:"]
pub struct FRAMEDROP_EN_R(crate::FieldReader<bool>);
impl FRAMEDROP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRAMEDROP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAMEDROP_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRAMEDROP_EN` writer - Frame dropping:"]
pub struct FRAMEDROP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAMEDROP_EN_W<'a> {
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
#[doc = "Field `FRAMEDROP_VAL` reader - ) Sets how many frames should be dropped after each received"]
pub struct FRAMEDROP_VAL_R(crate::FieldReader<u8>);
impl FRAMEDROP_VAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FRAMEDROP_VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAMEDROP_VAL_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRAMEDROP_VAL` writer - ) Sets how many frames should be dropped after each received"]
pub struct FRAMEDROP_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAMEDROP_VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 1)) | ((value as u32 & 0x3f) << 1);
        self.w
    }
}
#[doc = "Field `FRAMESLICE_EN` reader - nput frame slicing: - 1'b0: disable - 1'b1: enabl"]
pub struct FRAMESLICE_EN_R(crate::FieldReader<bool>);
impl FRAMESLICE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRAMESLICE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAMESLICE_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRAMESLICE_EN` writer - nput frame slicing: - 1'b0: disable - 1'b1: enabl"]
pub struct FRAMESLICE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAMESLICE_EN_W<'a> {
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
#[doc = "Input frame format: - 3'b000: RGB565 - 3'b001: RGB555 - 3'b010: RGB444 - 3'b100: BYPASS_LITEND - 3'b101: BYPASS_BIGEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FORMAT_A {
    #[doc = "0: `0`"]
    RGB565 = 0,
    #[doc = "1: `1`"]
    RGB555 = 1,
    #[doc = "2: `10`"]
    RGB444 = 2,
    #[doc = "4: `100`"]
    BYPASS_LITEND = 4,
    #[doc = "5: `101`"]
    BYPASS_BIGEND = 5,
}
impl From<FORMAT_A> for u8 {
    #[inline(always)]
    fn from(variant: FORMAT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FORMAT` reader - Input frame format: - 3'b000: RGB565 - 3'b001: RGB555 - 3'b010: RGB444 - 3'b100: BYPASS_LITEND - 3'b101: BYPASS_BIGEND"]
pub struct FORMAT_R(crate::FieldReader<u8>);
impl FORMAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FORMAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FORMAT_A> {
        match self.bits {
            0 => Some(FORMAT_A::RGB565),
            1 => Some(FORMAT_A::RGB555),
            2 => Some(FORMAT_A::RGB444),
            4 => Some(FORMAT_A::BYPASS_LITEND),
            5 => Some(FORMAT_A::BYPASS_BIGEND),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RGB565`"]
    #[inline(always)]
    pub fn is_rgb565(&self) -> bool {
        **self == FORMAT_A::RGB565
    }
    #[doc = "Checks if the value of the field is `RGB555`"]
    #[inline(always)]
    pub fn is_rgb555(&self) -> bool {
        **self == FORMAT_A::RGB555
    }
    #[doc = "Checks if the value of the field is `RGB444`"]
    #[inline(always)]
    pub fn is_rgb444(&self) -> bool {
        **self == FORMAT_A::RGB444
    }
    #[doc = "Checks if the value of the field is `BYPASS_LITEND`"]
    #[inline(always)]
    pub fn is_bypass_litend(&self) -> bool {
        **self == FORMAT_A::BYPASS_LITEND
    }
    #[doc = "Checks if the value of the field is `BYPASS_BIGEND`"]
    #[inline(always)]
    pub fn is_bypass_bigend(&self) -> bool {
        **self == FORMAT_A::BYPASS_BIGEND
    }
}
impl core::ops::Deref for FORMAT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORMAT` writer - Input frame format: - 3'b000: RGB565 - 3'b001: RGB555 - 3'b010: RGB444 - 3'b100: BYPASS_LITEND - 3'b101: BYPASS_BIGEND"]
pub struct FORMAT_W<'a> {
    w: &'a mut W,
}
impl<'a> FORMAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORMAT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rgb565(self) -> &'a mut W {
        self.variant(FORMAT_A::RGB565)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn rgb555(self) -> &'a mut W {
        self.variant(FORMAT_A::RGB555)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn rgb444(self) -> &'a mut W {
        self.variant(FORMAT_A::RGB444)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn bypass_litend(self) -> &'a mut W {
        self.variant(FORMAT_A::BYPASS_LITEND)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn bypass_bigend(self) -> &'a mut W {
        self.variant(FORMAT_A::BYPASS_BIGEND)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 8)) | ((value as u32 & 7) << 8);
        self.w
    }
}
#[doc = "Field `SHIFT` reader - Right shift of final pixel value (DivFactor) NOTE: not used if FORMAT == BYPASS"]
pub struct SHIFT_R(crate::FieldReader<u8>);
impl SHIFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SHIFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHIFT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHIFT` writer - Right shift of final pixel value (DivFactor) NOTE: not used if FORMAT == BYPASS"]
pub struct SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 11)) | ((value as u32 & 0x0f) << 11);
        self.w
    }
}
#[doc = "Field `EN` reader - Enable data rx from camera interface. The enable/disable happens only at the start of a frame. - 1'b0: disable - 1'b1: enable"]
pub struct EN_R(crate::FieldReader<bool>);
impl EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - Enable data rx from camera interface. The enable/disable happens only at the start of a frame. - 1'b0: disable - 1'b1: enable"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
    #[doc = "Bit 0 - Frame dropping:"]
    #[inline(always)]
    pub fn framedrop_en(&self) -> FRAMEDROP_EN_R {
        FRAMEDROP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:6 - ) Sets how many frames should be dropped after each received"]
    #[inline(always)]
    pub fn framedrop_val(&self) -> FRAMEDROP_VAL_R {
        FRAMEDROP_VAL_R::new(((self.bits >> 1) & 0x3f) as u8)
    }
    #[doc = "Bit 7 - nput frame slicing: - 1'b0: disable - 1'b1: enabl"]
    #[inline(always)]
    pub fn frameslice_en(&self) -> FRAMESLICE_EN_R {
        FRAMESLICE_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Input frame format: - 3'b000: RGB565 - 3'b001: RGB555 - 3'b010: RGB444 - 3'b100: BYPASS_LITEND - 3'b101: BYPASS_BIGEND"]
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:14 - Right shift of final pixel value (DivFactor) NOTE: not used if FORMAT == BYPASS"]
    #[inline(always)]
    pub fn shift(&self) -> SHIFT_R {
        SHIFT_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Enable data rx from camera interface. The enable/disable happens only at the start of a frame. - 1'b0: disable - 1'b1: enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Frame dropping:"]
    #[inline(always)]
    pub fn framedrop_en(&mut self) -> FRAMEDROP_EN_W {
        FRAMEDROP_EN_W { w: self }
    }
    #[doc = "Bits 1:6 - ) Sets how many frames should be dropped after each received"]
    #[inline(always)]
    pub fn framedrop_val(&mut self) -> FRAMEDROP_VAL_W {
        FRAMEDROP_VAL_W { w: self }
    }
    #[doc = "Bit 7 - nput frame slicing: - 1'b0: disable - 1'b1: enabl"]
    #[inline(always)]
    pub fn frameslice_en(&mut self) -> FRAMESLICE_EN_W {
        FRAMESLICE_EN_W { w: self }
    }
    #[doc = "Bits 8:10 - Input frame format: - 3'b000: RGB565 - 3'b001: RGB555 - 3'b010: RGB444 - 3'b100: BYPASS_LITEND - 3'b101: BYPASS_BIGEND"]
    #[inline(always)]
    pub fn format(&mut self) -> FORMAT_W {
        FORMAT_W { w: self }
    }
    #[doc = "Bits 11:14 - Right shift of final pixel value (DivFactor) NOTE: not used if FORMAT == BYPASS"]
    #[inline(always)]
    pub fn shift(&mut self) -> SHIFT_W {
        SHIFT_W { w: self }
    }
    #[doc = "Bit 31 - Enable data rx from camera interface. The enable/disable happens only at the start of a frame. - 1'b0: disable - 1'b1: enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cam_cfg_glob](index.html) module"]
pub struct CAM_CFG_GLOB_SPEC;
impl crate::RegisterSpec for CAM_CFG_GLOB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cam_cfg_glob::R](R) reader structure"]
impl crate::Readable for CAM_CFG_GLOB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cam_cfg_glob::W](W) writer structure"]
impl crate::Writable for CAM_CFG_GLOB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAM_CFG_GLOB to value 0"]
impl crate::Resettable for CAM_CFG_GLOB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
