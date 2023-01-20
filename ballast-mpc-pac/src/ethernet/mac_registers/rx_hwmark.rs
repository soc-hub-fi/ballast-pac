#[doc = "Register `Rx_Hwmark` reader"]
pub struct R(crate::R<RX_HWMARK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_HWMARK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_HWMARK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_HWMARK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Rx_Hwmark` writer"]
pub struct W(crate::W<RX_HWMARK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_HWMARK_SPEC>;
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
impl From<crate::W<RX_HWMARK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_HWMARK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Rx_Hwmark` reader - used to set receive Fifo high water mark"]
pub type RX_HWMARK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Rx_Hwmark` writer - used to set receive Fifo high water mark"]
pub type RX_HWMARK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RX_HWMARK_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - used to set receive Fifo high water mark"]
    #[inline(always)]
    pub fn rx_hwmark(&self) -> RX_HWMARK_R {
        RX_HWMARK_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - used to set receive Fifo high water mark"]
    #[inline(always)]
    #[must_use]
    pub fn rx_hwmark(&mut self) -> RX_HWMARK_W<0> {
        RX_HWMARK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "used to set receive Fifo high water mark\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_hwmark](index.html) module"]
pub struct RX_HWMARK_SPEC;
impl crate::RegisterSpec for RX_HWMARK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_hwmark::R](R) reader structure"]
impl crate::Readable for RX_HWMARK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_hwmark::W](W) writer structure"]
impl crate::Writable for RX_HWMARK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets Rx_Hwmark to value 0x1a"]
impl crate::Resettable for RX_HWMARK_SPEC {
    const RESET_VALUE: Self::Ux = 0x1a;
}
