#[doc = "Register `ACK_read` reader"]
pub struct R(crate::R<ACK_READ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACK_READ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACK_READ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACK_READ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACK_read` writer"]
pub struct W(crate::W<ACK_READ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACK_READ_SPEC>;
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
impl From<crate::W<ACK_READ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACK_READ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACK_read` reader - "]
pub struct ACK_READ_R(crate::FieldReader<u32>);
impl ACK_READ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ACK_READ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACK_READ_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACK_read` writer - "]
pub struct ACK_READ_W<'a> {
    w: &'a mut W,
}
impl<'a> ACK_READ_W<'a> {
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
    pub fn ack_read(&self) -> ACK_READ_R {
        ACK_READ_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ack_read(&mut self) -> ACK_READ_W {
        ACK_READ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register contains the ACK (interrupt enable) for each of the 32 interrupts or events. Writing to 0x1A10_901C sets the bits of the ACK register selected. Writing to 0x1A10_9020 clears the bits of the ACK register selected.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ack_read](index.html) module"]
pub struct ACK_READ_SPEC;
impl crate::RegisterSpec for ACK_READ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ack_read::R](R) reader structure"]
impl crate::Readable for ACK_READ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ack_read::W](W) writer structure"]
impl crate::Writable for ACK_READ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACK_read to value 0"]
impl crate::Resettable for ACK_READ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
