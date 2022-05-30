#[doc = "Register `rx_rb_size` reader"]
pub struct R(crate::R<RX_RB_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_RB_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_RB_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_RB_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rx_rb_size` writer"]
pub struct W(crate::W<RX_RB_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_RB_SIZE_SPEC>;
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
impl From<crate::W<RX_RB_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_RB_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rx_rb_size` reader - "]
pub struct RX_RB_SIZE_R(crate::FieldReader<u32>);
impl RX_RB_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RX_RB_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_RB_SIZE_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_rb_size` writer - "]
pub struct RX_RB_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_RB_SIZE_W<'a> {
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
    pub fn rx_rb_size(&self) -> RX_RB_SIZE_R {
        RX_RB_SIZE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rx_rb_size(&mut self) -> RX_RB_SIZE_W {
        RX_RB_SIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Stores the ring buffer size for the associated DMA channel in bytes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_rb_size](index.html) module"]
pub struct RX_RB_SIZE_SPEC;
impl crate::RegisterSpec for RX_RB_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_rb_size::R](R) reader structure"]
impl crate::Readable for RX_RB_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_rb_size::W](W) writer structure"]
impl crate::Writable for RX_RB_SIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rx_rb_size to value 0"]
impl crate::Resettable for RX_RB_SIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
