#[doc = "Register `I2S_MST_SETUP` reader"]
pub struct R(crate::R<I2S_MST_SETUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_MST_SETUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_MST_SETUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_MST_SETUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2S_MST_SETUP` writer"]
pub struct W(crate::W<I2S_MST_SETUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_MST_SETUP_SPEC>;
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
impl From<crate::W<I2S_MST_SETUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_MST_SETUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASTER_WORDS` reader - Sets how many words for each I2S phase"]
pub struct MASTER_WORDS_R(crate::FieldReader<u8, u8>);
impl MASTER_WORDS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MASTER_WORDS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASTER_WORDS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASTER_WORDS` writer - Sets how many words for each I2S phase"]
pub struct MASTER_WORDS_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_WORDS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !7) | (value as u32 & 7);
        self.w
    }
}
#[doc = "Field `MASTER_BITS` reader - Sets how many bits per word"]
pub struct MASTER_BITS_R(crate::FieldReader<u8, u8>);
impl MASTER_BITS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MASTER_BITS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASTER_BITS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASTER_BITS` writer - Sets how many bits per word"]
pub struct MASTER_BITS_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_BITS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `MASTER_LSB` reader - Enables LSB shifting"]
pub struct MASTER_LSB_R(crate::FieldReader<bool, bool>);
impl MASTER_LSB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MASTER_LSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASTER_LSB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASTER_LSB` writer - Enables LSB shifting"]
pub struct MASTER_LSB_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_LSB_W<'a> {
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
#[doc = "Field `MASTER_2CH` reader - Enables both channels"]
pub struct MASTER_2CH_R(crate::FieldReader<bool, bool>);
impl MASTER_2CH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MASTER_2CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASTER_2CH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASTER_2CH` writer - Enables both channels"]
pub struct MASTER_2CH_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_2CH_W<'a> {
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
#[doc = "Field `MASTER_EN` reader - Enables the Master"]
pub struct MASTER_EN_R(crate::FieldReader<bool, bool>);
impl MASTER_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MASTER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASTER_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASTER_EN` writer - Enables the Master"]
pub struct MASTER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_EN_W<'a> {
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
    pub fn master_words(&self) -> MASTER_WORDS_R {
        MASTER_WORDS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:12 - Sets how many bits per word"]
    #[inline(always)]
    pub fn master_bits(&self) -> MASTER_BITS_R {
        MASTER_BITS_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Enables LSB shifting"]
    #[inline(always)]
    pub fn master_lsb(&self) -> MASTER_LSB_R {
        MASTER_LSB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enables both channels"]
    #[inline(always)]
    pub fn master_2ch(&self) -> MASTER_2CH_R {
        MASTER_2CH_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 31 - Enables the Master"]
    #[inline(always)]
    pub fn master_en(&self) -> MASTER_EN_R {
        MASTER_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sets how many words for each I2S phase"]
    #[inline(always)]
    pub fn master_words(&mut self) -> MASTER_WORDS_W {
        MASTER_WORDS_W { w: self }
    }
    #[doc = "Bits 8:12 - Sets how many bits per word"]
    #[inline(always)]
    pub fn master_bits(&mut self) -> MASTER_BITS_W {
        MASTER_BITS_W { w: self }
    }
    #[doc = "Bit 16 - Enables LSB shifting"]
    #[inline(always)]
    pub fn master_lsb(&mut self) -> MASTER_LSB_W {
        MASTER_LSB_W { w: self }
    }
    #[doc = "Bit 17 - Enables both channels"]
    #[inline(always)]
    pub fn master_2ch(&mut self) -> MASTER_2CH_W {
        MASTER_2CH_W { w: self }
    }
    #[doc = "Bit 31 - Enables the Master"]
    #[inline(always)]
    pub fn master_en(&mut self) -> MASTER_EN_W {
        MASTER_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration of I2S master\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_mst_setup](index.html) module"]
pub struct I2S_MST_SETUP_SPEC;
impl crate::RegisterSpec for I2S_MST_SETUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_mst_setup::R](R) reader structure"]
impl crate::Readable for I2S_MST_SETUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_mst_setup::W](W) writer structure"]
impl crate::Writable for I2S_MST_SETUP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2S_MST_SETUP to value 0"]
impl crate::Resettable for I2S_MST_SETUP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
