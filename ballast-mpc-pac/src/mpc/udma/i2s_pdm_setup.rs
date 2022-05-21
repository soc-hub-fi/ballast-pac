#[doc = "Register `I2S_PDM_SETUP` reader"]
pub struct R(crate::R<I2S_PDM_SETUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_PDM_SETUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_PDM_SETUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_PDM_SETUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2S_PDM_SETUP` writer"]
pub struct W(crate::W<I2S_PDM_SETUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_PDM_SETUP_SPEC>;
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
impl From<crate::W<I2S_PDM_SETUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_PDM_SETUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDM_SHIFT` reader - Shifts the output of the filter"]
pub struct PDM_SHIFT_R(crate::FieldReader<u8, u8>);
impl PDM_SHIFT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PDM_SHIFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDM_SHIFT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDM_SHIFT` writer - Shifts the output of the filter"]
pub struct PDM_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> PDM_SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !7) | (value as u32 & 7);
        self.w
    }
}
#[doc = "Field `PDM_DECIMATION` reader - Sets the decimation ratio of the filter"]
pub struct PDM_DECIMATION_R(crate::FieldReader<u16, u16>);
impl PDM_DECIMATION_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PDM_DECIMATION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDM_DECIMATION_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDM_DECIMATION` writer - Sets the decimation ratio of the filter"]
pub struct PDM_DECIMATION_W<'a> {
    w: &'a mut W,
}
impl<'a> PDM_DECIMATION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 3)) | ((value as u32 & 0x03ff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Shifts the output of the filter"]
    #[inline(always)]
    pub fn pdm_shift(&self) -> PDM_SHIFT_R {
        PDM_SHIFT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:12 - Sets the decimation ratio of the filter"]
    #[inline(always)]
    pub fn pdm_decimation(&self) -> PDM_DECIMATION_R {
        PDM_DECIMATION_R::new(((self.bits >> 3) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - Shifts the output of the filter"]
    #[inline(always)]
    pub fn pdm_shift(&mut self) -> PDM_SHIFT_W {
        PDM_SHIFT_W { w: self }
    }
    #[doc = "Bits 3:12 - Sets the decimation ratio of the filter"]
    #[inline(always)]
    pub fn pdm_decimation(&mut self) -> PDM_DECIMATION_W {
        PDM_DECIMATION_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration of PDM module\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_pdm_setup](index.html) module"]
pub struct I2S_PDM_SETUP_SPEC;
impl crate::RegisterSpec for I2S_PDM_SETUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_pdm_setup::R](R) reader structure"]
impl crate::Readable for I2S_PDM_SETUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_pdm_setup::W](W) writer structure"]
impl crate::Writable for I2S_PDM_SETUP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2S_PDM_SETUP to value 0"]
impl crate::Resettable for I2S_PDM_SETUP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
