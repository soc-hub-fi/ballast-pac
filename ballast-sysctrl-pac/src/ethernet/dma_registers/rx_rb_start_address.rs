#[doc = "Register `rx_rb_start_address` reader"]
pub struct R(crate::R<RX_RB_START_ADDRESS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_RB_START_ADDRESS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_RB_START_ADDRESS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_RB_START_ADDRESS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rx_rb_start_address` writer"]
pub struct W(crate::W<RX_RB_START_ADDRESS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_RB_START_ADDRESS_SPEC>;
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
impl From<crate::W<RX_RB_START_ADDRESS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_RB_START_ADDRESS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rx_rb_start_address` reader - "]
pub struct RX_RB_START_ADDRESS_R(crate::FieldReader<u32, u32>);
impl RX_RB_START_ADDRESS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RX_RB_START_ADDRESS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_RB_START_ADDRESS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_rb_start_address` writer - "]
pub struct RX_RB_START_ADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_RB_START_ADDRESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rx_rb_start_address(&self) -> RX_RB_START_ADDRESS_R {
        RX_RB_START_ADDRESS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rx_rb_start_address(&mut self) -> RX_RB_START_ADDRESS_W {
        RX_RB_START_ADDRESS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rx_rb_start_address, tx_rb_start_address These registers store the start address for the ring buffer used by the DMA to read from or write to. Writing to these registers has a side effect of initializing the ring buffer from the start address and size registers in the respective DMA unit and resetting the write pointer (for RX) or the read pointer (for TX) to the start of the ring buffer. Therefore, the associated size register (rx_rb_size or tx_rb_size) should be set first. The DMA read and/or write channels remain inactive until the associated ring buffer is initialized by writing to this register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_rb_start_address](index.html) module"]
pub struct RX_RB_START_ADDRESS_SPEC;
impl crate::RegisterSpec for RX_RB_START_ADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_rb_start_address::R](R) reader structure"]
impl crate::Readable for RX_RB_START_ADDRESS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_rb_start_address::W](W) writer structure"]
impl crate::Writable for RX_RB_START_ADDRESS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rx_rb_start_address to value 0"]
impl crate::Resettable for RX_RB_START_ADDRESS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
