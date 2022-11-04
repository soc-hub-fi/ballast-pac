#[doc = "Register `c2c_rx_vld` reader"]
pub struct R(crate::R<C2C_RX_VLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2C_RX_VLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2C_RX_VLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2C_RX_VLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `c2c_rx_vld` writer"]
pub struct W(crate::W<C2C_RX_VLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2C_RX_VLD_SPEC>;
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
impl From<crate::W<C2C_RX_VLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2C_RX_VLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `drive_strength` reader - "]
pub struct DRIVE_STRENGTH_R(crate::FieldReader<u8>);
impl DRIVE_STRENGTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DRIVE_STRENGTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRIVE_STRENGTH_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `drive_strength` writer - "]
pub struct DRIVE_STRENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_STRENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u16 & 3);
        self.w
    }
}
#[doc = "Field `output_enable` reader - "]
pub struct OUTPUT_ENABLE_R(crate::FieldReader<bool>);
impl OUTPUT_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTPUT_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTPUT_ENABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `output_enable` writer - "]
pub struct OUTPUT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTPUT_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u16 & 1) << 5);
        self.w
    }
}
#[doc = "Field `pull_enable` reader - "]
pub struct PULL_ENABLE_R(crate::FieldReader<bool>);
impl PULL_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PULL_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_ENABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pull_enable` writer - "]
pub struct PULL_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u16 & 1) << 7);
        self.w
    }
}
#[doc = "Field `pull_down_up` reader - Pull down 0 Pull up 1"]
pub struct PULL_DOWN_UP_R(crate::FieldReader<bool>);
impl PULL_DOWN_UP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PULL_DOWN_UP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_DOWN_UP_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pull_down_up` writer - Pull down 0 Pull up 1"]
pub struct PULL_DOWN_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_DOWN_UP_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u16 & 1) << 8);
        self.w
    }
}
#[doc = "Field `input_enable` reader - "]
pub struct INPUT_ENABLE_R(crate::FieldReader<bool>);
impl INPUT_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INPUT_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INPUT_ENABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `input_enable` writer - "]
pub struct INPUT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u16 & 1) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn drive_strength(&self) -> DRIVE_STRENGTH_R {
        DRIVE_STRENGTH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn output_enable(&self) -> OUTPUT_ENABLE_R {
        OUTPUT_ENABLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pull_enable(&self) -> PULL_ENABLE_R {
        PULL_ENABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pull down 0 Pull up 1"]
    #[inline(always)]
    pub fn pull_down_up(&self) -> PULL_DOWN_UP_R {
        PULL_DOWN_UP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn input_enable(&self) -> INPUT_ENABLE_R {
        INPUT_ENABLE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn drive_strength(&mut self) -> DRIVE_STRENGTH_W {
        DRIVE_STRENGTH_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn output_enable(&mut self) -> OUTPUT_ENABLE_W {
        OUTPUT_ENABLE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pull_enable(&mut self) -> PULL_ENABLE_W {
        PULL_ENABLE_W { w: self }
    }
    #[doc = "Bit 8 - Pull down 0 Pull up 1"]
    #[inline(always)]
    pub fn pull_down_up(&mut self) -> PULL_DOWN_UP_W {
        PULL_DOWN_UP_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn input_enable(&mut self) -> INPUT_ENABLE_W {
        INPUT_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2c_rx_vld](index.html) module"]
pub struct C2C_RX_VLD_SPEC;
impl crate::RegisterSpec for C2C_RX_VLD_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [c2c_rx_vld::R](R) reader structure"]
impl crate::Readable for C2C_RX_VLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2c_rx_vld::W](W) writer structure"]
impl crate::Writable for C2C_RX_VLD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets c2c_rx_vld to value 0x8008"]
impl crate::Resettable for C2C_RX_VLD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8008
    }
}
