#[doc = "Register `CAM_RX_SIZE` reader"]
pub struct R(crate::R<CAM_RX_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAM_RX_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAM_RX_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAM_RX_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAM_RX_SIZE` writer"]
pub struct W(crate::W<CAM_RX_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAM_RX_SIZE_SPEC>;
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
impl From<crate::W<CAM_RX_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAM_RX_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_SIZE` reader - Buffer size in bytes. (128kBytes maximum) - Read: buffer size left - Write: set buffer size NOTE: Careful with size in byte. If you use uncompressed pixel data mapped on 16 bits, you have to declare buffer size in bytes even if buffer type is short."]
pub struct RX_SIZE_R(crate::FieldReader<u32>);
impl RX_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RX_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_SIZE_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_SIZE` writer - Buffer size in bytes. (128kBytes maximum) - Read: buffer size left - Write: set buffer size NOTE: Careful with size in byte. If you use uncompressed pixel data mapped on 16 bits, you have to declare buffer size in bytes even if buffer type is short."]
pub struct RX_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | (value as u32 & 0x0001_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16 - Buffer size in bytes. (128kBytes maximum) - Read: buffer size left - Write: set buffer size NOTE: Careful with size in byte. If you use uncompressed pixel data mapped on 16 bits, you have to declare buffer size in bytes even if buffer type is short."]
    #[inline(always)]
    pub fn rx_size(&self) -> RX_SIZE_R {
        RX_SIZE_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:16 - Buffer size in bytes. (128kBytes maximum) - Read: buffer size left - Write: set buffer size NOTE: Careful with size in byte. If you use uncompressed pixel data mapped on 16 bits, you have to declare buffer size in bytes even if buffer type is short."]
    #[inline(always)]
    pub fn rx_size(&mut self) -> RX_SIZE_W {
        RX_SIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX Camera uDMA transfer size of buffer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cam_rx_size](index.html) module"]
pub struct CAM_RX_SIZE_SPEC;
impl crate::RegisterSpec for CAM_RX_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cam_rx_size::R](R) reader structure"]
impl crate::Readable for CAM_RX_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cam_rx_size::W](W) writer structure"]
impl crate::Writable for CAM_RX_SIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAM_RX_SIZE to value 0"]
impl crate::Resettable for CAM_RX_SIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
