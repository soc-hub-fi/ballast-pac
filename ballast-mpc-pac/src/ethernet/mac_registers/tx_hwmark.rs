#[doc = "Register `Tx_Hwmark` reader"]
pub struct R(crate::R<TX_HWMARK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_HWMARK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_HWMARK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_HWMARK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Tx_Hwmark` writer"]
pub struct W(crate::W<TX_HWMARK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_HWMARK_SPEC>;
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
impl From<crate::W<TX_HWMARK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_HWMARK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Tx_Hwmark` reader - used to set transmit Fifo high water mark"]
pub type TX_HWMARK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Tx_Hwmark` writer - used to set transmit Fifo high water mark"]
pub type TX_HWMARK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX_HWMARK_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - used to set transmit Fifo high water mark"]
    #[inline(always)]
    pub fn tx_hwmark(&self) -> TX_HWMARK_R {
        TX_HWMARK_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - used to set transmit Fifo high water mark"]
    #[inline(always)]
    #[must_use]
    pub fn tx_hwmark(&mut self) -> TX_HWMARK_W<0> {
        TX_HWMARK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "used to set transmit Fifo high water mark\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_hwmark](index.html) module"]
pub struct TX_HWMARK_SPEC;
impl crate::RegisterSpec for TX_HWMARK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_hwmark::R](R) reader structure"]
impl crate::Readable for TX_HWMARK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_hwmark::W](W) writer structure"]
impl crate::Writable for TX_HWMARK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets Tx_Hwmark to value 0x09"]
impl crate::Resettable for TX_HWMARK_SPEC {
    const RESET_VALUE: Self::Ux = 0x09;
}
