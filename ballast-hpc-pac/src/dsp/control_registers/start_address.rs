#[doc = "Register `start_address` reader"]
pub struct R(crate::R<START_ADDRESS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<START_ADDRESS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<START_ADDRESS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<START_ADDRESS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `start_address` writer"]
pub struct W(crate::W<START_ADDRESS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<START_ADDRESS_SPEC>;
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
impl From<crate::W<START_ADDRESS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<START_ADDRESS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `start_address` reader - star"]
pub struct START_ADDRESS_R(crate::FieldReader<u32>);
impl START_ADDRESS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        START_ADDRESS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_ADDRESS_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `start_address` writer - star"]
pub struct START_ADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> START_ADDRESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - star"]
    #[inline(always)]
    pub fn start_address(&self) -> START_ADDRESS_R {
        START_ADDRESS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - star"]
    #[inline(always)]
    pub fn start_address(&mut self) -> START_ADDRESS_W {
        START_ADDRESS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [start_address](index.html) module"]
pub struct START_ADDRESS_SPEC;
impl crate::RegisterSpec for START_ADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [start_address::R](R) reader structure"]
impl crate::Readable for START_ADDRESS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [start_address::W](W) writer structure"]
impl crate::Writable for START_ADDRESS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets start_address to value 0"]
impl crate::Resettable for START_ADDRESS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
