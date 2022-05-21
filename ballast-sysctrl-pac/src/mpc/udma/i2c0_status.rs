#[doc = "Register `I2C0_STATUS` reader"]
pub struct R(crate::R<I2C0_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C0_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C0_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C0_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C0_STATUS` writer"]
pub struct W(crate::W<I2C0_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C0_STATUS_SPEC>;
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
impl From<crate::W<I2C0_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C0_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUSY` reader - I2C bus busy status flag: - 1'b0: no transfer on-going - 1'b1: transfer on-going"]
pub struct BUSY_R(crate::FieldReader<bool, bool>);
impl BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSY` writer - I2C bus busy status flag: - 1'b0: no transfer on-going - 1'b1: transfer on-going"]
pub struct BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY_W<'a> {
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
#[doc = "Field `ARB_LOST` reader - I2C arbitration lost status flag: - 1'b0: no error - 1'b1: arbitration lost error"]
pub struct ARB_LOST_R(crate::FieldReader<bool, bool>);
impl ARB_LOST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ARB_LOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARB_LOST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARB_LOST` writer - I2C arbitration lost status flag: - 1'b0: no error - 1'b1: arbitration lost error"]
pub struct ARB_LOST_W<'a> {
    w: &'a mut W,
}
impl<'a> ARB_LOST_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `ACK` reader - I2C ack flag, can be polling for busy: - 1'b0: ACK - 1'b1: NAK"]
pub struct ACK_R(crate::FieldReader<bool, bool>);
impl ACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - I2C bus busy status flag: - 1'b0: no transfer on-going - 1'b1: transfer on-going"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C arbitration lost status flag: - 1'b0: no error - 1'b1: arbitration lost error"]
    #[inline(always)]
    pub fn arb_lost(&self) -> ARB_LOST_R {
        ARB_LOST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C ack flag, can be polling for busy: - 1'b0: ACK - 1'b1: NAK"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C bus busy status flag: - 1'b0: no transfer on-going - 1'b1: transfer on-going"]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W {
        BUSY_W { w: self }
    }
    #[doc = "Bit 1 - I2C arbitration lost status flag: - 1'b0: no error - 1'b1: arbitration lost error"]
    #[inline(always)]
    pub fn arb_lost(&mut self) -> ARB_LOST_W {
        ARB_LOST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uDMA I2C Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c0_status](index.html) module"]
pub struct I2C0_STATUS_SPEC;
impl crate::RegisterSpec for I2C0_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c0_status::R](R) reader structure"]
impl crate::Readable for I2C0_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c0_status::W](W) writer structure"]
impl crate::Writable for I2C0_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C0_STATUS to value 0"]
impl crate::Resettable for I2C0_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
