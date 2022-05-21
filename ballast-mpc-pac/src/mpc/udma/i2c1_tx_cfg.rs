#[doc = "Register `I2C1_TX_CFG` reader"]
pub struct R(crate::R<I2C1_TX_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C1_TX_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C1_TX_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C1_TX_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C1_TX_CFG` writer"]
pub struct W(crate::W<I2C1_TX_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C1_TX_CFG_SPEC>;
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
impl From<crate::W<I2C1_TX_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C1_TX_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONTINOUS` reader - TX channel continuous mode bitfield: -1'b0: disabled -1'b1: enabled At the end of the buffer transfer, the uDMA reloads the address / buffer size and starts a new transfer."]
pub struct CONTINOUS_R(crate::FieldReader<bool, bool>);
impl CONTINOUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CONTINOUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONTINOUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONTINOUS` writer - TX channel continuous mode bitfield: -1'b0: disabled -1'b1: enabled At the end of the buffer transfer, the uDMA reloads the address / buffer size and starts a new transfer."]
pub struct CONTINOUS_W<'a> {
    w: &'a mut W,
}
impl<'a> CONTINOUS_W<'a> {
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
#[doc = "Field `EN` reader - TX channel enable and start transfer bitfield: -1'b0: disable -1'b1: enable and start the transfer This signal is used also to queue a transfer if one is already ongoing."]
pub struct EN_R(crate::FieldReader<bool, bool>);
impl EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - TX channel enable and start transfer bitfield: -1'b0: disable -1'b1: enable and start the transfer This signal is used also to queue a transfer if one is already ongoing."]
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `PENDING` reader - TX transfer pending in queue status flag: -1'b0: no pending transfer in the queue -1'b1: pending transfer in the queue"]
pub struct PENDING_R(crate::FieldReader<bool, bool>);
impl PENDING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLR` writer - TX channel clear and stop transfer bitfield: -1'b0: disabled -1'b1: stop and clear the on-going transfer"]
pub struct CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - TX channel continuous mode bitfield: -1'b0: disabled -1'b1: enabled At the end of the buffer transfer, the uDMA reloads the address / buffer size and starts a new transfer."]
    #[inline(always)]
    pub fn continous(&self) -> CONTINOUS_R {
        CONTINOUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - TX channel enable and start transfer bitfield: -1'b0: disable -1'b1: enable and start the transfer This signal is used also to queue a transfer if one is already ongoing."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TX transfer pending in queue status flag: -1'b0: no pending transfer in the queue -1'b1: pending transfer in the queue"]
    #[inline(always)]
    pub fn pending(&self) -> PENDING_R {
        PENDING_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX channel continuous mode bitfield: -1'b0: disabled -1'b1: enabled At the end of the buffer transfer, the uDMA reloads the address / buffer size and starts a new transfer."]
    #[inline(always)]
    pub fn continous(&mut self) -> CONTINOUS_W {
        CONTINOUS_W { w: self }
    }
    #[doc = "Bit 4 - TX channel enable and start transfer bitfield: -1'b0: disable -1'b1: enable and start the transfer This signal is used also to queue a transfer if one is already ongoing."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 6 - TX channel clear and stop transfer bitfield: -1'b0: disabled -1'b1: stop and clear the on-going transfer"]
    #[inline(always)]
    pub fn clr(&mut self) -> CLR_W {
        CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uDMA TX I2C stream configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c1_tx_cfg](index.html) module"]
pub struct I2C1_TX_CFG_SPEC;
impl crate::RegisterSpec for I2C1_TX_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c1_tx_cfg::R](R) reader structure"]
impl crate::Readable for I2C1_TX_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c1_tx_cfg::W](W) writer structure"]
impl crate::Writable for I2C1_TX_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C1_TX_CFG to value 0"]
impl crate::Resettable for I2C1_TX_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
