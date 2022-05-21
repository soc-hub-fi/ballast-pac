#[doc = "Register `RX_MAX_LENGTH` reader"]
pub struct R(crate::R<RX_MAX_LENGTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_MAX_LENGTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_MAX_LENGTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_MAX_LENGTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_MAX_LENGTH` writer"]
pub struct W(crate::W<RX_MAX_LENGTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_MAX_LENGTH_SPEC>;
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
impl From<crate::W<RX_MAX_LENGTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_MAX_LENGTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_MAX_LENGTH` reader - The length of Received packet beyond minimal length and maximal length will be droped in receive Fifo. if the packet is already trasmitting in user interface , a error flag will enclosed at end of packet ."]
pub struct RX_MAX_LENGTH_R(crate::FieldReader<u16, u16>);
impl RX_MAX_LENGTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RX_MAX_LENGTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_MAX_LENGTH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_MAX_LENGTH` writer - The length of Received packet beyond minimal length and maximal length will be droped in receive Fifo. if the packet is already trasmitting in user interface , a error flag will enclosed at end of packet ."]
pub struct RX_MAX_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_MAX_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - The length of Received packet beyond minimal length and maximal length will be droped in receive Fifo. if the packet is already trasmitting in user interface , a error flag will enclosed at end of packet ."]
    #[inline(always)]
    pub fn rx_max_length(&self) -> RX_MAX_LENGTH_R {
        RX_MAX_LENGTH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The length of Received packet beyond minimal length and maximal length will be droped in receive Fifo. if the packet is already trasmitting in user interface , a error flag will enclosed at end of packet ."]
    #[inline(always)]
    pub fn rx_max_length(&mut self) -> RX_MAX_LENGTH_W {
        RX_MAX_LENGTH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The length of Received packet beyond minimal length and maximal length will be droped in receive Fifo. if the packet is already trasmitting in user interface , a error flag will enclosed at end of packet .\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_max_length](index.html) module"]
pub struct RX_MAX_LENGTH_SPEC;
impl crate::RegisterSpec for RX_MAX_LENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_max_length::R](R) reader structure"]
impl crate::Readable for RX_MAX_LENGTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_max_length::W](W) writer structure"]
impl crate::Writable for RX_MAX_LENGTH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RX_MAX_LENGTH to value 0x2710"]
impl crate::Resettable for RX_MAX_LENGTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2710
    }
}
