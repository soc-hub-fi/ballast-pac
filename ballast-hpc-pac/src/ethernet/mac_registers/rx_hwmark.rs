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
pub struct RX_HWMARK_R(crate::FieldReader<u8>);
impl RX_HWMARK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_HWMARK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_HWMARK_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Rx_Hwmark` writer - used to set receive Fifo high water mark"]
pub struct RX_HWMARK_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_HWMARK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
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
    pub fn rx_hwmark(&mut self) -> RX_HWMARK_W {
        RX_HWMARK_W { w: self }
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
}
#[doc = "`reset()` method sets Rx_Hwmark to value 0x1a"]
impl crate::Resettable for RX_HWMARK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1a
    }
}
