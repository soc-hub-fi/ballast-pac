#[doc = "Register `Tx_Lwmark` reader"]
pub struct R(crate::R<TX_LWMARK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_LWMARK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_LWMARK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_LWMARK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Tx_Lwmark` writer"]
pub struct W(crate::W<TX_LWMARK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_LWMARK_SPEC>;
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
impl From<crate::W<TX_LWMARK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_LWMARK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Tx_Lwmark` reader - used to set transmit Fifo low water mark"]
pub type TX_LWMARK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Tx_Lwmark` writer - used to set transmit Fifo low water mark"]
pub type TX_LWMARK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX_LWMARK_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - used to set transmit Fifo low water mark"]
    #[inline(always)]
    pub fn tx_lwmark(&self) -> TX_LWMARK_R {
        TX_LWMARK_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - used to set transmit Fifo low water mark"]
    #[inline(always)]
    #[must_use]
    pub fn tx_lwmark(&mut self) -> TX_LWMARK_W<0> {
        TX_LWMARK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "used to set transmit Fifo low water mark\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_lwmark](index.html) module"]
pub struct TX_LWMARK_SPEC;
impl crate::RegisterSpec for TX_LWMARK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_lwmark::R](R) reader structure"]
impl crate::Readable for TX_LWMARK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_lwmark::W](W) writer structure"]
impl crate::Writable for TX_LWMARK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets Tx_Lwmark to value 0x08"]
impl crate::Resettable for TX_LWMARK_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
