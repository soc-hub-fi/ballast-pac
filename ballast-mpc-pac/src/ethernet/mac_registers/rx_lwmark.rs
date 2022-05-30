#[doc = "Register `Rx_Lwmark` reader"]
pub struct R(crate::R<RX_LWMARK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_LWMARK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_LWMARK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_LWMARK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Rx_Lwmark` writer"]
pub struct W(crate::W<RX_LWMARK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_LWMARK_SPEC>;
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
impl From<crate::W<RX_LWMARK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_LWMARK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Rx_Lwmark` reader - used to set receive Fifo low water mark"]
pub struct RX_LWMARK_R(crate::FieldReader<u8>);
impl RX_LWMARK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_LWMARK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_LWMARK_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Rx_Lwmark` writer - used to set receive Fifo low water mark"]
pub struct RX_LWMARK_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_LWMARK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - used to set receive Fifo low water mark"]
    #[inline(always)]
    pub fn rx_lwmark(&self) -> RX_LWMARK_R {
        RX_LWMARK_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - used to set receive Fifo low water mark"]
    #[inline(always)]
    pub fn rx_lwmark(&mut self) -> RX_LWMARK_W {
        RX_LWMARK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "used to set receive Fifo low water mark\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_lwmark](index.html) module"]
pub struct RX_LWMARK_SPEC;
impl crate::RegisterSpec for RX_LWMARK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_lwmark::R](R) reader structure"]
impl crate::Readable for RX_LWMARK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_lwmark::W](W) writer structure"]
impl crate::Writable for RX_LWMARK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Rx_Lwmark to value 0x10"]
impl crate::Resettable for RX_LWMARK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
