#[doc = "Register `I2S_TX_SIZE` reader"]
pub struct R(crate::R<I2S_TX_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_TX_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_TX_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_TX_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2S_TX_SIZE` writer"]
pub struct W(crate::W<I2S_TX_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_TX_SIZE_SPEC>;
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
impl From<crate::W<I2S_TX_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_TX_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_SIZE` reader - Buffer size in byte. (128kBytes maximum) - Read: buffer size left - Write: set buffer size"]
pub type TX_SIZE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TX_SIZE` writer - Buffer size in byte. (128kBytes maximum) - Read: buffer size left - Write: set buffer size"]
pub type TX_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_TX_SIZE_SPEC, u32, u32, 17, O>;
impl R {
    #[doc = "Bits 0:16 - Buffer size in byte. (128kBytes maximum) - Read: buffer size left - Write: set buffer size"]
    #[inline(always)]
    pub fn tx_size(&self) -> TX_SIZE_R {
        TX_SIZE_R::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - Buffer size in byte. (128kBytes maximum) - Read: buffer size left - Write: set buffer size"]
    #[inline(always)]
    #[must_use]
    pub fn tx_size(&mut self) -> TX_SIZE_W<0> {
        TX_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX Channel I2S uDMA transfer size of buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_tx_size](index.html) module"]
pub struct I2S_TX_SIZE_SPEC;
impl crate::RegisterSpec for I2S_TX_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_tx_size::R](R) reader structure"]
impl crate::Readable for I2S_TX_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_tx_size::W](W) writer structure"]
impl crate::Writable for I2S_TX_SIZE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2S_TX_SIZE to value 0"]
impl crate::Resettable for I2S_TX_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
