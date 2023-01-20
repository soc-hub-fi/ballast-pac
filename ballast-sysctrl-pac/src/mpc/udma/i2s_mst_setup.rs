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
pub type MASTER_WORDS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASTER_WORDS` writer - Sets how many words for each I2S phase"]
pub type MASTER_WORDS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_MST_SETUP_SPEC, u8, u8, 3, O>;
#[doc = "Field `MASTER_BITS` reader - Sets how many bits per word"]
pub type MASTER_BITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASTER_BITS` writer - Sets how many bits per word"]
pub type MASTER_BITS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_MST_SETUP_SPEC, u8, u8, 5, O>;
#[doc = "Field `MASTER_LSB` reader - Enables LSB shifting"]
pub type MASTER_LSB_R = crate::BitReader<bool>;
#[doc = "Field `MASTER_LSB` writer - Enables LSB shifting"]
pub type MASTER_LSB_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_MST_SETUP_SPEC, bool, O>;
#[doc = "Field `MASTER_2CH` reader - Enables both channels"]
pub type MASTER_2CH_R = crate::BitReader<bool>;
#[doc = "Field `MASTER_2CH` writer - Enables both channels"]
pub type MASTER_2CH_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_MST_SETUP_SPEC, bool, O>;
#[doc = "Field `MASTER_EN` reader - Enables the Master"]
pub type MASTER_EN_R = crate::BitReader<bool>;
#[doc = "Field `MASTER_EN` writer - Enables the Master"]
pub type MASTER_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_MST_SETUP_SPEC, bool, O>;
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
    #[must_use]
    pub fn master_words(&mut self) -> MASTER_WORDS_W<0> {
        MASTER_WORDS_W::new(self)
    }
    #[doc = "Bits 8:12 - Sets how many bits per word"]
    #[inline(always)]
    #[must_use]
    pub fn master_bits(&mut self) -> MASTER_BITS_W<8> {
        MASTER_BITS_W::new(self)
    }
    #[doc = "Bit 16 - Enables LSB shifting"]
    #[inline(always)]
    #[must_use]
    pub fn master_lsb(&mut self) -> MASTER_LSB_W<16> {
        MASTER_LSB_W::new(self)
    }
    #[doc = "Bit 17 - Enables both channels"]
    #[inline(always)]
    #[must_use]
    pub fn master_2ch(&mut self) -> MASTER_2CH_W<17> {
        MASTER_2CH_W::new(self)
    }
    #[doc = "Bit 31 - Enables the Master"]
    #[inline(always)]
    #[must_use]
    pub fn master_en(&mut self) -> MASTER_EN_W<31> {
        MASTER_EN_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2S_MST_SETUP to value 0"]
impl crate::Resettable for I2S_MST_SETUP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
