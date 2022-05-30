#[doc = "Register `I2S_TX_CFG` reader"]
pub struct R(crate::R<I2S_TX_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_TX_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_TX_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_TX_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2S_TX_CFG` writer"]
pub struct W(crate::W<I2S_TX_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_TX_CFG_SPEC>;
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
impl From<crate::W<I2S_TX_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_TX_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONTINOUS` reader - Channel continuous mode: -1'b0: disable -1'b1: enable At the end of the buffer the uDMA reloads the address and size and starts a new transfer."]
pub struct CONTINOUS_R(crate::FieldReader<bool>);
impl CONTINOUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CONTINOUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONTINOUS_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONTINOUS` writer - Channel continuous mode: -1'b0: disable -1'b1: enable At the end of the buffer the uDMA reloads the address and size and starts a new transfer."]
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
#[doc = "Channel transfer size used to increment uDMA buffer address pointer: - 2'b00: +1 (8 bits) - 2'b01: +2 (16 bits) - 2'b10: +4 (\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATASIZE_A {
    #[doc = "0: `0`"]
    _8BITS = 0,
    #[doc = "1: `1`"]
    _16BITS = 1,
    #[doc = "2: `10`"]
    _32BITS = 2,
}
impl From<DATASIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: DATASIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DATASIZE` reader - Channel transfer size used to increment uDMA buffer address pointer: - 2'b00: +1 (8 bits) - 2'b01: +2 (16 bits) - 2'b10: +4 ("]
pub struct DATASIZE_R(crate::FieldReader<u8>);
impl DATASIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATASIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DATASIZE_A> {
        match self.bits {
            0 => Some(DATASIZE_A::_8BITS),
            1 => Some(DATASIZE_A::_16BITS),
            2 => Some(DATASIZE_A::_32BITS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_8BITS`"]
    #[inline(always)]
    pub fn is_8bits(&self) -> bool {
        **self == DATASIZE_A::_8BITS
    }
    #[doc = "Checks if the value of the field is `_16BITS`"]
    #[inline(always)]
    pub fn is_16bits(&self) -> bool {
        **self == DATASIZE_A::_16BITS
    }
    #[doc = "Checks if the value of the field is `_32BITS`"]
    #[inline(always)]
    pub fn is_32bits(&self) -> bool {
        **self == DATASIZE_A::_32BITS
    }
}
impl core::ops::Deref for DATASIZE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATASIZE` writer - Channel transfer size used to increment uDMA buffer address pointer: - 2'b00: +1 (8 bits) - 2'b01: +2 (16 bits) - 2'b10: +4 ("]
pub struct DATASIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATASIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATASIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _8bits(self) -> &'a mut W {
        self.variant(DATASIZE_A::_8BITS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _16bits(self) -> &'a mut W {
        self.variant(DATASIZE_A::_16BITS)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _32bits(self) -> &'a mut W {
        self.variant(DATASIZE_A::_32BITS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 1)) | ((value as u32 & 3) << 1);
        self.w
    }
}
#[doc = "Field `EN` reader - Channel enable and start transfer: -1'b0: disable -1'b1: enable This signal is used also to queue a transfer if one is already ongoing"]
pub struct EN_R(crate::FieldReader<bool>);
impl EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - Channel enable and start transfer: -1'b0: disable -1'b1: enable This signal is used also to queue a transfer if one is already ongoing"]
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
#[doc = "Field `PENDING` reader - Transfer pending in queue status flag: -1'b0: free -1'b1: pending"]
pub struct PENDING_R(crate::FieldReader<bool>);
impl PENDING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDING_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLR` writer - Channel clear and stop transfer: -1'b0: disable -1'b1: enable"]
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
    #[doc = "Bit 0 - Channel continuous mode: -1'b0: disable -1'b1: enable At the end of the buffer the uDMA reloads the address and size and starts a new transfer."]
    #[inline(always)]
    pub fn continous(&self) -> CONTINOUS_R {
        CONTINOUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Channel transfer size used to increment uDMA buffer address pointer: - 2'b00: +1 (8 bits) - 2'b01: +2 (16 bits) - 2'b10: +4 ("]
    #[inline(always)]
    pub fn datasize(&self) -> DATASIZE_R {
        DATASIZE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - Channel enable and start transfer: -1'b0: disable -1'b1: enable This signal is used also to queue a transfer if one is already ongoing"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transfer pending in queue status flag: -1'b0: free -1'b1: pending"]
    #[inline(always)]
    pub fn pending(&self) -> PENDING_R {
        PENDING_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel continuous mode: -1'b0: disable -1'b1: enable At the end of the buffer the uDMA reloads the address and size and starts a new transfer."]
    #[inline(always)]
    pub fn continous(&mut self) -> CONTINOUS_W {
        CONTINOUS_W { w: self }
    }
    #[doc = "Bits 1:2 - Channel transfer size used to increment uDMA buffer address pointer: - 2'b00: +1 (8 bits) - 2'b01: +2 (16 bits) - 2'b10: +4 ("]
    #[inline(always)]
    pub fn datasize(&mut self) -> DATASIZE_W {
        DATASIZE_W { w: self }
    }
    #[doc = "Bit 4 - Channel enable and start transfer: -1'b0: disable -1'b1: enable This signal is used also to queue a transfer if one is already ongoing"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 6 - Channel clear and stop transfer: -1'b0: disable -1'b1: enable"]
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
#[doc = "TX Channel I2S uDMA transfer configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_tx_cfg](index.html) module"]
pub struct I2S_TX_CFG_SPEC;
impl crate::RegisterSpec for I2S_TX_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_tx_cfg::R](R) reader structure"]
impl crate::Readable for I2S_TX_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_tx_cfg::W](W) writer structure"]
impl crate::Writable for I2S_TX_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2S_TX_CFG to value 0x04"]
impl crate::Resettable for I2S_TX_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
