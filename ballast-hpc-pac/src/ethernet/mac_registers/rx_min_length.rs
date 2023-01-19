#[doc = "Register `RX_MIN_LENGTH` reader"]
pub struct R(crate::R<RX_MIN_LENGTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_MIN_LENGTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_MIN_LENGTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_MIN_LENGTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_MIN_LENGTH` writer"]
pub struct W(crate::W<RX_MIN_LENGTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_MIN_LENGTH_SPEC>;
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
impl From<crate::W<RX_MIN_LENGTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_MIN_LENGTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_MIN_LENGTH` reader - The length of Received packet beyond minimal length and maximal length will be droped in receive Fifo. if the packet is already trasmitting in user interface , a error flag will enclosed at end of packet ."]
pub type RX_MIN_LENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_MIN_LENGTH` writer - The length of Received packet beyond minimal length and maximal length will be droped in receive Fifo. if the packet is already trasmitting in user interface , a error flag will enclosed at end of packet ."]
pub type RX_MIN_LENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RX_MIN_LENGTH_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - The length of Received packet beyond minimal length and maximal length will be droped in receive Fifo. if the packet is already trasmitting in user interface , a error flag will enclosed at end of packet ."]
    #[inline(always)]
    pub fn rx_min_length(&self) -> RX_MIN_LENGTH_R {
        RX_MIN_LENGTH_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The length of Received packet beyond minimal length and maximal length will be droped in receive Fifo. if the packet is already trasmitting in user interface , a error flag will enclosed at end of packet ."]
    #[inline(always)]
    #[must_use]
    pub fn rx_min_length(&mut self) -> RX_MIN_LENGTH_W<0> {
        RX_MIN_LENGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The length of Received packet beyond minimal length and maximal length will be droped in receive Fifo. if the packet is already trasmitting in user interface , a error flag will enclosed at end of packet .\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_min_length](index.html) module"]
pub struct RX_MIN_LENGTH_SPEC;
impl crate::RegisterSpec for RX_MIN_LENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_min_length::R](R) reader structure"]
impl crate::Readable for RX_MIN_LENGTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_min_length::W](W) writer structure"]
impl crate::Writable for RX_MIN_LENGTH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_MIN_LENGTH to value 0x40"]
impl crate::Resettable for RX_MIN_LENGTH_SPEC {
    const RESET_VALUE: Self::Ux = 0x40;
}
