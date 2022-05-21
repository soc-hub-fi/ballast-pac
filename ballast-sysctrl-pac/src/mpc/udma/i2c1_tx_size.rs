#[doc = "Register `I2C1_TX_SIZE` reader"]
pub struct R(crate::R<I2C1_TX_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C1_TX_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C1_TX_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C1_TX_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C1_TX_SIZE` writer"]
pub struct W(crate::W<I2C1_TX_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C1_TX_SIZE_SPEC>;
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
impl From<crate::W<I2C1_TX_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C1_TX_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_SIZE` reader - TX buffer size bitfield in bytes. (128kBytes maximum) - Read: returns remaining buffer size to transfer. - Write: sets buffer size."]
pub struct TX_SIZE_R(crate::FieldReader<u32, u32>);
impl TX_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TX_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_SIZE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_SIZE` writer - TX buffer size bitfield in bytes. (128kBytes maximum) - Read: returns remaining buffer size to transfer. - Write: sets buffer size."]
pub struct TX_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - TX buffer size bitfield in bytes. (128kBytes maximum) - Read: returns remaining buffer size to transfer. - Write: sets buffer size."]
    #[inline(always)]
    pub fn tx_size(&self) -> TX_SIZE_R {
        TX_SIZE_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - TX buffer size bitfield in bytes. (128kBytes maximum) - Read: returns remaining buffer size to transfer. - Write: sets buffer size."]
    #[inline(always)]
    pub fn tx_size(&mut self) -> TX_SIZE_W {
        TX_SIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uDMA TX I2C buffer size configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c1_tx_size](index.html) module"]
pub struct I2C1_TX_SIZE_SPEC;
impl crate::RegisterSpec for I2C1_TX_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c1_tx_size::R](R) reader structure"]
impl crate::Readable for I2C1_TX_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c1_tx_size::W](W) writer structure"]
impl crate::Writable for I2C1_TX_SIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C1_TX_SIZE to value 0"]
impl crate::Resettable for I2C1_TX_SIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}