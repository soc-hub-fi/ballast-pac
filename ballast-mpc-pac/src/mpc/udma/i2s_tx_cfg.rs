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
pub type CONTINOUS_R = crate::BitReader<bool>;
#[doc = "Field `CONTINOUS` writer - Channel continuous mode: -1'b0: disable -1'b1: enable At the end of the buffer the uDMA reloads the address and size and starts a new transfer."]
pub type CONTINOUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_TX_CFG_SPEC, bool, O>;
#[doc = "Field `DATASIZE` reader - Channel transfer size used to increment uDMA buffer address pointer: - 2'b00: +1 (8 bits) - 2'b01: +2 (16 bits) - 2'b10: +4 ("]
pub type DATASIZE_R = crate::FieldReader<u8, DATASIZE_A>;
#[doc = "Channel transfer size used to increment uDMA buffer address pointer: - 2'b00: +1 (8 bits) - 2'b01: +2 (16 bits) - 2'b10: +4 (\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl DATASIZE_R {
    #[doc = "Get enumerated values variant"]
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
        *self == DATASIZE_A::_8BITS
    }
    #[doc = "Checks if the value of the field is `_16BITS`"]
    #[inline(always)]
    pub fn is_16bits(&self) -> bool {
        *self == DATASIZE_A::_16BITS
    }
    #[doc = "Checks if the value of the field is `_32BITS`"]
    #[inline(always)]
    pub fn is_32bits(&self) -> bool {
        *self == DATASIZE_A::_32BITS
    }
}
#[doc = "Field `DATASIZE` writer - Channel transfer size used to increment uDMA buffer address pointer: - 2'b00: +1 (8 bits) - 2'b01: +2 (16 bits) - 2'b10: +4 ("]
pub type DATASIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_TX_CFG_SPEC, u8, DATASIZE_A, 2, O>;
impl<'a, const O: u8> DATASIZE_W<'a, O> {
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
}
#[doc = "Field `EN` reader - Channel enable and start transfer: -1'b0: disable -1'b1: enable This signal is used also to queue a transfer if one is already ongoing"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Channel enable and start transfer: -1'b0: disable -1'b1: enable This signal is used also to queue a transfer if one is already ongoing"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_TX_CFG_SPEC, bool, O>;
#[doc = "Field `PENDING` reader - Transfer pending in queue status flag: -1'b0: free -1'b1: pending"]
pub type PENDING_R = crate::BitReader<bool>;
#[doc = "Field `CLR` writer - Channel clear and stop transfer: -1'b0: disable -1'b1: enable"]
pub type CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_TX_CFG_SPEC, bool, O>;
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
    #[must_use]
    pub fn continous(&mut self) -> CONTINOUS_W<0> {
        CONTINOUS_W::new(self)
    }
    #[doc = "Bits 1:2 - Channel transfer size used to increment uDMA buffer address pointer: - 2'b00: +1 (8 bits) - 2'b01: +2 (16 bits) - 2'b10: +4 ("]
    #[inline(always)]
    #[must_use]
    pub fn datasize(&mut self) -> DATASIZE_W<1> {
        DATASIZE_W::new(self)
    }
    #[doc = "Bit 4 - Channel enable and start transfer: -1'b0: disable -1'b1: enable This signal is used also to queue a transfer if one is already ongoing"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<4> {
        EN_W::new(self)
    }
    #[doc = "Bit 6 - Channel clear and stop transfer: -1'b0: disable -1'b1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn clr(&mut self) -> CLR_W<6> {
        CLR_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2S_TX_CFG to value 0x04"]
impl crate::Resettable for I2S_TX_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
