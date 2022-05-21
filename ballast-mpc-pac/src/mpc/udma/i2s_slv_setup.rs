#[doc = "Register `I2S_SLV_SETUP` reader"]
pub struct R(crate::R<I2S_SLV_SETUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_SLV_SETUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_SLV_SETUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_SLV_SETUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2S_SLV_SETUP` writer"]
pub struct W(crate::W<I2S_SLV_SETUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_SLV_SETUP_SPEC>;
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
impl From<crate::W<I2S_SLV_SETUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_SLV_SETUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLAVE_WORDS` reader - Sets how many words for each I2S phase"]
pub struct SLAVE_WORDS_R(crate::FieldReader<u8, u8>);
impl SLAVE_WORDS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLAVE_WORDS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_WORDS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_WORDS` writer - Sets how many words for each I2S phase"]
pub struct SLAVE_WORDS_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_WORDS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !7) | (value as u32 & 7);
        self.w
    }
}
#[doc = "Field `SLAVE_BITS` reader - Sets how many bits per word"]
pub struct SLAVE_BITS_R(crate::FieldReader<u8, u8>);
impl SLAVE_BITS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLAVE_BITS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_BITS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_BITS` writer - Sets how many bits per word"]
pub struct SLAVE_BITS_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_BITS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `SLAVE_LSB` reader - Enables LSB shifting"]
pub struct SLAVE_LSB_R(crate::FieldReader<bool, bool>);
impl SLAVE_LSB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_LSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_LSB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_LSB` writer - Enables LSB shifting"]
pub struct SLAVE_LSB_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_LSB_W<'a> {
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
#[doc = "Field `SLAVE_2CH` reader - Enables both channels"]
pub struct SLAVE_2CH_R(crate::FieldReader<bool, bool>);
impl SLAVE_2CH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_2CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_2CH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_2CH` writer - Enables both channels"]
pub struct SLAVE_2CH_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_2CH_W<'a> {
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
#[doc = "Field `SLAVE_EN` reader - Enables the Slave"]
pub struct SLAVE_EN_R(crate::FieldReader<bool, bool>);
impl SLAVE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_EN` writer - Enables the Slave"]
pub struct SLAVE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_EN_W<'a> {
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
    #[doc = "Bits 0:2 - Sets how many words for each I2S phase"]
    #[inline(always)]
    pub fn slave_words(&self) -> SLAVE_WORDS_R {
        SLAVE_WORDS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:12 - Sets how many bits per word"]
    #[inline(always)]
    pub fn slave_bits(&self) -> SLAVE_BITS_R {
        SLAVE_BITS_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Enables LSB shifting"]
    #[inline(always)]
    pub fn slave_lsb(&self) -> SLAVE_LSB_R {
        SLAVE_LSB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enables both channels"]
    #[inline(always)]
    pub fn slave_2ch(&self) -> SLAVE_2CH_R {
        SLAVE_2CH_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 31 - Enables the Slave"]
    #[inline(always)]
    pub fn slave_en(&self) -> SLAVE_EN_R {
        SLAVE_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sets how many words for each I2S phase"]
    #[inline(always)]
    pub fn slave_words(&mut self) -> SLAVE_WORDS_W {
        SLAVE_WORDS_W { w: self }
    }
    #[doc = "Bits 8:12 - Sets how many bits per word"]
    #[inline(always)]
    pub fn slave_bits(&mut self) -> SLAVE_BITS_W {
        SLAVE_BITS_W { w: self }
    }
    #[doc = "Bit 16 - Enables LSB shifting"]
    #[inline(always)]
    pub fn slave_lsb(&mut self) -> SLAVE_LSB_W {
        SLAVE_LSB_W { w: self }
    }
    #[doc = "Bit 17 - Enables both channels"]
    #[inline(always)]
    pub fn slave_2ch(&mut self) -> SLAVE_2CH_W {
        SLAVE_2CH_W { w: self }
    }
    #[doc = "Bit 31 - Enables the Slave"]
    #[inline(always)]
    pub fn slave_en(&mut self) -> SLAVE_EN_W {
        SLAVE_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration of I2S slave\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_slv_setup](index.html) module"]
pub struct I2S_SLV_SETUP_SPEC;
impl crate::RegisterSpec for I2S_SLV_SETUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_slv_setup::R](R) reader structure"]
impl crate::Readable for I2S_SLV_SETUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_slv_setup::W](W) writer structure"]
impl crate::Writable for I2S_SLV_SETUP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2S_SLV_SETUP to value 0"]
impl crate::Resettable for I2S_SLV_SETUP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
