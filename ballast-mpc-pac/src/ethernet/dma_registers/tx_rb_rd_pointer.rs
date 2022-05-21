#[doc = "Register `tx_rb_rd_pointer` reader"]
pub struct R(crate::R<TX_RB_RD_POINTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_RB_RD_POINTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_RB_RD_POINTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_RB_RD_POINTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `tx_rb_rd_pointer` reader - "]
pub struct TX_RB_RD_POINTER_R(crate::FieldReader<u32, u32>);
impl TX_RB_RD_POINTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TX_RB_RD_POINTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_RB_RD_POINTER_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tx_rb_rd_pointer(&self) -> TX_RB_RD_POINTER_R {
        TX_RB_RD_POINTER_R::new(self.bits)
    }
}
#[doc = "Stores the read pointers for the ring buffers. For the RX/write channel, the read pointer is maintained by the host processor. If there is a packet available in the MAC RX FIFO but writing it to memory would overwrite unread data -- i.e. advance the write pointer past the read pointer -- The DMA write channel stalls until the read pointer is updated to resolve this conflict. The value should point to the last read address or, when initialized, the last address of the ring buffer. For the TX/read channel, the read pointer register is read-only and maintained by the host processor. The processor must not write “past” the read pointer.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_rb_rd_pointer](index.html) module"]
pub struct TX_RB_RD_POINTER_SPEC;
impl crate::RegisterSpec for TX_RB_RD_POINTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_rb_rd_pointer::R](R) reader structure"]
impl crate::Readable for TX_RB_RD_POINTER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets tx_rb_rd_pointer to value 0"]
impl crate::Resettable for TX_RB_RD_POINTER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
