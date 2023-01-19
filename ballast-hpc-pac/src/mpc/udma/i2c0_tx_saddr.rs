#[doc = "Register `I2C0_TX_SADDR` reader"]
pub struct R(crate::R<I2C0_TX_SADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C0_TX_SADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C0_TX_SADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C0_TX_SADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C0_TX_SADDR` writer"]
pub struct W(crate::W<I2C0_TX_SADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C0_TX_SADDR_SPEC>;
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
impl From<crate::W<I2C0_TX_SADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C0_TX_SADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_SADDR` reader - TX buffer base address bitfield: - Read: returns value of the buffer pointer until transfer is finished. Else returns 0. - Write: sets buffer base address"]
pub type TX_SADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TX_SADDR` writer - TX buffer base address bitfield: - Read: returns value of the buffer pointer until transfer is finished. Else returns 0. - Write: sets buffer base address"]
pub type TX_SADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2C0_TX_SADDR_SPEC, u32, u32, 21, O>;
impl R {
    #[doc = "Bits 0:20 - TX buffer base address bitfield: - Read: returns value of the buffer pointer until transfer is finished. Else returns 0. - Write: sets buffer base address"]
    #[inline(always)]
    pub fn tx_saddr(&self) -> TX_SADDR_R {
        TX_SADDR_R::new(self.bits & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:20 - TX buffer base address bitfield: - Read: returns value of the buffer pointer until transfer is finished. Else returns 0. - Write: sets buffer base address"]
    #[inline(always)]
    #[must_use]
    pub fn tx_saddr(&mut self) -> TX_SADDR_W<0> {
        TX_SADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uDMA TX I2C buffer base address configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c0_tx_saddr](index.html) module"]
pub struct I2C0_TX_SADDR_SPEC;
impl crate::RegisterSpec for I2C0_TX_SADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c0_tx_saddr::R](R) reader structure"]
impl crate::Readable for I2C0_TX_SADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c0_tx_saddr::W](W) writer structure"]
impl crate::Writable for I2C0_TX_SADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2C0_TX_SADDR to value 0"]
impl crate::Resettable for I2C0_TX_SADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
