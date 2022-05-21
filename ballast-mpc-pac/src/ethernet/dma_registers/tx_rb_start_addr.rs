#[doc = "Register `tx_rb_start_addr` reader"]
pub struct R(crate::R<TX_RB_START_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_RB_START_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_RB_START_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_RB_START_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tx_rb_start_addr` writer"]
pub struct W(crate::W<TX_RB_START_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_RB_START_ADDR_SPEC>;
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
impl From<crate::W<TX_RB_START_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_RB_START_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tx_rb_start_addr` reader - "]
pub struct TX_RB_START_ADDR_R(crate::FieldReader<u32, u32>);
impl TX_RB_START_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TX_RB_START_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_RB_START_ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_rb_start_addr` writer - "]
pub struct TX_RB_START_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_RB_START_ADDR_W<'a> {
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
    pub fn tx_rb_start_addr(&self) -> TX_RB_START_ADDR_R {
        TX_RB_START_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tx_rb_start_addr(&mut self) -> TX_RB_START_ADDR_W {
        TX_RB_START_ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register to store the start address for the ring buffer used by the DMA to write to. Writing to the register has a side effect of initializing the ring buffer from the start address and size registers in the respective DMA unit and resetting the write pointer (for RX) to the start of the ring buffer. Therefore, the associated size register (tx_rb_size should be set first The DMA read and/or write channels remain inactive until the associated ring buffer is initialized by writing to this register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_rb_start_addr](index.html) module"]
pub struct TX_RB_START_ADDR_SPEC;
impl crate::RegisterSpec for TX_RB_START_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_rb_start_addr::R](R) reader structure"]
impl crate::Readable for TX_RB_START_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_rb_start_addr::W](W) writer structure"]
impl crate::Writable for TX_RB_START_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets tx_rb_start_addr to value 0"]
impl crate::Resettable for TX_RB_START_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
