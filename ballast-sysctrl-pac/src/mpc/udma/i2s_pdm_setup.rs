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
pub type PDM_SHIFT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PDM_SHIFT` writer - Shifts the output of the filter"]
pub type PDM_SHIFT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_PDM_SETUP_SPEC, u8, u8, 3, O>;
#[doc = "Field `PDM_DECIMATION` reader - Sets the decimation ratio of the filter"]
pub type PDM_DECIMATION_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PDM_DECIMATION` writer - Sets the decimation ratio of the filter"]
pub type PDM_DECIMATION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_PDM_SETUP_SPEC, u16, u16, 10, O>;
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
    #[must_use]
    pub fn pdm_shift(&mut self) -> PDM_SHIFT_W<0> {
        PDM_SHIFT_W::new(self)
    }
    #[doc = "Bits 3:12 - Sets the decimation ratio of the filter"]
    #[inline(always)]
    #[must_use]
    pub fn pdm_decimation(&mut self) -> PDM_DECIMATION_W<3> {
        PDM_DECIMATION_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2S_PDM_SETUP to value 0"]
impl crate::Resettable for I2S_PDM_SETUP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
